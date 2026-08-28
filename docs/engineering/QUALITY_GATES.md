# Quality Gates

## Normative language

Uppercase **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** have the meanings defined by BCP 14 (RFC 2119 and RFC 8174). Lowercase forms have their ordinary meaning.

## Purpose

A quality gate is a named predicate. A gate reports `pass`, `fail`, or `not_assessable`. It does not approve an artifact or exercise a decision right.

[`QUALITY_GATES.json`](QUALITY_GATES.json) owns gate IDs, checkpoint bindings,
predicate IDs, evaluator keys, and required evidence descriptors. This document
explains that executable contract and MUST NOT redefine it. The installed JSON
MUST be byte-identical to the packaged contract loaded by `harnessctl`.

**QG-001:** A workflow action that references a gate MUST proceed only when that gate reports `pass`.

**QG-002:** Missing, unreadable, or externally held required evidence MUST produce `not_assessable`; it MUST NOT be treated as `pass`.

**QG-003:** A failed or non-assessable required gate MUST leave lifecycle state unchanged.

**QG-009:** Predicate and gate status MUST aggregate in this order:
`fail` before `not_assessable` before `pass`. Every safely assessable predicate
MUST be reported even after another predicate fails.

**QG-010:** `harnessctl check` MUST evaluate gates at `start`, `pre-action`,
`transition`, and `handoff`. Transition planning and apply MUST evaluate, for
every transitioned artifact, the predicates the transition binding index binds
to that lifecycle edge through the same gate evaluator `check` uses, plus the
graph-structural checks it names, and MUST NOT write when any of them is not
`pass`. `harnessctl check --checkpoint transition --target STATE` renders the
same evaluation read-only. Preparation commands MUST run the same governed
checkpoint service before writing a VREC or RLS.

**QG-011:** A predicate MAY declare its own `checkpoints`; when it does not, it
inherits its gate's. A predicate whose inputs a transition does not receive
(the declared change set) is bound to `pre-action` and `handoff` only, so the
`handoff` checkpoint evaluates a superset of what the transition to
`implemented` evaluates, never a different set.

## Executable predicate registry

The evaluator key is closed. Policy files MUST NOT name an import, expression,
shell command, or repository-provided executable.

| Evaluator key | Exact assessment |
| --- | --- |
| `artifact_status` | Selected artifact status is one of the predicate's declared statuses. |
| `formal_graph_valid` | No blocking diagnostic affects the selected governing scope. |
| `repository_integrity` | No fixed repository-integrity blocker prevents reliable selected evaluation. |
| `execution_scope_declared` | The selected WO has one valid normalized `[execution_scope].paths` array. |
| `change_set_complete` | The caller explicitly asserted that the supplied changed-path set is complete. |
| `changed_paths_within_scope` | Every path in a complete declared change set matches one exact path or component-boundary directory prefix. |
| `start_preflight_ready` | Start preflight has no lifecycle-relevant blocker. |
| `review_preflight_ready` | Review preflight has no lifecycle-relevant blocker. |
| `review_evidence_available` | Work-order-keyed evidence names the selected artifact and checkpoint and binds the current formal-snapshot digest. |
| `authoring_ready` | The selected definition carries no template placeholder outside code and its `Open decisions` section, when present, reads `None`. Evaluated when a definition leaves `draft`. |
| `release_unit_ready` | A release contract that names a `candidate_commit` declares in `gates` exactly the work-order census derived from the `Harness-Work-Order` trailers between `previous_release_tag` and that commit; a contract without a candidate commit passes unmeasured. Evaluated when a release contract leaves `draft`. |

Missing completeness or required evidence is `not_assessable`, never `pass`.
Caller-declared completeness is retained evidence; it is not proof from a
trusted change baseline.

## Machine binding index

| Gate ID | Predicate IDs |
| --- | --- |
| `QG-G0-INTENT` | `QGP-G0-GRAPH`, `QGP-G0-INTEGRITY` |
| `QG-G1-DEFINITION` | `QGP-G1-GRAPH`, `QGP-G1-INTEGRITY`, `QGP-G1-AUTHORING` |
| `QG-G2-ARCHITECTURE` | `QGP-G2-GRAPH`, `QGP-G2-INTEGRITY`, `QGP-G2-AUTHORING` |
| `QG-G3-WORK-AUTHORIZATION` | `QGP-G3-STATUS`, `QGP-G3-GRAPH`, `QGP-G3-INTEGRITY`, `QGP-G3-SCOPE`, `QGP-G3-PREFLIGHT` |
| `QG-G4-IMPLEMENTATION-EVIDENCE` | `QGP-G4I-STATUS`, `QGP-G4I-GRAPH`, `QGP-G4I-INTEGRITY`, `QGP-G4I-SCOPE`, `QGP-G4I-COMPLETE`, `QGP-G4I-PATHS`, `QGP-G4I-PREFLIGHT`, `QGP-G4I-EVIDENCE` |
| `QG-G4-CANDIDATE-READY` | `QGP-G4C-STATUS`, `QGP-G4C-GRAPH`, `QGP-G4C-INTEGRITY` |
| `QG-G4-ASSURANCE-DECISION` | `QGP-G4A-GRAPH`, `QGP-G4A-INTEGRITY` |
| `QG-G4-VERIFIED-COVERAGE` | `QGP-G4V-GRAPH`, `QGP-G4V-INTEGRITY` |
| `QG-G5-RELEASE-PREPARATION` | `QGP-G5P-GRAPH`, `QGP-G5P-INTEGRITY`, `QGP-G5P-RELEASE-UNIT` |
| `QG-G5-RELEASE-DECISION` | `QGP-G5D-STATUS`, `QGP-G5D-GRAPH`, `QGP-G5D-INTEGRITY` |
| `QG-G5-EXTERNAL-ACTION` | `QGP-G5E-STATUS`, `QGP-G5E-GRAPH`, `QGP-G5E-INTEGRITY` |

## Transition binding index

Each lifecycle edge is bound to the predicates a transition evaluates and to the
graph-structural checks that stay in the evaluator. Contract loading fails with
`WEX-ECP-030` when an edge of the lifecycle registry has no binding.

| Family | Target | Predicate IDs | Structural checks |
| --- | --- | --- | --- |
| definition (intent, capability, requirement, verification, operating contract) | `approved` | `QGP-G1-GRAPH`, `QGP-G1-INTEGRITY`, `QGP-G1-AUTHORING` | `QGS-EDGE` |
| definition (specification, architecture, ADR) | `approved` | `QGP-G2-GRAPH`, `QGP-G2-INTEGRITY`, `QGP-G2-AUTHORING` | `QGS-EDGE` |
| definition (release contract) | `approved` | `QGP-G5P-GRAPH`, `QGP-G5P-INTEGRITY`, `QGP-G5P-RELEASE-UNIT` | `QGS-EDGE` |
| definition | `implemented`, `rejected` | none | `QGS-EDGE` |
| work order | `approved` | `QGP-G3-GRAPH`, `QGP-G3-INTEGRITY` | `QGS-EDGE`, `QGS-ASSURANCE` |
| work order | `in_progress` | `QGP-G3-STATUS`, `QGP-G3-GRAPH`, `QGP-G3-INTEGRITY`, `QGP-G3-SCOPE`, `QGP-G3-PREFLIGHT` | `QGS-EDGE` |
| work order | `implemented` | `QGP-G4I-STATUS`, `QGP-G4I-GRAPH`, `QGP-G4I-INTEGRITY`, `QGP-G4I-SCOPE`, `QGP-G4I-PREFLIGHT`, `QGP-G4I-EVIDENCE` | `QGS-EDGE` |
| work order | `verified` | `QGP-G4V-GRAPH`, `QGP-G4V-INTEGRITY` | `QGS-EDGE`, `QGS-VREC-COVERAGE` |
| work order | `released` | `QGP-G5D-GRAPH`, `QGP-G5D-INTEGRITY` | `QGS-EDGE`, `QGS-RLS-COVERAGE` |
| work order | `rejected` | none | `QGS-EDGE` |
| verification record | `verified` | `QGP-G4A-GRAPH`, `QGP-G4A-INTEGRITY` | `QGS-EDGE` |
| verification record | `superseded` | `QGP-G4A-GRAPH`, `QGP-G4A-INTEGRITY` | `QGS-EDGE`, `QGS-SUCCESSOR` |
| verification record | `rejected` | none | `QGS-EDGE` |
| release record | `released` | `QGP-G5D-STATUS`, `QGP-G5D-GRAPH`, `QGP-G5D-INTEGRITY` | `QGS-EDGE`, `QGS-VERIFIED-INCLUSION` |
| release record | `rejected` | none | `QGS-EDGE` |

At the `transition` checkpoint `review_evidence_available` accepts the
work-order evidence bound to the `handoff` checkpoint at the same formal
snapshot, so a transition never passes on weaker evidence than `check` saw.

### Graph-structural checks

Properties of the artifact graph shape alone, evaluated by the evaluator and
reported under the synthetic gate `QG-STRUCTURAL` so every refusal names its
check.

| Check | Exact assessment |
| --- | --- |
| `QGS-EDGE` | The source-to-target edge is declared in the lifecycle registry and permitted by the revision provenance policy. |
| `QGS-ASSURANCE` | A work order leaving `draft` classifies `commit_bound_verification` as `required` or `not_required`. |
| `QGS-VREC-COVERAGE` | A work order becoming `verified` is covered by a verified or released verification record. |
| `QGS-RLS-COVERAGE` | A work order becoming `released` is released by a released release record. |
| `QGS-VERIFIED-INCLUSION` | Every verification record a release record includes is verified before the record is released. |
| `QGS-SUCCESSOR` | A superseded verification record names a verified or released successor that preserves its work coverage. |

## Gate catalog

| ID | Evaluated when | Pass predicate | Required evidence | Failure result |
| --- | --- | --- | --- | --- |
| `QG-G0-INTENT` | Definition work is proposed | An approved INT defines the problem, outcome, scope boundary, and accountable product owner; approved CAP and REQ artifacts derive from it | INT, CAP, REQ metadata and bodies | Definition packet is not eligible for approval |
| `QG-G1-DEFINITION` | A definition packet or WO is reviewed | Every selected active REQ has selected active SPEC and VER coverage | Formal graph and selected packet | Packet or WO is not eligible |
| `QG-G2-ARCHITECTURE` | Architecture is applicable | Every selected ARCH identifies its architecturally significant requirement drivers and conforming specifications; its decision assessment is valid; each `adr_required` architecture has active deciding ADR coverage | ARCH, SPEC, REQ, ADR metadata and decision assessment | Architecture or WO is not eligible |
| `QG-G3-WORK-AUTHORIZATION` | Implementation start is requested | One approved WO selects the complete applicable chain, declares assurance applicability, passes start preflight, and has no scoped or repository blocker | WO, reading manifest, start-preflight result | Implementation MUST NOT start |
| `QG-G4-IMPLEMENTATION-EVIDENCE` | WO completion is requested | Approved scope is implemented; required checks pass; retained evidence identifies the WO; review preflight passes; no scoped or repository blocker remains | Diff review, commands and results, evidence path, review-preflight result | WO MUST remain `in_progress` or be explicitly rejected |
| `QG-G4-CANDIDATE-READY` | VREC preparation is requested | Selected WO set is `implemented`; declared VER coverage and retained evidence are exact; candidate commit is clean and immutable for the record | Commit identity, worktree state, WO set, VER set, evidence paths, snapshot | No VREC is written |
| `QG-G4-ASSURANCE-DECISION` | A ready VREC is reviewed | Candidate identity, selected work, verification contracts, retained evidence, required tests, and unresolved findings have been assessed against the VER pass criteria | Ready VREC and retained evidence | Assurance owner MUST reject, request remediation, or leave the VREC ready |
| `QG-G4-VERIFIED-COVERAGE` | Delivery is considered | At least one eligible verified or released VREC covers the exact intended work and candidate | VREC relations and commit identity | Integration or release preparation MUST NOT claim verified coverage |
| `QG-G5-RELEASE-PREPARATION` | RLS preparation is requested | All included VRECs are eligible and verified; released-work equality, release-contract coverage, and commit identity equality pass | REL, VRECs, WOs, version and commit inputs | No RLS is written |
| `QG-G5-RELEASE-DECISION` | A ready RLS is reviewed | Release scope, verified coverage, version, rollback conditions, and release evidence satisfy the REL contract | Ready RLS, REL and retained release evidence | Release owner MUST reject, request remediation, or leave the RLS ready |
| `QG-G5-EXTERNAL-ACTION` | A tag, merge, publication, deployment, or operation is proposed | The exact action, target, accountable owner, prerequisites, and recovery conditions are explicit | Action-specific request and evidence | The external action MUST NOT occur |

The G0-G5 labels group related gates for reporting. They MUST NOT replace the exact gate IDs above.

## Validation assessment planes

| Plane | Meaning | Blocking rule |
| --- | --- | --- |
| `structure` | Formal syntax, identity, type, and graph shape | Error severity blocks; warning severity does not |
| `governance` | Non-waivable lifecycle, assurance, and provenance invariants | Error severity blocks |
| `policy` | A rule activated by explicit repository configuration | Configured error severity blocks |
| `maintenance` | Compatibility or placement advice | Non-blocking unless another exact rule raises an error |

**QG-004:** A diagnostic plane MUST NOT change error-versus-warning severity, lifecycle authority, or process exit behavior.

**QG-005:** Gates MUST NOT be replaced by an aggregate health score.

## Exceptions

**QG-006:** A `MUST` gate has no implicit exception.

**QG-007:** A documented `SHOULD` deviation MUST state the rule ID, reason, impact, accountable owner, and compensating evidence.

**QG-008:** A tool MUST report the exact failed predicate. It MUST NOT report only a generic message such as "quality is insufficient."

Workflow actions reference these gate IDs from [WORKFLOW.json](WORKFLOW.json). Decision ownership is defined by [DECISION_RIGHTS.md](DECISION_RIGHTS.md).
