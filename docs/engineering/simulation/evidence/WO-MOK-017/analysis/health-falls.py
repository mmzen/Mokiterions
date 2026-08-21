"""WO-MOK-017: the health decline available to `VER-MOK-013`'s gauge scenario, at both commits.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/health-falls.py \
        <pre-capture-dir> <post-capture-dir>

Writes its report to stdout and exits `0` when every check passes, non-zero otherwise.

WHY A CORE WORK ORDER MEASURES A TUI TEST'S SCENARIO
---------------------------------------------------
`a_declining_mokiterion_shows_a_declining_bar` in `mokiterions-tui/tests/render.rs` asserts that the
health gauge tracks a declining Mokiterion. It guards itself first: if no Mokiterion still alive at tick
200 has fallen at least thirty points of health, it fails with "this scenario is unexercised" rather than
drawing a flat bar and passing. That guard fired at this candidate, and the fix was to select `social`
where the test previously took the default source.

Changing a test to make it pass is the move `SPEC-MOK-002` rule 12 exists to forbid, so the change has
to be shown to be a change of *scenario parameter* rather than a retreat from an assertion, and that
argument rests on a measurement: how much health decline actually exists at 200 ticks, per source, per
declared seed, at both commits. This reader takes that measurement from the retained captures rather
than from the TUI, which is what makes it checkable by someone who does not run the terminal.

The captures are the same runs the TUI would produce. `start(&["--seed", "42", "--ticks", "200"])` takes
the default policy and the default density, and the matrix cell `seed42-reference-d0.75` is that run at a
longer horizon -- the first 200 ticks of a 1,000-tick run are the whole of a 200-tick run, because
nothing in the engine reads the horizon except the stop condition. So the falls below are the falls the
test would see.

WHAT IS MEASURED, IN THE TEST'S OWN TERMS
-----------------------------------------
The test builds a health series per Mokiterion over 200 samples, keeps those **still alive at the end**,
and takes `first - last` as the fall. That definition is reproduced exactly, including its two
consequences: a Mokiterion that dies inside the window is excluded however far it fell, and the fall is
measured end to end rather than as the deepest dip, so a recovery masks a decline.
"""

import io
import os
import re
import sys

# LF regardless of platform: `.gitattributes` pins this evidence tree `-text`.
sys.stdout.reconfigure(encoding='utf-8', newline='\n')

TICKS = 200
GUARD = 30
SEEDS = (0, 1, 42, 123, 777)
SOURCES = ('reference', 'individual', 'social')

SURVIVAL = re.compile(r'^tick=(\d+) subject=(M\d+) event=survival_changed result=health:(\d+)->(\d+)')
DIED = re.compile(r'^tick=(\d+) subject=(M\d+) event=agent_died')

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


def falls(path):
    """Per Mokiterion alive at tick `TICKS`, its health at the first and last sample in the window."""
    first, last, died = {}, {}, {}
    with io.open(path, encoding='utf-8', newline='') as handle:
        for line in handle:
            match = SURVIVAL.match(line)
            if match:
                tick, subject, before, after = match.groups()
                if int(tick) > TICKS:
                    break
                first.setdefault(subject, int(before))
                last[subject] = int(after)
                continue
            death = DIED.match(line)
            if death and int(death.group(1)) <= TICKS:
                died[death.group(2)] = int(death.group(1))
    return {subject: (first[subject], last[subject]) for subject in first if subject not in died}, died


def main():
    pre_dir, post_dir = sys.argv[1:3]

    title = "WO-MOK-017: the health decline VER-MOK-013's gauge scenario has to work with"
    print(title)
    print('=' * len(title))
    print()
    print('Work order   WO-MOK-017 (the resource composition drift)')
    print('Subject      mokiterions-tui/tests/render.rs ::')
    print('             a_declining_mokiterion_shows_a_declining_bar')
    print('Measured on  the retained captures, at both commits, over the first 200 ticks')
    print('Reader       analysis/health-falls.py')
    print('Date         2026-08-21')
    print()
    print("The test's own definition of a fall is reproduced: the health of each Mokiterion **still")
    print('alive at tick 200**, first sample minus last. Its guard fails the run when the deepest such')
    print(f'fall is under {GUARD}, so the guard column below is the whole question.')
    print()
    print('    source      seed   deepest fall     living at 200    deaths inside      guard of 30')
    print('                       pre -> cand.     pre -> cand.     pre -> cand.       pre / cand.')
    table = {}
    for source in SOURCES:
        for seed in SEEDS:
            name = f'seed{seed}-{source}-d0.75-traceoff.txt'
            measured = {}
            for label, directory in (('pre', pre_dir), ('post', post_dir)):
                path = os.path.join(directory, name)
                if not check(os.path.exists(path), f'{name} is missing from the {label} capture'):
                    continue
                series, died = falls(path)
                deepest = max((start - end for start, end in series.values()), default=0)
                measured[label] = (deepest, len(series), len(died))
            if len(measured) != 2:
                continue
            table[(source, seed)] = measured
            (deep_pre, live_pre, dead_pre) = measured['pre']
            (deep_post, live_post, dead_post) = measured['post']
            verdict = (f'{"pass" if deep_pre >= GUARD else "FAILS":<5}/ '
                       f'{"pass" if deep_post >= GUARD else "FAILS"}')
            print(f'    {source:<11}{seed:>5}   {deep_pre:>4} ->{deep_post:>5}     '
                  f'{live_pre:>4} ->{live_post:>5}     {dead_pre:>4} ->{dead_post:>5}       {verdict}')
    print()
    for source in SOURCES:
        worst_pre = max(table[(source, seed)]['pre'][0] for seed in SEEDS)
        worst_post = max(table[(source, seed)]['post'][0] for seed in SEEDS)
        print(f'    deepest fall on any declared seed under `{source}`'
              f'{"":<{max(0, 12 - len(source))}}  pre-change {worst_pre:>3}, '
              f'candidate {worst_post:>3}')
    print()

    starving = [source for source in ('reference', 'individual')
                if max(table[(source, seed)]['post'][0] for seed in SEEDS) < GUARD]
    check(set(starving) == {'reference', 'individual'},
          'the guard does not fail under reference and individual at the candidate, '
          'so the scenario change needs a different justification')
    social_pre = table[('social', 42)]['pre'][0]
    social_post = table[('social', 42)]['post'][0]
    check(social_pre >= GUARD and social_post >= GUARD,
          f'social seed 42 does not clear the guard at both commits: {social_pre}, {social_post}')

    print('What the table establishes, in the order the argument needs it:')
    print()
    print('  1. **The guard genuinely fired.** At the candidate, no declared seed under `reference` or')
    print('     `individual` produces any fall at all among the Mokiterions alive at tick 200. The test')
    print('     could not have been left as it was: it would fail with "this scenario is unexercised",')
    print('     which is the guard doing its job rather than a defect.')
    print('  2. **`REQ-MOK-060` is why.** Health falls only once satiety reaches zero, so the corrected')
    print('     waste condition -- which feeds Mokiterions sooner -- removes the decline at this horizon.')
    print(f'     Seed 42 under `reference` fell {table[("reference", 42)]["pre"][0]} points before the')
    print(f'     correction and {table[("reference", 42)]["post"][0]} after it.')
    print('  3. **`social` is not a weaker scenario, and it is not a longer one either.** There the')
    print(f'     decline is combat damage rather than starvation: seed 42 falls {social_pre} points')
    print(f'     pre-change and {social_post} at the candidate. Selecting it decouples the test from the')
    print('     nutrition model that has now broken it twice, which is why it was preferred to raising')
    print('     the horizon until starvation reappears.')
    print('  4. **The guard and the assertions are untouched.** The threshold stays at thirty and both')
    print('     properties -- a strictly lower fill at the end, and fill monotone in health across every')
    print('     sample -- are asserted exactly as before. `VER-MOK-013` states the scenario as "200')
    print('     ticks at a declared seed at the reference viewport" and names no decision source, so the')
    print('     source is a parameter the scenario leaves open.')
    print()
    print('What this does not claim: that `social` will exercise the gauge forever. It is coupled to the')
    print('combat model instead of the nutrition model, which is a different dependency and not none. A')
    print('scenario that constructs a declining subject directly, rather than searching a live run for')
    print('one, is the durable answer, and it needs a `VER-MOK-013` amendment this work order has no')
    print('authority to make. Raised in `manual-assessment.md` for the technical owner.')

    print()
    if failures:
        print(f'RESULT: FAIL -- {len(failures)} check(s) failed:')
        for failure in failures:
            print(f'  {failure}')
    else:
        print('RESULT: PASS -- the guard fails under `reference` and `individual` at the candidate on')
        print('every declared seed, and `social` seed 42 clears it at both commits, so the scenario')
        print('change is a parameter selection made necessary by the correction rather than a retreat')
        print('from an assertion.')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
