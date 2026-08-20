+++
id = "REQ-MOK-043"
type = "requirement"
title = "Report per-tick world metrics as integers"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a tick completes and a structured record sink is configured, THE SYSTEM SHALL emit one metrics record stating the living population, the death count, the population and standing resource count by class of each territory, each territory's capacity and permanent-depletion state, and the sum and extremum of each dynamic attribute across the living population, as integers."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Report per-tick world metrics as integers

## Rationale

`REQ-MOK-042`'s event records say what *happened*. They do not say what *is*. A consumer wanting the standing
resource count of territory B at tick 40 must replay every initialization, consumption and regeneration record from
tick 0 and reimplement the resource rules to get there — which makes the consumer a second implementation of the
engine, and makes any disagreement between them a question of which one is wrong.

The engine already computes this state. Stating it directly is redundant with the event stream, and the redundancy
is the point: it is a **checkable** redundancy. A consumer can reconcile the metrics against its own replay of the
events, and a divergence is a defect the records themselves reveal. Emitting only events would be smaller and would
make every downstream question harder and every downstream answer less trustworthy.

Three facts have no event counterpart at all and are unreachable by any replay: a territory's **capacity**, whether
it is **permanently depleted**, and the standing distribution across resource classes. The engine holds all three
and the text stream states none of them; today they are visible only inside the observer's process. They are the
strongest argument for this record.

Per tick rather than per run is because a run's *shape* is the interesting object. Whether a population declines
steadily or collapses at one tick is not recoverable from an end-of-run figure, and the density curve this
repository already measured is exactly that kind of question.

**Integers only** is a constraint, not a formatting preference. An average of twelve `u8` attributes is a rounding
decision; specifying it would mean every consumer inherits it, and a float in the stream would put a formatting
implementation inside a byte-identical determinism contract. A sum and a count let each consumer round its own way,
losslessly, and an extremum answers the question an average hides — whether any individual is near death — which a
mean over twelve values cannot.

## Preconditions and trigger

The trigger is the completion of a tick, with a structured record sink configured. One record per completed tick, in
tick order, under any decision source, any density and any tick limit.

A tick counts as completed when the engine has finished applying its effects and the state the record describes is
the state at the tick's end. A tick that terminates the run is a completed tick and carries its record; the record
`REQ-MOK-044` requires is additional to it, not a substitute for it.

Absent a sink this requirement obliges nothing, and `REQ-MOK-045` requires the run to be indistinguishable from one
in a build without this capability.

## Required response

For each completed tick the system writes to the sink one metrics record stating, for the state at the end of that
tick:

- the tick number,
- the living population count,
- the cumulative death count,
- for each of the two territories: the number of living Mokiterions standing in it, the number of standing resources
  in it by resource class, its capacity, and whether it is permanently depleted, and
- for each of the four dynamic attributes — health, satiety, energy and fear — the sum of that attribute across the
  living population, and the extremum that answers the survival question for that attribute: the minimum for those
  whose depletion threatens survival, the maximum for those whose accumulation does.

And in addition:

- Every value is an integer. No mean, average, ratio, percentage or floating-point value appears.
- The living population count is the divisor for every sum in the same record, so a consumer computing an average
  needs no other record.
- The state described is the state at one instant — the tick's end — and not a mixture of values read at different
  points in the tick.
- Records appear in ascending tick order, one per completed tick, with no tick skipped and none repeated.
- Two runs with the same seed, configuration and sink configuration produce byte-identical metrics records.

## Failure and boundary behavior

- When the living population is zero, every sum is zero and every extremum is absent rather than zero. A minimum
  over an empty set is not `0`; reporting it as `0` would be indistinguishable from a living Mokiterion at zero
  health, which is a different world. The record states the absence explicitly.
- A permanently depleted territory reports a standing count of zero and its depletion state as true. The two are
  distinct facts: a territory may hold zero standing resources and still be replenishable, and the consumer must be
  able to tell those apart.
- Capacity is reported every tick even though it does not vary within a run. It is a fact about the world the record
  describes, and a consumer reading any single record should not need another to interpret it.
- A dead Mokiterion contributes to the death count and to no sum, no extremum and no territory population. Death is
  final, and the record does not carry a dead Mokiterion's last attribute values; those are in its event records.
- The record states no rate, no delta and no trend. A change between ticks is the consumer's subtraction, not the
  engine's claim.
- A sink write failure is `REQ-MOK-046`'s subject.
- No record carries a wall-clock time, a hostname, a filesystem path, an environment value or a credential.

## Constraints

- No metric for a phenomenon the engine does not compute. Conflict frequency, threat responses, retreats and
  surrenders are absent, and are not emitted as zero: a field fixed at zero reads as a measurement, and
  `SPEC-MOK-003` rule 4.5 set this repository's precedent by refusing to render a gauge for an attribute nothing
  yet consumed. They arrive with Phase 3 under a schema version increment.
- No outcome classification, label or verdict. `REQ-MOK-044` states that refusal for the terminal record and it
  holds here for the same reason.
- Additive, per `REQ-MOK-045`. Computing these figures reads authoritative state and draws no entropy; a metric
  whose computation moved the entropy stream would be refused outright.
- No new external dependency; the engine package's dependency table stays empty.
- The engine's library target resolves and opens no filesystem path.
- `SPEC-MOK-002` rule 6 is not relaxed. Computing a metric reads state; it does not expose a borrow of it.
- The record's field names, the resource-class vocabulary, the attribute names, which extremum each attribute
  carries, how absence is encoded, and the concrete syntax are `SPEC-MOK-006`'s to fix. This requirement fixes only
  which facts are stated, that they are integers, and that they describe one instant.

## Acceptance examples

### Example: normal behavior

**Given** a run at seed `0`, default density, `--policy reference`, two hundred ticks, with a sink configured

**When** the run completes

**Then** the sink holds exactly two hundred metrics records, one per tick in ascending order, each stating a living
count, a death count, both territories' population, standing resources by class, capacity and depletion state, and
the sum and extremum of all four dynamic attributes — every value an integer.

### Example: the metrics agree with the events

**Given** the same sink

**When** a consumer replays the event records to reconstruct the standing resource count of each territory at each
tick

**Then** the reconstruction equals the metrics record's figure at every tick, and the living and death counts agree
with the initialization and death events likewise.

### Example: an empty population

**Given** a seed and density at which the whole population dies before the tick limit

**When** the first tick after the last death completes

**Then** that tick's record states a living count of zero, every attribute sum as zero, and every extremum as
explicitly absent rather than as zero.

### Example: failure behavior

**Given** a sink that fails while a metrics record is written

**When** the engine attempts that write

**Then** the failure is surfaced and the run does not claim successful completion, per `REQ-MOK-046`.

## Open decisions

None. The field names, encodings and the choice of extremum per attribute are the technical owner's to fix in
`SPEC-MOK-006`. The product decisions — per-tick rather than per-run, integers rather than averages, deliberate
redundancy with the event stream, and no conflict metrics before Phase 3 — are settled here and by `INT-MOK-009`
principles 4 and 5.
