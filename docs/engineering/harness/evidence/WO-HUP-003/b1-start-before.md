# B1 - the refusal is real, enumerated, and captured before the transaction

Captured 2026-09-15 at `654e3f6`, before any write. Evaluator: se-harness 0.18.0 from the exact public wheel, outside the checkout, `python -I -m se_harness`.

## The set

Every work order in a state the adopted evaluator treats as authority-granting that has not reached `implemented`, read from artifact metadata across both domains. It has **one** member.

| Work order | Status | Lifecycle events |
|---|---|---|
| `WO-MOK-027` | `approved` | none (approved 2026-08-23 under 0.4.0, which wrote none) |

Every other work order is `implemented` or `verified`, and `WO-HUP-003` is this adoption's own.

## WO-MOK-027, before the transaction

    harnessctl check . --artifact WO-MOK-027 --checkpoint start

```text
Outcome
Blocked.

Done
- Evaluated start compliance for WO-MOK-027.

Not done
- The start checkpoint did not pass.

Blocked by
- QGP-G3-SCOPE: WEX-ECP-022: WO-MOK-027.md has no recorded engineering-owner approval
- QGP-G3-PREFLIGHT: mode 'managed'; expected 'seed'

Current lifecycle state
- WO-MOK-027 is approved.

Decision required
None.

Next
Escalate QGP-G3-SCOPE under DR-WO-SELECT (PROC-WO-START/STEP-WO-START-PREFLIGHT).

Command or response
Escalate to the accountable owner under DR-WO-SELECT: WEX-ECP-022: WO-MOK-027.md has no recorded engineering-owner approval

Change set
None.
complete: false

Gates
QGP-G3-STATUS: pass
QGP-G3-GRAPH: pass
QGP-G3-INTEGRITY: pass
QGP-G3-SCOPE: fail
QGP-G3-PREFLIGHT: fail
QGP-G3-DECISION: pass
```

## Reading

Two refusals. `QGP-G3-PREFLIGHT: mode 'managed'; expected 'seed'` is the unupgraded root seen by the newer evaluator and is expected to clear with the transaction. `QGP-G3-SCOPE: WEX-ECP-022: WO-MOK-027.md has no recorded engineering-owner approval` is the one `SPEC-HUP-001` rule 11 exists to find: 0.18.0 reads the execution grant from the approval event itself (`to = "approved"`, `decided_by = "engineering-owner"`, `scope_paths`), and this work order has no event. The readings after the transaction and after the repair are in `b2-start-after.md`.
