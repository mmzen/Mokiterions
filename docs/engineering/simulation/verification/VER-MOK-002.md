+++
id = "VER-MOK-002"
type = "verification"
title = "Perception and population viability verification"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
verifies = ["REQ-MOK-013", "REQ-MOK-014", "REQ-MOK-015"]
+++

# Verification Contract: Perception and population viability verification

## Independence

Verification checks observable behavior at the engine boundary and at the process boundary. It does not name
private Rust types, functions, modules, or files, and it does not assert how perception is computed or how the
reference source ranks candidates. Cases are stated so that any implementation satisfying `SPEC-MOK-001` passes.

Viability is measured by executing the simulation and counting survivors, never by re-deriving the expected
survivor count from the resource constants. A calculation that restates the specification would prove only
internal consistency.

The declared verification seed set is `0`, `1`, `42`, `123`, and `777`. It is fixed here so that viability
cannot be demonstrated on a favourable seed selected after the fact.

The declared density carrying a viability floor is `0.75%`, the default, and it is the only one. It is fixed here
so that the floor cannot later be demonstrated at a more comfortable density than the one the system ships with.
A floor is an obligation at a *point*, not a threshold: changing density changes how many coordinate draws
initialization performs, so each density is a distinct world rather than the same world with more food, and
passing at any higher density never stands in for passing at `0.75%`. The measured curve shows this per seed:
seed `0` leaves ten living at `0.50%` and eight at `0.75%`, and seed `777` leaves twelve at `1.00%` and eleven at
`1.25%`.

`1.50%` remains in this contract as a measured comparison point with no obligation attached. It is swept for
determinism and for the density curve, and its survivor counts are recorded as evidence, so that habitability
headroom is documented without being declared.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-013` | automated-test | Perception includes an in-radius resource with class, direction, and distance | Resource present with correct direction and distance |
| `REQ-MOK-013` | automated-test | Perception includes an in-radius living Mokiterion and excludes the observer | Other present, observer absent |
| `REQ-MOK-013` | automated-test | Out-of-radius resource and dead Mokiterion are excluded | Both absent; collections empty, not erroneous |
| `REQ-MOK-013` | automated-test | Perception crosses the territory boundary | In-radius entity across `y=63/64` is reported |
| `REQ-MOK-013` | automated-test | Perception order is stable and independent of collection iteration | Repeated observations byte-identical |
| `REQ-MOK-013` | automated-test | Perception consumes no entropy and mutates nothing | Entropy state and world state unchanged |
| `REQ-MOK-014` | automated-test | 1,000-tick run at the default density `0.75%` on each declared seed | ≥ 8 living Mokiterions on every seed |
| `REQ-MOK-014` | automated-test | Same runs report food consumption | Consumption events > 0 on every seed |
| `REQ-MOK-014` | evidence | 1,000-tick sweep at `1.50%` and across the density curve | Counts recorded in `density-curve.md`; no pass condition, no obligation |
| `REQ-MOK-014` | automated-test | Conditional regeneration is preserved | An emptied territory regenerates nothing, at any density |
| `REQ-MOK-014` | automated-test | Density resolves to the specified resource count | `0.15%`→12, `0.75%`→61, `1.50%`→122 per territory |
| `REQ-MOK-014` | automated-test | Density binds initialization, capacity, and replenishment target | A territory begins at, never exceeds, and is replenished toward the resolved count |
| `REQ-MOK-014` | automated-test | A density resolving to zero resources is invalid configuration | Rejected before initialization, exit code 2 |
| `REQ-MOK-014` | manual | Scarcity assessment at the default density | Not all seeds retain twelve survivors at `0.75%` |
| `REQ-MOK-015` | automated-test | Reference source moves toward a perceived resource, then eats it | Move proposed toward it; eat proposed when co-located and hungry |
| `REQ-MOK-015` | automated-test | Reference source does not consume wastefully | A near-full Mokiterion standing on a resource leaves it in place |
| `REQ-MOK-015` | automated-test | Reference source does not approach a resource it would decline | A near-full Mokiterion perceiving only clipping resources searches instead of approaching, and does not oscillate between two cells |
| `REQ-MOK-015` | automated-test | Reference source with no perceived food searches | A valid `move` is proposed, never `wait` |
| `REQ-MOK-015` | automated-test | Reference source sustains itself when energy is depleted | `sleep` proposed below the specified threshold, in preference to approach and search |
| `REQ-MOK-015` | automated-test | Reference source cannot mutate authoritative state | Boundary exposes no mutable state |
| `REQ-MOK-015` | automated-test | Both sources selectable; selection reported in output | Both run; output identifies the active source |
| `REQ-MOK-015` | automated-test | Reference source is deterministic | Two runs at one seed byte-identical |

## Acceptance scenarios

1. A default 1,000-tick run on each declared seed terminates by tick limit rather than extinction, leaves at
   least eight Mokiterions living, and reports non-zero food consumption.
2. A Mokiterion perceives a resource four cells east, moves toward it on successive ticks, arrives, and consumes
   it, with each step observable in the event stream.
3. A Mokiterion that perceives no resource keeps changing position on successive ticks, so the traced excerpt
   shows displacement rather than a repeated identical position.
4. A run with the random baseline source selected still executes correctly and is not held to the viability
   floor.
5. A territory emptied of resources regenerates nothing for the remainder of the run.

## Property and invariant tests

- Determinism: for every declared seed, two runs under each source at both swept densities produce
  byte-identical output and identical final state, preserving `REQ-MOK-009`. Density is parsed through exact
  integer arithmetic, so no floating-point value participates in the resolved resource count.
- Density invariants: at initialization every territory holds exactly the resolved count; across a whole run no
  territory ever exceeds it; and a territory reduced below it is replenished toward it but never past it.
- Attribute bounds: `health`, `satiety`, and `energy` remain within `0..=100` for every agent on every tick.
- Perception symmetry: if A perceives B at distance `d`, then B perceives A at distance `d`, both being living.
- Perception purity: building an observation leaves world state and entropy state unchanged.
- No action at a distance: a resource is removed only by a co-located eat, never by perception alone.
- Death finality: a dead Mokiterion receives no later observation, decision, action, trace, or survival update.
- Reference-source liveness: across a full run under the reference source, no `wait` is proposed, and energy
  never reaches zero for a living Mokiterion, because self-sustenance outranks approach and search.
- Entropy attribution: the reference source draws only for search steps. A run in which every Mokiterion always
  perceives a resource consumes no decision entropy at all.

## Static and architecture checks

- `cargo fmt --all -- --check` reports no differences.
- `cargo clippy --all-targets --all-features -- -D warnings` reports no findings.
- No external runtime dependency is added; `Cargo.toml` dependencies remain empty.
- The observation-to-proposed-action boundary still passes only copied or immutable values, and no decision
  source receives a mutable world, agent collection, resource collection, event log, or engine handle.
- Both decision sources implement the same boundary; neither has privileged access.

## Security and privacy checks

- No network access, credential read, or filesystem access is introduced.
- Output contains only configuration and simulation state, with no timestamps, absolute paths, or pointer values.
- Invalid input remains data and is never interpreted as code or a path.

## Performance and resilience checks

- A 10,000-tick run under each source completes without panic, and survivors plus deaths always equal twelve.
- Per-tick work remains bounded by twelve agents, the current resource collection, perception within a bounded
  radius, and emitted events.
- Extinction, if reached under the baseline source, terminates cleanly with exactly one summary.

## Manual assessments

- Read a 20-tick traced excerpt and confirm that reference-source behavior is legible: perception, approach,
  arrival, and consumption can be followed for one Mokiterion.
- Confirm that output and developer documentation identify the reference source as a development instrument
  rather than autonomous behavior.
- Assess measured carrying capacity against `INT-MOK-002`'s scarcity principle. A result at or near twelve
  survivors on every declared seed is an adverse observation requiring product review even though the literal
  floor is met.
- Measure the rate of two-cell oscillation, where a Mokiterion alternates between the same pair of cells, and
  compare it against the rate a random walk produces on the same world. The corrected rule 5 is expected to leave
  no systematic excess over that baseline; an excess is an adverse observation, because it means a Mokiterion is
  still re-targeting the cell it just left.
- Record the resource mix by calorie class at the end of a long run and compare it with the balanced initial mix.
  Divergence is expected and is characterised in the residual uncertainty above; it is retained so that a later
  change to the calorie table or to rule 5 can be measured against it.

## Evidence retention

Retain under `docs/engineering/simulation/evidence/WO-MOK-002/`:

- formatter, linter, test, and build output;
- the requirement-to-test mapping;
- per-seed 1,000-tick survivor counts and consumption totals, as the calibration record;
- the measured density curve, including densities that carry no obligation, and the per-seed counts behind it;
- the oscillation measurement and its random-walk baseline, and the end-of-run resource mix by calorie class;
- per-seed final resource count per territory, and whether either territory reached zero. Permanent depletion
  is reachable by design, so whether it actually occurs is a calibration fact worth recording;
- deterministic replay comparison for both sources;
- the 10,000-tick resilience result;
- the 20-tick traced manual-observation excerpt and its assessment;
- dependency, boundary, and credential review;
- a completion summary naming the final affected components.

## Residual uncertainty

- The survivor floor is verified on five declared seeds, not on the whole seed space. Passing does not prove
  viability for every seed.
- The floor is verified at one declared density and carries no claim at any other density, in either direction.
  Interpolation and extrapolation from a single point are not sound. The `1.50%` sweep is evidence about `1.50%`
  and nothing more.
- Measured carrying capacity depends on travel inefficiency and on resources spawning outside perception. The
  measurement is honest for the specified mapping but does not establish that the declared densities are optimal.
- The binding constraint on survival is the ratio of travel time to satiety drain, not the supply of resources.
  This was established by measurement, not derivation: a fourfold increase in regeneration yield changed no
  survivor count on any seed, and near-global perception changed almost none. The floors therefore rest on
  density, and any future change to world size, movement cost, satiety decay, or the calorie table invalidates
  them and requires re-measurement rather than re-derivation.
- **The floor has no margin.** The `0.75%` default was chosen to preserve scarcity, and its floor of eight sits
  exactly on the measured worst case of eight, reached on more than one declared seed. Any change to world size,
  movement cost, satiety decay, the calorie table, or the reference decision rules is likely to break it
  immediately. This is a deliberate trade rather than an oversight: `INT-MOK-002` treats abundance as failure, so
  the floor is stated at the scarce density the system ships with rather than at a comfortable one, and the
  `1.50%` sweep documents headroom without declaring it.
- **High-class resources accumulate, and the long horizon is worse than the short one.** The corrected rule 5
  makes a high-class resource both eatable and approachable only at satiety of at most `50`, so it is consumed
  less often than low or medium and occupies capacity that density fixes. Measurement at the default density puts
  high class at 45 of 61 resources in a territory by tick 1,000 and 45 again by tick 3,000, against a balanced
  initial third; under the previous rule the mix stayed near balanced. A 10,000-tick run at the default density
  reaches extinction at tick 9,154, where the previous rule left one survivor at tick 10,000.

  No requirement verified here speaks past tick 1,000, and at that horizon the corrected rule raises the measured
  worst case from three survivors to eight, so this contract records the effect rather than failing on it. The
  product owner accepted it knowingly on 2026-08-17 and deferred it to Phase 2. Two consequences follow for
  anyone reading a later measurement: the survivor floor is a claim about tick 1,000 and must not be read as a
  steady state, and any future requirement for long-horizon stability will need a rule that lets high-class
  resources be consumed, which will move this curve and require re-approval of `REQ-MOK-014`.
- Perception symmetry is checked as an invariant but is not proven exhaustively across all positions.
- This contract does not verify emergent behavior. It verifies that the world is survivable and perceptible,
  which is a precondition for studying behavior rather than evidence of it.
