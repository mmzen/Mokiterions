# The assurance decision of 2026-08-20 on `VREC-MOK-013`

`VREC-MOK-013` moved from `ready` to `verified` on 2026-08-20, by the repository owner acting as accountable
assurance owner. **The same instruction verified `REQ-MOK-048`, which the record it was given against said
was not verified and could not be verified without a manual assessment nobody available may take.** This
note records the decision, both of its parts, what it accepted, what it does **not** retire, where it
deviates from `VER-MOK-013`, and the measured harness state either side of it.

**This file is not among `VREC-MOK-013`'s `evidence_paths`, deliberately.** It postdates the commit that
record binds, and a record's evidence set is the capture's rather than the decision's. It follows the
`VREC-MOK-007` precedent at `e1d6f6e`, the `VREC-MOK-010` precedent at `8c29830` and the `VREC-MOK-011`
precedent at `965fe67`, whose `assurance-decision.md` is kept out of its record's paths for the same reason.
It is the **second** file in this directory outside that set; `identifier-collision.md` is the first, and the
directory now holds 27 files against `evidence_paths`' 25.

It takes no decision itself. It records one.

## The instruction

Verbatim, in the turn it was given:

> i approve VREC-MOK-013, making REQ-MOK-048 verified.

`DECISION_RIGHTS.md` reserves the `ready -> verified` transition to the accountable assurance owner and
states that record preparation never makes it. The implementation agent recorded the decision and did not
take it.

**The instruction named one artifact and two acts, and the second act was not one the record offered.** The
transition it authorized is the one the harness was asking for. The clause *"making `REQ-MOK-048`
verified"* is not: `VREC-MOK-013` as captured said in terms that this requirement was **not verified**, that
*"no status on this record can make it so"*, and that the route decided on 2026-08-20 — find an admissible
assessor — was *"the only one that verifies the requirement"*. Reading the clause as a consequence of the
status would have made the record's own words false without anyone saying so.

**So the basis was put back to the owner before anything was written**, with what each choice costs
measured rather than described:

| Option put | What it would have meant | Answer |
|---|---|---|
| Accept the automated cases | `REQ-MOK-048` verified on its four automated cases in place of the primary evidence, `VER-MOK-013` unamended, assessment 2 left outstanding, the deviation recorded here | **Taken** |
| Amend `VER-MOK-013` | Reclassify the discoverability assessment in the contract, which is a governed artifact amendment with its own approval act, and which would retire the contract's *"not the property"* sentence | Not taken |
| Transition the record only | `VREC-MOK-013` `verified` with `REQ-MOK-048` still disclosed as unverified — the `VREC-MOK-011` shape exactly | Not taken |

The answer was the first: **accept the automated cases, leave the contract alone.** Nothing here is approved
by implication, and the option not taken is recorded because the option taken is narrower than the sentence
that authorized it might read.

## What the decision accepted

**The accountable assurance owner accepts the retained evidence for `WO-MOK-013` at commit `41c20ca`, and
accepts `REQ-MOK-048`'s four automated cases in place of the manual assessment `VER-MOK-013` designates as
that requirement's primary evidence — with the assessment unperformed and the contract unamended.**

The four cases, all passing at the bound commit, discharged by three arriving tests in
`mokiterions-tui/tests/render.rs`:

| `VER-MOK-013` case | Test |
|---|---|
| The overlay key is on screen from the first frame | `the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport` |
| It survives every viewport above the floor | the same test — one measurement covers both, as `test-census.md` records |
| It is not timed | `the_hint_is_present_after_two_hundred_ticks_in_both_run_states` |
| It displaces no obligation | `the_hint_displaces_neither_the_announcement_nor_the_footer` |

`REQ-MOK-048`'s own front matter reads `verification_method = "automated-test"`, and has since it was
approved. **That is the strongest thing that can be said for this basis, and it is not nothing**: the
requirement as written asks for an automated method, and the automated method passes. It is also not the
whole of the governing text, which is the next section.

## Where this deviates from `VER-MOK-013`, in the contract's own words

`VER-MOK-013` is `approved` and unchanged. Three of its sentences bear on this decision and none of them is
edited:

- **Line 141:** the two manual assessments are *"the **primary evidence** for their requirements rather than
  a confirmation of the automated cases."*
- **Line 152:** *"**An automated case cannot perform this assessment**, and the automated case beside it —
  that the character is in the buffer — is a necessary condition and **not the property**."*
- **Lines 180–182:** *"Both manual assessments, each with its author, its date, its role, and the terminal it
  was performed on. **An assessment with no author is outstanding**, which is the condition `VREC-MOK-005`
  disclosed for all seven of `VER-MOK-005`'s and **which this contract is written to avoid repeating**."*

**What is verified is therefore the necessary condition, accepted as sufficient by decision, and not the
property `REQ-MOK-048` names.** The character is in the buffer at every viewport, at frame 1 and after 200
ticks, without displacing the announcement or the footer. Whether a person who has not read
`SPEC-MOK-003` rule 7 would find the controls from it is **not measured and is not claimed.**
`VER-MOK-013` is **not satisfied**, on one count and one only.

This is a deviation from an approved verification contract on one requirement, taken by the role that owns
verification, recorded where a reader of the record will meet it. It is not an amendment: the contract still
says what it says, and anyone reading `VER-MOK-013` alone will find the assessment contracted for and
outstanding, which is the correct thing for them to find.

## What the decision does not retire

1. **Manual assessment 2 is not performed, and is not recorded as performed.** It asks that a person who has
   not read `SPEC-MOK-003` rule 7 be shown one frame and asked how they would see the controls.
   `manual-assessment.md` still reads **OUTSTANDING** against it, with **author: none**, and was **not**
   edited to agree with this transition — it records the acceptance beside its own result rather than in
   place of it. **There is still no admissible assessor among the people this chain has available.**
2. **The assessment is still worth taking, and the route decided earlier still stands.** `VER-MOK-013`
   records that it is not repeatable — one assessor at most, since nobody can un-know the key — and
   `discoverability-assessment.md`, `discoverability-frame.txt` and `discoverability-frame-floor.txt` are the
   administrator's packet, unchanged. What moves is what taking it would do: it would **confirm or
   contradict** a verified requirement rather than first verify one. **A contradiction would be a finding
   against this decision**, not against any figure in `VREC-MOK-013`, and it would belong in a new record
   rather than as an edit to that one.
3. **`VER-MOK-013` is not amended and no artifact's `verification_method` is changed.** Amending the
   contract was one of the three options and was declined. `REQ-MOK-047`, `REQ-MOK-048` and `REQ-MOK-049`
   are byte-identical to their approved forms; the deviation lives in the record and in this note, which is
   where a deviation belongs.
4. **The merged tree is not verified.** `VREC-MOK-013` binds `41c20ca`, and `master` has since taken this
   chain through the merge at `798e5d5`. The code is identical — `git diff --stat 41c20ca 798e5d5 -- .
   ':(exclude)docs'` is empty, so every automated case, static check, buffer capture and comparison holds
   there unchanged — but the harness graph is not the graph the record's `artifact_snapshot_sha256` names.
   **A record bound to the merge commit is a new record rather than an edit of this one, and this decision
   does not create it.**
5. **`WO-MOK-013` stays `implemented`.** The instruction named the verification record. Moving a work order
   is the engineering owner's act and was not instructed; `WORKFLOW.md` makes a status change a record of
   authority rather than a confidence estimate, and this is the `VREC-MOK-011` contrast exactly — nothing is
   approved here by implication.
6. **Nothing is released, tagged or published by this decision.** No release record binds this work.
   `VER-MOK-013` is a verification contract and not a release gate, and a verified record is an input to a
   release decision rather than one. `RLS-MOK-001` released 0.1.0 from `755db72`, which does **not** contain
   `41c20ca` — checked with `git merge-base --is-ancestor`, not assumed. The merge that reached `master`
   preceded this transition and did not depend on it.
7. **No gate is re-measured and no provenance is rewritten.** Every figure in `VREC-MOK-013`'s gate table is
   the candidate commit's and stays there. `commit`, `git_object_format`, `worktree_state`, `verified_at`,
   `artifact_snapshot_sha256`, all 25 `evidence_paths`, both relations and the `title` — which still reads
   *"Verification candidate for WO-MOK-013"* — are exactly as `capture-verification` produced them, so the
   snapshot digest still names the graph holding the `ready` form of the file. **`status` is the only
   front-matter field that moved**: `updated` already read `2026-08-20`, because the capture and the decision
   fall on the same day, which is the `VREC-MOK-010` case rather than the `VREC-MOK-011` one.
8. **The nine reported findings are still reported and still unfixed**, in item 8 of `completion-summary.md`
   — the announcement ladder's three implemented rungs against the four amended rule 5 admits, and the floor
   header at exactly 34 columns with zero margin among them.
9. **Nothing out of scope is touched.** The eight `W-HEX-003` reassessments, the four `W-HEX-001`
   observations, `WO-MOK-008`'s draft disposition, `VREC-MOK-005`'s staleness, manual assessment 7 of
   `VER-MOK-005`, `ROADMAP.md`'s Phase 2 claim and the CI `W-REV-003` finding are all still open. This
   decision resolves none of them and none of them blocked it.

## The identifier collision was determined by the merge, not by this decision

`origin/governance/adr-mok-006-third-party-crates` claims `WO-MOK-013`, `VER-MOK-013`, `VREC-MOK-013` and
`REQ-MOK-047` as well. **This side reached `master` first**, at `798e5d5` on 2026-08-20T18:41:12+02:00,
thirty-one minutes after that branch's current tip `ee0c086` was pushed and while it was still unmerged. By
decision 3 of this chain the second branch to `master` renumbers, so **the renumbering is theirs and this
record's `id`, filename and `verifies_work_order` relation stand as captured.** That is an outcome of merge
order. `identifier-collision.md` carries the measurement, including the eight conflicted paths their branch
now reports against `master` and the four of those that are this collision. **This decision imposes nothing
on that branch and renumbers nothing.**

## Measured before and after

Both readings taken in this clone with `HEAD` at `798e5d5`. The "before" reading was taken by stashing the
record's edit and re-running, **not by recalling an earlier figure**, so the pair is a true pair at one
revision. The only front-matter difference between them is this record's `status`; everything else this commit
carries is prose, including this file's existence. **The "after" reading was re-taken once every edit in this
commit was in the tree**, and every figure below — the snapshot digest included — came back identical to the
reading taken immediately after the status move.

| Reading | Before | After |
|---|---|---|
| `validate_engineering_artifacts.py` | PASS — 109 artifacts, 0 errors, 0 warnings across four planes | **identical** |
| `check_engineering_harness.sh` | PASS — 109 artifacts, 374 relations, 0 errors | **identical** |
| `generate_harness_dashboard.py` | PASS — 109 artifacts, 374 relations, 0 errors, 12 warnings | **identical but for the snapshot digest** |
| dashboard snapshot | `ec31eea6b6003994b047f2eea3ef46eab751dbe61481559bf319f05c0d27df11` | `94526daa1c59d77ded3f7afd6fea560a59ddc752b96a8871bd9d553dbc4ea732` |
| `inspect_engineering_artifacts.py` findings | 25 — error 0, warning 12, info 13 | **identical** |
| `W-HEX-001` / `W-HEX-003` observations | 4 / 8 | **4 / 8, unchanged** |
| Decision required | **1** — `VREC-MOK-013` `[ready]` assurance-review | **0 — none** |
| Definitions pending | 1 — `WO-MOK-008` `[draft]` | 1, unchanged |
| Active work | 0 | 0, unchanged |
| Assurance pending | 0 | 0, unchanged |
| Suggested next steps | 14 | **13** |
| `harnessctl validate` | PASS — 109 / 0 / 0 | **identical** |
| `harnessctl preflight --phase review --work-order WO-MOK-013` | PASS, work order `implemented` | **identical** |
| `harnessctl doctor` | PASS on all 81 managed and seed checks | **identical** |

**The transition answers exactly the signal it was asked for and changes nothing else.** The diff of the two
*Suggested next steps* lists is one line long, and it is the line that was owed:
`decision_required -> review-assurance-decision (assurance-owner): Review retained evidence and record or
withhold the accountable verification decision. [VREC-MOK-013]`. No error count, warning count, finding
total or observation tally moves.

**A snapshot digest is comparable only against another taken in the same clone at the same `HEAD`.**
`build_snapshot` in `scripts/generate_harness_dashboard.py` hashes `repository.name` — the checkout
directory's name — and `repository.revision` from `git rev-parse HEAD` alongside the artifacts, the
relations and the findings. The pair above is a valid pair for that reason and for no weaker one; neither
digest is the digest of any commit, and no record binds either. This is the correction `WO-MOK-011`'s
`assurance-decision.md` records against its own table, applied in advance rather than after the fact. It
also does not hash artifact prose: the body rewriting in `VREC-MOK-013` moves the digest only through the
front-matter `status` change, so the same digest would follow from the status move alone.

`status` moving with `updated` held at `2026-08-20` introduced no `W-HEX-003` date observation: the inspector
reports the same eight, against the same artifacts, before and after.

## What this commit does

Prose, and one front-matter field. **`VREC-MOK-013.status` is the only status this commit moves** — `ready` →
`verified`. Everything else it touches is text, in three treatments kept distinct on purpose:

- **Rewritten, with the earlier form quoted in place.** `VREC-MOK-013`'s body, the *Status* paragraph of
  `docs/ROADMAP.md`, the file-count sentence of this pack's `README.md`, and the *Lifecycle* paragraph of
  `WO-MOK-013` that described the record as a `ready` candidate. Each of those asserted a lifecycle state
  that no longer holds, so leaving it would have made a current document wrong; each carries the sentence it
  replaced as a quotation so the change is reviewable rather than silent.
- **Added beside, not in place of.** Later-fact blocks in `README.md`, `completion-summary.md`,
  `closing-review.md`, `manual-assessment.md`, `identifier-collision.md`, `docs/ROADMAP.md` and
  `WO-MOK-013`'s *Transition to `implemented`* and approval subsections. The prose they annotate is
  unchanged, including every sentence that is still true of the state it describes.
- **Deliberately untouched.** `VER-MOK-013` in full; `manual-assessment.md`'s assessment 2 status line and
  its summary row, which still read **OUTSTANDING, author none**; `WO-MOK-013`'s status, scope and stop
  conditions; and every capture in this directory.

**No measurement, digest, capture, oracle result or manual assessment outcome changed**, and no source file
was touched, so the Rust gates are not this commit's to move. No secret is added.
