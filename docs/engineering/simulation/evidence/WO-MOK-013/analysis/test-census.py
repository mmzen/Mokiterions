"""Census every test in the workspace, now and at the baseline, name by name.

Run from the repository root:

    PYTHONIOENCODING=utf-8 python docs/engineering/simulation/evidence/WO-MOK-013/analysis/test-census.py

It writes nothing. Its stdout is retained as `../test-census.txt`.

`SPEC-MOK-004` rule 11 states the observer's total, the engine's total and the workspace's, and requires that a work
order which adds a test corrects the figures and that one which loses a test has a defect. `VER-MOK-013` requires the
per-target counts reconciled to those figures **arrival by arrival**. A count alone cannot distinguish a rename from a
removal plus an addition, and this work order contains exactly one rename, so the census is taken by name.

The method is a static count of `#[test]` functions per file. It is validated against the runtime figures rather than
trusted: `../test-output.txt` is the executed run and the totals here must equal it. A file's tests are attributed to
the file they are written in, which is how rules 9 and 10 attribute them; the executed target is `src/lib.rs` for the
three modules inside the library, `src/main.rs` for the binary, and the file itself for each integration test.
"""

import re
import subprocess
import sys

BASELINE = "ff3a155"

# Every file in the workspace that declares a test, at either commit. A file that exists at only one of the two is
# handled: `git show` failing means it did not exist then, and a missing file now means it was deleted.
FILES = [
    "mokiterions-core/src/lib.rs",
    "mokiterions-core/src/cli.rs",
    "mokiterions-core/src/simulation.rs",
    "mokiterions-core/src/main.rs",
    "mokiterions-core/tests/cli.rs",
    "mokiterions-core/tests/decisions.rs",
    "mokiterions-core/tests/density.rs",
    "mokiterions-core/tests/naming.rs",
    "mokiterions-core/tests/process.rs",
    "mokiterions-core/tests/termination.rs",
    "mokiterions-core/tests/viability.rs",
    "mokiterions-tui/src/lib.rs",
    "mokiterions-tui/src/authority.rs",
    "mokiterions-tui/src/export.rs",
    "mokiterions-tui/src/layout.rs",
    "mokiterions-tui/src/options.rs",
    "mokiterions-tui/src/render.rs",
    "mokiterions-tui/src/spatial.rs",
    "mokiterions-tui/src/state.rs",
    "mokiterions-tui/src/verification.rs",
    "mokiterions-tui/src/main.rs",
    "mokiterions-tui/tests/authority.rs",
    "mokiterions-tui/tests/export.rs",
    "mokiterions-tui/tests/layout.rs",
    "mokiterions-tui/tests/options.rs",
    "mokiterions-tui/tests/render.rs",
    "mokiterions-tui/tests/spatial.rs",
    "mokiterions-tui/tests/state.rs",
    "mokiterions-tui/tests/verification.rs",
]

TEST = re.compile(r"^\s*#\[test\]\s*$")
NAME = re.compile(r"^\s*(?:async\s+)?fn\s+([A-Za-z0-9_]+)")
IGNORE = re.compile(r"^\s*#\[ignore")


def names(text):
    """The names of every `#[test]` function in this source, in order.

    Rule 11 forbids `#[ignore]`, so an ignored test is reported rather than counted silently.
    """
    found, ignored = [], []
    lines = text.splitlines()
    for index, line in enumerate(lines):
        if not TEST.match(line):
            continue
        skipped = False
        for follow in lines[index + 1 : index + 8]:
            if IGNORE.match(follow):
                skipped = True
            match = NAME.match(follow)
            if match:
                (ignored if skipped else found).append(match.group(1))
                break
    return found, ignored


def read_now(path):
    try:
        with open(path, encoding="utf-8") as handle:
            return handle.read()
    except FileNotFoundError:
        return None


def read_then(path):
    completed = subprocess.run(
        ["git", "show", f"{BASELINE}:{path}"],
        capture_output=True,
    )
    if completed.returncode != 0:
        return None
    return completed.stdout.decode("utf-8")


def main():
    print(f"WO-MOK-013 test census — every test by name, {BASELINE} against this tree")
    print()
    print(f"{'file':<42}{'at ' + BASELINE:>12}{'now':>6}{'+':>4}{'-':>4}")
    rows = []
    arrivals, departures, ignored_any = {}, {}, []
    for path in FILES:
        now_text, then_text = read_now(path), read_then(path)
        now, now_ignored = names(now_text) if now_text is not None else ([], [])
        then, _ = names(then_text) if then_text is not None else ([], [])
        if now_ignored:
            ignored_any.append((path, now_ignored))
        gained = [name for name in now if name not in then]
        lost = [name for name in then if name not in now]
        if gained:
            arrivals[path] = gained
        if lost:
            departures[path] = lost
        rows.append((path, len(then), len(now)))
        print(f"{path:<42}{len(then):>12}{len(now):>6}{len(gained):>4}{len(lost):>4}")
    then_total = sum(then for _, then, _ in rows)
    now_total = sum(now for _, _, now in rows)
    print(f"{'workspace total':<42}{then_total:>12}{now_total:>6}")
    print()

    def package_total(prefix, column):
        return sum(row[column] for row in rows if row[0].startswith(prefix))

    print(f"{'engine (mokiterions-core)':<42}{package_total('mokiterions-core', 1):>12}"
          f"{package_total('mokiterions-core', 2):>6}")
    print(f"{'observer (mokiterions-tui)':<42}{package_total('mokiterions-tui', 1):>12}"
          f"{package_total('mokiterions-tui', 2):>6}")
    print()
    print("SPEC-MOK-004 rule 11 records 127 observer, 85 engine, 212 workspace at the baseline,")
    print("and 141 / 85 / 226 as corrected for WO-MOK-013.")
    print()
    print("Tier split, this tree, under rules 9 and 10:")
    internal = [path for path in FILES if path.startswith("mokiterions-tui/src/")]
    public = [path for path in FILES if path.startswith("mokiterions-tui/tests/")]
    by_path = {path: (then, now) for path, then, now in rows}
    print(f"  observer internal tier (src/) {sum(by_path[p][1] for p in internal):>5}")
    print(f"  observer public tier (tests/) {sum(by_path[p][1] for p in public):>5}")
    print()
    print("Arrivals, by name:")
    if not arrivals:
        print("  none")
    for path, gained in arrivals.items():
        print(f"  {path}  (+{len(gained)})")
        for name in gained:
            print(f"      {name}")
    print()
    print("Departures, by name:")
    if not departures:
        print("  none")
    for path, lost in departures.items():
        print(f"  {path}  (-{len(lost)})")
        for name in lost:
            print(f"      {name}")
    print()
    if ignored_any:
        print("#[ignore] found, which rule 11 forbids:")
        for path, found in ignored_any:
            print(f"  {path}: {', '.join(found)}")
    else:
        print("No #[test] carries #[ignore], as rule 11 requires.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
