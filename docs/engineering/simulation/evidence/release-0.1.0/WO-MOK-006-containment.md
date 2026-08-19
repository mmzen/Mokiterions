# Release containment for `WO-MOK-006` at candidate `dd0c2c0`

This file exists so that `VREC-MOK-009` — the aggregate verification record for the 0.1.0 release —
carries one statement per work order it covers, keyed to that work order's own ID. It records what
is true about `WO-MOK-006` **at the release candidate**, which is a different commit from the one
its own verification record binds.

It is not a re-verification. `VER-MOK-006`'s scenarios were performed once, against
`dd0c2c0`, and `VREC-MOK-006` is the record of that. Nothing here reopens or supersedes it.

## The work order and its own record

| | |
| --- | --- |
| Work order | `WO-MOK-006` — Give each package its own directory and the observer its own tested contract |
| Status | `implemented` |
| Requirements implemented | 3 |
| Verification definition | `VER-MOK-006` |
| Verification record | `VREC-MOK-006` (`verified`) |
| Commit that record binds | `dd0c2c0f7fc12c4fd11c457cfdc166d50b3d6ae4` |
| Verified at | 2026-08-18T09:34:13Z |
| Evidence paths that record binds | 67 |

## Containment in the release candidate

The commit `VREC-MOK-006` binds is an ancestor of the release candidate:

```console
$ git merge-base --is-ancestor dd0c2c0f7fc12c4fd11c457cfdc166d50b3d6ae4 \
    79512b4aba0a444c23605157bc4da4f2f7eb435b
$ echo $?
0
```

Exit `0` is the whole claim: the verified work is present in what is being released, and the
candidate is not a rebuild or a re-implementation of it.

## What changed after that record was verified

Between `dd0c2c0` and the candidate there are **42 commits**, of
which **6** tracked Rust files, manifests or the lockfile differ:

```console
$ git rev-list --count dd0c2c0..79512b4
42
$ git diff --name-only dd0c2c0 79512b4 \
    -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml' | wc -l
6
```

A non-zero count is expected and is not a defect. Each of the eight records in this release binds a
different commit, so all but the last necessarily have work layered on top of them — that is the
condition an aggregate record exists to reconcile, and it is why a release cannot simply cite the
per-work-order records and stop.

This work order gave each package its own directory and the observer its own tested contract. The
6 files that differ since `dd0c2c0` are all observer sources and their tests —
the work `WO-MOK-005` and `WO-MOK-007` did inside the contract this work order established, both of
them in this selection with their own verified records. The contract itself, and the tests this
work order introduced to hold it, are part of the 179 tests passing at the candidate.

Worth noting because it looks like an error and is not: this record was verified on 2026-08-18,
before `VREC-MOK-005`, yet it binds an **older** commit than `VREC-MOK-005` does. Verification order
and commit order are independent here, which is another reason a release cannot infer one candidate
commit from the set of per-work-order records.

## Renewed selection at the candidate

```console
$ python -m se_harness preflight --work-order WO-MOK-006 --phase review .
Harness preflight: PASS
Phase: review
Work order: WO-MOK-006 (implemented)
```

Exit `0`. Preflight is derived, read-only evidence: it confirms this work order still selects
cleanly against the artifact graph at the candidate — its reading manifest resolves, its relations
resolve, and its assurance classification is present. It does not verify work and does not approve
anything.

## What this file does not claim

- It does not claim `VER-MOK-006` was re-run at the candidate. It was not.
- It does not claim the files `VREC-MOK-006` binds are unchanged. The counts above say otherwise where
  they are non-zero.
- It does not carry a status decision. `VREC-MOK-009` is `ready` when it is captured, and only an
  accountable assurance owner may transition it.

What the release rests on, for this work order, is the pair: `VREC-MOK-006` verified its scope at an
ancestor commit, and the candidate's own checks — recorded in `candidate-checks.md` beside this
file — pass on the tree being released.
