"""VER-MOK-010 oracle 5: enumerate a package's public interface from its sources.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-010/analysis/interface.py <src-file> ...

Prints one line per `pub` line found outside every `#[cfg(test)]` block, qualified by the file it
was declared in, then the totals. The output is sorted, so two runs over two revisions of the same
sources can be compared with a plain diff.

What is counted is what `SPEC-MOK-004` rule 6 counts, so that the figure printed here is the figure
that rule records and not a second convention:

  * an **item** is one declaration written `pub` -- `pub fn`, `pub struct`, `pub enum`, `pub const`,
    `pub static`, `pub type`, `pub trait`, `pub use` or `pub mod`;
  * a **public field** is any other line that writes `pub`, which in this corpus is a public field of
    a public struct;
  * `pub(crate)`, `pub(super)` and `pub(in ...)` are not public and are excluded. This is the line
    `Observer::name_of` falls on, and it is why the observer's interface does not grow under
    `WO-MOK-010`;
  * a variant of a public enum is written without `pub` and is in neither figure, which rule 6's
    2026-08-19 amendment settled.

`#[cfg(test)]` blocks are skipped whole, by brace balance from the attribute's own item. That is what
keeps a test module's `pub fn ..._for_test` hook out of the enumeration: rule 7 keeps those out of the
library target, so they are not in the interface either.

The engine's enumeration under `SPEC-MOK-002` rule 5 is taken the same way, over that package's
sources, because the two rules count the same thing even though rule 5 also lists and justifies each
item by hand.
"""

import io
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')

ITEM = re.compile(r'\bpub\s+(fn|struct|enum|const|static|type|trait|use|mod)\b')
PUB = re.compile(r'(^|\s)pub\s')
RESTRICTED = re.compile(r'(^|\s)pub\(')


def outside_test_blocks(lines):
    """Every line of `lines` that is not inside a `#[cfg(test)]` item."""
    kept, index = [], 0
    while index < len(lines):
        if lines[index].strip().startswith('#[cfg(test)]'):
            index += 1
            balance, started = 0, False
            while index < len(lines):
                balance += lines[index].count('{') - lines[index].count('}')
                if '{' in lines[index]:
                    started = True
                index += 1
                if started and balance <= 0:
                    break
            continue
        kept.append(lines[index])
        index += 1
    return kept


def main():
    paths = sys.argv[1:]
    if not paths:
        print(__doc__)
        return 2
    rows, items, fields = [], 0, 0
    for path in paths:
        name = os.path.basename(path)
        lines = outside_test_blocks(io.open(path, encoding='utf-8').read().splitlines())
        for line in lines:
            if not PUB.search(line) or RESTRICTED.search(line):
                continue
            kind = 'item' if ITEM.search(line) else 'field'
            rows.append(f'{name:16} {kind:5} {line.strip()}')
            if kind == 'item':
                items += 1
            else:
                fields += 1
    for row in sorted(rows):
        print(row)
    print()
    print(f'items {items}  public fields {fields}  pub lines {items + fields}')
    return 0


if __name__ == '__main__':
    sys.exit(main())
