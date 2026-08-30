+++
id = "CAP-MOK-012"
type = "capability"
title = "Sweep a configuration space, retain facts bound to their source, and classify outcomes revisably"
status = "approved"
owners = ["product owner"]
created = "2026-08-30"
updated = "2026-08-30"

[relations]
derives_from = ["INT-MOK-012"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-30T16:11:24Z"
decided_by = "product owner"
reason = "Approved by the repository owner on 2026-08-30, by selecting the presented option, as part of the twelve-artifact Phase 4b chain. The chain converts docs/ROADMAP.md's Phase 4b open question into an approved shape on the strength of the measurement the roadmap reserved that decision to, taken at c90edc9 and recorded in ADR-MOK-008. It carries three disclosed and unrepaired findings: the threat mechanism is inert in 1447 of 1448 firings, the famine predicate is unreached in the swept space with food still standing at extinction, and no retreat event kind exists."
+++

# Capability: Sweep a configuration space, retain facts bound to their source, and classify outcomes revisably

## Actor and need

The **product owner** needs a distribution, because `INT-MOK-001`'s claim is about one and the repository can
currently produce only single runs. Thirty-five runs were enough to find a mechanism that fired 1,448 times and
worked once; one run was not, and had never been.

The **assurance owner** needs a retained row that is checkable without paying to reproduce it. A row carrying its
sweep coordinates and the digest of the stream it was derived from is checkable by re-running one cell. A prose figure
is not.

The **technical owner** needs the classification thresholds outside the engine and outside anything a release
freezes, so that revising a threshold is not a product change and does not invalidate a retained capture — the exact
property `SPEC-MOK-006` rule 8.7 declined to provide and reserved to this phase.

The **operator** needs to name a sweep rather than reconstruct one from shell history, and needs the batch to be cheap
enough to re-run rather than precious enough to protect.

## Capability statement

The repository executes a declared sweep of the engine over its configuration axes, derives from each run both the
facts the `run` record states and the behavioural counters it does not, retains one small fact row per run bound by
digest to the discarded stream it came from, and separately computes an outcome classification and a distribution over
those rows — with every threshold in the classifier and none in the retained data, so that a threshold changes without
invalidating a single retained row, and without the engine, its records or its schema version changing at all.

## Boundaries

**Included.**

- A batch driver that executes the engine once per cell of a declared sweep over the axes **decision source ×
  food density × seed**, at a stated tick horizon, and that names its default sweep rather than inventing one per
  invocation.
- Per-run derivation in two parts: the `run` record's own figures, copied without reinterpretation, and the
  behavioural counters the `run` record does not carry, derived by reading the event stream.
- Both an **attempted** and an **effective** count for each behavioural mechanism the record vocabulary lets the two
  be distinguished for, so that an inert mechanism reads as inert.
- Retention of one fact row per run, carrying the sweep coordinates, the engine version, the run record's figures, the
  derived counters, and the **sha256 of the event stream the row was derived from**.
- Discarding the event stream after it is read, on the measured basis that the same coordinates reproduce it byte for
  byte, across seeds and across build profiles.
- A separate classifier that reads retained rows and computes an outcome class per run from stated thresholds, and a
  distribution over any subset of the axes.
- An outcome vocabulary defined as decidable predicates over stated facts, covering at minimum extinction, collapse,
  asymmetric collapse, famine and coexistence, with every class either observed in the measurement or explicitly
  recorded as unobserved.
- Failure behavior at batch scale: a sweep with any incomplete cell is not presented as a distribution.
- Measured evidence that driving the engine from a batch changes no byte of any run's own output.

**Excluded.**

- **Any change to the engine.** No simulation rule, survival value, resource table, density mapping, perception
  radius, regeneration rule, default configuration, exit code, text stream byte, record field, record kind or schema
  version moves. `SPEC-MOK-006` is not amended and stays at schema 3.
- **Any new Cargo package, any third workspace member, any new external dependency in either package.**
  `ARCH-MOK-001`'s prohibition and `SPEC-MOK-004` rule 1 are untouched; `ADR-MOK-008` records why that is possible.
- **Any classification, label, verdict, severity or threshold in a retained row.** Rows carry facts. A row that
  carried a label would be invalidated by revising the label's definition, which is the failure mode this capability
  exists to avoid.
- **Any floating-point value in a retained row.** Integers and closed-vocabulary strings, on `SPEC-MOK-006` rule
  12.4's reasoning: a formatted decimal's bytes vary by platform and a retained row is compared for equality.
- **Any run of the `llm` decision source.** It is live-only, `REQ-MOK-072` forbids it without the owner's written
  authorization naming horizon, seed set and ceiling, and five thousand-tick seeds were measured at roughly $20.55.
  The batch drives the four in-process sources. Publishing a model-backed figure beside them is `WO-MOK-027`'s, under
  its own authorization.
- **Any network access, any credential, any environment read for anything but the operator's own invocation.**
- **Any repair of a defect the instrument finds.** The threat mechanism's inertness is reported and not fixed here.
  Fixing it changes simulation rules and needs its own chain; this capability's value depends on it reporting rather
  than resolving.
- **Any retention of a full event stream in the repository**, and any directory of them. A stream is a temporary file
  during a batch and does not survive it.
- **Any statement about what the distribution ought to show.** No target distribution, no required outcome, no
  passing shape. `INT-MOK-001`'s claim is that outcomes are not predetermined, and a required distribution would
  contradict it.
- **Any structured output from the observer**, whose export stays `SPEC-MOK-003` rule 9.4's text format.

## Outcomes

- A sentence of the form "at density 0.10 the population goes extinct in every seed, and at 0.75 it does not" is
  supported by a retained artifact and re-derivable by a reader in about a minute.
- A threshold changes, the classification changes with it, and every retained row is still valid — because no row
  ever carried the threshold or the label.
- An inert mechanism is visible as inert. The 1,448 threats that fired and the 1 that worked are two columns, not
  one, and no reading of the batch mistakes the first for behaviour.
- The evidence for a 400-run sweep fits in the repository, and the 1.2 GB of streams it was derived from does not
  need to.
- Phase 6's comparison becomes an argument over retained rows rather than a re-run of the experiment, and its
  negative claim — that no outcome threshold governs the result — is checkable, because the thresholds are in one
  named file that the retained rows do not depend on.
- The repository gains the ability to notice a regression in emergent behaviour, which single runs cannot show.

## Candidate requirements

- `REQ-MOK-078` — execute a declared sweep over the engine's configuration axes and retain one fact row per run.
- `REQ-MOK-079` — bind each retained row to the stream it was derived from, and discard the stream.
- `REQ-MOK-080` — report the behavioural counters the run record does not carry, attempted and effective separately.
- `REQ-MOK-081` — classify a run's outcome from stated facts, with every threshold outside every retained artifact.
- `REQ-MOK-082` — state a distribution over the sweep's axes from retained rows alone.
- `REQ-MOK-083` — refuse to present an incomplete sweep as a distribution.

No requirement is needed for the instruments' location, their command-line form, the fact row's field names, the
digest algorithm, the outcome vocabulary's spelling or the default sweep's exact values. `SPEC-MOK-008` fixes all of
them, in the same way `REQ-MOK-024` delegates every layout threshold to `SPEC-MOK-003`. No requirement is needed to
forbid a third package: `ARCH-MOK-001` already does, and `ADR-MOK-008` records that this capability does not ask for
an exception.
