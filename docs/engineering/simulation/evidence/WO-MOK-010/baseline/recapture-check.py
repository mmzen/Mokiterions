"""WO-MOK-010: the pre-change capture was completed by recapture, and this checks the recapture.

VER-MOK-010 oracle 1 declares a matrix of 40 cells. Eleven of them -- the ten 1,000-tick
default-density frozen-source runs and one 20-tick traced run -- were captured before the first line
of code changed, and are retained in full under `baseline/full/`. The remaining 31 cells (the 1.50%
density and the `--trace-actions` variants) were not, and were captured afterwards from a clean git
worktree at the same commit, 60fda9faffbd452752a34efa356f16cc6ad1d3ff.

That is a recapture, and oracle 1 forbids recapturing the baseline to resolve a discrepancy. It was
not used to resolve one: no discrepancy existed, and the 31 cells are ones the original capture never
covered. This script is the check that makes the recapture auditable rather than asserted -- it
compares the eleven cells the two captures have in common, byte for byte. If the worktree build
reproduces those eleven exactly, the 31 it adds are from the same world.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-010/baseline/recapture-check.py <recapture-dir>
"""

import hashlib
import io
import os
import sys

sys.stdout.reconfigure(encoding='utf-8')

HERE = os.path.dirname(os.path.abspath(__file__))
SEEDS = (0, 1, 42, 123, 777)


def digest(path):
    return hashlib.sha256(
        io.open(path, encoding='utf-8', newline='').read().encode('utf-8')
    ).hexdigest()


def main():
    recapture = os.path.join(sys.argv[1], 'raw')
    pairs = [
        (f'full/pre-{policy}-seed{seed}-ticks1000.log',
         f'seed{seed}_{policy}_d0.75_traceoff.log')
        for policy in ('baseline', 'reference')
        for seed in SEEDS
    ]
    pairs.append(('full/pre-reference-seed42-ticks20-trace.log',
                  'short_seed42_reference_traceon.log'))

    lines = [
        'WO-MOK-010 - recapture fidelity of the pre-change baseline',
        '',
        'commit: 60fda9faffbd452752a34efa356f16cc6ad1d3ff, built in a clean git worktree',
        'compared: the eleven cells captured before the first code change against their recapture',
        '',
        f'{"cell captured before the change":48} {"identical":9}  sha256',
    ]
    ok = True
    for retained, recaptured in pairs:
        left = digest(os.path.join(HERE, retained))
        right = digest(os.path.join(recapture, recaptured))
        ok = ok and left == right
        lines.append(f'{os.path.basename(retained):48} {str(left == right):9}  {left}')

    lines += ['', f'cells compared: {len(pairs)}',
              '', 'RESULT: ' + ('PASS' if ok else 'FAIL')]
    text = '\n'.join(lines) + '\n'
    io.open(os.path.join(HERE, 'recapture-check.txt'), 'w',
            encoding='utf-8', newline='\n').write(text)
    print(text)
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
