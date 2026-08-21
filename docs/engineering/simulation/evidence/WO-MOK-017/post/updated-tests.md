# Updated tests — WO-MOK-017

```
Work order   WO-MOK-017 (the resource composition drift)
Implements   REQ-MOK-060; SPEC-MOK-001 rules 5 and 19 as amended 2026-08-21
Covers       every test whose body this work order changed, and the one it added
Measured by  post/health-falls.txt, post/dead-neighbours.txt
Reconciled   post/test-census-reconciliation.md (name by name), post/test-run.txt
Date         2026-08-21
```

## What this document is for

`SPEC-MOK-002` rule 12 forbids weakening an assertion in order to move a test. That rule is what makes a
green suite worth anything after a change to the world, and it is also the rule a change like this one is
most likely to break quietly: the corrected non-waste condition moves which resources are eaten, so it
moves the satieties every scenario built around "one point above the level that still fits" was written
at, and it moves which Mokiterion starves and when.

Six test bodies changed and one test was added. This document takes each one and says what forced it,
what it asserts now, and why that is at least as strong as what it asserted before. Two of the six are
in the observer rather than the engine, and their justifications rest on measurements rather than on
reasoning — those measurements are `post/health-falls.txt` and `post/dead-neighbours.txt`, taken from the
retained captures at both commits, and they are the reason this document exists as evidence rather than
as an argument.

The name-by-name reconciliation of the census — 267 names before, 268 after, nothing removed, nothing
renamed, nothing ignored — is `post/test-census-reconciliation.md`. This document does not repeat it.

## The whole inventory

Line counts are the diff's own, `+added / -removed`, and they sum to the three files' totals: `+268 / -48`
in `mokiterions-core/src/simulation.rs`, of which `+63 / -20` is the engine change and the rest is below;
`+35 / -10` in `mokiterions-tui/src/state.rs`; `+18 / -2` in `mokiterions-tui/tests/render.rs`.

| # | Test | Target | Kind | Forced by |
|---|---|---|---|---|
| 1 | `the_corrected_non_waste_condition_admits_the_specified_boundaries` | `mokiterions-core` unit | **added**, `+123` | `REQ-MOK-060`: the new condition had no test of its own |
| 2 | `the_reference_source_does_not_consume_a_resource_it_does_not_need` | `mokiterions-core` unit | amended, `+5 / -3` | the satiety it is built at moved from `51` to `76` |
| 3 | `the_reference_source_does_not_approach_a_resource_it_would_decline` | `mokiterions-core` unit | amended, `+3 / -2` | the same satiety, in rule 5's case 3 |
| 4 | `at_tolerance_zero_the_trait_aware_source_proposes_what_the_reference_source_proposes` | `mokiterions-core` unit | amended, `+11 / -5` | its enumerated satiety set no longer straddled the corrected boundaries |
| 5 | `a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten` | `mokiterions-core` unit | amended, `+49 / -18` | one of its two worked pairs stopped being a pair |
| 6 | `a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour` | `mokiterions-tui` unit | amended, `+35 / -10` | the first Mokiterion to die is now the lowest identifier |
| 7 | `a_declining_mokiterion_shows_a_declining_bar` | `mokiterions-tui` integration | amended, `+18 / -2` | no starvation-driven health decline is left at 200 ticks |

One test-support helper was added alongside them, `highest_admitted_satiety` (`+14`), in
`mokiterions-core`'s test module. It is not a test and does not appear in the census.

No test was removed, renamed or marked `#[ignore]`. Two *assertions* were retracted, one in change 5 and
one in change 6, and each is named below with what replaced it. One asserted constant moved upward, in
change 4, and is argued there. Nothing else in the suite lost an assertion, and apart from those three,
every assertion in the suite is character-identical to what it was.

## 1. The added test

`the_corrected_non_waste_condition_admits_the_specified_boundaries` is the test `REQ-MOK-060` did not
have. The three amended engine functions were previously exercised only through the scenarios that happen
to cross them, which is how the omitted term survived twice — `SPEC-MOK-001` rule 5's condition and rule
19's tolerant test each dropped the same allowance, and every scenario built at a satiety derived from the
uncorrected boundary agreed with them.

It asserts, per calorie class, from literals transcribed out of the amended specification rather than
computed:

| class | restoration `R` | allowance `R * R / 100` | highest admitted satiety |
|---|---|---|---|
| low | `15` | `2` | `87` |
| medium | `30` | `9` | `79` |
| high | `50` | `25` | `75` |

and then, for each class, at the boundary satiety and one point above it, in **both** of rule 5's cases —
eating underfoot and approaching from one cell west — that the resource is taken at the boundary and
declined above it, with the draw count separating a deliberate approach from a search. Three properties
of this shape are worth naming because each closes a way the correction could have been wrong and still
passed everything else:

* **Inclusivity at the boundary.** `REQ-MOK-060`'s whole mechanism is that a resource wasting exactly its
  own allowance is taken rather than left standing. A condition written `<` instead of `<=` satisfies
  every other test in the suite.
* **Both cases from one condition.** Rule 5 screens eating and approaching by one test, and rule 5's
  second recorded defect was precisely the two disagreeing. A correction applied to eating alone would
  restore the two-cell oscillation.
* **The first clause still standing alone.** A low-class resource at satiety `80` fits outright, needing
  none of the allowance; a high-class one at the same satiety exceeds even the largest allowance in the
  table. Both are asserted, so the second clause cannot have swallowed the first.

The same test asserts `highest_admitted_satiety` against those same literals. That is deliberate: changes
2 and 3 derive their satieties from the helper, so a drift in the helper's arithmetic would move those
scenarios silently. Here it cannot — it is checked against the specification's own numbers in the same
loop.

## 2 and 3. The two reference-source scenarios

Both were written as "one satiety point above the level at which a high-class resource still fits", with
the level derived from the food table rather than from a threshold constant, because rule 5 states no
threshold. The level was `100 - 50 + 1 = 51`. Under the corrected condition it is `100 - 50 + 25 + 1 =
76`.

The change in both is the expression the satiety is derived from:

```rust
- simulation.agents[0].satiety = ATTRIBUTE_MAX - FoodClass::High.restoration().0 + 1;
+ simulation.agents[0].satiety = highest_admitted_satiety(FoodClass::High) + 1;
```

Every assertion in both tests is untouched. Change 2 still asserts that the resource is declined; change 3
still asserts that neither of two clipped resources is approached and that the step is a search. What
moved is the scenario's construction, and it moved in the direction that keeps the scenario the one the
test names: a scenario at satiety `51` would now be a scenario about a resource that *fits*, which is a
different test that happens to share a name. Deriving the level from the specification's arithmetic rather
than pinning it is why only one line each had to change, and it is why the next change to the food table or
the allowance will not need either.

## 4. The tolerance-zero identity test, and the constant that moved upward

`at_tolerance_zero_the_trait_aware_source_proposes_what_the_reference_source_proposes` discharges
`VER-MOK-010` oracle 3 by enumeration rather than by sampling: it builds every situation in the product of
five dimensions and asserts, in each, that the trait-aware source at `waste_tolerance = 0` proposes exactly
what the reference source proposes and consumes exactly as much of the stream.

**What forced it.** Its satiety dimension is not an arbitrary list. It is chosen to straddle every clipping
boundary the food table produces — one value below, one at, one above — because a boundary is where the two
sources would part if rule 19's tolerant test were not rule 5's own condition at `T = 0`. The boundaries
moved: `85`, `70` and `50` became `87`, `79` and `75`. The old list still enumerated 2,808 situations and
the test still passed, but it no longer straddled anything, and a test that passes for the wrong reason is
worse than one that fails.

**What changed.** The list grew from thirteen values to twenty-one, adding `74`, `75`, `76`, `78`, `79`,
`80`, `87` and `88` so that the corrected triples `74..=76`, `78..=80` and `86..=88` are all present. The three
*uncorrected* triples, `84..=86`, `69..=71` and `49..=51`, are **kept**: they are where a regression to the
omitted allowance would first show, and this is the test that would have to keep holding through such a
regression. The doc comment says both facts and dates the correction.

**The constant.** `assert_eq!(cases, 2_808)` became `assert_eq!(cases, 4_536)`, and this is the one place
in the change where an asserted number moved. It is named here because a moved constant is exactly what
`SPEC-MOK-002` rule 12 exists to catch. Three things make it a strengthening rather than a relaxation:

* it moved **up**, not down — 21 × 3 classes × 18 placements × 2 energies × 2 companion states, so every
  one of the 2,808 situations checked before is still checked under the same two assertions, and 1,728 more
  are;
* the assertion it stands beside is unchanged: `assert_eq!(cases, SATIETIES.len() * 3 * placements.len() *
  2 * 2)` already pinned the arithmetic, and the literal is the second, redundant guard whose whole purpose
  is that the set cannot shrink silently. A literal recomputed from the code it guards would be worthless;
  this one is recomputed from the specification's boundaries and then checked against the product;
* nothing else in the body moved. The two assertions inside the loop — proposal equality and stream
  equality — are character-identical.

## 5. The trait-difference test, and the one retracted assertion

`a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten` discharges `REQ-MOK-033`: that a
difference in `waste_tolerance` alone changes a proposal, in both of rule 19's worked cases, with the pair
either side of the deciding tolerance pinning the integer division as truncating rather than rounding.

**What was retracted.** The high-class pair at satiety `70` — `T = 40` eats, `T = 39` declines, because
`40 * 50 / 100 = 20` admits a waste of `20` and `39 * 50 / 100 = 19` does not. Under the corrected
condition rule 5's own allowance for that class is `25`, which admits a waste of `20` outright, so both
tolerances eat and the pair observes nothing. This is the same fact `SPEC-MOK-001`'s *Acceptance examples*
entry recorded as corrected rather than restated in the amendment of 2026-08-21, and it is the only place
in the suite where an assertion became false rather than merely relocated.

**What replaced it.** A low-class pair at satiety `88`: the waste is `3`, rule 5's allowance is
`15 * 15 / 100 = 2`, so the tolerance still decides — at `T = 20` (`300 / 100 = 3`) and not at `T = 19`
(`285 / 100 = 2`), and `19` is exactly where rounding would have admitted it. The truncation is pinned a
second time, at the other end of the food table, and neither value is at the range's bound, so the pair
survives a further narrowing of the range in a way the retracted one — which sat *on* the bound of `40` —
did not. The surviving medium-class pair at satiety `80`, `T = 34` against `T = 33`, is unmoved by
`REQ-MOK-060` and is unchanged.

**What was added.** Two assertions that sweep the trait's whole reachable range rather than naming a pair:

* a high-class resource at satiety `80` is declined at every tolerance in `0..=40` — the effect the
  2026-08-19 narrowing was made to produce, which `REQ-MOK-060` had to leave standing, and which it does,
  because `30` of waste exceeds both `25` and the `20` that `40 * 50 / 100` allows;
* a high-class resource at satiety `70` is eaten at every tolerance in `0..=40` **and** by the reference
  source, consuming no entropy in any of the 42 decisions — which is what `REQ-MOK-060` moved, and which
  records the trait as *masked* at that point rather than merely agreeing at two values.

That is why this change is a strengthening and not a substitution of equals. A pair asserts a difference at
two tolerances; a sweep asserts the absence of one across all forty-one. `SPEC-MOK-001`'s amended
*Behavioral trait* paragraph states the consequence — the band in which the trait changes a decision has
narrowed to low class above `T = 19` and medium class above `T = 33` — and this test is where that
narrowing is held to, at both of its edges.

## 6. The dead-selection scenario, in `mokiterions-tui/src/state.rs`

Rule 10.6 says a dead selection is retained and the next control moves to a living neighbour. The test
asserts it in both directions: Tab reaches the next living identifier above the dead one, BackTab the next
living identifier below it. Both need a dead Mokiterion with a living identifier on each side.

The pre-change scenario advanced to the **first** death and used that. `post/dead-neighbours.txt` measures
why that stopped working, from the capture cell the test's own defaults select — seed `0`, density `0.75`,
`--policy reference`:

| | first death | interior? | search settles on |
|---|---|---|---|
| pre-change | `M05` at tick 604 | yes | `M05` at tick 604 |
| candidate | `M01` at tick 314 | **no** | `M11` at tick 399 |

`M01` is the lowest identifier in the list, so no living identifier sorts below it and BackTab from it
wraps to the top instead of moving down. `assert!(backward < dead)` cannot hold on a wrap. The only way to
keep a scenario pinned to the first death would have been to weaken that assertion to admit the wrap,
which is a claim about a different behavior — and is exactly the move rule 12 forbids.

The amendment advances until a dead identifier holds living identifiers on **both** sides, which is the
state rule 10.6 describes. Every assertion that follows the loop — the selection is retained, it resolves
to no living agent, it resolves to a death, Tab moves to a living identifier above it, BackTab to a living
identifier below it — is unchanged, character for character. The measured margin is comfortable, tick 399 against a declared horizon of 700,
and the reader checks it at both commits so the amendment is not resting on one build's luck.

**The second retracted assertion is here**, and it is retracted by being subsumed. The pre-change loop ran
`while !observer.is_finished() && observer.deaths().is_empty()` and then asserted
`!observer.snapshot().agents.is_empty()`, "the scenario needs a living neighbour" — which is true of a
world with any survivor at all, including one whose only survivors sort above the dead identifier. In its
place the loop asserts `!observer.is_finished()` on every iteration with "the run ended without a dead
Mokiterion holding living identifiers on both sides, so rule 10.6's state was never reached". That is
strictly stronger: a non-empty population is implied by finding an identifier with living identifiers on
both sides of it, while the converse does not hold. The old guard is what let the failure through — with
`M01` dead and eleven Mokiterions living it passed, and the BackTab assertion two lines later was the one
that failed, on a precondition the scenario had never actually checked.

Why not the cheapest fix, which is `--policy baseline`? Because rule 4 applies no waste condition and the
baseline population starves together: all twelve identifiers die on tick 119, at both commits, so the tick
that produces a dead selection produces no living one. That is measured in the same file.

## 7. The declining-gauge scenario, in `mokiterions-tui/tests/render.rs`

`VER-MOK-013` acceptance scenario 1 states: "Run 200 ticks at a declared seed at the reference viewport.
For one Mokiterion whose health falls by at least thirty over the run, the filled-cell count of its health
gauge is non-increasing over that fall and strictly smaller at the end than at the start." The test guards
itself before it draws anything: if no Mokiterion still alive at tick 200 has fallen thirty points, it
fails with "this scenario is unexercised" rather than drawing a flat bar and passing.

That guard fired at this candidate. `post/health-falls.txt` measures the deepest such fall per source per
declared seed, at both commits, reproducing the test's own definition of a fall:

| source | deepest fall, pre-change → candidate |
|---|---|
| `reference` | 80 → **0** |
| `individual` | 45 → **0** |
| `social` | 85 → 85 |

Health falls only once satiety reaches zero, so a world that feeds sooner has no declining health to draw.
Seed 42 under `reference` fell 35 points before the correction and falls 0 after it. The guard was doing
its job, and the scenario had to move.

The amendment adds `--policy social` to both of the test's runs. Under `social` the decline is combat
damage rather than starvation: seed 42's deepest fall is 78 points at **both** commits, which is what makes
it the durable choice rather than a longer run — it decouples the test from the nutrition model that has
now broken it twice. Both observer constructions take the same flag, because the pass that finds the
subject and the pass that draws it must select the same source or they are not looking at the same
Mokiterion.

The guard threshold stays at thirty. Both assertions are untouched: the fill at the end is strictly below
the fill at the start, and across every one of the 200 samples the fill is monotone in the value, so no
pair of frames presents a lower health over a fuller bar. `VER-MOK-013`'s scenario names no decision
source, so the source is a parameter the scenario leaves open — the change is a selection inside the
scenario, not a change of it.

## What this does not claim, and what is outstanding

* **Not that `social` will exercise the gauge forever.** It is coupled to the combat model instead of the
  nutrition model, which is a different dependency and not none. A scenario that constructs a declining
  subject directly, rather than searching a live run for one, is the durable answer, and it needs a
  `VER-MOK-013` amendment this work order has no authority to make. Raised for the technical and assurance
  owners in `manual-assessment.md`, at the packet root.
* **The same question, for change 6.** The dead-selection scenario searches a 700-tick run for a state
  that could be constructed in one line through the existing test hooks. That test discharges two cases in
  `VER-MOK-005` — `REQ-MOK-021`'s "Selected Mokiterion dies" and `REQ-MOK-023`'s "Selection cycles living
  Mokiterions only" — and neither states how the state is reached, so the search is inside the scenario as
  written. Changing the scenario is a `VER-MOK-005` amendment and is raised in the same place.
* **Not that the tests being green makes the correction right.** Whether the condition is the right one is
  `post/divergence.txt`, `post/composition.txt` and `post/survivors.txt`. This document is only about
  whether the suite still asserts what it asserted before.
* **Not the whole census.** Six changed bodies and one addition are accounted for here.
  `post/test-census-reconciliation.md` accounts for all 268 names.
