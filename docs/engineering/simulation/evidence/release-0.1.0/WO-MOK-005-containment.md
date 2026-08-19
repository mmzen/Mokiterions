# Release containment for `WO-MOK-005` at candidate `f361370`

This file exists so that `VREC-MOK-009` — the aggregate verification record for the 0.1.0 release —
carries one statement per work order it covers, keyed to that work order's own ID. It records what
is true about `WO-MOK-005` **at the release candidate**, which is a different commit from the one
its own verification record binds.

It is not a re-verification. `VER-MOK-005`'s scenarios were performed once, against
`f361370`, and `VREC-MOK-005` is the record of that. Nothing here reopens or supersedes it.

## The work order and its own record

| | |
| --- | --- |
| Work order | `WO-MOK-005` — Split the workspace and implement the terminal observer |
| Status | `implemented` |
| Requirements implemented | 9 |
| Verification definition | `VER-MOK-005` |
| Verification record | `VREC-MOK-005` (`verified`) |
| Commit that record binds | `f3613701f3c55d3f3d849747c8cf0790a5729c14` |
| Verified at | 2026-08-19T07:50:45Z |
| Evidence paths that record binds | 21 |

## Containment in the release candidate

The commit `VREC-MOK-005` binds is an ancestor of the release candidate:

```console
$ git merge-base --is-ancestor f3613701f3c55d3f3d849747c8cf0790a5729c14 \
    79512b4aba0a444c23605157bc4da4f2f7eb435b
$ echo $?
0
```

Exit `0` is the whole claim: the verified work is present in what is being released, and the
candidate is not a rebuild or a re-implementation of it.

## What changed after that record was verified

Between `f361370` and the candidate there are **31 commits**, of
which **2** tracked Rust files, manifests or the lockfile differ:

```console
$ git rev-list --count f361370..79512b4
31
$ git diff --name-only f361370 79512b4 \
    -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml' | wc -l
2
```

A non-zero count is expected and is not a defect. Each of the eight records in this release binds a
different commit, so all but the last necessarily have work layered on top of them — that is the
condition an aggregate record exists to reconcile, and it is why a release cannot simply cite the
per-work-order records and stop.

The second-newest record in the selection and the second-smallest gap. Nine requirements — the
workspace split and the terminal observer — and only 2 tracked Rust or manifest files
differ between the commit its record binds and the candidate. Those differences are
`WO-MOK-007`'s colour banding, which is in this selection with its own verified record.

## Renewed selection at the candidate

```console
$ python -m se_harness preflight --work-order WO-MOK-005 --phase review .
Harness preflight: PASS
Phase: review
Work order: WO-MOK-005 (implemented)
```

Exit `0`. Preflight is derived, read-only evidence: it confirms this work order still selects
cleanly against the artifact graph at the candidate — its reading manifest resolves, its relations
resolve, and its assurance classification is present. It does not verify work and does not approve
anything.

## What this file does not claim

- It does not claim `VER-MOK-005` was re-run at the candidate. It was not.
- It does not claim the files `VREC-MOK-005` binds are unchanged. The counts above say otherwise where
  they are non-zero.
- It does not carry a status decision. `VREC-MOK-009` is `ready` when it is captured, and only an
  accountable assurance owner may transition it.

What the release rests on, for this work order, is the pair: `VREC-MOK-005` verified its scope at an
ancestor commit, and the candidate's own checks — recorded in `candidate-checks.md` beside this
file — pass on the tree being released.
