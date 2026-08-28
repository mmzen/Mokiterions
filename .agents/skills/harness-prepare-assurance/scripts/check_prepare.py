#!/usr/bin/env python3
"""Validate and invoke the closed Phase 4 assurance evaluator client."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import PurePosixPath
from typing import Any, Callable, Mapping, Sequence


SKILL = "harness-prepare-assurance"
REQUEST_SCHEMA = "se-harness-evaluator-client-request-v1"
RESULT_SCHEMA = "se-harness-evaluator-client-result-v1"
WORKFLOW_SCHEMA = "se-harness-workflow-v4"
INTERFACE_OPERATION = "delegated-workflow-prepare-vrec"
PHASE4_CATALOG = (
    "delegated-work-order-start",
    "change-bundle-apply",
    "delegated-work-order-complete",
    "delegated-vrec-prepare",
)
VREC_ID = re.compile(r"VREC-[A-Z0-9]+-[0-9]+")
WORK_ORDER_ID = re.compile(r"WO-[A-Z0-9]+-[0-9]+")
COMMIT = re.compile(r"[0-9a-f]{40}|[0-9a-f]{64}")
CONTROL = re.compile(r"[\x00-\x1f\x7f]")
MAX_ARGUMENTS = 256


class AssuranceGuardError(ValueError):
    """A bounded assurance-preparation rejection."""

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
        raise AssuranceGuardError("AEXASR007", "record destination is not portable")
    path = PurePosixPath(value)
    if path.is_absolute() or value.startswith("/") or any(part in {"", ".", ".."} for part in path.parts):
        raise AssuranceGuardError("AEXASR007", "record destination escapes or is ambiguous")
    selected = path.as_posix()
    if not selected.startswith("docs/engineering/") or not selected.endswith(".md"):
        raise AssuranceGuardError("AEXASR007", "record destination is not a canonical engineering Markdown path")
    return selected


def _client_arguments(value: Any) -> tuple[str, ...]:
    if not isinstance(value, dict) or set(value) != {"schema", "arguments", "delegation_sha256"}:
        raise AssuranceGuardError("AEXASR009", "evaluator request differs from the closed client schema")
    if value["schema"] != REQUEST_SCHEMA:
        raise AssuranceGuardError("AEXASR009", "evaluator request uses the wrong schema")
    digest = value["delegation_sha256"]
    if not isinstance(digest, str) or re.fullmatch(r"[0-9a-f]{64}", digest) is None:
        raise AssuranceGuardError("AEXASR009", "delegation evidence digest is invalid")
    raw = value["arguments"]
    if not isinstance(raw, list) or not 2 <= len(raw) <= MAX_ARGUMENTS:
        raise AssuranceGuardError("AEXASR009", "evaluator arguments must be a bounded array")
    arguments: list[str] = []
    for item in raw:
        if not isinstance(item, str) or not item or len(item) > 4096 or CONTROL.search(item):
            raise AssuranceGuardError("AEXASR009", "evaluator argument is invalid")
        arguments.append(item)
    if arguments[:2] != ["delegated-workflow", "prepare-vrec"]:
        raise AssuranceGuardError("AEXASR009", "evaluator request does not select delegated-workflow prepare-vrec")
    return tuple(arguments)


def _require_catalog(value: Sequence[Mapping[str, Any]]) -> None:
    try:
        actual = tuple(item["id"] for item in value)
    except (KeyError, TypeError):
        raise AssuranceGuardError("AEXASR010", "released evaluator catalog is unavailable or invalid") from None
    if actual != PHASE4_CATALOG:
        raise AssuranceGuardError("AEXASR010", "released evaluator lacks the exact Phase 4 catalog")


def invoke_assurance_client(
    request: Mapping[str, Any],
    *,
    catalog: Callable[[], Sequence[Mapping[str, Any]]],
    client: Callable[[tuple[str, ...]], Mapping[str, Any]],
) -> Mapping[str, Any]:
    """Invoke only delegated VREC preparation after closed client checks."""

    fields = {
        "schema", "explicit_skill", "workflow_schema", "interface_operation",
        "direct_target_write", "work_order", "state", "record_id",
        "record_destination", "candidate_commit", "record_exists",
        "preparation_actor", "completion_proof", "evaluator_request",
    }
    if set(request) != fields:
        raise AssuranceGuardError("AEXASR001", "request fields differ from the closed assurance guard input")
    if request["schema"] != REQUEST_SCHEMA or request["workflow_schema"] != WORKFLOW_SCHEMA:
        raise AssuranceGuardError("AEXASR001", "request schema or workflow capability is invalid")
    if request["explicit_skill"] != SKILL:
        raise AssuranceGuardError("AEXASR002", "explicit harness-prepare-assurance activation is required")
    if request["interface_operation"] != INTERFACE_OPERATION:
        raise AssuranceGuardError("AEXASR005", "request selects an unsupported evaluator interface")
    if request["direct_target_write"] is not False:
        raise AssuranceGuardError("AEXASR011", "direct governed-target writes are prohibited")
    if not isinstance(request["work_order"], str) or WORK_ORDER_ID.fullmatch(request["work_order"]) is None:
        raise AssuranceGuardError("AEXASR003", "one valid work-order ID is required")
    if request["state"] != "implemented":
        raise AssuranceGuardError("AEXASR005", "delegated VREC preparation requires implemented work")
    if not isinstance(request["record_id"], str) or VREC_ID.fullmatch(request["record_id"]) is None:
        raise AssuranceGuardError("AEXASR003", "one valid VREC identifier is required")
    candidate = request["candidate_commit"]
    if candidate is not None and (not isinstance(candidate, str) or COMMIT.fullmatch(candidate) is None):
        raise AssuranceGuardError("AEXASR004", "candidate commit must be absent or an exact lowercase identity")
    if request["record_exists"] is not False:
        raise AssuranceGuardError("AEXASR005", "the VREC identifier must be unused")
    actor = request["preparation_actor"]
    if not isinstance(actor, str) or not actor.strip() or len(actor) > 256 or CONTROL.search(actor):
        raise AssuranceGuardError("AEXASR006", "an explicit bounded preparation actor is required")
    destination = portable_path(request["record_destination"])
    if not isinstance(request["completion_proof"], Mapping) or not request["completion_proof"]:
        raise AssuranceGuardError("AEXASR008", "the complete delegated completion proof is required")
    arguments = _client_arguments(request["evaluator_request"])
    _require_catalog(catalog())
    result = client(arguments)
    if not isinstance(result, Mapping) or result.get("outcome") not in {"stopped", "prepared"}:
        raise AssuranceGuardError("AEXASR012", "evaluator returned an invalid VREC-preparation result")
    expected = (
        {"outcome", "result", "decision_packet"}
        if result["outcome"] == "stopped"
        else {"outcome", "record", "receipt", "result", "decision_packet"}
    )
    if set(result) != expected or not isinstance(result["decision_packet"], Mapping):
        raise AssuranceGuardError("AEXASR012", "evaluator result lacks the terminal decision packet")
    if result["outcome"] == "prepared" and result["record"] != destination:
        raise AssuranceGuardError("AEXASR012", "evaluator prepared an unexpected VREC destination")
    return {
        "schema": RESULT_SCHEMA,
        "outcome": result["outcome"],
        "interface_operation": INTERFACE_OPERATION,
        "evaluator_result": dict(result),
    }


def _load_request(path: str) -> dict[str, Any]:
    try:
        with open(path, "r", encoding="utf-8") as handle:
            value = json.load(handle)
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise AssuranceGuardError("AEXASR009", "request is not readable UTF-8 JSON") from exc
    if not isinstance(value, dict):
        raise AssuranceGuardError("AEXASR009", "request JSON must be an object")
    return value


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--request-json", required=True)
    args = parser.parse_args(argv)
    try:
        request = _load_request(args.request_json)
        destination = portable_path(request.get("record_destination"))
        result = invoke_assurance_client(
            request,
            catalog=lambda: tuple({"id": item} for item in PHASE4_CATALOG),
            client=lambda arguments: {
                "outcome": "stopped", "result": {},
                "decision_packet": {"arguments": list(arguments), "record": destination},
            },
        )
        result = dict(result)
        result["evaluator_invoked"] = False
        print(json.dumps(result, sort_keys=True, separators=(",", ":")))
        return 0
    except AssuranceGuardError as exc:
        print(json.dumps({"code": exc.code, "outcome": "blocked"}, sort_keys=True, separators=(",", ":")))
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
