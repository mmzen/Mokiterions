#!/usr/bin/env python3
"""Validate and invoke the closed Phase 4 work-order evaluator client."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import PurePosixPath
from typing import Any, Callable, Iterable, Mapping, Sequence


SKILL = "harness-execute-work-order"
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
ALLOWED_EFFECTS = {"implementation-write", "test-execution", "evidence-write"}
ARTIFACT_ID = re.compile(r"WO-[A-Z0-9]+-[0-9]+")
CONTROL = re.compile(r"[\x00-\x1f\x7f]")
MAX_PATHS = 512
MAX_ARGUMENTS = 256


class ScopeGuardError(ValueError):
    """A bounded work-order scope rejection."""

    def __init__(self, code: str, message: str) -> None:
        super().__init__(f"{code}: {message}")
        self.code = code


def portable_path(value: Any, *, allow_prefix: bool = False) -> str:
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
        raise ScopeGuardError("AEXEXE007", "path is not portable")
    trailing = value.endswith("/")
    candidate = value[:-1] if trailing else value
    path = PurePosixPath(candidate)
    if path.is_absolute() or candidate.startswith("/") or any(part in {"", ".", ".."} for part in path.parts):
        raise ScopeGuardError("AEXEXE007", "path escapes or is ambiguous")
    return path.as_posix() + ("/" if trailing and allow_prefix else "")


def _paths(values: Iterable[Any], *, allow_prefix: bool = False) -> tuple[str, ...]:
    result = tuple(portable_path(value, allow_prefix=allow_prefix) for value in values)
    if not result or len(result) > MAX_PATHS or len({item.casefold() for item in result}) != len(result):
        raise ScopeGuardError("AEXEXE008", "paths are empty, excessive, duplicate, or case-ambiguous")
    return result


def _admitted(path: str, scope: tuple[str, ...]) -> bool:
    folded = path.casefold()
    for item in scope:
        item_folded = item.casefold()
        if item.endswith("/") and folded.startswith(item_folded):
            return True
        if folded == item_folded:
            return True
    return False


def scope_digest(scope: tuple[str, ...]) -> str:
    raw = json.dumps(list(scope), ensure_ascii=False, separators=(",", ":")).encode("utf-8") + b"\n"
    return hashlib.sha256(raw).hexdigest()


def _client_arguments(value: Any) -> tuple[str, ...]:
    if not isinstance(value, dict) or set(value) != {"schema", "arguments", "delegation_sha256"}:
        raise ScopeGuardError("AEXEXE011", "evaluator request differs from the closed client schema")
    if value["schema"] != REQUEST_SCHEMA:
        raise ScopeGuardError("AEXEXE011", "evaluator request uses the wrong schema")
    digest = value["delegation_sha256"]
    if not isinstance(digest, str) or re.fullmatch(r"[0-9a-f]{64}", digest) is None:
        raise ScopeGuardError("AEXEXE011", "delegation evidence digest is invalid")
    raw = value["arguments"]
    if not isinstance(raw, list) or not 2 <= len(raw) <= MAX_ARGUMENTS:
        raise ScopeGuardError("AEXEXE011", "evaluator arguments must be a bounded array")
    arguments: list[str] = []
    for item in raw:
        if not isinstance(item, str) or not item or len(item) > 4096 or CONTROL.search(item):
            raise ScopeGuardError("AEXEXE011", "evaluator argument is invalid")
        arguments.append(item)
    if arguments[:2] != ["delegated-workflow", "execute"]:
        raise ScopeGuardError("AEXEXE011", "evaluator request does not select delegated-workflow execute")
    return tuple(arguments)


def _require_catalog(value: Sequence[Mapping[str, Any]]) -> None:
    try:
        actual = tuple(item["id"] for item in value)
    except (KeyError, TypeError):
        raise ScopeGuardError("AEXEXE012", "released evaluator catalog is unavailable or invalid") from None
    if actual != PHASE4_CATALOG:
        raise ScopeGuardError("AEXEXE012", "released evaluator lacks the exact Phase 4 catalog")


def invoke_work_order_client(
    request: Mapping[str, Any],
    *,
    catalog: Callable[[], Sequence[Mapping[str, Any]]],
    client: Callable[[tuple[str, ...]], Mapping[str, Any]],
) -> Mapping[str, Any]:
    """Invoke only delegated-workflow execute after closed client checks."""

    fields = {
        "schema", "explicit_skill", "workflow_schema", "interface_operation",
        "direct_target_write", "work_order", "state", "effect_class", "planned_paths",
        "execution_scope", "evaluator_request",
    }
    if set(request) != fields:
        raise ScopeGuardError("AEXEXE001", "request fields differ from the closed scope guard input")
    if request["schema"] != REQUEST_SCHEMA or request["workflow_schema"] != WORKFLOW_SCHEMA:
        raise ScopeGuardError("AEXEXE001", "request schema or workflow capability is invalid")
    if request["explicit_skill"] != SKILL:
        raise ScopeGuardError("AEXEXE002", "explicit harness-execute-work-order activation is required")
    if request["interface_operation"] != INTERFACE_OPERATION:
        raise ScopeGuardError("AEXEXE005", "request selects an unsupported evaluator interface")
    if request["direct_target_write"] is not False:
        raise ScopeGuardError("AEXEXE013", "direct governed-target writes are prohibited")
    if not isinstance(request["work_order"], str) or ARTIFACT_ID.fullmatch(request["work_order"]) is None:
        raise ScopeGuardError("AEXEXE003", "one valid work-order ID is required")
    if request["state"] != "approved":
        raise ScopeGuardError("AEXEXE004", "delegated workflow execution requires an approved work order")
    if request["effect_class"] not in ALLOWED_EFFECTS:
        raise ScopeGuardError("AEXEXE005", "effect class is not admitted")
    if not isinstance(request["planned_paths"], list) or not isinstance(request["execution_scope"], list):
        raise ScopeGuardError("AEXEXE006", "path sets must be arrays")
    planned = _paths(request["planned_paths"])
    scope = _paths(request["execution_scope"], allow_prefix=True)
    if any(not _admitted(path, scope) for path in planned):
        raise ScopeGuardError("AEXEXE009", "planned path is outside the current execution scope")
    arguments = _client_arguments(request["evaluator_request"])
    _require_catalog(catalog())
    result = client(arguments)
    required = {"outcome", "work_order", "start", "effects", "completion", "next"}
    if not isinstance(result, Mapping) or set(result) != required:
        raise ScopeGuardError("AEXEXE014", "evaluator returned an invalid delegated-workflow result")
    if result["outcome"] != "completed-at-git-stop" or result["work_order"] != request["work_order"]:
        raise ScopeGuardError("AEXEXE014", "evaluator did not reach the governed Git stop")
    if not isinstance(result["effects"], list) or not result["effects"]:
        raise ScopeGuardError("AEXEXE014", "evaluator result lacks an admitted bundle receipt")
    if not isinstance(result["next"], Mapping) or "decision_packet" not in result["next"]:
        raise ScopeGuardError("AEXEXE014", "evaluator result lacks the terminal decision packet")
    return {
        "schema": RESULT_SCHEMA,
        "outcome": result["outcome"],
        "interface_operation": INTERFACE_OPERATION,
        "planned_paths": list(planned),
        "scope_sha256": scope_digest(scope),
        "evaluator_result": dict(result),
    }


def _load_request(path: str) -> dict[str, Any]:
    try:
        with open(path, "r", encoding="utf-8") as handle:
            value = json.load(handle)
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise ScopeGuardError("AEXEXE011", "request is not readable UTF-8 JSON") from exc
    if not isinstance(value, dict):
        raise ScopeGuardError("AEXEXE011", "request JSON must be an object")
    return value


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--request-json", required=True)
    args = parser.parse_args(argv)
    try:
        request = _load_request(args.request_json)
        scope = _paths(request.get("execution_scope", []), allow_prefix=True)
        result = invoke_work_order_client(
            request,
            catalog=lambda: tuple({"id": item} for item in PHASE4_CATALOG),
            client=lambda arguments: {
                "outcome": "completed-at-git-stop",
                "work_order": request.get("work_order"),
                "start": {}, "effects": [{"receipt": {}}], "completion": {},
                "next": {"decision_packet": {}, "arguments": list(arguments)},
            },
        )
        result = dict(result)
        result["evaluator_invoked"] = False
        print(json.dumps(result, sort_keys=True, separators=(",", ":")))
        return 0
    except ScopeGuardError as exc:
        print(json.dumps({"code": exc.code, "outcome": "blocked"}, sort_keys=True, separators=(",", ":")))
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
