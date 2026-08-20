# `WO-MOK-012` escalation: `REQ-MOK-049`'s lethality bound is missed on every declared seed, and `surrender` never applies

| Field | Value |
|---|---|
| Candidate this escalation was raised at | `7c4aef3967406c05d80da963695898b77f5329e9` on `feature/phase-3-definition` |
| Suite there | 246 passing, **3 failing**, 0 ignored |
| Stop conditions triggered | **5**, **6**, **7**, and **11** |
| Date raised | 2026-08-20 |
| Resolution | §10. Package **A** of §8, selected by the product owner on 2026-08-20; stop conditions 5, 7 and 11 discharged |
| Suite after it | 248 passing, **1 failing**, 0 ignored — the same 249 tests, none added, removed or `#[ignore]`d |
| Still open | §11. Stop condition **6**, on data that did not exist when §1 was written |

**§1 to §9 are the record as it stood at `7c4aef3`, and they are not rewritten.** Work was stopped there,
nothing in §1 to §8 had been implemented, and §6 put the choice to the product owner with each option
measured because `REQ-MOK-049` states that "the measurement is reported and the owner decides". The answer
came, and §10 records it, what it implemented, and what implementing it measured. Two claims in the earlier
sections were falsified by that measurement rather than merely superseded by it; both are corrected where
they stand rather than edited away, on the pattern `REQ-MOK-049`'s corrected bullet sets.

## 1. The three failing cases, and the four obligations behind them

| Case | Obligation | State |
|---|---|---|
| `tests/viability.rs :: the_social_source_keeps_the_world_habitable_and_combat_lethal` | `REQ-MOK-049`, lethality | **0 combat deaths on all five declared seeds** |
| `tests/decisions.rs :: every_targeted_verb_applies_somewhere_in_the_declared_matrix` | `REQ-MOK-043`, `VER-MOK-012` oracle 4 | **`surrender` never applies**, in any of the 5,000 opportunity-ticks measured |
| `tests/viability.rs :: no_identifier_series_is_monotone_in_identifier_or_correlated_beyond_the_band` | `VER-MOK-012` oracle 5 | fails **downstream**: twelve strikes across five seeds cannot populate three per-identifier series — **and see §11: it fails on its own terms once the series are populated** |
| — | `REQ-MOK-051`, composition ceiling | **unimplemented**, 14 of 15 cells fail; deferred by product owner decision of 2026-08-20 |

`REQ-MOK-049`'s survivor floor of five is **met** at this candidate — 6, 4, 8, 4, 5 — but that is not a pass:
the requirement states that "both bounds hold simultaneously on each seed" and that "a matrix that trades one
against the other does not satisfy this requirement". This candidate is a matrix that trades one against the
other, in the peaceful direction.

Stop condition 6's first half is the monotonicity band, "a finding about turn order and the owner's to
weigh". It is **not** that here, and it should not be escalated as one: it fails for want of data. The
identifier-exchange test — the same condition's second half, the one that would mean resolution reads an
identifier and is a defect — **passes**.

> **Corrected 2026-08-20, after §10.** The last paragraph was right about this candidate and wrong about the
> mechanism, and the difference matters because it is the difference between a downstream failure and a
> finding. At `7c4aef3` the band failed for want of data — twelve strikes cannot populate three series — and
> "it should not be escalated as one" followed correctly from that. Package A's amendment populated the
> series, and the band **still fails**, now on 68 combat events with both attack series outside it. It is
> stop condition 6's first half proper. **§11 escalates it as one**, and the sentence above should be read as
> the state at `7c4aef3` and not as a judgment that survived the amendment. The identifier-exchange test still
> passes, so the defect half of the condition is still not triggered.

## 2. What is measured

`--policy social --density 0.75 --ticks 1000 --trace-actions`, the configuration `REQ-MOK-049` states its
obligation over, on `VER-MOK-002`'s declared seed set. Verb counts are proposals; §5's tables count
applications where that distinction matters.

| Seed | Survivors | Combat deaths | Other deaths | `attack` | `fight` | `threaten` | `surrender` | `retreat` | `approach` | `avoid` | Meals |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 6 | **0** | 6 | 0 | 0 | 0 | **0** | 0 | 196 | 1,606 | 304 |
| 1 | 4 | **0** | 8 | 0 | 0 | 0 | **0** | 0 | 179 | 1,465 | 255 |
| 42 | 8 | **0** | 4 | 1 | 3 | 227 | **0** | 1 | 242 | 1,058 | 298 |
| 123 | 4 | **0** | 8 | 2 | 6 | 227 | **0** | 2 | 140 | 872 | 205 |
| 777 | 5 | **0** | 4 | 0 | 0 | 0 | **0** | 0 | 216 | 1,328 | 273 |

`avoid` outnumbers `attack` **6,329 to 3**. Every strike in the entire declared matrix lands at tick 1, 2 or
3, between Mokiterions the initialization happened to place already in contact; seeds 0, 1 and 777 place none
and record no strike at any tick. Of 457 contacts across the matrix, **454 resolve as `threaten`** — `fear`
stood at or above `30` in 99.3% of them.

## 3. The cause: `fear`'s driver and the engagement gate are scoped to different radii

Rule 12 raises `fear` by `10` for every tick the acting Mokiterion **perceives** company, at
`PERCEPTION_RADIUS` `16`. `REQ-MOK-048` branches 3 and 4 engage only while `fear` is below `30`, and branch 3
requires **contact**, at `CONTACT_RADIUS` `1`.

    tick 1   company perceived at distance 16      fear  0 -> 10    approach
    tick 2   closed to 15                          fear 10 -> 20    approach
    tick 3   closed to 14                          fear 20 -> 30    approach
    tick 4   closed to 13, fear now >= 30          fear 30 -> 40    AVOID
    …        fear saturates at 100 and stays

`fear` crosses the gate on the third perceiving tick. Closing sixteen squares takes fifteen. **The approach
can never complete**, and `fear` sits at exactly `100` on 39% of all creature-turns, so no threshold
materially below `100` routes anywhere but `avoid`. The gate had to change shape, not value — which is what
§5 measures.

Three consequences follow, and the third is the one that matters most for the floor:

1. Contact is reached only where initialization supplied it, so `attack` is 3 in 5,000 opportunity-ticks.
2. Nobody is struck, so no `suffered` record is ever opened, so branch 1 never fires — and `surrender`,
   `retreat` and `fight` are all branch 1's. `surrender` additionally needs `fear` `>= 60` **at the moment of
   being struck**, and being struck needs the striker's `fear` below `30`; under one shared driver those two
   windows barely overlap. This is why `surrender` is not merely rare but structurally unreachable.
3. `avoid` displaces rule 19's seek-move, because branch 4 sits **ahead** of branch 5. Meals fall from
   378–417 under `individual` to 205–304 under `social`. **The survivor count is depressed by starvation, not
   by fighting** — in a candidate where no fighting happens at all.

## 4. The option the product owner selected, measured, does not work

On 2026-08-20 the product owner selected **"Rescope the fear rise"**: rule 12 becomes `company within
CONTACT_RADIUS -> fear +10 / otherwise -> fear -5`. The framing put to them stated the governance cost — it
amends an approved Phase 2 requirement and invalidates `WO-MOK-010`'s measured `fear` distribution — and
predicted that "a Mokiterion crossing 16 squares arrives at `fear` 0, engages, and its `fear` then climbs
while the fight lasts".

That prediction holds. The requirement still fails.

| Seed | Survivors | Combat deaths | Other deaths | `surrender` | Meals |
|---|---:|---:|---:|---:|---:|
| 0 | **1** | 5 | 6 | **0** | 92 |
| 1 | **2** | 3 | 7 | **0** | 109 |
| 42 | **1** | 2 | 9 | **0** | 96 |
| 123 | **3** | 1 | 8 | **0** | 128 |
| 777 | **2** | 3 | 7 | **0** | 80 |

The lethality bound is recovered on all five seeds. **The survivor floor of five now fails on all five**, at
1–3 against a floor of 5 — worse than the 4–8 it replaced. Meals fall further, to 80–128. And `surrender` is
still never proposed, so `VER-MOK-012` oracle 4 fails exactly as before.

Why: with `fear` driven by contact alone, a Mokiterion in lasting contact saturates and then `threaten`s
every tick — 535 to 818 times per seed — and branch 3 sits ahead of rule 19's seek-move, so it never walks to
food again. The candidate trades a peaceful matrix for a depopulated one, which is the same failure
`REQ-MOK-049` names in the other direction: "a world made uninhabitable by a combat-driven collapse of
foraging behavior fails it as squarely as one where they killed each other".

This is reported rather than worked around. The selection was made on a measurement that did not exist yet,
which is why it was put with that caveat, and the decision was remade against the numbers on the same date.
§6 records what was answered. **The selection stands**, so this section is a measurement of an option that
was kept, not of one that was withdrawn, and §7 measures the combination the answers actually select.

## 5. What the requirement's own stated lever does

`REQ-MOK-049` names one legitimate lever and rules the alternatives out:

> **The `social` source's own ordering and thresholds are the one legitimate lever**, because they are that
> source's behavior rather than a rule's constant. `REQ-MOK-048`'s survival-first ordering — **a Mokiterion
> acts socially only when it is neither hungry nor tired** — is what this floor rests on…

**The emphasized clause is not what `REQ-MOK-048` specifies.** Branch 2 is "a tolerated co-located resource,
or `energy` below `REFERENCE_SLEEP_THRESHOLD`" — food *underfoot*, or exhaustion. A Mokiterion that is hungry
with food four squares away is neither, so it falls to branch 3 or 4 and engages; rule 19's case 3 seek-move
is in branch **5**, behind society. At a default density of 0.75% a hungry Mokiterion is almost never standing
on food, so the property `REQ-MOK-049` says its floor rests on is a property the specified branch order does
not have. That is an inconsistency between two approved requirements, and it is the mechanism behind §3's
third consequence.

Two changes were measured against it, both wholly inside the named lever, and the matrix is the same five
seeds at the same density and tick count.

**Change A — the branch order.** Rule 19's case 3 moves ahead of branches 3 and 4: food perceived outranks
company perceived, and company outranks aimless wandering. It draws nothing, so branch 5's single draw is
unmoved. Rule 12 is untouched.

**Change B — the engagement threshold.** `ENGAGEMENT_FEAR_THRESHOLD` alone; the two answer thresholds of
branch 1 are separate constants and are not touched.

| Rule 12 | Order | `ENGAGEMENT` | Survivors | Combat deaths | `surrender` applied | Verdict |
|---|---|---:|---|---|---|---|
| perceive | as specified | 30 | 6, 4, 8, 4, 5 | 0, 0, 0, 0, 0 | 0 | shipped candidate — lethality fails |
| perceive | as specified | 100 | 5, 3, 4, 4, **1** | 1, 2, 2, 3, 1 | yes | floor fails on four seeds |
| contact | as specified | 30 | **1, 2, 1, 3, 2** | 5, 3, 2, 1, 3 | 0 | owner's selection — floor fails on all five |
| contact | food first | 30 | **1, 4, 2, 3, 2** | 10, 8, 9, 9, 10 | 1 seed only | floor fails on four seeds |
| perceive | food first | 30 | 12, 12, 9, 10, 12 | 0, 0, 0, 0, 0 | 0 | lethality fails |
| perceive | food first | 60 | 11, 10, 11, 10, 8 | 0, 0, 0, **1**, 0 | 1 seed only | lethality fails on four seeds |
| perceive | food first | 90 | 9, 9, 10, 9, 11 | 1, 1, 2, 3, **0** | yes | lethality fails on seed 777 |
| **perceive** | **food first** | **95** | **9, 10, 9, 9, 11** | **1, 2, 2, 3, 1** | **5, 10, 8, 6, 7** | **both bounds met on all five** |
| perceive | food first | 100 | 9, 8, 10, **6**, 9 | 1, 2, 2, 3, 1 | 8, 8, 6, 8, 7 | **both bounds met on all five** |

Only the last two rows satisfy `REQ-MOK-049` simultaneously on every declared seed, and in both of them **all
seven targeted verbs apply**, so `VER-MOK-012` oracle 4 passes too. Both leave rule 12 as Phase 2 approved
it, so `WO-MOK-010`'s measured `fear` distribution stands, and the 90-cell byte-identity of
`post/byte-identity.txt` is unaffected — the three pre-existing sources read neither the branch order nor the
threshold.

Neither change is a tuning commit. Change A makes `REQ-MOK-048` do what `REQ-MOK-049` describes it as doing;
change B is a constant `REQ-MOK-049` explicitly consigns to `REQ-MOK-048`'s governance — "changing those
thresholds is a change to `REQ-MOK-048` and is governed there… and it is not forbidden".

Both need an approved amendment to `REQ-MOK-048` before either is written. That is stop condition 11, and
this record is not an authorization.

## 6. What the product owner answered

The four questions of §5 were put on 2026-08-20 with the tables above as their framing, and answered the
same day. The answers are recorded verbatim as selected, before any reading is placed on them.

| # | Question | Answer selected |
|---|---|---|
| 1 | withdraw the "rescope the fear rise" selection? | **"Keep it, and lower the floor"** — rule 12 stays contact-driven, and `REQ-MOK-049`'s survivor floor "drops to one or two to match the world that produces" |
| 2 | amend `REQ-MOK-048`'s branch order? | **"Amend the branch order"** — rule 19's case 3 moves ahead of branches 3 and 4 |
| 3 | what `ENGAGEMENT_FEAR_THRESHOLD` becomes | **`95`**, selected against the row recording survivors 9, 10, 9, 9, 11 |
| 4 | does the floor of five stand? | **"Ratify five as it stands"** — "the floor stays at five of twelve" |

`REQ-MOK-051` was not re-put; its deferral of 2026-08-20 to a second amendment stands, and `WO-MOK-012`
cannot reach `implemented` while it does, whichever way the rest is decided.

**Answers 1 and 4 cannot both be written.** Answer 1 lowers the floor to one or two; answer 4 keeps it at
five. Nothing in the implementation can satisfy both, and no reading reconciles them, so neither has been
written into an artifact. §7 measures which of them the world can actually support, and §8 puts the
remaining choice.

**Resolved in §10.** Answer 1 was withdrawn by the product owner on 2026-08-20 and answers 2, 3 and 4 were
honored verbatim. This table stands unedited: answer 1 was selected, and a record that showed only the
answers that survived would make a withdrawn selection and a selection never made indistinguishable.

## 7. The combination the answers select, measured

Answers 1, 2 and 3 together select **contact-driven rule 12 + food-first branch order + gate `95`**. That
combination is in none of §5's rows: §5 measured the contact-driven rule only at gate `30`, and measured
gate `95` only under the perception-driven rule. It was therefore taken before anything was written, on the
same scaffold, the same five seeds, the same `--policy social --density 0.75 --ticks 1000 --trace-actions`.

The scaffold's fidelity was established first, on two rows already in §5. Re-run with every switch off it
reproduced the shipped candidate on all four columns — survivors 6, 4, 8, 4, 5, zero combat deaths, zero
`surrender`, and meals 304, 255, 298, 205, 273, which is §2's meals column to the unit. Re-run at
contact/food-first/`30` it reproduced that row of §5 exactly: 1, 4, 2, 3, 2 and 10, 8, 9, 9, 10. Both
controls agreeing is what makes the new rows below comparable to the old ones rather than a second reading.

| Rule 12 | Order | `ENGAGEMENT` | Survivors | Combat deaths | `surrender` applied | Meals | Floor 5 |
|---|---|---:|---|---|---|---|---|
| contact | as specified | 95 | 1, 2, 2, 2, 2 | 10, 10, 10, 10, 10 | 0, 2, 1, 0, 3 | 115–153 | ✗ ×5 |
| contact | food first | 30 | 1, 4, 2, 3, 2 | 10, 8, 9, 9, 10 | 0, 0, 0, 0, 1 | 153–219 | ✗ ×4 |
| contact | food first | 45 | 2, 3, 4, 2, 4 | 10, 9, 8, 9, 8 | 1, 1, 0, 0, 1 | 182–218 | ✗ ×5 |
| contact | food first | 50 | 2, 2, 1, 3, 3 | 9, 10, 10, 9, 8 | 1, 1, 0, 0, 2 | 148–189 | ✗ ×5 |
| **contact** | **food first** | **60** | **4, 3, 3, 4, 3** | **8, 9, 8, 8, 9** | **0, 6, 0, 1, 1** | **194–219** | ✗ ×5 |
| contact | food first | 70 | 2, 2, 2, 1, 3 | 10, 9, 9, 10, 9 | 0, 4, 6, 4, 3 | 135–227 | ✗ ×5 |
| contact | food first | 90 | 2, 4, 2, 3, 2 | 10, 8, 10, 9, 9 | 3, 4, 0, 2, 9 | 160–198 | ✗ ×5 |
| **contact** | **food first** | **95 — selected** | **3, 4, 2, 3, 2** | **9, 8, 10, 9, 10** | **1, 4, 0, 0, 8** | **151–198** | **✗ ×5** |
| contact | food first | 100 | 2, 4, 2, 3, 2 | 10, 8, 10, 9, 9 | 1, 4, 0, 0, 6 | 131–198 | ✗ ×5 |

Three findings, and the second is the one that decides the matter.

1. **The selected combination meets two of the three obligations.** Lethality holds on all five seeds, and
   `surrender` applies on three of them, so `VER-MOK-012` oracle 4 — which asks only that each verb apply
   *somewhere* in the declared matrix — passes. Answer 3's gate is what buys the `surrender`: at gate `30`
   the same variant applied it on one seed only.
2. **The survivor floor of five is not reachable under contact-driven rule 12 at any gate value.** The
   highest count on *any* seed anywhere in the band is `4`. The best worst-seed is `3`, at gate `60` — not at
   the selected `95`. Answer 4's five is not missed here by a margin that a better gate closes; it is above
   the whole curve. Answer 1 and answer 4 are therefore not merely inconsistent as text, they are
   inconsistent as physics.
3. **Answer 3's value is not the best one on its own branch.** Gate `95` was chosen against the
   perception-driven row's 9, 10, 9, 9, 11. Carried onto the contact-driven branch it gives 3, 4, 2, 3, 2,
   while gate `60` gives 4, 3, 3, 4, 3 with lethality and `surrender` still met and meals higher on four
   seeds. If the contact-driven rule is kept, the gate that goes with it is `60`.

Habitability stays depressed across the whole band: 131–227 meals against 205–304 at the shipped candidate
and 378–417 under `individual`. That is §3's third consequence surviving change A, because a Mokiterion held
in lasting contact saturates and `threaten`s rather than walking to food, and change A reorders only the
branch that fires when *no* company is in contact.

## 8. The one question the answers leave open

Exactly one decision remains, and it is a single choice among three coherent packages. Each is stated with
what it costs, because two of the three cost an approved Phase 2 requirement and the third costs answer 1.

| | Rule 12 | Gate | Floor | Result | What it costs |
|---|---|---:|---:|---|---|
| **A** | perception, **as Phase 2 approved** | 95 | **5** | 9, 10, 9, 9, 11 · 1, 2, 2, 3, 1 · `surrender` 5, 10, 8, 6, 7 | **answer 1 is withdrawn.** No Phase 2 amendment, `WO-MOK-010`'s measured `fear` distribution stands, and answers 2, 3 and 4 are all honored as given |
| **B** | contact | **60** | **3** | 4, 3, 3, 4, 3 · 8, 9, 8, 8, 9 · `surrender` on 3 seeds | **answer 3's `95` and answer 4's five are both withdrawn.** Amends Phase 2 rule 12, invalidating `WO-MOK-010`'s `fear` distribution; drops the floor from twelve-of-twelve's five to three |
| **C** | contact | 95 | **2** | 3, 4, 2, 3, 2 · 9, 8, 10, 9, 10 · `surrender` on 3 seeds | **answer 4 is withdrawn**, to the "one or two" answer 1 names. Same Phase 2 cost as B, and a floor two-sixths of the population, on a gate that is not the best on its own branch |

A is the only package that requires no Phase 2 amendment and no floor movement, and the only one where the
floor `REQ-MOK-049` was written around survives. B is the honest reading of answer 1 if the contact-driven
rule is wanted for its own sake, and it is the best the contact-driven world can do. C is answer 1, 2 and 3
taken literally with answer 4 given up.

None of the three is written. `REQ-MOK-049` reserves the floor to the product owner — "the evidence is what
the product owner ratifies the floor on" — and this record is not an authorization for any of them.

**Package A was selected on 2026-08-20**, against this table as its framing. §10 records the selection, the
five artifacts it amended, and what the implemented result measured.

## 9. What was not done

- No implementation adjustment was made to reach any bound. The measurements in §4, §5 and §7 were taken with
  a throwaway scaffold, behind environment switches, reverted each time before anything was written. §7's
  scaffold was re-applied a second time on 2026-08-20 and reverted again; the working tree is clean at
  `7c4aef3` and the suite there is 246 / 3 / 0. Two of §7's nine rows are controls that reproduce rows
  already in §2 and §5 to the unit, which is how the scaffold is shown not to be the measurement.
- **No floor, gate or branch order was written into any artifact.** `REQ-MOK-048` and `REQ-MOK-049` stand
  exactly as approved. §8's three packages are measured, not adopted, and the contradiction between answers 1
  and 4 was not resolved by choosing on the owner's behalf.

  > **True at `7c4aef3` and superseded by §10.** The branch order, the gate and the floor were all written on
  > 2026-08-20 *after* the product owner selected package A, each with its own amendment-record row and each
  > naming that selection as its approval. What this bullet asserts — that none of them was written *before*
  > the selection — is what it was for, and that remains true.
- No test assertion was relaxed, widened, removed or `#[ignore]`d. The three failing cases fail loudly, print
  their whole table, and name every seed. `post/test-census-reconciliation.md` accounts for the one rename
  name by name.
- No engine constant was tuned. `FEAR_INCREASE`, `FEAR_DECREASE`, `STRIKE_BASE_DAMAGE`,
  `STRIKE_CONDITION_DIVISOR`, `STRIKE_ENERGY_COST`, `THREAT_FEAR_INCREASE`, `SURRENDER_FEAR_THRESHOLD`,
  `RETREAT_FEAR_THRESHOLD` and `ENGAGEMENT_FEAR_THRESHOLD` all stand at their approved values.

  > **`ENGAGEMENT_FEAR_THRESHOLD` moved to `95` on 2026-08-20**, under `REQ-MOK-048`'s first amendment and the
  > product owner's selection of package A. The other eight still stand at their approved values, which is
  > what package A was chosen for.

## 10. Package A, written and implemented

The product owner selected **package A** on 2026-08-20, against §8's table. It withdraws answer 1 and honors
answers 2, 3 and 4 verbatim: `SPEC-MOK-001` rule 12 stays perception-driven exactly as Phase 2 approved it,
`REQ-MOK-048`'s branch order is amended so that rule 19's case 3 precedes the social branches,
`ENGAGEMENT_FEAR_THRESHOLD` becomes `95`, and `REQ-MOK-049`'s floor of five is ratified unchanged.

**Five artifacts were amended before any code was written, and each carries its own amendment-record row
naming that selection as its approval.** `SPEC-MOK-001` (rule 26's branch list, now six branches),
`REQ-MOK-048` (the order, the thresholds, and a new section recording what the first ordering could not do),
`REQ-MOK-049` (the ratification of five, and the correction of the bullet that described `REQ-MOK-048` as
having a property it did not have), `VER-MOK-012` (oracle 5's gate assertion, the draw-discipline
renumbering, the widened differential), and `WO-MOK-012` (four decision-table rows and the third defect).
`validate_engineering_artifacts.py --root .` passes at 116 artifacts, 0 errors, 0 warnings.

**One engine change carries all of it.** `ENGAGEMENT_FEAR_THRESHOLD` moved from `30` to `95`, and
`tolerant_movement_choice` was split into `tolerant_seek_choice` and `tolerant_search_choice` so that rule
19's case 3 could be hoisted ahead of rule 26's social branches **without** moving case 4 with it. Case 3
draws nothing from the shared stream; case 4 is the only part of rule 19 that draws. That is why the hoist is
a reordering of decisions rather than of the stream, and it is why `--policy individual` is untouched:
`tolerant_movement_choice` survives as the composition of the two halves in rule 19's own order.

| Seed | Survivors | Combat deaths | Other deaths | `attack` | `fight` | `threaten` | `surrender` | `retreat` | `approach` | `avoid` | Meals |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 9 | **1** | 2 | 8 | 0 | 11 | 5 | 2 | 412 | 1,069 | 369 |
| 1 | 10 | **2** | 0 | 14 | 0 | 7 | 10 | 2 | 305 | 618 | 371 |
| 42 | 9 | **2** | 1 | 14 | 3 | 18 | 8 | 4 | 328 | 595 | 336 |
| 123 | 9 | **3** | 0 | 15 | 6 | 43 | 6 | 6 | 329 | 574 | 326 |
| 777 | 11 | **1** | 0 | 8 | 0 | 49 | 7 | 0 | 414 | 986 | 389 |

Verb counts are proposals, on §2's convention. The resolutions behind them are `attack_resolved` 8, 14, 17,
21, 8 — which counts `attack` and `fight` together, since a `fight` resolves as a strike and reports the same
event — `threat_resolved` 11, 7, 18, 43, 49, and `surrender_resolved` 5, 10, 8, 6, 7.

**The implemented result reproduces the figures package A was selected against, to the unit.** Survivors 9,
10, 9, 9, 11 against §8's "9, 10, 9, 9, 11"; combat deaths 1, 2, 2, 3, 1; `surrender` 5, 10, 8, 6, 7. This
matters because those figures were taken on the throwaway scaffold of §7 and the selection was made on them:
had the written implementation measured anything else, the selection would have been made against a tree that
does not exist.

Three of the four stop conditions are discharged:

- **5 — `REQ-MOK-049`.** Both bounds hold simultaneously on all five declared seeds, with four survivors of
  margin at the worst seed. The floor was ratified rather than moved, and no damage, forfeit, survival or
  resource constant was tuned to reach it. `REQ-MOK-014`'s and `REQ-MOK-034`'s floors were the lever's stated
  precondition and are re-measured in the packet.
- **7 — a verb never applies.** `surrender` applies on all five seeds. `fight` applies on two of the five at
  the default density, which is inside oracle 4's obligation — it asks that each verb apply *somewhere* across
  the declared matrix — and is recorded here rather than left to be inferred from a passing test.
- **11 — a decision this work order does not carry.** The decision was put, answered, and written with its
  approval named in five amendment records. Nothing was chosen locally.

**Habitability recovered, which is the finding that the amendment was for.** Meals are 326–389 against
205–304 at the shipped candidate; §7's whole contact-driven band sat at 131–227. Against `individual` at
matched seeds — 400, 368, 378, 379, 417 — the `social` source now takes 92%, 101%, 89%, 86% and 93% of the
meals, so it forages within a seventh of the source it delegates to and exceeds it on one seed. The starvation
mechanism §3 identified — a hungry Mokiterion engaging instead of walking four squares to food — is gone.

`individual`'s range is recorded here as **368–417**. §7 and the framing of §8 gave it as 378–417, which was a
misread low end and not a movement: the 90-cell byte-identity below proves `individual` did not change. The
figure is corrected rather than overwritten because it appeared in a table the product owner selected against.

**The amendment moved nothing outside `social`, and that is measured rather than argued.** All 90 cells of the
three-source matrix — `baseline`, `reference` and `individual` × five declared seeds × densities 0.15, 0.75
and 1.50 × trace off and on — are **byte-identical** between `77f3b25` and the implemented candidate, compared
by digest over full output with identical exit codes. This is the check that the split of
`tolerant_movement_choice` was designed to pass: hoisting case 3 alone moves no draw, so every pre-existing
run reproduces exactly.

**The suite went from 246 / 3 / 0 to 248 / 1 / 0 over the same 249 tests.** Nothing was added, removed,
renamed or `#[ignore]`d. The two `REQ-MOK-049` failures pass, and the two failing tests that the amendment
itself broke were fixed — neither by relaxing an assertion. Both had encoded a claim the amendment falsified:

- `a_threat_composes_with_rule_12_in_turn_order_and_outlasts_its_tick` asserted the threatener's `fear`
  against a hard-coded `30` — a copy of the old constant — and asserted that its target threatened back. At a
  gate of `95` neither can hold: every threatener is within `FEAR_DECREASE` of `ATTRIBUTE_MAX`, so its own
  rule 12 write saturates, and a target at `THREAT_FEAR_INCREASE` is calm enough to answer with `attack`. The
  composition claim the test exists to prove survives **on the target**, unsaturated, at
  `THREAT_FEAR_INCREASE + 10`; the threatener's line became an assertion of saturation with a second
  assertion that saturation is forced by the constants rather than coincidental, and the target's answer is
  now asserted as the `attack` it is. `SPEC-MOK-001` rule 26 records the same thing as the cost of `95`.
- `the_acting_order_is_one_ascending_pass_per_tick_under_the_social_source` computed the width of a tick as
  `12 - dead.len()` snapshotted at the tick boundary, on a comment asserting that "a Mokiterion that dies
  *during* a tick has already acted in it". That is true of rule 13's decay and **false of rule 22's damage**.
  `SPEC-MOK-001` rule 13 says so outright: "a Mokiterion may die at a point in the tick where it has not yet
  acted, and it then receives no opportunity that tick or ever." The engine was right and the test's
  measurement point was wrong, so the width now narrows within the tick on exactly the deaths whose record
  precedes their holder's own trace line. It reads position rather than cause, which is what keeps it from
  being satisfiable by counting deaths — the two death paths share one event by that rule's design. **This
  test passed at `7c4aef3` because combat killed nobody there**; the case it got wrong was unreachable until
  lethality existed.

## 11. Still open: the identifier-monotonicity band, escalated as stop condition 6's first half

`tests/viability.rs :: no_identifier_series_is_monotone_in_identifier_or_correlated_beyond_the_band` fails at
the implemented candidate, and this time on its own terms. Over the five declared seeds, 1,000 ticks, default
density, `social` — 68 combat events:

| Series | M01 … M12 | Total | Spearman vs identifier | Band ±0.5 |
|---|---|---:|---:|---|
| survivals | 5, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4 | 48 | −0.414 | within |
| attacks applied | 3, 3, 6, 8, 0, 4, 7, 3, 5, 9, 7, 13 | 68 | **+0.586** | **outside** |
| attacks suffered | 3, 0, 7, 5, 0, 8, 5, 8, 7, 9, 7, 9 | 68 | **+0.731** | **outside** |

No series is monotone non-increasing, so the gross-advantage half of the oracle passes. The identifier-exchange
test — stop condition 6's *second* half, the one that would mean resolution reads an identifier and is a
defect — still passes. **This is the finding half, and `WO-MOK-012` reserves it: "the first is a finding about
turn order and is the owner's to weigh."**

### It is not a small-sample artifact, and the whole-population series understate it

The obvious reading is that 68 events across twelve identifiers cannot resolve a ±0.5 rank correlation — which
is what §1 said of the same row at `7c4aef3`, correctly, when there were twelve strikes. It was measured
rather than assumed. A diagnostic sweep of **1,000 seeds** at 1,000 ticks and the default density under
`social` gives 10,404 combat events, and the correlation does not dissolve:

| Series | vs identifier, M01…M12 | within territory A, M01…M06 | within territory B, M07…M12 | pooled by turn position in own territory |
|---|---:|---:|---:|---:|
| survivals | +0.084 | +0.486 | +0.829 | **+0.986** |
| attacks applied | **+0.601** | +0.943 | **+1.000** | **+1.000** |
| attacks suffered | −0.357 | −0.657 | −0.943 | **−0.943** |
| net strikes (applied − suffered) | +0.566 | **+1.000** | **+1.000** | **+1.000** |

**The sweep is diagnostic and is not a proposed remedy.** `VER-MOK-012`'s band text forbids the obvious use of
it in advance — "the residual is recorded below rather than reduced by widening the seed set, because a seed
set chosen to make this row pass would no longer be comparable with `REQ-MOK-014`'s and `REQ-MOK-034`'s" — and
the obligation stays on the declared five. The sweep exists to answer one question: whether the declared five
found something or stumbled on noise.

They found something. The reason the identifier series look weak is that the oracle's covariate is wrong.
Mokiterions are placed six to a territory by identifier — `M01`–`M06` in A, `M07`–`M12` in B — and they meet,
overwhelmingly, within their own. So the series against identifier 1…12 is a sawtooth that resets at `M07`,
and the reset masks a within-territory effect that is **perfectly ordered**. Pooled by turn position within
its own territory, across 1,000 seeds:

| Turn position in territory | 1st | 2nd | 3rd | 4th | 5th | 6th |
|---|---:|---:|---:|---:|---:|---:|
| attacks applied | 1,255 | 1,384 | 1,411 | 1,578 | 1,706 | 1,860 |
| attacks suffered | 1,653 | 1,651 | 1,574 | 1,633 | 1,413 | 1,270 |
| survivals, of 12,000 | 1,518 | 1,521 | 1,544 | 1,544 | 1,548 | 1,613 |

The last-acting Mokiterion in a territory strikes **48% more often** than the first-acting one and is struck
**23% less**. Net strikes rank `+1.000` against turn position in territory A and `+1.000` in territory B,
independently — twelve identifiers, two territories, and not one inversion. Survival follows in the same
direction and is ordered too, at `+0.986` pooled, but it is **small**: 75.9% against 80.7%, a spread of 4.8
percentage points.

### The mechanism is the one the specification already states

`SPEC-MOK-001` records it under rule 25, before any of this was measured: "A Mokiterion struck by a
*lower*-identified one reaches its own opportunity later in the same tick and answers with zero ticks of
latency; one struck by a *higher*-identified one has already acted and answers on the next tick. This is …
one of the two inputs to the identifier advantage `VER-MOK-012` bounds." A later actor answers within the
tick, and `fight` resolves as a strike, so the same asymmetry that lets it answer sooner is what raises its
applied count. Nothing reads an identifier; the advantage is turn order, exactly as `INT-MOK-009` anticipated.

### What this leaves the owner, and what has deliberately not been done

The band has **not** been widened, the seed set has **not** been changed, and the test has **not** been
renamed or `#[ignore]`d. Stop condition 8 forbids all four independently of this one, and the sweep is why
widening would be wrong on the merits rather than merely out of scope: on the covariate that carries the
effect the correlation is `+1.000`, so no band on identifier 1…12 that this world passes would mean the
advantage is absent. It would mean the oracle cannot see it.

Three things are true at once, and the decision among them is the product owner's and the assurance owner's,
not this record's:

1. **The engine has no defect.** Resolution is identifier-blind, the exchange test proves it, and the
   asymmetry is a stated consequence of rule 2's ascending pass that Phase 3 chose deliberately.
2. **The oracle as written fails, and it fails for the right reason.** It was built as a coarse tripwire for a
   gross advantage and it has caught a real, ordered, fine one. Passing it would now require either changing
   the world or changing the oracle.
3. **The advantage's survival magnitude is 4.8 percentage points** — measured, and the number the decision
   should be taken against, because survival is what an advantage means. `VER-MOK-012`'s manual assessment 11
   already reserves "the monotonicity band's adequacy" to the assurance owner, and this is the evidence that
   assessment was waiting for.

`WO-MOK-012` cannot reach `implemented` while this row fails, and `REQ-MOK-051` blocks it independently under
the deferral of 2026-08-20 in any case.
- No aggregate over the population was introduced to suppress combat when the population falls, which is the
  cheap route to `REQ-MOK-049` that `REQ-MOK-050` exists to forbid. `post/reads.md` enumerates every reader
  of a set in the engine and shows that none is reachable from a rule, a source or a validation.
- **`WO-MOK-012` has not been transitioned to `implemented`**, and will not be while an approved obligation
  fails. Nothing has been pushed.
