"""VER-MOK-012 oracle 1: retain the part of a 110 MB capture that a reviewer needs.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-012/analysis/retain.py \
        <capture-dir> <out-dir> <label>

The declared matrix is 90 cells and 110 MB of standard output, and the post-change capture is taken
twice -- once with no sink and once with a sink -- so retaining captures whole would put a third of a
gigabyte in the repository. What is committed instead, and why:

  * the manifest written by `digest.py` -- the SHA-256, byte count and line count of every cell's
    standard output, standard error, exit code and, in sink mode, record stream. This is what makes
    the comparison checkable rather than asserted, and it is byte-exact.
  * `<out-dir>/full/<cell>.txt` -- three cells whole, at seed 42 and the default density, one per
    decision source, without the trace. These are the same three cells `WO-MOK-011`'s evidence packet
    retained, so the two packets' retained artifacts compare directly, and a reviewer who wants to
    see that the *whole* of a stream is untouched rather than its head has three complete 1,000-tick
    streams to read and hash.
  * `<out-dir>/full/<cell>.jsonl`, where the capture has one -- the record stream of the same three
    cells, which is the primary artifact this phase produces.

Everything not retained is reproducible: `capture.sh` at the recorded commit reproduces the
pre-change capture, and at the candidate commit the two post-change captures. The digests in the
manifests are what detect a capture that failed to reproduce.
"""

import io
import os
import shutil
import sys

sys.stdout.reconfigure(encoding='utf-8')

FULL = [
    'seed42-baseline-d0.75-traceoff',
    'seed42-reference-d0.75-traceoff',
    'seed42-individual-d0.75-traceoff',
]


def main():
    capture_dir, out_dir, label = sys.argv[1:4]
    cells = sorted(name[:-4] for name in os.listdir(capture_dir) if name.endswith('.txt'))
    retained = 0
    streams = 0
    for cell in FULL:
        if cell not in cells:
            continue
        os.makedirs(os.path.join(out_dir, 'full'), exist_ok=True)
        shutil.copyfile(
            os.path.join(capture_dir, cell + '.txt'),
            os.path.join(out_dir, 'full', cell + '.txt'),
        )
        retained += 1
        sink = os.path.join(capture_dir, cell + '.jsonl')
        if os.path.exists(sink):
            shutil.copyfile(sink, os.path.join(out_dir, 'full', cell + '.jsonl'))
            streams += 1
    missing = [cell for cell in FULL if cell not in cells]
    print(f'{label}: retained {retained} whole text streams and {streams} whole record streams '
          f'of {len(cells)} captured cells')
    if missing:
        print('MISSING from the capture: ' + ', '.join(missing))
    return 1 if missing else 0


if __name__ == '__main__':
    sys.exit(main())
