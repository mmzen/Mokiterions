+++
id = "ARCH-MOK-001"
type = "architecture"
title = "Single-process simulation architecture"
status = "approved"
owners = ["technical owner"]
created = "2026-08-11"
updated = "2026-08-20"

[relations]
addresses = [
  "REQ-MOK-004",
  "REQ-MOK-008",
  "REQ-MOK-009",
  "REQ-MOK-010",
  "REQ-MOK-016",
  "REQ-MOK-042",
  "REQ-MOK-045",
  "REQ-MOK-050",
]
conforms_to = ["SPEC-MOK-001", "SPEC-MOK-002", "SPEC-MOK-006"]

[decision_assessment]
outcome = "adr_required"
triggers = [
  "system-boundary",
  "responsibility-or-dependency-direction",
  "public-interface-or-protocol",
  "security-privacy-or-trust-boundary",
  "concurrency-consistency-reliability-or-failure-strategy",
  "technology-framework-vendor-or-external-service",
  "difficult-to-reverse",
  "material-alternatives",
]
rationale = "This architecture establishes the foundation's system boundary as one in-process authoritative engine, fixes the dependency direction so decision sources never reach mutable world state, and defines the immutable observation and typed proposed-action contract as a maintained interface. It treats every decision source as untrusted and keeps future provider credentials outside the engine, selects deterministic single-owner state and seeded entropy as the consistency and reliability strategy, and defers an external model provider to an adapter at the same boundary. ADR-MOK-001 records the accepted alternatives and states that replacing engine authority requires a superseding ADR, making the decision materially difficult to reverse. Amended 2026-08-17: ADR-MOK-002 narrows one binary crate to one Cargo package with a library target and a thin binary target whose public interface is an enumerated read-only contract owned by SPEC-MOK-002, firing the declared public-interface-or-protocol and material-alternatives triggers, engine authority untouched. Amended again 2026-08-17: ADR-MOK-003 scopes this architecture to the engine package rather than the repository and admits a terminal observer as a separate package outside its boundary, governed by ARCH-MOK-002 on the approved requirement REQ-MOK-026, firing the declared system-boundary, responsibility-or-dependency-direction, technology-framework-vendor-or-external-service and material-alternatives triggers, engine authority again untouched. Amended 2026-08-20: ADR-MOK-006 decides the third-party-component policy, withdrawing the engine's empty-dependency-table rule for criteria-based admission against a per-package declared set, firing the declared technology-framework-vendor-or-external-service and material-alternatives triggers and leaving engine authority, dependency direction, the trust boundary and determinism untouched. It stays adr_required, covered by ADR-MOK-001 through ADR-MOK-006 together."
assessed_by = "technical owner"
+++

# Architecture: Single-process simulation architecture

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-11 | Original approved content for `CAP-MOK-001`. | Approved; implemented under `WO-MOK-001` and verified under `VREC-MOK-001`. |
| 2026-08-17 | Schema migration to typed `addresses` and `conforms_to` relations and an explicit `decision_assessment`. | technical owner |
| 2026-08-17 | One binary crate narrowed to one Cargo package built as a library target and a thin binary target; prohibited-pattern, quality-attribute, and conformance-check wording updated; `REQ-MOK-016` added to `addresses` and `SPEC-MOK-002` to `conforms_to`. Decided by `ADR-MOK-002`; engine authority unchanged. | repository owner, on the technical owner's behalf |
| 2026-08-17 | Scoped this architecture to the **simulation engine package** rather than to the repository. The one-binary-crate rule, the empty-dependency-graph rule, and the prohibition on UI frameworks now bind the engine package specifically, where they are stronger than before because they became checkable per package. A terminal observer is admitted as a separate package outside this architecture's boundary, governed by `ARCH-MOK-002` and decided by `ADR-MOK-003`, on the approved requirement `REQ-MOK-026` that the prohibition on separate crates always required. No engine boundary, dependency direction, trust boundary, or determinism property is relaxed. | Approved by the technical owner on 2026-08-17, together with `ARCH-MOK-002`, `ADR-MOK-003`, and `REQ-MOK-026`. |
| 2026-08-18 | Narrowed the prohibition on public items from "mutable **or owned** authoritative state" to a mutable borrow of, or a reference into, that state, and narrowed the matching conformance check the same way. The prohibition as written also forbade the owned, reference-free snapshots `SPEC-MOK-003` specifies, which is the capability `REQ-MOK-019` through `REQ-MOK-025` and `REQ-MOK-027` require and which grants no mutation. Nothing about mutation, dependency direction or determinism is relaxed. | **Ratified 2026-08-20 by the repository owner acting as technical owner**, as written and without modification, in the assessment review recorded under `WO-MOK-012`. It was **OUTSTANDING** from 2026-08-18 until that act. It was an approval precondition of `WO-MOK-005`, alongside the four `SPEC-MOK-002` amendments `SPEC-MOK-003` states, and it was not part of the 2026-08-17 approval: the wording it narrows reached `master` afterwards, under `WO-MOK-003`. This is the provision that makes the shipped observer legal, and the owner was shown it in those terms — that the prohibition as originally worded forbade the owned, reference-free snapshots the observer is built on, so ratifying it authorizes what is already in the tree rather than permitting something new. The narrowing grants no mutation: no public item yields a mutable borrow of, or a reference into, authoritative state in any build configuration. The implementation agent wrote this text and decided none of the substance. `VREC-MOK-001`, which binds this architecture's original content, is not edited. |
| 2026-08-20 | **Withdrew the engine package's empty-dependency-set rule** and replaced it with the declared-set form, decided by `ADR-MOK-006`. The prohibited pattern on network calls, credentials, asynchronous runtimes, databases, UI frameworks, plugin systems and dependency-injection containers keeps its first sentence verbatim — every item in it is preserved without relaxation — and its second sentence becomes: the set is exactly what `SPEC-MOK-002` declares, admission is `ADR-MOK-006`'s, and a crate shared with another workspace package is admissible only as a declared entry of both. The determinism prohibition is **extended** to bind admitted crates and not only first-party code. **A new prohibited pattern** reserves the proprietary core: no declared entry implements simulation semantics, owns or advances entropy, or validates an action. The empty-graph conformance check **splits in two** — set comparison at the declared versions and features, and a by-name scan for the prohibited capability classes — because an empty graph implied the second and a declared graph does not. *"And an empty dependency table"* is struck from the target-shape check, which gains offline resolution from the committed lockfile; **a new conformance check** states the review that decision 11 requires, retained under `VER-MOK-014`. *Quality attributes* gains registry independence and the declared-dependency-surface property, neither of which this list previously mentioned. `REQ-MOK-050` added to `addresses`, as `REQ-MOK-016` was added in the 2026-08-17 row. `decision_assessment.rationale` records `ADR-MOK-006` and its triggers; **the field stood at 1903 of the validator's 2000-character cap, so the two 2026-08-17 sentences were tightened to make room** — every claim each made is preserved, and the rows above are where the full narrative lives. No engine boundary, dependency direction, trust boundary or determinism property is relaxed: what is relaxed is a *count*, not a kind. | Approved 2026-08-20 by the repository owner acting as accountable technical owner, by way of `ADR-MOK-006`, whose *Required amendments* section states this amendment in full. Written under `WO-MOK-014`; the implementation agent wrote the text and did not decide it. **The 2026-08-18 row above was OUTSTANDING when this row was written and was not touched**: it was `WO-MOK-005`'s precondition, and this approval neither cleared nor inherited it. The repository owner acting as technical owner ratified it as written on 2026-08-20, in the assessment review recorded under `WO-MOK-012`, which reached this branch by merge after this row was written. The sentence is moved to the past tense under that work order's own rule that a later row asserting a provision remains OUTSTANDING is corrected and names the ratifying act; it is a statement of fact about another row's status, and no obligation changes either way. |
| 2026-08-20 | The optional structured record stream, under `CAP-MOK-009`. Twelve provisions. **Components** item 1 gains the entry point's sink duties — resolving the path, creating and truncating the destination, supplying the buffered writer, flushing and closing it, and removing a file it created on failure. Item 2 gains the engine's ownership of the run's cumulative measurement counters and of record production from authoritative events and state. The observation-surface paragraph gains a second host-supplied surface: the record sink is a **fifth responsibility of the engine package and not a fourth component**, and the **library target performs no filesystem operation**. **Dependency direction**'s fourth bullet distinguishes an output destination from persistence of state — nothing is read back and no state survives the process in a form the engine consumes. **Data and control flow** gains the second, optional branch `ordered event -> record projection -> host-supplied sink`, with the text branch unchanged and unconditional, and records that the projection mutates no state and draws no entropy. **Prohibited patterns** gains three: no filesystem operation in the library target, no entropy draw from a record-writing path, and no operator-supplied, environment-derived or free-text field in the stream while `SPEC-MOK-006` rule 3.3 is the totality argument for its escaping. **Determinism** extends to the record stream and states that configuring a sink moves neither the text bytes nor the draw sequence. **Debuggability** extends from action tracing to structured recording. **Conformance checks** gains four: no filesystem operation in the library target checked against its source; identical text bytes with and without a sink across every declared seed, each policy and tracing off and on; an identical per-tick entropy draw sequence; and every string-valued field a member of `SPEC-MOK-006` rule 3.2's enumeration, checked exhaustively. **Related architecture and ADRs** gains `ADR-MOK-005`. `addresses` gains `REQ-MOK-042` and `REQ-MOK-045`; `conforms_to` gains `SPEC-MOK-006`. `decision_assessment.rationale` records the three decisions `ADR-MOK-005` makes, the triggers they fire, and that the assessment stays `adr_required`. **No engine boundary, trust boundary, dependency direction, required pattern or determinism property is relaxed, and this amendment adds no entry to the engine package's declared dependency set, which is still empty.** This sentence originally read "the engine package's dependency table stays empty", invoking a rule of this architecture that `ADR-MOK-006` withdrew on the same date for the declared-set form; the superseded wording is recorded rather than deleted, and the `ADR-MOK-006` row above is the authority. | Approved 2026-08-20 by the repository owner acting as technical owner, together with `INT-MOK-009`, `CAP-MOK-009`, `REQ-MOK-042` through `REQ-MOK-046`, `SPEC-MOK-006`, `VER-MOK-012` and `WO-MOK-018`, and by way of `ADR-MOK-005`, which the same owner accepted on the same date and whose *Required amendments* section states all twelve in full. The implementation agent wrote the amended text under `WO-MOK-018`; it did not decide the substance. **The 2026-08-18 row above was OUTSTANDING when this row was written and was not touched.** It belonged to `WO-MOK-005` and awaited the same owner's separate act, and nothing in this amendment depended on it: `ADR-MOK-005` records that the prohibition this chain adds to is the one already on the page, and `VER-MOK-012` oracle 7 measured the state as it stood rather than as it would stand once that row was resolved. The repository owner acting as technical owner ratified that row as written on 2026-08-20, in the assessment review recorded under `WO-MOK-012`, which reached this branch by merge after this row was written. The sentence is moved to the past tense under that work order's own rule that a later row asserting a provision remains OUTSTANDING is corrected and names the ratifying act; this approval neither cleared nor inherited it. |

## Context and scope

The minimum foundation is a local Rust command-line program. It must establish an authoritative deterministic engine and a replaceable decision boundary without importing the complexity of a service architecture, UI, persistence layer, or external model client.

This architecture addresses the four requirement drivers that materially shape its boundaries rather than every requirement it conforms to: world authority (`REQ-MOK-004`), the in-process baseline decision source (`REQ-MOK-008`), reproducible entropy (`REQ-MOK-009`), and the text observation interface (`REQ-MOK-010`). The remaining foundation requirements are domain rules whose detailed behavior is governed by `SPEC-MOK-001`, which this architecture conforms to.

**Boundary of this architecture, as amended.** This architecture governs the **simulation engine package** and the command-line binary it produces: the components in the next section, the `SPEC-MOK-001` behavior they implement, and the `REQ-MOK-010` text stream. It does not govern the terminal observer package, which is a separate component boundary governed by `ARCH-MOK-002`. Every rule below is read as a rule about the engine package unless it explicitly says otherwise. That reading narrows nothing: each rule previously applied to a repository containing only the engine, and the engine package is exactly that scope. What the amendment removes is the implication that no other package may exist — an implication the prohibition on separate crates already qualified with "without an approved requirement", now satisfied by `REQ-MOK-026`.

## Components and responsibilities

1. **Application entry point** parses arguments, constructs the run, applies the optional action-trace output policy, streams formatted events, maps errors to exit codes, and owns process termination. Amended 2026-08-20 for `REQ-MOK-042`: it additionally resolves the optional record sink's path, creates and truncates the destination, supplies the buffered writer the engine writes records to, flushes and closes it, and removes a file it created when the run fails. Every one of those duties is the entry point's alone.
2. **Simulation engine** owns the world, agents, food, tick, entropy state, validation, rule application, event creation, termination, and summary. Amended 2026-08-20 for `REQ-MOK-042` and `REQ-MOK-043`: it additionally owns the run's cumulative measurement counters and the production of structured records from authoritative events and state. Records are produced by the owner of the facts they state, so no consumer has to re-derive a figure the engine already holds.
3. **Decision boundary** defines read-only observations and proposed core actions. Its only foundation implementation is the local seeded baseline.

These responsibilities may be represented by a small number of Rust modules in one Cargo package — the engine package — built as a library target and a thin binary target. They are logical boundaries, not a requirement to create a separate crate or framework for each component.

The application entry point is the binary target and stays thin: process startup, stream buffering, one call into the library target, flush handling, and exit-code mapping. The simulation engine and the decision boundary live in the library target, whose public interface is enumerated by `SPEC-MOK-002` rule 5 and exposes no mutable authoritative state.

The library target additionally exposes a read-only observation surface, specified by `SPEC-MOK-003` and admitted into rule 5's enumeration by the amendment that specification requires, through which a host process obtains an owned snapshot of authoritative state and advances the simulation one tick. That surface is a fourth responsibility of the engine package and not a fourth component: it mutates nothing, decides nothing, and grants no handle. The command-line binary and the observer are both hosts of it.

Amended 2026-08-20 for `REQ-MOK-042`: the library target additionally accepts a **host-supplied record sink** — a writer the caller owns, passed in as one optional parameter on the process-boundary function `SPEC-MOK-002` rule 4 enumerates. Writing structured records to it is a **fifth responsibility of the engine package and not a fourth component**, for the same reasons the observation surface is not: it mutates nothing, decides nothing, draws no entropy and grants no handle into engine state. **The library target performs no filesystem operation.** It does not open, create, truncate, remove, stat or name a file, and it never sees a path — the sink reaches it already open. That is what confines the one operator-supplied value that is interpreted as a filesystem path to the binary target, and it is checked against the library target's source rather than asserted.

## Dependency direction

- The application entry point may depend on the simulation engine and baseline decision implementation.
- The baseline may depend on immutable observation and proposed-action types exposed by the simulation boundary.
- The simulation engine must not depend on a concrete baseline, OpenAI client, terminal UI, database, or web framework. As amended, this is enforced by the package boundary rather than by convention: the engine package cannot depend on the observer package, and the observer package holds every user-interface dependency.
- No component depends on network infrastructure or on a persistence layer, database or index. Amended 2026-08-20 for `REQ-MOK-042`: the binary target's creation of one operator-named output file, at the operator's instruction, is an **output destination and not persistence of state**. Nothing is read back, no schema is owned, no query is issued, and no state survives the process in a form the engine consumes. A run that names a sink is as stateless on its next start as a run that does not.

## Data and control flow

```text
CLI configuration
      |
      v
Simulation engine --creates--> immutable observation
      ^                              |
      |                              v
validates and applies <------- proposed action
      |
      v
ordered event -> text formatter -> standard output
      |
      v
ordered event -> record projection -> host-supplied sink
```

The engine owns the event order and summary facts. Formatting does not mutate simulation state.

Amended 2026-08-20 for `REQ-MOK-042` and `REQ-MOK-044`: the second branch is **optional**, taken only when the host supplies a sink. The text branch is **unchanged and unconditional** — it is drawn first and it runs whether or not the second one does. Both branches read the same ordered event, so neither can observe a state the other does not. The record projection **mutates no simulation state and draws no entropy**: it reads the event, the counters the engine already maintains and the state the text stream already reports, and it writes. That is why configuring a sink cannot move a run.

## Trust boundaries

The decision boundary is untrusted with respect to world authority. All returned actions are validated even when produced by the bundled baseline. CLI input is also untrusted and is validated before engine construction.

## Required patterns

- One authoritative owner for mutable simulation state.
- Explicit immutable observation and proposed-action values at the decision boundary.
- One explicit seeded entropy state threaded through all stochastic operations.
- Stable ordering before any collection contents affect decisions or output.
- Saturating or checked arithmetic for bounded attributes.
- Ordinary Rust `Result` propagation for recoverable process errors.

## Prohibited patterns

- Mutable world references, callbacks, or engine handles exposed to decision sources.
- Global mutable simulation or entropy state.
- Wall-clock time, operating-system randomness, unordered iteration, or thread scheduling affecting results. This binds every crate in the engine package's declared set as well as first-party code: an admitted crate may not draw entropy, read wall-clock time, read the environment, or introduce iteration-order nondeterminism into any value the `REQ-MOK-010` stream, the authoritative event sequence or the final state observes, and where a crate offers such a capability behind a feature, the feature is off and its absence is part of the declared set.
- Network calls, API credentials, asynchronous runtimes, databases, UI frameworks, plugin systems, or dependency injection containers in the engine package. The engine package's external dependency set is exactly the set declared for it in `SPEC-MOK-002`; admission is governed by `ADR-MOK-006`, and a dependency shared with another package in the same workspace is admissible only as a declared entry of both.
- Crates in the engine package's declared set that implement simulation semantics — the rules `SPEC-MOK-001` fixes, the world model, or agent decision-making — or that own or advance entropy, or that perform action validation. A crate supplies standard functionality the simulation uses; it does not supply the simulation. This sits beside the prohibition on separate packages and services below: that one reserves *structure* to approved requirements, and this one reserves *substance* the same way.
- Panics for invalid operator input or invalid proposed actions.
- Separate Cargo packages, workspaces, or services without an approved requirement. The library and binary targets of the single engine package are not separate crates in this sense. `REQ-MOK-026` is the approved requirement for exactly one further package, the terminal observer; it authorizes no service, no network boundary, no separate release artifact, and no third package.
- Public items that expose a mutable borrow of, or a reference into, authoritative state, and any feature flag, `cfg` attribute, or test-support seam that exposes such access outside the crate. Owned, reference-free copies of already-reported state are admitted only where `SPEC-MOK-003` specifies them and `SPEC-MOK-002` rule 5 enumerates them.
- Added 2026-08-20 for `REQ-MOK-042` and `REQ-MOK-045`: **any filesystem operation in the engine package's library target.** Opening, creating, truncating, removing, renaming, copying, stating or listing a path, and reading a path from the environment, are the binary target's alone. The library target takes writers and returns results.
- Added 2026-08-20 for `REQ-MOK-044`: **any draw against the entropy stream from a record-writing path.** No record kind, no field, no counter and no failure path may consume a draw, because a run that recorded would then diverge from the same run that did not.
- Added 2026-08-20 for `REQ-MOK-045`: **any field in the record stream whose value is operator-supplied, environment-derived or free text**, while `SPEC-MOK-006` rule 3.3 is the totality argument for the stream's escaping. The escaping is provably total only because the value alphabet is closed; a single free-text field would withdraw the proof rather than weaken it, and rule 3.4 is what obliges a future requirement that needs one to pay for a general escaper first.

## Quality attributes

- **Simplicity:** one engine package with one library target and one thin binary target, and the minimum modules needed to make authority boundaries legible, plus at most the one further package `REQ-MOK-026` approves.
- **Determinism:** the same inputs produce byte-identical events, byte-identical structured records and final state. Amended 2026-08-20 for `REQ-MOK-044`: **configuring a sink changes neither the text stream's bytes nor the entropy draw sequence.** The record stream is an additional output of the same run, not a second run.
- **Testability:** rules can be tested through engine inputs and observable outputs without launching external systems, and the program's public contract can be tested from outside the implementation source files.
- **Debuggability:** optional action tracing exposes every decision opportunity without altering engine behavior. Amended 2026-08-20 for `REQ-MOK-042`: optional structured recording likewise exposes a run's facts — its resolved configuration, every ordered event, per-tick measurements and the run's cumulative figures — without altering engine behavior. Both are opt-in, both are additive, and neither is a mode the program runs differently in.
- **Extensibility at one seam:** a future OpenAI-backed source can implement the decision boundary without gaining mutation access.
- **Safety:** bounded values cannot wrap and invalid actions cannot partially apply.
- **Registry independence:** the engine package resolves, builds and tests from the committed lockfile in an environment with no package registry access. This was a side effect of having no dependencies and is now an attribute the architecture states, because `ADR-MOK-006` admits dependencies that could take it away.
- **A declared dependency surface:** each package's external dependency set is enumerated in a specification, so the surface is a reviewed declaration compared against the resolved graph rather than a consequence of whatever resolution produces. What the engine contains is answerable by reading one table and comparing it, in either direction.

## Conformance checks

- Review dependency direction and public APIs for mutable-state leakage.
- Confirm the **engine package's** resolved dependency graph equals the set `SPEC-MOK-002` declares for it, at the declared versions and the declared feature sets, with no undeclared entry and no declared entry missing. Resolve the graph per package rather than for the workspace, since a workspace graph containing the observer would not answer the question.
- Confirm by name that the engine package's resolved graph contains no network, asynchronous-runtime, database, model-provider or user-interface crate. This was previously an inference from the graph being empty; a declared graph does not carry it, so it is checked directly. The check is by name and is not exhaustive over those capability classes, which is why `VER-MOK-014` retains a review beside it.
- Confirm the engine package does not depend on the observer package, directly or transitively.
- Confirm the engine package's read-only observation surface exposes no mutating operation other than the single-tick advance, and returns owned values with no reference into engine state.
- Run deterministic replay tests and invalid-action atomicity tests.
- Confirm all stochastic code is reachable from the explicit seeded entropy owner.
- Confirm the engine builds as one Cargo package with exactly one library target and one binary target, that it resolves, builds and tests from the committed lockfile with no registry access, and that its tests run independently of the observer package and with no terminal present.
- Review each crate in the engine package's declared set against what it supplies, and confirm that no declared entry implements simulation semantics, owns or advances entropy, or validates an action. This is stated as a review because no graph read answers it; `VER-MOK-014` is where the assessment is retained, and an assessment that has not been made leaves that contract unsatisfied.
- Confirm the library target's public interface matches `SPEC-MOK-002` rule 5 as amended exactly, and that no public item yields a mutable borrow of, or a reference into, authoritative state.
- Added 2026-08-20: confirm the engine package's **library target performs no filesystem operation**, checked against its source rather than asserted — no `std::fs`, no `File`, no `OpenOptions`, no `remove_file`, no path type constructed from an argument, in any build configuration including tests.
- Added 2026-08-20: confirm the **text stream's bytes are identical with and without a sink**, at every declared seed, each policy, and tracing off and on. The comparison is byte-exact over the whole stream, with no whitespace or line-ending exemption.
- Added 2026-08-20: confirm the **per-tick entropy draw sequence is identical with and without a sink**. Equal final state is not sufficient: two draw sequences can end in the same place, so the check reads the draw count per tick.
- Added 2026-08-20: confirm **every string-valued field in the record stream is a member of `SPEC-MOK-006` rule 3.2's enumeration**, checked exhaustively over the closed sets the engine can emit rather than over a sample of one run.

## Related architecture and ADRs

- `ADR-MOK-001` records the engine-authoritative in-process decision boundary and its consequences for future model integration. It remains accepted and is not superseded: it requires a superseding ADR only for replacing engine authority, and a read-only observer replaces none of it.
- `ADR-MOK-002` records the library target with an enumerated read-only public interface, and the test placement rule that follows from it.
- `ARCH-MOK-002` governs the terminal observer package, outside this architecture's boundary.
- `ADR-MOK-003` decides the two-package split and the user-interface dependency, and is the deciding ADR for `ARCH-MOK-002` and for the third and fourth amendments recorded above.
- `ADR-MOK-005` decides the record sink's location — the binary target resolves and owns the file, the library target writes to a writer it is handed — the closed value alphabet that makes hand-written serialization provably total against an empty declared dependency set, and the extent of the public interface's growth, which is one optional parameter and no item. It is the deciding ADR for the 2026-08-20 amendment recorded above. **It supersedes nothing.** Engine authority, the trust boundary, the dependency direction and every determinism property stand as `ADR-MOK-001` records them.
- `ADR-MOK-006` decides the repository's third-party-component policy: it withdraws this architecture's empty-dependency-set rule, admits crates in both packages on the repository owner's stated criteria, and requires every admitted crate to be a declared entry of its package's set in a specification. It is the deciding ADR for the 2026-08-20 amendment recorded above, it decides `ARCH-MOK-002` as well, and it supersedes neither `ADR-MOK-001` nor `ADR-MOK-003` — it reverses `ADR-MOK-003` decision 4 and the word *"only"* in its decision 5, which that ADR's *Status* section records.

## Decision assessment

The `decision_assessment` block in this document's frontmatter carries the assessment. Its `rationale` field has a governed length limit of 2,000 characters and stood at **1997** characters once the 2026-08-20 `ADR-MOK-006` amendment above had taken its room, so the 2026-08-20 `ADR-MOK-005` paragraph did not fit inside what remained — nor did any pointer to it, the shortest admissible form of which overflowed the cap by thirteen characters. **The field therefore names `ADR-MOK-005` by range rather than by sentence, and this section carries its paragraph in full.** The closing clause "covered by `ADR-MOK-001`, `ADR-MOK-002`, `ADR-MOK-003` and `ADR-MOK-006` together" became "covered by `ADR-MOK-001` through `ADR-MOK-006` together", which is twenty-two characters shorter, brings the field to **1975**, and covers `ADR-MOK-005` by containing it. Nothing else in the field was removed, shortened or moved: `ADR-MOK-006`'s paragraph stays in the field it was approved into, byte for byte. **This form was chosen by the repository owner on 2026-08-21**, over relocating both 2026-08-20 paragraphs to this section, which would have left 405 characters of headroom but would have moved 490 already-approved characters out of the field. The range form was preferred because it edits one clause of approved text rather than relocating a paragraph of it. This is the only respect in which the amendment departs from the instruction to append the paragraph to the field itself.

**Amended 2026-08-20.** The location of the record sink, the hand-written serialization strategy and the extent of the public interface's growth are decided by `ADR-MOK-005`. The binary target resolves and owns the destination file, while the library target writes to a writer it is handed and performs no filesystem operation. Records are serialized by hand against a closed value alphabet rather than by a dependency, because the engine package's declared dependency set is empty and this chain sought no amendment to it under `ADR-MOK-006`. The interface grows by exactly one optional parameter on the process-boundary function and by no item. That decision fires the already-declared **public-interface-or-protocol**, **technology-framework-vendor-or-external-service** and **material-alternatives** triggers. Engine authority, the trust boundary, the dependency direction and the determinism properties are untouched — the record projection mutates no state and draws no entropy — so the assessment stays `adr_required` and is covered by `ADR-MOK-001` through `ADR-MOK-006` together.
