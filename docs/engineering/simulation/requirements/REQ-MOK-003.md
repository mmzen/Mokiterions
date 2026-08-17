+++
id = "REQ-MOK-003"
type = "requirement"
title = "Advance simulation time"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN a simulation tick advances, THE SYSTEM SHALL process each living Mokiterion once in stable order, apply survival decay, reduce health under critical survival conditions, and mark a Mokiterion dead when health reaches zero."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Advance simulation time

## Rationale

Time-dependent satiety, energy, health, and death create the minimum survival pressure required by Mokiterions.

## Preconditions and trigger

The simulation is running and has not reached a termination condition.

## Required response

For one tick, the engine processes every agent that was living at its scheduled turn exactly once, applies its action and survival updates in the specified order, decreases satiety and energy at configured rates, decreases health when a critical survival condition applies, and immediately marks health-zero agents dead.

## Failure and boundary behavior

- Attributes never fall below their minimum or rise above their maximum.
- A dead agent receives no later decision or action.
- An error while preparing a tick must not leave a partially applied tick presented as complete.

## Constraints

- Processing order is stable and deterministic.
- Time advances in discrete integer ticks.

## Acceptance examples

### Example: normal behavior

**Given** a living agent with non-critical survival attributes

**When** one tick completes

**Then** the agent is processed once and its satiety and energy reflect one configured interval of decay.

### Example: failure behavior

**Given** an agent whose health reaches zero during its turn

**When** the remainder of the tick is processed

**Then** that agent is dead and receives no additional action.

## Open decisions

None. Update ordering, decay rates, critical conditions, health damage, and attribute bounds are fixed by `SPEC-MOK-001`.
