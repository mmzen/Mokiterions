+++
id = "RLS-MOK-001"
type = "release_record"
title = "Release candidate 0.1.0"
status = "ready"
owners = ["release owner"]
created = "2026-08-19"
updated = "2026-08-19"
version = "0.1.0"
commit = "755db7297aa993f00d42f9c9794584b5d061f03d"
git_object_format = "sha1"
released_at = "2026-08-19T17:53:05Z"
authorized_by = "release owner"
tag = "v0.1.0"

[relations]
satisfies = ["REL-MOK-001"]
includes_verification = ["VREC-MOK-009"]
releases_work = ["WO-MOK-001", "WO-MOK-002", "WO-MOK-003", "WO-MOK-004", "WO-MOK-005", "WO-MOK-006", "WO-MOK-007", "WO-MOK-009"]
+++

# Release Record Candidate

This ready record proposes release `0.1.0` for `WO-MOK-001`, `WO-MOK-002`, `WO-MOK-003`, `WO-MOK-004`, `WO-MOK-005`, `WO-MOK-006`, `WO-MOK-007`, `WO-MOK-009` from candidate commit `755db7297aa993f00d42f9c9794584b5d061f03d`. An accountable release owner must review and transition it to `released`; this command did not approve, commit, tag, release, or publish anything.

The release candidate commit may precede the governance commit retaining this record. Any release tag must be created and checked by the authorized release process.

## Two revisions, and why the difference matters

This record names **two different commits**, and confusing them is the one mistake that would produce
a refused or a wrong release.

| | Commit | What it is |
| --- | --- | --- |
| **Authorized revision** | `755db7297aa993f00d42f9c9794584b5d061f03d` | What gets built, tagged and shipped. Taken from `VREC-MOK-009`, not from `HEAD`. |
| **Governance revision** | the tip of `governance/release-0.1.0`, and after merging, the merge commit on `master` | Where this record and the two decisions above it live. Never tagged. |

`prepare-release` took the commit from the verification record rather than from the checkout, which is
why the authorized revision is older than the file naming it. `755db72` is an ancestor of the
governance tip (`git merge-base --is-ancestor` exits `0`), and the only files differing between them
are `VREC-MOK-009.md` and `REL-MOK-001.md` — the two decision records. **No Rust source, manifest or
lockfile differs**, so the governance commits changed nothing that ships.

**The tag `v0.1.0` must point at `755db72`, not at the merge commit.** The authorization gate compares
the tag's target against this record's `commit` and refuses on mismatch, so tagging the merge produces
a refusal rather than a wrong release — but it is a refusal that costs a tag deletion to undo.

## What the gate will check

`scripts/check_release_authorization.py --tag v0.1.0` runs before anything is built, and refuses
unless all of the following hold:

1. `v0.1.0` exists as an annotated tag on the remote.
2. A release record for that tag exists with `status = "released"` — this record, after the release
   owner's decision.
3. The record's `commit` is the tag's target.
4. That commit is reachable from a release-bearing branch: the default branch, or any `release/*`
   branch. Local branches are not consulted.

Condition 2 is the one this record cannot satisfy by itself. It is `ready`.

Condition 4 is satisfied by the merge to `master` alone. The maintenance branch `release/0.1` is
required by `REL-MOK-001`'s promotion policy and by Phase C of the runbook, not by this check —
cutting it is a stabilization decision, not a way to pass the gate.

## What was rehearsed, and what was not

The authorizing path had never returned `0` in this repository, because no tag had ever existed. It
was exercised in a throwaway clone with its own object store: the two decisions simulated, this record
transitioned to `released`, and `v0.1.0` created against the **authorized** commit. The gate returned
`AUTHORIZED release 0.1.0 of RLS-MOK-001`, exit `0` — the first time it has ever authorized anything.
Both suites passed against that state (48 and 22 cases). Reachability refused before the merge and
reported `REACHABLE`, contained by `refs/remotes/origin/master`, after it.

That rehearsal establishes the gate's arithmetic and nothing about the workflow.
`.github/workflows/release.yml` still has never executed: no build has run, no archive has been
produced on any runner, and no draft release has ever been created. Transitioning this record to
`released` is what starts that first execution.

## What this record does not do

It creates no tag, moves no tag, cuts no branch, builds nothing and publishes nothing. It is `ready`,
which means prepared and not decided. `docs/engineering/DECISION_RIGHTS.md` reserves the transition to
`released` to an accountable release owner; the command that wrote this file exercised no decision
right, and the approval of `REL-MOK-001` does not extend to it.

After that transition, in this order: merge the governance revision to `master`; cut and push
`release/0.1` from `755db72`; push the annotated `v0.1.0` at `755db72`; then a person publishes the
draft the workflow uploads.
