# N2 - the root is internally consistent

Evaluator: se-harness 0.18.0 from the exact public wheel, outside the checkout, `python -I -m se_harness`. Captured 2026-09-15 on the working tree of branch `harness/wo-hup-003-adopt-se-harness-0.18.0` after all three steps and the three hand edits, before the implementation commit.

**Outcome `completed`: 64 checks, 64 PASS, 0 FAIL.** Against 63 FAIL of 93 before the transaction (`pre-doctor.md`).

| Check kind | Count |
|---|---|
| `AGENTS.md` | 1 |
| `CLAUDE.md` | 1 |
| `ENGINEERING_HARNESS.md` | 1 |
| `claude-import` | 1 |
| `config` | 1 |
| `distribution` | 7 |
| `docs/engineering/ARTIFACT_AUTHORING.md` | 1 |
| `docs/engineering/DECISION_RIGHTS.md` | 1 |
| `docs/engineering/OPERATING_CARD.md` | 1 |
| `docs/engineering/QUALITY_GATES.json` | 1 |
| `docs/engineering/QUALITY_GATES.md` | 1 |
| `docs/engineering/README.md` | 1 |
| `docs/engineering/TECHNICAL_COMMUNICATION.md` | 1 |
| `docs/engineering/TRACEABILITY.md` | 1 |
| `docs/engineering/WORKFLOW.json` | 1 |
| `docs/engineering/WORKFLOW.md` | 1 |
| `evaluator-payload` | 1 |
| `lock` | 1 |
| `managed` | 7 |
| `python` | 1 |
| `seed` | 27 |
| `selected-version` | 1 |
| `skill-ownership` | 1 |
| `hash-bound-class-declared` | 1 |
| `hash-bound-attribute-effective` | 1 |
| `hash-bound-mode-consistent` | 1 |

    harnessctl doctor .

```text
PASS AGENTS.md: required
PASS CLAUDE.md: required
PASS ENGINEERING_HARNESS.md: required
PASS claude-import: @AGENTS.md
PASS config: .engineering-harness.toml
PASS distribution:.gitattributes: matches distribution
PASS distribution:.gitignore: matches distribution
PASS distribution:AGENTS.md: matches distribution
PASS distribution:CLAUDE.md: matches distribution
PASS distribution:ENGINEERING_HARNESS.md: matches distribution
PASS distribution:docs/engineering/QUALITY_GATES.json: matches distribution
PASS distribution:docs/engineering/WORKFLOW.json: matches distribution
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
PASS evaluator-payload: installed checker payload matches its installation record
PASS lock: .engineering-harness.lock
PASS managed:.gitattributes: unchanged
PASS managed:.gitignore: unchanged
PASS managed:AGENTS.md: unchanged
PASS managed:CLAUDE.md: unchanged
PASS managed:ENGINEERING_HARNESS.md: unchanged
PASS managed:docs/engineering/QUALITY_GATES.json: unchanged
PASS managed:docs/engineering/WORKFLOW.json: unchanged
PASS python: 3.14.6
PASS seed:.engineering-harness.toml: present
PASS seed:.github/PULL_REQUEST_TEMPLATE.md: present
PASS seed:.github/workflows/engineering-harness.yml: present
PASS seed:GLOSSARY.md: present
PASS seed:docs/engineering/ARTIFACT_AUTHORING.md: present
PASS seed:docs/engineering/DECISION_RIGHTS.md: present
PASS seed:docs/engineering/OPERATING_CARD.md: present
PASS seed:docs/engineering/QUALITY_GATES.md: present
PASS seed:docs/engineering/README.md: present
PASS seed:docs/engineering/TECHNICAL_COMMUNICATION.md: present
PASS seed:docs/engineering/TRACEABILITY.md: present
PASS seed:docs/engineering/WORKFLOW.md: present
PASS seed:docs/engineering/templates/ADR.template.md: present
PASS seed:docs/engineering/templates/ARCHITECTURE.template.md: present
PASS seed:docs/engineering/templates/CAPABILITY.template.md: present
PASS seed:docs/engineering/templates/DECISION.template.md: present
PASS seed:docs/engineering/templates/INTENT.template.md: present
PASS seed:docs/engineering/templates/OPERATING_CONTRACT.template.md: present
PASS seed:docs/engineering/templates/README.md: present
PASS seed:docs/engineering/templates/RELEASE_CONTRACT.template.md: present
PASS seed:docs/engineering/templates/RELEASE_RECORD.template.md: present
PASS seed:docs/engineering/templates/REQUIREMENT.template.md: present
PASS seed:docs/engineering/templates/RISK.template.md: present
PASS seed:docs/engineering/templates/SPECIFICATION.template.md: present
PASS seed:docs/engineering/templates/VERIFICATION.template.md: present
PASS seed:docs/engineering/templates/VERIFICATION_RECORD.template.md: present
PASS seed:docs/engineering/templates/WORK_ORDER.template.md: present
PASS selected-version: config, installation and running checker versions must agree
PASS skill-ownership: skills are provided by the plugin
PASS hash-bound-class-declared: 2 file formats declared; consuming formats validate their own fields
PASS hash-bound-attribute-effective: 1 raw classes effective for 5 tracked paths
PASS hash-bound-mode-consistent: one mode per class: evaluator-evidence=raw, standard-lock=utf8-text-lf-v1
```
