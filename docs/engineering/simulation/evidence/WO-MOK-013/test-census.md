# Test census — WO-MOK-013

Measured 2026-08-20 on this branch against `ff3a155`, the commit this work order starts from. The
census is `test-census.txt` beside this file, produced by `analysis/test-census.py`; this document
reads it against `SPEC-MOK-004` rule 11's recorded figures and against `VER-MOK-013`'s contracted
cases.

**A count cannot distinguish a rename from a removal plus an addition, and this work order contains
exactly one rename.** So the census is taken by name, at both commits, per file.

## Method, and why it is trusted

`analysis/test-census.py` counts `#[test]` functions per source file statically, at `ff3a155` through
`git show` and in the working tree directly. The static figure is not taken on trust: it must agree
with `test-output.txt`, which is the executed `cargo test --workspace`, target by target. It does.

| Executed target | Files attributed to it | Static | Executed |
|---|---|---:|---:|
| engine `unittests src/lib.rs` | `src/lib.rs`, `src/cli.rs`, `src/simulation.rs` | 54 | 54 |
| engine `unittests src/main.rs` | `src/main.rs` | 0 | 0 |
| engine `tests/cli.rs` | itself | 13 | 13 |
| engine `tests/decisions.rs` | itself | 1 | 1 |
| engine `tests/density.rs` | itself | 2 | 2 |
| engine `tests/naming.rs` | itself | 3 | 3 |
| engine `tests/process.rs` | itself | 6 | 6 |
| engine `tests/termination.rs` | itself | 4 | 4 |
| engine `tests/viability.rs` | itself | 2 | 2 |
| observer `unittests src/lib.rs` | `render.rs` 20, `verification.rs` 9, `state.rs` 4 | 33 | 33 |
| observer `unittests src/main.rs` | `src/main.rs` | 8 | 8 |
| observer `tests/authority.rs` | itself | 4 | 4 |
| observer `tests/export.rs` | itself | 7 | 7 |
| observer `tests/layout.rs` | itself | 11 | 11 |
| observer `tests/options.rs` | itself | 8 | 8 |
| observer `tests/render.rs` | itself | 22 | 22 |
| observer `tests/spatial.rs` | itself | 7 | 7 |
| observer `tests/state.rs` | itself | 21 | 21 |
| observer `tests/verification.rs` | itself | 20 | 20 |
| **Workspace** | | **226** | **226** |

Both doc-test targets run 0, as they did before. No `#[test]` carries `#[ignore]`, which rule 11
forbids and the census checks rather than assumes.

## Reconciliation to rule 11

| | At `ff3a155` | Now | Rule 11 records |
|---|---:|---:|---|
| Engine | 85 | **85** | 85, "unchanged" |
| Observer | 127 | **141** | 141 |
| Workspace | 212 | **226** | 226 |
| Observer internal tier, rule 10 | 39 | **41** | 41 |
| Observer public tier, rule 9 | 88 | **100** | 100 |

`41 + 100 = 141` and `141 + 85 = 226`, which is the cross-check rule 11's paragraph names: the
workspace total is reproducible from rules 9 and 10 and not only from the paragraph that states it.

## The fourteen arrivals, arrival by arrival

Fifteen test names appear that did not exist at `ff3a155` and one ceases to exist, so the net is
fourteen — which is the figure rule 11 records.

| File | Test | Serves |
|---|---|---|
| `src/render.rs` | `a_ten_point_step_moves_the_fill_at_the_reference_viewport` | `REQ-MOK-047`, whole value range |
| `src/render.rs` | `every_bar_width_the_roster_can_produce_resolves_a_ten_point_step` | `REQ-MOK-047`, plane property |
| `tests/layout.rs` | `the_reference_roster_interior_holds_the_whole_population` | the reference interior is what the fit assumes |
| `tests/render.rs` | `every_living_mokiterion_has_an_entry_at_the_reference_viewport` | `REQ-MOK-020` preserved, not traded |
| `tests/render.rs` | `a_declining_mokiterion_shows_a_declining_bar` | acceptance scenario 1 |
| `tests/render.rs` | `the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport` | `REQ-MOK-048`, cold start and every viewport |
| `tests/render.rs` | `the_hint_is_present_after_two_hundred_ticks_in_both_run_states` | `REQ-MOK-048`, not timed |
| `tests/render.rs` | `the_hint_displaces_neither_the_announcement_nor_the_footer` | `REQ-MOK-048`, displaces no obligation |
| `tests/render.rs` | `the_announcement_states_the_axis_and_the_value_the_layout_decides_presence_from` | `REQ-MOK-049`, axis, value, key, and the floor |
| `tests/render.rs` | `the_announcement_is_emphasised_and_the_optional_segments_are_not` | `REQ-MOK-049`, emphasis |
| `tests/render.rs` | `the_announcement_and_the_hint_read_nothing_but_the_viewport` | layout-purity property |
| `tests/render.rs` | `the_announcement_appears_and_disappears_with_the_pane_it_names` | acceptance scenario 5 |
| `tests/render.rs` | `no_entry_is_lost_silently_at_any_viewport_presenting_the_roster` | no-entry-lost-silently invariant |
| `tests/verification.rs` | `the_announcement_and_the_hint_survive_the_loss_of_colour` | `REQ-MOK-049`, legible without colour |
| `tests/layout.rs` | `the_log_is_six_rows_wherever_it_is_present` | **not an arrival** — see below |

Twelve of the fourteen are in the public tier and two in the internal tier, and rule 10's paragraph
states why those two cannot move: both hold every attribute at a value in turn through the
`#[cfg(test)]` snapshot hook, which no test outside the crate can link.

## The one rename, and why it is not a loss

| At `ff3a155` | Now |
|---|---|
| `tests/layout.rs::the_log_is_ten_rows_only_where_both_thresholds_are_met` | `tests/layout.rs::the_log_is_six_rows_wherever_it_is_present` |

Decision 1 withdrew the ten-row log, so this test's subject ceased to exist and the work order
expected its removal to be reported. It was **kept and strengthened** instead: the two viewports that
carried ten rows are the two that now assert six, so the withdrawn growth is asserted absent rather
than left untested. `SPEC-MOK-004` rule 12 admits that as a rename with its assertion strengthened
and not as a loss, and rules 9 and 11 both record it. `tests/layout.rs` therefore reads 11 rather
than 12: it gains two names, loses one, and nets one.

**No test was lost.** The census names every departure, and the one it names is this rename's old
name.

## The engine is untouched

The engine's 85 is unmoved, file for file, and its 54-internal / 31-public split is unchanged. That
is what the work order's *Out of scope* requires, and it is measured three ways: this census by test
name, `interface.txt` declaration by declaration against `ff3a155`, and `engine-untouched.txt` as an
empty `git diff --stat` over `mokiterions-core/`.

## Coverage of `VER-MOK-013`

Fifteen automated cases, five acceptance scenarios and four properties. Every one is discharged, and
one case is discharged by a pre-existing test rather than an arrival:

| `VER-MOK-013` case | Discharged by |
|---|---|
| A ten-point step moves the fill, over the whole value range | `src/render.rs::a_ten_point_step_moves_the_fill_at_the_reference_viewport` |
| The gauge resolves at every viewport that presents a bar row | `src/render.rs::every_bar_width_the_roster_can_produce_resolves_a_ten_point_step` |
| The values are the engine's | `tests/render.rs::the_roster_presents_four_gauges_at_every_declared_viewport_that_presents_it` — **pre-existing**, and the row says so: "unchanged from `VER-MOK-005`'s obligation" |
| `REQ-MOK-020` is preserved, not traded | `tests/render.rs::every_living_mokiterion_has_an_entry_at_the_reference_viewport` |
| The reference interior is what the fit assumes | `tests/layout.rs::the_reference_roster_interior_holds_the_whole_population` |
| The overlay key is on screen from the first frame | `tests/render.rs::the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport` |
| It survives every viewport above the floor | the same case, which sweeps rule 5's derived table and the floor |
| It is not timed | `tests/render.rs::the_hint_is_present_after_two_hundred_ticks_in_both_run_states` |
| It displaces no obligation | `tests/render.rs::the_hint_displaces_neither_the_announcement_nor_the_footer` |
| The announcement names the axis and the value | `tests/render.rs::the_announcement_states_the_axis_and_the_value_the_layout_decides_presence_from` |
| The stated value is the layout's own | the same case, which reads every value from `layout` and fixes no literal |
| The overlay key is retained | the same case, which asserts the key beside the axis and the value |
| The announcement is emphasised | `tests/render.rs::the_announcement_is_emphasised_and_the_optional_segments_are_not` |
| Legible without colour | `tests/verification.rs::the_announcement_and_the_hint_survive_the_loss_of_colour` |
| Every excluded pane is announced | `tests/render.rs::the_announcement_states_the_axis_and_the_value_the_layout_decides_presence_from`, at the floor, where all three are due |
| Scenario 1, the declining Mokiterion | `tests/render.rs::a_declining_mokiterion_shows_a_declining_bar` |
| Scenario 2, the cold start | `tests/render.rs::the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport` |
| Scenarios 3 and 4, the narrowed terminal and the floor | `tests/verification.rs::the_announcement_and_the_hint_survive_the_loss_of_colour`, plus the two cases above at those viewports |
| Scenario 5, resize across a threshold | `tests/render.rs::the_announcement_appears_and_disappears_with_the_pane_it_names`, on one observer resized rather than two runs compared |
| Property, gauge resolution over the plane | `src/render.rs::every_bar_width_the_roster_can_produce_resolves_a_ten_point_step` |
| Invariant, no entry is lost silently | `tests/render.rs::no_entry_is_lost_silently_at_any_viewport_presenting_the_roster` |
| Property, layout purity | `tests/render.rs::the_announcement_and_the_hint_read_nothing_but_the_viewport` |
| Property, non-perturbation | **not an automated case** — `non-perturbation.txt` and `baseline-comparison.txt`, measured against `ff3a155` |

Three cases are discharged by a case that also discharges another, which is stated here rather than
padded into separate tests: an assertion that names the axis, the layout's own value and the key in
the same frame is one measurement, and splitting it would assert the same buffer three times.
