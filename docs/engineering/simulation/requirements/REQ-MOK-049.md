+++
id = "REQ-MOK-049"
type = "requirement"
title = "Keep the world habitable and combat lethal under the social decision source"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a simulation runs to 1,000 ticks using the social decision source at the default resource density, THE SYSTEM SHALL leave at least five of the twelve Mokiterions living and SHALL record at least one death attributable to combat, on every declared verification seed."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Keep the world habitable and combat lethal under the social decision source

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-20 | Original approved content: a two-sided bound per declared seed at the default density — at least five of twelve living at tick 1,000, and at least one death attributable to combat. | Approved 2026-08-20 by the repository owner acting as product owner. |
| 2026-08-20 | **The floor of five is ratified unchanged on the first measured curve, and the survival-first clause is corrected.** The value does not move: this row records the ratification the rationale and the open decision both reserved, so that the number stops being provisional. The *one legitimate lever* bullet is corrected where it described `REQ-MOK-048`'s ordering as "a Mokiterion acts socially only when it is neither hungry nor tired" — that was not what the five-branch ordering specified, and `REQ-MOK-048`'s amendment of the same date is what makes it true. The lethality bound stays an existence claim per seed; the deferred question of whether it should become a rate is answered below on the measured figures. | Ratified 2026-08-20 by the repository owner acting as product owner, on the measured curve in `evidence/WO-MOK-012/escalation.md`. Under the amended `REQ-MOK-048` the declared seeds leave 9, 10, 9, 9 and 11 living with 1, 2, 2, 3 and 1 combat deaths, so both bounds hold simultaneously on all five with four survivors of margin at the worst seed. Two alternatives were measured and declined in the same act: lowering the floor to three, and lowering it to two, each of which was reachable only by rescoping `SPEC-MOK-001` rule 12 to contact — which recovered lethality but put five beyond reach at every engagement threshold, and would have invalidated `WO-MOK-010`'s measured `fear` distribution. |

## Rationale

Two failures are possible once Mokiterions can kill each other, and they point in opposite directions. The world can
depopulate, in which case combat has replaced the simulation rather than joined it. Or combat can be inconsequential, in
which case the seven verbs are decoration and `fear` has a reader that never matters. A floor alone catches the first and
would be trivially satisfied by the second — a source that never attacks passes any survivor floor. So this requirement
is deliberately two-sided: a lower bound on survivors and a lower bound on lethality, both on every declared seed.

**Five of twelve.** `REQ-MOK-014` binds `reference` at eight of twelve at the default density, and `REQ-MOK-034` binds the
trait-aware source at the same eight. Five says that combat may cost this world three more Mokiterions than trait
individuality does, and no more.

**The floor was set at five rather than six, in advance of any measurement, by the product owner on 2026-08-20.** What
moved it is `REQ-MOK-044`'s damage decision taken in the same session: a range of `10..=30` kills a full-health target in
four to ten strikes, and a flat `5` `energy` cost does not stop a striker pressing an attack through to a kill. Six was
drafted before those values existed, and the owner's judgment is that a floor drafted against an unknown damage function
should not become the thing that forces the damage function to be re-decided.

**What moving it costs is stated rather than glossed.** Six was half the population, and the draft argued from that: below
half, the run stops being a society under pressure and becomes a cull, because with five or fewer alive at tick 1,000
contact at the rate `REQ-MOK-032`'s arithmetic gives is rare enough that the survivors are back to being solitary
foragers — the state this whole initiative exists to leave. **Five sits exactly on that line.** This obligation is
therefore weaker than the argument that produced six, and it stands as the owner's decision rather than as a figure
derived from the world's own arithmetic. Whether five was the right place is settled by the measured curve, not by this
text: `VER-MOK-012` records survivors per seed whether or not the floor is met, so the evidence will show how much
headroom five actually bought, and a curve that clusters at five or six is itself a finding about the damage decision.

**At least one death by combat.** `INT-MOK-009` states that an attack which cannot kill is a gesture, and
`REQ-MOK-044` makes death by combat reachable. Reachable is not reached. This obligation is what distinguishes a
mechanism that exists from a mechanism that happens, and it is stated per seed rather than in aggregate so that it cannot
be satisfied by one violent seed among many. `REQ-MOK-044` obliges the stream to distinguish death by combat from death
by starvation or exhaustion, which is what makes this countable from a run's own output.

**The floor stated here is provisional in a specific and precedented sense.** `REQ-MOK-014`'s amendment record shows what
happens to an unmeasured floor: its original absolute form measured zero survivors on every declared seed and had to be
restated, and its later floors were selected from a curve measured before a correction that invalidated them. This floor
is stated before any measurement exists, because a work order cannot be verified against an obligation that does not yet
exist — and it is expected to be ratified or amended by the product owner on the first measured curve, with the amendment
recorded in this requirement's own amendment record. What is **not** provisional is the shape: a two-sided bound, per
seed, at the default density.

**It stopped being provisional on 2026-08-20.** The first curve was measured, the product owner ratified five unchanged,
and the amendment record above carries the row. What the ratification bought is now a measured quantity rather than an
expectation: the declared seeds leave 9, 10, 9, 9 and 11 living, so the margin above the floor is four at the worst seed
and the curve does **not** cluster at five or six — which is a finding about the damage decision in the reassuring
direction, and the paragraph above reserved it as one.

It had already moved once, from six to five, before any measurement. That move carries **no amendment-record row**, and
deliberately: the requirement was `draft` when it happened, so the change was an edit to an unapproved obligation rather
than an amendment to an approved one. The amendment record begins at the approval this requirement was given, and every
movement of this floor after that belongs in that record — which is why the ratification of the same date carries a row
even though the number did not move. A ratification that left no trace would make an unmeasured floor and a measured one
indistinguishable in this file.

**One thing this requirement must not become.** A survivor floor is a population aggregate. It is read by the
*verification* path and by nothing inside the simulation — no rule, no source and no validation of a proposal may consult
it or anything like it. `REQ-MOK-050` states that separation, and it exists partly because this requirement makes the
temptation concrete: the easy way to meet a survivor floor is to have the engine stop combat when the population gets
low, and that is exactly `README.md`'s `if food < threshold: start_war()` with the sign reversed.

## Preconditions and trigger

The trigger is a completed 1,000-tick run at the default resource density under the `social` decision source of
`REQ-MOK-048`, on a seed in the declared verification set.

The obligation is stated at the default density only, on `REQ-MOK-014`'s reasoning: a higher density would document
headroom, and headroom is worth measuring as evidence rather than binding as an obligation.

## Required response

- **At least five of the twelve Mokiterions are living at tick 1,000**, on every declared verification seed, at the default
  density, under this source.
- **At least one death in the run is attributable to combat**, on every declared verification seed, distinguishable in the
  event stream from death by starvation or exhaustion under `REQ-MOK-044`.
- **Both bounds hold simultaneously on each seed.** A seed that depopulates and a seed that is peaceful are both failures,
  and a matrix that trades one against the other does not satisfy this requirement.
- **The floor is met by the behavior of the `social` source, not by weakening any rule.** No damage constant, no forfeit
  amount, no survival constant and no resource constant is tuned to reach this floor without the change being a governed
  amendment with its own approval. In particular, `REQ-MOK-014`'s and `REQ-MOK-034`'s floors must still hold after any
  such change, and `baseline` must still reproduce byte-identically.
- **The `social` source's own ordering and thresholds are the one legitimate lever**, because they are that source's
  behavior rather than a rule's constant. `REQ-MOK-048`'s ordering — survival first, then a perceived meal, and only then
  society — is what this floor rests on, given that `REQ-MOK-044`'s `5` `energy` cost brakes combat weakly. Changing that
  order or those thresholds is a change to `REQ-MOK-048` and is governed there; it is not a tuning commit, and it is not
  forbidden by the bullet above. **That lever was used on 2026-08-20**, in both of the ways this bullet contemplates, and
  the amendment record above records the result.

  **Corrected 2026-08-20.** This bullet described `REQ-MOK-048`'s ordering as "a Mokiterion acts socially only when it is
  neither hungry nor tired". **That was not what `REQ-MOK-048` specified.** Its branch 2 fired on a tolerated resource
  *underfoot* or on exhaustion, and its seek-move sat behind both social branches, so a Mokiterion hungry with food four
  squares away engaged instead of eating. At the default density of `0.75%` a hungry Mokiterion is almost never standing on
  food, which made the property this floor was said to rest on a property the specified order did not have — and it was the
  mechanism by which the first candidate starved its population in a world with no fighting in it. The clause is not merely
  reworded here: `REQ-MOK-048`'s amendment of the same date hoists the seek-move above society, so the sentence is now
  true of the requirement it describes. It is recorded as a correction rather than quietly fixed because for one candidate
  this text asserted a property of another requirement that that requirement did not have.
- **The measured curve is recorded as evidence**, on the pattern of `WO-MOK-002`'s `density-curve.md`, including survivors
  per seed, deaths by cause per seed, and the count of each targeted action proposed and applied. The evidence is what the
  product owner ratifies the floor on.

## Failure and boundary behavior

- A run with zero contacts on some seed fails the lethality bound. The remedy is a matrix that reaches contact or a source
  that seeks it, not an exemption.
- A run in which every Mokiterion dies fails the survivor bound whatever the cause of death, including starvation. This
  requirement does not distinguish causes on the survivor side; it binds habitability, and a world made uninhabitable by a
  combat-driven collapse of foraging behavior fails it as squarely as one where they killed each other.
- A run at a non-default density is outside this requirement's obligation. It may be captured as evidence.
- A run under `baseline`, `reference` or `individual` is outside this requirement. `REQ-MOK-014` and `REQ-MOK-034` bind
  those, and both are re-measured under this initiative because a change to the resource composition of the world moves
  them.
- Fewer than twelve living Mokiterions at initialization is not a reachable state; the population is fixed by
  `SPEC-MOK-001`.

## Constraints

- **No population aggregate is read inside the simulation** to satisfy this requirement, and nothing about a survivor count
  is available to any rule, source or proposal validation. `REQ-MOK-050`.
- **No entropy change.** Meeting this floor must not be achieved by moving the shared stream's consumption, which would
  diverge every pre-existing run.
- **Meeting this floor must not be achieved by any change to the `baseline` source.** Its outcomes are held
  byte-identical by this initiative. This is deliberately narrower than `REQ-MOK-034`'s existing clause, which names
  `reference` too; that clause is amended by `REQ-MOK-051`, which moves `reference` by construction.
- **The default density and the default policy are unchanged.** The floor is measured at the default density under an
  explicitly selected policy.
- The declared verification seed set is the existing one. This requirement does not introduce a seed set of its own, so
  that its results are comparable with `REQ-MOK-014`'s and `REQ-MOK-034`'s at matched seeds.

## Acceptance examples

### Example: normal behavior

**Given** the default density and the `social` decision source

**When** a 1,000-tick run completes on each declared verification seed

**Then** each run leaves at least five Mokiterions living and records at least one death attributable to combat.

### Example: the lethality bound bites

**Given** a source that reads `fear` but never proposes `attack` or `fight`

**When** the matrix runs

**Then** every seed leaves twelve survivors and every seed fails this requirement.

### Example: the habitability bound bites

**Given** a damage function large enough that encounters are usually fatal

**When** the matrix runs

**Then** seeds fall below five survivors and fail, and the remedy is a governed amendment to the specification's damage
constants rather than a change to the verification threshold.

### Example: comparability

**Given** the same declared seeds under `reference`, `individual` and the `social` source at the default density

**When** all three matrices run

**Then** the survivor counts are directly comparable, and the difference between the `social` source and `individual` is the
measured cost of combat.

### Example: failure behavior

**Given** any declared seed that leaves fewer than five living, or that records no death attributable to combat

**When** the verification matrix is evaluated

**Then** the requirement is not met, and the run's exit code is `0` regardless — habitability is a measured property, not a
runtime error.

## Open decisions

- **The floor's value was the product owner's to ratify or amend on the first measured curve, and was ratified at five on
  2026-08-20.** This decision is closed. Five was stated so that the work order had an obligation to verify against;
  `REQ-MOK-014`'s amendment record is the precedent for expecting the number to move once evidence exists, and for recording
  the move here rather than silently in a verification contract. It had already moved from six to five once, before
  measurement and while `draft`. On the curve it did not move again: the measured survivors are 9, 10, 9, 9 and 11, so five
  binds with margin rather than by luck. Two lower floors were measured and declined in the same act — three and two — each
  reachable only under a rescoped rule 12 that put five out of reach at every engagement threshold and would have
  invalidated an approved Phase 2 measurement to do it.
- Whether the lethality bound should be stronger than "at least one" — a rate rather than an existence claim — was deferred
  until the first curve exists. **The curve now exists and the bound stands as an existence claim**, on the product owner's
  ratification of 2026-08-20. The measured combat deaths are 1, 2, 2, 3 and 1 across the declared seeds, so a rate bound of
  two or more would fail on two seeds and a bound of one is what the measurement supports. Recorded rather than tightened:
  the figures are close enough to the bound that raising it would be selecting a threshold from the same five numbers it is
  meant to test, which is `REQ-MOK-014`'s amendment record's lesson about floors chosen from the curve they bind. An
  existence claim per seed remains the weakest form that cannot be satisfied by an inert mechanism.
