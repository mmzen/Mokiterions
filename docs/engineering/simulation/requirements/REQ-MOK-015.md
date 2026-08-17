+++
id = "REQ-MOK-015"
type = "requirement"
title = "Provide a reference decision source"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN the operator selects the reference decision source, THE SYSTEM SHALL obtain every Mokiterion decision from an in-process deterministic source that seeks and consumes perceived food, without network access or credentials."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-002"]
+++

# Requirement: Provide a reference decision source

## Rationale

`REQ-MOK-014` obliges the world to sustain a population, but a survivor count is only meaningful relative to a
stated policy. A world measured with the random baseline reports the competence of random walking, not the
viability of the world. The reference decision source is therefore the instrument that makes viability
measurable, which is why it belongs with `REQ-MOK-014` rather than in a later phase.

It has a second purpose. Without a competent non-model control, a later comparison cannot distinguish emergent
behavior produced by a model from outcomes any adequate policy would produce. The reference source is that
control.

It is explicitly not an attempt at intelligent behavior. It is a development instrument and must be identifiable
as one.

## Preconditions and trigger

The operator selects the reference decision source, either explicitly or by relying on the default, and the
engine requires a decision for a living Mokiterion.

## Required response

- The engine obtains exactly one proposed action per decision opportunity from the reference source, through the
  same observation-to-proposed-action boundary used by the baseline source.
- The source consumes the perceived surroundings from `REQ-MOK-013` and pursues food: it consumes a co-located
  resource when consuming it is not wasteful, moves toward an attractive perceived resource, sustains itself
  when depleted, and searches when it perceives nothing. The precedence among these, and the point at which
  consuming becomes worthwhile, are fixed by `SPEC-MOK-001`.
- Selection among equally ranked candidates is resolved deterministically.
- The operator can select the random baseline source instead, and both remain available.
- The selected source is identified in the simulation's output so that a run is never ambiguous about which
  policy produced it.

## Failure and boundary behavior

- With no perceived food, the source proposes a valid action that changes the situation rather than idling. A
  policy that stands still while blind cannot find food at all, so it would not be a usable reference.
- The source proposes only actions the engine currently considers valid; a proposal is nonetheless subject to
  engine validation, and rejection is handled exactly as specified for any decision source.
- The source has no privileged access. It observes what any decision source observes, and it cannot mutate
  world state.
- Ranking preferences are policy, not world rules. Changing them alters no authoritative behavior.

## Constraints

- Fully deterministic. Identical configuration and seed produce identical decisions, preserving `REQ-MOK-009`.
  Where the source needs a choice it cannot derive from perception, it draws from the specified seeded entropy
  stream and never from an unseeded or ambient source.
- No network access, credentials, filesystem access, or external dependency.
- Must not receive mutable state, which would violate `REQ-MOK-004`.
- Must remain small and readable. It is a reference instrument, not a strategy engine, and must not accumulate
  emergent-behavior heuristics such as cooperation, avoidance, or territorial preference.
- Must be documented in output and developer documentation as a development reference rather than autonomous
  behavior.

## Acceptance examples

### Example: normal behavior

**Given** the reference source selected and a perceived resource four cells east of a Mokiterion

**When** the engine requests a decision

**Then** the source proposes a valid eastward move, and on a later tick, once co-located, proposes eating that
resource.

### Example: failure behavior

**Given** the reference source selected and no perceived food anywhere within the radius

**When** the engine requests a decision

**Then** the source proposes a valid action rather than an invalid or absent one, and the run continues without
error.

## Open decisions

None. The reference source as the default selection was decided by the repository owner on 2026-08-17. Its
ranking rules and tie-breaking order are fixed by `SPEC-MOK-001`.
