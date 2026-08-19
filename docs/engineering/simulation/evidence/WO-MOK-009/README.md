# Evidence for WO-MOK-009

This directory retains implementation and verification evidence for `WO-MOK-009`, *Implement the release
process against SPEC-MOK-005*, captured on 2026-08-19.

The records are observations of the working tree. They do not independently approve verification, create a
candidate commit, or authorize release. `VER-MOK-008` is the verification contract these records serve; the
accountable assurance decision and any verification record remain the owner's act. `WO-MOK-009` is `draft`,
and nothing in this directory transitions it.

## Commit binding

| Fact | Value |
| --- | --- |
| Governance revision | `54c21abcfb9caa4474c9ca5f194289e055c86a23` (tip of `master`, 2026-08-19 10:49:44 +0200) |
| Implementation branch | `feature/release-ci`, in a linked worktree of the primary clone |
| Candidate commit | **None at capture time.** The records below observed a working tree with no commit of its own. The owner directed the commit afterwards: `17be4bad444a4199da53e72ae8be491ba5f46ee1`, recorded in `commit-binding.md` |
| Tags in the repository | none |
| Harness build used for every measurement | `se-harness==0.4.0`, installed as a wheel into a throwaway environment |
| Harness build present on the machine | an editable install of a live clone, reporting `0.4.1`. **Not used for any measurement here** |
| Toolchain | rustc 1.97.1, cargo 1.97.1 (`8bab26f4f 2026-07-14`), clippy and rustfmt from the same channel |
| Python | 3.14.6 |

Two of these rows are load-bearing and are the reason this table is longer than usual.

**The candidate's earlier state was never committed, and the commit does not change that.** Every claim in
this directory about the *current* state is checkable against the files as they stand, and now against a
fixed revision. Every claim about what the pre-existing candidate did — the whole "before" column of
`candidate-conformance.md` — rests on the change record kept while the work was done, because there is no
revision in which the candidate exists in its pre-conformance form: `17be4ba` adds all 34 files at once. That
file says so in its second section, `commit-binding.md` reconciles it against the commit, and a reader should
weigh it the same way either side of the commit.

**The harness build was pinned deliberately.** The machine's harness moved from `0.4.0` to `0.4.1` partway
through the work while this repository declares `0.4.0`. On the `0.4.1` build, `doctor` emits eight
`FAIL distribution:` lines that read like managed-file damage and are not: `distribution:` checks compare
the repository against the template the *installed* build ships, which is a different question from the
`managed:` checks that compare against the digest in `.engineering-harness.lock`. To keep the measurements
reproducible, every harness command in this evidence set was run from a pinned `0.4.0` wheel. The incident
is not only a hazard — it is the one piece of *observed* evidence for rule 7.1, and it is recorded as such
in `compliance-rehearsal.md` C1.

## Contents

### Read these two first

- `completion-summary.md` — the final affected components; the suite growth from 22 to 70 scenarios and
  what accounts for it; every one of the 18 `VER-MOK-008` rows that is not plainly *observed*, with what
  each needs; seven findings a reader should not have to discover; the four questions the owners settled on
  2026-08-19; and the seven remaining acts with whose they are.
- `candidate-conformance.md` — the rule-by-rule statement `WO-MOK-009`'s Lifecycle section requires: for
  each of rules 1 through 14 and every sub-rule, what the pre-existing candidate satisfied and what had to
  change. Twelve changes, ordered by how badly the unchanged form would have failed; two divergences
  deliberately left in place; one change outside the declared change surface.

### The scenario contract, reconciled

- `scenario-map.md` — `VER-MOK-008`'s scenarios against what was actually done. 65 rows: 47 observed, 3
  rehearsed, 15 not performed, 0 unexercisable. Defines *observed*, *rehearsed* and *not performed* before
  using them, cites all 70 tests by name and line, and reconciles the 30 tests the contract does not
  enumerate against the rule each exists for.
- `suite-output.md` — the run: 70 passed, 0 failed. Includes why every refusal test asserts the refusal
  *message* and not only the exit status.

### Authorization behavior

- `a2-transcript.md` — the A2 fixture in full: the gate authorizing a release whose record is provably
  absent from the tagged tree. This is the scenario the candidate's headline defect would have failed, and
  the reason `WO-MOK-009` names it as the worked example.
- `a5-refusal-ladder.md` — the gate run against this repository at every plausible tag, showing which rung
  of rule 4 refuses and why. Rule 4.10 is the rung that refuses today.
- `p4-worktree-comparison.md` — `git status --porcelain --untracked-files=all`, `git tag --list` and
  `git branch --list` before and after, three times, including against a real checkout: the gate writes
  nothing.

### Compliance and the declared checks

- `compliance-rehearsal.md` — C1 through C5. The real `0.4.0`/`0.4.1` mismatch that rule 7.1 caught; the
  review-preflight loop at the governance revision, with the note that `WO-MOK-001`'s pass depends on an
  uncommitted edit; the structural argument for C4 labelled as an argument; and C5 stated as a finding.
- `verification-output.md` — the declared checks and the harness commands, run and retained in full:
  `cargo fmt --check`, `clippy` with warnings as errors, the workspace test run, the dependency-tree
  assertion, `doctor`, and the artifact validator.
- `determinism-rehearsal.md` — the two-run comparison for rule 8.5: byte-identical output, identical final
  state, identical exit code, and `git status` unchanged across both.

### The commit these records describe

- `commit-binding.md` — the commit the owner directed after the captures were taken, the gates re-run on the
  committed tree, and the reconciliation of the three statements above that said there was no candidate
  commit. Also records why no verification record accompanies it.

### The process definition, read statically

- `static-checks.md` — S1 through S11, the eleven static claims, each derived from the frozen 616-line
  definition with the command that produced it and its verbatim output in an appendix. Every numeric
  citation in the prose resolves to the line it names.
- `toolchain-evidence.md` — T1 through T5 for rule 9: a clone selecting the declared version, a matching
  version passing, a newer and an older version refusing, and the declared components present.

## What is not here, and why

- **No run of the process.** There is no tag and no release record, and the candidate commit is on an
  unmerged branch, so the workflow has never executed. This is why V1–V6, P1–P3 and C4 are *not performed*
  rather than failing: they are observations of a run, and committing is not a run.
  `completion-summary.md` lists them individually with what each needs.
- **No read of a produced archive, and by design it could not be here.** `VER-MOK-008` M2 requires the
  archive to be read by someone who did not build the packaging step. That is this work order's author. So
  V1 through V6 do not close by further work in this directory; they close on the first run, read by
  someone else.
- **No status transition, and no prepared verification record.** `DECISION_RIGHTS.md:14` reserves
  transitions to `verified` and `released` to accountable owners. Ten new artifacts are `draft` and stay
  that way. The harness enforces the consequence rather than leaving it to good intentions:
  `capture-verification` refuses while `WO-MOK-009` is `draft`, with the transcript in
  `commit-binding.md`. `a5-refusal-ladder.md` records that the real graph holds no release record, which is
  a fact about the repository rather than something to fix here.
- **No copy of the process definition or the gate.** They are tracked files at
  `.github/workflows/release.yml`, `scripts/check_release_authorization.py` and
  `scripts/check_release_reachability.py`. `static-checks.md` cites them by line against a definition
  frozen before the citations were generated; a second copy of the same bytes would only be a second thing
  to keep in step.
- **No measurement from the `0.4.1` harness except the one that is the point.** The mismatch appears in
  `compliance-rehearsal.md` C1 as rule 7.1's observed evidence, and nowhere else.
- **No edit to `.github/workflows/engineering-harness.yml`, and no invocation of it.** Rule 13.8 forbids
  both. `static-checks.md` S8 records that it still matches the digest in `.engineering-harness.lock`, and
  that the compliance job re-runs the checks rather than calling it — which it could not do anyway, since
  that workflow declares no `workflow_call` trigger.
