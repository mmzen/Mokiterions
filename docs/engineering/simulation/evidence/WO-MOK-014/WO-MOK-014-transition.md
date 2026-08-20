# WO-MOK-014 — the transition to `implemented`, and what it makes stale

**Work order** `WO-MOK-014` · **Branch** `governance/adr-mok-006-third-party-crates` · **Implementation commit**
`84b21b9cdc8566cf2bde45b47dae415944d19dca` · **Date** 2026-08-20

This file records one governed act: `WO-MOK-014`'s status moving from `in_progress` to `implemented`. It is separate
from the rest of this packet because the rest of the packet was written before the act and says so, and because a
status change that rewrote the statements it invalidated would leave no trace that they had ever been true.

The hash of the commit that carries this transition is not in this file. `WORKFLOW.md` states that a record cannot
contain the hash of its own commit, and this file travels in the transition commit.

## The act

Status moved from `in_progress` to `implemented` on 2026-08-20 by the repository owner acting as accountable
**engineering owner**, in the instruction:

> you can: (1) commit and push, (2), mark WO-POK-013 as implemented

The identifier is written `WO-POK-013` in the instruction and is quoted as given. It is read as `WO-MOK-014` because
this repository has no `POK` artifact family, the instruction was given inside this work order's own implementation,
and its first clause authorized the commit that carries this work order's change. The reading is recorded rather than
applied silently, because the correction is the agent's and not the owner's.

Two acts, in two commits, in the order the instruction numbers them:

| Commit | What it carries | Status it leaves behind |
|---|---|---|
| `84b21b9cdc8566cf2bde45b47dae415944d19dca` | The whole change — four new artifacts, twelve amended, the checking program, the two workflows, and nineteen evidence files | `in_progress` |
| this one | The status change, its dated *Lifecycle* subsection, and this file | `implemented` |

**Why two and not one.** `WO-MOK-011` is the precedent: its transition was its own commit, `gov: set WO-MOK-011 to
implemented`, taken when the owner said so. The reason is not tidiness. `WO-MOK-014-completion-summary.md` and
`WO-MOK-014-harness.txt` were both written before this instruction existed and both state that the transition has not
happened and is not authorized. Folding the transition into the implementation commit would have made those statements
false in the commit that introduced them, and there would be no way to tell that from the tree.

## What moved in the tree

Two edits to one governed artifact, and this file:

| File | Edit |
|---|---|
| `work-orders/WO-MOK-014.md` front matter | `status = "in_progress"` → `status = "implemented"`. `updated` already reads `2026-08-20` and is unchanged |
| `work-orders/WO-MOK-014.md` *Lifecycle* | A new `### Transition to implemented` subsection at line 118, recording the instruction, the role, the date, the identifier reading, the two-commit shape, and what the transition does not carry |
| `evidence/WO-MOK-014/WO-MOK-014-transition.md` | This file |

No other governed artifact is touched. No specification, architecture document, requirement, verification contract or
ADR changes. No Rust source, no manifest, no lockfile, no workflow and no script changes, so nothing this transition
does can alter a measured figure in this packet — and the figures below are the check on that claim rather than the
assertion of it.

The subsection sits below the corrections note dated the same day rather than beside *Transition to `in_progress`*,
because it happened after that note and the section reads in the order the acts occurred.

## The harness at this tree

Measured after the edits above and after this file was added, from the repository root:

    $ python scripts/validate_engineering_artifacts.py --root .
    PASS | Artifacts: 106 | Errors: 0 | Warnings: 0 | exit 0
    Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0

    $ python scripts/generate_harness_dashboard.py --root .
    PASS | Artifacts: 106 | Relations: 357 | Errors: 0 | Warnings: 7 | exit 0
    Snapshot: 7dacb4d581429010ad5bd3c8bcbd3a11aed32c9f4836ff2b173ec2dee569724e

    $ python scripts/inspect_engineering_artifacts.py --root .
    106 artifacts | 357 relations | 19 findings | error 0 | warning 7 | info 12 | exit 0

    $ <pinned 0.4.0 venv>/python -m se_harness doctor .
    exit 0 | 81 PASS | 0 FAIL | 0 WARN

Every count is identical to `84b21b9`: same artifacts, same relations, same findings in every severity, same doctor
verdict. **One figure changes, and only one: the dashboard snapshot.**

| Snapshot | Tree |
|---|---|
| `2d709365ad39ad1ff367a315265a26b4f557cdebbcc05a70520065cfca04c530` | `84b21b9`, the implementation commit |
| `3e5b93963cfc051d60f6239dc4a0755596f927543d9e4b414a807543d95ffbec` | The status change and the *Lifecycle* subsection alone, before this file existed |
| `7dacb4d581429010ad5bd3c8bcbd3a11aed32c9f4836ff2b173ec2dee569724e` | This tree, with this file added — the figure this commit carries |

The intermediate figure is recorded because it separates two causes that would otherwise be one number. The snapshot
covers governed artifact content **and the evidence index**, and the index holds *paths*: `discover_evidence` at
`scripts/generate_harness_dashboard.py:370` returns a map of work-order identifier to sorted repository-relative file
paths, and nothing in it reads a file's bytes. So `3e5b939…` is what the one-word status change did, and the step to
`7dacb4d…` is this file's *name* entering `WO-MOK-014`'s evidence list — twenty paths where there were nineteen.

The same reading says the citation correction recorded below cannot have moved either figure: it edits an existing
evidence file's content, and no evidence content reaches the snapshot. `7dacb4d…` was measured after that edit, which is
the check on it rather than the assertion.

Doctor was measured with the pinned 0.4.0 interpreter, which is the version `.github/workflows/engineering-harness.yml`
installs by exact pin. `WO-MOK-014-harness.txt` records why that distinction is part of the figure and records the
wrong reading a reader gets from the machine-wide 0.4.1 install.

## The queue move, and the rule that causes it

    $ python scripts/inspect_engineering_artifacts.py --root .

    Active work (0):
    - none

    Assurance pending (1):
    - WO-MOK-014 [implemented] prepare-commit-bound-verification: Replace the engine's empty-dependency rule with a
      declared-set comparison, and check it at pull-request time and at release
      (docs/engineering/simulation/work-orders/WO-MOK-014.md)

    Decision required (0): none
    Definitions pending (1): WO-MOK-008 [draft]

`WO-MOK-014` left `active_work` and entered `assurance_pending`. Both halves follow from `status` alone, and the rule
is worth stating because the second half is conditional on a field this work order declares:

- `active_work` holds a work order whose status is `approved` or `in_progress` — nothing else. `implemented` is not in
  that set, so the move out is unconditional.
- `assurance_pending` holds a work order that is `implemented` **and** whose `[assurance]` block is valid with
  `commit_bound_verification = "required"` **and** which no verification record at `ready`, `verified` or `released`
  already covers through a declared `verifies_work_order` relation. `WO-MOK-014` classifies commit-bound verification
  `required`, and `VREC-MOK-014` does not exist, so it satisfies all three.

That last clause is what the queue is for: it names work that is done and unverified. `WO-MOK-014` leaves the queue
when `VREC-MOK-014` exists at `ready` or beyond — which is an act this transition does not perform and this work order
cannot perform for itself.

`WO-MOK-008` stays in `definition_pending` in this tree as it did in the baseline. Nothing here touches it.

## `W-HEX-001` became a live check at this transition, and it holds

`W-HEX-001` fires on a work order whose status is `implemented`, `verified` or `released` and which has **no evidence
document keyed to its own identifier**. Until this commit `WO-MOK-014` was `in_progress`, so the rule could not fire on
it whatever its evidence was named. It can now, and it does not:

    [warning] W-HEX-001 WO-MOK-010
    [warning] W-HEX-001 WO-MOK-011

Two observations, the same two as the baseline and as `84b21b9`, and `WO-MOK-014` is not among them.

This is the first commit at which that means anything. `WO-MOK-014-harness.txt` records the naming rule — every file in
this packet is `WO-MOK-014-*`, because `discover_evidence` in `scripts/generate_harness_dashboard.py` matches the
work-order identifier against each *file's* name and not its directory's — and records that `W-HEX-001` did not name
`WO-MOK-014` in the candidate tree. That was true and it was not yet a test. Had the packet been named the way
`evidence/WO-MOK-010/` and `evidence/WO-MOK-011/` are, this transition would have added a third observation to the list
above, and the warning count would have gone from 7 to 8.

## Statements in this packet that this transition affects

Five go stale and two stay true only of the tree they measured. Each was accurate at the commit it was written in, and
none is rewritten. They are enumerated here on the precedent of `VREC-MOK-005:433` — *"Four statements go stale, and
each was accurate when written"* — whose record lists what a later change put out of date rather than editing it in
place.

| Where | What it says | Standing now |
|---|---|---|
| `WO-MOK-014-completion-summary.md:11` | *"Commits — **none.** Nothing is committed, pushed, tagged or opened as a pull request"* | Stale in two of four. The change is committed as `84b21b9` and the branch is pushed to `origin`, both on the same instruction that authorized this transition. **No tag and no pull request exist**, and neither is authorized |
| `WO-MOK-014-completion-summary.md:12` | *"Work-order status — `in_progress` … **Not** transitioned to `implemented`"* | Stale. Superseded by this file and by the *Lifecycle* subsection at `WO-MOK-014.md:118` |
| `WO-MOK-014-completion-summary.md:54` | *"Evidence — 19 files under `evidence/WO-MOK-014/`"* | Stale by one. This file is the twentieth, and it is named `WO-MOK-014-*` like the other nineteen |
| `WO-MOK-014-completion-summary.md:346` | *"**Not authorized and not performed:** commit, push, tag, pull request, or transitioning `WO-MOK-014` to `implemented`"* | Stale in three of five: commit, push and the transition are authorized and performed. **Tag and pull request remain neither authorized nor performed** |
| `WO-MOK-014-harness.txt:148-154` | The Queues block, and *"`WO-MOK-014` is in `active_work` and not in `assurance_pending` because its status is `approved`. It is not moved to `implemented` in this tree"* | Stale, and it carried one error when written: the candidate tree's status was `in_progress`, not `approved`. The queue placement was the same under either, because `active_work` holds both statuses, so the error changed no figure. Corrected here rather than in place |
| `WO-MOK-014-harness.txt:53` and `WO-MOK-014-gates.txt:465` | Snapshot `2d70936…` | Still true of `84b21b9`, which is the tree those files measured. The snapshot at this tree is `3e5b939…`, above |
| `WO-MOK-014-gates.txt:484, 497` | The inspector's `active_work → continue-authorized-work [WO-MOK-014]` line, at `[in_progress]` | Still true of `84b21b9`. Superseded by the `assurance_pending` block above |

**One citation re-derived rather than left broken.** `WO-MOK-014-manual-assessment.md:52` cited manual assessment 4's
text at `WO-MOK-014:303`, which was correct at `84b21b9`. The *Lifecycle* subsection added here inserts 26 lines above
it, so the same sentence is now at `WO-MOK-014:329` and the citation is updated to it. A line number is a pointer and
not a claim, and a pointer that no longer resolves is a defect rather than a record of anything; both numbers are given
here so a reader at either commit can follow it. Every other line citation into `WO-MOK-014.md` in this packet points
above the insertion — `WO-MOK-014-amendments.md:108` cites `work-orders/WO-MOK-014.md:87` — and was re-derived and left
unchanged.

## What this transition does not do

`WORKFLOW.md` ends a work order's governance path at `implemented`, so this is the last transition `WO-MOK-014` takes
and the last act this work order's own authority reaches.

- **It is not a verification.** `commit_bound_verification` is classified `required`, `VREC-MOK-014` is not written, and
  a work order cannot approve the record that verifies it. `VER-MOK-014`'s five oracles, twelve acceptance scenarios and
  six manual assessments are evidenced in this packet; whether that evidence satisfies the contract is the assurance
  owner's reading, and `WO-MOK-014-manual-assessment.md` flags the one place where the contract's own wording makes that
  reading a judgement rather than a count.
- **It is not a release.** `RLS-MOK-001` is untouched.
- **No tag and no pull request.** The instruction names neither.
- **`VER-MOK-005` and `VER-MOK-008` stay owed.** The owner decided on 2026-08-20 to leave both for a separate change;
  `W-HEX-003` names them in this tree as it did in `84b21b9`, and this transition neither clears nor inherits them.
- **The four `OUTSTANDING` amendment rows from earlier work, and `VER-MOK-011`'s manual assessment 5, are untouched.**
  `WO-MOK-014-amendments.md` records the row-by-row check that establishes it, and nothing in this commit edits an
  amendment record.
