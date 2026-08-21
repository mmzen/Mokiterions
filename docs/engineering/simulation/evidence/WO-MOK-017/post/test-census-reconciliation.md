# Test census reconciliation — WO-MOK-017

```
Work order   WO-MOK-017 (the resource composition drift)
Retains      "the pre-change and post-change workspace test census, reconciled name by
             name, with every retracted test named"
Pre-change   pre/test-census.txt
Candidate    post/test-census.txt
Census by    analysis/test-census.py, byte-identical to WO-MOK-016's copy
Reconciled   analysis/census-reconcile.py
Date         2026-08-21
```

## The two censuses

| | names | passed | failed | ignored |
|---|---|---|---|---|
| pre-change | 267 | 267 | 0 | 0 |
| candidate | 268 | 268 | 0 | 0 |

Every row in both censuses reads `ok`: 535 of 535 across the two.
An `ignored` row would fail this reconciliation whatever the suite's exit code was, because
`#[ignore]` is the strongest way to weaken an assertion and the easiest one to miss in a green
run.

## The symmetric difference, in full

    names present at the candidate and not before   1
    names present before and not at the candidate   0

**Added:**

  * `simulation::tests::the_corrected_non_waste_condition_admits_the_specified_boundaries` in `unittests`

**Removed:**

  * none.

No name disappeared, so **no test was retracted and no test was renamed** — a rename presents
as one addition and one removal, and the removal set is empty. The additions are matched
against the removals by string similarity anyway, so that a rename could not hide behind a
coincidence of counts:

  * there is nothing to match against. The one addition is a new test and not a renamed
    one, and the reconciliation does not depend on a similarity threshold.

## Per target, so that one growth is visible against every other target holding still

| target | pre-change | candidate | change |
|---|---|---|---|
| `tests/authority.rs` | 4 | 4 | — |
| `tests/cli.rs` | 15 | 15 | — |
| `tests/decisions.rs` | 3 | 3 | — |
| `tests/density.rs` | 2 | 2 | — |
| `tests/export.rs` | 7 | 7 | — |
| `tests/layout.rs` | 11 | 11 | — |
| `tests/naming.rs` | 3 | 3 | — |
| `tests/options.rs` | 8 | 8 | — |
| `tests/process.rs` | 7 | 7 | — |
| `tests/render.rs` | 22 | 22 | — |
| `tests/spatial.rs` | 7 | 7 | — |
| `tests/state.rs` | 22 | 22 | — |
| `tests/termination.rs` | 5 | 5 | — |
| `tests/verification.rs` | 22 | 22 | — |
| `tests/viability.rs` | 5 | 5 | — |
| `unittests` | 124 | 125 | **+1** |

One target of 16 changed size, by one name.
`unittests` aggregates the unit-test binaries of both crates and the module path
distinguishes them, which
is why the engine addition and the observer amendment appear under one target here and are
separated by crate in `post/updated-tests.md`.

## The one addition

`simulation::tests::the_corrected_non_waste_condition_admits_the_specified_boundaries`

in `mokiterions-core`'s test module. It is the test `REQ-MOK-060` did not have: the corrected
non-waste condition asserted at every boundary `SPEC-MOK-001`'s amendment of 2026-08-21 states,
on both sides of each, in both of rule 5's cases, from literals transcribed out of the
specification rather than computed from the engine. `post/updated-tests.md` sets out what it
asserts and why each part of it is there.

## The six amended bodies

Line counts are the diff's own, `+added / -removed`. Every one of these names appears in both
censuses, which is asserted here: an amendment that went as far as a rename would be a
retraction wearing the old name, and it would show up as an addition and a removal above.

| test | target | lines | forced by |
|---|---|---|---|
| `the_reference_source_does_not_consume_a_resource_it_does_not_need` | `unittests` | `+5 / -3` | the satiety the scenario is built at moved from 51 to 76 |
| `the_reference_source_does_not_approach_a_resource_it_would_decline` | `unittests` | `+3 / -2` | the same satiety, in rule 5's case 3 |
| `at_tolerance_zero_the_trait_aware_source_proposes_what_the_reference_source_proposes` | `unittests` | `+11 / -5` | its enumerated satiety set no longer straddled the corrected boundaries |
| `a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten` | `unittests` | `+49 / -18` | one of rule 19's two worked pairs stopped being a pair |
| `a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour` | `unittests` | `+35 / -10` | the first Mokiterion to die is now the lowest identifier |
| `a_declining_mokiterion_shows_a_declining_bar` | `tests/render.rs` | `+18 / -2` | no starvation-driven health decline is left at 200 ticks |

`+121 / -40` across the six, against `+63 / -20` for the engine change
itself and `+123` for the added test and `+14` for its test-support helper. Those figures
reconcile the three changed files' totals exactly: `+268 / -48` in
`mokiterions-core/src/simulation.rs`, `+35 / -10` in `mokiterions-tui/src/state.rs` and
`+18 / -2` in `mokiterions-tui/tests/render.rs`.

One asserted constant moved upward inside the sixth of those bodies, and it is named here
because a moved constant is exactly what `SPEC-MOK-002` rule 12 is about. The identity test
enumerates every situation it can construct and asserts the size of the set it built:
`2_808` became `4_536` because its satiety list grew from thirteen values to twenty-one, so
that it straddles the corrected boundaries `87`, `79` and `75` as it straddled the uncorrected
`85`, `70` and `50` — which it keeps, as the place a regression to the omitted allowance would
first show. The assertion is strictly stronger afterwards: every situation it checked before is
still checked, under the same claim, and 1,728 more are.

Two assertions were retracted, both inside amended bodies and both replaced by stronger ones:
the high-class tolerance pair at satiety `70`, which the corrected condition made false, and
the dead-selection scenario's "the scenario needs a living neighbour", which passed while the
precondition it stood for was violated. `post/updated-tests.md` names each with what replaced
it and why the replacement is stronger rather than equivalent. **Apart from those two and the
enumeration size above, no assertion in the suite changed.**

In the specification, one *Acceptance examples* entry moved with the first of those two: the
high-class resource at satiety `70`, which the uncorrected condition declined under `reference`
and admitted under `individual` only at `T = 40`. `SPEC-MOK-001`'s amendment of 2026-08-21
corrected it rather than restating it, and says so in its own amendment record. Nothing else was
retracted there: the amendment preserved rule 19's `T = 0` proposal identity, so the identity
sentence, the `waste_tolerance` `0` entry and the test asserting them all stand, and
`REQ-MOK-033` needed no amendment row.

## What this reconciliation does not settle

  * **That an amended body still asserts what it asserted before.** That is not a property of
    a census. It is argued test by test in `post/updated-tests.md`, and for the two
    observer-side amendments it is measured from the retained captures at both commits in
    `post/health-falls.txt` and `post/dead-neighbours.txt`.
  * **That the suite covers `REQ-MOK-060`.** Coverage is `VER-MOK-016`'s five rows for the
    requirement, and the mapping to them is the completion report's, not this file's.
  * **The census reader's own docstring.** `analysis/test-census.py` is byte-identical to
    `WO-MOK-016`'s copy, deliberately, so that the two censuses are produced by one reader and
    the comparison above is not made by a tool written afterwards to suit it. Its remark that
    "three cases fail at this candidate and they appear here as `FAILED`" describes
    `WO-MOK-016`'s candidate. At this one nothing fails, as both header lines above record.
    The file is left untouched rather than corrected, because the byte-identity is the
    provenance and a corrected copy would be a different reader.

**RESULT: PASS** — 267 names became 268: one added, none removed,
none renamed, none ignored, and every amended body still carries its own name.
