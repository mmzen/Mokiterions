# The merge of `master` into the naming branch — what was resolved, what was re-measured, what is still owed

Merge parents: `feature/phase-2-5-naming` at `4606145` and `master` at `2157f77`.
Merge base: `524a675`. Date: 2026-08-19.

**A second merge follows this one.** `master` moved to `44c24d8` while this was being written (PR #28,
the 0.1.0 observation window and four loose ends). It is merged in too, and it conflicted with
nothing: the five files it touches — `docs/RELEASE_RUNBOOK.md`, `REPOSITORY_CONTEXT.md`,
`REQ-MOK-037.md`, `SPEC-MOK-005.md` and a new `evidence/release-0.1.0-observations/` note — are none
of them this chain's, none of them source, and none of them a file any measurement here reads. The
gates were re-run at it all the same: **212 passed, 0 failed, 0 ignored**, fmt and clippy clean with
both crates re-linted, validator PASS over 102 artifacts with 0 errors and 0 warnings, dashboard PASS
over 335 relations with the same 6 warnings, and the inspector's same 18 findings. The dashboard's
snapshot digest moves to `2f5274d4…` because `master` edited two of its own artifacts; every figure
below is unchanged, and the tables in this directory are measured at the `2157f77` merge as they say.

This directory is **not a verification record and takes no decision.** `VREC-MOK-011` is bound to
`9ddcf83` and stays bound to it; a record for the merge is a new record. What is here is the
measurement the merge itself needs: which conflicts there were, how each was resolved, and which of
`VER-MOK-011`'s oracles have been re-derived on the merged tree against which have not.

Two files outside this directory change with it, and both are disclosures rather than measurements:
the packet's `README.md` gains a paragraph and a section pointing here, and `SPEC-MOK-004`'s naming
row carries the re-measured figures with a sentence saying they were re-measured after the owner
approved the row. Every other file the packet retains is untouched, so every digest it records still
stands — and none of the ten files here is in `VREC-MOK-011`'s 221-path evidence list, because they
did not exist when that record was written.

## Why the merge is against `master` and not the branch this PR targets

Draft PR #22 targeted `feature/phase-2-individuality`. That branch has since merged whole into
`master` (PR #17 at `5eed5a9`), so it is now an ancestor of `master` and merging it would have settled
nothing: `master` has moved on past it with a 0.1.0 release (`RLS-MOK-001`, PRs #25 and #26), a
fixture-independence fix (#24), post-release governance facts (#27), release CI (`a8fa962`) and the
observation window (#28). The same four files conflicted against either branch, so the merge was taken
against `master` directly. **The pull request's base was then retargeted to `master` on the repository
owner's instruction**, which is why its diff reads this chain's work and not `master`'s.

## The four conflicts, and how each was resolved

Thirteen conflict regions in four files. None of them was a name.

| Path | Regions | Both sides wanted | Resolution |
|---|---|---|---|
| `mokiterions-tui/src/render.rs` | 8 | `master` made an entry line a sequence of styled spans so each gauge carries its own survival band (rule 4.7); this branch gave `entry_lines` a name parameter and a six-column field | both, in disjoint columns: the name is line one's first six columns, and line two is `master`'s span row untouched |
| `docs/.../SPEC-MOK-001.md` | 1 | each chain appended an amendment row | `master`'s three rows kept verbatim and in order, this branch's naming row appended after them |
| `docs/.../SPEC-MOK-003.md` | 1 | the same | `master`'s four rows kept verbatim, the naming row appended, and one sentence of **the naming row itself** rewritten (below) |
| `docs/.../SPEC-MOK-004.md` | 3 | the same, plus rule 9's and rule 10's counted tables | `master`'s rows and figures kept, the naming row appended with **every figure in it re-measured on the merge** |

`mokiterions-tui/tests/render.rs` did not conflict and was edited anyway; that is the one place the
merge had to change an inherited assertion's mechanism rather than its arity, and it is disclosed
below.

### `render.rs`: two designs for one roster entry

The two changes are orthogonal and the merge keeps both properties, each asserted:

- The name occupies line one's first six columns, so the identifier begins at column six. `master`'s
  35-column bar overhead and its `bar_width` arithmetic are on line two and are neither read nor
  written by the name.
- Line two is unchanged: the same spans, the same three bands with the same `80..=100` /
  `40..=79` / `0..=39` boundaries, the same deliberately unbanded `fear`. The naming test asserts
  this by comparing whole `Line` values rather than their text, so a band lost to the merge would
  fail it.
- `draw_roster` applies the row style to each `Line` returned rather than to a string, which is what
  keeps reversed video covering a selected entry while every gauge keeps its band inside it.

### The inherited tests the merge touched

| What | Where | Why |
|---|---|---|
| Six call sites gained the name argument | `src/render.rs`, in `the_bar_row_reproduces_the_specified_form`, `banding_changes_no_character_of_an_entry`, `each_gauge_carries_its_own_band_and_nothing_else_carries_one`, `a_band_reads_only_the_value_it_is_given`, `the_collapsed_form_takes_no_band` and `a_zero_value_is_a_zero_and_an_absent_value_is_a_dash` | `entry_lines` gained a parameter. Each passes a fabricated stand-in, `"Blip"` or `"Ort"`, so no engine name enters the observer |
| One expected string | `the_collapsed_form_takes_no_band`, now `"Ort   M01  A  h 12 s 55 e 88 f  7"` | rule 4 as amended for `REQ-MOK-041` puts a six-column field before the identifier. Clause 7's claim — that the collapsed form takes no band — is untouched, and the comment at the assertion says so |
| The row locator was made name-aware | `tests/render.rs`: `const NAME_COLUMNS: usize = 6` and `entry_row` matching the identifier at columns 6.. rather than at the start of the line | `master`'s locator found an entry by `starts_with(id)`, which the name field displaces. Two public-tier band tests failed on it before the fix. A bar row cannot be matched by accident: its columns six to eight are the health gauge's label and the opening cells of its bar |

No test was added, removed or renamed by the merge, which `census-reconciliation.txt` measures rather
than asserts. **Whether editing an inherited assertion's expected string is inside `VER-MOK-011`'s
stop condition 1 is the technical owner's reading, and it is unchanged by the merge having a reason:
the reason is recorded, the decision is not taken here.**

### The one sentence of an approved row that was rewritten

`SPEC-MOK-003`'s naming row ended with a sentence about `SPEC-MOK-003`'s other amendment rows that
`master`'s four new rows made stale. It is replaced by one that names those four rows and states that
clause 7 and the naming row do not meet — the bands colour cells on line two, the name is added to
line one, and the banded set, its three boundaries, the unbanded `fear`, the 35-column overhead and
`bar_width` are neither read nor written by it. The row says of itself that this sentence is the only
part of it the merge rewrote and that it changes no provision.

## What was re-derived here, and the figures

| Oracle or gate | Re-derived | Result |
|---|---|---|
| 3 — test census | **yes** | `master` 200 -> merge 212; 12 additions, all this work order's; **0 removals, 0 renames**. `census-reconciliation.txt`, `test-census.txt`, `test-run.txt` |
| 5 — governance half | **yes**, with the base changed from the branch base to `master` | **PASS**, `amendment-approvals.md` via `amendments-vs-master.py` |
| 5 — interface half | **yes** | rule 6 unchanged at **94 items / 118 `pub` lines / 24 public fields**; engine interface byte-identical; one parameter renamed in one signature. `interface.txt` |
| `SPEC-MOK-004` rules 9, 10, 11 | **yes** | rule 9 total **88**, rule 10 total **39**, rule 11 observer **127** / engine **85** / workspace **212**. The row in the specification carries these and discloses that they differ from the figures approved on the unmerged branch |
| rule 10's `render.rs` item counts | **yes** | 48 declarations, identical name sets at both trees. `render-items.txt` |
| the declared gates | **yes** | fmt clean, clippy clean with both crates re-linted, `cargo tree` 1 and 111 lines, one workspace `cargo test` at 212 passed / 0 failed / 0 ignored, validator PASS 102 artifacts 0/0, dashboard PASS 335 relations, inspector 18 findings. `gates.txt` |
| the harness state | **yes** | the merge adds three inspector readings, all three this chain's own, and **no warning**. `governance.txt` |
| the static searches | rerun 1 of 6 | no engine name anywhere in `mokiterions-tui`. The other five read the engine's naming path, which the merge does not touch |
| 1 — projected byte comparison | **no** | 90 matrix cells, ~110 MB of stream per side, captured against `524a675`. Owed |
| 2 — entropy neutrality | **no** | owed. Its two instruments and the mutation control need re-running on the merged tree |
| 4 — rendered buffers | **no** | owed, and the one most likely to have something to say: `master` rewrote the row the frames capture |

The merged tree differs from `master` in 242 files, 9 of them source: `simulation.rs`,
`tests/naming.rs`, `render.rs`, `spatial.rs`, `state.rs`, `verification.rs`, `tests/render.rs`,
`tests/spatial.rs` and `tests/verification.rs`, plus this chain's seven artifacts, three amended
specifications and the 221-path packet. Under `evidence/`, **only `WO-MOK-011` differs**: the
`WO-MOK-007` and `WO-MOK-010` packets in the merged tree are byte-identical to `master`'s, so the
merge inherits the parent chain's evidence whole rather than reconstructing it.

## Three things reported rather than fixed

1. **`amendment-approvals.md` here prints "measured against `2157f77`, the commit this work started
   from".** `2157f77` is the merge base, not where this work started; the sentence is the generator's
   own text with the base substituted. `amendments-vs-master.py` changes exactly one value, the base
   revision, and re-runs the script whole rather than editing its prose, because a generated report
   that has been hand-corrected is no longer generated. Both reports are retained: the packet's at
   the branch base, this one at the merge base.
2. **The inspector's `W-HEX-001` — "`WO-MOK-010` is implemented but has no evidence document keyed to
   its ID" — is `master`'s and is present at `2157f77` too.** This branch neither causes nor clears
   it. Retaining evidence keyed to `WO-MOK-010` is the engineering owner's act on that work order.
3. **Rule 10's inherited item figures for `render.rs` are one out** — 47 private plus 2 public against
   48 declared, the `pub(crate)` constant counted twice. `render-items.txt` shows the measurement.
   Correcting an approved row of a specification this work order does not otherwise amend belongs to
   the artifact owner.

## What is still owed before anything binds this merge

1. **Oracles 1, 2 and 4, and the mutation control, re-derived on the merged tree** — engineering. Not
   carried over: `master` rewrote `render.rs` and moved `simulation.rs`'s inline tests out, so the
   earlier claim that the engine-side diff was comment-only does not hold.
2. **A new verification record bound to the merge commit** — it is a new record, not an edit of
   `VREC-MOK-011`, and its gate capture must be taken at that commit with a clean `git status`. The
   capture in `gates.txt` was taken on a resolved working tree with `MERGE_HEAD` present, which is
   stated there.
3. **Manual assessment 5 of `VER-MOK-011`'s seven** — assurance owner. Still without an author; the
   contract is not satisfied while it is outstanding.
4. **The stop-condition-1 reading on the inherited `render.rs` tests** — technical owner.
5. **`WO-MOK-011` to `implemented`** — engineering owner.
6. **Retargeting PR #22's base to `master`, and pushing** — repository owner.

## Files here

| File | What it is |
|---|---|
| `README.md` | this note |
| `gates.txt` | the declared gates at the merged tree |
| `test-run.txt` | the whole `cargo test` log the census and the gate figures come from |
| `test-census.txt` | the merged tree's target-qualified census, 212 names |
| `census-reconciliation.txt` | oracle 3 across the merge: `master` 200 -> 212, twelve additions named, zero removals |
| `interface.txt` | oracle 5's interface half: four enumerations in full and the single line that differs |
| `render-items.txt` | rule 10's item counts at both trees, and the arithmetic note on the inherited row |
| `governance.txt` | validator, dashboard and inspector at `2157f77` and at the merge, side by side |
| `amendment-approvals.md` | oracle 5's governance half, generated at the merge base: **PASS** |
| `amendments-vs-master.py` | the wrapper that re-runs `analysis/amendments.py` with the base changed and nothing else |
