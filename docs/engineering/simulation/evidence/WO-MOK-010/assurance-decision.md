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

## What happened after this decision, and what it changed on `master`

This section is the canonical account of the facts that arrived after the two transitions. It is here rather than in
the bound records because those say what was true of the commits they name, and because this file is already outside
the verification record's `evidence_paths` for the same reason. Times are UTC.

**Out of draft, then merged as a merge commit.** The repository owner took pull request #17 out of draft and merged it
at **`5eed5a97235ab73d9964ff3ccde059b0ef74ddb3`**, 19:45:55Z. It is a merge commit and not a squash, which is what
keeps this chain's records meaningful on the default branch: `1a937a1` — the commit `VREC-MOK-010` binds — is an
ancestor of `master`, so the binding still resolves there rather than naming a commit that only a closed pull request
remembers. The chain arrived unaltered: `git diff 8c29830 5eed5a9` over `WO-MOK-010.md`, `VER-MOK-010.md`,
`VREC-MOK-010.md` and every file of this packet is **empty**, and the merge brought in only `master`'s own side
(`SPEC-MOK-005`, `WO-MOK-008`, `WO-MOK-009`, `VER-MOK-008` and the release chain). `Engineering Harness` passed on
`master` at the merge commit.

**`master` at `5eed5a9`:** validator PASS, 95 artifacts, 0 errors and 0 warnings across all four planes · harness 95
artifacts, 319 relations, 0 errors, **6** warnings · inspector error 0 / warning 6 / info 11, with *Decision
required*, *Active work* and *Assurance pending* all empty and one *Definitions pending* entry that is `master`'s own
(`WO-MOK-008` at `draft`). The 80-artifact figures in the table above are this branch's and are deliberately not
re-measured: they are the graph the decision was taken over.

**Release 0.1.0 was released while this pull request sat, and it does not include `WO-MOK-010`.** `d52fb4f`
(18:49:22Z) brought in `REL-MOK-001`, `VREC-MOK-009` and `RLS-MOK-001`; `7da7e73` (19:41:28Z) transitioned
`RLS-MOK-001` from `ready` to `released` — four minutes before this chain merged. The record authorizes version
`0.1.0`, tag `v0.1.0`, from authorized commit `755db7297aa993f00d42f9c9794584b5d061f03d`, and its `releases_work`
names `WO-MOK-001` through `WO-MOK-007` and `WO-MOK-009`. Checked rather than read off the front matter:
`scripts/check_release_authorization.py --tag v0.1.0` reports **AUTHORIZED** and
`scripts/check_release_reachability.py --commit 755db729…` reports **REACHABLE**, contained by `origin/master`.
**`WO-MOK-010` is in no release record.** Its work is on `master` and unreleased, and a release record is the only
thing that would release it, so *Verification is not release* above is now a statement with a released 0.1.0 beside it
rather than an abstraction.

**One warning changed meaning rather than changing count.** The paragraph above says `W-HEX-001` is "the warning every
implemented work order in this repository already carries", and on this branch it was: eight of eight. On `master` it
fires for **`WO-MOK-010` alone**, and the dashboard's warning total is 6 rather than 13. The cause is `master`'s
release evidence: it retains one containment statement per released work order, each in a **file named for that work
order** — `evidence/release-0.1.0/WO-MOK-005-containment.md` and seven more — and evidence discovery keys on file
names beginning with the work-order identifier. Those files cleared the warning for every work order 0.1.0 released.
This packet retains its evidence in a **directory** named for the work order instead, and `WO-MOK-010` is not in
0.1.0, so it has no keyed file and is now the sole observation. **Nothing was added here to match the pattern**: a
file created to satisfy a regex is not evidence, and the repository owner's decision of 2026-08-19 was to leave the
warning standing. It clears when `WO-MOK-010` enters a release record and gets a containment statement on the same
footing as the other eight.

**Decision 12's obligation still stands, and 0.1.0 shipped the debt it names.** Two facts, and they do not cancel.

The obligation — a work order of its own resolving the `VREC-MOK-005` layer, completing **before the next release
record** — was recorded in `1a937a1` at 17:50:30Z. 0.1.0's authorized commit `755db729` was made at **17:33:29Z**,
seventeen minutes earlier; `RLS-MOK-001` stamps `released_at` **17:53:05Z** and the commit preparing it, `9108566`, is
17:54:24Z. **The repository owner's decision of 2026-08-19 is that the obligation binds the next release record
prepared after it, not one whose candidate commit was already fixed when it was written**: 0.1.0 was in flight, so it
is not the record the obligation reaches. On that reading the obligation is **not breached and not discharged** — it
stands, and the release record that includes `WO-MOK-010` is the one it binds. The timestamps are given rather than
summarised so that the other reading remains available: by date alone, `RLS-MOK-001` is the next release record and a
reader may conclude the obligation was overtaken two and a half minutes after it was written.

What the reading does not soften: **0.1.0 releases `WO-MOK-005`**, the chain whose six amendment rows and seven manual
assessments are the eleven provisions this work order's gate overrode, and **the release chain does not disclose
them** — `VREC-MOK-009` lists `WO-MOK-005` as `verified` against `f361370` with no mention of the outstanding items,
and neither `RLS-MOK-001` nor `REL-MOK-001` mentions them. The status moved, the substance did not, and the substance
is now shipped under tag `v0.1.0`. No work order resolving it exists, on `master` or on any branch.
