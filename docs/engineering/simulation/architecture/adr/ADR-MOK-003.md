+++
id = "ADR-MOK-003"
type = "adr"
title = "Two-package split with a terminal observer over a read-only engine surface"
status = "approved"
owners = ["technical owner"]
created = "2026-08-17"
updated = "2026-08-18"

[relations]
decides = ["ARCH-MOK-002", "ARCH-MOK-001"]
+++

# ADR: Two-package split with a terminal observer over a read-only engine surface

## Status

Accepted by the technical owner on 2026-08-17, together with `ARCH-MOK-002` and the `ARCH-MOK-001` amendment.

`ADR-MOK-001` is not superseded by this decision. Neither is `ADR-MOK-002`, which this decision refines on the
workspace point; a dated note in that ADR's *Status* section records exactly which of its statements are narrowed.

**Note dated 2026-08-18.** The decision is unchanged. Its *Migration* section is corrected on two points of fact that
the merge with `master` exposed. `SPEC-MOK-002` requires **four** amendments rather than three: rule 3 freezes
`src/simulation.rs`'s contents against anything but a visibility change, and the observation surface is new code in
that file. And the two `ADR-MOK-002` statements this decision narrows are now named rather than left implicit. No
option was re-weighed and no consequence changed.

## Context

`INT-MOK-004` establishes that the project needs an instrument for watching behavior, and that the need grows with
each later phase rather than shrinking: traits and fear in Phase 2, conflict in Phase 3, and an untrusted
model-backed decision source in Phase 5 all produce behavior whose plausibility is a question about a spatial
situation at a specific tick. `CAP-MOK-004` states what the instrument does.

Building it collides with two prohibitions in `ARCH-MOK-001`, which is `approved` and was verified under
`VREC-MOK-001` and `VREC-MOK-002`:

> Network calls, API credentials, asynchronous runtimes, databases, **UI frameworks**, plugin systems, or dependency
> injection containers in the foundation.

> **Separate crates or services without an approved requirement.**

and with two of its conformance checks, which require the dependency graph to contain no UI libraries and the
program to build as one binary crate.

The second prohibition names its own unlock: separate crates are prohibited *without an approved requirement*, and
`REQ-MOK-026` is drafted as that requirement. The first does not, and requires this decision.

Three properties of the current system constrain every option:

1. **The engine has no external dependencies at all.** `[dependencies]` is empty, which makes "no network, no
   credentials, no async runtime, no database" checkable by reading one table rather than by auditing a graph.
   `ADR-MOK-001` relies on this when it states that the foundation requires no network and no credentials.
2. **Determinism is load-bearing.** `REQ-MOK-009` makes identical seed and configuration produce identical runs.
   Every figure the project holds rests on it, including four replay hashes retained under `VREC-MOK-002`. An
   interface introduces frame timing, input timing and resize events directly beside a deterministic engine.
3. **The text stream is verified behavior.** `REQ-MOK-010` was verified twice. Whatever is built must not disturb it.

## Decision drivers

- Keep the engine's external dependency set empty, and keep that fact checkable.
- Make it structurally impossible for the engine to depend on the interface, rather than conventionally discouraged.
- Guarantee that observing a run cannot change it, and make the guarantee demonstrable rather than argued.
- Leave the `REQ-MOK-010` text stream and its verified behavior untouched.
- Give the observer enough state to render the whole world, since a partial view is a weaker instrument.
- Keep the presentation testable without a terminal, since a claim about a screen that only a human has seen is weak
  evidence in this repository.
- Add no network, credential, asynchronous runtime, database, or service.
- Accept the smallest new trust surface that delivers the capability, and contain what is accepted.

## Considered options

### Option 1: One crate with a feature flag or a runtime mode

Keep a single package; add the framework as a dependency and select text or interface behavior at compile time or
run time.

Smallest diff, no workspace, and the engine's sources do not move. It fails the first driver outright: the
framework's 57 crates land in the same package as the engine, so the engine's dependency set is no longer empty and
"the engine has no external dependencies" becomes an assertion about module discipline rather than a fact about a
manifest. A feature flag narrows the default build but not the property — the dependency is declared, and nothing
prevents an engine module importing a rendering type under that feature. It also fails the second driver, because
the direction can only be maintained by review.

### Option 2: Two packages, observer hosts the engine, read-only snapshot surface

A workspace of two packages. The engine package keeps every simulation rule, keeps the text-stream binary, and gains
a public read-only surface: owned snapshots out, one single-tick advance in. The observer package depends on the
engine by path, holds the framework, and drives progression itself.

The engine's dependency set stays empty and stays checkable per package. The direction is enforced by Cargo.
Determinism is structural rather than defended: the observer calls the same advance the text binary calls, in the
same order, and consumes no entropy, so a run held for a minute and single-stepped nine times executes the identical
tick sequence. The text stream does not move. Presentation is testable in memory, because the framework provides a
buffer backend whose cells tests can assert. It costs a workspace, a maintained public surface, and 57 crates in the
observer.

### Option 3: Pipe the text stream into a separate consumer

Run the engine unchanged and pipe its output to an observer that parses it. Two processes, no shared code, no new
dependency in the engine, and the strongest imaginable isolation.

It cannot deliver the capability. The text stream does not carry full world state — there is no per-tick record of
every agent's position and every standing resource — so the whole-world view of `REQ-MOK-019` cannot be
reconstructed from it. Delivering it would require extending `REQ-MOK-010`'s output, which is verified behavior, to
carry a full state dump per tick, at which point the text stream is being redesigned to serve a display. It also
makes single-stepping impossible: a pipe cannot ask the producer to stop between ticks. Rejected as unable to meet
`REQ-MOK-019` and `REQ-MOK-023` rather than on cost.

### Option 4: Serialized snapshot protocol between engine and observer

The engine serializes a per-tick snapshot; the observer deserializes it. Replayable from a file, and a clean
contract.

Serialization in the engine means either a serialization dependency in the package whose dependency set must stay
empty, or a hand-rolled format and parser — a second output contract to specify, verify and maintain, for a benefit
option 2 already provides in-process. Rejected as cost without corresponding benefit at this stage. The option
remains open later if out-of-process observation is ever wanted, since option 2's snapshot types are exactly what
would be serialized.

### Option 5: Build no interface

Continue with text output and richer post-run analysis.

It is the only option that preserves every current property at zero cost, and it is a real alternative rather than a
strawman. It is rejected on the grounds `INT-MOK-004` states: the phases that follow produce behavior whose
assessment is a spatial question, and deferring the instrument means evaluating those phases without the means to
evaluate them. The cost of adding it does not fall over time, and the cost of not having it rises.

## Decision

Adopt option 2.

1. Restructure the repository as a Cargo workspace of exactly **two packages**: `Mokiterions`, the engine, and
   `mokiterions-tui`, the observer. No third package, no service, no separate release artifact.
2. Keep the engine package at the repository root with its sources in their existing location, so the
   `REQ-MOK-010` text stream does not move and its verified behavior is not disturbed by relocation.
3. Give the engine package a public read-only observation surface as specified by `SPEC-MOK-003`: owned,
   reference-free snapshots out; exactly one mutating operation, the single-tick advance, taking no operator data.
4. Keep the engine package's external dependency set **empty**, with no exception, including a dependency shared with
   the observer.
5. Adopt **`ratatui` version `0.30.2`** with `default-features = false` and features `crossterm`, `layout-cache`,
   `underline-color` as the observer package's only external dependency. Measured surface: **57 crates** including
   itself. The `serde` feature stays off, and no feature enabling networking, an asynchronous runtime, or
   serialization is enabled.
6. Confine every user-interface dependency to the observer package.
7. Grant the observer no engine handle and no mutable state. Operator influence over the simulation is limited to
   when the single-tick advance is called.
8. Confine wall-clock time to scheduling — when to draw, when to advance — and never pass it to the engine.
9. Factor layout and world-to-canvas mapping as pure functions of viewport size and snapshot content, and verify
   rendering by asserting an in-memory buffer, so presentation is covered by automated tests rather than by looking.
10. Amend `ARCH-MOK-001` to scope its one-crate rule, its empty-dependency-graph rule and its prohibition on UI
    frameworks to the engine package, where they become checkable per package. Relax no boundary, no trust property
    and no determinism property.
11. Keep `ADR-MOK-001` accepted and unsuperseded. It requires a superseding ADR only for replacing engine authority,
    and a read-only observer replaces none of it: the engine still owns all mutable state, decision sources still
    receive immutable observations and return typed proposals, and every proposal is still validated.

### Why this framework, and why a terminal

A terminal interface is chosen over a graphical or web one because a web interface would require a server, a
transport, a serialization format and a browser — network and asynchronous-runtime surface that `ADR-MOK-001`
excludes for reasons that still hold — and a graphical interface would add a windowing and rendering stack larger
than the one accepted here for a display whose content is a grid of discrete cells. The world is 128 × 128 discrete
cells with twelve agents and about 122 resources. That is natively a character grid.

`ratatui` is chosen over the alternatives for reasons that were measured rather than assumed. It builds on edition
2024 with the repository's toolchain. It provides a braille canvas whose 2 × 4 sub-cell dots address the full
128 × 128 world at one dot per world cell in a 64 × 32 character area — verified, including that two adjacent world
cells render as distinct dots within one character cell. It provides an in-memory test backend whose cells are
assertable, which is what makes the presentation verifiable to this repository's evidence standard rather than
demonstrable by screenshot. It requires no asynchronous runtime. Its serialization feature is optional and stays off.

Feature minimization was measured and is honestly modest: the full default feature set resolves to 63 crates and the
selected set to 57. The six crates removed are `time`, `time-core`, `deranged`, `num-conv`, `powerfmt` and
`ratatui-macros`, the first five pulled in only by a calendar widget this observer does not use. The remaining 57 are
structural — the framework's own crates, the terminal backend, a layout solver, a compact string type, and a
proc-macro chain. Trimming features does not make this a small dependency, and this decision does not claim it does.

## Consequences

### Positive

- The engine's empty dependency set survives, and becomes *more* rigorous: previously a property of a repository that
  happened to contain one crate, now a per-package fact that a workspace containing a 57-crate framework cannot
  weaken.
- The engine cannot depend on the interface. Cargo refuses, so the property cannot be lost to oversight or to a
  refactor under time pressure.
- Determinism under observation is structural. The observer calls the same advance in the same order and draws no
  entropy, so no amount of operator interaction can change a run.
- The `REQ-MOK-010` text stream is untouched, so `VREC-MOK-001` and `VREC-MOK-002` remain accurate about the behavior
  they verified.
- The engine remains buildable and testable in an environment with no package registry access and no terminal.
- The observation surface is reusable: a future serialized protocol, a headless recorder, or a different interface
  consumes the same snapshots without touching the engine.
- The authority boundary becomes visible at the moment it operates, which is what makes Phase 5's untrusted decision
  source assessable.

### Negative

- **The project takes its first external dependency, and it is 57 crates.** That is an upgrade obligation, a
  compile-time cost, and a supply-chain surface that did not exist. It is the real price of this decision and it is
  not small.
- `Cargo.lock` acquires 57 entries and becomes a file that must be reviewed on change rather than confirmed empty.
- The observation surface is a maintained public contract. Every future attribute — fear, traits, names, combat
  outcomes — must be added to it deliberately, and every snapshot field is a compatibility obligation.
- A workspace is more structure than one crate. Commands acquire package selection, and a contributor must know which
  package they are in.
- The observer's correctness is only as good as its tests, and a rendering defect shows an operator a wrong world
  while the simulation is right. This is why in-memory buffer assertions are required rather than optional.
- The framework's own defects can corrupt what an operator sees. They cannot corrupt a simulation outcome, but an
  operator could act on a wrong picture.
- Reversal is difficult in practice. Once the observer is the instrument used to assess later phases, removing it
  removes the means of assessment.

### Operational and security

- No network access, no credential, no model provider, no asynchronous runtime, and no database in either package.
- The observer writes the filesystem once per requested export and never reads it. An operator-supplied export path
  is data: never code, never a read.
- No credential, secret, environment variable, absolute path or wall-clock value appears in a frame or an export.
- The framework is untrusted third-party code, contained by being reachable only from the observer package, by
  receiving only owned snapshots, and by the engine remaining independently buildable without it. A compromise in it
  can affect what an operator sees and the observer process; it cannot alter a simulation outcome and cannot reach the
  engine's build.
- The pinned version and feature set are specified. Changing either is a specification amendment, not an
  implementation choice, so a dependency bump cannot silently enlarge the surface or enable serialization.

### Migration

The engine package keeps its name, both of its target names, and the location of its sources. `SPEC-MOK-002` rules 1
and 2 fix `Mokiterions` as the package name, `mokiterions` as the library target and `Mokiterions` as the binary
target, and rule 2 ties the binary's name to the first line of `USAGE`, whose content `SPEC-MOK-001`'s *Help output*
section fixes and `VER-MOK-004` verifies. Renaming any of the three to mark the split would edit a verified
operator-facing string for a cosmetic reason, so this decision leaves all three alone: the produced binary's filename
does not change, and no artifact, script, CI step or document has to.

The library target already exists. `ADR-MOK-002` decided it and `WO-MOK-003` implemented it, so the diff to verified
engine behavior here is narrower than a two-package split would suggest: not a library target and a public surface,
but the addition of an observation surface to a library target that already carries an enumerated read-only
interface. `SPEC-MOK-002` rule 5 states that that interface "grows only when an approved requirement needs it to
grow, and this specification is amended in the same act". `REQ-MOK-019` through `REQ-MOK-027` are those requirements,
and the amendment is one of the four this decision requires.

What the second package does contradict is `SPEC-MOK-002` rule 1, which admits "no third target, no second package,
no workspace". That prohibition was written when no approved requirement needed one. `REQ-MOK-026` now does, and it
is the approved requirement that both rule 1 and `ARCH-MOK-001`'s prohibited-pattern list reserve the exception for.
Rule 3 is contradicted in the same way and for the same reason: it freezes `src/simulation.rs`'s contents against
anything but a visibility change, and the observation surface is new code. `SPEC-MOK-003`'s *Compatibility and
migration* section states all four required amendments and `WO-MOK-005` makes them approval preconditions.

This decision also narrows two statements in `ADR-MOK-002`, which it refines and does not supersede: its
decision driver "Add no dependency, no second package, and no workspace", and the first bullet of its *Decision*
saying the library target "does not introduce a second package, a workspace, or a service". Both were correct when
written, under no approved requirement for one. `ADR-MOK-002`'s rejection of its own option 4 stands and is not
reversed here: option 4 would have split the engine across package boundaries, and this decision leaves the engine
package whole and adds a package that consumes it. A dated note in `ADR-MOK-002`'s *Status* section records this.

If out-of-process or recorded observation is ever wanted, preserve the snapshot semantics and serialize the same
types outside the engine, exactly as `ADR-MOK-001` prescribed for a future model adapter. Replacing engine authority,
allowing the observer to mutate state, or letting an operator alter the world would change determinism, consistency
and verification assumptions, and requires a superseding ADR.

## Validation

- Architecture review confirms no dependency edge from the engine package to the observer package, and no external
  dependency in the engine package.
- `cargo tree` for the engine package resolves to the engine package alone.
- Public-surface review confirms exactly one mutating operation, that it takes no operator data, and that snapshot
  types own their data and expose no mutating method.
- An observed run and an unobserved run at the same seed, configuration and decision source produce identical
  authoritative events and final state, with the observed run held, single-stepped, selected, panned, zoomed,
  filtered, exported and resized.
- Per-tick entropy draw counts are identical observed and unobserved.
- The engine package's tests pass with no terminal attached.
- Rendering is asserted from an in-memory buffer at each named viewport size in `SPEC-MOK-003`, including the floor and
  the size at which the whole world is presented at one dot per world cell.
- Dependency review confirms the observer's resolved graph matches the specified version and feature set, that
  `serde` is absent, and that no networking, asynchronous-runtime, database or model-provider crate appears.
- Terminal restoration is demonstrated on normal exit, on error exit, and on panic.
