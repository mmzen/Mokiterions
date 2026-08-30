+++
id = "REQ-MOK-079"
type = "requirement"
title = "Bind each retained row to the stream it came from, then discard the stream"
status = "draft"
owners = ["product owner"]
created = "2026-08-30"
updated = "2026-08-30"
statement = "WHEN a batch derives a fact row from a run's structured record stream, THE SYSTEM SHALL record in that row the cryptographic digest of the stream it read and the engine version that wrote it, and SHALL then discard the stream, such that the row's figures are re-derivable from the row's own coordinates."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-012"]
+++

# Requirement: Bind each retained row to the stream it came from, then discard the stream

## Rationale

A distribution needs many runs and a repository cannot hold their streams. Four hundred thousand-tick runs produce
about 1.2 GB. The rows derived from them are about 500 KB. Discarding the streams is not optional at this scale, and
discarding evidence is exactly the move that turns a measurement into an assertion.

The digest is what makes it safe rather than convenient. Determinism was measured on 2026-08-30 and it holds in both
directions that matter: the same coordinates produce a byte-identical stream on re-execution, and — the part that was
not certain in advance — a **debug build and a release build produce the same bytes**, both hashing to `ed1da4b6…`
for `social` at seed 2 over a thousand ticks. So a reader who has the repository and the row's coordinates can
regenerate the exact stream and confirm the digest, without the repository having kept it, and without needing to know
which profile the original batch used.

This is the same argument `SPEC-MOK-006` rule 8.6 makes inside a stream — "a disagreement is a defect the stream itself
reveals" — moved out one level. The row is checkable against a thing that no longer exists, because the thing is
reconstructible.

**The engine version is part of the binding, not decoration.** A digest identifies bytes; it does not say what
produced them. If the engine changes, the same coordinates produce different bytes, and a row whose digest no longer
reproduces must be distinguishable from a row that was wrong when written. The version is what makes the first case
legible as history rather than as corruption.

## Preconditions and trigger

The trigger is the completion of one cell's run and the reading of its stream.

The precondition is that the stream exists and was written completely. A stream whose run did not complete is
`REQ-MOK-083`'s subject and yields no row.

## Required response

For each completed run, the system:

- computes a cryptographic digest over the exact bytes of the structured record stream as written, before any
  transformation, normalisation or line-ending change;
- records that digest in the run's fact row, together with the engine version the stream's header record states;
- records enough of the cell's coordinates that the stream is regenerable from the row alone — decision source, food
  density, seed, tick horizon, and any other option the batch varied;
- discards the stream after the row is complete.

And in addition:

- The digest is over the stream the engine wrote, not over the row derived from it. A row that hashed its own contents
  would prove nothing about the run.
- The digest algorithm is named in the retained output, so a later change of algorithm does not make old rows
  ambiguous.
- No row retains a filesystem path, a hostname, a process identifier, a wall-clock time or an environment value. The
  temporary stream's location is the batch's private business and is not a fact about the run.
- Re-executing a row's coordinates on the same commit reproduces the stated digest, in either build profile.

## Failure and boundary behavior

- A stream that cannot be read after being written is a cell failure, not a row with an absent digest.
- A stream that cannot be removed after being read does not fail the run and does not fail the batch. The row is
  complete; the leftover file is an operator's to clean, and the batch says so rather than treating it as an error.
- A digest that does not reproduce on re-execution is a **defect this requirement exists to expose**. It means either
  the engine changed without the version changing, or determinism has been broken. Neither is repaired by re-recording
  the digest, and a batch offering to update a stale digest in place would defeat the requirement.
- Where the operator asks for it, a stream may be retained instead of discarded for a named cell. Retention is a
  debugging affordance and never a precondition of a row being valid.
- No row may be retained without a digest. A row with an unbound figure is weaker evidence than no row.

## Constraints

- **No change to the engine**, and in particular no digest computed inside it. The engine writes; the batch hashes.
  `SPEC-MOK-006` is not amended and no record gains a field.
- **No new external dependency.** A digest implementation must already be available to the instrument's runtime.
- Integer and closed-vocabulary values only; a digest is recorded as its lowercase hexadecimal string.
- The digest algorithm, its field name, the field naming the algorithm, the retention affordance's form and the
  temporary path convention are `SPEC-MOK-008`'s to fix. This requirement fixes only that every row is bound, that the
  binding is over the stream as written, that the engine version accompanies it, and that the stream is then
  discardable without loss.

## Acceptance examples

### Example: normal behavior

**Given** a batch over three cells

**When** it completes

**Then** each of the three rows carries a digest and an engine version, no stream remains on disk, and re-executing
any one cell's coordinates produces a stream whose digest equals the one that row states.

### Example: the profile-independence that makes it work

**Given** one cell executed from a debug build and the same cell from a release build

**When** both streams are hashed

**Then** the two digests are equal, so a row does not need to record which profile produced it.

### Example: a broken binding is visible

**Given** a retained row and a later commit that changes the engine's output without changing its version

**When** the row's coordinates are re-executed and hashed

**Then** the digest disagrees, and the disagreement is reported as a defect rather than reconciled by rewriting the
row.

### Example: no path leaks into evidence

**Given** any retained row

**When** it is read

**Then** it contains no filesystem path, no hostname, no timestamp and no environment value.

## Open decisions

None. The choice of digest algorithm is the technical owner's in `SPEC-MOK-008`. The product decision — that streams
are discarded and rows are bound rather than streams being retained or rows being trusted — is settled here and by
`INT-MOK-012` principle 4, on the measured determinism this rationale states.
