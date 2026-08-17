+++
id = "REQ-MOK-012"
type = "requirement"
title = "Trace every action optionally"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN a simulation runs with action tracing enabled, THE SYSTEM SHALL print exactly one action-trace line for every living Mokiterion decision opportunity on every tick."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Trace every action optionally

## Rationale

Developers and operators need to see what every Mokiterion attempted during each turn so they can debug simulation rules and understand observed behavior without making verbose action output mandatory for ordinary runs.

## Preconditions and trigger

The operator starts the simulation with the `--trace-actions` flag and a living Mokiterion reaches a decision opportunity.

## Required response

After the proposal has been validated and any valid action-specific mutation has been applied, the system prints exactly one deterministic action-trace line. It identifies the tick, Mokiterion, proposed action, acceptance or rejection, result or rejection reason, position, territory, health, satiety, and energy.

## Failure and boundary behavior

- Rejected proposals still produce one trace line when tracing is enabled.
- Dead Mokiterions receive no decision opportunity and therefore produce no later action trace.
- When `--trace-actions` is absent, no action-trace lines are printed; required core events and the final summary remain available.
- The flag accepts no value and duplicate occurrences are invalid configuration.

## Constraints

- Tracing is observational and cannot change decisions, entropy consumption, state transitions, or final state.
- Trace ordering follows authoritative agent processing order.
- Trace content is reproducible for identical configuration and seed.

## Acceptance examples

### Example: normal behavior

**Given** action tracing is enabled and twelve Mokiterions are alive at the start of a tick

**When** all twelve decision opportunities complete

**Then** exactly twelve ordered action-trace lines are printed for that tick.

### Example: failure behavior

**Given** action tracing is disabled

**When** a simulation tick completes

**Then** no action-trace line is printed and core events are still emitted normally.

## Open decisions

None. The flag and trace contract are fixed by `SPEC-MOK-001`.
