# A3 - the refusal is real

Captured 2026-08-28 at `0970363`, with the chain present but **unapproved**, so the
`[evaluator_upgrade]` packet in `WO-HUP-001` has no force under `SPEC-HUP-001` rule 4.

Evaluator: se-harness 0.8.0, installed from the public index into
`C:/Users/mathi/se-harness-eval-080`, outside this checkout.

## The plan warns

    harnessctl upgrade .

```text
notice: these released records predate evaluator-evidence enforcement and are not declared; applying an evaluator identity transition would be refused:
  RLS-MOK-001
declare them in an approved work order under [evaluator_upgrade].legacy_releases_without_evaluator_evidence
```

## The apply refuses, before any write

    harnessctl upgrade --apply .

```text
harnessctl: released records predate evaluator-evidence enforcement and are not declared; no files were written: RLS-MOK-001; declare them in an approved work order under [evaluator_upgrade].legacy_releases_without_evaluator_evidence
```

Exit status of the apply: 2

The working tree is unchanged by both commands:

```text
?? docs/engineering/harness/
```
