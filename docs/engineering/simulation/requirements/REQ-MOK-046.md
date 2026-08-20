+++
id = "REQ-MOK-046"
type = "requirement"
title = "Threatening raises the target's fear and does nothing else"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a valid threaten action resolves between two living Mokiterions in contact, THE SYSTEM SHALL increase the target's fear by a stated bounded amount and SHALL change no other attribute of either Mokiterion."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Threatening raises the target's fear and does nothing else

## Rationale

`threaten` is the verb that makes `fear` a currency rather than a readout. It costs the threatener nothing, it damages
nobody, and its whole effect is on the target's `fear` — which is the attribute `REQ-MOK-045` puts on the observation and
`REQ-MOK-048`'s source reads. That makes `threaten` the only action in this world whose effect is entirely a change to
what another Mokiterion will decide. It is the cheapest way to establish that social behavior here is behavior and not
bookkeeping: if a frightened Mokiterion retreats, then a Mokiterion that frightens another has changed the world without
touching it.

Costing nothing is a deliberate product decision and it is the one this requirement most exposes. A free action that has
an effect will be proposed often by any source that can propose it, and `VER-MOK-012` measures how often. The owner's
position is that a threat should be free because refusing to make it free would mean inventing an energy price for
speech, and because the check on over-threatening is that it accomplishes nothing against a target already at `100`
`fear` — not that it is expensive.

**The value the product owner decided on 2026-08-20** is a new constant of `30`, not a reuse of rule 12's `FEAR_INCREASE`
of `10`. Three things follow from it and are recorded here because they are the reason the value is `30` and not something
else. It takes **six quiet ticks** to shed, at rule 12's `FEAR_DECREASE` of `5`, so a threat outlasts the tick it was made
in and can still be read at a later opportunity. It is **three times an ordinary perception**, which makes a threat
attributable in a `fear` series rather than indistinguishable from having noticed a neighbour — the thing that makes
`VER-MOK-012`'s evidence readable. And it composes deliberately with `REQ-MOK-048`'s thresholds: that source attacks a
Mokiterion in contact while its own `fear` is below `30` and threatens otherwise, so **one threat is exactly enough to turn
a calm aggressor into a threatener**, and to turn an approach into an avoidance. Pinning `fear` straight to
`ATTRIBUTE_MAX` was declined for the opposite reason: it would flatten the gauge the `social` source exists to read, and
every threatened Mokiterion would look identical to it.

The composition with rule 12 is the subtlety, and it is stated rather than discovered. Rule 12 already writes `fear`:
`+10` when the acting Mokiterion's own observation lists another living Mokiterion, `-5` when it does not, saturating at
`0..=100`. A threat writes the *target's* `fear` at the *threatener's* turn. So within one tick a Mokiterion's `fear` can
be written twice, by two different Mokiterions' turns, and the order is rule 2's ascending identifier order. This
requirement does not reorder those writes and does not exempt either. It states that both apply, that both saturate, and
that the resulting value is whatever the two composed writes yield.

## Preconditions and trigger

The trigger is the engine applying a valid `threaten` under `REQ-MOK-043`: both Mokiterions living, in contact under
`REQ-MOK-042`, target not the actor, all checked against authoritative state.

## Required response

- **The target's `fear` increases by `30`, saturating at `ATTRIBUTE_MAX`.** The amount is a specification constant of its
  own, fixed in `SPEC-MOK-001` and distinct from rule 12's `FEAR_INCREASE` of `10`, and it is not derived from either
  Mokiterion's attributes — a threat is a threat regardless of who makes it, because deriving its force from the
  threatener's condition would be a reading of relative strength arriving by the back door.
- **Nothing else changes.** Not the target's `health`, `satiety`, `energy` or position; not the threatener's anything. No
  damage, no energy cost, no satiety transfer, no movement, no death.
- **`threaten` does not populate the suffered-attack record of `REQ-MOK-045`.** A threatened Mokiterion may not `fight`,
  `retreat` or `surrender` in answer, because it has not been attacked. It may propose `attack`, `avoid` or anything else
  its source chooses on the ordinary preconditions — a threat provokes by changing `fear`, not by unlocking verbs.
- **The threat is reported as one event carrying the transition it caused** — the threatener's identifier, the target's
  identifier, the increase applied and the target's resulting `fear`. As with `REQ-MOK-044`, this is what keeps the
  stream complete for a target that has already emitted its own `survival_changed` line for the tick.
- **Rule 12's own `fear` write is unaffected and still applies** to every living Mokiterion at its own opportunity. A
  Mokiterion may in one tick be threatened, be raised by rule 12 for perceiving somebody, or be lowered by rule 12 for
  perceiving nobody, in whichever order rule 2's turn order produces. Both writes saturate and neither is suppressed.

## Failure and boundary behavior

- A target already at `fear` `100` is threatened validly and its `fear` stays `100`. The action succeeds and its event
  reports an increase of `0` applied to a value already at the maximum; it is not a rejection, because the precondition
  was met.
- A target whose `fear` is lowered by rule 12 in the same tick, after being threatened, ends the tick with the composed
  value. A threat is not a floor and does not persist beyond the write.
- A threat against a Mokiterion that died earlier in the same tick is rejected under `REQ-MOK-043`.
- A threat at distance `2` or more is rejected on contact under `REQ-MOK-042`.
- Threatening the same target repeatedly across ticks is valid each time and saturates.
- No threat produces a configuration error, a runtime error or a non-zero exit code.

## Constraints

- **One constant, `30`, stated in `SPEC-MOK-001`** as its own named constant beside `FEAR_INCREASE` and `FEAR_DECREASE`
  rather than as a multiple of either. Its value is the product owner's; changing it is a governed change.
- **No entropy draw.**
- **No population aggregate read.** `REQ-MOK-050`.
- **`fear` is the only attribute any threat writes**, in this requirement and anywhere else. No rule outside rule 12,
  this requirement, and `INT-MOK-006`'s initialization writes `fear` at all.
- **The new rule is appended after `SPEC-MOK-001` rule 19**, not folded into rule 12, for `WO-MOK-010`'s renumbering
  reason and because rule 12 is about an agent's own decay while this is about one agent acting on another.
- No trait modifies a threat's force. Traits under `REQ-MOK-033` bias decisions, not resolutions.

## Acceptance examples

### Example: normal behavior

**Given** two living Mokiterions in contact and a target with `fear` below the maximum

**When** the threatener's valid `threaten` resolves

**Then** the target's `fear` rises by the stated constant, no other attribute of either Mokiterion changes, and one event
reports the increase and the resulting value.

### Example: saturation

**Given** a target at `fear` `100`

**When** a valid `threaten` resolves against it

**Then** the action succeeds, `fear` remains `100`, and the event reports the effective increase of `0`.

### Example: composition with rule 12

**Given** `M03` threatening `M07` during tick `t`, and `M07` perceiving another living Mokiterion at its own opportunity
later in tick `t`

**When** tick `t` completes

**Then** `M07`'s `fear` reflects both writes in turn order, saturating at `100`.

### Example: a threat is not an attack

**Given** a Mokiterion threatened and not attacked

**When** it reaches its next decision opportunity

**Then** its suffered-attack record is empty, and `fight`, `retreat` and `surrender` are rejected for it.

### Example: failure behavior

**Given** a `threaten` proposed against a Mokiterion at distance `2`, or against one that has died in the same tick

**When** it is validated

**Then** it is rejected naming the unmet precondition, the opportunity is consumed, and no `fear` changes as a result.

## Open decisions

- **The magnitude was decided by the product owner on 2026-08-20 and is `30`.** It is no longer open. Whether it equalled
  rule 12's `FEAR_INCREASE` of `10` was a decision and not a default, and it was taken the other way: a distinct constant
  says that being threatened differs from noticing a neighbour. It remains `SPEC-MOK-001`'s to state and the product
  owner's to approve as specification text, which is a separate act from approving this requirement.
- Whether the threat's event is its own kind or shares one with other targeted outcomes is the technical owner's, under
  `SPEC-MOK-003` rule 11 and `SPEC-MOK-002` rule 6.
