+++
id = "REQ-MOK-036"
type = "requirement"
title = "Re-establish declared compliance at the released revision before publishing"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "Before any release asset is published, the release process SHALL re-establish that the harness declares the artifact graph valid and every released work order passes review preflight at the revision holding the release record, and that the repository's declared formatting, lint, test, dependency-boundary and determinism checks pass at the authorized commit."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-007"]
+++

# Requirement: Re-establish declared compliance at the released revision before publishing

## Rationale

`REQ-MOK-035` establishes that a release was authorized. It does not establish that the thing authorized is in a
compliant state, and those are different claims.

The tempting shortcut is to treat a merged commit as a checked commit. Every pull request runs the managed harness
workflow, which validates the graph and preflights the work order, and every merge to the integration branch has
therefore passed it. But two things are true at release time that were not true at merge time.

First, **the release record did not exist when the code was merged.** It is written afterwards, by construction —
the harness states that the record "is intentionally created after the candidate commit it names". So the graph
that authorizes the release has never been validated as a whole by anything, unless it is validated now. The
records added since the candidate commit are precisely the records nothing has checked.

Second, **a check's result is a statement about a revision, not a property a commit keeps.** The declared lint gate
is `-D warnings`, which is version-sensitive: a compiler release can introduce a lint that fails an unchanged
commit. A dependency-boundary claim can be falsified by a lockfile change on a later commit that the release is
built from. Treating an old green result as a current fact is the specific error this requirement forbids.

There is a third reason, less about correctness and more about what a release means. This repository's claim is
that shipped behavior traces to approved decisions. If the released archive is produced without re-checking, the
strongest statement anyone can make about it is that it compiled. Re-checking at the released revision is what
makes the archive's provenance statement worth reading.

## Preconditions and trigger

**Trigger.** A release has been authorized under `REQ-MOK-035`, which has emitted the authorized commit, the
released work orders, and the revision at which the graph was read.

**Preconditions.**

- The authorized commit and the governance revision are both fixed values, established once and not re-derived.
- The harness configuration in the repository declares a harness version.
- The repository's declared checks are the ones recorded in `docs/engineering/REPOSITORY_CONTEXT.md`; this
  requirement does not define its own set.

## Required response

The process SHALL establish all of the following before any asset is published.

**At the revision holding the release record:**

1. **The harness in use is the harness the repository declares.** The installed harness's version equals the
   version the repository's harness configuration declares. A mismatch is refused rather than tolerated, because a
   different version can validate a different rule set.
2. **The harness reports the repository healthy.** The harness's own repository check passes, which includes the
   integrity of every managed file.
3. **The artifact graph is valid.** The repository's artifact validation reports zero errors across every governed
   plane.
4. **Every released work order passes review preflight.** For each work order the release record releases, the
   harness's review-phase preflight passes.

**At the authorized commit:**

5. **Formatting is clean.** The declared formatting check passes.
6. **Lints are clean.** The declared lint check passes across the whole workspace, all targets and all features,
   with warnings treated as errors, resolving dependencies from the committed lockfile without updating it.
7. **Tests pass.** The declared test command passes across the whole workspace, resolving dependencies from the
   committed lockfile without updating it. No test tier is skipped, feature-gated, ignored, or dependent on an
   interactive terminal.
8. **The engine's dependency boundary holds.** The engine package's dependency tree resolves to exactly one crate.
9. **Declared determinism holds.** For each decision policy the engine declares as deterministic, two runs with
   identical inputs produce byte-identical output, an identical final state and an identical exit code.

When all hold, the process may proceed to produce assets. When any does not hold, the process SHALL publish nothing
and SHALL name the check that failed.

## Failure and boundary behavior

- **A failing check refuses the release.** There is no advisory outcome, no warning that permits continuation, and
  no override that skips a check.
- **An installed harness version that differs from the declared one** is refused before any other harness check
  runs, in either direction — newer or older. A newer harness may apply rules the repository has not adopted; an
  older one may not apply rules it has.
- **A validation warning is not a failure.** The validator distinguishes errors from warnings and the repository
  currently carries several warnings and open amendment rows. Refusing on warnings would refuse a release the
  repository considers valid; the release contract is where a judgement about a warning belongs.
- **A preflight failure names the work order.** With several released work orders, a bare failure would not say
  which.
- **A determinism check applies only to policies declared deterministic.** A policy whose output legitimately
  varies is out of scope, and asserting byte-identity on it would be a false claim rather than a strict one.
- **A check that cannot run is a failure.** An unavailable toolchain, an unavailable harness, a missing configured
  value — each refuses. An unrunnable check has established nothing.
- **The two revisions are not interchangeable.** Graph checks run at the governance revision because the release
  record exists only there; code checks run at the authorized commit because that is what is being released.
  Running either at the other revision is a defect, even when both happen to pass.
- **No check writes to the repository.** A check that would modify a tracked file — reformatting rather than
  reporting, updating a lockfile rather than honouring it — is not permitted, because the released commit must be
  the commit that was checked.

## Constraints

- The checks are exactly the ones `docs/engineering/REPOSITORY_CONTEXT.md` declares, invoked with the same
  commands, so a developer can rehearse them locally before pushing a tag. The process introduces no gate that
  cannot be reproduced by hand.
- Dependency resolution honours the committed lockfile and does not update it, so the release is built from the
  dependency versions the repository recorded.
- The managed harness workflow is not invoked, reused, or modified. It is integrity checked, and it declares no
  reusable-workflow trigger.
- Harness commands are invoked in the form the repository documents, because the harness console entry point is
  not assumed present on the executable search path.
- No check requires a secret.
- Compilation results are not carried between the checking step and the production step. A cache shared between
  them would let an artifact built before a check reach the archive.

## Acceptance examples

**All checks pass.**
Given an authorized release naming commit `C` and governance revision `G`, releasing `WO-MOK-001`..`006`,
when compliance is re-established,
then the harness version matches the declared version, the repository check passes at `G`, artifact validation
reports zero errors at `G`, six review preflights pass at `G`, and formatting, lints, tests, the dependency-tree
count and the determinism comparisons pass at `C`; and the process proceeds.

**An invalid graph at the governance revision.**
Given the release record was merged together with an artifact that fails validation,
when compliance is re-established,
then validation reports at least one error at `G` and the process refuses, publishing nothing — even though
`REQ-MOK-035` authorized the release.

**A newly introduced lint.**
Given a compiler version whose lint set now flags code at `C` that was clean when it merged,
when the lint check runs at `C`,
then it fails and the process refuses, naming the lint check.

**A dependency added to the engine.**
Given commit `C` in which the engine package's dependency table is no longer empty,
when the dependency-boundary check runs,
then the engine's dependency tree resolves to more than one crate and the process refuses, naming the boundary.

**A work order that no longer passes review preflight.**
Given `WO-MOK-004` whose required evidence path was removed by a later commit reachable from `G`,
when review preflight runs for each released work order,
then the preflight for `WO-MOK-004` fails, the process refuses, and the refusal names `WO-MOK-004`.

**A harness version drift.**
Given an installed harness version that differs from the version the repository's harness configuration declares,
when compliance is re-established,
then the process refuses before running any harness check, naming both versions.

**A warning-only graph.**
Given a graph at `G` reporting zero errors and several warnings, including open amendment rows,
when compliance is re-established,
then validation is treated as passing and the process proceeds.

## Open decisions

- **Whether a validation warning should ever refuse a release.** Currently no. Whether particular warning classes
  — an unclassified work order, a stale amendment row — should be promoted to release-blocking is unresolved and
  belongs to the assurance owner.
- **Whether the determinism comparison should cover more than two runs, or run on more than one platform.** Two
  runs on one platform per deterministic policy is the current claim; a stronger claim would need its own
  requirement rather than a quiet widening here.
- **Whether the released commit's own graph should be validated in addition to the governance revision's.** It was
  validated when it merged, and validating it again would not exercise the records that authorize the release.
  Whether the redundancy is worth its cost is unresolved.
- **Whether preflight phases other than review should be required.** Review is the phase that corresponds to work
  already complete; whether an additional phase adds information at release time is unresolved.
