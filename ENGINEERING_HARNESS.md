# Engineering Harness for Mokiterions

This repository uses SE Harness 0.2.1. This file is the single managed contract and router. Repository-owned instructions may add stricter local constraints, but they cannot waive this contract.

## Authority model

- `docs/engineering/REPOSITORY_CONTEXT.md` contains owner-curated repository facts and commands. It informs execution but grants no product or governance authority.
- Formal artifacts under `docs/engineering/` are the only repository-native source of product intent, requirements, architecture decisions, work authorization, verification contracts, and release constraints.
- Source, tests, conversations, dashboards, preflight, and CI are observations or evidence. They do not approve work, verification, or release.

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
| Creating relations, evidence, VRECs, supersession, or releases | `docs/engineering/TRACEABILITY.md` |

Use the repository-owned `docs/engineering/README.md` only as the index of local artifact domains and supporting engineering documentation.

## Review and visualization

Run `harnessctl preflight . --work-order WO-... --phase review` for a completed pull-request candidate. Generate Harness Explorer with `harnessctl dashboard .` and open `target/harness-dashboard/index.html`. Both outputs are derived, read-only evidence.

## Commit-bound verification and release

After a separately authorized candidate commit contains implementation and evidence, `harnessctl capture-verification` may prepare a `ready` VREC in a later governance commit. After accountable assurance review, `harnessctl prepare-release` may prepare a `ready` release record bound to the same candidate commit. These commands never commit, push, tag, approve, release, publish, or deploy.

## Stop conditions

Stop and escalate when managed integrity fails, repository context is incomplete, the graph is invalid, no phase-eligible work order exists, the governing chain is incomplete, owner instructions materially conflict with this contract, required verification fails, or a requested action exceeds explicit authority.

> Approved intent and requirements define why work exists. Code and tests are evidence, not replacement product authority.
