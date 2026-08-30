#!/usr/bin/env python3
"""Tests for `scripts/classify_simulation_runs.py`, covering `SPEC-MOK-008` rule 20.1's classifier cases.

Rule 20.2: these tests execute no real engine run. They also execute no batch: the classifier reads
retained output and nothing else, so its fixtures are retained files written here. This file imports
nothing from the batch driver, which is rule 1.5's independence checked in both directions.

The load-bearing case is `test_editing_a_threshold_changes_classes_and_changes_no_retained_byte`. It is
what makes `SPEC-MOK-006` rule 8.7 true in practice rather than in intent.

Standard library only, per rule 1.3.
"""

from __future__ import annotations

import contextlib
import hashlib
import io
import json
import os
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import classify_simulation_runs as classifier  # noqa: E402


# ==================================================================================================
# Retained fixtures
# ==================================================================================================


def cell_row(
    source="social",
    density="0.75",
    seed=0,
    engine="0.1.0",
    survivors=8,
    deaths=4,
    roster=12,
    depleted=0,
    populations=(6, 6),
    effective=(3, 0, 1),
    attempted=(9, 40, 5),
):
    """One rule 8.1 `cell` record, carrying exactly the fields the classifier's predicates read."""
    return {
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
        "crossings": 17,
        "consumed": {"low": 100, "medium": 20, "high": 5},
        "regenerated": 90,
        "regeneration_skipped": {"depleted": depleted, "capacity": 11},
        "territories": {
            "A": {"population": populations[0], "low": 8, "medium": 3, "high": 1},
            "B": {"population": populations[1], "low": 9, "medium": 2, "high": 0},
        },
        "attempted": dict(zip(classifier.EFFECTIVE_KINDS, attempted)),
        "effective": dict(zip(classifier.EFFECTIVE_KINDS, effective)),
        "lethal_attacks": 2,
        "crosschecks_passed": 7,
    }


def sweep_row(sources=("social",), densities=("0.75",), seeds=(0,), cells=None, defaulted=()):
    return {
        "record": "sweep",
        "format": 1,
        "sources": list(sources),
        "densities": list(densities),
        "seeds": sorted(seeds),
        "ticks": 1000,
        "cells": cells if cells is not None else len(sources) * len(densities) * len(seeds),
        "binary_profile": "release",
        "digest_algorithm": "sha256",
        "defaulted_axes": list(defaulted),
    }


def batch_row(requested, retained, missing=()):
    return {
        "record": "batch",
        "complete": retained == requested,
        "requested": requested,
        "retained": retained,
        "missing": list(missing),
        "leftover_streams": [],
    }


def write_retained(directory, name, records):
    path = os.path.join(directory, name)
    with open(path, "w", encoding="utf-8", newline="\n") as handle:
        for record in records:
            handle.write(json.dumps(record, ensure_ascii=False, separators=(",", ":")) + "\n")
    return path


def floats_in(value, path="$"):
    """Every float reachable in a structure, with its path. Rule 18.3 admits none."""
    found = []
    if isinstance(value, float):
        found.append(path)
    elif isinstance(value, dict):
        for key, item in value.items():
            found.extend(floats_in(item, f"{path}.{key}"))
    elif isinstance(value, list):
        for index, item in enumerate(value):
            found.extend(floats_in(item, f"{path}[{index}]"))
    return found


def quiet_main(argv):
    """Call the classifier's entry point and return `(status, stdout, stderr)`."""
    out, err = io.StringIO(), io.StringIO()
    with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
        status = classifier.main(argv)
    return status, out.getvalue(), err.getvalue()


# ==================================================================================================
# Rule 16.5: the five ordered predicates
# ==================================================================================================


class TestPredicates(unittest.TestCase):
    def test_the_vocabulary_has_exactly_five_members_in_the_specified_order(self):
        """Rule 16.5: the order is part of the definition, so it is asserted rather than assumed."""
        self.assertEqual(
            classifier.CLASSES,
            ("extinction", "famine", "asymmetric_collapse", "collapse", "coexistence"),
        )
        self.assertEqual(len(classifier.CLAUSES), 5)

    def test_clause_1_extinction(self):
        self.assertEqual(classifier.classify(cell_row(survivors=0, deaths=0)), ("extinction", 1, None))

    def test_clause_2_famine(self):
        row = cell_row(survivors=8, deaths=1, depleted=3)
        self.assertEqual(classifier.classify(row), ("famine", 2, None))

    def test_clause_3_asymmetric_collapse(self):
        row = cell_row(survivors=6, deaths=1, populations=(0, 6))
        self.assertEqual(classifier.classify(row), ("asymmetric_collapse", 3, None))

    def test_clause_4_collapse(self):
        row = cell_row(survivors=6, deaths=6, roster=12)
        self.assertEqual(classifier.classify(row), ("collapse", 4, None))

    def test_clause_5_coexistence_is_the_residual(self):
        row = cell_row(survivors=11, deaths=1, roster=12)
        self.assertEqual(classifier.classify(row), ("coexistence", 5, None))

    def test_collapse_is_at_least_half_the_roster_not_more_than_half(self):
        """Rule 16.5's threshold, at its boundary. Six of twelve is a collapse; five is not."""
        self.assertEqual(classifier.classify(cell_row(survivors=6, deaths=6))[0], "collapse")
        self.assertEqual(classifier.classify(cell_row(survivors=7, deaths=5))[0], "coexistence")

    def test_an_odd_roster_at_the_boundary(self):
        """`deaths * 2 >= roster` is stated in integers, so an odd roster has no rounding question."""
        self.assertEqual(classifier.classify(cell_row(deaths=6, roster=11, survivors=5))[0], "collapse")
        self.assertEqual(
            classifier.classify(cell_row(deaths=5, roster=11, survivors=6))[0], "coexistence"
        )

    def test_an_emptied_territory_beside_an_emptied_one_is_not_asymmetric(self):
        """Rule 16.5: asymmetry needs one emptied and another not. Both emptied is extinction's case."""
        row = cell_row(survivors=3, deaths=1, populations=(0, 0))
        self.assertFalse(classifier.is_asymmetric_collapse(row))

    def test_famines_predicate_reads_depleted_and_not_standing_supply(self):
        """Rule 16.8: the clause is retained and no threshold over standing food is invented for it.

        This is the finding `WO-MOK-033` discloses. A row with a territory at zero food but
        `depleted` false is not a famine, because the engine's own depletion flag is the only fact
        available and it says otherwise.
        """
        row = cell_row(survivors=8, deaths=1, depleted=0)
        row["territories"]["A"].update({"low": 0, "medium": 0, "high": 0})
        self.assertFalse(classifier.is_famine(row))
        self.assertEqual(classifier.classify(row)[0], "coexistence")


class TestClauseOrdering(unittest.TestCase):
    """Rule 16.6: exactly one class per run, so where two clauses match the earlier one decides."""

    def test_extinction_precedes_collapse(self):
        """Nobody survived and everybody died. Both clause 1 and clause 4 match; clause 1 decides."""
        row = cell_row(survivors=0, deaths=12, roster=12)
        self.assertTrue(classifier.is_extinction(row))
        self.assertTrue(classifier.is_collapse(row))
        self.assertEqual(classifier.classify(row), ("extinction", 1, None))

    def test_famine_precedes_collapse(self):
        row = cell_row(survivors=4, deaths=8, roster=12, depleted=2)
        self.assertTrue(classifier.is_famine(row))
        self.assertTrue(classifier.is_collapse(row))
        self.assertEqual(classifier.classify(row), ("famine", 2, None))

    def test_asymmetric_collapse_precedes_collapse(self):
        row = cell_row(survivors=4, deaths=8, roster=12, populations=(0, 4))
        self.assertTrue(classifier.is_asymmetric_collapse(row))
        self.assertTrue(classifier.is_collapse(row))
        self.assertEqual(classifier.classify(row), ("asymmetric_collapse", 3, None))

    def test_extinction_precedes_famine(self):
        """The earliest matching clause decides however many later ones also match."""
        row = cell_row(survivors=0, deaths=12, roster=12, depleted=5, populations=(0, 0))
        for predicate in (
            classifier.is_extinction,
            classifier.is_famine,
            classifier.is_collapse,
        ):
            self.assertTrue(predicate(row), predicate.__name__)
        self.assertEqual(classifier.classify(row), ("extinction", 1, None))

    def test_every_row_gets_exactly_one_class_and_the_clause_that_assigned_it(self):
        """Rule 16.7: a class is traceable to its clause rather than to the classifier as a whole."""
        rows = [
            cell_row(survivors=0, deaths=12),
            cell_row(survivors=8, deaths=1, depleted=1),
            cell_row(survivors=6, deaths=1, populations=(0, 6)),
            cell_row(survivors=6, deaths=6),
            cell_row(survivors=11, deaths=1),
        ]
        assigned = [classifier.classify(row) for row in rows]
        self.assertEqual([name for name, _, _ in assigned], list(classifier.CLASSES))
        self.assertEqual([clause for _, clause, _ in assigned], [1, 2, 3, 4, 5])
        self.assertEqual([reason for _, _, reason in assigned], [None] * 5)


class TestUnclassified(unittest.TestCase):
    def test_a_row_matching_no_clause_is_reported_rather_than_defaulted_or_dropped(self):
        """Rule 16.10: reachable only if the clause set is edited so clause 5 is not the residual.

        The residual is checked by editing it away, because a residual clause cannot be tested any
        other way, and rule 16.10 states an outcome for exactly this edit.
        """
        original = classifier.CLAUSES
        self.addCleanup(setattr, classifier, "CLAUSES", original)
        classifier.CLAUSES = original[:4]

        row = cell_row(survivors=11, deaths=1)
        self.assertEqual(classifier.classify(row), ("unclassified", 0, "no clause matched"))

        report = classifier.distribution(sweep_row(), [row], batch_row(1, 1), ["source", "density"])
        group = report["groups"][0]
        self.assertEqual(group["rows"], 1)
        self.assertEqual(group["class_counts"]["unclassified"], 1)
        # The row is still present with its facts, so a reader can see what went unclassified.
        self.assertEqual(report["rows"][0]["class"], "unclassified")
        self.assertEqual(report["rows"][0]["seed"], 0)
        self.assertEqual(report["rows"][0]["reason"], "no clause matched")

    def test_a_row_lacking_a_fact_a_predicate_reads_is_reported_with_that_fact_named(self):
        """`VER-MOK-019` case Q4: not classified on the facts that happen to be present.

        `SPEC-MOK-008` rule 16 states no rule for this case. The treatment follows rule 16.10, which
        is the rule for a row no clause can assign, and adds neither a class nor a threshold.
        """
        for field in classifier.REQUIRED_FACTS:
            with self.subTest(missing=field):
                row = cell_row(survivors=0)  # An extinction, were the row complete.
                del row[field]
                name, clause, reason = classifier.classify(row)
                self.assertEqual((name, clause), ("unclassified", 0))
                self.assertIn(field, reason)
                self.assertTrue(reason.startswith("missing fact:"), reason)

    def test_a_nested_fact_is_named_by_its_path(self):
        """A row carrying `regeneration_skipped` without `depleted` is as unclassifiable as one without it."""
        row = cell_row(survivors=8, deaths=1)
        del row["regeneration_skipped"]["depleted"]
        _, _, reason = classifier.classify(row)
        self.assertEqual(reason, "missing fact: regeneration_skipped.depleted")

        row = cell_row(survivors=8, deaths=1)
        del row["territories"]["B"]["population"]
        _, _, reason = classifier.classify(row)
        self.assertEqual(reason, "missing fact: territories.B.population")

    def test_an_unclassifiable_row_does_not_stop_the_distribution(self):
        """The other rows in its group are still aggregated, and the group's figures still hold."""
        good = cell_row(seed=0, survivors=11, deaths=1)
        damaged = cell_row(seed=1, survivors=11, deaths=1)
        del damaged["roster"]
        report = classifier.distribution(
            sweep_row(seeds=(0, 1)), [good, damaged], batch_row(2, 2), ["source", "density"]
        )
        group = report["groups"][0]
        self.assertEqual(group["rows"], 2)
        self.assertEqual(group["class_counts"]["coexistence"], 1)
        self.assertEqual(group["class_counts"]["unclassified"], 1)
        self.assertEqual(sum(group["class_counts"].values()), 2)
        # `survivors` is carried by both rows, so its order statistics still range over both.
        self.assertEqual(group["survivors"], {"min": 11, "median": 11, "max": 11})

    def test_a_group_whose_rows_all_lack_a_figure_states_it_as_absent(self):
        """A figure no row carries is `null` rather than a fabricated zero."""
        row = cell_row(seed=0)
        del row["survivors"]
        report = classifier.distribution(
            sweep_row(), [row], batch_row(1, 1), ["source", "density"]
        )
        self.assertIsNone(report["groups"][0]["survivors"])
        self.assertEqual(report["groups"][0]["deaths"], {"min": 4, "median": 4, "max": 4})


# ==================================================================================================
# The invariance a retained capture depends on
# ==================================================================================================


class TestThresholdInvariance(unittest.TestCase):
    def test_editing_a_threshold_changes_classes_and_changes_no_retained_byte(self):
        """`SPEC-MOK-006` rule 8.7 and `REQ-MOK-081`, checked as a digest either side of the edit.

        `VER-MOK-019`'s reason for making this a case: the claim that a threshold is revisable without
        invalidating a retained capture is the whole reason no `cell` record carries a class. An
        assertion about where a threshold lives is checkable; an intention is not.
        """
        rows = [
            cell_row(seed=0, survivors=7, deaths=5, roster=12),
            cell_row(seed=1, survivors=8, deaths=4, roster=12),
            cell_row(seed=2, survivors=6, deaths=6, roster=12),
        ]
        with tempfile.TemporaryDirectory() as directory:
            path = write_retained(
                directory,
                "retained.jsonl",
                [sweep_row(seeds=(0, 1, 2))] + rows + [batch_row(3, 3)],
            )
            with open(path, "rb") as handle:
                before = handle.read()

            status, first, _ = quiet_main([path])
            self.assertEqual(status, 0)
            before_classes = [row["class"] for row in json.loads(first)["rows"]]

            # The edit: a collapse is now any death at all. Nothing else changes.
            original = classifier.CLAUSES
            self.addCleanup(setattr, classifier, "CLAUSES", original)
            classifier.CLAUSES = tuple(
                (name, (lambda row: row["deaths"] > 0) if name == "collapse" else predicate)
                for name, predicate in original
            )

            status, second, _ = quiet_main([path])
            self.assertEqual(status, 0)
            after_classes = [row["class"] for row in json.loads(second)["rows"]]

            with open(path, "rb") as handle:
                after = handle.read()

        self.assertEqual(before_classes, ["coexistence", "coexistence", "collapse"])
        self.assertEqual(after_classes, ["collapse", "collapse", "collapse"])
        self.assertNotEqual(before_classes, after_classes)
        # The retained rows are untouched, byte for byte, by a threshold edit that reclassified them.
        self.assertEqual(hashlib.sha256(before).hexdigest(), hashlib.sha256(after).hexdigest())

    def test_the_classifier_never_opens_the_retained_output_for_writing(self):
        """`REQ-MOK-081`: a retained row is immutable once written, so nothing here writes one."""
        with open(classifier.__file__, "r", encoding="utf-8") as handle:
            source = handle.read()
        self.assertNotIn('"w"', source)
        self.assertNotIn("'w'", source)
        self.assertNotIn('"a"', source)


# ==================================================================================================
# Rule 17: the distribution
# ==================================================================================================


DERIVE = object()  # A sentinel, because `batch=None` is itself a case rule 17.4 states an outcome for.


class TestDistribution(unittest.TestCase):
    def report(self, rows, axes=("source", "density"), sweep=None, batch=DERIVE):
        return classifier.distribution(
            sweep or sweep_row(),
            rows,
            batch_row(len(rows), len(rows)) if batch is DERIVE else batch,
            list(axes),
        )

    def test_class_counts_within_a_group_sum_to_its_row_count(self):
        """Rule 16.6's consequence, which is the check that exactly one class was assigned per run."""
        rows = [
            cell_row(seed=0, survivors=0, deaths=12),
            cell_row(seed=1, survivors=6, deaths=6),
            cell_row(seed=2, survivors=11, deaths=1),
            cell_row(seed=3, survivors=11, deaths=1),
        ]
        group = self.report(rows)["groups"][0]
        self.assertEqual(sum(group["class_counts"].values()), group["rows"])
        self.assertEqual(group["rows"], 4)
        self.assertEqual(group["class_counts"]["extinction"], 1)
        self.assertEqual(group["class_counts"]["collapse"], 1)
        self.assertEqual(group["class_counts"]["coexistence"], 2)

    def test_a_class_observed_zero_times_is_reported_as_zero(self):
        """Rule 16.9: an unobserved class is a finding about the simulation or about the sweep.

        `famine` is the measured case: zero in all 35 runs, and omitting it would hide that.
        """
        report = self.report([cell_row(survivors=11, deaths=1)])
        counts = report["groups"][0]["class_counts"]
        for name in classifier.CLASSES:
            self.assertIn(name, counts)
        self.assertEqual(counts["famine"], 0)
        self.assertEqual(counts["asymmetric_collapse"], 0)
        self.assertEqual(report["classes"], list(classifier.CLASSES))

    def test_dispersion_is_minimum_median_and_maximum(self):
        """Rule 17.1."""
        rows = [cell_row(seed=index, survivors=value) for index, value in enumerate([2, 9, 4, 7])]
        group = self.report(rows)["groups"][0]
        self.assertEqual(group["survivors"], {"min": 2, "median": 4, "max": 9})

    def test_the_median_of_an_even_sized_group_is_the_lower_central_value(self):
        """Rule 17.2, fixed in the specification so it is not an implementation accident."""
        self.assertEqual(classifier.median_lower([1, 2, 3, 4]), 2)
        self.assertEqual(classifier.median_lower([1, 2, 3]), 2)
        self.assertEqual(classifier.median_lower([5]), 5)
        self.assertEqual(classifier.median_lower([4, 4, 4, 4]), 4)

    def test_no_figure_in_the_report_is_a_floating_point_value(self):
        """Rule 17.2 and 18.3: no mean, so no float, so no figure claims unearned precision."""
        rows = [cell_row(seed=index, survivors=index) for index in range(3)]
        self.assertEqual(floats_in(self.report(rows)), [])

    def test_attempted_and_effective_stay_distinct_through_aggregation(self):
        """Rule 17.3: no group figure merges the two, which is the threat finding's whole point.

        A group here attempted 40 threats per run and 0 were effective. A merged figure would make
        that mechanism look active.
        """
        rows = [cell_row(seed=index, effective=(3, 0, 1), attempted=(9, 40, 5)) for index in range(3)]
        group = self.report(rows)["groups"][0]
        self.assertEqual(
            group["effective"]["threat_resolved"], {"min": 0, "median": 0, "max": 0}
        )
        self.assertEqual(set(group["effective"]), set(classifier.EFFECTIVE_KINDS))
        for name in group:
            self.assertNotIn("attempted_effective", name)
            self.assertNotIn("ineffective", name)

    def test_both_counters_are_reported_for_all_three_kinds(self):
        """`VER-MOK-019` case Q10, on rule 17.3's reading.

        Rule 17.1 enumerates order statistics for `survivors`, `deaths` and the three `effective`
        counters. `attempted` is aggregated beside them because rule 17.3 has nothing to keep distinct
        otherwise, and Q10 asks for both. A group carrying only one of the two would let the threat
        finding disappear at aggregation: 40 attempts and 0 effects read as an inactive mechanism from
        `effective` alone, and as an active one from `attempted` alone.
        """
        rows = [cell_row(seed=index, effective=(3, 0, 1), attempted=(9, 40, 5)) for index in range(3)]
        group = self.report(rows)["groups"][0]
        self.assertEqual(set(group["attempted"]), set(classifier.EFFECTIVE_KINDS))
        self.assertEqual(group["attempted"]["threat_resolved"], {"min": 40, "median": 40, "max": 40})
        self.assertEqual(group["effective"]["threat_resolved"], {"min": 0, "median": 0, "max": 0})
        self.assertNotEqual(group["attempted"], group["effective"])

    def test_the_rendering_states_both_counters_side_by_side(self):
        """Rule 16.3: a rendering of the same figures, so it carries both columns per kind."""
        rows = [cell_row(seed=index, effective=(3, 0, 1), attempted=(9, 40, 5)) for index in range(3)]
        rendered = classifier.render_markdown(self.report(rows))
        for kind in classifier.EFFECTIVE_KINDS:
            self.assertIn(f"attempted {kind}", rendered)
            self.assertIn(f"effective {kind}", rendered)

    def test_the_sweep_is_restated_in_full_and_completeness_travels_with_it(self):
        """Rule 17.4: an incomplete sweep's distribution cannot be read without its incompleteness."""
        sweep = sweep_row(sources=("social", "baseline"), seeds=(0, 1, 2), cells=6)
        missing = [{"source": "baseline", "density": "0.75", "seed": 2, "stage": "run", "reason": "x"}]
        report = self.report(
            [cell_row(seed=0), cell_row(seed=1)],
            sweep=sweep,
            batch=batch_row(6, 2, missing),
        )
        self.assertEqual(report["sweep"], sweep)
        self.assertIs(report["batch"]["complete"], False)
        self.assertEqual(report["batch"]["requested"], 6)
        self.assertEqual(report["batch"]["retained"], 2)
        self.assertEqual(report["batch"]["missing"], 1)

    def test_a_retained_file_with_no_batch_record_is_treated_as_incomplete(self):
        """Rule 17.4 and 17.8's principle: unknown completeness is not completeness."""
        report = self.report([cell_row()], batch=None)
        self.assertIs(report["batch"]["complete"], False)
        self.assertIsNone(report["batch"]["requested"])

    def test_the_engine_version_is_always_a_grouping_axis(self):
        """Rule 17.5: two engine versions are two experiments and are never merged silently."""
        rows = [
            cell_row(seed=0, engine="0.1.0", survivors=11, deaths=1),
            cell_row(seed=1, engine="0.2.0", survivors=11, deaths=1),
        ]
        report = self.report(rows)
        self.assertEqual(len(report["groups"]), 2)
        self.assertEqual(
            [group["key"]["engine"] for group in report["groups"]], ["0.1.0", "0.2.0"]
        )
        for group in report["groups"]:
            self.assertEqual(group["rows"], 1)

    def test_engine_is_in_the_key_even_when_group_by_names_only_seed(self):
        """Rule 17.5 holds unconditionally, not only for the default axes."""
        report = self.report([cell_row()], axes=("seed",))
        self.assertEqual(set(report["groups"][0]["key"]), {"engine", "seed"})

    def test_a_group_with_one_row_is_reported_with_equal_order_statistics(self):
        """Rule 17.6: not suppressed, and not presented as a distribution."""
        group = self.report([cell_row(survivors=5, deaths=7)])["groups"][0]
        self.assertEqual(group["rows"], 1)
        self.assertEqual(group["survivors"], {"min": 5, "median": 5, "max": 5})
        self.assertEqual(group["deaths"], {"min": 7, "median": 7, "max": 7})

    def test_grouping_by_all_three_axes_gives_one_group_per_row(self):
        rows = [cell_row(seed=index) for index in range(4)]
        report = self.report(rows, axes=("source", "density", "seed"))
        self.assertEqual(len(report["groups"]), 4)
        self.assertTrue(all(group["rows"] == 1 for group in report["groups"]))

    def test_groups_appear_in_first_encountered_order(self):
        """Rule 18.2's determinism: group order is a function of the retained row order."""
        rows = [
            cell_row(source="social", seed=0),
            cell_row(source="baseline", seed=0),
            cell_row(source="social", seed=1),
        ]
        report = self.report(rows)
        self.assertEqual(
            [group["key"]["source"] for group in report["groups"]], ["social", "baseline"]
        )

    def test_a_distribution_over_zero_rows_is_a_report_with_no_groups(self):
        """An empty sweep is reported rather than crashing: rule 17.6's principle at the limit."""
        report = self.report([], batch=batch_row(3, 0))
        self.assertEqual(report["groups"], [])
        self.assertEqual(report["rows"], [])
        self.assertIs(report["batch"]["complete"], False)


# ==================================================================================================
# Rule 17.8 and rule 14.3: reading input, and refusing it
# ==================================================================================================


class TestInput(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)

    def test_a_retained_file_with_no_sweep_record_is_refused(self):
        """Rule 17.8: without the sweep record neither the space swept nor its completeness is known."""
        path = write_retained(self.directory.name, "no-sweep.jsonl", [cell_row(), batch_row(1, 1)])
        status, _, err = quiet_main([path])
        self.assertEqual(status, 2)
        self.assertIn("no sweep record", err)
        self.assertIn("refused", err)

    def test_a_complete_retained_file_is_read(self):
        path = write_retained(
            self.directory.name, "ok.jsonl", [sweep_row(), cell_row(), batch_row(1, 1)]
        )
        sweep, rows, batch = classifier.read_retained(path)
        self.assertEqual(sweep["record"], "sweep")
        self.assertEqual(len(rows), 1)
        self.assertIs(batch["complete"], True)

    def test_an_unknown_record_kind_is_refused(self):
        """A retained file the classifier does not fully understand is not partially understood."""
        path = write_retained(
            self.directory.name,
            "odd.jsonl",
            [sweep_row(), {"record": "metrics", "tick": 3}, batch_row(1, 1)],
        )
        status, _, err = quiet_main([path])
        self.assertEqual(status, 2)
        self.assertIn("unknown record kind", err)

    def test_a_malformed_line_names_its_line_number(self):
        path = os.path.join(self.directory.name, "broken.jsonl")
        with open(path, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(json.dumps(sweep_row()) + "\n")
            handle.write("{not json\n")
        status, _, err = quiet_main([path])
        self.assertEqual(status, 2)
        self.assertIn("broken.jsonl:2", err)

    def test_a_missing_file_is_refused_rather_than_raising(self):
        status, _, err = quiet_main([os.path.join(self.directory.name, "absent.jsonl")])
        self.assertEqual(status, 2)
        self.assertIn("could not read", err)

    def test_blank_lines_are_skipped(self):
        path = os.path.join(self.directory.name, "blanks.jsonl")
        with open(path, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(json.dumps(sweep_row()) + "\n\n")
            handle.write(json.dumps(cell_row()) + "\n\n")
            handle.write(json.dumps(batch_row(1, 1)) + "\n")
        sweep, rows, batch = classifier.read_retained(path)
        self.assertEqual(len(rows), 1)


class TestGroupByOption(unittest.TestCase):
    def test_the_default_is_source_and_density(self):
        """Rule 16.2: density was the axis measured to move the outcome."""
        self.assertEqual(classifier.DEFAULT_GROUP_BY, ("source", "density"))
        self.assertEqual(classifier.parse_group_by("source,density"), ["source", "density"])

    def test_every_groupable_axis_is_accepted(self):
        for axis in classifier.GROUPABLE:
            with self.subTest(axis=axis):
                self.assertEqual(classifier.parse_group_by(axis), [axis])

    def test_an_axis_outside_the_three_is_refused(self):
        for axis in ("engine", "ticks", "survivors", "class"):
            with self.subTest(axis=axis):
                with self.assertRaises(classifier.InputError):
                    classifier.parse_group_by(axis)

    def test_a_repeated_axis_is_refused(self):
        with self.assertRaises(classifier.InputError):
            classifier.parse_group_by("source,source")

    def test_an_empty_axis_is_refused(self):
        for raw in ("", "source,", ",density", "source,,density"):
            with self.subTest(raw=raw):
                with self.assertRaises(classifier.InputError):
                    classifier.parse_group_by(raw)

    def test_a_refused_group_by_exits_2_and_writes_no_report(self):
        with tempfile.TemporaryDirectory() as directory:
            path = write_retained(directory, "r.jsonl", [sweep_row(), cell_row(), batch_row(1, 1)])
            status, out, err = quiet_main([path, "--group-by", "engine"])
        self.assertEqual(status, 2)
        self.assertEqual(out, "")
        self.assertIn("--group-by does not accept", err)

    def test_an_abbreviated_option_is_unknown(self):
        """`allow_abbrev=False`, matching the driver's parser."""
        with self.assertRaises(SystemExit):
            classifier.build_parser().parse_args(["r.jsonl", "--group"])


# ==================================================================================================
# Rules 16.3, 17.7 and 18.2: output
# ==================================================================================================


class TestOutput(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.path = write_retained(
            self.directory.name,
            "retained.jsonl",
            [sweep_row(sources=("social", "baseline"), seeds=(0, 1), cells=4)]
            + [
                cell_row(source="social", seed=0, survivors=11, deaths=1),
                cell_row(source="social", seed=1, survivors=6, deaths=6),
                cell_row(source="baseline", seed=0, survivors=0, deaths=12),
                cell_row(source="baseline", seed=1, survivors=0, deaths=12),
            ]
            + [batch_row(4, 4)],
        )

    def test_two_runs_over_the_same_input_are_byte_identical(self):
        """Rule 18.2."""
        first = quiet_main([self.path])
        second = quiet_main([self.path])
        self.assertEqual(first[1], second[1])
        self.assertEqual(first[0], 0)

    def test_json_is_the_default_format(self):
        status, out, _ = quiet_main([self.path])
        self.assertEqual(status, 0)
        report = json.loads(out)
        self.assertEqual(report["report"], "distribution")
        self.assertEqual(report["format"], classifier.REPORT_FORMAT)

    def test_markdown_is_a_rendering_and_states_the_same_counts(self):
        """Rule 16.3: regenerable, and not the authority for any figure."""
        _, rendered, _ = quiet_main([self.path, "--format", "markdown"])
        self.assertTrue(rendered.startswith("# Outcome distribution"))
        self.assertIn("authority for every figure", rendered)
        self.assertIn("| social | 0.75 |", rendered)
        self.assertIn("| baseline | 0.75 |", rendered)

    def test_the_rendering_states_no_conclusion_expectation_or_passing_shape(self):
        """Rule 17.7: counts, and nothing that reads as a verdict on them."""
        _, rendered, _ = quiet_main([self.path, "--format", "markdown"])
        lowered = rendered.lower()
        for forbidden in (
            "expected",
            "unexpected",
            "verdict",
            "conclusion",
            "acceptable",
            "unacceptable",
            "healthy",
            "unhealthy",
            "too many",
            "too few",
            "should be",
            "target",
            "regression",
            "improvement",
            "worse",
            "better",
        ):
            self.assertNotIn(forbidden, lowered, f"the rendering states {forbidden!r}")

    def test_an_incomplete_sweep_is_marked_incomplete_in_the_rendering(self):
        """Rule 17.4 and `REQ-MOK-083`: it cannot be read as a distribution over the stated sweep."""
        path = write_retained(
            self.directory.name,
            "partial.jsonl",
            [sweep_row(seeds=(0, 1, 2), cells=3)]
            + [cell_row(seed=0), cell_row(seed=1)]
            + [
                batch_row(
                    3,
                    2,
                    [{"source": "social", "density": "0.75", "seed": 2, "stage": "read", "reason": "x"}],
                )
            ],
        )
        _, rendered, _ = quiet_main([path, "--format", "markdown"])
        self.assertIn("**This sweep is incomplete.**", rendered)
        self.assertIn("complete: no", rendered)
        self.assertIn("- missing: 1", rendered)

    def test_a_complete_sweep_says_so_rather_than_staying_silent(self):
        _, rendered, _ = quiet_main([self.path, "--format", "markdown"])
        self.assertIn("complete: yes", rendered)
        self.assertNotIn("**This sweep is incomplete.**", rendered)

    def test_defaulted_axes_are_named_in_the_rendering(self):
        """Rule 6.5's `defaulted_axes` reaches the reader, who need not know the default sweep."""
        path = write_retained(
            self.directory.name,
            "defaulted.jsonl",
            [sweep_row(defaulted=("densities", "ticks")), cell_row(), batch_row(1, 1)],
        )
        _, rendered, _ = quiet_main([path, "--format", "markdown"])
        self.assertIn("- defaulted axes: densities, ticks", rendered)
        _, plain, _ = quiet_main([self.path, "--format", "markdown"])
        self.assertIn("- defaulted axes: none", plain)

    def test_seeds_are_rendered_as_the_ranges_the_driver_accepts(self):
        self.assertEqual(classifier._render_seeds(list(range(20))), "0-19")
        self.assertEqual(classifier._render_seeds([0, 1, 2, 5, 7, 8]), "0-2,5,7-8")
        self.assertEqual(classifier._render_seeds([4]), "4")
        self.assertEqual(classifier._render_seeds([]), "none")

    def test_the_rendering_names_no_stream_digest(self):
        """A digest belongs to the retained row that binds it, not to a rendering of a distribution."""
        _, rendered, _ = quiet_main([self.path, "--format", "markdown"])
        self.assertNotIn("stream_sha256", rendered)


# ==================================================================================================
# Rule 19: division of labour
# ==================================================================================================


class TestDivisionOfLabour(unittest.TestCase):
    def test_the_classifier_imports_only_the_standard_library(self):
        """Rule 1.3."""
        self.assertEqual(
            [name for name in imported_modules(classifier.__file__) if name not in STANDARD_LIBRARY],
            [],
        )

    def test_the_classifier_does_not_import_the_batch_driver(self):
        """Rule 1.5 and `REQ-MOK-082`: it works in a checkout with no engine binary present."""
        self.assertNotIn("run_simulation_batch", imported_modules(classifier.__file__))

    def test_the_classifier_executes_no_process(self):
        """Rule 19.2: the classifier reads retained output and nothing else."""
        with open(classifier.__file__, "r", encoding="utf-8") as handle:
            source = handle.read()
        for forbidden in ("subprocess", "os.system", "popen", "--binary", "--events-path"):
            self.assertNotIn(forbidden, source)

    def test_no_network_credential_or_live_option_appears(self):
        """Rule 19.4: neither instrument can reach a provider."""
        with open(classifier.__file__, "r", encoding="utf-8") as handle:
            source = handle.read()
        for forbidden in ("urllib", "socket", "http.client", "os.environ", "--live", "connector"):
            self.assertNotIn(forbidden, source)

    def test_every_threshold_lives_in_this_pair_of_files_and_nowhere_retained(self):
        """Rule 19.3, and `REQ-MOK-081`. The clause bodies are the only place a threshold is written."""
        with open(classifier.__file__, "r", encoding="utf-8") as handle:
            source = handle.read()
        self.assertIn("row[\"deaths\"] * 2 >= row[\"roster\"]", source)
        self.assertIn("row[\"survivors\"] == 0", source)


# `VER-MOK-019` case T5: checked against the running interpreter's own module set rather than against
# a hand-kept list, so a list that drifts cannot make the check pass.
STANDARD_LIBRARY = set(sys.stdlib_module_names) | {"__future__"}


def imported_modules(path):
    """Every top-level module name a file imports, read from its import graph rather than its prose."""
    import ast

    with open(path, "r", encoding="utf-8") as handle:
        tree = ast.parse(handle.read())
    names = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            names.extend(alias.name.split(".")[0] for alias in node.names)
        elif isinstance(node, ast.ImportFrom) and node.level == 0:
            names.append((node.module or "").split(".")[0])
    return [name for name in names if name]


if __name__ == "__main__":
    unittest.main(verbosity=2)
