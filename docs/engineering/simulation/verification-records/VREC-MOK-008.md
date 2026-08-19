+++
id = "VREC-MOK-008"
type = "verification_record"
title = "Verification candidate for WO-MOK-009"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"
commit = "d35f8172a0f91049aa2719bc34ca9dd7584f4380"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-19T14:11:54Z"
artifact_snapshot_sha256 = "0eeb2e47b1b4c4c19fa78ae19d388bb5960bb3e1d659e3f07e1ebdb74084ed64"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-009/README.md", "docs/engineering/simulation/evidence/WO-MOK-009/a2-transcript.md", "docs/engineering/simulation/evidence/WO-MOK-009/a5-refusal-ladder.md", "docs/engineering/simulation/evidence/WO-MOK-009/approval-and-transition.md", "docs/engineering/simulation/evidence/WO-MOK-009/candidate-conformance.md", "docs/engineering/simulation/evidence/WO-MOK-009/commit-binding.md", "docs/engineering/simulation/evidence/WO-MOK-009/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-009/compliance-rehearsal.md", "docs/engineering/simulation/evidence/WO-MOK-009/determinism-rehearsal.md", "docs/engineering/simulation/evidence/WO-MOK-009/id-collision.md", "docs/engineering/simulation/evidence/WO-MOK-009/p4-worktree-comparison.md", "docs/engineering/simulation/evidence/WO-MOK-009/release-artifact-types.md", "docs/engineering/simulation/evidence/WO-MOK-009/scenario-map.md", "docs/engineering/simulation/evidence/WO-MOK-009/snapshot-reproducibility.md", "docs/engineering/simulation/evidence/WO-MOK-009/static-checks.md", "docs/engineering/simulation/evidence/WO-MOK-009/suite-output.md", "docs/engineering/simulation/evidence/WO-MOK-009/toolchain-evidence.md", "docs/engineering/simulation/evidence/WO-MOK-009/verification-output.md"]

[relations]
verifies_work_order = ["WO-MOK-009"]
conforms_to = ["VER-MOK-008"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-009` to candidate commit `d35f8172a0f91049aa2719bc34ca9dd7584f4380`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## What this record claims

`WO-MOK-009` is `implemented` and `VER-MOK-008` is `approved`, both by the repository owner's decision of
2026-08-19, which the implementation agent recorded rather than made. At candidate commit
`d35f8172a0f91049aa2719bc34ca9dd7584f4380`, **`SPEC-MOK-005`'s fourteen rules, the release workflow and its
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
| `python -m unittest discover -s scripts -p 'test_check_release_*.py'` | **Ran 70 tests, 0 failed**, 49.747s — in a clone holding no tags; see the qualification below |
| Rule 7.4's loop — review preflight for every work order in the repository | PASS for `WO-MOK-001` through `WO-MOK-006` and `WO-MOK-009`; **FAIL for `WO-MOK-008`**, which is `draft` |
| `git status --porcelain --untracked-files=all` and `git tag --list` after all of the above | both empty |

**The suite row carries a qualification, and it is a finding rather than a footnote.** One test guards its
scenario by asserting that the repository under test holds no tags at all. It failed once during this work, on
a lightweight tag another session had left in this machine's shared object store — a tag on a stash commit,
present on no branch of this changeset and on no remote. The same suite is 70/70 in the same clone once that
tag is removed, and the gate refused identically either way: what failed is a guard, not the assertion that
discharges the scenario. The sharper form of the same defect was scheduled rather than accidental — the
runbook's Phase G told the operator to create the release tag and *then* run the suites, which would have
failed this test on every release. Phase G is reordered in this changeset; the one-line narrowing of the guard
is **not** taken, because it would change a declared file after `WO-MOK-009` was declared `implemented`.
`suite-output.md` holds the failure verbatim and the two-run comparison; `completion-summary.md` finding 9
holds the recommendation.

The suite was re-run at this commit rather than carried forward, because six of its cases read the real
repository rather than a fixture and both the approval transitions and this changeset's deletions changed what
they read. The validator's silence also means more here than it did before those transitions: `E007` and `E008`
exempt draft requirements, so `REQ-MOK-035` through `REQ-MOK-039` are now checked for active specification and
verification coverage where before they were skipped, and `SPEC-MOK-005`'s `specifies` and `VER-MOK-008`'s
`verifies` are what answer for them.

`WO-MOK-001`'s review preflight now passes on a committed tree, which `compliance-rehearsal.md` could not
claim: its rehearsal records that the PASS depended on an uncommitted `[assurance]` table and would have been a
`[W023]` FAIL at the governance revision. That edit is committed here — and it is the same edit finding 1 flags
as outside this work order's declared change surface, which the engineering owner affirmed on 2026-08-19 and
chose to keep. **The qualification is discharged by a change that is itself a disclosed deviation**, and both
halves of that sentence belong in front of an assurance owner.

Five questions this work order raised were answered by the owners on 2026-08-19 and are inside what this record
binds: `WO-MOK-001`'s classification affirmed and the deviation left standing as a finding; `VER-MOK-008` C5
amended so the row is *observed* rather than unexercisable; `SPEC-MOK-005` rule 12.5 amended to grant write
access per job, because GitHub Actions scopes `permissions` per job and the original per-step wording had no
expressible form; rule 14's ordering read as the runbook has it, with nothing changed; and the architecture
precondition confirmed, which the next section takes up.

## Why this record is `VREC-MOK-008` and not `VREC-MOK-007`

The record it replaces was prepared as `VREC-MOK-007` and could never have merged. `master` moved during this
work, and pull request #19 merged a **`verified`** `VREC-MOK-007` verifying `WO-MOK-007` at a different commit.
Pushing the branch surfaced it as the pull request's only merge conflict — one add/add on that single file. An
ID census across both sides, in `id-collision.md`, finds 69 shared IDs, every one at an identical path, and
this as the only collision; the eleven governed artifacts this work order introduces, `VER-MOK-008` among
them, are all free of `master`.

Renumbering meant re-capturing rather than renaming, because the ID is inside the file the managed command
produces. The superseded file was deleted in one commit and this record was captured against a later one, so
nothing about the earlier ID survives in the graph. **Nothing else moved**: the same work order, the same
verification contract, the same evidence, and no measurement recomputed on account of the ID.

Two consequences an assurance owner should see rather than discover. First, **this work order's own runbook
prescribed the check that would have caught it** — a `git ls-tree` against `origin/master` — and named the
claimant in advance: *"the in-flight `WO-MOK-007` will most likely claim `007`"*. The instruction was not
followed, which is why the runbook's Phase D now cites the incident instead of merely warning about it.
Second, **three references to the old ID remain in the repository and are deliberately not edited**:
`SPEC-MOK-005` line 486 and `REQ-MOK-037` lines 127 and 130 use it as a worked example, and both artifacts are
`approved`, so amending them is the technical and product owners' act rather than this record's; the other two
are transcripts of commands that were run, and rewriting a transcript would make it something else.
`id-collision.md` states the recommended disposition for each.

## What this record does not claim

**The process has never been run, so fifteen scenario rows are unmeasured rather than passing.** V1–V6, P1–P3
and C4 are observations of a run; there is no release tag, no release record, and the candidate commit sits on
an unmerged branch. Committing is not a run and neither is this record. `completion-summary.md` lists the
fifteen individually with what each needs.

**`VER-MOK-008` M2 cannot be closed by this work order at all.** It requires the produced archive to be read by
someone who did not build the packaging step, and that person is this work order's author. It closes on a first
run read by someone else, not by further evidence in this directory.

**`candidate-conformance.md`'s "before" column is a reconstruction, not a diff.** No revision of this
repository holds the pre-existing candidate in its pre-conformance form — `17be4ba` adds all 34 files at once —
so every claim about what the candidate satisfied before the work rests on the change record kept while it was
done. Every claim about the *current* state is checkable against this commit.

**One declared file is known to carry a defect that was not fixed.**
`scripts/test_check_release_authorization.py:621` guards on the ambient repository having no tags, which is a
property of whoever's machine runs it. The scenario it discharges needs only that one particular tag is absent.
It is left as it stands, recorded as finding 9 with the one-line change that would fix it, because
`WO-MOK-009` is `implemented` and changing the candidate afterwards is the owners' decision. No CI job runs
these suites, so nothing automated depends on it today; an operator following Phase G does, which is why the
runbook was reordered instead.

Four further limits, stated because they bound what the green gates above mean:

- **Rule 12.6's `environment: release` is a reference, not a protection.** Nothing in the repository configures
  that environment, so the second human gate the rule contemplates does not exist until a release owner
  creates it. Finding 4.
- **`REPOSITORY_CONTEXT.md`'s declared lint command lacks `--locked`** while rule 8.2 requires resolution
  against the committed lockfile. Finding 5.
- **`prepare-release` was never exercised**, because it takes a verified verification record as input and none
  existed at this candidate commit. `release-artifact-types.md` establishes that both the release record and
  the release contract are creatable and names the two nearly identical directory names the harness enforces;
  the command itself remains untried. **Verifying this record removes that blocker without discharging the
  row**: what now stands in the way is the absence of an `approved` `REL-MOK-001`, and creating and approving
  a release contract is the release owner's act rather than something this record authorizes.
- **The machine's own harness install is a moving target**, an editable clone that moved from `0.4.0` to
  `0.4.1` during the work while this repository declares `0.4.0`. Every measurement in this packet comes from a
  pinned `0.4.0` wheel instead. The mismatch is not only a hazard — it is rule 7.1's one piece of *observed*
  evidence, recorded as `compliance-rehearsal.md` C1.

Nothing here bears on the correctness of the simulation. `WO-MOK-009` changes no engine and no observer source
file, and `VER-MOK-001` through `VER-MOK-006` remain the contracts for that.

## The architecture precondition, which was confirmed rather than construed

`WO-MOK-009`'s approval precondition 2 asks the technical owner whether the release process needs an
architecture artifact. It was first discharged by reading the owner's approval of the pack as written — no
`architecture` relation, no `ARCH`, no `ADR` — and that reading was named, in the work order and in
`approval-and-transition.md`, as the one item in the packet a reader should check rather than accept. It was
then put back to the technical owner, who **confirmed it on 2026-08-19**: the instruction was *"architecture: I
confirm"*. So `SPEC-MOK-005` governs the release process alone by decision, and this record's candidate commit
already contains that confirmation.

The original reading is left standing in both files rather than replaced, because the confirmation is an answer
to it. No measurement moved and no provenance field was recomputed on account of it; what changed is the
authority behind one discharge.

## The snapshot field, and what it is reproducible from

`artifact_snapshot_sha256` above is the SHA-256 of a generated `target/harness-dashboard/dashboard-data.json`,
and that JSON records **the name of the directory the repository is checked out into** as the project name. So
the field binds the commit *and* the checkout directory's basename.

To reproduce it: clone this repository so the working directory is named `Mokiterions`, check out
`d35f8172a0f91049aa2719bc34ca9dd7584f4380`, and run `capture-verification` with the arguments this record's
provenance names. That is how this record was prepared, and it is why it was prepared in a throwaway clone
rather than in the linked worktree the work was done in — that worktree is named `Mokiterions-release-ci` and
yields a different hash from the same commit. Measured rather than assumed:

| Checkout directory basename | `artifact_snapshot_sha256` at this commit |
|---|---|
| `Mokiterions`, which is what a plain `git clone` produces | `0eeb2e47b1b4c4c19fa78ae19d388bb5960bb3e1d659e3f07e1ebdb74084ed64` |
| `othername`, a control clone of the same commit | `a20a709faa3ce0655c8716ba78d368e040d8a08b48bee9898f9808574af14258` |

Two consecutive runs in the same clone agree exactly, and the two captures above differ in `verified_at` and
this field and in nothing else — no other provenance field is a property of the checkout. Tags are not in the
hash either: the same clone with and without the stray tag described earlier produced identical values.
`snapshot-reproducibility.md` holds the measurement across five checkouts, the one differing line out of a
~170 KB JSON that identifies the cause, and the hypothesis it refuted: line endings are not in the hash. It is
not a defect against `SPEC-MOK-005`, because no rule and no `VER-MOK-008` scenario concerns this field and
`check_release_authorization.py` never recomputes one.

The snapshot was taken before this record existed, so it records the 79-artifact graph the record binds rather
than the 80-artifact graph containing it.

## What the accountable assurance owner was asked to decide, and decided

Whether the evidence at this candidate commit is sufficient to call `WO-MOK-009` verified **with the process
unrun, fifteen scenario rows not performed, M2 unclosable by this author, the "before" column a
reconstruction, and one declared file carrying a recorded but unfixed defect**. That was the whole of the
decision. The limitations are listed above and as nine findings in `completion-summary.md` rather than
summarized away, because a disclosure that survives only until it is accepted is not a disclosure.

**The accountable assurance owner answered yes on 2026-08-19**, with all five of those limitations stated in
front of them, and directed the transition of this record to `verified`. The question is kept above rather
than deleted, because what was disclosed is part of what was decided: this record is verified *with* the
process unrun, not despite it, and a later reader deciding whether to rely on it needs the terms and not only
the verdict.

The decision is bounded to this record. Verifying it did not merge pull request #20, approve `WO-MOK-008`,
tag anything, create a release record, configure the `release` environment, or perform the first run — and it
did not move `WO-MOK-009`, which stays `implemented`. `docs/engineering/WORKFLOW.md` line 20 states that the
VREC moves separately and that work-order status never substitutes for it, and the repository's own precedent
agrees: `WO-MOK-001` through `WO-MOK-007` are all `implemented` beside `verified` records. Each remaining act
is separate, with its own accountable owner, and `completion-summary.md`'s next-steps table names them with
whose they are.

## Authority

**This record was prepared `ready` and is `verified` by the accountable assurance owner's decision of
2026-08-19.** `docs/engineering/DECISION_RIGHTS.md` states that automation may prepare `ready` verification
and release records from bounded Git observations and that only accountable assurance and release owners may
transition them, and `ENGINEERING_HARNESS.md` states that harness commands may prepare records but never
exercise accountable decision rights. So the implementation agent prepared this record and wrote the `verified`
status the owner directed; it decided nothing. No command performs that transition, and none was asked to.

**The transition changed one field and nothing else.** `commit`, `git_object_format`, `worktree_state`,
`verified_at`, `artifact_snapshot_sha256`, `evidence_paths` and both relations are exactly as
`capture-verification` produced them, and a reader can confirm that against the diff of the transition commit.
A verified record whose provenance was recomputed at transition time would bind the wrong tree; the managed
`capture-verification` command produced every provenance field above, none was hand-edited, and this prose was
appended below the generated body without altering it.

**That is why the two paragraphs under *Verification Record Candidate* above still call this a `ready` record
and still say an assurance owner must transition it.** They are the managed command's own output, left exactly
as it wrote them: the transition edited the `status` field and the prose below it, not the command's text. That
keeps the prefix recomputable, and it was recomputed — `capture-verification` re-run at the candidate commit in
a fresh clone named `Mokiterions`, with the same arguments, emits 25 lines of which **23 are identical to this
file's first 25**, including `commit`, `worktree_state`, `git_object_format`, all eighteen `evidence_paths`,
both relations and `artifact_snapshot_sha256` — the last of which is the field
`snapshot-reproducibility.md` predicted would reproduce, measured again here. Two lines differ, and both differ
for a stated reason:

| Line | The fresh run | This record |
|---|---|---|
| `status` | `"ready"`, which is all the command may write | `"verified"` — the transition itself |
| `verified_at` | the moment of the re-run | `"2026-08-19T14:11:54Z"`, the moment of capture |

`verified_at` is a wall-clock stamp, so no re-run reproduces it by construction, and a record that refreshed it
on transition would bind the tree it was approved in rather than the tree that was verified. A preamble kept in
agreement by hand would be worth less than a prefix that can still be checked this way.
`assurance-decision.md` records the decision the preamble asks for.

That command refused to run while `WO-MOK-009` was `draft`, and the transcript of the refusal is retained in
`commit-binding.md`. It succeeded only after the owner approved the governing chain, which
`approval-and-transition.md` records together with the ten status transitions. That approval is a different
role's act: this record neither supplies, ratifies nor substitutes for it, because an assurance owner's
verification is a statement about evidence rather than about another role's approvals.

The record is committed after the candidate commit it names, as `docs/engineering/WORKFLOW.md` requires of a
record that cannot contain the hash of its own commit. It binds eighteen evidence files — seventeen records
and their directory index.

`WO-MOK-008` is `draft` and untouched by any of this. Pull request #20 is open and unmerged, and this branch
is behind `origin/master`, so the figures above describe this branch rather than the tree that a merge would
produce. The repository holds no release tag, and no release-record or release-contract directory exists in any
revision, which is why `a5-refusal-ladder.md` shows rule 4 refusing at every plausible tag. Verification is
not release, and it is not merge.
