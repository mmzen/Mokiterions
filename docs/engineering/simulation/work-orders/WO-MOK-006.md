+++
id = "WO-MOK-006"
type = "work_order"
title = "Give each package its own directory and the observer its own tested contract"
status = "draft"
owners = ["engineering owner"]
created = "2026-08-18"
updated = "2026-08-18"

[assurance]
commit_bound_verification = "required"
rationale = "The work relocates every source and test file of both packages, adds a library target that turns 97 already-public items into a maintained contract, and moves 77 of the observer's 109 tests out of the crate. Its central claim is that none of this changed a byte of either package's behavior, which is a claim of the same standing as REQ-MOK-009's determinism claim and cannot be asserted. It also changes trusted engineering state that later decisions rest on: the repository's build layout, the commands CI and operators run, and the location of the whole test corpus. A file move produces a diff in which every line is new, so review alone cannot establish faithfulness and the comparison against the predecessor commit must be bound to a commit."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-028", "REQ-MOK-029", "REQ-MOK-030"]
specifications = ["SPEC-MOK-004", "SPEC-MOK-002", "SPEC-MOK-003"]
architecture = ["ARCH-MOK-002", "ADR-MOK-003", "ADR-MOK-004"]
verification = ["VER-MOK-006"]
+++

# Work Order: Give each package its own directory and the observer its own tested contract

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

**This work order cannot be approved before its governing artifacts are.** It depends on `INT-MOK-005`,
`CAP-MOK-005`, `REQ-MOK-028`, `REQ-MOK-029` and `REQ-MOK-030` being approved by the product owner; on `SPEC-MOK-004`
and `ADR-MOK-004` being approved by the technical owner; and on `VER-MOK-006` being approved by the assurance owner.
Every one of them is `draft` at the time this work order is written. Preflight must report this exact work order as
eligible before implementation begins.

It depends additionally on four amendments to artifacts that are already approved, and in two cases already bound by
a `verified` record. `ADR-MOK-004`'s *Required amendments* section states them in full:

- **`SPEC-MOK-002`** — every clause that names a root-relative path, and rule 1's note asserting that the engine
  package is unchanged "in source location". The rules bind unchanged in substance; the paths move under
  `mokiterions-core/`. This specification is bound by a `verified` `VREC-MOK-003`.
- **`SPEC-MOK-003`** — the *Component layout* tree and its clause 3, which fix the engine's sources at the root and
  state that they "are not relocated"; *Data and interface contracts* rule 2, whose reasoning appeals to that clause;
  and both relevant entries of *Explicitly unspecified decisions*, one of which withholds the package layout and one
  of which delegates test organization. This specification is bound by a `verified` `VREC-MOK-005`.
- **`ARCH-MOK-002`** — the component that calls the observer host "the new binary", the *Testability without a
  terminal* quality attribute, its required and prohibited pattern lists, its conformance checks, its `addresses` and
  `conforms_to` relations, and its `decision_assessment.rationale`.
- **`docs/engineering/REPOSITORY_CONTEXT.md`** — its *Commands* and *Architecture* sections, and the sentence stating
  the two-tier test convention repository-wide on engine-scoped authority. That sentence's disagreement with the code
  is what `WO-MOK-005` disclosed as its fifteenth finding; this work order is what makes it true rather than what
  weakens it. `REPOSITORY_CONTEXT.md` is repository-owned guidance, not a governed artifact, so it is brought into
  line rather than amended.

**None of the four is part of any prior approval, and all four are outstanding.** They are the technical owner's act.
Writing the amended text is implementation; approving it is not, and each amendment record must say which of the two
happened. `ARCH-MOK-001` requires no amendment, which is itself an obligation to confirm rather than an assumption:
it names no source path, and every quality attribute and conformance check it states about the engine holds after the
move.

Both ADRs that decide `ARCH-MOK-002` are selected above. `ADR-MOK-004` decides the two points this work order
changes — the observer's target shape and the repository's package-directory layout. `ADR-MOK-003` is selected
because it decides the same architecture and remains in force unchanged: exactly two packages, one repository, one
version, one candidate commit, and an engine free of external dependencies. `VER-MOK-006` re-runs its validation
checks rather than assuming the move preserved them.

## Objective

Put each package's manifest, sources and tests under one directory named for that package, give the observer package
a library target whose public interface is exactly what was already public, and place every observer test in the tier
the access it requires assigns it to — while every operator-facing name, every command, and every byte either package
emits stays exactly as verified.

## In scope

1. **Relocate the engine package.** Move its manifest, `src/` and `tests/` under `mokiterions-core/`, with the
   manifest's tables unchanged. Make the root `Cargo.toml` a workspace manifest and nothing else, declaring the two
   members and `resolver = "3"`. Re-point the observer's path dependency at `../mokiterions-core`, still keyed by the
   engine's package name. `SPEC-MOK-004` rules 1 to 3 are the authority.
2. **Add the observer's library target.** Declare `[lib] name = "mokiterions_tui" path = "src/lib.rs"` and create
   `mokiterions-tui/src/lib.rs` containing the seven `pub mod` declarations and `#[cfg(test)] mod verification;` and
   nothing else. `SPEC-MOK-004` rules 4 and 5 are the authority.
3. **Re-point the observer's binary target at the library.** Remove its module declarations, reach the presentation
   layer as `use mokiterions_tui::…`, and leave its start-up, its launch decision, its loop, its scheduling, its idle
   calculation, its diagnostic report and its own `#[cfg(test)] mod tests` where they are.
4. **Classify all 109 observer tests by required access** and relocate the 77 that the interface suffices for into
   the eight files of `SPEC-MOK-004` rule 9, with their assertions verbatim. Leave the 32 of rule 10 inline.
5. **Apply the four amendments** listed under *Lifecycle*, each in its own artifact, each recorded in that artifact's
   amendment record with its approval status stated.
6. **Produce a superseding requirement-to-test mapping** covering every case in `VER-MOK-001`, `VER-MOK-002`,
   `VER-MOK-004` and `VER-MOK-005` at its new location.
7. **Complete `VER-MOK-006`** in full, including its comparison evidence and its manual assessments, and retain
   everything its evidence-retention section lists.

## Out of scope

- Any change to observable behavior in either package. No simulation rule, event, summary field, exit code,
  diagnostic, byte of `USAGE`, frame, pane, glyph, key binding, layout tier or export field changes.
- Renaming anything. Package `Mokiterions`, library `mokiterions`, binary `Mokiterions`, package and binary
  `mokiterions-tui`. `mokiterions-core` is a directory name only.
- Widening any item's visibility, and removing or gating any `#[cfg(test)]` attribute. The four hooks stay as they
  are, and the 16 tests that use them stay inline.
- Adding a public item to either package. The engine's interface is untouched and the observer's admits nothing.
- Restructuring the engine's tests. Their tier placement is `WO-MOK-003`'s verified outcome; only the directory
  containing them moves.
- Making the observer's binary target thin, or moving its start-up or loop into the library. `ADR-MOK-004`'s Option 4
  is the rejected alternative and the rejection is not to be revisited here.
- Any new dependency, dev-dependency, feature, build script, workspace dependency table, test framework, assertion
  library or snapshot tool, for any reason including test ergonomics.
- Any new package, target, binary, output stream, pane, simulation rule or product feature.
- Editing a record bound to a commit. `VREC-MOK-001` through `VREC-MOK-005` and the evidence retained under
  `WO-MOK-001` through `WO-MOK-005` name paths this work order moves and are left as written.
- The pre-existing artifact defects disclosed under `WO-MOK-005` that are unrelated to layout or test placement.
  They are recorded where they were disclosed and are not corrected opportunistically here.
- Any other product domain and any harness-managed policy file.

## Authorized decision envelope

The implementation agent may decide locally:

- the order of items within each source and test file, and the order of the seven `pub mod` declarations;
- whether public-tier helpers are duplicated per file or shared through a `tests/common/` module, in either package;
- whether the binary target imports the library's modules individually or as a group;
- whether an internal-tier helper is kept, renamed or consolidated, provided no assertion changes;
- doc comments, internal comments, and whether each manifest keeps the specification-citing comments it carries
  today;
- the order in which the relocation, the library target and the test moves are performed, provided every
  `SPEC-MOK-004` rule holds at the candidate commit;
- the mechanism used to compare relocated content against the predecessor commit, provided the comparison is
  mechanical and its method is recorded;
- the tier a test is assigned to **when rule 8 assigns it differently than `SPEC-MOK-004` rules 9 and 10 measured**.
  Rule 8 governs. The corrected count is recorded as evidence and the discrepancy reported; a test is never moved by
  weakening it, and a count is never reached by widening an item.

It may not decide anything the *Out of scope* or *Constraints* sections exclude, and it may not resolve a
specification contradiction by choosing one reading.

## Constraints

- `SPEC-MOK-004` is the structural authority. `SPEC-MOK-001` and `SPEC-MOK-003` remain the behavior authorities and
  are preserved byte for byte. `SPEC-MOK-002` remains the engine package's target, interface and test-placement
  authority, with its paths read under `mokiterions-core/`.
- The engine's dependency and dev-dependency tables stay empty, with no exception, including a dependency shared
  with the other member. `cargo tree -p Mokiterions` resolving to one crate is the proof.
- The observer's dependency set stays at one path dependency and one external dependency, at the version and feature
  set `SPEC-MOK-003` fixes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes workspace-wide with no `allow` attribute added
  and no narrowing of the invocation. A `dead_code` finding against a private item left without a user is a
  misclassification signal, not something to silence.
- Every observer rendering claim is asserted against an in-memory character buffer. No screenshot and no recording is
  admissible evidence for any rendering obligation, in either tier.
- Relocated content is compared against the predecessor commit, not reviewed as a diff.
- No credential, secret, token or environment value enters the repository, and neither package reads one.
- The move is not combined with any other change. A commit that both moves a file and edits its content makes the
  central claim unverifiable.

## Expected change surface

- `Cargo.toml` at the root: from a workspace-plus-package manifest to a workspace manifest declaring two members and
  the resolver.
- `mokiterions-core/Cargo.toml`, `mokiterions-core/src/` and `mokiterions-core/tests/`: the engine package's
  manifest and eight files, relocated with content unchanged.
- `mokiterions-tui/Cargo.toml`: a `[lib]` section added and the engine path dependency re-pointed.
- `mokiterions-tui/src/lib.rs`: new, seven `pub mod` declarations and one `#[cfg(test)] mod`.
- `mokiterions-tui/src/main.rs`: module declarations removed, access paths changed, everything else unchanged.
- `mokiterions-tui/src/authority.rs`, `export.rs`, `layout.rs`, `options.rs`, `spatial.rs`: their `#[cfg(test)]`
  modules removed entirely, non-test content byte-identical.
- `mokiterions-tui/src/render.rs`, `state.rs`, `verification.rs`: their `#[cfg(test)]` blocks reduced to the tests
  rule 10 keeps, non-test content byte-identical.
- `mokiterions-tui/tests/`: eight new files holding 77 relocated tests.
- `Cargo.lock`: recorded paths only.
- `.github/workflows/` and any script naming a package path or a member list, if the layout changes what they
  resolve. Whether any does is established by running them, not by reading them.
- `SPEC-MOK-002`, `SPEC-MOK-003` and `ARCH-MOK-002`: the amendments under *Lifecycle*, in their amendment records.
- `docs/engineering/REPOSITORY_CONTEXT.md`: commands, architecture, and the test-placement sentence.
- `docs/mokiterions/ROADMAP.md`, to record the restructure, if it records structural phases.
- Work-order-keyed evidence under `docs/engineering/simulation/evidence/WO-MOK-006/`.
- No other product domain and no harness-managed policy file.

## Required verification

- Capture the pre-change baseline **before any file moves**, at the commit the work begins from: the engine's output,
  final state and exit codes across every declared seed, decision source, declared density and trace setting; the
  invalid-input exit codes and standard-error text; the observer's frames and exports at every declared seed and
  viewport; and `cargo test -- --list` for both packages. The baseline is captured once and is not regenerated after
  a discrepancy.
- Complete every case, invariant, check and manual assessment in `VER-MOK-006`.
- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo build` and `cargo test` at the workspace root, and each under `-p Mokiterions` and
  `-p mokiterions-tui`.
- Run `cargo tree -p Mokiterions` and retain the output as the empty-dependency-set proof; run
  `cargo tree -p mokiterions-tui` and compare its graph against the `SPEC-MOK-003` figures.
- Execute every command form in `SPEC-MOK-004` rule 14 at the restructured root, including
  `cargo run --bin Mokiterions`, and retain each result. None is inferred from another.
- Extract the non-test content of each of the observer's seven `pub mod` files at the predecessor commit and at the
  candidate commit, and compare. All eight byte-identical is the `REQ-MOK-028` pass condition.
- Compare every relocated file against its predecessor-commit source, and enumerate each path reference the move
  required.
- Count the library target's public items per module under `SPEC-MOK-004` rule 6's counting rule, state the rule
  applied, and compare against the recorded 97 and its 25 accompanying fields and variants.
- Reconcile the test census name by name: 109 observer, 60 engine, 169 total, 0 ignored, before and after.
- For each of the 32 inline tests, record the private item or hook it requires; for each of the 77 relocated tests,
  record the public items it uses and that its assertions are verbatim.
- Re-run the `ARCH-MOK-001`, `ARCH-MOK-002`, `ADR-MOK-001` and `ADR-MOK-003` conformance and validation checks.
- Run the engine's 10,000-tick resilience case and the 1,000-tick per-seed population floor, comparing survivor
  counts against the baseline rather than against the floor.
- Confirm the observer sustains its declared frame and input intervals at every declared viewport.
- Perform and record the manual assessments, including the legibility assessment of the provenance-closed interface
  and the press on `render.rs`'s twelve inline tests.

## Evidence to record

Retain, under `docs/engineering/simulation/evidence/WO-MOK-006/`, every item listed in `VER-MOK-006`'s evidence
retention section. Four records carry the load and none of the others substitutes for them:

1. the **non-test content comparison** for the observer's seven `pub mod` files, which is the only proof that no item's
   visibility changed;
2. the **per-file comparison** of every relocated file against the predecessor commit, with each required path
   reference enumerated, which is the only proof that the move edited nothing;
3. the **executed output of every rule 14 command form**, which is the only proof that a virtual workspace root did
   not change what the operator's commands mean;
4. the **`cargo tree -p Mokiterions` output**, which is the only proof that the layout cost the engine's empty
   dependency set nothing.

No screenshot and no recording is admissible for any rendering obligation; retain buffer dumps accompanying their
assertions instead.

## Stop and escalate conditions

Stop before implementation if preflight does not report this exact work order as eligible, or if any governing
artifact remains `draft` or unapproved — in particular `SPEC-MOK-004`, without which no directory layout, no library
target and no observer tier is specified; the `SPEC-MOK-002` path amendments, without which the engine's own
specification would name paths that no longer exist while being bound by a `verified` record; the `SPEC-MOK-003`
amendments, without which a component layout bound by a `verified` record forbids the relocation outright; and the
`ARCH-MOK-002` amendments, without which an approved architecture describes the observer as a single binary.

Stop before implementation if the pre-change baseline cannot be captured, because every equivalence claim in
`VER-MOK-006` then has no oracle.

During implementation, stop and escalate if:

- any observer test can only be relocated by widening an item or by removing a `#[cfg(test)]` attribute. The correct
  outcome is that the test stays inline and the count is corrected; if the corrected count falls below the
  `INT-MOK-005` target of 70, that is a product decision and not an implementation one;
- a relocated test's assertion cannot survive the move. It returns to the internal tier; it is never loosened;
- the non-test content of any module file cannot be kept byte-identical. Whatever change appears necessary is a
  specification question, since rule 6 rests on that identity;
- `cargo run --bin Mokiterions` or any other rule 14 form cannot be made to resolve without renaming a package or a
  target. Rule 2 permits one workspace key as the correction; if no key restores it, escalate rather than rename;
- the engine's dependency table cannot be kept empty, or the observer's dependency cannot be confined to its own
  package, after the move;
- a `dead_code` or other lint finding cannot be resolved by returning a test to its correct tier, and would require
  an `allow` attribute, a narrowed invocation, or the deletion of an item;
- any relocated file's content differs from its source by anything other than a required path reference, however
  small and however plainly harmless;
- either package's output, exit codes, frames or exports differ from the baseline in any byte at any declared input.
  Equivalence is the point of the work, so a difference means the work is wrong, not that the baseline is stale;
- an existing `VER-MOK-001`, `VER-MOK-002`, `VER-MOK-004` or `VER-MOK-005` test fails, or would need to be modified;
- CI cannot resolve the workspace, or a workflow or script requires a change beyond a path or a member list;
- the observer's `verification` module cannot be declared from `lib.rs` without exposing an item;
- specified behavior is contradictory or needs a product or technical decision;
- required verification fails and cannot be corrected within authorized scope;
- requested changes expand into an excluded feature, a rename, or another artifact domain.

## Completion report format

Report:

1. implemented requirements and affected components;
2. any authorized local decisions, including the public-tier helper arrangement and the comparison mechanism used;
3. verification commands and results;
4. the final directory layout and the contents of all three manifests;
5. the non-test content comparison result for all eight observer module files, stated per file;
6. the per-file relocation comparison result, with every required path reference enumerated;
7. the executed result of every `SPEC-MOK-004` rule 14 command form, including the `USAGE` first line and both
   `cargo tree` outputs;
8. the public-item count per module, against the recorded 97;
9. the test census reconciliation and the final tier populations of both packages, with any corrected assignment
   named and justified by required access;
10. the amendment status of `SPEC-MOK-002`, `SPEC-MOK-003`, `ARCH-MOK-002` and `REPOSITORY_CONTEXT.md`, stating for
    each whether the text was written here and by whom it was approved;
11. retained evidence paths;
12. residual limitations and explicitly deferred items;
13. final worktree and candidate-commit status.
