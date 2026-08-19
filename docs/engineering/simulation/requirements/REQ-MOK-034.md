+++
id = "REQ-MOK-034"
type = "requirement"
title = "Sustain a viable population under the trait-aware decision source"
status = "draft"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "WHEN a simulation runs to 1,000 ticks using the trait-aware decision source at the default resource density, THE SYSTEM SHALL leave at least eight of the twelve Mokiterions living."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-006"]
+++

# Requirement: Sustain a viable population under the trait-aware decision source

## Rationale

Individuality is only worth having if the population survives it. A trait-aware source that diverged from the
reference source and starved would demonstrate divergence, which is not the outcome `INT-MOK-006` asks for.

The obligation belongs to a requirement of its own rather than to `REQ-MOK-014`, because `REQ-MOK-014` is stated over
the *reference* source and is measured on a curve that must not move. Extending it to cover a second source would
place both floors inside one artifact and make either one's re-approval touch the other. A separate requirement keeps
the control's verified number where it is.

## The stated floor

| Decision source | Density | Resources per territory | Window | Survivor floor |
|---|---:|---:|---:|---:|
| Trait-aware | `0.75%`, the default | 61 | 1,000 ticks | 8 of 12 |

**The number is the one `REQ-MOK-014` already carries, and reusing it is the substance of this requirement rather
than a convenience.** It says that individuality must not cost habitability: the world with twelve distinct
Mokiterions must be at least as survivable as the world with twelve identical ones, measured the same way, at the
same density, over the same window, on the same declared seeds. Choosing a different number would smuggle in a
product judgement about how much survival individuality may cost, and that judgement should be taken explicitly on
evidence, not written into a draft in advance.

One density is declared, and it is the default, for the reason `REQ-MOK-014` gives: a floor at a comfortable density
would document headroom rather than habitability. Headroom under this source is recorded as evidence, not as an
obligation.

## Preconditions and trigger

A simulation is started with the trait-aware decision source at the default density of `0.75%` and runs to a tick
limit of at least `1,000`.

## Required response

At tick `1,000`, at least eight Mokiterions are living. The obligation holds on every seed in the declared
verification seed set — `0`, `1`, `42`, `123` and `777` — not merely on a favourable seed.

## Failure and boundary behavior

- Fewer than eight survivors on any declared seed fails this requirement.
- Twelve survivors on every declared seed satisfies the literal floor and is an adverse observation, not a success,
  for the reason `REQ-MOK-014` states: it would indicate scarcity has been removed, contradicting `INT-MOK-002`'s
  scarcity principle. The verification contract reports it for product review.
- **The obligation must not be read as monotonic in density**, and it must not be read as transferable between
  sources. Each density is a separate world, because density changes how many coordinate draws initialization
  performs; and each source consumes the shared entropy stream at its own rate, so a floor met under one source says
  nothing about another. A row here can be added only by measurement on a full seed sweep, never by interpolation
  from `REQ-MOK-014`'s row or from a neighbouring density.
- Densities other than the default, tick limits other than `1,000`, and seeds outside the declared set carry no
  viability obligation under this source. They remain valid inputs and are worth exploring; they are unverified.
- This requirement states a survivor floor, not a prohibition on death, and not a claim about any horizon past tick
  `1,000`.
- A floor met at the trait range's lower bound alone would not satisfy this requirement. The measurement is of the
  population the seed actually produces, whose traits are distributed across the range; a source verified only where
  it collapses to the reference source has verified the reference source.

## Constraints

- Viability must be achieved by the decision policy, not by weakening death or enriching the world. The health,
  satiety and energy bounds, decay-driven health loss, the finality of death, the resource table, the density mapping
  and conditional regeneration are all unchanged, and `REQ-MOK-014`'s constraints apply here in full.
- Viability must not be achieved by narrowing the trait range until individuality disappears. A range collapsed
  toward the lower bound would satisfy this floor by reproducing the reference source and would defeat
  `REQ-MOK-033`. Narrowing the range on evidence is a legitimate specification amendment; narrowing it until the
  divergence evidence `VER-MOK-007` requires can no longer be produced is not.
- Determinism is preserved, so every measurement here is reproducible rather than statistical.
- Meeting this floor must not be achieved by any change to the reference or baseline source. Their outcomes are frozen
  under `REQ-MOK-033`.

## Acceptance examples

### Example: normal behavior

**Given** the trait-aware decision source at the default density of `0.75%`

**When** the simulation runs to tick `1,000` on each declared verification seed

**Then** at least eight Mokiterions are living at termination on every seed, and the run reports non-zero food
consumption.

### Example: failure behavior

**Given** the trait-aware decision source at the default density of `0.75%`

**When** a declared seed leaves seven Mokiterions living at tick `1,000`

**Then** the requirement fails. Correction requires either an amended `SPEC-MOK-001` — the tolerance range and the
tolerance test are specified, not implementation choices — or an amended floor approved by the product owner on the
measured evidence. It does not permit an implementation adjustment, and it does not permit changing the reference
source.

### Example: boundary behavior at an undeclared density

**Given** the trait-aware decision source at a density of `1.50%`

**When** the simulation runs to tick `1,000` on each declared verification seed

**Then** the run is valid and no viability obligation applies. The counts are recorded as evidence.

## Open decisions

None once approved: approving this requirement *is* the decision that individuality must not cost habitability at the
default density over the 1,000-tick window.

The product owner took that decision on 2026-08-19: the floor stays at eight, because individuality must not cost
habitability. The alternative it was taken against is recorded here rather than discarded, because a floor miss
returns to this same choice on measured evidence. That alternative is a lower floor with the survivor cost recorded as
an accepted price for individuality. Taking the decision now was far cheaper than taking it later — `REQ-MOK-014`'s
amendment record shows two floors approved from a curve measured before the change that invalidated them, and both
failed — and nothing here is measured yet.

Two considerations bound the risk of holding the number. The floor of eight is known to be reachable by this source at
one end of its trait range, because `REQ-MOK-033` makes it action-for-action identical to the reference source at the
range's lower bound, so the open question is whether the range's interior degrades the outcome rather than whether the
target is attainable at all. And a miss is a stop-and-escalate condition in the implementing work order, so it returns
here as a product decision on measured evidence instead of being tuned away in the specification.

One matter is deferred rather than open. `SPEC-MOK-001` rule 5 records that high-class resources accumulate to 45 of
61 standing resources in a territory by tick 1,000 under the reference source, and that a 10,000-tick run reaches
extinction. A per-Mokiterion tolerance is expected to reduce that accumulation, and `VER-MOK-007` measures it at both
horizons. No obligation is stated on the result, in either direction, because no long-horizon stability requirement
exists yet and inventing a target before the first measurement is the error this requirement's sibling made twice.
