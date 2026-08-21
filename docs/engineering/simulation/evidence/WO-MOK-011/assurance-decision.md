# The assurance decision of 2026-08-20 on `VREC-MOK-011`

`VREC-MOK-011` moved from `ready` to `verified` on 2026-08-20, by the repository owner acting as
accountable assurance owner. This note records the decision, what it accepted, what it did **not**
retire, and the measured harness state either side of it.

**This file is not among `VREC-MOK-011`'s `evidence_paths`, deliberately.** It postdates the commit that
record binds, and a record's evidence set is the capture's rather than the decision's. It follows the
`VREC-MOK-007` precedent at `e1d6f6e` and the `VREC-MOK-010` precedent at `8c29830`, whose
`assurance-decision.md` is kept out of its record's paths for the same reason.

It takes no decision itself. It records one.

## The instruction

Verbatim, in the turn it was given:

> i validate the verification record VREC-MOK-011, you can transition it, commit and PR

`DECISION_RIGHTS.md` reserves the `ready -> verified` transition to the accountable assurance owner and
states that record preparation never makes it. The implementation agent recorded the decision and did not
take it.

**The instruction named one artifact and three acts.** It named `VREC-MOK-011`, and it authorized
transition, commit and pull request. It did not name `WO-MOK-011`, so the work order was not moved — the
contrast with the `VREC-MOK-010` instruction of 2026-08-19 is explicit, because that one carried the
parenthetical *"(implying transitioning WO-MOK-10)"* and this one carries no equivalent. Nothing is
approved here by implication.

## What the decision accepted

**The accountable assurance owner accepts the retained evidence for `WO-MOK-011` at commit `9ddcf83`,
with manual assessment 5 of `VER-MOK-011`'s seven unperformed.**

That is stated in the words the `ready` record used to describe what accepting it would mean, so the
acceptance cannot be read wider than what was put in front of the owner. The record named two paths out
of its own `ready` state — the assessment being made, or *"the owner decides on the record and states what
is being accepted in its place"* — and the instruction took the second.

**Assessment 5 was not performed, and is not recorded as performed.** It asks whether
`baseline/projection.py`'s anchored pattern deletes the field this change adds and nothing else. The
instruction validated the record; it stated no judgement on the projection. `manual-assessment.md` still
reads **OUTSTANDING** against assessment 5 and was not edited to agree with the transition, and
`VER-MOK-011` is therefore still **not satisfied**, on one count and one only.

## What the decision does not retire

1. **The merged tree is not verified.** `VREC-MOK-011` binds `9ddcf83`, whose tree is not `master`'s.
   `merge/README.md` records that oracles 1 and 2, oracle 4's rendered buffers and the mutation control
   were **not** re-derived on the merge — oracle 3's census, oracle 5's two halves, `SPEC-MOK-004`'s rules
   9, 10 and 11, `render.rs`'s item counts, the declared gates and the harness state were. **A new
   verification record bound to the merge commit is still owed, and it is a new record rather than an edit
   of this one.** `master` carries this work unverified.
2. **The technical owner's reading of stop condition 1 is not taken.** Two inline call sites at the
   candidate commit, six plus one expected string after the merge. The reading is the same one and it is
   recorded either way, not decided here.
3. **`WO-MOK-011` stays `in_progress`.** Moving it to `implemented` is the engineering owner's act and
   was not instructed. The inspector still reports it under *Active work*.

   > **Later fact, added 2026-08-20.** It was instructed, in the next turn and as engineering owner: *"you
   > can also transition WO-MOK-011 as implemented and push it to same branch"*. The work order moved to
   > `implemented` in the commit after this decision's, and the reasoning above is why it took a second
   > instruction to do it. **That transition retires nothing on this list either** — `implemented` is not a
   > verification, and the four other items stand exactly as written. The *Measured before and after* table
   > below is the record transition's alone; the work-order transition's own figures are in `WO-MOK-011`'s
   > *Transition to `implemented`* subsection, which is where the reader should go for them.
4. **Nothing is released, tagged or published.** No release record binds this work. `RLS-MOK-001`
   released 0.1.0 from `755db72`, which does **not** contain `9ddcf83` — checked with `git merge-base
   --is-ancestor`, not assumed. A verified record is an input to a release decision, not one.
5. **No gate is re-measured.** Every figure in `VREC-MOK-011`'s gate table is the candidate commit's and
   stays there. `commit`, `verified_at`, `artifact_snapshot_sha256`, all 221 `evidence_paths`, both
   relations and the `title` — which still reads *"Verification candidate for WO-MOK-011"*, as
   `VREC-MOK-010`'s does after its own transition — are exactly as the capture produced them, so the
   snapshot digest still names the graph holding the `ready` form of the file. A decision does not
   re-measure provenance. `status` and `updated` are the only two front-matter fields that moved, and
   `updated` moved because the file did: the `VREC-MOK-010` transition changed `status` alone only
   because its `updated` already read the date the decision was taken.

## The merge preceded this decision and did not depend on it

Pull request [#22](https://github.com/mmzen/Mokiterions/pull/22) merged at `dec1b95` on 2026-08-19T21:11Z,
from `feature/phase-2-5-naming` into `master` after being retargeted from
`feature/phase-2-individuality` and taken out of draft — all on the owner's own instructions, and all
before this transition. `9ddcf83` is an ancestor of `master`.

**A merged pull request is not a verification, and this verification is not a merge.** The `ready` record
described the branch as local with a draft PR open against the parent, which was true when it was written;
both facts changed afterwards, and the record now says so beside its original words rather than in place
of them.

## Measured before and after

Both measured on this branch at `dec1b95`, the only difference being this record's `status` and `updated`
fields and this file's existence.

| Reading | Before | After |
|---|---|---|
| `validate_engineering_artifacts.py` | PASS — 102 artifacts, 0 errors, 0 warnings across four planes | **identical** |
| `generate_harness_dashboard.py` | PASS — 102 artifacts, 335 relations, 0 errors, 6 warnings | **identical but for the snapshot digest** |
| dashboard snapshot | `7e1eb31568647f82ed6a03df8e2b9cbc5a5c9e6a861d6bd6dc3c6ab92ded5b27` | `619e1e4b5ca5f37bc4554c9167ad769e7cfefeae92d5362e1f9e8995c49cbd5f` |
| `inspect_engineering_artifacts.py` findings | 18 — error 0, warning 6, info 12 | **identical** |
| Decision required | **1** — `VREC-MOK-011` `[ready]` assurance-review | **0 — none** |
| Definitions pending | 1 — `WO-MOK-008` `[draft]` | 1, unchanged |
| Active work | 1 — `WO-MOK-011` `[in_progress]` | 1, unchanged — **but see below** |
| Assurance pending | 0 | 0 |
| Suggested next steps | 9 | **8** |

**The transition answers exactly the signal it was asked for and changes nothing else.** The
`decision_required` queue empties from its one entry to none, *Suggested next steps* drops with it from 9
to 8, and no error count, no warning count and no finding total moves. The snapshot digest moves because
the artifact content moved.

> **A correction to the sentence above, made 2026-08-20 when the work-order transition was measured.** The
> snapshot digest is not purely a content digest, and describing it as one was imprecise. `build_snapshot`
> in `scripts/generate_harness_dashboard.py` puts `repository.name` — the checkout directory's name — and
> `repository.revision`, which is `git rev-parse HEAD`, into the hashed document alongside the artifacts,
> the relations and the findings. **The pair above is still a valid pair**, because both readings were
> taken in this clone with `HEAD` at `dec1b95` and the artifact content as the only moving input; measured
> after `965fe67` was committed, the same content hashes to
> `caae58cd29d3c9f189f71ff216e5f684b6e7bb84d886f68eb2c2c5057cb46f86` instead. So a snapshot digest is
> comparable only against another taken in the same clone at the same `HEAD` — which is what
> `merge/gates.txt` means when it says its digest "is not the digest of any commit and no record binds
> it". It does not hash artifact prose either, in the other direction: the body corrections made to
> `VREC-MOK-011` in the following commit leave the digest at
> `d9d4c7f90c49acde7d87dc01a73509699273630d0a5844a8b57a2baa1b7ea2b7` whether they are applied or not,
> because what is hashed is the normalized front matter, the relations and the findings. No figure in the
> table changes; only the explanation of the last row does.

> **Later fact, added 2026-08-20.** The *Active work* row is the record transition's alone. The
> work-order transition of the next commit takes it from 1 to **0** and moves three other figures with it
> — dashboard warnings 6 → 7, inspector findings 18 → 19, and a `W-HEX-001` against `WO-MOK-011` — all
> recorded, with their cause, in `WO-MOK-011`'s *Transition to `implemented`* subsection. The validator
> stays PASS at 102 / 0 / 0 across four planes through both.

`updated` moving from `2026-08-19` to `2026-08-20` introduced no `W-HEX-003` date observation: the
inspector reports the same five, against the same seven artifacts, before and after.

## Two things reported rather than repaired

1. **Oracle 5's governance half fails on the merged tree, and it failed there before this decision.**
   Re-running `analysis/amendments.py` on this branch prints `RESULT: FAIL`. It prints the same report,
   line for line, at a clean checkout of `dec1b95` with none of this commit's edits applied, so the
   failure is not this transition's. The script measures against `524a675`, the commit the work started
   from, and the artifacts it reads moved on `master` after that commit for reasons unrelated to this
   work order: `SPEC-MOK-002` and `VREC-MOK-007` are both listed under *What must not have changed at
   all* and both now read **changed**, although the candidate tree at `9ddcf83` leaves both blobs
   identical to the base — compared by blob hash rather than inferred — while `master` at `2157f77`
   does not. `merge/amendments-vs-master.py`, the wrapper that changes the base revision and nothing
   else, prints `RESULT: PASS` on the same tree and reproduces the committed
   `merge/amendment-approvals.md` exactly.

   **`VREC-MOK-011` is not among the artifacts oracle 5 reads**, so its status cannot move this report
   in either direction. `WO-MOK-011`'s row still expects `in_progress`, still finds `in_progress` and
   still passes, because the work order was not moved. That is the one difference from the
   `VREC-MOK-010` transition, which had to disclose a control its own status move broke; this one
   discloses a control that `master` broke. Neither retained report is edited to agree with anything: a
   generated report that has been hand-corrected is no longer generated.

   > **Later fact, added 2026-08-20.** The work-order transition of the next commit does break that row,
   > exactly as the `VREC-MOK-010` transition's did. Both runs then report it, and **each changes exactly
   > one line**: `in_progress` / yes becomes `implemented` / **no**. `merge/amendments-vs-master.py` goes
   > `RESULT: PASS` → `FAIL` on that single control; `analysis/amendments.py` was already `FAIL` for
   > `master`'s reason above and now carries this control too. Still neither retained report is edited.
2. **`W-HEX-001` against `WO-MOK-010` is `master`'s and is untouched here.** Evidence discovery keys on
   file names beginning with the work-order identifier and that chain retains a directory instead.
   Retaining evidence keyed to `WO-MOK-010` is the engineering owner's act on that work order. This
   branch neither causes nor clears it.

## What this commit does

Prose only. It changes one artifact's `status` and `updated`, rewrites the parts of that artifact that
would otherwise assert a lifecycle state it no longer has, and adds this file. **No measurement, digest,
capture, oracle result or judgement changed**, and no source file was touched, so the Rust gates are not
this commit's to move. No secret is added.
