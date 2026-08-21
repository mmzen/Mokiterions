"""VER-MOK-012: the retained sink-stream subset, and the proof that no retained stream carries the
path it was written to.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-018/analysis/retain-sink.py \
        <binary> <scratch-dir> <evidence-dir> <output-file>

## The retention this discharges, and the deviation from it

`VER-MOK-012` asks for "one full sink stream per declared seed at the default density under each
policy, with tracing off and on, retained as the primary artifact this phase produces". Taken
literally that is five seeds x three policies x two trace settings = **thirty whole record streams**,
and `sizes.txt` measured what they weigh: the default-density record stream is 2.7 MB untraced and
5.6 MB traced, so the thirty come to roughly 120 MB. Every evidence directory in this repository put
together is smaller than that -- `WO-MOK-010` retained 7.1 MB and `WO-MOK-011` 8.4 MB -- so the
retention as written cannot be satisfied, and the deviation is stated rather than quietly resolved.

What is retained instead is a bounded subset chosen so that each dimension of the declared set is
represented by a whole stream:

  * **all three policies**, at seed 42 and the default density, untraced. These are the same three
    cells `WO-MOK-011` retained whole and the same three this packet already retains as pre-change
    *text* streams in `baseline/full/`, so the record stream and the text stream of one run sit side
    by side, and the two packets' retained artifacts compare directly;
  * **tracing on**, as one whole stream. The traced cell retained is the baseline policy's, because at
    743 KB it is the smallest traced cell in the matrix and therefore the only one that can be
    retained whole without doubling the packet.

That is four whole record streams. The other 86 cells of the captured matrix are not absent from the
evidence: `post-sink-manifest.txt` carries the SHA-256, byte count and line count of every one of
them, and oracle 1's three comparisons are computed over all 90. A reviewer re-running `capture.sh`
at the recorded commit gets bytes whose digests either match that manifest or do not.

## Why the path check is a comparison and not a search

The obligation is that **no retained sink stream carries the path it was written to**, which is what
makes this evidence class safe to retain at all. A search for the path would establish only that one
spelling of it is absent. So each cell is run **twice, to two deliberately different destinations** --
one short, one a long nested directory whose name contains the marker `events_path_marker_A7B` -- and
the two record streams are required to be byte-identical. Byte-identity across two destinations is
the property itself: the bytes cannot depend on the path, because the path changed and they did not.

Three further things are then reported, in decreasing order of strength:

  * the digest of each retained stream against `post-sink-manifest.txt`, which was taken at a **third**
    destination during the capture. Three paths, one digest;
  * the complete set of distinct characters in the retained bytes. Neither `/` nor `\\` is among them,
    and no path can be spelled without one of them on any platform this repository builds for;
  * a substring scan for the marker, for the destination's own components, and for `events_path` --
    the key `SPEC-MOK-006` rule 5.4 shows in a counterexample precisely so that it is never emitted.

The character enumeration is the one that generalises: it is closed over the whole retained byte
range rather than over a list of spellings someone thought of.
"""

import hashlib
import io
import os
import shutil
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8')

MARKER = 'events_path_marker_A7B'

# cell, and why it is in the retained subset
RETAINED = [
    ('seed42-baseline-d0.75-traceoff',
     'the baseline policy, untraced — the smallest of the three, and the one whose population dies'),
    ('seed42-reference-d0.75-traceoff',
     'the reference policy, untraced — the default decision source'),
    ('seed42-individual-d0.75-traceoff',
     'the individual policy, untraced — the only one that reaches the tick limit here'),
    ('seed42-baseline-d0.75-traceon',
     'tracing on — the smallest traced cell in the matrix, so it can be retained whole'),
]

# Path-shaped needles. `events_path` is rule 5.4's counterexample key; the two separators are what a
# path cannot be spelled without.
NEEDLES = ('events_path', 'events-path', MARKER, '.jsonl', 'jsonl', '/', '\\', 'C:', '..')


def digest(path):
    hash = hashlib.sha256()
    with io.open(path, 'rb') as handle:
        for block in iter(lambda: handle.read(1 << 20), b''):
            hash.update(block)
    return hash.hexdigest()


def cell_arguments(cell):
    """The command line one capture cell stands for, parsed back out of its name."""
    seed, policy, density, trace = cell.split('-')
    arguments = [
        '--seed', seed.removeprefix('seed'),
        '--ticks', '1000',
        '--policy', policy,
        '--density', density.removeprefix('d'),
    ]
    if trace == 'traceon':
        arguments.append('--trace-actions')
    return arguments


def run(binary, cell, destination):
    """One run of one cell, with its sink at `destination`. Returns (stdout, stderr, code)."""
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    completed = subprocess.run(
        [binary, *cell_arguments(cell), '--events-path', destination], capture_output=True
    )
    return completed.stdout, completed.stderr, completed.returncode


def manifest_rows(path):
    rows = {}
    for line in io.open(path, encoding='utf-8').read().split('\n'):
        if not line.strip() or line.startswith('#'):
            continue
        parts = line.split()
        rows[parts[0]] = {
            'stdout': parts[1], 'bytes': int(parts[2]), 'lines': int(parts[3]),
            'stderr': parts[4], 'exit': int(parts[5]),
            'sink': parts[7], 'sink_bytes': int(parts[8]), 'sink_lines': int(parts[9]),
        }
    return rows


def main():
    binary, scratch, evidence, output_file = sys.argv[1:5]
    binary = os.path.abspath(binary)
    manifest = manifest_rows(os.path.join(evidence, 'post-sink-manifest.txt'))
    out_dir = os.path.join(evidence, 'post', 'full')
    os.makedirs(out_dir, exist_ok=True)

    lines = [
        '# VER-MOK-012: the retained sink-stream subset, and the path-independence of its bytes',
        '#',
        f'# binary: {binary}',
        '# command: python docs/engineering/simulation/evidence/WO-MOK-018/analysis/'
        'retain-sink.py \\',
        '#              <binary> <scratch-dir> <evidence-dir> <output-file>',
        '#',
        '# Each cell is run twice, to two different destinations, and the record streams are required',
        '# to be byte-identical. The digest is then compared against post-sink-manifest.txt, which was',
        '# taken at a third destination during the capture.',
        '',
    ]

    rows, failures, retained_bytes = [], [], 0
    characters = set()
    for cell, why in RETAINED:
        short = os.path.join(scratch, 'a', f'{cell}.jsonl')
        long = os.path.join(
            scratch, MARKER, 'a-deeply-nested-directory', 'and-another', f'{cell}.jsonl'
        )
        for path in (short, long):
            if os.path.exists(path):
                os.remove(path)

        first_out, first_err, first_code = run(binary, cell, short)
        second_out, second_err, second_code = run(binary, cell, long)

        first_bytes = io.open(short, 'rb').read()
        second_bytes = io.open(long, 'rb').read()

        expected = manifest[cell]
        checks = [
            ('two destinations, identical record bytes', first_bytes == second_bytes),
            ('two destinations, identical text stream', first_out == second_out),
            ('two destinations, identical exit code', first_code == second_code),
            ('record digest matches post-sink-manifest.txt',
             hashlib.sha256(first_bytes).hexdigest() == expected['sink']),
            ('record byte count matches the manifest', len(first_bytes) == expected['sink_bytes']),
            ('text digest matches the manifest',
             hashlib.sha256(first_out).hexdigest() == expected['stdout']),
            ('diagnostic stream empty', first_err == b'' and second_err == b''),
            ('exit code 0', first_code == 0),
        ]
        for label, ok in checks:
            if not ok:
                failures.append(f'{cell}: {label}')

        target = os.path.join(out_dir, f'{cell}.jsonl')
        shutil.copyfile(short, target)
        retained_bytes += len(first_bytes)
        characters |= set(first_bytes.decode('utf-8'))

        found = [
            (needle, first_bytes.decode('utf-8').count(needle))
            for needle in NEEDLES
            if needle in first_bytes.decode('utf-8')
        ]
        if found:
            failures.append(f'{cell}: path-shaped substring present: {found}')

        rows.append((cell, why, len(first_bytes), first_bytes.count(b'\n'), checks, found))
        os.remove(short)
        os.remove(long)

    # -------------------------------------------------------------------------------------------
    # Per cell.
    # -------------------------------------------------------------------------------------------
    for cell, why, size, count, checks, found in rows:
        lines.append('=' * 108)
        lines.append(f'{cell}')
        lines.append('=' * 108)
        lines.append(f'  why retained: {why}')
        lines.append(f'  command:      {" ".join(cell_arguments(cell))} --events-path <destination>')
        lines.append(f'  retained as:  post/full/{cell}.jsonl   {size:,} bytes, {count:,} records')
        lines.append(f'  destination 1: <scratch>/a/{cell}.jsonl')
        lines.append(f'  destination 2: <scratch>/{MARKER}/a-deeply-nested-directory/and-another/'
                     f'{cell}.jsonl')
        lines.append('')
        for label, ok in checks:
            lines.append(f'    {"yes" if ok else "NO ":<4} {label}')
        lines.append(
            f'    {"yes" if not found else "NO ":<4} no path-shaped substring occurs '
            f'({len(NEEDLES)} searched for)'
        )
        if found:
            lines.append(f'         found: {found}')
        lines.append('')

    # -------------------------------------------------------------------------------------------
    # The character enumeration -- the claim that closes over the bytes rather than over a list.
    # -------------------------------------------------------------------------------------------
    ordered = sorted(characters)
    printable = ''.join(character for character in ordered if character not in '\n\r\t')
    separators = [character for character in ('/', '\\') if character in characters]
    lines.append('=' * 108)
    lines.append('every distinct character in the retained bytes')
    lines.append('=' * 108)
    lines.append(f'  {len(characters)} distinct characters over {retained_bytes:,} retained bytes:')
    lines.append(f'    {printable}')
    control = {'\n': 'LF (0x0A), the record terminator', '\r': 'CR (0x0D)', '\t': 'TAB (0x09)'}
    lines.append(
        '    and, outside the printable range: '
        + ', '.join(control[character] for character in ordered if character in control)
    )
    lines.append('')
    lines.append(
        f'  path separators present: {separators if separators else "none — neither / nor \\"}'
    )
    lines.append(
        '  A path cannot be spelled without one of those two on any platform this repository builds'
    )
    lines.append(
        '  for, so the absence of both is not a statement about the destinations used here: it is a'
    )
    lines.append(
        '  statement about every destination. The byte range is closed and is printed above in full.'
    )
    if separators:
        failures.append(f'a path separator occurs in the retained bytes: {separators}')
    lines.append('')

    # -------------------------------------------------------------------------------------------
    # The retention deviation, with the figures it rests on.
    # -------------------------------------------------------------------------------------------
    declared_cells = 5 * 3 * 2
    lines.append('=' * 108)
    lines.append('the retention deviation')
    lines.append('=' * 108)
    lines.append(
        f'  VER-MOK-012 asks for {declared_cells} whole record streams: five declared seeds x three'
    )
    lines.append(
        '  policies x tracing off and on, at the default density. sizes.txt measured the weight: 2.7 MB'
    )
    lines.append(
        f'  untraced and 5.6 MB traced per stream, so the {declared_cells} come to roughly 120 MB.'
    )
    lines.append(
        '  WO-MOK-010 retained 7.1 MB of evidence and WO-MOK-011 8.4 MB. The retention as written'
    )
    lines.append('  cannot be satisfied, and this is the deviation rather than a resolution of it.')
    lines.append('')
    lines.append(
        f'  retained here: {len(rows)} whole record streams, {retained_bytes:,} bytes '
        f'({retained_bytes / (1 << 20):.1f} MiB)'
    )
    lines.append(
        f'  not retained:  {len(manifest) - len(rows)} cells, each with its SHA-256, byte count and'
    )
    lines.append(
        '                 line count in post-sink-manifest.txt, and each reproducible by capture.sh'
    )
    lines.append('                 at the recorded commit.')
    lines.append(
        '  the text stream of the same three untraced cells is retained pre-change in baseline/full/,'
    )
    lines.append(
        '  and oracle 1 establishes it is byte-identical post-change with and without a sink, so it is'
    )
    lines.append('  not duplicated here.')
    lines.append('')
    lines.append(f'# result: {"FAIL" if failures else "PASS"}')
    if failures:
        lines.append('')
        for failure in failures:
            lines.append(f'  {failure}')
    lines.append('')
    lines.append('# ---- full text of this script, retained as VER-MOK-012 requires ----')
    lines.append('')
    lines.extend(io.open(__file__, encoding='utf-8').read().split('\n'))

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(
        f'{len(rows)} record streams retained, {retained_bytes:,} bytes, '
        f'{len(characters)} distinct characters, {len(failures)} failing; written to {output_file}'
    )
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
