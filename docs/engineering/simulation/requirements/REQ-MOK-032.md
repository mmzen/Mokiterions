+++
id = "REQ-MOK-032"
type = "requirement"
title = "Maintain fear as a fourth bounded dynamic attribute driven by perceived company"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "WHEN a living Mokiterion's action opportunity in a tick has completed, THE SYSTEM SHALL update its fear attribute within the same bounded range as its other attributes, raising it when that Mokiterion perceived at least one other living Mokiterion and lowering it otherwise, and SHALL report the attribute wherever health, satiety and energy are reported."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-006"]
+++

# Requirement: Maintain fear as a fourth bounded dynamic attribute

## Rationale

The concept names four attributes and the engine computes three. `fear` is the one that shapes how a Mokiterion
reacts to another: whether it fights, retreats or surrenders. None of those responses exists yet, and none can be
specified against an attribute that does not exist, so the attribute has to come first.

The observer already agrees. `SPEC-MOK-003` rule 4.5 reserved trailing space in the roster's bar row for `fear` by
name, and required that space to render empty — "no label, no dash and no zero" — precisely because "an inert
`fear 0` would be a claim the engine cannot support". That reservation is a standing statement that the presentation
is ready and the engine is not. This requirement is what makes the claim supportable, and it is what lets the
reservation be filled rather than explained.

Company is the right driver because another Mokiterion is the only threat the world currently contains and because
the engine already perceives it. Rule 3's observation lists every living Mokiterion within the perception radius, so
an attribute driven by that list costs no new perception pass, no new state, no new traversal and no entropy. Rising
when someone is in view and decaying when nobody is means the value tracks something real about the run rather than
counting ticks.

**The driver is perception itself, not a narrower distance, and that is a measured choice rather than a lazy one.**
A tighter threshold would need a new constant, and the arithmetic says it would also make the attribute inert. With
twelve Mokiterions on a 128×128 grid the expected number of others inside a Chebyshev box of radius `r` is
`11 * (2r+1)^2 / 16384`: about `0.73` at the perception radius of `16`, about `0.19` at `8`, and about `0.05` at `4`.
Clustering around food raises all three, but the ordering holds. A four-cell threshold would leave `fear` at the
lower bound on roughly nineteen agent-ticks in twenty, verifiable only through constructed states — which is a
different way of shipping the inert value `SPEC-MOK-003` rule 4.5 refused. Reusing the perception radius removes a
constant instead of adding one, and makes the update a test of whether a list is empty.

Nothing reads `fear` in this scope, and that is deliberate rather than an omission: see `CAP-MOK-006`. What is
claimed here is that the value is computed, bounded, reproducible and responsive — and every one of those is
falsifiable today.

## Preconditions and trigger

The trigger is the completion of a living Mokiterion's action opportunity within a tick, at the point `SPEC-MOK-001`
rule 12 applies survival decay. The update happens once per living Mokiterion per tick, in the same ascending
identifier order as everything else in the tick.

`fear` is maintained under every decision source. It is world state, not policy state; a run at one seed holds the
same fear values whichever source produced the actions that led there, given the same actions.

## Required response

- Every Mokiterion holds a `fear` value in the same bounded integer range as `health`, `satiety` and `energy`, and it
  starts the run at the range's lower bound: no Mokiterion begins afraid.
- The update reads the observation the engine already created for that Mokiterion's decision opportunity in this
  tick. It performs no additional perception, examines no state the observation does not already carry, and consumes
  no entropy.
- When that observation's list of perceived living Mokiterions is non-empty, `fear` rises by the specified increment.
  When it is empty, `fear` falls by the specified decrement. The count does not scale the change: one perceived
  Mokiterion and four produce the same rise, and neither distance nor direction within the perception radius affects
  it.
- Both directions saturate within the range. The value never exceeds the upper bound and never falls below the lower
  bound, and saturation is a normal outcome rather than an error.
- The value is reported wherever the other three attributes are reported: in the initialization record, in the
  per-tick survival record as a before-and-after pair in the same form the other three use, in the action trace when
  it is enabled, and in the read-only observation snapshot the observer consumes.
- Because the observer receives it in the snapshot, the roster slot `SPEC-MOK-003` rule 4.5 reserved is filled by an
  engine-computed value and stops rendering empty. No new presentation requirement is created; `REQ-MOK-020` already
  obliges presenting survival state.

## Failure and boundary behavior

- There is no failure path and no new error. The update is total, cannot be skipped and cannot produce an
  out-of-range value, so it has no effect on the exit-code contract.
- A Mokiterion that is not living receives no `fear` update, exactly as `SPEC-MOK-001` rule 13 already denies dead
  Mokiterions observations, decisions, actions, traces and survival updates. Its last value is retained in state, as
  its other attributes are. It is not reported after death: the text record emits no further line for it, and the
  observation snapshot already carries living Mokiterions only, so `fear` inherits that boundary rather than
  introducing one.
- A Mokiterion that dies during the tick in which its `fear` was updated keeps the updated value. Death does not
  reset or clear it.
- At the lower bound, a decay leaves the value at the lower bound and still emits the before-and-after pair, so a
  reader can distinguish "was not updated" from "was updated and did not move" only by the Mokiterion's living
  state, which the record already carries. At the upper bound, a rise behaves symmetrically.
- The action trace reports `fear` as it stands *before* this tick's update, because `SPEC-MOK-001` rule 7 places the
  trace before survival decay and the other three attributes it reports are likewise pre-decay. The survival record
  reports the update itself. The two lines for one tick therefore disagree by one step, and that is the same
  disagreement they already carry for satiety and energy.
- Perception is not symmetric within a tick. A Mokiterion acting early observes the positions later-acting
  Mokiterions held before they moved, so two mutually adjacent Mokiterions may not both register the encounter on
  the same tick. This is the sequential-order asymmetry `SPEC-MOK-001` rule 2 already establishes for every rule in
  the tick; `fear` inherits it and does not correct it.
- Territory boundaries do not attenuate the driver. Perception already crosses `y=63/64`, so a Mokiterion may be
  made afraid by a neighbour in the other territory.
- The driver is exactly the perception radius, so the boundary case is perception's own boundary and no new one is
  introduced: a Mokiterion at Chebyshev distance `16` raises `fear` and one at `17` does not, because the second is
  not in the observation at all.

## Constraints

- Integer arithmetic with saturation. No floating-point value participates.
- No entropy. The update draws nothing from the shared stream, so it cannot move the sequence `REQ-MOK-014`'s
  measured floor rests on.
- No new perception. The update consumes the existing observation and must not re-scan the world, or the per-tick
  cost of the tick loop would change for a value nothing reads.
- No consumer. No decision source, no validation rule, no survival rule and no termination rule may read `fear` in
  this scope. Adding a consumer is a later governed change, and a consumer added here would silently change outcomes
  under the existing sources.
- The attribute joins the read-only observation snapshot, which is the only public interface growth this requirement
  needs. It carries a value, not a reference, and it grants no mutation, so `SPEC-MOK-002` rule 6 is undisturbed.
- The increment and the decrement are specification constants and are not fixed here, and the driver adds no constant
  at all: it is the perception radius `SPEC-MOK-001` already fixes. This requirement fixes only that the attribute
  exists, is bounded, starts at rest, rises when another living Mokiterion is perceived, decays when none is, and is
  reported alongside its three siblings.

## Acceptance examples

### Example: normal behavior

**Given** a living Mokiterion with `fear` at the lower bound whose observation for this tick contains one perceived
living Mokiterion

**When** its action opportunity completes

**Then** the survival record for that tick reports `fear` rising by exactly the specified increment, and the value
stays in range.

### Example: decay in isolation

**Given** a living Mokiterion whose observation for this tick contains no perceived living Mokiterion

**When** its action opportunity completes

**Then** the survival record reports `fear` falling by exactly the specified decrement, or remaining at the lower
bound if it was already there.

### Example: saturation

**Given** a living Mokiterion at the upper bound of the range that continues to perceive another living Mokiterion

**When** many consecutive ticks complete

**Then** the value remains at the upper bound on every tick and never exceeds it.

### Example: reproducibility

**Given** the seed `42`, the default density and a fixed decision source

**When** two separate processes run one thousand ticks

**Then** the two runs report the identical sequence of `fear` transitions for every Mokiterion.

### Example: failure behavior

**Given** a Mokiterion that died on an earlier tick

**When** later ticks complete

**Then** no survival record, action trace or decision is emitted for it, its retained `fear` value is unchanged from
the tick it died on, and it is absent from the observation snapshot exactly as it is today.

## Open decisions

None. The two step sizes are the technical owner's to fix in `SPEC-MOK-001`; `WO-MOK-007` records the values proposed
for them and the owner's decision of 2026-08-19. The product decisions — that the fourth attribute is `fear`, that
perceiving another living Mokiterion drives it, that it starts at rest, and that nothing reads it yet — are settled
here and in `CAP-MOK-006`.
