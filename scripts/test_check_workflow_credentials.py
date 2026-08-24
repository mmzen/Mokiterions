#!/usr/bin/env python3
"""Scenario suite for `check_workflow_credentials.py`.

Repository-owned, absent from `.engineering-harness.lock`.

`VER-MOK-018` cases **L21a** and **L21b** are the cases under test. The requirement they
verify, `REQ-MOK-073`, is unusual in that most of its subject is an *absence*, so a check
for it can pass by being broken: a scan whose patterns never match reports the same thing
as a repository that holds no credential. Most of the cases below therefore assert a
refusal on a fixture that plants one, and one group asserts the opposite — that the
ordinary spellings a real workflow uses are not findings, because a check that cried wolf
on `persist-credentials: false` would be turned off within a week.

The requirement's item 3 is the other way round: it requires a definition to *exist*, so
its cases are refusals on a fixture that removes one. Every temporary tree therefore
arrives with a replay step and the transcript it names already planted — see `Fixture` —
because otherwise every case asserting a pass would fail on the missing step rather than
on its own subject.

One case scans the repository's own definitions. That makes this suite fail on a commit
that plants a credential in a real workflow, in a developer's own `python -m unittest`
run, rather than only on the pull request. The check itself is the gate; this is the
smoke alarm beside it.

    python scripts/test_check_workflow_credentials.py
    python -m unittest discover -s scripts -p "test_*.py"
"""

from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

CHECK = Path(__file__).resolve().parent / "check_workflow_credentials.py"
REPOSITORY = CHECK.parent.parent

MINIMAL = """\
name: Fixture
on:
  pull_request:
jobs:
  fixture:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
"""

# A definition that satisfies clause E, in the shape the real one takes: the selection and
# the transcript on one line, because the check reads one line at a time.
REPLAY = """\
name: Fixture replay
on:
  pull_request:
jobs:
  replay:
    runs-on: ubuntu-latest
    steps:
      - run: ./Mokiterions --policy llm --transcript-path {transcript} --seed 0 --ticks 20
"""

# Nested rather than at the root, so that a check which forgot to join the named path to the
# root it was given would not find it by accident.
FIXTURE_TRANSCRIPT = "mokiterions-core/tests/transcript-fixture.jsonl"


class Outcome:
    """One run of the check, with its streams and its status."""

    def __init__(self, completed: subprocess.CompletedProcess[str]) -> None:
        self.status = completed.returncode
        self.stdout = completed.stdout
        self.stderr = completed.stderr

    @property
    def combined(self) -> str:
        return self.stdout + self.stderr


def run(root: Path) -> Outcome:
    return Outcome(
        subprocess.run(
            [sys.executable, str(CHECK), "--root", str(root)],
            capture_output=True,
            text=True,
            check=False,
        )
    )


class Fixture:
    """A tree with a `.github/workflows` directory and whatever a case puts in it.

    Plus a replay step and its transcript, planted at construction. Clause E requires one
    of every tree, so without it every case below that asserts a pass would be asserting
    the wrong refusal; the cases whose subject *is* the missing step take it away again.
    """

    def __init__(self, root: Path) -> None:
        self.root = root
        self.workflows = root / ".github" / "workflows"
        self.workflows.mkdir(parents=True)
        self.replay = self.replay_step()

    def replay_step(self, transcript: str = FIXTURE_TRANSCRIPT, name: str = "replay.yml") -> Path:
        """A definition that exercises the source in replay mode, and the file it names."""
        committed = self.root / transcript
        committed.parent.mkdir(parents=True, exist_ok=True)
        committed.write_text('{"transcript":"prefix"}\n', encoding="utf-8")
        return self.workflow(name, REPLAY.format(transcript=transcript))

    def without_replay_step(self) -> None:
        """Remove the planted definition. The transcript it named stays, unreferenced."""
        self.replay.unlink()

    def workflow(self, name: str, body: str) -> Path:
        path = self.workflows / name
        path.write_text(body, encoding="utf-8")
        return path

    def minimal(self, name: str = "fixture.yml") -> Path:
        return self.workflow(name, MINIMAL)

    def with_step_line(self, line: str, name: str = "fixture.yml") -> Path:
        """The minimal workflow plus one line inside its single step.

        The planted line is always the last line of the file, so a case can assert the
        line number the check reports without counting the preamble by hand.
        """
        return self.workflow(name, MINIMAL + f"      {line}\n")

    @property
    def planted_line_number(self) -> int:
        return len(MINIMAL.splitlines()) + 1


class Case(unittest.TestCase):
    """A temporary tree per case; nothing here touches the working repository."""

    def setUp(self) -> None:
        self._directory = tempfile.TemporaryDirectory()
        self.fixture = Fixture(Path(self._directory.name))

    def tearDown(self) -> None:
        self._directory.cleanup()

    def assertRefused(self, outcome: Outcome) -> None:
        self.assertEqual(outcome.status, 1, outcome.combined)
        self.assertIn("REFUSED", outcome.stderr, outcome.combined)

    def assertPassed(self, outcome: Outcome) -> None:
        self.assertEqual(outcome.status, 0, outcome.combined)
        self.assertIn("PASS", outcome.stdout, outcome.combined)
        self.assertNotIn("REFUSED", outcome.stderr, outcome.combined)


class TheRepositoryItself(Case):
    def test_the_repositorys_own_definitions_hold_no_credential(self) -> None:
        """The property the check exists to establish, on the tree it exists for."""
        outcome = run(REPOSITORY)
        self.assertPassed(outcome)
        self.assertIn(".github/workflows/release.yml", outcome.stdout)

    def test_the_scan_reports_what_it_read(self) -> None:
        """A pass has to be distinguishable from a scan that examined nothing."""
        outcome = run(REPOSITORY)
        self.assertRegex(outcome.stdout, r"\d+ file\(s\), \d+ non-comment line\(s\) scanned")

    def test_the_repositorys_own_replay_step_is_found_and_named(self) -> None:
        """`VER-MOK-018` case L21b, on the tree the case is about.

        Named rather than counted: a reader of a green log is told which definition
        exercises the source and which file it replays, so a step that quietly moved to a
        different transcript is visible in the log rather than only in a diff.
        """
        outcome = run(REPOSITORY)
        self.assertPassed(outcome)
        self.assertIn("CHECKED: VER-MOK-018 case L21b", outcome.stdout)
        self.assertIn(".github/workflows/provider-credentials.yml", outcome.stdout)
        self.assertIn(
            "mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl", outcome.stdout
        )

    def test_the_case_that_is_not_checkable_here_is_printed_rather_than_implied(self) -> None:
        """L21a is an owner attestation, and a pass must not read as covering it."""
        outcome = run(REPOSITORY)
        self.assertIn("NOT CHECKABLE HERE", outcome.stdout)
        self.assertIn("Actions", outcome.stdout)


class ScanA_Secrets(Case):
    def test_a_secret_reference_refuses_and_names_file_and_line(self) -> None:
        self.fixture.with_step_line("env: { PROVIDER: ${{ secrets.MODEL_PROVIDER_KEY }} }")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn(
            f".github/workflows/fixture.yml:{self.fixture.planted_line_number}", outcome.stderr
        )
        self.assertIn("not as a secret", outcome.stderr)

    def test_the_platforms_own_token_is_not_a_finding(self) -> None:
        self.fixture.with_step_line("env: { GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }} }")
        self.assertPassed(run(self.fixture.root))

    def test_a_secret_named_nothing_like_a_credential_still_refuses(self) -> None:
        """The allowlist is closed on purpose: a credential is spendable whatever its name."""
        self.fixture.with_step_line("env: { JUICE: ${{ secrets.SPARKLE_JUICE }} }")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("SPARKLE_JUICE", outcome.stderr)

    def test_the_subscript_form_refuses(self) -> None:
        self.fixture.with_step_line("""env: { P: ${{ secrets['MODEL_KEY'] }} }""")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("MODEL_KEY", outcome.stderr)

    def test_handing_over_the_whole_context_refuses(self) -> None:
        """`toJSON(secrets)` names nothing, so a name-based allowlist cannot see it."""
        self.fixture.with_step_line("env: { ALL: ${{ toJSON(secrets) }} }")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("whole secrets context", outcome.stderr)


class ScanB_Names(Case):
    def test_a_provider_name_refuses(self) -> None:
        self.fixture.with_step_line("run: curl https://api.openai.com/v1/responses")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("openai", outcome.stderr)

    def test_a_provider_name_refuses_in_any_case(self) -> None:
        self.fixture.with_step_line("env: { BASE: https://API.OpenAI.COM }")
        self.assertRefused(run(self.fixture.root))

    def test_a_credential_shaped_environment_name_refuses(self) -> None:
        self.fixture.with_step_line("env: { LUNA_API_KEY: fixture }")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("LUNA_API_KEY", outcome.stderr)

    def test_each_credential_word_is_covered(self) -> None:
        for name in (
            "PROVIDER_APIKEY",
            "MODEL_ACCESS_KEY",
            "SIGNING_PRIVATE_KEY",
            "CONNECTOR_SECRET",
            "CONNECTOR_TOKEN",
            "PROVIDER_CREDENTIAL",
            "ACCOUNT_PASSWORD",
            "ACCOUNT_PASSWD",
            "AUTH_BEARER",
        ):
            with self.subTest(name=name):
                self.fixture.with_step_line(f"env: {{ {name}: fixture }}")
                self.assertRefused(run(self.fixture.root))

    def test_the_platforms_token_names_are_not_findings(self) -> None:
        self.fixture.with_step_line("env: { GH_TOKEN: ${{ github.token }} }")
        self.assertPassed(run(self.fixture.root))

    def test_a_lower_case_input_that_merely_reads_as_credential_is_not_a_finding(self) -> None:
        """The false positive that would get this check switched off."""
        self.fixture.workflow(
            "checkout.yml",
            MINIMAL + "        with:\n          persist-credentials: false\n",
        )
        self.assertPassed(run(self.fixture.root))

    def test_an_unrelated_upper_case_name_is_not_a_finding(self) -> None:
        self.fixture.with_step_line("env: { CARGO_TERM_COLOR: always }")
        self.assertPassed(run(self.fixture.root))


class ScanC_Brokers(Case):
    def test_a_broker_action_refuses(self) -> None:
        for action in (
            "azure/login@v2",
            "aws-actions/configure-aws-credentials@v4",
            "google-github-actions/auth@v2",
            "hashicorp/vault-action@v3",
            "1password/load-secrets-action@v2",
        ):
            with self.subTest(action=action):
                self.fixture.with_step_line(f"uses: {action}")
                outcome = run(self.fixture.root)
                self.assertRefused(outcome)
                self.assertIn("fetches one", outcome.stderr)

    def test_a_broker_command_refuses(self) -> None:
        for command in (
            "run: op read op://vault/model/key",
            "run: vault kv get -field=key secret/model",
            "run: aws secretsmanager get-secret-value --secret-id model",
            "run: az keyvault secret show --name model",
            "run: gcloud secrets versions access latest --secret=model",
        ):
            with self.subTest(command=command):
                self.fixture.with_step_line(command)
                self.assertRefused(run(self.fixture.root))


class ScanD_LiveMode(Case):
    def test_a_live_mode_option_refuses(self) -> None:
        for option in ("--live", "--live-mode", "--no-replay", "--connector /usr/bin/connector"):
            with self.subTest(option=option):
                self.fixture.with_step_line(f"run: ./Mokiterions {option}")
                outcome = run(self.fixture.root)
                self.assertRefused(outcome)
                self.assertIn("live-mode selection", outcome.stderr)

    def test_a_longer_option_that_merely_begins_the_same_way_is_not_a_finding(self) -> None:
        self.fixture.with_step_line("run: ./Mokiterions --liveness-probe")
        self.assertPassed(run(self.fixture.root))

    def test_selecting_the_model_backed_source_with_no_transcript_refuses(self) -> None:
        """Every spelling of the selection, none of them naming a transcript."""
        for line in (
            "run: ./Mokiterions --policy llm",
            "run: ./Mokiterions --policy=llm",
            "policy: llm",
        ):
            with self.subTest(line=line):
                self.fixture.with_step_line(line)
                outcome = run(self.fixture.root)
                self.assertRefused(outcome)
                self.assertIn("names no transcript on the same line", outcome.stderr)

    def test_a_selection_naming_a_transcript_that_is_not_committed_refuses(self) -> None:
        self.fixture.with_step_line(
            "run: ./Mokiterions --policy llm --transcript-path fixtures/absent.jsonl"
        )
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("fixtures/absent.jsonl", outcome.stderr)
        self.assertIn("not a file in this repository", outcome.stderr)

    def test_a_selection_replaying_a_committed_transcript_passes(self) -> None:
        """The permitted case, which is the only thing `REQ-MOK-073` item 3 allows."""
        self.fixture.without_replay_step()
        self.fixture.replay_step(name="another.yml")
        outcome = run(self.fixture.root)
        self.assertPassed(outcome)
        self.assertIn(FIXTURE_TRANSCRIPT, outcome.stdout)

    def test_a_transcript_named_on_another_line_is_not_seen(self) -> None:
        """The one-line constraint, asserted rather than left as a comment.

        A `run: |` block would put the selection and its transcript on separate lines, and
        this check joins none, so it cannot tell that pair from a selection with no
        transcript at all. It refuses, which is the safe direction, and this case is what
        keeps the refusal deliberate: a future rewrite that reads block scalars will make
        this case fail and have to decide what the new behaviour is.
        """
        self.fixture.workflow(
            "block.yml",
            MINIMAL
            + "      run: |\n"
            + "        ./Mokiterions --policy llm \\\n"
            + f"          --transcript-path {FIXTURE_TRANSCRIPT}\n",
        )
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("names no transcript on the same line", outcome.stderr)

    def test_the_four_existing_sources_are_not_findings(self) -> None:
        for source in ("baseline", "reference", "individual", "social"):
            with self.subTest(source=source):
                self.fixture.with_step_line(f"run: ./Mokiterions --policy {source}")
                self.assertPassed(run(self.fixture.root))


class ScanE_TheSourceIsExercised(Case):
    """`REQ-MOK-073` item 3 and `VER-MOK-018` case L21b: the one clause that requires a
    definition to exist rather than to be absent."""

    def test_a_tree_that_never_exercises_the_source_refuses(self) -> None:
        self.fixture.without_replay_step()
        self.fixture.minimal()
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("no definition exercises the model-backed source", outcome.stderr)
        self.assertIn("L21b", outcome.stderr)

    def test_a_refused_selection_does_not_satisfy_this_clause(self) -> None:
        """A selection this check refused is not evidence that the source is exercised.

        Both refusals appear, which is the point: fixing the D refusal by deleting the step
        would leave the E refusal, and fixing it by naming a committed transcript clears
        both. Either way the tree cannot come to rest on a selection that may reach a
        provider.
        """
        self.fixture.without_replay_step()
        self.fixture.with_step_line("run: ./Mokiterions --policy llm")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("names no transcript on the same line", outcome.stderr)
        self.assertIn("no definition exercises the model-backed source", outcome.stderr)

    def test_a_pass_states_the_clause_it_established(self) -> None:
        self.fixture.minimal()
        outcome = run(self.fixture.root)
        self.assertPassed(outcome)
        self.assertIn("scan E", outcome.stdout)
        self.assertIn(f".github/workflows/replay.yml:{len(REPLAY.splitlines())}", outcome.stdout)


class WhatIsScanned(Case):
    def test_a_comment_is_not_a_reference(self) -> None:
        """A comment is resolved by no runner, and this check's own workflow needs to
        be able to state the prohibition it enforces."""
        self.fixture.workflow(
            "commented.yml",
            MINIMAL + "      # never add ${{ secrets.PROVIDER_API_KEY }} here, not ever\n",
        )
        self.assertPassed(run(self.fixture.root))

    def test_an_indented_comment_is_still_a_comment(self) -> None:
        self.fixture.workflow("commented.yml", MINIMAL + "          #   OPENAI_API_KEY\n")
        self.assertPassed(run(self.fixture.root))

    def test_the_yaml_suffix_is_scanned_too(self) -> None:
        self.fixture.with_step_line("env: { X: ${{ secrets.K }} }", name="fixture.yaml")
        self.assertRefused(run(self.fixture.root))

    def test_a_composite_action_is_scanned(self) -> None:
        """A step is where clause C's fetch happens, so the wider `.github` tree is read."""
        self.fixture.minimal()
        action = self.fixture.root / ".github" / "actions" / "broker"
        action.mkdir(parents=True)
        (action / "action.yml").write_text(
            "name: Broker\nruns:\n  using: composite\n  steps:\n"
            "    - uses: azure/login@v2\n",
            encoding="utf-8",
        )
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn(".github/actions/broker/action.yml", outcome.stderr)

    def test_every_finding_is_reported_not_only_the_first(self) -> None:
        self.fixture.workflow(
            "many.yml",
            MINIMAL
            + "      env: { A: ${{ secrets.ONE }} }\n"
            + "      run: ./Mokiterions --live\n"
            + "      uses: azure/login@v2\n",
        )
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("3 credential or live-mode reference(s)", outcome.stderr)

    def test_findings_across_files_are_counted_by_file(self) -> None:
        self.fixture.with_step_line("env: { A: ${{ secrets.ONE }} }", name="one.yml")
        self.fixture.with_step_line("env: { B: ${{ secrets.TWO }} }", name="two.yml")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("in 2 file(s)", outcome.stderr)


class FailsClosed(Case):
    def test_a_missing_directory_refuses(self) -> None:
        root = self.fixture.root / "elsewhere"
        root.mkdir()
        outcome = run(root)
        self.assertRefused(outcome)
        self.assertIn("is not a directory", outcome.stderr)

    def test_a_directory_holding_no_definition_refuses(self) -> None:
        # The planted replay step is a definition, so it goes: the subject here is the
        # directory that holds none, and this refusal happens before any scan runs.
        self.fixture.without_replay_step()
        (self.fixture.workflows / "notes.txt").write_text("nothing\n", encoding="utf-8")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("holds no", outcome.stderr)

    def test_a_definition_that_cannot_be_decoded_refuses(self) -> None:
        self.fixture.minimal()
        (self.fixture.workflows / "binary.yml").write_bytes(b"name: \xff\xfe\n")
        outcome = run(self.fixture.root)
        self.assertRefused(outcome)
        self.assertIn("not valid UTF-8", outcome.stderr)


class LeavesTheTreeAlone(Case):
    def test_the_check_writes_nothing(self) -> None:
        self.fixture.minimal()
        before = sorted(
            (path.relative_to(self.fixture.root).as_posix(), path.stat().st_size)
            for path in self.fixture.root.rglob("*")
            if path.is_file()
        )
        self.assertPassed(run(self.fixture.root))
        after = sorted(
            (path.relative_to(self.fixture.root).as_posix(), path.stat().st_size)
            for path in self.fixture.root.rglob("*")
            if path.is_file()
        )
        self.assertEqual(before, after)


if __name__ == "__main__":
    unittest.main(verbosity=2)
