+++
id = "VREC-MOK-018"
type = "verification_record"
title = "Verification candidate for WO-MOK-018"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-21"
updated = "2026-08-21"
commit = "6051ef218e51fb59c63fe5569b821be66c973cde"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-21T15:07:13Z"
artifact_snapshot_sha256 = "496f1a6d74241ab083e97687741a8fd9edc5ecadb199f7cbe9c4329cff2f878a"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-018/README.md", "docs/engineering/simulation/evidence/WO-MOK-018/completion-report.md", "docs/engineering/simulation/evidence/WO-MOK-018/filter-vocabulary.md", "docs/engineering/simulation/evidence/WO-MOK-018/filter-vocabulary.txt", "docs/engineering/simulation/evidence/WO-MOK-018/gates.txt", "docs/engineering/simulation/evidence/WO-MOK-018/inspector-one-line.txt", "docs/engineering/simulation/evidence/WO-MOK-018/inspector.md", "docs/engineering/simulation/evidence/WO-MOK-018/inspector.txt", "docs/engineering/simulation/evidence/WO-MOK-018/interface.md", "docs/engineering/simulation/evidence/WO-MOK-018/interface.txt", "docs/engineering/simulation/evidence/WO-MOK-018/non-perturbation.md", "docs/engineering/simulation/evidence/WO-MOK-018/non-perturbation.txt", "docs/engineering/simulation/evidence/WO-MOK-018/test-census.md", "docs/engineering/simulation/evidence/WO-MOK-018/test-census.txt", "docs/engineering/simulation/evidence/WO-MOK-018/wo018-non-perturbation-oracle.rs", "docs/engineering/simulation/evidence/WO-MOK-018/wo018-oracle.rs"]

[relations]
verifies_work_order = ["WO-MOK-018"]
conforms_to = ["VER-MOK-005"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-018` to candidate commit `6051ef218e51fb59c63fe5569b821be66c973cde`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

*Everything above this line is `capture-verification`'s output verbatim — the front matter, the heading and the two
paragraphs. Everything below is authored, and every figure in it was measured at the candidate rather than carried from
the evidence pack.*

## What is still open

**This record is `ready`. It is not `verified`, and nothing in it asserts that it is.** `WORKFLOW.md` moves a
verification record from `ready` to `verified` through a separate accountable decision by the assurance owner, on the
evidence, and that decision has not been taken. The instruction that produced this record was *"you can transition the
work orders to implemented, and prepare the verification record"*: the transition is commit `6051ef2`, this record is
the preparation, and the third act is the owner's.

The derived queues say the same thing in the harness's own words. At the candidate, `inspect` reports
`assurance_pending` holding exactly **`WO-MOK-018 [implemented] prepare-commit-bound-verification`** and
`decision_required` empty. In the working tree that carries this record, `assurance_pending` **empties** and
`decision_required` holds exactly **`VREC-MOK-018 [ready] assurance-review`**. The queue moved from *prepare* to
*review*, which is the whole of what this record accomplishes.

## The command produced this record, where the last three had to be composed by hand

`VREC-MOK-015`, `VREC-MOK-016` and `VREC-MOK-017` each record that `capture-verification` refuses from both positions —
from the working tree because the untracked record makes the worktree unclean, and from a clean checkout of the
candidate because the evidence it must validate does not exist there. **Neither refusal arises here, and the reason is
the order the commits were made in**, not a change in the tool:

- the sixteen evidence files were committed in `b034da3`, one commit before the candidate, so every declared path exists
  in the tree the command read;
- the transition commit `6051ef2` left the worktree clean, so `require_clean_worktree` passed both times the command
  calls it — once before measuring provenance and once after generating the snapshot.

So the front matter above is a capture and not a reconstruction. The rule it is subject to is unchanged and is
satisfied for the same reason as before: `WORKFLOW.md`'s *"a record cannot contain the hash of its own commit"* holds
because this file arrives in a later governance commit than the one it names.

## Every field of that capture was re-measured against the candidate

A captured field is still a claim, and each one below was measured independently rather than read back out of the file
it appears in.

| Field | How it was re-measured |
|---|---|
| `commit` | `git rev-parse HEAD` = `6051ef218e51fb59c63fe5569b821be66c973cde`, committed `2026-08-21 17:06:54 +0200`. Single parent `b034da3`; not a merge |
| `git_object_format` | `git rev-parse --show-object-format` → `sha1` |
| `worktree_state` | `git status --porcelain` in a detached worktree checked out at the candidate: **0 entries**. In the checkout that carries this work, the same command returns this record alone, which belongs to the commit that carries it and is not a tracked difference from `6051ef2` |
| `verified_at` | `2026-08-21T15:07:13Z`, the **capture** timestamp as the template names it, 19 seconds after the candidate's commit time of `15:06:54Z`. It is not the time of any decision, and no decision on this record has been taken |
| `evidence_paths` | `git ls-tree -r --name-only 6051ef2 -- docs/engineering/simulation/evidence/WO-MOK-018`, sorted: **16 declared against 16 in the tree, nothing left over on either side** — no declared path absent from the commit, and no file of the pack undeclared |
| `artifact_snapshot_sha256` | reproduced exactly — see *The snapshot figure* below, where the three ways to fail to reproduce it are each measured |

## What this record claims

### The candidate commit

| | |
|---|---|
| Commit | `6051ef218e51fb59c63fe5569b821be66c973cde` — *"gov: transition WO-MOK-018 from in_progress to implemented"* |
| Parent | `b034da3` — *"Implement WO-MOK-018: fear at death, and the filter count corrected"*, a single parent; this is not a merge commit |
| Branch | `feature/observer-fear-and-filter-count`, local only: `git branch -vv` shows **no upstream**, so nothing here has been pushed and no pull request exists |
| Difference from the parent | **one file, +37 / −3** — `work-orders/WO-MOK-018.md`, the transition and the paragraphs that record it. No source file, no manifest, no lockfile, no script and no workflow moves |
| Difference from the branch point `f2a79e1` | 24 files, **+2,330 / −35** |

**Why the candidate is this commit and not `b034da3`.** Commit-bound verification is classified `required` for
`WO-MOK-018`, so the work order's transition to `implemented` is part of what a record for it verifies; a record bound
to `b034da3` would bind a tree in which that transition has not happened. This is the reasoning `VREC-MOK-017` recorded
for the same choice at `ecba9fe`, and the consequence is the same too: **the work order's own file cannot name the
commit that carries it**, so its `VREC-MOK-018` paragraph defers the hash to this record rather than printing one.

### Where the candidate stands relative to `master`

Measured after `git fetch origin`, because `master` moved while this work was in progress.

| | |
|---|---|
| `origin/master` tip | `aeca80815b1cbf4d17000ec950b8d0abe06ec7cb` — *"Merge pull request #36 from mmzen/feature/phase-3-definition"*, `2026-08-21T15:33:45+02:00` |
| What that merge carried | the chain `VREC-MOK-017` verifies, `WO-MOK-016` included. `master` now holds that record as `verified` |
| Merge base with `origin/master` | `aeca808` — **`master`'s own tip**, so this branch is a direct descendant and there is nothing to rebase |
| Is the candidate an ancestor of `origin/master`? | **No.** `git merge-base --is-ancestor 6051ef2 origin/master` fails |
| Difference from `master` | exactly this work order's **24 files** — 17 added, 3 modified specifications and contract, 4 modified source files — **and nothing in the other direction**: no file `master` holds is absent or altered here |
| Source difference from `master` | the four files of the change alone. `Cargo.lock`, both manifests, the toolchain pin, every script and every workflow are byte-identical to `master`'s |

**This record reaches the candidate commit and nothing downstream of it.** No merge, no tag, no release and no
statement about `master` is part of it.

### The gates, measured at this commit

Every command in the form `WO-MOK-018`'s *Required verification* item 6 names, re-run at the candidate rather than
carried from the evidence pack, from a clean detached worktree. `se_harness` is the pinned `0.4.0` venv.

| Command | Result |
|---|---|
| `cargo test --workspace` | exit 0 — **21 `test result:` lines, 267 passed, 0 failed, 0 ignored, 0 measured, 0 filtered out** |
| `cargo fmt --all -- --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0 — **0 warning lines and 0 error lines** in the whole log |
| `cargo tree -p Mokiterions -e normal --locked --offline` | exit 0, **one line** — the engine declares no dependency |
| `python scripts/validate_engineering_artifacts.py --root .` | **PASS**, exit 0 — 133 artifacts, 0 errors, 0 warnings, all four planes at `E0/W0` |
| `bash scripts/check_engineering_harness.sh` | **PASS** — 133 artifacts, **483 relations** |
| `python scripts/check_declared_dependencies.py --root .` | exit 0 — *"Every declared set matches its resolved graph. 8.4a-8.4d pass."* |
| `python scripts/generate_harness_dashboard.py --root <worktree>` | **PASS**, exit 0 — 133 / 483 / 0 errors / **16 warnings**, snapshot `496f1a6d…`, the figure this record declares |
| `python scripts/inspect_engineering_artifacts.py --root .` | exit 0 — 133 artifacts, 483 relations, **32 findings: 0 error, 16 warning, 16 info**; queues as quoted in *What is still open* |
| `python -m se_harness validate .` | **PASS**, exit 0 — the same four planes at `E0/W0` |
| `python -m se_harness preflight . --work-order WO-MOK-018 --phase review` | **PASS** — **`WO-MOK-018 (implemented)`**, commit-bound verification **required**, decided by the **engineering owner**. The status the transition wrote is visible in the derived graph and not only in prose |

**Two notes, both of which the evidence pack's `gates.txt` already carries in its own form.**

**The preflight form differs from the work order's wording.** Item 6 names `preflight --phase review`; the command
requires `--work-order`, and without it exits 2 on an argument error rather than on a finding. The form run is the one
in the table.

**The four cargo gates were re-run rather than carried, and the carry would have been defensible.** `6051ef2` differs
from the tested tree by one documentation file, so no figure could have moved; `VREC-MOK-017` re-ran its source gates at
its own candidate for the reason that applies here too — a figure is a statement about a tree, and a reader is entitled
to see the statement made about *this* tree. The three tests this work order adds were additionally run by name at the
candidate and each passes: `a_death_carries_the_fear_the_engine_last_reported_for_its_subject`,
`the_inspector_presents_a_dead_subject_s_final_fear`, and
`state::tests::a_death_carries_no_attribute_the_engine_never_reported_for_its_subject`.

### The change surface this record binds

| Path | Lines | What changed |
|---|---|---|
| `mokiterions-tui/src/state.rs` | +127 / −8 | `Death` gains `pub fear: Option<u8>`; `latest_survival` widens to `BTreeMap<String, (u8, u8, u8)>`; two `ingest` arms; the internal absence case |
| `mokiterions-tui/src/render.rs` | +27 / −4 | the death branch presents four values paired across two lines |
| `mokiterions-tui/tests/state.rs` | +41 / −0 | the public-tier reported-`fear` case |
| `mokiterions-tui/tests/verification.rs` | +50 / −6 | the public-tier frame case |
| `SPEC-MOK-003.md`, `SPEC-MOK-004.md`, `VER-MOK-005.md` | +167 / −13 | rules 9 and 10; rules 6, 9, 10 and 11; four matrix rows and one residual |
| `WO-MOK-018.md` | +107 / −4 | the work order, its five amendments and its transition |
| `evidence/WO-MOK-018/` | 16 files, +1,811 / −0 | the retained pack |

**`mokiterions-core/` is not touched by one byte.** `git diff --numstat f2a79e1 6051ef2 -- mokiterions-core` is empty,
which is a measurement and not a claim, and it is why every engine figure in the pack is `WO-MOK-016`'s carried through
rather than this work order's subject.

### The coverage this record claims in `VER-MOK-005`

`VER-MOK-005` holds **88 automated cases** after its 2026-08-21 amendment, which amended two `REQ-MOK-021` rows and
added two. The four rows this work order is answerable for, and what answers each:

| Row | Case | Where |
|---|---|---|
| *Selected Mokiterion dies* — amended to name all four attributes, paired across two lines, with nothing clipped at any viewport presenting the inspector | `the_inspector_presents_a_dead_subject_s_final_fear` | `mokiterions-tui/tests/verification.rs` — public tier, asserted at the frame |
| *A dead subject's final fear is the engine's own* — added | `a_death_carries_the_fear_the_engine_last_reported_for_its_subject` at the state, and the frame case above at the pane | `mokiterions-tui/tests/state.rs` and `tests/verification.rs` |
| *An unreported final attribute is absent* — added | `a_death_carries_no_attribute_the_engine_never_reported_for_its_subject` | `mokiterions-tui/src/state.rs` — internal tier, because the state is reachable only through the private `ingest`; the absence is nonetheless asserted at the rendered pane and not only at the derived value |
| *Absent attributes are absent* — amended to confine `fear` to the living case and to present the `name` | the existing case, unchanged | `mokiterions-tui/tests/verification.rs` |

**What this claims about the other 84 rows is that the suite passes, and no more.** This pack contains no
requirement-to-test mapping for the whole contract; it maps the three arrivals to the four rows above and reports the
suite at 267 passed. A reader wanting row-by-row coverage of the contract as a whole has `VREC-MOK-005`'s mapping, which
is `verified` at commit `f3613701` and therefore predates all four of this contract's later amendments.

**No manual assessment is taken or claimed.** `VER-MOK-005` declares seven, and none of the seven has the death line as
its subject: the fifth is about the living roster's fourth bar. The 2026-08-21 amendment adds no assessment, which is
why the two new cases are automated and the one clause neither reaches is disclosed as a residual instead.

## The snapshot figure, and what it is a figure of

`artifact_snapshot_sha256` is `496f1a6d74241ab083e97687741a8fd9edc5ecadb199f7cbe9c4329cff2f878a`: the SHA-256 of
`target/harness-dashboard/dashboard-data.json` as the managed generator writes it at this commit — **133 artifacts, 483
relations, 0 errors, 16 warnings, PASS**. `capture-verification` runs the template's own copy of that generator, which
is identical in SHA-256 to the repository's `scripts/generate_harness_dashboard.py` as stored, `33c105ba…`, so the
figure is reproducible with the repository's own script.

**It was reproduced, and the three ways to fail to reproduce it were each isolated by measurement.** The hashed document
carries `git rev-parse HEAD`, the checkout's directory name and the finding set, so all three are variables a reader can
trip over:

| Measurement | `HEAD` in the hashed document | Tree | Directory name | Digest |
|---|---|---|---|---|
| `evidence/WO-MOK-018/gates.txt` | `f2a79e1` — those gates ran before the implementation was committed | the implementation, uncommitted | matches | `e87883bb…` |
| the implementation commit | `b034da3` | identical to the row above | matches | `9a1378f7…` |
| **this record's declared figure** | `6051ef2` | + the transition | matches | **`496f1a6d…`** |
| the same commit, a differently named checkout | `6051ef2` | identical | differs | `4a113d7d…` |
| a depth-1 clone of the same commit | `6051ef2` | identical | matches | `e16ae523…` |

Read down that table. **`e87883bb…` and `9a1378f7…` differ in the `HEAD` field alone** — same content, same directory
name — which is the whole reason the evidence pack's snapshot is not this record's, and a reader comparing the two
without that mechanism would be hunting a content change that does not exist. `4a113d7d…` isolates the directory name,
which `repository.name` takes from the checkout's own basename. `e16ae523…` isolates clone depth: the depth-1 clone
reports **33 warnings against 16**, the difference being **`W-REV-003` at 17 observations**, one for each of the sixteen
verification records and `RLS-MOK-001` in that tree, every one declaring a candidate commit a shallow clone cannot
reach. `.github/workflows/engineering-harness.yml` uses `actions/checkout@v4` with no `fetch-depth`, so **CI will not
reproduce this record's digest and the discrepancy will be the clone's.** `VREC-MOK-017` measured the same mechanism at
its own candidate; the managed workflow is not amended here, and *Out of scope* keeps it out.

**One figure serves, because this record does not exist at the commit it binds.** The generator ran before this file was
written, so the as-committed figure and the record-absent figure are the same number, and no later edit to this file —
including a transition to `verified` — can move it. `VREC-MOK-014` had to publish two digests for exactly the opposite
reason.

**Evidence enters the digest by path and not by bytes.** `gates.txt` records that adding six evidence documents did not
move the snapshot, which was correct for the tree it was measured in and is the same mechanism: the generator hashes
normalized front matter, relations, findings and the declared evidence paths, never the evidence files' contents. What
changes with **this** record is that the sixteen paths become declared, which is visible only in the working tree that
holds it — 134 artifacts, 485 relations, snapshot `f38ffb6d…`. **That figure is not a figure of any commit and this
record does not declare it**; the digest of the commit that carries this file cannot exist until that commit does.

## The sixteenth dashboard warning is this transition's own, and its cause is a filename

The dashboard reports 16 warnings here where the evidence pack reports 15, and the arrival is exactly **`W-HEX-001` on
`WO-MOK-018`**: *"WO-MOK-018 is implemented but has no evidence document keyed to its ID."* Measured at both commits:
`b034da3` gives `W-HEX-001` × 5 + `W-HEX-003` × 10 = 15; `6051ef2` gives 6 + 10 = 16. The transition is the cause, so
the sixteenth warning is a consequence of the owner's act rather than a regression in the change.

**The warning stands against sixteen retained files, and its mechanism is worth measuring rather than disputing.**
`discover_evidence` matches each evidence file's **name** against `^(WO-[A-Z0-9-]*\d{3})(?:-|\.|$)`; it does not read
the directory the file sits in. This repository keys evidence by **directory** — `evidence/WO-MOK-018/` with descriptive
filenames — so no file of this pack matches. That is why the rule fires on all six implemented work orders that use the
directory convention — `WO-MOK-010`, `-011`, `-012`, `-013`, `-016` and `-018` — and on none of the nine that happen to
carry a file named for the ID: `WO-MOK-001` through `-007` and `-009` through
`evidence/release-0.1.0/WO-MOK-00N-containment.md`, and `WO-MOK-014` through its own 34 prefixed files.

Renaming one file of the pack would close it. **That is not done here**, because the pack's paths are what this record's
`evidence_paths` declares and the tree is what it binds. It is recorded as a finding for a later work order, with the
mechanism measured so that the remedy is a decision and not an investigation.

## Three provisions this record does not ratify

Each is marked OUTSTANDING in the artifact that carries it, and this record neither approves nor weakens any of them. A
transition to `verified` would be a decision on the evidence at this commit and **would not be a ratification of these
three**; they are the technical owner's and the assurance owner's separate acts.

| Provision | Where | Why the approving act does not reach it |
|---|---|---|
| `SPEC-MOK-004` rule 6's figures — **amendment 5** — 94 items unmoved, public fields 24 → **25**, `pub` lines 118 → **119** | that rule's 2026-08-21 row, and `evidence/WO-MOK-018/interface.md` | not among the four amendments the owner approved, and the work order predicted the opposite in terms. The growth is forced by the presentation the owner did approve, and the alternative that avoids it moves the item count, which the work order's first stop-and-escalate condition forbids outright |
| rule 10.6's **two-line pairing** of the four death values | `SPEC-MOK-003` rule 10's 2026-08-21 row, and `evidence/WO-MOK-018/inspector.md` | inside an amendment the owner approved, but the owner was not shown this provision: the one-line form was implemented first and its clipping was found afterwards, by the frame case this work order adds |
| `VER-MOK-005`'s stale **`name`** in *Absent attributes are absent* | that contract's 2026-08-21 row | a correction of a false statement about another approved artifact, outside the approved scope, and **not this work order's defect** — the row has contradicted `SPEC-MOK-003` rule 10 since 2026-08-19 |

## Two statements in the retained evidence are falsified by the commit that carries them

Both were true when written at `b034da3` and are false at `6051ef2`, and **neither is edited**, because evidence is
re-run rather than corrected and this pack's files are declared above by path.

1. `evidence/WO-MOK-018/README.md`: *"`WO-MOK-018` is **`in_progress`** and is left there … No `VREC` is written here."*
   Its *What is not here* section says the same under *No verification record*.
2. `evidence/WO-MOK-018/completion-report.md`: *"The work order is left at **`in_progress`**"*, and point 8's *"It does
   not claim `implemented`. The status stays `in_progress`; the transition is the owner's."*

Both were accurate statements of an implementation agent's position, and both are falsified by the two acts that
followed: the owner's transition, and this record. **The falsifying acts are exactly the ones each sentence names as
belonging to someone else**, so what is stale is the tense and not the reasoning. `completion-report.md` point 5's
dashboard figure of **15 warnings** is stale in the same way and for the reason measured above.

## What this record does not claim

- **It does not claim verification.** The status is `ready`. The transition to `verified` is the assurance owner's act on
  this evidence at this commit, and no part of this record anticipates its outcome.
- **It does not claim the three OUTSTANDING provisions are ratified**, and it does not claim `WO-MOK-018`'s approval
  covers them.
- **It does not close `VER-MOK-005` row by row.** 84 of the 88 automated cases are claimed only by the suite passing;
  four are mapped above.
- **It takes no manual assessment**, and `VER-MOK-005` contracts none for either new case.
- **It does not reach Tier 3.** `fear` for a living selected subject, the cause of death, the encounter tally, a direct
  filter jump to the three new event types, and any canvas indication of an engagement are all in *Out of scope*, and
  `cycle_type_filter` needing up to fifteen presses of `e` is a rule 7 question this work order neither creates nor
  answers.
- **It does not measure rule 10.6's suppressed second line**, and the reason is structural rather than effortful — see
  the residual below.
- **It corrects no committed record and no other work order's evidence.** The two stale sentences above and
  `completion-report.md`'s warning figure are disclosed, not edited.
- **It is not a merge, a tag, a release or a push.** The branch has no upstream, no pull request exists, and
  `WORKFLOW.md` does not govern merging. A record binds a branch commit and never `master`'s merge.
- **It does not amend the managed harness workflow** whose depth-1 checkout will not reproduce the declared digest.

## Residual uncertainty this record carries forward

- **Rule 10.6's clause that a pair carrying neither of its values emits no line at all is not measured by any case, and
  cannot be without a prohibited pattern.** `VER-MOK-005`'s own residual bullet closes with *"Closing it would need a
  line-level assertion below the frame, which is a different kind of case than this contract declares."* The
  measurement is sharper than that, and the difference matters to whoever reads the bullet as an invitation: the
  line-level assertion would need `render::inspector_lines`, private to `render`, together with the no-record state,
  constructible only through `Observer::ingest`, private to `state`. Those modules are **siblings**, so no test module
  is a descendant of both; none of the four `#[cfg(test)]` hooks injects a death; and `ARCH-MOK-002` names both escapes
  — widening an item to reach it from a test, and adding a fifth hook — as prohibited patterns. **A regression that
  emitted the line blank would be invisible to every case in the contract.**
- **`VREC-MOK-005` is `verified` at `f3613701`, which predates all four of `VER-MOK-005`'s later amendments.** Its
  requirement-to-test mapping is the only row-by-row coverage of this contract in the tree, and four of its rows have
  since been withdrawn, amended or added. Whether that mapping should be re-derived is a question this record raises and
  does not answer.
- **A figure restated in prose from a derived value goes stale silently.** That is this work order's own first defect —
  rule 9 item 2 held a count of the engine's event vocabulary and `CAP-MOK-010` moved the vocabulary — and no artifact
  currently obliges a sweep for restated figures. `filter-vocabulary.md` carries it.
- **The `W-HEX-001` observation on `WO-MOK-018` will persist** until either an evidence file is named for the ID or the
  managed rule reads directories. Six implemented work orders carry it today.
- **CI's dashboard warning count will read 33 where this record declares 16**, and its snapshot will not match, for the
  depth-1 reason measured above. The remedy is upstream in the harness distribution or an owner decision to deviate.
- **Non-perturbation is established at five declared seeds and two depths and no further.** `REQ-MOK-025` is measured
  identical in every authoritative record and in final state, with seed 42 at 300 ticks agreeing with `WO-MOK-013`'s
  independently recorded 7,534; a perturbation reachable only at an undeclared seed is outside what was measured.
- **The two temporary oracles are retained and are not in the tree this record binds.** `cargo test --workspace` reports
  269 with the first present and 268 with the second, and neither figure is a census figure; the census is 267.

Every command behind every figure in this record is offline, reads no credential, secret, token or environment value,
and none appears in the retained evidence or in this file.
