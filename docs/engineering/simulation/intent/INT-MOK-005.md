+++
id = "INT-MOK-005"
type = "intent"
title = "Give each package its own directory and its own tested public contract"
status = "approved"
owners = ["product owner"]
created = "2026-08-18"
updated = "2026-08-18"

[relations]
+++

# Intent: Give each package its own directory and its own tested public contract

## Problem

The repository now holds two packages, and they are not built the same way. One of them was restructured under
`INT-MOK-003` so that its contract is stated and its tests are placed by the access they require. The other was
never restructured, because it did not exist yet.

**The observer has no stated contract and no test that reaches it from outside.** All 109 of its tests are
`#[cfg(test)] mod tests` blocks inside its implementation files, because `mokiterions-tui` builds one target — a
binary — and a Rust integration test links a library target. `mokiterions-tui/tests/` would not compile against
anything. This is not a placement choice that went badly; it is the same structural absence `INT-MOK-003`
identified in the engine, in a package that arrived after that intent was closed.

The consequence is the one `INT-MOK-003` already argued. 1,255 of those lines are `verification.rs`, the whole
cross-cutting suite behind `VER-MOK-005`: non-perturbation, export fidelity, presented-value fidelity, the
authority verdict, colour independence. It asserts what the observer promises, and it does so from inside the
binary, where a change to the observer's own surface cannot break it. `WO-MOK-005`'s completion report discloses
this as its fifteenth finding, and `VREC-MOK-005` records it as the eleventh item an assurance owner must weigh.

**The two packages also disagree about where a package's files live.** The engine's sources are at `src/` and its
tests at `tests/`, both at the repository root, because the root was the whole repository when they were written.
The observer's are under `mokiterions-tui/`. So the root directory holds one package's implementation and the
other package's directory side by side, and neither `Cargo.toml` at the root is only a workspace manifest nor
`src/` is only the engine's — the same file is the workspace root and the engine's package manifest. A reader
looking for the engine finds it by knowing history, and a reader looking for "the engine's tests" finds a
directory whose name says nothing about which package it belongs to.

Two documents already point opposite ways about the first problem. `SPEC-MOK-003` grants the implementation "test
organization, fixtures and helpers" under its *Explicitly unspecified decisions*, and the implementation took that
grant. `REPOSITORY_CONTEXT.md` states the two-tier convention as a repository constraint, and cites
`SPEC-MOK-002` rules 7 to 10 as its authority — a specification whose *Scope* says "Every rule below is a rule
about the engine package". The convention is stated repository-wide, its authority is engine-scoped, and the
specification that governs the observer hands the decision away. Nothing is being violated. There is also nothing
that requires the observer's tests to reach a contract, and no contract for them to reach.

`INT-MOK-003` is achieved and verified for the engine. It does not cover the observer, and a completed intent is
not the place to record a second package's structure.

## Desired outcomes

- The observer has an enumerated public interface, and a reader can tell from that enumeration what the package
  promises to anything outside itself.
- A tier of observer tests reaches the observer only through that interface, so a change to what the observer
  presents breaks a test rather than passing quietly.
- Observer tests that assert the presentation layer's internals stay next to the code they cover, for the same
  reason the engine's do.
- Each package's manifest, sources and tests are found under one directory named for that package, and the
  repository root holds the workspace manifest and no package's implementation.
- The repository states one test-placement convention that is true of both packages, on authority that covers
  both.
- Approved behavior is untouched. Every operator-facing name is unchanged, identical inputs produce
  byte-identical output, an identical final state and an identical exit code, and the observer presents the same
  frames.

## Actors and stakeholders

- The product owner accepts that this initiative changes no product behavior and consumes engineering capacity
  for an engineering outcome, as `INT-MOK-003` did.
- The technical owner owns the specification and architecture consequences: the observer's target shape, the
  package layout, and the amendments that permit both.
- Developers and implementation agents relocate files and tests and maintain the observer's public interface.
- Assurance reviewers confirm equivalence by comparison against verified output rather than by re-reading code.
- Operators are unaffected and must observe no difference of any kind, including in the commands they type and
  the names of the binaries they run.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Observer tests reaching the observer only through a stated public interface | 0 | ≥ 70 | Automated verification |
| Observer items widened to `pub` in order to relocate a test | 0 | 0 | Every build |
| Packages whose manifest, sources and tests are under the package's own directory | 1 of 2 | 2 of 2 | Static check |
| Automated tests present before and after the change | 169 | 169 | Automated verification |
| Engine tests, and their placement across the two tiers | 60 (37 + 23) | 60 (37 + 23) | Automated verification |
| Output bytes differing from the verified baseline at an identical seed and density | 0 | 0 | Automated verification |
| Operator-facing names changed — package, both engine targets, observer package and binary | 0 | 0 | Every build |
| Engine package external dependencies | 0 | 0 | Every build |
| Tests requiring a feature flag, environment variable, or `#[ignore]` to run | 0 | 0 | Every `cargo test` |

The first target is stated conservatively. Measured on the merged tree, 77 of the 109 observer tests name no
private item of their own module and no test-only hook, so 77 is what the placement rule is expected to move; the
target leaves room for a test that the rule reclassifies once it is actually written against the interface. The
measurement and its method are recorded in the implementing work order rather than asserted here.

## Non-goals

- Any change to simulation behavior, constants, event fields, event order, exit codes, diagnostic wording,
  `USAGE` text, key bindings, layout tiers, glyphs, or anything else an operator can see. This initiative is an
  equivalence-preserving restructuring and nothing else.
- Renaming anything. The engine package stays `Mokiterions`, its library target stays `mokiterions`, its binary
  stays `Mokiterions`, and the observer package and its binary stay `mokiterions-tui`. A directory moves; no name
  does.
- Relocating every observer test. Tests that reach the presentation layer's internals stay where they are.
- A test-support seam, feature-gated or otherwise, that exposes either package's internals outside its crate.
  The four `#[cfg(test)]` hooks in the observer stay `#[cfg(test)]`, and the tests that use them stay inline.
- Widening any item to `pub`, in either package, in order to relocate a test.
- A third package, a service boundary, a separate release artifact, or any new dependency in either package.
- Restructuring the engine's own tests. Their two-tier placement is `WO-MOK-003`'s outcome and is verified; this
  initiative changes the directory they sit in and nothing else about them.
- Splitting `render.rs`, `state.rs` or any other module for the sake of the move.
- Correcting the artifact defects `WO-MOK-005` disclosed. They are real and they are separate.

## Principles and immutable constraints

- The simulation engine remains the only authority over world state, and the observer remains unable to change
  it. Nothing here touches either property.
- A test that needs internal access is evidence that the test belongs beside the code, never a reason to open the
  boundary. Widening an item to `pub` in order to move a test is prohibited. This is `INT-MOK-003`'s principle,
  restated because it now has to hold in a second package.
- The observer's public interface is not a trust boundary and must not be treated as one. The engine's is, which
  is why `SPEC-MOK-002` enumerates the engine's item by item. The observer holds no authority, so its interface
  is closed by a different and stronger check: no item's visibility changes.
- The public interface is a ceiling, not a wish list. It grows only when an approved requirement needs it to.
- One command runs everything. `cargo test` covers every tier of both packages, and no tier is optional or
  conditional.
- Output bytes are frozen. Equivalence is demonstrated by comparison against verified output, not asserted.
- Operator-facing names are frozen. `cargo run --bin Mokiterions` and `cargo run -p mokiterions-tui` keep
  working, and the first line of `USAGE` is untouched.
- Keep it small. This is a directory move, a library target that re-declares modules that are already `pub`, and
  a relocation of tests that already pass.

## Risks and assumptions

- Fact: every cross-module item the observer's own cross-cutting suite reaches is already `pub`. Measured on the
  merged tree, the observer's non-test code declares 97 public items across its seven modules, and
  `verification.rs` imports them as `use crate::state::{Filter, Observer, Progression}` and
  `use crate::{authority, export, layout, options, render, spatial}`. Giving the package a library target that
  declares those modules `pub` therefore requires no widening at all, which is what makes this initiative
  compatible with the prohibition above rather than an exception to it.
- Fact: four `#[cfg(test)] pub fn` hooks on `Observer` — `select_for_test`, `set_overlay_for_test`,
  `replace_decisions_for_test` and `replace_snapshot_for_test` — are used by 16 tests. An integration test links
  the non-test build, so those hooks will not exist for it. Ungating them would be exactly the test-support seam
  the non-goals exclude. Those 16 tests therefore stay inline, and that is the placement rule working, not a
  compromise with it.
- Fact: 32 of the 109 observer tests name a private item of their own module or one of those hooks — 12 in
  `render.rs`, 8 in `verification.rs`, 8 in `main.rs`, 4 in `state.rs`. `render.rs` declares 39 private items and
  2 public ones, so most of its tests assert drawing internals and belong inline on the rule's own terms.
- Fact: the observer's binary target may keep its start-up and its loop. Nothing requires the observer's
  `main.rs` to become thin the way `SPEC-MOK-002` rule 4 required the engine's, because the engine's four
  exit-code tests were promoted to its public tier and the observer's four start-up tests are not: they reach a
  private `prepare` and a private `Launch`, and promoting either would be widening for a test. Leaving them
  inline is what keeps the addition count at zero.
- Fact: the repository root's `Cargo.toml` is both the workspace manifest and the engine's package manifest.
  Moving the engine into its own directory makes the root a virtual manifest, which changes how Cargo resolves a
  bare `cargo build`. The operator-facing forms must be re-verified rather than assumed, in particular
  `cargo run --bin Mokiterions`, and that verification belongs in the verification contract.
- Risk: relocating a test is an opportunity to weaken it. The mitigation is `INT-MOK-003`'s: a relocated test
  keeps its assertions verbatim, and a test that cannot is misclassified and stays inline.
- Risk: a file move produces a diff in which every line is new, so a reviewer cannot see what changed. The
  mitigation is a measured additivity proof — the relocated content compared byte for byte against its source —
  rather than a reviewed diff.
- Risk: the engine's `tests/` directory is named in a verified record. `VREC-MOK-003` binds `WO-MOK-003`'s
  evidence, and `VER-MOK-001`, `VER-MOK-002`, `VER-MOK-003` and `VER-MOK-004` name paths. Those records are
  bound to their commits and must not be edited; a superseding mapping is this initiative's evidence.
- Open decision: the technical owner must amend `SPEC-MOK-002` (its path clauses), `SPEC-MOK-003` (its component
  layout and its unspecified-decisions list) and `ARCH-MOK-002` (the observer's target shape). Those amendments
  are the instrument for this initiative, and no implementation may begin before they are settled.
- Assumption: `Cargo.lock` changes only where a path changes, and no version resolves differently. The engine's
  dependency table stays empty and the observer's stays at one direct dependency.
- Assumption: a library target for the observer adds a compilation unit and one test binary per public-tier
  file, so `cargo test` builds more artifacts and `target/` grows. Runtime behavior is unaffected.
