"""VER-MOK-012 oracle 1: hash every captured cell of one capture directory.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-018/analysis/digest.py \
        <capture-dir> <output-file> <label>

A capture directory is one produced by `capture.sh`: per cell a `<cell>.txt` standard-output stream,
a `<cell>.err` standard-error stream, a `<cell>.exit` exit code, and -- in sink mode -- a
`<cell>.jsonl` record stream. Writes one manifest line per cell:

    <cell>  <sha256 of stdout>  <bytes>  <lines>  <sha256 of stderr>  <exit>  [<sha256 of sink>  <bytes>  <lines>]

The comparison is byte-exact through SHA-256 rather than by retaining a 130 MB capture three times,
which is the retention form `WO-MOK-006`'s evidence packet established for this oracle and
`WO-MOK-007`'s and `WO-MOK-011`'s reused. A stated subset is retained whole by `retain.py`, because
the pre-change capture is the one artifact here that cannot be reproduced from the candidate commit.

Digests are taken over the file's bytes exactly as written. Nothing is decoded, normalized or
newline-translated: `SPEC-MOK-006` rule 11.1 admits no whitespace exemption, so a comparison that
normalized anything would be verifying a weaker property than the one required.
"""

import hashlib
import io
import os
import sys

sys.stdout.reconfigure(encoding='utf-8')


def read(path):
    with io.open(path, 'rb') as handle:
        return handle.read()


def figures(path):
    """The digest, byte count and line count of one captured stream."""
    data = read(path)
    return hashlib.sha256(data).hexdigest(), len(data), data.count(b'\n')


def main():
    capture_dir, output_file, label = sys.argv[1:4]
    cells = sorted(name[:-4] for name in os.listdir(capture_dir) if name.endswith('.txt'))
    lines = [
        f'# {label}',
        '# cell  sha256(stdout)  bytes  lines  sha256(stderr)  exit  [sha256(sink)  bytes  lines]',
        '',
    ]
    sinks = 0
    for cell in cells:
        digest, size, count = figures(os.path.join(capture_dir, cell + '.txt'))
        error_digest, error_size, _ = figures(os.path.join(capture_dir, cell + '.err'))
        code = read(os.path.join(capture_dir, cell + '.exit')).decode('ascii').strip()
        row = f'{cell:<34} {digest} {size:>9} {count:>7} {error_digest} {error_size:>4} {code:>2}'
        sink_path = os.path.join(capture_dir, cell + '.jsonl')
        if os.path.exists(sink_path):
            sink_digest, sink_size, sink_count = figures(sink_path)
            row += f' {sink_digest} {sink_size:>10} {sink_count:>7}'
            sinks += 1
        lines.append(row)
    lines.append('')
    lines.append(f'# {len(cells)} cells, {sinks} with a record stream')
    os.makedirs(os.path.dirname(os.path.abspath(output_file)), exist_ok=True)
    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines))
    print(f'{label}: {len(cells)} cells digested, {sinks} with a record stream')
    return 0


if __name__ == '__main__':
    sys.exit(main())
