+++
id = "REQ-MOK-025"
type = "requirement"
title = "Preserve simulation outcome under observation"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN a simulation is executed under observation with a given seed, configuration and decision source, THE SYSTEM SHALL produce the same authoritative event sequence and the same final state as the same run executed without observation."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: Preserve simulation outcome under observation

## Rationale

This is the requirement that permits the observer to exist at all.

`REQ-MOK-009` makes identical seed and configuration produce identical runs, and every measurement the project
holds rests on it: the eight-to-eleven survivor range, the tick-119 starvation arithmetic, the tick-9,154
extinction, the four replay hashes retained under `VREC-MOK-002`. If observation could alter an outcome, those
figures would describe only unobserved runs, an observer's report of a run would not be a report of a run anyone
else could reproduce, and no observation could ever be admitted as evidence.

The threats are concrete rather than theoretical. An observer draws frames on a schedule, reads input at arbitrary
moments, and is resized by the operating system — three sources of wall-clock and environment dependence sitting
directly beside a deterministic engine. Any of them leaking into the engine's entropy stream or turn ordering would
break determinism in a way that reproduces intermittently, which is the hardest kind of defect to find and the
easiest to mistake for a simulation result.

The existing precedent is exact and this requirement extends it: `--trace-actions` must not change decisions,
entropy, or state, and that property is verified. An observer is the same kind of thing at a larger scale.

## Preconditions and trigger

Two executions share the same seed, tick limit, resource density, decision source, and engine version. One runs
under observation; the other does not.

## Required response

The two executions produce:

- the same sequence of authoritative events, in the same order, with the same subjects and the same results;
- the same final state — the same living Mokiterions with the same health, satiety, energy, positions and
  territories, and the same standing resources with the same classes and positions;
- the same termination reason at the same tick.

The equality holds regardless of what the operator did while observing: how long the run was held, how many single
steps were taken, at what speed it ran, which Mokiterion was selected, how the view was panned or zoomed, which
filter was applied, whether an export was made, and how often the terminal was resized.

## Failure and boundary behavior

- Frame rate, draw timing, input timing, and resize events must not appear in any authoritative outcome.
- The observer must consume no value from the simulation entropy source. Entropy draw counts for a given tick must
  be identical observed and unobserved.
- The observer must not reorder ticks, reorder agent turns within a tick, skip a tick, or apply a tick twice.
- A failure inside the observer — a draw error, an input error, a failed export — must not leave a tick partially
  applied. Either the tick completed authoritatively or it did not run.
- A run terminated early by the operator produces a prefix of the unobserved run's events, identical up to the tick
  at which it stopped, and reports that it was ended early rather than claiming a completed run.

## Constraints

- The observer holds no mutable handle to world, agent, resource, event-log, or engine state, consistent with
  `ADR-MOK-001`.
- The observer offers no operator action that mutates simulation state. Choosing when the engine advances is the
  full extent of operator influence.
- Wall-clock time may be read only to schedule when the observer draws or when it advances a running engine, and
  never as an input to engine computation.
- Authoritative event content carries no wall-clock timestamp and no environment-specific value, as `REQ-MOK-009`
  already requires.

## Acceptance examples

### Example: normal behavior

**Given** seed 42 at the default density under the reference decision source, run for 200 ticks

**When** one execution is observed — held at tick 37, single-stepped nine times, with two different Mokiterions
selected, the view panned and zoomed, a filter applied and an export taken — and another is executed as the text
stream

**Then** both produce the same authoritative event sequence and the same final state, and the observer's exported
events match the text stream's authoritative events record for record.

### Example: failure behavior

**Given** an observed run in which the terminal is resized twenty times and the observer fails to draw once

**When** the run completes

**Then** the authoritative event sequence and final state are still identical to the unobserved run, and the draw
failure is reported as an observer failure rather than as a simulation result.

## Open decisions

None. The comparison method, the fields compared, and the permitted uses of wall-clock time are fixed by
`SPEC-MOK-003`.
