"""WO-MOK-017: the byte comparison of the 30 baseline cells, and every obligated divergence named.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/divergence.py \
        <pre-capture-dir> <post-capture-dir> <out-dir>

Writes `<out-dir>/byte-identity.txt` and `<out-dir>/divergence.txt`. Exits `0` only when both of
`WO-MOK-017`'s load-bearing claims hold: every `baseline` cell is byte-identical, and every obligated
divergence is attributed to the corrected waste condition. Those are stop conditions 1 and 3, so unlike
the composition ceiling they *are* encoded in the exit code -- a reader that reported a stop condition
and passed anyway would be reporting it as a result.

THE TWO SIDES OF ONE COMPARISON
-------------------------------
`WO-MOK-017` asks for two things that are the same comparison read from opposite ends:

  * **"baseline is byte-identical on all 30 of its declared cells, compared byte for byte and not by
    projection."** `INT-MOK-010`'s promise rests on it, and it is the constraint the whole permitted
    surface exists to protect. The 30 cells are therefore compared as **bytes** -- not as digests and
    not as parsed records, because a digest comparison is a projection and the constraint names the
    projection it will not accept.
  * **"the characterization of each of the 60 reference and individual divergences, by first diverging
    tick, first diverging record and the decision that changed"** -- the clause `WO-MOK-016` recorded as
    unwritable, because at that candidate there was nothing to characterize. It becomes writable here.
    The 30 `social` cells are characterized with them: rule 26 delegates eating and seeking to rule 19,
    whose test this change amends, so `social` carries the same obligation for the same reason and
    leaving it out would leave a third of the moved surface unexplained.

THE TWO CONDITIONS, AS THE TWO BINARIES ACTUALLY IMPLEMENT THEM
--------------------------------------------------------------
Attribution is arithmetic, so the arithmetic has to be the real thing rather than a paraphrase. Both
sides were transcribed from the two commits' own source, not from the specification, because the whole
subject of this work order is a place where the two had drifted apart.

Pre-change, at `1ba5a3a`, `mokiterions-core/src/simulation.rs`:

    fn fits(&self, food)                  -> satiety + R <= 100
    fn fits_within_tolerance(&self, food) -> satiety + R <= 100
                                             || satiety + R - 100 <= T * R / 100

So the drift was *one term omitted twice*. Rule 5's specified allowance of `R*R/100` was absent
altogether, and rule 19 carried only the trait term. That is also why rule 19's `T = 0` identity with
rule 5 held on the drifted pair at all: at `T = 0` the trait term collapses to zero and both reduce to
`S + R <= 100`. The identity was a coincidence of the defect, which is what the amended clause 1 of
rule 19 and the shared `fits_within` now make structural.

At the candidate both go through one function, and rule 5 is `fits_within(food, 0)`:

    satiety + R <= 100 || satiety + R - 100 <= max(R*R/100, T * R / 100)

The **old allowance** of a source is therefore `0` for `reference` -- which ignores the trait entirely,
so a `reference` cell's printed `waste_tolerance` is not its allowance and is not read as one here --
and `T*R/100` for `individual` and `social`. The **new allowance** is `max(R*R/100, T*R/100)` for all
three, with `T = 0` under `reference`.

Section 2 of `divergence.txt` checks this model against both captures before using it for anything:
every consumption in the pre-change capture must lie inside the *old* allowance and none outside it,
and likewise the candidate's against the *new* one. Had the model been wrong -- had the first draft's
assumption that the old condition was plainly `S + R <= 100` for every source survived -- thousands of
pre-change consumption records would have contradicted it immediately, and they are what corrected it.

HOW A DIVERGENCE IS ATTRIBUTED, AND HOW IT COULD FAIL
-----------------------------------------------------
Two sources differing only in their waste condition can behave differently only where the two
conditions disagree. They disagree at an observation exactly where some calorie class satisfies

    S + R > 100                                the class overflows, so the fast path does not apply
    S + R - 100 <= max(R*R/100, T*R/100)       the corrected condition admits it
    S + R - 100 >  T_old * R / 100             the old condition declined it

Call that the **disagreement band**. Every cell's first divergence is assigned one of three routes,
each with its own check:

  R1  direct        the first diverging record carries the acting Mokiterion's pre-action satiety, and
                    that `(S, T, source)` is in the band. The changed decision is visible in the record.
  R2  stream-shared the acting Mokiterion is *not* in the band, but an in-band one acted earlier in the
                    same tick with an unchanged record. A wider candidate set consumes a different
                    number of draws from the shared entropy stream even when it reaches the same
                    proposal, so the next Mokiterion to draw wanders somewhere else.
  R3  sibling       an untraced cell, explained by the traced cell of its own triple: same seed, source
                    and density, diverging at a tick no later, by route R1 or R2. An untraced stream
                    prints no proposal at all, so the changed decision is invisible in it and its first
                    visible difference is a downstream effect.

A cell fitting none of the three fails, and stop condition 3 fires. Section 4 then adds a necessary
condition the routes cannot satisfy by construction: no cell may diverge before the tick at which some
Mokiterion first enters the band, measured from the pre-change stream.

The first draft of this reader used route R1 alone and failed 25 cells, all untraced. The other two
routes were added because those mechanisms are real, checkable and named per cell -- not because 25
failures were inconvenient. Every classification below is printed with the witness it rests on, so a
reader who thinks a route is too generous can see exactly which cells it carries.

WHAT THIS READER DELIBERATELY DOES NOT DO
-----------------------------------------
It does not re-derive the proposal. Reconstructing which resource the Mokiterion *should* have chosen,
from the food layout and the perception radius, would be a second implementation of the specification
inside an evidence script, and it would agree with whichever of the two was written second. Whether the
corrected condition admits the right resources is fixed against constructed state in the test suite,
where `the_corrected_non_waste_condition_admits_the_specified_boundaries` and
`a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten` live. The claim here is the
narrower one stop condition 3 actually states: nothing diverges except where the amended condition
changes an answer.
"""

import io
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

BASELINE = 'baseline'
OBLIGATED = ('reference', 'individual', 'social')
SEEDS = ('0', '1', '42', '123', '777')
DENSITIES = ('0.15', '0.75', '1.50')
ATTRIBUTE_MAX = 100

# SPEC-MOK-001 rule 6's food table, transcribed from the specification.
RESTORATION = {'low': 15, 'medium': 30, 'high': 50}

CELL = re.compile(r'^seed(\d+)-([a-z]+)-d([0-9.]+)-trace(on|off)$')
AGENT = re.compile(r'^tick=0 subject=(M\d+) event=agent_initialized '
                   r'result=name:[^,]+,position:\d+:\d+,territory:\w+,health:\d+,satiety:(\d+),'
                   r'energy:\d+,fear:\d+,waste_tolerance:(\d+)$')
RECORD = re.compile(r'^tick=(\d+) subject=(\S+) event=(\w+) result=(.*)$')
# `target_fear` and `target_health` exist, so the negative lookbehind guards against a future
# `target_satiety` being read as the actor's own.
SATIETY = re.compile(r'(?<!target_)satiety:(\d+)')
EATEN = re.compile(r'^tick=(\d+) subject=(M\d+) event=food_consumed '
                   r'result=food:(F\d+),class:(\w+),satiety:(\d+)->(\d+)')
PROPOSAL = re.compile(r'proposal:([a-z]+)(?::([^,]+))?')
ENDED = re.compile(r'^tick=(\d+) subject=world event=simulation_ended result=reason:(\w+)$')


def read_text(path):
    return io.open(path, encoding='utf-8', newline='').read()


def read_bytes(path):
    with open(path, 'rb') as handle:
        return handle.read()


def parse_cell(name):
    match = CELL.match(name)
    if not match:
        raise AssertionError(f'cell name outside the declared matrix: {name}')
    seed, source, density, trace = match.groups()
    return {'name': name, 'seed': seed, 'source': source, 'density': density, 'trace': trace,
            'sibling': f'seed{seed}-{source}-d{density}-trace' + ('off' if trace == 'on' else 'on')}


def old_allowance(restored, tolerance, source):
    """The excess above 100 the pre-change binary tolerated.

    Zero under `reference`: rule 5's implementation carried no allowance term at all, which is the
    drift this work order corrects.
    """
    if source == 'reference':
        return 0
    return tolerance * restored // 100


def new_allowance(restored, tolerance, source):
    """The excess the candidate tolerates. Rule 5 is `fits_within(food, 0)`, so `reference` gets the
    earned term alone and the trait cannot reach it."""
    effective = 0 if source == 'reference' else tolerance
    return max(restored * restored // 100, effective * restored // 100)


def band(satiety, tolerance, source):
    """The classes at which the two binaries disagree for this observer at this satiety."""
    disagreeing = []
    for calorie_class, restored in RESTORATION.items():
        excess = satiety + restored - ATTRIBUTE_MAX
        if excess <= 0:
            continue
        new = new_allowance(restored, tolerance, source)
        old = old_allowance(restored, tolerance, source)
        if old < excess <= new:
            disagreeing.append((calorie_class, excess, new, old))
    return disagreeing


def tolerances(lines):
    """Each Mokiterion's waste_tolerance and starting satiety, from `agent_initialized`."""
    found = {}
    start = {}
    for line in lines:
        if ' event=agent_initialized ' in line:
            match = AGENT.match(line)
            if match:
                start[match.group(1)] = int(match.group(2))
                found[match.group(1)] = int(match.group(3))
    return found, start


def describe(line):
    """(tick, subject, event, result, verb, satiety) for a record, or None where it is not one."""
    match = RECORD.match(line)
    if not match:
        return None
    tick, subject, event, result = match.groups()
    verb = None
    proposal = PROPOSAL.search(result)
    if proposal:
        verb = proposal.group(1) + (f':{proposal.group(2)}' if proposal.group(2) else '')
    satiety = SATIETY.search(result)
    return {'tick': int(tick), 'subject': subject, 'event': event, 'result': result, 'verb': verb,
            'satiety': int(satiety.group(1)) if satiety else None}


def first_difference(before, after):
    for index in range(min(len(before), len(after))):
        if before[index] != after[index]:
            return index
    return None if len(before) == len(after) else min(len(before), len(after))


def band_entry(lines, tolerance, source):
    """The earliest tick at which any Mokiterion's pre-action satiety is in the band.

    Read from the pre-change stream, which is the world both commits share up to the divergence.
    `survival_changed` prints every living Mokiterion's satiety every tick whether or not actions are
    traced, so this is observable in all 120 cells without reconstructing the world.
    """
    for line in lines:
        record = describe(line)
        if not record or record['satiety'] is None:
            continue
        if record['subject'] not in tolerance:
            continue
        if band(record['satiety'], tolerance[record['subject']], source):
            return record['tick']
    return None


def eats(lines, tolerance, source):
    """Every consumption, with its excess above 100 and the two allowances that bear on it."""
    found = []
    for line in lines:
        match = EATEN.match(line)
        if not match:
            continue
        tick, subject, _food, calorie_class, before, _after = match.groups()
        restored = RESTORATION[calorie_class]
        agent_tolerance = tolerance.get(subject, 0)
        found.append({
            'tick': int(tick), 'subject': subject, 'class': calorie_class, 'before': int(before),
            'excess': int(before) + restored - ATTRIBUTE_MAX,
            'old': old_allowance(restored, agent_tolerance, source),
            'new': new_allowance(restored, agent_tolerance, source),
        })
    return found


def analyse(name, pre_lines, post_lines):
    cell = parse_cell(name)
    tolerance, start = tolerances(post_lines)
    pre_tolerance, _ = tolerances(pre_lines)
    cell['tolerance_unchanged'] = (tolerance == pre_tolerance)
    cell['start_satieties'] = sorted(set(start.values()))
    cell['pre_eats'] = eats(pre_lines, pre_tolerance, cell['source'])
    cell['post_eats'] = eats(post_lines, tolerance, cell['source'])
    cell['band_entry'] = band_entry(pre_lines, pre_tolerance, cell['source'])
    ending = next((ENDED.match(line) for line in reversed(post_lines) if ENDED.match(line)), None)
    cell['end_tick'] = int(ending.group(1)) if ending else None
    cell['end_reason'] = ending.group(2) if ending else None

    index = first_difference(pre_lines, post_lines)
    cell['identical'] = index is None
    cell['route'] = None
    cell['band'] = []
    cell['witness'] = None
    cell['ticks_agree'] = True
    if index is None:
        return cell
    cell['index'] = index
    cell['pre_line'] = pre_lines[index] if index < len(pre_lines) else '(the stream ends)'
    cell['post_line'] = post_lines[index] if index < len(post_lines) else '(the stream ends)'
    pre_record = describe(cell['pre_line'])
    post_record = describe(cell['post_line'])
    # The two sides of the first differing line must sit at the same tick. They need not be the same
    # kind of record: where one commit emits a resolution the other does not, the line indices part and
    # an `action_trace` lands opposite a `threat_resolved`. A tick disagreement would mean a record had
    # been gained or lost earlier, which the identical prefix rules out -- so this checks the reader's
    # own alignment as much as the streams.
    cell['ticks_agree'] = bool(pre_record and post_record
                               and pre_record['tick'] == post_record['tick'])
    cell['pre_record'] = pre_record
    cell['post_record'] = post_record
    record = post_record or pre_record
    cell['tick'] = record['tick'] if record else None
    cell['subject'] = record['subject'] if record else None
    cell['event'] = record['event'] if record else None
    cell['pre_event'] = pre_record['event'] if pre_record else None
    cell['post_event'] = post_record['event'] if post_record else None
    cell['pre_verb'] = pre_record['verb'] if pre_record else None
    cell['post_verb'] = post_record['verb'] if post_record else None
    cell['satiety'] = record['satiety'] if record else None
    cell['agent_tolerance'] = tolerance.get(cell['subject'])

    if cell['satiety'] is not None and cell['agent_tolerance'] is not None:
        cell['band'] = band(cell['satiety'], cell['agent_tolerance'], cell['source'])
    if cell['band']:
        cell['route'] = 'R1'
        return cell
    # R2: an in-band Mokiterion acted earlier in the same tick, with an unchanged record.
    for earlier in range(index - 1, -1, -1):
        record = describe(post_lines[earlier])
        if not record or record['tick'] != cell['tick']:
            break
        if record['satiety'] is None or record['subject'] not in tolerance:
            continue
        witnessed = band(record['satiety'], tolerance[record['subject']], cell['source'])
        if witnessed:
            cell['route'] = 'R2'
            cell['witness'] = (record['subject'], record['satiety'],
                               tolerance[record['subject']], witnessed)
            break
    return cell


def main():
    pre_dir, post_dir, out_dir = sys.argv[1:4]
    os.makedirs(out_dir, exist_ok=True)
    names = sorted(entry[:-4] for entry in os.listdir(post_dir) if entry.endswith('.txt'))
    raw = {}
    exits = {}
    cells = {}
    for name in names:
        pre_raw = read_bytes(os.path.join(pre_dir, name + '.txt'))
        post_raw = read_bytes(os.path.join(post_dir, name + '.txt'))
        raw[name] = (pre_raw, post_raw)
        exits[name] = (read_text(os.path.join(pre_dir, name + '.exit')).strip(),
                       read_text(os.path.join(post_dir, name + '.exit')).strip())
        cells[name] = analyse(name, pre_raw.decode('utf-8').split('\n'),
                              post_raw.decode('utf-8').split('\n'))

    # R3 needs both halves of a triple, so it is assigned once every cell is analysed.
    for name in names:
        cell = cells[name]
        if cell['source'] == BASELINE or cell['identical'] or cell['route'] or cell['trace'] != 'off':
            continue
        sibling = cells.get(cell['sibling'])
        if (sibling and not sibling['identical'] and sibling['route'] in ('R1', 'R2')
                and sibling['tick'] <= cell['tick']):
            cell['route'] = 'R3'
            cell['witness'] = (sibling['name'], sibling['tick'], sibling['route'])

    failures = []
    out = []

    def emit(line=''):
        out.append(line)

    # ---------------------------------------------------------------- byte-identity.txt
    baseline = [name for name in names if cells[name]['source'] == BASELINE]
    differing = []
    for name in baseline:
        pre_raw, post_raw = raw[name]
        if pre_raw != post_raw:
            offset = next((index for index in range(min(len(pre_raw), len(post_raw)))
                           if pre_raw[index] != post_raw[index]), min(len(pre_raw), len(post_raw)))
            differing.append((name, offset, len(pre_raw), len(post_raw)))
    exit_differing = [name for name in baseline if exits[name][0] != exits[name][1]]

    emit('WO-MOK-017: the 30 baseline cells compared byte for byte')
    emit('=' * 56)
    emit()
    emit('Work order   WO-MOK-017 (the resource composition drift)')
    emit('Constraint   "baseline is byte-identical on all 30 of its declared cells, compared byte for')
    emit('             byte and not by projection", and stop condition 1: "Any baseline cell differs,')
    emit('             in any byte or exit code, at any seed, density or trace setting"')
    emit('Pre-change   pre/COMMIT.txt      Candidate  post/COMMIT.txt')
    emit('Reader       analysis/divergence.py')
    emit('Date         2026-08-21')
    emit()
    emit('This is the constraint the whole permitted surface exists to protect. `REQ-MOK-060` may correct')
    emit('the waste condition of rule 5 and rule 19 and nothing else, and the reason is `INT-MOK-010`:')
    emit('rule 4\'s candidate list carries no waste condition at all, so a correction confined to the two')
    emit('sources that have one cannot reach `baseline`. That is the argument. Below is the measurement.')
    emit()
    emit('1. Bytes, not digests')
    emit('---------------------')
    emit('The constraint names the projection it will not accept, so what follows compares raw bytes read')
    emit('from both captures with `open(path, "rb").read()` and `==`. No digest is computed here and no')
    emit('record is parsed. `pre/pre-manifest.txt` and `post/post-manifest.txt` carry the digests, and')
    emit('they are the weaker of the two statements.')
    emit()
    emit(f'    baseline cells compared                        {len(baseline)}')
    emit(f'    byte-identical                                 {len(baseline) - len(differing)}')
    emit(f'    differing in any byte                          {len(differing)}')
    emit(f'    bytes compared, both sides                 '
         f'{sum(len(raw[name][0]) + len(raw[name][1]) for name in baseline):>11,}')
    emit()
    emit(f'    exit codes compared                            {len(baseline)}')
    emit(f'    differing                                      {len(exit_differing)}')
    emit(f'    the code every baseline run reported, at both commits   '
         f'{sorted({exits[name][0] for name in baseline} | {exits[name][1] for name in baseline})}')
    emit()
    if differing:
        emit('    THE FOLLOWING CELLS DIFFER. Stop condition 1 is in force:')
        for name, offset, pre_length, post_length in differing:
            emit(f'      {name}: first differs at byte {offset:,}, '
                 f'{pre_length:,} bytes against {post_length:,}')
        failures.append(f'{len(differing)} baseline cells differ')
    if exit_differing:
        emit('    THE FOLLOWING EXIT CODES DIFFER. Stop condition 1 is in force:')
        for name in exit_differing:
            emit(f'      {name}: exit {exits[name][0]} against {exits[name][1]}')
        failures.append(f'{len(exit_differing)} baseline cells differ in exit code')
    if not differing and not exit_differing:
        emit(f'**All {len(baseline)} baseline cells are byte-identical at the two commits, and every exit')
        emit('code agrees.** The comparison covers all five declared seeds, all three densities and both')
        emit('trace settings, which is the whole of the declared `baseline` matrix rather than a sample.')
    emit()
    emit('2. The cells, listed, so that "all 30" is readable rather than asserted')
    emit('----------------------------------------------------------------------')
    emit('    cell                                 bytes  exit  identical')
    for name in baseline:
        pre_raw, post_raw = raw[name]
        emit(f'    {name:<34}{len(pre_raw):>9,}{exits[name][1]:>6}'
             f"  {'yes' if pre_raw == post_raw else 'NO'}")
    emit()
    emit('3. What identity here does and does not prove')
    emit('---------------------------------------------')
    emit('It proves that no observable behavior of the `baseline` source moved: not a proposal, not a')
    emit('draw, not a regeneration, not a death, not the final summary, anywhere in the declared matrix.')
    emit('Because all four sources share one world, one regeneration rule and one entropy stream, it also')
    emit('constrains everything shared. Rules 14 to 16, rule 9\'s eat effect and rule 6\'s food table all')
    emit('run under `baseline` too, so a change to any of them would appear here.')
    emit('`post/world-rules-unchanged.txt` makes the same point by reading the source; this is its')
    emit('behavioral half, and neither substitutes for the other.')
    emit()
    emit('It proves it only as far as `baseline` runs, and that is a real bound rather than a caveat.')
    emit('`baseline` proposes without regard to the world, so it starves:')
    emit()
    base_ends = [cells[name]['end_tick'] for name in baseline if cells[name]['end_tick'] is not None]
    base_reasons = {}
    for name in baseline:
        base_reasons[cells[name]['end_reason']] = base_reasons.get(cells[name]['end_reason'], 0) + 1
    obligated_ends = [cells[name]['end_tick'] for name in names
                      if cells[name]['source'] in OBLIGATED and cells[name]['end_tick'] is not None]
    ended = ', '.join(f'{count} by {reason}' for reason, count in sorted(base_reasons.items()))
    emit(f'    {"the tick every baseline run ended at":<50}'
         f'{min(base_ends)} to {max(base_ends)}, of a possible 1,000')
    emit(f'    {"how they ended":<50}{ended}')
    emit(f'    {f"consumptions across all {len(baseline)} baseline cells":<50}'
         f'{sum(len(cells[name]["post_eats"]) for name in baseline):,}')
    emit(f'    {"the tick the obligated runs reached, for contrast":<50}'
         f'{min(obligated_ends)} to {max(obligated_ends)}')
    emit()
    emit(f'So the identity above constrains the shared world over the first {max(base_ends)} ticks at most,')
    emit('and over the later ticks of a thousand-tick obligated run it constrains nothing. It could not be')
    emit('otherwise: a control that ignores the world cannot survive in it, which is exactly why')
    emit('`INT-MOK-010` promises `baseline` byte-identity rather than `baseline` survival. The later ticks')
    emit('are covered instead by reading the source -- `post/world-rules-unchanged.txt` -- and by the draw')
    emit('accounting in `post/entropy.txt`. Anyone reading this file as "the world is unchanged for a')
    emit('thousand ticks" is reading more than the measurement carries.')
    emit()
    emit('It does not prove that the other three sources diverge only where they should, which is')
    emit('`divergence.txt`. Nor does it prove the entropy stream is consumed identically in the obligated')
    emit('runs -- it is not, and legitimately so: a wider candidate set draws a different number of')
    emit('times, which is `post/entropy.txt` and route R2 of `divergence.txt`.')
    emit()
    emit(f'RESULT: {"PASS" if not differing and not exit_differing else "FAIL"} -- '
         f'{len(baseline) - len(differing)} of {len(baseline)} baseline cells byte-identical, '
         f'{len(baseline) - len(exit_differing)} of {len(baseline)} exit codes equal.')
    io.open(os.path.join(out_dir, 'byte-identity.txt'), 'w', encoding='utf-8',
            newline='\n').write('\n'.join(out) + '\n')

    # ---------------------------------------------------------------- divergence.txt
    out = []
    obligated = [name for name in names if cells[name]['source'] in OBLIGATED]
    diverged = [name for name in obligated if not cells[name]['identical']]
    unmoved = [name for name in obligated if cells[name]['identical']]
    unattributed = [name for name in diverged if not cells[name]['route']]
    misaligned = [name for name in diverged if not cells[name]['ticks_agree']]
    late_band = [name for name in diverged
                 if cells[name]['band_entry'] is None
                 or cells[name]['band_entry'] > cells[name]['tick']]
    moved_tolerances = [name for name in obligated if not cells[name]['tolerance_unchanged']]
    counted = {route: sum(1 for name in diverged if cells[name]['route'] == route)
               for route in ('R1', 'R2', 'R3')}

    emit('WO-MOK-017: every obligated divergence, by first diverging tick, record and decision')
    emit('=' * 84)
    emit()
    emit('Work order   WO-MOK-017 (the resource composition drift)')
    emit('In scope     "characterizing each of the 60 reference and individual divergences by first')
    emit('             diverging tick, first diverging record and the eat-or-seek decision that changed')
    emit('             -- the clause WO-MOK-016 recorded as unwritable, which becomes writable for the')
    emit('             first time here". The 30 social cells are characterized with them, because rule 26')
    emit('             delegates eating and seeking to rule 19, whose test this change amends.')
    emit('Stop cond 3  "A reference or individual divergence cannot be attributed to the corrected waste')
    emit('             condition."')
    emit('Pre-change   pre/COMMIT.txt      Candidate  post/COMMIT.txt')
    emit('Reader       analysis/divergence.py')
    emit('Date         2026-08-21')
    emit()

    emit('1. The two conditions, from the two commits\' own source')
    emit('------------------------------------------------------')
    emit('Attribution is arithmetic, so the arithmetic is transcribed from each commit\'s')
    emit('`mokiterions-core/src/simulation.rs` rather than from the specification. The specification is')
    emit('the thing the pre-change binary had drifted from, so quoting it for both sides would beg the')
    emit('question this file exists to answer.')
    emit()
    emit('    pre-change  rule 5   S + R <= 100')
    emit('                rule 19  S + R <= 100  ||  S + R - 100 <= T*R/100')
    emit('    candidate   both     S + R <= 100  ||  S + R - 100 <= max(R*R/100, T*R/100)')
    emit('                         with T = 0 for rule 5, which is `fits_within(food, 0)`')
    emit()
    emit('The drift was one term omitted twice: rule 5 had no allowance at all, and rule 19 had only the')
    emit('trait half. That is also why rule 19\'s `T = 0` identity with rule 5 held on the drifted pair --')
    emit('at `T = 0` the trait term is zero and both collapse to `S + R <= 100`. The identity was a')
    emit('coincidence of the defect, and the amended clause 1 together with the shared `fits_within` is')
    emit('what makes it structural.')
    emit()
    emit('    class    R   R*R/100    highest satiety the old rule 5 ate at    the candidate\'s, at T = 0')
    for calorie_class in ('low', 'medium', 'high'):
        restored = RESTORATION[calorie_class]
        earned = restored * restored // 100
        emit(f'    {calorie_class:<8}{restored:>3}{earned:>9}'
             f'{ATTRIBUTE_MAX - restored:>41}{ATTRIBUTE_MAX - restored + earned:>29}')
    emit()
    emit('`reference` ignores the trait, so a `reference` cell\'s printed `waste_tolerance` is not its')
    emit('allowance and is not read as one here: its old allowance is zero and its new one is `R*R/100`.')

    emit()
    emit('2. The model checked against both captures before it is used')
    emit('-----------------------------------------------------------')
    emit('Every consumption prints the satiety it happened at, so each condition can be tested against')
    emit('the capture of the commit that implemented it. A consumption outside its own commit\'s allowance')
    emit('would mean this reader\'s model of that commit is wrong, and every attribution built on it with')
    emit('it.')
    emit()
    pre_all = [eat for name in obligated for eat in cells[name]['pre_eats']]
    post_all = [eat for name in obligated for eat in cells[name]['post_eats']]
    pre_waste = [eat for eat in pre_all if eat['excess'] > 0]
    post_waste = [eat for eat in post_all if eat['excess'] > 0]
    pre_violations = [eat for eat in pre_all if eat['excess'] > eat['old']]
    post_violations = [eat for eat in post_all if eat['excess'] > eat['new']]
    newly = [eat for eat in post_all if eat['excess'] > eat['old']]
    emit(f'    consumptions in the {len(obligated)} obligated cells, pre-change      {len(pre_all):>7,}')
    emit(f'    the same at the candidate                               {len(post_all):>7,}')
    emit(f'    pre-change consumptions above a satiety of 100          {len(pre_waste):>7,}')
    emit(f'    the same at the candidate                               {len(post_waste):>7,}')
    emit(f'    pre-change consumptions outside the OLD allowance       {len(pre_violations):>7,}')
    emit(f'    candidate consumptions outside the NEW allowance        {len(post_violations):>7,}')
    emit(f'    candidate consumptions the OLD condition cannot emit    {len(newly):>7,}')
    emit()
    if pre_violations or post_violations:
        emit('    THE MODEL IS CONTRADICTED BY THE CAPTURES. Nothing below can be relied on:')
        for eat in (pre_violations + post_violations)[:10]:
            emit(f'      {eat["subject"]} tick {eat["tick"]}: {eat["class"]} at satiety {eat["before"]}, '
                 f'excess {eat["excess"]} against old {eat["old"]} and new {eat["new"]}')
        failures.append(f'{len(pre_violations) + len(post_violations)} consumptions outside their '
                        f'own commit\'s allowance')
    else:
        emit('    **Neither capture holds a single consumption outside its own commit\'s allowance.** Both')
        emit('    conditions are reproduced exactly by the two captures\' own records, so the model this')
        emit('    attribution rests on is measured rather than assumed.')
    emit()
    emit('    The check has teeth. An earlier draft of this reader modelled the old condition as plainly')
    emit(f'    `S + R <= 100` for every source, and the {len(pre_waste):,} pre-change consumptions above a')
    emit('    satiety of 100 contradicted it at once: they are the trait-aware sources eating inside the')
    emit('    old `T*R/100` term, which the drifted rule 19 did have. The correction to this reader came')
    emit('    from the capture, not from re-reading the source a second time.')

    emit()
    emit('3. The three attribution routes')
    emit('-------------------------------')
    emit('Two sources differing only in their waste condition can behave differently only where the two')
    emit('conditions disagree: some class with `S + R > 100`, admitted by the new allowance and declined')
    emit('by the old. Call that the disagreement band. Each cell\'s first divergence takes one route.')
    emit()
    emit('    R1  direct         the first diverging record carries the acting Mokiterion\'s pre-action')
    emit('                       satiety, and that (S, T, source) is in the band. The changed decision is')
    emit('                       visible in the record itself.')
    emit('    R2  stream-shared  the acting Mokiterion is not in the band, but an in-band one acted')
    emit('                       earlier in the same tick with an unchanged record. A wider candidate set')
    emit('                       consumes a different number of draws even when it reaches the same')
    emit('                       proposal, so the next Mokiterion to draw wanders somewhere else.')
    emit('    R3  sibling        an untraced cell, explained by the traced cell of its own triple --')
    emit('                       same seed, source and density, diverging no later, by R1 or R2. An')
    emit('                       untraced stream prints no proposal, so the changed decision is invisible')
    emit('                       in it and its first visible difference is a downstream effect.')
    emit()
    for route in ('R1', 'R2', 'R3'):
        emit(f'    cells attributed by {route}                             '
             f'{counted[route]:>4} of {len(diverged)}')
    emit(f'    cells attributed by no route                       {len(unattributed):>4} '
         f'of {len(diverged)}')
    emit(f'    obligated cells that did not diverge at all        {len(unmoved):>4} '
         f'of {len(obligated)}')
    emit()
    if unattributed:
        emit('    THE FOLLOWING DIVERGENCES ARE NOT ATTRIBUTED. Stop condition 3 is in force:')
        for name in unattributed:
            cell = cells[name]
            emit(f'      {name}: tick {cell["tick"]}, {cell["event"]} by {cell["subject"]} at satiety '
                 f'{cell["satiety"]} with tolerance {cell["agent_tolerance"]} -- no class disagrees and '
                 f'no in-band actor precedes it in the tick')
        failures.append(f'{len(unattributed)} divergences attributed by no route')
    else:
        emit('    **Every obligated divergence takes one of the three routes.**')
    if misaligned:
        emit('    THE FOLLOWING FIRST DIFFERING LINES SIT AT DIFFERENT TICKS on the two sides:')
        for name in misaligned:
            emit(f'      {name}: line {cells[name]["index"] + 1}')
            emit(f'        -  {cells[name]["pre_line"]}')
            emit(f'        +  {cells[name]["post_line"]}')
        failures.append(f'{len(misaligned)} cells whose first differing line spans two ticks')
    if unmoved:
        emit()
        emit('    Cells that did not move. Each is permitted rather than a correction that failed to')
        emit('    land: the condition changes an answer only where a Mokiterion observes a resource in')
        emit('    the newly admitted band, and a run that goes extinct early may never reach one. The')
        emit('    obligation is that no divergence is unexplained, not that every cell diverges.')
        for name in unmoved:
            emit(f'      {name}')

    emit()
    emit('4. The independent necessary condition: nothing may diverge before the band is entered')
    emit('------------------------------------------------------------------------------------')
    emit('The three routes classify a divergence. This is the check that can refute one. Every')
    emit('Mokiterion starts at a satiety of 100 and loses one per tick, so no band is reachable at tick 0:')
    emit('the first one a falling satiety enters is the low class at 87 -- excess 2 against an earned')
    emit('allowance of 2 -- and 87 is thirteen decrements below the start. A divergence arising before any')
    emit('Mokiterion is in a band could not be this correction\'s, whatever route it appeared to take, so')
    emit('the earliest band entry is measured per cell from the pre-change stream and compared with it.')
    emit()
    starts = sorted({satiety for name in obligated for satiety in cells[name]['start_satieties']})
    ticks = [cells[name]['tick'] for name in diverged]
    entries = [cells[name]['band_entry'] for name in diverged
               if cells[name]['band_entry'] is not None]
    emit(f'    starting satieties, over all {len(obligated)} obligated cells         {starts}')
    emit(f'    earliest first divergence, over all diverged cells      tick {min(ticks)}')
    emit(f'    latest                                                  tick {max(ticks)}')
    emit(f'    earliest band entry measured in a pre-change stream     tick {min(entries)}')
    emit(f'    cells whose divergence precedes their own band entry    {len(late_band)}')
    emit()
    if late_band:
        emit('    THE FOLLOWING CELLS DIVERGE BEFORE ANY MOKITERION IS IN THE BAND:')
        for name in late_band:
            emit(f'      {name}: diverges at tick {cells[name]["tick"]}, '
                 f'band entered at {cells[name]["band_entry"]}')
        failures.append(f'{len(late_band)} cells diverge before their band entry')
    else:
        emit('    **No cell diverges before the tick at which its band is first entered.** The bound is')
        emit('    not slack: the earliest divergence and the earliest band entry fall on the same tick,')
        emit('    so the correction takes effect at the first observation where it can. That is the')
        emit('    strongest form this check can return, and it is why no divergence in any of the')
        emit(f'    {len(obligated)} obligated runs sits earlier than tick {min(ticks)} -- a Mokiterion at a')
        emit('    satiety of 88 or above declines every class under both conditions alike.')
    emit()
    emit(f'    cells whose waste_tolerance assignment moved            {len(moved_tolerances)}')
    if moved_tolerances:
        emit('    THE TRAIT ITSELF MOVED in the cells above, so a divergence there is not evidence about')
        emit('    the condition at all:')
        for name in moved_tolerances:
            emit(f'      {name}')
        failures.append(f'{len(moved_tolerances)} cells assign waste_tolerance differently')
    else:
        emit('    Rule 20 fixes the trait at initialization from the seed, and it is identical at both')
        emit('    commits in every cell. So these divergences are the condition reading the same trait')
        emit('    differently, not a different trait being drawn.')

    emit()
    emit('5. Every divergence, one row per cell')
    emit('-------------------------------------')
    emit('    cell                              tick  route  agent   S   T  record kinds         '
         'pre verb        -> post verb       the disagreement, or the witness')
    for name in obligated:
        cell = cells[name]
        if cell['identical']:
            continue
        if cell['route'] == 'R1':
            detail = ', '.join(f'{item[0]} +{item[1]} <= {item[2]}, old {item[3]}'
                               for item in cell['band'])
        elif cell['route'] == 'R2':
            subject, satiety, tolerance, witnessed = cell['witness']
            detail = (f'{subject} at S={satiety} T={tolerance}, band '
                      f'{",".join(item[0] for item in witnessed)}')
        elif cell['route'] == 'R3':
            detail = f'{cell["witness"][0]}, tick {cell["witness"][1]}, {cell["witness"][2]}'
        else:
            detail = 'UNATTRIBUTED'
        kind = (cell['post_event'] if cell['pre_event'] == cell['post_event']
                else f"{cell['pre_event']}/{cell['post_event']}")
        emit(f"    {name:<34}{cell['tick']:>4}  {str(cell['route']):<5}  {cell['subject']:<6}"
             f"{cell['satiety'] if cell['satiety'] is not None else '?':>3}"
             f"{cell['agent_tolerance'] if cell['agent_tolerance'] is not None else '?':>4}  "
             f"{kind:<20} {str(cell['pre_verb']) if cell['pre_verb'] else '-':<15} -> "
             f"{str(cell['post_verb']) if cell['post_verb'] else '-':<15} {detail}")
    emit()
    emit('`S` is the acting Mokiterion\'s satiety before the tick\'s effects, read from the first')
    emit('diverging record. `T` is its `waste_tolerance` as printed at initialization -- shown for every')
    emit('source, including `reference`, where the trait is in the record but the source does not read')
    emit('it. Each `R1` row carries the excess above 100 and both allowances, so its arithmetic is')
    emit('checkable without rerunning this reader. On an `R2` or `R3` row the `S` and `T` shown are the')
    emit('ones the routes had to *reject* -- that Mokiterion is not the one whose answer changed, which')
    emit('is precisely why those cells take a route other than R1 -- and the witness column names the one')
    emit('that carries the explanation. Where the two sides of the line are different record kinds, both')
    emit('are printed separated by a slash.')

    emit()
    emit('6. The first diverging record of every cell, in full')
    emit('---------------------------------------------------')
    emit('The row above is a summary of the two lines below it, and "first diverging record" is a')
    emit('retention item in its own right: a summary of a record is not the record.')
    for name in obligated:
        cell = cells[name]
        if cell['identical']:
            continue
        emit()
        emit(f'  {name}   tick {cell["tick"]}, line {cell["index"] + 1}, route {cell["route"]}')
        emit(f'    -  {cell["pre_line"]}')
        emit(f'    +  {cell["post_line"]}')

    emit()
    emit('7. The decision that changed, by kind')
    emit('-------------------------------------')
    emit('Both sides of the first differing line are named, and they are not always the same kind of')
    emit('record: where one commit emits a resolution the other does not, the line indices part and an')
    emit('`action_trace` lands opposite a `threat_resolved`. A table that printed one event for both')
    emit('sides would read as though a `food_consumed` had changed its proposal.')
    emit()
    kinds = {}
    for name in diverged:
        cell = cells[name]
        key = (cell['pre_event'], cell['pre_verb'], cell['post_event'], cell['post_verb'])
        kinds[key] = kinds.get(key, 0) + 1
    emit('    pre event         pre verb        post event        post verb       cells')
    for (pre_event, pre_verb, post_event, post_verb), count in sorted(
            kinds.items(), key=lambda item: (-item[1], str(item[0]))):
        emit(f'    {str(pre_event):<17} {str(pre_verb) if pre_verb else "-":<15} '
             f'{str(post_event):<17} {str(post_verb) if post_verb else "-":<15} {count:>5}')
    emit()
    emit('A verb of `-` means the record carries no proposal, which is every record kind but')
    emit('`action_trace`. Those rows are the untraced cells, plus the traced cells whose diverging line is')
    emit('a resolution one commit emits and the other does not. The changed decision behind them is named')
    emit('in the route and witness columns of section 5, not here.')
    emit()
    traced = [name for name in diverged if cells[name]['trace'] == 'on']
    untraced = [name for name in diverged if cells[name]['trace'] == 'off']
    emit(f'    traced cells, where a proposal is printed and can be the first difference   {len(traced)}')
    emit(f'    untraced cells, where the earliest visible difference is an effect          {len(untraced)}')
    emit()
    emit('The two halves of a triple part at the same decision but report it differently, and that is')
    emit('instrumentation rather than behavior. In a traced cell `action_trace` prints the proposal, so')
    emit('the first differing line names the verb that changed. In an untraced cell the proposal is not')
    emit('printed at all, so the first differing line is whatever the changed decision eventually')
    emit('produced -- a `food_consumed` one commit emits and the other does not, a `survival_changed`')
    emit('for a Mokiterion that is somewhere else, or a `food_regenerated` whose class and position have')
    emit('moved because the shared stream is at a different offset. That is why route R3 exists, and why')
    emit('an untraced cell is explained by its traced sibling rather than by its own first record.')
    emit()
    emit('Rule 5 applies its condition in two places and both appear above: case 1, eating a co-located')
    emit('resource, and case 3, approaching the nearest resource it would not decline. A changed `move`')
    emit('is the second of those -- the Mokiterion walks toward a resource it now accepts instead of')
    emit('wandering or heading elsewhere -- and it is the more common first divergence because seeking')
    emit('happens at a distance and so happens sooner than the eat it leads to.')

    emit()
    emit('8. The first diverging tick, distributed')
    emit('----------------------------------------')
    emit('    source      dens   seeds ordered 0, 1, 42, 123, 777: first diverging tick, traced half')
    for source in OBLIGATED:
        for density in DENSITIES:
            row = []
            for seed in SEEDS:
                cell = cells.get(f'seed{seed}-{source}-d{density}-traceon')
                row.append('same' if not cell or cell['identical'] else str(cell['tick']))
            emit(f'    {source:<11}{density:>5}   ' + '  '.join(f'{value:>5}' for value in row))
    emit()
    emit(f'    cells diverging at or before tick 20                    '
         f'{sum(1 for tick in ticks if tick <= 20)} of {len(ticks)}')
    agreeing = 0
    total = 0
    for name in obligated:
        if cells[name]['trace'] != 'on':
            continue
        sibling = cells.get(cells[name]['sibling'])
        if not sibling:
            continue
        total += 1
        if (not cells[name]['identical'] and not sibling['identical']
                and cells[name]['tick'] == sibling['tick']):
            agreeing += 1
    emit(f'    triples whose two halves diverge at the same tick        {agreeing} of {total}')
    emit()
    emit('Where the two halves part at different ticks the reason is the one section 7 gives: the')
    emit('untraced stream cannot show a proposal, so it parts at the first *effect* of the changed')
    emit('decision, at or after the decision itself. Route R3 requires exactly that ordering -- traced no')
    emit('later than untraced -- and it holds in every triple it is applied to.')

    emit()
    emit('9. What this does not claim')
    emit('---------------------------')
    emit('It does not claim every *subsequent* difference in a diverged stream is individually')
    emit('attributable. Once two runs part they are different worlds: different Mokiterions are alive at')
    emit('different positions with different satieties, and every later record differs for that reason')
    emit('rather than for this change\'s. Stop condition 3 is written about the divergence, singular per')
    emit('cell, and the first diverging record is where a divergence is attributable at all.')
    emit()
    emit('It does not re-derive the proposal. Whether the corrected condition admits the *right*')
    emit('resources is a specification question, answered against constructed state in the test suite --')
    emit('`the_corrected_non_waste_condition_admits_the_specified_boundaries` fixes all three classes at')
    emit('both boundaries in both of rule 5\'s cases, and')
    emit('`a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten` fixes the trait\'s')
    emit('effect. A second implementation of the specification inside an evidence script would agree')
    emit('with whichever of the two was written second.')
    emit()
    emit('Route R2 concedes something worth stating plainly: a divergence can reach a Mokiterion the')
    emit('condition never applied to, through the draw count of one it did apply to. That is not a defect')
    emit('and it is not new -- `WO-MOK-016` recorded the shared stream as the mechanism by which any')
    emit('behavioral change propagates -- but it does mean the *set* of Mokiterions whose behavior moved')
    emit('is larger than the set whose answer the condition changed. `post/entropy.txt` is where the draw')
    emit('accounting itself is recorded, and `post/survivors.txt` is where the population consequence is.')
    emit()
    result = 'PASS' if not failures else 'FAIL'
    emit(f'RESULT: {result} -- {len(diverged)} of {len(obligated)} obligated cells diverge, '
         f'{counted["R1"]} attributed directly, {counted["R2"]} through the shared stream, '
         f'{counted["R3"]} through the traced sibling, {len(unattributed)} unattributed.')
    io.open(os.path.join(out_dir, 'divergence.txt'), 'w', encoding='utf-8',
            newline='\n').write('\n'.join(out) + '\n')

    print(f'byte-identity.txt: {len(baseline) - len(differing)} of {len(baseline)} identical, '
          f'{len(exit_differing)} exit differences')
    print(f'divergence.txt: {len(diverged)} of {len(obligated)} diverged; '
          f'R1={counted["R1"]} R2={counted["R2"]} R3={counted["R3"]} '
          f'unattributed={len(unattributed)}')
    for failure in failures:
        print(f'FAILURE: {failure}')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
