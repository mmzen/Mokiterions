+++
id = "REQ-MOK-049"
type = "requirement"
title = "State the enlargement that restores an excluded pane, not only the key that borrows it"
status = "draft"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN the terminal observer excludes a pane because the viewport is too small, THE SYSTEM SHALL state the axis and the value at which that pane returns, in addition to the key that opens it as an overlay, and SHALL distinguish that statement from unemphasised text."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: State the enlargement that restores an excluded pane, not only the key that borrows it

## Rationale

`SPEC-MOK-003` rule 5 already obliges the observer to announce an excluded pane, and the implementation conforms. This
requirement is narrower than the finding that produced it, and the narrowing is deliberate.

The product owner reported on 2026-08-20 that a hidden pane is not indicated and asked for a permanent notice. **The
first half of that report is not correct and was not transcribed as though it were.** Rule 5's Announcement obligation
exists at line 405, it is permanent, and `announcement_text` in `render.rs` implements it: at `120 × 48` the header
reads `overlays: inspector i`. What the owner encountered was a notice that did not read as one. Two defects are real:

1. **It names the wrong remedy.** The notice says which key borrows the pane as an overlay. It does not say the terminal
   is too small, which axis is short, or what value would restore the pane to the layout. An operator who can resize
   their terminal is told to press a key instead — and the overlay is a full-body overlay, so it covers the view to show
   the inspector, which is not the same as having both.
2. **It carries no emphasis.** `render.rs:178` builds it with `Span::raw`, so it renders identically to the speed,
   zoom, selection and filter segments beside it. It is an obligation sharing a visual channel with optional detail,
   which is exactly the arrangement `status_line`'s own comment says it reserves space to avoid.

Rule 5 states the thresholds — roster `W ≥ 100`, inspector `W ≥ 140`, log `H ≥ 38` — and the operator cannot see them.
The information needed to act is in the specification and absent from the screen. The full measurement is in
`evidence/WO-MOK-012/adverse-observations.md`.

## Preconditions and trigger

Any pane that `SPEC-MOK-003` rule 5's pane table excludes at the current viewport, at any viewport above the floor.
Today that is the roster below `W = 100`, the inspector below `W = 140`, and the log below `H = 38`.

The trigger is exclusion, not the operator noticing it. Below the floor the observer writes the current and required
dimensions to standard error and exits `2`, which rule 5 already specifies and this requirement does not change — that
path already states the enlargement, and this requirement brings the in-terminal case up to it.

## Required response

For each excluded pane the announcement carries, in whatever order and wording rule 5 fixes:

1. **The axis and the value.** Which of `W` or `H` is short, and the value at which the pane returns — `W ≥ 140` for the
   inspector, `W ≥ 100` for the roster, `H ≥ 38` for the log. The values are rule 5's own thresholds, read from the
   layout rather than restated, so the two cannot drift.
2. **The overlay key**, which the notice already carries and which is not removed. Both remedies are stated because the
   overlay is available now and the enlargement is what restores the layout.
3. **Visual emphasis** distinguishing the announcement from the unemphasised optional segments beside it.

## Failure and boundary behavior

- **Emphasis is not the sole carrier.** `SPEC-MOK-003` rule 2.5 applies: the announcement's words carry it fully
  without colour or modifier, and the emphasis is redundant reinforcement. An announcement legible only when styled
  would fail rule 2.5 and this requirement together.
- **Where the viewport is too narrow for both remedies**, the axis and value are kept and the words are abbreviated,
  down to a shortest form that still carries a dimension. The existing `fit` mechanism already chooses between a long
  and a short form per segment, and the short form is where this obligation is tightest. Which of the two remedies
  survives last is a presentation decision that belongs to `SPEC-MOK-003` rule 5, not to this requirement.
- **Several panes excluded at once** are all announced. At `34 × 22` all three are excluded, which is the tightest case
  in both axes, and the announcement remains legible there or the requirement is not met.
- **A threshold value that no longer matches rule 5** is a defect. If the announcement states `W ≥ 140` while the
  layout admits the inspector at some other width, the observer is misinforming the operator about its own behavior,
  which is worse than the notice it replaces.
- **Monotonicity is untouched.** This requirement changes what is said about an excluded pane and not which panes any
  viewport presents, so rule 5's monotonicity obligation neither gains nor loses a case.

## Constraints

- **The announcement stays an obligation, ahead of optional detail.** It must not become droppable to make room for the
  hint of `REQ-MOK-048`, and `status_line`'s existing reservation of its width before any optional segment is the
  behavior this requirement depends on.
- **No new threshold and no new pane.** Rule 5's pane table is read, not written. This requirement introduces no
  viewport at which a pane's presence changes.
- **The values are derived, not written down twice.** The announcement reads the thresholds the layout uses. A literal
  restated in the presentation layer is a second source of truth and is what the boundary behavior above rejects.
- **No engine interaction.** Layout and its announcement depend on viewport dimensions alone, which rule 5's opening
  sentence fixes.

## Acceptance examples

### Example: normal behavior

**Given** the observer at `120 × 48`, where rule 5 excludes the inspector because `W < 140`

**When** a frame is drawn

**Then** the header states that the inspector needs `W ≥ 140` and that `i` opens it as an overlay, with emphasis
distinguishing that statement from the speed, zoom and filter segments beside it

### Example: failure behavior

**Given** the observer at `120 × 48` at `ff3a155`

**When** a frame is drawn

**Then** the header reads `overlays: inspector i` in unstyled text, naming neither the axis, nor the value, nor that
the terminal is too small — which is the state measured on 2026-08-20 and the state this requirement rejects

## Open decisions

None as a product decision. The product owner settled the content on 2026-08-20, recorded as decision 15 in
`evidence/WO-MOK-012/closing-review.md`: state both remedies rather than replacing one with the other, with the
example form `inspector needs W>=140 — overlay: i`.

The wording, the abbreviation ladder and the choice of emphasis are `SPEC-MOK-003` rule 5's to fix and the technical
owner's to approve. `WO-MOK-013` states the amendment.
