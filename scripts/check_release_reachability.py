#!/usr/bin/env python3
"""Refuse to publish a commit that no release-bearing branch contains.

Repository-owned, absent from `.engineering-harness.lock`, decides nothing, writes
nothing outside `$GITHUB_OUTPUT`.

This implements `SPEC-MOK-005` rule 5, and it is deliberately a separate program from
`check_release_authorization.py`. Rule 5 is a property of the *history*, not of the
artifact graph: a release record can be perfectly valid while naming a commit that
only ever existed on an abandoned feature branch. Keeping the two apart also leaves the
authorization gate runnable in a clone with no remote, which is how a developer
rehearses it.

A release-bearing branch is the default branch, or any `release/*` branch. Both are
enumerated from the remote's own references rather than from a list written here, so
cutting `release/0.2` needs no change to this file. Local branches are not consulted:
a commit reachable only from a local branch is not something a recipient can find.

`ENGINEERING_HARNESS.md` reserves branch creation to accountable humans. This program
creates no branch, fetches nothing, and moves no reference. If the reference it needs
is absent, that is a refusal, not something to go and fetch.

Usage:
    python scripts/check_release_reachability.py --root . --commit <full-sha>
    python scripts/check_release_reachability.py --root . --commit <full-sha> \
        --default-branch master --remote origin
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path

COMMIT_PATTERN = re.compile(r"^(?:[0-9a-f]{40}|[0-9a-f]{64})$")


class Refusal(Exception):
    """A reason to refuse publication."""


def git(root: Path, *arguments: str) -> subprocess.CompletedProcess[str]:
    try:
        return subprocess.run(
            ["git", "-C", str(root), *arguments],
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError as exc:
        raise Refusal(f"cannot run git: {exc}") from exc


def require_full_commit(commit: str) -> str:
    """A prefix, a branch name or `HEAD` is refused rather than resolved.

    The authorization gate emits a complete hash taken from the release record. Anything
    else reaching this program means the two are no longer talking about the same input,
    and resolving it here would hide that.
    """
    if COMMIT_PATTERN.fullmatch(commit) is None:
        raise Refusal(
            f"{commit!r} is not a complete lowercase commit hash. This check compares the "
            "commit the release record authorizes, which is always full length; it does not "
            "resolve names or abbreviations."
        )
    return commit


def require_commit_present(root: Path, commit: str) -> None:
    completed = git(root, "cat-file", "-e", f"{commit}^{{commit}}")
    if completed.returncode != 0:
        raise Refusal(
            f"commit {commit} is not present in this repository. A shallow or partial clone "
            "cannot establish reachability; fetch the full history instead of assuming it."
        )


def default_branch_name(root: Path, declared: str | None, remote: str) -> str:
    """The default branch, as declared by the caller or as the remote records it."""
    if declared:
        return declared
    completed = git(root, "symbolic-ref", "--quiet", f"refs/remotes/{remote}/HEAD")
    reference = completed.stdout.strip()
    prefix = f"refs/remotes/{remote}/"
    if completed.returncode == 0 and reference.startswith(prefix):
        return reference[len(prefix) :]
    raise Refusal(
        f"cannot determine the default branch: {remote}/HEAD is not set in this clone. "
        "Pass --default-branch explicitly rather than letting the check guess."
    )


def release_bearing_branches(root: Path, default_branch: str, remote: str) -> list[str]:
    """Every reference a release may be published from, in check order."""
    candidates: list[str] = []

    default_reference = f"refs/remotes/{remote}/{default_branch}"
    if git(root, "rev-parse", "--verify", "--quiet", default_reference).returncode == 0:
        candidates.append(default_reference)

    completed = git(
        root,
        "for-each-ref",
        "--format=%(refname)",
        f"refs/remotes/{remote}/release/*",
    )
    if completed.returncode != 0:
        raise Refusal(f"cannot enumerate {remote} release branches: {completed.stderr.strip()}")
    candidates.extend(line.strip() for line in completed.stdout.splitlines() if line.strip())

    if not candidates:
        raise Refusal(
            f"this clone holds no {remote}/{default_branch} reference and no "
            f"{remote}/release/* reference, so no release-bearing branch can be examined. "
            "Reachability is established against the remote's own references; it is not "
            "inferred from local branches."
        )
    return candidates


def reachable_from(root: Path, commit: str, reference: str) -> bool:
    return git(root, "merge-base", "--is-ancestor", commit, reference).returncode == 0


def check(root: Path, commit: str, declared_default: str | None, remote: str) -> dict[str, str]:
    require_full_commit(commit)
    require_commit_present(root, commit)
    default_branch = default_branch_name(root, declared_default, remote)
    candidates = release_bearing_branches(root, default_branch, remote)

    for reference in candidates:
        if reachable_from(root, commit, reference):
            return {"branch": reference, "commit": commit}

    raise Refusal(
        f"commit {commit} is not contained by any release-bearing branch. Examined: "
        f"{', '.join(candidates)}. A release is cut from the default branch or from a "
        "maintenance branch; a commit reachable only from a feature branch, or from no "
        "pushed branch at all, is not releasable."
    )


def emit(outputs: dict[str, str]) -> None:
    destination = os.environ.get("GITHUB_OUTPUT")
    if destination:
        with open(destination, "a", encoding="utf-8") as handle:
            for key, value in outputs.items():
                handle.write(f"{key}={value}\n")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", default=".", help="repository root")
    parser.add_argument("--commit", required=True, help="the authorized commit, full length")
    parser.add_argument(
        "--default-branch",
        default=None,
        help="the integration branch; read from <remote>/HEAD when omitted",
    )
    parser.add_argument("--remote", default="origin", help="the remote whose references are read")
    arguments = parser.parse_args(argv)

    root = Path(arguments.root).resolve()
    try:
        outputs = check(root, arguments.commit, arguments.default_branch, arguments.remote)
    except Refusal as refusal:
        print(f"REFUSED: {refusal}", file=sys.stderr)
        return 1

    print(f"REACHABLE commit {outputs['commit']}")
    print(f"  contained by         {outputs['branch']}")
    emit(outputs)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
