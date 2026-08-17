+++
id = "REQ-MOK-002"
type = "requirement"
title = "Initialize Mokiterions"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN the world is initialized, THE SYSTEM SHALL create twelve uniquely identified living Mokiterions, place six in each territory, and assign each a valid position, health, satiety, and energy."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Initialize Mokiterions

## Rationale

Twelve agents split evenly between the two territories are the minimum population defined by the product concept.

## Preconditions and trigger

A valid world has been created and contains enough valid coordinates for the configured initial placement.

## Required response

The system creates twelve living agents with stable unique identifiers. Six begin in each territory. Every initial attribute is within its declared bounds and every initial position is inside the assigned territory.

## Failure and boundary behavior

If a complete valid population cannot be placed, initialization fails atomically and reports the reason.

## Constraints

- Initialization is reproducible from the configured seed.
- No agent begins dead or outside the world.
- Fear is outside this foundation and is not required by this requirement.

## Acceptance examples

### Example: normal behavior

**Given** a valid initialized world

**When** the population is created

**Then** twelve living agents exist, identifiers are unique, and each territory contains six starting agents.

### Example: failure behavior

**Given** a placement configuration that cannot create all twelve valid agents

**When** population initialization is attempted

**Then** no partial population is retained and the program reports the failure.

## Open decisions

None. Attribute ranges, initial values, placement rules, and co-location behavior are fixed by `SPEC-MOK-001`.
