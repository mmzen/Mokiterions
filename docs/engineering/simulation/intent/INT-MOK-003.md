+++
id = "INT-MOK-003"
type = "intent"
title = "Make emergent behavior observable and attributable"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
+++

# Intent: Make emergent behavior observable and attributable

## Problem

The simulation now produces behavior worth watching, and there is no way to watch it.

`VREC-MOK-002` binds a world in which twelve Mokiterions perceive, travel, refuse food that would be wasted,
and survive at a measured rate of eight to eleven of twelve at tick 1,000. All of that reaches an operator as a
line-oriented text stream. A 1,000-tick default run emits thousands of lines describing one agent-tick each. The
stream is complete and it is deterministic, which is exactly what assurance needs, and it is the wrong shape for
three questions the project now has to answer.

**Where is everybody.** Position is the dominant variable in this world. `WO-MOK-002` measured that hunger, not
scarcity, kills: the baseline source reaches extinction at tick 142 with 122 resources still standing and both
territories at capacity. Spatial distribution is therefore the explanatory variable, and it is the one thing a
sequential text stream cannot show. Reconstructing the position of twelve agents and roughly 122 resources on a
128 × 128 grid from a scrolling log is arithmetic an operator performs badly and slowly.

**Why did that agent do that.** The engine already validates every proposal and can trace proposals and results
at the decision boundary. `--trace-actions` exposes it. But the operator sees the trace of whichever agent-tick
scrolls past, not the trace of the agent they are curious about, and cannot pause to look.

**Which behavior is authorized.** The engine accepts or rejects each proposal, and a rejection is an ordinary,
expected event rather than a fault. Today an accepted move and a rejected move both appear as text among
thousands of lines. The distinction that `ADR-MOK-001` exists to protect — that a decision source proposes and
only the engine mutates — is invisible in practice at the exact moment it matters.

The cost of this gap grows, not shrinks. Every later phase makes behavior less predictable by design: traits and
fear in Phase 2, conflict in Phase 3, and an LLM decision source in Phase 5, whose output is untrusted input
that must pass the same validation as the local baseline. Judging whether a model-driven Mokiterion behaves
plausibly, and whether the engine correctly refused it when it did not, is a question about a spatial situation
at a specific tick. Deferring observability until after those phases means evaluating them without the
instrument needed to evaluate them.

There is a second, narrower problem. The known long-horizon defect — High-class resources accumulate against
capacity, and a 10,000-tick run at the declared density reaches extinction at tick 9,154 — was found by
measuring aggregate resource mix at seven horizons. It was diagnosed, correctly, from numbers. But it presents
as a spatial phenomenon: territories that look full while the food anyone will walk to has gone. The next defect
of that family is cheaper to see than to compute.

## Desired outcome

An operator can watch a run unfold, stop it at any tick, look at any Mokiterion, and see both what that
Mokiterion asked to do and what the engine allowed — without the observation changing the run.

Concretely, the outcome is reached when an operator can answer each of these from the screen rather than from
arithmetic:

- Where every living Mokiterion and every standing resource is, at once, for the whole world.
- Which territory is approaching the depletion trap, before it is sprung, since a territory at zero never
  regenerates again.
- What a chosen Mokiterion's health, satiety and energy are, and what it is doing this tick.
- What that Mokiterion proposed, whether the engine accepted or rejected it, and on what grounds.
- Which events occurred, filtered to a type or a subject, exportable as a file that can be retained as
  work-order evidence.

The last item matters beyond convenience. Observation that produces a retainable artifact strengthens the
assurance chain; observation that produces only a fleeting screen weakens it, because a claim about behavior
becomes something an operator reports rather than something a reviewer can re-read.

## Non-goals

- **Not a control surface.** The operator observes and navigates. Nothing in this intent lets an operator move
  a Mokiterion, place or remove a resource, alter health, satiety, energy, or entropy, or edit world state.
  `ADR-MOK-001` gives the engine exclusive ownership of mutable state, and an operator is not an exception to
  it. Selecting the tick at which the engine advances is the full extent of operator influence, and it changes
  when the engine runs rather than what it computes.
- **Not a replacement for the text stream.** `REQ-MOK-010` is verified behavior and stays exactly as it is. This
  intent is additive. An operator who wants a diffable, greppable, deterministic record still gets one, and the
  observer must not become the only way to see what happened.
- **Not new simulation behavior.** No trait, fear, memory, name, combat, or model-provider integration is
  introduced here. Presenting a value the engine does not compute would make the screen a source of claims the
  simulation cannot support, which is worse than an empty pane.
- **Not a graphical or web interface.** A terminal observer is chosen deliberately, for reasons recorded in the
  deciding ADR rather than assumed here.
- **Not persistence or aggregate multi-run analysis.** Exporting the events of the run in progress is in scope.
  Storing runs, comparing runs, or classifying outcomes is not.

## Accountability boundary

The product owner is accountable for this intent and for whether observability is worth its cost at this point
in the sequence, given that it delivers no new simulation behavior.

Two consequences of pursuing it belong to that decision and are stated here rather than discovered later.

**It ends the zero-dependency property.** The foundation has no external dependencies, and that is currently
provable by inspection of an empty `[dependencies]` table. Any terminal user interface worth building is a
framework, and the measured surface of the proposed one is 57 crates. Whether that is acceptable, and under what
containment, is an architectural decision for the technical owner recorded in an ADR; whether the observability
is worth paying for at all is this intent's decision.

**It ends the one-crate property.** Component separation is required rather than preferred here, because the
guarantee worth having is that the engine cannot depend on the interface even by accident. Prose cannot enforce
that. A crate boundary can.

The technical owner remains accountable for the presentation contract and the component boundary, the assurance
owner for what constitutes evidence that a terminal interface behaves correctly and that observation did not
perturb the run, and the engineering owner for bounded work authorization. Approving this intent authorizes no
implementation.

## Success signals

- An operator locates the spatial cause of a death or a territory collapse by looking, and states the tick at
  which it became inevitable.
- A rejected proposal is visible as a rejection, at the tick it happened, with the proposing agent identified.
- A run observed through the interface and the same run executed as text produce identical simulation outcomes,
  and the equality is demonstrated rather than asserted.
- The engine component still builds with no external dependency, provably and per component.
- An exported event log from an observed run is retained as evidence under a work-order ID.
