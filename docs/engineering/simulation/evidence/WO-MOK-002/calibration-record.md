# Calibration record — WO-MOK-002

Measured 2026-08-17 against the committed tree, after the third `SPEC-MOK-001` amendment.
1,000 ticks under the reference source at the declared density of `0.75%`, which resolves to
61 resources per territory and 122 in the world.

`REQ-MOK-014` states a floor of **8 of 12**. **The floor is met on every declared seed.**

## Per-seed results at the declared density

| Seed | Survivors | Floor met | In A | In B | Consumed | Regenerated | Capacity skips | Depletion skips | Final food A | Final food B |
|---:|---:|:---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `0` | 8 | yes | 3 | 5 | 379 | 373 | 6 | 0 | 61 | 55 |
| `1` | 11 | yes | 4 | 7 | 410 | 384 | 5 | 0 | 57 | 39 |
| `42` | 8 | yes | 3 | 5 | 344 | 324 | 17 | 0 | 61 | 41 |
| `123` | 9 | yes | 4 | 5 | 324 | 323 | 21 | 0 | 61 | 60 |
| `777` | 11 | yes | 6 | 5 | 372 | 350 | 11 | 0 | 61 | 39 |

Every run terminated by tick limit, not by extinction. Consumption is non-zero on every seed,
between 324 and 410 resources per run, so survival is being earned by eating rather than by
never needing to eat.

**The measured worst case is exactly eight, on two of five seeds.** The floor has no margin, and
`VER-MOK-002` records that as a deliberate choice rather than an oversight.

## Depletion

No territory reached zero resources on any seed. `reason:depleted` was emitted **0 times** across
all five runs; every skipped regeneration was `reason:capacity`. Permanent local depletion
remains reachable by design and is exercised by
`food_regenerates_only_in_nonempty_nonfull_territories`, but it does not occur at this density
under this source.

Capacity skips rose relative to the previous rule (6 to 21 per run, against 2 to 21). Territories
now spend more regeneration opportunities full, which is consistent with the class accumulation
recorded below: capacity is increasingly held by resources agents do not seek.

## Death timing

| Seed | Deaths | Ticks at which they occurred |
|---:|---:|---|
| `0` | 4 | 604, 689, 733, 964 |
| `1` | 1 | 824 |
| `42` | 4 | 279, 383, 708, 993 |
| `123` | 3 | 244, 247, 274 |
| `777` | 1 | 474 |

The earliest death across all seeds is tick **244**. This is the third time this table has moved
outward, and the movement is the clearest single measure of what each amendment achieved:

| Tree | Earliest death | Latest death |
|---|---:|---:|
| Fixed endowment and capacity, density `0.15%` | 119 | not recorded |
| Density input, non-waste test on case 1 only | 212 | not recorded |
| Density input, non-waste test on cases 1 and 3 | **244** | 993 |

The two earlier figures are quoted from the measurements taken at the time and are not
reproducible against the current tree: the first needs constants this specification no longer
has, and the second needs the rule the third amendment replaced.

Deaths no longer cluster in an early window. On seed `0` the first death is at tick 604, more
than halfway through the run. The population is no longer dying during a supply ramp; it is
attriting under sustained scarcity, which is what `INT-MOK-002` asks the world to do.

## Cause of health loss

On seed `0`, every one of the **80** health-loss events was caused by satiety reaching zero.
Energy reached zero **0** times. The reference source sleeps below energy `20` in preference to
seeking or searching, so it never starves for want of rest. Starvation is the only mortality
channel in practice, which is why density and not the sleep threshold is the governing lever.

## Resource mix at termination

| Seed | A low / medium / high | B low / medium / high |
|---:|---|---|
| `0` | 7 / 9 / 45 | 7 / 16 / 32 |
| `1` | 4 / 10 / 43 | 7 / 7 / 25 |
| `42` | 6 / 17 / 38 | 8 / 7 / 26 |
| `123` | 8 / 11 / 42 | 4 / 12 / 44 |
| `777` | 13 / 20 / 28 | 8 / 13 / 18 |

Initialization cycles the three classes, so each territory begins at roughly 20 / 20 / 21. By
tick 1,000 high class dominates on every seed. This is the accumulation effect the product owner
accepted on 2026-08-17; its cause and its long-horizon consequence are in `density-curve.md` and
`determinism-and-resilience.md`, and it is recorded as residual uncertainty in `VER-MOK-002`.

## Measured carrying capacity

At the declared density of `0.75%`, the reference source sustains **8 to 11 of 12** Mokiterions
at tick 1,000, with a worst case of 8. That is the measured carrying capacity of the world at the
density it ships with, and later phases should treat it as the baseline a new decision source has
to beat rather than as a target to be reached by adding resources.

The 1,000-tick figure is not a steady state. Under sustained running the same configuration
declines: see the 10,000-tick result in `determinism-and-resilience.md`.
