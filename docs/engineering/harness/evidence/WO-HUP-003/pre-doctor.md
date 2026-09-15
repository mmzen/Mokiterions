# The 0.18.0 evaluator's `doctor` over the 0.8.0 root, before the transaction

Captured 2026-09-15 at `654e3f6`, before any write. Evaluator: se-harness 0.18.0 from the exact public wheel, outside the checkout, `python -I -m se_harness doctor . --json`.

**Outcome `failed`: 93 checks, 30 PASS, 63 FAIL.** The distance between the two roots, measured before it is closed.

| FAIL kind | Count | What it says |
|---|---|---|
| `distribution` | 6 | a locked file differs from the 0.18.0 distribution template |
| `evaluator-payload` | 1 | the installed checker payload is not the one the 0.8.0 lock recorded |
| `lock-entry` | 32 | the 0.8.0 lock's mode or file set differs from 0.18.0's: `managed` where 0.18.0 expects `seed`; `GLOSSARY.md`, `DECISION.template.md` and `RISK.template.md` missing |
| `lock-extra` | 23 | a 0.8.0-locked file is not in the 0.18.0 standard template: the three retired skill trees and the eight scripts |
| `selected-version` | 1 | config, installation and running checker versions must agree |

## Every failing check

```text
distribution:.gitignore: differs from distribution template
distribution:AGENTS.md: differs from distribution template
distribution:CLAUDE.md: differs from distribution template
distribution:ENGINEERING_HARNESS.md: differs from distribution template
distribution:docs/engineering/QUALITY_GATES.json: differs from distribution template
distribution:docs/engineering/WORKFLOW.json: differs from distribution template
evaluator-payload: installed checker payload matches its installation record
lock-entry:.agents/skills/harness-operator-brief/SKILL.md: mode 'managed'; expected 'seed'
lock-entry:.agents/skills/harness-operator-brief/scripts/check_brief.py: mode 'managed'; expected 'seed'
lock-entry:.agents/skills/harness-operator-brief/skill-contract.json: mode 'managed'; expected 'seed'
lock-entry:.agents/skills/harness-orient/SKILL.md: mode 'managed'; expected 'seed'
lock-entry:.agents/skills/harness-orient/scripts/orient.py: mode 'managed'; expected 'seed'
lock-entry:.agents/skills/harness-orient/skill-contract.json: mode 'managed'; expected 'seed'
lock-entry:.claude/skills/harness-orient/SKILL.md: mode 'managed'; expected 'seed'
lock-entry:.engineering-harness.toml: mode 'managed'; expected 'seed'
lock-entry:.github/workflows/engineering-harness.yml: mode 'managed'; expected 'seed'
lock-entry:GLOSSARY.md: missing
lock-entry:docs/engineering/ARTIFACT_AUTHORING.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/DECISION_RIGHTS.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/OPERATING_CARD.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/QUALITY_GATES.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/TECHNICAL_COMMUNICATION.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/TRACEABILITY.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/WORKFLOW.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/ADR.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/ARCHITECTURE.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/CAPABILITY.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/DECISION.template.md: missing
lock-entry:docs/engineering/templates/INTENT.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/OPERATING_CONTRACT.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/README.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/RELEASE_CONTRACT.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/RELEASE_RECORD.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/REQUIREMENT.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/RISK.template.md: missing
lock-entry:docs/engineering/templates/SPECIFICATION.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/VERIFICATION.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/VERIFICATION_RECORD.template.md: mode 'managed'; expected 'seed'
lock-entry:docs/engineering/templates/WORK_ORDER.template.md: mode 'managed'; expected 'seed'
lock-extra:.agents/skills/harness-draft-change/SKILL.md: not in standard template
lock-extra:.agents/skills/harness-draft-change/agents/openai.yaml: not in standard template
lock-extra:.agents/skills/harness-draft-change/scripts/guard.py: not in standard template
lock-extra:.agents/skills/harness-draft-change/skill-contract.json: not in standard template
lock-extra:.agents/skills/harness-execute-work-order/SKILL.md: not in standard template
lock-extra:.agents/skills/harness-execute-work-order/agents/openai.yaml: not in standard template
lock-extra:.agents/skills/harness-execute-work-order/scripts/check_scope.py: not in standard template
lock-extra:.agents/skills/harness-execute-work-order/skill-contract.json: not in standard template
lock-extra:.agents/skills/harness-prepare-assurance/SKILL.md: not in standard template
lock-extra:.agents/skills/harness-prepare-assurance/agents/openai.yaml: not in standard template
lock-extra:.agents/skills/harness-prepare-assurance/scripts/check_prepare.py: not in standard template
lock-extra:.agents/skills/harness-prepare-assurance/skill-contract.json: not in standard template
lock-extra:.claude/skills/harness-draft-change/SKILL.md: not in standard template
lock-extra:.claude/skills/harness-execute-work-order/SKILL.md: not in standard template
lock-extra:.claude/skills/harness-prepare-assurance/SKILL.md: not in standard template
lock-extra:scripts/artifact_layout_registry.py: not in standard template
lock-extra:scripts/check_engineering_harness.ps1: not in standard template
lock-extra:scripts/check_engineering_harness.sh: not in standard template
lock-extra:scripts/generate_harness_dashboard.py: not in standard template
lock-extra:scripts/harness_explorer/index.template.html: not in standard template
lock-extra:scripts/inspect_engineering_artifacts.py: not in standard template
lock-extra:scripts/select_harness_work_order.py: not in standard template
lock-extra:scripts/validate_engineering_artifacts.py: not in standard template
selected-version: config, installation and running checker versions must agree
```
