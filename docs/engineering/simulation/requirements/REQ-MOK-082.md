+++
id = "REQ-MOK-082"
type = "requirement"
title = "State a distribution over the sweep's axes from retained rows alone"
status = "draft"
owners = ["product owner"]
created = "2026-08-30"
updated = "2026-08-30"
statement = "WHEN retained fact rows for a completed sweep are read, THE SYSTEM SHALL state a distribution of outcome classes and behavioural figures over any requested subset of the sweep's axes, computed from the rows alone, and SHALL state the sweep that produced them."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-012"]
+++

# Requirement: State a distribution over the sweep's axes from retained rows alone

## Rationale

`INT-MOK-001`'s claim is that twelve Mokiterions "produce observable outcomes that are not predetermined by the
simulation engine". A distribution is the only shape of evidence that can support or refute it, and Phase 6's success
definition is "A defensible comparison. No particular behavior is required to occur; the requirement is that the
conditions permit emergence and that observed outcomes are not engine-determined."

That sentence has two halves and they need different things. *Not engine-determined* needs variation across seeds at
fixed conditions. *The conditions permit emergence* needs variation across conditions. So the distribution must be
statable over either axis independently, and over their combination, from the same rows — which is why aggregation is a
read-time operation over retained rows rather than something the batch bakes in.

**From rows alone** is the load-bearing phrase. If stating a distribution required re-running the sweep, then every
question asked of the data would cost the sweep again, and a reader without the ability to run the engine could ask
nothing. The 2026-08-30 measurement showed reading 20 rows takes 0.005 s against 0.94 s to produce them; at 400 runs
that ratio is what makes the evidence usable rather than archival.

**Stating the sweep is part of stating the distribution.** A table of outcome counts with no record of which cells
produced it cannot be compared to another table. The most common way to publish a misleading distribution is to omit
which configurations were and were not swept, and the 2026-08-30 finding that no famine occurred anywhere is exactly
the kind of result that means one thing if famine-producing conditions were swept and nothing at all if they were not.

## Preconditions and trigger

The trigger is a request to state a distribution over retained rows, naming the axes to aggregate over or accepting a
declared default.

The preconditions are that the rows exist and that the sweep they came from was complete; an incomplete sweep is
`REQ-MOK-083`'s subject and yields no distribution.

## Required response

For a requested aggregation, the system:

- reads the retained rows and reads nothing else — no stream, no engine execution, no network;
- groups the rows by the requested subset of the sweep's axes, and reports for each group the count of rows and the
  count of each outcome class within it;
- reports, for each group, the behavioural figures `REQ-MOK-080` states, keeping attempted and effective distinct
  through the aggregation;
- reports for each group at least one dispersion figure over the run-level figures, so that a group of twenty identical
  outcomes is distinguishable from a group of twenty varied ones;
- states the sweep the rows came from — every axis and every value on it, including values that produced no rows and
  why — and states the engine version, refusing to aggregate rows from more than one version into a single group
  without saying so;
- produces byte-identical output for the same rows and the same requested aggregation.

And in addition:

- The class counts within a group sum to the group's row count. Exactly one class per run makes that an equality, and
  `REQ-MOK-081`'s ordering is what guarantees it.
- No figure in the output is a floating-point value. A dispersion figure is stated as integers — an extremum, a range,
  or an order statistic — on `SPEC-MOK-006` rule 12.4's reasoning, and because a mean over twenty runs invites more
  confidence than twenty runs support.
- The output states no conclusion, no verdict, no pass, no expectation and no target distribution. It states counts. A
  required distribution would contradict `INT-MOK-001`'s claim that outcomes are not predetermined.
- A class that occurred zero times appears with a count of zero, and is not omitted.
- The output is regenerable from the rows at any later time, and nothing about it depends on when it was produced.

## Failure and boundary behavior

- An aggregation requested over rows from more than one engine version is refused, or performed with the versions
  reported as separate groups; it is never silently merged, because two engines are two experiments.
- An aggregation over zero rows is reported as zero rows, not as an empty distribution and not as an error.
- A group containing exactly one row is reported, with its dispersion figures stating that the group has one member.
  Suppressing small groups would hide precisely the cells a sweep failed to populate.
- A row that `REQ-MOK-081` could not classify is counted in its group's row total and reported in an unclassified
  column. It is not excluded, which would make the class counts sum correctly while hiding a defect.
- The output never claims a sweep was complete. Completeness is `REQ-MOK-083`'s statement and this requirement's output
  carries it rather than asserting it independently.

## Constraints

- **No engine execution and no stream read.** The distribution is a function of the retained rows and the requested
  aggregation.
- **No change to the engine, no amendment to `SPEC-MOK-006`, no new package and no new external dependency.**
- Integers and closed-vocabulary values only in any machine-readable output. A human-readable rendering may format as
  it likes and is regenerable rather than retained as the authority.
- The aggregation's command-line form, the default aggregation, the dispersion figures chosen, the output's exact shape
  and the human-readable rendering's format are `SPEC-MOK-008`'s to fix. This requirement fixes only that a
  distribution is statable from rows alone, over any subset of the axes, with the sweep stated and with no conclusion
  attached.

## Acceptance examples

### Example: not engine-determined

**Given** retained rows for four sources at twenty seeds each, at fixed density

**When** the distribution is stated over the source axis

**Then** each source's group reports its class counts and a dispersion figure over survivors, and the seed-to-seed
variation within a group is visible.

### Example: the conditions permit emergence

**Given** retained rows for one source at five densities

**When** the distribution is stated over the density axis

**Then** the extinction count at the lowest density and its absence at the highest are both visible in the same table.

### Example: the sweep is stated

**Given** any distribution output

**When** it is read

**Then** it names every axis and every value swept, including any value that produced no rows, and names the engine
version.

### Example: an unobserved class

**Given** rows in which no run met the famine predicate

**When** the distribution is stated

**Then** famine appears with a count of 0 in every group.

### Example: rows alone

**Given** retained rows and no engine binary present

**When** a distribution is requested

**Then** it is produced.

## Open decisions

None at this level. Which dispersion figures are stated, and what the default aggregation is, are the technical owner's
in `SPEC-MOK-008`. The product decisions — that the distribution is computed from rows alone, that it is statable over
any subset of the axes, that the sweep is stated with it, and that it carries no conclusion — are settled here and by
`INT-MOK-012` principles 1 and 2.
