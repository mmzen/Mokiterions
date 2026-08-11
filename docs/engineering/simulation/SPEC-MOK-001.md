+++
id = "SPEC-MOK-001"
type = "specification"
title = "Minimum simulation foundation behavior"
status = "approved"
owners = ["technical owner"]
created = "2026-08-11"
updated = "2026-08-11"

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
]
+++

# Specification: Minimum simulation foundation behavior

## Scope

This specification defines the smallest local, in-memory, text-only Mokiterions simulation. It fixes the world layout, initial state, tick order, survival values, core actions, food behavior, deterministic baseline decisions, output, and termination needed to implement `CAP-MOK-001`.

It does not define OpenAI integration, fear, combat, social behavior, persistence, or a user interface.

## Actors and external systems

- The operator starts the process and reads standard output and standard error.
- The baseline decision source proposes actions from bounded observations.
- The simulation engine is the only authority that changes world state.
- There are no external systems or network calls.

## Inputs

The binary accepts:

```text
Mokiterions [--seed <u64>] [--ticks <u64>] [--trace-actions]
```

- `--seed` defaults to `0`.
- `--ticks` defaults to `100` and must be greater than zero.
- `--trace-actions` accepts no value, defaults to disabled, and enables one detailed action trace for every living-agent decision opportunity.
- Options may appear in any order and may appear at most once.
- `--help` prints usage and exits successfully without starting a simulation.
- Unknown, duplicate, missing, or invalid option values are invalid configuration.

## Outputs

- Deterministic simulation events and the final summary are written to standard output.
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

Each territory starts with exactly three resources: one of each class. Initial food coordinates are unique within a territory and are selected using seeded entropy. A territory has a maximum capacity of twelve resources.

### Time and entropy

- Time is an integer tick beginning at `0`; agent processing begins on tick `1`.
- One explicit SplitMix64 pseudo-random stream, seeded from `--seed`, supplies all initialization, baseline-decision, and regeneration entropy.
- Random selection uses a stable candidate order and an unbiased bounded selection method.

## Behavioral rules

1. **Initialization order.** Create the entropy stream, world, initial food, and agents in that order. Emit initialization events only after the complete initial state is valid.
2. **Tick start.** Increment the tick once, then consider living agents in ascending identifier order.
3. **Observation.** For each considered agent, the engine creates a read-only observation containing tick, identity, position, territory, health, satiety, energy, co-located food, valid cardinal moves, and the complete list of currently valid core action proposals.
4. **Baseline decision.** Candidate proposals use this stable order: `wait`; `sleep` when energy is below `100`; `eat` for each co-located resource in identifier order; and valid `move` actions ordered north, east, south, west. The baseline consumes one entropy selection and returns one candidate.
5. **Validation.** The engine validates the returned proposal against current authoritative state. A rejected proposal consumes the action opportunity, produces a rejection result, and causes no action-specific mutation.
6. **Optional action trace.** When `--trace-actions` is enabled, emit exactly one `action_trace` line after validation and any valid action-specific mutation, but before survival decay. The line contains the tick, agent identifier, proposed action, `accepted` or `rejected` status, result or rejection reason, position, territory, health, satiety, and energy. When the flag is disabled, emit no `action_trace` lines. Trace configuration never changes entropy consumption or simulation state.
7. **Move.** A valid move changes one coordinate by one cell in a cardinal direction. Crossing `y=63/64` updates the derived territory and emits a crossing event. Movement has no additional energy cost in this foundation.
8. **Eat.** A valid eat selects one co-located resource by identifier, removes it, and restores the class values in the food table. Attribute values are capped at `100`.
9. **Sleep.** Sleep restores `20` energy, capped at `100`, before survival decay. It does not move the agent or consume food.
10. **Wait.** Wait causes no action-specific mutation.
11. **Survival decay.** After the action opportunity, subtract `2` satiety and `1` energy using saturation at zero. If either resulting value is zero, subtract `5` health using saturation at zero.
12. **Death.** When health becomes zero, mark the agent dead and emit one death event. Dead agents receive no later observations, decisions, actions, traces, or survival updates.
13. **Regeneration timing.** After all scheduled agents are processed, each territory receives one regeneration opportunity on ticks divisible by `10`.
14. **Regeneration condition.** A territory with between one and eleven resources adds exactly one resource. A territory with zero resources or twelve resources adds none and emits a skipped-regeneration event.
15. **Regeneration selection.** The new class is selected uniformly from low, medium, and high. Its coordinate is selected from currently food-free coordinates in that territory. Both selections use the shared entropy stream.
16. **Termination.** After regeneration, terminate if all agents are dead or the configured tick limit has been reached. Extinction takes precedence when both conditions occur on the same tick.
17. **Final summary.** Emit the termination reason, elapsed ticks, survivors, deaths, population by current territory, and remaining food by territory and calorie class exactly once.

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

`Observation` contains copied or immutable values. `ProposedAction` is one of:

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

Stable core event types are `world_initialized`, `food_initialized`, `agent_initialized`, `food_consumed`, `food_regenerated`, `food_regeneration_skipped`, `territory_crossed`, `survival_changed`, `agent_died`, and `simulation_ended`. Optional per-action lines use `action_trace`.

An action trace uses the same leading fields and stable details in this order:

```text
tick=<number> subject=<mokiterion-id> event=action_trace result=proposal:<action>,status:<accepted|rejected>,detail:<result-or-reason>,position:<x:y>,territory:<A|B>,health:<number>,satiety:<number>,energy:<number>
```

The final line begins with `summary` and reports fields in the order defined by rule 16. Stable details must not contain wall-clock timestamps, absolute paths, pointer values, or unordered collection formatting.

## Security and privacy properties

- The foundation reads no credentials and performs no network access.
- Output contains only simulation configuration and state.
- Invalid input is treated as data and never interpreted as code or a filesystem path.

## Performance and capacity

- The foundation supports exactly twelve agents and a 128 by 128 world.
- Per-tick work is bounded by the twelve agents, the current food collection, and emitted events.
- State is held in memory and no persistence is required.

## Observability

Every initialization, food consumption or regeneration result, territory crossing, survival attribute change, death, and termination is emitted in authoritative processing order. When action tracing is enabled, every living-agent decision opportunity additionally emits one ordered action trace. Identical runs with identical trace configuration produce byte-identical standard output.

## Compatibility and migration

There is no prior data or interface compatibility obligation. Future output or model interfaces may replace this foundation only through later approved artifacts.

## Examples and counterexamples

- A move from `(10, 63)` south to `(10, 64)` is valid and crosses from territory A to B.
- A move north from `(10, 0)` is invalid and leaves position unchanged.
- Eating a resource at another coordinate is invalid.
- If territory A has one resource on tick `10`, it regenerates one resource; if it has zero, it remains at zero.
- Sleeping at energy `90` raises energy to `100`, then tick decay leaves it at `99`.
- An agent at satiety `1`, energy `50`, and health `5` waits, reaches satiety `0`, loses `5` health, dies, and is not processed again.
- A tick beginning with twelve living agents emits exactly twelve `action_trace` lines when `--trace-actions` is enabled and none when it is disabled.

## Explicitly unspecified decisions

- Rust file and private type names.
- Choice of collection types where iteration is explicitly sorted before observable use.
- Internal error type layout and message wording, except for exit codes and required clarity.
- Test organization and helper functions.
- Cosmetic whitespace in help text.
