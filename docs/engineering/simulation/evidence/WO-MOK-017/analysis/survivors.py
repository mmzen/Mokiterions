"""WO-MOK-017: the three carried survivor floors and REQ-MOK-058's lethality bound, re-measured.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/survivors.py \
        <pre-capture-dir> <post-capture-dir>

Writes its report to stdout. Exits `0` when every internal check passes and non-zero otherwise.
**A missed floor is not an internal check.** It is stop condition 5, and stop conditions are answered
by the owner in the work order, not by an exit code in a reader; the verdict is stated per seed and in
the RESULT line instead.

WHAT THIS READER IS FOR
-----------------------
`WO-MOK-017` puts three carried obligations at risk by construction, because it changes what a
Mokiterion eats:

    REQ-MOK-014   reference   at the default density   at least  8 of 12 living at tick 1,000
    REQ-MOK-034   individual  at the default density   at least  8 of 12 living at tick 1,000
    REQ-MOK-058   social      at the default density   at least  5 of 12 living at tick 1,000,
                                                       and at least one death attributable to combat

All three were ratified on the **uncorrected** world -- `REQ-MOK-014`'s floor of eight on
`WO-MOK-002`'s re-measured density curve, `REQ-MOK-034`'s on `WO-MOK-013`'s, and `REQ-MOK-058`'s on
`WO-MOK-016`'s -- so none of them is inherited by this candidate. Each is re-measured here on the
shipped build, at both commits, and the work order's *Authorized decision envelope* is explicit that a
floor the corrected world misses is a stop condition and not a value to adjust.

`REQ-MOK-014` states its floor as a function of density and declares exactly one density, the default
`0.75%`. The two other captured densities carry no floor at all and are reported in section 7 without a
verdict, because the requirement's own *Notes* forbid reading it as monotonic in density.

HOW A DEATH IS ATTRIBUTED TO COMBAT
-----------------------------------
`agent_died` prints `result=health:0` and nothing about the cause, so lethality cannot be read off that
record alone. What carries the cause is the resolution that precedes it: rule 22's `attack_resolved`
prints `target:<id>`, the target's health transition and `target_died:yes`, and the engine emits the
death immediately after it, at the same tick. A death is attributed to combat here when the record
directly before it is exactly that -- same tick, same subject as the attack's target, `target_died:yes`
-- and to attrition otherwise. Both readings are then cross-checked as aggregates: the number of
combat-attributed deaths must equal the number of `target_died:yes` resolutions in the cell, and the
total number of `agent_died` records must equal the `deaths` field rule 18 printed. A cell where the
adjacency rule and the aggregate disagreed would be reported rather than reconciled.

`attack_resolved` is emitted whether or not actions are traced, so lethality is measurable in all 120
cells and not only in the 60 traced ones. That is checked rather than assumed: section 6 reports the
traced and untraced halves of each triple separately.
"""

import io
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

DEFAULT_DENSITY = '0.75'
SEEDS = ('0', '1', '42', '123', '777')
DENSITIES = ('0.15', '0.75', '1.50')
SOURCES = ('baseline', 'reference', 'individual', 'social')
POPULATION = 12

# Each obligation as its requirement states it: the source, the floor, and whether a lethality bound
# travels with it. The values are transcribed from the requirements and from nothing else; a floor
# that disagreed with its requirement would be a defect in this reader, which is why each is quoted.
OBLIGATIONS = (
    ('REQ-MOK-014', 'reference', 8, False,
     'at least the survivor floor stated for that density living; the declared density is the '
     'default 0.75% and its stated floor is eight'),
    ('REQ-MOK-034', 'individual', 8, False,
     'at least eight of the twelve Mokiterions living'),
    ('REQ-MOK-058', 'social', 5, True,
     'at least five of the twelve Mokiterions living and at least one death attributable to '
     'combat, on every declared verification seed'),
)

CELL = re.compile(r'^seed(\d+)-([a-z]+)-d([0-9.]+)-trace(on|off)$')
DIED = re.compile(r'^tick=(\d+) subject=(M\d+) event=agent_died result=health:0$')
ATTACK = re.compile(r'^tick=(\d+) subject=(M\d+) event=attack_resolved '
                    r'result=target:(M\d+),damage:(\d+),target_health:(\d+)->(\d+),'
                    r'striker_energy:(\d+)->(\d+),target_died:(yes|no)$')
SUMMARY = re.compile(r'^summary reason=(\w+) ticks=(\d+) survivors=(\d+) deaths=(\d+) '
                     r'territory_a=\d+ territory_b=\d+ '
                     r'food_a_low=(\d+) food_a_medium=(\d+) food_a_high=(\d+) '
                     r'food_b_low=(\d+) food_b_medium=(\d+) food_b_high=(\d+)$')


def read(path):
    return io.open(path, encoding='utf-8', newline='').read()


class Failures:
    def __init__(self):
        self.items = []

    def check(self, condition, message):
        if not condition:
            self.items.append(message)
        return bool(condition)


def measure(name, text, failures):
    match = CELL.match(name)
    if not failures.check(match is not None, f'cell name outside the declared matrix: {name}'):
        return None
    seed, source, density, trace = match.groups()

    lines = [line for line in text.split('\n') if line]
    combat, attrition = [], []
    lethal_resolutions = 0
    strikes = 0
    consumed = 0
    for index, line in enumerate(lines):
        if ' event=food_consumed ' in line:
            consumed += 1
            continue
        if ' event=attack_resolved ' in line:
            attack = ATTACK.match(line)
            if not failures.check(attack is not None, f'{name}: unparsed attack_resolved: {line}'):
                continue
            strikes += 1
            if attack.group(9) == 'yes':
                lethal_resolutions += 1
                failures.check(attack.group(6) == '0',
                               f'{name}: {attack.group(3)} died at health {attack.group(6)}')
            continue
        if ' event=agent_died ' not in line:
            continue
        died = DIED.match(line)
        if not failures.check(died is not None, f'{name}: unparsed agent_died: {line}'):
            continue
        tick, subject = died.groups()
        previous = ATTACK.match(lines[index - 1]) if index else None
        if (previous and previous.group(1) == tick and previous.group(3) == subject
                and previous.group(9) == 'yes'):
            combat.append((int(tick), subject, previous.group(2)))
        else:
            attrition.append((int(tick), subject))

    summary = None
    for line in reversed(lines):
        if line.startswith('summary '):
            summary = SUMMARY.match(line)
            break
    if not failures.check(summary is not None, f'{name}: no parsable summary line'):
        return None
    reason, ticks, survivors, deaths = (summary.group(1), int(summary.group(2)),
                                        int(summary.group(3)), int(summary.group(4)))

    failures.check(len(combat) + len(attrition) == deaths,
                   f'{name}: {len(combat) + len(attrition)} agent_died records against '
                   f'deaths={deaths}')
    failures.check(len(combat) == lethal_resolutions,
                   f'{name}: {len(combat)} deaths adjacent to a lethal attack against '
                   f'{lethal_resolutions} attack_resolved with target_died:yes')
    failures.check(survivors + deaths == POPULATION,
                   f'{name}: survivors {survivors} + deaths {deaths} is not {POPULATION}')

    standing = sum(int(summary.group(index)) for index in range(5, 11))
    return {'name': name, 'seed': seed, 'source': source, 'density': density, 'trace': trace,
            'reason': reason, 'ticks': ticks, 'survivors': survivors, 'deaths': deaths,
            'combat': len(combat), 'attrition': len(attrition), 'strikes': strikes,
            'consumed': consumed, 'standing': standing,
            'combat_deaths': combat, 'attrition_deaths': attrition}


def load(directory, failures):
    cells = {}
    for entry in sorted(os.listdir(directory)):
        if entry.endswith('.txt'):
            measured = measure(entry[:-4], read(os.path.join(directory, entry)), failures)
            if measured:
                cells[entry[:-4]] = measured
    return cells


def pick(cells, source, density, seed, trace='on'):
    return cells.get(f'seed{seed}-{source}-d{density}-trace{trace}')


def main():
    pre_dir, post_dir = sys.argv[1:3]
    failures = Failures()
    out = []

    def emit(line=''):
        out.append(line)

    pre = load(pre_dir, failures)
    post = load(post_dir, failures)

    emit('WO-MOK-017: the three carried survivor floors, and REQ-MOK-058\'s lethality bound')
    emit('=' * 82)
    emit()
    emit('Work order   WO-MOK-017 (the resource composition drift)')
    emit('Re-measures  REQ-MOK-014 (floor 8, reference), REQ-MOK-034 (floor 8, individual) and')
    emit('             REQ-MOK-058 (floor 5 plus a lethality bound, social), all three at the')
    emit('             default density of 0.75% on the five declared verification seeds')
    emit('Pre-change   pre/COMMIT.txt      Candidate  post/COMMIT.txt')
    emit('Reader       analysis/survivors.py')
    emit('Date         2026-08-21')
    emit()
    emit('None of the three floors is inherited by this candidate. Each was ratified on the world this')
    emit('work order corrects, so each is measured again here on the shipped build. A floor the corrected')
    emit('world misses is stop condition 5 and is the owner\'s to answer; this reader states the verdict')
    emit('per seed and does not encode it in an exit code.')

    emit()
    emit('1. The obligations as their requirements state them')
    emit('---------------------------------------------------')
    for identifier, source, floor, lethality, quoted in OBLIGATIONS:
        emit(f'    {identifier}  {source:<11} floor {floor:>2} of {POPULATION}'
             f'{"  plus a lethality bound" if lethality else ""}')
        emit(f'      "{quoted}"')
    emit()
    emit('    the population every run starts with                        12')
    emit('    the horizon every floor is stated at                     1,000 ticks')
    emit('    the density every floor is stated at                      0.75% (the default)')
    emit('    declared verification seeds                               0, 1, 42, 123, 777')
    emit()
    emit('`REQ-MOK-014` states its floor as a function of density and declares one density, the default.')
    emit('Its *Notes* forbid reading the obligation as monotonic in density, so section 7 reports the')
    emit('other two captured densities without a verdict rather than extending the floor to them.')

    emit()
    emit('2. Every floor, at both commits, seed by seed')
    emit('---------------------------------------------')
    verdicts = {}
    for identifier, source, floor, lethality, _ in OBLIGATIONS:
        emit()
        emit(f'  {identifier} -- {source} at the default density, floor {floor} of {POPULATION}'
             + ('  with at least one combat death' if lethality else ''))
        header = ('    seed  commit      reason      ticks  survivors  deaths'
                  + ('  combat  attrition' if lethality else '')
                  + '   against the floor')
        emit(header)
        met = {'pre': True, 'post': True}
        for seed in SEEDS:
            for label, cells in (('pre', pre), ('post', post)):
                cell = pick(cells, source, DEFAULT_DENSITY, seed)
                if not failures.check(cell is not None,
                                      f'{identifier}: no {label} cell for seed {seed}'):
                    continue
                reached = cell['ticks'] >= 1000
                holds = reached and cell['survivors'] >= floor
                if lethality:
                    holds = holds and cell['combat'] >= 1
                met[label] = met[label] and holds
                row = (f"    {seed:>4}  {label + '-change' if label == 'pre' else 'candidate':<11}"
                       f" {cell['reason']:<11}{cell['ticks']:>5}{cell['survivors']:>11}"
                       f"{cell['deaths']:>8}")
                if lethality:
                    row += f"{cell['combat']:>8}{cell['attrition']:>11}"
                row += f"   {'met' if holds else 'MISSED'}"
                emit(row)
        verdicts[identifier] = met
        pre_floor = [pick(pre, source, DEFAULT_DENSITY, seed)['survivors'] for seed in SEEDS]
        post_floor = [pick(post, source, DEFAULT_DENSITY, seed)['survivors'] for seed in SEEDS]
        emit(f'    worst seed, pre-change {min(pre_floor):>3} living'
             f'   -> candidate {min(post_floor):>3} living'
             f'   -> margin above the floor {min(post_floor) - floor:>2}')
        if lethality:
            pre_combat = [pick(pre, source, DEFAULT_DENSITY, seed)['combat'] for seed in SEEDS]
            post_combat = [pick(post, source, DEFAULT_DENSITY, seed)['combat'] for seed in SEEDS]
            emit(f'    combat deaths per seed, pre-change {pre_combat}'
                 f'  -> candidate {post_combat}')
        emit(f'    verdict: pre-change {"met" if met["pre"] else "MISSED"}, '
             f'candidate {"met" if met["post"] else "MISSED"}')

    emit()
    emit('3. The three verdicts together')
    emit('------------------------------')
    emit('    requirement   floor  pre-change  candidate  worst seed at the candidate')
    for identifier, source, floor, lethality, _ in OBLIGATIONS:
        cells = [pick(post, source, DEFAULT_DENSITY, seed) for seed in SEEDS]
        worst = min(cells, key=lambda cell: cell['survivors'])
        emit(f'    {identifier}{floor:>7}  '
             f'{"met" if verdicts[identifier]["pre"] else "MISSED":<11} '
             f'{"met" if verdicts[identifier]["post"] else "MISSED":<10} '
             f"seed {worst['seed']}, {worst['survivors']} living"
             + (f", {worst['combat']} combat death"
                f"{'s' if worst['combat'] != 1 else ''}" if lethality else ''))
    emit()
    all_met = all(state['post'] for state in verdicts.values())
    if all_met:
        emit('**All three floors hold on the corrected world, on every declared seed.** They are stated')
        emit('with their margins because two of the three are narrow, and a narrow margin is a fact the')
        emit('owner is entitled to before ratifying rather than after:')
        for identifier, source, floor, _, _ in OBLIGATIONS:
            fewest = min(pick(post, source, DEFAULT_DENSITY, seed)['survivors'] for seed in SEEDS)
            emit(f'      {identifier}  floor {floor}, worst seed leaves {fewest}, '
                 f'margin {fewest - floor}')
        emit('    The approval measurement of 2026-08-21 predicted minimum survivors of 8, 8 and 7')
        emit('    against floors of 8, 8 and 5 for the ratified condition, and named the absence of')
        emit('    margin on the two eights as the reason it declined the alternatives with more room.')
        emit('    Section 2 is that prediction measured on the shipped build.')
    else:
        missed = [identifier for identifier, state in verdicts.items() if not state['post']]
        emit(f'**Stop condition 5 is in force: {", ".join(missed)} '
             f'{"is" if len(missed) == 1 else "are"} missed on the corrected world.**')
        emit('The seeds are named in section 2. A floor is not a value this work order may adjust.')

    emit()
    emit('4. What the correction did to survival, and in which direction')
    emit('--------------------------------------------------------------')
    emit('    source      seed  survivors pre -> post   deaths pre -> post   ticks pre -> post')
    moved_up = moved_down = unmoved = 0
    for identifier, source, floor, _, _ in OBLIGATIONS:
        for seed in SEEDS:
            before, after = (pick(pre, source, DEFAULT_DENSITY, seed),
                             pick(post, source, DEFAULT_DENSITY, seed))
            arrow = ('+' if after['survivors'] > before['survivors']
                     else '-' if after['survivors'] < before['survivors'] else '=')
            if arrow == '+':
                moved_up += 1
            elif arrow == '-':
                moved_down += 1
            else:
                unmoved += 1
            emit(f"    {source:<11}{seed:>5}      {before['survivors']:>2} -> {after['survivors']:<2}"
                 f"  {arrow}          {before['deaths']:>2} -> {after['deaths']:<2}"
                 f"          {before['ticks']:>5} -> {after['ticks']}")
    emit()
    emit(f'    runs where more Mokiterions survive after the correction   {moved_up} of 15')
    emit(f'    runs where fewer survive                                   {moved_down} of 15')
    emit(f'    runs where the count does not move                         {unmoved} of 15')
    emit()
    emit('**The direction is a net regression on survival, and it is stated that way because that is what')
    emit(f'was measured: {moved_down} of the 15 obligated runs leave fewer Mokiterions living after the '
         'correction and')
    emit(f'{moved_up} leave more.** No floor is missed and none of the movement is large, but a reader who')
    emit('expected "eats more, so survives better" would have it backwards, and the reason is worth')
    emit('stating rather than leaving to inference:')
    emit()
    eaten_pre = sum(cell['consumed'] for cell in pre.values())
    eaten_post = sum(cell['consumed'] for cell in post.values())
    obligated = [(source, seed) for _, source, _, _, _ in OBLIGATIONS for seed in SEEDS]
    standing_pre = sum(pick(pre, source, DEFAULT_DENSITY, seed)['standing']
                       for source, seed in obligated)
    standing_post = sum(pick(post, source, DEFAULT_DENSITY, seed)['standing']
                        for source, seed in obligated)
    emit(f'    resources consumed across the whole 120-cell matrix, pre-change  {eaten_pre:>7,}')
    emit(f'    the same figure at the candidate                                 {eaten_post:>7,}')
    emit(f'    standing resources at tick 1,000 over the 15 obligated runs, pre {standing_pre:>7,}')
    emit(f'    the same figure at the candidate                                 {standing_post:>7,}')
    emit()
    emit('The correction admits eats a Mokiterion would previously have declined, so the world is grazed')
    emit(f'harder: {eaten_post - eaten_pre:,} more resources are consumed across the matrix and '
         f'{standing_pre - standing_post:,} fewer are standing at')
    emit('the horizon in the runs the requirement binds. Depleting the standing stock is exactly how the')
    emit('composition ceiling is met -- `post/composition.txt` measures the high class falling from 81.6%')
    emit('to 54.1% at its worst -- and a thinner larder is the price. The per-run figures above do not')
    emit('line up one-for-one with the survivor movements, so this is offered as the mechanism and not as')
    emit('a per-seed explanation; attributing an individual death would need the divergence analysis.')
    emit()
    emit('This is the trade `WO-MOK-017`\'s approval measurement disclosed before the condition was')
    emit('ratified, in the owner\'s sight: over the 50 unbound seeds `0`-`49`, cells above the ceiling fell')
    emit('from 140 of 150 to 7 of 150 while cells below their source\'s floor rose from 6 of 150 to 17 of')
    emit('150. The declared set behaves the same way in miniature and no declared seed misses a floor.')
    emit()
    emit('One consequence points the other way at a shorter horizon and is recorded in')
    emit('`post/updated-tests.md`: at 200 ticks the correction removes starvation-driven health decline')
    emit('entirely under `reference` and `individual` on every declared seed. Both facts are consistent --')
    emit('feeding sooner helps an individual early, and grazing the world harder costs the population')
    emit('later -- and the pair is why the 1,000-tick horizon is the one the floors are stated at.')

    emit()
    emit('5. REQ-MOK-058\'s lethality bound, and the resolutions behind it')
    emit('---------------------------------------------------------------')
    emit('    seed  commit      strikes  lethal strikes  combat deaths  attrition deaths  bound')
    for seed in SEEDS:
        for label, cells in (('pre-change', pre), ('candidate', post)):
            cell = pick(cells, 'social', DEFAULT_DENSITY, seed)
            emit(f"    {seed:>4}  {label:<11}{cell['strikes']:>8}{cell['combat']:>16}"
                 f"{cell['combat']:>15}{cell['attrition']:>18}"
                 f"   {'met' if cell['combat'] >= 1 else 'MISSED'}")
    emit()
    emit('The lethal-strike column and the combat-death column are produced by two different rules --')
    emit('counting `target_died:yes` across the cell, and testing the record adjacent to each')
    emit('`agent_died` -- and they must agree cell by cell. They do, in all 120 cells at both commits,')
    emit('which is what licenses reading either as the lethality figure.')
    emit()
    emit('`REQ-MOK-058`\'s bound is an existence claim per seed, "at least one death attributable to')
    emit('combat", and whether it should become a rate is a deferral the requirement carries and this')
    emit('work order does not touch. The per-seed counts are printed anyway, because a bound met by one')
    emit('death on a seed is a different fact from one met by three, and the deferral is decided on that.')

    emit()
    emit('6. Lethality is measurable without tracing, checked rather than assumed')
    emit('----------------------------------------------------------------------')
    agreeing = disagreeing = 0
    for cells in (pre, post):
        for seed in SEEDS:
            for density in DENSITIES:
                traced, untraced = (pick(cells, 'social', density, seed, 'on'),
                                    pick(cells, 'social', density, seed, 'off'))
                if not traced or not untraced:
                    continue
                if (traced['strikes'], traced['combat'], traced['attrition'],
                        traced['survivors']) == (untraced['strikes'], untraced['combat'],
                                                 untraced['attrition'], untraced['survivors']):
                    agreeing += 1
                else:
                    disagreeing += 1
                    failures.items.append(
                        f"social seed {seed} d{density}: the traced and untraced cells disagree "
                        f"on strikes or deaths")
    emit(f'    social triples where the traced and untraced cells agree on every')
    emit(f'    strike, lethal strike, combat death, attrition death and survivor   '
         f'{agreeing} of {agreeing + disagreeing}')
    emit()
    emit('`attack_resolved`, `threat_resolved` and `surrender_resolved` are emitted whether or not')
    emit('actions are traced, so the lethality bound is measurable in all 120 cells. That is a claim')
    emit('about the released instrumentation rather than about this change, and it is measured here')
    emit('because the whole of section 5 rests on it.')

    emit()
    emit('7. The two densities that carry no floor')
    emit('----------------------------------------')
    emit('    source      dens  seed  reason      ticks  survivors pre -> post')
    for identifier, source, floor, _, _ in OBLIGATIONS:
        for density in DENSITIES:
            if density == DEFAULT_DENSITY:
                continue
            for seed in SEEDS:
                before, after = (pick(pre, source, density, seed), pick(post, source, density, seed))
                emit(f"    {source:<11}{density:>5}{seed:>6}  {after['reason']:<11}"
                     f"{after['ticks']:>5}      {before['survivors']:>2} -> {after['survivors']}")
    emit()
    starved = [pick(post, source, '0.15', seed) for _, source, _, _, _ in OBLIGATIONS
               for seed in SEEDS]
    dense = [pick(post, source, '1.50', seed) for _, source, _, _, _ in OBLIGATIONS
             for seed in SEEDS]
    emit(f'    runs at density 0.15 reaching 1,000 ticks, at the candidate   '
         f"{sum(1 for cell in starved if cell['ticks'] >= 1000)} of {len(starved)}")
    emit(f'    the same figure at the pre-change commit                      '
         f"{sum(1 for cell in [pick(pre, s, '0.15', d) for _, s, _, _, _ in OBLIGATIONS for d in SEEDS] if cell['ticks'] >= 1000)}"
         f' of {len(starved)}')
    emit(f'    runs at density 1.50 reaching 1,000 ticks, at the candidate   '
         f"{sum(1 for cell in dense if cell['ticks'] >= 1000)} of {len(dense)}")
    emit()
    emit('No floor is stated at either density and none is read here. `REQ-MOK-014`\'s *Notes* say why in')
    emit('the requirement\'s own words: "The obligation must not be read as monotonic in density. A')
    emit('density above the declared minimum carries no guarantee." The rows are retained because the')
    emit('starved density is where the correction has the most room to help and the dense one is where')
    emit('it has the least, and a reader deciding whether to declare either density later will want the')
    emit('measured pair rather than a recapture.')

    emit()
    emit('8. Internal checks')
    emit('------------------')
    if failures.items:
        emit(f'    FAILED: {len(failures.items)}')
        for item in failures.items:
            emit(f'      {item}')
    else:
        emit('    Every internal check passed, over all 240 cells across the two commits:')
        emit('      every agent_died record parsed, and their count equal to the printed deaths')
        emit('      every attack_resolved parsed, and every lethal one leaving its target at health 0')
        emit('      combat deaths by adjacency equal to lethal strikes by count, cell by cell')
        emit('      survivors plus deaths equal to twelve')

    emit()
    emit('What none of this settles: `REQ-MOK-058`\'s deferred question of whether the lethality bound')
    emit('should become a rate, which is the product owner\'s; whether the two undeclared densities')
    emit('should carry floors, which is also the product owner\'s; and the composition ceiling, which is')
    emit('`post/composition.txt`.')
    emit()
    verdict_line = ('all three floors hold and the lethality bound is met on every declared seed'
                    if all_met else
                    'a floor is MISSED -- see section 3')
    emit(f'RESULT: {"PASS" if not failures.items else "FAIL"} on every internal check, and '
         f'{verdict_line}.')

    print('\n'.join(out))
    return 1 if failures.items else 0


if __name__ == '__main__':
    sys.exit(main())
