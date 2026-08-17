+++
id = "INT-MOK-001"
type = "intent"
title = "Establish the minimum simulation foundation"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"

[relations]
+++

# Intent: Establish the minimum simulation foundation

## Problem

Mokiterions has no executable simulation foundation. The world, survival rules, resources, decisions, and observations cannot yet be exercised or verified independently of an external model provider.

## Desired outcomes

- A Rust program runs a bounded Mokiterions simulation from initialization through termination.
- Twelve Mokiterions inhabit the specified world and interact with basic survival mechanics.
- The simulation engine remains the sole authority over world state.
- Identical configuration and entropy seed produce reproducible runs.
- Operators can reconstruct material activity from simple text output.
- A later decision source can use OpenAI GPT nano without changing authoritative simulation rules.

## Actors and stakeholders

- The product owner defines the intended simulation outcomes and accepts product behavior.
- Developers implement and maintain the Rust simulation.
- Operators run local experiments and inspect their results.
- Assurance reviewers assess requirement coverage and retained evidence.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Valid simulations that start and terminate without panic | 0% | 100% | Every automated and manual run |
| Repeated runs with identical configuration and seed that produce identical results | Not available | 100% | Automated verification |
| Material state mutations performed through validated engine operations | Not available | 100% | Automated verification and review |
| Approved requirements covered by automated tests | 0% | 100% | Before the first foundation release |
| Network services or API credentials required to run the foundation | Not applicable | 0 | Every local run |

## Non-goals

- OpenAI API integration, prompts, model memory, or model-provider operations.
- Combat, threats, surrender, fear-driven behavior, or social relationships.
- Sophisticated migration, cooperation, or conflict strategies.
- A graphical or web user interface.
- Persistence, reproduction, genetics, economies, crafting, or scripted narratives.

## Principles and immutable constraints

- Keep the implementation small, direct, and easy to inspect.
- Decision sources propose actions; only the simulation engine validates and applies them.
- Invalid actions never partially mutate authoritative state.
- Controlled entropy remains reproducible from an explicit seed.
- Text output is sufficient for this stage.
- Do not introduce abstractions solely for hypothetical future needs.

## Risks and assumptions

- Fact: the product concept calls for a 128 by 128 world, two territories, twelve Mokiterions, three food classes, survival state, conditional food regeneration, and observable behavior.
- Assumption: an engine-first foundation should be verified before adding an external LLM decision source.
- Risk: a temporary baseline decision source could be mistaken for final autonomous behavior; output and documentation must identify it as a development baseline.
- Open decision: numeric survival values and timing defaults must be fixed in specifications before the related requirements are approved.
- Approval decision: the repository owner validated this governing chain on 2026-08-11.
