+++
id = "REQ-MOK-019"
type = "requirement"
title = "Present and export a filterable event log"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN events occur during an observed run, THE SYSTEM SHALL present them tick-stamped and typed in authoritative order, allow the presentation to be restricted to a single event type or a single subject, and on operator request write the unrestricted observed events to a named file."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-003"]
+++

# Requirement: Present and export a filterable event log

## Rationale

An observation that leaves nothing behind cannot enter the assurance chain. `VREC-MOK-002` binds eleven evidence
files, and every claim it makes is re-readable because of them. A terminal interface whose output is a fleeting
screen would make behavioral claims something an operator reports rather than something a reviewer re-reads,
which weakens rather than strengthens the chain that `VREC-MOK-001` and `VREC-MOK-002` established. Export is
therefore part of the requirement, not a convenience attached to it.

Filtering exists because the log's density is the problem the interface is solving. Twelve agents at one event or
more per agent-tick means a 1,000-tick run produces thousands of records, and the operator's question is almost
always about one event type, such as deaths or rejections, or about one subject, such as a single Mokiterion or a
single territory.

Filtering the view while exporting everything is deliberate. A filtered export would produce evidence whose
completeness depends on an interface setting at the moment the operator pressed a key, and a reviewer could not
tell a filtered file from a complete one.

## Preconditions and trigger

An observed run has been initialized. The trigger for presentation is any authoritative event the engine emits.
The trigger for export is an explicit operator request.

## Required response

**Presentation.** Each event appears with its tick, its subject, its type, and its material result, in
authoritative event order. Events are presented as they occur, and the operator can see the most recent events
without acting.

**Filtering.** The operator can restrict the presentation to a single event type, or to a single subject, and can
clear the restriction. Filtering changes only what is displayed; it does not discard events from the retained set
and does not alter the run.

**Export.** On request, the observer writes the events it has observed for the run to a file. The exported set is
unrestricted regardless of any active filter, the records carry the same fields in the same order as the
presentation, and the file's contents are identical for two runs sharing seed, configuration and decision source.

## Failure and boundary behavior

- When events accumulate beyond what the interface retains, the retention bound is declared, and the operator is
  told that earlier events are no longer presentable rather than being shown a silently truncated log. An export
  states the same bound.
- When a filter matches no event, the presentation states that the filter matched nothing rather than appearing
  empty in a way indistinguishable from an idle run.
- When export fails — an unwritable path, a full volume, a rejected filename — the failure is surfaced to the
  operator, the run is not terminated, and no partially written file is presented as a complete export.
- An operator-supplied export path is treated as data. It is never interpreted as code, and it is not used to
  read from the filesystem.

## Constraints

- Exported records contain no wall-clock timestamp, no absolute filesystem path, and no environment-specific
  value, since `REQ-MOK-009` requires comparable output to be free of nondeterministic content and an
  unreproducible export cannot serve as evidence.
- Export contains no credential or secret.
- The event vocabulary, field order, retention bound, filter grammar, export format and default filename are
  fixed by `SPEC-MOK-002`. Where an event corresponds to one already specified by `SPEC-MOK-001`, its fields and
  vocabulary are those `SPEC-MOK-001` fixes.
- Presenting, filtering and exporting consume no simulation entropy and do not mutate simulation state.
- Export does not replace the `REQ-MOK-010` text stream, which remains available and unchanged.

## Acceptance examples

### Example: normal behavior

**Given** an observed 200-tick run in which three Mokiterions die

**When** the operator filters the presentation to death events and then exports

**Then** the presentation shows three records with their ticks and subjects, and the exported file contains every
observed event for the run rather than only the three deaths.

### Example: failure behavior

**Given** the operator requests an export to a path that cannot be written

**When** the export is attempted

**Then** the failure is reported to the operator, the observed run continues, and no file is presented as a
successful export.

## Open decisions

None. Vocabulary, field order, retention bound, filter grammar, export format and default filename are fixed by
`SPEC-MOK-002`.
