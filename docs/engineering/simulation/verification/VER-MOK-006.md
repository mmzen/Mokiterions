+++
id = "VER-MOK-006"
type = "verification"
title = "Package layout, observer contract, and observer test placement verification"
status = "draft"
owners = ["assurance owner"]
created = "2026-08-18"
updated = "2026-08-18"

[relations]
verifies = ["REQ-MOK-028", "REQ-MOK-029", "REQ-MOK-030"]
+++

# Verification Contract: Package layout, observer contract, and observer test placement verification

## Independence

This contract verifies a restructuring that is required to change nothing observable, which creates the same hazard
`VER-MOK-003` named and one more besides. The first hazard is that a suite which passes because nothing changed
proves nothing, since it would also pass if the restructuring had never happened. The second is specific to a file
move: **a moved file produces a diff in which every line is added and every line is removed**, so the ordinary review
of a diff cannot distinguish a faithful move from a move that quietly edited something. Every check below is written
so that it can fail, and the move is verified by comparison against the predecessor commit rather than by reading a
diff.

Four independent oracles are used, and none of them is the changed code's own test suite.

1. **A recorded pre-change baseline.** The engine's output and exit codes, and the observer's frames and exports,
   are captured *before* the change at the commit the work begins from, and compared byte for byte afterwards. The
   baseline is evidence, captured once and not regenerated after a discrepancy. A discrepancy is a defect in the
   restructuring, never a new baseline.
2. **A content comparison against the predecessor commit.** For every relocated file, the post-change content is
   compared against the pre-change content of the file it came from, extracted from the predecessor commit rather
   than from memory. For each of the observer's seven `pub mod` files, the content outside every `#[cfg(test)]` block is
   compared the same way. This is the check that `SPEC-MOK-004` rule 6 rests on, it does not read the tests, and it
   cannot be satisfied by a test that avoids a widened item.
3. **A test census.** Test counts and identities are taken from `cargo test -- --list` before and after, per package
   and per test binary, and reconciled name by name. A test that disappears, is renamed without record, or becomes
   ignored is a finding regardless of whether the suite is green.
4. **Cargo's own resolution of the operator's commands.** Each command form in `SPEC-MOK-004` rule 14 is executed at
   the restructured repository root. None is inferred from another, and none is assumed to work because it worked at
   a package root — a virtual workspace root resolves a bare command differently, which is the change's one real
   operator-facing risk.

The recorded pre-change counts, fixed here so they cannot be adjusted afterwards:

| Package | Location | Tests |
|---|---|---|
| observer | `mokiterions-tui/src/authority.rs` | 4 |
| observer | `mokiterions-tui/src/export.rs` | 7 |
| observer | `mokiterions-tui/src/layout.rs` | 7 |
| observer | `mokiterions-tui/src/main.rs` | 8 |
| observer | `mokiterions-tui/src/options.rs` | 7 |
| observer | `mokiterions-tui/src/render.rs` | 20 |
| observer | `mokiterions-tui/src/spatial.rs` | 7 |
| observer | `mokiterions-tui/src/state.rs` | 25 |
| observer | `mokiterions-tui/src/verification.rs` | 24 |
| **observer total** | all inline, one tier | **109** |
| engine | `src/simulation.rs` | 37 |
| engine | `src/cli.rs` | 0 |
| engine | `tests/` — `cli.rs` 12, `process.rs` 5, `termination.rs` 3, `density.rs` 2, `viability.rs` 1 | 23 |
| **engine total** | two tiers | **60** |
| **workspace total** | | **169** |

with **0 ignored** in either package, and the observer's public interface at **97 public items** — `authority` 5,
`export` 3, `layout` 13, `options` 8, `render` 2, `spatial` 19, `state` 47 — excluding the four `#[cfg(test)]` hooks.

**The counting rule is `SPEC-MOK-004` rule 6's and the census must state that it used it.** One item is one
declaration written `pub` outside every `#[cfg(test)]` block. A public field of a public struct and a variant of a
public enum are parts of the item that declares them and are not counted again; there are **25** of them, so the same
interface counted the other way is **122**. A census reporting 122 has not found a surplus of 25 items, and a census
that does not say which rule it applied is not evidence.

Verification names the observer's public items, because the interface is the subject of `REQ-MOK-028` and counting it
is the obligation. It does not name private types, private functions, or file-internal structure, and it asserts
nothing about how either package is organized behind its interface. It asserts nothing about which tier an engine
test is in: that is `VREC-MOK-003`'s verified outcome and this change moves only the directory containing it.

`REQ-MOK-029` and `REQ-MOK-030` are verified by static analysis, comparison and reconciliation rather than by
execution, because placement and location are properties of where a file lives, which no runtime assertion can
observe.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-028` | static-analysis | `mokiterions-tui/Cargo.toml` target declarations | Exactly one `[lib]` named `mokiterions_tui` at `src/lib.rs` and one `[[bin]]` named `mokiterions-tui` at `src/main.rs`; no third target, no build script |
| `REQ-MOK-028` | static-analysis | `mokiterions-tui/src/lib.rs` contents | Declares the seven modules `pub mod` and `verification` as `#[cfg(test)] mod`; defines no item and contains no test |
| `REQ-MOK-028` | static-analysis | `mokiterions-tui/src/main.rs` contents | Declares no module; reaches the presentation layer as `use mokiterions_tui::…`; retains its start-up, loop, scheduling and its own `#[cfg(test)] mod tests` |
| `REQ-MOK-028` | static-analysis | Non-test content of each of the seven `pub mod` files, compared against the predecessor commit | Byte-identical in all seven; no added, removed or altered visibility keyword anywhere. `verification.rs` is excluded and stated as excluded, being `#[cfg(test)]` in its entirety |
| `REQ-MOK-028` | static-analysis | Public-item census of the library target, under `SPEC-MOK-004` rule 6's counting rule, stated | 97 items with the per-module counts fixed above, and 25 public fields and variants belonging to them; surplus and shortfall are both findings |
| `REQ-MOK-028` | static-analysis | The four test-only hooks on the observer's state type | All four still carry `#[cfg(test)]`; none is named by any public-tier file |
| `REQ-MOK-028` | static-analysis | No conditional visibility | No `cfg`, feature, self dev-dependency or build script affects which items are public, in any build configuration including test builds |
| `REQ-MOK-028` | automated-test | A public-tier file compiles and runs against the library target alone | `cargo test` builds every `mokiterions-tui/tests/*.rs` target; none names a private item, and none uses `#[path]` or `include!` to re-enter the crate |
| `REQ-MOK-028` | static-analysis | Observer dependency set | One path dependency on `Mokiterions` and one on `ratatui`, at the version and feature set `SPEC-MOK-003` fixes; no addition |
| `REQ-MOK-028` | automated-test | Lint gate under the new target name | `cargo clippy --all-targets --all-features -- -D warnings` clean, with no `non_snake_case` and no `dead_code` finding, and no `allow` attribute added |
| `REQ-MOK-028` | review | The engine's public interface | Unchanged; `SPEC-MOK-002` rule 5's enumeration matches the engine library target item for item |
| `REQ-MOK-029` | static-analysis | Every observer test is in exactly one tier | No test outside `mokiterions-tui/tests/*.rs` and the `#[cfg(test)]` blocks of `render.rs`, `state.rs`, `verification.rs` and `main.rs` |
| `REQ-MOK-029` | static-analysis | Public tier reaches the code only through the interface | No `mokiterions-tui/tests/*.rs` file names a private type, function, module, field or hook |
| `REQ-MOK-029` | static-analysis | Public-tier arrangement matches `SPEC-MOK-004` rule 9 | The eight named files exist with the stated subjects and the stated counts; no one-test-per-file arrangement |
| `REQ-MOK-029` | static-analysis | Internal-tier arrangement matches `SPEC-MOK-004` rule 10 | 12 in `render.rs`, 8 in `verification.rs`, 4 in `state.rs`, 8 in `main.rs`; `authority.rs`, `export.rs`, `layout.rs`, `options.rs` and `spatial.rs` have no `#[cfg(test)]` module; `lib.rs` has no test |
| `REQ-MOK-029` | review | Every internal-tier test justified by required access | For each of the 32, name the private item or the hook it requires; a test with no such item is misplaced |
| `REQ-MOK-029` | review | Every relocated test justified by sufficiency | For each of the 77, the public interface suffices with assertions unchanged |
| `REQ-MOK-029` | static-analysis | Relocated assertions are verbatim | Line-by-line comparison of each relocated test body against its predecessor-commit form shows only the access path changed |
| `REQ-MOK-029` | automated-test | One invocation covers every tier of both packages | `cargo test` runs all of them with no feature, environment variable, ignore attribute, extra command, terminal, or working-directory dependence |
| `REQ-MOK-029` | automated-test | No observer test requires a terminal | Every rendering claim asserts against an in-memory character buffer; the suite passes with no terminal and no pseudo-terminal present |
| `REQ-MOK-029` | static-analysis | Test census reconciliation | 109 observer tests before and after, 60 engine, 169 workspace, 0 ignored, every name accounted for |
| `REQ-MOK-030` | static-analysis | Repository layout | Each package's manifest, `src/` and `tests/` under one directory named for it; `mokiterions-core/` and `mokiterions-tui/`; no third package directory and no nested workspace |
| `REQ-MOK-030` | static-analysis | Root `Cargo.toml` | Declares `[workspace]` with the two members and `resolver = "3"`, and declares no `[package]`, `[lib]`, `[[bin]]`, `[dependencies]` or `[workspace.dependencies]` table |
| `REQ-MOK-030` | static-analysis | Names and target kinds | Package `Mokiterions`, library `mokiterions`, binary `Mokiterions`, package and binary `mokiterions-tui`; no name and no kind differs from the predecessor commit |
| `REQ-MOK-030` | static-analysis | Engine dependency and dev-dependency tables | Both empty after the move, with no exception |
| `REQ-MOK-030` | static-analysis | The observer's path dependency | `Mokiterions = { path = "../mokiterions-core" }`; keyed by the engine's package name, with no feature and no version requirement added |
| `REQ-MOK-030` | static-analysis | Every relocated file, compared against the predecessor commit | Byte-identical apart from path references the move itself requires, each such reference enumerated and justified |
| `REQ-MOK-030` | static-analysis | `Cargo.lock` | Every dependency resolves to the same version as before; the only differences are recorded paths |
| `REQ-MOK-030` | automated-test | Each command form of `SPEC-MOK-004` rule 14, executed at the restructured root | Each resolves to the same target and produces the same output as at the predecessor commit; `cargo tree -p Mokiterions` resolves to the engine package alone |
| `REQ-MOK-030` | automated-test | The engine binary's identity | `cargo run --bin Mokiterions` runs the engine and the first line of `USAGE` is byte-identical to the verified text |
| `REQ-MOK-030` | static-analysis | Build warnings | No warning attributable to the layout, in particular none from an unstated workspace resolver |
| all three | automated-test | Byte-for-byte equivalence against the recorded baseline | No differing byte across the declared engine matrix or the declared observer matrix |
| all three | automated-test | Prior coverage preserved | Every case, invariant and check in `VER-MOK-001`, `VER-MOK-002`, `VER-MOK-004` and `VER-MOK-005` maps to a named test in its new location |
| all three | static-analysis | Required amendments present and approved | The `SPEC-MOK-002`, `SPEC-MOK-003`, `ARCH-MOK-002` and `REPOSITORY_CONTEXT.md` amendments named in `ADR-MOK-004` are approved before the change is verified; their absence fails this contract regardless of code state |

## Acceptance scenarios

1. A reviewer extracts the non-test content of each of the observer's seven `pub mod` files at the predecessor commit
   and at the candidate commit, compares them, and finds all eight byte-identical — establishing that no item's
   visibility changed and that `SPEC-MOK-004` rule 7's prohibition held.
2. A reviewer counts the library target's public items under the stated rule and finds 97, distributed exactly as the
   per-module figures above, then attempts to name one item that is public now and was not before, and cannot.
3. A public-tier file is compiled with the four hooks' `#[cfg(test)]` attributes intact and no test in it names a
   hook; a reviewer confirms the hooks are absent from the library target's interface.
4. The engine is run before and after the change with identical arguments at each declared seed, under each decision
   source, at each declared density, with and without `--trace-actions`, and the captured output, final state and
   exit code are identical in every combination.
5. The observer is driven before and after the change at each declared seed and each declared viewport, and the
   captured frames and the written exports are byte-identical in every combination.
6. `cargo test` is run once at the restructured root with no extra flag, no environment variable and no terminal
   present, and reports 169 tests with 0 ignored, distributed as rules 9, 10 and 11 of `SPEC-MOK-004` require.
7. For each of the 32 tests remaining inline, a reviewer names the private item or the hook it requires; for each of
   the 77 relocated tests, a reviewer confirms the interface suffices and the assertions are unchanged.
8. Each command form of rule 14 is executed at the restructured root, and each resolves to the same target as
   before. In particular `cargo run --bin Mokiterions` still starts the engine, and `cargo tree -p Mokiterions`
   still prints one crate.
9. A reviewer looks for a `[package]`, `[lib]`, `[[bin]]` or dependency table in the root manifest, and finds none.
10. The superseding requirement-to-test mapping accounts for every case in `VER-MOK-001`, `VER-MOK-002`,
    `VER-MOK-004` and `VER-MOK-005` at its new location, and the mappings retained under `WO-MOK-002` through
    `WO-MOK-005` are unmodified.

## Property and invariant tests

- **Equivalence, engine.** For every combination of declared seed, decision source, declared density and trace
  setting, pre-change and post-change output are byte-identical and the final state is identical. A single differing
  byte fails this contract.
- **Equivalence, observer.** For every combination of declared seed and declared viewport, pre-change and
  post-change frames and exports are byte-identical.
- **Determinism preserved.** Two post-change engine runs at one seed remain byte-identical, so `REQ-MOK-009` is
  intact and neither the layout nor the library target moved, split or re-seeded the entropy stream.
- **Non-perturbation preserved.** An engine run driven by the observer and an engine run driven by the text host
  produce identical tick sequences at the same seed, so `REQ-MOK-025` is intact.
- **Visibility conservation.** For every item in the observer's non-test code, its visibility at the candidate
  commit equals its visibility at the predecessor commit. This is the central invariant of `REQ-MOK-028`; one
  widened item fails it.
- **Attribute conservation.** Every `#[cfg(test)]` attribute present at the predecessor commit is present at the
  candidate commit, and none has been made unconditional.
- **Tier disjointness and totality.** Every observer test is in exactly one tier, and the union of the tiers is the
  whole suite. No test is in both and none is in neither.
- **Public-tier purity.** Each public-tier target compiles against its package's library target alone, with no
  `#[path]` include of a source file, no `include!`, and no other route back into private code.
- **Census conservation.** 109 observer tests, 60 engine tests, 169 total, 0 ignored, before and after.
- **Assertion conservation.** For each relocated test, the number and content of its assertions are unchanged.
- **Content conservation.** For each relocated file, content is byte-identical apart from enumerated path
  references.
- **No state escape.** No public item of the observer returns a mutable borrow of, or a reference into, engine-owned
  state, and none constructs an `Observation` or implements a `DecisionSource`.
- **Dependency conservation.** The engine's dependency graph is one crate; the observer's is the engine plus the
  `ratatui` graph fixed by `SPEC-MOK-003`, at unchanged versions.
- **Behavior surface unchanged.** No simulation constant, event field, event order, summary field, exit code,
  diagnostic message, byte of `USAGE`, key binding, layout tier, glyph or export field differs.

## Static and architecture checks

- `cargo fmt --all -- --check` reports no differences.
- `cargo clippy --all-targets --all-features -- -D warnings` reports no findings, and in particular no
  `non_snake_case` finding against the observer's library target name and no `dead_code` finding against a private
  item left without a user by the relocation. Neither the invocation nor an `allow` attribute is used to reach this
  result.
- `cargo build` and `cargo test` succeed workspace-wide and under each `-p` form.
- The `ARCH-MOK-001` conformance checks are re-run unchanged: the engine builds as one Cargo package with exactly
  one library target and one binary target and an empty dependency table; its graph contains no network,
  async-runtime, database or UI library; its public interface matches `SPEC-MOK-002` rule 5; no public item leaks
  mutable state. That this artifact needs no amendment is itself confirmed, by establishing that it names no source
  path.
- The `ARCH-MOK-002` conformance checks are re-run as amended, including the two added by `ADR-MOK-004`: the
  observer's public interface differs from its pre-change visibility in no item, and both of its tiers run under one
  `cargo test` with no terminal.
- `ADR-MOK-001`'s validation checks are re-run unchanged, in particular that no decision source receives a mutable
  world, agent collection, resource collection, event log or engine handle, and that deterministic replay holds.
- `ADR-MOK-003`'s validation checks are re-run: exactly two packages, one repository, one version, one candidate
  commit, and the engine dependency-free.
- The `SPEC-MOK-002`, `SPEC-MOK-003`, `ARCH-MOK-002` and `REPOSITORY_CONTEXT.md` amendments named in `ADR-MOK-004`
  are present and approved before the change is verified.
- `REPOSITORY_CONTEXT.md`'s *Commands* and *Architecture* sections name the new paths, and its test-placement
  sentence states both packages' tiers with authority cited for each.

## Security and privacy checks

- No network access, credential read, filesystem read, environment read or wall-clock read is introduced. Neither
  package's dependency set grows, and the engine's stays empty.
- The ten names `SPEC-MOK-002` rule 6 keeps private are confirmed private, and `Observation` and `DecisionSource`
  in particular are confirmed unreachable from either package's public interface.
- No test-support seam, feature flag, conditional visibility or self dev-dependency exists in any build
  configuration, including test builds. The four hooks are confirmed absent from the library target and from every
  public-tier file.
- Every public item of the observer is confirmed to be a presentation value, a pure function of a value, a
  read-only accessor returning a copy, or the export writer, so the library target grants no capability that did not
  already exist inside the crate.
- The export path stays data: it is validated as a string only, is never interpreted as code, is never used to read,
  and is not opened at start-up. The invalid-input diagnostics of both packages are byte-identical to the baseline.
- No credential, secret, environment variable, absolute path or wall-clock value appears in a frame or an export.
- The observation-to-proposed-action trust boundary is unmoved and unreachable from either public interface.

## Performance and resilience checks

- A 10,000-tick engine run under each decision source completes without panic, and survivors plus deaths equal
  twelve, as before.
- The 1,000-tick population floor at the declared density is met on every declared seed, with survivor counts
  identical to the recorded baseline rather than merely above the floor.
- The observer sustains its declared frame and input intervals at each declared viewport, with the same measured
  headroom as the baseline.
- Compile-time and `target/` growth from the additional targets is recorded as evidence. No pass condition attaches
  to it; it is retained so that a later objection to the arrangement can be assessed against a measurement.
- Runtime work per tick, per frame, memory use and output volume are unchanged, which follows from byte-for-byte
  equivalence and needs no separate measurement.

## Manual assessments

- Read `mokiterions-tui/src/lib.rs` and confirm it declares the seven modules and the `#[cfg(test)]` module and does
  nothing else. A `lib.rs` that has acquired an item is an adverse observation.
- Read `mokiterions-tui/src/main.rs` and confirm it declares no module and that its retained content is what it was:
  start-up, the launch decision, the loop, scheduling, the idle calculation and the diagnostic report. Confirm it is
  *not* a shim, and that no item was made public in the course of leaving it as it is.
- For each of the eight public-tier files, judge whether its subject is one a reader would recognise. A file that is
  a grab bag is an adverse observation even when every test in it is correctly placed.
- Confirm that no relocated test reads as weaker than its predecessor. Where a test's access path changed from
  `use crate::…` to `use mokiterions_tui::…`, confirm the value asserted is the same value.
- Confirm that the reason each of the 32 inline tests stayed inline is the access it requires, and not that
  relocating it looked laborious. The 12 in `render.rs` are the ones to press on, since that module has the largest
  private surface and the largest inline remainder.
- Judge whether the provenance-closed interface of `SPEC-MOK-004` rule 6 is legible to a reader who had not seen the
  change. An interface a reader cannot enumerate without running a script is an adverse observation, and the
  per-module counts are what the reader is given instead of a table.
- Read the enumerated path references that the move required, and confirm each is required by the move rather than
  an edit taken while the file was in transit.
- Confirm that the directory names, and only the directory names, changed for the engine, by reading the engine's
  relocated manifest against its predecessor.

## Evidence retention

Retain under `docs/engineering/simulation/evidence/WO-MOK-006/`:

- the recorded pre-change baseline: engine output for every declared seed, decision source, declared density and
  trace setting, exit codes and standard-error text for every invalid-input case, and observer frames and exports
  for every declared seed and viewport, with the commit each was captured at;
- the post-change comparison result for both matrices, and the comparison method;
- the non-test content comparison for each of the observer's seven `pub mod` files, against the predecessor commit,
  with `verification.rs`'s exclusion stated and its retained eight tests compared separately;
- the public-item census of the observer's library target, with per-module counts, the counting rule it applied
  stated explicitly, and its comparison against the 97 recorded here;
- the confirmation that all four `#[cfg(test)]` hooks retain their attribute and that no public-tier file names one;
- the test census: `cargo test -- --list` before and after, per package and per test binary, with the name-by-name
  reconciliation;
- the per-test placement justification, naming for each of the 32 inline tests the private item or hook it requires
  and for each of the 77 relocated tests the public items it uses;
- the per-test comparison showing that relocated assertions are verbatim;
- the per-file comparison showing that relocated content is byte-identical, with every required path reference
  enumerated and justified;
- the executed output of every command form in `SPEC-MOK-004` rule 14, including the `USAGE` first line and the two
  `cargo tree` results;
- the root manifest and both package manifests as they stand at the candidate commit;
- the `Cargo.lock` comparison;
- formatter, linter, build and test output, workspace-wide and per package;
- the re-run `ARCH-MOK-001`, `ARCH-MOK-002`, `ADR-MOK-001` and `ADR-MOK-003` conformance results;
- the 10,000-tick resilience result, the 1,000-tick per-seed survivor counts, and the observer's frame-interval
  measurement, each compared against the baseline;
- the compile-time and `target/` measurements, with no obligation attached;
- a superseding requirement-to-test mapping covering every case in `VER-MOK-001`, `VER-MOK-002`, `VER-MOK-004` and
  `VER-MOK-005` at its new location, stating explicitly that the mappings retained under `WO-MOK-002` through
  `WO-MOK-005` remain valid for their own commits and are not edited;
- confirmation that the `SPEC-MOK-002`, `SPEC-MOK-003`, `ARCH-MOK-002` and `REPOSITORY_CONTEXT.md` amendments were
  approved before the work began, with their approval dates;
- a completion summary naming the final directory layout, the observer's final target layout, the final public-item
  count, and the final tier populations of both packages.

## Residual uncertainty

- **No compile-fail harness exists.** That a public-tier test cannot reach a private item or a hook is verified by
  census, comparison and review, not by a mechanical proof that a violating test fails to compile. Such a proof
  would need a compile-failure test dependency, which is excluded. Review is load-bearing here, and a reviewer who
  reads only the tests will not detect a widened item that no test uses — which is why the content comparison is
  independent of the suite.
- **Provenance closure is only as good as the predecessor commit.** Rule 6 defines the interface as what was public
  before, so this contract verifies that the interface did not change and says nothing about whether the 97 items
  are the right 97. If some of them are public today only by accident, that accident is now the contract. Narrowing
  it is a later requirement and an amendment to `SPEC-MOK-004`, not a defect found here.
- Byte-for-byte equivalence is demonstrated across the declared matrices, not across the whole input space. It does
  not prove equivalence at an undeclared seed, density or viewport, though neither a directory move nor a library
  target has a plausible mechanism for input-dependent divergence.
- The placement rule is verified against the 109 observer tests that exist. It is not proven that every future
  observer test has an unambiguous tier, and a genuinely ambiguous case is an escalation rather than a judgement
  call.
- The measured 77/32 split is a measurement of the tests as written, and rule 9 states that rule 8 governs if
  implementation finds a test assigned differently. A corrected count is therefore not automatically a defect; what
  fails this contract is a corrected count that is not recorded, or one that moves a test by weakening it.
- This contract does not verify that the observer's public interface is *well designed*, only that it is counted,
  read-only, unchanged, and sufficient for the 77 relocated tests. Whether it is the right contract for a later
  phase will be established when that phase is written against it.
- The legibility benefit of one directory per package is not measured and is not measurable by a check here. What is
  verified is the property — each package's files under one directory named for it — and not that a reader is
  thereby better served.
- The initiative's central benefit, that a break in what the observer presents now fails a test with no private
  access to repair itself with, is not directly measured. Demonstrating it would require deliberately breaking the
  observer's contract, which is out of scope. What is verified is that a populated tier of 77 tests reaches the
  observer only through that contract, which is the precondition for the benefit rather than the benefit itself.
