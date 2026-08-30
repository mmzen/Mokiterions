# Handoff checkpoint evidence

Retained for the `in_progress` to `implemented` transition of `WO-MOK-031`.

artifact: WO-MOK-031
checkpoint: handoff
formal_snapshot_sha256: d718a77527d6722098f10fba37dc1c10e87308db10bf92a53e64621ba4ffb0d5

No measurement is taken and no figure originates here: this work order amends one verification
contract and one approved work order, and runs nothing. The binding is required of every work order
whatever its assurance classification, and this one's is `not_required`.

## The declared change set

```text
  docs/engineering/simulation/evidence/WO-MOK-031/handoff-check.md
  docs/engineering/simulation/verification/VER-MOK-018.md
  docs/engineering/simulation/work-orders/WO-MOK-027.md
  docs/engineering/simulation/work-orders/WO-MOK-031.md
```

No Rust file appears in it, and `cargo` was not re-run. This file is listed because it is part of the
change even though it is the file carrying the binding; the formal snapshot is taken over the
repository's artifacts and not over retained evidence, so naming it here does not move the digest
above.
