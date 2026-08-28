---
name: harness-execute-work-order
description: Execute one explicitly selected approved SE Harness work order through the delegated Phase 4 evaluator. Use for brokered implementation, evidence, and completion; never write the governed target directly, and stop before Git or assurance.
---

# Harness Execute Work Order

Implement exactly one approved, delegated work order. This skill is a
non-authoritative client. The exact released evaluator alone validates the
delegation, owns lifecycle admission, builds and applies the bundle, proves
completion, restores canonical state, and projects the next action.

## Required inputs

Require the exact skill name, one target, the structured launcher and expected
identity of its external released evaluator, exact requested outcome and
non-effects, one approved work-order ID, complete execution scope, verification
and evidence obligations, repository-owned commands, optional narrowing
constraints, exact Phase 4 delegation evidence, and a closed evaluator request
whose workspaces and runtime store are isolated from the target. Read
repository instructions, this complete core, and every file in the current
work-order manifest. Validate `skill-contract.json` and the portable-core
digest.

## Procedure

1. Reject implicit activation and any selection other than exactly
   `harness-execute-work-order` plus one work order.
2. Run evaluator `version`, released `identity`, and `doctor`, then request
   `delegated-workflow catalog`. Require the exact workflow-v4 catalog. Missing
   capability, invalid delegation, or a work order not in the evaluator's
   required start state stops before any effect.
3. Construct a closed candidate plan from current `[execution_scope].paths`.
   Operator constraints may narrow but never widen it. Produce candidate bytes
   only in the isolated proposed workspace; do not write target paths.
4. Run only structured repository-owned command arrays in their declared
   context and retain normalized results and evidence at declared paths.
5. Pass the closed argument vector through `scripts/check_scope.py`. The helper
   checks explicit activation and paths, rejects direct-target mode, and invokes
   only evaluator `delegated-workflow execute`; it has no write callback.
6. Accept success only when the evaluator returns continuous start, effect, and
   completion proofs plus the canonical candidate-commit decision packet. A
   session conflict, missing receipt, unexplained path, failed gate, or failed
   restitution is not partial success.
7. Return the structured client result and stop before Git. Do not execute the
   packet command or continue to VREC preparation.

## Boundaries

- Do not claim authority to start or complete the work order. The evaluator may
  perform only the formally delegated start, bundle, and completion operations.
- Do not prepare a VREC, verify, deliver, release, mutate Git, use credentials,
  access the network, or perform an external action.
- Stop on stale state, failed required gate, scope mismatch, unexpected path,
  missing evidence, invalid graph, or missing required capability.
- Use the complete single-agent procedure. Do not spawn or coordinate workers.

Evaluator receipts may prove the delegated lifecycle result, but neither this
skill nor completion attests that the implementation is correct.
