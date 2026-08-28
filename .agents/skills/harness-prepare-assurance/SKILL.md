---
name: harness-prepare-assurance
description: Prepare one SE Harness verification record through delegated Phase 4 VREC preparation. Use only when explicitly named with completion proof and valid delegation; stop for a required Git commit or the independent assurance decision.
---

# Harness Prepare Assurance

Prepare one reviewable verification record and its assurance decision packet.
This skill is a non-authoritative client. The exact released evaluator alone
validates completion proof, delegation, candidate state, canonical destination,
record preparation, restitution, and the next accountable action.

## Required inputs

Require the exact skill name, one target, a structured launcher and expected
identity for its external released evaluator, requested outcome and non-effects,
one implemented work order, applicable verification contracts, retained
evidence paths, the full evaluator-issued completion proof, optional existing
candidate commit, one unused collision-checked VREC ID and canonical
destination, exact Phase 4 delegation evidence, and an explicit preparation
actor. Read repository instructions and this complete core. Validate
`skill-contract.json` and the portable-core digest.

## Procedure

1. Reject implicit activation, a missing preparation actor, an ambiguous
   selection, or any exact skill value other than
   `harness-prepare-assurance`.
2. Run evaluator `version`, released `identity`, and `doctor`, then request
   `delegated-workflow catalog`. Require exact workflow-v4 capability and the
   selected work order in `implemented`.
3. Build the closed `delegated-workflow prepare-vrec` argument array from the
   retained completion proof and declared evidence. Treat evaluator-derived
   VREC and evidence destinations as the only possible governed outputs.
4. Pass the request through `scripts/check_prepare.py`. The helper rejects
   direct-target mode and invokes only evaluator preparation; it has no VREC
   file-writing callback.
5. If commit-bound preparation needs an exact candidate commit, return the
   evaluator's Git-action packet and stop without creating a record or commit.
   Otherwise require one evaluator-prepared `ready` VREC, receipt, and assurance
   packet bound to the declared completion proof.
6. Return the structured client result and stop before the independent
   assurance decision. Do not execute a packet command.

## Boundaries

- Do not write the VREC or evaluator evidence directly. Do not verify, reject,
  supersede, deliver, release, mutate Git, use credentials, access the network,
  or perform an external action.
- The preparation actor is evidence of preparation, never proof of assurance
  ownership or an assurance decision.
- Stop on a dirty or changed candidate, stale state, failed gate, missing
  evidence, ID collision, unexpected path, invalid graph, or missing required
  capability.
- Use the complete single-agent procedure. Do not spawn or coordinate workers.

The new record begins `ready` only because the existing preparation operation
creates it there. That creation is not a lifecycle transition or verification.
