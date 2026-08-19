# Candidate conformance for the 0.1.0 release

Every figure below was measured at candidate commit `79512b4aba0a444c23605157bc4da4f2f7eb435b`, in a
clean clone of the remote, with no tags present, from a pinned `se-harness==0.4.0` wheel. The clone's
checkout directory is named `Mokiterions`, which matters: `artifact_snapshot_sha256` incorporates the
directory basename, so a capture from a differently-named checkout produces a different snapshot hash
for the same tree.

The eight per-work-order files beside this one establish that each work order's verified commit is an
ancestor of the candidate. This file establishes the other half — that the candidate tree itself
passes everything the repository can ask of it.

## Repository checks

These are the four commands `.github/workflows/release.yml` runs in its `verify` job, run here in the
same form, including `--locked`:

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, no warning or error lines |
| `cargo test --workspace --locked` | **179 passed, 0 failed** |
| `cargo tree -p Mokiterions --locked` | one line, one crate — the engine's dependency table is still empty |

The `cargo tree` result is a requirement rather than an observation: the engine package is required
to have no dependencies, and a second line in that output is a release-blocking regression.

## Harness gates

| Gate | Result |
| --- | --- |
| `scripts/validate_engineering_artifacts.py --root .` | **PASS** — 83 artifacts, 0 errors, 0 warnings, all four planes E0/W0 |
| `python -m se_harness doctor .` | exit 0 — **81 PASS**, 0 WARN, 0 FAIL |
| `python -m se_harness preflight --work-order <id> --phase review .` | **PASS for all eight** work orders, exit 0 each |

The eight preflight runs are `WO-MOK-001` through `WO-MOK-007` and `WO-MOK-009`. `WO-MOK-008` is not
among them: it is a `draft` proposal, it is not in this release, and a review preflight against it is
the one that fails — which is the expected answer for a draft, not a defect.

## The gate's own scenario suites

| Suite | Result |
| --- | --- |
| `scripts/test_check_release_authorization.py` | **Ran 48 tests — OK** |
| `scripts/test_check_release_reachability.py` | **Ran 22 tests — OK** |

Neither suite runs in any CI workflow. `.github/workflows/release.yml`'s `verify` job is cargo-only,
and no workflow invokes either file, so these two results exist because they were run here by hand.
That is worth stating plainly: nothing in CI would have caught a regression in them.

## The gate refuses the release today, and that is the correct answer

```console
$ python scripts/check_release_authorization.py --tag v0.1.0
$ echo $?
1
```

The refusal is `REFUSED`, and the first fact it cannot establish is the tag itself — `v0.1.0` is not
a tag in this repository yet. It should refuse until Phase H creates the tag and the release record
reaches `released`. A gate that authorized this release at this moment would be broken.

This is also the check whose fixture defect was corrected in `30a6550`: five scenarios in the
authorization suite used to take their starting state from whatever the repository held when they
ran, so a real `v0.1.0` tag and a `released` release record would have broken them — four as errors
rather than failures. The suite now states its own precondition. That correction is one of the three
post-verification edits recorded in `WO-MOK-009-containment.md`.

## What the commit this file lives in adds

This commit adds documentation only: this directory, and a correction to `docs/RELEASE_RUNBOOK.md`
Phase D. No Rust source, no manifest and no lockfile differs from the candidate, so the four
repository-check results above carry over to it unchanged, and that is checkable rather than asserted:

```console
$ git diff --name-only 79512b4 HEAD -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml'
$ echo $?
0
```

Empty output. The harness gates are a different matter, because they read the documentation tree that
this commit changes, so `VREC-MOK-009` re-states `validate`, `doctor` and both suites as measured at
the commit it actually binds rather than relying on this file.
