+++
id = "ARCH-MOK-001"
type = "architecture"
title = "Single-process simulation architecture"
status = "approved"
owners = ["technical owner"]
created = "2026-08-11"
updated = "2026-08-11"

[relations]
constrains = [
  "SPEC-MOK-001",
  "REQ-MOK-001",
  "REQ-MOK-002",
  "REQ-MOK-003",
  "REQ-MOK-004",
  "REQ-MOK-005",
  "REQ-MOK-006",
  "REQ-MOK-007",
  "REQ-MOK-008",
  "REQ-MOK-009",
  "REQ-MOK-010",
  "REQ-MOK-011",
  "REQ-MOK-012",
]
+++

# Architecture: Single-process simulation architecture

## Context and scope

The minimum foundation is a local Rust command-line program. It must establish an authoritative deterministic engine and a replaceable decision boundary without importing the complexity of a service architecture, UI, persistence layer, or external model client.

## Components and responsibilities

1. **Application entry point** parses arguments, constructs the run, applies the optional action-trace output policy, streams formatted events, maps errors to exit codes, and owns process termination.
2. **Simulation engine** owns the world, agents, food, tick, entropy state, validation, rule application, event creation, termination, and summary.
3. **Decision boundary** defines read-only observations and proposed core actions. Its only foundation implementation is the local seeded baseline.

These responsibilities may be represented by a small number of Rust modules in one binary crate. They are logical boundaries, not a requirement to create a separate crate or framework for each component.

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
- Separate crates or services without an approved requirement.

## Quality attributes

- **Simplicity:** one binary crate and the minimum modules needed to make authority boundaries legible.
- **Determinism:** the same inputs produce byte-identical events and final state.
- **Testability:** rules can be tested through engine inputs and observable outputs without launching external systems.
- **Debuggability:** optional action tracing exposes every decision opportunity without altering engine behavior.
- **Extensibility at one seam:** a future OpenAI-backed source can implement the decision boundary without gaining mutation access.
- **Safety:** bounded values cannot wrap and invalid actions cannot partially apply.

## Conformance checks

- Review dependency direction and public APIs for mutable-state leakage.
- Confirm the dependency graph contains no network, async-runtime, database, or UI libraries.
- Run deterministic replay tests and invalid-action atomicity tests.
- Confirm all stochastic code is reachable from the explicit seeded entropy owner.
- Confirm the program builds as one Rust binary crate.

## Related ADRs

- `ADR-MOK-001` records the engine-authoritative in-process decision boundary and its consequences for future model integration.
