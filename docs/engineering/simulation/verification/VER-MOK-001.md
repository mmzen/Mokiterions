+++
id = "VER-MOK-001"
type = "verification"
title = "Minimum simulation foundation verification"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-11"
updated = "2026-08-11"

[relations]
verifies = [
  "REQ-MOK-001",
  "REQ-MOK-002",
  "REQ-MOK-003",
  "REQ-MOK-004",
  "REQ-MOK-005",
  "REQ-MOK-006",
  "REQ-MOK-007",
  "REQ-MOK-008",
  "REQ-MOK-009",
  "REQ-MOK-010",
  "REQ-MOK-011",
  "REQ-MOK-012",
]
+++

# Verification Contract: Minimum simulation foundation verification

## Independence

Verification cases derive from the requirements and `SPEC-MOK-001`, not from private implementation structure. Tests exercise public engine behavior, process inputs and outputs, or independently inspect final state. The assurance owner reviews the cases and evidence separately from implementation authorship.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-001` | Automated state and process tests | Valid and invalid world initialization | Exact dimensions and territorial partition; all food classes in both territories; invalid configuration starts no run |
| `REQ-MOK-002` | Automated state tests | Population initialization across representative seeds | Exactly twelve unique living agents, six per starting territory, valid positions and bounded attributes |
| `REQ-MOK-003` | Automated transition tests | Stable ordering, decay, critical damage, saturation, and death | Every scheduled living agent processed once; values and death follow the specification |
| `REQ-MOK-004` | Automated adversarial tests | Malformed, stale, unsupported, and impossible proposals | No partial action mutation; rejection is observable; survival timing remains correct |
| `REQ-MOK-005` | Automated action tests | Every core action, boundary movement, crossing, and dead-agent attempt | Exactly one valid action applied; all invalid cases rejected |
| `REQ-MOK-006` | Automated atomicity tests | Each calorie class, caps, missing food, and competing consumption | One resource removed once and exact bounded restoration applied |
| `REQ-MOK-007` | Automated clock and state tests | Counts zero, one, eleven, and twelve at regeneration ticks | Regeneration occurs only for counts one through eleven and stays within territory and capacity |
| `REQ-MOK-008` | Automated offline tests | Valid proposal selection across observations without environment credentials | One valid proposal returned deterministically with no network or secret access |
| `REQ-MOK-009` | Automated replay comparison | Repeated runs with same and different seeds | Same-seed stdout and final state are byte-identical; different-seed runs remain valid |
| `REQ-MOK-010` | Automated output-schema tests | All event types and rejected action | Required fields and stable ordering present; no nondeterministic values |
| `REQ-MOK-011` | Automated process tests | Tick-limit, extinction, help, invalid configuration, and output failure | Correct stopping point, one summary, required counts, and specified exit code |
| `REQ-MOK-012` | Automated trace-mode tests | Trace enabled and disabled, accepted and rejected actions, agent death | Exactly one ordered trace per living-agent decision opportunity when enabled; none when disabled; final state is identical between modes |

## Acceptance scenarios

1. Run `cargo run -- --seed 42 --ticks 100`; the process exits `0`, emits a valid initialization, ordered events, and one tick-limit or extinction summary.
2. Run the same built binary twice with seed `42` and `100` ticks; captured standard output is byte-identical.
3. Force a territory to zero food before a regeneration tick; it remains at zero while a non-empty territory can regenerate.
4. Propose an out-of-bounds move and a stale food identifier; neither partially changes action-specific state and both produce rejection events.
5. Drive an agent to health zero; it dies once and never receives another decision.
6. Run the same seed and tick limit with and without `--trace-actions`; core events and final state match, while only the traced run contains exactly one action trace per decision opportunity.

## Property and invariant tests

- Every coordinate is in exactly one territory.
- Agent attributes remain in `0..=100` after every transition.
- Dead agents never return to living state or act again.
- Resource identifiers are unique and a resource is consumed at most once.
- Food counts remain in `0..=12` per territory.
- Every applied action was valid at its authoritative processing point.
- Exactly one action opportunity is resolved per scheduled living agent per tick.
- Action tracing neither consumes entropy nor changes authoritative state.
- Same seed and configuration produce identical observable results.

Representative seeded scenario tests are required. Property-based testing libraries are optional and must not be introduced solely to satisfy this wording.

## Static and architecture checks

Run and retain output from:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

Review the crate dependency graph and public decision interface to confirm `ARCH-MOK-001` and `ADR-MOK-001`: one binary crate, no network or UI dependency, no mutable world access from the decision source, validation of every proposal, and no nondeterministic entropy source.

## Security and privacy checks

- Search source and configuration for embedded credentials, tokens, or provider endpoints.
- Confirm a clean environment without model credentials can build, test, and run the foundation.
- Confirm invalid CLI text is handled as data and does not access files or execute code.

## Performance and resilience checks

- Run at least one 10,000-tick seeded simulation to expose overflow, unbounded state growth, or lifecycle failures; no fixed wall-clock target is imposed.
- Exercise tick-limit and early-extinction termination.
- Exercise output-error handling through an injectable or test-controlled writer.

## Manual assessments

Inspect one 20-tick run with `--trace-actions` for readable per-agent action order, understandable accepted or rejected outcomes, visible territory crossings when they occur, and a comprehensible final summary. Manual inspection supplements but does not replace automated assertions.

## Evidence retention

Retain command output, automated test output, deterministic replay captures or hashes, dependency review, architecture review notes, and the manual observation result under:

```text
docs/engineering/simulation/evidence/<work-order-id>/
```

Every evidence item must identify the work-order ID and command or procedure that produced it.

## Residual uncertainty

- The seeded baseline demonstrates engine execution, not meaningful LLM autonomy or emergent intelligence.
- Seeded scenario coverage cannot prove all long-run outcomes.
- Text readability remains partly subjective until a future interface is specified.
