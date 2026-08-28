# N2 - the root is internally consistent

    harnessctl doctor .

FAIL lines: 0
PASS lines: 143

```text
PASS AGENTS.md: required
PASS CLAUDE.md: required
PASS ENGINEERING_HARNESS.md: required
PASS claude-import: @AGENTS.md
PASS config: .engineering-harness.toml
PASS distribution:.agents/skills/harness-draft-change/SKILL.md: matches distribution
PASS distribution:.agents/skills/harness-draft-change/agents/openai.yaml: matches distribution
PASS distribution:.agents/skills/harness-draft-change/scripts/guard.py: matches distribution
PASS distribution:.agents/skills/harness-draft-change/skill-contract.json: matches distribution
PASS distribution:.agents/skills/harness-execute-work-order/SKILL.md: matches distribution
PASS distribution:.agents/skills/harness-execute-work-order/agents/openai.yaml: matches distribution
PASS distribution:.agents/skills/harness-execute-work-order/scripts/check_scope.py: matches distribution
PASS distribution:.agents/skills/harness-execute-work-order/skill-contract.json: matches distribution
PASS distribution:.agents/skills/harness-operator-brief/SKILL.md: matches distribution
PASS distribution:.agents/skills/harness-operator-brief/scripts/check_brief.py: matches distribution
PASS distribution:.agents/skills/harness-operator-brief/skill-contract.json: matches distribution
PASS distribution:.agents/skills/harness-orient/SKILL.md: matches distribution
PASS distribution:.agents/skills/harness-orient/scripts/orient.py: matches distribution
PASS distribution:.agents/skills/harness-orient/skill-contract.json: matches distribution
PASS distribution:.agents/skills/harness-prepare-assurance/SKILL.md: matches distribution
PASS distribution:.agents/skills/harness-prepare-assurance/agents/openai.yaml: matches distribution
PASS distribution:.agents/skills/harness-prepare-assurance/scripts/check_prepare.py: matches distribution
PASS distribution:.agents/skills/harness-prepare-assurance/skill-contract.json: matches distribution
PASS distribution:.claude/skills/harness-draft-change/SKILL.md: matches distribution
PASS distribution:.claude/skills/harness-execute-work-order/SKILL.md: matches distribution
PASS distribution:.claude/skills/harness-orient/SKILL.md: matches distribution
PASS distribution:.claude/skills/harness-prepare-assurance/SKILL.md: matches distribution
PASS distribution:.engineering-harness.toml: matches distribution
PASS distribution:.gitattributes: matches distribution
PASS distribution:.github/workflows/engineering-harness.yml: matches distribution
PASS distribution:.gitignore: matches distribution
PASS distribution:AGENTS.md: matches distribution
PASS distribution:CLAUDE.md: matches distribution
PASS distribution:ENGINEERING_HARNESS.md: matches distribution
PASS distribution:docs/engineering/ARTIFACT_AUTHORING.md: matches distribution
PASS distribution:docs/engineering/DECISION_RIGHTS.md: matches distribution
PASS distribution:docs/engineering/OPERATING_CARD.md: matches distribution
PASS distribution:docs/engineering/QUALITY_GATES.json: matches distribution
PASS distribution:docs/engineering/QUALITY_GATES.md: matches distribution
PASS distribution:docs/engineering/TECHNICAL_COMMUNICATION.md: matches distribution
PASS distribution:docs/engineering/TRACEABILITY.md: matches distribution
PASS distribution:docs/engineering/WORKFLOW.json: matches distribution
PASS distribution:docs/engineering/WORKFLOW.md: matches distribution
PASS distribution:docs/engineering/templates/ADR.template.md: matches distribution
PASS distribution:docs/engineering/templates/ARCHITECTURE.template.md: matches distribution
PASS distribution:docs/engineering/templates/CAPABILITY.template.md: matches distribution
PASS distribution:docs/engineering/templates/INTENT.template.md: matches distribution
PASS distribution:docs/engineering/templates/OPERATING_CONTRACT.template.md: matches distribution
PASS distribution:docs/engineering/templates/README.md: matches distribution
PASS distribution:docs/engineering/templates/RELEASE_CONTRACT.template.md: matches distribution
PASS distribution:docs/engineering/templates/RELEASE_RECORD.template.md: matches distribution
PASS distribution:docs/engineering/templates/REQUIREMENT.template.md: matches distribution
PASS distribution:docs/engineering/templates/SPECIFICATION.template.md: matches distribution
PASS distribution:docs/engineering/templates/VERIFICATION.template.md: matches distribution
PASS distribution:docs/engineering/templates/VERIFICATION_RECORD.template.md: matches distribution
PASS distribution:docs/engineering/templates/WORK_ORDER.template.md: matches distribution
PASS distribution:scripts/artifact_layout_registry.py: matches distribution
PASS distribution:scripts/check_engineering_harness.ps1: matches distribution
PASS distribution:scripts/check_engineering_harness.sh: matches distribution
PASS distribution:scripts/generate_harness_dashboard.py: matches distribution
PASS distribution:scripts/harness_explorer/index.template.html: matches distribution
PASS distribution:scripts/inspect_engineering_artifacts.py: matches distribution
PASS distribution:scripts/select_harness_work_order.py: matches distribution
PASS distribution:scripts/validate_engineering_artifacts.py: matches distribution
PASS docs/engineering/ARTIFACT_AUTHORING.md: required
PASS docs/engineering/DECISION_RIGHTS.md: required
PASS docs/engineering/OPERATING_CARD.md: required
PASS docs/engineering/QUALITY_GATES.json: required
PASS docs/engineering/QUALITY_GATES.md: required
PASS docs/engineering/README.md: required
PASS docs/engineering/TECHNICAL_COMMUNICATION.md: required
PASS docs/engineering/TRACEABILITY.md: required
PASS docs/engineering/WORKFLOW.json: required
PASS docs/engineering/WORKFLOW.md: required
PASS lock: .engineering-harness.lock
PASS managed:.agents/skills/harness-draft-change/SKILL.md: unchanged
PASS managed:.agents/skills/harness-draft-change/agents/openai.yaml: unchanged
PASS managed:.agents/skills/harness-draft-change/scripts/guard.py: unchanged
PASS managed:.agents/skills/harness-draft-change/skill-contract.json: unchanged
PASS managed:.agents/skills/harness-execute-work-order/SKILL.md: unchanged
PASS managed:.agents/skills/harness-execute-work-order/agents/openai.yaml: unchanged
PASS managed:.agents/skills/harness-execute-work-order/scripts/check_scope.py: unchanged
PASS managed:.agents/skills/harness-execute-work-order/skill-contract.json: unchanged
PASS managed:.agents/skills/harness-operator-brief/SKILL.md: unchanged
PASS managed:.agents/skills/harness-operator-brief/scripts/check_brief.py: unchanged
PASS managed:.agents/skills/harness-operator-brief/skill-contract.json: unchanged
PASS managed:.agents/skills/harness-orient/SKILL.md: unchanged
PASS managed:.agents/skills/harness-orient/scripts/orient.py: unchanged
PASS managed:.agents/skills/harness-orient/skill-contract.json: unchanged
PASS managed:.agents/skills/harness-prepare-assurance/SKILL.md: unchanged
PASS managed:.agents/skills/harness-prepare-assurance/agents/openai.yaml: unchanged
PASS managed:.agents/skills/harness-prepare-assurance/scripts/check_prepare.py: unchanged
PASS managed:.agents/skills/harness-prepare-assurance/skill-contract.json: unchanged
PASS managed:.claude/skills/harness-draft-change/SKILL.md: unchanged
PASS managed:.claude/skills/harness-execute-work-order/SKILL.md: unchanged
PASS managed:.claude/skills/harness-orient/SKILL.md: unchanged
PASS managed:.claude/skills/harness-prepare-assurance/SKILL.md: unchanged
PASS managed:.engineering-harness.toml: unchanged
PASS managed:.gitattributes: unchanged
PASS managed:.github/workflows/engineering-harness.yml: unchanged
PASS managed:.gitignore: unchanged
PASS managed:AGENTS.md: unchanged
PASS managed:CLAUDE.md: unchanged
PASS managed:ENGINEERING_HARNESS.md: unchanged
PASS managed:docs/engineering/ARTIFACT_AUTHORING.md: unchanged
PASS managed:docs/engineering/DECISION_RIGHTS.md: unchanged
PASS managed:docs/engineering/OPERATING_CARD.md: unchanged
PASS managed:docs/engineering/QUALITY_GATES.json: unchanged
PASS managed:docs/engineering/QUALITY_GATES.md: unchanged
PASS managed:docs/engineering/TECHNICAL_COMMUNICATION.md: unchanged
PASS managed:docs/engineering/TRACEABILITY.md: unchanged
PASS managed:docs/engineering/WORKFLOW.json: unchanged
PASS managed:docs/engineering/WORKFLOW.md: unchanged
PASS managed:docs/engineering/templates/ADR.template.md: unchanged
PASS managed:docs/engineering/templates/ARCHITECTURE.template.md: unchanged
PASS managed:docs/engineering/templates/CAPABILITY.template.md: unchanged
PASS managed:docs/engineering/templates/INTENT.template.md: unchanged
PASS managed:docs/engineering/templates/OPERATING_CONTRACT.template.md: unchanged
PASS managed:docs/engineering/templates/README.md: unchanged
PASS managed:docs/engineering/templates/RELEASE_CONTRACT.template.md: unchanged
PASS managed:docs/engineering/templates/RELEASE_RECORD.template.md: unchanged
PASS managed:docs/engineering/templates/REQUIREMENT.template.md: unchanged
PASS managed:docs/engineering/templates/SPECIFICATION.template.md: unchanged
PASS managed:docs/engineering/templates/VERIFICATION.template.md: unchanged
PASS managed:docs/engineering/templates/VERIFICATION_RECORD.template.md: unchanged
PASS managed:docs/engineering/templates/WORK_ORDER.template.md: unchanged
PASS managed:scripts/artifact_layout_registry.py: unchanged
PASS managed:scripts/check_engineering_harness.ps1: unchanged
PASS managed:scripts/check_engineering_harness.sh: unchanged
PASS managed:scripts/generate_harness_dashboard.py: unchanged
PASS managed:scripts/harness_explorer/index.template.html: unchanged
PASS managed:scripts/inspect_engineering_artifacts.py: unchanged
PASS managed:scripts/select_harness_work_order.py: unchanged
PASS managed:scripts/validate_engineering_artifacts.py: unchanged
PASS python: 3.14.6
PASS scripts/generate_harness_dashboard.py: required
PASS scripts/harness_explorer/index.template.html: required
PASS scripts/validate_engineering_artifacts.py: required
PASS seed:.github/PULL_REQUEST_TEMPLATE.md: present
PASS seed:docs/engineering/README.md: present
PASS hash-bound-class-declared: 2 classes cover 1 tracked paths; 10 digest fields declared out of scope; vacuously declared evaluator-evidence: 0 tracked paths
PASS hash-bound-attribute-effective: 1 raw classes effective for 0 tracked paths
PASS hash-bound-mode-consistent: one mode per class: evaluator-evidence=raw, standard-lock=utf8-text-lf-v1
```
