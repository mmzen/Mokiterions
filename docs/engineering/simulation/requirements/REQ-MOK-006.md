+++
id = "REQ-MOK-006"
type = "requirement"
title = "Consume food"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN a living Mokiterion validly eats a selected food resource at its position, THE SYSTEM SHALL atomically remove that resource and restore satiety and energy according to its calorie class without exceeding configured maximums."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Consume food

## Rationale

Food must be scarce, consumable, and beneficial for the survival loop to function.

## Preconditions and trigger

A living agent selects an existing food resource located at the agent's current coordinate and proposes `eat`.

## Required response

The engine removes exactly the selected resource and applies the calorie class's configured satiety and energy restoration, capped at each attribute's maximum. Removal and restoration are one authoritative operation.

## Failure and boundary behavior

- A missing, stale, or non-co-located resource selection is invalid.
- Invalid consumption changes neither the resource collection nor the agent's action-specific attributes.
- One resource can be successfully consumed only once.

## Constraints

- The world contains low-, medium-, and high-calorie resource classes.
- Resource identity remains stable until consumption or another authoritative removal.

## Acceptance examples

### Example: normal behavior

**Given** a hungry agent co-located with a medium-calorie resource

**When** it validly eats that resource

**Then** the resource disappears and the agent gains the configured satiety and energy without exceeding maximums.

### Example: failure behavior

**Given** two agents attempt to eat the same resource in stable processing order

**When** the first consumption succeeds

**Then** the second attempt is rejected because the resource no longer exists.

## Open decisions

None. Calorie values and attribute maximums are fixed by `SPEC-MOK-001`.
