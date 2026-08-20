# Evidence — WO-MOK-013

`WO-MOK-013` answers the three adverse observations `VER-MOK-005`'s manual assessment pass produced and
`WO-MOK-012` recorded: the roster gauge resolved almost nothing, the overlay keys were undiscoverable, and a
hidden pane was not announced. It implements `REQ-MOK-047`, `REQ-MOK-048` and `REQ-MOK-049` under
`SPEC-MOK-003`'s six amended provisions. This directory is its retained evidence, and `VER-MOK-013`'s
*Evidence to record* is the list it satisfies.

**What this work order is.** It changes executable behaviour, which `WO-MOK-012` did not. `mokiterions-tui/`
gains the wider gauge, the three-line roster entry, the permanent key hint and the actionable
enlargement notice; `mokiterions-core/` is not touched by one byte, and `engine-untouched.txt` measures that
rather than stating it.

**What its status is.** `WO-MOK-013` is **`implemented`**, moved there on 2026-08-20 by the repository owner as
engineering owner. The implementation agent held it at `approved` through implementation and through the three
post-implementation decisions, because `implemented` asserts the completed change **and** the retained evidence as an
accountable judgement. `WORKFLOW.md` makes a status change a record of authority rather than a confidence estimate, so
the status does **not** assert that `VER-MOK-013` is satisfied — it is not, on one count. `VREC-MOK-013` was captured in
the same act, bound to `41c20ca`, and is a **`ready` candidate rather than `verified`**. The work order's *Transition to
`implemented`* subsection carries both statements.

> **Later fact, 2026-08-20.** `VREC-MOK-013` is now **`verified`**, transitioned by the repository owner as
> accountable assurance owner in a separate instruction and a separate commit. **`WO-MOK-013` did not move
> and stays `implemented`** — nothing is approved by implication — and the record's binding still ends at
> `41c20ca`. `assurance-decision.md` records the decision; the sentence above describes the pack as it stood
> when the work order was transitioned, which is what it is for.

**What it closes, and what it does not.** `VER-MOK-013`'s **first** manual assessment was taken on 2026-08-20
and is **SATISFIED** — "the bar carried it" — which answers the adverse observation on the roster gauges that
this whole chain exists for. The **second** is **outstanding with no author** and has **no admissible assessor
among the people this chain has available**: the contract asks for a person who has not read `SPEC-MOK-003`
rule 7, and the available assessor has ratified six amendments to that document. Of the three options
`manual-assessment.md` sets out, the owner took the one that verifies the requirement rather than the one that
closes the row, so it stays open with its material prepared. `VREC-MOK-013` therefore records `REQ-MOK-047`
and `REQ-MOK-049` and **discloses `REQ-MOK-048` as unverified**.

> **Later fact, 2026-08-20 — the last sentence no longer describes the record.** In a separate instruction
> the assurance owner verified `REQ-MOK-048` on its **four automated cases, accepted in place of the manual
> assessment** `VER-MOK-013` designates as its primary evidence. So the record verifies all three
> requirements, and the third on evidence the contract calls *"a necessary condition and not the property"*.
> **Assessment 2 is still OUTSTANDING with no author, `VER-MOK-013` is not amended, and the contract is
> therefore not satisfied on that one count** — the row stays open with its material prepared, exactly as
> above, and taking it would now confirm or contradict rather than verify. `assurance-decision.md` states what
> was accepted and what it does not retire.

**Decision 1 is the one trade in this work order, and it is a trade.** The roster's three-line entry needs
36 interior rows, which the reference viewport has only if the log stays at six. The product owner chose to
hold the log at six rows and keep `REQ-MOK-020`'s twelve entries intact rather than amend the requirement —
and then stood by that choice when the cost figure it had been put on turned out to be wrong. The log carries
**four** recent events where it carried eight, not six where it carried ten. `log-height.md` measures both
forms out of rendered buffers and exhibits the counterfactual; `closing-review.md` records the correction as
the agent's error in how the option was framed. On the live pass of 2026-08-20 the question was put a third
time, with the result on screen rather than only the arithmetic, and the choice **stands**.

## The files

| File | What it establishes |
|---|---|
| `README.md` | This index. Every file in the pack, what it establishes, and what is deliberately absent. |
| `completion-summary.md` | The eight-item completion report `WO-MOK-013`'s *Completion report format* specifies: decision 1 with both corrections to how it was put, the three requirements against their before figures, every amendment with its ratifying act, the change surface, the test and interface counts, the validator and inspector results, both manual assessments, and nine findings reported rather than fixed. |
| `closing-review.md` | The twenty-two decisions this work order rests on, each with the role that took it, plus the choices the agent made inside the authorized envelope, the three locations the work order's enumeration missed, and the four stop conditions that could have fired and did not. Eighteen were taken before implementation, three on the live pass that followed it, and the twenty-second is the lifecycle act that follows from those three. |
| `log-height.md` | Decision 1 measured in both directions: six rows against ten, four log records against eight, twelve roster entries against ten, and that `12 × 3 = 36` fits with no slack either way. Names what was **not** corrected. |
| `gauge-resolution.md` | `REQ-MOK-047` measured: the gauge at 13 cells rather than 2, 14 renderable states rather than 3, and a ten-point step moving the fill at **91 of 91** values where it moved at 11. Reads `gauge-resolution.txt` against the `WO-MOK-012` before form. |
| `test-census.md` | Rule 11's test totals reconciled arrival by arrival, the static census validated target by target against the executed run, the one rename with both names, and every `VER-MOK-013` case mapped to the test that discharges it. |
| `manual-assessment.md` | Both contracted manual assessments: the first **SATISFIED** on 2026-08-20 with its author, date, role and terminal, the traced subject it was taken on, and the five-point-step fact disclosed before the judgment; the second **outstanding, author none**, with its route decided and the two declined options recorded. |
| `assurance-decision.md` | The verification decision of 2026-08-20 on `VREC-MOK-013`: the instruction verbatim, the three bases it was read against and the one the owner chose, what the decision accepted, **where it deviates from `VER-MOK-013` and in the contract's own words**, the nine things it does not retire, the collision determined by merge order, and the harness state measured either side of it. **Not in `VREC-MOK-013`'s `evidence_paths`**: it postdates the commit that record binds. |
| `identifier-collision.md` | The second identifier collision, measured and left undecided: four names — `WO-MOK-013`, `VER-MOK-013`, `VREC-MOK-013` and `REQ-MOK-047` — claimed by `origin/governance/adr-mok-006-third-party-crates` as well, with both sides approved and both pushed. The renumbering cost from this side, the eight conflicts a merge reports and the four that are this collision, and the evidence directories that collide in path without colliding in any file name. **Not in `VREC-MOK-013`'s `evidence_paths`**: it postdates the commit that record binds. |
| `discoverability-assessment.md` | The packet for assessment 2, written to be handed to whoever administers it: who may assess and who may not, which frame to show, the question verbatim, what each of the three outcomes means and where it belongs, what the administrator must not say, and how to record the result. Usable without reading `SPEC-MOK-003` rule 7 to the assessor. |
| `discoverability-frame.txt` | The frame that assessment 2 shows: 48 rows × 160 columns, the reference viewport at tick 200 of seed 42, extracted unmodified from `frames.txt` lines 14–61 so it can be shown without the banner that names the viewport. `?` appears on the header row and nowhere else. |
| `discoverability-frame-floor.txt` | The same for the floor, 22 rows × 34 columns, `frames.txt` lines 389–410. The hint is `?` alone and three hidden panes are announced at the last rung. The harder case, for a second assessor only. |
| `frames.txt` | A frame at all nine renderable viewports plus the floor, each with the header line in full and every roster entry drawn. The instrument's own output for the hint, the announcement and the ladder. |
| `gauge-resolution.txt` | For every gauge width the implementation can produce, the filled-cell count at every value in `0..=100`, counted out of cells. The sweep establishing that 47 columns is the whole set rather than assuming rule 5. |
| `test-census.txt` | The census itself: `#[test]` counts by name per file, now and at `ff3a155`, with arrivals, departures, the tier split, and that no test carries `#[ignore]`. |
| `test-output.txt` | The executed `cargo test --workspace`: 226 passing, 0 failing, target by target. |
| `static-checks.txt` | `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, both `exit=0`. |
| `interface.txt` | `SPEC-MOK-004` rule 6 re-counted module by module: 94 items, 24 public fields, 118 `pub` lines, unchanged. Stop condition 3 measured. |
| `non-perturbation.txt` | The observed run against the unobserved run: 7,534 records identical in order, no first difference, with the engine summary line and the export trailer shown rather than diffed against each other. |
| `non-perturbation-unobserved.txt` | The engine's own stream, retained whole so the comparison is reproducible with `diff`. |
| `non-perturbation-observed-export.txt` | The observed run's export, retained whole for the same reason. |
| `baseline-comparison.txt` | The third leg `VER-MOK-013` names: the engine binary at `ff3a155`, in a detached worktree of that commit, producing the same 7,535 lines as the binary now. |
| `engine-untouched.txt` | `git diff --stat origin/master -- mokiterions-core/` empty, and `origin/master` established as an ancestor of this branch so the two baselines are one commit. |
| `validation.txt` | The graph validator `PASS` on 108 artifacts with zero errors and zero warnings on all four planes, the inspector's 23 findings, and **the eleven warnings attributed across three trees** — none introduced by this implementation. |
| `wo013-oracle.rs` | The temporary oracle that produced `gauge-resolution.txt`, `frames.txt` and the three non-perturbation captures. Placed in the tree, run once, removed; retained here and **not in the tree**. |
| `analysis/interface.py` | The script behind `interface.txt`: counts public declarations per module in the working tree and at `ff3a155`. |
| `analysis/test-census.py` | The script behind `test-census.txt`: counts `#[test]` functions **by name** per file at both commits, since a count cannot distinguish a rename from a removal plus an addition. Run it with `PYTHONIOENCODING=utf-8`. |

**Twenty-seven files, 2.28 MB**, of which the two retained event streams are 1.97 MB. **`VREC-MOK-013`'s
`evidence_paths` names 25 of the 27.** The two outside it are `identifier-collision.md` and
`assurance-decision.md`, both written after the commit that record binds — the first in the same commit as
the record, the second at the verification decision that followed. A record's evidence set is the capture's,
not a later act's, which is the `VREC-MOK-007`, `VREC-MOK-010` and `VREC-MOK-011` precedent.

> **Earlier form, for a reader diffing this file.** It read *"Twenty-six files, 2.24 MB… `evidence_paths`
> names 25 of the 26, and the twenty-sixth is `identifier-collision.md`"* until `assurance-decision.md` was
> added. The 25 named paths did not change.

## On the oracle

`wo013-oracle.rs` was placed as a `#[cfg(test)] mod` under `mokiterions-tui/src/`, run once, and removed. The
source is retained; **it is not in the tree**, and `engine-untouched.txt` and `interface.txt` are both measured
on the tree without it.

It is inside `src/` rather than a standalone binary because the value table reaches
`Observer::replace_snapshot_for_test`, which is `#[cfg(test)]` and which no target outside the crate can link.
**No item was widened for it**, so rule 6's interface is untouched by its presence.

**An oracle asserts nothing.** It writes down what the buffer holds, so that judging what the instrument shows
is a separate act. Two properties of this one are worth naming, because they are what make its captures
independent of the code they measure:

- **Every figure is read out of a rendered buffer.** Bar widths, filled counts and gauge positions are
  counted from cells. Nothing in it knows `bar_width` or the overhead constant. The `WO-MOK-012` oracle
  recomputed the fill arithmetic in its own code; this one does not, which is the independence
  `VER-MOK-013` requires of its cases.
- **A capture is re-run, not corrected.** `VER-MOK-013`'s *Evidence retention* says so, and every figure in
  the four analysis documents traces to a line in one of these captures.

This follows `WO-MOK-012`'s `assessment-oracle.rs`, `WO-MOK-010`'s `observer/frame-probe.rs` and
`WO-MOK-006`'s `frame-and-export-oracle.rs`.

## Reading order

`completion-summary.md` first — it is the eight-item report `WO-MOK-013` specifies, and it states what remains
open. Then `closing-review.md` for who decided what, and whichever of `log-height.md`, `gauge-resolution.md` or
`test-census.md` covers the measurement at hand. `manual-assessment.md` last, because it is the one document
here that still records an obligation alongside a result. `identifier-collision.md` is orthogonal to all of
them: read it before citing any of this chain's identifiers outside this branch.

`assurance-decision.md` was added after that order was written and belongs at its end, with
`manual-assessment.md`: read the two together, because the assessment that is outstanding and the decision
that verified the requirement without it are one situation described from two sides.

`discoverability-assessment.md` is not in that order: it is not written for a reader of this pack but for
whoever administers the one assessment left open. Read it when you are about to take that assessment, and not
before — and if you have not read `SPEC-MOK-003` rule 7, do not read it at all, because you are one of the few
people who can still be its assessor.

## What is not here

- **No verification decision.** `commit_bound_verification` is **`required`** and `VREC-MOK-013` now exists,
  bound to `41c20ca`, recording `REQ-MOK-047` on assessment 1 and `REQ-MOK-049` on the automated cases and
  **disclosing `REQ-MOK-048` as unverified**. It is a **`ready` candidate and not `verified`**:
  `DECISION_RIGHTS.md` reserves that transition to the accountable assurance owner, and the harness reports
  the outstanding act as `decision_required -> review-assurance-decision`.

  > **Later fact, 2026-08-20 — this bullet is discharged, and by an act outside this pack's authorization.**
  > The assurance owner took the decision: `VREC-MOK-013` is `verified`, `REQ-MOK-048` is verified on the
  > automated cases accepted in place of the primary evidence, and the `decision_required` queue is empty. The
  > bullet is kept as written because it is a true statement about what this pack contained when it was
  > assembled, and `assurance-decision.md` — added to this directory afterwards, and not to the record's
  > `evidence_paths` — is where the decision lives.
- **No commit hash of this work order's own commit.** A record cannot contain the hash of the commit that
  introduces it. `VREC-MOK-013` is written after the commit it binds for the same reason.
- **No status transition, approval, assessment or release act taken by the agent.** All twenty-two decisions
  were the repository owner's and are attributed in `closing-review.md`; assessment 2 is unauthored because
  no admissible person has looked yet.
- **No correction to `evidence/WO-MOK-005/frames.txt` and no edit to `VREC-MOK-005`.** That capture is a true
  record of what the instrument drew on the day it was taken, and the verification record it feeds is stale
  for reasons that are out of this work order's scope. `log-height.md` says so under *What was not done*.
- **No assessment material directory.** `WO-MOK-012` needed one because its evidence *was* the captures a
  person judged. Here `frames.txt` and `gauge-resolution.txt` serve that role directly and are named as the
  material in `manual-assessment.md`. The two `discoverability-frame*.txt` files sit at the top level for the
  same reason: they are extracts of `frames.txt`, not a separate body of material.
- **Nothing about the eight `W-HEX-003` reassessments, the `W-HEX-001`s — now four, this work order's own
  among them — `WO-MOK-008`'s draft disposition, `VREC-MOK-005`'s staleness, `VER-MOK-005`'s manual
  assessment 7, or `ROADMAP.md`'s Phase 2 claim.** All out of scope, all still open. `validation.txt`
  attributes the warnings; it resolves none.
- **No renumbering on either side of either identifier collision.** `identifier-collision.md` measures the
  second one and decides nothing; decision 3's rule sends both to the merge.

  > **Later fact, 2026-08-20.** Still no renumbering, and now none owed from this side: this chain reached
  > `master` first in **both** collisions, so decision 3's rule puts the renumbering on the two other
  > branches. That is an outcome of merge order, not an act taken here. `identifier-collision.md`'s *What the
  > merge determined* measures it — 9 conflicts against `master` for the first collision's branch, 8 for the
  > second's — and stop condition 8 did not fire, because its antecedent was *"if this branch is the second of
  > the two to reach `master`"*.
- **No push, pull request, tag or release.** `WO-MOK-013`'s *Out of scope* puts all four outside it, and none
  was authorized in this work.

  > **Later fact, 2026-08-20.** The owner authorized publication separately and the branch reached `master`
  > through pull request [#32](https://github.com/mmzen/Mokiterions/pull/32) at `798e5d5`. **Those are the
  > owner's acts and not this work order's discharge**, which is why the bullet stands as written; still no tag
  > and no release, and nothing here authorizes either. The merge changed no source file — `git diff --stat
  > 41c20ca 798e5d5 -- . ':(exclude)docs'` is empty — so every capture in this pack describes `master`'s code
  > as well as the branch's, while the record bound to the merge commit remains owed.
