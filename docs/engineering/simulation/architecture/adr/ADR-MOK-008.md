+++
id = "ADR-MOK-008"
type = "adr"
title = "The measuring instrument is a pair of standard-library scripts beside the governance instruments, and the third-package prohibition is not touched"
status = "approved"
owners = ["technical owner"]
created = "2026-08-30"
updated = "2026-08-30"

[relations]
decides = ["ARCH-MOK-001"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-30T16:11:24Z"
decided_by = "technical owner"
reason = "Approved by the repository owner on 2026-08-30, by selecting the presented option, as part of the twelve-artifact Phase 4b chain. The chain converts docs/ROADMAP.md's Phase 4b open question into an approved shape on the strength of the measurement the roadmap reserved that decision to, taken at c90edc9 and recorded in ADR-MOK-008. It carries three disclosed and unrepaired findings: the threat mechanism is inert in 1447 of 1448 firings, the famine predicate is unreached in the swept space with food still standing at extinction, and no retreat event kind exists."
+++

# ADR: The measuring instrument is a pair of standard-library scripts, and the third-package prohibition is not touched

## Status

**Accepted 2026-08-30 by the repository owner acting as accountable technical owner**, by selecting the presented
option in an interactive session. The same act approved `INT-MOK-012`, `CAP-MOK-012`, `REQ-MOK-078` through
`REQ-MOK-083`, `SPEC-MOK-008` and `VER-MOK-019`, and authorized `WO-MOK-033` to begin.

This record is the conversion of `docs/ROADMAP.md`'s Phase 4b open question into a decision, and that document reserved
the answer to a measurement rather than to an argument. The measurement was taken first and is recorded below; the
acceptance rests on it. Approving it is not approving silence about the three findings `WO-MOK-033` discloses and does
not repair.

## Context

`INT-MOK-012` and `CAP-MOK-012` require a batch driver, a per-run fact row, an outcome classifier and a distribution.
Nothing of the kind exists. The question is where the code lives, and it is a genuine architectural question rather
than a preference, because `ARCH-MOK-001` prohibits the obvious answer.

The prohibition is specific. Under *Prohibited patterns*: "Separate Cargo packages, workspaces, or services without an
approved requirement. The library and binary targets of the single engine package are not separate crates in this
sense. `REQ-MOK-026` is the approved requirement for exactly one further package, the terminal observer; it authorizes
no service, no network boundary, no separate release artifact, and no third package." `SPEC-MOK-004` rule 1 states the
same structurally: "No third package directory, no nested workspace, and no directory holding the sources of more than
one package." `ADR-MOK-003` decision 1 fixed the workspace as "exactly **two packages** … No third package, no
service, no separate release artifact", and `ADR-MOK-003`'s 2026 revision records that "Everything else this ADR
decides stands, including the two-package split with no third package and no service".

`ADR-MOK-005` already met this boundary once, for a related purpose. Its *Option 1.4* was "A third package that
converts a retained text stream into records", and its recorded consequence was that "`ARCH-MOK-001` prohibits a third
package without an approved requirement". That option was declined and the record stream was built inside the engine
instead, because the engine already held the values.

This time the engine does not hold the thing needed. A distribution is not a property of a run, and a classification is
forbidden to the engine outright: `SPEC-MOK-006` rule 8.7 states that no record of any kind carries "an outcome
classification, label, category, verdict, severity or interpretation", and gives the reason as "Classification is Phase
4b's, and a threshold must be revisable without invalidating a retained capture."

`docs/ROADMAP.md` left the shape open on purpose: "If a shell loop over the existing binary plus a script under
`scripts/` — where the Python instruments and their tests already live — produces the distribution evidence Phase 6
needs, then 4b is a runbook and a verification contract, and `ARCH-MOK-001`'s third-package prohibition is never
touched. If it does not, the third package is argued on that finding. Deciding it now would be deciding it without the
measurement."

## The measurement

Taken 2026-08-30 at commit `c90edc9`, against the release binary, on Windows.

| Quantity | Measured |
|---|---|
| 20 runs of 1,000 ticks driven by a shell loop | **0.94 s** wall total, 0.047 s per run |
| reading 20 runs' header and run records | **0.005 s** |
| scanning all 20 full event streams, 60,109,145 bytes | **0.813 s**, 0.041 s per run |
| one 1,000-tick `social` event stream | 2,723,014 bytes |
| the `run` record inside it | **1,040 bytes** |
| re-run at the same seed, event stream and standard output | **byte-identical** |
| the same cell from a debug build and a release build | **byte-identical**, both `ed1da4b6…` |
| extrapolated 400-cell sweep | ~19 s to run, ~16 s to scan, ~500 KB retained, ~1.2 GB if streams were kept |

Two facts about the engine's interface were measured because they constrain any design. `--events-path -` is refused —
"invalid `--events-path` value: `-`; expected a file path, and no path denotes a standard stream" — so no design can
pipe records and every design writes a file per run. An existing path is overwritten cleanly, so one reusable temporary
path suffices and peak disk during a batch is one stream rather than the whole sweep.

Two facts about the data were measured because they decide what the instrument must compute. The `run` record carries no
`attack_resolved`, `threat_resolved` or `surrender_resolved` count, so behavioural figures require the event stream.
And of 1,448 `threat_resolved` events across 35 runs, exactly **one** had `increase > 0`; the other 1,447 targeted a
creature already at `fear: 100`. A design that counted only attempts would have reported a wholly inert mechanism as the
most active behaviour in the batch.

Finally, a 130-line throwaway script over that loop produced a distribution that discriminates: `baseline`, the random
control, went extinct in 5 of 5 seeds at ticks 119 to 163, while the three rule-following sources all reached the
1,000-tick limit with 6 to 11 of 12 alive, and `social` was the only source producing any conflict. That is the shape
Phase 6 asks for, produced without a package.

## Decision

**The measuring instrument is a pair of standard-library Python scripts under `scripts/`, driven by a runbook, and no
third package is created.**

1. A **batch driver** executes the sweep, derives each run's fact row, hashes the stream, discards it, and retains the
   rows.
2. A **classifier and reporter** reads retained rows, computes the outcome class and states the distribution. Every
   threshold and every class definition lives here and in no retained artifact.
3. Both are Python, standard library only, and each ships with a `test_<name>.py` beside it, which is the convention
   every existing instrument in `scripts/` already follows.
4. `ARCH-MOK-001`'s prohibited-patterns list, `SPEC-MOK-004` rule 1, `ADR-MOK-003` decision 1, both `Cargo.toml`
   files, `Cargo.lock` and `SPEC-MOK-006` are **untouched**. No requirement is added to authorize a package, because
   none is needed.

The engineering owner took this decision on 2026-08-30, by selecting the presented option, with each option's cost
measured and stated.

## Options considered

### Option 1: a third Cargo package

A third workspace member holding the batch driver and the classifier.

Consequences. It requires a new approved requirement, because `ARCH-MOK-001` admits a package only "without an approved
requirement" being the prohibition's condition and `REQ-MOK-026` is exhausted on the observer. It requires amending
`ARCH-MOK-001`'s prohibited-patterns list, `SPEC-MOK-004` rule 1 and `ADR-MOK-003` decision 1, and `SPEC-MOK-004` rule
1's census. The measurement supports none of it: 0.047 s per run leaves nothing for a compiled language to improve.
And it works against `SPEC-MOK-006` rule 8.7, because thresholds compiled into a released package are not "revisable
without invalidating a retained capture" in any useful sense — revising one would be a product change, and the release
that carried the old threshold would remain the authority for figures published under it.

**Declined.** It costs four amendments and a requirement, and buys a slower path to a worse property.

### Option 2: a batch mode on the engine's binary target

A `--batch` or `--sweep` option on `Mokiterions`, sweeping and classifying in-process.

Consequences. It creates no package, so the prohibition is untouched, and the engine's binary target is already the
place where "the option is parsed, the path resolved, the file created and truncated" happens, so filesystem work there
is in order. But it puts outcome classification inside the product, which rule 8.7 forbids in records and which this
option would place one layer above them while making the thresholds part of a release. It also contradicts
`ARCH-MOK-001`'s quality attribute of "one engine package with one library target and one **thin** binary target": the
binary would gain a sweep enumerator, a subprocess or re-entrant execution model, a digest implementation and a
classifier. And it makes the instrument unable to measure a *different* engine version than the one it ships in, which
is precisely what a regression check needs.

**Declined.** It avoids the prohibition and pays for it in the product's shape.

### Option 3: a shell loop and no script at all

A documented runbook of shell commands, with analysis done ad hoc.

Consequences. Free, and it is what this repository has always done — and `INT-MOK-009` already named that method as
what must stop, because "it puts a parser the repository wrote between the engine's facts and the conclusion drawn from
them" and "does not scale to Phase 6, where the result *is* a distribution". A loop with no retained instrument cannot
satisfy `REQ-MOK-079`'s digest binding, `REQ-MOK-081`'s revisability or `REQ-MOK-083`'s per-cell incompleteness report,
and nothing about it is testable.

**Declined.** It is the status quo the intent exists to end.

### Option 4: scripts under `scripts/`, with the run record's conflict counters added to the engine

Option 4 as decided, plus an amendment to `SPEC-MOK-006` adding cumulative `attack`, `threat` and `surrender` counters
to the `run` record — which would make the behavioural figures self-checking under rule 8.6 and remove the need to read
a 3 MB stream at all.

Consequences. Genuinely attractive: rule 8.6's "every cumulative figure equals the number of corresponding event
records in the same stream" would then cover the behavioural counts, and a batch would read 1,040 bytes per run instead
of 3 MB. But rule 10.2 forces the schema from **3 to 4**, it is an engine change, rule 7.8's stated reason must move,
and every retained record-stream digest in the repository moves again — the 2026-08-23 ratification already moved forty
of them twice, and that consequence is recorded in `SPEC-MOK-006` as "not repairable by argument". Against a measured
scan cost of 0.041 s per run, the trade is a schema increment and a product change to save 40 milliseconds.

**Declined for now, and explicitly not foreclosed.** The engineering owner considered it on 2026-08-30 and chose to
scan the stream. If the counters are later added, this instrument becomes the oracle for that change rather than being
replaced by it, because the pre-change figures are already captured.

## Consequences

**Accepted.**

- Python becomes load-bearing for **published simulation results**, where until now it was load-bearing only for
  governance. `scripts/` gains its first instrument that is not a governance or CI check. This is a real widening of
  what the repository depends on Python for, and it is the price of every alternative being worse.
- The behavioural counters require reading each run's whole event stream, at a measured 0.041 s per thousand-tick run.
  The stream is read once, in the same pass that hashes it, and then discarded.
- The retained rows are bound to streams that no longer exist. That is safe only because determinism was measured in
  both directions, including across build profiles, and `REQ-MOK-079` makes a non-reproducing digest a reported defect
  rather than something to reconcile.
- The instrument cannot measure the `llm` source. It is live-only, `REQ-MOK-072` forbids running it without written
  authorization, and five thousand-tick seeds were measured at roughly $20.55. Publishing a model-backed figure beside
  the others stays `WO-MOK-027`'s under its own authorization.

**Not accepted, and stated so a later reader does not assume otherwise.**

- No package, workspace member, service, network boundary or release artifact is created.
- `ARCH-MOK-001`, `ARCH-MOK-002`, `ADR-MOK-003`, `SPEC-MOK-004` and `SPEC-MOK-006` are not amended by this decision.
- The engine's schema version stays at **3**. No record kind, field or value domain moves, so no retained digest
  anywhere in the repository moves.
- No Cargo dependency is added to either package. The instruments use the Python standard library only, so
  `SPEC-MOK-005` rule 15's declared-set comparison and `scripts/check_declared_dependencies.py` — which compare Cargo
  graphs — have nothing new to compare, and both packages' declared sets are unchanged.
- Neither `Cargo.toml`, `Cargo.lock`, nor any file under `mokiterions-core/src/` or `mokiterions-tui/src/` is touched.

## Required amendments

**None.** That is the substance of this decision rather than an absence in it. Every architectural constraint this
capability approaches — the third-package prohibition, the two-package census, the binary target's thinness, the record
schema, the declared dependency sets — is satisfied as written, and `docs/ROADMAP.md`'s conditional is resolved on its
first branch.

One documentation change is owed and is not an amendment: `docs/ROADMAP.md`'s Phase 4b section states the open question
this record closes, and its phrasing "distribution across seeds" and "stated as a table over 4a's `run` records" are
both contradicted by the measurement — seed is not the axis that moves the outcome, and the `run` record does not carry
the behavioural facts. The roadmap authorizes nothing and is outside every work order, so it is reconciled under its own
change.

## Verification

`VER-MOK-019` covers this decision's checkable claims: that no third package or workspace member exists, that no file
under either package's `src/` or either `Cargo.toml` is modified, that the schema version is still 3, that both declared
dependency sets are unchanged, and that the instruments import only the standard library.
