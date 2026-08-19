#!/usr/bin/env python3
"""Refuse to publish a release unless an accountable human already authorized it.

Repository-owned. This file is deliberately absent from `.engineering-harness.lock`:
`doctor` neither installs nor checks it, and it carries no managed-file integrity claim.

`ENGINEERING_HARNESS.md` forbids automation from exercising accountable decision
rights, and `DECISION_RIGHTS.md` reserves the transition to `released` for the release
owner. This script therefore decides nothing and authorizes nothing. It reads the
formal graph and answers one question:

    has the release owner already authorized publishing this exact tag,
    from this exact commit?

Every failure is a refusal to publish. It never edits an artifact, never creates a
tag, and never writes outside `$GITHUB_OUTPUT`.

This implements `SPEC-MOK-005` rule 3 and rule 4. Rule 5 -- whether the authorized
commit is reachable from a release-bearing branch -- is deliberately elsewhere, in
`check_release_reachability.py`: it is a property of the history rather than of the
graph, and keeping it out leaves this gate runnable in a clone with no remote.

The checks below deliberately mirror `se_harness.provenance.prepare_release`, which
enforced them when the record was *prepared*. They are re-checked here because a
release record is a text file, and because `prepare_release` ran before the tag
existed and so could not compare the two.

Usage:
    python scripts/check_release_authorization.py --root . --tag v0.1.0
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tomllib
from pathlib import Path
from typing import Any

# Mirrors se_harness.provenance. Kept as literals so a harness upgrade that widens a
# set cannot silently widen this gate.
ACTIVE_STATUSES = frozenset({"approved", "in_progress", "implemented", "verified", "released"})
RELEASABLE_WORK_STATUSES = frozenset({"implemented", "verified", "released"})
ELIGIBLE_VERIFICATION_STATUSES = frozenset({"verified", "released"})
COMMIT_LENGTHS = {"sha1": 40, "sha256": 64}
HEX = re.compile(r"[0-9a-f]+")
VERSION_PATTERN = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._+-]{0,63}$")

# Mirrors `EXCLUDED_DIRECTORY_NAMES` in `scripts/validate_engineering_artifacts.py`.
# SPEC-MOK-005 rule 4.2 requires the same exclusions here, so that a template or a
# retained evidence file is never mistaken for an artifact.
EXCLUDED_DIRECTORY_NAMES = frozenset(
    {"templates", "evidence", ".git", ".idea", "target", "node_modules"}
)

DEFAULT_ARTIFACT_ROOT = "docs/engineering"


class Refusal(Exception):
    """A reason to refuse publication."""


def read_front_matter(path: Path) -> dict[str, Any] | None:
    """Parse `+++`-delimited TOML front matter, exactly as se_harness does.

    Returns None when the file is not an artifact at all -- unreadable, empty, or with
    no `+++` opener -- because the artifact root also holds ordinary prose. Raises when
    a file *opens* front matter and then fails to parse: SPEC-MOK-005 rule 4.2 requires
    every artifact's front matter to parse, and silently skipping a malformed one would
    turn a corrupted release record into "no release record exists", which is a refusal
    for the wrong reason and reads as a governance failure rather than a damaged file.
    """
    try:
        lines = path.read_text(encoding="utf-8-sig").splitlines()
    except (OSError, UnicodeError):
        return None
    if not lines or lines[0].strip() != "+++":
        return None
    try:
        closing = lines.index("+++", 1)
    except ValueError:
        raise Refusal(f"{path.name} opens `+++` front matter that is never closed") from None
    try:
        metadata = tomllib.loads("\n".join(lines[1:closing]))
    except tomllib.TOMLDecodeError as exc:
        raise Refusal(f"{path.name} has front matter that does not parse: {exc}") from exc
    if not isinstance(metadata, dict):
        raise Refusal(f"{path.name} has front matter that is not a table")
    return metadata


def read_config(root: Path) -> dict[str, Any]:
    config = root / ".engineering-harness.toml"
    try:
        return tomllib.loads(config.read_text(encoding="utf-8-sig"))
    except (OSError, UnicodeError, tomllib.TOMLDecodeError) as exc:
        raise Refusal(f"cannot read .engineering-harness.toml: {exc}") from exc


def artifact_root(root: Path, config: dict[str, Any]) -> Path:
    """The artifact root the repository declares, not one this gate assumes."""
    harness = config.get("harness")
    declared = harness.get("artifact_root") if isinstance(harness, dict) else None
    relative = declared if isinstance(declared, str) and declared else DEFAULT_ARTIFACT_ROOT
    base = root / relative
    if not base.is_dir():
        raise Refusal(f"{relative} is missing: this is not a harnessed repository")
    return base


def require_full_commits(config: dict[str, Any]) -> None:
    """This gate compares complete hashes, so it refuses where abbreviations are allowed.

    SPEC-MOK-005 rule 4.3. Proceeding leniently would leave rule 4.6 -- the equality of
    the record's commit and the tagged commit -- comparing a prefix against a full hash,
    which is the one check the whole gate exists to perform.
    """
    provenance = config.get("revision_provenance")
    required = True
    if isinstance(provenance, dict):
        required = bool(provenance.get("require_full_commit", True))
    if not required:
        raise Refusal(
            ".engineering-harness.toml sets revision_provenance.require_full_commit = false. "
            "This gate compares complete commit hashes and will not run where abbreviated "
            "commits are permitted."
        )


def load_artifacts(base: Path, root: Path) -> dict[str, dict[str, Any]]:
    """Every formal artifact under the artifact root, keyed by ID."""
    artifacts: dict[str, dict[str, Any]] = {}
    for path in sorted(base.rglob("*.md")):
        relative = path.relative_to(base)
        if EXCLUDED_DIRECTORY_NAMES.intersection(relative.parts[:-1]):
            continue
        metadata = read_front_matter(path)
        if metadata is None:
            continue
        identifier = metadata.get("id")
        if not isinstance(identifier, str) or not identifier:
            continue
        if identifier in artifacts:
            raise Refusal(
                f"duplicate artifact ID {identifier}: "
                f"{artifacts[identifier]['__path']} and {path.relative_to(root).as_posix()}"
            )
        metadata["__path"] = path.relative_to(root).as_posix()
        artifacts[identifier] = metadata
    return artifacts


def relation(metadata: dict[str, Any], name: str) -> set[str]:
    relations = metadata.get("relations")
    if not isinstance(relations, dict):
        return set()
    value = relations.get(name, [])
    if not isinstance(value, list):
        return set()
    return {item for item in value if isinstance(item, str)}


def require(metadata: dict[str, Any] | None, identifier: str, expected_type: str) -> dict[str, Any]:
    """A relation's name is not evidence about its target. The target's own type is."""
    if metadata is None:
        raise Refusal(f"{identifier} does not exist in the formal graph")
    if metadata.get("type") != expected_type:
        raise Refusal(f"{identifier} is a {metadata.get('type')!r}, not a {expected_type!r}")
    return metadata


def full_commit(metadata: dict[str, Any], identifier: str) -> str:
    commit = metadata.get("commit")
    object_format = metadata.get("git_object_format")
    if not isinstance(commit, str) or not commit:
        raise Refusal(f"{identifier} carries no commit")
    expected = COMMIT_LENGTHS.get(object_format if isinstance(object_format, str) else "")
    if expected is None:
        raise Refusal(f"{identifier} declares an unsupported git_object_format: {object_format!r}")
    if len(commit) != expected or HEX.fullmatch(commit) is None:
        raise Refusal(
            f"{identifier} does not carry a full {object_format} commit "
            f"(expected {expected} lowercase hex characters, got {len(commit)})"
        )
    return commit


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


def resolve_tag(root: Path, tag: str) -> str:
    """The commit this tag points at. Resolution is confined to `refs/tags/`."""
    completed = git(root, "rev-parse", "--verify", "--quiet", f"refs/tags/{tag}^{{commit}}")
    commit = completed.stdout.strip()
    if completed.returncode != 0 or not commit:
        raise Refusal(
            f"{tag} is not a tag in this repository. "
            "The release owner creates the tag; this workflow never does."
        )
    return commit


def require_annotated_tag(root: Path, tag: str) -> None:
    """SPEC-MOK-005 rule 3. An annotated tag records who tagged and when; a lightweight
    tag records neither, so it carries no evidence that a person created it."""
    completed = git(root, "cat-file", "-t", f"refs/tags/{tag}")
    object_type = completed.stdout.strip()
    if completed.returncode != 0 or not object_type:
        raise Refusal(f"cannot read the git object for tag {tag}")
    if object_type != "tag":
        raise Refusal(
            f"{tag} is a lightweight tag (git object type: {object_type}). A release tag "
            "must be annotated, so that the tagger identity and date are recorded."
        )


def select_record(artifacts: dict[str, dict[str, Any]], tag: str) -> dict[str, Any]:
    """The single `released` release record that claims this tag."""
    released = [
        metadata
        for metadata in artifacts.values()
        if metadata.get("type") == "release_record" and metadata.get("status") == "released"
    ]
    if not released:
        prepared = [
            metadata.get("id")
            for metadata in artifacts.values()
            if metadata.get("type") == "release_record"
        ]
        detail = (
            f" Found {len(prepared)} release record(s) not yet transitioned to released: "
            f"{', '.join(sorted(str(item) for item in prepared))}."
            if prepared
            else " No release record exists at all."
        )
        raise Refusal(
            "no release record has status 'released'. Only the release owner may make that "
            "transition; this workflow will not publish without it." + detail
        )

    declared = [metadata for metadata in released if metadata.get("tag") == tag]
    if declared:
        matches = declared
    else:
        # A release record's `tag` field is optional. When absent, fall back to the
        # conventional v<version> tag. A record that declares a *different* tag is not
        # a match under either rule, so a mismatched tag cannot slip through.
        matches = [
            metadata
            for metadata in released
            if metadata.get("tag") is None
            and isinstance(metadata.get("version"), str)
            and f"v{metadata['version']}" == tag
        ]

    if len(matches) != 1:
        raise Refusal(
            f"expected exactly one released release record for tag {tag}, found {len(matches)}: "
            f"{', '.join(sorted(str(item.get('id')) for item in matches)) or 'none'}"
        )
    return matches[0]


def authorize(root: Path, tag: str) -> dict[str, str]:
    config = read_config(root)
    require_full_commits(config)
    base = artifact_root(root, config)

    tag_commit = resolve_tag(root, tag)
    require_annotated_tag(root, tag)
    artifacts = load_artifacts(base, root)

    record = select_record(artifacts, tag)
    record_id = str(record.get("id"))
    record_commit = full_commit(record, record_id)

    # The load-bearing check: the tag the operator pushed points at the commit the
    # release owner actually authorized. Everything else is downstream of this.
    if record_commit != tag_commit:
        raise Refusal(
            f"{record_id} authorizes commit {record_commit}, but tag {tag} points at "
            f"{tag_commit}. Refusing to publish assets the release owner did not approve."
        )

    version = record.get("version")
    if not isinstance(version, str) or VERSION_PATTERN.fullmatch(version) is None:
        raise Refusal(f"{record_id} carries an unusable version: {version!r}")

    released_work = relation(record, "releases_work")
    if not released_work:
        raise Refusal(f"{record_id} releases no work order")

    contract_ids = relation(record, "satisfies")
    if len(contract_ids) != 1:
        raise Refusal(f"{record_id} must satisfy exactly one release contract, found {len(contract_ids)}")
    contract_id = next(iter(contract_ids))
    contract = require(artifacts.get(contract_id), contract_id, "release_contract")
    if contract.get("status") not in ACTIVE_STATUSES:
        raise Refusal(f"release contract {contract_id} is {contract.get('status')!r}, not active")
    ungated = released_work - relation(contract, "gates")
    if ungated:
        raise Refusal(
            f"release contract {contract_id} does not gate: {', '.join(sorted(ungated))}"
        )

    verification_ids = relation(record, "includes_verification")
    if not verification_ids:
        raise Refusal(f"{record_id} includes no verification record")

    covered_work: set[str] = set()
    commits: set[str] = set()
    for verification_id in sorted(verification_ids):
        verification = require(artifacts.get(verification_id), verification_id, "verification_record")
        status = verification.get("status")
        if status not in ELIGIBLE_VERIFICATION_STATUSES:
            raise Refusal(
                f"verification record {verification_id} is {status!r}. A release requires "
                "'verified' or 'released'; a 'ready' record is a proposal the assurance "
                "owner has not accepted."
            )
        commits.add(full_commit(verification, verification_id))
        covered_work |= relation(verification, "verifies_work_order")

    if commits != {record_commit}:
        raise Refusal(
            f"included verification records bind {sorted(commits)}, which is not exactly "
            f"the released commit {record_commit}. An aggregate release needs one "
            "verification record captured at the final candidate commit."
        )

    if covered_work != released_work:
        unverified = released_work - covered_work
        unreleased = covered_work - released_work
        details = []
        if unverified:
            details.append(f"released but not verified: {', '.join(sorted(unverified))}")
        if unreleased:
            details.append(f"verified but not released: {', '.join(sorted(unreleased))}")
        raise Refusal("released work does not match verification coverage; " + "; ".join(details))

    for work_order_id in sorted(released_work):
        work_order = require(artifacts.get(work_order_id), work_order_id, "work_order")
        if work_order.get("status") not in RELEASABLE_WORK_STATUSES:
            raise Refusal(
                f"work order {work_order_id} is {work_order.get('status')!r}; "
                "a released work order must be implemented, verified, or released"
            )

    return {
        "commit": record_commit,
        "version": version,
        "record": record_id,
        "contract": contract_id,
        "work_orders": " ".join(sorted(released_work)),
        "verification_records": " ".join(sorted(verification_ids)),
    }


def emit(outputs: dict[str, str]) -> None:
    destination = os.environ.get("GITHUB_OUTPUT")
    if destination:
        with open(destination, "a", encoding="utf-8") as handle:
            for key, value in outputs.items():
                handle.write(f"{key}={value}\n")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", default=".", help="repository root")
    parser.add_argument("--tag", required=True, help="the tag under release")
    arguments = parser.parse_args(argv)

    root = Path(arguments.root).resolve()
    try:
        outputs = authorize(root, arguments.tag)
    except Refusal as refusal:
        print(f"REFUSED: {refusal}", file=sys.stderr)
        print(
            "\nThis gate does not grant authority. It reports that the accountable "
            "release decision recorded under the artifact root does not authorize "
            "publishing this tag.",
            file=sys.stderr,
        )
        return 1

    print(f"AUTHORIZED release {outputs['version']} of {outputs['record']}")
    for label, key in (
        ("candidate commit", "commit"),
        ("release contract", "contract"),
        ("released work", "work_orders"),
        ("included verification", "verification_records"),
    ):
        print(f"  {label:<21} {outputs[key]}")
    emit(outputs)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
