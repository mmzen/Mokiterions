#!/usr/bin/env python3
"""Phase F of VER-MOK-019: acceptance scenarios 1, 2, 5 and 6, and the famine measurement.

Scenarios 3 and 4 are met in phase C, where the threshold edit and the lost-axis batch are built from
fixtures. The four here need the real engine over the default sweep, which is 4 sources x 5 densities
x 20 seeds = 400 cells, and they need the classifier's own distribution rather than a recomputation of
it. So this phase runs the sweep once, retains it, and reads it four ways.

`SPEC-MOK-008` rule 17.7 forbids a passing shape, and the contract repeats it: scenario 1 "passes on
the distribution being stated and legible, not on any particular shape". Every statement about the
shape here is therefore a measurement recorded beside the case, not a condition of it. What the cases
do assert is that the figures are legible, that the classifier's classes agree with this contract's own
straight-line oracle on all 400 rows, and -- scenario 5 -- that a row survives its stream's deletion.
"""

from __future__ import annotations

import json
import os
import random
import shutil
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from oracle import (  # noqa: E402
    CLASSIFIER,
    DRIVER,
    KINDS,
    REPO,
    WORK,
    Report,
    classifier,
    driver,
    oracle_classify,
    oracle_crosscheck,
    oracle_scan,
    read_jsonl,
    sha256_whole,
)

OUT = f"{WORK}/phase-f"
SWEEP = f"{OUT}/default-sweep.jsonl"

SOURCES = ("baseline", "reference", "individual", "social")
DENSITIES = ("0.10", "0.25", "0.50", "0.75", "1.00")

# WO-MOK-033 stage 2's measurement over 35 runs at c90edc9, which this phase re-measures over 400.
PRIOR_THREAT = (1448, 1)
PRIOR_DEPLETED = 0
PRIOR_STANDING_FOOD_AT_010 = 8


def keep(name, text):
    os.makedirs(OUT, exist_ok=True)
    with open(f"{OUT}/{name}", "w", encoding="utf-8", newline="\n") as handle:
        handle.write(text if text.endswith("\n") else text + "\n")


def table(headings, rows):
    """A fixed-width table, so a reader compares columns without counting commas."""
    widths = [max(len(str(row[index])) for row in [headings] + rows) for index in range(len(headings))]
    lines = ["  ".join(str(cell).ljust(width) for cell, width in zip(headings, widths)).rstrip(),
             "  ".join("-" * width for width in widths)]
    for row in rows:
        lines.append("  ".join(str(cell).ljust(width) for cell, width in zip(row, widths)).rstrip())
    return lines


# ==================================================================================================
# The sweep, and the two distributions, produced once
# ==================================================================================================


def produce(report):
    """Run the default sweep and both distributions, retaining the commands beside the outputs."""
    if os.path.isdir(OUT):
        shutil.rmtree(OUT)
    os.makedirs(OUT)
    stream_dir = f"{OUT}/streams"
    os.makedirs(stream_dir)

    commands = []
    argv = ["--out", SWEEP, "--stream-dir", stream_dir]
    status, out, err = driver(argv)
    commands.append(f"python {os.path.relpath(DRIVER, REPO)} " + " ".join(argv) + f"   -> exit {status}")
    records = read_jsonl(SWEEP)
    rows = [record for record in records if record.get("record") == "cell"]
    sweep = next(record for record in records if record.get("record") == "sweep")
    batch = next(record for record in records if record.get("record") == "batch")
    digest = sha256_whole(SWEEP)

    reports = {}
    for axes in ("source", "source,density"):
        for form in ("json", "markdown"):
            arguments = [SWEEP, "--group-by", axes, "--format", form]
            code, text, error = classifier(arguments)
            name = f"distribution-{axes.replace(',', '-')}.{'json' if form == 'json' else 'md'}"
            keep(name, text)
            commands.append(
                f"python {os.path.relpath(CLASSIFIER, REPO)} "
                + " ".join(arguments).replace(SWEEP, os.path.basename(SWEEP))
                + f"   -> exit {code}, {len(text.encode('utf-8'))} bytes, kept as {name}")
            if form == "json":
                reports[axes] = json.loads(text)

    # The stream directory is empty afterwards -- rule 7.3 -- so it is removed rather than retained.
    if os.path.isdir(stream_dir) and not os.listdir(stream_dir):
        os.rmdir(stream_dir)

    keep("commands.txt", "\n".join(
        ["The commands that produced every file in this directory, in order, run from the repository",
         "root with the release binary resolved by the driver's own rule 3.", ""] + commands))

    # The same 400 cells were swept in phase E; a separate invocation reproducing the same bytes is
    # worth recording where it is free.
    earlier = f"{WORK}/phase-e/default-sweep-jobs1.jsonl"
    earlier_digest = sha256_whole(earlier) if os.path.isfile(earlier) else None

    report.record(
        "The default sweep and both distributions are produced and retained",
        status == 0 and batch["complete"] and len(rows) == 400
        and all(axes in reports for axes in ("source", "source,density")),
        f"the 400-cell default sweep exited {status} with all 400 cells retained "
        f"({os.path.getsize(SWEEP)} bytes, sha256 {digest[:16]}..., "
        f"{'byte-identical to' if earlier_digest == digest else 'DIFFERENT FROM'} phase E's separate "
        f"invocation) and the classifier produced four reports, two groupings in both formats; "
        f"the commands are retained beside them",
    )
    return rows, sweep, batch, reports, digest


# ==================================================================================================
# Scenario 1: a distribution that discriminates
# ==================================================================================================


def case_scenario_1(report, rows, reports):
    by_source = reports["source"]
    counts = {group["key"]["source"]: group["class_counts"] for group in by_source["groups"]}

    # Independence point 1: the classifier's class for all 400 rows against this contract's own
    # straight-line reading of rule 16.5.
    oracle_counts = {source: {name: 0 for name in by_source["classes"]} for source in SOURCES}
    disagreements = []
    per_row = {(row["source"], row["density"], row["seed"]): row for row in rows}
    for entry in by_source["rows"]:
        row = per_row[(entry["source"], entry["density"], entry["seed"])]
        name, clause = oracle_classify(row)
        oracle_counts[row["source"]][name] += 1
        if name != entry["class"] or clause != entry["clause"]:
            disagreements.append(
                f"{entry['source']}:{entry['density']}:{entry['seed']} classifier {entry['class']}"
                f"/{entry['clause']} oracle {name}/{clause}")

    classes = list(by_source["classes"])
    lines = table(["source"] + classes + ["survivors med", "deaths med"],
                  [[source] + [counts[source][name] for name in classes]
                   + [next(g["survivors"]["median"] for g in by_source["groups"]
                           if g["key"]["source"] == source),
                      next(g["deaths"]["median"] for g in by_source["groups"]
                           if g["key"]["source"] == source)]
                   for source in SOURCES])

    # The three direction statements. Each is a measurement of something an approved artifact already
    # states, and each is recorded whichever way it comes out rather than being a pass condition.
    conflict = {source: {kind: sum(row["attempted"][kind] for row in rows
                                   if row["source"] == source) for kind in KINDS}
                for source in SOURCES}
    quiet = [source for source in SOURCES
             if source != "social" and sum(conflict[source].values()) == 0]
    survivors = {source: sum(row["survivors"] for row in rows if row["source"] == source)
                 for source in SOURCES}
    extinctions = {source: sum(1 for row in rows if row["source"] == source and row["survivors"] == 0)
                   for source in SOURCES}

    directions = [
        ("SPEC-MOK-001's data-and-interface paragraph: none of the three conflict events ever appears "
         "under baseline, reference or individual, because no targeted verb is proposed there",
         f"attempted totals are {json.dumps(conflict)}; the sources with none at all are "
         f"{quiet or 'none'}"),
        ("baseline selects uniformly over rule 4's candidate list and does not seek food, so it should "
         "fare worse than the three sources that do",
         f"survivors summed over 100 cells per source: {json.dumps(survivors)}; extinctions: "
         f"{json.dumps(extinctions)}"),
        ("REQ-MOK-058 states a survivor floor for social three below REQ-MOK-014's for the default, so "
         "social should not outlive reference",
         f"reference {survivors['reference']} survivors against social {survivors['social']}, "
         f"{'which holds' if survivors['social'] <= survivors['reference'] else 'which is reversed'}"),
    ]

    keep("scenario-1-distribution.txt", "\n".join(
        ["Scenario 1: a distribution that discriminates, over the 400-cell default sweep.", "",
         "Class counts per source, 100 cells each. Grouped by source alone, which is not the",
         "classifier's default grouping and is asked for here because the scenario is about the",
         "sources.", ""]
        + lines
        + ["",
           "The same 400 rows classified again by this contract's own straight-line reading of rule",
           f"16.5's five clauses in order: {len(disagreements)} disagreement(s) with the classifier.",
           "Both class and clause number are compared, so a row reaching the right class by the wrong",
           "clause is a disagreement.", ""]
        + [f"  oracle {source:11} {json.dumps(oracle_counts[source])}" for source in SOURCES]
        + [f"  DISAGREEMENT {line}" for line in disagreements]
        + ["",
           "Direction, against what the approved artifacts already say. None of these is a pass",
           "condition: SPEC-MOK-008 rule 17.7 forbids a passing shape, and the contract requires the",
           "distribution to be legible rather than to come out any particular way.", ""]
        + [f"  {claim}\n    measured: {finding}" for claim, finding in directions]))

    report.record(
        "Scenario 1: the distribution is stated, legible, and agrees with the oracle",
        not disagreements and len(by_source["groups"]) == 4
        and all(sum(counts[source].values()) == 100 for source in SOURCES),
        f"four groups of 100 cells, every class named in each with zeros stated; the classifier and "
        f"this contract's oracle agree on all {len(by_source['rows'])} rows including the clause "
        f"number; measured distribution "
        + "; ".join(f"{source} "
                    + ",".join(f"{name}={counts[source][name]}" for name in classes
                               if counts[source][name])
                    for source in SOURCES),
        note=("the direction statements are retained in scenario-1-distribution.txt and are "
              "measurements, not conditions: conflict events appear under social only "
              f"({quiet} have none), and survivors per source are {json.dumps(survivors)}"),
    )


# ==================================================================================================
# Scenario 2: the density axis, and the seed at twenty
# ==================================================================================================


def case_scenario_2(report, rows, reports):
    grouped = reports["source,density"]
    classes = list(grouped["classes"])
    counts = {(group["key"]["source"], group["key"]["density"]): group["class_counts"]
              for group in grouped["groups"]}

    lines = table(["source", "density"] + classes + ["rows"],
                  [[source, density] + [counts[(source, density)][name] for name in classes]
                   + [sum(counts[(source, density)].values())]
                   for source in SOURCES for density in DENSITIES])

    # Does the distribution differ across densities at fixed source? Measured as the number of
    # distinct class-count vectors among the five densities.
    shapes = {}
    for source in SOURCES:
        vectors = [tuple(counts[(source, density)][name] for name in classes) for density in DENSITIES]
        shapes[source] = len({vector for vector in vectors})
    moving = [source for source in SOURCES if shapes[source] > 1]
    pinned = [source for source in SOURCES if shapes[source] == 1]

    # A source whose distribution does not move across densities is only interesting if it *could*
    # have. `baseline` puts all 100 of its cells in one class, so its distribution is at a floor and no
    # density can move it; the run's own figures still move, which is measured here rather than
    # asserted, so that "density moves nothing here" is not confused with "density moves nothing".
    saturation = {source: sorted((name, count) for name, count in
                                 {name: sum(counts[(source, density)][name] for density in DENSITIES)
                                  for name in classes}.items() if count)
                  for source in pinned}
    figures_across_density = {}
    for source in pinned:
        vectors = set()
        for density in DENSITIES:
            members = [row for row in rows if row["source"] == source and row["density"] == density]
            vectors.add((sum(row["deaths"] for row in members),
                         sum(row["crossings"] for row in members),
                         sum(sum(row["consumed"].values()) for row in members),
                         sum(row["run_ticks"] for row in members)))
        figures_across_density[source] = len(vectors)

    # The 2026-08-30 finding at five seeds, re-measured at twenty: does seed change the class inside a
    # group, or only the figures?
    per_group = {}
    for row in rows:
        per_group.setdefault((row["source"], row["density"]), []).append(row)
    split = {}
    figures_move = 0
    for key, members in per_group.items():
        names = {oracle_classify(row)[0] for row in members}
        if len(names) > 1:
            split[f"{key[0]}:{key[1]}"] = sorted(names)
        if len({(row["survivors"], row["deaths"], row["crossings"]) for row in members}) > 1:
            figures_move += 1

    keep("scenario-2-density-and-seed.txt", "\n".join(
        ["Scenario 2: the density axis, and the seed re-measured at twenty seeds.", "",
         "Class counts per source and density, 20 seeds each.", ""]
        + lines
        + ["",
           "Distinct class-count vectors among the five densities, at fixed source. One would mean",
           "density moves nothing for that source.", ""]
        + [f"  {source:11} {shapes[source]} of 5 distinct" for source in SOURCES]
        + ([""] + [
            "A source whose five densities give one vector is at a floor or a ceiling rather than",
            "indifferent to density, and that is measured rather than argued: the class counts below",
            "are the whole 100 cells, and the run figures are summed per density, so a source whose",
            "class is pinned can still be seen to respond.", ""]
           + [f"  {source:11} classes over 100 cells {saturation[source]}; distinct "
              f"(deaths, crossings, consumed, ticks) vectors across the five densities: "
              f"{figures_across_density[source]} of 5" for source in pinned]
           if pinned else [])
        + ["",
           "The seed, at twenty rather than five. WO-MOK-033 stage 2 measured, over 35 runs, that the",
           "seed varies the figures without changing the class. Re-measured over all 20 groups of 20",
           "seeds:",
           "",
           f"  groups whose figures move with the seed        {figures_move} of {len(per_group)}",
           f"  groups whose *class* moves with the seed       {len(split)} of {len(per_group)}", ""]
        + ([f"    {name}: {', '.join(members)}" for name, members in sorted(split.items())]
           or ["    none"])
        + ["",
           "The contract's own words: \"whichever way it comes out is recorded -- a reversal at twenty",
           "seeds is a finding, not a failure\". It came out as a reversal: at twenty seeds the seed",
           "does change the class in some groups, which five seeds did not reach. That is a finding",
           "about the earlier measurement's reach, not about either instrument, and it is reported",
           "unrepaired."] if split else
          ["", "It came out the same way as at five seeds: the seed moves figures and not classes."]))

    report.record(
        "Scenario 2: density moves the class distribution, and the seed is re-measured at twenty",
        # The scenario asks for the difference across densities to be stated as a measurement. It is
        # required here of at least one source, and of every source whose class distribution is not
        # already saturated in a single class: `baseline` puts all 100 cells in `extinction`, so its
        # distribution has nowhere to move and pinning it is a floor rather than a null result.
        bool(moving) and len(per_group) == 20
        and all(len(saturation[source]) == 1 for source in pinned),
        f"at fixed source the five densities produce {', '.join(f'{source} {shapes[source]}/5' for source in SOURCES)} "
        f"distinct class-count vectors, so density moves the outcome for {', '.join(moving)}"
        + (f"; {', '.join(pinned)} does not move because it is at a floor -- all 100 of its cells are "
           f"{saturation[pinned[0]][0][0]} at every density -- and its run figures still answer to "
           f"density in {figures_across_density[pinned[0]]} of 5 distinct vectors" if pinned else "")
        + f"; the seed "
        f"moves the figures in {figures_move} of 20 groups and moves the class in {len(split)}"
        + (f" ({', '.join(sorted(split))}), which reverses the 2026-08-30 five-seed finding and is "
           f"recorded as a finding rather than a failure" if split
           else ", which reproduces the 2026-08-30 five-seed finding"),
    )
    return split


# ==================================================================================================
# Scenario 5: a stream that no longer exists is re-derived
# ==================================================================================================


def case_scenario_5(report, rows, digest):
    """The row is chosen by the sweep's own digest, so the choice is reproducible and not curated.

    "Chosen at random" needs a source of randomness a reviewer can rerun. Seeding from the sweep's
    sha256 means the choice is a function of the sweep alone: a different sweep picks a different row,
    and this sweep picks this row for anyone who repeats the arithmetic.
    """
    picker = random.Random(int(digest[:16], 16))
    index = picker.randrange(len(rows))
    original = rows[index]
    coordinates = (original["source"], original["density"], original["seed"])

    stream_dir = f"{OUT}/rederive-streams"
    os.makedirs(stream_dir, exist_ok=True)
    out = f"{OUT}/rederived.jsonl"
    arguments = ["--out", out, "--stream-dir", stream_dir, "--sources", coordinates[0],
                 "--densities", coordinates[1], "--seeds", str(coordinates[2]),
                 "--ticks", str(original["ticks"]), "--keep-stream", ":".join(
                     (coordinates[0], coordinates[1], str(coordinates[2])))]
    status, text, error = driver(arguments)
    again = [record for record in read_jsonl(out) if record.get("record") == "cell"]
    fresh = again[0] if again else {}

    # Independence point 1 again, and point 3's real engine: the kept stream is counted from its bytes
    # by this contract's oracle, so the reproduction is checked against a count that is not the
    # driver's, and rule 10's seven equalities are recomputed on it.
    kept = f"{stream_dir}/kept-{coordinates[0]}-{coordinates[1]}-{coordinates[2]}.jsonl"
    independent = oracle_scan(kept) if os.path.isfile(kept) else None
    crosschecks = oracle_crosscheck(independent) if independent else []

    differing = sorted(key for key in set(original) | set(fresh)
                       if original.get(key) != fresh.get(key))
    oracle_faults = []
    if independent is None:
        oracle_faults.append("the stream was not kept, so no independent count was taken")
    else:
        if independent["digest"] != original["stream_sha256"]:
            oracle_faults.append("the independent digest differs from the row's stream_sha256")
        for kind in KINDS:
            if independent["attempted"][kind] != original["attempted"][kind]:
                oracle_faults.append(f"attempted {kind}")
            if independent["effective"][kind] != original["effective"][kind]:
                oracle_faults.append(f"effective {kind}")
        if independent["lethal_attacks"] != original["lethal_attacks"]:
            oracle_faults.append("lethal_attacks")
        run = independent["run"]
        for field in ("survivors", "deaths", "crossings", "regenerated"):
            if run[field] != original[field]:
                oracle_faults.append(field)
        if [name for name, _, _, ok in crosschecks if not ok]:
            oracle_faults.append("a rule 10 equality failed on the re-executed stream")

    keep("scenario-5-rederivation.txt", "\n".join([
        "Scenario 5: a stream that no longer exists is re-derived from its coordinates alone.", "",
        f"The 400 rows' file has sha256 {digest}.",
        f"Its first 16 hex digits seed a Mersenne twister, which chooses index {index} of "
        f"{len(rows)}.",
        f"That row's coordinates are source={coordinates[0]} density={coordinates[1]} "
        f"seed={coordinates[2]}, ticks={original['ticks']}.",
        "",
        "The original stream was removed by rule 7.3 when the sweep read it. The cell is re-executed",
        "from the coordinates, with nothing carried over but the coordinates and the horizon:", "",
        f"  exit                    {status}",
        f"  fields differing        {differing or 'none'}",
        f"  stream_sha256 original  {original['stream_sha256']}",
        f"  stream_sha256 again     {fresh.get('stream_sha256')}",
        "",
        "And the re-executed stream is counted from its bytes by this contract's own oracle, so the",
        "reproduction is not checked against the instrument that produced the claim:", "",
        f"  independent digest      {independent['digest'] if independent else 'not taken'}",
        f"  independent bytes       {independent['bytes'] if independent else '-'}",
        f"  independent records     {independent['records'] if independent else '-'}",
        f"  independent attempted   {json.dumps(independent['attempted']) if independent else '-'}",
        f"  independent effective   {json.dumps(independent['effective']) if independent else '-'}",
        f"  rule 10 equalities      "
        + (f"{sum(1 for _, _, _, ok in crosschecks if ok)} of {len(crosschecks)} hold"
           if crosschecks else "not taken"),
        f"  oracle faults           {oracle_faults or 'none'}",
        "",
        "The whole row, as retained the second time:",
        json.dumps(fresh, indent=1),
    ]))

    report.record(
        "Scenario 5: a chosen row is reproduced from its coordinates alone",
        status == 0 and bool(again) and not differing and not oracle_faults,
        f"row {index} of 400, chosen by the sweep's own digest, is {coordinates[0]}:"
        f"{coordinates[1]}:{coordinates[2]}; re-executing that cell reproduced every field of the "
        f"record including stream_sha256 {original['stream_sha256'][:16]}..., and an independent count "
        f"over the re-executed stream's bytes reproduced the digest, all three attempted and "
        f"effective counts, lethal_attacks, survivors, deaths, crossings and regenerated, with "
        f"{sum(1 for _, _, _, ok in crosschecks if ok)} of {len(crosschecks)} rule 10 equalities "
        f"holding",
    )


# ==================================================================================================
# Scenario 6: the instrument finds a defect
# ==================================================================================================


def case_scenario_6(report, rows):
    def totals(subset):
        return (sum(row["attempted"]["threat_resolved"] for row in subset),
                sum(row["effective"]["threat_resolved"] for row in subset))

    attempted, effective = totals(rows)
    by_source = {source: totals([row for row in rows if row["source"] == source])
                 for source in SOURCES}
    by_density = {density: totals([row for row in rows if row["density"] == density])
                  for density in DENSITIES}
    by_pair = {(source, density): totals([row for row in rows if row["source"] == source
                                          and row["density"] == density])
               for source in SOURCES for density in DENSITIES}
    firing_rows = [row for row in rows if row["attempted"]["threat_resolved"] > 0]
    effective_rows = [row for row in rows if row["effective"]["threat_resolved"] > 0]

    keep("threat-inertness.txt", "\n".join(
        ["Scenario 6: the instrument finds a defect. The threat inertness of SPEC-MOK-008 rule 9.6,",
         "reproduced over the 400-cell default sweep and retained as the *before* measurement for a",
         "later repair chain.", "",
         f"whole sweep: {attempted} threats resolved, {effective} of them raised fear "
         f"({effective / attempted:.5%} effective)" if attempted else "no threat was resolved", "",
         f"WO-MOK-033 stage 2's figure at c90edc9, over 35 runs: {PRIOR_THREAT[0]} resolved, "
         f"{PRIOR_THREAT[1]} effective.", "",
         "Per source, 100 cells each:", ""]
        + table(["source", "attempted", "effective"],
                [[source, by_source[source][0], by_source[source][1]] for source in SOURCES])
        + ["", "Per density, 80 cells each:", ""]
        + table(["density", "attempted", "effective"],
                [[density, by_density[density][0], by_density[density][1]]
                 for density in DENSITIES])
        + ["", "Per source and density, 20 cells each:", ""]
        + table(["source", "density", "attempted", "effective"],
                [[source, density, by_pair[(source, density)][0], by_pair[(source, density)][1]]
                 for source in SOURCES for density in DENSITIES])
        + ["",
           f"rows in which a threat resolved at all      {len(firing_rows)} of {len(rows)}",
           f"rows in which one raised fear              {len(effective_rows)} of {len(rows)}",
           "",
           "This is retained as a *before* measurement and nothing here repairs it. The contract's",
           "words: the scenario \"passes when the instrument surfaces it; it does not require the",
           "defect to be absent, and it does not require it to be repaired\". INT-MOK-012 principle 7",
           "is the ground. The repair is a later chain by the owner's decision of 2026-08-30, and this",
           "figure is what that chain will be measured against."]))

    report.record(
        "Scenario 6: the instrument surfaces the threat inertness as a figure",
        attempted > 0 and effective * 100 < attempted,
        f"over 400 cells the engine resolved {attempted} threats and {effective} raised fear "
        f"({effective / attempted:.4%}), against {PRIOR_THREAT[1]} of {PRIOR_THREAT[0]} measured over "
        f"35 runs at c90edc9; the figure is reported per source (only social resolves any: "
        f"{by_source['social'][0]}) and per density "
        + ", ".join(f"{density} {by_density[density][0]}/{by_density[density][1]}"
                    for density in DENSITIES)
        + "; it is retained as the before measurement for a later repair chain and is not repaired "
          "here",
    )


# ==================================================================================================
# The famine measurement
# ==================================================================================================


def case_famine(report, rows, reports):
    depleted = sum(row["regeneration_skipped"]["depleted"] for row in rows)
    capacity = sum(row["regeneration_skipped"]["capacity"] for row in rows)
    depleted_rows = [row for row in rows if row["regeneration_skipped"]["depleted"] > 0]
    by_density = {density: sum(row["regeneration_skipped"]["depleted"] for row in rows
                               if row["density"] == density) for density in DENSITIES}

    low = [row for row in rows if row["density"] == "0.10"]
    standing = []
    for row in sorted(low, key=lambda row: (row["source"], row["seed"])):
        food = {name: figures["low"] + figures["medium"] + figures["high"]
                for name, figures in row["territories"].items()}
        standing.append([row["source"], row["seed"], row["reason"], row["survivors"],
                         *(food[name] for name in sorted(food)), sum(food.values())])
    totals_at_010 = sorted({entry[-1] for entry in standing})
    extinct_at_010 = sum(1 for row in low if row["survivors"] == 0)

    # Rule 16.9 over the real sweep: the class no row reaches is still named in every group.
    missing_famine = [json.dumps(group["key"]) for group in reports["source,density"]["groups"]
                      if "famine" not in group["class_counts"]]
    famine_counted = sum(group["class_counts"].get("famine", 0)
                         for group in reports["source,density"]["groups"])

    keep("famine.txt", "\n".join(
        ["The famine measurement: regeneration_skipped.depleted across the whole 400-cell sweep, and",
         "the standing-food figures at density 0.10 at the final tick.", "",
         f"depleted, summed over 400 runs      {depleted}",
         f"capacity, summed over 400 runs      {capacity}",
         f"runs with any depleted skip         {len(depleted_rows)} of {len(rows)}",
         f"famine rows in the distribution     {famine_counted}",
         f"groups omitting the famine class    {missing_famine or 'none'}", "",
         "depleted per density:", ""]
        + table(["density", "depleted"], [[density, by_density[density]] for density in DENSITIES])
        + ["",
           "Density 0.10, where WO-MOK-033 stage 2 recorded that the population goes extinct in every",
           "seed while both territories still hold 8 units of food. All 80 cells at that density, food",
           "standing at the final tick:", ""]
        + table(["source", "seed", "reason", "survivors", "food A", "food B", "food total"], standing)
        + ["",
           f"distinct standing-food totals at 0.10   {totals_at_010}",
           f"cells extinct at 0.10                   {extinct_at_010} of {len(low)}",
           f"stage 2's figure per territory          {PRIOR_STANDING_FOOD_AT_010}", "",
           "What this measures and what it does not. The famine predicate is rule 16.5's second",
           "clause, and it reads depleted > 0. Across 400 runs the swept space does not reach it, and",
           "at the density where the population always dies the food is still standing, so the deaths",
           "are not starvation by this predicate's own terms. VER-MOK-019's residual uncertainty 4 is",
           "the boundary here: the class is verified as stated and disclosed, not as correct, and",
           "redefining the predicate is a semantics decision for the product owner. This measurement",
           "is the disclosure, not a proposal."]))

    report.record(
        "The famine measurement is taken across the sweep and at density 0.10",
        depleted == PRIOR_DEPLETED and not depleted_rows and not missing_famine
        and famine_counted == 0 and len(low) == 80,
        f"regeneration_skipped.depleted is {depleted} over all 400 runs -- reproducing stage 2's "
        f"figure of {PRIOR_DEPLETED} over 35 -- against {capacity} capacity skips, so no row reaches "
        f"rule 16.5's famine clause and the class is reported with a count of zero in all 20 groups "
        f"rather than omitted; at density 0.10 {extinct_at_010} of 80 cells end extinct with standing "
        f"food totals of {totals_at_010} units across the two territories, so the population dies "
        f"with food on the ground; disclosed and not repaired",
    )


def main():
    report = Report("phase F: acceptance scenarios and the famine measurement")
    rows, sweep, batch, reports, digest = produce(report)
    case_scenario_1(report, rows, reports)
    case_scenario_2(report, rows, reports)
    case_scenario_5(report, rows, digest)
    case_scenario_6(report, rows)
    case_famine(report, rows, reports)
    return 0 if report.write(f"{OUT}/result.json") else 1


if __name__ == "__main__":
    sys.exit(main())
