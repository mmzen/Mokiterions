+++
id = "INT-MOK-002"
type = "intent"
title = "Make the world habitable and perceptible"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
+++

# Intent: Make the world habitable and perceptible

## Problem

The verified foundation runs, but its world cannot support the population it contains, and its Mokiterions
cannot perceive anything beyond the cell they occupy.

Two independent facts establish this. First, population satiety demand is `2` per tick for each of twelve
Mokiterions, or `24` per tick, while conditional regeneration supplies one resource per territory every ten
ticks, or approximately `6.3` satiety per tick against a mean resource value of `31.67`. Supply falls short of
demand by roughly `3.8` times, which implies a steady-state carrying capacity near three of twelve. Second, an
observation exposes only co-located food, so no decision source can locate a resource it is not already
standing on.

The consequences are not hypothetical. A default run ends in extinction at tick `69` having consumed no food
at all, and this outcome is a property of the specified world rather than of the baseline decision source. No
decision source of any sophistication changes it. Adding an external model to this world would produce mass
starvation and would demonstrate nothing about autonomous behavior.

The foundation intent `INT-MOK-001` is achieved and its purpose no longer covers this work. Its non-goals do
not exclude perception or resource balance; the reason for a distinct intent is that establishing an
executable foundation and making that foundation habitable are different outcomes with different measures.

## Desired outcomes

- A Mokiterion perceives nearby resources and nearby Mokiterions and can therefore act on its surroundings.
- The world sustains most of its population over a long run instead of starving all of it.
- Scarcity remains a real constraint, so survival requires acquiring resources rather than being granted them.
- An operator running the program with no arguments observes a living population rather than certain extinction.
- A competent non-model decision source demonstrates that the world is survivable, establishing a reference
  against which later model-driven behavior can be compared.
- Determinism, engine authority, and credential-free local execution are preserved exactly as verified.

## Actors and stakeholders

- The product owner sets the population viability target and accepts observable world behavior.
- Developers implement perception, the resource economy, and the reference decision source.
- Operators run local experiments and observe whether the population persists.
- Assurance reviewers confirm that viability is measured rather than asserted.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Living Mokiterions after 1,000 ticks under the reference policy | 0 | ≥ 8 of 12 | Automated verification |
| Resources consumed during a default run | 0 | > 0 | Automated verification |
| Observations exposing resources beyond the occupied cell | 0% | 100% | Automated verification |
| Repeated runs at an identical seed producing identical results | 100% | 100% | Automated verification |
| Network services or credentials required to run the simulation | 0 | 0 | Every local run |

## Non-goals

- OpenAI or other model-provider integration, prompts, or agent memory.
- Fear, combat, threats, retreat, surrender, or social relationships.
- Persistence, structured or machine-readable output, and aggregate multi-run analysis.
- Reproduction, genetics, economies, crafting, or scripted narratives.
- A graphical or web user interface.
- Deliberate strategy such as cooperation, migration, or raiding. The reference decision source exists to
  prove the world is survivable, not to model intelligent behavior.

## Principles and immutable constraints

- The simulation engine remains the only authority over world state.
- Perception is read-only. A decision source receives values and never a handle to mutable state.
- Every stochastic result continues to derive from the explicit seeded entropy stream.
- Scarcity is preserved. Sufficient abundance to make resource acquisition unnecessary is a failure, not a
  success.
- Keep the implementation small and direct, and add no abstraction that an approved requirement does not need.
- The reference decision source must be identifiable as a development instrument and never presented as
  autonomous behavior.

## Risks and assumptions

- Fact: the specified resource economy supports approximately three of twelve Mokiterions, and the default run
  ends in extinction at tick `69` with zero consumption.
- Fact: travel is not free. At `2` satiety per tick and one cell of movement per tick, a medium resource funds
  roughly fifteen cells of travel, so seeking distant food currently costs more than it returns.
- Assumption: reducing satiety decay and increasing regeneration yield is sufficient to reach the viability
  target without eliminating scarcity.
- Risk: effective carrying capacity is lower than the analytical figure because agents lose satiety while
  travelling and never find resources that spawn outside perception. Final constants therefore require
  measurement, and a measured result outside the intended band is an escalation rather than a silent retune.
- Risk: tuning far enough to guarantee survival could remove competitive pressure and suppress the emergent
  behavior the project exists to observe.
- Open decision: none. The viability target, perception scope, default decision source, and specification
  strategy were decided by the repository owner on 2026-08-17.
