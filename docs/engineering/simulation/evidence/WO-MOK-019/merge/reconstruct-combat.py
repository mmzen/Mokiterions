"""VER-MOK-012 oracle 2, extended for rule 26's vocabulary: the text stream rebuilt from the record
stream where the run carries combat.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-019/merge/reconstruct-combat.py \
        <capture-dir> <output-file>

## Why this file exists at all

`analysis/reconstruct.py` is the retained oracle and it is **not modified**: it is imported here by
path and its `reconstruct`, `event_line` and `no_event_specific_branch` are the ones that run. Two of
its functions are rebound before the walk, and the two replacements are below in full. The retained
file stays byte-identical because `VER-MOK-012`'s record and its digests are bound to it; what the
merge needs is not a different oracle but two more of rule 6.5's shapes, and a reader must be able to
see exactly which two.

The retained oracle FAILS on this branch's merge, and its failure is the measurement that motivated
this file. Run over the ninety cells of the reference matrix it reports fifteen of the ninety
differing -- every `--trace-actions` cell and no other -- and over the thirty `social` cells fifteen
of thirty, again exactly the traced ones. The first differing line of each names its cause:

    reconstructed  ...result=proposal:approach:M06,status:accepted,...,fear:0,suffered:[]
    written        ...result=proposal:approach,target:M06,status:accepted,...,fear:0

## The two shapes rule 6.5 does not yet have

1. **A proposal whose target is a sibling field of the text line.** The record nests the target inside
   the proposal, `{"action":"approach","target":"M06"}`, on the reasoning that a verb and its target
   read from one `Action` in one arm cannot disagree. The text line renders the verb alone and then
   `target:M06` as its own comma field, because a parser that reads the line positionally must not
   have to know which verbs carry a target. So one record field becomes two text fields, and that is
   a pairing rule rather than a scalar rule: it belongs in `render_result` and not in `render_value`.

   The retained walk already renders `{"action":"eat","food":"F0016"}` as `eat:F0016` and
   `{"action":"move","direction":"east"}` as `move:east`, which the text confirms. `target` is
   therefore the one second key that does *not* join to its verb, and the dispatch below keys on that
   name. **That is a departure from rule 6.6's stated design and is reported as one.** The retained
   oracle's own docstring says a reconstructor needing a further case "would itself be evidence that
   the record shapes had drifted from the rule", and by that standard this case is evidence. The
   `merge/README.md` puts the alternative to the owner: writing `target` as a sibling field of
   `proposal` in the *record* as well, which is what the text does, costs nothing and needs no new
   shape at all. This file measures the merge as it currently stands; it does not prefer it.

2. **An array of objects.** `suffered` is rule 25's window and the record always carries it, empty or
   not, under rule 4.4. The text omits the clause entirely when the array is empty and otherwise
   renders it as `suffered:M12:29`, several entries joined by `;`. Both halves are shape-driven and
   need no field name: an empty array is a field the text does not write, and a non-empty one is its
   elements joined by `;`, each element being its own values joined by `:` -- which is the same rule
   the coordinate already follows. Rule 6.5 has no array shape because until this merge no record
   carried an array inside a `result`.

`no_event_specific_branch` still runs, and it still runs over the reconstruction that is actually
used: it reads `render_value`, `render_result` and `event_line` out of the retained module's
namespace, and the first two are the ones defined here by the time it looks. Neither mentions an
event name. The `target` dispatch is on a *field* name, which is what rule 6.6's walk is made of --
`from`, `to`, `x`, `y` and `action` are all field names in the retained file.

Neither function reads the engine's source. The comparison is against the engine's output.
"""

import importlib.util
import io
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RETAINED = os.path.join(HERE, os.pardir, 'analysis', 'reconstruct.py')


def load_retained():
    """The retained oracle, imported from its own file and left unmodified on disk."""
    specification = importlib.util.spec_from_file_location('reconstruct_retained', RETAINED)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def render_value(value):
    """One result value, rendered as the text stream renders it. Rules 6.5 and 6.6, plus the array.

    Everything above the array case is the retained function verbatim, so that a reader compares two
    functions and not two files. Dispatch is on the value's shape alone.
    """
    if isinstance(value, list):
        # Rule 25's window. Each element is its own values joined by `:`, which is the coordinate's
        # rule applied to a second object, and the elements are joined by `;`. An empty array does
        # not reach here: `render_result` drops the field, because the text line has no clause for it.
        return ';'.join(':'.join(str(item) for item in element.values()) for element in value)
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
                # `target` is handled by `render_result`, which needs two text fields for it. Any
                # other second key joins to the verb, which is what `eat` and `move` do.
                if keys[1] == 'target':
                    return str(value['action'])
                return f'{value["action"]}:{value[keys[1]]}'
        raise ValueError(f'not one of rule 6.5\'s composite shapes: {value!r}')
    if isinstance(value, bool):
        # Rule 4.5 admits exactly two booleans and neither is a result field. A boolean here would
        # mean the record states something the text does not.
        raise ValueError(f'a result value is a boolean: {value!r}')
    return str(value)


def render_result(result):
    """The text `result=` field, from an event record's `result` object. Rule 6.6's walk, entire.

    Two fields of the record do not map one-to-one onto a text field, and both are pairing rules:

    * a proposal carrying a `target` becomes two text fields, the verb and then the target;
    * an empty array becomes no text field at all.
    """
    fields = []
    for key, value in result.items():
        if isinstance(value, dict) and list(value)[:1] == ['action'] and 'target' in value:
            fields.append(f'{key}:{value["action"]}')
            fields.append(f'target:{value["target"]}')
            continue
        if isinstance(value, list) and not value:
            continue
        fields.append(f'{key}:{render_value(value)}')
    return ','.join(fields)


def main():
    capture_dir, output_file = sys.argv[1:3]
    retained = load_retained()
    retained.render_value = render_value
    retained.render_result = render_result

    cells = sorted(name[:-6] for name in os.listdir(capture_dir) if name.endswith('.jsonl'))
    if not cells:
        print(f'no record stream in {capture_dir}', file=sys.stderr)
        return 1

    results = []
    failures = 0
    event_names = set()
    for cell in cells:
        rebuilt = retained.reconstruct(os.path.join(capture_dir, cell + '.jsonl'), event_names)
        with io.open(os.path.join(capture_dir, cell + '.txt'), 'rb') as handle:
            actual = handle.read()

        if rebuilt == actual:
            results.append((cell, 'identical', actual.count(b'\n'), ''))
            continue

        failures += 1
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

    special_cased = retained.no_event_specific_branch(event_names)

    total = sum(count for _, _, count, _ in results)
    lines = [
        '# VER-MOK-012 oracle 2, extended for rule 26: the text stream reconstructed from the records',
        '#',
        f'# capture directory: {capture_dir}',
        '# command: python docs/engineering/simulation/evidence/WO-MOK-019/merge/'
        'reconstruct-combat.py <capture-dir> <output-file>',
        '#',
        '# The retained analysis/reconstruct.py is imported unmodified and two of its functions are',
        '# rebound: render_value gains rule 25\'s array and render_result gains the two pairing rules',
        '# a proposal\'s target and an empty array need. Both replacements are reproduced in full at',
        '# the foot of this file, ahead of the retained oracle they replace them in.',
        '#',
        '# Every text line of every cell is rebuilt from its record and compared byte for byte with',
        '# the standard output the same process wrote. No projection, no normalization, no sampling.',
        '',
    ]
    for cell, verdict, count, detail in results:
        row = f'{cell:<34} {verdict:<9} {count:>7} lines'
        if detail:
            row += f'  {detail}'
        lines.append(row)

    lines.append('')
    lines.append(f'# {len(cells)} cells, {total} text lines reconstructed, {failures} cells differing')
    lines.append('')
    lines.append(
        '# No event-type-specific branch. The check is the retained oracle\'s own, run over the'
    )
    lines.append(
        '# reconstruction actually used: it reads render_value, render_result and event_line out of'
    )
    lines.append(
        f'# the retained module, where the first two are this file\'s. All'
        f' {len(event_names)} event names the'
    )
    lines.append('# captured streams carry were searched for:')
    for name in sorted(event_names):
        lines.append(f'#   {name}')
    if special_cased:
        lines.append(f'# BRANCHES ON: {special_cased}')
    else:
        lines.append('# none of them appears in the reconstruction.')
    lines.append('')
    lines.append(f'# result: {"FAIL" if failures or special_cased else "PASS"}')
    lines.append('')
    lines.append('# ---- full text of the two rebound functions and of this driver ----')
    lines.append('')
    lines.extend(io.open(os.path.abspath(__file__), 'r', encoding='utf-8').read().split('\n'))
    lines.append('')
    lines.append('# ---- full text of the retained oracle, imported unmodified ----')
    lines.append('')
    lines.extend(io.open(RETAINED, 'r', encoding='utf-8').read().split('\n'))

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(
        f'{len(cells)} cells, {total} text lines reconstructed, {failures} differing; '
        f'{len(event_names)} event names, {len(special_cased)} branched on'
    )
    return 1 if failures or special_cased else 0


if __name__ == '__main__':
    sys.exit(main())
