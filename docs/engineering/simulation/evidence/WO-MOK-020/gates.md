# WO-MOK-020 gates

`WO-MOK-020`'s *Required verification* lists six items. All six were run and all six results are below,
including the one that **FAILs by design**. These are observations of the working tree; they approve
nothing and authorize no release.

Captured 2026-08-22 on branch `feature/help-output-clarity`, base
`f7b1c452039dc2f03010ca8b8cc81e73c54727c0`.

**Two capture points, and which sections belong to which.** Sections 1 to 5 and the `doctor` and `validate`
readings were first taken at commit `6d6be40`, when the work order was `draft` and its five amendments were
**OUTSTANDING**. The owner then ratified all five and directed the transition to `implemented`, which
changed markdown only — no `.rs` file, no `Cargo.toml` and no lockfile. Section 6 was re-run in full after
those edits and is reported at the ratified state; `doctor` and `validate` returned the same figures as
before, which is expected, since neither reads amendment prose. The code gates were not re-run after the
doc-only edits and are reported as taken at `6d6be40`. What binds a tree rather than a moment is
`VREC-MOK-020`, prepared against the commit that carries both.

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

Every touched source file is uniformly CRLF, matching the checkout, with **zero** bare LF:

| File | CRLF | bare LF | bare CR |
| --- | ---: | ---: | ---: |
| `mokiterions-core/src/cli.rs` | 231 | 0 | 0 |
| `mokiterions-core/tests/cli.rs` | 577 | 0 | 0 |
| `mokiterions-tui/src/options.rs` | 242 | 0 | 0 |
| `mokiterions-tui/tests/options.rs` | 184 | 0 | 0 |
| `work-orders/WO-MOK-020.md` | 390 | 0 | 0 |
| `requirements/REQ-MOK-018.md` | 157 | 0 | 0 |
| `specifications/SPEC-MOK-001.md` | 836 | 0 | 0 |
| `specifications/SPEC-MOK-003.md` | 1265 | 0 | 0 |
| `verification/VER-MOK-004.md` | 280 | 0 | 0 |
| `evidence/WO-MOK-020/README.md` | 61 | 0 | 0 |
| `evidence/WO-MOK-020/gates.md` | 324 | 0 | 0 |
| `evidence/WO-MOK-020/usage-text.md` | 431 | 0 | 0 |
| `evidence/WO-MOK-020/preflight-implemented.txt` | 85 | 0 | 0 |

Re-measured 2026-08-22 after the ratification edits; the work-order row was 265 before them and the four
amended artifacts and the preflight capture were not in this table at all. The **bare CR** column was added
because the first attempt at `preflight-implemented.txt` had 69 of them: `harnessctl` already emits `CRLF`,
so writing its output through a `CRLF` translation produced `\r\r\n` on every line the harness had wrapped.
The capture was rewritten by normalizing to `LF` before the single translation, and the column exists so
that a reader can see the check was run rather than assumed.

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
  assurance. `commit_bound_verification` is `required`, and `VREC-MOK-020` now exists at status `ready`,
  prepared against the commit that carries the ratifications. `ready` is not `verified`: accepting a record
  is the assurance owner's separate act, and nothing below this line was decided by the agent that wrote
  it. Until that acceptance, the gates on this page are the strongest claim available — that the tree was
  measured, not that the work is verified.
- **The width argument put to the owner for amendment 2 was wrong**, and `WO-MOK-020` now records it as
  wrong rather than dropping it. `--ticks <whole number>` would not have overrun any line: that synopsis
  line is 55 columns after the change and 49 before, and the widest synopsis line is 68. The ratification
  of `<number>` stands on its own; the cost figure offered beside it did not hold.
