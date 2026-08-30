#!/usr/bin/env python3
"""Assign one outcome class per retained run and state a distribution over the sweep's axes.

This is the classifier `SPEC-MOK-008` rules 16 to 19 specify. It reads the retained output of
`scripts/run_simulation_batch.py` and nothing else: it executes no process, reads no record stream,
and never writes into retained output. A retained row is immutable once written.

**Every threshold and every clause in this file lives here and in `SPEC-MOK-008`, and in no retained
artifact.** That is what makes a threshold revisable without invalidating a retained capture, which is
`SPEC-MOK-006` rule 8.7's requirement and the reason no `cell` record carries a class. Editing a clause
below changes classes and changes no retained row, which is checkable by digesting the retained output
before and after the edit.

Standard library only, per rule 1.3. This file imports nothing from the batch driver, so it works in a
checkout with no engine binary present, which `REQ-MOK-082` requires.
"""

from __future__ import annotations

import argparse
import json
import sys

REPORT_FORMAT = 1

# --- Rule 16.2: the axes a distribution may aggregate over. ---------------------------------------
GROUPABLE = ("source", "density", "seed")
DEFAULT_GROUP_BY = ("source", "density")

# --- Rule 9.1's three kinds, whose `effective` counts rule 17.1 aggregates. -----------------------
EFFECTIVE_KINDS = ("attack_resolved", "threat_resolved", "surrender_resolved")


# ==================================================================================================
# The outcome vocabulary
# ==================================================================================================
#
# Rule 16.5: five members, ordered predicates, first match wins, and the order is part of the
# definition. Every predicate reads only fields rule 8.1 retains.
#
# The measured support for each at commit c90edc9, over 35 runs, is stated in SPEC-MOK-008 rule 16.5.
# One class is deliberately unreached: see `famine` below.


def is_extinction(row):
    """Nobody survived. Measured in 15 of 35 runs: every `baseline` cell and every cell at 0.10."""
    return row["survivors"] == 0


def is_famine(row):
    """A territory is permanently depleted.

    **Measurement does not support this predicate, and rule 16.8 discloses that rather than resolving
    it.** `regeneration_skipped.depleted` was 0 in all 35 runs at every density including 0.10. At
    0.10, where the population goes extinct in every seed, each territory still ends with 8 units of
    food standing and `depleted` false -- so extinction at low density is a failure to *reach* food
    rather than to have it, and this clause would not have fired even where starvation is the
    intuitive reading.

    The clause is retained because permanent depletion is a real engine state and a class that named
    it is the honest place for it. Inventing a threshold over standing supply to make it fire would
    assert a semantics no measurement supports, which `REQ-MOK-081` forbids.
    """
    return row["regeneration_skipped"]["depleted"] > 0


def is_asymmetric_collapse(row):
    """One territory emptied while another did not. None observed in 35 runs."""
    populations = [figures["population"] for figures in row["territories"].values()]
    return sum(1 for value in populations if value == 0) == 1 and any(
        value > 0 for value in populations
    )


def is_collapse(row):
    """At least half the roster died. Measured in 7 of 35 runs: one `social` cell at 0.75, and 0.25."""
    return row["deaths"] * 2 >= row["roster"]


def is_coexistence(row):
    """The residual clause. Measured in 13 of 35 runs.

    Rule 16.10: while this clause is the residual no row can match no predicate. If the clause set is
    ever edited so that a row can, `classify` reports it as unclassified with its facts named, and
    neither defaults nor drops it.
    """
    return True


# The order is the definition. Rule 16.6: exactly one class per run, which is what makes the class
# counts within a group sum to the group's row count.
CLAUSES = (
    ("extinction", is_extinction),
    ("famine", is_famine),
    ("asymmetric_collapse", is_asymmetric_collapse),
    ("collapse", is_collapse),
    ("coexistence", is_coexistence),
)

CLASSES = tuple(name for name, _ in CLAUSES)


# The fields rule 16.5's predicates read. A row lacking one cannot be classified on the rest.
REQUIRED_FACTS = ("survivors", "deaths", "roster", "regeneration_skipped", "territories")


def missing_facts(row):
    """The facts rule 16.5's predicates need that this row does not carry, named in a fixed order."""
    absent = [name for name in REQUIRED_FACTS if name not in row]

    skipped = row.get("regeneration_skipped")
    if "regeneration_skipped" not in absent and (
        not isinstance(skipped, dict) or "depleted" not in skipped
    ):
        absent.append("regeneration_skipped.depleted")

    territories = row.get("territories")
    if "territories" not in absent:
        if not isinstance(territories, dict):
            absent.append("territories")
        else:
            for name, figures in territories.items():
                if not isinstance(figures, dict) or "population" not in figures:
                    absent.append(f"territories.{name}.population")

    return absent


def classify(row):
    """Return `(class, clause, reason)`. `clause` is the 1-based position that assigned the class.

    Rule 16.7: a class is traceable to its clause rather than to the classifier as a whole. `reason` is
    `None` for a classified row and states why for one that is not.

    A row lacking a fact a predicate reads is reported as unclassified with that fact named, rather
    than classified on the facts that happen to be present. `SPEC-MOK-008` rule 16 states no rule for
    this case; the treatment follows rule 16.10, which is the rule for a row no clause can assign, and
    it adds neither a class to rule 16.5's five nor a threshold.
    """
    absent = missing_facts(row)
    if absent:
        return "unclassified", 0, "missing fact: " + ", ".join(absent)
    for position, (name, predicate) in enumerate(CLAUSES, start=1):
        if predicate(row):
            return name, position, None
    # Reachable only if the clause set is edited so clause 5 is no longer the residual. Rule 16.10.
    return "unclassified", 0, "no clause matched"


# ==================================================================================================
# Reading the retained output
# ==================================================================================================


class InputError(Exception):
    """The retained output cannot be read as a distribution's input."""


def read_retained(path):
    """Return `(sweep, rows, batch)`. Rule 16.4: this is the only thing the classifier reads."""
    sweep = None
    batch = None
    rows = []
    try:
        handle = open(path, "r", encoding="utf-8")
    except OSError as error:
        raise InputError(f"could not read {path!r}: {error}") from error
    with handle:
        for number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            try:
                record = json.loads(line)
            except ValueError as error:
                raise InputError(f"{path}:{number} is not a JSON record: {error}") from error
            kind = record.get("record")
            if kind == "sweep":
                sweep = record
            elif kind == "cell":
                rows.append(record)
            elif kind == "batch":
                batch = record
            else:
                raise InputError(f"{path}:{number} carries an unknown record kind {kind!r}")

    # Rule 17.8: a distribution over retained output whose `sweep` record is absent is refused.
    # Unknown completeness is treated as incomplete.
    if sweep is None:
        raise InputError(
            f"{path} carries no sweep record, so the sweep it covers and its completeness are "
            f"unknown; a distribution over it is refused"
        )
    return sweep, rows, batch


# ==================================================================================================
# The distribution
# ==================================================================================================


def median_lower(values):
    """The median, taking the lower of the two central values for an even-sized group.

    Rule 17.2 fixes this here so it is not an implementation accident, and rule 17.2 is also why this
    returns an order statistic rather than a mean: no figure in the machine-readable output is a
    floating-point value, and a mean over twenty runs invites more confidence than twenty runs support.
    """
    ordered = sorted(values)
    return ordered[(len(ordered) - 1) // 2]


def spread(values):
    """Rule 17.1's dispersion: minimum, median and maximum, all integers.

    `None` where no row in the group carries the figure at all, which is the only honest answer for a
    group whose rows are all unclassifiable for want of it. A retained row written by the batch driver
    always carries every figure, so this arises only over a hand-built or damaged file.
    """
    present = [value for value in values if isinstance(value, int)]
    if not present:
        return None
    return {"min": min(present), "median": median_lower(present), "max": max(present)}


def group_key(row, axes):
    """Build a group's key.

    Rule 17.5: the engine version is an implicit grouping axis and is always part of the key, so rows
    carrying two engine versions are never merged silently. Two engine versions are two experiments.
    """
    key = {"engine": row.get("engine")}
    for axis in axes:
        key[axis] = row.get(axis)
    return key


def distribution(sweep, rows, batch, axes):
    """Build the rule 17 distribution."""
    classified = []
    for row in rows:
        name, clause, reason = classify(row)
        classified.append((row, name, clause, reason))

    ordered_keys = []
    buckets = {}
    for row, name, clause, reason in classified:
        key = group_key(row, axes)
        token = json.dumps(key, sort_keys=True)
        if token not in buckets:
            buckets[token] = {"key": key, "members": []}
            ordered_keys.append(token)
        buckets[token]["members"].append((row, name, clause, reason))

    groups = []
    for token in ordered_keys:
        bucket = buckets[token]
        members = bucket["members"]
        counts = {name: 0 for name in CLASSES}
        counts["unclassified"] = 0
        for _, name, _, _ in members:
            counts[name] += 1
        group = {
            "key": bucket["key"],
            # Rule 17.6: a group with one row is reported, with its order statistics all equal.
            "rows": len(members),
            "class_counts": counts,
            "survivors": spread([row.get("survivors") for row, _, _, _ in members]),
            "deaths": spread([row.get("deaths") for row, _, _, _ in members]),
            # Rule 17.3: `attempted` and `effective` stay distinct through aggregation, and no group
            # figure merges them. Both are aggregated, under separate keys, for all three kinds.
            #
            # Rule 17.1's enumeration names order statistics for `survivors`, `deaths` and the three
            # `effective` counters and does not name `attempted`. Rule 17.3 has no content unless
            # `attempted` survives aggregation -- there is nothing to keep distinct from `effective`
            # otherwise -- and `VER-MOK-019` case Q10 requires both reported for all three kinds. The
            # two are aggregated here on that reading. It adds an integer figure and no float, no
            # class and no threshold, so no rule of 16, 17 or 18 is contradicted; rule 17.1's list is
            # incomplete rather than exclusive, and that is disclosed rather than amended.
            "attempted": {
                kind: spread(
                    [(row.get("attempted") or {}).get(kind) for row, _, _, _ in members]
                )
                for kind in EFFECTIVE_KINDS
            },
            "effective": {
                kind: spread(
                    [(row.get("effective") or {}).get(kind) for row, _, _, _ in members]
                )
                for kind in EFFECTIVE_KINDS
            },
        }
        groups.append(group)

    # Rule 17.4: the sweep is restated in full and the batch's completeness travels with the
    # distribution, so an incomplete sweep's distribution cannot be read without its incompleteness.
    if batch is None:
        completeness = {"complete": False, "requested": None, "retained": len(rows), "missing": None}
    else:
        completeness = {
            "complete": batch["complete"],
            "requested": batch["requested"],
            "retained": batch["retained"],
            "missing": len(batch["missing"]),
        }

    return {
        "report": "distribution",
        "format": REPORT_FORMAT,
        "sweep": sweep,
        "batch": completeness,
        "group_by": list(axes),
        # Rule 16.9: a class observed zero times is reported with a count of zero and is not omitted,
        # because an unobserved class is a finding about the simulation or about the sweep.
        "classes": list(CLASSES),
        "groups": groups,
        # Rule 16.7: each row's class is traceable to the ordered clause that assigned it. `reason` is
        # null for a classified row and states why for a row that could not be classified, so an
        # unclassifiable row is reported with its facts rather than defaulted or dropped.
        "rows": [
            dict(
                group_key(row, GROUPABLE),
                **{"class": name, "clause": clause, "reason": reason},
            )
            for row, name, clause, reason in classified
        ],
    }


# ==================================================================================================
# Rendering
# ==================================================================================================


def render_markdown(report):
    """A rendering, regenerable, and not the authority for any figure. Rule 16.3.

    Rule 17.7: this states counts and no conclusion, verdict, expectation, target distribution or
    passing shape.
    """
    sweep = report["sweep"]
    batch = report["batch"]
    lines = []
    lines.append("# Outcome distribution")
    lines.append("")
    lines.append(
        "A rendering of the machine-readable report, which is the authority for every figure below."
    )
    lines.append("")
    lines.append("## The sweep")
    lines.append("")
    lines.append(f"- sources: {', '.join(sweep['sources'])}")
    lines.append(f"- densities: {', '.join(sweep['densities'])}")
    lines.append(f"- seeds: {_render_seeds(sweep['seeds'])}")
    lines.append(f"- tick horizon: {sweep['ticks']}")
    lines.append(f"- cells requested: {sweep['cells']}")
    lines.append(f"- binary profile: {sweep['binary_profile']}")
    lines.append(f"- digest algorithm: {sweep['digest_algorithm']}")
    lines.append(
        "- defaulted axes: "
        + (", ".join(sweep["defaulted_axes"]) if sweep["defaulted_axes"] else "none")
    )
    lines.append("")
    lines.append("## Completeness")
    lines.append("")
    lines.append(f"- complete: {'yes' if batch['complete'] else 'no'}")
    lines.append(f"- requested: {batch['requested']}")
    lines.append(f"- retained: {batch['retained']}")
    lines.append(f"- missing: {batch['missing']}")
    if not batch["complete"]:
        lines.append("")
        lines.append(
            "**This sweep is incomplete.** The figures below cover the retained rows only and are "
            "not a distribution over the sweep the first section states."
        )
    lines.append("")
    lines.append("## Classes")
    lines.append("")
    axes = report["group_by"]
    header = ["engine"] + list(axes) + ["rows"] + list(report["classes"]) + ["unclassified"]
    lines.append("| " + " | ".join(header) + " |")
    lines.append("|" + "|".join(["---"] * len(header)) + "|")
    for group in report["groups"]:
        cells = [group["key"]["engine"]] + [str(group["key"][axis]) for axis in axes]
        cells.append(str(group["rows"]))
        cells.extend(str(group["class_counts"][name]) for name in report["classes"])
        cells.append(str(group["class_counts"]["unclassified"]))
        lines.append("| " + " | ".join(cells) + " |")
    lines.append("")
    lines.append("## Dispersion")
    lines.append("")
    lines.append("Minimum, median and maximum. The median of an even-sized group is the lower central value.")
    lines.append("")
    measures = ["survivors", "deaths"]
    for kind in EFFECTIVE_KINDS:
        measures.append(f"attempted {kind}")
        measures.append(f"effective {kind}")
    header = ["engine"] + list(axes) + measures
    lines.append("| " + " | ".join(header) + " |")
    lines.append("|" + "|".join(["---"] * len(header)) + "|")
    for group in report["groups"]:
        cells = [group["key"]["engine"]] + [str(group["key"][axis]) for axis in axes]
        cells.append(_render_spread(group["survivors"]))
        cells.append(_render_spread(group["deaths"]))
        for kind in EFFECTIVE_KINDS:
            cells.append(_render_spread(group["attempted"][kind]))
            cells.append(_render_spread(group["effective"][kind]))
        lines.append("| " + " | ".join(cells) + " |")
    lines.append("")
    return "\n".join(lines) + "\n"


def _render_spread(figures):
    if figures is None:
        return "n/a"
    return f"{figures['min']} / {figures['median']} / {figures['max']}"


def _render_seeds(seeds):
    """Render a seed list compactly, collapsing contiguous runs the way `--seeds` accepts them."""
    if not seeds:
        return "none"
    ordered = sorted(seeds)
    pieces = []
    start = previous = ordered[0]
    for value in ordered[1:]:
        if value == previous + 1:
            previous = value
            continue
        pieces.append(str(start) if start == previous else f"{start}-{previous}")
        start = previous = value
    pieces.append(str(start) if start == previous else f"{start}-{previous}")
    return ",".join(pieces)


# ==================================================================================================
# Entry point
# ==================================================================================================


def build_parser():
    parser = argparse.ArgumentParser(
        prog="classify_simulation_runs.py",
        description=(
            "Assign one outcome class per retained run and state a distribution over the sweep's "
            "axes. Reads the retained output of run_simulation_batch.py and nothing else. Every "
            "threshold lives in this file and in SPEC-MOK-008, never in a retained row."
        ),
        allow_abbrev=False,
    )
    parser.add_argument("retained", help="the retained output of run_simulation_batch.py")
    parser.add_argument(
        "--group-by",
        default=",".join(DEFAULT_GROUP_BY),
        help="comma-separated axes from source, density, seed; default source,density",
    )
    parser.add_argument(
        "--format", choices=("json", "markdown"), default="json", help="output form; default json"
    )
    return parser


def parse_group_by(raw):
    axes = [piece.strip() for piece in raw.split(",")]
    if any(axis == "" for axis in axes):
        raise InputError(f"malformed --group-by {raw!r}; axes are comma-separated and none is empty")
    seen = set()
    for axis in axes:
        if axis not in GROUPABLE:
            raise InputError(
                f"--group-by does not accept {axis!r}; it accepts " + ", ".join(GROUPABLE)
            )
        if axis in seen:
            raise InputError(f"--group-by repeats {axis!r}")
        seen.add(axis)
    return axes


def main(argv=None):
    args = build_parser().parse_args(argv)
    try:
        axes = parse_group_by(args.group_by)
        sweep, rows, batch = read_retained(args.retained)
    except InputError as error:
        # Rule 14.3's convention: 2 is a configuration error, so an operator sees one convention
        # across both instruments. Rule 14.1 states statuses for the batch driver only.
        print(f"configuration error: {error}", file=sys.stderr)
        return 2

    report = distribution(sweep, rows, batch, axes)

    if args.format == "markdown":
        sys.stdout.write(render_markdown(report))
    else:
        # Rule 18.2: two runs over the same retained output with the same options are byte-identical.
        sys.stdout.write(json.dumps(report, ensure_ascii=False, indent=1, sort_keys=False) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
