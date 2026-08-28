+++
id = "WO-HUP-002"
type = "work_order"
title = "Give WO-MOK-026 and WO-MOK-027 the execution scope 0.8.0 requires, and oblige the next adoption to check for what froze them"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-28"
updated = "2026-08-28"

[assurance]
commit_bound_verification = "required"
rationale = "This work amends two approved work orders that authorize every remaining piece of forward work in this repository, and an execution scope is not documentation: it is the boundary the evaluator enforces at the handoff checkpoint, so a scope that is too narrow refuses legitimate work and a scope that is too wide silently stops being a boundary at all. Neither failure is visible by inspecting the diff. That every affected work order starts afterwards is a claim about an enumerated set evaluated by the evaluator, not about the two files anyone thought to change. That each amended scope admits exactly the surface its own work order already describes is a mapping between prose and paths that only an explicit item-by-item reading can settle. The work also amends a specification that a verified chain was measured against, so the artifact a later reader would cite as the oracle is itself part of what changes."
decided_by = "engineering owner"

[execution_scope]
paths = [
  "docs/engineering/harness/",
  "docs/engineering/simulation/work-orders/WO-MOK-026.md",
  "docs/engineering/simulation/work-orders/WO-MOK-027.md",
]

[relations]
implements = ["REQ-HUP-002"]
specifications = ["SPEC-HUP-001"]
verification = ["VER-HUP-002"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T21:05:00Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "approved"
to = "in_progress"
decided_at = "2026-08-28T21:11:22Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "in_progress"
to = "implemented"
decided_at = "2026-08-28T21:12:11Z"
decided_by = "engineering owner"
+++

# Work Order: repair the execution scope the 0.8.0 adoption required

## Lifecycle

Governance work. `approved` authorizes the amendments; `implemented` follows them and the retained evidence.
Commit-bound verification is `required`, so a verification record binding an exact candidate is a separate,
later act. Under 0.8.0 the move to `implemented` is two transitions, through `in_progress`.

## Objective

Make every already-approved work order startable under the adopted 0.8.0 root, and state the obligation that
would have caught the condition, so that the next adoption is required to look rather than required to be
lucky.

## In scope

1. **`WO-MOK-026` and `WO-MOK-027` each gain an `[execution_scope].paths` table**, derived item by item from
   that work order's own *Expected change surface*, with a trailing amendment record in each stating what was
   added, by whose decision, and on what date. Nothing else in either artifact moves — not `status`, not a
   relation, not an assurance field, not a word of the scope prose the table is derived from.
2. **`SPEC-HUP-001` gains rule 11**, obliging an adoption to evaluate the start checkpoint of every
   authority-granting, unimplemented work order under the adopted evaluator, and to surface every refusal. Added
   in place with an amendment record, as this repository amends approved specifications.
3. **The two boundaries the prose could not settle**, recorded as decisions rather than derivations:
   `mokiterions-core/Cargo.toml` is inside `WO-MOK-026`'s scope, because the canned connector must be a real
   child process and therefore a declared target; the connector protocol document lives at
   `docs/CONNECTOR_PROTOCOL.md`, beside the other repository-owned documents under `docs/`; and `WO-MOK-027`'s
   comparison report, which its surface calls only "a new document", lives at `docs/PHASE_5_MEASUREMENT.md`.
   All three were decided by the owner on 2026-08-28. The third was reached by this work order's own stop
   condition firing rather than by inference: `WO-MOK-027`'s text does not support a path, so one was not
   invented.
4. **The evidence `VER-HUP-002` contracts**, including the enumerated before-and-after start-checkpoint readings.

## Out of scope

- **Every product change.** No file under `mokiterions-core/` or `mokiterions-tui/`, and no Cargo or toolchain
  manifest, is touched. Naming `mokiterions-core/Cargo.toml` *inside another work order's scope* is not editing
  it.
- **Doing any of `WO-MOK-026`'s or `WO-MOK-027`'s work.** This work order changes what those two declare, never
  what they deliver. Not one line of the connector, the live path, the gates, the accounting or the ceiling is
  written here.
- **Amending `VER-HUP-001` or `VREC-HUP-001`.** The first is attested by the second, and the second is verified
  and terminal. The finding against them is recorded in `REQ-HUP-002`'s rationale, not by editing them.
- **Correcting `VREC-HUP-001`'s warning-count evidence.** That erratum is real and is separately owed; a
  verified record admits an additional record, never a correction.
- **Re-opening the 0.8.0 adoption.** `WO-HUP-001` is `implemented` and its chain is verified. This is a
  successor, not a supersession.

## Authorized decision envelope

The engineering owner may, without further authorization: choose evidence paths below
`docs/engineering/harness/evidence/WO-HUP-002/`; and word the amendment records and summaries.

The engineering owner may **not**, under this work order: widen either amended scope beyond what that work
order's own *Expected change surface* describes; change any field of `WO-MOK-026` or `WO-MOK-027` other than
adding the scope table and its amendment record; amend any artifact not named in item 2; or begin the work
either amended work order authorizes.

## Constraints

- A derived scope is derived. Every path in it traces to an item of that work order's own surface text, and the
  mapping is retained. The two exceptions are the owner decisions of item 3, and they are labelled as such.
- A scope that admits more than its work order describes defeats the boundary it exists to be, and `P2` exists
  to catch exactly that.
- The evaluator is the released 0.8.0 one, outside the checkout, invoked as `python -I -m se_harness`.

## Expected change surface

- `docs/engineering/simulation/work-orders/WO-MOK-026.md` — an `[execution_scope]` table and an amendment
  record.
- `docs/engineering/simulation/work-orders/WO-MOK-027.md` — the same.
- `docs/engineering/harness/specifications/SPEC-HUP-001.md` — rule 11 and an amendment record.
- `docs/engineering/harness/requirements/REQ-HUP-002.md`, `verification/VER-HUP-002.md`,
  `work-orders/WO-HUP-002.md` — this chain.
- `docs/engineering/harness/evidence/WO-HUP-002/` — the retained evidence.

## Required verification

`VER-HUP-002` in full: B1 through B5, P1, P2 and M1. B2 carries `REQ-HUP-002`.

B1 requires the refusals to be **enumerated before the repair**, over every authority-granting unimplemented
work order rather than over the two anyone happened to notice.

## Evidence to record

Under `docs/engineering/harness/evidence/WO-HUP-002/`, as `VER-HUP-002`'s *Evidence retention* enumerates. The
completion summary additionally records, as a stated finding against the preceding chain:

> The 0.4.0-to-0.8.0 adoption reported complete success — `validate` 0 errors, `doctor` 0 FAIL, and all eleven
> of `VER-HUP-001`'s assessments passing — while leaving both of this repository's approved work orders
> unstartable. Nothing in that chain was wrong. Nothing in it looked.

## Stop and escalate conditions

Stop and return to the engineering owner when:

- the enumeration of B1 finds a refused work order beyond `WO-MOK-026` and `WO-MOK-027`;
- a refusal survives the amendment, or a new refusal appears at a later checkpoint;
- an item of either work order's surface text cannot be mapped to a path without a judgment the work order does
  not support — item 3's two decisions are the known ones, and a third is not this work order's to take; or
- repairing a refusal would require changing a field this work order does not authorize.

## Completion report format

1. The enumerated set of work orders checked, and each one's reading before and after.
2. The surface-to-scope mapping for each amended work order, item by item.
3. The two owner decisions of item 3, labelled as decisions.
4. `validate`, with the full error and warning counts.
5. The change surface, against the out-of-scope directories.
6. The finding against `WO-HUP-001`'s chain, stated above.
7. What is left owed: the commit-bound verification record, and the separate `W024` erratum.
