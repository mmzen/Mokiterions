# Mokiterions Functional Roadmap

> **Authority note.** This document is repository-owned functional planning. It is **not** approved product
> intent, a requirement, an architecture decision, work authorization, a verification contract, or release
> authority. Only formal artifacts under `docs/engineering/` carry that authority, per `ENGINEERING_HARNESS.md`.
> Nothing here authorizes implementation. Each phase below must be converted into an approved artifact chain
> before any work begins.

- **Created:** 2026-08-17
- **Scope:** macro phases following the completed minimum simulation foundation (`WO-MOK-001`)
- **Audience:** product owner, engineering owner, assurance owner

## Purpose

`WO-MOK-001` delivered a verified, deterministic, engine-authoritative foundation. This roadmap proposes the
macro evolutions from that foundation to the project's stated success criteria: twelve autonomous Mokiterions
inhabiting the world over an extended simulation and producing outcomes that are not predetermined by the
engine, with the whole lineage traceable through SE Harness.

## Current state

Implemented and verified under `VREC-MOK-001` (commit `ecd03a8`):

- 128 × 128 world, two territories, twelve Mokiterions, three food classes
- `health`, `satiety`, `energy`; bounded decay; death at zero health
- Actions `wait`, `sleep`, `eat`, `move`; engine-side validation with no partial mutation
- Conditional food regeneration, including permanent territory depletion
- Deterministic SplitMix64 entropy from an explicit seed
- Line-oriented text observation stream and optional `--trace-actions`
- A local seeded baseline decision source, explicitly a development placeholder

Not yet implemented: perception beyond the agent's own cell, `fear`, per-agent individuality, social and
combat behavior, LLM-backed decisions, structured or aggregate observability.

## Three constraints that shape the ordering

Three properties of the current system, rather than preference, determine the phase sequence.

### 1. The world as specified cannot sustain twelve agents

Satiety is the only binding survival constraint, because sleep restores energy at no cost while satiety has no
non-food source. The population-level food economy is therefore decisive:

| Quantity | Value |
|---|---:|
| Satiety demand (2/tick × 12 agents) | 24 satiety/tick |
| Regeneration rate (1 item per territory per 10 ticks) | 0.2 items/tick |
| Mean food value across the three classes | 31.67 satiety |
| Regeneration supply | ≈6.3 satiety/tick |
| **Supply-to-demand deficit** | **≈3.8×** |
| **Implied steady-state carrying capacity** | **≈3 of 12 agents** |

Initial stock does not close the gap. Six items carry roughly 190 satiety, about eight ticks of population
demand, and climbing from six items to the 24-item cap would require about 90 ticks of zero consumption while
agents begin starving at tick 50.

The observed extinction at tick 69 on default settings is therefore **not** a deficiency of the baseline
policy. No policy of any sophistication survives this configuration; optimal play preserves roughly three
agents. Introducing an intelligent decision source before correcting the food economy would produce mass
death and yield no information about emergent behavior.

Because `SPEC-MOK-001`'s constants are approved, and `WO-MOK-001`'s decision envelope explicitly prohibited
changing specified constants, correcting this is a governed specification change supported by evidence, not a
tuning adjustment.

### 2. Agents cannot perceive anything beyond their own cell

`Observation` exposes only `co_located_food`. There is no nearby food, no nearby Mokiterions, no memory, and
no recent-event context. Any decision source, heuristic or model-backed, is effectively blind and cannot
navigate. Perception is a prerequisite for every behavioral phase.

### 3. A live model provider collides with the reproducibility requirement

`REQ-MOK-009` requires reproducible entropy, and `INT-MOK-001` sets a success measure of 100% identical
results for repeated runs at an identical seed. A live provider cannot satisfy this. The collision requires an
explicit governance decision, addressed in Phase 5.

## Dependency graph

```text
Phase 0  Harness upgrade landed under governance
   │
   ▼
Phase 1  Perceptible, survivable world  ── prerequisite for everything below
   │
   ▼
Phase 2  Individuality: traits and fear
   │
   ├──────────────┬───────────────┐
   ▼              ▼               │
Phase 3        Phase 4            │   (3 and 4 are independent
Conflict       Observability      │    and may run in parallel)
   │              │               │
   └──────┬───────┘               │
          ▼                       │
    Phase 5  LLM decision source ─┘
          │
          ▼
    Phase 6  Emergence evaluation
```

Running alongside Phases 1–2: a bounded **LLM feasibility experiment** (see Cross-cutting work).

---

## Phase 0 — Land the in-flight harness upgrade

**Prerequisite gate, not a product phase.**

The working tree contains an SE Harness 0.2.1 → 0.4.0 migration, including a canonical artifact-layout move
and a schema migration of `ARCH-MOK-001`. That architecture artifact is `approved` yet modified, which is
itself a governance act. Landing this under one explicit governance work order prevents every later phase from
building on an unauthorized base.

**Also resolve:**

- The evidence-detection mismatch: `scripts/generate_harness_dashboard.py` keys evidence on **file name**,
  while `WO-MOK-001` instructed evidence into a **directory** named for the work order. This produces a
  misleading `W-HEX-001` warning despite evidence existing. Align the convention in one direction.
- `W-HEX-003`: `ADR-MOK-001` now predates its newer declared target `ARCH-MOK-001` and needs reassessment.

**Verification:** `harnessctl doctor` clean, validator passing, `inspect` findings resolved or explicitly
accepted with rationale.

---

## Phase 1 — A perceptible, survivable world

**Goal.** Make survival possible, and make the world legible to a decision source.

**Why first.** Removes both hard blockers identified above. No downstream behavior is measurable until an
agent can locate food and the food economy can support a population.

**In scope**

- Perception radius: nearby food with direction and distance, nearby Mokiterions, territory boundaries
- The `explore` action
- Rebalanced food economy governed by an explicit **target carrying capacity**
- A deterministic heuristic reference policy (eat → seek nearest food → sleep) proving the world is survivable

**The carrying-capacity decision.** Target capacity should become an approved, verified requirement parameter
rather than an accidental consequence of constants. The productive regime is *narrow scarcity*: capacity
around 8–10 of 12, so competition genuinely constrains behavior while extinction is not preordained.
Abundance suppresses emergent behavior as effectively as famine does.

**The heuristic policy is a scientific control.** Without it, later phases cannot distinguish "the model
produced emergent cooperation" from "any competent policy survives this world." That distinction is the
difference between a defensible result and an unsupported claim.

**Out of scope.** Fear, combat, social behavior, model integration, structured output.

**Artifact chain.** New intent — `INT-MOK-001`'s non-goals bar this work, so it cannot simply be widened —
plus a new capability, roughly 8–10 requirements, a new specification, a verification contract, and 2–3 work
orders. `SPEC-MOK-001`'s food-economy constants require explicit amendment or supersession.

**Key verification.** Heuristic policy sustains at least the target capacity across ≥1,000 ticks on multiple
seeds; measured carrying capacity within an approved tolerance; byte-identical determinism preserved.

---

## Phase 2 — Individuality: traits and fear

**Goal.** Twelve agents that are genuinely not interchangeable.

**Why here.** All agents are currently identical and share a single entropy stream, so the concept's
"individuality without randomness" is unimplemented. `fear` is a hard prerequisite for Phase 3. Completing the
agent state model here means the observation contract stabilizes **once**, so Phase 5's provider adapter is
written against a final contract instead of being rewritten twice.

**In scope**

- Per-agent trait vector derived deterministically from seed and agent ID (for example caution, aggression,
  sociability)
- `fear` as the fourth dynamic attribute, with defined rise and decay dynamics
- Per-agent entropy substreams, so identical situations can diverge across individuals

**Out of scope.** Combat resolution, model integration.

**Key verification.** Identical situations demonstrably produce divergent choices across agents; traits fully
reproducible from seed; fear dynamics bounded and observable.

---

## Phase 3 — Social encounter and conflict

**Goal.** Complete the action contract: `approach`, `avoid`, `threaten`, `attack`, `fight`, `retreat`,
`surrender`.

**Why here.** Requires Phase 2's `fear` and Phase 1's perception of nearby agents. Completing the action set
before model integration means the decision contract is authored once.

**In scope**

- Encounter detection and resolution
- Combat energy and health costs; death through combat
- Response selection influenced by fear, health, energy, perceived relative strength, and prior interactions
- Interaction history feeding back into observation

**Critical constraint.** No population-level scripting. The concept explicitly forbids rules of the form
`if food < threshold: start_war()`. Conflict must be a consequence of individual decisions under world rules.
Verification should assert the **absence** of population-level triggers, not merely the presence of combat.

**Governance.** `REQ-MOK-005` ("Apply core actions") requires extension or supersession.

---

## Phase 4 — Analytical observability

**Goal.** Build the measuring instrument.

**Why here.** The claim that outcomes are not predetermined cannot be supported by a single-line text summary.
It requires outcome distributions across many seeds. **This phase is independent of Phase 3** — different
modules, no shared dependency — so the two may proceed in parallel.

**In scope**

- Structured event stream (for example JSONL) alongside the existing text stream
- World-level metrics: surviving population, population per territory, available food, consumption and
  regeneration, average health, satiety, and energy, territory crossings, conflict frequency, deaths,
  survival time
- Multi-seed batch runner and run persistence
- Outcome classification (coexistence, famine, collapse, extinction, and similar)

**Constraint.** Preserve the existing text output so `REQ-MOK-010` remains satisfied. Add, do not replace.

---

## Phase 5 — LLM decision source

**Goal.** Replace the baseline with a model-backed decision source at the **existing** trust boundary.

**Why last.** Every input the model consumes is produced by Phases 1–4. This ordering also matches
`INT-MOK-001`'s own recorded assumption that an engine-first foundation should be verified before adding an
external decision source, and it defers the only expensive, nondeterministic, credential-bearing work until
all cheap deterministic verification is complete.

**The central decision: determinism strategy.**

| Option | Assessment |
|---|---|
| **Record / replay** (recommended) | Live runs record every prompt→response pair to a transcript; replay mode reads the transcript and is fully deterministic. Preserves reproducible verification without constraining live behavior. |
| Response cache keyed on observation hash | Makes repeats deterministic but collapses exactly the individuality Phase 2 establishes. |
| Temperature 0 | Not a bitwise determinism guarantee from any provider. Should not be treated as one. |

Record/replay aligns unusually well with the project's engineering objective: **the transcript becomes
commit-bound verification evidence**, making it provable which model outputs produced a verified run.

**Governance.** `ADR-MOK-001` does **not** require superseding. `ARCH-MOK-001`'s own rationale already defers
"an external model provider to an adapter at the same boundary." What is needed is a **new** ADR covering the
provider adapter and the determinism strategy, plus an explicit decision on `REQ-MOK-009` and on
`INT-MOK-001`'s 100% reproducibility success measure.

**Constraints.** Confirm the exact OpenAI model identifier before integration. Credentials remain outside the
repository per `REPOSITORY_CONTEXT.md`. The decision source must remain unable to mutate authoritative state.

---

## Phase 6 — Emergence evaluation

**Goal.** Produce the actual proof-of-concept result.

**In scope.** Run random, heuristic, and model-backed policies across many seeds and report outcome
distributions: sustainable consumption, famine, migration, territorial behavior, raiding, aggression and
retaliation, surrender, war, asymmetric collapse, coexistence, extinction.

**Why this closes both objectives.** It answers whether emergence is real — model outcomes differing
*qualitatively* from the heuristic control, not merely scoring better — and simultaneously demonstrates
whether SE Harness traced the entire lineage from intent to exact commit.

**Success definition.** A defensible comparison. No particular behavior is required to occur; the requirement
is that the conditions permit emergence and that observed outcomes are not engine-determined.

---

## Cross-cutting work

### LLM feasibility experiment (parallel to Phases 1–2)

A bounded, throwaway spike measuring cost per 1,000 ticks, latency per decision, and schema-valid response
rate. SE Harness 0.4.0 supports `experiments/` as a first-class artifact type, so this can be governed
evidence rather than an ad-hoc investigation. It retires integration risk early **without** committing the
architecture, which is what makes deferring Phase 5 to last acceptable.

### Ordering alternative

The alternative to the sequence above is front-loading the model integration to Phase 3, immediately after
perception and traits, to retire integration risk earlier — accepting that the decision contract is rewritten
when combat lands. The roadmap recommends against this, preferring the feasibility experiment as a cheaper
means to the same risk reduction.

## Governance deltas by phase

| Phase | Existing artifacts affected |
|---|---|
| 0 | `ARCH-MOK-001` schema migration; managed harness files; evidence-naming convention |
| 1 | `SPEC-MOK-001` food-economy constants amended or superseded; new intent and capability required |
| 2 | New requirements only; observation contract extended |
| 3 | `REQ-MOK-005` extended or superseded |
| 4 | `REQ-MOK-010` preserved, extended additively |
| 5 | `REQ-MOK-009` and `INT-MOK-001` reproducibility measure decided; new ADR for provider adapter |
| 6 | New verification contract; no existing artifact changed |

## Open decisions requiring owner input

Two decisions are cheaper to settle now than at the phase in which they bind:

1. **Target carrying capacity for Phase 1.** The roadmap argues for 8–10 of 12 to hold the system in narrow
   scarcity. This value shapes the Phase 1 requirement set and its verification tolerances.
2. **Determinism strategy for Phase 5.** Record/replay is recommended. This propagates into Phase 4's output
   format, so deciding it before Phase 4 avoids rework.

## Maintenance

Update this roadmap when a phase is converted into an approved artifact chain, when a phase is completed,
reordered, or abandoned, or when a constraint above is invalidated by measurement. Formal authority always
comes from typed artifact metadata and accountable lifecycle decisions under `docs/engineering/`, never from
this document.
