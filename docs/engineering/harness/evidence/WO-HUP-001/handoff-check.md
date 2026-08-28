# Handoff checkpoint evidence

Retained for the `in_progress` to `implemented` transition of `WO-HUP-001`, which the
`QGP-G4I-EVIDENCE` gate refuses without it.

artifact: WO-HUP-001
checkpoint: handoff
formal_snapshot_sha256: b72a268a39cb156a427a729ed793975a98636c45ff2ea02e93fa79099fbd742e

The digest above is the formal snapshot of the artifact graph as it stood when the handoff
check was evaluated. It is a binding to *that* graph: an artifact change moves the snapshot,
and this evidence stops satisfying the gate until it is re-bound. The snapshot covers governed
artifacts and not retained evidence, so this file sitting inside the packet it belongs to does
not move the digest it names.

## The declared change set

69 paths, asserted complete with `--changes-complete`, every one of them admitted by
`WO-HUP-001`'s `[execution_scope].paths`. Measured against `master`:

```text
  .agents/skills/harness-draft-change/SKILL.md
  .agents/skills/harness-draft-change/agents/openai.yaml
  .agents/skills/harness-draft-change/scripts/guard.py
  .agents/skills/harness-draft-change/skill-contract.json
  .agents/skills/harness-execute-work-order/SKILL.md
  .agents/skills/harness-execute-work-order/agents/openai.yaml
  .agents/skills/harness-execute-work-order/scripts/check_scope.py
  .agents/skills/harness-execute-work-order/skill-contract.json
  .agents/skills/harness-operator-brief/SKILL.md
  .agents/skills/harness-operator-brief/scripts/check_brief.py
  .agents/skills/harness-operator-brief/skill-contract.json
  .agents/skills/harness-orient/SKILL.md
  .agents/skills/harness-orient/scripts/orient.py
  .agents/skills/harness-orient/skill-contract.json
  .agents/skills/harness-prepare-assurance/SKILL.md
  .agents/skills/harness-prepare-assurance/agents/openai.yaml
  .agents/skills/harness-prepare-assurance/scripts/check_prepare.py
  .agents/skills/harness-prepare-assurance/skill-contract.json
  .claude/skills/harness-draft-change/SKILL.md
  .claude/skills/harness-execute-work-order/SKILL.md
  .claude/skills/harness-orient/SKILL.md
  .claude/skills/harness-prepare-assurance/SKILL.md
  .engineering-harness.lock
  .engineering-harness.toml
  .gitattributes
  .github/workflows/engineering-harness.yml
  .github/workflows/release.yml
  AGENTS.md
  CLAUDE.md
  ENGINEERING_HARNESS.md
  docs/engineering/ARTIFACT_AUTHORING.md
  docs/engineering/DECISION_RIGHTS.md
  docs/engineering/OPERATING_CARD.md
  docs/engineering/QUALITY_GATES.json
  docs/engineering/QUALITY_GATES.md
  docs/engineering/TECHNICAL_COMMUNICATION.md
  docs/engineering/TRACEABILITY.md
  docs/engineering/WORKFLOW.json
  docs/engineering/WORKFLOW.md
  docs/engineering/harness/capabilities/CAP-HUP-001.md
  docs/engineering/harness/evidence/WO-HUP-001/a1-validate.md
  docs/engineering/harness/evidence/WO-HUP-001/a2-plan-declared.md
  docs/engineering/harness/evidence/WO-HUP-001/a3-undeclared-refusal.md
  docs/engineering/harness/evidence/WO-HUP-001/a4-in-tree-validator.md
  docs/engineering/harness/evidence/WO-HUP-001/a5-release-record-unmoved.md
  docs/engineering/harness/evidence/WO-HUP-001/a6-version-references.md
  docs/engineering/harness/evidence/WO-HUP-001/a7-no-product-effect.md
  docs/engineering/harness/evidence/WO-HUP-001/completion-summary.md
  docs/engineering/harness/evidence/WO-HUP-001/n1-declaration-resolution.md
  docs/engineering/harness/evidence/WO-HUP-001/n2-doctor.md
  docs/engineering/harness/evidence/WO-HUP-001/s1-no-secrets.md
  docs/engineering/harness/evidence/WO-HUP-001/transition-unblocked.md
  docs/engineering/harness/evidence/WO-HUP-001/upgrade-transaction.json
  docs/engineering/harness/intent/INT-HUP-001.md
  docs/engineering/harness/requirements/REQ-HUP-001.md
  docs/engineering/harness/specifications/SPEC-HUP-001.md
  docs/engineering/harness/verification/VER-HUP-001.md
  docs/engineering/harness/work-orders/WO-HUP-001.md
  docs/engineering/templates/README.md
  docs/engineering/templates/RELEASE_CONTRACT.template.md
  docs/engineering/templates/RELEASE_RECORD.template.md
  docs/engineering/templates/REQUIREMENT.template.md
  docs/engineering/templates/VERIFICATION_RECORD.template.md
  docs/engineering/templates/WORK_ORDER.template.md
  scripts/generate_harness_dashboard.py
  scripts/harness_explorer/index.template.html
  scripts/inspect_engineering_artifacts.py
  scripts/select_harness_work_order.py
  scripts/validate_engineering_artifacts.py
```

Execution scope is enforced here rather than at preflight: an out-of-scope path is refused as
`QGP-G4I-PATHS`, and completeness cannot be inferred from the absence of one.
