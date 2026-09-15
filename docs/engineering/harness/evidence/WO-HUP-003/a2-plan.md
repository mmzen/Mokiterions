# A2 (part 1) - transaction 1's plan, before apply

Captured 2026-09-15 at `654e3f6`. Command: `python -I -m se_harness upgrade .` (read-only; the JSON form reports `written: false`). `SPEC-HUP-001` rule 3.

**64 files: 3 add, 28 adopt, 23 remove, 3 unchanged, 7 update. No path is `customized` and none is `conflict`.**

`adopt` moves a file's lock mode from `managed` to `seed` and changes none of its bytes; `remove` deletes a file that is not in the 0.18.0 standard template; `update` rewrites a locked file or fragment; `add` seeds a file that did not exist.

```text
remove     .agents/skills/harness-draft-change/SKILL.md
remove     .agents/skills/harness-draft-change/agents/openai.yaml
remove     .agents/skills/harness-draft-change/scripts/guard.py
remove     .agents/skills/harness-draft-change/skill-contract.json
remove     .agents/skills/harness-execute-work-order/SKILL.md
remove     .agents/skills/harness-execute-work-order/agents/openai.yaml
remove     .agents/skills/harness-execute-work-order/scripts/check_scope.py
remove     .agents/skills/harness-execute-work-order/skill-contract.json
adopt      .agents/skills/harness-operator-brief/SKILL.md
adopt      .agents/skills/harness-operator-brief/scripts/check_brief.py
adopt      .agents/skills/harness-operator-brief/skill-contract.json
adopt      .agents/skills/harness-orient/SKILL.md
adopt      .agents/skills/harness-orient/scripts/orient.py
adopt      .agents/skills/harness-orient/skill-contract.json
remove     .agents/skills/harness-prepare-assurance/SKILL.md
remove     .agents/skills/harness-prepare-assurance/agents/openai.yaml
remove     .agents/skills/harness-prepare-assurance/scripts/check_prepare.py
remove     .agents/skills/harness-prepare-assurance/skill-contract.json
remove     .claude/skills/harness-draft-change/SKILL.md
remove     .claude/skills/harness-execute-work-order/SKILL.md
adopt      .claude/skills/harness-orient/SKILL.md
remove     .claude/skills/harness-prepare-assurance/SKILL.md
update     .engineering-harness.toml
adopt      .github/workflows/engineering-harness.yml
update     .gitignore
update     AGENTS.md
update     CLAUDE.md
update     ENGINEERING_HARNESS.md
add        GLOSSARY.md
adopt      docs/engineering/ARTIFACT_AUTHORING.md
adopt      docs/engineering/DECISION_RIGHTS.md
adopt      docs/engineering/OPERATING_CARD.md
update     docs/engineering/QUALITY_GATES.json
adopt      docs/engineering/QUALITY_GATES.md
adopt      docs/engineering/TECHNICAL_COMMUNICATION.md
adopt      docs/engineering/TRACEABILITY.md
update     docs/engineering/WORKFLOW.json
adopt      docs/engineering/WORKFLOW.md
adopt      docs/engineering/templates/ADR.template.md
adopt      docs/engineering/templates/ARCHITECTURE.template.md
adopt      docs/engineering/templates/CAPABILITY.template.md
add        docs/engineering/templates/DECISION.template.md
adopt      docs/engineering/templates/INTENT.template.md
adopt      docs/engineering/templates/OPERATING_CONTRACT.template.md
adopt      docs/engineering/templates/README.md
adopt      docs/engineering/templates/RELEASE_CONTRACT.template.md
adopt      docs/engineering/templates/RELEASE_RECORD.template.md
adopt      docs/engineering/templates/REQUIREMENT.template.md
add        docs/engineering/templates/RISK.template.md
adopt      docs/engineering/templates/SPECIFICATION.template.md
adopt      docs/engineering/templates/VERIFICATION.template.md
adopt      docs/engineering/templates/VERIFICATION_RECORD.template.md
adopt      docs/engineering/templates/WORK_ORDER.template.md
remove     scripts/artifact_layout_registry.py
remove     scripts/check_engineering_harness.ps1
remove     scripts/check_engineering_harness.sh
remove     scripts/generate_harness_dashboard.py
remove     scripts/harness_explorer/index.template.html
remove     scripts/inspect_engineering_artifacts.py
remove     scripts/select_harness_work_order.py
remove     scripts/validate_engineering_artifacts.py
summary: 64 files, 3 unchanged
```
