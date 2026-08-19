# Requirement-to-test mapping

`VER-MOK-007`'s matrix, properties and acceptance scenarios against the tests that carry them. Every named test
passes in `test-run.txt`. Internal-tier tests are in `mokiterions-tui/src/render.rs`; public-tier tests are in
`mokiterions-tui/tests/render.rs`.

Coverage is stated exactly. Where a scenario's own numbers are not asserted literally, that is said rather than
implied.

## Matrix

| Case | Tier | Test | What it asserts |
|---|---|---|---|
| `band-text-identity` | internal | `banding_changes_no_character_of_an_entry` | 21 widths × 101 values = 2121 gauge cases against the unbanded form re-derived inside the test, plus the whole bar row at `FULL_BAR` including indent and separators |
| | internal | `the_bar_row_reproduces_the_specified_form` | rule 4's mockup line byte for byte at twenty cells, unchanged from before clause 7 |
| `band-domain` | internal | `the_survival_bands_are_the_three_the_rule_fixes` | all 101 values in `0..=100` against the band table stated in the test |
| `band-boundaries` | internal | `the_survival_bands_are_the_three_the_rule_fixes` | `band(39)` red, `band(40)` orange, `band(79)` orange, `band(80)` green, each by its own literal |
| `zero-is-red-and-empty` | internal | `a_zero_value_is_a_zero_and_an_absent_value_is_a_dash` | `0` renders as `0` with an empty bar, stays distinct from `—`, and takes the low band |
| | internal | `the_survival_bands_are_the_three_the_rule_fixes` | `band(0)` is the low band, by literal |
| `three-bands-in-one-entry` | internal | `each_gauge_carries_its_own_band_and_nothing_else_carries_one` | one entry at 12/55/88 rendering three different band colours on one row |
| | public | `the_survival_bands_reach_the_frame_and_three_differ_in_one_entry` | the same in the terminal's own cells, on a run's roster: three pairwise-distinct gauge colours in one entry, and across all twelve entries colour equal ⟺ band equal |
| `no-retained-tick` | internal | `a_band_reads_only_the_value_it_is_given` | interleaved reads of the same value return the same bands, so no hidden previous-value state exists |
| | static | `no-retained-tick.txt` | one call site of `band`, its argument a parameter; no previous/delta/trend state in the observer's source; the pre-existing `latest_survival` disclosed and shown unreachable from the roster path |
| `selected-entry-composes` | public | `a_selected_entry_keeps_its_bands_under_reversed_video` | the same entry before and after selection: `REVERSED` absent then present, gauge colours **identical**, text identical |
| `reserved-slot-stays-empty` | internal | `each_gauge_carries_its_own_band_and_nothing_else_carries_one` | exactly six spans, the row ending at the third value, so the reserved slot has no character and therefore no colour |
| | internal | `the_bar_row_reproduces_the_specified_form` | `lines[1].trim_end() == lines[1]` and the row's exact cell count |
| `collapsed-form-unstyled` | internal | `the_collapsed_form_takes_no_band` | the one-line form's text unchanged and every span's foreground `None` |

## Property and invariant tests

| Property | Test | Cases |
|---|---|---|
| 1. Text identity | `banding_changes_no_character_of_an_entry` | 2121 gauge cases (asserted as `21 * 101`), plus one whole-entry case at twenty cells |
| 2. Total, disjoint banding | `the_survival_bands_are_the_three_the_rule_fixes` | 101 values, each compared against exactly one band from the table stated in the test |
| 3. Monotone banding | `the_survival_bands_are_the_three_the_rule_fixes` | 100 adjacent pairs, none improving as the value falls |
| 4. Style locality | `each_gauge_carries_its_own_band_and_nothing_else_carries_one` | indent and both separators carry no band; no span carries a modifier |
| | `the_survival_bands_reach_the_frame_and_three_differ_in_one_entry` | in the buffer, every cell outside the three gauges on all twelve bar rows reads `Color::Reset`; each gauge's cells share one foreground or the read fails |
| 5. Non-perturbation, unchanged | `verification::drawing_is_pure`, `drawing_never_advances_the_simulation` | pre-existing, unmodified, both passing |

## Acceptance scenarios

| Scenario | Covered by | Literal? |
|---|---|---|
| 1 — the specified entry, banded (100, 81, 72 at twenty cells) | `the_bar_row_reproduces_the_specified_form` | **Yes.** The mockup line is asserted byte for byte and the three bands are asserted as high, high, middle. This is the scenario's point: 72 in the middle band while its neighbours are high is what a band read from the row would get wrong. |
| 2 — a Mokiterion in trouble (44, 8, 91) | `each_gauge_carries_its_own_band_and_nothing_else_carries_one` at 12, 55, 88 | **No, and stated.** The obligation — three values in three bands, three differing styles, the numbers unchanged — is asserted with different numbers in the same shape. `band-domain` separately asserts that 44, 8 and 91 take the middle, low and high bands. The two together carry the scenario; its own three numbers appear in no assertion. |
| 3 — the roster at the reference viewport, twelve entries spanning three bands | `the_survival_bands_reach_the_frame_and_three_differ_in_one_entry`, and `frames.txt` | **Partly.** Twelve entries at 160×48 are asserted present with band-consistent colour throughout; the living count and the pane's text-identity against a pre-clause-7 capture are not asserted in this test. Text identity is carried by property 1 at the entry level and by the nine pre-existing viewport cases in `every_declared_viewport_renders_and_annotates_what_it_presents`, all unmodified and passing. |
| 4 — starvation (satiety 0) | `a_zero_value_is_a_zero_and_an_absent_value_is_a_dash` | **Yes** for the rendering and the band. The scenario's reference to `SPEC-MOK-001`'s five-health penalty is context, not an assertion of this contract; the engine's own tests own that rule and are unchanged. |

## Static and architecture checks

| Check | Evidence |
|---|---|
| 1. `cargo fmt --check` clean | `static-checks.txt` |
| 2. `cargo clippy --workspace --all-targets` no finding | `static-checks.txt` |
| 3. No file under `mokiterions-core/` differs from `master` | `core-and-manifests-untouched.txt`, empty diff |
| 4. `Cargo.toml` unchanged in both packages, and `Cargo.lock` | `core-and-manifests-untouched.txt`, empty diff |
| 5. No previous-tick field, delta or two-tick comparison in the roster path | `no-retained-tick.txt` |
| 6. Whole workspace passes; exactly two pre-existing tests change; before and after recorded verbatim | `test-run.txt` (179 passing), `test-run-before.txt` (172 passing), `changed-assertions.md` |

## Security, privacy, performance

`VER-MOK-007`'s security and privacy section asserts a property of the change rather than a test: the band is a
function of one `u8` already in a snapshot the observer already holds, and the change opens no input path, reads no
file and performs no network access. `core-and-manifests-untouched.txt` shows no new dependency. The pre-existing
`the_footer_carries_the_provenance_and_nothing_environment_specific` still asserts that no absolute path, credential
or environment value reaches a frame, and it is unmodified.

Performance check 1 is arithmetic, not a measurement: three comparisons and three styles per entry, at most
thirty-six comparisons per frame for twelve Mokiterions. Check 2 — every viewport from the floor upward without panic,
including the widths where `bar_width` returns zero — is carried by the unmodified
`every_declared_viewport_renders_and_annotates_what_it_presents`, `below_the_floor_nothing_is_presented`,
`a_bar_row_shrinks_to_its_pane_and_never_overflows_it` and `the_collapsed_form_takes_no_band`.

## Manual

Three manual assessments, all **OUTSTANDING** with no author. See `manual-assessment.md`.
