# WO-MOK-007 escalation: the survivor floor of eight is missed on three declared seeds

**Stop condition 6.** *"The survivor floor of eight is missed on any declared seed under the new source at the default
density. The correction is either an amended `SPEC-MOK-001` constant approved by the technical owner or an amended
floor approved by the product owner on the measured evidence. It is never an implementation adjustment, and it is
never a change to the reference source."*

`REQ-MOK-034` states the same thing from the other side: a floor miss *"does not permit an implementation adjustment,
and it does not permit changing the reference source."*

Work is stopped at this condition. Nothing below proposes an implementation adjustment, and the reference source is
untouched and byte-identically verified as such.

## What was measured

`--policy individual --density 0.75 --ticks 1000`, the configuration `REQ-MOK-034` states the obligation over, on the
declared verification seed set.

| Seed | Baseline | Reference | **Trait-aware** | Floor | Result |
|---|---:|---:|---:|---:|---|
| 0 | 0 (extinct t119) | 8 | **7** | 8 | **MISS by 1** |
| 1 | 0 (extinct t119) | 11 | **8** | 8 | met, at zero margin |
| 42 | 0 (extinct t142) | 8 | **4** | 8 | **MISS by 4** |
| 123 | 0 (extinct t168) | 9 | **7** | 8 | **MISS by 1** |
| 777 | 0 (extinct t134) | 11 | **9** | 8 | met |

Three of five declared seeds miss. No run reached extinction; every run reached the tick limit and reported non-zero
food consumption, so this is a survivor-count miss and not a collapse.

## The miss is not marginal, and this matters to the choice

Five seeds cannot distinguish "just below the line" from "centred below the line". Extending the same measurement to
fifty consecutive seeds settles it. `REQ-MOK-034`'s own *Open decisions* section warns against exactly this — the
sibling requirement had two floors approved from too little data and both failed — so the distribution was measured
before any option was put forward.

| Source | n | min | mean | max | seeds below 8 |
|---|---:|---:|---:|---:|---|
| Reference | 50 | 5 | 9.72 | 12 | 3 (**6%**) |
| Trait-aware | 50 | 4 | **7.40** | 11 | 23 (**46%**) |

**The trait-aware source's mean survivor count is below the floor.** The declared five are not unlucky; they are
representative, and two of them (8 and 9) are the lucky ones. A floor of eight is not missed by a hair on three seeds
— it is missed on roughly half of all worlds.

For context on the control: the reference source misses the same floor on 6% of undeclared seeds. `REQ-MOK-014` binds
it only on the declared set, so that is not a violation, but it does establish the scale against which 46% should be
read.

## Why survivors fall: the trait's upper half is a dominated strategy

The dead hold a higher mean tolerance than the living on four of the five declared seeds.

| Seed | Survivors | Dead: id, tolerance, tick of death | Mean tolerance, dead | Mean tolerance, living |
|---|---:|---|---:|---:|
| 0 | 7 | M10:28@t225, M11:37@t281, M05:0@t394, M08:83@t538, M04:98@t811 | 49.2 | 43.4 |
| 1 | 8 | M09:42@t250, M11:73@t253, M08:75@t310, M07:39@t965 | 57.2 | 39.4 |
| 42 | 4 | M11:76@t229, M12:40@t275, M05:25@t626, M01:97@t650, M04:53@t731, M03:12@t734, M10:97@t750, M08:74@t828 | 59.2 | 25.0 |
| 123 | 7 | M12:1@t324, M07:96@t468, M02:4@t609, M03:16@t639, M06:4@t818 | 24.2 | 53.7 |
| 777 | 9 | M12:59@t392, M02:58@t398, M11:74@t893 | 63.7 | 31.0 |

The mechanism is arithmetic, not competitive. A Mokiterion at satiety `80` that eats a high-class resource gains `20`
points of satiety and destroys the other `30`: the resource is removed from a world with conditional, capped
regeneration, and the eater is back at satiety `100` — exactly where a resource of any class would have put it. It
has converted a 50-point resource into 20 points of value and needs food again no later than before.

A high tolerance therefore buys its holder nothing and costs the world a resource. **The upper half of the trait range
is not a different strategy; it is a strictly worse one.** That is what makes the next section a question about the
range rather than about the floor.

This also confirms the effect `REQ-MOK-034` deferred rather than obliged: high-class accumulation is reduced. At seed 0,
tick 1,000, territory A holds 45 high-class resources of 61 under the reference source and 22 under the trait-aware
source.

## Long-horizon behaviour is unaffected, and if anything better

`--ticks 10000`, default density, so the deferred long-horizon question in `REQ-MOK-034` is answered on this evidence
too. No obligation is stated on it in either direction.

| Seed | Reference | Trait-aware |
|---|---|---|
| 0 | extinct t5423 | extinct t6395 |
| 1 | extinct t8273 | extinct t7291 |
| 42 | tick limit, 1 survivor | tick limit, 1 survivor |
| 123 | extinct t9154 | **tick limit, 1 survivor** |
| 777 | extinct t9598 | **tick limit, 1 survivor** |

The reference source reaches tick 10,000 on one of five seeds; the trait-aware source on four. The floor miss is a
property of the 1,000-tick window, not of long-run stability.

## Everything else in the verification contract passes

Reported here so the escalation is read against a complete picture rather than as a single failure.

- **Additivity (oracle 1), the central failure mode, stop condition 4: PASS.** Eleven captures — both frozen sources
  x five declared seeds at 1,000 ticks, plus a 20-tick traced excerpt — compared against commit
  `60fda9faffbd452752a34efa356f16cc6ad1d3ff`. All 11 byte-identical across roughly 87,000 lines after projecting out
  the two added fields. No difference anywhere.
- **Entropy neutrality (oracle 2), stop condition 5: PASS**, and demonstrated able to fail — see
  `negative-control/oracle-2.txt`. Recorded initialization draw counts hold at every seed and swept density; at
  `0.15%` the count is exactly `2 x (24 + 12) = 72` with no rejection, so there is no slack in which a thirteenth
  draw could hide.
- **Lower-bound equivalence (oracle 3), stop condition 7: PASS** over the full enumerated set of 2,808 situations,
  comparing both the proposal and the post-decision stream state.
- **Oscillation, stop condition 8: PASS, with margin.** The measurement method is `SPEC-MOK-001` rule 5's own, and it
  reproduces the recorded control figures exactly (reference seed 42 = 10.6%; pooled numerator 5,888; baseline pooled
  numerator 835), which is what licenses the new figures.

  | Source | s0 | s1 | s42 | s123 | s777 | Pooled |
  |---|---:|---:|---:|---:|---:|---:|
  | Reference (rule 5's recorded residual: 10.6%) | 10.0% | 10.1% | 10.6% | 11.4% | 12.1% | 10.8% |
  | **Trait-aware** | **8.7%** | **9.2%** | **9.0%** | **8.6%** | **10.4%** | **9.2%** |
  | Baseline, unbiased walk | 11.6% | 11.7% | 12.0% | 10.4% | 12.1% | 11.6% |

  The trait-aware source is below rule 5's residual and below the walk rate on every seed. Extending the tolerant test
  to seeking did not reintroduce the two-cell oscillation.
- **Roster narrowing, stop condition 9: PASS.** `bar_width(45) == 2`, which is the `(45 - 35) / 4 = 2` the work order
  anticipated, and 2 is at least one cell at every declared viewport.
- Stop conditions 10, 11, 12 and 14: not triggered. No constant was adjusted, no item widened to `pub`, no `allow`, no
  relaxed assertion, no ignored test, no `ARCH-MOK-001` amendment, no scope growth, no new dependency.

## The permitted corrections, measured

`REQ-MOK-034` permits exactly two, and names what each requires.

### Option A — amend `SPEC-MOK-001`'s trait range (technical owner)

The range and the tolerance test are specified, not implementation choices, so this is an amendment and not an
adjustment. Upper bounds were swept, each on the declared five and then on fifty consecutive seeds. **The five-seed
result alone is misleading and is shown here to make that visible.**

| Range | Declared five | 5-seed verdict | n=50 min | n=50 mean | n=50 below 8 |
|---|---|---|---:|---:|---:|
| `0..=100`, as specified | 7, 8, 4, 7, 9 | misses | 4 | 7.40 | 23 (46%) |
| `0..=75` | 10, 9, 8, 10, 5 | misses | — | — | — |
| `0..=60` | 11, 9, 11, 9, 10 | holds | 4 | 9.60 | 6 (12%) |
| `0..=50` | 8, 11, 9, 8, 7 | misses | 7 | 9.88 | 3 (6%) |
| **`0..=40`** | **11, 9, 9, 10, 12** | **holds** | **7** | **9.94** | **2 (4%)** |
| `0..=30` | 10, 12, 10, 9, 9 | holds | 6 | 10.32 | 2 (4%) |
| `0..=25` | 7, 11, 10, 7, 9 | misses | — | — | — |
| `0..=20` | 12, 11, 9, 7, 11 | misses | — | — | — |
| `0..=10` | 10, 12, 8, 10, 11 | holds | 6 | 10.18 | 2 (4%) |

Two things to read off this table. First, the declared-five verdict is **not monotonic** in the bound — `0..=50`
misses while `0..=60` and `0..=40` hold, `0..=25` and `0..=20` miss while `0..=30` and `0..=10` hold. Any bound chosen
because it holds on five seeds is chosen by luck. Second, the fifty-seed columns *are* orderly, and they separate
`0..=60` (12% below floor, worse than the reference source's 6%) from `0..=40` (4% below floor, better than the
reference source's 6%).

`0..=40` is the narrowest claim that is supported rather than fitted: it holds on all five declared seeds, and it holds
on the distribution behind them.

It also satisfies `REQ-MOK-034`'s anti-collapse constraint, which forbids narrowing the range until individuality
disappears. At `T = 40` the tolerant test still admits a materially wider satiety window than the reference source does
at `T = 0`, in every resource class:

| Class | Restores | Reference source (`T = 0`) eats up to satiety | `T = 40` eats up to satiety | Divergence window |
|---|---:|---:|---:|---:|
| High | 50 | 50 | 70 | 20 points |
| Medium | 30 | 70 | 82 | 12 points |
| Low | 15 | 85 | 91 | 6 points |

So the divergence evidence `VER-MOK-007` requires remains producible across all three classes. Because the upper half
of the range is a dominated strategy rather than a distinct one, truncating it removes bad strategies and not
individuality. This bound has **not** yet been measured for divergence instances or re-verified for the other oracles;
choosing it means re-running the whole matrix on the amended constant.

### Option B — amend `REQ-MOK-034`'s floor (product owner)

To hold on the declared five the floor would have to fall to **4**. Over fifty seeds the minimum is also 4, so 4 is
both the declared worst case and the observed worst case — but it is the extreme tail of a distribution centred on
7.40, and setting a floor at a tail is how the two superseded floors in `REQ-MOK-014`'s amendment record were set.

A floor of 4 of 12 contradicts what `REQ-MOK-034` says it exists to assert — *"individuality must not cost
habitability"*, and *"the world with twelve distinct Mokiterions must be at least as survivable as the world with
twelve identical ones"*. That is a legitimate product decision to reverse; `REQ-MOK-034`'s *Open decisions* section
already records this alternative as the one the floor of eight was chosen against, so reversing it is a recorded and
anticipated move rather than an improvised one. But it is a reversal, and it should be taken knowingly.

### Option C — amend the tolerance test's form (technical owner)

`REQ-MOK-034` names *"the tolerance range and the tolerance test"* as specified, so the test's form is also amendable.
Nothing here is measured, and a new form would need the whole matrix re-run. Recorded for completeness because the
mechanism above points at the test rather than at the trait: the test currently lets tolerance scale the permitted
waste as a fraction of the resource, which grants the most latitude exactly where waste is most expensive — the
high-class resources. Not recommended without a reason to prefer it to Option A, which is simpler and measured.

## Recommendation

**Option A at `0..=40`.**

It is the only option supported by a distribution rather than by five points. It satisfies `REQ-MOK-034` as written —
both the floor and the constraint against buying the floor by erasing individuality — so it does not ask the product
owner to give up the requirement's stated purpose. It leaves the frozen sources frozen and the additivity result
intact, since the trait derivation is entropy-neutral and the bound is an argument to it. And it removes a region of
the trait space that the mechanism above shows to be strictly self-harmful, which is a defensible thing for a
specification to exclude on evidence.

The cost is honest and should be stated: re-running the full verification matrix on the amended constant, and a
narrower trait range than the `0..=100` chosen on 2026-08-19 before anything was measured.

If the preference is instead to keep `0..=100` and let the population pay for it, that is Option B, and the floor
should be set from the distribution rather than from the declared five.

## Status of the work at the point of stopping

Complete and passing: the engine implementation, the observer implementation, all fourteen new internal-tier tests,
the additivity comparison, the entropy-neutrality check and its negative control, the lower-bound enumeration, the
oscillation measurement, and the long-horizon runs.

**`cargo test --workspace` does not pass, and the single failure is this escalation.**
`the_trait_aware_source_sustains_the_population_at_every_declared_density` in
`mokiterions-core/tests/viability.rs` asserts `REQ-MOK-034`'s floor of eight and fails on seed 0 with the message
*"seed 0 at density 0.75% left only 7 survivors under the trait-aware source, below the stated floor of 8"*. The test
is left failing deliberately: it states the approved requirement, and weakening its assertion to make the suite green
is the implementation adjustment `REQ-MOK-034` forbids. It will pass under Option A without edit, and under Option B
only after the amended floor is approved and transcribed into it.

Also outstanding, and independent of this decision: the observer's four-gauge cell-position assertions at the declared
viewports, the divergence-instance record, the test census, and the tooling runs.

Blocked on this decision: the viability test's asserted floor, the completion summary, and `VREC-MOK-007`. Nothing is
committed pending the decision, and no artifact has been amended in anticipation of any option.
