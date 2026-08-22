# `WO-MOK-008` completion report

In the six-point format the work order's *Completion report format* section fixes.

The work order is left at **`in_progress`**. The transition to `implemented` asserts the completed change
**and** the retained evidence as an accountable judgement, and that is the owner's act rather than the
implementation agent's — the precedent is `WO-MOK-018`, held at `in_progress` through implementation for
this reason, and `WO-MOK-013` before it. No `VREC` is written here.

## 1. Rule 8 as amended, and the tests that discharge each of its cases

`SPEC-MOK-003` rule 8 gains five clauses, decided by the repository owner acting as technical owner on
2026-08-22 and recorded in that specification's amendment row of the same date. In the rule's own terms:

| Clause | What it fixes | Discharged by |
|---|---|---|
| 4 | The order of loss — candidate commit, retained count with its marker, current tick, active source, resource density, configured tick limit — and that a field is shed only where the width will not hold it | `fields_leave_the_footer_in_the_order_clause_4_fixes`, `the_footer_sheds_no_more_fields_than_the_width_requires`, `the_footer_sheds_the_specified_fields_at_the_floor` |
| 5 | The retained count and its truncation marker are one field | `the_retained_count_and_its_marker_are_never_separated` |
| 6 | The entropy seed is never shed | `the_entropy_seed_survives_the_floor_across_the_whole_u64_range`, `the_footer_never_clips_a_value_at_any_declared_viewport` |
| 7 | Where not even the seed and one field fit, the seed is presented alone rather than any value cut | `no_footer_value_is_ever_cut_at_any_width`, and the `None` arm of `the_footer_sheds_no_more_fields_than_the_width_requires` |
| 8 | The labelling is the implementation's, the values are not: no abbreviation, no rounding, no re-basing | `no_labelling_changes_a_value` |
| 2, amended | Supplied means non-empty, so the three states of the compile-time variable are two outcomes | `a_commit_variable_set_to_the_empty_string_is_not_a_supplied_commit` |
| 4, first row | A candidate commit is shed before every field the rule requires — **OUTSTANDING** | `the_candidate_commit_is_shed_before_every_field_rule_8_requires` |

`verification-mapping.md` carries the same mapping from `VER-MOK-005`'s side, with what each case sweeps and
which tier it sits in. Ten cases are added: eight in-crate, two cross-crate.

**The implementation is one ladder and the search order is fixed by the rule rather than chosen.** Clause 8
makes labelling free and clause 4 sheds only under width pressure, so **shedding is the outer loop**: for
each number of fields lost, every labelling is tried before one more field goes. Two consequences are
recorded in `footer-shedding.md` because neither is obvious from the rule — at the greatest shed level the
row uses the *widest* labelling that fits, and clause 7's fall-back is unreachable at any viewport the
observer draws.

## 2. Authorized local decisions

Each falls inside the work order's *Authorized decision envelope*, which grants private names, the ladder's
expression in code, test names and fixtures, and comments.

| Decision | What was chosen | Why |
|---|---|---|
| The ladder's expression | `Provenance` value struct, `SHED_ORDER`, `PRESENTATION_ORDER`, `Spelling`/`SPELLINGS`, `Provenance::{seed_field, field, marker, row}`, `footer_row`, `supplied` — all private | Composing the row from a value struct is what lets the in-crate tier reach states no run reaches (a 20-digit tick limit at its own final tick) with nothing public widened. `pub const COMMIT` is unchanged; the interface count does not move. |
| Two orders, not one | `SHED_ORDER` is the order of loss; `PRESENTATION_ORDER` is the order the row presents what it kept | Shedding removes a field; it never moves one. One constant would have conflated the two and rule 8 states them separately. |
| The four labellings | spaced, labelled, terse, tersest — widest first | Clause 8's latitude. The values are identical in all four, which `no_labelling_changes_a_value` asserts rather than assumes. |
| Fixture values | Chosen so no field's text is a substring of another's | The order assertions match on substrings; a collision would pass on the collision rather than on the field. A tick limit of `1000000` contains a retained count of `100000`, which is exactly the trap. |
| The `--exact` omission | Cases are filtered by bare name, without `--exact` | `--exact` needs the full module path for an in-crate case, so `--lib NAME -- --exact` runs 0 tests. That silently reported passes during the counterfactual run and is recorded in `counterfactual.md`. |

**Nothing in the reserved set was decided here.** No field survives or is lost by the agent's choice, no
tier was invented, no field's value or source changed, rule 8.3's four prohibitions stand unedited, the
public interface is not widened, and the one status transition made — `draft` → `in_progress` — was
directed by the owner in the same act that approved the amendments.

## 3. Verification commands and results

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | **exit 0**, no warning. `--locked` is not optional: `.github/workflows/release.yml` passes it |
| `cargo test --workspace` | **312 passed, 0 failed**, 0 ignored. 302 of those existed before this work order and measured nothing about rule 8 |
| `cargo tree -p Mokiterions -e normal --locked --offline` | one crate. `git diff --stat -- mokiterions-core` is **empty** |
| `git diff --stat` | 5 files, 887 insertions, 68 deletions — `SPEC-MOK-003.md` 41, `VER-MOK-005.md` 64, `WO-MOK-008.md` 4, `render.rs` 679, `tests/verification.rs` 167 |
| `harnessctl validate` | **PASS**, 147 artifacts, 0 errors, 0 warnings |
| `harnessctl preflight --work-order WO-MOK-008 --phase start` | **PASS** |
| Deterministic replay | 5 declared verification seeds × 2 runs × 2 streams: **20 digests, no difference** |

Every harness command was run from the pinned 0.4.0 virtual environment. The machine-wide 0.4.1 install
FAILs this repository on template skew alone and no verdict is reported from it.

**The three `MOKITERIONS_COMMIT` states**, each measured at a rendered frame, twice — once after
`cargo clean -p mokiterions-tui` and once with no clean at all:

| Variable | `option_env!` reads | Field at `160 × 48` | Row at `34 × 22` |
|---|---|---|---|
| unset | `None` | absent | `s0 t100 d0.75% reference @0 e136` (32 cols) |
| set to `""` | `Some("")` | **absent**, per clause 2 as amended | identical, 32 cols |
| set to a 40-character SHA-1 | `Some("0123…4567")` | `commit 0123456789abcdef0123456789abcdef01234567` | identical, 32 cols |

**One correction to the work order's own reasoning, made as a measurement.** *Required verification* asks
for a forced recompile between values because "neither package has a build script, so Cargo does not treat
the variable as a rebuild input". That is not so: `rustc` records every variable an `env!` or `option_env!`
expansion reads in the crate's dependency information, and cargo invalidates on it. The clean and unclean
runs agree in all three states. The forced recompile was run regardless and both runs are retained.

## 4. Retained evidence paths

All under `docs/engineering/simulation/evidence/WO-MOK-008/`, indexed by that directory's `README.md`.

| File | What it establishes |
|---|---|
| `footer-tier-fallthrough.md` | The defect report, **unchanged**, as *Evidence to record* requires |
| `footer-shedding.md` | The floor arithmetic, the four candidates with their measured cost, the owner's grounds for the order, the alternative not taken for the OUTSTANDING provision, and the ladder |
| `rendered-footers.md` | 63 rendered rows per tree, before and after, at every declared viewport across seven configurations |
| `commit-states.md` | The three variable states, with the recompile measured rather than assumed |
| `counterfactual.md` | Each added case run against the superseded renderer |
| `replay.md` | The deterministic replay comparison |
| `gates.md` | Formatter, linter, test census and dependency tree |
| `verification-mapping.md` | `VER-MOK-005`'s extended cases against the tests that discharge them |
| `completion-report.md` | This report |
| `README.md` | The index, and what is deliberately absent |

## 5. Whether a release may stamp a commit, and at what length — as a measurement

**Yes, at any length, including a full 40-character SHA-1, at no cost to any field rule 8 requires.**

The measurement is the third column of the table in point 3 and the run behind it is in `commit-states.md`:
the row at `34 × 22` is **character for character identical in all three states of the variable**, 32
columns of 34. Clause 4's first row is what produces that — the commit is shed before every field the rule
requires, so a stamp of any length is absent at the floor and present wherever the width holds it.

The general form is stronger than the floor case, and
`the_candidate_commit_is_shed_before_every_field_rule_8_requires` is the measurement: across 4 seeds, 4
stamp lengths and every width from 0 to 200, **a stamped run sheds exactly the fields the same run sheds
unstamped**. Not fewer at some width, not more at another — the same, at every one.

Two things this does not say. It does not say a release *should* stamp one; the work order puts that
question out of scope and this only removes the obstacle. And the provision it rests on is **OUTSTANDING**:
if the owner reverses the commit's position in clause 4, this answer reverses with it, and
`footer-shedding.md` records the measured cost of that reversal — at the floor with the default seed, a
40-character stamp would displace both the resource density and the active decision source.

## 6. Residual limitations, and final worktree and candidate-commit status

**Residual limitations.**

- **`REQ-MOK-027` is not satisfiable at the floor** and rule 8 as amended concedes it. Six fields against
  a twenty-digit seed beside a twenty-digit tick limit is 43 columns of 34 before a separator. The
  requirement is the product owner's and the residual is disclosed in `VER-MOK-005` rather than resolved.
  The added cases verify the concession; they do not verify the requirement as written.
- **Clause 4's first row is OUTSTANDING.** The owner decided the order over the six preamble fields and was
  not shown the commit's position, because the question put to them did not contain it.
- **The counterfactual is not part of the gate.** A case added to `VER-MOK-005` in future is not run
  against a superseded implementation unless its author chooses to, and this work order's two blind cases
  were caught by nothing else. `VER-MOK-005` discloses it.
- **Three of the ten added cases are blind by construction** and are named rather than counted: they assert
  `Provenance::row`, which is machinery clause 4 introduces, so against a true revert they would not compile.
- **The rewritten row's field-set half is derived across seven viewports rather than swept.**
  `verification-mapping.md` states the derivation and the assumption that would lapse it — that the footer
  is one row and its content depends on the width alone.

**Findings carried, none of them this work order's defect.**

| Finding | Where it is recorded |
|---|---|
| `WO-MOK-008`'s own *Approval preconditions* item 1 says `SPEC-MOK-003` "already carries an **OUTSTANDING** amendment row from 2026-08-18". **That row was ratified on 2026-08-20 under `WO-MOK-012`.** The open row is the 2026-08-21 rule 10 two-line pairing, carried by `WO-MOK-018`. The work-order body is the engineering owner's prose and is not edited here | this report; `README.md` |
| `rustfmt` rewrote both touched Rust files entirely to LF in a worktree where `core.autocrlf` is true and every other Rust file is CRLF. Both were converted back and `--check` re-run clean | `gates.md` |
| The 302 pre-existing cases passed against a renderer presenting a tick limit of `18446744073` where the run's was `18446744073709551615`. A passing suite was never evidence about rule 8 | `counterfactual.md`, `gates.md` |
| The work order's forced-recompile reasoning is incorrect, and the correction is a measurement | `commit-states.md`, point 3 above |

**Final worktree.** Five tracked files modified — `SPEC-MOK-003.md`, `VER-MOK-005.md`, `WO-MOK-008.md`,
`mokiterions-tui/src/render.rs`, `mokiterions-tui/tests/verification.rs` — and ten evidence files under
this directory, nine of them new. No engine source, no manifest, no `Cargo.lock`, no harness-managed file,
and no unrelated change: the worktree was clean at `f7b1c45` before this work began. The out-of-repo probe
behind `rendered-footers.md` and `commit-states.md` lives at `C:/Users/mathi/mok-footer-probe` and is
**deliberately not in the repository**, so nothing here depends on it and it cannot be mistaken for a test;
every driver script it is called from writes only into `target/`, which is gitignored.

**Candidate commit.** The change is committed on **`wo-mok-008-footer-shedding`**, branched from `master` at
`f7b1c45`. This report is part of that commit, so it cannot name the commit's own hash; the branch tip is
the candidate. **Nothing is pushed and no pull request is opened** — the owner's authorization was to
commit on a branch and stop.
