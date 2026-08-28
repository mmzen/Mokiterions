+++
id = "SPEC-HUP-001"
type = "specification"
title = "The standard-root adoption transaction: what it reads, what it may change, what it must refuse, and what must hold afterwards"
status = "approved"
owners = ["engineering owner"]
created = "2026-08-28"
updated = "2026-08-28"

[relations]
specifies = ["REQ-HUP-001"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T20:00:00Z"
decided_by = "engineering owner"
+++

# Specification: the standard-root adoption transaction

## Scope

This specification fixes the transaction by which this repository's standard root moves from one exact
released `se_harness` version to another. It governs the adoption only. It fixes no simulation rule, no
product behavior and no release act, and it does not decide the content of the managed policy the adopted
version brings.

## Actors and external systems

- The **engineering owner** authorizes the adoption in a work order and performs the transaction.
- The **adopted evaluator** is an exact published `se_harness` release, installed into an environment outside
  this checkout. It plans and applies the transaction and afterwards grades the tree.
- The **public package index** supplies the evaluator. No credential is used and no artifact is fetched from
  anywhere else.

## Inputs

1. The exact version string of a published `se_harness` release, named in the authorizing work order.
2. The repository working tree at the commit the transaction runs on.
3. The authorizing work order's `[evaluator_upgrade]` packet, when releases require declaration.

## Outputs

1. The rewritten standard root: managed files replaced, fragments integrated, new managed files added.
2. `.engineering-harness.lock`, rewritten to the adopted evaluator's lock schema and recording its identity.
3. Retained transaction evidence, written to a repository path below the artifact root.

## State model

The root is in exactly one of three states.

- **Current** — `.engineering-harness.toml` names the adopted version and the lock records it. `doctor`
  reports no FAIL.
- **Behind** — the root names an earlier version. The repository still operates under that earlier evaluator,
  and a newer evaluator may refuse it.
- **Frozen** — a newer evaluator reads an error the repository cannot repair, because repairing it needs a
  lifecycle transition and transitions refuse on an invalid graph.

The transaction moves *behind* to *current*. Rule 4 exists so that it never moves *behind* to *frozen*.

## Behavioral rules

**Rule 1 — exactness.** The adopted version is an exact published release, named in full in the authorizing
work order. A range, a pre-release, a local build, a source checkout and a development version are each
refused as an adoption target.

**Rule 2 — the evaluator is outside the tree.** The transaction is planned, applied and graded by an
evaluator installed outside this checkout. No script inside the repository performs the adoption, and the
in-tree managed scripts are the transaction's *subject*, never its instrument.

**Rule 3 — plan before apply.** The transaction is planned first, and the plan states every path it would
add, update or integrate together with a count of what it leaves unchanged. Applying re-derives the plan and
refuses if the tree or the plan moved in the interval.

**Rule 4 — declaration before transition.** A released record carrying no evaluator evidence is declared in
the authorizing work order, in an `[evaluator_upgrade]` table taking the schema
`se-harness-evaluator-upgrade-v1`, the scope `standard-root-only`, and an array
`legacy_releases_without_evaluator_evidence` naming each such record.

The declaration has force only from a work order whose status grants authority and which carries a `draft` to
`approved` lifecycle event with a `decided_at` instant later than the record's `released_at`. An undeclared
record refuses the transaction **before the first write**.

**Rule 5 — a declaration exempts and never writes.** Resolving a declared record adds no field to it,
recomputes none, and repoints none. `RLS-MOK-001` after this transaction is byte-identical to `RLS-MOK-001`
before it.

**Rule 6 — the declaration is closed to what predates enforcement.** A record may be declared only while it
is `released`, carries neither an evaluator-evidence path nor digest, and was released before the declaring
work order was approved. A record that could have carried evidence is not eligible, and the declaration is
not a way to skip producing it.

**Rule 7 — repository-owned version references move with the root.** A repository-owned file that names the
evaluator version is moved to the adopted version in the same change as the transaction. At this adoption
that is `.github/workflows/release.yml`, whose `SE_HARNESS_VERSION` names the version its jobs install.
Leaving it behind would have one workflow install one evaluator to grade a root installed by another.

**Rule 8 — evidence is retained.** The transaction writes its evidence to a repository path below the
artifact root, naming the adopted version, the file surface, and every identifier the packet declared.

**Rule 9 — customization refuses.** A managed file this repository has edited is reported as customized and
refuses the transaction with no file written. It is never merged, never overwritten silently, and never
resolved inside the transaction.

**Rule 10 — no product effect.** The transaction changes no file under `mokiterions-core/` or
`mokiterions-tui/`, no `Cargo.toml`, no `Cargo.lock` and no `rust-toolchain.toml`, and creates, moves and
deletes no tag.

## Error and recovery behavior

- **Undeclared released record.** Refused before any write, naming each undeclared identifier and the packet
  field that would declare it. Recovery: declare it under rule 4, or abandon the adoption.
- **Customization or conflict.** Refused with `installation has conflicts or customizations; no files were
  written`. Recovery: restore the managed file, or accept the customization as repository-owned and remove it
  from managed control by its own change.
- **Plan drift between plan and apply.** Refused with `upgrade plan or installed root changed before apply`.
  Recovery: re-plan and re-apply on a settled tree.
- **Evidence path already occupied.** Refused with no file written. Recovery: choose an unused path.

In every case the transaction is all-or-nothing: a refusal leaves the root exactly as it was.

## Data and interface contracts

The lock is the adopted evaluator's, in its own schema. At `0.8.0` it is schema 3 and records the evaluator's
`version`, a `payload_manifest`, a `payload_sha256`, and an `archive_name`/`archive_sha256` pair that is null
when the evaluator was installed from the index rather than from a named archive. This repository asserts
nothing about that shape beyond the fact that the adopted evaluator writes it and reads it back.

The `[evaluator_upgrade]` packet's three fields are the adopted evaluator's contract, not this repository's.
Its `scope` is `standard-root-only`: the packet authorizes a root adoption and nothing else.

## Security and privacy properties

The transaction reads no credential, sends nothing outside the repository, and spawns no process other than
the evaluator the owner installed. The evaluator is acquired from the public index at an exact version over
the index's own transport. No retained evidence carries a secret, because none is read.

## Performance and capacity

Not a constraint. The transaction is a bounded file rewrite over a plan of tens of files and completes in
seconds. No figure is fixed here and none is measured.

## Observability

After the transaction the adopted evaluator's `validate` and `doctor` operations report the root's state, and
the retained evidence records what the transaction did. Those three, together, are the whole observable
surface; the transaction emits no log of its own.

## Compatibility and migration

**The managed continuous-integration trigger does not match this repository's default branch.** The `0.8.0`
managed workflow fires push runs on `main`, `release/**` and `candidate/**`. This repository's default branch
is `master`, so after the adoption the managed lane runs on pull requests and no longer on pushes to the
default branch. The workflow is a managed file: editing it would report it customized and refuse every
subsequent adoption under rule 9.

This is recorded as a post-adoption effect and is **not** settled here, on the engineering owner's decision of
2026-08-28. Pull-request runs are unaffected, and every merge into `master` to date has been gated by one.
What is lost is the post-merge push run on the default branch, which a verification record may quote as
evidence.

Existing artifacts are not migrated. The adopted evaluator reads the `simulation` domain exactly as the
previous one did, and the 141 authoring advisories it reports are pre-existing wording observations, not
migration work.

## Examples and counterexamples

**Example.** Exact public `0.8.0` is named in `WO-HUP-001`; `RLS-MOK-001` is declared; the plan reads 61
files, 13 unchanged, no customization; the transaction applies; the lock becomes schema 3 at `0.8.0`;
`validate` reports 0 errors and `doctor` no FAIL. Conforms.

**Counterexample.** The same adoption, but the managed workflow is first edited to say `master`. The plan
reports the workflow as customized and the transaction refuses with nothing written. Rule 9, and the reason
the branch question is settled by its own change rather than inside this one.

**Counterexample.** `RLS-MOK-001` is given a hand-written `evaluator_evidence_path` pointing at a file
produced today. Validation passes and the transaction proceeds. This violates rule 5 and is a fabrication: the
`0.1.0` release commit was never evaluated by the evaluator that path would name.

## Explicitly unspecified decisions

- Which evaluator version is adopted next, and when. Each adoption names its own version in its own work
  order.
- Whether this repository's default branch is renamed to `main`. Deferred by the engineering owner on
  2026-08-28; see *Compatibility and migration*.
- Whether the 141 authoring advisories are addressed, and by whom. They are outside this transaction.
- The content of the managed policy the adopted version brings. This specification governs the act of
  adoption, not what is adopted.
