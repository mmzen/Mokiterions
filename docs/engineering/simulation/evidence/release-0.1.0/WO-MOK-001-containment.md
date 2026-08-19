# Release containment for `WO-MOK-001` at candidate `ecd03a8`

This file exists so that `VREC-MOK-009` — the aggregate verification record for the 0.1.0 release —
carries one statement per work order it covers, keyed to that work order's own ID. It records what
is true about `WO-MOK-001` **at the release candidate**, which is a different commit from the one
its own verification record binds.

It is not a re-verification. `VER-MOK-001`'s scenarios were performed once, against
`ecd03a8`, and `VREC-MOK-001` is the record of that. Nothing here reopens or supersedes it.

## The work order and its own record

| | |
| --- | --- |
| Work order | `WO-MOK-001` — Implement the minimum simulation foundation |
| Status | `implemented` |
| Requirements implemented | 12 |
| Verification definition | `VER-MOK-001` |
| Verification record | `VREC-MOK-001` (`verified`) |
| Commit that record binds | `ecd03a89c0c8680d8ce82d7767b787a49aa815fb` |
| Verified at | 2026-08-11T19:56:00Z |
| Evidence paths that record binds | 3 |

## Containment in the release candidate

The commit `VREC-MOK-001` binds is an ancestor of the release candidate:

```console
$ git merge-base --is-ancestor ecd03a89c0c8680d8ce82d7767b787a49aa815fb \
    79512b4aba0a444c23605157bc4da4f2f7eb435b
$ echo $?
0
```

Exit `0` is the whole claim: the verified work is present in what is being released, and the
candidate is not a rebuild or a re-implementation of it.

## What changed after that record was verified

Between `ecd03a8` and the candidate there are **80 commits**, of
which **35** tracked Rust files, manifests or the lockfile differ:

```console
$ git rev-list --count ecd03a8..79512b4
80
$ git diff --name-only ecd03a8 79512b4 \
    -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml' | wc -l
35
```

A non-zero count is expected and is not a defect. Each of the eight records in this release binds a
different commit, so all but the last necessarily have work layered on top of them — that is the
condition an aggregate record exists to reconcile, and it is why a release cannot simply cite the
per-work-order records and stop.

This is the widest gap in the selection, and the reason is structural rather than worrying: this
work order implemented the whole engine, and almost everything since has been built on top of it.
The files its verification named have since moved twice — `WO-MOK-003` split the crate into library
and binary targets, and `WO-MOK-005` split the workspace again — so a path-level comparison against
`ecd03a8` compares against a layout that no longer exists. Both of those relayouts are
themselves work orders in this selection, with their own verified records.

One thing about this work order is worth stating because it is not visible from its status. Its
assurance classification was recorded on 2026-08-19, long after it was approved on 2026-08-11: the
declaration did not exist when the work order was written, and a review preflight is what surfaced
the omission. The classification records a decision about coverage that `VREC-MOK-001` had already
provided; it created no new obligation and no status moved for it.

## Renewed selection at the candidate

```console
$ python -m se_harness preflight --work-order WO-MOK-001 --phase review .
Harness preflight: PASS
Phase: review
Work order: WO-MOK-001 (implemented)
```

Exit `0`. Preflight is derived, read-only evidence: it confirms this work order still selects
cleanly against the artifact graph at the candidate — its reading manifest resolves, its relations
resolve, and its assurance classification is present. It does not verify work and does not approve
anything.

## What this file does not claim

- It does not claim `VER-MOK-001` was re-run at the candidate. It was not.
- It does not claim the files `VREC-MOK-001` binds are unchanged. The counts above say otherwise where
  they are non-zero.
- It does not carry a status decision. `VREC-MOK-009` is `ready` when it is captured, and only an
  accountable assurance owner may transition it.

What the release rests on, for this work order, is the pair: `VREC-MOK-001` verified its scope at an
ancestor commit, and the candidate's own checks — recorded in `candidate-checks.md` beside this
file — pass on the tree being released.
