+++
id = "REQ-MOK-018"
type = "requirement"
title = "Expose proposed action and engine authority decision"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN an operator selects a Mokiterion, THE SYSTEM SHALL display that Mokiterion's authoritative state together with the action its decision source proposed on the most recently completed tick and whether the engine accepted or rejected that proposal."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-003"]
+++

# Requirement: Expose proposed action and engine authority decision

## Rationale

`ADR-MOK-001` establishes that a decision source proposes and only the engine mutates, and that a rejected
proposal is a consumed action opportunity rather than a fault. That boundary is the project's central design
commitment and it is currently invisible in practice: an accepted move and a rejected move both appear as text
among thousands of lines, so the one thing the architecture exists to guarantee cannot be observed at the moment
it operates.

The requirement matters more with each later phase. A model-backed decision source is untrusted input that must
pass the same validation as the local baseline. Judging whether such a source behaves plausibly, and whether the
engine correctly refused it when it did not, requires seeing the proposal and the verdict side by side for a
chosen agent at a chosen tick. Without this, an operator can only observe that the world did not change and must
guess whether the source proposed nothing, proposed a wait, or proposed something illegal.

Selection is the mechanism because the information is per-agent and detailed. Presenting proposal and verdict for
all twelve simultaneously would either truncate the detail or displace the spatial view.

## Preconditions and trigger

An observed run has been initialized, at least one tick has completed, and the operator has selected a
Mokiterion. Selection persists across ticks until changed or cleared.

## Required response

For the selected Mokiterion, the display presents:

- its identifier, position, and territory;
- its health, satiety and energy;
- the action its decision source proposed on the most recently completed tick, including the target of that
  proposal where the action has one;
- whether the engine accepted or rejected the proposal;
- when the proposal was rejected, the engine's stated ground for rejection;
- the action the engine actually applied.

Accepted and rejected outcomes are visually distinguishable from each other without relying on colour alone.

When a selected Mokiterion dies, the display presents its death rather than continuing to show stale live state,
and the selection is handled by the rule declared in `SPEC-MOK-002` rather than left undefined.

## Failure and boundary behavior

- Before the first tick completes, the pane indicates that no proposal has yet been made rather than displaying an
  empty or fabricated proposal.
- When no Mokiterion is selected, the pane indicates that nothing is selected. It does not default to an
  arbitrary agent, since an operator could then read one agent's proposal as another's.
- A rejected proposal is presented as an expected outcome, not as an error condition or a warning about the
  program's own health.
- The displayed proposal and verdict are those of the tick most recently completed for that Mokiterion. The
  display must not present a proposal from one tick beside a verdict from another.

## Constraints

- The observer reads the proposal and the verdict from the engine. It does not re-derive, re-evaluate, or predict
  either, since a display that computed its own verdict could disagree with the engine and would then be
  asserting authority the engine holds.
- The observer receives no mutable handle to world, agent, resource, event-log, or engine state, consistent with
  `ADR-MOK-001`.
- Obtaining proposal and verdict information consumes no simulation entropy and does not alter any decision, in
  keeping with the existing obligation that action tracing be observational only.
- Pane content, layout, the accepted-and-rejected distinction, and selection behavior on death are fixed by
  `SPEC-MOK-002`.

## Acceptance examples

### Example: normal behavior

**Given** an observed run in which the selected Mokiterion is standing on a low-class resource at satiety 81

**When** the tick completes and a frame is drawn

**Then** the pane shows the proposed action as eating that resource, the engine outcome as accepted, and the
applied action as the same eat.

### Example: failure behavior

**Given** the selected Mokiterion is at the western edge of the world and its decision source proposes a move
further west

**When** the tick completes and a frame is drawn

**Then** the pane shows the proposed westward move, the engine outcome as rejected with the engine's stated
ground, and no applied movement — and the run continues normally.

## Open decisions

None. Pane content, the visual distinction between accepted and rejected, behavior before the first tick, and
selection behavior when the selected Mokiterion dies are fixed by `SPEC-MOK-002`.
