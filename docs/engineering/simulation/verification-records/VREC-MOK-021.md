+++
id = "VREC-MOK-021"
type = "verification_record"
title = "Verification candidate for WO-MOK-008"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-22"
updated = "2026-08-22"
commit = "3da6acca1e8cb53f20ea13869c6f4bc425b979f2"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-22T08:35:53Z"
artifact_snapshot_sha256 = "4cb4044e3462be3820006e2172047def2d8a5bcdf0a56f32e2c92497d2ac9450"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-008/README.md", "docs/engineering/simulation/evidence/WO-MOK-008/commit-states.md", "docs/engineering/simulation/evidence/WO-MOK-008/completion-report.md", "docs/engineering/simulation/evidence/WO-MOK-008/counterfactual.md", "docs/engineering/simulation/evidence/WO-MOK-008/footer-shedding.md", "docs/engineering/simulation/evidence/WO-MOK-008/footer-tier-fallthrough.md", "docs/engineering/simulation/evidence/WO-MOK-008/gates.md", "docs/engineering/simulation/evidence/WO-MOK-008/rendered-footers.md", "docs/engineering/simulation/evidence/WO-MOK-008/replay.md", "docs/engineering/simulation/evidence/WO-MOK-008/verification-mapping.md"]

[relations]
verifies_work_order = ["WO-MOK-008"]
conforms_to = ["VER-MOK-005"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-008` to candidate commit `3da6acca1e8cb53f20ea13869c6f4bc425b979f2`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

*Everything above this line is `capture-verification`'s output verbatim — the front matter, the heading and the two
paragraphs. Everything below is authored, and every figure in it was measured at the candidate rather than carried from
the evidence pack.*

## What is still open

**This record is `ready`. It is not `verified`, and nothing in it asserts that it is.** `WORKFLOW.md` moves a
verification record from `ready` to `verified` through a separate accountable decision by the assurance owner, on the
evidence, and that decision has not been taken. The instruction that produced this record was *"ok, you can (1)
transition the work order to implemented, (2) prepare the verification record as ready, (3) commit, (4) create the PR"*:
the transition is commit `3da6acc`, this record is the preparation, the commit and the pull request are the third and
fourth acts, and **the transition to `verified` is not among the four**. `DECISION_RIGHTS.md` reserves it to the
accountable assurance owner.

The derived queues say the same thing in the harness's own words. At the candidate, `inspect` reports
`assurance_pending` holding exactly **`WO-MOK-008 [implemented] prepare-commit-bound-verification`** and
`decision_required` empty. In the working tree that carries this record, `assurance_pending` **empties** and
`decision_required` holds exactly **`VREC-MOK-021 [ready] assurance-review`** — *"Review retained evidence and record or
withhold the accountable verification decision."* The queue moved from *prepare* to *review*, which is the whole of what
this record accomplishes.

## This record is `VREC-MOK-021` because `VREC-MOK-020` was taken while this work was in progress

The candidate commit was first written naming `VREC-MOK-020`, and that number was checked before it was used: against
the working tree, against every remote ref after `git fetch --all`, and against all fifteen pull requests, where
`VREC-MOK-019` was the maximum. A concurrent session then claimed `VREC-MOK-020` for `WO-MOK-017` at commit `a30ae32`,
transitioned it to `verified`, and merged it to `master` through
**[#41](https://github.com/mmzen/Mokiterions/pull/41)** at `2026-08-22T10:31:37+02:00` — after this branch's evidence
was committed and before it was pushed. **The identifier space is shared across branches and sessions, so a local
maximum is not a free number**, and `git fetch` at the moment of choosing does not make it one.

The remedy was taken in the only order that works. `WORKFLOW.md` provides no way to re-point a verification record at a
different commit, so the collision had to be resolved **before any record existed**: the candidate commit was amended
twice — once for the four references in `WO-MOK-008.md`, once for the two in its own message — and this record was then
captured fresh at the resulting commit. The amendments are disclosed here rather than left implicit because they moved
the candidate's hash, and because everything in this record is measured at the amended commit and nothing is carried
from the two hashes that preceded it. `VREC-MOK-021` was re-checked the same way immediately before capture and is
unclaimed in the working tree, in every remote ref, in every reachable commit and in all fifteen pull requests.

**The residual is unchanged in kind and is stated rather than closed.** Another session's unpushed branch could hold a
`VREC-MOK-021` this repository cannot see. Nothing available to an offline check would reveal it, and the collision
above is what that risk looks like when it lands.

## The command produced this record, where three of its predecessors had to be composed by hand

`VREC-MOK-015`, `VREC-MOK-016` and `VREC-MOK-017` each record that `capture-verification` refuses from both positions —
from the working tree because the untracked record makes the worktree unclean, and from a clean checkout of the
candidate because the evidence it must validate does not exist there. **Neither refusal arises here, and the reason is
the order the commits were made in**, not a change in the tool:

- the nine new evidence files were committed in `4979abd`, one commit before the candidate, so every declared path
  exists in the tree the command read;
- the transition commit left the worktree clean, so `require_clean_worktree` passed both times the command calls it —
  once before measuring provenance and once after generating the snapshot.

So the front matter above is a capture and not a reconstruction. `WORKFLOW.md`'s rule that *"a record cannot contain the
hash of its own commit"* is satisfied for the same reason as before: this file arrives in a later governance commit than
the one it names.

## Every field of that capture was re-measured against the candidate

A captured field is still a claim, and each one below was measured independently rather than read back out of the file
it appears in.

| Field | How it was re-measured |
|---|---|
| `commit` | `git rev-parse HEAD` = `3da6acca1e8cb53f20ea13869c6f4bc425b979f2`, committed `2026-08-22 10:35:11 +0200`. `git rev-list --parents -1` shows the single parent `4979abd`; not a merge |
| `git_object_format` | `git rev-parse --show-object-format` → `sha1` |
| `worktree_state` | `git status --porcelain` in a detached worktree checked out at the candidate: **0 entries**. In the checkout that carries this work, the same command returns this record alone, which belongs to the commit that carries it and is not a tracked difference from `3da6acc` |
| `verified_at` | `2026-08-22T08:35:53Z`, the **capture** timestamp as the template names it, 42 seconds after the candidate's commit time of `08:35:11Z`. It is not the time of any decision, and no decision on this record has been taken |
| `evidence_paths` | `git ls-tree -r --name-only 3da6acc -- docs/engineering/simulation/evidence/WO-MOK-008`, sorted: **10 declared against 10 in the tree, nothing left over on either side** — no declared path absent from the commit, and no file of the pack undeclared |
| `artifact_snapshot_sha256` | reproduced exactly, and by the repository's own gate rather than only the harness's bundled generator — see *The snapshot figure* below, where the three ways to fail to reproduce it are each measured |

## What this record claims

### The candidate commit

| | |
|---|---|
| Commit | `3da6acca1e8cb53f20ea13869c6f4bc425b979f2` — *"gov(WO-MOK-008): transition from in_progress to implemented"* |
| Parent | `4979abdb71697c02b1a930aaebb74af7939e4b3c` — *"fix(WO-MOK-008): make the provenance footer shed fields in a specified order"*, a single parent; this is not a merge commit |
| Branch | `wo-mok-008-footer-shedding`. At capture `git branch -vv` showed **no upstream**; the push and the pull request are the owner's third and fourth authorized acts and follow this record |
| Difference from the parent | **one file, +16 / −2** — `work-orders/WO-MOK-008.md`, the transition and the paragraphs that record it. No source file, no manifest, no lockfile, no script and no workflow moves |
| Difference from the branch point `f7b1c45` | **14 files, +1,934 / −69** |

**Why the candidate is this commit and not `4979abd`.** Commit-bound verification is classified `required` for
`WO-MOK-008` — `preflight` prints the classification and the engineering owner's rationale — so the work order's
transition to `implemented` is part of what a record for it verifies; a record bound to `4979abd` would bind a tree in
which that transition has not happened. This is the reasoning `VREC-MOK-017` recorded at `ecba9fe` and `VREC-MOK-018` at
`6051ef2`, and the consequence is the same too: **the work order's own file cannot name the commit that carries it**, so
its `VREC-MOK-021` paragraph defers the hash to this record rather than printing one.

### Where the candidate stands relative to `master`

Measured after `git fetch --all`, because `master` moved twice while this work was in progress.

| | |
|---|---|
| `origin/master` tip | `5bdf6077539f349cca25794cc14979e6cd49501d` — *"Merge pull request #41 from mmzen/governance/wo-mok-017-closure"*, `2026-08-22 10:31:37 +0200` |
| What that merge carried | `WO-MOK-017` to `implemented`, its eleventh manual assessment, and `VREC-MOK-020` `verified` at `a30ae32` — the record that took this branch's intended identifier |
| Merge base with `origin/master` | `f7b1c452039dc2f03010ca8b8cc81e73c54727c0` — this branch's own base, **not** `master`'s tip |
| Distance | `git rev-list --left-right --count origin/master...3da6acc` → **5 behind, 2 ahead** |
| Is the candidate an ancestor of `origin/master`? | **No.** `git merge-base --is-ancestor 3da6acc origin/master` fails |
| Difference from `master` | this work order's **14 files** in one direction, and in the other the four files of #41's closure work — `WO-MOK-017.md`, two `evidence/WO-MOK-017/` documents and `VREC-MOK-020.md`. **No file is changed on both sides** |
| Source difference from `master` | the two files of the change alone. `Cargo.lock`, both manifests, the toolchain pin, every script and every workflow are byte-identical to `master`'s, and `mokiterions-core/` is not touched by one byte |

**This branch is deliberately not rebased.** A rebase would move the candidate and orphan the commit this record binds,
and `WORKFLOW.md` offers no re-pointing; the record binds a branch commit and never `master`'s merge. What was measured
instead is the merge itself, in a scratch worktree, so that the reviewer is not asked to take mergeability on faith.
`git merge origin/master` **applies cleanly with no conflict** — 4 files changed, +1,004 / −31, none of them a file this
branch also changed — and the merged tree validates **PASS at 149 artifacts, 0 errors, 0 warnings**, with the dashboard
reporting 149 artifacts, 529 relations, 0 errors and 18 warnings. **A duplicate identifier would be a validation error,
so `PASS` is the measurement that matters here**: the merged graph carries `VREC-MOK-020` and `VREC-MOK-021` as distinct
artifacts. That merge was measured at the commit that carries this record rather than at the candidate, since the
identifier collision is the thing being ruled out and this record is one half of it. **The figures are of a merge nobody
has yet decided to make, this record declares none of them, and no digest is quoted for that tree** — `master` can move
again and every count above is `5bdf607`'s.

### The gates, measured at this commit

Every command in the form `WO-MOK-008`'s *Required verification* names, re-run at the candidate rather than carried from
the evidence pack, from a clean detached worktree whose directory basename matches this checkout's. `se_harness` is the
pinned `0.4.0` venv, the version `.engineering-harness.toml` pins and the managed workflow installs.

| Command | Result |
|---|---|
| `cargo test --workspace` | exit 0 — **22 `test result:` lines, 312 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out** |
| `cargo fmt --all -- --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0 — **0 warning lines and 0 error lines** in the whole log |
| `cargo tree -p Mokiterions -e normal --locked --offline` | exit 0, **one line** — the engine declares no dependency |
| `python scripts/validate_engineering_artifacts.py --root .` | **PASS**, exit 0 — 147 artifacts, 0 errors, 0 warnings, all four planes at `E0/W0` |
| `bash scripts/check_engineering_harness.sh` | **PASS**, exit 0 — 147 artifacts, **525 relations**, 0 errors, 17 warnings, snapshot `4cb4044e…`, **the figure this record declares** |
| `python scripts/check_declared_dependencies.py --root .` | exit 0 — *"Every declared set matches its resolved graph. 8.4a-8.4d pass."* |
| `python scripts/inspect_engineering_artifacts.py --root .` | exit 0 — 147 artifacts, 525 relations, **36 findings: 0 error, 17 warning, 19 info**; queues as quoted in *What is still open* |
| `python -m se_harness validate .` | **PASS**, exit 0 — the same four planes at `E0/W0` |
| `python -m se_harness preflight . --work-order WO-MOK-008 --phase review` | **PASS** — **`WO-MOK-008 (implemented)`**, commit-bound verification **required**, decided by the **engineering owner**. The status the transition wrote is visible in the derived graph and not only in prose |

**Three notes, all of which the evidence pack's `gates.md` already carries in its own form.**

**The preflight form differs from the work order's wording.** The work order names `preflight --phase review`; the
command requires `--work-order`, and without it exits 2 on an argument error rather than on a finding. The form run is
the one in the table.

**The four cargo gates were re-run rather than carried, and the carry would have been defensible.** `3da6acc` differs
from the tested tree by one documentation file, so no figure could have moved; a figure is nonetheless a statement about
a tree, and a reader is entitled to see the statement made about *this* tree. **The ten cases this work order adds were
additionally run by name at the candidate, and each matched exactly one test and passed** — no name matched two targets
and none matched none.

**The counterfactual run is not one of these gates.** `counterfactual.md` records each added case against the superseded
renderer, which is a measurement of the cases and not a gate on the tree; `cargo test --workspace` at this commit is the
gate.

### The change surface this record binds

Measured `f7b1c45..3da6acc`, both commits of the branch together.

| Path | Lines | What changed |
|---|---|---|
| `mokiterions-tui/src/render.rs` | +619 / −60 | the private footer machinery — `Provenance`, `SHED_ORDER`, the presentation order, `supplied`, and `footer_row` with the shed as its outer loop — and nine in-crate cases |
| `mokiterions-tui/tests/verification.rs` | +167 / −0 | two cross-crate cases: the declared-viewport sweep, and the four floor rows asserted character for character |
| `docs/…/specifications/SPEC-MOK-003.md` | +38 / −3 | rule 8's preamble sentence and clauses 4 to 8, and clause 2's two sentences on *supplied* |
| `docs/…/verification/VER-MOK-005.md` | +61 / −3 | the rewritten `REQ-MOK-027` footer row, eight added rows, one amended in place, a *Provenance survival* property, the declared footer seed and tick-limit sets, three retained-evidence items and two residual bullets |
| `docs/…/work-orders/WO-MOK-008.md` | +17 / −3 | the amendment row marked ratified, the three transitions and the lifecycle record |
| `docs/…/evidence/WO-MOK-008/` | 9 files, +1,032 / −0 | the retained pack's new documents; `footer-tier-fallthrough.md` predates this work order and is unchanged |

**`mokiterions-core/` is not touched by one byte**, and `git diff --numstat f7b1c45 3da6acc -- mokiterions-core`
returning empty is the measurement behind that sentence. No public interface widens: `Provenance`, `SHED_ORDER`,
`SPELLINGS`, `supplied` and `footer_row` are all private, which is why nine of the ten cases are in-crate.

### The coverage this record claims in `VER-MOK-005`

The contract holds **96 `automated-test` matrix rows** after its 2026-08-22 amendment, against the 88 it held before,
and **0 `manual-assessment` rows**. The mapping of the ten arriving cases to the rows they answer is
`evidence/WO-MOK-008/verification-mapping.md`, and it states for each row *what the case sweeps* rather than only which
case it is — because the defect this work order fixes is precisely a row whose universal quantifier was mapped to a
single case.

**The row count and the case count differ, and the amendment row uses one word for both.** Eight matrix rows are added
and nine tests arrive with them — the *entropy seed survives the floor* row is discharged twice, in-crate across 44
seeds and cross-crate at four floor rows asserted character for character — plus a tenth test under the *Commit field is
compile-time or absent* row, which is amended in place rather than added. So the contract's headline *"nine cases
added"* counts tests, its body's *"Eight cases are added"* counts rows, and **96 − 88 = 8** is the row arithmetic. Both
statements are true of what they count; the ambiguity is recorded here so that a reader need not derive it.

**What this claims about the other 86 rows is that the suite passes, and no more.** The pack contains no
requirement-to-test mapping for the whole contract; it maps the ten arrivals to the ten rows they answer and reports the
suite at 312 passed, of which **302 are pre-existing cases this work order neither adds nor changes**.

**No manual assessment is taken, owed or waived.** `VER-MOK-005` declares seven, and none has the footer as its subject:
they are `INT-MOK-004`'s three questions, whole-world legibility, the distinguishability of `UNDERLINED` from
`REVERSED`, a rejection reading as an authority outcome, the fourth roster bar's computed `fear`, the overview glyph's
positional fidelity, and terminal usability after a panic. The 2026-08-22 amendment adds none, which is why all ten
arriving cases are automated.

## The snapshot figure, and what it is a figure of

`artifact_snapshot_sha256` is `4cb4044e3462be3820006e2172047def2d8a5bcdf0a56f32e2c92497d2ac9450`: the SHA-256 of
`target/harness-dashboard/dashboard-data.json` as the managed generator writes it at this commit — **147 artifacts, 525
relations, 0 errors, 17 warnings, PASS**. It was reproduced by `scripts/check_engineering_harness.sh`, the repository's
own gate, and not only by the harness's bundled generator.

**The three ways to fail to reproduce it were each isolated by measurement.** The hashed document carries
`git rev-parse HEAD`, the checkout's directory basename and the finding set, so all three are variables a reader can
trip over:

| Measurement | `HEAD` in the hashed document | Tree | Directory basename | Warnings | Digest |
|---|---|---|---|---|---|
| the implementation commit | `4979abd` | the change and the pack, without the transition | matches | 16 | `46fbf948…` |
| **this record's declared figure** | `3da6acc` | + the transition | matches | 17 | **`4cb4044e…`** |
| the same commit, a differently named checkout | `3da6acc` | identical | differs | 17 | `3b07449c…` |
| a `--depth 1` clone of the same commit | `3da6acc` | identical | matches | **37** | `c9054821…` |

Read down that table. **`46fbf948…` and `4cb4044e…` differ by one documentation file and the `HEAD` field**, which is
the whole reason no snapshot in the evidence pack is this record's, and a reader comparing them without that mechanism
would be hunting a content change that does not exist. `3b07449c…` isolates the directory basename, which
`repository.name` takes from the checkout's own; the two digests differ at identical content and an identical finding
set. `c9054821…` isolates clone depth: the depth-1 clone reports **37 warnings against 17**, the difference being
`W-REV-003` at **20 observations**, one for each of the nineteen verification records and `RLS-MOK-001` in that tree,
every one declaring a candidate commit a shallow clone cannot reach. `.github/workflows/engineering-harness.yml` uses
`actions/checkout@v4` with no `fetch-depth`, so **CI will not reproduce this record's digest and the discrepancy will be
the clone's.** On a `pull_request` event the action checks out the ephemeral merge ref as well, so CI's revision is a
commit that exists on no branch and its digest is a figure of no commit in this repository. The managed workflow is not
amended here, and *Out of scope* keeps it out.

**CI's warning count must be read off `W-REV-003` and not off `I-REV-001`.** The two rules count different sets, and
neither is a proxy for the other — which the four measurements below establish rather than assert:

| Tree | `W-REV-003` | `I-REV-001` |
|---|---|---|
| full clone at the candidate | does not fire — every candidate commit is reachable | 19 |
| full clone carrying this record | does not fire | 19 |
| `--depth 1` clone at the candidate | 20 | not measured |
| `--depth 1` clone carrying this record | **21** | 20 |

So the informational count is itself depth-sensitive, and it neither equals nor tracks the warning's. Arithmetic on it is
how `VREC-MOK-018` published a warning prediction that was wrong by one, and the correction is recorded in that record's
own decision section. The figure here is taken from `W-REV-003`'s own count and **measured rather than derived**, at the
commit that carries this record: a depth-1 clone of it reports **38** warnings — `W-HEX-001` × 8 + `W-HEX-003` × 9 +
`W-REV-003` × 21 — at 148 artifacts and 527 relations. **No digest is quoted for that clone, and none should be.** Its
`HEAD` is this file's own commit, so the figure moves with every amendment of the commit that carries this record, while
the warning count does not: the count is a property of the finding set and the clone's depth alone. The only digest this
record declares is the candidate's.

**One figure serves, because this record does not exist at the commit it binds.** The generator ran before this file was
written, so the as-committed figure and the record-absent figure are the same number, and no later edit to this file —
including a transition to `verified` — can move it. `VREC-MOK-014` had to publish two digests for exactly the opposite
reason.

**Evidence enters the digest by path and not by bytes.** The generator hashes normalized front matter, relations,
findings and the declared evidence paths, never the evidence files' contents — so no figure here is a checksum of the
pack. What changes with **this** record is that the ten paths become declared, and that is visible only in a tree holding
it: **148 artifacts, 527 relations, 17 warnings** against the candidate's 147 / 525 / 17, the two added relations being
this record's own. Editing this file's prose moves none of those four numbers, which was measured by generating the
snapshot before and after the body below was written. **No digest is given for such a tree**, and the reason is the
`HEAD` field again: any tree carrying this file has this file's commit as its `HEAD`, so the digest would be a figure of
a commit that did not exist when the figure was taken. The digest of the commit that carries this record cannot be stated
inside it, for the same reason `WORKFLOW.md` forbids a record naming its own commit.

**The pack's line endings are what the digest does not check, and they were measured anyway.** `.gitattributes` marks
`docs/engineering/simulation/evidence/**` as `-text`, so the ten retained documents are stored and checked out **LF on
every platform**: measured byte for byte in this Windows checkout with `core.autocrlf=true` and again in the depth-1
clone, all ten carry zero CRLF pairs and are pairwise identical in SHA-256, while `mokiterions-tui/src/render.rs` is
CRLF in both — 2,628 line endings, `66c8a096…`. A digest recorded inside the pack therefore survives a fresh clone; the
artifact `.md` files, this record included, are CRLF by the repository's convention.

## The eighth `W-HEX-001` observation is this transition's own, and its cause is a filename

The dashboard reports 17 warnings here where the parent commit reports 16, and the arrival is exactly **`W-HEX-001` on
`WO-MOK-008`**: an implemented work order with no evidence document keyed to its ID. `W-HEX-001` goes **7 → 8** across
the two commits and `W-HEX-003` stays at 9. The transition is the cause, so the seventeenth warning is a consequence of
the owner's act rather than a regression in the change.

**The warning stands against ten retained files, and its mechanism is worth measuring rather than disputing.** The rule
matches each evidence file's **name** against `^(WO-[A-Z0-9-]*\d{3})`; it does not read the directory the file sits in.
This repository keys evidence by **directory** — `evidence/WO-MOK-008/` with descriptive filenames — so no file of this
pack matches. That is why the rule fires on the eight implemented work orders that use the directory convention and on
none of those that happen to carry a file named for the ID. Renaming one file would close it. **That is not done here**,
because the pack's paths are what this record's `evidence_paths` declares and the tree is what it binds. It is recorded
as a finding for a later work order, with the mechanism measured so that the remedy is a decision and not an
investigation.

## Two provisions this record does not ratify

Each is disclosed in the artifact that carries it, and this record neither approves nor weakens either. A transition to
`verified` would be a decision on the evidence at this commit and **would not be a ratification of these two**.

| Provision | Where | Why the approving act does not reach it |
|---|---|---|
| **Clause 4's first row** — the candidate commit shed ahead of every field `REQ-MOK-027` names | `SPEC-MOK-003`'s row of 2026-08-22, which marks it **OUTSTANDING for the technical owner's ratification**, and `VER-MOK-005`'s row of the same date | the owner fixed the order over rule 8's **six preamble fields** and was not shown the commit's position, which that question does not contain. The agent placed it first because clause 2 already makes it the one contingent field. The case that measures it — `the_candidate_commit_is_shed_before_every_field_rule_8_requires` — asserts the position rather than assuming it, so a reversal changes one array element, one table row and that case |
| The **`REQ-MOK-027` floor residual** — that these cases verify rule 8 as amended and not the requirement as written, which the amendment concedes cannot hold at `34 × 22` | `VER-MOK-005`'s residual bullets of 2026-08-22 | it is the **product owner's** disclosure to resolve or accept. `implemented` records the engineering owner's judgement of completeness and `verified` would record the assurance owner's judgement of the evidence; neither is a requirement decision |

Both are recorded in `WO-MOK-008.md`'s own lifecycle section, which states in terms that `implemented` records
authority and not confidence — `WORKFLOW.md`: *"A status change records authority; it is not a confidence estimate."*

## Five statements in the retained evidence are falsified by the acts that followed it

All were true when written at the tree `4979abd` carries, and all are false at `3da6acc` or after it. **None is
edited**, because evidence is re-run rather than corrected and this pack's files are declared above by path, so an edit
would change what this record binds.

1. `README.md`: *"`WO-MOK-008` is **`in_progress`** and is left there"*, and `completion-report.md`: *"The work order is
   left at **`in_progress`**"*. The transition is the candidate commit.
2. `README.md`'s *"**No `VREC` is written by this session**"* under *No verification record*, and
   `completion-report.md`'s *"No `VREC` is written here."* This file is that record.
3. `README.md`: *"**No push and no pull request.** The change is committed on `wo-mok-008-footer-shedding` and stops
   there."*
4. `completion-report.md`: *"**Nothing is pushed and no pull request is opened** — the owner's authorization was to
   commit and stop."* Accurate of the authorization it names; a later instruction in the same session authorized both.
5. `completion-report.md`: *"the one status transition made — `draft` → `in_progress`"*. Three transitions are now
   recorded in the work order, all under the owner's explicit override of its own prohibition.

**The falsifying acts are exactly the ones each sentence reserves to someone else**, so what is stale is the tense and
not the reasoning. Nothing in the pack restates a dashboard digest or a warning count, so no snapshot figure in it is
stale.

## What this record does not claim

- **It does not claim verification.** The status is `ready`. The transition to `verified` is the assurance owner's act on
  this evidence at this commit, and no part of this record anticipates its outcome.
- **It does not claim clause 4's first row is ratified**, and it does not claim `WO-MOK-008`'s approval or its
  `implemented` status covers it.
- **It does not resolve the `REQ-MOK-027` floor residual**, which `VER-MOK-005` discloses as the product owner's.
- **It does not close `VER-MOK-005` row by row.** 86 of the 96 automated rows are claimed only by the suite passing; ten
  are mapped.
- **It takes no manual assessment**, and `VER-MOK-005` contracts none for any of the ten cases.
- **It does not claim the counterfactual is a gate.** `counterfactual.md` measures the cases against the superseded
  renderer: 7 of the 10 fail there, 3 are blind by construction and are named as such, and two of the seven were
  themselves blind when first written and were strengthened for that reason.
- **It does not claim the rewritten row's field-set half is swept.** That half is asserted at the two ends of the
  declared range and **derived across the middle**, on the stated assumption that the footer is one row and its content
  a function of width alone. `verification-mapping.md` names it as the one place the mapping argues rather than measures.
- **It does not reach any engine change, any change to what the footer means, a widened observer interface, a build
  script, a new dependency, or whether release CI stamps a commit at all.** All are in *Out of scope*.
- **It corrects no committed record and no other work order's evidence.** The five stale sentences above are disclosed,
  not edited, and `VREC-MOK-020` — another session's record, `verified` on `master` — is neither read nor touched here.
- **It is not a merge, a tag, a release or a push decision.** The merge measurement above is offered as evidence of
  mergeability and is a figure of no commit; `WORKFLOW.md` does not govern merging, and a record binds a branch commit
  and never `master`'s merge.
- **It does not amend the managed harness workflow** whose depth-1 checkout will not reproduce the declared digest.

## Residual uncertainty this record carries forward

- **No person has looked at a shed footer.** All ten cases assert in-memory character buffers, and `VER-MOK-005`'s seven
  manual assessments run at declared verification seeds of at most three digits, where nothing sheds. The shedding this
  work order specifies is therefore unreachable in the only configuration a human being is contracted to look at. That
  a buffer is correct does not establish that an operator reads a 34-column row carrying a 20-digit seed and nothing
  else as *the seed and nothing else* rather than as a defect.
- **Clause 4's first row is measured against an unratified provision**, so one of the ten cases asserts a position the
  technical owner may reverse. The reversal is cheap by construction and is named above; the uncertainty is whether it
  is the right order, not whether the code conforms to it.
- **Three of the ten cases are blind by construction** — they pass against the superseded renderer as well as this one,
  and they are named in `counterfactual.md`. A regression only they would catch does not exist.
- **The derivation across the middle of the declared viewport range lapses if the footer ever becomes two rows or its
  content ever depends on height.** `verification-mapping.md` states the assumption for exactly that reason, and
  `VER-MOK-005`'s *Provenance survival* property measures the monotonicity half rather than deriving it.
- **`REQ-MOK-027`'s field set cannot hold at the floor and the requirement is unchanged.** Rule 8 as amended concedes
  it; the requirement still obliges it. Until the product owner acts, the contract verifies the specification against
  itself on this point.
- **The `W-HEX-001` observation on `WO-MOK-008` will persist** until either an evidence file is named for the ID or the
  managed rule reads directories. Eight implemented work orders carry it today.
- **CI's dashboard warning count reads 40 where this record declares 17, and its snapshot does not match. This bullet
  predicted 38 and the prediction was wrong; the figure below is the run's.** The first `pull_request` run of
  [#44](https://github.com/mmzen/Mokiterions/pull/44) reports **149 artifacts, 529 relations, 0 errors, 40 warnings** —
  `W-HEX-001` × 9 + `W-HEX-003` × 9 + `W-REV-003` × 22. **Two mechanisms compose and the prediction used only one.** The
  depth-1 checkout fires `W-REV-003` once per candidate-declaring artifact, which is the 38 measured above; and
  `actions/checkout@v4` on a `pull_request` event reads the **ephemeral merge ref**, so CI's tree is this branch merged
  with `master` rather than this branch. That merge brings `VREC-MOK-020` — one more artifact, two more relations, one
  more `W-REV-003` — and `WO-MOK-017` at `implemented` with directory-keyed evidence, one more `W-HEX-001`. A depth-1
  clone of this branch's own tip still reports 38: **the two figures describe two different trees, and CI's is a commit
  that exists on no branch.** Its digest is deliberately not quoted here for that reason. **No figure this record binds
  is affected** — the declared `4cb4044e…` and the candidate's 147 / 525 / 17 are unmoved, and CI reports 0 errors and
  `PASS` on every job. The remedy for the count itself is upstream in the harness distribution or an owner decision to
  deviate.
- **`master` moved twice during this work and may move again before the merge.** The mergeability measurement above is a
  statement about `5bdf607`; a later tip could conflict where that one does not, and the identifier collision recorded
  above is what concurrent motion looks like when it reaches this branch. Re-deriving it is the reviewer's, since a
  rebase here would orphan the commit this record binds.
- **`WO-MOK-017` reads `in_progress` in the tree this record binds.** It was transitioned to `implemented` on `master`
  in #41, which this branch does not contain, so `inspect` at the candidate lists it under `active_work`. That is a fact
  about the branch's base and not a claim about that work order, whose evidence was not read here.

Every command behind every figure in this record is offline, reads no credential, secret, token or environment value,
and none appears in the retained evidence or in this file.
