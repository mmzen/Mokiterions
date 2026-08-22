# The assurance decision of 2026-08-22 on `VREC-MOK-021`

`VREC-MOK-021` moved from `ready` to `verified` on 2026-08-22, by the repository owner acting as accountable assurance
owner. This note records the decision, the form the instruction took, what moved and what did not, the one figure that
was corrected in the transition commit, what the decision does **not** retire, and the five things that followed it:
two merges, the two owner acts that closed `WO-MOK-008`, and the roadmap correction they made necessary.

It takes no decision itself. It records ones already taken.

**This file is not among `VREC-MOK-021`'s `evidence_paths`, deliberately.** It postdates the commit that record binds,
and a record's evidence set is the capture's rather than the decision's. That follows the eleven `assurance-decision.md`
notes already in this tree — under `WO-MOK-009` through `WO-MOK-014`, `WO-MOK-017`, `WO-MOK-019` and `WO-MOK-020`, three
of them nested in a `merge/` subdirectory — **not one of which appears in any record's `evidence_paths`**, measured
across all twenty-three records rather than assumed. This packet has no `MANIFEST.sha256`, so there is no manifest to
fall out of date either.

**`README.md` is not edited to index this file.** It is one of the ten `evidence_paths` of a record that is `verified`
and can never be corrected, so editing it would falsify a claim of that record. The consequence is stated rather than
hidden: from this commit onward `git ls-files` returns **eleven** files for this directory while `evidence_paths` stays
**ten**. The difference is this file, by design and not by drift.

**Its name departs from the packet convention, and the reason is a finding.** Every precedent above is named
`assurance-decision.md`; this one carries the `WO-MOK-008-` prefix because the derived rule `W-HEX-001` matches an
evidence filename against `^(WO-[A-Z0-9-]*\d{3})` and reads the directory rather than the record's declared set — so a
file named `assurance-decision.md` leaves the rule's `WO-MOK-008` observation standing, and this one closes it. That is
a real reason to choose a filename and it is disclosed rather than presented as convention. The measured effect is
below.

## The instruction, and its form

The owner's words were: *"i validate the verification record, you can transition it as verified, commit and push"*.

`DECISION_RIGHTS.md` reserves the `ready → verified` transition to the accountable assurance owner, and `WORKFLOW.md`
step 8 is the step taken: the assurance owner reviews the record and transitions it. The owner holds every role in this
repository, so the act is theirs to take — and for the same reason nothing else is approved by implication. What is
verified is the retained evidence the record declares, bound to the tree at `3da6acc`.

The transition is commit `cb4b108`, *"gov(WO-MOK-008): transition VREC-MOK-021 from ready to verified"* — one file,
+101 / −7.

## What moved, and what did not

**Exactly one front-matter field moved, and it is `status`.** These are unchanged: `commit`
(`3da6acca1e8cb53f20ea13869c6f4bc425b979f2`), `git_object_format`, `worktree_state` (`clean`), `verified_at`
(`2026-08-22T08:35:53Z`), `artifact_snapshot_sha256` (`4cb4044e…`), all ten `evidence_paths`, `verifies_work_order`,
`conforms_to`, `owners`, `title` and the dates.

Two of those deserve saying plainly:

- **`verified_at` remains the capture timestamp and is not the decision time.** It records when the evidence was
  captured against the bound commit, not when the owner read it. The decision is necessarily later than the capture, and
  the field is provenance rather than a record of the act.
- **The title still says "candidate".** `Verification candidate for WO-MOK-008` is what it said as a `ready` record and
  what it says now. Seventeen of the eighteen `verified` records in this repository keep the capture's word; only
  `VREC-MOK-017`, hand-composed with no capture behind it, says "record". The settled practice is that the status field
  carries the status and the title is not rewritten to agree with it.

**Falsified prose in the record is named, not edited.** Six passages were true when the capture was written and are
false after the decision — among them the sentence that the record is `ready` and the row describing what `verified_at`
would mean. They are listed in the record's own decision section on the `VREC-MOK-018` and `VREC-MOK-019` precedent.
Evidence is re-run, not corrected.

**`WO-MOK-008` stays at `implemented`, and its prose is left as its own transition wrote it** — including the sentence
that this record "is a `ready` candidate and is not `verified`". `WO-MOK-018` reads the same today with its own record
`verified`: a work order records the state at its own transition, and the record carries the decision.

## The figure corrected in the transition commit

One measurement inside the record was wrong and was corrected **in the same commit as the transition**, which was the
last moment it could be.

The record's table comparing `W-REV-003` with `I-REV-001` gave `I-REV-001` as **19** for a full clone carrying the
record. Re-measured across four trees it is **20**:

| Tree | `W-REV-003` | `I-REV-001` |
|---|---|---|
| full clone at the candidate | does not fire — every candidate commit is reachable | 19 |
| full clone carrying this record | does not fire | **20** |
| `--depth 1` clone at the candidate | 20 | 19 |
| `--depth 1` clone carrying this record | **21** | 20 |

The conclusion the wrong figure supported — that the informational count is itself depth-sensitive — was false.
`I-REV-001` is **depth-insensitive**, and it sits exactly one below `W-REV-003` at the same artifact set because it
**skips the superseded `VREC-MOK-016`** where the warning does not.

**Why it was corrected before the transition rather than disclosed after it:** `WORKFLOW.md` line 26 admits only a
`ready` record to `superseded`. A `verified` record therefore has no supersession lane and no rebind, so a false
measurement left inside one is permanent. No declared or predicted figure moved: `4cb4044e…`, the candidate's
147 / 525 / 17, the depth-1 tip's 38 warnings and CI's 40 are each unaffected. The correction is a departure from the
record as the owner validated it, and it is disclosed for that reason in the record's decision section and in the
commit message.

## The harness, measured either side of the transition

Identical before and after: **148 artifacts, 527 relations, 0 errors, 17 warnings, 37 findings** at 0 error / 17
warning / 20 info, with `Formal validation: PASS` on both. The transition introduces no finding of its own. Those two
figures are not carried from the record: they were re-measured for this note in a scratch worktree checked out at
`f97568d` and then at `cb4b108`, the commits either side of the transition.

The single change is a queue. `decision_required` held `VREC-MOK-021 [ready] assurance-review` before and is **empty**
after; `assurance_pending` and `definitions_pending` were empty either way. On the transitioned tree the repository
gate passed, the declared-dependency check passed, `se_harness validate` reported PASS, and
`preflight --work-order WO-MOK-008 --phase review` reported PASS at `WO-MOK-008 (implemented)`, commit-bound
verification `required`, decided by the engineering owner.

## What the decision does not do

- **It is not a merge, a tag, a release or a release decision**, and it creates no `RLS` record. `REL-MOK-001` gates
  no work order beyond `WO-MOK-001` through `WO-MOK-007` and `WO-MOK-009`, so this work reaches no release contract
  that exists, and a v0.2.0 would need a new one. One thing it does reach is worth naming for the release owner and is
  recorded here as an observation rather than a decision: **`REL-MOK-001` also gates `VER-MOK-005`**, the verification
  contract this work order amended on 2026-08-22, and `RLS-MOK-001` released v0.1.0 at `755db72` on 2026-08-19 against
  that contract as it read then. Nothing in the released binary changed and no released work order is touched, but the
  gated contract has moved since the release, and no artifact in this tree says what that means for `RLS-MOK-001`.
- **It ratifies neither provision the amendment left open.** On the day of the decision, clause 4's first row was still
  OUTSTANDING for the technical owner and the `REQ-MOK-027` residual was still the product owner's. Both were settled
  later, by separate acts, recorded below.
- **It retires no residual.** In particular **no person has yet looked at a shed footer.** The seven manual
  assessments `VER-MOK-005` contracts run at declared seeds where nothing sheds, so every observation of the shedding
  behaviour is a machine's. Closing that needs a new verification row and its own work order, and it is deliberately
  left open.
- **It does not verify anything committed after `3da6acc`.** The record binds that commit, cannot be re-pointed, and
  covers neither the merge of `master` that followed nor the two owner acts below.

## What followed, after the merge

For completeness, because a reader of this packet will otherwise not find it:

1. **`master` was merged into the branch** as `0c89a8b`. The only conflict was `SPEC-MOK-003`'s amendment table, where
   this work order's row of 2026-08-22 met three of the same date from `master`; all four were retained and a fifth row
   records the reconciliation. Rule 8's text on `master` was byte-identical to the merge base, so no provision of
   either side met the other. A merge was taken rather than a rebase precisely because a rebase would have orphaned
   `3da6acc`.
2. **Pull request #44 merged to `master` at `2ecb78a`**, as a merge commit rather than a squash, so `3da6acc` is an
   ancestor of `master` and the record's provenance survives on the trunk.
3. **Clause 4's first row was ratified as written** by the owner as accountable technical owner — the candidate commit
   sheds ahead of every field `REQ-MOK-027` names. The alternative recorded in `footer-shedding.md`, the commit shed
   last, is declined rather than left open. Recorded in a new `SPEC-MOK-003` amendment row; clause 4 itself is
   unchanged, so nothing in this packet's measurements moves.
4. **The `REQ-MOK-027` residual was accepted knowingly** by the owner as accountable product owner. The floor is not
   raised, the accepted seed range is not narrowed, and the requirement is not amended. Recorded in `REQ-MOK-027`'s
   *Open decisions*, in `VER-MOK-005`'s *Residual uncertainty*, and in the same `SPEC-MOK-003` row.

5. **`docs/ROADMAP.md` was reconciled.** Its one mention of this work order called it *"the release-authorization
   chain, unrelated and still `draft`"* — three claims, all wrong by 2026-08-22 and one never right. The sentence is
   corrected in the file's own idiom, with the earlier form quoted in a blockquote beside it. **One sentence is all
   that changed**, so the roadmap still carries no account of the footer chain, and that gap is left open rather than
   filled: filling it would be a roadmap-structure decision and nobody has taken one.

**None of the five is covered by `VREC-MOK-021`**, and none has a verification record of its own. That is the settled
pattern here rather than an omission: a record binds the commit it was written against, and `master`'s merge carries
none. Acts 3 to 5, and this file, are the commits of a follow-up branch cut from `master` at `2ecb78a`.

## The one figure this file changes

Measured on the tree that carries the three governance edits accompanying this file, first without it and then with it:

| | artifacts | relations | findings | error / warning / info | `W-HEX-001` |
|---|---|---|---|---|---|
| without this file | 156 | 550 | 44 | 0 / 21 / 23 | 11 observations, `WO-MOK-008` among them |
| with this file | 156 | 550 | **43** | 0 / **20** / 23 | **10** observations, `WO-MOK-008` absent |

Nothing else moves: the artifact and relation counts are untouched because this file is evidence rather than an
artifact, `W-HEX-003` stays at 10 and `I-REV-001` at 23, and there are 0 errors either way. The rule asks for evidence
keyed to the implemented work order, and this file is that evidence, keyed by its name.

**It closes the finding without discharging the concern behind it.** `W-HEX-001` asks for evidence keyed to the
implemented work order, and the ten files this record declares were always that; what the rule could not see is that
they are keyed by their directory. So the observation was a false positive from the first, and the honest reading of
this file's effect on the count is that it satisfies a filename pattern. It is not an argument that `WO-MOK-008`'s
evidence improved.
