# Evidence — WO-MOK-007

`SPEC-MOK-003` rule 4 clause 7: the roster's three survival bars carry a colour band read from the value each
presents. Green `80..=100`, orange `40..=79`, red `0..=39`.

This index maps every file here to `VER-MOK-007`'s twelve evidence-retention bullets. Two files carry no bullet and
say so.

The gate captures were taken from the working tree with `HEAD` at `0b0fe5d` — the commit recording the owner's
approval and the two approved `SPEC-MOK-003` provisions — and the implementation uncommitted. They therefore evidence
the change, not the commit. `VER-MOK-007` requires commit-bound verification; that is `VREC-MOK-007`'s job and it is
**not authored here**. See `completion-summary.md` disclosure 8.3.

## The twelve bullets

| # | Bullet | File | Contents |
|---|---|---|---|
| 1 | workspace test output, before and after | `test-run-before.txt`, `test-run.txt` | `master` @ `54c21ab` in a throwaway worktree: **172 passed, 0 failed**. After: **179 passed, 0 failed** across 19 targets. Both are the full `cargo test --workspace` output, not a summary line. |
| 2 | band domain sweep and its case count | `band-sweeps.txt` | The domain assertion quoted from source, with the band table the test states independently of the implementation's constants. **101 values** in `0..=100`, plus **100 monotone adjacent pairs**. |
| 3 | the two boundary cases, each by literal value | `band-sweeps.txt` | `band(39)`, `band(40)`, `band(79)`, `band(80)` each asserted by its own literal, and `band(0)` and `band(100)` likewise — **6 literal assertions**. |
| 4 | text-identity sweep and its case count | `band-sweeps.txt` | The sweep quoted from source: 21 bar widths × 101 values = **2121 cases**, asserted in the test as `21 * 101`, against the unbanded form re-derived inside the test rather than captured from the implementation. |
| 5 | the reference-viewport roster as text, spanning three bands | `frames.txt` | The roster pane at 160×48, unselected and with `M05` selected, each bar row followed by a per-cell band map (`......GGGGGGGGGGGG..OOOOOOOOOOOO..RRRRRRRRRRRR`). `M05` at h100 s59 e39 spans all three bands. Includes the throwaway harness's source, so the capture is reproducible. |
| 6 | `cargo fmt --check` and `cargo clippy` output | `static-checks.txt` | `cargo fmt --all -- --check` exit 0, no output. `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0, no finding. |
| 7 | the empty diff of `mokiterions-core/` against `master` | `core-and-manifests-untouched.txt` | `git diff --stat master -- mokiterions-core/` empty; `git diff --stat master -- '*Cargo.toml' Cargo.lock` empty; whole-tree diff against `master` lists **5 files**, all accounted for. |
| 8 | `grep` results for retained-tick and delta patterns | `no-retained-tick.txt` | One `band(` call site in shipped code (`render.rs:540`, inside `gauge`); `gauge` and `band` quoted whole; grep for `previous\|prev\|delta\|trend\|last_tick\|_ago\|history` across the observer, every hit a comment or a key label. Discloses the pre-existing `latest_survival` at `state.rs:161,193,224,228` and shows it unreachable from the roster path. |
| 9 | `harnessctl validate`, `preflight`, `inspect`, `dashboard` | `harness-gates.txt`, `harness-inspect-and-dashboard.txt` | `validate` PASS, 70 artifacts, 0 errors, 0 warnings. `preflight --phase review --work-order WO-MOK-007` **PASS**. `inspect` PASS, 215 relations. `dashboard` PASS, 0 errors, 9 pre-existing `W-HEX-001`/`W-HEX-003` warnings. `doctor` PASS on every managed file, included because it is what shows no harness-managed file was edited. |
| 10 | verbatim before and after of each changed test assertion | `changed-assertions.md` | Both permitted tests, before and after in full. Every asserted string byte-identical; one band assertion **added** to each. Records that no third pre-existing test changed, which is `WO-MOK-007` constraint 6's stop condition. |
| 11 | `manual-assessment.md` with an author or an outstanding marker | `manual-assessment.md` | All **three assessments OUTSTANDING, no author**, with the environment stated (no terminal, no display) rather than left implicit. |
| 12 | `completion-summary.md`, numbering every disclosure | `completion-summary.md` | The work order's eight required disclosures, numbered, with clause 7 read back out of the source. Item 8 numbers eight things not done, including the outstanding amendment and the untouched lifecycle status. |

## Files carrying no bullet

| File | Why it is here |
|---|---|
| `requirement-to-test-mapping.md` | `VER-MOK-007`'s nine matrix rows, five properties and four acceptance scenarios against the tests that carry them. Scenario 2 is marked "No, and stated" and scenario 3 "Partly", so partial coverage is on the record rather than inferable only by reading the tests. |
| `outstanding-amendment.md` | A **third** amendment provision the owner has not seen: `SPEC-MOK-004` rules 9, 10 and 11 record test and private-item counts that seven new tests and six new private items make wrong. Drafted in full, marked **OUTSTANDING**, and **not applied** — `WO-MOK-007`'s change surface ends "no other artifact". |

## What a reviewer should check first

1. `changed-assertions.md` — whether any rendered character moved. It is the constraint the whole clause turns on, and the two tests that could have caught a move are quoted whole.
2. `frames.txt` — whether the bands are where they should be, on a real frame, including under selection.
3. `completion-summary.md` item 8 — what is outstanding. Two items need the owner: the `SPEC-MOK-004` amendment and the three manual assessments.
