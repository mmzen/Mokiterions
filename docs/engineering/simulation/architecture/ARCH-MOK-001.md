+++
id = "ARCH-MOK-001"
type = "architecture"
title = "Single-process simulation architecture"
status = "approved"
owners = ["technical owner"]
created = "2026-08-11"
updated = "2026-08-17"

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
rationale = "This architecture establishes the foundation's system boundary as one in-process authoritative engine, fixes the dependency direction so decision sources never reach mutable world state, and defines the immutable observation and typed proposed-action contract as a maintained interface. It treats every decision source as untrusted and keeps future provider credentials outside the engine, selects deterministic single-owner state and seeded entropy as the consistency and reliability strategy, and defers an external model provider to an adapter at the same boundary. ADR-MOK-001 records the accepted alternatives and states that replacing engine authority requires a superseding ADR, which makes the decision materially difficult to reverse. Amended 2026-08-17: the target shape of the program is decided by ADR-MOK-002, which narrows one binary crate to one Cargo package built as a library target and a thin binary target, and makes the library target's public interface an enumerated read-only contract owned by SPEC-MOK-002. That decision fires the already-declared public-interface-or-protocol and material-alternatives triggers and leaves engine authority untouched, so this assessment stays adr_required and is now covered by ADR-MOK-001 and ADR-MOK-002 together."
assessed_by = "technical owner"
+++

# Architecture: Single-process simulation architecture

## Context and scope

The minimum foundation is a local Rust command-line program. It must establish an authoritative deterministic engine and a replaceable decision boundary without importing the complexity of a service architecture, UI, persistence layer, or external model client.

This architecture addresses the four requirement drivers that materially shape its boundaries rather than every requirement it conforms to: world authority (`REQ-MOK-004`), the in-process baseline decision source (`REQ-MOK-008`), reproducible entropy (`REQ-MOK-009`), and the text observation interface (`REQ-MOK-010`). The remaining foundation requirements are domain rules whose detailed behavior is governed by `SPEC-MOK-001`, which this architecture conforms to.

## Components and responsibilities

1. **Application entry point** parses arguments, constructs the run, applies the optional action-trace output policy, streams formatted events, maps errors to exit codes, and owns process termination.
2. **Simulation engine** owns the world, agents, food, tick, entropy state, validation, rule application, event creation, termination, and summary.
3. **Decision boundary** defines read-only observations and proposed core actions. Its only foundation implementation is the local seeded baseline.

These responsibilities may be represented by a small number of Rust modules in one Cargo package, built as a library target and a thin binary target. They are logical boundaries, not a requirement to create a separate crate or framework for each component.

The application entry point is the binary target and stays thin: process startup, stream buffering, one call into the library target, flush handling, and exit-code mapping. The simulation engine and the decision boundary live in the library target, whose public interface is enumerated by `SPEC-MOK-002` rule 5 and exposes no mutable authoritative state.

## Dependency direction

- The application entry point may depend on the simulation engine and baseline decision implementation.
- The baseline may depend on immutable observation and proposed-action types exposed by the simulation boundary.
- The simulation engine must not depend on a concrete baseline, OpenAI client, terminal UI, database, or web framework.
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
- Network calls, API credentials, asynchronous runtimes, databases, UI frameworks, plugin systems, or dependency injection containers in the foundation.
- Panics for invalid operator input or invalid proposed actions.
- Separate Cargo packages, workspaces, or services without an approved requirement. The library and binary targets of the single package are not separate crates in this sense.
- Public items that expose mutable or owned authoritative state, or any feature flag, `cfg` attribute, or test-support seam that exposes it outside the crate.

## Quality attributes

- **Simplicity:** one Cargo package with one library target and one thin binary target, and the minimum modules needed to make authority boundaries legible.
- **Determinism:** the same inputs produce byte-identical events and final state.
- **Testability:** rules can be tested through engine inputs and observable outputs without launching external systems, and the program's public contract can be tested from outside the implementation source files.
- **Debuggability:** optional action tracing exposes every decision opportunity without altering engine behavior.
- **Extensibility at one seam:** a future OpenAI-backed source can implement the decision boundary without gaining mutation access.
- **Safety:** bounded values cannot wrap and invalid actions cannot partially apply.

## Conformance checks

- Review dependency direction and public APIs for mutable-state leakage.
- Confirm the dependency graph contains no network, async-runtime, database, or UI libraries.
- Run deterministic replay tests and invalid-action atomicity tests.
- Confirm all stochastic code is reachable from the explicit seeded entropy owner.
- Confirm the program builds as one Cargo package with exactly one library target and one binary target, with an empty dependency table.
- Confirm the library target's public interface matches `SPEC-MOK-002` rule 5 exactly, and that no public item yields mutable or owned authoritative state.

## Related ADRs

- `ADR-MOK-001` records the engine-authoritative in-process decision boundary and its consequences for future model integration.
- `ADR-MOK-002` records the library target with an enumerated read-only public interface, and the test placement rule that follows from it.

## Amendment record

| Date | Amendment | Authority |
|---|---|---|
| 2026-08-17 | Schema migration to typed `addresses` and `conforms_to` relations and an explicit `decision_assessment`. | technical owner |
| 2026-08-17 | One binary crate narrowed to one Cargo package built as a library target and a thin binary target; prohibited-pattern, quality-attribute, and conformance-check wording updated; `REQ-MOK-016` added to `addresses` and `SPEC-MOK-002` to `conforms_to`. Decided by `ADR-MOK-002`; engine authority unchanged. | repository owner, on the technical owner's behalf |
