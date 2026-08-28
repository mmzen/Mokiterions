# B3 and B4 - the change surface

## Every path this change touches

```text
docs/engineering/harness/capabilities/CAP-HUP-001.md
docs/engineering/harness/evidence/WO-HUP-002/b1-refusals-before.md
docs/engineering/harness/evidence/WO-HUP-002/b2-starts-after.md
docs/engineering/harness/evidence/WO-HUP-002/b3-b4-change-surface.md
docs/engineering/harness/evidence/WO-HUP-002/b5-validate.md
docs/engineering/harness/requirements/REQ-HUP-002.md
docs/engineering/harness/specifications/SPEC-HUP-001.md
docs/engineering/harness/verification/VER-HUP-002.md
docs/engineering/harness/work-orders/WO-HUP-002.md
docs/engineering/simulation/work-orders/WO-MOK-026.md
docs/engineering/simulation/work-orders/WO-MOK-027.md
```

## B4 - protected paths

```text
mokiterions-core/ : 0
mokiterions-tui/  : 0
Cargo / toolchain manifests: 0
```

Naming `mokiterions-core/Cargo.toml` inside another work order's scope is not editing it.

## B3 - governed artifacts modified, and how

```text
WO-MOK-026.md                                                  status lines changed: 0
WO-MOK-027.md                                                  status lines changed: 0
SPEC-HUP-001.md                                                status lines changed: 0
CAP-HUP-001.md                                                 status lines changed: 0
```

Each carries an amendment record. No `status`, relation or assurance field moves on any of them --
except `SPEC-HUP-001`'s `specifies`, which is widened to name `REQ-HUP-002` and is disclosed in its own
amendment record as required by `E007`.
