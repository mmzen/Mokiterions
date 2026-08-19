+++
id = "VREC-MOK-007"
type = "verification_record"
title = "Verification candidate for WO-MOK-009"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"
commit = "6a5636f998e870ef5691a89fd0407d77bb2a07af"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-19T13:40:07Z"
artifact_snapshot_sha256 = "0ea6384b65910de509ed631b830366901d3a2bbb5968b8ab908e767c5c815d7d"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-009/README.md", "docs/engineering/simulation/evidence/WO-MOK-009/a2-transcript.md", "docs/engineering/simulation/evidence/WO-MOK-009/a5-refusal-ladder.md", "docs/engineering/simulation/evidence/WO-MOK-009/approval-and-transition.md", "docs/engineering/simulation/evidence/WO-MOK-009/candidate-conformance.md", "docs/engineering/simulation/evidence/WO-MOK-009/commit-binding.md", "docs/engineering/simulation/evidence/WO-MOK-009/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-009/compliance-rehearsal.md", "docs/engineering/simulation/evidence/WO-MOK-009/determinism-rehearsal.md", "docs/engineering/simulation/evidence/WO-MOK-009/p4-worktree-comparison.md", "docs/engineering/simulation/evidence/WO-MOK-009/release-artifact-types.md", "docs/engineering/simulation/evidence/WO-MOK-009/scenario-map.md", "docs/engineering/simulation/evidence/WO-MOK-009/snapshot-reproducibility.md", "docs/engineering/simulation/evidence/WO-MOK-009/static-checks.md", "docs/engineering/simulation/evidence/WO-MOK-009/suite-output.md", "docs/engineering/simulation/evidence/WO-MOK-009/toolchain-evidence.md", "docs/engineering/simulation/evidence/WO-MOK-009/verification-output.md"]

[relations]
verifies_work_order = ["WO-MOK-009"]
conforms_to = ["VER-MOK-008"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-009` to candidate commit `6a5636f998e870ef5691a89fd0407d77bb2a07af`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## What this record claims

`WO-MOK-009` is `implemented` and `VER-MOK-008` is `approved`, both by the repository owner's decision of
2026-08-19, which the implementation agent recorded rather than made. At candidate commit
`6a5636f998e870ef5691a89fd0407d77bb2a07af`, **`SPEC-MOK-005`'s fourteen rules, the release workflow and its
authorization gate were verified statically and against constructed fixtures, and the process has never been
run.** The second half is the shape of this record, and it is stated first because it is what the assurance
owner is being asked to weigh rather than a caveat to it.

`VER-MOK-008`'s enumeration holds 65 scenario rows — A1–A5, R1–R24, C1–C5, V1–V6, P1–P4, T1–T5, S1–S11 and
M1–M5. Across them: **47 observed, 3 rehearsed, 15 not performed, 0 unexercisable.** `scenario-map.md` states
each row individually, defines *observed*, *rehearsed* and *not performed* before using them, and reconciles
the 30 automated cases the enumeration does not name against the rule each exists for.

Gates re-run in a clean checkout of this commit, from a pinned `se-harness==0.4.0` wheel:

| Gate | Result |
|---|---|
| `scripts/validate_engineering_artifacts.py --root .` | PASS — 79 artifacts, 0 errors, 0 warnings, all four planes E0/W0 |
| `python -m se_harness doctor .` | exit 0 — 81 verdict lines, 81 PASS, 0 WARN, 0 FAIL |
| `python -m se_harness preflight . --work-order WO-MOK-009 --phase review` | PASS, work order `implemented`, eligible |
| `python -m unittest discover -s scripts -p 'test_check_release_*.py'` | **Ran 70 tests, 0 failed**, 49.652s |
| Rule 7.4's loop — review preflight for every work order in the repository | PASS for `WO-MOK-001` through `WO-MOK-006` and `WO-MOK-009`; **FAIL for `WO-MOK-008`**, which is `draft` |
| `git status --porcelain --untracked-files=all` after all of the above | empty |

The suite was re-run at this commit rather than carried forward, because six of its cases read the real
repository rather than a fixture and the approval transitions changed what they read. The validator's silence
also means more here than it did before those transitions: `E007` and `E008` exempt draft requirements, so
`REQ-MOK-035` through `REQ-MOK-039` are now checked for active specification and verification coverage where
before they were skipped, and `SPEC-MOK-005`'s `specifies` and `VER-MOK-008`'s `verifies` are what answer for
them.

`WO-MOK-001`'s review preflight now passes on a committed tree, which `compliance-rehearsal.md` could not
claim: its rehearsal records that the PASS depended on an uncommitted `[assurance]` table and would have been a
`[W023]` FAIL at the governance revision. That edit is committed here — and it is the same edit finding 1 flags
as outside this work order's declared change surface, which the engineering owner affirmed on 2026-08-19 and
chose to keep. **The qualification is discharged by a change that is itself a disclosed deviation**, and both
halves of that sentence belong in front of an assurance owner.

Four questions this work order raised were answered by the owners on 2026-08-19 and are inside what this record
binds: `WO-MOK-001`'s classification affirmed and the deviation left standing as a finding; `VER-MOK-008` C5
amended so the row is *observed* rather than unexercisable; `SPEC-MOK-005` rule 12.5 amended to grant write
access per job, because GitHub Actions scopes `permissions` per job and the original per-step wording had no
expressible form; and rule 14's ordering read as the runbook has it, with nothing changed.

## What this record does not claim

**The process has never been run, so fifteen scenario rows are unmeasured rather than passing.** V1–V6, P1–P3
and C4 are observations of a run; there is no tag, no release record, and the candidate commit sits on an
unmerged branch. Committing is not a run and neither is this record. `completion-summary.md` lists the fifteen
individually with what each needs.

**`VER-MOK-008` M2 cannot be closed by this work order at all.** It requires the produced archive to be read by
someone who did not build the packaging step, and that person is this work order's author. It closes on a first
run read by someone else, not by further evidence in this directory.

**`candidate-conformance.md`'s "before" column is a reconstruction, not a diff.** No revision of this
repository holds the pre-existing candidate in its pre-conformance form — `17be4ba` adds all 34 files at once —
so every claim about what the candidate satisfied before the work rests on the change record kept while it was
done. Every claim about the *current* state is checkable against this commit.

**The architecture question rests on a reading of an approval.** `WO-MOK-009`'s approval precondition 2 asks
the technical owner whether the release process needs an architecture artifact. The owner approved the pack as
written — no `architecture` relation, no `ARCH`, no `ADR` — so the decision recorded is that none is required
and `SPEC-MOK-005` governs the release process alone. That is inference from an approval rather than a
separately worded decision; it is named as the weakest link in `approval-and-transition.md` and in the work
order's own *How each precondition was discharged* section, and it is the one item in this packet a reader
should check rather than accept. No measurement moves under either reading.

Four further limits, stated because they bound what the green gates above mean:

- **Rule 12.6's `environment: release` is a reference, not a protection.** Nothing in the repository configures
  that environment, so the second human gate the rule contemplates does not exist until a release owner
  creates it. Finding 4.
- **`REPOSITORY_CONTEXT.md`'s declared lint command lacks `--locked`** while rule 8.2 requires resolution
  against the committed lockfile. Finding 5.
- **`prepare-release` was never exercised**, because it takes a verified verification record as input and none
  existed. `release-artifact-types.md` establishes that both the release record and the release contract are
  creatable and names the two nearly identical directory names the harness enforces; the command itself remains
  untried.
- **The machine's own harness install is a moving target**, an editable clone that moved from `0.4.0` to
  `0.4.1` during the work while this repository declares `0.4.0`. Every measurement in this packet comes from a
  pinned `0.4.0` wheel instead. The mismatch is not only a hazard — it is rule 7.1's one piece of *observed*
  evidence, recorded as `compliance-rehearsal.md` C1.

Nothing here bears on the correctness of the simulation. `WO-MOK-009` changes no engine and no observer source
file, and `VER-MOK-001` through `VER-MOK-006` remain the contracts for that.

## The snapshot field, and what it is reproducible from

`artifact_snapshot_sha256` above is the SHA-256 of a generated `target/harness-dashboard/dashboard-data.json`,
and that JSON records **the name of the directory the repository is checked out into** as the project name. So
the field binds the commit *and* the checkout directory's basename.

To reproduce it: clone this repository so the working directory is named `Mokiterions`, check out
`6a5636f998e870ef5691a89fd0407d77bb2a07af`, and run `capture-verification` with the arguments this record's
provenance names. That is how this record was prepared, and it is why it was prepared in a throwaway clone
rather than in the linked worktree the work was done in — that worktree is named `Mokiterions-release-ci` and
yields a different hash from the same commit. Measured rather than assumed:

| Checkout directory basename | `artifact_snapshot_sha256` at this commit |
|---|---|
| `Mokiterions`, which is what a plain `git clone` produces | `0ea6384b65910de509ed631b830366901d3a2bbb5968b8ab908e767c5c815d7d` |
| `othername`, a control clone of the same commit | `15fe74b030a9bc0c4f45a9d5e4c3c094c6e958f72861bf7577f20cceb1628b83` |

Two consecutive runs in the same clone agree exactly, and the two captures above differ in `verified_at` and
this field and in nothing else — no other provenance field is a property of the checkout.
`snapshot-reproducibility.md` holds the measurement across five checkouts, the one differing line out of a
~170 KB JSON that identifies the cause, and the hypothesis it refuted: line endings are not in the hash. It is
not a defect against `SPEC-MOK-005`, because no rule and no `VER-MOK-008` scenario concerns this field and
`check_release_authorization.py` never recomputes one.

The snapshot was taken before this record existed, so it records the 79-artifact graph the record binds rather
than the 80-artifact graph containing it.

## What the accountable assurance owner is being asked to decide

Whether the evidence at this candidate commit is sufficient to call `WO-MOK-009` verified **with the process
unrun, fifteen scenario rows not performed, M2 unclosable by this author, the "before" column a
reconstruction, and the architecture decision resting on a reading**. That is the whole of the decision. The
limitations are listed above and as seven findings in `completion-summary.md` rather than summarized away,
because a disclosure that survives only until it is accepted is not a disclosure.

Verifying this record would not merge pull request #20, approve `WO-MOK-008`, tag anything, create a release
record, configure the `release` environment, or perform the first run. Each is a separate act with its own
accountable owner, and `completion-summary.md`'s next-steps table names them with whose they are.

## Authority

**This record is `ready`, and only the accountable assurance owner may make it `verified`.**
`docs/engineering/DECISION_RIGHTS.md` states that automation may prepare `ready` verification and release
records from bounded Git observations and that only accountable assurance and release owners may transition
them, and `ENGINEERING_HARNESS.md` states that harness commands may prepare records but never exercise
accountable decision rights. The managed `capture-verification` command produced every provenance field above;
none was hand-edited, and this prose was appended below the generated body without altering it.

That command refused to run while `WO-MOK-009` was `draft`, and the transcript of the refusal is retained in
`commit-binding.md`. It succeeded only after the owner approved the governing chain, which
`approval-and-transition.md` records together with the ten status transitions and the one reading in them to
check. That approval is a different role's act: this record neither supplies, ratifies nor substitutes for it,
because an assurance owner's verification is a statement about evidence rather than about another role's
approvals.

The record is committed after the candidate commit it names, as `docs/engineering/WORKFLOW.md` requires of a
record that cannot contain the hash of its own commit.

`WO-MOK-008` is `draft` and untouched by any of this. Pull request #20 is open and unmerged. The repository
holds no tag, and no release-record or release-contract directory exists in any revision, which is why
`a5-refusal-ladder.md` shows rule 4 refusing at every plausible tag. Verification is not release, and it is
not merge.
