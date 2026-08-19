# Commit binding

Every other record in this directory was captured on 2026-08-19 against a working tree with no commit of
its own. The repository owner then directed the commit. This record names the commit those observations
describe, and reconciles the three statements elsewhere in the directory that said there was none.

It is a separate record rather than an edit to the others for a reason `docs/engineering/WORKFLOW.md`
states directly: *"A record cannot contain the hash of its own commit."* Rewriting a capture record to
name a commit created after the capture would make it describe a tree it did not observe.

## The commit

| Fact | Value |
| --- | --- |
| Candidate commit | `17be4bad444a4199da53e72ae8be491ba5f46ee1` |
| Committed | 2026-08-19 14:37:49 +0200 |
| Branch | `feature/release-ci`, pushed to `origin` |
| Parent | `54c21abcfb9caa4474c9ca5f194289e055c86a23` — the governance revision every record names, and the tip of `master` |
| Contents | 34 files, 8,950 insertions, 2 deletions |
| Worktree after the commit | clean; `git status --porcelain --untracked-files=all` is empty |
| Pull request | [#20](https://github.com/mmzen/Mokiterions/pull/20), open, against `master` |
| Tags | still none |

The commit sits directly on the governance revision, so the two revisions `SPEC-MOK-005` distinguishes have
not yet come apart: there is a candidate commit and no later governance commit on `master`. That matters for
reading `p4-worktree-comparison.md`, whose reachability transcripts were taken when `HEAD` equalled
`origin/master`. They now differ by exactly this commit, and the check's verdict against
`refs/remotes/origin/master` is unchanged, because `17be4ba` is not reachable from the remote's `master` —
which is the correct answer for an unmerged branch and the reason rule 5 exists.

## Gates on the committed tree

Both re-run after the commit, from the pinned `se-harness==0.4.0` wheel, so that these numbers describe the
committed tree rather than an uncommitted one.

```text
$ python -m se_harness doctor .
81 verdict lines: PASS 81, WARN 0, FAIL 0
exit=0

$ python scripts/validate_engineering_artifacts.py --root .
Engineering artifact validation: PASS
Artifacts: 79 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
exit=0
```

## What the commit changes about these records, and what it does not

**Three statements are reconciled here rather than rewritten in place.**

| Where | What it said | How to read it now |
| --- | --- | --- |
| `README.md`, Commit binding | Candidate commit **None**; the changeset is uncommitted working-tree state | true at capture; the commit above is that changeset, and the row now points here |
| `candidate-conformance.md`, first section | the candidate was never committed, so this is not a diff | **still true, and not because of the commit** — see below |
| `completion-summary.md`, finding 2 | the same, as a disclosure | unchanged in substance; the finding is about the *absence of a pre-conformance revision*, which no later commit can supply |

**The reconstruction claim survives the commit intact, and this is the one point worth being careful
about.** `candidate-conformance.md` states what the pre-existing candidate did before it was brought into
conformance with `SPEC-MOK-005`. That earlier state was never committed. `17be4ba` adds all 34 files at
once, so `git show` renders it as an addition, not as a before-and-after: there is no revision in this
repository in which the candidate exists in its pre-conformance form. So the "before" column still rests on
the change record kept during the work, exactly as that file says, and a reader should weigh it the same way
after the commit as before it.

What the commit does add is that every *current* claim in this directory is now checkable against a fixed
revision rather than against a working tree that could move underneath a reader.

**Every capture header is left as written.** Nine records name HEAD `54c21abc…` — six of them in their
opening lines, three of those with the phrase "with this work order's changes present but uncommitted", and
three more further down. That is what was observed, and it stays. Four files name `17be4ba`: this record, and
the three reconciled above — `README.md`, `candidate-conformance.md` and `completion-summary.md`.

## What this record is not

**It is not a verification record, and none exists.** `docs/engineering/WORKFLOW.md` step 7 prescribes
committing the clean final candidate source and evidence and then running `capture-verification`. The first
half is done. The second refuses:

```text
$ python -m se_harness capture-verification . --id VREC-MOK-007 \
    --work-order WO-MOK-009 --verification VER-MOK-008 \
    --evidence docs/engineering/simulation/evidence/WO-MOK-009/README.md \
    --owner "assurance owner"
harnessctl: work order WO-MOK-009 must be active
exit=2
```

`WO-MOK-009` is `draft`. An active status is `approved`, `in_progress`, `implemented`, `verified` or
`released`, and reaching any of them is a governance decision belonging to the engineering owner — for this
work order, one that its own approval preconditions gate behind a technical-owner architecture decision.
The refusal is the harness declining to bind a commit-bound assurance record to unapproved work, which is
the behaviour a reader should want. It was not worked around, and no record was hand-written in its place.

The same reason makes the managed harness CI fail on pull request #20, at its *Review preflight* step. The
push-event run on `17be4ba` does not include that step and exercises the rest.

**It records no decision.** No status was transitioned, no tag exists, nothing was released, published or
merged, and the commit it names is on an unmerged branch. `VER-MOK-008`'s scenario totals are unchanged at
47 observed, 3 rehearsed, 15 not performed, 0 unexercisable: committing is not a run, so V1–V6, P1–P3 and
C4 are exactly as `scenario-map.md` leaves them.

This record and `release-artifact-types.md` take the directory to fifteen files — fourteen records and the
index. Both were written after the commit they describe.
