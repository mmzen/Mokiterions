# The 19 seeds named to `--replace-file`

Every adopted file was compared, byte for byte after line-ending normalization and template substitution, with its 0.8.0 distribution template and its 0.18.0 distribution template, on 2026-09-15 at `8ab8a30`. **All 28 adopted files equal their 0.8.0 template**, so none carries a repository edit and replacing one loses nothing. 21 differ from their 0.18.0 template; two of those are `harness-orient` skill files that the skill switch removes; the 19 below were named.

| Path | Equals its 0.8.0 template | Equals its 0.18.0 template before replacement |
|---|---|---|
| `.github/workflows/engineering-harness.yml` | yes | no |
| `docs/engineering/ARTIFACT_AUTHORING.md` | yes | no |
| `docs/engineering/DECISION_RIGHTS.md` | yes | no |
| `docs/engineering/OPERATING_CARD.md` | yes | no |
| `docs/engineering/QUALITY_GATES.md` | yes | no |
| `docs/engineering/TRACEABILITY.md` | yes | no |
| `docs/engineering/WORKFLOW.md` | yes | no |
| `docs/engineering/templates/ADR.template.md` | yes | no |
| `docs/engineering/templates/ARCHITECTURE.template.md` | yes | no |
| `docs/engineering/templates/CAPABILITY.template.md` | yes | no |
| `docs/engineering/templates/INTENT.template.md` | yes | no |
| `docs/engineering/templates/README.md` | yes | no |
| `docs/engineering/templates/RELEASE_CONTRACT.template.md` | yes | no |
| `docs/engineering/templates/RELEASE_RECORD.template.md` | yes | no |
| `docs/engineering/templates/REQUIREMENT.template.md` | yes | no |
| `docs/engineering/templates/SPECIFICATION.template.md` | yes | no |
| `docs/engineering/templates/VERIFICATION.template.md` | yes | no |
| `docs/engineering/templates/VERIFICATION_RECORD.template.md` | yes | no |
| `docs/engineering/templates/WORK_ORDER.template.md` | yes | no |

Not named, because they already equal their 0.18.0 text: `docs/engineering/TECHNICAL_COMMUNICATION.md`, `docs/engineering/templates/OPERATING_CONTRACT.template.md`, the three `harness-operator-brief` files, `.claude/skills/harness-orient/SKILL.md` and `.agents/skills/harness-orient/skill-contract.json`.

Not named, because they carry or will carry repository content: `.engineering-harness.toml`, `docs/engineering/README.md`, `.github/PULL_REQUEST_TEMPLATE.md`, `GLOSSARY.md`.

After the second transaction every one of the 19 equals its 0.18.0 template, measured the same way.
