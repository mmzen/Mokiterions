# Completion summary

`WO-MOK-009`, Evidence to record: *"a completion summary identifying the final affected components and
every `VER-MOK-008` scenario that remains rehearsed rather than observed."*

Captured 2026-08-19, worktree of the primary clone, branch `feature/release-ci`, HEAD
`54c21abcfb9caa4474c9ca5f194289e055c86a23`, harness `0.4.0` from a pinned wheel.

## Status of this work order

`WO-MOK-009` was `draft` throughout the work this record summarizes, and its own stop condition said to
halt while that was true and while review preflight did not pass. So read the measurements below as
prepared work measured against `SPEC-MOK-005` as the contract, captured before any approval existed.

**The owner has since approved the chain and authorized the transition, on 2026-08-19.** Nine artifacts are
`approved` and `WO-MOK-009` is `implemented`; review preflight now passes. The implementation agent recorded
the transitions and made none of the decisions — `docs/engineering/DECISION_RIGHTS.md:14` reserves them to
accountable owners — and `approval-and-transition.md` states what was transitioned, what was deliberately
left `draft`, and the one reading of the approval — which was put back to the technical owner and confirmed
on 2026-08-19 rather than left standing as a construal. Nothing in this record's measurements changed,
because a status transition edits a field.

## Final affected components

### Process definition and the checks it invokes — new

| Path | Lines | What it is |
| --- | --- | --- |
| `.github/workflows/release.yml` | 616 | the release process: 5 jobs, 23 named `run:` steps, 9 marketplace actions |
| `scripts/check_release_authorization.py` | 411 | the rule 3 / rule 4 authorization gate, runnable by hand |
| `scripts/check_release_reachability.py` | 181 | the rule 5 reachability check, separate so it runs in a clone with no remote |
| `scripts/test_check_release_authorization.py` | 721 | 48 scenarios |
| `scripts/test_check_release_reachability.py` | 265 | 22 scenarios |
| `rust-toolchain.toml` | 37 | the rule 9 compiler declaration, `channel = "1.97.1"` with `rustfmt` and `clippy` |
| `docs/RELEASE_RUNBOOK.md` | 422 | the rule 14 human sequence, outside the governed artifact root |

### Governed artifacts — new

`INT-MOK-007`, `CAP-MOK-007`, `REQ-MOK-035` through `REQ-MOK-039`, `SPEC-MOK-005`, `VER-MOK-008`,
`WO-MOK-008`, `WO-MOK-009`. Ten artifacts, all `draft` when they were written; nine of the ten now carry the
owner's 2026-08-19 approval, and `WO-MOK-008` alone is still `draft`.

### Governed artifacts — modified

| Path | What changed | In scope? |
| --- | --- | --- |
| `docs/engineering/REPOSITORY_CONTEXT.md` | setup line rewritten for the toolchain pin and `python -m se_harness`; a `- Release:` bullet added | yes — `WO-MOK-009` names it |
| `docs/engineering/simulation/work-orders/WO-MOK-001.md` | an `[assurance]` table, `decided_by`, an `updated` bump, a Lifecycle note | **no — see below** |

### Evidence — this work order

Sixteen records and a directory index, listed in `README.md`. Four were written after the captures, as the
owner directed each act: `commit-binding.md`, which names the commit; `release-artifact-types.md`, which
discharges half of this work order's fourth approval precondition; `approval-and-transition.md`, which
records the approvals and the ten status transitions; and `snapshot-reproducibility.md`, which records what
`VREC-MOK-007`'s snapshot field is reproducible from, because the first attempt to recompute it from the same
commit did not match.

### Not this work order

`docs/engineering/simulation/evidence/WO-MOK-008/footer-tier-fallthrough.md` (124 lines) belongs to
`WO-MOK-008`, the defect report. It is in the same changeset and is named here only so a reader of the
diff is not surprised by it.

### Untouched

No Rust source, no `Cargo.toml`, no `Cargo.lock`, no managed harness file. `static-checks.md` S8 records
that `.github/workflows/engineering-harness.yml` still matches the hash in `.engineering-harness.lock`
(`0b850207fb04c054f3c936bcbf2e2a18c1d9f79de14232b25fa328a0159c5ed4`), and S9 that
`cargo tree -p Mokiterions --locked` is unchanged.

## The scenario suite grew from 22 to 70

`WO-MOK-009`'s Expected change surface records that *"the candidate currently carries 22 scenarios"*.
`VER-MOK-008` enumerates 29 fixture-drivable scenarios (A1–A5, R1–R24). The suite now holds **70**: 48 in
the authorization file, 22 in the reachability file.

Counted from the test names themselves, so a reader can re-derive it:

| Group | Count |
| --- | --- |
| one test per enumerated row — all 29 of A1–A5 and R1–R24 are covered, none missing | 29 |
| further tests named for an enumerated row | 7 |
| P4 cases (`test_p4…`) | 4 |
| property cases (`test_property…`) | 7 |
| cases naming no enumerated row | 23 |
| **total** | **70** |

The seven that share an enumerated row's name are A3's converse (a record stating a *different* tag is not
silently matched), A4's machine-readable channel as distinct from its human summary, four further rungs of
A5's ladder, and R23's local-`release/*` variant — because rule 5 requires the branch to be *present on the
remote*, and a local one in the runner's clone must not satisfy it.

The 23 that name no row are anchored one by one in `scenario-map.md` to the rule or the Error-and-recovery
clause each exists for. That anchoring is not decoration: `VER-MOK-008` says *"one that refuses more is not
thereby conforming, because an unlisted refusal is an unspecified behavior"*, so a test asserting a refusal
the specification does not ask for would be a defect in the suite. Nine of the eighteen reachability extras
are positive or reporting cases rather than refusals.

Seven of the 23 — every unenumerated case in the gate suite, the other sixteen being in the reachability
suite — exist because writing them found a defect rather than because the enumeration asked. Two are the
front-matter parse cases that closed rule 4.2's fail-open path; three are the artifact-root cases that
closed the hard-coded root; and two are tolerances rather than refusals, which keep rule 4.2 from refusing
this repository for having a README in an evidence directory. `candidate-conformance.md` records what each
one found.

`scenario-map.md` cuts the same 70 differently, and the two cuts reconcile exactly rather than disagreeing.
Its two *Extra cases* tables hold 30 tests: the 23 in the row above that name no enumerated row and are not
property cases, plus 5 that go beyond one test per enumerated row and are anchored individually anyway (A3's
converse, A4's machine channel, two further A5 rungs, R23's local branch), plus the 2 property cases that
needed an anchor of their own. The other 5 property cases are credited in that file's *Property and
invariant tests* table instead, which is why the extras tables are not simply "the tests naming no row".
Between all of its tables it cites every one of the 70 by name and line, with no test uncited and no citation
resolving to nothing. `suite-output.md` holds the run: 70 passed, 0 failed.

## Every scenario that remains rehearsed or not performed

`scenario-map.md` defines the three words and holds the full table: **65 rows — 47 observed, 3 rehearsed,
15 not performed, 0 unexercisable**, with C3, M5 and V5 counted as split rows. What follows is every row and
check that is not plainly **observed** — the 18 non-observed scenario rows, plus the split halves and the two
performance checks that fall short of observation — so that a reader assessing residual risk does not have to
filter the table.

### Rehearsed — exercised locally, but not by the process on a runner

| Row | Why | Evidence |
| --- | --- | --- |
| C2 | the loop and both predicates were run by hand at the governance revision; the workflow did not run them | `compliance-rehearsal.md` C2 |
| C3 (half) | the checks all ran and passed; *"at the authorized commit"* rests on a definition read plus the run-time `git rev-parse HEAD` assertion | `verification-output.md`, `static-checks.md` S11 |
| T3, T4 | the refusal in both directions was driven by editing the declaration, not by a runner with a different toolchain | `toolchain-evidence.md` T3, T4 |
| the refusal-cost performance claim | read from the job graph, not measured | `scenario-map.md`, performance table |

### Not performed — requires a run, a produced archive, or an owner

| Row | What it needs |
| --- | --- |
| C4 — a failing check publishes nothing | a run whose `verify` job fails. `compliance-rehearsal.md` C4 gives the structural argument (`publish` needs all four jobs) and labels it as an argument |
| V1 — every required provenance line present and correct | a produced archive |
| V2 — the commit in the statement is complete and matches | a produced archive |
| V3 — a checksum verifies | a produced archive and a standard tool |
| V4 — the notes agree with the archive | a produced archive and a release object |
| V5 (half) — no credential, no local path in the statement | a produced archive. The static half, S5, is observed |
| V6 — two builds differ only by run identity | two runs |
| P1 — a refused run leaves no trace | a run. `static-checks.md` S1 covers the definition-level half |
| P2 — an authorized run leaves the release invisible | a run and a release object |
| P3 — no status changes across a successful run | a run |
| M1 — the human sequence is adequate | the assurance owner reads `docs/RELEASE_RUNBOOK.md` |
| M2 — the archive, read by someone who did not build it | by construction, someone other than this work order |
| M3 — the refusal set is complete | the owner compares the R table against rules 4 and 5. `scenario-map.md` supplies the material |
| M4 — the release contract's judgements | there is no release contract yet; it is a precondition of a release, not of this work order |
| M5 (half) — the two revisions were not conflated | a log read. The mechanism is observed: A2 for the gate, S11 for the process |

**V1 through V6 are the largest single gap, and it is structural rather than an omission.** M2 requires the
archive to be read by someone who did not build the packaging step. That is this work order's author. So
no amount of further local work closes V1–V6; they close on the first run, read by someone else. This is
stated in `scenario-map.md` and repeated here because a reader counting "15 not performed" should know that
six of them are not-yet rather than not-done, and that one of them cannot be done here at all.

### The one row that was unexercisable, and is now closed

**C5 — warnings do not refuse.** The substance always held and was always demonstrated. The method *as
originally written* was *"asserting the validation step's success against its own reported warning count
being non-zero"*, and the rule 7.3 validation step reports `Artifacts: 79 | Errors: 0 | Warnings: 0`, so
that assertion could not be made against a conformant implementation. The nine warnings the row reached for
live in `se_harness inspect`, which rule 7 does not require and the process does not run.

That was a gap in `VER-MOK-008` rather than in the implementation — the standing M3 gives the contract
itself — and it was raised for the assurance owner rather than worked around. **The assurance owner amended
the row on 2026-08-19**, dropping the non-zero clause: C5 now asks for the step's success while asserting
nothing whatever about its warning count, because the *absence* of a predicate on the count is what
establishes the count is unread.

Under the amended wording C5 is **observed**, and the witness is the transcript already in
`compliance-rehearsal.md` C5. The scenario totals above move accordingly: 47 observed, 3 rehearsed, 15 not
performed, 0 unexercisable.

## Seven findings a reader should not have to discover

1. **`WO-MOK-001.md` was modified, outside this work order's declared change surface.** It gained the
   `[assurance]` table the harness's review-phase preflight now requires. Without it,
   `preflight --work-order WO-MOK-001 --phase review` fails with
   `[W023] … assurance classification is missing`, and rule 7.4 refuses, so a first release is impossible.
   The gap is pre-existing — the declaration did not exist when `WO-MOK-001` was approved on 2026-08-11,
   and `WO-MOK-002` on 2026-08-17 is the first to carry one — and this work order's rule 7.4 rehearsal is
   what surfaced it. No status was transitioned. **The engineering owner affirmed the classification and its
   rationale on 2026-08-19 and chose to keep the edit, letting the scope deviation stand as this stated
   finding** rather than widen `WO-MOK-009`'s change surface or open a separate governance work order.
   `WO-MOK-001`'s Lifecycle section records the affirmation at the artifact itself;
   `candidate-conformance.md`'s closing section holds the full evidence and the paths not taken; and
   `compliance-rehearsal.md` C2 records that its half-three capture depends on this edit.

2. **The candidate's pre-conformance state was never committed, so `candidate-conformance.md` is a
   reconstruction rather than a diff.** The owner directed a commit after these records were captured —
   `17be4ba`, named in `commit-binding.md` — and it does not change this: all 34 files land at once, so no
   revision holds the candidate in its earlier form. Every *current* claim is checkable against the files;
   only the "before" column rests on the change record kept during the work. Said plainly there and repeated
   here because it bounds what that file can be relied on for.

3. **Rule 12.5 was unsatisfiable as written, and the rule changed rather than the implementation.** GitHub
   Actions scopes `permissions` per job, not per step, so the original *"the attaching step is the only step
   granted write access"* had no expressible form. It was raised as a specification question, and the
   technical owner amended 12.5 on 2026-08-19 to confine write access to the smallest job containing the
   attaching step, with the credential reaching that step alone — which is what the definition already did.
   This is the only rule in the conformance statement whose "satisfied" comes from amending the rule, and it
   is flagged as such there. `candidate-conformance.md`, rule 12; `SPEC-MOK-005`'s amendment record carries
   the entry.

4. **Rule 12.6's `environment: release` is a reference, not a protection.** Nothing in the repository
   configures that environment. The rule asks only that the gate be enablable by configuration alone,
   which is satisfied; but "12.6 satisfied" must not be read as "an approval gate exists".

5. **`REPOSITORY_CONTEXT.md`'s declared lint command lacks `--locked`** while rule 8.2 requires resolution
   from the committed lockfile. The workflow adds it, which is strictly stricter. Left in place because
   editing that command is beyond what `WO-MOK-009` names for the file; it is a one-line change for
   whoever next opens it.

6. **The local harness install is a moving target.** It is an editable install of a live clone that moved
   from `0.4.0` to `0.4.1` mid-session while this repository declares `0.4.0`. On the mismatched build,
   `doctor` emits eight `FAIL distribution:` lines that read like managed-file damage and are not — they
   compare the repository against the template the *installed* build ships, distinct from the `managed:`
   checks that compare against `.engineering-harness.lock`. All harness measurements in this evidence set
   were taken with a pinned `se-harness==0.4.0` wheel in a throwaway environment. CI is unaffected: the
   workflow installs the declared version. The incident is also the one piece of *observed* evidence for
   rule 7.1, recorded in `compliance-rehearsal.md` C1.

7. **The runbook cuts the maintenance branch earlier than rule 14 lists it, and that reading was accepted.**
   Rule 14's list puts the branch after the release record; `docs/RELEASE_RUNBOOK.md` cuts it in Phase C,
   before the verification record, the contract and the record. That follows `REPOSITORY_CONTEXT.md`, which
   cuts `release/<major>.<minor>` *"when a release enters stabilization"* — before publication — and it
   matters because stabilization commits move the candidate commit to the tip of `release/0.1`, which the
   runbook states and rule 14's ordering would obscure. **The technical owner accepted on 2026-08-19 that
   rule 14 requires coverage of the seven acts and an order, not that order**, so neither the runbook nor the
   rule was changed. `candidate-conformance.md`, rule 14.

## Harness state at the end of the work

Re-run with the pinned `0.4.0` wheel after the last evidence file was written, again after the first four
decisions below were written into `SPEC-MOK-005`, `VER-MOK-008` and `WO-MOK-001`, and again after the ten
status transitions, so that the numbers below describe the tree as it stands rather than as it stood
mid-work. Every capture agrees.

| Command | Result |
| --- | --- |
| `python -m se_harness doctor .` | exit 0 — 81 verdict lines, PASS 81, WARN 0, FAIL 0 |
| `python scripts/validate_engineering_artifacts.py --root .` | exit 0 — 79 artifacts, 0 errors, 0 warnings, all four planes clean |
| `python -m se_harness preflight . --work-order WO-MOK-009 --phase review` | **exit 1 while the chain was `draft`**, which was the reason this work order's own stop condition was unmet; **exit 0, PASS, after the transitions** |

The artifact count is unchanged at 79 across the five evidence files added at the end, which is the
measured form of the claim that the validator does not walk `evidence` directories, and unchanged again
across the ten transitions, because a transition edits a field. What the transitions do change is what the
silence is worth: `E007` and `E008` exempt draft requirements and now check `REQ-MOK-035`–`039` against
`SPEC-MOK-005`'s `specifies` and `VER-MOK-008`'s `verifies`. Transcripts in `verification-output.md` and
`approval-and-transition.md`.

## Settled on 2026-08-19

Five questions this work order raised were put to the owners and answered the same day. Each is recorded at
the artifact it changed, or at the note it settles, as well as here.

| Question | Decision | What changed |
| --- | --- | --- |
| `WO-MOK-001`'s missing assurance classification, and the scope deviation it created | keep the edit, affirm the table | `WO-MOK-001.md` Lifecycle section records the affirmation; the deviation stands as a stated finding |
| `VER-MOK-008` C5's unexercisable method | drop the non-zero clause | `VER-MOK-008` C5 amended; the row is now **observed** |
| Rule 12.5's step-level grant, which the platform cannot express | amend the rule to say *job*, and record why | `SPEC-MOK-005` rule 12.5 amended, with an amendment-record entry; the workflow comment at the grant reworded |
| Rule 14's ordering versus the runbook's Phase C | accept the runbook's order | nothing changed; the reading is recorded |
| Approval precondition 2's architecture question, first discharged by reading the approval rather than by a worded decision | confirm it — no architecture artifact is required | no artifact changed; `WO-MOK-009`'s *How each precondition was discharged* and `approval-and-transition.md` now record a decision where they recorded a construal, and both keep the reading it answers |

## What has to happen next, and by whom

| # | Act | Whose | State on 2026-08-19 |
| --- | --- | --- | --- |
| 1 | decide the architecture question `WO-MOK-009` names as an approval precondition | technical owner | **done** — first taken by approving the pack with no architecture artifact in it, then **confirmed on 2026-08-19** when that reading was put back to them; no architecture artifact is required |
| 2 | approve `INT-MOK-007`, `CAP-MOK-007`, `REQ-MOK-035`–`039` | as `DECISION_RIGHTS.md` assigns | **done** |
| 3 | approve `SPEC-MOK-005`, including the amended rule 12.5 | technical owner | **done** |
| 4 | approve `VER-MOK-008`, with the amended C5 | assurance owner | **done** |
| 5 | approve `WO-MOK-009` and transition it | engineering owner | **done** — `implemented` |
| 5b | approve `WO-MOK-008` | engineering owner | outstanding — deliberately not part of this approval |
| 6 | run the process once and read V1–V6 and P1–P3 | release owner, plus a reader who is not this work order's author | outstanding |
| 7 | configure the `release` environment if rule 12.6's second human gate is wanted | release owner | outstanding |

Steps 6 and 7 are reserved acts. Nothing in this changeset performs them, and nothing here should be read
as authorizing them.

**Six acts have been performed on the owner's instruction since the captures, and none is step 6 or 7.**
The changeset was committed as `17be4ba` and opened as pull request #20, both recorded in
`commit-binding.md`. The owner then approved the chain and authorized the work order's transition, recorded
in `approval-and-transition.md` and in `WO-MOK-009`'s own *Approval record*, which lets the fourth act
happen: `capture-verification` had refused while the work order was `draft`, and `VREC-MOK-007` is now
prepared and **`ready`**. Fifth, the technical owner confirmed the architecture reading that the fourth
act's records had flagged, which closes row 1 above by decision rather than by construal. Sixth, the owner
authorized carrying the unpushed governance commits to `origin`, which updates pull request #20 and turns
the review-preflight failure recorded in `approval-and-transition.md` into a pass.

Making `VREC-MOK-007` `verified` is the accountable assurance owner's act and was not taken. Merging pull
request #20 was not instructed and was not done — a push is not a merge, and it authorizes nothing that
rule 13 reserves.
