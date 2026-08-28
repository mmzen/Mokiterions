# WO-HUP-002 completion summary

Every already-approved work order in this repository is startable again under the adopted 0.8.0 root. This
follows the work order's *Completion report format*.

## 1. The enumerated set, before and after

The set is every work order in a state the evaluator treats as authority-granting that has not reached
`implemented`, derived from artifact metadata rather than from the two anyone happened to notice. It has two
members.

| Work order | Before | After |
|---|---|---|
| `WO-MOK-026` | `QGP-G3-SCOPE: has no assessable execution scope` | **Completed** — `DR-WO-START` offered |
| `WO-MOK-027` | `QGP-G3-SCOPE: has no assessable execution scope` | **Completed** |

Refusals before: **2**. Refusals after: **0**. That is `REQ-HUP-002`'s measure, and `B2` carries it.

The refusals were captured before the repair, not reconstructed after it, because a repaired tree cannot
produce them again.

## 2. The surface-to-scope mapping

Retained in full as `p1-p2-surface-to-scope.md`, item by item for both work orders. `WO-MOK-026` takes 15
paths, `WO-MOK-027` takes 5. Every surface item maps to an admitting path (`P1`) and every path is claimed by a
surface item (`P2`).

`P2` does real work on `WO-MOK-027`, whose surface states positively that it changes no Rust source and not
`SPEC-MOK-004`. Its scope admits no path under either package, which makes that claim enforceable instead of
merely written down.

## 3. The three owner decisions

Not derivations, and labelled as decisions everywhere they appear:

1. `mokiterions-core/Cargo.toml` is inside `WO-MOK-026`'s scope — the canned connector must be a real child
   process and therefore a declared target.
2. The connector protocol document lives at `docs/CONNECTOR_PROTOCOL.md`.
3. `WO-MOK-027`'s comparison report lives at `docs/PHASE_5_MEASUREMENT.md`.

The third was **not** anticipated when this work order was approved. It exists because the work order's own
stop-and-escalate condition fired: the text said a third judgment "is not this work order's to take", the
report's path was exactly such a judgment, and it was escalated to the owner and decided rather than invented.
`VER-HUP-002`'s residual uncertainty was drafted saying two boundaries and was corrected to three before
approval was committed.

## 4. Validate

**0 errors. 142 warnings**, stated by code rather than by family:

```text
   5 [W-AUT-001]   34 [W-AUT-002]   25 [W-AUT-003]   77 [W-AUT-004]   1 [W024]
```

`B5` requires the total the evaluator prints, precisely because `VREC-HUP-001`'s `a1-validate.md` grepped only
the `W-AUT-` codes and reported 141. That erratum is not corrected here — a verified record admits an
additional record, never a correction — and it remains separately owed.

## 5. The change surface

No file under `mokiterions-core/` or `mokiterions-tui/`, and no Cargo or toolchain manifest, is touched.
Naming `mokiterions-core/Cargo.toml` inside another work order's scope is not editing it.

Four governed artifacts are modified, each with an amendment record: `WO-MOK-026`, `WO-MOK-027`, `SPEC-HUP-001`
and `CAP-HUP-001`. One relation moves — `SPEC-HUP-001`'s `specifies`, widened to name `REQ-HUP-002`, without
which that requirement is active with no specification coverage and validation fails `E007`. It is disclosed in
that specification's own amendment record rather than left for a reader to find in a diff.

Not one line of the connector, the live path, the gates, the accounting or the ceiling is written here. This
work order changed what two work orders **declare**, never what they deliver.

## 6. The finding against the preceding chain

> The 0.4.0-to-0.8.0 adoption reported complete success — `validate` 0 errors, `doctor` 0 FAIL, and all eleven
> of `VER-HUP-001`'s assessments passing — while leaving both of this repository's approved work orders
> unstartable. Nothing in that chain was wrong. Nothing in it looked.

The condition is invisible to validation by construction: an approved work order with no `[execution_scope]`
has every field its schema requires and every relation resolves. It is visible only to an operator who tries to
begin work, which happens after the adoption is verified and merged.

`SPEC-HUP-001` rule 11 is the obligation that would have caught it, stated where the *next* adoption meets it
rather than only in this repair's evidence. That placement is what `M1` confirms.

## 7. What is left owed

- **The commit-bound verification record** for this work order. `required` assurance is discharged separately.
- **The `W024` erratum** against `VREC-HUP-001`, which needs an additional record and is untouched by this work.
- **`WO-HUP-001`'s missing governance for the branch rename**, unchanged and unrelated to this repair.
- **Later checkpoints.** `VER-HUP-002` checks the `start` checkpoint only. A work order that starts may still
  meet an adoption-introduced refusal at `pre-action`, `transition` or `handoff`, and those are reachable only
  during execution.
