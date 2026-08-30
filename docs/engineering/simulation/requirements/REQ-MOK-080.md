+++
id = "REQ-MOK-080"
type = "requirement"
title = "Count behavioural events the run record omits, attempted and effective separately"
status = "approved"
owners = ["product owner"]
created = "2026-08-30"
updated = "2026-08-30"
statement = "WHEN a batch derives a fact row from a run's structured record stream, THE SYSTEM SHALL count each behavioural event kind the run record does not carry, and SHALL state separately how many times each fired and how many times it changed authoritative state."
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

# Requirement: Count behavioural events the run record omits, attempted and effective separately

## Rationale

Phase 6 is asked to report on aggression, retaliation, surrender and war. The `run` record carries none of them. It
carries survivors, deaths, crossings, consumption and regeneration, and `SPEC-MOK-006` rule 7.8 states the reason: "The
record carries no field for a phenomenon the engine does not compute, and does not carry such a field at zero." The
2026-08-21 amendment to that specification is explicit that when combat arrived "the `run` record gains nothing", and
rule 7.8's stated reason was corrected at the time to rule 10.4's — no approved requirement needed them there.

This is that requirement, and it is satisfied outside the engine. The counts are derived by reading the event stream,
which the batch already reads and already hashes. Measured cost: 0.041 s per thousand-tick run, 0.813 s for twenty
streams totalling 60 MB. The alternative — adding cumulative conflict counters to the run record — would force
`SPEC-MOK-006`'s schema from 3 to 4 under rule 10.2, change the engine, and move every retained stream digest in the
repository. It was considered and declined; `ADR-MOK-008` records that.

**The attempted and effective split is not fastidiousness, it is the finding that produced this clause.** Across 35
runs on 2026-08-30 the stream carried 1,448 `threat_resolved` events. Exactly **one** had `increase > 0`. The other
1,447 targeted a creature whose `fear` was already at its ceiling of 100, so the event fired, was recorded, was
counted — and changed nothing. On the attempted count alone, the seed with 106 inert threats is the most aggressive run
in the batch. On the effective count it is the least. A single number here does not merely lose precision; it inverts
the ordering, and it would have hidden a wholly inert mechanism behind a large, healthy-looking figure.

The two counts are separate facts and neither is derivable from the other. A consumer may take their difference; the
row does not, because a difference is an interpretation and `SPEC-MOK-006` rule 7.7's principle — "A change between
ticks is the consumer's subtraction" — applies to this row for the same reason.

## Preconditions and trigger

The trigger is the batch reading a completed run's stream, in the same pass that computes its digest.

The precondition is that the stream is complete. The counts are over the whole stream or they are not taken.

The requirement is indifferent to whether per-action tracing was enabled. Tracing adds `action_trace` records and
changes no resolution record, so the counts hold over whichever records exist.

## Required response

For each completed run, the system:

- counts the occurrences of each behavioural event kind in the stream that the run record does not already state a
  cumulative figure for, at minimum `attack_resolved`, `threat_resolved` and `surrender_resolved`;
- states, for each such kind where the record vocabulary permits the distinction, both the number of occurrences and
  the number of occurrences that changed authoritative state, using that kind's own recorded fields as the test;
- states each count as an integer, and states a count of zero rather than omitting the field, so that a mechanism that
  never fired is distinguishable from a mechanism that was not counted;
- records the counts in the run's fact row alongside the run record's own figures, without merging the two sets or
  recomputing one from the other.

And in addition:

- For every cumulative figure the run record already states, the derived count must equal it. `SPEC-MOK-006` rule 8.6
  makes that equality a property of the stream, so a disagreement is a defect the batch reveals rather than a
  discrepancy it reconciles.
- No count is a rate, a ratio, a percentage, a mean or a floating-point value.
- The event kinds counted are named from the stream's own closed vocabulary. The vocabulary is fifteen kinds, of which
  fourteen were observed across the 2026-08-30 measurement and the fifteenth, `action_trace`, appears only with
  tracing enabled.
- Where a phenomenon Phase 6 names has **no event kind at all**, the row states no field for it and the absence is
  recorded as a gap rather than approximated. Retreat is such a case: no `retreat_resolved` record exists in the
  vocabulary.

## Failure and boundary behavior

- An event kind present in a stream and absent from the counted set is a defect: the row would understate behaviour
  silently. The counted set is checked against the vocabulary rather than assumed to cover it.
- An event kind whose effectiveness cannot be decided from its own recorded fields is counted for attempts only, and
  the row does not carry an effective count for it. Inventing one would require the batch to model a rule the engine
  owns.
- A stream in which a derived count disagrees with the run record's cumulative figure yields a reported defect. The
  row is not retained with the disagreement papered over, and neither figure is preferred over the other by the batch.
- Zero attempts and zero effective is a valid, meaningful row. It is the shape a run with no conflict has, and it is
  not an absence.
- A count is never inferred from a metrics record's attribute sums. Fear rising is not a threat landing.

## Constraints

- **No change to the engine, no record field added, and `SPEC-MOK-006` is not amended.** The schema version stays at 3
  and no retained digest anywhere in the repository moves.
- **No new external dependency**, and no simulation rule re-implemented in the consumer. The effectiveness test reads
  fields the record already carries; it does not recompute what the engine decided.
- Integers only.
- Which event kinds are counted, which effectiveness test applies to each, the field names, and the disposition of
  kinds with no effectiveness test are `SPEC-MOK-008`'s to fix. This requirement fixes only that the omitted
  behavioural kinds are counted, that attempted and effective are separate facts where the records permit it, and that
  a gap is disclosed rather than filled.

## Acceptance examples

### Example: normal behavior

**Given** a `social` run at a thousand ticks whose stream carries 9 `attack_resolved`, 12 `threat_resolved` and 6
`surrender_resolved` records

**When** the row is derived

**Then** the row states those three attempted counts, and states the effective counts beside them.

### Example: the inversion this clause prevents

**Given** two `social` runs, one with 12 threats and one with 106

**When** both rows are derived

**Then** both state 0 effective threats, so no reading of the two rows ranks the second as the more aggressive run.

### Example: cross-check against the run record

**Given** any run's stream

**When** the batch counts `territory_crossed` records

**Then** the count equals the run record's `crossings`, and a disagreement is reported as a defect.

### Example: a mechanism that never fired

**Given** a `reference` run, whose source proposes no conflict

**When** the row is derived

**Then** the row states attack, threat and surrender counts of 0 rather than omitting those fields.

### Example: a disclosed gap

**Given** Phase 6's interest in retreat

**When** the vocabulary is checked

**Then** no `retreat_resolved` kind exists, the row carries no retreat field, and the gap is recorded rather than
approximated from another kind.

## Open decisions

None.

Whether the run record should eventually gain cumulative conflict counters, making these counts
self-checking under rule 8.6 and removing the need to read a stream, is a separate question the owner considered on
2026-08-30 and deferred; `ADR-MOK-008` records the decision and its cost. The product decisions — that behavioural
counts are stated, that attempted and effective are separate, and that a gap is disclosed — are settled here and by
`INT-MOK-012` principle 5.
