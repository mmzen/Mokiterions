#!/usr/bin/env python3
"""Phase E of VER-MOK-019: the performance and resilience checks.

Nothing here asserts a threshold. `ADR-MOK-008` extrapolated a 400-cell sweep from 20 runs and the
contract's own words are that "a material divergence is a finding about the extrapolation rather than
a failure of the instrument". So each figure is measured, reported beside the prediction, and the
case passes on the measurement having been taken and being legible.

Two things here are genuinely resilience rather than cost: a stream large enough to establish that
the driver does not read a whole file into memory, and an interruption delivered to a running batch.
"""

from __future__ import annotations

import ctypes
import ctypes.wintypes as wintypes
import glob
import json
import os
import platform
import shutil
import signal
import subprocess
import sys
import threading
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from oracle import (  # noqa: E402
    CLASSIFIER,
    DRIVER,
    REPO,
    WORK,
    Report,
    read_jsonl,
    run,
    sha256_whole,
)

OUT = f"{WORK}/phase-e"

# ADR-MOK-008's measurement table, at docs/.../adr/ADR-MOK-008.md:70-78.
PREDICTED = {
    "batch wall seconds": 19.0,
    "scan wall seconds": 16.0,
    "retained bytes": 500_000,
    "one 1,000-tick social stream bytes": 2_723_014,
    "streams if kept, bytes": 1_200_000_000,
}
LONG_RUN_FLOOR = 2_723_014


class PROCESS_MEMORY_COUNTERS(ctypes.Structure):
    _fields_ = [
        ("cb", wintypes.DWORD),
        ("PageFaultCount", wintypes.DWORD),
        ("PeakWorkingSetSize", ctypes.c_size_t),
        ("WorkingSetSize", ctypes.c_size_t),
        ("QuotaPeakPagedPoolUsage", ctypes.c_size_t),
        ("QuotaPagedPoolUsage", ctypes.c_size_t),
        ("QuotaPeakNonPagedPoolUsage", ctypes.c_size_t),
        ("QuotaNonPagedPoolUsage", ctypes.c_size_t),
        ("PagefileUsage", ctypes.c_size_t),
        ("PeakPagefileUsage", ctypes.c_size_t),
    ]


_PSAPI = ctypes.WinDLL("psapi", use_last_error=True)


def working_set(handle):
    """The driver process's working set and its peak, in bytes, or None if the handle has gone."""
    counters = PROCESS_MEMORY_COUNTERS()
    counters.cb = ctypes.sizeof(counters)
    if not _PSAPI.GetProcessMemoryInfo(wintypes.HANDLE(handle), ctypes.byref(counters),
                                       counters.cb):
        return None
    return counters.WorkingSetSize, counters.PeakWorkingSetSize


def keep(name, text):
    os.makedirs(OUT, exist_ok=True)
    with open(f"{OUT}/{name}", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(text if text.endswith("\n") else text + "\n")


def fresh(directory):
    if os.path.isdir(directory):
        shutil.rmtree(directory)
    os.makedirs(directory)
    return directory


def measured_batch(arguments, stream_dir, label, poll=0.002):
    """Run the driver, timing it and watching both the stream directory and its own memory.

    The stream watcher is a polling thread rather than a filesystem notification, because P5's claim
    is about the peak and a notification can coalesce. `poll` is 2 ms, which over a sweep of this
    length is tens of thousands of samples.
    """
    os.makedirs(OUT, exist_ok=True)
    out_log = f"{OUT}/{label}.log"
    peak = {"bytes": 0, "files": 0, "samples": 0, "working_set": 0, "memory_samples": 0}
    stop = threading.Event()

    handle_box = {}

    def watch():
        last_memory = 0.0
        while not stop.is_set():
            total = 0
            files = 0
            try:
                for entry in os.scandir(stream_dir):
                    if entry.is_file():
                        files += 1
                        try:
                            total += entry.stat().st_size
                        except OSError:
                            pass
            except OSError:
                pass
            peak["bytes"] = max(peak["bytes"], total)
            peak["files"] = max(peak["files"], files)
            peak["samples"] += 1
            now = time.perf_counter()
            if "handle" in handle_box and now - last_memory > 0.020:
                last_memory = now
                reading = working_set(handle_box["handle"])
                if reading:
                    peak["working_set"] = max(peak["working_set"], reading[1])
                    peak["memory_samples"] += 1
            time.sleep(poll)

    watcher = threading.Thread(target=watch, daemon=True)
    watcher.start()
    started = time.perf_counter()
    with open(out_log, "wb") as sink:
        process = subprocess.Popen([sys.executable, DRIVER] + list(arguments), cwd=REPO,
                                   stdout=sink, stderr=subprocess.STDOUT)
        handle_box["handle"] = int(process._handle)
        # One reading while the handle is certainly open, so a very short run still reports a figure.
        reading = working_set(handle_box["handle"])
        if reading:
            peak["working_set"] = max(peak["working_set"], reading[1])
        status = process.wait()
        final = working_set(handle_box["handle"])
        if final:
            peak["working_set"] = max(peak["working_set"], final[1])
    elapsed = time.perf_counter() - started
    stop.set()
    watcher.join(timeout=2)
    with open(out_log, "r", encoding="utf-8", errors="replace") as handle:
        log = handle.read()
    return status, log, elapsed, peak


# ==================================================================================================
# The default sweep's cost, at one job and at four
# ==================================================================================================


def case_default_sweep(report):
    lines = [
        f"platform: {platform.platform()}",
        f"processor: {platform.processor()}",
        f"logical processors: {os.cpu_count()}",
        f"python: {platform.python_version()} ({platform.python_implementation()})",
        f"engine: target/release/Mokiterions.exe, release profile",
        "",
    ]
    figures = {}
    outputs = {}
    for jobs in (1, 4):
        stream_dir = fresh(f"{OUT}/streams-jobs{jobs}")
        out_path = f"{OUT}/default-sweep-jobs{jobs}.jsonl"
        status, log, elapsed, peak = measured_batch(
            ["--out", out_path, "--stream-dir", stream_dir, "--jobs", str(jobs)],
            stream_dir, f"default-sweep-jobs{jobs}")
        size = os.path.getsize(out_path)
        records = read_jsonl(out_path)
        batch = records[-1]
        figures[jobs] = {
            "status": status,
            "wall_seconds": round(elapsed, 3),
            "retained_bytes": size,
            "cells": batch["requested"],
            "retained": batch["retained"],
            "complete": batch["complete"],
            "peak_stream_bytes": peak["bytes"],
            "peak_stream_files": peak["files"],
            "stream_samples": peak["samples"],
            "peak_working_set_bytes": peak["working_set"],
            "digest": sha256_whole(out_path),
            "leftover": os.listdir(stream_dir),
        }
        outputs[jobs] = out_path
        lines.append(f"--jobs {jobs}: " + json.dumps(figures[jobs]))

    # The classifier's cost, over the retained rows, in both formats and on both axes.
    scan = {}
    for label, options in (("source,density json", ["--format", "json"]),
                           ("source,density markdown", ["--format", "markdown"]),
                           ("seed json", ["--group-by", "seed", "--format", "json"])):
        started = time.perf_counter()
        status, out, err = run([sys.executable, CLASSIFIER, outputs[1]] + options)
        scan[label] = {"status": status, "wall_seconds": round(time.perf_counter() - started, 3),
                       "output_bytes": len(out)}
    lines.append("")
    lines.extend(f"classifier {label}: {json.dumps(value)}" for label, value in scan.items())

    identical = figures[1]["digest"] == figures[4]["digest"]
    prediction_lines = [
        "",
        "beside ADR-MOK-008's extrapolation from 20 runs (that record's table, rows 1 to 8):",
        f"  batch wall time   predicted ~{PREDICTED['batch wall seconds']} s   "
        f"measured {figures[1]['wall_seconds']} s at --jobs 1, {figures[4]['wall_seconds']} s at "
        f"--jobs 4",
        f"  scanning          predicted ~{PREDICTED['scan wall seconds']} s   "
        f"measured {scan['source,density json']['wall_seconds']} s",
        f"  retained bytes    predicted ~{PREDICTED['retained bytes']}        "
        f"measured {figures[1]['retained_bytes']}",
        f"  peak stream bytes predicted one stream, ~{PREDICTED['one 1,000-tick social stream bytes']}"
        f"   measured {figures[1]['peak_stream_bytes']} at --jobs 1 over "
        f"{figures[1]['stream_samples']} samples, {figures[4]['peak_stream_bytes']} at --jobs 4",
        f"  streams if kept   predicted ~{PREDICTED['streams if kept, bytes']} bytes; not kept, so "
        f"not measured",
    ]
    keep("cost.txt", "\n".join(lines + prediction_lines))

    report.record(
        "The default sweep's cost is measured and reported",
        figures[1]["status"] == 0 and figures[1]["complete"]
        and figures[1]["retained"] == 400 and all(v["status"] == 0 for v in scan.values()),
        f"400 of 400 cells retained at --jobs 1 in {figures[1]['wall_seconds']} s against a "
        f"~{PREDICTED['batch wall seconds']} s prediction, {figures[1]['retained_bytes']} bytes "
        f"retained against ~{PREDICTED['retained bytes']}, and the classifier scanned them in "
        f"{scan['source,density json']['wall_seconds']} s against a ~"
        f"{PREDICTED['scan wall seconds']} s prediction, on {platform.platform()}",
        note=(
            "The scanning prediction is the divergence, and it is a finding about the extrapolation "
            "rather than about the instrument. ADR-MOK-008's ~16 s is the cost of scanning 400 full "
            "event streams, which is what the throwaway script did; the instrument as specified "
            "scans each stream once inside the batch, as part of the cell, and the classifier then "
            "reads only the retained rows. So the ~16 s is inside the batch figure and the "
            "classifier's own cost is three orders of magnitude below the prediction. The "
            "prediction was of a different design, not of this one."
        ),
    )
    report.record(
        "P5 peak disk is one stream, over the default sweep",
        figures[1]["peak_stream_files"] <= 1
        and figures[1]["peak_stream_bytes"] <= PREDICTED["one 1,000-tick social stream bytes"] * 1.2
        and not figures[1]["leftover"] and not figures[4]["leftover"],
        f"over {figures[1]['stream_samples']} samples of the stream directory at 2 ms during a "
        f"400-cell batch, at most {figures[1]['peak_stream_files']} file(s) existed at once and the "
        f"peak was {figures[1]['peak_stream_bytes']} bytes -- against a whole sweep of roughly "
        f"{PREDICTED['streams if kept, bytes']} bytes -- and the directory is empty afterwards at "
        f"both job counts",
    )
    report.record(
        "B3, B27 and P2 hold under --jobs 4 on the default sweep",
        identical and figures[4]["status"] == 0 and figures[4]["retained"] == 400
        and figures[4]["peak_stream_files"] <= 4,
        f"the 400-cell default sweep at --jobs 4 produced output byte-identical to --jobs 1 "
        f"({figures[1]['digest'][:16]}..., {figures[1]['retained_bytes']} bytes) in "
        f"{figures[4]['wall_seconds']} s against {figures[1]['wall_seconds']} s, with at most "
        f"{figures[4]['peak_stream_files']} concurrent streams and peak "
        f"{figures[4]['peak_stream_bytes']} bytes",
    )
    return outputs[1], figures


# ==================================================================================================
# Scanning cost against stream bytes, at two horizons
# ==================================================================================================


def case_linearity(report):
    """Two horizons, and the engine's execution held out of the comparison as far as it can be.

    A batch's wall time is the engine's execution plus the driver's scan. Comparing two horizons of
    real cells measures the sum, and the engine's own cost also grows with the horizon, so the sum
    alone cannot say whether the scan is linear. The stand-in binary writes a prepared stream, so a
    batch against it holds the engine's part still and varies only the bytes scanned.

    It does not hold it at zero, and the first attempt at this measurement got that wrong: the
    stand-in is a Python process behind a .bat, so each cell carries roughly 0.2 s of interpreter
    startup, and over ten cells that constant was most of the batch. Measured that way the wall time
    grew 1.42x while the bytes grew 3.17x, which reads as sublinear and is really a constant term
    swamping a linear one. So the constant is measured directly, at a third and much shorter horizon,
    and subtracted; what is then compared is the marginal cost per megabyte at the two real horizons,
    which is the figure the bullet is about. The second attempt got the constant wrong too, by taking
    it from the `small.jsonl` fixture at `--ticks 0` -- a horizon the driver refuses before executing
    anything, so the row timed a start-up and not ten cells. It is a 10-tick engine stream now.
    """
    plans = {}
    prepared = {}
    # The third point, and the one that makes the other two mean anything: a horizon whose stream is
    # small enough that its scan cost is negligible, so the batch's wall time is ten per-cell
    # constants. The first attempt used the `small.jsonl` fixture at `--ticks 0`, which the driver
    # refuses in argument validation -- it exited 2 in 0.111 s having executed no cell at all, so the
    # "constant" was one driver start-up rather than ten. A real 10-tick stream at `--ticks 10` runs
    # the same code path as the other two rows.
    for horizon in (10, 1000, 4000):
        path = f"{OUT}/scan-stream-{horizon}.jsonl"
        run([f"{REPO}/target/release/Mokiterions.exe", "--seed", "0", "--ticks", str(horizon),
             "--policy", "social", "--density", "1.00", "--events-path", path])
        prepared[horizon] = (path, os.path.getsize(path))

    # The stand-in reads its instructions from shim-plan.json, so the plan is rewritten per horizon.
    plan_path = f"{WORK}/shim-plan.json"
    with open(plan_path, "r", encoding="utf-8") as handle:
        original = handle.read()

    rows = []
    try:
        for horizon, (stream, size) in prepared.items():
            with open(plan_path, "w", encoding="utf-8", newline="\n") as handle:
                json.dump({"*": {"stream": os.path.relpath(stream, WORK).replace("\\", "/")}},
                          handle)
            stream_dir = fresh(f"{OUT}/streams-scan-{horizon}")
            status, log, elapsed, peak = measured_batch(
                ["--out", f"{OUT}/scan-{horizon}.jsonl", "--binary", f"{WORK}/shim.bat",
                 "--stream-dir", stream_dir, "--sources", "reference", "--densities", "0.75",
                 "--seeds", "0-9", "--ticks", str(horizon)],
                stream_dir, f"scan-{horizon}")
            plans[horizon] = {
                "stream_bytes": size, "cells": 10, "status": status,
                "wall_seconds": round(elapsed, 4),
                "bytes_scanned": size * 10,
                "seconds_per_megabyte": round(elapsed / (size * 10 / 1_048_576), 5),
            }
            rows.append(f"stand-in, {horizon} ticks: {json.dumps(plans[horizon])}")
    finally:
        with open(plan_path, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(original)

    real = {}
    for horizon in (250, 1000):
        stream_dir = fresh(f"{OUT}/streams-real-{horizon}")
        status, log, elapsed, peak = measured_batch(
            ["--out", f"{OUT}/real-{horizon}.jsonl", "--stream-dir", stream_dir,
             "--sources", "social", "--densities", "1.00", "--seeds", "0-9",
             "--ticks", str(horizon)],
            stream_dir, f"real-{horizon}")
        total = sum(row.get("run_ticks", 0) for row in read_jsonl(f"{OUT}/real-{horizon}.jsonl")
                    if row.get("record") == "cell")
        real[horizon] = {"status": status, "wall_seconds": round(elapsed, 4), "run_ticks": total,
                         "peak_stream_bytes": peak["bytes"]}
        rows.append(f"real engine, {horizon} ticks over 10 cells: {json.dumps(real[horizon])}")

    constant, small, large = plans[10], plans[1000], plans[4000]
    marginal = {}
    for label, point in (("1,000 ticks", small), ("4,000 ticks", large)):
        megabytes = (point["bytes_scanned"] - constant["bytes_scanned"]) / 1_048_576
        marginal[label] = round(
            (point["wall_seconds"] - constant["wall_seconds"]) / megabytes, 5)
    ratio_bytes = large["stream_bytes"] / small["stream_bytes"]
    ratio_time = large["wall_seconds"] / small["wall_seconds"]
    spread = (max(marginal.values()) / min(marginal.values())) if min(marginal.values()) > 0 else 0

    keep("scanning-linearity.txt", "\n".join(
        ["Scanning cost against stream bytes, at two horizons plus a constant-term measurement.", "",
         "The stand-in writes a prepared stream, so the engine's part of a cell is held still and",
         "only the bytes scanned vary. It is a Python process behind a .bat, so each cell carries an",
         "interpreter startup; the 10-tick row measures that constant, and it is subtracted before",
         "the two horizons are compared. 10 cells per row.", ""]
        + rows
        + ["",
           f"per-cell constant    {constant['wall_seconds'] / 10:.4f} s, from a "
           f"{constant['stream_bytes']}-byte stream",
           f"stream bytes ratio   {ratio_bytes:.3f}  ({small['stream_bytes']} -> "
           f"{large['stream_bytes']})",
           f"raw wall time ratio  {ratio_time:.3f}  ({small['wall_seconds']} s -> "
           f"{large['wall_seconds']} s)  -- the constant is still in this one",
           "",
           "marginal cost, with the constant subtracted:",
           ] + [f"  {label:12} {value} s per megabyte" for label, value in marginal.items()]
        + [f"  spread       {spread:.3f}x",
           "",
           "A linear scan makes the two marginal figures equal. A quadratic one makes the second",
           "the byte ratio times the first -- here that would be about "
           f"{marginal['1,000 ticks'] * ratio_bytes:.4f} s per megabyte rather than "
           f"{marginal['4,000 ticks']}. Neither figure is a threshold; both are retained so a later",
           "regression has a baseline."]))

    # Within a factor of two of each other. That distinguishes linear from quadratic at a threefold
    # size difference -- quadratic would put them a factor of three apart -- without asserting a
    # constant, which the contract's own words forbid.
    report.record(
        "Scanning cost is linear in stream bytes",
        all(p["status"] == 0 for p in (constant, small, large)) and 0.5 <= spread <= 2.0,
        f"with the {constant['wall_seconds'] / 10:.3f} s per-cell constant measured on a "
        f"{constant['stream_bytes']}-byte stream and subtracted, the marginal cost is "
        f"{marginal['1,000 ticks']} s per megabyte at 1,000 ticks and {marginal['4,000 ticks']} at "
        f"4,000 -- a spread of {spread:.2f}x over a {ratio_bytes:.2f}x change in stream size, where "
        f"a quadratic scan would have put them {ratio_bytes:.2f}x apart",
    )
    return prepared


# ==================================================================================================
# A long run, and the driver's memory
# ==================================================================================================


def case_long_run(report):
    """One long horizon for the bullet, a short one for comparison, and an isolated memory probe.

    The bullet asks for a stream materially larger than ADR-MOK-008's measured 2,723,014 bytes and
    for the memory high-water to be reported. The first attempt at this case also *asserted* that
    the batch's peak working set was below the stream's size, and that assertion is wrong: a bare
    CPython interpreter's working set is about 27 MB, so a 12.3 MB stream fails a test the driver
    passes on the merits. Nothing about the interpreter's own baseline is a claim of this work order.

    The second attempt compared the *change* in the batch's peak working set against the change in
    stream bytes across two horizons, and measured 0.60 added bytes of working set per added stream
    byte. That is well under a whole-file read but too close to it to leave as the whole answer, and
    a batch process is a poor instrument for the question: it also starts an interpreter, spawns an
    engine and captures the engine's output. So the shape is measured where the claim actually lives,
    by calling the driver's own `scan_file` in a fresh process per stream at three sizes, one of them
    a stream longer than any this engine will produce before the population dies. Both readings are
    retained; the pass condition is on the isolated one.
    """
    measurements = {}
    for horizon in (1000, 12000):
        stream_dir = fresh(f"{OUT}/streams-long-{horizon}")
        out_path = f"{OUT}/long-run-{horizon}.jsonl"
        status, log, elapsed, peak = measured_batch(
            ["--out", out_path, "--stream-dir", stream_dir, "--sources", "social",
             "--densities", "1.00", "--seeds", "0", "--ticks", str(horizon),
             "--keep-stream", "social:1.00:0"],
            stream_dir, f"long-run-{horizon}")
        kept = sorted(glob.glob(f"{stream_dir}/kept-*.jsonl"))
        size = os.path.getsize(kept[0]) if kept else 0
        row = [r for r in read_jsonl(out_path) if r.get("record") == "cell"]
        measurements[horizon] = {
            "status": status, "wall_seconds": round(elapsed, 3), "stream_bytes": size,
            "run_ticks": row[0]["run_ticks"] if row else None,
            "reason": row[0]["reason"] if row else None,
            "peak_working_set": peak["working_set"], "memory_samples": peak["memory_samples"],
            "log": log,
        }

    short, long = measurements[1000], measurements[12000]
    delta_bytes = long["stream_bytes"] - short["stream_bytes"]
    delta_peak = long["peak_working_set"] - short["peak_working_set"]
    share = (delta_peak / delta_bytes) if delta_bytes else 0

    # The isolated readings. The third stream is the 12,000-tick one doubled: this engine's population
    # at density 1.00 dies at about 9,800 ticks, so no single run reaches 25 MB, and the question here
    # is only how `scan_file`'s memory answers to bytes. A doubled stream carries two header records
    # and two run records, which `scan_file` accepts -- it keeps the last of each -- and which no cell
    # row is derived from. It is an input to a memory measurement and to nothing else.
    kept_long = sorted(glob.glob(f"{OUT}/streams-long-12000/kept-*.jsonl"))
    kept_short = sorted(glob.glob(f"{OUT}/streams-long-1000/kept-*.jsonl"))
    doubled = f"{OUT}/doubled-stream.jsonl"
    if kept_long:
        with open(doubled, "wb") as sink:
            for _ in range(2):
                with open(kept_long[0], "rb") as source:
                    shutil.copyfileobj(source, sink)

    probes = []
    for path in [p for p in (kept_short[:1] + kept_long[:1] + [doubled]) if os.path.isfile(p)]:
        completed = subprocess.run(
            [sys.executable, f"{WORK}/scan_memory_probe.py", path],
            capture_output=True, text=True)
        probes.append(json.loads(completed.stdout) if completed.returncode == 0
                      else {"stream_bytes": os.path.getsize(path), "error": completed.stderr.strip(),
                            "added_per_stream_byte": None})

    isolated_ok = (
        len(probes) == 3
        and all(probe.get("added_per_stream_byte") is not None for probe in probes)
        # Literally what "not reading the file whole" means: the scan adds less memory than the file
        # has bytes. A whole-file read would add at least the bytes; a whole-file *parse* several
        # times more, because a parsed record is larger than its own JSON text.
        and all(probe["added_per_stream_byte"] < 1.0 for probe in probes)
        # And the share falls as the stream grows, which is what a fixed working set looks like when
        # it is divided by a growing denominator.
        and probes[-1]["added_per_stream_byte"] <= probes[0]["added_per_stream_byte"]
    )

    keep("long-run.txt", "\n".join([
        "A stream materially larger than ADR-MOK-008's measured 2,723,014 bytes, to establish that",
        "the driver scans rather than loading a file into memory.", "",
        "one cell each, social at density 1.00, seed 0, at two horizons:", ""]
        + [f"  {horizon:>6} ticks: " + json.dumps(
            {k: v for k, v in figures.items() if k != "log"})
           for horizon, figures in measurements.items()]
        + ["",
           f"  ADR-MOK-008's 1,000-tick figure   {LONG_RUN_FLOOR}",
           f"  long horizon against it           {long['stream_bytes'] / LONG_RUN_FLOOR:.2f}x",
           f"  stream bytes added                {delta_bytes}",
           f"  peak working set added            {delta_peak}",
           f"  added memory per added byte       {share:.4f}",
           "",
           "The working set is the driver's own process, sampled at 20 ms through",
           "GetProcessMemoryInfo on its process handle. The engine's memory is a separate process",
           "and is not the claim: what is being checked is that the process that reads the stream",
           "does not grow with it. That figure is reported and not tested, for two reasons. A bare",
           "CPython interpreter is already about 27 MB, which is larger than the long horizon's",
           "stream and is nothing to do with this driver; and a batch process does more than scan --",
           "it starts an interpreter, spawns an engine and captures the engine's output -- so its",
           "high-water answers the question only indirectly.",
           "",
           "The scan in isolation. One fresh process per stream, importing the repository's own",
           "scripts/run_simulation_batch.py and calling scan_file once; the working set is read after",
           "the import and before the scan, and the process's lifetime peak after it.", ""]
        + [f"  {json.dumps(probe)}" for probe in probes]
        + ["",
           "The third stream is the 12,000-tick one doubled, because this engine's population at",
           "density 1.00 dies at about 9,800 ticks and no single run reaches 25 MB. It carries two",
           "header records and two run records, which scan_file accepts and keeps the last of; it is",
           "an input to this memory measurement and to nothing else, and no cell row comes from it.",
           "",
           "A reader that held the file would add at least the stream's bytes to its high-water, and",
           "one that parsed it whole several times more, since a parsed record is larger than its own",
           "JSON text. What the three rows show instead is a roughly fixed addition, so the added",
           "bytes per stream byte fall as the stream grows. Nothing here is a threshold except the",
           "1.0 that separates scanning from holding, which is not chosen but is what the words mean.",
           "",
           short["log"], "", long["log"]]))

    report.record(
        "A long run, and the driver's memory high-water",
        all(figures["status"] == 0 for figures in measurements.values())
        and long["stream_bytes"] > LONG_RUN_FLOOR
        and delta_bytes > 0 and isolated_ok,
        f"one 12,000-tick cell produced a {long['stream_bytes']}-byte stream, "
        f"{long['stream_bytes'] / LONG_RUN_FLOOR:.2f}x ADR-MOK-008's measured 2,723,014, in "
        f"{long['wall_seconds']} s -- the run ended at {long['run_ticks']} ticks by "
        f"{long['reason']}; the batch's own peak working set was {long['peak_working_set']} bytes "
        f"over {long['memory_samples']} samples against {short['peak_working_set']} at 1,000 ticks, "
        f"which is {share:.4f} per added stream byte. In isolation scan_file added "
        + ", ".join(f"{probe['added_per_stream_byte']} per byte at {probe['stream_bytes']}"
                    for probe in probes)
        + " -- a falling share of a growing stream, so the file is scanned and not held",
    )


# ==================================================================================================
# Interruption
# ==================================================================================================


INTERRUPT_LAUNCHER = '''
"""Delivers the operator's interrupt to the driver, on a platform that cannot send one directly.

Windows disables CTRL_C_EVENT for a process group created with CREATE_NEW_PROCESS_GROUP, and sending
it to the shared console would interrupt the measuring process too. So the parent sends
CTRL_BREAK_EVENT to the driver's own group, and this launcher translates it into the KeyboardInterrupt
that an operator pressing Ctrl+C in a console raises. Nothing else about the run changes: the driver
is executed under its own __main__ with its own argument vector.
"""
import runpy
import signal
import sys


def raise_keyboard_interrupt(signum, frame):
    raise KeyboardInterrupt


signal.signal(signal.SIGBREAK, raise_keyboard_interrupt)
driver = sys.argv[1]
sys.argv = [driver] + sys.argv[2:]
runpy.run_path(driver, run_name="__main__")
'''


def case_interruption(report):
    launcher = f"{OUT}/interrupt_launcher.py"
    os.makedirs(OUT, exist_ok=True)
    with open(launcher, "w", encoding="utf-8", newline="\n") as handle:
        handle.write(INTERRUPT_LAUNCHER.lstrip())

    stream_dir = fresh(f"{OUT}/streams-interrupted")
    out_path = f"{OUT}/interrupted.jsonl"
    log_path = f"{OUT}/interrupted.log"
    with open(log_path, "wb") as sink:
        process = subprocess.Popen(
            [sys.executable, launcher, DRIVER, "--out", out_path, "--stream-dir", stream_dir,
             "--sources", "baseline,reference,individual,social", "--densities", "0.75",
             "--seeds", "0-19", "--ticks", "1000"],
            cwd=REPO, stdout=sink, stderr=subprocess.STDOUT,
            creationflags=subprocess.CREATE_NEW_PROCESS_GROUP)
        time.sleep(3.0)
        os.kill(process.pid, signal.CTRL_BREAK_EVENT)
        status = process.wait(timeout=120)
    with open(log_path, "r", encoding="utf-8", errors="replace") as handle:
        log = handle.read()

    records = read_jsonl(out_path) if os.path.isfile(out_path) else []
    sweep = next((r for r in records if r["record"] == "sweep"), None)
    batch = next((r for r in records if r["record"] == "batch"), None)
    cells = [r for r in records if r["record"] == "cell"]
    stages = {}
    for entry in (batch or {}).get("missing", []):
        stages[entry["stage"]] = stages.get(entry["stage"], 0) + 1
    leftover_on_disk = sorted(os.listdir(stream_dir))

    keep("interruption.txt", "\n".join([
        "A 80-cell batch interrupted 3 s in, by CTRL_BREAK_EVENT to its own process group,",
        "translated to KeyboardInterrupt by the launcher retained beside this file.", "",
        f"  exit                  {status}",
        f"  sweep record written  {sweep is not None}",
        f"  batch record written  {batch is not None}",
        f"  cell records          {len(cells)}",
        f"  requested / retained  {(batch or {}).get('requested')} / "
        f"{(batch or {}).get('retained')}",
        f"  complete              {(batch or {}).get('complete')}",
        f"  missing by stage      {stages}",
        f"  leftover_streams      {(batch or {}).get('leftover_streams')}",
        f"  files left on disk    {leftover_on_disk or 'none'}",
        "",
        "the batch record as written:",
        json.dumps(batch, indent=1) if batch else "  none",
        "",
        "the driver's own report:",
        log,
    ]))

    not_attempted = stages.get("not_attempted", 0)
    faults = []
    if sweep is None or batch is None:
        faults.append("a record is missing from the retained output")
    if not_attempted == 0:
        faults.append("no cell was named not_attempted")
    if batch and batch["complete"]:
        faults.append("the batch called itself complete")
    if batch and batch["retained"] + len(batch["missing"]) != batch["requested"]:
        faults.append("P1 fails: retained plus missing does not equal requested")
    if leftover_on_disk:
        faults.append(f"streams left on disk: {leftover_on_disk}")

    report.record(
        "Interruption",
        not faults,
        f"a 80-cell batch interrupted 3 s in exited {status} having written its sweep and batch "
        f"records and {len(cells)} cell records; {not_attempted} cells are named not_attempted, "
        f"completeness is {(batch or {}).get('complete')}, missing by stage is {stages}, and "
        f"{len(leftover_on_disk)} stream(s) remain on disk; faults {faults or 'none'}",
    )


# ==================================================================================================


def main():
    report = Report("phase E: performance and resilience")
    shutil.rmtree(OUT, ignore_errors=True)
    os.makedirs(OUT, exist_ok=True)
    case_default_sweep(report)
    case_linearity(report)
    case_long_run(report)
    case_interruption(report)
    return 0 if report.write(f"{OUT}/result.json") else 1


if __name__ == "__main__":
    sys.exit(main())
