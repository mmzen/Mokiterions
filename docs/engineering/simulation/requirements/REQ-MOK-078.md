+++
id = "REQ-MOK-078"
type = "requirement"
title = "Execute a declared sweep and retain one fact row per run"
status = "approved"
owners = ["product owner"]
created = "2026-08-30"
updated = "2026-08-30"
statement = "WHEN an operator requests a batch over a declared sweep of decision sources, food densities and seeds at a stated tick horizon, THE SYSTEM SHALL execute the engine once per cell and retain exactly one fact row per completed run, carrying that run's sweep coordinates and the figures its run record states."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-012"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-30T16:11:24Z"
decided_by = "product owner"
reason = "Approved by the repository owner on 2026-08-30, by selecting the presented option, as part of the twelve-artifact Phase 4b chain. The chain converts docs/ROADMAP.md's Phase 4b open question into an approved shape on the strength of the measurement the roadmap reserved that decision to, taken at c90edc9 and recorded in ADR-MOK-008. It carries three disclosed and unrepaired findings: the threat mechanism is inert in 1447 of 1448 firings, the famine predicate is unreached in the swept space with food still standing at extinction, and no retreat event kind exists."
+++

# Requirement: Execute a declared sweep and retain one fact row per run

## Rationale

Every quantitative claim this repository has made was produced by a loop written for the occasion and then discarded.
The figures were sound and the method does not survive contact with Phase 6, where the result *is* the distribution
and the loop is therefore part of the evidence rather than scaffolding around it.

A **declared** sweep is the point. A sweep that exists only as an operator's shell history cannot be cited, cannot be
re-run by a reader, and cannot be compared against the sweep that produced last week's figure. Naming the axes and
naming a default makes a batch a thing that can be referred to.

The axes are decision source, food density and seed, and that set is established by measurement rather than by
preference. On 2026-08-30, five seeds at each of four sources at density 0.75 produced exactly two outcome classes and
three of the four sources produced the same one in every seed. The same source across five densities produced
extinction in 3 of 3 runs at 0.10, one or two survivors of twelve at 0.25, and seven to nine at 0.75. Seed varies the
figures; density varies the outcome. A sweep over seeds alone — which is what `docs/ROADMAP.md` proposed for this
phase — would have reported one distinguishable fact.

**One row per completed run**, and the row is small. A thousand-tick event stream is about 3 MB and the run record
inside it is 1,040 bytes. Four sources by five densities by twenty seeds is 400 runs: about 1.2 GB of streams and
about 500 KB of rows. The row is what survives.

## Preconditions and trigger

The trigger is an operator request for a batch, naming a sweep or accepting the declared default.

The preconditions are that the engine's binary is available to execute, and that a writable location exists for the
retained rows and for the temporary stream of the run in progress.

No precondition involves a network, a credential, or an environment variable other than those the operator's own
invocation needs. The `llm` decision source is not a permitted axis value; `REQ-MOK-072` governs it and no batch may
reach it.

## Required response

For the requested sweep, the system:

- enumerates the cells as the cross product of the requested decision sources, food densities and seeds, at the
  requested tick horizon, in a stated deterministic order;
- executes the engine once per cell, with that cell's coordinates and no other configuration difference, directing the
  engine's structured record stream to a location the batch controls;
- retains, for each run that completed, exactly one fact row carrying at minimum the cell's decision source, food
  density, seed and tick horizon, the engine version the run reported, and every figure the run record states —
  termination reason, tick count, survivors, deaths, crossings, consumption by food class, regeneration count, both
  skipped-regeneration counts, and each territory's final population and standing supply by class;
- copies those figures without reinterpretation, rounding, aggregation or renaming of their meaning, so that a row's
  figure and the run record's figure are the same fact;
- retains the rows such that two batches over the same sweep produce byte-identical retained output.

And in addition:

- The order in which cells execute does not affect any row's content. A row is a function of its cell.
- Exactly one row exists per completed run. No cell contributes two rows, and no row is synthesised for a cell that
  did not run.
- The retained rows carry no outcome class, label, verdict or threshold; `REQ-MOK-081` owns classification and
  `CAP-MOK-012` excludes it from retained data.
- The retained rows carry no floating-point value. Food density is a closed-vocabulary string as the engine's own
  option takes it, not a computed decimal.
- Driving a run from a batch changes no byte of that run's own output. A run executed by the batch and the same run
  executed by hand produce identical streams.

## Failure and boundary behavior

- A cell whose run fails to complete produces no row, and the batch's completeness is `REQ-MOK-083`'s subject. A
  failed cell is not filled with zeroes and is not silently skipped.
- A sweep naming `llm` as a decision source is refused before any run starts, and the refusal names the requirement
  that forbids it. It is not attempted and permitted to fail.
- A sweep with an empty axis produces no runs and is refused rather than reported as a batch of zero.
- A duplicate value on any axis is refused or collapsed, and which of the two happens is stated rather than left to
  observation, because a duplicated seed would double-weight a cell in every later distribution.
- The engine's own refusals stay the engine's. A density the engine rejects is reported as that cell's failure with
  the engine's own message, not translated.
- No batch writes into a location holding governance artifacts, and no batch removes a file it did not create.

## Constraints

- **No change to the engine.** The batch drives the existing binary through its existing options. No option is added,
  no record changes, and the schema version stays at 3.
- **No new Cargo package and no new external dependency in either package.** `ARCH-MOK-001` and `SPEC-MOK-004` rule 1
  are untouched; `ADR-MOK-008` records the decision and its alternatives.
- The engine refuses `--events-path -`, so the batch writes a file per run. It overwrites an existing path cleanly, so
  one reusable path is sufficient and peak disk during a batch is one stream.
- Integer and closed-vocabulary values only in retained rows.
- The concrete file format, field names, field order, default sweep values, cell ordering, temporary path convention,
  duplicate-value disposition and command-line form are `SPEC-MOK-008`'s to fix. This requirement fixes only that a
  declared sweep executes, that one row per completed run is retained, and that the row states the run record's facts
  unaltered.

## Acceptance examples

### Example: normal behavior

**Given** the declared default sweep of four decision sources, five densities and twenty seeds at a thousand ticks

**When** the batch completes

**Then** exactly 400 fact rows are retained, each naming its own source, density, seed and horizon, and each row's
survivor count equals the survivor count in the run record of the run that cell produced.

### Example: the axes are the ones that matter

**Given** a sweep of one source over densities `0.10` and `0.75` at three seeds each

**When** the batch completes

**Then** the three rows at `0.10` and the three at `0.75` differ in survivor count in a way the seed axis alone did
not produce, which is the measured basis for density being an axis.

### Example: reproducibility

**Given** two batches over the same sweep, on the same commit

**When** both complete

**Then** the retained output of the two is byte-identical.

### Example: driving does not disturb

**Given** one cell executed by the batch, and the same coordinates executed by hand

**When** both complete

**Then** the two event streams are byte-identical and the two standard-output streams are byte-identical.

### Example: the excluded source

**Given** a sweep naming `llm` among its decision sources

**When** the batch is requested

**Then** it is refused before any run starts, and the refusal names `REQ-MOK-072`.

## Open decisions

None.

Whether a batch may run in continuous integration, and whether any sweep's output is committed
rather than produced on demand, are `SPEC-MOK-008`'s and the work order's respectively. The product decisions — that
the sweep is declared rather than ad hoc, that the axes are source, density and seed, and that the retained unit is
one small row per run rather than a stream — are settled here and by `INT-MOK-012` principles 2 and 4.
