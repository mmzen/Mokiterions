# `WO-MOK-012` escalation: `REQ-MOK-049`'s lethality bound is missed on every declared seed, and `surrender` never applies

| Field | Value |
|---|---|
| Candidate | `7c4aef3967406c05d80da963695898b77f5329e9` on `feature/phase-3-definition` |
| Suite | 246 passing, **3 failing**, 0 ignored |
| Stop conditions triggered | **5**, **6**, **7**, and **11** |
| Date | 2026-08-20 |

**Work is stopped.** Nothing below has been implemented. `REQ-MOK-049`'s bounds are not adjusted to fit the
measurement, no damage, forfeit, survival or resource constant is tuned, `baseline` is byte-identical on all
30 cells, and the `reference` source is untouched. §6 puts the choice to the product owner with each option
measured, because `REQ-MOK-049` states that "the measurement is reported and the owner decides".

## 1. The three failing cases, and the four obligations behind them

| Case | Obligation | State |
|---|---|---|
| `tests/viability.rs :: the_social_source_keeps_the_world_habitable_and_combat_lethal` | `REQ-MOK-049`, lethality | **0 combat deaths on all five declared seeds** |
| `tests/decisions.rs :: every_targeted_verb_applies_somewhere_in_the_declared_matrix` | `REQ-MOK-043`, `VER-MOK-012` oracle 4 | **`surrender` never applies**, in any of the 5,000 opportunity-ticks measured |
| `tests/viability.rs :: no_identifier_series_is_monotone_in_identifier_or_correlated_beyond_the_band` | `VER-MOK-012` oracle 5 | fails **downstream**: twelve strikes across five seeds cannot populate three per-identifier series |
| — | `REQ-MOK-051`, composition ceiling | **unimplemented**, 14 of 15 cells fail; deferred by product owner decision of 2026-08-20 |

`REQ-MOK-049`'s survivor floor of five is **met** at this candidate — 6, 4, 8, 4, 5 — but that is not a pass:
the requirement states that "both bounds hold simultaneously on each seed" and that "a matrix that trades one
against the other does not satisfy this requirement". This candidate is a matrix that trades one against the
other, in the peaceful direction.

Stop condition 6's first half is the monotonicity band, "a finding about turn order and the owner's to
weigh". It is **not** that here, and it should not be escalated as one: it fails for want of data. The
identifier-exchange test — the same condition's second half, the one that would mean resolution reads an
identifier and is a defect — **passes**.

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
which is why it was put with that caveat, and the decision now needs to be remade against the numbers.

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

## 6. What the product owner is being asked

1. **Whether to withdraw the "rescope the fear rise" selection**, on §4's measurement. It recovers lethality
   and loses habitability, and it costs a Phase 2 amendment to do so.
2. **Whether to amend `REQ-MOK-048`'s branch order** so that rule 19's case 3 precedes branches 3 and 4 —
   change A. This is also, independently, a correction of the inconsistency §5 documents between
   `REQ-MOK-048`'s branch 2 and `REQ-MOK-049`'s account of it, and it is worth deciding on that ground even
   if the threshold question is answered differently.
3. **What `ENGAGEMENT_FEAR_THRESHOLD` becomes** — change B. `95` and `100` both satisfy the requirement.
   `100` is self-describing, since it equals `ATTRIBUTE_MAX`: only a Mokiterion at maximum `fear` declines to
   engage. `95` is an arbitrary number, and holds four more survivors of margin above the floor at its worst
   seed (9) than `100` does (6). The band is narrow either way — `90` fails on one seed — and that narrowness
   is itself a finding: this requirement is met only where the gate is "engage unless saturated", and a value
   chosen for roundness will not do.
4. **Whether the floor of five stands**, now that the first curve on which any lethal matrix exists has been
   measured. `REQ-MOK-049` reserves this: "the evidence is what the product owner ratifies the floor on".
5. **`REQ-MOK-051`.** Unimplemented, deferred on 2026-08-20 to a second amendment. It is an approved
   requirement inside this work order's scope and `WO-MOK-012` cannot reach `implemented` while it stands,
   whichever way 1 to 4 are decided.

## 7. What was not done

- No implementation adjustment was made to reach any bound. The measurements in §4 and §5 were taken with a
  throwaway scaffold, behind environment switches, which was reverted before this record was written; the
  working tree is clean at `7c4aef3` and the suite there is 246 / 3 / 0.
- No test assertion was relaxed, widened, removed or `#[ignore]`d. The three failing cases fail loudly, print
  their whole table, and name every seed. `post/test-census-reconciliation.md` accounts for the one rename
  name by name.
- No engine constant was tuned. `FEAR_INCREASE`, `FEAR_DECREASE`, `STRIKE_BASE_DAMAGE`,
  `STRIKE_CONDITION_DIVISOR`, `STRIKE_ENERGY_COST`, `THREAT_FEAR_INCREASE`, `SURRENDER_FEAR_THRESHOLD`,
  `RETREAT_FEAR_THRESHOLD` and `ENGAGEMENT_FEAR_THRESHOLD` all stand at their approved values.
- No aggregate over the population was introduced to suppress combat when the population falls, which is the
  cheap route to `REQ-MOK-049` that `REQ-MOK-050` exists to forbid. `post/reads.md` enumerates every reader
  of a set in the engine and shows that none is reachable from a rule, a source or a validation.
- **`WO-MOK-012` has not been transitioned to `implemented`**, and will not be while an approved obligation
  fails. Nothing has been pushed.
