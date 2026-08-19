#!/usr/bin/env python3
"""Scenario suite for `check_release_reachability.py`.

Repository-owned, absent from `.engineering-harness.lock`.

`VER-MOK-008` scenarios **R23** and **R24** live here rather than in the authorization gate's
suite, because `SPEC-MOK-005` rule 5 places reachability outside the gate: it is a property of
the history, not of the artifact graph. `VER-MOK-008` says either component may carry them, so
long as both refuse.

The fixtures build remote-tracking references with `git update-ref` rather than by talking to a
server. That is the same namespace a clone populates, so the check cannot tell the difference,
and no test needs a network.

    python scripts/test_check_release_reachability.py
    python -m unittest discover -s scripts -p "test_*.py"
"""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

CHECK = Path(__file__).resolve().parent / "check_release_reachability.py"

ABSENT_COMMIT = "0" * 40


def git(root: Path, *arguments: str) -> str:
    completed = subprocess.run(
        ["git", "-C", str(root), *arguments],
        capture_output=True,
        text=True,
        check=True,
    )
    return completed.stdout.strip()


class Fixture:
    """A repository whose `refs/remotes/origin/*` namespace is built by hand.

    `master` is the default branch and `origin/HEAD` points at it, which is what a clone of a
    normal repository looks like. Every other reference is added by the test that needs it.
    """

    def __init__(self, root: Path) -> None:
        self.root = root
        git(root, "init", "--quiet", "--initial-branch=master")
        git(root, "config", "user.email", "fixture@example.invalid")
        git(root, "config", "user.name", "Fixture Owner")
        self.base = self.commit("base")
        self.remote_ref("refs/remotes/origin/master", self.base)
        git(root, "symbolic-ref", "refs/remotes/origin/HEAD", "refs/remotes/origin/master")

    def commit(self, name: str) -> str:
        (self.root / f"{name}.txt").write_text(f"{name}\n", encoding="utf-8")
        git(self.root, "add", "-A")
        git(self.root, "commit", "--quiet", "-m", name)
        return git(self.root, "rev-parse", "HEAD")

    def remote_ref(self, reference: str, commit: str) -> None:
        git(self.root, "update-ref", reference, commit)

    def branch_from_base(self, branch: str, name: str) -> str:
        """A local branch off the base commit, with one commit on it. Returns its tip."""
        git(self.root, "switch", "--quiet", "--create", branch, self.base)
        tip = self.commit(name)
        git(self.root, "switch", "--quiet", "master")
        return tip

    def snapshot(self) -> tuple[str, ...]:
        return (
            git(self.root, "status", "--porcelain", "--untracked-files=all"),
            git(self.root, "tag", "--list"),
            git(self.root, "branch", "--list"),
            git(self.root, "for-each-ref", "--format=%(refname) %(objectname)"),
            git(self.root, "rev-parse", "HEAD"),
        )

    def run_check(self, commit: str, *extra: str, **environment: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(CHECK), "--root", str(self.root), "--commit", commit, *extra],
            capture_output=True,
            text=True,
            check=False,
            env={**os.environ, **environment} if environment else None,
        )


class ReachabilityTest(unittest.TestCase):
    def setUp(self) -> None:
        self._temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self._temporary.cleanup)
        self.fixture = Fixture(Path(self._temporary.name))

    def assert_reachable(self, commit: str, expected_branch: str, *extra: str) -> None:
        before = self.fixture.snapshot()
        result = self.fixture.run_check(commit, *extra)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("REACHABLE", result.stdout)
        self.assertIn(expected_branch, result.stdout)
        self.assertEqual(before, self.fixture.snapshot(), "the check changed the repository")

    def assert_refused(self, commit: str, expected: str, *extra: str) -> subprocess.CompletedProcess[str]:
        before = self.fixture.snapshot()
        result = self.fixture.run_check(commit, *extra)
        self.assertEqual(result.returncode, 1, f"expected a refusal, got:\n{result.stdout}")
        self.assertIn("REFUSED", result.stderr)
        self.assertIn(expected, result.stderr)
        self.assertEqual(before, self.fixture.snapshot(), "the check changed the repository")
        return result

    # -- accepted --------------------------------------------------------------------------

    def test_accepts_a_commit_on_the_default_branch(self) -> None:
        self.assert_reachable(self.fixture.base, "refs/remotes/origin/master")

    def test_accepts_a_commit_on_a_maintenance_branch(self) -> None:
        tip = self.fixture.branch_from_base("release/0.1", "stabilization")
        self.fixture.remote_ref("refs/remotes/origin/release/0.1", tip)
        self.assert_reachable(tip, "refs/remotes/origin/release/0.1")

    def test_accepts_a_commit_a_maintenance_branch_merely_contains(self) -> None:
        # Reachability, not tip equality: the release branch was cut and then moved on.
        tip = self.fixture.branch_from_base("release/0.1", "stabilization")
        self.fixture.remote_ref("refs/remotes/origin/release/0.1", tip)
        self.fixture.remote_ref("refs/remotes/origin/master", self.fixture.base)
        self.assert_reachable(self.fixture.base, "refs/remotes/origin/master")

    def test_accepts_a_maintenance_branch_nobody_named_in_advance(self) -> None:
        """The branch list is enumerated from the remote, so `release/9.9` needs no edit here."""
        tip = self.fixture.branch_from_base("release/9.9", "future line")
        self.fixture.remote_ref("refs/remotes/origin/release/9.9", tip)
        self.assert_reachable(tip, "refs/remotes/origin/release/9.9")

    def test_prefers_the_default_branch_when_both_contain_the_commit(self) -> None:
        tip = self.fixture.branch_from_base("release/0.1", "stabilization")
        self.fixture.remote_ref("refs/remotes/origin/release/0.1", tip)
        self.fixture.remote_ref("refs/remotes/origin/master", tip)
        self.assert_reachable(tip, "refs/remotes/origin/master")

    def test_reads_the_default_branch_from_the_remotes_own_head(self) -> None:
        # No --default-branch is passed; origin/HEAD is the only source.
        self.assert_reachable(self.fixture.base, "refs/remotes/origin/master")

    def test_honours_an_explicitly_declared_default_branch(self) -> None:
        """The workflow passes the platform's declared default branch rather than guessing."""
        self.fixture.remote_ref("refs/remotes/origin/main", self.fixture.base)
        git(self.fixture.root, "update-ref", "-d", "refs/remotes/origin/master")
        git(self.fixture.root, "symbolic-ref", "--delete", "refs/remotes/origin/HEAD")
        self.assert_reachable(
            self.fixture.base, "refs/remotes/origin/main", "--default-branch", "main"
        )

    def test_emits_the_containing_branch_for_the_workflow(self) -> None:
        destination = Path(self._temporary.name) / "outputs.txt"
        result = self.fixture.run_check(self.fixture.base, GITHUB_OUTPUT=str(destination))
        self.assertEqual(result.returncode, 0, result.stderr)
        emitted = dict(
            line.split("=", 1) for line in destination.read_text(encoding="utf-8").splitlines()
        )
        self.assertEqual(emitted["branch"], "refs/remotes/origin/master")
        self.assertEqual(emitted["commit"], self.fixture.base)

    # -- VER-MOK-008 R23 and R24 -----------------------------------------------------------

    def test_r23_the_authorized_commit_is_reachable_only_from_a_feature_branch(self) -> None:
        """Otherwise any commit anyone pushed could be released, given a tag."""
        tip = self.fixture.branch_from_base("feature/x", "feature work")
        self.fixture.remote_ref("refs/remotes/origin/feature/x", tip)
        refusal = self.assert_refused(tip, "is not contained by any release-bearing branch")
        self.assertIn("refs/remotes/origin/master", refusal.stderr)
        self.assertNotIn("feature/x", refusal.stderr)

    def test_r24_the_authorized_commit_is_reachable_from_no_remote_branch(self) -> None:
        """A commit that exists only locally, with the remote still at the base commit."""
        local = self.fixture.commit("unpushed")
        self.assertNotEqual(local, self.fixture.base)
        self.assert_refused(local, "is not contained by any release-bearing branch")

    def test_r23_a_local_release_branch_does_not_count(self) -> None:
        """Local branches are not consulted: a recipient cannot find a commit that never left."""
        tip = self.fixture.branch_from_base("release/0.1", "stabilization")
        self.assertEqual(git(self.fixture.root, "rev-parse", "release/0.1"), tip)
        self.assert_refused(tip, "is not contained by any release-bearing branch")

    def test_names_every_branch_it_examined(self) -> None:
        release = self.fixture.branch_from_base("release/0.1", "stabilization")
        self.fixture.remote_ref("refs/remotes/origin/release/0.1", release)
        unpushed = self.fixture.commit("unpushed")
        refusal = self.assert_refused(unpushed, "Examined:")
        self.assertIn("refs/remotes/origin/master", refusal.stderr)
        self.assertIn("refs/remotes/origin/release/0.1", refusal.stderr)

    # -- inputs the check will not resolve --------------------------------------------------

    def test_refuses_an_abbreviated_commit(self) -> None:
        self.assert_refused(self.fixture.base[:12], "is not a complete lowercase commit hash")

    def test_refuses_a_revision_name_rather_than_a_hash(self) -> None:
        # `HEAD` is a perfectly good revision and exactly what must not be accepted here.
        self.assert_refused("HEAD", "is not a complete lowercase commit hash")

    def test_refuses_an_uppercase_commit(self) -> None:
        self.assert_refused(self.fixture.base.upper(), "is not a complete lowercase commit hash")

    def test_refuses_a_commit_that_is_not_present(self) -> None:
        self.assert_refused(ABSENT_COMMIT, "is not present in this repository")

    # -- references the check needs and will not create ------------------------------------

    def test_refuses_when_the_clone_holds_no_release_bearing_reference(self) -> None:
        git(self.fixture.root, "update-ref", "-d", "refs/remotes/origin/master")
        self.assert_refused(
            self.fixture.base, "no release-bearing branch can be examined", "--default-branch", "master"
        )

    def test_refuses_when_the_default_branch_cannot_be_determined(self) -> None:
        git(self.fixture.root, "symbolic-ref", "--delete", "refs/remotes/origin/HEAD")
        self.assert_refused(self.fixture.base, "cannot determine the default branch")

    def test_refuses_a_remote_that_has_no_references(self) -> None:
        self.assert_refused(
            self.fixture.base,
            "no release-bearing branch can be examined",
            "--remote",
            "upstream",
            "--default-branch",
            "master",
        )

    # -- properties ------------------------------------------------------------------------

    def test_p4_the_check_writes_nothing(self) -> None:
        (self.fixture.root / "untracked.txt").write_text("untracked\n", encoding="utf-8")
        before = self.fixture.snapshot()
        self.fixture.run_check(self.fixture.base)
        self.fixture.run_check(ABSENT_COMMIT)
        self.assertEqual(before, self.fixture.snapshot())
        self.assertEqual(
            (self.fixture.root / "untracked.txt").read_text(encoding="utf-8"), "untracked\n"
        )

    def test_property_the_check_is_idempotent(self) -> None:
        first = self.fixture.run_check(self.fixture.base)
        second = self.fixture.run_check(self.fixture.base)
        self.assertEqual((first.returncode, first.stdout), (second.returncode, second.stdout))

    def test_property_the_check_fetches_nothing(self) -> None:
        """A missing reference is a refusal, not a reason to go and get it.

        Pointing the remote at a path that does not exist would make any fetch fail loudly;
        the check must reach its refusal without attempting one.
        """
        git(self.fixture.root, "remote", "add", "origin", str(self.fixture.root / "nowhere"))
        local = self.fixture.commit("unpushed")
        self.assert_refused(local, "is not contained by any release-bearing branch")


if __name__ == "__main__":
    unittest.main(verbosity=2)
