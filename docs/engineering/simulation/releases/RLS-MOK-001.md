+++
id = "RLS-MOK-001"
type = "release_record"
title = "Release candidate 0.1.0"
status = "released"
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

# Released Release Record

This record authorizes release `0.1.0` of `WO-MOK-001`, `WO-MOK-002`, `WO-MOK-003`, `WO-MOK-004`, `WO-MOK-005`, `WO-MOK-006`, `WO-MOK-007`, `WO-MOK-009` from authorized commit `755db7297aa993f00d42f9c9794584b5d061f03d`. An accountable release owner transitioned it from `ready` to `released`; the final section records that decision. The command that prepared the record approved nothing and did not commit, tag, release, or publish.

This is the artifact the authorization gate reads. It is the whole of the authority to publish `v0.1.0`, and it is the first one this repository has ever held.

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

Condition 2 is satisfied by this record, as of the decision recorded at the end of this file.
Conditions 1 and 3 depend on a tag that does not exist yet, so the gate refuses on condition 1 until
the release owner creates it. That refusal is the designed order of operations, not an obstacle in it.

Condition 4 is satisfied, and no longer as a projection: after the governance revision was merged,
`check_release_reachability.py --commit 755db72… --default-branch master --remote origin` exits `0`
and reports the commit `contained by refs/remotes/origin/master`, measured against the real remote
rather than a clone. The maintenance branch `release/0.1` is
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

It creates no tag, moves no tag, cuts no branch, builds nothing and publishes nothing. Being `released`
authorizes those acts; it does not perform them, and no command in this repository performs them
either. `docs/engineering/DECISION_RIGHTS.md` reserves them to an accountable release owner, and the
command that prepared this file exercised no decision right.

What remains, in this order, and all of it by hand: cut and push `release/0.1` from `755db72`; push the
annotated `v0.1.0` at `755db72`; then a person publishes the draft the workflow uploads. The governance
revision was merged to `master` before this record was transitioned, so that step is already behind us.

## The decision, recorded

`status` is `released`. The accountable release owner decided on 2026-08-19, and this section records
that decision rather than constituting it — the implementation agent transcribed the transition and
exercised no decision right in doing so.

This is the decision the entire release is downstream of. Everything else is either a check that
reports it or an act that carries it out, so what it was taken against is stated plainly:

- **The candidate.** `755db7297aa993f00d42f9c9794584b5d061f03d`, verified by `VREC-MOK-009`, reachable
  from `origin/master`, and carrying the eight work orders whose own records are each an ancestor of
  it.
- **The contract.** `REL-MOK-001`, `approved`, including the two things it does not soften: the
  first-run limitation, and the absence of signatures, attestation, SBOM and reproducible builds.
- **The first execution.** `.github/workflows/release.yml` has never run. The gate's authorizing path
  was rehearsed in a throwaway clone and nothing more than the gate was rehearsed. This decision
  starts the first real execution of the release process, and the run's own logs are therefore
  evidence to retain rather than a routine build record.
- **What is still open.** The `release` environment named by `SPEC-MOK-005` rule 12.6 is unconfigured
  in repository settings. `WO-MOK-008` remains `draft` and is not in this release.

`released_at` above reads `2026-08-19T17:53:05Z`, which is when `prepare-release` wrote the record and
not when this decision was taken. The field is generated, the decision is dated in this section, and
the two are hours apart on the same day. Nothing shipped carries the timestamp:
`PROVENANCE.txt` records the record, contract, work orders, verification records, tag, commit, target,
toolchain and build URL, and no time from this file.
