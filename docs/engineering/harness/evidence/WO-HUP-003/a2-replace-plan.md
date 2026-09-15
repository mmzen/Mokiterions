# A2 (part 2) - transaction 2's plan, before apply

Evaluator: se-harness 0.18.0 from the exact public wheel, outside the checkout, `python -I -m se_harness`. Captured 2026-09-15 on the working tree of branch `harness/wo-hup-003-adopt-se-harness-0.18.0` after transaction 1 and before transaction 2.

**41 files: 22 unchanged, 19 update. No path is `customized` and none is `conflict`.** Command: `python -I -m se_harness upgrade . --replace-file <path>` with the 19 paths of `replace-list.md`; read-only, `written: false`.

The first transaction accepts no `--replace-file` for these paths, measured at `8ab8a30` as `--replace-file must name a seeded file`, because a file is a seed only once the lock says so, and the first transaction is what says so. Hence two transactions, each planned before it is applied.

```text
update     .github/workflows/engineering-harness.yml
update     docs/engineering/ARTIFACT_AUTHORING.md
update     docs/engineering/DECISION_RIGHTS.md
update     docs/engineering/OPERATING_CARD.md
update     docs/engineering/QUALITY_GATES.md
update     docs/engineering/TRACEABILITY.md
update     docs/engineering/WORKFLOW.md
update     docs/engineering/templates/ADR.template.md
update     docs/engineering/templates/ARCHITECTURE.template.md
update     docs/engineering/templates/CAPABILITY.template.md
update     docs/engineering/templates/INTENT.template.md
update     docs/engineering/templates/README.md
update     docs/engineering/templates/RELEASE_CONTRACT.template.md
update     docs/engineering/templates/RELEASE_RECORD.template.md
update     docs/engineering/templates/REQUIREMENT.template.md
update     docs/engineering/templates/SPECIFICATION.template.md
update     docs/engineering/templates/VERIFICATION.template.md
update     docs/engineering/templates/VERIFICATION_RECORD.template.md
update     docs/engineering/templates/WORK_ORDER.template.md
summary: 41 files, 22 unchanged
```
