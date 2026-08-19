+++
id = "INT-MOK-007"
type = "intent"
title = "Make a released build traceable to the decisions that authorized it"
status = "draft"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
+++

# Intent: Make a released build traceable to the decisions that authorized it

## Problem

Six work orders are `implemented` and six verification records are `verified`, and none of that work has ever
left the repository. There are **zero git tags**, no release contract, no release record, and no published
asset. Both package manifests declare `0.1.0` and always have. Anyone who wants to run Mokiterions clones the
repository and builds it.

That is a reasonable state for a proof of concept, and it is not the problem. The problem is what happens the
first time someone wants a build without building it.

**The repository can produce a build, but it cannot produce a build that says what it is.** The observer already
reads a compile-time `MOKITERIONS_COMMIT` (`mokiterions-tui/src/render.rs:34`) so that a build can carry its own
provenance, and nothing populates it. A binary handed to someone is therefore indistinguishable from a binary
built from uncommitted work, a reverted commit, or an unmerged branch. The traceability this repository spends
considerable effort on — an intent that derives a capability that derives requirements a specification specifies
and a verification contract verifies, bound to a commit by a verification record — stops at the repository
boundary. Cross that boundary and the chain is gone.

**Nothing states which decisions a release contains.** `docs/engineering/TRACEABILITY.md` and
`docs/engineering/DECISION_RIGHTS.md` describe release records and release authority, the harness ships
`prepare-release` and both release templates, and the artifact graph has room for a `release_contract` and a
`release_record`. None has ever been used. So the question "which decided work is in this build" currently has
no answer that is written down anywhere, for any build.

**The six verification records cannot answer it either.** Each binds a different commit — one per work order,
which is exactly right for verifying one work order and exactly wrong for describing an aggregate. A release of
all six is a claim about **one** commit, and no existing record makes that claim. The harness enforces this
directly: `prepare-release` refuses unless the included verification records identify one candidate commit.

**And the acts that make a release are the ones nobody may automate.** `ENGINEERING_HARNESS.md` states that
harness commands "may prepare records, but never exercise accountable decision rights or commit, push, tag,
release, publish, or deploy". `DECISION_RIGHTS.md` reserves the transition to `released` to the release owner.
So a release cannot be a button, and it cannot be a script that decides. Whatever automation exists must be
strictly downstream of a decision a person already recorded — which means the decision has to be recorded
somewhere first, and today there is nowhere for it to go.

The consequence is not hypothetical. The moment a build is shared, this repository's central claim — that every
behavior traces to an approved decision — becomes unverifiable by the person holding the build, who is precisely
the person who most needs it.

## Desired outcomes

- A person who receives a build can read, from the build itself, the exact commit it was compiled from and the
  identifiers of the records that authorized it, without access to the repository.
- One artifact states which work a given release contains, and one artifact records the decision to release it.
  Both are ordinary members of the artifact graph, subject to the same validation as everything else.
- An aggregate release is bound to a single commit, by a verification record captured at that commit, so the
  claim "this release contains this decided work, verified" is a claim about one thing rather than six.
- Publication is impossible unless that decision already exists. Not discouraged, not reviewed afterwards —
  refused, by default, with the missing fact named.
- The checks this repository declares are re-established at the revision actually being released, rather than
  inferred from the fact that some earlier pull request was green.
- Every accountable act — tagging, transitioning a record, making a release visible — remains a thing a person
  does, and the ordered sequence of those acts is written down instead of living in one person's memory.
- A release that must be stabilized has a branch to stabilize on, cut at the moment the branching model says.

## Actors and stakeholders

- **The product owner** accepts that this initiative delivers no simulation behavior and consumes engineering
  capacity for a distribution outcome, as `INT-MOK-003` and `INT-MOK-005` did for structural ones.
- **The release owner** bears the decision. They decide what a release contains, they transition the release
  record, they create the tag, and they make the release visible. This initiative exists to give that role
  somewhere to record its decisions and something that refuses to act without them.
- **The assurance owner** decides whether the aggregate evidence supports a release, by transitioning one new
  verification record captured at the candidate commit.
- **The technical owner** owns the specification of the pipeline and the contract for what a published asset
  states.
- **A recipient of a build** is a stakeholder who has no other stakeholder's access. They cannot read the
  artifact graph, run the harness, or inspect the history. Everything they can verify, they must verify from the
  asset in their hands. This is the actor the initiative is for.
- **Developers and implementation agents** run the pipeline's checks locally before proposing a candidate.
- **Operators of the simulation** are unaffected. No simulation or observer behavior changes.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Published releases whose contents are stated by an artifact in the graph | 0 of 0 | all | Per release |
| Published assets from which the full commit can be read without the repository | 0 | all | Per release |
| Distinct commits bound by the verification records one release includes | 6 | 1 | Per release |
| Accountable acts performed by automation — tag, transition, publish, push | 0 | 0 | Every run |
| Declared repository checks re-run at the released revision before publication | 0 | all | Per release |
| Released work orders re-checked by review preflight at release time | 0 | all | Per release |
| Compiler version recorded with each published asset | 0 | all | Per release |
| Secrets the release process requires to be held in the repository | 0 | 0 | Every run |
| Published assets lacking a checksum published beside them | — | 0 | Per release |
| Ways to publish an asset without a `released` release record | unbounded | 0 | Every run |

The last row is the initiative's actual purpose and the only one whose baseline is not a count. Today nothing
prevents anyone from building and distributing anything; the target is that the supported path refuses.

## Non-goals

- **Any change to simulation or observer behavior.** Identical inputs must produce byte-identical output, an
  identical final state and an identical exit code, and the observer must present identical frames.
- **Publication to a package registry, a container registry, an installer channel, or a package manager.** An
  archive attached to a tag is the whole distribution model.
- **Cryptographic signing, attestation, or a supply-chain provenance framework.** A checksum beside each asset
  and a provenance statement inside it are the stated ceiling. Signing is a separate decision with its own key
  custody problem, and this initiative must not pre-empt it.
- **Reproducible builds in the strict sense.** Two builds of one commit on two machines are not claimed to be
  bit-identical. Determinism is claimed only where the repository already claims it: identical seeded output
  from one build of one binary.
- **Automating any accountable act.** Not tagging, not transitioning a record, not making a release visible, not
  creating a branch, not bumping a version in a manifest. Each is reserved to a person by
  `ENGINEERING_HARNESS.md` and `docs/engineering/DECISION_RIGHTS.md`, and this initiative does not seek an
  exception.
- **A versioning policy.** Whether `0.1.0` becomes `0.2.0` or `1.0.0`, and what a version communicates, is a
  product decision this initiative neither makes nor needs.
- **Release notes generated from commit history.** What a release contains is what its release record
  authorizes, not whatever landed in a commit range.
- **Stamping the commit into the binary.** The observer's footer cannot currently carry it — see the risks below
  — and `WO-MOK-008` owns that question. This initiative routes provenance around the binary rather than
  changing what the binary displays.
- **Editing the managed harness workflow.** `.github/workflows/engineering-harness.yml` is managed and integrity
  checked; it declares no `workflow_call` trigger and is not a component this initiative may reuse or modify.
- **Retrofitting the six existing verification records.** They are bound to their commits and are correct about
  their own work. An aggregate release adds a record; it does not edit one.

## Principles and immutable constraints

- **No automation exercises an accountable decision right.** This is `ENGINEERING_HARNESS.md`'s rule, not a
  preference derived from it, and it constrains every design below.
- **Fail closed.** Every check refuses by default and passes only on an established fact. A check that cannot
  determine the answer refuses; it does not proceed.
- **A refusal names the fact it could not establish.** A gate that says only "denied" moves the problem rather
  than reporting it.
- **A release names exactly one commit, and a tag is never moved.** A moved tag falsifies every prior statement
  about it, including statements already in someone else's hands.
- **Verify at the revision being released, do not infer from history.** A green pull request proves something
  about the tree at that time.
- **The graph is read where the graph is complete.** A release record quotes the commit it authorizes and can
  therefore only be written afterwards, so the record and the code it authorizes are never in the same tree.
  Any design that assumes otherwise is wrong.
- **No secret enters the repository.** `docs/engineering/REPOSITORY_CONTEXT.md` requires it, and the release
  process must need nothing beyond the hosting platform's own scoped credential.
- **Releasing adds no dependency to either package.** The engine's dependency table stays empty, with no
  exception, per `ARCH-MOK-001` as amended.
- **What a recipient can check, a recipient can check alone.** A provenance claim that requires the repository
  to verify is not provenance for the actor this initiative serves.

## Risks and assumptions

- **Fact: the six verification records bind six different commits.** `VREC-MOK-001` binds
  `ecd03a89c0c8680d8ce82d7767b787a49aa815fb`; the others bind their own. An aggregate release therefore requires
  one additional verification record captured at the final candidate commit, and the harness refuses to prepare
  a release record without it.
- **Fact: a record cannot name the commit that contains it.** The harness states this in both directions — "the
  release candidate commit may precede the governance commit retaining this record" and "the record is
  intentionally created after the candidate commit it names, avoiding self-referential commit metadata". The
  consequence is that the tagged tree does not contain its own release record, so governance state must be read
  from the integration branch and the tag consulted separately. A design that reads the graph from the tag
  refuses every genuine release.
- **Fact: the observer's provenance footer cannot currently carry a commit.** Measured at the narrowest declared
  viewport (34x22), the `short` footer tier is 33 columns wide and the stamp costs at least three, so any stamp
  — even one character — forces a drop to the `tiny` tier and breaks a test the repository declares.
  `docs/engineering/simulation/evidence/WO-MOK-008/footer-tier-fallthrough.md` records the measurement and
  `WO-MOK-008` owns the fix. Until it is settled, provenance travels beside the binary rather than inside it,
  and the initiative must not treat that as a defect in the release process.
- **Fact: `MOKITERIONS_COMMIT` is not a Cargo rebuild input.** Neither package has a build script, so setting
  the variable and re-running `cargo` can reuse artifacts compiled without it and report a pass that means
  nothing. Any future attempt to stamp a build must force a recompile and prove it did.
- **Fact: the branching model already answers where a release stabilizes.**
  `docs/engineering/REPOSITORY_CONTEXT.md` cuts `release/<major>.<minor>` from the integration branch "when a
  release enters stabilization" — before publication, not after — and identifies released versions with tags
  such as `v1.4.0`. This initiative adopts that model rather than inventing one, which means automation cannot
  create the branch: by the time automation runs, the moment has passed.
- **Risk: an automated gate becomes the authority it was built to serve.** A gate that decides is worse than no
  gate, because it launders automation as governance. The mitigation is that every check re-reads a decision a
  person recorded and the process publishes only an unpublished draft, leaving the final act to a person.
- **Risk: a gate that is never exercised negatively is assumed to work.** A refusal path that has never refused
  is untested behavior. The mitigation is that the refusal conditions are enumerated by the verification
  contract, not by the implementation, and each is exercised against a repository built to fail it.
- **Risk: verification becomes the implementation's own test suite.** The gate's tests are written by whoever
  writes the gate. The mitigation is that at least one oracle is independent of both: the current repository,
  where the correct answer is a refusal and remains so until a release record exists.
- **Risk: the version in a manifest and the version in a release disagree.** Bumping a manifest is authorized
  work, not a release step, so the two can drift. The mitigation is that the release record states the version
  and the pipeline uses that value rather than reading a manifest.
- **Assumption: the hosting platform can attach files to an existing tag without creating one, and can withhold
  a release from public view until a person acts.** If either is untrue on the platform in use, the publication
  boundary must be re-specified rather than relaxed.
- **Assumption: an exact compiler version can be selected by a file in the repository.** The measured toolchain
  is `rustc 1.97.1 (8bab26f4f 2026-07-14)`; both packages declare `edition = "2024"` and the workspace declares
  `resolver = "3"`, so the floor is far below the pin and the pin is about recording what built the evidence,
  not about a minimum.
- **Open decision: whether an architecture and a deciding ADR are required.** A release and publication pipeline
  touches the deployment and operating model, which is an architecturally significant trigger. No active
  architecture addresses any requirement derived from this intent, so none is currently applicable; whether one
  should exist is the technical owner's decision and must be settled before the implementing work order is
  approved.
- **Open decision: whether the first release proceeds with `WO-MOK-008` open.** The footer defect is measured
  and unfixed. Releasing knowingly is a legitimate choice; releasing unknowingly is not. The release contract is
  where that choice is recorded.
