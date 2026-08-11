# Mokiterions

**Mokiterions is an autonomous artificial-life simulation where 12 LLM-driven agents inhabit a shared 128×128 world, competing for food, energy, and survival. Individual traits, resource scarcity, and controlled entropy drive emergent behaviors such as cooperation, migration, conflict, surrender, and extinction without hard-coded strategies.**

The project is implemented in **Rust** and serves as a proof of concept for [SE Harness](https://github.com/mmzen/se_harness), a repository-native software-engineering harness used to govern the project from intent and requirements through implementation, verification evidence, and exact Git commits.

## Why this project exists

Mokiterions has two objectives.

### 1. Build an autonomous multi-agent simulation

The simulation explores whether simple survival constraints, individual differences, scarce resources, and LLM-based decision-making can produce meaningful emergent behavior.

The world does not explicitly script strategies such as cooperation, migration, resource conservation, or war. Mokiterions perceive their situation and decide how to act within deterministic world rules.

### 2. Evaluate agentic software engineering

Mokiterions is also an engineering experiment.

The repository is developed under **SE Harness** to evaluate whether a coding agent can implement a non-trivial stateful system while remaining governed by explicit intent, requirements, specifications, architecture, authorized work, verification, evidence, and human approval.

The expected engineering chain is:

```text
Intent
  ↓
Capabilities
  ↓
Requirements
  ↓
Specifications / Architecture
  ↓
Authorized Work
  ↓
Implementation
  ↓
Verification
  ↓
Evidence
  ↓
Exact Git Commit
  ↓
Human Release Decision
```

The objective is not only to make the simulation work, but to make material changes **traceable, explainable, verifiable, and evidence-backed**.

## The world

The Mokiterions live on a **128 × 128** two-dimensional virtual grid.

The grid is divided into two equal territories. The initial population consists of **12 Mokiterions**, with six starting in each territory.

```text
┌─────────────────────────────────┐
│           Territory A           │
│          6 Mokiterions          │
├─────────────────────────────────┤
│           Territory B           │
│          6 Mokiterions          │
└─────────────────────────────────┘
```

Territories establish the initial population groups, but they do not prescribe permanent alliances or hostility.

## Mokiterions

Each Mokiterion is an autonomous agent whose fundamental objective is:

> **Survive.**

A Mokiterion has a dynamic internal state including:

| Attribute | Purpose |
|---|---|
| `health` | Physical condition. The Mokiterion dies when health reaches zero. |
| `satiety` | Hunger state. Food restores satiety. |
| `energy` | Ability to act. Food and sleep restore energy. |
| `fear` | Influences reactions to threats, combat, retreat, and surrender. |

Mokiterions are deliberately not identical.

A small **entropy component** introduces controlled behavioral variation so that agents facing similar situations do not necessarily make identical decisions.

The intent is to create individuality without reducing behavior to randomness.

## Resources

The world contains three types of food:

- low-calorie food;
- medium-calorie food;
- high-calorie food.

Eating consumes the resource and restores satiety and energy according to its calorific value.

Food resources can respawn after a delay, but only under one important condition:

> **Food may respawn in a territory only if at least one food resource still exists in that territory.**

If all food in one territory is consumed before regeneration occurs, that territory can therefore lose its ability to regenerate food.

This creates the possibility of sustainable consumption, overconsumption, scarcity, famine, migration, competition for resources, raids into the opposing territory, and conflict. These outcomes are consequences of the world state and agent decisions rather than predefined scenarios.

## Survival mechanics

Satiety and energy decrease over time.

If either becomes critically low for long enough, health begins to decrease.

```text
Satiety ↓
Energy  ↓
    │
    ├── sufficient ───────────────► normal survival
    │
    └── critically low
             │
             ▼
          Health ↓
             │
             ▼
          Health = 0
             │
             ▼
            Death
```

To survive, Mokiterions must balance movement, exploration, food acquisition, eating, sleeping, danger avoidance, social interaction, territory crossing, and conflict.

## Autonomous decision-making

Each Mokiterion is backed by an **AI agent using a Large Language Model**.

At a decision point, the agent receives a bounded representation of the Mokiterion's current situation, potentially including:

- health;
- satiety;
- energy;
- fear;
- position;
- nearby resources;
- nearby Mokiterions;
- territory;
- recent events;
- remembered observations;
- threats;
- previous interactions.

The agent then selects an allowed action.

Example actions include:

```text
move
explore
eat
sleep
wait
approach
avoid
cross territory
threaten
attack
fight
retreat
surrender
```

A key design principle is the separation between **decision-making** and **world authority**:

```text
LLM Agent
   │
   └── selects an intended action
              │
              ▼
      Simulation Engine
              │
              ├── validates the action
              ├── applies world rules
              ├── updates authoritative state
              └── records the result
```

The LLM can decide what a Mokiterion attempts to do. It cannot directly modify the world, create resources, change attributes, teleport, or bypass simulation rules.

## Scarcity and conflict

Resource scarcity may change the behavior of Mokiterions and the relationship between the two populations.

A Mokiterion facing starvation might decide to explore more aggressively, conserve energy, cross into the other territory, take resources used by another population, retreat from competition, threaten another Mokiterion, or attack.

The simulation must **not** contain rules such as:

```text
if food < threshold:
    start_war()
```

War, cooperation, migration, or coexistence should emerge from individual decisions.

## Combat

When attacked, a Mokiterion can respond by:

- fighting back;
- retreating;
- surrendering.

Fear is an important input, but not necessarily the only one.

The decision can also be influenced by current health, energy, perceived relative strength, nearby Mokiterions, previous interactions, scarcity, and individual entropy.

Combat consumes energy and may reduce health. Death occurs when health reaches zero.

## Emergent behavior

The system is intentionally designed to make outcomes possible without making them mandatory.

Potential emergent behaviors include:

- sustainable resource consumption;
- uncontrolled consumption followed by famine;
- exploration and migration;
- territorial behavior;
- cooperation and avoidance;
- implicit alliances;
- resource raids;
- aggression and retaliation;
- surrender;
- war;
- asymmetric population collapse;
- coexistence;
- extinction.

A successful simulation does not require any particular behavior to occur.

Its purpose is to create conditions in which those behaviors **can emerge from autonomous decisions**.

## Observability

The simulation should make important behavior reconstructable.

Useful agent-level information includes state over time, position, perceptions, chosen actions, decision rationale where available, food consumption, sleeping, territory crossings, encounters, combat, health changes, and death.

Useful world-level information includes surviving population, population per territory, available food, food consumption and regeneration, average health, average satiety, average energy, territory crossings, conflict frequency, deaths, and survival time.

Observability serves both the simulation itself and the engineering-assurance goals of the project.

## Engineering with SE Harness

Mokiterions is governed through **SE Harness**.

Rather than treating code as the starting point, development begins with explicit engineering artifacts describing why a change exists, what behavior is expected, how the system is constrained, and how the implementation will be verified.

For a material change, the intended lineage is conceptually:

```text
Intent
  └── Requirement
        ├── Specification
        ├── Architecture
        ├── Verification contract
        └── Authorized work
                 │
                 ▼
           Implementation
                 │
                 ▼
              Evidence
                 │
                 ▼
        Verification record
                 │
                 ▼
           Exact Git commit
```

For example:

```text
Requirement
Food must disappear when consumed.
        │
        ▼
Specification
Consumption atomically removes the selected
food resource from the authoritative grid state.
        │
        ▼
Implementation
Rust source code implementing resource consumption.
        │
        ▼
Verification
Unit and simulation tests exercising consumption.
        │
        ▼
Evidence
Retained test results and traceability information.
        │
        ▼
Verification Record
Evidence bound to the exact candidate Git commit.
```

The proof of concept therefore tests two things simultaneously:

1. whether the autonomous simulation behaves correctly;
2. whether an agentic engineering workflow can demonstrate **why the delivered implementation should be trusted**.

Approval and release authority remain human responsibilities.

## Technology

The initial implementation uses:

- **Rust** for the simulation and application code;
- **LLM-backed agents** for Mokiterion decision-making;
- **SE Harness** for specification-driven engineering governance, verification, traceability, and evidence;
- **Git** as the authoritative source and commit lineage.

Additional technical choices will be captured through the project's engineering specifications and architecture decisions rather than defined prematurely in this README.

## Initial scope

The first milestone targets the smallest complete simulation capable of meaningful autonomous behavior:

- [ ] 128 × 128 world;
- [ ] two territories;
- [ ] 12 Mokiterions;
- [ ] three food-resource types;
- [ ] food consumption;
- [ ] conditional resource regeneration;
- [ ] health, satiety, energy, and fear;
- [ ] behavioral entropy;
- [ ] movement and exploration;
- [ ] eating;
- [ ] sleeping;
- [ ] LLM-based decisions;
- [ ] territory crossing;
- [ ] attack, fight, retreat, and surrender;
- [ ] death;
- [ ] simulation history;
- [ ] basic observability;
- [ ] SE Harness traceability from requirements to verified implementation evidence.

## Non-goals for the first version

The initial version is not intended to implement reproduction, genetics, complex economies, crafting, technology trees, governments, explicit diplomacy systems, sophisticated social organizations, or scripted narratives.

These may be explored later if the core simulation provides a useful foundation.

## Success criteria

The first version is successful when **12 autonomous Mokiterions can inhabit the world over an extended simulation, perceive their situation, independently make survival-oriented decisions, interact with resources and one another, and produce observable outcomes that are not predetermined by the simulation engine.**

From an engineering perspective, the project is successful when SE Harness can trace material behavior from **intent → requirements → specifications and architecture → authorized work → implementation → verification evidence → exact Git commit**, providing a practical proof of concept for governed agentic software engineering.
