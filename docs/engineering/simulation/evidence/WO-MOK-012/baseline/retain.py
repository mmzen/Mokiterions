"""VER-MOK-012 oracle 1: retain the part of a 110 MB capture that a reviewer needs.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-012/baseline/retain.py \
        <capture-dir> <out-dir> <label>

`VER-MOK-012` retains this capture "by per-cell SHA-256 digest, with a stated subset retained whole".
`manifest.py` writes the digests. This script writes the stated subset, and the subset is stated here:

  * `<out-dir>/init/<cell>.txt` -- the initialization block of **every** cell: every line up to and
    including the last `tick=0 ` line. About 2 KB per cell. This change does not touch initialization,
    which is exactly why the block is worth 180 KB: it is the cheapest per-cell demonstration that the
    two captures are of the same 90 worlds, and it carries the food table that oracle 1's companion
    check on rules 14 to 16 is read on. `WO-MOK-011` retained the same block for the same cells, so
    the two work orders' captures can be compared without a reader reconciling two retention schemes.
  * `<out-dir>/summary.txt` -- rule 18's final summary line of every cell, one line per cell. This is
    the artifact `VER-MOK-012` requires "at both commits", and at 90 lines it is the whole of it.
  * `<out-dir>/census.txt` -- per cell, the count of every `event=` kind, of every verb proposed, and
    of the two fields this change adds to `action_trace`. Its value on this side of the change is that
    it is all zeros where it matters: **no `target:` field, no `suffered:` field and none of the seven
    targeted verbs occurs anywhere in the 110 MB.** Oracle 5 and oracle 6 are claims of absence, and a
    claim of absence is worth more when the same measurement is on record before the change as well as
    after it.
  * `<out-dir>/full/seed42-baseline-d0.75-traceon.txt` -- one cell whole, 414 KB. It is a *traced*
    `baseline` cell, which is where a `target` or `suffered` field would appear first if this change
    leaked into the source `CAP-MOK-009` forbids it to touch. Oracle 1's load-bearing claim is byte
    equality on the 30 `baseline` cells, and this is the single cell on which a reviewer can read that
    claim instead of recomputing a digest.

What is deliberately **not** retained whole, with the reason:

  * The three untraced `seed42-*-d0.75-traceoff` streams `WO-MOK-011` retained. They are already in the
    repository under `evidence/WO-MOK-011/post/full/`, and `cross-check.txt` records that all three
    are byte-identical to this capture's copies. Retaining them again would add 2.6 MB and no fact.
  * A whole traced `reference` or `individual` stream, 3 MB each. Their divergence after the change is
    expected under `REQ-MOK-051` and is *characterized by computed output* over both captures, not by
    reading two 23,000-line streams side by side. A reviewer who wants them reproduces the capture
    from the recorded commit: `capture.sh` takes 5 seconds for all 90 cells.

Everything not retained is reproducible from `COMMIT.txt` and `capture.sh`, and `pre-manifest.txt` is
what detects a reproduction that failed to reproduce.
"""

import io
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')

FULL = ['seed42-baseline-d0.75-traceon']

# The seven verbs REQ-MOK-042 adds, and the two fields REQ-MOK-043 adds. Counted by name rather than
# discovered, so that a zero is reported as a zero instead of vanishing from the census.
NEW_VERBS = ['attack', 'threaten', 'retreat', 'surrender', 'avoid', 'approach', 'observe']
NEW_FIELDS = ['target', 'suffered']

EVENT = re.compile(r' event=([a-z_]+)')
VERB = re.compile(r' result=proposal:([a-z]+)')


def read(path):
    return io.open(path, encoding='utf-8', newline='').read()


def write(path, text):
    directory = os.path.dirname(path)
    if directory:
        os.makedirs(directory, exist_ok=True)
    io.open(path, 'w', encoding='utf-8', newline='').write(text)


def initialization(text):
    """Every line up to and including the last `tick=0 ` line, in order."""
    lines = text.split('\n')
    last = max(index for index, line in enumerate(lines) if line.startswith('tick=0 '))
    return '\n'.join(lines[:last + 1]) + '\n'


def summary(text):
    """Rule 18's final summary line."""
    for line in reversed(text.split('\n')):
        if line.startswith('summary '):
            return line
    return '(no summary line)'


def census(text):
    """One line of `key=count` pairs, and the counts that must be zero here, as a dict."""
    events, verbs = {}, {}
    for kind in EVENT.findall(text):
        events[kind] = events.get(kind, 0) + 1
    for verb in VERB.findall(text):
        verbs[verb] = verbs.get(verb, 0) + 1
    must_be_zero = {f'verb:{v}': verbs.get(v, 0) for v in NEW_VERBS}
    must_be_zero.update({f'field:{f}': text.count(f',{f}:') for f in NEW_FIELDS})
    parts = [f'event:{k}={events[k]}' for k in sorted(events)]
    parts += [f'verb:{v}={verbs.get(v, 0)}' for v in sorted(set(verbs) | set(NEW_VERBS))]
    parts += [f'field:{f}={text.count(f",{f}:")}' for f in NEW_FIELDS]
    return ' '.join(parts), must_be_zero


def main():
    capture_dir, out_dir, label = sys.argv[1:4]
    cells = sorted(name[:-4] for name in os.listdir(capture_dir) if name.endswith('.txt'))
    summaries = [f'# {label}', '# rule 18 final summary line, one per cell', '']
    censuses = [f'# {label}',
                '# per cell: count of each event kind, each verb proposed, and each field',
                '# REQ-MOK-042 added no verb and REQ-MOK-043 no field at this commit, so every',
                '# targeted verb and both fields are listed here at zero.', '']
    nonzero = []
    for cell in cells:
        text = read(os.path.join(capture_dir, cell + '.txt'))
        write(os.path.join(out_dir, 'init', cell + '.txt'), initialization(text))
        summaries.append(f'{cell:34} {summary(text)}')
        row, must_be_zero = census(text)
        censuses.append(f'{cell:34} {row}')
        nonzero += [f'{cell} {key}={count}' for key, count in sorted(must_be_zero.items()) if count]
        if cell in FULL:
            write(os.path.join(out_dir, 'full', cell + '.txt'), text)
    write(os.path.join(out_dir, 'summary.txt'), '\n'.join(summaries) + '\n')
    write(os.path.join(out_dir, 'census.txt'), '\n'.join(censuses) + '\n')
    missing = [cell for cell in FULL if cell not in cells]
    print(f'{label}: retained {len(cells)} initialization blocks, {len(cells)} summary lines, '
          f'{len(cells)} census rows, and {len(FULL) - len(missing)} of {len(FULL)} '
          f'stated cells whole')
    print(f'  seven targeted verbs and both new fields, across all {len(cells)} cells: '
          f'{"all zero" if not nonzero else "NOT all zero"}')
    if missing:
        print('MISSING from the capture: ' + ', '.join(missing))
    for line in nonzero:
        print('UNEXPECTED before the change: ' + line)
    return 1 if missing or nonzero else 0


if __name__ == '__main__':
    sys.exit(main())
