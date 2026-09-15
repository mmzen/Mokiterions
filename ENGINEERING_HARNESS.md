# Engineering Harness for Mokiterions

This repository uses SE Harness 0.18.0.

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**,
**SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and
**OPTIONAL** in this document are to be interpreted as described in BCP 14
(RFC 2119 and RFC 8174) when, and only when, they appear in all capitals.

## Goals

This harness makes engineering authority explicit, limits execution to approved
scope, binds assurance and release claims to exact evidence, and gives every
actor the same deterministic next step. These goals are informative; the rules
below and the installed evaluator's machine policy are normative.

## Global invariants

`HRN-001` - Formal artifacts under `docs/engineering/` are the repository-native
source of product intent, requirements, architecture decisions, work authority,
verification contracts, assurance decisions, and release decisions. Code,
tests, commits, tickets, dashboards, and conversation text are evidence or
observations; they MUST NOT substitute for formal authority.

`HRN-002` - Repository facts and commands belong in the owner-controlled region
of `AGENTS.md`. That content is repository-owned and MUST NOT grant product,
engineering, assurance, release, or external-action authority. This harness does
not scaffold, track, or require it.

`HRN-003` - An actor MUST select one bounded artifact scope before acting. It
MUST NOT report findings from unrelated work orders as findings of the selected
scope. A discovered issue outside scope MAY be identified only as an unassessed
observation with its artifact ID, and MUST NOT block the selected work unless a
declared dependency or gate connects it.

`HRN-004` - Only `harnessctl` MAY compute lifecycle legality and the canonical
next action. Agent prose, prompts, Skills, dashboards, and repository notes MAY
render or invoke that result and MUST NOT redefine it.

`HRN-005` - Preparation, inspection, validation, and evidence capture MUST NOT
exercise a decision right. A state change requires the exact artifact, target
state, accountable actor, passing gates, and an explicit applied transition.

`HRN-006` - A transition MUST change only the artifacts explicitly selected by
the accountable actor. Related artifact states MUST NOT be synchronized by
inference.

`HRN-007` - Repository-owned instructions MAY add stricter local constraints.
They MUST NOT waive, weaken, or contradict this managed contract.

`HRN-008` - Managed integrity, graph validation, scope validation, and required
gates MUST fail closed. A warning MUST NOT be treated as approval or accepted
risk.

## Scope of these obligations

The bounded-scope invariant above, the lifecycle handoff rules, and the stop
conditions bind an actor executing or reporting a lifecycle stage. Reading,
analysis, and answering questions are unconstrained, provided no lifecycle
state changes, no decision right is exercised, and no finding is presented as
a formal result.

Before acting on a lifecycle stage, read `docs/engineering/OPERATING_CARD.md`,
the selected work order, and every governing artifact listed by the phase
reading manifest. For authoring, design or review, also read and apply
[the authoring and review policy](docs/engineering/ARTIFACT_AUTHORING.md), including
its shared design principle and the questions relevant to the selected work.
This applies whether or not a skill is used. Other routed policies below are
reference material unless the selected procedure requires them.

## Routing

The installed evaluator owns executable policy. Its machine `WORKFLOW.json`
and `QUALITY_GATES.json`, this router, and the instruction/ignore fragments are
locked. Human explanations, artifact templates, CI and owner settings are
editable files. Editing the selected version is not an upgrade; it must agree
with the installation record.

An explicit upgrade keeps editable files by default. Use `--replace-file PATH`
for each editable file that should receive the supplied template. Owner content
beside managed fragments is preserved. The guides below explain policy; they
do not replace the installed evaluator's decisions.

| Subject | Guide and machine policy |
| --- | --- |
| Lifecycle states, transitions, procedures, next actions, and handoff fields | `docs/engineering/WORKFLOW.md` and its machine-readable `WORKFLOW.json` |
| Roles, accountabilities, delegation, and reserved decisions | `docs/engineering/DECISION_RIGHTS.md` |
| Gate criteria, executable predicates, validation planes, pass/fail behavior, and exceptions | `docs/engineering/QUALITY_GATES.md` and `docs/engineering/QUALITY_GATES.json` |
| Normative chain, artifact applicability, relation types, and coverage | `docs/engineering/TRACEABILITY.md` |
| Eligible operator and technical-artifact English prose | `docs/engineering/TECHNICAL_COMMUNICATION.md` |
| Artifact authoring locations and templates | `docs/engineering/templates/README.md` |
| Authoring, design simplicity and review questions | `docs/engineering/ARTIFACT_AUTHORING.md` |
| Repository-specific facts and commands | the owner-controlled region of `AGENTS.md` |

`docs/engineering/README.md` is an index. It MUST NOT become a second policy
source.

## Lifecycle handoff

After completing a lifecycle stage or reaching a stop condition, the actor MUST
obtain the selected workflow result using result schema 2. The structured
result is authoritative. The actor MUST preserve its actual artifact IDs,
lifecycle state, observed effects, material non-effects, blockers, accountable
decision, recommended next action, and command argument boundaries or suggested
response meaning. It MUST NOT claim an effect, decision, or authority absent
from that result.

For human interaction, the actor SHOULD summarize the result clearly and
concisely. It MAY adapt wording and structure to the user and situation, add
relevant explanation, and omit empty fields. It MUST distinguish completed
work from remaining work, identify any required accountable decision, and
recommend exactly one next action. Complete alternatives MAY be shown
separately. It MUST NOT add an unrelated finding or another recommendation.

Automation reads the structured JSON result. Human wording is not evidence
identity: new result digests bind machine fields, including candidate, lifecycle
state, and command arguments. The complete handoff procedure is
`WORKFLOW.md#lifecycle-handoff-procedure`.

## Stop conditions

The actor MUST stop before changing state or scope when any of these conditions
is true:

- managed integrity fails;
- the selected governing chain is invalid or artifact IDs are ambiguous;
- no phase-eligible selected work order exists;
- a required governing artifact or gate is missing;
- a required check fails;
- owner instructions conflict with this contract;
- remediation would exceed the selected work order; or
- the requested action lacks the decision right or explicit authority defined
  by the routed policies.

The actor MUST report the failing rule or gate, the unchanged lifecycle state,
and one exact retry or accountable escalation. It MUST NOT ask an open-ended
next-action question when `harnessctl` provides a canonical recommendation.
