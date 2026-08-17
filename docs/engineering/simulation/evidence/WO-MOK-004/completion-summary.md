# WO-MOK-004 completion summary

Reported in the order `WO-MOK-004`'s *Completion report format* requires.

## 1. Requirement and affected components

`REQ-MOK-018` — *State every option's effect and default in the help output*, under `SPEC-MOK-001`'s
*Help output* section.

| Component | Change |
| --- | --- |
| `src/cli.rs` | the `USAGE` constant rewritten, with a doc comment; nothing else in the file touched |
| `tests/cli.rs` | 7 tests and 8 private helpers added; the 5 existing tests unchanged |
| `tests/process.rs` | 1 test added; the 4 existing tests unchanged |
| `docs/engineering/simulation/specifications/SPEC-MOK-001.md` | the amendment, approved before the code changed |

Unchanged: `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/main.rs`, `src/simulation.rs`,
`tests/density.rs`, `tests/termination.rs`, `tests/viability.rs`, and every managed harness file.

## 2. The final usage text

Byte-identical to the rendering `WO-MOK-004` authorized before the work began, verified by extracting the
work order's `text` block and diffing it against the program's output.

```text
Usage: Mokiterions [--seed <u64>] [--ticks <u64>] [--policy <baseline|reference>]
                   [--density <percent>] [--trace-actions]
       Mokiterions --help

Options:
  --seed <u64>                   Entropy stream seed. Default: 0.
  --ticks <u64>                  Ticks to run; must be greater than zero.
                                 Default: 100.
  --policy <baseline|reference>  Decision source. Default: reference.
  --density <percent>            Resource density per territory, at most two
                                 decimal places. Default: 0.75.
  --trace-actions                Emit one action trace per living-agent decision
                                 opportunity. Off unless given.
  --help                         Print this usage and exit without running.

Options may appear in any order and at most once.

The reference policy is a deterministic development instrument, not autonomous
behavior. It seeks and consumes perceived food so that world viability can be
measured. The baseline policy selects uniformly among valid actions.

--density is the percentage of a territory's cells that hold a resource. It sets
the initial endowment, the territory capacity, and the replenishment target
together. Only the densities declared in the requirements carry a population
viability floor.
```

| Measure | Before | After |
| --- | ---: | ---: |
| Lines | 12 | **25** |
| Non-blank lines | 10 | **21** |
| Documented defaults | 1 of 5 | **5 of 5** |
| Bytes | 700 | 1,326 |
| Carriage-return bytes | 0 | **0** |

## 3. Authorized local decisions

All inside `WO-MOK-004`'s decision envelope.

- **Construction.** `concat!`, one string literal per output line with an explicit `\n`, rather than one
  long escaped literal or a multi-line literal. A multi-line literal would take its line endings from
  however the file was checked out on a Windows worktree with end-of-line conversion, which would make the
  program's output depend on the checkout rather than the commit. `concat!` is a `std` macro evaluated at
  compile time, so it adds no dependency and keeps `USAGE` a `pub const &str`.
- **Alignment and width.** Descriptions start at column 34, aligned to the longest option-and-placeholder
  (`--policy <baseline|reference>`) plus two spaces. Continuation lines align with the description column.
  The widest introduced line is 80 columns, within the envelope's limit. The widest line in the text is
  81 — the first line of the synopsis, unchanged by this work.
- **Wording.** Six descriptions, each one sentence or two, within the substance `SPEC-MOK-001` fixes.
  `--policy`'s is the shortest at two words, leaning on the paragraph below the block that says what the
  sources are; that division is why the paragraph was kept.
- **Prose edit.** The density paragraph lost `with at most two decimal places` and `It defaults to 0.75.`
  and was re-wrapped. Both facts are now in the `--density` entry. This is `SPEC-MOK-001`'s
  state-each-fact-once clause, not a stylistic choice: it is the one place where a second copy of a
  default already existed.
- **One test name differs from the work order's list.** `every_documented_option_is_accepted_by_the_parser`
  was written as `the_documented_options_are_exactly_the_options_the_parser_accepts`, because it asserts
  both directions and the authorized name understates the second. Test names are inside the envelope.

## 4. Tests added

All eight are public tier under `SPEC-MOK-002` rule 8, in files whose declared subjects already cover
them. Every one reaches the program through rule 5's interface.

| Test | File | Public items used |
| --- | --- | --- |
| `every_option_the_synopsis_names_has_an_options_entry` | `tests/cli.rs` | `cli::USAGE` |
| `the_documented_options_are_exactly_the_options_the_parser_accepts` | `tests/cli.rs` | `cli::USAGE`, `cli::parse` |
| `each_documented_default_parses_to_the_applied_default` | `tests/cli.rs` | `cli::USAGE`, `cli::parse`, `cli::Command`, `simulation::Config` |
| `the_entries_state_the_constraints_that_decide_validity` | `tests/cli.rs` | `cli::USAGE`, `cli::parse` |
| `the_flags_state_their_effect_and_no_default_value` | `tests/cli.rs` | `cli::USAGE`, `cli::parse`, `cli::Command`, `simulation::Config` |
| `each_declared_default_is_stated_once` | `tests/cli.rs` | `cli::USAGE` |
| `the_help_text_states_order_and_repetition` | `tests/cli.rs` | `cli::USAGE`, `cli::parse`, `cli::Command`, `simulation::Config` |
| `the_diagnostic_path_appends_the_whole_usage_text` | `tests/process.rs` | `execute`, `cli::USAGE` |

No check is written only against `cli::USAGE`; such a check would hold for every possible value of it,
including an empty string. `new-tests.md` names the outside oracle each test uses. The central test
declares no expected value: it scrapes each stated default out of the printed text, feeds it back through
`cli::parse`, and requires the result to equal the configuration an empty argument list produces.

The coverage-totality property is written over the parser's own `match` arms, read out of `src/cli.rs`
through `include_str!`, not over a hand-written list — the weak form `VER-MOK-004`'s residual uncertainty
warns "looks identical when it passes". The cost is a documented coupling to that file's layout.

## 5. Census reconciliation

52 → **60**, +8, 0 ignored before and after. `diff` of the two sorted `cargo test -- --list` outputs
contains eight `>` lines and no `<` line and no changed line, which is itself the proof that all 52
pre-existing names survive unaltered: a rename would show as a paired removal and addition.

Additions: the eight named in §4. Per-executable: `tests/cli.rs` 5 → 12, `tests/process.rs` 4 → 5,
everything else unchanged, internal tier steady at 37, no test changed tier, no test file created.

## 6. Verification commands and results

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | exit 0, no differences |
| `cargo clippy --all-targets --all-features -- -D warnings` | exit 0, no findings, re-run after `cargo clean -p Mokiterions`; no `allow` added, invocation not narrowed |
| `cargo build` | exit 0 |
| `cargo test` | exit 0 — 60 passed, 0 failed, 0 ignored, one invocation, no extra flag or environment variable |
| `cargo doc --no-deps --lib` | 13 public items, identical to `WO-MOK-003`'s inventory |
| `validate_engineering_artifacts.py` | PASS — 40 artifacts, 0 errors, 0 warnings |
| `harnessctl preflight --phase review --work-order WO-MOK-004` | PASS |
| `harnessctl doctor` | PASS — every managed file matches distribution |

Baseline comparison, cell class by cell class, against the surface `VER-MOK-004` stated in advance:

| Cell class | Cells | Required | Observed |
| --- | ---: | --- | --- |
| Matrix manifest and summary lines | 43 | byte-identical | **byte-identical** — three `diff`s empty, including the two full traced runs |
| `--help` | 1 | exit and standard error unchanged, standard output replaced | exit `0`, standard error empty, standard output 699 → 1,325 bytes |
| Invalid configuration | 14 | exit, empty standard output, and the `configuration error:` line identical; usage block replaced | **all 14 identical** on exit `2`, `stdout_len: 0`, and the diagnostic line |
| Valid single tick | 1 | byte-identical | **byte-identical** |

Mechanically: removing the standard-error-body lines from the 342-line diff leaves four lines, and all
four are the help case's standard-output digest and length. Nothing outside the stated surface changed.
A repeated post-change run at one seed is digest-identical to itself.

## 7. Deliberate scratch divergences

Three performed, none committed; `src/cli.rs` verified back to digest `5498a62e…` after each, and
`grep` for the three scratch strings returns 0.

| # | Divergence | Result |
| ---: | --- | --- |
| A | applied `--ticks` default 100 → 1000, text untouched | `4 failed; 8 passed`. `each_documented_default_parses_to_the_applied_default`: *"the help states 100 as the default for --ticks, which is not the value the program applies when --ticks is omitted"*. Three pre-existing tests also failed, since they pin the applied defaults. |
| B | printed `--seed` default 0 → 1, code untouched | `1 failed; 11 passed`. Only `each_documented_default_parses_to_the_applied_default`: *"the help states 1 as the default for --seed, which is not the value the program applies when --seed is omitted"*. |
| C | `"--verbose"` arm added to `parse`, no help entry | `1 failed; 11 passed`. `the_documented_options_are_exactly_the_options_the_parser_accepts`: *"every option the parser accepts is described, and nothing else is"*. |

**B is the load-bearing one.** All five pre-existing `tests/cli.rs` tests passed under it, including
`defaults_are_stable`, and so would `help_exits_successfully`, which asserts the help output equals
`cli::USAGE`. Before this work the program could have advertised a default it did not apply with the whole
suite green. C is beyond scenario 3 and discharges `VER-MOK-004`'s recorded residual uncertainty about the
coverage property.

## 8. Per-seed survivor counts

`8, 11, 8, 9, 11` on seeds `0, 1, 42, 123, 777` at 1,000 ticks, declared density, reference source —
identical to the baseline, and identical to what `VER-MOK-004` fixed in advance. Whole summary lines
match, including per-territory population and per-class resource counts, not just the totals. Two seeds sit
on the floor of eight with zero margin and both still do, at the same tick, with the same distribution.

The four 10,000-tick runs at seed `123` are byte-identical to `WO-MOK-003/resilience-10k.txt`, with
survivors plus deaths equal to twelve in every run. A first attempt at seed `42` produced non-matching
rows; the cause was the invocation, not the program, and it is recorded in
`resilience-and-viability.md` rather than discarded.

## 9. Retained evidence

Under `docs/engineering/simulation/evidence/WO-MOK-004/`:

| Path | Contents |
| --- | --- |
| `capture.sh` | the capture script, byte-identical to `WO-MOK-003`'s, run unmodified for both sides |
| `baseline/` | pre-change capture at `08dc8d47`: `manifest.txt` (43 cells), `summaries.txt`, `exit-codes.txt` (16 cases), `full/` (2 traced runs), `usage.txt`, `test-census.txt` |
| `after/` | the same records post-change, plus `test-run.txt` |
| `baseline-comparison.md` | the comparison, class by class, with the classification method |
| `exit-codes.diff.txt` | the raw 342-line diff of the 16 named cases |
| `usage-text.md`, `usage-text.diff.txt` | both texts in full, the line-by-line diff, and the measurements |
| `specification-to-help.md` | the `SPEC-MOK-001` table transcribed and diffed against the rendered help |
| `new-tests.md` | the eight tests, their oracles, and the run output |
| `drift-demonstrations.md`, `drift-scratch-{a,b,c}.txt` | the three divergences and their transcripts |
| `public-surface-inventory.md` | 13 items, regenerated, diffed against `WO-MOK-003`'s |
| `test-census.md` | the name-by-name reconciliation |
| `resilience-and-viability.md`, `resilience-10k.txt` | survivor counts and the four long runs |
| `static-checks.txt` | fmt, clippy, build transcripts, and `git status` |
| `amendment-approval.md` | the ordering evidence for the `SPEC-MOK-001` amendment |
| `completion-summary.md` | this file |

## 10. Residual limitations and deferred items

- **The `--ticks` default is unchanged at `100` and its value remains a deferred product decision**,
  recorded in `REQ-MOK-018`'s *Open decisions* on 2026-08-17. Every viability obligation in this
  repository is stated at 1,000 ticks, so a run with no arguments satisfies no declared viability floor.
  This work makes that visible by printing `100` and deliberately does not change it. An operator reading
  `Default: 100.` beside a 1,000-tick floor may reasonably find it misleading; that is an observation for
  the product owner, not a defect in this work.
- **Readability is assessed, not measured.** No mechanical check establishes that a description helps an
  operator. The assessment is recorded in `specification-to-help.md`, by the standard `VER-MOK-004` sets:
  no entry may be a restatement of its own option's name.
- **The coverage-totality test is coupled to the layout of `src/cli.rs`.** Reformatting the `match` arms
  breaks it loudly rather than silently, which is the right direction, but it is a coupling and it is
  recorded rather than hidden.
- **Verified against the declared matrix, not the input space.** Undeclared seeds, undeclared densities,
  and the release profile are not covered.
- **Terminal rendering is not verified.** Widths are measured; behaviour in a narrow window, under
  wrapping, or through a pager is not. A later complaint about it is an amendment, not a defect.
- **Wording is not claimed to be optimal**, only present, accurate against the specification, unique,
  equal to the applied defaults, and readable. Improving it is an amendment to `SPEC-MOK-001`'s *Help
  output* section.
- **No architecture is engaged**, deliberately, with the counter-argument disclosed in `WO-MOK-004`.

## 11. Worktree and candidate-commit status

| Fact | Value |
| --- | --- |
| Branched from | `master` at `08dc8d47aa569c77fb7c9b091e328eb8f40c5285`, the commit the baseline was captured at |
| Implementation branch | `feature/help-output-options-block` |
| Candidate commit | named by `VREC-MOK-004`; not stated here, because a count of commits on the branch is falsified by the next one |
| Modified tracked files | `src/cli.rs`, `tests/cli.rs`, `tests/process.rs`, `docs/engineering/simulation/specifications/SPEC-MOK-001.md` |
| Added paths | `REQ-MOK-018.md`, `VER-MOK-004.md`, `WO-MOK-004.md`, `evidence/WO-MOK-004/` |
| Managed harness files | untouched; `doctor` PASS |
| Scratch residue | none; `src/cli.rs` digest verified after each divergence |
| Released, tagged, or published | **No.** |

`WO-MOK-004`'s assurance block requires **commit-bound verification**. The equivalence claim is only
worth something against a recorded baseline at a specific commit, so verification cannot be discharged
against an uncommitted tree — which is why the branch was taken from the exact commit the baseline was
captured at, so the diff under review is the whole of the change the comparison measured.

The commit and the pull request are not the verification decision. `VREC-MOK-004` records that decision
against the candidate commit, and the transition to `verified` is the accountable assurance owner's act.
