# `REQ-MOK-059`: what every rule, source and validation path reads

| Field | Value |
|---|---|
| Requirement | `REQ-MOK-059`, *Read no population aggregate anywhere a decision or a rule is made* |
| Owner | assurance owner |
| Method | `static-analysis` — a documented manual examination, which the requirement's *Open decisions* admits |
| Candidate | `7c4aef3967406c05d80da963695898b77f5329e9` |
| Source examined | `mokiterions-core/src/simulation.rs`, lines 1–2972 (everything above `#[cfg(test)]`) |
| Date | 2026-08-20 |

`REQ-MOK-059` is discharged "by a documented static examination of every rule evaluation, every
`DecisionSource` implementation and every validation path, enumerating what each reads — **not by an
assertion that nobody wrote the forbidden line**". So this file does not argue that the prohibited shape
is absent. It enumerates the readers and states, for each, what it reads.

It is written to be re-derived rather than trusted: the requirement's *Constraints* say the enumeration
"is expected to be re-derived when rules or sources change, on the ground that a static claim about
reads goes stale exactly when the code moves". §1 is the mechanical part, so a re-derivation starts from
a command rather than from this file's prose.

---

## 1. The candidate set, found mechanically

Anything that could read a set has to iterate, count, sum, search or fold one. Every such expression in
the production half of the engine, found by pattern rather than by reading for suspicion:

    $ sed -n '1,2972p' mokiterions-core/src/simulation.rs > prod.rs        # above #[cfg(test)]
    $ grep -nE '\.count\(\)|\.sum\(\)|\.len\(\)|\.filter\(|\.any\(|\.all\(|\.position\(|\.max\(\)|\.min_by|\.max_by|\.fold\(' prod.rs

46 expressions, in 17 functions. Two more readers were added by hand because they search a collection
with `.find()`, which the pattern above does not name: `Observation::nearest_in_contact` and
`Observation::nearest_beyond_contact`. Every one of the 48 is classified below, and none is omitted.

The classification uses three verdicts:

| Verdict | Meaning |
|---|---|
| **own** | reads the acting Mokiterion's own state or its own observation — permitted, and the requirement's *Failure and boundary behavior* says counting entries in one's own observation is explicitly not a violation |
| **named** | reads one or two individuals the rule names — permitted, and bounded, by the *Required response* |
| **outside** | reads a set, and is outside the obligation — each with the reason, as the *Constraints* require rather than permit |

## 2. The four decision sources

| Source | Reads | Verdict |
|---|---|---|
| `BaselineDecisionSource::decide` | `observation.valid_actions` and its `.len()`, for one entropy selection. Nothing else. | **own** |
| `ReferenceDecisionSource::decide` | via `best_fitting_co_located_food`, `best_fitting_distant_food`, `fits`, `allows`, `valid_moves`: `observation.perceived_food`, `observation.satiety`, `observation.energy`, `observation.valid_actions` | **own** |
| `IndividualDecisionSource::decide` | the same, with `fits_within_tolerance` in place of `fits`, which adds `observation.waste_tolerance` | **own** |
| `SocialDecisionSource::decide` | `observation.suffered` (branch 1), `observation.fear` (branches 1, 3, 4), `tolerant_survival_choice`'s reads (branch 2), `observation.perceived_mokiterions` via `nearest_in_contact` and `nearest_beyond_contact` (branches 3, 4), `tolerant_movement_choice`'s reads (branch 5) | **own** |

Every read in the column above resolves to a field of the `Observation` the source was handed. The
signature is what makes that checkable rather than a claim: `fn decide(&mut self, observation:
&Observation, entropy: &mut DecisionEntropy<'_>) -> Action`. A source holds no reference to `Simulation`,
no `&[Agent]`, no world state, and the four structs are `BaselineDecisionSource`,
`ReferenceDecisionSource`, `IndividualDecisionSource` and `SocialDecisionSource` — all unit or
`#[derive(Default)]` with no fields, so none of them carries state between opportunities either.

Three `.len()` reads sit inside sources — `valid_actions.len()` at line 897, `moves.len()` at 953 and
1014. All three are lengths of the observation's own lists, taken to bound an entropy selection. The
requirement addresses this case directly: "A source reading the number of entries in *its own
observation* … does **not** violate this requirement."

`nearest_in_contact` and `nearest_beyond_contact` use `.find()`, returning the first match and no count.
They can be read as the temptation `REQ-MOK-059`'s rationale names — "target selection would like a
ranking" — so it is worth saying what they do instead: rule 3 has already sorted
`perceived_mokiterions` by ascending distance then identifier, so the first match *is* the nearest and
lowest-identifier one, and neither function computes an order, a count, or a comparison across the
list.

## 3. The observation itself

`REQ-MOK-059` requires that "the observation carries no aggregate". The struct is 13 fields, at line
626, and the enumeration is the field list:

| Field | What it is |
|---|---|
| `tick`, `agent_id`, `position`, `territory` | the observer's own identity and place |
| `health`, `satiety`, `energy`, `fear`, `waste_tolerance` | the observer's own attributes |
| `suffered: Vec<SufferedAttack>` | attacks the observer took, each naming one attacker and one damage |
| `co_located_food: Vec<String>` | resource identifiers underfoot |
| `perceived_food: Vec<PerceivedFood>` | one entry per perceived resource: class, distance, direction |
| `perceived_mokiterions: Vec<PerceivedMokiterion>` | one entry per perceived Mokiterion: identifier, distance, direction |
| `valid_actions: Vec<Action>` | rule 4's candidate list |

No living count, no population total, no territory census, no world food total, no tick-scoped
statistic, no summary of any set. Every list is per-individual entries, and the two perceived lists are
bounded by `PERCEPTION_RADIUS`, so their lengths are facts about one Mokiterion's perception rather than
about the population — which is the distinction the requirement draws: "between counting what one
individual can see and counting what exists".

`fear` and `suffered` are the two fields this change adds, and both are the observer's own. `suffered`
carries the attacker's identifier and the damage and **not a reference to the attacker**, so a source
reading it cannot reach the attacker's health, position or fear.

The builder, `Simulation::observation` at line 2163, iterates the population once at line 2194
(`.filter(|(index, other)| *index != agent_index && other.alive)`) and then by distance. That iteration
produces the per-individual list above and no derived number: **outside** in form, **own** in what it
yields, and it is the only place a source's inputs are drawn from the population at all.

## 4. Rules 1 to 26

Every behavioral rule, with what its evaluation reads. Rules with no reader in §1's set read only the
acting Mokiterion's own fields, and that is stated rather than implied.

| Rule | Where | Reads | Verdict |
|---|---|---|---|
| 1 world, 2 attributes | struct definitions and constants | nothing at evaluation time | **own** |
| 3 perception | `observation`, 2163 | the acting agent's own state; the population once, to emit per-individual perceived entries within the radius | **own** (§3) |
| 4 candidate list | `observation`, 2168–2226 | the acting agent's position and the resources at it; the four directions' in-bounds test | **own** |
| 5 turn order, termination | `run_tick`, 2113; `step`, 2080 | `0..self.agents.len()` bounds the pass; `agents.iter().all(|a| !a.alive)` is the extinction test | **outside**, §5 |
| 6 validation | `apply_action`, `validate_targeted`, 2248/2396 | see §6 | **named** |
| 7 application | `apply_action`, 2248 | the acting agent, and for a targeted verb the one named target | **named** |
| 8 movement | `apply_move`, 2340 | the acting agent's position and the world bounds | **own** |
| 9 eat | `apply_action`, 2287 | the acting agent's satiety; one resource found by identifier *and* position | **named** |
| 10 sleep | `apply_action` | the acting agent's energy | **own** |
| 11 wait | `apply_action` | nothing | **own** |
| 12 fear | `apply_survival`, 2797 | the acting agent's own `fear`, and the `perceived_company` boolean carried from *its own* observation at line 2121 | **own** |
| 13 survival, death | `apply_survival`, 2777 | the acting agent's own health, satiety, energy | **own** |
| 14 initial placement | `Simulation::new`, 1753 | a territory's resource count | **outside**, §5 |
| 15 regeneration | `regenerate_food`, 2842 | a territory's resource count against its capacity | **outside**, §5 |
| 16 class selection | `regenerate_food`, 2865 | `FoodClass::ALL.len()`, a constant; a free coordinate | **outside**, §5 |
| 17 events | `emit` | the record being written | **own** |
| 18 final summary | `summary`, 2896; `food_counts`, 2920 | survivors, deaths, per-territory living counts, per-territory food | **outside**, §5 |
| 19 tolerant source | §2 | the observation only | **own** |
| 20 contact | `in_contact`, 2378 | two agents by index: both `alive` flags and both positions | **named** |
| 21 targeted application | `apply_targeted_action`, 2465 | the acting agent and the one target the proposal named | **named** |
| 22 strike | `resolve_strike`, 2573 | the striker's own `energy` and `health` for the damage; the target's `health` | **named** |
| 23 threat | `resolve_threat`, 2652 | the target's `fear` | **named** |
| 24 surrender | `resolve_surrender`, 2694 | the subject's `satiety` and the recipient's `satiety` | **named** |
| 25 suffered window | `resolve_strike` 2600 writes, `run_tick` 2150 clears | the target's own record; the acting agent's own record | **named** |
| 26 social source | §2 | the observation only | **own** |

Rule 22's damage is the one place an arithmetic value is computed from attributes, and it is computed
from **one** Mokiterion's: `let condition = u16::from(striker.energy) + u16::from(striker.health)`, at
line 2580. It reads nothing about the target except the `health` it subtracts from, and nothing
about anyone else at all. `REQ-MOK-059` names this case as permitted and bounded.

## 5. The seven aggregate reads, and why each is outside the obligation

`REQ-MOK-059`'s trigger is "the evaluation of any `SPEC-MOK-001` rule that governs a Mokiterion's
behavior, any consultation of a decision source, and any validation of a proposed action". The engine
holds seven readers of a set. None is any of those three, and the requirement's *Constraints* require
them enumerated with the reason rather than omitted.

| # | Reader | What it reads | Why outside |
|---|---|---|---|
| 1 | `Simulation::new`, 1753 | resources already placed in a territory | Rule 14 places resources. It decides nothing about any Mokiterion and runs before any Mokiterion acts. Named as outside by the requirement's own boundary paragraph. |
| 2 | `regenerate_food`, 2842–2869 | a territory's resource count against capacity; the free coordinates | Rules 15 and 16. The requirement states these satisfy it "as they stand", and that a change making a *Mokiterion's* behavior depend on that count would violate it. It does not. |
| 3 | `summary`, 2896–2907 | survivors, deaths, living per territory | Rule 18's report. "A run's terminal summary reporting survivors does not violate this requirement, because nothing reads it back" — and nothing does: `summary` is called once, at the end of `run`, and its `RunSummary` is returned to the caller, never to a rule. |
| 4 | `food_counts`, 2920 | remaining food per territory and class | Rule 18's report, same reason. |
| 5 | `entity_initialization_events`, 2027 | `foods.len()`, `agents.len()`, `Territory::ALL.len()` | A `Vec::with_capacity` hint and a field of the initialization record. Reporting and allocation; no branch reads either. |
| 6 | `Simulation::step`, 2080 | `agents.iter().all(\|agent\| !agent.alive)` | **The one that deserves the most care.** This is a genuine aggregate over the population, read by the engine, on every tick. It is the extinction test, and what it decides is whether the *run* ends — `TerminationReason::Extinction`. It governs no Mokiterion's behavior: it is read after every Mokiterion has acted, its result reaches no source, no validation and no rule, and it cannot change any proposal or any resolution. Recorded here explicitly rather than folded into "reporting", because it is neither placement nor a report, and a reader checking this enumeration against the requirement should be given it rather than have to find it. |
| 7 | `run_tick`, 2113 | `0..self.agents.len()` | The bound of the acting pass. It decides *that* each living Mokiterion is given one opportunity, which is rule 5's turn order, and not *what* any of them proposes. No source can see it; a source is handed one observation and cannot tell how many opportunities the tick contains. |

`Simulation::snapshot`, 1948–1980, also reads a living count and a death count. It is the observer's
input, and the requirement is explicit: "The observer displaying a population count does not violate
this requirement. `mokiterions-tui` decides nothing." It is listed here for completeness and not counted
among the seven, because it is not on the engine's own decision path at all.

## 6. Every validation path

| Path | Reads | Verdict |
|---|---|---|
| `Observation::allows`, 805 | the observation's own `valid_actions` | **own** |
| `apply_action`'s rule 6 checks, 2248 | the acting agent's `alive`; for `eat`, one resource matched on identifier *and* position, at 2287 | **named** |
| `validate_targeted` check 1, 2408–2417 | the target found by identifier (`.position(\|other\| other.id == target)`), then its `alive` flag, then the index comparison against the actor | **named** |
| `validate_targeted` check 2, 2422 | the distance between the two named agents, against `PERCEPTION_RADIUS` | **named** |
| `validate_targeted` check 3, 2431 | `in_contact(actor, target)` — two `alive` flags and two positions | **named** |
| `validate_targeted` check 4, 2445–2451 | the **actor's own** `suffered` record, searched for the target's identifier | **own** |
| rule 6 check 5 | `apply_targeted_move`, 2515 — the resulting coordinate against the world bounds | **own** |

The `.position(...)` at 2408 is a scan of the population, and it is the read most likely to be
mistaken for a violation. What it computes is a name-to-index lookup: its result is one individual or
`target_unknown`, and it produces no count, no ranking and no summary. The requirement permits reading
"the state of any individual Mokiterion the rule names", and a lookup is how a named individual is
reached in a `Vec`.

Each rejection reason — `target_unknown`, `target_dead`, `target_is_actor`, `target_not_perceived`,
`target_not_in_contact`, `target_not_in_record`, `out_of_bounds`, `target_co_located`, `agent_dead` —
names the unmet precondition of that one proposal, derived from the two Mokiterions and the world cell.
None is derived from a population state, and none varies with how many Mokiterions are alive.

## 7. `fear`'s writers, and every path writing a second Mokiterion's state

`REQ-MOK-059` does not itself demand these two lists; `VER-MOK-016`'s retention does, and they belong
here because they are the write-side counterpart of the same bound.

**`fear` has exactly two writers in production code.**

| Line | Writer | Whose `fear` | Driver |
|---|---|---|---|
| 2797–2801 | `apply_survival`, rule 12 | the acting Mokiterion's own | the `perceived_company` boolean, computed at line 2121 from that Mokiterion's own observation, `+FEAR_INCREASE` or `-FEAR_DECREASE`, saturating at both bounds |
| 2660 | `resolve_threat`, rule 23 | the **target's** | `+THREAT_FEAR_INCREASE`, saturating at `ATTRIBUTE_MAX` |

No third path writes it. Neither writer reads a population state: rule 12's driver is the acting
Mokiterion's own perception — the case `REQ-MOK-059`'s *acceptance examples* single out as satisfying the
requirement — and rule 23's is a constant applied to one named target.

Rule 12's driver is read at line 2121, **before** the source is consulted and the action applied, and
carried into `apply_survival` as a parameter. That ordering is deliberate and is stated in the comment
there: the write is from *this* observation's perceived list, not from a re-perception after the action.

**Five paths write a second Mokiterion's state.** All five are resolutions, all five name their second
party, and all five are reached only through `validate_targeted`.

| Line | Path | Second party's field written |
|---|---|---|
| 2594 | `resolve_strike`, rule 22 | the target's `health`, by the damage |
| 2600 | `resolve_strike`, rule 25 | the target's `suffered` record, one entry pushed |
| 2606 | `resolve_strike`, rule 13 | the target's `alive`, set false where `health` reached 0 |
| 2660 | `resolve_threat`, rule 23 | the target's `fear` |
| 2707 | `resolve_surrender`, rule 24 | the recipient's `satiety`, by the transfer |

Two further writes in the same functions are the **actor's** own: the striker's `energy` at 2587
(`STRIKE_ENERGY_COST`), and the subject's `satiety` at 2702 (half, forfeited). And one more clears the
actor's own record: `run_tick` at 2150, rule 25's close.

So the whole write surface of one Mokiterion onto another is five lines, in three functions, each
reached through one validation, each on one named target. Nothing writes a set, nothing writes a third
party, and `a_resolution_touches_nobody_but_its_two_parties` asserts the negative side of that at
runtime.

## 8. Verdict, and what this does not settle

**`REQ-MOK-059` is met at this candidate.** Every read in every decision source, every behavioral rule
evaluation and every validation path resolves to the acting Mokiterion's own state, its own observation,
or an individual the rule names. The seven readers of a set are enumerated in §5 with the reason each is
outside the obligation, and the one that most deserved scrutiny — the extinction test — is recorded
rather than glossed.

The shape `REQ-MOK-059`'s rationale warns about is worth naming against the measurement it would
corrupt: `REQ-MOK-058` is **failing** at this candidate, and the cheapest way to make it pass would be
for the engine to suppress combat while few Mokiterions are alive. No such reader exists — §5 is the
complete list of readers of a living count, and both entries that read one, `step` and `summary`, are
read by nothing behavioral. The failure is left standing instead, which is the outcome this requirement
exists to force.

What this does not settle: it is a static examination of one commit, it is manual, and it goes stale the
moment a rule or a source moves. It establishes nothing about whether the reads are *correct* — only
about what is read. And it is not a verification verdict: `VER-MOK-016` is the contract and
`VREC-MOK-012` will be the record.
