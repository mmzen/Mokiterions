# Requirement-to-test mapping — WO-MOK-002

Regenerated 2026-08-17 after the third `SPEC-MOK-001` amendment. Every row in `VER-MOK-002`'s
requirement-to-evidence matrix appears below with the test or evidence that discharges it. Tests
live in `src/simulation.rs`, `src/cli.rs`, and `src/main.rs`.

**All 52 tests pass. Nothing is failing, skipped, or ignored.**

## REQ-MOK-013 — bounded perception

| Verification case | Test | Result |
|---|---|---|
| In-radius resource with class, direction, distance | `perception_reports_in_radius_food_with_class_direction_and_distance` | pass |
| In-radius living Mokiterion, observer excluded | `perception_reports_living_neighbours_and_never_the_observer` | pass |
| Out-of-radius resource and dead Mokiterion excluded | `perception_excludes_distant_resources_and_dead_neighbours` | pass |
| Perception crosses the territory boundary | `perception_crosses_the_territory_boundary` | pass |
| Order stable and independent of collection iteration | `perception_order_is_stable_and_independent_of_collection_order` | pass |
| Perception consumes no entropy and mutates nothing | `building_an_observation_consumes_no_entropy_and_mutates_nothing` | pass |

Also covering the radius contract: `the_radius_boundary_is_inclusive_and_exclusive_by_one_cell`
and `co_located_entities_are_reported_at_distance_zero_without_a_direction`.

## REQ-MOK-014 — population viability at a stated density

| Verification case | Test or evidence | Result |
|---|---|---|
| 1,000 ticks at the declared `0.75%` on each declared seed, floor 8 | `the_reference_source_sustains_the_population_at_every_declared_density` | **pass — 8, 11, 8, 9, 11** |
| Same runs report food consumption | same test, consumption assertion | pass, 324 to 410 per run |
| Density curve including undeclared densities | `density-curve.md` | recorded, no obligation |
| Conditional regeneration preserved at any density | `food_regenerates_only_in_nonempty_nonfull_territories` | pass |
| Density resolves to `0.15%`→12, `0.75%`→61, `1.50%`→122 | `density_resolves_to_the_specified_resource_count` | pass |
| Density binds initialization, capacity, replenishment target | `density_binds_initialization_capacity_and_the_replenishment_target` | pass |
| Zero-resource density rejected, exit code 2 | `a_density_resolving_to_no_resources_is_rejected` and `a_density_resolving_to_no_resources_exits_with_code_two_before_initialization` | pass |
| Scarcity assessment at the default density, manual | `manual-observation.md` | pass, no seed retains twelve |

The floor row was left failing under the previous amendment at three survivors. It passes here
because `SPEC-MOK-001` rule 5 was amended, not because the floor was lowered: the floor was
*raised* from five to eight in the same decision. Per-seed data is in `calibration-record.md`.

## REQ-MOK-015 — reference decision source

| Verification case | Test | Result |
|---|---|---|
| Moves toward a perceived resource, then eats it | `the_reference_source_approaches_then_consumes_a_perceived_resource` | pass |
| Does not consume wastefully | `the_reference_source_does_not_consume_a_resource_it_does_not_need` | pass |
| Does not approach a resource it would decline; does not re-target the cell just left | `the_reference_source_does_not_approach_a_resource_it_would_decline` | pass |
| Searches rather than waiting when it perceives nothing | `the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing` | pass |
| Sleeps below the threshold, in preference to approach and search | `the_reference_source_sustains_itself_before_seeking_or_searching` | pass |
| Cannot mutate authoritative state | `the_reference_source_cannot_mutate_authoritative_state` | pass |
| Both sources selectable, selection reported | `both_sources_run_are_reported_and_are_byte_identically_reproducible`, `both_policies_are_selectable_and_reference_is_the_default` | pass |
| Deterministic | same test, plus `repeated_runs_are_byte_identical` | pass |

Also covering the specified ranking rules: `the_reference_source_prefers_the_nearest_then_richest_resource`
and `the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach`.

The new case is the one the third amendment added. It asserts three things at the boundary
without naming how ranking is implemented: a Mokiterion perceiving only resources that would be
clipped proposes a move whose entropy draw count is `1`, which identifies it as a search step
rather than an approach; a resource that does fit is still approached deterministically with zero
draws, so the filter is selective rather than blanket; and standing one cell from a declined
resource does not produce a deterministic step back onto it, which is the two-cell cycle the
amendment removes.

## Property and invariant tests

| Invariant | Test | Result |
|---|---|---|
| Determinism, both sources, both swept densities | `both_sources_run_are_reported_and_are_byte_identically_reproducible`, `determinism-and-resilience.md` | pass |
| Density invariants: starts at, never exceeds, replenishes toward | `density_binds_initialization_capacity_and_the_replenishment_target`, `regeneration_adds_only_what_remaining_capacity_allows`, `food_regeneration_respects_capacity` | pass |
| Initial state matches the contract at the resolved density | `initial_world_population_and_food_match_the_contract` | pass |
| Attribute bounds within `0..=100` | `attributes_stay_within_bounds_across_a_long_reference_run`, `survival_decay_saturates_and_death_is_final` | pass |
| Perception symmetry | `perception_is_symmetric_between_living_mokiterions` | pass |
| Perception purity | `building_an_observation_consumes_no_entropy_and_mutates_nothing` | pass |
| No action at a distance | `perception_grants_no_ability_to_act_at_a_distance` | pass |
| Death finality | `survival_decay_saturates_and_death_is_final` | pass |
| Reference liveness: never waits, energy never zero | `the_reference_source_never_waits_and_never_runs_its_energy_to_zero` | pass |
| Entropy attribution: only search draws | `the_reference_source_prefers_the_nearest_then_richest_resource`, `the_reference_source_does_not_approach_a_resource_it_would_decline`, and the co-located and approach tests assert zero draws | pass |
| Bounded long run under either source | `a_long_run_is_bounded_under_either_source`, `a_long_configured_run_is_bounded_and_does_not_panic` | pass |
| Entropy stream stability | `splitmix64_sequence_is_stable`, `initialization_is_seeded_and_reproducible` | pass |
| Untrusted proposals validated and traced | `untrusted_decisions_are_validated_and_traced`, `invalid_move_does_not_mutate_action_state` | pass |
| Termination and single summary | `tick_limit_terminates_with_one_summary`, `extinction_takes_precedence_at_the_tick_limit` | pass |

## Command-line contract

| Check | Test | Result |
|---|---|---|
| Defaults stable, reference is the default policy | `defaults_are_stable`, `both_policies_are_selectable_and_reference_is_the_default` | pass |
| `--density` accepted forms and rejections | `density_is_accepted_in_the_specified_forms_and_rejected_otherwise` | pass |
| Duplicate and missing option values rejected | `duplicates_and_missing_values_are_rejected` | pass |
| Options accepted in any order | `options_work_in_any_order` | pass |
| Invalid configuration exits 2; help exits 0; output failure exits 1 | `invalid_configuration_exits_with_code_two`, `help_exits_successfully`, `output_failure_exits_with_code_one` | pass |

## Static, security, and performance checks

| Check | Evidence | Result |
|---|---|---|
| `cargo fmt --all -- --check` | `static-checks.txt` | no differences |
| `cargo clippy --all-targets --all-features -- -D warnings` | `static-checks.txt` | no findings |
| `cargo build`, `cargo test` | `static-checks.txt`, `test-run.txt` | build clean; 52 pass, 0 fail |
| Dependencies remain empty | `static-checks.txt`, `boundary-and-security-review.md` | confirmed |
| Trust boundary passes only copied or immutable values | `boundary-and-security-review.md` | confirmed |
| No network, credential, or filesystem access | `boundary-and-security-review.md` | confirmed |
| 10,000-tick resilience under each source | `determinism-and-resilience.md` | no panic, conservation holds |
| Oscillation rate against a random-walk baseline | `manual-observation.md`, `density-curve.md` | 10.6% against a 12.2% floor |
| End-of-run resource mix by class | `calibration-record.md`, `density-curve.md` | recorded; drift characterised |

## Coverage summary

52 tests, all passing. Every verification row in `VER-MOK-002` has a named discharging test or
evidence file. `REQ-MOK-013`, `REQ-MOK-014`, and `REQ-MOK-015` are fully discharged at the
horizons those requirements state. The one behavior recorded as accepted rather than verified is
long-horizon stability, which no requirement in scope asserts; see the residual uncertainty in
`VER-MOK-002`.
