+++
id = "WO-MOK-009"
type = "work_order"
title = "Implement the release authorization gate, the release process and the compiler declaration"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-19"
updated = "2026-08-19"

[assurance]
commit_bound_verification = "required"
rationale = "This work decides whether an asset may leave the repository at all, so every later release decision rests on it being correct at a known commit. It is also the mechanism that re-establishes compliance for a release, which means a defect in it silently weakens the evidence behind every release that follows. The compiler declaration additionally changes the conditions under which all future evidence is produced. A commit-bound record is required because the refusal behavior is only meaningful as a statement about a specific revision of the gate and the process."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-035", "REQ-MOK-036", "REQ-MOK-037", "REQ-MOK-038", "REQ-MOK-039"]
specifications = ["SPEC-MOK-005"]
verification = ["VER-MOK-008"]
+++

# Work Order: Implement the release authorization gate, the release process and the compiler declaration

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

**Approval record.** On 2026-08-19 the repository owner, acting in all four accountable roles, approved the complete
governing chain — `INT-MOK-007`, `CAP-MOK-007`, `REQ-MOK-035` through `REQ-MOK-039`, `SPEC-MOK-005` and
`VER-MOK-008` — and authorized this work order, which was transitioned `draft` → `approved` → `in_progress` →
`implemented` on the same date. The implementation agent recorded the transitions; it did not make the decision.
`WO-MOK-008` is not part of that authorization and remains `draft`: it is a defect report against `SPEC-MOK-003`
with no implementation, and nothing here approves it.

**The four transitions of this work order were recorded as one act, and the reason is the disclosure below.** The
candidate was already complete and its evidence already retained when approval was given, so there was no interval
in which `approved` or `in_progress` described the repository. Recording the path rather than only its endpoint keeps
the lifecycle honest about which authority each step carries — approval authorized the scope, implementation records
the completed change — while not pretending the two were separated in time. What bridges them is the rule-by-rule
conformance statement in `docs/engineering/simulation/evidence/WO-MOK-009/candidate-conformance.md`, which states
what the candidate already satisfied and what had to change, because approval authorizes implementing
`SPEC-MOK-005` and does not adopt the candidate.

### A candidate implementation already exists, and it was written before this chain

Stated plainly because it changes what approval means here. A working candidate — an authorization gate, its
scenario suite, a five-stage release process, a compiler declaration and an operator procedure — exists on the
unmerged branch `feature/release-ci`, and it was produced **before** `INT-MOK-007`, `CAP-MOK-007`,
`REQ-MOK-035`..`039`, `SPEC-MOK-005` and `VER-MOK-008` existed. The governing artifacts were written afterwards, to
describe what a release must do; they were not derived from the candidate, but they were written by someone who had
seen it, and that is a real influence rather than a nominal one.

Two consequences follow.

1. **Approval does not adopt the candidate.** It authorizes implementing `SPEC-MOK-005`. Where the candidate
   conforms, it may be used; where it does not, it changes. The specification is the authority in every case, and
   the completion report states, rule by rule, what the candidate already satisfied and what had to change.
2. **The candidate's own scenario suite is not sufficient evidence.** `VER-MOK-008`'s Independence section addresses
   this directly: the refusal set is enumerated by the contract, and coverage is measured against that enumeration
   rather than against the candidate's existing tests. The candidate currently carries 22 scenarios;
   `VER-MOK-008` enumerates 24 refusals plus 5 authorizing scenarios plus the C, P, S, T, V and M families, so the
   suite grows rather than being inherited.

One defect the candidate has already surfaced is worth recording here, because it is the failure `SPEC-MOK-005`
rule 1 exists to prevent and it survived a passing suite: the candidate originally read the artifact graph from the
tagged tree, which cannot contain its own release record, so it would have refused every genuine release. The
scenario that would have caught it — `VER-MOK-008` A2 — did not exist, because the fixture wrote its artifacts into
the working tree at the tagged commit and the two revisions were therefore the same object. Treat that as the
worked example of why A2 is mandatory rather than illustrative.

### Approval preconditions

**1. The chain must be transitioned as a whole, in order.** `REQ-MOK-035`..`039` must not be transitioned to an
active status before `SPEC-MOK-005` and `VER-MOK-008` are also active. The validator raises a coverage error for a
requirement in an active status with no active specification or no active verification, and the five requirements
would trip both. Draft artifacts are exempt, which is why every artifact in this chain is currently `draft` and why
the order of transitions is a precondition rather than a detail.

**2. The technical owner must decide the architecture question.** A release and publication pipeline touches the
deployment and operating model, which is an architecturally significant trigger. No active architecture addresses
any of `REQ-MOK-035`..`039` — `ARCH-MOK-001` and `ARCH-MOK-002` address engine and observer requirements — so the
`architecture` relation is deliberately absent from this work order. That is a statement about what exists, not a
judgement that none should. If the technical owner decides one is required, an architecture and a deciding record
must exist and this work order must gain the relation before approval.

**3. `SPEC-MOK-005` must be approved by the technical owner** and `VER-MOK-008` by the assurance owner. Both are
`draft` and neither has been reviewed. `SPEC-MOK-005`'s amendment record carries the initial row as **Draft**.

**4. The release-record and release-contract types must be exercisable.** Neither has ever been used in this
repository. If `prepare-release` or `create-artifact` cannot produce a conforming artifact for either type, that is
a harness question to settle before implementation rather than during it.

### How each precondition was discharged

Recorded here rather than by editing the four above, so that what was required before the decision stays readable
beside what was done about it.

**1 — discharged by construction.** The ten transitions were written in one edit and one commit, so no revision of
this repository holds `REQ-MOK-035`..`039` active without `SPEC-MOK-005` and `VER-MOK-008` also active. The validator
was re-run on the transitioned tree: 79 artifacts, 0 errors, 0 warnings, with `E007` and `E008` now live against the
five requirements rather than exempt.

**2 — decided by the technical owner, and confirmed on the same date once the reading behind it was put to them.** The
`architecture` relation is absent and no architecture or deciding record was created, so the decision is that none is
required and that `SPEC-MOK-005` governs the release process alone. `WO-MOK-002` and `WO-MOK-004` are the precedent for
a work order without the relation; `WO-MOK-001`, `WO-MOK-003`, `WO-MOK-005` and `WO-MOK-006` carry one.

This was **first recorded as a reading of the approval rather than a separately worded decision**, and flagged as the
one item in this record a reader should check rather than accept, because precondition 2 says that if an architecture
*is* required then it, a deciding record and the relation must exist *before* approval — so construing an approval that
omitted all three would have made the approval premature rather than merely under-documented. It was put to the
technical owner on that basis, and **they confirmed it on 2026-08-19**: the instruction was *"architecture: I
confirm"*. The precondition is therefore discharged by a decision rather than by construal, and the correction path —
an `ARCH`, an `ADR`, an `architecture` relation here, and an amendment to this note — is closed rather than open. The
original reading is left standing above because the confirmation is an answer to it, and removing the question would
leave the discharge looking as though it had never needed asking. Nothing in the evidence changed either way.

**3 — discharged.** `SPEC-MOK-005` is `approved` by the technical owner and `VER-MOK-008` by the assurance owner,
both 2026-08-19. `SPEC-MOK-005`'s amendment record now carries the technical owner against its original row as well
as against the rule 12.5 amendment.

**4 — half discharged, and the other half is not reachable from here.** `create-artifact` produces both types:
`RLS-` under `simulation/releases/`, `REL-` under `simulation/release/`, with the transcripts in
`docs/engineering/simulation/evidence/WO-MOK-009/release-artifact-types.md`. `prepare-release` takes a verification
record as input, so it cannot be exercised before one exists and is eligible. That ordering is inherent to the
command rather than an omission, and it is a precondition of a release rather than of this work order — the same
position `VER-MOK-008` M4 already takes about the release contract.

## Objective

Implement `SPEC-MOK-005`: an authorization gate that refuses a release the artifact graph does not authorize, a
release process that re-establishes declared compliance and produces provenance-bearing assets without performing
any accountable act, a single compiler declaration honoured by every step, and the documented human sequence rule 14
requires.

The measure of success is the refusal, not the release. A run that publishes when authorized is the easy half; a run
that refuses every one of `VER-MOK-008`'s 24 enumerated conditions, names the fact it could not establish, and
leaves the repository untouched is the work.

## In scope

- The **authorization gate**: a repository-owned program implementing `SPEC-MOK-005` rule 4, runnable by hand with a
  repository root and a tag, writing nothing, emitting the authorized facts and a human-readable summary.
- The gate's **scenario suite**, covering every scenario `VER-MOK-008` enumerates in its A, R, P4 and property
  families, against fixture repositories built with real git objects and real `+++` front matter.
- The **release process** implementing rules 1 through 3, 5 through 8, and 10 through 13: the two revisions, the
  pinned governance revision, reachability, ordering, harness compliance, the declared repository checks, asset
  composition, the provenance statement, the publication boundary and the reserved-act prohibitions.
- The **compiler declaration** implementing rule 9, in one tracked file, with the components the declared checks
  need, and the version assertion in every verifying and building step.
- The **documented human sequence** implementing rule 14, outside the governed artifact root, disclaiming authority.
- Updating `docs/engineering/REPOSITORY_CONTEXT.md` — which is repository-owned — so that its setup line states the
  compiler declaration selects the toolchain, and its commands section points at the release procedure.
- Retained evidence under this work-order ID, including the static enumeration `VER-MOK-008` S1 through S11 requires.

## Out of scope

- **Any change to either Rust package.** No source, no manifest, no lockfile, no dependency, no build script. The
  engine's dependency table stays empty with no exception, and neither package's public interface is touched, so
  `SPEC-MOK-002` rule 5 and `SPEC-MOK-004` rule 6 hold by construction.
- **Any change to simulation or observer behavior**, output, or exit codes.
- **Stamping the commit into either binary.** `SPEC-MOK-005` rule 10.7 records why the build sets no commit stamp;
  `WO-MOK-008` owns the underlying defect, and this work order neither fixes nor depends on it.
- **Performing any accountable act.** Creating a tag, transitioning any artifact, cutting the maintenance branch,
  making a release visible, or bumping a version. Rule 13 forbids the process from doing them and this work order
  does not do them either.
- **Creating the release contract, the release record, or the aggregate verification record.** Those are the release
  and assurance owners' artifacts. This work order builds the mechanism that reads them.
- **Editing `.github/workflows/engineering-harness.yml`** or any other managed file. It is integrity checked, and
  rule 13.8 forbids invoking it as well.
- **Signing, attestation, registry or installer publication.** Out of scope by `INT-MOK-007`.
- **Additional supported targets** beyond the three rule 10 lists.
- **Actually performing a release.** Approval authorizes building the mechanism, not exercising it on a real
  publication.

## Authorized decision envelope

The implementation agent may choose:

- the gate's implementation language, internal structure, function and helper names, and how it parses front matter;
- the fixture construction strategy, scenario names, and how scenarios are parameterized, provided every enumerated
  scenario is present and each asserts both a failing status and the named fact;
- the exact wording of each refusal message, provided it names the fact that could not be established;
- the runner images, the archive container format, the checksum algorithm, and the release-notes prose beyond the
  facts rule 11.4 requires;
- comments and non-authoritative developer documentation.

The implementation agent may not decide which facts authorize a release, add or remove a refusal condition, relax any
rule 13 prohibition, widen a step's access, read the version or commit from anywhere other than the release record,
duplicate the declared compiler version into a second file, change a supported target, transition any artifact's
status, or perform any act rule 13 reserves.

If a rule of `SPEC-MOK-005` cannot be satisfied as written, that is a specification question for the technical
owner, not an implementation choice. Stop and escalate.

## Constraints

- Follow `SPEC-MOK-005`, `REQ-MOK-035` through `REQ-MOK-039`, and `VER-MOK-008`.
- `ENGINEERING_HARNESS.md`: harness commands never exercise accountable decision rights, and never commit, push,
  tag, release, publish or deploy. Nothing in this work order creates an exception, and the process built here is
  bound by the same rule.
- `docs/engineering/DECISION_RIGHTS.md`: only accountable assurance and release owners transition records to
  `verified` or `released`.
- `docs/engineering/REPOSITORY_CONTEXT.md`: no credential or secret enters the repository.
- The declared checks are the ones `REPOSITORY_CONTEXT.md` states, invoked with the commands it states, so a
  developer can rehearse the process locally.
- Harness commands are invoked as `python -m se_harness`; the console entry point is not on the executable search
  path in this repository.
- Add no dependency to either package and no build script.
- Do not edit any managed file. `python -m se_harness doctor .` must continue to pass, and
  `.github/workflows/engineering-harness.yml` must keep its recorded hash.
- Preserve unrelated changes in the worktree.

## Expected change surface

- The authorization gate and its scenario suite, as repository-owned scripts outside the artifact root.
- The release process definition, as a repository-owned workflow.
- The compiler declaration, as one tracked file at the repository root.
- The documented human procedure, under `docs/` and outside `docs/engineering/`.
- `docs/engineering/REPOSITORY_CONTEXT.md`, which is repository-owned after installation: the setup and commands
  sections only.
- Work-order-keyed evidence under `docs/engineering/simulation/evidence/WO-MOK-009/`.
- No Rust source, no manifest, no lockfile, no managed file, no artifact other than this work order's own status and
  its evidence.

## Required verification

- Complete every scenario in `VER-MOK-008`, and state for each whether it passed, and where its evidence is.
- Run the gate's scenario suite and show every A and R scenario passing, with the count reconciled against
  `VER-MOK-008`'s enumeration. A scenario in the contract with no test is a gap to report, not to omit.
- Show **A2** explicitly: a fixture in which the release record provably does not exist in the tagged tree, and the
  gate authorizing anyway. Assert the record's absence from the tagged tree rather than assuming it.
- Show **A5**: the gate run against this repository as it stands, refusing, naming the absent release record; and
  the refusal ladder as each governance artifact is added.
- Show **P4**: the gate leaves the working tree, including untracked files, unchanged, and creates no tag or branch.
- Show **S1 through S11** as a written enumeration of what was read and what was found, since a static read leaves
  no output of its own.
- Show the process definition parsing, and the revision each step reads: default branch for authorization, the
  pinned governance revision for compliance, the authorized commit for checks and builds.
- Show the compiler declaration in effect in a clone — the active toolchain reporting the declared version and
  naming the declaration as the reason — and the assertion refusing on a deliberate mismatch in both directions.
- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`.
- Run `cargo test --workspace --locked` and show that no test count changed, since no Rust source is touched.
- Run `cargo tree -p Mokiterions --locked` and show it resolves to one crate.
- Run `python -m se_harness doctor .`, `python scripts/validate_engineering_artifacts.py --root .` and
  `python -m se_harness inspect .`, and show zero errors.
- Run a review preflight for this work order and show it eligible.

## Evidence to record

Retain, under `docs/engineering/simulation/evidence/WO-MOK-009/`:

- the scenario suite's full output, with the scenario-to-contract mapping for every `VER-MOK-008` scenario;
- the A2 transcript, including the assertion that the tagged tree lacks the release record;
- the A5 refusal ladder against the real repository;
- the P4 before-and-after worktree comparison;
- the S1 through S11 static enumeration, naming each command read and each finding;
- the toolchain evidence: the active toolchain in a clone, and both directions of the mismatch refusal;
- formatter, linter, test and dependency-tree output, with the test count shown unchanged;
- harness `doctor`, `validate` and `inspect` output;
- a rule-by-rule statement of what the pre-existing candidate satisfied and what changed, per the Lifecycle section;
- a completion summary identifying the final affected components and every `VER-MOK-008` scenario that remains
  rehearsed rather than observed.

## Stop and escalate conditions

Stop before implementation if preflight does not report this exact work order as eligible, if `SPEC-MOK-005` is not
approved by the technical owner, if `VER-MOK-008` is not approved by the assurance owner, if the architecture
question in the approval preconditions is unresolved, or if the five requirements have been transitioned to an
active status ahead of the specification and the verification contract.

During implementation, stop and escalate if:

- a rule of `SPEC-MOK-005` cannot be satisfied as written, or two rules conflict;
- satisfying a rule appears to require performing an act rule 13 reserves, widening a step's access beyond the
  attaching step, adding a secret to the repository, or editing a managed file;
- the harness cannot produce a conforming release contract or release record, which makes the gate's inputs
  hypothetical;
- an enumerated refusal condition cannot be constructed as a fixture, since a refusal that cannot be exercised
  cannot be claimed;
- the reachability check cannot distinguish a maintenance branch from a feature branch on the hosting platform;
- the platform cannot attach assets to an existing tag without creating one, or cannot hold a release invisible —
  either falsifies an `INT-MOK-007` assumption and re-opens the specification;
- any declared check begins to fail at the candidate commit for a reason this work order did not introduce;
- the test count or either package's public interface changes, which would mean the work has reached the packages.

## Completion report format

Report:

1. each `SPEC-MOK-005` rule, and what implements it;
2. each `VER-MOK-008` scenario, its result, and its evidence path — including every scenario that is rehearsed
   rather than observed, since the repository has never released;
3. the rule-by-rule account of the pre-existing candidate: satisfied, changed, or discarded;
4. any authorized local decisions taken from the envelope above;
5. verification commands and results, including the toolchain mismatch refusals in both directions;
6. retained evidence paths;
7. what the release owner must still do before a release is possible, stated as the remaining acts rather than as a
   recommendation: the aggregate verification record, the release contract, the release record, the maintenance
   branch, the annotated tag, and the publication;
8. residual limitations, and final worktree and candidate-commit status.
