+++
id = "ADR-MOK-001"
type = "adr"
title = "Engine-authoritative in-process decision boundary"
status = "approved"
owners = ["technical owner"]
created = "2026-08-11"
updated = "2026-08-20"

[relations]
decides = ["ARCH-MOK-001"]
+++

# ADR: Engine-authoritative in-process decision boundary

## Status

Accepted. Amended in place 2026-08-17 on one structural point, as recorded in the *Decision* section below and
required by `ADR-MOK-002`. The substantive decision — option 3, engine authority, and the immutable observation and
typed proposal boundary — is unchanged, so this ADR is not superseded.

**Note dated 2026-08-20. What supports one of this ADR's assertions has changed; the assertion has not.** This ADR
states that the foundation requires no network access and no credentials, and that testing simulation mechanics needs
neither. Until now that rested on the engine package having no dependencies at all: an empty table made the claim
checkable by reading one line. `ADR-MOK-006` withdrew that rule and admits third-party crates in both packages against
a declared set. **The support is withdrawn and the assertion is not.** It now rests on two things instead: decision 4
of `ADR-MOK-006`, which preserves every prohibition in this ADR's trust envelope without relaxation — no network, no
API credentials, no secret in the repository, no asynchronous runtime, no database, no plugin system, no
dependency-injection container, and no model provider in either package — and the two `ARCH-MOK-001` conformance checks
that replace the empty-graph inference: the resolved graph equals the declared set, and a by-name scan confirms no
crate of a prohibited capability class is in it.

No decision, no option, no consequence and no driver of this ADR changes, and it is **not superseded**. The one thing a
reader should carry away is that the trust envelope is now defended by a check rather than implied by an empty
manifest, and `ADR-MOK-006`'s *Negative* consequences say plainly that those are not the same thing.

## Context

Mokiterions must combine autonomous decision sources with deterministic world rules. The foundation uses a local seeded baseline, while a later stage is expected to use OpenAI GPT nano. A decision source may be unreliable, malformed, unavailable, or influenced by information that does not match current authoritative state.

The first stage also follows KISS: one local Rust process, text output, no UI, and no external runtime dependency. The architecture therefore needs a boundary that protects world authority now without introducing a distributed system or model-specific design.

## Decision drivers

- Preserve one authoritative and reconstructable world state.
- Prevent current and future decision sources from bypassing simulation rules.
- Keep identical seeded foundation runs reproducible.
- Test simulation mechanics without network access, credentials, or an LLM.
- Permit later OpenAI integration without rewriting core rules.
- Keep the first implementation small and understandable.
- Make invalid or stale action proposals safe and observable.

## Considered options

### Option 1: Allow each decision source to mutate its Mokiterion and nearby world state

This is direct and initially requires little ceremony, but it distributes world authority, makes conflicts and ordering difficult to reason about, allows model output to bypass rules, and weakens deterministic verification.

### Option 2: Place decisions and world mutation in separate services

This creates a strong deployment boundary and could scale independently, but it introduces networking, serialization, retries, consistency questions, credentials, and operational complexity before the simulation foundation is proven.

### Option 3: Use one in-process authoritative engine with an explicit proposal boundary

The engine owns all mutable state. A decision source receives an immutable bounded observation and returns a proposed action. The engine validates the proposal against current state and alone applies accepted mutations. The foundation baseline and later OpenAI adapter use the same conceptual boundary.

## Decision

Adopt option 3.

- Run the foundation as one Rust process and one Cargo package, built as a library target and a thin binary target.
  *(Amended in place 2026-08-17 under `ADR-MOK-002`, which narrows "one binary crate" to this. The library and
  binary targets of the single package are not separate crates in the sense prohibited by `ARCH-MOK-001`, and the
  library target's public interface is a closed read-only enumeration owned by `SPEC-MOK-002` rule 5. Engine
  authority, and the prohibition on exposing mutable state to a decision source, are unchanged and now apply to a
  surface that is actually reachable.)*
- Give the simulation engine exclusive ownership of mutable world, agent, food, time, and entropy state.
- Expose copied or immutable observations to decision sources.
- Accept only typed proposed actions: wait, sleep, eat a selected resource, or move one cardinal cell.
- Validate every proposal, including proposals from the bundled baseline.
- Apply accepted actions exactly once through engine operations.
- Treat rejected proposals as consumed action opportunities with no action-specific mutation.
- Keep action tracing observational; enabling it must not change decisions, entropy, or state.
- Implement the foundation with a local seeded baseline and no network dependency.
- Integrate a future OpenAI decision source as an adapter to the same boundary rather than granting it engine access.

## Consequences

### Positive

- Simulation rules remain centralized, deterministic, and independently testable.
- Invalid, stale, or malformed decisions cannot partially mutate the world.
- The engine can be developed and verified before model integration.
- A future model provider can be replaced without changing rule authority.
- Optional action traces can show proposals and results at a precise boundary.

### Negative

- Observation and proposed-action types become maintained contracts.
- Every new action requires coordinated observation, validation, application, and verification changes.
- A later network-backed model adapter must translate provider responses into typed proposals and handle latency and failure outside the engine's mutation path.
- One process does not independently scale decision execution; scaling is not a first-stage objective.

### Operational and security

- The foundation requires no network or credentials.
- Future provider credentials must remain outside the engine and repository.
- Model output is untrusted input and must pass the same validation as the local baseline.

### Migration

If later requirements justify remote or parallel decision execution, preserve the observation and proposal semantics while introducing transport outside the authoritative engine. Replacing engine authority itself requires a superseding ADR because it changes determinism, consistency, security, and verification assumptions.

## Validation

- Architecture review confirms no decision source receives a mutable world, agent, resource collection, event log, or engine handle.
- Automated tests demonstrate that invalid and stale proposals cause no partial action-specific mutation.
- Deterministic replay tests demonstrate that the local baseline and optional action tracing do not introduce nondeterminism.
- Dependency review confirms the foundation has no networking, async-runtime, model-provider, database, or UI dependency.
- Trace-mode tests confirm enabling action output does not change authoritative final state.
