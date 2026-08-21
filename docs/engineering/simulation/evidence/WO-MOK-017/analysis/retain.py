"""VER-MOK-016 oracle 1 under WO-MOK-017: retain the part of a 156 MB capture a reviewer needs.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/retain.py \
        <capture-dir> <out-dir> <label>

`VER-MOK-016` retains this capture "by per-cell SHA-256 digest, with a stated subset retained whole".
`manifest.py` writes the digests. This script writes the stated subset, and the subset is stated here:

  * `<out-dir>/init/<cell>.txt` -- the initialization block of **every** cell: every line up to and
    including the last `tick=0 ` line, which is one `world_initialized`, one `food_initialized` per
    resource, one `agent_initialized` per Mokiterion and rule 20's `decision_source_selected`. This
    change does not touch initialization, which is why the block is worth retaining: it is the cheapest
    per-cell demonstration that the two captures are of the same 120 worlds, and it carries the initial
    food placement that the check on rules 14 to 16 is read against. `WO-MOK-011` and `WO-MOK-016`
    retained the same block, so the three captures compare without a reader reconciling three schemes.
  * `<out-dir>/summary.txt` -- rule 18's final summary line of every cell, one line per cell. This is
    the artifact `VER-MOK-016` requires "at both commits", and at 120 lines it is the whole of it. It
    carries the six food counts `REQ-MOK-060`'s ceiling is measured from and the survivor count the
    three floors are measured from, so the composition and survivor analyses read this file rather than
    the capture, and a reviewer can recompute every headline figure from a committed 120-line file.
  * `<out-dir>/census.txt` -- per cell, the count of every `event=` kind and of every verb proposed.
    Verb counts come from `action_trace` and are therefore present only in the 60 traced cells; the
    event counts are present in all 120. On this change the load-bearing rows are `event:food_consumed`
    and `verb:eat`: the corrected condition is expected to raise both under `reference`, `individual`
    and `social`, and to leave both **exactly** unchanged under `baseline`, whose rule 4 candidate list
    carries no waste condition at all.
  * `<out-dir>/eaten.txt` -- per cell and per calorie class, the number of resources consumed, the
    **highest pre-eat satiety** at which one was consumed, and the satiety wasted above the attribute
    maximum. Every field is read from `food_consumed`, which rule 17 emits with `satiety:<before>-><after>`
    in every cell whether or not actions are traced, so waste is `before + restoration - after` and is
    measured rather than inferred.

    This is the census that makes the corrected condition visible **in the stream** instead of inferred
    from the composition it produces, and the highest pre-eat satiety is the column that does it. Under
    `reference` before the change that column cannot exceed `85`, `70` and `50` for low, medium and high
    class, because rule 5's condition admits nothing above them; after the change it cannot exceed `87`,
    `79` and `75`. Those are the two numbers `SPEC-MOK-001`'s amendment states, read directly off 120
    cells. Under `baseline` the column is unbounded at both commits and the wasted total is non-zero at
    both, which is the same fact from the other side: the source this change must not reach never had
    the condition to relax.
  * `<out-dir>/full/seed42-baseline-d0.75-traceon.txt` -- one cell whole. It is a *traced* `baseline`
    cell, which is where a divergence would appear first if this change leaked into the source
    `REQ-MOK-060` forbids it to touch. Oracle 1's load-bearing claim is byte equality on the 30
    `baseline` cells, and this is the single cell on which a reviewer can read that claim instead of
    recomputing a digest. `WO-MOK-016` retained the same cell.
  * `<out-dir>/full/seed42-reference-d0.15-traceon.txt` -- one cell whole, on **this** work order and
    not on `WO-MOK-016`. That work order retained no traced `reference` stream because its divergence
    was expected to be empty. Here the divergence is the deliverable, so one obligated cell is retained
    whole at both commits and a reviewer can read the changed decision itself -- the tick at which the
    two streams part, and the proposal that parted them -- rather than trusting the divergence analysis
    that reports the same thing for all 90 obligated cells.

    The cell is at density `0.15` and not the obligated default of `0.75`, and the reason is the cost:
    the traced `reference` cell at `0.75` is 2,971,142 bytes against this one's 854,427, and retaining
    it at both commits would add nearly 6 MB for a reading this cell already supports. Every
    `reference` cell of the declared matrix carries the divergence obligation, not only the default
    density, and at 6,241 lines this one is the more readable worked example. Nothing is lost on the
    composition side by the choice: `summary.txt` carries rule 18's counts for all 120 cells including
    every default-density one, so the ceiling and the floors are recomputable from this packet in full,
    and it is only the line-level reading of one divergence that is localized to a single cell.

What is deliberately **not** retained whole, with the reason:

  * The other 118 streams, about 156 MB. Everything not retained is reproducible from `COMMIT.txt` and
    `capture.sh` in six seconds for the whole matrix, and the manifest is what detects a reproduction
    that failed to reproduce.
"""

import io
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

FULL = ['seed42-baseline-d0.75-traceon', 'seed42-reference-d0.15-traceon']

# The food table of SPEC-MOK-001 rule 6, restated here only so that wasted satiety can be computed
# from `food_consumed`. A drift between this table and the engine's would show up as a negative waste,
# which is asserted against below rather than left to a reader to notice.
RESTORATION = {'low': 15, 'medium': 30, 'high': 50}

EVENT = re.compile(r' event=([a-z_]+)')
VERB = re.compile(r' result=proposal:([a-z]+)')
CONSUMED = re.compile(r' event=food_consumed result=food:[^,]+,class:(\w+),satiety:(\d+)->(\d+)')


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
    events, verbs = {}, {}
    for kind in EVENT.findall(text):
        events[kind] = events.get(kind, 0) + 1
    for verb in VERB.findall(text):
        verbs[verb] = verbs.get(verb, 0) + 1
    parts = [f'event:{k}={events[k]}' for k in sorted(events)]
    parts += [f'verb:{v}={verbs[v]}' for v in sorted(verbs)]
    return ' '.join(parts)


def eaten(cell, text):
    """Per calorie class: eats, highest pre-eat satiety, and satiety wasted above the maximum."""
    counts, highest, wasted, wasteful = {}, {}, {}, {}
    for calorie_class, before, after in CONSUMED.findall(text):
        before, after = int(before), int(after)
        waste = before + RESTORATION[calorie_class] - after
        if waste < 0:
            raise AssertionError(
                f'{cell}: {calorie_class} restored more than the food table permits, '
                f'{before}->{after}; this script\'s RESTORATION table has drifted from the engine'
            )
        counts[calorie_class] = counts.get(calorie_class, 0) + 1
        highest[calorie_class] = max(highest.get(calorie_class, 0), before)
        wasted[calorie_class] = wasted.get(calorie_class, 0) + waste
        wasteful[calorie_class] = wasteful.get(calorie_class, 0) + (1 if waste else 0)
    total = sum(counts.values())
    parts = [f'eats={total}']
    for calorie_class in ('low', 'medium', 'high'):
        if calorie_class in counts:
            parts.append(
                f'{calorie_class}:n={counts[calorie_class]},'
                f'maxS={highest[calorie_class]},'
                f'wasted={wasted[calorie_class]}in{wasteful[calorie_class]}'
            )
        else:
            parts.append(f'{calorie_class}:n=0')
    return ' '.join(parts)


def main():
    capture_dir, out_dir, label = sys.argv[1:4]
    cells = sorted(name[:-4] for name in os.listdir(capture_dir) if name.endswith('.txt'))
    summaries = [f'# {label}', '# rule 18 final summary line, one per cell', '']
    censuses = [f'# {label}',
                '# per cell: count of each event kind, then of each verb proposed.',
                '# Verb counts come from action_trace and so appear only in the 60 traced cells.', '']
    eats = [f'# {label}',
            '# per cell and calorie class, from `food_consumed`: n resources eaten, maxS the highest',
            '# pre-eat satiety one was eaten at, wasted the satiety lost above the attribute maximum',
            '# and, after `in`, how many of the n eats lost any.',
            '# maxS is the waste condition read off the stream. Under reference before this change it',
            '# cannot exceed 85/70/50 for low/medium/high; after it, 87/79/75. Under baseline it is',
            '# unbounded at both commits, because rule 4 applies no waste condition at all.', '']
    for cell in cells:
        text = read(os.path.join(capture_dir, cell + '.txt'))
        write(os.path.join(out_dir, 'init', cell + '.txt'), initialization(text))
        summaries.append(f'{cell:34} {summary(text)}')
        censuses.append(f'{cell:34} {census(text)}')
        eats.append(f'{cell:34} {eaten(cell, text)}')
        if cell in FULL:
            write(os.path.join(out_dir, 'full', cell + '.txt'), text)
    write(os.path.join(out_dir, 'summary.txt'), '\n'.join(summaries) + '\n')
    write(os.path.join(out_dir, 'census.txt'), '\n'.join(censuses) + '\n')
    write(os.path.join(out_dir, 'eaten.txt'), '\n'.join(eats) + '\n')
    missing = [cell for cell in FULL if cell not in cells]
    print(f'{label}: retained {len(cells)} initialization blocks, {len(cells)} summary lines, '
          f'{len(cells)} census rows, {len(cells)} eat censuses, and '
          f'{len(FULL) - len(missing)} of {len(FULL)} stated cells whole')
    if missing:
        print('MISSING from the capture: ' + ', '.join(missing))
    return 1 if missing else 0


if __name__ == '__main__':
    sys.exit(main())
