# WO-MOK-006 evidence packet

`VER-MOK-006`'s evidence-retention list has 19 bullets. Each is below with the file that discharges
it, so retention completeness is checkable without reading the packet.

| Retention bullet | File |
|---|---|
| The recorded pre-change baseline: engine output for every declared seed, decision source, density and trace setting, exit codes and standard-error text for every invalid-input case, observer frames and exports for every declared seed and viewport, with the commit each was captured at | `baseline/` — `capture-engine.sh`, `capture-observer.sh`, `frame-and-export-oracle.rs`, `engine/`, `observer/`; all captured at **191db01** |
| The post-change comparison result for both matrices, and the comparison method | `comparison/engine-matrix.txt`, `comparison/observer-matrix.txt` |
| The non-test content comparison for each of the observer's seven `pub mod` files, with `verification.rs`'s exclusion stated and its retained eight tests compared separately | `non-test-content.txt` (seven digests, and why `verification.rs` is excluded); `verbatim-comparison.txt` (its eight retained tests) |
| The public-item census of the observer's library target, with per-module counts, the counting rule stated explicitly, and its comparison against the recorded 97 | `public-item-census.txt` |
| The confirmation that all four `#[cfg(test)]` hooks retain their attribute and that no public-tier file names one | `hooks-and-visibility.txt` |
| The test census: `cargo test -- --list` before and after, per package and per test binary, with the name-by-name reconciliation | `test-census.txt` |
| The per-test placement justification for each of the 32 inline tests and each of the 77 relocated tests | `test-placement.md` |
| The per-test comparison showing that relocated assertions are verbatim | `verbatim-comparison.txt` |
| The per-file comparison showing that relocated content is byte-identical, with every required path reference enumerated and justified | `file-comparison.txt` |
| The executed output of every command form in `SPEC-MOK-004` rule 14, including the `USAGE` first line and the two `cargo tree` results | `command-forms.txt` (the forms and the `USAGE` line); `dependencies.txt` (both `cargo tree` results) |
| The root manifest and both package manifests as they stand at the candidate commit | `manifests.txt` |
| The `Cargo.lock` comparison | `dependencies.txt` |
| Formatter, linter, build and test output, workspace-wide and per package | `static-checks.txt`, `test-run.txt` |
| The re-run `ARCH-MOK-001`, `ARCH-MOK-002`, `ADR-MOK-001` and `ADR-MOK-003` conformance results | `conformance.md` |
| The 10,000-tick resilience result, the 1,000-tick per-seed survivor counts, and the observer's frame-interval measurement, each compared against the baseline | `resilience.txt` (both long runs and the frame budget); `comparison/engine-matrix.txt` (the per-seed survivor counts) |
| The compile-time and `target/` measurements, with no obligation attached | `compile-and-target.txt` |
| A superseding requirement-to-test mapping covering every case in `VER-MOK-001`, `VER-MOK-002`, `VER-MOK-004` and `VER-MOK-005` at its new location, stating that the retained mappings remain valid and are not edited | `requirement-to-test-mapping.md` |
| Confirmation that the `SPEC-MOK-002`, `SPEC-MOK-003`, `ARCH-MOK-002` and `REPOSITORY_CONTEXT.md` amendments were approved before the work began, with their approval dates | `amendment-approvals.md` |
| A completion summary naming the final directory layout, the observer's final target layout, the final public-item count, and the final tier populations of both packages | `completion-summary.md` |

**Two files are not on the retention list.** `manual-assessment.md` discharges `VER-MOK-006`'s
*Manual assessments* section, which is a separate obligation of eight judgements no script can make.
This `README.md` is the map itself.

## Read these four first

- **`completion-summary.md`** — the thirteen things `WO-MOK-006`'s report format asks for, and ten
  disclosures. The ones that need a decision rather than a read: **A**, the reading that approving
  `ADR-MOK-004` approved its four enumerated amendments, which two `verified` records make consequential;
  **D**, a real regression found and fixed here — seven bare `cargo run` forms in `SIMULATION_RULES.md`
  had stopped resolving; and **E**, `VER-MOK-001` step 1, which would also no longer resolve and was
  deliberately left alone.
- **`manual-assessment.md`** — eight judgements, seven with nothing adverse and **one qualified
  adverse**: the observer's public interface cannot be enumerated from any approved artifact. That is a
  property of `SPEC-MOK-004` rule 6 as approved, not a defect in the change, and it is the packet's
  honest cost.
- **`conformance.md`** — `ARCH-MOK-001`'s prohibited patterns and conformance checks walked one at a
  time, because the claim that a move of the whole engine leaves the engine's architecture untouched is
  exactly the claim a reader should not take on trust. `ARCH-MOK-001` has a zero-line diff.
- **`requirement-to-test-mapping.md`** — where every one of the 152 cited tests now lives. All 152
  resolve; three identifiers written in test-name style are not tests and are explained; 17 of the 169
  are not cited by name in any prior mapping and are listed with the binary each runs in.

## The four independent oracles

`VER-MOK-006`'s central claim is that none of this changed a byte of either package's behavior. That
claim is not carried by one measurement:

1. **The recorded baseline** — captured at 191db01 before any file moved, then re-run after.
   `comparison/` holds 40 observer cells (35 frames, 5 exports), the engine's 43 cells and the
   observer's 24 start-up cases; 0 differing lines and 0 differing digests in all of them.
2. **Content comparison against 191db01** — nine engine files byte-identical at blob level; seven
   observer modules byte-identical outside their test blocks; `main.rs`'s 396-line retained region
   identical by SHA-256.
3. **Test-census reconciliation** — 169 before, 169 after, name by name in both directions, with 77 of
   the observer's now compiled outside the crate, where a test that reached a private item could not
   link at all.
4. **Cargo's own resolution** — every command form re-executed, every exit code and summary line
   compared, `Cargo.lock` byte-identical five ways.

Each can fail without the others failing, which is why all four are here.

## What none of this establishes

The observer's public interface is verified as counted, read-only, unchanged and sufficient for the 77
tests that now depend on it. It is **not** verified as well designed, and rule 6 makes it "what was
public before" — so if some of the 97 items are public by accident, that accident is now the contract.

Equivalence is demonstrated across the declared matrices, not across the input space. There is no
compile-fail harness: that a violating test *would* fail to compile is an argument from how Rust links
integration tests, supported by 77 tests that do compile, not a mechanical proof. And the central
benefit — that a break in what the observer presents now fails a test with no private access to repair
itself with — would require deliberately breaking the contract to demonstrate. What is verified is the
precondition for it.

The implementation is uncommitted at the time this packet was written. There is no candidate commit and
`VREC-MOK-006` is not drafted; a verification record binds a commit, and this one has nothing to bind.
