"""Portable canonical artifact-layout registry for repository-local scripts."""

from __future__ import annotations

import re
from pathlib import Path


ARTIFACT_DIRECTORIES: dict[str, tuple[str, ...]] = {
    "intent": ("intent",),
    "capability": ("capabilities",),
    "requirement": ("requirements",),
    "specification": ("specifications",),
    "architecture": ("architecture",),
    "adr": ("architecture", "adr"),
    "verification": ("verification",),
    "work_order": ("work-orders",),
    "verification_record": ("verification-records",),
    "release_contract": ("release",),
    "release_record": ("releases",),
    "operating_contract": ("operations",),
}

ARTIFACT_PREFIXES = {
    "intent": "INT-",
    "capability": "CAP-",
    "requirement": "REQ-",
    "specification": "SPEC-",
    "architecture": "ARCH-",
    "adr": "ADR-",
    "verification": "VER-",
    "work_order": "WO-",
    "verification_record": "VREC-",
    "release_contract": "REL-",
    "release_record": "RLS-",
    "operating_contract": "OPS-",
}

DOMAIN_PATTERN = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
RESERVED_DOMAINS = frozenset(
    {
        ".agents", ".codex", ".git", ".idea", "acceptance", "architecture",
        "capabilities", "evidence", "experiments", "intent", "node_modules",
        "operations", "release", "releases", "requirements", "specifications",
        "target", "templates", "verification", "verification-records", "work-orders",
    }
)


def artifact_domain_from_relative_path(value: str | Path) -> str | None:
    raw = Path(value)
    if raw.is_absolute() or "\\" in str(value):
        return None
    parts = raw.parts
    if len(parts) < 4 or parts[:2] != ("docs", "engineering"):
        return None
    candidate = parts[2]
    if len(candidate) > 64 or DOMAIN_PATTERN.fullmatch(candidate) is None or candidate in RESERVED_DOMAINS:
        return None
    return candidate


def common_artifact_domain(paths: list[str | Path]) -> str | None:
    if not paths:
        return None
    domains = [artifact_domain_from_relative_path(path) for path in paths]
    if any(domain is None for domain in domains):
        return None
    unique = set(domains)
    return next(iter(unique)) if len(unique) == 1 else None


def canonical_artifact_relative_path(domain: str, artifact_type: str, artifact_id: str) -> Path:
    return Path("docs") / "engineering" / domain / Path(*ARTIFACT_DIRECTORIES[artifact_type]) / f"{artifact_id}.md"


def repository_record_relative_path(artifact_type: str, artifact_id: str, domain: str | None) -> Path:
    if domain is not None:
        return canonical_artifact_relative_path(domain, artifact_type, artifact_id)
    return Path("docs") / "engineering" / Path(*ARTIFACT_DIRECTORIES[artifact_type]) / f"{artifact_id}.md"
