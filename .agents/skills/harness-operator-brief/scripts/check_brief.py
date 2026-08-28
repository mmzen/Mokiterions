#!/usr/bin/env python3
"""Validate a bounded operator brief and exact protected-content bindings."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any, Iterable, Mapping, Sequence


SKILL = "harness-operator-brief"
PROFILE = "operator-communication"
SOURCE_KINDS = {"structured-harness-result", "bounded-technical-text"}
PROTECTED_KINDS = {
    "acceptance-threshold",
    "canonical-restitution-block",
    "code",
    "command",
    "decision-meaning",
    "diagnostic",
    "digest",
    "established-terminology",
    "evaluator-output",
    "evidence",
    "field-name",
    "formula",
    "identifier",
    "legal-qualification",
    "lifecycle-meaning",
    "log",
    "machine-data",
    "normative-statement",
    "operator-supplied-text",
    "path",
    "quotation",
    "safety-qualification",
    "schema",
    "url",
    "version",
}
SHA256 = re.compile(r"[0-9a-f]{64}")
IDENTIFIER = re.compile(r"[a-z0-9](?:[a-z0-9-]{0,126}[a-z0-9])?")
MAX_SOURCE_BYTES = 262_144
MAX_RENDERED_BYTES = 524_288
MAX_REQUEST_BYTES = 1_048_576
MAX_SPANS = 256
REQUEST_FIELDS = {
    "explicit_skill",
    "profile",
    "source_kind",
    "source_text",
    "source_sha256",
    "protected_spans",
    "rendered_text",
    "bindings",
    "changed_paths",
}
SPAN_FIELDS = {"id", "kind", "start", "end", "sha256"}
BINDING_FIELDS = {"id", "start", "end", "sha256"}


class BriefCheckError(ValueError):
    """A stable, bounded technical-communication diagnostic."""

    def __init__(self, code: str, message: str) -> None:
        super().__init__(f"{code}: {message}")
        self.code = code


def _duplicate_keys(pairs: Iterable[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise BriefCheckError("TCM001", "request JSON contains a duplicate key")
        result[key] = value
    return result


def _closed_object(value: Any, fields: set[str], code: str) -> Mapping[str, Any]:
    if not isinstance(value, dict) or set(value) != fields:
        raise BriefCheckError(code, "object fields differ from the closed schema")
    return value


def _bounded_text(value: Any, *, limit: int, code: str) -> tuple[str, bytes]:
    if not isinstance(value, str):
        raise BriefCheckError(code, "value must be text")
    try:
        encoded = value.encode("utf-8")
    except UnicodeEncodeError as exc:
        raise BriefCheckError(code, "value is not valid UTF-8 text") from exc
    if len(encoded) > limit:
        raise BriefCheckError(code, "text exceeds its byte limit")
    for character in value:
        if ord(character) < 32 and character not in "\t\n\r":
            raise BriefCheckError(code, "text contains a prohibited control character")
        if ord(character) == 127:
            raise BriefCheckError(code, "text contains a prohibited control character")
    return value, encoded


def _digest(value: Any, code: str) -> str:
    if not isinstance(value, str) or SHA256.fullmatch(value) is None:
        raise BriefCheckError(code, "digest must be lowercase SHA-256")
    return value


def _offset(value: Any, code: str) -> int:
    if not isinstance(value, int) or isinstance(value, bool) or value < 0:
        raise BriefCheckError(code, "offset must be a non-negative integer")
    return value


def _validate_spans(value: Any, source: bytes) -> tuple[tuple[Mapping[str, Any], bytes], ...]:
    if not isinstance(value, list) or len(value) > MAX_SPANS:
        raise BriefCheckError("TCM007", "protected spans must be a bounded array")
    result: list[tuple[Mapping[str, Any], bytes]] = []
    seen: set[str] = set()
    previous_end = 0
    for raw in value:
        span = _closed_object(raw, SPAN_FIELDS, "TCM007")
        span_id = span["id"]
        if not isinstance(span_id, str) or IDENTIFIER.fullmatch(span_id) is None or span_id in seen:
            raise BriefCheckError("TCM007", "protected span ID is invalid or duplicate")
        kind = span["kind"]
        if kind not in PROTECTED_KINDS:
            raise BriefCheckError("TCM007", "protected span kind is unsupported")
        start = _offset(span["start"], "TCM007")
        end = _offset(span["end"], "TCM007")
        if start < previous_end or start >= end or end > len(source):
            raise BriefCheckError("TCM007", "protected spans overlap, are unordered, or exceed source bounds")
        selected = source[start:end]
        if _digest(span["sha256"], "TCM008") != hashlib.sha256(selected).hexdigest():
            raise BriefCheckError("TCM008", "protected span digest does not match source bytes")
        seen.add(span_id)
        previous_end = end
        result.append((span, selected))
    return tuple(result)


def _validate_bindings(
    value: Any,
    rendered: bytes,
    spans: tuple[tuple[Mapping[str, Any], bytes], ...],
) -> None:
    if not isinstance(value, list) or len(value) != len(spans):
        raise BriefCheckError("TCM009", "output bindings do not match the protected span set")
    previous_end = 0
    for raw, (span, source_bytes) in zip(value, spans, strict=True):
        binding = _closed_object(raw, BINDING_FIELDS, "TCM009")
        if binding["id"] != span["id"]:
            raise BriefCheckError("TCM009", "output binding order or identity differs from source spans")
        start = _offset(binding["start"], "TCM009")
        end = _offset(binding["end"], "TCM009")
        if start < previous_end or start >= end or end > len(rendered):
            raise BriefCheckError("TCM009", "output bindings overlap, are unordered, or exceed output bounds")
        selected = rendered[start:end]
        digest = _digest(binding["sha256"], "TCM009")
        if digest != hashlib.sha256(selected).hexdigest() or selected != source_bytes:
            raise BriefCheckError("TCM010", "protected output bytes differ from their source span")
        previous_end = end


def validate_brief(request: Mapping[str, Any]) -> dict[str, Any]:
    """Validate one closed request without writing or invoking another effect."""

    value = _closed_object(request, REQUEST_FIELDS, "TCM002")
    if value["explicit_skill"] != SKILL:
        raise BriefCheckError("TCM003", "explicit harness-operator-brief activation is required")
    if value["profile"] != PROFILE:
        raise BriefCheckError("TCM004", "operator-communication is the only supported profile")
    if value["source_kind"] not in SOURCE_KINDS:
        raise BriefCheckError("TCM005", "source kind is unsupported")

    _, source = _bounded_text(
        value["source_text"],
        limit=MAX_SOURCE_BYTES,
        code="TCM006",
    )
    if _digest(value["source_sha256"], "TCM006") != hashlib.sha256(source).hexdigest():
        raise BriefCheckError("TCM006", "source digest does not match source bytes")
    spans = _validate_spans(value["protected_spans"], source)

    _, rendered = _bounded_text(
        value["rendered_text"],
        limit=MAX_RENDERED_BYTES,
        code="TCM011",
    )
    _validate_bindings(value["bindings"], rendered, spans)
    if value["changed_paths"] != []:
        raise BriefCheckError("TCM012", "read-only brief requires exactly zero changed paths")

    canonical = [span for span, _ in spans if span["kind"] == "canonical-restitution-block"]
    if canonical and (
        len(spans) != 1
        or canonical[0]["start"] != 0
        or canonical[0]["end"] != len(source)
        or rendered != source
    ):
        raise BriefCheckError("TCM013", "a canonical restitution block must be returned alone and unchanged")

    return {
        "changed_paths": [],
        "outcome": "completed",
        "profile": PROFILE,
        "protected_binding_count": len(spans),
        "source_sha256": value["source_sha256"],
    }


def _load_request(path: str) -> Mapping[str, Any]:
    try:
        raw = Path(path).read_bytes()
    except OSError as exc:
        raise BriefCheckError("TCM001", "request JSON is not readable") from exc
    if len(raw) > MAX_REQUEST_BYTES:
        raise BriefCheckError("TCM001", "request JSON exceeds its byte limit")
    try:
        value = json.loads(raw.decode("utf-8"), object_pairs_hook=_duplicate_keys)
    except BriefCheckError:
        raise
    except (UnicodeDecodeError, json.JSONDecodeError, RecursionError) as exc:
        raise BriefCheckError("TCM001", "request is not valid bounded UTF-8 JSON") from exc
    if not isinstance(value, dict):
        raise BriefCheckError("TCM001", "request JSON must be an object")
    return value


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--request-json", required=True)
    args = parser.parse_args(argv)
    try:
        result = validate_brief(_load_request(args.request_json))
        print(json.dumps(result, sort_keys=True, separators=(",", ":")))
        return 0
    except BriefCheckError as exc:
        print(json.dumps({"code": exc.code, "outcome": "blocked"}, sort_keys=True, separators=(",", ":")))
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
