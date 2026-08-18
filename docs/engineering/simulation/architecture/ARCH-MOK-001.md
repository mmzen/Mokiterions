+++
id = "ARCH-MOK-001"
type = "architecture"
title = "Single-process simulation architecture"
status = "approved"
owners = ["technical owner"]
created = "2026-08-11"
updated = "2026-08-18"

[relations]
addresses = [
  "REQ-MOK-004",
  "REQ-MOK-008",
  "REQ-MOK-009",
  "REQ-MOK-010",
  "REQ-MOK-016",
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
rationale = "This architecture establishes the foundation's system boundary as one in-process authoritative engine, fixes the dependency direction so decision sources never reach mutable world state, and defines the immutable observation and typed proposed-action contract as a maintained interface. It treats every decision source as untrusted and keeps future provider credentials outside the engine, selects deterministic single-owner state and seeded entropy as the consistency and reliability strategy, and defers an external model provider to an adapter at the same boundary. ADR-MOK-001 records the accepted alternatives and states that replacing engine authority requires a superseding ADR, which makes the decision materially difficult to reverse. Amended 2026-08-17: the target shape of the program is decided by ADR-MOK-002, which narrows one binary crate to one Cargo package built as a library target and a thin binary target, and makes the library target's public interface an enumerated read-only contract owned by SPEC-MOK-002. That decision fires the already-declared public-interface-or-protocol and material-alternatives triggers and leaves engine authority untouched, so this assessment stays adr_required and is now covered by ADR-MOK-001 and ADR-MOK-002 together. Amended 2026-08-17 a second time: this architecture is scoped to the simulation engine package rather than to the repository, and a terminal observer is admitted as a separate package outside its boundary, governed by ARCH-MOK-002 and decided by ADR-MOK-003 on the approved requirement REQ-MOK-026. That decision fires the already-declared system-boundary, responsibility-or-dependency-direction, technology-framework-vendor-or-external-service and material-alternatives triggers and again leaves engine authority untouched, so the assessment stays adr_required and is covered by ADR-MOK-001, ADR-MOK-002 and ADR-MOK-003 together."
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
- Wall-clock time, operating-system randomness, unordered iteration, or thread scheduling affecting results.
- Network calls, API credentials, asynchronous runtimes, databases, UI frameworks, plugin systems, or dependency injection containers in the engine package. The engine package's external dependency set is empty and admits no exception, including a dependency shared with another package in the same workspace.
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

## Conformance checks

- Review dependency direction and public APIs for mutable-state leakage.
- Confirm the **engine package's** dependency graph is empty, and therefore contains no network, async-runtime, database, or UI libraries. Resolve the graph per package rather than for the workspace, since a workspace graph containing the observer would not answer the question.
- Confirm the engine package does not depend on the observer package, directly or transitively.
- Confirm the engine package's read-only observation surface exposes no mutating operation other than the single-tick advance, and returns owned values with no reference into engine state.
- Run deterministic replay tests and invalid-action atomicity tests.
- Confirm all stochastic code is reachable from the explicit seeded entropy owner.
- Confirm the engine builds as one Cargo package with exactly one library target and one binary target and an empty dependency table, and that its tests run independently of the observer package and with no terminal present.
- Confirm the library target's public interface matches `SPEC-MOK-002` rule 5 as amended exactly, and that no public item yields a mutable borrow of, or a reference into, authoritative state.

## Related architecture and ADRs

- `ADR-MOK-001` records the engine-authoritative in-process decision boundary and its consequences for future model integration. It remains accepted and is not superseded: it requires a superseding ADR only for replacing engine authority, and a read-only observer replaces none of it.
- `ADR-MOK-002` records the library target with an enumerated read-only public interface, and the test placement rule that follows from it.
- `ARCH-MOK-002` governs the terminal observer package, outside this architecture's boundary.
- `ADR-MOK-003` decides the two-package split and the user-interface dependency, and is the deciding ADR for `ARCH-MOK-002` and for the third and fourth amendments recorded above.
