# WO-MOK-003 evidence: test census and reconciliation

`VER-MOK-003` fixes the suite size at exactly the pre-change count: no test may be lost and none
may be added. This record reconciles the two censuses name by name, so the claim rests on set
equality rather than on two totals happening to agree.

## Method

Both censuses come from `cargo test -- --list`, not from counting `#[test]` attributes by hand.
The pre-change census is `baseline/test-census.txt`, captured on the unmodified tree at commit
`77010d02319051a20f8e45282f9c813ce4199956` before any edit. The post-change census is
`after/test-census.txt`. Because relocation changes the reported path of a test — an integration
test reports a bare name where a unit test reported `module::tests::name` — the reconciliation
compares the leaf test names and tracks the path change separately as the relocation itself.

Post-change file attribution is derived by reading `#[test]` declarations out of each `tests/*.rs`
file and cross-checking every name against what `cargo test -- --list` reported. The generator
fails rather than guesses if a file declares a test cargo did not report, if cargo reports a test
no file claims, or if two files claim the same name.

## Result

| Quantity | Count |
| --- | --- |
| Tests before | 52 |
| Tests after | 52 |
| Lost | 0 |
| Gained | 0 |
| Relocated | 15 |
| Unchanged in place | 37 |

No test present before is absent after. The set difference is empty.

No test present after is absent before. No test was written for this work order, which is
deliberate: `VER-MOK-003` conserves the census, so adding a test to cover a subject that
relocation left internal would itself be a violation. The two such subjects are recorded in
`test-placement.md` instead.

## Relocated tests — 15

| Test | Before | After |
| --- | --- | --- |
| `both_policies_are_selectable_and_reference_is_the_default` | `src/cli.rs` | `tests/cli.rs` |
| `defaults_are_stable` | `src/cli.rs` | `tests/cli.rs` |
| `density_is_accepted_in_the_specified_forms_and_rejected_otherwise` | `src/cli.rs` | `tests/cli.rs` |
| `duplicates_and_missing_values_are_rejected` | `src/cli.rs` | `tests/cli.rs` |
| `options_work_in_any_order` | `src/cli.rs` | `tests/cli.rs` |
| `a_density_resolving_to_no_resources_is_rejected` | `src/simulation.rs` | `tests/density.rs` |
| `density_resolves_to_the_specified_resource_count` | `src/simulation.rs` | `tests/density.rs` |
| `a_density_resolving_to_no_resources_exits_with_code_two_before_initialization` | `src/main.rs` | `tests/process.rs` |
| `help_exits_successfully` | `src/main.rs` | `tests/process.rs` |
| `invalid_configuration_exits_with_code_two` | `src/main.rs` | `tests/process.rs` |
| `output_failure_exits_with_code_one` | `src/main.rs` | `tests/process.rs` |
| `a_long_configured_run_is_bounded_and_does_not_panic` | `src/simulation.rs` | `tests/termination.rs` |
| `a_long_run_is_bounded_under_either_source` | `src/simulation.rs` | `tests/termination.rs` |
| `tick_limit_terminates_with_one_summary` | `src/simulation.rs` | `tests/termination.rs` |
| `the_reference_source_sustains_the_population_at_every_declared_density` | `src/simulation.rs` | `tests/viability.rs` |

## Tests unchanged in place — 37

All 37 remain in `src/simulation.rs`. Their bodies were not touched by this work order; the file
changed around them only in that four items became `pub` and one `impl RunSummary` block was
added.

- `action_tracing_is_optional_complete_and_observational`
- `attributes_stay_within_bounds_across_a_long_reference_run`
- `both_sources_run_are_reported_and_are_byte_identically_reproducible`
- `building_an_observation_consumes_no_entropy_and_mutates_nothing`
- `co_located_entities_are_reported_at_distance_zero_without_a_direction`
- `density_binds_initialization_capacity_and_the_replenishment_target`
- `eating_is_atomic_bounded_and_single_use`
- `extinction_takes_precedence_at_the_tick_limit`
- `food_regenerates_only_in_nonempty_nonfull_territories`
- `food_regeneration_respects_capacity`
- `initial_world_population_and_food_match_the_contract`
- `initialization_is_seeded_and_reproducible`
- `invalid_move_does_not_mutate_action_state`
- `movement_crosses_territory_and_is_observable`
- `perception_crosses_the_territory_boundary`
- `perception_excludes_distant_resources_and_dead_neighbours`
- `perception_grants_no_ability_to_act_at_a_distance`
- `perception_is_symmetric_between_living_mokiterions`
- `perception_order_is_stable_and_independent_of_collection_order`
- `perception_reports_in_radius_food_with_class_direction_and_distance`
- `perception_reports_living_neighbours_and_never_the_observer`
- `regeneration_adds_only_what_remaining_capacity_allows`
- `repeated_runs_are_byte_identical`
- `sleep_restores_energy_without_exceeding_the_maximum`
- `splitmix64_sequence_is_stable`
- `survival_decay_saturates_and_death_is_final`
- `the_radius_boundary_is_inclusive_and_exclusive_by_one_cell`
- `the_reference_source_approaches_then_consumes_a_perceived_resource`
- `the_reference_source_cannot_mutate_authoritative_state`
- `the_reference_source_does_not_approach_a_resource_it_would_decline`
- `the_reference_source_does_not_consume_a_resource_it_does_not_need`
- `the_reference_source_never_waits_and_never_runs_its_energy_to_zero`
- `the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach`
- `the_reference_source_prefers_the_nearest_then_richest_resource`
- `the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing`
- `the_reference_source_sustains_itself_before_seeking_or_searching`
- `untrusted_decisions_are_validated_and_traced`

## Per-file distribution

| File | Before | After |
| --- | --- | --- |
| `src/cli.rs` | 5 | 0 |
| `src/main.rs` | 4 | 0 |
| `src/simulation.rs` | 43 | 37 |
| `tests/cli.rs` | 0 | 5 |
| `tests/density.rs` | 0 | 2 |
| `tests/process.rs` | 0 | 4 |
| `tests/termination.rs` | 0 | 3 |
| `tests/viability.rs` | 0 | 1 |
| **Total** | **52** | **52** |

`src/main.rs` and `src/cli.rs` both drop to zero. `src/main.rs` holds no tests because rule 3
leaves it a shim with nothing testable in it; `src/cli.rs` holds none because every one of its
tests needed only `cli::parse` and its error strings, so rule 7 moved all five.

The single `cargo test` invocation behind `after/test-census.txt` is transcribed in full in
`test-run.txt`: 52 passed, 0 failed, 0 ignored, across seven targets.
