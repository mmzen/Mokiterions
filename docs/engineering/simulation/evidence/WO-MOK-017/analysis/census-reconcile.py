"""WO-MOK-017: the pre-change and post-change test census, reconciled name by name.

Usage, from **this packet's own directory** rather than from the repository root:

    cd docs/engineering/simulation/evidence/WO-MOK-017
    python analysis/census-reconcile.py pre/test-census.txt post/test-census.txt \
        > post/test-census-reconciliation.md

Writes its report to stdout and exits `0` when every check passes, non-zero otherwise.

The working directory matters here and nowhere else in `analysis/`, because this reader prints the two
paths it was given in its own header and a reader is entitled to reproduce the file byte for byte. The
committed report carries the two short paths above, so it is those two arguments that produce it.

WHAT THIS FILE IS FOR
---------------------
`WO-MOK-017` retains "the pre-change and post-change workspace test census, reconciled name by name,
with every retracted test named". The reconciliation is mechanical and so is done mechanically: the two
censuses are read as sets of `(target, name)` pairs and the symmetric difference is reported in full. A
count that matches proves nothing on its own -- one test removed and one added reads as `267 -> 268` too
-- so what is reported is the membership, and the counts are a consequence of it.

Three things are checked rather than stated:

  * **Nothing was removed and nothing renamed.** A rename presents as one addition and one removal, so
    the removal set being empty settles both. Each addition is nevertheless matched against every
    removal by string similarity, so that a rename could not hide behind a coincidence of counts.
  * **Nothing was silenced.** Every row's outcome is read, and an `ignored` row is a failure of this
    reconciliation whatever the suite's exit code was. `SPEC-MOK-002` rule 12 forbids weakening an
    assertion to move a test; `#[ignore]` is the strongest possible weakening and the easiest to miss.
  * **Every amended test kept its name.** The six bodies this work order amended are listed below with
    the diff's own line counts, and each is required to appear in *both* censuses. A body amended to the
    point of being renamed is a retraction wearing the old name's clothes, and this is what would catch
    it.

WHAT THIS FILE DOES NOT DO
--------------------------
It does not read the tests. That an amended body still asserts what it asserted before is not a property
of a census, and it is argued test by test in `post/updated-tests.md`, with the two observer-side
amendments measured against the retained captures in `post/health-falls.txt` and
`post/dead-neighbours.txt`. This reader knows only that the names are what they were.

ON THE CENSUS READER ITSELF
---------------------------
`analysis/test-census.py`, which produced both inputs, is byte-identical to `WO-MOK-016`'s copy, so the
two censuses are comparable by construction and the comparison is not made by a reader written after the
fact to suit it. Its docstring's remark that "three cases fail at this candidate and they appear here as
`FAILED`" is true of `WO-MOK-016`'s candidate and false of this one, where nothing fails; the file is
left untouched rather than corrected, because its byte-identity is the provenance and a corrected copy
would be a different reader. The remark is corrected here instead, which is where a reader of *this*
packet looks.
"""

import collections
import difflib
import io
import re
import sys

# LF regardless of platform: `.gitattributes` pins this evidence tree `-text`.
sys.stdout.reconfigure(encoding='utf-8', newline='\n')

HEADER = re.compile(r'^# (\d+) test names; reported totals: (\d+) passed, (\d+) failed, (\d+) ignored')
ROW = re.compile(r'^(.+?) :: (.+?) :: (.+)$')

# The added test, and what it is for. Required to be in the post census and absent from the pre.
ADDED = ('unittests', 'simulation::tests::the_corrected_non_waste_condition_admits_the_specified_boundaries')

# The amended bodies, with the diff's own `+added / -removed` counts. Each is required to appear in
# both censuses under exactly this name.
AMENDED = (
    ('unittests', 'simulation::tests::the_reference_source_does_not_consume_a_resource_it_does_not_need',
     5, 3, 'the satiety the scenario is built at moved from 51 to 76'),
    ('unittests', 'simulation::tests::the_reference_source_does_not_approach_a_resource_it_would_decline',
     3, 2, 'the same satiety, in rule 5\'s case 3'),
    ('unittests',
     'simulation::tests::at_tolerance_zero_the_trait_aware_source_proposes_what_the_reference_source_'
     'proposes',
     11, 5, 'its enumerated satiety set no longer straddled the corrected boundaries'),
    ('unittests', 'simulation::tests::a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten',
     49, 18, 'one of rule 19\'s two worked pairs stopped being a pair'),
    ('unittests', 'state::tests::a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour',
     35, 10, 'the first Mokiterion to die is now the lowest identifier'),
    ('tests/render.rs', 'a_declining_mokiterion_shows_a_declining_bar',
     18, 2, 'no starvation-driven health decline is left at 200 ticks'),
)

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


def read_census(path):
    """The census's declared header figures, and its rows as `{(target, name): outcome}`."""
    declared, rows = None, {}
    with io.open(path, encoding='utf-8', newline='') as handle:
        for line in handle:
            header = HEADER.match(line.rstrip('\n'))
            if header:
                declared = tuple(int(group) for group in header.groups())
                continue
            if line.startswith('#') or not line.strip():
                continue
            row = ROW.match(line.rstrip('\n'))
            if row:
                rows[(row.group(1), row.group(2))] = row.group(3)
    return declared, rows


def main():
    pre_path, post_path = sys.argv[1:3]
    (pre_declared, pre_rows) = read_census(pre_path)
    (post_declared, post_rows) = read_census(post_path)

    print('# Test census reconciliation — WO-MOK-017')
    print()
    print('```')
    print('Work order   WO-MOK-017 (the resource composition drift)')
    print('Retains      "the pre-change and post-change workspace test census, reconciled name by')
    print('             name, with every retracted test named"')
    print(f'Pre-change   {pre_path}')
    print(f'Candidate    {post_path}')
    print('Census by    analysis/test-census.py, byte-identical to WO-MOK-016\'s copy')
    print('Reconciled   analysis/census-reconcile.py')
    print('Date         2026-08-21')
    print('```')
    print()

    print('## The two censuses')
    print()
    print('| | names | passed | failed | ignored |')
    print('|---|---|---|---|---|')
    for label, declared, rows in (('pre-change', pre_declared, pre_rows),
                                  ('candidate', post_declared, post_rows)):
        if not check(declared is not None, f'the {label} census carries no header line'):
            continue
        names, passed, failed, ignored = declared
        check(names == len(rows),
              f'the {label} census declares {names} names and carries {len(rows)} rows')
        check(failed == 0, f'the {label} census reports {failed} failing test(s)')
        check(ignored == 0, f'the {label} census reports {ignored} ignored test(s)')
        print(f'| {label} | {names} | {passed} | {failed} | {ignored} |')
    print()
    outcomes = collections.Counter(list(pre_rows.values()) + list(post_rows.values()))
    check(set(outcomes) == {'ok'},
          f'the censuses carry outcomes other than ok: {sorted(set(outcomes) - {"ok"})}')
    print(f'Every row in both censuses reads `ok`: {outcomes["ok"]} of {outcomes.total()} across the two.')
    print('An `ignored` row would fail this reconciliation whatever the suite\'s exit code was, because')
    print('`#[ignore]` is the strongest way to weaken an assertion and the easiest one to miss in a green')
    print('run.')
    print()

    added = sorted(set(post_rows) - set(pre_rows))
    removed = sorted(set(pre_rows) - set(post_rows))
    print('## The symmetric difference, in full')
    print()
    print(f'    names present at the candidate and not before   {len(added)}')
    print(f'    names present before and not at the candidate   {len(removed)}')
    print()
    for label, names in (('Added', added), ('Removed', removed)):
        print(f'**{label}:**')
        print()
        if not names:
            print('  * none.')
        for target, name in names:
            print(f'  * `{name}` in `{target}`')
        print()

    check(len(added) == 1 and added[0] == ADDED,
          f'the added set is {added}, not exactly the one expected addition')
    check(not removed, f'{len(removed)} test(s) were removed: {removed}')
    check(len(post_rows) - len(pre_rows) == 1,
          f'the census grew by {len(post_rows) - len(pre_rows)}, not by one')

    print('No name disappeared, so **no test was retracted and no test was renamed** — a rename presents')
    print('as one addition and one removal, and the removal set is empty. The additions are matched')
    print('against the removals by string similarity anyway, so that a rename could not hide behind a')
    print('coincidence of counts:')
    print()
    if removed:
        for target, name in added:
            best = max(removed, key=lambda gone: difflib.SequenceMatcher(None, name, gone[1]).ratio())
            ratio = difflib.SequenceMatcher(None, name, best[1]).ratio()
            print(f'  * `{name}` against `{best[1]}`: {ratio:.2f}')
    else:
        print('  * there is nothing to match against. The one addition is a new test and not a renamed')
        print('    one, and the reconciliation does not depend on a similarity threshold.')
    print()

    print('## Per target, so that one growth is visible against every other target holding still')
    print()
    pre_by_target = collections.Counter(target for target, _ in pre_rows)
    post_by_target = collections.Counter(target for target, _ in post_rows)
    print('| target | pre-change | candidate | change |')
    print('|---|---|---|---|')
    moved = 0
    for target in sorted(set(pre_by_target) | set(post_by_target)):
        before, after = pre_by_target[target], post_by_target[target]
        if before != after:
            moved += 1
        print(f'| `{target}` | {before} | {after} | '
              f'{"**+" + str(after - before) + "**" if before != after else "—"} |')
    check(moved == 1, f'{moved} targets changed size, not one')
    print()
    print(f'One target of {len(set(pre_by_target) | set(post_by_target))} changed size, by one name.')
    print('`unittests` aggregates the unit-test binaries of both crates and the module path')
    print('distinguishes them, which')
    print('is why the engine addition and the observer amendment appear under one target here and are')
    print('separated by crate in `post/updated-tests.md`.')
    print()

    print('## The one addition')
    print()
    print(f'`{ADDED[1]}`')
    print()
    print('in `mokiterions-core`\'s test module. It is the test `REQ-MOK-060` did not have: the corrected')
    print('non-waste condition asserted at every boundary `SPEC-MOK-001`\'s amendment of 2026-08-21 states,')
    print('on both sides of each, in both of rule 5\'s cases, from literals transcribed out of the')
    print('specification rather than computed from the engine. `post/updated-tests.md` sets out what it')
    print('asserts and why each part of it is there.')
    print()

    print('## The six amended bodies')
    print()
    print('Line counts are the diff\'s own, `+added / -removed`. Every one of these names appears in both')
    print('censuses, which is asserted here: an amendment that went as far as a rename would be a')
    print('retraction wearing the old name, and it would show up as an addition and a removal above.')
    print()
    print('| test | target | lines | forced by |')
    print('|---|---|---|---|')
    for target, name, plus, minus, reason in AMENDED:
        present = (target, name) in pre_rows and (target, name) in post_rows
        check(present, f'the amended test {name} is not in both censuses')
        short = name.rsplit('::', 1)[-1]
        print(f'| `{short}`{"" if present else " **MISSING**"} | `{target}` | `+{plus} / -{minus}` '
              f'| {reason} |')
    print()
    total_plus = sum(plus for _, _, plus, _, _ in AMENDED)
    total_minus = sum(minus for _, _, _, minus, _ in AMENDED)
    print(f'`+{total_plus} / -{total_minus}` across the six, against `+63 / -20` for the engine change')
    print('itself and `+123` for the added test and `+14` for its test-support helper. Those figures')
    print('reconcile the three changed files\' totals exactly: `+268 / -48` in')
    print('`mokiterions-core/src/simulation.rs`, `+35 / -10` in `mokiterions-tui/src/state.rs` and')
    print('`+18 / -2` in `mokiterions-tui/tests/render.rs`.')
    print()
    print('One asserted constant moved upward inside the sixth of those bodies, and it is named here')
    print('because a moved constant is exactly what `SPEC-MOK-002` rule 12 is about. The identity test')
    print('enumerates every situation it can construct and asserts the size of the set it built:')
    print('`2_808` became `4_536` because its satiety list grew from thirteen values to twenty-one, so')
    print('that it straddles the corrected boundaries `87`, `79` and `75` as it straddled the uncorrected')
    print('`85`, `70` and `50` — which it keeps, as the place a regression to the omitted allowance would')
    print('first show. The assertion is strictly stronger afterwards: every situation it checked before is')
    print('still checked, under the same claim, and 1,728 more are.')
    print()
    print('Two assertions were retracted, both inside amended bodies and both replaced by stronger ones:')
    print('the high-class tolerance pair at satiety `70`, which the corrected condition made false, and')
    print('the dead-selection scenario\'s "the scenario needs a living neighbour", which passed while the')
    print('precondition it stood for was violated. `post/updated-tests.md` names each with what replaced')
    print('it and why the replacement is stronger rather than equivalent. **Apart from those two and the')
    print('enumeration size above, no assertion in the suite changed.**')
    print()
    print('In the specification, one *Acceptance examples* entry moved with the first of those two: the')
    print('high-class resource at satiety `70`, which the uncorrected condition declined under `reference`')
    print('and admitted under `individual` only at `T = 40`. `SPEC-MOK-001`\'s amendment of 2026-08-21')
    print('corrected it rather than restating it, and says so in its own amendment record. Nothing else was')
    print('retracted there: the amendment preserved rule 19\'s `T = 0` proposal identity, so the identity')
    print('sentence, the `waste_tolerance` `0` entry and the test asserting them all stand, and')
    print('`REQ-MOK-033` needed no amendment row.')
    print()

    print('## What this reconciliation does not settle')
    print()
    print('  * **That an amended body still asserts what it asserted before.** That is not a property of')
    print('    a census. It is argued test by test in `post/updated-tests.md`, and for the two')
    print('    observer-side amendments it is measured from the retained captures at both commits in')
    print('    `post/health-falls.txt` and `post/dead-neighbours.txt`.')
    print('  * **That the suite covers `REQ-MOK-060`.** Coverage is `VER-MOK-016`\'s five rows for the')
    print('    requirement, and the mapping to them is the completion report\'s, not this file\'s.')
    print('  * **The census reader\'s own docstring.** `analysis/test-census.py` is byte-identical to')
    print('    `WO-MOK-016`\'s copy, deliberately, so that the two censuses are produced by one reader and')
    print('    the comparison above is not made by a tool written afterwards to suit it. Its remark that')
    print('    "three cases fail at this candidate and they appear here as `FAILED`" describes')
    print('    `WO-MOK-016`\'s candidate. At this one nothing fails, as both header lines above record.')
    print('    The file is left untouched rather than corrected, because the byte-identity is the')
    print('    provenance and a corrected copy would be a different reader.')

    print()
    if failures:
        print(f'**RESULT: FAIL** — {len(failures)} check(s) failed:')
        print()
        for failure in failures:
            print(f'  * {failure}')
    else:
        print(f'**RESULT: PASS** — {len(pre_rows)} names became {len(post_rows)}: one added, none removed,')
        print('none renamed, none ignored, and every amended body still carries its own name.')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
