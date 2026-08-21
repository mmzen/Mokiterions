+++
id = "REQ-MOK-060"
type = "requirement"
title = "Bound the class composition of a territory's standing resources"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-21"
statement = "WHEN a simulation runs to 1,000 ticks at the default resource density under the reference, trait-aware or social decision source, THE SYSTEM SHALL leave no calorie class holding more than three fifths of any territory's standing resources, on every declared verification seed."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-010"]
+++

# Requirement: Bound the class composition of a territory's standing resources

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-20 | Original approved content for `CAP-MOK-010`: a ceiling of one half, per territory, per class, per seed, at the default density under the reference, trait-aware and social sources, with the correction surface fixed at `SPEC-MOK-001` rule 5's non-waste condition and rule 19's tolerant form. | Approved 2026-08-20 by the repository owner acting as product owner, together with `REQ-MOK-051` through `REQ-MOK-059`, `SPEC-MOK-001`'s contact amendment, `VER-MOK-016` and `WO-MOK-016`. The ceiling was ratified at one half in advance of measurement against `60%` and `40%`, and *Open decisions* left it open to amendment on the first measured curve. |
| 2026-08-21 | **The ceiling is amended from one half to three fifths**, in the statement, in *Required response*, in *Failure and boundary behavior*, in three *Acceptance examples* and in *Open decisions*. Nothing else moves: the surface the correction may use, the four prohibited sites, the three obligated sources, the per-territory and per-seed form, the default density, the 1,000-tick horizon, the declared seed set, the two carried survivor floors and the `45` of `61` this requirement ends are all untouched. This row is the amendment *Open decisions* reserved, taken on the ground it reserved it for: **one half is not reachable inside this requirement's own permitted surface while `REQ-MOK-014`'s and `REQ-MOK-034`'s floors of eight and `REQ-MOK-058`'s floor of five hold.** That was measured before this row was written, over roughly 55 parameter points of three mechanism shapes on all 15 obligated cells, and the wall is not a tuning artifact: the closest point that meets one half on all 15 cells leaves 6 of 12 living under `reference` and `individual`, and no point that holds all three floors meets one half at all. The best floor-respecting point measured is the condition ratified with this row, and its worst class share is `54.1%` — four points above one half, and six below three fifths. That is the shape of the wall: the surface can be pushed until the ceiling is met or until the floors hold, and not both. Three fifths is met with margin — worst share `54.1%` at the ratified condition — so this row concedes `10` points of drift above the balanced initial third rather than abandoning the obligation. | Approved 2026-08-21 by the repository owner acting as product owner, on the measurement retained in `docs/engineering/simulation/evidence/WO-MOK-017/approval/`, against three declined alternatives, each measured rather than argued: amending `REQ-MOK-014`'s and `REQ-MOK-034`'s floors of eight to six and `REQ-MOK-058`'s of five to three, declined because a measured floor is what a correction is held against and because the relaxation it needs makes `waste_tolerance` behaviorally inert, voiding `CAP-MOK-006`'s individuality — `reference` and `individual` print identical summaries on all five declared seeds at that point; widening the correction surface to rule 15, rule 16, rule 9's eat effect or the food table, declined because it ends `INT-MOK-010`'s byte-identity promise for `baseline`; and leaving this requirement unimplemented, declined because the drift it names is measured and uncorrected. The implementation agent measured the options and wrote this text; it did not decide the substance. |

## Rationale

`SPEC-MOK-001` rule 5 records this effect and records that it was left alone: "measurement at the default density puts
high class at 45 of 61 resources in a territory by tick 1,000, against a balanced initial third. This is accepted at the
1,000-tick horizon `REQ-MOK-014` states... It is recorded as a known long-horizon effect rather than a defect, and
addressing it is out of scope for this revision." It also records that rule 19's per-Mokiterion tolerance was expected to
reduce it, and closes: "**No obligation is stated on the result in either direction.**" This requirement is that
obligation, and stating it amends that sentence.

**Why here, and why now.** The effect and this initiative invalidate the same two things. `REQ-MOK-014` binds
`reference` at eight survivors of twelve at the default density and `REQ-MOK-034` binds the trait-aware source at eight;
both floors were measured on a world in which three quarters of a territory's standing food is high class by tick 1,000,
and both must be re-measured if that composition changes. Combat requires them re-measured too. Doing both corrections in
one work order means the two floors are re-measured **once**, against a world that has both properties, instead of twice
against two intermediate worlds that will never ship. That was the product owner's decision and it is the reason a
resource-composition requirement appears in a capability about conflict.

**Why it is worth correcting at all.** The mechanism is stated in rule 5 itself: a high-class resource restores `50`
satiety, so the non-waste condition makes it eatable only at satiety of at most `50`. High class is therefore passed over
more often than low or medium, and what is passed over stays standing while regeneration keeps adding classes uniformly.
The result is a world whose food is increasingly the food nobody can eat. That is not a balance complaint; it is a slow
drift toward a state where the density an operator selected no longer describes the food actually available to a
Mokiterion, which makes `REQ-MOK-014`'s density-indexed floor progressively less meaningful the longer a run goes.

**Where the correction may be made, and where it may not.** This constraint is the load-bearing part of this requirement,
because it is what keeps `INT-MOK-010`'s promise that `baseline` reproduces byte-identically.

- The correction **may** be made in the waste condition of the decision sources — rule 5's `satiety + restoration <= 100`
  and rule 19's tolerant form `S + R - 100 <= T * R / 100`. Those rules are `reference`'s and `individual`'s proposal
  logic. Changing them changes what those two sources choose, and leaves `baseline` untouched: rule 4's candidate list
  offers `eat` for each co-located resource with no waste condition at all, so no relaxation of a waste condition can
  reach it.
- The correction **may not** be made in rule 16's uniform class selection, in rule 15's regeneration amount, in rule 9's
  eat effect, or in the food table's restoration values. Each of those is world behavior under every policy, so each would
  change what a `baseline` run does and diverge every pre-existing baseline capture. Rule 16's class selection would
  additionally risk moving the shared entropy stream's consumption, which diverges everything.

So the additivity cost of this requirement is bounded and stated in advance: `reference` and `individual` move,
`baseline` does not. That is narrower than `REQ-MOK-034`'s existing clause, which states that both `reference`'s and
`baseline`'s outcomes "are frozen" — and **that clause is amended by this requirement**, narrowed to `baseline`, with the
amendment recorded in `REQ-MOK-034`'s own amendment record and approved as an amendment rather than assumed by this text.

**Three fifths, per territory, per seed.** The initial composition is a balanced third. A ceiling of three fifths permits
real drift — the mechanism is not being abolished, and a source that prefers calories should still leave more high class
standing than low — while ruling out the measured 45 of 61. **The value was one half until 2026-08-21 and the amendment
record states why it moved**: one half is not reachable inside the surface this requirement permits while the two carried
survivor floors and `REQ-MOK-058`'s hold, which was measured before the value was changed rather than discovered after
it. Three fifths leaves `10` points above the balanced third rather than one half's `17`, and it is met at the ratified
condition with a measured worst case of `54.1%`, so the obligation retains margin rather than being drawn to the edge of
what the mechanism can do. It is stated per territory rather than over the world because the two
territories regenerate independently under rules 14 to 16 and a world average would hide a single territory's drift. It is
stated per seed for the reason `REQ-MOK-058`'s bounds are: an aggregate over seeds can be met while individual runs fail.

## Preconditions and trigger

The trigger is a completed 1,000-tick run at the default resource density under `reference`, `individual` or the `social`
source of `REQ-MOK-057`, on a seed in the declared verification set.

`baseline` is outside this requirement's obligation, and deliberately: it is the source this initiative holds
byte-identical, so it cannot be made to satisfy a new obligation about the world it produces.

## Required response

- **At tick 1,000, in each territory, no calorie class holds more than three fifths of that territory's standing
  resources**, on every declared verification seed, at the default density, under each of the three sources named. Exactly
  three fifths satisfies the obligation; the breach is a share strictly above it.
- **The composition is measurable from the run's own output.** Rule 18's final summary already reports remaining food by
  territory and calorie class, so the measurement needs no new instrumentation and no new event.
- **`REQ-MOK-014`'s survivor floor of eight of twelve under `reference` still holds after the correction**, re-measured on
  the declared seed set. A correction that fixes composition by starving the population is not a correction.
- **`REQ-MOK-034`'s survivor floor of eight of twelve under the trait-aware source still holds after the correction**,
  re-measured on the same seeds.
- **Every `baseline` run reproduces byte-identically** against the pre-change capture, on the declared matrix. This is the
  check that the correction was placed where this requirement permits it.
- **Every divergence in `reference` and `individual` output is attributed to this correction** and to nothing else, and the
  attribution is recorded as evidence — a re-capture of both sources' declared matrix, with the divergence characterized
  rather than merely acknowledged.
- **The corrected condition is stated in `SPEC-MOK-001`**, as an amendment to rule 5 and rule 19 rather than as a new
  appended rule, because it changes those two sources' proposal logic and a second condition stated elsewhere would leave
  two waste conditions in the specification.

## Failure and boundary behavior

- A territory driven to a single class, or to zero of some class, satisfies the ceiling as stated and is nonetheless a
  degenerate outcome; the measured composition per class per territory is recorded as evidence so that the owner can see it
  and decide whether a floor is also wanted. This requirement states a ceiling only.
- A territory reduced to very few standing resources makes the ratio coarse — at four standing resources, three of one
  class breaches three fifths. The obligation is stated on the ratio regardless, because a territory that has drifted to
  four resources is already a `REQ-MOK-014` problem.
- A depleted territory, holding zero standing resources, holds no class above three fifths of zero and satisfies the
  ceiling vacuously. Rule 15 makes permanent local depletion reachable at every density and this requirement does not change that.
- A run at a non-default density is outside the obligation and may be captured as evidence.
- A run beyond 1,000 ticks is outside the obligation. `VER-MOK-010` measured the 10,000-tick horizon for the trait-aware
  source and a longer-horizon obligation is not stated here, because the drift is by nature a long-horizon effect and
  binding it at a horizon nothing else is bound at would make one requirement the whole of the project's long-run policy.
- No composition outcome produces a runtime error or a non-zero exit code.

## Constraints

- **`baseline` is byte-identical.** No change to rule 4, to rule 9's eat effect, to the food table, to rules 14 to 16, or to
  anything else a `baseline` run reads.
- **The shared entropy stream's consumption is unchanged in kind and order.** The correction takes no new draw and moves no
  existing one. Where `reference` and `individual` runs consume a different total number of draws because they consume food
  at different times, that is a consequence of different world evolution and not of a changed draw discipline.
- **No entropy substream.** Still declined, for Phase 2's reason.
- **No population aggregate is read.** In particular, no source may read the class composition of its territory in order to
  decide what to eat. `REQ-MOK-059` bounds the population; this constraint extends the same discipline to the world's
  composition, because a source that read it would be the same shape of defect one step removed.
- **No new attribute, no new event, no new interface item.** Rule 18 already reports what is measured.
- **The default density and the default policy are unchanged.**
- The declared verification seed set is the existing one, so the re-measured floors are comparable at matched seeds with
  `REQ-MOK-014`'s and `REQ-MOK-034`'s originals.

## Acceptance examples

### Example: normal behavior

**Given** the default density under `reference`

**When** a 1,000-tick run completes on each declared verification seed

**Then** rule 18's summary shows no class holding more than three fifths of either territory's standing resources.

### Example: the effect this corrects

**Given** the pre-change behavior at the default density under `reference`

**When** the same measurement is taken

**Then** high class stands at 45 of 61 in a territory — `73.8%`, above three fifths and above the one half this ceiling
first stated — and the requirement is not met, which is the recorded state rule 5 accepted and this requirement ends.

### Example: baseline is untouched

**Given** the declared matrix under `baseline`, captured before the change

**When** each run is repeated after it

**Then** every output is byte-identical.

### Example: the carried floors survive

**Given** the declared seed set at the default density

**When** the matrix is re-measured under `reference` and under `individual`

**Then** each leaves at least eight of twelve living, satisfying `REQ-MOK-014` and `REQ-MOK-034` against the corrected
world.

### Example: failure behavior

**Given** any declared seed where a class holds more than three fifths of a territory's standing resources at tick 1,000
under a bound source, or any `baseline` run that diverges from its pre-change capture

**When** the matrix is evaluated

**Then** the requirement is not met, and the finding names the seed, the source, the territory and the class.

## Open decisions

- **Which of the permitted mechanisms is used was the technical owner's, and it is closed.** Decided 2026-08-21 on the
  measurement in `evidence/WO-MOK-017/approval/`: rule 5's waste condition is relaxed, and rule 19's tolerant test is
  amended only in that its first clause becomes a reference to rule 5's condition rather than the literal `S + R <= 100`.
  A tolerance floor was declined. The reason is measured and structural rather than aesthetic: `individual` and `social`
  both read rule 19's test and not rule 5's, so without that restatement a rule 5 correction reaches one of the three
  obligated sources; and a tolerance floor alone, at every value in the trait's range including its bound of `40`, leaves
  14 of the 15 obligated cells breaching, because it cannot reach `reference` at all. The numeric form ratified with it is
  `S + R - 100 <= R * R / 100`, which is rule 19's own arithmetic at a tolerance equal to the restoration itself.
- **The ceiling's value was ratified at one half by the product owner on 2026-08-20 and amended to three fifths on
  2026-08-21.** The original ratification was taken in advance of measurement against two alternatives: `60%`, which would
  have been easier to meet with a smaller relaxation of the waste condition and therefore gentler on the re-measured
  floors, and `40%`, which would likely have needed both permitted mechanisms and moved `reference` and `individual`
  further. One half was kept because it leaves `17` points of headroom above the balanced initial third — so real drift
  remains permitted — while ruling out the measured `45` of `61`. **The first measured curve then showed one half
  unreachable inside this requirement's permitted surface while the carried floors hold, and the amendment this entry
  reserved was taken to `60%`** — the alternative already named and declined here, for the exact reason given for it. The
  amendment record carries the figures. The value is not open a second time on the same evidence.
- Whether a per-class floor should accompany the ceiling **remains deferred**, and was reserved again by the product owner
  on 2026-08-21 to the corrected composition `WO-MOK-017` measures and retains, rather than taken on the approval
  measurement. A ceiling alone can be satisfied by a world that has drifted the other way, and the evidence will show
  whether it has. What the approval measurement already shows is that it has not inverted — the dominant class stays high
  or medium on every obligated cell and no class reaches zero — while the smallest class share at the ratified condition
  is `14.3%`, low class at 8 of 56 in the second territory under `individual` on seed `42`, so the question is a real one
  rather than a formality.
