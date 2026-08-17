# Boundary, dependency, and security review — WO-MOK-002

Regenerated 2026-08-17 after the third `SPEC-MOK-001` amendment.

The rule 5 correction changed which candidate the reference source returns and nothing else. It
added no input, no state, no collection, and no field to the observation: the non-waste test reads
`satiety` and the perceived resource's class, both of which the observation already carried. The
reference source remains a unit struct with no state, so nothing in this review's conclusions moved
and the sections below are re-confirmed rather than re-derived. One constant was removed —
the reference hunger threshold — because the corrected rule derives the same bound from the food
table instead of restating it.

## Dependencies

`Cargo.toml` has an empty `[dependencies]` section. `Cargo.lock` contains only the crate
itself. Nothing was added by this work order, and `ARCH-MOK-001`'s standard-library-only
constraint is intact. No escalation for a new dependency was needed.

## Trust boundary

`ADR-MOK-001` makes the decision source the trust boundary: it proposes, and the engine
validates and mutates. The signature is unchanged by this work order:

```rust
fn decide(&mut self, observation: &Observation, entropy: &mut DecisionEntropy<'_>) -> Action;
```

A decision source receives exactly two things:

- `&Observation`, an immutable value built for it, holding copied coordinates, attribute
  values, perceived-entity records, and the valid-action list. It holds no reference into the
  world, no agent collection, no resource collection, no event log, and no engine handle.
- `&mut DecisionEntropy`, a narrow wrapper exposing only `choose_index` and a draw counter.

Both sources implement the same trait and neither has privileged access. The reference source
is a unit struct with no state. `the_reference_source_cannot_mutate_authoritative_state`
verifies at the boundary that a full decision leaves world state and entropy state unchanged,
and `perception_grants_no_ability_to_act_at_a_distance` verifies that a perceived resource is
removed only by a co-located eat.

Density did not widen this boundary. It is a configuration value resolved to a resource count
inside the engine. No `Density` value and no capacity figure is exposed to a decision source,
so a source cannot infer or influence the resource economy other than by perceiving resources.

## The new input as an attack surface

`--density` takes untrusted operator text. It is treated as data throughout:

- Parsing accepts only ASCII digits and at most one `.`, with at most two decimal places.
  Anything else is rejected with a message and exit code 2.
- The parsed value is an integer count of hundredths of a percent. No floating-point value is
  constructed, so no `NaN`, infinity, or platform-dependent rounding can enter.
- Overflow is handled with `checked_mul` and `checked_add`, and the value is capped at `100`,
  so no arithmetic can wrap.
- A density resolving to zero resources is rejected before initialization rather than producing
  an unrecoverable world. `a_density_resolving_to_no_resources_exits_with_code_two_before_initialization`
  asserts that no simulation output is emitted in that case.
- The rejected value is never interpolated into a path, a command, or executable form. It
  appears only in a diagnostic string on stderr.

The upper bound of `100%` resolves to 8,192 resources per territory, which is every cell. That
is a legal but degenerate configuration and carries no viability obligation. It is bounded, so
it cannot be used to exhaust memory beyond the world's own cell count.

## Network, credential, and filesystem access

None is introduced. A scan of `src/` finds no `std::net`, no `std::fs`, no HTTP or async client,
and no credential, key, token, or secret handling. The only environment access is
`std::env::args` in `main`, for command-line arguments.

`WO-MOK-002` excludes model-provider integration, prompts, agent memory, and credentials, and
nothing of that kind was added. No secret is present in the repository or in any evidence file
here.

## Output hygiene

Output contains only configuration echo and simulation state: ticks, identifiers, event names,
coordinates, classes, and attribute values. No timestamp, absolute path, pointer value,
username, or hostname is emitted, so output is diffable across machines. This is what makes the
byte-identical replay comparison in `determinism-and-resilience.md` meaningful.

The density appears in output only as part of the world initialization and resource events,
which is state rather than provenance.

## Resource bounds

Per-tick work is bounded by twelve agents, the resource collection, perception within a radius
of 16, and emitted events. Density raises the resource-collection constant factor: 61 per
territory at the default and 122 at `1.50%`, against 12 under the superseded constant. A
10,000-tick run completes in well under a second, so the increase is measured and immaterial at
these sizes. There is no unbounded allocation, no recursion, and no `unsafe` block anywhere in
the crate.
