# Workflow

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**,
**SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and
**OPTIONAL** in this document are to be interpreted as described in BCP 14
(RFC 2119 and RFC 8174) when, and only when, they appear in all capitals.

This document defines the procedure bound to the executable workflow contract.
[`WORKFLOW.json`](WORKFLOW.json) defines each artifact family's admitted states,
permitted transitions, authority effect, release-version reservation,
transitionability, visibility, and predecessor-adapter need. It also defines
ordered next-action rules, typed procedures, effects, non-effects, gate IDs,
and decision-right IDs. The installed `WORKFLOW.json` MUST be byte-identical to
the contract loaded by `harnessctl`. Command steps in that contract use argument
arrays. The array, not a displayed shell string, is authoritative.

## Workflow authority

`WFL-001` - A lifecycle transition MUST appear in `WORKFLOW.json`, pass every
gate named by the selected workflow rule, and be exercised by the role named by
its decision-right ID. Prose, adjacency, a passed command, or the status of a
related artifact MUST NOT authorize a transition.

`WFL-002` - A transition changes only the artifacts explicitly selected by the
actor. A VREC decision MUST NOT change a referenced work order. An RLS decision
MUST NOT change an included VREC or work order. A work-order decision MUST NOT
change its definitions, VRECs, or RLS records.

`WFL-003` - `harnessctl focus` and `harnessctl transition` MUST select the first
matching recommendation in the ordered `recommendations` array. They MUST NOT
invent, merge, or skip recommendations.

`WFL-004` - A command that prepares, validates, inspects, captures, or renders
information MUST NOT exercise a decision right. Only an explicit actor decision
and an applied permitted transition change lifecycle state.

`WFL-005` - `rejected` is terminal. `superseded` is terminal and applies only
to a ready VREC with one eligible successor. Historical lifecycle events MUST
remain append-only.

Managed-file integrity uses schema-3 SHA-256 over the versioned `utf8-text-lf-v1` representation and binds the installed released-evaluator payload plus its archive when available. LF, CRLF, and CR are equivalent line terminators; all other content distinctions remain significant. Schema-1 and schema-2 locks remain readable, but ordinary mutation requires a schema-3 evaluator match; older locks migrate only through a separately reviewed upgrade whose target evaluator is installed from already-published wheel bytes. `doctor` and mutation plans are read-only, and customized, ambiguous, or identity-mismatched content is never overwritten.

Lifecycle transition apply, non-dry-run domain and artifact authoring, renumber apply, verification capture, and release preparation all acquire the same evaluator authority before writing. Verification capture retains canonical normalized evaluator evidence and binds its path and SHA-256 in the ready VREC. Release preparation repeats that observation, requires the locked wheel name and digest, and binds it in the ready RLS. Changing, removing, or substituting those evidence bytes invalidates the record; the evidence is technical provenance, not an assurance or release decision.

## Delegated Phase 4 operations

`WORKFLOW.json` schema v4 defines the complete delegated operation catalog.
Absence from this table denies an operation; a prior receipt creates no
standing authority. Every row requires current formal delegation, the exact
released evaluator, a fresh stable observation, a unique admitted nonce, the
named passing gates, one logical `implementation-worker`, the `implementer`
profile, and no recovery-required state.

| Operation | Decision right | Current WO state | Result |
| --- | --- | --- | --- |
| `delegated-work-order-start` | `DR-WO-START` | `approved` | Existing legal transition to `in_progress` plus a start receipt |
| `change-bundle-apply` | Started-work execution; no additional right | `in_progress` | Brokered target effect plus an effect receipt |
| `delegated-work-order-complete` | `DR-WO-COMPLETE` | `in_progress` | Existing legal transition to `implemented` plus a completion receipt |
| `delegated-vrec-prepare` | `DR-VREC-PREPARE` | `implemented` | One undecided ready VREC plus an assurance decision packet |

Delegated completion MUST prove an uninterrupted start/effect state chain,
exact admitted and final changed paths, successful required tests and gates,
retained evidence digests, explicit deviations, explicit residual uncertainty,
and no active effect journal. Missing or not-assessable proof MUST NOT be
treated as pass. Verification preparation MUST stop before Git when a required
candidate commit is absent. `PROC-CANDIDATE-COMMIT` binds that stop to
`STEP-CANDIDATE-COMMIT-AUTHORIZE`; its response requests the exact repository-
owner action and performs no staging, commit, branch, push, merge, assurance,
release, credential, network, or external effect.

## State model

The `lifecycles` object in `WORKFLOW.json` is the single machine-readable state
registry. Transition planning and graph validation MUST derive their state
vocabularies and transition edges from it. Authority-sensitive checks MUST read
`grants_authority`; release-version uniqueness MUST read `reserves_version`.
Consumers MUST fail closed on an invalid registry and MUST NOT substitute a
fallback status set.

The permitted transitions are:

| Artifact class | From | To |
| --- | --- | --- |
| Definition | `draft` | `approved`, `rejected` |
| Definition | `approved` | `implemented`, `rejected` |
| Work order | `draft` | `approved`, `rejected` |
| Work order | `approved` | `in_progress`, `rejected` |
| Work order | `in_progress` | `implemented`, `rejected` |
| Work order | `implemented` | `verified`, `released` |
| Work order | `verified` | `released` |
| Verification record | `ready` | `verified`, `rejected`, `superseded` |
| Release record | `ready` | `released`, `rejected` |

Rows without a listed outgoing transition are terminal. All lifecycle rows are
historically visible. Rejected VREC and RLS rows grant no authority, reserve no
version, and declare that an explicit predecessor adapter is required when an
older evaluator cannot understand them. This marker reports a compatibility
fact; it does not hide history, create a view, or upgrade the predecessor.

Terminal compatibility vocabulary also includes definition `ready`,
`in_progress`, `verified`, `released`, and `superseded`, and work-order `ready`
and `superseded`. These rows preserve readable history and existing validation
fixtures; they do not add a transition. Definition `in_progress`, `verified`,
and `released` preserve their historical authority effect. The other four rows
grant no authority. None reserves a release version.

The JSON contract is authoritative if this summary and the contract differ.
Conformance tests MUST fail on such a difference.

## End-to-end procedure

1. The author MUST use
   `harnessctl scaffold-domain . --domain <lowercase-kebab-domain>` once for a
   new domain, then
   `harnessctl create-artifact . --domain <domain> --type <type> --id <ID>` for
   each new draft. The author MUST complete accountable fields and validate the
   graph before requesting a decision.
2. The product or domain owner MUST approve the intent, capabilities, and
   requirements after `QG-G0-INTENT` and `QG-G1-DEFINITION` pass.
3. The technical owner MUST approve specifications and architecture after
   `QG-G1-DEFINITION` and `QG-G2-ARCHITECTURE` pass. Each architecture MUST
   declare its decision applicability. ADR count follows coherent significant
   decisions; no one-ADR-per-requirement rule exists.
4. The assurance owner MUST approve the verification contracts. The engineering
   owner MAY then approve one bounded work order after
   `QG-G3-WORK-AUTHORIZATION` passes.
5. Before implementation, the implementation actor MUST run
   `harnessctl focus . --artifact WO-...` and
   `harnessctl preflight . --work-order WO-... --phase start`, read every file
   in the manifest, and receive an explicit start decision.
6. The implementation actor MUST change only the authorized scope, retain
   work-order-keyed evidence, and run
   `harnessctl preflight . --work-order WO-... --phase review` plus the
   repository checks named by the owner-controlled region of `AGENTS.md`.
7. The engineering owner MAY mark only that work order `implemented` after
   `QG-G4-IMPLEMENTATION-EVIDENCE` passes.
8. When commit-bound verification is `required`, the authorized actor MUST
   commit the clean candidate before preparing a VREC. `capture-verification`
   MUST bind that exact candidate, work-order set, verification-contract set,
   and retained evidence. The resulting VREC remains `ready`.
9. The assurance owner MUST decide the VREC independently. A verified VREC does
   not change its work orders. Repository integration and release preparation
   are separate paths with separate authority.
10. Release preparation MUST use eligible verified coverage at one exact
   candidate commit. The release owner MUST decide the ready RLS independently.
   A released RLS does not tag, publish, deploy, or operate anything.
11. Each external action MUST receive its own explicit authority after
    `QG-G5-EXTERNAL-ACTION` passes.

## Bound procedures

Each row names its exact procedure in `WORKFLOW.json`. `harnessctl check`
resolves the first matching workflow row and that procedure. An actor MUST NOT
replace a procedure with an unbound instruction such as "run preflight" or
"use exact inputs." "Result" states the only permitted lifecycle effect; the
contract's `non_effects` remain mandatory.

| Workflow ID | When | Gate / decision right | Procedure ID | Result |
| --- | --- | --- | --- | --- |
| `WFL-WO-READY-VREC` | Focused WO is `implemented`; a related VREC is `ready`. | `QG-G4-ASSURANCE-DECISION` / `DR-VREC-DECIDE` | `PROC-FOCUS-RELATED` | Focus the ready VREC. The assurance owner decides only that VREC; the WO remains `implemented`. |
| `WFL-WO-VERIFIED-VREC` | Focused WO is `implemented`; a related VREC is `verified` or `released`. | `QG-G4-VERIFIED-COVERAGE` / `DR-DELIVERY-SELECT` | `PROC-DELIVERY-SELECT` | Selection changes neither record. Complete alternatives are `PROC-REPOSITORY-INTEGRATION` and `PROC-PREPARE-RELEASE`. |
| `WFL-WO-PREPARE-VREC` | Focused WO is `implemented`; no ready, verified, or released VREC covers it. | `QG-G4-CANDIDATE-READY` / `DR-VREC-PREPARE` | `PROC-WO-PREPARE-VREC` | Create one ready VREC; do not change or verify the WO. |
| `WFL-WO-START` | Focused WO is `approved`. | `QG-G3-WORK-AUTHORIZATION` / `DR-WO-START` | `PROC-WO-START` | Execute the six ordered typed steps. Only the selected WO may become `in_progress`. |
| `WFL-WO-IMPLEMENT` | Focused WO is `in_progress`. | `QG-G4-IMPLEMENTATION-EVIDENCE` / `DR-WO-COMPLETE` | `PROC-WO-IMPLEMENT` | Completion changes only the WO to `implemented`; it does not verify work. |
| `WFL-WO-COMPLETED` | Focused WO is `verified` or `released`. | No gate / `DR-RELATED-RECORD-SELECT` | `PROC-FOCUS-SELECTED` | Projection changes nothing. |
| `WFL-VREC-DECIDE` | Focused VREC is `ready`. | `QG-G4-ASSURANCE-DECISION` / `DR-VREC-DECIDE` | `PROC-VREC-DECIDE` | Change only the VREC. Complete alternatives are `PROC-VREC-REJECT` and `PROC-VREC-SUPERSEDE`. |
| `WFL-VREC-DELIVER` | Focused VREC is `verified` or `released`. | `QG-G4-VERIFIED-COVERAGE` / `DR-DELIVERY-SELECT` | `PROC-DELIVERY-SELECT` | Selection changes nothing. `PROC-REPOSITORY-INTEGRATION` is a complete alternative. |
| `WFL-RLS-DECIDE` | Focused RLS is `ready`. | `QG-G5-RELEASE-DECISION` / `DR-RLS-DECIDE` | `PROC-RLS-DECIDE` | Change only the RLS. `PROC-RLS-REJECT` is a complete alternative. |
| `WFL-RLS-EXTERNAL` | Focused RLS is `released`. | `QG-G5-EXTERNAL-ACTION` / `DR-EXTERNAL-ACTION` | `PROC-EXTERNAL-ACTION` | Release status performs no external action. |
| `WFL-REJECTED` | Focused artifact is `rejected`. | No gate / `DR-REMEDIATION-SCOPE` | `PROC-REMEDIATE` | Preserve rejected history; remediation does not expand selected scope. |
| `WFL-VREC-SUPERSEDED` | Focused VREC is `superseded`. | No gate / `DR-RELATED-RECORD-SELECT` | `PROC-FOCUS-SELECTED` | Preserve the old VREC as release-ineligible history. |
| `WFL-DEFINITION-COMPLETE` | Focused definition is `approved`. | `QG-G1-DEFINITION`, `QG-G2-ARCHITECTURE` / `DR-DEFINITION-DECIDE` | `PROC-DEFINITION-COMPLETE` | Change only the explicitly selected definition. |
| `WFL-DEFINITION-WORK` | Focused definition is `implemented`. | `QG-G3-WORK-AUTHORIZATION` / `DR-WO-SELECT` | `PROC-DEFINITION-WORK` | Selecting work changes no lifecycle state. |
| `WFL-DEFAULT-REVIEW` | No earlier rule matches. | No gate / `DR-RELATED-RECORD-SELECT` | `PROC-FOCUS-SELECTED` | Report current state; change nothing. |
| `WFL-FAIL-REMEDIATE` | A workflow command fails. | No gate / `DR-REMEDIATION-SCOPE` | `PROC-REMEDIATE` | Report the exact blocker and unchanged state. |

## Ordered procedure registry

The arrays below are a human index. `WORKFLOW.json` remains authoritative for
argument boundaries, gate IDs, effects, non-effects, decision roles, permitted
outcomes, and response values.

| Procedure ID | Ordered typed steps |
| --- | --- |
| `PROC-WO-START` | `STEP-WO-START-FOCUS` command `harnessctl focus . --artifact {artifact_id}`; `STEP-WO-START-PREFLIGHT` command `harnessctl preflight . --work-order {artifact_id} --phase start`; `STEP-WO-START-DECIDE` decision `DR-WO-START`; `STEP-WO-START-PREVIEW` transition-preview command; `STEP-WO-START-APPLY` transition-apply command; `STEP-WO-START-FINAL-FOCUS` command `harnessctl focus . --artifact {artifact_id}`. |
| `PROC-WO-IMPLEMENT` | `STEP-WO-IMPLEMENT-CHECK` command `harnessctl check . --artifact {artifact_id} --checkpoint handoff`; `STEP-WO-IMPLEMENT-DECIDE` decision `DR-WO-COMPLETE`. |
| `PROC-WO-PREPARE-VREC` | `STEP-WO-PREPARE-VREC-DECIDE` decision `DR-VREC-PREPARE`. |
| `PROC-CANDIDATE-COMMIT` | `STEP-CANDIDATE-COMMIT-AUTHORIZE` decision `DR-EXTERNAL-ACTION`; request exact candidate-commit authority and perform no Git action. |
| `PROC-FOCUS-SELECTED` | `STEP-FOCUS-SELECTED` command `harnessctl focus . --artifact {artifact_id}`. |
| `PROC-FOCUS-RELATED` | `STEP-FOCUS-RELATED` command `harnessctl focus . --artifact {related_id}`. |
| `PROC-VREC-DECIDE` | `STEP-VREC-DECIDE` decision `DR-VREC-DECIDE`. |
| `PROC-VREC-REJECT` | `STEP-VREC-REJECT` decision `DR-VREC-DECIDE`. |
| `PROC-VREC-SUPERSEDE` | `STEP-VREC-SUPERSEDE` decision `DR-VREC-DECIDE`. |
| `PROC-DELIVERY-SELECT` | `STEP-DELIVERY-SELECT` decision `DR-DELIVERY-SELECT`. |
| `PROC-REPOSITORY-INTEGRATION` | `STEP-REPOSITORY-INTEGRATION` decision `DR-DELIVERY-SELECT`. |
| `PROC-PREPARE-RELEASE` | `STEP-PREPARE-RELEASE` decision `DR-DELIVERY-SELECT`. |
| `PROC-RLS-DECIDE` | `STEP-RLS-DECIDE` decision `DR-RLS-DECIDE`. |
| `PROC-RLS-REJECT` | `STEP-RLS-REJECT` decision `DR-RLS-DECIDE`. |
| `PROC-EXTERNAL-ACTION` | `STEP-EXTERNAL-ACTION` decision `DR-EXTERNAL-ACTION`. |
| `PROC-REMEDIATE` | `STEP-REMEDIATE-FOCUS` command `harnessctl focus . --artifact {artifact_id}`. |
| `PROC-DEFINITION-COMPLETE` | `STEP-DEFINITION-COMPLETE` decision `DR-DEFINITION-DECIDE`. |
| `PROC-DEFINITION-WORK` | `STEP-DEFINITION-WORK` decision `DR-WO-SELECT`. |

A command step that names gates also declares one `corrective` form per
predicate of those gates: a command argument array that differs from the
evaluated command, an escalation naming a decision right, or a response. When
`harnessctl check` is blocked, it renders the corrective form of the first
failing predicate under `Next` and `Command or response`. A contract whose
corrective form repeats the evaluated command fails to load with `WEX-ADS-001`.

## Transition procedure

For an accountable lifecycle decision:

1. Run `harnessctl focus . --artifact <ID>` and read the current state,
   recommendation, required authority, command or response, and alternatives.
2. Identify the matching ordered workflow rule in `WORKFLOW.json`. Verify its
   named gates pass and the actor holds its named decision right.
3. Obtain a statement naming the artifact, target state, and actor. Rejection
   MUST include a reason. VREC supersession MUST name the successor in `--reason`.
4. Preview the exact transition without `--apply`:
   `harnessctl transition . --set <ID>=<state> --decision <ID>=<actor>`.
5. Compare the preview to the explicit decision. Apply the same command with
   `--apply` only when they match.
6. Run `harnessctl focus . --artifact <ID>` again and report the resulting
   handoff. Do not transition a related artifact unless the actor separately
   selected and authorized it.

Use one transition packet when several definitions are mutually dependent and
each transition is explicitly named. A packet MUST NOT infer an omitted target.

## Lifecycle handoff procedure

After a completed stage or a stop condition, the agent MUST obtain the selected
workflow result from `harnessctl check` or another workflow command; every
workflow command emits the one schema-2 result. The structured result is authoritative for the outcome,
selected scope, effects, non-effects, blockers, final lifecycle state,
accountable decision, declared alternatives, and next procedure step. The
agent MUST NOT recompute or replace those values.

For an adaptive human handoff, the agent SHOULD lead with the outcome or the
decision the user needs to make. It MAY adapt wording, order, and headings; add
relevant explanation; merge fields whose separate meanings remain clear; and
omit empty fields. It MUST:

1. use actual artifact IDs and state whether the selected operation completed
   or blocked;
2. distinguish observed effects from incomplete expected effects;
3. preserve every exact blocker and every material non-effect whose omission
   could imply approval, transition, verification, release, Git, publication,
   deployment, operation, or another external effect;
4. state the final lifecycle state and identify the accountable role and exact
   decision when one is required;
5. recommend exactly one current typed procedure step;
6. preserve command argument values and boundaries or the operative meaning of
   a suggested response; and
7. present only workflow-declared complete alternatives, separately from the
   primary recommendation.

The agent MUST NOT invent an effect, authority, blocker, decision, alternative,
or next action; add a repository-wide finding to the selected result; ask an
open-ended question instead of presenting the selected recommendation; or turn
an alternative into a second next action.

When exact headings, field order, whitespace, or bytes are required, the
application or automation MUST invoke the deterministic schema-2 human renderer
directly and use its output unchanged. Model transcription MUST NOT be used for
exact rendering. The direct renderer's existing headings and empty-value rules
remain its contract; they do not constrain an adaptive agent handoff.

## Failure procedure

On a failed command, failed gate, invalid graph, missing authority, or scope
conflict, the agent MUST stop before the transition, report the exact failing
criterion and unchanged state, and recommend one safe retry or one accountable
escalation. The retry is the corrective form `harnessctl check` renders for the
first failing predicate; rerunning the evaluated command unchanged is never the
retry. Remediation that changes scope, accepts risk, or exercises a reserved
decision right requires a new explicit decision.

A record cannot contain the hash of its own commit. A VREC or RLS therefore
resides in a governance commit after the exact candidate commit it governs.
