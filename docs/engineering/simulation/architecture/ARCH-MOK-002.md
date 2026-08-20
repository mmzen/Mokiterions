+++
id = "ARCH-MOK-002"
type = "architecture"
title = "Terminal observer as a separate package over a read-only engine surface"
status = "approved"
owners = ["technical owner"]
created = "2026-08-17"
updated = "2026-08-20"

[relations]
addresses = [
  "REQ-MOK-021",
  "REQ-MOK-025",
  "REQ-MOK-026",
  "REQ-MOK-028",
  "REQ-MOK-047",
]
conforms_to = ["SPEC-MOK-003", "SPEC-MOK-004"]

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
rationale = "This architecture introduces a second package and therefore a new system boundary where the repository previously had one crate, and it fixes the dependency direction across that boundary so the engine cannot reach the interface. It promotes the engine's read-only observation surface to a maintained public interface consumed by a second component. It selects a specific external framework and version as the first external dependency the project has ever taken, at a measured surface of 57 crates, which introduces a supply-chain and upgrade obligation the foundation did not have. Reversal is difficult in practice: once the observer is the instrument used to assess later phases, removing it removes the means of assessment, and the observation surface becomes a contract other work depends on. Material alternatives exist and were rejected — a single crate with a feature flag, a piped text-stream consumer, a serialized snapshot protocol, and building no interface at all — each with different consequences for determinism and for the engine's empty dependency set. Amended 2026-08-18: the observer package now builds a library target as well as a binary, which promotes its presentation layer to a stated public interface with a maintained contract, and each package's manifest, sources and tests move under its own directory. Both are boundary and public-interface changes to this architecture rather than to the engine's, and `ADR-MOK-004` decides them; the alternatives it rejected — documenting the asymmetry, a feature-gated test-support seam, and a thin observer binary mirroring the engine's — are material and were weighed. Neither change alters the dependency direction, the framework selection, the trust boundary or the non-perturbation property, so the triggers already recorded are the same triggers. Amended 2026-08-20: ADR-MOK-006 replaces the empty-set premise with a per-package declared set; no boundary, direction or trust property moves."
assessed_by = "technical owner"
+++

# Architecture: Terminal observer as a separate package over a read-only engine surface

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-17 | Original approved content for `CAP-MOK-004`. | Approved; implemented under `WO-MOK-005` and verified under `VREC-MOK-005`. |
| 2026-08-18 | The observer package's target shape and the repository's package-directory layout, for `REQ-MOK-028`, `REQ-MOK-029` and `REQ-MOK-030`. Component 4 no longer calls the observer host "the new binary": it is a library target and a binary target, and component 5's presentation layer is what the library target carries. *Testability without a terminal* extended from "assertable in memory" to "assertable in memory through a stated public interface, from a test tier outside the crate". A required pattern added for the library target and its provenance-closed interface; three prohibited patterns added — widening an item to reach it from a test, ungating a `#[cfg(test)]` item, and any test-support seam. Four conformance checks added. `addresses` grew by `REQ-MOK-028`, `conforms_to` by `SPEC-MOK-004`, and `decision_assessment.rationale` records both changes against the triggers already declared. No dependency edge, no trust boundary, no non-perturbation property and no quality attribute other than testability changes. | Approved 2026-08-18 by the repository owner as technical owner, by way of `ADR-MOK-004`, whose *Required amendments* section states this amendment in full. The implementation agent wrote the text under `WO-MOK-006`; it did not decide it. `VREC-MOK-005` binds this architecture's 2026-08-17 content to `WO-MOK-005`'s commit and is not edited. |
| 2026-08-20 | **The engine's empty-dependency premise becomes a per-package declared set**, decided by `ADR-MOK-006`. Prohibited pattern 1 keeps its first clause — no dependency edge from engine to observer — and its second becomes an external dependency in either package that is not a declared entry of that package's set. The prohibition on re-deriving the engine's validation verdict is **extended** to the observer's declared crates, and with it decision 11's reservation of the proprietary core; it gains reach and does not change meaning. *Containment* is rewritten: the property was that the engine's set survived a 57-crate framework *empty*, and it is now that the set survives it *declared*, established by comparing the resolved graph against the declaration per package. The first conformance check becomes that comparison for both packages instead of *"`cargo tree` for the engine package resolves to the engine package alone"*. **Prohibited pattern 2 — a user-interface dependency anywhere but the observer package, including a dependency shared by both — is deliberately unchanged**: it is the one dependency prohibition this relaxation does not touch. **This row also reaches three clauses the deciding ADR did not enumerate** — component 1's *"Holds no dependency on anything"*, the *Dependency direction* diagram and its bullet *"The engine package depends on nothing"*, and the driver sentence naming `REQ-MOK-026`'s empty-set clause — because each asserted the withdrawn rule, and `REQ-MOK-047` is added to `addresses` beside `REQ-MOK-026` as the requirement carrying what replaced it. Nothing about the dependency *direction*, the trust boundary, non-perturbation, the observation surface or determinism moves. | Approved 2026-08-20 by the repository owner acting as accountable technical owner, by way of `ADR-MOK-006`, whose *Required amendments* section states this amendment in full. The implementation agent wrote the text under `WO-MOK-013`; it did not decide it. `VREC-MOK-005` binds this architecture's 2026-08-17 content and is not edited. |

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
independence that makes the engine's declared dependency set enforceable per package (`REQ-MOK-026`, whose
empty-dependency clause `ADR-MOK-006` withdrew, and `REQ-MOK-047`, which carries what replaced it). The remaining observer
requirements are presentation rules whose detailed behavior is governed by `SPEC-MOK-003`, which this architecture
conforms to.

## Components and responsibilities

1. **Engine package** — unchanged in authority. Owns world, agents, resources, tick, entropy, validation, rule
   application, event creation, termination, summary, and the `REQ-MOK-010` text stream. Additionally exposes the
   read-only observation surface of `SPEC-MOK-003`. Holds exactly the dependencies `SPEC-MOK-002` declares for it,
   which is nothing as that declaration stands, and in particular holds no user-interface dependency.
2. **Observation surface** — the engine package's public boundary toward hosts: owned snapshots out, one single-tick
   advance in. It is a maintained interface, not an internal convenience.
3. **Command-line host** — the existing binary. Constructs a run, advances it to completion, streams text events.
   Unchanged.
4. **Observer host** — the observer package's binary target. Constructs a run, advances it under operator control,
   renders snapshots, handles input, retains and exports events. Holds every user-interface dependency.
   **Amended 2026-08-18.** This component read "the new binary". The observer package builds two targets: a library
   target carrying component 5, and this binary, which is the only thing that acquires a terminal, decides whether to
   launch, schedules and loops. The binary keeps that work rather than becoming thin — `ADR-MOK-004`'s Option 4 is
   the rejected alternative — because its start-up and its launch decision are covered by tests that reach private
   items, and promoting them would widen the interface for a test.
5. **Presentation layer** — the observer package's library target: layout selection, world-to-canvas mapping, pane
   rendering, key dispatch, event retention and export. Deliberately factored so that layout and mapping are pure
   functions of viewport size and snapshot content, and therefore testable without a terminal. **Amended
   2026-08-18.** This layer is now a library target with an enumerated public interface, so it is reachable from a
   test tier outside the crate. The interface is closed by provenance rather than by enumeration: it is exactly the
   items that were already public before the target existed, and `SPEC-MOK-004` rule 6 counts them and forbids
   widening. It is not a trust boundary — component 2 is — and it holds no authority.

Components 3 and 4 are peer hosts of component 2. Neither is privileged, and neither can do anything the other
cannot, because their access is the same surface.

## Dependency direction

```text
mokiterions-tui  ──depends on──▶  Mokiterions
       │                                 │
       ├── ratatui 0.30.2 (+ 56)         └── (its declared set: empty today)
```

Each side of the diagram is a package's declared set, not a fixed shape: `SPEC-MOK-003` declares the observer's and
`SPEC-MOK-002` declares the engine's, and `ADR-MOK-006` governs what may join either. The engine's is empty as this is
written, and an empty declared set is a fact about the current declaration rather than a rule.

- The observer package depends on the engine package by path, and on the user-interface framework.
- The engine package depends on what `SPEC-MOK-002` declares for it — nothing, as that specification's declared set
  stands today — and in particular not on the observer package.
- No dependency edge runs from engine to observer. The direction is enforced by Cargo, not by review.
- Neither package depends on network, credential, asynchronous-runtime, database, or model-provider infrastructure.
  This is preserved without relaxation by `ADR-MOK-006` decision 4 and checked by name, having previously followed from
  the engine's set being empty and the observer's being one framework.

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
- The presentation layer built as a library target whose public interface is closed by provenance — exactly the items
  that were already public — so that a tier of tests reaches it from outside the crate and a change to what it
  presents fails a test that has no private access to repair itself with. Added 2026-08-18 for `REQ-MOK-028`.
- Each package's manifest, sources and tests under one directory named for that package, with the repository root
  holding a workspace manifest and no package's implementation. Added 2026-08-18 for `REQ-MOK-030`.
- Unconditional terminal restoration on every exit path, including panic.
- Bounded event retention with a declared capacity and a visible truncation marker.
- Wall-clock time confined to scheduling: when to draw, when to advance. Never an engine input.

## Prohibited patterns

- Any dependency edge from the engine package to the observer package, or an external dependency in either package
  that is not a declared entry of that package's set under `ADR-MOK-006`.
- A user-interface dependency anywhere but the observer package, including a dependency shared by both.
- A mutable handle to world, agent, resource, event-log or engine state crossing the boundary.
- Any operator control that mutates simulation state, or any additional mutating operation on the observation
  surface.
- Wall-clock time, frame timing, input timing, terminal dimensions, or terminal capabilities reaching engine
  computation.
- Re-deriving the engine's validation verdict in the observer, which would let the display disagree with the engine
  about what was authorized. This binds the observer's declared crates as well as its own code: no entry in the
  observer package's declared set may re-derive the verdict either, and none may implement simulation semantics, own or
  advance entropy, or validate an action. The prohibition gains reach rather than changing meaning, which is
  `ADR-MOK-006` decision 11.
- Advancing more than one tick per scheduling opportunity to recover from falling behind.
- Reading repository files, invoking version control, or performing network access at run time.
- Serialization, asynchronous runtimes, threads sharing simulation state, or a third package.
- Presenting a value the engine does not compute, including an inert placeholder that reads as a computed zero.
- Widening any item's visibility, in either package, in order to reach it from a test. A test that needs internal
  access belongs beside the code. Added 2026-08-18 for `REQ-MOK-028` and `REQ-MOK-029`.
- Removing, gating or otherwise relaxing a `#[cfg(test)]` attribute so that an item exists in a non-test build. The
  four hooks on the observer's state type stay as they are. Added 2026-08-18.
- Any test-support seam, feature-gated or otherwise, that exposes either package's internals outside its crate.
  Added 2026-08-18.

## Quality attributes

- **Non-perturbation:** an observed run and an unobserved run are byte-identical in authoritative events and final
  state. This is the architecture's primary attribute, and the one-way flow exists to make it structural rather than
  defended.
- **Containment:** the engine package's dependency set is exactly what `SPEC-MOK-002` declares for it, the observer's
  57-crate framework is not in that set, and both facts are established per package by comparing the resolved graph
  against the declaration rather than argued. **Amended 2026-08-20:** the property this attribute names used to be that
  the engine's set survived the framework's introduction *empty*; after `ADR-MOK-006` it is that the engine's set
  survives it *declared*. Containment is unchanged in what it protects — nothing the observer pulls in reaches the
  engine's build — and changed in what proves it.
- **Testability without a terminal:** layout, mapping and rendering are assertable in memory, so presentation is
  covered by automated tests rather than by screenshots. **Amended 2026-08-18:** and assertable *through a stated
  public interface, from a test tier outside the crate*. In-memory assertion alone left every observer test inside the
  binary, where it could reach any private item and so could be repaired against a changed contract without the
  change being visible. The tier outside the crate is what makes a break in what the observer presents fail a test.
  Tests that assert the layer's internals stay beside the code, which is the same attribute, not an exception to it.
- **Independent failure:** the observer can fail to build or run without affecting whether the engine builds, tests
  or runs.
- **Legibility of authority:** the proposal-and-verdict pair crosses the boundary as data, so the boundary
  `ADR-MOK-001` protects becomes visible at the moment it operates.
- **Simplicity, bounded:** exactly two packages, one repository, one version, one candidate commit. This is a
  component boundary, not a service boundary.

## Conformance checks

- Confirm each package's resolved dependency graph equals the set declared for it — `SPEC-MOK-002` for the engine
  package, `SPEC-MOK-003` for the observer package — at the declared versions and the declared feature sets, with no
  undeclared entry and no declared entry missing. Resolve per package: a workspace graph would not answer it.
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

Added 2026-08-18 for `REQ-MOK-028`, `REQ-MOK-029` and `REQ-MOK-030`:

- Confirm the observer package builds a library target and a binary target, that the library target's public
  interface is exactly the items that were already public before it existed, and that no item's visibility widened.
  The check is that each module file the library declares is identical outside its `#[cfg(test)]` blocks to its
  content at the predecessor commit.
- Confirm every `#[cfg(test)]` item in the observer retains its attribute, and that no test outside the crate names
  one.
- Confirm every observer test is in exactly one tier, that each test outside the crate reaches the code only through
  the library target's public interface with its assertions unchanged, and that one `cargo test` invocation runs both
  tiers of both packages with no feature, environment variable, `#[ignore]` or terminal.
- Confirm each package's manifest, sources and tests are under one directory named for that package, that the
  repository root's manifest declares no package, and that every package name, target name, target kind and operator
  command resolves as it did before.

## Related architecture and ADRs

- `ARCH-MOK-001`, as amended on 2026-08-17, governs the engine package. Its boundary, trust model, determinism
  properties and dependency direction are unchanged by this architecture.
- `ADR-MOK-001` remains accepted. It requires a superseding ADR only for replacing engine authority; a read-only
  observer replaces none of it, and the observation-and-proposal semantics it fixes are untouched.
- `ADR-MOK-003` is the deciding ADR for this architecture and for the `ARCH-MOK-001` amendment. It records the
  rejected alternatives and the measured dependency surface. Its decision 4 and the word *"only"* in its decision 5
  are reversed by `ADR-MOK-006`; everything else it decides stands, and it is not superseded.
- `ADR-MOK-006` decides the repository's third-party-component policy and decides this architecture together with
  `ARCH-MOK-001`. It replaces the engine's empty-dependency rule and the observer's escalation gate with one admission
  test against a per-package declared set, and it is the deciding ADR for the 2026-08-20 amendment recorded above.
  `ratatui`'s pin is unaffected by it: that pin was already a declared entry and stays one.
