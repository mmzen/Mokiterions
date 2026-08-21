"""VER-MOK-012 oracle 1, comparison G: the fourth policy's text streams, `master` against the merge.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-018/merge/social-vs-master.py \
        <master-social-manifest> <merge-social-manifest> <output-file>

`analysis/compare.py` cannot take this comparison, and the reason is a manifest format and not a
disagreement about the property. `WO-MOK-016` captured the `social` cells on `master` with a script
that wrote no standard-error stream, so its manifest carries four columns -- the cell, the digest of
standard output, its byte and line counts, and the exit code -- while `digest.py` writes six and
`compare.py` reads six positionally. This script pairs the four columns the two manifests share and
reports the two the left-hand side does not have rather than inventing them.

**Nothing else is relaxed.** No projection is applied, no normalization, no tolerance: a digest either
matches or it does not, and the exit status is non-zero if any cell differs or if the two manifests do
not cover the same thirty cells. The property under test is the one oracle 1 states for every other
cell of the matrix -- that this branch's record stream perturbs no text stream -- asked of the fourth
policy, whose text streams `master` measured and whose record streams did not exist there.
"""

import io
import re
import sys

LEFT = ('stdout', 'bytes', 'lines', 'exit')
RIGHT = ('stdout', 'bytes', 'lines', 'stderr', 'stderr_bytes', 'exit')

# The left-hand manifest ends in a plain-text summary that is not comment-prefixed -- `cells: 30`,
# `  social: 30`, `exit codes observed: ['0']`. A cell line is recognized by its name instead of by
# position, so a footer is skipped rather than read as three cells with no counterpart.
CELL = re.compile(r'^seed\d+-[a-z]+-d\d+\.\d+-trace(on|off)$')


def load(path, columns):
    cells = {}
    with io.open(path, 'r', encoding='utf-8') as handle:
        for line in handle:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            fields = line.split()
            if not CELL.match(fields[0]):
                continue
            cells[fields[0]] = dict(zip(columns, fields[1:1 + len(columns)]))
    return cells


def main():
    left_path, right_path, output_file = sys.argv[1:4]
    left, right = load(left_path, LEFT), load(right_path, RIGHT)

    lines = [
        '# oracle 1, comparison G: the fourth policy on `master` against the fourth policy on the merge',
        f'# left:  {left_path}',
        f'# right: {right_path}',
        '#',
        '# Compared columns: sha256(stdout), bytes, lines, exit code.',
        '# The left-hand manifest carries no standard-error column, so the right-hand one\'s two are',
        '# reported below rather than compared. No projection, no normalization, no tolerance.',
        '',
    ]

    missing = sorted(set(left) ^ set(right))
    differing = 0
    for cell in sorted(set(left) & set(right)):
        columns = [name for name in LEFT if left[cell][name] != right[cell][name]]
        if columns:
            differing += 1
            lines.append(f'{cell:34s} DIFFERS in {", ".join(columns)}')
            for name in columns:
                lines.append(f'{"":34s}   left  {name:12s} {left[cell][name]}')
                lines.append(f'{"":34s}   right {name:12s} {right[cell][name]}')
        else:
            lines.append(f'{cell:34s} identical')

    stderr_empty = sum(1 for cell in right if right[cell]['stderr_bytes'] == '0')
    lines += [
        '',
        f'# {len(set(left) & set(right))} cells compared, {differing} differing',
        f'# result: {"FAIL" if differing or missing else "PASS"}',
        f'# {stderr_empty} of {len(right)} right-hand cells wrote nothing to standard error,'
        ' which the left-hand manifest does not record',
    ]
    if missing:
        lines.append(f'# cells present on one side only: {", ".join(missing)}')

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(f'comparison G: {len(set(left) & set(right))} cells compared, {differing} differing')
    return 1 if differing or missing else 0


if __name__ == '__main__':
    sys.exit(main())
