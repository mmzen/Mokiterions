#!/usr/bin/env python3
"""Records the exact argument vector the driver hands a child, then writes a valid stream.

`VER-MOK-019`'s *Security and privacy* first and second bullets turn on what rule 4.4 passes to a
child: no connector option, no credential path, no live selection. B2 establishes that a cell's
stream is byte-identical to a hand execution with rule 4.4's arguments, which does not by itself
exclude an extra option that leaves the stream unchanged. This captures the vector itself.

It is a separate file from `shim.py` on purpose: that file's bytes are already hashed into the
phase B and phase C evidence, and a tool used to take a measurement should not be edited after the
measurement it took.
"""

from __future__ import annotations

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
LOG = os.path.join(HERE, "phase-d", "child-argv.jsonl")
STREAM = os.path.join(HERE, "fixture-streams", "small.jsonl")


def main(argv):
    os.makedirs(os.path.dirname(LOG), exist_ok=True)
    with open(LOG, "a", encoding="utf-8", newline="\n") as handle:
        handle.write(json.dumps({"argv": argv, "environ_names": sorted(os.environ)}) + "\n")

    options = {}
    index = 0
    while index < len(argv) - 1:
        if argv[index].startswith("--"):
            options[argv[index][2:]] = argv[index + 1]
            index += 2
        else:
            index += 1

    if "events-path" in options:
        with open(STREAM, "rb") as handle:
            raw = handle.read()
        with open(options["events-path"], "wb") as handle:
            handle.write(raw)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
