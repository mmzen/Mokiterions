"""VER-MOK-016 oracle 1 under WO-MOK-017: digest every captured cell. No projection.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/manifest.py \
        <capture-dir> <out-path> <label>

`<capture-dir>` is one produced by `../capture.sh`: 120 `<cell>.txt` streams and 120 `<cell>.exit`
exit codes. Writes one manifest listing, per cell, the raw SHA-256, the byte count, the line count and
the exit code.

**There is no projection here, and its absence is the point.** This change adds no field to any record
and no record kind, so `VER-MOK-016` oracle 1 compares the 30 `baseline` cells byte for byte with
nothing removed. If a projection turns out to be needed for `baseline`, the change has touched
something `REQ-MOK-060` forbids it to touch, and adding one here would hide exactly that. This is
`WO-MOK-016`'s script with the cell count raised from 90 to 120, because `social` is now captured by
the same script at both commits.

The 90 `reference`, `individual` and `social` cells are expected to differ after the change, under
`REQ-MOK-060`. Their digests are recorded for the same reason the `baseline` ones are: a difference
must be *characterized*, and a characterization starts from knowing which cells moved.
"""

import hashlib
import io
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

EXPECTED_CELLS = 120


def read(path):
    return io.open(path, encoding='utf-8', newline='').read()


def main():
    capture_dir, out_path, label = sys.argv[1:4]
    cells = sorted(name[:-4] for name in os.listdir(capture_dir) if name.endswith('.txt'))
    lines = [
        f'# {label}',
        '# cell  sha256(raw)  bytes  lines  exit',
        '',
    ]
    by_policy = {}
    for cell in cells:
        text = read(os.path.join(capture_dir, cell + '.txt'))
        code = read(os.path.join(capture_dir, cell + '.exit')).strip()
        raw = hashlib.sha256(text.encode('utf-8')).hexdigest()
        count = text.count('\n')
        lines.append(f'{cell:34} {raw} {len(text):>9} {count:>7} {code}')
        policy = cell.split('-')[1]
        by_policy.setdefault(policy, []).append(cell)
    lines += ['', f'cells: {len(cells)}']
    for policy in sorted(by_policy):
        lines.append(f'  {policy}: {len(by_policy[policy])}')
    exits = {read(os.path.join(capture_dir, c + '.exit')).strip() for c in cells}
    lines.append(f'exit codes observed: {sorted(exits)}')
    lines.append('')
    io.open(out_path, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines))
    print(f'{label}: {len(cells)} cells, exit codes {sorted(exits)}')
    return 0 if len(cells) == EXPECTED_CELLS and exits == {'0'} else 1


if __name__ == '__main__':
    sys.exit(main())
