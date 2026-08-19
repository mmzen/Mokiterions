# Release containment for `WO-MOK-007` at candidate `dfab77b`

This file exists so that `VREC-MOK-009` — the aggregate verification record for the 0.1.0 release —
carries one statement per work order it covers, keyed to that work order's own ID. It records what
is true about `WO-MOK-007` **at the release candidate**, which is a different commit from the one
its own verification record binds.

It is not a re-verification. `VER-MOK-007`'s scenarios were performed once, against
`dfab77b`, and `VREC-MOK-007` is the record of that. Nothing here reopens or supersedes it.

## The work order and its own record

| | |
| --- | --- |
| Work order | `WO-MOK-007` — Colour the roster survival bars by value band |
| Status | `implemented` |
| Requirements implemented | 1 |
| Verification definition | `VER-MOK-007` |
| Verification record | `VREC-MOK-007` (`verified`) |
| Commit that record binds | `dfab77b72d2d4db1700fc1ddb4ad7ab96be998e2` |
| Verified at | 2026-08-19T12:23:14Z |
| Evidence paths that record binds | 15 |

## Containment in the release candidate

The commit `VREC-MOK-007` binds is an ancestor of the release candidate:

```console
$ git merge-base --is-ancestor dfab77b72d2d4db1700fc1ddb4ad7ab96be998e2 \
    79512b4aba0a444c23605157bc4da4f2f7eb435b
$ echo $?
0
```

Exit `0` is the whole claim: the verified work is present in what is being released, and the
candidate is not a rebuild or a re-implementation of it.

## What changed after that record was verified

Between `dfab77b` and the candidate there are **22 commits**, of
which **0** tracked Rust files, manifests or the lockfile differ:

```console
$ git rev-list --count dfab77b..79512b4
22
$ git diff --name-only dfab77b 79512b4 \
    -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml' | wc -l
0
```

A non-zero count is expected and is not a defect. Each of the eight records in this release binds a
different commit, so all but the last necessarily have work layered on top of them — that is the
condition an aggregate record exists to reconcile, and it is why a release cannot simply cite the
per-work-order records and stop.

**No tracked Rust file or manifest differs between the commit this work order's record binds and
the release candidate.** It is the only work order in the selection of which that is true, so its
verification needs no argument about intervening change: the code that was verified is the code
being released, byte for byte. The commits after `dfab77b` are the governance commits that
recorded it, this work order's own evidence, and the release preparation.

## Renewed selection at the candidate

```console
$ python -m se_harness preflight --work-order WO-MOK-007 --phase review .
Harness preflight: PASS
Phase: review
Work order: WO-MOK-007 (implemented)
```

Exit `0`. Preflight is derived, read-only evidence: it confirms this work order still selects
cleanly against the artifact graph at the candidate — its reading manifest resolves, its relations
resolve, and its assurance classification is present. It does not verify work and does not approve
anything.

## What this file does not claim

- It does not claim `VER-MOK-007` was re-run at the candidate. It was not.
- It does not claim the files `VREC-MOK-007` binds are unchanged. The counts above say otherwise where
  they are non-zero.
- It does not carry a status decision. `VREC-MOK-009` is `ready` when it is captured, and only an
  accountable assurance owner may transition it.

What the release rests on, for this work order, is the pair: `VREC-MOK-007` verified its scope at an
ancestor commit, and the candidate's own checks — recorded in `candidate-checks.md` beside this
file — pass on the tree being released.
