+++
id = "REQ-MOK-013"
type = "requirement"
title = "Perceive the local surroundings"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN the engine builds an observation for a living Mokiterion, THE SYSTEM SHALL include every food resource and every other living Mokiterion within the configured perception radius, each with its relative direction and distance, in a stable order."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-002"]
+++

# Requirement: Perceive the local surroundings

## Rationale

An observation currently exposes only co-located food. A decision source therefore cannot locate a resource it
is not already standing on, which makes deliberate food acquisition impossible and reduces movement to an
undirected walk. Perception is the precondition for every behavioral phase that follows.

Nearby Mokiterions are included even though no requirement in this capability consumes them. This settles the
observation contract in one change rather than amending it again when fear dynamics arrive.

## Preconditions and trigger

The engine is building an observation for a living Mokiterion during its decision opportunity, before any
action is proposed.

## Required response

The observation includes, in addition to the fields already specified:

- every food resource whose position lies within the configured perception radius of the observer, each with
  its identifier, calorie class, relative direction, and distance;
- every other living Mokiterion within the same radius, each with its identifier, relative direction, and
  distance.

Both collections use a stable, documented ordering that does not depend on unordered collection iteration.
Distance uses the metric fixed by `SPEC-MOK-001`. The observer never appears in its own list of nearby
Mokiterions.

## Failure and boundary behavior

- An empty radius yields empty collections rather than an error.
- Resources and Mokiterions outside the radius are absent, not reported at a truncated distance.
- Perception does not stop at a territory boundary. A boundary labels state and does not block observation.
- Perception is read-only and never mutates world state, entropy, or the observed entities.
- Dead Mokiterions never appear in perception.
- Co-located resources continue to be reported as they were before this requirement, and remain valid `eat`
  targets.

## Constraints

- Perception must not consume entropy, so it cannot alter reproducibility.
- The observation remains an immutable value. Adding perception must not expose a handle to mutable state,
  which would violate `REQ-MOK-004`.
- Perception adds no ability to act at a distance. It reports; it does not reach.

## Acceptance examples

### Example: normal behavior

**Given** a Mokiterion at `(40, 20)` and a medium resource at `(44, 20)` with a perception radius of `16`

**When** the engine builds that Mokiterion's observation

**Then** the observation lists the resource with its identifier, class, an easterly relative direction, and a
distance of `4`, and the Mokiterion is not listed among nearby Mokiterions.

### Example: failure behavior

**Given** a Mokiterion at `(40, 20)`, a resource at `(100, 20)`, and a dead Mokiterion at `(41, 20)`

**When** the engine builds that Mokiterion's observation

**Then** the distant resource is absent because it lies outside the radius, the dead Mokiterion is absent, and
both collections are empty rather than erroneous.

## Open decisions

None. Perception scope covering both resources and Mokiterions was decided by the repository owner on
2026-08-17. The radius value and distance metric are fixed by `SPEC-MOK-001`.
