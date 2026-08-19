+++
id = "REQ-MOK-035"
type = "requirement"
title = "Refuse to produce or publish a release the graph does not authorize"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "When a release is requested for a tag, the release process SHALL establish that a release record with status 'released' names the exact commit that tag resolves to and that every fact that record depends on holds, SHALL produce no asset and publish nothing when it cannot, and SHALL name the fact it could not establish."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-007"]
+++

# Requirement: Refuse to produce or publish a release the graph does not authorize

## Rationale

A tag is a name a person attaches to a commit. Nothing about creating one proves the commit was decided, verified,
or intended for release, and a tag can be created by mistake, from the wrong branch, or before the record that
would authorize it exists. Between the tag and the release there is therefore a gap that no earlier check can
close, because every earlier check ran before the tag existed.

`prepare-release` closes the part of the gap it can reach: it refuses to write a release record unless the graph
supports one. `.github/workflows/engineering-harness.yml` closes another part: it validates the graph on every
pull request. Neither can answer the question that matters at release time — *does the tag name the commit the
release owner authorized* — because both ran before the tag existed.

This requirement makes the answer to that question a precondition for producing anything at all. The default is
refusal, and the only route out of it is a decision that is already recorded in the graph. That inverts the usual
arrangement, in which automation acts and a person is expected to notice if it should not have.

The refusal must name the missing fact for a practical reason: the release owner reads it while deciding what to
do next, and "authorization failed" leaves them to re-derive the whole check by hand.

## Preconditions and trigger

**Trigger.** A release is requested for a tag, either by pushing a tag matching the released-version naming
convention or by an explicit request naming an existing tag.

**Preconditions.** None. This requirement holds in every state of the repository, including the state in which no
release record, no release contract and no verification record exists — which is the repository's state as of
2026-08-19 and in which the correct response is a refusal.

The requirement deliberately assumes nothing about how the tag came to exist, who created it, or whether earlier
automation succeeded. Every fact it relies on is re-established from the repository at the moment of the request.

## Required response

The process SHALL establish all of the following, and SHALL produce no asset and publish nothing unless all hold.

1. **The tag exists and is annotated.** The named tag resolves to a commit in the repository, and the tag object
   is an annotated tag, so it records who created it and when. A lightweight tag is refused.
2. **A release record authorizes it.** Exactly one release record with status `released` claims the tag, and the
   full commit it names equals the commit the tag resolves to. Character-for-character equality of full-length
   hashes; no abbreviation is accepted on either side.
3. **The record's commit is full length.** The record's commit is a complete hash for the object format the
   repository declares, and the repository declares that full commits are required.
4. **The gating contract is active.** The contract the record satisfies exists, is a release contract, and holds
   a status that is active rather than draft, rejected or superseded.
5. **The contract gates the released work.** Every work order the record releases appears in the contract's gated
   set.
6. **The released work is releasable.** Every work order the record releases exists and holds a status of
   `implemented`, `verified` or `released`.
7. **Verification binds the same commit.** Every verification record the release record includes exists, holds a
   status of `verified` or `released`, and names the same commit the release record names.
8. **Released and verified work agree.** The set of work orders the release record releases equals the set the
   included verification records cover. Work released but not verified is refused; work verified but not released
   is refused.
9. **The version is usable.** The record states a version that can be used as a release name and an archive name
   component without further interpretation.
10. **The commit is reachable from a release-bearing branch.** The authorized commit is an ancestor of the
    integration branch or of some maintenance branch. A commit reachable only from a feature or bugfix branch is
    refused.

When all hold, the process SHALL emit the facts it authorized on — commit, version, release record, release
contract, released work orders and included verification records — so that every later step reads them from that
one statement rather than re-deriving them.

When any does not hold, the process SHALL write a refusal that names the fact it could not establish, and SHALL
terminate with a failing status.

## Failure and boundary behavior

- **No release record at all.** Refused, naming the absence. This is the repository's current state and the
  behavior is not an error condition to be worked around.
- **Two release records claim one tag.** Refused. Ambiguous authorization is not authorization, and choosing one
  would be the process deciding.
- **Two artifacts share an identifier.** Refused before any other check, because the graph cannot be read
  unambiguously.
- **A record that is `ready` but not `released`.** Refused. `ready` is a prepared proposal; only the release owner
  transitions it, and the process must not treat a proposal as a decision.
- **A record that names an unrelated commit.** Refused, whether the mismatch is a mistake or a moved tag. The
  process does not distinguish the two, because the response is the same.
- **A tag that was force-moved after the record was written.** Refused, by the same equality check. This is the
  case the requirement most needs to catch, because the graph looks entirely correct.
- **An abbreviated commit in the record.** Refused rather than expanded. Expanding it would make the process
  resolve an ambiguity that belongs to whoever wrote the record.
- **A record that states no tag.** Accepted if the tag equals the conventional name derived from the record's
  version. The tag field is optional in the harness's own template, and refusing every record that omits it would
  refuse records the harness itself produces.
- **A referenced artifact that does not exist.** Refused, naming the identifier and the relation that pointed at
  it.
- **An artifact whose type is not the type the relation requires.** Refused. The relation's name is not evidence
  about the target; the target's own declared type is.
- **A repository that does not require full commits.** Refused. The process's central equality check is between
  full hashes, and it must not run where the repository permits abbreviations.
- **Failure while establishing a fact** — an unreadable artifact, malformed front matter, a git command that
  fails — is a refusal, not a pass. The process never treats an unanswerable question as answered.
- **A partial outcome is not permitted.** A refusal at any point leaves no asset produced and nothing attached.

## Constraints

- The check runs before any build, so a refusal costs no compilation.
- The check performs no write of any kind: no artifact is created or edited, no commit is made, no tag is created
  or moved, no status is transitioned.
- The graph is read at the revision where the release record exists, which is later than the commit the record
  names. Reading the graph from the tagged tree would find no record and refuse every genuine release.
- The revision at which the graph is read is fixed once and reused by every later step, so a concurrent push
  cannot change the answer mid-run.
- The facts emitted are the only source for the version, commit and identifiers used later. No later step reads a
  package manifest, a branch name or a commit range to determine what is being released.
- The check requires no secret and no network access beyond reading the repository.

## Acceptance examples

**Authorized: the ordinary case.**
Given a release record `RLS-MOK-001` with status `released`, version `0.1.0`, tag `v0.1.0` and commit `C`,
satisfying an approved contract that gates `WO-MOK-001`..`006`, releasing those six work orders, and including a
`verified` verification record that also names `C` and covers exactly those six,
and given `C` is an ancestor of the integration branch,
and given the annotated tag `v0.1.0` resolves to `C`,
when a release is requested for `v0.1.0`,
then the process authorizes it and emits `C`, `0.1.0`, `RLS-MOK-001`, the contract, the six work orders and the
verification record.

**Authorized: the record lives in a later commit than the one it names.**
Given the graph above, and given the release record was committed after `C` so that the tree at `v0.1.0` does not
contain the release record at all,
when a release is requested for `v0.1.0`,
then the process authorizes it. This is the normal order of events, not an exception.

**Authorized: the record omits the tag field.**
Given the graph above with the record's tag field absent and its version `1.0.0`,
when a release is requested for `v1.0.0`,
then the process authorizes it.

**Refused: the tag moved.**
Given a correct and unmodified graph authorizing commit `C`,
and given the tag `v0.1.0` was force-moved to a later commit `D`,
when a release is requested for `v0.1.0`,
then the process refuses, naming that the tagged commit is not the commit the release owner approved, and produces
nothing.

**Refused: the decision was never taken.**
Given a release record with status `ready`,
when a release is requested for its tag,
then the process refuses, naming that no release record holds status `released`.

**Refused: verification does not bind the released commit.**
Given a release record naming commit `C`, including a verification record that is `verified` but names commit `E`,
when a release is requested,
then the process refuses, naming that the release requires one verification record captured at the final
candidate commit.

**Refused: released work the contract does not gate.**
Given a release record releasing `WO-MOK-001` and `WO-MOK-002`, and a contract gating only `WO-MOK-001`,
when a release is requested,
then the process refuses, naming `WO-MOK-002`.

**Refused: work released but not verified.**
Given a release record releasing `WO-MOK-001` and `WO-MOK-002`, and included verification covering only
`WO-MOK-001`,
when a release is requested,
then the process refuses, naming `WO-MOK-002` as released but not verified.

**Refused: a commit reachable only from a feature branch.**
Given a graph authorizing commit `F`, reachable from `feature/example` and from no integration or maintenance
branch,
when a release is requested for a tag resolving to `F`,
then the process refuses, naming that the commit is not reachable from a release-bearing branch.

**Refused: nothing has been decided.**
Given the repository as it stands, with no release record, no release contract and no aggregate verification
record,
when a release is requested for any tag,
then the process refuses, naming that no release record exists.

## Open decisions

- **Whether a maintenance branch older than the current release line remains release-bearing indefinitely.** The
  reachability check accepts any maintenance branch that exists on the remote. Whether an abandoned line should be
  excluded, and by what rule, is unresolved and is deliberately left permissive rather than guessed.
- **Whether the conventional tag fallback should be removed once release records reliably state their tag.** It
  exists because the field is optional in the harness template. If every record in practice states it, the
  fallback is a widening with no purpose.
- **Whether a release should be refusable on the grounds of an open amendment row or an open defect report.** Four
  amendment rows and one measured defect are currently open; the release contract, not this requirement, records
  the judgement, and whether that judgement should be mechanically enforced is unresolved.
