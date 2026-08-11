+++
id = "REQ-MOK-001"
type = "requirement"
title = "Initialize the world"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"
statement = "WHEN a simulation starts with valid configuration, THE SYSTEM SHALL create a 128 by 128 world divided into two equal territories and place low-, medium-, and high-calorie food in both territories."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: Initialize the world

## Rationale

The foundational simulation needs the world dimensions, territorial split, and resource classes established by the product concept.

## Preconditions and trigger

The operator starts a simulation with a valid entropy seed, tick limit, and world configuration.

## Required response

The system creates exactly 16,384 addressable coordinates, assigns every coordinate to exactly one of two equal territories, and places at least one resource of each calorie class in each territory.

## Failure and boundary behavior

Invalid configuration prevents the simulation from starting, produces a readable error, and creates no partial world state.

## Constraints

- Coordinates remain within the 128 by 128 bounds.
- Territories are complete, equal, non-overlapping partitions of the world.
- Initial placement is derived from the configured entropy seed.

## Acceptance examples

### Example: normal behavior

**Given** a valid configuration and seed

**When** the simulation starts

**Then** it contains two territories of 8,192 coordinates each and every food class is present in both territories.

### Example: failure behavior

**Given** an invalid tick limit or seed configuration

**When** initialization is attempted

**Then** the program reports the invalid field and does not start a simulation.

## Open decisions

None. Initial quantity, placement rules, and food capacity are fixed by `SPEC-MOK-001`.
