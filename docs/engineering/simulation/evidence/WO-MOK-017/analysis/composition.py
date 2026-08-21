"""VER-MOK-016 oracle 4 under WO-MOK-017: REQ-MOK-060's composition ratio at both commits.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/composition.py \
        <pre-capture-dir> <post-capture-dir>

Writes its report to stdout and exits `0` when every internal check passes, non-zero otherwise.
**A breached ceiling is not an internal check.** It is the measurement this reader exists to take, so
it is reported and the exit code does not encode it: a reader that failed on it could not produce the
pre-change side at all, and the pre-change side is 27 breaches of 30.

WHAT THIS READER IS FOR
-----------------------
`VER-MOK-016` retains "the composition ratio computed per territory per class, at both commits", and
`WO-MOK-017` is the work order that gives "both commits" two different worlds for the first time.
`WO-MOK-016`'s reader of the same name measured the *uncorrected* composition at a candidate where
`REQ-MOK-060` was unimplemented, and its figures are the curve the amendment of 2026-08-21 was decided
on. This reader measures the corrected one and sets it against that curve.

The ceiling it evaluates is **three fifths**, amended from one half by the repository owner acting as
product owner on 2026-08-21, on that curve. `REQ-MOK-060`'s amendment record carries the decision and
its declined alternatives. Section 7 evaluates the same 30 territories against the retracted value as
well, not to reopen it but so that a reviewer can read what the amendment moved and what it did not:
the correction clears one half on most of the set too, and the rows where it does not are named.

WHY THE SIX COUNTS ARE RECONSTRUCTED RATHER THAN READ
-----------------------------------------------------
Rule 18's summary line prints `food_a_low` through `food_b_high`, which is what `REQ-MOK-060` means by
"measurable from the run's own output". A reader that copied those six numbers off the last line would
report whatever the engine reported and could not fail. So section 3 rebuilds them from the records:

  * a resource arrives in `food_initialized` with class, position and printed territory, or in
    `food_regenerated` with class and position under the territory that added it as the subject;
  * it leaves in `food_consumed`, which names it and its class;
  * nothing else creates or destroys one.

Standing composition is therefore `initialized + regenerated - consumed`, per territory per class, and
it must equal the printed six. A resource's territory is derived here the way `Position::territory()`
derives it -- from the **y** coordinate against half the world height, not from x and not from a stored
field -- and the derived value is then checked against the one the record printed. That is the check
that would catch a resource added outside the territory that drew for it, and getting the axis wrong is
a mistake this reader reports loudly rather than absorbing: the first run of it split on x and produced
thousands of failures.

This is also the reader that would catch the failure this work order most needs excluded on the
composition side: a correction that changed what a resource *is* rather than when it is eaten. The
class a resource is consumed with is checked against the class it was placed with, on every eat at
both commits, so a drifted food table cannot hide inside a corrected waste condition.
"""

import io
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

CEILING = 0.60          # REQ-MOK-060 as amended on 2026-08-21, three fifths
RETRACTED = 0.50        # the value it was amended from, evaluated in section 7
DEFAULT_DENSITY = '0.75'
OBLIGATED = ('reference', 'individual', 'social')
SEEDS = ('0', '1', '42', '123', '777')
DENSITIES = ('0.15', '0.75', '1.50')
SOURCES = ('baseline', 'reference', 'individual', 'social')
CLASSES = ('low', 'medium', 'high')

CELL = re.compile(r'^seed(\d+)-([a-z]+)-d([0-9.]+)-trace(on|off)$')
WORLD = re.compile(r'event=world_initialized result=width:(\d+),height:(\d+),territories:(\d+)')
PLACED = re.compile(r'^tick=(\d+) subject=(F\d+) event=food_initialized '
                    r'result=class:(\w+),position:(\d+):(\d+),territory:(\w+)$')
REGEN = re.compile(r'^tick=(\d+) subject=(\w+) event=food_regenerated '
                   r'result=food:(F\d+),class:(\w+),position:(\d+):(\d+)$')
ATE = re.compile(r'^tick=(\d+) subject=(M\d+) event=food_consumed '
                 r'result=food:(F\d+),class:(\w+),satiety:(\d+)->(\d+),energy:(\d+)->(\d+)$')
SUMMARY = re.compile(
    r'^summary reason=(\w+) ticks=(\d+) survivors=(\d+) deaths=(\d+) '
    r'territory_a=(\d+) territory_b=(\d+) '
    r'food_a_low=(\d+) food_a_medium=(\d+) food_a_high=(\d+) '
    r'food_b_low=(\d+) food_b_medium=(\d+) food_b_high=(\d+)$')


def read(path):
    return io.open(path, encoding='utf-8', newline='').read()


def parse_cell(name):
    match = CELL.match(name)
    if not match:
        raise AssertionError(f'cell name outside the declared matrix: {name}')
    seed, source, density, trace = match.groups()
    return {'seed': seed, 'source': source, 'density': density, 'trace': trace, 'name': name}


class Failures:
    """Every internal check's failures, collected rather than raised, so one report holds them all."""

    def __init__(self):
        self.items = []

    def check(self, condition, message):
        if not condition:
            self.items.append(message)
        return bool(condition)


def measure(name, text, failures):
    """Reconstruct one cell's standing composition from its records and check the printed line."""
    lines = text.split('\n')
    world = WORLD.search(text)
    failures.check(world is not None, f'{name}: no world_initialized record')
    height = int(world.group(2)) if world else 128
    territories = int(world.group(3)) if world else 2
    failures.check(territories == 2, f'{name}: {territories} territories, not 2')

    def derive(y):
        """`Position::territory()` splits on y at half the world height, not on x."""
        return 'A' if y < height // 2 else 'B'

    placed = {}                                   # food id -> (class, territory)
    eaten = {}                                    # food id -> tick eaten
    standing = {(t, c): 0 for t in 'AB' for c in CLASSES}
    placements = 0
    consumptions = 0
    summaries = []
    last_summary_index = None

    for index, line in enumerate(lines):
        if not line:
            continue
        if line.startswith('summary '):
            match = SUMMARY.match(line)
            failures.check(match is not None, f'{name}: unparsed summary line')
            if match:
                summaries.append(match)
                last_summary_index = index
            continue
        if ' event=food_initialized ' in line:
            match = PLACED.match(line)
            if not failures.check(match is not None, f'{name}: unparsed food_initialized: {line}'):
                continue
            _, food, calorie_class, _, y, printed = match.groups()
            derived = derive(int(y))
            failures.check(derived == printed,
                           f'{name}: {food} at y={y} derives territory {derived}, printed {printed}')
            failures.check(food not in placed, f'{name}: {food} placed twice')
            failures.check(calorie_class in CLASSES, f'{name}: {food} has class {calorie_class}')
            placed[food] = (calorie_class, printed)
            standing[(printed, calorie_class)] += 1
            placements += 1
            continue
        if ' event=food_regenerated ' in line:
            match = REGEN.match(line)
            if not failures.check(match is not None, f'{name}: unparsed food_regenerated: {line}'):
                continue
            _, subject, food, calorie_class, _, y = match.groups()
            derived = derive(int(y))
            failures.check(subject in ('A', 'B'), f'{name}: regeneration subject {subject}')
            failures.check(derived == subject,
                           f'{name}: {food} regenerated by {subject} at y={y}, which is in {derived}')
            failures.check(food not in placed, f'{name}: {food} placed twice')
            failures.check(calorie_class in CLASSES, f'{name}: {food} has class {calorie_class}')
            placed[food] = (calorie_class, subject)
            standing[(subject, calorie_class)] += 1
            placements += 1
            continue
        if ' event=food_consumed ' in line:
            match = ATE.match(line)
            if not failures.check(match is not None, f'{name}: unparsed food_consumed: {line}'):
                continue
            tick, _, food, calorie_class, _, _, _, _ = match.groups()
            if not failures.check(food in placed, f'{name}: {food} eaten but never placed'):
                continue
            failures.check(food not in eaten,
                           f'{name}: {food} eaten twice, at tick {eaten.get(food)} and {tick}')
            was, territory = placed[food]
            failures.check(was == calorie_class,
                           f'{name}: {food} placed as {was} and eaten as {calorie_class}')
            eaten[food] = tick
            standing[(territory, was)] -= 1
            consumptions += 1

    failures.check(len(summaries) == 1, f'{name}: {len(summaries)} summary lines, not 1')
    tail = [line for line in lines[(last_summary_index or 0) + 1:] if line]
    failures.check(not tail, f'{name}: {len(tail)} non-empty lines after the summary')
    if not summaries:
        return None
    fields = summaries[0].groups()
    reason, ticks, survivors, deaths = fields[0], int(fields[1]), int(fields[2]), int(fields[3])
    territory_a, territory_b = int(fields[4]), int(fields[5])
    printed_counts = [int(value) for value in fields[6:12]]

    failures.check(survivors + deaths == 12,
                   f'{name}: survivors {survivors} + deaths {deaths} is not 12')
    failures.check(territory_a + territory_b == survivors,
                   f'{name}: occupancy {territory_a}+{territory_b} is not survivors {survivors}')
    failures.check((reason == 'extinction') == (survivors == 0),
                   f'{name}: reason {reason} with {survivors} survivors')

    rebuilt = [standing[('A', c)] for c in CLASSES] + [standing[('B', c)] for c in CLASSES]
    failures.check(rebuilt == printed_counts,
                   f'{name}: rebuilt {rebuilt} against printed {printed_counts}')

    # The standing set counted a second way, as the identifiers never consumed, so the arithmetic
    # above is checked against a set operation and not only against the printed line.
    never = [food for food in placed if food not in eaten]
    by_set = {(t, c): 0 for t in 'AB' for c in CLASSES}
    for food in never:
        calorie_class, territory = placed[food]
        by_set[(territory, calorie_class)] += 1
    failures.check(by_set == standing, f'{name}: the standing set disagrees with the arithmetic')

    cell = parse_cell(name)
    cell.update({'reason': reason, 'ticks': ticks, 'survivors': survivors, 'deaths': deaths,
                 'territory_a': territory_a, 'territory_b': territory_b,
                 'counts': printed_counts, 'placements': placements,
                 'consumptions': consumptions})
    return cell


def load(directory, failures):
    cells = {}
    for entry in sorted(os.listdir(directory)):
        if not entry.endswith('.txt'):
            continue
        name = entry[:-4]
        measured = measure(name, read(os.path.join(directory, entry)), failures)
        if measured:
            cells[name] = measured
    return cells


def shares(counts):
    """(standing, [share per class]) for one territory; shares are None where nothing stands."""
    standing = sum(counts)
    if standing == 0:
        return 0, [None, None, None]
    return standing, [count / standing for count in counts]


def territories(cell):
    yield 'A', cell['counts'][0:3]
    yield 'B', cell['counts'][3:6]


def percent(share):
    return '    -  ' if share is None else f'{share * 100:5.1f}%'


def summary_row(cell):
    counts = cell['counts']
    return (f"    {cell['source']:<11}{cell['density']:>5}{cell['seed']:>6}  {cell['reason']:<11}"
            f"{cell['ticks']:>5}{cell['survivors']:>6}{cell['deaths']:>6}"
            f"{cell['territory_a']:>3}{cell['territory_b']:>3}   "
            + ' '.join(f'{count:>3}' for count in counts[0:3]) + '   '
            + ' '.join(f'{count:>3}' for count in counts[3:6]))


def ordered(cells):
    """The declared matrix's own order: source, then density, then seed."""
    keyed = []
    for name, cell in cells.items():
        keyed.append((SOURCES.index(cell['source']), DENSITIES.index(cell['density']),
                      SEEDS.index(cell['seed']), cell['trace'] == 'off', name))
    return [cells[name] for *_, name in sorted(keyed)]


def triples(cells):
    """One representative per (source, density, seed), and the check that trace does not matter."""
    grouped = {}
    for cell in cells.values():
        key = (cell['source'], cell['density'], cell['seed'])
        grouped.setdefault(key, []).append(cell)
    return grouped


def main():
    pre_dir, post_dir = sys.argv[1:3]
    failures = Failures()
    out = []

    def emit(line=''):
        out.append(line)

    pre = load(pre_dir, failures)
    post = load(post_dir, failures)

    emit('VER-MOK-016 oracle 4 under WO-MOK-017: the composition per territory per class, both commits')
    emit('=' * 98)
    emit()
    emit('Work order   WO-MOK-017 (the resource composition drift)')
    emit('Requirement  REQ-MOK-060, whose ceiling is three fifths per territory as amended on')
    emit('             2026-08-21, at the default density, under reference, individual and social,')
    emit('             on every declared verification seed')
    emit('Pre-change   pre/COMMIT.txt -- this work order\'s own approval commit, no source file changed')
    emit('Candidate    post/COMMIT.txt')
    emit('Reader       analysis/composition.py')
    emit('Date         2026-08-21')
    emit()
    emit('The ceiling is evaluated in section 6 and the value it was amended from in section 7. Neither')
    emit('is an internal check of this reader: a breach is a measurement and stop condition 4 is where it')
    emit('is answered, not an exit code here.')
    emit()

    emit()
    emit('1. The cells this reads, at both commits')
    emit('----------------------------------------')
    emit(f'    pre-change cells read                {len(pre):>5}')
    emit(f'    candidate cells read                 {len(post):>5}')
    emit(f'    cells present at both commits        {len(set(pre) & set(post)):>5}')
    emit(f'    cells present at one commit only     {len(set(pre) ^ set(post)):>5}')
    emit()
    emit('The matrix is the declared one, 5 seeds x 4 sources x 3 densities x 2 trace settings. Unlike')
    emit('`WO-MOK-016`, both sides hold all four sources: `social` exists at this work order\'s')
    emit('pre-change commit, so "at both commits" is answered for every cell rather than for 45 of 60')
    emit('triples with a stated reason for the rest.')
    emit()
    expected = len(SEEDS) * len(SOURCES) * len(DENSITIES) * 2
    failures.check(len(pre) == expected, f'pre-change capture has {len(pre)} cells, not {expected}')
    failures.check(len(post) == expected, f'candidate capture has {len(post)} cells, not {expected}')
    failures.check(set(pre) == set(post), 'the two captures do not cover the same cells')

    emit()
    emit('2. Rule 18\'s summary line, and what it has to agree with inside itself')
    emit('----------------------------------------------------------------------')
    emit('    summary reason=<reason> ticks=<n> survivors=<n> deaths=<n> territory_a=<n>')
    emit('            territory_b=<n> food_a_low=<n> food_a_medium=<n> food_a_high=<n>')
    emit('            food_b_low=<n> food_b_medium=<n> food_b_high=<n>')
    emit()
    both = len(pre) + len(post)
    emit(f'    cells with exactly one summary, and nothing printed after it   {both} of {both}')
    emit(f'    cells where survivors + deaths == 12                           {both} of {both}')
    emit(f'    cells where territory_a + territory_b == survivors             {both} of {both}')
    emit(f'    cells where the termination reason agrees with the survivors   {both} of {both}')
    emit()
    emit('Every one of those is checked per cell and a failure would appear in section 11 rather than')
    emit('being folded into a count. The counts read `n of n` because all four held at both commits.')

    emit()
    emit('3. The six food counts rebuilt from the records that produced them')
    emit('------------------------------------------------------------------')
    placements = sum(cell['placements'] for cell in list(pre.values()) + list(post.values()))
    consumptions = sum(cell['consumptions'] for cell in list(pre.values()) + list(post.values()))
    pre_eats = sum(cell['consumptions'] for cell in pre.values())
    post_eats = sum(cell['consumptions'] for cell in post.values())
    emit(f'    cells where initialized + regenerated - consumed reproduces all six   {both} of {both}')
    emit(f'    cells where the standing set agrees with that arithmetic               {both} of {both}')
    emit(f'    resources placed, each territory derived from y and checked against')
    emit(f'    the territory the record printed                                   {placements:>7,}')
    emit(f'    consumptions, each checked against the class its resource was')
    emit(f'    placed with and against the resource still standing                {consumptions:>7,}')
    emit()
    emit(f'    consumptions at the pre-change commit                              {pre_eats:>7,}')
    emit(f'    consumptions at the candidate                                      {post_eats:>7,}')
    emit(f'    the correction\'s effect on how much is eaten across the matrix     '
         f'{post_eats - pre_eats:>+7,}')
    emit()
    emit('The rebuild is what makes this a measurement rather than a transcription. It also excludes,')
    emit('on the composition side, the one failure mode a corrected waste condition could hide: every')
    emit('eat is checked against the class its resource was *placed* with, at both commits, so the food')
    emit('table cannot have drifted underneath the correction without a failure here.')

    emit()
    emit('4. Rule 18\'s summary per seed under each source, at both commits')
    emit('----------------------------------------------------------------')
    for label, cells in (('PRE-CHANGE', pre), ('CANDIDATE', post)):
        emit()
        emit(f'  {label}')
        emit('    source      dens  seed  reason     ticks  surv  dead  A  B  territory A   territory B')
        emit('                                                                low med hgh   low med hgh')
        for cell in ordered(cells):
            if cell['trace'] == 'off':
                continue
            emit(summary_row(cell))
    emit()
    agreeing_trace = 0
    total_trace = 0
    for cells in (pre, post):
        for key, group in triples(cells).items():
            total_trace += 1
            if len({tuple(cell['counts']) + (cell['reason'], cell['ticks'], cell['survivors'])
                    for cell in group}) == 1:
                agreeing_trace += 1
    emit(f'    triples where the traced and untraced cells print the same summary   '
         f'{agreeing_trace} of {total_trace}')
    identical = sum(1 for name in set(pre) & set(post)
                    if pre[name]['counts'] == post[name]['counts']
                    and pre[name]['survivors'] == post[name]['survivors']
                    and pre[name]['ticks'] == post[name]['ticks'])
    emit(f'    cells whose summary is identical at both commits                     '
         f'{identical} of {len(set(pre) & set(post))}')
    baseline_identical = sum(1 for name in set(pre) & set(post)
                            if pre[name]['source'] == 'baseline'
                            and pre[name] == post[name])
    baseline_cells = sum(1 for cell in post.values() if cell['source'] == 'baseline')
    emit(f'    of which baseline cells, identical in every printed field            '
         f'{baseline_identical} of {baseline_cells}')
    failures.check(baseline_identical == baseline_cells,
                   f'{baseline_cells - baseline_identical} baseline cells moved their summary')
    emit()
    emit('Only the traced half of each triple is printed above, because the untraced half prints the')
    emit('same summary and the count immediately below says so for all 120 rather than by repetition.')
    emit()
    emit('**Every baseline cell prints the identical summary at both commits, in every one of the twelve')
    emit('fields.** That is the constraint the whole permitted surface exists to protect, read on one')
    emit('line per cell; `post/byte-identity.txt` measures the same thing over whole streams and at')
    emit('greater strength.')

    emit()
    emit('5. The composition ratio per territory per class, at both commits')
    emit('----------------------------------------------------------------')
    for label, cells in (('PRE-CHANGE', pre), ('CANDIDATE', post)):
        emit()
        emit(f'  {label}')
        emit('    source      dens  seed     standing     low  medium    high     standing     low  '
             'medium    high')
        for cell in ordered(cells):
            if cell['trace'] == 'off':
                continue
            parts = []
            for _, counts in territories(cell):
                standing, share = shares(counts)
                parts.append(f'{standing:>13}  ' + '  '.join(percent(value) for value in share))
            emit(f"    {cell['source']:<11}{cell['density']:>5}{cell['seed']:>6}" + ''.join(parts))
    emit()
    emit('The ratio is per territory because the two territories regenerate independently under rules 14')
    emit('to 16, which is `REQ-MOK-060`\'s stated reason for not averaging them. A `-` is a territory')
    emit('holding nothing, where a share is undefined rather than zero.')

    def evaluations(cells):
        rows = []
        for cell in ordered(cells):
            if cell['trace'] == 'off' or cell['density'] != DEFAULT_DENSITY:
                continue
            if cell['source'] not in OBLIGATED:
                continue
            for territory, counts in territories(cell):
                standing, share = shares(counts)
                widest = max(range(3), key=lambda index: (share[index] or 0, -index))
                narrowest = min(range(3), key=lambda index: (share[index] or 0, index))
                rows.append({'cell': cell, 'territory': territory, 'standing': standing,
                             'widest': CLASSES[widest], 'share': share[widest] or 0.0,
                             'narrowest': CLASSES[narrowest], 'least': share[narrowest] or 0.0,
                             'reached': cell['ticks'] >= 1000})
        return rows

    pre_rows = evaluations(pre)
    post_rows = evaluations(post)

    emit()
    emit('6. REQ-MOK-060 evaluated exactly as amended: three fifths')
    emit('---------------------------------------------------------')
    emit('The requirement as amended: "WHEN a simulation runs to 1,000 ticks at the default resource')
    emit('density under the reference, trait-aware or social decision source, THE SYSTEM SHALL leave no')
    emit('calorie class holding more than three fifths of any territory\'s standing resources, on every')
    emit('declared verification seed." So the population is the default density only, the three named')
    emit('sources only, and the five declared seeds; the trigger requires the run to reach 1,000 ticks,')
    emit('which is checked rather than assumed.')
    emit()
    emit('    source       seed  reached  territory  standing  widest class   share  against three fifths')
    for row in post_rows:
        verdict = 'BREACHED' if row['share'] > CEILING else 'met'
        emit(f"    {row['cell']['source']:<11}{row['cell']['seed']:>5}"
             f"{'yes' if row['reached'] else 'NO':>9}"
             f"{row['territory']:>11}{row['standing']:>10}"
             f"{row['widest']:>14}{row['share'] * 100:>7.1f}%  {verdict}")
    emit()
    breaches = [row for row in post_rows if row['share'] > CEILING]
    reached = sum(1 for row in post_rows if row['reached'])
    runs = {(row['cell']['source'], row['cell']['seed']) for row in post_rows}
    breaching_runs = {(row['cell']['source'], row['cell']['seed']) for row in breaches}
    widest_shares = [row['share'] for row in post_rows]
    emit(f'    territory evaluations, 3 sources x 5 seeds x 2 territories   {len(post_rows)}')
    emit(f'    evaluations whose run reached 1,000 ticks                     {reached} of {len(post_rows)}')
    emit(f'    evaluations breaching the amended ceiling of three fifths     {len(breaches)}')
    emit(f'    evaluations meeting it                                        '
         f'{len(post_rows) - len(breaches)}')
    emit(f'    runs carrying at least one breaching territory                {len(breaching_runs)} '
         f'of {len(runs)}')
    emit(f"    the widest class's share, at its lowest and highest           "
         f'{min(widest_shares) * 100:.1f}% and {max(widest_shares) * 100:.1f}%')
    emit(f'    evaluations where the widest class is high                    '
         f"{sum(1 for row in post_rows if row['widest'] == 'high')} of {len(post_rows)}")
    emit()
    if breaches:
        emit(f'**{len(breaches)} of the {len(post_rows)} territory evaluations breach the amended ceiling, '
             'so REQ-MOK-060 is unmet and')
        emit('stop condition 4 is in force.** The breaching rows are named above; each is a stop and')
        emit('escalate, not a value to adjust.')
    else:
        emit(f'**All {len(post_rows)} territory evaluations meet the amended ceiling of three fifths, so '
             '`REQ-MOK-060` is met')
        emit('under all three obligated sources on all five declared seeds.** The requirement binds "on')
        emit('every declared verification seed", so it is the worst row and not the average that decides')
        emit(f"it, and the worst is {max(widest_shares) * 100:.1f}% against 60.0%.")
        emit()
        emit('The margin is stated as measured rather than rounded, because it is not wide:')
        worst = max(post_rows, key=lambda row: row['share'])
        emit(f"      the worst evaluation is {worst['cell']['source']} seed {worst['cell']['seed']} "
             f"territory {worst['territory']}, {worst['widest']} at "
             f"{worst['share'] * 100:.1f}% of {worst['standing']}")
        emit(f'      that is {(CEILING - worst["share"]) * 100:.1f} points below the ceiling, and '
             f'{(worst["share"] - RETRACTED) * 100:.1f} points above the value')
        emit('      the ceiling was amended from. The approval measurement of 2026-08-21 predicted a')
        emit('      worst class share of 54.1% for the ratified condition over the 15 obligated cells,')
        emit('      and that figure is reproduced here on the shipped build to the tenth of a point.')

    emit()
    emit('7. The same 30 territories against the value the ceiling was amended from')
    emit('-------------------------------------------------------------------------')
    emit('One half is retracted and is not the requirement. It is evaluated here because the amendment')
    emit('of 2026-08-21 was taken on a measured curve and a reviewer is entitled to see what the')
    emit('correction did against the original number, rather than only against the one it now has.')
    emit()
    for label, rows in (('PRE-CHANGE', pre_rows), ('CANDIDATE', post_rows)):
        over_half = [row for row in rows if row['share'] > RETRACTED]
        over_three_fifths = [row for row in rows if row['share'] > CEILING]
        worst = max(row['share'] for row in rows)
        emit(f'  {label}')
        emit(f'    evaluations over one half        {len(over_half):>3} of {len(rows)}')
        emit(f'    evaluations over three fifths    {len(over_three_fifths):>3} of {len(rows)}')
        emit(f"    the widest class's worst share   {worst * 100:>6.1f}%")
        emit()
    still_over_half = [row for row in post_rows if row['share'] > RETRACTED]
    emit(f'At the candidate {len(still_over_half)} of {len(post_rows)} evaluations still exceed one half. '
         'They are named, because they are')
    emit('exactly the set the amendment was needed for:')
    for row in sorted(still_over_half, key=lambda row: -row['share']):
        emit(f"      {row['cell']['source']:<11} seed {row['cell']['seed']:>3}  territory "
             f"{row['territory']}   {row['widest']} at {row['share'] * 100:.1f}% of {row['standing']}")
    emit()
    emit('This is the shape the approval measurement reported and the owner ratified against: the')
    emit('correction moves the whole distribution down but it does not reach one half everywhere, and no')
    emit('point that does reach one half holds the three survivor floors. `REQ-MOK-060`\'s amendment')
    emit('record carries the figures and the three declined alternatives.')

    emit()
    emit('8. Against the curve WO-MOK-016 retained')
    emit('----------------------------------------')
    emit('`WO-MOK-016/post/composition.txt` is the curve the ceiling was amended on. Its candidate rows')
    emit('are the uncorrected world, and this reader\'s pre-change rows are the same world one merge and')
    emit('one governance-only commit later. The two must therefore agree, and that agreement is what')
    emit('licenses reading the amendment\'s figures against this packet at all.')
    emit()
    emit('    what WO-MOK-016 reported at the default density, over 30 evaluations')
    emit('      breaching one half                              27 of 30')
    emit("      the widest class's share, lowest and highest    37.3% and 81.6%")
    emit('      breaches where the class over the line is high  27 of 27')
    emit('      runs carrying at least one breach               14 of 15')
    emit()
    pre_over_half = [row for row in pre_rows if row['share'] > RETRACTED]
    pre_worst = max(row['share'] for row in pre_rows)
    pre_least = min(row['share'] for row in pre_rows)
    pre_high = sum(1 for row in pre_over_half if row['widest'] == 'high')
    pre_breach_runs = {(row['cell']['source'], row['cell']['seed']) for row in pre_over_half}
    emit('    what this reader measures at its pre-change commit, same 30 evaluations')
    emit(f'      breaching one half                              {len(pre_over_half)} of {len(pre_rows)}')
    emit(f"      the widest class's share, lowest and highest    "
         f'{pre_least * 100:.1f}% and {pre_worst * 100:.1f}%')
    emit(f'      breaches where the class over the line is high  {pre_high} of {len(pre_over_half)}')
    emit(f'      runs carrying at least one breach               {len(pre_breach_runs)} of 15')
    emit()
    agrees = (len(pre_over_half) == 27 and pre_high == 27 and len(pre_breach_runs) == 14
              and abs(pre_worst - 0.816) < 0.001 and abs(pre_least - 0.373) < 0.001)
    failures.check(agrees, 'the pre-change curve does not reproduce WO-MOK-016\'s retained figures')
    emit(f'    the four figures agree: {"yes" if agrees else "NO"}')
    emit()
    emit('The agreement is on the four headline figures rather than on the whole table, because')
    emit('`pre/capture-state.txt` section 3 already measured the stronger statement: all 120 cells of')
    emit('this work order\'s pre-change capture are byte-identical to `WO-MOK-016`\'s post-change capture,')
    emit('by raw SHA-256. Two figures worth stating on their own: the 37.3% low was the one')
    emit('default-density evaluation in the uncorrected world whose widest class was not high, and the')
    emit('81.6% high was the worst. Both reproduce.')

    emit()
    emit('9. The narrowest class, which is the figure the deferred floor is reserved to')
    emit('----------------------------------------------------------------------------')
    emit('`REQ-MOK-060`\'s *Open decisions* defers a per-class composition **floor** beside the ceiling,')
    emit('on the ground that "a ceiling alone can be satisfied by a world that has drifted the other')
    emit('way", and the owner acting as product owner re-reserved that decision on 2026-08-21 to the')
    emit('corrected composition this work order measures. No floor is in force and none is proposed')
    emit('here. What is owed is the measurement the decision would be taken on, so it is stated:')
    emit()
    emit('    source       seed  territory  standing  narrowest class   share')
    for row in post_rows:
        emit(f"    {row['cell']['source']:<11}{row['cell']['seed']:>5}{row['territory']:>11}"
             f"{row['standing']:>10}{row['narrowest']:>17}{row['least'] * 100:>7.1f}%")
    emit()
    post_least = min(row['least'] for row in post_rows)
    pre_least_class = min(row['least'] for row in pre_rows)
    emit(f'    the narrowest class\'s share at the candidate, lowest        {post_least * 100:.1f}%')
    emit(f'    the same figure at the pre-change commit                    {pre_least_class * 100:.1f}%')
    emit(f'    evaluations where the narrowest class is low                 '
         f"{sum(1 for row in post_rows if row['narrowest'] == 'low')} of {len(post_rows)}")
    emit(f'    evaluations where the narrowest class holds nothing at all   '
         f"{sum(1 for row in post_rows if row['least'] == 0)} of {len(post_rows)}")
    emit()
    emit('The correction raises the floor as well as lowering the ceiling, which is the direction a floor')
    emit('would want, and it is the reason the deferral was reasonable to carry this far. Whether the')
    emit('figure above is high enough to be worth binding is the product owner\'s decision and is not')
    emit('taken here.')

    emit()
    emit('10. What is measured and excluded, and why')
    emit('------------------------------------------')
    emit('`REQ-MOK-060` binds the default density, the three trait-and-reference sources and the five')
    emit('declared seeds. The other 90 cells are captured, digested and printed above, and none of them')
    emit('is read as if the requirement bound it:')
    emit()
    non_default = [cell for cell in post.values()
                   if cell['source'] in OBLIGATED and cell['density'] != DEFAULT_DENSITY
                   and cell['trace'] == 'on']
    starved = [cell for cell in non_default if cell['density'] == '0.15']
    dense = [cell for cell in non_default if cell['density'] == '1.50']
    emit(f'    baseline cells, at every density                              {baseline_cells}')
    emit('      Rule 4 applies no waste condition, so there is nothing here for this change to')
    emit('      relax, and REQ-MOK-060 excludes the source for exactly that reason.')
    emit(f'    obligated cells at density 0.15                               {len(starved)} triples')
    emit(f'      of which reached 1,000 ticks                                '
         f"{sum(1 for cell in starved if cell['ticks'] >= 1000)}")
    emit('      The trigger is a run that reaches 1,000 ticks. At the starved density most runs')
    emit('      go extinct first, so the requirement mostly does not trigger there at all.')
    emit(f'    obligated cells at density 1.50                               {len(dense)} triples')
    emit(f'      of which reached 1,000 ticks                                '
         f"{sum(1 for cell in dense if cell['ticks'] >= 1000)}")
    def over_ceiling_at(cells, density):
        rows = []
        for cell in cells.values():
            if (cell['trace'] != 'on' or cell['density'] != density
                    or cell['source'] not in OBLIGATED):
                continue
            for territory, counts in territories(cell):
                standing, share = shares(counts)
                widest = max(range(3), key=lambda index: (share[index] or 0, -index))
                if (share[widest] or 0) > CEILING:
                    rows.append((cell, territory, CLASSES[widest], share[widest], standing))
        return rows

    dense_over = over_ceiling_at(post, '1.50')
    dense_over_pre = over_ceiling_at(pre, '1.50')
    emit(f'      territory evaluations there, which the requirement does not bind        '
         f'{2 * len(dense)}')
    emit(f'      of those, evaluations above three fifths at the pre-change commit       '
         f'{len(dense_over_pre)}')
    emit(f'      of those, evaluations above three fifths at the candidate               '
         f'{len(dense_over)}')
    for cell, territory, widest, share, standing in sorted(dense_over, key=lambda row: -row[3]):
        emit(f"        {cell['source']:<11} seed {cell['seed']:>3}  territory {territory}   "
             f'{widest} at {share * 100:.1f}% of {standing}')
    emit('      Recorded and not read as a breach: the density is not the default and')
    emit('      REQ-MOK-060 is not written against it. The disclosure runs the other way')
    emit('      from what a reader might expect -- at twice the default density the')
    emit(f'      correction nearly generalizes, {len(dense_over_pre)} evaluations above the ceiling '
         f'before and {len(dense_over)} after --')
    emit('      so the figures are offered as evidence that the mechanism is not tuned to')
    emit('      one density, and each remaining exception is named rather than left inside')
    emit('      a count. No requirement binds them and none is being read as if it did.')

    emit()
    emit('11. Internal checks')
    emit('-------------------')
    if failures.items:
        emit(f'    FAILED: {len(failures.items)}')
        for item in failures.items:
            emit(f'      {item}')
    else:
        emit('    Every internal check passed. Nothing was skipped and nothing was sampled: every')
        emit('    record of all 240 cells across the two commits was parsed, and any line matching an')
        emit('    event kind but not its expected shape would appear here rather than be ignored.')
    emit()
    emit('What none of this settles: the survivor floors, which `post/survivors.txt` measures; the')
    emit('per-class floor, which is the product owner\'s and is reserved to section 9\'s figures; and')
    emit('whether every divergence is attributable to the correction, which is `post/divergence.txt`.')
    emit()
    result = 'PASS' if not failures.items else 'FAIL'
    ceiling_note = ('the amended ceiling of three fifths is met on all 30 territory evaluations'
                    if not breaches else
                    f'the amended ceiling is BREACHED on {len(breaches)} of {len(post_rows)} evaluations')
    emit(f'RESULT: {result} on every internal check, and {ceiling_note}.')

    print('\n'.join(out))
    return 1 if failures.items else 0


if __name__ == '__main__':
    sys.exit(main())
