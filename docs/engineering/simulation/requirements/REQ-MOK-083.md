+++
id = "REQ-MOK-083"
type = "requirement"
title = "Refuse to present an incomplete sweep as a distribution"
status = "approved"
owners = ["product owner"]
created = "2026-08-30"
updated = "2026-08-30"
statement = "WHEN any cell of a requested sweep does not produce a retained fact row, THE SYSTEM SHALL report the sweep as incomplete, naming each missing cell and the reason, and SHALL refuse to state a distribution over the rows it did obtain unless the incompleteness is stated with them."
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

# Requirement: Refuse to present an incomplete sweep as a distribution

## Rationale

`REQ-MOK-046` establishes the principle inside one run: a sink failure is surfaced and the engine "does not claim
successful completion", because a partial stream presented as a complete run is worse than no stream. This requirement
is that principle at batch scale, and at batch scale it is more dangerous, because a distribution missing some of its
cells still looks like a distribution.

The specific failure mode is quiet and plausible. A sweep of 400 cells in which 12 failed still produces 388 rows, still
aggregates, still prints a table with every column populated, and every figure in it is wrong in a direction nobody can
see — because the cells that fail are not a random sample. A density the engine rejects fails at one end of the axis; a
run that exhausts something fails where the simulation is most stressed. Losing those cells removes exactly the
observations a distribution exists to report.

**Naming each missing cell and its reason** is what makes the report actionable rather than a warning. "12 cells failed"
tells a reader that the figure is unsound. "These 12 cells, all at density 0.10, failed with this message" tells them
what the figure is missing, and lets them decide whether the remaining 388 answer their question.

The refusal is deliberately not absolute. Refusing outright would make a batch with one transient failure unusable and
would push an operator toward re-running until the batch happened to be clean, which is worse than reporting honestly.
So the rows may be aggregated, and the incompleteness travels with them and cannot be separated from them.

## Preconditions and trigger

The trigger is any of: a cell whose run did not complete, a cell whose stream could not be read, a cell whose derived
counts disagreed with its run record under `REQ-MOK-080`, a cell refused before execution, or a requested cell for which
no row exists for any other reason.

The precondition is that the sweep was requested, so its cells are enumerable. A sweep that could not be enumerated is
refused before any run and is not an incomplete sweep — it is not a sweep.

## Required response

When any requested cell yields no retained row, the system:

- reports the sweep as incomplete, stating the number of cells requested and the number of rows retained;
- names each missing cell by its full coordinates, and states for each the reason it produced no row, in the terms of
  the failure that occurred rather than a generic one;
- distinguishes a cell refused before execution from a cell whose run failed, and a run that failed from a run that
  completed but whose row could not be derived;
- exits with a status that distinguishes an incomplete sweep from a complete one, so that an automated caller cannot
  consume an incomplete sweep by ignoring output;
- carries the incompleteness into any distribution stated over the sweep's rows, such that the distribution cannot be
  read without it.

And in addition:

- The rows that were obtained are retained and are valid. A cell's failure does not invalidate the rows of cells that
  succeeded, and it does not cause them to be discarded.
- No missing cell is filled with a zero, a default, an interpolation, a repeated neighbour or a row from another
  sweep.
- A sweep in which **every** cell failed is reported as such and states no distribution, since there is nothing to
  state one over.
- The report of incompleteness is regenerable from what the batch retained. It is not only a message printed once at
  the time.
- A complete sweep says so explicitly. A reader must not have to infer completeness from the absence of a warning.

## Failure and boundary behavior

- A cell that fails on one attempt and succeeds on a retry produces a row and is not reported as missing. Whether the
  batch retries at all, and how many times, is `SPEC-MOK-008`'s to fix; a silent retry that hid a nondeterministic
  failure would be a defect, so any retry is stated.
- A cell refused because it named the `llm` source is a refusal of the sweep under `REQ-MOK-078`, before any run
  starts, and not an incomplete sweep.
- A batch interrupted by the operator is an incomplete sweep and is reported as one, with the cells not reached named as
  not attempted rather than as failed.
- Where the incompleteness cannot itself be recorded — the retained output is unwritable — the batch fails loudly and
  retains nothing rather than retaining rows whose incompleteness is unrecorded.
- A distribution stated over rows whose sweep record is absent is refused. Rows without a sweep record cannot be known
  to be complete, and unknown completeness is treated as incomplete.

## Constraints

- **No change to the engine.** A cell's failure is reported in the engine's own terms; the batch does not translate,
  summarise or improve an engine message.
- **No new external dependency.**
- Integers and closed-vocabulary values in the machine-readable record of incompleteness; the reason may carry the
  engine's own message, which is the one place a batch's output carries text it did not author.
- The exit-status values, the record of incompleteness, its field names, the retry policy and the exact form in which
  incompleteness attaches to a distribution are `SPEC-MOK-008`'s to fix. This requirement fixes only that
  incompleteness is detected, named per cell with a reason, distinguishable by exit status, and inseparable from any
  distribution over the affected rows.

## Acceptance examples

### Example: normal behavior

**Given** a sweep of 60 cells in which 3 runs fail

**When** the batch completes

**Then** it reports 60 requested and 57 retained, names the 3 cells with their coordinates and reasons, and exits with
the status reserved for an incomplete sweep.

### Example: the distribution carries it

**Given** the rows of that incomplete sweep

**When** a distribution is stated

**Then** the incompleteness is stated with it and cannot be read separately from it.

### Example: nothing is fabricated

**Given** a missing cell at density `0.10`, seed `7`

**When** the rows are read

**Then** no row exists for that cell, and no row for a neighbouring seed or density stands in for it.

### Example: completeness is explicit

**Given** a sweep in which every cell produced a row

**When** the batch completes

**Then** it states that the sweep is complete, and exits with the status reserved for a complete sweep.

### Example: total failure

**Given** a sweep in which no cell produced a row

**When** the batch completes

**Then** it reports that, retains no rows, and states no distribution.

## Open decisions

None.

The exit-status values and the retry policy are the technical owner's in `SPEC-MOK-008`. The product
decisions — that incompleteness is per-cell and reasoned rather than a count, that partial rows remain valid and are not
discarded, that no cell is ever fabricated, and that incompleteness cannot be separated from a distribution — are
settled here and by `INT-MOK-012` principle 6.
