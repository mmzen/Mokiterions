#!/usr/bin/env python3
"""An independently written oracle for VER-MOK-019.

This file imports nothing from either instrument. Every count, equality and predicate below is
written from `SPEC-MOK-008`'s text as straight-line code over a stream's bytes, so that a comparison
against the instrument is a comparison between two paths over the same bytes rather than the
instrument confirming its own arithmetic. That is the contract's *Independence* point 1.

Two implementation choices are deliberately unlike the driver's, so that a shared mistake is less
likely: the digest is taken over the whole file's bytes in one call rather than accumulated per line,
and the records are recovered by splitting the bytes rather than by iterating the file object.

It lives outside the repository. `WO-MOK-033`'s execution scope names four files under `scripts/` and
this is not one of them; it is retained as evidence instead.
"""

from __future__ import annotations

import hashlib
import json
import os
import subprocess

REPO = "C:/Users/mathi/Mokiterions-understand-20260829-1340"
WORK = "C:/Users/mathi/mok-scratch/ver019"
RELEASE = REPO + "/target/release/Mokiterions.exe"
DEBUG = REPO + "/target/debug/Mokiterions.exe"
DRIVER = REPO + "/scripts/run_simulation_batch.py"
CLASSIFIER = REPO + "/scripts/classify_simulation_runs.py"

KINDS = ("attack_resolved", "threat_resolved", "surrender_resolved")


# ==================================================================================================
# Process helpers
# ==================================================================================================


def run(argv, cwd=REPO, env=None, stdin=None, timeout=1800):
    """Run a command and return `(status, stdout, stderr)` with both streams as text."""
    completed = subprocess.run(
        argv, cwd=cwd, env=env, input=stdin, capture_output=True, timeout=timeout
    )
    return (
        completed.returncode,
        completed.stdout.decode("utf-8", errors="replace"),
        completed.stderr.decode("utf-8", errors="replace"),
    )


def engine(binary, seed, ticks, policy, density, events_path, extra=()):
    """Run the engine by hand with rule 4.4's arguments, plus anything a case adds deliberately."""
    argv = [
        binary,
        "--seed", str(seed),
        "--ticks", str(ticks),
        "--policy", policy,
        "--density", density,
        "--events-path", events_path,
    ]
    argv.extend(extra)
    return run(argv)


def driver(args, cwd=REPO, env=None):
    """Run the batch driver through its command line, which is the only surface this oracle uses."""
    return run(["python", DRIVER] + list(args), cwd=cwd, env=env)


def classifier(args, cwd=REPO, env=None):
    return run(["python", CLASSIFIER] + list(args), cwd=cwd, env=env)


# ==================================================================================================
# Reading bytes
# ==================================================================================================


def sha256_whole(path):
    """A digest over the file's raw bytes, taken in one call before any decode. `VER-MOK-019` B9."""
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def sha256_bytes(raw):
    return hashlib.sha256(raw).hexdigest()


def read_jsonl(path):
    """Recover the records of a JSONL file, ignoring blank lines."""
    with open(path, "rb") as handle:
        raw = handle.read()
    records = []
    for line in raw.split(b"\n"):
        if not line.strip():
            continue
        records.append(json.loads(line.decode("utf-8")))
    return records


# ==================================================================================================
# The independent stream oracle
# ==================================================================================================


def oracle_scan(path):
    """Count a record stream from its bytes. Written from `SPEC-MOK-008` rules 9 and 11.

    Returns the digest, the header and run records, a per-kind event tally, the three attempted
    counts, the three effective counts under rule 9.2's predicates, and the lethal-attack count.
    """
    with open(path, "rb") as handle:
        raw = handle.read()

    digest = hashlib.sha256(raw).hexdigest()

    header = None
    run_record = None
    tally = {}
    attempted = {kind: 0 for kind in KINDS}
    effective = {kind: 0 for kind in KINDS}
    lethal = 0
    lines = 0
    events = 0

    for line in raw.split(b"\n"):
        if not line.strip():
            continue
        lines += 1
        record = json.loads(line.decode("utf-8"))
        kind = record["record"]
        if kind == "header":
            header = record
        elif kind == "run":
            run_record = record
        elif kind == "event":
            events += 1
            name = record["event"]
            tally[name] = tally.get(name, 0) + 1
            if name == "attack_resolved":
                attempted[name] += 1
                # Rule 9.2: an attack is effective when it did damage.
                if record["result"]["damage"] > 0:
                    effective[name] += 1
                # Rule 9.3: lethality is a third figure, not a narrowing of effectiveness.
                if record["result"]["target_died"] == "yes":
                    lethal += 1
            elif name == "threat_resolved":
                attempted[name] += 1
                # Rule 9.2: a threat is effective when it raised fear.
                if record["result"]["increase"] > 0:
                    effective[name] += 1
            elif name == "surrender_resolved":
                attempted[name] += 1
                # Rule 9.2: a surrender is effective when food changed hands.
                if record["result"]["transferred"] > 0:
                    effective[name] += 1

    return {
        "digest": digest,
        "bytes": len(raw),
        "records": lines,
        "events": events,
        "header": header,
        "run": run_record,
        "tally": tally,
        "attempted": attempted,
        "effective": effective,
        "lethal_attacks": lethal,
    }


def oracle_crosscheck(scan):
    """Rule 10.1's seven equalities, recomputed. Returns a list of `(name, left, right, ok)`."""
    run_record = scan["run"]
    tally = scan["tally"]

    def count(name):
        return tally.get(name, 0)

    consumed = run_record["consumed"]
    skipped = run_record["regeneration_skipped"]
    roster = len(run_record["agents"])

    pairs = [
        ("territory_crossed = crossings", count("territory_crossed"), run_record["crossings"]),
        ("agent_died = deaths", count("agent_died"), run_record["deaths"]),
        (
            "food_consumed = consumed sum",
            count("food_consumed"),
            consumed["low"] + consumed["medium"] + consumed["high"],
        ),
        ("food_regenerated = regenerated", count("food_regenerated"), run_record["regenerated"]),
        (
            "food_regeneration_skipped = skipped sum",
            count("food_regeneration_skipped"),
            skipped["depleted"] + skipped["capacity"],
        ),
        ("agent_initialized = agents length", count("agent_initialized"), roster),
        ("survivors + deaths = agents length", run_record["survivors"] + run_record["deaths"], roster),
    ]
    return [(name, left, right, left == right) for name, left, right in pairs]


# ==================================================================================================
# The independent classification oracle
# ==================================================================================================


def oracle_classify(row):
    """Rule 16.5's five clauses in order, first match wins. Written from the specification's text."""
    if row["survivors"] == 0:
        return "extinction", 1
    if row["regeneration_skipped"]["depleted"] > 0:
        return "famine", 2
    populations = [figures["population"] for figures in row["territories"].values()]
    if populations.count(0) == 1 and max(populations) > 0:
        return "asymmetric_collapse", 3
    if row["deaths"] * 2 >= row["roster"]:
        return "collapse", 4
    return "coexistence", 5


# ==================================================================================================
# Rule 8.1's declared shape, transcribed from the specification's example record
# ==================================================================================================

INTEGER_FIELDS = (
    "seed", "ticks", "run_ticks", "roster", "survivors", "deaths", "crossings", "regenerated",
    "lethal_attacks", "crosschecks_passed",
)
STRING_FIELDS = ("record", "source", "density", "engine", "stream_sha256", "reason")
CONSUMED_KEYS = ("low", "medium", "high")
SKIPPED_KEYS = ("depleted", "capacity")
TERRITORY_KEYS = ("population", "low", "medium", "high")


def row_shape_faults(row):
    """Every way a `cell` record departs from rule 8.1's shape, named. Empty means conforming."""
    faults = []

    for name in STRING_FIELDS:
        if name not in row:
            faults.append(f"missing {name}")
        elif not isinstance(row[name], str):
            faults.append(f"{name} is {type(row[name]).__name__}, not a string")
    for name in INTEGER_FIELDS:
        if name not in row:
            faults.append(f"missing {name}")
        elif isinstance(row[name], bool) or not isinstance(row[name], int):
            faults.append(f"{name} is {type(row[name]).__name__}, not an integer")

    if row.get("record") != "cell":
        faults.append(f"record is {row.get('record')!r}, not 'cell'")
    digest = row.get("stream_sha256", "")
    if len(digest) != 64 or any(character not in "0123456789abcdef" for character in digest):
        faults.append("stream_sha256 is not 64 lowercase hexadecimal characters")

    for name, keys in (("consumed", CONSUMED_KEYS), ("regeneration_skipped", SKIPPED_KEYS)):
        block = row.get(name)
        if not isinstance(block, dict):
            faults.append(f"{name} is not an object")
            continue
        for key in keys:
            if key not in block:
                faults.append(f"missing {name}.{key}")
            elif isinstance(block[key], bool) or not isinstance(block[key], int):
                faults.append(f"{name}.{key} is not an integer")
        for key in block:
            if key not in keys:
                faults.append(f"unexpected {name}.{key}")

    for name in ("attempted", "effective"):
        block = row.get(name)
        if not isinstance(block, dict):
            faults.append(f"{name} is not an object")
            continue
        for kind in KINDS:
            if kind not in block:
                faults.append(f"missing {name}.{kind}")
            elif isinstance(block[kind], bool) or not isinstance(block[kind], int):
                faults.append(f"{name}.{kind} is not an integer")
        for key in block:
            if key not in KINDS:
                faults.append(f"unexpected {name}.{key}")

    territories = row.get("territories")
    if not isinstance(territories, dict) or not territories:
        faults.append("territories is not a non-empty object")
    else:
        for label, figures in territories.items():
            if not isinstance(figures, dict):
                faults.append(f"territories.{label} is not an object")
                continue
            for key in TERRITORY_KEYS:
                if key not in figures:
                    faults.append(f"missing territories.{label}.{key}")
                elif isinstance(figures[key], bool) or not isinstance(figures[key], int):
                    faults.append(f"territories.{label}.{key} is not an integer")

    return faults


def floats_in(value, path="$"):
    """Every path at which a float appears. `VER-MOK-019` T9 and P7."""
    found = []
    if isinstance(value, float):
        found.append(path)
    elif isinstance(value, dict):
        for key, member in value.items():
            found.extend(floats_in(member, f"{path}.{key}"))
    elif isinstance(value, list):
        for index, member in enumerate(value):
            found.extend(floats_in(member, f"{path}[{index}]"))
    return found


# ==================================================================================================
# Result accumulation
# ==================================================================================================


class Report:
    """Accumulates one entry per case so a phase's output is a table rather than a log."""

    def __init__(self, phase):
        self.phase = phase
        self.entries = []

    def record(self, case, ok, detail, note=None):
        entry = {"case": case, "ok": bool(ok), "detail": detail}
        if note is not None:
            entry["note"] = note
        self.entries.append(entry)
        print(f"[{'PASS' if ok else 'FAIL'}] {case}: {detail}")
        return entry

    def write(self, path):
        failures = [entry["case"] for entry in self.entries if not entry["ok"]]
        payload = {
            "phase": self.phase,
            "cases": len(self.entries),
            "failed": failures,
            "entries": self.entries,
        }
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(json.dumps(payload, ensure_ascii=False, indent=1) + "\n")
        print(f"\n{self.phase}: {len(self.entries) - len(failures)} of {len(self.entries)} passed")
        if failures:
            print("failed: " + ", ".join(failures))
        return not failures
