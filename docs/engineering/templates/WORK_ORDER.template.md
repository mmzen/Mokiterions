+++
id = "WO-xxx"
type = "work_order"
title = "<Bounded implementation objective>"
status = "draft"
owners = ["<engineering owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"

[assurance]
commit_bound_verification = "<required or not_required>"
rationale = "<why future decisions do or do not require commit-bound assurance>"
decided_by = "<accountable role>"

[execution_scope]
paths = [
  "<exact/repository-relative/path>",
  "<repository-relative/component-prefix/>",
]

# Optional. Delete this entire table when no agentic delegation is intended.
[agentic_delegation]
schema = "se-harness-agentic-delegation-v1"
delegated_by = "<accountable-role>"
delegate = "<logical-worker>"
decision_rights = ["DR-WO-START"]
operations = ["<closed-evaluator-operation>"]
execution_profiles = ["<approved-logical-profile>"]
paths = ["<path-within-execution-scope>"]
required_evidence = [
  { kind = "verification", path = "<retained-evidence-path>" },
]
valid_until = "YYYY-MM-DDTHH:MM:SSZ"
max_retry = 0
max_parallel_writers = 1
child_delegation = false
stop_before = [
  "accountable-decision-required",
  "action-time-authorization-required",
]

[relations]
implements = ["REQ-xxx"]
specifications = ["SPEC-xxx"]
verification = ["VER-xxx"]
+++

# Work Order: <title>

## Lifecycle

Use `approved` to authorize bounded execution and `implemented` after the work and retained evidence are complete. Governance-only work normally stops at `implemented`. Use `verified` or `released` only when an eligible commit-bound VREC explicitly covers this work order under the repository's configured provenance policy.

Before approval, classify commit-bound verification explicitly. Use `required` when future engineering, assurance, operational, or release decisions will rely on the correctness of changed executable behavior, managed policy, CI, definitions, traceability, or other trusted engineering state. Use `not_required` only when the work solely records or transports an already authorized verification, release, supersession, publication, or deployment decision. Split mixed scope or use `required`; uncertainty requires escalation rather than an inferred default.

Before approval, replace every `[execution_scope].paths` placeholder. An entry
ending in `/` admits that directory and its descendants. Every other entry
admits one exact repository-relative path. Do not use absolute paths,
backslashes, wildcards, dot components, drive prefixes, URIs, or duplicate
case variants.

The optional agentic_delegation table records a maximum delegation; it does not
start work or grant standing authority. Delete the table when delegation is not
intended. When retained, replace every placeholder, keep every delegated and
evidence path within execution_scope.paths, use only managed decision rights,
evaluator operations, logical profiles, and roles, and set a bounded UTC
expiry. The exact released evaluator still derives a narrower, short-lived
envelope from fresh live state for each request.

Add `architecture = ["ARCH-xxx", "ADR-xxx"]` under `[relations]` when architecture applies. The relation selects every applicable architecture plus every required deciding ADR. An ADR may be omitted only for a selected architecture whose accepted `decision_assessment` is `no_significant_decision`; every `adr_required` architecture needs at least one selected active ADR that decides it.

An architecture is applicable when it addresses an architecturally significant requirement implemented by this work order. Every selected architecture must conform to at least one of the selected specifications. Omit the `architecture` relation only when no active architecture addresses any implemented requirement. Routine requirements without an active `addresses` edge do not require fabricated architecture coverage. A present `architecture` relation must not be empty.

## Objective

## In scope

## Out of scope

## Authorized decision envelope

What may the implementation agent decide locally?

## Constraints

## Expected change surface

Use components rather than guessed files when the code has not yet been inspected.

## Required verification

## Evidence to record

## Stop and escalate conditions

## Completion report format
