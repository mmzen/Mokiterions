+++
id = "REQ-MOK-042"
type = "requirement"
title = "Project every text observation record onto a structured record"
status = "draft"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN the engine emits a text observation record and a structured record sink is configured, THE SYSTEM SHALL emit exactly one structured record carrying the same tick, subject, event type, and result fields, in the same order as the text record."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Project every text observation record onto a structured record

## Rationale

The text record already states every fact `REQ-MOK-010` and `REQ-MOK-012` oblige the engine to state. What it does
not do is let a program read those facts without knowing the format, because the result field is a single
`key:value,key:value` string whose keys differ per event type and whose stability comes from a determinism property
rather than from a schema.

Two ways out exist. A consumer can parse the text — which is what this repository has done for every quantitative
claim so far, and which places a parser the repository wrote between the engine's facts and the conclusion drawn
from them. Or the engine can state the same facts in a structured form. The second is preferred because the engine
already holds the values structured; the text record is the lossy step.

The word that carries the weight is **projection**. The structured stream is not a second, independently authored
account of the run: it is the same facts in a second encoding, one record per line, in the same order. That
relationship is what makes the two streams impossible to drift apart, because it is checkable — a text line
reconstructed from its record must equal the text line the engine wrote, byte for byte. A design that instead let
the structured stream carry *more* or *fewer* events than the text stream would need its own completeness argument
and would create a second thing to keep true. This requirement refuses that.

**Exactly one** is equally deliberate in both directions. One record per text line means no event is dropped; one
text line per record means no event is invented, including for state the engine holds but does not report. Facts
the text stream never states belong to `REQ-MOK-043` and `REQ-MOK-044`, which have their own records and cannot
disturb this correspondence.

## Preconditions and trigger

The trigger is the emission of any text observation record required by `REQ-MOK-010` or `REQ-MOK-012`, at any tick,
under any decision source, any density, and any tick limit, with a structured record sink configured.

The precondition is that a sink is configured. Absent a sink, this requirement obliges nothing, and `REQ-MOK-045`
requires the run to be indistinguishable from one in a build without this capability.

The requirement is indifferent to which options produced the text record. In particular it holds with per-action
tracing both off and on: tracing changes which text records exist, and this requirement then holds of whichever
records do exist. The two options are orthogonal and neither implies the other.

## Required response

For each emitted text observation record, in the order the engine emits them, the system writes to the sink exactly
one structured record that carries:

- the same tick,
- the same subject — the same agent, territory, or resource identifier the text record names, or the same absence of
  a subject where the text record names none,
- the same event type, drawn from the same closed vocabulary, in a form a consumer can compare for equality without
  interpreting it, and
- the same result facts, decomposed into named fields rather than carried as one string, with each value in the
  domain and units the text record uses.

And in addition:

- The records appear in the sink in the same relative order as their text counterparts. Where the text stream's
  order is authoritative event order, so is the structured stream's.
- Each text record is reconstructible from its structured record: given the record and the format rules, a consumer
  can produce the exact bytes of the text line, and no fact present in the text line is absent from the record.
- The correspondence is total in both directions within a run. There is no emitted text observation record without a
  structured record, and no structured event record without an emitted text observation record.
- The stream states its own schema version and the configuration that produced it, so a retained sink is
  interpretable without the command line that created it.
- Two runs with the same seed, configuration and sink configuration write byte-identical sinks.

## Failure and boundary behavior

- A sink write failure is `REQ-MOK-046`'s subject. This requirement does not weaken it: a record that cannot be
  written is not treated as written, and the correspondence is not repaired by silently skipping a record.
- Where a text record's result carries no fields, the structured record carries the tick, subject and event type and
  an empty result. It is not omitted, and it is not given invented fields.
- A run that emits no observation record at all cannot occur — initialization always emits — but if it did, the
  correspondence would hold vacuously and the stream would still carry its schema version and configuration.
- The record must not carry a value the text record does not state and the engine did not derive from the same
  authoritative state at the same instant. Restating a value read later in the tick is a defect, not an
  optimization.
- No record carries a wall-clock time, a hostname, a filesystem path, an environment value, a process identifier, or
  a credential. The subject vocabulary is the engine's own identifiers.
- No record carries a floating-point value, a mean, a ratio or a percentage, including where the text record's own
  value is a decimal string. Where a decimal appears in the text stream the structured record carries it in the same
  lossless form the engine holds it in.

## Constraints

- Additive. The text stream's bytes, ordering and content are unchanged; `REQ-MOK-045` states that obligation and
  this requirement may not be read as licensing any exception to it.
- No new external dependency, and the engine package's dependency table stays empty. A serialization library is not
  available.
- The engine's library target resolves, opens and interprets no filesystem path. It writes to a sink a host
  supplies.
- The public interface grows only by what this requirement needs, under `SPEC-MOK-002` rule 5's growth clause, and
  `SPEC-MOK-002` rule 6 is not relaxed: nothing added here yields a mutable borrow of, or a reference into,
  authoritative state in any build configuration including tests.
- Integer and closed-vocabulary values only. The concrete syntax, the record framing, the field names, the event-type
  spelling, the escaping rules and the value alphabet are `SPEC-MOK-006`'s to fix; this requirement fixes only that
  the correspondence exists, is one-to-one, is order-preserving and is reconstructible.

## Acceptance examples

### Example: normal behavior

**Given** a run at seed `42`, default density, `--policy reference`, one hundred ticks, with a sink configured

**When** the run completes

**Then** the sink holds exactly as many event records as the run wrote text observation records, the *n*-th event
record corresponds to the *n*-th text record, and reconstructing each text line from its record reproduces the
standard-output stream byte for byte.

### Example: tracing does not disturb the correspondence

**Given** the same run with per-action tracing enabled

**When** the run completes

**Then** the correspondence still holds one-to-one over the larger set of text records, and every action trace has
exactly one structured record.

### Example: reproducibility

**Given** two processes at seed `123` with identical options and sink configuration

**When** both complete

**Then** the two sinks are byte-identical.

### Example: failure behavior

**Given** a sink that fails partway through a run

**When** the engine attempts the failing write

**Then** the failure is surfaced, the run does not claim successful completion, and the partial stream is not
presented as a complete run — as `REQ-MOK-046` requires.

## Open decisions

None. The record syntax, framing, vocabulary spelling, field names and value alphabet are the technical owner's to
fix in `SPEC-MOK-006`. The product decisions — that the structured stream is a projection of the text stream rather
than an independent account, and that the text stream remains authoritative and unchanged — are settled here and by
`INT-MOK-009` principle 1.
