+++
id = "CAP-MOK-004"
type = "capability"
title = "Observe and inspect a running simulation in a terminal"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
derives_from = ["INT-MOK-004"]
+++

# Capability: Observe and inspect a running simulation in a terminal

## Actor and need

The actor is an **operator**: the person running the simulation to understand its behavior. That is the same
actor `CAP-MOK-001` and `CAP-MOK-002` serve, in a mode those capabilities do not provide. Both existing
capabilities deliver a completed run as text and answer *what happened in aggregate*. This one answers *what is
happening now, to whom, and where*, while it happens.

Three sub-roles share the actor and pull the capability in the same direction:

- The **product owner** watching whether the population behaves plausibly, who needs the whole world at once.
- The **technical owner** asking why one agent did one thing, who needs a single agent's proposal and the
  engine's response to it.
- The **assurance owner** deciding whether an observation is admissible, who needs the observation to be
  reproducible and to leave a file behind.

The need is not presentation. It is attribution. The operator must be able to connect an outcome to the position,
state, and authorized action that produced it, at the tick it happened, and must be able to do so before the
behavior scrolls away.

## Capability statement

`An operator can run a bounded simulation under direct observation, seeing the whole world's Mokiterions and
resources at once, pausing and advancing it a single tick at a time, selecting any Mokiterion to inspect its
survival state, its proposed action and the engine's authority decision on that proposal, filtering the event
stream by type or subject, and exporting the observed events as a file — while the simulation's outcome remains
identical to the same run executed without observation.`

## Actor-visible behavior

An operator starts an observed run by choosing the observer component instead of the text component, with the
same seed, tick limit, density and policy inputs. From there:

**They see the world.** Every living Mokiterion and every standing resource appears in one view at its own
position, with the territory boundary drawn and each half's standing resource count shown as a headline figure
rather than something to be counted. Resource class is distinguishable. Territory membership is
distinguishable. Because a territory that reaches zero resources never regenerates again, approaching depletion
is surfaced as a warning rather than left to be inferred from a falling number.

**They control time, not the world.** The run can be paused, advanced exactly one tick, resumed, and run at a
chosen speed. Pausing holds the engine between ticks; it does not freeze a half-applied tick, and it does not
alter what the next tick computes. There is no operator action that changes world state.

**They select and inspect.** Any Mokiterion can be selected. The selection shows its identifier, position,
territory, health, satiety and energy, and its current action. It also shows the action the decision source
proposed and whether the engine accepted or rejected it, so that the boundary `ADR-MOK-001` protects is legible
at the moment it acts. A selected Mokiterion can be followed as it moves.

**They read and keep the events.** Events appear tick-stamped and typed as they occur, newest visible, filterable
to one event type or one subject. The visible log can be written to a file whose contents are stable for a given
seed and configuration, so it can be retained as work-order evidence rather than described.

**They see the run's provenance.** The seed and configuration that produced what is on screen are displayed with
it, so a screen capture is self-identifying, and each event type names the requirement that authorizes the
behavior it reports.

**It works in the terminal they have.** At the reference size the full layout is present. As the terminal
narrows, panes are dropped or overlaid in a declared order, and the operator is told what is hidden rather than
silently losing it. Below a floor the observer refuses to run and says what size it needs, instead of drawing a
corrupt world.

## Boundaries

- **Observation only.** The operator cannot mutate world state. The single influence available is when the
  engine advances.
- **One run at a time, in memory.** No run is saved, reloaded, or compared with another.
- **Only values the engine computes.** Where the eventual design shows a quantity the engine does not yet
  produce — fear, kills, combats, remembered places, model latency, per-agent names — the capability shows
  nothing there. It does not substitute a placeholder that reads as a real zero, and it does not derive a proxy.
  Those panes fill when the phases that create the data land.
- **The text stream is unaffected.** `REQ-MOK-010` output remains available and unchanged. This capability is an
  alternative view, never a migration.
- **Terminal only.** No graphical, web, or remote interface.

## Dependencies and assumptions

- The engine's authoritative state must be readable as an immutable snapshot without granting a mutable handle,
  consistent with `ADR-MOK-001`.
- The engine must be advanceable one tick at a time by its caller, since single-stepping is the operator's
  primary instrument and cannot be simulated by a delay.
- Observation must consume no simulation entropy and must not reorder ticks or agent turns, or `REQ-MOK-009` is
  broken and every prior measurement becomes unreproducible.
- The engine must remain buildable and testable with no external dependency after the interface exists, which
  requires a component boundary rather than a convention.

## Rationale

`INT-MOK-004` argues that position is this world's dominant variable and that later phases increase behavioral
unpredictability by design. A capability that presents position, state, and the authority decision together, at a
tick the operator chooses, is the smallest thing that makes those phases assessable. Anything smaller — richer
text, a summary table, a post-run report — still asks the operator to reconstruct a spatial situation from a
sequence, which `INT-MOK-004` identifies as the actual failure.

The capability is deliberately stated as observation with no mutation. The moment an operator can place a
resource or move an agent, every run becomes an intervention, determinism stops being a property of seed and
configuration, and `REQ-MOK-009` no longer means anything. Excluding mutation is what allows this capability to
be added without weakening the assurance chain that `VREC-MOK-001` and `VREC-MOK-002` established.
