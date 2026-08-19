# The accountable assurance decision on VREC-MOK-008

Measured 2026-08-19, on the branch `feature/release-ci`, immediately after the decision was given. This file
records a decision the implementation agent did not make and could not make.

## What was decided, and by whom

The accountable assurance owner reviewed the retained evidence and directed the transition of
`docs/engineering/simulation/verification-records/VREC-MOK-008.md` from `ready` to **`verified`**. The
instruction was *"I approve, you can transition the verification record, and push"*.

`docs/engineering/DECISION_RIGHTS.md:14` reserves this transition: *"Only accountable assurance and release
owners may transition those records to `verified` or `released`."* `docs/engineering/WORKFLOW.md` line 20 adds
that the VREC *"moves separately from `ready` to `verified` through an accountable human decision"*. No harness
command performs it, and none was asked to — `capture-verification` emits `ready` and stops, which
`commit-binding.md` and the record's own Authority section both state.

**The decision was taken with the record's five stated limitations in front of the owner**, because the record
puts them before its verdict rather than after it: the release process has never been run, so fifteen of
`VER-MOK-008`'s 65 rows are *not performed*; M2 cannot be closed by this work order's author at all;
`candidate-conformance.md`'s "before" column is a reconstruction rather than a diff; one declared file,
`scripts/test_check_release_authorization.py`, carries a recorded but unfixed defect; and the record's own
snapshot field binds the checkout directory's basename as well as the commit. The record is verified *with*
those terms, not despite them, and it keeps the question it was asked beside the answer.

## What the transition changed

Exactly one field:

```diff
-status = "ready"
+status = "verified"
```

Every provenance field is as `capture-verification` produced it — `commit`, `git_object_format`,
`worktree_state`, `verified_at`, `artifact_snapshot_sha256`, all eighteen `evidence_paths`, and both relations.
None was recomputed at transition time, and the diff of the transition commit is the check.

That matters more than it looks. `verified_at` is the capture timestamp and `commit` is the candidate commit
`d35f8172a0f91049aa2719bc34ca9dd7584f4380`; a record whose provenance were refreshed on transition would bind
the tree it was *approved* in rather than the tree that was *verified*, and every later release check compares
complete hashes. `snapshot-reproducibility.md` is what a reader uses to recompute the snapshot, and it stays
correct only because nothing here touched it.

Four sections of the record's prose were amended in the same commit, and none of them asserts anything new:
the question put to the assurance owner now records that it was answered, the Authority section records who
answered it and why the generated preamble was not touched, and the `prepare-release` limitation records that
its blocking input now exists. The questions are kept rather than deleted.

**The command's own preamble still reads as of `ready`, deliberately.** `capture-verification` writes two
sentences calling the file a ready record and saying an assurance owner must transition it, and those were left
exactly as it wrote them. Editing them by hand would buy a self-consistent file at the cost of the property
that makes the provenance checkable — and the check was run rather than asserted. In a fresh clone named
`Mokiterions` at candidate commit `d35f817`, `capture-verification` with the record's own arguments emits 25
lines, of which **23 are byte-identical to the record's first 25**:

```text
fresh output bytes: 2271 | committed prefix bytes: 2274
fresh lines: 25 | committed prefix lines: 25
differing lines: 2
  line 5   fresh: status = "ready"                        committed: status = "verified"
  line 12  fresh: verified_at = "2026-08-19T16:07:03Z"    committed: verified_at = "2026-08-19T14:11:54Z"
```

`status` is the transition. `verified_at` is a wall-clock stamp that **no re-run can reproduce by
construction**, which is the honest limit of this check and the reason the field was not refreshed at
transition time. Everything a later release decision compares does reproduce: `commit`, `worktree_state`,
`git_object_format`, all eighteen `evidence_paths`, both relations, and `artifact_snapshot_sha256` —
`0eeb2e47…` again, from a clone made after the transition, which is `snapshot-reproducibility.md`'s claim
measured a second time and by a different route. A reader who takes the preamble for a contradiction should
read it as a timestamp.

## What the transition did not change

**`WO-MOK-009` stays `implemented`.** `WORKFLOW.md` line 16 permits work-order status `verified` only *"when an
eligible commit-bound VREC explicitly covers that work and configured provenance requires it"*, and line 20
states that *"work-order status never substitutes for either record."* The repository's own precedent is
uniform: `WO-MOK-001` through `WO-MOK-007` are all `implemented` beside `verified` records, and `master`'s
pull request #19 set `WO-MOK-007` to `implemented` in the same change that verified `VREC-MOK-007`. The
instruction was to transition the verification record, and that is what was transitioned.

**No other artifact moved.** `WO-MOK-008` is still `draft`. No release contract and no release record exists in
any revision. No tag was created. Pull request #20 is still open and unmerged.

**No measurement in this directory moved.** A status transition edits a field; it does not re-run a scenario.
The 65-row totals, the 70-test suite, the eleven static checks and the toolchain evidence all stand as
captured, and every "not performed" row is still not performed.

## The measured before and after

`python -m se_harness inspect .` from the pinned `se-harness==0.4.0` wheel, at three states of the same branch.
This is the one local command that reports whose decision is outstanding, and `WORKFLOW.md` line 18 is why:
inspection reports implemented `required` work without active VREC coverage as assurance follow-up.

| State | `Decision required` | `Assurance pending` | Graph |
| --- | --- | --- | --- |
| `d35f817` — no record yet | 0 | **1 — `WO-MOK-009 [implemented] prepare-commit-bound-verification`** | 79 artifacts, 238 relations, 16 findings |
| `3d55c2a` — record `ready` | **1 — `VREC-MOK-008 [ready] assurance-review`** | 0 | 80 artifacts, 240 relations, 17 findings |
| this commit — record `verified` | **0 — none** | 0 | 80 artifacts, 240 relations, 17 findings |

Two readings, and the second is the one that matters.

**Preparing the record closed the assurance-follow-up row; verifying it closed the decision row.** They are
different rows for different owners. At `3d55c2a` the harness named the outstanding act and whose it was:

```text
Decision required (1):
- VREC-MOK-008 [ready] assurance-review: Verification candidate for WO-MOK-009
Suggested next steps (12):
- decision_required -> review-assurance-decision (assurance-owner): Review retained evidence and
  record or withhold the accountable verification decision. [VREC-MOK-008]
```

Both lines are gone at this commit, and *Suggested next steps* falls from 12 to 11. Nothing else in the
inspection differs — same 80 artifacts, same 240 relations, same 17 findings, same severity split of error 0,
warning 10, info 7, and `I-REV-001` already counted `VREC-MOK-008` among its seven observations while it was
still `ready`.

**No check distinguishes a verified record from a ready one on the evidence.** `validate`, `doctor` and review
`preflight` return the same verdicts either side of the transition, and `ACTIVE_COVERAGE_STATUSES` admits both.
`inspect` is the only command whose output moves, and it moves because the decision was *recorded*, not because
anything was *proved*. That is the honest shape of this act: the assurance owner's judgement is the whole of
what changed, which is exactly why `DECISION_RIGHTS.md` reserves it to a person.

## Gates on the transitioned tree

Re-run from the pinned `0.4.0` wheel after the edit, in the working tree:

| Gate | Result |
| --- | --- |
| `python scripts/validate_engineering_artifacts.py --root .` | exit 0 — PASS, 80 artifacts, 0 errors, 0 warnings, all four planes E0/W0 |
| `python -m se_harness doctor .` | exit 0 — 81 verdict lines, 81 PASS, 0 WARN, 0 FAIL |
| `python -m se_harness preflight . --work-order WO-MOK-009 --phase review` | exit 0 — PASS, `WO-MOK-009 (implemented)`, commit-bound verification `required` |
| `python -m se_harness inspect .` | exit 0 — `Decision required (0)`, `Assurance pending (0)`, `Active work (0)`, `Definitions pending (1): WO-MOK-008 [draft]` |
| `python scripts/check_release_authorization.py --root . --tag v0.1.0` | **exit 1 — REFUSED**, unchanged |

The last row is the point of this whole file. The gate's refusal is verbatim:

```text
REFUSED: v0.1.0 is not a tag in this repository. The release owner creates the tag; this workflow never does.
```

**A verified verification record is an input to a release decision, not a release decision.** `a5-refusal-ladder.md`
records rule 4's rungs in order, and verifying this record climbed none of them: there is no tag, no release
contract and no release record, and the ladder's last rung — a release record transitioned to `released` — is
reserved to the release owner. Nothing about this transition brings a release closer than one artifact.

## What is now unblocked, and what still is not

`release-artifact-types.md` records that `prepare-release` could not be exercised because it requires a
verified verification record as input and none existed. **That blocker is gone and the row is still not
discharged.** `prepare-release` also requires an `approved` release contract, and `REL-MOK-001` does not exist
in any revision; creating and approving it is the release owner's act. So `VER-MOK-008`'s fifteen
*not performed* rows are unchanged in number, and the first run remains the only thing that closes V1–V6 and
P1–P3.

## Why this file is not among the record's evidence paths

`VREC-MOK-008` binds eighteen evidence files, and this is not one of them. It cannot be: the record binds
candidate commit `d35f817`, this file postdates it, and a record cannot cite evidence about its own
transition without the same self-reference `WORKFLOW.md` line 22 forbids for commit hashes. The eighteen paths
are the evidence the decision was taken *on*; this file is the record of the decision itself, which is why it
sits beside them rather than inside the set. `README.md` indexes it on those terms.

**Three of the eighteen were edited in the transition commit, and a reader should know which.** `README.md`,
`completion-summary.md` and `approval-and-transition.md` each asserted that `VREC-MOK-008` was `ready`, which
stopped being true; each now carries the later fact beside what it said rather than instead of it. The record
binds paths and not hashes, so their current text is newer than the text it verified —
`git show d35f817:<path>` reads the verified bytes. Nothing else among the eighteen is touched, and **no
measurement in any of them was re-derived or overwritten.** Beyond the status claims, the edits add the two
counts this file changes — the directory going from eighteen files to nineteen, and the validator reading 80 at
the branch tip against the 79 captured mid-work — both stated as additions with their earlier figures kept and
dated. A transition that quietly restated a measurement would be the thing to worry about; this one restates
what is *counted*, not what was *measured*.

This record takes the directory to nineteen files — eighteen records and the index — while the verification
record still binds eighteen. That gap is not drift; it is the sentence above, and it will hold for any
further record written after the candidate commit. `id-collision.md`'s closing count of eighteen was correct
when it was written.
