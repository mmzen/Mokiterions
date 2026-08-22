+++
id = "VREC-MOK-023"
type = "verification_record"
title = "Verification candidate for WO-MOK-020"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-22"
updated = "2026-08-22"
commit = "f633edaebe712ad8e6b139691db001458cf87867"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-22T16:47:09Z"
artifact_snapshot_sha256 = "b8c6c422d3b85f1513538b52f5e5d57b65d5e886be179eaa51476837bfff4616"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-020/01-format-candidate.txt", "docs/engineering/simulation/evidence/WO-MOK-020/02-lint-candidate.txt", "docs/engineering/simulation/evidence/WO-MOK-020/03-test-candidate.txt", "docs/engineering/simulation/evidence/WO-MOK-020/04-format-and-lint-base.txt", "docs/engineering/simulation/evidence/WO-MOK-020/05-test-base.txt", "docs/engineering/simulation/evidence/WO-MOK-020/06-engine-records-seed42-candidate.jsonl", "docs/engineering/simulation/evidence/WO-MOK-020/07-engine-stream-seed42-candidate.txt", "docs/engineering/simulation/evidence/WO-MOK-020/08-engine-output-unmoved.txt", "docs/engineering/simulation/evidence/WO-MOK-020/09-measure-spec-mok-004.py", "docs/engineering/simulation/evidence/WO-MOK-020/10-spec-mok-004-measured.txt", "docs/engineering/simulation/evidence/WO-MOK-020/11-independent-count.txt", "docs/engineering/simulation/evidence/WO-MOK-020/12-frames-and-columns.txt", "docs/engineering/simulation/evidence/WO-MOK-020/13-extinction-frame.txt", "docs/engineering/simulation/evidence/WO-MOK-020/14-long-run-truncation.txt", "docs/engineering/simulation/evidence/WO-MOK-020/15-per-tick-cost-driver.rs", "docs/engineering/simulation/evidence/WO-MOK-020/15-per-tick-cost.txt", "docs/engineering/simulation/evidence/WO-MOK-020/16-export-capture-driver.rs", "docs/engineering/simulation/evidence/WO-MOK-020/17-export-unmoved.txt", "docs/engineering/simulation/evidence/WO-MOK-020/18-dependency-sets-unmoved.txt", "docs/engineering/simulation/evidence/WO-MOK-020/19-harness-validate-and-preflight.txt", "docs/engineering/simulation/evidence/WO-MOK-020/MANIFEST.sha256", "docs/engineering/simulation/evidence/WO-MOK-020/README.md", "docs/engineering/simulation/evidence/WO-MOK-020/completion-report.md"]

[relations]
verifies_work_order = ["WO-MOK-020"]
conforms_to = ["VER-MOK-017"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-020` to candidate commit `f633edaebe712ad8e6b139691db001458cf87867`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## The decision: transitioned to `verified` on 2026-08-22

The repository owner, acting as accountable **assurance owner**, took this transition on 2026-08-22. Their
instruction, verbatim:

> i validate the verification record that can be transitioned, you can also commit + push, I will merge the PR
> (ignore PR #44)

**`status` moved from `ready` to `verified`, and nothing else in the frontmatter moved.** The harness at 0.4.0 has no
`transition` subcommand, so the field was hand-edited in this commit; `commit`, `worktree_state`, `verified_at`,
`artifact_snapshot_sha256`, `evidence_paths` and both relations stand exactly as `capture-verification` wrote them.
So `verified_at = "2026-08-22T16:47:09Z"` is the **capture** time and not the decision time: the tool wrote it, and a
transition that moves only `status` does not get to rewrite provenance to read better. The document's heading still
says *Candidate* for the same reason.

### The seven manual assessments, now recorded

`VER-MOK-017` requires each assessment "with the assessing role, the date, and the evidence read" and states that **an
unsigned assessment is not a recorded one**. All seven were put to the owner on 2026-08-22, each separately and each
with the evidence displayed rather than described, and each was answered separately. **One person holds all three
roles in this repository**, which is disclosed here rather than left to be inferred: no assessment was answered by
implication from another, and the three roles are that person's three distinct acts.

| # | Role | Assessment | Recorded answer | Evidence read |
|---|---|---|---|---|
| 1 | product owner | Does the pane answer the question it was added for? | **Met.** A Mokiterion's profile is legible from the two pane states without reference to the log. | The selected and no-selection panes of `12-frames-and-columns.txt`, displayed in full |
| 2 | product owner | Can the population state be misread as one Mokiterion's? | **Met — it cannot.** `nothing selected` and the selecting hint stand above the figures, and the heading reads `population activity` rather than a name and an identifier. | The same two panes and the extinction pane of `13-extinction-frame.txt`, judged as frames and not against the specification text, as the row requires |
| 3 | technical owner | Is the rule 10 item 7 amendment sound? | **Met on all three grounds.** Removing `kills` and `combats` follows the 2026-08-19 procedure; the suffered-attack ground is untouched; and the amendment states the zero-is-a-measurement distinction rather than assuming it. | `SPEC-MOK-003` rule 10 item 7 and its amendment row, and the ground recorded in `f633eda` that `attack_resolved` states a fatal strike and each conflict verb is a reported record |
| 4 | technical owner | Are `WO-MOK-020` §3's two corrections correct and correctly scoped? | **Met on both.** The *State model* table can now be read as exhaustive at twenty-four fields, and rule 4's corrected sentence still forbids deriving a name, which is the clause `REQ-MOK-041` rests on. | `WO-MOK-020` §3, the amended table, rule 4's corrected sentence, and the measured cost of the declined alternative recorded in `f633eda` — `name_of` plus fourteen references, two failing tests and a `REQ-MOK-041` amendment |
| 5 | assurance owner | Is `REQ-MOK-059` still met, and are O17 and O19 sufficient evidence? | **Met, and sufficient.** The aggregate exists only where nothing the engine reads can see it. | `O19.1`–`O19.4` and `O17` as executed: `03-test-candidate.txt`, `08-engine-output-unmoved.txt` and `18-dependency-sets-unmoved.txt` |
| 6 | assurance owner | Are the `SPEC-MOK-004` figures right? | **Met.** Rule 9's amended `117` was recomputed independently of the retained output and agrees. | A static count of `#[test]` across the eight public-tier files — 4, 7, 11, 8, 29, 7, 22, 29 — summing to **117**, taken without reading `10-spec-mok-004-measured.txt`, which is the `cargo test -- --list` output the row forbids relying on |
| 7 | technical owner | Are the labels the engine's own vocabulary rather than an interpretation of it? | **Met.** | `mokiterions-tui/src/state.rs:206`–`214` read against `mokiterions-core/src/simulation.rs:615`–`623`: nine of the eleven labels are byte-identical, and `eat` and `move` are the engine's verb without the payload its records carry after the colon, because a label heads a count of a kind and `eat:f07` is not a kind |

**Two things about assessment 6 are stated rather than glossed.** The contract asks the *assessor* to recompute, and
the recomputation was performed by the implementation agent and shown to the owner, who accepted it. So this row
records the owner's **reliance on a measurement the agent took**, not the owner's own arithmetic; the method and the
eight per-file counts are written above so that anyone can repeat it in one command. And the recount is independent of
the retained output but not of the tree: it reads the same test files `cargo test -- --list` enumerates.

**Assessment 7 carries one finding worth naming**, because it is the kind of thing that goes silently stale. The
observer's label strings are its own copy, not a call into the engine's `Display`, and **no test asserts the two
sides agree** — nor can one easily, because `O19.2` forbids the engine from naming the observer's type at all. The
parity above was established by reading both sites at this commit. The owner assessed the labels as met on that basis
and declined to make a parity test a condition of this transition.

### What this transition supersedes in the text below

The sections that follow are the candidate's own case, left **unedited**, because they are the record of what was
true when the decision was asked for and a transition adds a decision rather than rewriting the argument for it.
Four statements in them are superseded by this section and by nothing else:

1. **"All seven of `VER-MOK-017`'s manual assessments are outstanding"**, in the next section. They are recorded
   above.
2. **The coverage table's *Manual assessments* row, "none answered".** Read it as 7 of 7 answered, above.
3. **"That the change is verified"**, in *What this record does not claim*. It is now claimed, by the owner and on
   this date.
4. **"That a human has seen the pane"** — superseded only in part. Assessments 1, 2 and 7 are recorded, and they
   were taken against rendered-buffer captures, because no interactive terminal exists in this environment. That
   remains `VER-MOK-017`'s *Residual uncertainty* 2 and is carried forward unchanged.

Every other claim, figure and caveat below stands as written and is part of what is verified here.

### The figures re-measured before the field moved

A `verified` record can never be corrected, so this commit is the last point at which any figure in it could be
fixed. Each was measured again at this commit, in this checkout, from the pinned 0.4.0 environment. **All seven are
identical to what the record already states, so nothing was corrected.**

| Re-measured | Result | Matches the record |
|---|---|---|
| `cargo fmt --all -- --check` | `exit=0` | yes |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | no warning, no error | yes |
| `cargo test --workspace --locked` | 332 passed, 0 failed, 0 ignored | yes |
| `se_harness validate` | PASS, 0 errors, 0 warnings, **153** artifacts; **152** with this file moved aside and the worktree clean again afterwards | yes |
| `se_harness preflight --work-order WO-MOK-020 --phase review` | PASS | yes |
| `sha256sum -c MANIFEST.sha256` | 22 of 22 `OK`, exit 0 | yes |
| `evidence_paths` against `git ls-files` | 23 and 23, equal as sets | yes |

The identifier was also re-checked after a fetch against **every** local and remote ref, not against the local
maximum: `VREC-MOK-023` exists on this branch alone, and `WO-MOK-020` on this branch and its remote alone, `master`
having renumbered its own to `WO-MOK-024`.

### What `verified` authorizes, and what it does not

It authorizes nothing beyond itself. **No release, tag or publish** — that needs a `REL-` record and none is
written. It does not discharge the reconciliation `SPEC-MOK-004`'s ratified row owes at the next merge, it does not
make `docs/ROADMAP.md` accurate, and it does not retire the residual uncertainties below.

The owner authorized the push of this branch in the same instruction and **reserved the merge to themselves**. The
constraint in *This branch must not be rebased* is now load-bearing rather than advisory: a squash-merge produces a
different commit object, and this record can be neither re-pointed at it nor corrected, because it is `verified`.
**Merge, do not squash and do not rebase.** If integration is taken as a squash anyway, the remedy is a successor
record with every field measured afresh.

## What this record is, and why it is `ready` rather than `verified`

`harnessctl capture-verification` wrote the two paragraphs above and the frontmatter, and it prepared this record
at `ready`. **It is not a verdict.** Everything below was measured by the implementation agent at the bound commit
and is presented so that the accountable assurance owner can decide; the agent is not a party that can verify its
own work, which is why `VER-MOK-017`'s *Independence* section exists and why the seven manual assessments are
reserved to the owners.

**One thing stands between this record and `verified`, and it is not a measurement.** All seven of
`VER-MOK-017`'s manual assessments are outstanding. Two are the product owner's (1, 2), three the technical
owner's (3, 4, 7) and two the assurance owner's (5, 6). The technical owner's ratification of both amendment rows
on 2026-08-22 decided the substance assessments 3 and 4 turn on — whether the rule 10 item 7 amendment is sound,
and whether the two corrections are correct and correctly scoped — but **an assessment is a statement recorded in
a verification record, and no such statement exists yet.** Assessment 6 is deliberately not answered by the
packet: it requires recomputing at least one amended `SPEC-MOK-004` figure independently of the retained command
output, and `10-spec-mok-004-measured.txt` *is* that output.

## The candidate commit

The bound commit is `f633edaebe712ad8e6b139691db001458cf87867`, the governance commit that ratified both
amendment rows and moved `WO-MOK-020` to `implemented`. It is bound rather than the implementation commit
`5a09ec4dc8fdb038d6b5784ec52672824a2c5f1f` for one reason: at `5a09ec4` both amendment rows read **OUTSTANDING**,
so a record bound there would bind a chain whose specification coverage was unratified.

**The source tree is identical at both commits, and that is measured rather than assumed.**
`git diff --stat 5a09ec4 f633eda -- mokiterions-core mokiterions-tui Cargo.toml Cargo.lock` is empty. So every
transcript in the packet, whose header states the candidate as "`HEAD=ccb0584` plus the `WO-MOK-020` change
surface", describes the source tree of the commit this record binds, byte for byte, even though it was taken one
commit earlier. What `f633eda` changed is six documents: two specifications, the work order's `status`, and the
packet's `README.md`, completion report and `MANIFEST.sha256`.

## Where the candidate stands relative to `master`

Six commits ahead of `origin/master` and **twelve behind** it, measured at capture — that is, before the commit
that adds this file, which makes it seven ahead. The branch has not been pushed and no pull request contains this
record.

**`master` moved under this chain in a way worth naming.** Between `d4a17c1`, the `master` merged into `ccb0584`,
and `3ca2028`, another agent's work order **also numbered `WO-MOK-020` was renumbered to `WO-MOK-024` and its
record `VREC-MOK-020` to `VREC-MOK-022`** (commit `cdd88dd`), which is why this chain keeps `WO-MOK-020` and why
this record is `VREC-MOK-023`: `021` is taken by the unmerged `wo-mok-008-footer-shedding` branch and `022` by
`master`. The identifier was checked against every local and remote ref after a fetch, not against the local
maximum, which is `020` here and would have been wrong.

**Two prospective merges were measured, in worktrees that have been discarded.** Neither figure is this record's
claim; both are stated so that the reconciliation `SPEC-MOK-004`'s ratified row owes is a known quantity rather
than a surprise.

| Prospective merge | Conflicts | Suite | Rule 9 | Rule 10 | Observer | Workspace |
|---|---|---|---|---|---|---|
| this candidate as it stands | — | 332 passed, 0 failed | 117 | 58 | 175 | 332 |
| `origin/master` merged in | one, `SPEC-MOK-003`'s amendment table only; both source files auto-merge | 333 passed, 0 failed, `fmt` and `clippy` clean | 118 | 58 | 176 | 333 |
| `origin/wo-mok-008-footer-shedding` merged in | one, the same table; both source files auto-merge | 342 passed, 0 failed | 119 | 66 | 185 | 342 |

In both cases the only conflict is two amendment rows of the same date meeting in one table, resolved by keeping
both, with no provision of either read or touched. The combined merge of both was **not** measured, so no figure
is offered for it.

## The gates, re-measured at this commit

| Check | Command | Result |
|---|---|---|
| Format | `cargo fmt --all -- --check` | `exit=0` |
| Lint | `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | no warning, no error |
| Suite | `cargo test --workspace --locked` | **332 passed, 0 failed, 0 ignored** |
| Artifacts | `se_harness validate` | **PASS**, 0 errors, 0 warnings; **153** artifacts with this record present, **152** with it removed |
| Preflight | `se_harness preflight --work-order WO-MOK-020 --phase review` | **PASS** |
| Evidence bytes | `sha256sum -c MANIFEST.sha256` | every one of the 22 hashed files `OK` |
| Evidence paths | the 23 `evidence_paths` above against `git ls-files` for the packet | equal, as sets, 23 and 23 |

Both harness commands were run from the pinned 0.4.0 environment,
`C:/Users/mathi/harness-venv-040/Scripts/python.exe -m se_harness`. The machine-wide 0.4.1 is not used, because
it fails on template skew alone and its verdict would say nothing about this repository.

## The coverage this record claims in `VER-MOK-017`

Every automated row of the contract was executed at this commit. The transcript for each row that is a test is
`03-test-candidate.txt`.

| Contract section | Rows | Claim |
|---|---|---|
| Requirement-to-evidence matrix | 20, `O1`–`O20` | all executed. `O1` is `11-independent-count.txt`: five declared seeds, twelve subjects each, **60 subject rows and 60 all-zero difference rows**, compared after *every* completed tick rather than at the four the row requires. `O8` is `14-long-run-truncation.txt`, executed to truncation. `O14` is `13-extinction-frame.txt` at tick 119. `O16` is `12-frames-and-columns.txt`, widest line 40 of interior 42 at the reference viewport and at the presence threshold. `O17` is `08-engine-output-unmoved.txt`, ten digest pairs identical. `O18` is `17-export-unmoved.txt`. `O19` and `O20` are the static checks below. The remainder are tests. |
| Acceptance scenarios | 6 | all six covered. Scenario 5 is the extinction frame reached with nothing pressed; scenario 6 — a viewport below the inspector's threshold — is `both_pane_states_present_the_same_figures_as_the_overlay_below_the_threshold`. |
| Property and invariant tests | 6, `P1`–`P6` | all six are tests. `P4`'s bound is also executed rather than argued: 12 records of 120 bytes plus 36 bytes of identifiers at tick 6,600, with 100,000 retained events of 100,000 capacity and `truncated true`. |
| Static and architecture checks | 5, `O19.1`–`O19.4`, `O20.1` | all five are tests and all five pass: the engine declares no dependency on the observer, no profile identifier appears under `mokiterions-core/`, the engine still exposes exactly two mutating entry points (`run` at `simulation.rs:2004`, `advance_tick` at `2101`), no total can reach an observation or a decision source, and no total is a float and no presented figure is a ratio. |
| Security and privacy checks | 3 | all three answered. No frame carries an environment value (a pre-existing test); the export carries no path, no host user name and no credential pattern, measured on the exported bytes in `17-export-unmoved.txt`; and the whole packet was scanned case-insensitively for thirteen credential patterns before `MANIFEST.sha256` was written, with every hit accounted for in `README.md`. |
| Performance and resilience | `O8`, `P4` | both executed. The per-tick cost is retained as a **seven-configuration series with an upper bound of about 25 µs/tick**, not as a before-and-after pair, and the reason is stated below. |
| Evidence retention | 8 bullets | all eight answered, **four by substitution**, each stated on its bullet in `README.md` rather than by reinterpreting the bullet. |
| Manual assessments | 7 | **none answered.** See above. |

**The four substitutions, named because they are the difference between what the contract asked for and what is
retained.** The export is retained once and by tested reconstruction rather than as a third near-duplicate copy;
the engine's streams are retained in full for one declared seed of five, with all ten digests and the recompute
command for the other eight; no interactive terminal run is retained, because no interactive terminal exists in
this environment, so the pane evidence is rendered-buffer captures; and the per-tick cost is a series and a bound
rather than a pair.

## The snapshot figure, and what it is a figure of

`artifact_snapshot_sha256 = b8c6c422d3b85f1513538b52f5e5d57b65d5e886be179eaa51476837bfff4616`.

It is a digest over the **declared evidence paths and the repository's state at capture**, not over the evidence
bytes. It moves if a declared path is renamed, if `HEAD` moves, if the checkout's directory basename differs, or
if the clone depth differs — so **it is not reproducible in another checkout of the same commit**, and a reader
who recomputes it elsewhere and gets a different value has found nothing. **It is also a figure of the tree
*without* this record**, because the tool builds the snapshot before it writes the file: recomputing it with this
record present gives a different digest, which is nobody's declared value. The byte-level check on the evidence is
`MANIFEST.sha256`, which hashes 22 of the packet's 23 files and verified `OK` at this commit. The manifest cannot
hash itself, and it cannot state its own digest; that digest is
`1c010607e44169bc954b9235f6221513b533e627421bc93d97cdaeedd775bbc8` and it is recorded in `f633eda`'s message.

## Statements bound by this record that have already aged

Nothing in the packet may be edited now without falsifying this binding, so the corrections are stated here.

1. **`19-harness-validate-and-preflight.txt` records `validate` at 152 artifacts.** True when captured and true at
   the bound commit; it reads **153** the moment this record exists as a file. Both figures are measured above.
2. **The `SPEC-MOK-003` cell ratified in `f633eda` says of the packet that "no commit-bound record binds that
   packet".** That was true of the act it describes and stops being true when this record is committed. It is not
   corrected in the specification, because the sentence was a statement about the ratification commit's own
   reach; this record is the thing that changes it, and this line is the record of that.
3. **Every transcript header names the candidate as "`HEAD=ccb0584` plus the `WO-MOK-020` change surface".** That
   describes the source tree of both `5a09ec4` and `f633eda`, measured identical. Read it as the source tree, not
   as the commit this record binds.

## What this record does not claim

- **That the change is verified.** This record is `ready`. `verified` is the assurance owner's act, taken by
  moving the `status` field and nothing else, and it requires the seven manual assessments.
- **That the change is released.** `verified` authorizes no release, tag or publish; a `REL-` record exists for
  that and none is written.
- **That the figures survive a merge.** They are measured against `ccb0584` on a branch commit that is not an
  ancestor of `master`. The table above measures how two prospective merges move them, and `SPEC-MOK-004`'s
  ratified row already owes a reconciliation for exactly this reason.
- **That the accumulation's per-tick cost is 25 µs.** That is an *upper bound* set by the instrument's
  resolution. The default release profile measures 266.3 µs/tick at this candidate against 153.0 at the base;
  three configurations establish that as codegen-unit partitioning in a file the measured path never calls, and
  at `codegen-units = 1` the candidate is faster than the base. The roughly 113 µs is real for anyone running the
  default build and no artifact carries a per-tick budget that would receive it.
- **That rule 10.6's retention is still exercised on a terminal death.** The dead-selection fixture was
  retargeted from `--policy baseline` to `--policy reference`, because `baseline`'s first death *is* extinction
  and the new clause 9 clears the selection there. Every assertion holds as written, on a mid-run death.
- **That a human has seen the pane.** No interactive terminal run is retained. Manual assessments 1, 2 and 7 are
  the acts that would close that, and they are outstanding.
- **That `docs/ROADMAP.md` is accurate.** Its entry still records these four artifacts as `draft` and the
  amendments as unwritten. `WO-MOK-020` puts the roadmap out of its own scope, so it is reconciled under its own
  change and not here.

## This branch must not be rebased, and a squash would orphan this record

A rebase rewrites `f633eda` and orphans the commit this record binds. So does a squash-merge of the pull request
this branch will carry: the squashed commit is a different object, and this record can be neither re-pointed at
it nor corrected once `verified`. If `master`'s later work is needed, **merge** it — the trial above shows the
merge is clean but for one additive amendment-table conflict. If integration is taken as a squash anyway, the
remedy is a **successor record** with every field measured afresh, not an edit of this one; more than one record
per work order is normal and expected here.

## Residual uncertainty carried forward

`VER-MOK-017`'s five, unchanged and not narrowed by anything measured here: both sides of every count read the
same engine, so the oracles are independent of the observer and not of the engine; frames are presentation and
carry no claim of record; `O3`'s identity assumes one decision record per Mokiterion per completed tick;
`O10`'s monotonicity is checked on the declared seeds, so a non-monotone total is detectable rather than
impossible; and the performance figures are indicative, retained so a later regression has a baseline.

Two are this record's own. The per-tick instrument cannot separate the accumulation's cost from noise, which is
why the figure is a bound. And the absence of an interactive terminal means every pane claim rests on a rendered
buffer, which is what `VER-MOK-017`'s *Residual uncertainty* 2 already says a frame can and cannot evidence.
