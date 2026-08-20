# The assurance decision of 2026-08-20 on `VREC-MOK-012`

`VREC-MOK-012` moved from `ready` to `verified` on 2026-08-20, by the repository owner acting as
accountable assurance owner. This note records the decision, the form the instruction took, what it
accepted, what it did **not** retire, and the measured harness state either side of it.

**This file is not among `VREC-MOK-012`'s `evidence_paths`, deliberately.** It postdates the commit that
record binds, and a record's evidence set is the capture's rather than the decision's. It follows the
`VREC-MOK-007`, `VREC-MOK-010` and `VREC-MOK-011` precedents, whose `assurance-decision.md` is kept out
of its record's paths for the same reason.

It takes no decision itself. It records one.

## The two instructions of 2026-08-20, and which one did what

The day carried two owner acts on this chain, and conflating them would overstate the first.

**The first, verbatim:**

> i validate VER-MOK-012 and and 8 manual assessments

That act named the verification contract and its eight manual assessments. It did **not** name this
record, so it did not transition it, and `DECISION_RIGHTS.md` reserves the `ready` → `verified`
transition to the accountable assurance owner as a separate act. It was recorded in commit `56d17d9`
with this record left at `ready` — the record stayed `ready` for a commit of its own, which is the
visible evidence that the two acts are two.

**The second** came after the owner asked, verbatim, *"ask questions interactively, I will answer"*, and
took the form of four answers to four framed questions rather than a sentence. Recorded as selected,
with the option label in each case:

| Question | Answer selected |
|---|---|
| Does your validation of `VER-MOK-012` also carry the assurance decision moving `VREC-MOK-012` from `ready` to `verified`? | **Transition it to verified** |
| How should the nine defects measured in the approved artifacts be handled? | **Defer to a correction work order** |
| `SPEC-MOK-004`'s rule 11 amendment row is recorded not approved. What is your decision as technical owner? | **Approve the row** |
| What lands where — pushing `56d17d9`, `WO-MOK-012`'s status, and PR #31's draft state? | **Push, WO implemented, PR ready** |

**The form matters and is recorded rather than smoothed over.** Each answer is a selection from options
that stated their own measured cost, so what the owner accepted is what the option said, and this note
quotes the options rather than paraphrasing them. Each of the four questions was answered separately, so
unlike the first instruction of the day — one act covering eight assessments — none of these four is
carried by another. The implementation agent measured the options, put them, recorded the answers, and
decided none of them.

The four answers are four acts by three roles the same owner holds: assurance owner for the transition,
technical owner for the amendment row, engineering owner for the work-order status and the push. They are
committed separately, in that dependency order, because the transition's acceptance depends on the
amendment row already being approved.

## What the decision accepted

**The accountable assurance owner accepts the retained evidence for `WO-MOK-012` at commit `50364a3`,
with two *Evidence retention* bullets unmet as written, nine measured defects uncorrected, and the eight
manual assessments recorded as one blanket act.**

That is stated in the terms the `ready` record used to describe what accepting it would mean, so the
acceptance cannot be read wider than what was put in front of the owner.

- **The seven oracles as measured**, all passing. The acceptance carries one known weakness, named in the
  record and not discovered afterwards: **defect 1 leaves oracle 5's size assertion for one domain of
  thirteen without an independent witness**, because `SPEC-MOK-006` rule 3.2 says the direction domain
  holds "the eight fixed direction words" and it holds four, and the oracle's expected size was
  transcribed from the engine. For that one domain the oracle compares the engine against itself. What
  bounds it is measured rather than argued: **0 diagonal direction words appear in any of the seven
  retained streams** — four record streams and three text streams — so the domain the specification
  misdescribes is one no retained record uses.
- **The two retention deviations, as they stand.** Bullet 3's post-change standard output is held as
  digests only and bullet 4's thirty full sink streams are four. Nothing was re-captured and nothing was
  deleted. Accepting bullet 3's substitution accepts that oracle 1's byte-identity result across all 90
  cells is what stands in for the unretained text streams — which the packet names rather than relies on
  silently, because a reader who does not accept oracle 1 should not accept the substitution either.
- **The eight manual assessments as recorded**, including assessment 8, whose second half — that the
  replay consumer is derived from `SPEC-MOK-001` rather than from the engine's code — admits no
  mechanical check and is a reading, and assessment 4, recorded against measured substance rather than
  against its own prompt because the prompt is defect 5.
- **The nine defects uncorrected**, on the disposition below.

**`VER-MOK-012` is therefore answered in substance and still not satisfied in every literal respect.**
The decision is recorded against that description of the contract and not against a claim of full
satisfaction.

## What the decision did not have to accept, because the act before it removed the item

**The `SPEC-MOK-004` rule 11 amendment.** When the question was put, that row was the fourth item on the
record's own list of what stood unresolved, recorded as made beyond `ADR-MOK-005`'s approved list and not
approved. The owner approved it as technical owner in commit `ce45988`, immediately before this one. So
this decision does not accept an unapproved amendment beyond the approved list — there is none left to
accept.

How the approval was written matters to this packet and is recorded here too. It was **prepended** to the
row's status cell with the cell's original text kept unedited beneath, because
`amendment-approvals.md` measures that cell's wording and the measurement predates the approval.
`analysis/amendments.py` was re-run with the approval present and reproduces the retained file **byte for
byte**, exit 0. The generated file is left exactly as generated.

## The nine defects: deferred, which is a decision and not a repair

The owner's disposition is **a correction work order in Phase 4b**, carrying all nine with its own
verification contract and evidence. Recorded precisely:

- **None of the nine is corrected**, and none is dismissed as not a defect. They stand as
  `completion-summary.md` item 16 measured them, in `SPEC-MOK-006` (2), `VER-MOK-012` (2),
  `ADR-MOK-005` (2), `SPEC-MOK-002` (1), `WO-MOK-012` (1) and `REQ-MOK-045` (1).
- **The correction work order does not exist.** Commissioning it is a definition act with its own
  approval, and it was not instructed in this turn. Nothing in this chain assumes it, and neither this
  note nor `VREC-MOK-012` treats the deferral as though the corrections had been made.
- **What the deferral changes is the status of the question, not the state of the artifacts.** Before it,
  nine defects sat in approved artifacts with no owner disposition. After it, they sit there with one:
  a scheduled debt rather than an open question.
- **What it costs is stated rather than left implicit.** Six approved artifacts keep a known wrong
  statement in them until that work order lands, one of which — defect 1 — is the reason oracle 5's
  witness is partial. Correcting them now was the alternative offered and its measured cost was the
  reason to decline: five of the six have no `## Amendment record` section at all, having been created by
  this chain, so each would need one; and one of the nine sits in `VER-MOK-012` itself, the contract the
  owner had validated hours earlier.

## What the decision does not retire

1. **The three carried-forward `OUTSTANDING` amendment rows.** `ARCH-MOK-001` 2026-08-18,
   `SPEC-MOK-002` 2026-08-18, `SPEC-MOK-004` 2026-08-19. None of them is this chain's to pay, all three
   are untouched, and `amendment-approvals.md` section 5 names rather than counts them. The 2026-08-20
   approval reaches one row and no other.
2. **The merge, and a record bound to it.** `50364a3` is a branch commit and is not an ancestor of
   `master`. On a merged tree the gates, the census, the interface enumeration and oracles 1 through 6
   need re-running rather than carrying over, and a record bound to the merge commit is a new record.
   Marking PR #31 ready for review asks for that review; it does not perform the merge.
3. **`VER-MOK-012`'s seven *Residual uncertainty* items**, inherited unchanged.
4. **Everything in `VREC-MOK-012`'s *What this record does not claim*.** That list is what the owner
   accepted the record with, not a list the decision shortens.

## Measured before and after

Both measured **in this clone with `HEAD` at `ce45988`**, the only difference being this record's
`status` field and this file's existence. The `ready` version was restored to disk, measured, and the
`verified` version put back, so the pair is a comparison of one input.

| Reading | Before | After |
|---|---|---|
| `validate_engineering_artifacts.py` | PASS — 114 artifacts, 0 errors, 0 warnings across four planes | **identical** |
| `generate_harness_dashboard.py` | PASS — 114 artifacts, 373 relations, 0 errors, 7 warnings | **identical but for the snapshot digest** |
| dashboard snapshot | `522c834f6a24556584857b05fa2117df3151edb870b36f1e7d674eb0223abb89` | `5e6d59be877e735d402c2dcb50abdada8390e9d3321d2fe800f042aab578e1a4` |
| `inspect_engineering_artifacts.py` findings | 20 — error 0, warning 7, info 13 | **identical** |
| Decision required | **1** — `VREC-MOK-012` `[ready]` assurance-review | **0 — none** |
| Definitions pending | 1 — `WO-MOK-008` `[draft]` | 1, unchanged |
| Active work | 1 — `WO-MOK-012` `[in_progress]` | 1, unchanged — **but see below** |
| Assurance pending | 0 | 0 |
| Suggested next steps | 10 | **9** |

**The transition answers exactly the signal it was asked for and changes nothing else.** The
`decision_required` queue empties from its one entry to none, *Suggested next steps* falls from 10 to 9
with it, and no error count, no warning count and no finding total moves. `updated` did not move either:
it already read `2026-08-20`, so `status` is the only front-matter field this commit changes.

**On the snapshot digest.** It is not purely a digest of artifact content: `build_snapshot` in
`scripts/generate_harness_dashboard.py` hashes `repository.name` — the checkout directory's name — and
`repository.revision` alongside the artifacts, the relations and the findings. The pair above is a valid
pair because both readings were taken in this clone at the same `HEAD` with the artifact front matter as
the only moving input. Taken in another clone, or after another commit, the same content hashes
differently. This is `VREC-MOK-011`'s correction of the same table, reused rather than rediscovered.

**On the *Active work* row.** It stays at 1 through this commit because `WO-MOK-012` is still
`in_progress`. The work-order transition authorized in the same turn is the next commit's act, and by the
`WO-MOK-011` precedent it is what takes that row to 0 — and it moves other figures with it, because the
inspector raises a `W-HEX-001` evidence observation against a work order once it is `implemented`. Those
figures are measured and recorded in the commit that moves them, not predicted here.

> **Later fact, 2026-08-20, in the commit after this one: they were measured, and the prediction above
> held in both halves.** *Active work* went 1 → 0 and a third `W-HEX-001` observation appeared against
> `WO-MOK-012`, taking dashboard warnings 7 → 8 and inspector findings 20 → 21 with error counts
> unmoved. The full pair is in `WO-MOK-012`'s *Transition to `implemented`* section, measured in this
> clone at `HEAD` `5634b6a` with the work order's `status` as the only moving input. **One figure that
> row did not attempt to predict is worth reading there**: *Assurance pending* stayed at **0**, because
> the decision this note records had already put a `verified` record over that work order before its
> status arrived. Had the two commits gone in the other order, the work order would have sat in
> `assurance_pending` for a commit.

## Reported rather than repaired

1. **Oracle 7's script is pinned to the candidate state and must not be re-run as a re-measurement after
   these governance acts.** `analysis/amendments.py` carries `CHAIN` with `WO-MOK-012` at `in_progress`
   and asserts it, so once the work order moves to `implemented` the script reports a status finding
   against the very transition that was authorized. That is the script measuring the tree it was written
   for, not a defect in either. The retained `amendment-approvals.md` is a measurement of the candidate
   tree; its own constants say so.

   > **Later fact: it was re-run in the next commit and reported exactly that, and nothing more** — exit
   > 1, one finding, *WO-MOK-012 carries status `implemented`, expected `in_progress`*, with two places
   > moving in 291 lines. The output went to a scratch file outside the repository and the retained report
   > shows no modification. Predicting the failure and then declining to observe it would have left the
   > claim untested, so it was observed; what was not done is regenerate the report.
2. **`amendment-approvals.md` still reports the eight manual assessments as `OUTSTANDING`.** It is
   generated, it is left as generated, and that is the state its own generation measured. The packet
   `README.md` discloses it. Hand-editing a generated artifact to agree with a later fact is what this
   repository's evidence rule forbids.
3. **`VREC-MOK-012`'s `title` still reads *candidate*.** The record was captured as one and a decision
   does not rewrite the capture. `VREC-MOK-010`'s and `VREC-MOK-011`'s read the same way after their
   transitions.

## What this commit does

It sets `status = "verified"` on `VREC-MOK-012`, adds this file, and records the decision in the record's
body, the packet `README.md` and `manual-assessment.md`. It corrects no defect, resolves no amendment
row, moves no work order, merges nothing, tags nothing and releases nothing.

> **Later fact, 2026-08-20.** The three later-fact blockquotes above were added to this file in the
> commit that followed, which moved `WO-MOK-012` to `implemented`. **The sentence above therefore
> describes the commit that created this file and not the state of the branch**, and it is left as
> written: this commit moved no work order, and the next one moved exactly one by a different accountable
> role. What that commit still did **not** do is the rest of the list — no defect corrected, no amendment
> row resolved, nothing merged, tagged or released.
