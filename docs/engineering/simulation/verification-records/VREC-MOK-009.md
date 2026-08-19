+++
id = "VREC-MOK-009"
type = "verification_record"
title = "Verification candidate for 8 work orders"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"
commit = "755db7297aa993f00d42f9c9794584b5d061f03d"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-19T17:35:20Z"
artifact_snapshot_sha256 = "e38036d92c0e5e7d30a058eed1017948039a112e7ec2ff204842d112295d7e44"
evidence_paths = ["docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-001-containment.md", "docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-002-containment.md", "docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-003-containment.md", "docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-004-containment.md", "docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-005-containment.md", "docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-006-containment.md", "docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-007-containment.md", "docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-009-containment.md", "docs/engineering/simulation/evidence/release-0.1.0/candidate-checks.md"]

[relations]
verifies_work_order = ["WO-MOK-001", "WO-MOK-002", "WO-MOK-003", "WO-MOK-004", "WO-MOK-005", "WO-MOK-006", "WO-MOK-007", "WO-MOK-009"]
conforms_to = ["VER-MOK-001", "VER-MOK-002", "VER-MOK-003", "VER-MOK-004", "VER-MOK-005", "VER-MOK-006", "VER-MOK-007", "VER-MOK-008"]
+++

# Verified Verification Record

This record binds retained evidence for `WO-MOK-001`, `WO-MOK-002`, `WO-MOK-003`, `WO-MOK-004`, `WO-MOK-005`, `WO-MOK-006`, `WO-MOK-007`, `WO-MOK-009` to candidate commit `755db7297aa993f00d42f9c9794584b5d061f03d`, and an accountable assurance owner has transitioned it to `verified`. That decision is recorded at the end of this file. The command that prepared the record approved nothing and did not commit, tag, release or publish.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## What this record claims, and what it does not

This is the first aggregate record in this repository, and the shape of it is stated before the results
because it is what the assurance owner was asked to weigh.

**It is not a re-verification of eight work orders.** Each of `VER-MOK-001` through `VER-MOK-008` was
performed once, against its own commit, and `VREC-MOK-001` through `VREC-MOK-008` are the records of
those performances. None of them is reopened, superseded or edited here. `conforms_to` above lists all
eight verification definitions because the harness requires that set to be exactly the union of the
selected work orders' `verification` relations — **it does not mean their scenarios were re-executed at
this commit. They were not.**

What this record adds is the one thing those eight cannot state about themselves: each binds a
**different** commit, and none binds the commit being released. A release needs exactly one. So this
record claims two things, and only two:

1. For each of the eight work orders, the commit its own `verified` record binds is an **ancestor** of
   `755db72` — the verified work is present in what is being released, not rebuilt.
2. The tree at `755db72` passes every check this repository can run against it.

Together those say: the work was verified, and the thing being released contains it and is itself sound.
That is a weaker claim than "eight work orders were verified at `755db72`", and it is the true one.

## The eight records this one stands on

| Work order | Definition | Its record | Commit that record binds | Ancestor | Rust/manifest delta since |
| --- | --- | --- | --- | --- | --- |
| `WO-MOK-001` | `VER-MOK-001` | `VREC-MOK-001` | `ecd03a8` | yes | 35 files |
| `WO-MOK-002` | `VER-MOK-002` | `VREC-MOK-002` | `68163ac` | yes | 34 files |
| `WO-MOK-003` | `VER-MOK-003` | `VREC-MOK-003` | `a7f39f1` | yes | 34 files |
| `WO-MOK-004` | `VER-MOK-004` | `VREC-MOK-004` | `95f0aa2` | yes | 33 files |
| `WO-MOK-005` | `VER-MOK-005` | `VREC-MOK-005` | `f361370` | yes | 2 files |
| `WO-MOK-006` | `VER-MOK-006` | `VREC-MOK-006` | `dd0c2c0` | yes | 6 files |
| `WO-MOK-007` | `VER-MOK-007` | `VREC-MOK-007` | `dfab77b` | yes | **0 files** |
| `WO-MOK-009` | `VER-MOK-008` | `VREC-MOK-008` | `d35f817` | yes | 2 files |

All eight are `verified`. "Ancestor" is `git merge-base --is-ancestor <commit> 755db72` exiting `0`,
run for each; the per-work-order evidence files show the command and the exit code individually.

The deltas are expected, not defects: eight records bind eight commits, so all but the newest necessarily
have work layered above them. Where a delta is non-zero, the work in it belongs to **another work order
in this same selection**, with its own verified record — the crate split, the workspace split, the
package layout and the colour banding. "Another" rather than "a later one" on purpose: ID order and
commit order do not agree here, as the note below shows. `WO-MOK-007` is the exception in the useful
direction: no tracked Rust file or manifest differs between its verified commit and this one, so the
code that was verified is the code being released, byte for byte.

`VREC-MOK-006` was verified on 2026-08-18, before `VREC-MOK-005`, yet binds an **older** commit.
Verification order and commit order are independent here, which is a second reason a release cannot
infer one candidate from a set of per-work-order records.

## Gates run at this commit

Measured at `755db72` with a clean worktree, in a clone of the remote whose checkout directory is named
`Mokiterions`, from a pinned `se-harness==0.4.0` wheel:

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, no warning or error lines |
| `cargo test --workspace --locked` | **179 passed, 0 failed** |
| `cargo tree -p Mokiterions --locked` | one line, one crate |
| `scripts/validate_engineering_artifacts.py --root .` | **PASS** — 0 errors, 0 warnings, all four planes E0/W0 |
| `python -m se_harness doctor .` | exit 0 — **81 PASS**, 0 WARN, 0 FAIL |
| `preflight --phase review`, all eight work orders | **PASS**, exit 0 each |
| `scripts/test_check_release_authorization.py` | **Ran 48 — OK** |
| `scripts/test_check_release_reachability.py` | **Ran 22 — OK** |
| `scripts/check_release_authorization.py --tag v0.1.0` | exit 1, **REFUSED** |

The refusal is the correct answer and not a failing gate: `v0.1.0` is not a tag in this repository yet.
It must refuse until the release owner creates the tag and this release's record reaches `released`.

Two figures need stating precisely rather than rounded. `validate` counts **83** artifacts at `755db72`
itself and **84** in the worktree that also holds this file, because this record is the artifact being
added — both were observed, and neither is a discrepancy. And the four `cargo` results describe the same
sources as candidate `79512b4`: `git diff --name-only 79512b4 755db72 -- '*.rs' Cargo.toml Cargo.lock
'*/Cargo.toml'` is empty, so `755db72` adds documentation only.

Neither gate suite runs in CI. `.github/workflows/release.yml`'s `verify` job is cargo-only and no
workflow invokes either file, so those two results exist because they were run by hand here.

## What this record does not close

**Three edits inside `WO-MOK-009`'s declared change surface landed after `VREC-MOK-008` was verified, and
no record covered them until this one.** They are carried here because this is where they belong, not
because they were verified again:

| File | Change | Where |
| --- | --- | --- |
| `docs/RELEASE_RUNBOOK.md` | +61 / −20 | `b30b9e3` — the eligible work-order set had gone stale |
| `scripts/test_check_release_authorization.py` | +41 / −5 | `30a6550` — the A5 fixture now states its own precondition |
| `docs/RELEASE_RUNBOOK.md` | Phase D | `755db72` — the worked example was not executable |

All three are documentation or test corrections made on the owner's instruction of 2026-08-19 and
disclosed as stated deviations when made. `scripts/check_release_authorization.py` and
`.github/workflows/release.yml` — the gate script and the workflow whose behaviour `VER-MOK-008`
measured — are **byte-identical** to what `VREC-MOK-008` bound, confirmed by blob hash rather than by
diff summary. The third edit exists because capturing this record refused: the runbook stated the
evidence-keying rule directly above an example that violated it, and
`evidence/release-0.1.0/README.md` records the refusal and the correction.

**The release process has still never been run.** `VREC-MOK-008` said so about itself and this record
does not change it. Every A and R scenario in the gate suite runs against constructed fixtures and
throwaway clones; no tag has ever been pushed, no draft release has ever been produced, and
`.github/workflows/release.yml` has never executed. Running it is what this release does, and the first
real execution is therefore also the first observation of it.

Two further limitations, unchanged and not closed here: the `release` environment that
`SPEC-MOK-005` rule 12.6 names is still unconfigured in repository settings, and
`docs/engineering/REPOSITORY_CONTEXT.md`'s lint command omits `--locked` where the workflow uses it.

## The decision, recorded

`status` is `verified`. The accountable assurance owner made that decision on 2026-08-19, and this
section records it rather than constituting it — the implementation agent transcribed the transition
and exercised no decision right in doing so.

What the decision was taken against is stated so a later reader can judge the decision and not only
the record: the nine evidence files this record binds, the two claims in the first section, and the
explicit statement that the eight underlying verifications were **not** re-run at this commit. The
owner was shown, before deciding, that this record does not close three post-verification edits inside
`WO-MOK-009`'s declared change surface, and that the release process has never been executed. Both are
stated above in the terms the decision accepted them in.

The decision does not extend to the release. `docs/engineering/DECISION_RIGHTS.md` keeps assurance and
release apart: `RLS-MOK-001` reaching `released` is a separate act by the release owner, and nothing in
this record anticipates or supplies it.
