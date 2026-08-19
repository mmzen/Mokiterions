# Scenario-to-contract map — every `VER-MOK-008` row

`VER-MOK-008`'s Evidence retention section requires "the scenario-to-contract mapping for every
`VER-MOK-008` scenario". This is that mapping. Captured 2026-08-19 in a linked worktree of the primary
clone, branch `feature/release-ci`, HEAD `54c21abcfb9caa4474c9ca5f194289e055c86a23`, with this work
order's changes present but uncommitted.

Every harness measurement referenced here was taken with the pinned wheel `se-harness==0.4.0`, the
version `.engineering-harness.toml` declares and the version the workflow installs. See
`compliance-rehearsal.md` C1 for why that qualification is load-bearing.

## How to read the status column

Three values, and the distinction between them is the point of this file rather than a formality.

- **observed** — the thing the row describes was made to happen and its result was recorded. For a
  scenario suite row this means the named test ran and passed; for a static row it means the search was
  run and its output is retained.
- **rehearsed** — the row's *mechanism* was exercised against a fixture or by hand, but not on the path
  the contract describes. Every compliance row that a real release run would produce falls here, because
  the repository has never released.
- **not performed** — nothing was measured. Either the row requires a reserved act this work order does
  not perform, or it requires a person who is not the implementer.

`VER-MOK-008`'s own Residual uncertainty already states the general form of this: *"Until one has, C, P
and V evidence is rehearsed rather than observed, and the difference is real."* This file makes that
statement row by row rather than in aggregate.

The enumeration holds 65 scenario rows — A1–A5, R1–R24, C1–C5, V1–V6, P1–P4, T1–T5, S1–S11, M1–M5.
Across them: **47 observed, 3 rehearsed, 15 not performed, and 0 unexercisable.**

C5 was the sixty-fifth and it was unexercisable as originally written. The assurance owner amended the row
on 2026-08-19, on the evidence in `compliance-rehearsal.md` C5, and under the amended wording it is
observed. Its own section below keeps the original finding, because that finding is what the amendment
rests on and deleting it would leave the row looking as though it had always been satisfiable.

Three rows are split, and each says so where it appears rather than only here: C3's checks are observed
while its *"at the authorized commit"* clause is rehearsed; M5's mechanism is observed while the log read
it asks for is not performed; and V5's static half is observed while its archive read is not. Each is
counted under the status of the half the row is chiefly about.

## Authorizing scenarios

| Row | Test | Status | Note |
| --- | --- | --- | --- |
| A1 — a correct graph authorizes | `test_a1_a_correct_graph_authorizes` (`:226`) | **observed** | |
| A2 — the record lives in a later commit | `test_a2_the_record_lives_in_a_later_commit_than_the_one_it_names` (`:230`) | **observed** | `a2-transcript.md` holds the transcript, including the assertion that the record's path is absent from the tagged tree |
| A3 — a record that states no tag | `test_a3_a_record_that_states_no_tag` (`:254`) | **observed** | |
| A4 — the authorized facts are reported | `test_a4_the_authorized_facts_are_reported` (`:264`) | **observed** | the human-readable summary |
| A5 — this repository refuses, today | `test_a5_the_real_repository_refuses_every_tag_today` (`:619`), `test_a5_the_real_graph_holds_no_release_record` (`:624`), `test_a5_the_refusal_ladder` (`:628`) | **observed** | against the real graph, not a fixture; `a5-refusal-ladder.md` holds the six-rung ladder |

A5 is the contract's second independence measure — "one oracle is independent of both the gate and its
tests: this repository, today" — and it is the row this work order can discharge most strongly, because
the oracle is not constructed. The ladder records which refusal remained after each governance artifact
was added, which `VER-MOK-008` calls out separately in Evidence retention.

## Refusal scenarios

Each row breaks exactly one fact in A1's otherwise-authorizing graph. Every one asserts a failing exit
status **and** a message containing the named fact, which is what the contract requires: *"Asserting only
the status would pass a gate that refuses everything for one reason."*

| Row | Rule | Test | Status |
| --- | --- | --- | --- |
| R1 | 4.1 | `test_r1_the_requested_tag_does_not_exist` (`:288`) | **observed** |
| R2 | 4.1 | `test_r2_the_requested_name_is_a_branch_not_a_tag` (`:292`) | **observed** |
| R3 | 3 | `test_r3_the_tag_is_lightweight_rather_than_annotated` (`:300`) | **observed** |
| R4 | 4.2 | `test_r4_two_artifacts_share_an_identifier` (`:305`) | **observed** |
| R5 | 4.3 | `test_r5_the_repository_does_not_require_full_commits` (`:313`) | **observed** |
| R6 | 4.4 | `test_r6_no_release_record_exists_at_all` (`:319`) | **observed** |
| R7 | 4.4 | `test_r7_the_release_record_is_ready_not_released` (`:324`) | **observed** |
| R8 | 4.4 | `test_r8_two_released_records_claim_the_same_tag` (`:329`) | **observed** |
| R9 | 4.5 | `test_r9_the_records_commit_is_abbreviated` (`:338`) | **observed** |
| R10 | 4.6 | `test_r10_the_records_commit_is_an_unrelated_commit` (`:342`) | **observed** |
| R11 | 4.6 | `test_r11_the_tag_was_force_moved_after_the_record_was_written` (`:346`) | **observed** |
| R12 | 4.7 | `test_r12_the_gating_contract_does_not_exist` (`:354`) | **observed** |
| R13 | 4.7 | `test_r13_the_gating_contracts_declared_type_is_not_a_release_contract` (`:359`) | **observed** |
| R14 | 4.7 | `test_r14_the_gating_contract_is_draft` (`:363`) | **observed** |
| R15 | 4.8 | `test_r15_the_contract_does_not_gate_a_released_work_order` (`:367`) | **observed** |
| R16 | 4.9 | `test_r16_a_released_work_order_is_approved_not_releasable` (`:371`) | **observed** |
| R17 | 4.10 | `test_r17_an_included_verification_record_does_not_exist` (`:375`) | **observed** |
| R18 | 4.10 | `test_r18_an_included_verification_record_is_ready` (`:380`) | **observed** |
| R19 | 4.10 | `test_r19_an_included_verification_record_names_another_commit` (`:384`) | **observed** |
| R20 | 4.11 | `test_r20_work_is_released_but_not_verified` (`:388`) | **observed** |
| R21 | 4.11 | `test_r21_work_is_verified_but_not_released` (`:392`) | **observed** |
| R22 | 4.12 | `test_r22_the_records_version_is_not_usable_as_a_name` (`:396`) | **observed** |
| R23 | 5 | `test_r23_the_authorized_commit_is_reachable_only_from_a_feature_branch` (reachability, `:171`) | **observed** |
| R24 | 5 | `test_r24_the_authorized_commit_is_reachable_from_no_remote_branch` (reachability, `:179`) | **observed** |

R23 and R24 live in the reachability suite rather than the gate suite, which the contract explicitly
permits: *"may be exercised against the process rather than the gate, since rule 5 places them outside
the gate so the gate stays runnable in a clone with no remote."* Both refuse. Which component carries
them is recorded here so that a reader looking for them in the gate suite does not conclude they are
missing.

R1 through R22 are in `scripts/test_check_release_authorization.py`; R23 and R24 in
`scripts/test_check_release_reachability.py`. Line references are to those files.

## Compliance scenarios

| Row | Status | Evidence |
| --- | --- | --- |
| C1 — version equality precedes every harness command | **observed** on a real mismatch, and **rehearsed** in both directions plus the unreadable case | `compliance-rehearsal.md` C1 |
| C2 — the graph is checked at the governance revision | **rehearsed**; all three halves pass | `compliance-rehearsal.md` C2 |
| C3 — the declared checks pass at the authorized commit | **observed** for the checks; **rehearsed** for "at the authorized commit" | `verification-output.md`, `determinism-rehearsal.md`, `compliance-rehearsal.md` C3 |
| C4 — a failing check publishes nothing | **not performed** | `compliance-rehearsal.md` C4, with the structural argument stated as such |
| C5 — warnings do not refuse | **observed** under the row as amended 2026-08-19; unexercisable as originally written | `compliance-rehearsal.md` C5 |

C1 is the one compliance row this work order can call observed, and it was not planned that way. Partway
through the work the machine's harness moved from `0.4.0` to `0.4.1` while this repository went on
declaring `0.4.0`, and the rule 7.1 predicate caught it with no fixture at all. That incident is the
better evidence and it is recorded in full.

C3's split needs stating precisely. The formatting, lint, test, dependency-tree and determinism checks
all ran and all passed, so the *checks* are observed. What is rehearsed is the clause *"at the authorized
commit"*: locally the checks ran against a working tree, not against a checkout the workflow pinned. What
stands behind that clause is a definition read plus a run-time assertion — `verify` checks out
`needs.authorize.outputs.commit` and its second step compares `git rev-parse HEAD` against it — recorded
in `static-checks.md` S11.

### C5 — the finding, and the amendment it produced

C5's substance always held and was always demonstrated. Its *method*, as originally written, named an
instrument that reports zero warnings, so the assertion it asked for could not be made against a
conformant implementation:

> This is checked by asserting the validation step's success against its own reported warning count being
> non-zero.

The rule 7.3 validation step is `python scripts/validate_engineering_artifacts.py --root .`, and it
reports `Artifacts: 79 | Errors: 0 | Warnings: 0` across all four planes. The nine warnings the row was
reaching for live in `se_harness inspect`, which rule 7 does not require the process to run and which the
process does not run. That was **a gap in `VER-MOK-008`, not in the implementation** — the standing the
contract gives itself in M3 — and it was raised for the assurance owner rather than worked around.

**The assurance owner amended the row on 2026-08-19**, taking the first of the three dispositions
`compliance-rehearsal.md` C5 sets out: drop the non-zero clause. C5 now asks for the step's success while
asserting nothing about its warning count, on the reasoning that the *absence* of a predicate on the count
is what establishes the count is unread — which is the property rule 7.3 cares about. Under that wording
the row is **observed**, and the witness is the transcript already in `compliance-rehearsal.md` C5.

Two things worth keeping in view. The row is now satisfiable by a repository whose validator reports zero
warnings *and* by one that reports many, which is the right breadth for a rule about warnings being
ignored. And the reason the original clause was tempting — `inspect` does report warnings — is also a
reason to keep counts out of contract rows: `inspect`'s total is a function of the harness build, `9`
under `0.4.0` and `3` under `0.4.1` on this same commit.

## Provenance scenarios

| Row | Status | Why |
| --- | --- | --- |
| V1 — every required line is present and correct | **not performed** | |
| V2 — the commit is complete and matches | **not performed** | |
| V3 — a checksum verifies | **not performed** | |
| V4 — the notes agree with the archive | **not performed** | |
| V5 — no credential and no local path | **not performed** as a read of a produced archive; the static half (S5) is **observed** | |
| V6 — two builds differ only by run identity | **not performed** | |

All six require a produced archive, which requires a run, which requires a tag and a push — both reserved
to the release owner. Beyond that, M2 requires that V1 through V6 be performed *by someone other than the
implementer*, so this work order structurally cannot discharge them even if a run existed. That is the
contract's principal independence measure for `REQ-MOK-037` and stating it here is the correct outcome,
not a shortfall to be argued around.

The one half that is discharged is S5's: the provenance statement's *construction* was read line by line
and reads only the authorized facts, the build matrix, the compiler and the platform run identity. See
`static-checks.md` S5, which also records the `$RUNNER_OS` read that does **not** reach the statement, and
the rule 10.7 guard on `MOKITERIONS_COMMIT`.

## Persistence and reserved-act scenarios

| Row | Status | Evidence |
| --- | --- | --- |
| P1 — a refused run leaves no trace | **not performed** | requires a run; `static-checks.md` S1 records the definition-level half |
| P2 — an authorized run leaves the release invisible | **not performed** | requires a run and a release object |
| P3 — no status changes across a successful run | **not performed** | requires a run |
| P4 — the gate writes nothing | **observed**, three times | `p4-worktree-comparison.md` |

P4 is covered by `test_p4_the_gate_writes_nothing_when_it_authorizes` (`:402`),
`test_p4_the_gate_writes_nothing_when_it_refuses` (`:412`) and
`test_p4_the_gate_leaves_a_real_checkout_unchanged` (`:702`), plus `test_p4_the_check_writes_nothing`
(reachability, `:238`). The third of those runs against a real checkout rather than a fixture, which is
the case the contract cares about; `p4-worktree-comparison.md` records the before-and-after comparison
including untracked files.

P1 through P3 are the rows `VER-MOK-008` marks "on a run". Their definition-level halves — no
`continue-on-error`, no job `if:`, no write outside `publish`, no status field touched anywhere — are in
`static-checks.md` S1, S2, S3 and S11, and the contract already says what that is worth: *"S1 through S4
establish that the process definition contains no reserved act. They do not establish that no tool the
process invokes performs one on its behalf."*

## Toolchain scenarios

| Row | Status | Evidence |
| --- | --- | --- |
| T1 — a clone selects the declared version | **observed**, with one caveat | `toolchain-evidence.md` T1 |
| T2 — a matching version passes | **observed** | `toolchain-evidence.md` T2 |
| T3 — a newer version refuses | **rehearsed** | `toolchain-evidence.md` T3 |
| T4 — an older version refuses | **rehearsed** | `toolchain-evidence.md` T4 |
| T5 — the declared components are present | **observed** | `toolchain-evidence.md` T5 |

T1's caveat: the fresh clone was taken from the remote, where `rust-toolchain.toml` does not yet exist,
so the file was placed into the pristine clone by hand before querying the toolchain. The query itself
then reported the declared version and named the repository's declaration as the reason, which is what T1
asks for; but the clone was not pristine at the moment of measurement, and `toolchain-evidence.md` says
so at the point of measurement rather than here.

T3 and T4 are rehearsed by fabricating a declaration in a scratch directory and running the predicate
against the real compiler — the same technique C1's rehearsal uses, and for the same reason: the
alternative is installing and removing compilers to make a check fire. `VER-MOK-008` anticipates this in
Residual uncertainty: *"C4 and T3–T4 are rehearsed by deliberate breakage."* Both directions refuse and
both name both versions, which is what establishes rule 9.5's *"the comparison is against the exact
version, not a minimum"*.

## Static and architecture checks

| Row | Status | Where |
| --- | --- | --- |
| S1 — no tag, branch, commit or push | **observed** | `static-checks.md` S1, Appendix A, Appendix B |
| S2 — write access is confined | **observed**, with a platform limitation stated | `static-checks.md` S2 |
| S3 — no artifact is written | **observed** | `static-checks.md` S3 |
| S4 — no version is changed | **observed** | `static-checks.md` S4 |
| S5 — no secret is required or emitted | **observed** | `static-checks.md` S5 |
| S6 — no check writes to the tracked tree | **observed** | `static-checks.md` S6, `determinism-rehearsal.md` |
| S7 — the declared compiler version has one home | **observed** | `static-checks.md` S7, Appendix C |
| S8 — the managed harness workflow is untouched | **observed** | `static-checks.md` S8, Appendix D |
| S9 — the engine's dependency table is unchanged | **observed** | `static-checks.md` S9 |
| S10 — the human procedure lives outside the artifact root | **observed** | `static-checks.md` S10 |
| S11 — the process definition parses | **observed** | `static-checks.md` S11, Appendix D |

All eleven are observed in the sense the contract intends — the search was run and its raw output is
retained rather than paraphrased. Two carry statements a reader should not have to dig for:

- **S2** holds against rule 12.5 as amended on 2026-08-19, and the amendment is why. The rule originally
  asked that write access be confined to the attaching *step*, which GitHub Actions cannot express —
  `permissions` is a job-level key. It now asks that write access be confined to the smallest job
  containing the attaching step and that the credential reach that step alone, which is what the
  definition does and what S2 measures. The platform reasoning is recorded at the grant itself so the
  job-level scope does not later read as carelessness.
- **S8**'s verdict is a function of the harness build as well as of the repository. At `0.4.0` `doctor`
  is 81 `PASS` lines and no `FAIL`; at `0.4.1` the same commit reports eight `FAIL distribution:` lines.
  Neither reading is a tampering finding.

## Manual assessments

| Row | Status | Why |
| --- | --- | --- |
| M1 — the human sequence | **not performed** | the assurance owner reads `docs/RELEASE_RUNBOOK.md` and confirms it. The runbook exists and states the ordered acts, the accountable role per act, the preconditions, and its own lack of authority; confirming that is the owner's act |
| M2 — the archive, read by someone who did not build it | **not performed**, and structurally cannot be performed by this work order | see the Provenance section |
| M3 — the refusal set is complete | **not performed** | the owner compares the R table against rule 4 and rule 5. This file supplies the comparison material; the judgement is theirs. C5 above is an instance of M3's own standing being exercised |
| M4 — the release contract's judgements | **not performed** | there is no release contract yet. It is a precondition of a release, not of this work order |
| M5 — the two revisions were not conflated | **not performed** as a log read; the mechanism is **observed** | A2 makes it mechanical for the gate; `static-checks.md` S11 records the three run-time assertions that make it mechanical for the process |

All five are the assurance owner's acts by construction. Listing them as not performed is the accurate
statement: this work order prepares records and does not exercise accountable decision rights.

## Property and invariant tests

`VER-MOK-008` names eight. All eight are covered.

| Property | Test | Status |
| --- | --- | --- |
| Refusal totality | `test_p4_the_gate_writes_nothing_when_it_refuses` (`:412`), `test_property_a_refusal_is_idempotent` (`:444`) | **observed** across the refusal set, not per scenario |
| Authorization is a conjunction | verified by construction — every R scenario breaks exactly one fact from the A1 fixture | **observed** structurally |
| Commit equality is total, not prefix | `test_property_commit_equality_is_total_not_prefix` (`:422`) | **observed** |
| Set equality is symmetric | R20 and R21, both directions | **observed** |
| Type is read, never inferred | `test_property_type_is_read_for_a_work_order` (`:429`), `test_property_type_is_read_for_a_verification_record` (`:433`), and R13 for the contract | **observed** for all three relation targets |
| Idempotence of the gate | `test_property_the_gate_is_idempotent` (`:437`), `test_property_the_check_is_idempotent` (reachability, `:248`) | **observed** |
| The governance revision is fixed | `harness`, `verify` and `build` step 2 each assert `git rev-parse HEAD` against the value handed to them; `static-checks.md` S11 | **rehearsed** — the assertions are read, not run |
| Determinism of the provenance statement | `static-checks.md` S5 reads the construction; V6 would exercise it | **not performed**; the "no other input reaches the file" half is observed statically |

The type-is-read property is worth singling out. `VER-MOK-008` states that R13 exercises it for the
contract and *"the property extends to work orders and verification records"* without giving those two
rows. Two tests were written for them rather than leaving the extension asserted, which is why the suite
carries three type-is-read cases against the contract's one.

## Security and privacy checks

| Check | Covered by | Status |
| --- | --- | --- |
| No secret in the repository | S5; the restricted-path constraint in `REPOSITORY_CONTEXT.md` | **observed** — `grep -n 'secrets\.'` over the definition returns nothing |
| The provenance statement leaks nothing | V5 and S5 | **observed** statically; V5 **not performed** |
| A moved tag cannot redirect a release | R11 | **observed** |
| A branch cannot impersonate a tag | R2 | **observed** — resolution is confined to `refs/tags/` |
| A feature branch cannot be released | R23 | **observed** |
| An artifact cannot lie about its type | R13 and the type-is-read property | **observed** |
| The gate cannot alter what it inspects | P4 | **observed**, including against a real checkout |
| Only one step can write | S2 | **observed**, up to the per-job platform limitation |
| Assets are not published by automation | P2 | **not performed** — requires a run. `--draft` on the create path and the absence of any visibility flag on the re-run path are recorded in `static-checks.md` S1 |

## Performance and resilience checks

| Check | Covered by | Status |
| --- | --- | --- |
| A refusal costs no compilation | job order: `authorize` has no `needs`, and `build` needs it | **rehearsed** — read from the definition, not measured on a refused run |
| A compliance failure costs no packaging | C4 | **not performed**; the structural argument is in `compliance-rehearsal.md` C4 |
| A concurrent push cannot change the answer mid-run | the fixed-revision property, by the contract's own choice — *"rather than by racing, since a race is not reliably reproducible"* | **rehearsed** |
| The full history is available where it is needed | `fetch-depth: 0` on the `authorize` checkout (line 89) — the only job that resolves a tag object and walks ancestry. The other three checkouts take a named commit and read files at it, so they do not need it | **observed** in the definition; **observed** in the suite, where every fixture resolves a real annotated tag and a real ancestry. See the note below |
| No performance target is asserted | nothing to check | n/a |

**A note on the history row, because an earlier reading of it was wrong.** `fetch-depth: 0` appears once,
on the `authorize` checkout, not on all four. That is sufficient, because `authorize` is the only job that
resolves `refs/tags/<tag>`, reads the tag object's type, and asks whether a commit is an ancestor of a
branch — the three operations a shallow clone would break. `harness`, `verify` and `build` each check out a
commit named for them by `authorize` and then read files at it, which a depth-1 checkout of a specific SHA
satisfies. Recorded explicitly so that the single occurrence does not later read as an omission on the
other three, and so that anyone who *moves* a history-dependent check into one of those jobs knows it needs
the flag added.

## Reconciling 70 tests against the enumeration

The two suites hold 70 tests: 48 in `scripts/test_check_release_authorization.py` and 22 in
`scripts/test_check_release_reachability.py`. `suite-output.md` holds the full output; both suites report
`OK`.

Partitioned by test rather than by row, because that is what the two tables below do: **40 of the 70 carry
an enumerated row's name** — 29 that are the only test for their row, 7 further tests for a row already
covered (A3 and A4 gain one each, A5 four, R23 one), and 4 P4 cases — and **30 carry none**.

The two tables below hold 30 tests, but not the same 30. They add the five row-named cases that go beyond
one test per row and are worth an anchor of their own, and they omit the five `test_property_*` cases
already credited in *Property and invariant tests* above. The eight properties are not a disjoint bucket:
each is carried by tests counted elsewhere, which is what that table shows. Between the tables in this file
every one of the 70 is accounted for.

The 30 are tests the contract does not enumerate, and `VER-MOK-008` is explicit that this does not
make them conforming behavior on its own: *"one that refuses more is not thereby conforming, because an
unlisted refusal is an unspecified behavior."* Each one below is therefore anchored to a rule or to the
Error and recovery behavior section of `SPEC-MOK-005`, so that none of them is a refusal the
specification does not ask for.

### Extra cases in the gate suite

| Test | Anchor | What it establishes |
| --- | --- | --- |
| `test_a3_a_record_stating_a_different_tag_is_not_a_match` (`:259`) | 4.4 | A3's converse. The conventional-tag fallback applies only when the record states *no* tag; a record stating a different one is not silently matched |
| `test_a4_the_authorized_facts_are_emitted_for_the_workflow` (`:270`) | 4, closing paragraph; Data and interface contracts | the machine-readable channel, separately from A4's human-readable summary. *"The authorized facts are the only channel between authorization and every later step"* |
| `test_refuses_front_matter_that_does_not_parse` (`:453`) | 4.2; "Every check fails closed" | *"an unreadable artifact, malformed front matter … each is a refusal, never a pass"* |
| `test_refuses_front_matter_that_is_never_closed` (`:467`) | Data and interface contracts | the closing delimiter is *"the first `+++` line after the first"*; an unterminated block is a refusal, not an empty parse |
| `test_ignores_prose_that_carries_no_front_matter` (`:474`) | 4.2 | a tolerance, not a refusal. A README under the artifact root is not an artifact and must not become a duplicate-identifier failure |
| `test_ignores_files_under_an_excluded_directory` (`:481`) | 4.2 | *"Directories the validator excludes are excluded here too, so a template is not mistaken for an artifact"* |
| `test_reads_the_declared_artifact_root_rather_than_assuming_one` (`:497`) | Inputs; 4.2 | the artifact root comes from the harness configuration, not from a hard-coded path |
| `test_refuses_when_the_declared_artifact_root_is_missing` (`:506`) | "an absent configuration value … is a refusal" | fail-closed on a configuration that points nowhere |
| `test_refuses_when_the_configuration_is_unreadable` (`:511`) | same | fail-closed on a configuration that cannot be parsed |
| `test_a5_a_ready_verification_record_still_refuses` (`:663`) | 4.10; A5's ladder | one rung of the ladder, isolated: `ready` is not an eligible status |
| `test_a5_the_six_existing_records_cannot_release_an_aggregate` (`:670`) | 4.11; Compatibility and migration | the six existing records cover their own work at their own commits, so they cannot stand in for the aggregate record rule 4.10 requires at the candidate commit |
| `test_property_a_refusal_is_idempotent` (`:444`) | Refusal totality; Idempotence | the idempotence property on the refusal side, which the contract states for the gate's outcome generally |

Two of those twelve — `test_ignores_prose_that_carries_no_front_matter` and
`test_ignores_files_under_an_excluded_directory` — are tolerances rather than refusals, and they exist
because the real repository contains both kinds of file. Without them rule 4.2 would refuse this
repository for having a README in its evidence directories.

### Extra cases in the reachability suite

| Test | Anchor | What it establishes |
| --- | --- | --- |
| `test_accepts_a_commit_on_the_default_branch` (`:119`) | 5 | the positive case for the default branch |
| `test_accepts_a_commit_on_a_maintenance_branch` (`:122`) | 5; the second conforming example | a maintenance-branch tip that is not an ancestor of the default branch |
| `test_accepts_a_commit_a_maintenance_branch_merely_contains` (`:127`) | 5 | ancestry, not tip equality |
| `test_accepts_a_maintenance_branch_nobody_named_in_advance` (`:134`) | 5 | *"enumerated from the remote's own references rather than assumed"* |
| `test_prefers_the_default_branch_when_both_contain_the_commit` (`:140`) | 5 | which branch is reported when several qualify — an observability property, not an authorization one |
| `test_reads_the_default_branch_from_the_remotes_own_head` (`:146`) | 1, last two bullets | the default branch is discovered, not hard-coded |
| `test_honours_an_explicitly_declared_default_branch` (`:150`) | 1 | the event payload's name wins when supplied |
| `test_emits_the_containing_branch_for_the_workflow` (`:159`) | Observability | the branch is reported so the run states why the commit qualified |
| `test_names_every_branch_it_examined` (`:191`) | *"a refusal names the fact"* | a refusal that lists the branches searched is actionable; one that says "not reachable" is not |
| `test_r23_a_local_release_branch_does_not_count` (`:185`) | 5 | *"present on the remote"* is the requirement. A local `release/0.1` in the runner's clone must not satisfy it |
| `test_refuses_an_abbreviated_commit` (`:201`) | 4.5, applied here | abbreviations are refused wherever a commit is read, not only in the record |
| `test_refuses_a_revision_name_rather_than_a_hash` (`:204`) | 4.5 | `HEAD` or a branch name is not a commit |
| `test_refuses_an_uppercase_commit` (`:208`) | 4.5 | *"only lowercase hexadecimal characters"* |
| `test_refuses_a_commit_that_is_not_present` (`:211`) | "Every check fails closed" | a commit the clone does not hold is a refusal, not an assumption of reachability |
| `test_refuses_when_the_clone_holds_no_release_bearing_reference` (`:216`) | 5, second paragraph | R24's fixture-level sibling: no default branch and no maintenance branch at all |
| `test_refuses_when_the_default_branch_cannot_be_determined` (`:222`) | 1 | *"An absent name is a refusal, not a fallback to a hard-coded branch name"* |
| `test_refuses_a_remote_that_has_no_references` (`:226`) | 5 | an empty remote refuses rather than passing vacuously |
| `test_property_the_check_fetches_nothing` (`:253`) | 4's *"writes nothing"*, extended | the check performs no network operation, so it cannot mutate the remote or depend on its reachability at run time |

None of these adds a refusal condition the specification does not state. Nine of the eighteen are
positive or reporting cases rather than refusals; the nine refusals are all applications of rule 4.5,
rule 5, rule 1 or the fail-closed clause to the reachability component.

## What this file does not establish

- **That the enumeration is complete.** That is M3, and it is the assurance owner's. `VER-MOK-008` says
  so itself: *"A missing row is a gap in this contract, not in the implementation."* C5 is one instance
  already found and closed by amendment; that one was found only because its method was attempted rather
  than assumed, which is not a guarantee about the rows nobody attempted.
- **That any scenario has run on the release path.** None has. The repository has never released, and
  every C, P and V row that requires a run is marked accordingly above rather than argued into a pass.
- **That the gate refuses a fact neither the specification nor the implementer thought of.** The
  contract's first residual uncertainty, unchanged by anything here.
