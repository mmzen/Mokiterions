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

**What its status is.** `WO-MOK-013` is **`approved`**. Implementation landed on 2026-08-20 on the
repository owner's direction, and **the transition to `in_progress` or `implemented` is an owner act that was
not taken here.** `WORKFLOW.md` makes a status change a record of authority rather than a confidence estimate,
and start preflight accepts `approved`, so nothing is blocked by leaving it there. The work order's *Lifecycle*
section carries the same statement.

**What it does not close.** Both of `VER-MOK-013`'s manual assessments are **outstanding with no author**, and
one of them has **no admissible assessor available**: the contract asks for a person who has not read
`SPEC-MOK-003` rule 7, and the only assessor has ratified six amendments to that document. `manual-assessment.md`
sets out three options and takes none. Because `VER-MOK-013` classes both as primary evidence for their
requirements, `VREC-MOK-013` cannot record any of the three requirements as verified while they stand.

**Decision 1 is the one trade in this work order, and it is a trade.** The roster's three-line entry needs
36 interior rows, which the reference viewport has only if the log stays at six. The product owner chose to
hold the log at six rows and keep `REQ-MOK-020`'s twelve entries intact rather than amend the requirement —
and then stood by that choice when the cost figure it had been put on turned out to be wrong. The log carries
**four** recent events where it carried eight, not six where it carried ten. `log-height.md` measures both
forms out of rendered buffers and exhibits the counterfactual; `closing-review.md` records the correction as
the agent's error in how the option was framed.

## The files

| File | What it establishes |
|---|---|
| `README.md` | This index. Every file in the pack, what it establishes, and what is deliberately absent. |
| `completion-summary.md` | The eight-item completion report `WO-MOK-013`'s *Completion report format* specifies: decision 1 with both corrections to how it was put, the three requirements against their before figures, every amendment with its ratifying act, the change surface, the test and interface counts, the validator and inspector results, both manual assessments, and nine findings reported rather than fixed. |
| `closing-review.md` | The eighteen decisions this work order rests on, each with the role that took it, plus the choices the agent made inside the authorized envelope, the three locations the work order's enumeration missed, and the four stop conditions that could have fired and did not. |
| `log-height.md` | Decision 1 measured in both directions: six rows against ten, four log records against eight, twelve roster entries against ten, and that `12 × 3 = 36` fits with no slack either way. Names what was **not** corrected. |
| `gauge-resolution.md` | `REQ-MOK-047` measured: the gauge at 13 cells rather than 2, 14 renderable states rather than 3, and a ten-point step moving the fill at **91 of 91** values where it moved at 11. Reads `gauge-resolution.txt` against the `WO-MOK-012` before form. |
| `test-census.md` | Rule 11's test totals reconciled arrival by arrival, the static census validated target by target against the executed run, the one rename with both names, and every `VER-MOK-013` case mapped to the test that discharges it. |
| `manual-assessment.md` | Both contracted manual assessments, **outstanding, author none**, with the corrected procedure, the prepared material, and the assessor constraint that neither the agent nor the owner can satisfy for the second. |
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

Twenty-two files, 2.2 MB, of which the two retained event streams are 1.97 MB.

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
here that records an obligation rather than a result.

## What is not here

- **No verification record.** `VER-MOK-013` classifies `commit_bound_verification` as **`required`**, and
  `VREC-MOK-013` does not exist. It is captured against the commit that carries this implementation, by a
  separate accountable act, and it cannot record `REQ-MOK-047` or `REQ-MOK-048` as verified while the two
  manual assessments stand outstanding.
- **No commit hash of this work order's own commit.** A record cannot contain the hash of the commit that
  introduces it.
- **No status transition, approval, assessment or release act taken by the agent.** All eighteen decisions
  were the repository owner's and are attributed in `closing-review.md`; the two manual assessments are
  unauthored because no person looked.
- **No correction to `evidence/WO-MOK-005/frames.txt` and no edit to `VREC-MOK-005`.** That capture is a true
  record of what the instrument drew on the day it was taken, and the verification record it feeds is stale
  for reasons that are out of this work order's scope. `log-height.md` says so under *What was not done*.
- **No assessment material directory.** `WO-MOK-012` needed one because its evidence *was* the captures a
  person judged. Here `frames.txt` and `gauge-resolution.txt` serve that role directly and are named as the
  material in `manual-assessment.md`.
- **Nothing about the eight `W-HEX-003` reassessments, the three `W-HEX-001`s, `WO-MOK-008`'s draft
  disposition, `VREC-MOK-005`'s staleness, `VER-MOK-005`'s manual assessment 7, or `ROADMAP.md`'s Phase 2
  claim.** All out of scope, all still open. `validation.txt` attributes the warnings; it resolves none.
- **No push, pull request, tag or release.** `WO-MOK-013`'s *Out of scope* puts all four outside it, and none
  was authorized in this work.
