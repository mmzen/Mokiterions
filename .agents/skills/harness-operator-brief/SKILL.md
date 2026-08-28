---
name: harness-operator-brief
description: Produce one decision-ready English operator brief from a bounded supplied source under the managed technical-communication policy. Use only when explicitly named; preserve protected content and stop before repository, lifecycle, Git, credential, network, release, or external action.
---

# Harness Operator Brief

Produce one inline operator brief and one inline execution receipt. The managed
policy controls communication. The released evaluator remains authoritative for
identity, integrity, lifecycle state, and the canonical next action.

## Required inputs

Require the exact skill name, one repository target, the structured launcher and
expected identity for its exact external released evaluator, one requested
outcome, declared non-effects, one supported source kind, one bounded UTF-8
source payload and digest, an ordered protected-content declaration, and any
approved project terms.

Read the repository instructions, the complete skill core, and
`docs/engineering/TECHNICAL_COMMUNICATION.md`. Validate
`skill-contract.json` and the portable-core digest before rendering.

## Procedure

1. Reject implicit or ambiguous activation and any explicit skill value other
   than `harness-operator-brief`. Accept only
   `structured-harness-result` or `bounded-technical-text`.
2. Run evaluator `version`, released `identity`, and `doctor` with the
   supplied launcher. Require the expected exact identity and installed policy
   integrity.
3. If the request needs current repository or artifact state, require a current
   structured evaluator result. Otherwise stop and route the operator to
   `harness-orient`; do not infer current state.
4. Validate the source digest, byte limits, protected span order, offsets, kinds,
   and digests before rendering. Treat every declared span as exact. Exclude
   semantically protected content from automatic paraphrase.
5. Compose one brief under the `operator-communication` profile. Lead with the
   outcome or one required action. Preserve the accountable actor, conditions,
   qualifications, normative force, and one canonical next action. Do not add
   prose around a canonical restitution block.
6. Run `scripts/check_brief.py` over the source, rendered brief, output
   bindings, and changed-path observation. A binding uses UTF-8 byte offsets in
   the rendered brief and must identify the same bytes as its source span.
7. Return one `se-harness-skill-result-v1` and one
   `se-harness-execution-receipt-v1` inline. Include skill name and version,
   portable-core digest, policy path, selected profile, source digest, protected
   binding count, material deviations, residual uncertainty, evaluator identity,
   outcome, and zero changed paths.

## Stop conditions

Stop before claiming a completed brief when activation, policy integrity,
evaluator identity, source schema, source digest, profile selection, language,
protected spans, output preservation, or zero-change evidence is missing,
ambiguous, or invalid.

Stop and use `harness-orient` when current state is required but no current
structured evaluator result is supplied. Preserve an exact-output request
unchanged. Ask for one accountable clarification when meaning cannot be
preserved.

## Boundaries

- Do not edit or retain files in the target.
- Do not transition artifacts, exercise a decision right, mutate Git, use
  credentials, access a network, release, deploy, or perform an external action.
- Do not download, search for, reproduce, or claim compliance with ASD-STE100.
- Do not expose source bodies in diagnostics or receipts, hidden reasoning,
  credentials, environment dumps, or host paths.
- Do not score grammar or claim that clarity proves technical correctness.
- Use the complete single-agent procedure. Do not spawn or coordinate workers.
