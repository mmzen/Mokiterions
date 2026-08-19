+++
id = "CAP-MOK-007"
type = "capability"
title = "Publish a release that refuses to exist without a recorded authorization"
status = "draft"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
derives_from = ["INT-MOK-007"]
+++

# Capability: Publish a release that refuses to exist without a recorded authorization

## Actor and need

Three actors need something the repository cannot currently give them.

**The release owner** needs to convert a set of decided work into a distributable release without becoming the
only safeguard against distributing the wrong thing. Today every check that a release is legitimate would be a
check they perform themselves, from memory, at the moment they are most under pressure to ship. They need the
mechanism to refuse when their own decision is absent or inconsistent, so that their attention is spent on the
decision rather than on its bookkeeping.

**A recipient of a build** needs to establish what they are holding, using only what they were handed. They have
no clone, no harness, and no access to the artifact graph. They need the commit, the version, and the identifiers
of the authorizing records to be present in the thing itself, and they need a way to establish that the bytes
they received are the bytes that were produced.

**The assurance owner** needs the checks this repository declares to have been re-established at the exact
revision being released — not at whatever revision a pull request happened to be green on. They need that
statement to be produced by something other than the person asking for the release.

None of the three needs a faster release. All three need a release that cannot happen quietly.

## Capability statement

The repository can produce, from a tag naming a commit that a recorded release decision authorizes, a set of
platform archives that each state their own provenance and each carry a published checksum — and can refuse, with
the missing fact named, whenever that recorded decision is absent, inconsistent, or does not name the tagged
commit.

The capability is the refusal as much as the production. A pipeline that builds when authorized is common; a
pipeline whose default is to refuse, and whose only route to a build is a decision a person already recorded in
the artifact graph, is what this capability delivers.

## Boundaries

Inside the capability:

- Resolving a tag to a commit, and establishing whether a recorded release decision authorizes that exact commit.
- Reading the artifact graph at the revision where the release decision exists, which is later than the commit
  the decision names.
- Re-establishing harness compliance and the repository's declared checks before anything is produced.
- Selecting one declared compiler version and refusing to build or verify with another.
- Producing one archive per supported target, each containing the built binaries, the licence and readme, and a
  provenance statement; producing a checksum beside each archive.
- Attaching the result to the existing tag in a state that is not yet visible to the public.
- Stating, in every refusal, which fact could not be established.

Outside the capability:

- Deciding what a release contains, what version it carries, or whether the evidence is sufficient. Those are
  the release owner's and assurance owner's decisions, recorded before this capability does anything.
- Creating, moving, or deleting a tag.
- Transitioning any artifact's status, or writing, editing or committing any artifact.
- Creating or pushing any branch, including the maintenance branch the branching model requires.
- Making a release visible.
- Changing a version in a package manifest.
- Any change to simulation or observer behavior, output, or exit codes.
- Signing, attestation, registry publication, and installers.
- Stamping the commit into the compiled binary. The observer's provenance footer cannot currently carry it, and
  `WO-MOK-008` owns that question.

## Outcomes

- A tag whose commit no recorded decision authorizes produces no asset, no draft, and no partial upload — only a
  refusal that names what was missing.
- A published archive answers, from its own contents, four questions a recipient would otherwise have to ask
  someone: which commit, which version, which decisions, and which compiler.
- The identity of the release is stated once, by the release record, and every later step reads it from there
  rather than re-deriving it from a manifest, a branch name, or a commit range.
- Two independent facts must agree before anything is built: the tag resolves to a commit, and a `released`
  release record names that same commit. Neither alone suffices.
- A release built from a maintenance branch is releasable on the same terms as one built from the integration
  branch, and a release built from a feature branch is not releasable at all.
- The ordered sequence of human acts is written down, so a release is repeatable by someone who has not done one
  before.
- Failure is loud and early: the authorization check is the first thing that runs and the cheapest thing to fail.

## Actor-visible behavior

For the **release owner**: they record a decision in the graph, create an annotated tag, and push it. Either a
draft release appears carrying archives and checksums, or a named refusal appears and nothing else happens.
Between those two outcomes there is no third state in which some assets exist. The final act — making the release
visible — is theirs, and is the same act whether the pipeline succeeded on the first attempt or the fourth.

For a **recipient**: they download an archive and a checksum, verify the checksum with a standard tool, and read
a plain-text provenance file inside the archive. That file names the commit in full, the tag, the version, the
release record, the release contract, the released work orders, the included verification record, the build
target and the compiler version. Nothing in that list requires the repository to interpret.

For the **assurance owner**: the pipeline's own record shows the declared checks passing at the released commit
and the harness declaring the graph valid at the revision holding the release record, with every released work
order passing review preflight.

For a **developer**: the checks the pipeline runs are the checks already declared in
`docs/engineering/REPOSITORY_CONTEXT.md`, runnable locally with the same commands, so the pipeline introduces no
gate that cannot be rehearsed before pushing.

## Dependencies and assumptions

- The artifact graph already supports the two record types this capability reads. The harness ships
  `release_contract` and `release_record` templates, `prepare-release`, and validation rules for both; the
  capability consumes that machinery rather than adding a parallel one.
- An aggregate release depends on one verification record captured at the candidate commit. The six existing
  records bind six different commits, and the harness refuses a release record whose included records do not
  identify exactly one commit.
- The release record is written after the commit it names, so it lives on a later revision. Reading the graph
  from the tagged tree would find no record at all. This is a property of the harness, stated in its own source,
  not an accident of any particular history.
- The hosting platform is assumed able to attach files to an existing tag without creating one, and to hold a
  release unpublished until a person acts. If it cannot, the boundary above must be re-specified, not relaxed.
- The managed harness workflow `.github/workflows/engineering-harness.yml` is integrity checked and declares no
  reusable-workflow trigger. This capability re-runs the harness itself rather than invoking that workflow.
- The compiler version is assumed selectable by a file in the repository, and the currently measured version is
  `rustc 1.97.1 (8bab26f4f 2026-07-14)`.
- No secret beyond the platform's own scoped credential is assumed available, because
  `docs/engineering/REPOSITORY_CONTEXT.md` forbids one in the repository.

## Candidate requirements

- **Authorization before production.** A release run refuses to produce or publish anything unless a release
  record with status `released` names the exact commit the tag resolves to, and every fact that record depends on
  — an active gating contract, verification bound to that commit, releasable work orders, a full-length commit —
  holds. Every refusal names the fact it could not establish.
- **Compliance re-established at release time.** Before any asset is published, the harness declares the graph
  valid and every released work order passes review preflight at the revision holding the release record, and the
  repository's declared formatter, linter, test, dependency-boundary and determinism checks pass at the released
  commit.
- **Provenance carried by the asset.** Each published archive states its own commit, tag, version, authorizing
  record identifiers, target and compiler version, and each has a checksum published beside it. No archive states
  a credential or an absolute local path.
- **Accountable acts reserved to people.** The pipeline's own effects are confined to reading the repository,
  producing assets, and attaching them to an existing tag as an unpublished draft. Tagging, transitioning,
  publishing, branching and version changes remain human acts, and their order is documented.
- **A recorded compiler.** Building or verifying a release uses one compiler version declared in the repository,
  refuses any other, and records the version alongside the asset.

## Rationale

The alternative shapes were considered and rejected.

**Trusting the pull request.** The managed harness workflow already validates the graph on every pull request, so
one could argue a merged commit is a validated commit. It is — about the tree at that time, and about the graph as
it stood then. A release record is added afterwards, by definition, and the question at release time is whether
*that* addition is valid and whether the tag points where it claims. Neither question exists at merge time.

**A gate inside the harness.** `prepare-release` already refuses to write a release record unless the graph
supports it, which covers everything up to the moment the record is written. What it cannot cover is the tag: the
tag is created afterwards, by a person, and can name any commit. The gap between "a valid record exists" and
"the tag names the commit that record authorizes" is exactly the gap this capability closes, and it can only be
closed after the tag exists.

**A single job that decides and publishes.** Simpler, and wrong in a specific way: it would make the pipeline the
authority. Splitting authorization from production, and stopping at an unpublished draft, keeps the pipeline
strictly downstream of a recorded human decision and strictly upstream of a human act — which is the arrangement
`ENGINEERING_HARNESS.md` requires and the reason the capability is expressed as a refusal.

**Generating notes from commit history.** Cheap and misleading. A commit range describes what was written; a
release record describes what was authorized. When they differ, the second one is the true statement about the
release, and it is the one a recipient needs.
