#!/usr/bin/env python3
"""Do the three regression tests bite? Run each against a copy patched back to its own defect.

`VER-MOK-019`'s *Independence* point 2 in general form: a test that passes against the defect it
claims to catch verifies nothing. Verification found three defects in the driver; each was repaired
with a test, and this instrument is the record that each of those tests fails on the pre-fix code.

Both driver and suite are copied into this directory, so the repository's own copies are never
edited. One patch is applied per run, on a fresh copy, so a passing test cannot be a second defect's
patch masking the first. Every patch keeps the surrounding structure -- `pass` for a removed
statement, `if True:` for a removed `finally:` -- so the patched form isolates the defect rather than
deleting the code around it.

Exit 0 means all three tests failed on their pre-fix copy, which is the result being claimed.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys

REPO = "C:/Users/mathi/Mokiterions-understand-20260829-1340"
HERE = os.path.dirname(os.path.abspath(__file__))

# --------------------------------------------------------------------------------------------------
# Defect 1: rule 2.3's explicit path was not normalised to the platform's separator. Found by case B1
# against the real engine: a forward-slash relative path passes `os.path.isfile` and then fails to
# launch, so all 400 cells fail at once.
# --------------------------------------------------------------------------------------------------
NORMALISE_AFTER = """        explicit = os.path.normpath(explicit)
        parent = os.path.basename(os.path.dirname(os.path.abspath(explicit)))"""
NORMALISE_BEFORE = """        pass  # pre-fix: the operator's separators are passed through unchanged
        parent = os.path.basename(os.path.dirname(os.path.abspath(explicit)))"""

# --------------------------------------------------------------------------------------------------
# Defect 2: rule 7.1's reusable path was derived from the cell's position in the sweep, so under rule
# 2.8's `--jobs` two concurrent cells could hold one path. Found against the real engine: two of
# eight cells lost.
# --------------------------------------------------------------------------------------------------
SLOT_AFTER = """            def one_from_pool(cell):
                slot = slots.get()
                try:
                    return one(cell, slot)
                finally:
                    slots.put(slot)

            with concurrent.futures.ThreadPoolExecutor(max_workers=jobs) as pool:
                futures = {pool.submit(one_from_pool, cell): cell for cell in cells}"""
SLOT_BEFORE = """            def one_from_pool(cell, index):
                return one(cell, index % jobs)  # pre-fix: the slot is the sweep position

            with concurrent.futures.ThreadPoolExecutor(max_workers=jobs) as pool:
                futures = {pool.submit(one_from_pool, cell, index): cell
                           for index, cell in enumerate(cells)}"""

# --------------------------------------------------------------------------------------------------
# Defect 3: rule 7.3's stream removal was skipped when a cell was interrupted, leaving a stream on
# disk while the batch record's `leftover_streams` said none was left.
# --------------------------------------------------------------------------------------------------
REMOVAL_AFTER = """        try:
            row, fault = execute_cell(cell, ticks, binary, path, keep_to=keep_to, runner=runner)
            if keep_to is not None and os.path.isfile(keep_to):
                report(f"kept stream for {cell[0]}:{cell[1]}:{cell[2]} at {keep_to}")
        finally:"""
REMOVAL_BEFORE = """        row, fault = execute_cell(cell, ticks, binary, path, keep_to=keep_to, runner=runner)
        if keep_to is not None and os.path.isfile(keep_to):
            report(f"kept stream for {cell[0]}:{cell[1]}:{cell[2]} at {keep_to}")
        if True:  # pre-fix: the removal is on the normal path only"""

CASES = (
    ("the explicit binary path is normalised",
     NORMALISE_AFTER, NORMALISE_BEFORE,
     "TestBinaryResolution.test_an_explicit_path_is_normalised_to_the_platforms_separator"),
    ("no two concurrent cells share a reusable path",
     SLOT_AFTER, SLOT_BEFORE,
     "TestRetainedFileAndStream.test_no_two_concurrent_cells_share_a_reusable_path"),
    ("the stream is removed when a cell is interrupted",
     REMOVAL_AFTER, REMOVAL_BEFORE,
     "TestRetainedFileAndStream.test_the_stream_is_removed_when_a_cell_is_interrupted"),
)


def run_one(title, after, before, test):
    for name in ("run_simulation_batch.py", "test_run_simulation_batch.py"):
        shutil.copyfile(f"{REPO}/scripts/{name}", f"{HERE}/{name}")

    with open(f"{HERE}/run_simulation_batch.py", "r", encoding="utf-8", newline="") as handle:
        source = handle.read()
    print(f"--- {title}")
    print(f"    test: {test}")
    if after not in source:
        print("    ANCHOR MOVED: the fixed form was not found, so nothing was patched")
        return False
    with open(f"{HERE}/run_simulation_batch.py", "w", encoding="utf-8", newline="") as handle:
        handle.write(source.replace(after, before, 1))

    completed = subprocess.run(
        [sys.executable, f"{HERE}/test_run_simulation_batch.py", "-v", test],
        cwd=HERE, capture_output=True, text=True)
    tail = [line for line in completed.stderr.splitlines() if line.strip()]
    print(f"    exit {completed.returncode} on the pre-fix copy")
    for line in tail[-6:]:
        print(f"    | {line}")
    bites = completed.returncode != 0
    print(f"    {'BITES' if bites else 'DOES NOT BITE -- the test passes on the defect'}")
    return bites


def main():
    results = [run_one(*case) for case in CASES]
    print()
    print(f"{sum(results)} of {len(results)} tests fail on their own pre-fix copy")
    # The patched copies are working files, not evidence: the repaired driver is in the repository and
    # a patched copy of it retained beside the packet would be a second, wrong reading of the same
    # file. Only this instrument and its output are retained.
    for name in ("run_simulation_batch.py", "test_run_simulation_batch.py"):
        if os.path.isfile(f"{HERE}/{name}"):
            os.remove(f"{HERE}/{name}")
    return 0 if all(results) else 1


if __name__ == "__main__":
    sys.exit(main())
