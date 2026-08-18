+++
id = "ADR-MOK-004"
type = "adr"
title = "Observer library target with a provenance-closed interface, and one directory per package"
status = "approved"
owners = ["technical owner"]
created = "2026-08-18"
updated = "2026-08-18"

[relations]
decides = ["ARCH-MOK-002"]
+++

# ADR: Observer library target with a provenance-closed interface, and one directory per package

## Status

Accepted by the technical owner on 2026-08-18, together with `ARCH-MOK-002`'s amendment. It refines `ARCH-MOK-002`
on two points — the observer package's target shape and the repository's package-directory layout — and supersedes
nothing. `ARCH-MOK-001`, `ADR-MOK-001`, `ADR-MOK-002` and `ADR-MOK-003` are unaffected in substance: engine
authority, the trust boundary, the dependency direction, the two-package count and the measured dependency surface
are all exactly as decided.

The amendments this ADR requires are listed under *Required amendments* and are the technical owner's act. This ADR
does not claim them, and `WO-MOK-006` makes them approval preconditions in the same way `WO-MOK-003` and
`WO-MOK-005` did for their own chains. The owner's approval of this ADR on 2026-08-18 covered those four
amendments as stated here; `WO-MOK-006`'s *Approval record* states that reading, and it is disclosed in the
completion report rather than treated as settled.

## Context

`ARCH-MOK-002` describes the observer as "the new binary" and factors its presentation layer so that layout and
mapping are pure functions, "and therefore testable without a terminal". The second half of that sentence is
satisfied — 109 tests assert against in-memory buffers and none constructs a terminal — but only from inside the
binary. The architecture's quality attribute *Testability without a terminal* is met; its implied consequence, that
the presentation contract can be exercised from outside the component, is not, and cannot be: a Rust integration
test links a library target, and the observer has none.

Two facts make this cheap to correct, and both were measured on the merged tree rather than assumed.

First, the observer's cross-module items are already public. Its non-test code declares 97 public items across seven
modules — counting one declaration written `pub` as one item, as `SPEC-MOK-004` rule 6 defines it — and its own
cross-cutting suite reaches the others as `use crate::state::{Filter, Observer, Progression}`
and `use crate::{authority, export, layout, options, render, spatial}`. A library target that declares those modules
`pub` exposes exactly what is already `pub`. No item is widened, so `INT-MOK-003`'s prohibition — "widening an item
to `pub` in order to move a test is prohibited", restated by `INT-MOK-005` — is not engaged at all. This is the
material difference from `ADR-MOK-002`, which had to justify five additions one at a time because the engine's
interface is a trust boundary.

Second, the tests that cannot move are identifiable by rule rather than by taste. 32 of the 109 name a private item
of their own module or one of four `#[cfg(test)] pub fn` hooks on `Observer`. The rendering module declares 39
private items against 2 public ones, and 12 of its 20 tests assert drawing internals. The four hooks reach states a
run cannot produce — a rejected proposal, a world stripped of standing resources — and 16 tests use them. An
integration test links the non-test build, so those hooks do not exist for it.

Separately, the repository root's `Cargo.toml` is both the workspace manifest and the engine's package manifest, and
the engine's `src/` and `tests/` sit at the root beside `mokiterions-tui/`. `SPEC-MOK-003` fixed that deliberately —
clause 3 of its component layout says "the engine's sources are not relocated, so the `REQ-MOK-010` text stream does
not move" — which was the right call while the observer was being built and the text stream's verified behavior was
the thing most at risk. That risk is now discharged: `VREC-MOK-005` records the engine's whole test corpus and its
`src/main.rs` and `src/cli.rs` byte-identical across the observer's introduction. What remains is an asymmetry with
no remaining justification, and one that would be entrenched by giving the observer a `src/lib.rs` and a `tests/`
while the engine keeps root-level equivalents.

## Decision drivers

- The observer's verification suite must be able to fail because the observer's contract changed, which is the
  reason `INT-MOK-003` gave for the engine and which applies unchanged here.
- No item may be widened and no `#[cfg(test)]` attribute removed. A decision that buys a test tier with public
  surface is worse than the condition it corrects.
- The observer's interface is not a trust boundary. Treating it as one — enumerating 97 items and maintaining that
  enumeration — would cost more than it protects and would rot on the first refactor.
- Nothing an operator types or reads may change. The engine's binary name is in the first line of `USAGE`, which
  `SPEC-MOK-001` fixes and `VER-MOK-004` verifies.
- The engine's empty dependency set must survive the move, per-package and provably.
- Reversal must stay cheap. A library target and a directory move are both undoable without touching behavior.
- The change must be reviewable. A file move produces a diff in which every line is new, so the decision must be
  accompanied by a comparison mechanism rather than by a request to read a diff.

## Considered options

### Option 1: Leave the observer as one binary target and document the asymmetry

Record in `REPOSITORY_CONTEXT.md` that the two-tier convention applies to the engine only, and leave all 109
observer tests inline. Cost: nothing. Consequence: the cross-cutting suite behind `VER-MOK-005` stays unable to fail
for the reason it exists, and every later phase that adds a pane inherits that. This resolves the documentary
disagreement `WO-MOK-005` disclosed by weakening the convention rather than by meeting it, and it is the option that
leaves the repository with two structural standards permanently. Rejected.

### Option 2: Library target plus a feature-gated test-support seam, so every observer test moves

Add `#[cfg(feature = "test-support")] pub` to the private drawing helpers, the private layout constants and the four
hooks, and relocate all 109 tests. Consequence: the seam is enabled in exactly the build that exposes it, so gating
makes the exposure conditional in no sense that matters. `INT-MOK-003` excludes it by name, `SPEC-MOK-002` rule 6
excludes it for the engine and `ARCH-MOK-001` prohibits it. Adopting it for the observer would establish, in the
same repository, that the rule holds where it is convenient. Rejected, and rejected for the same reason
`ADR-MOK-002` rejected its own Option 2.

### Option 3: Library target declaring the already-public modules, tests split by required access, and one directory per package

Give the observer a library target whose public interface is exactly the items that were already `pub`, keep the
binary target's start-up and loop where they are, classify every test by the access it requires, and move the engine
package into `mokiterions-core/` so that each package's manifest, sources and tests are under one directory named
for it. No item is widened, no attribute is removed, no name changes, and no public item is added to either
package. **Selected.**

### Option 4: Library target with a thin binary, mirroring `SPEC-MOK-002` rules 3 and 4

Additionally move the observer's start-up and loop into a module inside the library and reduce `main.rs` to a shim,
as the engine did. Consequence: the engine's shim works because `execute` is public for a reason the binary itself
requires, and rule 7 could then place the four exit-code tests in the public tier. The observer's equivalent
function, `prepare`, returns a private `Launch`; a thin `main.rs` would need one public entry point, and the four
start-up tests would still reach `prepare` and `Launch` privately, so they would stay in the internal tier either
way. The option therefore adds a public item and a module boundary and moves no test. Rejected as cost without
effect — and if `prepare` and `Launch` were made public *so that* those four tests could move, that is widening for
a test, which Option 2's rejection already forbids.

## Decision

The observer package builds a library target and a binary target. The library target declares the package's seven
presentation modules; its public interface is exactly the set of items already public in the observer's non-test
code, and no item's visibility changes. The binary target keeps its start-up, its loop and its scheduling, and
reaches the modules through the library rather than declaring them again.

**The observer's public interface is closed by provenance, not by enumeration.** `SPEC-MOK-002` rule 5 closes the
engine's interface as a list of items, because each engine item is a potential path to authoritative state and must
be justified individually. The observer holds no authority: it cannot mutate world state, it cannot decide, and it
never reaches `Observation` or `DecisionSource`. So its interface is closed by the check "no item's visibility
differs from its visibility before the library target existed" — a property of a diff, decidable mechanically,
which cannot drift out of date and which enforces the no-widening prohibition directly rather than as a side
effect. Growth still requires an approved requirement, and the specification is amended in the same act.

**Every observer test is placed by the access it requires**, on `SPEC-MOK-002` rule 7's test verbatim: if the test
can be written using only the library target's public interface, with its assertions unchanged and with no item
widened, it is in the public tier; otherwise it is in the internal tier, beside its subject. The four `#[cfg(test)]`
hooks stay `#[cfg(test)]`, and the tests that use them are internal by definition.

**The cross-cutting internal tests stay inside the library crate.** The eight hook-using tests in the observer's
cross-cutting suite reach items in several modules and a `#[cfg(test)]` hook, so they belong to no single module's
internal tier and cannot leave the crate. They are declared from the library root as a `#[cfg(test)]` module. This
is the one place the observer's structure differs from the engine's, where `SPEC-MOK-002` rule 3 leaves `src/lib.rs`
with no test, and the reason is that the engine has no cross-module internal test and the observer has eight.

**Each package occupies one directory.** The engine package moves to `mokiterions-core/`, taking its manifest, its
`src/` and its `tests/`. The repository root's `Cargo.toml` becomes a workspace manifest and nothing else. The
observer's path dependency is re-pointed at the engine's new directory and stays keyed by the engine's package name.

**Nothing is renamed.** Package `Mokiterions`, library target `mokiterions`, binary target `Mokiterions`, observer
package and binary `mokiterions-tui`. The observer's library target is named in snake case, as `SPEC-MOK-002` rule 2
requires of the engine's and for the identical reason: the declared lint gate implies `non_snake_case`. The
directory name `mokiterions-core` is a directory name and is not a package name; it is chosen because it says which
package it contains, and because an earlier revision of `WO-MOK-005` that renamed the *package* to
`mokiterions-core` was reverted precisely so that the operator-facing names would not move.

## Required amendments

Each is the technical owner's act. `WO-MOK-006` makes all four approval preconditions.

### `ARCH-MOK-002`

- *Components and responsibilities* item 4 calls the observer host "the new binary". Amend to state that the
  observer package builds a library target carrying the presentation layer and a binary target carrying start-up,
  the loop and scheduling, and that the host role is the binary target's.
- *Quality attributes*, *Testability without a terminal*: extend to state that the presentation contract is
  exercised from outside the component, through the library target, and not only in memory from inside it.
- *Required patterns*: add that the observer's public interface is closed by provenance — no item's visibility
  changes — and that observer tests are placed by required access.
- *Prohibited patterns*: add that no item is widened and no `#[cfg(test)]` attribute removed in order to relocate a
  test, and that the observer's binary target does not declare the presentation modules a second time.
- *Conformance checks*: add a check that the observer's public interface differs from its pre-change visibility in
  no item, a check that both tiers run under one `cargo test` with no terminal, and a check that each package's
  manifest, sources and tests are under one directory named for that package.
- `addresses`: add `REQ-MOK-028`. `conforms_to`: add `SPEC-MOK-004`.
- `decision_assessment.rationale`: record that the target shape of the observer package and the repository's
  package-directory layout are decided by this ADR, which fires the already-declared public-interface-or-protocol
  and material-alternatives triggers and leaves the boundary, the dependency direction and non-perturbation
  untouched, so the assessment stays `adr_required` and is covered by `ADR-MOK-003` and `ADR-MOK-004` together.

### `SPEC-MOK-003`

- *Component layout*: the tree and clauses 3 and 4. Clause 3 states that the engine's sources are not relocated;
  it is the provision this ADR reverses, and the reason it existed — protecting the `REQ-MOK-010` text stream while
  the observer was built — is discharged by `VREC-MOK-005`. Clause 4's names are unchanged and stay.
- *Data and interface contracts* rule 2 argues that narrowing `Simulation::run` away "would mean relocating the
  engine's sources, which the component layout below forbids". The sources are now relocated and `run` is still on
  the interface, because it is the `REQ-MOK-010` whole-run entry point that the engine's binary calls. The clause's
  reasoning is stale even though its conclusion holds, and its path reference changes.
- *Explicitly unspecified decisions*: the grant of "test organization, fixtures and helpers" is narrowed for the
  observer by `SPEC-MOK-004`, and the withholding of "the package layout" is what makes this ADR necessary rather
  than optional. Both entries are amended to point at `SPEC-MOK-004`.

### `SPEC-MOK-002`

- Every path clause: *Inputs*, rule 1's target table, rule 3, rule 4, rule 5's `grep` check, rule 8's file table
  and rule 9's locations. Each names a root-relative path that moves under `mokiterions-core/`.
- Rule 1's amendment note says the engine package is "this one at the root, unchanged in package name, in both
  target names and in source location". The package name and both target names are still unchanged; the source
  location is not.
- *Scope* and *Compatibility and migration*: record that `SPEC-MOK-004` owns the repository's package-directory
  layout, and that rules 7 to 10 remain the engine package's test-placement contract while `SPEC-MOK-004` is the
  observer's.

### `REPOSITORY_CONTEXT.md`

- *Commands* and *Architecture*: the engine's paths and entry points.
- *Repository constraints*, test placement: the convention is stated repository-wide on engine-scoped authority.
  Amend it to state both packages' tiers and to cite `SPEC-MOK-002` rules 7 to 10 for the engine and
  `SPEC-MOK-004` for the observer. This is the sentence whose disagreement with the code `WO-MOK-005` disclosed.

## Consequences

### Positive

- The observer's contract is stated and exercised. A change to what it presents, exports, lays out or maps breaks a
  test that has no private access to repair itself with.
- `VER-MOK-005`'s cross-cutting suite moves from inside the binary to outside the crate for the 16 of its 24 tests
  that can make the move, so the majority of the observer's assurance rests on the contract rather than on the
  interior.
- The no-widening prohibition becomes checkable rather than reviewed: a visibility diff answers it.
- The repository states one test convention that is true of both packages, on authority that covers both, which
  closes `WO-MOK-005`'s fifteenth disclosure without weakening anything.
- Either package's files are found from its name, and workspace-wide settings become separable from the engine's
  package settings.
- The engine's empty dependency set is re-demonstrated per package after the move, so the property is re-measured
  rather than inherited.

### Negative

- 32 observer tests stay in the internal tier, so the observer's suite is split in a way a reader must understand
  the rule to predict. The rule is stated, but it is one more rule.
- The observer's `main.rs` stays 431 lines and is not thin, so the two packages' binary targets are shaped
  differently. Option 4 is the alternative and it buys nothing.
- The library root carries a `#[cfg(test)]` cross-cutting module, which `SPEC-MOK-002` rule 3 forbids for the
  engine. The asymmetry is deliberate and narrow, and it is stated rather than left to be noticed.
- `cargo test` builds more artifacts: a library target, a binary target that links it, and one test binary per
  public-tier file in each package. `target/` grows.
- A bare `cargo build` or `cargo test` at a virtual workspace root does not resolve the way it does at a package
  root. The operator-facing forms must be re-verified, and `REPOSITORY_CONTEXT.md`'s command list must be correct
  rather than approximately correct.
- The move touches nearly every path reference in the specification set, which is a large documentary change for an
  outcome with no behavioral content.

### Operational and security

- No network access, credential read, environment read, or wall-clock read is introduced. Neither package gains a
  dependency.
- The trust boundary is unmoved. A decision source still receives immutable observations and returns typed
  proposals; the observer's public interface reaches neither, and `Observation` and `DecisionSource` stay among the
  ten names `SPEC-MOK-002` rule 6 keeps private.
- World authority is unchanged. The observer's interface carries presentation types, layout and mapping functions,
  the event buffer, read-only accessors and the export writer. None of them can mutate a world, and the engine's
  interface gains nothing.
- The four `#[cfg(test)]` hooks stay compiled out of every shipped artifact, and are now additionally unreachable
  from the observer's own public tier, which is a strictly stronger position than today.
- Export behavior is untouched: an operator-supplied path stays data, is never interpreted as code and is never
  used to read.

### Migration

- One work order, `WO-MOK-006`, performs the move, the library target and the test relocation together, because a
  library target without the relocation leaves the suite where it is and a relocation without the move entrenches
  the asymmetry the same work order is correcting.
- Equivalence is demonstrated by comparison, not by review: relocated content compared byte for byte against its
  source at the predecessor commit, and both packages' outputs compared against verified baselines.
- Records bound to commits are not edited. `VREC-MOK-001` through `VREC-MOK-005` and the retained evidence under
  `WO-MOK-001` through `WO-MOK-005` name old paths and stay as they are; a superseding requirement-to-test mapping
  is produced as `WO-MOK-006` evidence.
- Reversal is a directory move and the deletion of one file. Nothing in either package's behavior depends on the
  outcome.

## Validation

- The observer package declares exactly one library target and one binary target, and no build script.
- The visibility of every item in the observer's non-test code is identical to its visibility at the predecessor
  commit, checked as a diff rather than asserted.
- The four `#[cfg(test)]` hooks are still `#[cfg(test)]`, and no public-tier test names one.
- The observer's binary target declares no presentation module and reaches them through the library target.
- Each package's manifest, sources and tests are under one directory named for that package, and the root manifest
  declares no package.
- `cargo test` runs every tier of both packages with no terminal present, no feature, no environment variable and
  no `#[ignore]`, and the executed test count matches the pre-change count in each tier of each package.
- `cargo tree -p Mokiterions` resolves to the engine package alone.
- `cargo run --bin Mokiterions` runs the engine binary and its `USAGE` first line is byte-identical to the verified
  text; `cargo run -p mokiterions-tui` runs the observer.
- The engine's text stream, summary line and exit codes are byte-identical to the verified baseline at every
  declared seed and density, with and without `--trace-actions`, under both decision sources.
- The observer's frames and exports are identical to the pre-change observer's at every declared seed and viewport.
