# Evidence for WO-MOK-004

This directory retains implementation and verification evidence for `WO-MOK-004`, *State every option's
effect and default in the help output*, captured on 2026-08-17.

The records are observations of the working tree. They do not independently approve verification, create
a candidate commit, or authorize release. `VER-MOK-004` is the verification contract these records serve;
the accountable assurance decision and any verification record remain the owner's act.

## Commit binding

| Fact | Value |
| --- | --- |
| Pre-change baseline captured at | `08dc8d47aa569c77fb7c9b091e328eb8f40c5285` (tip of `master`) |
| Source confirmed untouched at capture | `git status --short -- src tests Cargo.toml Cargo.lock` reported 0 lines |
| Baseline regenerated after capture | **No.** Captured once, before the first edit, and never re-run against a modified tree. |
| Implementation branch | `feature/help-output-options-block` |
| Candidate commit | named by `VREC-MOK-004`; the records here were captured from the working tree that became it |
| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable |

`VER-MOK-004` faces the mirror image of `VER-MOK-003`'s hazard: this change is required to alter exactly
one observable thing and nothing else, and the suite already contained the test a careless reviewer would
reach for. `tests/process.rs::help_exits_successfully` asserts the help output *equals* `cli::USAGE`,
which is true for every possible value of the constant including an empty string. Every check in this
directory is therefore written against a source outside the constant, and `drift-demonstrations.md`
records the central one failing in both directions rather than asserting that it would.

## A note on line endings in these files

The repository is worked on under Windows with `core.autocrlf = true`, so every text file here is stored
in the object database with `LF` and appears in a Windows working tree with `CRLF`. The digests and
lengths recorded in `baseline/manifest.txt`, `after/manifest.txt`, and `exit-codes.txt` were computed
from the program's own bytes at capture time and are the authoritative record. The retained `usage.txt`
files are readable copies of that output, not byte-exact artifacts of a later checkout: re-hashing one
after checkout will not reproduce the recorded digest, and that is the checkout's line-ending
conversion, not drift in the program. `WO-MOK-003`'s evidence is stored the same way.

The measurement that does matter is in `usage-text.md`: the bytes the program emits contain **zero**
carriage returns, on this machine and on any other, because `cli::USAGE` is built from one string
literal per line with an explicit `\n` rather than from a multi-line literal that would inherit its line
endings from the checkout.

## Contents

### The equivalence oracle

- `capture.sh` — the capture script, byte-identical to the one used under `WO-MOK-003` (verified with
  `diff`) and run unmodified for both sides. 43 matrix cells, 16 named cases. Reads no environment and
  emits no timestamp, which is what makes its output comparable.
- `baseline/manifest.txt`, `baseline/summaries.txt`, `baseline/exit-codes.txt`, `baseline/full/` — the
  pre-change capture.
- `baseline/usage.txt`, `baseline/test-census.txt` — the pre-change help output and test listing.
- `after/` — the same records from the finished tree, plus `after/test-run.txt`.
- `baseline-comparison.md` — the comparison against the change surface `VER-MOK-004` stated in advance,
  class by class, with the method used to classify each changed line mechanically.
- `exit-codes.diff.txt` — the raw 342-line diff of the 16 named cases.

### The change to observable output

- `usage-text.md` — both texts in full, the line-by-line diff, the measurements, and the confirmation
  that the emitted text is byte-identical to the rendering `WO-MOK-004` authorized before the work began.
- `usage-text.diff.txt` — the raw diff.
- `specification-to-help.md` — the first oracle: `SPEC-MOK-001`'s *Help output* table transcribed and
  diffed against the rendered help. No shortfall, no surplus, no defect; documented defaults 1 of 5 → 5
  of 5.

### The tests

- `new-tests.md` — the eight added tests, the oracle each uses outside the constant, why the
  `tests/process.rs` addition was needed, why the central test carries no expected values, and the
  strong form of the coverage-totality property.
- `test-census.md` — `cargo test -- --list` before and after, reconciled name by name: 52 → 60, eight
  additions, zero removals, zero renames, zero ignored.
- `baseline/test-census.txt`, `after/test-census.txt` — the two raw listings.
- `drift-demonstrations.md` and `drift-scratch-{a,b,c}.txt` — the check demonstrated failing: applied
  default moved, printed default moved, and a parser option added without a help entry. None committed.

### The interface

- `public-surface-inventory.md` — 13 public items, regenerated from rustdoc and diffed against the
  inventory retained under `WO-MOK-003`. No addition, no removal, no change of form; `cli::USAGE` is
  still a constant.

### Verification output

- `static-checks.txt` — the `fmt`, `clippy`, and `build` transcripts, all exit 0, with a forced recheck
  after `cargo clean -p Mokiterions`, and `git status` for the source tree.
- `after/test-run.txt` — the full `cargo test` transcript: 60 passed, 0 failed, 0 ignored.
- `resilience-and-viability.md` and `resilience-10k.txt` — the per-seed survivor counts against the
  baseline and the four 10,000-tick runs against `WO-MOK-003`'s record.

### Governance and closure

- `amendment-approval.md` — that the `SPEC-MOK-001` amendment was approved **before** the usage text
  changed, with the `W016` preflight finding against the drafts as independent evidence of the ordering.
- `completion-summary.md` — the completion report in the eleven-item form `WO-MOK-004` requires.

## What is not here, and why

- **No copy of `after/full/`.** The two post-change traced runs were compared file to file and are
  identical to `baseline/full/`. Their digests are in `after/manifest.txt`, which is the durable record.
- **No `SPEC-MOK-002` amendment.** Its rule 11 and `VER-MOK-003`'s *Behavior surface unchanged*
  invariant, which together forbid any byte of `USAGE` changing, are scoped to `WO-MOK-003`'s
  restructuring — a discharged contract that binds that work order and not this one. The reasoning is in
  `amendment-approval.md` so that the absence is deliberate rather than an omission.
- **No architecture or ADR evidence.** No architecture addresses `REQ-MOK-018`. `WO-MOK-004` records the
  omission with the counter-argument against it, and `VER-MOK-004`'s static checks record it as a check
  performed rather than a step skipped.
- **No edit to earlier evidence or verification records.** `WO-MOK-001`/`002`/`003` evidence and
  `VREC-MOK-001`/`002`/`003` are bound to their own commits and were not opened for writing.
- **No line-width test.** Width and wrapping are what `SPEC-MOK-001` explicitly leaves unspecified, so
  freezing them in a test would enforce a decision the specification declines to make. The widths are
  measured in `usage-text.md` instead.
- **No release act.** The work is committed to a branch and opened as a pull request. Nothing was
  tagged, published, or released, and the accountable verification decision is recorded separately in
  `VREC-MOK-004` rather than implied by the commit existing.
