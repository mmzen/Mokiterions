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

`WFL-003` - `harnessctl check` and `harnessctl transition` MUST select the first
matching recommendation in the ordered `recommendations` array. They MUST NOT
invent, merge, or skip recommendations.

`WFL-004` - A command that prepares, validates, inspects, captures, or renders
information MUST NOT exercise a decision right. Only an explicit actor decision
and an applied permitted transition change lifecycle state.

`WFL-005` - `rejected` is terminal. `superseded` is terminal and applies only
to a ready VREC with one eligible successor. Historical lifecycle events MUST
remain append-only.

Managed-file integrity uses SHA-256 over the versioned `utf8-text-lf-v1` representation and binds the installed released-evaluator payload plus its archive when available. LF, CRLF, and CR are equivalent line terminators; all other content distinctions remain significant. Schema 3 is the default repository format. Schema 4 selects plugin skills with a portable provider record under SPEC-PLG-021. Skill replacement is file-only cleanup with atomic lock saving and ordinary retries; it grants no lifecycle authority. Schemas 1 and 2 are refused before writes. Ordinary mutation requires the exact evaluator identity bound by the selected supported lock. `doctor` and mutation plans are read-only, and customized, ambiguous, or identity-mismatched content is never overwritten.

Lifecycle transition apply, non-dry-run domain and artifact authoring, verification capture, and release preparation all acquire the same evaluator authority before writing. Verification capture retains canonical normalized evaluator evidence and binds its path and SHA-256 in the ready VREC. Release preparation repeats that observation, requires the locked wheel name and digest, and binds it in the ready RLS. Changing, removing, or substituting those evidence bytes invalidates the record; the evidence is technical provenance, not an assurance or release decision.

## Approved execution

Work-order approval grants the execution operations in
`DECISION_RIGHTS.md#approved-execution`. A person or agent follows the same
procedure with the same local approval, scope and evidence checks. The existing
operation identifiers below remain stable; they do not select different routes.

| Operation | Decision right | Current WO state | Result |
| --- | --- | --- | --- |
| `delegated-work-order-start` | `DR-WO-START` | `approved` | Executor applies the selected transition to `in_progress`. |
| `delegated-work-order-complete` | `DR-WO-COMPLETE` | `in_progress` | Executor records `implemented` after the required handoff checks. |
| `delegated-vrec-prepare` | `DR-VREC-PREPARE` | `implemented` | Executor prepares one ready VREC from the explicitly selected WO set. |

`harnessctl check` returns the next procedure command directly. It does not
request another owner start, completion or preparation decision. Commands still
check actual approval and local gates before writing. A missing grant, changed
scope or failed check blocks the affected operation; there is no owner-route
bypass. CI is assessed separately at integration and publication.

For an approved selected WO, run start preflight, preview the start transition,
apply it, then inspect the resulting state. For completion, run handoff checks,
preview the completion transition and apply it. Prepare required verification
using the returned capture command with inspected record ID, verification and
evidence inputs. Substitute the actual executor identity for the example
`delegated-executor` actor when appropriate; identity never changes the checks.

An implemented WO classified `not_required` needs no new VREC. Execution is
complete; report that result and follow any already authorized delivery. Do not
interpret the read-only completion projection as another execution loop.
Existing explicitly prepared records remain eligible for their owner decisions.

Verification capture hashes selected artifacts directly. It does not build a
dashboard. To capture a committed candidate while keeping local edits, use
`capture-verification --candidate-commit <commit> --test-command <executable> <args>`.
The test command runs without a shell in a temporary checkout; a failed command
or tracked test mutation prevents the record. The record keeps its actual commit
and test result, and the temporary checkout is removed.

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
| Decision | `open` | `decided`, `deferred`, `withdrawn` |
| Decision | `deferred` | `decided`, `withdrawn` |
| Risk | `identified` | `raised`, `withdrawn` |
| Risk | `raised` | `accepted`, `avoided`, `mitigating`, `withdrawn` |
| Risk | `mitigating` | `mitigated`, `withdrawn` |

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
   requirements after `QG-G1-DEFINITION` passes.
3. The technical owner MUST approve specifications and architecture after
   `QG-G1-DEFINITION` and `QG-G2-ARCHITECTURE` pass. Each architecture MUST
   declare its decision applicability. ADR count follows coherent significant
   decisions; no one-ADR-per-requirement rule exists.
4. The assurance owner MUST approve the verification contracts. The engineering
   owner MAY then approve one bounded work order after
   `QG-G3-WORK-AUTHORIZATION` passes.
5. Before implementation, the implementation actor MUST run
   `harnessctl check . --artifact WO-...` (which composes
   `harnessctl preflight . --work-order WO-... --phase start` into its
   reading manifest), read every file in that manifest, and receive an
   explicit start decision.
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

Use `harnessctl check . --artifact ARTIFACT-ID` to see the next action. Checks,
including `--checkpoint pre-action`, select their procedure automatically.
`--procedure` is optional and selects only a declared alternative.
A work order may list ordinary evidence files in its top-level `evidence_paths`;
those files need no machine header and must exist inside the repository.
Generated evidence headers remain supported, including extra descriptive text fields.

Each row names its exact procedure in `WORKFLOW.json`. `harnessctl check`
resolves the first matching workflow row and that procedure. An actor MUST NOT
replace a procedure with an unbound instruction such as "run preflight" or
"use exact inputs." "Result" states the only permitted lifecycle effect; the
contract's `non_effects` remain mandatory.

| Workflow ID | When | Gate / decision right | Procedure ID | Result |
| --- | --- | --- | --- | --- |
| `WFL-WO-READY-VREC` | Focused WO is `implemented`; a related VREC is `ready`. | `QG-G4-ASSURANCE-DECISION` / `DR-VREC-DECIDE` | `PROC-FOCUS-RELATED` | Focus the ready VREC. The assurance owner decides only that VREC; the WO remains `implemented`. |
| `WFL-WO-VERIFIED-VREC` | Focused WO is `implemented`; a related VREC is `verified` or `released`. | `QG-G4-VERIFIED-COVERAGE` / `DR-DELIVERY-SELECT` | `PROC-DELIVERY-SELECT` | Selection changes neither record. Complete alternatives are `PROC-REPOSITORY-INTEGRATION` and `PROC-PREPARE-RELEASE`. |
| `WFL-WO-NO-VREC` | Focused WO is `implemented` and assurance is `not_required`. | No new assurance gate / `DR-RELATED-RECORD-SELECT` | `PROC-FOCUS-SELECTED` | Report completed execution; no VREC is required. |
| `WFL-WO-PREPARE-VREC` | Focused WO is `implemented`; no ready, verified, or released VREC covers it. | `QG-G4-CANDIDATE-READY` / `DR-VREC-PREPARE` | `PROC-WO-PREPARE-VREC` | `STEP-WO-PREPARE-VREC-CAPTURE` command under the selected work-order approvals. |
| `WFL-WO-START` | Focused WO is `approved`. | `QG-G3-WORK-AUTHORIZATION` / `DR-WO-START` | `PROC-WO-START` | `STEP-WO-START-PREFLIGHT` command, `STEP-WO-START-PREVIEW` command, `STEP-WO-START-APPLY` command, `STEP-WO-START-FINAL-FOCUS` command. |
| `WFL-WO-IMPLEMENT` | Focused WO is `in_progress`. | `QG-G4-IMPLEMENTATION-EVIDENCE` / `DR-WO-COMPLETE` | `PROC-WO-IMPLEMENT` | `STEP-WO-IMPLEMENT-CHECK` command, `STEP-WO-IMPLEMENT-PREVIEW` command, `STEP-WO-IMPLEMENT-APPLY` command. |
| `WFL-WO-COMPLETED` | Focused WO is `verified` or `released`. | No gate / `DR-RELATED-RECORD-SELECT` | `PROC-FOCUS-SELECTED` | Projection changes nothing. |
| `WFL-VREC-DECIDE` | Focused VREC is `ready`. | `QG-G4-ASSURANCE-DECISION` / `DR-VREC-DECIDE` | `PROC-VREC-DECIDE` | Change only the VREC. Complete alternatives are `PROC-VREC-REJECT` and `PROC-VREC-SUPERSEDE`. |
| `WFL-VREC-DELIVER` | Focused VREC is `verified` or `released`. | `QG-G4-VERIFIED-COVERAGE` / `DR-DELIVERY-SELECT` | `PROC-DELIVERY-SELECT` | Selection changes nothing. `PROC-REPOSITORY-INTEGRATION` is a complete alternative. |
| `WFL-RLS-DECIDE` | Focused RLS is `ready`. | `QG-G5-RELEASE-DECISION` / `DR-RLS-DECIDE` | `PROC-RLS-DECIDE` | Change only the RLS. `PROC-RLS-REJECT` is a complete alternative. |
| `WFL-RLS-EXTERNAL` | Focused RLS is `released`. | `QG-G5-EXTERNAL-ACTION` / `DR-EXTERNAL-ACTION` | `PROC-EXTERNAL-ACTION` | Release status performs no external action. |
| `WFL-REJECTED` | Focused artifact is `rejected`. | No gate / `DR-REMEDIATION-SCOPE` | `PROC-REMEDIATE` | Preserve rejected history; remediation does not expand selected scope. |
| `WFL-VREC-SUPERSEDED` | Focused VREC is `superseded`. | No gate / `DR-RELATED-RECORD-SELECT` | `PROC-FOCUS-SELECTED` | Preserve the old VREC as release-ineligible history. |
| `WFL-DEFINITION-COMPLETE` | Focused definition is `approved`. | `QG-G1-DEFINITION`, `QG-G2-ARCHITECTURE` / `DR-DEFINITION-DECIDE` | `PROC-DEFINITION-COMPLETE` | Change only the explicitly selected definition. |
| `WFL-DEFINITION-WORK` | Focused definition is `implemented`. | `QG-G3-WORK-AUTHORIZATION` / `DR-WO-SELECT` | `PROC-DEFINITION-WORK` | Selecting work changes no lifecycle state. |
| `WFL-DEC-OPEN` | Focused decision is `open` or `deferred`. | No gate / `DR-DECISION-DISPOSE` | `PROC-DEC-DISPOSE` | Dispose only the selected decision with one declared option; the artifacts it blocks change state through their own transitions afterwards. |
| `WFL-DEC-CLOSED` | Focused decision is `decided` or `withdrawn`. | No gate / `DR-RELATED-RECORD-SELECT` | `PROC-FOCUS-SELECTED` | Retained history; changes nothing. |
| `WFL-DEFAULT-REVIEW` | No earlier rule matches. | No gate / `DR-RELATED-RECORD-SELECT` | `PROC-FOCUS-SELECTED` | Report current state; change nothing. |
| `WFL-FAIL-REMEDIATE` | A workflow command fails. | No gate / `DR-REMEDIATION-SCOPE` | `PROC-REMEDIATE` | Report the exact blocker and unchanged state. |

## Ordered procedure registry

The arrays below are a human index. `WORKFLOW.json` remains authoritative for
argument boundaries, gate IDs, effects, non-effects, decision roles, permitted
outcomes, and response values.

| Procedure ID | Ordered typed steps |
| --- | --- |
| `PROC-WO-START` | `STEP-WO-START-PREFLIGHT`, `STEP-WO-START-PREVIEW`, `STEP-WO-START-APPLY`, `STEP-WO-START-FINAL-FOCUS`: commands using recorded approval, without another owner start decision. |
| `PROC-WO-IMPLEMENT` | `STEP-WO-IMPLEMENT-CHECK`, `STEP-WO-IMPLEMENT-PREVIEW`, `STEP-WO-IMPLEMENT-APPLY`: handoff, preview and completion commands under the existing grant. |
| `PROC-WO-PREPARE-VREC` | `STEP-WO-PREPARE-VREC-CAPTURE`: prepare using inspected inputs and each selected WO approval. |
| `PROC-FOCUS-SELECTED` | `STEP-FOCUS-SELECTED` command `harnessctl check . --artifact {artifact_id}`. |
| `PROC-FOCUS-RELATED` | `STEP-FOCUS-RELATED` command `harnessctl check . --artifact {related_id}`. |
| `PROC-VREC-DECIDE` | `STEP-VREC-DECIDE` decision `DR-VREC-DECIDE`. |
| `PROC-VREC-REJECT` | `STEP-VREC-REJECT` decision `DR-VREC-DECIDE`. |
| `PROC-VREC-SUPERSEDE` | `STEP-VREC-SUPERSEDE` decision `DR-VREC-DECIDE`. |
| `PROC-DELIVERY-SELECT` | `STEP-DELIVERY-SELECT` decision `DR-DELIVERY-SELECT`. |
| `PROC-REPOSITORY-INTEGRATION` | `STEP-REPOSITORY-INTEGRATION` decision `DR-DELIVERY-SELECT`. |
| `PROC-PREPARE-RELEASE` | `STEP-PREPARE-RELEASE` decision `DR-DELIVERY-SELECT`. |
| `PROC-RLS-DECIDE` | `STEP-RLS-DECIDE` decision `DR-RLS-DECIDE`. |
| `PROC-RLS-REJECT` | `STEP-RLS-REJECT` decision `DR-RLS-DECIDE`. |
| `PROC-EXTERNAL-ACTION` | `STEP-EXTERNAL-ACTION` decision `DR-EXTERNAL-ACTION`. |
| `PROC-REMEDIATE` | `STEP-REMEDIATE-FOCUS` command `harnessctl check . --artifact {artifact_id}`. |
| `PROC-DEFINITION-COMPLETE` | `STEP-DEFINITION-COMPLETE` decision `DR-DEFINITION-DECIDE`. |
| `PROC-DEFINITION-WORK` | `STEP-DEFINITION-WORK` decision `DR-WO-SELECT`. |
| `PROC-DEC-DISPOSE` | `STEP-DEC-DISPOSE` decision `DR-DECISION-DISPOSE`; the response is the `harnessctl decide` command with the declared option, the role and the verbatim reason. |

A command step that names gates also declares one `corrective` form per
predicate of those gates: a command argument array that differs from the
evaluated command, an escalation naming a decision right, or a response. When
`harnessctl check` is blocked, it renders the corrective form of the first
failing predicate under `Next` and `Command or response`. A contract whose
corrective form repeats the evaluated command fails to load with `WEX-ADS-001`.
The completeness corrective names `harnessctl check ... --from-git <base>`,
which derives the change set from Git; an operation that could not select or
act names `harnessctl check . --artifact <ID>` as the retry, never the
evaluated command.

## Transition procedure

For an accountable lifecycle decision:

1. Run `harnessctl check . --artifact <ID>` (no checkpoint: the projection) and read the current state,
   recommendation, required authority, command or response, and alternatives.
2. Identify the matching ordered workflow rule in `WORKFLOW.json`. Verify its
   named gates pass and the actor holds its named decision right.
3. Obtain a statement naming the artifact, target state, and actor. Rejection
   MUST include a reason. VREC supersession MUST name the successor in `--reason`.
4. Preview the exact transition without `--apply`:
   `harnessctl transition . --set <ID>=<state> --decision <ID>=<actor>`.
5. Compare the preview to the explicit decision. Apply the same command with
   `--apply` only when they match.
6. Run `harnessctl check . --artifact <ID>` again and report the resulting
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

For automation, read the schema-2 JSON result. New results declare
`digest_format = "machine-fields-v1"`; their hash binds machine fields such as
candidate identity, scope, state, gate status, and command arguments. Explanatory
wording is excluded. Retained results without this marker still use their
original digest format. Human headings and wording can evolve without changing
machine evidence identity.

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
