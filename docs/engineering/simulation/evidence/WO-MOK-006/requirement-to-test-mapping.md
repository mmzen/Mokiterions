# Requirement-to-test mapping, superseding, after the package-directory move

## What this document is, and what it does not do

`ADR-MOK-004`'s *Migration* section says: "Records bound to commits are not edited. `VREC-MOK-001`
through `VREC-MOK-005` and the retained evidence under `WO-MOK-001` through `WO-MOK-005` name old
paths and stay as they are; a superseding requirement-to-test mapping is produced as `WO-MOK-006`
evidence." This is that document.

**The prior mappings remain valid and are not edited.** Each of them is evidence retained against a
specific verification record and a specific commit, and each was accurate about the tree it was
written for:

| Document | Verification record | Cited tests |
|---|---|---|
| `WO-MOK-001/requirement-mapping.md` | `VER-MOK-001` | 13 |
| `WO-MOK-002/requirement-to-test-mapping.md` | `VER-MOK-002` | 48 |
| `WO-MOK-003/requirement-to-test-mapping.md` | `VER-MOK-004` | 50 |
| `WO-MOK-004/new-tests.md` | `VER-MOK-004`, added cases | 8 |
| `WO-MOK-005/requirement-to-test-mapping.md` | `VER-MOK-005` | 94 |

Those documents name paths such as `src/lib.rs` and `tests/cli.rs`, and for the observer they name
tests inside `mokiterions-tui/src/main.rs`. After this change, none of those paths exists. Editing
them would rewrite evidence about a commit in which the old paths were correct, which is why the
harness treats a retained artifact as immutable and why this document exists instead.

What this document supersedes is the *path* half of those mappings, and nothing else. Every
requirement-to-test claim they make still holds; this document says where each cited test now lives
and adds the mapping for the three requirements this work order introduces.

## The translation rule

For the engine, the rule is a prefix and nothing more:

    src/<file>      ->  mokiterions-core/src/<file>
    tests/<file>    ->  mokiterions-core/tests/<file>

Every engine test keeps its file, its module path and its name. `file-comparison.txt` records that
all nine engine source files are byte-identical at blob level, so no engine test moved between files
and none was renamed. A reader holding `WO-MOK-002/requirement-to-test-mapping.md` needs only the
prefix.

For the observer, the rule is not a prefix, because tests moved between files:

    mokiterions-tui/src/main.rs, module `authority::tests`      ->  mokiterions-tui/tests/authority.rs
    mokiterions-tui/src/main.rs, module `export::tests`         ->  mokiterions-tui/tests/export.rs
    mokiterions-tui/src/main.rs, module `layout::tests`         ->  mokiterions-tui/tests/layout.rs
    mokiterions-tui/src/main.rs, module `options::tests`        ->  mokiterions-tui/tests/options.rs
    mokiterions-tui/src/main.rs, module `render::tests`         ->  split: 8 to tests/render.rs, 12 stay
    mokiterions-tui/src/main.rs, module `spatial::tests`        ->  mokiterions-tui/tests/spatial.rs
    mokiterions-tui/src/main.rs, module `state::tests`          ->  split: 21 to tests/state.rs, 4 stay
    mokiterions-tui/src/main.rs, module `verification`          ->  split: 16 to tests/verification.rs, 8 stay
    mokiterions-tui/src/main.rs, module `tests` (the binary's)  ->  unmoved, still in src/main.rs

A test that stayed did so because it names a private item or a `#[cfg(test)]` hook, which is a fact
about the code rather than a preference. `test-placement.md` gives the reason for each of the 32, and
`hooks-and-visibility.txt` explains why a hook cannot be reached from `tests/` at all. The tests that
stayed and previously lived in `src/main.rs`'s module tree now live in `src/lib.rs`'s, since the
modules themselves moved there; the binary's own eight-test module is the exception and is still in
`src/main.rs`.

A cited name is enough to locate a test in either package: `test-census.txt` lists all 169 by binary
and name, and no name appears twice across the whole workspace.

## The three requirements this work order introduces

`WO-MOK-006` implements `REQ-MOK-028`, `REQ-MOK-029` and `REQ-MOK-030`. None of the three is a
statement about simulation or presentation behavior, so none is discharged by a new test — this work
order adds no test and removes none, which is itself one of its conformance checks. Each is
discharged by a measurement retained in this packet, and the mapping is to those measurements.

### `REQ-MOK-028` — the observer's presentation contract is exercised from outside the crate

| What must hold | Where it is measured |
|---|---|
| The observer builds a library target whose public interface is exactly the items already public | `non-test-content.txt`: seven modules, seven SHA-256 digests identical, 78,332 bytes; `manifests.txt`: one `[lib]`, one `[[bin]]` |
| A tier of tests reaches the presentation layer through that interface | `test-census.txt`: 77 tests in eight files under `mokiterions-tui/tests/`; `test-run.txt`: all 77 pass |
| Those tests assert what they asserted before | `verbatim-comparison.txt`: 76 of 77 bodies byte-identical, the 77th differing in one path, twice |
| No item is widened and no `#[cfg(test)]` attribute removed | `non-test-content.txt` (the digests), `hooks-and-visibility.txt` (nine attributes, four hooks, all retained) |
| Tests that need internal access stay beside the code | `test-placement.md`: 32 tests, each with the private item or hook that holds it inside |

The 77 public-tier tests are themselves the strongest evidence for this requirement, and for a reason
worth stating: a test in `tests/` links the library compiled without `--cfg test`, so a test that
reached a private item or a hook **could not compile**. That all 77 compile and pass is a compiler
verdict that each is writable through the public interface, not an assessment.

### `REQ-MOK-029` — every test is in exactly one tier, placed by required access

| What must hold | Where it is measured |
|---|---|
| Each test in exactly one tier, no duplicates | `test-census.txt`: 169 tests by name; engine 37 + 23, observer 32 + 77; no name in two tiers |
| The count is unchanged from before the move | `test-census.txt` and `test-run.txt`: 169 before, 169 after, per tier and per package |
| Placement follows `SPEC-MOK-002` rule 7's test | `test-placement.md`: 32 internal with the item each names, 77 public with the public items each uses |
| One `cargo test` runs both tiers of both packages | `test-run.txt`: 169 passed, 0 failed, **0 ignored in all 19 result lines**, no feature, no environment variable, no terminal |
| No `#[ignore]` anywhere | `test-census.txt`, and the 19 result lines each reporting `0 ignored` |

### `REQ-MOK-030` — one directory per package

| What must hold | Where it is measured |
|---|---|
| Each package's manifest, sources and tests under one directory named for it | `file-comparison.txt`: the two trees and the nine renames; `manifests.txt` |
| The root manifest declares no package | `manifests.txt`: `[workspace]` and `resolver = "3"`, no `[package]` section |
| Every name, target kind and operator command resolves as before | `command-forms.txt`, `static-checks.txt`, `dependencies.txt` |
| The engine's behavior is unchanged by the move | `comparison/engine-matrix.txt`: seven captures, 0 differing lines; `file-comparison.txt`: nine identical blob ids |
| The engine's empty dependency set survives, re-measured per package | `dependencies.txt`: `cargo tree -p Mokiterions` resolves to the engine alone |

The `command-forms.txt` entry is where this requirement was **not** initially satisfied. Seven bare
`cargo run` invocations in `SIMULATION_RULES.md` stopped resolving at a virtual-manifest root with
two binaries. All seven were corrected to name `--bin Mokiterions` and re-run to exit 0.

## Resolution of every cited test name

The table below was produced mechanically: each test name cited in the five prior mapping documents
was looked up in the candidate's `cargo test -- --list` output for both packages. It is not a
transcription of the prior documents' requirement claims — those stand as written — but a check that
every test they rely on still exists, and a statement of where.

**152 distinct cited tests across the five documents; every one resolves.** Three identifiers written
in test-name style are not tests, and are explained after the table. 17 of the candidate's 169 tests
are not cited by name in any prior mapping; they are listed at the end with their locations.

```
=== VER-MOK-001 — cited in WO-MOK-001/requirement-mapping.md
   13 cited test names, 13 distinct tests, all present

   as cited there                                                                                  candidate test binary                  as `cargo test -- --list` reports it now
   action_tracing_is_optional_complete_and_observational                                           mokiterions-core/src/lib.rs            simulation::tests::action_tracing_is_optional_complete_and_observational
   eating_is_atomic_bounded_and_single_use                                                         mokiterions-core/src/lib.rs            simulation::tests::eating_is_atomic_bounded_and_single_use
   extinction_takes_precedence_at_the_tick_limit                                                   mokiterions-core/src/lib.rs            simulation::tests::extinction_takes_precedence_at_the_tick_limit
   food_regenerates_only_in_nonempty_nonfull_territories                                           mokiterions-core/src/lib.rs            simulation::tests::food_regenerates_only_in_nonempty_nonfull_territories
   food_regeneration_respects_capacity                                                             mokiterions-core/src/lib.rs            simulation::tests::food_regeneration_respects_capacity
   initial_world_population_and_food_match_the_contract                                            mokiterions-core/src/lib.rs            simulation::tests::initial_world_population_and_food_match_the_contract
   initialization_is_seeded_and_reproducible                                                       mokiterions-core/src/lib.rs            simulation::tests::initialization_is_seeded_and_reproducible
   invalid_move_does_not_mutate_action_state                                                       mokiterions-core/src/lib.rs            simulation::tests::invalid_move_does_not_mutate_action_state
   repeated_runs_are_byte_identical                                                                mokiterions-core/src/lib.rs            simulation::tests::repeated_runs_are_byte_identical
   splitmix64_sequence_is_stable                                                                   mokiterions-core/src/lib.rs            simulation::tests::splitmix64_sequence_is_stable
   survival_decay_saturates_and_death_is_final                                                     mokiterions-core/src/lib.rs            simulation::tests::survival_decay_saturates_and_death_is_final
   tick_limit_terminates_with_one_summary                                                          mokiterions-core/tests/termination.rs  tick_limit_terminates_with_one_summary
   untrusted_decisions_are_validated_and_traced                                                    mokiterions-core/src/lib.rs            simulation::tests::untrusted_decisions_are_validated_and_traced

=== VER-MOK-002 — cited in WO-MOK-002/requirement-to-test-mapping.md
   48 cited test names, 48 distinct tests, all present

   as cited there                                                                                  candidate test binary                  as `cargo test -- --list` reports it now
   a_density_resolving_to_no_resources_exits_with_code_two_before_initialization                   mokiterions-core/tests/process.rs      a_density_resolving_to_no_resources_exits_with_code_two_before_initialization
   a_density_resolving_to_no_resources_is_rejected                                                 mokiterions-core/tests/density.rs      a_density_resolving_to_no_resources_is_rejected
   a_long_configured_run_is_bounded_and_does_not_panic                                             mokiterions-core/tests/termination.rs  a_long_configured_run_is_bounded_and_does_not_panic
   a_long_run_is_bounded_under_either_source                                                       mokiterions-core/tests/termination.rs  a_long_run_is_bounded_under_either_source
   attributes_stay_within_bounds_across_a_long_reference_run                                       mokiterions-core/src/lib.rs            simulation::tests::attributes_stay_within_bounds_across_a_long_reference_run
   both_policies_are_selectable_and_reference_is_the_default                                       mokiterions-core/tests/cli.rs          both_policies_are_selectable_and_reference_is_the_default
   both_sources_run_are_reported_and_are_byte_identically_reproducible                             mokiterions-core/src/lib.rs            simulation::tests::both_sources_run_are_reported_and_are_byte_identically_reproducible
   building_an_observation_consumes_no_entropy_and_mutates_nothing                                 mokiterions-core/src/lib.rs            simulation::tests::building_an_observation_consumes_no_entropy_and_mutates_nothing
   co_located_entities_are_reported_at_distance_zero_without_a_direction                           mokiterions-core/src/lib.rs            simulation::tests::co_located_entities_are_reported_at_distance_zero_without_a_direction
   defaults_are_stable                                                                             mokiterions-core/tests/cli.rs          defaults_are_stable
   density_binds_initialization_capacity_and_the_replenishment_target                              mokiterions-core/src/lib.rs            simulation::tests::density_binds_initialization_capacity_and_the_replenishment_target
   density_is_accepted_in_the_specified_forms_and_rejected_otherwise                               mokiterions-core/tests/cli.rs          density_is_accepted_in_the_specified_forms_and_rejected_otherwise
   density_resolves_to_the_specified_resource_count                                                mokiterions-core/tests/density.rs      density_resolves_to_the_specified_resource_count
   duplicates_and_missing_values_are_rejected                                                      mokiterions-core/tests/cli.rs          duplicates_and_missing_values_are_rejected
   extinction_takes_precedence_at_the_tick_limit                                                   mokiterions-core/src/lib.rs            simulation::tests::extinction_takes_precedence_at_the_tick_limit
   food_regenerates_only_in_nonempty_nonfull_territories                                           mokiterions-core/src/lib.rs            simulation::tests::food_regenerates_only_in_nonempty_nonfull_territories
   food_regeneration_respects_capacity                                                             mokiterions-core/src/lib.rs            simulation::tests::food_regeneration_respects_capacity
   help_exits_successfully                                                                         mokiterions-core/tests/process.rs      help_exits_successfully
   initial_world_population_and_food_match_the_contract                                            mokiterions-core/src/lib.rs            simulation::tests::initial_world_population_and_food_match_the_contract
   initialization_is_seeded_and_reproducible                                                       mokiterions-core/src/lib.rs            simulation::tests::initialization_is_seeded_and_reproducible
   invalid_configuration_exits_with_code_two                                                       mokiterions-core/tests/process.rs      invalid_configuration_exits_with_code_two
   invalid_move_does_not_mutate_action_state                                                       mokiterions-core/src/lib.rs            simulation::tests::invalid_move_does_not_mutate_action_state
   options_work_in_any_order                                                                       mokiterions-core/tests/cli.rs          options_work_in_any_order
   output_failure_exits_with_code_one                                                              mokiterions-core/tests/process.rs      output_failure_exits_with_code_one
   perception_crosses_the_territory_boundary                                                       mokiterions-core/src/lib.rs            simulation::tests::perception_crosses_the_territory_boundary
   perception_excludes_distant_resources_and_dead_neighbours                                       mokiterions-core/src/lib.rs            simulation::tests::perception_excludes_distant_resources_and_dead_neighbours
   perception_grants_no_ability_to_act_at_a_distance                                               mokiterions-core/src/lib.rs            simulation::tests::perception_grants_no_ability_to_act_at_a_distance
   perception_is_symmetric_between_living_mokiterions                                              mokiterions-core/src/lib.rs            simulation::tests::perception_is_symmetric_between_living_mokiterions
   perception_order_is_stable_and_independent_of_collection_order                                  mokiterions-core/src/lib.rs            simulation::tests::perception_order_is_stable_and_independent_of_collection_order
   perception_reports_in_radius_food_with_class_direction_and_distance                             mokiterions-core/src/lib.rs            simulation::tests::perception_reports_in_radius_food_with_class_direction_and_distance
   perception_reports_living_neighbours_and_never_the_observer                                     mokiterions-core/src/lib.rs            simulation::tests::perception_reports_living_neighbours_and_never_the_observer
   regeneration_adds_only_what_remaining_capacity_allows                                           mokiterions-core/src/lib.rs            simulation::tests::regeneration_adds_only_what_remaining_capacity_allows
   repeated_runs_are_byte_identical                                                                mokiterions-core/src/lib.rs            simulation::tests::repeated_runs_are_byte_identical
   splitmix64_sequence_is_stable                                                                   mokiterions-core/src/lib.rs            simulation::tests::splitmix64_sequence_is_stable
   survival_decay_saturates_and_death_is_final                                                     mokiterions-core/src/lib.rs            simulation::tests::survival_decay_saturates_and_death_is_final
   the_radius_boundary_is_inclusive_and_exclusive_by_one_cell                                      mokiterions-core/src/lib.rs            simulation::tests::the_radius_boundary_is_inclusive_and_exclusive_by_one_cell
   the_reference_source_approaches_then_consumes_a_perceived_resource                              mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_approaches_then_consumes_a_perceived_resource
   the_reference_source_cannot_mutate_authoritative_state                                          mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_cannot_mutate_authoritative_state
   the_reference_source_does_not_approach_a_resource_it_would_decline                              mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_does_not_approach_a_resource_it_would_decline
   the_reference_source_does_not_consume_a_resource_it_does_not_need                               mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_does_not_consume_a_resource_it_does_not_need
   the_reference_source_never_waits_and_never_runs_its_energy_to_zero                              mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_never_waits_and_never_runs_its_energy_to_zero
   the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach                         mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach
   the_reference_source_prefers_the_nearest_then_richest_resource                                  mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_prefers_the_nearest_then_richest_resource
   the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing                     mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing
   the_reference_source_sustains_itself_before_seeking_or_searching                                mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_sustains_itself_before_seeking_or_searching
   the_reference_source_sustains_the_population_at_every_declared_density                          mokiterions-core/tests/viability.rs    the_reference_source_sustains_the_population_at_every_declared_density
   tick_limit_terminates_with_one_summary                                                          mokiterions-core/tests/termination.rs  tick_limit_terminates_with_one_summary
   untrusted_decisions_are_validated_and_traced                                                    mokiterions-core/src/lib.rs            simulation::tests::untrusted_decisions_are_validated_and_traced

=== VER-MOK-004 — cited in WO-MOK-003/requirement-to-test-mapping.md
   50 cited test names, 50 distinct tests, all present

   as cited there                                                                                  candidate test binary                  as `cargo test -- --list` reports it now
   a_density_resolving_to_no_resources_exits_with_code_two_before_initialization                   mokiterions-core/tests/process.rs      a_density_resolving_to_no_resources_exits_with_code_two_before_initialization
   a_density_resolving_to_no_resources_is_rejected                                                 mokiterions-core/tests/density.rs      a_density_resolving_to_no_resources_is_rejected
   a_long_configured_run_is_bounded_and_does_not_panic                                             mokiterions-core/tests/termination.rs  a_long_configured_run_is_bounded_and_does_not_panic
   a_long_run_is_bounded_under_either_source                                                       mokiterions-core/tests/termination.rs  a_long_run_is_bounded_under_either_source
   action_tracing_is_optional_complete_and_observational                                           mokiterions-core/src/lib.rs            simulation::tests::action_tracing_is_optional_complete_and_observational
   attributes_stay_within_bounds_across_a_long_reference_run                                       mokiterions-core/src/lib.rs            simulation::tests::attributes_stay_within_bounds_across_a_long_reference_run
   both_policies_are_selectable_and_reference_is_the_default                                       mokiterions-core/tests/cli.rs          both_policies_are_selectable_and_reference_is_the_default
   both_sources_run_are_reported_and_are_byte_identically_reproducible                             mokiterions-core/src/lib.rs            simulation::tests::both_sources_run_are_reported_and_are_byte_identically_reproducible
   building_an_observation_consumes_no_entropy_and_mutates_nothing                                 mokiterions-core/src/lib.rs            simulation::tests::building_an_observation_consumes_no_entropy_and_mutates_nothing
   co_located_entities_are_reported_at_distance_zero_without_a_direction                           mokiterions-core/src/lib.rs            simulation::tests::co_located_entities_are_reported_at_distance_zero_without_a_direction
   defaults_are_stable                                                                             mokiterions-core/tests/cli.rs          defaults_are_stable
   density_binds_initialization_capacity_and_the_replenishment_target                              mokiterions-core/src/lib.rs            simulation::tests::density_binds_initialization_capacity_and_the_replenishment_target
   density_is_accepted_in_the_specified_forms_and_rejected_otherwise                               mokiterions-core/tests/cli.rs          density_is_accepted_in_the_specified_forms_and_rejected_otherwise
   density_resolves_to_the_specified_resource_count                                                mokiterions-core/tests/density.rs      density_resolves_to_the_specified_resource_count
   duplicates_and_missing_values_are_rejected                                                      mokiterions-core/tests/cli.rs          duplicates_and_missing_values_are_rejected
   eating_is_atomic_bounded_and_single_use                                                         mokiterions-core/src/lib.rs            simulation::tests::eating_is_atomic_bounded_and_single_use
   extinction_takes_precedence_at_the_tick_limit                                                   mokiterions-core/src/lib.rs            simulation::tests::extinction_takes_precedence_at_the_tick_limit
   food_regenerates_only_in_nonempty_nonfull_territories                                           mokiterions-core/src/lib.rs            simulation::tests::food_regenerates_only_in_nonempty_nonfull_territories
   food_regeneration_respects_capacity                                                             mokiterions-core/src/lib.rs            simulation::tests::food_regeneration_respects_capacity
   help_exits_successfully                                                                         mokiterions-core/tests/process.rs      help_exits_successfully
   initial_world_population_and_food_match_the_contract                                            mokiterions-core/src/lib.rs            simulation::tests::initial_world_population_and_food_match_the_contract
   initialization_is_seeded_and_reproducible                                                       mokiterions-core/src/lib.rs            simulation::tests::initialization_is_seeded_and_reproducible
   invalid_configuration_exits_with_code_two                                                       mokiterions-core/tests/process.rs      invalid_configuration_exits_with_code_two
   invalid_move_does_not_mutate_action_state                                                       mokiterions-core/src/lib.rs            simulation::tests::invalid_move_does_not_mutate_action_state
   options_work_in_any_order                                                                       mokiterions-core/tests/cli.rs          options_work_in_any_order
   output_failure_exits_with_code_one                                                              mokiterions-core/tests/process.rs      output_failure_exits_with_code_one
   perception_crosses_the_territory_boundary                                                       mokiterions-core/src/lib.rs            simulation::tests::perception_crosses_the_territory_boundary
   perception_excludes_distant_resources_and_dead_neighbours                                       mokiterions-core/src/lib.rs            simulation::tests::perception_excludes_distant_resources_and_dead_neighbours
   perception_grants_no_ability_to_act_at_a_distance                                               mokiterions-core/src/lib.rs            simulation::tests::perception_grants_no_ability_to_act_at_a_distance
   perception_is_symmetric_between_living_mokiterions                                              mokiterions-core/src/lib.rs            simulation::tests::perception_is_symmetric_between_living_mokiterions
   perception_order_is_stable_and_independent_of_collection_order                                  mokiterions-core/src/lib.rs            simulation::tests::perception_order_is_stable_and_independent_of_collection_order
   perception_reports_in_radius_food_with_class_direction_and_distance                             mokiterions-core/src/lib.rs            simulation::tests::perception_reports_in_radius_food_with_class_direction_and_distance
   perception_reports_living_neighbours_and_never_the_observer                                     mokiterions-core/src/lib.rs            simulation::tests::perception_reports_living_neighbours_and_never_the_observer
   regeneration_adds_only_what_remaining_capacity_allows                                           mokiterions-core/src/lib.rs            simulation::tests::regeneration_adds_only_what_remaining_capacity_allows
   repeated_runs_are_byte_identical                                                                mokiterions-core/src/lib.rs            simulation::tests::repeated_runs_are_byte_identical
   splitmix64_sequence_is_stable                                                                   mokiterions-core/src/lib.rs            simulation::tests::splitmix64_sequence_is_stable
   survival_decay_saturates_and_death_is_final                                                     mokiterions-core/src/lib.rs            simulation::tests::survival_decay_saturates_and_death_is_final
   the_radius_boundary_is_inclusive_and_exclusive_by_one_cell                                      mokiterions-core/src/lib.rs            simulation::tests::the_radius_boundary_is_inclusive_and_exclusive_by_one_cell
   the_reference_source_approaches_then_consumes_a_perceived_resource                              mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_approaches_then_consumes_a_perceived_resource
   the_reference_source_cannot_mutate_authoritative_state                                          mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_cannot_mutate_authoritative_state
   the_reference_source_does_not_approach_a_resource_it_would_decline                              mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_does_not_approach_a_resource_it_would_decline
   the_reference_source_does_not_consume_a_resource_it_does_not_need                               mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_does_not_consume_a_resource_it_does_not_need
   the_reference_source_never_waits_and_never_runs_its_energy_to_zero                              mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_never_waits_and_never_runs_its_energy_to_zero
   the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach                         mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach
   the_reference_source_prefers_the_nearest_then_richest_resource                                  mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_prefers_the_nearest_then_richest_resource
   the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing                     mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing
   the_reference_source_sustains_itself_before_seeking_or_searching                                mokiterions-core/src/lib.rs            simulation::tests::the_reference_source_sustains_itself_before_seeking_or_searching
   the_reference_source_sustains_the_population_at_every_declared_density                          mokiterions-core/tests/viability.rs    the_reference_source_sustains_the_population_at_every_declared_density
   tick_limit_terminates_with_one_summary                                                          mokiterions-core/tests/termination.rs  tick_limit_terminates_with_one_summary
   untrusted_decisions_are_validated_and_traced                                                    mokiterions-core/src/lib.rs            simulation::tests::untrusted_decisions_are_validated_and_traced

=== VER-MOK-004 (new tests) — cited in WO-MOK-004/new-tests.md
   8 cited test names, 8 distinct tests, all present

   as cited there                                                                                  candidate test binary                  as `cargo test -- --list` reports it now
   each_declared_default_is_stated_once                                                            mokiterions-core/tests/cli.rs          each_declared_default_is_stated_once
   each_documented_default_parses_to_the_applied_default                                           mokiterions-core/tests/cli.rs          each_documented_default_parses_to_the_applied_default
   every_option_the_synopsis_names_has_an_options_entry                                            mokiterions-core/tests/cli.rs          every_option_the_synopsis_names_has_an_options_entry
   the_diagnostic_path_appends_the_whole_usage_text                                                mokiterions-core/tests/process.rs      the_diagnostic_path_appends_the_whole_usage_text
   the_documented_options_are_exactly_the_options_the_parser_accepts                               mokiterions-core/tests/cli.rs          the_documented_options_are_exactly_the_options_the_parser_accepts
   the_entries_state_the_constraints_that_decide_validity                                          mokiterions-core/tests/cli.rs          the_entries_state_the_constraints_that_decide_validity
   the_flags_state_their_effect_and_no_default_value                                               mokiterions-core/tests/cli.rs          the_flags_state_their_effect_and_no_default_value
   the_help_text_states_order_and_repetition                                                       mokiterions-core/tests/cli.rs          the_help_text_states_order_and_repetition

=== VER-MOK-005 — cited in WO-MOK-005/requirement-to-test-mapping.md
   94 cited test names, 94 distinct tests, all present

   as cited there                                                                                  candidate test binary                  as `cargo test -- --list` reports it now
   authority::tests::an_ordinary_record_resolves_from_its_own_payload                              mokiterions-tui/tests/authority.rs     an_ordinary_record_resolves_from_its_own_payload
   authority::tests::every_event_type_the_observer_can_present_has_an_entry                        mokiterions-tui/tests/authority.rs     every_event_type_the_observer_can_present_has_an_entry
   authority::tests::the_decision_source_maps_by_the_source_the_record_names                       mokiterions-tui/tests/authority.rs     the_decision_source_maps_by_the_source_the_record_names
   authority::tests::the_mapping_is_the_specified_one                                              mokiterions-tui/tests/authority.rs     the_mapping_is_the_specified_one
   export::tests::an_unwritable_path_is_reported_and_leaves_nothing_behind                         mokiterions-tui/tests/export.rs        an_unwritable_path_is_reported_and_leaves_nothing_behind
   export::tests::nothing_environment_specific_reaches_the_file                                    mokiterions-tui/tests/export.rs        nothing_environment_specific_reaches_the_file
   export::tests::records_use_the_engines_own_line_format_in_authoritative_order                   mokiterions-tui/tests/export.rs        records_use_the_engines_own_line_format_in_authoritative_order
   export::tests::the_closing_line_states_the_count_and_the_truncation                             mokiterions-tui/tests/export.rs        the_closing_line_states_the_count_and_the_truncation
   export::tests::the_default_path_is_relative_and_derived_from_the_run                            mokiterions-tui/tests/export.rs        the_default_path_is_relative_and_derived_from_the_run
   export::tests::the_same_records_always_produce_the_same_bytes                                   mokiterions-tui/tests/export.rs        the_same_records_always_produce_the_same_bytes
   layout::tests::every_region_stays_inside_the_viewport_and_the_body_rows_are_contiguous          mokiterions-tui/tests/layout.rs        every_region_stays_inside_the_viewport_and_the_body_rows_are_contiguous
   layout::tests::excluded_panes_are_the_ones_the_tier_omits                                       mokiterions-tui/tests/layout.rs        excluded_panes_are_the_ones_the_tier_omits
   layout::tests::the_declared_viewports_yield_the_declared_canvases                               mokiterions-tui/tests/layout.rs        the_declared_viewports_yield_the_declared_canvases
   layout::tests::the_floor_is_the_specified_one                                                   mokiterions-tui/tests/layout.rs        the_floor_is_the_specified_one
   layout::tests::tier_minimums_hold_wherever_the_tier_declares_one                                mokiterions-tui/tests/layout.rs        tier_minimums_hold_wherever_the_tier_declares_one
   layout::tests::tiers_match_the_specified_table_including_its_boundaries                         mokiterions-tui/tests/layout.rs        tiers_match_the_specified_table_including_its_boundaries
   options::tests::an_export_path_is_taken_verbatim_as_data                                        mokiterions-tui/tests/options.rs       an_export_path_is_taken_verbatim_as_data
   options::tests::defaults_match_the_specified_values                                             mokiterions-tui/tests/options.rs       defaults_match_the_specified_values
   options::tests::speed_steps_are_clamped_at_both_ends                                            mokiterions-tui/tests/options.rs       speed_steps_are_clamped_at_both_ends
   render::tests::a_bar_row_shrinks_to_its_pane_and_never_overflows_it                             mokiterions-tui/src/lib.rs             render::tests::a_bar_row_shrinks_to_its_pane_and_never_overflows_it
   render::tests::a_depleted_territory_is_stated_in_words_at_every_width                           mokiterions-tui/src/lib.rs             render::tests::a_depleted_territory_is_stated_in_words_at_every_width
   render::tests::a_region_states_the_world_range_it_presents                                      mokiterions-tui/tests/render.rs        a_region_states_the_world_range_it_presents
   render::tests::a_reported_failure_reaches_the_header                                            mokiterions-tui/tests/render.rs        a_reported_failure_reaches_the_header
   render::tests::a_resize_changes_the_layout_and_nothing_else                                     mokiterions-tui/src/lib.rs             render::tests::a_resize_changes_the_layout_and_nothing_else
   render::tests::a_zero_value_is_a_zero_and_an_absent_value_is_a_dash                             mokiterions-tui/src/lib.rs             render::tests::a_zero_value_is_a_zero_and_an_absent_value_is_a_dash
   render::tests::below_the_floor_nothing_is_presented                                             mokiterions-tui/tests/render.rs        below_the_floor_nothing_is_presented
   render::tests::detail_zoom_places_every_visible_entity_at_its_mapped_cell                       mokiterions-tui/tests/render.rs        detail_zoom_places_every_visible_entity_at_its_mapped_cell
   render::tests::drawing_never_advances_the_simulation                                            mokiterions-tui/tests/render.rs        drawing_never_advances_the_simulation
   render::tests::every_declared_viewport_renders_and_annotates_what_it_presents                   mokiterions-tui/tests/render.rs        every_declared_viewport_renders_and_annotates_what_it_presents
   render::tests::the_authority_overlay_names_identifiers_for_every_event_type                     mokiterions-tui/src/lib.rs             render::tests::the_authority_overlay_names_identifiers_for_every_event_type
   render::tests::the_bar_row_reproduces_the_specified_form                                        mokiterions-tui/src/lib.rs             render::tests::the_bar_row_reproduces_the_specified_form
   render::tests::the_footer_carries_the_provenance_and_nothing_environment_specific               mokiterions-tui/tests/render.rs        the_footer_carries_the_provenance_and_nothing_environment_specific
   render::tests::the_footer_survives_the_narrowest_viewport                                       mokiterions-tui/src/lib.rs             render::tests::the_footer_survives_the_narrowest_viewport
   render::tests::the_header_names_the_panes_that_are_only_overlays                                mokiterions-tui/tests/render.rs        the_header_names_the_panes_that_are_only_overlays
   render::tests::the_help_overlay_lists_every_bound_key                                           mokiterions-tui/src/lib.rs             render::tests::the_help_overlay_lists_every_bound_key
   render::tests::the_inspector_states_absence_rather_than_inventing_a_subject                     mokiterions-tui/src/lib.rs             render::tests::the_inspector_states_absence_rather_than_inventing_a_subject
   render::tests::the_log_shows_the_newest_records_and_reports_an_empty_filter                     mokiterions-tui/src/lib.rs             render::tests::the_log_shows_the_newest_records_and_reports_an_empty_filter
   render::tests::the_territory_rule_marks_the_row_between_the_territories                         mokiterions-tui/src/lib.rs             render::tests::the_territory_rule_marks_the_row_between_the_territories
   spatial::tests::a_character_cell_covers_two_by_four_world_cells_in_overview_and_one_in_detail   mokiterions-tui/tests/spatial.rs       a_character_cell_covers_two_by_four_world_cells_in_overview_and_one_in_detail
   spatial::tests::glyphs_are_the_assigned_ones                                                    mokiterions-tui/tests/spatial.rs       glyphs_are_the_assigned_ones
   spatial::tests::territory_a_is_above_territory_b                                                mokiterions-tui/tests/spatial.rs       territory_a_is_above_territory_b
   spatial::tests::the_camera_is_clamped_so_the_region_never_leaves_the_world                      mokiterions-tui/tests/spatial.rs       the_camera_is_clamped_so_the_region_never_leaves_the_world
   spatial::tests::the_overview_dot_grid_is_one_dot_per_world_cell                                 mokiterions-tui/tests/spatial.rs       the_overview_dot_grid_is_one_dot_per_world_cell
   spatial::tests::the_territory_rule_is_present_exactly_when_the_boundary_is_visible              mokiterions-tui/tests/spatial.rs       the_territory_rule_is_present_exactly_when_the_boundary_is_visible
   spatial::tests::the_whole_world_needs_both_axes_and_never_width_alone                           mokiterions-tui/tests/spatial.rs       the_whole_world_needs_both_axes_and_never_width_alone
   state::tests::a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour        mokiterions-tui/src/lib.rs             state::tests::a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour
   state::tests::a_death_carries_the_tick_and_the_engine_computed_final_values                     mokiterions-tui/tests/state.rs         a_death_carries_the_tick_and_the_engine_computed_final_values
   state::tests::a_finished_run_refuses_to_advance_and_stays_inspectable                           mokiterions-tui/tests/state.rs         a_finished_run_refuses_to_advance_and_stays_inspectable
   state::tests::a_key_release_is_not_a_press                                                      mokiterions-tui/tests/state.rs         a_key_release_is_not_a_press
   state::tests::a_single_step_is_accepted_only_while_held_and_advances_exactly_one_tick           mokiterions-tui/tests/state.rs         a_single_step_is_accepted_only_while_held_and_advances_exactly_one_tick
   state::tests::a_subject_filter_needs_a_selection                                                mokiterions-tui/tests/state.rs         a_subject_filter_needs_a_selection
   state::tests::a_whole_world_overview_cannot_be_panned_off_the_world                             mokiterions-tui/tests/state.rs         a_whole_world_overview_cannot_be_panned_off_the_world
   state::tests::an_unbound_key_changes_nothing                                                    mokiterions-tui/tests/state.rs         an_unbound_key_changes_nothing
   state::tests::escape_closes_an_overlay_before_it_clears_a_selection                             mokiterions-tui/tests/state.rs         escape_closes_an_overlay_before_it_clears_a_selection
   state::tests::every_overlay_has_its_bound_key                                                   mokiterions-tui/tests/state.rs         every_overlay_has_its_bound_key
   state::tests::filtering_changes_presentation_only                                               mokiterions-tui/tests/state.rs         filtering_changes_presentation_only
   state::tests::following_centres_the_selection_and_clamps_identically                            mokiterions-tui/tests/state.rs         following_centres_the_selection_and_clamps_identically
   state::tests::initialization_events_are_retained_in_authoritative_order                         mokiterions-tui/tests/state.rs         initialization_events_are_retained_in_authoritative_order
   state::tests::panning_moves_one_world_cell_and_clamps_at_every_edge                             mokiterions-tui/tests/state.rs         panning_moves_one_world_cell_and_clamps_at_every_edge
   state::tests::quit_is_the_only_key_that_asks_to_exit                                            mokiterions-tui/tests/state.rs         quit_is_the_only_key_that_asks_to_exit
   state::tests::selection_clears_itself_when_no_living_mokiterion_remains                         mokiterions-tui/src/lib.rs             state::tests::selection_clears_itself_when_no_living_mokiterion_remains
   state::tests::selection_cycles_in_roster_order_and_escape_clears_it                             mokiterions-tui/tests/state.rs         selection_cycles_in_roster_order_and_escape_clears_it
   state::tests::shared_cells_are_counted_at_the_rendered_granularity                              mokiterions-tui/tests/state.rs         shared_cells_are_counted_at_the_rendered_granularity
   state::tests::speed_steps_through_the_fixed_ladder_and_clamps                                   mokiterions-tui/tests/state.rs         speed_steps_through_the_fixed_ladder_and_clamps
   state::tests::the_event_buffer_drops_the_oldest_record_and_says_so                              mokiterions-tui/tests/state.rs         the_event_buffer_drops_the_oldest_record_and_says_so
   state::tests::the_highlighted_record_is_the_newest_until_the_operator_scrolls                   mokiterions-tui/src/lib.rs             state::tests::the_highlighted_record_is_the_newest_until_the_operator_scrolls
   state::tests::the_type_filter_cycles_the_whole_vocabulary_then_returns_to_none                  mokiterions-tui/tests/state.rs         the_type_filter_cycles_the_whole_vocabulary_then_returns_to_none
   tests::a_run_the_operator_ended_reports_itself_as_ended_early                                   mokiterions-tui/src/main.rs            tests::a_run_the_operator_ended_reports_itself_as_ended_early
   tests::a_viewport_below_the_floor_is_refused_with_both_dimensions_and_code_two                  mokiterions-tui/src/main.rs            tests::a_viewport_below_the_floor_is_refused_with_both_dimensions_and_code_two
   tests::the_idle_wait_never_exceeds_the_nearest_deadline                                         mokiterions-tui/src/main.rs            tests::the_idle_wait_never_exceeds_the_nearest_deadline
   verification::a_death_removes_the_subject_from_the_presentation_and_is_corroborated             mokiterions-tui/tests/verification.rs  a_death_removes_the_subject_from_the_presentation_and_is_corroborated
   verification::a_degenerate_world_still_draws_a_frame                                            mokiterions-tui/src/lib.rs             verification::a_degenerate_world_still_draws_a_frame
   verification::a_filter_changes_what_is_presented_and_nothing_else                               mokiterions-tui/src/lib.rs             verification::a_filter_changes_what_is_presented_and_nothing_else
   verification::a_finished_run_stays_inspectable_and_exportable                                   mokiterions-tui/tests/verification.rs  a_finished_run_stays_inspectable_and_exportable
   verification::a_smaller_world_row_never_renders_below_a_larger_one                              mokiterions-tui/tests/verification.rs  a_smaller_world_row_never_renders_below_a_larger_one
   verification::an_injected_export_failure_leaves_the_tick_intact                                 mokiterions-tui/tests/verification.rs  an_injected_export_failure_leaves_the_tick_intact
   verification::an_operator_ended_run_is_a_prefix_of_the_unobserved_run                           mokiterions-tui/tests/verification.rs  an_operator_ended_run_is_a_prefix_of_the_unobserved_run
   verification::drawing_is_pure                                                                   mokiterions-tui/src/lib.rs             verification::drawing_is_pure
   verification::every_declared_viewport_has_its_derived_canvas_with_a_header_and_a_footer         mokiterions-tui/tests/verification.rs  every_declared_viewport_has_its_derived_canvas_with_a_header_and_a_footer
   verification::every_distinction_survives_the_loss_of_colour                                     mokiterions-tui/src/lib.rs             verification::every_distinction_survives_the_loss_of_colour
   verification::every_presented_value_is_the_snapshots                                            mokiterions-tui/src/lib.rs             verification::every_presented_value_is_the_snapshots
   verification::exports_are_reproducible_and_are_the_engines_own_records                          mokiterions-tui/tests/verification.rs  exports_are_reproducible_and_are_the_engines_own_records
   verification::holding_consumes_nothing_however_long_it_is_held                                  mokiterions-tui/tests/verification.rs  holding_consumes_nothing_however_long_it_is_held
   verification::layout_reads_nothing_but_the_dimensions                                           mokiterions-tui/tests/verification.rs  layout_reads_nothing_but_the_dimensions
   verification::no_frame_carries_an_environment_value                                             mokiterions-tui/tests/verification.rs  no_frame_carries_an_environment_value
   verification::no_shipped_decision_source_has_a_proposal_rejected                                mokiterions-tui/tests/verification.rs  no_shipped_decision_source_has_a_proposal_rejected
   verification::observed_and_unobserved_runs_are_identical_on_every_declared_seed                 mokiterions-tui/tests/verification.rs  observed_and_unobserved_runs_are_identical_on_every_declared_seed
   verification::one_advance_is_one_tick_and_a_finished_run_refuses                                mokiterions-tui/tests/verification.rs  one_advance_is_one_tick_and_a_finished_run_refuses
   verification::overview_encodes_no_resource_class_and_detail_zoom_does                           mokiterions-tui/src/lib.rs             verification::overview_encodes_no_resource_class_and_detail_zoom_does
   verification::per_tick_records_match_so_the_observer_draws_no_entropy                           mokiterions-tui/tests/verification.rs  per_tick_records_match_so_the_observer_draws_no_entropy
   verification::presentation_state_survives_every_resize                                          mokiterions-tui/src/lib.rs             verification::presentation_state_survives_every_resize
   verification::the_applied_action_presented_is_always_the_engines                                mokiterions-tui/tests/verification.rs  the_applied_action_presented_is_always_the_engines
   verification::the_declared_sets_are_the_contracts                                               mokiterions-tui/tests/verification.rs  the_declared_sets_are_the_contracts
   verification::the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault             mokiterions-tui/src/lib.rs             verification::the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault
```

## The three identifiers that are not test names

The resolver treats any snake_case identifier in a mapping document as a candidate test name, which
picks up three that are not. Each is accounted for rather than left as an unresolved citation.

- **`every_documented_option_is_accepted_by_the_parser`**, cited by `WO-MOK-004/new-tests.md`. It is
  the name `WO-MOK-004` authorized; the test was written as
  `the_documented_options_are_exactly_the_options_the_parser_accepts`, and that document's own
  section *One name differs from the work order* discloses the difference and its reason. The test
  exists and is in the census; only the authorized name was never used. Nothing about this change
  touched it.
- **`replace_decisions_for_test`** and **`replace_snapshot_for_test`**, cited by
  `WO-MOK-005/requirement-to-test-mapping.md`. These are two of the four `#[cfg(test)]` hooks, named
  in that document's prose to explain how the tests using them reach an otherwise unreachable state.
  They are functions, not tests. Both still carry `#[cfg(test)]` at `state.rs:604` and `state.rs:617`
  and `hooks-and-visibility.txt` records them.

## The 17 tests no prior mapping names

169 tests exist; 152 are cited by name in a prior mapping. These are the remaining 17, with the
binary each runs in:

```
   mokiterions-tui/src/main.rs              a_cadence_that_has_never_run_is_due_and_one_just_run_is_not
   mokiterions-tui/tests/export.rs          a_written_file_holds_exactly_the_rendered_records
   mokiterions-tui/src/main.rs              an_export_path_is_not_touched_at_start_up
   mokiterions-tui/src/main.rs              an_invalid_input_is_refused_before_the_terminal_with_code_two
   mokiterions-tui/src/lib.rs               an_overlay_covers_the_body_and_leaves_the_header_and_the_footer
   mokiterions-tui/src/main.rs              help_exits_successfully_on_standard_output
   mokiterions-tui/tests/options.rs         help_wins_over_every_other_input
   mokiterions-tui/tests/state.rs           initial_state_is_the_specified_one
   mokiterions-core/src/lib.rs              movement_crosses_territory_and_is_observable
   mokiterions-tui/tests/options.rs         observer_inputs_are_validated
   mokiterions-tui/tests/options.rs         simulation_inputs_keep_the_engine_parser_and_its_rejections
   mokiterions-core/src/lib.rs              sleep_restores_energy_without_exceeding_the_maximum
   mokiterions-tui/src/lib.rs               the_log_cursor_scrolls_only_inside_the_log_overlay
   mokiterions-tui/tests/layout.rs          the_one_to_one_threshold_with_the_inspector_shown_is_157_columns
   mokiterions-tui/src/main.rs              the_tick_interval_is_a_thousand_milliseconds_over_the_speed
   mokiterions-tui/tests/state.rs           the_world_extent_matches_the_engine
   mokiterions-tui/tests/options.rs         tracing_is_always_on_and_cannot_be_turned_off
```

Each is covered by a prior contract's case without being named in that contract's mapping
prose — a mapping document cites the test that most directly discharges a clause, and a clause
frequently has more than one test against it. Two examples make the pattern clear:
`the_world_extent_matches_the_engine` and `initial_state_is_the_specified_one` both fall under
`VER-MOK-005`'s initial-state clause, which cites a different test for it; and
`sleep_restores_energy_without_exceeding_the_maximum` falls under `VER-MOK-002`'s bounded-attribute
clause, which cites `attributes_stay_within_bounds_across_a_long_reference_run`.

**What is claimed here is narrow, and stated so it cannot be over-read.** No requirement's coverage
rests on one of these 17 alone — that is what "152 of 152 cited tests resolve" establishes, since
every mapped clause keeps the test it was mapped to. What is *not* claimed is that the 17 are
redundant or removable. They were not audited for that, and this work order neither added nor removed
a test.

## What this document establishes

- Every test any prior mapping relies on still exists in the candidate, under a stated path.
- The three requirements this work order introduces are discharged by named measurements in this
  packet, each of which can be read independently.
- The prior mappings need no edit: with the prefix rule for the engine and the file table for the
  observer, each remains usable as written against the tree it described.

It does not restate the prior mappings' requirement claims, and does not revisit them. `VER-MOK-006`
is a conformance contract about a relocation, not a re-verification of `REQ-MOK-001` through
`REQ-MOK-027`, and this document does not pretend otherwise.
