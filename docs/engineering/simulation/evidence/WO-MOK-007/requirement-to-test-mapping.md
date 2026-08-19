# Requirement-to-test mapping — WO-MOK-007

One row per case in `VER-MOK-007`'s requirement-to-evidence matrix, in the contract's own order,
against the test or the retained file that discharges it. Test names are as `cargo test -- --list`
reports them. Engine tests are qualified: `simulation::tests::` is the engine's internal tier, and
`cli::`, `process::`, `termination::`, `viability::` and `decisions::` are its public tier. The
`mokiterions-tui` package's targets are unqualified, as they were in `WO-MOK-005`'s mapping, except
that its internal-tier tests keep their `render::tests::` and `verification::` module paths.

Where a case is discharged by something other than a test named for it, the note says so plainly.
The last section lists every such case in one place, because a case covered by a measurement, by an
aggregate test or by a structural argument is weaker evidence than a case with its own assertion,
and a reviewer should not have to find that out by reading the table.

**Two rows in the matrix are not satisfied.** They are the last two rows of the last table, and they
are the reason no verification record can be written against this commit. They are stated in place
rather than collected into a footnote.

## `REQ-MOK-031` — A fixed behavioral trait per Mokiterion

| Case | Test | Note |
|---|---|---|
| Two processes at one seed, on each declared seed | `simulation::tests::the_twelve_traits_are_the_recorded_ones_and_are_neither_uniform_nor_out_of_range`, `post/additivity.txt`, `measurements/traits.txt` | The recorded expectation is a checked-in table of sixty values, five seeds × twelve identifiers, written as literals rather than re-derived — which is what makes `negative-control/oracle-2.txt` able to fail. The two-*process* half is `post/additivity.txt`: twenty new-source cells captured twice by two OS processes, all twenty byte-identical. The test itself runs two `Simulation` values in one process |
| Trait values across the roster, on each declared seed | `simulation::tests::the_twelve_traits_are_the_recorded_ones_and_are_neither_uniform_nor_out_of_range`, `measurements/traits.txt` | Nine to eleven distinct values per seed, minimum `0`, maximum `40`, every value inside `0..=40` |
| Trait values across seeds for one identifier | `simulation::tests::the_trait_reads_the_seed_and_not_only_the_identifier` | `M01` differs between at least two declared seeds, so the derivation is not a function of the identifier alone |
| Shared stream state before and after derivation (oracle 2) | `simulation::tests::trait_derivation_leaves_the_shared_stream_where_it_found_it`, `negative-control/oracle-2.txt` | Recorded draw counts, not a before-and-after comparison around a derivation that takes no stream — see caveat 1 |
| Trait invariance across a run | `simulation::tests::the_trait_is_fixed_for_the_run_and_independent_of_every_configuration` | Each value at tick 1,000 equals its value at tick 0 |
| Trait independence from configuration | `simulation::tests::the_trait_is_fixed_for_the_run_and_independent_of_every_configuration` | The same twelve values across all three sources, the swept densities and the tick limits, in one test because the claim is one claim |
| Derivation source, statically | `interface-and-purity.txt` | Zero floating-point types and zero decimal literals in the engine's non-test source. The absence of time, environment, address and unordered-iteration input is a reading of `derive_waste_tolerance`, which is nine lines over `seed`, `number` and the engine's own generator — see caveat 2 |
| Initialization record | `process::the_trait_aware_source_runs_to_completion_and_reports_each_trait_once`, `measurements/traits.txt` | Twelve `agent_initialized` lines, twelve `waste_tolerance:` occurrences in the whole stream, and exactly one per subject, so the trait is stated once and nowhere else |
| Public interface | `interface-and-purity.txt` | Public items 49 → 49, fields 42 → 43, variants 47 → 48: the trait itself adds nothing — see caveat 3 |

## `REQ-MOK-032` — A `fear` attribute driven by perception

| Case | Test | Note |
|---|---|---|
| Perceiving at least one other living Mokiterion | `simulation::tests::fear_rises_at_the_perception_boundary_and_decays_one_cell_beyond_it`, `measurements/fear.txt` | 7,244 `+10` steps over 111,604 agent-ticks, and no other rising magnitude |
| Perceiving no other living Mokiterion | `simulation::tests::fear_rises_at_the_perception_boundary_and_decays_one_cell_beyond_it`, `measurements/fear.txt` | 13,940 `-5` steps and 90,201 holds; the holds are the lower bound and the ceiling — see caveat 4 |
| Perception boundary at Chebyshev `16` and at `17` | `simulation::tests::fear_rises_at_the_perception_boundary_and_decays_one_cell_beyond_it`, `measurements/fear.txt` | The constructed case is exact, and the measurement corroborates it on real runs: 245 to 427 agent-ticks at exactly `16` per seed, all rising; 238 to 448 at exactly `17`, all falling |
| Count, distance and direction insensitivity | `simulation::tests::fear_ignores_how_many_are_perceived_how_far_and_in_which_direction` | One perceived and four give the same increment; distance `1` and distance `16` give the same increment |
| Saturation at both bounds over many consecutive ticks | `simulation::tests::fear_saturates_at_both_bounds_and_is_reported_every_tick`, `measurements/fear.txt` | Eleven increments then twenty-one decrements, asserted at both bounds, with the debug build's overflow check active. Zero range violations and zero out-of-vocabulary steps across every run |
| Initial value | `process::the_trait_aware_source_runs_to_completion_and_reports_each_trait_once` | Twelve occurrences of `,fear:0,waste_tolerance:` — the field pairing is what makes it an initialization assertion, since `waste_tolerance:` appears on no other line |
| A dead Mokiterion | `simulation::tests::a_dead_mokiterion_reports_no_fear_and_no_decision` | No `survival_changed`, no `action_trace` and no decision line after the death tick |
| Trace ordering against the survival record | `simulation::tests::the_trace_reports_the_fear_the_survival_record_then_changes` | The trace ends with the pre-update value and the record reports the transition, which is rule 7's order asserted rather than assumed |
| Reproducibility of the transition sequence | `simulation::tests::individual_runs_are_reported_and_byte_identically_reproducible`, `post/additivity.txt` | Byte-identical output is a stronger statement than an identical transition sequence and contains it |
| No new perception and no entropy, statically | `post/additivity.txt`, `simulation::tests::building_an_observation_consumes_no_entropy_and_mutates_nothing` | The `fear` update runs under all three sources, so if it drew from the shared stream or rebuilt an observation the frozen sources' streams would have moved. Forty-two frozen-source cells are byte-identical to the pre-change baseline — see caveat 5 |
| No consumer | `interface-and-purity.txt` | Twenty-four occurrences of the identifier, one mutation at `simulation.rs:1962`, and zero code occurrences inside `trait DecisionSource`, all three `fn decide` and `struct Observation` |
| Snapshot field (oracle 4) | `render::the_roster_presents_four_gauges_at_every_declared_viewport_that_presents_it`, `verification::every_presented_value_is_the_snapshots` | The four presented gauge values are compared against the snapshot's own `health`, `satiety`, `energy` and `fear` as multisets, at every roster-drawing viewport, after thirty ticks so the values are computed ones — see caveat 6 |
| Roster renders four bars (oracle 4) | `render::the_roster_presents_four_gauges_at_every_declared_viewport_that_presents_it`, `render::tests::the_bar_row_reproduces_the_specified_form`, `render::tests::a_zero_value_is_a_zero_and_an_absent_value_is_a_dash`, `observer/roster-frames.txt` | Four gauges of equal width, each at least one cell, at absolute columns `gauges[0].column + index * (bar + 8)`. The zero case is the exact rendered line |
| The reserved slot is no longer empty (oracle 4) | `render::the_fourth_gauge_is_a_proportional_bar_at_zero_and_away_from_it`, `observer/roster-frames.txt` | `VREC-MOK-005` finding 3 is closed by cell position: the `f` gauge is found at columns 36, 38–39 and 41–43 at all four roster-drawing viewports, never blank, never a dash, never zero-width — see caveat 7 |

## `REQ-MOK-033` — A trait-aware decision source, selectable

| Case | Test | Note |
|---|---|---|
| Option acceptance | `cli::the_trait_aware_policy_is_selectable_and_does_not_become_the_default`, `process::invalid_configuration_exits_with_code_two`, `process::the_diagnostic_path_appends_the_whole_usage_text` | The name is exact: `individuals`, `Individual` and `indiv` are all rejected, and a repeated `--policy` is rejected |
| Usage text | `cli::the_entries_state_the_constraints_that_decide_validity`, `cli::each_declared_default_is_stated_once`, `cli::the_documented_options_are_exactly_the_options_the_parser_accepts`, `options::the_usage_text_advertises_every_policy_the_engine_accepts` | The first asserts both directions: every value the help names parses, and a value it does not name is rejected. The last is the observer's own usage text, which advertised two values while accepting three until this work order — see caveat 8 |
| Selected-source record | `process::the_trait_aware_source_runs_to_completion_and_reports_each_trait_once`, `authority::the_decision_source_maps_by_the_source_the_record_names` | Exactly one `decision_source_selected result=source:individual` line, and the observer resolves that record's authority to `REQ-MOK-033` |
| Lower-bound equivalence over the enumerated situation set (oracle 3) | `simulation::tests::at_tolerance_zero_the_trait_aware_source_proposes_what_the_reference_source_proposes`, `measurements/equivalence.txt`, `negative-control/oracle-3.txt` | 2,808 situations, enumerated as 13 × 3 × 18 × 2 × 2 and counted three independent ways. Both the proposal and the stream position are compared, which is what the negative control shows to matter |
| Divergence from a trait difference alone | `simulation::tests::a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten` | Two Mokiterions at one coordinate, one satiety, one resource, differing only in trait: one eats and one does not, and swapping the traits swaps the proposals |
| Divergence in a real run | `measurements/divergence.txt` | **A measurement, not a test, and the weakest row in this table.** Three to ten divergent situations per thousand-tick run, and zero cases of two Mokiterions facing the same situation on the same tick — see caveat 9 |
| The tolerant test governs eating *and* seeking | `simulation::tests::the_tolerant_test_governs_seeking_as_well_as_eating`, `measurements/oscillation.txt` | A Mokiterion whose tolerance declines a co-located resource does not target it on the next tick, so rule 5's second recorded defect does not reappear |
| Oscillation rate under the new source | `measurements/oscillation.txt` | Pooled 10.8% against rule 5's 10.6% and the 12.2% unbiased walk; per-seed 10.0% to 11.8% — see caveat 10 |
| Never proposes `wait` | `decisions::the_trait_aware_source_never_waits_and_proposes_only_valid_actions`, `measurements/proposals.txt` | Zero `proposal:wait` over 500 traced ticks, and zero across all ten measured runs |
| Determinism | `simulation::tests::individual_runs_are_reported_and_byte_identically_reproducible`, `post/additivity.txt` | Twenty cells captured twice by separate processes, byte-identical, and all twenty differing from the reference source's |
| Validation is not relaxed | `decisions::the_trait_aware_source_never_waits_and_proposes_only_valid_actions`, `verification::no_shipped_decision_source_has_a_proposal_rejected`, `simulation::tests::untrusted_decisions_are_validated_and_traced`, `measurements/proposals.txt` | Zero `status:rejected` under the new source, and rule 6's path is unchanged: the inherited test still shows an invalid proposal rejected without action-specific mutation. The observer's sweep was extended to the third policy rather than left naming a claim it no longer checked |
| The control is unchanged, projected byte comparison (oracle 1) | `post/additivity.txt`, `baseline/pre-manifest.txt`, `post/post-manifest.txt`, `baseline/exit-codes.txt`, `post/exit-codes.txt` | Forty-two frozen-source cells, zero differing bytes, exit codes identical — see caveat 11 |
| The projection used by oracle 1 | `baseline/projection.py`, `baseline/recapture-check.txt`, `manual-assessment.md` §6 | Three anchored patterns, quoted in full in the assessment; applied to the pre-change stream alone it is a no-op on all eleven retained cells |
| Public interface growth | `interface-and-purity.txt` | `+ Policy::Individual` and `+ AgentSnapshot.fear`, zero removals; the source type and the observation stay private |
| Tolerance comparison, statically | `interface-and-purity.txt` | Zero floats in code; the tolerant test is `S + R - 100 <= T * R / 100` in `u16` with truncating division |

## `REQ-MOK-034` — The population survives under the new source

| Case | Test | Note |
|---|---|---|
| 1,000 ticks at `0.75%` on each declared seed | `viability::the_trait_aware_source_sustains_the_population_at_every_declared_density`, `measurements/viability.txt` | 11, 9, 9, 10 and 12 survivors against a floor of 8 — after the trait range was narrowed to `0..=40`; see `escalation.md` |
| The same runs report consumption | `viability::the_trait_aware_source_sustains_the_population_at_every_declared_density`, `measurements/viability.txt` | 400, 368, 378, 379 and 417 consumption events |
| Traits actually spanned in the verified runs | `measurements/traits.txt`, `viability::the_trait_aware_source_sustains_the_population_at_every_declared_density` | Nine to eleven distinct values per seed, reaching `40`, so the floor was not met by a population that reproduces the reference source. The public test reads the traits out of the run's own `agent_initialized` lines rather than re-deriving them |
| Scarcity assessment | `manual-assessment.md` | **Manual, and outstanding.** The adverse condition the contract names — twelve survivors on every seed — did not occur: one seed of five retains twelve. The judgement is still the product owner's |

## All four requirements

| Case | Evidence | Note |
|---|---|---|
| Prior coverage preserved | `test-census.txt`, `static-checks.txt` | 169 → 190 tests over 19 → 20 runners, reconciled name by name; 21 additions, zero removals, zero ignored, zero filtered out. One runner is new and the one test that changed tier is named there |
| **Required amendments present and approved** (oracle 5) | `amendment-approvals.md` | **This row is not satisfied.** Every provision the owner approved on 2026-08-19 is present in both the amendment record and the specification text, checked over disjoint text. But three amendments were written during implementation beyond the owner's stated list — one taken as a decision under a stop condition, two **OUTSTANDING** — and the six amendments `VREC-MOK-005` left outstanding are still outstanding. The contract says absence fails it regardless of code state, and it does |
| The `VREC-MOK-005` gate | `README.md`, `WO-MOK-007` *Decision record*, `amendment-approvals.md` | **This row is not satisfied either.** `WO-MOK-005`'s six amendments and seven manual assessments were not resolved before implementation began; the repository owner overrode the gate on 2026-08-19. The mitigation is checked rather than asserted: every amendment row dated before 2026-08-19 is byte-identical to **60fda9f** |

## Caveats a reviewer should read before trusting the table

1. **Oracle 2 is not a before-and-after comparison.** Trait derivation constructs a generator of its
   own and drops it, so the shared stream is not passed to it and a comparison around it is
   tautological. What carries the check is the recorded draw count: `72` for twelve Mokiterions at
   the declared density, written as a literal. `negative-control/oracle-2.txt` perturbs the
   derivation to draw from the shared stream and the count becomes `84` — twelve extra, one per
   agent. That control also records something a reviewer should know: a re-derived expectation would
   have accepted the perturbed trait table, and only the recorded row rejected it.
2. **`REQ-MOK-031`'s static derivation case is part measurement, part reading.** The
   floating-point half is counted by `interface-and-purity.txt`. That no time, environment, address
   or unordered-iteration value reaches the derivation is a reading of nine lines, corroborated by
   the fact that sixty trait values are asserted as literals and reproduce across processes,
   machines' clock positions and run order. It is not separately asserted.
3. **The public-interface rows read literally do not match the census, and are satisfied jointly.**
   `REQ-MOK-031`'s row says `SPEC-MOK-002` rule 5's enumeration grows by the `fear` field alone;
   `REQ-MOK-033`'s says exactly one option value is added. The census finds exactly those two
   additions and nothing else, so both rows hold together, but neither describes the whole change on
   its own. A reviewer checking one row against `interface-and-purity.txt` will see two additions
   where the row names one.
4. **`fear` sits at its ceiling on 39% of agent-ticks.** Of 111,604 agent-ticks, 48% to 62% carry a
   non-zero value and 39% carry exactly `100`. The step and range invariants hold everywhere, so
   nothing in the matrix fails; the observation is that the `+10`/`-5` pair saturates quickly in a
   world where perception is usually populated. Nothing reads `fear`, so no outcome can falsify the
   pair — which is why this is recorded as an observation and named in `manual-assessment.md`
   assessment 4 rather than passed over.
5. **The `fear`-update entropy case is argued from oracle 1, not asserted directly.** No test counts
   draws around the update. The argument is that the update runs under every source, so a draw or a
   rebuilt observation would move the frozen sources' streams, and forty-two frozen-source cells are
   byte-identical to the recorded pre-change baseline. That is the observable form of the claim. The
   nearest direct assertion is the inherited
   `simulation::tests::building_an_observation_consumes_no_entropy_and_mutates_nothing`, which
   covers the observation and not the update.
6. **Snapshot-to-record agreement is covered in two halves, not end to end.** The snapshot-to-frame
   half is asserted at every roster-drawing viewport against the snapshot's own values. The
   record-to-field half is
   `simulation::tests::fear_saturates_at_both_bounds_and_is_reported_every_tick`, which compares the
   field against the transitions the records report. No single test carries a value from the text
   record through the snapshot to a rendered cell.
7. **Only one bar width is reachable through `render::draw`.** The roster pane is 47 columns wide and
   rule 4's collapse threshold is also 47, so the drawn roster is always two-line at a 45-column
   interior and `bar_width(45) = min(20, (45 - 35) / 4) = 2` is the only width any declared viewport
   produces. `observer/roster-frames.txt` swept 134 viewport renders and found bar widths observed
   = `{2}` and forms = two-line only. The `min(20, …)` cap and the collapsed one-line form are
   carried by three named internal render tests that call the layout arithmetic directly. This is a
   reachability finding about `SPEC-MOK-003` rule 4, not a defect in the fourth gauge.
8. **The observer's usage text was wrong before this work order touched it.** It advertised
   `--policy <baseline|reference>` while forwarding whatever the engine accepts, so a third value
   would have been accepted and undocumented. `options::the_usage_text_advertises_every_policy_the_engine_accepts`
   is a new test that asserts the general property rather than the third value, so the gap cannot
   reopen when a fourth source is added.
9. **`REQ-MOK-033`'s real-run divergence case is the weakest row in this mapping.** The contract asks
   for at least one tick per declared seed at which two living Mokiterions *in comparable situations*
   proposed different actions attributable to their traits. There are 3 to 10 such situations per
   run, against a figure the contract itself names as failure within a factor of three, and **zero**
   cases of two Mokiterions facing the same situation on the same tick — so no divergence is ever
   visible side by side in one frame. `measurements/divergence.txt` also records a counterexample to
   the naive reading of the trait ordering at seed 0: a Mokiterion at tolerance `10` never accepted a
   resource that one at tolerance `6` did, because the situations they met were not the same. The 54
   to 97 waste-accepting eats per run are the same behavior counted without requiring a coincidence,
   but substituting that measure for the one the contract names is the product owner's call.
10. **The oscillation comparison reproduces `WO-MOK-002`'s rates and not its denominators.**
    `measurements/oscillation.txt` reproduces the recorded 1,097/10,339 and 174/1,427 counts exactly,
    but its own pooled denominators are larger by exactly 96 in both rows — one per Mokiterion-run
    of the eight runs involved, which is what a boundary convention off by one tick looks like. The
    rates agree to 0.1 percentage point either way. Separately, the seed-0 margin against the
    unbiased-walk rate is 0.008 percentage points, which is not a margin a reader should treat as
    established.
11. **Oracle 1 covers eleven cells with certainty and forty on an argument.** Eleven of the forty
    declared cells were captured before the first line of code changed. The other 31 were captured
    afterwards from a clean git worktree at the same commit, which is a recapture; oracle 1 forbids
    recapturing the baseline *to resolve a discrepancy*, and this was not that, but the distinction
    rests on the recapture being from the same world. `baseline/recapture-check.txt` compares the
    eleven cells the two captures share, byte for byte, and all eleven match. A reviewer who does not
    accept the argument should read oracle 1 as covering eleven cells.
12. **Every automated case here is a claim about text or about a character buffer.** No case was
    verified by looking at a terminal, and the judgements that need a person — including whether the
    fourth gauge is legible beside the other three, and whether `fear` at its ceiling on 39% of
    agent-ticks is the intended behavior — are in `manual-assessment.md`, where five of seven are
    outstanding.
