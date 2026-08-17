+++
id = "REQ-MOK-020"
type = "requirement"
title = "Control run progression and observation focus by keyboard"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN the operator presses a bound key, THE SYSTEM SHALL perform the bound observation control — pausing, resuming, advancing exactly one tick, changing progression speed, changing or clearing the selection, following the selection, changing the view region or fidelity, changing the filter, exporting, or exiting — and SHALL apply no other effect."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-003"]
+++

# Requirement: Control run progression and observation focus by keyboard

## Rationale

Single-stepping is the operator's primary instrument, and it cannot be approximated by slowing the run down. The
questions this interface exists to answer are of the form "what did that agent propose on the tick it died", which
requires stopping between two specific ticks and looking. A run that only plays at a chosen rate forces the
operator to catch the moment, and the moment is usually one tick wide.

Pausing must hold the engine *between* ticks rather than inside one. A tick applies twelve agent turns in fixed
order and then may regenerate resources; stopping partway through would present a state that no completed tick
ever produced, and an operator reading it would draw conclusions about a state the simulation does not have.

Keyboard binding rather than pointer interaction follows from the medium: a terminal interface has a keyboard
available everywhere and a mouse available inconsistently, and every control here is a discrete verb rather than a
spatial gesture.

The closing clause matters as much as the list. Every operator input is an observation control. There is no input
that mutates world state, because `ADR-MOK-001` gives the engine exclusive ownership of mutable state and an
operator is not an exception. Choosing *when* the engine advances is the full extent of operator influence, and it
changes when the engine runs rather than what it computes.

## Preconditions and trigger

The observer is running and has an input source. The trigger is a key press.

## Required response

Each bound key performs exactly its bound control:

- **hold and release progression** — stop the engine between completed ticks, and resume from that point;
- **advance one tick** — complete exactly one further tick and stop again;
- **change progression speed** — alter how frequently the engine advances while running;
- **select and clear selection** — move the selection between living Mokiterions, and remove it;
- **follow** — keep the view region centred on the selected Mokiterion as it moves;
- **change view region or fidelity** — pan and zoom the spatial view;
- **change filter** — apply or clear the event-log restriction;
- **export** — write the observed events to a file;
- **exit** — leave the observer and restore the terminal.

Advancing one tick while already held completes one tick and returns to held. Advancing is available only while
held, so that a single-step and a running engine cannot both be driving progression.

An unbound key is ignored. It produces no action, no state change, and no error condition that interrupts
observation.

## Failure and boundary behavior

- Exiting restores the terminal to its prior mode. An abnormal termination must also restore it rather than
  leaving the operator's terminal unusable.
- Advancing past the configured tick limit, or past extinction, does not advance the simulation. The observer
  reports that the run has terminated and remains inspectable, so the final state can be examined.
- Speed changes are bounded by declared limits and cannot be set to a value that prevents the observer from
  responding to further input.
- A key pressed while a frame is being drawn is not lost, and it is not applied twice.
- No key press mutates health, satiety, energy, position, entropy, resource placement, tick order, or agent turn
  order.

## Constraints

- The complete key binding table, speed steps and their bounds, selection order, and the terminal modes entered
  and restored are fixed by `SPEC-MOK-002`.
- Input handling consumes no simulation entropy. Whether, when, and how often the operator presses a key must not
  appear in any authoritative outcome.
- Holding progression must leave the engine at a completed-tick boundary.
- The observer is quit by a bound key. No on-screen control element is required, and the presence of one must not
  become the only way to perform any control.

## Acceptance examples

### Example: normal behavior

**Given** an observed run held at tick 40

**When** the operator presses the advance-one-tick key three times

**Then** the run stands at tick 43, held, and every intervening tick was applied exactly once in order.

### Example: failure behavior

**Given** an observed run configured for 100 ticks and currently held at tick 100

**When** the operator presses the advance-one-tick key

**Then** the tick count remains 100, the observer reports the run as terminated by its tick limit, and the final
state remains inspectable.

## Open decisions

None. The binding table, speed bounds, selection order and terminal-mode handling are fixed by `SPEC-MOK-002`.
