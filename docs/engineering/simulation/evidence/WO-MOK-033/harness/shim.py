#!/usr/bin/env python3
"""A stand-in for the engine's binary, so a cell can be made to fail in a chosen way.

`VER-MOK-019` cases B17, B19 to B26 and Q16 need a cell that produces a crafted stream or a chosen
failure. The engine cannot be asked for one: it is deterministic and correct on every configuration
measured, which is exactly why the contract's *Independence* point 2 says a check with no failing case
is not a verified check.

This is launched by the driver as `--binary`, so the driver's own child-process path, stream read,
cross-check and stage assignment all run for real. It reads its instructions from `shim-plan.json`
beside it and never from its environment.
"""

from __future__ import annotations

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PLAN = os.path.join(HERE, "shim-plan.json")


def main(argv):
    options = {}
    index = 0
    while index < len(argv) - 1:
        if argv[index].startswith("--"):
            options[argv[index][2:]] = argv[index + 1]
            index += 2
        else:
            index += 1

    with open(PLAN, "r", encoding="utf-8") as handle:
        plan = json.load(handle)

    key = f"{options.get('policy')}:{options.get('density')}:{options.get('seed')}"

    # A record of every invocation, so a case can establish that no child ran at all. `VER-MOK-019`
    # B6 requires a usage error to spawn no child, and the absence of an output file does not show it.
    if plan.get("_log"):
        with open(os.path.join(HERE, plan["_log"]), "a", encoding="utf-8", newline="\n") as handle:
            handle.write(key + "\n")

    entry = plan.get(key, plan.get("*", {}))

    # A chosen non-zero exit, with the engine's own words on standard error. Case B21.
    #
    # Written as bytes rather than as text, so that the plan's bytes are exactly the bytes the child
    # emits. A text-mode write would translate every newline on this platform and B21 would then be
    # comparing the driver's fidelity against a message the fixture never actually sent.
    if entry.get("status"):
        sys.stderr.buffer.write(entry.get("stderr", "").encode("utf-8"))
        sys.stderr.buffer.flush()
        return int(entry["status"])

    # A cell that leaves no stream at all, which the driver must report at the `read` stage.
    if entry.get("no_stream"):
        return 0

    source = entry.get("stream")
    if source is None:
        return 0
    with open(os.path.join(HERE, source), "rb") as handle:
        raw = handle.read()
    if entry.get("truncate_to") is not None:
        raw = raw[: entry["truncate_to"]]

    with open(options["events-path"], "wb") as handle:
        handle.write(raw)
    sys.stdout.write(entry.get("stdout", "shim run complete\n"))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
