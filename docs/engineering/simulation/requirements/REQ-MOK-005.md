+++
id = "REQ-MOK-005"
type = "requirement"
title = "Apply core actions"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-20"
statement = "WHEN a living Mokiterion selects a valid move, eat, sleep, or wait action, THE SYSTEM SHALL apply exactly that action once during the current tick."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Apply core actions

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-11 | Original approved content for `CAP-MOK-001`. | Approved; implemented under `WO-MOK-001` and verified under `VREC-MOK-001`. |
| 2026-08-20 | The four-verb enumeration is re-read as the **core** set rather than as the whole action contract, beside which `REQ-MOK-043` places seven targeted verbs. *Required response* gains that re-reading and its closing sentence is corrected from "Exactly one core action is applied for the decision opportunity" to a statement about one action of any kind. **Nothing this requirement obliges changes**: each of the four verbs keeps its effect word for word, the statement is untouched because its `WHEN` clause was already scoped to the four, the title already said *core*, and no acceptance example moves. | Approved 2026-08-20 by the repository owner acting as product owner, in the single act `WO-MOK-012`'s *Required amendments* section describes. This requirement is **amended and not superseded**, because it is cited by `CAP-MOK-001`, by `SPEC-MOK-001`'s and `VER-MOK-001`'s covered-requirement lists, by `WO-MOK-001`, by `SPEC-MOK-003` rule 11's authority table as the authority for `territory_crossed`, and by two locations in `mokiterions-tui`, two of those artifacts being released under `RLS-MOK-001`. It was an approval precondition of `WO-MOK-012` and is stated in full in that work order's *Required amendments* section. The implementation agent wrote the text and did not decide the substance. |

## Rationale

Movement, eating, sleeping, and waiting are the smallest action set capable of exercising the world and basic survival mechanics.

## Preconditions and trigger

A living agent has received a decision opportunity and the proposed action has passed engine validation.

## Required response

- `move` changes position by one cardinal coordinate and updates the current territory when the boundary is crossed.
- `eat` invokes the food-consumption behavior for a selected co-located resource.
- `sleep` restores configured energy and does not move or consume food.
- `wait` performs no action-specific state change.

Exactly one action is applied for the decision opportunity, and where that action is a core action it is exactly one of
the four above.

**These four are the core set, and they are not the whole action contract.** `REQ-MOK-043` places seven targeted verbs
beside them — `approach`, `avoid`, `threaten`, `attack`, `fight`, `retreat` and `surrender` — so a decision opportunity
may yield a targeted action instead of a core one, and then no core action is applied at all. That is why the sentence
above states one action rather than one core action. This requirement is unchanged in what it obliges: each of the four
verbs has exactly the effect stated, `move`'s territory crossing is still observable as an event, and the one-action
bound is still one action. What is corrected is a reading, not an obligation — this requirement never claimed the four
were exhaustive, and its title has always said *core*.

## Failure and boundary behavior

- Dead agents cannot act.
- Movement outside the world is invalid.
- Eating without a valid selected co-located resource is invalid.
- Normal tick-level survival effects still apply to sleeping and waiting agents.

## Constraints

- Territory boundaries do not block otherwise valid movement.
- A territory crossing is observable as an event.

## Acceptance examples

### Example: normal behavior

**Given** a living agent adjacent to the other territory

**When** it validly moves across the boundary

**Then** its position and current territory change and one crossing event is recorded.

### Example: failure behavior

**Given** a dead agent

**When** an action is proposed for it

**Then** the action is rejected and no action-specific state changes.

## Open decisions

None. Sleep recovery, action costs, co-location rules, and action representation are fixed by `SPEC-MOK-001`.
