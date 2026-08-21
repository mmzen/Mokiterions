"""VER-MOK-012 oracle 4: the entropy state per tick, and the same state against the pre-change build.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-018/analysis/entropy.py \
        <evidence-dir> <output-file>

Inputs, both retained beside the output:

  * `entropy-states.txt`     -- the raw capture of the two state tests: the state after
                               initialization per declared seed and swept density, and the state at
                               tick 1,000 per seed, density and policy, with and without a sink;
  * `entropy-per-tick.txt`   -- the raw capture of the boundary-by-boundary comparison, 4,388 rows;
  * `oracle1/*.txt`          -- the three byte comparisons, for the result lines quoted in Part 3.

## Why this artifact is assembled and not simply captured

`VER-MOK-012` asks `entropy.txt` for two things: the per-tick state comparison with and without a
sink, and "the state after initialization and at tick 1,000 **against the pre-change build**". The
first is a capture. The second cannot be, and that is the whole difficulty of this artifact.

`Simulation::entropy_state` is `#[cfg(test)]` and was added by this work order. The pre-change build
at `de33d744` has no such accessor, so no figure of this kind can be read out of it. Adding the
accessor to the pre-change tree would produce a different build, and oracle 1's own rule is that the
baseline is captured once and never recaptured to resolve a discrepancy. So the pre-change
comparison is made by consequence rather than by reading, and Part 3 states the argument and the
residual it leaves. Part 4 supplies what makes that argument tight: an independent recovery of the
**draw count** behind every state figure in the artifact.

## What Part 4 computes, and why it is worth computing

`SplitMix64::next_u64` adds one fixed odd constant to the state per draw and derives the returned
value from the state afterwards, so the state is a draw counter:

    state after k draws = (seed + k * 0x9E3779B97F4A7C15) mod 2**64

The constant is odd, so it is invertible modulo 2**64, and the draw count comes back out of any
recorded state by one multiplication. This script inverts it by modular inversion, in Python,
without running the engine.

Two things follow, and neither is a restatement of what the tests already assert:

  * every recorded state is checked to be the seed advanced by a *small non-negative integer* number
    of draws. A state that did not come from this generator seeded from this seed would invert to an
    essentially uniform 64-bit number: fifteen recorded states all inverting into a window of 2**20
    is a fact about the engine, not an artifact of the arithmetic;
  * "the states are equal" becomes "the draw counts are equal", which is the property the pre-change
    argument in Part 3 needs, because every value the engine draws is a function of the state before
    the draw.

The derivation also settles a question the captures alone leave open. Boundary 0 is taken after
`Simulation::new` and before the initialization events are emitted, and it is **not** the seed:
construction places resources and agents, and those placements draw. Part 4 reports how many draws
construction took at each seed and density, and shows that boundary 0 and boundary 1 have the same
count -- emitting the initialization events draws nothing, because those events report placements
that construction already made.
"""

import io
import os
import re
import sys

GAMMA = 0x9E37_79B9_7F4A_7C15  # simulation.rs:877, the one constant next_u64 adds per draw
MASK = (1 << 64) - 1
WINDOW = 1 << 20  # the bound every recovered draw count is required to fall under

# The three constants construction's draw count is a closed form of, quoted from the engine rather
# than guessed: 128 * 64 cells per territory (simulation.rs:25), twelve agents (simulation.rs:1483),
# and two draws per placement attempt (simulation.rs:2517-2518).
CELLS_PER_TERRITORY = 128 * 64
AGENTS = 12

COMMIT = 'bb4a21491eff321cbfd14ba3ea794e34535e3033'
BASELINE = 'de33d744'
DATE = '2026-08-20'  # a literal, so that re-running this script does not restamp the artifact

INIT = re.compile(
    r'^seed=(\d+) density=([\d.]+) after_initialization=0x([0-9a-f]+) '
    r'combinations=(\d+) all_equal=(\S+)$'
)
FINAL = re.compile(
    r'^seed=(\d+) density=([\d.]+) policy=(\S+) ticks=(\d+) reason=(\S+) '
    r'nosink=0x([0-9a-f]+) sink=0x([0-9a-f]+) (\S+)$'
)
BOUNDARY = re.compile(
    r'^seed=(\d+) density=([\d.]+) policy=(\S+) trace=(\S+) boundary=(\d+) '
    r'nosink=0x([0-9a-f]+) sink=0x([0-9a-f]+) (\S+)$'
)

PRE_CHANGE = """\
The obligation is the state after initialization and at tick 1,000 *against the pre-change build*.
It cannot be discharged by reading, and this is why.

`Simulation::entropy_state` is `#[cfg(test)]` and this work order added it. The pre-change build at
{baseline} does not have it, and no accessor means no figure. The two ways of getting one are both
closed:

  * adding the accessor to the pre-change tree makes a different build, and the figure would then
    describe that build rather than the one oracle 1's baseline was captured from;
  * recapturing the baseline is what oracle 1's first obligation forbids: "Oracle 1's baseline is
    captured before the first code change, from a tracked-clean worktree, with the commit recorded,
    and is never recaptured to resolve a discrepancy."

So the comparison is made by consequence. The argument has four steps, and Part 4 supplies the one
that is not obvious:

  1. **The state is a draw counter.** Part 4 recovers, for every figure in Parts 1 and 2, a
     non-negative integer k such that the state is the seed advanced k times. Two builds therefore
     hold the same state at a point in a run exactly when they have taken the same number of draws
     by that point. Nothing else can produce an equal state.
  2. **Every drawn value is a function of the state before the draw.** `next_u64` advances the state
     and derives its return value from the state it advanced to. So if two builds' draw counts
     differ by even one at some point in a run, every value drawn after that point differs.
  3. **Every draw the engine takes reaches the output.** Draws are taken to place a resource, to
     place an agent, and to decide a move. Each of those is reported: a placement by an
     initialization event, a decision by the movement or the action-trace event it produces. There
     is no draw whose result is discarded. `choose_index` can loop -- it rejects values above its
     modulo threshold -- but a rejection changes the draw count, and by step 2 that shifts every
     value after it.
  4. **Oracle 1 finds no difference.** The pre-change text stream is compared with the post-change
     one, byte for byte, over all ninety declared cells, without a sink and with one:

{oracle1}

     Those comparisons cover five seeds, three policies, three densities and both tracing settings,
     and the streams they compare run to thousands of events each.

Put together: if the pre-change build had taken a different number of draws at any point that
precedes an emitted event, the text stream would differ there and oracle 1 would have found it.
It found nothing in ninety cells. So the pre-change draw counts agree with Parts 1 and 2's at every
point that reaches the output, and by step 1 the states agree there too.

**The residual, stated rather than left to be noticed.** The argument covers draw counts up to the
last draw whose value reaches the output. It does not cover a draw taken *after* that -- a trailing
draw at the very end of a run, whose result nothing consumes. Such a draw would be invisible to
oracle 1 and would change Part 2's tick-1,000 figure, so Part 2's figures are established against
the pre-change build only up to a trailing draw. Two things bound that residual and neither closes
it:

  * step 3 above: no shipped call site takes a draw and discards its value, so a trailing draw would
    have to be a call that this work order removed, and this work order removed no call site --
    `interface.txt` compares the two revisions item for item;
  * `a_record_sink_moves_no_entropy_draw_at_any_tick_boundary` closes the same question for the sink,
    in both directions, at every boundary. That is the comparison this change could plausibly break.
    The pre-change comparison is the one that has to rest on consequence.

This is recorded as a limitation of the evidence, not as a passing check. `VER-MOK-012`'s residual
uncertainty section is where it belongs, and `completion-summary.md` restates it.\
"""


def resources_per_territory(density):
    """`Density::resources_per_territory` (simulation.rs:121), in integers, from the printed text."""
    whole, _, fraction = density.partition('.')
    hundredths = int(whole or 0) * 100 + int(f'{fraction:<02s}'.replace(' ', '0') or 0)
    return hundredths * CELLS_PER_TERRITORY // 10_000


def draws(state, seed):
    """How many times the seed was advanced to reach `state`. Exact, by modular inversion."""
    return ((state - seed) * pow(GAMMA, -1, 1 << 64)) & MASK


def section(text, marker):
    """(everything before the line that starts with `marker`, that line and everything after)."""
    lines = text.split('\n')
    for index, line in enumerate(lines):
        if line.startswith(marker):
            return lines[:index], lines[index:]
    raise SystemExit(f'the marker {marker!r} is not in the capture')


def oracle1_results(evidence):
    """The result lines of the three byte comparisons, quoted rather than restated."""
    quoted = []
    for name in ('pre-vs-post-nosink', 'pre-vs-post-sink', 'post-nosink-vs-post-sink'):
        path = os.path.join(evidence, 'oracle1', f'{name}.txt')
        lines = io.open(path, encoding='utf-8').read().split('\n')
        title = next(line for line in lines if line.startswith('# oracle 1'))
        counted = next(line for line in lines if line.startswith('# ') and 'cells compared' in line)
        result = next(line for line in lines if line.startswith('# result'))
        quoted.append(
            f'     oracle1/{name}.txt\n'
            f'       {title.lstrip("# ")}\n'
            f'       {counted.lstrip("# ")}, {result.lstrip("# ")}'
        )
    return '\n\n'.join(quoted)


def main():
    evidence, output_file = sys.argv[1:3]
    states_path = os.path.join(evidence, 'entropy-states.txt')
    per_tick_path = os.path.join(evidence, 'entropy-per-tick.txt')
    states_text = io.open(states_path, encoding='utf-8').read()
    per_tick_text = io.open(per_tick_path, encoding='utf-8').read()

    part1, part2 = section(states_text, '# Part 2 --')
    failures = []

    # -----------------------------------------------------------------------------------------------
    # Parse.
    # -----------------------------------------------------------------------------------------------
    initial = []
    for line in part1:
        match = INIT.match(line)
        if match:
            initial.append(
                (int(match.group(1)), match.group(2), int(match.group(3), 16),
                 int(match.group(4)), match.group(5))
            )
    final = []
    for line in part2:
        match = FINAL.match(line)
        if match:
            final.append(
                (int(match.group(1)), match.group(2), match.group(3), int(match.group(4)),
                 match.group(5), int(match.group(6), 16), int(match.group(7), 16), match.group(8))
            )
    boundaries = []
    for line in per_tick_text.split('\n'):
        match = BOUNDARY.match(line)
        if match:
            boundaries.append(
                (int(match.group(1)), match.group(2), match.group(3), match.group(4),
                 int(match.group(5)), int(match.group(6), 16), int(match.group(7), 16),
                 match.group(8))
            )
    if not initial or not final or not boundaries:
        raise SystemExit('a capture parsed to no rows: the print format has changed')

    lines = [
        '# VER-MOK-012 oracle 4: the entropy state, per tick and against the pre-change build',
        '#',
        '# work order:  WO-MOK-018',
        f'# commit:      {COMMIT} (working tree, WO-MOK-018 implementation)',
        f'# baseline:    {BASELINE} (oracle 1\'s pre-change capture)',
        f'# date:        {DATE}',
        '# command:     python docs/engineering/simulation/evidence/WO-MOK-018/analysis/'
        'entropy.py \\',
        '#                <evidence-dir> <output-file>',
        f'# inputs:      entropy-states.txt, entropy-per-tick.txt, oracle1/*.txt',
        '#',
        '# This is the artifact WO-MOK-018 declares as `entropy.txt`. It carries five parts:',
        '#',
        '#   Part 1  the state after initialization, per declared seed and swept density',
        '#   Part 2  the state at tick 1,000, per seed, density and policy, with and without a sink',
        '#   Part 3  the same figures against the pre-change build, and the residual that leaves',
        '#   Part 4  an independent recovery of the draw count behind every figure above',
        '#   Part 5  the per-tick comparison: every boundary of every series, summarised',
        '#',
        '# Parts 1 and 2 are the raw test output, embedded verbatim from entropy-states.txt so that',
        '# this artifact reads on its own. Part 5 summarises the 4,388-row boundary table and does',
        '# not reproduce it: the table is retained whole beside this file as entropy-per-tick.txt,',
        '# and Part 5 accounts for every one of its rows.',
        '',
        '=' * 108,
        'Part 1 -- the state after initialization',
        '=' * 108,
        '',
    ]
    lines.extend(part1)
    lines.append('=' * 108)
    lines.append('Part 2 -- the state at the thousandth tick')
    lines.append('=' * 108)
    lines.append('')
    lines.extend(part2)

    # -----------------------------------------------------------------------------------------------
    # Part 3 -- the pre-change comparison. Prose, with the oracle 1 result lines quoted from file.
    # -----------------------------------------------------------------------------------------------
    lines.append('=' * 108)
    lines.append('Part 3 -- against the pre-change build')
    lines.append('=' * 108)
    lines.append('')
    lines.extend(
        PRE_CHANGE.format(baseline=BASELINE, oracle1=oracle1_results(evidence)).split('\n')
    )
    lines.append('')

    # -----------------------------------------------------------------------------------------------
    # Part 4 -- the draw counts, recovered by inverting the generator's increment.
    # -----------------------------------------------------------------------------------------------
    inverse = pow(GAMMA, -1, 1 << 64)
    lines.append('=' * 108)
    lines.append('Part 4 -- the draw count behind every state above')
    lines.append('=' * 108)
    lines.append('')
    lines.append('  the generator, quoted from the source it is implemented in:')
    lines.append('')
    lines.append('    simulation.rs:872   fn new(seed: u64) -> Self { Self { state: seed } }')
    lines.append('    simulation.rs:877   self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15)')
    lines.append('')
    lines.append('  so  state after k draws = (seed + k * gamma) mod 2**64,  with gamma odd and')
    lines.append(f'  therefore invertible:  gamma     = 0x{GAMMA:016x}')
    lines.append(f'                         gamma**-1 = 0x{inverse:016x}   (computed here, by')
    lines.append('                                     modular inversion, without running the engine)')
    lines.append(
        f'    gamma * gamma**-1 mod 2**64 == 1: '
        f'{"yes" if (GAMMA * inverse) & MASK == 1 else "NO"}'
    )
    if (GAMMA * inverse) & MASK != 1:
        failures.append('the recovered inverse is not the inverse')
    lines.append('')
    lines.append('  k = ((state - seed) * gamma**-1) mod 2**64, and every k below is required to be')
    lines.append(f'  a non-negative integer under {WINDOW:,} (2**20). A state that did not come from')
    lines.append('  this generator seeded from this seed inverts to an essentially uniform 64-bit')
    lines.append('  number, so each row landing in that window is a fact about the engine.')
    lines.append('')
    lines.append('  Part 1, the state after initialization -- these are construction\'s draws:')
    lines.append('')
    lines.append(f'    {"seed":>6} {"density":>9} {"state":>20} {"draws":>9}')
    initial_draws = {}
    for seed, density, state, combinations, all_equal in initial:
        count = draws(state, seed)
        initial_draws[(seed, density)] = count
        lines.append(f'    {seed:>6} {density:>9} {f"0x{state:016x}":>20} {count:>9,}')
        if count >= WINDOW:
            failures.append(
                f'seed {seed} density {density}: the recovered draw count {count} is not under 2**20'
            )
        if all_equal != 'yes' or combinations != 12:
            failures.append(f'seed {seed} density {density}: {combinations} combinations, '
                            f'all_equal={all_equal}')
    lines.append('')
    lines.append(
        f'    every recovered count is a non-negative integer under 2**20: '
        f'{"yes" if not failures else "NO"}'
    )
    lines.append(
        f'    largest count over the {len(initial)} rows: '
        f'{max(initial_draws.values()):,} draws'
    )
    lines.append('')

    # Draws should grow with the density, because capacity is resolved from it and every placement
    # draws. This is a prediction the model makes and the figures can refuse.
    lines.append('  what the counts say about the density, per seed:')
    lines.append('')
    densities = sorted({density for _, density, _, _, _ in initial}, key=float)
    seeds = sorted({seed for seed, _, _, _, _ in initial})
    header = ''.join(f'{f"d={density}":>14}' for density in densities)
    lines.append(f'    {"seed":>6}{header}   monotone in the density')
    monotone_all = True
    for seed in seeds:
        counts = [initial_draws[(seed, density)] for density in densities]
        monotone = all(a < b for a, b in zip(counts, counts[1:]))
        monotone_all = monotone_all and monotone
        cells = ''.join(f'{count:>14,}' for count in counts)
        lines.append(f'    {seed:>6}{cells}   {"yes" if monotone else "NO"}')
    lines.append('')
    lines.append(
        f'    strictly increasing with the density at every seed: '
        f'{"yes" if monotone_all else "NO"}'
    )
    lines.append('    A placement draws, and the capacity a density resolves to is how many placements')
    lines.append('    there are, so this is the direction the counts had to move. It is reported')
    lines.append('    because a check that only ever confirms is worth little: the figures were free')
    lines.append('    to refuse it.')
    if not monotone_all:
        failures.append('a draw count does not increase with the density')
    lines.append('')
    lines.append('  every one of construction\'s draws, accounted for:')
    lines.append('')
    lines.append('    `choose_free_coordinate` (simulation.rs:2511) takes exactly two draws per')
    lines.append('    attempt -- a column from WORLD_SIZE and a row from TERRITORY_HEIGHT -- and')
    lines.append('    attempts again when the coordinate it lands on is already occupied. It is called')
    lines.append('    once per resource and once per agent, and nothing else in `Simulation::new`')
    lines.append('    draws: the resource class cycles rather than being drawn (simulation.rs:1463)')
    lines.append('    and the name is a table lookup (simulation.rs:1494). So')
    lines.append('')
    lines.append('      draws = 2 * (placements + retries),   placements = 2 * per_territory + agents')
    lines.append('')
    lines.append(f'    with per_territory = hundredths_of_percent * {CELLS_PER_TERRITORY} / 10000')
    lines.append(f'    (simulation.rs:121, integer division) and agents = {AGENTS}')
    lines.append('    (simulation.rs:1483). That closes the count, so the retries come back out of it:')
    lines.append('')
    lines.append(
        f'      {"density":>9} {"per territory":>14} {"placements":>11} {"draws if none":>14}'
        + ''.join(f'{f"seed {seed}":>10}' for seed in seeds)
    )
    for density in densities:
        placements = 2 * resources_per_territory(density) + AGENTS
        floor = 2 * placements
        retries = []
        for seed in seeds:
            count = initial_draws[(seed, density)]
            surplus = count - floor
            if surplus < 0 or surplus % 2:
                failures.append(
                    f'seed {seed} density {density}: {count} draws is not {floor} plus an even '
                    f'number of retries'
                )
            retries.append(surplus // 2)
        lines.append(
            f'      {density:>9} {resources_per_territory(density):>14} {placements:>11} '
            f'{floor:>14}' + ''.join(f'{f"+{retry}":>10}' for retry in retries)
        )
    lines.append('')
    lines.append('    The last five columns are retries, not residuals: every count is the closed-form')
    lines.append('    figure plus twice a small non-negative integer. Two consequences worth stating.')
    lines.append('    Retries rise with the density, which is what a fuller territory does. And every')
    lines.append('    count being even means `choose_index` never rejected a draw in any of these runs:')
    lines.append('    a rejection adds one draw and a retry adds two, so a rejection would show as an')
    lines.append('    odd count. Its threshold is 2**64 mod the bound, which is why.')
    lines.append('')
    lines.append('  Part 2, the state at tick 1,000 -- construction\'s draws plus the run\'s:')
    lines.append('')
    lines.append(
        f'    {"seed":>6} {"density":>9} {"policy":>11} {"ticks":>6} {"reason":>11} '
        f'{"draws":>10} {"of which the run":>17} {"sink":>6}'
    )
    for seed, density, policy, ticks, reason, nosink, sink, verdict in final:
        count = draws(nosink, seed)
        sink_count = draws(sink, seed)
        run = count - initial_draws[(seed, density)]
        lines.append(
            f'    {seed:>6} {density:>9} {policy:>11} {ticks:>6} {reason:>11} '
            f'{count:>10,} {run:>17,} {"equal" if sink_count == count else "DIFFERS":>6}'
        )
        if sink_count != count:
            failures.append(f'seed {seed} density {density} policy {policy}: the sink run took '
                            f'{sink_count} draws against {count} without one')
        if verdict != 'equal':
            failures.append(f'seed {seed} density {density} policy {policy}: capture says {verdict}')
        if run < 0:
            failures.append(f'seed {seed} density {density} policy {policy}: the run took {run} '
                            f'draws, which is fewer than none')
        if count >= WINDOW:
            failures.append(f'seed {seed} density {density} policy {policy}: the recovered draw '
                            f'count {count} is not under 2**20')
    lines.append('')
    lines.append(f'    {len(final)} rows: five declared seeds x three swept densities x three')
    lines.append('    policies. The run column is the count minus Part 1\'s for the same seed and')
    lines.append('    density, so it is the draws the ticks themselves took -- which is why it varies')
    lines.append('    with the policy while Part 1 does not.')
    lines.append('')

    # -----------------------------------------------------------------------------------------------
    # Part 5 -- the per-tick table, summarised, with every row accounted for.
    # -----------------------------------------------------------------------------------------------
    lines.append('=' * 108)
    lines.append('Part 5 -- the per-tick comparison, every boundary of every series')
    lines.append('=' * 108)
    lines.append('')
    lines.append('  Retained whole in entropy-per-tick.txt. Every row of it is accounted for below:')
    lines.append('  each series is one configuration run twice, once with a sink and once without,')
    lines.append('  with the state compared at every tick boundary. Boundary 0 is after')
    lines.append('  `Simulation::new` and before the initialization events are emitted; boundary 1 is')
    lines.append('  after they are; boundary n+1 is after tick n. The final boundary is where the run')
    lines.append('  ended, so a series that died out is shorter than one that reached the tick limit.')
    lines.append('')
    lines.append(
        f'    {"seed":>6} {"density":>8} {"policy":>11} {"trace":>6} {"rows":>5} '
        f'{"k(0)":>8} {"k(1)":>8} {"k(last)":>9} {"draws/tick":>12} {"all equal":>10}'
    )
    series = {}
    for seed, density, policy, trace, boundary, nosink, sink, verdict in boundaries:
        series.setdefault((seed, density, policy, trace), []).append(
            (boundary, nosink, sink, verdict)
        )
    equal_rows, differing_rows, monotone_series, zero_at_initialization = 0, 0, 0, 0
    for key in sorted(series, key=lambda key: (key[0], key[1], key[2], key[3])):
        seed, density, policy, trace = key
        rows = sorted(series[key])
        counts = [draws(nosink, seed) for _, nosink, _, _ in rows]
        for boundary, nosink, sink, verdict in rows:
            if nosink == sink and verdict == 'equal':
                equal_rows += 1
            else:
                differing_rows += 1
                failures.append(f'{key} boundary {boundary}: {verdict}')
            if draws(sink, seed) != draws(nosink, seed):
                failures.append(f'{key} boundary {boundary}: draw counts differ')
        monotone = all(a <= b for a, b in zip(counts, counts[1:]))
        monotone_series += 1 if monotone else 0
        if not monotone:
            failures.append(f'{key}: the draw count goes backwards')
        if counts[0] == counts[1]:
            zero_at_initialization += 1
        else:
            failures.append(
                f'{key}: emitting the initialization events took {counts[1] - counts[0]} draws'
            )
        deltas = [b - a for a, b in zip(counts[1:], counts[2:])]
        span = f'{min(deltas)}..{max(deltas)}' if deltas else '-'
        expected = initial_draws.get((seed, density))
        if expected is not None and counts[1] != expected:
            failures.append(
                f'{key}: boundary 1 is {counts[1]} draws against Part 1\'s {expected}'
            )
        lines.append(
            f'    {seed:>6} {density:>8} {policy:>11} {str(trace):>6} {len(rows):>5} '
            f'{counts[0]:>8,} {counts[1]:>8,} {counts[-1]:>9,} {span:>12} '
            f'{"yes" if monotone else "NO":>10}'
        )
    lines.append('')
    lines.append(f'    series: {len(series)}   (five declared seeds x three policies x tracing off')
    lines.append('            and on, at the default density, 150 ticks)')
    lines.append(f'    rows:   {len(boundaries):,}   of which {equal_rows:,} equal and '
                 f'{differing_rows} differing')
    lines.append('')
    lines.append('  Three things this table establishes that the equality alone does not:')
    lines.append('')
    lines.append(
        f'    * the draw count never goes backwards, in {monotone_series} of {len(series)} series. It'
    )
    lines.append('      could not, if the state is a counter -- so this is a check on the model in')
    lines.append('      Part 4 as much as on the engine;')
    lines.append(
        f'    * k(0) == k(1) in {zero_at_initialization} of {len(series)} series: emitting the'
    )
    lines.append('      initialization events draws nothing. Those events report placements that')
    lines.append('      `Simulation::new` already made, and boundary 0 is therefore not the seed but')
    lines.append('      the seed already advanced by construction\'s placements;')
    lines.append('    * boundary 1 agrees with Part 1\'s figure for the same seed and density in')
    lines.append('      every series, which is a cross-check between two separately captured tests.')
    lines.append('')
    lines.append('  The draws/tick column is the range of per-tick draw counts within the series,')
    lines.append('  reported as a range because a mean would be a floating-point figure and this')
    lines.append('  packet keeps to integers.')
    lines.append('')
    lines.append(f'# result: {"FAIL" if failures else "PASS"}')
    if failures:
        lines.append('')
        for failure in failures[:40]:
            lines.append(f'  {failure}')
        if len(failures) > 40:
            lines.append(f'  ... and {len(failures) - 40} more')
    lines.append('')
    lines.append('# ---- full text of this script, retained as VER-MOK-012 requires ----')
    lines.append('')
    lines.extend(io.open(__file__, encoding='utf-8').read().split('\n'))

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(
        f'{len(initial)} initialization rows, {len(final)} tick-1,000 rows, '
        f'{len(boundaries):,} boundary rows over {len(series)} series, {len(failures)} failing; '
        f'written to {output_file}'
    )
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
