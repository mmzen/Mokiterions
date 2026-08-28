+++
id = "WO-MOK-029"
type = "work_order"
title = "Name the three command-line options SPEC-MOK-007 describes eleven times and never names"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-28"
updated = "2026-08-28"

[assurance]
commit_bound_verification = "not_required"
rationale = "This work order changes no executable behavior. It states three option names a specification refers to descriptively and never fixes, so that the host implementing them is implementing something written down rather than something chosen at the keyboard. Nothing is measured, nothing is run, and no published figure moves. The claim a later reader needs is that the amended text names what it names, which the diff establishes by inspection. `WO-MOK-026` implements against these names and its assurance is `required`; a commit-bound record here would bind evidence about a document rather than about behaviour."
decided_by = "engineering owner"

[execution_scope]
paths = [
  "docs/engineering/simulation/evidence/WO-MOK-029/",
  "docs/engineering/simulation/specifications/SPEC-MOK-007.md",
  "docs/engineering/simulation/work-orders/WO-MOK-029.md",
]

[relations]
implements = ["REQ-MOK-071", "REQ-MOK-072", "REQ-MOK-077"]
specifications = ["SPEC-MOK-007"]
verification = ["VER-MOK-018"]
architecture = ["ARCH-MOK-001", "ARCH-MOK-002", "ADR-MOK-007"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T21:44:33Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "approved"
to = "in_progress"
decided_at = "2026-08-28T21:45:19Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "in_progress"
to = "implemented"
decided_at = "2026-08-28T21:45:38Z"
decided_by = "engineering owner"
+++

# Work Order: name the three unnamed options

## Lifecycle

Governance work, stopping at `implemented`. Assurance is `not_required` for the reason its rationale gives: this
work order amends a document and runs nothing. Under 0.8.0 the move to `implemented` is two transitions, through
`in_progress`, and the handoff checkpoint requires a snapshot binding whatever the assurance classification —
which `WO-MOK-028` learned by being refused, and which this work order's scope admits an evidence directory for.

## Objective

Fix the names of the connector path option, the live-mode selection and the spend ceiling, so that `WO-MOK-026`
implements a written contract rather than a choice made at the keyboard.

## Why this exists, and why it is not `WO-MOK-028`

`SPEC-MOK-007` names two options, `--transcript-path` and `--transcript-output`. It refers to three others
**eleven times** — "the connector path" at lines 93, 539, 703, 709, 716 and 744, "a live-mode selection" or
"the live-mode selection" at 572, 716, 718 and 754, and "the spend ceiling" at 669 — and names none of them.
Rule 18.4.2 lists five options a host acts on and spells only two.

An unnamed option is the same defect as an unnamed field, answered the same way. The operator types these; a
reader of the specification cannot tell what to type; and two implementations of this specification would offer
different command lines while both conforming.

This is a **fourth** thing `SPEC-MOK-007` left open, after the three `WO-MOK-028` closed. That work order's
stop-and-escalate conditions name this exact case — *"a fourth thing turns out to be unspecified, rather than
being folded in silently"* — and it fired while its successor's parser work was starting. `WO-MOK-028` is
`implemented` and `QGS-EDGE` refuses `implemented` to `in_progress`, so it cannot absorb the work: a successor is
the only route that does not either reopen a closed work order or leave the names unwritten.

## In scope

1. **`--connector-path <path>`**, the executable the host spawns. Named for the pattern every path-carrying option
   in this repository already follows: `--events-path`, `--transcript-path`, `--transcript-output`. It follows
   rule 18.4 exactly — the shared parser validates it and discards the value, and the binary target re-reads
   the raw argument.
2. **`--live`**, the explicit live-mode selection of rule 13.1. A bare flag, on the `--trace-actions` precedent,
   because it selects rather than carries. It is one of rule 13.1's two conditions and satisfies neither of the
   other's: the credential is the connector's, and no single component can authorise spending.
3. **`--spend-ceiling <amount>`**, the declared ceiling of rule 14.6. Named with the word that says what is being
   limited, which `--ceiling` alone does not.
4. **Rule 18's text updated to use the three names** wherever it currently describes them, so that the descriptive
   phrases and the names cannot drift apart.

## Out of scope

- **Every implementation.** No Rust file is touched. `WO-MOK-026` implements all three options; this work order is
  what lets it.
- **The amount's unit and grammar for `--spend-ceiling`.** Rule 14.2 already fixes cost as integer arithmetic in a
  stated minor unit, and how the option's value is spelled is `WO-MOK-026`'s to implement under that rule. This
  work order names the option and does not re-specify the arithmetic.
- **Any rule other than 18.** Rules 1 to 17, 19 and 20 keep their text; rule 13.1's two conditions and rule 14.6's
  before-spending check are untouched and are only referred to.
- **`SPEC-MOK-004` rule 1's drift**, recorded as a finding under `WO-MOK-026` and not corrected here.

## Authorized decision envelope

The engineering owner may word the amendment record.

The engineering owner may **not**, under this work order: choose different names from the three above, which the
owner decided on 2026-08-28; name a fourth option; change any rule other than 18; or write any implementation.

## Constraints

- The three names are the owner's decision of 2026-08-28, not a derivation, and the amendment record says so.
- Every neighbouring rule keeps its text, and rule 18's existing sub-rule numbers do not move.
- The names are consistent with the surface that exists rather than with a scheme invented here.

## Expected change surface

- `docs/engineering/simulation/specifications/SPEC-MOK-007.md` — rule 18 and an amendment record.
- `docs/engineering/simulation/work-orders/WO-MOK-029.md` — this work order's own lifecycle events.
- `docs/engineering/simulation/evidence/WO-MOK-029/handoff-check.md` — the handoff snapshot binding.

## Required verification

None contracted beyond `SPEC-MOK-005`'s standing gates. `VER-MOK-018` is the applicable contract and no case of
it is discharged here: its cases are about a running system and this work order runs nothing. `WO-MOK-026`
discharges them.

## Evidence to record

The handoff checkpoint's snapshot binding, and nothing else. No measurement is taken. `WO-MOK-028`'s evidence
section originally said "None retained" and was corrected when the gate refused it; this one is written knowing
that `not_required` assurance means no commit-bound record is owed and not that no evidence exists.

## Stop and escalate conditions

Stop and return to the engineering owner when:

- a **fifth** unspecified thing appears in `SPEC-MOK-007`, rather than being folded in silently. Two have now
  been found in succession, so a third occurrence is a fact about the specification and not a coincidence;
- naming an option would contradict a name already published in this repository; or
- rule 18's existing text cannot be updated without changing a rule outside 18.

## Completion report format

1. The three names, and the existing option each was made consistent with.
2. The lines of rule 18 that changed.
3. `validate` under the released evaluator.
4. Confirmation that no Rust file is in the change.
