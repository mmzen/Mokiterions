+++
id = "WO-MOK-012"
type = "work_order"
title = "Emit a structured record stream to an operator-named sink, leaving the observed run unchanged"
status = "draft"
owners = ["engineering owner"]
created = "2026-08-20"
updated = "2026-08-20"

[assurance]
commit_bound_verification = "required"
rationale = "The load-bearing claim of this work is a claim of absence: that the text stream is byte-identical, that no entropy draw moved, and that every run recorded before the capability existed still reproduces. A claim of absence cannot be asserted, only checked against a capture bound to the commit the work began from. The exposure is real rather than theoretical — a single accidental draw in a record-writing path would produce a different world while every record in the stream looked entirely plausible, and it would silently retire REQ-MOK-014's and REQ-MOK-034's measured survivor floors, the density viability curve and the fifty-seed distribution that fixed WASTE_TOLERANCE_MAX. Separately, the change writes to a file for the first time from the engine, adds six cumulative counters to authoritative state, and grows the engine's public interface, so SPEC-MOK-002 rules 5 and 6 must be re-checked against the enumeration rather than argued."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-042", "REQ-MOK-043", "REQ-MOK-044", "REQ-MOK-045", "REQ-MOK-046"]
specifications = ["SPEC-MOK-001", "SPEC-MOK-002", "SPEC-MOK-006"]
verification = ["VER-MOK-012"]
architecture = ["ARCH-MOK-001", "ADR-MOK-005"]
+++

# Work Order: Emit a structured record stream

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and the retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

`ARCH-MOK-001` is declared because it carries an `addresses` edge to `REQ-MOK-010`, the text observation interface this
work must leave untouched, and because the change crosses the boundaries it fixes: it introduces the engine's first
filesystem effect and its first output stream that is not the text stream. `ADR-MOK-005` is declared as the ADR that
decides the resulting collisions. **`ARCH-MOK-001` requires amendment**, and the amendment is stated in full in
`ADR-MOK-005` and reproduced in outline below. Unlike `WO-MOK-011`, this work order does not and cannot claim the
architecture needs none.

### Approval preconditions

**This work order cannot be approved before its governing artifacts are.** Every one of the following is `draft` at
the time of writing and none has been approved:

| Artifact | Approving role | State at drafting |
|---|---|---|
| `INT-MOK-009` | product owner | `draft` |
| `CAP-MOK-009` | product owner | `draft` |
| `REQ-MOK-042` … `REQ-MOK-046` | product owner | `draft` |
| `SPEC-MOK-006` | technical owner | `draft` |
| `ADR-MOK-005` | technical owner | `draft`, not accepted |
| The `ARCH-MOK-001` amendment `ADR-MOK-005` requires | technical owner | not written, not approved |
| The `SPEC-MOK-001` amendments `ADR-MOK-005` requires | technical owner | not written, not approved |
| The `SPEC-MOK-002` amendments `ADR-MOK-005` requires | technical owner | not written, not approved |
| `VER-MOK-012` | assurance owner | `draft` |
| This work order | engineering owner | `draft` |

Approving this work order does not approve any of them. In this repository one person holds all three accountable
roles, which makes each approval cheap to obtain and correspondingly easy to skip; nothing here is approved by
implication, and an artifact whose approval is not recorded is not approved.

The `ARCH-MOK-001`, `SPEC-MOK-001` and `SPEC-MOK-002` amendments are **approval preconditions of this work order**,
not deliverables of it. `VER-MOK-012` oracle 7 checks their presence and approval, and their absence fails that
contract regardless of the state of the code.

### Precondition status of the wider repository

Stated factually, at `ff3a155`, and taking no decision about any of it.

- **Every verification record in the repository is `verified`.** `VREC-MOK-001` through `VREC-MOK-011` are all
  `verified` at `ff3a155`, which `origin/master` reached during this chain's drafting: `965fe67` carried
  `VREC-MOK-011` from `ready` to `verified` and `89e00ec` set `WO-MOK-011` to `implemented`. This chain is drafted and
  based on `ff3a155` rather than on the `dac9bac` it was begun at, so its predecessor is fully verified and no
  judgement about an unverified predecessor is required. An earlier decision to proceed regardless has been overtaken
  by events and is recorded in `INT-MOK-009` as having lost its subject.
- **Every work order but one is `implemented`.** `WO-MOK-001` through `WO-MOK-007` and `WO-MOK-009` through
  `WO-MOK-011` are `implemented`. `WO-MOK-008` is `draft` — the release-authorization chain, unrelated to this work
  and not cited by it.
- **One amendment row remains outstanding and predates this chain.** `ARCH-MOK-001`'s amendment record carries a row
  dated 2026-08-18, narrowing the prohibition on public items from "mutable **or owned** authoritative state" to a
  mutable borrow of or reference into that state, whose approval column reads **OUTSTANDING** and names itself an
  approval precondition of `WO-MOK-005`. It is not this chain's amendment, this work order does not claim it, and
  nothing here depends on its substance: the narrowing it describes is favourable to owned snapshots, and every public
  item this work touches carries owned values or a caller-supplied writer either way.

  It is nonetheless a row on an artifact this work order proposes to amend. **The technical owner should expect to
  resolve it in the same act that approves this chain's `ARCH-MOK-001` amendment**, because adding a row beneath an
  outstanding one leaves the record harder to read than it was. Whether to do so is the owner's call and this work
  order does not require it; `VER-MOK-012` oracle 7 records its state either way.

### Why this chain is numbered 009, 042–046, 006, 005 and 012

The identifier space in this repository is shared across branches and across agents working in parallel clones, and a
collision costs a renaming of approved artifacts. Every identifier in this chain — `INT-MOK-009`, `CAP-MOK-009`,
`REQ-MOK-042` through `REQ-MOK-046`, `SPEC-MOK-006`, `ADR-MOK-005`, `VER-MOK-012`, `WO-MOK-012`, and `VREC-MOK-012`
when the verification record is written — was checked against every local branch, every remote branch and every tag,
twice: once at drafting and once after the fetch that discovered `ff3a155`. All are free in every ref.

`VER-MOK-009` is free and is deliberately left free. `docs/RELEASE_RUNBOOK.md:263` records why that gap exists, and
filling it opportunistically would destroy the record.

### Decision record

On 2026-08-20 the repository owner, acting as product owner and technical owner, took eleven decisions this packet is
drafted around. They are design decisions, not approvals. All eleven are recorded in `docs/PHASE_4_PROPOSAL.md`; the
ones that bind this work order are:

| Decision | Outcome | Role |
|---|---|---|
| Scope of this phase | Phase 4a alone. Batch execution and outcome classification stay unauthorized | product owner |
| How the destination is named | One option, `--events-path`; the library takes a writer and the binary opens the file | technical owner |
| Per-tick state | Emitted as its own record kind every tick, redundantly with the event stream, deliberately | product owner |
| Numbers | Integers only. No mean, ratio or rate; the consumer divides | product owner |
| Traces | Mirror `--trace-actions`; one record per emitted text line, never more and never fewer | product owner |
| An existing destination | Overwritten without prompting; a partial file the process created is removed on failure | technical owner |
| Conflict metrics | Excluded until Phase 3 computes conflicts, and admitted then under a schema increment | product owner |
| Schema version | A `schema` field in the header record from the first stream, a declared compatibility surface | technical owner |
| Where the rules live | A new `SPEC-MOK-006`, with amendment seams in `SPEC-MOK-001` rather than a rewrite of it | technical owner |

Three are recorded with their reasoning because the reasoning is what makes them defensible:

**Integers only, and the consumer divides.** The engine holds no floating-point value anywhere and `REQ-MOK-009`'s
determinism rests partly on that. A mean in the stream would be either a float — admitting the first float into a
record path — or a rounded integer, which is a lossy judgement the engine has no authority to make. The consumer that
wants a mean has the sum and the count.

**Per-tick metrics, despite the redundancy.** Most of the metrics record is derivable by replaying the event stream,
and the redundancy is the point twice over: it makes the stream usable without a replay engine, and it gives
`VER-MOK-012` oracle 6 two independently-computed figures to disagree. Three fields — capacity, permanent depletion,
and the attribute extrema — have no event counterpart at all, so a stream of events alone would not carry them.

**No outcome classification, at all, anywhere in the stream.** A classification is a threshold applied to figures, and
a threshold in a retained stream cannot be revised without invalidating every stream retained under the old one.
Leaving it out keeps every capture valid across every future revision of the judgement. This is a product decision
held deliberately, not a limitation inherited from the schedule, and it is `SPEC-MOK-006` rule 8.7.

### Required amendments to approved specifications and architecture

Three approved artifacts require amendment. Each is stated in full in `ADR-MOK-005` under *Required amendments*, and
each requires the technical owner's separate act. **The implementation agent writes the amended text and records in
each amendment record that it did so; it does not decide the substance.**

**`ARCH-MOK-001`** — twelve provisions. In outline: the entry point's responsibilities gain resolving the sink path,
creating and removing the file, and supplying the writer; the engine's responsibilities gain the record projection and
the cumulative counters; *Dependency direction*'s sentence that "No component depends on network or persistence
infrastructure" is qualified for the binary target and left absolute for the library target; the data-flow diagram
gains the record branch off the same ordered event; the *Prohibited patterns* clause on an empty dependency set is
restated unchanged and explicitly reaffirmed against the temptation to add a JSON library; *Quality attributes*
Determinism and Debuggability gain the stream; the eight *Conformance checks* gain the library target's freedom from
filesystem calls and the single emission site; `REQ-MOK-042` through `REQ-MOK-046` join `addresses`; `SPEC-MOK-006`
joins `conforms_to`; and `decision_assessment.rationale` records `ADR-MOK-005` as the ADR its `adr_required` outcome
called for.

**`SPEC-MOK-001`** — nine amendment seams. In outline: *Scope*'s sentence that it "does not define … structured
output" is narrowed to say that `SPEC-MOK-006` does; *Actors*' "There are no external systems or network calls" is
qualified for the file the binary target creates; the *Inputs* synopsis and option list gain `--events-path`; the
*Help output* section gains its usage entry; *Outputs* gains the sink stream by reference; *Error and recovery* gains
the sink failure cases and states that the exit-code contract is unchanged; *Security and privacy*'s "Invalid input is
treated as data and never interpreted as code or a filesystem path" is narrowed to the library target and stated as
holding absolutely there; *Performance and capacity*'s "State is held in memory and no persistence is required" is
qualified; and *Observability*'s byte-identical claim gains the sink's determinism. *Compatibility and migration* and
*Explicitly unspecified decisions* gain matching entries.

**The proposal in `docs/PHASE_4_PROPOSAL.md` named five seams. A close reading found nine.** The four additions are
*Actors*, *Observability*, *Compatibility and migration* and *Explicitly unspecified decisions*. The growth is
recorded rather than quietly matched to the earlier number, because a specification amended in five of the nine places
it needed would be internally inconsistent in exactly the way an amendment record exists to prevent.

**`SPEC-MOK-002`** — five provisions. In outline: rule 5's enumeration records `execute`'s new signature; rule 5's
mechanical signature check is updated so that it still fails on drift; the growth clause records `REQ-MOK-042` as the
approved requirement that justified the growth; the exit-code enumeration is restated unchanged, because "no new exit
code" is a claim worth recording as a decision; and **rule 6 receives no amendment** — recorded explicitly so that the
absence is a decision rather than an omission. The added counters are private, the sink is a caller-supplied writer,
and nothing added yields a mutable borrow of or a reference into authoritative state. That is a claim, and
`VER-MOK-012` checks it item for item rather than accepting it.

## Objective

Make a run measurable by a program: emit one JSONL record per emitted text line, one metrics record per completed
tick, and one terminal run record, to a sink the operator names — and demonstrate that the run itself did not move.
No text byte changed, no entropy draw moved, every previously recorded run reproduces, and no existing test's
assertions changed.

## In scope

- `--events-path <path>`, parsed by the existing option machinery, at most once, with a missing value and a value
  beginning with `--` treated as a missing value, and the empty string and `-` rejected as invalid configuration.
- The library target accepting a sink as a `Write` implementation on one new optional parameter of `execute`, and
  performing no filesystem operation of any kind.
- The binary target resolving the path, creating and truncating the file, supplying a buffered writer, flushing and
  closing it, and removing the file it created on failure.
- The header, event, metrics and run record kinds exactly as `SPEC-MOK-006` rules 5 through 8 fix them, in the order
  rule 9 fixes, with the field order rule 2.3 fixes.
- The record projection placed at the single point every authoritative event already passes through, so that the
  one-to-one correspondence with the text stream is structural rather than maintained.
- The six cumulative counters `SPEC-MOK-006`'s *State model* adds, each incremented in the same statement sequence as
  the event it counts, each saturating, each private, and each read only by the record producer.
- Per-Mokiterion death ticks, retained as `Option<u64>` and reported in the run record.
- The `final` object in the run record carrying the summary line's twelve figures, so that the text summary line is
  reconstructible and the correspondence is total in both directions.
- The closed value alphabet as a property of the code: values written between quotation marks unaltered, with the
  exhaustive check `VER-MOK-012` oracle 5 requires.
- The twelve `ARCH-MOK-001`, nine `SPEC-MOK-001` and five `SPEC-MOK-002` provisions above, written as amendments the
  technical owner has approved, each with an amendment-record row.
- Every check, measurement and manual assessment `VER-MOK-012` requires, and the evidence it retains.
- Updating `SIMULATION_RULES.md` and the functional roadmap with this phase's entry.

## Out of scope

- **Batch execution.** No loop over seeds, no run-set concept, no aggregate across runs, no parallelism. Phase 4b.
- **Outcome classification.** No label, category, verdict, severity, threshold or interpretation, in any record kind,
  at any seed. This is the single most tempting scope creep in this work and it is prohibited rather than deferred.
- Any consumer: no reader, parser, schema file, example script, or documentation of how to analyse the stream beyond
  what `SPEC-MOK-006`'s examples show. The reconstructor and replay consumer `VER-MOK-012` oracles 2 and 6 require are
  verification artifacts, retained as evidence, and are not product.
- Any change to the text stream: not a field, not a separator, not an order, not a line ending, not one byte.
- Any change to a simulation rule, survival value, resource table, density mapping, perception radius, regeneration
  schedule, decision source, default, floor, or exit-code mapping for any case that exists today.
- Any new exit code.
- Any draw against the shared entropy stream, added, removed, moved or made conditional.
- Any external dependency, in either package. In particular no JSON or serialization library, and no dependency shared
  with `mokiterions-tui`.
- Any floating-point type or operation, anywhere in this change.
- Any mean, average, ratio, rate, percentage or derived statistic in any record.
- Any field for a phenomenon the engine does not compute: no conflict, combat, threat, retreat or surrender field, at
  zero or otherwise. `SPEC-MOK-006` rule 10.4 forbids reserving them.
- Any second sink, sink rotation, size limit, stream truncation, directory convention, log file or second diagnostic
  destination.
- Any path, wall-clock time, duration, hostname, user, environment value or process identifier in any record —
  including the sink's own path.
- Any reading of a file, directory or environment variable. The one filesystem effect is a write to a destination the
  operator named.
- Any change to `mokiterions-tui`, beyond what a changed `execute` signature mechanically requires at its call sites.
  The observer does not consume the record stream and gains no option.
- Any change to `Observation`, `PerceivedMokiterion`, `DecisionSource` or `AgentSnapshot`.
- Any growth of the engine's public interface beyond `execute`'s one parameter.
- Any edit to a record bound by a `verified` verification record, any edit to retained evidence, and any edit to an
  existing amendment-record row.
- Resolving `ARCH-MOK-001`'s outstanding 2026-08-18 amendment row. That is the technical owner's act, disclosed above.
- Publishing anything. This work commits to a branch. A push, a pull request or a release is the owner's act, in the
  turn they ask for it.

## Authorized decision envelope

The implementation agent may decide locally:

- Rust file organization, private type and function names, where a private helper lives, and whether the record writer
  is a private struct, a set of free functions or a module.
- The exact form of `execute`'s new parameter — a generic `Option<&mut W>` bounded by `Write`, an `Option<&mut dyn
  Write>`, or an equivalent — provided it is exactly one parameter, provided the library performs no filesystem
  operation, and provided `SPEC-MOK-002` rule 5's enumeration and its mechanical check are amended to the form chosen.
- The integer widths of intermediate accumulations, provided every counter is `u64`, every counter saturates, and no
  sum over the living population can overflow in a debug build.
- Which test tier each new test belongs to, applying `SPEC-MOK-002`'s rule as written: the public tier only if the
  test is writable through the library target's public interface with its assertions unchanged. **Widening an item to
  `pub` in order to relocate a test is prohibited**, and a test that seems to need it belongs inline.
- The `#[cfg(test)]` entropy-state accessor `VER-MOK-012` oracle 4 requires, provided it returns an owned `u64`, is
  `#[cfg(test)]` in the merged tree, and is named by internal-tier tests only.
- The internal organization of test modules, and test helper functions.
- The exact wording of the amended text, once the technical owner has approved the substance, and the wording of the
  plain-language and roadmap updates.
- The form of the evidence scripts, the reconstructor and the replay consumer, provided each is retained in full and
  reviewed, and provided the reconstructor derives from `SPEC-MOK-006` rule 6.6 alone.
- How the binary target establishes that it created the destination, provided a destination it cannot establish it
  created is not removed and the diagnostic says so.
- The exact wording of the diagnostic messages, within `SPEC-MOK-006` rule 13.5.

The implementation agent may **not** decide:

- Any record kind, field name, field order, value form, or the framing. Those are `SPEC-MOK-006`'s.
- Whether a field is added to a record. A field the specification does not name is out of scope even if it is
  obviously useful; it needs a specification amendment first.
- Whether any figure is expressed as a non-integer, or any derived statistic is added.
- Whether the stream carries an outcome classification. It does not.
- The option's name, its default, or the two rejected spellings.
- Whether the library performs a filesystem operation. It does not, and a design that needs it to is the wrong design.
- Whether an existing destination is overwritten, or whether a partial file is removed.
- The schema version's starting value or its increment triggers.
- Any relaxation, widening, removal, rename or `#[ignore]` of an existing test, and any edit to the assertions in
  `mokiterions-core/tests/process.rs` or `mokiterions-core/tests/viability.rs`.
- Any `allow` attribute, any lint suppression, or any change to a gate command.
- Any change to a constant, a floor, a default or an exit code.
- The substance of any amendment, or the addition of an amendment row for an amendment the technical owner has not
  approved.
- Whether the pre-change baseline is recaptured. It is captured once, before the first code change, and a discrepancy
  is never resolved by recapturing it.

## Constraints

- **The run does not move.** Not one byte of the text stream, not one entropy draw, not one exit code, not one
  simulation outcome. `REQ-MOK-045` and `SPEC-MOK-006` rule 11 admit no tolerance.
- **Every run recorded before this change reproduces byte for byte.** The captures retained under `WO-MOK-002`,
  `WO-MOK-010` and `WO-MOK-011` are the check, and unlike `VER-MOK-010`'s comparison no projection is applied, because
  nothing in the text stream is permitted to change.
- **No existing test's assertions change.** Call sites of `execute` change mechanically because its signature does;
  assertions do not.
- The engine's dependency and dev-dependency tables stay empty. `cargo tree -p Mokiterions` resolves to one package.
  `ARCH-MOK-001` admits no exception, including a dependency shared with `mokiterions-tui`.
- The observer's dependency set stays at one path dependency and `ratatui` at its pinned version and feature set.
- The engine's library target contains no filesystem operation: no file creation or opening, no path resolution, no
  removal, no directory access, no temporary-file use.
- The engine's public interface grows by exactly one parameter on `execute` and by no item.
- No public item yields a mutable borrow of, or a reference into, authoritative state or any counter, in any build
  configuration including test builds.
- Integer and text operations only. No floating point anywhere in this change.
- The sink stream is UTF-8, line-feed terminated including the last line, with no byte-order mark, no carriage return
  and no blank line, fixed by the specification rather than taken from the platform.
- No unordered collection is iterated where the traversal order reaches a record, a text line or a decision.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` and
  `cargo test` at the workspace root all clean, in one invocation each, with nothing suppressed. `--locked` is
  mandatory.
- No secret, credential or model-provider key enters the repository, in code, in a fixture or in retained evidence.

## Expected change surface

**`mokiterions-core/src/cli.rs`**

- `Config` gains an optional sink path. `--events-path` joins the option table, the at-most-once check and the
  missing-value handling, using the existing `option_value` helper unchanged so that a value beginning with `--` is a
  missing value for free.
- The empty string and `-` are rejected with the configuration error, before the run.
- `USAGE` gains one line, in the existing `concat!` form, one literal per line.
- Tests: the option's presence and default held equal between `USAGE` and the parser by the existing test that already
  holds every other option's, extended rather than duplicated.

**`mokiterions-core/src/lib.rs`**

- `execute` gains one optional sink parameter. Its three exit codes are unchanged: `0`, `1` for an output failure now
  including a sink write failure, `2` for invalid configuration.
- The doc comment records that the library writes to a sink it is given and opens nothing.

**`mokiterions-core/src/simulation.rs`**

- Six private cumulative counters and a per-Mokiterion `Option<u64>` death tick, incremented in the same statement
  sequence as the events they count, saturating, with no reader inside the engine but the record producer.
- A private record writer: the four record kinds, the fixed field order, integer formatting, and string values written
  between quotation marks unaltered on the closed alphabet.
- **The record projection is added at `emit`**, the single function every authoritative event already passes through,
  so that `REQ-MOK-042`'s correspondence cannot drift. A second emission site is a defect, not an alternative.
- The metrics record computed at the tick boundary from state the engine already holds, with the extrema absent rather
  than zero when the living population is empty.
- The run record emitted after the final metrics record, carrying the twelve figures `emit_summary` already writes as
  its `final` object.
- Internal-tier tests: the counters against event counts, entropy neutrality either side of every record path, the
  entropy state at tick boundaries with and without a sink, the exhaustive alphabet enumeration with its per-domain
  size assertions, the empty-living-population extrema, and the absence of any counter reader.

**`mokiterions-core/src/main.rs`** (or the binary target's current file)

- Path resolution, file creation with truncation, a buffered writer, flush and close before success is reported, and
  removal of a file this process created on failure. This is `mokiterions-tui/src/export.rs`'s `write_file` pattern
  applied to a stream rather than to a completed buffer, and it is the precedent to follow rather than reinvent.
- The binary target stops being a nineteen-line shim. `ARCH-MOK-001`'s "stays thin" is amended accordingly, and
  manual assessment 5 in `VER-MOK-012` is where the owner judges whether it stayed thin enough.

**`mokiterions-tui`**

- Call sites of `execute` in `src/verification.rs` and `tests/verification.rs` pass the new parameter as absent.
  Nothing else changes, no assertion changes, and the observer gains no option and no knowledge of the record stream.

**Tests that must be updated because their subject changed, and only these:** the `execute` call sites above, and the
`USAGE`-versus-parser test extended for one option. Every other existing test is expected to pass untouched, and the
census is what demonstrates it.

**Documentation:** `SIMULATION_RULES.md` and `../../../ROADMAP.md`.

## Required verification

`VER-MOK-012`, in full: its seven oracles, its requirement-to-evidence matrix, its property and invariant tests, its
static, architecture, security, privacy, performance and resilience checks, and its eight manual assessments. A manual
assessment that is not recorded is outstanding, and the contract is not satisfied while any remains outstanding.

Three obligations deserve restating here because they are the ones most easily skipped:

1. **Oracle 1's baseline is captured before the first code change**, from a tracked-clean worktree, with the commit
   recorded, and is never recaptured to resolve a discrepancy.
2. **Oracles 4, 5 and 6 must each be demonstrated to fail.** `VER-MOK-012` acceptance scenarios 4, 5 and 6 require a
   deliberate perturbation, a confirmed failure, and a revert. A check that has never failed has not been shown to
   work, and these three are the checks the whole contract's strength rests on.
3. **Oracle 3 uses a JSON parser this repository does not own.** The engine cannot link one, so its output's validity
   must be established by something other than the code that wrote it.

## Evidence to record

Under `docs/engineering/simulation/evidence/WO-MOK-012/`, everything `VER-MOK-012`'s *Evidence retention* section
lists. In particular:

- `baseline/COMMIT.txt`, the commit the pre-change capture was taken at, with the worktree tracked-clean at capture;
- `capture.sh` and every comparison, reconstruction, replay and census script, retained in full so that every figure
  is reproducible from a recorded command rather than trusted;
- the three-way byte comparison over the declared matrix: pre-change, post-change sinkless, post-change with a sink,
  with exit codes and standard error, and **no projection**;
- one full sink stream per declared seed under each policy with tracing off and on, retained as the primary artifact
  this phase produces;
- `reconstruct.py` and its byte-comparison result for every combination, with the confirmation that it carries no
  event-type-specific branch;
- `replay.py` and its per-tick reconciliation result per seed;
- `json-validity.txt`, the external-parser check with the exact command, including the integer-only assertion;
- `entropy.txt`, the per-tick state comparison with and without a sink, and the state after initialization and at tick
  1,000 against the pre-change build;
- `alphabet.txt`, every domain's members, its size, and the emitted bytes;
- `negative-controls.txt`, the deliberate-perturbation results for oracles 4, 5 and 6;
- `additivity.txt`, the re-run of every configuration retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011`;
- the five failure captures and the overwrite capture, each with standard error, exit code and the destination's state
  afterwards;
- `sizes.txt`, the stream size for a 1,000-tick run and a 10,000-tick traced run;
- `static-checks.txt`, `interface.txt` compared item for item, and the test census before and after reconciled name by
  name;
- `gates.txt`, the four gate commands and their output;
- `manual-assessment.md`, the eight assessments with role and date;
- `amendment-approvals.md`, oracle 7's check, including the recorded state of `ARCH-MOK-001`'s 2026-08-18 row;
- `completion-summary.md`.

No retained sink stream carries the path it was written to, which is what makes this evidence class safe to retain
without redaction.

## Stop and escalate conditions

Stop and escalate rather than proceeding, if:

1. **Oracle 1 shows any difference** in the text stream, the diagnostic stream or the exit code, in any cell of the
   declared matrix, with or without a sink. There is no threshold below which this is acceptable.
2. **Oracle 4 shows any movement** in the entropy state, at any tick, at any seed.
3. **Any retained pre-change capture fails to reproduce** byte for byte.
4. **Any existing test's assertions would have to change**, beyond the mechanical `execute` call sites and the
   extended `USAGE`-versus-parser test.
5. A record cannot be produced without a value outside `SPEC-MOK-006` rule 3.2's enumeration. That means either the
   enumeration is wrong or the field is, and the answer is an escaper with its own verification — which is a
   specification amendment and a scope increase, not an implementation decision.
6. A figure the specification requires cannot be computed from state the engine already holds plus the six counters.
   Adding state beyond the *State model* section is a specification amendment.
7. The library target would have to touch the filesystem, or `execute` would have to grow by more than one parameter,
   or the public interface would have to grow by an item.
8. A metric would be more naturally expressed as a non-integer. The answer is the sum and the count, and if that is
   genuinely insufficient it is a product decision, not an implementation one.
9. A record kind, field or classification not in `SPEC-MOK-006` seems necessary. Useful is not the same as authorized.
10. An entropy draw appears necessary anywhere in a record path.
11. `SPEC-MOK-002` rule 6 turns out to require amendment after all. The design was chosen so that it does not, so
    needing one means the design is wrong rather than the specification is.
12. The amendments this work order names as approval preconditions are not approved when implementation would begin.

## Completion report format

The completion summary records, in this order:

1. what changed, file by file, and what deliberately did not — the text stream, the entropy stream, `AgentSnapshot`,
   `Observation`, the exit codes, the defaults, the observer;
2. one full record stream, quoted, at one declared seed with a short tick limit, so that a reader can see the artifact
   this phase produced without running anything;
3. oracle 1's three-way byte comparison over the whole declared matrix, stated as a count of differing bytes, with the
   explicit statement that no projection was applied;
4. oracle 2's reconstruction result, and the confirmation that the reconstructor has no per-event-type branch;
5. oracle 3's external-parser result, including the integer-only assertion and the exact command;
6. oracle 4's per-tick entropy comparison, and the state figures against the pre-change build;
7. oracle 5's enumeration, with each domain's size beside the specification's;
8. oracle 6's reconciliation, per tick, and the counter comparison against event counts;
9. the negative controls: what was perturbed, which oracle failed, and the confirmation of the revert;
10. the additivity result over every retained pre-change capture;
11. the failure captures, the overwrite capture, and the state of each destination afterwards;
12. the census reconciliation, the public-interface comparison item for item, and the rule 6 re-check;
13. the four gates' output;
14. the three amendment records, each row quoted, with the approval each carries, and the recorded state of
    `ARCH-MOK-001`'s 2026-08-18 row;
15. the eight manual assessments, or an explicit statement of which are outstanding and who owes them;
16. residual uncertainty, including everything under *Stop and escalate conditions* that remains open, everything
    under `VER-MOK-012`'s *Residual uncertainty*, and the statement that `VREC-MOK-012` is a separate commit-bound
    record that this work order does not write and cannot self-approve.
