"""VER-MOK-012 oracle 2: rebuild the text stream from the record stream and compare bytes.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-019/analysis/reconstruct.py \
        <capture-dir> <output-file>

For every cell of a sink-mode capture directory, this reads `<cell>.jsonl`, reconstructs what the
text stream must have been, and compares it byte for byte with `<cell>.txt` -- the standard output the
same process wrote in the same invocation. Zero differing bytes is the pass condition.

`REQ-MOK-042` says the structured stream carries the same facts as the text stream. That is an
**invertibility** claim, and an invertibility claim needs no oracle: it is checked by inverting the
writer and comparing against a stream the writer did not produce. This script cannot be fooled by
asserting what the new code does, because it makes no assertion about the code. It consumes the
writer's output and is compared against a different output of the same run.

## Where every rule in this script comes from

`render_result` is written from `SPEC-MOK-006` rule 6.6 and from nothing else, and **contains no
event-type-specific branch**: it dispatches on the *shape* of a value, never on the event's name. Rule
6.6 exists so that three composite shapes suffice -- a coordinate, a before-and-after pair, and a
proposed action -- and a reconstructor that needed a twelfth case would itself be evidence that the
record shapes had drifted from the rule.

`VER-MOK-012` requires that property of this file, so `no_event_specific_branch` checks it mechanically
rather than leaving it a claim in a comment. The check takes the event names it looks for from the
captured streams and not from a list written here, so it cannot go stale: whatever the engine emitted is
what is searched for. It reads the source of the three reconstruction functions alone, which is why the
names may appear in `main` -- the check is a statement about the reconstructor, not about the file.

`event_line` is the text line's framing, which is `SPEC-MOK-001`'s and not rule 6.6's: four
space-separated `key=value` fields, no padding. `summary_line` is `SPEC-MOK-006` rule 8.3's, which
fixes the twelve figures the run record must carry so that the text summary line is reconstructible
too. Both are stated once, here, so that a reader can see the entire text format this script assumes.

Neither reads the engine's source. The comparison is against the engine's *output*.
"""

import inspect
import io
import json
import os
import sys


def render_value(value):
    """One result value, rendered as the text stream renders it. SPEC-MOK-006 rules 6.5 and 6.6.

    Dispatch is on the value's shape alone. An object's key set decides which of rule 6.5's three
    composite shapes it is; anything else is the scalar the text carries.
    """
    if isinstance(value, dict):
        keys = list(value)
        if keys == ['x', 'y']:
            return f'{value["x"]}:{value["y"]}'
        if keys == ['from', 'to']:
            return f'{value["from"]}->{value["to"]}'
        if keys[0] == 'action':
            if len(keys) == 1:
                return str(value['action'])
            if len(keys) == 2:
                return f'{value["action"]}:{value[keys[1]]}'
        raise ValueError(f'not one of rule 6.5\'s three composite shapes: {value!r}')
    if isinstance(value, bool):
        # No result field is a boolean: rule 6.5 requires the action trace's verdict to reach the
        # record as the string `accepted` or `rejected`, because that is what the text carries. A
        # boolean here would mean the record states something the text does not.
        raise ValueError(f'a result value is a boolean: {value!r}')
    return str(value)


def render_result(result):
    """The text `result=` field, from an event record's `result` object. Rule 6.6's walk, entire."""
    return ','.join(f'{key}:{render_value(value)}' for key, value in result.items())


def event_line(record):
    """One text event line. The framing is SPEC-MOK-001's; the result is rule 6.6's."""
    return (
        f'tick={record["tick"]} '
        f'subject={record["subject"]} '
        f'event={record["event"]} '
        f'result={render_result(record["result"])}'
    )


def summary_line(run):
    """The text summary line, from the run record. SPEC-MOK-006 rule 8.3's twelve figures."""
    territories = run['final']['territories']
    a, b = territories['A'], territories['B']
    return (
        f'summary reason={run["reason"]}'
        f' ticks={run["ticks"]}'
        f' survivors={run["survivors"]}'
        f' deaths={run["deaths"]}'
        f' territory_a={a["population"]}'
        f' territory_b={b["population"]}'
        f' food_a_low={a["low"]}'
        f' food_a_medium={a["medium"]}'
        f' food_a_high={a["high"]}'
        f' food_b_low={b["low"]}'
        f' food_b_medium={b["medium"]}'
        f' food_b_high={b["high"]}'
    )


def no_event_specific_branch(event_names):
    """VER-MOK-012's requirement on this file, checked instead of claimed.

    The reconstruction is the three functions above. If any of them mentions an event name that the
    captured streams carry, rule 6.6's generic walk was not sufficient and the reconstructor was
    special-cased to compensate -- which is exactly the drift the requirement exists to detect.
    """
    reconstruction = ''.join(
        inspect.getsource(function) for function in (render_value, render_result, event_line)
    )
    return sorted(name for name in event_names if name in reconstruction)


def reconstruct(stream_path, event_names):
    """The whole text stream a record stream implies, as bytes."""
    lines = []
    with io.open(stream_path, 'r', encoding='utf-8', newline='') as handle:
        for raw in handle:
            raw = raw.rstrip('\n')
            if not raw:
                continue
            record = json.loads(raw)
            kind = record['record']
            if kind == 'event':
                event_names.add(record['event'])
                lines.append(event_line(record))
            elif kind == 'run':
                lines.append(summary_line(record))
            # `header` and `metrics` have no text counterpart. The header states the resolved
            # configuration, which the text stream never printed, and the metrics record is the one
            # thing this change adds that the text stream does not carry -- SPEC-MOK-006 rule 7.8.
    return ('\n'.join(lines) + '\n').encode('utf-8')


def main():
    capture_dir, output_file = sys.argv[1:3]
    cells = sorted(
        name[:-6] for name in os.listdir(capture_dir) if name.endswith('.jsonl')
    )
    if not cells:
        print(f'no record stream in {capture_dir}', file=sys.stderr)
        return 1

    results = []
    failures = 0
    event_names = set()
    for cell in cells:
        rebuilt = reconstruct(os.path.join(capture_dir, cell + '.jsonl'), event_names)
        with io.open(os.path.join(capture_dir, cell + '.txt'), 'rb') as handle:
            actual = handle.read()

        if rebuilt == actual:
            results.append((cell, 'identical', actual.count(b'\n'), ''))
            continue

        failures += 1
        # A failure names the line, not the run: VER-MOK-012 requires the property asserted line by
        # line rather than by a digest, so that a reviewer is told where.
        rebuilt_lines = rebuilt.split(b'\n')
        actual_lines = actual.split(b'\n')
        detail = f'{len(rebuilt_lines)} reconstructed lines against {len(actual_lines)} written'
        for number, (left, right) in enumerate(zip(rebuilt_lines, actual_lines), start=1):
            if left != right:
                detail = (
                    f'first difference at line {number}: '
                    f'reconstructed {left!r} against written {right!r}'
                )
                break
        results.append((cell, 'DIFFERS', actual.count(b'\n'), detail))

    source = io.open(__file__, 'r', encoding='utf-8').read()

    lines = [
        '# VER-MOK-012 oracle 2: the text stream reconstructed from the record stream',
        '#',
        f'# capture directory: {capture_dir}',
        '# command: python docs/engineering/simulation/evidence/WO-MOK-019/analysis/reconstruct.py '
        '<capture-dir> <output-file>',
        '#',
        '# Every text line of every cell is rebuilt from its record by SPEC-MOK-006 rule 6.6\'s walk',
        '# and compared byte for byte with the standard output the same process wrote. There is no',
        '# projection, no normalization and no sampling: the comparison is over whole streams.',
        '',
    ]
    for cell, verdict, count, detail in results:
        row = f'{cell:<34} {verdict:<9} {count:>7} lines'
        if detail:
            row += f'  {detail}'
        lines.append(row)

    special_cased = no_event_specific_branch(event_names)

    total = sum(count for _, _, count, _ in results)
    lines.append('')
    lines.append(f'# {len(cells)} cells, {total} text lines reconstructed, {failures} cells differing')
    lines.append('')
    lines.append(
        '# No event-type-specific branch. The source of render_value, render_result and event_line was'
    )
    lines.append(
        f'# searched for each of the {len(event_names)} event names the captured streams carry:'
    )
    for name in sorted(event_names):
        lines.append(f'#   {name}')
    if special_cased:
        lines.append(f'# BRANCHES ON: {special_cased}')
    else:
        lines.append('# none of them appears in the reconstruction. Rule 6.6\'s generic walk sufficed.')
    lines.append('')
    lines.append(f'# result: {"FAIL" if failures or special_cased else "PASS"}')
    lines.append('')
    lines.append('# ---- full text of the reconstructor, retained as VER-MOK-012 requires ----')
    lines.append('')
    lines.extend(source.split('\n'))

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(
        f'{len(cells)} cells, {total} text lines reconstructed, {failures} differing; '
        f'{len(event_names)} event names, {len(special_cased)} branched on'
    )
    return 1 if failures or special_cased else 0


if __name__ == '__main__':
    sys.exit(main())
