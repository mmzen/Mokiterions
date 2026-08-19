+++
id = "SPEC-MOK-004"
type = "specification"
title = "Package directories, observer targets, and observer test placement"
status = "approved"
owners = ["technical owner"]
created = "2026-08-18"
updated = "2026-08-19"

[relations]
specifies = ["REQ-MOK-028", "REQ-MOK-029", "REQ-MOK-030"]
+++

# Specification: Package directories, observer targets, and observer test placement

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-18 | Original content for `REQ-MOK-028`, `REQ-MOK-029` and `REQ-MOK-030`. | Approved by the technical owner; implemented under `WO-MOK-006` and verified under `VREC-MOK-006`. |
| 2026-08-19 | Recorded figures and two subject lines corrected, because `SPEC-MOK-003` rule 5 as amended the same day replaced the observer's four-row layout-tier table with one threshold per pane, and the implementation that conforms to it removes the `layout` module's `Tier` enum, that enum's `label` method, the `tier_for` function and the `Panes::tier` field. **Rule 6**: the recorded extent falls from 13 `layout` items to **10** and from **97** items to **94**; the same interface counted the other way falls from 122 to **118**, and its public fields from 25 to **24**. What the field figure counts is now stated, because 25 is reproducible only as 122 − 97 — it counts public fields, and the variants of a public enum are not written `pub` and were never in it. The `layout` row's subject, "viewport tiers and the pane geometry of each", is corrected: there are no tiers to name. A **Reduction** clause is added, symmetric with the existing **Growth** clause, so that removal is governed the way addition already was. The byte-identity check is scoped to the `WO-MOK-006` restructuring it was written for, exactly as `SPEC-MOK-002` rule 3's freeze was scoped to `WO-MOK-003`; unscoped it forbids the observer's code from ever changing again, which was never its subject and is not what rule 6 is for. **Rule 9**: `tests/layout.rs` rises from 7 tests to **10** and the public tier from 77 to **80**. The +3 is a net figure and the rule now carries the measured composition: three tests go, because they assert the tier table and the per-tier minimums that rule 5 no longer defines, and six arrive — the rename of the third departure, the per-pane threshold case, the ten-row log case and the monotonicity sweep that `VER-MOK-005` as amended requires, and two one-to-one threshold cases that its **Mapping injectivity** property needs once rule 5 creates a second placement regime. **Rule 11**: the observer's executed total rises from 109 to **112** and the workspace's from 169 to **172**, with the "same before and after" clause scoped to the restructuring. **Rule 12**: its second paragraph is scoped to the `WO-MOK-006` restructuring, exactly as rule 13's clause is and for the same reason — unscoped it forbids any later work order from renaming a test in either package, which is a freeze on test maintenance the rule was not written to impose, and it would make `excluded_panes_are_the_ones_the_tier_omits` unrenamable after the term "tier" was deleted from the specification it cites. The assertion itself is unchanged; only the name and the rule it names change. **Rule 13** and the second worked example no longer say "layout tier". No rule changes what it requires. No target, target name, path, package name, tier boundary, hook or prohibition changes, and no item's visibility widens: the interface only loses items, so rule 7 is satisfied by construction. | **OUTSTANDING.** Requires the technical owner. The repository owner approved the `SPEC-MOK-003` rule 5 amendment that authorizes the removal and directed the implementation in the same act on 2026-08-19, but was not shown this consequence: the implementation agent found it after the fact, by measuring the interface against this rule, and has reported it rather than treating the rule 5 approval as covering it. Records bound to commits are not re-opened — `VREC-MOK-006` measured 97 items and 169 tests and both were correct at its commit. |

## Scope

This is the structural contract for the **repository's package layout** and for the **terminal observer package**:
where each package's manifest, sources and tests live; which targets the observer package builds; exactly what its
library target makes public; and where every automated test of that package lives.

It stands to the observer package as `SPEC-MOK-002` stands to the engine package, and it takes over the two subjects
`SPEC-MOK-003` withheld or delegated:

- `SPEC-MOK-003`'s *Explicitly unspecified decisions* withholds "the package layout", so no approved artifact fixes
  it. This specification fixes it, for both packages.
- The same section grants "test organization, fixtures and helpers" to the implementation agent. Rules 8 to 11 below
  narrow that grant for the observer package to file-internal organization and helper structure, exactly as
  `SPEC-MOK-002` rules 7 to 10 narrowed `SPEC-MOK-001`'s equivalent grant for the engine.

It states no simulation behavior and no presentation behavior. `SPEC-MOK-001` remains the single behavior contract
for the engine and `SPEC-MOK-003` for the observer, and rule 13 below binds this specification to preserving both
byte for byte.

It changes nothing about the engine package other than which directory holds it. The engine's targets, its target
names, its closed public interface and its two test tiers stay exactly as `SPEC-MOK-002` states them; every clause
of that specification that names a root-relative path is amended to the new directory and nothing else. Rule 3
below is the only rule here that touches the engine, and it moves files.

## Actors and external systems

- Cargo, which decides what a workspace member is, what a target is, what an integration test may link, and how a
  bare command resolves at a virtual workspace root.
- Clippy under `-D warnings`, whose `non_snake_case` lint constrains the observer's library target name exactly as
  it constrains the engine's.
- Implementation agents and developers, who locate a package's files, place tests, and maintain the observer's
  public interface.
- The operator, whose commands and whose output must not change.
- No external service, network endpoint, credential, or filesystem location outside the repository participates.

## Inputs

The repository root's `Cargo.toml`; each package's `Cargo.toml`; the engine's source files and the test files under
its `tests/` directory; the observer's ten source files and the test files under its `tests/` directory;
`Cargo.lock`; and the commands `cargo build`, `cargo test`, `cargo run`, `cargo tree`, `cargo fmt` and
`cargo clippy --all-targets --all-features -- -D warnings`, in both their workspace-wide and their `-p` forms.

## Outputs

Two library artifacts and two executable artifacts. Test binaries: one per package per internal-tier target, and one
per public-tier file. The observable output of both packages, which this specification requires to be unchanged.

## State model

This specification governs no runtime state. Its subject is the compile-time arrangement of the repository, which
has two states: conformant, when every rule below holds, and non-conformant, when any does not. There is no partial
or transitional state — a build either satisfies the rules or fails a conformance check.

## Behavioral rules

### 1. Repository layout

Each package's manifest, sources and tests are under one directory named for that package. The repository root
holds the workspace manifest, the lock file, the repository-level configuration and documentation, and no package.

```text
Cargo.toml                     # workspace manifest only; declares no package
Cargo.lock
mokiterions-core/              # the engine package, whose name stays `Mokiterions`
  Cargo.toml
  src/                         # lib.rs, main.rs, cli.rs, simulation.rs
  tests/                       # the engine's public tier, five files
mokiterions-tui/               # the observer package, whose name stays `mokiterions-tui`
  Cargo.toml
  src/                         # lib.rs, main.rs, and the eight module files
  tests/                       # the observer's public tier, eight files
```

`mokiterions-core` is a directory name and is not a package name. It is chosen so that the directory says which
package it holds; the package inside it is `Mokiterions`, unchanged. The directory is not named `Mokiterions`
because a directory named for the binary invites the belief that renaming one renames the other, and because an
earlier revision of `WO-MOK-005` that renamed the package itself was reverted precisely to keep the operator-facing
names still.

No third package directory, no nested workspace, and no directory holding the sources of more than one package.

### 2. Workspace manifest

The root `Cargo.toml` declares `[workspace]` with `members = ["mokiterions-core", "mokiterions-tui"]` and
`resolver = "3"`, and declares no `[package]`, `[lib]`, `[[bin]]`, `[dependencies]` or `[workspace.dependencies]`
table. The resolver is stated explicitly because a virtual manifest does not inherit a member's edition default, and
an unstated resolver is both a behavioral difference and a build-time warning.

A `default-members` or a member-level `default-run` key is permitted if, and only if, rule 14's command check shows
that a form the operator uses no longer resolves without it. It is then the only addition, and the resolution it
restores is stated in the manifest as a comment. A rename is never the correction.

`Cargo.lock` stays at the repository root. No dependency resolves to a different version as a result of this
layout; the lock file changes only where a recorded path changes.

### 3. Engine package relocation

The engine package's manifest, its `src/` directory and its `tests/` directory move under `mokiterions-core/`. The
`[package]`, `[lib]`, `[[bin]]` and `[dependencies]` tables move with the manifest unchanged, including the empty
dependency table and the target paths `src/lib.rs` and `src/main.rs`, which are package-relative and therefore
already correct.

Every `SPEC-MOK-002` rule continues to bind the engine package with its paths read under `mokiterions-core/`. The
package name stays `Mokiterions`, the library target stays `mokiterions`, the binary target stays `Mokiterions`, the
dependency and dev-dependency tables stay empty with no exception, and rules 7 to 10 of that specification remain
the engine's test-placement contract. This specification states nothing about which tier an engine test is in.

The observer package's dependency on the engine becomes `Mokiterions = { path = "../mokiterions-core" }`. It stays a
path dependency, stays keyed by the engine's package name, and gains no feature and no version requirement. The
observer's `ratatui` dependency, its version and its feature set are unchanged; `SPEC-MOK-003` remains their
authority.

No file is renamed while it is moved, and no module is split, merged or reordered as part of the move.

### 4. Observer targets

The observer package name stays `mokiterions-tui`. It declares exactly two targets:

| Target | Kind | Name | Path |
|---|---|---|---|
| Library | `[lib]` | `mokiterions_tui` | `mokiterions-tui/src/lib.rs` |
| Binary | `[[bin]]` | `mokiterions-tui` | `mokiterions-tui/src/main.rs` |

No third target and no build script. The library target is named in snake case for the reason `SPEC-MOK-002` rule 2
gives for the engine's: the declared lint gate implies `non_snake_case`, and a library crate named
`mokiterions-tui` is not a legal crate name at all. `mokiterions_tui` is also the name Cargo would derive, so the
declaration is explicit rather than novel.

The binary target keeps the name `mokiterions-tui`. It is the operator-facing command, it appears in the observer's
own usage text, and `SPEC-MOK-003` fixes it.

### 5. Observer module ownership

`mokiterions-tui/src/lib.rs` declares, and contains nothing else:

```rust
pub mod authority;
pub mod export;
pub mod layout;
pub mod options;
pub mod render;
pub mod spatial;
pub mod state;

#[cfg(test)]
mod verification;
```

It defines no item, holds no state and declares no test of its own. The `verification` module is declared here and
not from the binary target because rule 10 places the cross-cutting internal tests inside the library crate.

`mokiterions-tui/src/main.rs` keeps its contents: the start-up path, the `Launch` decision, `fn main`, the event
loop, the frame and input scheduling, the idle calculation, the diagnostic report, and its own `#[cfg(test)] mod
tests`. It declares no module. It reaches the presentation layer through the library target — `use mokiterions_tui::…`
in place of `crate::…` — and it compiles no module a second time.

The **seven** `pub mod` files keep their contents. Outside their `#[cfg(test)]` blocks they are byte-identical to
their content at the predecessor commit; inside them, tests leave under rule 9 and no test's assertions change under
rule 12. A module's internal cross-references stay written as `crate::…`, which remains correct inside a library
crate.

`mokiterions-tui/src/verification.rs` is the exception, and it is an exception only because the whole file is test
code: it is declared `#[cfg(test)]`, so it has no non-test content for the byte-identity clause to bind. Its module
documentation and its import list change as the 16 tests of rule 9 leave, and an import left unused by that departure
is removed rather than allowed. The eight tests it keeps are unchanged under rule 12.

The binary target is not required to become thin, and rule 5 of `SPEC-MOK-002` is not applied to it. The engine's
binary is a shim because `execute` is public for a reason the binary itself requires and because that made four
exit-code tests public-tier. The observer's equivalent function returns a private type, so a shim would add a
public item and relocate no test; see `ADR-MOK-004`'s Option 4.

### 6. The observer's public interface, closed by provenance

**The library target's public interface is exactly the set of items that were public in the observer's non-test code
at the predecessor commit.** No item is added to it, no item is removed from it, and no item's visibility changes.

**Scope of the byte-identity check, as amended 2026-08-19.** The clause above and the diff check below are the
`WO-MOK-006` restructuring's obligation: they say that turning already-public items into a maintained contract added
nothing, removed nothing and widened nothing. They are not a freeze on the observer's code for all later work, in the
same way that `SPEC-MOK-002` rule 3's freeze on `src/simulation.rs` is scoped to the `WO-MOK-003` restructuring it
was written for. Afterwards the interface is governed by the **Growth** and **Reduction** clauses below and by rule 7,
whose no-widening prohibition stands unscoped and at every commit.

The set is closed by provenance rather than by enumeration, and the check is a property of the diff: for each of the
seven `pub mod` files, the content outside every `#[cfg(test)]` block is byte-identical to the predecessor commit's.
Byte-identity implies that no `pub`, `pub(crate)`, `pub(super)` or private item changed, so the no-widening
prohibition of rule 7 is enforced directly rather than as a consequence of a list someone maintains. `verification.rs`
is not part of this check and cannot be: it is `#[cfg(test)]` in its entirety and declares no item at all, so it can
neither hold nor widen a member of the interface.

Its measured extent, recorded so that a later reader can tell whether the interface has grown and so that
verification has a figure to compare against. The figures are the `WO-MOK-006` predecessor commit's, amended
2026-08-19 for the one reduction there has been since.

**What is counted.** One **item** is one declaration written `pub` outside every `#[cfg(test)]` block — a `pub fn`,
`pub struct`, `pub enum`, `pub const`, `pub static`, `pub type`, `pub trait` or `pub use`. A public field of a public
struct and a variant of a public enum are parts of the item that declares them and are not counted again; there are
**24** public fields, and a count of **118** is the same interface counted the other way — every line that writes
`pub`, which is one per item plus one per public field. A variant of a public enum is written without `pub` and is in
neither figure, so a count that includes variants is larger than 118 and is not this figure; that is what the
2026-08-19 amendment settled, since the superseded pair 25 and 122 was reproducible only as 122 − 97. The four
`#[cfg(test)]` hooks are excluded, because rule 7 keeps them out of the library target. Verification states which
rule it counted by, and a figure derived under the other rule is not a shortfall.

| Module | Public items | Subject |
|---|---|---|
| `authority` | 5 | the engine's verdict and its presentation |
| `export` | 3 | the export writer and its rendered form |
| `layout` | 10 | the viewport floor, the pane thresholds and the geometry each one yields |
| `options` | 8 | the observer's own argument handling |
| `render` | 2 | the frame entry points |
| `spatial` | 19 | world-to-canvas mapping |
| `state` | 47 | the observer's state, its accessors, its filters and its event buffer |
| **Total** | **94** | |

The `layout` row read 13 items and "viewport tiers and the pane geometry of each" until the 2026-08-19 amendment. The
three items it lost are the `Tier` enum, that enum's `label` method and the `tier_for` function, together with the
`Panes::tier` field, which is the one of the 25 recorded public fields that goes. `SPEC-MOK-003` rule 5 as amended is
the authority: it decides each pane on one threshold in one axis, so there is no configuration left to name, to label,
or to return.

`SPEC-MOK-002` rule 5 closes the engine's interface as a list of items and justifies each one, because every engine
item is a potential path to authoritative state. That is not the situation here, and rule 6 of *this* specification
is deliberately weaker in form and stronger in effect: weaker, because it names no item; stronger, because it admits
nothing at all. The observer holds no authority over world state, so there is no admission to justify — and a
94-row table would have to be re-derived on every refactor that renames a private helper, which is a maintenance
burden that buys nothing.

**Growth.** The interface grows only when an approved requirement needs it to grow, and this rule is amended in the
same act, recording the added items and the requirement that authorizes them. A test is never that requirement.

**Reduction, added 2026-08-19.** The interface shrinks only when an approved requirement, or an approved amendment to
the specification that states it, removes the need for an item, and this rule is amended in the same act, recording
the removed items, the figures they change and the authority that removes them. Neither convenience nor a refactor is
that authority: an item that no longer has a caller is still part of the contract until the amendment says otherwise,
and the module's own tests are never the reason. A reduction cannot widen anything, so it is never in tension with
rule 7; what it can do is silently break a reader who relies on this rule's figures, which is why the figures are
amended and not merely re-measured.

### 7. Prohibited

- No item's visibility is widened. In particular no private, `pub(crate)` or `pub(super)` item becomes `pub`, and
  no item becomes `pub(crate)` from private, in order to relocate a test.
- No `#[cfg(test)]` attribute is removed, and no `#[cfg(test)]` item becomes unconditional. The four test-only hooks
  on the observer's state type — the ones that select a subject, set an overlay, replace the decision list and
  replace the snapshot — stay `#[cfg(test)]`, so they are absent from the library target and unreachable from any
  test outside the crate.
- No feature flag, `cfg` attribute, self dev-dependency, or conditional-visibility mechanism makes a private item
  or a hook reachable from outside the crate, including in test builds. There is no test-support seam, and the
  prohibition is `SPEC-MOK-002` rule 6's, restated for this package.
- The binary target does not declare the presentation modules a second time.
- No public item of the engine package is added. `SPEC-MOK-002` rule 5 stays closed; this specification adds nothing
  to it and needs nothing from it.
- No dependency, dev-dependency, feature, build script or workspace dependency table is introduced in either
  package.

### 8. Tiers and the placement rule

Every observer test belongs to exactly one tier, and the tier is determined by the access the test requires:

- if the test can be written using only rule 6's interface, with its assertions unchanged and with no item widened,
  it belongs to the **public tier**;
- otherwise it belongs to the **internal tier**.

This is `SPEC-MOK-002` rule 7 verbatim, with rule 6 of this specification in place of rule 5 of that one. A test is
not left inline for convenience when the interface suffices, and a test is not promoted to the public tier by
widening the interface. Required access is a property of the test as written; the subject it covers does not decide
the tier, and neither does the file it currently sits in.

A test that reaches one of the four hooks is in the internal tier by definition, because the hook does not exist in
the build a public-tier test links.

### 9. Public tier

Located in `mokiterions-tui/tests/`, one file per subject, each compiled as its own integration-test target and
reaching the code as `use mokiterions_tui::…`. The arrangement, with the count each file receives:

| File | Subject | Tests |
|---|---|---|
| `tests/authority.rs` | the engine's verdict and its presentation | 4 |
| `tests/export.rs` | export content and its rendered form | 7 |
| `tests/layout.rs` | the pane thresholds, the floor and pane geometry | 10 |
| `tests/options.rs` | the observer's argument handling | 7 |
| `tests/render.rs` | frame content asserted through the frame entry points | 8 |
| `tests/spatial.rs` | world-to-canvas mapping | 7 |
| `tests/state.rs` | observer state, accessors, filters and the event buffer | 21 |
| `tests/verification.rs` | the cross-cutting properties: non-perturbation, export fidelity, presented-value fidelity, the authority verdict, colour independence | 16 |
| **Total** | | **80** |

The counts began as the measured outcome of applying rule 8 to the 109 tests at the `WO-MOK-006` predecessor commit —
77 in this table — and they are stated so that a relocation that loses or invents a test is detectable. They are not a
quota: if applying rule 8 during implementation assigns a test differently than measured, rule 8 governs, the count is
corrected here, and the discrepancy is recorded as work-order evidence. A further file may be added when a further
public subject appears. One file per test is not the arrangement.

`tests/layout.rs` reads 10 rather than 7 as amended 2026-08-19, and the 10 is a net figure rather than three
additions to an unchanged seven. `SPEC-MOK-003` rule 5 as amended adds a monotonicity obligation over the whole plane
and a threshold per pane, and `VER-MOK-005` as amended requires a case for each; the same amendment deletes the tier
table those thresholds replace. The measured composition is three tests gone and six added:

| Gone | Why |
|---|---|
| `tiers_match_the_specified_table_including_its_boundaries` | asserts `tier_for` and the four-row table, neither of which rule 5 still defines |
| `tier_minimums_hold_wherever_the_tier_declares_one` | asserts the per-tier minimum columns and rows the table carried |
| `excluded_panes_are_the_ones_the_tier_omits` | renamed rather than deleted; see the first addition below |

| Added | Obligation it carries |
|---|---|
| `excluded_panes_are_the_ones_the_viewport_omits` | the rename of the third test above, same assertion against the amended rule |
| `each_pane_appears_at_its_threshold_on_the_axis_that_constrains_it` | `VER-MOK-005`'s pane-presence case, checked on both sides of each of the three thresholds |
| `the_log_is_ten_rows_only_where_both_thresholds_are_met` | `VER-MOK-005`'s log-height case |
| `enlarging_the_viewport_never_removes_a_pane` | `VER-MOK-005`'s layout-monotonicity case and property |
| `the_one_to_one_threshold_with_the_roster_alone_is_113_columns` | `VER-MOK-005`'s mapping-injectivity property, in the roster-only band the amendment creates |
| `the_vertical_one_to_one_threshold_is_44_rows` | the same property on the other axis, which the amendment moves |

The last two are not named by `VER-MOK-005`'s case table. They belong to its **Mapping injectivity** property, which
the pre-existing `the_one_to_one_threshold_with_the_inspector_shown_is_157_columns` covered when the inspector was the
only regime; rule 5 as amended creates a second regime, so the property needs a threshold case in it and one on the
row axis. Every item all nine name is already in rule 6's interface, so rule 8 places them in the public tier and
nothing widens.

### 10. Internal tier

Located in a `#[cfg(test)]` module inside the crate, beside the code it covers:

| Location | Tests | Why they cannot move |
|---|---|---|
| `mokiterions-tui/src/render.rs` | 12 | assert drawing internals; the module declares 39 private items — 27 functions and 12 constants — against 2 public ones, and 5 of the 12 additionally use a hook |
| `mokiterions-tui/src/verification.rs` | 8 | reach several modules **and** a hook, so they belong to no single module's tier and cannot leave the crate |
| `mokiterions-tui/src/state.rs` | 4 | 3 use a hook; 1 asserts a private detail of the state type |
| `mokiterions-tui/src/main.rs` | 8 | every one requires a private item of the binary: 4 name `tick_interval`, `due`, `idle_for` or `report` directly, and 4 reach the start-up function and the private `Launch` type through two helpers in the test module |
| **Total** | **32** | |

`mokiterions-tui/src/verification.rs` is declared from `lib.rs` under rule 5 and contains only the eight tests above.
This is the one place the observer's structure differs from the engine's, where `SPEC-MOK-002` rule 3 leaves
`src/lib.rs` with no test: the engine has no cross-module internal test and the observer has eight. `lib.rs` itself
still declares no test.

`mokiterions-tui/src/authority.rs`, `export.rs`, `layout.rs`, `options.rs` and `spatial.rs` are left with no
`#[cfg(test)]` module, because rule 8 assigns every one of their tests to the public tier.

### 11. One invocation

`cargo test` compiles and runs every tier of both packages. No tier requires a feature, an environment variable, an
`#[ignore]` attribute, a separate command, a terminal, a pseudo-terminal, or a particular working directory.

`cargo test -p mokiterions-tui` runs the observer's three internal-tier targets and its eight public-tier targets and
nothing else. `cargo test -p Mokiterions` runs the engine's two tiers and nothing else, with the observer excluded
from the build — `SPEC-MOK-002` rule 10 as amended is the authority for that form and it is unaffected here.

The number of executed tests is the same before and after the restructuring: 109 for the observer, 60 for the engine,
169 for the workspace, with the per-tier split of rules 9 and 10. As amended 2026-08-19 the observer's total is **112**
and the workspace's **172**, the engine's 60 unchanged, the three added by `WO-MOK-005`'s conformance to
`SPEC-MOK-003` rule 5 as amended. The clause this paragraph exists for is conservation across a move, not a ceiling on
the corpus: a work order that adds a test corrects these figures here, and one that loses a test has a defect.

An observer test asserts a rendering claim against an in-memory character buffer. A test requiring a terminal, a
pseudo-terminal, a screenshot, or a recording is not admissible in either tier; `SPEC-MOK-003` is the authority and
this rule restates it because the public tier is a new place such a test could be written.

### 12. Test content preservation

A relocated test keeps its assertions verbatim. Only the path by which it reaches the code changes — a `use` of the
library target in place of `use super::*` or `use crate::…`. A relocated test whose assertions cannot survive the
move is a rule 8 misclassification and stays in the internal tier.

No assertion is weakened, generalized, replaced by a looser observation, split, merged or renamed, in either tier,
in either package, **by the `WO-MOK-006` restructuring**. A helper used only by relocated tests moves with them; a
helper used by tests in both tiers is duplicated or shared under the delegation in *Explicitly unspecified
decisions*, and its behavior does not change.

The scoping in bold is the 2026-08-19 amendment, and it is the same scoping rule 13 receives for the same reason.
This rule is about a move: its first paragraph says so in every sentence. Read without the scope, its second
paragraph forbids any later work order from ever renaming a test in either package, which is a freeze on test
maintenance that no requirement asks for and that this rule was not written to impose. A later work order that
changes an assertion is governed by the specification and contract it works under; if it weakens one, that is a
defect there, and this rule is not what detects it.

### 13. Behavior preservation

The restructuring is equivalence-preserving for both packages.

For identical arguments and identical seed, the engine emits byte-identical output, reaches an identical final state,
and returns an identical exit code, with and without `--trace-actions`, under both decision sources, at every
declared density. No simulation constant, event field, event order, summary field, exit code, diagnostic message, or
byte of `USAGE` changes. Every case, invariant and check in `VER-MOK-001`, `VER-MOK-002` and `VER-MOK-004` remains
covered.

For identical arguments, identical seed and identical viewport, the observer presents identical frames, writes
byte-identical exports, responds to identical key bindings, resolves identical panes, draws identical glyphs and
returns identical exit codes. Every case, invariant and check in `VER-MOK-005` remains covered. The clause read
"selects identical layout tiers" until the 2026-08-19 amendment; it is the same obligation, named the way
`SPEC-MOK-003` rule 5 as amended names it, and it binds this restructuring rather than later work that amends what a
viewport yields.

Relocated content is compared, not reviewed. Each moved file's content is byte-identical to its content at the
predecessor commit apart from path references the move itself requires, and the comparison is performed against the
predecessor commit rather than asserted, because a move produces a diff in which every line is new.

### 14. Operator commands

Each form below resolves to the same target and produces the same output as at the predecessor commit, and each is
executed rather than assumed. A virtual workspace root does not resolve a bare command the way a package root does,
and this rule exists because that is the change's one real operator-facing risk.

| Command | Required outcome |
|---|---|
| `cargo run --bin Mokiterions` | runs the engine binary; the first line of `USAGE` is byte-identical to the verified text |
| `cargo run -p mokiterions-tui` | runs the observer binary |
| `cargo build -p Mokiterions` / `cargo build -p mokiterions-tui` | builds that package alone |
| `cargo test` / `cargo test -p <package>` | rule 11 |
| `cargo tree -p Mokiterions` | resolves to the engine package alone, which is the check that the layout cost the engine's empty dependency table nothing |
| `cargo tree -p mokiterions-tui` | resolves to the observer, the engine, and the `ratatui` graph fixed by `SPEC-MOK-003` |
| `cargo fmt` / `cargo clippy --all-targets --all-features -- -D warnings` | pass workspace-wide with no `allow` attribute added and no narrowing of the invocation |

A form that no longer resolves is corrected under rule 2 and never by renaming a package or a target.

## Error and recovery behavior

- A public-tier file that fails to compile because it names an item rule 6 does not expose is a rule 8 signal:
  reclassify the test to the internal tier. It is never grounds for widening an item, and rule 6 does not grow to
  admit it.
- A test that compiles in the public tier only after its assertion is loosened is a rule 12 defect. The correction
  is that the test returns to the internal tier in its original form.
- A private item left with no user by the relocation, surfacing as a dead-code warning under the lint gate, is a
  rule 8 signal that the test which used it was misclassified. The correction is to return that test to the internal
  tier, never to delete the item and never to add an `allow` attribute.
- A clippy failure caused by a target name is corrected under rule 4, never by narrowing the lint invocation.
- A test-count difference before and after, in either direction, is a failure: a test was lost or silently added.
- Any observable difference from either package's verified baseline is a defect in the restructuring, not a new
  baseline.
- A verified record or a retained evidence file that names an old path is not edited. It is bound to its commit by
  its verification record; a superseding mapping is produced as work-order evidence instead.

## Data and interface contracts

The observer's public interface carries presentation values, pure functions of values, read-only accessors and the
export writer. It reaches no engine mutation path: it cannot construct an `Observation`, cannot implement or invoke
a `DecisionSource`, and holds no mutable borrow of engine-owned state. The engine's interface is unchanged, so
`SPEC-MOK-002`'s *Data and interface contracts* section continues to hold verbatim with its paths read under
`mokiterions-core/`.

The observer's own state type keeps sole ownership of its snapshot, its event buffer and its selection. Making its
already-public accessors reachable from outside the crate hands out the copies they already returned inside it, and
grants no capability that did not exist at the predecessor commit — which is the whole of why rule 6 can admit
nothing and still be sufficient.

The dependency direction is unchanged and unchangeable: the observer depends on the engine by path, the engine
depends on nothing, and rule 3 preserves both. Rule 1's directory layout carries no dependency meaning; a package's
directory does not make it a dependency of anything.

## Security and privacy properties

- No network access, credential read, environment read, wall-clock read, or filesystem read is introduced. Neither
  package gains a dependency.
- Rule 7 is the security-relevant rule of this specification. It preserves `REQ-MOK-004` and `ADR-MOK-001` by
  keeping every prohibited engine item private and by keeping the observer's four hooks compiled out of every
  shipped artifact, in test builds as well as release builds.
- The trust boundary is unmoved. A decision source still receives immutable observations and returns typed
  proposals. `Observation` and `DecisionSource` stay among the ten names `SPEC-MOK-002` rule 6 keeps private, and
  the observer's public interface reaches neither.
- World authority is unchanged. Nothing on the observer's interface can mutate a world, and the observer still
  calls exactly one mutating engine method.
- Export behavior is unchanged. An operator-supplied export path is data, is validated as a string only, is never
  interpreted as code, is never used to read, and is not opened at start-up. No credential, secret, environment
  variable, absolute path or wall-clock value appears in a frame or an export.
- A directory move creates no new trust boundary and no new artifact. Two packages remain, one repository, one
  version, one candidate commit.

## Performance and capacity

Runtime behavior is unchanged; rule 13 requires it. Compile-time cost grows: the observer's binary target links its
library target, and each public-tier file becomes an additional test target, so the observer contributes eleven test
binaries where it contributed one, and `target/` grows. Per-tick work, per-frame work, memory use, frame budget,
input latency and output volume are unaffected. A first build after the move is a full rebuild, because every
package path changes.

## Observability

Unchanged. The engine's event stream, action-trace lines, summary line and exit codes, and the observer's frames,
panes, glyphs, diagnostics, exports and exit codes are exactly as verified. This specification adds no log, metric
or diagnostic, and the observer's diagnostic report is untouched.

## Compatibility and migration

- `SPEC-MOK-002` requires amendment wherever it names a root-relative path — its *Inputs*, rule 1's target table and
  its 2026-08-18 note, rule 3, rule 4, rule 5's `grep` check, rule 8's file table and rule 9's locations — and
  wherever it implies the engine package is at the root. Its rules bind unchanged in substance; only the paths move.
  Its rules 7 to 10 remain the engine's test-placement contract, and this specification is the observer's.
- `SPEC-MOK-003` requires amendment in its *Component layout*, whose tree and clause 3 fix the engine's sources at
  the root; in *Data and interface contracts* rule 2, whose parenthetical justifies keeping `Simulation::run` public
  by appealing to a component layout that "forbids" relocating the engine's sources — the conclusion holds, because
  `run` is the `REQ-MOK-010` whole-run entry point that the engine's own binary calls, but the reasoning and the
  path reference are stale; and in *Explicitly unspecified decisions*, both in the entry that withholds the package
  layout and in the entry that delegates test organization.
- `ARCH-MOK-002` requires amendment in the component that calls the observer host "the new binary", in the
  *Testability without a terminal* quality attribute, in its required and prohibited patterns, and in its
  conformance checks, and it gains `REQ-MOK-028` among the requirements it addresses and this specification among
  those it conforms to. `ADR-MOK-004` states the amendments; `WO-MOK-006` makes them an approval precondition.
- `ARCH-MOK-001` needs no amendment. It names no source path, and every quality attribute and conformance check it
  states about the engine package — one library target, one thin binary target, an empty dependency table — holds
  unchanged after the move.
- `REPOSITORY_CONTEXT.md` requires updating in its *Commands* and *Architecture* sections, and in the sentence that
  states the two-tier test convention repository-wide while citing engine-scoped authority for it. That sentence's
  disagreement with the code is what `WO-MOK-005` disclosed; after this specification it is true of both packages,
  citing `SPEC-MOK-002` rules 7 to 10 for the engine and this specification for the observer.
- Records bound to commits are not re-opened. `VREC-MOK-001` through `VREC-MOK-005` and the retained evidence under
  `WO-MOK-001` through `WO-MOK-005` name paths that this specification moves, and they stay as written.
- Reversal is a directory move and the deletion of one file. No behavior in either package depends on the outcome,
  which is what makes this specification safe to conform to in one work order.

## Examples and counterexamples

**Example.** The cross-cutting non-perturbation test drives the observer over a fixed number of ticks and asserts
that a second engine run from the same seed produces a byte-identical text stream. It reaches the engine through the
engine's public interface and the observer through already-public items, and names no hook. Rule 8 places it in the
public tier, rule 9 places it in `tests/verification.rs`, and rule 12 keeps its assertion verbatim.

**Example.** The layout tests assert which panes a given viewport yields. Every item they named was among the
`layout` module's 13 public items at the predecessor commit, so all seven moved to `tests/layout.rs` and `layout.rs`
is left with no `#[cfg(test)]` module at all. The same holds of the three tests added there since, against the 10
items rule 6 records now.

**Counterexample.** A rendering test asserts the exact form of a bar row by calling a private drawing helper and
reading two private constants. Making the helper and the constants `pub` so the test can move to `tests/render.rs`
violates rule 7, misapplies rule 8, and is the widening `INT-MOK-005` prohibits. The test stays in
`src/render.rs`.

**Counterexample.** A test builds a world with no standing resources by calling the hook that replaces the
snapshot, then asserts the empty-resource pane. Removing the hook's `#[cfg(test)]` attribute so the test can move
violates rule 7 and installs the test-support seam `SPEC-MOK-002` rule 6 and `ARCH-MOK-001` both deny. The test
stays in the internal tier.

**Counterexample.** Replacing that rendering test's assertion with "the rendered pane contains a digit" in order to
place it in the public tier violates rule 12. The relocated test is weaker, so the relocation is a defect rather
than a move.

**Counterexample.** Moving the engine to `mokiterions-core/` and renaming the package to `mokiterions-core` at the
same time violates rule 3 and rule 14: `cargo tree -p Mokiterions` would fail, `cargo run --bin Mokiterions` would
resolve to nothing, and the first line of `USAGE` would change. The directory name and the package name are
independent, and only the directory moves.

**Counterexample.** Adding `[workspace.dependencies]` to the root manifest so both packages could share a version
key violates rule 2 and rule 7, and would give the engine package a dependency table that is not empty in
substance, against `SPEC-MOK-002` rule 1 and `ARCH-MOK-001`.

## Explicitly unspecified decisions

- Ordering of items within each source or test file, and whether public-tier helpers are duplicated per file or
  shared through a `tests/common/` module in either package.
- The order in which the seven `pub mod` declarations appear in the observer's `lib.rs`, provided all seven are
  present and the `verification` declaration stays `#[cfg(test)]`.
- Whether the observer's binary target imports the library's modules individually or as a group, provided it
  declares no module of its own.
- Whether the internal tier's existing helper functions are kept, renamed or consolidated, provided no assertion
  changes.
- Doc comments, internal comments, and non-authoritative developer notes, including whether each manifest keeps the
  specification-citing comments it carries today.
- Whether the observer's module files are later split into further private modules. Rules 6 and 7 constrain
  visibility, not file count.
- The order in which the relocation, the library target and the test moves are performed within the implementing
  work order, provided every rule holds at the candidate commit.
