"""Re-count both packages' public interfaces, as VER-MOK-013's static checks require.

Run from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-013/analysis/interface.py

It writes nothing. Its stdout is retained as `../interface.txt`.

Two obligations are discharged here, and neither is asserted from the diff:

  * `SPEC-MOK-004` rule 6 records the observer's interface as 94 items, 118 `pub` lines and 24 public fields, with a
    per-module table. Rule 6's counting rule is applied literally: an item is one declaration written `pub` outside
    every `#[cfg(test)]` block, in one of the seven modules `lib.rs` declares `pub mod`. `pub(crate)` is not public and
    is not counted. A `pub` line that is not a declaration is a public field, which is how 94 + 24 = 118.
  * `SPEC-MOK-002` rules 5 and 6 hold the engine's public interface. It is re-counted against `ff3a155`, the commit
    `WO-MOK-013` starts from, declaration by declaration rather than by a figure.
"""

import re
import subprocess
import sys

BASELINE = "ff3a155"

OBSERVER_MODULES = [
    "authority",
    "export",
    "layout",
    "options",
    "render",
    "spatial",
    "state",
]

ENGINE_FILES = [
    "mokiterions-core/src/lib.rs",
    "mokiterions-core/src/cli.rs",
    "mokiterions-core/src/simulation.rs",
    "mokiterions-core/src/main.rs",
]

DECLARATION = re.compile(
    r"^pub\s+(?:unsafe\s+|async\s+|extern\s+\"[^\"]*\"\s+)*"
    r"(fn|struct|enum|union|trait|type|const|static|mod|use|impl)\b"
)


def public_lines(text):
    """Yield (line number, stripped line) for every `pub` line outside every `#[cfg(test)]` block.

    A `#[cfg(test)]` region is skipped by brace depth from the attribute to the close of the item it attaches to,
    which is how rule 6's "outside every `#[cfg(test)]` block" is measured rather than guessed at.
    """
    lines = text.splitlines()
    index = 0
    while index < len(lines):
        line = lines[index]
        if line.strip().startswith("#[cfg(test)]"):
            index += 1
            depth = 0
            opened = False
            while index < len(lines):
                depth += lines[index].count("{") - lines[index].count("}")
                if "{" in lines[index]:
                    opened = True
                index += 1
                if opened and depth <= 0:
                    break
            continue
        stripped = line.strip()
        if stripped.startswith("pub ") or stripped == "pub":
            yield index + 1, stripped
        index += 1


def classify(stripped):
    return "item" if DECLARATION.match(stripped) else "field"


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def read_at(commit, path):
    completed = subprocess.run(
        ["git", "show", f"{commit}:{path}"],
        capture_output=True,
        check=True,
    )
    return completed.stdout.decode("utf-8")


def count(text):
    items, fields = [], []
    for number, stripped in public_lines(text):
        (items if classify(stripped) == "item" else fields).append((number, stripped))
    return items, fields


def main():
    print("SPEC-MOK-004 rule 6 — the observer's public interface, re-counted")
    print()
    print(f"{'module':<12}{'items':>7}{'fields':>8}{'pub lines':>11}")
    total_items = total_fields = 0
    per_module = {}
    for module in OBSERVER_MODULES:
        items, fields = count(read(f"mokiterions-tui/src/{module}.rs"))
        per_module[module] = (items, fields)
        total_items += len(items)
        total_fields += len(fields)
        print(f"{module:<12}{len(items):>7}{len(fields):>8}{len(items) + len(fields):>11}")
    print(f"{'total':<12}{total_items:>7}{total_fields:>8}{total_items + total_fields:>11}")
    print()
    print(f"recorded: 94 items, 24 public fields, 118 pub lines")
    print(f"measured: {total_items} items, {total_fields} public fields, {total_items + total_fields} pub lines")
    moved = (total_items, total_fields, total_items + total_fields) != (94, 24, 118)
    print(f"figure movement: {'YES — stop condition 3 of WO-MOK-013 fires' if moved else 'none'}")
    print()
    print("Every public declaration, module by module, so the figure is inspectable and not only stated:")
    for module in OBSERVER_MODULES:
        items, fields = per_module[module]
        print()
        print(f"--- mokiterions-tui/src/{module}.rs")
        for number, stripped in items:
            print(f"  item  {number:>5}  {stripped}")
        for number, stripped in fields:
            print(f"  field {number:>5}  {stripped}")
    print()
    print()
    print(f"SPEC-MOK-002 rules 5 and 6 — the engine's public interface, re-counted against {BASELINE}")
    print()
    engine_now = engine_then = 0
    unchanged = True
    for path in ENGINE_FILES:
        now_items, now_fields = count(read(path))
        then_items, then_fields = count(read_at(BASELINE, path))
        now = [line for _, line in now_items + now_fields]
        then = [line for _, line in then_items + then_fields]
        engine_now += len(now)
        engine_then += len(then)
        same = now == then
        unchanged = unchanged and same
        print(f"{path:<40}{len(then):>5} at {BASELINE}{len(now):>6} now   {'identical' if same else 'CHANGED'}")
        if not same:
            for line in sorted(set(then) - set(now)):
                print(f"    lost    {line}")
            for line in sorted(set(now) - set(then)):
                print(f"    gained  {line}")
    print(f"{'total':<40}{engine_then:>5} at {BASELINE}{engine_now:>6} now   "
          f"{'unchanged item for item' if unchanged else 'CHANGED'}")
    print()
    print("The engine's surface is re-counted declaration by declaration rather than inferred from an empty diff.")
    print("The empty diff is retained separately, as `engine-untouched.txt`.")
    return 0 if not moved and unchanged else 1


if __name__ == "__main__":
    sys.exit(main())
