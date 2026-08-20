# Phase 4 Proposal — Analytical Observability

> **Authority note.** This document is repository-owned functional planning, in the same class as
> `docs/mokiterions/ROADMAP.md`. It is **not** approved product intent, a requirement, an architecture decision,
> work authorization, a verification contract, or release authority. Only formal artifacts under
> `docs/engineering/` carry that authority, per `ENGINEERING_HARNESS.md`. Nothing here authorizes implementation.
> Every identifier proposed below is a proposal for an identifier, not a claim on one.

- **Created:** 2026-08-20
- **Updated:** 2026-08-20, with the owner's eleven decisions recorded and the drafted packet's departures from this
  proposal stated
- **Proposes:** the artifact packet for `ROADMAP.md` Phase 4, and one scope reduction the owner must decide
- **Audience:** product owner, technical owner, assurance owner
- **Base:** written against `master` at `dec1b95`. The packet drafted from it is based on `master` at `ff3a155`, which
  `origin/master` reached after this proposal was written. See *Decisions recorded* below.
- **Status:** the owner took all six open decisions, plus five more, on 2026-08-20. The packet exists on
  `feature/phase-4a-definition` as `draft` artifacts. Nothing is approved.

## Recommendation in one paragraph

Split Phase 4 at the boundary between **measurement** and **interpretation**, and authorize only the first half now.
Phase 4a gives the engine a structured record stream — one JSON record per authoritative event, plus per-tick world
metrics the event stream does not already state — which is additive, deterministic, integer-only, and provably a
projection of the existing text stream. Phase 4b — multi-seed batch execution, run persistence, and outcome
classification — is left unauthorized, because all three of its parts either collide with approved architecture or
encode a judgment that does not belong in an authoritative stream, and because 4a is the thing that makes 4b cheap.
The split is recommended on evidence, not on caution: 4a needs no new package, no new dependency, and no new
authority, while 4b needs at least one of each and cannot be specified honestly until 4a's records exist.

## What the roadmap asked for, and what this proposes

| Phase 4 *In scope* | Proposed | Why |
|---|---|---|
| Structured event stream (for example JSONL) alongside the existing text stream | **Phase 4a.** One JSONL record per emitted text line, in the same order, carrying the same facts | This is the whole of Phase 4's product value. Everything else in the list is a consumer of it |
| World-level metrics | **Phase 4a, narrowed.** Only facts the engine holds and the event stream does not already state, and **only integers** | Means and averages are excluded — see *No floats* below. Conflict frequency is excluded because the engine has no conflict — see *No inert fields* |
| Multi-seed batch runner and run persistence | **Phase 4b. Not proposed for authorization now** | A batch runner has nowhere to live. `ARCH-MOK-001` prohibits a third package, and `REQ-MOK-026` "authorizes no service, no network boundary, no separate release artifact, and **no third package**". A loop over a deterministic CLI is a runbook and a shell line until someone shows it is not |
| Outcome classification (coexistence, famine, collapse, extinction) | **Phase 4b, and never in the engine's stream** | A classification is an interpretation of facts. Putting the label in the authoritative record makes a threshold change rewrite history. 4a instead emits the **facts a classifier consumes**, so a reclassification re-reads old runs rather than invalidating them |

The reduction is the same shape as the one the product owner made on 2026-08-19 for Phase 2, and for the same
reason: the roadmap's *In scope* list was written before the constraints were priced.

## Three collisions with approved architecture

Each is resolvable, and each resolution is a design choice the technical owner has to make rather than one the
implementation agent may assume.

### 1. There is no JSON library, and there will not be one

`ARCH-MOK-001` prohibits any engine dependency and "admits no exception, including a dependency shared with another
package in the same workspace." `serde` and `serde_json` are therefore unavailable, and `cargo tree -p Mokiterions`
resolving to one crate is a `REPOSITORY_CONTEXT.md` verification step.

JSON must be written by hand. That is normally where escaping bugs live, and here it is not, because **the value
domain is closed**: every field a record carries is an identifier matching `M[0-9]{2}` or `F[0-9]{4}`, one of twelve
fixed names, one of a fixed set of lowercase enum words, or an integer. No value is operator-supplied and no value
contains a quote, a backslash, or a control character. The proposal is that `SPEC-MOK-006` **declare that alphabet**
and that the writer be verified by exhaustion over it, so that escaping is provably total rather than defensively
coded. If a later phase admits a free-text field, the escaping obligation arrives with it and not before.

### 2. A structured stream needs a destination, and the engine has never opened a file

`SPEC-MOK-001` states that "invalid input is treated as data and never interpreted as code or a filesystem path"
and that "state is held in memory and no persistence is required". `ARCH-MOK-001`'s data-flow diagram ends at
standard output, and its component list gives the binary target one job: buffer the streams, call `execute`, map
the exit code.

| Considered | Assessment |
|---|---|
| **`--events-path <path>`, library takes a writer, binary opens the file** (recommended) | `cli::parse` returns the path as **data** in `Config`; the library never opens it and never touches `std::fs`. `execute` gains a fourth parameter, a sink the host supplies. The engine's dependency table stays empty, persistence stays out of the library, and the *Security and privacy* provision needs narrowing to say the **library** never interprets input as a path — which stays literally true |
| Structured records on standard error | Needs no path at all, which is its whole appeal. Rejected: `execute` writes `runtime error: …` to standard error, so an interrupted run corrupts the tail of the file, and a stream that is only valid when nothing goes wrong is not evidence |
| Structured records replace the text stream behind `--format` | Rejected outright. `REQ-MOK-010` obliges the text record whenever the events occur; a flag that suppresses it makes an approved requirement conditional on an option |
| Text stream and JSONL interleaved on standard output | Rejected. It ends byte-identical text output, which is the property every existing determinism test and every retained capture depends on |

The recommended form has direct precedent one package away: `mokiterions-tui` already resolves an `--export` path,
and `SPEC-MOK-003` rule 9.4 to 9.6 already governs a written artifact. What is new is only that the engine does it.

### 3. `execute` is enumerated, so its signature is a governed surface

`SPEC-MOK-002` rule 5 is a closed enumeration of the library target's public interface, and rule 6 forbids handing
out a borrow of authoritative state. A fourth parameter on `execute` is public-interface growth and needs an
amendment under rule 5's own growth clause — the same clause the observer's snapshots and Phase 2's fourth
attribute were admitted under. It does **not** touch rule 6: a sink is the host's, not the engine's, and a record
written into it is a value.

Adding `execute_with_records` beside `execute` would avoid the amendment and is not recommended: it would leave two
process boundaries with two contracts, and rule 5 exists to prevent exactly that.

## Design of the record stream

### One record per line, discriminated

```json
{"record":"header","schema":1,"engine":"0.1.0","config":{"seed":0,"ticks":200,"policy":"reference","density":"0.75","trace_actions":false}}
{"record":"event","tick":0,"subject":"M01","event":"agent_initialized","result":{"name":"Zug","position":{"x":89,"y":34},"territory":"A","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":6}}
{"record":"metrics","tick":1,"living":12,"deaths":0,"population":{"A":6,"B":6},"health":{"sum":1200,"min":100},"satiety":{"sum":1188,"min":99},"energy":{"sum":1188,"min":99},"fear":{"sum":0,"max":0},"territories":{"A":{"standing":61,"low":20,"medium":20,"high":21,"capacity":61,"depleted":false},"B":{"standing":61,"low":20,"medium":20,"high":21,"capacity":61,"depleted":false}}}
{"record":"run","reason":"tick_limit","ticks":200,"survivors":12,"deaths":0,"crossings":4,"consumed":{"low":31,"medium":18,"high":2},"regenerated":38,"regeneration_skipped":{"at_capacity":1,"depleted":0},"agents":[{"id":"M01","name":"Zug","died_at":null,"territory":"A"}]}
```

Four record kinds. `event` is a **projection**: exactly one per emitted text line, in the same order, carrying the
same facts under the same vocabulary, whatever `--trace-actions` is set to. `header`, `metrics` and `run` are the
three kinds with no text counterpart, `run` excepted in that the text summary states a subset of it. Naming that
exception explicitly is the point of the `record` discriminator: a consumer that wants only the projection filters
on one field.

### No floats

The stream carries **integers only**. Where an average is wanted, the record carries the **sum and the population
count** and the consumer divides. This is not fastidiousness: an average of twelve `u8` attributes is a rounding
decision, `SPEC-MOK-001` would have to specify it, float formatting would enter a byte-identical determinism
contract, and every future consumer would inherit a precision it did not choose. Sums cost nothing and lose nothing.

### No inert fields

The roadmap's metric list includes **conflict frequency**. The engine has no conflict, and Phase 3 is independent
of this phase and may not land first. Emitting `"conflicts":0` would be a claim the engine cannot support.
`SPEC-MOK-003` rule 4.5 set the precedent when it *refused* a fear gauge the engine could not yet compute, and
Phase 2 recorded the opposite case — an attribute computed and consumed by nothing — as a residual rather than as
completeness. Conflict metrics are therefore out of scope, and the schema's version field is how they arrive later.

### What the engine must start retaining

The `Simulation` struct holds no cumulative counter today: no crossing count, no consumption or regeneration
totals, no death tick. Phase 4a adds them. They are pure counters — they read no entropy, draw nothing from the
shared stream, and change no proposal — so **the additivity property Phases 2 and 2.5 established is preserved**:
every run predating this change decides the same things in the same order and emits the same text bytes. That is a
claim to measure across the declared seeds, not to assert.

The stream is written as it is produced. `Simulation` already keeps `collected_events` absent for the text host so
that "a long run retains nothing it does not need", and the record stream follows the same discipline.

## Verification obligations

The contract should stand or fall on four mechanical checks and no screenshot.

1. **Projection.** For every declared seed × each of the three policies × trace on and off, the `event` records
   map one-to-one onto the text lines, in order, with every field equal. A parser that reconstructs each text line
   from its record and compares bytes is the cheapest form of this and is the recommended one.
2. **Non-perturbation.** The text stream is byte-identical with and without `--events-path`, and the per-tick
   entropy draw count is identical. This is the engine-side twin of the observer's headline test, and
   `mokiterions-tui/src/verification.rs` is the pattern to copy.
3. **Additivity.** Runs recorded before this change reproduce byte for byte after it.
4. **Escaping totality.** The writer is exercised over the declared value alphabet exhaustively, and the output of
   every run in check 1 parses as JSON by a parser the repository does not own — a Python one-liner in the evidence
   capture, since the engine cannot link one.

Failure behavior needs its own case: a sink that fails mid-run must exit `1` and must not let the run claim
successful completion, matching `SPEC-MOK-001`'s existing standard-output rule. A truncated record file that
reports success would be worse than no record file.

## Proposed artifact packet

| Proposed ID | Type | Subject |
|---|---|---|
| `INT-MOK-009` | intent | Make a run measurable by something other than a human reading lines |
| `CAP-MOK-009` | capability | Structured, integer-only measurement of a run, additive to the text record |
| `REQ-MOK-042` | requirement | Emit one structured record per authoritative event, as a projection of the text record |
| `REQ-MOK-043` | requirement | Report per-tick world metrics the event stream does not already state |
| `REQ-MOK-044` | requirement | Report an end-of-run measurement carrying the facts an outcome classification consumes, and no classification |
| `REQ-MOK-045` | requirement | Leave the text stream, the entropy draw sequence, and every prior run's output unchanged |
| `REQ-MOK-046` | requirement | Surface a record-sink failure and refuse to claim a completed run |
| `SPEC-MOK-006` | specification | The record schema, the closed value alphabet, the four record kinds, the metric set, and the sink contract |
| `ADR-MOK-005` | ADR | The structured stream as an additive projection; writer-not-path; hand-written JSON on a closed alphabet; no classification in the engine |
| `VER-MOK-012` | verification | The four checks above, plus the failure case |
| `WO-MOK-012` | work order | Phase 4a implementation |

**In-place amendments required, none of them substantive to world rules:**

- `SPEC-MOK-001` — *Outputs* gains the record sink; *Help output* gains `--events-path`; *Security and privacy*
  narrows "never interpreted as … a filesystem path" to the library; *Performance and capacity* acknowledges a
  written stream; *Observability* states the projection.
- `SPEC-MOK-002` — rule 5's enumeration grows by `execute`'s fourth parameter and the record-writer item, under
  rule 5's growth clause. Rule 6 is re-checked, not amended.
- `ARCH-MOK-001` — the data-flow diagram gains the second sink, and the amendment states that persistence stays
  out of the library and the dependency table stays empty.
- `SPEC-MOK-002` rules 7 to 10 — the new tests' tiers. The projection and non-perturbation checks are
  public-tier; the JSON writer's escaping cases are internal.

**A new specification rather than an amendment to `SPEC-MOK-001`,** on the precedent `SPEC-MOK-003` set: a record
schema is an independently versioned surface with its own compatibility obligation, and `SPEC-MOK-001` is the
world-rules contract. The amendments above are the seams where the two meet.

**Correction, on drafting.** This section named **five** `SPEC-MOK-001` seams. Drafting `ADR-MOK-005` against the
approved text found **nine**: the four additions are *Actors* ("There are no external systems or network calls"),
*Observability*, *Compatibility and migration*, and *Explicitly unspecified decisions*. `ARCH-MOK-001` likewise needs
twelve provisions rather than the two named above, including `addresses` and `conforms_to` edges and a
`decision_assessment.rationale` naming `ADR-MOK-005`. `SPEC-MOK-002` needs five rather than two, and the
"record-writer item" named above **is not one of them** — the record writer stays private, so the interface grows by
`execute`'s one parameter and by nothing else. The counts are corrected here rather than left to be discovered,
because a specification amended in five of the nine places it needed would be internally inconsistent in exactly the
way an amendment record exists to prevent.

**Every identifier above must be re-checked against every remote ref immediately before the packet is created.**
`VER-MOK-009` is free and is deliberately left free: `docs/RELEASE_RUNBOOK.md` explains the gap, and filling it
would make that explanation read as an error. Two branches claiming `010` simultaneously already cost this
repository an eighteen-conflict renumbering under `WO-MOK-011`; other agents are working the same identifier space
now.

## Sequencing

```text
Phase 4a  Structured measurement in the engine   ── proposed for authorization
   │
   ├── evidence: N-seed captures become machine-checkable at no marginal cost
   │
   ▼
Phase 4b  Distribution and classification        ── deferred, and cheaper once 4a exists
   │
   ├── the package question decided on evidence rather than in advance
   └── classification stated as a table over 4a's `run` records
```

Phase 4a is independent of Phase 3 and does not block it. If Phase 3 lands first, its events join the projection
for free and its metrics arrive as `schema: 2`.

**What Phase 4b needs before it can be specified honestly**, and what 4a produces toward it: a measured answer to
whether a batch loop needs to be a program at all. If a shell loop over the existing binary plus a script under
`scripts/` — where twelve Python instruments and their tests already live — produces the distribution evidence
Phase 6 needs, then Phase 4b is a runbook and a verification contract, and `ARCH-MOK-001`'s third-package
prohibition is never touched. If it does not, the third package is argued on that finding, with the same shape of
requirement `REQ-MOK-026` used for the observer. Deciding this now would be deciding it without the measurement.

## Decisions recorded

The repository owner took the six decisions above, and five more that arose from them, on **2026-08-20**, acting as
product owner and technical owner. They are design decisions, not approvals: every artifact in the packet is `draft`
and none is approved.

| # | Decision | Outcome | Role |
|---|---|---|---|
| 1 | The split | **Phase 4a alone.** Batch execution and outcome classification stay unauthorized | product owner |
| 2 | The sink | **`--events-path`.** The library takes a `Write`; the binary target opens, flushes, closes and removes the file | technical owner |
| 3 | Metrics at all | **Emit them**, per tick, redundantly with the event stream, deliberately | product owner |
| 4 | Integers only | **Confirmed.** No mean, average, ratio, rate or percentage. The consumer divides | product owner |
| 5 | Traces | **Mirror `--trace-actions`.** One record per emitted text line, never more and never fewer | product owner |
| 6 | An existing destination | **Overwritten** without prompting; a partial file the process created is **removed** on failure | technical owner |
| 7 | Conflict metrics | **Excluded** until Phase 3 computes conflicts, and admitted then under a schema increment | product owner |
| 8 | Schema version | **Confirmed.** A `schema` field in the header record from the first stream, a declared compatibility surface | technical owner |
| 9 | Where the rules live | **A new `SPEC-MOK-006`**, with amendment seams in `SPEC-MOK-001` rather than a rewrite of it | technical owner |
| 10 | Whether to begin now | **Begin.** Treat the outstanding verification backlog as unrelated to this work, and record no gate override | product owner |
| 11 | What to produce now | **The full packet as `draft` artifacts**, committed on a branch and not pushed | product owner |

Decision 5 was not in the list above because the question only became visible while designing the record kinds: an
action trace is a text line but not an authoritative event, so a stream that carried traces unconditionally would break
the one-to-one correspondence, and a stream that dropped them would lose the only per-decision record the engine
produces. Mirroring the flag keeps the correspondence total in both directions.

Decision 6 likewise arose from the design: `SPEC-MOK-003` rule 9.4 already truncates without prompting for the
observer's export, so the only real question was whether the engine should differ, and the answer is that one
file-writing convention is better than two.

**Decision 10 was taken over a stated objection, and the objection has since been overtaken by events.** At the time it
was taken, `VREC-MOK-011` was `ready` rather than `verified` and `WO-MOK-011` was `in_progress`. The objection recorded
was that beginning new work while a verification record is outstanding "is exactly the reasoning `WO-MOK-010`'s gate
existed to prevent, and it leaves no record of the judgment"; the owner's decision was to begin regardless and to
record no override, and `WO-MOK-012` was written to state the precondition status factually rather than to assert it
was clear. **`origin/master` then advanced to `ff3a155`**, carrying `VREC-MOK-011` to `verified` and `WO-MOK-011` to
`implemented`. Every verification record in the repository is now `verified` and every work order but the `draft`
`WO-MOK-008` is `implemented`. The packet was rebased onto `ff3a155`, so decision 10 no longer has a subject and the
objection no longer has force. It is recorded here because a decision taken over an objection should not disappear from
the record when the objection stops mattering.

**One precondition remains outstanding and predates this chain.** `ARCH-MOK-001`'s amendment record carries a row dated
2026-08-18 whose approval column reads **OUTSTANDING**, and which names itself an approval precondition of
`WO-MOK-005`. It is not this chain's row and `WO-MOK-012` does not claim it, but this chain proposes to add a row
beneath it, so the technical owner should expect to resolve it in the same act. `VER-MOK-012` oracle 7 records its
state either way.

## What this proposal does not close

`fear` is still read by nothing. The trait vector is still one trait. Per-agent entropy substreams are still
unimplemented. High-class resource accumulation is still carried, and `REQ-MOK-014`'s and `REQ-MOK-034`'s floors
still describe tick 1,000 rather than a steady state. Phase 4a would make all five of those *easier to measure* and
would resolve none of them.

**Corrected on 2026-08-20.** This paragraph originally read that "`WO-MOK-010` and `WO-MOK-011` are implemented and
unverified, `VREC-MOK-010` and `VREC-MOK-011` are `ready` candidates that take no decision". That was wrong about the
first and stale about the second. `VREC-MOK-010` was already `verified` and `WO-MOK-010` already `implemented` when
this proposal was written; only `VREC-MOK-011` was outstanding, at `ready`, with `WO-MOK-011` `in_progress` and an
unmerged branch carrying it to `verified`. That branch has since merged as `ff3a155`, so **every verification record in
the repository is `verified` and every work order but the `draft` `WO-MOK-008` is `implemented`.** Phase 2's and Phase
2.5's verification is closed, and the decision-rights question this paragraph raised has no live instance.

The one governance item still open is `ARCH-MOK-001`'s outstanding 2026-08-18 amendment row, described under
*Decisions recorded*. `WO-MOK-008` — the release-authorization chain — remains `draft` and is unrelated to this work.
