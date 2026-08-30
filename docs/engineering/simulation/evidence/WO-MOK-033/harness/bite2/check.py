#!/usr/bin/env python3
"""Does the interruption test bite? Run it against a copy patched back to the pre-fix driver.

`VER-MOK-019`'s *Independence* point 2 in general form: a test that passes against the defect it
claims to catch verifies nothing. Both files are copied so the repository's own copies are never
edited.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys

REPO = "C:/Users/mathi/Mokiterions-understand-20260829-1340"
HERE = os.path.dirname(os.path.abspath(__file__))

AFTER = """        try:
            row, fault = execute_cell(cell, ticks, binary, path, keep_to=keep_to, runner=runner)
            if keep_to is not None and os.path.isfile(keep_to):
                report(f"kept stream for {cell[0]}:{cell[1]}:{cell[2]} at {keep_to}")
        finally:"""

BEFORE = """        row, fault = execute_cell(cell, ticks, binary, path, keep_to=keep_to, runner=runner)
        if keep_to is not None and os.path.isfile(keep_to):
            report(f"kept stream for {cell[0]}:{cell[1]}:{cell[2]} at {keep_to}")
        if True:"""


def main():
    for name in ("run_simulation_batch.py", "test_run_simulation_batch.py"):
        shutil.copyfile(f"{REPO}/scripts/{name}", f"{HERE}/{name}")

    with open(f"{HERE}/run_simulation_batch.py", "r", encoding="utf-8", newline="") as handle:
        source = handle.read()
    if AFTER not in source:
        print("the fixed form was not found; the patch anchor has moved")
        return 2
    # `if True:` keeps the body's indentation, so the removal still runs on the normal path and only
    # the interrupt path loses it. That isolates the defect rather than deleting the removal.
    patched = source.replace(AFTER, BEFORE)
    with open(f"{HERE}/run_simulation_batch.py", "w", encoding="utf-8", newline="") as handle:
        handle.write(patched)

    completed = subprocess.run(
        [sys.executable, f"{HERE}/test_run_simulation_batch.py", "-v",
         "TestRetainedFileAndStream.test_the_stream_is_removed_when_a_cell_is_interrupted"],
        cwd=HERE, capture_output=True, text=True)
    print(f"exit {completed.returncode}")
    print(completed.stdout)
    print(completed.stderr)
    return 0 if completed.returncode != 0 else 1


if __name__ == "__main__":
    sys.exit(main())
