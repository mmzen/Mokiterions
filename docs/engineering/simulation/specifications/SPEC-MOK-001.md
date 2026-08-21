+++
id = "SPEC-MOK-001"
type = "specification"
title = "Minimum simulation foundation behavior"
status = "approved"
owners = ["technical owner"]
created = "2026-08-11"
updated = "2026-08-20"

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
  "REQ-MOK-051",
  "REQ-MOK-052",
  "REQ-MOK-053",
  "REQ-MOK-054",
  "REQ-MOK-055",
  "REQ-MOK-056",
  "REQ-MOK-057",
  "REQ-MOK-058",
  "REQ-MOK-059",
  "REQ-MOK-060",
]
+++

# Specification: Minimum simulation foundation behavior

## Scope

This specification defines the smallest local, in-memory, text-only Mokiterions simulation. It fixes the world layout, initial state, tick order, survival values, core actions, food behavior, bounded perception, per-Mokiterion identity and behavioral variation, deterministic decision sources, contact and conflict between Mokiterions, output, and termination needed to implement `CAP-MOK-001`, `CAP-MOK-002`, `CAP-MOK-006`, `CAP-MOK-008`, and `CAP-MOK-010`.

It does not define OpenAI integration, persistence, structured output, or a user interface. It defines one behavioral trait, the `fear` attribute, a decision source that reads it, and the seven targeted actions `approach`, `avoid`, `threaten`, `attack`, `fight`, `retreat` and `surrender`.

What remains undefined here is a boundary of `CAP-MOK-010` rather than of this specification, and is stated as that capability's exclusion rather than as a gap: cooperation, sharing, grouping, alliance, reputation, mating and kinship; territorial defence, `y=63/64` still labelling state and nothing more; any memory of an encounter surviving past the remembering Mokiterion's own next decision opportunity; and any perception of another Mokiterion's `health`, `energy` or `waste_tolerance`, so that no Mokiterion reads another's strength before acting on it.

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
| 2026-08-19 | Individuality, under `CAP-MOK-006`. Nine provisions amended and one rule appended. *Scope* no longer excludes fear and individual traits and now names `CAP-MOK-006`, while recording that no rule reads `fear`. *Actors* names three sources. *Inputs* and *Help output* take the third `--policy` value `individual`. *State model / Mokiterion* gains `fear` among the bounded attributes, starting at `0`, and gains the trait. A new *Behavioral trait* subsection fixes `waste_tolerance`, its range `0..=100`, and its derivation from the seed and the identifier through a generator of its own. *Time and entropy* records that trait derivation is the one exception to the single shared stream. *Data and interface contracts* puts `waste_tolerance` on the observation and fixes `fear` in the `agent_initialized`, `survival_changed` and `action_trace` details, with the trait reported once in `agent_initialized`. Rule 1 places the derivation. Rule 3 carries the trait and states that `fear` is deliberately absent from the observation. Rule 7 states that the traced `fear` is the pre-update value. Rule 12 becomes *Survival decay and fear* and gains the update: `+10` when rule 3's perceived-Mokiterion list is non-empty, `-5` when it is empty, saturating at both bounds, reading that list and nothing else, consuming no entropy. Rule 5's accumulation paragraph gains a cross-reference to `REQ-MOK-034` and `VER-MOK-010`; **rule 5 is otherwise untouched and the reference source's behavior is unchanged**. Rule 19 is appended, specifying the trait-aware source and the tolerant test `S + R - 100 <= T * R / 100`, applied to eating and seeking alike, proposal-identical to rule 5 at `T = 0`. A *Behavioral rules* preamble states that rule order stops matching tick order for that one rule. | Approved 2026-08-19 by the repository owner acting as technical owner, together with `INT-MOK-006`, `CAP-MOK-006`, `REQ-MOK-031` through `REQ-MOK-034`, `VER-MOK-010` and `WO-MOK-010`. The implementation agent wrote the amended text under `WO-MOK-010` and chose the derivation's salt constant within the approved shape; it did not decide the substance. Appending rule 19 rather than renumbering thirteen rules, the trait's full range, the two `fear` step sizes and the driver were the owner's decisions of the same date, recorded in `WO-MOK-010`. |
| 2026-08-19 | Narrowed the `waste_tolerance` range from `0..=100` to `0..=40` in *Behavioral trait*, and with it the bounded selection the derivation takes. Rule 19's upper-bound note now reads the bound as `T = 40`, giving `S + R - 100 <= 2R/5`, and records that no reachable tolerance accepts restoration that is entirely clipped. The two *Acceptance examples* that cited the unreachable tolerances `60` and `58` are replaced by a medium-class pair at `34` and `33`, which also fixes the division as truncating, and a high-class pair at `40` and `39` at the bound. **No other provision changed: the derivation, the salt, the tolerant test's form, rule 19's case order, the `fear` provisions and rule 5 are all untouched, and the baseline and reference sources remain byte-identically unchanged.** | Approved 2026-08-19 by the repository owner acting as technical owner, on the measured evidence in `docs/engineering/simulation/evidence/WO-MOK-010/escalation.md`, in correction of `REQ-MOK-034`'s survivor floor missed on three of five declared seeds under `WO-MOK-010` stop condition 6. The first form of *Behavioral trait* named this amendment as the one to make on that evidence. The owner chose narrowing the range over amending the floor; the implementation agent measured the sweep, recommended the bound and wrote the amended text, and did not decide the substance. `REQ-MOK-031`, `REQ-MOK-033` and `REQ-MOK-034` need no amendment, each having delegated the range to this specification. |
| 2026-08-19 | Corrected the *Help output* sentence this work order's first amendment added. It read "The explanatory prose on the decision sources describes all three, and states which one is the default", which contradicted the same section's *Every default stated in the options block* paragraph — approved 2026-08-17 — under which each default is stated once and the prose is where the removed copy lived. The sentence now states that the prose describes all three sources and states neither a default nor a value constraint. **The three-source description is retained; only the default clause is withdrawn.** No behavior changed, and the implementation was already on the corrected side of the contradiction. | **Ratified 2026-08-19 by the repository owner acting as technical owner**, in the closing review of `WO-MOK-010` recorded in `evidence/WO-MOK-010/closing-review.md`. It was **OUTSTANDING** until that act: written by the implementation agent under `WO-MOK-010` on 2026-08-19 and approved by nobody, being a correction to text the technical owner had approved on the same date. The defect was found by reading the amended section against the inherited test `cli::each_declared_default_is_stated_once`, which asserts that the explanatory prose contains no default at all; satisfying the withdrawn clause would have required relaxing an assertion `VREC-MOK-004` binds, which `WO-MOK-010` forbids. Recorded in `evidence/WO-MOK-010/completion-summary.md` §13 and checked in `evidence/WO-MOK-010/amendment-approvals.md`. |
| 2026-08-19 | Naming, under `CAP-MOK-008`. Five provisions. *Scope* names `CAP-MOK-008` and per-Mokiterion identity. *State model / Mokiterion* gains the name. A new *Name* subsection fixes the twelve names and their assignment to `M01` through `M12`, the character and length domain, the pairwise distinctness of the names and of their twelve first characters, and the three load-bearing properties: the assignment reads neither the seed nor the configuration, naming performs no draw against any generator, and nothing in the engine reads a name. It also records that a name is not carried on the rule 3 observation, for the reason `fear` is not. *Time and entropy* records that naming is not an exception to the single shared stream because it is not a draw at all. *Data and interface contracts* puts `name` first in the `agent_initialized` details, before `position`, reports it once and on no other record kind, and states that both the name's leading position and `waste_tolerance`'s trailing position are fixed because two test suites parse that record positionally. Rule 1 places the assignment at agent creation. **No other provision changed: no rule, no decision source, no constant, no floor, no attribute, no ordering and no exit code is touched, and every run's outcome and entropy sequence are unchanged.** | Approved 2026-08-19 by the repository owner acting as technical owner, together with `INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`, `REQ-MOK-041`, `VER-MOK-011` and `WO-MOK-011`. The twelve names and their assignment are the product owner's decision of the same date, recorded in `WO-MOK-011`; the name's position in the record and the decision that the observer sources it from the retained event stream rather than from a new public interface item are the technical owner's decisions of the same date, recorded there too. The implementation agent wrote the amended text under `WO-MOK-011` and did not decide the substance. |
| 2026-08-20 | Contact, conflict and society, under `CAP-MOK-010`. Thirteen provisions amended, of which seven are appended rules, and the frontmatter's `specifies` gains `REQ-MOK-051` through `REQ-MOK-060`. *Scope* names `CAP-MOK-010` and no longer excludes interaction between Mokiterions; cooperation, memory of encounters and perceived relative strength stay excluded, restated as boundaries of this capability rather than of this document. *Actors*, *Inputs* and *Help output* name a fourth decision source, `social`, with `reference` still the default. *State model / Mokiterion* gains **no attribute** and one item of transient state, the record of attacks suffered since that Mokiterion's previous decision opportunity; a new *Contact* subsection fixes the contact relation at Chebyshev distance `1` between living Mokiterions, recomputed from positions and never stored. *Name*'s justification for a name's absence from the observation is replaced, because it rested on `fear`'s absence and that no longer holds; the name stays off the observation. *Time and entropy* and the *Behavioral rules* preamble state each appended rule's position in tick order, in an eight-row table, since rule order stopped matching tick order at rule 19. Rule 3 carries `fear` and the suffered-attack record and its valid-proposal list is **untouched**, so rule 6 becomes the complete statement of what may be proposed. *Data and interface contracts* fixes the eleven-kind action contract, the three added event types with their field lists, and the `action_trace` line's conditional `suffered` field. Rule 6 extends validation to targeted proposals against the target's authoritative state, rule 7 fixes the trace-before-clearing order, rule 12's closing "No rule reads `fear`" is **inverted** and the composition of its two writers stated, and rule 13 states that combat death uses its existing path, event and finality. **Rules 20 to 26 are appended after rule 19 and not inserted**, on `WO-MOK-010`'s precedent and for its stated reason: contact, targeted actions, combat resolution, threat, surrender, the suffered-attack window, and the `social` source with its five ordered branches. Rule 5's accumulation paragraph and rule 19's tolerance test now state `REQ-MOK-060`'s ceiling as an obligation where they stated none, and name the two places the correction may be made; **the numeric form of the corrected waste condition is a second, later amendment**, named here as the one to make, on the 2026-08-19 *Behavioral trait* precedent where an approved rule named in advance the amendment to make on measurement. The measured 45 of 61 figure is **retained** as the state that obligation ends. | Approved 2026-08-20 by the repository owner acting as technical owner, in the **single act this amendment's own ordering requires**: together with `REQ-MOK-051` through `REQ-MOK-060`, `VER-MOK-016` and `WO-MOK-016`. The act is single because this amendment's `specifies` relation is what makes those ten requirements approvable at all — without it `validate` raises `E007` on every one of them and `preflight --phase start` raises `W016`, both measured on 2026-08-20 and recorded in that work order. Implementation begins after this act and not before. It is stated in full in `WO-MOK-016`'s *Required amendments* section. The implementation agent wrote the text and did not decide the substance: the eleven values it fixes were the owners' decisions of 2026-08-19 and 2026-08-20, and the three the validation did not supply were taken on 2026-08-20, all recorded in that work order's *Decision record*. Eight consequences the text derived rather than decided are named in that work order's *Required amendments* section; the owner took the four of them that were genuinely open before approving, and those four are recorded in its decision table with the alternatives declined. |
| 2026-08-20 | Rule 21's co-location fallback and rule 6's paragraph on it are corrected to name **both `avoid` and `retreat`**, under `REQ-MOK-052`. Both named `avoid` alone. `retreat` reaches distance zero on identical terms: the two verbs are one move away from the target, differ from `approach` only in sign, resolve through the same axis rule, and `retreat`'s precondition — the target named in the actor's suffered-attack record — requires no contact but does not exclude it. `retreat` is the more reachable of the two, because rule 26 has no branch proposing `avoid` at distance zero while its answer branch selects `retreat` for a defender at `fear` of at least `30` whose attacker struck from the same cell and has not moved. **Nothing about the fallback's behavior changes**, for either verb: north, then east, south, west, drawing no entropy, and `approach` at distance zero stays a rejection. This is a correction of an incomplete enumeration rather than of an obligation, and the previously-stated form is retained above in the amended paragraphs so that a reader of `VREC-MOK-012` can see what was understated. | Approved 2026-08-20 by the repository owner acting as technical owner, on the discrepancy being put to them with the alternatives — recording it as residual uncertainty in `VER-MOK-016`, or restricting `retreat` in the code to match the text literally. Both were declined: the second would reject a co-located retreat, which is the case rule 26 most often produces. The implementation agent found the discrepancy while enumerating rule 21's paths for `REQ-MOK-059` and did not decide it. The implementation is unchanged by this amendment and was already what the corrected text states; `WO-MOK-016`'s derived consequence 4 is corrected there in the same act, its parenthetical having asserted that rule 26 never reaches this case. |
| 2026-08-20 | Rule 26's branch order and engagement threshold, under `REQ-MOK-057`'s first amendment. **Six provisions of that rule alone; no other rule is touched, and rule 12 in particular is not.** The five-branch list becomes six: rule 19's case 3 is hoisted out of branch 5 into a new branch 3 ahead of the two social branches, so that food perceived outranks company perceived and company outranks aimless wandering, and rule 19's case 4 stays alone at branch 6. The engagement threshold moves from `30` to `95` in both places it is read. The *normative ordering* paragraph now forbids delegating branch 3 as well as branch 2 to rule 19 whole. The *three constants* paragraph states `95`. The *composition with rule 23* paragraph is rewritten: it claimed that one threat moves a calm Mokiterion from `attack` to `threaten`, which was true at `30` and is false at `95`, and the composition now runs through branch 1's answer thresholds, which are unchanged. A new paragraph states what `95` means and records that it is an arbitrary constant rather than a derived one. The *entropy discipline* paragraph renumbers, and the differential equality with rule 19 **widens** from "no Mokiterion perceived" to "case 1, 2 or 3 applies", which is strictly more observations. | Approved 2026-08-20 by the repository owner acting as product owner and technical owner, on the measured evidence in `evidence/WO-MOK-016/escalation.md`. The rule as first approved was unsatisfiable against `REQ-MOK-058`: `fear` is driven by perception at radius `16` and engagement required contact at radius `1` below `fear` `30`, so the gate closed on the third perceiving tick, no approach could complete, nothing struck anything and branch 1 never fired. Seventeen variants were measured across three levers. The owner chose between three packages, and selected the one that **leaves rule 12 as Phase 2 approved it** — declining an earlier selection to rescope rule 12 to contact, which recovered lethality but was measured to put `REQ-MOK-058`'s floor beyond reach at every gate value, and which would have invalidated `WO-MOK-010`'s measured `fear` distribution. `REQ-MOK-058`'s floor of five is ratified unchanged in the same act. |

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
- A decision source proposes actions from bounded observations. Four exist: the random baseline source, the food-seeking reference source, the trait-aware individual source, and the social source of rule 26, which is the only one that reads `fear` and the only one that proposes any targeted action.
- The simulation engine is the only authority that changes world state.
- There are no external systems or network calls.

## Inputs

The binary accepts:

```text
Mokiterions [--seed <u64>] [--ticks <u64>]
            [--policy <baseline|reference|individual|social>]
            [--density <percent>] [--trace-actions]
```

- `--seed` defaults to `0`.
- `--ticks` defaults to `100` and must be greater than zero.
- `--policy` selects the decision source and defaults to `reference`. Only `baseline`, `reference`, `individual`, and `social` are valid values. `individual` selects the trait-aware source of rule 19, which reads each Mokiterion's `waste_tolerance`; it is not the default, and whether it should become the default is deferred to a later governed decision under `REQ-MOK-033`. `social` selects the source of rule 26, which reads `fear` and the suffered-attack record and is the only source that proposes a targeted action; it is not the default, and it is not proposed as one, because the survivor floor `REQ-MOK-058` states for it is three below the floor `REQ-MOK-014` states for the default.
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

The explanatory prose on the decision sources describes all four. It states no default and no value constraint:
the options block below states each of those once, and the prose is where an earlier copy of them lived. The
options block contains one entry for each option the program accepts — `--seed`, `--ticks`, `--policy`,
`--density`, `--trace-actions`, and `--help`, in that order. Each entry states the option, its value placeholder
where it takes a value, and a short description of its effect that is intelligible without this specification at
hand. Each entry additionally states:

| Option | Stated default | Stated constraint |
|---|---|---|
| `--seed` | `0` | none |
| `--ticks` | `100` | must be greater than zero |
| `--policy` | `reference` | only `baseline`, `reference`, `individual`, and `social` are valid, which the value placeholder states |
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

Multiple agents may occupy the same coordinate after initialization. Agents do not block movement. An agent and a food resource may share a coordinate. Neither statement is changed by contact: two Mokiterions in contact still do not block each other and may still stand on one cell.

Each Mokiterion additionally holds one item of **transient state**, the record of attacks suffered since its own previous decision opportunity. It is listed apart from the attributes above because it is not one of them:

- **It is not a bounded `0..=100` attribute and is not reported as one.** No `agent_initialized` or `survival_changed` field carries it, it has no maximum in that range, and it is not subject to saturating arithmetic. It is a list of what happened, not a level.
- **It is per-Mokiterion and never per-pair.** Nothing in this specification stores a relationship between two Mokiterions. The record belongs to the Mokiterion that suffered the attacks and names the attackers; there is no matching record on any attacker, and no structure keyed by a pair.
- **It opens when an attack lands and closes at the sufferer's own next decision opportunity**, which is rule 25's subject. It is cleared then whether or not the Mokiterion answered, and whether or not it was still alive to answer.
- **It starts empty at initialization**, so nothing about it depends on the seed, and it holds nothing at all under `baseline`, `reference` or `individual`, none of which can attack.
- It carries, for each attack, the attacking Mokiterion's identifier and the damage that attack dealt. Its ordering is the order the attacks landed, which is rule 2's ascending-identifier order among the attackers.

It is transient in the strict sense: no state derived from it survives the opportunity that clears it, so this specification defines no encounter memory. That exclusion is `CAP-MOK-010`'s and *Scope* states it.

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

A name is not carried on the observation of rule 3, because no decision source reads it and a value carried where nothing
reads it is an inert field. It reaches an observer through the reported record instead.

**This reason once ran through `fear` and no longer can.** The sentence above read "for the same reason `fear` is not",
which held while rule 12's closing sentence held; rule 12 now has a reader and rule 3 carries `fear`, so the shared
reason is gone and the name's own reason has to stand alone. It does: `REQ-MOK-041` obliges that nothing reads a name,
and the absence of a reader here is therefore an obligation rather than a stage this specification has not reached yet.
The name stays off the observation, and what changed is the argument for it and not the obligation.

The names are invented rather than borrowed from any living language's given-name stock. Which twelve words they are, and
which identifier each belongs to, is a product decision recorded in `WO-MOK-011`; it is not an implementation choice and
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

### Contact

- The contact radius is `1` cell. Two Mokiterions are **in contact** when both are living and the Chebyshev distance between their coordinates is at most `1`.
- The distance is the same Chebyshev distance *Perception* defines, computed by the same rule. Contact adds a radius constant and no second notion of distance, so a cell diagonally adjacent is in contact exactly as an orthogonally adjacent one is, and two Mokiterions sharing a coordinate are in contact at distance `0`.
- **Contact is recomputed from current positions and never stored.** No field on a Mokiterion, no collection in the world and no entry on an observation records that two Mokiterions are in contact. It is a predicate over two positions, evaluated when a rule asks.
- **Contact is symmetric; acting on it is not.** If `M03` is in contact with `M09` then `M09` is in contact with `M03`, because the distance is. What is asymmetric is when each of them may act on the fact, which rule 2's ascending-identifier order fixes and rule 25 states the consequence of.
- Contact requires both Mokiterions living, so a Mokiterion is never in contact with a dead one and never with itself.
- Contact does not stop at a territory boundary. Two Mokiterions one cell apart across `y=63/64` are in contact, in different territories, and this specification states no consequence of the boundary for them.
- Evaluating contact is read-only, consumes no entropy, and mutates nothing.

### Time and entropy

- Time is an integer tick beginning at `0`; agent processing begins on tick `1`.
- One explicit SplitMix64 pseudo-random stream, seeded from `--seed`, supplies all initialization, decision-source, and regeneration entropy. Trait derivation is the one exception and lies outside it: *Behavioral trait* fixes a generator of its own that leaves this stream's state unmoved. Naming is not an exception because it is not a draw at all: *Name* is a table lookup and consumes from no generator. The four decision sources consume from this stream at different rates, so a run under one source is comparable only with runs under the same source.
- **No resolution of a targeted action draws from this stream or from any other generator.** Contact, damage, the threat's `fear` increase, the surrender transfer and every targeted move are computed from current state by integer arithmetic, so the stream's position after a tick is a function of the decisions taken and never of the encounters resolved. The social source of rule 26 takes at most one draw per opportunity, never for a social branch, and only through rule 19's own search step; rule 26 states that discipline as a consequence of its branch order.
- Random selection uses a stable candidate order and an unbiased bounded selection method.

## Behavioral rules

Rules 1 through 18 are stated in tick order. **Rules 19 through 26 are not**, and each is appended for the same reason:
rules 1 through 18 keep the numbers the specifications, the verification contracts, the verification records, the
retained evidence and the source comments already cite. Inserting rule 19 beside rule 5 would have renumbered thirteen
rules and invalidated every one of those citations; inserting seven more would have renumbered them again. The cost is
that rule order stops matching tick order for eight rules, which this paragraph exists to state rather than leave to be
discovered, and each one's position in tick order is stated here:

| Rule | Subject | Position in tick order |
|---|---|---|
| 19 | Trait-aware decision | Rule 5's position: a decision, taken between rule 3's observation and rule 6's validation |
| 20 | Contact | No position. It is a predicate over two positions, evaluated whenever a rule asks, and it never runs on its own |
| 21 | Targeted actions | Rule 8's through rule 11's position: the application of a validated proposal, after rule 6 and before rule 7 |
| 22 | Combat resolution | Inside rule 21, when the applied action is `attack` or `fight` |
| 23 | Threat | Inside rule 21, when the applied action is `threaten` |
| 24 | Surrender | Inside rule 21, when the applied action is `surrender` |
| 25 | The suffered-attack window | Twice: it opens inside rule 22, at the attacker's turn; it closes at the sufferer's own turn, after rule 7's trace line and before rule 12 |
| 26 | The social decision source | Rule 5's position, as rule 19 |

Two consequences of the table are worth stating plainly. **Rule 21 and everything under it run inside one Mokiterion's
turn**, so a Mokiterion acted on by a lower-identified one has its state changed between its own rule 3 observation of
the previous tick and its own next one. And **rule 25's two halves are the only place in this specification where one
rule's effects are separated by another Mokiterion's whole cycle**, which is what makes the response a deferred proposal
rather than a resolution.

1. **Initialization order.** Create the entropy stream, world, initial food, and agents in that order. Each agent's name is assigned as *Name* specifies at the point that agent is created, by table lookup on its identifier's number, drawing nothing. Each agent's `waste_tolerance` is derived as *Behavioral trait* specifies at the point that agent is created, from the seed and its identifier; because the derivation uses a generator of its own, it cannot move the shared stream's draw sequence and the placement draws are unaffected. Emit initialization events only after the complete initial state is valid, then emit the selected decision source exactly once before tick processing begins.
2. **Tick start.** Increment the tick once, then consider living agents in ascending identifier order.
3. **Observation.** For each considered agent, the engine creates a read-only observation containing tick, identity, position, territory, health, satiety, energy, `fear`, `waste_tolerance`, the suffered-attack record, co-located food, perceived food, perceived Mokiterions, valid cardinal moves, and the complete list of currently valid core action proposals. The trait is carried on the observation so that a decision source can read the acting Mokiterion's trait without reaching into authoritative state.

   **`fear` and the suffered-attack record are carried, and this replaces the refusal that stood here for two revisions.** The withheld form read "`fear` is deliberately **not** carried: no rule and no decision source reads it", and it was correct for as long as that was true. `REQ-MOK-054` now obliges both fields to be on the observation and `REQ-MOK-057` obliges the reader that makes them non-inert: the social source of rule 26 branches on `fear` and answers from the record. Neither field is inert and neither is carried speculatively. The `fear` carried is the value standing at the start of this agent's opportunity, which is the value after the previous tick's rule 12 update plus any threat applied by an earlier-acting Mokiterion in this tick; rule 26 states where it comes from and rule 23 states what may have moved it.

   **The list of currently valid proposals is unchanged, and no targeted action ever appears in it.** It enumerates the core proposals of `REQ-MOK-005` — `wait`, `sleep`, `eat` for each co-located resource, and the valid cardinal moves — exactly as it did before this revision. The consequence is specified rather than incidental: **this rule no longer enumerates everything a decision source may legitimately propose.** A source may propose any of rule 21's seven targeted actions without that action appearing in this list, and such a proposal is not thereby invalid; rule 6 is the complete statement of what may be proposed and is the only place a targeted proposal is judged. A reader who takes this list as the whole contract will be wrong about the social source.

   The reason the list is not extended is that extending it would move a decision source that has nothing to do with this change. Rule 4's baseline consumes one entropy selection over the length of this list, so a longer list moves that selection, and every run ever recorded under `baseline` would diverge. `REQ-MOK-060` holds those runs byte-identical and `CAP-MOK-010` excludes any change to what `baseline` proposes. Making the list's growth conditional on the selected policy was considered and declined, because it would make this rule's output a function of the configuration rather than of world state; `WO-MOK-016` records that decision.

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

   One consequence of the corrected rule is specified rather than accidental. A high-class resource restores `50` satiety, so it is both eatable and approachable only at satiety of at most `50`. High-class resources are therefore consumed less often than low or medium and accumulate against the territory capacity that density fixes: measurement at the default density puts high class at 45 of 61 resources in a territory by tick 1,000, against a balanced initial third. That measurement was accepted at the 1,000-tick horizon `REQ-MOK-014` states, where the corrected rule raises the measured worst case from three survivors to eight, and it was recorded as a known long-horizon effect rather than a defect while addressing it stayed out of scope. **It is no longer out of scope, and an obligation is now stated where this paragraph previously stated none.** `REQ-MOK-060` states it: at tick 1,000, at the default density, under the reference source, the trait-aware source of rule 19 and the social source of rule 26, **no calorie class holds more than half of any territory's standing resources**, on every declared verification seed. The 45 of 61 is the state that obligation ends. It is a ceiling and not a floor: real drift away from the balanced initial third stays permitted, and a source that prefers calories should still leave more high class standing than low. Rule 19's per-Mokiterion tolerance was expected to reduce this accumulation, because twelve tolerances replace one shared threshold, and `REQ-MOK-034` and `VER-MOK-010` measured the effect at the 1,000-tick and 10,000-tick horizons under that source; the obligation is nonetheless stated on the composition itself rather than on that source's contribution to it, because a ceiling met under one of three sources is not a property of the world.

   **Where the correction is made, and where it may not be.** The condition that produces the accumulation is the non-waste test of case 1 and case 3 above, and rule 19's tolerant form of it. Those two are the reference source's and the trait-aware source's proposal logic, and relaxing either changes what those two sources choose while leaving `baseline` untouched, because rule 4's candidate list offers `eat` for each co-located resource under no waste condition at all and no relaxation of a waste condition can reach it. The correction is therefore made in this rule's condition, in rule 19's tolerant test, or in both. It is **not** made in rule 16's uniform class selection, in rule 15's regeneration amount, in rule 9's eat effect, or in the food table's restoration values: each of those is world behavior under every policy, so each would change what a `baseline` run does and diverge every capture taken before it, and rule 16's would additionally move the shared stream's consumption. This is the surface `REQ-MOK-060` permits, and it is stated here because an amendment to this paragraph is what bounds the amendment that follows it.

   **The sentence this paragraph used to close with is retracted rather than dropped.** It read "No obligation is stated on the result in either direction, and the reference source's own behavior is unchanged." Both halves are now false: the obligation above is stated, and this source's own behavior moves, which is the divergence `REQ-MOK-060` requires attributed to this correction and to nothing else. What survives unchanged is `baseline`, and only `baseline`.

   **The numeric form of the corrected condition is a later amendment to this specification, and this paragraph names it as the one to make.** Which of the two permitted mechanisms is used, and by how much, is decided on measurement under `WO-MOK-016`: the ceiling is fixed in advance and the relaxation that reaches it is not, and a figure written here ahead of the sweep would be a guess this specification would then have to defend against its own evidence. This is the precedent *Behavioral trait* set on 2026-08-19, where its first form named the narrowing to make on `VER-MOK-010`'s evidence and the measured amendment followed. Until that amendment lands, case 1 and case 3 apply `S + R <= 100` exactly as written above, rule 19 applies its tolerant test exactly as written there, and the two obligations the relaxation is held against are `REQ-MOK-014`'s survivor floor of eight of twelve under this source and `REQ-MOK-034`'s under rule 19's: a correction that meets the ceiling by starving the population is not a correction.

   Because cardinal movement costs one tick per cell on either axis, the axis rule in case 3 changes the shape of an approach path but never its total cost.
6. **Validation.** The engine validates the returned proposal against current authoritative state. A rejected proposal consumes the action opportunity, produces a rejection result, and causes no action-specific mutation.

   **For a targeted proposal the engine validates against the target's authoritative state as well as the actor's**, and a rejected targeted proposal mutates **neither** Mokiterion. The absence of mutation is an obligation about two agents rather than one, which is a strengthening of this rule and not a relaxation of it.

   **This rule is the complete statement of what may be proposed.** Rule 3's list of currently valid proposals does not carry targeted actions and is not the gate: a verb absent from that list is not thereby invalid, and a verb present in it is not thereby applied. Validation is this rule's judgement alone, taken against authoritative state at the moment of application, and it never consults the observation the source read. A source cannot widen what it may do by what it was shown, and cannot narrow it either.

   Every targeted proposal is checked in this order, and the first unmet condition is the rejection reason:

   1. the target exists, lives, and is not the actor — for all seven verbs;
   2. the target is perceived, at a Chebyshev distance of at most the perception radius of `16` — for `approach` and `avoid`;
   3. the target is in contact, at a Chebyshev distance of at most `1` — for `threaten`, `attack` and `fight`;
   4. the target is named in the actor's suffered-attack record — for `fight`, `retreat` and `surrender`;
   5. the resulting move is a valid cardinal move under rule 8 — for `approach`, `avoid` and `retreat`.

   **`fight` carries two preconditions and not one.** It answers an unanswered attack, so condition 4 applies; it is also a strike, resolving through the same function `attack` resolves through, so condition 3 applies as well. An attacker that struck and then stepped out of contact cannot be fought, and the rejection names contact. `retreat` and `surrender` require no contact: retreating from an attacker that has moved away is a move away from a known position, and a surrender is a transfer rather than a reach.

   **`avoid` and `retreat` against a co-located target are not rejections.** "Away from distance zero" has no direction, and rejecting it would make co-location an inescapable state; the appended rule under rule 21 fixes the direction both take. **Amended 2026-08-20**: this paragraph named `avoid` alone, and `retreat` reaches distance zero on the same terms — the two verbs are one move away from the target, differing from `approach` only in sign, and `retreat`'s preconditions require no contact but do not exclude it. A defender struck from its own cell by an attacker that has not moved is exactly the case rule 26's answer branch selects `retreat` for.

   Where a targeted move's preferred axis is invalid the other axis is used, on rule 5 case 3's precedent, and where both are invalid the proposal is rejected as an invalid move — rule 8's existing reason, reached by a targeted verb.
7. **Optional action trace.** When `--trace-actions` is enabled, emit exactly one `action_trace` line after validation and any valid action-specific mutation, but before survival decay. The line contains the tick, agent identifier, proposed action, its target where the action has one, `accepted` or `rejected` status, result or rejection reason, position, territory, health, satiety, energy, `fear`, and — only where it is non-empty — the suffered-attack record. Because this rule places the trace before survival decay, every attribute it reports is the pre-decay value, and `fear` is therefore the value held **before** rule 12's update for this tick. The trace and the `survival_changed` line for one tick disagree by one step for `fear` exactly as they already do for satiety and energy. When the flag is disabled, emit no `action_trace` lines. Trace configuration never changes entropy consumption or simulation state.

   **Targeted actions are traced on the same terms as core ones**: one line per decision opportunity, after validation and any valid mutation, before survival decay, whether the proposal was applied or rejected. A targeted proposal names its target in a `target` field of its own, present only on those lines, as *Data and interface contracts* fixes.

   **The line also reports the suffered-attack record, and it reports it before the record is cleared.** This is this rule's existing principle applied once more rather than a second convention beside it: the same rule already fixes that the traced `fear` is the value held before rule 12's update, so the traced record is likewise the record the source read. Clearing first would empty the field on exactly the lines it exists to explain — every line where an answer was proposed, and every line where a Mokiterion carrying an unanswered attack proposed something else instead. The alternative was declined for that reason, and it was a real alternative: `position`, `territory`, `health`, `satiety` and `energy` on this line are all read *after* the action applies, so clearing first would have been the uniform choice.

   **The record's clearing is positioned identically whether or not the flag is set.** The record closes when the opportunity is taken, under `REQ-MOK-054`, and that point does not move with trace configuration; the trace reads the record on its way past. A clearing placed *after* an emission that only sometimes happens would make `--trace-actions` change simulation state, which the sentence above forbids and which no evidence taken with the flag off would catch.
8. **Move.** A valid move changes one coordinate by one cell in a cardinal direction. Crossing `y=63/64` updates the derived territory and emits a crossing event. Movement has no additional energy cost in this foundation.
9. **Eat.** A valid eat selects one co-located resource by identifier, removes it, and restores the class values in the food table. Attribute values are capped at `100`.
10. **Sleep.** Sleep restores `20` energy, capped at `100`, before survival decay. It does not move the agent or consume food.
11. **Wait.** Wait causes no action-specific mutation.
12. **Survival decay and fear.** After the action opportunity, subtract `1` satiety and `1` energy using saturation at zero. If either resulting value is zero, subtract `5` health using saturation at zero. Then update `fear` from the rule 3 observation built for this agent's decision opportunity in this tick: add `10` when that observation's perceived-Mokiterion list holds at least one entry, and subtract `5` when it is empty. Both directions saturate within `0..=100`, and saturation is a normal outcome rather than an error. The update reads that observation and nothing else, performs no new perception, consumes no entropy, and is emitted in the `survival_changed` line together with the three decay values.

    **The driver adds no distance constant.** It is rule 3's own list — non-empty or empty — and rule 3's list is bounded by the perception radius of `16`. A narrower threshold was considered and rejected on arithmetic: with twelve Mokiterions on a 128×128 grid the expected number of others inside a Chebyshev box of radius `r` is `11 * (2r+1)^2 / 16384`, which is about `0.73` at radius `16`, `0.19` at `8`, and `0.05` at `4`. A four-cell threshold would hold `fear` at `0` on roughly nineteen agent-ticks in twenty, making it verifiable only through constructed states — the inert attribute `SPEC-MOK-003` rule 4.5 refused, reached by a different route. Reusing the radius removes a constant instead of adding one. `REQ-MOK-032` records the arithmetic.

    Rule 3's list is what one agent perceived at its own decision opportunity, so the within-tick asymmetry rule 2 establishes applies here too: two mutually adjacent Mokiterions may not both register the encounter on the same tick, because the earlier one observed the later one's position before it moved. Territory boundaries do not attenuate the driver, since perception already crosses `y=63/64`.

    **`fear` is read.** For two phases this paragraph closed "**No rule reads `fear`.** It is computed, bounded, reported and otherwise inert; a consumer is a later governed change" — and named the change that would invert it. This is that change. The source of rule 26 reads `fear` at every decision opportunity under `--policy social`, obliged by `REQ-MOK-057`, and rule 23's threat writes another Mokiterion's `fear`, obliged by `REQ-MOK-055`. `fear` is no longer inert, and this rule is no longer the whole of its life. **This rule's own computation is unchanged**: the same `+10` when rule 3's perceived-Mokiterion list is non-empty and `-5` when it is empty, the same driver, the same saturation, the same single observation read, the same absence of any entropy draw.

    **`fear` now has two writers within one tick, and their composition is stated rather than left to be derived.** A Mokiterion's `fear` may be raised by `30` under rule 23 at a threatener's turn and moved by `10` or `5` under this rule at its own turn, in whichever order rule 2's ascending identifier order produces. Both writes saturate within `0..=100`, neither is suppressed, and neither is deferred: there is no accumulation of pending `fear`, and no write is skipped because another happened first. A Mokiterion threatened twice and perceiving somebody rises by `30`, `30` and `10` in one tick, saturating at `100` if it gets there.

    **Which value a source reads follows from that, and rule 26 states it where a reader will need it.** This rule writes at the acting Mokiterion's own turn, after its action; rule 3 builds the observation at that same turn, before it. So the `fear` a source reads is the value standing after the *previous* tick's write under this rule, plus any threat applied to it by an earlier-acting Mokiterion in the current tick. Every threshold in rule 26 is sensitive to that position, which is why it is written down twice rather than derived once.
13. **Death.** When health becomes zero, mark the agent dead and emit one death event. Dead agents receive no later observations, decisions, actions, traces, or survival updates.

    **`health` may now reach zero through rule 22's damage as well as through rule 12's decay, and such a death uses this path, this event and this finality.** Death stays one concept: there is no second death, no combat-specific event, and no difference in what a dead Mokiterion is. What differs is only when it happens — rule 22 resolves inside another Mokiterion's turn, so a Mokiterion may die at a point in the tick where it has not yet acted, and it then receives no opportunity that tick or ever. Its suffered-attack record dies with it, unread. That a given death was combat's is recoverable from the stream through `attack_resolved`'s `target_died` field, which is why this event gains no cause detail.

    A dead Mokiterion is also no longer a valid target: rule 6's first condition rejects every targeted proposal naming it, including a `fight` answering the attack it made while it lived.
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

    **This test is one of the two places `REQ-MOK-060`'s correction may be made**, the other being rule 5's non-waste condition, and rule 5's accumulation paragraph states the obligation, the permitted surface and the reason the surface stops there. Which mechanism is used — relaxing rule 5's condition, raising the tolerance floor this test evaluates against, or both — is decided on measurement under `WO-MOK-016` and written into this rule and rule 5 by the amendment that paragraph names. Two things about that amendment are fixed here rather than left to it.

    **A tolerance floor belongs to this test and not to the trait's range.** `REQ-MOK-060` permits the correction in rule 5's condition and in this rule's tolerant form, and in nothing else; *Behavioral trait*'s range of `0..=40` and its derivation are outside that permission. So a floor, if it is chosen, raises the tolerance this test evaluates against and leaves the derived value, its range and its reported `waste_tolerance` alone. A Mokiterion's reported trait therefore continues to mean what it means today.

    **The `T = 0` identity is not automatically preserved, and the amendment must say what becomes of it.** The identity above holds because this test's first clause is `S + R <= 100`, which is rule 5's condition as that rule is written today. Relaxing rule 5's condition alone breaks the identity unless this clause is restated as rule 5's condition rather than as that literal inequality; introducing a floor breaks it by making tolerance zero unreachable in the test. Either outcome is permitted and neither may be silent: the amendment states whether the identity survives, and if it does not, it retracts the sentence above, the `waste_tolerance` `0` entry in *Acceptance examples*, and any test asserting them, naming each.

    Only the search step of case 4 consumes entropy. Cases 1 through 3 consume none. The source never proposes `wait`: a blind Mokiterion searches rather than stands still. Because the tolerance decides how often case 4 is reached, this source consumes the shared stream at a rate of its own, so runs under it are comparable only with runs under it, at the same seed and density.

    A proposal from this source is validated under rule 6 with no relaxation. Reading the trait grants the source no authority: it proposes, and the engine decides.
20. **Contact.** Two living Mokiterions are in contact when the Chebyshev distance between their positions is `1` or less, computed by the same distance rule 3's perception uses. The contact radius is `1`, and contact includes co-location at distance `0`. This rule states a relation and occupies no position in tick order: it is a predicate rules 21 and 26 evaluate, recomputed from current positions at the moment it is asked and never stored.

    Contact holds between living Mokiterions only. A dead Mokiterion is in contact with nothing, and contact ends the moment either party dies with no event of its own, because there is no contact state to leave.

    **Contact is symmetric and acting on it is not.** If A is in contact with B then B is in contact with A, but rule 2 gives each Mokiterion its whole cycle in its own turn, so the two evaluate the relation at different points and may not both act on the same configuration: the earlier-acting Mokiterion evaluates contact against the later one's position before it has moved. This is the asymmetry rule 12 records for the `fear` driver, reached through position instead of perception.

    Contact crosses `y=63/64`. Two Mokiterions in different territories at Chebyshev distance `1` are in contact, on the same ground perception already crosses the boundary: a territory label states state and nothing more.

    Contact reads positions and consumes no entropy. Mokiterions still do not block movement and may still share a coordinate; this rule adds no occupancy rule and no collision.
21. **Targeted actions.** In tick order this rule occupies the position of rules 8 to 11: a targeted action is applied at the acting Mokiterion's own opportunity, after rule 6's validation and before rule 7's trace. There are seven verbs, each with its own precondition, checked in the order rule 6 fixes:

    | Verb | Precondition beyond a living target that is not the actor | Effect |
    |---|---|---|
    | `approach` | the target is perceived | one cell toward it, as a rule 8 move |
    | `avoid` | the target is perceived | one cell away from it, as a rule 8 move |
    | `threaten` | the target is in contact | rule 23 |
    | `attack` | the target is in contact | rule 22 |
    | `fight` | the target is in contact **and** named in the actor's suffered-attack record | rule 22 |
    | `retreat` | the target is named in the actor's suffered-attack record | one cell away from it, as a rule 8 move |
    | `surrender` | the target is named in the actor's suffered-attack record | rule 24 |

    **A valid targeted action applies exactly once**, during the current tick, at the acting Mokiterion's own opportunity. None applies twice, none applies to a second target, and none defers its effect to a later tick. One opportunity yields one action, targeted or not.

    **`approach`, `avoid` and `retreat` are rule 8 moves and nothing more**: one cell, one cardinal axis, no additional energy cost, and a crossing of `y=63/64` updates the derived territory and emits its event exactly as any move does. `approach` moves on the horizontal axis while the target's direction has an easterly or westerly component and otherwise on the vertical axis, using the other axis when the preferred one is invalid — rule 5 case 3's axis rule, unchanged. `avoid` and `retreat` use the same axis choice in the opposite direction. This is why they add no event type: each mutates only the actor, and rule 7's trace and rule 8's crossing event already carry everything they change.

    **`avoid` and `retreat` against a co-located target move north, and where north is invalid, east, then south, then west.** Distance zero has no direction to move away from, and rejecting the proposal would make co-location an inescapable state. The order is rule 5 case 4's own cardinal order, reused so that this case introduces no new ordering, and it consumes no entropy: it is the first valid direction in a fixed order rather than a selection among them. `approach` against a co-located target has no such reading and is rejected: there is no cell to move toward.

    **Amended 2026-08-20, under `REQ-MOK-052`: this paragraph named `avoid` alone and understated one reachable path.** Both verbs move away from the target and differ from `approach` only in sign, so both meet distance zero identically, and `retreat`'s preconditions — the target named in the actor's suffered-attack record — require no contact but do not exclude it. `retreat` is in fact the *more* reachable of the two here: rule 26 has no branch that proposes `avoid` at distance zero, its contact branch covering that distance, whereas its answer branch selects `retreat` for a defender at `fear` of at least `30` whose attacker struck from the same cell and has not moved. The clause was correct in what it fixed and incomplete in what it named; naming one verb where two share the path would have left a reader to infer that the other is rejected. What the fallback does is unchanged, for either verb.

    **No selection among Mokiterions happens in this rule.** A proposal arrives naming exactly one target, so the nearest-then-lowest-identifier tie-break that chooses between candidate Mokiterions belongs to rule 26, which chooses, and not to this rule, which applies.

    **The action contract is closed at eleven kinds.** `wait`, `sleep`, `eat`, `move` and these seven are all a source may propose and all the engine will apply. Nothing here admits a composite action, an action naming two targets, or an action spanning ticks.
22. **Combat resolution.** One resolution, invoked by `attack` and by `fight` alike. The striker is the acting Mokiterion; the target is the Mokiterion its proposal names.

    **Damage is `10 + (striker.energy + striker.health) / 10`**, the division truncating toward zero, evaluated at the moment of resolution from the striker's `energy` and `health` at that moment and from nothing else — not the target's attributes, not the tick, not either identifier, not any trait, not any population aggregate. Both terms are bounded by `ATTRIBUTE_MAX`, so the range is `10..=30`. Integer arithmetic only, in a width no intermediate overflows.

    Damage derived from the striker's own condition is what makes a weakened Mokiterion a weaker attacker without any Mokiterion reading another's strength — `CAP-MOK-010`'s exclusion of perceived relative strength, held from the other side.

    **The target's `health` falls by that amount, saturating at zero. The striker pays a flat `5` `energy`, saturating at zero**, whether or not the target dies. Both use the saturating arithmetic rule 12 uses; neither wraps, and neither is clamped by any other attribute.

    **No resolution deals `0`.** The constant term is `10`, so the minimum is `10` and the bound holds by construction rather than by a clamp. It is stated because a resolution removing nothing would be indistinguishable in the stream from a rejection, and it constrains any later amendment to the function.

    The strike costs `5` because an attack that were free would dominate every alternative for every source that has it. `5` is a cost rather than a deterrent; the deterrent is the source's own ordering.

    **No entropy is drawn** by validation, damage computation, cost application, or death handling.

    Where the target's `health` reaches zero it is dead at that moment, through rule 13's path and event. Where the striker's `energy` reaches zero it does not die of that: rule 12's decay is what converts a zero attribute into `health` loss, at the striker's own turn, exactly as it does today.

    The resolution emits one `attack_resolved` line carrying both parties' transitions, as *Data and interface contracts* fixes. That line does not distinguish `attack` from `fight`, and it is not meant to: the two are one resolution. A reader tells them apart by the pairing — a `fight` names as its target a Mokiterion that struck the subject earlier — and directly from rule 7's trace, whose `proposal` field carries the verb.
23. **Threat.** The target's `fear` rises by `30`, saturating at `ATTRIBUTE_MAX`. The amount is a specification constant of its own, standing beside rule 12's `FEAR_INCREASE` of `10` and `FEAR_DECREASE` of `5` rather than being derived as a multiple of either, and it is derived from neither Mokiterion's attributes: a threat is a threat regardless of who makes it, because deriving its force from the threatener's condition would be a reading of relative strength arriving by the back door.

    **Nothing else changes.** Not the target's `health`, `satiety`, `energy` or position; not the threatener's anything. No damage, no energy cost, no satiety transfer, no movement, no death. The only thing a threat costs its maker is the opportunity it spends.

    **A threat opens no suffered-attack window.** A threatened Mokiterion has not been attacked, so it may not `fight`, `retreat` or `surrender` in answer. It may propose `attack`, `avoid` or anything else its source chooses on the ordinary preconditions. A threat provokes by changing `fear`, not by unlocking verbs — and `30` is exactly the step that moves rule 26's own branches, which is the mechanism by which a threat changes what another Mokiterion decides.

    Rule 12's write is unaffected and still applies to every living Mokiterion at its own opportunity. The composition of the two writers within one tick is stated in rule 12.

    The resolution emits one `threat_resolved` line carrying the increase applied and the target's resulting `fear`.
24. **Surrender.** The surrendering Mokiterion forfeits `satiety / 2` of its own `satiety`, the division truncating toward zero, evaluated at the moment of resolution and derived only from its own `satiety` — not from the recipient's attributes, not from the damage suffered, not from either identifier, not from any trait.

    **Its `satiety` falls by that amount, saturating at zero; the recipient's rises by the same amount, saturating at `ATTRIBUTE_MAX`, and any excess is destroyed.** The excess is not returned, not banked, and not converted into another attribute. `surrender_resolved` reports the discarded amount so that a run's own output shows where non-conservation occurred rather than leaving it to be inferred.

    Half of what is held can never exceed what is held, so the saturation at zero constrains a later amendment to the proportion rather than any case reachable under this one. **A surrender below `satiety` `2` transfers `0` and still succeeds**: a Mokiterion with nothing to give gives nothing, and it has still declined to fight.

    **No damage is dealt to either party**, no `energy` is paid by either, neither moves, and **no `fear` is written in either direction**. Rule 23 is the only rule besides rule 12 that writes `fear`.

    The suffered-attack window closes as it does for any answer, under rule 25. A Mokiterion that surrenders may not also `fight` or `retreat` that attack.

    **Death by starvation remains rule 12's.** A surrender may reduce a `satiety` to zero, and a Mokiterion that then dies dies through the survival path, with that event and that cause. A surrender is not itself a cause of death.
25. **The suffered-attack window.** This rule occupies two positions in tick order. It **opens** inside rule 22, when damage resolves against a Mokiterion, and it **closes** at that Mokiterion's own next decision opportunity, after rule 7's trace and before rule 12's decay.

    **What it carries**: for each attack that has landed since that Mokiterion's previous decision opportunity, the striking Mokiterion's identifier and the damage dealt, in the order the attacks resolved. Where a Mokiterion has been struck more than once in the window, all are carried.

    **It is transient per-Mokiterion state and not an attribute.** It is not bounded in `0..=100`, it is not reported in `agent_initialized` or `survival_changed`, it hangs off the Mokiterion that suffered the attacks, and no structure indexed by pairs of Mokiterions exists. It starts empty and, under `baseline`, `reference` and `individual`, stays empty for the whole run, because no targeted verb is proposed under those sources.

    **It closes when the opportunity is taken, not when it is used.** After a Mokiterion's source has been consulted and its proposal applied or rejected, the record is cleared — whether it answered, proposed something else, or had its proposal rejected. Each Mokiterion gets exactly one opportunity to answer any given attack, and a Mokiterion struck twice may answer one attacker while the window closes on both, because one opportunity yields one action. Whether that discards a meaningful number of encounters is measurement's to report and not this rule's to prevent.

    **The latency this produces is asymmetric and is stated rather than corrected.** Rule 2 runs each Mokiterion's whole cycle in its own turn, in ascending identifier order. A Mokiterion struck by a *lower*-identified one reaches its own opportunity later in the same tick and answers with zero ticks of latency; one struck by a *higher*-identified one has already acted and answers on the next tick. This is the within-tick asymmetry rule 12 records for the `fear` driver and rule 20 records for contact, and it is one of the two inputs to the identifier advantage `VER-MOK-016` bounds.

    **Nothing else reads the record.** It is not an input to damage, not to contact, not to survival decay, and not to any source but rule 26's. It is not reported as an attribute. A Mokiterion that dies of an attack never reads it, and the record dies with it.
26. **The `social` decision source.** Selected by `--policy social`. This rule is appended rather than placed beside rule 5, for the reason the *Behavioral rules* preamble states; in tick order it occupies rule 5's position. It reads the acting Mokiterion's own `fear` and its suffered-attack record, both carried on the rule 3 observation, and returns the **first** applicable branch:

    1. **An unanswered attack is answered.** Where the suffered-attack record is non-empty, answer the **first** attack in it — the record is in resolution order, so this is deterministic — choosing by the actor's own `fear`: `surrender` at `60` or above, `retreat` at `30` or above, and `fight` below `30`.
    2. **Survival comes before society.** Where rule 19's case 1 or case 2 applies — a co-located resource passing the tolerant test, or `energy` below `20` — propose exactly what rule 19 proposes, which is `eat` or `sleep`.
    3. **Food perceived outranks company perceived.** Where rule 19's case 3 applies — a resource passing the tolerant test perceived at a distance of `1` or more, with a step toward it that rule 6 admits — propose exactly that step. This branch draws nothing: it is rule 19's case 3 alone, with case 4 left at branch 6.
    4. **A Mokiterion in contact is engaged.** Where a living Mokiterion is in contact under rule 20, propose `attack` while the actor's own `fear` is below `95` and `threaten` otherwise, against the nearest such Mokiterion, breaking ties by lowest identifier.
    5. **A Mokiterion merely perceived is closed on or avoided.** Where a living Mokiterion is perceived at a distance of `2` or more, propose `approach` while the actor's own `fear` is below `95` and `avoid` otherwise, on the same tie-break.
    6. **Otherwise rule 19's case 4 decides.** Propose exactly what rule 19's search step proposes. This is the only branch that draws.

    **Amended 2026-08-20, under `REQ-MOK-057`'s first amendment.** As first approved this rule had five branches, with rule 19's case 3 and case 4 together at branch 5 behind society, and the engagement threshold at `30`. Both were measured to be unsatisfiable against `REQ-MOK-058`: `fear` rises `10` per tick on company *perceived* at radius `16` while engagement required `fear` below `30` and contact at radius `1`, so the gate closed on the third perceiving tick and closing sixteen squares takes fifteen. No approach could complete, `attack` was proposed 3 times in 5,000 opportunity-ticks, nothing struck anything, and branch 1 — which owns `fight`, `retreat` and `surrender` — never fired. Meanwhile branch 4's `avoid` displaced the seek-move at branch 5, so survivors fell to starvation in a world with no fighting in it. Hoisting case 3 and moving the gate to `95` was measured to satisfy both of `REQ-MOK-058`'s bounds on all five declared seeds; `evidence/WO-MOK-016/escalation.md` records the seventeen variants measured and the three the product owner chose between. **Rule 12 is untouched by this amendment**, which is why `WO-MOK-010`'s measured `fear` distribution still stands.

    **Branch 1 answers the first attack in the record whether or not that answer can succeed.** It reads the record and the actor's own `fear`, and it does not re-check rule 21's preconditions before proposing. Two cases follow from rule 2's order, and both are intended. The attacker may have died at the hands of a third Mokiterion since the attack resolved, in which case every one of the three answers names a target that is not living. And where `fear` is below `30` the answer is `fight`, which rule 21 requires contact for, and the attacker may have moved out of contact at its own turn — `retreat` and `surrender` require only the record, so only `fight` can fail this way. In both cases rule 6 rejects the proposal, the opportunity is spent, no state changes for either Mokiterion, and rule 25 clears the record regardless. **The technical owner decided this on 2026-08-20**, against a branch that skipped to the next answerable attack: swinging at an attacker who fled or died is behavior rather than a defect, the rejection and its reason are visible on the trace line, and it is the asymmetric latency of rule 25 made observable rather than hidden. The cost is a spent turn, and it is accepted. `VER-MOK-016` measures it rather than assuming it stays rare.

    **The ordering is normative rather than illustrative.** An implementation may not reorder the branches, and may not test branch 2 or branch 3 by delegating to rule 19 whole and inspecting the result: rule 19's case 4 takes an entropy selection, so a delegation reaching it would consume a draw this source then discarded whenever a later branch fired, silently moving the shared stream for a decision that was never used. Branches 2 and 3 must therefore test rule 19's cases 1, 2 and 3 without reaching its case 4, which is what makes hoisting case 3 above branches 4 and 5 legitimate rather than a reordering of the draw.

    **Three constants and no more**: the answer thresholds `60` and `30`, and the engagement threshold `95`. This source introduces no survival constant of its own. Branch 2's condition is rule 19's own cases 1 and 2, branch 3's is its case 3, and branches 2, 3 and 6 delegate to rule 19 rather than restating it, so a Mokiterion under this source that never meets another behaves exactly as it would under `--policy individual`, trait and all.

    The three thresholds compose with rule 23 by design, and the composition runs through **branch 1** rather than through the engagement gate. A threat adds `30`, so two threats, or a threat and four ticks of perception, reach `60` and make a defender `surrender` rather than `fight`, and one threat alone moves it from `fight` to `retreat`. That is the mechanism by which `threaten` changes what another Mokiterion decides.

    **What the engagement threshold of `95` means, and what it gives up.** At `95` the gate reads "engage unless nearly saturated": only a Mokiterion within `5` of `ATTRIBUTE_MAX` declines. A single threat therefore no longer moves a calm Mokiterion from `attack` to `threaten`, which the five-branch form of this rule claimed and which was true at `30`. That claim is withdrawn, and its withdrawal is the price of the gate: the value is not derivable from rule 23's `30` or from any other constant, it is the measured point at which `REQ-MOK-058`'s two bounds first hold together on every declared seed, and `90` fails on one seed while `100` holds with less survivor margin. **An arbitrary constant in a specification is a cost**, recorded here rather than dressed up as a derivation, and the narrowness of the band is itself the finding: this source is viable only where the gate is "engage unless saturated".

    **Where in the tick the `fear` it reads comes from.** Rule 3 builds the observation at the acting Mokiterion's own turn, and rule 12 writes `fear` at that same turn, after the action. So the value this source reads is the one standing after the previous tick's rule 12 write, plus any threat applied to it by an earlier-acting Mokiterion in the current tick. Every threshold above is sensitive to that position, and it is written here so that a reader need not derive it from rule ordering.

    **The entropy discipline is a consequence of the ordering rather than a rule beside it.** Branches 1, 4 and 5 return without a draw. Branch 2 returns `eat` or `sleep` and branch 3 returns a directed step, both of which rule 19 reaches without a draw. Only branch 6 can draw, and it draws exactly what rule 19's case 4 draws: at most one selection, from the single shared stream, in rule 19's own order. So this source takes **at most one draw per opportunity and never for a social decision**.

    Because branches 4 and 5 pre-empt rule 19's case 4, a run under this source consumes **fewer** draws than a run under `--policy individual` on the same seed, and diverges from it at the first opportunity where a Mokiterion is perceived and no tolerated resource is. That is different world evolution rather than a changed draw discipline. The equality that does hold is stated per observation, and the 2026-08-20 amendment **widened** it: where the suffered-attack record is empty and rule 19's case 1, 2 or 3 applies, this source and rule 19's propose the identical action — whether or not a Mokiterion is perceived. Under the five-branch form the equality required that none be perceived; hoisting case 3 means a perceived Mokiterion no longer displaces a perceived meal, so the cheapest oracle on this source now covers strictly more observations than it did.

    A proposal from this source is validated under rule 6 with no relaxation. Reading `fear` and the suffered-attack record grants the source no authority: it proposes, and the engine decides.

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

`Observation` contains copied or immutable values, including the acting Mokiterion's own `fear`, its `waste_tolerance`, the record of attacks it has suffered since its previous decision opportunity, and the perceived resources and perceived Mokiterions of rule 3. A perceived entry carries an identifier, a relative direction, a distance, and, for a resource, its calorie class. It carries no reference to the entity it describes, so a decision source can read a perceived entity but cannot reach it. An entry in the suffered-attack record carries the striking Mokiterion's identifier and the damage it dealt, in the order the attacks resolved, and nothing further: it records what happened to the observer rather than the condition of whoever did it, which is how a defender answers without any Mokiterion reading another's strength. **The observation gains those two items and nothing else.** Its list of currently valid proposals is unchanged and never carries a targeted action, for the reason rule 3 states.

`ProposedAction` is one of:

```text
Wait
Sleep
Eat { food_id }
Move { direction: North | East | South | West }
Attack { target }
Threaten { target }
Fight { target }
Retreat { target }
Surrender { target }
Approach { target }
Avoid { target }
```

The seven targeted forms all name the other Mokiterion in one field called `target`, and the uniformity is deliberate. `retreat` retreats *from* its target and `surrender` surrenders *to* its target, so two directional field names were available; one name was chosen because rule 6 checks the same three things about the named Mokiterion in all seven cases — that it exists, that it lives, and that it is not the actor — and a contract that spelled the same referent three ways would invite a validation path per spelling. A `target` is a Mokiterion identifier and never a reference: naming one grants a source no more reach than perceiving one does.

The decision source never receives a mutable world, agent, resource collection, event log, or engine handle.

Event lines use stable key-value fields in this order:

```text
tick=<number> subject=<identifier> event=<event-type> result=<stable-details>
```

Stable core event types are `world_initialized`, `food_initialized`, `agent_initialized`, `decision_source_selected`, `food_consumed`, `food_regenerated`, `food_regeneration_skipped`, `territory_crossed`, `attack_resolved`, `threat_resolved`, `surrender_resolved`, `survival_changed`, `agent_died`, and `simulation_ended`. Optional per-action lines use `action_trace`. The details of `decision_source_selected` name the active source as `baseline`, `reference`, `individual`, or `social`.

**The vocabulary gains three types and no more, and the count is a consequence of one rule rather than of one type per verb.** A new type exists where a resolution moves a *second* Mokiterion's state, because that is the transition no other record can carry. `attack_resolved` is shared by `attack` and `fight`, which are one resolution invoked by either party; `threat_resolved` and `surrender_resolved` take one each. `approach`, `avoid` and `retreat` take none: each mutates only the actor and resolves as a rule 8 move, so rule 7's trace already reports every transition they cause and rule 8's crossing event still fires where they cross `y=63/64`. A type reporting nothing a record already carries would be interface growth for nothing. Seven types, one per verb, and a single `encounter_resolved` carrying the verb in its detail were both considered and declined.

Three record kinds carry the attributes, and their stable details are fixed in this order:

```text
tick=0 subject=<mokiterion-id> event=agent_initialized result=name:<letters>,position:<x:y>,territory:<A|B>,health:<number>,satiety:<number>,energy:<number>,fear:<number>,waste_tolerance:<number>
tick=<number> subject=<mokiterion-id> event=survival_changed result=health:<a>-><b>,satiety:<a>-><b>,energy:<a>-><b>,fear:<a>-><b>
```

`fear` follows the other three attributes in every record that carries them, and `waste_tolerance` is reported once, in `agent_initialized`. The trait is reported and never restated, because it cannot change during a run. The name is reported once, in `agent_initialized`, for the same reason, and it is reported on no other record kind.

**The name is the first detail and `waste_tolerance` remains the last, and both positions are fixed rather than
incidental.** Two test suites parse this record positionally — one asserts `fear` and `waste_tolerance` as an adjacent
pair, the other reads the trait by splitting on the record's final field — so appending the name would have required
changing assertions that `VREC-MOK-005` and `VREC-MOK-007` bind, which `WO-MOK-011` forbids. Placing it first also puts
identity at the head of the record, where a reader looks for it. A later field is appended after `waste_tolerance` only
by an amendment that states what it does to those parsers.

Three resolution record kinds carry a second Mokiterion's transitions, and their stable details are fixed in this order:

```text
tick=<number> subject=<striker-id> event=attack_resolved result=target:<mokiterion-id>,damage:<number>,target_health:<a>-><b>,striker_energy:<a>-><b>,target_died:<yes|no>
tick=<number> subject=<threatener-id> event=threat_resolved result=target:<mokiterion-id>,increase:<number>,target_fear:<a>-><b>
tick=<number> subject=<surrendering-id> event=surrender_resolved result=recipient:<mokiterion-id>,transferred:<number>,discarded:<number>,subject_satiety:<a>-><b>,recipient_satiety:<a>-><b>
```

The subject of each is the Mokiterion at whose opportunity the resolution happened, which for `attack_resolved` is the striker whether it proposed `attack` or `fight`. Each uses the `<a>-><b>` transition form `survival_changed` already uses, rather than a second notation for the same idea.

**Each of these records carries the transitions the resolution caused, in both directions, and that is an obligation rather than a convenience.** Rule 2 runs each Mokiterion's whole cycle inside its own turn. A Mokiterion acted on by a lower-identified one has not yet emitted its `survival_changed` line for the tick, and one acted on by a higher-identified one already has, so a transition inflicted on a target is a change that would otherwise appear in no record on the tick it happened. Carrying it here makes the stream complete in both directions and keeps it reconstructible without knowing turn order.

Three further details are fixed for the same reason: `target_died` reports the death this strike caused, so that death by combat is distinguishable in the stream from death by starvation or exhaustion without adding a cause field to `agent_died`; `discarded` reports satiety a surrender transferred into a full recipient and lost, so that a run's own output shows where non-conservation occurred rather than leaving it to be inferred; and `increase` reports a threat's applied amount alongside the resulting `fear`, because saturation at `100` makes the applied amount unrecoverable from the transition alone.

**None of the three ever appears under `baseline`, `reference` or `individual`**, because no targeted verb is proposed under those sources. That is why adding them to the vocabulary leaves every capture taken before this amendment byte-identical.

An action trace uses the same leading fields and stable details in this order:

```text
tick=<number> subject=<mokiterion-id> event=action_trace result=proposal:<action>[,target:<mokiterion-id>],status:<accepted|rejected>,detail:<result-or-reason>,position:<x:y>,territory:<A|B>,health:<number>,satiety:<number>,energy:<number>,fear:<number>[,suffered:<attacker-id>:<damage>[;<attacker-id>:<damage>]...]
```

**A targeted proposal reports its target in a `target` field of its own, immediately after `proposal`, and that field is present only for the seven targeted verbs.** A trace line that named the verb alone would leave the one fact that distinguishes two otherwise identical opportunities out of the record, and rejections in particular are unreadable without it: `status:rejected` with `detail:target not in contact` says nothing about which Mokiterion was out of contact. The technical owner decided the separate field on 2026-08-20, against rendering the target inside `proposal` as `attack:M03`; the decision is recorded in `WO-MOK-016`. Its consequence is stated here rather than left to a reader: **this line now has two conditionally-present fields**, `target` and `suffered`, so a parser may not assume a fixed field count on `action_trace` and must address details by name. The four core verbs render exactly as they render today — `proposal:move:south` keeps the inner colon it has always had for its direction, because `CAP-MOK-010` holds those lines byte-identical — so the line carries two conventions at once: `move`'s direction inside `proposal`, and a targeted verb's subject beside it. That is the price of the decision, and it is paid in the parser rather than in any existing capture, none of which changes.

**The `suffered` field is appended after `fear`, and it is present only when the suffered-attack record is non-empty.** The conditional presence is forced rather than chosen, and it is forced for a different reason than `target`'s: `target` is absent from a `wait` line because a `wait` has no target to report, while `suffered` would have a value to report on every line and is still withheld. Every detail on every other record kind is unconditional, and a field appended unconditionally here would change every `action_trace` line of every run, including every `baseline` run with `--trace-actions` — and `CAP-MOK-010` holds `baseline` byte-identical against its pre-change capture, which no amendment in this initiative may spend. Under `baseline`, `reference` and `individual` no attack is ever proposed, so no record is ever non-empty and the field never appears; under `social` it appears on exactly the lines it explains. The entries are the record's own, in the order the attacks resolved, so the field is ordered and not a collection formatted arbitrarily.

`suffered` reports the record as the source read it, before it is cleared: rule 7 fixes that order and states why. It is therefore the second field on this line whose value predates the action rather than following it, alongside `fear`, and the two are read at the same point for the same reason.

**Neither added field touches a parser bound by evidence, and that is checked rather than assumed.** The positional obligation this section records is on `agent_initialized` — one suite asserts `fear` and `waste_tolerance` as an adjacent pair, another reads the trait by splitting on that record's final field — and this amendment does not touch `agent_initialized` at all. On the `action_trace` line `suffered` is appended last and `target` is inserted second, and the insertion is the one that would break a positional reader, because it shifts `status` and everything after it by one field. **It shifts them on targeted lines only**, and a targeted line exists only under `social`: no capture taken before this amendment contains one, and no parser reading a `baseline`, `reference` or `individual` trace will ever encounter the shift. Any suite that parses this line positionally must be named in the evidence for this amendment with its assertion count before and after, and none may have an assertion relaxed, widened or removed to accommodate either field.

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
