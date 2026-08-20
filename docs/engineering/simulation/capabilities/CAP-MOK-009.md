+++
id = "CAP-MOK-009"
type = "capability"
title = "Measure a run from records the engine writes, additively and in integers"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"

[relations]
derives_from = ["INT-MOK-009"]
+++

# Capability: Measure a run from records the engine writes

## Actor and need

The **product owner** needs a run's facts in a form that supports a claim about many runs, because the project's
success criterion is a statement about a distribution of outcomes and the only surfaces that exist today are a
human-facing line stream and a terminal.

The **assurance owner** needs a retained capture that a program checks. A figure in an evidence file is currently
a figure a bespoke parser produced from a stream designed for a reader, and the parser is not the evidence.

The **technical owner** needs the measurement contract authored once, before Phase 3 adds conflict facts and
Phase 4b consumes distributions, and needs it authored without a new dependency, a new package, or any relaxation
of the engine's authority or determinism.

The **operator** needs to enable measurement without wondering whether it changed the run.

## Capability statement

The engine writes, to a sink a host supplies, one structured record per authoritative text record, one header
record stating the schema version and the run's configuration, one metrics record per completed tick carrying the
engine facts the event stream does not already state, and one terminal record carrying the facts an outcome
classification consumes — all in integers, all deterministic, all additive, so that a program can measure a run
without knowing the engine's rules and without changing what it measures.

## Boundaries

**Included.**

- One sink, configured by one option, absent by default. When it is absent the engine behaves exactly as it does
  today, down to the byte.
- A record framing of one JSON object per line, discriminated by a `record` field with four values, over a value
  alphabet the specification closes so that escaping is provably total.
- The `event` record as a **projection** of the text record: exactly one per emitted text line, in the same order,
  carrying the same facts, such that each text line is reconstructible from its record. The projection holds
  whatever `--trace-actions` is set to, so the two options stay orthogonal.
- A `header` record carrying the schema version, the engine version, and the resolved configuration.
- A `metrics` record per completed tick carrying the living population, deaths, per-territory population and
  standing resources by class, each territory's capacity and permanent-depletion state, and the sum and extremum of
  each of the four dynamic attributes across the living population.
- A terminal `run` record carrying the termination reason, tick count, survivor and death counts, cumulative
  territory crossings, consumption by food class, regeneration and skipped-regeneration counts, and each
  Mokiterion's name, identifier, final territory and death tick — and no classification.
- The cumulative counters the engine must begin retaining to state those figures, which read no entropy.
- Failure behavior that refuses to present a partial stream as a complete run.
- Measured evidence that the text stream, the entropy draw sequence, and every run predating this change are
  unchanged.

**Excluded.**

- Any outcome classification, label, verdict, or judgment in the stream. `REQ-MOK-044` states the refusal
  explicitly rather than leaving it to be inferred from a field list.
- Any average, mean, ratio, percentage, or floating-point value.
- Any conflict, combat, threat, retreat or surrender metric. The engine computes none of them.
- Batch execution across seeds, run persistence beyond the one stream, a run index, and any directory convention.
- Structured output from the observer, whose export stays the text format of `SPEC-MOK-003` rule 9.4.
- Any change to a simulation rule, a survival value, the resource table, the density mapping, the perception
  radius, regeneration, the finality of death, the default configuration, the exit-code contract for cases that
  exist today, or what any decision source proposes.
- Any change to the text stream's bytes or ordering, under any option combination.
- Any new package, any new external dependency, and any growth of the engine's dependency table.
- Any filesystem operation in the engine's library target, which resolves no path and opens no file.
- Any growth of the engine's public interface beyond `execute`'s sink parameter and the items the record writer
  needs, and any relaxation of `SPEC-MOK-002` rule 6.

## Outcomes

- A run's records answer a quantitative question without the engine being re-run and without a parser being
  written for the occasion.
- A classification threshold can change without invalidating a retained record, because no retained record carries
  a classification.
- A comparison across policies is a comparison of records, and the records are the engine's own statements rather
  than a consumer's reconstruction of them.
- Enabling measurement is provably free: the text bytes and the entropy draws are identical, and every capture
  retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011` still reproduces.
- Phase 3's conflict facts and Phase 4b's distributions have a declared place to arrive, announced by a schema
  version rather than discovered by a consumer.
- The facts the observer could see but never state — capacity, permanent depletion, standing supply per class —
  leave the terminal.

## Candidate requirements

- `REQ-MOK-042` — emit one structured record per authoritative text record, as a reconstructible projection.
- `REQ-MOK-043` — report per-tick world metrics the event stream does not already state, in integers.
- `REQ-MOK-044` — report an end-of-run measurement carrying the declared classification facts, and no
  classification.
- `REQ-MOK-045` — leave the text stream, the entropy draw sequence, and every prior run's output unchanged.
- `REQ-MOK-046` — surface a sink failure and refuse to claim a completed run.

No requirement is needed for the sink option itself, for the record framing, or for the schema version: `INT-MOK-009`
principle 7 and `REQ-MOK-042` together oblige a sink to exist and `SPEC-MOK-006` fixes its form, its resolution and
its versioning, in the same way `REQ-MOK-024` delegates every layout threshold to `SPEC-MOK-003`.
