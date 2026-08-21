# `master` merged into this branch — the evidence a record binds

Merge commit: `9599c0a91bb2b6e183bce3a5e82b570d547594f8`.
Parents: `4a32a95` (this branch) and `6b02573` (`origin/master`). Merge base: `ff3a155`. Date: 2026-08-20.

**This directory is what `VREC-MOK-015` binds, and it is the first time this repository has bound a merge commit.**
`evidence/WO-MOK-011/merge/README.md` is the precedent, and it is a precedent in both directions: it recorded a merge
in the same shape as this one and then said of itself that it is *"not a verification record and takes no decision"*,
that `VREC-MOK-011` *"is bound to `9ddcf83` and stays bound to it"*, and that *"a record for the merge is a new
record."* That record was never written. `VER-MOK-014:353` carries the same item as an open residual — *"the
verification record owed against the merge commit"* — and `WO-MOK-014-merge.md` restates it. `VREC-MOK-015` is that
record. It names commit `9599c0a`, it lists every file here, and it does **not** supersede `VREC-MOK-014`: that record
is `verified`, the template permits supersession only of a `ready` record, and it stays bound to `65ac88b`.

What is still not decided here: `VREC-MOK-015` is `status = "ready"`. Moving it to `verified` is the accountable
assurance owner's act, on the evidence in this directory, and nothing in this directory takes it.

## Why every oracle was re-run rather than carried over

`contract-identity.txt` answers the question a reader needs answered before reusing any figure from `VREC-MOK-014`:
the merge changed no build input, no line of the checking program's logic, and no line of either declared set. On that
showing the figures could have been argued forward from a diff. They were re-derived instead, for the reason
`determinism.txt` states in its own case: the cheap way to be wrong is to reason from a diff of the paths one thought
to list. Two of the five oracles could not have been argued forward at all — oracle 3 because byte-identical replay is
a measurement and not an inference, and oracle 5 because the merge is the one act on this branch that edited amendment
records.

## What was re-derived at `9599c0a`, and the figures

| Oracle, row or gate | Re-derived | Result |
|---|---|---|
| 1 — the resolved graph, read from Cargo | **yes** | observer external crates **57 / 63 / 62**, union **66**, `--target all` **71**; engine **1** crate on every target; `cargo metadata --locked` **182** packages with `syn 1.0.109` and `thiserror 1.0.69` reaching no build. `WO-MOK-014-graphs.txt`, `WO-MOK-014-counts.txt` |
| 1 — features and build scripts | **yes** | `ratatui 0.30.2` resolves with `crossterm`, `layout-cache`, `underline-color` and implied `std`, with `default` and `serde` absent; build scripts **7 / 9 / 9**, **10** in union. `WO-MOK-014-features.txt`, `WO-MOK-014-build-scripts.txt` |
| 2 — the declaration, read from the specification text | **yes** | `check_declared_dependencies.py` exit **0**: *"Every declared set matches its resolved graph. 8.4a-8.4d pass."* Its own unit suite: **126 tests, OK**. `WO-MOK-014-check-run.txt`, `WO-MOK-014-check-tests.txt`, and `contract-identity.txt` §3 for the section-text comparison |
| 3 — determinism, byte-identical replay | **yes** | 90 cells twice: run A **==** run B **==** `WO-MOK-011`'s retained post-change manifest **==** this packet's manifest at `65ac88b`, on raw hash and exit code, all 90. `determinism.txt`, `replay-manifest.txt` |
| 4 — the independently written prohibition list | **yes** | the same 126 terms over the union graph; raw hits exactly `mio 1.2.2` and `signal-hook-mio 0.2.5`, Linux and macOS only, both disclosed and accepted; residue none. `WO-MOK-014-scan.txt` |
| 5 — the governance state of what this change amends | **yes**, and re-coordinated | all twelve governed amendments present and `approved`; the four with no authority present and consistent; **21** cited coordinates resolved by content, 14 unmoved, 2 corrected in place, 3 moved by the merge, **2 stale before it**. `amendments.md` |
| The refusal demonstration | **yes** | ten cases, ten refusals, **0** cases that passed when they should have refused; byte-identical to the retained capture but for two identifier lines. `WO-MOK-014-injection.txt` |
| Offline resolution, build and test of the engine | **yes** | both commands exit **0** under `CARGO_HTTP_PROXY=http://127.0.0.1:1`, with the control `cargo search` failing at **101**; `Cargo.lock` sha256 `4154a6f1…` before and after, the same value the packet recorded. `WO-MOK-014-offline-build.txt` |
| The compiled gates | **yes** | fmt clean, clippy clean with both crates touched first, `cargo test --workspace --locked` **226 passed / 0 failed / 0 ignored** over 21 targets — observer 141, engine 85 — which is what `SPEC-MOK-004` rule 11 states. `gates.txt` |
| The harness gates | **yes**, in two columns | all five PASS at exit 0 in both. At the candidate clean: **114 artifacts / 396 relations / 0 errors / 10 warnings**, snapshot `b3470146…`, `decision_required` empty. With this act in the tree: 115 / 398 / 0 / 11, `decision_required` holding `VREC-MOK-015 [ready]`. `doctor` **81 PASS, no FAIL**; `preflight` identical in both. `harness.txt` |
| The contract's subject | **yes** | build inputs, the checking program, both workflows and both declaring sections compared `65ac88b` → `9599c0a` by blob identity or section text. `contract-identity.txt` |

Nothing in the union of those figures moved a dependency, a version, a feature, a count, a build script or a
comparison line. Two things moved and both are named where they occur: the workspace test total, **212 → 226**, where
the 14 are `master`'s observer tests arriving with the only source change the merge brings — `mokiterions-tui`, six
files — and wall-clock timings, which no contract row reads.

## Manual assessments: what carries over and what does not

The contract's assessments are judgements, not measurements, so re-running a command cannot renew one. What
`contract-identity.txt` establishes is that each judgement's **subject** is unchanged at this commit:

- **1** (no declared entry implements simulation semantics) and **2** (decision 1's criteria applied to each entry) —
  recorded 2026-08-20 by the technical owner and carried over, because the declared sets are byte-identical: one entry,
  `ratatui 0.30.2`, on each side.
- **3** (the reach of the by-name scan) — recorded by the assurance owner and carried over, because the term list, its
  provenance and the scan's output are unchanged.
- **4** (the determinism baseline substitution) — unchanged, and this packet strengthens what it stands on: the
  substituted baseline is compared against here and reproduces on all 90 cells.
- **6** (the disclosed transitive capabilities) — recorded, accepted, and carried over: the two disclosed hits are the
  two measured hits at this commit, on the same two targets, with the same versions.
- **5** does not exist in this contract, as `assurance-decision.md` records.
- **7** (the strength the change gives up) is **still found not due** rather than satisfied. It falls due at the first
  admission of a crate beyond `ratatui`, and this merge admits none.
- **`VER-MOK-011`'s fifth manual assessment is untouched and still owed.** It belongs to another contract and no
  measurement here reaches it.

## Reported rather than fixed

1. **The three captures report one, two and three `git status --porcelain` entries, in that order.** Each is true at its
   own capture time, and the order is the order this act accumulated: `gates.txt` was taken before
   `WO-MOK-014-merge.md` was corrected, `WO-MOK-014-offline-build.txt` after that correction, and `harness.txt` last,
   with `VREC-MOK-015` written. Every entry in all three is content of the commit that carries this directory. No
   tracked file the compiled gates read differs from `9599c0a` in any of the three, which is the property those lines
   exist to assert. No file is edited to match another.
2. **The injection script here differs from the retained one on one line.** `root` walks one directory further up,
   because this copy sits one level deeper, and the run instruction in its header names the new path. Nothing else
   differs: the ten cases, their anchors, their expectations and the filter are byte-identical, and so is the output
   they produce but for the two identifier lines the renumbering moved.
3. **`WO-MOK-014-offline-build.txt` here is emitted by a script and the retained one was composed by hand.** The
   commands are the retained file's commands unchanged; `offline-build.sh` beside it is the whole of what produced this
   copy, so the comparison between the two is a comparison of measurements rather than of two transcriptions.
4. **Two coordinates cited in `WO-MOK-014-amendments.md` were already stale before this merge**, one of them at the
   very commit `VREC-MOK-014` binds. They are corrected in `amendments.md` here and **not** in the file that carries
   them, on the precedent `evidence/WO-MOK-012/amendment-ratifications.md` sets for `VREC-MOK-005`: a commit-bound
   record is not edited, because what it says was true of the tree it was taken from. The amendments they point at are
   present and approved; only the line numbers beside them are wrong.
5. **No digest of this directory's own commit exists.** `harness.txt`'s dashboard reading is a figure of this commit's
   content measured while `HEAD` was still `9599c0a`, and it says so. `VREC-MOK-015`'s `artifact_snapshot_sha256` is
   the digest of `9599c0a` itself, taken in a clean checkout, which is a different and stable figure.
6. **`VREC-MOK-015`'s prose was finished after `harness.txt` was taken, and the digest did not move.** The harness row
   of its gates table could only be filled once the capture existed. Re-running the dashboard after that edit returned
   `27c27144…` again, unchanged, because `build_snapshot` hashes the normalized front matter, the relations and the
   findings and not artifact prose. The evidence list in that front matter — including `harness.sh`, added when this
   file was written — was fixed **before** the capture, so `harness.txt`'s column B is a figure of the final content.
7. **This record's presence raises `W-REV-004`, and nothing here closes it.** The inspector reports *"`VREC-MOK-015` is
   ready but its work is fully covered by verified or released records; review possible supersession without inferring
   authority"* and suggests `review-verification-supersession` to the assurance owner. It is a correct question: one
   work order now carries two records. Why supersession is unavailable — `VREC-MOK-014` is `verified`, and the template
   permits superseding a `ready` record only — is stated in that record and in this file, and stating it is not
   closing the finding.

## Files here

| File | What it is |
|---|---|
| `README.md` | this note |
| `contract-identity.txt` | whether the contract is still about the same tree: build inputs, the checking program, both workflows, both declaring sections |
| `gates.txt` | fmt, clippy, the workspace test run at 226, the checking program's own 126 unit tests, and `git rev-parse`/`git status` at the merge commit |
| `harness.sh` | what emitted `harness.txt`, in two columns: the candidate commit clean, and this checkout with the act present |
| `harness.txt` | `validate`, `dashboard`, `inspect`, `doctor`, `preflight` — taken last, after `VREC-MOK-015` was written |
| `determinism.txt` | oracle 3 at this commit: two runs of the 90-cell matrix, compared three ways |
| `replay-manifest.txt` | run A's 90 cells as `name hash exit code`, sorted |
| `amendments.md` | oracle 5 at this commit, with all 21 cited coordinates re-derived by content |
| `WO-MOK-014-capture.sh` | a byte-identical copy of the retained capture script, run here |
| `WO-MOK-014-graphs.txt` | the per-target resolved graphs for both packages |
| `WO-MOK-014-counts.txt` | the figures a reader might confuse, each with the question it answers |
| `WO-MOK-014-features.txt` | the resolved feature set, including the negative assertions |
| `WO-MOK-014-build-scripts.txt` | the build-script crate set per target, against the declared table |
| `WO-MOK-014-scan.txt` | oracle 4: the terms, the raw hits before any disclosure, the disclosed set, the residue |
| `WO-MOK-014-check-run.txt` | the checking program's full run on the real root |
| `WO-MOK-014-check-tests.txt` | its 126 unit tests |
| `WO-MOK-014-injection.sh` | the refusal harness, one line changed for its new depth |
| `WO-MOK-014-injection.txt` | its output: ten cases, ten refusals |
| `offline-build.sh` | what emitted the offline capture |
| `WO-MOK-014-offline-build.txt` | the engine built and tested offline with the registry unreachable |

Two files outside this directory belong to the same act and are in `VREC-MOK-015`'s evidence list:
`../WO-MOK-014-merge.md`, which records the conflicts and their resolution and is corrected in four places here, and
`../WO-MOK-014-renumbering.md`, which is why the header lines of the re-derived captures differ from the retained ones.
Neither is in `VREC-MOK-014`'s list — they did not exist when it was written.

## What is still owed

1. **`VREC-MOK-015` from `ready` to `verified`** — the accountable assurance owner. The evidence is here; the decision
   is not. The same owner holds the `W-REV-004` supersession question the inspector raises, which is a second act and
   not a consequence of the first.
2. **`VER-MOK-011`'s fifth manual assessment** — assurance owner. Still without an author, as
   `evidence/WO-MOK-011/merge/README.md` recorded on 2026-08-19 and as `VER-MOK-014:352` restates.
3. **`VER-MOK-014`'s manual assessment 7** — technical owner, at the first admission of a crate beyond `ratatui`. Found
   not due, not satisfied.
4. **Pull request #33** — the repository owner. Four of its body's statements are overtaken by the three commits after
   it was last edited, and the body is not edited here.
5. **The `VER-MOK-005` / `VER-MOK-008` rewording** — a separate work order after #33 merges, as the owner directed. No
   part of it is done here.
