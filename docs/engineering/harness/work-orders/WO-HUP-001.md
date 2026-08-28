+++
id = "WO-HUP-001"
type = "work_order"
title = "Adopt exact public se_harness 0.8.0 as the standard root, declaring RLS-MOK-001 as a release that predates evaluator evidence"
status = "approved"
owners = ["engineering owner"]
created = "2026-08-28"
updated = "2026-08-28"

[assurance]
commit_bound_verification = "required"
rationale = "This work changes managed policy, continuous integration and the traceability surface that every later engineering, assurance and release decision in this repository is read through. Three claims cannot be settled by inspecting the diff. That the complete artifact graph validates with zero errors is a claim about 1,350 artifacts read by an evaluator, not about the 48 files the transaction rewrites. That the declaration exempted RLS-MOK-001 rather than writing evidence into it is a claim about bytes that did not change, checkable only against the record as it stood before. That no product behavior moved is a claim about the whole package tree rather than about the paths anyone intended to touch. The transaction also replaces the in-tree validator, so the instrument a later reader would reach for is itself part of what changes, and only an external evaluator bound to an exact commit can settle whether the result is sound."
decided_by = "engineering owner"

[evaluator_upgrade]
schema = "se-harness-evaluator-upgrade-v1"
scope = "standard-root-only"
legacy_releases_without_evaluator_evidence = ["RLS-MOK-001"]

[execution_scope]
paths = [
  ".agents/",
  ".claude/",
  ".engineering-harness.lock",
  ".engineering-harness.toml",
  ".gitattributes",
  ".github/workflows/engineering-harness.yml",
  ".github/workflows/release.yml",
  "AGENTS.md",
  "CLAUDE.md",
  "ENGINEERING_HARNESS.md",
  "docs/engineering/ARTIFACT_AUTHORING.md",
  "docs/engineering/DECISION_RIGHTS.md",
  "docs/engineering/OPERATING_CARD.md",
  "docs/engineering/QUALITY_GATES.json",
  "docs/engineering/QUALITY_GATES.md",
  "docs/engineering/TECHNICAL_COMMUNICATION.md",
  "docs/engineering/TRACEABILITY.md",
  "docs/engineering/WORKFLOW.json",
  "docs/engineering/WORKFLOW.md",
  "docs/engineering/harness/",
  "docs/engineering/templates/",
  "scripts/generate_harness_dashboard.py",
  "scripts/harness_explorer/index.template.html",
  "scripts/inspect_engineering_artifacts.py",
  "scripts/select_harness_work_order.py",
  "scripts/validate_engineering_artifacts.py",
]

[relations]
implements = ["REQ-HUP-001"]
specifications = ["SPEC-HUP-001"]
verification = ["VER-HUP-001"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T20:00:00Z"
decided_by = "engineering owner"
+++

# Work Order: adopt exact public se_harness 0.8.0

## Lifecycle

Governance work. `approved` authorizes the transaction; `implemented` follows the applied transaction and its
retained evidence. Commit-bound verification is `required`, so a verification record binding an exact
candidate commit is a separate, later act and is not authorized by this work order.

The `[evaluator_upgrade]` packet above carries `SPEC-HUP-001` rule 4's declaration. It has force only once
this work order is approved, and only from a `draft` to `approved` lifecycle event whose instant is later than
`RLS-MOK-001`'s `released_at` of `2026-08-19T17:53:05Z`.

## Objective

Move this repository's standard root from `se_harness` **0.4.0**, installed 2026-08-11, to exact public
**0.8.0**, and leave the complete artifact graph validating with zero errors under that evaluator.

## In scope

1. Declaring `RLS-MOK-001` under the `[evaluator_upgrade]` packet above, so the transaction is not refused and
   the repository is not frozen by a release cut before evaluator evidence existed.
2. Applying the transaction with an evaluator installed from the public index at exact version `0.8.0`, into
   an environment outside this checkout.
3. Moving `.github/workflows/release.yml`'s `SE_HARNESS_VERSION` from `0.4.0` to `0.8.0`, under
   `SPEC-HUP-001` rule 7. This file is repository-owned, so the transaction does not move it and this work
   order does.
4. Adding the `harness` artifact domain that carries this chain.
5. Retaining the evidence `VER-HUP-001` contracts.

## Out of scope

- **Every simulation change.** No file under `mokiterions-core/` or `mokiterions-tui/`, and no Cargo or
  toolchain manifest, is touched.
- **Any release act.** `RLS-MOK-001` stays `released` at `0.1.0`, at the commit it binds, byte-identical.
- **Renaming the default branch.** The managed workflow's push trigger does not match `master`; the
  engineering owner deferred that question on 2026-08-28. It is recorded, not repaired.
- **The 141 authoring advisories** the `0.8.0` evaluator reports against existing requirements. They are
  pre-existing, they are `maintenance` severity, and addressing them is not this transaction.
- **Editing any managed file.** Doing so would report it customized and refuse every later adoption.
- **Any verification record.** `required` assurance is discharged separately.

## Authorized decision envelope

The engineering owner may, without further authorization: choose the exact evidence paths below
`docs/engineering/harness/evidence/WO-HUP-001/`; re-run the plan and the apply on a settled tree after a
refusal; and word the retained summaries.

The engineering owner may **not**, under this work order: adopt a version other than exact public `0.8.0`;
declare any record other than `RLS-MOK-001`; edit a managed file; write evidence into a release record; or
rename a branch. Each of those is a fresh decision.

## Constraints

- `SPEC-HUP-001` rules 1 to 10 govern the transaction in full.
- The evaluator is installed outside the checkout. The in-tree validator is a cross-check only, never a pass.
- The transaction is all-or-nothing. A refusal leaves the root as it was, and the response is to satisfy the
  refusal's stated condition, never to work around it.
- `RLS-MOK-001` is not edited, by this work order or by the transaction.

## Expected change surface

Measured on 2026-08-28 at `0970363` by planning the transaction: **61 files, 13 unchanged, 48 add or update,
no customization and no conflict.**

- **Managed, updated (22):** `.engineering-harness.lock`, `.engineering-harness.toml`,
  `.github/workflows/engineering-harness.yml`, `AGENTS.md`, `CLAUDE.md`, `ENGINEERING_HARNESS.md`,
  `docs/engineering/DECISION_RIGHTS.md`, `docs/engineering/QUALITY_GATES.md`,
  `docs/engineering/TRACEABILITY.md`, `docs/engineering/WORKFLOW.md`, five files under
  `docs/engineering/templates/`, and five under `scripts/`.
- **Fragment, integrated (1):** `.gitattributes`.
- **Managed, added:** `docs/engineering/ARTIFACT_AUTHORING.md`, `docs/engineering/OPERATING_CARD.md`,
  `docs/engineering/QUALITY_GATES.json`, `docs/engineering/TECHNICAL_COMMUNICATION.md`,
  `docs/engineering/WORKFLOW.json`, and the `.agents/` and `.claude/` skill trees.
- **Repository-owned, this work order's own edit (1):** `.github/workflows/release.yml`.
- **New artifacts (6):** this chain, under `docs/engineering/harness/`.

The lock moves to schema 3 and records evaluator `0.8.0` with a null `archive_name`/`archive_sha256` pair,
which is the shape an index install produces. `.engineering-harness.toml` keeps `schema_version = 2` — that
field is the configuration's schema, not the lock's — and its `tool_version` becomes `0.8.0`.

## Required verification

`VER-HUP-001` in full: A1 through A7, N1, N2, S1 and M1. A1 carries `REQ-HUP-001`.

A3 requires the **refusal** to be observed, so the undeclared plan is captured before the declaration is
approved rather than reconstructed afterwards.

## Evidence to record

Under `docs/engineering/harness/evidence/WO-HUP-001/`, as `VER-HUP-001`'s *Evidence retention* enumerates.
The completion summary additionally records, as a stated post-adoption effect:

> The managed lane's push trigger fires on `main`, `release/**` and `candidate/**`. This repository's default
> branch is `master`, so after this adoption the managed lane runs on pull requests and no longer on pushes to
> the default branch. Pull-request runs are unaffected. The branch name is not settled by this work order.

## Stop and escalate conditions

Stop and return to the engineering owner when:

- the plan reports any path as `customized` or `conflict`;
- the transaction refuses for a reason other than the undeclared record this work order declares;
- `validate` reports any error after the transaction, including one this work order did not predict;
- the in-tree validator disagrees with the external evaluator on the error count;
- `RLS-MOK-001` shows any difference at all; or
- satisfying a refusal would require editing a managed file, declaring a further record, or adopting a
  different version.

## Completion report format

1. The adopted version, and the evaluator installation it was run from.
2. The plan as measured: files, unchanged, add or update, customization.
3. The declared identifiers and the two instants N1 requires.
4. `validate` and `doctor` results, with counts.
5. The in-tree validator's agreement.
6. The change surface, against out-of-scope directories.
7. The post-adoption effect above, stated as deferred.
8. What is left owed: the commit-bound verification record.
