+++
id = "REQ-MOK-027"
type = "requirement"
title = "Display run provenance and the authority for each event type"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-22"
statement = "WHEN the observer displays simulation state or events, THE SYSTEM SHALL display alongside them the run's entropy seed, tick limit, resource density and active decision source, and SHALL name, for any displayed event type, the requirement that authorizes the behavior that event reports."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: Display run provenance and the authority for each event type

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-22 | **No normative provision changed. The statement, the required response, the constraints, the failure behaviour and both acceptance examples are untouched, and the requirement continues to oblige exactly what it obliged before.** What changes is *Open decisions*, which now records a matter deferred rather than open: `SPEC-MOK-003` rule 8 as amended on 2026-08-22 sheds provenance fields in a specified order when the footer row cannot hold them, and clause 6 concedes that at the declared floor of `34 × 22` a twenty-digit entropy seed beside a twenty-digit tick limit is 43 characters of 34 — so at that viewport this requirement is **not met**, and the concession is accepted knowingly instead of being closed. The statement is deliberately **not** narrowed to what the floor can hold. | **Accepted 2026-08-22 by the repository owner acting as accountable product owner**, the role `SPEC-MOK-003`'s rule 8 amendment row and `VER-MOK-005`'s *Residual uncertainty* both name for this residual, with the three courses those artifacts enumerate put alongside acceptance and the cost of each measured: raising rule 5's floor to the 50, 64 or 84 columns that carry all six fields, which is what `WO-MOK-008` names as its escalation trigger and which moves rule 5, `REQ-MOK-024` and two `VER-MOK-005` viewport rows; narrowing the accepted seed range; or amending this statement. None was taken. The implementation agent wrote the text, measured the arithmetic and decided nothing. The act is not covered by `VREC-MOK-021`, which is `verified` and bound to `3da6acc`, a commit that predates it. |

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
  reached by the operator are fixed by `SPEC-MOK-003`.
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
by `SPEC-MOK-003`.

One matter is deferred rather than open, and it is a concession against this requirement rather than a gap in it.
`SPEC-MOK-003` rule 8 as amended on 2026-08-22 sheds fields in a specified order when the footer row cannot hold them,
and clause 6 concedes that at the declared floor of `34 × 22` a twenty-digit entropy seed beside a twenty-digit tick
limit is 43 characters of 34, so what the floor guarantees is the entropy seed alone. This requirement states that
provenance is never the information sacrificed to degradation, and at that viewport it is not met. **It is accepted
knowingly by the product owner on 2026-08-22**, recorded in `VER-MOK-005` as residual uncertainty and in
`SPEC-MOK-003`'s amendment row of that date. The statement above is left exactly as it stands: the requirement goes on
obliging the whole of what it obliged, because narrowing it to what the narrowest viewport can hold would remove the
pressure that any future widening answers. Three courses were put alongside acceptance and none was taken — raising
rule 5's floor to the 50, 64 or 84 columns that carry all six fields, narrowing the accepted seed range, or amending
this statement — and the first is what `WO-MOK-008` names as its escalation trigger, moving rule 5, `REQ-MOK-024` and
two `VER-MOK-005` viewport rows. **The acceptance is void if the floor, the accepted seed range or rule 8's field set
changes**, and any future requirement that widens the observer's declared viewports must address it.
