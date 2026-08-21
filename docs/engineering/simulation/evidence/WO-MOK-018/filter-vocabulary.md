# Filter vocabulary — `SPEC-MOK-003` rule 9 item 2's figure

Amendment 1 changes rule 9 item 2 from "one of the **eleven** core types" to "one of the **fourteen**
core types". This file establishes the corrected figure by measurement. `filter-vocabulary.txt` is the
capture; nothing below restates a number that was not read out of the tree.

## The measurement

`EventType::ALL` has **15** members. Enumerated with the `event=` string each renders to, and
partitioned as `SPEC-MOK-001`'s *Data and interface contracts* partitions them — "Stable core event
types are …" followed by "Optional per-action lines use `action_trace`" — the count is:

| Class | Count | Members |
|---|---|---|
| stable core | **14** | `world_initialized`, `food_initialized`, `agent_initialized`, `decision_source_selected`, `survival_changed`, `agent_died`, `food_consumed`, `food_regenerated`, `food_regeneration_skipped`, `territory_crossed`, `attack_resolved`, `threat_resolved`, `surrender_resolved`, `simulation_ended` |
| optional | **1** | `action_trace` |
| total | **15** | |

Rule 9 item 2 as amended reads "one of the fourteen core types or to `action_trace`", so the sentence
now accounts for all fifteen: fourteen by the figure and one by name. Before the amendment it
accounted for twelve of them and silently excluded three.

## Why the figure was eleven, and what made it fourteen

The three types that arrived after the figure was last written are `attack_resolved`,
`threat_resolved` and `surrender_resolved`, added by `SPEC-MOK-001`'s 2026-08-20 amendment under
`CAP-MOK-010` and implemented by `WO-MOK-016`. That amendment's own text records inserting them
"where `SPEC-MOK-001` lists them, after `territory_crossed`, so that no existing type moves relative
to any other" — which is why the observer's filter needed no code change and why nothing failed. The
count in rule 9 item 2 is the only place the arrival was not carried through.

Eleven plus those three is fourteen, so the corrected figure is not a re-count of an unrelated set:
it is the same set the rule always meant, plus the three the engine gained.

## What this measures, and what it does not

It measures the vocabulary. It does **not** measure that the observer's filter cycles the whole
vocabulary — that is already asserted by the observer's existing sweeps over `EventType::ALL`, which
is why the work order's *Required verification* item 4 says no new assertion is required for the
figure itself and that one would belong to the public tier if added. None was added. A test that
restated `14` as a literal would restate the very number under correction; the sweeps read
`EventType::ALL`'s length instead, which is the reason they did not fail while the prose was wrong.

That is the shape of this defect and it is worth naming: **prose that duplicates a value the code
derives will go stale silently.** Rule 9 item 2 now names one figure and one type rather than
enumerating, so it is still duplication, and it will go stale again the next time the engine's
vocabulary grows. The narrower alternative — stating the count as "every type `EventType::ALL`
declares" — was not taken, because rule 9 is a specification of the observer's obligations and a
specification that defers its own content to an implementation symbol states nothing a reader can
check. The duplication is deliberate and the maintenance cost is real.
