+++
id = "REQ-MOK-004"
type = "requirement"
title = "Enforce world authority"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN any decision source proposes an action, THE SYSTEM SHALL validate the action against authoritative world state before applying it and prevent the decision source from directly changing world state."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Enforce world authority

## Rationale

Separating decisions from authoritative state mutation prevents current and future model-backed decision sources from bypassing simulation rules.

## Preconditions and trigger

A living agent reaches its decision point and a decision source returns a proposed action.

## Required response

The engine evaluates the proposal using current authoritative state. A valid action is applied exactly once by the engine. An invalid action consumes the agent's turn, produces a rejection result, and causes no action-specific state mutation.

## Failure and boundary behavior

- Malformed, unsupported, stale, or impossible proposals are invalid.
- Validation failure never partially applies an action.
- Tick-level survival effects may still apply when an action is rejected.

## Constraints

- Decision sources receive observations rather than mutable world references.
- Every action-specific mutation is attributable to one validated engine operation.

## Acceptance examples

### Example: normal behavior

**Given** a living agent adjacent to an in-bounds coordinate

**When** its decision source proposes movement into that coordinate

**Then** the engine validates and applies the movement once.

### Example: failure behavior

**Given** an agent at the world edge

**When** its decision source proposes movement outside the world

**Then** the position remains unchanged and the action result is rejected; the result is printed when optional action tracing is enabled.

## Open decisions

None. Observation and proposal contracts are fixed by `SPEC-MOK-001`.
