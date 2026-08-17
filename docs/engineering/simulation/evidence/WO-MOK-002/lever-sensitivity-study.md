# Lever sensitivity study — WO-MOK-002

## Historical scope

This study was measured **before** the density amendment, against the superseded
`SPEC-MOK-001` that fixed the initial endowment at three per territory and the territory
capacity at `12`, equivalent to a density of `0.15%`. Those constants no longer exist; density
is now an input. The study is retained unchanged because it is the evidence that identified
capacity as the governing lever, and because the first escalation cites it.

Read it as the record of a diagnosis, not as a description of the current system. Current
measurements are in `calibration-record.md` and `density-curve.md`.

## Why this exists

`WO-MOK-002` forbids an unauthorized retune: a measured shortfall against `REQ-MOK-014` "requires an amended
specification and re-approval, never an unauthorized retune". It does not forbid measurement. This study
measures which specified constant actually governs the survivor count, so that the escalation to the technical
and product owners carries numbers instead of a guess.

**Nothing in this study is adopted.** Every variant was measured in a scratch build and reverted. The committed
implementation matches `SPEC-MOK-001` exactly: capacity `12`, regeneration interval `10`, regeneration yield
`2`, satiety decay `1`, energy decay `1`, perception radius `16`, hunger threshold `50`, sleep threshold `20`.

## Method

For each variant, survivors at 1,000 ticks under the reference source, on the five declared seeds
`0, 1, 42, 123, 777`. Baseline (as specified) is the first row of each block.

## Results

| Variant | seed 0 | seed 1 | seed 42 | seed 123 | seed 777 | Verdict |
|---|---:|---:|---:|---:|---:|---|
| **As specified** (radius 16, yield 2, capacity 12, threshold 50) | 0 | 0 | 0 | 0 | 0 | floor 8 not met |
| Perception radius 24 | 0 | 0 | 1 | 0 | 0 | not binding |
| Perception radius 32 | 3 | 1 | 2 | 0 | 0 | not binding |
| Perception radius 48 | 2 | 1 | 2 | 2 | 2 | not binding |
| Perception radius 64 | 1 | 1 | 2 | 0 | 0 | not binding |
| Perception radius 96 (near-global) | 1 | 1 | 2 | 0 | 0 | **not binding** |
| Committed-heading search, radius 16 | 0 | 0 | 0 | 0 | 0 | not binding |
| Committed-heading search, radius 24 | 1 | 1 | 0 | 0 | 0 | not binding |
| Committed-heading search, radius 32 | 1 | 0 | 2 | 2 | 0 | **not binding** |
| Regeneration yield 3 | 0 | 0 | 0 | 0 | 0 | no effect |
| Regeneration yield 4 | 0 | 0 | 0 | 0 | 0 | no effect |
| Regeneration yield 6 | 0 | 0 | 0 | 0 | 0 | no effect |
| Regeneration yield 8 | 0 | 0 | 0 | 0 | 0 | **no effect at all** |
| Non-wasteful eating instead of threshold 50 | 0 | 1 | 0 | 0 | 0 | not binding |
| Capacity 24 | 2 | 4 | 1 | 0 | 1 | improves |
| Capacity 48 | 5 | 4 | 3 | 3 | 3 | improves |
| Capacity 96 | 5 | 6 | 5 | 5 | 4 | **improves, then plateaus at ~5** |
| Capacity 48 + initial endowment 12 | 6 | 2 | 3 | 5 | 4 | improves |
| Capacity 48 + initial endowment 24 | 7 | 6 | 3 | 4 | 5 | **best measured; still short of 8** |
| Capacity 36 + initial endowment 18 | 1 | 2 | 2 | 4 | 6 | improves |
| Capacity 48 + initial 24 + non-wasteful eating | 6 | 6 | 2 | 4 | 4 | no additional gain |
| Capacity 48 + initial 12 + non-wasteful eating | 5 | 4 | 0 | 3 | 8 | no additional gain |

## What the numbers mean

**Perception radius is not the constraint.** This was the first hypothesis, because a traced agent random-walks
for its entire life without ever perceiving food. It is wrong: at radius 96 an agent sees essentially its whole
territory and still dies. Seeing food is not the problem.

**The search rule is not the constraint.** A random cardinal step gives diffusive coverage, roughly `√n` cells
in `n` ticks, so replacing it with a committed heading held for 24 ticks gives ballistic coverage. It changes
almost nothing. Covering ground faster is not the problem.

**Regeneration yield has literally no effect, and this is the key measurement.** Raising yield from `2` to `8`
— a fourfold supply increase — does not save a single Mokiterion on any seed. The reason is in the calibration
record: territories already sit *at capacity* for a third to two thirds of all regeneration opportunities. Once
a territory is full, extra yield is discarded on arrival. **`FOOD_CAPACITY_PER_TERRITORY` is the true ceiling on
supply; `REGENERATION_YIELD` only controls how fast that ceiling is reached.** The amendment raised the lever
that does nothing and left the lever that does everything untouched.

**Capacity is the dominant lever, because capacity sets density and density sets travel time.** A territory is
128 × 64 = 8,192 cells. At capacity 12 that is one resource per 683 cells, so the nearest resource is typically
about 13 cells away in Chebyshev terms and about 18 cells of Manhattan travel. An agent moves one cell per tick
and loses one satiety per tick, and a mean resource restores 31.67 satiety. So roughly 18 of every 31 available
ticks are spent walking, before any competition, any overshoot, and any time spent with no visible target.
Raising capacity shortens the walk. That is why it is the only lever that moves the number.

**Capacity alone plateaus at about five survivors**, and the reason is in the death ticks. Deaths cluster at
ticks 119–280, and a territory starts with three resources and gains two per ten ticks, so it needs roughly 45
ticks to reach capacity 12 and roughly 225 ticks to reach capacity 48. The population is dying during the ramp,
before the higher ceiling has any effect. Raising the initial endowment alongside capacity addresses that, and
that combination is the best measured — but at 3 to 7 survivors it still misses the floor of 8.

## Conclusion

No single constant that `SPEC-MOK-001` currently fixes reaches eight survivors on all five declared seeds, and
the best two-constant combination measured reaches seven on its best seed and three on its worst. The shortfall
is structural: it is the ratio of travel time to satiety drain, evaluated during the regeneration ramp. It is
not a supply shortfall and not a perception shortfall.

Choosing among the remaining options is a product and technical owner decision, not an implementation decision.
See `escalation.md`.
