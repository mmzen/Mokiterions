+++
id = "REQ-MOK-024"
type = "requirement"
title = "Display run provenance and the authority for each event type"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN the observer displays simulation state or events, THE SYSTEM SHALL display alongside them the run's entropy seed, tick limit, resource density and active decision source, and SHALL name, for any displayed event type, the requirement that authorizes the behavior that event reports."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-003"]
+++

# Requirement: Display run provenance and the authority for each event type

## Rationale

Two distinct problems are solved by putting provenance on the screen.

**A screen capture must identify its own run.** The most likely way an observation enters a discussion is as an
image or a pasted pane. Without the seed and configuration beside it, such a capture is unfalsifiable: nobody can
reproduce it, and nobody can tell whether it came from the default density or a favourable one. Per-seed survivor
counts are demonstrably non-monotonic in density, so an observation whose density is unknown supports no
comparison at all. Provenance on screen makes every capture a reproducible claim.

**An operator should be able to tell authorized behavior from incidental behavior.** The project's whole premise is
that behavior traces to approved artifacts, and `ENGINEERING_HARNESS.md` makes typed artifacts the only carriers of
authority. When an operator sees a Mokiterion refuse a feast it is standing on, the useful question is whether that
is specified behavior or an accident. Naming the requirement behind each event type answers it at the point of
confusion instead of sending the operator to search the documentation, and it makes the traceability chain
something the operator uses rather than something that exists in a directory.

The second half is deliberately scoped to event *types* rather than event instances. A type-to-requirement mapping
is a finite, reviewable table that can be verified as complete and correct. Deriving authority per instance would
require the observer to reason about which artifact governs a particular occurrence, which is a judgement the
observer is not entitled to make.

## Preconditions and trigger

The observer is drawing a frame for an initialized run.

## Required response

**Provenance.** The entropy seed, the configured tick limit, the resource density, the active decision source, and
the current tick are displayed with the run, in a position that remains present at every viewport size at which the
observer draws.

**Authority.** For any event type the observer presents, the operator can obtain the identifier of the requirement
that authorizes the behavior that event reports. The mapping covers every event type the observer can display, and
it is declared rather than inferred.

Where the run's provenance includes the commit under which it was built, and that value is available to the
observer without reading the repository at run time, it is displayed too. Where it is not available, the field is
absent rather than filled with a guess.

## Failure and boundary behavior

- Provenance is never the information sacrificed to degradation. If the layout must shed content, provenance is
  retained.
- Every displayable event type has a mapping. An event type without one is a defect in the mapping rather than an
  event displayed with unknown authority, and the observer states that the mapping is missing rather than
  displaying a plausible identifier.
- Displayed provenance is the configuration actually in force for the run, read from the engine's configuration
  rather than re-parsed from operator input, so that a defaulted value and an explicitly supplied value display
  identically.
- No credential, secret, absolute filesystem path, environment variable, or wall-clock timestamp appears in
  provenance.

## Constraints

- The provenance fields, their placement, the complete event-type-to-requirement mapping, and how the mapping is
  reached by the operator are fixed by `SPEC-MOK-002`.
- The mapping names requirement identifiers only. The observer does not restate requirement text, since a restated
  obligation could drift from the artifact that holds it and `SIMULATION_RULES.md` already carries the standing rule
  that only the artifact is binding.
- The observer does not read repository files at run time to build the mapping or to obtain provenance.
- Displaying provenance consumes no simulation entropy and does not mutate simulation state.

## Acceptance examples

### Example: normal behavior

**Given** an observed run started with seed 42, a 1,000-tick limit, the default density and the reference decision
source

**When** any frame is drawn

**Then** the seed, tick limit, density, decision source and current tick are visible with the run.

### Example: failure behavior

**Given** the operator asks for the authority behind a food-regeneration-skipped event

**When** the mapping is consulted

**Then** the identifier of the requirement authorizing conditional regeneration is displayed, and no requirement
text is restated in place of the identifier.

## Open decisions

None. Provenance fields and placement, the event-type-to-requirement mapping, and the means of reaching it are fixed
by `SPEC-MOK-002`.
