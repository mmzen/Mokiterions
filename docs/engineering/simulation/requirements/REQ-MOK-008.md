+++
id = "REQ-MOK-008"
type = "requirement"
title = "Provide a baseline decision source"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN the foundation simulation requires an agent decision, THE SYSTEM SHALL obtain one currently valid core action from an in-process seeded baseline decision source without network access or credentials."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Provide a baseline decision source

## Rationale

The engine must be runnable and testable before external model integration while keeping decisions separate from world authority.

## Preconditions and trigger

A living agent reaches its decision point in the foundation configuration.

## Required response

The baseline receives a read-only bounded observation and uses the simulation's seeded entropy to select one currently valid action from `move`, `eat`, `sleep`, and `wait`. It returns a proposal to the engine and never mutates world state directly.

## Failure and boundary behavior

If no context-specific action other than `wait` is available, the baseline returns `wait`. It does not contact a network service, read credentials, or claim to provide final LLM-backed behavior.

## Constraints

- The baseline remains deterministic for identical observations and entropy state.
- It is a development mechanism, not a hard-coded product strategy.
- OpenAI GPT nano integration is outside this requirement.

## Acceptance examples

### Example: normal behavior

**Given** a living agent with at least one valid core action

**When** a decision is requested

**Then** the baseline returns one valid proposal and the engine independently validates it.

### Example: failure behavior

**Given** a local run without network connectivity or model credentials

**When** decisions are requested

**Then** the simulation continues using the baseline without an external-service error.

## Open decisions

None. Observation fields and seeded baseline selection are fixed by `SPEC-MOK-001`.
