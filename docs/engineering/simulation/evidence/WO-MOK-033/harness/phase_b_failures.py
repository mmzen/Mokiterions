#!/usr/bin/env python3
"""VER-MOK-019 phase B: the cross-checks bite, and incompleteness is per-cell.

B13's fixture half, B14's fixture half, B15, B17, B19 to B26, Q16, Q17 and acceptance scenario 4. Every
case here runs the driver's real command line with a stand-in binary, so the child-process path, the
stream read, the cross-checks and the stage assignment are the driver's own.
"""

from __future__ import annotations

import json
import os
import shutil

import fixtures
from oracle import WORK, Report, classifier, driver, read_jsonl

OUT = WORK + "/phase-b"
SHIM = WORK + "/shim.bat"
PLAN = WORK + "/shim-plan.json"
STORE = WORK + "/fixture-streams"


def fresh(path):
    shutil.rmtree(path, ignore_errors=True)
    os.makedirs(path, exist_ok=True)
    return path


def plan(entries):
    with open(PLAN, "w", encoding="utf-8", newline="\n") as handle:
        handle.write(json.dumps(entries, ensure_ascii=False, indent=1) + "\n")


def sweep(name, args, streams=None):
    """Run a batch through the real command line against the stand-in binary."""
    out = f"{OUT}/{name}.jsonl"
    stream_dir = fresh(f"{OUT}/streams-{name}")
    status, stdout, stderr = driver(
        ["--out", out, "--binary", SHIM, "--ticks", "50", "--stream-dir", stream_dir] + list(args)
    )
    with open(f"{OUT}/{name}-transcript.txt", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(f"status {status}\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}\n")
    return status, out, stdout, stderr


def split(path):
    records = read_jsonl(path)
    return (
        [r for r in records if r["record"] == "sweep"][0],
        [r for r in records if r["record"] == "cell"],
        [r for r in records if r["record"] == "batch"][0],
    )


# ==================================================================================================
# B17: seven fixtures, each breaking exactly one of rule 10.1's equalities by one
# ==================================================================================================

AGENTS_13 = [
    {"id": f"M{index:02d}", "name": f"N{index:02d}", "territory": "A", "died_at": None}
    for index in range(1, 14)
]

BREAKS = [
    ("crossings", {"crossings": 3}, "territory_crossed"),
    ("deaths", {"deaths": 2, "survivors": 10}, "agent_died"),
    ("consumed sum", {"consumed": {"low": 4, "medium": 0, "high": 0}}, "food_consumed"),
    ("regenerated", {"regenerated": 2}, "food_regenerated"),
    ("skipped sum", {"regeneration_skipped": {"depleted": 0, "capacity": 2}},
     "food_regeneration_skipped"),
    ("agents length vs initialized", {"agents": AGENTS_13, "survivors": 12}, "agent_initialized"),
    ("survivors + deaths", {"survivors": 12}, "survivors + deaths"),
]


def case_b17(report):
    fresh(STORE)
    entries = {}
    for index, (label, overrides, _) in enumerate(BREAKS):
        raw = fixtures.stream(events=fixtures.BODY, run_overrides=overrides)
        name = f"b17-{index}-{label.replace(' ', '-')}.jsonl"
        fixtures.write(STORE, name, raw)
        entries[f"social:0.75:{index}"] = {"stream": f"fixture-streams/{name}"}
    plan(entries)

    status, out, _, _ = sweep(
        "b17", ["--sources", "social", "--densities", "0.75", "--seeds", "0-6"]
    )
    sweep_record, rows, batch = split(out)

    faults = []
    if rows:
        faults.append(f"{len(rows)} rows were retained; none should be")
    by_seed = {entry["seed"]: entry for entry in batch["missing"]}
    for index, (label, _, check_name) in enumerate(BREAKS):
        entry = by_seed.get(index)
        if entry is None:
            faults.append(f"seed {index} ({label}) produced no missing entry")
            continue
        if entry["stage"] != "crosscheck":
            faults.append(f"seed {index} ({label}) at stage {entry['stage']!r}")
        if check_name not in entry["reason"]:
            faults.append(f"seed {index} reason does not name {check_name!r}: {entry['reason']}")
        # Rule 10.3: both disagreeing figures are named, and exactly one equality broke.
        if entry["reason"].count("!=") != 1:
            faults.append(f"seed {index} broke {entry['reason'].count('!=')} equalities: {entry['reason']}")

    with open(f"{OUT}/b17-refusals.json", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(json.dumps(
            {"status": status, "retained": batch["retained"], "missing": batch["missing"]},
            ensure_ascii=False, indent=1) + "\n")

    report.record(
        "B17 the seven cross-checks bite",
        status == 3 and not faults and len(batch["missing"]) == 7,
        f"status {status}, 7 fixtures each breaking one equality by one produced {len(rows)} rows and "
        f"{len(batch['missing'])} refusals at stage crosscheck; faults {faults or 'none'}",
    )


# ==================================================================================================
# B13's fixture half, B14's fixture half, B15
# ==================================================================================================


def case_ineffective(report):
    ineffective = {
        0: ("attack_resolved", {"damage": 0, "target_died": "no"}),
        1: ("threat_resolved", {"increase": 0}),
        2: ("surrender_resolved", {"transferred": 0}),
        3: ("attack_resolved", {"damage": 9, "target_died": "yes"}),
    }
    entries = {}
    for seed, (kind, result) in ineffective.items():
        raw = fixtures.stream(events=[fixtures.event(kind, result=result)])
        name = f"b13-{seed}-{kind}.jsonl"
        fixtures.write(STORE, name, raw)
        entries[f"social:0.75:{seed}"] = {"stream": f"fixture-streams/{name}"}
    plan(entries)

    status, out, _, _ = sweep(
        "b13", ["--sources", "social", "--densities", "0.75", "--seeds", "0-3"]
    )
    _, rows, _ = split(out)
    by_seed = {row["seed"]: row for row in rows}

    faults = []
    for seed, (kind, _) in ineffective.items():
        row = by_seed.get(seed)
        if row is None:
            faults.append(f"seed {seed} produced no row")
            continue
        if row["attempted"][kind] != 1:
            faults.append(f"seed {seed} attempted[{kind}] = {row['attempted'][kind]}, not 1")
        expected_effective = 1 if seed == 3 else 0
        if row["effective"][kind] != expected_effective:
            faults.append(
                f"seed {seed} effective[{kind}] = {row['effective'][kind]}, not {expected_effective}"
            )
        # B15: a kind with no conflict states 0 rather than omitting the key.
        for other in row["attempted"]:
            if other == kind:
                continue
            if row["attempted"][other] != 0 or row["effective"][other] != 0:
                faults.append(f"seed {seed} {other} is not zero")

    lethal = by_seed.get(3, {})
    report.record(
        "B13 an ineffective conflict counts as attempted and not as effective",
        status == 0 and not any("attempted" in f or "effective" in f for f in faults),
        f"damage 0, increase 0 and transferred 0 each gave attempted 1 and effective 0; faults "
        f"{[f for f in faults if 'attempted' in f or 'effective' in f] or 'none'}",
    )
    report.record(
        "B14 a lethal attack reports 1, 1 and 1",
        lethal.get("attempted", {}).get("attack_resolved") == 1
        and lethal.get("effective", {}).get("attack_resolved") == 1
        and lethal.get("lethal_attacks") == 1,
        f"attempted {lethal.get('attempted', {}).get('attack_resolved')}, effective "
        f"{lethal.get('effective', {}).get('attack_resolved')}, lethal_attacks "
        f"{lethal.get('lethal_attacks')}",
    )
    report.record(
        "B15 zero is stated, not omitted",
        not any("is not zero" in fault for fault in faults)
        and all(len(row["attempted"]) == 3 and len(row["effective"]) == 3 for row in rows),
        f"every row states all three kinds in both maps; faults "
        f"{[f for f in faults if 'zero' in f] or 'none'}",
    )


# ==================================================================================================
# B19, B20, B21, B22, B23, B26 and scenario 4
# ==================================================================================================

ENGINE_MESSAGE = (
    "error: --density must leave at least one resource per territory\n"
    "Usage: Mokiterions [--seed <number>] [--ticks <number>]\n"
)


def case_incompleteness(report):
    good = fixtures.stream(events=fixtures.BODY)
    fixtures.write(STORE, "good.jsonl", good)

    # A stream whose cross-checks pass but which lacks a field the row needs: the `derive` stage.
    no_final = fixtures.stream(events=fixtures.BODY, drop_run_keys=("final",))
    fixtures.write(STORE, "no-final.jsonl", no_final)

    entries = {
        "*": {"stream": "fixture-streams/good.jsonl"},
        # `run` stage: the engine exits non-zero and its words travel verbatim. B21.
        "reference:0.50:3": {"status": 2, "stderr": ENGINE_MESSAGE},
        # `read` stage: no stream at all.
        "individual:0.25:1": {"no_stream": True},
        # `derive` stage: a stream that reads and cross-checks but cannot produce a row.
        "social:1.00:4": {"stream": "fixture-streams/no-final.jsonl"},
    }
    plan(entries)

    status, out, _, _ = sweep(
        "b19",
        ["--sources", "reference,individual,social", "--densities", "0.25,0.50,0.75,1.00",
         "--seeds", "0-4", "--ticks", "50"],
    )
    sweep_record, rows, batch = split(out)

    # 3 sources x 4 densities x 5 seeds = 60 cells.
    faults = []
    if sweep_record["cells"] != 60:
        faults.append(f"sweep states {sweep_record['cells']} cells, not 60")
    if batch["requested"] != 60 or batch["retained"] != 57 or batch["complete"] is not False:
        faults.append(
            f"requested {batch['requested']}, retained {batch['retained']}, complete "
            f"{batch['complete']}"
        )
    if len(rows) != 57:
        faults.append(f"{len(rows)} cell records, not 57")

    stages = {}
    for entry in batch["missing"]:
        for name in ("source", "density", "seed", "stage", "reason"):
            if name not in entry:
                faults.append(f"a missing entry lacks {name}")
        stages[entry["stage"]] = entry
    report.record(
        "B19 incompleteness is per-cell",
        status == 3 and not faults and len(batch["missing"]) == 3,
        f"status {status}, requested {batch['requested']}, retained {batch['retained']}, complete "
        f"{batch['complete']}, {len(batch['missing'])} named by full coordinates with a stage and a "
        f"reason; faults {faults or 'none'}",
    )
    report.record(
        "B20 three failure stages, distinguishable",
        sorted(stages) == ["derive", "read", "run"],
        f"stages produced here: {sorted(stages)}; reasons: "
        + "; ".join(f"{stage}={stages[stage]['reason'][:70]}" for stage in sorted(stages)),
        note=(
            "B20's remaining three stages: `crosscheck` is B17 above, `not_attempted` is phase E's "
            "interruption, and `refused` is unreachable through the command line -- rule 4.2 refuses "
            "a live source at the sweep level with status 2 before any cell executes, so rule 12.3's "
            "per-cell `refused` stage cannot be produced by any invocation. Disclosed as a "
            "specification gap; the driver's own suite reaches it through the module."
        ),
    )

    verbatim = stages.get("run", {}).get("reason", "")
    report.record(
        "B21 the engine's own words, byte for byte",
        verbatim == f"engine exited 2: {ENGINE_MESSAGE}",
        f"reason is {verbatim!r}; the child's standard error appears unaltered, including its newlines "
        f"and its usage line",
    )

    coordinates = {(row["source"], row["density"], row["seed"]) for row in rows}
    fabricated = [
        cell for cell in (("reference", "0.50", 3), ("individual", "0.25", 1), ("social", "1.00", 4))
        if cell in coordinates
    ]
    duplicates = len(coordinates) != len(rows)
    report.record(
        "B22 nothing is fabricated",
        not fabricated and not duplicates and len(rows) == batch["retained"],
        f"no row exists at the 3 failed coordinates ({fabricated or 'confirmed absent'}); no "
        f"coordinate appears twice ({'duplicates found' if duplicates else 'confirmed'}); cell record "
        f"count {len(rows)} equals retained {batch['retained']}",
    )

    # B26: the 57 rows that survived are complete, valid and classifiable.
    status_class, stdout_class, stderr_class = classifier([out, "--group-by", "source,density"])
    parsed = json.loads(stdout_class) if status_class == 0 else {}
    unclassified = [row for row in parsed.get("rows", []) if row["class"] == "unclassified"]
    report.record(
        "B26 partial rows survive",
        status_class == 0 and len(parsed.get("rows", [])) == 57 and not unclassified,
        f"all {len(parsed.get('rows', []))} surviving rows classified, {len(unclassified)} "
        f"unclassified; none was discarded because another cell failed"
        + (f"; stderr {stderr_class.strip()}" if stderr_class.strip() else ""),
    )

    # Q16: the distribution carries the incompleteness.
    _, markdown, _ = classifier([out, "--format", "markdown"])
    with open(f"{OUT}/q16-distribution.md", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(markdown)
    report.record(
        "Q16 the distribution carries the incompleteness",
        parsed["batch"]["complete"] is False
        and parsed["batch"]["missing"] == 3
        and parsed["batch"]["requested"] == 60
        and "This sweep is incomplete" in markdown,
        f"machine-readable report states complete false, requested 60, retained "
        f"{parsed['batch']['retained']}, missing {parsed['batch']['missing']}; the rendering carries "
        f"the incompleteness above the figures",
    )
    return out


# ==================================================================================================
# Scenario 4: a sweep that loses every cell at one density
# ==================================================================================================


def case_scenario_four(report):
    entries = {"*": {"stream": "fixture-streams/good.jsonl"}}
    for source in ("reference", "social"):
        for seed in range(3):
            entries[f"{source}:0.10:{seed}"] = {"status": 2, "stderr": "error: no food to place\n"}
    plan(entries)

    status, out, _, _ = sweep(
        "s4", ["--sources", "reference,social", "--densities", "0.10,0.75", "--seeds", "0-2"]
    )
    sweep_record, rows, batch = split(out)
    status_class, stdout_class, _ = classifier([out])
    parsed = json.loads(stdout_class) if status_class == 0 else {}
    _, markdown, _ = classifier([out, "--format", "markdown"])
    with open(f"{OUT}/s4-distribution.md", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(markdown)

    densities_in_rows = {row["density"] for row in rows}
    named = {entry["density"] for entry in batch["missing"]}
    report.record(
        "Scenario 4 an axis value lost at one end",
        status == 3
        and "0.10" in sweep_record["densities"]
        and "0.10" not in densities_in_rows
        and named == {"0.10"}
        and "0.10" in markdown,
        f"density 0.10 is stated in the sweep record, absent from the {len(rows)} rows "
        f"(rows carry {sorted(densities_in_rows)}), and named in all {len(batch['missing'])} missing "
        f"entries; the rendering shows it in the sweep and not in the class table",
    )


# ==================================================================================================
# B23, B24, B25, Q17
# ==================================================================================================


def case_statuses(report):
    plan({"*": {"stream": "fixture-streams/good.jsonl"}})
    status_complete, complete_out, _, _ = sweep(
        "b23-complete", ["--sources", "social", "--densities", "0.75", "--seeds", "0-2"]
    )

    plan({"*": {"status": 1, "stderr": "error: everything failed\n"}})
    status_total, total_out, _, _ = sweep(
        "b24-total", ["--sources", "social", "--densities", "0.75", "--seeds", "0-2"]
    )
    total_records = read_jsonl(total_out)
    total_sweep, total_rows, total_batch = split(total_out)

    # Rule 15.1: a usage error exits 2 and writes nothing.
    missing_out = f"{OUT}/b23-usage.jsonl"
    if os.path.isfile(missing_out):
        os.remove(missing_out)
    status_usage, _, stderr_usage = driver(
        ["--out", missing_out, "--binary", SHIM, "--sources", "llm"]
    )

    # Rule 14.1's status 1: the driver could not complete its own work.
    status_absent, _, stderr_absent = driver(
        ["--out", f"{OUT}/b23-nobinary.jsonl", "--binary", f"{OUT}/does-not-exist.exe",
         "--sources", "social", "--densities", "0.75", "--seeds", "0"]
    )

    report.record(
        "B23 the four exit statuses",
        status_complete == 0 and status_total == 3 and status_usage == 2 and status_absent == 1,
        f"0 for a complete sweep, 3 for an incomplete one whose output was written, 2 for a usage "
        f"error, 1 for a driver failure; observed {status_complete}, {status_total}, {status_usage}, "
        f"{status_absent}",
    )
    report.record(
        "B5 the llm refusal",
        status_usage == 2
        and not os.path.isfile(missing_out)
        and "REQ-MOK-072" in stderr_usage,
        f"status {status_usage}, output file "
        f"{'absent' if not os.path.isfile(missing_out) else 'WRITTEN'}, message names REQ-MOK-072: "
        f"{stderr_usage.strip()[:150]}",
    )

    status_class, stdout_class, stderr_class = classifier([total_out])
    parsed = json.loads(stdout_class) if status_class == 0 else {}
    _, total_markdown, _ = classifier([total_out, "--format", "markdown"])
    report.record(
        "B24 total failure",
        status_total == 3
        and len(total_rows) == 0
        and total_batch["complete"] is False
        and [r["record"] for r in total_records] == ["sweep", "batch"]
        and parsed.get("groups") == []
        and "This sweep is incomplete" in total_markdown,
        f"the retained file holds exactly {[r['record'] for r in total_records]} with complete "
        f"{total_batch['complete']} and {total_batch['requested']} requested; the classifier states "
        f"{len(parsed.get('groups', []))} groups and carries the incompleteness",
        note=(
            "read as: the classifier states no distribution *of outcomes* over a sweep with no rows. "
            "It exits 0 with an empty group list rather than refusing, which is Q14's requirement "
            "that a zero-row aggregation is not an error; rule 17.8's refusal is reserved for a "
            "missing sweep record, which Q17 covers."
        ),
    )

    # B25: unwritable output.
    unwritable = f"{OUT}/no-such-directory/out.jsonl"
    status_unwritable, _, stderr_unwritable = driver(
        ["--out", unwritable, "--binary", SHIM, "--sources", "social", "--densities", "0.75",
         "--seeds", "0"]
    )
    report.record(
        "B25 unwritable output",
        status_unwritable == 1 and not os.path.isfile(unwritable),
        f"status {status_unwritable}, nothing retained "
        f"({'confirmed absent' if not os.path.isfile(unwritable) else 'A FILE EXISTS'}); message: "
        f"{stderr_unwritable.strip()[:120]}",
    )

    # Q17: no sweep record.
    orphan = f"{OUT}/q17-no-sweep.jsonl"
    rows_only = [r for r in read_jsonl(complete_out) if r["record"] != "sweep"]
    with open(orphan, "w", encoding="utf-8", newline="\n") as handle:
        for record in rows_only:
            handle.write(json.dumps(record, ensure_ascii=False) + "\n")
    status_orphan, stdout_orphan, stderr_orphan = classifier([orphan])
    report.record(
        "Q17 no sweep record",
        status_orphan == 2 and stdout_orphan == "" and "sweep record" in stderr_orphan,
        f"status {status_orphan} over a file holding {len(rows_only)} records and no sweep record, "
        f"nothing on standard output; message: {stderr_orphan.strip()[:140]}",
    )


def main():
    report = Report("phase B: refusals and incompleteness")
    fresh(OUT)
    case_b17(report)
    case_ineffective(report)
    case_incompleteness(report)
    case_scenario_four(report)
    case_statuses(report)
    return report.write(OUT + "/result.json")


if __name__ == "__main__":
    raise SystemExit(0 if main() else 1)
