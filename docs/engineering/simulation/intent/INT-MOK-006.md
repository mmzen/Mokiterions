+++
id = "INT-MOK-006"
type = "intent"
title = "Make Mokiterions individually distinguishable"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
+++

# Intent: Make Mokiterions individually distinguishable

## Problem

**The twelve Mokiterions are interchangeable, and the engine is what makes them so.** `SPEC-MOK-001`'s state model
gives each one an identifier, a coordinate, a derived territory, three attributes and a living flag. Nothing else.
Both decision sources are pure functions of the observation, so two Mokiterions holding identical observations
propose identical actions, in every run, at every seed. The identifier decides who acts first and nothing else.

The concept the project is built on says the opposite. It asks for "individuality without reducing behavior to
randomness", and names `fear` as a fourth attribute that shapes reactions to threat, combat, retreat and surrender.
Neither exists. `SPEC-MOK-003` rule 4.5 already reserves trailing space in the observer's roster for `fear` by name,
and requires that space to render with no label, no dash and no zero, "because an inert `fear 0` would be a claim the
engine cannot support". The reservation has been correct and empty since Phase 1.5.

**Uniformity is not only a missing feature; it is the mechanism of a disclosed defect.** `SPEC-MOK-001` rule 5 makes
a resource eatable and approachable only while its satiety restoration would not be clipped, so a high-class
resource is wanted only at satiety of at most `50`. Every Mokiterion applies that identical threshold at the identical
moment, so the whole population declines the same third of the larder together. Measured at the default density,
high class reaches 45 of 61 standing resources in a territory by tick 1,000, against a balanced initial third, and a
10,000-tick run reaches extinction at tick 9,154 with territories full. `REQ-MOK-014` speaks only to tick 1,000, so
this is accepted rather than failing, and it is recorded as residual uncertainty in `VER-MOK-002` and as carried-in
scope in the functional roadmap. The population starves in front of food that no one will walk to, and the reason
no one will walk to it is that everyone is the same.

Two later phases are blocked behind this. Conflict needs `fear` before a Mokiterion can choose between fighting,
retreating and surrendering. A model-backed decision source needs the observation contract to stop changing, and
completing the attribute model now is what lets that contract be written once instead of twice.

`INT-MOK-002` made the world habitable and perceptible and was verified under `VREC-MOK-002`. It is achieved, and a
completed intent is not the place to record a new outcome.

## Desired outcomes

- Each Mokiterion carries a behavioral trait that is its own, fixed for the run, and derived from nothing but the
  seed and its identifier — so individuality is reproducible rather than random.
- Two Mokiterions in the same situation can make different choices, and the difference is attributable to a stated
  value rather than to a coin toss.
- `fear` exists as a fourth dynamic attribute that the engine computes, that is reported wherever the other three
  are, and that the observer's reserved roster slot finally holds.
- The operator can select the individual-aware decision source, compare it against the two existing sources at a
  matched seed and density, and see the comparison mean something because the existing sources did not move.
- The world remains at least as habitable under the new source as the product owner already required of the
  reference source.
- Phase 3 inherits a complete attribute model, and Phase 5 inherits an observation contract that has stopped
  changing.

## Actors and stakeholders

- The product owner decides whether individuality is worth a survivor cost, if measurement shows one, and owns the
  habitability floor the new source must meet.
- The technical owner owns the specification consequences: a fourth attribute, a per-agent derived entropy source,
  a third decision source, and the two narrow interface amendments they need.
- The assurance owner owns the central claim of this initiative, which is a claim of *absence*: that the two
  existing decision sources did not change.
- Developers and implementation agents implement the trait, the attribute and the third source.
- Operators gain one option value and one attribute in the output. Nothing they already run changes its meaning.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Mokiterions able to choose differently from another in an identical situation | 0 of 12 | 12 of 12 | Automated verification |
| Dynamic attributes the engine computes | 3 | 4 | Automated verification |
| Decision sources selectable by the operator | 2 | 3 | Every build |
| Survivor count, death count, final positions and standing food differing from the verified baseline under `--policy baseline` and `--policy reference` | 0 | 0 | Automated verification, declared seed set |
| Entropy draws taken from the shared stream by trait derivation | not applicable | 0 | Automated verification |
| Survivors at tick 1,000 under the new source at the default density | not applicable | ≥ 8 of 12 on every declared seed | Automated verification |
| High-class share of a territory's standing resources at tick 1,000 | 45 of 61 under `reference` | measured and recorded, no target | Automated verification |
| Roster bars the observer renders | 3 of a reserved 4 | 4 of 4 | Automated verification, in-memory buffer |
| Engine package external dependencies | 0 | 0 | Every build |
| New traits carrying no consumer | not applicable | 0 | Static check |

The third measure is the one that carries this initiative's risk, and it is stated as an absence deliberately. The
seventh is recorded rather than targeted: the accumulation effect is expected to fall, but committing to a number
before measuring it is the mistake `REQ-MOK-014` made twice.

## Non-goals

- **Per-agent entropy substreams.** The functional roadmap lists them in this phase and they are deliberately not
  pursued here. They change which stream every decision draw comes from, which moves regeneration placement, which
  invalidates `REQ-MOK-014`'s measured floor and forces its re-approval. What they buy is *stochastic* divergence,
  and this initiative's whole premise is that individuality should not be randomness. Deterministic traits deliver
  the outcome; substreams would deliver it again, at the price of re-approving a verified number. If they are wanted
  later they are a standalone initiative that pays its own re-measurement.
- **More than one trait.** `aggression` and `sociability` have no consumer until conflict exists. Shipping them now
  would ship exactly the inert value `SPEC-MOK-003` rule 4.5 prohibits for `fear`. They belong to the phase that
  reads them.
- **A behavioral consumer for `fear`.** Its purpose is reaction to threat, and there is no threat until conflict
  exists. This initiative computes it, bounds it and reports it; the phase that adds combat is the one that reads it.
- Combat, threatening, fleeing, cooperation, encounter resolution, or interaction history.
- Model-backed decisions, structured output, persistence, or a batch runner.
- **Any change to `--policy baseline` or `--policy reference`.** They are the scientific control. Altering either
  would destroy the comparison that later phases depend on.
- Any change to the world constants, the survival values, the resource table, the density mapping, the perception
  radius, regeneration, or the finality of death.
- Making the new source the default. That is a separate decision, after its floor is measured.
- Fixing high-class accumulation for the reference source. This initiative gives the new source a mechanism that
  addresses it; correcting rule 5 for the reference source would move the curve `REQ-MOK-014` is measured on and is
  a separate governed change.
- A fourth decision source, a new package, a new dependency, or any new component.

## Principles and immutable constraints

- The engine remains the only authority over world state. A trait is engine-owned data, and a decision source that
  reads it gains no more power than one that reads satiety.
- Determinism is absolute. Individuality must be reproducible from the seed, so the same seed and the same source
  produce the same twelve traits and the same run, byte for byte.
- **The control must survive.** The two existing sources are how any later claim about a smarter source is made
  meaningful. Their outcomes are frozen, and the freeze is demonstrated by comparison against verified output rather
  than asserted.
- No inert attribute. An attribute the engine does not compute is absent, not zero. `SPEC-MOK-003` rule 4.5 states
  this for `fear`, and this initiative honours it by computing `fear` rather than by displaying a placeholder.
- No trait without a consumer. A number the engine derives and nothing reads is not individuality.
- The public interface is a ceiling. It grows only where an approved requirement needs it, and this initiative needs
  it in exactly two places.
- Keep it small. One trait, one attribute, one decision source, two interface items.

## Risks and assumptions

- **Fact: this initiative cannot begin yet.** It amends `SPEC-MOK-002` and `SPEC-MOK-003`, and both carry
  amendments that are written but unapproved: `VREC-MOK-005` records six outstanding provisions across
  `SPEC-MOK-002`, `SPEC-MOK-003` and `ARCH-MOK-001`, `WO-MOK-005` is `in_progress`, and `VREC-MOK-005` is `ready`.
  Amending an artifact whose last amendment is unapproved is the failure Phase 0 existed to prevent. The gate is
  stated in the implementing work order.
- Fact: `SPEC-MOK-003` rule 4.5 reserved the roster's fourth bar for `fear` by name, so the observer needs no new
  requirement to present it. `REQ-MOK-020` already obliges presenting survival state.
- Fact: the reservation is currently notional. `VREC-MOK-005` finding 3 records that `bar_width` divides the
  interior by three, so the reserved slot is zero-wide at the reference roster — absent there rather than empty,
  which is not what rule 4.5 describes. Filling the slot is also what fixes it.
- Fact: perception already reports every living Mokiterion within radius `16` with a Chebyshev distance, so a fear
  rule driven by whether that list is empty needs no new perception pass, no new state and no entropy.
- Fact, and the reason the driver is perception itself rather than a tighter distance: with eleven other Mokiterions
  on a 128×128 grid, the expected number inside a Chebyshev box of radius `r` is `11 * (2r+1)^2 / 16384` — about
  `0.73` at the perception radius of `16`, about `0.19` at `8`, and about `0.05` at `4`. A narrow band would leave
  `fear` at its lower bound on roughly nineteen agent-ticks in twenty, which is a different way of shipping the inert
  value `SPEC-MOK-003` rule 4.5 refused.
- Assumption, and the property the whole design rests on: a trait derived by a pure function of the seed and the
  identifier consumes nothing from the shared entropy stream, so initialization performs the same draws in the same
  order and every existing run is unchanged. This is verified, not assumed, and its failure is a stop condition.
- Assumption: a trait expressed as tolerance for clipped restoration collapses to the reference source's own rule at
  tolerance zero, so the new source can be pinned against the control by construction rather than by argument.
- **Risk: individuality may cost survivors.** A Mokiterion that tolerates waste extracts less satiety per resource.
  Whether the population-level gain from draining the accumulated larder outweighs the per-meal loss is a measured
  question with a plausible negative answer. The mitigation is that the tolerance range is a specification constant:
  narrowing it is an amendment the technical owner can make on evidence, not a code change, and a floor miss
  escalates rather than being tuned away.
- Risk: `fear`'s rise and decay constants cannot be falsified by outcome while nothing consumes it. They are
  verifiable as bounded, reproducible and responsive to perception, which is what the roadmap asks of this phase,
  and no more is claimed for them.
- Risk: two existing event lines gain a field, so the output stream is not byte-identical to the previous build even
  where the outcome is. Every equivalence claim in this initiative is about survivors, deaths, positions and
  standing resources, never about output bytes, and the distinction must be stated wherever the claim is made.
- Assumption: the engine's dependency table stays empty and the observer's stays at one path dependency and
  `ratatui`. Nothing here needs either to change.
