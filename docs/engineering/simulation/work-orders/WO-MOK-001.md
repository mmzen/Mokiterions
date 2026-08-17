+++
id = "WO-MOK-001"
type = "work_order"
title = "Implement the minimum simulation foundation"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-11"
updated = "2026-08-11"

[relations]
implements = [
  "REQ-MOK-001",
  "REQ-MOK-002",
  "REQ-MOK-003",
  "REQ-MOK-004",
  "REQ-MOK-005",
  "REQ-MOK-006",
  "REQ-MOK-007",
  "REQ-MOK-008",
  "REQ-MOK-009",
  "REQ-MOK-010",
  "REQ-MOK-011",
  "REQ-MOK-012",
]
specifications = ["SPEC-MOK-001"]
architecture = ["ARCH-MOK-001", "ADR-MOK-001"]
verification = ["VER-MOK-001"]
+++

# Work Order: Implement the minimum simulation foundation

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the completed change and retained evidence. Verification and release require separate commit-bound records.

## Objective

Replace the placeholder Rust program with the smallest complete local implementation of `SPEC-MOK-001`, producing a deterministic, text-only, engine-authoritative Mokiterions foundation covered by `VER-MOK-001`.

## In scope

- Parse the specified seed, tick-limit, and help command-line inputs.
- Implement the 128 by 128 world and two territories.
- Initialize twelve Mokiterions and six food resources from seeded entropy.
- Implement bounded survival state, tick ordering, and death.
- Implement validation and the `move`, `eat`, `sleep`, and `wait` actions.
- Implement conditional food regeneration.
- Implement the local seeded baseline decision source.
- Emit deterministic text events and final summaries.
- Implement the optional `--trace-actions` output with one detailed line per living-agent decision opportunity.
- Add automated tests covering every requirement and verification invariant.
- Retain verification evidence under this work-order ID.

## Out of scope

- OpenAI API or GPT nano integration.
- Prompts, agent memory, rationales, or model-provider credentials.
- Fear, combat, threats, retreat, surrender, or social behavior.
- Graphical or web UI, persistence, networking, async runtimes, databases, or services.
- Reproduction, genetics, economies, crafting, or scripted narratives.
- Changes to approved behavior or artifact lifecycle outside this work order.

## Authorized decision envelope

The implementation agent may choose:

- private Rust type, function, module, and file names;
- standard-library collection types that preserve specified observable ordering;
- internal error types and exact diagnostic wording;
- test module organization, fixtures, and helper functions;
- comments and non-authoritative developer documentation.

The implementation agent may not change specified constants, action semantics, event fields, exit codes, trust boundaries, requirement scope, or lifecycle status. No external dependency may be added without escalation and an updated architectural decision.

## Constraints

- Follow `INT-MOK-001`, `SPEC-MOK-001`, `ARCH-MOK-001`, `ADR-MOK-001`, and `VER-MOK-001`.
- Keep one Rust binary crate and prefer the standard library.
- Use no network, model, UI, persistence, async-runtime, or database dependency.
- Keep decision sources unable to mutate authoritative state.
- Make every stochastic result derive from the specified entropy stream.
- Preserve unrelated user changes and do not modify generated output as source.

## Expected change surface

- Rust application entry point and a small number of simulation and decision modules under `src/`.
- `Cargo.toml` and `Cargo.lock` only when required by the implementation; external runtime dependencies are not expected.
- Unit or integration tests within the Rust crate.
- Work-order-keyed evidence under `docs/engineering/simulation/evidence/WO-MOK-001/`.
- No other product domain or harness-managed policy file.

## Required verification

- Complete every case and review in `VER-MOK-001`.
- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --all-targets --all-features -- -D warnings`.
- Run `cargo test`.
- Run `cargo build`.
- Capture two identical seeded runs and demonstrate byte-identical output.
- Verify traced and untraced runs have identical authoritative final state and that trace counts match decision opportunities.
- Capture one 10,000-tick resilience run and one 20-tick manual-observation run.

## Evidence to record

Retain, under `docs/engineering/simulation/evidence/WO-MOK-001/`:

- formatter, linter, test, and build output;
- the requirement-to-test mapping;
- deterministic replay outputs or their hashes and comparison result;
- optional action-trace on/off comparison and trace-count result;
- the long-run resilience result;
- dependency and architecture-conformance review;
- security and credential review;
- manual text-output assessment;
- a completion summary identifying the final affected components.

## Stop and escalate conditions

Stop before implementation if preflight does not report this exact work order as eligible or if any governing artifact remains incomplete or unapproved. During implementation, stop and escalate if:

- specified behavior is contradictory or cannot be implemented without a product decision;
- an external dependency appears necessary;
- the decision source would need mutable world access;
- deterministic output cannot be preserved;
- required verification fails and cannot be corrected within authorized scope;
- requested changes expand into an excluded feature or another artifact domain.

## Completion report format

Report:

1. implemented requirements and affected components;
2. any authorized local decisions;
3. verification commands and results;
4. retained evidence paths;
5. residual limitations and explicitly deferred features;
6. final worktree and candidate-commit status.
