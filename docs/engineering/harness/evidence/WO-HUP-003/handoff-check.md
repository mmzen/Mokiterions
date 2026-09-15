# The handoff checkpoint

Evaluator: se-harness 0.18.0 from the exact public wheel, outside the checkout, `python -I -m se_harness`. Captured 2026-09-15 at `7dd0030`, the implementation commit, with the change set derived from Git against `main`.

## Run 1, before the evidence packet existed

    harnessctl check . --artifact WO-HUP-003 --checkpoint handoff --from-git main

```text
Outcome
Blocked.

Done
- Evaluated handoff compliance for WO-HUP-003.

Not done
- The handoff checkpoint did not pass.

Blocked by
- QGP-G4I-EVIDENCE: No readable evidence for WO-HUP-003, checkpoint handoff, and formal snapshot b7ce520821a5a0d41e217dfefde0cfe89e4a3864279416884d5d2cfdc10112da is available; write the header with harnessctl evidence . --artifact WO-HUP-003 --checkpoint handoff.

Current lifecycle state
- WO-HUP-003 is in_progress.

Decision required
None.

Next
Supply the corrective input for QGP-G4I-EVIDENCE (PROC-WO-IMPLEMENT/STEP-WO-IMPLEMENT-CHECK).

Command or response
harnessctl evidence . --artifact WO-HUP-003 --checkpoint handoff

Change set
- .agents/skills/harness-draft-change/agents/openai.yaml
- .agents/skills/harness-draft-change/scripts/guard.py
- .agents/skills/harness-draft-change/skill-contract.json
- .agents/skills/harness-draft-change/SKILL.md
- .agents/skills/harness-execute-work-order/agents/openai.yaml
- .agents/skills/harness-execute-work-order/scripts/check_scope.py
- .agents/skills/harness-execute-work-order/skill-contract.json
- .agents/skills/harness-execute-work-order/SKILL.md
- .agents/skills/harness-operator-brief/scripts/check_brief.py
- .agents/skills/harness-operator-brief/skill-contract.json
- .agents/skills/harness-operator-brief/SKILL.md
- .agents/skills/harness-orient/scripts/orient.py
- .agents/skills/harness-orient/skill-contract.json
- .agents/skills/harness-orient/SKILL.md
- .agents/skills/harness-prepare-assurance/agents/openai.yaml
- .agents/skills/harness-prepare-assurance/scripts/check_prepare.py
- .agents/skills/harness-prepare-assurance/skill-contract.json
- .agents/skills/harness-prepare-assurance/SKILL.md
- .claude/skills/harness-draft-change/SKILL.md
- .claude/skills/harness-execute-work-order/SKILL.md
- .claude/skills/harness-orient/SKILL.md
- .claude/skills/harness-prepare-assurance/SKILL.md
- .engineering-harness.lock
- .engineering-harness.toml
- .github/workflows/engineering-harness.yml
- .github/workflows/release.yml
- .gitignore
- AGENTS.md
- CLAUDE.md
- docs/engineering/ARTIFACT_AUTHORING.md
- docs/engineering/DECISION_RIGHTS.md
- docs/engineering/harness/evidence/WO-HUP-003/a1-validate.md
- docs/engineering/harness/evidence/WO-HUP-003/a2-plan.md
- docs/engineering/harness/evidence/WO-HUP-003/a2-replace-plan.md
- docs/engineering/harness/evidence/WO-HUP-003/a5-release-record-unmoved.md
- docs/engineering/harness/evidence/WO-HUP-003/a6-version-references.md
- docs/engineering/harness/evidence/WO-HUP-003/a7-no-product-effect.md
- docs/engineering/harness/evidence/WO-HUP-003/b1-start-before.md
- docs/engineering/harness/evidence/WO-HUP-003/b2-start-after.md
- docs/engineering/harness/evidence/WO-HUP-003/b3-p1-p2-wo-mok-027.md
- docs/engineering/harness/evidence/WO-HUP-003/handoff.json
- docs/engineering/harness/evidence/WO-HUP-003/identity.md
- docs/engineering/harness/evidence/WO-HUP-003/n2-doctor.md
- docs/engineering/harness/evidence/WO-HUP-003/pre-doctor.md
- docs/engineering/harness/evidence/WO-HUP-003/replace-list.md
- docs/engineering/harness/evidence/WO-HUP-003/replace-transaction.json
- docs/engineering/harness/evidence/WO-HUP-003/skill-ownership.md
- docs/engineering/harness/evidence/WO-HUP-003/upgrade-transaction.json
- docs/engineering/harness/work-orders/WO-HUP-003.md
- docs/engineering/OPERATING_CARD.md
- docs/engineering/QUALITY_GATES.json
- docs/engineering/QUALITY_GATES.md
- docs/engineering/simulation/work-orders/WO-MOK-027.md
- docs/engineering/templates/ADR.template.md
- docs/engineering/templates/ARCHITECTURE.template.md
- docs/engineering/templates/CAPABILITY.template.md
- docs/engineering/templates/DECISION.template.md
- docs/engineering/templates/INTENT.template.md
- docs/engineering/templates/README.md
- docs/engineering/templates/RELEASE_CONTRACT.template.md
- docs/engineering/templates/RELEASE_RECORD.template.md
- docs/engineering/templates/REQUIREMENT.template.md
- docs/engineering/templates/RISK.template.md
- docs/engineering/templates/SPECIFICATION.template.md
- docs/engineering/templates/VERIFICATION.template.md
- docs/engineering/templates/VERIFICATION_RECORD.template.md
- docs/engineering/templates/WORK_ORDER.template.md
- docs/engineering/TRACEABILITY.md
- docs/engineering/WORKFLOW.json
- docs/engineering/WORKFLOW.md
- docs/RELEASE_RUNBOOK.md
- ENGINEERING_HARNESS.md
- GLOSSARY.md
- scripts/artifact_layout_registry.py
- scripts/check_engineering_harness.ps1
- scripts/check_engineering_harness.sh
- scripts/generate_harness_dashboard.py
- scripts/harness_explorer/index.template.html
- scripts/inspect_engineering_artifacts.py
- scripts/select_harness_work_order.py
- scripts/validate_engineering_artifacts.py
complete: true

Gates
QGP-G4I-STATUS: pass
QGP-G4I-GRAPH: pass
QGP-G4I-INTEGRITY: pass
QGP-G4I-SCOPE: pass
QGP-G4I-COMPLETE: pass
QGP-G4I-PATHS: pass
QGP-G4I-PREFLIGHT: pass
QGP-G4I-EVIDENCE: not_assessable
QGP-G4I-DECISION: pass
```

Every gate passes except `QGP-G4I-EVIDENCE`, which is `not_assessable` until a packet bound to the formal snapshot exists. The corrective it names was run:

    harnessctl evidence . --artifact WO-HUP-003 --checkpoint handoff

```text
Outcome
Completed.

Done
- Wrote the handoff evidence packet of WO-HUP-003 at docs/engineering/harness/evidence/WO-HUP-003/WO-HUP-003-handoff.md to formal snapshot b7ce520821a5a0d41e217dfefde0cfe89e4a3864279416884d5d2cfdc10112da.

Not done
None.

Current lifecycle state
- WO-HUP-003 is in_progress.

Decision required
None.

Next
Run the bound command (PROC-WO-IMPLEMENT/STEP-WO-IMPLEMENT-CHECK).

Command or response
harnessctl check . --artifact WO-HUP-003 --checkpoint handoff

Change set
None.
complete: false

Gates
None.
```

## Run 2, with the packet in place

```text
Outcome
Completed.

Done
- Evaluated handoff compliance for WO-HUP-003.

Not done
None.

Current lifecycle state
- WO-HUP-003 is in_progress.

Decision required
None.

Next
Run the bound command (PROC-WO-IMPLEMENT/STEP-WO-IMPLEMENT-PREVIEW).

Command or response
harnessctl transition . --set WO-HUP-003=implemented --decision WO-HUP-003=delegated-executor

Change set
- .agents/skills/harness-draft-change/agents/openai.yaml
- .agents/skills/harness-draft-change/scripts/guard.py
- .agents/skills/harness-draft-change/skill-contract.json
- .agents/skills/harness-draft-change/SKILL.md
- .agents/skills/harness-execute-work-order/agents/openai.yaml
- .agents/skills/harness-execute-work-order/scripts/check_scope.py
- .agents/skills/harness-execute-work-order/skill-contract.json
- .agents/skills/harness-execute-work-order/SKILL.md
- .agents/skills/harness-operator-brief/scripts/check_brief.py
- .agents/skills/harness-operator-brief/skill-contract.json
- .agents/skills/harness-operator-brief/SKILL.md
- .agents/skills/harness-orient/scripts/orient.py
- .agents/skills/harness-orient/skill-contract.json
- .agents/skills/harness-orient/SKILL.md
- .agents/skills/harness-prepare-assurance/agents/openai.yaml
- .agents/skills/harness-prepare-assurance/scripts/check_prepare.py
- .agents/skills/harness-prepare-assurance/skill-contract.json
- .agents/skills/harness-prepare-assurance/SKILL.md
- .claude/skills/harness-draft-change/SKILL.md
- .claude/skills/harness-execute-work-order/SKILL.md
- .claude/skills/harness-orient/SKILL.md
- .claude/skills/harness-prepare-assurance/SKILL.md
- .engineering-harness.lock
- .engineering-harness.toml
- .github/workflows/engineering-harness.yml
- .github/workflows/release.yml
- .gitignore
- AGENTS.md
- CLAUDE.md
- docs/engineering/ARTIFACT_AUTHORING.md
- docs/engineering/DECISION_RIGHTS.md
- docs/engineering/harness/evidence/WO-HUP-003/a1-validate.md
- docs/engineering/harness/evidence/WO-HUP-003/a2-plan.md
- docs/engineering/harness/evidence/WO-HUP-003/a2-replace-plan.md
- docs/engineering/harness/evidence/WO-HUP-003/a5-release-record-unmoved.md
- docs/engineering/harness/evidence/WO-HUP-003/a6-version-references.md
- docs/engineering/harness/evidence/WO-HUP-003/a7-no-product-effect.md
- docs/engineering/harness/evidence/WO-HUP-003/b1-start-before.md
- docs/engineering/harness/evidence/WO-HUP-003/b2-start-after.md
- docs/engineering/harness/evidence/WO-HUP-003/b3-p1-p2-wo-mok-027.md
- docs/engineering/harness/evidence/WO-HUP-003/handoff.json
- docs/engineering/harness/evidence/WO-HUP-003/identity.md
- docs/engineering/harness/evidence/WO-HUP-003/n2-doctor.md
- docs/engineering/harness/evidence/WO-HUP-003/pre-doctor.md
- docs/engineering/harness/evidence/WO-HUP-003/replace-list.md
- docs/engineering/harness/evidence/WO-HUP-003/replace-transaction.json
- docs/engineering/harness/evidence/WO-HUP-003/skill-ownership.md
- docs/engineering/harness/evidence/WO-HUP-003/upgrade-transaction.json
- docs/engineering/harness/evidence/WO-HUP-003/WO-HUP-003-handoff.md
- docs/engineering/harness/work-orders/WO-HUP-003.md
- docs/engineering/OPERATING_CARD.md
- docs/engineering/QUALITY_GATES.json
- docs/engineering/QUALITY_GATES.md
- docs/engineering/simulation/work-orders/WO-MOK-027.md
- docs/engineering/templates/ADR.template.md
- docs/engineering/templates/ARCHITECTURE.template.md
- docs/engineering/templates/CAPABILITY.template.md
- docs/engineering/templates/DECISION.template.md
- docs/engineering/templates/INTENT.template.md
- docs/engineering/templates/README.md
- docs/engineering/templates/RELEASE_CONTRACT.template.md
- docs/engineering/templates/RELEASE_RECORD.template.md
- docs/engineering/templates/REQUIREMENT.template.md
- docs/engineering/templates/RISK.template.md
- docs/engineering/templates/SPECIFICATION.template.md
- docs/engineering/templates/VERIFICATION.template.md
- docs/engineering/templates/VERIFICATION_RECORD.template.md
- docs/engineering/templates/WORK_ORDER.template.md
- docs/engineering/TRACEABILITY.md
- docs/engineering/WORKFLOW.json
- docs/engineering/WORKFLOW.md
- docs/RELEASE_RUNBOOK.md
- ENGINEERING_HARNESS.md
- GLOSSARY.md
- scripts/artifact_layout_registry.py
- scripts/check_engineering_harness.ps1
- scripts/check_engineering_harness.sh
- scripts/generate_harness_dashboard.py
- scripts/harness_explorer/index.template.html
- scripts/inspect_engineering_artifacts.py
- scripts/select_harness_work_order.py
- scripts/validate_engineering_artifacts.py
complete: true

Gates
QGP-G4I-STATUS: pass
QGP-G4I-GRAPH: pass
QGP-G4I-INTEGRITY: pass
QGP-G4I-SCOPE: pass
QGP-G4I-COMPLETE: pass
QGP-G4I-PATHS: pass
QGP-G4I-PREFLIGHT: pass
QGP-G4I-EVIDENCE: pass
QGP-G4I-DECISION: pass
```

All nine gates pass, and the next bound command is the completion transition under the recorded approval. The formal snapshot `b7ce5208…112da` is the one the packet binds; retained evidence, including this file and the completion summary, does not move it.
