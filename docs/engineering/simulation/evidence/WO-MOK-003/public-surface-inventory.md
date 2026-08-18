# WO-MOK-003 evidence: public surface inventory

`VER-MOK-003` requires the library target's public interface to be enumerated and compared against
`SPEC-MOK-002` rule 5, with any authorized item that was not added named explicitly. This is that
enumeration.

## Method

The inventory was produced two independent ways and the two agree.

1. **From rustdoc.** `cargo doc --no-deps --lib` renders one page per public item. Listing the
   generated pages under `target/doc/mokiterions/` gives 15 HTML files, of which `index.html` (the
   crate page) and `all.html` (the generated index) are not items. The remaining **13 pages are the
   13 public items**:

   ```
   cli/index.html                                simulation/index.html
   cli/constant.USAGE.html                       simulation/constant.CELLS_PER_TERRITORY.html
   cli/enum.Command.html                         simulation/struct.Config.html
   cli/fn.parse.html                             simulation/struct.Density.html
   fn.execute.html                               simulation/enum.Policy.html
                                                 simulation/struct.RunSummary.html
                                                 simulation/struct.Simulation.html
                                                 simulation/enum.TerminationReason.html
   ```

   Rustdoc is the right oracle here because it reports reachability, not syntax: an item declared
   `pub` inside a private module would not appear, and an item re-exported into a public path would.

2. **From the sources.** Grepping declared `pub` items in `src/lib.rs`, `src/cli.rs`, and
   `src/simulation.rs` yields the same 13 items plus their public members. `git diff master --
   src/simulation.rs` shows exactly seven added `pub` tokens in that file — one constant, one
   method, one enum, and four accessors — and no change to any `derive` attribute.

An earlier working note in this change recorded "15 public items". That was the rustdoc *page*
count. The item count is 13.

## Inventory

### Crate root — 3 items

| Item | Form | Classification |
| --- | --- | --- |
| `cli` | `pub mod` | namespace; declared by rule 3 |
| `simulation` | `pub mod` | namespace; declared by rule 3 |
| `execute` | `fn(args, &mut W, &mut E) -> u8` | pure function of its arguments; owns no state and holds nothing across calls |

### `cli` — 3 items

| Item | Form | Classification |
| --- | --- | --- |
| `USAGE` | `&'static str` constant | value |
| `Command` | `enum { Help, Run(Config) }` | value type; the payload is a `Config`, itself a value |
| `parse` | `fn(args) -> Result<Command, String>` | value constructor |

### `simulation` — 7 items

| Item | Public members | Classification |
| --- | --- | --- |
| `CELLS_PER_TERRITORY` | — | `usize` constant; a fixed world dimension |
| `Config` | fields `seed`, `tick_limit`, `policy`, `density`, `trace_actions` | value type, configuration inbound |
| `Density` | `DEFAULT`, `parse`, `resources_per_territory`, `Display` | value type with a private field; `DEFAULT` is a value, `parse` a value constructor, `resources_per_territory` and `Display` pure functions of the value |
| `Policy` | variants `Baseline` and `Reference`, `parse`, `Default` | value type |
| `RunSummary` | `reason`, `ticks`, `survivors`, `deaths` | opaque value type; all eight fields private; each accessor returns an owned copy |
| `Simulation` | `new`, `run` | opaque handle; no public field; `new` is a constructor, `run` drives the engine and returns a `RunSummary` by value |
| `TerminationReason` | variants `TickLimit` and `Extinction`, `Display` | value type |

Public members total: 5 `Config` fields, 2 `Command` variants, 2 `Policy` variants, 2
`TerminationReason` variants, `Density::{DEFAULT, parse, resources_per_territory}`, `Policy::parse`,
`Simulation::{new, run}`, and 4 `RunSummary` accessors.

## Diff against rule 5

### Already public, and unchanged — 9 of 9 rows present

| Rule 5 row | Present | Changed by this work order |
| --- | --- | --- |
| `cli::USAGE` | yes | no |
| `cli::Command` | yes | no |
| `cli::parse` | yes | no |
| `simulation::Config` | yes | no |
| `simulation::Density` | yes | gained the addition below; `DEFAULT` and `parse` untouched |
| `simulation::Policy` | yes | no |
| `simulation::RunSummary` | yes | gained the accessors below; all fields still private |
| `simulation::Simulation::new` | yes | no |
| `simulation::Simulation::run` | yes | no |

### Authorized additions — 4 of 5 rows taken in full, 1 taken in part

| Rule 5 addition | Added | Note |
| --- | --- | --- |
| `execute` | yes | moved from `src/main.rs` to `src/lib.rs`, signature and behavior unchanged |
| `simulation::TerminationReason` | yes | with its pre-existing `Display` impl |
| `Density::resources_per_territory` | yes | visibility only; the body is unchanged |
| `simulation::CELLS_PER_TERRITORY` | yes | visibility only; the expression is unchanged |
| `RunSummary` accessors | **4 of 6 values** | see below |

### Authorized items deliberately not added

Rule 5 states that its additions list "is a ceiling, not a checklist" and that "an item on it that no
relocated test requires must not be added". Two of the six values the accessor row authorizes are
therefore absent:

- **population per territory** — the `territory_a` and `territory_b` counts;
- **resource counts per territory by calorie class** — the `food_a` and `food_b` arrays.

No relocated test reads either. `tests/viability.rs` asserts on total survivors, not on their
distribution across territories, and no public-tier test asserts on resource counts at all. The two
fields stay private and the reason is recorded in the doc comment above `impl RunSummary` in
`src/simulation.rs`, so a later reader finds it at the point of decision rather than only here.

Had a relocated test needed them, adding them would have been in scope without an amendment. Not
needing them, adding them would have widened the maintained contract for nothing.

### Nothing outside the two lists

The 13 items and their members are a subset of rule 5's union. Specifically absent from the public
interface, and confirmed absent by rustdoc having generated no page for any of them: `Mokiterion`,
`Food`, `Coordinate`, `Direction`, `RelativeDirection`, `Territory`, `FoodClass`, `Action`,
`ActionResult`, `Observation`, `PerceivedFood`, `PerceivedMokiterion`, `SplitMix64`,
`DecisionEntropy`, `DecisionSource`, every simulation constant other than `CELLS_PER_TERRITORY`, and
every private method of `Simulation`. That is rule 6's list, item for item.

## Trait impls reachable through the interface

Making a type public makes its trait impls publicly reachable, so they belong in an honest inventory
even though rule 5's *Form* column names only `Display`.

| Type | Reachable impls | Introduced by this change |
| --- | --- | --- |
| `Command`, `Config`, `Density`, `Policy` | `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`; `Policy` also `Default`; `Density` also `Display` | no — all pre-existing on already-public types |
| `RunSummary` | `Debug`, `Clone`, `PartialEq`, `Eq` | no — pre-existing derives on an already-public type |
| `TerminationReason` | `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Display` | reachability is new; the impls are not |
| `Simulation` | none | — |

`TerminationReason` is the one type whose impls became reachable. It is a two-variant enum with no
payload, so `Debug`, `Clone`, `Copy`, `PartialEq` and `Eq` on it convey exactly what the variant
already conveys and grant no access to anything else. `git diff` confirms no `derive` attribute was
added or edited anywhere in this change.

`RunSummary`'s `Debug` renders its private field values as text, including the two counts
deliberately left without accessors. That is pre-existing behavior of an already-public type, not
something this work order introduced, and it is a read of values the summary line already prints. It
is recorded here rather than silently, and assessed in `boundary-review.md`.
