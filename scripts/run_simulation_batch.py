#!/usr/bin/env python3
"""Sweep the Mokiterions engine over a declared configuration space and retain one bound fact row per run.

This is the batch driver `SPEC-MOK-008` rules 1 to 15 specify. It enumerates cells, executes the engine's
binary target once per cell as a child process, reads and hashes each record stream, derives one `cell`
record from it, discards the stream, and writes the retained output.

It classifies nothing and holds no threshold. `scripts/classify_simulation_runs.py` does that, and rule 19
divides the two so that a threshold can be revised without invalidating a retained row.

Standard library only, per rule 1.3.
"""

from __future__ import annotations

import argparse
import concurrent.futures
import hashlib
import io
import json
import os
import queue
import re
import shutil
import subprocess
import sys
import tempfile
from collections import Counter

# --- Rule 6.4: the retained file format's version. ------------------------------------------------
FORMAT = 1

# --- Rule 3.1: the declared default sweep. Changing any of these is an amendment to SPEC-MOK-008. -
DEFAULT_SOURCES = ("baseline", "reference", "individual", "social")
DEFAULT_DENSITIES = ("0.10", "0.25", "0.50", "0.75", "1.00")
DEFAULT_SEEDS = tuple(range(20))
DEFAULT_TICKS = 1000

# The engine's own `--policy` domain. `llm` is listed because rule 4.2 refuses it by name rather than
# by failing to recognise it: a sweep naming `llm` must be told why, not told it made a typo.
ENGINE_SOURCES = ("baseline", "reference", "individual", "social", "llm")
LIVE_ONLY_SOURCES = ("llm",)

# --- Rule 9.1: the three kinds the run record states no cumulative figure for, in rule 9.2's order.
ATTEMPTED_KINDS = ("attack_resolved", "threat_resolved", "surrender_resolved")

# --- Rule 12.3: the closed stage vocabulary. ------------------------------------------------------
STAGES = ("refused", "run", "read", "crosscheck", "derive", "not_attempted")

# A density is written "with at most two decimal places" and must not exceed 100, which is the
# engine's own statement of this option's domain. The further constraint that a density leave at
# least one resource per territory is a rule of the world rather than of the option, so it is left to
# the engine and its refusal is carried verbatim under rule 13.2. Rule 10.4's principle: this driver
# re-implements no simulation rule.
DENSITY_FORM = re.compile(r"^(?:0|[1-9][0-9]*)(?:\.[0-9]{1,2})?$")


class UsageError(Exception):
    """A rule 15.1 usage or configuration error. Nothing executes and nothing is written; status 2."""


class DriverError(Exception):
    """The driver could not complete its own work: rule 14.1's status 1."""


# ==================================================================================================
# Axis parsing
# ==================================================================================================


def parse_list(raw, axis):
    """Split a comma-separated axis. Rule 2.4."""
    values = [part.strip() for part in raw.split(",")]
    if any(value == "" for value in values):
        raise UsageError(f"malformed {axis}: {raw!r}; values are comma-separated and none may be empty")
    return values


def parse_seeds(raw):
    """Parse a seed specification: comma-separated integers and inclusive `a-b` ranges. Rule 2.4."""
    seeds = []
    for part in [piece.strip() for piece in raw.split(",")]:
        if part == "":
            raise UsageError(f"malformed seed specification: {raw!r}; no element may be empty")
        match = re.fullmatch(r"(\d+)-(\d+)", part)
        if match:
            low, high = int(match.group(1)), int(match.group(2))
            if high < low:
                raise UsageError(
                    f"malformed seed specification: {part!r}; a range is inclusive and written low-high"
                )
            seeds.extend(range(low, high + 1))
            continue
        if not re.fullmatch(r"\d+", part):
            raise UsageError(
                f"malformed seed specification: {part!r}; expected an integer or an inclusive range a-b"
            )
        seeds.append(int(part))
    return seeds


def check_axis(values, axis):
    """Rule 15.1: an empty axis and a duplicate value on any axis are usage errors.

    Rule 4.1 states why duplicates are refused rather than collapsed: a silently collapsed duplicate
    and an accepted one differ in every later distribution, so the operator should say which was meant.
    """
    if not values:
        raise UsageError(f"empty axis: {axis}; every axis needs at least one value")
    seen = set()
    for value in values:
        if value in seen:
            raise UsageError(f"duplicate value on axis {axis}: {value!r}")
        seen.add(value)


def validate_sources(sources):
    """Rule 4.2 and rule 15.1."""
    for source in sources:
        if source in LIVE_ONLY_SOURCES:
            raise UsageError(
                f"refused: decision source {source!r} obtains its decisions from a model and is "
                f"refused before any cell executes, per REQ-MOK-072; no sweep may include it"
            )
        if source not in ENGINE_SOURCES:
            raise UsageError(
                f"the engine does not accept decision source {source!r}; it accepts "
                + ", ".join(ENGINE_SOURCES)
            )


def validate_densities(densities):
    """Rule 15.1, over the option's own domain rather than over the world's."""
    for density in densities:
        if not DENSITY_FORM.fullmatch(density):
            raise UsageError(
                f"the engine does not accept density {density!r}; a density is written with at most "
                f"two decimal places"
            )
        if not 0 < float(density) <= 100:
            raise UsageError(
                f"the engine does not accept density {density!r}; it must be greater than 0 and must "
                f"not exceed 100"
            )


def validate_out(path):
    """Rule 15.1: no batch may write into governance artifacts."""
    parts = os.path.normpath(os.path.abspath(path)).replace("\\", "/").split("/")
    for index in range(len(parts) - 1):
        if parts[index] == "docs" and parts[index + 1] == "engineering":
            raise UsageError(
                f"--out may not name a path under docs/engineering/: {path!r}; retained output is not "
                f"a governance artifact and a batch does not write one"
            )


def parse_keep_stream(values):
    """Rule 2.7: `<source>:<density>:<seed>`, repeatable."""
    kept = []
    for value in values:
        pieces = value.split(":")
        if len(pieces) != 3:
            raise UsageError(
                f"malformed --keep-stream {value!r}; expected <source>:<density>:<seed>"
            )
        source, density, seed = pieces
        if not re.fullmatch(r"\d+", seed):
            raise UsageError(f"malformed --keep-stream {value!r}; the seed must be an integer")
        kept.append((source, density, int(seed)))
    return kept


# ==================================================================================================
# The sweep
# ==================================================================================================


def resolve_sweep(args):
    """Apply rule 2.6: absent all four axes the default sweep runs; some replaces only those.

    Returns the four axes and rule 6.5's `defaulted_axes`.
    """
    defaulted = []

    if args.sources is None:
        sources = list(DEFAULT_SOURCES)
        defaulted.append("sources")
    else:
        sources = parse_list(args.sources, "sources")

    if args.densities is None:
        densities = list(DEFAULT_DENSITIES)
        defaulted.append("densities")
    else:
        densities = parse_list(args.densities, "densities")

    if args.seeds is None:
        seeds = list(DEFAULT_SEEDS)
        defaulted.append("seeds")
    else:
        seeds = parse_seeds(args.seeds)

    if args.ticks is None:
        ticks = DEFAULT_TICKS
        defaulted.append("ticks")
    else:
        ticks = args.ticks

    check_axis(sources, "sources")
    check_axis(densities, "densities")
    check_axis(seeds, "seeds")
    validate_sources(sources)
    validate_densities(densities)
    if ticks <= 0:
        raise UsageError(f"non-positive tick horizon: {ticks}; a horizon is greater than zero")

    return sources, densities, seeds, ticks, defaulted


def build_cells(sources, densities, seeds):
    """Rule 4.3: source in the order given, then density in the order given, then seed ascending."""
    cells = []
    for source in sources:
        for density in densities:
            for seed in sorted(seeds):
                cells.append((source, density, seed))
    return cells


def resolve_binary(explicit):
    """Rule 2.3. Returns the path and rule 6.3's `binary_profile`.

    The path is normalised to the platform's separator. On Windows a relative path written with
    forward slashes passes `os.path.isfile` and then fails to launch, because the child is started
    from a command line the system parses rather than from a path it opens. Normalising here means an
    operator may write the path either way. No retained byte carries it: rule 11.6 keeps paths out of
    rows and rule 6.3 retains only the profile.
    """
    suffix = ".exe" if os.name == "nt" else ""
    if explicit is not None:
        if not os.path.isfile(explicit):
            raise DriverError(f"engine binary not found: {explicit!r}")
        explicit = os.path.normpath(explicit)
        parent = os.path.basename(os.path.dirname(os.path.abspath(explicit)))
        profile = parent if parent in ("release", "debug") else "other"
        return explicit, profile
    for profile in ("release", "debug"):
        candidate = os.path.join("target", profile, "Mokiterions" + suffix)
        if os.path.isfile(candidate):
            return candidate, profile
    raise DriverError(
        "engine binary not found: looked for "
        + " then ".join(
            os.path.join("target", profile, "Mokiterions" + suffix) for profile in ("release", "debug")
        )
        + "; build one or name it with --binary"
    )


# ==================================================================================================
# Reading a stream
# ==================================================================================================


def scan_file(path):
    """Scan a stream from disk one line at a time. Rule 7.1's consequence for a large stream.

    A thousand-tick stream is about 3 MB and a longer horizon scales with it, so the stream is never
    held in memory whole. Iterating a binary file yields each line with its terminator, so hashing
    every line in order hashes the file's exact bytes and rule 11.2 still holds.
    """
    with open(path, "rb") as handle:
        return _scan(handle)


def scan_stream(raw):
    """Scan a stream already in memory. The fixture path, and the same code as `scan_file`."""
    return _scan(io.BytesIO(raw))


def _scan(lines):
    """Read a stream once: hash its bytes, count events, and find the header and run records.

    Rule 11.2: the digest is over the stream's exact bytes as the engine wrote them, accumulated
    before any decoding, normalisation or line-ending change.
    """
    digest = hashlib.sha256()

    header = None
    run = None
    counted = Counter()
    attempted = {kind: 0 for kind in ATTEMPTED_KINDS}
    effective = {kind: 0 for kind in ATTEMPTED_KINDS}
    lethal_attacks = 0

    for index, line in enumerate(lines):
        # The digest covers every byte including the terminator, whether or not the line carries a
        # record, so it is accumulated before the line is examined at all.
        digest.update(line)
        if not line.strip():
            continue
        try:
            record = json.loads(line.decode("utf-8"))
        except (ValueError, UnicodeDecodeError) as error:
            raise ValueError(f"line {index + 1} is not a JSON record: {error}") from error
        kind = record.get("record")
        if kind == "header":
            header = record
        elif kind == "run":
            run = record
        elif kind == "event":
            event = record.get("event")
            counted[event] += 1
            if event in attempted:
                attempted[event] += 1
                result = record.get("result") or {}
                # Rule 9.2: each test reads the record's own field and nothing else.
                if event == "attack_resolved":
                    if int(result.get("damage", 0)) > 0:
                        effective[event] += 1
                    # Rule 9.3: a third figure, not a narrowing of `effective`.
                    if result.get("target_died") == "yes":
                        lethal_attacks += 1
                elif event == "threat_resolved":
                    if int(result.get("increase", 0)) > 0:
                        effective[event] += 1
                elif event == "surrender_resolved":
                    if int(result.get("transferred", 0)) > 0:
                        effective[event] += 1

    if header is None:
        raise ValueError("the stream carries no header record")
    if run is None:
        raise ValueError("the stream carries no run record")

    return {
        "digest": digest.hexdigest(),
        "header": header,
        "run": run,
        "counted": counted,
        "attempted": attempted,
        "effective": effective,
        "lethal_attacks": lethal_attacks,
    }


def crosscheck(scan):
    """Rule 10.1's seven equalities. Returns (passed, failures).

    Rule 10.4: every equality here is one `SPEC-MOK-006` rule 8.6 already asserts as a property of
    the stream. None is this driver's invention.
    """
    run = scan["run"]
    counted = scan["counted"]
    consumed = run["consumed"]
    skipped = run["regeneration_skipped"]
    roster = len(run["agents"])

    checks = [
        ("territory_crossed", counted["territory_crossed"], run["crossings"], "crossings"),
        ("agent_died", counted["agent_died"], run["deaths"], "deaths"),
        (
            "food_consumed",
            counted["food_consumed"],
            consumed["low"] + consumed["medium"] + consumed["high"],
            "consumed sum",
        ),
        ("food_regenerated", counted["food_regenerated"], run["regenerated"], "regenerated"),
        (
            "food_regeneration_skipped",
            counted["food_regeneration_skipped"],
            skipped["depleted"] + skipped["capacity"],
            "regeneration_skipped sum",
        ),
        ("agent_initialized", counted["agent_initialized"], roster, "agents length"),
        ("survivors + deaths", run["survivors"] + run["deaths"], roster, "agents length"),
    ]

    passed = 0
    failures = []
    for name, observed, expected, label in checks:
        if observed == expected:
            passed += 1
        else:
            # Rule 10.3: neither figure is preferred. Both are named.
            failures.append(f"{name} {observed} != {label} {expected}")
    return passed, failures


def derive_row(cell, scan, passed):
    """Build the rule 8.1 `cell` record. Key order is fixed here because rule 18.1 hashes this file."""
    source, density, seed = cell
    run = scan["run"]
    header = scan["header"]
    territories = run["final"]["territories"]
    return {
        "record": "cell",
        "source": source,
        "density": density,
        "seed": seed,
        "ticks": header["config"]["ticks"],
        # Rule 8.3: the version the stream states, not one the driver inferred.
        "engine": header["engine"],
        "stream_sha256": scan["digest"],
        # Rule 8.4: the run record's own figures, copied without reinterpretation.
        "reason": run["reason"],
        "run_ticks": run["ticks"],
        # Rule 8.5: the length of the agents array. The array itself is a deliberate loss.
        "roster": len(run["agents"]),
        "survivors": run["survivors"],
        "deaths": run["deaths"],
        "crossings": run["crossings"],
        "consumed": {
            "low": run["consumed"]["low"],
            "medium": run["consumed"]["medium"],
            "high": run["consumed"]["high"],
        },
        "regenerated": run["regenerated"],
        "regeneration_skipped": {
            "depleted": run["regeneration_skipped"]["depleted"],
            "capacity": run["regeneration_skipped"]["capacity"],
        },
        "territories": {
            name: {
                "population": figures["population"],
                "low": figures["low"],
                "medium": figures["medium"],
                "high": figures["high"],
            }
            for name, figures in territories.items()
        },
        # Rule 9.4: both counts for all three kinds, and a zero is stated rather than omitted.
        "attempted": {kind: scan["attempted"][kind] for kind in ATTEMPTED_KINDS},
        "effective": {kind: scan["effective"][kind] for kind in ATTEMPTED_KINDS},
        "lethal_attacks": scan["lethal_attacks"],
        "crosschecks_passed": passed,
    }


# ==================================================================================================
# Executing one cell
# ==================================================================================================


def execute_cell(cell, ticks, binary, stream_path, keep_to=None, runner=None):
    """Run one cell and derive its row.

    Returns `(row, None)` or `(None, missing)` where `missing` is a rule 12.3 entry without its
    coordinates, which the caller adds.
    """
    source, density, seed = cell

    # Rule 4.2 is enforced at the sweep level before any cell executes. This guard is the same
    # refusal at the cell level, so that no path through this function can reach a provider even if a
    # caller assembled cells without validating them. It is also rule 12.3's `refused` stage.
    if source in LIVE_ONLY_SOURCES:
        return None, {
            "stage": "refused",
            "reason": (
                f"decision source {source!r} obtains its decisions from a model and is refused "
                f"under REQ-MOK-072"
            ),
        }

    # Rule 4.4: exactly these arguments and no others. No tracing option, no transcript option, no
    # live option, and no connector option.
    argv = [
        binary,
        "--seed",
        str(seed),
        "--ticks",
        str(ticks),
        "--policy",
        source,
        "--density",
        density,
        "--events-path",
        stream_path,
    ]

    execute = runner if runner is not None else _spawn
    try:
        status, stderr = execute(argv)
    except OSError as error:
        return None, {"stage": "run", "reason": f"could not execute the engine: {error}"}

    if status != 0:
        # Rule 13.2: the engine's own message verbatim. The driver does not translate, summarise or
        # improve it, which means a configuration error carries the engine's usage text too.
        return None, {"stage": "run", "reason": f"engine exited {status}: {stderr}"}

    # Rule 7.4: the copy happens before the reusable path is overwritten, and changes no row.
    if keep_to is not None and os.path.isfile(stream_path):
        try:
            shutil.copyfile(stream_path, keep_to)
        except OSError as error:
            print(f"could not keep the stream for {source}:{density}:{seed}: {error}")

    try:
        scan = scan_file(stream_path)
    except (OSError, ValueError) as error:
        return None, {"stage": "read", "reason": f"could not read the record stream: {error}"}

    try:
        passed, failures = crosscheck(scan)
    except (KeyError, TypeError) as error:
        return None, {"stage": "derive", "reason": f"could not derive the row: {error}"}

    if failures:
        # Rule 10.3: a failed check is a cell failure. The row is not retained with the disagreement
        # recorded, because retaining it would be deciding which of the engine's two accounts to believe.
        return None, {"stage": "crosscheck", "reason": "; ".join(failures)}

    try:
        row = derive_row(cell, scan, passed)
    except (KeyError, TypeError) as error:
        return None, {"stage": "derive", "reason": f"could not derive the row: {error}"}

    return row, None


def _spawn(argv):
    """Rule 5.1: one child process per cell; its output and status are captured.

    Rule 5.2: standard output is not retained. It is read and dropped so the child cannot block on a
    full pipe. Rule 4.5: no environment variable is added, removed or read here.
    """
    completed = subprocess.run(argv, capture_output=True)
    return completed.returncode, completed.stderr.decode("utf-8", errors="replace")


# ==================================================================================================
# The batch
# ==================================================================================================


def stream_path_for(stream_dir, slot):
    """Rule 7.1: one reusable path the driver controls.

    Rule 2.8 permits concurrency, and two concurrent cells cannot share one path, so there is one
    reusable path per worker slot. At the default `--jobs 1` that is exactly one path and rule 7.1's
    stated consequence holds: peak disk is one stream.
    """
    return os.path.join(stream_dir, f"batch-stream-{slot}.jsonl")


def run_batch(cells, ticks, binary, stream_dir, jobs, keep, report=print, runner=None):
    """Execute every cell and return `(rows, missing, leftover_streams)`, rows in rule 4.3's order.

    `runner` replaces the child-process call. It exists because rule 20.2 requires the tests to
    execute no real engine run, which is only possible if the process boundary is a seam.
    """
    keep_set = set(keep)
    results = {}
    missing = {}
    leftover = []

    def one(cell, slot):
        path = stream_path_for(stream_dir, slot)
        keep_to = None
        if cell in keep_set:
            source, density, seed = cell
            keep_to = os.path.join(stream_dir, f"kept-{source}-{density}-{seed}.jsonl")
        try:
            row, fault = execute_cell(cell, ticks, binary, path, keep_to=keep_to, runner=runner)
            if keep_to is not None and os.path.isfile(keep_to):
                report(f"kept stream for {cell[0]}:{cell[1]}:{cell[2]} at {keep_to}")
        finally:
            # Rule 7.3: the stream is removed after being read. A stream that cannot be removed fails
            # neither the cell nor the batch; the leftover path is reported.
            #
            # In a `finally`, because an interrupt is the case where it matters most. Rule 7.3 is what
            # makes rule 7.2's peak-disk consequence true, and an operator's Ctrl+C arrives inside
            # `execute_cell` far more often than between cells -- that is where the seconds are.
            # Measured before this was a `finally`: an 80-cell sweep interrupted 3 s in left
            # `batch-stream-0.jsonl` on disk while the `batch` record's `leftover_streams` was empty,
            # so the retained output positively stated that nothing was left behind when something
            # was. That is worse than the file itself: rule 12.1's record would have been wrong.
            try:
                if os.path.isfile(path):
                    os.remove(path)
            except OSError:
                leftover.append(path)
        return cell, row, fault

    interrupted = False
    try:
        if jobs <= 1:
            for cell in cells:
                _, row, fault = one(cell, 0)
                if row is not None:
                    results[cell] = row
                else:
                    missing[cell] = fault
        else:
            # Rule 7.1: one reusable path per worker, held for the whole of a cell and then returned.
            #
            # A slot must not be derived from the cell's position in the sweep. With eight cells over
            # four workers, position 4 begins as soon as *any* of positions 0 to 3 finishes, which is
            # not necessarily position 0, so `index % jobs` does not exclude two concurrent cells from
            # one path. Measured before this pool existed: of eight cells at `--jobs 4`, two were
            # lost, one reading a half-written line and one finding the file already removed by the
            # other. Taking the slot from a pool makes exclusion a property of the pool's size rather
            # than of the scheduler's order.
            slots = queue.SimpleQueue()
            for slot in range(jobs):
                slots.put(slot)

            def one_from_pool(cell):
                slot = slots.get()
                try:
                    return one(cell, slot)
                finally:
                    slots.put(slot)

            with concurrent.futures.ThreadPoolExecutor(max_workers=jobs) as pool:
                futures = {pool.submit(one_from_pool, cell): cell for cell in cells}
                for future in concurrent.futures.as_completed(futures):
                    cell, row, fault = future.result()
                    if row is not None:
                        results[cell] = row
                    else:
                        missing[cell] = fault
    except KeyboardInterrupt:
        interrupted = True

    if interrupted:
        for cell in cells:
            if cell not in results and cell not in missing:
                # Rule 12.3's `not_attempted` stage; rule 12.4 forbids filling it with anything else.
                missing[cell] = {"stage": "not_attempted", "reason": "interrupted by operator"}

    # Rule 4.3: the retained output is written in this order whatever order cells completed in, which
    # is what makes rule 18.1's byte-identity hold under `--jobs` greater than 1.
    rows = [results[cell] for cell in cells if cell in results]
    missing_rows = [
        {
            "source": cell[0],
            "density": cell[1],
            "seed": cell[2],
            "stage": missing[cell]["stage"],
            "reason": missing[cell]["reason"],
        }
        for cell in cells
        if cell in missing
    ]
    return rows, missing_rows, leftover


def sweep_record(sources, densities, seeds, ticks, cells, profile, defaulted):
    """Rule 6.3."""
    return {
        "record": "sweep",
        "format": FORMAT,
        "sources": list(sources),
        "densities": list(densities),
        # Rule 6.6: every axis value is stated, including values that produce no rows.
        "seeds": sorted(seeds),
        "ticks": ticks,
        "cells": len(cells),
        "binary_profile": profile,
        "digest_algorithm": "sha256",
        "defaulted_axes": list(defaulted),
    }


def batch_record(requested, rows, missing_rows, leftover):
    """Rule 12.1."""
    return {
        "record": "batch",
        # Rule 12.2: completeness is stated explicitly, never inferred from a missing warning.
        "complete": len(rows) == requested,
        "requested": requested,
        "retained": len(rows),
        "missing": missing_rows,
        "leftover_streams": list(leftover),
    }


def write_output(path, records):
    """Rule 6.1: one JSON object per line, UTF-8, `\\n` on every platform."""
    with open(path, "w", encoding="utf-8", newline="\n") as handle:
        for record in records:
            handle.write(json.dumps(record, ensure_ascii=False, separators=(",", ":")))
            handle.write("\n")


# ==================================================================================================
# Entry point
# ==================================================================================================


def build_parser():
    parser = argparse.ArgumentParser(
        prog="run_simulation_batch.py",
        description=(
            "Sweep the Mokiterions engine over a declared configuration space and retain one bound "
            "fact row per run. Absent all four axis options the declared default sweep of "
            "SPEC-MOK-008 rule 3 runs: 4 sources x 5 densities x 20 seeds at 1000 ticks, 400 cells."
        ),
        allow_abbrev=False,
    )
    parser.add_argument("--out", required=True, help="file the retained output is written to")
    parser.add_argument("--binary", help="engine binary; absent, target/release then target/debug")
    parser.add_argument("--sources", help="comma-separated decision sources")
    parser.add_argument("--densities", help="comma-separated food densities")
    parser.add_argument("--seeds", help="seeds: comma-separated integers and inclusive a-b ranges")
    parser.add_argument("--ticks", type=int, help="tick horizon for every cell of the sweep")
    parser.add_argument(
        "--keep-stream",
        action="append",
        default=[],
        metavar="SOURCE:DENSITY:SEED",
        help="retain that one cell's stream at a reported path; may be given more than once",
    )
    parser.add_argument("--jobs", type=int, default=1, help="execute cells concurrently; default 1")
    parser.add_argument(
        "--stream-dir",
        help="directory for the reusable temporary stream path; absent, the platform's temporary directory",
    )
    return parser


def main(argv=None, runner=None):
    """Rule 14.1's exit statuses are this function's return values.

    `runner` replaces the child-process call and is threaded to `run_batch` for the same reason rule
    20.2 gives: the tests execute no real engine run.
    """
    args = build_parser().parse_args(argv)

    # Rule 14.1: status 2 means nothing executed and nothing was written, so every rule 15.1 check
    # runs before the output is created or the binary is resolved.
    try:
        validate_out(args.out)
        sources, densities, seeds, ticks, defaulted = resolve_sweep(args)
        keep = parse_keep_stream(args.keep_stream)
        if args.jobs < 1:
            raise UsageError(f"--jobs must be at least 1: {args.jobs}")
    except UsageError as error:
        print(f"configuration error: {error}", file=sys.stderr)
        return 2

    cells = build_cells(sources, densities, seeds)

    try:
        binary, profile = resolve_binary(args.binary)
    except DriverError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    print(f"engine binary: {binary} ({profile})")

    # Rule 2.2: the driver creates the output and truncates it, and creates no directory it was not
    # given. Rule 12.7: where the output cannot be written the driver retains nothing.
    try:
        with open(args.out, "w", encoding="utf-8", newline="\n"):
            pass
    except OSError as error:
        print(f"error: retained output is not writable: {error}", file=sys.stderr)
        return 1

    owned_stream_dir = None
    if args.stream_dir is not None:
        stream_dir = args.stream_dir
        if not os.path.isdir(stream_dir):
            print(f"error: --stream-dir is not a directory: {stream_dir!r}", file=sys.stderr)
            return 1
    else:
        # Rule 7.2: absent an operator-supplied location the platform's temporary directory is used.
        # It is not under docs/, not under scripts/, and not the operator's output path.
        owned_stream_dir = tempfile.mkdtemp(prefix="mokiterions-batch-")
        stream_dir = owned_stream_dir
    print(f"temporary stream directory: {stream_dir}")

    try:
        rows, missing_rows, leftover = run_batch(
            cells, ticks, binary, stream_dir, args.jobs, keep, runner=runner
        )
    finally:
        # Rule 15.2: the driver removes no file it did not create.
        if owned_stream_dir is not None:
            shutil.rmtree(owned_stream_dir, ignore_errors=True)

    records = [sweep_record(sources, densities, seeds, ticks, cells, profile, defaulted)]
    records.extend(rows)
    records.append(batch_record(len(cells), rows, missing_rows, leftover))

    try:
        write_output(args.out, records)
    except OSError as error:
        print(f"error: could not write the retained output: {error}", file=sys.stderr)
        return 1

    for path in leftover:
        print(f"leftover stream could not be removed: {path}")
    print(f"retained {len(rows)} of {len(cells)} cells in {args.out}")

    # Rule 14.2: 3 is distinct from 0 so an automated caller cannot consume an incomplete sweep by
    # ignoring output.
    return 0 if len(rows) == len(cells) else 3


if __name__ == "__main__":
    sys.exit(main())
