#!/usr/bin/env python3
"""Tests for `scripts/run_simulation_batch.py`, covering `SPEC-MOK-008` rule 20.1's enumerated cases.

Rule 20.2: **these tests execute no real engine run.** They build fixture streams and replace the
child-process call through the `runner` seam, which keeps them fast and keeps the suite independent of
a built binary. `VER-MOK-019` case B1 is the end-to-end case against the real engine and is executed
under the work order rather than here, because a fixture corpus can encode a shape the engine never
emits.

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
import threading
import time
import unittest
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import run_simulation_batch as batch  # noqa: E402


# ==================================================================================================
# Fixture streams
# ==================================================================================================


def event(kind, result=None, tick=1, subject="M01"):
    return {"record": "event", "tick": tick, "subject": subject, "event": kind, "result": result or {}}


def build_stream(
    events=(),
    roster=12,
    deaths=0,
    ticks=10,
    engine="0.1.0",
    seed=0,
    policy="social",
    density="0.75",
    run_overrides=None,
    drop_run_keys=(),
    populations=(6, 6),
):
    """Build a stream whose run record agrees with its events on all seven rule 10.1 equalities.

    Every figure the cross-checks read is derived from the events, so a fixture is consistent by
    construction and a test that wants a disagreement states exactly one override.
    """
    initialized = [
        event("agent_initialized", {"name": f"N{index:02d}"}, subject=f"M{index:02d}")
        for index in range(1, roster + 1)
    ]
    body = list(initialized) + list(events)
    counted = Counter(record["event"] for record in body)

    agents = []
    for index in range(1, roster + 1):
        died_at = 5 if index <= deaths else None
        agents.append(
            {"id": f"M{index:02d}", "name": f"N{index:02d}", "territory": "A", "died_at": died_at}
        )

    run = {
        "record": "run",
        "reason": "tick_limit",
        "ticks": ticks,
        "survivors": roster - deaths,
        "deaths": deaths,
        "crossings": counted["territory_crossed"],
        # The three classes sum to the `food_consumed` count, which is the equality rule 10.1 checks.
        "consumed": {"low": counted["food_consumed"], "medium": 0, "high": 0},
        "regenerated": counted["food_regenerated"],
        "regeneration_skipped": {"depleted": 0, "capacity": counted["food_regeneration_skipped"]},
        "final": {
            "territories": {
                "A": {"population": populations[0], "low": 10, "medium": 11, "high": 12},
                "B": {"population": populations[1], "low": 13, "medium": 14, "high": 15},
            }
        },
        "agents": agents,
    }
    if run_overrides:
        for key, value in run_overrides.items():
            run[key] = value
    for key in drop_run_keys:
        run.pop(key, None)

    header = {
        "record": "header",
        "schema": 3,
        "engine": engine,
        "config": {
            "seed": seed,
            "ticks": ticks,
            "policy": policy,
            "density": density,
            "trace_actions": False,
        },
    }

    records = [header] + body + [run]
    text = "".join(json.dumps(record, separators=(",", ":")) + "\n" for record in records)
    return text.encode("utf-8")


def quiet_main(argv, runner=None):
    """Call the driver's entry point with its progress reporting captured.

    The driver reports progress on standard output by design. Every test below asserts on its return
    value and on the file it wrote, so the progress lines are captured to keep the test log readable.
    """
    with contextlib.redirect_stdout(io.StringIO()):
        return batch.main(argv, runner=runner)


def runner_writing(raw, status=0, stderr=""):
    """A `runner` that writes a fixture stream where the engine would have written one."""

    def run(argv):
        path = argv[argv.index("--events-path") + 1]
        if raw is not None:
            with open(path, "wb") as handle:
                handle.write(raw)
        return status, stderr

    return run


CONFLICT_EVENTS = (
    event("attack_resolved", {"target": "M02", "damage": 29, "target_died": "no"}),
    event("attack_resolved", {"target": "M03", "damage": 0, "target_died": "no"}),
    event("attack_resolved", {"target": "M04", "damage": 44, "target_died": "yes"}),
    event("threat_resolved", {"target": "M02", "increase": 7}),
    event("threat_resolved", {"target": "M03", "increase": 0}),
    event("threat_resolved", {"target": "M04", "increase": 0}),
    event("surrender_resolved", {"recipient": "M02", "transferred": 9}),
    event("surrender_resolved", {"recipient": "M03", "transferred": 0}),
)


# ==================================================================================================
# Rule 3 and rule 4: the default sweep, enumeration and ordering
# ==================================================================================================


class TestDefaultSweep(unittest.TestCase):
    def test_declared_default_is_four_by_five_by_twenty_at_one_thousand(self):
        """Rule 3.1 and 3.2. The default sweep is a value in the specification, not in help text."""
        self.assertEqual(batch.DEFAULT_SOURCES, ("baseline", "reference", "individual", "social"))
        self.assertEqual(batch.DEFAULT_DENSITIES, ("0.10", "0.25", "0.50", "0.75", "1.00"))
        self.assertEqual(batch.DEFAULT_SEEDS, tuple(range(20)))
        self.assertEqual(batch.DEFAULT_TICKS, 1000)
        cells = batch.build_cells(
            batch.DEFAULT_SOURCES, batch.DEFAULT_DENSITIES, batch.DEFAULT_SEEDS
        )
        self.assertEqual(len(cells), 400)

    def test_no_axis_option_selects_the_default_sweep(self):
        """Rule 2.6."""
        args = batch.build_parser().parse_args(["--out", "out.jsonl"])
        sources, densities, seeds, ticks, defaulted = batch.resolve_sweep(args)
        self.assertEqual(tuple(sources), batch.DEFAULT_SOURCES)
        self.assertEqual(tuple(densities), batch.DEFAULT_DENSITIES)
        self.assertEqual(tuple(seeds), batch.DEFAULT_SEEDS)
        self.assertEqual(ticks, 1000)
        self.assertEqual(defaulted, ["sources", "densities", "seeds", "ticks"])

    def test_partially_defaulted_sweep_names_only_the_defaulted_axes(self):
        """Rule 2.6 and rule 6.5: a partially defaulted sweep is legible without knowing the default."""
        args = batch.build_parser().parse_args(
            ["--out", "out.jsonl", "--seeds", "0-2", "--sources", "social"]
        )
        sources, densities, seeds, ticks, defaulted = batch.resolve_sweep(args)
        self.assertEqual(sources, ["social"])
        self.assertEqual(seeds, [0, 1, 2])
        self.assertEqual(tuple(densities), batch.DEFAULT_DENSITIES)
        self.assertEqual(ticks, 1000)
        self.assertEqual(defaulted, ["densities", "ticks"])

    def test_cells_are_ordered_source_then_density_then_seed_ascending(self):
        """Rule 4.3. The seed axis is ascending whatever order it was given in."""
        cells = batch.build_cells(["social", "baseline"], ["0.75", "0.10"], [2, 0, 1])
        self.assertEqual(
            cells,
            [
                ("social", "0.75", 0),
                ("social", "0.75", 1),
                ("social", "0.75", 2),
                ("social", "0.10", 0),
                ("social", "0.10", 1),
                ("social", "0.10", 2),
                ("baseline", "0.75", 0),
                ("baseline", "0.75", 1),
                ("baseline", "0.75", 2),
                ("baseline", "0.10", 0),
                ("baseline", "0.10", 1),
                ("baseline", "0.10", 2),
            ],
        )

    def test_seed_ranges_are_inclusive_and_mix_with_integers(self):
        """Rule 2.4."""
        self.assertEqual(batch.parse_seeds("0-3"), [0, 1, 2, 3])
        self.assertEqual(batch.parse_seeds("5,7-9,11"), [5, 7, 8, 9, 11])
        self.assertEqual(batch.parse_seeds("4-4"), [4])


# ==================================================================================================
# Rule 4.4: the child's argument vector
# ==================================================================================================


class TestBinaryResolution(unittest.TestCase):
    def test_an_explicit_path_is_normalised_to_the_platforms_separator(self):
        """A forward-slash relative path on Windows passes `isfile` and then fails to launch.

        `VER-MOK-019` case B1 found this against the real engine; the `runner` seam cannot, because it
        never reaches the process boundary. The case is kept here so a regression is caught by the
        suite rather than by a 400-cell batch reporting 400 failures.

        The relative path is built inside a temporary tree rather than from this file's own location.
        Taken from `os.path.relpath(__file__)`, it carries a separator only when the suite is started
        from somewhere other than `scripts/`: run from that directory the path is a bare file name,
        the forward-slash substitution does nothing, and the case passes whether the normalisation is
        there or not. Verification measured that -- the assertion held against a copy with the
        normalisation removed -- so the path is now constructed to carry a separator always.
        """
        relative = "target/release/Mokiterions.exe"
        with tempfile.TemporaryDirectory() as directory:
            os.makedirs(os.path.join(directory, "target", "release"))
            with open(os.path.join(directory, *relative.split("/")), "wb"):
                pass
            # A relative path is read against the working directory, so the case has to have one.
            # `os.chdir` rather than `contextlib.chdir`, which would put a Python 3.11 floor under a
            # suite whose specification states no minimum version.
            origin = os.getcwd()
            os.chdir(directory)
            try:
                path, profile = batch.resolve_binary(relative)
            finally:
                os.chdir(origin)
        self.assertEqual(path, os.path.normpath(relative))
        self.assertEqual(profile, "release")
        if os.name == "nt":
            self.assertNotIn("/", path)

    def test_a_named_binary_that_is_absent_is_a_driver_error(self):
        """Rule 2.3, and rule 14.1's status 1 rather than a per-cell failure 400 times over."""
        with self.assertRaises(batch.DriverError):
            batch.resolve_binary(os.path.join(tempfile.gettempdir(), "no-such-engine-xyz"))

    def test_the_profile_is_read_from_the_containing_directory(self):
        """Rule 6.3: `release`, `debug`, or `other` for a path that is neither."""
        with tempfile.TemporaryDirectory() as directory:
            for name, expected in (("release", "release"), ("debug", "debug"), ("staging", "other")):
                os.makedirs(os.path.join(directory, name), exist_ok=True)
                candidate = os.path.join(directory, name, "Mokiterions.exe")
                with open(candidate, "wb"):
                    pass
                with self.subTest(directory=name):
                    self.assertEqual(batch.resolve_binary(candidate)[1], expected)


class TestChildArguments(unittest.TestCase):
    def test_exactly_rule_4_4s_arguments_and_no_others(self):
        """Rule 4.4: no tracing option, no transcript option, no live option, no connector option."""
        seen = []

        def spy(argv):
            seen.append(list(argv))
            path = argv[argv.index("--events-path") + 1]
            with open(path, "wb") as handle:
                handle.write(build_stream())
            return 0, ""

        with tempfile.TemporaryDirectory() as directory:
            stream = os.path.join(directory, "s.jsonl")
            row, fault = batch.execute_cell(
                ("social", "0.75", 3), 10, "ENGINE", stream, runner=spy
            )
        self.assertIsNone(fault)
        self.assertIsNotNone(row)
        self.assertEqual(
            seen[0],
            [
                "ENGINE",
                "--seed",
                "3",
                "--ticks",
                "10",
                "--policy",
                "social",
                "--density",
                "0.75",
                "--events-path",
                stream,
            ],
        )
        for forbidden in ("--trace-actions", "--transcript-path", "--live", "--connector-path"):
            self.assertNotIn(forbidden, seen[0])


# ==================================================================================================
# Rule 9: the behavioural counters and their effectiveness tests
# ==================================================================================================


class TestBehaviouralCounters(unittest.TestCase):
    def scan(self, events):
        return batch.scan_stream(build_stream(events))

    def test_attempted_counts_every_occurrence(self):
        """Rule 9.1."""
        scan = self.scan(CONFLICT_EVENTS)
        self.assertEqual(
            scan["attempted"],
            {"attack_resolved": 3, "threat_resolved": 3, "surrender_resolved": 2},
        )

    def test_each_effectiveness_test_reads_its_own_field(self):
        """Rule 9.2: damage, increase and transferred, each greater than zero."""
        scan = self.scan(CONFLICT_EVENTS)
        self.assertEqual(
            scan["effective"],
            {"attack_resolved": 2, "threat_resolved": 1, "surrender_resolved": 1},
        )

    def test_a_wholly_inert_mechanism_reads_zero_effective_against_nonzero_attempted(self):
        """Rule 9.6's measured case: threats that fired and changed nothing.

        This is the inversion the split exists for. On `attempted` this fixture is the most active
        threatener imaginable; on `effective` it is wholly inert.
        """
        inert = tuple(
            event("threat_resolved", {"target": "M02", "increase": 0}) for _ in range(106)
        )
        scan = self.scan(inert)
        self.assertEqual(scan["attempted"]["threat_resolved"], 106)
        self.assertEqual(scan["effective"]["threat_resolved"], 0)

    def test_lethal_attacks_is_a_third_figure_not_a_narrowing(self):
        """Rule 9.3: a lethal attack is also an effective one, and the two counts coexist."""
        scan = self.scan(CONFLICT_EVENTS)
        self.assertEqual(scan["lethal_attacks"], 1)
        self.assertEqual(scan["effective"]["attack_resolved"], 2)

    def test_a_zero_count_is_stated_rather_than_omitted(self):
        """Rule 9.4: a mechanism that never fired is distinguishable from one that was not counted."""
        row = self.row(())
        for kind in batch.ATTEMPTED_KINDS:
            self.assertEqual(row["attempted"][kind], 0)
            self.assertEqual(row["effective"][kind], 0)

    def test_no_retreat_field_is_synthesised(self):
        """Rule 9.7: the vocabulary has no `retreat_resolved`, so no field is invented for it."""
        row = self.row(CONFLICT_EVENTS)
        flat = json.dumps(row)
        self.assertNotIn("retreat", flat)
        self.assertNotIn("retreat_resolved", batch.ATTEMPTED_KINDS)

    def test_the_driver_states_no_difference_of_two_counters(self):
        """Rule 9.5: a consumer may subtract; the driver states no such difference."""
        row = self.row(CONFLICT_EVENTS)
        self.assertNotIn("ineffective", row)
        self.assertNotIn("inert", row)

    def row(self, events):
        scan = batch.scan_stream(build_stream(events))
        passed, failures = batch.crosscheck(scan)
        self.assertEqual(failures, [])
        return batch.derive_row(("social", "0.75", 0), scan, passed)


# ==================================================================================================
# Rule 8: the cell record
# ==================================================================================================


class TestCellRecord(unittest.TestCase):
    def row(self, **kwargs):
        scan = batch.scan_stream(build_stream(**kwargs))
        passed, failures = batch.crosscheck(scan)
        self.assertEqual(failures, [])
        return batch.derive_row(("social", "0.75", 4), scan, passed)

    def test_roster_is_retained_and_the_agents_array_is_not(self):
        """Rule 8.5: a deliberate loss, recorded as one and recoverable by re-running the cell."""
        row = self.row(roster=12)
        self.assertEqual(row["roster"], 12)
        self.assertNotIn("agents", row)
        self.assertNotIn("Zug", json.dumps(row))

    def test_run_ticks_is_distinct_from_the_requested_horizon(self):
        """Rule 8.4: conflating the two would make every early termination unreadable.

        The fixture is an extinction at tick 137 of a requested 1000, which is the shape the run
        record's `reason` distinguishes and the shape that makes the two fields differ.
        """
        scan = batch.scan_stream(
            build_stream(ticks=1000, run_overrides={"ticks": 137, "reason": "extinct"})
        )
        passed, _ = batch.crosscheck(scan)
        row = batch.derive_row(("baseline", "0.10", 0), scan, passed)
        self.assertEqual(row["ticks"], 1000)
        self.assertEqual(row["run_ticks"], 137)
        self.assertEqual(row["reason"], "extinct")

    def test_engine_comes_from_the_header_record(self):
        """Rule 8.3: not a version the driver inferred."""
        self.assertEqual(self.row(engine="9.9.9")["engine"], "9.9.9")

    def test_density_is_the_engines_own_string(self):
        """Rule 8.2: not a computed decimal, which is how rule 17.2 is satisfied on this axis."""
        row = self.row()
        self.assertIsInstance(row["density"], str)
        self.assertEqual(row["density"], "0.75")

    def test_no_class_label_verdict_or_threshold_appears(self):
        """Rule 8.6 and `SPEC-MOK-006` rule 8.7: a row carrying a label would not be revisable."""
        flat = json.dumps(self.row(events=CONFLICT_EVENTS))
        for forbidden in (
            "class",
            "label",
            "verdict",
            "severity",
            "threshold",
            "extinction",
            "famine",
            "collapse",
            "coexistence",
        ):
            self.assertNotIn(forbidden, flat)

    def test_no_float_appears_anywhere_in_a_row(self):
        """Rule 8.7 and rule 18.3."""
        self.assertEqual(floats_in(self.row(events=CONFLICT_EVENTS)), [])

    def test_no_path_host_pid_or_clock_appears(self):
        """Rule 11.6: a retained row carrying a duration would not be comparable between machines."""
        flat = json.dumps(self.row())
        for forbidden in ("path", "host", "pid", "elapsed", "duration", "started", "wall"):
            self.assertNotIn(forbidden, flat)

    def test_the_digest_is_over_the_streams_exact_bytes(self):
        """Rule 11.2 and 11.3: over the stream, never over the row."""
        raw = build_stream(events=CONFLICT_EVENTS)
        scan = batch.scan_stream(raw)
        self.assertEqual(scan["digest"], hashlib.sha256(raw).hexdigest())

    def test_scanning_from_disk_and_from_memory_agree_byte_for_byte(self):
        """The driver scans a line at a time so a long horizon is not held in memory whole.

        `VER-MOK-019`'s performance check states that property. Both paths run the same code, and the
        digest is the check that the incremental hash covers every byte including the terminators.
        """
        raw = build_stream(events=CONFLICT_EVENTS)
        with tempfile.TemporaryDirectory() as directory:
            path = os.path.join(directory, "stream.jsonl")
            with open(path, "wb") as handle:
                handle.write(raw)
            from_disk = batch.scan_file(path)
        from_memory = batch.scan_stream(raw)
        self.assertEqual(from_disk["digest"], hashlib.sha256(raw).hexdigest())
        self.assertEqual(from_disk, from_memory)

    def test_a_stream_with_no_trailing_newline_hashes_as_written(self):
        """Rule 11.2: the digest is over the bytes the engine wrote, not over a normalised form."""
        raw = build_stream()[:-1]
        self.assertFalse(raw.endswith(b"\n"))
        self.assertEqual(batch.scan_stream(raw)["digest"], hashlib.sha256(raw).hexdigest())

    def test_one_altered_byte_changes_the_digest(self):
        """`VER-MOK-019` case B9's second half."""
        raw = build_stream(events=CONFLICT_EVENTS)
        altered = raw.replace(b'"damage":29', b'"damage":28', 1)
        self.assertNotEqual(raw, altered)
        self.assertNotEqual(batch.scan_stream(raw)["digest"], batch.scan_stream(altered)["digest"])


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


# ==================================================================================================
# Rule 10: the cross-checks
# ==================================================================================================


class TestCrossChecks(unittest.TestCase):
    def test_a_consistent_stream_passes_all_seven(self):
        """Rule 10.2: all seven held in all 35 streams measured at c90edc9."""
        scan = batch.scan_stream(build_stream(events=CONFLICT_EVENTS))
        passed, failures = batch.crosscheck(scan)
        self.assertEqual(failures, [])
        self.assertEqual(passed, 7)

    def test_each_of_the_seven_equalities_has_a_failing_case(self):
        """Rule 10.1, one fixture per equality, each breaking exactly that one.

        `VER-MOK-019` case B17's reason: a batch over real runs cannot distinguish a working check from
        one that always returns true, because all seven hold on every real stream measured. A check
        with no failing case is not a verified check.
        """
        # Two equalities share the `agents` array, and two share `deaths`, so a fixture that breaks
        # exactly one of those must hold the other's terms constant. That is stated per case rather
        # than worked around, because a fixture breaking two checks would not distinguish them.
        cases = [
            ("territory_crossed", {"crossings": 99}, "crossings"),
            ("agent_died", {"deaths": 2, "survivors": 10}, "deaths"),
            ("food_consumed", {"consumed": {"low": 99, "medium": 0, "high": 0}}, "consumed sum"),
            ("food_regenerated", {"regenerated": 99}, "regenerated"),
            (
                "food_regeneration_skipped",
                {"regeneration_skipped": {"depleted": 3, "capacity": 96}},
                "regeneration_skipped sum",
            ),
            (
                "agent_initialized",
                {"agents": [{"id": f"M{n:02d}", "died_at": None} for n in range(5)], "survivors": 4},
                "agents length",
            ),
            ("survivors + deaths", {"survivors": 3}, "agents length"),
        ]
        events = (
            event("territory_crossed", {"to": "B"}),
            event("agent_died", {"cause": "starvation"}),
            event("food_consumed", {"food": "F1"}),
            event("food_regenerated", {"food": "F2"}),
            event("food_regeneration_skipped", {"reason": "capacity"}),
        )
        for name, override, label in cases:
            with self.subTest(check=name):
                scan = batch.scan_stream(
                    build_stream(events=events, deaths=1, run_overrides=override)
                )
                passed, failures = batch.crosscheck(scan)
                self.assertEqual(len(failures), 1, f"{name}: {failures}")
                self.assertEqual(passed, 6)
                self.assertTrue(failures[0].startswith(name), failures[0])
                self.assertIn(label, failures[0])

    def test_a_failed_check_produces_a_missing_cell_and_no_row(self):
        """Rule 10.3: the row is not retained with the disagreement recorded, and neither figure wins."""
        raw = build_stream(run_overrides={"crossings": 41})
        with tempfile.TemporaryDirectory() as directory:
            row, fault = batch.execute_cell(
                ("social", "0.75", 0),
                10,
                "ENGINE",
                os.path.join(directory, "s.jsonl"),
                runner=runner_writing(raw),
            )
        self.assertIsNone(row)
        self.assertEqual(fault["stage"], "crosscheck")
        self.assertIn("territory_crossed 0 != crossings 41", fault["reason"])

    def test_a_retained_row_always_states_seven_passed(self):
        """Rule 10.3's consequence: a row with a failed check is never retained, so the field is 7."""
        scan = batch.scan_stream(build_stream(events=CONFLICT_EVENTS))
        passed, _ = batch.crosscheck(scan)
        row = batch.derive_row(("social", "0.75", 0), scan, passed)
        self.assertEqual(row["crosschecks_passed"], 7)


# ==================================================================================================
# Rule 12.3: each of the six failure stages
# ==================================================================================================


class TestFailureStages(unittest.TestCase):
    def stage(self, cell=("social", "0.75", 0), **kwargs):
        with tempfile.TemporaryDirectory() as directory:
            row, fault = batch.execute_cell(
                cell, 10, "ENGINE", os.path.join(directory, "s.jsonl"), **kwargs
            )
        return row, fault

    def test_stage_refused(self):
        """Rule 12.3 `refused`, and rule 4.2's refusal repeated at the cell level.

        The sweep-level check refuses `llm` before any cell executes. This guard is the same refusal
        one layer down, so no path through `execute_cell` can reach a provider.
        """
        row, fault = self.stage(cell=("llm", "0.75", 0), runner=runner_writing(build_stream()))
        self.assertIsNone(row)
        self.assertEqual(fault["stage"], "refused")
        self.assertIn("REQ-MOK-072", fault["reason"])

    def test_stage_run(self):
        """Rule 12.3 `run`, with rule 13.2's verbatim engine message."""
        row, fault = self.stage(
            runner=runner_writing(None, status=2, stderr="configuration error: --density 0.00")
        )
        self.assertIsNone(row)
        self.assertEqual(fault["stage"], "run")
        self.assertEqual(fault["reason"], "engine exited 2: configuration error: --density 0.00")

    def test_stage_read_when_the_stream_is_absent(self):
        """Rule 12.3 `read`."""
        row, fault = self.stage(runner=runner_writing(None, status=0))
        self.assertIsNone(row)
        self.assertEqual(fault["stage"], "read")

    def test_stage_read_when_the_stream_lacks_a_run_record(self):
        """Rule 12.3 `read`: an incomplete stream is unreadable, not a row with holes."""
        raw = b'{"record":"header","schema":3,"engine":"0.1.0","config":{"ticks":10}}\n'
        row, fault = self.stage(runner=runner_writing(raw))
        self.assertIsNone(row)
        self.assertEqual(fault["stage"], "read")
        self.assertIn("run record", fault["reason"])

    def test_stage_crosscheck(self):
        """Rule 12.3 `crosscheck`, which rule 10.3 defines."""
        row, fault = self.stage(runner=runner_writing(build_stream(run_overrides={"deaths": 5})))
        self.assertIsNone(row)
        self.assertEqual(fault["stage"], "crosscheck")

    def test_stage_derive(self):
        """Rule 12.3 `derive`: the cross-checks agree but the row cannot be built."""
        row, fault = self.stage(runner=runner_writing(build_stream(drop_run_keys=("final",))))
        self.assertIsNone(row)
        self.assertEqual(fault["stage"], "derive")
        self.assertIn("could not derive", fault["reason"])

    def test_stage_not_attempted(self):
        """Rule 12.3 `not_attempted`: the batch stopped first, and rule 12.4 fills nothing in."""
        cells = batch.build_cells(["social"], ["0.75"], [0, 1, 2])
        calls = []

        def interrupting(argv):
            calls.append(argv)
            if len(calls) == 2:
                raise KeyboardInterrupt
            path = argv[argv.index("--events-path") + 1]
            with open(path, "wb") as handle:
                handle.write(build_stream())
            return 0, ""

        with tempfile.TemporaryDirectory() as directory:
            rows, missing, leftover = batch.run_batch(
                cells, 10, "ENGINE", directory, 1, [], report=lambda *_: None, runner=interrupting
            )
        self.assertEqual(len(rows), 1)
        stages = {(entry["seed"]): entry["stage"] for entry in missing}
        self.assertEqual(stages, {1: "not_attempted", 2: "not_attempted"})
        self.assertEqual(missing[0]["reason"], "interrupted by operator")

    def test_every_stage_in_the_closed_vocabulary_is_reachable(self):
        """Rule 12.3's vocabulary is closed, and rule 20.1 requires a case for each of the six."""
        self.assertEqual(
            batch.STAGES, ("refused", "run", "read", "crosscheck", "derive", "not_attempted")
        )


# ==================================================================================================
# Rule 12: the batch record and completeness
# ==================================================================================================


class TestBatchRecord(unittest.TestCase):
    def test_a_complete_batch_states_complete_true_explicitly(self):
        """Rule 12.2: a reader must not infer completeness from the absence of a warning."""
        record = batch.batch_record(2, [{"record": "cell"}, {"record": "cell"}], [], [])
        self.assertIs(record["complete"], True)
        self.assertEqual((record["requested"], record["retained"]), (2, 2))

    def test_an_incomplete_batch_names_every_missing_cell_with_a_stage_and_reason(self):
        """Rule 12.3, and rule 12.4: nothing is filled with a zero, a default or a neighbour."""
        missing = [
            {"source": "social", "density": "0.10", "seed": 7, "stage": "run", "reason": "engine exited 2: x"}
        ]
        record = batch.batch_record(2, [{"record": "cell"}], missing, [])
        self.assertIs(record["complete"], False)
        self.assertEqual(record["missing"], missing)

    def test_one_cells_failure_does_not_discard_another_cells_row(self):
        """Rule 12.5, and rule 13.3: a cell failure does not stop the batch."""
        cells = batch.build_cells(["social"], ["0.75"], [0, 1, 2])

        def flaky(argv):
            seed = argv[argv.index("--seed") + 1]
            if seed == "1":
                return 2, "configuration error: refused"
            path = argv[argv.index("--events-path") + 1]
            with open(path, "wb") as handle:
                handle.write(build_stream(seed=int(seed)))
            return 0, ""

        with tempfile.TemporaryDirectory() as directory:
            rows, missing, leftover = batch.run_batch(
                cells, 10, "ENGINE", directory, 1, [], report=lambda *_: None, runner=flaky
            )
        self.assertEqual([row["seed"] for row in rows], [0, 2])
        self.assertEqual([entry["seed"] for entry in missing], [1])
        self.assertEqual(missing[0]["stage"], "run")

    def test_a_batch_where_every_cell_failed_writes_no_cell_record(self):
        """Rule 12.6."""
        cells = batch.build_cells(["social"], ["0.75"], [0, 1])
        with tempfile.TemporaryDirectory() as directory:
            rows, missing, _ = batch.run_batch(
                cells,
                10,
                "ENGINE",
                directory,
                1,
                [],
                report=lambda *_: None,
                runner=runner_writing(None, status=2, stderr="refused"),
            )
        record = batch.batch_record(len(cells), rows, missing, [])
        self.assertEqual(rows, [])
        self.assertIs(record["complete"], False)
        self.assertEqual(len(record["missing"]), 2)

    def test_rows_are_written_in_rule_4_3s_order_whatever_order_they_completed_in(self):
        """Rule 4.3 and rule 18.1, which is what makes `--jobs` greater than 1 safe."""
        cells = batch.build_cells(["social", "baseline"], ["0.75"], [0, 1, 2])
        with tempfile.TemporaryDirectory() as directory:
            rows, missing, _ = batch.run_batch(
                cells, 10, "ENGINE", directory, 4, [], report=lambda *_: None,
                runner=runner_writing(build_stream()),
            )
        self.assertEqual(
            [(row["source"], row["seed"]) for row in rows],
            [("social", 0), ("social", 1), ("social", 2),
             ("baseline", 0), ("baseline", 1), ("baseline", 2)],
        )
        self.assertEqual(missing, [])


# ==================================================================================================
# Rule 15.1: every usage and configuration error
# ==================================================================================================


class TestUsageErrors(unittest.TestCase):
    """Rule 15.1's list in full. Every one is refused before anything executes, with status 2."""

    def refuse(self, argv):
        with tempfile.TemporaryDirectory() as directory:
            out = os.path.join(directory, "out.jsonl")
            status = quiet_main(["--out", out] + argv, runner=self.fail_if_called)
            self.assertFalse(os.path.exists(out), "rule 14.1: status 2 writes nothing")
        return status

    def fail_if_called(self, argv):
        raise AssertionError("rule 14.1: status 2 means nothing executed")

    def test_unknown_option(self):
        with self.assertRaises(SystemExit) as caught:
            batch.build_parser().parse_args(["--out", "o", "--sweep-everything"])
        self.assertEqual(caught.exception.code, 2)

    def test_an_abbreviated_option_is_unknown(self):
        """`allow_abbrev=False`: an abbreviation is not silently accepted as the option it prefixes."""
        with self.assertRaises(SystemExit):
            batch.build_parser().parse_args(["--out", "o", "--source", "social"])

    def test_malformed_seed_specification(self):
        for spec in ("0-", "-3", "1,,2", "x", "3-1", "1.5"):
            with self.subTest(seeds=spec):
                self.assertEqual(self.refuse(["--seeds", spec]), 2)

    def test_empty_axis(self):
        with self.assertRaises(batch.UsageError):
            batch.check_axis([], "sources")
        self.assertEqual(self.refuse(["--sources", ""]), 2)

    def test_duplicate_value_on_any_axis(self):
        """Rule 4.1: refused rather than collapsed, because the two differ in every distribution."""
        self.assertEqual(self.refuse(["--sources", "social,social"]), 2)
        self.assertEqual(self.refuse(["--densities", "0.75,0.75"]), 2)
        self.assertEqual(self.refuse(["--seeds", "1,2,1"]), 2)
        self.assertEqual(self.refuse(["--seeds", "0-3,2"]), 2)

    def test_a_source_the_engine_does_not_accept(self):
        self.assertEqual(self.refuse(["--sources", "cooperative"]), 2)

    def test_a_density_the_engine_does_not_accept(self):
        for density in ("0.755", "abc", "101", "0", "-1"):
            with self.subTest(density=density):
                self.assertEqual(self.refuse(["--densities", density]), 2)

    def test_llm_among_the_sources(self):
        """Rule 4.2: refused before any cell executes, and the refusal names REQ-MOK-072."""
        self.assertEqual(self.refuse(["--sources", "social,llm"]), 2)
        with self.assertRaises(batch.UsageError) as caught:
            batch.validate_sources(["llm"])
        self.assertIn("REQ-MOK-072", str(caught.exception))

    def test_non_positive_tick_horizon(self):
        self.assertEqual(self.refuse(["--ticks", "0"]), 2)
        self.assertEqual(self.refuse(["--ticks", "-5"]), 2)

    def test_out_under_docs_engineering(self):
        """Rule 15.1: no batch can write into governance artifacts."""
        for path in (
            "docs/engineering/simulation/evidence/x.jsonl",
            os.path.join("docs", "engineering", "x.jsonl"),
        ):
            with self.subTest(path=path):
                with self.assertRaises(batch.UsageError):
                    batch.validate_out(path)
        self.assertEqual(
            quiet_main(
                ["--out", "docs/engineering/x.jsonl", "--sources", "social"],
                runner=self.fail_if_called,
            ),
            2,
        )

    def test_a_docs_path_that_is_not_docs_engineering_is_allowed(self):
        """The prohibition is on governance artifacts, not on the word `docs`."""
        batch.validate_out("docs/notes/x.jsonl")
        batch.validate_out("engineering/x.jsonl")

    def test_jobs_below_one(self):
        self.assertEqual(self.refuse(["--jobs", "0"]), 2)


# ==================================================================================================
# Rule 14.1: each exit status
# ==================================================================================================


class TestExitStatuses(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.out = os.path.join(self.directory.name, "out.jsonl")
        # An existing file stands in for the binary. The runner seam means it is never executed, so
        # rule 20.2 holds: this test runs no engine.
        self.binary = os.path.abspath(__file__)

    def run_main(self, extra, runner):
        return quiet_main(
            [
                "--out",
                self.out,
                "--binary",
                self.binary,
                "--sources",
                "social",
                "--densities",
                "0.75",
                "--ticks",
                "10",
                "--stream-dir",
                self.directory.name,
            ]
            + extra,
            runner=runner,
        )

    def test_status_0_when_every_cell_produced_a_row(self):
        status = self.run_main(["--seeds", "0-1"], runner_writing(build_stream()))
        self.assertEqual(status, 0)
        records = read_records(self.out)
        self.assertIs(records[-1]["complete"], True)
        self.assertEqual(records[-1]["retained"], 2)

    def test_status_3_when_a_cell_is_missing_and_the_output_is_written(self):
        """Rule 14.2: 3 is distinct from 0 so an incomplete sweep cannot be consumed by ignoring output."""

        def flaky(argv):
            if argv[argv.index("--seed") + 1] == "1":
                return 2, "configuration error: refused"
            path = argv[argv.index("--events-path") + 1]
            with open(path, "wb") as handle:
                handle.write(build_stream())
            return 0, ""

        status = self.run_main(["--seeds", "0-1"], flaky)
        self.assertEqual(status, 3)
        records = read_records(self.out)
        self.assertIs(records[-1]["complete"], False)
        self.assertEqual(records[-1]["retained"], 1)

    def test_status_2_for_a_usage_error_writes_nothing(self):
        status = quiet_main(
            ["--out", self.out, "--sources", "llm"],
            runner=lambda argv: (_ for _ in ()).throw(AssertionError("executed")),
        )
        self.assertEqual(status, 2)
        self.assertFalse(os.path.exists(self.out))

    def test_status_1_when_the_binary_is_not_found(self):
        status = quiet_main(
            [
                "--out",
                self.out,
                "--binary",
                os.path.join(self.directory.name, "no-such-engine"),
                "--sources",
                "social",
                "--densities",
                "0.75",
                "--seeds",
                "0",
                "--ticks",
                "10",
            ],
            runner=runner_writing(build_stream()),
        )
        self.assertEqual(status, 1)

    def test_status_1_when_the_retained_output_is_unwritable(self):
        """Rule 12.7: the driver retains nothing rather than rows whose completeness is unrecorded."""
        status = quiet_main(
            [
                "--out",
                os.path.join(self.directory.name, "no-such-directory", "out.jsonl"),
                "--binary",
                self.binary,
                "--sources",
                "social",
                "--densities",
                "0.75",
                "--seeds",
                "0",
                "--ticks",
                "10",
            ],
            runner=runner_writing(build_stream()),
        )
        self.assertEqual(status, 1)


def read_records(path):
    with open(path, "r", encoding="utf-8") as handle:
        return [json.loads(line) for line in handle if line.strip()]


# ==================================================================================================
# Rules 6, 7 and 18: the retained file, the temporary stream, determinism
# ==================================================================================================


class TestRetainedFileAndStream(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.binary = os.path.abspath(__file__)

    def sweep(self, name, extra=()):
        out = os.path.join(self.directory.name, name)
        status = quiet_main(
            ["--out", out, "--binary", self.binary, "--sources", "social,baseline",
             "--densities", "0.75", "--seeds", "0-1", "--ticks", "10",
             "--stream-dir", self.directory.name] + list(extra),
            runner=runner_writing(build_stream(events=CONFLICT_EVENTS)),
        )
        self.assertEqual(status, 0)
        return out

    def test_three_record_kinds_in_rule_6_2s_order(self):
        records = read_records(self.sweep("a.jsonl"))
        self.assertEqual(records[0]["record"], "sweep")
        self.assertEqual(records[-1]["record"], "batch")
        self.assertEqual({r["record"] for r in records[1:-1]}, {"cell"})
        self.assertEqual(sorted({r["record"] for r in records}), ["batch", "cell", "sweep"])

    def test_the_record_names_are_not_the_engine_streams(self):
        """Rule 6.2: so no reader confuses this file with an engine stream."""
        records = read_records(self.sweep("b.jsonl"))
        engine_kinds = {"header", "event", "metrics", "run"}
        self.assertEqual({r["record"] for r in records} & engine_kinds, set())

    def test_line_endings_are_newline_on_every_platform(self):
        """Rule 6.1: fixed because the file is compared for equality and hashed."""
        with open(self.sweep("c.jsonl"), "rb") as handle:
            raw = handle.read()
        self.assertNotIn(b"\r", raw)
        self.assertTrue(raw.endswith(b"\n"))

    def test_two_batches_over_the_same_sweep_are_byte_identical(self):
        """Rule 18.1."""
        with open(self.sweep("d1.jsonl"), "rb") as handle:
            first = handle.read()
        with open(self.sweep("d2.jsonl"), "rb") as handle:
            second = handle.read()
        self.assertEqual(first, second)

    def test_concurrent_and_serial_batches_are_byte_identical(self):
        """Rule 2.8: `--jobs` changes no row, because a row is a function of its cell."""
        with open(self.sweep("e1.jsonl"), "rb") as handle:
            serial = handle.read()
        with open(self.sweep("e2.jsonl", ["--jobs", "4"]), "rb") as handle:
            concurrent_out = handle.read()
        self.assertEqual(serial, concurrent_out)

    def test_no_float_appears_anywhere_in_the_retained_output(self):
        """Rule 18.3."""
        for record in read_records(self.sweep("f.jsonl")):
            self.assertEqual(floats_in(record), [], record)

    def test_the_stream_is_removed_after_being_read(self):
        """Rule 7.3."""
        self.sweep("g.jsonl")
        leftovers = [
            name for name in os.listdir(self.directory.name) if name.startswith("batch-stream-")
        ]
        self.assertEqual(leftovers, [])

    def test_the_stream_is_removed_when_a_cell_is_interrupted(self):
        """Rule 7.3 where it matters: an interrupt arrives inside a cell, not between two.

        Rule 12.3's `not_attempted` stage exists because an operator interrupts a batch, and rule
        7.3's removal is what makes rule 7.2's peak-disk consequence true. The two meet in the case
        this covers: the interrupt lands while a cell is executing, which is where a sweep spends
        essentially all of its time. Measured before the removal was in a `finally`: an 80-cell sweep
        interrupted 3 s in left `batch-stream-0.jsonl` on disk while the `batch` record's
        `leftover_streams` was empty, so the retained output stated positively that nothing remained.
        """
        raw = build_stream(events=CONFLICT_EVENTS)

        def interrupting_runner(argv):
            path = argv[argv.index("--events-path") + 1]
            with open(path, "wb") as handle:
                handle.write(raw)
            # The stream is on disk and unread, which is exactly the state a Ctrl+C reaches.
            raise KeyboardInterrupt

        out = os.path.join(self.directory.name, "interrupted.jsonl")
        status = quiet_main(
            ["--out", out, "--binary", self.binary, "--sources", "social", "--densities", "0.75",
             "--seeds", "0-3", "--ticks", "10", "--stream-dir", self.directory.name],
            runner=interrupting_runner,
        )

        leftovers = sorted(
            name for name in os.listdir(self.directory.name) if name.startswith("batch-stream-")
        )
        self.assertEqual(leftovers, [], "rule 7.3's removal did not happen on an interrupt")

        records = read_records(out)
        batch_record = records[-1]
        self.assertEqual(status, 3)
        self.assertFalse(batch_record["complete"])
        self.assertEqual(batch_record["retained"], 0)
        self.assertEqual(
            [entry["stage"] for entry in batch_record["missing"]], ["not_attempted"] * 4
        )
        # The record must agree with the disk, which is the half that was wrong.
        self.assertEqual(batch_record["leftover_streams"], [])

    def test_one_reusable_path_at_the_default_jobs(self):
        """Rule 7.1: peak disk is one stream, which is why the path is reused."""
        self.assertEqual(
            batch.stream_path_for("D", 0), os.path.join("D", "batch-stream-0.jsonl")
        )
        self.assertNotEqual(batch.stream_path_for("D", 0), batch.stream_path_for("D", 1))

    def test_no_two_concurrent_cells_share_a_reusable_path(self):
        """Rule 7.1 under rule 2.8, as an exclusion property rather than as an output comparison.

        This is the case that catches a slot derived from the cell's position in the sweep. With
        eight cells over four workers, position 4 begins as soon as any of positions 0 to 3 finishes,
        so `index % jobs` can hand one path to two cells at once. The existing byte-identity test
        cannot see it: a runner that writes its stream instantly leaves no window for two cells to
        overlap, and the defect is invisible without one. Measured against a real engine before this
        test existed, two of eight cells were lost -- one read a half-written line, the other found
        the file already removed.

        The cells are given deliberately unequal durations, because equal ones hide the defect too: if
        the first four cells finish together, the second four start together and no two ever overlap
        on a shared slot. The real engine produced the unequal durations by itself; here the first
        cell is made slow so that the scheduling is reproduced deterministically rather than waited
        for.
        """
        raw = build_stream(events=CONFLICT_EVENTS)
        lock = threading.Lock()
        spans = []

        def slow_runner(argv):
            path = argv[argv.index("--events-path") + 1]
            policy = argv[argv.index("--policy") + 1]
            seed = argv[argv.index("--seed") + 1]
            with lock:
                entry = [path, time.perf_counter(), None]
                spans.append(entry)
            # The first cell of the sweep holds its slot while the other three of the first wave
            # finish, which is what frees a worker for the fifth cell.
            time.sleep(0.20 if (policy, seed) == ("social", "0") else 0.005)
            with open(path, "wb") as handle:
                handle.write(raw)
            with lock:
                entry[2] = time.perf_counter()
            return 0, ""

        out = os.path.join(self.directory.name, "exclusion.jsonl")
        status = quiet_main(
            ["--out", out, "--binary", self.binary, "--sources", "social,baseline",
             "--densities", "0.75", "--seeds", "0-3", "--ticks", "10",
             "--stream-dir", self.directory.name, "--jobs", "4"],
            runner=slow_runner,
        )
        self.assertEqual(status, 0)
        self.assertEqual(len(spans), 8)

        def overlapping(first, second):
            return first[1] < second[2] and second[1] < first[2]

        collisions = [
            (first[0], second[0])
            for index, first in enumerate(spans)
            for second in spans[index + 1:]
            if first[0] == second[0] and overlapping(first, second)
        ]
        self.assertEqual(collisions, [], "two concurrent cells were given one reusable path")

        # The test is only meaningful if cells did in fact overlap, so that is asserted too.
        overlaps = sum(
            1
            for index, first in enumerate(spans)
            for second in spans[index + 1:]
            if overlapping(first, second)
        )
        self.assertGreater(overlaps, 0, "no two cells overlapped, so exclusion was not exercised")

    def test_keep_stream_retains_one_cells_stream_and_changes_no_row(self):
        """Rule 2.7 and 7.4."""
        plain = read_records(self.sweep("h1.jsonl"))
        kept = read_records(self.sweep("h2.jsonl", ["--keep-stream", "social:0.75:1"]))
        self.assertEqual(
            [r for r in plain if r["record"] == "cell"],
            [r for r in kept if r["record"] == "cell"],
        )
        self.assertTrue(os.path.isfile(os.path.join(self.directory.name, "kept-social-0.75-1.jsonl")))

    def test_the_sweep_record_states_every_axis_value(self):
        """Rule 6.6: including values that produce no rows."""
        record = read_records(self.sweep("i.jsonl"))[0]
        self.assertEqual(record["sources"], ["social", "baseline"])
        self.assertEqual(record["densities"], ["0.75"])
        self.assertEqual(record["seeds"], [0, 1])
        self.assertEqual(record["cells"], 4)
        self.assertEqual(record["format"], 1)
        self.assertEqual(record["digest_algorithm"], "sha256")

    def test_the_stream_path_is_not_under_docs_or_scripts(self):
        """Rule 7.2."""
        directory = tempfile.gettempdir()
        path = batch.stream_path_for(directory, 0).replace("\\", "/")
        self.assertNotIn("/docs/", path)
        self.assertNotIn("/scripts/", path)


# ==================================================================================================
# Rule 19: division of labour
# ==================================================================================================


class TestDivisionOfLabour(unittest.TestCase):
    def test_the_driver_holds_no_threshold_and_names_no_class(self):
        """Rule 19.1: the driver executes and derives; it classifies nothing."""
        with open(batch.__file__, "r", encoding="utf-8") as handle:
            source = handle.read()
        # The vocabulary's names belong to the classifier. Neither may appear in the driver's code.
        for name in ("extinction", "famine", "asymmetric_collapse", "coexistence"):
            self.assertNotIn(name, source, f"{name!r} is the classifier's, not the driver's")

    def test_the_driver_imports_only_the_standard_library(self):
        """Rule 1.3."""
        self.assertEqual(third_party_imports(batch.__file__), [])

    def test_the_driver_does_not_import_the_classifier(self):
        """Rule 1.5: each instrument runs without the other.

        The driver's docstring names the classifier's path, which is a reference and not a coupling.
        This reads the import graph, so the two are distinguished.
        """
        self.assertNotIn("classify_simulation_runs", imported_modules(batch.__file__))

    def test_no_network_credential_or_live_option_appears(self):
        """Rule 19.4: neither instrument can reach a provider."""
        with open(batch.__file__, "r", encoding="utf-8") as handle:
            source = handle.read()
        for forbidden in ("urllib.request", "socket", "http.client", "--live", "--connector-path"):
            self.assertNotIn(forbidden, source)
        self.assertNotIn("os.environ", source)


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


def third_party_imports(path):
    """Every module a file imports that is not in the set above. Rule 1.3 admits none."""
    return [name for name in imported_modules(path) if name not in STANDARD_LIBRARY]


if __name__ == "__main__":
    unittest.main(verbosity=2)
