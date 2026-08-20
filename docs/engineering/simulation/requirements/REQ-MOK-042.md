+++
id = "REQ-MOK-042"
type = "requirement"
title = "Detect contact between Mokiterions"
status = "draft"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHILE two living Mokiterions occupy coordinates at a Chebyshev distance of at most one, THE SYSTEM SHALL treat them as in contact, and SHALL make that contact available to the acting Mokiterion's decision opportunity as the precondition of every action that targets another Mokiterion."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Detect contact between Mokiterions

## Rationale

Every action that acts *on* another Mokiterion needs a stated range, and the range has to be narrow enough that the
action means something. `SPEC-MOK-001` rule 3's perception radius is `16`, which is the right radius for noticing
somebody and the wrong one for hitting them: a Mokiterion sixteen cells away is a fortnight of walking in tick terms and
is not in a fight.

Adjacency is chosen, and the arithmetic is `REQ-MOK-032`'s own. The expected number of other Mokiterions inside a
Chebyshev box of radius `r` on this world is `11 * (2r+1)^2 / 16384`. At `r=1` that is about `0.0060` per agent-tick —
roughly 72 contact agent-ticks in a 1,000-tick run with all twelve alive, before any Mokiterion deliberately closes
distance. That is rare, and it is deliberately not made less rare by widening the radius. It is made less rare by
`approach`, which `REQ-MOK-043` provides, because a Mokiterion that seeks contact is the behavior worth observing and a
Mokiterion that is merely near one is not.

Co-location alone was considered and rejected. Two Mokiterions on the same cell is `r=0`, about `0.0007` per agent-tick,
which would make combat verifiable only through constructed states — the failure mode `REQ-MOK-032` rejected a four-cell
`fear` threshold for, and the inert-value problem `SPEC-MOK-003` rule 4.5 refused, reached again by a third route.

Contact is defined as a relation rather than as stored state. Nothing records that two Mokiterions were in contact; it
is recomputed from positions at each decision opportunity, exactly as perception is. This keeps the world's authoritative
state where it is and means no contact can go stale.

## Preconditions and trigger

The trigger is a living Mokiterion's decision opportunity under `SPEC-MOK-001` rule 2, at which the observation of rule 3
is built.

Both parties must be living. A dead Mokiterion is in contact with nobody, and neither is a living Mokiterion with a dead
one, on the same ground rule 3 already excludes dead Mokiterions from the perceived list.

Contact is symmetric as a relation over positions and asymmetric in when it is *registered*, because rule 2 gives each
Mokiterion its own decision opportunity in ascending identifier order. This is the within-tick asymmetry rule 12 already
documents for `fear` — "two mutually adjacent Mokiterions may not both register the encounter on the same tick, because
the earlier one observed the later one's position before it moved" — and it is not a defect to be corrected here.

## Required response

- Two living Mokiterions are in contact when the Chebyshev distance between their coordinates is at most `1`, computed by
  the same distance the perceived-Mokiterion list of rule 3 is ordered by. Co-location, distance `0`, is contact.
- The acting Mokiterion's observation identifies which of its perceived Mokiterions are in contact. It does so by
  carrying the distance it already carries; no separate flag is required, and none is added, because the observation
  already states the distance of every perceived Mokiterion and a consumer can compare it to `1`.
- Contact at distance `2` or greater is not contact. Territory does not attenuate it: two Mokiterions adjacent across
  `y=63/64` are in contact, on the same ground perception already crosses that boundary.
- Contact is a precondition validated by the engine, not by the decision source. A proposal that targets a Mokiterion
  not in contact is rejected under `SPEC-MOK-001` rule 6, consuming the opportunity and mutating nothing.
- Contact is evaluated against authoritative state at validation time, not against the observation the source read. A
  target that was in contact when the observation was built and is not at validation time — because it moved or died in
  an earlier Mokiterion's turn of the same tick — yields a rejection.

## Failure and boundary behavior

- A Mokiterion is never in contact with itself. The observer never appears in its own perceived list under rule 3, so
  self-contact is unrepresentable rather than guarded against.
- Contact with a Mokiterion that died earlier in the same tick is not contact. This is a reachable path, not a
  theoretical one: rule 13 marks the dead immediately and a later-acting Mokiterion's observation was built after that.
- The radius is a specification constant, not a configuration input. There is no `--contact-radius`, and the constant is
  not derived from the perception radius, because the two answer different questions and a shared constant would tie
  them.
- No exit code, configuration error or runtime error arises from contact or its absence.

## Constraints

- One new constant. Contact reuses `SPEC-MOK-001` rule 3's Chebyshev distance rather than introducing a second distance
  notion; a Euclidean or Manhattan contact test beside a Chebyshev perception test would make two different senses of
  "near" observable in one system.
- Contact adds no field to the observation and no item to the engine's public interface. The distance that decides it is
  already carried.
- Contact consumes no entropy and reads no population aggregate — no count of how many Mokiterions are in contact
  anywhere else, no territory total, nothing outside the acting Mokiterion's own observation. `REQ-MOK-050` states that
  obligation generally; it is repeated here because contact is the first place a census would be tempting.
- Contact is not stored. No per-pair state, no memory of prior contact. That is `CAP-MOK-009`'s exclusion of interaction
  memory and this requirement does not breach it.

## Acceptance examples

### Example: normal behavior

**Given** two living Mokiterions at coordinates one cell apart on either axis or diagonally

**When** the earlier-identified one reaches its decision opportunity

**Then** its observation lists the other at distance `1`, and actions targeting it are valid proposals.

### Example: the boundary

**Given** two living Mokiterions at Chebyshev distance `1`, and a second pair at distance `2`

**When** each acting Mokiterion reaches its decision opportunity

**Then** the first pair is in contact and the second is not; a targeted proposal against the distance-`2` Mokiterion is
rejected.

### Example: co-location is contact

**Given** two living Mokiterions on the same coordinate

**When** either reaches its decision opportunity

**Then** the other is in contact at distance `0`, and targeted proposals against it are valid.

### Example: contact across the territory boundary

**Given** one living Mokiterion at `y=63` and another at `y=64` in the same column

**When** either reaches its decision opportunity

**Then** they are in contact, and no crossing event is emitted by the contact itself.

### Example: failure behavior

**Given** a Mokiterion whose observation listed a target in contact, and that target dies or moves to distance `2` in an
earlier Mokiterion's turn of the same tick

**When** the proposal is validated

**Then** it is rejected with a reason naming the unmet precondition, the opportunity is consumed, and no state changes.

## Open decisions

None at this level. The constant's value is `1` and is fixed by this requirement; its name and placement in
`SPEC-MOK-001` are the technical owner's.
