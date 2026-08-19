#!/usr/bin/env python3
"""Scenario suite for `check_release_authorization.py`.

Repository-owned, like the script it tests, and absent from `.engineering-harness.lock`.

Every test is named after the scenario it discharges in
`docs/engineering/simulation/verification/VER-MOK-008.md`, so the mapping from contract to
coverage is mechanical rather than argued. `VER-MOK-008` enumerates the refusal set from
`SPEC-MOK-005` rule 4; coverage is measured against that enumeration and not against this
file's branches. A scenario in the contract with no test here is a gap to report.

`R23` and `R24` are reachability scenarios. Rule 5 places reachability outside the gate, so
they live in `test_check_release_reachability.py`.

The gate exists to refuse, so every refusal asserts two things: a failing exit status, and a
message naming the fact that could not be established. Asserting only the status would pass a
gate that refuses everything for one reason. Each test additionally asserts that the run left
the repository untouched, which is `VER-MOK-008`'s refusal-totality property applied across
the whole set rather than once.

Fixtures are real: `git init`, real commits, real annotated tags, real `+++`-delimited TOML.
Nothing is stubbed, so a scenario cannot pass because a stub agreed with the gate about what
git would have said. `A5` goes further and runs against this repository's own artifact graph,
in a throwaway clone so that nothing real is touched.

    python scripts/test_check_release_authorization.py
    python -m unittest discover -s scripts -p "test_*.py"
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path

GATE = Path(__file__).resolve().parent / "check_release_authorization.py"
REPOSITORY = Path(__file__).resolve().parents[1]

CONFIG = """\
[harness]
schema_version = 2
tool_version = "0.4.0"
project_name = "Fixture"
artifact_root = "{artifact_root}"

[revision_provenance]
require_full_commit = {require_full_commit}
"""

ZERO_COMMIT = "0" * 40
OTHER_COMMIT = "a" * 40


def git(root: Path, *arguments: str) -> str:
    completed = subprocess.run(
        ["git", "-C", str(root), *arguments],
        capture_output=True,
        text=True,
        check=True,
    )
    return completed.stdout.strip()


def run_gate(root: Path, tag: str, **environment: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(GATE), "--root", str(root), "--tag", tag],
        capture_output=True,
        text=True,
        check=False,
        env={**os.environ, **environment} if environment else None,
    )


class Fixture:
    """A synthetic harnessed repository with one annotated tag, `v1.0.0`."""

    def __init__(self, root: Path) -> None:
        self.root = root
        self.artifact_root = root / "docs" / "engineering"
        self.set_config()
        self.domain = self.artifact_root / "domain"
        for directory in ("work-orders", "verification", "release"):
            (self.domain / directory).mkdir(parents=True, exist_ok=True)

        git(root, "init", "--quiet", "--initial-branch=master")
        git(root, "config", "user.email", "fixture@example.invalid")
        git(root, "config", "user.name", "Fixture Owner")
        git(root, "add", "-A")
        git(root, "commit", "--quiet", "-m", "fixture")
        self.commit = git(root, "rev-parse", "HEAD")
        git(root, "tag", "-a", "v1.0.0", "-m", "release 1.0.0")

    def set_config(self, *, artifact_root: str = "docs/engineering", require_full_commit: str = "true") -> None:
        (self.root / ".engineering-harness.toml").write_text(
            CONFIG.format(artifact_root=artifact_root, require_full_commit=require_full_commit),
            encoding="utf-8",
        )

    def artifact(self, relative: str, front_matter: str) -> Path:
        path = self.domain / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        body = textwrap.dedent(front_matter).strip()
        path.write_text(f"+++\n{body}\n+++\n\nbody\n", encoding="utf-8")
        return path

    def write_graph(
        self,
        *,
        record_status: str = "released",
        record_commit: str | None = None,
        record_tag: str | None = "v1.0.0",
        version: str = "1.0.0",
        verification_status: str = "verified",
        verification_commit: str | None = None,
        verification_type: str = "verification_record",
        work_order_status: str = "implemented",
        work_order_type: str = "work_order",
        gates: tuple[str, ...] = ("WO-001",),
        releases_work: tuple[str, ...] = ("WO-001",),
        covers_work: tuple[str, ...] = ("WO-001",),
        contract_status: str = "approved",
        contract_type: str = "release_contract",
    ) -> None:
        """A graph that authorizes `v1.0.0`, unless a keyword breaks it on purpose."""
        record_commit = self.commit if record_commit is None else record_commit
        verification_commit = self.commit if verification_commit is None else verification_commit
        listed = lambda ids: ", ".join(f'"{identifier}"' for identifier in ids)

        for stale in ("release/RLS-002.md",):
            (self.domain / stale).unlink(missing_ok=True)

        self.artifact(
            "work-orders/WO-001.md",
            f"""
            id = "WO-001"
            type = "{work_order_type}"
            status = "{work_order_status}"
            """,
        )
        self.artifact(
            "verification/VREC-001.md",
            f"""
            id = "VREC-001"
            type = "{verification_type}"
            status = "{verification_status}"
            commit = "{verification_commit}"
            git_object_format = "sha1"

            [relations]
            verifies_work_order = [{listed(covers_work)}]
            """,
        )
        self.artifact(
            "release/REL-001.md",
            f"""
            id = "REL-001"
            type = "{contract_type}"
            status = "{contract_status}"

            [relations]
            gates = [{listed(gates)}]
            """,
        )
        tag_line = f'tag = "{record_tag}"' if record_tag is not None else ""
        self.artifact(
            "release/RLS-001.md",
            f"""
            id = "RLS-001"
            type = "release_record"
            status = "{record_status}"
            version = "{version}"
            commit = "{record_commit}"
            git_object_format = "sha1"
            {tag_line}

            [relations]
            satisfies = ["REL-001"]
            includes_verification = ["VREC-001"]
            releases_work = [{listed(releases_work)}]
            """,
        )

    def snapshot(self) -> tuple[str, str, str]:
        """Everything the gate is forbidden to change: the tree, the tags, the branches."""
        return (
            git(self.root, "status", "--porcelain", "--untracked-files=all"),
            git(self.root, "tag", "--list"),
            git(self.root, "branch", "--list"),
        )

    def run_gate(self, tag: str = "v1.0.0", **environment: str) -> subprocess.CompletedProcess[str]:
        return run_gate(self.root, tag, **environment)


class GateTest(unittest.TestCase):
    def setUp(self) -> None:
        self._temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self._temporary.cleanup)
        self.fixture = Fixture(Path(self._temporary.name))

    def assert_authorized(self, tag: str = "v1.0.0") -> subprocess.CompletedProcess[str]:
        before = self.fixture.snapshot()
        result = self.fixture.run_gate(tag)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("AUTHORIZED", result.stdout)
        self.assertEqual(before, self.fixture.snapshot(), "the gate changed the repository")
        return result

    def assert_refused(self, expected: str, tag: str = "v1.0.0") -> subprocess.CompletedProcess[str]:
        before = self.fixture.snapshot()
        result = self.fixture.run_gate(tag)
        self.assertEqual(result.returncode, 1, f"expected a refusal, got:\n{result.stdout}")
        self.assertIn("REFUSED", result.stderr)
        self.assertIn(expected, result.stderr)
        # VER-MOK-008 refusal totality: asserted on every refusal, not once.
        self.assertEqual(before, self.fixture.snapshot(), "the gate changed the repository")
        return result

    # -- authorizing scenarios -------------------------------------------------------------

    def test_a1_a_correct_graph_authorizes(self) -> None:
        self.fixture.write_graph()
        self.assert_authorized()

    def test_a2_the_record_lives_in_a_later_commit_than_the_one_it_names(self) -> None:
        """The real order of events, and the one the workflow's checkout must match.

        A release record quotes the candidate commit's hash, so it can only be written in a
        later commit -- the tagged tree never contains its own release record. The gate reads
        the graph from the integration branch and resolves the tag separately, so this must
        authorize. The absence is asserted rather than assumed.
        """
        candidate = self.fixture.commit
        self.fixture.write_graph(record_commit=candidate)
        git(self.fixture.root, "add", "-A")
        git(self.fixture.root, "commit", "--quiet", "-m", "record the release")
        self.assertNotEqual(git(self.fixture.root, "rev-parse", "HEAD"), candidate)

        absent = subprocess.run(
            ["git", "-C", str(self.fixture.root), "cat-file", "-e",
             "v1.0.0:docs/engineering/domain/release/RLS-001.md"],
            capture_output=True,
            check=False,
        )
        self.assertNotEqual(absent.returncode, 0, "the tagged tree should not hold the record")

        self.assert_authorized()

    def test_a3_a_record_that_states_no_tag(self) -> None:
        # `tag` is optional; the conventional v<version> tag is then the match.
        self.fixture.write_graph(record_tag=None)
        self.assert_authorized()

    def test_a3_a_record_stating_a_different_tag_is_not_a_match(self) -> None:
        # The v<version> fallback applies only when the record states no tag at all.
        self.fixture.write_graph(record_tag="v9.9.9")
        self.assert_refused("expected exactly one released release record for tag v1.0.0")

    def test_a4_the_authorized_facts_are_reported(self) -> None:
        self.fixture.write_graph()
        stdout = self.assert_authorized().stdout
        for expected in ("1.0.0", "RLS-001", "REL-001", "WO-001", "VREC-001", self.fixture.commit):
            self.assertIn(expected, stdout)

    def test_a4_the_authorized_facts_are_emitted_for_the_workflow(self) -> None:
        # The workflow consumes these as step outputs; nothing else is written anywhere.
        self.fixture.write_graph()
        destination = Path(self._temporary.name) / "outputs.txt"
        result = self.fixture.run_gate("v1.0.0", GITHUB_OUTPUT=str(destination))
        self.assertEqual(result.returncode, 0, result.stderr)
        emitted = dict(
            line.split("=", 1) for line in destination.read_text(encoding="utf-8").splitlines()
        )
        self.assertEqual(emitted["commit"], self.fixture.commit)
        self.assertEqual(emitted["version"], "1.0.0")
        self.assertEqual(emitted["record"], "RLS-001")
        self.assertEqual(emitted["contract"], "REL-001")
        self.assertEqual(emitted["work_orders"], "WO-001")
        self.assertEqual(emitted["verification_records"], "VREC-001")

    # -- refusal scenarios, keyed to VER-MOK-008's table ------------------------------------

    def test_r1_the_requested_tag_does_not_exist(self) -> None:
        self.fixture.write_graph()
        self.assert_refused("is not a tag in this repository", tag="v2.0.0")

    def test_r2_the_requested_name_is_a_branch_not_a_tag(self) -> None:
        """Tag resolution is confined to `refs/tags/`, so a branch cannot impersonate a tag."""
        self.fixture.write_graph()
        git(self.fixture.root, "branch", "v2.0.0")
        # The name resolves as a revision; it still is not a tag.
        self.assertEqual(git(self.fixture.root, "rev-parse", "v2.0.0"), self.fixture.commit)
        self.assert_refused("is not a tag in this repository", tag="v2.0.0")

    def test_r3_the_tag_is_lightweight_rather_than_annotated(self) -> None:
        self.fixture.write_graph(record_tag="v1.1.0")
        git(self.fixture.root, "tag", "v1.1.0")
        self.assert_refused("is a lightweight tag", tag="v1.1.0")

    def test_r4_two_artifacts_share_an_identifier(self) -> None:
        self.fixture.write_graph()
        source = self.fixture.domain / "release" / "RLS-001.md"
        (self.fixture.domain / "release" / "RLS-001-copy.md").write_text(
            source.read_text(encoding="utf-8"), encoding="utf-8"
        )
        self.assert_refused("duplicate artifact ID RLS-001")

    def test_r5_the_repository_does_not_require_full_commits(self) -> None:
        """The gate compares complete hashes, so it will not run where prefixes are allowed."""
        self.fixture.write_graph()
        self.fixture.set_config(require_full_commit="false")
        self.assert_refused("require_full_commit = false")

    def test_r6_no_release_record_exists_at_all(self) -> None:
        self.fixture.write_graph()
        (self.fixture.domain / "release" / "RLS-001.md").unlink()
        self.assert_refused("No release record exists at all")

    def test_r7_the_release_record_is_ready_not_released(self) -> None:
        self.fixture.write_graph(record_status="ready")
        refusal = self.assert_refused("no release record has status 'released'")
        self.assertIn("RLS-001", refusal.stderr)

    def test_r8_two_released_records_claim_the_same_tag(self) -> None:
        self.fixture.write_graph()
        source = self.fixture.domain / "release" / "RLS-001.md"
        duplicate = self.fixture.domain / "release" / "RLS-002.md"
        duplicate.write_text(
            source.read_text(encoding="utf-8").replace("RLS-001", "RLS-002"), encoding="utf-8"
        )
        self.assert_refused("expected exactly one released release record")

    def test_r9_the_records_commit_is_abbreviated(self) -> None:
        self.fixture.write_graph(record_commit=self.fixture.commit[:7])
        self.assert_refused("does not carry a full sha1 commit")

    def test_r10_the_records_commit_is_an_unrelated_commit(self) -> None:
        self.fixture.write_graph(record_commit=ZERO_COMMIT)
        self.assert_refused("Refusing to publish assets the release owner did not approve")

    def test_r11_the_tag_was_force_moved_after_the_record_was_written(self) -> None:
        # The graph is entirely correct; the tag simply moved to a later commit.
        self.fixture.write_graph()
        git(self.fixture.root, "add", "-A")
        git(self.fixture.root, "commit", "--quiet", "-m", "later work")
        git(self.fixture.root, "tag", "--force", "-a", "v1.0.0", "-m", "retag")
        self.assert_refused("Refusing to publish assets the release owner did not approve")

    def test_r12_the_gating_contract_does_not_exist(self) -> None:
        self.fixture.write_graph()
        (self.fixture.domain / "release" / "REL-001.md").unlink()
        self.assert_refused("REL-001 does not exist in the formal graph")

    def test_r13_the_gating_contracts_declared_type_is_not_a_release_contract(self) -> None:
        self.fixture.write_graph(contract_type="work_order")
        self.assert_refused("REL-001 is a 'work_order', not a 'release_contract'")

    def test_r14_the_gating_contract_is_draft(self) -> None:
        self.fixture.write_graph(contract_status="draft")
        self.assert_refused("REL-001 is 'draft', not active")

    def test_r15_the_contract_does_not_gate_a_released_work_order(self) -> None:
        self.fixture.write_graph(gates=("WO-002",))
        self.assert_refused("does not gate: WO-001")

    def test_r16_a_released_work_order_is_approved_not_releasable(self) -> None:
        self.fixture.write_graph(work_order_status="approved")
        self.assert_refused("work order WO-001 is 'approved'")

    def test_r17_an_included_verification_record_does_not_exist(self) -> None:
        self.fixture.write_graph()
        (self.fixture.domain / "verification" / "VREC-001.md").unlink()
        self.assert_refused("VREC-001 does not exist in the formal graph")

    def test_r18_an_included_verification_record_is_ready(self) -> None:
        self.fixture.write_graph(verification_status="ready")
        self.assert_refused("VREC-001 is 'ready'")

    def test_r19_an_included_verification_record_names_another_commit(self) -> None:
        self.fixture.write_graph(verification_commit=OTHER_COMMIT)
        self.assert_refused("one verification record captured at the final candidate commit")

    def test_r20_work_is_released_but_not_verified(self) -> None:
        self.fixture.write_graph(gates=("WO-001", "WO-002"), releases_work=("WO-001", "WO-002"))
        self.assert_refused("released but not verified: WO-002")

    def test_r21_work_is_verified_but_not_released(self) -> None:
        self.fixture.write_graph(covers_work=("WO-001", "WO-002"))
        self.assert_refused("verified but not released: WO-002")

    def test_r22_the_records_version_is_not_usable_as_a_name(self) -> None:
        self.fixture.write_graph(version="not a version!")
        self.assert_refused("unusable version")

    # -- P4: the gate writes nothing --------------------------------------------------------

    def test_p4_the_gate_writes_nothing_when_it_authorizes(self) -> None:
        self.fixture.write_graph()
        git(self.fixture.root, "add", "-A")
        git(self.fixture.root, "commit", "--quiet", "-m", "record the release")
        (self.fixture.root / "untracked.txt").write_text("untracked\n", encoding="utf-8")
        before = self.fixture.snapshot()
        self.assertEqual(self.fixture.run_gate().returncode, 0)
        self.assertEqual(before, self.fixture.snapshot())
        self.assertEqual((self.fixture.root / "untracked.txt").read_text(encoding="utf-8"), "untracked\n")

    def test_p4_the_gate_writes_nothing_when_it_refuses(self) -> None:
        # Every refusal test asserts this too; this one states it as its own claim.
        self.fixture.write_graph(record_status="ready")
        (self.fixture.root / "untracked.txt").write_text("untracked\n", encoding="utf-8")
        before = self.fixture.snapshot()
        self.assertEqual(self.fixture.run_gate().returncode, 1)
        self.assertEqual(before, self.fixture.snapshot())

    # -- properties and invariants ---------------------------------------------------------

    def test_property_commit_equality_is_total_not_prefix(self) -> None:
        """A record commit that is a strict prefix of the tagged commit is still refused."""
        prefix = self.fixture.commit[:39]
        self.assertTrue(self.fixture.commit.startswith(prefix))
        self.fixture.write_graph(record_commit=prefix)
        self.assert_refused("does not carry a full sha1 commit")

    def test_property_type_is_read_for_a_work_order(self) -> None:
        self.fixture.write_graph(work_order_type="requirement")
        self.assert_refused("WO-001 is a 'requirement', not a 'work_order'")

    def test_property_type_is_read_for_a_verification_record(self) -> None:
        self.fixture.write_graph(verification_type="verification")
        self.assert_refused("VREC-001 is a 'verification', not a 'verification_record'")

    def test_property_the_gate_is_idempotent(self) -> None:
        self.fixture.write_graph()
        first = self.fixture.run_gate()
        second = self.fixture.run_gate()
        self.assertEqual(first.returncode, second.returncode)
        self.assertEqual(first.stdout, second.stdout)

    def test_property_a_refusal_is_idempotent(self) -> None:
        self.fixture.write_graph(record_status="ready")
        first = self.fixture.run_gate()
        second = self.fixture.run_gate()
        self.assertEqual(first.returncode, second.returncode)
        self.assertEqual(first.stderr, second.stderr)

    # -- rule 4.2: the graph must be unambiguous -------------------------------------------

    def test_refuses_front_matter_that_does_not_parse(self) -> None:
        """A damaged release record must be named, not silently treated as absent.

        Skipping an unparseable file would turn this into `R6` -- "no release record exists
        at all" -- which is a true statement about the parsed graph and a false statement
        about the repository. That is the fail-open case rule 4.2 exists to close.
        """
        self.fixture.write_graph()
        (self.fixture.domain / "release" / "RLS-001.md").write_text(
            '+++\nid = "RLS-001"\nstatus = "released\n+++\n\nbody\n', encoding="utf-8"
        )
        refusal = self.assert_refused("RLS-001.md has front matter that does not parse")
        self.assertNotIn("No release record exists at all", refusal.stderr)

    def test_refuses_front_matter_that_is_never_closed(self) -> None:
        self.fixture.write_graph()
        (self.fixture.domain / "release" / "OPEN.md").write_text(
            '+++\nid = "OPEN-001"\ntype = "release_record"\n', encoding="utf-8"
        )
        self.assert_refused("OPEN.md opens `+++` front matter that is never closed")

    def test_ignores_prose_that_carries_no_front_matter(self) -> None:
        self.fixture.write_graph()
        (self.fixture.domain / "README.md").write_text(
            "# Domain\n\nOrdinary prose lives under the artifact root too.\n", encoding="utf-8"
        )
        self.assert_authorized()

    def test_ignores_files_under_an_excluded_directory(self) -> None:
        """Retained evidence and templates are not artifacts, exactly as the validator sees it.

        Without the exclusion this authorizing graph would refuse with a duplicate identifier,
        because evidence for a release quite reasonably quotes the record it describes.
        """
        self.fixture.write_graph()
        source = (self.fixture.domain / "release" / "RLS-001.md").read_text(encoding="utf-8")
        for excluded in ("evidence/WO-001", "templates"):
            directory = self.fixture.domain / excluded
            directory.mkdir(parents=True, exist_ok=True)
            (directory / "quoted.md").write_text(source, encoding="utf-8")
        self.assert_authorized()

    # -- the declared artifact root --------------------------------------------------------

    def test_reads_the_declared_artifact_root_rather_than_assuming_one(self) -> None:
        self.fixture.write_graph()
        relocated = self.fixture.root / "governance"
        shutil.move(str(self.fixture.artifact_root), str(relocated))
        self.fixture.artifact_root = relocated
        self.fixture.domain = relocated / "domain"
        self.fixture.set_config(artifact_root="governance")
        self.assert_authorized()

    def test_refuses_when_the_declared_artifact_root_is_missing(self) -> None:
        self.fixture.write_graph()
        self.fixture.set_config(artifact_root="somewhere-else")
        self.assert_refused("somewhere-else is missing: this is not a harnessed repository")

    def test_refuses_when_the_configuration_is_unreadable(self) -> None:
        self.fixture.write_graph()
        (self.fixture.root / ".engineering-harness.toml").unlink()
        self.assert_refused("cannot read .engineering-harness.toml")


class A5RealRepositoryTest(unittest.TestCase):
    """VER-MOK-008 A5: this repository, today, is an oracle nobody constructed.

    As long as no `released` release record exists, the correct answer for every tag is a
    refusal, and that answer is independent of both the gate and the fixtures above. The
    refusal ladder then adds the governance artifacts one at a time and records which refusal
    remains, which is the one measurement that shows the gate discriminating between adjacent
    incomplete states of a real graph.

    Everything runs in a throwaway clone. Creating a tag is an act `DECISION_RIGHTS.md`
    reserves to the release owner, so it happens in a copy that is deleted afterwards and
    never in the repository itself.
    """

    @classmethod
    def setUpClass(cls) -> None:
        if subprocess.run(
            ["git", "-C", str(REPOSITORY), "rev-parse", "--is-inside-work-tree"],
            capture_output=True,
            check=False,
        ).returncode != 0:
            raise unittest.SkipTest(f"{REPOSITORY} is not a git working tree")

    def setUp(self) -> None:
        self._temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self._temporary.cleanup)
        self.clone = Path(self._temporary.name) / "clone"
        subprocess.run(
            ["git", "clone", "--quiet", "--no-hardlinks", "--single-branch",
             str(REPOSITORY), str(self.clone)],
            capture_output=True,
            text=True,
            check=True,
        )
        git(self.clone, "config", "user.email", "fixture@example.invalid")
        git(self.clone, "config", "user.name", "Fixture Owner")
        self.commit = git(self.clone, "rev-parse", "HEAD")
        self.simulation = self.clone / "docs" / "engineering" / "simulation"
        self.work_orders = tuple(f"WO-MOK-{index:03d}" for index in range(1, 7))

    def refusal(self, tag: str = "v0.1.0") -> str:
        result = run_gate(self.clone, tag)
        self.assertEqual(result.returncode, 1, f"expected a refusal, got:\n{result.stdout}")
        return result.stderr

    def write(self, relative: str, front_matter: str) -> None:
        path = self.simulation / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        body = textwrap.dedent(front_matter).strip()
        path.write_text(f"+++\n{body}\n+++\n\nfixture body\n", encoding="utf-8")

    def add_release_contract(self) -> None:
        gated = ", ".join(f'"{identifier}"' for identifier in self.work_orders)
        self.write(
            "release/REL-MOK-001.md",
            f"""
            id = "REL-MOK-001"
            type = "release_contract"
            status = "approved"

            [relations]
            gates = [{gated}]
            """,
        )

    def add_aggregate_verification_record(self, *, status: str = "verified") -> None:
        covered = ", ".join(f'"{identifier}"' for identifier in self.work_orders)
        self.write(
            "verification-records/VREC-MOK-007.md",
            f"""
            id = "VREC-MOK-007"
            type = "verification_record"
            status = "{status}"
            commit = "{self.commit}"
            git_object_format = "sha1"

            [relations]
            verifies_work_order = [{covered}]
            conforms_to = ["VER-MOK-001"]
            """,
        )

    def add_release_record(self, *, status: str) -> None:
        released = ", ".join(f'"{identifier}"' for identifier in self.work_orders)
        self.write(
            "releases/RLS-MOK-001.md",
            f"""
            id = "RLS-MOK-001"
            type = "release_record"
            status = "{status}"
            version = "0.1.0"
            commit = "{self.commit}"
            git_object_format = "sha1"
            tag = "v0.1.0"

            [relations]
            satisfies = ["REL-MOK-001"]
            includes_verification = ["VREC-MOK-007"]
            releases_work = [{released}]
            """,
        )

    def test_a5_the_real_repository_refuses_every_tag_today(self) -> None:
        """No tag exists, so the first fact the gate cannot establish is the tag itself."""
        self.assertEqual(git(self.clone, "tag", "--list"), "", "the repository has no tags")
        self.assertIn("is not a tag in this repository", self.refusal())

    def test_a5_the_real_graph_holds_no_release_record(self) -> None:
        git(self.clone, "tag", "-a", "v0.1.0", "-m", "rehearsal 0.1.0")
        self.assertIn("No release record exists at all", self.refusal())

    def test_a5_the_refusal_ladder(self) -> None:
        """Which refusal remains after each governance artifact is added.

        The order below is Phase D through F of the release runbook. The last refusal to
        disappear is the release record's transition to `released`, which is the accountable
        act the whole process is downstream of.
        """
        ladder: list[tuple[str, str]] = []
        git(self.clone, "tag", "-a", "v0.1.0", "-m", "rehearsal 0.1.0")

        ladder.append(("annotated tag only", "No release record exists at all"))
        self.assertIn(ladder[-1][1], self.refusal())

        self.add_release_contract()
        ladder.append(("+ release contract", "No release record exists at all"))
        self.assertIn(ladder[-1][1], self.refusal())

        self.add_aggregate_verification_record()
        ladder.append(("+ aggregate verification record", "No release record exists at all"))
        self.assertIn(ladder[-1][1], self.refusal())

        self.add_release_record(status="ready")
        ladder.append(("+ release record, ready", "not yet transitioned to released: RLS-MOK-001"))
        self.assertIn(ladder[-1][1], self.refusal())

        self.add_release_record(status="released")
        result = run_gate(self.clone, "v0.1.0")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("AUTHORIZED", result.stdout)
        for identifier in (*self.work_orders, "REL-MOK-001", "VREC-MOK-007", self.commit):
            self.assertIn(identifier, result.stdout)

        # The rung that only the release owner can climb.
        self.assertEqual(ladder[-1][0], "+ release record, ready")

    def test_a5_a_ready_verification_record_still_refuses(self) -> None:
        git(self.clone, "tag", "-a", "v0.1.0", "-m", "rehearsal 0.1.0")
        self.add_release_contract()
        self.add_aggregate_verification_record(status="ready")
        self.add_release_record(status="released")
        self.assertIn("VREC-MOK-007 is 'ready'", self.refusal())

    def test_a5_the_six_existing_records_cannot_release_an_aggregate(self) -> None:
        """Each existing record binds its own work order's commit, and they differ.

        This is the fact Phase D of the runbook exists for, taken from the real graph rather
        than from a fixture: an aggregate release needs one new record at the final candidate
        commit, because the six cannot agree on one.
        """
        git(self.clone, "tag", "-a", "v0.1.0", "-m", "rehearsal 0.1.0")
        self.add_release_contract()
        included = ", ".join(f'"VREC-MOK-{index:03d}"' for index in range(1, 7))
        released = ", ".join(f'"{identifier}"' for identifier in self.work_orders)
        self.write(
            "releases/RLS-MOK-001.md",
            f"""
            id = "RLS-MOK-001"
            type = "release_record"
            status = "released"
            version = "0.1.0"
            commit = "{self.commit}"
            git_object_format = "sha1"
            tag = "v0.1.0"

            [relations]
            satisfies = ["REL-MOK-001"]
            includes_verification = [{included}]
            releases_work = [{released}]
            """,
        )
        self.assertIn(
            "one verification record captured at the final candidate commit", self.refusal()
        )

    def test_p4_the_gate_leaves_a_real_checkout_unchanged(self) -> None:
        before = (
            git(self.clone, "status", "--porcelain", "--untracked-files=all"),
            git(self.clone, "tag", "--list"),
            git(self.clone, "branch", "--list"),
            git(self.clone, "rev-parse", "HEAD"),
        )
        run_gate(self.clone, "v0.1.0")
        run_gate(self.clone, "v9.9.9")
        after = (
            git(self.clone, "status", "--porcelain", "--untracked-files=all"),
            git(self.clone, "tag", "--list"),
            git(self.clone, "branch", "--list"),
            git(self.clone, "rev-parse", "HEAD"),
        )
        self.assertEqual(before, after)


if __name__ == "__main__":
    unittest.main(verbosity=2)
