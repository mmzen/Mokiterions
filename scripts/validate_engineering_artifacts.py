#!/usr/bin/env python3
"""Validate specification-driven engineering artifacts.

The validator intentionally uses only the Python 3.11+ standard library so it can
run before the repository's normal toolchain is available.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import asdict, dataclass
from datetime import datetime
from pathlib import Path
from typing import Any, Iterable

try:
    import tomllib
except ModuleNotFoundError as exc:  # pragma: no cover - version guard
    raise SystemExit("Python 3.11 or later is required (missing tomllib).") from exc


ALLOWED_STATUSES = {
    "draft",
    "ready",
    "approved",
    "in_progress",
    "implemented",
    "verified",
    "released",
    "superseded",
    "rejected",
}

ACTIVE_COVERAGE_STATUSES = {
    "approved",
    "in_progress",
    "implemented",
    "verified",
    "released",
}

TYPE_PREFIX = {
    "intent": "INT-",
    "capability": "CAP-",
    "requirement": "REQ-",
    "specification": "SPEC-",
    "architecture": "ARCH-",
    "adr": "ADR-",
    "verification": "VER-",
    "work_order": "WO-",
    "release_contract": "REL-",
    "verification_record": "VREC-",
    "release_record": "RLS-",
    "operating_contract": "OPS-",
    "risk_acceptance": "RISK-",
}

ID_PATTERN = re.compile(r"^[A-Z][A-Z0-9-]*-\d{3}$")
ISO_DATE_PATTERN = re.compile(r"^\d{4}-\d{2}-\d{2}$")
SHA256_PATTERN = re.compile(r"^[0-9a-f]{64}$")
GIT_COMMIT_PATTERNS = {
    "sha1": re.compile(r"^[0-9a-f]{40}$"),
    "sha256": re.compile(r"^[0-9a-f]{64}$"),
}

RELEASABLE_WORK_STATUSES = {
    "implemented",
    "verified",
    "released",
}
EXCLUDED_DIRECTORY_NAMES = {"templates", "evidence", ".git", ".idea", "target", "node_modules"}

PROVENANCE_RELATION_TARGET_TYPES: dict[tuple[str, str], set[str]] = {
    ("verification_record", "verifies_work_order"): {"work_order"},
    ("verification_record", "conforms_to"): {"verification"},
    ("verification_record", "superseded_by"): {"verification_record"},
    ("release_record", "satisfies"): {"release_contract"},
    ("release_record", "includes_verification"): {"verification_record"},
    ("release_record", "releases_work"): {"work_order"},
}


@dataclass(frozen=True, order=True)
class Diagnostic:
    path: str
    code: str
    message: str


@dataclass
class Artifact:
    path: Path
    metadata: dict[str, Any]
    body: str

    @property
    def artifact_id(self) -> str:
        value = self.metadata.get("id")
        return value if isinstance(value, str) else "<unknown>"

    @property
    def artifact_type(self) -> str:
        value = self.metadata.get("type")
        return value if isinstance(value, str) else "<unknown>"

    @property
    def status(self) -> str:
        value = self.metadata.get("status")
        return value if isinstance(value, str) else "<unknown>"

    @property
    def relations(self) -> dict[str, Any]:
        value = self.metadata.get("relations", {})
        return value if isinstance(value, dict) else {}


@dataclass
class ValidationReport:
    artifacts: list[Artifact]
    errors: list[Diagnostic]
    warnings: list[Diagnostic]

    @property
    def valid(self) -> bool:
        return not self.errors

    def to_dict(self, root: Path) -> dict[str, Any]:
        def relative(path: Path) -> str:
            try:
                return path.resolve().relative_to(root.resolve()).as_posix()
            except ValueError:
                return path.as_posix()

        return {
            "valid": self.valid,
            "artifact_count": len(self.artifacts),
            "error_count": len(self.errors),
            "warning_count": len(self.warnings),
            "errors": [asdict(item) for item in sorted(self.errors)],
            "warnings": [asdict(item) for item in sorted(self.warnings)],
            "artifacts": [
                {
                    "id": artifact.artifact_id,
                    "type": artifact.artifact_type,
                    "status": artifact.status,
                    "path": relative(artifact.path),
                }
                for artifact in sorted(self.artifacts, key=lambda item: (item.artifact_id, item.path.as_posix()))
            ],
        }


def load_revision_policy(repository_root: Path) -> dict[str, bool]:
    defaults = {"required_for_verified_work": False, "required_for_release": False}
    path = repository_root / ".engineering-harness.toml"
    if not path.is_file():
        return defaults
    try:
        metadata = tomllib.loads(path.read_text(encoding="utf-8-sig"))
    except (OSError, UnicodeError, tomllib.TOMLDecodeError):
        return defaults
    policy = metadata.get("revision_provenance", {})
    if not isinstance(policy, dict):
        return defaults
    return {
        key: value if isinstance((value := policy.get(key)), bool) else default
        for key, default in defaults.items()
    }


def _display_path(path: Path, root: Path) -> str:
    try:
        return path.resolve().relative_to(root.resolve()).as_posix()
    except ValueError:
        return path.as_posix()


def _is_excluded(path: Path, artifact_root: Path) -> bool:
    try:
        relative_parts = path.relative_to(artifact_root).parts
    except ValueError:
        relative_parts = path.parts
    return any(part in EXCLUDED_DIRECTORY_NAMES for part in relative_parts[:-1])


def discover_candidate_files(artifact_root: Path) -> list[Path]:
    if not artifact_root.exists():
        return []
    return sorted(
        path
        for path in artifact_root.rglob("*.md")
        if path.is_file() and not _is_excluded(path, artifact_root)
    )


def parse_formal_artifact(path: Path, report_root: Path) -> tuple[Artifact | None, Diagnostic | None]:
    try:
        text = path.read_text(encoding="utf-8-sig")
    except (OSError, UnicodeError) as exc:
        return None, Diagnostic(_display_path(path, report_root), "E001", f"cannot read artifact: {exc}")

    if not text.startswith("+++\n") and text != "+++":
        return None, None

    lines = text.splitlines()
    try:
        closing_index = lines.index("+++", 1)
    except ValueError:
        return None, Diagnostic(
            _display_path(path, report_root),
            "E001",
            "formal artifact starts TOML front matter but has no closing +++ delimiter",
        )

    front_matter_text = "\n".join(lines[1:closing_index])
    body = "\n".join(lines[closing_index + 1 :]).lstrip("\n")

    try:
        metadata = tomllib.loads(front_matter_text)
    except tomllib.TOMLDecodeError as exc:
        return None, Diagnostic(
            _display_path(path, report_root),
            "E001",
            f"invalid TOML front matter: {exc}",
        )

    if not isinstance(metadata, dict):
        return None, Diagnostic(_display_path(path, report_root), "E001", "front matter must be a TOML table")

    return Artifact(path=path, metadata=metadata, body=body), None


def load_artifacts(artifact_root: Path, report_root: Path) -> tuple[list[Artifact], list[Diagnostic]]:
    artifacts: list[Artifact] = []
    errors: list[Diagnostic] = []
    for path in discover_candidate_files(artifact_root):
        artifact, error = parse_formal_artifact(path, report_root)
        if error is not None:
            errors.append(error)
        elif artifact is not None:
            artifacts.append(artifact)
    return artifacts, errors


def _add_error(errors: list[Diagnostic], artifact: Artifact, report_root: Path, code: str, message: str) -> None:
    errors.append(Diagnostic(_display_path(artifact.path, report_root), code, message))


def _require_non_empty_string(
    artifact: Artifact,
    field: str,
    errors: list[Diagnostic],
    report_root: Path,
) -> str | None:
    value = artifact.metadata.get(field)
    if not isinstance(value, str) or not value.strip():
        _add_error(errors, artifact, report_root, "E002", f"field '{field}' must be a non-empty string")
        return None
    return value.strip()


def _require_non_empty_string_list(
    artifact: Artifact,
    field: str,
    errors: list[Diagnostic],
    report_root: Path,
    *,
    code: str = "E002",
    container: dict[str, Any] | None = None,
) -> list[str] | None:
    source = artifact.metadata if container is None else container
    value = source.get(field)
    if not isinstance(value, list) or not value or any(not isinstance(item, str) or not item.strip() for item in value):
        _add_error(errors, artifact, report_root, code, f"field '{field}' must be a non-empty array of strings")
        return None
    return [item.strip() for item in value]


def _validate_git_identity(
    artifact: Artifact,
    errors: list[Diagnostic],
    report_root: Path,
) -> tuple[str | None, str | None]:
    commit = _require_non_empty_string(artifact, "commit", errors, report_root)
    object_format = _require_non_empty_string(artifact, "git_object_format", errors, report_root)
    if object_format is not None and object_format not in GIT_COMMIT_PATTERNS:
        _add_error(
            errors,
            artifact,
            report_root,
            "E009",
            "field 'git_object_format' must be 'sha1' or 'sha256'",
        )
    elif commit is not None and object_format is not None and not GIT_COMMIT_PATTERNS[object_format].fullmatch(commit):
        _add_error(
            errors,
            artifact,
            report_root,
            "E009",
            f"field 'commit' must be a full lowercase {object_format} Git object ID",
        )
    return commit, object_format


def _validate_timestamp(
    artifact: Artifact,
    field: str,
    errors: list[Diagnostic],
    report_root: Path,
) -> None:
    value = _require_non_empty_string(artifact, field, errors, report_root)
    if value is not None:
        try:
            datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ")
        except ValueError:
            _add_error(errors, artifact, report_root, "E009", f"field '{field}' must use a valid YYYY-MM-DDTHH:MM:SSZ timestamp")


def _validate_evidence_paths(
    artifact: Artifact,
    errors: list[Diagnostic],
    repository_root: Path,
) -> None:
    paths = _require_non_empty_string_list(artifact, "evidence_paths", errors, repository_root)
    if paths is None:
        return
    resolved_root = repository_root.resolve()
    for raw_path in paths:
        relative = Path(raw_path)
        if relative.is_absolute() or "\\" in raw_path or any(part in {"", ".", ".."} for part in relative.parts):
            _add_error(
                errors,
                artifact,
                repository_root,
                "E012",
                f"evidence path must be a normalized repository-relative path: '{raw_path}'",
            )
            continue
        candidate = repository_root / relative
        probe = repository_root
        symlinked = False
        for part in relative.parts:
            probe = probe / part
            if probe.is_symlink():
                symlinked = True
                break
        try:
            resolved = candidate.resolve()
            resolved.relative_to(resolved_root)
        except (OSError, ValueError):
            _add_error(errors, artifact, repository_root, "E012", f"evidence path escapes the repository: '{raw_path}'")
            continue
        if symlinked:
            _add_error(errors, artifact, repository_root, "E012", f"evidence path must not traverse a symlink: '{raw_path}'")
        elif not candidate.is_file():
            _add_error(errors, artifact, repository_root, "E012", f"evidence path does not identify an existing file: '{raw_path}'")


def validate_common_metadata(artifacts: list[Artifact], report_root: Path) -> list[Diagnostic]:
    errors: list[Diagnostic] = []
    seen: dict[str, Artifact] = {}

    for artifact in artifacts:
        artifact_id = _require_non_empty_string(artifact, "id", errors, report_root)
        artifact_type = _require_non_empty_string(artifact, "type", errors, report_root)
        _require_non_empty_string(artifact, "title", errors, report_root)
        status = _require_non_empty_string(artifact, "status", errors, report_root)
        _require_non_empty_string_list(artifact, "owners", errors, report_root)
        created = _require_non_empty_string(artifact, "created", errors, report_root)
        updated = _require_non_empty_string(artifact, "updated", errors, report_root)

        if artifact_id is not None:
            if not ID_PATTERN.fullmatch(artifact_id):
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E002",
                    f"id '{artifact_id}' must use uppercase letters/digits/hyphens and end in a three-digit sequence",
                )
            previous = seen.get(artifact_id)
            if previous is not None:
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E003",
                    f"duplicate id '{artifact_id}' also declared in {_display_path(previous.path, report_root)}",
                )
            else:
                seen[artifact_id] = artifact

        if artifact_type is not None:
            expected_prefix = TYPE_PREFIX.get(artifact_type)
            if expected_prefix is None:
                _add_error(errors, artifact, report_root, "E002", f"unknown artifact type '{artifact_type}'")
            elif artifact_id is not None and not artifact_id.startswith(expected_prefix):
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E004",
                    f"id '{artifact_id}' must start with '{expected_prefix}' for type '{artifact_type}'",
                )

        if status is not None and status not in ALLOWED_STATUSES:
            _add_error(errors, artifact, report_root, "E002", f"unknown status '{status}'")

        for field_name, field_value in (("created", created), ("updated", updated)):
            if field_value is not None and not ISO_DATE_PATTERN.fullmatch(field_value):
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E002",
                    f"field '{field_name}' must use YYYY-MM-DD",
                )

        relations = artifact.metadata.get("relations", {})
        if not isinstance(relations, dict):
            _add_error(errors, artifact, report_root, "E006", "field 'relations' must be a TOML table")

    return errors


def validate_type_specific_metadata(artifacts: list[Artifact], report_root: Path) -> list[Diagnostic]:
    errors: list[Diagnostic] = []

    relation_requirements: dict[str, tuple[str, ...]] = {
        "capability": ("derives_from",),
        "requirement": ("derives_from",),
        "specification": ("specifies",),
        "architecture": ("constrains",),
        "adr": ("decides",),
        "verification": ("verifies",),
        "work_order": ("implements", "specifications", "architecture", "verification"),
        "release_contract": ("gates",),
        "verification_record": ("verifies_work_order", "conforms_to"),
        "release_record": ("satisfies", "includes_verification", "releases_work"),
        "operating_contract": ("assures",),
    }

    for artifact in artifacts:
        artifact_type = artifact.metadata.get("type")
        if not isinstance(artifact_type, str):
            continue

        if artifact_type == "requirement":
            statement = _require_non_empty_string(artifact, "statement", errors, report_root)
            _require_non_empty_string(artifact, "verification_method", errors, report_root)
            if statement is not None and re.search(r"\bSHALL\b", statement) is None:
                _add_error(errors, artifact, report_root, "E005", "requirement statement must contain normative keyword SHALL")

        if artifact_type == "verification_record":
            _validate_git_identity(artifact, errors, report_root)
            worktree_state = _require_non_empty_string(artifact, "worktree_state", errors, report_root)
            if worktree_state is not None and worktree_state != "clean":
                _add_error(errors, artifact, report_root, "E009", "field 'worktree_state' must be 'clean'")
            _validate_timestamp(artifact, "verified_at", errors, report_root)
            snapshot_hash = _require_non_empty_string(artifact, "artifact_snapshot_sha256", errors, report_root)
            if snapshot_hash is not None and not SHA256_PATTERN.fullmatch(snapshot_hash):
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E009",
                    "field 'artifact_snapshot_sha256' must be a lowercase SHA-256 value",
                )
            _validate_evidence_paths(artifact, errors, report_root)
            if artifact.status not in {"ready", "verified", "released", "superseded"}:
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E009",
                    "verification_record status must be ready, verified, released, or superseded",
                )
            if artifact.status == "superseded":
                _validate_timestamp(artifact, "superseded_at", errors, report_root)
                _require_non_empty_string(artifact, "supersession_authorized_by", errors, report_root)
                successors = _require_non_empty_string_list(
                    artifact,
                    "superseded_by",
                    errors,
                    report_root,
                    code="E009",
                    container=artifact.relations,
                )
                if successors is not None and len(successors) != 1:
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E009",
                        "relation 'superseded_by' must contain exactly one verification record",
                    )
            else:
                for field_name in ("superseded_at", "supersession_authorized_by"):
                    if field_name in artifact.metadata:
                        _add_error(
                            errors,
                            artifact,
                            report_root,
                            "E009",
                            f"field '{field_name}' is allowed only when verification_record status is superseded",
                        )
                if "superseded_by" in artifact.relations:
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E009",
                        "relation 'superseded_by' is allowed only when verification_record status is superseded",
                    )

        if artifact_type == "release_record":
            _validate_git_identity(artifact, errors, report_root)
            _require_non_empty_string(artifact, "version", errors, report_root)
            _validate_timestamp(artifact, "released_at", errors, report_root)
            authorized_by = _require_non_empty_string(artifact, "authorized_by", errors, report_root)
            owners = artifact.metadata.get("owners", [])
            if authorized_by is not None and isinstance(owners, list) and authorized_by not in owners:
                _add_error(errors, artifact, report_root, "E009", "field 'authorized_by' must identify one of the record owners")
            tag = artifact.metadata.get("tag")
            if tag is not None and (not isinstance(tag, str) or not tag.strip()):
                _add_error(errors, artifact, report_root, "E009", "field 'tag' must be a non-empty string when present")
            if artifact.status not in {"ready", "released"}:
                _add_error(errors, artifact, report_root, "E009", "release_record status must be ready or released")

        required_relations = relation_requirements.get(artifact_type, ())
        relations = artifact.metadata.get("relations", {})
        if not isinstance(relations, dict):
            continue
        for relation_name in required_relations:
            _require_non_empty_string_list(
                artifact,
                relation_name,
                errors,
                report_root,
                code="E005",
                container=relations,
            )

    return errors


def validate_relations(artifacts: list[Artifact], report_root: Path) -> list[Diagnostic]:
    errors: list[Diagnostic] = []
    catalog = {
        artifact.artifact_id: artifact
        for artifact in artifacts
        if artifact.artifact_id != "<unknown>"
    }

    for artifact in artifacts:
        relations = artifact.metadata.get("relations", {})
        if not isinstance(relations, dict):
            continue
        for relation_name, targets in sorted(relations.items()):
            if not isinstance(targets, list):
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E006",
                    f"relation '{relation_name}' must be an array of artifact IDs",
                )
                continue
            for target in targets:
                if not isinstance(target, str) or not target.strip():
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E006",
                        f"relation '{relation_name}' contains a non-string or empty target",
                    )
                    continue
                if target == artifact.artifact_id:
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E006",
                        f"artifact '{artifact.artifact_id}' must not reference itself via '{relation_name}'",
                    )
                elif target not in catalog:
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E006",
                        f"artifact '{artifact.artifact_id}' relation '{relation_name}' references unknown target '{target}'",
                    )
                else:
                    allowed_types = PROVENANCE_RELATION_TARGET_TYPES.get((artifact.artifact_type, relation_name))
                    target_type = catalog[target].artifact_type
                    if allowed_types is not None and target_type not in allowed_types:
                        expected = ", ".join(sorted(allowed_types))
                        _add_error(
                            errors,
                            artifact,
                            report_root,
                            "E011",
                            f"relation '{relation_name}' target '{target}' must have type {expected}, found {target_type}",
                        )
    return errors


def validate_revision_consistency(
    artifacts: list[Artifact],
    report_root: Path,
    *,
    require_verified_work: bool = False,
) -> list[Diagnostic]:
    errors: list[Diagnostic] = []
    catalog = {artifact.artifact_id: artifact for artifact in artifacts if artifact.artifact_id != "<unknown>"}
    release_versions: dict[str, list[Artifact]] = {}
    supersession_cycle_nodes = _supersession_cycle_nodes(artifacts)

    if require_verified_work:
        verified_work = {
            work_order_id
            for record in artifacts
            if record.artifact_type == "verification_record" and record.status in {"verified", "released"}
            for work_order_id in _relation_targets(record, "verifies_work_order")
        }
        for work_order in artifacts:
            if (
                work_order.artifact_type == "work_order"
                and work_order.status in {"verified", "released"}
                and work_order.artifact_id not in verified_work
            ):
                _add_error(
                    errors,
                    work_order,
                    report_root,
                    "E010",
                    f"{work_order.status} work order requires coverage by a verified or released verification record",
                )

    for artifact in artifacts:
        if artifact.artifact_type == "verification_record":
            for field_name in ("evidence_paths",):
                duplicates = _duplicate_strings(artifact.metadata.get(field_name))
                if duplicates:
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E010",
                        f"field '{field_name}' contains duplicate values: {', '.join(duplicates)}",
                    )
            for relation_name in ("verifies_work_order", "conforms_to", "superseded_by"):
                duplicates = _duplicate_strings(artifact.relations.get(relation_name))
                if duplicates:
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E010",
                        f"relation '{relation_name}' contains duplicate targets: {', '.join(duplicates)}",
                    )
            work_order_ids = _relation_targets(artifact, "verifies_work_order")
            verification_ids = _relation_targets(artifact, "conforms_to")
            declared_verification: set[str] = set()
            for work_order_id in work_order_ids:
                work_order = catalog.get(work_order_id)
                if work_order is None or work_order.artifact_type != "work_order":
                    continue
                declared_verification.update(_relation_targets(work_order, "verification"))
                if artifact.status in {"ready", "verified", "released"} and work_order.status not in ACTIVE_COVERAGE_STATUSES:
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E010",
                        f"active verification record requires active work order '{work_order_id}'",
                    )
            for verification_id in verification_ids:
                verification = catalog.get(verification_id)
                if (
                    verification is not None
                    and verification.artifact_type == "verification"
                    and artifact.status in {"ready", "verified", "released"}
                    and verification.status not in ACTIVE_COVERAGE_STATUSES
                ):
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E010",
                        f"active verification record requires active verification contract '{verification_id}'",
                    )
            missing_verification = declared_verification - verification_ids
            extra_verification = verification_ids - declared_verification
            if len(work_order_ids) > 1 and missing_verification:
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"verification record is missing contracts declared by selected work: {', '.join(sorted(missing_verification))}",
                )
            if extra_verification:
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"verification record includes contracts not declared by selected work: {', '.join(sorted(extra_verification))}",
                )
            if len(work_order_ids) > 1:
                evidence_paths = artifact.metadata.get("evidence_paths", [])
                normalized_paths = [item for item in evidence_paths if isinstance(item, str)] if isinstance(evidence_paths, list) else []
                uncovered = [
                    work_order_id
                    for work_order_id in sorted(work_order_ids)
                    if not any(
                        re.match(rf"^{re.escape(work_order_id)}(?:-|\.|$)", Path(path).name)
                        for path in normalized_paths
                    )
                ]
                if uncovered:
                    _add_error(
                        errors,
                        artifact,
                        report_root,
                        "E010",
                        f"aggregate evidence is not keyed to work orders: {', '.join(uncovered)}",
                    )
            if artifact.status == "superseded":
                successor_ids = sorted(_relation_targets(artifact, "superseded_by"))
                if len(successor_ids) == 1:
                    successor_id = successor_ids[0]
                    successor = catalog.get(successor_id)
                    if successor is not None and successor.artifact_type == "verification_record":
                        if successor.status not in {"verified", "released"}:
                            _add_error(
                                errors,
                                artifact,
                                report_root,
                                "E010",
                                f"superseding verification record '{successor_id}' must be verified or released",
                            )
                        missing_work = work_order_ids - _relation_targets(successor, "verifies_work_order")
                        if missing_work:
                            _add_error(
                                errors,
                                artifact,
                                report_root,
                                "E010",
                                f"superseding verification record '{successor_id}' omits work orders: {', '.join(sorted(missing_work))}",
                            )
            if artifact.artifact_id in supersession_cycle_nodes:
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"verification supersession cycle detected among: {', '.join(sorted(supersession_cycle_nodes))}",
                )

        if artifact.artifact_type != "release_record":
            continue
        for relation_name in ("satisfies", "includes_verification", "releases_work"):
            duplicates = _duplicate_strings(artifact.relations.get(relation_name))
            if duplicates:
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"relation '{relation_name}' contains duplicate targets: {', '.join(duplicates)}",
                )
        version = artifact.metadata.get("version")
        if isinstance(version, str) and version.strip():
            release_versions.setdefault(version.strip(), []).append(artifact)
        release_commit = artifact.metadata.get("commit")
        release_format = artifact.metadata.get("git_object_format")
        released_work = _relation_targets(artifact, "releases_work")
        for work_order_id in released_work:
            work_order = catalog.get(work_order_id)
            if (
                work_order is not None
                and work_order.artifact_type == "work_order"
                and artifact.status in {"ready", "released"}
                and work_order.status not in RELEASABLE_WORK_STATUSES
            ):
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"active release record requires implemented, verified, or released work order '{work_order_id}'",
                )
        verification_work: set[str] = set()
        for verification_id in _relation_targets(artifact, "includes_verification"):
            verification = catalog.get(verification_id)
            if verification is None or verification.artifact_type != "verification_record":
                continue
            if verification.status in {"ready", "verified", "released"}:
                verification_work.update(_relation_targets(verification, "verifies_work_order"))
            if artifact.status in {"ready", "released"} and verification.status == "superseded":
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"active release record must not include superseded verification record '{verification_id}'",
                )
            if release_commit != verification.metadata.get("commit") or release_format != verification.metadata.get("git_object_format"):
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"release commit does not match verification record '{verification_id}'",
                )
            if artifact.status == "released" and verification.status not in {"verified", "released"}:
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"released record requires verified included record '{verification_id}'",
                )
        missing_work = released_work - verification_work
        if missing_work:
            _add_error(
                errors,
                artifact,
                report_root,
                "E010",
                f"released work orders are not covered by included verification records: {', '.join(sorted(missing_work))}",
            )
        extra_work = verification_work - released_work
        if extra_work:
            _add_error(
                errors,
                artifact,
                report_root,
                "E010",
                f"included verification records cover work orders absent from the release: {', '.join(sorted(extra_work))}",
            )
        for contract_id in _relation_targets(artifact, "satisfies"):
            contract = catalog.get(contract_id)
            if contract is None or contract.artifact_type != "release_contract":
                continue
            if artifact.status in {"ready", "released"} and contract.status not in ACTIVE_COVERAGE_STATUSES:
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"active release record requires active release contract '{contract_id}'",
                )
            ungated = released_work - _relation_targets(contract, "gates")
            if ungated:
                _add_error(
                    errors,
                    artifact,
                    report_root,
                    "E010",
                    f"release contract '{contract_id}' does not gate work orders: {', '.join(sorted(ungated))}",
                )

    for version, records in sorted(release_versions.items()):
        if len(records) < 2:
            continue
        record_ids = ", ".join(sorted(record.artifact_id for record in records))
        for record in records:
            _add_error(errors, record, report_root, "E010", f"duplicate release record version '{version}' among {record_ids}")
    return errors


def _supersession_cycle_nodes(artifacts: list[Artifact]) -> set[str]:
    graph = {
        artifact.artifact_id: sorted(_relation_targets(artifact, "superseded_by"))
        for artifact in artifacts
        if artifact.artifact_type == "verification_record"
    }
    state: dict[str, int] = {}
    cycle_nodes: set[str] = set()

    for start in sorted(graph):
        if state.get(start, 0) != 0:
            continue
        path = [start]
        positions = {start: 0}
        frames = [(start, 0)]
        state[start] = 1
        while frames:
            node, successor_index = frames[-1]
            successors = graph.get(node, [])
            if successor_index >= len(successors):
                frames.pop()
                path.pop()
                positions.pop(node, None)
                state[node] = 2
                continue
            successor = successors[successor_index]
            frames[-1] = (node, successor_index + 1)
            if successor not in graph:
                continue
            successor_state = state.get(successor, 0)
            if successor_state == 0:
                state[successor] = 1
                positions[successor] = len(path)
                path.append(successor)
                frames.append((successor, 0))
            elif successor_state == 1:
                cycle_nodes.update(path[positions[successor] :])
    return cycle_nodes


def _relation_targets(artifact: Artifact, relation_name: str) -> set[str]:
    value = artifact.relations.get(relation_name, [])
    if not isinstance(value, list):
        return set()
    return {item for item in value if isinstance(item, str)}


def _duplicate_strings(value: Any) -> list[str]:
    if not isinstance(value, list):
        return []
    strings = [item.strip() for item in value if isinstance(item, str) and item.strip()]
    seen: set[str] = set()
    duplicates: set[str] = set()
    for item in strings:
        if item in seen:
            duplicates.add(item)
        seen.add(item)
    return sorted(duplicates)


def validate_requirement_coverage(artifacts: list[Artifact], report_root: Path) -> list[Diagnostic]:
    errors: list[Diagnostic] = []
    active_specs = [
        artifact
        for artifact in artifacts
        if artifact.artifact_type == "specification" and artifact.status in ACTIVE_COVERAGE_STATUSES
    ]
    active_verifications = [
        artifact
        for artifact in artifacts
        if artifact.artifact_type == "verification" and artifact.status in ACTIVE_COVERAGE_STATUSES
    ]

    specified = set().union(*(_relation_targets(item, "specifies") for item in active_specs)) if active_specs else set()
    verified = set().union(*(_relation_targets(item, "verifies") for item in active_verifications)) if active_verifications else set()

    for artifact in artifacts:
        if artifact.artifact_type != "requirement" or artifact.status not in ACTIVE_COVERAGE_STATUSES:
            continue
        if artifact.artifact_id not in specified:
            _add_error(
                errors,
                artifact,
                report_root,
                "E007",
                f"active requirement '{artifact.artifact_id}' has no active specification coverage",
            )
        if artifact.artifact_id not in verified:
            _add_error(
                errors,
                artifact,
                report_root,
                "E008",
                f"active requirement '{artifact.artifact_id}' has no active verification coverage",
            )

    return errors


def validate_repository(repository_root: Path, artifact_root: Path | None = None) -> ValidationReport:
    repository_root = repository_root.resolve()
    selected_artifact_root = (artifact_root or repository_root / "docs" / "engineering").resolve()
    revision_policy = load_revision_policy(repository_root)

    artifacts, parse_errors = load_artifacts(selected_artifact_root, repository_root)
    errors = list(parse_errors)

    if not selected_artifact_root.exists():
        errors.append(
            Diagnostic(
                _display_path(selected_artifact_root, repository_root),
                "E001",
                "artifact root does not exist",
            )
        )
    else:
        errors.extend(validate_common_metadata(artifacts, repository_root))
        errors.extend(validate_type_specific_metadata(artifacts, repository_root))
        errors.extend(validate_relations(artifacts, repository_root))
        errors.extend(
            validate_revision_consistency(
                artifacts,
                repository_root,
                require_verified_work=revision_policy["required_for_verified_work"],
            )
        )
        errors.extend(validate_requirement_coverage(artifacts, repository_root))

    return ValidationReport(
        artifacts=artifacts,
        errors=sorted(set(errors)),
        warnings=[],
    )


def render_human(report: ValidationReport) -> str:
    status = "PASS" if report.valid else "FAIL"
    lines = [
        f"Engineering artifact validation: {status}",
        f"Artifacts: {len(report.artifacts)} | Errors: {len(report.errors)} | Warnings: {len(report.warnings)}",
    ]
    if report.errors:
        lines.append("")
        lines.append("Errors:")
        for diagnostic in sorted(report.errors):
            lines.append(f"- [{diagnostic.code}] {diagnostic.path}: {diagnostic.message}")
    if report.warnings:
        lines.append("")
        lines.append("Warnings:")
        for diagnostic in sorted(report.warnings):
            lines.append(f"- [{diagnostic.code}] {diagnostic.path}: {diagnostic.message}")
    return "\n".join(lines)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Validate engineering artifact identity, relations, and coverage.")
    parser.add_argument("--root", type=Path, default=Path.cwd(), help="Repository root (default: current directory).")
    parser.add_argument(
        "--artifact-root",
        type=Path,
        default=None,
        help="Artifact directory. Relative paths are resolved below --root; default: docs/engineering.",
    )
    parser.add_argument("--json", action="store_true", dest="as_json", help="Emit a machine-readable JSON report.")
    return parser


def main(argv: Iterable[str] | None = None) -> int:
    args = build_parser().parse_args(list(argv) if argv is not None else None)
    repository_root = args.root.resolve()
    artifact_root = args.artifact_root
    if artifact_root is not None and not artifact_root.is_absolute():
        artifact_root = repository_root / artifact_root

    report = validate_repository(repository_root, artifact_root)
    if args.as_json:
        print(json.dumps(report.to_dict(repository_root), indent=2, sort_keys=True))
    else:
        print(render_human(report))
    return 0 if report.valid else 1


if __name__ == "__main__":
    raise SystemExit(main())
