"""WO-MOK-017: which identifier dies first, and whether it has living identifiers on both sides.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/dead-neighbours.py \
        <pre-capture-dir> <post-capture-dir>

Writes its report to stdout and exits `0` when every check passes, non-zero otherwise.

WHY A CORE WORK ORDER MEASURES A TUI TEST'S SCENARIO
---------------------------------------------------
`a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour` in
`mokiterions-tui/src/state.rs` asserts rule 10.6 in both directions: from a dead selection, Tab reaches
the next living identifier *above* it and BackTab the next living identifier *below* it. Both assertions
need a dead Mokiterion with a living identifier on each side. The pre-change scenario advanced to the
first death and used that, which worked only because the first identifier to die happened to be an
interior one. The test discharges two cases in `VER-MOK-005`: `REQ-MOK-021`'s "Selected Mokiterion dies"
and `REQ-MOK-023`'s "Selection cycles living Mokiterions only".

`REQ-MOK-060` moved which identifier dies first. Changing a test to make it pass is what
`SPEC-MOK-002` rule 12 forbids, so the amendment has to be shown to be a change of *scenario search*
rather than a retreat from an assertion -- and that rests on a measurement: at each commit, which
identifiers die in what order, and whether the first one to die is interior. This reader takes that
measurement from the retained captures, so it is checkable by someone who does not run the terminal.

The capture cell is the run the test performs. `start(&["--policy", "reference", "--ticks", "700",
"--start-paused"])` declares no seed and no density, so it takes the defaults -- seed `0` and density
`0.75` -- which is the matrix cell `seed0-reference-d0.75`. The cell is a thousand ticks where the test
asks for seven hundred, and the first seven hundred ticks of the longer run are the whole of the
shorter one, because nothing in the engine reads the horizon except the stop condition.

WHAT INTERIORITY MEANS HERE
---------------------------
The test's own predicate is reproduced: a dead identifier `d` is usable when some **living** identifier
sorts below `d` and some living identifier sorts above it, evaluated at the moment the test looks. Both
halves matter and they fail differently. `M01` can never have a living identifier below it, so BackTab
from `M01` wraps to the top of the list; asserting `backward < dead` on a wrap is impossible, and
relaxing the assertion to admit the wrap would be the rule-12 violation. The order is the observer's
list order, which is the identifier's own lexical order, and the search takes the first usable dead
identifier in death order exactly as `observer.deaths().iter().find(...)` does.
"""

import io
import os
import re
import sys

# LF regardless of platform: `.gitattributes` pins this evidence tree `-text`.
sys.stdout.reconfigure(encoding='utf-8', newline='\n')

HORIZON = 700
CELL = 'seed0-reference-d0.75-traceoff.txt'
BASELINE_CELL = 'seed0-baseline-d0.75-traceoff.txt'

DIED = re.compile(r'^tick=(\d+) subject=(M\d+) event=agent_died')
SUBJECT = re.compile(r'^tick=\d+ subject=(M\d+) ')

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


def read_cell(path):
    """Every identifier the cell mentions, and its deaths in the order they occur."""
    identifiers, deaths = set(), []
    with io.open(path, encoding='utf-8', newline='') as handle:
        for line in handle:
            subject = SUBJECT.match(line)
            if subject:
                identifiers.add(subject.group(1))
            death = DIED.match(line)
            if death:
                deaths.append((int(death.group(1)), death.group(2)))
    return identifiers, deaths


def search(identifiers, deaths, horizon):
    """The test's loop: the first dead identifier holding living identifiers on both sides.

    Returns `(tick, subject, trail)`, where `trail` records each death in order together with
    whether the set of dead identifiers was usable once that death had landed. `subject` is `None`
    when the horizon passes without the state ever being reached, which is the assertion the
    amended test added rather than a case it leaves open.
    """
    dead, trail, found = [], [], None
    for tick, subject in deaths:
        if tick > horizon:
            break
        dead.append(subject)
        living = identifiers - set(dead)
        usable = next((candidate for candidate in dead
                       if any(other < candidate for other in living)
                       and any(other > candidate for other in living)), None)
        trail.append((tick, subject, sorted(living), usable))
        if usable and not found:
            found = (tick, usable)
    return (found[0] if found else None), (found[1] if found else None), trail


def main():
    pre_dir, post_dir = sys.argv[1:3]

    title = "WO-MOK-017: the dead identifier rule 10.6's both-directions scenario needs"
    print(title)
    print('=' * len(title))
    print()
    print('Work order   WO-MOK-017 (the resource composition drift)')
    print('Subject      mokiterions-tui/src/state.rs ::')
    print('             a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour')
    print(f'Measured on  the retained captures, at both commits, over the first {HORIZON} ticks')
    print(f'Cell         {CELL} -- the defaults the test takes')
    print('Reader       analysis/dead-neighbours.py')
    print('Date         2026-08-21')
    print()

    measured = {}
    for label, directory in (('pre-change', pre_dir), ('candidate', post_dir)):
        path = os.path.join(directory, CELL)
        if not check(os.path.exists(path), f'{CELL} is missing from the {label} capture'):
            continue
        identifiers, deaths = read_cell(path)
        check(len(identifiers) == 12,
              f'the {label} cell names {len(identifiers)} identifiers, not the expected twelve')
        measured[label] = (identifiers, deaths, search(identifiers, deaths, HORIZON))

    if len(measured) != 2:
        print('RESULT: FAIL -- the two captures were not both readable:')
        for failure in failures:
            print(f'  {failure}')
        return 1

    print('1. The death order at each commit')
    print('---------------------------------')
    print('Each row is one death, in the order the observer records it. "usable" names the first dead')
    print('identifier that holds living identifiers on both sides once that death has landed, which is')
    print("the test's own search, evaluated after every advance.")
    for label in ('pre-change', 'candidate'):
        identifiers, deaths, _ = measured[label]
        print()
        print(f'    {label}: {len(identifiers)} identifiers, '
              f'{len([1 for tick, _ in deaths if tick <= HORIZON])} deaths inside {HORIZON} ticks')
        print(f'    {"tick":>6}  {"dies":<5}  {"living after":<44}  usable')
        for tick, subject, living, usable in measured[label][2][2]:
            shown = ' '.join(living)
            print(f'    {tick:>6}  {subject:<5}  {shown:<44}  {usable or "-- none --"}')

    print()
    print('2. What moved, and why the pinned scenario stopped working')
    print('---------------------------------------------------------')
    rows = []
    for label in ('pre-change', 'candidate'):
        _, deaths, (tick, subject, trail) = measured[label]
        first_tick, first_subject = deaths[0]
        first_usable = trail[0][3] == first_subject
        rows.append((label, first_tick, first_subject, first_usable, tick, subject))
        print(f'    {label:<11} first death {first_subject} at tick {first_tick:<4} '
              f'{"interior" if first_usable else "NOT interior":<12} '
              f'search settles on {subject or "nothing"} at tick {tick if tick else "--"}')
    print()
    pre_first_usable = rows[0][3]
    post_first_usable = rows[1][3]
    check(pre_first_usable,
          'the first death was not interior before the change either, so the pinned scenario was '
          'already broken and this amendment needs a different justification')
    check(not post_first_usable,
          f'the first death at the candidate ({rows[1][2]}) is interior, so the pinned scenario '
          'would still pass and the amendment is unmotivated')
    lowest = min(measured['candidate'][0])
    check(rows[1][2] == lowest,
          f'the first death at the candidate is {rows[1][2]}, not the lowest identifier {lowest}, '
          'so the BackTab wrap is not the reason the scenario broke')
    print(f'    The first identifier to die is {rows[1][2]} at the candidate, and {rows[1][2]} is the lowest')
    print('    identifier in the list. No living identifier sorts below it, so BackTab from it wraps to')
    print('    the top rather than moving down, and `assert!(backward < dead)` cannot hold. The only way')
    print('    to keep a scenario pinned to the first death would have been to weaken that assertion to')
    print('    admit the wrap -- a different claim about a different behavior, and exactly what')
    print('    `SPEC-MOK-002` rule 12 forbids. Searching for the state rule 10.6 describes keeps both')
    print('    assertions as written.')
    print()
    for label in ('pre-change', 'candidate'):
        tick, subject, _ = measured[label][2]
        check(subject is not None and tick <= HORIZON,
              f'the {label} run reaches no usable dead identifier within {HORIZON} ticks, '
              'so the amended search would not terminate')
    print(f'    The search terminates at both commits well inside the {HORIZON}-tick horizon the test')
    print(f'    declares: tick {measured["pre-change"][2][0]} pre-change on '
          f'{measured["pre-change"][2][1]}, tick {measured["candidate"][2][0]} at the candidate on '
          f'{measured["candidate"][2][1]}. The amendment')
    print('    asserts that rather than assuming it: if the run finishes with no such state, the test')
    print('    fails saying so instead of falling out of the loop having proved nothing.')

    print()
    print('3. Why the scenario cannot simply take the baseline policy')
    print('---------------------------------------------------------')
    print("The comment's other claim -- that baseline cannot serve because it starves the whole")
    print('population at once -- is measured here too, since it is what rules out the cheapest fix.')
    for label, directory in (('pre-change', pre_dir), ('candidate', post_dir)):
        path = os.path.join(directory, BASELINE_CELL)
        if not check(os.path.exists(path), f'{BASELINE_CELL} is missing from the {label} capture'):
            continue
        identifiers, deaths = read_cell(path)
        ticks = sorted({tick for tick, _ in deaths})
        check(len(ticks) == 1 and len(deaths) == len(identifiers),
              f'the {label} baseline cell does not lose its whole population on one tick: '
              f'{len(deaths)} deaths over {len(ticks)} tick(s)')
        print(f'    {label:<11} {len(deaths)} of {len(identifiers)} identifiers die on tick '
              f'{ticks[0] if len(ticks) == 1 else ticks}')
    print()
    print('    Rule 4 applies no waste condition, so the baseline population starves together and the')
    print('    tick that produces a dead selection produces no living one. That is also why these two')
    print('    rows are identical: `INT-MOK-010` promises byte-identity for `--policy baseline`, and')
    print('    `post/byte-identity.txt` measures the promise kept across all thirty baseline cells.')

    print()
    print('4. What this does not claim')
    print('---------------------------')
    print('  * Not that the identifier is stable. It is not, and that is the point: the amended test')
    print('    searches for the state rather than naming the Mokiterion, so the next change to the')
    print('    world moves the subject without touching the test.')
    print('  * Not that a live run is the right way to reach rule 10.6\'s state. Constructing a dead')
    print('    selection with living neighbours directly would be a scenario coupled to nothing, and it')
    print(f'    would not need a {HORIZON}-tick run to find one. This test discharges two cases in')
    print('    `VER-MOK-005` -- `REQ-MOK-021`\'s "Selected Mokiterion dies" and `REQ-MOK-023`\'s')
    print('    "Selection cycles living Mokiterions only" -- and neither states how the state is')
    print('    reached, so the search is inside the scenario as written. Changing it to a constructed')
    print('    one would be a `VER-MOK-005` amendment this work order has no authority to make; raised')
    print('    in `manual-assessment.md`.')
    print('  * Not that the deaths themselves are correct. Which identifiers die and when is')
    print('    `post/survivors.txt` and `post/divergence.txt`, not this file.')

    print()
    if failures:
        print(f'RESULT: FAIL -- {len(failures)} check(s) failed:')
        for failure in failures:
            print(f'  {failure}')
    else:
        print('RESULT: PASS -- the first identifier to die was interior before the change and is the')
        print('lowest identifier at the candidate, so the pinned scenario broke on the BackTab wrap; the')
        print('search for rule 10.6\'s state terminates at both commits, and both assertions stand as')
        print('written.')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
