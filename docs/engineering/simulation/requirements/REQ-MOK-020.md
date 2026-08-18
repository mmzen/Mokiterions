+++
id = "REQ-MOK-020"
type = "requirement"
title = "Present survival state for every living Mokiterion"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN an observed simulation is running or paused, THE SYSTEM SHALL display for every living Mokiterion its identifier, territory, health, satiety, energy, and the action applied on the most recently completed tick, without requiring the operator to scroll at the reference viewport size."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: Present survival state for every living Mokiterion

## Rationale

The spatial view shows where Mokiterions are; it cannot show how close each one is to dying. Health, satiety and
energy are the three numbers that decide the outcome, and the mechanism that kills is a threshold rather than a
gradient: satiety or energy reaching zero costs five health per tick, and health reaching zero is final. An
operator watching for the moment a population turns needs to see all three numbers for all twelve agents
simultaneously, because the interesting event is one agent crossing a threshold while the others do not.

The no-scroll obligation exists because scrolling defeats the purpose. A roster the operator must page through
presents the same sequential access problem as the text stream, and the population is small and fixed at twelve,
so simultaneous presentation is achievable rather than aspirational.

The action applied on the last tick is included here, rather than only in the inspector, because the roster is
where divergence becomes visible: eleven agents searching while one sleeps is a pattern, and it is only a pattern
if all twelve actions are readable at once.

## Preconditions and trigger

An observed run has been initialized and the observer is drawing a frame.

## Required response

For each living Mokiterion, in a stable order that does not change between frames, the display presents:

- its identifier;
- the territory it currently occupies;
- health, satiety and energy, each as a value and as a proportional visual indicator so that relative magnitude
  is readable without reading digits;
- the action the engine applied to it on the most recently completed tick.

The display also presents the count of living Mokiterions, so that a death is visible as a change to a total and
not only as a row disappearing.

Dead Mokiterions are excluded from the roster. A Mokiterion that dies during observation is removed from the
roster on the tick its death is applied.

At the reference viewport size, every living Mokiterion's entry is visible at once with no scrolling and no
truncation of the values named above.

## Failure and boundary behavior

- When the viewport cannot present all living entries, the display indicates how many entries are hidden rather
  than silently omitting them.
- A value of zero is presented as zero and remains distinguishable from an absent or not-yet-computed value.
- When every Mokiterion is dead, the roster presents an explicit extinction state rather than an empty pane whose
  emptiness could be read as a rendering fault.

## Constraints

- Values are read from authoritative engine state. The display computes no derived survival estimate, no
  predicted time to death, and no quantity the engine does not produce.
- Attributes the engine does not compute — including fear, traits, per-agent names, age, kills, combats, and
  remembered locations — are absent from the roster. The layout may reserve their position, and it must not
  render an inert value that reads as a computed zero.
- Ordering, field selection, indicator form, reserved positions, and the reference viewport size are fixed by
  `SPEC-MOK-003`.
- Displaying state consumes no simulation entropy and does not mutate state.

## Acceptance examples

### Example: normal behavior

**Given** an observed run at the reference viewport size with twelve living Mokiterions

**When** a frame is drawn

**Then** twelve entries are visible simultaneously, each showing its identifier, territory, health, satiety,
energy and last applied action, and the living count reads twelve.

### Example: failure behavior

**Given** eleven Mokiterions have died and one remains with satiety zero

**When** a frame is drawn

**Then** one entry is visible, its satiety reads zero rather than appearing blank, and the living count reads one.

## Open decisions

None. Field selection, ordering, indicator form, treatment of not-yet-computed attributes, and the reference
viewport size are fixed by `SPEC-MOK-003`.
