# The assurance decision of 2026-08-19, and the two transitions it carried

This file records what happened *after* the commit `VREC-MOK-010` binds: the accountable assurance owner's
verification decision, and the work-order transition the same instruction authorized. It is the record of those acts.
What they were taken on is in `VREC-MOK-010` itself, in `closing-review.md` for the twelve decisions that preceded
them, and in `manual-assessment.md` and `amendment-approvals.md` for the measurements behind those.

**It is deliberately not among `VREC-MOK-010`'s `evidence_paths`.** That list is the capture's, taken against commit
`1a937a1a9a3ff24c23e45946ad023bde95f83d02`, and this file postdates it. Adding it would put a file describing a later
decision inside the evidence set that decision was taken about, which is the self-reference the whole chain is
arranged to avoid. The record points at this file in prose instead. It is also why the packet's retention list, which
`VER-MOK-010` fixes at fifteen bullets, is unchanged: this file discharges none of them and claims none.

## The instruction

Verbatim, on 2026-08-19: *"you can push on existing branch, swicth VREC-MOK-010 to verified, commit and push again
(implying transitioning WO-MOK-10)"*.

Three acts are authorized in it, and each was performed once: the push of the two already-committed governance
commits, the `ready` → `verified` transition of `VREC-MOK-010`, and — by the parenthetical, which is the owner's own
words and not the agent's reading of them — the `in_progress` → `implemented` transition of `WO-MOK-010`. **The
implementation agent recorded both transitions and decided neither.** `DECISION_RIGHTS.md` reserves the
`ready` → `verified` transition to the accountable assurance owner and states that record preparation never makes it;
`WORKFLOW.md` states that the verification record moves separately through an accountable human decision and that
work-order status never substitutes for it.

**Who acted in which role.** The repository owner holds all three accountable roles and acted as assurance owner for
the verification decision and as engineering owner for the work-order transition. That is the same division
`closing-review.md` records for the twelve decisions of the same day, and the reason each act is named separately
here: one instruction can authorize two acts without either becoming an approval of the other.

## The order, and why it is that order

1. **`c03e4d8`** — the re-captured `ready` record, committed before the instruction and pushed under it. A `ready`
   record may be re-captured; a `verified` one may not. Re-capturing first and deciding second is what kept that
   option open, and it is why the record's figures describe the commit it binds rather than the commit it sits in.
2. **`084b608`** — `WO-MOK-010` from `in_progress` to `implemented`, with a new *Transition to `implemented`*
   subsection recording what was implemented, the gates, the lineage, and what the status does not do.
3. **The commit carrying this file** — `VREC-MOK-010` from `ready` to `verified`, plus this record and the edits named
   below.

The work order was moved *before* the verification record, so that the record could state the work order's final
status instead of going stale one commit later. That is `master`'s own precedent: `335f8c8` set `WO-MOK-007` to
`implemented` and `a7ddf74` transitioned `VREC-MOK-007` five seconds later, and its message says the ordering was the
whole reason — `VREC-MOK-005` and `VREC-MOK-006` each still open with "is `in_progress`" for work orders that are now
`implemented`, and were correctly left alone, because a bound verified record is not re-edited to track state that
moved after it.

## What changed in `VREC-MOK-010`

`status` is the only front-matter field that changed. `commit`, `git_object_format`, `worktree_state`, `verified_at`,
`artifact_snapshot_sha256`, all 59 `evidence_paths`, both relations and the `title` — which still reads *candidate* —
are exactly as the capture produced them. In particular the snapshot digest still names the 80-artifact graph holding
the *retired* form of that file: a decision does not re-measure provenance, and the record says so.

The body was corrected in five places rather than left to go stale, which is available only at the moment of
transition: the heading and opening record the decision and quote what the file said as a candidate; the claims
section opens with the work order's status at the candidate commit and the move afterwards; the harness rows are left
as measured with the move from 12 warnings to 13 explained beneath them; *What must happen before this record can be
verified* becomes *what had to happen, and what still stands*, with item 1 discharged and item 2 not; and *Scope of
the transition being requested* becomes *Scope of the transition taken*, stating what the status records in the same
words the candidate used to describe what would be accepted.

## What the decision accepted

**The retained evidence for `WO-MOK-010` at commit `1a937a1`, with one row of `VER-MOK-010`'s requirement-to-evidence
matrix unsatisfied and the fifteen disclosures read.** The unsatisfied row is the `VREC-MOK-005` gate: that chain's
six amendment rows and seven manual assessments, eleven provisions by its own count, which the owner's override of
2026-08-19 deferred and the closing review let stand with an obligation attached — a work order of its own, completing
before the next release record. **The obligation is recorded, not discharged**, no such work order exists, and this
decision does not create one.

**Nothing outstanding is retired by either status.** No assessment is performed, no ratification supplied, no
amendment in the `WO-MOK-005` layer approved, and `WO-MOK-005` is not transitioned. Verification is not release: no
release record exists, and a verified verification record is an input to a release decision rather than one. PR #17
**remains a draft**; taking it out of draft, approving a review and merging are the repository owner's acts and none
was performed.

## Measured before and after

Run at each step on a clean worktree, with `scripts/validate_engineering_artifacts.py`,
`scripts/check_engineering_harness.sh` and `scripts/inspect_engineering_artifacts.py`.

| Figure | Before, at `c03e4d8` | After `WO-MOK-010` → `implemented` | After `VREC-MOK-010` → `verified` |
|---|---|---|---|
| Validator | PASS, 80 artifacts, 0 errors, 0 warnings, four planes clean | unchanged | unchanged |
| Harness graph | 80 artifacts, 248 relations, 0 errors | unchanged | unchanged |
| Dashboard warnings | 12 | **13** | 13 |
| Inspector findings | 20 | **21** | 21 |
| Inspector severity | error 0, warning 12, info 8 | error 0, warning **13**, info 8 | error 0, warning 13, info 8 |
| `decision_required` | 1 — `VREC-MOK-010` `[ready]` | 1 | **0** |
| Active work | 1 — `WO-MOK-010` `[in_progress]` | **0** | 0 |
| Assurance pending | 0 | 0 | 0 |
| Suggested next steps | 14 | 14 | **13** |

Three figures moved and each has one cause:

- **The thirteenth warning is `W-HEX-001` against `WO-MOK-010`** — "retain evidence keyed to the implemented work
  order". It is the warning every implemented work order in this repository already carries: evidence discovery keys
  on file *names* beginning with the work-order identifier, and this chain retains its evidence in a directory named
  `WO-MOK-010/` instead, so the observation count goes from seven to eight and now lists `WO-MOK-001` through
  `WO-MOK-007` and `WO-MOK-010`. Nothing was renamed to silence it: renaming fifty-nine retained files would
  invalidate every recorded digest in the packet, which is a far worse trade than carrying the warning `master`
  carries seven times over.
- **Active work drops to 0** because the work the inspector was recommending be continued is complete.
- **`decision_required` drops to 0 and the steps with it** because the decision it was asking for was taken. That
  queue and *Assurance pending* are different rows for different owners; the second was already 0 once the record was
  prepared.

`I-REV-001` still reports eight observations, `VREC-MOK-010` among them: the rule compares a record's declared
candidate commit against the observed checkout, and this record declares `1a937a1` while sitting in a later commit by
construction. `VREC-MOK-010` explains that in full.

## One retained capture no longer reproduces, and it is not edited

`analysis/amendments.py` — oracle 5 — checks as its first control that every artifact in the chain is `approved` and
that **the work order is `in_progress`**. That was true of the commit it was captured against and is no longer true.
Re-run after the transition it reports exactly one control failing:

```
| `WO-MOK-010` | `implemented` | `in_progress` | 2026-08-19 | **NO** |
...
RESULT: FAIL — 1 finding(s) above.
```

Everything else in the regenerated file is byte-identical to the retained one, including all four ratification checks
and the other sixteen controls on the checks themselves. **The retained `amendment-approvals.md` is not edited to
track a status that moved after it was taken** — a capture corrected into agreement is not a capture of anything, which
is the discipline this packet applies to its own streams. Note for anyone re-running it: the script writes its output
over the retained file in place, so a re-run after the transition rewrites bound evidence. It did here, and the file
was restored from git; the retained capture again reads `in_progress` / `yes`, as taken at `1a937a1`.

## Four bound files were edited to stop asserting the earlier state

Each keeps what it said, with the later fact beside it rather than in place of it: `README.md`,
`completion-summary.md`, `manual-assessment.md` and `closing-review.md` each stated that the record was a `ready`
candidate and the work order `in_progress`, which was true when written and true of the commit they were captured
against. None of their measurements, digests, captures or judgements is touched; what changed is the sentence about
lifecycle state, and in each case the earlier sentence is still readable. `requirement-to-test-mapping.md` needed no
edit: it states the matrix row by row and the row it reports unsatisfied is unsatisfied still.
