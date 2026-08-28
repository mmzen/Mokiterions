+++
id = "VER-HUP-002"
type = "verification"
title = "Evidence that every approved work order starts under the adopted evaluator, and that the repair changed only what it declared"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-28"
updated = "2026-08-28"

[relations]
verifies = ["REQ-HUP-002"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T21:05:00Z"
decided_by = "assurance owner"
+++

# Verification Contract: approved work remains startable

## Independence

As in `VER-HUP-001`, the evaluator is installed outside the checkout at an exact version and invoked as
`python -I -m se_harness`. The `isolated_python` field is not optional: `VREC-HUP-001`'s first capture was
discarded for carrying it false, and a proof without it is refused.

The engineering owner performs the repair and the assurance owner accepts the evidence. No assessment here
requires a second human reviewer.

## Requirement-to-evidence matrix

| Requirement | Obligation | Evidence |
|---|---|---|
| `REQ-HUP-002` | Every already-approved work order is startable under the adopted evaluator | B1, B2 |

## Acceptance scenarios

**B1 — the refusal is real, and it is enumerated rather than sampled.** Before the repair, evaluate the
`start` checkpoint of *every* work order in an authority-granting state that has not reached `implemented`.
**Pass:** the set of refusals is stated in full, with each refusal's code and text. A contract that showed one
refusal would not establish how many there are.

**B2 — every one of them starts afterwards.** After the repair, evaluate the same checkpoint over the same
enumerated set. **Pass:** zero refusals. This is `REQ-HUP-002`'s measure.

**B3 — the repair changed only the artifacts it declared.** Diff the whole change. **Pass:** the only governed
artifacts modified are those the work order names, each carries an amendment record, and no `status`, no
relation and no assurance field moves on any of them.

**B4 — no product effect.** **Pass:** no file under `mokiterions-core/` or `mokiterions-tui/`, and no Cargo or
toolchain manifest, appears in the change. The repair is about what a work order *declares*, not about what any
work order does.

**B5 — the graph still validates.** **Pass:** `validate` reports 0 errors, and the warning total is stated in
full rather than by code family. `VREC-HUP-001`'s `a1-validate.md` counted only `W-AUT-*` and understated the
total by one; this contract states the number the evaluator prints.

## Property and invariant tests

**P1 — the amended scopes admit the work each work order already describes.** For each amended work order,
every path named or implied by its own *Expected change surface* is admitted by the `[execution_scope].paths`
the amendment adds. Evidence: the mapping, surface item to admitting path, for each.

**P2 — no scope is wider than its work order.** No amended scope admits a path outside what that work order's
own text describes. A scope that admitted everything would pass B2 and mean nothing.

## Static and architecture checks

None contracted. The repair introduces no executable code and touches no architecture.

## Security and privacy checks

None contracted. The repair reads no credential, contacts nothing, and adds no input path. `WO-MOK-026`'s
credential handling is that work order's subject and `VER-MOK-018`'s obligation, not this one's.

## Performance and resilience checks

None contracted.

## Manual assessments

**M1 — the cause is recorded where the next adoption will meet it.** The assurance owner confirms that the
condition is stated as a rule in `SPEC-HUP-001` rather than only in this repair's evidence, so that an adoption
performed by someone who never read this chain is still obliged to check.

## Evidence retention

Retained under `docs/engineering/harness/evidence/WO-HUP-002/`:

- the enumerated set of work orders and their start-checkpoint readings before the repair (B1);
- the same enumeration and readings after it (B2);
- the change surface (B3, B4);
- `validate` output with its full error and warning counts (B5);
- the surface-to-scope mapping for each amended work order (P1, P2);
- a completion summary.

## Residual uncertainty

- **This contract checks the start checkpoint, not the whole of a work order's life.** A work order that starts
  may still be refused at `pre-action`, `transition` or `handoff` for reasons the adoption introduced. Those
  gates are reached only during execution and cannot be evaluated in advance from an unstarted work order.
- **`WO-MOK-026`'s and `WO-MOK-027`'s scopes are derived from their own prose**, written before an execution
  scope was a field. **Three** boundaries were not decidable from that prose and were settled by owner decision
  on 2026-08-28 rather than read out of the artifacts: whether `mokiterions-core/Cargo.toml` is in scope, where
  the connector protocol document lives, and where `WO-MOK-027`'s comparison report lives. A later reader should
  treat all three as decisions, not derivations. The third was found by `WO-HUP-002`'s stop-and-escalate
  condition firing during the repair, after this contract was first drafted saying two.
- **The repair is retrospective.** It states what the adoption should have measured. It cannot establish that
  the 0.4.0-to-0.8.0 adoption introduced no *other* condition of the same shape — one invisible to validation
  and visible only on use.
