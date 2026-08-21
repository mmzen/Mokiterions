"""VER-MOK-012 oracle 1: compare two digest manifests cell for cell.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-018/analysis/compare.py \
        <left-manifest> <right-manifest> <output-file> <label>

Both manifests are produced by `digest.py`. The comparison is over the six columns every
manifest has -- the standard-output digest, its byte and line counts, the standard-error digest,
its byte count and the exit code -- because those are the columns oracle 1 is about. A
sink-configured manifest carries three further columns for the record stream; they have no
counterpart in a sinkless manifest and are reported rather than compared.

**No projection is applied and none is available to apply.** `VER-MOK-012` oracle 1 is explicit
that a contract needing one here would be verifying a different requirement, so this script has
no normalization, no tolerance, no exemption and no option to add one. A digest either matches or
it does not.

One line per cell is written, so a failure names the cell and the column rather than reporting a
count. The exit status is non-zero if any cell differs or if the two manifests do not cover the
same cells, which is itself a failure: a capture that lost a cell has not been compared.
"""

import io
import sys

SHARED = ('stdout', 'bytes', 'lines', 'stderr', 'stderr_bytes', 'exit')


def load(path):
    """One manifest as an ordered mapping of cell name to its columns."""
    cells = {}
    with io.open(path, 'r', encoding='utf-8') as handle:
        for line in handle:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            columns = line.split()
            cell = columns[0]
            row = dict(zip(SHARED, columns[1:7]))
            if len(columns) > 7:
                row['sink'] = columns[7]
                row['sink_bytes'] = columns[8]
                row['sink_lines'] = columns[9]
            cells[cell] = row
    return cells


def main():
    left_path, right_path, output_file, label = sys.argv[1:5]
    left = load(left_path)
    right = load(right_path)

    lines = [
        f'# {label}',
        f'# left:  {left_path}',
        f'# right: {right_path}',
        '#',
        '# Compared columns: sha256(stdout), bytes, lines, sha256(stderr), stderr bytes, exit code.',
        '# No projection, no normalization, no tolerance. VER-MOK-012 oracle 1.',
        '',
    ]

    failures = 0

    missing = sorted(set(left) ^ set(right))
    for cell in missing:
        side = 'right' if cell in left else 'left'
        lines.append(f'{cell:<34} MISSING from the {side} manifest')
        failures += 1

    for cell in sorted(set(left) & set(right)):
        differences = [
            f'{column}: {left[cell][column]} != {right[cell][column]}'
            for column in SHARED
            if left[cell][column] != right[cell][column]
        ]
        if differences:
            lines.append(f'{cell:<34} DIFFERS  ' + '; '.join(differences))
            failures += 1
        else:
            lines.append(f'{cell:<34} identical')

    compared = len(set(left) & set(right))
    lines.append('')
    lines.append(f'# {compared} cells compared, {failures} differing')
    lines.append(f'# result: {"FAIL" if failures else "PASS"}')

    # The record-stream columns have no counterpart on the sinkless side, so they are stated
    # rather than compared. A reviewer reading this file should not have to open the manifest to
    # learn whether the sink columns were present at all.
    with_sink = sum(1 for row in right.values() if 'sink' in row)
    if with_sink:
        lines.append(
            f'# {with_sink} right-hand cells additionally carry a record stream, not compared here'
        )

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(f'{label}: {compared} cells compared, {failures} differing')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
