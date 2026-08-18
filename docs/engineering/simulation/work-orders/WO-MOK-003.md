+++
id = "WO-MOK-003"
type = "work_order"
title = "Split the crate into library and binary targets and place tests by required access"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-17"
updated = "2026-08-17"

[assurance]
commit_bound_verification = "required"
rationale = "This work changes trusted engineering state that later decisions rely on: the crate's target layout, the public interface that Phase 4 and Phase 5 will be written against, and the location of the tests that carry the evidence for every previously verified requirement. Its central claim is an equivalence claim, and assurance and release decisions will rely on that claim having been demonstrated against a recorded baseline at a specific commit rather than asserted."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-016", "REQ-MOK-017"]
specifications = ["SPEC-MOK-002", "SPEC-MOK-001"]
architecture = ["ARCH-MOK-001", "ADR-MOK-001", "ADR-MOK-002"]
verification = ["VER-MOK-003"]
+++

# Work Order: Split the crate into library and binary targets and place tests by required access

## Lifecycle

This work order was approved on 2026-08-17 by the repository owner acting as technical, engineering, and assurance
owner, and moved to `in_progress` on the same date. Approval authorizes only the scope below. Transition to
`implemented` requires the completed change and retained evidence. Verification and release require separate
commit-bound records.

Moved to `implemented` on 2026-08-17. The change is complete, `cargo fmt`, `cargo clippy -D warnings`, `cargo build`,
and `cargo test` all pass with 52 tests and no failures, skips, or ignores, and every item in `VER-MOK-003`'s
evidence retention list is retained under `docs/engineering/simulation/evidence/WO-MOK-003/`. The equivalence claim
is demonstrated byte-for-byte against the pre-change baseline captured at commit
`77010d02319051a20f8e45282f9c813ce4199956`. `implemented` records that the authorized change was made and its
evidence retained; it asserts no verification verdict.

Commit-bound verification is classified `required` above, so assurance acceptance requires a separate
verification record naming a candidate commit. The work was retained on branch
`feature/library-target-and-test-placement` in one clean candidate commit,
`a7f39f18520433cddc72c044c6fca1b24104bb7d`, and `VREC-MOK-003` was then prepared by
`harnessctl capture-verification` at `ready`, binding that commit. This work order stays `implemented`: verification
is carried by that record, and its transition from `ready` to `verified` is the accountable assurance owner's
separate later act.

Architecture is selected, unlike `WO-MOK-002`, and the selection is not nominal. `ARCH-MOK-001` states one binary
crate as a quality attribute and as a conformance check and prohibits separate crates without an approved
requirement, so this work order cannot be conformed to under the architecture as written; `ADR-MOK-001` states the
same constraint as a decision bullet; and `ADR-MOK-002` is the instrument that revises both on that one point.
`REQ-MOK-016` is architecturally significant under `ARCH-MOK-001`'s already-declared `public-interface-or-protocol`
trigger, which is why the required amendment adds it to that architecture's `addresses` relation.

`SPEC-MOK-001` is selected alongside `SPEC-MOK-002` because it is the contract this work order must leave
untouched. `SPEC-MOK-002` says what to build; `SPEC-MOK-001` says what must not change, and byte-for-byte
equivalence against it is the central obligation.

### Approval preconditions

Approval was blocked until all four held. They are governance acts the technical owner owns, and no implementation
could begin before they were complete and approved. All four were satisfied on 2026-08-17 by the repository owner
acting as technical owner, and approval followed.

1. **Satisfied.** `ARCH-MOK-001` is amended exactly as tabulated in `ADR-MOK-002`, including the `addresses` and
   `conforms_to` relation additions, and the amendment is recorded in that architecture's amendment record.
2. **Satisfied.** The technical owner decided in-place amendment rather than supersession, as `ADR-MOK-002`
   recommends, and applied it: `ADR-MOK-001`'s decision bullet now reads "one Cargo package, built as a library
   target and a thin binary target", with a dated note, and its *Status* records the amendment.
3. **Satisfied.** `SPEC-MOK-001`'s *Explicitly unspecified decisions* entry on test organization is narrowed to
   helper functions and test-module-internal organization, with target layout, public interface, and test placement
   delegated to `SPEC-MOK-002`, recorded as a row in its amendment record.
4. **Satisfied.** `ADR-MOK-002`, `SPEC-MOK-002`, `INT-MOK-003`, `CAP-MOK-003`, `REQ-MOK-016`, `REQ-MOK-017`, and
   `VER-MOK-003` are `approved`.

The implementation agent that drafted this packet did not approve any of the above. `DECISION_RIGHTS.md` assigns
specification and architecture to the technical owner and prohibits an implementation agent from self-approving an
architecture decision assessment or its supporting ADR. The repository owner, who holds every accountable role in
this repository, cleared the gate explicitly and directed the amendments to be applied as tabulated.

## Objective

Give the program a stated, enumerated, read-only public interface and a tier of tests that reaches it, by splitting
the single binary target into a library target and a thin binary target and relocating exactly those tests that
require nothing more than that interface — while changing no observable behavior, as specified in `SPEC-MOK-002` and
covered by `VER-MOK-003`.

## In scope

- Declare the two targets of `SPEC-MOK-002` rule 1 in `Cargo.toml`: `[lib] name = "mokiterions", path = "src/lib.rs"`
  and `[[bin]] name = "Mokiterions", path = "src/main.rs"`.
- Add `src/lib.rs` declaring `pub mod cli;` and `pub mod simulation;`.
- Move `execute` from `src/main.rs` to `src/lib.rs` and make it public, with its signature, behavior, diagnostic
  text, and exit codes unchanged.
- Reduce `src/main.rs` to the shim of `SPEC-MOK-002` rule 3: startup, stream locking and buffering, one call to
  `execute`, flush handling, exit-code mapping. No module declarations, no tests.
- Add only those items from `SPEC-MOK-002` rule 5's authorized-additions list that a relocated test actually
  requires: a public `TerminationReason`, read-only copying `RunSummary` accessors, a public
  `Density::resources_per_territory`, and a public `CELLS_PER_TERRITORY`.
- Relocate to `tests/` every test that can be written using only rule 5's interface with its assertions unchanged —
  expected to be the five parsing tests, the four exit-code tests, and the density, termination, and viability tests
  that need only the summary and resolved density, arranged into the five files of rule 8.
- Leave in `#[cfg(test)] mod tests` every test that requires deliberately non-public state, with its assertions and
  helpers unchanged.
- Capture the pre-change baseline required by `VER-MOK-003` before making any change, and the post-change comparison
  after.
- Produce the superseding requirement-to-test mapping covering every case in `VER-MOK-001` and `VER-MOK-002` at its
  new location.
- Update `REPOSITORY_CONTEXT.md`'s *Architecture* section so the entry-point line names the library and binary
  targets. Its test-placement constraint is already recorded and needs no change.
- Retain every evidence item listed in `VER-MOK-003` under this work order's ID.

## Out of scope

- Any change to observable behavior. No simulation constant, event field, event order, summary field, exit code,
  diagnostic message, or byte of `USAGE` may change.
- Any change to `SPEC-MOK-001`'s specified behavior, to perception, survival, regeneration, density mapping,
  termination, or either decision source.
- A test-support seam, feature flag, conditional visibility, self dev-dependency, or build script.
- A second Cargo package, a workspace, a third target, or any dependency including a test or assertion library.
- Relocating tests that require authoritative state, and any widening of visibility undertaken to relocate one.
- Splitting `src/simulation.rs` into several modules, renaming private items, or any other refactor not required by
  a rule in `SPEC-MOK-002`. Tidying is not authorized here.
- Doc-comment coverage, published API documentation, or a semantic-versioning commitment.
- Perception, fear, traits, combat, social behavior, model-provider integration, structured output, persistence, or
  aggregate analysis.
- Editing `WO-MOK-002`'s retained evidence, `VREC-MOK-001`, or `VREC-MOK-002`.
- Any harness-managed policy file, and any artifact lifecycle change outside this work order.

## Authorized decision envelope

The implementation agent may choose:

- the exact names and signatures of the `RunSummary` accessors, provided each returns an owned copy;
- ordering of items within each source and test file;
- whether public-tier helpers are duplicated per file or shared through a `tests/common/` module, and where the
  failing-writer helper lives;
- whether the internal tier's existing helpers are kept, renamed, or consolidated, provided no assertion changes;
- which authorized rule 5 additions to omit, when no relocated test requires them;
- the comparison mechanism for the baseline, provided it is byte-exact and its method is recorded;
- comments and non-authoritative developer notes.

The implementation agent may **not**:

- add any public item outside `SPEC-MOK-002` rule 5's two lists, or widen any item to relocate a test;
- change any assertion in any test, whether relocated or left inline;
- change `execute`'s signature, its exit codes, or any diagnostic string;
- change target names, target paths, or the package name;
- relax the lint gate, narrow its invocation, or add an `allow` attribute to satisfy it;
- add a dependency, dev-dependency, feature, or `cfg`-conditional visibility;
- regenerate the pre-change baseline after observing a discrepancy;
- change requirement scope, specification content, or any artifact's lifecycle status.

A test that will not compile in the public tier is a placement signal, not a request for a rule 5 addition. The
correct response is to leave it in the internal tier and record why.

## Constraints

- Follow `INT-MOK-003`, `SPEC-MOK-002`, and `VER-MOK-003`, and preserve everything verified under `VREC-MOK-001` and
  `VREC-MOK-002` without exception.
- Keep one Cargo package with exactly two targets, and prefer the standard library. Dependencies and
  dev-dependencies remain empty.
- Keep every decision source unable to mutate authoritative state and unable to act at a distance. `REQ-MOK-004`
  and `ADR-MOK-001` are unchanged by this work and constrain it fully.
- Preserve `REQ-MOK-009`. Do not move, split, re-seed, or re-order the entropy stream. A target split has no reason
  to touch it, and any diff that does is out of scope.
- Capture the baseline first. A change made before the baseline exists cannot be verified and must be reverted.
- Keep the binary target thin. Logic that migrates from `main.rs` goes to `lib.rs` unchanged, not rewritten.
- Preserve unrelated user changes and do not modify generated output as source.

## Expected change surface

- `Cargo.toml`: two new target sections. `Cargo.lock` is not expected to change, since no dependency is added.
- A new `src/lib.rs`: two public module declarations and the relocated `execute`.
- `src/main.rs`: reduced to a shim; its four tests and its failing-writer helper move out.
- `src/simulation.rs`: visibility changes for `TerminationReason`, `CELLS_PER_TERRITORY`, and
  `Density::resources_per_territory`; new `RunSummary` accessors; removal of the test functions that relocate, with
  the remainder of the test module and its helpers intact.
- `src/cli.rs`: removal of the test functions that relocate; the parsing code itself unchanged.
- A new `tests/` directory containing the five files of `SPEC-MOK-002` rule 8.
- `docs/engineering/REPOSITORY_CONTEXT.md`: the *Architecture* entry-point line.
- Work-order-keyed evidence under `docs/engineering/simulation/evidence/WO-MOK-003/`.
- No other product domain and no harness-managed policy file.

## Required verification

- Complete every case, invariant, check, and manual assessment in `VER-MOK-003`.
- Capture the pre-change baseline at the starting commit: output for every declared seed, both decision sources,
  every declared density, and both trace settings, plus exit codes and standard-error text for every invalid-input
  case.
- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --all-targets --all-features -- -D warnings`.
- Run `cargo build`.
- Run `cargo test` once, with no extra flag or environment variable.
- Run `cargo test -- --list` before and after, and reconcile the census name by name against the recorded 52 tests
  and 0 ignored.
- Compare post-change output against the baseline byte-for-byte across the full matrix.
- Produce the public-surface inventory and diff it against `SPEC-MOK-002` rule 5.
- Re-run the 1,000-tick per-seed survivor measurement at the declared density and compare it against the baseline
  counts, not merely against the floor.
- Run one 10,000-tick resilience run under each decision source.
- Record the compile-time measurement, with no obligation attached.

## Evidence to record

Retain, under `docs/engineering/simulation/evidence/WO-MOK-003/`, every item listed in `VER-MOK-003`'s evidence
retention section.

Two items carry particular weight. The **pre-change baseline** is the only independent oracle for the equivalence
claim; it is captured once, before any edit, and is never regenerated. The **superseding requirement-to-test
mapping** is what keeps `VER-MOK-001` and `VER-MOK-002` traceable after relocation; it must state explicitly that
`WO-MOK-002`'s retained mapping remains valid for its own commit and was not edited, because that record is bound to
commit `68163ac` by `VREC-MOK-002`.

## Stop and escalate conditions

Stop before implementation if preflight does not report this exact work order as eligible, if any approval
precondition above is unmet, or if any governing artifact remains incomplete or unapproved.

During implementation, stop and escalate if:

- any output byte, exit code, or diagnostic string differs from the recorded baseline and the cause is not an obvious
  local mistake that can be corrected within this scope. A differing baseline is never accepted as the new truth;
- a test assigned to the public tier requires an item outside `SPEC-MOK-002` rule 5, and leaving it in the internal
  tier would leave a `VER-MOK-001` or `VER-MOK-002` case covered only by a weaker assertion;
- the public interface cannot be made sufficient for the relocated tests without exposing mutable or owned
  authoritative state;
- the lint gate cannot be satisfied by naming the targets as rule 2 specifies;
- the test census cannot be reconciled, because a test is lost, duplicated, or silently disabled;
- `cargo test` cannot run both tiers in one invocation without a feature, environment variable, or ignore attribute;
- determinism or the population floor changes in any way;
- an external dependency, dev-dependency, or build script appears necessary;
- a rule in `SPEC-MOK-002` proves contradictory, or a test's tier is genuinely ambiguous under rule 7;
- the required amendments to `ARCH-MOK-001`, `ADR-MOK-001`, or `SPEC-MOK-001` turn out to be insufficient or to
  conflict with another approved artifact;
- required verification fails and cannot be corrected within authorized scope;
- the change expands into tidying, renaming, module splitting, or another artifact domain.

## Completion report format

Report:

1. implemented requirements and affected components;
2. the final target layout, and the final public interface as an enumerated list with each item classified;
3. any authorized local decisions, including which rule 5 additions were omitted and why;
4. the tier populations, with the per-test placement justification and the count reconciliation;
5. verification commands and results, including the byte-for-byte comparison result across the full matrix;
6. the per-seed survivor counts compared against the baseline;
7. retained evidence paths;
8. residual limitations and explicitly deferred items;
9. final worktree and candidate-commit status.
