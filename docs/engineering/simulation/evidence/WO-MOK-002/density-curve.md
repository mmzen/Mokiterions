# Density curve — WO-MOK-002

Measured 2026-08-17 against the committed tree, after the third `SPEC-MOK-001` amendment
extended the rule 5 non-waste test from case 1 to case 3. Nothing here is a scratch build.

Survivors at 1,000 ticks under the reference source, on the declared seed set
`0, 1, 42, 123, 777`. Every figure is a direct execution of the shipped binary; none is
derived from the resource constants.

## The curve

| Density | Per territory | seed 0 | seed 1 | seed 42 | seed 123 | seed 777 | **Worst case** |
|---:|---:|---:|---:|---:|---:|---:|---:|
| `0.15%` | 12 | 0 | 0 | 0 | 0 | 0 | **0** |
| `0.25%` | 20 | 1 | 2 | 1 | 2 | 1 | **1** |
| `0.50%` | 40 | 10 | 8 | 6 | 8 | 7 | **6** |
| **`0.75%`, the default** | **61** | **8** | **11** | **8** | **9** | **11** | **8** |
| `1.00%` | 81 | 10 | 12 | 10 | 11 | 12 | **10** |
| `1.25%` | 102 | 10 | 12 | 12 | 11 | 11 | **10** |
| `1.50%` | 122 | 11 | 12 | 12 | 11 | 12 | **11** |
| `2.00%` | 163 | 12 | 12 | 12 | 12 | 12 | **12** |
| `3.00%` | 245 | 12 | 12 | 12 | 12 | 12 | **12** |

`REQ-MOK-014` declares one density, the default `0.75%`, with a floor of **8 of 12**. That row
is the obligation. Every other row is evidence and carries no claim.

## What the correction changed

The same sweep on the previous rule, where the non-waste test governed eating but not seeking,
is in the superseded table below. The correction moved the whole curve and moved the default
row furthest.

| Density | Previous worst case | Corrected worst case |
|---:|---:|---:|
| `0.50%` | 1 | 6 |
| `0.75%`, the default | 3 | **8** |
| `1.00%` | 8 | 10 |
| `1.50%` | 7 | 11 |

At the default density the worst case rose from three survivors to eight. That is the whole
reason the floor of eight — the original product intent — could be stated at the scarce default
rather than at a comfortable density.

The previous curve was also not monotonic in its worst case: `1.00%` measured 8 and `1.50%`
measured 7. The corrected curve is non-decreasing across all nine rows. That is an observation
about these nine points, not a property, and the next section is why it must not be read as one.

## Non-monotonicity is still real, per seed

Density is not a dial that adds food to a fixed world. The resource count determines how many
coordinate draws initialization performs, so a different density consumes the shared entropy
stream differently and produces a different world. Two measured counter-examples:

| Seed | Lower density | Higher density | Direction |
|---:|---|---|---|
| `0` | 10 survivors at `0.50%` | **8** at `0.75%` | falls as density rises |
| `777` | 12 survivors at `1.00%` | **11** at `1.25%` | falls as density rises |

The worst case happens to be non-decreasing here; individual seeds are not. This is why
`REQ-MOK-014` declares a floor at a point rather than a threshold, and why a new density row
would need its own full seed sweep rather than interpolation.

## Where the correction does nothing, and why

The non-waste test compares satiety plus restoration against the attribute maximum of `100`.
That makes the eatable-and-approachable window a property of the calorie class:

| Class | Satiety restored | Eatable and approachable at satiety of at most |
|---|---:|---:|
| Low | 15 | 85 |
| Medium | 30 | 70 |
| High | 50 | **50** |

For high class the condition is `satiety <= 50`, numerically identical to the fixed threshold of
`50` that the second amendment replaced. So for the richest third of the resource table nothing
about *when* it may be consumed changed at all. What changed is that a Mokiterion no longer
walks toward one while it cannot use it, which is what removed the oscillation.

The same table has a second consequence, visible in the trace and recorded in
`manual-observation.md`: a Mokiterion above satiety 85 finds nothing worth approaching in the
whole world and searches until satiety falls to 85. Foraging is now driven by need rather than
by proximity.

## Confirmation experiment

The claim that the correction is the cause, rather than something else in the tree, was tested
rather than assumed. Reverting case 3 to target the nearest resource without the fit test, in a
scratch build, reproduced the superseded worst cases of 3 at `0.75%` and 7 at `1.50%` exactly.
The scratch build was reverted and `src/simulation.rs` verified byte-identical afterwards.

## Oscillation, measured

The two-cell oscillation is counted as the fraction of traced agent-ticks in which an agent's
position equals its position two ticks earlier and differs from its position one tick earlier.
The random baseline source gives the noise floor that an unbiased cardinal walk produces on the
same world under the same measurement.

| Tree | Seed 42 | Pooled over five declared seeds |
|---|---:|---:|
| Previous rule, non-waste test on case 1 only | **35.7%** | — |
| Corrected rule, test on cases 1 and 3 | **10.6%** | **10.8%** (5,888 of 54,392) |
| Random baseline source, same measurement | 12.2% (174 of 1,427) | 11.6% (835 of 7,203) |

The corrected rule sits below the random-walk floor on both comparisons, so the systematic
oscillation is gone rather than reduced. It is below rather than equal because approach steps
are deliberate and directional, and a Mokiterion walking a straight line toward a resource
cannot return to the cell it occupied two ticks ago.

The baseline sample is small because the baseline source reaches extinction at tick 168 at this
density. That is a limitation of the comparison, not of the corrected rule: a larger baseline
sample could only move a figure that the corrected rule already sits under.

## High-class accumulation

The disclosed cost of the correction. Because a high-class resource is approachable only at
satiety of at most `50`, it is sought less often than low or medium and occupies capacity that
density fixes. Resource mix per territory at tick 1,000, at the default density, against a
balanced initial mix of roughly 20 / 20 / 21:

| Seed | Territory A low / medium / high | Territory B low / medium / high |
|---:|---|---|
| `0` | 7 / 9 / **45** | 7 / 16 / **32** |
| `1` | 4 / 10 / **43** | 7 / 7 / **25** |
| `42` | 6 / 17 / **38** | 8 / 7 / **26** |
| `123` | 8 / 11 / **42** | 4 / 12 / **44** |
| `777` | 13 / 20 / **28** | 8 / 13 / **18** |

Under the previous rule the mix stayed near balanced at tick 1,000: territory A measured
24 / 15 / 22 on seed `0` and 26 / 19 / 16 on seed `42`. Those two figures come from the earlier
measurement of the previous rule and are not reproducible against the current tree without
reverting the correction. The drift is caused by the correction and is expected from the class
table above.

It does not settle. Seed `123` at tick 3,000 measures 9 / 7 / **45** and 9 / 9 / **43**, and the
10,000-tick consequence is in `determinism-and-resilience.md`.

The product owner accepted this on 2026-08-17 and deferred it to Phase 2. `REQ-MOK-014` states a
floor at tick 1,000 and makes no claim about a steady state; `VER-MOK-002` records the effect as
residual uncertainty.
