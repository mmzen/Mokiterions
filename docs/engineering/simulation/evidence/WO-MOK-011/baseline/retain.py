"""VER-MOK-011 oracle 1: retain the part of a 110 MB capture that a reviewer needs.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-011/baseline/retain.py \
        <capture-dir> <out-dir> <label>

The declared matrix is 90 cells and 110 MB, which is not a reasonable thing to commit twice. What is
committed instead, and why each piece:

  * `<out-dir>/manifest` (written by `compare.py`) -- the SHA-256 of every cell, raw and projected.
    This is what makes the comparison checkable rather than asserted.
  * `<out-dir>/init/<cell>.txt` -- the initialization block of **every** cell: every line up to and
    including the last `tick=0` line. This is where the added field appears, so it is the highest
    value per byte in the whole capture, and it lets a reviewer read the twelve names at all five
    seeds, all three sources and all three densities directly. About 2 KB per cell.
  * `<out-dir>/full/<cell>.txt` -- three cells whole, at seed 42 and the default density, one per
    decision source, without the trace. A reviewer who wants to see that the *rest* of a stream is
    untouched -- not just its head -- has three complete 1,000-tick streams to hash and read.

Everything not retained is reproducible: `capture.sh` at the recorded commit reproduces the
pre-change capture, and at the candidate the post-change one. The digests in the manifests are what
detect a capture that failed to reproduce.
"""

import io
import os
import sys

FULL = [
    'seed42-baseline-d0.75-traceoff',
    'seed42-reference-d0.75-traceoff',
    'seed42-individual-d0.75-traceoff',
]


def read(path):
    return io.open(path, encoding='utf-8', newline='').read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    io.open(path, 'w', encoding='utf-8', newline='').write(text)


def initialization(text):
    """Every line up to and including the last `tick=0` line, in order."""
    lines = text.split('\n')
    last = max(index for index, line in enumerate(lines) if line.startswith('tick=0 '))
    return '\n'.join(lines[:last + 1]) + '\n'


def main():
    capture_dir, out_dir, label = sys.argv[1:4]
    cells = sorted(name[:-4] for name in os.listdir(capture_dir) if name.endswith('.txt'))
    for cell in cells:
        text = read(os.path.join(capture_dir, cell + '.txt'))
        write(os.path.join(out_dir, 'init', cell + '.txt'), initialization(text))
        if cell in FULL:
            write(os.path.join(out_dir, 'full', cell + '.txt'), text)
    missing = [cell for cell in FULL if cell not in cells]
    print(f'{label}: retained {len(cells)} initialization blocks and '
          f'{len(FULL) - len(missing)} whole streams')
    if missing:
        print('MISSING from the capture: ' + ', '.join(missing))
    return 1 if missing else 0


if __name__ == '__main__':
    sys.exit(main())
