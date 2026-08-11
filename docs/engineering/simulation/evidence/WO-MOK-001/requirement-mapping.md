# Requirement-to-test mapping for WO-MOK-001

| Requirement | Automated coverage |
|---|---|
| `REQ-MOK-001` | `initial_world_population_and_food_match_the_contract`, CLI invalid-configuration tests |
| `REQ-MOK-002` | `initial_world_population_and_food_match_the_contract`, `initialization_is_seeded_and_reproducible` |
| `REQ-MOK-003` | `survival_decay_saturates_and_death_is_final`, `extinction_takes_precedence_at_the_tick_limit` |
| `REQ-MOK-004` | `invalid_move_does_not_mutate_action_state`, `untrusted_decisions_are_validated_and_traced`, `eating_is_atomic_bounded_and_single_use` |
| `REQ-MOK-005` | movement, sleep, eating, invalid action, and baseline run tests |
| `REQ-MOK-006` | `eating_is_atomic_bounded_and_single_use` |
| `REQ-MOK-007` | `food_regenerates_only_in_nonempty_nonfull_territories`, `food_regeneration_respects_capacity` |
| `REQ-MOK-008` | `repeated_runs_are_byte_identical`, `action_tracing_is_optional_complete_and_observational`; all tests run without network or credentials |
| `REQ-MOK-009` | `splitmix64_sequence_is_stable`, `initialization_is_seeded_and_reproducible`, `repeated_runs_are_byte_identical` |
| `REQ-MOK-010` | initialization, crossing, consumption, survival, death, regeneration, termination, summary, and output-failure assertions across the suite |
| `REQ-MOK-011` | `tick_limit_terminates_with_one_summary`, `extinction_takes_precedence_at_the_tick_limit`, CLI exit-code tests |
| `REQ-MOK-012` | `action_tracing_is_optional_complete_and_observational`, `untrusted_decisions_are_validated_and_traced` |

## Cross-cutting invariants

- Attribute saturation and final death are exercised directly.
- Food identity is single-use and territory capacity is bounded.
- Invalid and stale action proposals do not partially mutate action state.
- Same-seed runs are byte-identical.
- Trace mode preserves authoritative state and core output.
- A run configured for 10,000 ticks terminates safely without panic.
