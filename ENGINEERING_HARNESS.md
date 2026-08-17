# Engineering Harness for Mokiterions

This repository uses SE Harness 0.4.0. This file is the single managed contract and router. Repository-owned instructions may add stricter local constraints, but they cannot waive this contract.

## Authority model

- `docs/engineering/REPOSITORY_CONTEXT.md` contains owner-curated repository facts and commands. It informs execution but grants no product or governance authority.
- Formal artifacts under `docs/engineering/` are the only repository-native source of product intent, requirements, architecture decisions, work authorization, verification contracts, and release constraints.
- Source, tests, conversations, dashboards, preflight, and CI are observations or evidence. They do not approve work, verification, or release.

## Author engineering artifacts

Use `harnessctl scaffold-domain . --domain <lowercase-kebab-domain>` to establish the canonical domain organization. Use `harnessctl create-artifact . --domain <domain> --type <type> --id <ID>` to create one incomplete draft from the installed template in its canonical type directory. Complete accountable fields and validate the graph before approval.

Canonical paths make repositories predictable, but paths never establish artifact identity, type, relations, lifecycle state, or authority. Existing valid flat layouts remain supported and may produce nonblocking migration guidance. Installation and upgrade never move repository-owned artifacts automatically.

## Start implementation

1. Select one bounded work order.
2. Run `harnessctl preflight . --work-order WO-...`.
3. Read every file in the returned manifest.
4. Inspect the affected implementation, tests, templates, and documentation.
5. Implement only the authorized scope and retain work-order-keyed evidence.

Preflight is read-only. It validates installed integrity, repository-context completeness, the formal graph, phase-appropriate work-order status, and the complete governing chain. It does not prove that the material was read or that a diff semantically matches the work order.

## Policy router

| Decision point | Managed policy |
| --- | --- |
| Creating or changing artifacts and lifecycle state | `docs/engineering/WORKFLOW.md` |
| Approving work, assurance, release, risk, or operations | `docs/engineering/DECISION_RIGHTS.md` |
| Defining or executing verification and release gates | `docs/engineering/QUALITY_GATES.md` |
| Defining artifact purpose, applicability, reuse, or relations; recording evidence, VRECs, supersession, or releases | `docs/engineering/TRACEABILITY.md` |

Use the repository-owned `docs/engineering/README.md` only as the index of local artifact domains and supporting engineering documentation.

## Review and visualization

Review readiness and visualization follow `docs/engineering/WORKFLOW.md`, subject to `QUALITY_GATES.md`. Preflight and Harness Explorer outputs are derived, read-only evidence; neither approves work nor verifies a candidate.

## Commit-bound verification and release

Verification and release follow `docs/engineering/WORKFLOW.md`, subject to `QUALITY_GATES.md`, `TRACEABILITY.md`, and `DECISION_RIGHTS.md`. VRECs and release records must identify the exact candidate commit they govern and therefore reside in later governance commits. Harness commands may prepare records, but never exercise accountable decision rights or commit, push, tag, release, publish, or deploy.

## Stop conditions

Stop and escalate when managed integrity fails, repository context is incomplete, the graph is invalid, no phase-eligible work order exists, the governing chain is incomplete, owner instructions materially conflict with this contract, required verification fails, or a requested action exceeds explicit authority.

> Approved intent and requirements define why work exists. Code and tests are evidence, not replacement product authority.
