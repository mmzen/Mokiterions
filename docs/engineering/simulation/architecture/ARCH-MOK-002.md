+++
id = "ARCH-MOK-002"
type = "architecture"
title = "Terminal observer as a separate package over a read-only engine surface"
status = "approved"
owners = ["technical owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
addresses = [
  "REQ-MOK-021",
  "REQ-MOK-025",
  "REQ-MOK-026",
]
conforms_to = ["SPEC-MOK-003"]

[decision_assessment]
outcome = "adr_required"
triggers = [
  "system-boundary",
  "responsibility-or-dependency-direction",
  "public-interface-or-protocol",
  "technology-framework-vendor-or-external-service",
  "difficult-to-reverse",
  "material-alternatives",
]
rationale = "This architecture introduces a second package and therefore a new system boundary where the repository previously had one crate, and it fixes the dependency direction across that boundary so the engine cannot reach the interface. It promotes the engine's read-only observation surface to a maintained public interface consumed by a second component. It selects a specific external framework and version as the first external dependency the project has ever taken, at a measured surface of 57 crates, which introduces a supply-chain and upgrade obligation the foundation did not have. Reversal is difficult in practice: once the observer is the instrument used to assess later phases, removing it removes the means of assessment, and the observation surface becomes a contract other work depends on. Material alternatives exist and were rejected — a single crate with a feature flag, a piped text-stream consumer, a serialized snapshot protocol, and building no interface at all — each with different consequences for determinism and for the engine's empty dependency set."
assessed_by = "technical owner"
+++

# Architecture: Terminal observer as a separate package over a read-only engine surface

## Context and scope

`ARCH-MOK-001` establishes one authoritative in-process engine and, until this architecture, one crate. `INT-MOK-004`
argues that the project now needs an instrument for watching behavior, and `CAP-MOK-004` states what that instrument
does. Providing it requires the two things `ARCH-MOK-001` prohibited without an approved requirement: an external
user-interface framework, and a second package.

This architecture governs the observer package and the boundary between it and the engine. It does not govern any
simulation rule, and it does not govern the engine's internals, which remain under `ARCH-MOK-001` as amended on the
same date. `ARCH-MOK-001`'s prohibitions are not relaxed by this architecture; they are scoped to the engine package,
where they became checkable per package rather than asserted for a repository.

It addresses the three requirement drivers that materially shape its boundaries rather than every requirement it
conforms to: the promotion of proposal-and-authority information to a cross-boundary public interface
(`REQ-MOK-021`), the preservation of simulation outcome under observation (`REQ-MOK-025`), and the component
independence that makes the engine's empty dependency set enforceable (`REQ-MOK-026`). The remaining observer
requirements are presentation rules whose detailed behavior is governed by `SPEC-MOK-003`, which this architecture
conforms to.

## Components and responsibilities

1. **Engine package** — unchanged in authority. Owns world, agents, resources, tick, entropy, validation, rule
   application, event creation, termination, summary, and the `REQ-MOK-010` text stream. Additionally exposes the
   read-only observation surface of `SPEC-MOK-003`. Holds no dependency on anything.
2. **Observation surface** — the engine package's public boundary toward hosts: owned snapshots out, one single-tick
   advance in. It is a maintained interface, not an internal convenience.
3. **Command-line host** — the existing binary. Constructs a run, advances it to completion, streams text events.
   Unchanged.
4. **Observer host** — the new binary. Constructs a run, advances it under operator control, renders snapshots,
   handles input, retains and exports events. Holds every user-interface dependency.
5. **Presentation layer** — inside the observer package: layout selection, world-to-canvas mapping, pane rendering,
   key dispatch, event retention and export. Deliberately factored so that layout and mapping are pure functions of
   viewport size and snapshot content, and therefore testable without a terminal.

Components 3 and 4 are peer hosts of component 2. Neither is privileged, and neither can do anything the other
cannot, because their access is the same surface.

## Dependency direction

```text
mokiterions-tui  ──depends on──▶  mokiterions-core
       │                                 │
       ├── ratatui 0.30.2 (+ 56)         └── (nothing)
```

- The observer package depends on the engine package by path, and on the user-interface framework.
- The engine package depends on nothing, and in particular not on the observer package.
- No dependency edge runs from engine to observer. The direction is enforced by Cargo, not by review.
- Neither package depends on network, credential, asynchronous-runtime, database, or model-provider infrastructure.

The asymmetry is the point. A convention that the engine "should not" import rendering types is unenforceable in one
crate and fails silently. A package that does not list the observer as a dependency cannot import from it at all.

## Data and control flow

```text
CLI configuration
      |
      v
  Simulation  ──snapshot()──▶  WorldSnapshot (owned)  ──▶  layout ──▶ panes ──▶ terminal
      ▲                                                                            |
      |                                                                            v
  advance_tick()  ◀────── progression control only ──────────────────  key press / timer
      |
      v
  ordered events ──▶ retention buffer ──▶ export file
```

Control flows one way in each direction and the two never cross. Snapshots flow out of the engine and are never
handed back. Operator input reaches the engine only as a decision to advance, carrying no data. There is no path by
which a key press, a viewport size, a frame time, or a rendered value re-enters engine computation.

The engine's own text-stream path is untouched and continues to exist beside this one.

## Trust boundaries

- The observer is **untrusted with respect to world authority**, exactly as a decision source is. It receives owned
  values and holds no mutable handle. The single mutating call it may make changes nothing about *what* the engine
  computes, only *when*.
- The operator is untrusted with respect to world authority. No operator action mutates world state.
- Operator input — command-line arguments and the export path — is untrusted and validated before use. The export
  path is data: never code, never a read.
- The **user-interface framework is untrusted third-party code**, and this is the new trust boundary the
  architecture introduces. It is contained by three properties: it is a dependency of the observer package only, so
  it cannot be reached from the engine; it is granted no engine handle, only owned snapshots; and the engine remains
  independently buildable and testable without it. A defect or a supply-chain compromise in it can corrupt what an
  operator sees and can compromise the observer process; it cannot alter a simulation outcome, and it cannot reach
  the engine's build.
- The terminal is an untrusted external system whose dimensions and capabilities are validated rather than assumed.

## Required patterns

- One-way dependency from observer to engine, expressed as a package dependency.
- Owned, reference-free snapshots as the only outbound state transfer.
- Exactly one mutating operation on the observation surface, and it takes no operator data.
- Layout and world-to-canvas mapping as pure functions of viewport size and snapshot content, so they are testable
  without a terminal and cannot depend on time or run history.
- Rendering into an in-memory buffer that tests can assert cell by cell, so presentation claims are verified rather
  than inspected.
- Unconditional terminal restoration on every exit path, including panic.
- Bounded event retention with a declared capacity and a visible truncation marker.
- Wall-clock time confined to scheduling: when to draw, when to advance. Never an engine input.

## Prohibited patterns

- Any dependency edge from the engine package to the observer package, or any external dependency in the engine
  package.
- A user-interface dependency anywhere but the observer package, including a dependency shared by both.
- A mutable handle to world, agent, resource, event-log or engine state crossing the boundary.
- Any operator control that mutates simulation state, or any additional mutating operation on the observation
  surface.
- Wall-clock time, frame timing, input timing, terminal dimensions, or terminal capabilities reaching engine
  computation.
- Re-deriving the engine's validation verdict in the observer, which would let the display disagree with the engine
  about what was authorized.
- Advancing more than one tick per scheduling opportunity to recover from falling behind.
- Reading repository files, invoking version control, or performing network access at run time.
- Serialization, asynchronous runtimes, threads sharing simulation state, or a third package.
- Presenting a value the engine does not compute, including an inert placeholder that reads as a computed zero.

## Quality attributes

- **Non-perturbation:** an observed run and an unobserved run are byte-identical in authoritative events and final
  state. This is the architecture's primary attribute, and the one-way flow exists to make it structural rather than
  defended.
- **Containment:** the engine's empty dependency set survives the introduction of a 57-crate framework, provably and
  per package.
- **Testability without a terminal:** layout, mapping and rendering are assertable in memory, so presentation is
  covered by automated tests rather than by screenshots.
- **Independent failure:** the observer can fail to build or run without affecting whether the engine builds, tests
  or runs.
- **Legibility of authority:** the proposal-and-verdict pair crosses the boundary as data, so the boundary
  `ADR-MOK-001` protects becomes visible at the moment it operates.
- **Simplicity, bounded:** exactly two packages, one repository, one version, one candidate commit. This is a
  component boundary, not a service boundary.

## Conformance checks

- Confirm `cargo tree` for the engine package resolves to the engine package alone.
- Confirm the engine package does not list the observer package as a dependency, directly or transitively.
- Confirm every user-interface dependency appears in the observer package's manifest and in no other.
- Confirm the engine package's tests pass with no terminal attached.
- Confirm the observation surface's public items expose no `&mut self` operation other than the single-tick advance,
  and that snapshot types own their data and offer no mutating method.
- Confirm an observed run and an unobserved run at the same seed, configuration and decision source produce
  identical authoritative events and final state, with the observed run subjected to holding, single-stepping,
  selection, panning, zooming, filtering, export and resizing.
- Confirm per-tick entropy draw counts are identical observed and unobserved.
- Confirm layout and world-to-canvas mapping are exercised by tests that construct no terminal.
- Confirm rendered output is asserted from an in-memory buffer at each named viewport size in `SPEC-MOK-003`.
- Confirm the terminal is restored on normal exit, on error exit, and on panic.
- Confirm no repository read, version-control invocation, or network access occurs at run time.

## Related architecture and ADRs

- `ARCH-MOK-001`, as amended on 2026-08-17, governs the engine package. Its boundary, trust model, determinism
  properties and dependency direction are unchanged by this architecture.
- `ADR-MOK-001` remains accepted. It requires a superseding ADR only for replacing engine authority; a read-only
  observer replaces none of it, and the observation-and-proposal semantics it fixes are untouched.
- `ADR-MOK-003` is the deciding ADR for this architecture and for the `ARCH-MOK-001` amendment. It records the
  rejected alternatives and the measured dependency surface.
