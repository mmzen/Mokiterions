+++
id = "REQ-MOK-031"
type = "requirement"
title = "Derive a per-Mokiterion behavioral trait from the seed and the identifier"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "WHEN a simulation is initialized, THE SYSTEM SHALL give each Mokiterion one behavioral trait value fixed for the run and determined solely by the run seed and that Mokiterion's identifier, without consuming any value from the shared entropy stream."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-006"]
+++

# Requirement: Derive a per-Mokiterion behavioral trait from the seed and the identifier

## Rationale

Individuality has to come from somewhere, and there are only two candidates: the entropy stream or the seed and the
identifier. Drawing from the stream would make each Mokiterion different *and* make every existing run different,
because the shared stream orders resource placement, decision fallbacks and regeneration, and inserting twelve draws
at initialization shifts every value that follows. `REQ-MOK-014`'s survivor floor is a measured number on that
sequence; moving the sequence retires the measurement.

Deriving the trait instead makes it a *function* of the run's identity. The same seed produces the same twelve
Mokiterions, so stating the seed states the population, and `REQ-MOK-009`'s reproducibility guarantee extends to
individuality without being restated. The stream is untouched, so every existing outcome stands.

The constraint that carries the weight is the last clause. A derivation that quietly borrowed one value from the
shared stream would satisfy every other sentence in this requirement and silently invalidate the verified floor, so
consuming nothing is an obligation of this requirement rather than an implementation note.

## Preconditions and trigger

The trigger is initialization of a simulation, before the first tick, under any decision source and any density.

The trait is assigned to every Mokiterion the run creates, regardless of which decision source is selected. The
trait is a property of the Mokiterion, not of the source: making the state model depend on the selected source would
mean two runs at one seed hold different world state, which no other attribute does. A source that does not read the
trait simply does not read it, exactly as the baseline source does not read energy.

## Required response

The system assigns each Mokiterion exactly one trait value, of a bounded integer range the specification fixes, and:

- The value is a function of the run seed and the Mokiterion's identifier alone. No other input contributes: not the
  density, not the selected decision source, not the tick limit, not the Mokiterion's initial coordinate or
  territory, and not the order in which Mokiterions are created beyond what the identifier already expresses.
- The value never changes after initialization. Nothing in any tick, and no proposed or applied action, alters it.
- Distinct identifiers at one seed are able to receive distinct values; the derivation does not collapse the roster
  to a single value.
- The derivation performs no draw against the shared entropy stream. After initialization, the shared stream stands
  at exactly the position it stood at in a build without traits, having produced exactly the same values in the same
  order.
- The value is reported once per Mokiterion, at initialization, in the text observation record `REQ-MOK-010`
  requires, so an operator reading a run's output can see the population it was given.

## Failure and boundary behavior

- There is no failure path. The derivation is total: every seed the command line accepts and every identifier the run
  creates yields a value in range. It cannot error, cannot be absent and cannot be out of range, so there is no
  configuration error, no runtime error and no exit-code consequence.
- At the range's endpoints the value is a legitimate outcome, not a defect. A Mokiterion at the lower bound is a
  valid individual and not an uninitialized one, and the reported value does not distinguish "derived as zero" from
  "not derived", because every Mokiterion is derived.
- Two identifiers may receive equal values at some seeds. Coincidence is not a violation; only a derivation that
  produces one value for all identifiers at a seed is.
- The trait is not a dynamic attribute. It is absent from the roster's attribute bars and from the per-tick survival
  record, because it does not vary per tick and reporting it there would imply that it can.

## Constraints

- Integer arithmetic only. No floating-point value participates in the derivation or in the mapping to the trait's
  range, for the reason `SPEC-MOK-001` gives for the density conversion: the result must be identical on every
  target.
- The derivation is a pure function. Given the same seed and identifier it returns the same value with no dependence
  on wall-clock time, environment, address values, iteration order of any unordered collection, or process state.
- The engine's dependency table stays empty. The derivation uses only the standard library and the pseudo-random
  generator the engine already owns.
- The trait adds no public interface item. It is reported through the text record, not through the observation
  snapshot, because no approved requirement needs the observer to render it and `SPEC-MOK-002` rule 5 holds the
  public interface to what approved requirements need.
- The trait's range, its name in the record, and the exact derivation are specification matters and are not fixed
  here. This requirement fixes only that the value exists, is bounded, is reproducible, is per-Mokiterion, and costs
  the shared stream nothing.

## Acceptance examples

### Example: normal behavior

**Given** the seed `42`

**When** the simulation is initialized twice, in separate processes

**Then** both runs assign the same trait value to each of the twelve identifiers, and the initialization record of
each run reports the same twelve values in the same order.

### Example: the trait varies across the roster

**Given** any accepted seed

**When** the simulation is initialized

**Then** the twelve reported trait values are not all equal.

### Example: the shared stream is untouched

**Given** the seeds `0`, `1`, `42`, `123` and `777`, and the default density

**When** a run of one thousand ticks completes under `--policy reference` and under `--policy baseline`

**Then** the survivor count, the death count, every surviving Mokiterion's final coordinate, and the standing
resource count and class distribution of each territory are identical to those a build without traits produces.

### Example: failure behavior

**Given** any accepted seed

**When** the simulation is initialized

**Then** no Mokiterion holds a trait value outside the specified range, and no code path reports a trait as absent,
unset or unavailable, because there is no such state to report.

## Open decisions

None. The trait's range and derivation are the technical owner's to fix in `SPEC-MOK-001`; the product decision — that
individuality is derived from the seed rather than drawn from the stream — is settled by this requirement and by
`INT-MOK-006`'s rejection of per-agent entropy substreams.
