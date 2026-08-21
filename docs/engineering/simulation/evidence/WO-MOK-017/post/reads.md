# WO-MOK-017: no decision source reads composition

| | |
|---|---|
| Work order | `WO-MOK-017` (the resource composition drift) |
| Retains | "the enumeration showing no source reads composition" |
| Candidate | `post/COMMIT.txt` |
| Reader | `analysis/reads.py`, over the candidate's `mokiterions-core/src/simulation.rs` |
| Date | 2026-08-21 |

`REQ-MOK-060` bounds a territory's class composition, and `post/composition.txt` attributes
the corrected ratio to Mokiterions eating resources they used to leave standing. That
attribution holds only if composition is something the simulation *produces*. A source that
could read the per-class counts would turn the measured curve into a control loop, would make
the ceiling satisfiable by steering rather than by feeding, and would break `SPEC-MOK-001`
rule 3's contract about what a source may see. So the claim is enumerated twice below: once
from the type a source is handed, and once over every function a source can reach.

## 1. Where composition exists in the engine, and who reads it

Composition is the per-territory, per-class count of standing resources. One function computes
it, and every caller of that function is listed -- not a sample of them.

`Simulation::food_counts(territory) -> [usize; 3]`, lines 2998-3008, is the only place the
counts are formed. Its callers ahead of the test module:

| caller | line | what it feeds | on a decision path |
|---|---|---|---|
| `Simulation::territory_snapshot` | 2075 | the TUI's `TerritorySnapshot`, an observer surface | no |
| `Simulation::summary` | 2993 | rule 18's `RunSummary`, emitted after the run ends | no |
| `Simulation::summary` | 2994 | rule 18's `RunSummary`, emitted after the run ends | no |

That is 3 call sites in 2 functions -- `Simulation::summary` calls it
once per territory -- and both functions are reporting surfaces. Rule 18's summary is written
once the run is over, so nothing can act on it. The TUI's snapshot is read by an observer that
never proposes an action -- `INT-MOK-010` separates the two, and the composition figures in
`post/composition.txt` are recovered from the summary line for exactly this reason.

## 2. The structural enumeration: what a source is handed

This is the argument that survives future edits, because it is about the type rather than about
what today's bodies happen to do.

```rust
fn decide(&mut self, observation: &Observation, entropy: &mut DecisionEntropy<'_>) -> Action;
```

Declared at line 929. A source receives an `&Observation` and an
`&mut DecisionEntropy`, and nothing else. It is handed no `&Simulation`, so `self.foods` and
`food_counts` are not merely unused by the four sources -- they are **out of scope by type**.
A source cannot read composition without a signature change, and a signature change is a
visible amendment to rule 3 rather than a quiet edit inside a source body.

So the whole of what a source may see is the observation. Every field of it, enumerated in
declaration order, with what it could tell a source about composition:

| field | type | carries composition |
|---|---|---|
| `tick` | `u64` | no — a scalar clock |
| `agent_id` | `String` | no — an identity |
| `position` | `Coordinate` | no — one coordinate |
| `territory` | `Territory` | no — which territory, not what is in it |
| `health` | `u8` | no — the observer's own attribute |
| `satiety` | `u8` | no — the observer's own attribute |
| `energy` | `u8` | no — the observer's own attribute |
| `fear` | `u8` | no — the observer's own attribute |
| `waste_tolerance` | `u8` | no — the observer's own trait |
| `suffered` | `Vec<SufferedAttack>` | no — attacks suffered, with attacker and damage |
| `co_located_food` | `Vec<String>` | no — ids underfoot, no class and no counts |
| `perceived_food` | `Vec<PerceivedFood>` | **class-bearing, but not composition** — see below |
| `perceived_mokiterions` | `Vec<PerceivedMokiterion>` | no — id, direction, distance |
| `valid_actions` | `Vec<Action>` | no — rule 5's core proposals |

14 fields, each classified. `perceived_food` is the only one that needs an
argument rather than a glance, and it gets one because it is the field a sceptical reader
should press on:

```rust
struct PerceivedFood {
    id: String,
    class: FoodClass,
    direction: Option<RelativeDirection>,
    distance: u8,
}
```

It carries `class`, so a source knows the calorie class of resources it can *see*. That is not
composition, on three counts, and the difference is the whole of this section:

* It is **radius-limited**. The list is built inside `PERCEPTION_RADIUS` of the observer, so it
  is a local window and not a territory. A territory is 128 by 64 cells.
* It is **a list of individuals, not counts**. Nothing sums it by class, and nothing compares
  the sums. Composition is the ratio between three totals; this is a sorted sequence of items.
* It **omits what has been eaten**. Composition is a statement about what remains standing
  across a territory, including everything out of perception. No source can see that set.

## 3. The empirical enumeration: every function each source can reach

The structural argument is about the four sources' signature. This one closes the remaining
gap -- a helper reading composition on a source's behalf -- by asking the question of every
function each source can transitively reach, not just of the four `decide` bodies.

| source | `decide` at | functions reachable | any reads composition |
|---|---|---|---|
| `baseline` | 947 | 13 | none |
| `reference` | 969 | 23 | none |
| `individual` | 1110 | 27 | none |
| `social` | 1147 | 28 | none |

The patterns searched for are `food_counts`, `foods`, `food_a`, `food_b`, `standing` and
`counts[` -- the computing function, the authoritative collection it reads, the summary fields
it feeds and the snapshot field beside them. The call graph is followed by bare callee name,
which over-approximates: each closure contains functions the source may not truly reach, so a
clean result here is stronger than the graph strictly requires rather than weaker.

Which fields the sources *do* read, for contrast. This table is built from each `decide` body
alone, plus the helpers it hands the observation to, named separately. The over-approximated
closure is deliberately **not** used here: it reaches `Observation::is_consistent`, which
touches every field in a `debug_assert!`, so a closure-wide intersection returns all fourteen
for all four sources and says nothing. Over-approximation is the safe direction for the
negative claim above and the useless direction for this one.

| source | fields read in `decide` itself | functions called where the observation appears |
|---|---|---|
| `baseline` | `valid_actions` | `choose_index`, `is_consistent` |
| `reference` | `energy` | `allows`, `best_fitting_co_located_food`, `best_fitting_distant_food`, `is_consistent`, `valid_moves` |
| `individual` | none | `is_consistent`, `tolerant_movement_choice`, `tolerant_survival_choice` |
| `social` | `fear`, `suffered` | `is_consistent`, `nearest_beyond_contact`, `nearest_in_contact`, `tolerant_search_choice`, `tolerant_seek_choice`, `tolerant_survival_choice` |

The third column is literal: the functions called on a line where `observation` also appears.
`baseline`'s `choose_index` is there because it is handed `observation.valid_actions.len()`,
not the observation, and `is_consistent` is a `debug_assert!` in all four.

`baseline` is the row that matters. It reads `valid_actions`, draws over its length, and calls
none of the `fits` family or the rule 19 helpers -- so the corrected condition is unreachable
from rule 4. That is why a change to the waste arithmetic leaves `--policy baseline` alone, and
why `post/byte-identity.txt` can hold in all thirty baseline cells. The other three reach the
condition through `best_fitting_*` or the `tolerant_*` helpers named beside them, which is the
path every divergence in `post/divergence.txt` is attributed to.

## 4. What this does not claim

* **Not** that the sources are indifferent to class. They are not: preference is ranked by
  calorie class, and `REQ-MOK-060` exists because that preference plus an over-strict waste
  condition left high-class resources standing. The sources are causally *upstream* of
  composition, which is exactly why the correction moves it. Being upstream of a quantity is
  not reading it.
* **Not** that nothing in the repository reads composition. Two things do, both listed in
  section 1, and the TUI's observer is one of them. `INT-MOK-010` separates observation from
  decision, and this file is evidence that the separation holds on the decision side.
* **Not** a guarantee about future sources. It is a measurement of the four that exist at this
  candidate. What generalizes is section 2: while `decide` takes only an observation, a new
  source inherits the same inability, and granting one composition means amending rule 3 in
  the open.

**RESULT: PASS — composition is computed in one function with two callers, both reporting
surfaces and neither on a decision path; a source is handed no reference that could reach
it; and none of the functions the four sources can transitively reach touches it.**
