# Verification output

Captured 2026-08-19 in a linked worktree of the primary clone, on branch `feature/release-ci`, HEAD
`54c21abcfb9caa4474c9ca5f194289e055c86a23`, with this work order's changes present but uncommitted.
The worktree's absolute path is redacted to `<checkout>` here and below: no other evidence file in
this repository carries an absolute local path, and none of the findings depend on it.

The commands are the ones `docs/engineering/REPOSITORY_CONTEXT.md` declares, plus the dependency-tree
command `mokiterions-core/Cargo.toml` names. They are the same commands the workflow's `verify` job
re-runs at the authorized commit.

**Harness build.** The four harness rows below are harness `0.4.0`, the version
`.engineering-harness.toml` declares and the version the workflow installs as a pinned wheel. The
qualification matters because `doctor` and `preflight` compare this repository against the templates
their own release ships, so their verdicts are a function of the harness build as well as of the
repository: the same commit that passes under `0.4.0` reports eight `distribution:` failures under
`0.4.1`. `compliance-rehearsal.md` records that in full, under C1, together with the rule 7.1 check
that turns it into one sentence instead of eight.

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | exit 0, no diff |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, no warning |
| `cargo test --workspace --locked` | exit 0 — **172 passed, 0 failed, 0 ignored**, 19 binaries |
| `cargo tree -p Mokiterions --locked` | one crate |
| `python -m se_harness doctor .` | PASS, every managed file unchanged |
| `python scripts/validate_engineering_artifacts.py --root .` | PASS — 79 artifacts, 0 errors, 0 warnings |
| `python -m se_harness inspect .` | reports the ten new `draft` artifacts |
| `python -m se_harness preflight . --work-order WO-MOK-009 --phase review` | **exit 1**, as expected while the chain is `draft` |

## The test count is unchanged, and why that is checkable

`WO-MOK-009` touches no Rust. `git status --porcelain --untracked-files=all` filtered to
`*Cargo.toml`, `*Cargo.lock` and `*.rs` prints **nothing**: no source file, no manifest and no
lockfile is in this changeset. The suite that ran is therefore the same suite that ran at
`54c21abc`, and 172 is that suite's count rather than a number this work produced.

One caveat stated plainly: `rust-toolchain.toml` is *new* in this changeset, so it could in principle
change which compiler runs the suite. It does not — the pin is `1.97.1` and the compiler already
installed and selected is `rustc 1.97.1 (8bab26f4f 2026-07-14)`. See `toolchain-evidence.md`.

## The preflight failure is the expected outcome, not a defect

`WO-MOK-009` is `draft`, and its own stop condition says so: preflight cannot pass for a `draft` work
order whose governing chain is also `draft`. The diagnostics are exactly the two the validator's
status rules predict — `W005` for the work order's own status, `W013` once per inactive governing
artifact — and no others:

```text
[W005] …/work-orders/WO-MOK-009.md: status 'draft' is not eligible for review;
       expected one of approved, implemented, in_progress, released, verified
[W013] …/capabilities/CAP-MOK-007.md: governing artifact CAP-MOK-007 is not active
[W013] …/intent/INT-MOK-007.md: governing artifact INT-MOK-007 is not active
[W013] …/requirements/REQ-MOK-035.md … REQ-MOK-039.md: governing artifact … is not active
[W013] …/specifications/SPEC-MOK-005.md: governing artifact SPEC-MOK-005 is not active
[W013] …/verification/VER-MOK-008.md: governing artifact VER-MOK-008 is not active
```

Every one of those clears by an approval transition, and every one of those transitions is an
accountable owner's act. None was performed here. `validate` passing with 0 errors and 0 warnings
while `preflight` fails is the harness distinguishing *the graph is well-formed* from *this work is
authorized to proceed* — and only the second is outstanding.

## Full transcripts

### Cargo

```text
$ cargo fmt --all -- --check
exit=0

$ cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.62s
exit=0

$ cargo test --workspace --locked
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.47s
     Running unittests src\lib.rs (target\debug\deps\mokiterions-3867f5b7d4d681ad.exe)

running 37 tests
test simulation::tests::co_located_entities_are_reported_at_distance_zero_without_a_direction ... ok
test simulation::tests::food_regenerates_only_in_nonempty_nonfull_territories ... ok
test simulation::tests::eating_is_atomic_bounded_and_single_use ... ok
test simulation::tests::food_regeneration_respects_capacity ... ok
test simulation::tests::initial_world_population_and_food_match_the_contract ... ok
test simulation::tests::extinction_takes_precedence_at_the_tick_limit ... ok
test simulation::tests::building_an_observation_consumes_no_entropy_and_mutates_nothing ... ok
test simulation::tests::invalid_move_does_not_mutate_action_state ... ok
test simulation::tests::perception_crosses_the_territory_boundary ... ok
test simulation::tests::movement_crosses_territory_and_is_observable ... ok
test simulation::tests::perception_excludes_distant_resources_and_dead_neighbours ... ok
test simulation::tests::initialization_is_seeded_and_reproducible ... ok
test simulation::tests::perception_grants_no_ability_to_act_at_a_distance ... ok
test simulation::tests::perception_order_is_stable_and_independent_of_collection_order ... ok
test simulation::tests::splitmix64_sequence_is_stable ... ok
test simulation::tests::perception_reports_in_radius_food_with_class_direction_and_distance ... ok
test simulation::tests::perception_reports_living_neighbours_and_never_the_observer ... ok
test simulation::tests::regeneration_adds_only_what_remaining_capacity_allows ... ok
test simulation::tests::perception_is_symmetric_between_living_mokiterions ... ok
test simulation::tests::sleep_restores_energy_without_exceeding_the_maximum ... ok
test simulation::tests::survival_decay_saturates_and_death_is_final ... ok
test simulation::tests::the_radius_boundary_is_inclusive_and_exclusive_by_one_cell ... ok
test simulation::tests::the_reference_source_approaches_then_consumes_a_perceived_resource ... ok
test simulation::tests::the_reference_source_does_not_approach_a_resource_it_would_decline ... ok
test simulation::tests::the_reference_source_does_not_consume_a_resource_it_does_not_need ... ok
test simulation::tests::the_reference_source_cannot_mutate_authoritative_state ... ok
test simulation::tests::the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach ... ok
test simulation::tests::the_reference_source_prefers_the_nearest_then_richest_resource ... ok
test simulation::tests::the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing ... ok
test simulation::tests::the_reference_source_sustains_itself_before_seeking_or_searching ... ok
test simulation::tests::untrusted_decisions_are_validated_and_traced ... ok
test simulation::tests::action_tracing_is_optional_complete_and_observational ... ok
test simulation::tests::repeated_runs_are_byte_identical ... ok
test simulation::tests::the_reference_source_never_waits_and_never_runs_its_energy_to_zero ... ok
test simulation::tests::attributes_stay_within_bounds_across_a_long_reference_run ... ok
test simulation::tests::both_sources_run_are_reported_and_are_byte_identically_reproducible ... ok
test simulation::tests::density_binds_initialization_capacity_and_the_replenishment_target ... ok

test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

     Running unittests src\main.rs (target\debug\deps\Mokiterions-c8f7f3eae82c82c7.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\cli.rs (target\debug\deps\cli-ea1fe31a556a11ea.exe)

running 12 tests
test both_policies_are_selectable_and_reference_is_the_default ... ok
test density_is_accepted_in_the_specified_forms_and_rejected_otherwise ... ok
test defaults_are_stable ... ok
test the_entries_state_the_constraints_that_decide_validity ... ok
test each_documented_default_parses_to_the_applied_default ... ok
test options_work_in_any_order ... ok
test the_documented_options_are_exactly_the_options_the_parser_accepts ... ok
test duplicates_and_missing_values_are_rejected ... ok
test each_declared_default_is_stated_once ... ok
test the_help_text_states_order_and_repetition ... ok
test the_flags_state_their_effect_and_no_default_value ... ok
test every_option_the_synopsis_names_has_an_options_entry ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\density.rs (target\debug\deps\density-aa384b05c61f2961.exe)

running 2 tests
test a_density_resolving_to_no_resources_is_rejected ... ok
test density_resolves_to_the_specified_resource_count ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\process.rs (target\debug\deps\process-44af230791f0b687.exe)

running 5 tests
test the_diagnostic_path_appends_the_whole_usage_text ... ok
test a_density_resolving_to_no_resources_exits_with_code_two_before_initialization ... ok
test invalid_configuration_exits_with_code_two ... ok
test help_exits_successfully ... ok
test output_failure_exits_with_code_one ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\termination.rs (target\debug\deps\termination-3962b6ea9d95e5a7.exe)

running 3 tests
test tick_limit_terminates_with_one_summary ... ok
test a_long_configured_run_is_bounded_and_does_not_panic ... ok
test a_long_run_is_bounded_under_either_source ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s

     Running tests\viability.rs (target\debug\deps\viability-6b215a30b755f00d.exe)

running 1 test
test the_reference_source_sustains_the_population_at_every_declared_density ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.40s

     Running unittests src\lib.rs (target\debug\deps\mokiterions_tui-2fbcc6609753ce76.exe)

running 24 tests
test render::tests::a_bar_row_shrinks_to_its_pane_and_never_overflows_it ... ok
test render::tests::a_depleted_territory_is_stated_in_words_at_every_width ... ok
test render::tests::the_bar_row_reproduces_the_specified_form ... ok
test render::tests::a_zero_value_is_a_zero_and_an_absent_value_is_a_dash ... ok
test render::tests::the_help_overlay_lists_every_bound_key ... ok
test state::tests::the_log_cursor_scrolls_only_inside_the_log_overlay ... ok
test render::tests::the_territory_rule_marks_the_row_between_the_territories ... ok
test render::tests::the_footer_survives_the_narrowest_viewport ... ok
test state::tests::the_highlighted_record_is_the_newest_until_the_operator_scrolls ... ok
test render::tests::an_overlay_covers_the_body_and_leaves_the_header_and_the_footer ... ok
test render::tests::the_authority_overlay_names_identifiers_for_every_event_type ... ok
test render::tests::a_resize_changes_the_layout_and_nothing_else ... ok
test verification::every_presented_value_is_the_snapshots ... ok
test render::tests::the_log_shows_the_newest_records_and_reports_an_empty_filter ... ok
test verification::every_distinction_survives_the_loss_of_colour ... ok
test state::tests::selection_clears_itself_when_no_living_mokiterion_remains ... ok
test render::tests::the_inspector_states_absence_rather_than_inventing_a_subject ... ok
test verification::the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault ... ok
test verification::overview_encodes_no_resource_class_and_detail_zoom_does ... ok
test verification::a_filter_changes_what_is_presented_and_nothing_else ... ok
test verification::drawing_is_pure ... ok
test verification::a_degenerate_world_still_draws_a_frame ... ok
test state::tests::a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour ... ok
test verification::presentation_state_survives_every_resize ... ok

test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.36s

     Running unittests src\main.rs (target\debug\deps\mokiterions_tui-cacaff985fad9c9e.exe)

running 8 tests
test tests::a_cadence_that_has_never_run_is_due_and_one_just_run_is_not ... ok
test tests::the_tick_interval_is_a_thousand_milliseconds_over_the_speed ... ok
test tests::help_exits_successfully_on_standard_output ... ok
test tests::an_invalid_input_is_refused_before_the_terminal_with_code_two ... ok
test tests::the_idle_wait_never_exceeds_the_nearest_deadline ... ok
test tests::an_export_path_is_not_touched_at_start_up ... ok
test tests::a_run_the_operator_ended_reports_itself_as_ended_early ... ok
test tests::a_viewport_below_the_floor_is_refused_with_both_dimensions_and_code_two ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\authority.rs (target\debug\deps\authority-eff95bcdd131dbd9.exe)

running 4 tests
test an_ordinary_record_resolves_from_its_own_payload ... ok
test the_decision_source_maps_by_the_source_the_record_names ... ok
test the_mapping_is_the_specified_one ... ok
test every_event_type_the_observer_can_present_has_an_entry ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\export.rs (target\debug\deps\export-a7c26a0bd291cfde.exe)

running 7 tests
test records_use_the_engines_own_line_format_in_authoritative_order ... ok
test nothing_environment_specific_reaches_the_file ... ok
test the_default_path_is_relative_and_derived_from_the_run ... ok
test the_same_records_always_produce_the_same_bytes ... ok
test an_unwritable_path_is_reported_and_leaves_nothing_behind ... ok
test a_written_file_holds_exactly_the_rendered_records ... ok
test the_closing_line_states_the_count_and_the_truncation ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running tests\layout.rs (target\debug\deps\layout-428970b899246a03.exe)

running 10 tests
test each_pane_appears_at_its_threshold_on_the_axis_that_constrains_it ... ok
test the_log_is_ten_rows_only_where_both_thresholds_are_met ... ok
test the_one_to_one_threshold_with_the_inspector_shown_is_157_columns ... ok
test the_declared_viewports_yield_the_declared_canvases ... ok
test the_floor_is_the_specified_one ... ok
test the_one_to_one_threshold_with_the_roster_alone_is_113_columns ... ok
test the_vertical_one_to_one_threshold_is_44_rows ... ok
test every_region_stays_inside_the_viewport_and_the_body_rows_are_contiguous ... ok
test excluded_panes_are_the_ones_the_viewport_omits ... ok
test enlarging_the_viewport_never_removes_a_pane ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\options.rs (target\debug\deps\options-9145103ef3454b8b.exe)

running 7 tests
test an_export_path_is_taken_verbatim_as_data ... ok
test defaults_match_the_specified_values ... ok
test simulation_inputs_keep_the_engine_parser_and_its_rejections ... ok
test speed_steps_are_clamped_at_both_ends ... ok
test tracing_is_always_on_and_cannot_be_turned_off ... ok
test observer_inputs_are_validated ... ok
test help_wins_over_every_other_input ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\render.rs (target\debug\deps\render-672d1ffbd6e3eb65.exe)

running 8 tests
test detail_zoom_places_every_visible_entity_at_its_mapped_cell ... ok
test below_the_floor_nothing_is_presented ... ok
test a_reported_failure_reaches_the_header ... ok
test the_header_names_the_panes_that_are_only_overlays ... ok
test the_footer_carries_the_provenance_and_nothing_environment_specific ... ok
test a_region_states_the_world_range_it_presents ... ok
test every_declared_viewport_renders_and_annotates_what_it_presents ... ok
test drawing_never_advances_the_simulation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running tests\spatial.rs (target\debug\deps\spatial-6b42680631cd88b2.exe)

running 7 tests
test a_character_cell_covers_two_by_four_world_cells_in_overview_and_one_in_detail ... ok
test the_camera_is_clamped_so_the_region_never_leaves_the_world ... ok
test the_whole_world_needs_both_axes_and_never_width_alone ... ok
test glyphs_are_the_assigned_ones ... ok
test territory_a_is_above_territory_b ... ok
test the_territory_rule_is_present_exactly_when_the_boundary_is_visible ... ok
test the_overview_dot_grid_is_one_dot_per_world_cell ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests\state.rs (target\debug\deps\state-269098bb3c362ab5.exe)

running 21 tests
test an_unbound_key_changes_nothing ... ok
test a_whole_world_overview_cannot_be_panned_off_the_world ... ok
test a_subject_filter_needs_a_selection ... ok
test escape_closes_an_overlay_before_it_clears_a_selection ... ok
test a_key_release_is_not_a_press ... ok
test every_overlay_has_its_bound_key ... ok
test following_centres_the_selection_and_clamps_identically ... ok
test filtering_changes_presentation_only ... ok
test a_single_step_is_accepted_only_while_held_and_advances_exactly_one_tick ... ok
test panning_moves_one_world_cell_and_clamps_at_every_edge ... ok
test initialization_events_are_retained_in_authoritative_order ... ok
test quit_is_the_only_key_that_asks_to_exit ... ok
test a_finished_run_refuses_to_advance_and_stays_inspectable ... ok
test selection_cycles_in_roster_order_and_escape_clears_it ... ok
test shared_cells_are_counted_at_the_rendered_granularity ... ok
test speed_steps_through_the_fixed_ladder_and_clamps ... ok
test the_world_extent_matches_the_engine ... ok
test the_type_filter_cycles_the_whole_vocabulary_then_returns_to_none ... ok
test initial_state_is_the_specified_one ... ok
test a_death_carries_the_tick_and_the_engine_computed_final_values ... ok
test the_event_buffer_drops_the_oldest_record_and_says_so ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests\verification.rs (target\debug\deps\verification-eddf947fada740f8.exe)

running 16 tests
test an_injected_export_failure_leaves_the_tick_intact ... ok
test layout_reads_nothing_but_the_dimensions ... ok
test one_advance_is_one_tick_and_a_finished_run_refuses ... ok
test a_finished_run_stays_inspectable_and_exportable ... ok
test the_declared_sets_are_the_contracts ... ok
test a_death_removes_the_subject_from_the_presentation_and_is_corroborated ... ok
test every_declared_viewport_has_its_derived_canvas_with_a_header_and_a_footer ... ok
test the_applied_action_presented_is_always_the_engines ... ok
test an_operator_ended_run_is_a_prefix_of_the_unobserved_run ... ok
test no_shipped_decision_source_has_a_proposal_rejected ... ok
test a_smaller_world_row_never_renders_below_a_larger_one ... ok
test exports_are_reproducible_and_are_the_engines_own_records ... ok
test per_tick_records_match_so_the_observer_draws_no_entropy ... ok
test holding_consumes_nothing_however_long_it_is_held ... ok
test observed_and_unobserved_runs_are_identical_on_every_declared_seed ... ok
test no_frame_carries_an_environment_value ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.57s

   Doc-tests mokiterions

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mokiterions_tui

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

exit=0

$ cargo tree -p Mokiterions --locked
Mokiterions v0.1.0 (<checkout>\mokiterions-core)
exit=0
```

### Harness

```text
$ python -m se_harness doctor .
PASS AGENTS.md: required
PASS CLAUDE.md: required
PASS ENGINEERING_HARNESS.md: required
PASS claude-import: @AGENTS.md
PASS config: .engineering-harness.toml
PASS distribution:.engineering-harness.toml: matches distribution
PASS distribution:.github/workflows/engineering-harness.yml: matches distribution
PASS distribution:.gitignore: matches distribution
PASS distribution:AGENTS.md: matches distribution
PASS distribution:CLAUDE.md: matches distribution
PASS distribution:ENGINEERING_HARNESS.md: matches distribution
PASS distribution:docs/engineering/DECISION_RIGHTS.md: matches distribution
PASS distribution:docs/engineering/QUALITY_GATES.md: matches distribution
PASS distribution:docs/engineering/TRACEABILITY.md: matches distribution
PASS distribution:docs/engineering/WORKFLOW.md: matches distribution
PASS distribution:docs/engineering/templates/ADR.template.md: matches distribution
PASS distribution:docs/engineering/templates/ARCHITECTURE.template.md: matches distribution
PASS distribution:docs/engineering/templates/CAPABILITY.template.md: matches distribution
PASS distribution:docs/engineering/templates/INTENT.template.md: matches distribution
PASS distribution:docs/engineering/templates/OPERATING_CONTRACT.template.md: matches distribution
PASS distribution:docs/engineering/templates/README.md: matches distribution
PASS distribution:docs/engineering/templates/RELEASE_CONTRACT.template.md: matches distribution
PASS distribution:docs/engineering/templates/RELEASE_RECORD.template.md: matches distribution
PASS distribution:docs/engineering/templates/REQUIREMENT.template.md: matches distribution
PASS distribution:docs/engineering/templates/SPECIFICATION.template.md: matches distribution
PASS distribution:docs/engineering/templates/VERIFICATION.template.md: matches distribution
PASS distribution:docs/engineering/templates/VERIFICATION_RECORD.template.md: matches distribution
PASS distribution:docs/engineering/templates/WORK_ORDER.template.md: matches distribution
PASS distribution:scripts/artifact_layout_registry.py: matches distribution
PASS distribution:scripts/check_engineering_harness.ps1: matches distribution
PASS distribution:scripts/check_engineering_harness.sh: matches distribution
PASS distribution:scripts/generate_harness_dashboard.py: matches distribution
PASS distribution:scripts/harness_explorer/index.template.html: matches distribution
PASS distribution:scripts/inspect_engineering_artifacts.py: matches distribution
PASS distribution:scripts/select_harness_work_order.py: matches distribution
PASS distribution:scripts/validate_engineering_artifacts.py: matches distribution
PASS docs/engineering/DECISION_RIGHTS.md: required
PASS docs/engineering/QUALITY_GATES.md: required
PASS docs/engineering/README.md: required
PASS docs/engineering/REPOSITORY_CONTEXT.md: required
PASS docs/engineering/TRACEABILITY.md: required
PASS docs/engineering/WORKFLOW.md: required
PASS lock: .engineering-harness.lock
PASS managed:.engineering-harness.toml: unchanged
PASS managed:.github/workflows/engineering-harness.yml: unchanged
PASS managed:.gitignore: unchanged
PASS managed:AGENTS.md: unchanged
PASS managed:CLAUDE.md: unchanged
PASS managed:ENGINEERING_HARNESS.md: unchanged
PASS managed:docs/engineering/DECISION_RIGHTS.md: unchanged
PASS managed:docs/engineering/QUALITY_GATES.md: unchanged
PASS managed:docs/engineering/TRACEABILITY.md: unchanged
PASS managed:docs/engineering/WORKFLOW.md: unchanged
PASS managed:docs/engineering/templates/ADR.template.md: unchanged
PASS managed:docs/engineering/templates/ARCHITECTURE.template.md: unchanged
PASS managed:docs/engineering/templates/CAPABILITY.template.md: unchanged
PASS managed:docs/engineering/templates/INTENT.template.md: unchanged
PASS managed:docs/engineering/templates/OPERATING_CONTRACT.template.md: unchanged
PASS managed:docs/engineering/templates/README.md: unchanged
PASS managed:docs/engineering/templates/RELEASE_CONTRACT.template.md: unchanged
PASS managed:docs/engineering/templates/RELEASE_RECORD.template.md: unchanged
PASS managed:docs/engineering/templates/REQUIREMENT.template.md: unchanged
PASS managed:docs/engineering/templates/SPECIFICATION.template.md: unchanged
PASS managed:docs/engineering/templates/VERIFICATION.template.md: unchanged
PASS managed:docs/engineering/templates/VERIFICATION_RECORD.template.md: unchanged
PASS managed:docs/engineering/templates/WORK_ORDER.template.md: unchanged
PASS managed:scripts/artifact_layout_registry.py: unchanged
PASS managed:scripts/check_engineering_harness.ps1: unchanged
PASS managed:scripts/check_engineering_harness.sh: unchanged
PASS managed:scripts/generate_harness_dashboard.py: unchanged
PASS managed:scripts/harness_explorer/index.template.html: unchanged
PASS managed:scripts/inspect_engineering_artifacts.py: unchanged
PASS managed:scripts/select_harness_work_order.py: unchanged
PASS managed:scripts/validate_engineering_artifacts.py: unchanged
PASS python: 3.14.6
PASS scripts/generate_harness_dashboard.py: required
PASS scripts/harness_explorer/index.template.html: required
PASS scripts/validate_engineering_artifacts.py: required
PASS seed:.github/PULL_REQUEST_TEMPLATE.md: present
PASS seed:docs/engineering/README.md: present
PASS seed:docs/engineering/REPOSITORY_CONTEXT.md: present
exit=0

$ python scripts/validate_engineering_artifacts.py --root .
Engineering artifact validation: PASS
Artifacts: 79 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
exit=0

$ python -m se_harness inspect .
Harness inspection
Repository: Mokiterions-release-ci @ 54c21abcfb9caa4474c9ca5f194289e055c86a23
Formal validation: PASS
Graph: 79 artifacts | 238 relations | 15 findings
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
Finding severity: error 0 | warning 9 | info 6

Decision required (0):
- none

Definitions pending (11):
- CAP-MOK-007 [draft] complete-definition: Publish a release that refuses to exist without a recorded authorization (docs/engineering/simulation/capabilities/CAP-MOK-007.md)
- INT-MOK-007 [draft] complete-definition: Make a released build traceable to the decisions that authorized it (docs/engineering/simulation/intent/INT-MOK-007.md)
- REQ-MOK-035 [draft] complete-definition: Refuse to produce or publish a release the graph does not authorize (docs/engineering/simulation/requirements/REQ-MOK-035.md)
- REQ-MOK-036 [draft] complete-definition: Re-establish declared compliance at the released revision before publishing (docs/engineering/simulation/requirements/REQ-MOK-036.md)
- REQ-MOK-037 [draft] complete-definition: Carry provenance inside every published asset (docs/engineering/simulation/requirements/REQ-MOK-037.md)
- REQ-MOK-038 [draft] complete-definition: Confine the release process to acts that are not accountable decisions (docs/engineering/simulation/requirements/REQ-MOK-038.md)
- REQ-MOK-039 [draft] complete-definition: Build and verify a release with one compiler version the repository declares (docs/engineering/simulation/requirements/REQ-MOK-039.md)
- SPEC-MOK-005 [draft] complete-definition: Release authorization, compliance re-establishment, asset provenance and reserved acts (docs/engineering/simulation/specifications/SPEC-MOK-005.md)
- VER-MOK-008 [draft] complete-definition: Release authorization, compliance, provenance and reserved-act verification (docs/engineering/simulation/verification/VER-MOK-008.md)
- WO-MOK-008 [draft] complete-definition: Make the provenance footer shed fields without losing authoritative information (docs/engineering/simulation/work-orders/WO-MOK-008.md)
- WO-MOK-009 [draft] complete-definition: Implement the release authorization gate, the release process and the compiler declaration (docs/engineering/simulation/work-orders/WO-MOK-009.md)

Active work (0):
- none

Assurance pending (0):
- none

Findings (15):
- [WARNING] W-HEX-001 (derived): 6 observations [WO-MOK-001, WO-MOK-002, WO-MOK-003, WO-MOK-004, WO-MOK-005, WO-MOK-006]
- [WARNING] W-HEX-003 (derived): 3 observations [ADR-MOK-001, ARCH-MOK-001, ARCH-MOK-002, SPEC-MOK-003, SPEC-MOK-004]
- [INFO] I-REV-001 (derived): 6 observations [VREC-MOK-001, VREC-MOK-002, VREC-MOK-003, VREC-MOK-004, VREC-MOK-005, VREC-MOK-006]

Suggested next steps (20):
- W-HEX-001 -> retain-work-order-evidence (engineering-owner): Retain evidence keyed to the implemented work order and reassess the observation. Repeated for 6 source observations. [WO-MOK-001, WO-MOK-002, WO-MOK-003, WO-MOK-004, WO-MOK-005, WO-MOK-006]
- W-HEX-003 -> reassess-dependent-artifact (artifact-owner): Reassess the older source against its newer declared dependency or parent. Repeated for 3 source observations. [ADR-MOK-001, ARCH-MOK-001, ARCH-MOK-002, SPEC-MOK-003, SPEC-MOK-004]
- definition_pending -> complete-or-dispose-definition (artifact-owner): Complete the definition or explicitly dispose of it through an allowed governed state. Repeated for 11 source observations. [CAP-MOK-007, INT-MOK-007, REQ-MOK-035, REQ-MOK-036, REQ-MOK-037, REQ-MOK-038, REQ-MOK-039, SPEC-MOK-005, +3 more]

Authority: repository-local, derived observation. Inspection does not validate by exit status, approve, authorize, verify, release, or remediate.
exit=0

$ python -m se_harness preflight . --work-order WO-MOK-009 --phase review
Harness preflight: FAIL
Phase: review
Work order: WO-MOK-009 (draft)

Assurance classification:
- Commit-bound verification: required
- Decided by: engineering owner
- Rationale: This work decides whether an asset may leave the repository at all, so every later release decision rests on it being correct at a known commit. It is also the mechanism that re-establishes compliance for a release, which means a defect in it silently weakens the evidence behind every release that follows. The compiler declaration additionally changes the conditions under which all future evidence is produced. A commit-bound record is required because the refusal behavior is only meaningful as a statement about a specific revision of the gate and the process.

Diagnostics:
- [W005] docs/engineering/simulation/work-orders/WO-MOK-009.md: status 'draft' is not eligible for review; expected one of approved, implemented, in_progress, released, verified
- [W013] docs/engineering/simulation/capabilities/CAP-MOK-007.md: governing artifact CAP-MOK-007 is not active
- [W013] docs/engineering/simulation/intent/INT-MOK-007.md: governing artifact INT-MOK-007 is not active
- [W013] docs/engineering/simulation/requirements/REQ-MOK-035.md: governing artifact REQ-MOK-035 is not active
- [W013] docs/engineering/simulation/requirements/REQ-MOK-036.md: governing artifact REQ-MOK-036 is not active
- [W013] docs/engineering/simulation/requirements/REQ-MOK-037.md: governing artifact REQ-MOK-037 is not active
- [W013] docs/engineering/simulation/requirements/REQ-MOK-038.md: governing artifact REQ-MOK-038 is not active
- [W013] docs/engineering/simulation/requirements/REQ-MOK-039.md: governing artifact REQ-MOK-039 is not active
- [W013] docs/engineering/simulation/specifications/SPEC-MOK-005.md: governing artifact SPEC-MOK-005 is not active
- [W013] docs/engineering/simulation/verification/VER-MOK-008.md: governing artifact VER-MOK-008 is not active

Reading manifest:
- ENGINEERING_HARNESS.md
- docs/engineering/REPOSITORY_CONTEXT.md
- docs/engineering/README.md
- docs/engineering/WORKFLOW.md
- docs/engineering/DECISION_RIGHTS.md
- docs/engineering/QUALITY_GATES.md
- docs/engineering/TRACEABILITY.md
- docs/engineering/simulation/intent/INT-MOK-007.md
- docs/engineering/simulation/capabilities/CAP-MOK-007.md
- docs/engineering/simulation/requirements/REQ-MOK-035.md
- docs/engineering/simulation/requirements/REQ-MOK-036.md
- docs/engineering/simulation/requirements/REQ-MOK-037.md
- docs/engineering/simulation/requirements/REQ-MOK-038.md
- docs/engineering/simulation/requirements/REQ-MOK-039.md
- docs/engineering/simulation/specifications/SPEC-MOK-005.md
- docs/engineering/simulation/verification/VER-MOK-008.md
- docs/engineering/simulation/work-orders/WO-MOK-009.md

Repository commands:
- additional_required_verification: Run the engine with `cargo run --bin Mokiterions -- <options>` and inspect its text output; run the observer with `cargo run -p mokiterions-tui -- <options>` in an interactive terminal. `cargo tree -p Mokiterions` must resolve to one crate — the engine's dependency table is required to stay empty. Behavior-specific verification must be added as simulation requirements are approved.
- build: `cargo build`. `cargo build -p Mokiterions` builds the engine package alone, which is the form that answers a question about the engine. The root is a virtual workspace manifest, so a bare `cargo build` builds both members; the `-p` form is the one to reach for when the answer must be about one package.
- lint_or_format: `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- setup: Install `rustup`. `rust-toolchain.toml` pins the toolchain, so the first `cargo` command run in this repository installs and selects that exact version together with `rustfmt` and `clippy`; do not install a toolchain by hand and do not override the pin with `+stable`. Harness commands run as `python -m se_harness` — `harnessctl` is not on `PATH`. No additional project setup is currently required.
- test: `cargo test` runs both packages and both tiers of each. `cargo test -p Mokiterions` runs the engine package alone, with no terminal present; `cargo test -p mokiterions-tui` runs the observer package alone, also with no terminal present.

Authority boundary: Preflight is derived, read-only evidence. It does not approve artifacts, authorize a diff, verify work, release software, commit, push, tag, publish, or deploy.
exit=1
```

## Re-run after the evidence landed

The table at the top of this file was captured before the last four evidence records were written.
Writing evidence adds files under `docs/engineering/simulation/evidence/WO-MOK-009/`, and the validator
excludes `evidence` directories from its walk, so the artifact count should not move — but "should not"
is a prediction, and rule 4.2's excluded-directory defect was found precisely because that prediction had
never been checked. Both commands were therefore run again, with the same pinned `0.4.0` wheel, after the
last file was written.

```text
$ python -m se_harness doctor .
81 verdict lines: PASS 81, WARN 0, FAIL 0
exit=0

$ python scripts/validate_engineering_artifacts.py --root .
Engineering artifact validation: PASS
Artifacts: 79 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
exit=0
```

Unchanged: 79 artifacts, 0 errors, 0 warnings, and every `distribution:` and `managed:` row still PASS.
Four files were added to the evidence directory between the two captures — `scenario-map.md`,
`candidate-conformance.md`, `completion-summary.md` and `README.md`, taking it to thirteen — and the
artifact count did not move. That is the observation that makes the exclusion a measured fact here rather
than a reading of the validator's source.

**One thing this does not establish.** `doctor` passing under `0.4.0` says nothing about `0.4.1`, where the
eight `distribution:` rows fail. That is not a defect in this repository — the rows compare against the
template the *installed* build ships — but it does mean a reader who runs `doctor` on a newer harness will
not reproduce the output above. The version that produced it is stated at the top of this file, and rule
7.1 is the check that turns the discrepancy into one legible sentence on a runner.

## Third capture — after the owners' decisions were written in

The two captures above both predate the four decisions recorded in `completion-summary.md`, "Settled on
2026-08-19". Two of those decisions changed the body of a governed artifact: `SPEC-MOK-005` rule 12.5 was
restated and gained a second amendment-record row, and `VER-MOK-008` C5 was rewritten. `WO-MOK-001.md` also
gained a paragraph. Editing a governed artifact re-exercises the validator's front-matter, traceability and
governance-plane checks in a way that adding an evidence file does not, so both commands were run a third
time against the tree as it now stands.

```text
$ python -m se_harness doctor .
81 verdict lines: PASS 81, WARN 0, FAIL 0
exit=0

$ python scripts/validate_engineering_artifacts.py --root .
Engineering artifact validation: PASS
Artifacts: 79 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
exit=0
```

Identical to the second capture, which is the expected result and the reason for taking it: the amendments
restate rules and methods in prose, and none of them adds, removes or re-links an artifact. Three artifacts
changed content and the artifact count, error count and warning count all held. What this does *not* say is
that the amended rules are right — that is the approval in step 3 and step 4 of `completion-summary.md`'s
next-steps table, and it is a reading, not a command's exit code.
