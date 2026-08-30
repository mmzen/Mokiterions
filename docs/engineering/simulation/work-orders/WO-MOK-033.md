+++
id = "WO-MOK-033"
type = "work_order"
title = "Build the measuring instrument: a batch sweep, bound fact rows, and a revisable outcome classification"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-30"
updated = "2026-08-30"

[assurance]
commit_bound_verification = "required"
rationale = "The change is executable: two instruments and two test files under `scripts/`, and every figure this repository publishes about the simulation from here on is produced by them. `VER-MOK-019` asserts satisfaction through automated cases, and three of its claims are only meaningful when bound to a commit -- that a retained row's digest reproduces the stream that was destroyed, that editing a threshold changes no retained row, and that no file under either package's `src/` or either `Cargo.toml` was touched. `ADR-MOK-008`'s substance is that no amendment is required, and that claim is falsifiable only as a check at a commit. A later reader who finds a distribution published under this work order needs a record naming the commit at which the instrument that produced it existed."
decided_by = "engineering owner"

[execution_scope]
paths = [
  "docs/engineering/simulation/architecture/adr/ADR-MOK-008.md",
  "docs/engineering/simulation/capabilities/CAP-MOK-012.md",
  "docs/engineering/simulation/evidence/WO-MOK-033/",
  "docs/engineering/simulation/intent/INT-MOK-012.md",
  "docs/engineering/simulation/requirements/REQ-MOK-078.md",
  "docs/engineering/simulation/requirements/REQ-MOK-079.md",
  "docs/engineering/simulation/requirements/REQ-MOK-080.md",
  "docs/engineering/simulation/requirements/REQ-MOK-081.md",
  "docs/engineering/simulation/requirements/REQ-MOK-082.md",
  "docs/engineering/simulation/requirements/REQ-MOK-083.md",
  "docs/engineering/simulation/specifications/SPEC-MOK-008.md",
  "docs/engineering/simulation/verification/VER-MOK-019.md",
  "docs/engineering/simulation/verification-records/",
  "docs/engineering/simulation/work-orders/WO-MOK-033.md",
  "scripts/classify_simulation_runs.py",
  "scripts/run_simulation_batch.py",
  "scripts/test_classify_simulation_runs.py",
  "scripts/test_run_simulation_batch.py",
]

[relations]
implements = [
  "REQ-MOK-078",
  "REQ-MOK-079",
  "REQ-MOK-080",
  "REQ-MOK-081",
  "REQ-MOK-082",
  "REQ-MOK-083",
]
specifications = ["SPEC-MOK-008"]
verification = ["VER-MOK-019"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-30T16:11:24Z"
decided_by = "engineering owner"
reason = "Approved for execution by the repository owner acting as accountable engineering owner on 2026-08-30, by selecting the presented option, together with the eleven definitions this packet carries. Assurance is required and not discretionary: the change is executable, and three of VER-MOK-019's claims are only meaningful bound to a commit - that a retained row's digest reproduces a stream that was destroyed, that editing a threshold changes no retained row, and that no file under either package's src or either Cargo.toml was touched. ADR-MOK-008's substance is that no amendment is required, which is falsifiable only as a check at a commit."

[[lifecycle_events]]
from = "approved"
to = "in_progress"
decided_at = "2026-08-30T16:18:11Z"
decided_by = "engineering owner"
reason = "Execution started 2026-08-30 under the owner's approval of the chain in the same interactive session, by selecting the presented option. Scope is the two instruments under scripts/, their two test files, VER-MOK-019's cases and the retained evidence packet. No engine file, Cargo.toml or Cargo.lock is in scope."

[[lifecycle_events]]
from = "in_progress"
to = "implemented"
decided_at = "2026-08-30T18:18:07Z"
decided_by = "engineering-owner"
reason = "Marked implemented by the repository owner acting as accountable engineering owner on 2026-08-30, by selecting the presented option to approve and execute the chain through to preparing and verifying VREC-MOK-027 -- an endpoint unreachable without this transition. Both instruments and both suites are built (143 tests, all passing), the 400-cell default sweep is retained at 278,734 bytes with the same digest from four separate productions including one at --jobs 4, and 84 of VER-MOK-019's cases pass across six phases. The evidence packet is retained under docs/engineering/simulation/evidence/WO-MOK-033/ with a SHA-256 manifest over 147 files. Four defects were found and repaired with tests that fail on the pre-fix code; nine gaps in SPEC-MOK-008 and one wrong citation in VER-MOK-019 are disclosed rather than amended, per condition 6. VER-MOK-019's eight manual assessments are reserved to the product, technical and assurance owners and are NOT recorded; VREC-MOK-027 discloses that gap rather than closing it."
+++

# Work Order: Build the measuring instrument

## Lifecycle

`draft` to `approved` authorizes execution and, in the same act, approves the eleven governance artifacts this packet
carries. `approved` to `in_progress` starts it. `in_progress` to `implemented` requires both instruments, both test
files, every `VER-MOK-019` case executed, and the retained evidence. Assurance is `required`, so this work order does
not stop at `implemented`: `VREC-MOK-027` covers it.

## Objective

Build the two instruments `SPEC-MOK-008` specifies — `scripts/run_simulation_batch.py` and
`scripts/classify_simulation_runs.py`, each with its test file — and produce the default sweep's distribution as
evidence. Change no engine file, add no package, add no dependency, and leave `SPEC-MOK-006`'s schema at 3.

## Why this exists, and what it ends

Every quantitative claim this repository has made about the simulation was produced by writing a bespoke parser against a
human-facing stream, or by writing a throwaway probe against engine internals. `INT-MOK-009` named that method as one
that must stop, because it puts a parser the repository wrote between the engine's facts and the conclusion drawn from
them, and because it does not scale to Phase 6, where the result *is* a distribution.

Phase 4a built the stream. This work order builds the thing that reads it at scale. After it, a claim about what usually
happens in this simulation is a re-runnable command over retained rows rather than a figure someone once measured.

## The chain this packet carries

Eleven artifacts, drafted with this work order and approved in the same act as it:

| Artifact | What it fixes |
|---|---|
| `INT-MOK-012` | the intent: state what usually happens, instead of what happened once |
| `CAP-MOK-012` | the capability: sweep, retain bound facts, classify revisably |
| `REQ-MOK-078` | execute a declared sweep and retain one fact row per run |
| `REQ-MOK-079` | bind each row to its stream by digest, then discard the stream |
| `REQ-MOK-080` | count the behavioural events the `run` record omits, attempted and effective separately |
| `REQ-MOK-081` | classify from stated facts, with every threshold outside every retained artifact |
| `REQ-MOK-082` | state a distribution over the sweep's axes from retained rows alone |
| `REQ-MOK-083` | refuse to present an incomplete sweep as a distribution |
| `ADR-MOK-008` | where the instrument lives, and that the third-package prohibition is not touched |
| `SPEC-MOK-008` | every command line, field, predicate, threshold, cross-check and exit status |
| `VER-MOK-019` | the verification contract, including the cases that make `ADR-MOK-008` falsifiable |

## The measurement this packet rests on

`docs/ROADMAP.md` reserved Phase 4b's shape to a measurement rather than an argument: "If a shell loop over the existing
binary plus a script under `scripts/` … produces the distribution evidence Phase 6 needs, then 4b is a runbook and a
verification contract, and `ARCH-MOK-001`'s third-package prohibition is never touched. If it does not, the third package
is argued on that finding. Deciding it now would be deciding it without the measurement."

The measurement was taken on 2026-08-30 at `c90edc9` and is recorded in `ADR-MOK-008`. It resolved the conditional on its
first branch: 400 cells cost about 19 s to run and 16 s to scan, a shell loop plus a throwaway script already produced a
distribution that discriminates between the four decision sources, and nothing a compiled package could add is visible at
0.047 s per run. **No amendment to any architecture artifact is required**, which is `ADR-MOK-008`'s substance rather
than an absence in it.

## Three findings this work order carries rather than repairs

Recorded here so that approving this work order is not read as approving silence about them.

1. **The threat mechanism is inert.** Across 35 runs the streams carried 1,448 `threat_resolved` events of which
   exactly **one** had `increase > 0`; the other 1,447 targeted a creature already at `fear: 100`. Fear is not
   universally saturated — at tick 1,000 the mean is 39 and the maximum 95 — so threat *targeting* is selecting
   already-saturated creatures. This is a defect in the simulation. The repository owner decided on 2026-08-30 that this
   work order **discloses** it and a later chain repairs it, so that the *before* figure exists as a measurement a repair
   cannot manufacture after the fact. `SPEC-MOK-008` rule 9.6 records it and rule 9's attempted-and-effective split
   exists because of it.
2. **Famine is unreachable in the swept space, and not for the intuitive reason.**
   `regeneration_skipped.depleted` was 0 in all 35 runs. At density `0.10`, where the population goes extinct in every
   seed, both territories end with **8 units of food still standing** and `depleted` false. So extinction at low density
   is not resource exhaustion; the population fails to reach food rather than to have it. `SPEC-MOK-008` rule 16.8
   retains the class, states that measurement does not support its predicate, and refuses to invent one.
3. **No `retreat_resolved` event kind exists.** `docs/ROADMAP.md`'s Phase 6 names retreat; the event vocabulary has
   fifteen kinds and none for it. `SPEC-MOK-008` rule 9.7 forbids synthesising a retreat figure from another kind, and
   `VER-MOK-019` case T8 records the gap.

## Steps

1. Write `scripts/run_simulation_batch.py` to `SPEC-MOK-008` rules 2 to 15. Standard library only.
2. Write `scripts/test_run_simulation_batch.py`, covering rule 20.1's enumerated cases, on fixture streams.
3. Write `scripts/classify_simulation_runs.py` to rules 16 to 19. Standard library only.
4. Write `scripts/test_classify_simulation_runs.py`, covering rule 20.1's classifier cases.
5. Execute `VER-MOK-019`'s cases. B1 and B7 require the built binary; the rest are fixture-based.
6. Run the default sweep. Retain its output, its distribution in both formats, and the measured cost beside
   `ADR-MOK-008`'s predictions.
7. Produce the three findings' figures over the full 400-cell sweep, in particular the threat attempted-and-effective
   split, retained as the *before* measurement for the later repair chain.
8. Run `scripts/check_declared_dependencies.py`, `cargo clippy --workspace --all-targets --all-features --locked --
   -D warnings`, `cargo fmt --check`, and `cargo test --locked --workspace`. The last three are evidence that the
   product is genuinely untouched.
9. Retain the evidence packet under `docs/engineering/simulation/evidence/WO-MOK-033/` with a SHA-256 manifest.

## Out of scope

- **Any change to either package.** No file under `mokiterions-core/src/` or `mokiterions-tui/src/`, no `Cargo.toml`, no
  `Cargo.lock`. `VER-MOK-019` case T2 checks it.
- **Any amendment to `SPEC-MOK-006`**, including the run-record conflict counters `ADR-MOK-008` option 4 considered.
  The schema stays 3, so no retained record-stream digest in the repository moves.
- **Repairing the threat mechanism.** Its own chain, by the owner's 2026-08-30 decision.
- **Redefining the famine predicate.** A semantics decision for the product owner over a measurement that does not exist
  yet.
- **`docs/ROADMAP.md`.** The roadmap is outside every work order. Its Phase 4b section states the open question
  `ADR-MOK-008` closes, and its phrasings "distribution across seeds" and "stated as a table over 4a's `run` records" are
  both contradicted by the measurement — seed is not the axis that moves the outcome, and the `run` record carries no
  behavioural counter. That reconciliation is owed under its own `docs(roadmap):` change, which carries no work-order
  trailer.
- **The `llm` decision source.** Live-only, refused before any cell executes, and `REQ-MOK-072` governs it. Five
  thousand-tick seeds were measured at roughly $20.55. Publishing a model-backed figure beside the others stays
  `WO-MOK-027`'s under its own authorization.
- **Any live run, any provider call, any credential.** No case in `VER-MOK-019` permits one.
- **Running a sweep in continuous integration.** Neither required nor forbidden; `SPEC-MOK-008` leaves it open.

## Stop-and-escalate conditions

1. A rule 10 cross-check fails on a real engine run. That is a defect the stream itself reveals, and deciding which of
   the engine's two accounts to believe is not this work order's to make.
2. A digest fails to reproduce under B7. Either the engine changed without its version changing, or determinism has
   broken. Neither is repairable here and neither may be worked around by updating the digest.
3. A retained row cannot be produced without carrying a classification, a threshold or a floating-point value. That
   would mean `SPEC-MOK-008` and `SPEC-MOK-006` rule 8.7 are in conflict, which is an amendment and not an
   implementation decision.
4. The instrument cannot be written without a non-standard-library import. That falsifies `ADR-MOK-008`'s premise and
   the decision must be re-taken, not worked around.
5. Any figure this work order would publish requires a threshold the owner has not decided. `SPEC-MOK-008` rule 16.5's
   are decided; a new one is not.
6. An approved artifact appears to need amending. `WO-MOK-026` stop condition 6 applies: an implementation agent does
   not amend an approved artifact on its own judgement.
7. Any credential or token appears in any produced byte. Stop, do not commit the evidence, and escalate. Committed
   evidence containing a credential cannot be corrected.

## The numbering survey

Recorded because `WO-MOK-016` recorded the same lesson and this packet nearly repeated the mistake it warns about.

A survey by `grep` alone reports a maximum verification identifier of `VER-MOK-036`, and an earlier draft of this packet
numbered its contract that. Both `VER-MOK-034` and `VER-MOK-035` are citations rather than artifacts: `034` is a
typographic citation of `REQ-MOK-034` inside `evidence/WO-MOK-010/completion-summary.md`, and `035` appears only inside
`WO-MOK-016`'s own account of this same trap. A survey of declared `id` fields across **every ref**, not only the working
tree, gives the real state: the verification family is `001` to `018` with deliberate gaps at `009` and `015`, both of
which `docs/RELEASE_RUNBOOK.md:263` explains and neither of which this chain fills. The contract is therefore
`VER-MOK-019`.

The same survey across every ref confirms `INT-MOK-012`, `CAP-MOK-012`, `REQ-MOK-078` through `REQ-MOK-083`,
`ADR-MOK-008`, `SPEC-MOK-008`, `WO-MOK-033` and `VREC-MOK-027` are free. There is no work-order-to-verification pairing
convention to honour: the ten most recent work orders name existing contracts, seven of them `VER-MOK-018`.

**One consequence for a deferred packet.** Taking `WO-MOK-033` here means the rule 14 fifth-price work order, which
`WO-MOK-032` deferred and which is not yet drafted, becomes `WO-MOK-034` when it is written.

## Evidence

Retained under `docs/engineering/simulation/evidence/WO-MOK-033/`, per `VER-MOK-019`'s *Evidence retention*, with a
SHA-256 manifest covering every file, written with the line endings the manifest hashes.

## Decision record

The repository owner, acting as product owner, technical owner and engineering owner, took the following on 2026-08-30 in
an interactive session, each by selecting the presented option with the alternatives' costs measured. They are design
decisions, not approvals.

| Decision | Outcome | Role |
|---|---|---|
| Where the instrument lives | A pair of standard-library Python scripts under `scripts/` with a runbook. No third package, no batch mode on the engine's binary, and `ARCH-MOK-001` untouched | technical owner |
| What a run retains | The run record's figures plus derived event counts and a `stream_sha256`, with the stream scanned and then discarded. About 500 KB for 400 runs against about 1.2 GB if streams were kept | technical owner |
| The sweep's axes | Decision source, food density and seed, at one horizon — 400 cells. Density was measured to move the outcome where seed moves only the figures | product owner |
| The threat defect | This work order discloses it and measures it; a later chain repairs it. The *before* figure exists already, which a later repair cannot manufacture | product owner |
| Whether the run record gains conflict counters | Declined for now and explicitly not foreclosed. It would save a measured 40 ms per run and cost a schema increment from 3 to 4, an engine change, and every retained stream digest in the repository | technical owner |
