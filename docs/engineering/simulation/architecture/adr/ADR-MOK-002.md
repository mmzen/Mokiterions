+++
id = "ADR-MOK-002"
type = "adr"
title = "Library target with an enumerated read-only public interface"
status = "approved"
owners = ["technical owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
decides = ["ARCH-MOK-001"]
+++

# ADR: Library target with an enumerated read-only public interface

## Status

Accepted 2026-08-17 by the repository owner acting as technical owner. It refines `ARCH-MOK-001` and `ADR-MOK-001` on
one point — the crate-target shape of the program — and supersedes neither.

The amendments this ADR requires were applied on approval: `ARCH-MOK-001` in its amendment record, `ADR-MOK-001` in
place with a dated note on its *Decision* bullet, and `SPEC-MOK-001` as a row in its amendment record. On the choice
this ADR left to the technical owner, in-place amendment of `ADR-MOK-001` was selected over supersession, for the
reason stated under *Required amendments*.

## Context

`ARCH-MOK-001` fixes the foundation as one in-process authoritative engine. Three of its statements bear on target
shape rather than on authority:

- *Components and responsibilities*: "These responsibilities may be represented by a small number of Rust modules
  in one binary crate."
- *Prohibited patterns*: "Separate crates or services without an approved requirement."
- *Quality attributes*: "**Simplicity:** one binary crate and the minimum modules needed…", and the matching
  conformance check "Confirm the program builds as one Rust binary crate."

`ADR-MOK-001` states the same thing as a decision bullet: "Run the foundation as one Rust process and one binary
crate."

`REQ-MOK-016` and `REQ-MOK-017` cannot be met under those statements, and the obstruction is a property of Rust
rather than a preference. An integration test outside an implementation source file is compiled as its own crate
and links a **library** target; it cannot link a binary target. A crate with no library target therefore has no
place to put a test other than inside its own source files, and consequently has no interface that anything outside
it can reach. That is the state of this repository: fifty-two tests, all with private access, and no stated public
contract for Phase 4's additional output stream or Phase 5's provider adapter to be written against.

Two further facts constrain the options. First, the declared lint gate is
`cargo clippy --all-targets --all-features -- -D warnings`, which implies `non_snake_case`; a library target
inheriting the package name `Mokiterions` fails it outright. Second, thirty-four of the fifty-two tests assign
agent state directly or call private engine methods, so no arrangement relocates all of them without exposing
authoritative state.

`ARCH-MOK-001`'s `decision_assessment` already declares the triggers this decision fires: `system-boundary`,
`public-interface-or-protocol`, `difficult-to-reverse`, and `material-alternatives`. Once a public interface is
stated and written against, retracting it costs more than never having stated it, which is why this is an ADR and
not a local implementation choice.

## Decision drivers

- Preserve engine authority exactly. `REQ-MOK-004` and `ADR-MOK-001`'s prohibition on exposing mutable world state
  are not negotiable, and a public interface must not become the hole they were written to prevent.
- Give the program a contract that a test, and later an adapter, can be written against.
- Keep the operator-facing command name `Mokiterions`, which appears in `USAGE` and in operator instructions.
- Keep the lint gate at `-D warnings`. A naming problem is solved by naming, not by relaxing a gate.
- Add no dependency, no second package, and no workspace.
- Change no observable behavior. This decision must be invisible to an operator.
- Keep the decision cheap to reverse in code, and honest about the part that is not cheap to reverse in contract.

## Considered options

### Option 1: Keep one binary target and leave every test inline

Costs nothing and risks nothing. It is also the status quo, and it does not meet `REQ-MOK-016` or `REQ-MOK-017`. The
program continues to promise nothing to anything outside itself; the items already written `pub` remain
indistinguishable from implementation details; no test can fail because the operator-visible contract changed; and
the Phase 5 adapter contract is deferred a third time. Rejected because it satisfies neither requirement.

### Option 2: Library target plus a feature-gated test-support seam, so every test moves

Add a library target and a `test-support` feature that re-exports engine internals — the agent collection,
observation construction, action application, survival, regeneration — so all fifty-two tests can live in `tests/`.
The result is uniform and the "tests go under `tests/`" instruction is satisfied literally.

Rejected on three independent grounds. It publishes mutable authoritative state, and gating that behind a feature
does not make the exposure conditional in any sense that matters, because the feature is enabled during the build
that exposes it; the substance of `ADR-MOK-001`'s prohibition is violated. It requires a self dev-dependency and
roughly a hundred and fifty lines of seam that exists only to be tested through. And it forces the rewriting of
thirty-four test bodies, each rewrite an opportunity to weaken an assertion that `VREC-MOK-002` already depends on —
the largest correctness risk in an initiative whose whole purpose is to change nothing.

### Option 3: Library target with an enumerated read-only public interface, tests split by required access

Declare a library target `mokiterions` at `src/lib.rs` and a binary target `Mokiterions` at `src/main.rs` within the
one existing package. Move the process-boundary function `execute` into the library. Enumerate the public interface
as a closed list: the items already written `pub`, plus a public `TerminationReason`, read-only `RunSummary`
accessors returning copies, `Density::resources_per_territory`, and the `CELLS_PER_TERRITORY` constant. Publish
nothing else, and prohibit widening anything to accommodate a test. Place each test by the access it requires:
roughly fourteen to eighteen in `tests/`, the remainder beside the code they assert.

### Option 4: A separate Cargo package for the engine, consumed by a thin CLI package

The strongest boundary and the only option that makes the interface a versioned artifact. Rejected outright: it is
what `ARCH-MOK-001` prohibits and what `ADR-MOK-001`'s option 2 already rejected, it introduces a workspace and
cross-package versioning, and it buys nothing that option 3 does not, since both requirements are satisfied by a
library target within one package.

## Decision

Adopt option 3, as specified in `SPEC-MOK-002`.

- Build the program as one Cargo package with two targets: a library target `mokiterions` and a binary target
  `Mokiterions`. This replaces "one binary crate" with "one package, one library target, one thin binary target",
  and it does not introduce a second package, a workspace, or a service.
- Name the library target in snake case, because `-D warnings` implies `non_snake_case`. Keep the binary target
  named `Mokiterions`, because it is the operator-facing command.
- Keep the binary target thin: process startup, stream locking and buffering, one call into the library, flush
  handling, exit-code mapping. No module declarations and no tests.
- Treat the library target's public interface as a closed enumeration owned by `SPEC-MOK-002` rule 5. It grows only
  when an approved requirement needs it to, and the specification is amended in the same act.
- Admit an item to that interface only when it is a value, a value constructor, a pure function of a value, or an
  accessor returning a copy of a fact the program already emits.
- Prohibit, in every build configuration including test builds, any public path to the world grid, the agent
  collection, the resource collection, the tick counter, the entropy state, the event log, or any mutating engine
  handle. `ADR-MOK-001`'s prohibition is unchanged and now applies to a surface that is actually reachable.
- Prohibit any test-support seam, feature flag, conditional visibility, or self dev-dependency introduced to make a
  test relocatable.
- Assign each test to a tier by the access it requires, never by its subject, and never by widening the interface.
- Preserve behavior byte-for-byte. Equivalence is demonstrated by comparison against verified output, not asserted.

## Required amendments

This ADR does not itself amend the artifacts below. Each amendment is the technical owner's act, recorded at
approval, and `WO-MOK-003` makes all of them approval preconditions.

### `ARCH-MOK-001`

| Location | Current | Required |
|---|---|---|
| *Components and responsibilities*, closing paragraph | "…a small number of Rust modules in one binary crate." | "…a small number of Rust modules in one Cargo package, built as a library target and a thin binary target." |
| *Prohibited patterns* | "Separate crates or services without an approved requirement." | "Separate Cargo packages, workspaces, or services without an approved requirement. The library and binary targets of the single package are not separate crates in this sense." |
| *Quality attributes*, Simplicity | "one binary crate and the minimum modules needed…" | "one Cargo package with one library target and one thin binary target, and the minimum modules needed…" |
| *Quality attributes*, Testability | "…without launching external systems." | Add: "and the program's public contract can be tested from outside the implementation source files." |
| *Conformance checks* | "Confirm the program builds as one Rust binary crate." | "Confirm the program builds as one Cargo package with exactly one library target and one binary target, with an empty dependency table." |
| *Conformance checks* | — | Add: "Confirm the library target's public interface matches `SPEC-MOK-002` rule 5 exactly, and that no public item yields mutable or owned authoritative state." |
| `[relations] addresses` | four requirements | Add `REQ-MOK-016`, which is architecturally significant under the already-declared `public-interface-or-protocol` trigger. |
| `[relations] conforms_to` | `SPEC-MOK-001` | Add `SPEC-MOK-002`. This is required for consistency, not optional: an addressed requirement must be specified by a conforming specification. |
| `[decision_assessment] rationale` | — | Record that this decision is covered by `ADR-MOK-002` in addition to `ADR-MOK-001`. |
| *Related ADRs* | `ADR-MOK-001` only | Add `ADR-MOK-002`. |

The existing `decision_assessment` outcome stays `adr_required` and its trigger list needs no change; the triggers
this decision fires are already declared there.

### `ADR-MOK-001`

Its *Decision* bullet "Run the foundation as one Rust process and one binary crate" is narrowed to "Run the
foundation as one Rust process and one Cargo package, built as a library target and a thin binary target." Nothing
else in it changes.

The recommendation is an in-place amendment with a dated note, matching this repository's practice of amending
`SPEC-MOK-001` in place, because `ADR-MOK-001`'s substantive decision — option 3, engine authority, the immutable
observation and typed proposal boundary — is entirely unchanged and only its structural expression narrows.
Superseding it is the alternative and would be the right choice only if the owner judges that a reader must see the
original text unmodified. `ADR-MOK-001`'s own migration clause reserves supersession for replacing engine
authority, which this does not do. The decision is the technical owner's.

### `SPEC-MOK-001`

Its *Explicitly unspecified decisions* entry "Test organization and helper functions" is narrowed by a one-row
in-place amendment: file-internal organization and helper structure remain delegated, while target layout, public
interface, and test placement are governed by `SPEC-MOK-002`. Recorded in that specification's amendment record.

## Consequences

### Positive

- The program has a stated contract, and a tier of tests exercises it, so a break in the operator-visible surface
  fails a test that has no private access with which to repair itself.
- `ADR-MOK-001`'s prohibition on exposing mutable state becomes enforceable against a surface that is actually
  reachable, rather than vacuously true of a crate nothing can reach.
- Phase 4's additional output stream and Phase 5's provider adapter are written against one settled contract
  instead of negotiating one per phase.
- The engine's white-box tests keep their direct assertions, so the initiative's correctness risk stays low.
- The change is reversible in code by deleting three lines of `Cargo.toml` and re-inlining the public tier.

### Negative

- The public interface is a maintained contract from the moment it is written against, and that part is not cheap
  to reverse. This is the cost the decision accepts deliberately.
- `execute`'s shape — an argument iterator, two writers, a `u8` return — becomes public, and later phases inherit it.
- Two target names differ by case convention, which will read as an inconsistency to anyone who has not read rule 2.
- The suite is arranged in two tiers, so a future author must apply a placement rule rather than following one
  habit.
- `cargo test` builds more artifacts and `target/` grows.

### Operational and security

- No network access, credential, filesystem path, environment read, or wall-clock read is introduced.
- Every authorized public addition returns a copy of a value the program already prints, so the surface grants no
  capability that did not already exist inside the crate.
- The prohibition on exposing authoritative state holds in test builds as well as release builds, because no seam
  or feature is permitted to relax it.
- The observation-to-proposed-action trust boundary is unmoved, and nothing in the public interface reaches it.

### Migration

Behavior is preserved byte-for-byte, so there is no operator-facing migration. If a later requirement needs an item
the enumeration lacks, add it to `SPEC-MOK-002` rule 5 under that requirement's authority and record why it is
read-only; never widen an item to relocate a test. If a later requirement genuinely needs mutable engine access
from outside the crate, that reverses `ADR-MOK-001`'s decision on world authority and requires a superseding ADR,
not an addition to rule 5.

## Validation

- `Cargo.toml` declares exactly one `[lib]` and one `[[bin]]`, with the names and paths of `SPEC-MOK-002` rule 1,
  and an empty dependency table.
- `cargo clippy --all-targets --all-features -- -D warnings` reports no findings, including no `non_snake_case`
  finding against either target name.
- A reviewed inventory of every `pub` item reachable from the library root matches `SPEC-MOK-002` rule 5 exactly,
  with no surplus and no member added that no relocated test requires.
- Review confirms no public item returns a mutable borrow, an owned engine collection, a reference into
  engine-owned state, an iterator over engine-owned collections, a mutating trait object, or a closure holding
  engine state, and that `Simulation` has no public field.
- No `cfg` attribute, feature, or dev-dependency affects which items are public. The dev-dependency table is empty.
- `src/main.rs` declares no module and contains no test; `src/lib.rs` contains no simulation logic and no test.
- The public tier names no private type, function, module, or field, and compiles against the library target alone.
- One `cargo test` invocation runs both tiers, executes the same number of tests as before relocation, and reports
  no ignored test.
- Byte-for-byte comparison against output captured before the change, across the declared seeds, both decision
  sources, the declared densities, and both trace settings, shows no difference; exit codes match for every
  invalid-input case.
