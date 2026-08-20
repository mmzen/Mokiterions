+++
id = "VREC-MOK-015"
type = "verification_record"
title = "Verification candidate for WO-MOK-014 at the merge commit"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-20"
commit = "9599c0a91bb2b6e183bce3a5e82b570d547594f8"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-20T19:46:51Z"
artifact_snapshot_sha256 = "b3470146ac4facaa77b6863a672c7ee04304c7de973f99d084183309a8b4b912"
evidence_paths = [
  "docs/engineering/simulation/evidence/WO-MOK-014/WO-MOK-014-merge.md",
  "docs/engineering/simulation/evidence/WO-MOK-014/WO-MOK-014-renumbering.md",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/README.md",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-build-scripts.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-capture.sh",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-check-run.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-check-tests.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-counts.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-features.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-graphs.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-injection.sh",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-injection.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-offline-build.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-scan.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/amendments.md",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/contract-identity.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/determinism.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/gates.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/harness.sh",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/harness.txt",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/offline-build.sh",
  "docs/engineering/simulation/evidence/WO-MOK-014/merge/replay-manifest.txt",
]

[relations]
verifies_work_order = ["WO-MOK-014"]
conforms_to = ["VER-MOK-014"]
+++

# Verification Record Candidate

This record names commit `9599c0a91bb2b6e183bce3a5e82b570d547594f8` — the merge of `origin/master` into this branch —
as a candidate satisfying `VER-MOK-014`, and the evidence in
`docs/engineering/simulation/evidence/WO-MOK-014/merge/` as the evidence for it.

**It is the first record in this repository bound to a merge commit, and the item it discharges was written down twice
before it existed.** `evidence/WO-MOK-011/merge/README.md` recorded a merge of the same shape on 2026-08-19, said of
itself that it is *"not a verification record and takes no decision"*, that `VREC-MOK-011` *"is bound to `9ddcf83` and
stays bound to it"*, and that *"a record for the merge is a new record"* — and no such record followed. `VER-MOK-014`
carries the same item as its last residual: *"the verification record owed against the merge commit"*. This is that
record, for this merge. It is not that record for `WO-MOK-011`'s merge, which is still owed and belongs to that chain.

**It is `ready`, and it decides nothing.** Moving it to `verified` is the accountable assurance owner's act on the
evidence listed above. Nothing here takes that act, and no part of this record's preparation was an acceptance.

## The front matter is the capture's provenance, not a decision's

Every field above is a measurement, and three of them were measured in a way that has to be stated because the harness
tool would not produce them.

**`harnessctl capture-verification` refused to emit this record, twice, and both refusals are correct.** Run in this
checkout it stops with `harnessctl: revision provenance requires a clean Git worktree`: the worktree necessarily holds
the untracked evidence packet and the corrected `WO-MOK-014-merge.md`, because both are in this record's own commit. Run
in a clean checkout of the candidate it stops with `harnessctl: evidence file does not exist`, because at `9599c0a` the
evidence this record binds is not committed yet. The tool's pattern is the one `VREC-MOK-014` followed — evidence
committed first, record captured from a clean tree afterwards — and following it here would bind the record to a commit
whose only content beyond this merge is this record's own evidence, rather than to the merge.

So the front matter was composed by hand and each field was measured directly:

| Field | How it was measured |
|---|---|
| `commit` | `git rev-parse HEAD` at the merge commit, which was already committed and pushed before any figure below was taken |
| `worktree_state` | `git status --porcelain` in a detached worktree checked out at `9599c0a`: **0 entries**. In this checkout the same command returns the packet and one modification, both of which belong to the commit that carries this record and neither of which is a tracked difference from `9599c0a` in anything the contract reads |
| `verified_at` | the capture timestamp, as the template names it. It is not an approval time: `status` is `ready` |
| `artifact_snapshot_sha256` | `python scripts/generate_harness_dashboard.py` in that same detached worktree, whose leaf directory name equals this repository's because `build_snapshot` puts the checkout's directory name into the hashed document. **PASS, 114 artifacts, 396 relations, 0 errors, 10 warnings, snapshot `b3470146…`**, and `sha256sum target/harness-dashboard/dashboard-data.json` equal to it |

The precedent for a hand-composed record front matter re-measured at the commit it names is `VREC-MOK-014`'s own
re-point, which edited its front matter and re-measured both `65ac88b` figures *"from a clean checkout at that commit"*.

## What this record claims

### The candidate commit

| | |
|---|---|
| Commit | `9599c0a91bb2b6e183bce3a5e82b570d547594f8` |
| Branch | `governance/adr-mok-006-third-party-crates`, pushed to `origin` as `3c3c2e4..9599c0a` |
| First parent | `4a32a95a173b95f88fbc4bf93f3ad2e04ac35ee6`, this branch's renumbering of the `013` chain to `014` |
| Second parent | `6b02573cacde0c58cbd8a74d86a23010b6f180cb`, `origin/master` |
| Merge base | `ff3a155f3ce006fdc38abb62df3fca4a2c3c3aa3`, the tip of `master` this branch was cut from |
| Worktree | clean at the commit; `Cargo.lock` byte-identical to `65ac88b` and to `origin/master` |
| Contents relative to the first parent | 71 files, +26,871 / −281. Six are source — `mokiterions-tui/src/{layout,render,verification}.rs` and their three test files, +1,487 / −203 — and they are `master`'s. 65 are governance documents. **Zero** files under `mokiterions-core/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `.github/` or `scripts/` |

**Why the candidate is the merge commit and not the record's own commit.** The template requires this record to be
committed after the candidate it names, and `WORKFLOW.md`'s rule that a record cannot carry the hash of its own commit
is the same rule seen from the other side. `9599c0a` existed and was pushed before any measurement below was taken,
which is what `gates.txt` records with `git rev-parse HEAD` beside `git status --porcelain`.

**Why `VREC-MOK-014` is not re-pointed to it instead.** That record is `verified`. The template permits supersession of
a `ready` record only, and re-pointing a verified record would move a decision the owner took about a specific tree onto
a different tree. `evidence/WO-MOK-011/merge/README.md` had already settled the question in words this record adopts: a
record for the merge is a new record.

### The gates, measured at this commit

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, no warning line, both crates touched first so neither reports a cached result |
| `cargo test --workspace --locked` | exit 0 — 21 `test result:` lines, **226 passed, 0 failed, 0 ignored, 0 filtered out**; observer 141, engine 85 = 54 lib + 31 integration + 0 in the binary. `SPEC-MOK-004` rule 11 states 141 / 85 / 226 |
| `python scripts/check_declared_dependencies.py --root .` | exit 0 — *"Every declared set matches its resolved graph. 8.4a-8.4d pass."*, with `disclosed and accepted` printed for `mio 1.2.2` and `signal-hook-mio 0.2.5` |
| `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 126 tests`, `OK`, exit 0 |
| `CARGO_HTTP_PROXY=http://127.0.0.1:1 cargo build -p Mokiterions --locked --offline` | exit 0, with the control `cargo search` failing at exit 101 under the same proxy |
| `CARGO_HTTP_PROXY=http://127.0.0.1:1 cargo test -p Mokiterions --locked --offline` | exit 0 — 85 tests, `Cargo.lock` sha256 `4154a6f1…` before and after, `git status --porcelain` unchanged across both commands |
| `python scripts/validate_engineering_artifacts.py --root .` | **PASS**, exit 0 — 0 errors, 0 warnings, all four planes `E0/W0`. **114 artifacts** at the candidate clean, **115** with this record present |
| `python scripts/generate_harness_dashboard.py --root .` | **PASS**, exit 0 — at the candidate clean **114 artifacts / 396 relations / 0 errors / 10 warnings**, snapshot `b3470146…`, the figure this record declares; with this record present 115 / 398 / 0 / 11, snapshot `27c27144…`. In both columns the printed snapshot equals `sha256sum target/harness-dashboard/dashboard-data.json` |
| `python scripts/inspect_engineering_artifacts.py --root .` | **PASS**, exit 0 — with this record present `decision_required` holds exactly one entry, `VREC-MOK-015 [ready] assurance-review`; `active_work` and `assurance_pending` are empty; `definitions_pending` holds only `WO-MOK-008 [draft]`, which predates this chain |
| `python -m se_harness doctor .` under the pinned 0.4.0 venv | exit 0 — **81 `PASS` lines, no `FAIL`**, byte-for-byte the same list in both columns: every managed file unchanged, every distribution file matched, lock and config present |
| `python -m se_harness preflight . --work-order WO-MOK-014 --phase review` | **PASS**, exit 0, identical in both columns — `WO-MOK-014 (implemented)`, commit-bound verification **required**, decided by the engineering owner |
| `git rev-parse HEAD` before and after every check | `9599c0a…`, with no tracked file differing from it |

The compiled figures are in `merge/gates.txt`, the offline pair in `merge/WO-MOK-014-offline-build.txt`, and the harness
five in `merge/harness.txt`, taken last and in two columns — the candidate with nothing of this act in the tree, and
this checkout with all of it — so that the difference between the columns is this act, measured. The one figure that
moved from `VREC-MOK-014`'s reading is the workspace test total, **212 → 226**: the 14 are `master`'s observer tests,
arriving with the six source files above.

**This record's presence raises one new warning, and this record does not close it.** `W-REV-004` — *"`VREC-MOK-015` is
ready but its work is fully covered by verified or released records; review possible supersession without inferring
authority"* — names both records and suggests `review-verification-supersession` to the assurance owner. The harness is
right to ask, because one work order now carries two records; the reason supersession is not available is stated below,
and stating it is not the same as closing a finding whose own words are *not to infer authority*. `I-REV-001` reads 14
observations in both columns only because this record's declared commit is the observed revision at the moment of
capture; from the commit that carries this file onward it reads 15, and `merge/harness.txt` says so where the numbers
are printed.

### The five oracles, re-derived rather than carried over

`VER-MOK-014`'s *Independence* section names five. `merge/contract-identity.txt` establishes that the merge changed no
build input, no line of the checking program's logic and no line of either declared set, so each figure could have been
argued forward from a diff. Each was measured again instead.

| Oracle | Retained in | What it returned at this commit |
|---|---|---|
| 1 — the resolved graph read from Cargo, never from a manifest or lockfile | `merge/WO-MOK-014-graphs.txt`, `-counts.txt`, `-features.txt`, `-build-scripts.txt` | engine: 1 node, 0 external crates, all three release targets. Observer: **57** on `x86_64-pc-windows-msvc`, **63** on `x86_64-unknown-linux-gnu`, **62** on `aarch64-apple-darwin`, **66** in union, **71** at `--target all`, against **182** packages in `cargo metadata --locked`; `syn 1.0.109` and `thiserror 1.0.69` reachable from neither package. Features: `crossterm`, `layout-cache`, `underline-color` plus implied `std`, with `default` and `serde` absent. Build scripts **7 / 9 / 9**, **10** in union |
| 2 — the declaration read from the specification text, not restated in code | `merge/WO-MOK-014-check-run.txt`, `-check-tests.txt` | both directions equal for both packages, exit 0; the program's own suite `Ran 126 tests, OK`. `contract-identity.txt` §3 compares the two declaring sections `65ac88b` → `9599c0a`: 8 differing lines between them, every one accounted for by three identifier substitutions, no line added or removed, and `SPEC-MOK-005` rule 15 byte-identical |
| 3 — a retained pre-existing capture as the determinism baseline | `merge/determinism.txt`, `merge/replay-manifest.txt`, `merge/WO-MOK-014-capture.sh` | 90 cells captured twice at this commit — 5 seeds × 3 policies × 3 densities × 2 trace modes at 1,000 ticks. **run A == run B**, **run A == `WO-MOK-011`'s retained post-change manifest**, and **run A == this packet's manifest taken at `65ac88b`** — all 90 cells, raw hash and exit code, three ways. The four contract-named seed-`123` cells reproduce exactly: `baseline 0.75%` `fcd03d6f…`, `baseline 1.50%` `44a448a1…`, `reference 0.75%` `cebe44c4…`, `reference 1.50%` `9621f5f8…` |
| 4 — an independently written prohibition list | `merge/WO-MOK-014-scan.txt` | the same 126 terms over the union graph. **Raw hits before any disclosure: exactly `mio 1.2.2` and `signal-hook-mio 0.2.5`, on Linux and macOS.** Residue after disclosure: none |
| 5 — the governance state of what this change amends | `merge/amendments.md` | all sixteen entries of `ADR-MOK-006`'s *Required amendments* present; the twelve landing in a governed artifact `approved`; the four with no authority present and consistent; `ARCH-MOK-001` carrying both replacement conformance checks at unchanged lines. **21 cited coordinates resolved by content**: 14 unmoved, 2 corrected in place by the merge, 3 moved by it, and **2 stale before it** |

Oracle 3 is the one no diff could have settled, and oracle 5 is the one the merge could most easily have broken: this
merge is the only act on this branch that edited amendment records, because `master` ratified four rows that four of
this branch's amendment sentences asserted were still outstanding.

Two demonstrations the contract asks for by name were re-derived too:

- **`merge/WO-MOK-014-injection.txt` — the check refusing, ten ways.** Ten cases, ten refusals, and **0 cases that
  passed when they should have refused**. The output is byte-identical to the retained capture apart from two lines
  carrying the old chain identifier. The harness copy here differs from the retained script on one line, the depth of
  its walk to the repository root, because it sits one directory deeper.
- **`merge/WO-MOK-014-offline-build.txt` — the engine offline.** Both commands succeed with the registry unreachable,
  from the committed lockfile, with the lockfile byte-identical afterwards and the working tree unchanged. It is emitted
  by `merge/offline-build.sh`, running the retained file's commands unchanged, so the comparison against the retained
  capture is a comparison of measurements rather than of two transcriptions.

### The manual assessments

`VER-MOK-014` declares six, numbered 1, 2, 3, 4, 6 and 7 with no fifth. A judgement is not renewed by re-running a
command, so what this record claims about them is narrower: their **subject** is unchanged at this commit, and
`contract-identity.txt` is the measurement that says so.

| # | Assessment | Role | State at this commit |
|---|---|---|---|
| 1 | No declared entry implements simulation semantics | technical owner | Recorded 2026-08-20 and **carried over**: the declared sets are byte-identical, one entry each side, `ratatui 0.30.2` |
| 2 | Decision 1's criteria applied to each entry | technical owner | Recorded 2026-08-20 and **carried over**: same entry, same version, same features, and the measured debt it names — 57/63/62 crates, 7/9/9 build scripts, the disclosed `mio` capability — reproduces here |
| 3 | The reach of the by-name scan | assurance owner | Recorded 2026-08-20 and **carried over**: the term list, its provenance and the scan's output are unchanged |
| 4 | The determinism baseline substitution | assurance owner | Unchanged, and better supported here than at `65ac88b`: the substituted baseline is compared against on all 90 cells |
| 5 | — | — | Does not exist |
| 6 | The disclosed transitive capabilities | technical owner | Accepted 2026-08-20 and **carried over**: the two disclosed hits are the two measured hits at this commit, same versions, same two targets, and the acceptance's limit — a compiled and uncalled capability — is unchanged |
| 7 | The strength this change gives up | technical owner | **Still found not due, not satisfied.** It falls due at the first admission of a crate beyond `ratatui`, and this merge admits none |

**`VER-MOK-011`'s fifth manual assessment is untouched and still owed.** It belongs to another contract, no measurement
here reaches it, and this record does not narrow it.

## The snapshot figure, and what it is a figure of

`artifact_snapshot_sha256` is `b3470146ac4facaa77b6863a672c7ee04304c7de973f99d084183309a8b4b912`: the digest of the
governed graph of commit `9599c0a` exactly as that commit stands — **114 artifacts, 396 relations, 0 errors, 10
warnings, PASS** — measured in a detached worktree checked out at that commit, with `git status --porcelain` returning
nothing and the worktree's leaf directory name equal to this repository's.

**Here the as-committed figure and the record-absent figure are the same number, and that is a property of this record
rather than a general rule.** `VREC-MOK-014` had to publish two figures for `65ac88b` — `a12ec1a3…` with the record held
out of the tree and `dcb28212…` with it present — because it already existed at the commit it binds. This record does
not exist at `9599c0a`, and neither does the evidence packet it lists: both arrive in the commit that carries this file.
So one figure serves, and it is invariant under every later edit to this record for the reason `VREC-MOK-014` records —
`build_snapshot` hashes the normalized front matter, the relations and the findings, and not artifact prose.

**No digest is given for the commit that carries this record, and none can be.** `build_snapshot` writes
`git rev-parse HEAD` into the hashed document in 25 places, so that digest does not exist until the commit does, and a
figure measured beforehand would be a working-tree figure presented as a commit figure — the error `VREC-MOK-014`'s
*The snapshot figures* section documents in three recorded values. `merge/harness.txt`'s dashboard reading is exactly
such a working-tree figure, taken with `HEAD` still at `9599c0a` and this record's content present, and it says so
where it is printed. `merge/WO-MOK-014-merge.md` carried two figures of the same kind, `da19c004…` and `348251422…`,
both measured while `HEAD` was still `4a32a95`; they are corrected in that file in this commit, with the correction
recorded rather than the figures deleted.

## What this record does not claim

- **It does not verify `master`'s observer work.** Six source files and 14 tests arrive with this merge and they are
  `master`'s; `master`'s own verification record covers them. `VER-MOK-014` claims nothing about the observer's
  rendering, and `merge/determinism.txt` states this limit where a reader might mistake replay equality for coverage of
  the thing that changed.
- **It does not discharge `VREC-MOK-002`**, which stays bound to the commit and the content it was written for, nor
  does it edit the four 2026-08-17 replay hashes that record's evidence carries.
- **It does not supersede `VREC-MOK-014` and cannot.** That record is `verified`; supersession may move a `ready`
  record only. Both records stand: one work order with two records, bound to two commits, each with its own evidence
  and its own reading.
- **It closes no assessment.** Assessments 1, 2, 3, 4 and 6 are carried over with their subject shown unchanged, not
  re-taken; assessment 7 is found not due; `VER-MOK-011`'s fifth is owed to another contract.
- **It ratifies no amendment.** Four amendment sentences this branch wrote asserted that a 2026-08-18 provision was
  still outstanding. The repository owner ratified those provisions under `WO-MOK-012` on 2026-08-20, which is that
  work order's act; this merge only corrected the sentences that the ratification made false, and `merge/amendments.md`
  says so.
- **It corrects no committed record.** Two coordinates cited in `WO-MOK-014-amendments.md` are stale — one of them
  already stale at `65ac88b`, the commit the verified record binds. The true coordinates are recorded in
  `merge/amendments.md`; the file that carries the stale ones is not edited, on the precedent
  `evidence/WO-MOK-012/amendment-ratifications.md` states for `VREC-MOK-005`.
- **It takes no release decision and no merge decision.** Pull request #33 is the repository owner's, its body is not
  edited here, and nothing in this record makes any commit release-eligible.
- **It is not an assurance decision.** `status` is `ready`. What it would accept, if the accountable assurance owner
  accepts it, is the evidence enumerated in `evidence_paths` as satisfying `VER-MOK-014` at commit
  `9599c0a91bb2b6e183bce3a5e82b570d547594f8`, and nothing beyond it.

## What had to happen before this record could be captured, and what still stands

Three acts preceded it, each on the repository owner's instruction and each recorded where it happened: the merge
conflict was resolved and the colliding identifiers renumbered inside the existing work order; the merge was committed
as `9599c0a` and pushed; and the contract was re-derived at that commit into the packet this record lists. The order
matters — the candidate had to exist before its evidence could be a figure of it, which is the mistake
`WO-MOK-011`'s merge capture disclosed about itself and this one avoids.

What still stands, none of it this record's to take:

1. **This record from `ready` to `verified`** — the accountable assurance owner, who also holds the `W-REV-004`
   supersession question the inspector raises against this record and `VREC-MOK-014`. Two acts, not one.
2. **`VER-MOK-011`'s fifth manual assessment** — assurance owner. Outstanding since 2026-08-19.
3. **`VER-MOK-014`'s manual assessment 7** — technical owner, at the first admission of a crate beyond `ratatui`.
4. **A verification record for `WO-MOK-011`'s merge** — still owed to that chain; this record is not it.
5. **Pull request #33** — the repository owner. Four statements in its body are overtaken by the three commits made
   after it was last edited.
6. **The `VER-MOK-005` / `VER-MOK-008` rewording** — a separate work order after #33 merges, as the owner directed.
