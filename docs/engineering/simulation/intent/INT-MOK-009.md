+++
id = "INT-MOK-009"
type = "intent"
title = "Make a run measurable by something other than a human reading lines"
status = "draft"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"

[relations]
+++

# Intent: Make a run measurable by something other than a human reading lines

## Problem

The project's stated success criterion is that twelve Mokiterions "produce observable outcomes that are not
predetermined by the simulation engine." That is a claim about a **distribution** of outcomes across many runs, and
this repository has no way to state one.

What it has instead is a line-oriented text stream designed for a person, and an observer designed for a person
watching one run. Both are good at what they are for. Neither is a measurement surface:

- `REQ-MOK-010`'s text record is a human-facing format. Every field is positional, every value is embedded in a
  `key:value,key:value` result string, and its stability is a determinism property rather than a schema.
- `SPEC-MOK-003` rule 9.4's export is deliberately *the same text format*, "the `SPEC-MOK-001` line format", so it
  inherits the same problem. It is the observer's only retainable artifact and it is not machine-oriented.
- Facts the engine holds and the stream never states — a territory's capacity, whether it is permanently depleted,
  how many crossings a run saw, how much of each food class was consumed — are reachable only by the observer's
  snapshot surface, in memory, in a terminal.

The consequence is visible in this repository's own history. Every quantitative claim made so far — the density
curve, the fifty-seed trait distribution that narrowed `WASTE_TOLERANCE_MAX` to `40`, the ninety-run projection
check under `WO-MOK-011` — was produced by writing a bespoke parser against a human-facing stream, or by writing a
throwaway Rust probe against engine internals and retaining it as evidence
(`evidence/WO-MOK-010/observer/frame-probe.rs`, `evidence/WO-MOK-011/observer/frame-dump.rs`). Those measurements
are sound. The method does not scale to Phase 6, where the result *is* a distribution, and it puts a parser the
repository wrote between the engine's facts and the conclusion drawn from them.

## Desired outcomes

- A run states its own facts in a form a program reads without knowing the engine's rules, and states the facts
  that an outcome classification consumes without stating the classification.
- A quantitative claim about many runs is checked by re-reading retained records, not by re-running the engine and
  re-writing a parser.
- Enabling measurement changes nothing about the run being measured, and this is measured rather than assumed.
- The facts a later phase needs — Phase 3's conflicts, Phase 4b's distributions, Phase 6's comparison — have a
  declared place to arrive, so the measurement surface is designed once instead of extended under pressure.

## Actors and stakeholders

- The **product owner** bears the project's central claim. Without distributions it rests on narrative.
- The **assurance owner** needs evidence a parser can check. A retained capture that only a human can read is the
  weakest evidence this repository accepts, and `SPEC-MOK-003`'s buffer-assertion rule was written for exactly
  that reason on the observer's side.
- The **technical owner** needs the record schema fixed before Phase 3 and Phase 4b consume it, so that the
  contract is authored once rather than rewritten twice — the same argument that placed Phase 2 before Phase 5.
- The **operator** running experiments benefits, and is not the reason this exists.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Machine-readable records emitted per run | 0 | one per emitted text line, plus one header, one per completed tick, and one terminal record | Any run with a sink configured |
| Declared classification facts obtainable without re-implementing engine rules in the consumer | partial | all declared in `SPEC-MOK-006` | End-of-run record |
| Bytes by which the standard-output text stream differs when a sink is configured | n/a | 0 | Each declared seed, each policy, trace on and off |
| Entropy draws by which a run differs when a sink is configured | n/a | 0 | Each declared seed |
| Runs recorded before this change that no longer reproduce | n/a | 0 | The retained captures of `WO-MOK-002`, `WO-MOK-010`, `WO-MOK-011` |
| Records requiring a bespoke parser to reconstruct their text counterpart | all | 0 | Each declared seed |
| Non-integer values in the stream | n/a | 0 | Every record of every run |

## Non-goals

This intent does not solve, and must not be read as authorizing:

- **Batch execution across seeds.** One run writes one stream. A loop over runs is Phase 4b and is unauthorized.
- **Outcome classification.** Naming a run a famine, a collapse or a coexistence is an interpretation of facts. This
  intent delivers the facts and explicitly refuses the label — see *Principles*.
- **Run persistence beyond one stream per run.** No database, no index, no run registry, no directory convention.
- **Conflict, combat or social metrics.** The engine has no conflict. Phase 3 is independent of this work and may
  land after it.
- **Any change to a simulation rule**, to the text stream, to the exit-code contract, to the observer, to the
  default configuration, or to what any decision source proposes.
- **A third package, a new dependency, or any growth of the engine's dependency table**, which stays empty.
- Structured output from the observer. The observer's export stays the text format `SPEC-MOK-003` rule 9.4 fixes.
- Model-backed decisions, per-agent entropy substreams, a second trait, and any consumer for `fear`.

## Principles and immutable constraints

Downstream decisions may not violate these.

1. **Additive.** `REQ-MOK-010` stays unconditional. No option, and no combination of options, may suppress,
   reorder, or alter the text record. A structured stream that costs the text stream its bytes is refused.
2. **The engine's authority is untouched.** Records are values written out. Nothing in this work reads a proposal,
   validates an action, or reaches authoritative state; `ARCH-MOK-001` and `ADR-MOK-001` are not reopened.
3. **Facts, not judgments.** The engine states what happened. It does not state what that means. A classification
   threshold must be changeable without invalidating a retained record, which is only true if the record carries no
   classification.
4. **Integers only.** No average, mean, ratio, or floating-point value enters the stream. Where an average is
   wanted the record carries a sum and a count and the consumer divides. Float formatting has no place in a
   byte-identical determinism contract, and an average of twelve `u8` attributes is a rounding decision that would
   otherwise have to be specified and then inherited by every future consumer.
5. **No inert fields.** A field the engine cannot compute is not emitted at zero. `SPEC-MOK-003` rule 4.5 set this
   precedent by *refusing* a fear gauge before `fear` existed, and Phase 2 recorded the converse — an attribute
   computed and consumed by nothing — as a residual rather than as completeness. Conflict metrics wait for Phase 3.
6. **Self-describing.** A retained record stream states its own schema version and the configuration that produced
   it, so a file can be understood without the shell line that made it.
7. **The library interprets no path.** The engine's library target does not open, create, resolve, or interpret a
   filesystem path. A host supplies a sink; the library writes to it.
8. **Determinism.** Two runs with the same seed, configuration and sink configuration produce byte-identical
   streams, and no record carries a wall-clock time, a hostname, an absolute path, an environment value, or a
   credential.

## Risks and assumptions

**Facts.**

- The engine package's dependency table is empty and `ARCH-MOK-001` "admits no exception, including a dependency
  shared with another package in the same workspace." No JSON library is available and none will be.
- `Simulation` retains no cumulative counter today: no crossing count, no consumption or regeneration total, no
  death tick. Every one of those must be added for `REQ-MOK-044`.
- `SPEC-MOK-001` currently states that "state is held in memory and no persistence is required" and that invalid
  input is "never interpreted as code or a filesystem path". Both provisions need narrowing.
- `execute` is enumerated by `SPEC-MOK-002` rule 5, so its signature is a governed surface.
- `mokiterions-tui` already resolves a path and writes a retainable artifact under `SPEC-MOK-003` rule 9.4 to 9.6,
  including removing a partial file on failure. The behavior is not new to the product, only to the engine.

**Assumptions.**

- That hand-written JSON is safe here because the value domain is closed: every field is an identifier matching
  `M[0-9]{2}` or `F[0-9]{4}`, one of twelve fixed names, one of a fixed set of lowercase words, or an integer. No
  value is operator-supplied. **This assumption is load-bearing and is stated as a rule in `SPEC-MOK-006` so that
  it is checked rather than trusted.** If a later phase admits a free-text field, the escaping obligation arrives
  with it.
- That adding cumulative counters is entropy-neutral, and therefore that every run predating this change reproduces
  byte for byte. This is the additivity property Phases 2 and 2.5 established, and `REQ-MOK-045` requires it to be
  measured rather than asserted.
- That one stream per run is sufficient for Phase 4b, and that a batch loop over a deterministic command-line
  program may not need to be a program. Phase 4b decides that on this work's evidence.

**Open decisions.** None outstanding at the time of drafting. Eleven decisions taken by the repository owner on
2026-08-20 are recorded in `docs/PHASE_4_PROPOSAL.md`, and the six that bind a downstream artifact appear as rules
in `SPEC-MOK-006` or as options in `ADR-MOK-005`.

One of the eleven was a decision to proceed while `VREC-MOK-011` was `ready` rather than `verified`, treating that
record's outstanding state as unrelated to this work and declining to record a gate override. **That decision no
longer has a subject.** `origin/master` advanced to `ff3a155` during drafting, carrying `VREC-MOK-011` to `verified`
and `WO-MOK-011` to `implemented`; every verification record in the repository is now `verified` and every work order
but the `draft` `WO-MOK-008` is `implemented`. This chain is drafted against `ff3a155`, so it begins from a fully
verified predecessor and needs no judgement about an unverified one. `WO-MOK-012` states the precondition status
factually, including the one amendment row that remains outstanding and predates this chain.
