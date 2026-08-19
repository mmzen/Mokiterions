"""VER-MOK-010 oracle 1: hash every captured cell, apply the projection, and compare.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-010/baseline/compare.py \
        <pre-capture-dir> <post-capture-dir> <output-dir>

Each capture directory is one produced by `capture.sh`: 90 `<cell>.txt` streams and 90 `<cell>.exit`
exit codes. Writes three files into the output directory:

  * pre-manifest.txt   -- per-cell digest of the pre-change capture, raw and projected
  * post-manifest.txt  -- the same for the post-change capture
  * additivity.txt     -- the comparison, cell by cell, and the overall result

The comparison is byte-exact through SHA-256 rather than by retaining 110 MB of streams twice, which
is the retention form `WO-MOK-006`'s evidence packet established for the same oracle and
`WO-MOK-007`'s reused. A stated subset of the pre-change capture is retained whole by `retain.py`,
because it is the one artifact here that cannot be reproduced from the candidate commit.

A cell passes on three conditions, all of them checked:

  * the projected post-change stream is byte-identical to the raw pre-change stream;
  * the projection is a no-op on the pre-change stream;
  * the exit codes are equal.

The third is checked because `REQ-MOK-040` states it and because an exit-code change is the one
difference a stream comparison cannot see.
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
    return sorted(name[:-4] for name in os.listdir(directory) if name.endswith('.txt'))


def write(path, text):
    io.open(path, 'w', encoding='utf-8', newline='\n').write(text)


def manifest(directory, out_path, label):
    lines = [
        f'# {label}',
        '# cell  sha256(raw)  sha256(projected)  bytes  lines  exit',
        '',
    ]
    recorded = {}
    for cell in cells(directory):
        text = read(os.path.join(directory, cell + '.txt'))
        code = read(os.path.join(directory, cell + '.exit')).strip()
        projected = project(text)
        recorded[cell] = (digest(text), digest(projected), len(text), code)
        lines.append(
            f'{cell:34} {digest(text)} {digest(projected)} '
            f'{len(text):9} {text.count(chr(10)):7} {code:>4}'
        )
    write(out_path, '\n'.join(lines) + '\n')
    return recorded


def main():
    pre_dir, post_dir, out_dir = sys.argv[1:4]
    commit = read(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'COMMIT.txt')).strip()
    pre = manifest(pre_dir, os.path.join(out_dir, 'pre-manifest.txt'),
                   f'VER-MOK-010 oracle 1: pre-change capture, commit {commit}')
    post = manifest(post_dir, os.path.join(out_dir, 'post-manifest.txt'),
                    'VER-MOK-010 oracle 1: post-change capture, the candidate tree')

    report = [
        'VER-MOK-010 oracle 1 - projected byte comparison',
        '',
        f'pre-change commit: {commit}',
        'projection: baseline/projection.py, applied to both captures',
        '',
        'A cell passes when the projected post-change stream is byte-identical to the pre-change',
        'stream, when the projection is a no-op on the pre-change stream, and when the two exit',
        'codes are equal. Equality is SHA-256 over the whole stream, not over summary counts.',
        '',
        f'{"cell":34} {"proj(post)==pre":16} {"proj(pre)==pre":15} {"exit":9} {"bytes removed":13}',
    ]
    failures = []
    for cell in sorted(pre):
        if cell not in post:
            failures.append(f'{cell}: absent from the post-change capture')
            continue
        pre_raw, pre_projected, pre_bytes, pre_code = pre[cell]
        post_raw, post_projected, post_bytes, post_code = post[cell]
        equal = post_projected == pre_raw
        noop = pre_projected == pre_raw
        codes = pre_code == post_code
        removed = post_bytes - len(project(read(os.path.join(post_dir, cell + '.txt'))))
        report.append(f'{cell:34} {str(equal):16} {str(noop):15} '
                      f'{pre_code + "==" + post_code:9} {removed:13}')
        if not (equal and noop and codes):
            failures.append(f'{cell}: projected-equal={equal} projection-noop-on-pre={noop} '
                            f'exit-equal={codes}')
    extra = sorted(set(post) - set(pre))
    for cell in extra:
        failures.append(f'{cell}: present in the post-change capture and absent from the pre-change one')

    report += ['', f'cells compared: {len(pre)}',
               f'cells in the post-change capture: {len(post)}',
               '', 'RESULT: ' + ('PASS' if not failures else 'FAIL')]
    if failures:
        report += ['', 'failures:'] + [f'  {failure}' for failure in failures]

    write(os.path.join(out_dir, 'additivity.txt'), '\n'.join(report) + '\n')
    print('\n'.join(report[-6:]))
    return 0 if not failures else 1


if __name__ == '__main__':
    sys.exit(main())
