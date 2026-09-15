# Decision Rights

## Normative language

Uppercase **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** have the meanings defined by BCP 14 (RFC 2119 and RFC 8174). Lowercase forms have their ordinary meaning.

## Purpose

This document defines who is accountable for each governance decision. It does not define workflow order, quality predicates, or artifact relations.

## Roles and accountabilities

| Role | Accountable for | Not granted by the role |
| --- | --- | --- |
| Product or domain owner | Intent, capabilities, observable requirements, and product priority | Architecture acceptance, assurance, release, or production operation |
| Technical owner | Specifications, architecture, decision applicability, and ADR acceptance | Product priority, assurance, or release |
| Engineering owner | Approving work-order scope and changes to it | Verification or release of the resulting candidate |
| Executor (person or agent) | Starting, implementing, checking and recording approved work; preparing required verification | Scope approval, assurance, release or external delivery |
| Assurance owner | Verification contracts, evidence assessment, VREC verification, rejection, and supersession | Product scope, implementation scope, or release |
| Repository owner | Repository integration actions such as pull-request creation or merge | Verification, release, publication, deployment, or operation |
| Release owner | Release preparation inputs and the RLS release decision | Repository integration, publication, deployment, or operation |
| Service owner | Operating contracts and production operation | Product definition, verification, or release |

Holding one role does not grant another role. One person MAY hold several roles when repository policy permits it, but each decision MUST identify the role exercised.

## Decision-right catalog

| ID | Decision or action | Accountable role | Required input | Recorded result |
| --- | --- | --- | --- | --- |
| `DR-DEFINITION-DECIDE` | Approve or reject an intent, capability, requirement, specification, architecture, ADR, verification contract, release contract, or operating contract | Owner named for that artifact type | Complete draft and applicable gates | Explicit lifecycle decision on the selected artifact |
| `DR-WO-SELECT` | Approve or reject one bounded work order | Engineering owner | Complete governing chain and assurance classification | Explicit work-order decision |
| `DR-WO-START` | Start approved implementation | Executor under WO approval | Recorded scope approval and passing `QG-G3-WORK-AUTHORIZATION` | Selected WO becomes `in_progress` |
| `DR-WO-COMPLETE` | Record implementation completion | Executor under WO approval | Passing `QG-G4-IMPLEMENTATION-EVIDENCE` and unchanged approved scope | Selected WO becomes `implemented` |
| `DR-VREC-PREPARE` | Prepare a ready VREC | Executor under each selected WO approval | Passing `QG-G4-CANDIDATE-READY` and approved inputs | One ready VREC; no assurance decision |
| `DR-VREC-DECIDE` | Verify, reject, or supersede a ready VREC | Assurance owner | Ready VREC and passing `QG-G4-ASSURANCE-DECISION` | Decision on the selected VREC only |
| `DR-DELIVERY-SELECT` | Select repository integration or release preparation | Repository owner or release owner for the chosen path | Verified coverage | Explicit path-specific instruction |
| `DR-RLS-PREPARE` | Prepare a ready RLS | Release owner | Passing `QG-G5-RELEASE-PREPARATION` and exact release inputs | One ready RLS; no release decision |
| `DR-RLS-DECIDE` | Release or reject a ready RLS | Release owner | Passing `QG-G5-RELEASE-DECISION` | Decision on the selected RLS only |
| `DR-EXTERNAL-ACTION` | Tag, publish, deploy, operate, merge, or perform another external action | Owner accountable for that exact action | Exact target and applicable gate evidence | Explicit authorization limited to the stated action |
| `DR-RELATED-RECORD-SELECT` | Select a related VREC, RLS, or WO for inspection | No decision authority required | Exact artifact ID | Read-only selection |
| `DR-REMEDIATION-SCOPE` | Authorize new or revised remediation scope | Owner of the affected definition or work | Failed criterion and proposed bounded remediation | New or revised governed scope |
| `DR-DECISION-DISPOSE` | Decide, defer, or withdraw a pending decision | Owner named on the artifact the decision blocks (the engineering owner for a work order); for a deviation, the owner named on the specification it departs from | Open or deferred `DEC-` and one declared option, with a revisit trigger for a deferral or an accepted deviation | The option, its label, the role, the time and the verbatim reason recorded as the decision's disposition; the blocked artifacts unchanged |

## Explicit decisions

**DR-001:** A lifecycle decision MUST identify the selected artifact, target state, accountable actor, and decision meaning.

**DR-002:** Approval of one artifact MUST NOT be interpreted as approval of another artifact.

**DR-003:** Authority for one action MUST NOT be inferred from an earlier or later action. Work-order approval expressly grants the execution operations listed in DR-015; using that grant requires no additional owner decision.

**DR-004:** Silence, tool execution, a passing check, a commit, a pull request, or elapsed time MUST NOT count as an accountable decision.

**DR-005:** Automation MAY prepare a decision candidate, but it MUST NOT claim that an accountable owner made the decision.

**DR-006:** A rejection MUST include a non-empty reason. A supersession MUST identify exactly one eligible successor.

## Delegation and separation

**DR-007:** Delegation MUST be explicit, limited to named decision rights, and recorded before the delegated decision.

**DR-008:** A delegate MUST use its own identity and MUST NOT impersonate the delegating owner.

**DR-009:** Repository policy MAY require role separation. When it does, one actor MUST NOT exercise conflicting rights for the same candidate.

**DR-010:** When the required accountable role is absent or ambiguous, the workflow MUST stop and request that exact role.

**DR-011:** An actor MUST NOT approve its own architecture decision assessment unless it explicitly holds the technical-owner role for that artifact. Drafting the assessment grants no approval right.

## Preparation is not a decision

**DR-012:** Preparing a VREC or RLS MUST record preparation provenance and MUST NOT record the later accountable decision actor.

**DR-013:** Verifying a VREC MUST change only that VREC. It MUST NOT change a referenced WO or RLS.

**DR-014:** Releasing an RLS MUST change only that RLS. It MUST NOT change an included VREC or WO.

## Approved execution

**DR-015:** Approving a work order authorizes execution of its approved scope,
including start, implementation, local edits and commits, required checks,
evidence capture, completion recording and required verification-record
preparation. The executor MUST use the same local approval, scope and evidence
checks whether it is a person or an agent. It continues covered work without
renewed permission for these operations while the approved scope and applicable
conditions remain satisfied. It MUST record work actually performed under its
own identity. Approval leaves the WO approved until start is applied.

This is the only execution procedure. New work orders need no delegation
table, route setting or separate execution approval. Selecting and scheduling
work remain explicit; approval does not start every approved work order.
Verification preparation follows the approved assurance classification and
checks each explicitly selected WO. A preliminary merge and live CI are not
local authorization requirements. CI still applies at integration/publication.

Scope changes, acceptance of the result and external delivery remain with the
accountable owners unless the specific delivery action is already authorized.
Reuse that authorization without a duplicate request. The executor role grants
no definition approval, assurance, release, merge or publication rights.
Project-required role separation applies within this same procedure.

Historical events keep their original meaning. New approval events record
scope without the former delegation-class field. Existing explicit execution
grants remain usable under the same scope checks. An older approval without
that grant requires owner approval of remaining execution through the existing
amendment process. Neither current metadata nor an actor label can manufacture
a missing historical grant; historical records are not rewritten.

Workflow order is defined by [WORKFLOW.md](WORKFLOW.md). Gate predicates are defined by [QUALITY_GATES.md](QUALITY_GATES.md). Artifact relations are defined by [TRACEABILITY.md](TRACEABILITY.md).
