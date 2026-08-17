+++
id = "CAP-MOK-001"
type = "capability"
title = "Run and observe a basic Mokiterions simulation"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-11"

[relations]
derives_from = ["INT-MOK-001"]
+++

# Capability: Run and observe a basic Mokiterions simulation

## Actor and need

An operator needs to run the simulation locally and observe whether its foundational world and survival rules behave consistently before external model decisions and advanced interactions are introduced.

## Capability statement

`An operator can run a bounded Mokiterions survival simulation locally, using a deterministic baseline decision source, and observe its progress and final state through text output.`

## Boundaries

- The capability covers one in-memory simulation process.
- It uses a local seeded decision baseline rather than an external LLM.
- It covers movement, eating, sleeping, waiting, survival decay, death, food regeneration, territory crossing, and text observation.
- It excludes combat, social behavior, persistence, and graphical interfaces.

## Outcomes

- The simulation can be exercised without network access or secrets.
- Core rules are independently testable and reproducible.
- A later model integration can propose actions without obtaining state-mutation authority.

## Candidate requirements

- `REQ-MOK-001` Initialize the world.
- `REQ-MOK-002` Initialize Mokiterions.
- `REQ-MOK-003` Advance simulation time.
- `REQ-MOK-004` Enforce world authority.
- `REQ-MOK-005` Apply core actions.
- `REQ-MOK-006` Consume food.
- `REQ-MOK-007` Regenerate food conditionally.
- `REQ-MOK-008` Provide a baseline decision source.
- `REQ-MOK-009` Produce reproducible entropy.
- `REQ-MOK-010` Emit text observations.
- `REQ-MOK-011` Terminate cleanly.
- `REQ-MOK-012` Trace every action optionally.
