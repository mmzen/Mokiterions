# B1 - the refusal is real, and enumerated

Captured 2026-08-28 at `330c086`, **before** any repair. The set is every work order in a state the
adopted evaluator treats as authority-granting that has not reached `implemented` — derived from artifact
metadata, not from the two anyone happened to notice. It has **two** members.

Evaluator: se-harness 0.8.0 from the public index, outside the checkout, `python -I -m se_harness`.

## WO-MOK-026 (`approved`)

    harnessctl check . --artifact WO-MOK-026 --checkpoint start

```text
Outcome
Blocked.
Blocked by
- QGP-G3-SCOPE: WO-MOK-026 has no assessable execution scope.
Next
Escalate QGP-G3-SCOPE under DR-WO-SELECT (PROC-WO-START/STEP-WO-START-PREFLIGHT).
```

## WO-MOK-027 (`approved`)

    harnessctl check . --artifact WO-MOK-027 --checkpoint start

```text
Outcome
Blocked.
Blocked by
- QGP-G3-SCOPE: WO-MOK-027 has no assessable execution scope.
Next
Escalate QGP-G3-SCOPE under DR-WO-SELECT (PROC-WO-START/STEP-WO-START-PREFLIGHT).
```

## Reading

Both refuse, and for the same reason: each was approved on 2026-08-23 under the 0.4.0 work-order template,
which carried no `[execution_scope]` table. These two are the **only** work orders in the repository that
are authorized and unstarted, so at this commit every piece of authorized forward work is frozen.

At the same commit `validate` reports **0 errors**. The condition is invisible to validation.
