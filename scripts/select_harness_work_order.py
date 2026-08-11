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


class SelectionError(ValueError):
    """A bounded pull-request work-order selection error."""


def _unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise SelectionError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def select_work_order(body: str) -> str:
    if not isinstance(body, str):
        raise SelectionError("pull-request body must be text")
    matches = WORK_ORDER_LINE.findall(body)
    if len(matches) != 1:
        raise SelectionError(
            f"expected exactly one standalone Harness-Work-Order field; found {len(matches)}"
        )
    return matches[0]


def select_from_event(path: Path) -> str:
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
    return select_work_order(pull_request.get("body"))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Select one structured Harness-Work-Order field from a GitHub pull-request event."
    )
    parser.add_argument("--event", type=Path, required=True)
    return parser


def main(argv: list[str] | None = None) -> int:
    try:
        args = build_parser().parse_args(argv)
        print(select_from_event(args.event))
        return 0
    except SelectionError as exc:
        print(f"work-order selection: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
