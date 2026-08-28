# P1 and P2 — the surface-to-scope mapping

`P1` is that every item of a work order's own *Expected change surface* is admitted by the scope the amendment
adds. `P2` is that nothing else is. Both are read item by item, because a scope that admitted everything would
satisfy `B2` and mean nothing at all.

An entry ending in `/` admits that directory and its descendants. Every other entry admits one exact path.

## `WO-MOK-026` — 15 paths

| Surface item | Admitting path | Basis |
|---|---|---|
| The connector protocol document | `docs/CONNECTOR_PROTOCOL.md` | **owner decision** |
| The canned connector, a test fixture in an existing package's test tree | `mokiterions-core/tests/` | derived |
| …and the target declaration that makes it a real child process | `mokiterions-core/Cargo.toml` | **owner decision** |
| The engine's binary target: spawn, streams, reap, env pass-through, three options, usage text | `mokiterions-core/src/main.rs` | derived |
| The engine's shared parser: three options recognised, validated, at-most-once, values discarded | `mokiterions-core/src/cli.rs` | derived |
| The observer's option parsing: three refusals before the terminal is entered | `mokiterions-tui/src/options.rs`, `mokiterions-tui/src/main.rs` | derived |
| The engine's library target: usage figures, accumulator, ceiling check, ratio, run-record fields | `mokiterions-core/src/simulation.rs`, `mokiterions-core/src/lib.rs` | derived |
| The engine's test tiers | `mokiterions-core/tests/`, `mokiterions-tui/tests/` | derived |
| The evidence path: the live run's four artifacts and two attestations | `docs/engineering/simulation/evidence/WO-MOK-026/` | derived |
| `REPOSITORY_CONTEXT.md`, per the amendments | `docs/engineering/REPOSITORY_CONTEXT.md` | derived, named in the surface |
| `SPEC-MOK-003`'s *Start-up inputs*, per the amendments | `docs/engineering/simulation/specifications/SPEC-MOK-003.md` | derived, named |
| `SPEC-MOK-004` rule 11, per the amendments | `docs/engineering/simulation/specifications/SPEC-MOK-004.md` | derived, named |
| The work order's own lifecycle events and amendment records | `docs/engineering/simulation/work-orders/WO-MOK-026.md` | structural |

**P1: every surface item maps.** **P2: every path is claimed by an item.** No path in this scope is unaccounted
for above, and the two owner decisions are labelled rather than presented as readings of the text.

Two exclusions are worth stating because a reader may expect them. `Cargo.lock` is **not** admitted: neither
package gains a dependency, which is the whole of `ADR-MOK-006` decision 3 and item 5 of the work order's *In
scope*. `SPEC-MOK-004` rule 1 is **not** in scope even though the file is, because the surface says so
explicitly — the file is admitted for rule 11's figures, and admitting a file is not admitting every rule in it.

## `WO-MOK-027` — 5 paths

| Surface item | Admitting path | Basis |
|---|---|---|
| The comparison report, "a new document" | `docs/PHASE_5_MEASUREMENT.md` | **owner decision** |
| The evidence path: one directory per run, each with transcript, record stream, run record, authorization | `docs/engineering/simulation/evidence/WO-MOK-027/` | derived |
| A static check, "beside `scripts/validate_engineering_artifacts.py`" | `scripts/` | derived — the surface names the directory itself |
| `INT-MOK-001`, amended per `ADR-MOK-007` | `docs/engineering/simulation/intent/INT-MOK-001.md` | derived, named |
| The work order's own lifecycle events and amendment records | `docs/engineering/simulation/work-orders/WO-MOK-027.md` | structural |

**P2 is load-bearing here.** This work order's surface states positively that it changes *no* Rust source and
*not* `SPEC-MOK-004`: "This stage adds no test, no package directory and no public item, so rule 11's figures do
not move and rule 1's layout does not either." The scope therefore admits **no** path under `mokiterions-core/`
or `mokiterions-tui/`, and not `SPEC-MOK-004.md`. That is not an omission — it is the work order's own claim
made enforceable, and if the stage turns out to need either, the work order says that is an escalation rather
than a widening.

## The three owner decisions

None of the three is a derivation, and each is recorded as a decision in `WO-HUP-002` item 3 and in the amended
work order's own amendment record:

1. `mokiterions-core/Cargo.toml` is inside `WO-MOK-026`'s scope.
2. The connector protocol document lives at `docs/CONNECTOR_PROTOCOL.md`.
3. `WO-MOK-027`'s comparison report lives at `docs/PHASE_5_MEASUREMENT.md`.

The third exists because `WO-HUP-002`'s stop-and-escalate condition fired during the repair: its text said a
third judgment "is not this work order's to take", the comparison report's path was exactly such a judgment, and
it was escalated and decided rather than inferred. The condition working is recorded here because a stop
condition that never fires is indistinguishable from one that does not work.
