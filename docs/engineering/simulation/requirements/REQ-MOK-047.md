+++
id = "REQ-MOK-047"
type = "requirement"
title = "Render a survival gauge at a width that resolves the value it presents"
status = "draft"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHERE the terminal observer presents a survival attribute as a proportional gauge at the reference viewport size, THE SYSTEM SHALL render that gauge at a width at which any change of ten in the value it presents changes the number of filled cells."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: Render a survival gauge at a width that resolves the value it presents

## Rationale

`REQ-MOK-020` obliges the observer to present survival state for every living Mokiterion. It says what must be
presented and it says the operator must not have to scroll to reach it. It says nothing about whether the presentation
carries the value, and at the reference viewport the gauge no longer does.

The measurement is in `evidence/WO-MOK-012/adverse-observations.md` and reduces to one line of arithmetic. The bar row
carries four gauges in `5 + 4 × 6 + 3 × 2 = 35` columns of overhead, so
`bar_width(interior) = min(20, (interior - 35) / 4)`, and the reference roster's 45-column interior yields **2**. A
two-cell bar filled by `value × width / 100` has three renderable states for the 101 values it presents: empty for
`0..=49`, half for `50..=99`, full at `100`. An operator watching a Mokiterion decline from 99 to 50 sees no change in
the bar at all.

That is not a defect against any approved artifact, which is why it needs a requirement. `SPEC-MOK-003` rule 4 states
the narrowing to two cells explicitly, and the technical owner accepted it on 2026-08-19 rather than widening the
roster pane. The bar was still doing something at six cells, and the fourth gauge cost it four. **What was accepted as
a narrowing turned out to be a loss of the quantity**, and the product owner reported it from a live 200-tick pass on
2026-08-20 as one of three adverse observations. This requirement is the obligation whose absence let a gauge become
decorative while every automated case still passed.

## Preconditions and trigger

The observer is drawing above the floor at the reference viewport size, and a pane presents a survival attribute as a
proportional gauge. Today that pane is the roster and the attributes are health, satiety, energy and `fear`; the
obligation is written on the gauge rather than on the pane, so a later pane presenting a gauge inherits it.

## Required response

Each gauge is rendered at a width `w` such that for every value `v` in `0..=90`, the filled-cell count at `v + 10`
differs from the filled-cell count at `v`.

Under the `filled = value × w / 100` fill rule of `SPEC-MOK-003` rule 4 this is satisfied exactly when `w ≥ 10`,
because `10 × w / 100 ≥ 1` is what makes a ten-point step cross a cell boundary. The requirement states the observable
property rather than the width, so a change to the fill rule cannot satisfy it by arithmetic alone.

## Failure and boundary behavior

- **Below the reference viewport size** the obligation does not apply. `SPEC-MOK-003` rule 5 degrades the layout by
  design and rule 4 already collapses an entry to numeric values without bars below 47 columns, where the number
  carries the level directly. A narrow terminal presenting a coarse gauge is not a violation; the reference viewport
  presenting one is.
- **A gauge the specification bands** keeps its band. This requirement adds resolution and removes nothing: rule 4
  clause 7's three survival bands and clause 5's unbanded `fear` are untouched, and a band remains a second
  presentation of a number rather than the number.
- **`0` and an absent value** stay distinguishable exactly as rule 4 clause 4 requires. A wider gauge does not change
  what an empty bar means, and `—` remains the absent form.
- **A value outside `0..=100`** is not reachable: `SPEC-MOK-001` bounds every survival attribute to that range, and the
  observer presents what the engine reported.

## Constraints

- **No derived survival estimate.** `REQ-MOK-020`'s constraint holds unchanged. This requirement changes how many
  states a gauge can show, not what quantity it shows, and it introduces no trend, no rate and no threshold.
- **No engine change.** The values are the ones `SPEC-MOK-002`'s read-only surface already reports. Nothing here
  reaches the engine, its state, its entropy or its event stream.
- **Layout stays a pure function of viewport dimensions.** `SPEC-MOK-003` rule 5's opening obligation is untouched;
  a gauge width derived from the pane interior is already such a function.
- **The reference viewport is `160 × 48`** as `SPEC-MOK-003` rule 5 fixes it, and this requirement does not move it.

## Acceptance examples

### Example: normal behavior

**Given** the observer at the reference viewport size with twelve living Mokiterions

**When** a Mokiterion's health falls from 99 to 89 over the ticks the engine applies

**Then** the number of filled cells in its health gauge is smaller at 89 than it was at 99

### Example: failure behavior

**Given** a gauge rendered two cells wide, as `SPEC-MOK-003` rule 4 yields at a 45-column roster interior

**When** health falls from 99 to 50

**Then** the filled-cell count is 1 at both values, and the requirement is violated — this is the state measured at
`ff3a155` and the state this requirement rejects

## Open decisions

**One, and it is blocking. It is a decision about `REQ-MOK-020`, not about this requirement.**

At the reference viewport the roster interior is 45 × 32 columns and rows. Satisfying this requirement means moving
from four gauges on one line to two, which raises the entry from two lines to three, and 12 entries × 3 lines = 36
rows do not fit in 32. `REQ-MOK-020` obliges all twelve to be visible "without requiring the operator to scroll at the
reference viewport size", and `VER-MOK-005`'s matrix makes that an automated pass condition: "At `160 × 48` all twelve
two-line entries are present in the roster pane; none is hidden."

**The geometry admits no arrangement satisfying both this requirement and `REQ-MOK-020` as approved, at a 47-column
roster and a 10-row log.** Four gauges of width 13 on one line need an 87-column interior, which is the roster
widening the technical owner declined on 2026-08-19.

`WO-MOK-013` states the two ways out, with what each amends and what each costs, and carries the decision. Neither is
taken here, and this requirement must not be approved before one is: approving it alone would leave two approved
requirements that cannot both be met.
