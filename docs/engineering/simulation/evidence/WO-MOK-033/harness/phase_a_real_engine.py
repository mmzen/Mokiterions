#!/usr/bin/env python3
"""VER-MOK-019 phase A: the cases that run the real engine end to end.

B1, B2, B7, B8, B9, B10, B12, B13 (real half), B14, B16, B18, Q1 over B1's rows, and P1, P2, P5, P6,
P7. Contract *Independence* point 3 is why this phase exists: the instruments' own tests are on
fixtures by rule 20.2, and a fixture corpus can encode a stream shape the engine does not emit.
"""

from __future__ import annotations

import json
import os
import shutil
import threading
import time

from oracle import (
    DEBUG, RELEASE, WORK, Report, classifier, driver, engine, floats_in, oracle_classify,
    oracle_crosscheck, oracle_scan, read_jsonl, row_shape_faults, sha256_whole,
)

OUT = WORK + "/phase-a"
SOURCES = "reference,social"
DENSITIES = "0.50,0.75"
SEEDS = "0-1"
TICKS = 1000
CELLS = [
    (source, density, seed)
    for source in SOURCES.split(",")
    for density in DENSITIES.split(",")
    for seed in (0, 1)
]


def fresh(path):
    shutil.rmtree(path, ignore_errors=True)
    os.makedirs(path, exist_ok=True)
    return path


def kept_path(stream_dir, cell):
    source, density, seed = cell
    return f"{stream_dir}/kept-{source}-{density}-{seed}.jsonl"


def main():
    report = Report("phase A: real engine")
    fresh(OUT)
    streams = fresh(OUT + "/streams")

    # ----------------------------------------------------------------------------------------------
    # B1: a real sweep of eight cells, every row conforming to rule 8.1
    # ----------------------------------------------------------------------------------------------
    retained = OUT + "/b1.jsonl"
    keep_args = []
    for source, density, seed in CELLS:
        keep_args += ["--keep-stream", f"{source}:{density}:{seed}"]
    status, stdout, stderr = driver(
        ["--out", retained, "--binary", RELEASE, "--sources", SOURCES, "--densities", DENSITIES,
         "--seeds", SEEDS, "--ticks", str(TICKS), "--stream-dir", streams] + keep_args
    )
    with open(OUT + "/b1-transcript.txt", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(f"status {status}\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}\n")

    records = read_jsonl(retained)
    sweep = [record for record in records if record["record"] == "sweep"]
    rows = [record for record in records if record["record"] == "cell"]
    batch = [record for record in records if record["record"] == "batch"]

    faults = []
    for row in rows:
        for fault in row_shape_faults(row):
            faults.append(f"{row['source']}:{row['density']}:{row['seed']} {fault}")
    coordinates = [(row["source"], row["density"], row["seed"]) for row in rows]
    report.record(
        "B1 real-engine conformance",
        status == 0 and len(rows) == 8 and coordinates == CELLS and not faults,
        f"status {status}, {len(rows)} of 8 rows, order {'as rule 4.3' if coordinates == CELLS else coordinates}, "
        f"shape faults {faults or 'none'}",
    )

    # ----------------------------------------------------------------------------------------------
    # P1: one row per completed cell
    # ----------------------------------------------------------------------------------------------
    batch_record = batch[0]
    report.record(
        "P1 one row per completed cell",
        len(rows) == batch_record["retained"]
        and batch_record["retained"] + len(batch_record["missing"]) == batch_record["requested"]
        and batch_record["requested"] == sweep[0]["cells"],
        f"cell records {len(rows)}, retained {batch_record['retained']}, missing "
        f"{len(batch_record['missing'])}, requested {batch_record['requested']}, "
        f"sweep cells {sweep[0]['cells']}",
    )

    # ----------------------------------------------------------------------------------------------
    # P7 and T9: no floating-point value in any retained byte
    # ----------------------------------------------------------------------------------------------
    found = []
    for index, record in enumerate(records):
        found.extend(f"record {index}{path[1:]}" for path in floats_in(record))
    with open(retained, "rb") as handle:
        raw_retained = handle.read()
    report.record(
        "P7/T9 integers only in retained output",
        not found,
        f"{len(records)} records scanned, floats {found or 'none'}",
    )

    # ----------------------------------------------------------------------------------------------
    # B9: the digest is over the file's raw bytes, computed by this contract's own code
    # ----------------------------------------------------------------------------------------------
    scans = {}
    digest_faults = []
    for cell in CELLS:
        path = kept_path(streams, cell)
        if not os.path.isfile(path):
            digest_faults.append(f"{cell} stream was not kept")
            continue
        scans[cell] = oracle_scan(path)
    for row in rows:
        cell = (row["source"], row["density"], row["seed"])
        if cell not in scans:
            continue
        if scans[cell]["digest"] != row["stream_sha256"]:
            digest_faults.append(f"{cell} {scans[cell]['digest']} != {row['stream_sha256']}")
    report.record(
        "B9 hashed as written",
        not digest_faults and len(scans) == 8,
        f"{len(scans)} streams digested independently over raw bytes, disagreements "
        f"{digest_faults or 'none'}",
    )

    # B9's second half: a stream altered by one byte digests differently.
    sample = kept_path(streams, CELLS[0])
    with open(sample, "rb") as handle:
        original = handle.read()
    altered = OUT + "/b9-altered.jsonl"
    position = len(original) // 2
    mutated = bytearray(original)
    mutated[position] = mutated[position] ^ 0x01
    with open(altered, "wb") as handle:
        handle.write(bytes(mutated))
    report.record(
        "B9 one altered byte changes the digest",
        sha256_whole(altered) != sha256_whole(sample) and len(mutated) == len(original),
        f"byte {position} flipped in {len(original)} bytes; digest {sha256_whole(sample)[:16]}... "
        f"became {sha256_whole(altered)[:16]}...",
    )

    # ----------------------------------------------------------------------------------------------
    # B12, B13 real half, B14: independent counts
    # ----------------------------------------------------------------------------------------------
    instrument_table = []
    oracle_table = []
    count_faults = []
    for row in rows:
        cell = (row["source"], row["density"], row["seed"])
        scan = scans[cell]
        instrument_table.append({
            "cell": f"{cell[0]}:{cell[1]}:{cell[2]}",
            "attempted": row["attempted"],
            "effective": row["effective"],
            "lethal_attacks": row["lethal_attacks"],
        })
        oracle_table.append({
            "cell": f"{cell[0]}:{cell[1]}:{cell[2]}",
            "attempted": scan["attempted"],
            "effective": scan["effective"],
            "lethal_attacks": scan["lethal_attacks"],
        })
        if row["attempted"] != scan["attempted"]:
            count_faults.append(f"{cell} attempted {row['attempted']} != {scan['attempted']}")
        if row["effective"] != scan["effective"]:
            count_faults.append(f"{cell} effective {row['effective']} != {scan['effective']}")
        if row["lethal_attacks"] != scan["lethal_attacks"]:
            count_faults.append(
                f"{cell} lethal_attacks {row['lethal_attacks']} != {scan['lethal_attacks']}"
            )
    with open(OUT + "/b12-b14-counts.json", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(json.dumps(
            {"instrument": instrument_table, "oracle": oracle_table,
             "diff": count_faults or "no disagreement"},
            ensure_ascii=False, indent=1) + "\n")
    report.record(
        "B12 independent attempted counts",
        not any("attempted" in fault for fault in count_faults),
        f"three kinds over {len(rows)} real streams; disagreements "
        f"{[f for f in count_faults if 'attempted' in f] or 'none'}",
    )
    report.record(
        "B13 independent effective counts (real half)",
        not any("effective" in fault for fault in count_faults),
        f"rule 9.2's three predicates recomputed; disagreements "
        f"{[f for f in count_faults if 'effective' in f] or 'none'}",
        note="the fixture half -- damage 0, increase 0, transferred 0 -- is phase B",
    )
    report.record(
        "B14 lethality is not a narrowing",
        not any("lethal" in fault for fault in count_faults),
        f"lethal_attacks recomputed as target_died == 'yes'; disagreements "
        f"{[f for f in count_faults if 'lethal' in f] or 'none'}",
    )

    # ----------------------------------------------------------------------------------------------
    # P6: attempted bounds effective, and effective bounds lethality
    # ----------------------------------------------------------------------------------------------
    bound_faults = []
    for row in rows:
        for kind in row["attempted"]:
            if row["effective"][kind] > row["attempted"][kind]:
                bound_faults.append(f"{row['source']}:{row['seed']} {kind} effective > attempted")
        if row["lethal_attacks"] > row["effective"]["attack_resolved"]:
            bound_faults.append(f"{row['source']}:{row['seed']} lethal > effective attacks")
    report.record(
        "P6 attempted bounds effective",
        not bound_faults,
        f"checked over {len(rows)} rows; violations {bound_faults or 'none'}",
    )

    # ----------------------------------------------------------------------------------------------
    # B16: the seven cross-checks, independently recomputed
    # ----------------------------------------------------------------------------------------------
    crosscheck_lines = []
    crosscheck_faults = []
    for row in rows:
        cell = (row["source"], row["density"], row["seed"])
        results = oracle_crosscheck(scans[cell])
        crosscheck_lines.append({
            "cell": f"{cell[0]}:{cell[1]}:{cell[2]}",
            "instrument_crosschecks_passed": row["crosschecks_passed"],
            "oracle": [{"check": name, "left": left, "right": right, "equal": ok}
                       for name, left, right, ok in results],
        })
        if row["crosschecks_passed"] != 7:
            crosscheck_faults.append(f"{cell} crosschecks_passed {row['crosschecks_passed']}")
        for name, left, right, ok in results:
            if not ok:
                crosscheck_faults.append(f"{cell} {name}: {left} != {right}")
    with open(OUT + "/b16-crosschecks.json", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(json.dumps(crosscheck_lines, ensure_ascii=False, indent=1) + "\n")
    report.record(
        "B16 the seven cross-checks pass",
        not crosscheck_faults,
        f"7 equalities recomputed over {len(rows)} real streams = {7 * len(rows)} comparisons; "
        f"disagreements {crosscheck_faults or 'none'}",
    )

    # ----------------------------------------------------------------------------------------------
    # B7: the digest reproduces, from both build profiles
    # ----------------------------------------------------------------------------------------------
    reproduction = []
    reproduce_faults = []
    for row in rows:
        cell = (row["source"], row["density"], row["seed"])
        entry = {"cell": f"{cell[0]}:{cell[1]}:{cell[2]}", "retained": row["stream_sha256"]}
        for profile, binary in (("release", RELEASE), ("debug", DEBUG)):
            path = f"{OUT}/repro-{profile}.jsonl"
            code, _, err = engine(binary, cell[2], TICKS, cell[0], cell[1], path)
            if code != 0:
                reproduce_faults.append(f"{cell} {profile} exited {code}: {err.strip()[:120]}")
                continue
            digest = sha256_whole(path)
            entry[profile] = digest
            if digest != row["stream_sha256"]:
                reproduce_faults.append(f"{cell} {profile} {digest} != {row['stream_sha256']}")
        reproduction.append(entry)
    with open(OUT + "/b7-reproduction.json", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(json.dumps(reproduction, ensure_ascii=False, indent=1) + "\n")
    report.record(
        "B7 the digest reproduces from both profiles",
        not reproduce_faults,
        f"{len(rows)} cells re-executed from coordinates alone under release and debug; "
        f"failures {reproduce_faults or 'none'}",
    )

    # ----------------------------------------------------------------------------------------------
    # B2: transparency to the engine
    # ----------------------------------------------------------------------------------------------
    cell = CELLS[0]
    hand_one = OUT + "/b2-hand-1.jsonl"
    hand_two = OUT + "/b2-hand-2.jsonl"
    code_one, out_one, _ = engine(RELEASE, cell[2], TICKS, cell[0], cell[1], hand_one)
    code_two, out_two, _ = engine(RELEASE, cell[2], TICKS, cell[0], cell[1], hand_two)
    with open(kept_path(streams, cell), "rb") as handle:
        batch_bytes = handle.read()
    with open(hand_one, "rb") as handle:
        hand_bytes = handle.read()
    report.record(
        "B2 transparency to the engine",
        code_one == 0 and code_two == 0 and hand_bytes == batch_bytes and out_one == out_two,
        f"the batch's stream and the hand run's stream are byte-identical over {len(hand_bytes)} "
        f"bytes; the hand run's standard output is byte-identical across two executions "
        f"({len(out_one)} bytes)",
        note=(
            "met in part by substitution: rule 5.2 has the driver read the child's standard output "
            "and drop it, so the batch child's standard output is not observable from outside the "
            "driver and cannot be compared byte for byte. The observable half -- the stream -- is "
            "compared, and the engine's standard output is shown reproducible for the same arguments."
        ),
    )

    # ----------------------------------------------------------------------------------------------
    # B18: tracing independence, over real streams
    # ----------------------------------------------------------------------------------------------
    traced = OUT + "/b18-traced.jsonl"
    code_traced, _, _ = engine(
        RELEASE, cell[2], TICKS, cell[0], cell[1], traced, extra=["--trace-actions"]
    )
    traced_scan = oracle_scan(traced)
    plain_scan = oracle_scan(hand_one)
    same = (
        traced_scan["attempted"] == plain_scan["attempted"]
        and traced_scan["effective"] == plain_scan["effective"]
        and traced_scan["lethal_attacks"] == plain_scan["lethal_attacks"]
        and traced_scan["run"] == plain_scan["run"]
    )
    report.record(
        "B18 tracing independence",
        code_traced == 0 and same,
        f"--trace-actions present: {traced_scan['records']} records, {traced_scan['bytes']} bytes; "
        f"absent: {plain_scan['records']} records, {plain_scan['bytes']} bytes; every derived count "
        f"and the whole run record {'identical' if same else 'DIFFER'}",
    )

    # ----------------------------------------------------------------------------------------------
    # B10: --keep-stream changes no row
    # ----------------------------------------------------------------------------------------------
    plain_streams = fresh(OUT + "/streams-plain")
    plain_out = OUT + "/b10-plain.jsonl"
    status_plain, _, _ = driver(
        ["--out", plain_out, "--binary", RELEASE, "--sources", SOURCES, "--densities", DENSITIES,
         "--seeds", SEEDS, "--ticks", str(TICKS), "--stream-dir", plain_streams]
    )
    with open(plain_out, "rb") as handle:
        plain_bytes = handle.read()
    report.record(
        "B10 --keep-stream changes no row",
        status_plain == 0 and plain_bytes == raw_retained,
        f"the eight-cell batch with eight kept streams and the same batch with none produced "
        f"{'byte-identical' if plain_bytes == raw_retained else 'DIFFERING'} retained output "
        f"({len(raw_retained)} bytes)",
    )

    # ----------------------------------------------------------------------------------------------
    # B8: the stream is gone
    # ----------------------------------------------------------------------------------------------
    leftover = sorted(os.listdir(plain_streams))
    report.record(
        "B8 the stream is gone",
        not leftover and not batch_record["leftover_streams"],
        f"the stream directory of a batch with no --keep-stream holds {leftover or 'nothing'} after "
        f"eight cells; the batch record's leftover_streams is "
        f"{batch_record['leftover_streams'] or 'empty'}",
    )

    # ----------------------------------------------------------------------------------------------
    # B8 and P5: peak stream bytes never exceed one stream, over a 20-cell batch
    # ----------------------------------------------------------------------------------------------
    peak_streams = fresh(OUT + "/streams-peak")
    peak = {"bytes": 0, "files": 0, "samples": 0}
    stop = threading.Event()

    def watch():
        while not stop.is_set():
            total = 0
            names = []
            try:
                names = os.listdir(peak_streams)
                for name in names:
                    try:
                        total += os.path.getsize(os.path.join(peak_streams, name))
                    except OSError:
                        pass
            except OSError:
                pass
            peak["bytes"] = max(peak["bytes"], total)
            peak["files"] = max(peak["files"], len(names))
            peak["samples"] += 1
            time.sleep(0.002)

    watcher = threading.Thread(target=watch, daemon=True)
    watcher.start()
    status_peak, _, _ = driver(
        ["--out", OUT + "/b8-peak.jsonl", "--binary", RELEASE, "--sources", "reference",
         "--densities", "0.75", "--seeds", "0-19", "--ticks", str(TICKS),
         "--stream-dir", peak_streams]
    )
    stop.set()
    watcher.join(timeout=5)

    # The baseline has to be the largest stream of *this* sweep, so the same 20 cells are run again
    # with every stream kept, in a directory the peak measurement never saw.
    sizes_dir = fresh(OUT + "/streams-peak-sizes")
    size_keep = []
    for seed in range(20):
        size_keep += ["--keep-stream", f"reference:0.75:{seed}"]
    driver(
        ["--out", OUT + "/b8-peak-sizes.jsonl", "--binary", RELEASE, "--sources", "reference",
         "--densities", "0.75", "--seeds", "0-19", "--ticks", str(TICKS),
         "--stream-dir", sizes_dir] + size_keep
    )
    kept_sizes = [
        os.path.getsize(os.path.join(sizes_dir, name))
        for name in os.listdir(sizes_dir)
        if name.startswith("kept-")
    ]
    one_stream = max(kept_sizes)
    report.record(
        "B8/P5 peak disk is one stream",
        status_peak == 0 and peak["files"] <= 1 and peak["bytes"] <= one_stream,
        f"20 cells at --jobs 1, {peak['samples']} samples at 2 ms: peak {peak['files']} file(s) and "
        f"{peak['bytes']} bytes on disk, against the largest of the same sweep's {len(kept_sizes)} "
        f"streams at {one_stream} bytes (smallest {min(kept_sizes)})",
    )

    # ----------------------------------------------------------------------------------------------
    # P2 and B27: a row is a function of its cell, at two job counts
    # ----------------------------------------------------------------------------------------------
    concurrent_out = OUT + "/p2-jobs4.jsonl"
    concurrent_streams = fresh(OUT + "/streams-jobs4")
    status_jobs, _, _ = driver(
        ["--out", concurrent_out, "--binary", RELEASE, "--sources", SOURCES, "--densities",
         DENSITIES, "--seeds", SEEDS, "--ticks", str(TICKS), "--jobs", "4",
         "--stream-dir", concurrent_streams]
    )
    with open(concurrent_out, "rb") as handle:
        concurrent_bytes = handle.read()
    report.record(
        "B27/P2 batch determinism under --jobs 1 and --jobs 4",
        status_jobs == 0 and concurrent_bytes == raw_retained,
        f"--jobs 4 produced {'byte-identical' if concurrent_bytes == raw_retained else 'DIFFERING'} "
        f"retained output to --jobs 1 over the same eight cells",
    )

    # P2's profile half: the same cells from a debug build produce the same rows but for the profile.
    debug_out = OUT + "/p2-debug.jsonl"
    debug_streams = fresh(OUT + "/streams-debug")
    status_debug, _, _ = driver(
        ["--out", debug_out, "--binary", DEBUG, "--sources", SOURCES, "--densities", DENSITIES,
         "--seeds", SEEDS, "--ticks", str(TICKS), "--stream-dir", debug_streams]
    )
    debug_records = read_jsonl(debug_out)
    debug_rows = [record for record in debug_records if record["record"] == "cell"]
    debug_sweep = [record for record in debug_records if record["record"] == "sweep"][0]
    report.record(
        "P2 a row is a function of its cell, across build profiles",
        status_debug == 0 and debug_rows == rows and debug_sweep["binary_profile"] == "debug",
        f"the debug build's eight rows are {'identical' if debug_rows == rows else 'DIFFERENT'} to "
        f"the release build's; the sweep record states binary_profile "
        f"{debug_sweep['binary_profile']!r}",
    )

    # ----------------------------------------------------------------------------------------------
    # Q1: exactly one class per row, with its clause
    # ----------------------------------------------------------------------------------------------
    status_class, stdout_class, stderr_class = classifier([retained, "--group-by", "source,density"])
    parsed = json.loads(stdout_class) if status_class == 0 else {}
    class_faults = []
    for row, reported in zip(rows, parsed.get("rows", [])):
        expected, clause = oracle_classify(row)
        if reported["class"] != expected or reported["clause"] != clause:
            class_faults.append(
                f"{row['source']}:{row['density']}:{row['seed']} instrument "
                f"{reported['class']}/{reported['clause']} != oracle {expected}/{clause}"
            )
        if reported["reason"] is not None:
            class_faults.append(f"{row['seed']} carries a reason: {reported['reason']}")
    with open(OUT + "/q1-classes.json", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(json.dumps(parsed, ensure_ascii=False, indent=1) + "\n")
    report.record(
        "Q1 exactly one class, traceable to its clause",
        status_class == 0 and len(parsed.get("rows", [])) == len(rows) and not class_faults,
        f"status {status_class}, {len(parsed.get('rows', []))} classified rows, disagreements with "
        f"the independent classifier {class_faults or 'none'}"
        + (f", stderr {stderr_class.strip()}" if stderr_class.strip() else ""),
    )

    return report.write(OUT + "/result.json")


if __name__ == "__main__":
    raise SystemExit(0 if main() else 1)
