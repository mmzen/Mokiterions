+++
id = "REQ-MOK-010"
type = "requirement"
title = "Emit text observations"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN initialization, a material state change, food regeneration, territory crossing, death, or termination occurs, THE SYSTEM SHALL emit a plain-text event identifying its tick, subject, event type, and result."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Emit text observations

## Rationale

Operators and assurance reviewers need a simple way to reconstruct material simulation behavior without a user interface.

## Preconditions and trigger

The engine completes an observable foundation event.

## Required response

The system emits one ordered plain-text record containing the simulation tick, the relevant agent or territory, a stable event type, and the material result. Core events cover initialization, survival changes, consumption, regeneration, crossings, deaths, and termination. Per-action output is governed separately by `REQ-MOK-012`.

## Failure and boundary behavior

- Output failure is surfaced rather than silently discarding events.
- Events do not expose credentials or other secrets.
- Verbosity must remain bounded by the configured run length and event count.

## Constraints

- No graphical or web UI is required.
- Output ordering follows authoritative event ordering.
- The format remains deterministic for reproducibility checks.

## Acceptance examples

### Example: normal behavior

**Given** an agent crosses from territory A to territory B on tick 12

**When** the move is applied

**Then** the output contains tick 12, the agent identifier, a territory-crossing event type, and both territories.

### Example: failure behavior

**Given** the output destination fails while a core event is emitted

**When** the system attempts to write that event

**Then** the failure is surfaced and the run does not claim successful completion.

## Open decisions

None. Event vocabulary, field order, state-change behavior, and output destination are fixed by `SPEC-MOK-001`.
