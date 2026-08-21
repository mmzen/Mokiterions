+++
id = "INT-MOK-010"
type = "intent"
title = "Let Mokiterions act on each other"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"

[relations]
+++

# Intent: Let Mokiterions act on each other

## Problem

**Twelve Mokiterions share a world and cannot touch each other.** `SPEC-MOK-001`'s state model says agents "do not
block movement" and "may occupy the same coordinate after initialization", and nothing in the eighteen behavioral rules
gives one Mokiterion any effect on another. Two of them can stand on the same cell for a thousand ticks. The closed
action contract is `wait`, `sleep`, `eat` and `move`; every one of the four acts on the world or on the actor, and none
of them has a target. The population is twelve solitary foragers who happen to be in the same field.

**`fear` is the measure of this, and the specification says so in as many words.** Rule 12 computes it — `+10` when the
acting Mokiterion's own observation lists another living Mokiterion, `-5` when it does not — and closes: "**No rule
reads `fear`.** It is computed, bounded, reported and otherwise inert; a consumer is a later governed change." Rule 3
carries the same admission from the other side, refusing to put `fear` on the observation because "no rule and no
decision source reads it". `INT-MOK-006` shipped an attribute whose only purpose is to be read, and deliberately
shipped no reader. **This initiative is the governed change those two sentences name.** It is not a new idea about
`fear`; it is the discharge of a debt `SPEC-MOK-001` wrote down twice.

**The plain-language guide states the gap without softening it.** `SIMULATION_RULES.md` §16 lists what the simulation
does not do yet, and four of its entries are this: no fighting, no threatening, no fleeing, no cooperating, and no
memory of anyone met. A reader who arrives from `README.md` — which promises `threaten`, `attack`, `fight`, `retreat`
and `surrender` by name, and territory that means something — meets a forager. The distance between the concept and the
artifact is at its widest here, and it is wider than the distance perception or individuality had to close.

**The thing that makes this hard is the thing that makes it worth doing.** `README.md` states the project's central
prohibition: no `if food < threshold: start_war()`. Conflict must arise from what individual Mokiterions do, not from a
population-level rule that fires on an aggregate. Every phase so far has been able to satisfy that prohibition by not
having the capability at all. This one cannot. It builds the mechanism by which conflict *could* be scripted, and its
verification obligation is therefore a claim of **absence** — that no rule, no source and no validation path reads a
population aggregate — of the same kind `REQ-MOK-041` made about names and `REQ-MOK-032` made about `fear`, and harder,
because here the capability being denied is one the change deliberately creates.

**Phase 5 needs this before it can mean anything.** A model-backed decision source proposing from a four-verb contract
in which nothing has a target would be an expensive way to choose between eating and walking. The agency a language
model is being brought in to exercise is social agency, and it does not currently exist to be exercised.

## Desired outcomes

- Two Mokiterions that come into contact can act on each other: approach, avoid, threaten, attack, and — as a defender
  — fight, retreat or surrender.
- **`fear` is read.** A decision source consults it, so the attribute stops being a gauge and becomes an input, and
  rule 12's closing sentence and rule 3's refusal are both retired by amendment rather than left contradicted.
- Death through combat is reachable, and reached, at every declared seed. An attack that cannot kill is a gesture.
- **A defender chooses its own response.** Fight, retreat and surrender are decisions the defending Mokiterion's own
  source takes on its own next opportunity, not outcomes the engine computes on its behalf. What a Mokiterion does when
  attacked is the most interesting decision in the world and it is not the engine's to make.
- The world stays habitable with combat in it: a stated survivor floor, measured, for the source that fights.
- **No population-level trigger exists, and that is verified as an absence rather than inferred from its not having
  been written.**
- An operator reading a run's own output can tell who attacked whom, what it cost, and what the defender did about it,
  without reading the source.
- Every run recorded under `baseline`, `reference` and `individual` before this change reproduces afterwards, except
  where the resource-composition correction below deliberately moves it — and where it does, the divergence is
  measured and attributed to that correction rather than to combat.

## Actors and stakeholders

- The operator gains encounters in the event stream and in the observer, and a fourth `--policy` value, `social`, to run
  them under. Nothing they run today behaves differently unless they ask for it.
- The **product owner** owns the decisions that are about the world rather than about code: that combat may kill, that
  a Mokiterion which surrenders gives up food to the one it surrendered to, that threatening costs nothing and
  frightens, and the survivor floor the fighting source must meet.
- The **technical owner** owns three consequences with no precedent in this engine: that one agent's action now writes
  another agent's state, that a defender's response is deferred to its own next decision opportunity rather than
  resolved inside the attacker's, and that the new rules are appended rather than inserted.
- The **assurance owner** owns two claims. The first is the absence of population-level triggers. The second is new in
  kind: that the acting order rule 2 fixes does not hand a systematic advantage to low identifiers, which deterministic
  resolution makes a real possibility rather than a theoretical one.
- Implementation agents get the largest single behavioral change since `WO-MOK-001`, and the first one where the
  additivity property cannot be fully preserved.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Rules that read `fear` | 0 | at least 1 | Static check |
| Decision sources that read `fear` | 0 | 1 | Static check |
| Proposable action kinds in the closed contract | 4 | 11 | Automated verification |
| Agent attributes writable by another agent's action | 0 | 3 | Static check |
| Deaths attributable to combat per 1,000-tick run under the new source | 0 | greater than 0 on every declared seed | Automated verification |
| Survivors at 1,000 ticks under the new source at the default density | not applicable | at least the stated floor, on every declared seed | Automated verification |
| Population aggregates read by any rule, source or validation path | 0 | 0 | Static check and review |
| Win rate and survival by identifier | not measured | not monotonic in identifier | Automated verification |
| Runs under `baseline` differing from the pre-change capture | 0 | 0 | Automated verification, declared matrix |
| High-class share of a territory's standing resources at tick 1,000 | 45 of 61 | below the stated ceiling | Automated verification |
| Engine package external dependencies | 0 | 0 | Every build |

Two of these need their honesty stated at the point of measurement rather than in a footnote. **The `baseline` target of
zero divergence is achievable only because the new verbs are withheld from that source**; adding them would change the
size of its candidate list, move its single entropy selection, and diverge every pre-existing baseline run. And the
high-class measure is the one carried-forward Phase 1 limitation this initiative absorbs, which means `reference` and
`individual` runs *do* move — see *Risks and assumptions*.

## Non-goals

- **Interaction memory.** Nothing remembers who it met, beyond the one-opportunity window a defender needs to know it
  was attacked. Persistent grudges, alliances and reputation are a later phase, and the roadmap's Phase 3 places them
  there deliberately.
- **Perceived relative strength.** A Mokiterion does not read another's health, energy or trait. Whether response
  selection is impoverished without it is a question this initiative *measures* rather than answers; if the measurement
  says the defender's choice is degenerate, that is the evidence a later change would be authorized on.
- **Cooperation, sharing, mating, grouping, or any positive social act.** Conflict first, because `fear` is what exists
  to be read.
- **Territory meaning anything.** Crossing `y=63/64` still only emits an event. Territorial defence is not in this
  initiative.
- **Any change to `baseline`, `reference` or `individual` proposal behavior.** They gain no verb. `REQ-MOK-034`'s
  requirement that their outcomes stay frozen is honoured for `baseline` and is amended for the other two only for the
  resource-composition correction, which is a separate obligation with its own requirement.
- **Model-backed decisions, persistence, structured output, a batch runner, any new package or dependency.**
- **Entropy substreams.** Still declined, for the reason Phase 2 declined them.
- Combat animation, sound, damage numbers on the map, or any observer feature beyond presenting what the engine reports.

## Principles and immutable constraints

- **The engine is the only authority.** It validates every proposal against current state, and a rejected proposal
  consumes the opportunity and mutates nothing. A targeted action is validated against the target's state too, and a
  source that proposes an attack on a Mokiterion that has died or moved out of contact gets a rejection, not a
  retarget.
- **No population-level trigger, and no population aggregate read anywhere.** This is `README.md`'s prohibition and it
  is not narrowed. A Mokiterion decides from its own observation. It may not read a census, a survivor count, a
  territory total, or any other agent's attributes beyond what its observation carries.
- **Determinism is absolute.** Resolution takes no entropy draw, so `REQ-MOK-009` and `INT-MOK-001`'s identical-results
  measure are untouched by the resolution rule itself.
- **No inert value.** Every field this initiative adds to the observation is read by the source it is added for. The
  attribute this initiative exists to make readable was itself the precedent for that constraint, under `SPEC-MOK-003`
  rule 4.5.
- **A defender's response is a decision, not a computation.** If the engine chose it, the most consequential moment in
  the world would be hard-coded, and Phase 5's model would arrive to find it already decided.
- **Appended, not inserted.** New rules go after rule 19, on the precedent and for the stated reason of `WO-MOK-010`:
  inserting renumbers rules that the specifications, verification contracts, verification records, retained evidence
  and source comments all cite.
- **The identifier remains the join key and the acting order.** Ordering by anything else would move every outcome.

## Risks and assumptions

- **Fact: `SPEC-MOK-001` already authorizes this direction twice**, in rule 12's "a consumer is a later governed
  change" and in rule 3's stated reason for withholding `fear`. Neither is a licence to implement, and both mean the
  amendment this initiative makes is the one the specification anticipated rather than a reversal.
- **Fact: one sentence elsewhere breaks the moment `fear` is carried.** The *Name* subsection justifies a name's
  absence from the observation "for the same reason `fear` is not". That reason is about to stop existing. A name's
  absence stands on its own ground — no decision source reads it — and the subsection must say so itself instead of
  borrowing.
- **Fact: contact is rare, and the arithmetic is the project's own.** `REQ-MOK-032` records that the expected number of
  other Mokiterions inside a Chebyshev box of radius `r` is `11 * (2r+1)^2 / 16384`. At the adjacency radius of `1`
  that is about `0.0060` per agent-tick, or roughly 72 contact agent-ticks per 1,000-tick run before any Mokiterion
  deliberately closes distance. This is why `approach` is in this initiative rather than a later one: without a verb
  that raises the contact rate, combat would be verifiable only through constructed states — the failure mode
  `REQ-MOK-032` rejected a four-cell fear threshold for.
- **Assumption, and the largest one: response selection is meaningful from `fear`, `health`, `energy` and
  attacks-suffered alone.** Perceived relative strength is withheld. If a defender's choice collapses to one branch on
  every declared seed, this initiative has built a mechanism whose most interesting decision is degenerate, and the
  measurement that shows it is the deliverable rather than a failure.
- **Risk: deterministic resolution plus ascending-identifier acting order may hand `M01` a systematic advantage.**
  Nothing draws to wash it out. Rule 12 already documents the within-tick asymmetry this comes from — "two mutually
  adjacent Mokiterions may not both register the encounter on the same tick" — and combat converts an asymmetry in
  bookkeeping into an asymmetry in survival. This is bounded by measurement rather than assumed away, and the bound is
  a stated obligation.
- **Risk, and the one this initiative cannot avoid paying: the additivity property does not survive intact.** Absorbing
  the high-class accumulation effect changes what `reference` and `individual` do, and `REQ-MOK-034` currently states
  that their outcomes "are frozen". How much it costs depends on where the correction is placed, and the placement is
  constrained rather than free: correcting the *sources'* non-waste condition moves `reference` and `individual` and
  cannot reach `baseline`, whose candidate list applies no waste condition at all; correcting the world instead — rule
  16's uniform class selection, rule 15's amount, rule 9's eat effect, or the food table — would move every policy's
  runs, and rule 16 would move the shared stream's draws as well. The mechanism is therefore chosen from the first set,
  on measurement inside the work order, and **`baseline` is the one source that can be held byte-identical, and it is.**
- **Risk: the reporting gap the tick order creates.** Rule 2 runs each Mokiterion's whole cycle inside its own turn, so
  a Mokiterion attacked by a higher identifier has already emitted its `survival_changed` line for that tick. The
  damage would appear in no per-agent record. Every event this initiative adds must therefore carry the transitions it
  causes, or the text stream will hold strictly less than the state does — and `REQ-MOK-010`'s stream is what every
  test, every evidence capture and the observer all read.
- **Risk: the event vocabulary is a public interface.** `EventType::ALL` is a `pub const` array whose length is part of
  the engine's public surface, and `SPEC-MOK-003` rule 11 requires every event type to map to an authorizing
  requirement. New event types are not free; they are counted, and the count is stated.
- Assumption: eleven verbs is the whole of the contract `README.md` promises, so this initiative closes the action
  contract rather than opening an expandable one.
- Assumption: the engine's dependency table stays empty. Integer arithmetic on existing attributes needs nothing.
