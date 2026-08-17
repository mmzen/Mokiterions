# Evidence for WO-MOK-003

This directory retains implementation and verification evidence for `WO-MOK-003`, *Library target,
enumerated public interface, and test relocation*, captured on 2026-08-17.

The records are observations of the working tree. They do not independently approve verification, create
a candidate commit, or authorize release. `VER-MOK-003` is the verification contract these records serve;
the accountable assurance decision and any verification record remain the owner's act.

## Commit binding

| Fact | Value |
| --- | --- |
| Pre-change baseline captured at | `77010d02319051a20f8e45282f9c813ce4199956` (tip of `master`) |
| Implementation branch | `feature/library-target-and-test-placement` |
| Baseline regenerated after capture | **No.** Captured once, before the first edit, and never re-run against a modified tree. |
| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable |

The baseline is the only independent oracle for the equivalence claim, because the 52 tests were
relocated by the same change they would otherwise be judging. Where a pristine build was needed later —
for the compile-time measurement — it was obtained by extracting the baseline commit with `git archive`
into a scratch directory, never by reverting the working tree.

## Contents

### The equivalence oracle

- `baseline/capture.sh` — the capture script, run unmodified for both sides. 43 matrix cells, 16
  diagnostic cases. Reads no environment and emits no timestamp, which is what makes its output
  comparable.
- `baseline/manifest.txt` — SHA-256, line count, and exit code per cell, captured pre-change.
- `baseline/summaries.txt` — the full final summary line of each of the 40 matrix cells.
- `baseline/exit-codes.txt` — argv, exit code, stdout digest and length, and full standard-error text
  for `--help` and 15 invalid or boundary invocations.
- `baseline/viability.txt` — the five reference-policy summary lines at the declared density.
- `baseline/full/` — two 20-tick traced runs retained in full text, one per decision source.
- `after/` — the same five records captured from the finished tree. All five are byte-identical to
  `baseline/`.
- `baseline-comparison.md` — the comparison method, why hashing was chosen, what the matrix covers and
  what it does not, and the result.

### Test placement and preservation

- `test-census.md` — `cargo test -- --list` before and after, reconciled name by name: 52 / 52, 0 lost,
  0 gained, 15 relocated, 37 in place, with the per-file distribution.
- `baseline/test-census.txt`, `after/test-census.txt` — the two raw listings behind it.
- `test-placement.md` — the per-test placement justification. For each of the 37 internal tests, the
  non-public item it requires, derived mechanically. For each of the 15 relocated tests, the rule 5 items
  it uses.
- `relocated-test-diff.md` — the method and the summary: 11 of 15 bodies character-identical, 4 changed
  only by field read → accessor call, 0 changed otherwise.
- `relocated-test-diff-body.md` — the per-test diff itself.

### The public interface

- `public-surface-inventory.md` — the 13 public items, each classified, derived independently from
  rustdoc and from the sources; the diff against `SPEC-MOK-002` rule 5; the two authorized accessors
  deliberately omitted; and the trait impls reachable through the interface.
- `boundary-review.md` — the confirmation that no public item yields mutable or owned authoritative
  state, argued from signatures and from the absence of any conditional-visibility seam.

### Verification output

- `static-checks.txt` — toolchain versions and the `fmt`, `clippy`, and `build` transcripts, all exit 0.
- `test-run.txt` — the full `cargo test` transcript: 52 passed, 0 failed, 0 ignored, across seven test
  executables plus a doc-test pass reporting zero tests.
- `requirement-to-test-mapping.md` — the **superseding** mapping. Every case in `VER-MOK-001` and
  `VER-MOK-002` at its new location, with the explicit statement that `WO-MOK-002`'s retained mapping
  remains valid for its own commit `68163ac` and was not edited.
- `resilience-and-viability.md` — the four 10,000-tick runs and the per-seed survivor counts, compared
  against the baseline.
- `resilience-10k.txt` — the raw transcript of those four runs.

### Governance and closure

- `amendment-approvals.md` — confirmation that the `ARCH-MOK-001`, `ADR-MOK-001`, and `SPEC-MOK-001`
  amendments were approved before the work began, with dates, authority, and the ordering of the steps.
- `compile-time.md` — the measured compile-time and disk cost, with no obligation attached.
- `completion-summary.md` — the final target layout, the final public interface, the final tier
  populations, the adverse observations, and the residual limitations.

## What is not here, and why

- **No copy of `after/full/`.** The two post-change traced runs were compared file to file and are
  identical to `baseline/full/`. Their digests are recorded in `after/manifest.txt`, which is the durable
  record; a second copy of the same bytes would be noise.
- **No edit to `WO-MOK-001`'s or `WO-MOK-002`'s evidence.** Both are bound to their own commits by
  `VREC-MOK-001` and `VREC-MOK-002`. Neither was opened for writing. The superseding mapping in this
  directory is how relocation is reconciled with them.
- **No new test.** `VER-MOK-003` conserves the census at 52. The two rule 8 subjects that relocation left
  in the internal tier are recorded in `test-placement.md` and `completion-summary.md` rather than covered
  by a new public-tier test, because adding one would have violated the conservation requirement.
