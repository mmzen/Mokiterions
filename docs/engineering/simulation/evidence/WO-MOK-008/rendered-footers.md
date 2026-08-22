# Rendered footers, before and after, at every declared viewport

Real frames through the real `render::draw`, read out of the rendered character buffer's bottommost row.
The measurement is taken by a probe that lives **outside** this checkout and path-depends on
`mokiterions-tui`, so nothing in the repository depends on it and it cannot be mistaken for a test. The
same binary is built twice — against this tree and against the superseded renderer substituted behind the
same private signature — so the two columns are two runs of one program rather than one run and a
transcription. `render.rs` is restored afterwards and the revert verified by SHA-256.

## What is swept

Nine declared viewports; `33 × 21` is the declared one-below-floor case, refused at start-up, so no footer
exists there to report. Seven configurations, chosen because rule 8's order of loss is decided by the digit
counts of the entropy seed and the configured tick limit **together**: seed 0, seed 999, seed 1e12, seed
`u64::MAX`, seed 0 with `--ticks u64::MAX`, seed 9999 with `--ticks u64::MAX`, and both at `u64::MAX`. 63
rendered rows per tree.

Each row is annotated with the run's own values, read from the observer's public surface rather than
restated from the arguments, so **whether a drawn row presents a value the run does not hold is a
measurement**. A `CUT` mark means exactly that: a maximal digit run in the drawn row equals no value of
the run. It is not inferred from the row's length — the row `s18446744073709551615 t100 @0 e136` fills the
floor's 34 columns to the last cell and is not cut.

## Result

**5 of 63 rendered rows differ, and all five are at the `34 × 22` floor.** The eight wider viewports are
character-for-character unchanged, which is the intended blast radius: rule 8's order of loss engages only
where the width will not hold every field.

**2 of the 5 presented a value the run did not hold**, and those two are the defect:

| Configuration | Before | After |
|---|---|---|
| seed 9999, ticks `u64::MAX` | `s9999 t18446744073709551615 @0 e13` — the retained count **136** read as **13** | `s9999 t18446744073709551615 d0.75%` |
| seed and ticks `u64::MAX` | `s18446744073709551615 t18446744073` — the tick limit **18446744073709551615** read as **18446744073** | `seed 18446744073709551615` |

The other three differ without having been wrong: the superseded ladder's unspecified fall-through dropped
the resource density and the active decision source silently, and the new row keeps whichever of them the
width holds. `seed u64::MAX` at default ticks is the sharpest of the three — 34 columns before and 33
after, so the row got *narrower* while gaining a field, because the ladder tries every labelling before
losing one.

## Full sweep

```
== 160 x 48 ==
  default, seed 0  [seed=0 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols 
    after    |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  seed 999, 3 digits  [seed=999 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols 
    after    |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols
  seed 1e12, 13 digits  [seed=1000000000000 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols 
    after    |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols
  seed u64::MAX, 20 digits  [seed=18446744073709551615 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols 
    after    |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols
  seed 0, ticks u64::MAX  [seed=0 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols 
    after    |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols
  seed 9999, ticks u64::MAX  [seed=9999 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols 
    after    |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols
  seed and ticks u64::MAX  [seed=18446744073709551615 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols 
    after    |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols

== 160 x 44 ==
  default, seed 0  [seed=0 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols 
    after    |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  seed 999, 3 digits  [seed=999 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols 
    after    |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols
  seed 1e12, 13 digits  [seed=1000000000000 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols 
    after    |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols
  seed u64::MAX, 20 digits  [seed=18446744073709551615 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols 
    after    |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols
  seed 0, ticks u64::MAX  [seed=0 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols 
    after    |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols
  seed 9999, ticks u64::MAX  [seed=9999 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols 
    after    |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols
  seed and ticks u64::MAX  [seed=18446744073709551615 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols 
    after    |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols

== 160 x 40 ==
  default, seed 0  [seed=0 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols 
    after    |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  seed 999, 3 digits  [seed=999 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols 
    after    |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols
  seed 1e12, 13 digits  [seed=1000000000000 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols 
    after    |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols
  seed u64::MAX, 20 digits  [seed=18446744073709551615 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols 
    after    |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols
  seed 0, ticks u64::MAX  [seed=0 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols 
    after    |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols
  seed 9999, ticks u64::MAX  [seed=9999 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols 
    after    |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols
  seed and ticks u64::MAX  [seed=18446744073709551615 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols 
    after    |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols

== 140 x 44 ==
  default, seed 0  [seed=0 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols 
    after    |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  seed 999, 3 digits  [seed=999 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols 
    after    |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols
  seed 1e12, 13 digits  [seed=1000000000000 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols 
    after    |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols
  seed u64::MAX, 20 digits  [seed=18446744073709551615 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols 
    after    |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols
  seed 0, ticks u64::MAX  [seed=0 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols 
    after    |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols
  seed 9999, ticks u64::MAX  [seed=9999 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols 
    after    |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols
  seed and ticks u64::MAX  [seed=18446744073709551615 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols 
    after    |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols

== 140 x 43 ==
  default, seed 0  [seed=0 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols 
    after    |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  seed 999, 3 digits  [seed=999 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols 
    after    |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols
  seed 1e12, 13 digits  [seed=1000000000000 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols 
    after    |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols
  seed u64::MAX, 20 digits  [seed=18446744073709551615 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols 
    after    |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols
  seed 0, ticks u64::MAX  [seed=0 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols 
    after    |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols
  seed 9999, ticks u64::MAX  [seed=9999 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols 
    after    |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols
  seed and ticks u64::MAX  [seed=18446744073709551615 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols 
    after    |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols

== 120 x 48 ==
  default, seed 0  [seed=0 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols 
    after    |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  seed 999, 3 digits  [seed=999 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols 
    after    |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols
  seed 1e12, 13 digits  [seed=1000000000000 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols 
    after    |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols
  seed u64::MAX, 20 digits  [seed=18446744073709551615 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols 
    after    |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols
  seed 0, ticks u64::MAX  [seed=0 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols 
    after    |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols
  seed 9999, ticks u64::MAX  [seed=9999 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols 
    after    |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols
  seed and ticks u64::MAX  [seed=18446744073709551615 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols 
    after    |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols

== 120 x 30 ==
  default, seed 0  [seed=0 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols 
    after    |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  seed 999, 3 digits  [seed=999 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols 
    after    |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols
  seed 1e12, 13 digits  [seed=1000000000000 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols 
    after    |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols
  seed u64::MAX, 20 digits  [seed=18446744073709551615 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols 
    after    |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols
  seed 0, ticks u64::MAX  [seed=0 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols 
    after    |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols
  seed 9999, ticks u64::MAX  [seed=9999 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols 
    after    |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols
  seed and ticks u64::MAX  [seed=18446744073709551615 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols 
    after    |seed 18446744073709551615  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136| 106 cols

== 100 x 30 ==
  default, seed 0  [seed=0 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols 
    after    |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  seed 999, 3 digits  [seed=999 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols 
    after    |seed 999  ticks 100  density 0.75%  source reference  tick 0  events 136|  72 cols
  seed 1e12, 13 digits  [seed=1000000000000 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols 
    after    |seed 1000000000000  ticks 100  density 0.75%  source reference  tick 0  events 136|  82 cols
  seed u64::MAX, 20 digits  [seed=18446744073709551615 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols 
    after    |seed 18446744073709551615  ticks 100  density 0.75%  source reference  tick 0  events 136|  89 cols
  seed 0, ticks u64::MAX  [seed=0 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols 
    after    |seed 0  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  87 cols
  seed 9999, ticks u64::MAX  [seed=9999 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols 
    after    |seed 9999  ticks 18446744073709551615  density 0.75%  source reference  tick 0  events 136|  90 cols
  seed and ticks u64::MAX  [seed=18446744073709551615 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before   |s18446744073709551615 t18446744073709551615 d0.75% reference @0 e136|  68 cols 
    after    |s18446744073709551615 t18446744073709551615 d0.75% reference @0 e136|  68 cols

== 34 x 22 ==
  default, seed 0  [seed=0 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |s0 t100 d0.75% reference @0 e136|  32 cols 
    after    |s0 t100 d0.75% reference @0 e136|  32 cols
  seed 999, 3 digits  [seed=999 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before   |s999 t100 d0.75% reference @0 e136|  34 cols 
    after    |s999 t100 d0.75% reference @0 e136|  34 cols
  seed 1e12, 13 digits  [seed=1000000000000 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before * |s1000000000000 t100 @0 e136|  27 cols 
    after  * |s1000000000000 t100 d0.75% r @0|  31 cols
  seed u64::MAX, 20 digits  [seed=18446744073709551615 ticks=100 tick=0 events=136 density=0.75 source=reference]
    before * |s18446744073709551615 t100 @0 e136|  34 cols 
    after  * |s18446744073709551615 t100 d0.75%|  33 cols
  seed 0, ticks u64::MAX  [seed=0 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before * |s0 t18446744073709551615 @0 e136|  32 cols 
    after  * |s0 t18446744073709551615 d0.75% r|  33 cols
  seed 9999, ticks u64::MAX  [seed=9999 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before * |s9999 t18446744073709551615 @0 e13|  34 cols CUT: presents 13
    after  * |s9999 t18446744073709551615 d0.75%|  34 cols
  seed and ticks u64::MAX  [seed=18446744073709551615 ticks=18446744073709551615 tick=0 events=136 density=0.75 source=reference]
    before * |s18446744073709551615 t18446744073|  34 cols CUT: presents 18446744073
    after  * |seed 18446744073709551615|  25 cols
```
