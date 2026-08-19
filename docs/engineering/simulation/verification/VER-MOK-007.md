+++
id = "VER-MOK-007"
type = "verification"
title = "Individuality verification: derived trait, fear attribute, trait-aware source, and the unchanged control"
status = "draft"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
verifies = ["REQ-MOK-031", "REQ-MOK-032", "REQ-MOK-033", "REQ-MOK-034"]
+++

# Verification Contract: Individuality verification

## Independence

This contract carries an unusual centre of gravity. Three of the four requirements add behavior, and adding behavior
is easy to verify: run it and look. The claim that decides whether this change is sound is the *fourth* kind of claim,
and it is a claim of absence — that `--policy baseline` and `--policy reference` produce exactly what they produced
before. A suite written by the same effort that made the change is the worst possible witness to that, because the
natural way to write it is to assert what the new code does, and code that quietly consumed one extra value from the
shared entropy stream would satisfy every such assertion while silently retiring `REQ-MOK-014`'s measured floor.

So the additivity claim is verified against a recorded artifact of the world before the change, and the remaining
claims are verified against oracles that are not the changed code's own opinion of itself.

Five independent oracles are used.

1. **A recorded pre-change baseline, compared under a stated projection.** The engine's complete standard output and
   exit code are captured *before* any code changes, at the commit the work begins from, across the declared matrix:
   the seeds `0`, `1`, `42`, `123` and `777`; `--policy baseline` and `--policy reference`; the default density
   `0.75%` and the swept densities `VER-MOK-002` declares; `--ticks 1000`; with and without `--trace-actions`.
   Afterwards the same matrix is captured again, and the two streams are compared **byte for byte after removing the
   fields this change adds** — and nothing else. The projection is a stated, reviewed transformation that deletes the
   `fear` field from `agent_initialized`, `survival_changed` and `action_trace` lines and the trait field from
   `agent_initialized` lines. Any other difference, anywhere in the stream, is a defect in this change.

   Comparing projected streams rather than summary counts is deliberate. A survivor count matching by coincidence is
   possible; forty thousand lines of event stream matching line for line while the entropy sequence moved is not.
   The projection is the one place this oracle could be subverted, so it is reviewed as evidence in its own right, its
   full text is retained, and it is verified to be a no-op on the pre-change stream — a projection that also deleted
   a position or a resource identifier would fail that check.

   The baseline is captured once. A discrepancy is never resolved by recapturing it.

2. **The shared entropy stream's own position.** An internal-tier test records the shared stream's state immediately
   before and immediately after trait derivation for every Mokiterion, and asserts they are equal. This is the direct
   form of `REQ-MOK-031`'s load-bearing constraint, and it can fail for a reason oracle 1 cannot always expose: a
   derivation that drew a value and then restored the stream's position would pass oracle 1 on every seed and still be
   the wrong design.

3. **Arithmetic equivalence at the trait's lower bound, over an enumerated situation set.** The trait-aware source and
   the reference source are given the identical observation and asked for a proposal, with the acting Mokiterion's
   trait at the range's lower bound, over an enumerated set of situations rather than a sampled one: every calorie
   class co-located and absent, at satiety values spanning the range in steps that straddle every clipping boundary
   the food table produces, with and without a perceived resource in each cardinal and diagonal direction, and with
   the search fallback reachable and unreachable. Every pair of proposals must be equal. This oracle does not read the
   trait-aware source's implementation; it compares two sources' outputs.

4. **An in-memory character buffer, cell by cell.** Every claim about the roster's fourth bar is asserted against the
   rendered buffer, never against a screenshot, a recording or a terminal, as `REQ-MOK-029` and `SPEC-MOK-004` require
   of every observer rendering claim. The assertions name cell positions and characters.

5. **The governance state of the artifacts this change amends.** The required `SPEC-MOK-001`, `SPEC-MOK-002` and
   `SPEC-MOK-003` amendments must be present and approved before this contract can be satisfied, and the amendments
   already outstanding under `VREC-MOK-005` must be resolved. Their absence fails this contract regardless of the
   state of the code, for the reason `VER-MOK-006` gives in the same position: an amendment nobody approved is not a
   specification.

The declared verification seed set is `0`, `1`, `42`, `123` and `777`, fixed by `VER-MOK-002` and reused unchanged
here so that this change's measurements and the control's are taken on the same worlds.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-031` | automated-test | Two processes initialize at one seed, on each declared seed | The twelve trait values are identical between processes, and identical to a recorded expectation checked into the suite |
| `REQ-MOK-031` | automated-test | Trait values across the roster, on each declared seed | Not all twelve equal on any declared seed; every value inside the specified range |
| `REQ-MOK-031` | automated-test | Trait values across seeds for one identifier | `M01`'s value differs between at least two declared seeds, so the derivation reads the seed and not only the identifier |
| `REQ-MOK-031` | automated-test | Shared stream state before and after derivation (oracle 2) | Equal, for every Mokiterion, at every declared seed and density |
| `REQ-MOK-031` | automated-test | Trait invariance across a run | Each Mokiterion's value at tick 1,000 equals its value at tick 0, under every source |
| `REQ-MOK-031` | automated-test | Trait independence from configuration | The twelve values at one seed are identical across all three sources, across the swept densities, and across tick limits |
| `REQ-MOK-031` | static-analysis | Derivation source | No floating-point type or operation appears; no time, environment, address or unordered-iteration input; standard library and the engine's own generator only |
| `REQ-MOK-031` | automated-test | Initialization record | Each `agent_initialized` line reports the trait exactly once, for all twelve, in the specified field form |
| `REQ-MOK-031` | static-analysis | Public interface | The trait adds no item to the engine's public interface; `SPEC-MOK-002` rule 5's enumeration grows by the `fear` field alone |
| `REQ-MOK-032` | automated-test | A living Mokiterion perceiving at least one other living Mokiterion | `survival_changed` reports `fear` rising by exactly the specified increment |
| `REQ-MOK-032` | automated-test | A living Mokiterion perceiving no other living Mokiterion | `survival_changed` reports `fear` falling by exactly the specified decrement, or holding at the lower bound |
| `REQ-MOK-032` | automated-test | Perception boundary, constructed at Chebyshev distance `16` and at `17` | `16` rises, `17` decays. The driver is perception's own boundary, so an off-by-one here is an off-by-one in perception and a finding against `SPEC-MOK-001` |
| `REQ-MOK-032` | automated-test | Count, distance and direction insensitivity | One perceived Mokiterion and four produce the same increment; distance `1` and distance `16` produce the same increment |
| `REQ-MOK-032` | automated-test | Saturation at both bounds over many consecutive ticks | The value never exceeds the upper bound and never falls below the lower bound; no wrap and no panic in a debug build |
| `REQ-MOK-032` | automated-test | Initial value | Every Mokiterion reports `fear` at the range's lower bound in `agent_initialized` |
| `REQ-MOK-032` | automated-test | A dead Mokiterion | No `survival_changed`, no `action_trace` and no decision line after the death tick; absent from the snapshot, as it already is |
| `REQ-MOK-032` | automated-test | Trace ordering against the survival record | For one tick, `action_trace` reports the pre-update value and `survival_changed` reports the transition, consistent with `SPEC-MOK-001` rule 7 placing the trace before decay |
| `REQ-MOK-032` | automated-test | Reproducibility | Two processes at one seed produce the identical sequence of `fear` transitions for all twelve, on each declared seed |
| `REQ-MOK-032` | static-analysis | No new perception and no entropy | The update reads the existing observation only; no additional world traversal, no distance recomputation over the grid, and no draw against the shared stream |
| `REQ-MOK-032` | static-analysis | No consumer | No decision source, validation rule, survival rule, regeneration rule or termination rule reads `fear`; the field has exactly one writer and no reader inside the engine |
| `REQ-MOK-032` | automated-test | Snapshot field (oracle 4) | `fear` present on every living Mokiterion's snapshot entry, equal to the value the survival record reported for that tick |
| `REQ-MOK-032` | automated-test | Roster renders four bars (oracle 4) | At every declared viewport, the bar row shows four gauges of equal width, each at least one cell; a computed `0` renders as `0` with an empty bar per `SPEC-MOK-003` rule 4.4 |
| `REQ-MOK-032` | automated-test | The reserved slot is no longer empty (oracle 4) | No declared viewport renders the fourth slot as blank, as a dash, or as zero-width; `VREC-MOK-005` finding 3 is closed by cell-position assertion, not by inspection |
| `REQ-MOK-033` | automated-test | Option acceptance | The third value is accepted; an unrecognized value still exits `2` with the usage text on standard error and runs no ticks |
| `REQ-MOK-033` | automated-test | Usage text | States the third value, its effect and the unchanged default, satisfying `REQ-MOK-018`; the default remains the reference source |
| `REQ-MOK-033` | automated-test | Selected-source record | The new source is reported exactly once before agent processing, per `SPEC-MOK-001` rule 1 |
| `REQ-MOK-033` | automated-test | Lower-bound equivalence over the enumerated situation set (oracle 3) | Every proposal pair equal, with the enumerated set and its size stated |
| `REQ-MOK-033` | automated-test | Divergence from a trait difference alone | Two Mokiterions with different traits and otherwise identical observations propose different actions in at least the constructed clipping case; two with equal traits propose identically |
| `REQ-MOK-033` | automated-test | Divergence in a real run, not only in a constructed state | At each declared seed at the default density, at least one tick exists at which two living Mokiterions in comparable situations proposed different actions attributable to their traits, with the tick and the pair recorded |
| `REQ-MOK-033` | automated-test | The tolerant test governs eating *and* seeking | A Mokiterion whose tolerance declines a co-located resource does not target that same resource on the following tick; the two-cell oscillation `SPEC-MOK-001` rule 5 records as its second defect does not reappear |
| `REQ-MOK-033` | automated-test | Oscillation rate under the new source | Measured over 1,000 ticks at each declared seed and reported against rule 5's recorded 10.6% residual and the 12.2% unbiased-walk rate; a rate above the walk rate is a finding |
| `REQ-MOK-033` | automated-test | Never proposes `wait` | No `wait` proposal in any run under the new source, matching the reference source's stated behavior |
| `REQ-MOK-033` | automated-test | Determinism | Two processes at one seed, density and tick limit produce byte-identical output under the new source, on each declared seed |
| `REQ-MOK-033` | automated-test | Validation is not relaxed | Every proposal is one the observation listed as valid; the engine's rule 6 validation path is unchanged and still rejects an invalid proposal without action-specific mutation |
| `REQ-MOK-033` | automated-test | **The control is unchanged**, projected byte comparison against the recorded baseline (oracle 1) | Zero differing bytes across the whole declared matrix under `--policy baseline` and `--policy reference`; exit codes identical |
| `REQ-MOK-033` | review | The projection used by oracle 1 | Deletes only the added fields; verified to be a no-op on the pre-change stream; full text retained as evidence |
| `REQ-MOK-033` | static-analysis | Public interface growth | Exactly one option value added; the trait-aware source and the observation remain private, preserving the `ADR-MOK-001` boundary under `SPEC-MOK-002` rule 6 |
| `REQ-MOK-033` | static-analysis | Tolerance comparison | Integer arithmetic only; no floating-point type or operation |
| `REQ-MOK-034` | automated-test | 1,000-tick run under the new source at `0.75%` on each declared seed | ≥ 8 living on every seed, with the per-seed count recorded |
| `REQ-MOK-034` | automated-test | The same runs report consumption | Consumption events > 0 on every seed |
| `REQ-MOK-034` | automated-test | Traits actually spanned in the verified runs | The twelve trait values per seed are recorded, and at least one seed's population includes values away from the lower bound, so the floor is not met by a population that reproduces the reference source |
| `REQ-MOK-034` | manual | Scarcity assessment | Not all declared seeds retain twelve survivors; twelve everywhere is an adverse observation requiring product review |
| all four | automated-test | Prior coverage preserved | Every case, invariant and check in `VER-MOK-001` through `VER-MOK-006` still maps to a passing test; the workspace census reconciles name by name against the predecessor commit, with additions accounted for and no removal |
| all four | static-analysis | **Required amendments present and approved** (oracle 5) | The `SPEC-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-003` amendments named in `WO-MOK-007` are approved, and the amendments left outstanding by `VREC-MOK-005` are resolved, before this change is verified. Absence fails this contract regardless of code state |

## Acceptance scenarios

1. A reviewer captures the declared matrix at the commit the work begins from, applies the stated projection to both
   the pre-change and post-change streams, and finds them byte-identical under `--policy baseline` and under
   `--policy reference` on every seed, at every declared density, with and without `--trace-actions`. The reviewer
   then applies the projection to the pre-change stream alone and confirms it changes nothing there.
2. A reviewer runs the same matrix under the new source and finds the streams differ from the reference source's, as
   they must, and finds each of them reproducible across two processes.
3. A reviewer records the shared stream's state either side of trait derivation and finds it unmoved, then deliberately
   perturbs the derivation to draw one value, confirms oracle 2 fails, and reverts. A check that cannot be made to
   fail has not been demonstrated to work.
4. A reviewer enumerates the situation set of oracle 3, sets every trait to the range's lower bound, and finds the
   trait-aware source's proposal equal to the reference source's in every one.
5. A reviewer constructs two Mokiterions at one coordinate, at one satiety, on one high-class resource, with different
   traits, and observes one propose `eat` and the other not — then swaps the two trait values and observes the
   proposals swap with them.
6. A reviewer places one Mokiterion at Chebyshev distance `16` from another and finds `fear` rising, moves it one cell
   further and finds `fear` decaying, because at `17` the other is no longer in the observation.
7. A reviewer drives a Mokiterion to the upper bound of `fear` and holds it there for a hundred ticks, and finds the
   value unchanged and the process free of overflow in a debug build.
8. A reviewer renders the roster at each declared viewport into an in-memory buffer and reads four gauges from the bar
   row, naming the cell positions of each label, each bar and each value, and confirms the fourth is neither blank nor
   zero-width anywhere.
9. A reviewer runs 1,000 ticks under the new source at the default density on all five declared seeds and finds at
   least eight living on each, and records the twelve trait values behind each result.
10. A reviewer searches the engine for a read of `fear` and finds exactly one writer and no reader.
11. A reviewer confirms every amendment this change requires is approved, and that no amendment left outstanding by
    `VREC-MOK-005` remains outstanding.

## Property and invariant tests

- **Trait determinism.** For every declared seed and every identifier, the derived value is stable across processes,
  across sources, across densities and across tick limits, and lies inside the specified range.
- **Trait immutability.** No tick, action, rejection or death changes a trait value.
- **Entropy neutrality.** The shared stream's state after initialization is a function of seed and density alone,
  independent of the selected source and identical to the pre-change build's. This is asserted as a property, not
  only observed through output.
- **`fear` range invariant.** For every Mokiterion on every tick of every run at every declared seed under all three
  sources, `fear` lies within the attribute range. Asserted over full runs, not sampled ticks.
- **`fear` step invariant.** Every reported transition is exactly the specified increment, exactly the specified
  decrement, or a saturating truncation of one of them at a bound. No other magnitude occurs.
- **`fear` correspondence.** For every living Mokiterion on every tick, the direction of the transition matches
  whether that Mokiterion's own observation for that tick listed any other living Mokiterion. This is the property
  that distinguishes a perception-driven attribute from a tick counter.
- **Lower-bound collapse.** Over the enumerated situation set, the trait-aware source at the trait's lower bound is
  proposal-identical to the reference source.
- **Monotone tolerance.** For one observation and one acting Mokiterion, if a resource is accepted at some tolerance
  it is accepted at every higher tolerance. A non-monotone tolerance would mean the comparison is not the intended
  one.
- **Validity closure.** Every proposal the new source returns appears in the observation's list of currently valid
  proposals.
- **Determinism of the whole engine.** `REQ-MOK-009`'s byte-identical reproducibility holds under the new source at
  every declared seed, exactly as it does under the other two.
- **Snapshot–record agreement.** For every tick and every living Mokiterion, the snapshot's four attribute values
  equal the values the text record reports for that tick.
- **Layout purity.** Bar width remains a pure function of viewport width, unchanged in kind by the fourth bar.

## Static and architecture checks

- `cargo fmt --all --check` clean.
- `cargo clippy --all-targets --all-features -- -D warnings` clean, with no `allow` attribute added and no lint
  suppressed to accommodate this change.
- `cargo test` at the workspace root runs every tier of both packages in one invocation, with no feature, environment
  variable, ignore attribute, extra command, terminal or working-directory dependence.
- The engine's dependency and dev-dependency tables are empty, with no exception, and `cargo tree -p Mokiterions`
  resolves to the engine package alone. `ARCH-MOK-001` as amended admits no exception, including a dependency shared
  with the observer.
- The observer's dependency set is one path dependency on `Mokiterions` and `ratatui` at the pinned version and
  feature set, with nothing added.
- No new package, no new target, no build script, and no change to any package, library or binary name.
- No floating-point type or operation appears in trait derivation, the tolerance comparison, or the `fear` update.
- The engine's public interface grows by exactly two items: the `fear` field on the observation snapshot's Mokiterion
  entry, and the third decision-source value. `SPEC-MOK-002` rule 5's enumeration is compared item for item.
- No public item yields a mutable borrow of, or a reference into, authoritative state, in any build configuration
  including test builds. `SPEC-MOK-002` rule 6 is re-checked because a new field was added to a public type.
- Test placement follows `SPEC-MOK-004`: a new test lives in the public tier only if it is writable through the
  library target's public interface with its assertions unchanged; no item is widened to `pub` to relocate a test.
  Every internal-tier test added here names the private item or hook it requires.
- No architecture amendment is required. No active architecture `addresses` any of `REQ-MOK-031` through
  `REQ-MOK-034`, and the technical owner confirms in review that `ARCH-MOK-001`'s component boundaries, prohibited
  patterns and dependency prohibition are satisfied unchanged.

## Security and privacy checks

- No new input, no new file, no new network access, no new environment dependence and no new credential path. The
  simulation remains a closed local computation over command-line arguments.
- No model-provider credential or other secret enters the repository, in code, in a test fixture, or in retained
  evidence.
- Retained evidence contains simulation output only. It carries no personal data, and the trait and `fear` values are
  derived properties of fictional entities.
- **The trait derivation is not a security primitive.** SplitMix64 is not a cryptographic generator and the seed is a
  public command-line argument, so trait values are trivially predictable by design. Nothing in this change may be
  read as relying on their unpredictability, and no future requirement should be built on one.
- The observation snapshot still carries values only, so a new field grants the observer no new authority.
  `ADR-MOK-001`'s trust boundary is unchanged: the trait-aware source proposes and the engine decides.

## Performance and resilience checks

- Per-tick work is unchanged in order. The `fear` update reads an observation the engine already built, so no
  additional traversal of the grid, the resource collection or the agent collection is introduced, and the check that
  none was added is a static one rather than a timing one.
- Trait derivation is a fixed twelve operations at initialization and does not scale with density.
- A 1,000-tick run under the new source at the default density completes within the bound `SPEC-MOK-001` states for
  the existing sources, on each declared seed.
- A 10,000-tick run under the new source at the default density completes without panic, without overflow in a debug
  build, and without unbounded growth in retained state.
- The observer's frame remains within the bound `SPEC-MOK-003` states. A fourth gauge adds bounded work per roster
  entry and the roster is at most twelve entries.
- No arithmetic in this change can panic in a debug build: every addition and subtraction on an attribute or a
  tolerance comparison is saturating or provably in range, asserted over full runs rather than argued.

## Manual assessments

Each of the following is an explicit judgement recorded by the accountable role. An unrecorded assessment is an
outstanding assessment, and this contract is not satisfied while any remains outstanding.

1. **Scarcity, by the product owner.** At the default density under the new source, not all declared seeds retain
   twelve survivors. Twelve everywhere satisfies `REQ-MOK-034`'s literal floor and contradicts `INT-MOK-002`'s
   scarcity principle, and must be reported as adverse rather than passed over.
2. **Individuality is meaningful, by the product owner.** Reading the recorded divergences, the trait produces
   behavior a reader would call different individuals rather than arithmetic noise. A change that satisfies every
   automated check while producing one visible divergence per thousand ticks has not delivered `INT-MOK-006`.
3. **The accumulation result, by the product owner.** The measured high-class share at tick 1,000 and the tick-10,000
   outcome under the new source are read against the reference source's 45 of 61 and its extinction at tick 9,154, and
   the owner records whether the result is an improvement, a regression or neither. No obligation is stated on it by
   `REQ-MOK-034`, and the judgement is recorded rather than inferred.
4. **`fear`'s constants, by the technical owner.** The increment and the decrement produce a value that moves on a
   timescale a reader can interpret — neither saturating in the first few ticks nor effectively constant across a run.
   This is a judgement because nothing consumes `fear`, so no outcome can falsify the constants. The owner also
   records the measured share of agent-ticks on which `fear` was non-zero: a value that stays at the lower bound
   across a real run would be the inert attribute `SPEC-MOK-003` rule 4.5 refused, arriving by a different route.
5. **Roster legibility at four bars, by the technical owner.** Filling the reserved slot narrows the bars. At the
   reference roster's 45-column interior, four gauges leave `(45 − 35) / 4 = 2` cells per bar where three left 6. The
   owner records that a 2-cell bar beside its 3-column numeric value is acceptable at that viewport, or requires the
   roster pane to be widened in `SPEC-MOK-003` rule 5 instead. Either way the decision is stated in the rule 4.5
   amendment, not settled in the implementation.
6. **The projection, by the assurance owner.** The transformation oracle 1 applies deletes only the fields this
   change adds, and the owner confirms it could not mask a change to a position, an identifier, an event kind, an
   ordering or an attribute value.
7. **The absence of a `fear` consumer, by the technical owner.** No engine rule reads the attribute, and the owner
   confirms that the value's only purpose in this scope is to be reported.

## Evidence retention

Retained under `docs/engineering/simulation/evidence/WO-MOK-007/`:

- the pre-change baseline capture of the declared matrix, captured before any code change, with the commit it was
  taken at recorded;
- the post-change capture of the same matrix, and the projected comparison result for every combination;
- the full text of the projection, and the result of applying it to the pre-change stream alone;
- the twelve derived trait values per declared seed, as the record of which populations the floor was measured on;
- per-seed 1,000-tick survivor counts and consumption totals under the new source;
- per-seed final resource count and class distribution per territory under the new source at tick 1,000, and whether
  either territory reached zero;
- the tick-10,000 result under the new source at the default density, alongside the reference source's recorded
  extinction at tick 9,154;
- the measured oscillation rate under the new source per declared seed, against rule 5's 10.6% and the 12.2%
  unbiased-walk rate;
- the recorded divergence instances behind the real-run divergence check, each naming the tick, the two Mokiterions,
  their traits and their differing proposals;
- rendered roster buffers at each declared viewport, as text, with the four gauges' cell positions stated;
- the enumerated situation set used by oracle 3, and its size;
- the workspace test census before and after, reconciled name by name;
- `cargo fmt`, `cargo clippy`, `cargo test` and `cargo tree -p Mokiterions` output;
- the seven manual assessments above, each with its accountable role and date;
- the amendment-approval check of oracle 5.

Evidence is retained in the repository, is reproducible from the recorded commands and commit, and contains no
secret.

## Residual uncertainty

- **`fear` cannot be falsified by outcome.** Nothing reads it, so no run can be wrong because the constants are
  wrong. This contract verifies that the value is bounded, reproducible, correctly directed and correctly reported;
  it cannot verify that the constants are good. The first requirement that consumes `fear` will be the first real
  test of them, and it should expect to amend them.
- **The floor is verified on five declared seeds.** Passing does not prove viability for every seed, and it proves
  nothing at any density but the default, for the reason `REQ-MOK-014` states: each density is a different world.
- **The floor is verified on one trait distribution per seed** — the one the derivation produces. This contract
  records the twelve values so a reader can see which populations were measured, but it does not sweep the tolerance
  range. A narrower or wider range is a different experiment and would need its own measurement.
- **Additivity is verified over the declared matrix, not over the input space.** A change that consumed extra entropy
  only at a density outside the sweep, or only past tick 1,000, would pass oracle 1. Oracle 2 is what reduces this
  risk, by asserting the neutrality directly rather than inferring it from output.
- **Oracle 1 depends on its projection.** A projection that deleted more than the added fields would hide a real
  change. Two mitigations are stated — the no-op check on the pre-change stream, and the assurance owner's review —
  and neither is a proof.
- The tick-10,000 measurement carries no obligation, in either direction. It is recorded because
  `SPEC-MOK-001` rule 5 and `VER-MOK-002` record the reference source's long-horizon behavior and a comparison is
  worth having, not because a target exists.
- The oscillation rate is measured under the new source but no threshold is stated as a requirement. A rate above the
  unbiased-walk rate is treated as a finding by this contract, which is an assurance judgement rather than an
  approved obligation.
- **This change closes `VREC-MOK-005` finding 3 but does not close that record.** The finding disappears because the
  fourth bar becomes real; the six outstanding amendments and the seven outstanding assessments that record names are
  a precondition of this work, not an output of it.
- No claim is made about the observer's appearance to a human eye. Every rendering claim here is a claim about
  characters in a buffer at stated positions, and legibility is manual assessment 5.
