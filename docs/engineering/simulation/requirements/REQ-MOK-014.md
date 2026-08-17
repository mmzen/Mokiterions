+++
id = "REQ-MOK-014"
type = "requirement"
title = "Sustain a viable population at a stated resource density"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN a simulation runs to 1,000 ticks using the reference decision source at a declared resource density, THE SYSTEM SHALL leave at least the survivor floor stated for that density living."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-002"]
+++

# Requirement: Sustain a viable population at a stated resource density

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-17 | Original approved content: an absolute floor of eight of twelve at the default configuration. | Approved 2026-08-17 by the repository owner acting as product owner. |
| 2026-08-17 | Restated as a conditional obligation over declared resource densities, with a tabulated floor. Original absolute form measured zero survivors on every declared seed under `WO-MOK-002`. | Approved 2026-08-17 by the repository owner acting as product owner, on the measured evidence in `docs/engineering/simulation/evidence/WO-MOK-002/`. |
| 2026-08-17 | Reduced the declared density set to the default `0.75%` alone and raised its floor from five to eight. The two floors approved earlier were selected from a curve measured before the rule 5 correction that the same decision mandated, and both failed. The `1.50%` headroom row was withdrawn because it now retains all twelve survivors on a majority of declared seeds. | Approved 2026-08-17 by the repository owner acting as product owner, on the re-measured curve in `docs/engineering/simulation/evidence/WO-MOK-002/density-curve.md` following the third `SPEC-MOK-001` amendment. |

## Rationale

The original form of this requirement stated an absolute floor of eight of twelve at whatever resource constants
`SPEC-MOK-001` happened to fix. Implementation measured **zero survivors on every declared seed**. The
requirement was not merely miscalibrated; it was the wrong shape. An absolute survivor floor silently depends on
a resource density that the requirement never named, so it could not be reasoned about, and being wrong about
that density made the requirement unsatisfiable rather than merely demanding.

The obligation that the world must sustain a population is a genuine product requirement and stays. What changes
is that it is now stated *as a function of the resource density the operator selects*, and density becomes an
explicit simulation input rather than a hidden constant. This makes the requirement measurable, makes the
habitability of the world a subject of study rather than an assumption, and makes future policy comparisons
possible at matched density.

The floor below was chosen from measurement, not estimation. See
`docs/engineering/simulation/evidence/WO-MOK-002/density-curve.md`.

## The stated floor

| Minimum density | Resources per territory | Survivor floor at 1,000 ticks |
|---:|---:|---:|
| `0.75%`, the default | 61 | 8 of 12 |

Density is the fraction of a territory's cells holding a resource, as defined by `SPEC-MOK-001`. The row was
measured across the full declared verification seed set.

One density is declared, and it is the default. Two considerations produced that shape rather than a table.

A higher row would have documented habitability headroom, and the natural candidate, `1.50%`, retains all twelve
Mokiterions on a majority of declared seeds. Stating a floor there would mean permanently declaring a density
whose own verification raises the adverse observation below. Headroom is worth knowing and is recorded as
measured evidence in `density-curve.md`; it is not worth an obligation.

The floor of eight was the product intent from the outset, and the corrected `SPEC-MOK-001` rule 5 delivers it at
the scarce default rather than at a comfortable density. That is the claim worth making: the world is habitable
where it is meant to be tight. The margin is disclosed in `VER-MOK-002` and is thin by choice.

## Preconditions and trigger

A simulation is started with the reference decision source at one of the declared densities above, and runs to a
tick limit of at least `1,000`.

## Required response

At tick `1,000`, at least the stated floor for the selected density is living. The obligation holds across every
seed in the declared verification seed set, not merely on a favourable seed.

## Failure and boundary behavior

- Fewer survivors than the floor stated for that density, on any declared seed, fails this requirement.
- Twelve survivors on every declared seed at the default density satisfies the literal floor but indicates that
  scarcity has been removed. `VER-MOK-002` reports this as an adverse observation requiring product review,
  because it contradicts `INT-MOK-002`'s scarcity principle.
- **The obligation must not be read as monotonic in density.** A density above the declared minimum carries no
  guarantee. Changing density changes how many coordinate draws initialization performs, so two densities produce
  different worlds rather than the same world with more food. Two measured examples: seed `0` leaves ten living at
  `0.50%` and eight at `0.75%`, and seed `777` leaves twelve at `1.00%` and eleven at `1.25%`. Each declared
  density is a separate obligation verified on its own full seed sweep. Adding a density row requires re-approval
  and its own measurement, never interpolation from this one.
- Densities other than the one declared above carry no viability obligation at all. They remain valid inputs, and
  the operator may explore them, but they are unverified.
- A non-default tick limit, a non-declared seed, or the random baseline source carries no viability obligation;
  the baseline in particular is expected to starve.
- The requirement states a survivor floor, not a prohibition on death. Individual deaths are expected.

## Constraints

- Viability must be achieved by the resource economy, not by weakening death. Health, satiety, and energy bounds,
  decay-driven health loss, and the finality of death are unchanged.
- Conditional regeneration is preserved exactly. A territory emptied of resources must still lose its ability to
  regenerate, so permanent local depletion remains reachable at any density.
- Viability must not be achieved by granting resources to Mokiterions. Food must still be located, reached, and
  consumed.
- Determinism is preserved, so every measurement here is reproducible rather than statistical.
- Density selection must not become a way to satisfy this requirement by inspection. The default must be at or
  above a declared minimum, and the floor for the default must be verified at the default.

## Acceptance examples

### Example: normal behavior

**Given** the reference decision source at the default density of `0.75%`

**When** the simulation runs to tick `1,000` on each declared verification seed

**Then** at least eight Mokiterions are living at termination on every seed, and the run reports non-zero food
consumption.

### Example: failure behavior

**Given** the reference decision source at the default density of `0.75%`

**When** a declared seed leaves seven Mokiterions living at tick `1,000`

**Then** the requirement fails, and correction requires an amended specification rather than an implementation
adjustment, because both the density-to-resource mapping and the reference decision rules are specified.

### Example: boundary behavior at a higher, undeclared density

**Given** the reference decision source at a density of `1.50%`

**When** the simulation runs to tick `1,000` on each declared verification seed

**Then** the run is valid and no viability obligation applies. The measured counts are recorded as evidence, and a
result at or near twelve on every seed there is an observation about that density, not a failure of this
requirement.

### Example: boundary behavior

**Given** a density of `5.00%`, which is not a declared minimum

**When** the simulation runs to tick `1,000`

**Then** the run is valid and no viability obligation applies, because only the declared density carries a floor.

## Open decisions

None. The floor of eight at the default density of `0.75%` and the 1,000-tick window were decided by the
repository owner on 2026-08-17 from measured evidence. The verification seed set and the declared density set are
fixed by `VER-MOK-002`; the density-to-resource mapping and the rounding rule are fixed by `SPEC-MOK-001`.

One matter is deferred rather than open. The corrected rule 5 makes high-class resources consumable only at
satiety of at most `50`, so they accumulate against the capacity that density fixes, and a 10,000-tick run at the
default density reaches extinction where the previous rule left one survivor. No requirement here speaks past
tick `1,000`, so this does not bear on the stated floor. It is accepted knowingly, recorded in `VER-MOK-002` as
residual uncertainty and in the functional roadmap as Phase 2 scope, and any future long-horizon stability
requirement must address it.
