#!/usr/bin/env python3
"""Tests for `check_transcript_reading.py`.

A check over a committed artifact passes trivially when it looks at the wrong thing, and the
transcript it reads is one file that will not change on its own. So each case below breaks the
committed transcript in the one way the case it belongs to exists to catch, and requires the
program to refuse. A check with no negative case here is a check nobody has seen fail.

`VER-MOK-018` cases **L4**, **L5**, **L6**, **L12**, **L13**, **L14**, **L15a** and **L17** are the
cases under test. Run with:

    python3 scripts/test_check_transcript_reading.py

Reads the repository's own committed transcript, writes only into a temporary directory, reaches no
network and reads no credential.
"""

from __future__ import annotations

import io
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import check_transcript_reading as checker  # noqa: E402

TRANSCRIPT = (
    Path(__file__).resolve().parent.parent
    / "mokiterions-core"
    / "tests"
    / "transcript-seed0-ticks20-hunting.jsonl"
)


def records() -> list[dict]:
    """The committed transcript, as a list of records that a case can edit."""
    text = TRANSCRIPT.read_bytes().decode("utf-8")
    return [json.loads(line) for line in text.splitlines()]


def written(records_: list[dict], directory: Path, name: str = "t.jsonl") -> Path:
    """One transcript file, framed as rule 11.2 frames one and written with LF endings."""
    path = directory / name
    body = "".join(f"{json.dumps(record, separators=(',', ':'))}\n" for record in records_)
    with io.open(path, "w", encoding="utf-8", newline="") as handle:
        handle.write(body)
    return path


class Outcome:
    """One run of the program: its exit status and everything it printed."""

    def __init__(self, status: int, text: str) -> None:
        self.status = status
        self.text = text


def run(path: Path) -> Outcome:
    captured = io.StringIO()
    stdout, sys.stdout = sys.stdout, captured
    try:
        status = checker.main([str(path)])
    finally:
        sys.stdout = stdout
    return Outcome(status, captured.getvalue())


class TranscriptAsCommitted(unittest.TestCase):
    """The tree's own transcript, which is the case the evidence rests on."""

    def test_the_committed_transcript_passes_every_case(self) -> None:
        outcome = run(TRANSCRIPT)
        self.assertEqual(outcome.status, 0, outcome.text)
        for label in ("L4", "L5", "L6", "L12", "L13", "L14", "L15a", "L17"):
            self.assertIn(f"PASS  {label} ", outcome.text)
        self.assertNotIn("FAIL", outcome.text)

    def test_the_report_names_the_transcript_and_counts_the_requests(self) -> None:
        """`WO-MOK-025` evidence item 5 asks for both, so both are asserted rather than assumed."""
        outcome = run(TRANSCRIPT)
        self.assertIn(TRANSCRIPT.name, outcome.text)
        self.assertIn("requests examined: 221", outcome.text)
        self.assertIn("records: 233 (12 prefix, 221 exchange)", outcome.text)


class BrokenTranscripts(unittest.TestCase):
    """One edit per case, each the failure the case exists to catch."""

    def setUp(self) -> None:
        self.directory = Path(tempfile.mkdtemp(prefix="transcript-reading-"))

    def edited(self, mutate) -> Outcome:
        all_records = records()
        mutate(all_records)
        return run(written(all_records, self.directory))

    def first_exchange(self, all_records: list[dict]) -> dict:
        return next(record for record in all_records if record["transcript"] == "exchange")

    # ---- L4 -----------------------------------------------------------------------------------
    def test_a_withheld_action_fails_l4(self) -> None:
        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["permitted"] = exchange["permitted"].replace("  move east\n", "")

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L4", outcome.text)
        self.assertIn("withheld ['move east']", outcome.text)

    def test_an_unauthorised_action_fails_l4(self) -> None:
        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["permitted"] += "  eat F9999\n"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L4", outcome.text)
        self.assertIn("offered unauthorised ['eat F9999']", outcome.text)

    def test_a_move_off_the_grid_fails_l4(self) -> None:
        """The grid edge is the one part of the enumerator that is arithmetic rather than a list."""

        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["observation"] = exchange["observation"].replace(
                "position: 89:34 in territory A", "position: 0:0 in territory A"
            )

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L4", outcome.text)
        self.assertIn("offered unauthorised ['move north', 'move west']", outcome.text)

    def test_sleep_at_full_energy_fails_l4(self) -> None:
        def mutate(all_records):
            exchange = next(
                record
                for record in all_records
                if record["transcript"] == "exchange" and "  sleep\n" in record["permitted"]
            )
            exchange["observation"] = "\n".join(
                "energy: 100" if line.startswith("energy: ") else line
                for line in exchange["observation"].split("\n")
            )

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L4", outcome.text)
        self.assertIn("offered unauthorised ['sleep']", outcome.text)

    # ---- L6 -----------------------------------------------------------------------------------
    def test_a_contact_verb_against_a_distant_target_fails_l6(self) -> None:
        def mutate(all_records):
            exchange = next(
                record
                for record in all_records
                if record["transcript"] == "exchange"
                and "  approach M04\n" in record["permitted"]
            )
            exchange["permitted"] += "  attack M04\n"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L6", outcome.text)
        self.assertIn("attack M04's precondition is not met", outcome.text)

    def test_a_verb_against_an_uncarried_target_fails_l6(self) -> None:
        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["permitted"] += "  avoid M07\n"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L6", outcome.text)
        self.assertIn("names a Mokiterion the request never carried", outcome.text)

    # ---- L5 -----------------------------------------------------------------------------------
    def test_a_run_that_never_enumerates_a_targeted_action_fails_l5(self) -> None:
        """The case's own words: such a run "fails this case as unexercised"."""

        def mutate(all_records):
            keep = [record for record in all_records if record["transcript"] == "prefix"]
            for record in all_records:
                if record["transcript"] != "exchange":
                    continue
                observation = record["observation"]
                head, _, _tail = observation.partition("mokiterions perceived:\n")
                record["observation"] = f"{head}mokiterions perceived:\n  none\n"
                record["permitted"] = "".join(
                    line + "\n"
                    for line in record["permitted"].split("\n")
                    if line
                    and line.strip().split(" ")[0]
                    not in checker.TARGETED
                )
                keep.append(record)
            all_records[:] = keep

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L5", outcome.text)
        self.assertIn("0 of 221 requests enumerate a targeted action", outcome.text)

    def test_a_pass_through_of_the_core_list_fails_l5(self) -> None:
        """An implementation that returned the observation's own list, which rule 7.2 forbids."""

        def mutate(all_records):
            for record in all_records:
                if record["transcript"] != "exchange":
                    continue
                record["permitted"] = "".join(
                    line + "\n"
                    for line in record["permitted"].split("\n")
                    if line
                    and line.strip().split(" ")[0]
                    not in checker.TARGETED
                )

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        # It fails L4 as well, because withholding an authorised action is an incomplete
        # enumeration. Both are asserted: a reader has to be able to tell the two apart.
        self.assertIn("FAIL  L5", outcome.text)
        self.assertIn("FAIL  L4", outcome.text)
        self.assertIn("104 have a set equal to the core-proposal list", outcome.text)

    # ---- L12 ----------------------------------------------------------------------------------
    def test_another_mokiterions_attribute_fails_l12(self) -> None:
        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["observation"] = exchange["observation"].replace(
                "  M04 direction south_west distance 13",
                "  M04 direction south_west distance 13 health 74",
            )

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L12", outcome.text)
        self.assertIn("carries something else", outcome.text)

    def test_a_field_no_rule_authorises_fails_l12(self) -> None:
        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["observation"] = exchange["observation"].replace(
                "fear: 0\n", "fear: 0\nM04 fear: 40\n"
            )

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L12", outcome.text)
        self.assertIn("a line no rule authorises", outcome.text)

    def test_a_reordered_observation_fails_l12(self) -> None:
        """Rule 6.1 fixes the order, and two runs that differ in it are not one configuration."""

        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["observation"] = exchange["observation"].replace(
                "health: 100\nsatiety: 100\n", "satiety: 100\nhealth: 100\n"
            )

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L12", outcome.text)
        self.assertIn("rule 6.1 fixes", outcome.text)

    # ---- L13 ----------------------------------------------------------------------------------
    def test_a_figure_on_a_list_heading_fails_l13(self) -> None:
        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["observation"] = exchange["observation"].replace(
                "resources perceived:\n", "resources perceived: 13\n"
            )

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L13", outcome.text)
        self.assertIn("the heading carries a figure", outcome.text)

    def test_an_aggregate_over_the_population_fails_l13(self) -> None:
        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["permitted"] = exchange["permitted"].replace(
                "ACTIONS YOU MAY TAKE\n", "ACTIONS YOU MAY TAKE\n"
            )
            exchange["observation"] = exchange["observation"].replace(
                "WHAT YOU SEE\n", "WHAT YOU SEE\nliving: 11\n"
            )

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L13", outcome.text)
        self.assertIn("the request says 'living'", outcome.text)

    def test_a_response_carried_into_a_request_fails_l13(self) -> None:
        """Rule 12's replay carries no history, and this is the shape a history would arrive in."""

        def mutate(all_records):
            exchanges = [r for r in all_records if r["transcript"] == "exchange"]
            exchanges[0]["response"] = "approach M04"
            exchanges[1]["observation"] = exchanges[1]["observation"].replace(
                "WHAT YOU SEE\n", "WHAT YOU SEE\nprevious answer: approach M04\n"
            )

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L13", outcome.text)

    # ---- L14 ----------------------------------------------------------------------------------
    def test_a_prefix_that_moves_across_the_run_fails_l14(self) -> None:
        def mutate(all_records):
            exchanges = [r for r in all_records if r["transcript"] == "exchange"]
            last = [r for r in exchanges if r["actor"] == "M01"][-1]
            last["prefix_digest"] = "fnv1a64:0000000000000000"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L14", outcome.text)
        self.assertIn("the prefix moved", outcome.text)

    def test_an_accumulated_request_fails_l14(self) -> None:
        def mutate(all_records):
            exchanges = [r for r in all_records if r["transcript"] == "exchange"]
            theirs = [r for r in exchanges if r["actor"] == "M01"]
            theirs[-1]["observation"] = theirs[0]["observation"] + theirs[-1]["observation"]

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L14", outcome.text)
        self.assertIn("contains the other's observation", outcome.text)

    def test_a_session_identifier_fails_l14(self) -> None:
        def mutate(all_records):
            self.first_exchange(all_records)["conversation_id"] = "c_01"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L14", outcome.text)
        self.assertIn("carries 'conversation'", outcome.text)

    # ---- L15a ---------------------------------------------------------------------------------
    def test_a_shared_block_that_varies_fails_l15a(self) -> None:
        def mutate(all_records):
            prefixes = [r for r in all_records if r["transcript"] == "prefix"]
            prefixes[-1]["blocks"][0] = prefixes[-1]["blocks"][0] + "\n"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L15a", outcome.text)
        self.assertIn("2 distinct shared blocks", outcome.text)

    def test_a_prefix_digest_that_does_not_match_its_record_fails_l15a(self) -> None:
        def mutate(all_records):
            prefixes = [r for r in all_records if r["transcript"] == "prefix"]
            prefixes[0]["digest"] = "fnv1a64:ffffffffffffffff"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L15a", outcome.text)
        self.assertIn("is not the prefix record's", outcome.text)

    def test_an_exchange_reading_another_mokiterions_prefix_fails_l15a(self) -> None:
        def mutate(all_records):
            prefixes = {r["actor"]: r for r in all_records if r["transcript"] == "prefix"}
            exchange = self.first_exchange(all_records)
            exchange["prefix"] = "M02"
            exchange["prefix_digest"] = prefixes["M02"]["digest"]

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L15a", outcome.text)
        self.assertIn("reads M02's prefix", outcome.text)

    def test_the_observation_before_the_prefix_fails_l15a(self) -> None:
        """The layout is what the caching rests on, so its order is checked and not assumed."""

        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            reordered = {}
            for key in (
                "transcript",
                "version",
                "tick",
                "actor",
                "observation",
                "permitted",
                "prefix",
                "prefix_digest",
                "response",
                "usage",
                "action",
                "fallback",
            ):
                reordered[key] = exchange[key]
            exchange.clear()
            exchange.update(reordered)

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L15a", outcome.text)
        self.assertIn("fields out of order", outcome.text)

    # ---- L17 ----------------------------------------------------------------------------------
    def test_a_floating_point_value_fails_l17(self) -> None:
        """A cost figure is the form a float would most plausibly arrive in.

        Written into `usage`, which rule 11.3.1 leaves absent until a provider is called: the object
        this case guards is the one `WO-MOK-026` will fill in.
        """

        def mutate(all_records):
            self.first_exchange(all_records)["usage"] = {"cost_usd": 0.42}

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L17", outcome.text)
        self.assertIn("the floating-point value", outcome.text)

    def test_a_decimal_fraction_in_a_text_value_fails_l17(self) -> None:
        """A float rendered into prose before it was written is not caught by its type."""

        def mutate(all_records):
            exchange = self.first_exchange(all_records)
            exchange["response"] = "density 0.75"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L17", outcome.text)
        self.assertIn("a decimal fraction", outcome.text)

    def test_a_field_that_names_a_clock_fails_l17(self) -> None:
        def mutate(all_records):
            self.first_exchange(all_records)["usage"] = {"started": "before the others"}

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L17", outcome.text)
        self.assertIn("a field named 'started'", outcome.text)

    def test_a_value_shaped_like_a_clock_reading_fails_l17(self) -> None:
        """The field name is innocent here; the value is the timestamp."""

        def mutate(all_records):
            self.first_exchange(all_records)["response"] = "chosen at 12:30:05"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L17", outcome.text)
        self.assertIn("a value shaped like a clock reading", outcome.text)

    def test_an_epoch_second_fails_l17(self) -> None:
        """A clock that arrived as a number, which no field name would give away."""

        def mutate(all_records):
            self.first_exchange(all_records)["usage"] = {"prompt_tokens": 1_756_000_000}

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L17", outcome.text)
        self.assertIn("in the range a clock reads", outcome.text)

    def test_a_path_fails_l17(self) -> None:
        def mutate(all_records):
            self.first_exchange(all_records)["response"] = "read from docs/transcript.jsonl"

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("FAIL  L17", outcome.text)
        self.assertIn("the path separator", outcome.text)
        self.assertIn("a file extension", outcome.text)

    def test_the_transcripts_own_em_dashes_do_not_fail_l17(self) -> None:
        """The withdrawn clause, stated as a test: block A's prose is not a violation.

        `SPEC-MOK-007` rule 11.4.1 replaced the closed alphabet with a round trip, and this is what
        would notice if the clause were ever reinstated in this program by someone reading the case's
        text alone. The count is asserted, so a prose change that dropped the em dashes shows up here
        rather than silently making the case vacuous.
        """
        outcome = run(TRANSCRIPT)
        self.assertEqual(outcome.status, 0, outcome.text)
        self.assertIn("24 character(s) outside ASCII by design", outcome.text)

    # ---- the file itself ----------------------------------------------------------------------
    def test_a_carriage_return_is_refused(self) -> None:
        path = self.directory / "crlf.jsonl"
        body = TRANSCRIPT.read_bytes().decode("utf-8").replace("\n", "\r\n")
        path.write_bytes(body.encode("utf-8"))
        outcome = run(path)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("carriage return", outcome.text)
        self.assertIn(".gitattributes", outcome.text)

    def test_a_missing_final_newline_is_refused(self) -> None:
        path = self.directory / "unframed.jsonl"
        path.write_bytes(TRANSCRIPT.read_bytes().rstrip(b"\n"))
        outcome = run(path)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("rule 11.2", outcome.text)

    def test_a_record_of_neither_kind_is_refused(self) -> None:
        def mutate(all_records):
            all_records.append({"transcript": "summary", "version": 1})

        outcome = self.edited(mutate)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("neither a prefix nor an exchange", outcome.text)

    def test_a_file_that_is_not_a_transcript_is_refused(self) -> None:
        path = self.directory / "notjson.jsonl"
        path.write_bytes(b"not a record\n")
        outcome = run(path)
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("not a record", outcome.text)

    def test_a_transcript_that_is_not_there_is_refused_and_not_a_traceback(self) -> None:
        """The refusal path `main` already had, reached rather than bypassed.

        A missing file used to raise `OSError` past the `except Failure` handler, so the program
        printed a traceback where a FAIL line naming the transcript was intended. The handler was
        unreachable, which is the kind of defect only a test for the absent case finds.
        """
        outcome = run(self.directory / "absent.jsonl")
        self.assertEqual(outcome.status, 1, outcome.text)
        self.assertIn("absent.jsonl", outcome.text)
        self.assertIn("the transcript could not be read", outcome.text)
        self.assertNotIn("Traceback", outcome.text)

    def test_the_reported_path_carries_no_platform_separator(self) -> None:
        """The report line is captured into retained evidence, so it must not name a platform.

        `Path` prints with the running platform's separator, so the same command on Windows and on
        Linux would produce two captures differing in a line that says nothing about the run.
        """
        outcome = run(TRANSCRIPT)
        self.assertIn(f"transcript: {TRANSCRIPT.as_posix()}", outcome.text)
        self.assertNotIn("\\", outcome.text)


if __name__ == "__main__":
    unittest.main(verbosity=2)
