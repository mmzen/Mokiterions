# Handoff checkpoint evidence

Retained for the `in_progress` to `implemented` transition of `WO-HUP-002`.

artifact: WO-HUP-002
checkpoint: handoff
formal_snapshot_sha256: af9f6113bfc956a6870bd0f88191d81fcd47646e35bb8ce4da24965b8dba32a4

The digest binds the artifact graph as it stood when the handoff check was evaluated. It moved from
`WO-HUP-001`'s `b72a268a` because this work order amends four governed artifacts and adds three, which is
what a formal snapshot is for. The snapshot covers governed artifacts and not retained evidence, so this
file does not move the digest it names.

## The declared change set

Every path asserted complete, and every one admitted by `WO-HUP-002`'s own `[execution_scope].paths`:
`docs/engineering/harness/`, plus the two simulation work orders it amends and nothing else.

```text
  docs/engineering/harness/capabilities/CAP-HUP-001.md
  docs/engineering/harness/evidence/WO-HUP-002/b1-refusals-before.md
  docs/engineering/harness/evidence/WO-HUP-002/b2-starts-after.md
  docs/engineering/harness/evidence/WO-HUP-002/b3-b4-change-surface.md
  docs/engineering/harness/evidence/WO-HUP-002/b5-validate.md
  docs/engineering/harness/evidence/WO-HUP-002/completion-summary.md
  docs/engineering/harness/evidence/WO-HUP-002/m1-cause-recorded.md
  docs/engineering/harness/evidence/WO-HUP-002/p1-p2-surface-to-scope.md
  docs/engineering/harness/requirements/REQ-HUP-002.md
  docs/engineering/harness/specifications/SPEC-HUP-001.md
  docs/engineering/harness/verification/VER-HUP-002.md
  docs/engineering/harness/work-orders/WO-HUP-002.md
  docs/engineering/simulation/work-orders/WO-MOK-026.md
  docs/engineering/simulation/work-orders/WO-MOK-027.md
```
