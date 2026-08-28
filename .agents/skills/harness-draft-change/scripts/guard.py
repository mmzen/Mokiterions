#!/usr/bin/env python3
"""Validate and invoke the closed Phase 4 draft evaluator-client boundary."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import PurePosixPath
from typing import Any, Callable, Iterable, Mapping, Sequence


SKILL = "harness-draft-change"
REQUEST_SCHEMA = "se-harness-evaluator-client-request-v1"
RESULT_SCHEMA = "se-harness-evaluator-client-result-v1"
WORKFLOW_SCHEMA = "se-harness-workflow-v4"
INTERFACE_OPERATION = "delegated-workflow-execute"
PHASE4_CATALOG = (
    "delegated-work-order-start",
    "change-bundle-apply",
    "delegated-work-order-complete",
    "delegated-vrec-prepare",
)
ALLOWED_EFFECTS = {"draft-create", "draft-revise", "planning-note-write"}
ARTIFACT_ID = re.compile(r"[A-Z][A-Z0-9]*-[A-Z0-9]+-[0-9]+")
CONTROL = re.compile(r"[\x00-\x1f\x7f]")
MAX_PATHS = 128
MAX_ARGUMENTS = 256


class DraftGuardError(ValueError):
    """A bounded draft-plan rejection."""

    def __init__(self, code: str, message: str) -> None:
        super().__init__(f"{code}: {message}")
        self.code = code


def portable_path(value: Any) -> str:
    if (
        not isinstance(value, str)
        or not value
        or len(value) > 4096
        or CONTROL.search(value)
        or "\\" in value
        or "://" in value
        or "*" in value
        or "?" in value
    ):
        raise DraftGuardError("AEXDRF007", "planned path is not portable")
    path = PurePosixPath(value)
    if path.is_absolute() or value.startswith("/") or any(part in {"", ".", ".."} for part in path.parts):
        raise DraftGuardError("AEXDRF007", "planned path escapes or is ambiguous")
    return path.as_posix()


def _closed_paths(values: Iterable[Any]) -> tuple[str, ...]:
    result = tuple(portable_path(value) for value in values)
    if not result or len(result) > MAX_PATHS or len({item.casefold() for item in result}) != len(result):
        raise DraftGuardError("AEXDRF008", "planned paths are empty, excessive, duplicate, or case-ambiguous")
    return result


def _client_arguments(value: Any) -> tuple[str, ...]:
    if not isinstance(value, dict) or set(value) != {"schema", "arguments", "delegation_sha256"}:
        raise DraftGuardError("AEXDRF013", "evaluator request differs from the closed client schema")
    if value["schema"] != REQUEST_SCHEMA:
        raise DraftGuardError("AEXDRF013", "evaluator request uses the wrong schema")
    digest = value["delegation_sha256"]
    if not isinstance(digest, str) or re.fullmatch(r"[0-9a-f]{64}", digest) is None:
        raise DraftGuardError("AEXDRF013", "delegation evidence digest is invalid")
    raw = value["arguments"]
    if not isinstance(raw, list) or not 2 <= len(raw) <= MAX_ARGUMENTS:
        raise DraftGuardError("AEXDRF013", "evaluator arguments must be a bounded array")
    arguments: list[str] = []
    for item in raw:
        if not isinstance(item, str) or not item or len(item) > 4096 or CONTROL.search(item):
            raise DraftGuardError("AEXDRF013", "evaluator argument is invalid")
        arguments.append(item)
    if arguments[:2] != ["delegated-workflow", "execute"]:
        raise DraftGuardError("AEXDRF013", "evaluator request does not select delegated-workflow execute")
    return tuple(arguments)


def _require_catalog(value: Sequence[Mapping[str, Any]]) -> None:
    try:
        actual = tuple(item["id"] for item in value)
    except (KeyError, TypeError):
        raise DraftGuardError("AEXDRF014", "released evaluator catalog is unavailable or invalid") from None
    if actual != PHASE4_CATALOG:
        raise DraftGuardError("AEXDRF014", "released evaluator lacks the exact Phase 4 catalog")


def invoke_draft_client(
    request: Mapping[str, Any],
    *,
    catalog: Callable[[], Sequence[Mapping[str, Any]]],
    client: Callable[[tuple[str, ...]], Mapping[str, Any]],
) -> Mapping[str, Any]:
    """Invoke only the evaluator client after closed non-authoritative checks."""

    expected_fields = {
        "schema", "explicit_skill", "workflow_schema", "interface_operation",
        "direct_target_write", "work_order", "state", "effect_class", "planned_paths",
        "allowed_paths", "revisions", "evaluator_request",
    }
    if set(request) != expected_fields:
        raise DraftGuardError("AEXDRF001", "request fields differ from the closed draft guard input")
    if request["schema"] != REQUEST_SCHEMA or request["workflow_schema"] != WORKFLOW_SCHEMA:
        raise DraftGuardError("AEXDRF001", "request schema or workflow capability is invalid")
    if request["explicit_skill"] != SKILL:
        raise DraftGuardError("AEXDRF002", "explicit harness-draft-change activation is required")
    if request["interface_operation"] != INTERFACE_OPERATION:
        raise DraftGuardError("AEXDRF003", "request selects an unsupported evaluator interface")
    if request["direct_target_write"] is not False:
        raise DraftGuardError("AEXDRF015", "direct governed-target writes are prohibited")
    if not isinstance(request["work_order"], str) or re.fullmatch(r"WO-[A-Z0-9]+-[0-9]+", request["work_order"]) is None:
        raise DraftGuardError("AEXDRF016", "one valid delegated work order is required")
    if request["state"] != "approved":
        raise DraftGuardError("AEXDRF016", "delegated workflow execution requires an approved work order")
    if request["effect_class"] not in ALLOWED_EFFECTS:
        raise DraftGuardError("AEXDRF003", "effect class is not admitted")
    if not isinstance(request["planned_paths"], list) or not isinstance(request["allowed_paths"], list):
        raise DraftGuardError("AEXDRF004", "path sets must be arrays")
    planned = _closed_paths(request["planned_paths"])
    allowed = _closed_paths(request["allowed_paths"])
    if not set(planned).issubset(allowed):
        raise DraftGuardError("AEXDRF009", "planned path is outside declared draft destinations")
    if request["effect_class"] == "planning-note-write" and (
        len(planned) != 1 or not planned[0].startswith("docs/notes/") or not planned[0].endswith(".md")
    ):
        raise DraftGuardError("AEXDRF010", "planning-note effect requires one declared Markdown note")
    revisions = request["revisions"]
    if not isinstance(revisions, dict) or len(revisions) > MAX_PATHS:
        raise DraftGuardError("AEXDRF005", "revisions must be a bounded object")
    for artifact, state in revisions.items():
        if not isinstance(artifact, str) or ARTIFACT_ID.fullmatch(artifact) is None or state != "draft":
            raise DraftGuardError("AEXDRF011", "every selected revision must identify a current draft")
    arguments = _client_arguments(request["evaluator_request"])
    _require_catalog(catalog())
    result = client(arguments)
    required = {"outcome", "work_order", "start", "effects", "completion", "next"}
    if not isinstance(result, Mapping) or set(result) != required:
        raise DraftGuardError("AEXDRF017", "evaluator returned an invalid delegated-workflow result")
    if result["outcome"] != "completed-at-git-stop" or result["work_order"] != request["work_order"]:
        raise DraftGuardError("AEXDRF017", "evaluator did not reach the governed Git stop")
    if not isinstance(result["next"], Mapping) or "decision_packet" not in result["next"]:
        raise DraftGuardError("AEXDRF017", "evaluator result lacks the terminal decision packet")
    return {
        "schema": RESULT_SCHEMA,
        "outcome": result["outcome"],
        "interface_operation": INTERFACE_OPERATION,
        "planned_paths": list(planned),
        "evaluator_result": dict(result),
    }


def _load_request(path: str) -> dict[str, Any]:
    try:
        value = json.loads(open(path, "r", encoding="utf-8").read())
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise DraftGuardError("AEXDRF006", "request is not readable UTF-8 JSON") from exc
    if not isinstance(value, dict):
        raise DraftGuardError("AEXDRF006", "request JSON must be an object")
    return value


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--request-json", required=True)
    args = parser.parse_args(argv)
    try:
        request = _load_request(args.request_json)
        result = invoke_draft_client(
            request,
            catalog=lambda: tuple({"id": item} for item in PHASE4_CATALOG),
            client=lambda arguments: {
                "outcome": "completed-at-git-stop",
                "work_order": request.get("work_order"),
                "start": {}, "effects": [], "completion": {},
                "next": {"decision_packet": {}, "arguments": list(arguments)},
            },
        )
        result = dict(result)
        result["evaluator_invoked"] = False
        print(json.dumps(result, sort_keys=True, separators=(",", ":")))
        return 0
    except DraftGuardError as exc:
        print(json.dumps({"code": exc.code, "outcome": "blocked"}, sort_keys=True, separators=(",", ":")))
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
