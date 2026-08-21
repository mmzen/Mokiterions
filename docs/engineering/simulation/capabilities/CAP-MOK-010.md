+++
id = "CAP-MOK-010"
type = "capability"
title = "Social encounter and conflict: a targeted action contract, a defender's own response, and a source that reads fear"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"

[relations]
derives_from = ["INT-MOK-010"]
+++

# Capability: Social encounter and conflict

## Actor and need

The **operator** needs a run in which the twelve Mokiterions are aware of each other in a way that has consequences.
Today two of them can occupy one cell for a thousand ticks and the only trace is a number going up. The operator also
needs to be able to read the encounter from the run's own output — who acted on whom, what it cost, and what the
defender did next — without inspecting the source.

The **product owner** needs the gap between `README.md` and the artifact closed at its widest point. Five of the verbs
that document names — `threaten`, `attack`, `fight`, `retreat`, `surrender` — do not exist, and `SIMULATION_RULES.md`
§16 lists their absence among the things the simulation does not do yet. The owner also needs the closing of that gap to
*not* be the thing the concept forbids: conflict arising from a population-level rule rather than from individuals.

The **technical owner** needs `fear` to stop being inert. `SPEC-MOK-001` rule 12 states that no rule reads it and that
a consumer is a later governed change; rule 3 withholds it from the observation for the same reason. Both sentences are
outstanding obligations written into an approved specification, and they are discharged by amendment here or they stay
outstanding.

The **assurance owner** needs the absence claim to be checkable. "No population-level trigger" is not verifiable by
reading a diff for the string `start_war`; it needs to be expressed as a property over what rules and sources are
permitted to read, in the way `REQ-MOK-041` expressed "nothing reads a name".

## Capability statement

A Mokiterion in contact with another may act on it — approaching, avoiding, threatening or attacking it — and a
Mokiterion that has been acted on chooses its own answer on its own next decision opportunity, fighting, retreating or
surrendering; the engine resolves every such action deterministically against current state, applies its cost to both
parties, reports the transitions it caused in the event stream, and permits death by combat, while a fourth decision
source reads `fear` and what it has suffered in order to choose — so that conflict in this world is the sum of what
individual Mokiterions decided, and can be shown to be nothing else.

## Boundaries

**Included.**

- **Contact** defined as Chebyshev adjacency, radius `1`, between two living Mokiterions, adding one constant and
  reusing the distance function rule 3's perception already uses.
- **Seven new proposable actions**, taking the closed contract from four kinds to eleven: `approach` and `avoid`, which
  target a perceived Mokiterion and resolve as movement; `threaten` and `attack`, which require contact; and `fight`,
  `retreat` and `surrender`, which are available to a Mokiterion that has been attacked.
- **Deterministic resolution.** Damage is an integer function of the striker's energy and health. No entropy draw is
  taken, so `REQ-MOK-009` is untouched and every outcome is explainable from the two Mokiterions' reported states.
- **Cross-agent mutation**, for the first time in this engine: an attack writes the target's `health`, a threat writes
  the target's `fear`, and a surrender writes both parties' `satiety`.
- **Death by combat**, through the existing rule 13 path, so that death stays one concept with one event.
- **The deferred response.** `attack` resolves immediately under engine authority; the defender's `fight`, `retreat` or
  `surrender` is a proposal its own source makes on its own next opportunity, enabled by an attacks-suffered field on
  the rule 3 observation. Response latency is zero ticks when the defender's identifier is higher than the attacker's
  and one tick when it is lower — the same within-tick asymmetry rule 12 already documents for `fear`.
- **`fear` on the rule 3 observation**, and rule 12's "no rule reads `fear`" and rule 3's stated reason both amended.
- **A fourth decision source, `social`**, selectable by `--policy`, which reads `fear`, `health`, `energy` and
  attacks-suffered, and is the only source that proposes any of the seven new verbs.
- **A survivor floor for that source**, stated and measured, so that combat is not permitted to make the world
  uninhabitable unmeasured.
- **The high-class resource accumulation effect absorbed**, with a stated ceiling on the class composition of a
  territory's standing resources — the Phase 1 limitation `SPEC-MOK-001` rule 5 recorded as accepted and out of scope
  for that revision, and which invalidates the same two survivor floors combat does.
- **New event types**, each carrying the state transitions it caused, each mapped to an authorizing requirement under
  `SPEC-MOK-003` rule 11, and counted as public interface growth under `SPEC-MOK-002` rule 6.
- **Evidence of absence**: that no rule, decision source or validation path reads any population aggregate, and that
  win rate and survival are not monotonic in identifier.

**Excluded.**

- **Interaction memory beyond one decision opportunity.** Nothing persists who met whom. A defender knows it was
  attacked since it last acted, and that window closes when it acts.
- **Perceived relative strength.** No Mokiterion reads another's `health`, `energy` or `waste_tolerance`. Whether the
  defender's choice is impoverished without it is measured here and decided later.
- **Any change to what `baseline`, `reference` or `individual` propose.** None of them gains a verb. `baseline` in
  particular is held byte-identical, because widening its candidate list would move its single entropy selection and
  diverge every pre-existing run under it.
- **Cooperation, sharing, grouping, alliance, reputation, mating, kinship.**
- **Territorial defence.** Crossing `y=63/64` continues to emit an event and mean nothing else.
- **Any change to the perception radius, the food table, the density mapping, regeneration timing, the finality of
  death, the exit-code contract, the default policy, or the default density.**
- **Entropy substreams**, still declined for Phase 2's reason.
- **Model-backed decisions, persistence, structured output, batch execution, a new package, target or dependency.**
- Any observer feature beyond presenting what the engine reports.

## Outcomes

- An operator running the fourth policy sees encounters happen, sees Mokiterions die of them, and can reconstruct any
  one of them from the event stream alone.
- `fear` has a reader, so `SPEC-MOK-001` rule 12's closing sentence and rule 3's refusal are retired by the amendment
  they anticipated, rather than standing as permanent admissions.
- `SIMULATION_RULES.md` §16 loses four of its entries.
- The action contract `README.md` documents is complete, so no later phase has to add a verb to it.
- Phase 5's decision source arrives to find social agency available to exercise and a defender's response still
  undecided, which is the decision worth giving a model.
- The two carried survivor floors are re-measured once, against a world that has both combat and corrected resource
  composition in it, rather than twice against two intermediate worlds.
- The absence of population-level scripting is a measured property of this system rather than a claim about its authors'
  intentions.

## Candidate requirements

- `REQ-MOK-051` — contact, and the encounter the engine detects from it.
- `REQ-MOK-052` — the engine applies the seven new actions, beside `REQ-MOK-005`'s four rather than replacing it.
- `REQ-MOK-053` — combat resolution: deterministic cost to both parties, and death by combat.
- `REQ-MOK-054` — the deferred response: `fear` and attacks-suffered on the observation, and the defender's own answer.
- `REQ-MOK-055` — `threaten` raises the target's `fear` and does nothing else.
- `REQ-MOK-056` — `surrender` forfeits satiety to the attacker and ends the encounter.
- `REQ-MOK-057` — the `social` decision source, which reads `fear`.
- `REQ-MOK-058` — habitability under that source: the survivor floor.
- `REQ-MOK-059` — no population aggregate is read by any rule, source or validation path.

- `REQ-MOK-060` — the class composition of a territory's standing resources stays within a stated ceiling.

Ten rather than fewer, because each states an obligation that can fail on its own and be measured on its own, and
because they do not share an accountable owner: `REQ-MOK-053`, `REQ-MOK-056`, `REQ-MOK-058` and `REQ-MOK-060` are
product judgements about how violent and how survivable this world is, while `REQ-MOK-054` and `REQ-MOK-059` are
technical and assurance obligations respectively.

**Three existing requirements are amended rather than restated here.** `REQ-MOK-014`'s and `REQ-MOK-034`'s survivor
floors are invalidated by this capability and re-measured once, against a world holding both combat and corrected
resource composition. `REQ-MOK-034` is additionally amended in its clause that the `reference` and `baseline` sources'
outcomes "are frozen": `REQ-MOK-060` moves `reference` and `individual` by construction, so that clause is narrowed to
`baseline`, which this capability does hold byte-identical. And `REQ-MOK-005` — "Apply core actions", whose statement
enumerates move, eat, sleep and wait — is amended so that its enumeration reads as the core set beside which
`REQ-MOK-052` places the targeted set. **It is amended and not superseded**, deliberately: it is cited by
`CAP-MOK-001`, by `SPEC-MOK-001`'s and `VER-MOK-001`'s covered-requirement lists, by `WO-MOK-001`, by `SPEC-MOK-003`'s
rule 11 authority table as the authority for `territory_crossed`, and by two locations in `mokiterions-tui`. Two of
those artifacts are released under `RLS-MOK-001`. Supersession would move all of it to buy nothing.
