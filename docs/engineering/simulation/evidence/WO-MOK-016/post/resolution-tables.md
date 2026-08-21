# Rules 22, 23 and 24 resolved: the tables, the engine's column, and what perturbing each term breaks

| Field | Value |
|---|---|
| Retention item | "the constructed-state resolution tables: inputs, the value computed from the specification on paper, and the value the engine produced, for damage, energy cost, threat increase and forfeit, including every boundary case" |
| Authority | `SPEC-MOK-001` rules 22, 23 and 24, as approved 2026-08-20 |
| Reader | `analysis/resolutions.py`, which does not import, parse or read `simulation.rs` |
| Exit code | **`0`** — 24 of 24 retained rows and 1,742 of 1,742 released resolutions agree with the tables |
| Raw output | `post/resolutions.txt`, 262 lines, retained whole; every table below is transcribed from it |
| Constructed-state source | three retained unit tests: `mokiterions-core/src/simulation.rs:5411`, `:5570`, `:5639` |
| Released capture | `post/social-manifest.txt`'s 30 cells at `59d61b915630fd55f04bcdbb346aa22cdbfdfff6`, re-verified at **30 of 30 digests** before being read |
| Baseline commit | `39662d13abd08e3410648d1c59ad38384f8ad2d2` |
| Controls | six in the reader, five in the engine; one term each, applied alone, every one reverted |
| Date | 2026-08-20 |

---

## 1. The clause asks for three columns, and they have three different sources

The retention item names them: **inputs**, **the value computed from the specification on paper**, and
**the value the engine produced**. They cannot come from one place, and most of the work in this file is
in keeping them apart.

**The paper column may not be the code's.** A table built by running the engine and writing down what it
said would agree with the engine by construction and would be a transcript, not a check. So the four
functions are transcribed from the rules and from nothing else, in `analysis/resolutions.py`, which never
opens `simulation.rs`:

    damage(e, h)       = 10 + (e + h) // 10                    rule 22
    cost(e)            = min(5, e)                             rule 22, saturating at zero
    increase(f)        = min(f + 30, 100) - f                  rule 23, saturating at the bound
    forfeit(s)         = s // 2                                rule 24
    transferred(s, r)  = min(forfeit(s), 100 - r)              rule 24, the excess destroyed
    discarded(s, r)    = forfeit(s) - transferred(s, r)        rule 24

**The tables are exhaustive rather than sampled.** Each function is of one or two attributes bounded in
`0..=100`, so the domain is enumerable — 10,201 pairs for damage, 101 values for the increase, 10,201 for
the forfeit and its destruction — and each table below is that domain collapsed onto its distinct outputs
with the interval of inputs producing each. This is why "including every boundary case" needs no
judgement here: a boundary is the first or last row of an interval, and an enumeration produces all of
them whether or not anyone thought of them.

**The engine column has two halves, and they are unequal.** Constructed state gives exact inputs and an
asserted output; whole runs give whatever the world reached.

| Half | What it is | Size |
|---|---|---:|
| constructed state | the input/output literals of the three retained unit tests, transcribed with their line numbers | 24 rows |
| released streams | every `attack_resolved`, `threat_resolved` and `surrender_resolved` line of the 30-cell `social` capture | 1,742 lines |

The constructed-state half is the tests' literals rather than a re-run, and that is sound for one stated
reason: **the suite passes at 250 of 250 at the committed state**, so the engine produced exactly those
literals. A row here that disagreed with the paper function would be a test asserting a wrong value,
which is worth finding and is the reason they are transcribed rather than described.

The 1,742 lines are **871 distinct resolutions taken twice**. The matrix runs every cell with
`--trace-actions` and without, and on all **15 of 15** trace pairs the sequence of resolution lines is
identical — same lines, same order, 871 on each side — which is measured here rather than assumed,
because a reader that double-counted would report every coverage figure twice over. Per run that is 156
strikes, 620 threats and 95 surrenders, which is what `post/runs.md` §3 and `post/branches.md` §§4 to 6
count and what `post/observer.md` §8 counts as emissions. All three agree; this file works on all thirty
cells because a resolution line does not depend on the trace flag.

---

## 2. The four magnitudes, the sentences that fix them, and where the engine keeps them

| Rule | The sentence, verbatim | Engine site | Held as |
|---|---|---|---|
| 22 | "Damage is `10 + (striker.energy + striker.health) / 10`", "the division truncating toward zero" | `simulation.rs:2621` | `STRIKE_BASE_DAMAGE` at `:23`, `STRIKE_CONDITION_DIVISOR` at `:27` |
| 22 | "The striker pays a flat `5` `energy`, saturating at zero" | `simulation.rs:2628` | `STRIKE_ENERGY_COST` at `:30` |
| 23 | "The target's `fear` rises by `30`, saturating at `ATTRIBUTE_MAX`" | `simulation.rs:2701` | `THREAT_FEAR_INCREASE` at `:35` |
| 24 | "forfeits `satiety / 2` of its own `satiety`, the division truncating toward zero" | `simulation.rs:2741` | **an unnamed `2`** |

**Three of the four magnitudes are named constants and rule 24's proportion is not.** Line 2741 reads
`let forfeit = subject_satiety_before / 2;`, and the retained test recomputes it the same way at
`:5678`. The reader gives it the name `FORFEIT_DIVISOR` for its own controls; the engine has no such
name. This is recorded because it is asymmetric, not because it is wrong — §11.2 measures that the
proportion is nonetheless pinned, by the test's literal transfer/discard pairs rather than by its
`forfeit` expression.

---

## 3. Rule 22's damage over its whole domain

Both terms bounded in `0..=100`, so 10,201 ordered pairs, collapsing onto 21 distinct values:

| damage | `energy + health` | pairs |
|---:|---|---:|
| 10 | `0..=9` | 55 |
| 11 | `10..=19` | 155 |
| 12 | `20..=29` | 255 |
| 13 | `30..=39` | 355 |
| 14 | `40..=49` | 455 |
| 15 | `50..=59` | 555 |
| 16 | `60..=69` | 655 |
| 17 | `70..=79` | 755 |
| 18 | `80..=89` | 855 |
| 19 | `90..=99` | 955 |
| 20 | `100..=109` | 965 |
| 21 | `110..=119` | 865 |
| 22 | `120..=129` | 765 |
| 23 | `130..=139` | 665 |
| 24 | `140..=149` | 565 |
| 25 | `150..=159` | 465 |
| 26 | `160..=169` | 365 |
| 27 | `170..=179` | 265 |
| 28 | `180..=189` | 165 |
| 29 | `190..=199` | 65 |
| 30 | `200` | 1 |
| | | **10,201 = 101 × 101** |

The pair count is a check on the enumeration and not decoration: it sums to the domain exactly, so no
input is in two rows and none is in none. Two of rule 22's own claims fall out as the first and last row
rather than needing a separate measurement — "**No resolution deals `0`**", and "the range is `10..=30`".

**The table is over the declared domain, not the reachable one.** A striker at `health` `0` is dead and
takes no opportunity, so the 10 pairs of the first row with `health` `0` cannot occur in a run. This is
why the retained test's smallest row is `(0, 1)` and not `(0, 0)`: `1 / 10` truncates to `0`, so damage
`10` is reached at the smallest *live* condition there is, and the minimum is exercised without asserting
anything about a dead striker.

---

## 4. Rule 22's energy cost over its whole domain

| `energy` before | paid | `energy` after |
|---|---:|---|
| `0` | 0 | `0` |
| `1` | 1 | `0` |
| `2` | 2 | `0` |
| `3` | 3 | `0` |
| `4` | 4 | `0` |
| `5..=100` | 5 | `0..=95` |

Six rows, of which five are the saturating case. "Saturating at zero" is reachable and is not a
formality: below `5` the striker pays what it has. §10 measures that no released run reaches any of those
five rows, and §7 that four retained rows do.

---

## 5. Rule 23's increase over its whole domain

| `fear` before | applied | `fear` after |
|---|---:|---|
| `0..=70` | 30 | `30..=100` |
| `71` | 29 | `100` |
| `72` | 28 | `100` |
| … | `100 - f` | `100` |
| `99` | 1 | `100` |
| `100` | 0 | `100` |

32 distinct rows: one flat interval of 71 values, then 30 descending single-value rows, then the fixed
point. `post/resolutions.txt` §3 holds all 32 enumerated. The boundary falls between `70`, the last
`fear` at which the constant applies whole, and `71`, the first at which it does not. The retained rows
take `69` and `70` on the whole side and `85` and `100` on the saturating side, so the saturating
*behaviour* is covered while the first input that saturates, `71`, has a paper column and no engine
column — §7 shows that `69` and `70` are also the only two threat rows a changed constant can contradict.

`REQ-MOK-055` requires the increase *applied* rather than the constant *attempted*, so the last row is
`increase:0` on an accepted action rather than a rejection. That is the row every released threat lands
on, and §10 says how many.

---

## 6. Rule 24's forfeit, and where a forfeit starts being destroyed

The forfeit collapses onto 51 rows, one per output, each covering two inputs:

| `satiety` before | forfeited | `satiety` after |
|---|---:|---|
| `0..=1` | 0 | `0..=1` |
| `2..=3` | 1 | `1..=2` |
| `4..=5` | 2 | `2..=3` |
| `2f..=2f+1` | `f` | `f..=f+1` |
| `98..=99` | 49 | `49..=50` |
| `100` | 50 | `50` |

The destruction has its own dimension — for each forfeit, the recipient `satiety` at which the whole of
it still fits, and the largest amount that can be destroyed:

| forfeit | first `satiety` giving it | whole while recipient ≤ | largest discard |
|---:|---:|---:|---:|
| 0 | 0 | 100 | 0 |
| 1 | 2 | 99 | 1 |
| `f` | `2f` | `100 - f` | `f` |
| 49 | 98 | 51 | 49 |
| 50 | 100 | 50 | 50 |

`post/resolutions.txt` §§4 and 5 hold both dimensions enumerated, 51 rows each. The one boundary rule 24
names in its own words is the first table's row `0..=1` — "**A surrender below `satiety` `2` transfers `0`
and still succeeds**" — and the retained rows take it twice, at `1` and at `0`. The second table's
extremes are the forfeit that can never be destroyed at all (`0`, whole for every recipient) and the
forfeit of `50`, whole while the recipient is at or below `50` and destroyed entire when the recipient is
already at `100`; the retained rows take both ends of that one, at `100/50` and `100/100`.

**One of rule 24's saturations is unreachable, and the rule says so.** "Half of what is held can never
exceed what is held, so the saturation at zero constrains a later amendment to the proportion rather than
any case reachable under this one." The enumeration confirms it over all 101 inputs:
`s - s // 2 >= s // 2 >= 0`, so the subject-side saturation never engages under this proportion. The
recipient-side saturation at `ATTRIBUTE_MAX` engages constantly — §10 counts 180 of 190.

---

## 7. The engine's constructed-state column, and which of its rows can fail

The `contradicted by` column is measured, not annotated. Each of the reader's six controls is applied
alone, the row recomputed, and the control listed if the row then disagrees with what the engine
produced. **A row contradicted by nothing agrees with the engine under all six perturbations at once**,
and therefore says nothing about any of those terms.

`strike_damage_is_the_strikers_own_condition_and_the_cost_is_flat`, `simulation.rs:5413-5421`:

| `energy` | `health` | paper | engine | contradicted by |
|---:|---:|---:|---:|---|
| 0 | 1 | 10 | 10 | `damage-base` |
| 0 | 9 | 10 | 10 | `damage-base` `damage-divisor` |
| 0 | 10 | 11 | 11 | `damage-base` |
| 5 | 5 | 11 | 11 | `damage-base` |
| 50 | 50 | 20 | 20 | `damage-base` `damage-divisor` |
| 0 | 100 | 20 | 20 | `damage-base` `damage-divisor` |
| 100 | 1 | 20 | 20 | `damage-base` `damage-divisor` |
| 99 | 100 | 29 | 29 | `damage-base` `damage-divisor` |
| 100 | 100 | 30 | 30 | `damage-base` `damage-divisor` |

`a_threat_moves_the_targets_fear_and_nothing_else`, `simulation.rs:5572-5576`:

| `fear` before | applied | paper | engine | contradicted by |
|---:|---:|---:|---:|---|
| 0 | 30 | 30 | 30 | `threat-constant` |
| 69 | 30 | 99 | 99 | `threat-constant` |
| 70 | 30 | 100 | 100 | `threat-constant` `saturation` |
| 85 | 15 | 100 | 100 | `saturation` |
| 100 | 0 | 100 | 100 | `saturation` |

`a_surrender_forfeits_half_its_own_satiety_and_discards_what_does_not_fit`, `simulation.rs:5642-5655`:

| `satiety` | recipient | paper t/d | engine t/d | contradicted by |
|---:|---:|---|---|---|
| 80 | 30 | 40/0 | 40/0 | `forfeit-divisor` |
| 20 | 50 | 10/0 | 10/0 | `forfeit-divisor` |
| 81 | 30 | 40/0 | 40/0 | `forfeit-divisor` |
| 100 | 50 | 50/0 | 50/0 | `forfeit-divisor` `saturation` |
| 100 | 90 | 10/40 | 10/40 | `forfeit-divisor` `saturation` |
| 100 | 100 | 0/50 | 0/50 | `forfeit-divisor` `saturation` |
| 3 | 50 | 1/0 | 1/0 | **—** |
| 2 | 50 | 1/0 | 1/0 | `forfeit-divisor` |
| 1 | 50 | 0/0 | 0/0 | **—** |
| 0 | 50 | 0/0 | 0/0 | **—** |

Three readings, none of them visible from the fact that all 24 rows agree.

**The `energy` cost column of the first table pins nothing.** No damage row is contradicted by
`strike-cost`, because the test does not assert the cost against a value: it derives it at `:5440` as
`energy.saturating_sub(STRIKE_ENERGY_COST)` and asserts the engine against that, including inside the
whole-line stream assertion at `:5474`. The row shapes are right and the oracle is circular in one
column. §11.2 measures what does catch a changed cost, and it is two other tests.

**One threat row is circular the same way.** `(0u8, THREAT_FEAR_INCREASE)` at `:5572` writes its
expectation *as* the constant, and the guard at `:5611-5612` — `if before <= ATTRIBUTE_MAX -
THREAT_FEAR_INCREASE { assert_eq!(increase, THREAT_FEAR_INCREASE) }` — compares the constant with itself.
The reader flags that row under `threat-constant` because the reader's paper value moves while the
transcribed engine value does not; the engine's own test does not fail there. That is exactly the
difference between the reader's 3 flagged rows and the engine control's 2 failing tests in §11.

**Three surrender rows pin no arithmetic, and are still the right rows to have.** `3/50`, `1/50` and
`0/50` survive all six perturbations, because halving `3` by `2` or by `3` gives `1` either way and
halving `1` or `0` gives `0` under anything. They are not there for the proportion: they are rule 24's
"a surrender below `satiety` `2` transfers `0` and still succeeds", and what they assert is
`result.accepted` on a transfer of nothing. The row that does pin the proportion at the bottom of the
range is `2/50`, where a divisor of `3` gives `0` and the test demands `1`.

---

## 8. The released capture: 1,742 resolutions, and what an `attack_resolved` line cannot say

| stream event | lines checked | disagreements |
|---|---:|---:|
| `attack_resolved` | 312 | **0** |
| `threat_resolved` | 1,240 | **0** |
| `surrender_resolved` | 190 | **0** |
| | **1,742** | **0** |

A threat and a surrender are fully determined by their own line, so both are recomputed exactly: the
increase against `min(30, 100 - before)`, the transition against `before + increase`, the forfeit against
`before // 2`, the transfer against `min(forfeit, 100 - recipient)`, the discard against the remainder,
and both parties' reported transitions against all of it.

**`attack_resolved` carries the striker's `energy` and not its `health`**, so the damage cannot be
recomputed from the line. Two weaker statements are checked instead: the target's `health` transition
must be the saturating subtraction of the damage reported, and the damage must imply a **non-empty**
striker-`health` interval under the formula. Inverting `10 + (e + h) / 10` at a known `e` gives an
interval of admissible `h`, and a wrong base or divisor empties it for some lines — which is why this is
a check and not a restatement of the line.

**This is the weaker of two checks on rule 22 in this packet, and the wider one.** `analysis/runs.py`
closes the damage *exactly*, at its line 371, because on a traced cell the striker's own `action_trace`
line carries the `health` the resolution used; that is one of the thirteen checks `post/branches.md` §1
reports at zero, and it covers the 156 strikes of the fifteen traced cells. This reader covers all 312
lines of all thirty cells and says less per line. Rules 23 and 24 are closed by **both** readers, written
independently of each other, and neither imports the engine.

One free cross-check falls out of the attack lines, and it is worth stating because it relates two
sections of two files:

| | |
|---|---:|
| attack lines where the damage exceeded the target's `health` | 34 |
| attack lines where the damage equalled it exactly | 8 |
| attack lines reporting `target_died:yes` | **42** |

The first two sum to the third, so no released strike killed without the arithmetic saying it would and
none failed to kill when it did. 42 lines are 21 distinct deaths, which is the 21 combat deaths
`post/branches.md` §4 counts.

---

## 9. Which saturations engage, and where each one is exercised

| Saturation | Reachable | Constructed state | Released runs |
|---|---|---|---|
| rule 22, target's `health` at zero | yes | yes — `a_strike_that_empties_health_kills_through_rule_13_and_no_other_path` | **yes**, 34 of 312 lines |
| rule 22, striker's `energy` at zero | yes | yes — 4 retained rows at `energy` `0` | **no**, 0 of 312 |
| rule 23, `fear` at `ATTRIBUTE_MAX` | yes | yes — retained rows `70`, `85`, `100` | **yes**, all 1,240 |
| rule 24, recipient's `satiety` at `ATTRIBUTE_MAX` | yes | yes — retained rows `100/90` and `100/100` | **yes**, 180 of 190 |
| rule 24, subject's `satiety` at zero | **no — unreachable by construction** | n/a | n/a |

The last row is the one the specification itself predicts, and §6 is where it is measured over all 101
inputs rather than argued.

---

## 10. What the runs reach, and what nothing reaches

**Damage, over the 312 released attack lines:**

| damage | lines | | damage | lines |
|---:|---:|---|---:|---:|
| 17 | 2 | | 24 | 12 |
| 18 | 4 | | 25 | 40 |
| 19 | 6 | | 26 | 50 |
| 20 | 14 | | 27 | 14 |
| 21 | 4 | | 28 | 54 |
| 22 | 12 | | 29 | 64 |
| 23 | 34 | | 30 | 2 |

14 of the 21 values in §3's table, and the whole lower half of the range is absent: nothing below `17`
occurs, because a Mokiterion that strikes has been fed and rested enough to have proposed an attack. The
retained rows cover `10`, `11`, `20`, `29` and `30`.

**Damage `12` through `16` is produced by no retained table row and by no released resolution.** Five of
the 21 values are measured by neither half of the engine column. A damage value is written two ways in
this source — `damage:<n>` inside a stream line and `damage: <n>` inside a record literal — and searching
the workspace for both finds exactly one occurrence in `12..=16`: the hand-built
`SufferedAttack { attacker: "M02", damage: 12 }` at `simulation.rs:6830`, inside
`answering_one_attack_closes_the_window_on_every_attack_in_it`. A constructed record is not a resolution,
and rule 25's window carries whatever damage it is handed. The interval is continuous in §3 and each of
its five rows is one integer wide, so this is a gap in the measurement rather than a gap that could hide
a defect in the function; it is recorded because the retention clause asks for every boundary case and
these five values have only a paper column.

**Energy cost:** the amount paid is `5` on all 312 lines. Striker `energy` before the cost spans `21` to
`100` across 42 distinct values, so it never enters `0..=4`, and §4's five saturating rows — the ones
where a striker pays less than `5` because it holds less — are reached by constructed state alone.

**Threat increase:** all 1,240 lines read `increase:0` at `target_fear:100->100`, a single distinct input
across the whole matrix. **This is not a new finding here** — `post/branches.md` §5 measures it on the
620 traced events, gives the mechanism as the composition of the `95` proposal gate with rule 12's step
sizes, and records it for the product owner. What this file adds is the count over all thirty cells and
the control in §11.3.

**Surrender:** subject `satiety` before the forfeit spans `22` to `94` across 25 distinct values;
recipient `satiety` before spans `26` to `99` across 17; the recipient ends at exactly `100` on 180 of
the 190 lines; 180 lines discard something. Two rows of §6 are reached by constructed state only — a
recipient *already* at `100`, where the whole forfeit is destroyed, and a subject below `satiety` `2`,
where the transfer is `0`. `post/branches.md` §6 records both absences on the per-run figures.

---

## 11. Controls

### 11.1 Six in the reader

A checker reporting zero disagreements is worth reading only if it can report one. Each control perturbs
one term of the reader's own arithmetic and leaves everything else alone; no build is mutated and no
capture is retaken. The expected result of every one is a non-zero exit.

| Control | Term | retained rows | released resolutions | failing checks |
|---|---|---:|---:|---:|
| `damage-base` | `10` → `11` | 9 of 24 | **0** of 1,742 | 0 |
| `damage-divisor` | `10` → `9` | 6 of 24 | **0** of 1,742 | 0 |
| `strike-cost` | `5` → `7` | **0** of 24 | 312 of 1,742 | 312 |
| `threat-constant` | `30` → `25` | 3 of 24 | **0** of 1,742 | 0 |
| `forfeit-divisor` | `2` → `3` | 7 of 24 | 190 of 1,742 | 382 |
| `saturation` | `100` → `99` | 6 of 24 | 1,426 of 1,742 | 1,606 |

The last two columns are different units and the reader now prints both, because one resolution can fail
several checks at once — a wrong forfeit moves the subject's transition, the transfer and the discard
together, which is why `forfeit-divisor` reports 382 checks over all 190 surrender lines. An earlier
draft of the reader reported the check count against the line total as though they were comparable; the
`in error` and `failing checks` rows of `post/resolutions.txt` §7 keep them apart.

All six exit `1`. **The split is the finding, not the total.** Rule 22's two magnitudes are contradicted
only by the retained rows and by nothing in 1,742 released lines — the consequence of §8's missing
`health` field, and the reason `analysis/runs.py`'s exact check on the traced cells matters. Rule 22's
flat cost is the mirror image: no retained row contradicts it and every attack line does. **The two
halves of the engine column cover disjoint parts of one rule**, so neither could be dropped for the
other.

`threat-constant` reports `0` released disagreements for a different reason than `damage-base` does: at
`fear` `100` the paper increase is `0` under a constant of `30` and under one of `25` alike. A stream of
saturated threats cannot distinguish one constant from another at all — §11.3 measures how far that goes.

### 11.2 Five in the engine

Each is one line of `mokiterions-core/src/simulation.rs`, applied alone, the whole workspace suite run
under `cargo test --locked --workspace --no-fail-fast`, and the line reverted. The worktree is clean and
the suite is at 250 of 250 at the committed state.

| Control | Line | Result | What failed |
|---|---|---|---|
| `STRIKE_BASE_DAMAGE` `10` → `11` | `:23` | 227 passed, **23 failed** | 21 of them the engine's own `debug_assert!` at `:2624`; 2 oracle assertions, at `:5442` and `:6442` |
| `STRIKE_BASE_DAMAGE` `10` → `9` | `:23` | 243 passed, **7 failed** | 7 oracle assertions in 7 tests, at `:5442`, `:5530`, `:5926`, `:6442`, `:6472`, `:6540`, `:6705`. **No `debug_assert!` fired** |
| `STRIKE_ENERGY_COST` `5` → `7` | `:30` | 248 passed, **2 failed** | `a_strike_that_empties_health_kills_through_rule_13_and_no_other_path` at `:5536`, `the_suffered_window_opens_in_resolution_order_and_closes_at_the_next_opportunity` at `:5981` |
| `THREAT_FEAR_INCREASE` `30` → `25` | `:35` | 248 passed, **2 failed** | `a_threat_moves_the_targets_fear_and_nothing_else` at `:5597`, `"increase:25"` against `"increase:30"`; `a_threat_composes_with_rule_12_in_turn_order_and_outlasts_its_tick` at `:6662`, `left: 20 right: 25` |
| forfeit `/ 2` → `/ 3` | `:2741` | 248 passed, **2 failed** | `a_surrender_forfeits_half_its_own_satiety_and_discards_what_does_not_fit`, `a_surrender_buys_no_immunity_and_is_paid_again` |

Four readings.

**The base constant is guarded asymmetrically, and only the downward control shows it.** Raising it to
`11` fails 23 tests, but 21 of those are one internal assertion —
`debug_assert!((STRIKE_BASE_DAMAGE..=30).contains(&damage))` at `:2624` — whose upper bound is the
literal `30` while its lower bound is written as the constant itself. Any full-condition striker then
computes `31` and panics, and constructed encounters start at `100/100`, so most combat tests trip it
before reaching their own assertions. Lowering the constant to `9` moves the assertion's own lower bound
down with it, nothing panics, and what remains is the true oracle coverage: **7 tests, 7 distinct
assertion sites**. A self-check that moves with the value it checks is worth knowing about; it is not a
defect, because `debug_assert!` is not an oracle and compiles out of a release build, but a reader
counting failures under the upward control would have credited the suite with 23.

**The flat cost is pinned by whole-line stream assertions and not by the table that names it.** §7
predicted this from the source and the control confirms it. Both tests that fail assert a whole stream
line as a string, and neither is about the cost: one compares the strike's complete output against
`…,striker_energy:100->95,target_died:yes` beside the `agent_died` line, the other a complete
`action_trace` line whose `energy:95` field is the striker's attribute after paying. Meanwhile
`strike_damage_is_the_strikers_own_condition_and_the_cost_is_flat` — whose own name claims the cost —
passes at a cost of `7`. **One assertion would close it**: a literal expected cost in the retained
table's rows, in place of the derived `energy.saturating_sub(STRIKE_ENERGY_COST)` at `:5440`. This is a
residual of a clause the work order satisfies as written, recorded here rather than fixed, because the
rows and their oracle are approved artifacts' business.

**Rule 23's constant is pinned by its duration, not only by its magnitude.** The second failing test is
about rule 12's composition: it threatens a target, moves the threatener away, and asserts the shedding
sequence `[25, 20, 15, 10, 5, 0]` — six quiet ticks at rule 12's `FEAR_DECREASE` of `5`, which is what
"`30` is three times rule 12's step" means when measured. Under a constant of `25` the first tick lands
on `20` instead of `25` and the sequence fails at `:6662`.

**Rule 24's unnamed proportion is pinned anyway.** The `/ 3` control fails both surrender tests despite
the test recomputing `subject_before / 2` at `:5678` with the same expression the engine uses, because
that expression is cross-checked against the row's literal `transferred + discarded` at `:5681`. The
literal pair is the oracle; the derived `forfeit` is a convenience.

### 11.3 The threat constant is inert in whole runs, and a re-capture measures it byte for byte

`post/branches.md` §5 records that no released threat has any numeric effect and gives the mechanism.
The stronger statement is available and was measured: **the full 30-cell `social` capture, retaken from a
build with `THREAT_FEAR_INCREASE = 25`, is byte-identical to `post/social-manifest.txt` on 30 of 30
cells.** Not equal in outcome — equal in every byte of 45.6 MB of stream, including the `threat_resolved`
lines themselves, which still read `increase:0,target_fear:100->100`.

So rule 23's own justification — "`30` is exactly the step that moves rule 26's own branches, which is
the mechanism by which a threat changes what another Mokiterion decides" — holds in constructed state,
where `:6662`'s six-tick shed and the `30` threshold of rule 26's answer branch both depend on it, and
moves nothing whatever in the declared matrix. **This is for the product owner and is not a defect
claim**: the rule is implemented correctly, `post/branches.md` §5 has already put the observation and its
two consequences to them, and this control quantifies the reach of it. The mutated build was deleted and
its capture removed after measurement; nothing in this packet depends on either.

---

## 12. What this file does not establish

**It is not a verification verdict.** `VER-MOK-016` is the contract, `VREC-MOK-012` will be the record,
and neither is written by the implementation.

**The paper column is a hand transcription of four sentences.** A transcription error the engine does
*not* share is loud: the reader reports a disagreement, which is exactly what the six controls of §11.1
demonstrate it can do. What no arrangement here can catch is a misreading the engine **shares** — if a
rule was read the same wrong way on both sides, both columns agree and both are wrong. The mitigation is
that §2 quotes the four sentences beside the formulas, so a reviewer can compare rule against formula in
one place. It is not that the transcription is checked.

**The released stream does not check rule 22's magnitude.** §11.1 measures that: a base of `11` or a
divisor of `9` passes all 1,742 lines. What checks it in whole runs is `analysis/runs.py`'s exact
reconstruction on the fifteen traced cells; what checks it in constructed state is nine retained rows and
the damage literals of six other tests — the seven that fail under the downward control of §11.2, less
the retained table itself. The `debug_assert!` at `:2624` is not part of that answer in either
direction: §11.2 measures that it dominates the failure count under an upward mutation, and it compiles
out of a release build.

**Damage `12` through `16` has a paper column and no engine column** — from either half. §10 states it.

**The reader's coverage figures describe the declared matrix and not the engine.** Five seeds, one
source, three densities, 1,000 ticks. "No release reaches a striker `energy` below `21`" is a statement
about those thirty cells; `long-horizon.md`'s 10,000-tick run is not yet written and may reach otherwise.

**Nothing here measures rule 25's window, rule 13's death path, rule 26's choice or the trace ordering.**
`post/branches.md`, `post/runs.md` and `post/observer.md` are those measurements. §8's death cross-check
is a consistency check between this reader and `post/branches.md` §4, not a measurement of rule 13.

**The five engine controls each mutated one line and each was reverted.** Their evidence is the recorded
failure counts and sites above; the mutated builds do not exist, and every figure elsewhere in this
packet comes from the unmutated source at `59d61b9` or the commits the manifests name.
