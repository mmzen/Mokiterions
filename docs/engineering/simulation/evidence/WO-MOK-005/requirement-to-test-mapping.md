# Requirement-to-test mapping — WO-MOK-005

One row per case in `VER-MOK-005`'s requirement-to-evidence matrix, in the contract's own order,
against the test or the retained file that discharges it. Test names are as `cargo test -- --list`
reports them; the `mokiterions-tui` package's targets are unqualified, and engine tests are marked.

Where a case is discharged by something other than a test named for it, the note says so plainly.
The last section lists every such case in one place, because a case covered by an aggregate test or
by a structural argument is weaker evidence than a case with its own assertion, and a reviewer
should not have to find that out by reading the table.

## `REQ-MOK-019` — Render the whole world in one view

| Case | Test | Note |
|---|---|---|
| Whole world at the reference viewport | `layout::tests::the_declared_viewports_yield_the_declared_canvases`, `verification::every_declared_viewport_has_its_derived_canvas_with_a_header_and_a_footer`, `spatial::tests::the_whole_world_needs_both_axes_and_never_width_alone` | Interior 67 × 32 asserted from the resolved layout and from the drawn buffer |
| Every entity represented | `verification::every_presented_value_is_the_snapshots` | Iterates the snapshot's agents and territories and requires each in the frame |
| Coordinate mapping and orientation | `spatial::tests::territory_a_is_above_territory_b`, `spatial::tests::the_overview_dot_grid_is_one_dot_per_world_cell`, `render::tests::detail_zoom_places_every_visible_entity_at_its_mapped_cell`, `verification::a_smaller_world_row_never_renders_below_a_larger_one` | The property case sweeps every declared viewport, both zooms and every camera position |
| Territory boundary is drawn | `spatial::tests::the_territory_rule_is_present_exactly_when_the_boundary_is_visible`, `render::tests::the_territory_rule_marks_the_row_between_the_territories` | |
| Per-territory standing counts are presented | `verification::every_presented_value_is_the_snapshots`, `render::tests::a_depleted_territory_is_stated_in_words_at_every_width` | Compared against the snapshot's `standing` and `capacity` |
| Dead Mokiterions are not rendered | `verification::a_death_removes_the_subject_from_the_presentation_and_is_corroborated` | Runs to a real death, then asserts the identifier absent from the roster pane and the glyph absent from the canvas in both zooms |
| Shared cell precedence and marking | `state::tests::shared_cells_are_counted_at_the_rendered_granularity`, `verification::every_distinction_survives_the_loss_of_colour` | The second asserts exactly one underlined cell and that it carries the lower identifier's glyph |
| Region annotation | `render::tests::a_region_states_the_world_range_it_presents`, `render::tests::every_declared_viewport_renders_and_annotates_what_it_presents` | The `34 × 22` and `140 × 44` ranges are asserted literally |
| Detail zoom is one cell per world cell | `render::tests::detail_zoom_places_every_visible_entity_at_its_mapped_cell`, `spatial::tests::a_character_cell_covers_two_by_four_world_cells_in_overview_and_one_in_detail` | |
| Resource class glyphs appear per class | `spatial::tests::glyphs_are_the_assigned_ones`, `verification::overview_encodes_no_resource_class_and_detail_zoom_does` | |
| Mokiterion glyph takes precedence over a co-located resource | `render::tests::detail_zoom_places_every_visible_entity_at_its_mapped_cell` | |
| Overview encodes no per-resource class | `verification::overview_encodes_no_resource_class_and_detail_zoom_does` | Asserts the three class glyphs absent from the overview canvas and present in the detail canvas, on one snapshot holding one resource of each class |
| Degenerate worlds render | `verification::a_degenerate_world_still_draws_a_frame` | No living Mokiterions, no standing resources, and both, each at all six renderable viewports |
| Legibility without colour | `verification::every_distinction_survives_the_loss_of_colour` | Reads a projection of the frame that colour has been discarded from, so it cannot pass by colour |

## `REQ-MOK-020` — Present survival state for every living Mokiterion

| Case | Test | Note |
|---|---|---|
| Twelve entries without scrolling at the reference viewport | `verification::every_presented_value_is_the_snapshots` | At tick 9 all twelve are living and every identifier is required in the frame |
| Roster order is acting order | `state::tests::selection_cycles_in_roster_order_and_escape_clears_it` | Ascending identifier order, which `SPEC-MOK-001` makes acting order |
| Bars and numerics agree | `render::tests::the_bar_row_reproduces_the_specified_form` | Asserted as the exact line for health 100, satiety 81 and energy 72 — see the caveats |
| Zero renders as `0` with an empty bar | `render::tests::a_zero_value_is_a_zero_and_an_absent_value_is_a_dash` | |
| Reserved space carries no value | `render::tests::the_bar_row_reproduces_the_specified_form` | Nothing follows the third value: no label, no dash, no zero |
| Collapse below 47 columns | `render::tests::a_zero_value_is_a_zero_and_an_absent_value_is_a_dash`, `render::tests::a_bar_row_shrinks_to_its_pane_and_never_overflows_it`, `layout::tests::the_declared_viewports_yield_the_declared_canvases` | The one-line form is asserted as an exact string |
| Death is corroborated | `verification::a_death_removes_the_subject_from_the_presentation_and_is_corroborated`, `state::tests::a_death_carries_the_tick_and_the_engine_computed_final_values` | The entry disappears, the living count falls by exactly the number of deaths, and the death total rises by the same |
| Applied action is the engine's | `verification::the_applied_action_presented_is_always_the_engines` | Every living Mokiterion, every tick of a fifteen-tick run |

## `REQ-MOK-021` — Expose proposed action and engine authority decision

| Case | Test | Note |
|---|---|---|
| Proposal, verdict and applied action are presented | `render::tests::the_inspector_states_absence_rather_than_inventing_a_subject`, `verification::every_presented_value_is_the_snapshots` | |
| A rejection presents the engine's ground | `verification::the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault` | Reached through the test hook; see the caveats |
| Rejection is not a fault | `verification::the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault` | No fault colouring, no error wording, no diagnostic |
| Proposal and outcome share a tick | `verification::every_presented_value_is_the_snapshots` | Structural: both are read from one `WorldSnapshot`, so they cannot come from different ticks |
| Verdict is never re-derived in the observer | `verification::the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault` | The injected decision is self-contradictory on purpose: a legal move carrying `rejected` |
| Nothing selected does not default | `render::tests::the_inspector_states_absence_rather_than_inventing_a_subject` | |
| Before tick 1 | `render::tests::the_inspector_states_absence_rather_than_inventing_a_subject` | "no proposal has yet been made" |
| Selected Mokiterion dies | `state::tests::a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour`, `state::tests::a_death_carries_the_tick_and_the_engine_computed_final_values` | |
| Absent attributes are absent | `render::tests::a_zero_value_is_a_zero_and_an_absent_value_is_a_dash`, `render::tests::the_bar_row_reproduces_the_specified_form` | No fear, traits, name, age, kills, combats, remembered locations, latency or per-agent entropy field exists to render |

## `REQ-MOK-022` — Present and export a filterable event log

| Case | Test |
|---|---|
| Newest events visible without operator action | `render::tests::the_log_shows_the_newest_records_and_reports_an_empty_filter`, `state::tests::the_highlighted_record_is_the_newest_until_the_operator_scrolls` |
| Line format is `SPEC-MOK-001`'s | `export::tests::records_use_the_engines_own_line_format_in_authoritative_order`, `state::tests::initialization_events_are_retained_in_authoritative_order` |
| Type and subject filters restrict presentation only | `state::tests::filtering_changes_presentation_only`, `state::tests::the_type_filter_cycles_the_whole_vocabulary_then_returns_to_none`, `state::tests::a_subject_filter_needs_a_selection`, `verification::a_filter_changes_what_is_presented_and_nothing_else` |
| Empty filter result is stated | `render::tests::the_log_shows_the_newest_records_and_reports_an_empty_filter` |
| Export ignores the active filter | `verification::a_filter_changes_what_is_presented_and_nothing_else` |
| Export path resolution | `export::tests::the_default_path_is_relative_and_derived_from_the_run`, `options::tests::an_export_path_is_taken_verbatim_as_data` |
| Export trailer | `export::tests::the_closing_line_states_the_count_and_the_truncation` |
| Export reproducibility | `export::tests::the_same_records_always_produce_the_same_bytes`, `verification::exports_are_reproducible_and_are_the_engines_own_records`, `export-fidelity.txt` |
| Buffer bound and truncation marker | `state::tests::the_event_buffer_drops_the_oldest_record_and_says_so`, `export::tests::the_closing_line_states_the_count_and_the_truncation` |
| Export failure handling | `export::tests::an_unwritable_path_is_reported_and_leaves_nothing_behind`, `render::tests::a_reported_failure_reaches_the_header`, `verification::an_injected_export_failure_leaves_the_tick_intact` |
| Exports contain no environment values | `export::tests::nothing_environment_specific_reaches_the_file` |

## `REQ-MOK-023` — Control run progression and observation focus by keyboard

| Case | Test |
|---|---|
| Every binding in rule 7 acts | `render::tests::the_help_overlay_lists_every_bound_key`, `state::tests::every_overlay_has_its_bound_key`, and the per-control cases below |
| Single-step is accepted only while held | `state::tests::a_single_step_is_accepted_only_while_held_and_advances_exactly_one_tick` |
| Speed steps are clamped | `state::tests::speed_steps_through_the_fixed_ladder_and_clamps`, `options::tests::speed_steps_are_clamped_at_both_ends` |
| Selection cycles living Mokiterions only | `state::tests::selection_cycles_in_roster_order_and_escape_clears_it`, `state::tests::a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour`, `state::tests::selection_clears_itself_when_no_living_mokiterion_remains` |
| Panning is clamped to the world | `state::tests::panning_moves_one_world_cell_and_clamps_at_every_edge`, `state::tests::a_whole_world_overview_cannot_be_panned_off_the_world`, `spatial::tests::the_camera_is_clamped_so_the_region_never_leaves_the_world` |
| Follow requires a selection | `state::tests::following_centres_the_selection_and_clamps_identically` |
| `Esc` precedence | `state::tests::escape_closes_an_overlay_before_it_clears_a_selection` |
| Unbound keys are ignored | `state::tests::an_unbound_key_changes_nothing` |
| A key press is applied exactly once | `state::tests::a_key_release_is_not_a_press`, `state::tests::speed_steps_through_the_fixed_ladder_and_clamps` |
| Stepping is never invisible | `state::tests::a_single_step_is_accepted_only_while_held_and_advances_exactly_one_tick` (asserts `force_draw`) |
| Finished run refuses to advance | `state::tests::a_finished_run_refuses_to_advance_and_stays_inspectable`, `verification::a_finished_run_stays_inspectable_and_exportable`, `verification::one_advance_is_one_tick_and_a_finished_run_refuses` |
| Quit is the only exit control | `state::tests::quit_is_the_only_key_that_asks_to_exit` |

## `REQ-MOK-024` — Degrade the layout across viewport sizes

| Case | Test |
|---|---|
| Pane placement at every declared viewport | `layout::tests::each_pane_appears_at_its_threshold_on_the_axis_that_constrains_it`, `layout::tests::the_log_is_ten_rows_only_where_both_thresholds_are_met`, `layout::tests::enlarging_the_viewport_never_removes_a_pane` |
| Canvas interior at every declared viewport | `layout::tests::the_declared_viewports_yield_the_declared_canvases`, `verification::every_declared_viewport_has_its_derived_canvas_with_a_header_and_a_footer` |
| Whole-world claim per viewport | `render::tests::every_declared_viewport_renders_and_annotates_what_it_presents`, `spatial::tests::the_whole_world_needs_both_axes_and_never_width_alone`, `layout::tests::the_one_to_one_threshold_with_the_inspector_shown_is_157_columns`, `layout::tests::the_one_to_one_threshold_with_the_roster_alone_is_113_columns`, `layout::tests::the_vertical_one_to_one_threshold_is_44_rows` |
| Header and footer are never excluded | `verification::every_declared_viewport_has_its_derived_canvas_with_a_header_and_a_footer`, `render::tests::the_footer_survives_the_narrowest_viewport`, `verification::a_degenerate_world_still_draws_a_frame` |
| Excluded panes are announced and reachable | `layout::tests::excluded_panes_are_the_ones_the_viewport_omits`, `render::tests::the_header_names_the_panes_that_are_only_overlays`, `state::tests::every_overlay_has_its_bound_key` |
| Hidden roster entries are counted | — unreachable at every declared viewport; see the caveats |
| Floor is refused at start-up | `tests::a_viewport_below_the_floor_is_refused_with_both_dimensions_and_code_two`, `layout::tests::the_floor_is_the_specified_one`, `layout-and-viewports.txt` |
| Floor mid-run suspends drawing only | `render::tests::below_the_floor_nothing_is_presented`, `verification::presentation_state_survives_every_resize` |
| State survives resize | `verification::presentation_state_survives_every_resize`, `render::tests::a_resize_changes_the_layout_and_nothing_else` |
| Layout is a pure function of dimensions | `verification::layout_reads_nothing_but_the_dimensions`, `layout::tests::every_region_stays_inside_the_viewport_and_the_body_rows_are_contiguous` |

## `REQ-MOK-025` — Preserve simulation outcome under observation

| Case | Test |
|---|---|
| Observed and unobserved streams are identical | `verification::observed_and_unobserved_runs_are_identical_on_every_declared_seed`, `non-perturbation.txt`, `export-fidelity.txt` |
| Interaction does not perturb | same case: the interaction script runs every tick of every seed's run |
| Entropy draw counts match per tick | `verification::per_tick_records_match_so_the_observer_draws_no_entropy` — see the caveats |
| Held state consumes nothing | `verification::holding_consumes_nothing_however_long_it_is_held` |
| Completed-tick boundary only | `verification::one_advance_is_one_tick_and_a_finished_run_refuses`, `verification::holding_consumes_nothing_however_long_it_is_held`, `render::tests::drawing_never_advances_the_simulation` |
| No catch-up | `verification::one_advance_is_one_tick_and_a_finished_run_refuses`, `tests::the_idle_wait_never_exceeds_the_nearest_deadline`, `resilience.txt` |
| Early exit yields a prefix | `verification::an_operator_ended_run_is_a_prefix_of_the_unobserved_run`, `tests::a_run_the_operator_ended_reports_itself_as_ended_early` |
| Wall clock reaches no authoritative value | `verification::no_frame_carries_an_environment_value`, `render::tests::the_footer_carries_the_provenance_and_nothing_environment_specific`, `export::tests::nothing_environment_specific_reaches_the_file` |
| Observer failure leaves the tick intact | `verification::an_injected_export_failure_leaves_the_tick_intact`, `render::tests::a_reported_failure_reaches_the_header` |
| Rendering purity | `verification::drawing_is_pure`, `render::tests::drawing_never_advances_the_simulation` |

## `REQ-MOK-026` — Keep the engine component independent of the observer

| Case | Evidence |
|---|---|
| Engine dependency set is empty | `dependency-review.txt`: `cargo tree -p Mokiterions` on every edge kind resolves to the package alone |
| No engine-to-observer edge | `dependency-review.txt`: the observer appears in no engine resolution |
| User-interface dependency is confined | `dependency-review.txt`: `ratatui` and its transitive crates appear only in the observer's resolution |
| Engine builds and tests without a terminal | `test-run.txt`: `cargo build -p Mokiterions` and `cargo test -p Mokiterions`, 60 tests |
| Surface exposes one mutating operation | **Does not hold as written.** `boundary-and-security-review.md` finds two, `run` and `advance_tick`; `SPEC-MOK-003` rule 2 is amended accordingly and the amendment is outstanding. A review of the public surface, not a test |
| Snapshots are owned and inert | `verification::drawing_is_pure`, `boundary-and-security-review.md` — see the caveats |
| Advance takes no operator data | `boundary-and-security-review.md`: `advance_tick(&mut self)` takes no argument |
| Resolved dependency graph and feature set | `dependency-review.txt`: `serde` absent, features exactly `crossterm`, `layout-cache`, `underline-color`, crate count measured |
| Two packages exactly | `static-checks.txt`: `cargo tree --workspace --depth 0` |

## `REQ-MOK-027` — Display run provenance and the authority for each event type

| Case | Test |
|---|---|
| Footer provenance fields | `render::tests::the_footer_carries_the_provenance_and_nothing_environment_specific`, `render::tests::the_footer_survives_the_narrowest_viewport` |
| Defaulted and explicit values present identically | `render::tests::the_footer_carries_the_provenance_and_nothing_environment_specific`, `options::tests::defaults_match_the_specified_values` |
| Commit field is compile-time or absent | `render::tests::the_footer_carries_the_provenance_and_nothing_environment_specific` — the absent case only; see the caveats |
| Footer carries no environment values | `verification::no_frame_carries_an_environment_value` |
| Authority mapping is exhaustive | `authority::tests::every_event_type_the_observer_can_present_has_an_entry`, `verification::the_declared_sets_are_the_contracts`, `render::tests::the_authority_overlay_names_identifiers_for_every_event_type` |
| Source-dependent mapping | `authority::tests::the_decision_source_maps_by_the_source_the_record_names` |
| A missing mapping is stated, not guessed | `authority::tests::an_ordinary_record_resolves_from_its_own_payload` |
| Mapping names identifiers only | `authority::tests::the_mapping_is_the_specified_one` |

## Caveats a reviewer should read before trusting the table

1. **`REQ-MOK-024`'s hidden-roster-entry case is unreachable at every declared viewport, and
   reachable off it.** The roster pane is 26 rows at the tightest declared viewport that places
   one, 120 × 30 and 100 × 30, so its interior holds exactly 24 rows against twelve two-line
   entries, and every other declared viewport gives it more; the roster overlay at 34 × 22 holds
   sixteen one-line rows against twelve entries. Nothing is hidden anywhere in the declared set,
   so the branch that states the hidden count is reached by no test.

   `SPEC-MOK-003` rule 5's 2026-08-19 amendment changed what that means. The superseded table
   placed a roster only where the viewport also had the rows for a log, so a roster was always at
   least 34 rows; the amended rule places it on width alone, so a 100 × 22 terminal — legal, above
   the floor, and outside the declared set — now shows eight entries and reports four hidden.
   `layout-and-viewports.txt` measures the capacity at every declared viewport and at the short
   ones the amendment admits, and carries the pane dumps at 100 × 22 and 100 × 26 that show the
   count in the pane's title. That is evidence the branch behaves, not an assertion that it does:
   no test reads the count, because no declared viewport reaches it. Twelve is the fixed population
   `SPEC-MOK-001` sets, so the branch also exists for a population the engine cannot currently
   produce, and the honest statement remains that it is unasserted rather than covered.
2. **`REQ-MOK-021`'s rejection cases are reached through a test-only hook.** Neither shipped
   decision source can have a proposal rejected — `verification::no_shipped_decision_source_has_a_proposal_rejected`
   asserts that as a fact over 400 ticks of both policies — so acceptance scenario 2 describes an
   unreachable state, and `replace_decisions_for_test` is the only way to reach it. The hook is
   `#[cfg(test)]` and is not in the shipped binary. This is a finding about the contract, recorded
   in the completion summary, not a shortcut in the evidence.
3. **`REQ-MOK-019`'s no-standing-resources case is reached the same way.** Rule 15 makes
   regeneration conditional on one remaining resource, and neither source consumes fast enough to
   empty a territory before the population dies, so `replace_snapshot_for_test` reaches it. The
   no-living-Mokiterions half of the same case is reached through a real run.
4. **`REQ-MOK-025`'s entropy case compares records, not a counter.** The specified observation
   surface exposes no draw counter and adding one would be public surface no artifact specifies, so
   the comparison is per-tick identity of every entropy-bearing record. The engine's entropy is one
   sequential stream, so a single extra draw would shift every later value; per-tick identity across
   a whole run therefore cannot hold if the counts differed. This is the observable form of the
   claim, argued in `non-perturbation.txt`, not the claim itself.
5. **`REQ-MOK-020`'s bar arithmetic is asserted at three values, not swept.** The exact rendered
   line is asserted for health 100, satiety 81 and energy 72, and separately for 0, which pins
   `round(value / 5)` at four points including both ends. No test sweeps 0 through 100.
6. **`REQ-MOK-027`'s commit field is verified absent, not present.** `COMMIT` is `option_env!`, so
   the present case needs a build with the variable set, and this run had none. What is verified is
   that no repository file is read and no version-control command is invoked to obtain it — which
   is the part of the case that could go wrong at run time.
7. **`REQ-MOK-026`'s surface cases are a review, not a test.** That the public surface has exactly
   one `&mut self` operation is a statement about the surface as a whole, which a test can only
   sample. It is discharged in `boundary-and-security-review.md` by enumerating the surface. The
   nearest measured corroboration is `verification::drawing_is_pure`, which holds a cloned snapshot
   across every draw at every viewport and requires it unchanged; no test holds a clone across an
   `advance` and re-asserts it, because `WorldSnapshot` owns its data and shares nothing with the
   simulation, which is what makes that case a type-level fact rather than a behavioral one.
8. **Every automated case is a claim about a character buffer.** No case in this table was verified
   by looking at a terminal. The legibility, colour-independence and panic-recovery assessments that
   need a person are in `manual-assessment.md` and are outstanding.
