+++
id = "SPEC-MOK-001"
type = "specification"
title = "Minimum simulation foundation behavior"
status = "approved"
owners = ["technical owner"]
created = "2026-08-11"
updated = "2026-08-19"

[relations]
specifies = [
  "REQ-MOK-001",
  "REQ-MOK-002",
  "REQ-MOK-003",
  "REQ-MOK-004",
  "REQ-MOK-005",
  "REQ-MOK-006",
  "REQ-MOK-007",
  "REQ-MOK-008",
  "REQ-MOK-009",
  "REQ-MOK-010",
  "REQ-MOK-011",
  "REQ-MOK-012",
  "REQ-MOK-013",
  "REQ-MOK-014",
  "REQ-MOK-015",
  "REQ-MOK-018",
  "REQ-MOK-031",
  "REQ-MOK-032",
  "REQ-MOK-033",
  "REQ-MOK-034",
  "REQ-MOK-040",
]
+++

# Specification: Minimum simulation foundation behavior

## Scope

This specification defines the smallest local, in-memory, text-only Mokiterions simulation. It fixes the world layout, initial state, tick order, survival values, core actions, food behavior, bounded perception, per-Mokiterion identity and behavioral variation, deterministic decision sources, output, and termination needed to implement `CAP-MOK-001`, `CAP-MOK-002`, `CAP-MOK-006`, and `CAP-MOK-008`.

It does not define OpenAI integration, combat, social behavior, persistence, structured output, or a user interface. It defines one behavioral trait and the `fear` attribute, but no rule reads `fear`: threat response, retreat, surrender and encounter memory remain undefined.

This is the single behavior contract for the simulation core. It is amended in place rather than superseded, so that no two active specifications state conflicting survival or resource values.

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-11 | Original approved content for `CAP-MOK-001`. | Approved; implemented under `WO-MOK-001` and verified under `VREC-MOK-001`. |
| 2026-08-17 | Added bounded perception, the reference decision source, the `--policy` input, and the decision-source output line. Changed satiety decay from `2` to `1` and regeneration from one resource to `2`. | Approved 2026-08-17 by the repository owner acting as technical owner, together with `REQ-MOK-013`, `REQ-MOK-014`, `REQ-MOK-015`, and `WO-MOK-002`. |
| 2026-08-17 | Narrowed the *Explicitly unspecified decisions* entry on test organization: helper functions and the internal organization of a test module remain delegated, while crate target layout, the public interface, and test placement are governed by `SPEC-MOK-002`. No specified behavior changed. | Approved 2026-08-17 by the repository owner acting as technical owner, as required by `ADR-MOK-002` and as an approval precondition of `WO-MOK-003`. |
| 2026-08-17 | Replaced the fixed initial endowment and fixed territory capacity with a single `--density` input that sets the initial density, the capacity ceiling, and the replenishment target together. Corrected rule 5 case 1 from a fixed satiety threshold to the non-wasteful rule `REQ-MOK-015` already required. | Approved 2026-08-17 by the repository owner acting as technical owner, together with the amended `REQ-MOK-014`, on the measured evidence in `docs/engineering/simulation/evidence/WO-MOK-002/`. |
| 2026-08-17 | Extended the rule 5 non-waste test from case 1 to case 3, so a Mokiterion neither eats nor approaches a resource whose restoration would be clipped. Removed the unreachable fallback clause from case 1. Retracted the false claim that correcting case 1 alone removed the two-cell oscillation. | Approved 2026-08-17 by the repository owner acting as technical owner, together with the amended `REQ-MOK-014`, on the measured evidence in `docs/engineering/simulation/evidence/WO-MOK-002/density-curve.md`. |
| 2026-08-17 | Specified the content of the help text: an options block stating each accepted option's effect, its default where it has one, and its value constraint where it has one, with the stated defaults required to equal the applied defaults. Narrowed the *Explicitly unspecified decisions* entry on help text to alignment, width, and wrapping. No simulation behavior changed. | Approved 2026-08-17 by the repository owner acting as technical owner, together with `REQ-MOK-018`, `VER-MOK-004`, and `WO-MOK-004`, as an approval precondition of that work order. |
| 2026-08-19 | Individuality, under `CAP-MOK-006`. Nine provisions amended and one rule appended. *Scope* no longer excludes fear and individual traits and now names `CAP-MOK-006`, while recording that no rule reads `fear`. *Actors* names three sources. *Inputs* and *Help output* take the third `--policy` value `individual`. *State model / Mokiterion* gains `fear` among the bounded attributes, starting at `0`, and gains the trait. A new *Behavioral trait* subsection fixes `waste_tolerance`, its range `0..=100`, and its derivation from the seed and the identifier through a generator of its own. *Time and entropy* records that trait derivation is the one exception to the single shared stream. *Data and interface contracts* puts `waste_tolerance` on the observation and fixes `fear` in the `agent_initialized`, `survival_changed` and `action_trace` details, with the trait reported once in `agent_initialized`. Rule 1 places the derivation. Rule 3 carries the trait and states that `fear` is deliberately absent from the observation. Rule 7 states that the traced `fear` is the pre-update value. Rule 12 becomes *Survival decay and fear* and gains the update: `+10` when rule 3's perceived-Mokiterion list is non-empty, `-5` when it is empty, saturating at both bounds, reading that list and nothing else, consuming no entropy. Rule 5's accumulation paragraph gains a cross-reference to `REQ-MOK-034` and `VER-MOK-007`; **rule 5 is otherwise untouched and the reference source's behavior is unchanged**. Rule 19 is appended, specifying the trait-aware source and the tolerant test `S + R - 100 <= T * R / 100`, applied to eating and seeking alike, proposal-identical to rule 5 at `T = 0`. A *Behavioral rules* preamble states that rule order stops matching tick order for that one rule. | Approved 2026-08-19 by the repository owner acting as technical owner, together with `INT-MOK-006`, `CAP-MOK-006`, `REQ-MOK-031` through `REQ-MOK-034`, `VER-MOK-007` and `WO-MOK-007`. The implementation agent wrote the amended text under `WO-MOK-007` and chose the derivation's salt constant within the approved shape; it did not decide the substance. Appending rule 19 rather than renumbering thirteen rules, the trait's full range, the two `fear` step sizes and the driver were the owner's decisions of the same date, recorded in `WO-MOK-007`. |
| 2026-08-19 | Narrowed the `waste_tolerance` range from `0..=100` to `0..=40` in *Behavioral trait*, and with it the bounded selection the derivation takes. Rule 19's upper-bound note now reads the bound as `T = 40`, giving `S + R - 100 <= 2R/5`, and records that no reachable tolerance accepts restoration that is entirely clipped. The two *Acceptance examples* that cited the unreachable tolerances `60` and `58` are replaced by a medium-class pair at `34` and `33`, which also fixes the division as truncating, and a high-class pair at `40` and `39` at the bound. **No other provision changed: the derivation, the salt, the tolerant test's form, rule 19's case order, the `fear` provisions and rule 5 are all untouched, and the baseline and reference sources remain byte-identically unchanged.** | Approved 2026-08-19 by the repository owner acting as technical owner, on the measured evidence in `docs/engineering/simulation/evidence/WO-MOK-007/escalation.md`, in correction of `REQ-MOK-034`'s survivor floor missed on three of five declared seeds under `WO-MOK-007` stop condition 6. The first form of *Behavioral trait* named this amendment as the one to make on that evidence. The owner chose narrowing the range over amending the floor; the implementation agent measured the sweep, recommended the bound and wrote the amended text, and did not decide the substance. `REQ-MOK-031`, `REQ-MOK-033` and `REQ-MOK-034` need no amendment, each having delegated the range to this specification. |
| 2026-08-19 | Corrected the *Help output* sentence this work order's first amendment added. It read "The explanatory prose on the decision sources describes all three, and states which one is the default", which contradicted the same section's *Every default stated in the options block* paragraph — approved 2026-08-17 — under which each default is stated once and the prose is where the removed copy lived. The sentence now states that the prose describes all three sources and states neither a default nor a value constraint. **The three-source description is retained; only the default clause is withdrawn.** No behavior changed, and the implementation was already on the corrected side of the contradiction. | **OUTSTANDING** — written by the implementation agent under `WO-MOK-007` on 2026-08-19, approved by nobody. It is a correction to text the technical owner approved on this date, so it needs that owner's ratification. The defect was found by reading the amended section against the inherited test `cli::each_declared_default_is_stated_once`, which asserts that the explanatory prose contains no default at all; satisfying the withdrawn clause would have required relaxing an assertion `VREC-MOK-004` binds, which `WO-MOK-007` forbids. Recorded in `evidence/WO-MOK-007/completion-summary.md` §13 and checked in `evidence/WO-MOK-007/amendment-approvals.md`. |
| 2026-08-19 | Naming, under `CAP-MOK-008`. Five provisions. *Scope* names `CAP-MOK-008` and per-Mokiterion identity. *State model / Mokiterion* gains the name. A new *Name* subsection fixes the twelve names and their assignment to `M01` through `M12`, the character and length domain, the pairwise distinctness of the names and of their twelve first characters, and the three load-bearing properties: the assignment reads neither the seed nor the configuration, naming performs no draw against any generator, and nothing in the engine reads a name. It also records that a name is not carried on the rule 3 observation, for the reason `fear` is not. *Time and entropy* records that naming is not an exception to the single shared stream because it is not a draw at all. *Data and interface contracts* puts `name` first in the `agent_initialized` details, before `position`, reports it once and on no other record kind, and states that both the name's leading position and `waste_tolerance`'s trailing position are fixed because two test suites parse that record positionally. Rule 1 places the assignment at agent creation. **No other provision changed: no rule, no decision source, no constant, no floor, no attribute, no ordering and no exit code is touched, and every run's outcome and entropy sequence are unchanged.** | Approved 2026-08-19 by the repository owner acting as technical owner, together with `INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`, `REQ-MOK-041`, `VER-MOK-010` and `WO-MOK-010`. The twelve names and their assignment are the product owner's decision of the same date, recorded in `WO-MOK-010`; the name's position in the record and the decision that the observer sources it from the retained event stream rather than from a new public interface item are the technical owner's decisions of the same date, recorded there too. The implementation agent wrote the amended text under `WO-MOK-010` and did not decide the substance. |

The released implementation at commit `09c4e1a` conforms to the 2026-08-11 content. `VREC-MOK-001` remains the
commit-bound record of that earlier content.

The first 2026-08-17 amendment was implemented and measured under `WO-MOK-002`, and its viability claim failed:
zero survivors on every declared seed. The second amendment corrects that failure. Its reasoning is recorded in
`docs/engineering/simulation/evidence/WO-MOK-002/escalation.md`, and the two facts that drove it are that
regeneration yield had no measurable effect on survival because territory capacity was the real ceiling on
standing supply, and that near-global perception did not help either, because the binding constraint was travel
time against satiety drain during the regeneration ramp.

The second amendment was also implemented and measured, and its viability claim also failed, at three survivors
against a stated floor of five. The cause was a defect in the rule it introduced rather than in the density
design: the non-waste test governed eating but not seeking, so the two effects the amendment claimed to remove
were only half removed. The third amendment closes that gap and is the amendment the floor of eight rests on.
Applying the test to both cases raised the measured worst case at the default density from three survivors to
eight. It also removes a false statement that the second amendment left standing in this document.

## Actors and external systems

- The operator starts the process, selects a decision source, and reads standard output and standard error.
- A decision source proposes actions from bounded observations. Three exist: the random baseline source, the food-seeking reference source, and the trait-aware individual source.
- The simulation engine is the only authority that changes world state.
- There are no external systems or network calls.

## Inputs

The binary accepts:

```text
Mokiterions [--seed <u64>] [--ticks <u64>]
            [--policy <baseline|reference|individual>]
            [--density <percent>] [--trace-actions]
```

- `--seed` defaults to `0`.
- `--ticks` defaults to `100` and must be greater than zero.
- `--policy` selects the decision source and defaults to `reference`. Only `baseline`, `reference`, and `individual` are valid values. `individual` selects the trait-aware source of rule 19, which reads each Mokiterion's `waste_tolerance`; it is not the default, and whether it should become the default is deferred to a later governed decision under `REQ-MOK-033`.
- `--density` selects the resource density as a percentage of a territory's cells and defaults to `0.75`. It accepts a decimal value with at most two decimal places, so the smallest representable step is `0.01`. It must resolve to at least one resource per territory and must not exceed `100`.
- `--trace-actions` accepts no value, defaults to disabled, and enables one detailed action trace for every living-agent decision opportunity.
- Options may appear in any order and may appear at most once.
- `--help` prints usage and exits successfully without starting a simulation.
- Unknown, duplicate, missing, or invalid option values are invalid configuration.

### Help output

`--help` writes the usage text to standard output and exits `0`. The same usage text follows the
`configuration error:` line on standard error when configuration is invalid. There is one usage text and two paths
that emit it, so the content below is a property of that text and not of either path.

The usage text contains, in order: the synopsis block above; an options block; a statement that options may appear
in any order and at most once; and the explanatory prose on the decision sources and on what `--density` binds
together.

The explanatory prose on the decision sources describes all three. It states no default and no value constraint:
the options block below states each of those once, and the prose is where an earlier copy of them lived. The
options block contains one entry for each option the program accepts — `--seed`, `--ticks`, `--policy`,
`--density`, `--trace-actions`, and `--help`, in that order. Each entry states the option, its value placeholder
where it takes a value, and a short description of its effect that is intelligible without this specification at
hand. Each entry additionally states:

| Option | Stated default | Stated constraint |
|---|---|---|
| `--seed` | `0` | none |
| `--ticks` | `100` | must be greater than zero |
| `--policy` | `reference` | only `baseline`, `reference`, and `individual` are valid, which the value placeholder states |
| `--density` | `0.75` | at most two decimal places |
| `--trace-actions` | no value; the entry states that tracing is off unless the option is given | none |
| `--help` | none | none |

Every default stated in the options block is the value the program applies when the option is omitted. The two are
required to be equal, and that equality is verified rather than maintained by convention.

Each of these facts is stated once. Where the explanatory prose previously repeated a default or a value constraint
that the options block now states, the repetition is removed.

The options block states no behavior this specification does not state elsewhere. It restates the *Inputs* list for
the operator's benefit, and where the two differ this specification's *Inputs* list governs.

## Outputs

- Deterministic simulation events and the final summary are written to standard output.
- The selected decision source is reported exactly once on standard output, before agent processing begins, so that no run is ambiguous about which policy produced it.
- Detailed per-action trace lines are additionally written to standard output only when `--trace-actions` is enabled.
- Usage and configuration errors are written to standard error.
- Successful help or simulation completion exits with code `0`.
- Invalid configuration exits with code `2`.
- An unrecoverable runtime or output failure exits with code `1`.

## State model

### World

- Coordinates are integer pairs `(x, y)` where both values are in `0..=127`.
- Territory A contains `y` values `0..=63`.
- Territory B contains `y` values `64..=127`.
- Territory boundaries label state but do not block movement.

### Mokiterion

Each Mokiterion contains:

- a stable identifier from `M01` through `M12`;
- a name, fixed for the run and identical in every run, specified in *Name* below;
- a current coordinate;
- a derived current territory;
- integer `health`, `satiety`, `energy`, and `fear` values in `0..=100`;
- one behavioral trait, `waste_tolerance`, fixed for the run and specified in *Behavioral trait* below;
- a living or dead state.

`health`, `satiety`, and `energy` start at `100`; `fear` starts at `0`, so no Mokiterion begins afraid. `M01` through `M06` start in territory A and `M07` through `M12` start in territory B. Initial coordinates are selected without duplicate agent positions using seeded entropy.

Multiple agents may occupy the same coordinate after initialization. Agents do not block movement. An agent and a food resource may share a coordinate.

### Name

Each Mokiterion has one name. It carries no behavior: it exists so that an operator reading a run follows twelve
individuals rather than twelve serial numbers, and it is the only property of a Mokiterion this specification defines
that nothing in the engine reads.

- The name is taken from this table, by the identifier's number. The assignment is total, injective and fixed:

  | Identifier | Name | Identifier | Name |
  |---|---|---|---|
  | `M01` | `Zug` | `M07` | `Hozz` |
  | `M02` | `Krul` | `M08` | `Nurb` |
  | `M03` | `Quib` | `M09` | `Vonk` |
  | `M04` | `Sput` | `M10` | `Gorm` |
  | `M05` | `Trok` | `M11` | `Xob` |
  | `M06` | `Womp` | `M12` | `Drix` |

- The table holds exactly as many names as the world holds Mokiterions, which *Performance and capacity* fixes at
  twelve. A table longer or shorter than the population is a defect, not a spare.
- **The assignment reads neither the seed nor the configuration.** `M05` is `Trok` at every seed, at every density, under
  every decision source and at every tick limit. This is what makes a name usable as a label across runs and across
  retained evidence files.
- **Naming performs no draw, against the shared stream of *Time and entropy* or against any other generator.** It is a
  table lookup. This is a stronger property than *Behavioral trait*'s, which needs a generator of its own; naming needs
  none, so the shared stream cannot move and every run predating this revision is unchanged.
- The name is fixed at initialization and never changes. No tick, action, rejection, death or regeneration alters it.
- Every name consists only of characters in `A`–`Z` and `a`–`z`, and is between one and five characters long. The
  character restriction exists because the name is written unescaped into the comma-and-colon delimited details of *Data
  and interface contracts*; the length restriction exists because `SPEC-MOK-003` rule 4 renders the name in a
  fixed-width column.
- The twelve names are pairwise distinct, **and so are their twelve first characters** — `Z K Q S T W H N V G X D`. The
  first-character property is an obligation rather than an aesthetic: `SPEC-MOK-003` rule 2 derives a one-character map
  glyph from the name, and rule 2.5 requires every distinction carrying identity to survive the loss of colour. None of
  the twelve is `A`, `B`, `F` or `M`, the letters the output already uses for the two territory labels, the resource
  identifier prefix and the Mokiterion identifier prefix, so no name's initial can be read as one of those.
- No name equals any identifier this specification defines, so a name can never be read as an identifier.
- **Nothing reads a name.** No rule, no decision source, no validation path, no ordering, no tie-break and no
  termination condition consults it. Acting order is ascending *identifier* order under rule 2; ordering by name would
  reorder acting order and move every outcome, so the absence of a reader is a specified property and not an accident of
  the current implementation.

A name is not carried on the observation of rule 3, for the same reason `fear` is not: no decision source reads it, and a
value carried where nothing reads it is an inert field. It reaches an observer through the reported record instead.

The names are invented rather than borrowed from any living language's given-name stock. Which twelve words they are, and
which identifier each belongs to, is a product decision recorded in `WO-MOK-010`; it is not an implementation choice and
is not to be reordered.

### Behavioral trait

Each Mokiterion holds exactly one behavioral trait. It is what makes two Mokiterions in identical situations behave
differently, and it is the only per-Mokiterion variation this specification defines.

- The trait is `waste_tolerance`, an integer in `0..=40`. It is fixed at initialization and never changes during a
  run.
- It is derived from the run seed and the Mokiterion's identifier alone. For the Mokiterion whose identifier is
  `M<nn>`, construct a SplitMix64 generator whose initial state is `seed XOR (nn * 0xC2B2AE3D27D4EB4F)`, using
  wrapping multiplication over 64 bits, where `nn` is the identifier's number `1` through `12`. Take one unbiased
  bounded selection in `0..=40` from that generator. That value is the trait.
- **The derivation uses a generator of its own, constructed for the derivation and discarded after it. It neither
  reads nor advances the shared stream of *Time and entropy*.** This is the property that keeps every run predating
  this revision unchanged, and it is asserted directly against the shared stream's state rather than inferred from
  output.
- Integer arithmetic only. No floating-point value participates in the derivation or in any use of the trait.
- The trait is derived under every decision source and at every density, because it is a property of the Mokiterion
  rather than of the source. Only rule 19 reads it.

The range is `0..=40`, narrowed on measured evidence from the `0..=100` this subsection specified in its first form.
Under rule 19 a Mokiterion at tolerance `T` eats a high-class resource restoring `50` satiety up to satiety
`50 + T/2`, so `T = 0` reproduces rule 5 exactly and `T = 40` eats such a resource up to satiety `70`.

The first form chose the full range on the argument that a wider range gives stronger divergence evidence, and recorded
that narrowing it was this subsection's amendment to make if measurement showed the range cost survivors. Measurement
showed exactly that. At `0..=100` the trait-aware source left seven, four and seven survivors on declared seeds `0`,
`42` and `123`, missing `REQ-MOK-034`'s floor of eight on three of five, and over fifty consecutive seeds its mean was
`7.40` — below the floor rather than near it.

**The upper half of the old range was not a second strategy but a worse one.** A Mokiterion at satiety `80` that eats a
high-class resource gains `20` satiety and destroys `30`: it removes a resource from a world whose regeneration is
conditional and capped, and arrives at satiety `100`, which a resource of any class would have reached. Tolerance past
the point where that trade turns bad buys its holder nothing, and the measured deaths follow it — on four of the five
declared seeds the Mokiterions that died held a higher mean tolerance than those that lived. Narrowing the range
therefore removes a dominated region of the trait space rather than removing individuality.

`0..=40` is chosen from the distribution and not from the declared seeds. Upper bounds were swept first on the declared
five and then on fifty consecutive seeds, and **the declared-five result is not monotonic in the bound** — `0..=50`
misses the floor while both `0..=60` and `0..=40` hold — so any bound chosen because it passes five seeds is chosen by
luck. On the fifty-seed distribution `0..=40` leaves a mean of `9.94` and falls below eight on 4% of seeds, against the
reference source's own 6%, while `0..=60` falls below on 12%.

The narrowing does not collapse the range toward the reference source, which `REQ-MOK-034` forbids. At `T = 40` the
tolerant test admits a satiety window twenty points wider than rule 5's for high-class resources, twelve wider for
medium and six wider for low, so divergence remains available in every calorie class; and `T = 0` still reproduces rule
5 exactly, which is what `REQ-MOK-033` pins. A range narrowed until the divergence evidence could no longer be produced
would defeat `REQ-MOK-033`, and this one does not.

### Food

Each resource has a stable identifier, coordinate, territory, and calorie class:

| Class | Satiety restored | Energy restored |
|---|---:|---:|
| Low | 15 | 5 |
| Medium | 30 | 10 |
| High | 50 | 20 |

### Resource density

Density is the single input that governs how much food a territory holds. It is stated as a percentage of that
territory's cells.

- A territory contains `128 * 64 = 8192` cells. Density is expressed relative to one territory, not to the whole world.
- The percentage is parsed as an exact integer count of hundredths of a percent, so `0.75` becomes `75`. Floating-point arithmetic is not used anywhere in this conversion, because `REQ-MOK-009` requires byte-identical reproducibility.
- The resource count per territory is `hundredths * 8192 / 10000`, using integer division that truncates toward zero. So `0.15%` yields `12`, `0.75%` yields `61`, and `1.50%` yields `122`.
- A density that truncates to zero resources is invalid configuration, because a territory with no resources can never regenerate under rule 15 and the run would be predetermined.

The resolved resource count binds **three** roles at once, and this is the substance of the input rather than an
incidental consequence:

1. the number of resources each territory holds at initialization;
2. the maximum capacity of a territory;
3. the level that regeneration replenishes toward.

Binding all three is deliberate. When capacity alone was raised, a territory still began near-empty and climbed
toward its ceiling over hundreds of ticks, and the population died during that ramp before the ceiling had any
effect. A single density value that sets the starting level, the ceiling, and the target removes the ramp: a
territory begins at its intended density and regeneration restores it toward that same density after consumption.

Initial resources are assigned classes by cycling through low, medium, and high in that order, so composition is
an even three-way split up to a remainder of at most two. Initial coordinates are unique within a territory and
are drawn from currently food-free coordinates in that territory using seeded entropy, evaluated after each
resource already placed.

Because the resource count determines how many coordinate draws initialization performs, two runs at different
densities consume the entropy stream differently and are therefore different worlds rather than the same world
holding more food. Runs are comparable only with runs at the same density, exactly as they are comparable only
with runs under the same decision source.

### Perception

- The perception radius is `16` cells.
- Distance is Chebyshev distance: the greater of the absolute differences in `x` and `y`. An entity is perceived when that distance is at most `16`.
- Relative direction is one of `north`, `north_east`, `east`, `south_east`, `south`, `south_west`, `west`, `north_west`, determined by the sign of the coordinate differences. A non-zero difference in both axes yields a diagonal direction.
- Perception does not stop at a territory boundary and is not blocked by other Mokiterions.
- Perception is read-only, consumes no entropy, and mutates nothing.

### Time and entropy

- Time is an integer tick beginning at `0`; agent processing begins on tick `1`.
- One explicit SplitMix64 pseudo-random stream, seeded from `--seed`, supplies all initialization, decision-source, and regeneration entropy. Trait derivation is the one exception and lies outside it: *Behavioral trait* fixes a generator of its own that leaves this stream's state unmoved. Naming is not an exception because it is not a draw at all: *Name* is a table lookup and consumes from no generator. The three decision sources consume from this stream at different rates, so a run under one source is comparable only with runs under the same source.
- Random selection uses a stable candidate order and an unbiased bounded selection method.

## Behavioral rules

Rules 1 through 18 are stated in tick order. **Rule 19 is not**: it specifies a third decision source and belongs
beside rule 5, but it is appended so that rules 1 through 18 keep the numbers the specifications, the verification
contracts, the verification records, the retained evidence and the source comments already cite. Inserting it beside
rule 5 would have renumbered thirteen rules and invalidated every one of those citations. The cost is that rule order
stops matching tick order for exactly one rule, which this paragraph exists to state rather than leave to be
discovered.

1. **Initialization order.** Create the entropy stream, world, initial food, and agents in that order. Each agent's name is assigned as *Name* specifies at the point that agent is created, by table lookup on its identifier's number, drawing nothing. Each agent's `waste_tolerance` is derived as *Behavioral trait* specifies at the point that agent is created, from the seed and its identifier; because the derivation uses a generator of its own, it cannot move the shared stream's draw sequence and the placement draws are unaffected. Emit initialization events only after the complete initial state is valid, then emit the selected decision source exactly once before tick processing begins.
2. **Tick start.** Increment the tick once, then consider living agents in ascending identifier order.
3. **Observation.** For each considered agent, the engine creates a read-only observation containing tick, identity, position, territory, health, satiety, energy, `waste_tolerance`, co-located food, perceived food, perceived Mokiterions, valid cardinal moves, and the complete list of currently valid core action proposals. The trait is carried on the observation so that a decision source can read the acting Mokiterion's trait without reaching into authoritative state. `fear` is deliberately **not** carried: no rule and no decision source reads it.

   Perceived food lists every resource within the perception radius, including co-located resources, each with identifier, class, relative direction, and distance, ordered by ascending distance and then by identifier. Perceived Mokiterions lists every other living Mokiterion within the radius, each with identifier, relative direction, and distance, ordered by ascending distance and then by identifier. The observer never appears in its own perceived-Mokiterion list, and dead Mokiterions never appear. Co-located food remains reported separately and unchanged.
4. **Baseline decision.** Candidate proposals use this stable order: `wait`; `sleep` when energy is below `100`; `eat` for each co-located resource in identifier order; and valid `move` actions ordered north, east, south, west. The baseline consumes one entropy selection and returns one candidate.
5. **Reference decision.** The reference source returns the first applicable candidate in this order:

   1. `eat` the co-located resource of the highest calorie class whose satiety restoration would not exceed the attribute maximum of `100`, breaking ties by lowest identifier;
   2. `sleep` when energy is below `20`;
   3. `move` one cell toward the nearest perceived resource at a distance greater than zero whose satiety restoration would not exceed the attribute maximum of `100`, breaking ties by highest calorie class and then by lowest identifier. Move on the horizontal axis while the perceived direction has an easterly or westerly component, otherwise on the vertical axis. When that move is invalid, use the other axis;
   4. `move` as a search step, selected from the valid `move` actions in the order north, east, south, west using one entropy selection.

   Only the search step of case 4 consumes entropy. Cases 1 through 3 consume none. The reference source never proposes `wait`: a blind Mokiterion searches rather than stands still.

   Cases 1 and 3 apply the same non-waste test, and applying it to both is the point of each. `REQ-MOK-015` requires consuming "when consuming it is not wasteful": eating a resource whose restoration would be clipped by the attribute maximum discards the remainder, and it can drive a territory to zero resources, which permanently ends its regeneration under rule 15. Case 3 applies the identical test to *seeking*, so a Mokiterion never walks toward a resource it would decline on arrival. When every perceived resource would be clipped, case 3 does not apply and the Mokiterion searches under case 4, because approaching a resource it will not eat accomplishes less than looking for one it will.

   Two earlier forms of these rules were defective. The record is kept because the second defect was found only by measurement, after this specification had asserted the opposite.

   The first form gated case 1 on a fixed satiety threshold of `50` alone. Satiety above `50` could never be spent, because eating again always required decaying back to `50` first, so the range 51 to 100 was buffer that could not fund travel. The non-waste test removes that dead buffer.

   The second form applied the non-waste test to case 1 only, and it produced a stable two-cell oscillation: a Mokiterion standing on a resource it declined stepped off under case 3, immediately perceived that same resource as the nearest resource at a distance greater than zero, and stepped back. **An earlier revision of this specification claimed that correcting case 1 removed this effect. That claim was false and is retracted.** For high-class resources the non-waste condition is satisfied only at satiety of at most `50`, numerically identical to the threshold it replaced, so the richest third of the resource table oscillated exactly as before — measured at 35.7% of agent-ticks against the 12.2% an unbiased cardinal walk produces on the same world. Extending the test to case 3 removes the cause rather than the symptom, because the resource just left is excluded from targeting for exactly as long as it would be declined. The measured residual is 10.6%, below the random-walk rate.

   One consequence of the corrected rule is specified rather than accidental. A high-class resource restores `50` satiety, so it is both eatable and approachable only at satiety of at most `50`. High-class resources are therefore consumed less often than low or medium and accumulate against the territory capacity that density fixes: measurement at the default density puts high class at 45 of 61 resources in a territory by tick 1,000, against a balanced initial third. This is accepted at the 1,000-tick horizon `REQ-MOK-014` states, where the corrected rule raises the measured worst case from three survivors to eight. It is recorded as a known long-horizon effect rather than a defect, and addressing it is out of scope for this revision. Rule 19's per-Mokiterion tolerance is expected to reduce this accumulation, because twelve tolerances replace one shared threshold; `REQ-MOK-034` and `VER-MOK-007` measure the effect at the 1,000-tick and 10,000-tick horizons under that source. **No obligation is stated on the result in either direction, and the reference source's own behavior is unchanged.**

   Because cardinal movement costs one tick per cell on either axis, the axis rule in case 3 changes the shape of an approach path but never its total cost.
6. **Validation.** The engine validates the returned proposal against current authoritative state. A rejected proposal consumes the action opportunity, produces a rejection result, and causes no action-specific mutation.
7. **Optional action trace.** When `--trace-actions` is enabled, emit exactly one `action_trace` line after validation and any valid action-specific mutation, but before survival decay. The line contains the tick, agent identifier, proposed action, `accepted` or `rejected` status, result or rejection reason, position, territory, health, satiety, energy, and `fear`. Because this rule places the trace before survival decay, every attribute it reports is the pre-decay value, and `fear` is therefore the value held **before** rule 12's update for this tick. The trace and the `survival_changed` line for one tick disagree by one step for `fear` exactly as they already do for satiety and energy. When the flag is disabled, emit no `action_trace` lines. Trace configuration never changes entropy consumption or simulation state.
8. **Move.** A valid move changes one coordinate by one cell in a cardinal direction. Crossing `y=63/64` updates the derived territory and emits a crossing event. Movement has no additional energy cost in this foundation.
9. **Eat.** A valid eat selects one co-located resource by identifier, removes it, and restores the class values in the food table. Attribute values are capped at `100`.
10. **Sleep.** Sleep restores `20` energy, capped at `100`, before survival decay. It does not move the agent or consume food.
11. **Wait.** Wait causes no action-specific mutation.
12. **Survival decay and fear.** After the action opportunity, subtract `1` satiety and `1` energy using saturation at zero. If either resulting value is zero, subtract `5` health using saturation at zero. Then update `fear` from the rule 3 observation built for this agent's decision opportunity in this tick: add `10` when that observation's perceived-Mokiterion list holds at least one entry, and subtract `5` when it is empty. Both directions saturate within `0..=100`, and saturation is a normal outcome rather than an error. The update reads that observation and nothing else, performs no new perception, consumes no entropy, and is emitted in the `survival_changed` line together with the three decay values.

    **The driver adds no distance constant.** It is rule 3's own list — non-empty or empty — and rule 3's list is bounded by the perception radius of `16`. A narrower threshold was considered and rejected on arithmetic: with twelve Mokiterions on a 128×128 grid the expected number of others inside a Chebyshev box of radius `r` is `11 * (2r+1)^2 / 16384`, which is about `0.73` at radius `16`, `0.19` at `8`, and `0.05` at `4`. A four-cell threshold would hold `fear` at `0` on roughly nineteen agent-ticks in twenty, making it verifiable only through constructed states — the inert attribute `SPEC-MOK-003` rule 4.5 refused, reached by a different route. Reusing the radius removes a constant instead of adding one. `REQ-MOK-032` records the arithmetic.

    Rule 3's list is what one agent perceived at its own decision opportunity, so the within-tick asymmetry rule 2 establishes applies here too: two mutually adjacent Mokiterions may not both register the encounter on the same tick, because the earlier one observed the later one's position before it moved. Territory boundaries do not attenuate the driver, since perception already crosses `y=63/64`. **No rule reads `fear`.** It is computed, bounded, reported and otherwise inert; a consumer is a later governed change.
13. **Death.** When health becomes zero, mark the agent dead and emit one death event. Dead agents receive no later observations, decisions, actions, traces, or survival updates.
14. **Regeneration timing.** After all scheduled agents are processed, each territory receives one regeneration opportunity on ticks divisible by `10`.
15. **Regeneration condition.** A territory holding at least one resource and fewer than its capacity adds `2` resources, or fewer when fewer free capacity slots remain. A territory with zero resources adds none and emits a skipped-regeneration event, so permanent local depletion remains reachable at every density. A territory already at capacity adds none and emits a skipped-regeneration event.
16. **Regeneration selection.** Each new class is selected uniformly from low, medium, and high. Each coordinate is selected from currently food-free coordinates in that territory, evaluated after any resource added earlier in the same opportunity. All selections use the shared entropy stream, first class then coordinate, for each added resource in turn.
17. **Termination.** After regeneration, terminate if all agents are dead or the configured tick limit has been reached. Extinction takes precedence when both conditions occur on the same tick.
18. **Final summary.** Emit the termination reason, elapsed ticks, survivors, deaths, population by current territory, and remaining food by territory and calorie class exactly once.
19. **Trait-aware decision.** Selected by `--policy individual`. This rule is appended rather than placed beside rule 5, for the reason the *Behavioral rules* preamble states; in tick order it occupies rule 5's position. Let `T` be the acting Mokiterion's `waste_tolerance` as carried on the rule 3 observation. The source returns the first applicable candidate in this order:

    1. `eat` the co-located resource of the highest calorie class that passes the tolerant test below, breaking ties by lowest identifier;
    2. `sleep` when energy is below `20`;
    3. `move` one cell toward the nearest perceived resource at a distance greater than zero that passes the tolerant test, breaking ties by highest calorie class and then by lowest identifier, using rule 5 case 3's axis rule unchanged: the horizontal axis while the perceived direction has an easterly or westerly component, otherwise the vertical axis, and the other axis when that move is invalid;
    4. `move` as a search step, selected from the valid `move` actions in the order north, east, south, west using one entropy selection.

    **The tolerant test.** A resource restoring `R` satiety to a Mokiterion at satiety `S` passes when `S + R <= 100`, or else when `S + R - 100 <= T * R / 100`, using integer division that truncates toward zero. Integer arithmetic only.

    **The test governs cases 1 and 3 alike**, for the reason rule 5 gives for applying its own test to both: a Mokiterion that declined the resource underfoot but still targeted it would step off, perceive it again as the nearest resource at a distance greater than zero, and step back. Applying the tolerant test to eating alone would reintroduce the two-cell oscillation rule 5 records as its second defect.

    At `T = 0` the second clause requires `S + R - 100 <= 0`, which is rule 5's non-waste test, so **this source is proposal-identical to rule 5 for every observation at tolerance zero**. At the range's upper bound of `T = 40` the second clause reads `S + R - 100 <= 2R/5`, so a high-class resource is eaten up to satiety `70`, a medium-class one up to `82` and a low-class one up to `91`. No reachable tolerance accepts restoration that is entirely clipped; the first form of *Behavioral trait* permitted that at `T = 100` and the narrowing recorded there removed it. For a high-class resource restoring `50` the test admits satiety up to `50 + T/2`.

    Only the search step of case 4 consumes entropy. Cases 1 through 3 consume none. The source never proposes `wait`: a blind Mokiterion searches rather than stands still. Because the tolerance decides how often case 4 is reached, this source consumes the shared stream at a rate of its own, so runs under it are comparable only with runs under it, at the same seed and density.

    A proposal from this source is validated under rule 6 with no relaxation. Reading the trait grants the source no authority: it proposes, and the engine decides.

## Error and recovery behavior

- Configuration is fully validated before state initialization.
- Initialization is atomic: failure produces no simulation events or partial run.
- Invalid action proposals are recoverable rejections and do not terminate the run.
- Integer arithmetic saturates at attribute bounds and never wraps.
- A standard-output write failure terminates the process with runtime exit code `1`; no successful summary is claimed.

## Data and interface contracts

The decision boundary exposes only:

```text
Observation -> ProposedAction
```

`Observation` contains copied or immutable values, including the acting Mokiterion's `waste_tolerance` and the perceived resources and perceived Mokiterions of rule 3. A perceived entry carries an identifier, a relative direction, a distance, and, for a resource, its calorie class. It carries no reference to the entity it describes, so a decision source can read a perceived entity but cannot reach it. `ProposedAction` is one of:

```text
Wait
Sleep
Eat { food_id }
Move { direction: North | East | South | West }
```

The decision source never receives a mutable world, agent, resource collection, event log, or engine handle.

Event lines use stable key-value fields in this order:

```text
tick=<number> subject=<identifier> event=<event-type> result=<stable-details>
```

Stable core event types are `world_initialized`, `food_initialized`, `agent_initialized`, `decision_source_selected`, `food_consumed`, `food_regenerated`, `food_regeneration_skipped`, `territory_crossed`, `survival_changed`, `agent_died`, and `simulation_ended`. Optional per-action lines use `action_trace`. The details of `decision_source_selected` name the active source as `baseline`, `reference`, or `individual`.

Three record kinds carry the attributes, and their stable details are fixed in this order:

```text
tick=0 subject=<mokiterion-id> event=agent_initialized result=name:<letters>,position:<x:y>,territory:<A|B>,health:<number>,satiety:<number>,energy:<number>,fear:<number>,waste_tolerance:<number>
tick=<number> subject=<mokiterion-id> event=survival_changed result=health:<a>-><b>,satiety:<a>-><b>,energy:<a>-><b>,fear:<a>-><b>
```

`fear` follows the other three attributes in every record that carries them, and `waste_tolerance` is reported once, in `agent_initialized`. The trait is reported and never restated, because it cannot change during a run. The name is reported once, in `agent_initialized`, for the same reason, and it is reported on no other record kind.

**The name is the first detail and `waste_tolerance` remains the last, and both positions are fixed rather than
incidental.** Two test suites parse this record positionally — one asserts `fear` and `waste_tolerance` as an adjacent
pair, the other reads the trait by splitting on the record's final field — so appending the name would have required
changing assertions that `VREC-MOK-005` and `VREC-MOK-007` bind, which `WO-MOK-010` forbids. Placing it first also puts
identity at the head of the record, where a reader looks for it. A later field is appended after `waste_tolerance` only
by an amendment that states what it does to those parsers.

An action trace uses the same leading fields and stable details in this order:

```text
tick=<number> subject=<mokiterion-id> event=action_trace result=proposal:<action>,status:<accepted|rejected>,detail:<result-or-reason>,position:<x:y>,territory:<A|B>,health:<number>,satiety:<number>,energy:<number>,fear:<number>
```

The final line begins with `summary` and reports fields in the order defined by rule 18. Stable details must not contain wall-clock timestamps, absolute paths, pointer values, or unordered collection formatting.

## Security and privacy properties

- The foundation reads no credentials and performs no network access.
- Output contains only simulation configuration and state.
- Invalid input is treated as data and never interpreted as code or a filesystem path.

## Performance and capacity

- The foundation supports exactly twelve agents and a 128 by 128 world.
- Per-tick work is bounded by the twelve agents, the current food collection, perception within the bounded radius, and emitted events.
- State is held in memory and no persistence is required.

## Observability

Every initialization, decision-source selection, food consumption or regeneration result, territory crossing, survival attribute change, death, and termination is emitted in authoritative processing order. When action tracing is enabled, every living-agent decision opportunity additionally emits one ordered action trace. Identical runs with identical trace configuration produce byte-identical standard output.

## Compatibility and migration

There is no prior data or interface compatibility obligation. Future output or model interfaces may replace this foundation only through later approved artifacts.

## Examples and counterexamples

- A move from `(10, 63)` south to `(10, 64)` is valid and crosses from territory A to B.
- A move north from `(10, 0)` is invalid and leaves position unchanged.
- Eating a resource at another coordinate is invalid.
- At the default density of `0.75%`, each territory resolves to `61` resources and begins holding exactly that many. If territory A has been eaten down to `58` on tick `10`, it regenerates two and reaches `60`; if it holds `60`, it adds one and reaches capacity `61`; if it holds `61`, it adds none; if it holds zero, it remains at zero forever.
- A density of `0.15%` resolves to `12` resources per territory, reproducing the resource count of the original 2026-08-11 world, and a density of `1.50%` resolves to `122`.
- A density of `0.01%` resolves to `8192 * 1 / 10000 = 0` resources and is invalid configuration.
- Sleeping at energy `90` raises energy to `100`, then tick decay leaves it at `99`.
- An agent at satiety `1`, energy `50`, and health `5` waits, reaches satiety `0`, loses `5` health, dies, and is not processed again.
- A Mokiterion at `(40, 20)` perceives a medium resource at `(44, 20)` as `east` at distance `4`, and does not perceive a resource at `(100, 20)`.
- The same Mokiterion under the reference source, with energy of at least `20`, proposes `move east` on each tick until it is co-located, then proposes `eat` for that resource because a medium resource restores `30` and satiety is at most `70`. Under the baseline source it may propose any valid action.
- A Mokiterion under the reference source perceiving no resource proposes a search `move`, never `wait`. At energy `19` it proposes `sleep` instead, and resumes searching once energy is at least `20`.
- A Mokiterion standing on a high-calorie resource at satiety `80` does not eat it, because `80 + 50` exceeds `100`. It leaves the resource in place, and the resource still counts toward the territory's regeneration condition. Standing on a low-calorie resource at satiety `80` it does eat, because `80 + 15` fits.
- A Mokiterion standing only on a high-calorie resource at satiety `45` eats it despite the waste, because satiety is at most `50` and starving beside food is worse.
- A Mokiterion at `(40, 20)` perceiving only a resource at `(43, 17)` moves `east` first, because the perceived direction `north_east` has an easterly component and the horizontal axis is taken first.
- A tick beginning with twelve living agents emits exactly twelve `action_trace` lines when `--trace-actions` is enabled and none when it is disabled.
- A Mokiterion whose `waste_tolerance` is `0`, under `--policy individual`, proposes on every decision opportunity exactly what the reference source proposes for the same observation.
- A Mokiterion at satiety `80` standing on a medium-class resource declines it under `--policy reference`, because `80 + 30` exceeds `100`. Under `--policy individual` it eats it when its `waste_tolerance` is at least `34`, because `80 + 30 - 100 = 10` and `34 * 30 / 100 = 10` under truncating division, and declines it at `33`, because `33 * 30 / 100 = 9`. The pair either side of `34` is what fixes the division as truncating rather than rounding.
- A Mokiterion at satiety `70` standing on a high-class resource declines it under `--policy reference`, because `70 + 50` exceeds `100`. Under `--policy individual` it eats it only at the range's upper bound of `40`, because `70 + 50 - 100 = 20` and `40 * 50 / 100 = 20`, and declines it at `39`, because `39 * 50 / 100 = 19`. A high-class resource at satiety `80` is declined at every reachable tolerance, which is the effect the narrowing in *Behavioral trait* was made to produce.
- A Mokiterion whose rule 3 observation lists one other living Mokiterion, and one whose observation lists four, both gain `10` fear: neither count, nor distance, nor direction within the radius scales the change.
- A Mokiterion at `fear` `3` whose observation lists nobody falls to `0` and stays there while it stays alone; the `survival_changed` line still reports `fear:0->0`.
- A Mokiterion perceiving another at Chebyshev distance `16` gains fear; at `17` it does not, because the other is not in the rule 3 observation at all.

## Explicitly unspecified decisions

- Rust file and private type names.
- Choice of collection types where iteration is explicitly sorted before observable use.
- Internal error type layout and message wording, except for exit codes and required clarity.
- Test helper functions, and the internal organization of a test module within the source file that owns it. Crate
  target layout, the public interface, and which tier a test belongs to are not unspecified: they are governed by
  `SPEC-MOK-002`.
- The private organization of the trait-aware source's code, and whether it shares helper code with the reference source, provided rule 19's proposals and rule 5's are exactly as specified. What is not unspecified: the trait's name, range and derivation, the tolerant test, the `fear` driver and its two step sizes, and the third `--policy` value, all fixed above.
- Column alignment, column widths, line wrapping, and cosmetic whitespace in the help text. Which options the
  options block describes, and which defaults and constraints it states, are not unspecified: they are fixed by the
  *Help output* section above.
