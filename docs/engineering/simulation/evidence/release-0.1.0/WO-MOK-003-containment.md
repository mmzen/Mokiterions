# Release containment for `WO-MOK-003` at candidate `a7f39f1`

This file exists so that `VREC-MOK-009` — the aggregate verification record for the 0.1.0 release —
carries one statement per work order it covers, keyed to that work order's own ID. It records what
is true about `WO-MOK-003` **at the release candidate**, which is a different commit from the one
its own verification record binds.

It is not a re-verification. `VER-MOK-003`'s scenarios were performed once, against
`a7f39f1`, and `VREC-MOK-003` is the record of that. Nothing here reopens or supersedes it.

## The work order and its own record

| | |
| --- | --- |
| Work order | `WO-MOK-003` — Split the crate into library and binary targets and place tests by required access |
| Status | `implemented` |
| Requirements implemented | 2 |
| Verification definition | `VER-MOK-003` |
| Verification record | `VREC-MOK-003` (`verified`) |
| Commit that record binds | `a7f39f18520433cddc72c044c6fca1b24104bb7d` |
| Verified at | 2026-08-17T17:46:18Z |
| Evidence paths that record binds | 29 |

## Containment in the release candidate

The commit `VREC-MOK-003` binds is an ancestor of the release candidate:

```console
$ git merge-base --is-ancestor a7f39f18520433cddc72c044c6fca1b24104bb7d \
    79512b4aba0a444c23605157bc4da4f2f7eb435b
$ echo $?
0
```

Exit `0` is the whole claim: the verified work is present in what is being released, and the
candidate is not a rebuild or a re-implementation of it.

## What changed after that record was verified

Between `a7f39f1` and the candidate there are **68 commits**, of
which **34** tracked Rust files, manifests or the lockfile differ:

```console
$ git rev-list --count a7f39f1..79512b4
68
$ git diff --name-only a7f39f1 79512b4 \
    -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml' | wc -l
34
```

A non-zero count is expected and is not a defect. Each of the eight records in this release binds a
different commit, so all but the last necessarily have work layered on top of them — that is the
condition an aggregate record exists to reconcile, and it is why a release cannot simply cite the
per-work-order records and stop.

This work order is one of the reasons the earlier records cannot be compared against the candidate
by path: it is the change that split the crate into library and binary targets and moved tests to
sit with the access they need. Its own verification measured the split itself, and the split is
still the shape of the tree at the candidate — `WO-MOK-005` divided the workspace above it without
undoing it.

## Renewed selection at the candidate

```console
$ python -m se_harness preflight --work-order WO-MOK-003 --phase review .
Harness preflight: PASS
Phase: review
Work order: WO-MOK-003 (implemented)
```

Exit `0`. Preflight is derived, read-only evidence: it confirms this work order still selects
cleanly against the artifact graph at the candidate — its reading manifest resolves, its relations
resolve, and its assurance classification is present. It does not verify work and does not approve
anything.

## What this file does not claim

- It does not claim `VER-MOK-003` was re-run at the candidate. It was not.
- It does not claim the files `VREC-MOK-003` binds are unchanged. The counts above say otherwise where
  they are non-zero.
- It does not carry a status decision. `VREC-MOK-009` is `ready` when it is captured, and only an
  accountable assurance owner may transition it.

What the release rests on, for this work order, is the pair: `VREC-MOK-003` verified its scope at an
ancestor commit, and the candidate's own checks — recorded in `candidate-checks.md` beside this
file — pass on the tree being released.
