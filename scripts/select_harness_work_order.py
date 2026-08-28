"""Select exactly one structured harness work-order ID from a GitHub event."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any


MAX_EVENT_BYTES = 2 * 1024 * 1024
WORK_ORDER_LINE = re.compile(
    r"^Harness-Work-Order:[ \t]*(WO-[A-Z][A-Z0-9-]*-\d{3})[ \t]*$",
    re.MULTILINE,
)
_WORK_ORDER_LINE_WITH_CR = re.compile(
    r"^Harness-Work-Order:[ \t]*WO-[A-Z][A-Z0-9-]*-\d{3}[ \t]*(\r)$",
    re.MULTILINE,
)
RESTITUTION_LINE = re.compile(
    r"^Harness-Restitution:[ \t]*([0-9a-f]{64})[ \t]*$",
    re.MULTILINE,
)


class SelectionError(ValueError):
    """A bounded pull-request work-order selection error."""


def _unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise SelectionError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def carriage_return_trailer_offsets(body: str) -> list[int]:
    return [
        len(body[: match.start(1)].encode("utf-8"))
        for match in _WORK_ORDER_LINE_WITH_CR.finditer(body)
    ]


def select_work_order(body: str) -> str:
    if not isinstance(body, str):
        raise SelectionError("pull-request body must be text")
    matches = WORK_ORDER_LINE.findall(body)
    if len(matches) != 1:
        offsets = carriage_return_trailer_offsets(body)
        if offsets:
            raise SelectionError(
                f"W-ADS-001: the Harness-Work-Order line ends with a carriage return at byte offset {offsets[0]}; "
                "write the body with LF line endings and push again"
            )
        raise SelectionError(
            f"expected exactly one standalone Harness-Work-Order field; found {len(matches)}"
        )
    return matches[0]


def select_restitution_digest(body: str) -> str:
    if not isinstance(body, str):
        raise SelectionError("pull-request body must be text")
    matches = RESTITUTION_LINE.findall(body)
    if len(matches) > 1:
        raise SelectionError(f"expected at most one standalone Harness-Restitution field; found {len(matches)}")
    return matches[0] if matches else ""


def select_from_event(path: Path, field: str = "work-order") -> str:
    try:
        size = path.stat().st_size
        if size > MAX_EVENT_BYTES:
            raise SelectionError("GitHub event exceeds the size limit")
        event = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=_unique_object)
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise SelectionError(f"cannot read GitHub event: {exc}") from exc
    if not isinstance(event, dict):
        raise SelectionError("GitHub event root must be an object")
    pull_request = event.get("pull_request")
    if not isinstance(pull_request, dict):
        raise SelectionError("GitHub event has no pull_request object")
    if field == "restitution-digest":
        return select_restitution_digest(pull_request.get("body"))
    return select_work_order(pull_request.get("body"))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Select one structured Harness-Work-Order field from a GitHub pull-request event."
    )
    parser.add_argument("--event", type=Path, required=True)
    parser.add_argument("--field", choices=("work-order", "restitution-digest"), default="work-order")
    return parser


def main(argv: list[str] | None = None) -> int:
    try:
        args = build_parser().parse_args(argv)
        print(select_from_event(args.event, field=args.field))
        return 0
    except SelectionError as exc:
        print(f"work-order selection: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
