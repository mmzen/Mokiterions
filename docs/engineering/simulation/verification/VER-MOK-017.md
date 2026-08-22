+++
id = "VER-MOK-017"
type = "verification"
title = "Cumulative activity totals equal an independent count over the engine's own records, and computing them changes no run"
status = "draft"
owners = ["assurance owner"]
created = "2026-08-22"
updated = "2026-08-22"

[relations]
verifies = ["REQ-MOK-061", "REQ-MOK-062"]
+++

# Verification Contract: Cumulative activity totals

## Independence

Three things keep this contract independent of how the observer is written.

1. **Every total is checked against a count this contract's own tests compute**, by straight-line code over the
   engine's records, never by calling the accumulator that produced the figure. A test that asks the accumulator to
   confirm itself verifies nothing; the oracle is an independently written count and the comparison is between two
   paths to the same engine records.
2. **Where two engine records state the same fact, both are used.** The observer reads a verb from the decision
   record; this contract also counts the event the engine emits when that verb resolves. An accumulator error and a
   source error are then distinguishable, and neither can hide behind the other. This is the one design property that
   makes the totals checkable at all, and it exists because the engine emits `food_consumed`, `territory_crossed` and
   `attack_resolved` for facts the decision record states differently.
3. **The tests live in the tier `SPEC-MOK-004` rule 9 assigns**, exercising the observer through its stated public
   interface from outside the crate wherever that tier applies, so no oracle depends on a private item and no item is
   widened to be tested. Widening an item to reach it from a test is a prohibited pattern under `ARCH-MOK-002` and is
   a failure of this contract, not a means of satisfying it.

This contract is authored before the implementation exists and names no file, no type and no function of it.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-061` | automated test | **O1** independent count, per Mokiterion, every declared seed | each of the fourteen totals equals the independently computed count, for every Mokiterion, at the final tick and at three intermediate ticks |
| `REQ-MOK-061` | automated test | **O2** second-record cross-check | applied `eat` equals `food_consumed` events with that subject; applied `attack` plus applied `fight` equals `attack_resolved` events with that subject; crossings equals `territory_crossed` events with that subject |
| `REQ-MOK-061` | automated test | **O3** opportunity identity | for every Mokiterion at every completed tick, the eleven verb totals plus the rejected total equals the decision-opportunity count |
| `REQ-MOK-061` | automated test | **O4** killed total | the killed total equals the number of `attack_resolved` events whose subject is that Mokiterion and which report the target as died, and equals it for a run in which both `attack` and `fight` occur |
| `REQ-MOK-061` | automated test | **O5** frozen at death | for a Mokiterion that dies at tick `d`, every total at every tick after `d` equals its value at `d`, and no total is removed |
| `REQ-MOK-061` | automated test | **O6** zero is a measurement | a Mokiterion that never applied `attack` presents `0` for it, while `age` and per-agent entropy are absent from the pane entirely |
| `REQ-MOK-061` | automated test | **O7** flag independence | every total is identical with `--trace-actions` set and unset, for the same seed |
| `REQ-MOK-061` | automated test | **O8** truncation independence | for a run long enough that the event buffer reports `truncated`, every total still equals the independent count |
| `REQ-MOK-062` | automated test | **O9** population sum | every population total equals the sum of the per-Mokiterion totals, at the final tick and at three intermediate ticks, in a run with at least one death |
| `REQ-MOK-062` | automated test | **O10** dead members retained | the population totals never decrease from one tick to the next, on every declared seed |
| `REQ-MOK-062` | automated test | **O11** engine's own figures | the presented tick, living count and death count are the values the engine's snapshot reports, and the initialized count is the number of Mokiterions the engine's initialization records name |
| `REQ-MOK-062` | automated test | **O12** death split | deaths attributed to a strike equals the population killed total; the remainder equals the engine's death count minus that total and is never negative and never labelled with a cause |
| `REQ-MOK-062` | automated test | **O13** the clause it must not break | with nothing selected the pane states that nothing is selected and states the selecting control, and does so above the totals; no total is presented without that statement |
| `REQ-MOK-062` | automated test | **O14** extinction arrives here | when the last living Mokiterion dies the selection is cleared, this state is presented, and the living count presented is `0` |
| both | automated test | **O15** before tick 1 | in both selection states, no figure is presented, and the pane states that no tick has completed |
| both | automated test | **O16** nothing is clipped | in every pane state, the widest rendered line is no wider than the pane's interior at the reference viewport and at the inspector's presence threshold |
| both | automated test | **O17** run identity | the observed run's event sequence and final state are identical to the unobserved run's, and per-tick entropy draw counts are identical, on every declared seed |
| both | automated test | **O18** export unchanged | the rule 9.4 export for a given seed and configuration is byte-identical to its pre-change capture, and carries no total |
| both | static check | **O19** no aggregate crosses the boundary | see *Static and architecture checks* |
| both | static check | **O20** integers only | no floating-point type appears in the added observer code, and no rendered figure contains a decimal separator, a percent sign or a ratio |

## Acceptance scenarios

1. **A profile that distinguishes two policies.** Two runs of the same seed and tick count under two decision sources
   produce per-Mokiterion totals that differ, and the direction of the difference matches the sources' stated
   behaviour. This is the requirement's purpose and is checked as an outcome rather than assumed from the arithmetic.
2. **A selected Mokiterion, mid-run.** The current-tick decision record is unchanged from what `REQ-MOK-021` requires,
   and the totals appear beneath it without displacing or altering any line the pane presented before.
3. **A selected Mokiterion that dies of combat.** The death, the tick of death and the four final attributes are
   presented under rule 10 item 6 exactly as `WO-MOK-018` left them, together with frozen totals, and the killer's own
   killed total advances on that tick.
4. **Nothing selected, mid-run.** `Esc` from a selected Mokiterion reaches the population state, and a following
   `Tab` returns to a selected one with that Mokiterion's totals intact — no total is reset by traversing the states.
5. **A run that ends in extinction.** The final frame presents the population's completed totals without an operator
   act.
6. **A viewport that excludes the inspector.** At a width below the inspector's threshold, both pane states are
   reachable as the rule 5 overlay and present the same figures.

## Property and invariant tests

- **P1 — Additivity.** Every total is monotone non-decreasing in tick, for every Mokiterion and for the population.
  A total that falls is a defect whatever its value.
- **P2 — Conservation.** The population total for each figure equals the sum over Mokiterions at every tick, not only
  at the end.
- **P3 — Exhaustiveness over the contract.** Every one of the eleven action kinds has a total, checked by a case that
  fails if the action contract grows and a kind is not presented. The check reads the contract rather than a list
  copied from it.
- **P4 — Boundedness.** Retained state is one record per initialized Mokiterion and does not grow with ticks: the
  retained record count after a long run equals the initialized count.
- **P5 — Determinism of presentation.** Two frames of the same tick present identical figures, per *Data and interface
  contracts* clause 4.
- **P6 — No total is reachable from a decision.** See O19; stated as an invariant because it is the property
  `REQ-MOK-059` obliges and the one a later refactor is likeliest to break silently.

## Static and architecture checks

- **O19.1** `mokiterions-core` declares no dependency on the observer package, and its manifest is unchanged by this
  work order.
- **O19.2** No identifier of the retained totals type appears anywhere under `mokiterions-core/`.
- **O19.3** The engine's public interface is unchanged: the enumerated surface `SPEC-MOK-002` rule 5 records is
  identical before and after, and `grep -n 'pub fn .*&mut self' mokiterions-core/src/simulation.rs` returns exactly
  `run` and `advance_tick`.
- **O19.4** No `Observation` is constructed by the observer, and no `DecisionSource` implementation is reachable from
  it.
- **O20.1** No `f32` or `f64` appears in the observer code this work order adds.
- **Interface census.** The recorded figures in `SPEC-MOK-004` rules 6 and 9 match a fresh measurement at the
  candidate commit, and the commands that produced the measurement are retained.
- **Lint and format.** `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` is clean, and
  `cargo fmt --check` is clean.

## Security and privacy checks

- No total, and no frame, carries any environment value, any path outside the resolved export path, or any operator
  identity. The existing property that no frame carries an environment value continues to hold.
- The retained state is in-process only: nothing is written to disk by the accumulation, and the export is unchanged
  under O18.
- No credential, token or provider secret is introduced, read or retained. The evidence packet is inspected for
  incidental capture of any such value before it is committed.

## Performance and resilience checks

- **Per-tick cost.** Accumulation is bounded by the decision records and events of one tick and adds no per-tick work
  proportional to the run's length. Measured as wall-clock for a declared long run before and after, reported as a
  figure rather than as an assurance, with the measurement's own variance stated.
- **Memory.** Retained state is measured for a run at the declared population and reported, against P4's bound.
- **Overflow.** A total cannot wrap. Either the chosen width makes saturation unreachable for any admissible run,
  which is stated with the arithmetic, or saturation is demonstrated to be the behaviour at the limit.
- **The long run.** O8's truncating run is executed rather than reasoned about, because it is the case the design
  exists to survive and the one a shorter test suite would never reach.

## Manual assessments

Each is recorded with the assessing role, the date, and the evidence read. An unsigned assessment is not a recorded
one.

1. **Does the pane answer the question it was added for?** The assessor reads the two pane states for a completed run
   and states whether a Mokiterion's profile is legible from them without reference to the log. Assessed by the
   product owner.
2. **Can the population state be misread as one Mokiterion's?** Assessed by the product owner against captured
   frames, not against the specification text.
3. **Is the rule 10 item 7 amendment sound?** The assessor states whether removing `kills` and `combats` follows the
   2026-08-19 procedure, whether the suffered-attack ground is genuinely untouched, and whether the amendment states
   the zero-is-a-measurement distinction rather than assuming it. Assessed by the technical owner.
4. **Are the two corrections in `WO-MOK-020` §3 correct and correctly scoped?** Specifically whether the *State model*
   table can now be read as exhaustive, and whether rule 4's corrected sentence still forbids everything it forbade.
   Assessed by the technical owner.
5. **Is `REQ-MOK-059` still met?** The assessor states whether the boundary argument holds structurally and whether
   O17 and O19 together are sufficient evidence for it. Assessed by the assurance owner.
6. **Are the `SPEC-MOK-004` figures right?** The assessor recomputes at least one of the amended figures independently
   of the retained command output. Assessed by the assurance owner.
7. **Label wording.** Whether the labels are readable as the engine's own vocabulary and not as an interpretation of
   it. Assessed by the technical owner.

## Evidence retention

Retained under `docs/engineering/simulation/evidence/WO-MOK-020/` with a SHA-256 manifest covering every file, written
with the line endings the manifest hashes, on the `-text` attribute that tree carries.

- Full command transcripts: clippy, fmt, the whole test run, before and after.
- The independent-count comparison per declared seed, retained as the two counted tables and their diff, so a reviewer
  recomputes rather than reads a conclusion.
- Frame captures: selected living, selected dead, no-selection population, pre-tick-1 in both states, extinction, and
  one capture at the inspector's presence threshold width.
- The widest-line column measurement per pane state with the interior width beside it.
- The `SPEC-MOK-004` measurements with their commands.
- The pre-change and post-change export captures for O18, with digests.
- The long-run capture for O8, including the `truncated` indicator's state.
- Performance and memory figures with the commands and the platform stated.

Where a bullet is met by substitution rather than as written, the record says so on that bullet, in the terms
`VREC-MOK-019` used, rather than reinterpreting the bullet.

## Residual uncertainty

1. **Both sides of every count read the same engine.** The oracles are independent of the *observer*, not of the
   engine: a defect in the engine's own records would be reproduced identically by the accumulator and by the
   independent count, and this contract would pass. That risk is carried by `SPEC-MOK-001`'s own verification and is
   not reducible here. It is the reason O2 uses a second engine record wherever one exists, which narrows the gap
   without closing it.
2. **Frames are presentation, not a record.** `SPEC-MOK-003`'s *Outputs* states that no claim rests on a frame alone.
   The capture-based oracles — O13, O15, O16 — therefore evidence what was rendered at a moment and not a property of
   every possible viewport. O16 is checked at two widths, which is a sample of a plane.
3. **The identity in O3 assumes a decision record per Mokiterion per completed tick.** If that proves not to hold in
   some engine state, the identity is unattainable as `REQ-MOK-061` clause 2 states it, and `WO-MOK-020`'s first stop
   condition applies. This contract does not resolve it in advance because resolving it is a requirement decision.
4. **O10's monotonicity is checked on the declared seeds**, so a non-monotone population total is detectable rather
   than proven impossible.
5. **The performance figures are indicative.** They are retained so a later regression has a baseline, not because a
   threshold is being asserted.
