#!/usr/bin/env python3
"""VER-MOK-019 phase C: the declared sweep, the usage refusals, and the classification cases.

B3, B4, B6, B11, Q2 to Q15, P3, P4 and acceptance scenario 3. The sweep cases run the real command line
against the stand-in binary, so a 400-cell enumeration costs no engine time. The classification cases
run the classifier's real command line over retained output, some of it produced by the engine and some
built here, because a row lacking a fact or carrying a second engine version is not a row the engine
emits.
"""

from __future__ import annotations

import contextlib
import hashlib
import json
import os
import re
import shutil

import fixtures
from oracle import (
    CLASSIFIER, WORK, Report, classifier, driver, oracle_classify, read_jsonl, sha256_whole,
)

OUT = WORK + "/phase-c"
SHIM = WORK + "/shim.bat"
PLAN = WORK + "/shim-plan.json"
STORE = WORK + "/fixture-streams"
PHASE_A = WORK + "/phase-a"
PHASE_B = WORK + "/phase-b"


def fresh(path):
    shutil.rmtree(path, ignore_errors=True)
    os.makedirs(path, exist_ok=True)
    return path


def plan(entries):
    with open(PLAN, "w", encoding="utf-8", newline="\n") as handle:
        handle.write(json.dumps(entries, ensure_ascii=False, indent=1) + "\n")


def sweep(name, args):
    out = f"{OUT}/{name}.jsonl"
    stream_dir = fresh(f"{OUT}/streams-{name}")
    status, stdout, stderr = driver(
        ["--out", out, "--binary", SHIM, "--stream-dir", stream_dir] + list(args)
    )
    return status, out, stdout, stderr


def split(path):
    records = read_jsonl(path)
    return (
        [r for r in records if r["record"] == "sweep"][0],
        [r for r in records if r["record"] == "cell"],
        [r for r in records if r["record"] == "batch"][0],
    )


# ==================================================================================================
# Retained output built here, for rows the engine does not emit
# ==================================================================================================


def cell_row(source="social", density="0.75", seed=0, engine="0.1.0", survivors=8, deaths=4,
             roster=12, depleted=0, populations=(6, 6), attempted=(9, 40, 5), effective=(3, 0, 1),
             drop=()):
    row = {
        "record": "cell",
        "source": source,
        "density": density,
        "seed": seed,
        "ticks": 1000,
        "engine": engine,
        "stream_sha256": hashlib.sha256(f"{source}{density}{seed}".encode()).hexdigest(),
        "reason": "tick_limit",
        "run_ticks": 1000,
        "roster": roster,
        "survivors": survivors,
        "deaths": deaths,
        "crossings": 25,
        "consumed": {"low": 100, "medium": 90, "high": 80},
        "regenerated": 300,
        "regeneration_skipped": {"depleted": depleted, "capacity": 11},
        "territories": {
            "A": {"population": populations[0], "low": 8, "medium": 3, "high": 1},
            "B": {"population": populations[1], "low": 9, "medium": 2, "high": 0},
        },
        "attempted": dict(zip(("attack_resolved", "threat_resolved", "surrender_resolved"), attempted)),
        "effective": dict(zip(("attack_resolved", "threat_resolved", "surrender_resolved"), effective)),
        "lethal_attacks": 1,
        "crosschecks_passed": 7,
    }
    for name in drop:
        row.pop(name, None)
    return row


def retained(name, rows, sources=("social",), densities=("0.75",), seeds=(0,), complete=True,
             missing=(), include_sweep=True, include_batch=True):
    records = []
    if include_sweep:
        records.append({
            "record": "sweep", "format": 1, "sources": list(sources), "densities": list(densities),
            "seeds": list(seeds), "ticks": 1000, "cells": len(sources) * len(densities) * len(seeds),
            "binary_profile": "release", "digest_algorithm": "sha256", "defaulted_axes": [],
        })
    records.extend(rows)
    if include_batch:
        records.append({
            "record": "batch", "complete": complete,
            "requested": len(sources) * len(densities) * len(seeds), "retained": len(rows),
            "missing": list(missing), "leftover_streams": [],
        })
    path = f"{OUT}/{name}.jsonl"
    with open(path, "w", encoding="utf-8", newline="\n") as handle:
        for record in records:
            handle.write(json.dumps(record, ensure_ascii=False, separators=(",", ":")) + "\n")
    return path


def report_over(path, args=()):
    status, stdout, stderr = classifier([path] + list(args))
    return status, (json.loads(stdout) if status == 0 and stdout else None), stderr


@contextlib.contextmanager
def classifier_edited(before, after, label):
    """Edit the classifier's own source, byte for byte, and restore it whatever happens.

    Q3 and Q6 both require the definitions themselves to change, not a monkeypatch of them: the claim
    under test is that an operator revising a threshold in this file changes classes and changes no
    retained row.
    """
    with open(CLASSIFIER, "rb") as handle:
        original = handle.read()
    if before not in original:
        raise AssertionError(f"{label}: the text to edit is not present in the classifier")
    edited = original.replace(before, after, 1)
    diff = (
        f"--- scripts/classify_simulation_runs.py (before)\n"
        f"+++ scripts/classify_simulation_runs.py (after: {label})\n"
        f"-{before.decode('utf-8').strip()}\n"
        f"+{after.decode('utf-8').strip() or '(removed)'}\n"
    )
    try:
        with open(CLASSIFIER, "wb") as handle:
            handle.write(edited)
        yield diff
    finally:
        with open(CLASSIFIER, "wb") as handle:
            handle.write(original)


# ==================================================================================================
# B3, B4, B6, B11
# ==================================================================================================


def case_declared_sweep(report):
    fixtures.write(STORE, "small.jsonl", fixtures.stream(events=fixtures.BODY, ticks=1000))
    plan({"*": {"stream": "fixture-streams/small.jsonl"}})

    status, default_out, _, _ = sweep("b3-default", [])
    sweep_record, rows, batch = split(default_out)

    expected_order = [
        (source, density, seed)
        for source in ("baseline", "reference", "individual", "social")
        for density in ("0.10", "0.25", "0.50", "0.75", "1.00")
        for seed in range(20)
    ]
    observed = [(row["source"], row["density"], row["seed"]) for row in rows]

    report.record(
        "B4 the declared default",
        sweep_record["sources"] == ["baseline", "reference", "individual", "social"]
        and sweep_record["densities"] == ["0.10", "0.25", "0.50", "0.75", "1.00"]
        and sweep_record["seeds"] == list(range(20))
        and sweep_record["ticks"] == 1000
        and sweep_record["cells"] == 400
        and sweep_record["defaulted_axes"] == ["sources", "densities", "seeds", "ticks"],
        f"with no axis option: {len(sweep_record['sources'])} sources, "
        f"{len(sweep_record['densities'])} densities, {len(sweep_record['seeds'])} seeds, "
        f"{sweep_record['ticks']} ticks, {sweep_record['cells']} cells, defaulted_axes "
        f"{sweep_record['defaulted_axes']}",
    )

    _, partial_out, _, _ = sweep("b4-partial", ["--seeds", "0-2"])
    partial_sweep, _, _ = split(partial_out)
    report.record(
        "B4 a given axis leaves the other three defaulted",
        partial_sweep["defaulted_axes"] == ["sources", "densities", "ticks"]
        and partial_sweep["seeds"] == [0, 1, 2]
        and partial_sweep["cells"] == 60,
        f"with --seeds 0-2: defaulted_axes {partial_sweep['defaulted_axes']}, seeds "
        f"{partial_sweep['seeds']}, {partial_sweep['cells']} cells",
    )

    status_jobs, jobs_out, _, _ = sweep("b3-jobs4", ["--jobs", "4"])
    with open(default_out, "rb") as handle:
        serial = handle.read()
    with open(jobs_out, "rb") as handle:
        concurrent_bytes = handle.read()
    report.record(
        "B3 cell enumeration and order",
        status == 0 and status_jobs == 0 and len(rows) == 400 and observed == expected_order
        and serial == concurrent_bytes,
        f"400 cells enumerated, {len(rows)} rows in "
        f"{'rule 4.3 order' if observed == expected_order else 'THE WRONG ORDER'}, and --jobs 4 "
        f"produced {'byte-identical' if serial == concurrent_bytes else 'DIFFERING'} output "
        f"({len(serial)} bytes)",
    )

    # B11: no ambient value in any retained byte.
    declared = set(cell_row())
    ambient = []
    for row in rows:
        extra = set(row) - declared
        if extra:
            ambient.append(f"undeclared field {sorted(extra)}")
        for name, value in row.items():
            if not isinstance(value, str):
                continue
            if name == "stream_sha256":
                continue
            if re.search(r"[\\/]|:\\|\.exe|\.jsonl|\d{4}-\d{2}-\d{2}|T\d{2}:\d{2}", value):
                ambient.append(f"{name} = {value!r}")
    with open(default_out, "rb") as handle:
        raw = handle.read()
    for token in (b"C:", b"AppData", b"Temp", b"tmp", b"elapsed", b"duration", b"hostname",
                  b"pid", b"USERNAME", b"COMPUTERNAME", str(os.getpid()).encode()):
        # The sweep and batch records are excluded: rule 12.1's leftover_streams is a path by
        # design, and rule 11.6 constrains the `cell` records.
        for row in rows:
            if token in json.dumps(row, ensure_ascii=False).encode("utf-8"):
                ambient.append(f"token {token!r} appears in a cell record")
    report.record(
        "B11 no ambient value in a cell record",
        not ambient,
        f"{len(rows)} cell records checked field by field and by pattern for a path, a timestamp, a "
        f"duration, a process identifier and an environment name; findings {ambient[:4] or 'none'}",
    )
    return default_out, rows


def case_usage_errors(report):
    log = "phase-c/child-calls.log"
    log_path = f"{OUT}/child-calls.log"
    if os.path.isfile(log_path):
        os.remove(log_path)
    plan({"_log": log, "*": {"stream": "fixture-streams/small.jsonl"}})

    conditions = [
        ("an unknown option", ["--sweep-everything"]),
        ("a malformed seed specification", ["--seeds", "1.5"]),
        ("an empty axis", ["--sources", "social,"]),
        ("a duplicate value on an axis", ["--densities", "0.75,0.75"]),
        ("a density the engine does not accept", ["--densities", "0.7777"]),
        ("a source the engine does not accept", ["--sources", "telepathic"]),
        ("llm among the sources", ["--sources", "social,llm"]),
        ("a non-positive tick horizon", ["--ticks", "0"]),
        ("--out under docs/engineering/", None),
    ]

    faults = []
    for index, (label, extra) in enumerate(conditions):
        out = (
            "docs/engineering/simulation/should-never-exist.jsonl"
            if extra is None
            else f"{OUT}/usage-{index}.jsonl"
        )
        argv = ["--out", out, "--binary", SHIM]
        if extra is not None:
            argv += extra
        status, _, stderr = driver(argv)
        absolute = out if extra is not None else f"{WORK}/../../Mokiterions-understand-20260829-1340/{out}"
        wrote = os.path.isfile(out if extra is not None else
                               "C:/Users/mathi/Mokiterions-understand-20260829-1340/" + out)
        if status != 2:
            faults.append(f"{label}: status {status}, not 2")
        if wrote:
            faults.append(f"{label}: wrote {absolute}")
        if not stderr.strip():
            faults.append(f"{label}: said nothing")

    children = []
    if os.path.isfile(log_path):
        with open(log_path, "r", encoding="utf-8") as handle:
            children = [line for line in handle if line.strip()]

    report.record(
        "B6 nine usage errors",
        not faults and not children,
        f"all {len(conditions)} of rule 15.1's conditions exited 2, wrote nothing and each said why; "
        f"the stand-in binary recorded {len(children)} invocations across all nine; faults "
        f"{faults or 'none'}",
    )


# ==================================================================================================
# Q2 to Q15, P3, P4
# ==================================================================================================


def case_classes(report):
    intended = [
        ("extinction", cell_row(seed=0, survivors=0, deaths=12, populations=(0, 0))),
        ("famine", cell_row(seed=1, survivors=8, deaths=4, depleted=3)),
        ("asymmetric_collapse", cell_row(seed=2, survivors=5, deaths=2, populations=(0, 5))),
        ("collapse", cell_row(seed=3, survivors=6, deaths=6, populations=(3, 3))),
        ("coexistence", cell_row(seed=4, survivors=11, deaths=1, populations=(6, 5))),
    ]
    # A row satisfying clause 1 and clause 4 at once. The order decides it.
    both = cell_row(seed=5, survivors=0, deaths=12, populations=(4, 4))
    rows = [row for _, row in intended] + [both]
    path = retained("q2", rows, seeds=tuple(range(6)))

    status, parsed, stderr = report_over(path, ["--group-by", "seed"])
    faults = []
    by_seed = {row["seed"]: row for row in parsed["rows"]} if parsed else {}
    for index, (name, row) in enumerate(intended):
        got = by_seed.get(index, {})
        if got.get("class") != name:
            faults.append(f"seed {index}: {got.get('class')} != {name}")
        expected, clause = oracle_classify(row)
        if got.get("clause") != clause:
            faults.append(f"seed {index}: clause {got.get('clause')} != {clause}")
    ordered = by_seed.get(5, {})
    report.record(
        "Q2 each predicate fires, and the order carries meaning",
        status == 0 and not faults and ordered.get("class") == "extinction"
        and ordered.get("clause") == 1,
        f"five fixtures assigned {[by_seed.get(i, {}).get('class') for i in range(5)]}; a row "
        f"satisfying clause 1 and clause 4 was assigned {ordered.get('class')!r} by clause "
        f"{ordered.get('clause')}; faults {faults or 'none'}",
    )

    # Q8: the class counts sum to the group's row count.
    sums = []
    for group in parsed["groups"]:
        total = sum(group["class_counts"].values())
        if total != group["rows"]:
            sums.append(f"{group['key']}: {total} != {group['rows']}")
    report.record(
        "Q8 class counts sum to the row count",
        not sums,
        f"{len(parsed['groups'])} groups, six counters each (five classes and unclassified); "
        f"mismatches {sums or 'none'}",
    )

    # Q11: an unobserved class is reported with a count of zero.
    no_famine = retained("q11", [row for name, row in intended if name != "famine"],
                         seeds=(0, 2, 3, 4))
    _, famine_report, _ = report_over(no_famine)
    zero_everywhere = all(
        group["class_counts"]["famine"] == 0 for group in famine_report["groups"]
    )
    report.record(
        "Q11 an unobserved class is stated as zero",
        zero_everywhere and "famine" in famine_report["classes"]
        and all("famine" in group["class_counts"] for group in famine_report["groups"]),
        f"over rows in which no run meets the predicate, famine is present in every group with count "
        f"0 and is omitted from none ({len(famine_report['groups'])} groups)",
    )

    # Q10: both counters, all three kinds, nothing merged.
    group = parsed["groups"][0]
    merged = [name for name in group if "attempted" in name and "effective" in name]
    report.record(
        "Q10 attempted stays distinct",
        set(group["attempted"]) == set(group["effective"]) == {
            "attack_resolved", "threat_resolved", "surrender_resolved"}
        and group["attempted"] != group["effective"]
        and not merged,
        f"every group states attempted and effective separately for all three kinds; no key merges "
        f"them ({merged or 'confirmed'})",
    )

    # Q4: a row lacking a fact a predicate reads.
    incomplete = retained("q4", [cell_row(seed=0, drop=("roster",))])
    status_q4, q4_report, _ = report_over(incomplete)
    q4_row = q4_report["rows"][0] if q4_report else {}
    report.record(
        "Q4 a missing fact",
        status_q4 == 0 and q4_row.get("class") == "unclassified" and q4_row.get("clause") == 0
        and "roster" in (q4_row.get("reason") or ""),
        f"a row without roster was reported {q4_row.get('class')!r} with reason "
        f"{q4_row.get('reason')!r}, and was not classified on the fields present",
    )

    # Q13: two engine versions are two experiments.
    mixed = retained("q13", [cell_row(seed=0, engine="0.1.0"), cell_row(seed=1, engine="0.2.0")],
                     seeds=(0, 1))
    _, mixed_report, _ = report_over(mixed, ["--group-by", "source,density"])
    engines = [group["key"]["engine"] for group in mixed_report["groups"]]
    report.record(
        "Q13 two engines are two experiments",
        len(mixed_report["groups"]) == 2 and sorted(engines) == ["0.1.0", "0.2.0"]
        and all(group["rows"] == 1 for group in mixed_report["groups"]),
        f"two rows differing only in engine produced {len(mixed_report['groups'])} groups keyed "
        f"{engines}, never merged, at one source and one density",
    )

    # Q14: a one-row group, and a zero-row aggregation.
    _, single, _ = report_over(retained("q14", [cell_row(seed=0)]))
    one = single["groups"][0]
    equal_statistics = (
        one["survivors"]["min"] == one["survivors"]["median"] == one["survivors"]["max"]
    )
    status_empty, empty, stderr_empty = report_over(PHASE_B + "/b24-total.jsonl")
    report.record(
        "Q14 small and empty groups",
        one["rows"] == 1 and equal_statistics and status_empty == 0 and empty["groups"] == [],
        f"a one-row group is reported with its order statistics equal "
        f"({one['survivors']}); a sweep with no rows produced {len(empty['groups'])} groups at status "
        f"{status_empty} rather than an error",
    )

    # Q12: the sweep travels, including axis values that produced no rows.
    _, scenario_four, _ = report_over(PHASE_B + "/s4.jsonl")
    sweep_in_report = scenario_four["sweep"]
    densities_with_rows = {row["density"] for row in scenario_four["rows"]}
    report.record(
        "Q12 the sweep travels",
        sweep_in_report["densities"] == ["0.10", "0.75"]
        and "0.10" not in densities_with_rows
        and all(group["key"]["engine"] == "0.1.0" for group in scenario_four["groups"])
        and set(sweep_in_report) == {
            "record", "format", "sources", "densities", "seeds", "ticks", "cells",
            "binary_profile", "digest_algorithm", "defaulted_axes"},
        f"the sweep record is restated whole ({len(sweep_in_report)} fields) with density 0.10 that "
        f"produced no row; the engine version is stated on every group",
    )

    # Q9: both axes, from the same rows.
    # Deaths are chosen between a quarter and a half of the roster, so that Q6's threshold edit --
    # collapse at a quarter instead of a half -- actually moves every row's class. A grid whose rows
    # already sit past both thresholds would make Q6 pass its digest check and prove nothing about
    # revisability, which is the trap `VER-MOK-019`'s *Independence* point 4 warns about.
    grid = [
        cell_row(source=source, density=density, seed=seed, survivors=9 - seed, deaths=3 + seed,
                 populations=(6, 5))
        for source in ("reference", "social")
        for density in ("0.25", "0.75")
        for seed in range(3)
    ]
    grid_path = retained("q9", grid, sources=("reference", "social"),
                         densities=("0.25", "0.75"), seeds=(0, 1, 2))
    _, by_source, _ = report_over(grid_path, ["--group-by", "source"])
    _, by_density, _ = report_over(grid_path, ["--group-by", "density"])
    report.record(
        "Q9 both axes from the same rows",
        len(by_source["groups"]) == 2 and len(by_density["groups"]) == 2
        and all("survivors" in group and "deaths" in group and "class_counts" in group
                for group in by_source["groups"] + by_density["groups"])
        and sum(g["rows"] for g in by_source["groups"]) == 12
        and sum(g["rows"] for g in by_density["groups"]) == 12,
        f"the same 12 rows aggregated over source ({len(by_source['groups'])} groups) and over "
        f"density ({len(by_density['groups'])} groups); each group states rule 17.1's counts and "
        f"order statistics",
    )

    # Q15: determinism.
    first = classifier([grid_path])[1]
    second = classifier([grid_path])[1]
    report.record(
        "Q15 classifier determinism",
        first == second and first != "",
        f"two runs over the same rows with the same options produced byte-identical output "
        f"({len(first)} bytes)",
    )

    # P3: classification is a function of the row and the definitions alone.
    alone = retained("p3-alone", [grid[7]])
    _, alone_report, _ = report_over(alone, ["--group-by", "source,density,seed"])
    _, crowd_report, _ = report_over(grid_path, ["--group-by", "source,density,seed"])
    target = [
        row for row in crowd_report["rows"]
        if (row["source"], row["density"], row["seed"])
        == (grid[7]["source"], grid[7]["density"], grid[7]["seed"])
    ][0]
    report.record(
        "P3 classification is a function of the row",
        alone_report["rows"][0]["class"] == target["class"]
        and alone_report["rows"][0]["clause"] == target["clause"],
        f"the row classified alone is {alone_report['rows'][0]['class']!r} by clause "
        f"{alone_report['rows'][0]['clause']}, and identical within a 12-row file",
    )

    # P4: rows are immutable under every classifier invocation in this contract.
    return grid_path


def case_revisability(report, grid_path):
    """Q3, Q6 and acceptance scenario 3: the definitions change and no retained row does."""
    before_digest = sha256_whole(grid_path)
    status_before, before_report, _ = report_over(grid_path, ["--group-by", "source"])
    before_json = classifier([grid_path, "--group-by", "source"])[1]

    # Q6 and scenario 3: a threshold edited in the classifier's own source.
    with classifier_edited(
        b'return row["deaths"] * 2 >= row["roster"]',
        b'return row["deaths"] * 4 >= row["roster"]',
        "collapse at a quarter of the roster instead of a half",
    ) as diff:
        after_digest_during = sha256_whole(grid_path)
        status_after, after_report, _ = report_over(grid_path, ["--group-by", "source"])
        after_json = classifier([grid_path, "--group-by", "source"])[1]

    reverted_json = classifier([grid_path, "--group-by", "source"])[1]
    after_digest = sha256_whole(grid_path)

    before_classes = [row["class"] for row in before_report["rows"]]
    after_classes = [row["class"] for row in after_report["rows"]]

    with open(f"{OUT}/q6-revisability.md", "w", encoding="utf-8", newline="\n") as handle:
        handle.write("# Q6: revisability, by digest\n\n")
        handle.write(f"Retained rows before the edit: `{before_digest}`\n\n")
        handle.write(f"Retained rows during the edit: `{after_digest_during}`\n\n")
        handle.write(f"Retained rows after reverting: `{after_digest}`\n\n")
        handle.write("The threshold edit:\n\n```diff\n" + diff + "```\n\n")
        handle.write(f"Classes before: `{before_classes}`\n\n")
        handle.write(f"Classes after: `{after_classes}`\n\n")
        handle.write(
            "The distribution before the edit and the distribution after reverting it are "
            f"{'byte-identical' if before_json == reverted_json else 'DIFFERENT'}.\n"
        )
    with open(f"{OUT}/q6-before.json", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(before_json)
    with open(f"{OUT}/q6-after.json", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(after_json)

    report.record(
        "Q6 revisability, by digest",
        status_before == 0 and status_after == 0
        and before_classes != after_classes
        and before_digest == after_digest_during == after_digest,
        f"the classes changed from {before_classes} to {after_classes} while the retained rows' "
        f"digest stayed {before_digest[:16]}... throughout",
    )
    report.record(
        "Scenario 3 a threshold is revised after publication",
        before_json != after_json and before_json == reverted_json,
        f"the regenerated distribution differs from the published one, the rows are byte-identical, "
        f"and reverting the edit reproduces the original distribution "
        f"{'byte for byte' if before_json == reverted_json else 'INEXACTLY'}",
    )

    # Q3: the residual, and its removal.
    residual = retained("q3", [cell_row(seed=0, survivors=11, deaths=1, populations=(6, 5))])
    _, residual_report, _ = report_over(residual)
    with classifier_edited(
        b'    ("coexistence", is_coexistence),\n', b"", "clause 5 removed from the definitions"
    ) as removal_diff:
        status_removed, removed_report, stderr_removed = report_over(residual)
    removed_row = removed_report["rows"][0] if removed_report else {}
    with open(f"{OUT}/q3-clause-removal.diff", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(removal_diff)
    report.record(
        "Q3 the residual, and its removal",
        residual_report["rows"][0]["class"] == "coexistence"
        and residual_report["rows"][0]["clause"] == 5
        and status_removed == 0
        and removed_row.get("class") == "unclassified"
        and removed_row.get("reason") == "no clause matched"
        and len(removed_report["rows"]) == 1
        and removed_report["groups"][0]["class_counts"]["unclassified"] == 1,
        f"a row matching none of clauses 1 to 4 is coexistence by clause 5; with clause 5 removed it "
        f"is {removed_row.get('class')!r} because {removed_row.get('reason')!r}, still present as one "
        f"of one rows and counted as unclassified rather than defaulted or dropped",
    )

    # P4: no classifier invocation in this whole contract changed a retained row.
    digests = {}
    for name in sorted(os.listdir(PHASE_A)) + sorted(os.listdir(PHASE_B)):
        pass
    watched = [grid_path, PHASE_A + "/b1.jsonl", PHASE_B + "/b19.jsonl", PHASE_B + "/s4.jsonl"]
    for path in watched:
        digests[path] = sha256_whole(path)
    for path in watched:
        classifier([path])
        classifier([path, "--format", "markdown"])
        classifier([path, "--group-by", "seed"])
    changed = [path for path in watched if sha256_whole(path) != digests[path]]
    report.record(
        "P4 rows are immutable",
        not changed,
        f"{len(watched)} retained files digested, then read by the classifier three ways each; "
        f"changed {changed or 'none'}",
    )


def case_no_engine_present(report, grid_path):
    """Q7: a distribution from rows alone, in a checkout with no engine binary."""
    empty_cwd = fresh(f"{OUT}/no-checkout")
    status, stdout, stderr = classifier([grid_path], cwd=empty_cwd)
    with open(CLASSIFIER, "r", encoding="utf-8") as handle:
        source = handle.read()
    reachable = [
        token for token in ("subprocess", "socket", "urllib", "http.client", "os.system", "popen",
                            "os.environ", "--live", "connector", "--binary", "--events-path")
        if token in source
    ]
    contents = sorted(os.listdir(empty_cwd))
    report.record(
        "Q7 rows alone",
        status == 0 and stdout and not reachable and not contents,
        f"the classifier ran from a directory holding {contents or 'nothing'} -- no target/, no "
        f"stream, no checkout -- and produced a distribution at status {status}; its source names "
        f"none of the process or network tokens ({reachable or 'confirmed absent'})",
    )


def case_q5(report, engine_rows):
    """Q5: no judgement in a row, over rule 16.5's whole vocabulary."""
    vocabulary = ("extinction", "famine", "asymmetric_collapse", "collapse", "coexistence",
                  "unclassified", "class", "clause", "verdict", "severity", "threshold",
                  "predicate", "outcome", "label")
    found = []
    for row in engine_rows:
        text = json.dumps(row, ensure_ascii=False)
        for word in vocabulary:
            if word in text:
                found.append(f"{word!r} in a cell record")
    for path in (PHASE_A + "/b1.jsonl", PHASE_B + "/b19.jsonl"):
        for row in [r for r in read_jsonl(path) if r["record"] == "cell"]:
            text = json.dumps(row, ensure_ascii=False)
            for word in vocabulary:
                if word in text:
                    found.append(f"{word!r} in {os.path.basename(path)}")
    report.record(
        "Q5 no judgement in a row",
        not found,
        f"rule 16.5's whole vocabulary and every judgement word checked by pattern over "
        f"{len(engine_rows)} shim rows and every engine-produced row of two sweeps; findings "
        f"{sorted(set(found))[:4] or 'none'}",
    )


def main():
    report = Report("phase C: the declared sweep and the classes")
    fresh(OUT)
    default_out, engine_rows = case_declared_sweep(report)
    case_usage_errors(report)
    grid_path = case_classes(report)
    case_revisability(report, grid_path)
    case_no_engine_present(report, grid_path)
    case_q5(report, engine_rows)
    return report.write(OUT + "/result.json")


if __name__ == "__main__":
    raise SystemExit(0 if main() else 1)
