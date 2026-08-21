# Non-perturbation — `REQ-MOK-025` at this work order's change

`non-perturbation.txt` is the measurement. This file states what it establishes and what it cannot.

## Why the property is re-measured at all

`REQ-MOK-025` says observing a run must not change it. This work order adds a field to the observer's
`Death`, a map to its state, two arms to its private `ingest`, and two lines to the inspector's death
branch. Every one of those is downstream of the engine's stream — none writes back to it — so the
expected result is no movement. **That is exactly why it is measured rather than asserted:** a property
whose expected answer is "nothing changed" is the property a regression hides in.

`mokiterions-core` is untouched by this change. The diff against `f2a79e1` touches four documents and
four files, all four of them under `mokiterions-tui/`. The unobserved run in the comparison below is
therefore the same engine code as before the change, which is what makes the comparison a test of the
observer and not of both sides at once.

## What was compared

`VER-MOK-005`'s primary case drives two runs of the same seed and configuration:

- **the unobserved run** is `Simulation::run` — the engine binary's whole behaviour, one function away
  from its `main`, writing into memory instead of to a file;
- **the observed run** is stepped one tick at a time, drawn into a `TestBackend` at a viewport that
  rotates through all ten declared viewports (including `33 × 21`, which is below the floor and refused
  rather than rendered), and interacted with on every round through the twelve-key script the contract
  drives its own observed runs with — Tab, `z`, `l`, `j`, `e`, `f`, BackTab, `L`, PageUp, `c`, Esc and an
  unbound `W`, with `+` and `-` walked on top.

The two authoritative streams are then compared record by record. The oracle that produced the capture
was appended to `tests/verification.rs` so it reused that file's declared seed set, viewport set and
helpers; reimplementing the comparison would have measured the reimplementation.

## The result

All five declared seeds, at two depths:

| Depth | Records per seed (0, 1, 42, 123, 777) | Identical | First difference | Final state |
|---|---|---|---|---|
| 60 ticks — the contract's depth | 1616, 1622, 1623, 1615, 1620 | yes, all five | none | agrees |
| 300 ticks — `WO-MOK-013`'s depth | 7603, 7589, 7534, 7301, 7599 | yes, all five | none | agrees |

Identical means the same record count and the same records in the same order, with no first differing
index to report. "Final state agrees" is the stronger of the two: the engine's own summary line, and the
same line reconstructed from the observer's final state, are byte-identical — reproduced in full in the
capture for seed 42 at 300 ticks, down to each of the eleven counters.

## The corroboration that matters

**Seed 42 at 300 ticks produces 7534 records, which is the figure `WO-MOK-013` recorded at its own
capture.** That work order measured the same seed and depth on a tree that predates this change, so the
agreement is between two independent captures and not two readings of the same one. Had this change
perturbed the stream, that figure would be the first thing to move.

The second depth exists for that reason alone. The contract runs its primary case at 60 ticks, which is
sufficient for the property but is shallower than the recorded evidence; running 300 as well is what
makes an earlier work order's number a check on this one.

## What this does not establish

The comparison is over the authoritative stream and the final state. It does not — and `REQ-MOK-025`
does not ask it to — say anything about what the observer *presents*; the inspector's death line is
covered by `inspector.md` and by the three tests `test-census.md` places. A change that presented the
wrong value while leaving the stream untouched would pass every figure on this page.

The five seeds are the declared set, not a sweep. A perturbation reachable only at an undeclared seed is
outside what this measurement can see, which is the standing limit of the declared-set method and not a
gap this work order introduces.
