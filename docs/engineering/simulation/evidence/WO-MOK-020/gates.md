# WO-MOK-020 gates

`WO-MOK-020`'s *Required verification* lists nine items. All nine were run. Items 1 to 6 are reported below,
including the one that **FAILs by design**; items 7 to 9 are `VER-MOK-004`'s own rows and are reported in
`nonperturbation/`, `defaults-divergences.txt` and `resilience-and-interface.txt`, with their results
summarized here. These are observations of the working tree; they approve nothing and authorize no release.

Captured 2026-08-22 on branch `feature/help-output-clarity`, base
`f7b1c452039dc2f03010ca8b8cc81e73c54727c0`.

**Two capture points, and which sections belong to which.** Sections 1 to 5 and the `doctor` and `validate`
readings were first taken at commit `6d6be40`, when the work order was `draft` and its five amendments were
**OUTSTANDING**. The owner then ratified all five and directed the transition to `implemented`, which
changed markdown only — no `.rs` file, no `Cargo.toml` and no lockfile. Section 6 was re-run in full after
those edits and is reported at the ratified state; `doctor` and `validate` returned the same figures as
before, which is expected, since neither reads amendment prose. The code gates were not re-run after the
doc-only edits and are reported as taken at `6d6be40`. Sections 7 to 9 came later still, after commit
`6d5b532` recorded the ratification, and their base side is a fresh worktree at `f7b1c45` rather than
anything reused. Section 6 was then run a **third** time, on the finished tree, and returned the same
figures a third time — 81 of 81 `doctor` checks PASS with 31 managed files `unchanged`, `validate` 148
artifacts with 0 errors and 0 warnings, `preflight --phase review` PASS at exit 0 — which is what should
happen, since items 7 to 9 added evidence files and amended no artifact the harness reads for structure.
What binds a tree rather than a moment is `VREC-MOK-020`, prepared against the commit that carries all of
it.

## Toolchain

```
rustc 1.97.1 (8bab26f4f 2026-07-14)
cargo 1.97.1 (c980f4866 2026-06-30)
clippy 0.1.97 (8bab26f4f6 2026-07-14)
rustfmt 1.9.0-stable (8bab26f4f6 2026-07-14)
Python 3.14.6
se-harness 0.4.0
```

The harness version is the pin, not the machine-wide install. The 0.4.1 on `PATH` fails this repository on
template skew alone, which is a fact about the installed harness rather than about the repository, so every
harness command below was run from the 0.4.0 virtual environment at `C:\Users\mathi\.harness040`.

## Summary

| # | Gate | Command | Result |
| ---: | --- | --- | --- |
| 1 | Formatting | `cargo fmt --all -- --check` | **PASS** — no difference |
| 2 | Lint | `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | **PASS** — exit 0, no warning |
| 3 | Tests | `cargo test --locked` | **PASS** — 303 passed, 0 failed, 0 ignored |
| 4 | Drift test bites | scratch divergence, then `cargo test -p mokiterions-tui --test options` | **FAILED as intended**, naming `--density`; scratch reverted |
| 5 | Rendered texts | `cargo run -q -p <pkg> -- --help` on both packages, before and after | **PASS** — both exit 0; retained in `before/` and `after/` |
| 6 | Harness | `harnessctl doctor`, `validate_engineering_artifacts.py`, `harnessctl preflight` | doctor **PASS**, validate **PASS**, preflight `review` **PASS** and `start` **FAIL by design** |
| 7 | Non-perturbation | `nonperturbation/capture.sh`, unmodified, on a base worktree and on the candidate | **PASS** — 43 of 43 cells byte-identical; 1 of 16 named cases changed, and it is `--help` |
| 8 | Printed defaults are the applied defaults | three scratch divergences against the **rewritten** text | **PASS** — each fails, and the load-bearing one fails exactly 1 test of 18; all three reverted |
| 9 | Resilience and public surface | 10,000-tick runs, 1,000-tick survivor floor, `cargo doc` and a `pub` inventory | **PASS on the checks, with two of the contract's figures found stale** — see amendment 6 |

Items 7 to 9 were added to *Required verification* on 2026-08-22, while executing `VER-MOK-004` to prepare
`VREC-MOK-020`. They are not gates this work order set itself; they are the contract's rows, and the work
order's original case for them — that no engine source is in the diff — is a claim about a diff where the
rows are claims about every run.

## 1. Formatting

```
$ cargo fmt --all -- --check
$ echo $?
0
```

No output and exit 0. `cli.rs`'s and `options.rs`'s `concat!` arms are one string literal per line, which
`rustfmt` leaves alone, so the rewritten constants are formatted as committed rather than reflowed.

## 2. Lint

The repository's own gate command, from `.engineering-harness.toml`'s `lint_or_format`, including
`--locked`:

```
$ cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
    Checking Mokiterions v0.1.0 (mokiterions-core)
    Checking mokiterions-tui v0.1.0 (mokiterions-tui)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.74s
$ echo $?
0
```

`--locked` is not optional here: the release workflow passes it, so a lint run without it can resolve
around `Cargo.lock` and pass locally against dependencies the release would refuse. It resolved without
touching the lockfile, which is also the evidence that no dependency was added.

## 3. Tests

`cargo test --locked`, all twenty-two targets across both packages:

| Package | Target | Passed |
| --- | --- | ---: |
| `Mokiterions` | `unittests src/lib.rs` | 97 |
| `Mokiterions` | `unittests src/main.rs` | 0 |
| `Mokiterions` | `tests/cli.rs` | **18** |
| `Mokiterions` | `tests/decisions.rs` | 3 |
| `Mokiterions` | `tests/density.rs` | 2 |
| `Mokiterions` | `tests/naming.rs` | 3 |
| `Mokiterions` | `tests/process.rs` | 7 |
| `Mokiterions` | `tests/records.rs` | 17 |
| `Mokiterions` | `tests/termination.rs` | 5 |
| `Mokiterions` | `tests/viability.rs` | 5 |
| `mokiterions-tui` | `unittests src/lib.rs` | 34 |
| `mokiterions-tui` | `unittests src/main.rs` | 8 |
| `mokiterions-tui` | `tests/authority.rs` | 4 |
| `mokiterions-tui` | `tests/export.rs` | 7 |
| `mokiterions-tui` | `tests/layout.rs` | 11 |
| `mokiterions-tui` | `tests/options.rs` | **9** |
| `mokiterions-tui` | `tests/render.rs` | 22 |
| `mokiterions-tui` | `tests/spatial.rs` | 7 |
| `mokiterions-tui` | `tests/state.rs` | 22 |
| `mokiterions-tui` | `tests/verification.rs` | 22 |
| both | doc-tests | 0 |
| | **Total** | **303** |

**0 failed, 0 ignored, 0 filtered out**, on every target.

### The two changed test targets, against the base commit

Measured in a `git worktree` at `f7b1c45` rather than inferred from the diff:

| Target | At base | Now | Reading |
| --- | ---: | ---: | --- |
| `mokiterions-tui/tests/options.rs` | 8 | 9 | the one added test, `the_shared_entries_are_the_engines_own_words` |
| `mokiterions-core/tests/cli.rs` | 18 | 18 | **no test added, removed, renamed or skipped.** All eighteen pass against the rewritten text with every assertion exactly as `VREC-MOK-004` bound it. |

The 18-to-18 row is the one that matters. `tests/cli.rs` is the file a restructure of this text is most
likely to break, and the available shortcut was to relax an assertion; `WO-MOK-010`'s precedent forbids
that, and `WO-MOK-020`'s first stop-and-escalate condition would have applied. **It was not needed.** One
helper changed and no assertion did: `options_block()` now ends the block at the first line carrying text
in column one rather than at the first blank line, because entries are separated by blank lines and a
blank line no longer closes the block. That is the terminator, not the oracle.

## 4. The drift test bites

The four shared entries are byte-identical copies held equal by a test rather than by a shared literal,
for the reason `usage-text.md` records. A test asserting that two copies match is worthless if it cannot
fail, so it was made to fail rather than argued to be capable of it.

**Scratch change**, in the observer's copy only, one character:

```
-      How much food the world holds, as a percentage of one territory's 8192
+      How much food the world holds, as a percentage of one territory's 8193
```

Result — the full retained output is `drift-demonstration.txt`:

```
running 1 test
the_shared_entries_are_the_engines_own_words --- FAILED

---- the_shared_entries_are_the_engines_own_words stdout ----
thread 'the_shared_entries_are_the_engines_own_words' panicked at mokiterions-tui\tests\options.rs:179:9:
the observer describes --density in words the engine's own help does not use:
  --density <percent>
      How much food the world holds, as a percentage of one territory's 8193
      cells, written with at most two decimal places. Default: 0.75. The one
      ...

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s
```

It fails, it names the option, and it prints the divergent entry so the reader does not have to go find
it. **The scratch change was reverted and is not in the commit**, verified two ways rather than assumed:

```
$ cargo test -q -p mokiterions-tui --test options
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo run -q -p mokiterions-tui -- --help | diff - after/observer-usage.txt
$ echo $?
0
```

The second is the stronger of the two: the program's live output is byte-identical to the text retained as
evidence, so the retained text is what the reverted tree prints.

## 5. Rendered texts and the width check

Both programs, both sides, `--help`, all four exiting `0`. The before side was rendered from a
`git worktree` at the base commit, not reconstructed from a diff.

| Text | Widest line | Lines past column 80 |
| --- | ---: | ---: |
| engine before | 81 | 2 |
| engine after | **80** | **0** |
| observer before | 84 | 6 |
| observer after | **79** | **0** |

Both texts now fit eighty columns and neither did before. `REQ-MOK-018`'s amended *Verbosity stays
bounded* clause states that bound, and the observer's text was five lines over it on the first rendering;
those five were rewrapped before this capture. The rewrap moved no word and changed no claim. Full texts,
diffs and the rest of the measurements are in `usage-text.md`.

### Line endings

Every file this work order touches is measured, all thirty-five of them, not a sample. **Every text file is
uniformly CRLF with zero bare LF and zero bare CR; the two shell scripts are uniformly LF by intent.**

| File | CRLF | bare LF | bare CR |
| --- | ---: | ---: | ---: |
| `mokiterions-core/src/cli.rs` | 231 | 0 | 0 |
| `mokiterions-core/tests/cli.rs` | 577 | 0 | 0 |
| `mokiterions-tui/src/options.rs` | 242 | 0 | 0 |
| `mokiterions-tui/tests/options.rs` | 184 | 0 | 0 |
| `work-orders/WO-MOK-020.md` | 493 | 0 | 0 |
| `requirements/REQ-MOK-018.md` | 157 | 0 | 0 |
| `specifications/SPEC-MOK-001.md` | 836 | 0 | 0 |
| `specifications/SPEC-MOK-003.md` | 1265 | 0 | 0 |
| `verification/VER-MOK-004.md` | 280 | 0 | 0 |
| `evidence/WO-MOK-020/README.md` | 92 | 0 | 0 |
| `evidence/WO-MOK-020/gates.md` | 453 | 0 | 0 |
| `evidence/WO-MOK-020/usage-text.md` | 434 | 0 | 0 |
| `evidence/WO-MOK-020/before/engine-usage.txt` | 38 | 0 | 0 |
| `evidence/WO-MOK-020/before/observer-usage.txt` | 22 | 0 | 0 |
| `evidence/WO-MOK-020/after/engine-usage.txt` | 71 | 0 | 0 |
| `evidence/WO-MOK-020/after/observer-usage.txt` | 84 | 0 | 0 |
| `evidence/WO-MOK-020/engine-usage.diff.txt` | 105 | 0 | 0 |
| `evidence/WO-MOK-020/observer-usage.diff.txt` | 98 | 0 | 0 |
| `evidence/WO-MOK-020/drift-demonstration.txt` | 26 | 0 | 0 |
| `evidence/WO-MOK-020/preflight-implemented.txt` | 85 | 0 | 0 |
| `evidence/WO-MOK-020/defaults-divergences.txt` | 60 | 0 | 0 |
| `evidence/WO-MOK-020/resilience-and-interface.txt` | 91 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/README.md` | 69 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/manifest-base.txt` | 43 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/manifest-after.txt` | 43 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/summaries-base.txt` | 40 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/summaries-after.txt` | 40 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/exit-codes-base.txt` | 642 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/exit-codes-after.txt` | 1104 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/field-comparison.txt` | 36 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/both-paths.txt` | 26 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/resilience-10k-base.txt` | 16 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/resilience-10k-after.txt` | 16 | 0 | 0 |
| `evidence/WO-MOK-020/nonperturbation/capture.sh` | 0 | **85** | 0 |
| `evidence/WO-MOK-020/nonperturbation/resilience-10k.sh` | 0 | **11** | 0 |

**The two bold cells are not defects.** `capture.sh` is `LF` because it is a byte-for-byte copy of
`WO-MOK-004`'s script and `cmp` reports no difference; converting it would have broken the one property
that makes it worth copying. `resilience-10k.sh` is `LF` to match it. Both are shell scripts, where `LF` is
the portable choice, and `.gitattributes`' `-text` on this directory means neither is converted in either
direction on checkout. A first attempt wrote `capture.sh` with `CRLF`, `cmp` reported `differ: char 20,
line 1`, and the copy was redone with `cp` — the claim that it is unmodified would otherwise have been
false while looking true.

Re-measured 2026-08-22 after items 7 to 9. Before them this table had thirteen rows and the work-order row
read 390; before the ratification edits it read 265. The **bare CR** column was added because the first
attempt at `preflight-implemented.txt` had 69 of them: `harnessctl` already emits `CRLF`, so writing its
output through a `CRLF` translation produced `\r\r\n` on every line the harness had wrapped. The capture was
rewritten by normalizing to `LF` before the single translation, and the column exists so that a reader can
see the check was run rather than assumed.

Two rows are self-referential and are stated as such rather than left to look like independent
measurements. `gates.md`'s own row and `work-orders/WO-MOK-020.md`'s row count files that this section's
last edit changed. Both were measured after that edit and then written in, which is possible only because
writing a number into a table row does not change how many rows the table has.

And the measurement that survives a checkout: **both rendered texts contain zero carriage returns**,
because both constants are one string literal per line with an explicit `\n` rather than a multi-line
literal that would inherit its endings from the working tree. The observer's constant acquires that
property here; it was a single escaped literal before.

## 6. Harness

### `harnessctl doctor` — PASS

```
$ harnessctl doctor; echo $?
0
```

**81 checks, all PASS, no WARN and no FAIL.** All 31 managed files report `unchanged`, including
`docs/engineering/templates/WORK_ORDER.template.md`, which is the template `WO-MOK-020.md` was written
against.

### `validate_engineering_artifacts.py` — PASS

```
$ python scripts/validate_engineering_artifacts.py
Engineering artifact validation: PASS
Artifacts: 148 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
```

148 artifacts, the new work order among them, across all four planes with no error and no warning. This
is what confirms `WO-MOK-020`'s frontmatter and its four `[relations]` edges resolve: `REQ-MOK-018`,
`SPEC-MOK-001`, `SPEC-MOK-003` and `VER-MOK-004` all exist and all accept the relation.

**148 is the count at the commit this page is committed in, which does not contain `VREC-MOK-020`.** The
record is generated against that commit and committed after it, so its own file cannot be inside it. With
the record present the same command reports **149 artifacts, 0 errors and 0 warnings**, which was run and is
the figure a reader checking out the branch tip will see. Both are stated because neither alone matches
every checkout.

### `harnessctl preflight` — three readings

The full output of the two post-ratification runs is retained verbatim in `preflight-implemented.txt`.

| # | When | Phase | Status read | Verdict | Exit | Diagnostic |
| ---: | --- | --- | --- | --- | ---: | --- |
| A | before ratification | `start` | `draft` | **FAIL** | 1 | `W005` status `draft` not eligible for start |
| B | after ratification | `start` | `implemented` | **FAIL** | 1 | `W005` status `implemented` not eligible for start |
| C | after ratification | `review` | `implemented` | **PASS** | 0 | none |

**A.** Recorded rather than worked around:

```
Harness preflight: FAIL
Phase: start
Work order: WO-MOK-020 (draft)

Diagnostics:
- [W005] docs/engineering/simulation/work-orders/WO-MOK-020.md: status 'draft'
  is not eligible for start; expected one of approved, in_progress
```

`W005` was the harness saying a `draft` work order does not authorize implementation, which was exactly
true: the code was written on the owner's direct instruction of 2026-08-22 and the five amendments that
instruction did not cover were **OUTSTANDING**. Moving the status to `approved` would have cleared the
diagnostic and would have been the implementation agent approving its own authorization, which
`DECISION_RIGHTS.md` reserves to the engineering owner. It stayed red until the owner acted.

**B — and this is the correction.** `WO-MOK-020`'s *Lifecycle* first said the diagnostic "clears on this
transition". It does not. The `start` phase admits `approved` and `in_progress` only, so at `implemented`
it raises `W005` again, for the opposite reason:

```
Harness preflight: FAIL
Phase: start
Work order: WO-MOK-020 (implemented)

Diagnostics:
- [W005] docs/engineering/simulation/work-orders/WO-MOK-020.md: status
  'implemented' is not eligible for start; expected one of approved, in_progress
```

Both readings are retained because a reader who sees only one of them learns the wrong thing. The status
never passes `start`: it was short of the window, then past it. There is no moment in this work order's
life at which `--phase start` returns PASS, and the `approved` state that would have produced one existed
only as a step inside the single act the owner took on 2026-08-22.

**C.** The phase that applies at `implemented`:

```
Harness preflight: PASS
Phase: review
Work order: WO-MOK-020 (implemented)
```

Exit 0, and **no `Diagnostics:` block at all** — not a block with zero entries. Nothing about the work
order, its relations, its assurance classification or its reading manifest was flagged, and the preflight
resolved all fourteen documents of the manifest, from `INT-MOK-001` down through the four related
artifacts, all four of which are the amended ones.

**Transcription note.** `harnessctl` writes cp1252 to a pipe on this machine, so the two em dashes it
echoes out of the work order's assurance rationale leave as byte `0x97`. `preflight-implemented.txt`
stores them decoded, as UTF-8. Nothing else in that capture was altered and no line was elided.

## 7. Non-perturbation — `VER-MOK-004`'s matrix, executed on both sides

Full report in `nonperturbation/README.md`; the captures are the files beside it. The script is
`WO-MOK-004`'s `capture.sh`, copied byte for byte, `cmp`-clean, so the cells are the cells the contract
fixed rather than a set chosen after the fact. The base side is a real `git worktree` at
`f7b1c452039dc2f03010ca8b8cc81e73c54727c0`, built there; the candidate side is this branch, built here.

| Comparison | Result |
| --- | --- |
| all 43 matrix cells — SHA-256, line count, exit code | **identical** |
| all 40 matrix summary lines | **identical** |
| the two 20-tick traced runs, `diff -r` on the full text | **identical** |
| the 16 named cases, on the four fields the contract fixes | **one changed of sixteen**, and it is `--help` |
| both emission paths | **byte-identical**, 3601 bytes and 71 lines each, equal to `after/engine-usage.txt` |

The one change is `--help`'s standard output, 2195 → 3600 bytes, which is the subject of this work order.
Each of the fourteen invalid-configuration cases holds exit `2`, an empty standard output and a
byte-identical `configuration error:` line; their standard error grew from 39 to 72 lines because the usage
block appended after that line grew from 38 to 71. The valid single-tick case is byte-identical in every
field, 14386 bytes at exit `0`. Neither stream contains a carriage return on this Windows machine.

## 8. Printed defaults are the applied defaults — re-run against the rewritten text

Full report in `defaults-divergences.txt`. `VER-MOK-004` acceptance scenario 3 names three divergences that
must each fail the suite. `VREC-MOK-004` demonstrated them against the **previous** text, which this work
order replaced, so citing that record would have been citing a demonstration about text that no longer
exists. All three were re-run here and all three were reverted.

| Divergence | Scratch change | Result |
| --- | --- | --- |
| A — applied default moves, text left alone | `cli.rs:215` `ticks.unwrap_or(100)` → `unwrap_or(1000)` | **FAILED**, 12 passed and 6 failed |
| B — printed default moves, parser left alone | the `--seed` entry's `Default: 0.` → `Default: 1.` | **FAILED**, 17 passed and **exactly 1** failed |
| C — parser accepts an option the text does not list | a `"--verbose"` arm before the catch-all | **FAILED**, 17 passed and exactly 1 failed |

**B is the load-bearing direction and the reason to re-run rather than cite.** A one-character edit to prose
inside a `concat!` arm fails exactly one test of eighteen,
`each_documented_default_parses_to_the_applied_default`, and its message names the option and both values.
That is the property `REQ-MOK-018` exists for, demonstrated on the text as rewritten. C fails
`the_documented_options_are_exactly_the_options_the_parser_accepts`, which prints both sorted lists.

Confirmation that nothing survived: `git status --porcelain` showed only the new evidence directory, the
suite returned to 303 passed, and the program's live `--help` was byte-identical to the retained 3601-byte
`after/engine-usage.txt`.

## 9. Resilience and public surface

Full report in `resilience-and-interface.txt`.

| Check | Result |
| --- | --- |
| 10,000 ticks under all four decision sources, base against candidate | **identical**, `diff` exit 0; every row exit 0 and survivors + deaths = 12 |
| 1,000-tick survivor floor, five seeds | `8 9 10 9 9` on **both** sides; floor 8 |
| public interface | **one `pub` line pair differs**, the observer's `USAGE` reformat, across 30 public items; both constants remain `pub const USAGE: &str` |
| `cargo doc` | one warning, **inherited**: `public documentation for 'suffered' links to private item 'SufferedAttack'` at `mokiterions-core/src/simulation.rs:1542`, and `git diff f7b1c45 -- mokiterions-core/src/simulation.rs` is empty |

**Two of the contract's figures no longer describe the tree, and that is a finding about the contract, not a
gate failure.** `VER-MOK-004`'s *Performance and resilience* row fixes survivor counts `8, 11, 8, 9, 11`;
the measurement is `8, 9, 10, 9, 9`, identical on both sides of this change, so the divergence predates it.
The same row's "same termination ticks" clause no longer holds either: `reference` at density `0.75` now
runs to the 10,000-tick limit with 2 survivors, where `WO-MOK-004` recorded extinction at tick 9154. Both
are recorded as amendment 6 rather than quietly reinterpreted, because a contract row that names wrong
numbers cannot be marked satisfied by measuring different ones.

## What is not gated here

- **GitHub issue 40**, the observer's `--events-path` silently writing nothing. Deferred by the owner on
  2026-08-22; disclosed in the observer's text; out of scope, and no gate here covers a fix that was not
  made.
- **The two misplaced amendment rows in `SPEC-MOK-001`.** The 2026-08-21 row sits under the document's H1
  above *Scope*, and the 2026-08-20 record-stream row sits below the table separated by a blank line, so
  neither renders as a table row. `validate` does not read markdown table structure, so its PASS is not
  evidence that they are fine. Recorded as an observation; moving an approved row is the technical owner's
  act.
- **Verification is not accepted here.** Item 6 of *Required verification* is the harness gates, not
  assurance. `commit_bound_verification` is `required`, and `VREC-MOK-020` stands at status `ready`, bound
  to the commit that carries this page. `ready` is not `verified`: accepting a record is the assurance
  owner's separate act, and nothing below this line was decided by the agent that wrote it. Until that
  acceptance, the gates on this page are the strongest claim available — that the tree was measured, not
  that the work is verified.
- **Three of `VER-MOK-004`'s checks are not satisfied as written, and no gate here can satisfy them.** Its
  *Line width* row asserts a synopsis first line of 81 columns before and after and calls the synopsis
  unchanged: the before synopsis first line is 49 columns, the widest synopsis line is 68, and the synopsis
  did change under ratified amendment 2. Its *Test placement* row names `tests/cli.rs` for the added test;
  this work order adds none there and adds one to `mokiterions-tui/tests/options.rs`. Its *Performance and
  resilience* survivor counts and its "same termination ticks" clause are the two stale figures in section 9
  above. **Amendment 6 of `WO-MOK-020` is the fix and it is OUTSTANDING** — it was found after the owner's
  six acts and was not put to them. Marking those rows met would be the implementation agent rewriting the
  contract it is measured against.
- **The width argument put to the owner for amendment 2 was wrong**, and `WO-MOK-020` now records it as
  wrong rather than dropping it. `--ticks <whole number>` would not have overrun any line: that synopsis
  line is 55 columns after the change and 49 before, and the widest synopsis line is 68. The ratification
  of `<number>` stands on its own; the cost figure offered beside it did not hold.
