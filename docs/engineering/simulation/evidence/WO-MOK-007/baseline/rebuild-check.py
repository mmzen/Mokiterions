"""WO-MOK-007: the committed source rebuilt, and every captured cell reproduced byte for byte.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-007/baseline/rebuild-check.py \
        <first-capture-dir> <second-capture-dir>

Why this check exists, stated plainly because it records two mistakes. The first capture of the
post-change matrix -- the capture every figure in this packet is derived from -- was taken with a
binary that then fell behind the source in the tree, twice:

  1. `cargo fmt` reformatted the engine source after that binary had been built. Nothing in the
     evidence itself would have shown it; the discrepancy was found by comparing the binary's
     timestamp against the source's.
  2. Two further edits landed in `simulation.rs` afterwards, both while the completion summary was
     being assembled: one test was moved out of the crate's `#[cfg(test)]` block into the public
     tier, and one `debug_assert!` invariant on the observation was tightened from `ATTRIBUTE_MAX`
     to `WASTE_TOLERANCE_MAX`, the bound the amended range actually states.

Each of those has a reason it cannot matter -- reformatting moves whitespace; test code is not
linked into the binary; `debug_assert!` expands to nothing in a release build and the whole suite
runs in a debug build without this one firing. Those are exactly the kind of claim this repository
does not accept on reasoning alone. So the tree was rebuilt from the committed source and the whole
matrix was captured again, and this compares that capture against the first one cell by cell by
SHA-256 over the entire stream. If every cell is identical, the evidence captured with the earlier
binary is evidence about the committed source, and all three edits are shown -- not argued -- to
have changed nothing.

Cells present in only one capture are reported rather than skipped: the later capture was taken
after the ten tick-10,000 long-horizon runs had been added to the first, so the first is the larger
set, and a reader should see that asymmetry named instead of inferring it from a count.
"""

import hashlib
import io
import os
import sys

sys.stdout.reconfigure(encoding='utf-8')

HERE = os.path.dirname(os.path.abspath(__file__))


def digest(path):
    return hashlib.sha256(
        io.open(path, encoding='utf-8', newline='').read().encode('utf-8')
    ).hexdigest()


def cells(directory):
    raw = os.path.join(directory, 'raw')
    return {name: os.path.join(raw, name) for name in sorted(os.listdir(raw))}


def main():
    first_dir, second_dir = sys.argv[1:3]
    first, second = cells(first_dir), cells(second_dir)
    shared = sorted(set(first) & set(second))
    only_first = sorted(set(first) - set(second))
    only_second = sorted(set(second) - set(first))

    lines = [
        'WO-MOK-007 - the committed source rebuilt, and the captured cells reproduced',
        '',
        'Why this check exists, including the two mistakes it records: see the header of',
        'baseline/rebuild-check.py.',
        '',
        f'first capture:  {first_dir}   the capture every figure in this packet derives from,',
        '                             taken with a binary that then fell behind the source twice',
        f'second capture: {second_dir}   binary rebuilt from the committed source',
        '',
        f'  cells in both captures:      {len(shared)}',
        f'  cells only in the first:     {len(only_first)}',
        f'  cells only in the second:    {len(only_second)}',
        '',
    ]

    identical = []
    differing = []
    for name in shared:
        left, right = digest(first[name]), digest(second[name])
        (identical if left == right else differing).append((name, left, right))

    lines += [f'  identical, byte for byte:    {len(identical)}',
              f'  differing:                   {len(differing)}',
              '',
              'Every cell in both captures, with the digest that is common to the two:',
              '']
    width = max(len(name) for name in shared)
    for name, left, right in identical:
        lines.append(f'  {name.ljust(width)}  identical  {left}')
    for name, left, right in differing:
        lines.append(f'  {name.ljust(width)}  DIFFERS    {left}  vs  {right}')

    if only_first:
        lines += ['',
                  'Present in the first capture only. These are the tick-10,000 long-horizon runs, which',
                  'were added to the first capture after it was taken and were not re-run for the second;',
                  'they carry no obligation in either direction and `measurements/long-horizon.txt` says',
                  'so. They are named here so the asymmetry in the counts above is not left to inference:',
                  '']
        lines += [f'  {name}' for name in only_first]
    if only_second:
        lines += ['', 'Present in the second capture only:', '']
        lines += [f'  {name}' for name in only_second]

    ok = not differing and bool(shared)
    lines += ['',
              'RESULT: ' + ('PASS - the rebuilt binary reproduced every shared cell byte for byte, so '
                            'none of the\n        three later edits changed any behaviour and the '
                            'retained evidence is evidence\n        about the committed source'
                            if ok else 'FAIL')]

    text = '\n'.join(lines) + '\n'
    io.open(os.path.join(HERE, 'rebuild-check.txt'), 'w',
            encoding='utf-8', newline='\n').write(text)
    print(text)
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
