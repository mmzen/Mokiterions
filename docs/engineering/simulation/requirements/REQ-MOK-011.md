+++
id = "REQ-MOK-011"
type = "requirement"
title = "Terminate cleanly"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN the configured tick limit is reached or every Mokiterion is dead, THE SYSTEM SHALL terminate cleanly and emit a final simulation summary."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Terminate cleanly

## Rationale

A bounded run and stable summary make the foundation practical to execute, test, and compare.

## Preconditions and trigger

The simulation reaches its configured tick limit or its living population becomes zero.

## Required response

The engine stops scheduling decisions and ticks, emits a final summary, and exits successfully. The summary includes the termination reason, elapsed ticks, survivor and death counts, population by current territory, and remaining food by territory and calorie class.

## Failure and boundary behavior

- The final summary is emitted once.
- No agent acts after a termination condition is reached.
- Invalid startup configuration exits unsuccessfully and is not reported as a completed simulation.

## Constraints

- A tick limit is mandatory for foundation runs.
- Summary ordering and values remain deterministic.

## Acceptance examples

### Example: normal behavior

**Given** a simulation configured for 100 ticks with surviving agents

**When** tick 100 completes

**Then** the simulation stops and reports tick-limit termination with the required counts.

### Example: failure behavior

**Given** all agents die before the tick limit

**When** the last death is applied

**Then** the simulation stops without scheduling another decision and reports extinction termination.

## Open decisions

None. Tick defaults, command-line configuration, summary format, and exit codes are fixed by `SPEC-MOK-001`.
