#!/usr/bin/env python3
"""Generate a deterministic, offline engineering-harness dashboard.

The generator reuses the repository validator as the authoritative parser and
validation core. It adds read-only graph projection, coverage, impact support,
derived consistency findings, readiness evidence, controlled experiment import,
and a self-contained viewer. Only the Python 3.11+ standard library is used.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from collections import Counter, defaultdict, deque
from datetime import UTC, datetime
from pathlib import Path
from typing import Any, Iterable, Sequence

from validate_engineering_artifacts import (
    ACTIVE_COVERAGE_STATUSES,
    Artifact,
    Diagnostic,
    ValidationReport,
    architecture_traceability_state,
    decision_assessment_state,
    load_revision_policy,
    validate_repository,
)


SNAPSHOT_SCHEMA = "harness-dashboard-snapshot-v1"
EXPERIMENT_SCHEMA = "harness-experiment-result-v1"
FINDING_RULES_VERSION = "harness-findings-v7"
QUALITY_GATES_VERSION = "quality-gates-2026-08-10"
DEFAULT_ARTIFACT_ROOT = Path("docs") / "engineering"
DEFAULT_OUTPUT_ROOT = Path("target") / "harness-dashboard"
DEFAULT_EXPERIMENT_ROOT = Path("docs") / "engineering" / "experiments" / "results"
MAX_EXPERIMENT_BYTES = 1_000_000
ACTIVE_WORK_ORDER_STATUSES = ACTIVE_COVERAGE_STATUSES
IMPLEMENTED_STATUSES = {"implemented", "verified", "released"}
INACTIVE_GOVERNING_STATUSES = {"draft", "rejected", "superseded"}
SEVERITY_ORDER = {"error": 0, "warning": 1, "info": 2}
WORK_ORDER_RELATIONS = ("implements", "specifications", "architecture", "verification")
TEMPORAL_REASSESSMENT_RELATIONS = {
    "capability": frozenset({"derives_from"}),
    "requirement": frozenset({"derives_from"}),
    "specification": frozenset({"specifies"}),
    "architecture": frozenset({"addresses", "conforms_to", "constrains"}),
    "adr": frozenset({"decides"}),
    "verification": frozenset({"verifies"}),
    "release_contract": frozenset({"gates"}),
    "operating_contract": frozenset({"assures"}),
    "work_order": frozenset(WORK_ORDER_RELATIONS),
}
TEMPORAL_REASSESSMENT_INACTIVE_STATUSES = frozenset({"rejected", "superseded"})
TEMPORAL_REASSESSMENT_WORK_ORDER_STATUSES = frozenset({"draft", "approved", "in_progress"})
EXPERIMENT_MEASURES = (
    "clarifications",
    "retries",
    "evaluator_defects",
    "wall_seconds",
    "tokens",
    "cost",
)


class GenerationError(RuntimeError):
    """A bounded configuration or generation failure."""


def _is_within(path: Path, parent: Path) -> bool:
    try:
        path.relative_to(parent)
        return True
    except ValueError:
        return False


def _paths_overlap(first: Path, second: Path) -> bool:
    return _is_within(first, second) or _is_within(second, first)


def resolve_repository_root(value: Path) -> Path:
    root = value.resolve()
    if not root.exists() or not root.is_dir():
        raise GenerationError("repository root must be an existing readable directory")
    return root


def resolve_artifact_root(repository_root: Path, value: Path | None) -> Path:
    candidate = value or DEFAULT_ARTIFACT_ROOT
    resolved = candidate.resolve() if candidate.is_absolute() else (repository_root / candidate).resolve()
    if not _is_within(resolved, repository_root):
        raise GenerationError("artifact root must resolve within the repository root")
    return resolved


def resolve_output_root(
    repository_root: Path,
    artifact_root: Path,
    value: Path | None,
) -> Path:
    candidate = value or DEFAULT_OUTPUT_ROOT
    resolved = candidate.resolve() if candidate.is_absolute() else (repository_root / candidate).resolve()
    if _paths_overlap(resolved, repository_root) and not _is_within(resolved, repository_root):
        raise GenerationError("output root must not contain the repository root")
    if resolved == repository_root or _paths_overlap(resolved, artifact_root):
        raise GenerationError("output root must not overlap the repository or artifact root")
    if resolved.exists() and resolved.is_symlink():
        raise GenerationError("output root must not be a symbolic link")
    if resolved.exists() and not resolved.is_dir():
        raise GenerationError("output root collides with a non-directory path")
    return resolved


def repository_relative(path: Path, repository_root: Path) -> str:
    try:
        return path.resolve().relative_to(repository_root).as_posix()
    except ValueError:
        raise GenerationError("a repository source path resolved outside the repository root") from None


def _safe_repository_reference(value: Any, repository_root: Path) -> str | None:
    if not isinstance(value, str) or not value.strip():
        return None
    candidate = Path(value.strip())
    if candidate.is_absolute():
        return None
    resolved = (repository_root / candidate).resolve()
    if not _is_within(resolved, repository_root):
        return None
    return resolved.relative_to(repository_root).as_posix()


def git_revision(repository_root: Path) -> str | None:
    git = shutil.which("git")
    if git is None:
        return None
    try:
        completed = subprocess.run(
            [git, "-C", str(repository_root), "rev-parse", "HEAD"],
            check=False,
            capture_output=True,
            text=True,
            timeout=5,
        )
    except (OSError, subprocess.SubprocessError):
        return None
    revision = completed.stdout.strip()
    if completed.returncode != 0 or not re.fullmatch(r"[0-9a-fA-F]{7,64}", revision):
        return None
    return revision.lower()


def git_commit_availability(repository_root: Path, commits: Sequence[str]) -> dict[str, bool | None]:
    unique = sorted({commit for commit in commits if re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", commit)})
    if not unique:
        return {}
    git = shutil.which("git")
    if git is None or git_revision(repository_root) is None:
        return {commit: None for commit in unique}
    try:
        completed = subprocess.run(
            [git, "-C", str(repository_root), "cat-file", "--batch-check=%(objectname) %(objecttype)"],
            input="".join(f"{commit}^{{commit}}\n" for commit in unique),
            check=False,
            capture_output=True,
            text=True,
            timeout=5,
        )
    except (OSError, subprocess.SubprocessError):
        return {commit: None for commit in unique}
    if completed.returncode != 0:
        return {commit: None for commit in unique}
    lines = completed.stdout.splitlines()
    result: dict[str, bool | None] = {}
    for index, commit in enumerate(unique):
        line = lines[index] if index < len(lines) else ""
        result[commit] = bool(re.fullmatch(r"[0-9a-f]{40,64} commit", line.strip()))
    return result


def _string(value: Any, fallback: str = "") -> str:
    return value.strip() if isinstance(value, str) and value.strip() else fallback


def _string_list(value: Any) -> list[str]:
    if not isinstance(value, list):
        return []
    return [item.strip() for item in value if isinstance(item, str) and item.strip()]


def normalize_artifacts(report: ValidationReport, repository_root: Path) -> list[dict[str, Any]]:
    catalog = {
        artifact.artifact_id: artifact
        for artifact in report.artifacts
        if artifact.artifact_id != "<unknown>"
    }
    active_decisions_by_architecture: dict[str, set[str]] = defaultdict(set)
    for decision in report.artifacts:
        if decision.artifact_type != "adr" or decision.status not in ACTIVE_COVERAGE_STATUSES:
            continue
        for architecture_id in _string_list(decision.relations.get("decides")):
            active_decisions_by_architecture[architecture_id].add(decision.artifact_id)

    normalized: list[dict[str, Any]] = []
    for artifact in report.artifacts:
        item: dict[str, Any] = {
            "id": artifact.artifact_id,
            "type": artifact.artifact_type,
            "title": _string(artifact.metadata.get("title"), "<untitled>"),
            "status": artifact.status,
            "owners": _string_list(artifact.metadata.get("owners")),
            "created": _string(artifact.metadata.get("created")) or None,
            "updated": _string(artifact.metadata.get("updated")) or None,
            "path": repository_relative(artifact.path, repository_root),
            "authority": "formal",
        }
        if artifact.artifact_type == "requirement":
            item["statement"] = _string(artifact.metadata.get("statement")) or None
            item["verification_method"] = _string(artifact.metadata.get("verification_method")) or None
        if artifact.artifact_type == "architecture":
            item["architecture_traceability"] = architecture_traceability_state(
                artifact,
                catalog,
            )
            assessment = decision_assessment_state(artifact)
            deciding_adrs = sorted(active_decisions_by_architecture.get(artifact.artifact_id, set()))
            if assessment["state"] == "valid":
                if assessment["outcome"] == "adr_required":
                    state = "adr_required_covered" if deciding_adrs else "adr_required_missing"
                else:
                    state = "no_significant_decision_justified"
            elif assessment["state"] == "legacy_missing":
                state = "legacy_adr_covered" if deciding_adrs else "legacy_adr_missing"
            else:
                state = f"assessment_{assessment['state']}"
            item["decision_assessment"] = {
                "state": state,
                "outcome": assessment["outcome"],
                "triggers": assessment["triggers"],
                "rationale": assessment["rationale"],
                "assessed_by": assessment["assessed_by"],
                "deciding_adrs": deciding_adrs,
            }
        if artifact.artifact_type == "verification_record":
            item["commit"] = _string(artifact.metadata.get("commit")) or None
            item["git_object_format"] = _string(artifact.metadata.get("git_object_format")) or None
            item["worktree_state"] = _string(artifact.metadata.get("worktree_state")) or None
            item["verified_at"] = _string(artifact.metadata.get("verified_at")) or None
            item["artifact_snapshot_sha256"] = _string(artifact.metadata.get("artifact_snapshot_sha256")) or None
            item["evidence_paths"] = _string_list(artifact.metadata.get("evidence_paths"))
            item["superseded_at"] = _string(artifact.metadata.get("superseded_at")) or None
            item["supersession_authorized_by"] = _string(artifact.metadata.get("supersession_authorized_by")) or None
        if artifact.artifact_type == "release_record":
            item["commit"] = _string(artifact.metadata.get("commit")) or None
            item["git_object_format"] = _string(artifact.metadata.get("git_object_format")) or None
            item["version"] = _string(artifact.metadata.get("version")) or None
            item["tag"] = _string(artifact.metadata.get("tag")) or None
            item["released_at"] = _string(artifact.metadata.get("released_at")) or None
            item["authorized_by"] = _string(artifact.metadata.get("authorized_by")) or None
        normalized.append(item)
    return sorted(normalized, key=lambda item: (item["id"], item["path"]))


def build_declared_relations(artifacts: Sequence[Artifact]) -> list[dict[str, Any]]:
    catalog = {artifact.artifact_id for artifact in artifacts if artifact.artifact_id != "<unknown>"}
    relations: list[dict[str, Any]] = []
    for artifact in artifacts:
        for relation_name, targets in sorted(artifact.relations.items()):
            if not isinstance(targets, list):
                continue
            for target in targets:
                if not isinstance(target, str) or not target.strip():
                    continue
                clean_target = target.strip()
                relations.append(
                    {
                        "source": artifact.artifact_id,
                        "relation": relation_name,
                        "target": clean_target,
                        "authority": "declared",
                        "target_exists": clean_target in catalog,
                    }
                )
    return sorted(
        relations,
        key=lambda item: (item["source"], item["relation"], item["target"], not item["target_exists"]),
    )


def build_architecture_transitive_relations(artifacts: Sequence[Artifact]) -> list[dict[str, Any]]:
    catalog = {
        artifact.artifact_id: artifact
        for artifact in artifacts
        if artifact.artifact_id != "<unknown>"
    }
    paths: dict[tuple[str, str], set[str]] = defaultdict(set)
    for architecture in artifacts:
        if architecture.artifact_type != "architecture":
            continue
        traceability = architecture_traceability_state(architecture, catalog)
        if traceability["state"] not in {"typed", "dual_declared"}:
            continue
        for specification_id in traceability["conforms_to"]:
            specification = catalog.get(specification_id)
            if specification is None or specification.artifact_type != "specification":
                continue
            for requirement_id in _string_list(specification.relations.get("specifies")):
                if requirement_id in catalog:
                    paths[(architecture.artifact_id, requirement_id)].add(specification_id)
    return [
        {
            "source": source,
            "relation": "conforms_transitively_to_requirement",
            "target": target,
            "authority": "derived",
            "target_exists": True,
            "via": sorted(via),
        }
        for (source, target), via in sorted(paths.items())
    ]


def _diagnostic_payload(
    diagnostic: Diagnostic,
    severity: str,
    artifacts_by_path: dict[str, list[str]],
) -> dict[str, Any]:
    return {
        "code": diagnostic.code,
        "severity": severity,
        "path": diagnostic.path,
        "message": diagnostic.message,
        "artifacts": sorted(artifacts_by_path.get(diagnostic.path, [])),
        "authority": "validator",
    }


def normalize_diagnostics(
    report: ValidationReport,
    normalized_artifacts: Sequence[dict[str, Any]],
) -> list[dict[str, Any]]:
    artifacts_by_path: dict[str, list[str]] = defaultdict(list)
    for artifact in normalized_artifacts:
        artifacts_by_path[artifact["path"]].append(artifact["id"])
    diagnostics = [
        *(_diagnostic_payload(item, "error", artifacts_by_path) for item in report.errors),
        *(_diagnostic_payload(item, "warning", artifacts_by_path) for item in report.warnings),
    ]
    return sorted(
        diagnostics,
        key=lambda item: (
            SEVERITY_ORDER[item["severity"]],
            item["code"],
            item["path"],
            item["message"],
        ),
    )


def discover_evidence(repository_root: Path) -> dict[str, list[str]]:
    engineering_root = repository_root / "docs" / "engineering"
    evidence: dict[str, list[str]] = defaultdict(list)
    if not engineering_root.exists():
        return {}
    work_order_pattern = re.compile(r"^(WO-[A-Z0-9-]*\d{3})(?:-|\.|$)")
    for path in sorted(engineering_root.rglob("*")):
        if not path.is_file() or "evidence" not in path.parts:
            continue
        match = work_order_pattern.match(path.name)
        if match is not None:
            evidence[match.group(1)].append(repository_relative(path, repository_root))
    return {key: sorted(set(paths)) for key, paths in sorted(evidence.items())}


def _valid_relations(relations: Sequence[dict[str, Any]]) -> list[dict[str, Any]]:
    return [relation for relation in relations if relation["target_exists"]]


def compute_impact(
    selected: str,
    relations: Sequence[dict[str, Any]],
) -> dict[str, list[str]]:
    nodes = {
        value
        for relation in relations
        if relation.get("target_exists", True)
        for value in (relation["source"], relation["target"])
    }
    if selected not in nodes:
        return {
            "direct_inbound": [],
            "direct_outbound": [],
            "transitive_inbound": [],
            "transitive_outbound": [],
        }

    outbound: dict[str, set[str]] = defaultdict(set)
    inbound: dict[str, set[str]] = defaultdict(set)
    for relation in relations:
        if not relation.get("target_exists", True):
            continue
        outbound[relation["source"]].add(relation["target"])
        inbound[relation["target"]].add(relation["source"])

    def closure(start: str, adjacency: dict[str, set[str]]) -> set[str]:
        seen = {start}
        queue = deque(sorted(adjacency.get(start, set())))
        while queue:
            current = queue.popleft()
            if current in seen:
                continue
            seen.add(current)
            queue.extend(sorted(adjacency.get(current, set()) - seen))
        seen.discard(start)
        return seen

    direct_outbound = outbound.get(selected, set())
    direct_inbound = inbound.get(selected, set())
    outbound_closure = closure(selected, outbound)
    inbound_closure = closure(selected, inbound)
    return {
        "direct_inbound": sorted(direct_inbound),
        "direct_outbound": sorted(direct_outbound),
        "transitive_inbound": sorted(inbound_closure - direct_inbound),
        "transitive_outbound": sorted(outbound_closure - direct_outbound),
    }


def _strongly_connected_components(
    artifact_ids: Sequence[str],
    relations: Sequence[dict[str, Any]],
) -> list[list[str]]:
    adjacency: dict[str, list[str]] = {artifact_id: [] for artifact_id in artifact_ids}
    reverse: dict[str, list[str]] = {artifact_id: [] for artifact_id in artifact_ids}
    self_loops: set[str] = set()
    for relation in relations:
        if not relation.get("target_exists", True):
            continue
        source = relation["source"]
        target = relation["target"]
        if source not in adjacency or target not in adjacency:
            continue
        adjacency[source].append(target)
        reverse[target].append(source)
        if source == target:
            self_loops.add(source)
    for graph in (adjacency, reverse):
        for node in graph:
            graph[node] = sorted(set(graph[node]))

    visited: set[str] = set()
    finish_order: list[str] = []
    for start in sorted(adjacency):
        if start in visited:
            continue
        visited.add(start)
        stack: list[tuple[str, int]] = [(start, 0)]
        while stack:
            node, index = stack[-1]
            neighbors = adjacency[node]
            if index < len(neighbors):
                neighbor = neighbors[index]
                stack[-1] = (node, index + 1)
                if neighbor not in visited:
                    visited.add(neighbor)
                    stack.append((neighbor, 0))
            else:
                stack.pop()
                finish_order.append(node)

    assigned: set[str] = set()
    components: list[list[str]] = []
    for start in reversed(finish_order):
        if start in assigned:
            continue
        component: list[str] = []
        stack = [start]
        assigned.add(start)
        while stack:
            node = stack.pop()
            component.append(node)
            for neighbor in reverse[node]:
                if neighbor not in assigned:
                    assigned.add(neighbor)
                    stack.append(neighbor)
        component.sort()
        if len(component) > 1 or component[0] in self_loops:
            components.append(component)
    return sorted(components)


def _finding(
    rule: str,
    severity: str,
    message: str,
    artifacts: Iterable[str] = (),
    paths: Iterable[str] = (),
    evidence: Iterable[str] = (),
    *,
    authority: str = "derived",
) -> dict[str, Any]:
    return {
        "rule": rule,
        "severity": severity,
        "message": message,
        "artifacts": sorted(set(artifacts)),
        "paths": sorted(set(paths)),
        "evidence": sorted(set(evidence)),
        "authority": authority,
    }


def _supports_temporal_reassessment(
    source: dict[str, Any],
    relation: dict[str, Any],
) -> bool:
    """Return whether a declared edge has governed reassessment meaning."""

    if relation.get("authority") != "declared":
        return False
    supported = TEMPORAL_REASSESSMENT_RELATIONS.get(source["type"], frozenset())
    if relation["relation"] not in supported:
        return False
    if source["status"] in TEMPORAL_REASSESSMENT_INACTIVE_STATUSES:
        return False
    if source["type"] == "work_order":
        return source["status"] in TEMPORAL_REASSESSMENT_WORK_ORDER_STATUSES
    return True


def build_findings(
    normalized_artifacts: Sequence[dict[str, Any]],
    relations: Sequence[dict[str, Any]],
    diagnostics: Sequence[dict[str, Any]],
    evidence_by_work_order: dict[str, list[str]],
    revision_provenance: Sequence[dict[str, Any]],
    revision_policy: dict[str, bool],
) -> list[dict[str, Any]]:
    artifacts = {artifact["id"]: artifact for artifact in normalized_artifacts}
    findings = [
        _finding(
            diagnostic["code"],
            diagnostic["severity"],
            diagnostic["message"],
            diagnostic["artifacts"],
            [diagnostic["path"]],
            authority="validator",
        )
        for diagnostic in diagnostics
    ]

    for artifact in normalized_artifacts:
        if (
            artifact["type"] == "work_order"
            and artifact["status"] in IMPLEMENTED_STATUSES
            and not evidence_by_work_order.get(artifact["id"])
        ):
            findings.append(
                _finding(
                    "W-HEX-001",
                    "warning",
                    f"{artifact['id']} is {artifact['status']} but has no evidence document keyed to its ID.",
                    [artifact["id"]],
                    [artifact["path"]],
                )
            )

    relations_by_source: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for relation in relations:
        relations_by_source[relation["source"]].append(relation)

    for artifact in normalized_artifacts:
        if artifact["type"] != "work_order" or artifact["status"] not in ACTIVE_WORK_ORDER_STATUSES:
            continue
        invalid_governing: set[str] = set()
        for relation in relations_by_source.get(artifact["id"], []):
            if relation["relation"] not in WORK_ORDER_RELATIONS or not relation["target_exists"]:
                continue
            target = artifacts.get(relation["target"])
            if target is not None and target["status"] in INACTIVE_GOVERNING_STATUSES:
                invalid_governing.add(target["id"])
        if invalid_governing:
            findings.append(
                _finding(
                    "W-HEX-002",
                    "warning",
                    f"{artifact['id']} references inactive governing artifacts: {', '.join(sorted(invalid_governing))}.",
                    [artifact["id"], *invalid_governing],
                    [artifact["path"], *(artifacts[item]["path"] for item in invalid_governing)],
                )
            )

    stale_relations: set[tuple[str, str, str]] = set()
    for relation in relations:
        if not relation["target_exists"]:
            continue
        source = artifacts.get(relation["source"])
        target = artifacts.get(relation["target"])
        if source is None or target is None or not source["updated"] or not target["updated"]:
            continue
        relation_key = (source["id"], relation["relation"], target["id"])
        if (
            _supports_temporal_reassessment(source, relation)
            and source["updated"] < target["updated"]
            and relation_key not in stale_relations
        ):
            stale_relations.add(relation_key)
            findings.append(
                _finding(
                    "W-HEX-003",
                    "warning",
                    f"{source['id']} predates newer declared {relation['relation']} target {target['id']} and may require reassessment.",
                    [source["id"], target["id"]],
                    [source["path"], target["path"]],
                    [
                        f"relation={relation['relation']}",
                        f"{source['updated']} < {target['updated']}",
                    ],
                )
            )

    components = _strongly_connected_components(sorted(artifacts), _valid_relations(relations))
    for component in components:
        findings.append(
            _finding(
                "W-HEX-004",
                "warning",
                f"Declared dependency traversal contains a cycle among: {', '.join(component)}.",
                component,
                [artifacts[item]["path"] for item in component if item in artifacts],
            )
        )

    connected: set[str] = set()
    for relation in relations:
        if relation["target_exists"]:
            connected.add(relation["source"])
            connected.add(relation["target"])
    for artifact in normalized_artifacts:
        if artifact["type"] != "intent" and artifact["id"] not in connected:
            findings.append(
                _finding(
                    "W-HEX-005",
                    "warning",
                    f"{artifact['id']} has no valid declared edge after invalid targets are removed.",
                    [artifact["id"]],
                    [artifact["path"]],
                )
            )

    relation_counts = Counter(
        (relation["source"], relation["relation"], relation["target"])
        for relation in relations
    )
    for (source, relation_name, target), count in sorted(relation_counts.items()):
        if count > 1:
            paths = [artifacts[source]["path"]] if source in artifacts else []
            findings.append(
                _finding(
                    "W-HEX-006",
                    "warning",
                    f"{source} repeats target {target} {count} times in relation {relation_name}.",
                    [source, target],
                    paths,
                    [f"duplicate_count={count}"],
                )
            )

    verified_by_work: dict[str, set[str]] = defaultdict(set)
    released_by_work: dict[str, set[str]] = defaultdict(set)
    for entry in revision_provenance:
        for work_order in entry["work_orders"]:
            if entry["kind"] == "verification" and entry["status"] in {"verified", "released"}:
                verified_by_work[work_order].add(entry["id"])
            if entry["kind"] == "release" and entry["status"] == "released":
                released_by_work[work_order].add(entry["id"])
        if entry["match_state"] == "different" and entry["status"] != "superseded":
            findings.append(
                _finding(
                    "I-REV-001",
                    "info",
                    f"Observed checkout differs from declared candidate commit on {entry['id']}; this can be expected in a later governance commit.",
                    [entry["id"]],
                    evidence=[f"declared={entry['commit']}", f"observed={entry['observed_revision'] or 'unavailable'}"],
                )
            )
        if entry["commit_available"] is False:
            findings.append(
                _finding(
                    "W-REV-003",
                    "warning",
                    f"Declared candidate commit on {entry['id']} is unavailable in the current clone.",
                    [entry["id"]],
                    evidence=[f"declared={entry['commit']}"],
                )
            )

    verification_entries = [entry for entry in revision_provenance if entry["kind"] == "verification"]
    for source in verification_entries:
        source_work = set(source["work_orders"])
        if source["status"] != "ready" or not source_work or source["superseded_by"]:
            continue
        possible_successors = sorted(
            target["id"]
            for target in verification_entries
            if target["id"] != source["id"]
            and target["status"] in {"verified", "released"}
            and source_work <= set(target["work_orders"])
        )
        if possible_successors:
            findings.append(
                _finding(
                    "W-REV-004",
                    "warning",
                    f"{source['id']} is ready but its work is fully covered by verified or released records; review possible supersession without inferring authority.",
                    [source["id"], *possible_successors],
                    [artifacts[source["id"]]["path"], *(artifacts[item]["path"] for item in possible_successors)],
                    [f"possible_successors={','.join(possible_successors)}"],
                )
            )

    for artifact in normalized_artifacts:
        if artifact["type"] != "work_order":
            continue
        if (
            revision_policy["required_for_release"]
            and artifact["status"] == "released"
            and not released_by_work.get(artifact["id"])
        ):
            findings.append(
                _finding(
                    "W-REV-002",
                    "warning",
                    f"{artifact['id']} is released but has no released commit-bound release record.",
                    [artifact["id"]],
                    [artifact["path"]],
                )
            )

    return sorted(
        findings,
        key=lambda item: (
            SEVERITY_ORDER[item["severity"]],
            item["rule"],
            item["artifacts"],
            item["message"],
        ),
    )


def build_coverage(
    normalized_artifacts: Sequence[dict[str, Any]],
    relations: Sequence[dict[str, Any]],
) -> list[dict[str, Any]]:
    artifacts = {artifact["id"]: artifact for artifact in normalized_artifacts}
    active_specs: dict[str, set[str]] = defaultdict(set)
    active_verifications: dict[str, set[str]] = defaultdict(set)
    for relation in relations:
        if not relation["target_exists"]:
            continue
        source = artifacts.get(relation["source"])
        target = artifacts.get(relation["target"])
        if source is None or target is None or target["type"] != "requirement":
            continue
        if relation["relation"] == "specifies" and source["type"] == "specification" and source["status"] in ACTIVE_COVERAGE_STATUSES:
            active_specs[target["id"]].add(source["id"])
        if relation["relation"] == "verifies" and source["type"] == "verification" and source["status"] in ACTIVE_COVERAGE_STATUSES:
            active_verifications[target["id"]].add(source["id"])

    coverage: list[dict[str, Any]] = []
    for artifact in normalized_artifacts:
        if artifact["type"] != "requirement":
            continue
        active = artifact["status"] in ACTIVE_COVERAGE_STATUSES
        specifications = sorted(active_specs.get(artifact["id"], set()))
        verifications = sorted(active_verifications.get(artifact["id"], set()))
        missing: list[str] = []
        if active and not specifications:
            missing.append("specification")
        if active and not verifications:
            missing.append("verification")
        coverage.append(
            {
                "requirement": artifact["id"],
                "status": artifact["status"],
                "active": active,
                "specifications": specifications,
                "verifications": verifications,
                "specified": bool(specifications),
                "verified": bool(verifications),
                "missing": missing,
            }
        )
    return sorted(coverage, key=lambda item: item["requirement"])


def _condition(
    condition_id: str,
    label: str,
    state: str,
    evidence: Iterable[str] = (),
) -> dict[str, Any]:
    return {
        "id": condition_id,
        "label": label,
        "state": state,
        "evidence": sorted(set(evidence)),
    }


def _gate(gate_id: str, label: str, conditions: Sequence[dict[str, Any]]) -> dict[str, Any]:
    states = {condition["state"] for condition in conditions}
    state = "unsatisfied" if "unsatisfied" in states else "not_assessable" if "not_assessable" in states else "satisfied"
    return {"gate": gate_id, "label": label, "state": state, "conditions": list(conditions)}


def build_readiness(
    normalized_artifacts: Sequence[dict[str, Any]],
    relations: Sequence[dict[str, Any]],
    diagnostics: Sequence[dict[str, Any]],
    evidence_by_work_order: dict[str, list[str]],
    revision_policy: dict[str, bool],
) -> list[dict[str, Any]]:
    artifacts = {artifact["id"]: artifact for artifact in normalized_artifacts}
    valid_relations = _valid_relations(relations)
    outbound: dict[str, list[dict[str, Any]]] = defaultdict(list)
    inbound: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for relation in valid_relations:
        outbound[relation["source"]].append(relation)
        inbound[relation["target"]].append(relation)
    diagnostic_artifacts = {
        artifact_id
        for diagnostic in diagnostics
        if diagnostic["severity"] == "error"
        for artifact_id in diagnostic["artifacts"]
    }

    readiness: list[dict[str, Any]] = []
    for work_order in normalized_artifacts:
        if work_order["type"] != "work_order":
            continue
        by_relation: dict[str, list[str]] = defaultdict(list)
        for relation in outbound.get(work_order["id"], []):
            by_relation[relation["relation"]].append(relation["target"])
        for relation_name in by_relation:
            by_relation[relation_name] = sorted(set(by_relation[relation_name]))

        reachable: set[str] = set()
        queue = deque([work_order["id"]])
        while queue:
            current = queue.popleft()
            for relation in outbound.get(current, []):
                if relation["target"] not in reachable:
                    reachable.add(relation["target"])
                    queue.append(relation["target"])
        intents = sorted(
            artifact_id
            for artifact_id in reachable
            if artifacts.get(artifact_id, {}).get("type") == "intent"
            and artifacts[artifact_id]["status"] in ACTIVE_COVERAGE_STATUSES
        )
        requirements = by_relation.get("implements", [])
        required_contracts = [
            artifact_id
            for relation_name in ("specifications", "architecture", "verification")
            for artifact_id in by_relation.get(relation_name, [])
        ]
        governing_active = bool(required_contracts) and all(
            artifacts.get(artifact_id, {}).get("status") in ACTIVE_COVERAGE_STATUSES
            for artifact_id in required_contracts
        )
        requirements_active = bool(requirements) and all(
            artifacts.get(artifact_id, {}).get("status") in ACTIVE_COVERAGE_STATUSES
            for artifact_id in requirements
        )
        requirements_clean = requirements_active and not any(
            artifact_id in diagnostic_artifacts for artifact_id in requirements
        )
        release_ids = sorted(
            {
                relation["source"]
                for relation in inbound.get(work_order["id"], [])
                if relation["relation"] == "gates"
                and artifacts.get(relation["source"], {}).get("type") == "release_contract"
            }
        )
        release_active = bool(release_ids) and all(
            artifacts[item]["status"] in ACTIVE_COVERAGE_STATUSES for item in release_ids
        )
        assured_targets = set(release_ids) | set(requirements)
        operations_ids = sorted(
            {
                relation["source"]
                for target in assured_targets
                for relation in inbound.get(target, [])
                if relation["relation"] == "assures"
                and artifacts.get(relation["source"], {}).get("type") == "operating_contract"
            }
        )
        operations_active = bool(operations_ids) and all(
            artifacts[item]["status"] in ACTIVE_COVERAGE_STATUSES for item in operations_ids
        )
        evidence_paths = evidence_by_work_order.get(work_order["id"], [])
        verification_record_ids = sorted(
            {
                relation["source"]
                for relation in inbound.get(work_order["id"], [])
                if relation["relation"] == "verifies_work_order"
                and artifacts.get(relation["source"], {}).get("type") == "verification_record"
                and artifacts[relation["source"]]["status"] in {"verified", "released"}
            }
        )
        release_record_ids = sorted(
            {
                relation["source"]
                for relation in inbound.get(work_order["id"], [])
                if relation["relation"] == "releases_work"
                and artifacts.get(relation["source"], {}).get("type") == "release_record"
                and artifacts[relation["source"]]["status"] in {"ready", "released"}
            }
        )
        released_record_ids = [
            artifact_id for artifact_id in release_record_ids if artifacts[artifact_id]["status"] == "released"
        ]

        gates = [
            _gate(
                "G0",
                "Intent ready",
                [
                    _condition("intent_chain", "Approved intent is reachable", "satisfied" if intents else "unsatisfied", intents),
                    _condition("intent_quality", "Outcome quality and stakeholder agreement", "not_assessable"),
                ],
            ),
            _gate(
                "G1",
                "Requirement ready",
                [
                    _condition("requirements_declared", "In-scope requirements are active", "satisfied" if requirements_active else "unsatisfied", requirements),
                    _condition("requirement_metadata", "Requirement metadata has no validator error", "satisfied" if requirements_clean else "unsatisfied", requirements),
                    _condition("requirement_semantics", "Domain meaning and examples are adequate", "not_assessable"),
                ],
            ),
            _gate(
                "G2",
                "Engineering ready",
                [
                    _condition("governing_contracts", "Specification, architecture, and verification are active", "satisfied" if governing_active else "unsatisfied", required_contracts),
                    _condition("work_authorization", "Work order is active", "satisfied" if work_order["status"] in ACTIVE_WORK_ORDER_STATUSES else "unsatisfied", [work_order["id"]]),
                ],
            ),
            _gate(
                "G3",
                "Implementation complete",
                [
                    _condition("implementation_status", "Work order records implementation completion", "satisfied" if work_order["status"] in IMPLEMENTED_STATUSES else "unsatisfied", [work_order["id"]]),
                    _condition("verification_evidence", "Work-order evidence is retained", "satisfied" if evidence_paths else "unsatisfied", evidence_paths),
                    _condition(
                        "verified_revision",
                        "A verified record binds the candidate commit",
                        "satisfied" if verification_record_ids else "unsatisfied" if revision_policy["required_for_verified_work"] else "not_assessable",
                        verification_record_ids,
                    ),
                    _condition("repository_checks", "All required implementation checks passed", "not_assessable"),
                ],
            ),
            _gate(
                "G4",
                "Release ready",
                [
                    _condition("release_contract", "An active release contract gates the work", "satisfied" if release_active else "unsatisfied", release_ids),
                    _condition(
                        "release_revision",
                        "A release record identifies the verified commit",
                        "satisfied" if release_record_ids else "unsatisfied" if revision_policy["required_for_release"] else "not_assessable",
                        release_record_ids,
                    ),
                    _condition("promotion_evidence", "Security, provenance, compatibility, and rollback evidence passed", "not_assessable"),
                ],
            ),
            _gate(
                "G5",
                "Operationally accepted",
                [
                    _condition("operating_contract", "An active operating contract assures the chain", "satisfied" if operations_active else "unsatisfied", operations_ids),
                    _condition(
                        "released_revision",
                        "An authorized release record binds the released commit",
                        "satisfied" if released_record_ids else "unsatisfied" if revision_policy["required_for_release"] and work_order["status"] == "released" else "not_assessable",
                        released_record_ids,
                    ),
                    _condition("observation_window", "Post-release operating evidence is within bounds", "not_assessable"),
                ],
            ),
        ]
        readiness.append(
            {
                "work_order": work_order["id"],
                "status": work_order["status"],
                "gates": gates,
            }
        )
    return sorted(readiness, key=lambda item: item["work_order"])


def build_revision_provenance(
    normalized_artifacts: Sequence[dict[str, Any]],
    relations: Sequence[dict[str, Any]],
    observed_revision: str | None,
    commit_availability: dict[str, bool | None],
) -> list[dict[str, Any]]:
    relations_by_source: dict[str, list[dict[str, Any]]] = defaultdict(list)
    relations_by_target: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for relation in relations:
        if relation["target_exists"]:
            relations_by_source[relation["source"]].append(relation)
            relations_by_target[relation["target"]].append(relation)
    result: list[dict[str, Any]] = []
    for artifact in normalized_artifacts:
        if artifact["type"] not in {"verification_record", "release_record"}:
            continue
        commit = artifact.get("commit")
        match_state = (
            "not_assessable"
            if not commit or not observed_revision
            else "exact"
            if commit == observed_revision
            else "different"
        )
        relation_names = {relation["relation"]: [] for relation in relations_by_source[artifact["id"]]}
        for relation in relations_by_source[artifact["id"]]:
            relation_names[relation["relation"]].append(relation["target"])
        work_relation = "verifies_work_order" if artifact["type"] == "verification_record" else "releases_work"
        supersedes = sorted(
            {
                relation["source"]
                for relation in relations_by_target[artifact["id"]]
                if relation["relation"] == "superseded_by"
            }
        )
        lifecycle_class = (
            "historical"
            if artifact["status"] == "superseded"
            else "active_candidate"
            if artifact["status"] == "ready"
            else "assured"
        )
        result.append(
            {
                "id": artifact["id"],
                "kind": "verification" if artifact["type"] == "verification_record" else "release",
                "status": artifact["status"],
                "commit": commit,
                "git_object_format": artifact.get("git_object_format"),
                "observed_revision": observed_revision,
                "match_state": match_state,
                "commit_available": commit_availability.get(commit),
                "work_orders": sorted(set(relation_names.get(work_relation, []))),
                "verification_records": sorted(set(relation_names.get("includes_verification", []))),
                "contracts": sorted(set(relation_names.get("conforms_to", []) + relation_names.get("satisfies", []))),
                "superseded_by": sorted(set(relation_names.get("superseded_by", []))),
                "supersedes": supersedes,
                "superseded_at": artifact.get("superseded_at"),
                "supersession_authorized_by": artifact.get("supersession_authorized_by"),
                "lifecycle_class": lifecycle_class,
                "version": artifact.get("version"),
                "tag": artifact.get("tag"),
                "authority": "declared",
            }
        )
    return sorted(result, key=lambda item: (item["kind"], item["id"]))


def _is_number(value: Any) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def _sanitize_treatment(
    value: Any,
    dimensions: Sequence[dict[str, Any]],
    repository_root: Path,
    issues: list[str],
    label: str,
) -> dict[str, Any]:
    if not isinstance(value, dict):
        issues.append(f"treatment '{label}' must be an object")
        value = {}
    raw_scores = value.get("scores", {})
    if not isinstance(raw_scores, dict):
        issues.append(f"treatment '{label}' scores must be an object")
        raw_scores = {}
    scores: dict[str, int | float | None] = {}
    for dimension in dimensions:
        dimension_id = dimension["id"]
        score = raw_scores.get(dimension_id)
        if score is None:
            scores[dimension_id] = None
        elif not _is_number(score) or score < 0 or score > dimension["maximum"]:
            issues.append(f"treatment '{label}' score '{dimension_id}' is outside 0..{dimension['maximum']}")
            scores[dimension_id] = None
        else:
            scores[dimension_id] = score

    sanitized: dict[str, Any] = {"scores": scores}
    for measure in EXPERIMENT_MEASURES:
        raw = value.get(measure)
        if raw is None:
            sanitized[measure] = None
        elif not _is_number(raw) or raw < 0:
            issues.append(f"treatment '{label}' measure '{measure}' must be a non-negative number or null")
            sanitized[measure] = None
        else:
            sanitized[measure] = raw

    evidence: list[str] = []
    raw_evidence = value.get("evidence", [])
    if not isinstance(raw_evidence, list):
        issues.append(f"treatment '{label}' evidence must be an array")
    else:
        for entry in raw_evidence:
            safe = _safe_repository_reference(entry, repository_root)
            if safe is None:
                issues.append(f"treatment '{label}' contains an unsafe evidence path")
            else:
                evidence.append(safe)
    sanitized["evidence"] = sorted(set(evidence))
    return sanitized


def _import_experiment(path: Path, repository_root: Path) -> dict[str, Any]:
    relative_path = repository_relative(path, repository_root)
    issues: list[str] = []
    try:
        if path.stat().st_size > MAX_EXPERIMENT_BYTES:
            raise GenerationError("experiment result exceeds the one-megabyte input limit")
        payload = json.loads(path.read_text(encoding="utf-8-sig"))
    except (OSError, UnicodeError, json.JSONDecodeError, GenerationError) as exc:
        return {
            "path": relative_path,
            "status": "invalid",
            "issues": [str(exc)],
        }
    if not isinstance(payload, dict):
        return {
            "path": relative_path,
            "status": "invalid",
            "issues": ["experiment result must be a JSON object"],
        }

    def required_string(field: str) -> str | None:
        value = payload.get(field)
        if not isinstance(value, str) or not value.strip():
            issues.append(f"field '{field}' must be a non-empty string")
            return None
        return value.strip()

    schema = required_string("schema")
    if schema is not None and schema != EXPERIMENT_SCHEMA:
        issues.append(f"unsupported experiment schema '{schema}'")
    trial_id = required_string("trial_id")
    protocol = required_string("protocol")
    base_revision = required_string("base_revision")
    work_order = required_string("work_order")
    evaluator = required_string("evaluator")

    rubric = payload.get("rubric")
    dimensions: list[dict[str, Any]] = []
    rubric_id: str | None = None
    if not isinstance(rubric, dict):
        issues.append("field 'rubric' must be an object")
    else:
        raw_rubric_id = rubric.get("id")
        if isinstance(raw_rubric_id, str) and raw_rubric_id.strip():
            rubric_id = raw_rubric_id.strip()
        else:
            issues.append("field 'rubric.id' must be a non-empty string")
        raw_dimensions = rubric.get("dimensions")
        if not isinstance(raw_dimensions, list) or not raw_dimensions:
            issues.append("field 'rubric.dimensions' must be a non-empty array")
        else:
            seen_dimensions: set[str] = set()
            for index, dimension in enumerate(raw_dimensions):
                if not isinstance(dimension, dict):
                    issues.append(f"rubric dimension {index} must be an object")
                    continue
                dimension_id = dimension.get("id")
                maximum = dimension.get("maximum")
                if not isinstance(dimension_id, str) or not dimension_id.strip():
                    issues.append(f"rubric dimension {index} has an invalid id")
                    continue
                clean_id = dimension_id.strip()
                if clean_id in seen_dimensions:
                    issues.append(f"rubric dimension '{clean_id}' is duplicated")
                    continue
                if not _is_number(maximum) or maximum <= 0:
                    issues.append(f"rubric dimension '{clean_id}' has an invalid maximum")
                    continue
                seen_dimensions.add(clean_id)
                dimensions.append({"id": clean_id, "maximum": maximum})

    treatments = payload.get("treatments")
    if not isinstance(treatments, dict):
        issues.append("field 'treatments' must be an object")
        treatments = {}
    sanitized_treatments = {
        label: _sanitize_treatment(treatments.get(label), dimensions, repository_root, issues, label)
        for label in ("baseline", "harness")
    }
    result = {
        "path": relative_path,
        "status": "valid" if not issues else "invalid",
        "issues": sorted(set(issues)),
        "schema": schema,
        "trial_id": trial_id,
        "protocol": protocol,
        "base_revision": base_revision,
        "work_order": work_order,
        "rubric": {"id": rubric_id, "dimensions": dimensions},
        "treatments": sanitized_treatments,
        "evaluator": evaluator,
    }
    return result


def import_experiments(repository_root: Path) -> list[dict[str, Any]]:
    experiment_root = repository_root / DEFAULT_EXPERIMENT_ROOT
    if not experiment_root.exists():
        return []
    return sorted(
        (_import_experiment(path, repository_root) for path in experiment_root.glob("*.json") if path.is_file()),
        key=lambda item: (item.get("trial_id") or "", item["path"]),
    )


def build_snapshot(
    repository_root: Path,
    artifact_root: Path,
    report: ValidationReport,
) -> dict[str, Any]:
    normalized_artifacts = normalize_artifacts(report, repository_root)
    relations = sorted(
        [
            *build_declared_relations(report.artifacts),
            *build_architecture_transitive_relations(report.artifacts),
        ],
        key=lambda item: (
            item["source"],
            item["relation"],
            item["target"],
            item["authority"],
        ),
    )
    diagnostics = normalize_diagnostics(report, normalized_artifacts)
    observed_revision = git_revision(repository_root)
    revision_policy = load_revision_policy(repository_root)
    commit_availability = git_commit_availability(
        repository_root,
        [artifact.get("commit") for artifact in normalized_artifacts if isinstance(artifact.get("commit"), str)],
    )
    revision_provenance = build_revision_provenance(
        normalized_artifacts,
        relations,
        observed_revision,
        commit_availability,
    )
    evidence_by_work_order = discover_evidence(repository_root)
    evidence = [
        {"work_order": work_order, "paths": paths}
        for work_order, paths in sorted(evidence_by_work_order.items())
    ]
    findings = build_findings(
        normalized_artifacts,
        relations,
        diagnostics,
        evidence_by_work_order,
        revision_provenance,
        revision_policy,
    )
    return {
        "schema": SNAPSHOT_SCHEMA,
        "finding_rules_version": FINDING_RULES_VERSION,
        "quality_gates_version": QUALITY_GATES_VERSION,
        "repository": {
            "name": repository_root.name,
            "revision": observed_revision,
            "artifact_root": repository_relative(artifact_root, repository_root),
            "valid": report.valid,
        },
        "artifacts": normalized_artifacts,
        "relations": relations,
        "diagnostics": diagnostics,
        "findings": findings,
        "coverage": build_coverage(normalized_artifacts, relations),
        "readiness": build_readiness(
            normalized_artifacts,
            relations,
            diagnostics,
            evidence_by_work_order,
            revision_policy,
        ),
        "revision_provenance": revision_provenance,
        "revision_policy": revision_policy,
        "experiments": import_experiments(repository_root),
        "evidence": evidence,
    }


def generate_snapshot(
    repository_root: Path,
    artifact_root: Path | None = None,
) -> tuple[dict[str, Any], ValidationReport, Path]:
    resolved_repository = resolve_repository_root(repository_root)
    resolved_artifacts = resolve_artifact_root(resolved_repository, artifact_root)
    report = validate_repository(resolved_repository, resolved_artifacts)
    return build_snapshot(resolved_repository, resolved_artifacts, report), report, resolved_artifacts


def serialize_json(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n"


def _safe_embedded_json(value: Any) -> str:
    payload = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
    replacements = {
        "&": "\\u0026",
        "<": "\\u003c",
        ">": "\\u003e",
        "\u2028": "\\u2028",
        "\u2029": "\\u2029",
    }
    for source, replacement in replacements.items():
        payload = payload.replace(source, replacement)
    return payload


def render_dashboard(snapshot: dict[str, Any]) -> str:
    template_path = Path(__file__).resolve().parent / "harness_explorer" / "index.template.html"
    try:
        template = template_path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as exc:
        raise GenerationError("owned dashboard template is unavailable or unreadable") from exc
    marker = "__HARNESS_SNAPSHOT_JSON__"
    if template.count(marker) != 1:
        raise GenerationError("owned dashboard template must contain exactly one snapshot marker")
    return template.replace(marker, _safe_embedded_json(snapshot))


def _sha256(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def _safe_remove_tree(path: Path, parent: Path) -> None:
    resolved_path = path.resolve()
    resolved_parent = parent.resolve()
    if resolved_path == resolved_parent or not _is_within(resolved_path, resolved_parent):
        raise GenerationError("refusing to remove a path outside the intended output parent")
    if path.exists():
        shutil.rmtree(path)


def write_output_transactionally(output_root: Path, files: dict[str, str]) -> None:
    output_parent = output_root.parent.resolve()
    output_parent.mkdir(parents=True, exist_ok=True)
    if output_root.exists() and output_root.is_symlink():
        raise GenerationError("output root became a symbolic link")
    if output_root.exists() and not output_root.is_dir():
        raise GenerationError("output root became a non-directory path")

    temporary = Path(tempfile.mkdtemp(prefix=f".{output_root.name}.next-", dir=output_parent))
    backup: Path | None = None
    promoted = False
    try:
        for name, content in sorted(files.items()):
            destination = temporary / name
            destination.write_text(content, encoding="utf-8", newline="\n")
        expected = set(files)
        actual = {path.name for path in temporary.iterdir() if path.is_file()}
        if actual != expected:
            raise GenerationError("temporary output is incomplete")

        if output_root.exists():
            backup = Path(tempfile.mkdtemp(prefix=f".{output_root.name}.previous-", dir=output_parent))
            backup.rmdir()
            output_root.replace(backup)
        temporary.replace(output_root)
        promoted = True
        if backup is not None:
            _safe_remove_tree(backup, output_parent)
    except Exception:
        if not promoted and backup is not None and backup.exists() and not output_root.exists():
            backup.replace(output_root)
        raise
    finally:
        if temporary.exists():
            _safe_remove_tree(temporary, output_parent)
        if promoted and backup is not None and backup.exists():
            _safe_remove_tree(backup, output_parent)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Generate a deterministic, offline engineering-harness dashboard."
    )
    parser.add_argument("--root", type=Path, required=True, help="Repository root.")
    parser.add_argument(
        "--artifact-root",
        type=Path,
        default=None,
        help="Artifact directory below --root; default: docs/engineering.",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=None,
        help="Output directory; default: target/harness-dashboard below --root.",
    )
    return parser


def _display_output(output_root: Path, repository_root: Path) -> str:
    if _is_within(output_root, repository_root):
        return output_root.relative_to(repository_root).as_posix()
    return "<explicit-external-output>"


def main(argv: Iterable[str] | None = None) -> int:
    started = time.perf_counter()
    args = build_parser().parse_args(list(argv) if argv is not None else None)
    try:
        repository_root = resolve_repository_root(args.root)
        artifact_root = resolve_artifact_root(repository_root, args.artifact_root)
        output_root = resolve_output_root(repository_root, artifact_root, args.output)
        report = validate_repository(repository_root, artifact_root)
        snapshot = build_snapshot(repository_root, artifact_root, report)
        snapshot_text = serialize_json(snapshot)
        dashboard_text = render_dashboard(snapshot)
        outcome = "generated-valid" if report.valid else "generated-invalid"
        summary = {
            "schema": "harness-dashboard-generation-v1",
            "outcome": outcome,
            "generated_at": datetime.now(UTC).isoformat().replace("+00:00", "Z"),
            "repository_revision": snapshot["repository"]["revision"],
            "artifact_count": len(snapshot["artifacts"]),
            "relation_count": len(snapshot["relations"]),
            "validator_error_count": len(report.errors),
            "warning_count": sum(1 for item in snapshot["findings"] if item["severity"] == "warning"),
            "output": _display_output(output_root, repository_root),
            "snapshot_sha256": _sha256(snapshot_text),
            "dashboard_sha256": _sha256(dashboard_text),
            "elapsed_ms": int((time.perf_counter() - started) * 1000),
        }
        write_output_transactionally(
            output_root,
            {
                "dashboard-data.json": snapshot_text,
                "generation-summary.json": serialize_json(summary),
                "index.html": dashboard_text,
            },
        )
        label = "PASS" if report.valid else "INVALID"
        print(
            "Harness Explorer generation: "
            f"{label} | Artifacts: {len(snapshot['artifacts'])} | "
            f"Relations: {len(snapshot['relations'])} | "
            f"Errors: {len(report.errors)} | "
            f"Warnings: {summary['warning_count']} | "
            f"Output: {summary['output']} | "
            f"Snapshot: {summary['snapshot_sha256']}"
        )
        for diagnostic in report.errors:
            print(f"[{diagnostic.code}] {diagnostic.path}: {diagnostic.message}", file=sys.stderr)
        return 0 if report.valid else 1
    except (GenerationError, OSError, UnicodeError, json.JSONDecodeError) as exc:
        print(f"Harness Explorer generation failed: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
