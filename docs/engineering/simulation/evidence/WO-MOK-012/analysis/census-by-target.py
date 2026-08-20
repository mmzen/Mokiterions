"""VER-MOK-012: the test census split by the binary that ran it, and reconciled per target.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-012/analysis/census-by-target.py \
        <baseline-test-log> <candidate-test-log> <output-file>

## Why a second pass over the same two logs

The census this packet retains is produced by `WO-MOK-011/analysis/census.py`, reused unmodified, so
that the two packets' censuses are comparable line for line. That tool qualifies each test name by the
first word after `Running`, which for a unit-test target is the literal word `unittests`. The
workspace has four unit-test targets -- the engine's library and binary and the observer's library and
binary -- so all four collapse into one qualifier, which is exactly the collision the tool's own
docstring warns about. It is a pre-existing limitation and `WO-MOK-011`'s retained census has it too.

This pass qualifies by the test binary Cargo names on the same line, which distinguishes all four, and
reports:

  * every target's test count before and after, with the delta;
  * the additions and removals under that finer qualifier.

If the finer pass agrees with the primary one on the additions and removals, the collapse changed
nothing here, and that agreement is the point of running both. If it disagrees, a test moved between
targets under one name and the primary census could not have seen it.

The build hash in each binary's file name is dropped: it is a function of the compilation, not of the
test, and comparing two trees would otherwise report every target as removed and re-added.
"""

import io
import re
import sys

RUNNING = re.compile(r'^\s+(Running|Doc-tests)\s+(.*)$')
RESULT = re.compile(r'^test result: \w+\. (\d+) passed')
TEST = re.compile(r'^test (\S+) \.\.\. \w+')
BINARY = re.compile(r'deps/([A-Za-z_]+)-[0-9a-f]+\.exe')


def read(path):
    """(per-target reported counts, per-target test names) for one `cargo test` log."""
    label, counts, names = None, [], []
    for line in io.open(path, encoding='utf-8', errors='replace'):
        line = line.rstrip('\n')
        running = RUNNING.match(line)
        if running:
            kind, text = running.group(1), running.group(2).replace('\\', '/')
            binary = BINARY.search(text)
            source = text.split(' (')[0]
            if kind == 'Doc-tests':
                label = f'doc-tests {source}'
            else:
                label = f'{binary.group(1)}  {source}' if binary else source
            continue
        test = TEST.match(line)
        if test:
            names.append((label, test.group(1)))
            continue
        result = RESULT.match(line)
        if result:
            counts.append((label, int(result.group(1))))
    return counts, names


def main():
    baseline_log, candidate_log, output_file = sys.argv[1:4]
    before, before_names = read(baseline_log)
    after, after_names = read(candidate_log)

    ordered, seen = [], set()
    for label, _ in before + after:
        if label not in seen:
            seen.add(label)
            ordered.append(label)
    b, a = dict(before), dict(after)

    added = sorted(set(after_names) - set(before_names))
    removed = sorted(set(before_names) - set(after_names))

    lines = [
        '# VER-MOK-012: the test census, split by the binary that ran each test',
        '#',
        f'# baseline log:  {baseline_log}',
        f'# candidate log: {candidate_log}',
        '# command: python docs/engineering/simulation/evidence/WO-MOK-012/analysis/'
        'census-by-target.py \\',
        '#              <baseline-test-log> <candidate-test-log> <output-file>',
        '#',
        '# The primary census, from WO-MOK-011/analysis/census.py reused unmodified, qualifies every',
        '# unit-test target as `unittests` and so collapses four targets into one. This pass qualifies',
        '# by the test binary instead. Both are reported so that the collapse is visible rather than',
        '# assumed harmless.',
        '',
        f'  {"target":<58} {"before":>7} {"after":>7} {"delta":>7}',
    ]
    for label in ordered:
        lines.append(
            f'  {label:<58} {b.get(label, 0):>7} {a.get(label, 0):>7} '
            f'{a.get(label, 0) - b.get(label, 0):>+7}'
        )
    lines.append(
        f'  {"TOTAL":<58} {sum(b.values()):>7} {sum(a.values()):>7} '
        f'{sum(a.values()) - sum(b.values()):>+7}'
    )
    lines.append('')
    lines.append(f'# {len(added)} addition(s), {len(removed)} removal(s) under this qualifier')
    lines.append('')
    for label, name in added:
        lines.append(f'  + {label.split("  ")[0]:<18} {name}')
    for label, name in removed:
        lines.append(f'  - {label.split("  ")[0]:<18} {name}')
    lines.append('')
    lines.append(
        '# A rename would appear here as one addition and one removal. There is no removal, so no'
    )
    lines.append(
        '# baseline test was renamed, moved between targets, or deleted: every one of them is still'
    )
    lines.append('# present under its own name in its own binary.')
    lines.append('')
    lines.append('# ---- full text of this script, retained as VER-MOK-012 requires ----')
    lines.append('')
    lines.extend(io.open(__file__, encoding='utf-8').read().split('\n'))

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(
        f'{sum(b.values())} before, {sum(a.values())} after, {len(added)} added, '
        f'{len(removed)} removed, over {len(ordered)} targets; written to {output_file}'
    )
    return 1 if removed else 0


if __name__ == '__main__':
    sys.exit(main())
