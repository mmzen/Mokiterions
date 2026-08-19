# Release containment for `WO-MOK-002` at candidate `68163ac`

This file exists so that `VREC-MOK-009` — the aggregate verification record for the 0.1.0 release —
carries one statement per work order it covers, keyed to that work order's own ID. It records what
is true about `WO-MOK-002` **at the release candidate**, which is a different commit from the one
its own verification record binds.

It is not a re-verification. `VER-MOK-002`'s scenarios were performed once, against
`68163ac`, and `VREC-MOK-002` is the record of that. Nothing here reopens or supersedes it.

## The work order and its own record

| | |
| --- | --- |
| Work order | `WO-MOK-002` — Implement perception, resource rebalance, and the reference decision source |
| Status | `implemented` |
| Requirements implemented | 3 |
| Verification definition | `VER-MOK-002` |
| Verification record | `VREC-MOK-002` (`verified`) |
| Commit that record binds | `68163ac452619e2f8d5a05ed3a73d42b920ba5f6` |
| Verified at | 2026-08-17T15:49:52Z |
| Evidence paths that record binds | 11 |

## Containment in the release candidate

The commit `VREC-MOK-002` binds is an ancestor of the release candidate:

```console
$ git merge-base --is-ancestor 68163ac452619e2f8d5a05ed3a73d42b920ba5f6 \
    79512b4aba0a444c23605157bc4da4f2f7eb435b
$ echo $?
0
```

Exit `0` is the whole claim: the verified work is present in what is being released, and the
candidate is not a rebuild or a re-implementation of it.

## What changed after that record was verified

Between `68163ac` and the candidate there are **73 commits**, of
which **34** tracked Rust files, manifests or the lockfile differ:

```console
$ git rev-list --count 68163ac..79512b4
73
$ git diff --name-only 68163ac 79512b4 \
    -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml' | wc -l
34
```

A non-zero count is expected and is not a defect. Each of the eight records in this release binds a
different commit, so all but the last necessarily have work layered on top of them — that is the
condition an aggregate record exists to reconcile, and it is why a release cannot simply cite the
per-work-order records and stop.

The engine behaviour this work order added — perception, the resource rebalance and the reference
decision source — has been carried through both later relayouts without being reopened. What
changed under it is where it lives, not what it does.

## Renewed selection at the candidate

```console
$ python -m se_harness preflight --work-order WO-MOK-002 --phase review .
Harness preflight: PASS
Phase: review
Work order: WO-MOK-002 (implemented)
```

Exit `0`. Preflight is derived, read-only evidence: it confirms this work order still selects
cleanly against the artifact graph at the candidate — its reading manifest resolves, its relations
resolve, and its assurance classification is present. It does not verify work and does not approve
anything.

## What this file does not claim

- It does not claim `VER-MOK-002` was re-run at the candidate. It was not.
- It does not claim the files `VREC-MOK-002` binds are unchanged. The counts above say otherwise where
  they are non-zero.
- It does not carry a status decision. `VREC-MOK-009` is `ready` when it is captured, and only an
  accountable assurance owner may transition it.

What the release rests on, for this work order, is the pair: `VREC-MOK-002` verified its scope at an
ancestor commit, and the candidate's own checks — recorded in `candidate-checks.md` beside this
file — pass on the tree being released.
