+++
id = "REQ-MOK-044"
type = "requirement"
title = "Report an end-of-run measurement carrying no classification"
status = "draft"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a run terminates and a structured record sink is configured, THE SYSTEM SHALL emit one run record stating the termination reason, the tick count reached, the survivor and death counts, the cumulative territory crossing count, the consumption count by resource class, the regeneration and skipped-regeneration counts, and each Mokiterion's identifier, name, final territory and death tick, and SHALL NOT state an outcome classification."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Report an end-of-run measurement carrying no classification

## Rationale

A claim about a *distribution* of outcomes is a claim about one figure per run. Reading it out of the per-tick
records means reading the whole run to learn its one result, and reading it out of the event stream means summing
records and hoping none was missed. One terminal record per run makes a run's result a single line, so a hundred
runs are a hundred lines and a distribution is a count over them.

The existing text summary line already states four of these figures — reason, ticks, survivors, deaths — and stops
there. The rest are facts the engine has never stated anywhere: how many crossings a run saw, how much of each
resource class was eaten, how often regeneration ran and how often it was skipped and why, and when each Mokiterion
died. The engine does not even *retain* most of them today; there is no crossing counter, no consumption total and
no death tick in its state. Retaining them is the substance of this requirement, and it is why this requirement is
the largest of the five in implementation cost while being the smallest in output.

The negative clause is not decoration. Phase 4b will classify runs — famine, collapse, coexistence, or whatever
vocabulary is approved — and a classification is an interpretation of these facts against thresholds nobody has
chosen yet. If the engine wrote the label, then changing a threshold would invalidate every retained record, and the
records are meant to outlive the thresholds. Worse, a run's classification would become an engine behavior subject
to `REQ-MOK-009`'s determinism guarantee, which would make a threshold change a change to the simulation's
observable output. Keeping the label out keeps the facts durable and keeps the classification cheap to revise. So
the refusal is stated as an obligation of this requirement rather than left to be inferred from the field list.

Per-Mokiterion outcomes are included because the roster is the unit of the project's central claim. "Nine survived"
and "nine survived, and the three who died all died in territory B before tick 30" are different findings, and only
the second distinguishes a world from a coincidence.

## Preconditions and trigger

The trigger is termination of a run for any reason the engine recognizes — reaching the tick limit, or any earlier
termination condition — with a structured record sink configured.

The record is emitted once per run, after the final tick's metrics record and after every event record. It is
additional to the text summary line `REQ-MOK-010` requires and does not replace it.

Absent a sink this requirement obliges nothing, and `REQ-MOK-045` requires the run to be indistinguishable from one
in a build without this capability.

A run that terminates because a write failed is not a terminated run in this sense; it is `REQ-MOK-046`'s subject,
and this record is not written for it.

## Required response

On termination the system writes to the sink one run record stating:

- the termination reason, from the same closed vocabulary the text summary uses,
- the tick count the run reached,
- the survivor count and the death count,
- the cumulative count of territory crossings over the whole run,
- the cumulative count of resource consumptions over the whole run, by resource class,
- the cumulative count of regeneration events, and the cumulative count of skipped regenerations, distinguished by
  the reason each was skipped, and
- for each Mokiterion the run created, whether living or dead: its identifier, its name, the territory it stood in
  at termination or at death, and the tick at which it died, stated as explicitly absent for a survivor.

And in addition:

- Every count is an integer, and every count is cumulative over the run rather than a rate or an average.
- The record states no outcome classification, label, category, verdict, judgment, severity, or interpretation of
  the figures it carries. It states what happened and not what it means.
- The per-Mokiterion entries appear in a deterministic order fixed by the specification.
- The figures are consistent with the records that precede them: the survivor and death counts equal the final
  metrics record's, and each cumulative count equals the number of corresponding event records in the same stream.
- Two runs with the same seed, configuration and sink configuration produce byte-identical run records.

## Failure and boundary behavior

- A survivor's death tick is stated as explicitly absent, not as zero and not as the tick limit. Tick `0` is a
  legitimate death tick under some configurations, so a sentinel would be ambiguous.
- A run in which nothing was consumed, nothing regenerated, nobody crossed and nobody died states zeros for those
  counts. A zero here is a measurement — the engine computed it — which is exactly the distinction `REQ-MOK-043`'s
  constraint on absent phenomena draws.
- A skipped regeneration is reported by reason. "Skipped because the territory was at capacity" and "skipped because
  the territory is permanently depleted" are different world states, and collapsing them would lose the difference
  between a healthy world and a dead one.
- Where the whole population dies, the record still states every Mokiterion, each with its death tick, and the
  survivor count is zero. A run with no survivors is a result, not an error.
- A dead Mokiterion's final territory is the territory it stood in when it died. Death is final, so this value is
  stable from that tick onward.
- A cumulative count cannot decrease and cannot exceed what the run's events support. A counter that disagreed with
  the event stream is a defect the stream itself reveals.
- A sink write failure while this record is written is `REQ-MOK-046`'s subject, and a run whose terminal record is
  incomplete must not be readable as a complete run.
- No record carries a wall-clock time, a duration, a hostname, a filesystem path, an environment value or a
  credential. Survival time is measured in ticks.

## Constraints

- No outcome classification, in this record or any other, under any option. Classification is Phase 4b and is
  unauthorized; `CAP-MOK-009` excludes it.
- No conflict, combat, threat, retreat or surrender figure, and none emitted as zero. The engine computes none of
  them; they arrive with Phase 3 under a schema version increment.
- No average, mean, ratio, percentage or floating-point value. Where an average is wanted the record carries a sum
  and a count.
- Additive, per `REQ-MOK-045`. The counters this requirement obliges the engine to retain must draw no entropy and
  must not alter any simulation rule, any decision, any applied action or the text stream. Every run recorded before
  they existed must still reproduce byte for byte.
- No new external dependency; the engine package's dependency table stays empty.
- The engine's library target resolves and opens no filesystem path.
- `SPEC-MOK-002` rule 6 is not relaxed by the counters or by anything that reads them.
- The field names, the termination-reason spelling, the resource-class vocabulary, the skip-reason vocabulary, the
  per-Mokiterion ordering, how absence is encoded, and the concrete syntax are `SPEC-MOK-006`'s to fix. This
  requirement fixes which facts are stated, that they are integers, that they are cumulative, and that no
  classification is among them.

## Acceptance examples

### Example: normal behavior

**Given** a run at seed `42`, default density, `--policy reference`, two hundred ticks, with a sink configured

**When** the run terminates at the tick limit

**Then** the sink's last record states the termination reason, `200` ticks, the survivor and death counts, the
cumulative crossing count, consumption counts for each resource class, regeneration and skipped-regeneration counts
by reason, and one entry per Mokiterion with its identifier, name, final territory and death tick or explicit
absence thereof.

### Example: the record carries no judgment

**Given** any run at any seed, density, policy and tick limit, with a sink configured

**When** the run terminates

**Then** no field of the run record, and no field of any other record, states a classification, label, category,
verdict or severity for the run.

### Example: the counts agree with the stream

**Given** the same sink

**When** a consumer counts the territory-crossing, consumption, regeneration and skipped-regeneration event records

**Then** each count equals the corresponding cumulative figure in the run record, and the per-class consumption
counts sum to the total number of consumption event records.

### Example: total population loss

**Given** a seed and density at which every Mokiterion dies before the tick limit

**When** the run terminates

**Then** the run record states a survivor count of zero, a death count of twelve, a death tick for every
Mokiterion, and no classification of the run as a failure, a collapse or anything else.

### Example: failure behavior

**Given** a sink that fails while the run record is written

**When** the engine attempts that write

**Then** the failure is surfaced, the run does not claim successful completion, and the truncated stream is not
presented as a complete run, per `REQ-MOK-046`.

## Open decisions

None. Field names, vocabularies, ordering and absence encoding are the technical owner's to fix in `SPEC-MOK-006`.
The product decisions — that the terminal record carries the facts a classification consumes, that it carries no
classification, that survival is measured in ticks, and that per-Mokiterion outcomes are part of a run's result —
are settled here and by `INT-MOK-009` principle 3.
