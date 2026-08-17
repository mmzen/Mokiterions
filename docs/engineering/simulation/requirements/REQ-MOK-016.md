+++
id = "REQ-MOK-016"
type = "requirement"
title = "Render the whole world in one view"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN an observed simulation is running or paused, THE SYSTEM SHALL render every living Mokiterion and every standing resource at its authoritative position in a single spatial view, with the territory boundary drawn and each territory's standing resource count displayed."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-003"]
+++

# Requirement: Render the whole world in one view

## Rationale

Position is the dominant explanatory variable in this world. `WO-MOK-002` measured a population reaching
extinction with 122 resources standing and both territories at capacity, which establishes that distance to food,
not the quantity of food, decides outcomes. A view that shows all positions at once is therefore the primary
instrument, and a view that shows a fraction of the world is a different and weaker instrument.

Standing resource count per territory is displayed rather than left to be counted because it is the leading
indicator of the one irreversible event in the simulation: a territory that reaches zero resources never
regenerates again. That transition is invisible until it has already happened unless the count is on screen.

## Preconditions and trigger

An observed run has been initialized, and the observer is drawing a frame. The simulation may be advancing or
held between ticks; both states render.

## Required response

The spatial view presents, for the tick most recently completed:

- every living Mokiterion, at its authoritative coordinates, distinguishable from resources and attributed to
  its current territory;
- every standing resource, at its authoritative coordinates, with its class distinguishable at the fidelity
  `SPEC-MOK-002` declares for per-resource class, and with each territory's per-class counts available at every
  fidelity;
- the boundary between territory A and territory B, drawn at its specified location;
- the number of standing resources currently in territory A and in territory B.

Dead Mokiterions are not rendered, consistent with their absence from perception and from the population.

The view is oriented so that territory A appears above territory B, matching the coordinate convention used
throughout the project's documentation.

Where the viewport cannot present the whole world at the specified fidelity, the view presents a contiguous
region of it and indicates that the world extends beyond what is shown, so that absence of a Mokiterion from
the view is never mistaken for its death.

## Failure and boundary behavior

- When two or more entities occupy the same rendered position, the view must not silently show one of them as if
  it were alone; the specification declares a deterministic precedence and an indication that the position is
  shared.
- When a territory's standing resource count reaches zero, the view indicates the permanence of that state
  rather than displaying it as an ordinary count of zero.
- At a fidelity where a per-resource class cannot be encoded, the view presents resource presence and the per-class
  counts remain available. A character cell is indivisible for text and carries one foreground colour, so at the
  fidelity that presents the whole world one mark can carry position or class but not both; presenting position and
  reporting class as counts is required, and inventing a per-resource class distinction that the fidelity cannot
  carry is a defect.
- Rendering must not fail or panic on an empty world, a world with no living Mokiterions, or a world in which
  every resource has been consumed.
- Rendering reads state and never mutates it. A rendering failure must not corrupt or advance the simulation.

## Constraints

- The view reflects authoritative engine state only. It displays no quantity the engine does not compute.
- The rendered fidelity, glyph and colour assignment, coordinate mapping, orientation, shared-position
  precedence, and zoom behavior are fixed by `SPEC-MOK-002`.
- Rendering consumes no simulation entropy.
- The view must remain legible without colour, since colour is an enhancement and not the carrier of identity.

## Acceptance examples

### Example: normal behavior

**Given** an observed run at the default density with twelve living Mokiterions and 61 standing resources in each
territory

**When** a frame is drawn at a viewport that can present the whole world

**Then** twelve Mokiterion marks and 122 resource marks appear at their authoritative coordinates, the territory
boundary is drawn between them, and the two territory counts read 61 and 61.

### Example: failure behavior

**Given** territory B has reached zero standing resources

**When** a frame is drawn

**Then** territory B's count is presented as permanently depleted rather than as an ordinary zero, and the view
continues to render territory A normally.

## Open decisions

None. Fidelity, mapping, orientation, glyphs, colours, precedence on shared positions, and the conditions under
which the view presents a region rather than the whole world are fixed by `SPEC-MOK-002`.
