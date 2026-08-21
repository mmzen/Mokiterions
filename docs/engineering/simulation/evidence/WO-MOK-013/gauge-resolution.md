# Gauge resolution — WO-MOK-013

Measured 2026-08-20 on this branch. The measurement is `gauge-resolution.txt` beside this file,
produced by the oracle retained as `wo013-oracle.rs`; this document reads it against
`evidence/WO-MOK-012/assessment-material/bar-quantization.txt`, which is the same measurement before
the change and which is what `VER-MOK-013`'s *Evidence retention* names as the before form.

Nothing in either form knows the fill arithmetic. A gauge is located in a rendered buffer by its
cells, and its filled count is the number of `█` characters drawn in it.

## The two arithmetics

| | Before, at `ff3a155` | After |
|---|---|---|
| Gauges per line | 4 | 2 |
| Entry height | 2 rows | 3 rows |
| Bar row overhead | `5 + 4 × 6 + 3 × 2 = 35` | `5 + 2 × 6 + 1 × 2 = 19` |
| `bar_width(interior)` | `min(20, (interior − 35) / 4)` | `min(20, (interior − 19) / 2)` |
| At the reference interior of 45 | **2 cells** | **13 cells** |

The roster pane is 47 columns wherever it is present, so its interior is 45 and one width is the
whole set. The after form establishes that by sweeping the plane rather than by reading rule 5:
`34..=200 × 22..=60` yields **one** distinct roster pane width, 47, and therefore one gauge width,
13. `VER-MOK-013` asks for the table "for each gauge width the implementation can produce"; there is
one, and a second would have appeared in `gauge-resolution.txt` as a second table.

## What 13 cells resolve

| | Before | After |
|---|---:|---:|
| Distinct renderable states for 101 values | **3** | **14** |
| Widest span of values drawing the same bar | **50** | **8** |
| Narrowest span other than `100` alone | **50** | 7 |
| Ten-point steps that move the fill, of the 91 in `0..=90` | **11** | **91** |

The before form's three states are `░░` for `0..=49`, `█░` for `50..=99` and `██` at `100`. The
91-step figure for it is arithmetic on that measured table rather than a second measurement: with
`filled = value × 2 / 100` the count changes only where the value crosses 50 or 100, so a ten-point
step moves the fill at the ten values `40..=49` and at `90`, and at the other 80 it does not. The
before form's own summary states the consequence more directly — "two attributes 49 apart draw the
same bar" — and the product owner reported it as such from a live pass.

The after form's 91 of 91 is measured, at every value and on all four gauges, and it is the property
`REQ-MOK-047` states: *any change of ten in the value it presents changes the number of filled
cells*. It holds because `(v + 10) × 13 / 100` exceeds `v × 13 / 100` by at least one for every `v`
— ten points is 1.3 cells at this width — and the smallest step it could survive on is a width of
10. Thirteen is what fits, not what was needed.

The full mapping from value to filled count is in `gauge-resolution.txt` as a run table: fourteen
rows, each naming the values that draw it, together covering `0..=100` exactly once. That is the
count at every value in the range, in run-length form, and expanding it to 101 rows would add no
measurement.

## The four gauges together

`gauge-resolution.txt` records all four gauges of the first entry at the ten multiples of ten, and
the four columns are identical at every one — `0, 1, 2, 3, 5, 6, 7, 9, 10, 11, 13`. That is expected
and is worth stating: `SPEC-MOK-003` rule 4 clause 7 bands three gauges and leaves `fear` unbanded,
and a band is a colour on a span. It changes no character and no cell count, so the unbanded gauge
quantizes exactly as the three banded ones do. The band is why `fear`'s column is not required to
match; the arithmetic is why it does.

The step from `3` at 30 to `5` at 40 is not an error. `40 × 13 / 100 = 5.2`, and `30 × 13 / 100 =
3.9` truncates to 3, so a ten-point step sometimes moves the fill by two cells. `REQ-MOK-047`
obliges the count to change, not to change by one.

## What is not claimed

- **Resolution is not accuracy.** Thirteen cells still map 101 values onto 14 states, so two values
  seven apart can draw the same bar. The requirement is a ten-point floor and this meets it with no
  margin at 7 and 8; the three-digit numeric value beside each bar carries the exact level, as it
  did before, and `SPEC-MOK-003` rule 2.5's redundancy rests on that and not on the fill.
- **This says nothing about the overlay below 47 columns.** The sweep above covers the roster *pane*.
  Rule 4's collapsed one-line form, drawn when the drawing area is narrower than 47 columns, has no
  bars at all, so there is no gauge there to resolve; the automated case asserts instead that no
  viewport without a roster draws a gauge, and that no bar of zero cells is ever drawn.
- **The cost is the entry height, and it is paid elsewhere.** Two gauges per line makes the entry
  three rows, which is what forced decision 1 to hold the log at six rows. `log-height.md` measures
  that consequence. This document measures only what the 13 cells buy.
