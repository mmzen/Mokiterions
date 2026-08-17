# WO-MOK-003 evidence: per-test placement justification

`VER-MOK-003` requires two things of this record: for each test that stayed in `src/`, the
non-public item it needs; and for each test that moved, the public items it uses. Every test in
the suite appears in exactly one of the two tables below.

The internal-tier column is derived mechanically rather than asserted. A script parses
`src/simulation.rs`, builds a map from every member name to the `impl` block that owns it, and
then scans each test body for references to items that are not `pub` in the post-change crate:
private struct fields, private methods and associated constants resolved to their owning type,
private types, private module-level constants, private free functions, and the two test helpers
that themselves reach private state. **No internal-tier test has an empty dependency list.** No
test, therefore, is inline merely because it started there, which is what `SPEC-MOK-002` rule 7
exists to prevent.

## Public tier

Each entry names the rule 5 items the relocated test uses. In every case the published interface
is sufficient with the assertions unchanged; `relocated-test-diff.md` shows that 11 of the 15
bodies are character-identical and the other 4 differ only by reading a value through an accessor
instead of a field.

### `tests/cli.rs` — 5 tests

| Test | Rule 5 items it uses |
| --- | --- |
| `defaults_are_stable` | `cli::parse`, `cli::Command::Run`, `simulation::Config` and its five public fields, `Policy::Reference`, `Density::DEFAULT` |
| `options_work_in_any_order` | `cli::parse`, `cli::Command::Run`, `Config`, `Policy::Baseline`, `Density::parse` |
| `both_policies_are_selectable_and_reference_is_the_default` | `cli::parse`, `cli::Command::Run`, `Config`, `Policy::Baseline`, `Policy::Reference`, `Density::DEFAULT` |
| `duplicates_and_missing_values_are_rejected` | `cli::parse` only, through its error strings |
| `density_is_accepted_in_the_specified_forms_and_rejected_otherwise` | `cli::parse`, `cli::Command::Run`, `Config`, `Policy::Reference`, `Density::DEFAULT` |

### `tests/process.rs` — 4 tests

| Test | Rule 5 items it uses |
| --- | --- |
| `help_exits_successfully` | `execute`, `cli::USAGE` |
| `invalid_configuration_exits_with_code_two` | `execute` only; the usage text is matched as a substring of standard error rather than compared to `cli::USAGE` |
| `a_density_resolving_to_no_resources_exits_with_code_two_before_initialization` | `execute` |
| `output_failure_exits_with_code_one` | `execute`, plus the local `FailingWriter` implementing `std::io::Write` |

### `tests/density.rs` — 2 tests

| Test | Rule 5 items it uses |
| --- | --- |
| `density_resolves_to_the_specified_resource_count` | `CELLS_PER_TERRITORY`, `Density::parse`, `Density::resources_per_territory`, `Density::DEFAULT`, `Density`'s `Display` and `PartialEq` |
| `a_density_resolving_to_no_resources_is_rejected` | `Density::parse`, `Density::resources_per_territory` |

### `tests/termination.rs` — 3 tests

| Test | Rule 5 items it uses |
| --- | --- |
| `tick_limit_terminates_with_one_summary` | `Config`, `Policy::Baseline`, `Density::DEFAULT`, `Simulation::new`, `Simulation::run`, `RunSummary::reason`, `RunSummary::ticks`, `TerminationReason::TickLimit` |
| `a_long_configured_run_is_bounded_and_does_not_panic` | `Config`, `Policy::Baseline`, `Density::DEFAULT`, `Simulation::new`, `Simulation::run`, `RunSummary::ticks`, `RunSummary::survivors`, `RunSummary::deaths` |
| `a_long_run_is_bounded_under_either_source` | `Config`, `Policy::Baseline`, `Policy::Reference`, `Density::DEFAULT`, `Simulation::new`, `Simulation::run`, `RunSummary::ticks`, `RunSummary::survivors`, `RunSummary::deaths` |

### `tests/viability.rs` — 1 tests

| Test | Rule 5 items it uses |
| --- | --- |
| `the_reference_source_sustains_the_population_at_every_declared_density` | `Config`, `Policy::Reference`, `Density::parse`, `Simulation::new`, `Simulation::run`, `RunSummary::reason`, `RunSummary::survivors`, `TerminationReason::TickLimit` |

Public-tier total: **15 tests** across five files.

## Internal tier

All 37 remain in the single `#[cfg(test)] mod tests` block of `src/simulation.rs`.
`src/cli.rs` retains no test module at all: every argument-parsing test needed only `cli::parse`
and its error strings, so once those five moved there was nothing private left in that file to
assert against. `src/lib.rs` and `src/main.rs` contain no tests, as rules 3 and 4 require.

| Test | Non-public items it requires |
| --- | --- |
| `initial_world_population_and_food_match_the_contract` | `Simulation::config` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Coordinate::territory`, `Coordinate`, `FoodClass`, `Territory`, `Territory::ALL`, `FoodClass::ALL` |
| `initialization_is_seeded_and_reproducible` | `Simulation::agents` (private field), the helper `state_snapshot`, which clones `agents`, `foods`, and `entropy` |
| `splitmix64_sequence_is_stable` | `SplitMix64::next_u64`, `SplitMix64` |
| `repeated_runs_are_byte_identical` | the helper `state_snapshot`, which clones `agents`, `foods`, and `entropy` |
| `action_tracing_is_optional_complete_and_observational` | the helper `state_snapshot`, which clones `agents`, `foods`, and `entropy` |
| `invalid_move_does_not_mutate_action_state` | `Simulation::agents` (private field), `Simulation::foods` (private field), `Simulation::apply_action`, `Action`, `Coordinate`, `Direction` |
| `movement_crosses_territory_and_is_observable` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::apply_action`, `Action`, `Coordinate`, `Direction` |
| `sleep_restores_energy_without_exceeding_the_maximum` | `Simulation::agents` (private field), `Simulation::apply_action`, `Action` |
| `eating_is_atomic_bounded_and_single_use` | `Simulation::agents` (private field), `Simulation::foods` (private field), `Simulation::apply_action`, `Action`, `FoodClass` |
| `survival_decay_saturates_and_death_is_final` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::apply_survival` |
| `food_regenerates_only_in_nonempty_nonfull_territories` | `Simulation::tick` (private field), `Simulation::foods` (private field), `Simulation::food_counts`, `Simulation::regenerate_food`, `Coordinate::territory`, `Territory`, `REGENERATION_YIELD` |
| `food_regeneration_respects_capacity` | `Simulation::config` (private field), `Simulation::tick` (private field), `Simulation::foods` (private field), `Simulation::food_counts`, `Simulation::regenerate_food`, `Coordinate::territory`, `Coordinate`, `Food`, `FoodClass`, `Territory` |
| `untrusted_decisions_are_validated_and_traced` | `Simulation::agents` (private field), `Simulation::run_with_source`, `Coordinate`, `InvalidNorthDecisionSource` |
| `extinction_takes_precedence_at_the_tick_limit` | `Simulation::agents` (private field), `Simulation::foods` (private field), `RunSummary::reason` (private field), `RunSummary::survivors` (private field) |
| `regeneration_adds_only_what_remaining_capacity_allows` | `Simulation::config` (private field), `Simulation::tick` (private field), `Simulation::foods` (private field), `Simulation::food_counts`, `Simulation::regenerate_food`, `Coordinate::territory`, `Coordinate`, `Food`, `FoodClass`, `Territory` |
| `perception_reports_in_radius_food_with_class_direction_and_distance` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Simulation::observation`, `Coordinate`, `Food`, `FoodClass`, `PerceivedFood`, `RelativeDirection` |
| `perception_reports_living_neighbours_and_never_the_observer` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::observation`, `Coordinate`, `PerceivedMokiterion`, `RelativeDirection` |
| `perception_excludes_distant_resources_and_dead_neighbours` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Simulation::observation`, `Coordinate`, `Food`, `FoodClass` |
| `the_radius_boundary_is_inclusive_and_exclusive_by_one_cell` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Simulation::observation`, `Coordinate`, `Food`, `FoodClass`, `PERCEPTION_RADIUS` |
| `perception_crosses_the_territory_boundary` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Simulation::observation`, `Coordinate`, `Food`, `FoodClass`, `RelativeDirection`, `Territory` |
| `co_located_entities_are_reported_at_distance_zero_without_a_direction` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Simulation::observation`, `Coordinate`, `Food`, `FoodClass` |
| `perception_order_is_stable_and_independent_of_collection_order` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Observation::is_consistent`, `Simulation::observation`, `Coordinate`, `Food`, `FoodClass` |
| `perception_is_symmetric_between_living_mokiterions` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::observation` |
| `building_an_observation_consumes_no_entropy_and_mutates_nothing` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::observation`, the helper `state_snapshot`, which clones `agents`, `foods`, and `entropy` |
| `density_binds_initialization_capacity_and_the_replenishment_target` | `Simulation::config` (private field), `Simulation::tick` (private field), `Simulation::food_counts`, `Simulation::regenerate_food`, `Coordinate::territory`, `Territory`, `REGENERATION_INTERVAL`, `Territory::ALL` |
| `attributes_stay_within_bounds_across_a_long_reference_run` | `Simulation::agents` (private field), `ATTRIBUTE_MAX` |
| `the_reference_source_approaches_then_consumes_a_perceived_resource` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Action`, `Coordinate`, `Direction`, `Food`, `FoodClass`, the helper `decide_once`, which calls private `Simulation::observation` and drives `DecisionSource` directly |
| `the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Action`, `Coordinate`, `Direction`, `Food`, `FoodClass`, the helper `decide_once`, which calls private `Simulation::observation` and drives `DecisionSource` directly |
| `the_reference_source_prefers_the_nearest_then_richest_resource` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Action`, `Coordinate`, `Direction`, `Food`, `FoodClass`, the helper `decide_once`, which calls private `Simulation::observation` and drives `DecisionSource` directly |
| `the_reference_source_does_not_consume_a_resource_it_does_not_need` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `FoodClass::restoration`, `Action`, `Coordinate`, `Food`, `FoodClass`, `ATTRIBUTE_MAX`, the helper `decide_once`, which calls private `Simulation::observation` and drives `DecisionSource` directly |
| `the_reference_source_does_not_approach_a_resource_it_would_decline` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `FoodClass::restoration`, `Action`, `Coordinate`, `Direction`, `Food`, `FoodClass`, `Mokiterion`, `ATTRIBUTE_MAX`, the helper `decide_once`, which calls private `Simulation::observation` and drives `DecisionSource` directly |
| `the_reference_source_sustains_itself_before_seeking_or_searching` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Action`, `Coordinate`, `Direction`, `Food`, `FoodClass`, `REFERENCE_SLEEP_THRESHOLD`, the helper `decide_once`, which calls private `Simulation::observation` and drives `DecisionSource` directly |
| `the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing` | `Simulation::tick` (private field), `Simulation::agents` (private field), `Simulation::foods` (private field), `Action`, `Coordinate`, the helper `decide_once`, which calls private `Simulation::observation` and drives `DecisionSource` directly |
| `the_reference_source_cannot_mutate_authoritative_state` | `Simulation::tick` (private field), `Simulation::agents` (private field), the helper `state_snapshot`, which clones `agents`, `foods`, and `entropy`, the helper `decide_once`, which calls private `Simulation::observation` and drives `DecisionSource` directly |
| `perception_grants_no_ability_to_act_at_a_distance` | `Simulation::agents` (private field), `Simulation::foods` (private field), `Coordinate`, `Food`, `FoodClass` |
| `both_sources_run_are_reported_and_are_byte_identically_reproducible` | the helper `state_snapshot`, which clones `agents`, `foods`, and `entropy` |
| `the_reference_source_never_waits_and_never_runs_its_energy_to_zero` | `Simulation::agents` (private field) |

Internal-tier total: **37 tests**. Suite total: **52 tests**.

## Tier disjointness and totality

- 15 public-tier plus 37 internal-tier is 52, the recorded pre-change total.
- No test is in both tiers. Each relocated body was deleted from its source file in the same
  change that created the `tests/` file holding it, and `test-census.md` reconciles 52 unique
  names before against 52 after, name by name, with none gained and none lost.
- No test is in neither tier. `cargo test` discovers 52 tests across seven targets and
  `test-run.txt` names each one, so the two tables above are exhaustive rather than merely
  consistent.

## Two rule 8 subjects that remain internal

Rule 8 names subjects per file, and two of those subjects turn out to be only partly reachable
through rule 5. Both are recorded here rather than resolved by widening the interface, which rule
7 forbids, or by weakening the test, which rule 12 forbids.

1. **Termination by extinction** — a `tests/termination.rs` subject.
   `extinction_takes_precedence_at_the_tick_limit` forces extinction inside one tick by clearing
   the resource collection and writing agent health and satiety directly. Rule 6 prohibits
   exposing either collection in any build configuration. The only public substitute would be to
   assert that extinction appears somewhere in a long scarce run, which trades an exact assertion
   for a probabilistic one — precisely the substitution `SPEC-MOK-002`'s third counterexample
   names. The test stays in `src/simulation.rs` with its assertions intact.
2. **The relationship between density, initial endowment, and capacity** — a `tests/density.rs`
   subject. `density_binds_initialization_capacity_and_the_replenishment_target` reads
   per-territory resource counts *before* any tick runs and then drives `regenerate_food` across
   a replenishment interval. `RunSummary` reports counts only after a run has ended, and no
   public item drives regeneration. `tests/density.rs` covers the resolved-count half of the
   subject; the binding half stays internal.

Neither case triggers the work order's stop-and-escalate condition. That condition fires when
leaving a test inline would leave a `VER-MOK-001` or `VER-MOK-002` case covered *only by a weaker
assertion*. Both tests keep their original assertions verbatim at their original strength, so no
verification case is weakened by either decision. Both are recorded as adverse observations
against rule 8's subject wording in `completion-summary.md`.
