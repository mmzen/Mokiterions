# B2 - the enumerated set, after the transaction and after the repair

Evaluator: se-harness 0.18.0 from the exact public wheel, outside the checkout, `python -I -m se_harness`. Captured 2026-09-15 on the working tree of branch `harness/wo-hup-003-adopt-se-harness-0.18.0` at three moments.

The set is `b1-start-before.md`'s: one member, `WO-MOK-027`.

## After transaction 1, before any hand edit

The `QGP-G3-PREFLIGHT` refusal is gone with the root; `WEX-ECP-022` survives the adoption, which is what `SPEC-HUP-001` rule 11 predicts and `REQ-HUP-002` forbids leaving in place.

```text
Outcome
Blocked.

Done
- Evaluated start compliance for WO-MOK-027.

Not done
- The start checkpoint did not pass.

Blocked by
- QGP-G3-SCOPE: WEX-ECP-022: WO-MOK-027.md has no recorded engineering-owner approval

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
QGP-G3-PREFLIGHT: pass
QGP-G3-DECISION: pass
```

## After transaction 2 and the skill switch, before any hand edit

```text
Outcome
Blocked.

Done
- Evaluated start compliance for WO-MOK-027.

Not done
- The start checkpoint did not pass.

Blocked by
- QGP-G3-SCOPE: WEX-ECP-022: WO-MOK-027.md has no recorded engineering-owner approval

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
QGP-G3-PREFLIGHT: pass
QGP-G3-DECISION: pass
```

## After the repair

One scoped approval event on `WO-MOK-027`, decided by the owner on 2026-09-15 and recorded with an amendment record; see `b3-p1-p2-wo-mok-027.md`.

```text
Outcome
Completed.

Done
- Evaluated start compliance for WO-MOK-027.

Not done
None.

Current lifecycle state
- WO-MOK-027 is approved.

Decision required
None.

Next
Run the bound command (PROC-WO-START/STEP-WO-START-PREVIEW).

Command or response
harnessctl transition . --set WO-MOK-027=in_progress --decision WO-MOK-027=delegated-executor

Change set
None.
complete: false

Gates
QGP-G3-STATUS: pass
QGP-G3-GRAPH: pass
QGP-G3-INTEGRITY: pass
QGP-G3-SCOPE: pass
QGP-G3-PREFLIGHT: pass
QGP-G3-DECISION: pass
```

**Refusals before: 2 (one of them the unupgraded root). After the transaction: 1. After the repair: 0.** `REQ-HUP-002`'s measure is the last figure.
