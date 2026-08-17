+++
id = "REQ-MOK-007"
type = "requirement"
title = "Regenerate food conditionally"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN a territory's food-regeneration delay expires, THE SYSTEM SHALL permit food to regenerate only when at least one food resource remains in that territory."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Regenerate food conditionally

## Rationale

The loss of regeneration after complete depletion is the central scarcity mechanism described by the product concept.

## Preconditions and trigger

A scheduled regeneration opportunity reaches its configured delay for one territory.

## Required response

If at least one resource exists in the territory, the engine applies the configured seeded regeneration rule. If no resource exists there, the opportunity creates no food.

## Failure and boundary behavior

- Food never regenerates outside the applicable territory or world bounds.
- A territory with zero food remains unable to regenerate food through this mechanism.
- Regeneration respects any configured resource capacity.

## Constraints

- Regeneration outcomes are reproducible from configuration and seed.
- Each regeneration or skipped regeneration is observable.

## Acceptance examples

### Example: normal behavior

**Given** a territory contains one food resource when its delay expires

**When** regeneration is processed

**Then** the configured seeded regeneration rule may add food within that territory.

### Example: failure behavior

**Given** a territory contains no food when its delay expires

**When** regeneration is processed

**Then** no food is added to that territory.

## Open decisions

None. Regeneration timing, amount, class selection, placement, capacity, and skipped opportunities are fixed by `SPEC-MOK-001`.
