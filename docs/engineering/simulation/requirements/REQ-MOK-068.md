+++
id = "REQ-MOK-068"
type = "requirement"
title = "Leave the four existing decision sources byte-identical, entropy draws included"
status = "approved"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN a simulation runs under the baseline, reference, trait-aware or social decision source, THE SYSTEM SHALL produce the same standard output bytes, the same structured record stream bytes, the same per-tick entropy draw counts and the same exit code as the same configuration produced before the model-backed source existed."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Leave the four existing decision sources byte-identical, entropy draws included

## Rationale

Every figure this repository has published rests on a run under one of four sources, and `INT-MOK-010` carries a
byte-identity promise for `baseline` specifically. Adding a fifth source touches the code that selects a source, the
code that composes an observation and, if care is not taken, the entropy stream itself. A change that moved any of
those would not merely add a source; it would silently retire every recorded measurement, including the survivor
floors `REQ-MOK-014`, `REQ-MOK-034` and `REQ-MOK-058` state and the composition figures `REQ-MOK-060` was amended
against.

The entropy clause is the one that needs stating explicitly, because it is the one an output comparison can miss. Two
runs can print identical text while drawing a different number of values from the shared stream, and the divergence
then appears later, at a different seed or a longer horizon. `WO-MOK-020` established that per-tick draw counts are
compared rather than inferred, and that precedent is adopted here rather than reasoned about again.

## Preconditions and trigger

- A run configured for `baseline`, `reference`, `individual` or `social`, at any seed, tick limit and density in the
  declared verification set.
- A retained capture of the same configuration from a commit before the model-backed source existed, with its recorded
  digest.

## Required response

1. Standard output bytes equal to the retained capture's.
2. Structured record stream bytes equal to the retained capture's, where one was captured.
3. Per-tick entropy draw counts equal to the retained capture's, for the whole run.
4. The same exit code.

## Failure and boundary behavior

- **Any byte differs.** The check fails and reports the first differing offset with the configuration that produced it.
  There is no tolerance and no normalisation step: the comparison is `cmp` on retained bytes.
- **A draw count differs while the output matches.** The check fails. This is the case the entropy clause exists for.
- **The pre-change capture is missing for a configuration.** The configuration is captured at the base commit before
  the change is made, which is what `WO-MOK-025` obliges. A missing capture is a gap in evidence rather than a passing
  check.
- **The model-backed source is selected.** This requirement says nothing about that run. It binds the four existing
  sources only.

## Constraints

- The comparison is made on both sides of the change, at a stated base commit and a stated candidate commit, on the
  declared seed set and every source. Retained captures carry their own digests, and
  `docs/engineering/simulation/evidence/**` is exempt from end-of-line conversion so those digests reproduce on any
  platform.
- No configuration is excluded because the change "cannot" affect it. `baseline` in particular is included, because it
  is the source whose selection is most sensitive to the size of a candidate list and it carries `INT-MOK-010`'s
  promise.
- The observer is included where it drives the same interface, so that an observed run and an unobserved run remain
  byte-identical as `ADR-MOK-006`'s validation list already requires.

## Acceptance examples

### Example: normal behavior

**Given** captures of all four sources at the five declared seeds and the default density, taken at the base commit

**When** the same configurations are run at the candidate commit

**Then** all twenty output streams, all twenty record streams and all twenty per-tick draw-count series compare equal,
and every exit code matches.

### Example: failure behavior

**Given** a candidate that adds the fifth source by extending the candidate list rule 4 offers `baseline`

**When** `baseline` is run at seed 0

**Then** the check fails inside the first ticks, because one extra entry moved `baseline`'s selection, and the failure
is reported against `INT-MOK-010`'s byte-identity promise.

## Open decisions

None.
