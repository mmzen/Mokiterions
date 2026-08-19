"""WO-MOK-007 oracle 4: the drawn roster, checked cell by cell against rule 4 as amended.

Usage, from the repository root:

    cp docs/engineering/simulation/evidence/WO-MOK-007/observer/frame-probe.rs \
       mokiterions-tui/tests/frame_probe.rs
    cargo test -p mokiterions-tui --test frame_probe
    rm mokiterions-tui/tests/frame_probe.rs
    python docs/engineering/simulation/evidence/WO-MOK-007/analysis/frames.py

`observer/frame-probe.rs` renders the observer into an in-memory backend and writes down two things
per frame: the cells the roster pane holds, and the attribute values the same frame was drawn from.
It asserts nothing. This script supplies the expectation.

The expectation is rebuilt here from `SPEC-MOK-003` rule 4 as amended on 2026-08-19 -- the indent, the
six cells each gauge spends on things that are not bar, the two-cell separators, the truncating
`value * width / 100` -- and not from `render.rs`. That is the whole point of splitting the probe from
the analysis: `BAR_ROW_OVERHEAD` is a constant in the product, and an oracle that read it back would
be comparing the product against itself. Here the number 35 is composed from the parts the rule names,
and if the product's constant and the rule's parts disagree, the reconstruction below fails.

Each entry's bar row is rebuilt in full, character for character, from the snapshot values and the
pane's width, and compared against the cells the backend received. The four gauges' cell columns are
then reported and independently predicted, because a row that is right and mispositioned is still
wrong.
"""

import io
import os
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8')

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.exists(os.path.join(ROOT, 'Cargo.toml')):
    ROOT = os.path.dirname(ROOT)
PROBE = os.path.join(ROOT, 'target', 'probe-out')
OUT = os.path.join(os.path.dirname(HERE), 'observer', 'roster-frames.txt')

# Rule 4 as amended, composed from its parts rather than copied from the product.
INDENT = 5              # the bar row is indented to the width of the identifier column
LABEL = 1               # `h`, `s`, `e` or `f`
GAP_BEFORE_BAR = 1
GAP_AFTER_BAR = 1
VALUE = 3               # the value is right-aligned in three columns
SEPARATOR = 2           # two spaces between one gauge and the next
GAUGES = 4              # `h s e f`, in that order
ORDER = ['h', 's', 'e', 'f']
FULL_BAR = 20           # rule 4's mockup width, and the cap
TWO_LINE_WIDTH = 47     # rule 4's collapse threshold, on the pane's width
FILLED, EMPTY = '█', '░'

OVERHEAD = INDENT + GAUGES * (LABEL + GAP_BEFORE_BAR + GAP_AFTER_BAR + VALUE) + (GAUGES - 1) * SEPARATOR

# The internal tests that carry the provisions no drawn frame can reach, run at the end.
INTERNAL = ['render::tests::the_bar_row_reproduces_the_specified_form',
            'render::tests::a_bar_row_shrinks_to_its_pane_and_never_overflows_it',
            'render::tests::a_zero_value_is_a_zero_and_an_absent_value_is_a_dash']

GAUGE_RUN = re.compile(f'([hsef]) ([{FILLED}{EMPTY}]+) +(\\d+)')


def bar_width(interior):
    """Rule 4's bar length for an interior of this width: four bars, capped at the mockup's twenty."""
    if interior < OVERHEAD:
        return 0
    return min(FULL_BAR, (interior - OVERHEAD) // GAUGES)


def gauge(label, value, width):
    """One gauge as rule 4 specifies it. The division truncates, and `0` is an empty bar."""
    filled = min(width, value * width // 100)
    return f'{label} {FILLED * filled}{EMPTY * (width - filled)} {value:>3}'


def bar_row(values, width):
    """The whole bar row: the indent, then the four gauges in order, separated by two spaces."""
    return ' ' * INDENT + (' ' * SEPARATOR).join(
        gauge(label, values[label], width) for label in ORDER)


def frames(path):
    """The per-viewport records in one capture file."""
    text = io.open(path, encoding='utf-8').read()
    head, *blocks = text.split('-- viewport ')
    for block in blocks:
        lines = block.split('\n')
        viewport = lines[0].strip()
        record = {'viewport': viewport, 'caption': head.split('\n')[0].strip(), 'agents': [],
                  'rows': [], 'roster': None}
        for line in lines[1:]:
            if line.startswith('tier '):
                record['tier'] = line[5:].strip()
            elif line.startswith('agent '):
                fields = line.split()
                values = dict(field.split('=') for field in fields[2:])
                record['agents'].append({
                    'id': fields[1],
                    'h': int(values['health']), 's': int(values['satiety']),
                    'e': int(values['energy']), 'f': int(values['fear'])})
            elif line.startswith('roster x='):
                record['roster'] = {key: int(value) for key, value in
                                    (field.split('=') for field in line.split()[1:])}
            elif line.startswith('row '):
                index, cells = line[4:].split(' |', 1)
                record['rows'].append((int(index), cells[:-1]))
        yield record


def check(record, problems, coverage):
    """Rebuilds every bar row in one frame and compares it against the cells the backend received."""
    rect = record['roster']
    if rect is None:
        return None
    interior = rect['width'] - 2
    bar = bar_width(interior)
    two_line = rect['width'] >= TWO_LINE_WIDTH
    where = f"{record['caption']} {record['viewport']}"

    body = record['rows'][1:-1]                       # the title row and the bottom border are not entries
    for index, cells in record['rows']:
        if len(cells) != rect['width']:
            problems.append(f'{where} row {index} is {len(cells)} cells wide, not {rect["width"]}')
    for index, cells in body:
        if cells[0] != '│' or cells[-1] != '│':
            problems.append(f'{where} row {index} is not bounded by the pane border')

    per_entry = 2 if two_line else 1
    entries = []
    for position in range(0, len(body), per_entry):
        group = body[position:position + per_entry]
        if len(group) < per_entry:
            break
        text = [cells[1:-1] for _, cells in group]
        if not text[0].strip():
            break                                     # past the last entry: the pane is padded blank
        entries.append((group[0][0], text))

    if len(entries) != len(record['agents']):
        problems.append(f'{where} drew {len(entries)} entries for {len(record["agents"])} living '
                        f'Mokiterions')

    positions = []
    for (row_index, text), agent in zip(entries, record['agents']):
        if not text[0].startswith(agent['id']):
            problems.append(f'{where} row {row_index} is {text[0][:5]!r}, expected {agent["id"]}')
        for label in ORDER:
            coverage.setdefault(label, set()).add(agent[label])
        row = text[-1]
        expected = bar_row(agent, bar) if two_line else None
        if two_line:
            if row.rstrip() != expected:
                problems.append(f'{where} {agent["id"]} bar row\n'
                                f'    expected |{expected}|\n'
                                f'    observed |{row.rstrip()}|')
            if len(row) != interior:
                problems.append(f'{where} {agent["id"]} bar row is {len(row)} cells, not {interior}')
        found = GAUGE_RUN.findall(row)
        if [label for label, _, _ in found] != ORDER:
            problems.append(f'{where} {agent["id"]} gauges are '
                            f'{[label for label, _, _ in found]}, not {ORDER}')
        # Cell columns, observed and predicted. The prediction is the pane's own x, its border, the
        # indent, and the fixed stride of one gauge -- nothing read back from the product.
        for order, (label, drawn, printed) in enumerate(found):
            column = rect['x'] + 1 + row.index(f'{label} {drawn} ')
            predicted = rect['x'] + 1 + INDENT + order * (bar + LABEL + GAP_BEFORE_BAR +
                                                          GAP_AFTER_BAR + VALUE + SEPARATOR)
            if column != predicted:
                problems.append(f'{where} {agent["id"]} gauge {label} starts at column {column}, '
                                f'predicted {predicted}')
            if int(printed) != agent[label]:
                problems.append(f'{where} {agent["id"]} gauge {label} prints {printed}, and the '
                                f'frame was drawn from {agent[label]}')
            if drawn.count(FILLED) != min(bar, agent[label] * bar // 100):
                problems.append(f'{where} {agent["id"]} gauge {label} fills '
                                f'{drawn.count(FILLED)} of {bar} at value {agent[label]}')
            if row_index == entries[0][0]:
                positions.append((label, column, column + 2, column + 1 + bar,
                                  column + 3 + bar, column + 5 + bar))

    return {'viewport': record['viewport'], 'tier': record['tier'], 'pane': rect,
            'interior': interior, 'bar': bar, 'two_line': two_line, 'entries': len(entries),
            'rows': len(entries) * per_entry, 'positions': positions}


def verbatim(record):
    return [f'  {index:>3} |{cells}|' for index, cells in record['rows']]


def main():
    if not os.path.isdir(PROBE):
        raise SystemExit(f'no probe output at {PROBE}; run observer/frame-probe.rs first')

    problems = []
    coverage = {}
    declared = []
    captures = ['declared-seed42-tick0.txt', 'declared-seed42-tick200.txt',
                'declared-seed42-tick1000.txt']
    verbatim_frames = []
    for name in captures:
        for record in frames(os.path.join(PROBE, name)):
            summary = check(record, problems, coverage)
            declared.append((record, summary))
            if record['viewport'] == '160x48':
                verbatim_frames.append(record)

    sweep = []
    for record in frames(os.path.join(PROBE, 'sweep-height48-seed42-tick200.txt')):
        summary = check(record, problems, coverage)
        sweep.append((record, summary))

    drawn = [summary for _, summary in sweep if summary]
    pane_widths = sorted({summary['pane']['width'] for summary in drawn})
    bars = sorted({summary['bar'] for summary in drawn})
    forms = sorted({'two-line' if summary['two_line'] else 'one-line' for summary in drawn})
    absent = [record['viewport'] for record, summary in sweep if summary is None]

    lines = [
        'VER-MOK-007 oracle 4 - the drawn roster, cell by cell',
        '=====================================================',
        '',
        'Why the probe and the expectation are separate programs, and what is rebuilt rather than',
        'read back: see the header of analysis/frames.py. The probe is retained beside this file as',
        'observer/frame-probe.rs; it renders through `render::draw` into ratatui\'s in-memory backend',
        'and records the cells, the pane the layout gave the roster, and the attribute values the',
        'frame was drawn from. Every expectation below is composed here from rule 4 as amended.',
        '',
        'The geometry, composed from the parts rule 4 names',
        '',
        f'  indent, to the width of the identifier column          {INDENT:>3}',
        f'  per gauge: label, space, space, three-column value      {LABEL + GAP_BEFORE_BAR + GAP_AFTER_BAR + VALUE:>3}  x {GAUGES} = '
        f'{GAUGES * (LABEL + GAP_BEFORE_BAR + GAP_AFTER_BAR + VALUE):>3}',
        f'  separators between the four gauges                      {SEPARATOR:>3}  x {GAUGES - 1} = '
        f'{(GAUGES - 1) * SEPARATOR:>3}',
        f'  {"-" * 55}',
        f'  cells a bar row needs that are not bar                       {OVERHEAD:>3}',
        '',
        f'  bar(interior)        = min({FULL_BAR}, (interior - {OVERHEAD}) / {GAUGES}), and 0 below {OVERHEAD}',
        '  filled(value, bar)   = value * bar / 100, truncating, capped at bar',
        f'  entry form           = two lines at a pane width of {TWO_LINE_WIDTH} or more, one line below',
        '',
        f'  The product spends the same 35 cells, as `BAR_ROW_OVERHEAD`. It is not read from there:',
        '  the sum above is this script\'s, and a disagreement between it and the product appears',
        '  below as a failed reconstruction rather than as agreement by construction.',
        '',
        'The declared viewports',
        '',
    ]
    # The column widths are taken from the data, so a longer tier label cannot silently push a row
    # out of its column and make the table read as something other than what was measured.
    caption_width = max(len(record['caption']) for record, _ in declared)
    tier_width = max(len(record['tier']) for record, _ in declared)
    lines += [
        f'  {"capture":{caption_width}}  {"viewport":9}  {"tier":{tier_width}}  {"pane":9}  {"int":>3}  '
        f'{"bar":>3}  {"form":8}  {"entries":>7}  {"rows":>4}',
        f'  {"-" * caption_width}  {"-" * 9}  {"-" * tier_width}  {"-" * 9}  ---  ---  {"-" * 8}  '
        f'-------  ----',
    ]
    for record, summary in declared:
        caption = record['caption']
        if summary is None:
            lines.append(f'  {caption:{caption_width}}  {record["viewport"]:9}  '
                         f'{record["tier"]:{tier_width}}  {"none":9}')
            continue
        pane = f'{summary["pane"]["width"]}x{summary["pane"]["height"]}'
        lines.append(f'  {caption:{caption_width}}  {summary["viewport"]:9}  '
                     f'{summary["tier"]:{tier_width}}  {pane:9}  '
                     f'{summary["interior"]:3}  {summary["bar"]:3}  '
                     f'{"two-line" if summary["two_line"] else "one-line":8}  '
                     f'{summary["entries"]:7}  {summary["rows"]:4}')

    first = declared[0][0]['caption']
    without = [record['viewport'] for record, summary in declared
               if record['caption'] == first and not summary]
    reconstructed = sum(summary['entries'] for _, summary in declared if summary)
    reconstructed += sum(summary['entries'] for _, summary in sweep if summary)
    lines += [
        '',
        f'  Bar rows rebuilt character for character and compared: {reconstructed}',
        f'  Discrepancies: {len(problems)}',
        '',
        f'  {len(without)} of the {len(without) + len(set(summary["viewport"] for _, summary in declared if summary))} '
        f'declared viewports carry no roster -- {", ".join(without)} -- and those rows are',
        '  not evidence about one. 100x30 and 34x22 are tier D, where rule 5 drops the roster and the',
        '  inspector to keep the world view; 100x30 reaches tier D on its height, not its width, since',
        '  rule 5\'s tier C row asks for 38 rows. 33x21 is below the floor, where rule 6 replaces the',
        '  whole frame with the size notice. `WO-MOK-005` and `WO-MOK-006` verify all three, and this',
        '  work order changes none of them.',
        '',
        'Where the four gauges sit, observed and predicted',
        '',
        '  Absolute frame columns of the first entry\'s bar row, 0-based, at each declared viewport',
        '  that draws a roster. The prediction is the pane\'s own x, its border, the indent and the',
        '  stride of one gauge; it is not read back from the frame.',
        '',
        f'  {"viewport":9}  {"gauge":5}  {"label":>5}  {"bar":>7}  {"value":>7}',
        f'  {"-" * 9}  {"-" * 5}  -----  -------  -------',
    ]
    seen = set()
    for record, summary in declared:
        if not summary or not summary['positions'] or summary['viewport'] in seen:
            continue
        seen.add(summary['viewport'])
        for label, label_column, bar_start, bar_end, value_start, value_end in summary['positions']:
            lines.append(f'  {summary["viewport"]:9}  {label:5}  {label_column:5}  '
                         f'{f"{bar_start}-{bar_end}":>7}  {f"{value_start}-{value_end}":>7}')

    lines += [
        '',
        'Two frames verbatim',
        '',
        '  The cells the backend received, at the widest declared viewport. Row indices are the',
        '  frame\'s own; the pipes are this record\'s and fix the pane\'s first and last column.',
        '',
    ]
    for record in verbatim_frames[:2]:
        lines += [f'  {record["caption"]}, viewport {record["viewport"]}, tier {record["tier"]}', '']
        lines += verbatim(record)
        lines += ['']
    lines += [
        '  Read the two together for rule 4.4: at tick 0 every `f` gauge is `' + EMPTY * 2 + '   0`, an empty',
        '  bar and a printed zero, and at tick 200 the same column carries `' + FILLED * 2 + ' 100` for the',
        '  Mokiterions at the ceiling. The gauge is not a placeholder in either frame.',
        '',
        'Value coverage',
        '',
        f'  {"attribute":10}  {"distinct":>8}  {"0":>5}  {"100":>5}  values',
        f'  {"-" * 10}  {"-" * 8}  -----  -----  ------',
    ]
    names = {'h': 'health', 's': 'satiety', 'e': 'energy', 'f': 'fear'}
    for label in ORDER:
        values = sorted(coverage.get(label, set()))
        shown = ', '.join(str(value) for value in values)
        if len(shown) > 60:
            shown = f'{values[0]} .. {values[-1]}'
        lines.append(f'  {names[label]:10}  {len(values):8}  {str(0 in values):>5}  '
                     f'{str(100 in values):>5}  {shown}')

    lines += [
        '',
        '  Both bounds of `fear` are drawn: the empty bar and the full one. The intermediate values',
        '  are whatever the run produced, not a chosen set -- these are frames of a real run, and the',
        '  arithmetic is checked at every value that appeared rather than at values picked to suit it.',
        '',
        'What no drawn frame can reach, and who carries it instead',
        '',
        f'  Every width from the floor to 160 was swept at height 48. Where a roster is drawn its pane',
        f'  is {", ".join(str(width) for width in pane_widths)} columns wide, always, because rule 5 gives the pane a fixed width. So:',
        '',
        f'    bar widths observed across the whole sweep:   {", ".join(str(bar) for bar in bars)}',
        f'    entry forms observed:                         {", ".join(forms)}',
        f'    widths that draw no roster at all:            {len(absent)} '
        f'({absent[0].split("x")[0]}..{absent[-1].split("x")[0]} columns, tier D)',
        '',
        f'  The pane\'s fixed width is {pane_widths[0]} and rule 4\'s collapse threshold is {TWO_LINE_WIDTH}. They are equal, so',
        '  the drawn roster takes the two-line form at every viewport, exactly at the boundary, and',
        '  the one-line form is unreachable through `render::draw`. The interior is 45, so',
        f'  bar = ({45} - {OVERHEAD}) / {GAUGES} = {bar_width(45)} and the {FULL_BAR}-cell cap is unreachable too. This is a fact',
        '  about the layout, not a gap in the sweep, and it is worth stating plainly: the amendment\'s',
        '  bar-width arithmetic is exercised in a drawn frame at exactly one width.',
        '',
        '  The provisions the frames cannot reach are carried by the render module\'s own tests, which',
        '  reach `entry_lines` and `bar_width` directly -- they are internal-tier for that reason, and',
        '  `test-census.txt` accounts for them:',
        '',
    ]

    ok = not problems
    for name in INTERNAL:
        finished = subprocess.run(
            ['cargo', 'test', '-p', 'mokiterions-tui', '--lib', name, '--', '--exact'],
            cwd=ROOT, capture_output=True, text=True, encoding='utf-8', errors='replace')
        output = finished.stdout + finished.stderr
        result = next((line.strip() for line in output.split('\n')
                       if line.strip().startswith('test result:')), '(no result line)')
        ok = ok and finished.returncode == 0 and result.startswith('test result: ok.')
        lines += [f'  $ cargo test -p mokiterions-tui --lib {name.split("::")[-1]} -- --exact',
                  f'    {result}', '']

    lines += [
        '  The first pins rule 4\'s mockup line byte for byte at the twenty-cell width, including that',
        '  the row ends at the fourth gauge with no fifth group and no trailing padding. The second',
        '  checks the bar width at every interior from 1 to 199, including that a pane too narrow for',
        '  a bar asks for none rather than for a negative width. The third is rule 4.4 at zero.',
        '',
    ]

    if problems:
        lines += ['Discrepancies', '']
        lines += [f'  {problem}' for problem in problems[:40]]
        if len(problems) > 40:
            lines.append(f'  ... and {len(problems) - 40} more')
        lines += ['']

    lines += ['RESULT: ' + (
        f'PASS - {reconstructed} bar rows rebuilt from rule 4 and the snapshot values match the\n'
        '        cells the backend received, character for character, and every gauge sits in the\n'
        '        column the rule puts it in'
        if ok else 'FAIL')]

    io.open(OUT, 'w', encoding='utf-8', newline='\n').write(
        '\n'.join(line.rstrip() for line in lines) + '\n')
    print('\n'.join(lines))
    print(f'written to: {OUT}')
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
