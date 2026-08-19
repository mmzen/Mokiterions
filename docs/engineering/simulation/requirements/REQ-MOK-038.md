+++
id = "REQ-MOK-038"
type = "requirement"
title = "Confine the release process to acts that are not accountable decisions"
status = "draft"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "The release process SHALL confine its own effects to reading the repository, producing assets, and attaching them to an existing tag as a release that is not publicly visible; it SHALL NOT create, move or delete a tag, transition or modify any artifact, create or push any branch, or change a version in a package manifest; and the ordered sequence of human acts a release requires SHALL be documented in the repository."
verification_method = "static-analysis"

[relations]
derives_from = ["CAP-MOK-007"]
+++

# Requirement: Confine the release process to acts that are not accountable decisions

## Rationale

`ENGINEERING_HARNESS.md` states that harness commands "may prepare records, but never exercise accountable decision
rights or commit, push, tag, release, publish, or deploy". `docs/engineering/DECISION_RIGHTS.md` states that "only
accountable assurance and release owners may transition those records to `verified` or `released`; record
preparation never creates commits, tags, or publications."

Those rules govern the harness. Nothing yet extends them to a pipeline that runs on every pushed tag with write
access to the repository, and a pipeline is where they are easiest to violate — not maliciously, but because each
individual convenience is defensible. Creating the tag from a version file saves a step. Flipping a record to
`released` on success keeps the graph tidy. Cutting the maintenance branch automatically prevents someone from
forgetting. Publishing rather than drafting removes a click. Each of those, taken alone, is a small improvement in
convenience and a complete inversion of where authority sits: the pipeline would then be deciding, and the human
would be reviewing what automation had already done.

The inversion is worth naming precisely, because it is the failure this requirement exists to prevent. A gate that
decides is worse than no gate. No gate leaves the responsibility visibly with a person. A deciding gate moves the
responsibility to automation while leaving the appearance of governance intact, and the appearance is what everyone
downstream relies on.

The documentation clause is part of the same requirement rather than a separate one, because the two are the same
constraint seen from either side. Every act the process refuses to perform is an act a person must perform, and an
undocumented sequence of human acts is not a safeguard — it is a single point of failure wearing the costume of
one. If the acts are reserved to people, the order must be written down.

## Preconditions and trigger

**Trigger.** Any run of the release process, in any outcome — authorized, refused, or failed part-way.

**Preconditions.** None. This requirement constrains the process unconditionally, including in its failure paths,
where cleanup logic is the usual source of an unintended write.

## Required response

**The process's permitted effects are exactly:**

1. reading the repository at the revisions established by `REQ-MOK-035`;
2. running the checks required by `REQ-MOK-036`, which write nothing to the repository;
3. producing assets and their checksums, per `REQ-MOK-037`;
4. attaching those assets to a tag that already exists, as a release that is not publicly visible;
5. writing its own diagnostic output, including refusals.

**The process SHALL NOT:**

6. create, move, delete, or force-update any tag, whether locally or on the remote;
7. transition the status of any artifact, or write, edit, delete, or commit any artifact;
8. commit or push anything to any branch;
9. create any branch, including the maintenance branch the branching model requires;
10. change a version in a package manifest, a lockfile, or any other tracked file;
11. make a release publicly visible;
12. hold, require, or emit a secret beyond the hosting platform's own scoped credential.

**Write access SHALL be confined to the single step that attaches assets.** Every other step runs with read access
only, so that a defect in a check cannot produce a write at all.

**The ordered sequence of human acts SHALL be documented in the repository**, naming for each act who performs it,
what must already be true before it, and what it produces. That sequence covers at minimum: deciding the release
contents and version; capturing and transitioning the aggregate verification record; approving the release
contract; preparing and transitioning the release record; cutting the maintenance branch; creating and pushing the
annotated tag; and making the release visible.

**That documentation SHALL state that it is not authority.** It describes acts; it does not approve them, and where
it disagrees with a governed artifact the artifact prevails.

## Failure and boundary behavior

- **A refusal leaves no trace in the repository.** No branch, no tag, no commit, no attached asset, no partially
  created release.
- **A failure part-way through asset production leaves no published release.** Assets are attached in a single step
  or not at all, and the release remains invisible regardless.
- **Attaching assets to an existing tag is not tag creation and is permitted.** Creating a tag because it is absent
  is prohibited, and the absence of the tag is a refusal.
- **Requesting a release for a tag that exists only locally is refused.** A tag reachable only on one machine cannot
  be attached to, and creating it remotely would be tag creation.
- **The maintenance branch is not created by the process, even when it is obviously required.** The branching model
  cuts it when a release enters stabilization, which is before the tag exists; by the time the process runs, the
  moment has passed and creating it would place it at the wrong commit as well as being the wrong actor's act.
- **An optional additional human approval before assets are uploaded is permitted**, provided it can be enabled by
  configuration without editing the process. Adding a required reviewer narrows what automation may do; it never
  widens it.
- **A convenience that would perform a reserved act is refused as a design, not weighed against its benefit.** The
  benefit is real and irrelevant.
- **Diagnostic output is not a repository write**, and refusals must be verbose. The constraint is on repository
  and publication state, not on reporting.
- **A step that needs write access to do its job is a step in the wrong place.** The response is to move the work,
  not to widen the access.

## Constraints

- The process is confined to reading the repository, producing files outside the tracked tree, and one publication
  call that attaches assets to an existing tag without creating one.
- The publication call is invoked in a form that fails if the tag is absent rather than creating it.
- The default publication state is not publicly visible, and no configuration of the process changes that default.
- Write access is granted at the narrowest scope the platform allows, to the one step that needs it.
- The documented human sequence lives in the repository, outside the governed artifact root, so that it is
  ordinary operator documentation rather than an artifact claiming authority it does not have.
- The documented sequence names roles, not people.
- The managed harness workflow is neither edited nor invoked; it is integrity checked and declares no
  reusable-workflow trigger.

## Acceptance examples

**A refused run changes nothing.**
Given a repository with no release record and a pushed annotated tag,
when the release process runs and refuses,
then no branch, tag, commit, artifact change, attached asset or visible release exists that did not exist before,
and the refusal names the missing release record.

**An authorized run produces an invisible release.**
Given an authorized release,
when the process completes,
then a release exists holding the assets and their checksums, it is not publicly visible, the tag points at the
same commit it pointed at before the run, and no artifact's status changed.

**The absent tag is refused, not created.**
Given a request naming a tag that does not exist on the remote,
when the process runs,
then it refuses, naming the tag as absent, and no tag is created.

**The maintenance branch is not created.**
Given an authorized release whose commit is on the integration branch and for which no maintenance branch exists,
when the process completes,
then no branch was created, and the documented sequence names cutting that branch as the release owner's act.

**No status is transitioned on success.**
Given an authorized release whose release record has status `released` and whose released work orders have status
`implemented`,
when the process completes successfully,
then every artifact's status is exactly what it was before the run.

**Only the attaching step holds write access.**
Given the process definition,
when the access granted to each step is read,
then every step other than the one that attaches assets holds read access only.

**A required reviewer can be added without editing the process.**
Given the process as written,
when an additional approval is required before assets are uploaded,
then it takes effect through configuration alone and the process is unchanged.

**The human sequence is documented and disclaims authority.**
Given the repository,
when the release documentation is read,
then it states the ordered acts, names the accountable role for each, and states that it is not authority and does
not override a governed artifact.

**The version is not changed.**
Given a release record stating version `0.2.0` and package manifests declaring `0.1.0`,
when the process completes,
then the manifests still declare `0.1.0` and the assets state `0.2.0`.

## Open decisions

- **Whether the additional human approval before upload should be required rather than available.** Requiring it
  adds a second gate and a second thing to wait for; the release owner's judgement is currently the deciding
  factor, and the process supports either.
- **Whether the process should be permitted to delete an invisible release it previously attached**, so that a
  re-run replaces rather than accumulates. Deleting an unpublished draft destroys nothing a person relied on, but
  it is still a delete, and it is deliberately left prohibited pending a decision.
- **Whether the documented sequence should be automatable in part as a local, human-invoked script.** A script the
  release owner runs deliberately is a different thing from automation that runs on a push, and whether that
  distinction should be exploited is unresolved.
- **Whether an architectural decision record should be required for the deployment and operating model this
  requirement constrains.** A release and publication pipeline is an architecturally significant trigger; no active
  architecture currently addresses it.
