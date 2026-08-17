+++
id = "CAP-MOK-002"
type = "capability"
title = "Observe a surviving population in a perceptible world"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
derives_from = ["INT-MOK-002"]
+++

# Capability: Observe a surviving population in a perceptible world

## Actor and need

An operator needs to observe a Mokiterions population that persists over a long run, driven by agents that can
see and reach the resources around them. Without this, the operator can only observe how quickly a blind
population starves, which reveals nothing about behavior.

## Capability statement

`An operator can run a long bounded simulation in which Mokiterions perceive nearby resources and nearby
Mokiterions, acquire food deliberately, and sustain most of the population, using a deterministic reference
decision source and no network access.`

## Boundaries

- The capability extends `CAP-MOK-001` rather than replacing it. Movement, eating, sleeping, waiting, survival
  decay, death, conditional regeneration, territory crossing, and text observation remain as verified.
- It adds bounded perception of nearby resources and nearby Mokiterions to the observation.
- It adds a selectable deterministic reference decision source that seeks and consumes food.
- It adds a measurable population viability obligation for the default configuration.
- It excludes model-provider integration, fear, combat, social behavior, persistence, structured output,
  aggregate multi-run analysis, and graphical interfaces.
- Perception reports nearby Mokiterions, but no requirement in this capability consumes that information. It
  exists so that the observation contract is settled once.

## Outcomes

- An operator running the program with no arguments observes a living population rather than certain extinction.
- Resource acquisition becomes a deliberate act rather than an accident of position.
- World viability becomes a measured property with a stated floor rather than an untested assumption.
- The reference decision source provides a control for comparing later model-driven behavior.
- Determinism, engine authority, and credential-free execution remain intact.

## Candidate requirements

- `REQ-MOK-013` Perceive the local surroundings.
- `REQ-MOK-014` Sustain a viable population.
- `REQ-MOK-015` Provide a reference decision source.
