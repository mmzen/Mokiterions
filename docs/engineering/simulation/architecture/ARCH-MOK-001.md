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
  "REQ-MOK-047",
]
conforms_to = ["SPEC-MOK-001", "SPEC-MOK-002"]

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
rationale = "This architecture establishes the foundation's system boundary as one in-process authoritative engine, fixes the dependency direction so decision sources never reach mutable world state, and defines the immutable observation and typed proposed-action contract as a maintained interface. It treats every decision source as untrusted and keeps future provider credentials outside the engine, selects deterministic single-owner state and seeded entropy as the consistency and reliability strategy, and defers an external model provider to an adapter at the same boundary. ADR-MOK-001 records the accepted alternatives and states that replacing engine authority requires a superseding ADR, making the decision materially difficult to reverse. Amended 2026-08-17: ADR-MOK-002 narrows one binary crate to one Cargo package with a library target and a thin binary target whose public interface is an enumerated read-only contract owned by SPEC-MOK-002, firing the declared public-interface-or-protocol and material-alternatives triggers, engine authority untouched. Amended again 2026-08-17: ADR-MOK-003 scopes this architecture to the engine package rather than the repository and admits a terminal observer as a separate package outside its boundary, governed by ARCH-MOK-002 on the approved requirement REQ-MOK-026, firing the declared system-boundary, responsibility-or-dependency-direction, technology-framework-vendor-or-external-service and material-alternatives triggers, engine authority again untouched. Amended 2026-08-20: ADR-MOK-006 decides the third-party-component policy, withdrawing the engine's empty-dependency-table rule for criteria-based admission against a per-package declared set, firing the declared technology-framework-vendor-or-external-service and material-alternatives triggers and leaving engine authority, dependency direction, the trust boundary and determinism untouched. It stays adr_required, covered by ADR-MOK-001, ADR-MOK-002, ADR-MOK-003 and ADR-MOK-006 together."
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
| 2026-08-18 | Narrowed the prohibition on public items from "mutable **or owned** authoritative state" to a mutable borrow of, or a reference into, that state, and narrowed the matching conformance check the same way. The prohibition as written also forbade the owned, reference-free snapshots `SPEC-MOK-003` specifies, which is the capability `REQ-MOK-019` through `REQ-MOK-025` and `REQ-MOK-027` require and which grants no mutation. Nothing about mutation, dependency direction or determinism is relaxed. | **OUTSTANDING.** Requires the technical owner. It is an approval precondition of `WO-MOK-005`, alongside the four `SPEC-MOK-002` amendments `SPEC-MOK-003` states. It was not part of the 2026-08-17 approval: the wording it narrows reached `master` afterwards, under `WO-MOK-003`. |
| 2026-08-20 | **Withdrew the engine package's empty-dependency-set rule** and replaced it with the declared-set form, decided by `ADR-MOK-006`. The prohibited pattern on network calls, credentials, asynchronous runtimes, databases, UI frameworks, plugin systems and dependency-injection containers keeps its first sentence verbatim — every item in it is preserved without relaxation — and its second sentence becomes: the set is exactly what `SPEC-MOK-002` declares, admission is `ADR-MOK-006`'s, and a crate shared with another workspace package is admissible only as a declared entry of both. The determinism prohibition is **extended** to bind admitted crates and not only first-party code. **A new prohibited pattern** reserves the proprietary core: no declared entry implements simulation semantics, owns or advances entropy, or validates an action. The empty-graph conformance check **splits in two** — set comparison at the declared versions and features, and a by-name scan for the prohibited capability classes — because an empty graph implied the second and a declared graph does not. *"And an empty dependency table"* is struck from the target-shape check, which gains offline resolution from the committed lockfile; **a new conformance check** states the review that decision 11 requires, retained under `VER-MOK-013`. *Quality attributes* gains registry independence and the declared-dependency-surface property, neither of which this list previously mentioned. `REQ-MOK-047` added to `addresses`, as `REQ-MOK-016` was added in the 2026-08-17 row. `decision_assessment.rationale` records `ADR-MOK-006` and its triggers; **the field stood at 1903 of the validator's 2000-character cap, so the two 2026-08-17 sentences were tightened to make room** — every claim each made is preserved, and the rows above are where the full narrative lives. No engine boundary, dependency direction, trust boundary or determinism property is relaxed: what is relaxed is a *count*, not a kind. | Approved 2026-08-20 by the repository owner acting as accountable technical owner, by way of `ADR-MOK-006`, whose *Required amendments* section states this amendment in full. Written under `WO-MOK-013`; the implementation agent wrote the text and did not decide it. **The 2026-08-18 row above stays OUTSTANDING and was not touched**: it is `WO-MOK-005`'s precondition, and this approval neither clears nor inherits it. |

## Context and scope

The minimum foundation is a local Rust command-line program. It must establish an authoritative deterministic engine and a replaceable decision boundary without importing the complexity of a service architecture, UI, persistence layer, or external model client.

This architecture addresses the four requirement drivers that materially shape its boundaries rather than every requirement it conforms to: world authority (`REQ-MOK-004`), the in-process baseline decision source (`REQ-MOK-008`), reproducible entropy (`REQ-MOK-009`), and the text observation interface (`REQ-MOK-010`). The remaining foundation requirements are domain rules whose detailed behavior is governed by `SPEC-MOK-001`, which this architecture conforms to.

**Boundary of this architecture, as amended.** This architecture governs the **simulation engine package** and the command-line binary it produces: the components in the next section, the `SPEC-MOK-001` behavior they implement, and the `REQ-MOK-010` text stream. It does not govern the terminal observer package, which is a separate component boundary governed by `ARCH-MOK-002`. Every rule below is read as a rule about the engine package unless it explicitly says otherwise. That reading narrows nothing: each rule previously applied to a repository containing only the engine, and the engine package is exactly that scope. What the amendment removes is the implication that no other package may exist — an implication the prohibition on separate crates already qualified with "without an approved requirement", now satisfied by `REQ-MOK-026`.

## Components and responsibilities

1. **Application entry point** parses arguments, constructs the run, applies the optional action-trace output policy, streams formatted events, maps errors to exit codes, and owns process termination.
2. **Simulation engine** owns the world, agents, food, tick, entropy state, validation, rule application, event creation, termination, and summary.
3. **Decision boundary** defines read-only observations and proposed core actions. Its only foundation implementation is the local seeded baseline.

These responsibilities may be represented by a small number of Rust modules in one Cargo package — the engine package — built as a library target and a thin binary target. They are logical boundaries, not a requirement to create a separate crate or framework for each component.

The application entry point is the binary target and stays thin: process startup, stream buffering, one call into the library target, flush handling, and exit-code mapping. The simulation engine and the decision boundary live in the library target, whose public interface is enumerated by `SPEC-MOK-002` rule 5 and exposes no mutable authoritative state.

The library target additionally exposes a read-only observation surface, specified by `SPEC-MOK-003` and admitted into rule 5's enumeration by the amendment that specification requires, through which a host process obtains an owned snapshot of authoritative state and advances the simulation one tick. That surface is a fourth responsibility of the engine package and not a fourth component: it mutates nothing, decides nothing, and grants no handle. The command-line binary and the observer are both hosts of it.

## Dependency direction

- The application entry point may depend on the simulation engine and baseline decision implementation.
- The baseline may depend on immutable observation and proposed-action types exposed by the simulation boundary.
- The simulation engine must not depend on a concrete baseline, OpenAI client, terminal UI, database, or web framework. As amended, this is enforced by the package boundary rather than by convention: the engine package cannot depend on the observer package, and the observer package holds every user-interface dependency.
- No component depends on network or persistence infrastructure.

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
```

The engine owns the event order and summary facts. Formatting does not mutate simulation state.

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

## Quality attributes

- **Simplicity:** one engine package with one library target and one thin binary target, and the minimum modules needed to make authority boundaries legible, plus at most the one further package `REQ-MOK-026` approves.
- **Determinism:** the same inputs produce byte-identical events and final state.
- **Testability:** rules can be tested through engine inputs and observable outputs without launching external systems, and the program's public contract can be tested from outside the implementation source files.
- **Debuggability:** optional action tracing exposes every decision opportunity without altering engine behavior.
- **Extensibility at one seam:** a future OpenAI-backed source can implement the decision boundary without gaining mutation access.
- **Safety:** bounded values cannot wrap and invalid actions cannot partially apply.
- **Registry independence:** the engine package resolves, builds and tests from the committed lockfile in an environment with no package registry access. This was a side effect of having no dependencies and is now an attribute the architecture states, because `ADR-MOK-006` admits dependencies that could take it away.
- **A declared dependency surface:** each package's external dependency set is enumerated in a specification, so the surface is a reviewed declaration compared against the resolved graph rather than a consequence of whatever resolution produces. What the engine contains is answerable by reading one table and comparing it, in either direction.

## Conformance checks

- Review dependency direction and public APIs for mutable-state leakage.
- Confirm the **engine package's** resolved dependency graph equals the set `SPEC-MOK-002` declares for it, at the declared versions and the declared feature sets, with no undeclared entry and no declared entry missing. Resolve the graph per package rather than for the workspace, since a workspace graph containing the observer would not answer the question.
- Confirm by name that the engine package's resolved graph contains no network, asynchronous-runtime, database, model-provider or user-interface crate. This was previously an inference from the graph being empty; a declared graph does not carry it, so it is checked directly. The check is by name and is not exhaustive over those capability classes, which is why `VER-MOK-013` retains a review beside it.
- Confirm the engine package does not depend on the observer package, directly or transitively.
- Confirm the engine package's read-only observation surface exposes no mutating operation other than the single-tick advance, and returns owned values with no reference into engine state.
- Run deterministic replay tests and invalid-action atomicity tests.
- Confirm all stochastic code is reachable from the explicit seeded entropy owner.
- Confirm the engine builds as one Cargo package with exactly one library target and one binary target, that it resolves, builds and tests from the committed lockfile with no registry access, and that its tests run independently of the observer package and with no terminal present.
- Review each crate in the engine package's declared set against what it supplies, and confirm that no declared entry implements simulation semantics, owns or advances entropy, or validates an action. This is stated as a review because no graph read answers it; `VER-MOK-013` is where the assessment is retained, and an assessment that has not been made leaves that contract unsatisfied.
- Confirm the library target's public interface matches `SPEC-MOK-002` rule 5 as amended exactly, and that no public item yields a mutable borrow of, or a reference into, authoritative state.

## Related architecture and ADRs

- `ADR-MOK-001` records the engine-authoritative in-process decision boundary and its consequences for future model integration. It remains accepted and is not superseded: it requires a superseding ADR only for replacing engine authority, and a read-only observer replaces none of it.
- `ADR-MOK-002` records the library target with an enumerated read-only public interface, and the test placement rule that follows from it.
- `ARCH-MOK-002` governs the terminal observer package, outside this architecture's boundary.
- `ADR-MOK-003` decides the two-package split and the user-interface dependency, and is the deciding ADR for `ARCH-MOK-002` and for the third and fourth amendments recorded above.
- `ADR-MOK-006` decides the repository's third-party-component policy: it withdraws this architecture's empty-dependency-set rule, admits crates in both packages on the repository owner's stated criteria, and requires every admitted crate to be a declared entry of its package's set in a specification. It is the deciding ADR for the 2026-08-20 amendment recorded above, it decides `ARCH-MOK-002` as well, and it supersedes neither `ADR-MOK-001` nor `ADR-MOK-003` — it reverses `ADR-MOK-003` decision 4 and the word *"only"* in its decision 5, which that ADR's *Status* section records.
