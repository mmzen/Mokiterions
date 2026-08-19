# Release containment for `WO-MOK-009` at candidate `d35f817`

This file exists so that `VREC-MOK-009` — the aggregate verification record for the 0.1.0 release —
carries one statement per work order it covers, keyed to that work order's own ID. It records what
is true about `WO-MOK-009` **at the release candidate**, which is a different commit from the one
its own verification record binds.

It is not a re-verification. `VER-MOK-008`'s scenarios were performed once, against
`d35f817`, and `VREC-MOK-008` is the record of that. Nothing here reopens or supersedes it.

## The work order and its own record

| | |
| --- | --- |
| Work order | `WO-MOK-009` — Implement the release authorization gate, the release process and the compiler declaration |
| Status | `implemented` |
| Requirements implemented | 5 |
| Verification definition | `VER-MOK-008` |
| Verification record | `VREC-MOK-008` (`verified`) |
| Commit that record binds | `d35f8172a0f91049aa2719bc34ca9dd7584f4380` |
| Verified at | 2026-08-19T14:11:54Z |
| Evidence paths that record binds | 18 |

## Containment in the release candidate

The commit `VREC-MOK-008` binds is an ancestor of the release candidate:

```console
$ git merge-base --is-ancestor d35f8172a0f91049aa2719bc34ca9dd7584f4380 \
    79512b4aba0a444c23605157bc4da4f2f7eb435b
$ echo $?
0
```

Exit `0` is the whole claim: the verified work is present in what is being released, and the
candidate is not a rebuild or a re-implementation of it.

## What changed after that record was verified

Between `d35f817` and the candidate there are **17 commits**, of
which **2** tracked Rust files, manifests or the lockfile differ:

```console
$ git rev-list --count d35f817..79512b4
17
$ git diff --name-only d35f817 79512b4 \
    -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml' | wc -l
2
```

A non-zero count is expected and is not a defect. Each of the eight records in this release binds a
different commit, so all but the last necessarily have work layered on top of them — that is the
condition an aggregate record exists to reconcile, and it is why a release cannot simply cite the
per-work-order records and stop.

**This work order needs more care than the other seven, and the reason is disclosed here rather
than inferred.** Two files inside its declared change surface were edited after `VREC-MOK-008` was
verified, both on the repository owner's instruction of 2026-08-19, and `VREC-MOK-008` does not
cover either edit:

| File | Change | Commit |
| --- | --- | --- |
| `docs/RELEASE_RUNBOOK.md` | +61 / -20 | `b30b9e3` — corrected the eligible work-order set, which had gone stale |
| `scripts/test_check_release_authorization.py` | +41 / -5 | `30a6550` — made the A5 fixture state its own precondition instead of inheriting it |

Both fall inside the surface `WO-MOK-009` declares — "the authorization gate and its scenario
suite" and "the documented human procedure, under `docs/` and outside `docs/engineering/`" — and
both were recorded as stated deviations when they were made. A third edit is in the commit this
file belongs to: the same runbook's Phase D, whose worked example was not executable, for the
reason `README.md` in this directory sets out.

The 2 Rust files that differ since `d35f817` are a separate matter and are
already covered: they are `mokiterions-tui/src/render.rs` and `mokiterions-tui/tests/render.rs`,
which is `WO-MOK-007`'s colour banding, verified by `VREC-MOK-007` and in this selection. So the
Rust delta under this record needs no argument; the three corrections above are the part that does.

So the honest statement is not that `WO-MOK-009` is unchanged since it was verified. It is that
`VREC-MOK-008` verified the release machinery statically and against constructed fixtures at
`d35f817`, that three documentation-and-test corrections have landed since, that none of
them touches `scripts/check_release_authorization.py` or `.github/workflows/release.yml` — the gate
script and the workflow whose behaviour was verified are byte-identical to what `VREC-MOK-008`
bound — and that the gate's own two suites pass at the candidate, 48 and 22 cases. This aggregate
record is where those three edits are carried, because no other record covers them.

The remaining gap is the one `VREC-MOK-008` already stated about itself and this record does not
close: **the release process still has never been run.** Running it is what this release does.

## Renewed selection at the candidate

```console
$ python -m se_harness preflight --work-order WO-MOK-009 --phase review .
Harness preflight: PASS
Phase: review
Work order: WO-MOK-009 (implemented)
```

Exit `0`. Preflight is derived, read-only evidence: it confirms this work order still selects
cleanly against the artifact graph at the candidate — its reading manifest resolves, its relations
resolve, and its assurance classification is present. It does not verify work and does not approve
anything.

## What this file does not claim

- It does not claim `VER-MOK-008` was re-run at the candidate. It was not.
- It does not claim the files `VREC-MOK-008` binds are unchanged. The counts above say otherwise where
  they are non-zero.
- It does not carry a status decision. `VREC-MOK-009` is `ready` when it is captured, and only an
  accountable assurance owner may transition it.

What the release rests on, for this work order, is the pair: `VREC-MOK-008` verified its scope at an
ancestor commit, and the candidate's own checks — recorded in `candidate-checks.md` beside this
file — pass on the tree being released.
