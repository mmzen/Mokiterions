+++
id = "VER-MOK-003"
type = "verification"
title = "Public contract and test placement verification"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
verifies = ["REQ-MOK-016", "REQ-MOK-017"]
+++

# Verification Contract: Public contract and test placement verification

## Independence

This contract verifies a restructuring that is required to change nothing observable, which creates a specific
hazard: a suite that passes because nothing changed proves nothing, since it would also pass if the restructuring
had never happened. Every check below is therefore written so that it can fail.

Three independent oracles are used, and none of them is the changed code's own test suite.

1. **A recorded pre-change baseline.** Output and exit codes are captured from the program *before* the change, at
   the commit the work begins from, and compared byte-for-byte afterwards. The baseline is evidence, captured once
   and not regenerated after a discrepancy. A discrepancy is a defect in the restructuring, never a new baseline.
2. **A public-surface inventory.** Every `pub` item reachable from the library root is enumerated mechanically and
   diffed against `SPEC-MOK-002` rule 5. Surplus and shortfall are both findings. This check does not read the
   tests, so it cannot be satisfied by a test that avoids a leaked item.
3. **A test census.** Test counts and identities are taken from `cargo test -- --list` before and after, and
   reconciled name by name. A test that disappears, is renamed without record, or becomes ignored is a finding
   regardless of whether the suite is green.

The recorded pre-change counts, fixed here so they cannot be adjusted afterwards, are **52 tests: 5 in `src/cli.rs`,
43 in `src/simulation.rs`, and 4 in `src/main.rs`**, with **0 ignored**.

Verification names public items, because the public interface is the subject of `REQ-MOK-016` and enumerating it is
the obligation. It does not name private types, private functions, or file-internal structure, and it asserts
nothing about how the library is organized behind rule 5.

`REQ-MOK-017` is verified by static analysis and reconciliation rather than by execution, because placement is a
property of where a test lives, which no runtime assertion can observe.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-016` | static-analysis | `Cargo.toml` declares exactly one `[lib]` named `mokiterions` at `src/lib.rs` and one `[[bin]]` named `Mokiterions` at `src/main.rs` | Both present, no third target, no workspace, no build script |
| `REQ-MOK-016` | static-analysis | Dependency and dev-dependency tables | Both empty |
| `REQ-MOK-016` | automated-test | A public-tier file compiles and runs against the library target alone | `cargo test` builds every `tests/*.rs` target; none names a private item |
| `REQ-MOK-016` | static-analysis | Public-surface inventory diffed against `SPEC-MOK-002` rule 5 | Exact match: no surplus item, and no listed addition present that no relocated test requires |
| `REQ-MOK-016` | review | Each public item classified as value, value constructor, pure function of a value, or copying accessor | Every item classified; no item unclassifiable |
| `REQ-MOK-016` | review | No public item yields mutable or owned authoritative state | No mutable borrow, owned engine collection, reference into engine state, iterator over engine collections, mutating trait object, or closure holding engine state; `Simulation` has no public field |
| `REQ-MOK-016` | static-analysis | Prohibited items of `SPEC-MOK-002` rule 6 | None public and none reachable from a public item |
| `REQ-MOK-016` | static-analysis | No conditional visibility | No `cfg`, feature, self dev-dependency, or build script affects which items are public |
| `REQ-MOK-016` | static-analysis | `src/main.rs` contents | Declares no module, contains no test, and contains only startup, buffering, one library call, flush handling, and exit-code mapping |
| `REQ-MOK-016` | static-analysis | `src/lib.rs` contents | Declares the two public modules and `execute`; contains no simulation logic and no test |
| `REQ-MOK-016` | automated-test | `execute` signature, behavior, and exit codes | `0` for help and success, `1` for output failure, `2` for invalid configuration, with usage text on standard error |
| `REQ-MOK-016` | automated-test | Lint gate under both target names | `cargo clippy --all-targets --all-features -- -D warnings` clean, with no `non_snake_case` finding |
| `REQ-MOK-017` | static-analysis | Every test is in exactly one tier | No test outside `tests/*.rs` and the `#[cfg(test)] mod tests` blocks of `src/cli.rs` and `src/simulation.rs` |
| `REQ-MOK-017` | static-analysis | Public tier reaches the code only through the public interface | No `tests/*.rs` file names a private type, function, module, or field |
| `REQ-MOK-017` | static-analysis | Public-tier arrangement matches `SPEC-MOK-002` rule 8 | The five named files exist with the stated subjects; no one-test-per-file arrangement |
| `REQ-MOK-017` | review | Every internal-tier test justified by required access | For each, name the non-public item it needs; a test with no such item is misplaced |
| `REQ-MOK-017` | review | Every relocated test justified by sufficiency | For each, the public interface suffices with assertions unchanged |
| `REQ-MOK-017` | static-analysis | Relocated assertions are verbatim | Line-by-line diff of each relocated test body against its pre-change form shows only the access path changed |
| `REQ-MOK-017` | automated-test | One invocation covers both tiers | `cargo test` runs both without a feature, environment variable, ignore attribute, extra command, or working-directory dependence |
| `REQ-MOK-017` | static-analysis | Test census reconciliation | 52 tests before, 52 after, 0 ignored, every name accounted for |
| `REQ-MOK-016`, `REQ-MOK-017` | automated-test | Byte-for-byte equivalence against the recorded baseline | No differing byte across the declared matrix |
| `REQ-MOK-016`, `REQ-MOK-017` | automated-test | `VER-MOK-001` and `VER-MOK-002` coverage preserved | Every case, invariant, and check maps to a named test in its new location |

## Acceptance scenarios

1. A reviewer reads the public-surface inventory alongside `SPEC-MOK-002` rule 5 and finds the two lists identical,
   with each item classified as a value, a value constructor, a pure function of a value, or a copying accessor.
2. A reviewer attempts to name one public item through which a caller could change world, agent, resource, tick,
   entropy, or event-log state, and cannot.
3. The program is run before and after the change with identical arguments at each declared seed, under each
   decision source, at each declared density, with and without `--trace-actions`, and the captured output is
   byte-identical in every combination.
4. The four invalid-input invocations produce the same exit codes and the same standard-error text before and after.
5. `cargo test` is run once with no extra flag or environment variable and reports the same test total as before
   the change, with no ignored test.
6. For each of the tests remaining inline, a reviewer names the non-public item it requires; for each relocated
   test, a reviewer confirms the public interface suffices and the assertions are unchanged.
7. The superseding requirement-to-test mapping accounts for every case in `VER-MOK-001` and `VER-MOK-002` at its new
   location, and `WO-MOK-002`'s retained mapping is unmodified.

## Property and invariant tests

- **Equivalence.** For every combination of declared seed, decision source, declared density, and trace setting,
  pre-change and post-change output are byte-identical and the final state is identical. This is the central
  invariant; a single differing byte fails this contract.
- **Determinism preserved.** Two post-change runs at one seed remain byte-identical, so `REQ-MOK-009` is intact and
  the target split did not move, split, or re-seed the entropy stream.
- **Exit-code totality.** Every path through `execute` still returns `0`, `1`, or `2`, and each of the three is
  exercised.
- **No state escape.** Every public function's return type is an owned value, a `Result` or `Option` of owned
  values, or a copy of a reported fact. No return type mentions a private engine type by reference.
- **Reachability closure.** No private type is reachable from outside the crate through a public item's type
  parameters, associated types, trait bounds, or public fields.
- **Tier disjointness and totality.** Every test is in exactly one tier, and the union of the tiers is the whole
  suite. No test is in both and none is in neither.
- **Public-tier purity.** Each public-tier target compiles against the library target alone, with no `#[path]`
  include of a source file, no `include!`, and no other route back into private code.
- **Census conservation.** Test count is conserved at 52, and the ignored count is 0 before and after.
- **Assertion conservation.** For each relocated test, the number and content of its assertions are unchanged.
- **Behavior surface unchanged.** No simulation constant, event field, event order, summary field, exit code,
  diagnostic message, or byte of `USAGE` differs.

## Static and architecture checks

- `cargo fmt --all -- --check` reports no differences.
- `cargo clippy --all-targets --all-features -- -D warnings` reports no findings, and in particular no
  `non_snake_case` finding against either target name. Neither the invocation nor an `allow` attribute is used to
  reach this result.
- `cargo build` and `cargo test` succeed. `Cargo.toml` gains only the two target sections; dependencies stay empty.
- The `ARCH-MOK-001` conformance checks are re-run as amended: one Cargo package with exactly one library target
  and one binary target; the dependency graph contains no network, async-runtime, database, or UI library; the
  public interface matches `SPEC-MOK-002` rule 5; no public item leaks mutable state.
- `ADR-MOK-001`'s validation checks are re-run unchanged, in particular that no decision source receives a mutable
  world, agent collection, resource collection, event log, or engine handle, and that deterministic replay holds.
- The `ARCH-MOK-001`, `ADR-MOK-001`, and `SPEC-MOK-001` amendments named in `ADR-MOK-002` are present and approved
  before the change is verified. Their absence fails this contract regardless of code state.
- `REPOSITORY_CONTEXT.md`'s *Architecture* section names the library and binary targets rather than a single entry
  point.

## Security and privacy checks

- No network access, credential read, filesystem access, environment read, or wall-clock read is introduced. The
  dependency table remains empty.
- Every public item is confirmed to return a copy of a value the program already prints, so the surface grants no
  capability that did not already exist inside the crate.
- No test-support seam, feature flag, conditional visibility, or self dev-dependency exists in any build
  configuration, including test builds.
- Invalid input remains data and is never interpreted as code or a path; the invalid-input diagnostics are
  byte-identical to the baseline.
- The observation-to-proposed-action trust boundary is unmoved and unreachable from the public interface.

## Performance and resilience checks

- A 10,000-tick run under each decision source completes without panic, and survivors plus deaths equal twelve, as
  before.
- The 1,000-tick population floor at the declared density is met on every declared seed, with survivor counts
  identical to the recorded baseline rather than merely above the floor.
- Compile-time growth from the additional targets is recorded as evidence. No pass condition attaches to it; it is
  retained so that a later objection to the arrangement can be assessed against a measurement.
- Runtime work per tick, memory use, and output volume are unchanged, which follows from byte-for-byte equivalence
  and needs no separate measurement.

## Manual assessments

- Read `src/main.rs` and confirm it is a shim: startup, buffering, one call, flush handling, exit-code mapping, and
  nothing else.
- Read the public-surface inventory and judge whether a reader who had not seen the code could tell from it what
  the program promises. An inventory that requires reading the implementation to interpret is an adverse
  observation.
- For each of the five public-tier files, judge whether its subject is one an operator would recognise. A file that
  is a grab bag is an adverse observation even when every test in it is correctly placed.
- Confirm that no relocated test reads as weaker than its predecessor. Where a test's access path changed from a
  private field read to an accessor, confirm the value asserted is the same value.
- Confirm that the reason each inline test stayed inline is the access it requires, and not that relocating it
  looked laborious.
- Assess whether any authorized rule 5 addition was added without a relocated test requiring it. An unused addition
  is an adverse observation, because the enumeration is a ceiling rather than a checklist.

## Evidence retention

Retain under `docs/engineering/simulation/evidence/WO-MOK-003/`:

- the recorded pre-change baseline: captured output for every declared seed, decision source, declared density, and
  trace setting, plus exit codes and standard-error text for every invalid-input case, with the commit it was
  captured at;
- the post-change comparison result for the same matrix, and the comparison method;
- the public-surface inventory and its diff against `SPEC-MOK-002` rule 5, with each item's classification;
- the boundary review confirming no public item yields mutable or owned authoritative state;
- the test census: `cargo test -- --list` before and after, and the name-by-name reconciliation;
- the per-test placement justification, naming for each inline test the non-public item it requires and for each
  relocated test the public items it uses;
- the per-test diff showing that relocated assertions are verbatim;
- a superseding requirement-to-test mapping covering every case in `VER-MOK-001` and `VER-MOK-002` at its new
  location, stating explicitly that `WO-MOK-002`'s retained mapping remains valid for its own commit and is not
  edited;
- formatter, linter, build, and test output;
- the 10,000-tick resilience result and the 1,000-tick per-seed survivor counts, compared against the baseline;
- the compile-time measurement, with no obligation attached;
- confirmation that the `ARCH-MOK-001`, `ADR-MOK-001`, and `SPEC-MOK-001` amendments were approved before the work
  began, with their approval dates;
- a completion summary naming the final target layout, the final public interface, and the final tier populations.

## Residual uncertainty

- **No compile-fail harness exists.** That a test cannot reach authoritative state is verified by inventory and
  review, not by a mechanical proof that a violating test fails to compile. Such a proof would need a
  compile-failure test dependency, which is excluded. Review is therefore load-bearing here, and a reviewer who
  reads only the tests will not detect a leaked item that no test uses. This is why the inventory is independent of
  the suite.
- Byte-for-byte equivalence is demonstrated across the declared matrix, not across the whole input space. It does
  not prove equivalence at an undeclared seed or density, though a target split has no plausible mechanism for
  seed-dependent divergence.
- The public interface is verified to be read-only as written today. Nothing in this contract prevents a later
  change from adding a mutating accessor; that is governed by `SPEC-MOK-002` rule 5 and by `ADR-MOK-002`'s
  migration clause, and depends on those being applied rather than on a check here.
- The placement rule is verified against the fifty-two tests that exist. It is not proven that every future test
  has an unambiguous tier, and a genuinely ambiguous case is an escalation rather than a judgement call.
- This contract does not verify that the public interface is *well designed*, only that it is enumerated, read-only,
  sufficient for the relocated tests, and unchanged in behavior. Whether it is the right contract for Phase 4 and
  Phase 5 will be established when those phases are written against it, and a shortfall discovered then is an
  amendment to `SPEC-MOK-002` rather than a defect in this verification.
- The initiative's benefit — that a break in the operator-visible contract now fails a test — is not directly
  measured. Demonstrating it would require deliberately breaking the contract, which is out of scope. What is
  verified is that a populated tier of tests reaches the program only through that contract, which is the
  precondition for the benefit rather than the benefit itself.
