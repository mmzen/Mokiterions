# Conformance: every architecture and ADR check this change touches

`VER-MOK-006` verifies the work order's own conformance checks; the files beside this one carry those.
This file answers a different question — whether the two architectures and the four ADRs that govern
this repository still hold after the change — because a layout change is exactly the kind of change
that can satisfy its own contract and quietly break an older one.

The general answer, stated once so that it does not have to be repeated under every check below:

> **The engine's nine source files are byte-identical to the predecessor commit at blob level, and
> the observer's seven presentation modules are byte-identical outside their `#[cfg(test)]` blocks.**
> `file-comparison.txt` and `non-test-content.txt` are those measurements. Any check whose subject is
> a property *of that code* is therefore satisfied by the identity, not by re-argument. Only the
> checks whose subject is a *manifest*, a *target shape*, a *file location*, a *visibility* or a
> *command* can have changed, and those are the ones answered in detail.

The one file that is neither new nor byte-identical is the observer's `src/main.rs`. Its 396 lines
from `const FRAME_INTERVAL` to the end of the file hash to
`2842d01b965d983f7371b88627cd7e94c346493d10f954918f78e61a92cb3a8f` in both trees. The entire delta is
its header: seven `mod` declarations and one `#[cfg(test)] mod verification;` removed, and two
`use crate::` lines rewritten to `use mokiterions_tui::`, 431 lines becoming 425. Nothing that runs,
draws, schedules or restores was touched.

---

## `ARCH-MOK-001`, as amended 2026-08-17 — the engine package

### Prohibited patterns

| Prohibition | Status | Basis |
|---|---|---|
| Mutable world references, callbacks or engine handles exposed to decision sources | holds | `simulation.rs` byte-identical (2,993 lines) |
| Global mutable simulation or entropy state | holds | same |
| Wall-clock, OS randomness, unordered iteration or thread scheduling affecting results | holds | same; and `resilience.txt` re-derives four 10,000-tick runs to identical summaries |
| Network, credentials, async runtime, database, UI framework, plugin system or DI container **in the engine package**; dependency set empty, no exception, "including a dependency shared with another package in the same workspace" | holds | `manifests.txt`: `mokiterions-core/Cargo.toml`'s `[dependencies]` is empty. `dependencies.txt`: `cargo tree -p Mokiterions` resolves to `Mokiterions v0.1.0` and nothing else |
| Panics for invalid operator input or invalid proposed actions | holds | `cli.rs` byte-identical; 12 `tests/cli.rs` tests and `baseline/engine/exit-codes.txt` |
| Separate packages, workspaces or services without an approved requirement; `REQ-MOK-026` authorizes exactly one further package | holds | `manifests.txt`: `members = ["mokiterions-core", "mokiterions-tui"]`. Two members, the two `REQ-MOK-026` admits. No third package, no service, no separate release artifact |
| Public items exposing a mutable borrow of or reference into authoritative state, or any feature flag, `cfg` attribute or test-support seam exposing such access outside the crate | holds | `lib.rs` byte-identical, so the enumerated interface is unchanged item for item. Neither package declares a feature; `--all-features` in `static-checks.txt` therefore adds nothing |

The empty-dependency prohibition is the one this change could most plausibly have broken, and the way
it would have broken is worth naming: pointing the observer's path dependency at a moved directory is
a one-line edit, and getting it wrong in the other direction — adding `mokiterions-tui` to the engine's
manifest to "share" something — is the failure mode. `mokiterions-core/Cargo.toml` has no
`[dependencies]` entry at all, and `cargo tree -p Mokiterions` is the check that does not rely on
reading the manifest correctly.

The amendment's own words — "Resolve the graph per package rather than for the workspace, since a
workspace graph containing the observer would not answer the question" — are followed:
`dependencies.txt` records the per-package tree, and separately records the workspace tree so the
difference is visible.

### Conformance checks

- **Dependency direction and public APIs reviewed for mutable-state leakage.** `lib.rs` and
  `simulation.rs` byte-identical; `public-item-census.txt` records the observer's 97 public items and
  none of them is an engine type carrying a mutable borrow.
- **Engine package's dependency graph empty, therefore no network, async-runtime, database or UI
  library.** `dependencies.txt`.
- **Engine does not depend on the observer, directly or transitively.** `dependencies.txt`: the engine
  tree is one line. Cargo would reject the cycle in any case, since the observer depends on the engine.
- **Observation surface exposes no mutating operation other than the single-tick advance and returns
  owned values.** `lib.rs` and `simulation.rs` byte-identical.
- **Deterministic replay and invalid-action atomicity tests run.** `test-run.txt`: 60 engine tests
  pass, including `tests/process.rs` (5) and `tests/termination.rs` (3).
- **All stochastic code reachable from the explicit seeded entropy owner.** `simulation.rs`
  byte-identical.
- **Engine builds as one package with exactly one library target and one binary target and an empty
  dependency table, and its tests run independently of the observer and with no terminal present.**
  `manifests.txt`: one `[lib]`, one `[[bin]]`, empty `[dependencies]`. `test-run.txt`:
  `cargo test -p Mokiterions` reports 60 passed over 8 result lines, exit 0, with no observer artifact
  in the invocation and no terminal.
- **Library target's public interface matches `SPEC-MOK-002` rule 5 as amended exactly.**
  `lib.rs` byte-identical, so the enumeration it satisfied at `VREC-MOK-005` it satisfies now.

### Quality attribute: *Simplicity*

"one engine package with one library target and one thin binary target … plus at most the one further
package `REQ-MOK-026` approves". Two packages, four targets, no build script in either. The engine's
binary is still 19 lines.

---

## `ARCH-MOK-002`, as amended 2026-08-18 — the observer package

### The four checks added 2026-08-18, which are this change's own

**1. "the observer package builds a library target and a binary target, that the library target's
public interface is exactly the items that were already public before it existed, and that no item's
visibility widened. The check is that each module file the library declares is identical outside its
`#[cfg(test)]` blocks to its content at the predecessor commit."**

Satisfied, and satisfied by exactly the check the architecture names rather than by an equivalent.
`non-test-content.txt` strips every `#[cfg(test)]` block from each of the seven modules in both trees
and compares SHA-256 digests: **seven of seven identical, 78,332 bytes**. Since the files are
identical, every `pub` in them is in the same place with the same spelling, so provenance closure is
not a judgement — it is the digest. The per-file `pub` counts are recorded as a second, independent
reading: 5 / 3 / 21 / 12 / 2 / 25 / 54 = 122, unchanged.

`manifests.txt` records the observer's `[lib] name = "mokiterions_tui" path = "src/lib.rs"` and
`[[bin]] name = "mokiterions-tui" path = "src/main.rs"`: one of each, no build script, no example, no
benchmark.

**2. "every `#[cfg(test)]` item in the observer retains its attribute, and that no test outside the
crate names one."**

`hooks-and-visibility.txt` enumerates all nine `#[cfg(test)]` attributes in the observer's `src/` and
the four hooks among them, each still carrying its attribute on the line above it. No hook was added,
removed or altered. The second half is enforced by the compiler rather than by inspection: an item
that does not exist in the non-test build cannot be named from `tests/`, so the 77 public-tier tests
compiling is the proof. `test-placement.md` records, for each of the 32 internal-tier tests, which
private item or hook holds it inside.

**3. "every observer test is in exactly one tier, that each test outside the crate reaches the code
only through the library target's public interface with its assertions unchanged, and that one
`cargo test` invocation runs both tiers of both packages with no feature, environment variable,
`#[ignore]` or terminal."**

`test-census.txt` lists all 169 tests by name and tier, 32 + 77 for the observer, with no name in two
tiers. `verbatim-comparison.txt` is the assertions-unchanged half: 76 of the 77 relocated bodies are
byte-identical to the predecessor's and the 77th differs in one path, twice. `test-run.txt` is the
single-invocation half: `cargo test --workspace` reports 169 passed, 0 failed, **0 ignored in all 19
result lines**, with no feature flag, no environment variable and no terminal.

**4. "each package's manifest, sources and tests are under one directory named for that package, that
the repository root's manifest declares no package, and that every package name, target name, target
kind and operator command resolves as it did before."**

`file-comparison.txt` records the two directories and the nine renames. `manifests.txt` records that
the root manifest contains `[workspace]` and `resolver = "3"` and no `[package]` section at all — the
check is that `[package]` is absent, and it is. Names: package `Mokiterions`, library `mokiterions`,
binary `Mokiterions`, package and binary `mokiterions-tui`, all unchanged; the observer's library
target `mokiterions_tui` is new, so there is no prior spelling for it to differ from.

The last clause — "every … operator command resolves as it did before" — is the one that **did not
hold when first measured**, and `command-forms.txt` records the finding and the fix. Seven bare
`cargo run` invocations in `SIMULATION_RULES.md` fail at a virtual-manifest root with two binaries.
All seven now name `--bin Mokiterions` and all seven were re-run to exit 0.

### Required patterns

| Pattern | Status | Basis |
|---|---|---|
| One-way dependency, expressed as a package dependency | holds | `manifests.txt`: `Mokiterions = { path = "../mokiterions-core" }`, keyed by package name, re-pointed from `".."` |
| Owned, reference-free snapshots as the only outbound state transfer | holds | engine `lib.rs` / `simulation.rs` byte-identical |
| Exactly one mutating operation, taking no operator data | holds | same |
| Layout and mapping as pure functions, testable without a terminal | holds | `layout.rs` and `spatial.rs` byte-identical outside tests; their 14 relocated tests contain **no** `Terminal`, `TestBackend`, `stdout` or raw-mode reference |
| Rendering into an in-memory buffer assertable cell by cell | holds | `tests/render.rs` and `tests/verification.rs` construct `Terminal::new(TestBackend::new(w, h))`, ratatui's in-memory backend, which touches no tty; no other test file references either |
| **Presentation layer as a library target closed by provenance** (added 2026-08-18) | holds | this change; the seven digests above |
| **Each package's manifest, sources and tests under one directory named for it** (added 2026-08-18) | holds | this change; `file-comparison.txt` |
| Unconditional terminal restoration on every exit path, including panic | holds | `main.rs` lines 122–137, inside the 396 byte-identical lines: `ratatui::try_init` installs the panic hook, `ratatui::try_restore` is called unconditionally after the loop and its error is folded into the exit code |
| Bounded event retention with declared capacity and visible truncation marker | holds | `state.rs` byte-identical outside tests; `comparison/observer-matrix.txt` records `truncated=false` and 68,948 of 100,000 records at 10,000 ticks |
| Wall-clock confined to scheduling | holds | `FRAME_INTERVAL`, `INPUT_INTERVAL`, `tick_interval`, `due` and `idle_for` are all inside the 396 byte-identical lines |

### Prohibited patterns

Every prohibition holds. The four whose subject this change could have moved:

- **"Widening any item's visibility, in either package, in order to reach it from a test."** The
  seven digests are the answer for the observer and the nine blob ids for the engine. Not one item's
  visibility changed anywhere, so the prohibition is not merely obeyed — it had no opportunity to be
  engaged. `public-item-census.txt` and `hooks-and-visibility.txt` record that no `pub(crate)`,
  `pub(super)` or `pub(in …)` exists in the observer either, so there is no intermediate visibility
  that could have been quietly widened to `pub`.
- **"Removing, gating or otherwise relaxing a `#[cfg(test)]` attribute … The four hooks on the
  observer's state type stay as they are."** They do, at `state.rs:587`, `592`, `604` and `617`.
  `hooks-and-visibility.txt` also records what was declined: publishing them would have moved 12 more
  tests to the public tier and raised 77 to 89. It was not done.
- **"Any test-support seam, feature-gated or otherwise, that exposes either package's internals
  outside its crate."** Neither package declares a feature. `manifests.txt` shows no `[features]`
  table in either, and `--all-features` in `static-checks.txt` therefore changes nothing.
- **"the observer's binary target does not declare the presentation modules a second time"** (from
  `ADR-MOK-004`'s amendment list). `main.rs` declares no `mod` at all now; the diff above is that
  removal. Declaring them twice would compile the modules twice and give the package two copies of
  every type, which is why the prohibition exists and why `main.rs` reaches them through
  `use mokiterions_tui::…`.

Also unchanged, and worth stating because these are the ones an operator could be harmed by: no
serialization, no async runtime, no threads sharing state, no third package; no repository read, no
version-control invocation and no network access at run time; no operator control that mutates
simulation state.

### Conformance checks carried from before

- `cargo tree` for the engine resolves to the engine alone — `dependencies.txt`.
- Engine does not list the observer — `dependencies.txt`.
- Every UI dependency in the observer's manifest and no other — `manifests.txt`; the engine's
  `[dependencies]` is empty, which is the strongest form of this check.
- Engine's tests pass with no terminal — `test-run.txt`, 60 passed.
- Observation surface: no `&mut self` beyond the advance, snapshots own their data — engine sources
  byte-identical.
- **Observed and unobserved runs identical, with the observed run held, single-stepped, selected,
  panned, zoomed, filtered, exported and resized**, and **per-tick entropy draw counts identical.**
  These were established by `VREC-MOK-005` against the code this change does not alter. What this
  change owes is that the property was not disturbed, and that is what
  `comparison/observer-matrix.txt` measures: 40 cells identical, six retained seed-42 viewports
  identical, five exports identical at 74,855 / 74,909 / 74,811 / 75,001 / 74,717 bytes and
  624 / 623 / 624 / 622 / 623 records. `comparison/engine-matrix.txt` does the same for the engine:
  seven files, 0 differing lines, survivor counts 8 / 11 / 8 / 9 / 11 identical.
- Layout and mapping exercised by tests constructing no terminal — as tabulated above.
- Rendered output asserted from an in-memory buffer at each named viewport size — `tests/render.rs`
  (8) and `tests/verification.rs` (16), bodies verbatim from the predecessor.
- Terminal restored on normal exit, error exit and panic — `main.rs`, byte-identical region.
- No repository read, version-control invocation or network access at run time — sources
  byte-identical; and the export path is still validated as a string only and never opened at
  start-up.

### Quality attributes

*Non-perturbation* and *Legibility of authority* rest on byte-identical code. *Containment* is
re-measured per package after the move, which `ADR-MOK-004` listed as a positive consequence and
`dependencies.txt` delivers. *Independent failure* is stronger than before: the engine is now a
directory with its own manifest and no reference to the observer.

*Testability without a terminal*, as amended, is the attribute this change exists to satisfy — "and
assertable *through a stated public interface, from a test tier outside the crate*". 77 of the
observer's 109 tests are now outside the crate. *Simplicity, bounded* — "exactly two packages, one
repository, one version, one candidate commit" — holds: two members, both at `0.1.0`.

---

## `ADR-MOK-001` — engine-authoritative in-process decision boundary

Not superseded, and untouched. Its *Decision* has eleven bullets; every one is a property of
`simulation.rs`, `lib.rs`, `cli.rs` or `main.rs` in the engine package, and all four are
byte-identical at blob level. Its first bullet — "one Rust process and one Cargo package, built as a
library target and a thin binary target" — is the only one whose *words* this change comes near, and
it is satisfied: the engine is still one package with one library and one thin binary. The package
moved from the repository root into `mokiterions-core/`, which is a directory, not a package count.

Its *Validation* list is discharged by `test-run.txt`'s 60 engine tests and by `dependencies.txt`'s
empty tree, both re-run on the candidate rather than inherited.

## `ADR-MOK-002` — library target with an enumerated read-only interface

Not superseded. The engine's `lib.rs` is byte-identical, so the enumeration it decided stands
unchanged. `ADR-MOK-004`'s Option 4 considered mirroring `SPEC-MOK-002` rules 3 and 4 in the observer
and rejected it, so the engine's thin-binary shape is neither copied nor disturbed.

## `ADR-MOK-003` — two-package split with a terminal observer

Not superseded, and its eleven decision clauses hold with one deliberately reversed:

- Clause 1, **exactly two packages**: holds. `members` has two entries.
- Clause 2, "Keep the engine package at the repository root with its sources in their existing
  location, so the `REQ-MOK-010` text stream does not move": **this is the clause this change
  reverses**, and `ADR-MOK-004`'s *Context* states why the reason for it is discharged —
  `VREC-MOK-005` recorded the engine's whole corpus byte-identical across the observer's
  introduction. The clause's *purpose* is nonetheless honoured rather than waived:
  `comparison/engine-matrix.txt` shows the text stream did not move in the sense that matters, 0
  differing lines across seven captures. `SPEC-MOK-003`'s corresponding clause 3 is amended in the
  same act, which `amendment-approvals.md` records.
- Clauses 3, 4, 6, 7, 8, 9: hold on byte-identical code and an empty dependency table.
- Clause 5, **ratatui `0.30.2`, `default-features = false`, features `crossterm`, `layout-cache`,
  `underline-color`, 57 crates including itself**: holds exactly. `dependencies.txt` records
  `Cargo.lock` byte-identical five ways at blob `824003e1cbd4757f28d09849e2faca0a4546e99f`, the
  observer tree at 57 crates and 166 features, and `serde` absent.
- Clauses 10 and 11: the `ARCH-MOK-001` amendment and `ADR-MOK-001`'s standing, both addressed above.

Its *Migration* section says the engine "keeps its name, both of its target names, and the location
of its sources". The first two hold; the third is what `ADR-MOK-004` reverses, on the record, with
`SPEC-MOK-002` and `SPEC-MOK-003` amended accordingly. Its stated reason for leaving the names alone —
that renaming would edit a verified operator-facing string — is exactly why the package is still
`Mokiterions` while only the directory is `mokiterions-core`.

## `ADR-MOK-004` — this change's own ADR

Its *Validation* list, item by item:

| Validation item | Evidence |
|---|---|
| Exactly one library target and one binary target, no build script | `manifests.txt` |
| Visibility of every item in the observer's non-test code identical to the predecessor, checked as a diff | `non-test-content.txt` — seven digests, 78,332 bytes |
| The four `#[cfg(test)]` hooks still `#[cfg(test)]`, and no public-tier test names one | `hooks-and-visibility.txt`, `test-placement.md` |
| The binary declares no presentation module and reaches them through the library | the `main.rs` diff above |
| One directory per package, root manifest declares no package | `manifests.txt`, `file-comparison.txt` |
| `cargo test` runs every tier of both packages with no terminal, feature, environment variable or `#[ignore]`, and each tier's count matches the pre-change count | `test-run.txt` (169 / 0 / 0 across 19 lines), `test-census.txt` (169 reconciled by name) |
| `cargo tree -p Mokiterions` resolves to the engine alone | `dependencies.txt` |
| `cargo run --bin Mokiterions` runs the engine and its `USAGE` first line is byte-identical to the verified text; `cargo run -p mokiterions-tui` runs the observer | `command-forms.txt`, `baseline/observer/usage.txt` |
| Engine's text stream, summary line and exit codes byte-identical at every declared seed and density, with and without `--trace-actions`, under both decision sources | `comparison/engine-matrix.txt`, `baseline/engine/` |
| Observer's frames and exports identical at every declared seed and viewport | `comparison/observer-matrix.txt` |

One item on that list is worth calling out as the load-bearing one. "**checked as a diff rather than
asserted**" is the phrase that distinguishes this evidence packet from a review, and it is why
`non-test-content.txt` compares digests of stripped files rather than recording a reviewer's opinion
that nothing was widened. `ADR-MOK-004` anticipated the review problem in its own decision drivers —
"a file move produces a diff in which every line is new, so the decision must be accompanied by a
comparison mechanism rather than by a request to read a diff" — and the four comparison oracles in
this packet are that mechanism.

---

## What is not claimed

- No claim that the observer's presentation is *correct*, only that it is unchanged. Correctness was
  `VER-MOK-005`'s question and this change alters no rendering code.
- No claim about `#[cfg(test)]` hook necessity. `hooks-and-visibility.txt` records why each exists;
  this change neither validates nor revisits that.
- No claim that the 17 tests `requirement-to-test-mapping.md` records as uncited by name are
  unnecessary — only that no requirement's coverage rests on them alone.
- No claim on `SPEC-MOK-004`'s eleven-test-binary prediction beyond the measurement:
  `compile-and-target.txt` and `test-run.txt` record ten, and there is no obligation attached to the
  figure to fail.
