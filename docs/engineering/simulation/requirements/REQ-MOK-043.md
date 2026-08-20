+++
id = "REQ-MOK-043"
type = "requirement"
title = "Apply targeted actions"
status = "draft"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a living Mokiterion selects a valid approach, avoid, threaten, attack, fight, retreat, or surrender action naming another Mokiterion, THE SYSTEM SHALL validate it against authoritative state and apply exactly that action once during the current tick, or reject it without any action-specific mutation."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Apply targeted actions

## Rationale

`REQ-MOK-005` obliges the system to apply a valid move, eat, sleep or wait action exactly once. Those four are the
*core* set: each acts on the world or on the actor, and none names another Mokiterion. This requirement states the same
obligation for the seven actions that do name one, and it is deliberately a second requirement beside `REQ-MOK-005`
rather than a replacement for it.

**`REQ-MOK-005` is amended, not superseded.** Its four-verb enumeration is re-read as the core set; the requirement
itself stays, keeps its identifier, and keeps every citation. Those citations are the reason: `CAP-MOK-001`,
`SPEC-MOK-001`'s and `VER-MOK-001`'s covered-requirement lists, `WO-MOK-001`, `SPEC-MOK-003` rule 11's authority table
where it authorizes `territory_crossed`, and two locations in `mokiterions-tui`. `VER-MOK-001` and `WO-MOK-001` are
released under `RLS-MOK-001`. Retiring an identifier that a released verification contract covers, to buy a tidier
statement, is the trade `WO-MOK-010` declined when it appended rule 19 rather than renumbering thirteen rules.

The seven divide into three groups that share one obligation and differ in precondition. `approach` and `avoid` name a
*perceived* Mokiterion and resolve as movement, so they need perception and not contact — a Mokiterion that can only
approach what it is already touching cannot close distance, and closing distance is what makes the rest of this
capability reachable at a contact rate of `0.0060` per agent-tick. `threaten` and `attack` require contact.
`fight`, `retreat` and `surrender` require that the acting Mokiterion has been attacked since its last opportunity,
which is `REQ-MOK-045`'s field.

The obligation itself is `REQ-MOK-005`'s, unchanged in shape: exactly once, or rejected with nothing mutated. That
matters more here than it did there, because a targeted action mutates a *second* Mokiterion, and a partially applied
one would leave two Mokiterions inconsistent rather than one.

## Preconditions and trigger

The trigger is a living Mokiterion's decision opportunity under `SPEC-MOK-001` rule 2, returning one of the seven
actions with a named target.

The target is named by identifier. A proposal naming no target, or a target that is not a Mokiterion, is not
representable in the action contract rather than rejected at validation.

Every precondition is checked against authoritative state at validation time, never against the observation the source
read. The two can differ within one tick, because an earlier-acting Mokiterion may have moved or died since.

## Required response

- **Validation precedes mutation.** The engine checks the acting Mokiterion is living, the target exists and is living,
  the target is not the actor, and the action's own precondition holds — perception for `approach` and `avoid`, contact
  for `threaten` and `attack`, an unanswered attack for `fight`, `retreat` and `surrender`.
- **A valid action is applied exactly once**, during the current tick, at the acting Mokiterion's own opportunity. No
  targeted action applies twice, applies to a second target, or defers its own effect to a later tick.
- **A rejected proposal consumes the action opportunity, produces a rejection result naming the unmet precondition, and
  causes no action-specific mutation** — to either Mokiterion. This is `SPEC-MOK-001` rule 6 and it is not relaxed for
  targeted actions; it is strengthened, in that the absence of mutation is an obligation about two agents.
- **`approach` moves one cell toward the target and `avoid` moves one cell away from it**, on the cardinal-move contract
  `SPEC-MOK-001` rule 8 already fixes, so a targeted move is a move: one cell, one axis, no additional energy cost, and
  a territory crossing emits its event exactly as it does today. Neither may move onto an invalid coordinate; where the
  preferred axis is invalid, the other axis is used, on rule 5 case 3's precedent.
- **`avoid` on a co-located target moves one cell in a specified direction** rather than rejecting, because "away from
  distance zero" has no direction and a rejection would make co-location an inescapable state.
- **The engine reports every targeted action's outcome in the event stream**, and every event it emits carries the state
  transitions that action caused, including transitions to the *target*. This is an obligation and not a convenience:
  `SPEC-MOK-001` rule 2 runs each Mokiterion's whole cycle inside its own turn, so a Mokiterion acted on by a
  higher-identified one has already emitted its `survival_changed` line for that tick and the change would otherwise
  appear in no record.
- **The action trace, when enabled, reports targeted actions on the same terms as core ones** — one line per decision
  opportunity, after any valid mutation and before survival decay, per `SPEC-MOK-001` rule 7.

## Failure and boundary behavior

- A proposal against a dead Mokiterion is rejected. This is reachable within a tick, not only across ticks.
- A proposal against a Mokiterion outside the action's required range is rejected; the range is the action's, so a
  `threaten` at distance `2` is rejected while an `approach` at distance `2` is valid.
- A proposal naming the acting Mokiterion itself is rejected. It is not representable through the observation, which
  never lists the observer, but validation does not rely on that.
- `fight`, `retreat` or `surrender` proposed by a Mokiterion that has not been attacked since its last opportunity is
  rejected. These are answers, and there is nothing to answer.
- A dead Mokiterion proposes nothing, receives no observation and takes no action opportunity, per `SPEC-MOK-001`
  rule 13. Nothing here changes that.
- No targeted action produces a configuration error, a runtime error or a non-zero exit code.

## Constraints

- **Eleven action kinds, and the contract stays closed.** The set is fixed by `SPEC-MOK-001`; there is no extension
  point, no generic "interact" verb and no data-driven action table.
- **No entropy.** Validation and application of a targeted action take no draw from the shared stream. Where a source
  needs to choose among equally valid targets it does so by a stated deterministic order, not by a selection.
- **No population aggregate is read** by any precondition, target selection, ordering or tie-break. `REQ-MOK-050` states
  this generally.
- **Acting order is unchanged.** Ascending identifier order under `SPEC-MOK-001` rule 2. A targeted action does not
  grant the target an out-of-order opportunity, does not re-enter the decision loop, and does not let one Mokiterion act
  twice in a tick.
- **Tie-breaks are specified, not incidental.** Where two candidate targets are equally eligible, the lowest identifier
  is chosen, on the precedent of rule 5 cases 1 and 3.
- The engine's dependency table stays empty.

## Acceptance examples

### Example: normal behavior

**Given** two living Mokiterions in contact

**When** the earlier-identified one proposes `attack` naming the other

**Then** the action is applied exactly once, its costs are applied to both parties as `REQ-MOK-044` specifies, and one
event reports the outcome together with the transitions it caused.

### Example: approach needs perception, not contact

**Given** two living Mokiterions at Chebyshev distance `9`

**When** one proposes `approach` naming the other

**Then** the proposal is valid and it moves one cell toward the target; a `threaten` at the same distance is rejected.

### Example: the target dies earlier in the same tick

**Given** `M03` whose observation listed `M07` in contact, and `M07` reduced to zero health during `M05`'s turn

**When** `M03`'s `attack` on `M07` is validated

**Then** it is rejected naming the unmet precondition, `M03`'s opportunity is consumed, and neither Mokiterion's state
changes.

### Example: an answer with nothing to answer

**Given** a living Mokiterion that has not been attacked since its last opportunity

**When** it proposes `fight`, `retreat` or `surrender`

**Then** each is rejected, the opportunity is consumed, and no state changes.

### Example: avoiding from co-location

**Given** two living Mokiterions on one coordinate

**When** one proposes `avoid` naming the other

**Then** it moves one cell in the direction `SPEC-MOK-001` fixes for that case, rather than the proposal being rejected.

### Example: failure behavior

**Given** any rejected targeted proposal

**When** the tick completes

**Then** neither Mokiterion's `health`, `satiety`, `energy`, `fear` or position changed as a result of it, the rejection
is reported with its reason, and the run's exit code is unaffected.

## Open decisions

- The direction `avoid` takes from a co-located target is `SPEC-MOK-001`'s to fix. It must be deterministic and must not
  consume entropy.
- Whether `approach` and `avoid` emit their own event kind or reuse the movement path's reporting **was** the technical
  owner's, constrained by `SPEC-MOK-003` rule 11 requiring every event type to map to an authorizing requirement, and by
  `SPEC-MOK-002` rule 6 counting the public interface growth that a new `EventType` variant is. **It was decided on
  2026-08-20: they reuse the movement path's reporting and emit no event kind of their own**, together with `retreat`, on
  the ground that each mutates only the acting Mokiterion and so carries no transition rule 7's `action_trace` does not
  already report. This requirement's seven verbs therefore map to three new event types rather than seven, and none of
  the three is authorized by this requirement — `attack_resolved` by `REQ-MOK-044`, `threat_resolved` by `REQ-MOK-046`,
  `surrender_resolved` by `REQ-MOK-047`. It is no longer open, and it remains `SPEC-MOK-001`'s and `SPEC-MOK-002`'s to
  state as text.
