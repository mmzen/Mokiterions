# Release containment for `WO-MOK-004` at candidate `95f0aa2`

This file exists so that `VREC-MOK-009` — the aggregate verification record for the 0.1.0 release —
carries one statement per work order it covers, keyed to that work order's own ID. It records what
is true about `WO-MOK-004` **at the release candidate**, which is a different commit from the one
its own verification record binds.

It is not a re-verification. `VER-MOK-004`'s scenarios were performed once, against
`95f0aa2`, and `VREC-MOK-004` is the record of that. Nothing here reopens or supersedes it.

## The work order and its own record

| | |
| --- | --- |
| Work order | `WO-MOK-004` — State every option's effect and default in the help output |
| Status | `implemented` |
| Requirements implemented | 1 |
| Verification definition | `VER-MOK-004` |
| Verification record | `VREC-MOK-004` (`verified`) |
| Commit that record binds | `95f0aa2079d4abced1c01f4c09b5c66dc5ab29fe` |
| Verified at | 2026-08-17T19:09:46Z |
| Evidence paths that record binds | 34 |

## Containment in the release candidate

The commit `VREC-MOK-004` binds is an ancestor of the release candidate:

```console
$ git merge-base --is-ancestor 95f0aa2079d4abced1c01f4c09b5c66dc5ab29fe \
    79512b4aba0a444c23605157bc4da4f2f7eb435b
$ echo $?
0
```

Exit `0` is the whole claim: the verified work is present in what is being released, and the
candidate is not a rebuild or a re-implementation of it.

## What changed after that record was verified

Between `95f0aa2` and the candidate there are **60 commits**, of
which **33** tracked Rust files, manifests or the lockfile differ:

```console
$ git rev-list --count 95f0aa2..79512b4
60
$ git diff --name-only 95f0aa2 79512b4 \
    -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml' | wc -l
33
```

A non-zero count is expected and is not a defect. Each of the eight records in this release binds a
different commit, so all but the last necessarily have work layered on top of them — that is the
condition an aggregate record exists to reconcile, and it is why a release cannot simply cite the
per-work-order records and stop.

The smallest surface in the selection: one requirement, about the help output stating every
option's effect and default. The help text is exercised by the workspace test run that passes at
the candidate, so this is the work order whose verified behaviour is most directly re-observed by
the candidate's own checks.

## Renewed selection at the candidate

```console
$ python -m se_harness preflight --work-order WO-MOK-004 --phase review .
Harness preflight: PASS
Phase: review
Work order: WO-MOK-004 (implemented)
```

Exit `0`. Preflight is derived, read-only evidence: it confirms this work order still selects
cleanly against the artifact graph at the candidate — its reading manifest resolves, its relations
resolve, and its assurance classification is present. It does not verify work and does not approve
anything.

## What this file does not claim

- It does not claim `VER-MOK-004` was re-run at the candidate. It was not.
- It does not claim the files `VREC-MOK-004` binds are unchanged. The counts above say otherwise where
  they are non-zero.
- It does not carry a status decision. `VREC-MOK-009` is `ready` when it is captured, and only an
  accountable assurance owner may transition it.

What the release rests on, for this work order, is the pair: `VREC-MOK-004` verified its scope at an
ancestor commit, and the candidate's own checks — recorded in `candidate-checks.md` beside this
file — pass on the tree being released.
