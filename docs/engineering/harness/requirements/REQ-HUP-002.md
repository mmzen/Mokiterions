+++
id = "REQ-HUP-002"
type = "requirement"
title = "An adopted evaluator leaves every already-approved work order startable"
status = "approved"
owners = ["product owner"]
created = "2026-08-28"
updated = "2026-08-28"
statement = "WHEN the repository adopts an exact released evaluator version as its standard root, THE SYSTEM SHALL leave every work order that was approved under the superseded root startable under the adopted one."
verification_method = ["demonstration"]
priority = "must"
source = "WO-MOK-026, blocked at QGP-G3-SCOPE on 2026-08-28"
measure = "0 work orders in an authority-granting state whose start checkpoint is refused by the adopted evaluator"

# `REQ-HUP-001` states the obligation this one was mistaken for. That one is about the graph
# being *valid*; this one is about approved work being *startable*. They are independent: the
# graph validated with zero errors on 2026-08-28 at the same moment two approved work orders
# could not be started at all, which is how the gap survived a whole verified chain.

[relations]
derives_from = ["CAP-HUP-001"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T21:05:00Z"
decided_by = "product owner"
+++

# Requirement: an adopted evaluator leaves approved work startable

## Rationale

`REQ-HUP-001` obliges the adopted evaluator to validate the graph with zero errors, and `WO-HUP-001`
delivered exactly that. It was not enough.

Measured on 2026-08-28 at `330c086`, immediately after the 0.8.0 adoption merged, the repository
validated with **0 errors** and, at the same moment, **neither of its two approved work orders could be
started**:

```text
QGP-G3-SCOPE: WO-MOK-026 has no assessable execution scope.
QGP-G3-SCOPE: WO-MOK-027 has no assessable execution scope.
```

Both were approved on 2026-08-23 under the 0.4.0 root, whose work-order template carried no
`[execution_scope]` table. 0.8.0 requires one to start work and enforces it at the `start` checkpoint.
`WO-MOK-026` and `WO-MOK-027` are the *only* work orders in an authority-granting state that have not
been implemented, so every piece of authorized forward work in this repository was frozen by an
adoption that reported complete success.

The gap is not a defect in the evaluator. It is a defect in what the adoption measured. An
approved-but-unstartable work order is a perfectly valid artifact: it has every field its schema
requires, every relation resolves, and no check `REQ-HUP-001` contracts will ever look at it. The
condition is invisible to validation and visible only to an operator who tries to begin work — which,
by construction, happens after the adoption is verified and merged.

This requirement exists so that the next adoption is obliged to look.

## Behavior

When an exact released evaluator version is adopted as the standard root:

- every work order in a state the adopted evaluator treats as authority-granting, and which has not
  reached `implemented`, has its `start` checkpoint evaluated by that evaluator;
- none of them is refused for a reason arising from the adoption itself;
- a refusal that *is* found is repaired, or the adoption is not complete; and
- the check is performed against the adopted evaluator, not the superseded one, because the superseded
  one cannot express the condition.

## Assumptions and dependencies

- The adopted evaluator exposes a start checkpoint that can be evaluated without mutating anything.
  0.8.0 does: `check --artifact <id> --checkpoint start` is read-only.
- The set of work orders to check is enumerable from artifact metadata alone.
- Repairing a refusal may require amending an approved artifact, which is an accountable owner act and
  not something an adoption performs on its own authority. This requirement obliges the adoption to
  *surface* the refusal; it does not authorize the repair.

## Acceptance examples

### Example: normal behavior

An adoption moves the root to a new exact release. Three work orders are `approved` and unstarted.
Each is evaluated at its start checkpoint under the adopted evaluator and each passes. The requirement
is satisfied, and the evidence is the three readings.

### Example: failure behavior

The 0.4.0 to 0.8.0 adoption of `WO-HUP-001`. `validate` reported 0 errors, `doctor` 0 FAIL, and every
one of `VER-HUP-001`'s eleven assessments passed, while `WO-MOK-026` and `WO-MOK-027` were both
unstartable. Nothing in that chain was wrong; nothing in it looked. Under this requirement the same
adoption reports two refusals and is not complete until they are repaired or the owner accepts them.

## Open decisions

None. Whether a given refusal is repaired by amendment, by rescoping, or by accepting it is a decision
for the accountable owner at the time; this requirement fixes only that the refusal must be found.
