# WO-MOK-003 evidence: relocated test diff

`SPEC-MOK-002` rule 12 permits exactly two kinds of change to a relocated test: a `use` of the library
target in place of `use super::*`, and a public accessor in place of a private field read. Everything
else — every assertion, every literal, every message — is verbatim. A relocated test whose assertions
could not survive the move is a rule 7 misclassification, so an unexplained diff here is a defect
rather than a detail.

`VER-MOK-003` requires the diff to be recorded per test. The per-test record is
`relocated-test-diff-body.md`; this file states the method and the summary so the body can be read as
evidence rather than as a list.

## Method

Each relocated test body was extracted from the pre-change source at commit
`77010d02319051a20f8e45282f9c813ce4199956` and from its post-change file, dedented to a common
indentation, and compared line by line. Dedenting is necessary and is not a content change: a test
inside `mod tests` is indented one level deeper than the same test at the top level of a file in
`tests/`, so every line of every relocated test differs in leading whitespace by four spaces. Comparing
without dedenting would report 15 of 15 tests as changed and say nothing.

The comparison is on the body only. The `use` lines, the module header, and the doc comments at the
top of each new file are not part of any test body; they are the relocation itself and are reviewable
in the files.

## Summary

| Outcome | Tests |
| --- | --- |
| Character-identical after dedenting | **11** |
| Changed only by field read → accessor call | **4** |
| Changed in any other way | **0** |

The 11 identical bodies are the five in `tests/cli.rs`, the four in `tests/process.rs`, and the two in
`tests/density.rs`. None of them read a private field: the parsing and process tests assert on values
they constructed or on bytes they captured, and the density tests assert on `Density`, whose relevant
accessor is a rule 5 addition made by visibility change alone.

The four changed bodies are the three in `tests/termination.rs` and the one in `tests/viability.rs`.
Every change in all four is the same substitution, applied to `RunSummary`:

| Before | After |
| --- | --- |
| `summary.reason` | `summary.reason()` |
| `summary.ticks` | `summary.ticks()` |
| `summary.survivors` | `summary.survivors()` |
| `summary.deaths` | `summary.deaths()` |

Nine lines across four tests, and no other line in any of them. This is the substitution rule 12
names explicitly. It is also value-preserving in the strict sense: each accessor returns the same
field by copy, and every field involved is `Copy`, so the expression yields the identical value the
field read yielded. The comparisons around them — `== TerminationReason::TickLimit`, `== 1`,
`<= 10_000`, `survivors + deaths == 12`, `survivors >= floor` — are untouched, as are the assertion
macros, the failure messages, and the format arguments inside them.

## What is not in the diff

- **No assertion was removed.** The census in `test-census.md` accounts for all 52 tests, and no
  relocated test lost a line: the diffs above are substitutions, not deletions.
- **No assertion was weakened.** No `assert_eq!` became an `assert!`, no exact count became a
  "greater than zero", and no direct state check became an assertion about output text. That
  substitution is what `SPEC-MOK-002`'s third counterexample prohibits, and where it would have been
  necessary the test stayed internal instead — the two cases are recorded in `test-placement.md`.
- **No literal changed.** Seeds, tick limits, densities, expected resource counts, survivor floors,
  and error substrings are the values the pre-change tests used.
- **No helper's behavior changed.** Three helpers moved with the tests that use them: `config_with`
  into `tests/cli.rs`, `config` into `tests/termination.rs`, and `config_at` into `tests/viability.rs`,
  along with `FailingWriter` into `tests/process.rs` and the `DECLARED_SEEDS` and `DECLARED_FLOORS`
  constants into `tests/viability.rs`. Each is copied, not rewritten. `SPEC-MOK-002` leaves helper
  location unspecified, so placing each beside its only user is a choice inside the envelope; the
  values they produce are unchanged, which is what rule 12 constrains.
- **No `#[ignore]`, no feature gate, no conditional compilation** was added to any relocated test.
  Rule 10 requires one `cargo test` invocation to run both tiers, and `test-run.txt` shows 52 passed
  and 0 ignored.
