# WO-MOK-003 evidence: superseding requirement-to-test mapping

## Status of this mapping, and of the ones before it

This mapping **supersedes**, for the current commit, the location claims in:

- `docs/engineering/simulation/evidence/WO-MOK-001/requirement-mapping.md`, and
- `docs/engineering/simulation/evidence/WO-MOK-002/requirement-to-test-mapping.md`.

Both of those remain **valid for their own commits and are not edited**. `WO-MOK-002`'s mapping in
particular states that "tests live in `src/simulation.rs`, `src/cli.rs`, and `src/main.rs`", which was
true when it was written and is bound to commit `68163ac` by `VREC-MOK-002`. Editing a retained
verification record to match a later tree would destroy the thing that makes it a record. Neither
file was opened for writing during this work order.

What this mapping adds is the **new location** of every test, so that every case in `VER-MOK-001` and
`VER-MOK-002` remains traceable after relocation. It changes no verdict. `SPEC-MOK-002` rule 11
requires every case in both contracts to remain covered, and rule 12 requires relocated assertions to
be verbatim, so the coverage claim below is the pre-change coverage claim with an address added.

**Result: 52 tests, all passing. No case in either contract lost its discharging test. Nothing is
failing, skipped, or ignored.** The transcript is `test-run.txt`.

## Reading the location column

Fifteen tests moved. Every other test is in `src/simulation.rs`, unchanged and in place. To keep the
tables readable, a location is given only where it is not `src/simulation.rs`:

| Location | Tests |
| --- | --- |
| `tests/cli.rs` | `defaults_are_stable`, `options_work_in_any_order`, `both_policies_are_selectable_and_reference_is_the_default`, `duplicates_and_missing_values_are_rejected`, `density_is_accepted_in_the_specified_forms_and_rejected_otherwise` |
| `tests/process.rs` | `help_exits_successfully`, `invalid_configuration_exits_with_code_two`, `a_density_resolving_to_no_resources_exits_with_code_two_before_initialization`, `output_failure_exits_with_code_one` |
| `tests/density.rs` | `density_resolves_to_the_specified_resource_count`, `a_density_resolving_to_no_resources_is_rejected` |
| `tests/termination.rs` | `tick_limit_terminates_with_one_summary`, `a_long_configured_run_is_bounded_and_does_not_panic`, `a_long_run_is_bounded_under_either_source` |
| `tests/viability.rs` | `the_reference_source_sustains_the_population_at_every_declared_density` |

Note the two paths that changed *kind* rather than only file: the five CLI tests were in
`src/cli.rs`'s test module, and the four process tests were in `src/main.rs`'s test module. `src/main.rs`
now has no test module at all, because rule 3 leaves it a shim.

## VER-MOK-001 — minimum simulation foundation

| Requirement | Discharging tests | New location | Result |
| --- | --- | --- | --- |
| `REQ-MOK-001` | `initial_world_population_and_food_match_the_contract`; the invalid-configuration tests | invalid-configuration tests → `tests/cli.rs` and `tests/process.rs` | pass |
| `REQ-MOK-002` | `initial_world_population_and_food_match_the_contract`, `initialization_is_seeded_and_reproducible` | in place | pass |
| `REQ-MOK-003` | `survival_decay_saturates_and_death_is_final`, `extinction_takes_precedence_at_the_tick_limit` | in place | pass |
| `REQ-MOK-004` | `invalid_move_does_not_mutate_action_state`, `untrusted_decisions_are_validated_and_traced`, `eating_is_atomic_bounded_and_single_use`, `the_reference_source_cannot_mutate_authoritative_state` | in place | pass |
| `REQ-MOK-005` | the movement, sleep, eating, invalid-action, and baseline-run tests | in place | pass |
| `REQ-MOK-006` | `eating_is_atomic_bounded_and_single_use` | in place | pass |
| `REQ-MOK-007` | `food_regenerates_only_in_nonempty_nonfull_territories`, `food_regeneration_respects_capacity`, `regeneration_adds_only_what_remaining_capacity_allows` | in place | pass |
| `REQ-MOK-008` | `repeated_runs_are_byte_identical`, `action_tracing_is_optional_complete_and_observational`; the whole suite runs with no network and no credentials | in place | pass |
| `REQ-MOK-009` | `splitmix64_sequence_is_stable`, `initialization_is_seeded_and_reproducible`, `repeated_runs_are_byte_identical` | in place | pass |
| `REQ-MOK-010` | the initialization, crossing, consumption, survival, death, regeneration, termination, summary, and output-failure assertions across the suite | output-failure → `tests/process.rs`; termination and summary → `tests/termination.rs`; rest in place | pass |
| `REQ-MOK-011` | `tick_limit_terminates_with_one_summary`, `extinction_takes_precedence_at_the_tick_limit`, the exit-code tests | tick-limit → `tests/termination.rs`; exit codes → `tests/process.rs`; extinction in place | pass |
| `REQ-MOK-012` | `action_tracing_is_optional_complete_and_observational`, `untrusted_decisions_are_validated_and_traced` | in place | pass |

`REQ-MOK-011` is the one row split across tiers, and it is worth naming why rather than leaving it to
inference. The tick-limit case moved because it needs only `Simulation::new`, `Simulation::run`, and
the summary accessors. The extinction case did not move because it forces extinction inside one tick
by clearing the resource collection and writing agent health and satiety directly, which rule 6
forbids exposing. Both assertions are unchanged and both still discharge their case; the requirement
is covered at full strength across the two tiers. See `test-placement.md`.

### VER-MOK-001's other check categories

| Check category | Evidence | Result |
| --- | --- | --- |
| Acceptance scenarios 1–6 | `baseline/full/`, `baseline/manifest.txt`, `baseline/summaries.txt`, and the tests above | pass; every scenario's output is byte-identical to the pre-change capture |
| Property and invariant tests (nine listed) | unchanged and in place in `src/simulation.rs`; `a_long_run_is_bounded_under_either_source` → `tests/termination.rs` | pass |
| Static and architecture checks (`fmt`, `clippy`, `test`, `build`) | `static-checks.txt`, `test-run.txt` | all exit 0 |
| Dependency-graph and public-decision-interface review | `boundary-review.md`, `public-surface-inventory.md` | confirmed; see the amendment note below |
| Security and privacy checks | `boundary-review.md` | no credential, network, filesystem, environment, or wall-clock access introduced |
| Performance and resilience: a 10,000-tick run | `resilience-and-viability.md` | pass under both sources |
| Manual assessment of a 20-tick traced run | `baseline/full/short_seed42_reference_trace_on.txt` and `completion-summary.md` | pass; byte-identical to the pre-change run |

One row in that table interacts with this work order rather than merely being re-run.
VER-MOK-001's architecture check reads "one binary crate". The crate now builds as one package with a
library target and a thin binary target. That is why `ADR-MOK-002` required, and the technical owner
approved, an amendment to `ARCH-MOK-001` and an in-place amendment to `ADR-MOK-001` before any code
changed; `amendment-approvals.md` records the approvals and dates. The check as amended — one Cargo
package, exactly one library target and one binary target, empty dependency table — is satisfied and is
verified in `completion-summary.md`. The substance the original check protected, that there is no
second crate and no service, is intact.

## VER-MOK-002 — perception and population viability

### REQ-MOK-013 — bounded perception

| Verification case | Test | New location | Result |
| --- | --- | --- | --- |
| In-radius resource with class, direction, distance | `perception_reports_in_radius_food_with_class_direction_and_distance` | in place | pass |
| In-radius living Mokiterion, observer excluded | `perception_reports_living_neighbours_and_never_the_observer` | in place | pass |
| Out-of-radius resource and dead Mokiterion excluded | `perception_excludes_distant_resources_and_dead_neighbours` | in place | pass |
| Perception crosses the territory boundary | `perception_crosses_the_territory_boundary` | in place | pass |
| Order stable and independent of collection iteration | `perception_order_is_stable_and_independent_of_collection_order` | in place | pass |
| Consumes no entropy and mutates nothing | `building_an_observation_consumes_no_entropy_and_mutates_nothing` | in place | pass |

Also covering the radius contract, both in place: `the_radius_boundary_is_inclusive_and_exclusive_by_one_cell`
and `co_located_entities_are_reported_at_distance_zero_without_a_direction`.

Every perception test stayed internal, and that is the expected outcome rather than a shortfall:
perception is asserted by constructing an `Observation` and reading `PerceivedFood` and
`PerceivedMokiterion`, all three of which rule 6 prohibits exposing.

### REQ-MOK-014 — population viability at a stated density

| Verification case | Test or evidence | New location | Result |
| --- | --- | --- | --- |
| 1,000 ticks at `0.75%` on each declared seed, floor 8 | `the_reference_source_sustains_the_population_at_every_declared_density` | `tests/viability.rs` | **pass — 8, 11, 8, 9, 11** |
| Same runs report food consumption | same test, consumption assertion | `tests/viability.rs` | pass |
| Density curve including undeclared densities | `WO-MOK-002/density-curve.md`; `baseline/manifest.txt` and `summaries.txt` re-confirm `1.50%` | evidence | recorded, no obligation |
| Conditional regeneration preserved at any density | `food_regenerates_only_in_nonempty_nonfull_territories` | in place | pass |
| Density resolves to `0.15%`→12, `0.75%`→61, `1.50%`→122 | `density_resolves_to_the_specified_resource_count` | `tests/density.rs` | pass |
| Density binds initialization, capacity, replenishment target | `density_binds_initialization_capacity_and_the_replenishment_target` | in place | pass |
| Zero-resource density rejected, exit code 2 | `a_density_resolving_to_no_resources_is_rejected`; `a_density_resolving_to_no_resources_exits_with_code_two_before_initialization` | `tests/density.rs`; `tests/process.rs` | pass |
| Scarcity assessment at the default density, manual | `WO-MOK-002/manual-observation.md`; re-confirmed by `summaries.txt` | evidence | pass, no seed retains twelve |

The survivor counts `8, 11, 8, 9, 11` are the counts `VREC-MOK-002` recorded. They are reproduced here
from `viability.txt`, which is byte-identical between the pre-change and post-change captures. The
floor of eight has no margin and is met exactly on two seeds, as `VER-MOK-002` records — which is
precisely why byte-identity rather than "still passes" is the standard this work order was held to.

### REQ-MOK-015 — reference decision source

| Verification case | Test | New location | Result |
| --- | --- | --- | --- |
| Moves toward a perceived resource, then eats it | `the_reference_source_approaches_then_consumes_a_perceived_resource` | in place | pass |
| Does not consume wastefully | `the_reference_source_does_not_consume_a_resource_it_does_not_need` | in place | pass |
| Does not approach a resource it would decline | `the_reference_source_does_not_approach_a_resource_it_would_decline` | in place | pass |
| Searches rather than waiting when it perceives nothing | `the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing` | in place | pass |
| Sleeps below the threshold, before approach and search | `the_reference_source_sustains_itself_before_seeking_or_searching` | in place | pass |
| Cannot mutate authoritative state | `the_reference_source_cannot_mutate_authoritative_state` | in place | pass |
| Both sources selectable, selection reported | `both_sources_run_are_reported_and_are_byte_identically_reproducible`; `both_policies_are_selectable_and_reference_is_the_default` | in place; `tests/cli.rs` | pass |
| Deterministic | same, plus `repeated_runs_are_byte_identical` | in place | pass |

Also covering the specified ranking rules, both in place:
`the_reference_source_prefers_the_nearest_then_richest_resource` and
`the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach`.

`the_reference_source_cannot_mutate_authoritative_state` is the test that verifies the trust boundary,
and it stayed internal necessarily: relocating it would have required exposing the state it exists to
prove is unexposed. That is recorded in `boundary-review.md` as well, because it is the clearest case
of rule 7 and rule 6 agreeing.

### Property and invariant tests

| Invariant | Test | New location | Result |
| --- | --- | --- | --- |
| Determinism, both sources, both swept densities | `both_sources_run_are_reported_and_are_byte_identically_reproducible`; `baseline-comparison.md` | in place | pass |
| Density invariants: starts at, never exceeds, replenishes toward | `density_binds_initialization_capacity_and_the_replenishment_target`, `regeneration_adds_only_what_remaining_capacity_allows`, `food_regeneration_respects_capacity` | in place | pass |
| Initial state matches the contract at the resolved density | `initial_world_population_and_food_match_the_contract` | in place | pass |
| Attribute bounds within `0..=100` | `attributes_stay_within_bounds_across_a_long_reference_run`, `survival_decay_saturates_and_death_is_final` | in place | pass |
| Perception symmetry | `perception_is_symmetric_between_living_mokiterions` | in place | pass |
| Perception purity | `building_an_observation_consumes_no_entropy_and_mutates_nothing` | in place | pass |
| No action at a distance | `perception_grants_no_ability_to_act_at_a_distance` | in place | pass |
| Death finality | `survival_decay_saturates_and_death_is_final` | in place | pass |
| Reference liveness: never waits, energy never zero | `the_reference_source_never_waits_and_never_runs_its_energy_to_zero` | in place | pass |
| Entropy attribution: only search draws | `the_reference_source_prefers_the_nearest_then_richest_resource`, `the_reference_source_does_not_approach_a_resource_it_would_decline`, and the co-located and approach tests | in place | pass |
| Bounded long run under either source | `a_long_run_is_bounded_under_either_source`, `a_long_configured_run_is_bounded_and_does_not_panic` | `tests/termination.rs` | pass |
| Entropy stream stability | `splitmix64_sequence_is_stable`, `initialization_is_seeded_and_reproducible` | in place | pass |
| Untrusted proposals validated and traced | `untrusted_decisions_are_validated_and_traced`, `invalid_move_does_not_mutate_action_state` | in place | pass |
| Termination and single summary | `tick_limit_terminates_with_one_summary`, `extinction_takes_precedence_at_the_tick_limit` | `tests/termination.rs`; in place | pass |

### Command-line contract

| Check | Test | New location | Result |
| --- | --- | --- | --- |
| Defaults stable, reference is the default policy | `defaults_are_stable`, `both_policies_are_selectable_and_reference_is_the_default` | `tests/cli.rs` | pass |
| `--density` accepted forms and rejections | `density_is_accepted_in_the_specified_forms_and_rejected_otherwise` | `tests/cli.rs` | pass |
| Duplicate and missing option values rejected | `duplicates_and_missing_values_are_rejected` | `tests/cli.rs` | pass |
| Options accepted in any order | `options_work_in_any_order` | `tests/cli.rs` | pass |
| Invalid configuration exits 2, help exits 0, output failure exits 1 | `invalid_configuration_exits_with_code_two`, `help_exits_successfully`, `output_failure_exits_with_code_one` | `tests/process.rs` | pass |

This is the section the refactor moved wholesale, and it is the section whose evidence is strongest as
a result. These tests now reach the code the way an external consumer does — `use mokiterions::cli`,
`use mokiterions::execute` — so their passing is evidence about the published interface and not only
about the crate's interior.

### Static, security, and performance checks

| Check | Evidence | Result |
| --- | --- | --- |
| `cargo fmt --all -- --check` | `static-checks.txt` | no differences |
| `cargo clippy --all-targets --all-features -- -D warnings` | `static-checks.txt` | no findings, including the `non_snake_case` gate that dictated the library target's name |
| `cargo build`, `cargo test` | `static-checks.txt`, `test-run.txt` | build clean; 52 pass, 0 fail, 0 ignored |
| Dependencies remain empty | `Cargo.toml`; `boundary-review.md` | `[dependencies]` empty, no `[dev-dependencies]`, no build script, `Cargo.lock` unchanged |
| Trust boundary passes only copied or immutable values | `boundary-review.md` | confirmed |
| No network, credential, or filesystem access | `boundary-review.md` | confirmed |
| 10,000-tick resilience under each source | `resilience-and-viability.md` | no panic; survivors + deaths = 12 in both |
| Oscillation rate against a random-walk baseline | `WO-MOK-002/manual-observation.md` | unchanged; the reference source's code was not modified and its output is byte-identical |
| End-of-run resource mix by class | `WO-MOK-002/calibration-record.md`; `baseline/summaries.txt` | unchanged; the per-class counts in all 40 summary lines are byte-identical |

The last two rows are discharged by equivalence rather than by re-measurement. Both are measurements
of simulation behavior, and `baseline-comparison.md` establishes that the observable output of all 43
captured cells is byte-for-byte what it was before the change. Re-deriving an oscillation rate from
identical event streams would produce an identical number by construction.

## Coverage summary

- Every row of `VER-MOK-001`'s requirement-to-evidence matrix has a named discharging test, at its new
  location where it moved.
- Every row of `VER-MOK-002`'s matrix, every listed property and invariant, and every command-line and
  static check likewise.
- 52 tests, all passing, in one `cargo test` invocation with no feature, no environment variable, and
  no `#[ignore]`.
- No verdict changed, no assertion weakened, and no case became evidence-only that was previously a
  test.

The one thing this work order changes about coverage is where two rule 8 *subjects* are covered from,
not whether they are covered: termination by extinction and the density-to-capacity binding are
asserted from the internal tier rather than the public tier. Both keep their original assertions.
`test-placement.md` records them, and neither is a `VER-MOK-001` or `VER-MOK-002` case left to a weaker
assertion, which is the condition that would have required stopping.
