# Evidence — WO-MOK-008

`WO-MOK-008` closes a defect in the observer's provenance footer. `SPEC-MOK-003` rule 8 enumerated six
fields, did not rank them, and said nothing about what the footer does when the narrowest tier still will
not fit — so the renderer answered with an unspecified fifth string and handed the pane a row wider than the
pane. The pane kept a prefix. **An operator at `34 × 22` read a configured tick limit of `18446744073`
where the run's was `18446744073709551615`, and a retained-event count of `13` where the run had retained
`136`.** Both are values no field of the run holds. This directory is the work order's retained evidence,
and its *Evidence to record* is the list satisfied here.

**What this work order is.** It changes executable behaviour in `mokiterions-tui/src/render.rs` and nowhere
else in code; **`mokiterions-core/` is not touched by one byte**, which `git diff --stat -- mokiterions-core`
measures rather than states. The rest of the scope is specification and verification text no build reads,
which is why `commit_bound_verification` is `required` rather than inferred. No requirement is created:
`REQ-MOK-024` and `REQ-MOK-027` are `approved` and already carry the obligation.

**What its status is.** `WO-MOK-008` is **`in_progress`** and is left there. `implemented` asserts the
completed change **and** the retained evidence as an accountable judgement, which is the owner's act and not
the implementation agent's — the precedent is `WO-MOK-018`, and `WO-MOK-013` before it. **No `VREC` is
written here.**

**One provision is OUTSTANDING and it is not presented as approved text.** The owner approved the
`SPEC-MOK-003` rule 8 amendment, fixed clause 4's order over the six preamble fields, approved the
`VER-MOK-005` extension, and directed implementation and evidence in one act on 2026-08-22. Clause 4's
**first row — the candidate commit shed ahead of every other field — was not part of the question put to
them**, and is marked OUTSTANDING in the specification's row of the same date. `footer-shedding.md` records
the alternative not taken, its measured cost, and exactly what a reversal would move: one table row, one
array element, and one case.

**The measurement that decided the whole shape.** Three alternatives to shedding were measured before the
order was fixed, and each is recorded with its cost rather than dismissed: a wider floor costs **50** columns
for a `u64::MAX` seed at the default tick limit and **84** at the arithmetic worst case; a second footer row
takes the canvas interior at the floor to **15** rows against rule 5's declared minimum of `32 × 16`; a
denser radix reaches **72** columns in hexadecimal and **63** in base 36, against a floor of **34**. None
reaches the floor, which is why rule 8 clause 6 guarantees the entropy seed rather than the field set.

**The finding that matters most is not about the footer.** The whole 302-case suite passed against the
defective renderer, and **two of the ten cases added here were themselves blind to the defect when first
written** — one swept the declared seeds at the default tick limit, where the defect does not appear; the
other asserted that no value in the row was cut, which the superseded renderer satisfied because it never
cut a row itself. Both were caught by running each added case against the superseded implementation, and by
nothing else. `counterfactual.md` is that run.

## The files

| File | What it establishes |
|---|---|
| `README.md` | This index. Every file in the pack, what it establishes, and what is deliberately absent. |
| `completion-report.md` | The six-point report `WO-MOK-008`'s *Completion report format* specifies: rule 8 as amended with the test discharging each clause, the five authorized local decisions and the reserved set left alone, every verification command with its result and the three `MOKITERIONS_COMMIT` states, the retained paths, whether a release may stamp a commit **stated as a measurement**, and the residuals, findings and final worktree state. |
| `footer-tier-fallthrough.md` | **The defect report, unchanged.** It predates this work order and *Evidence to record* requires it retained as it stands. |
| `footer-shedding.md` | The arithmetic the technical owner was shown before fixing rule 8: the floor's 43-of-34 columns, the four candidates with their measured cost, the two positions in the order decided on measured rather than proposed ground, the alternative not taken for the OUTSTANDING provision, and the ladder with the two consequences that are not obvious from the rule. **It decides nothing.** |
| `rendered-footers.md` | Real frames through the real `render::draw`, before and after: nine declared viewports × seven configurations = **63 rows per tree**. 5 of 63 differ, all at the floor; the eight wider viewports are character-for-character unchanged; **2 of the 5 presented a value the run did not hold**. |
| `counterfactual.md` | Each of the ten added cases run against the superseded renderer. **7 of 10 fail there.** The 3 that pass are blind by construction and named as such rather than counted as coverage. Carries the two cases that were blind when first written and what strengthened each. |
| `commit-states.md` | The three states of `MOKITERIONS_COMMIT` at a rendered frame, each run clean and unclean. The floor row is identical in all three, which is completion-report item 5's measurement. Corrects the work order's own recompile reasoning by measuring it. |
| `verification-mapping.md` | `VER-MOK-005`'s eight added rows and two changed rows against the tests that discharge them, with **what each case sweeps** and which of `SPEC-MOK-004`'s two tiers it sits in and why. Names the one place the mapping argues rather than measures, and the assumption that would lapse the argument. |
| `replay.md` | Two identical runs per declared verification seed, both the record stream and standard output compared as bytes. **20 digests, no difference.** States what this establishes and what it does not. |
| `gates.md` | Every declared gate with its output: formatter, linter at `--locked`, the 312-case census by target, and the dependency tree. Records the `rustfmt` line-ending finding rather than absorbing it. |

Ten files. Every figure in the nine written documents traces to a command in `gates.md` or to a captured run
embedded in `rendered-footers.md`, `commit-states.md`, `replay.md` or `counterfactual.md`.

## On the probe

`rendered-footers.md` and `commit-states.md` are measured through a probe binary that **lives outside this
checkout** and path-depends on `mokiterions-tui`. That placement is the point: nothing in the repository
depends on it, no interface is widened for it, no `#[cfg(test)]` hook is added, and it cannot be mistaken
for a test that the suite runs. `ARCH-MOK-002` names both of those patterns as prohibited and rule 6's
interface count is untouched by the probe's existence.

**A probe asserts nothing.** It writes down what the rendered buffer holds, so that judging what the
instrument shows is a separate act. Each row it reports is annotated with **the run's own values, read from
`Observer::config`, `Observer::snapshot` and `Observer::events`** rather than restated from the arguments —
which is what makes "this row presents a value the run does not hold" a measurement rather than an
inspection. The `CUT` marks in `rendered-footers.md` are that measurement, and they are not inferred from
the row's length: `s18446744073709551615 t100 @0 e136` fills the floor's 34 columns to the last cell and is
not cut.

The before column is the same binary built against the superseded renderer, substituted behind the same
private signature. `render.rs` is restored from an in-memory copy afterwards and the revert verified by
SHA-256 in both directions. `counterfactual.md` uses the same substitution.

This follows `WO-MOK-018`'s `wo018-oracle.rs`, `WO-MOK-013`'s `wo013-oracle.rs` and `WO-MOK-010`'s
`observer/frame-probe.rs`. It departs from them in one way, recorded rather than left implicit: **those
oracles were placed in the tree, run once and removed, and this one never entered the tree at all.** It is
not retained here as a snippet, because it is an ordinary cargo project rather than a module of this one and
a copy in this directory would be a second thing to keep true.

## Reading order

`completion-report.md` first — it is the six-point report the work order specifies and it states what remains
open. Then `counterfactual.md`, which is the one document that changes how the rest should be read: it is why
ten passing cases are not offered as coverage. `footer-shedding.md` next, for the arithmetic behind the
decision and the one provision the owner has not ratified. Then `rendered-footers.md` for the defect at real
frames, `verification-mapping.md` for the obligation-by-obligation discharge, and `commit-states.md`,
`replay.md` and `gates.md` last — each closes its half of the required verification and carries no obligation
forward.

## What is not here

- **No verification record.** `commit_bound_verification` is **`required`** and no `VREC` is written by this
  work order. A record binds a branch commit and never `master`'s merge, and writing one is a separate act
  with a separate owner.
- **No manual assessment.** `VER-MOK-005` contracts none for these cases. All ten are automated and the
  `REQ-MOK-027` residual is disclosed rather than assessed.
- **No claim that a passing case is evidence.** The counterfactual is the reason this pack exists in this
  shape, and `VER-MOK-005` discloses that the counterfactual is **not** part of the gate.
- **No engine measurement beyond "untouched".** No file of `mokiterions-core` changes, so the engine's own
  recorded figures are carried through rather than re-measured here. The replay comparison establishes the
  precondition for `REQ-MOK-025`'s non-perturbation property, not that property itself.
- **No non-perturbation comparison of its own.** `VER-MOK-005` measures an observed run against an
  unobserved one; no engine call site changed and the footer reads the configuration and the retained
  buffer by copy into a value struct that owns its fields. `WO-MOK-018`'s `non-perturbation.md` is the
  standing measurement.
- **No decision about whether a release stamps a commit.** *Out of scope* keeps that question elsewhere.
  Point 5 of the completion report states only that a stamp of any length now costs the footer nothing,
  as a measurement.
- **No wider floor, no second footer row, and no denser radix.** All three were measured and all three are
  recorded in `footer-shedding.md` with their cost. Each would have moved rule 5, and rule 5 is not this
  work order's.
- **No new dependency, no build script, and no widened public interface.** Every one is named in *Out of
  scope*; `pub const COMMIT` is unchanged and every type the added cases reach is private.
- **No edit to `WO-MOK-008`'s body.** Its *Approval preconditions* item 1 names an OUTSTANDING
  `SPEC-MOK-003` row from 2026-08-18 that was ratified on 2026-08-20 under `WO-MOK-012`. The body is the
  engineering owner's prose; the stale claim is carried as a finding in the completion report instead. Only
  `status` and `updated` change in that file.
- **No figure edited inside another work order's evidence, and none inside any `VREC`.** Evidence is re-run,
  not corrected; a record bound to a commit remains true of that commit.
- **No push and no pull request.** The change is committed on `wo-mok-008-footer-shedding` and stops there.
