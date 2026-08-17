# Manual observation — WO-MOK-002

Measured 2026-08-17 against the committed tree, after the third `SPEC-MOK-001` amendment.
`VER-MOK-002` asks for a 20-tick traced excerpt in which perception, approach, arrival, and
consumption can be followed for one Mokiterion, plus assessments of scarcity, oscillation, and
end-of-run resource mix.

## 20-tick excerpt: M01, seed 42, density 0.75%

Trimmed to the fields that matter. Health is `100` throughout and is omitted.

```
tick=1  M01 move:east   position:12:1 satiety:100
tick=2  M01 move:north  position:12:0 satiety:99
tick=3  M01 move:south  position:12:1 satiety:98
tick=4  M01 move:east   position:13:1 satiety:97
tick=5  M01 move:south  position:13:2 satiety:96
tick=6  M01 move:north  position:13:1 satiety:95
tick=7  M01 move:south  position:13:2 satiety:94
tick=8  M01 move:south  position:13:3 satiety:93
tick=9  M01 move:north  position:13:2 satiety:92
tick=10 M01 move:east   position:14:2 satiety:91
tick=11 M01 move:west   position:13:2 satiety:90
tick=12 M01 move:south  position:13:3 satiety:89
tick=13 M01 move:north  position:13:2 satiety:88
tick=14 M01 move:south  position:13:3 satiety:87
tick=15 M01 move:east   position:14:3 satiety:86
tick=16 M01 move:east   position:15:3 satiety:85
tick=17 M01 move:east   position:16:3 satiety:84
tick=18 M01 move:east   position:17:3 satiety:83
tick=19 M01 move:north  position:17:2 satiety:82
tick=20 M01 move:north  position:17:1 satiety:81
tick=21 M01 eat:F0019   position:17:1 satiety:95   detail food:F0019;class:low
```

This excerpt is unusually legible, and every part of it is checkable against rule 5.

**F0019 was visible the whole time.** It is a low-class resource initialized at `17:1`:

```
tick=0 subject=F0019 event=food_initialized result=class:low,position:17:1,territory:A
```

M01 is at `12:1` from tick 1, a Chebyshev distance of 5, well inside the perception radius of
`16`. So the first fifteen ticks are not a Mokiterion failing to see food.

**Ticks 1 to 15 are search, because nothing in the world fits.** A low-class resource restores
`15` satiety, so it is worth approaching only at satiety of at most `85`. M01 begins at `100`.
Rule 5 case 3 finds no eligible target anywhere, so case 4 applies and each step is an unbiased
cardinal draw. The walk shows it: east, north, south, east, south, north, south, south, north,
east, west — reversals and revisits, exactly what a random walk looks like.

**Tick 16 is the first tick at which approach is possible, and approach begins on that tick.**
The trace records state before survival decay, so the decision at tick 16 was taken at satiety
`85`, the first value at which `85 + 15 = 100` does not exceed the attribute maximum. The change
in behavior is immediate and total.

**Ticks 16 to 20 are a textbook case-3 approach.** From `14:3` to `17:1` is three cells east and
two cells north. Rule 5 case 3 moves on the horizontal axis while the perceived direction has an
easterly or westerly component, otherwise on the vertical axis. The trace is east, east, east,
then north, north — three horizontal steps until the easterly component is exhausted, then two
vertical. Five steps for a five-step Manhattan distance, with no wasted move.

**Tick 21 is case 1.** Co-located, low class, satiety `80` after decay, restoration `15`, total
`95`, not clipped. It eats.

Read end to end, the corrected rule produces a Mokiterion that forages when it is hungry enough
to benefit and wanders when it is not. Under the previous rule the same Mokiterion would have
walked to the nearest resource immediately and stood on it.

## Assessment: is search-while-full a problem?

It is a visible consequence of the amendment and worth stating plainly rather than burying.
A Mokiterion above satiety `85` has no eligible target in the entire world and searches, so the
opening ticks of every run are a pure random walk and full Mokiterions never travel purposefully.

Assessed against `INT-MOK-002`, this is acceptable and arguably an improvement. Movement costs
no energy, so the walk is not wasteful; the alternative the previous rule produced was worse,
because a full Mokiterion would walk to a resource it could not use, sit on it, and hold it out
of reach of a hungrier neighbour. Foraging is now driven by need rather than by proximity.

What it does cost is legibility of intent: for a stretch of every life, the reference source is
indistinguishable from the random baseline. Anyone reading a trace should check satiety before
concluding that a Mokiterion is behaving randomly.

## Assessment: two-cell oscillation

The defect the third amendment exists to remove. Counted as the fraction of an agent's traced
ticks in which its position equals its position two ticks earlier and differs from one tick
earlier. Seed 42, density `0.75%`:

| Agent | Oscillating ticks | Ticks lived | Rate |
|---|---:|---:|---:|
| M01 | 55 | 277 | 19.9% |
| M02 | 88 | 998 | 8.8% |
| M03 | 95 | 998 | 9.5% |
| M04 | 122 | 991 | 12.3% |
| M05 | 104 | 998 | 10.4% |
| M06 | 112 | 998 | 11.2% |
| M07 | 79 | 998 | 7.9% |
| M08 | 60 | 381 | 15.7% |
| M09 | 109 | 998 | 10.9% |
| M10 | 78 | 998 | 7.8% |
| M11 | 97 | 998 | 9.7% |
| M12 | 98 | 706 | 13.9% |
| **Total** | **1,097** | **10,339** | **10.6%** |

Against a random-walk floor of **12.2%**, measured by running the baseline source on the same
seed and density through the identical counter. **The corrected rule sits below the floor, so the
systematic oscillation is gone rather than merely reduced.** Under the previous rule the same
measurement on the same seed gave 35.7%.

The split is exact and worth recording. Four Mokiterions died on this seed — M01 at tick 279,
M08 at 383, M12 at 708, M04 at 993 — and **all four are the four above the floor**. All eight
survivors are below it. So what remains of the measurement is not a systematic inefficiency but a
marker: a Mokiterion that spent its final stretch at low satiety, circling near resources too
rich to use, both oscillates and dies. The oscillation is a symptom of the accumulation effect
recorded below, not a residue of the defect the amendment removed.

## Assessment: scarcity

At the declared density of `0.75%`, survivors at tick 1,000 are 8, 11, 8, 9, and 11 on the five
declared seeds. **No seed retains all twelve.** The adverse observation in `VER-MOK-002` — that
twelve survivors everywhere would mean scarcity had been removed — is not triggered, and the
world remains a place where Mokiterions die of starvation.

This is the reason `REQ-MOK-014` declares only the default density. At `1.50%` the counts are 11,
12, 12, 11, 12: three of five seeds retain everyone, which would trip that observation on a row
the requirement had itself declared. Those figures are recorded as evidence in `density-curve.md`
and carry no obligation.

## Assessment: resource mix

Territories begin at roughly 20 low, 20 medium, 21 high, because initialization cycles the
classes. At tick 1,000 territory A on seed 42 holds 6 low, 17 medium, 38 high. Every seed shows
the same direction, and the effect is characterised in `density-curve.md`.

Its visible signature in a trace is high-class resources sitting untouched for hundreds of ticks
while Mokiterions walk past them, which is correct behavior under rule 5 and not a defect: a
high-class resource is worth approaching only at satiety of at most `50`. The consequence for
long runs is in `determinism-and-resilience.md`, and the product owner accepted it on 2026-08-17
as Phase 2 scope.

## Assessment: the reference source is a development instrument

Output identifies the active source on its own line before tick processing begins, and `--help`
states that the reference policy is "a deterministic development instrument, not autonomous
behavior. It seeks and consumes perceived food so that world viability can be measured." Nothing
in this work order presents its behavior as emergent. `ADR-MOK-001` keeps it behind the same
decision boundary as the baseline, and it holds no state.
