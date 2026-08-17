+++
id = "SPEC-MOK-001"
type = "specification"
title = "Minimum simulation foundation behavior"
status = "approved"
owners = ["technical owner"]
created = "2026-08-11"
updated = "2026-08-17"

[relations]
specifies = [
  "REQ-MOK-001",
  "REQ-MOK-002",
  "REQ-MOK-003",
  "REQ-MOK-004",
  "REQ-MOK-005",
  "REQ-MOK-006",
  "REQ-MOK-007",
  "REQ-MOK-008",
  "REQ-MOK-009",
  "REQ-MOK-010",
  "REQ-MOK-011",
  "REQ-MOK-012",
  "REQ-MOK-013",
  "REQ-MOK-014",
  "REQ-MOK-015",
]
+++

# Specification: Minimum simulation foundation behavior

## Scope

This specification defines the smallest local, in-memory, text-only Mokiterions simulation. It fixes the world layout, initial state, tick order, survival values, core actions, food behavior, bounded perception, deterministic decision sources, output, and termination needed to implement `CAP-MOK-001` and `CAP-MOK-002`.

It does not define OpenAI integration, fear, individual traits, combat, social behavior, persistence, structured output, or a user interface.

This is the single behavior contract for the simulation core. It is amended in place rather than superseded, so that no two active specifications state conflicting survival or resource values.

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-11 | Original approved content for `CAP-MOK-001`. | Approved; implemented under `WO-MOK-001` and verified under `VREC-MOK-001`. |
| 2026-08-17 | Added bounded perception, the reference decision source, the `--policy` input, and the decision-source output line. Changed satiety decay from `2` to `1` and regeneration from one resource to `2`. | Approved 2026-08-17 by the repository owner acting as technical owner, together with `REQ-MOK-013`, `REQ-MOK-014`, `REQ-MOK-015`, and `WO-MOK-002`. |
| 2026-08-17 | Narrowed the *Explicitly unspecified decisions* entry on test organization: helper functions and the internal organization of a test module remain delegated, while crate target layout, the public interface, and test placement are governed by `SPEC-MOK-002`. No specified behavior changed. | Approved 2026-08-17 by the repository owner acting as technical owner, as required by `ADR-MOK-002` and as an approval precondition of `WO-MOK-003`. |
| 2026-08-17 | Replaced the fixed initial endowment and fixed territory capacity with a single `--density` input that sets the initial density, the capacity ceiling, and the replenishment target together. Corrected rule 5 case 1 from a fixed satiety threshold to the non-wasteful rule `REQ-MOK-015` already required. | Approved 2026-08-17 by the repository owner acting as technical owner, together with the amended `REQ-MOK-014`, on the measured evidence in `docs/engineering/simulation/evidence/WO-MOK-002/`. |
| 2026-08-17 | Extended the rule 5 non-waste test from case 1 to case 3, so a Mokiterion neither eats nor approaches a resource whose restoration would be clipped. Removed the unreachable fallback clause from case 1. Retracted the false claim that correcting case 1 alone removed the two-cell oscillation. | Approved 2026-08-17 by the repository owner acting as technical owner, together with the amended `REQ-MOK-014`, on the measured evidence in `docs/engineering/simulation/evidence/WO-MOK-002/density-curve.md`. |

The released implementation at commit `09c4e1a` conforms to the 2026-08-11 content. `VREC-MOK-001` remains the
commit-bound record of that earlier content.

The first 2026-08-17 amendment was implemented and measured under `WO-MOK-002`, and its viability claim failed:
zero survivors on every declared seed. The second amendment corrects that failure. Its reasoning is recorded in
`docs/engineering/simulation/evidence/WO-MOK-002/escalation.md`, and the two facts that drove it are that
regeneration yield had no measurable effect on survival because territory capacity was the real ceiling on
standing supply, and that near-global perception did not help either, because the binding constraint was travel
time against satiety drain during the regeneration ramp.

The second amendment was also implemented and measured, and its viability claim also failed, at three survivors
against a stated floor of five. The cause was a defect in the rule it introduced rather than in the density
design: the non-waste test governed eating but not seeking, so the two effects the amendment claimed to remove
were only half removed. The third amendment closes that gap and is the amendment the floor of eight rests on.
Applying the test to both cases raised the measured worst case at the default density from three survivors to
eight. It also removes a false statement that the second amendment left standing in this document.

## Actors and external systems

- The operator starts the process, selects a decision source, and reads standard output and standard error.
- A decision source proposes actions from bounded observations. Two exist: the random baseline source and the food-seeking reference source.
- The simulation engine is the only authority that changes world state.
- There are no external systems or network calls.

## Inputs

The binary accepts:

```text
Mokiterions [--seed <u64>] [--ticks <u64>] [--policy <baseline|reference>]
            [--density <percent>] [--trace-actions]
```

- `--seed` defaults to `0`.
- `--ticks` defaults to `100` and must be greater than zero.
- `--policy` selects the decision source and defaults to `reference`. Only `baseline` and `reference` are valid values.
- `--density` selects the resource density as a percentage of a territory's cells and defaults to `0.75`. It accepts a decimal value with at most two decimal places, so the smallest representable step is `0.01`. It must resolve to at least one resource per territory and must not exceed `100`.
- `--trace-actions` accepts no value, defaults to disabled, and enables one detailed action trace for every living-agent decision opportunity.
- Options may appear in any order and may appear at most once.
- `--help` prints usage and exits successfully without starting a simulation.
- Unknown, duplicate, missing, or invalid option values are invalid configuration.

## Outputs

- Deterministic simulation events and the final summary are written to standard output.
- The selected decision source is reported exactly once on standard output, before agent processing begins, so that no run is ambiguous about which policy produced it.
- Detailed per-action trace lines are additionally written to standard output only when `--trace-actions` is enabled.
- Usage and configuration errors are written to standard error.
- Successful help or simulation completion exits with code `0`.
- Invalid configuration exits with code `2`.
- An unrecoverable runtime or output failure exits with code `1`.

## State model

### World

- Coordinates are integer pairs `(x, y)` where both values are in `0..=127`.
- Territory A contains `y` values `0..=63`.
- Territory B contains `y` values `64..=127`.
- Territory boundaries label state but do not block movement.

### Mokiterion

Each Mokiterion contains:

- a stable identifier from `M01` through `M12`;
- a current coordinate;
- a derived current territory;
- integer `health`, `satiety`, and `energy` values in `0..=100`;
- a living or dead state.

All three attributes start at `100`. `M01` through `M06` start in territory A and `M07` through `M12` start in territory B. Initial coordinates are selected without duplicate agent positions using seeded entropy.

Multiple agents may occupy the same coordinate after initialization. Agents do not block movement. An agent and a food resource may share a coordinate.

### Food

Each resource has a stable identifier, coordinate, territory, and calorie class:

| Class | Satiety restored | Energy restored |
|---|---:|---:|
| Low | 15 | 5 |
| Medium | 30 | 10 |
| High | 50 | 20 |

### Resource density

Density is the single input that governs how much food a territory holds. It is stated as a percentage of that
territory's cells.

- A territory contains `128 * 64 = 8192` cells. Density is expressed relative to one territory, not to the whole world.
- The percentage is parsed as an exact integer count of hundredths of a percent, so `0.75` becomes `75`. Floating-point arithmetic is not used anywhere in this conversion, because `REQ-MOK-009` requires byte-identical reproducibility.
- The resource count per territory is `hundredths * 8192 / 10000`, using integer division that truncates toward zero. So `0.15%` yields `12`, `0.75%` yields `61`, and `1.50%` yields `122`.
- A density that truncates to zero resources is invalid configuration, because a territory with no resources can never regenerate under rule 15 and the run would be predetermined.

The resolved resource count binds **three** roles at once, and this is the substance of the input rather than an
incidental consequence:

1. the number of resources each territory holds at initialization;
2. the maximum capacity of a territory;
3. the level that regeneration replenishes toward.

Binding all three is deliberate. When capacity alone was raised, a territory still began near-empty and climbed
toward its ceiling over hundreds of ticks, and the population died during that ramp before the ceiling had any
effect. A single density value that sets the starting level, the ceiling, and the target removes the ramp: a
territory begins at its intended density and regeneration restores it toward that same density after consumption.

Initial resources are assigned classes by cycling through low, medium, and high in that order, so composition is
an even three-way split up to a remainder of at most two. Initial coordinates are unique within a territory and
are drawn from currently food-free coordinates in that territory using seeded entropy, evaluated after each
resource already placed.

Because the resource count determines how many coordinate draws initialization performs, two runs at different
densities consume the entropy stream differently and are therefore different worlds rather than the same world
holding more food. Runs are comparable only with runs at the same density, exactly as they are comparable only
with runs under the same decision source.

### Perception

- The perception radius is `16` cells.
- Distance is Chebyshev distance: the greater of the absolute differences in `x` and `y`. An entity is perceived when that distance is at most `16`.
- Relative direction is one of `north`, `north_east`, `east`, `south_east`, `south`, `south_west`, `west`, `north_west`, determined by the sign of the coordinate differences. A non-zero difference in both axes yields a diagonal direction.
- Perception does not stop at a territory boundary and is not blocked by other Mokiterions.
- Perception is read-only, consumes no entropy, and mutates nothing.

### Time and entropy

- Time is an integer tick beginning at `0`; agent processing begins on tick `1`.
- One explicit SplitMix64 pseudo-random stream, seeded from `--seed`, supplies all initialization, decision-source, and regeneration entropy. The two decision sources consume from it at different rates, so a run under one source is comparable only with runs under the same source.
- Random selection uses a stable candidate order and an unbiased bounded selection method.

## Behavioral rules

1. **Initialization order.** Create the entropy stream, world, initial food, and agents in that order. Emit initialization events only after the complete initial state is valid, then emit the selected decision source exactly once before tick processing begins.
2. **Tick start.** Increment the tick once, then consider living agents in ascending identifier order.
3. **Observation.** For each considered agent, the engine creates a read-only observation containing tick, identity, position, territory, health, satiety, energy, co-located food, perceived food, perceived Mokiterions, valid cardinal moves, and the complete list of currently valid core action proposals.

   Perceived food lists every resource within the perception radius, including co-located resources, each with identifier, class, relative direction, and distance, ordered by ascending distance and then by identifier. Perceived Mokiterions lists every other living Mokiterion within the radius, each with identifier, relative direction, and distance, ordered by ascending distance and then by identifier. The observer never appears in its own perceived-Mokiterion list, and dead Mokiterions never appear. Co-located food remains reported separately and unchanged.
4. **Baseline decision.** Candidate proposals use this stable order: `wait`; `sleep` when energy is below `100`; `eat` for each co-located resource in identifier order; and valid `move` actions ordered north, east, south, west. The baseline consumes one entropy selection and returns one candidate.
5. **Reference decision.** The reference source returns the first applicable candidate in this order:

   1. `eat` the co-located resource of the highest calorie class whose satiety restoration would not exceed the attribute maximum of `100`, breaking ties by lowest identifier;
   2. `sleep` when energy is below `20`;
   3. `move` one cell toward the nearest perceived resource at a distance greater than zero whose satiety restoration would not exceed the attribute maximum of `100`, breaking ties by highest calorie class and then by lowest identifier. Move on the horizontal axis while the perceived direction has an easterly or westerly component, otherwise on the vertical axis. When that move is invalid, use the other axis;
   4. `move` as a search step, selected from the valid `move` actions in the order north, east, south, west using one entropy selection.

   Only the search step of case 4 consumes entropy. Cases 1 through 3 consume none. The reference source never proposes `wait`: a blind Mokiterion searches rather than stands still.

   Cases 1 and 3 apply the same non-waste test, and applying it to both is the point of each. `REQ-MOK-015` requires consuming "when consuming it is not wasteful": eating a resource whose restoration would be clipped by the attribute maximum discards the remainder, and it can drive a territory to zero resources, which permanently ends its regeneration under rule 15. Case 3 applies the identical test to *seeking*, so a Mokiterion never walks toward a resource it would decline on arrival. When every perceived resource would be clipped, case 3 does not apply and the Mokiterion searches under case 4, because approaching a resource it will not eat accomplishes less than looking for one it will.

   Two earlier forms of these rules were defective. The record is kept because the second defect was found only by measurement, after this specification had asserted the opposite.

   The first form gated case 1 on a fixed satiety threshold of `50` alone. Satiety above `50` could never be spent, because eating again always required decaying back to `50` first, so the range 51 to 100 was buffer that could not fund travel. The non-waste test removes that dead buffer.

   The second form applied the non-waste test to case 1 only, and it produced a stable two-cell oscillation: a Mokiterion standing on a resource it declined stepped off under case 3, immediately perceived that same resource as the nearest resource at a distance greater than zero, and stepped back. **An earlier revision of this specification claimed that correcting case 1 removed this effect. That claim was false and is retracted.** For high-class resources the non-waste condition is satisfied only at satiety of at most `50`, numerically identical to the threshold it replaced, so the richest third of the resource table oscillated exactly as before — measured at 35.7% of agent-ticks against the 12.2% an unbiased cardinal walk produces on the same world. Extending the test to case 3 removes the cause rather than the symptom, because the resource just left is excluded from targeting for exactly as long as it would be declined. The measured residual is 10.6%, below the random-walk rate.

   One consequence of the corrected rule is specified rather than accidental. A high-class resource restores `50` satiety, so it is both eatable and approachable only at satiety of at most `50`. High-class resources are therefore consumed less often than low or medium and accumulate against the territory capacity that density fixes: measurement at the default density puts high class at 45 of 61 resources in a territory by tick 1,000, against a balanced initial third. This is accepted at the 1,000-tick horizon `REQ-MOK-014` states, where the corrected rule raises the measured worst case from three survivors to eight. It is recorded as a known long-horizon effect rather than a defect, and addressing it is out of scope for this revision.

   Because cardinal movement costs one tick per cell on either axis, the axis rule in case 3 changes the shape of an approach path but never its total cost.
6. **Validation.** The engine validates the returned proposal against current authoritative state. A rejected proposal consumes the action opportunity, produces a rejection result, and causes no action-specific mutation.
7. **Optional action trace.** When `--trace-actions` is enabled, emit exactly one `action_trace` line after validation and any valid action-specific mutation, but before survival decay. The line contains the tick, agent identifier, proposed action, `accepted` or `rejected` status, result or rejection reason, position, territory, health, satiety, and energy. When the flag is disabled, emit no `action_trace` lines. Trace configuration never changes entropy consumption or simulation state.
8. **Move.** A valid move changes one coordinate by one cell in a cardinal direction. Crossing `y=63/64` updates the derived territory and emits a crossing event. Movement has no additional energy cost in this foundation.
9. **Eat.** A valid eat selects one co-located resource by identifier, removes it, and restores the class values in the food table. Attribute values are capped at `100`.
10. **Sleep.** Sleep restores `20` energy, capped at `100`, before survival decay. It does not move the agent or consume food.
11. **Wait.** Wait causes no action-specific mutation.
12. **Survival decay.** After the action opportunity, subtract `1` satiety and `1` energy using saturation at zero. If either resulting value is zero, subtract `5` health using saturation at zero.
13. **Death.** When health becomes zero, mark the agent dead and emit one death event. Dead agents receive no later observations, decisions, actions, traces, or survival updates.
14. **Regeneration timing.** After all scheduled agents are processed, each territory receives one regeneration opportunity on ticks divisible by `10`.
15. **Regeneration condition.** A territory holding at least one resource and fewer than its capacity adds `2` resources, or fewer when fewer free capacity slots remain. A territory with zero resources adds none and emits a skipped-regeneration event, so permanent local depletion remains reachable at every density. A territory already at capacity adds none and emits a skipped-regeneration event.
16. **Regeneration selection.** Each new class is selected uniformly from low, medium, and high. Each coordinate is selected from currently food-free coordinates in that territory, evaluated after any resource added earlier in the same opportunity. All selections use the shared entropy stream, first class then coordinate, for each added resource in turn.
17. **Termination.** After regeneration, terminate if all agents are dead or the configured tick limit has been reached. Extinction takes precedence when both conditions occur on the same tick.
18. **Final summary.** Emit the termination reason, elapsed ticks, survivors, deaths, population by current territory, and remaining food by territory and calorie class exactly once.

## Error and recovery behavior

- Configuration is fully validated before state initialization.
- Initialization is atomic: failure produces no simulation events or partial run.
- Invalid action proposals are recoverable rejections and do not terminate the run.
- Integer arithmetic saturates at attribute bounds and never wraps.
- A standard-output write failure terminates the process with runtime exit code `1`; no successful summary is claimed.

## Data and interface contracts

The decision boundary exposes only:

```text
Observation -> ProposedAction
```

`Observation` contains copied or immutable values, including the perceived resources and perceived Mokiterions of rule 3. A perceived entry carries an identifier, a relative direction, a distance, and, for a resource, its calorie class. It carries no reference to the entity it describes, so a decision source can read a perceived entity but cannot reach it. `ProposedAction` is one of:

```text
Wait
Sleep
Eat { food_id }
Move { direction: North | East | South | West }
```

The decision source never receives a mutable world, agent, resource collection, event log, or engine handle.

Event lines use stable key-value fields in this order:

```text
tick=<number> subject=<identifier> event=<event-type> result=<stable-details>
```

Stable core event types are `world_initialized`, `food_initialized`, `agent_initialized`, `decision_source_selected`, `food_consumed`, `food_regenerated`, `food_regeneration_skipped`, `territory_crossed`, `survival_changed`, `agent_died`, and `simulation_ended`. Optional per-action lines use `action_trace`. The details of `decision_source_selected` name the active source as `baseline` or `reference`.

An action trace uses the same leading fields and stable details in this order:

```text
tick=<number> subject=<mokiterion-id> event=action_trace result=proposal:<action>,status:<accepted|rejected>,detail:<result-or-reason>,position:<x:y>,territory:<A|B>,health:<number>,satiety:<number>,energy:<number>
```

The final line begins with `summary` and reports fields in the order defined by rule 18. Stable details must not contain wall-clock timestamps, absolute paths, pointer values, or unordered collection formatting.

## Security and privacy properties

- The foundation reads no credentials and performs no network access.
- Output contains only simulation configuration and state.
- Invalid input is treated as data and never interpreted as code or a filesystem path.

## Performance and capacity

- The foundation supports exactly twelve agents and a 128 by 128 world.
- Per-tick work is bounded by the twelve agents, the current food collection, perception within the bounded radius, and emitted events.
- State is held in memory and no persistence is required.

## Observability

Every initialization, decision-source selection, food consumption or regeneration result, territory crossing, survival attribute change, death, and termination is emitted in authoritative processing order. When action tracing is enabled, every living-agent decision opportunity additionally emits one ordered action trace. Identical runs with identical trace configuration produce byte-identical standard output.

## Compatibility and migration

There is no prior data or interface compatibility obligation. Future output or model interfaces may replace this foundation only through later approved artifacts.

## Examples and counterexamples

- A move from `(10, 63)` south to `(10, 64)` is valid and crosses from territory A to B.
- A move north from `(10, 0)` is invalid and leaves position unchanged.
- Eating a resource at another coordinate is invalid.
- At the default density of `0.75%`, each territory resolves to `61` resources and begins holding exactly that many. If territory A has been eaten down to `58` on tick `10`, it regenerates two and reaches `60`; if it holds `60`, it adds one and reaches capacity `61`; if it holds `61`, it adds none; if it holds zero, it remains at zero forever.
- A density of `0.15%` resolves to `12` resources per territory, reproducing the resource count of the original 2026-08-11 world, and a density of `1.50%` resolves to `122`.
- A density of `0.01%` resolves to `8192 * 1 / 10000 = 0` resources and is invalid configuration.
- Sleeping at energy `90` raises energy to `100`, then tick decay leaves it at `99`.
- An agent at satiety `1`, energy `50`, and health `5` waits, reaches satiety `0`, loses `5` health, dies, and is not processed again.
- A Mokiterion at `(40, 20)` perceives a medium resource at `(44, 20)` as `east` at distance `4`, and does not perceive a resource at `(100, 20)`.
- The same Mokiterion under the reference source, with energy of at least `20`, proposes `move east` on each tick until it is co-located, then proposes `eat` for that resource because a medium resource restores `30` and satiety is at most `70`. Under the baseline source it may propose any valid action.
- A Mokiterion under the reference source perceiving no resource proposes a search `move`, never `wait`. At energy `19` it proposes `sleep` instead, and resumes searching once energy is at least `20`.
- A Mokiterion standing on a high-calorie resource at satiety `80` does not eat it, because `80 + 50` exceeds `100`. It leaves the resource in place, and the resource still counts toward the territory's regeneration condition. Standing on a low-calorie resource at satiety `80` it does eat, because `80 + 15` fits.
- A Mokiterion standing only on a high-calorie resource at satiety `45` eats it despite the waste, because satiety is at most `50` and starving beside food is worse.
- A Mokiterion at `(40, 20)` perceiving only a resource at `(43, 17)` moves `east` first, because the perceived direction `north_east` has an easterly component and the horizontal axis is taken first.
- A tick beginning with twelve living agents emits exactly twelve `action_trace` lines when `--trace-actions` is enabled and none when it is disabled.

## Explicitly unspecified decisions

- Rust file and private type names.
- Choice of collection types where iteration is explicitly sorted before observable use.
- Internal error type layout and message wording, except for exit codes and required clarity.
- Test helper functions, and the internal organization of a test module within the source file that owns it. Crate
  target layout, the public interface, and which tier a test belongs to are not unspecified: they are governed by
  `SPEC-MOK-002`.
- Cosmetic whitespace in help text.
