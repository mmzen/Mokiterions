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

Added and verified under `VREC-MOK-002` (commit `68163ac`), completing Phase 1:

- Bounded perception: nearby resources and nearby living Mokiterions with direction and distance, radius 16
- Resource density as an explicit simulation input, binding initialization, capacity, and replenishment
- A deterministic reference decision source (eat → sustain → approach → search), selectable by `--policy`
- A measured population viability floor of eight of twelve survivors at 1,000 ticks on five declared seeds

Not yet implemented: `fear`, per-agent individuality, social and combat behavior, LLM-backed decisions, any
interactive or structured observability.

Two limitations carried forward from Phase 1 are recorded in `VER-MOK-002`'s residual uncertainty and are not
restated as analysis here: high-class resources accumulate against capacity, and the viability floor is a claim
about tick 1,000 rather than a steady state.

**The two hard blockers of the next section are resolved.** Constraints 1 and 2 below are retained as the recorded
rationale for the phase ordering, not as a description of the current system. Their figures describe the
pre-Phase-1 constants and are superseded by the measurements in
`docs/engineering/simulation/evidence/WO-MOK-002/`.

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
Phase 1.5  Terminal observer  ── the instrument every later phase is assessed with
   │
   ▼
Phase 2  Individuality: traits and fear
   │
   ├──────────────┬───────────────┐
   ▼              ▼               │
Phase 3        Phase 4            │   (3 and 4 are independent
Conflict       Analytical         │    and may run in parallel)
   │           observability      │
   │              │               │
   └──────┬───────┘               │
          ▼                       │
    Phase 5  LLM decision source ─┘
          │
          ▼
    Phase 6  Emergence evaluation
```

Phase 1.5 is not a prerequisite of Phase 2 in a technical sense — Phase 2 could be built without it. It is placed
here because it is the instrument by which Phases 2 through 6 are *judged*, and building it after the behavior it
would have helped assess wastes its main benefit.

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
- Rebalanced food economy governed by an explicit **target carrying capacity**
- A deterministic heuristic reference policy (eat → sustain → seek nearest food → search) proving the world is
  survivable

No `explore` action. Directed navigation is movement plus perception, so a separate action would add a verified
behavior without adding capability. It was dropped when the packet was drafted.

**The carrying-capacity decision.** Target capacity should become an approved, verified requirement parameter
rather than an accidental consequence of constants. The productive regime is *narrow scarcity*: capacity
around 8–10 of 12, so competition genuinely constrains behavior while extinction is not preordained.
Abundance suppresses emergent behavior as effectively as famine does.

**The heuristic policy is a scientific control.** Without it, later phases cannot distinguish "the model
produced emergent cooperation" from "any competent policy survives this world." That distinction is the
difference between a defensible result and an unsupported claim.

**Out of scope.** Fear, combat, social behavior, model integration, structured output.

**Artifact chain.** New intent. `INT-MOK-001` is not widened, because its purpose and success measures were
achieved and verified under `VREC-MOK-001`; its non-goals do not exclude perception or food balance, but a
completed intent is not the place to record new outcomes.

The packet drafted on 2026-08-17 is: `INT-MOK-002`, `CAP-MOK-002`, `REQ-MOK-013` (perception), `REQ-MOK-014`
(viable population), `REQ-MOK-015` (reference decision source), `VER-MOK-002`, `WO-MOK-002`, and an in-place
amendment of `SPEC-MOK-001`. It is deliberately smaller than the 8–10 requirements estimated here: every
existing requirement is constant-free, so the food rebalance needs no requirement change, only the amended
specification. `SPEC-MOK-001` is amended rather than superseded so that no two active specifications state
conflicting survival values.

**Key verification.** Heuristic policy sustains at least the target capacity across ≥1,000 ticks on multiple
seeds; measured carrying capacity within an approved tolerance; byte-identical determinism preserved.

**Status.** Delivered. `WO-MOK-002` is `implemented` and verified under `VREC-MOK-002` at commit `68163ac`.

---

## Phase 1.5 — Terminal observer

**Goal.** Build the instrument for watching behavior, as a separate component that cannot affect what it watches.

**Why here, and why it is numbered 1.5.** It was not in the original sequence. It is inserted because Phase 1
produced the first result the text stream could not explain: extinction with 122 resources standing and both
territories at capacity. Position, not supply, decides outcomes, and position is exactly what a line-oriented
stream conveys worst. Every phase after this one produces behavior whose plausibility is a question about a
spatial situation at a specific tick — *why did that agent walk away from food*, *why did those two fight*, *is
this model output reasonable given what the agent could see*. Deferring the instrument means assessing those
phases without the means to assess them, and the cost of building it does not fall over time.

It is numbered 1.5 rather than made a new Phase 2 because it delivers no simulation behavior. Nothing about the
world changes.

**In scope**

- A terminal user interface as a **second package**, over a read-only observation surface on the engine package
- Whole-world spatial view, survival state for every living Mokiterion, an inspector exposing each proposal and
  the engine's verdict on it, a filterable and exportable event log, keyboard control of progression and focus,
  and run provenance with the authorizing requirement for each event type
- Layout that degrades across viewport sizes without losing authoritative information

**Out of scope.** Every simulation rule. This phase is additive to `SPEC-MOK-001` and changes nothing in it. Also
out of scope: every attribute the target design anticipates that the engine does not compute — fear, traits,
names, combat outcomes, remembered locations, per-agent entropy, model latency. The specification reserves space
for one of them and requires that space to render empty. An inert `fear 0` would be a claim the engine cannot
support, and it is prohibited rather than merely discouraged.

**This phase ends two properties the project has held since `WO-MOK-001`,** and both are the product owner's
decision rather than a technical detail:

1. **The empty dependency set.** `ratatui 0.30.2` resolves to a measured 57 crates. Trimming features saves six.
   This is the project's first external dependency, and the containment strategy is that it is confined to the
   observer package — which is *why* the two-package split is a requirement rather than a preference.
2. **The single crate.** `ARCH-MOK-001` prohibited separate crates *without an approved requirement*.
   `REQ-MOK-023` is that requirement, for exactly one further package.

**Distinct from Phase 4.** Phase 4 is a batch measuring instrument: structured output, multi-seed runs, outcome
distributions, statistics over many runs. Phase 1.5 is a live interactive instrument for one run. They answer
different questions — *what happens across many runs* against *what is happening in this one* — and neither
substitutes for the other. Phase 4 remains where it is.

**Artifact chain.** The packet drafted on 2026-08-17 is: `INT-MOK-003`, `CAP-MOK-003`, `REQ-MOK-016` through
`REQ-MOK-024`, `SPEC-MOK-002`, `ARCH-MOK-002`, `ADR-MOK-002`, an in-place amendment of `ARCH-MOK-001`,
`VER-MOK-003`, and `WO-MOK-003`. Every artifact is `draft` and awaits its accountable owner.

Two of those artifacts are load-bearing rather than procedural. `REQ-MOK-023` is the approved requirement that
`ARCH-MOK-001`'s prohibition on separate crates has always named as its own unlock. The `ARCH-MOK-001` amendment
scopes its prohibition on user-interface frameworks to the engine package, where it becomes checkable per package
and therefore stronger than before. Without both, this phase is prohibited outright.

`ADR-MOK-001` is **not** superseded. Its migration clause requires a superseding ADR only for replacing engine
authority, and a read-only observer replaces none of it.

**Key verification.** An observed run and an unobserved run at the same seed are byte-identical in authoritative
events and final state, under heavy operator interaction, with identical per-tick entropy draw counts. The engine
package's dependency set is empty, proved per package. Rendering is asserted cell by cell from an in-memory
buffer at each declared viewport, because a claim about a screen only a human has seen is the weakest evidence
this project accepts — and the observer's whole value is being trusted about what it shows.

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
- **Carried in from Phase 1: high-class resource accumulation.** `SPEC-MOK-001` rule 5 makes a resource both
  eatable and approachable only while its satiety restoration would not be clipped, which for high class means
  satiety of at most `50`. High-class resources are therefore sought least often and accumulate against the
  capacity that density fixes — roughly three quarters of standing supply by tick 1,000 — so territories stay
  full while the food anyone will walk to grows scarce. The population declines against a full larder, and a
  10,000-tick run at the default density reaches extinction at tick 9,154. This was measured, disclosed, and
  accepted by the product owner on 2026-08-17 because no Phase 1 requirement speaks past tick 1,000, where the
  rule is decisively better than its predecessor. See
  `docs/engineering/simulation/evidence/WO-MOK-002/density-curve.md`.

**Out of scope.** Combat resolution, model integration.

**Key verification.** Identical situations demonstrably produce divergent choices across agents; traits fully
reproducible from seed; fear dynamics bounded and observable.

**Note on the carried-in item.** Fixing accumulation means changing when a rich resource may be consumed, which
moves the density curve and invalidates `REQ-MOK-014`'s floor of eight. Expect that requirement to need
re-approval on a fresh measurement rather than an argument. A long-horizon stability requirement should be stated
at the same time, because the current floor is a claim about tick 1,000 and deliberately not about a steady state.

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

**Relationship to Phase 1.5.** Phase 1.5's terminal observer does not deliver any part of this phase, and this
phase does not make it redundant. Phase 1.5 answers *what is happening in this run*; this phase answers *what
happens across many runs*. Phase 1.5's event export is a per-run record in the existing text format, deliberately
not a structured stream, so the structured output this phase adds is still new work. If Phase 1.5's snapshot
contract turns out to be the natural source for structured output, that is a convenience to exploit rather than a
dependency to rely on.

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
| 1 | `SPEC-MOK-001` amended in place; new intent and capability added; no existing requirement changed |
| 1.5 | `ARCH-MOK-001` amended in place to scope its one-crate, empty-dependency and UI-framework rules to the engine package; new intent, capability, nine requirements, specification, architecture and ADR added; `ADR-MOK-001` not superseded; `SPEC-MOK-001` unchanged |
| 2 | New requirements only; observation contract extended |
| 3 | `REQ-MOK-005` extended or superseded |
| 4 | `REQ-MOK-010` preserved, extended additively |
| 5 | `REQ-MOK-009` and `INT-MOK-001` reproducibility measure decided; new ADR for provider adapter |
| 6 | New verification contract; no existing artifact changed |

## Open decisions requiring owner input

Two decisions are cheaper to settle now than at the phase in which they bind:

1. **Target carrying capacity for Phase 1.** *Decided 2026-08-17:* a floor of at least eight of twelve survivors
   at 1,000 ticks under the reference source, recorded as `REQ-MOK-014` and measured on the seed set declared in
   `VER-MOK-002`. All twelve surviving on every declared seed is an adverse observation, not a success.
2. **Determinism strategy for Phase 5.** Record/replay is recommended. This propagates into Phase 4's output
   format, so deciding it before Phase 4 avoids rework.

## Maintenance

Update this roadmap when a phase is converted into an approved artifact chain, when a phase is completed,
reordered, or abandoned, or when a constraint above is invalidated by measurement. Formal authority always
comes from typed artifact metadata and accountable lifecycle decisions under `docs/engineering/`, never from
this document.
