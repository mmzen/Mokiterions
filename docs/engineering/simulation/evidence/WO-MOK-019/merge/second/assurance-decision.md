# Assurance decision — `VREC-MOK-019` moved from `ready` to `verified`

**This file is not in `VREC-MOK-019`'s `evidence_paths`, and that is deliberate.** A record's evidence paths name what
the accountable owner accepted, and all fifty-four it names are the account of the candidate commit
`e96648a2a6524a80c761b791378ca91289f02ba2` and the renumbering that preceded it. This file postdates that commit and
is the reasoning behind the status rather than evidence the status accepts. `VREC-MOK-019`'s own *deliberately not
declared* section states the rule in advance — *"if this record is transitioned to `verified` the note recording that
decision must likewise stay out"* — and this file is the case it anticipated. The precedents are
`evidence/WO-MOK-011/assurance-decision.md`, `evidence/WO-MOK-014/assurance-decision.md`,
`evidence/WO-MOK-014/merge/assurance-decision.md` and `evidence/WO-MOK-019/assurance-decision.md`, the last being the
decision on `VREC-MOK-012` for this same work order.

**It takes no decision itself. It records one.**

It is in `merge/second/` rather than one or two directories above because `evidence/WO-MOK-019/assurance-decision.md`
is already taken by the decision on `VREC-MOK-012`, and because the evidence this decision was taken on is this
directory and the first merge's packet beside it. One work order now holds two decisions, one per record, each next
to the evidence it was taken on — the same arrangement `WO-MOK-014` reached for the same reason.

## The instruction

> i validate the verification record, you can transition, commit and push

`DECISION_RIGHTS.md:14` states that *"only accountable assurance and release owners may transition those records to
`verified` or `released`; record preparation never creates commits, tags, or publications"*. The repository owner
holds that role, as they hold the technical, product, engineering and release roles, so **nothing here is approved by
implication** — the instruction named one validation and three acts, and this file is the record of exactly those.

**The instruction does not name the record, and the referent is unambiguous for a measurable reason.**
`VREC-MOK-015`'s instruction named its record; this one says *"the verification record"*. At the commit the
instruction was given, `VREC-MOK-019` was the repository's only artifact at `ready`, the only entry in the
inspector's `decision_required` queue — the queue an assurance validation answers — and the only subject of the
preceding report. There was one verification record it could mean. That is recorded rather than assumed away,
because a definite article is not an identifier.

**`WO-MOK-019` was not moved.** No instruction named it, it stays at `implemented`, and `WORKFLOW.md` is explicit
that a work order's status never substitutes for a verification record's. The work order is now covered by two
`verified` records bound to two commits, `50364a3` and `e96648a`.

## What the decision accepted

In the `ready` record's own words, which the transition took as they stood: commit `e96648a` — the second merge of
`origin/master` into this chain — as the candidate satisfying `VER-MOK-012`, and the fifty-four retained files listed
in `evidence_paths` as the evidence for it. What that evidence is — six oracles re-derived at the merge rather than
argued forward from a diff, the record-stream accounting that closes at `3981726 = 14 × 284409`, the twelve-versus-
fifteen event-kind census across 120 capture cells, the alphabet closure measured over 1,365,884 records, the ten
gates at 301 tests, and the renumbering's disclosure — is enumerated in the record, in `README.md` beside this file
and in `../README.md`, and is not restated here.

**Two empty diffs are what the acceptance rests on, and they are measurements rather than inferences.**
`git diff efe20e3 e96648a -- mokiterions-core` and `git diff 7f4792a e96648a -- mokiterions-tui` are both empty, so
the engine that wrote every retained stream is byte-identical at the candidate and the observer is byte-identical to
`master`'s. `gates.txt` in this directory states that had either been non-empty the 120 cells would have had to be
re-taken.

**No new judgement was required of the assurance owner, and the record says plainly which judgement it could not
reach.** `VER-MOK-012`'s eight manual assessments are all recorded, on 2026-08-20, so the contract's clause that
*"an unrecorded assessment is an outstanding assessment"* is satisfied as written. Residual 6 of the record discloses
that assessment 1's answer — *"Sufficient. No fact is named missing"* — was given against the twelve-kind stream at
`50364a3` while the candidate emits fifteen kinds and a `suffered` field on all 284,409 `action_trace` records, so it
is satisfied as written and stale in substance. **This decision does not re-record it and cannot**: it is a judgement
held by the product owner. The status accepts the evidence at this commit; it does not assert that the specification
is complete.

## `W-REV-004`: a finding this transition silences without answering

The `ready` record raised one warning that `VREC-MOK-012`'s transition did not have to face:

> `W-REV-004` (derived): `VREC-MOK-019` is ready but its work is fully covered by verified or released records;
> review possible supersession without inferring authority. `[VREC-MOK-012, VREC-MOK-019]`
>
> `W-REV-004` → `review-verification-supersession` (assurance-owner): Assess explicit supersession against one
> eligible verified or released successor; do not transition automatically.

**It is gone from the graph after this transition, and it was not answered.** Its predicate is `ready`; the record is
now `verified`, so the finding stops being raised. No supersession was assessed, none was taken, and neither the
record nor this file rules on the question the finding put.

Four things follow, and all four are disclosed rather than left to be discovered:

1. **The finding was reported to the owner before the instruction was given.** It was measured when the `ready`
   record was prepared, written into the record as its own section with **both** readings argued and neither
   inferred, and reported in words to the owner as one of the two decisions the preparation left open. The
   instruction to transition came after that report and chose the transition over the supersession.
2. **The direction of the rule is the opposite of the obvious reading, and the owner was told so.** It does not ask
   whether `VREC-MOK-012` should be superseded by the new record. It asks whether the **new** record is redundant —
   `possible_successors=VREC-MOK-012` — and therefore whether this candidate should have been superseded rather than
   transitioned. Reading it the other way round would have made the transition look like the finding's own
   recommendation, which it is not.
3. **This transition forecloses the supersession route for `VREC-MOK-019` itself.**
   `VERIFICATION_RECORD.template.md:30` permits a separate governance decision to move only a **`ready`** record to
   `superseded` and states no route for a `verified` one. Both records over this work order are now `verified`, so
   neither can be superseded by the other under the template as written. That is the same exposure
   `evidence/WO-MOK-014/merge/assurance-decision.md` priced when the equivalent finding was silenced there, arising a
   second time from the same clause.
4. **What stands in its place is stated, not measured.** `VREC-MOK-012` binds `50364a3`, a commit that predates both
   merges and whose tree has no `attack_resolved`, no `threat_resolved`, no `surrender_resolved`, no `suffered` field
   and none of `master`'s governance; `VREC-MOK-019` binds `e96648a`. The second is not the first re-pointed, the two
   commits differ in content, and each record carries the evidence measured at its own. One work order with two
   `verified` records is the outcome, and the graph accepts it: no finding replaced `W-REV-004` and no error or queue
   entry appeared.

## Measured before and after

Both readings were taken in this clone with `HEAD` at `e6dfc782542f5d078e35055fff7782733ddbd870` and the artifact
content as the only moving input, which is the only way a snapshot digest pair is comparable: `build_snapshot` puts
`git rev-parse HEAD` into the hashed document and the checkout directory's name into it once, so the same content
measured after the commit that carries this file exists will hash to something else again. **No digest is given here
for the commit that carries this file, and none can be.** The digest `VREC-MOK-019` declares, `c4da6a5e…`, is neither
of the figures below: it is the digest of the candidate `e96648a`, measured in a clean detached worktree of a
matching directory name, and this transition does not touch it.

| Reading | Before, `status = "ready"` | After, `status = "verified"` |
|---|---|---|
| `validate_engineering_artifacts.py` | PASS — 147 artifacts, 0 errors, 0 warnings across four planes | **identical** |
| `generate_harness_dashboard.py` | PASS — 147 artifacts, 525 relations, 0 errors, **18** warnings | PASS — 147, 525, 0 errors, **17** warnings |
| dashboard snapshot | `b6225a4391fab3b9dd72170d0b50094e02308c10f04d4cb0319d3414bb88d230` | `b6943c3acc767d9f33f61f3e8e29acd58a48e267374af5a20d485fbf5cc00775` |
| `inspect_engineering_artifacts.py` findings | 37 — error 0, warning 18, info 19 | **36 — error 0, warning 17, info 19** |
| `W-REV-004` | 1 observation, `[VREC-MOK-012, VREC-MOK-019]` | **absent** |
| `W-HEX-001` / `W-HEX-003` / `I-REV-001` observations | 7 / 10 / 19 | **7 / 10 / 19, unchanged** |
| Decision required | **1** — `VREC-MOK-019` `[ready]` assurance-review | **0 — none** |
| Definitions pending | 2 — `WO-MOK-008` `[draft]`, `WO-MOK-017` `[draft]` | 2, unchanged |
| Active work | 0 — none | 0, unchanged |
| Assurance pending | 0 — none | 0, unchanged |
| Suggested next steps | 21 | **19** |
| `preflight --work-order WO-MOK-019 --phase review` | PASS, exit 0, `WO-MOK-019 (implemented)`, commit-bound verification `required` | **PASS, exit 0, unchanged** |
| `doctor` under the pinned 0.4.0 venv | 81 PASS, 0 FAIL, exit 0 | **81 PASS, 0 FAIL, exit 0** |
| `unittest discover -s scripts` | not taken before the edit | `Ran 126 tests`, **`OK`** |

The two rows the `before` column takes from the candidate's own reading at this same `HEAD` rather than from a run
made for this table are `preflight` and `doctor`; both were re-run for the `after` column and both are unchanged.
`unittest discover` is an after-only reading and is marked as one rather than given a fabricated `before`.

**This transition moves a warning count, and the previous record's did not always.** `VREC-MOK-014`'s transition
moved the `decision_required` queue from 1 to 0 and no warning count with it. This one moves 18 to 17 and the finding
total 37 to 36, because `W-REV-004`'s predicate is the status that changed — exactly as `VREC-MOK-015`'s transition
did. Two of the two vanished next-step entries are that finding's and the emptied queue's. A reader who expects a
status change to be invisible in the warning count would be right about `VREC-MOK-014` and wrong here.

**The snapshot digest moves for two reasons and not for a third.** `status` is inside the normalized front matter,
and the findings list is hashed too, so dropping `W-REV-004` moves the digest as well. It does **not** move for the
prose added to the record — the new *assurance decision* section, its quotation of what the candidate said and its
naming of the three falsified sentences — because artifact prose is not hashed. That was checked rather than assumed.
The digest was measured three times at the same `HEAD`: with `status` moved and no prose added, `b6943c3a…`; with the
whole decision section written, `b6943c3a…`; and with this file present and untracked, `b6943c3a…`. All three agree.

**Why this file cannot move it, read out of the hashed document rather than argued.** Two mechanisms carry evidence
into the snapshot and this file meets neither.

- **Declared paths.** A record's `evidence_paths` are hashed as the strings they are, with no existence check and no
  content hash beside them. This file is not declared, by the choice the first paragraph records.
- **Discovered evidence, keyed on the filename.** The document's `evidence` node holds **9** entries covering
  `WO-MOK-001` through the release packet, and every path in it is prefixed with its work order's identifier. The
  number of entries for `WO-MOK-019` is **zero**: not one file in this work order's 110-file packet is named
  `WO-MOK-019-*`, which is precisely the `W-HEX-001` observation the graph reports against it. So the discovery index
  cannot see this file, and it could not have seen it under any name inside this directory that did not begin
  `WO-MOK-019-`.

**The reason is the name, not the subject matter.** Had this file been called `WO-MOK-019-assurance-decision.md` it
would have entered the discovered-evidence index and moved the digest without ever being declared anywhere. That is
the same keying `W-HEX-001` reports against seven work orders, seen from the inside, and it is why the digest figures
above can be written inside the packet they cover.

**`updated` did not move and introduced no `W-HEX-003` observation.** It already read `2026-08-21` — the capture date
and the decision date both, the capture timestamped `17:04:11Z` and the decision taken later the same day — so the
ten date observations are the same ten against the same artifacts before and after.

## What the decision does not retire

1. **`VER-MOK-012`'s per-kind oracle still covers twelve event kinds where the verified tree emits fifteen.** This is
   residual 1 of the record and the largest thing the status leaves open. `oracle1/record-kinds.txt` in the directory
   above measures twelve under `baseline`, `reference` and `individual` and fifteen under `social`, in all thirty
   social cells. Extending the contract is the artifact owner's act and no part of it is taken here.
2. **The two `OUTSTANDING` amendment rows stand.** The 2026-08-21 `SPEC-MOK-006` row — three record shapes, five
   `result.detail` values, a closed `target_died` domain and fourteen field rows, all derived from the streams rather
   than from reading `master`'s source — and the 2026-08-21 `SPEC-MOK-004` row for this merge's five conflict
   regions. Both are the technical owner's separate acts. This decision claimed no ratification of either, before or
   after.
3. **`ARCH-MOK-002` must still be reassessed against `SPEC-MOK-004` as amended.** `W-HEX-003` names it before and
   after this transition, and the obligation is owed whether or not the warning had appeared, because what changed in
   `SPEC-MOK-004` is substantive rather than a date.
4. **The method question on the retained captures is unanswered**, and this decision does not answer it. The
   renumbering swept `WO-MOK-018` into `WO-MOK-019` across 132 occurrences in 45 retained captures, where the
   `013 → 014` precedent refused the equivalent sweep on the reasoning that *"a capture that has been improved is no
   longer a capture"*. `../../renumbering.md` discloses it and states the reading rule for the verbatim bytes.
   `VREC-MOK-012`'s `title`, which reads *"Verification candidate for WO-MOK-019"*, rides on the same answer. Until
   it is answered, 45 files this decision accepted as evidence carry an identifier the tools that produced them never
   printed.
5. **Assessment 1 of `VER-MOK-012` is stale in substance and is not re-recorded**, for the reason given above. The
   eight `detail` words `master`'s targeted validation can produce remain exercised by neither capture, one of them
   unreachable while a `debug_assert!` invariant holds.
6. **`VER-MOK-012`'s nine residual-uncertainty bullets are inherited unchanged** — the alphabet argument being only
   as complete as the enumeration, non-perturbation being verified over the declared matrix, the replay consumer
   being a second implementation of part of the engine, and the rest.
7. **The release.** `REL-MOK-001` is `approved`; `RLS-MOK-001` released 0.1.0 from `755db72`, which does not include
   this work. **No release record binds this work and none is created here**, and this status makes no commit
   release-eligible.
8. **The integration was not this decision's to take and had already been taken.** Pull request #31 merged at
   2026-08-21T16:52:03Z as `fac152f`, before this record existed. The record discloses that ordering in its own
   second paragraph. `e96648a` is an ancestor of `origin/master`, which is what keeps it bindable, and verification
   is not merge: nothing in this decision authorized the integration retroactively or should be read as having done
   so.
9. **No CI run has ever evaluated review preflight against `WO-MOK-019`.** Residual 9 of the record measures why: the
   `candidate` job took its work order from the pull-request body's stale `Harness-Work-Order: WO-MOK-012` trailer
   and reported commit-bound verification `not_required`, the inverse of this work order's `required`. The merge
   commit `fac152f` carries the pull request's title permanently. **The evidence that review preflight passes at
   this work order is in the record's gates table and in the `after` column above, measured locally, and nowhere
   else.** The push this instruction authorizes lands on `feature/phase-4a-definition`, whose pull request is
   already merged, so it will produce a `push`-event run — which has no preflight step at all — and no new
   `pull_request` run. This decision does not create a pull request and none is asked for.

## Five statements in the declared evidence are falsified by this commit, and none is edited

All five are in `VREC-MOK-019`'s `evidence_paths`, all five were faithful when taken, and **a `verified` record's
evidence is not rewritten after the decision** — the rule `evidence/WO-MOK-012/amendment-ratifications.md` states for
`VREC-MOK-005`, and the reason a generated report that has been hand-corrected is no longer generated. This file is
where the later state is recorded, which is what a decision file is for. A reader who resolves these paths at
`00e58a9` or on `master` at `fac152f` will find them as quoted.

1. `merge/README.md:627` — *"The decision about this tree belongs to a new verification record at the merge commit,
   `VREC-MOK-019`, and it is not written yet."* It is written, and it is `verified`.
2. `merge/README.md:505` lists *"A new verification record at the merge commit, `VREC-MOK-019`"* among the acts the
   packet still owes. That act is discharged by this commit; the four other items in that list are not, and items 1
   through 4 of *What the decision does not retire* above say which.
3. `merge/second/gates.txt:103` — *"The verification record at the merge commit — `VREC-MOK-019`, not yet written —
   is where a decision about this tree is recorded, and it binds a commit."* The clause after the dash is exactly
   right and is what happened; only *not yet written* is stale.
4. `merge/second/governance.txt:85` — *"the decision this chain still owes is its own, and it is the one this queue
   cannot show, because `VREC-MOK-019` is not written."* The queue showed it as
   `VREC-MOK-019 [ready] assurance-review` from `e6dfc78` until this commit, and shows nothing now.
5. `merge/second/governance.txt:175` — *"It will move again when `VREC-MOK-019` declares the paths in this directory,
   and that change is expected rather than a discrepancy."* The prediction held. The `before` and `after` digests in
   the table above are both figures of a tree in which those paths are declared, and neither is the `c4da6a5e…` that
   `governance.txt` was written beside.

`../renumbering.md:113` calls `VREC-MOK-019` *"the number `merge/README.md` names for the record still owed"*. That
sentence is about the identifier being free, which it was and which the record's existence confirms rather than
contradicts, so it is listed here as an adjacent reading and not as a sixth falsification.

## What this commit does

Prose and one front-matter field. It moves `VREC-MOK-019`'s `status` from `ready` to `verified`, adds an *assurance
decision* section to that record which quotes the instruction, names the one field that moved and names the three
sentences the authorized acts falsified, and adds this file. **The rest of the record is retained exactly as the
candidate wrote it**, which is a deliberate departure from `VREC-MOK-015`'s transition — that one rewrote the
falsified sentences in place — and follows `VREC-MOK-018`'s later practice instead, on the principle that a record
binds a commit and stays true of that commit.

**No measurement, digest, capture, oracle result or judgement in the retained evidence changed**, and no file in
`evidence_paths` was touched. No Rust source, manifest, lockfile, workflow or script is touched, so no code figure is
this commit's to move. Nothing is renumbered, nothing is merged, no work order moves, no tag and no release is taken.

Every command behind every figure in this file is offline, reads no credential, secret, token or environment value,
and none appears in this file or in any retained evidence.
