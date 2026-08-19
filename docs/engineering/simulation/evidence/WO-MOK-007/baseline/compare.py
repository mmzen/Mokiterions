"""VER-MOK-007 oracle 1: hash every captured cell, apply the projection, and compare.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-007/baseline/compare.py \
        <pre-capture-dir> <post-capture-dir> <output-dir>

Writes three files into the output directory:

  * pre-manifest.txt   -- per-cell digest of the pre-change capture, raw and projected
  * post-manifest.txt  -- the same for the post-change capture
  * additivity.txt     -- the comparison, cell by cell, and the overall result

The comparison is byte-exact through SHA-256 rather than by retaining both streams in full, which is
the retention form WO-MOK-006's evidence packet established for the same oracle. The pre-change
1,000-tick default-density streams are retained in full as well, because they are the one artifact
here that cannot be reproduced from the candidate commit.
"""

import hashlib
import io
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from projection import project  # noqa: E402

sys.stdout.reconfigure(encoding='utf-8')


def read(path):
    return io.open(path, encoding='utf-8', newline='').read()


def digest(text):
    return hashlib.sha256(text.encode('utf-8')).hexdigest()


def cells(directory):
    raw = os.path.join(directory, 'raw')
    return sorted(name[:-4] for name in os.listdir(raw) if name.endswith('.log'))


def manifest(directory, out_path, label):
    lines = [
        f'# {label}',
        '# cell  sha256(raw)  sha256(projected)  bytes  lines',
        '',
    ]
    digests = {}
    for cell in cells(directory):
        text = read(os.path.join(directory, 'raw', cell + '.log'))
        projected = project(text)
        digests[cell] = (digest(text), digest(projected), len(text))
        lines.append(
            f'{cell:44} {digest(text)} {digest(projected)} '
            f'{len(text):9} {text.count(chr(10)):7}'
        )
    write(out_path, '\n'.join(lines) + '\n')
    return digests


def write(path, text):
    io.open(path, 'w', encoding='utf-8', newline='\n').write(text)


def main():
    pre_dir, post_dir, out_dir = sys.argv[1:4]
    pre = manifest(pre_dir, os.path.join(out_dir, 'pre-manifest.txt'),
                   'VER-MOK-007 oracle 1: pre-change capture, commit 60fda9faffbd452752a34efa356f16cc6ad1d3ff')
    post = manifest(post_dir, os.path.join(out_dir, 'post-manifest.txt'),
                    'VER-MOK-007 oracle 1: post-change capture, the candidate tree')

    report = [
        'VER-MOK-007 oracle 1 - projected byte comparison',
        '',
        'pre-change commit: 60fda9faffbd452752a34efa356f16cc6ad1d3ff',
        'projection: baseline/projection.py, applied to both captures',
        '',
        'A cell passes when the projected post-change stream is byte-identical to the pre-change',
        'stream, and when the projection is a no-op on the pre-change stream. Both are checked as',
        'SHA-256 equality over the whole stream, not over summary counts.',
        '',
        f'{"cell":44} {"proj(post)==pre":16} {"proj(pre)==pre":15} {"bytes removed":13}',
    ]
    frozen = [cell for cell in pre if '_baseline_' in cell or '_reference_' in cell
              or cell == 'no_arguments' or cell.startswith('short_seed42_reference')]
    failures = []
    for cell in sorted(frozen):
        if cell not in post:
            failures.append(f'{cell}: absent from the post-change capture')
            continue
        pre_raw, pre_projected, pre_bytes = pre[cell]
        post_raw, post_projected, post_bytes = post[cell]
        equal = post_projected == pre_raw
        noop = pre_projected == pre_raw
        removed = post_bytes - len(project(read(os.path.join(post_dir, 'raw', cell + '.log'))))
        report.append(f'{cell:44} {str(equal):16} {str(noop):15} {removed:13}')
        if not (equal and noop):
            failures.append(f'{cell}: projected-equal={equal} projection-noop-on-pre={noop}')

    report += ['', f'frozen-source cells compared: {len(frozen)}']

    # The new source's own reproducibility: two processes, one seed, one density, one tick limit.
    pairs = sorted({cell[:-2] for cell in post if cell.endswith('_a') and '_individual_' in cell})
    report += ['', 'REQ-MOK-009 under the new source: two processes per cell, byte-identical',
               f'{"cell":44} {"pass a == pass b":16}']
    for stem in pairs:
        a, b = post.get(stem + '_a'), post.get(stem + '_b')
        equal = a is not None and a == b
        report.append(f'{stem:44} {str(equal):16}')
        if not equal:
            failures.append(f'{stem}: the two processes differ')
    report += ['', f'new-source cells compared: {len(pairs)}']

    # The new source must differ from the reference source, or the change did nothing.
    report += ['', 'The new source differs from the reference source (it must, or nothing changed)',
               f'{"cell":44} {"differs":16}']
    differing = 0
    for stem in pairs:
        reference = stem.replace('_individual_', '_reference_')
        if reference in post and post[reference][0] != post[stem + '_a'][0]:
            differing += 1
            report.append(f'{stem:44} {"True":16}')
        elif reference in post:
            report.append(f'{stem:44} {"False":16}')
            failures.append(f'{stem}: identical to the reference source')
    report += ['', f'differing cells: {differing} of {len(pairs)}']

    report += ['', 'RESULT: ' + ('PASS' if not failures else 'FAIL')]
    if failures:
        report += ['', 'failures:'] + [f'  {failure}' for failure in failures]

    write(os.path.join(out_dir, 'additivity.txt'), '\n'.join(report) + '\n')
    print('\n'.join(report[-8:]))
    return 0 if not failures else 1


if __name__ == '__main__':
    sys.exit(main())
