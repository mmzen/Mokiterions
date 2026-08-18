# Test placement: the tier of every one of the observer's 109 tests, and why

`INT-MOK-003` gives the rule. A test belongs to the **public tier**, in `tests/`, when it can be
written entirely through the package's public interface. It belongs to the **internal tier**, beside
the code in `src/`, when it cannot. `SPEC-MOK-004` rules 8 to 10 apply that rule to the observer.

The decision is not a judgement call here, because Rust decides it. A file under `tests/` is its own
crate: it links the library target compiled *without* `--cfg test`, so a private item is invisible to
it and a `#[cfg(test)]` item does not exist in the build at all. A test that names either one cannot
compile there, whatever anyone would prefer. `hooks-and-visibility.txt` sets that out in full.

  Placement summary

    package            internal tier (src/)   public tier (tests/)   total
    Mokiterions                          37                     23      60
    mokiterions-tui                      32                     77     109
    ------------------------------------------------------------------------
    workspace                            69                    100     169

  The engine's 60 tests were already tiered this way and none of them moved; `test-census.txt`
  reconciles them name by name. What this work order decided was the observer's 109, all of which
  were internal before it: 77 moved out, 32 stayed, and every one of the 32 is accounted for below.

## The internal tier: 32 tests, each with the private item or hook that keeps it there

Method: each test's own text, plus the text of every helper in its test module that it calls, is
searched for the crate's private declarations and its four `#[cfg(test)]` hooks. A private function
must be *called* (`name(` or `.name(`), not merely mentioned, so that `.count()` on an iterator is
not mistaken for the private `render::count` and `export::write_records` is not mistaken for the
private `Observer::export`. Every one of the 32 has at least one genuine reason; none is unexplained.

### `src/render.rs` — 12 tests

Test-module helpers: start, frame_of, rows, text_of, press

  - `the_territory_rule_marks_the_row_between_the_territories`
    private: BOUNDARY_GLYPH
    reached via `start`, `frame_of`, `rows`, `press`
  - `the_bar_row_reproduces_the_specified_form`
    private: BAR_ROW_OVERHEAD, FULL_BAR, count, entry_lines
  - `a_bar_row_shrinks_to_its_pane_and_never_overflows_it`
    private: BAR_ROW_OVERHEAD, FULL_BAR, bar_width
  - `a_zero_value_is_a_zero_and_an_absent_value_is_a_dash`
    private: ABSENT, entry_lines
  - `a_depleted_territory_is_stated_in_words_at_every_width`
    private: count, territory_line
  - `the_footer_survives_the_narrowest_viewport`
    private: count
    reached via `start`, `frame_of`, `rows`
  - `the_inspector_states_absence_rather_than_inventing_a_subject`
    hook: select_for_test
    reached via `start`, `frame_of`, `text_of`
  - `the_log_shows_the_newest_records_and_reports_an_empty_filter`
    hook: select_for_test
    reached via `start`, `frame_of`, `text_of`, `press`
  - `an_overlay_covers_the_body_and_leaves_the_header_and_the_footer`
    hook: set_overlay_for_test
    reached via `start`, `frame_of`, `rows`
  - `the_help_overlay_lists_every_bound_key`
    private: help_lines
  - `the_authority_overlay_names_identifiers_for_every_event_type`
    hook: set_overlay_for_test
    reached via `start`, `frame_of`, `text_of`
  - `a_resize_changes_the_layout_and_nothing_else`
    hook: select_for_test
    reached via `start`, `frame_of`, `press`

### `src/state.rs` — 4 tests

Test-module helpers: start, press, send

  - `the_log_cursor_scrolls_only_inside_the_log_overlay`
    private: scroll_log
    reached via `start`, `send`
  - `the_highlighted_record_is_the_newest_until_the_operator_scrolls`
    hook: set_overlay_for_test
    reached via `start`, `send`
  - `a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour`
    hook: select_for_test
    reached via `start`, `send`
  - `selection_clears_itself_when_no_living_mokiterion_remains`
    hook: select_for_test
    reached via `start`, `send`

### `src/main.rs` — 8 tests

Test-module helpers: prepared, code_of

  - `help_exits_successfully_on_standard_output`
    private: Launch, prepare
    reached via `prepared`, `code_of`
  - `an_invalid_input_is_refused_before_the_terminal_with_code_two`
    private: Launch, prepare
    reached via `prepared`, `code_of`
  - `a_viewport_below_the_floor_is_refused_with_both_dimensions_and_code_two`
    private: Launch, prepare
    reached via `prepared`, `code_of`
  - `an_export_path_is_not_touched_at_start_up`
    private: Launch, prepare
    reached via `prepared`, `code_of`
  - `the_tick_interval_is_a_thousand_milliseconds_over_the_speed`
    private: tick_interval
  - `a_cadence_that_has_never_run_is_due_and_one_just_run_is_not`
    private: FRAME_INTERVAL, due
  - `a_run_the_operator_ended_reports_itself_as_ended_early`
    private: Launch, prepare, report
    reached via `prepared`
  - `the_idle_wait_never_exceeds_the_nearest_deadline`
    private: INPUT_INTERVAL, Launch, idle_for, prepare, tick_interval
    reached via `prepared`

### `src/verification.rs` — 8 tests

Test-module helpers: observer_for, press, tap, observed_lines, frame, flatten, region, canvas_of, monochrome, symbols, contrived

  - `drawing_is_pure`
    hook: select_for_test
    reached via `observer_for`, `observed_lines`, `frame`, `flatten`
  - `presentation_state_survives_every_resize`
    hook: select_for_test
    reached via `observer_for`, `tap`, `frame`
  - `a_filter_changes_what_is_presented_and_nothing_else`
    hook: select_for_test
    reached via `observer_for`, `tap`, `observed_lines`
  - `every_presented_value_is_the_snapshots`
    hook: select_for_test
    reached via `observer_for`, `frame`, `flatten`
  - `the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault`
    hook: replace_decisions_for_test, select_for_test
    reached via `observer_for`, `frame`, `flatten`, `region`
  - `a_degenerate_world_still_draws_a_frame`
    hook: replace_snapshot_for_test
    reached via `observer_for`, `frame`, `flatten`, `region`
  - `overview_encodes_no_resource_class_and_detail_zoom_does`
    hook: replace_snapshot_for_test
    reached via `observer_for`, `tap`, `frame`, `flatten`, `region`, `canvas_of`, `contrived`
  - `every_distinction_survives_the_loss_of_colour`
    hook: replace_snapshot_for_test, select_for_test
    reached via `observer_for`, `tap`, `frame`, `canvas_of`, `monochrome`, `symbols`, `contrived`

### Why `src/main.rs`'s 8 tests are internal by necessity rather than by choice

A binary target exposes no linkable interface at all. Nothing declared in `main.rs` — `prepare`,
`Launch`, `tick_interval`, `due`, `idle_for`, `report`, `FRAME_INTERVAL`, `INPUT_INTERVAL` — can be
named by any test outside the file, so all eight of its tests are internal whatever they touch.
Four of them reach `prepare` and `Launch` only through the module's own helpers `prepared` and
`code_of`; the reasons column names the private items reached, including those reached that way.

Moving these eight would have meant moving start-up parsing, the cadence arithmetic and the closing
report out of the binary and into the library. That is a design change to the component boundary
`ARCH-MOK-002` fixes, not a test reorganisation, and this work order did not make it.

## The public tier: 77 tests, each with the public items it reaches

Method and its limits. For each relocated test, its own text plus the text of every helper it calls
is searched for the names in the observer's public interface — the 97 items of
`public-item-census.txt` together with their public fields and variants. The match is on the leaf
name, so it cannot tell `EventBuffer::len` from `Vec::len`; the list below is therefore an upper
bound on what each test reaches, and its purpose is the negative one — no name outside the public
interface appears anywhere in it.

The proof that these 77 are genuinely writable through the interface is not this list. It is that
they compile and pass as eight separate external crates: `cargo test -p mokiterions-tui` builds
`tests/authority.rs`, `tests/export.rs`, `tests/layout.rs`, `tests/options.rs`, `tests/render.rs`,
`tests/spatial.rs`, `tests/state.rs` and `tests/verification.rs` against `mokiterions_tui` as a
dependency, with no `--cfg test`, and all 77 pass. A test reaching one private item or one hook
would have failed to compile. `test-run.txt` records the run; `verbatim-comparison.txt` records that
their bodies are unchanged, so what passes is the same assertion that passed before the move.


### `tests/authority.rs` — 4 tests
  - `every_event_type_the_observer_can_present_has_an_entry`
    public items: for_type, len, table
  - `the_mapping_is_the_specified_one`
    public items: None, for_type
  - `the_decision_source_maps_by_the_source_the_record_names`
    public items: A, None, for_event, for_type, tick
  - `an_ordinary_record_resolves_from_its_own_payload`
    public items: for_event, health, tick

### `tests/export.rs` — 7 tests
  - `records_use_the_engines_own_line_format_in_authoritative_order`
    helpers: buffer, rendered
    public items: EventBuffer, energy, events, health, height, len, new, push, satiety, tick, truncated, width, write_records
  - `the_closing_line_states_the_count_and_the_truncation`
    helpers: buffer, rendered
    public items: EVENT_CAPACITY, EventBuffer, energy, events, health, height, new, push, satiety, tick, truncated, width, write_records
  - `the_same_records_always_produce_the_same_bytes`
    helpers: buffer, rendered
    public items: EventBuffer, energy, events, health, height, new, push, satiety, tick, width, write_records
  - `nothing_environment_specific_reaches_the_file`
    helpers: buffer, rendered
    public items: C, EventBuffer, contains, energy, events, health, height, new, push, satiety, tick, truncated, width, write_records
  - `the_default_path_is_relative_and_derived_from_the_run`
    public items: contains, default_path, events, log
  - `a_written_file_holds_exactly_the_rendered_records`
    helpers: buffer, rendered
    public items: EventBuffer, energy, events, health, height, log, new, push, satiety, tick, width, write_file, write_records
  - `an_unwritable_path_is_reported_and_leaves_nothing_behind`
    helpers: buffer
    public items: EventBuffer, energy, events, health, height, log, new, push, satiety, tick, width, write_file

### `tests/layout.rs` — 7 tests
  - `the_floor_is_the_specified_one`
    public items: below_floor
  - `tiers_match_the_specified_table_including_its_boundaries`
    public items: A, B, C, D, Tier, tier_for
  - `the_declared_viewports_yield_the_declared_canvases`
    helpers: viewport
    public items: A, B, C, D, Tier, canvas_cells, height, new, resolve, tier, view, viewport, width
  - `every_region_stays_inside_the_viewport_and_the_body_rows_are_contiguous`
    helpers: viewport
    public items: footer, header, height, inspector, log, new, overlay, resolve, roster, view, viewport, width
  - `tier_minimums_hold_wherever_the_tier_declares_one`
    helpers: viewport
    public items: A, B, height, new, resolve, view, viewport, width
  - `excluded_panes_are_the_ones_the_tier_omits`
    helpers: viewport
    public items: Inspector, Log, Pane, Roster, height, is_empty, new, overlay_only, resolve, viewport, width
  - `the_one_to_one_threshold_with_the_inspector_shown_is_157_columns`
    helpers: viewport
    public items: canvas_cells, height, inspector, new, resolve, view, viewport, width

### `tests/options.rs` — 7 tests
  - `defaults_match_the_specified_values`
    helpers: run
    public items: Help, None, Options, Run, Startup, config, export_path, parse, speed, start_paused
  - `tracing_is_always_on_and_cannot_be_turned_off`
    helpers: run
    public items: Help, Options, Run, Startup, config, parse
  - `simulation_inputs_keep_the_engine_parser_and_its_rejections`
    helpers: run
    public items: Help, Options, Run, Startup, config, parse
  - `observer_inputs_are_validated`
    helpers: run
    public items: Help, Options, Run, Startup, events, export_path, log, parse, speed, start_paused
  - `help_wins_over_every_other_input`
    public items: Help, Startup, parse, speed
  - `an_export_path_is_taken_verbatim_as_data`
    helpers: run
    public items: A, Help, Options, Run, Startup, events, export_path, log, parse
  - `speed_steps_are_clamped_at_both_ends`
    public items: faster, slower

### `tests/render.rs` — 8 tests
  - `every_declared_viewport_renders_and_annotates_what_it_presents`
    helpers: frame_of, rows, start, text_of
    public items: Help, Observer, Run, Startup, contains, draw, footer, height, new, parse, tick, width
  - `a_region_states_the_world_range_it_presents`
    helpers: frame_of, rows, start, text_of
    public items: Help, Observer, Run, Startup, contains, draw, height, new, parse, width
  - `below_the_floor_nothing_is_presented`
    helpers: frame_of, rows, start, text_of
    public items: Help, Observer, Run, Startup, draw, height, is_empty, is_finished, new, parse, snapshot, tick, viewport, width
  - `detail_zoom_places_every_visible_entity_at_its_mapped_cell`
    helpers: frame_of, press, start
    public items: A, Detail, Help, Observer, Run, Startup, Zoom, cell_of, contains, draw, handle_key, height, id, iter, new, parse, record_geometry, resource_glyph, snapshot, viewport, width, zoom
  - `the_header_names_the_panes_that_are_only_overlays`
    helpers: frame_of, rows, start
    public items: A, Help, Observer, Run, Startup, Tier, contains, draw, height, inspector, new, parse, roster, width
  - `the_footer_carries_the_provenance_and_nothing_environment_specific`
    helpers: frame_of, rows, start
    public items: A, C, Help, Observer, Run, Startup, contains, draw, events, footer, height, new, parse, tick, truncated, width
  - `a_reported_failure_reaches_the_header`
    helpers: frame_of, rows, start
    public items: Help, Observer, Run, Startup, contains, draw, header, height, new, parse, set_notice, width
  - `drawing_never_advances_the_simulation`
    helpers: frame_of, start
    public items: Help, Observer, Run, Startup, advance, draw, events, height, iter, new, parse, snapshot, tick, width

### `tests/spatial.rs` — 7 tests
  - `the_whole_world_needs_both_axes_and_never_width_alone`
    public items: Overview, Viewport, Zoom, height, is_whole_world, last_x, last_y, resolve, viewport, width
  - `territory_a_is_above_territory_b`
    public items: A, B, Overview, Viewport, Zoom, dot_of, resolve, viewport
  - `the_overview_dot_grid_is_one_dot_per_world_cell`
    public items: Overview, Viewport, Zoom, bounds, dot_of, len, new, resolve, viewport
  - `a_character_cell_covers_two_by_four_world_cells_in_overview_and_one_in_detail`
    public items: Detail, None, Overview, Viewport, Zoom, cell_of, resolve
  - `the_camera_is_clamped_so_the_region_never_leaves_the_world`
    public items: A, Detail, Overview, Viewport, Zoom, camera, camera_limit, last_x, last_y, origin_x, origin_y, resolve
  - `the_territory_rule_is_present_exactly_when_the_boundary_is_visible`
    public items: Detail, Overview, Viewport, Zoom, resolve, shows_territory_boundary
  - `glyphs_are_the_assigned_ones`
    public items: A, B, C, D, agent_glyph, id, resource_glyph

### `tests/state.rs` — 21 tests
  - `the_world_extent_matches_the_engine`
    helpers: start
    public items: Help, Observer, Run, Startup, WORLD_SIZE, events, height, iter, matches, new, parse, width
  - `initial_state_is_the_specified_one`
    helpers: start
    public items: Filter, Held, Help, None, Observer, Overlay, Overview, Progression, Run, Running, Startup, Zoom, camera, events, filter, follow, new, overlay, parse, progression, selection, snapshot, speed, tick, truncated, zoom
  - `initialization_events_are_retained_in_authoritative_order`
    helpers: start
    public items: Help, Observer, Run, Startup, events, filter, iter, new, parse, presented, tick
  - `a_single_step_is_accepted_only_while_held_and_advances_exactly_one_tick`
    helpers: press, send, start
    public items: Held, Help, KeyResponse, Observer, Progression, Run, Running, Startup, force_draw, handle_key, new, parse, progression, snapshot, tick
  - `a_finished_run_refuses_to_advance_and_stays_inspectable`
    helpers: press, send, start
    public items: Help, KeyResponse, Observer, Run, Startup, advance, events, handle_key, is_finished, len, new, parse, snapshot, termination_reason, tick
  - `speed_steps_through_the_fixed_ladder_and_clamps`
    helpers: press, send, start
    public items: Help, KeyResponse, Observer, Run, Startup, handle_key, new, parse, speed
  - `selection_cycles_in_roster_order_and_escape_clears_it`
    helpers: press, send, start
    public items: Help, KeyResponse, None, Observer, Run, Startup, handle_key, new, parse, selection
  - `escape_closes_an_overlay_before_it_clears_a_selection`
    helpers: press, send, start
    public items: Help, KeyResponse, None, Observer, Overlay, Run, Startup, handle_key, new, overlay, parse, selection
  - `every_overlay_has_its_bound_key`
    helpers: press, send, start
    public items: Authority, Help, Inspector, KeyResponse, Log, Observer, Overlay, Roster, Run, Startup, handle_key, new, overlay, parse
  - `panning_moves_one_world_cell_and_clamps_at_every_edge`
    helpers: press, send, start
    public items: Detail, Help, KeyResponse, Observer, Run, Startup, WORLD_SIZE, Zoom, camera, handle_key, new, parse, record_geometry, zoom
  - `a_whole_world_overview_cannot_be_panned_off_the_world`
    helpers: press, send, start
    public items: Help, KeyResponse, Observer, Run, Startup, camera, handle_key, new, parse, record_geometry
  - `following_centres_the_selection_and_clamps_identically`
    helpers: press, send, start
    public items: Help, KeyResponse, Observer, Run, Startup, apply_follow, camera, contains, follow, handle_key, new, parse, record_geometry, selected_agent, viewport
  - `the_type_filter_cycles_the_whole_vocabulary_then_returns_to_none`
    helpers: press, send, start
    public items: Filter, Help, KeyResponse, None, Observer, Run, Startup, Type, filter, handle_key, new, parse
  - `filtering_changes_presentation_only`
    helpers: press, send, start
    public items: Filter, Help, KeyResponse, None, Observer, Run, Startup, Subject, advance, events, filter, handle_key, is_empty, iter, len, new, parse, presented
  - `a_subject_filter_needs_a_selection`
    helpers: press, send, start
    public items: Filter, Help, KeyResponse, None, Observer, Run, Startup, filter, handle_key, new, parse
  - `an_unbound_key_changes_nothing`
    helpers: press, send, start
    public items: Help, KeyResponse, Observer, Run, Startup, camera, events, force_draw, handle_key, len, new, overlay, parse, quit, selection, snapshot, speed, tick
  - `a_key_release_is_not_a_press`
    helpers: press, start
    public items: Help, Observer, Run, Startup, handle_key, new, parse, snapshot, tick
  - `quit_is_the_only_key_that_asks_to_exit`
    helpers: press, send, start
    public items: Help, KeyResponse, Observer, Run, Startup, handle_key, new, parse, quit
  - `the_event_buffer_drops_the_oldest_record_and_says_so`
    public items: EVENT_CAPACITY, EventBuffer, iter, len, new, push, tick, truncated
  - `a_death_carries_the_tick_and_the_engine_computed_final_values`
    helpers: start
    public items: Help, Observer, Run, Startup, advance, death_of, deaths, energy, health, id, is_empty, is_finished, iter, new, parse, roster, satiety, snapshot, tick
  - `shared_cells_are_counted_at_the_rendered_granularity`
    helpers: press, send, start
    public items: Help, KeyResponse, Observer, Run, Startup, handle_key, new, parse, record_geometry, shared_cell_count

### `tests/verification.rs` — 16 tests
  - `observed_and_unobserved_runs_are_identical_on_every_declared_seed`
    helpers: frame, interact, observed_lines, observed_run, observer_for, press, summary_from, tap, unobserved
    public items: A, Observer, Run, Startup, advance, below_floor, config, deaths, draw, events, filter, handle_key, height, is_finished, iter, len, new, parse, snapshot, termination_reason, tick, width
  - `per_tick_records_match_so_the_observer_draws_no_entropy`
    helpers: by_tick, frame, interact, observed_lines, observed_run, observer_for, press, tap, unobserved
    public items: A, Observer, Run, Startup, advance, below_floor, config, contains, draw, events, filter, handle_key, height, is_finished, iter, len, new, parse, push, tick, width
  - `holding_consumes_nothing_however_long_it_is_held`
    helpers: by_tick, frame, interact, observed_lines, observer_for, press, tap, unobserved
    public items: Observer, Run, Startup, advance, below_floor, config, draw, events, handle_key, height, iter, len, new, parse, push, snapshot, tick, width
  - `an_operator_ended_run_is_a_prefix_of_the_unobserved_run`
    helpers: frame, interact, observed_lines, observer_for, press, tap, unobserved
    public items: Observer, Run, Startup, advance, below_floor, config, draw, ended_early, events, handle_key, height, is_finished, iter, len, mark_ended_early, new, parse, width
  - `one_advance_is_one_tick_and_a_finished_run_refuses`
    helpers: observer_for
    public items: Observer, Run, Startup, advance, is_finished, new, parse, snapshot, termination_reason, tick
  - `a_finished_run_stays_inspectable_and_exportable`
    helpers: flatten, frame, observer_for, press, region, tap
    public items: Observer, Run, Startup, advance, below_floor, contains, draw, events, handle_key, height, is_finished, len, new, parse, presented, snapshot, tick, width, write_records
  - `layout_reads_nothing_but_the_dimensions`
    helpers: interact, observer_for, press, tap
    public items: Observer, Run, Startup, advance, handle_key, height, iter, new, parse, resolve, snapshot, tick, width
  - `every_declared_viewport_has_its_derived_canvas_with_a_header_and_a_footer`
    helpers: flatten, frame, observer_for, region
    public items: A, B, Observer, Run, Startup, advance, below_floor, canvas_cells, contains, draw, footer, header, height, len, new, parse, resolve, view, width
  - `a_smaller_world_row_never_renders_below_a_larger_one`
    helpers: frame, observer_for, press, tap
    public items: None, Observer, Run, Startup, advance, below_floor, camera, cell_of, draw, handle_key, height, last_y, new, origin_x, origin_y, parse, viewport, width, zoom
  - `exports_are_reproducible_and_are_the_engines_own_records`
    helpers: frame, interact, observed_run, observer_for, press, tap, unobserved
    public items: A, Observer, Run, Startup, advance, below_floor, config, draw, events, handle_key, height, is_finished, len, new, parse, truncated, width, write_records
  - `the_applied_action_presented_is_always_the_engines`
    helpers: flatten, frame, observer_for, region
    public items: Observer, Run, Startup, advance, below_floor, contains, draw, height, id, iter, new, parse, presented, snapshot, width
  - `no_shipped_decision_source_has_a_proposal_rejected`
    helpers: observer_for
    public items: Observer, Run, Startup, advance, is_finished, new, parse, snapshot, tick
  - `no_frame_carries_an_environment_value`
    helpers: flatten, frame, interact, observer_for, press, region, tap
    public items: C, Observer, Run, Startup, advance, below_floor, contains, draw, handle_key, height, new, parse, width
  - `an_injected_export_failure_leaves_the_tick_intact`
    helpers: observed_lines, observer_for, press, tap
    public items: Observer, Run, Startup, advance, events, handle_key, iter, log, new, notice, parse, snapshot, tick
  - `the_declared_sets_are_the_contracts`
    public items: below_floor, contains, for_type, height, len, width
  - `a_death_removes_the_subject_from_the_presentation_and_is_corroborated`
    helpers: canvas_of, frame, observer_for, press, region, tap
    public items: Observer, Run, Startup, advance, agent_glyph, below_floor, contains, deaths, draw, filter, handle_key, height, id, is_finished, iter, len, new, parse, resolve, roster, snapshot, view, viewport, width, zoom

77 relocated tests, every name they reach belonging to the public interface.
