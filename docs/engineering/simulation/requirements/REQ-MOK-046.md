+++
id = "REQ-MOK-046"
type = "requirement"
title = "Surface a record sink failure and refuse to claim a completed run"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a structured record sink cannot be opened or a record cannot be written to it, THE SYSTEM SHALL report the failure on the diagnostic stream, terminate with the runtime failure exit code, and leave no partial record stream that reads as a complete run."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Surface a record sink failure and refuse to claim a completed run

## Rationale

A record stream exists to be read later by a program, usually with nobody watching. That is precisely the condition
under which a silent truncation does the most damage: the consumer sees a run that ended at tick 137 with nine
survivors and has no way to know the run actually reached tick 200. A truncated stream is not incomplete data, it is
**wrong** data, because every terminal figure it appears to support is missing and every per-tick figure it does
carry describes a run whose end the reader has invented.

The repository already holds both halves of the answer. `REQ-MOK-010`'s failure clause established that "output
failure is surfaced rather than silently discarding events" and that "the run does not claim successful completion",
and `SPEC-MOK-001` maps a standard-output write failure to runtime exit code `1` with no summary claimed. Separately,
`SPEC-MOK-003` rules 9.5 and 9.6 established the file half on the observer's side: report the failure, and remove the
partial file rather than leave it. This requirement is those two precedents applied to a new sink, not a new policy.

Removal rather than retention is deliberate. A partial file left on disk is indistinguishable from a complete one to
any later reader, including the operator who finds it a week later, and the terminal record's absence is a weak
signal to rely on when the alternative is no file at all. An absent file cannot be misread.

The distinction between a sink that cannot be opened and a record that cannot be written matters because they happen
at different times and cost different things. The first happens before the run and costs nothing; the second happens
mid-run, after the text stream has already told the truth about part of the run, and the process must then decide
what to do with the part-written sink.

## Preconditions and trigger

Two triggers, both requiring a sink to have been configured:

1. The configured sink cannot be opened or prepared, before the first tick.
2. A write to an opened sink fails, or the sink cannot be flushed and closed successfully at the end of the run,
   at any point during or after the run.

The requirement is silent about, and does not apply to, a run with no sink configured. It also does not displace the
existing standard-output failure behavior: a text-stream write failure remains what `REQ-MOK-010` and `SPEC-MOK-001`
already make it, and this requirement adds a second failure source with the same exit code rather than a second exit
code.

## Required response

**When the sink cannot be opened**, before any tick runs:

- The system reports the failure, naming the sink and the reason, on the diagnostic stream.
- The system terminates with the runtime failure exit code.
- No tick runs, no text observation record is emitted, and no simulation output is produced. A run that cannot be
  recorded is not run, because a partly-recorded run is the outcome this requirement exists to prevent.

**When a write to an opened sink fails**, at any point:

- The system reports the failure, naming the reason, on the diagnostic stream.
- The system terminates with the runtime failure exit code.
- The system does not claim successful completion: no success summary is presented as the run's result.
- The system leaves no partial record stream that a later reader can mistake for a complete run. Where the sink is a
  file the process created, the partial file is removed.
- The failure is not retried, not suppressed, and not deferred to the end of the run. A record that could not be
  written is not treated as written, and the run does not continue writing records past a failure.

**In both cases:**

- The exit code is the same runtime failure code an existing standard-output write failure produces. This capability
  introduces no new exit code, and `SPEC-MOK-002`'s enumerated exit-code contract is unchanged.
- The diagnostic message is deterministic in form, distinguishes a sink failure from a standard-output failure, and
  carries no credential and no value beyond the sink's identity and the underlying reason.

## Failure and boundary behavior

- A malformed sink argument — absent, empty, or one of the spellings the specification reserves because they would
  make the sink interleave with the text stream — is an invalid *configuration*, rejected before the run with the
  configuration error exit code and the usage text, exactly as every other malformed option is. Whether the
  filesystem accepts a well-formed path is a runtime matter and carries the runtime failure code. The two are
  distinguished by whether the argument's form is wrong or the environment refused it.
- An existing file at the sink's destination is overwritten without prompting and is not a failure. This matches
  `SPEC-MOK-003` rule 9.4's existing behavior for the observer's export; a differing rule for the engine would be
  surprising in a repository with one file-writing precedent.
- Removal is limited to a sink the process itself created. The process does not delete a destination it did not
  create, and where it cannot determine that it created the destination, it does not remove it and says so.
- A failure of the removal itself does not mask the original failure. The write failure is what is reported and what
  determines the exit code; a failed cleanup is reported additionally, never instead.
- A failure while writing the terminal record is a failure like any other. The run's last record being absent is not
  a lesser fault, because the terminal record is the one a consumer needs most.
- A failure on flush or close at the end of the run is a write failure. A record buffered and never flushed was not
  written, and reporting success would be a false claim.
- The bytes written to standard output before the failure remain what they were. This requirement does not retract
  or rewrite the text stream, and `REQ-MOK-045` continues to hold up to the point of failure.
- Simultaneous failure of both streams reports both if it can and otherwise reports what it can. The exit code is
  the same runtime failure code either way, so no precedence question arises.

## Constraints

- No new exit code, and no change to the meaning of an existing one.
- No new external dependency; the engine package's dependency table stays empty.
- The engine's library target performs no filesystem operation. It writes to a sink it was given and reports a write
  failure to its caller; opening, creating and removing a destination belong to the host that resolved the path. The
  library therefore cannot itself remove a partial file, and this requirement's removal obligation falls on the host
  — which is where the equivalent obligation already sits in `mokiterions-tui`.
- The diagnostic stream is the existing standard-error stream. No new stream, no log file, no second diagnostic
  destination.
- Failure behavior is deterministic: the same failure produces the same message form and the same exit code on every
  run and every target.
- The message wording, the reserved sink spellings, and how the host determines that it created the destination are
  `SPEC-MOK-006`'s to fix. This requirement fixes that the failure is surfaced, that the exit code is the existing
  runtime failure code, and that no partial stream survives to be misread.

## Acceptance examples

### Example: normal behavior

**Given** a sink destination in a directory that does not exist

**When** the engine is invoked with that sink and any otherwise valid options

**Then** the failure and its reason are reported on standard error, the process exits with the runtime failure code,
no tick runs, and standard output carries no observation record.

### Example: a write fails mid-run

**Given** a sink that accepts records and then fails on a write during the run

**When** the failing write is attempted

**Then** the failure is reported on standard error, the process exits with the runtime failure code, no successful
summary is claimed, and the partial sink the process created no longer exists.

### Example: an existing destination

**Given** a sink destination that already holds a file from an earlier run

**When** a new run is invoked with the same destination and completes

**Then** the file holds only the new run's records, and the overwrite is not reported as a failure.

### Example: failure behavior

**Given** a sink argument whose form the specification reserves because it would interleave with the text stream

**When** the engine is invoked with it

**Then** the configuration is rejected before the run with the configuration error exit code and the usage text on
standard error, and no sink is created.

## Open decisions

None. Message wording, the reserved spellings, and the host-side creation test are the technical owner's to fix in
`SPEC-MOK-006`. The product decisions — that a sink failure is surfaced rather than tolerated, that it carries the
existing runtime failure code rather than a new one, that a partial stream is removed rather than retained, and that
an existing destination is overwritten — are settled here, by `REQ-MOK-010`'s failure clause, and by `SPEC-MOK-003`
rules 9.4 to 9.6.
