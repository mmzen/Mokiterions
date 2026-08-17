+++
id = "REQ-MOK-009"
type = "requirement"
title = "Produce reproducible entropy"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN two simulations use identical configuration and entropy seed, THE SYSTEM SHALL produce identical initialization, baseline decisions, state transitions, events, and final state."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Produce reproducible entropy

## Rationale

Controlled behavioral variation must remain reproducible so failures and outcomes can be verified and reconstructed.

## Preconditions and trigger

Two simulations start with identical versioned code, configuration, entropy seed, and initial external inputs.

## Required response

The simulations consume entropy deterministically and produce equivalent initial state, baseline action proposals, authoritative transitions, event order, and final state.

## Failure and boundary behavior

- Wall-clock time, thread scheduling, hash iteration order, and environment-specific values must not alter comparable outcomes.
- Different seeds may produce different valid outcomes.

## Constraints

- Every stochastic foundation behavior draws from the explicitly seeded simulation entropy source.
- Comparable text output contains no nondeterministic timestamps or paths.

## Acceptance examples

### Example: normal behavior

**Given** two 100-tick runs with the same configuration and seed

**When** both complete

**Then** their event output and final state are identical.

### Example: failure behavior

**Given** otherwise identical runs with different seeds

**When** both complete

**Then** they may differ, but each remains internally valid.

## Open decisions

None. The generator, seed representation, and entropy ordering are fixed by `SPEC-MOK-001`.
