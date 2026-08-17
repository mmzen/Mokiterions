+++
id = "REQ-MOK-021"
type = "requirement"
title = "Degrade the layout across viewport sizes without losing authoritative information"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN the terminal viewport is smaller than the reference size, THE SYSTEM SHALL apply the declared degradation order, indicate which panes are hidden and whether the spatial view is showing a region rather than the whole world, and refuse to start with a stated required size when the viewport is below the declared floor."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-003"]
+++

# Requirement: Degrade the layout across viewport sizes without losing authoritative information

## Rationale

A terminal interface has no control over its own dimensions. The operator's terminal is whatever it is, and it can
be resized while the run is in progress. An interface that assumes one size produces one of two failures: it draws
outside the viewport and corrupts the display, or it silently compresses panes until the values in them are wrong.
The second failure is the dangerous one, because a truncated satiety value or a spatial view showing a quarter of
the world without saying so leads an operator to a confident wrong conclusion.

Refusing to start below a floor is preferable to drawing something degraded past usefulness. The spatial view has
a fidelity below which distinct world positions become indistinguishable, and a view in which two Mokiterions
sixteen cells apart appear to share a position is not a weaker instrument but a misleading one.

Announcing what is hidden is the requirement's core. Degradation itself is unavoidable; degradation the operator
cannot detect is a defect.

## Preconditions and trigger

The observer is starting, or is drawing a frame, and the viewport dimensions are known. A resize during the run is
a trigger, not an error.

## Required response

**At or above the reference size**, every pane specified for the complete layout is present, and the spatial view
presents the whole world at full fidelity.

**Below the reference size**, panes are hidden, overlaid, or restacked in the order `SPEC-MOK-002` declares, and
the layout adapts in the same way for the same dimensions every time. Degradation is deterministic in viewport
size and in nothing else.

**Whenever anything is hidden or reduced**, the observer indicates it: which panes are unavailable at the current
size, how they can be reached if they can be reached, how many roster entries are not visible, and whether the
spatial view is presenting a region of the world rather than all of it.

**Below the declared floor**, the observer does not draw. It reports the current dimensions and the dimensions it
requires, and exits with the specified status.

**On resize**, the layout is recomputed for the new dimensions and the run continues. A resize does not restart the
run, lose the selection, discard retained events, or alter the simulation.

## Failure and boundary behavior

- No pane is drawn outside the viewport, and no content is clipped in a way that could be read as a different
  value. A number that does not fit is not shown truncated.
- The spatial view's fidelity does not fall below the declared minimum. When the space available would require
  less, the view presents a region and says so.
- A resize to below the floor mid-run does not terminate the run: the observer reports the required size and
  resumes drawing when the viewport is large enough again.
- A viewport reported as zero in either dimension, or a viewport whose size cannot be determined, is handled as
  below the floor rather than causing an arithmetic failure.
- Degradation never hides the run's provenance, since a screen capture that does not identify its own run cannot
  serve as evidence.

## Constraints

- The reference size, the floor, the complete pane set, the degradation order, the thresholds at which each step
  applies, the spatial view's minimum fidelity, and the exit status used on refusal are fixed by `SPEC-MOK-002`.
- Layout selection depends only on viewport dimensions. It does not depend on run state, tick number, entropy, or
  wall-clock time, so that the same dimensions always yield the same layout.
- Layout computation consumes no simulation entropy and does not mutate simulation state.
- Degradation changes what is presented and never what is retained. Hidden events remain exportable, and hidden
  roster entries remain part of the population.

## Acceptance examples

### Example: normal behavior

**Given** a viewport at the reference size, resized during the run to a width below the threshold at which the
inspector pane is dropped

**When** the next frame is drawn

**Then** the inspector is no longer occupying width, the observer indicates that it is available as an overlay, the
spatial view and roster remain correct for the new width, the selection is retained, and the run continues
uninterrupted.

### Example: failure behavior

**Given** a viewport below the declared floor at start-up

**When** the observer starts

**Then** it draws no interface, reports the current and required dimensions, and exits with the specified status
rather than rendering a corrupt view.

## Open decisions

None. The reference size, floor, pane set, degradation order and thresholds, minimum spatial fidelity, and refusal
exit status are fixed by `SPEC-MOK-002`.
