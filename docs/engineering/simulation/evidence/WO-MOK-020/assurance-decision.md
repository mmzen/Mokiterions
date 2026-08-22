# The assurance decision of 2026-08-22 on `VREC-MOK-023`

`VREC-MOK-023` moved from `ready` to `verified` on 2026-08-22, by the repository owner acting as accountable assurance
owner. This note records the decision, the form the instruction took, the seven manual assessments it rests on, what
it accepted, what it does **not** retire, and the harness measured either side of it.

It takes no decision itself. It records one.

**This file is not among `VREC-MOK-023`'s `evidence_paths`, and it is not in `MANIFEST.sha256`, deliberately.** It
postdates the commit that record binds, and a record's evidence set is the capture's rather than the decision's. That
follows the `assurance-decision.md` of `WO-MOK-009` through `WO-MOK-014`, `WO-MOK-017` and `WO-MOK-019`, every one of
which is kept out of its record's paths for the same reason.

**`README.md` is not edited to index this file, and that is a departure from `WO-MOK-019`'s packet, which added a
"Later fact" note to its own README.** That option does not exist here. `README.md` is one of the 23
`evidence_paths`, its bytes are hashed by `MANIFEST.sha256`, and `VREC-MOK-023` is `verified` and records that all 22
hashed files verify `OK`. Editing the README would falsify a claim of a record that can never be corrected. So the
README's opening sentence — "Twenty-three files, including this one, the completion report and `MANIFEST.sha256`" —
stays as written: **it is true of the evidence set, which is still twenty-three.** This is the twenty-fourth file in
the directory and the first that is not evidence.

**One figure of the record reads differently after this file exists, and it is a figure of the bound commit rather
than of the directory.** `VREC-MOK-023`'s gates table records the 23 `evidence_paths` as "equal, as sets" to
`git ls-files` for this packet, measured — as that table's heading says — at the commit the record binds. From this
commit onward `git ls-files` returns **24** for the directory and `evidence_paths` stays **23**. That difference is
this file, by design and not by drift; `MANIFEST.sha256` still verifies **22 of 22 `OK`**, because the manifest and
the evidence set are unchanged.

## The instruction, and its form

**Verbatim:**

> i validate the verification record that can be transitioned, you can also commit + push, I will merge the PR (ignore
> PR #44)

The definite article is not an identifier, so the referent is recorded rather than assumed: `VREC-MOK-023` was the
only `ready` verification record in the repository, the only subject of the report the instruction answered, and the
only record the phrase "that can be transitioned" can pick out. `PR #44` belongs to another chain and was left alone.

**The instruction authorized a push, which the standing working agreement does not.** The agreement is to commit on
the branch and stop; this turn's instruction reversed that for this turn, and reserved the merge to the owner.

## The seven manual assessments

`VER-MOK-017` requires each assessment "with the assessing role, the date, and the evidence read", and states that
**an unsigned assessment is not a recorded one**. All seven were outstanding when the record was captured, and the
record said so as the one thing standing between it and `verified`. They were put to the owner on 2026-08-22, **each
separately, and each with the evidence displayed rather than described** — the selected and no-selection panes of
`12-frames-and-columns.txt` and the extinction pane of `13-extinction-frame.txt` were shown in full, not summarised.

The full table, with the answers as recorded, is in `VREC-MOK-023`'s opening section. In summary: 1 and 2 met by the
product owner, on the frames rather than the specification text; 3 and 4 met by the technical owner on all their
stated grounds; 5 met by the assurance owner, with `O17` and `O19` accepted as sufficient evidence for the
`REQ-MOK-059` boundary; 6 met by the assurance owner on an independent recomputation of rule 9's amended figure; and
7 met by the technical owner on the labels.

**One person holds all three roles here, and the record discloses it rather than leaving it to be inferred.** No
assessment was answered by implication from another. The technical owner's ratification of both amendment rows earlier
the same day decided the substance assessments 3 and 4 turn on, and it was recorded then that it discharged neither,
because an assessment is a statement recorded in a verification record.

**Two things about assessment 6 are stated rather than glossed.** `VER-MOK-017` asks the *assessor* to recompute at
least one amended `SPEC-MOK-004` figure independently of the retained command output. The recomputation — a static
count of `#[test]` across the eight public-tier test files, 4, 7, 11, 8, 29, 7, 22 and 29, summing to **117** — was
performed by the implementation agent and shown to the owner, who accepted it. So the record states the owner's
**reliance on a measurement the agent took**, not the owner's own arithmetic, and writes the method out so that anyone
can repeat it in one command. And the recount is independent of `10-spec-mok-004-measured.txt`, which is the
`cargo test -- --list` output the row forbids relying on, but it is not independent of the tree: it reads the same
files that enumeration reads.

**Assessment 7 carries one finding, named because it is the kind of thing that goes silently stale.** The observer's
eleven label strings are its own copy in `src/state.rs`, not a call into the engine's `Display`, and **no test asserts
that the two sides agree** — nor can one easily, because `O19.2` forbids the engine from naming the observer's type at
all. Parity was established by reading `src/state.rs:206`–`214` against `mokiterions-core/src/simulation.rs:615`–`623`
at the bound commit: nine are byte-identical, and `eat` and `move` are the engine's verb without the payload its
records carry after the colon. The owner assessed the labels as met on that basis and **declined to make a parity test
a condition of this transition**.

## What the transition changed, and what it did not

**`status` moved from `ready` to `verified`. Nothing else in the record moved.** There is no `transition` subcommand
at harness 0.4.0, so the field was hand-edited; `commit`, `worktree_state`, `verified_at`,
`artifact_snapshot_sha256`, `evidence_paths` and both relations stand as `capture-verification` wrote them, so
`verified_at` remains the **capture** time and not the decision time. The document's heading still reads *Candidate*
for the same reason.

**Four statements of the candidate's own prose are superseded by the record's new opening section rather than edited
out of it**, because they record what was true when the decision was asked for: that all seven assessments were
outstanding, the coverage table's "none answered" row, the claim that the change is not verified, and — in part only
— that no human has seen the pane.

**Every figure the record states was re-measured before the field moved**, since a `verified` record can never be
corrected and that commit was the last point at which one could be fixed. All identical, so nothing was corrected:
`fmt` exit 0; `clippy` at `-D warnings` clean; **332 passed, 0 failed, 0 ignored**; `validate` PASS at **153**
artifacts with 0 errors and 0 warnings, and 152 with the record's file moved aside and the worktree clean again
afterwards; `preflight --work-order WO-MOK-020 --phase review` PASS; 22 of 22 manifest files `OK`; and the 23
`evidence_paths` equal as a set to `git ls-files` for this directory. Both harness commands from the pinned `0.4.0`
environment.

## What it does not retire

- **No release.** `verified` authorizes no tag, publish or deploy. A `REL-` record is what covers a release and none
  is written.
- **The five residual uncertainties of `VER-MOK-017`** stand unchanged, and so do the record's own two: the per-tick
  instrument cannot separate the accumulation's cost from noise, and no interactive terminal exists here, so every
  pane claim rests on a rendered buffer.
- **The default-profile per-tick residual of roughly 113 µs** is real for anyone running the default build, is
  established as codegen-unit partitioning in a file the measured path never calls, and **no artifact carries a
  per-tick budget that would receive it**.
- **`docs/ROADMAP.md`** still records these four artifacts as `draft` and the amendments as unwritten. `WO-MOK-020`
  puts the roadmap out of its own scope, so it is reconciled under its own change.
- **The label parity test** of assessment 7, declined as a condition and owed to a further work order if it is wanted.

## The three acts that followed, on the same day

They are recorded here because they are separate acts and conflating them with the transition would overstate it.

1. **`master` at `3ca2028` was merged in** — a merge and not a rebase, because a rebase or squash rewrites `f633eda`
   and orphans this record's provenance, which cannot be repaired. One conflict, `SPEC-MOK-003`'s amendment table,
   resolved by keeping all three rows of 2026-08-22.
2. **`SPEC-MOK-004`'s owed reconciliation was written** by the owner as technical owner: rule 9 to **118**, rule 11 to
   the observer **176** and the workspace **333**, rules 6 and 10 re-measured unmoved, the engine unmoved at 157. The
   record's own 332 and 117 are figures of `f633eda` and are not restated: the record declines to claim that its
   figures survive a merge and names this reconciliation as what accounts for the difference.
3. **`WO-MOK-020` moved from `implemented` to `verified`** by the owner as engineering owner, its *Lifecycle*
   condition — "an eligible commit-bound verification record against `VER-MOK-017`" — met by this record. It was put
   as its own question after the record was verified, and was not taken by implication from it.

**Harness after all three:** `validate` PASS at **155** artifacts, 0 errors, 0 warnings, four planes `E0/W0`;
`preflight --work-order WO-MOK-020 --phase review` PASS; the workspace suite at **333 passed, 0 failed, 0 ignored**.

**No byte of this packet's evidence changed in any of it.** `MANIFEST.sha256` verifies 22 of 22 `OK` at every commit
since the record was bound, and the packet's own `09-measure-spec-mok-004.py` was executed for the reconciliation and
not edited.
