# The verification record was renumbered, because `VREC-MOK-007` was not free

Measured 2026-08-19, immediately after the branch was pushed. The push is what surfaced this: the pull request
went to `CONFLICTING`, and the conflict was a record this work order had created under an ID that `master`
already owns.

## What happened

`master` moved while this work was in progress. Its tip is now `7a2b502`, ten commits ahead of this branch's
merge base at `54c21abc`, and among those ten is pull request #19, `governance/vrec-mok-007-candidate`:

| Commit on `master` | What it did |
| --- | --- |
| `335f8c8` | set `WO-MOK-007` to `implemented` |
| `a7ddf74` | transitioned `VREC-MOK-007` to `verified` |
| `f278fd3` | merged pull request #19 |

So `docs/engineering/simulation/verification-records/VREC-MOK-007.md` exists on `master` as a **`verified`**
record that verifies `WO-MOK-007`, conforms to `VER-MOK-007`, binds candidate commit `dfab77b7…` and lists 15
evidence paths under `evidence/WO-MOK-007/`. This branch created a different file at the same path, `ready`,
verifying `WO-MOK-009` against `VER-MOK-008` with 17 evidence paths.

`git merge-tree --write-tree origin/master HEAD` reports exactly one conflict across the whole merge:

```text
CONFLICT (add/add): Merge conflict in docs/engineering/simulation/verification-records/VREC-MOK-007.md
```

The record on this branch was therefore renumbered to **`VREC-MOK-008`**, which is the next ID free on both
sides. Renumbering means re-capturing rather than renaming: the ID is inside the file the managed command
produces, so the superseded file was deleted in one commit and `capture-verification` re-run under the new ID
against that commit.

## The ID census behind that choice

Every artifact ID on both sides, compared rather than assumed. 71 IDs on `origin/master`, 80 on this branch,
**69 shared and every one of the 69 at an identical path**:

| | IDs |
| --- | --- |
| This branch only | `INT-MOK-007`, `CAP-MOK-007`, `REQ-MOK-035`–`039`, `SPEC-MOK-005`, `VER-MOK-008`, `WO-MOK-008`, `WO-MOK-009` |
| `master` only | `VER-MOK-007`, `WO-MOK-007` |
| Shared, same path, different content | `VREC-MOK-007` — the collision, and the only one |

So the eleven governed artifacts this work order introduces are clear of `master` and none of them needs
renumbering. `VER-MOK-008` in particular is free because `master`'s newest verification contract is
`VER-MOK-007`; the number is shared with the renumbered record only in the way `VER-` and `VREC-` prefixes
always share numbers.

## What this corrects, and it is a correction rather than a stale fact

`approval-and-transition.md` said, of the same ID, that *"the ID is the next free one in the sequence and
skipping it to avoid the collision would leave a gap that is harder to explain than the collision itself."*
**That was wrong, and it was wrong when it was written.** The ID was not free — pull request #19 had already
merged — and the sentence reasoned about a collision with a fixture inside this evidence directory while the
real collision was with a `verified` record on `master`. It is corrected there rather than deleted, because
the reasoning is what needs correcting and not only the conclusion.

The check that would have caught it is one command, and this work order's own runbook already prescribed it.
`docs/RELEASE_RUNBOOK.md` said, before this record corrected it, *"Pick the next free record ID.
`VREC-MOK-001` through `006` are taken, and the in-flight `WO-MOK-007` will most likely claim `007`"*, followed
by a `git ls-tree` against `origin/master` and the comment *"adjust to the next genuinely free ID"*. The
prediction was correct and the instruction was not followed: the ID was taken from the local branch's sequence instead of from the remote. **The runbook is not
at fault and neither is the harness** — `capture-verification` has no way to know what `master` holds, and it
accepted `VREC-MOK-007` because nothing on this branch used it.

## Three references that are now misleading, and what should be done about each

1. **`SPEC-MOK-005` line 486 and `REQ-MOK-037` lines 127 and 130 use `VREC-MOK-007` as a worked example** — a
   hypothetical `verified` aggregate covering `WO-MOK-001` through `WO-MOK-006`. On `master` that ID now names
   a real record verifying `WO-MOK-007` alone. Both artifacts are `approved` as of 2026-08-19, so the
   illustrative ID is **not edited here**: amending an approved specification and an approved requirement is
   the technical and product owners' act, and the examples are illustrative rather than normative — no rule
   and no acceptance criterion depends on which ID the example uses. The recommended disposition is a
   one-token amendment to each, replacing the example ID with one that cannot be claimed, at the owners'
   convenience.
2. **`a5-refusal-ladder.md`'s rung-3 fixture invents an aggregate record under the same ID.** It is a
   transcript of what was run, so it is not rewritten. The fixture was constructed inside a throwaway clone
   and never existed in this repository; what it demonstrates — that a `verified` aggregate still does not
   authorize a release — does not depend on the ID it used.
3. **`commit-binding.md` retains the refused `capture-verification` invocation with `--id VREC-MOK-007`.**
   Also a transcript, also left as run. The command it records refused for an unrelated reason: the work order
   was `draft` at the time.

`docs/RELEASE_RUNBOOK.md` is not a governed artifact and is part of this changeset, so its example was updated
to name this collision rather than merely warn about it.

## What did not change

No measurement in this directory moves. The renumbered record binds the same evidence, the same work order and
the same verification contract; only the ID and the commit it was captured from differ, and
`snapshot-reproducibility.md`'s finding applies to the new capture unchanged.

**The conflict was a collision of identity, not of content.** Nothing on `master` and nothing on this branch
was edited to resolve it: one file was deleted and one was created under a free ID. After the renumbering,
`git merge-tree --write-tree origin/master HEAD` reports no conflict, which is the check that closes this
record.

**One hazard stays open.** `VREC-MOK-008` is free on `master` today. This branch is still ten commits behind
and another session may claim it before this pull request merges, exactly as one claimed `007`. The check
before merge is the same `git ls-tree` the runbook prescribes, and it is cheap enough to run again.

This record takes the directory to eighteen files — seventeen records and the index.
`snapshot-reproducibility.md`'s closing count of seventeen was correct when it was written.
