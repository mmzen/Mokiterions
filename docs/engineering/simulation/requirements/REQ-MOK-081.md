+++
id = "REQ-MOK-081"
type = "requirement"
title = "Classify a run's outcome from stated facts, with every threshold outside every retained artifact"
status = "approved"
owners = ["product owner"]
created = "2026-08-30"
updated = "2026-08-30"
statement = "WHEN a retained fact row is read, THE SYSTEM SHALL compute exactly one outcome class for that run from a declared predicate over the row's stated facts, and SHALL hold every threshold and every class definition outside every retained artifact, so that changing a definition changes no retained row."
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

# Requirement: Classify a run's outcome from stated facts, with every threshold outside every retained artifact

## Rationale

`SPEC-MOK-006` rule 8.7 forbids the engine to state a classification, in any record kind, and gives the reason in one
sentence: "Classification is Phase 4b's, and a threshold must be revisable without invalidating a retained capture."
That refusal was correct and it left the classification unowned. Nothing in this repository defines what *extinction*
or *collapse* means as a decidable predicate, so no run has ever been classified and Phase 6 has nothing to count.

The revisability requirement is the substance of this requirement, not a caveat on it. A threshold is a judgement, and
judgements about this simulation will change: the measurement of 2026-08-30 found no famine anywhere in 35 runs —
`regeneration_skipped.depleted` was 0 in every one — which either means the class needs a different predicate or means
the conditions that produce it were not swept. Either way, someone will edit the definition. If any retained row
carried the label, every row retained before the edit would be silently wrong, and a repository that had published a
distribution would have published something it could no longer reproduce.

So the split is: **rows carry facts, the classifier carries judgement.** A retained row states that a run ended with 0
survivors; it does not state that this was *extinction*. That the two are obviously the same thing today is exactly why
the separation must be structural rather than trusted — the obvious cases are not the ones that get revised.

**Exactly one class per run** is deliberate. Overlapping classes make a distribution's columns not sum to its row
count, and a reader cannot tell an overlap from a double count. The predicates are therefore ordered and the first
match wins, and the order is part of the definition rather than an implementation accident.

## Preconditions and trigger

The trigger is reading a retained fact row, whether immediately after a batch or long afterwards.

The preconditions are that the row is complete and that its figures are the ones `REQ-MOK-078` and `REQ-MOK-080`
require. Classification reads rows; it never reads a stream and never executes the engine.

## Required response

For each retained row read, the system:

- computes exactly one outcome class, from a declared, ordered set of predicates over facts the row states;
- draws every class name from a closed vocabulary that covers at minimum extinction, collapse, asymmetric collapse,
  famine and coexistence;
- states, for any run, which predicate matched, so that a class is traceable to the clause that assigned it rather than
  to the classifier as a whole;
- holds every threshold, every predicate and the whole vocabulary in the classifier, and writes none of them into any
  retained row.

And in addition:

- Re-running the classifier over unchanged rows produces unchanged classes. Classification is a function of the row and
  the definitions, and of nothing else — not of the order rows are read, not of how many rows are present, and not of
  any other row's contents.
- Changing a threshold changes classes and changes **no** retained row. This is checkable by digesting the retained
  rows before and after a threshold edit.
- The classifier reads no event stream and re-implements no simulation rule. If a class cannot be decided from stated
  facts, the fact it needs is `REQ-MOK-078`'s or `REQ-MOK-080`'s to state, and the class is not decided by reaching
  around them.
- A class the sweep never produced is reported as observed zero times rather than omitted from the vocabulary. An
  unobserved class is a finding about the simulation or about the sweep, and deleting it would erase the finding.
- No class is a judgement about whether the run was good, correct, expected or acceptable. The classes name what
  happened.

## Failure and boundary behavior

- A row matching no predicate is a defect in the definitions, and it is reported as an unclassified row naming the
  facts that fell through. It is not assigned a default class and it is not dropped, because both would hide the gap.
- A row matching more than one predicate is not possible, because the predicates are ordered and the first match wins;
  but a definition set in which two predicates could match the same row with the order reversed is a defect worth
  reporting, since it means the order is carrying meaning that the definitions do not state.
- A row missing a fact a predicate needs is reported as unclassifiable for a stated reason, not classified on the
  facts that are present.
- A class whose predicate references a threshold that no measurement supports is admissible, and the absence of
  support is stated. Famine is such a case today.
- The classifier never edits, rewrites or annotates a retained row. Its output is a separate thing, and a retained row
  is immutable once written.

## Constraints

- **No change to the engine and no amendment to `SPEC-MOK-006`.** This requirement is the discharge of that
  specification's rule 8.7, not a revision of it.
- **No threshold, class name, label, verdict or severity in any retained artifact**, including any evidence file that a
  batch produces. A human-readable report may of course state classes; it is a rendering, and it is regenerable.
- **No new external dependency**, and no floating-point comparison in a predicate. Thresholds are integers or
  closed-vocabulary values, on `SPEC-MOK-006` rule 12.4's reasoning.
- The class vocabulary's exact members, each predicate's exact form, the ordering, the thresholds' values, and the
  file the definitions live in are `SPEC-MOK-008`'s to fix. This requirement fixes only that exactly one class is
  computed per run, that it comes from stated facts, that the definitions live outside retained data, and that an
  unmatched row is reported rather than defaulted.

## Acceptance examples

### Example: normal behavior

**Given** a retained row stating 0 survivors and 12 deaths with reason `extinction`

**When** it is classified

**Then** it is assigned exactly one class, and the clause that assigned it is stated.

### Example: revisability, which is the point

**Given** 400 retained rows, and a digest over them

**When** a threshold in the classifier is changed and the classifier is re-run

**Then** the classes change, the digest over the retained rows is unchanged, and no row was rewritten.

### Example: an unobserved class is still reported

**Given** the 2026-08-30 measurement, in which `regeneration_skipped.depleted` was 0 in all 35 runs

**When** the distribution is computed

**Then** famine is reported as observed 0 times, and its absence is a stated finding rather than a missing column.

### Example: a row that falls through

**Given** a row whose facts match no predicate

**When** it is classified

**Then** it is reported as unclassified with its facts named, and it is neither defaulted nor dropped.

### Example: determinism

**Given** the same rows and the same definitions

**When** the classifier runs twice

**Then** the two outputs are byte-identical.

## Open decisions

None.

The vocabulary's members and each threshold's value are the technical owner's in `SPEC-MOK-008`,
which must state for each one whether measurement supports it. The product decisions — that a classification exists at
all, that it is exactly one class per run, that its definitions live outside retained data, and that an unobserved class
is reported rather than removed — are settled here and by `INT-MOK-012` principle 3.
