+++
id = "REQ-MOK-005"
type = "requirement"
title = "Apply core actions"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN a living Mokiterion selects a valid move, eat, sleep, or wait action, THE SYSTEM SHALL apply exactly that action once during the current tick."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Apply core actions

## Rationale

Movement, eating, sleeping, and waiting are the smallest action set capable of exercising the world and basic survival mechanics.

## Preconditions and trigger

A living agent has received a decision opportunity and the proposed action has passed engine validation.

## Required response

- `move` changes position by one cardinal coordinate and updates the current territory when the boundary is crossed.
- `eat` invokes the food-consumption behavior for a selected co-located resource.
- `sleep` restores configured energy and does not move or consume food.
- `wait` performs no action-specific state change.

Exactly one core action is applied for the decision opportunity.

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
