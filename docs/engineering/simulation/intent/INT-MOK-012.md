+++
id = "INT-MOK-012"
type = "intent"
title = "State what usually happens, instead of what happened once"
status = "approved"
owners = ["product owner"]
created = "2026-08-30"
updated = "2026-08-30"

[relations]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-30T16:11:24Z"
decided_by = "product owner"
reason = "Approved by the repository owner on 2026-08-30, by selecting the presented option, as part of the twelve-artifact Phase 4b chain. The chain converts docs/ROADMAP.md's Phase 4b open question into an approved shape on the strength of the measurement the roadmap reserved that decision to, taken at c90edc9 and recorded in ADR-MOK-008. It carries three disclosed and unrepaired findings: the threat mechanism is inert in 1447 of 1448 firings, the famine predicate is unreached in the swept space with food still standing at extinction, and no retreat event kind exists."
+++

# Intent: State what usually happens, instead of what happened once

## Problem

This repository can run the simulation and it can record a run. It cannot state a result.

`INT-MOK-009` closed the gap that mattered first: a run now writes machine-readable records, so a program reads a
run's facts without knowing the engine's rules. `CAP-MOK-009` deliberately stopped there, and its *Excluded* list
names what it stopped short of in four items — "batch execution across seeds, run persistence beyond the one stream,
a run index, and any directory convention" — and in one more, "any conflict, combat, threat, retreat or surrender
metric".

The consequence is that the project's central claim is still unsupported. `INT-MOK-001` requires that twelve
Mokiterions "produce observable outcomes that are not predetermined by the simulation engine". That is a statement
about a **distribution**, and a distribution is not a run. One run at one seed and one density is an anecdote, and
this repository currently produces anecdotes at very high quality.

Three specific things are missing, and each is missing for a different reason.

- **Nothing executes many runs.** There is no batch driver. Every quantitative claim in this repository's history —
  the density curve, the fifty-seed trait distribution, the ninety-run projection check — was produced by a loop
  written for the occasion and thrown away, or by a throwaway Rust probe retained as evidence. `INT-MOK-009` named
  that method as unscalable and did not replace it.
- **Nothing states an outcome.** `SPEC-MOK-006` rule 8.7 forbids the engine to state a classification, in any record
  kind, and says why: "Classification is Phase 4b's, and a threshold must be revisable without invalidating a
  retained capture." The refusal was correct and it left the classification unowned. No artifact in this repository
  defines what *extinction*, *famine*, *collapse* or *coexistence* mean as a decidable predicate over a run's facts.
- **The facts a behavioural claim needs are not where a distribution can cheaply reach them.** The `run` record
  carries survivors, deaths, crossings, consumption and regeneration. It carries no attack, threat or surrender
  count, by `SPEC-MOK-006` rule 7.8's design. So a claim about aggression or surrender is reachable only by reading a
  whole event stream, which is about 3 MB per thousand-tick run and cannot be retained at the scale a distribution
  needs.

There is a fourth problem, and it is the one that makes this intent urgent rather than tidy. **Without a distribution
the repository cannot tell a working mechanism from an inert one.** A measurement taken on 2026-08-30 across 35 runs
found 1,448 `threat_resolved` events of which exactly **one** had any effect, the other 1,447 landing on a target
whose fear was already at its ceiling of 100. Every one of those runs terminated normally, every stream validated,
and every individual run looked healthy. The defect is invisible in one run and obvious in thirty-five. Phase 6 is
asked to report that "the conditions permit emergence"; a repository that cannot count a mechanism's effective
firings cannot support that sentence, and cannot notice when it stops being true.

## Desired outcomes

- A claim of the form *"under these conditions, this usually happens"* is producible, re-derivable and checkable by a
  reader who has the repository and no special knowledge of it.
- The configuration space is swept along the axes that actually move the outcome, established by measurement rather
  than by assumption, and the sweep that produced any published figure is named rather than described.
- An outcome classification exists, is written down as a predicate over stated facts, and can be **changed without
  invalidating anything already retained** — which is the property `SPEC-MOK-006` rule 8.7 reserved this phase to
  provide.
- A behavioural figure distinguishes a mechanism that fired from a mechanism that had an effect, so an inert
  mechanism reads as inert instead of as active.
- Retaining the evidence for a distribution costs kilobytes rather than gigabytes, and every retained row remains
  bound to the run it came from, so a figure is re-derivable rather than merely believable.
- No new package, no new Cargo dependency, and no change to the engine, the records it writes, or the schema version.
  The measuring instrument does not modify what it measures.

## Actors and stakeholders

- The **product owner** bears `INT-MOK-001`'s claim about non-predetermined outcomes and currently has no evidence of
  the right shape to support it. This is the artifact that changes that.
- The **assurance owner** needs a figure that is checkable without re-running an expensive thing. A retained row bound
  to its source by digest is checkable; a number in a prose paragraph is not.
- The **technical owner** needs the classification's thresholds outside anything that a release freezes, so revising a
  threshold is not a product change, and needs the instrument kept out of the engine's authority boundary entirely.
- The **operator** running experiments gets a named sweep instead of a shell history, and is not the reason this
  exists.
- The **model-backed source** is a stakeholder by exclusion. It is live-only, `REQ-MOK-072` forbids running it without
  the owner's written authorization, and five thousand-tick seeds were measured at roughly $20.55. It is out of this
  intent's reach by cost, and enters distribution work only through separately authorized runs.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Sweeps executable by name rather than by a loop written for the occasion | 0 | 1 declared default, and any operator-stated sweep | Each batch |
| Runs in the declared default sweep | 1 | 400 | Each batch |
| Wall time for the declared default sweep | n/a | under 120 s | Each batch |
| Retained bytes per run of a sweep | ~3,000,000 | under 1,000 | Each batch |
| Retained rows that cannot be re-derived from the stated configuration | n/a | 0 | Each batch |
| Outcome classes defined as a decidable predicate over stated facts | 0 | all of them | The classifier |
| Retained rows carrying a classification, a label or a threshold | n/a | 0 | Each batch |
| Behavioural mechanisms whose effective firings are counted separately from their attempted firings | 0 | all that the record vocabulary distinguishes | Each batch |
| Bytes by which a run's own output differs because a batch drove it | n/a | 0 | Each run of each batch |
| New Cargo packages, new dependencies, changed record schema version | n/a | 0, 0, 0 | The whole change |

## Principles

1. **The instrument is not the engine, and it is not in the engine.** Nothing here changes a simulation rule, a
   record, a schema version or an exit code. The engine is driven, not modified. A measurement that required changing
   the thing measured would not be a measurement.
2. **The axes are established by measurement, not by intuition.** The roadmap's own phrasing for this phase says
   "distribution across seeds"; measurement on 2026-08-30 showed seed barely moves the outcome and density moves it
   entirely, from extinction in 3 of 3 runs at density 0.10 to 7 to 9 survivors of 12 at 0.75. An axis is declared
   because it was shown to matter.
3. **No retained artifact carries a classification.** Facts are retained; labels are computed on read. This is
   `SPEC-MOK-006` rule 8.7's requirement discharged rather than restated, and it is what makes a threshold revisable.
4. **A retained figure is bound to its source or it is not retained.** Each row carries the sweep coordinates, the
   engine version and the digest of the stream it was derived from. The stream itself is discarded, and byte-identity
   across seeds and across build profiles — measured, not assumed — is what makes that safe.
5. **An attempted action and an effective action are different facts.** Counting only attempts made a wholly inert
   threat mechanism read as the most active behaviour in the batch. Both counts are stated; neither is derived from
   the other.
6. **A partial batch is not a distribution.** A sweep that could not complete every cell says so and does not present
   the cells it managed as a result. This is `REQ-MOK-046`'s principle at batch scale.
7. **The instrument may find defects, and reporting them is its purpose rather than a side effect.** The first
   measurement taken under this intent found one. An instrument tuned to produce agreeable distributions would be
   worthless.

## Constraints and assumptions

- `ARCH-MOK-001` prohibits "Separate Cargo packages, workspaces, or services without an approved requirement" and
  records that `REQ-MOK-026` "authorizes no service, no network boundary, no separate release artifact, and no third
  package". `SPEC-MOK-004` rule 1 states the same as "No third package directory, no nested workspace". This intent
  is satisfiable without touching either, and `ADR-MOK-008` records that decision against its alternatives.
- `SPEC-MOK-006` is not amended. The schema version stays at 3, no record kind is added, and no field is added to any
  record.
- The engine refuses `--events-path -` with "expected a file path, and no path denotes a standard stream", so a batch
  writes a file per run. It overwrites an existing path cleanly, so one reusable path suffices and peak disk is one
  stream.
- The four in-process decision sources are in reach; `llm` is not, for the reason given above.
- `docs/ROADMAP.md` authorizes nothing. It states this phase's open question and reserves the answer to measurement;
  this intent is the conversion of that phase into an artifact chain, and the measurement is what it rests on.
