# Rule 18's summary per source, and REQ-MOK-060's composition ceiling on an uncorrected candidate

| Field | Value |
|---|---|
| Retention item | "rule 18's final summary per seed under each of the four sources, and the composition ratio computed per territory per class, at both commits" |
| Oracle | `VER-MOK-016` oracle 4 in both of its coverage rows — the ceiling itself, and "measured from rule 18's summary", which requires the composition to be read from the run's own final summary with no new event and no new instrumentation — together with oracle 1's row asking for the pre-change state as the contrast; `REQ-MOK-060` |
| Reader | `analysis/composition.py`, over 120 cells — the 90-cell three-source matrix and the 30 `social` cells |
| Exit code | **`0`** — 35 checked properties, plus a guard that fails on any capture line that is neither an event nor a summary |
| Raw output | `post/composition.txt` |
| Capture | `git archive 7c4aef3967406c05d80da963695898b77f5329e9` (90 cells) and `git archive 59d61b915630fd55f04bcdbb346aa22cdbfdfff6` (30 `social` cells), every cell digest-matched against `post/post-manifest.txt` or `post/social-manifest.txt`; the pre-change side from `baseline/summary.txt` and `baseline/init/` at `39662d13abd08e3410648d1c59ad38384f8ad2d2` |
| Date | 2026-08-21 |

---

## 1. What this record is, on a requirement that is not implemented

`REQ-MOK-060` is the composition ceiling: at 1,000 ticks, at the default resource density, under
`reference`, `individual` or `social`, no calorie class may hold more than half of any territory's
standing resources, on every declared verification seed. **It is unimplemented at this candidate.**
The product owner deferred it on 2026-08-20 and the descope of 2026-08-21 carried it to
`WO-MOK-017`, so `WO-MOK-016` no longer claims it.

That makes this file a different kind of evidence from the rest of the packet, and the difference is
worth stating before any figure appears. Every other record here measures whether something holds.
This one measures **the curve a deferred decision will be taken on**, because `WO-MOK-016.md` says it
must: "The measurement each is to be decided on is nonetheless taken here and retained in
`composition.md`, so `WO-MOK-017` inherits the curve and not just the question." Two decisions are
waiting on it — the ceiling's value, and whether a per-class floor should accompany it, which are the
two halves of `VER-MOK-016`'s manual assessment 5.

So the breach in §6 is reported as a measurement and never as a failed check. A reader that exited
non-zero on it would be reporting an approved deferral as a defect. Everything else here is a check,
and `analysis/composition.py` exits non-zero if any of the 35 fires.

Three things are deliberately not done.

- **No temporary build.** `post/branches.md` §1 states the reason and it applies unchanged: a figure
  from a build that no longer exists is not re-derivable by anyone, so it is not evidence.
- **No new test.** `post/test-census-reconciliation.md` fixes the census figures this packet
  retains, and adding a test would move them.
- **No new event and no new instrumentation**, which is not a choice but oracle 4's own clause. The
  composition has to be readable from the run's output as released, and it is: rule 18's summary
  carries all six counts.

## 2. The line everything here is read from

`emit_summary` (`mokiterions-core/src/simulation.rs:2967`) prints one line at the end of every run:

```
summary reason=<reason> ticks=<n> survivors=<n> deaths=<n> territory_a=<n> territory_b=<n>
        food_a_low=<n> food_a_medium=<n> food_a_high=<n> food_b_low=<n> food_b_medium=<n> food_b_high=<n>
```

`food_counts` (`:2955`) computes the six food fields by walking the standing resources and bucketing
each one by `food.position.territory()` and by its class. That is exactly what `REQ-MOK-060` means
when it says the composition is measurable from the run's own output.

What the line does **not** carry is a ratio. Six counts are not a share, and the requirement is
stated as a share of a territory's standing resources, so §5 computes one. That is the whole of the
gap between what the engine reports and what the requirement is written against — which is itself
worth recording, because it means the requirement can be evaluated on any run anyone has already
captured, including the ninety cells captured before it existed.

Four properties of the line are checked at every one of the 120 cells, and all hold:

| Property | Result |
|---|---|
| Exactly one `summary` line, and it is the last line the cell prints | 120 of 120 |
| `survivors` + `deaths` = 12, the initial population | 120 of 120 |
| `territory_a` + `territory_b` = `survivors` | 120 of 120 |
| `reason` is `extinction` or `tick_limit`, and is `extinction` exactly when `survivors` is 0 | 120 of 120 |

The first is not a formality. Every figure in this file is read off that line, so where it sits in
the stream is part of what is being measured: a summary printed before the last tick would be a
different measurement wearing the same name.

## 3. Why the summary is not transcribed but reconstructed

A reader that copied twelve numbers off the last line of each cell would report whatever the engine
reported and could not fail. It would confirm nothing except its own arithmetic. So the six food
counts are rebuilt from the records that produced them, and the released stream makes that possible
because every standing resource is accounted for in it:

- it arrives in `food_initialized result=class:low,position:47:52,territory:A`, with class, position
  and territory;
- or in `food_regenerated subject=<A|B> result=food:F0123,class:medium,position:99:58`, with class
  and position, under the territory that added it;
- it leaves in `food_consumed result=food:F0001,class:low,satiety:81->96,energy:81->86`, which names
  it and its class;
- and nothing else in the released records creates or destroys one.

So the standing composition at the end of a run is `initialized + regenerated − consumed`, per
territory per class, and it must equal what rule 18 printed.

| What is checked | Result |
|---|---|
| Cells where the reconstruction reproduces all six printed counts | **120 of 120** |
| Resources placed, each with its territory derived from its position and checked against the territory the record printed | 39,924 |
| Consumptions, each checked against the class the resource was placed with and against the resource still standing | 26,136 |
| Cells where the arithmetic count and the surviving-identifier set agree | 120 of 120 |

Two details make this stronger than a checksum. `food_counts` derives a resource's territory from
its **position** rather than from a stored field, so the reader does the same and then checks the
derived territory against the one the record printed — `territory:A` on a placement, the emitting
territory on a regeneration. That is the check that would catch a resource added outside the
territory that drew for it. And the standing set is counted two ways, arithmetically and as the set
of identifiers never consumed, so a resource counted twice or a consumption of something never
placed would show up as a disagreement between two paths rather than as a plausible number.

## 4. Rule 18's final summary per seed under each of the four sources

The retention item's first half, in full. Sixty rows: four sources at three densities on five seeds.
The traced and untraced cell of each triple print the same summary in **60 of 60** cases, so one row
covers both.

| Source | Density | Seed | Reason | Ticks | Surv. | Dead | A | B | A low | A med | A high | B low | B med | B high |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `baseline` | 0.15 | 0 | `extinction` | 119 | 0 | 12 | 0 | 0 | 4 | 4 | 4 | 4 | 4 | 4 |
| `baseline` | 0.15 | 1 | `extinction` | 119 | 0 | 12 | 0 | 0 | 4 | 4 | 4 | 4 | 4 | 4 |
| `baseline` | 0.15 | 42 | `extinction` | 119 | 0 | 12 | 0 | 0 | 4 | 4 | 4 | 4 | 4 | 4 |
| `baseline` | 0.15 | 123 | `extinction` | 119 | 0 | 12 | 0 | 0 | 4 | 4 | 4 | 4 | 4 | 4 |
| `baseline` | 0.15 | 777 | `extinction` | 119 | 0 | 12 | 0 | 0 | 4 | 4 | 4 | 4 | 4 | 4 |
| `baseline` | 0.75 | 0 | `extinction` | 119 | 0 | 12 | 0 | 0 | 21 | 20 | 20 | 21 | 20 | 20 |
| `baseline` | 0.75 | 1 | `extinction` | 119 | 0 | 12 | 0 | 0 | 21 | 20 | 20 | 21 | 20 | 20 |
| `baseline` | 0.75 | 42 | `extinction` | 142 | 0 | 12 | 0 | 0 | 21 | 20 | 20 | 21 | 20 | 20 |
| `baseline` | 0.75 | 123 | `extinction` | 168 | 0 | 12 | 0 | 0 | 21 | 20 | 20 | 21 | 20 | 20 |
| `baseline` | 0.75 | 777 | `extinction` | 134 | 0 | 12 | 0 | 0 | 21 | 20 | 20 | 20 | 21 | 20 |
| `baseline` | 1.50 | 0 | `extinction` | 193 | 0 | 12 | 0 | 0 | 40 | 42 | 40 | 41 | 41 | 40 |
| `baseline` | 1.50 | 1 | `extinction` | 134 | 0 | 12 | 0 | 0 | 41 | 41 | 40 | 41 | 41 | 40 |
| `baseline` | 1.50 | 42 | `extinction` | 169 | 0 | 12 | 0 | 0 | 41 | 41 | 40 | 41 | 41 | 40 |
| `baseline` | 1.50 | 123 | `extinction` | 119 | 0 | 12 | 0 | 0 | 41 | 41 | 40 | 41 | 41 | 40 |
| `baseline` | 1.50 | 777 | `extinction` | 142 | 0 | 12 | 0 | 0 | 40 | 40 | 42 | 41 | 41 | 40 |
| `reference` | 0.15 | 0 | `extinction` | 409 | 0 | 12 | 0 | 0 | 1 | 5 | 6 | 3 | 2 | 7 |
| `reference` | 0.15 | 1 | `extinction` | 709 | 0 | 12 | 0 | 0 | 6 | 4 | 2 | 3 | 4 | 5 |
| `reference` | 0.15 | 42 | `extinction` | 569 | 0 | 12 | 0 | 0 | 3 | 4 | 5 | 3 | 5 | 4 |
| `reference` | 0.15 | 123 | `extinction` | 469 | 0 | 12 | 0 | 0 | 5 | 3 | 4 | 0 | 9 | 3 |
| `reference` | 0.15 | 777 | `extinction` | 927 | 0 | 12 | 0 | 0 | 4 | 1 | 7 | 5 | 3 | 4 |
| `reference` | 0.75 | 0 | `tick_limit` | 1000 | 8 | 4 | 3 | 5 | 7 | 9 | 45 | 7 | 16 | 32 |
| `reference` | 0.75 | 1 | `tick_limit` | 1000 | 11 | 1 | 4 | 7 | 4 | 10 | 43 | 7 | 7 | 25 |
| `reference` | 0.75 | 42 | `tick_limit` | 1000 | 8 | 4 | 3 | 5 | 6 | 17 | 38 | 8 | 7 | 26 |
| `reference` | 0.75 | 123 | `tick_limit` | 1000 | 9 | 3 | 4 | 5 | 8 | 11 | 42 | 4 | 12 | 44 |
| `reference` | 0.75 | 777 | `tick_limit` | 1000 | 11 | 1 | 6 | 5 | 13 | 20 | 28 | 8 | 13 | 18 |
| `reference` | 1.50 | 0 | `tick_limit` | 1000 | 11 | 1 | 4 | 7 | 18 | 23 | 81 | 7 | 9 | 43 |
| `reference` | 1.50 | 1 | `tick_limit` | 1000 | 12 | 0 | 5 | 7 | 8 | 17 | 67 | 10 | 8 | 75 |
| `reference` | 1.50 | 42 | `tick_limit` | 1000 | 12 | 0 | 6 | 6 | 5 | 13 | 76 | 10 | 14 | 69 |
| `reference` | 1.50 | 123 | `tick_limit` | 1000 | 11 | 1 | 5 | 6 | 14 | 18 | 84 | 9 | 16 | 64 |
| `reference` | 1.50 | 777 | `tick_limit` | 1000 | 12 | 0 | 5 | 7 | 6 | 14 | 67 | 7 | 14 | 80 |
| `individual` | 0.15 | 0 | `extinction` | 674 | 0 | 12 | 0 | 0 | 1 | 7 | 4 | 4 | 4 | 4 |
| `individual` | 0.15 | 1 | `extinction` | 757 | 0 | 12 | 0 | 0 | 6 | 6 | 0 | 5 | 2 | 5 |
| `individual` | 0.15 | 42 | `extinction` | 494 | 0 | 12 | 0 | 0 | 6 | 3 | 3 | 3 | 4 | 5 |
| `individual` | 0.15 | 123 | `extinction` | 463 | 0 | 12 | 0 | 0 | 6 | 3 | 3 | 3 | 6 | 3 |
| `individual` | 0.15 | 777 | `extinction` | 379 | 0 | 12 | 0 | 0 | 5 | 5 | 2 | 4 | 3 | 5 |
| `individual` | 0.75 | 0 | `tick_limit` | 1000 | 11 | 1 | 6 | 5 | 7 | 10 | 24 | 10 | 13 | 37 |
| `individual` | 0.75 | 1 | `tick_limit` | 1000 | 9 | 3 | 5 | 4 | 7 | 11 | 42 | 22 | 16 | 21 |
| `individual` | 0.75 | 42 | `tick_limit` | 1000 | 9 | 3 | 4 | 5 | 13 | 17 | 31 | 13 | 11 | 36 |
| `individual` | 0.75 | 123 | `tick_limit` | 1000 | 10 | 2 | 7 | 3 | 9 | 11 | 40 | 8 | 14 | 39 |
| `individual` | 0.75 | 777 | `tick_limit` | 1000 | 12 | 0 | 8 | 4 | 6 | 6 | 41 | 10 | 8 | 26 |
| `individual` | 1.50 | 0 | `tick_limit` | 1000 | 12 | 0 | 5 | 7 | 6 | 27 | 85 | 4 | 14 | 47 |
| `individual` | 1.50 | 1 | `tick_limit` | 1000 | 12 | 0 | 5 | 7 | 15 | 24 | 72 | 3 | 14 | 50 |
| `individual` | 1.50 | 42 | `tick_limit` | 1000 | 12 | 0 | 8 | 4 | 10 | 9 | 52 | 10 | 23 | 67 |
| `individual` | 1.50 | 123 | `tick_limit` | 1000 | 11 | 1 | 5 | 6 | 10 | 17 | 80 | 13 | 12 | 45 |
| `individual` | 1.50 | 777 | `tick_limit` | 1000 | 11 | 1 | 4 | 7 | 15 | 27 | 65 | 11 | 13 | 74 |
| `social` | 0.15 | 0 | `tick_limit` | 1000 | 1 | 11 | 1 | 0 | 4 | 3 | 5 | 5 | 3 | 4 |
| `social` | 0.15 | 1 | `extinction` | 393 | 0 | 12 | 0 | 0 | 3 | 6 | 3 | 4 | 4 | 4 |
| `social` | 0.15 | 42 | `extinction` | 583 | 0 | 12 | 0 | 0 | 3 | 4 | 5 | 5 | 2 | 5 |
| `social` | 0.15 | 123 | `extinction` | 509 | 0 | 12 | 0 | 0 | 3 | 5 | 4 | 2 | 4 | 6 |
| `social` | 0.15 | 777 | `extinction` | 426 | 0 | 12 | 0 | 0 | 3 | 3 | 6 | 4 | 7 | 1 |
| `social` | 0.75 | 0 | `tick_limit` | 1000 | 9 | 3 | 3 | 6 | 9 | 16 | 36 | 2 | 7 | 40 |
| `social` | 0.75 | 1 | `tick_limit` | 1000 | 10 | 2 | 4 | 6 | 6 | 12 | 43 | 5 | 10 | 46 |
| `social` | 0.75 | 42 | `tick_limit` | 1000 | 9 | 3 | 6 | 3 | 6 | 13 | 41 | 7 | 15 | 39 |
| `social` | 0.75 | 123 | `tick_limit` | 1000 | 9 | 3 | 4 | 5 | 12 | 11 | 33 | 7 | 13 | 41 |
| `social` | 0.75 | 777 | `tick_limit` | 1000 | 11 | 1 | 6 | 5 | 6 | 11 | 39 | 10 | 9 | 41 |
| `social` | 1.50 | 0 | `tick_limit` | 1000 | 11 | 1 | 5 | 6 | 9 | 15 | 87 | 11 | 14 | 62 |
| `social` | 1.50 | 1 | `tick_limit` | 1000 | 10 | 2 | 6 | 4 | 9 | 28 | 78 | 10 | 27 | 82 |
| `social` | 1.50 | 42 | `tick_limit` | 1000 | 11 | 1 | 5 | 6 | 5 | 13 | 84 | 5 | 9 | 82 |
| `social` | 1.50 | 123 | `tick_limit` | 1000 | 11 | 1 | 7 | 4 | 8 | 20 | 74 | 9 | 15 | 71 |
| `social` | 1.50 | 777 | `tick_limit` | 1000 | 10 | 2 | 7 | 3 | 3 | 12 | 80 | 11 | 21 | 90 |

**The pre-change commit prints an identical summary on all 45 triples it holds.** That is not a null
result to be passed over — it is `REQ-MOK-060` being unimplemented, read on a single line. The
requirement's permitted correction site is the sources' waste condition; no source's waste condition
was changed; so no run's standing composition changed either. `post/byte-identity.txt` measures the
same thing over whole streams and at far greater strength, and this is the clause of the retention
item that asks for it summary by summary.

The 30 `social` cells have no pre-change side and cannot have one: the source does not exist at the
baseline commit, so a fourth `--policy` value would have failed thirty runs with a configuration
error. `capture.sh` deliberately takes three sources and `capture-social.sh` is the separate
capture. For `social`, "at both commits" is answered by one commit and the reason for it, which is
what this file states rather than leaving a column blank.

## 5. The composition ratio per territory per class

The retention item's second half: the share each class holds of its own territory's standing
resources, which is the form `REQ-MOK-060` is written in. By §4 every row below is the pre-change
commit's too, wherever the pre-change capture holds the triple.

| Source | Density | Seed | A standing | A low | A med | A high | B standing | B low | B med | B high |
|---|---|---|---|---|---|---|---|---|---|---|
| `baseline` | 0.15 | 0 | 12 | 33.3% | 33.3% | 33.3% | 12 | 33.3% | 33.3% | 33.3% |
| `baseline` | 0.15 | 1 | 12 | 33.3% | 33.3% | 33.3% | 12 | 33.3% | 33.3% | 33.3% |
| `baseline` | 0.15 | 42 | 12 | 33.3% | 33.3% | 33.3% | 12 | 33.3% | 33.3% | 33.3% |
| `baseline` | 0.15 | 123 | 12 | 33.3% | 33.3% | 33.3% | 12 | 33.3% | 33.3% | 33.3% |
| `baseline` | 0.15 | 777 | 12 | 33.3% | 33.3% | 33.3% | 12 | 33.3% | 33.3% | 33.3% |
| `baseline` | 0.75 | 0 | 61 | 34.4% | 32.8% | 32.8% | 61 | 34.4% | 32.8% | 32.8% |
| `baseline` | 0.75 | 1 | 61 | 34.4% | 32.8% | 32.8% | 61 | 34.4% | 32.8% | 32.8% |
| `baseline` | 0.75 | 42 | 61 | 34.4% | 32.8% | 32.8% | 61 | 34.4% | 32.8% | 32.8% |
| `baseline` | 0.75 | 123 | 61 | 34.4% | 32.8% | 32.8% | 61 | 34.4% | 32.8% | 32.8% |
| `baseline` | 0.75 | 777 | 61 | 34.4% | 32.8% | 32.8% | 61 | 32.8% | 34.4% | 32.8% |
| `baseline` | 1.50 | 0 | 122 | 32.8% | 34.4% | 32.8% | 122 | 33.6% | 33.6% | 32.8% |
| `baseline` | 1.50 | 1 | 122 | 33.6% | 33.6% | 32.8% | 122 | 33.6% | 33.6% | 32.8% |
| `baseline` | 1.50 | 42 | 122 | 33.6% | 33.6% | 32.8% | 122 | 33.6% | 33.6% | 32.8% |
| `baseline` | 1.50 | 123 | 122 | 33.6% | 33.6% | 32.8% | 122 | 33.6% | 33.6% | 32.8% |
| `baseline` | 1.50 | 777 | 122 | 32.8% | 32.8% | 34.4% | 122 | 33.6% | 33.6% | 32.8% |
| `reference` | 0.15 | 0 | 12 | 8.3% | 41.7% | 50.0% | 12 | 25.0% | 16.7% | 58.3% |
| `reference` | 0.15 | 1 | 12 | 50.0% | 33.3% | 16.7% | 12 | 25.0% | 33.3% | 41.7% |
| `reference` | 0.15 | 42 | 12 | 25.0% | 33.3% | 41.7% | 12 | 25.0% | 41.7% | 33.3% |
| `reference` | 0.15 | 123 | 12 | 41.7% | 25.0% | 33.3% | 12 | 0.0% | 75.0% | 25.0% |
| `reference` | 0.15 | 777 | 12 | 33.3% | 8.3% | 58.3% | 12 | 41.7% | 25.0% | 33.3% |
| `reference` | 0.75 | 0 | 61 | 11.5% | 14.8% | 73.8% | 55 | 12.7% | 29.1% | 58.2% |
| `reference` | 0.75 | 1 | 57 | 7.0% | 17.5% | 75.4% | 39 | 17.9% | 17.9% | 64.1% |
| `reference` | 0.75 | 42 | 61 | 9.8% | 27.9% | 62.3% | 41 | 19.5% | 17.1% | 63.4% |
| `reference` | 0.75 | 123 | 61 | 13.1% | 18.0% | 68.9% | 60 | 6.7% | 20.0% | 73.3% |
| `reference` | 0.75 | 777 | 61 | 21.3% | 32.8% | 45.9% | 39 | 20.5% | 33.3% | 46.2% |
| `reference` | 1.50 | 0 | 122 | 14.8% | 18.9% | 66.4% | 59 | 11.9% | 15.3% | 72.9% |
| `reference` | 1.50 | 1 | 92 | 8.7% | 18.5% | 72.8% | 93 | 10.8% | 8.6% | 80.6% |
| `reference` | 1.50 | 42 | 94 | 5.3% | 13.8% | 80.9% | 93 | 10.8% | 15.1% | 74.2% |
| `reference` | 1.50 | 123 | 116 | 12.1% | 15.5% | 72.4% | 89 | 10.1% | 18.0% | 71.9% |
| `reference` | 1.50 | 777 | 87 | 6.9% | 16.1% | 77.0% | 101 | 6.9% | 13.9% | 79.2% |
| `individual` | 0.15 | 0 | 12 | 8.3% | 58.3% | 33.3% | 12 | 33.3% | 33.3% | 33.3% |
| `individual` | 0.15 | 1 | 12 | 50.0% | 50.0% | 0.0% | 12 | 41.7% | 16.7% | 41.7% |
| `individual` | 0.15 | 42 | 12 | 50.0% | 25.0% | 25.0% | 12 | 25.0% | 33.3% | 41.7% |
| `individual` | 0.15 | 123 | 12 | 50.0% | 25.0% | 25.0% | 12 | 25.0% | 50.0% | 25.0% |
| `individual` | 0.15 | 777 | 12 | 41.7% | 41.7% | 16.7% | 12 | 33.3% | 25.0% | 41.7% |
| `individual` | 0.75 | 0 | 41 | 17.1% | 24.4% | 58.5% | 60 | 16.7% | 21.7% | 61.7% |
| `individual` | 0.75 | 1 | 60 | 11.7% | 18.3% | 70.0% | 59 | 37.3% | 27.1% | 35.6% |
| `individual` | 0.75 | 42 | 61 | 21.3% | 27.9% | 50.8% | 60 | 21.7% | 18.3% | 60.0% |
| `individual` | 0.75 | 123 | 60 | 15.0% | 18.3% | 66.7% | 61 | 13.1% | 23.0% | 63.9% |
| `individual` | 0.75 | 777 | 53 | 11.3% | 11.3% | 77.4% | 44 | 22.7% | 18.2% | 59.1% |
| `individual` | 1.50 | 0 | 118 | 5.1% | 22.9% | 72.0% | 65 | 6.2% | 21.5% | 72.3% |
| `individual` | 1.50 | 1 | 111 | 13.5% | 21.6% | 64.9% | 67 | 4.5% | 20.9% | 74.6% |
| `individual` | 1.50 | 42 | 71 | 14.1% | 12.7% | 73.2% | 100 | 10.0% | 23.0% | 67.0% |
| `individual` | 1.50 | 123 | 107 | 9.3% | 15.9% | 74.8% | 70 | 18.6% | 17.1% | 64.3% |
| `individual` | 1.50 | 777 | 107 | 14.0% | 25.2% | 60.7% | 98 | 11.2% | 13.3% | 75.5% |
| `social` | 0.15 | 0 | 12 | 33.3% | 25.0% | 41.7% | 12 | 41.7% | 25.0% | 33.3% |
| `social` | 0.15 | 1 | 12 | 25.0% | 50.0% | 25.0% | 12 | 33.3% | 33.3% | 33.3% |
| `social` | 0.15 | 42 | 12 | 25.0% | 33.3% | 41.7% | 12 | 41.7% | 16.7% | 41.7% |
| `social` | 0.15 | 123 | 12 | 25.0% | 41.7% | 33.3% | 12 | 16.7% | 33.3% | 50.0% |
| `social` | 0.15 | 777 | 12 | 25.0% | 25.0% | 50.0% | 12 | 33.3% | 58.3% | 8.3% |
| `social` | 0.75 | 0 | 61 | 14.8% | 26.2% | 59.0% | 49 | 4.1% | 14.3% | 81.6% |
| `social` | 0.75 | 1 | 61 | 9.8% | 19.7% | 70.5% | 61 | 8.2% | 16.4% | 75.4% |
| `social` | 0.75 | 42 | 60 | 10.0% | 21.7% | 68.3% | 61 | 11.5% | 24.6% | 63.9% |
| `social` | 0.75 | 123 | 56 | 21.4% | 19.6% | 58.9% | 61 | 11.5% | 21.3% | 67.2% |
| `social` | 0.75 | 777 | 56 | 10.7% | 19.6% | 69.6% | 60 | 16.7% | 15.0% | 68.3% |
| `social` | 1.50 | 0 | 111 | 8.1% | 13.5% | 78.4% | 87 | 12.6% | 16.1% | 71.3% |
| `social` | 1.50 | 1 | 115 | 7.8% | 24.3% | 67.8% | 119 | 8.4% | 22.7% | 68.9% |
| `social` | 1.50 | 42 | 102 | 4.9% | 12.7% | 82.4% | 96 | 5.2% | 9.4% | 85.4% |
| `social` | 1.50 | 123 | 102 | 7.8% | 19.6% | 72.5% | 95 | 9.5% | 15.8% | 74.7% |
| `social` | 1.50 | 777 | 95 | 3.2% | 12.6% | 84.2% | 122 | 9.0% | 17.2% | 73.8% |

The ratio is per territory because the two territories regenerate independently under rules 14 to
16, which is `REQ-MOK-060`'s stated reason for not averaging them. §9 measures whether that choice
is load-bearing at this candidate.

Note the standing totals, which are not fixed: a territory at the default density starts with 61
resources and ends with anywhere from 39 to 61. Rule 16's regeneration is probabilistic, so a
territory that is eaten down faster than it regenerates carries fewer resources for the rest of the
run — and a share of a smaller total moves more per resource. That is why the ceiling is stated as a
share rather than as a count, and it is why §6 prints the standing total beside every share.

## 6. `REQ-MOK-060` evaluated exactly as stated

The requirement's trigger is a 1,000-tick run at the default density under one of three named
sources; its obligation is that no class holds more than half of any territory's standing resources;
its population is the five declared verification seeds. Thirty evaluations follow — three sources,
five seeds, two territories — and the trigger is checked rather than assumed: every one of the
fifteen runs reaches `reason=tick_limit ticks=1000`.

| Source | Seed | Reached 1,000 | Terr. | Standing | Widest class | Share | Against one half |
|---|---|---|---|---|---|---|---|
| `reference` | 0 | yes | A | 61 | `high` | 73.8% | **breached** |
| `reference` | 0 | yes | B | 55 | `high` | 58.2% | **breached** |
| `reference` | 1 | yes | A | 57 | `high` | 75.4% | **breached** |
| `reference` | 1 | yes | B | 39 | `high` | 64.1% | **breached** |
| `reference` | 42 | yes | A | 61 | `high` | 62.3% | **breached** |
| `reference` | 42 | yes | B | 41 | `high` | 63.4% | **breached** |
| `reference` | 123 | yes | A | 61 | `high` | 68.9% | **breached** |
| `reference` | 123 | yes | B | 60 | `high` | 73.3% | **breached** |
| `reference` | 777 | yes | A | 61 | `high` | 45.9% | met |
| `reference` | 777 | yes | B | 39 | `high` | 46.2% | met |
| `individual` | 0 | yes | A | 41 | `high` | 58.5% | **breached** |
| `individual` | 0 | yes | B | 60 | `high` | 61.7% | **breached** |
| `individual` | 1 | yes | A | 60 | `high` | 70.0% | **breached** |
| `individual` | 1 | yes | B | 59 | `low` | 37.3% | met |
| `individual` | 42 | yes | A | 61 | `high` | 50.8% | **breached** |
| `individual` | 42 | yes | B | 60 | `high` | 60.0% | **breached** |
| `individual` | 123 | yes | A | 60 | `high` | 66.7% | **breached** |
| `individual` | 123 | yes | B | 61 | `high` | 63.9% | **breached** |
| `individual` | 777 | yes | A | 53 | `high` | 77.4% | **breached** |
| `individual` | 777 | yes | B | 44 | `high` | 59.1% | **breached** |
| `social` | 0 | yes | A | 61 | `high` | 59.0% | **breached** |
| `social` | 0 | yes | B | 49 | `high` | 81.6% | **breached** |
| `social` | 1 | yes | A | 61 | `high` | 70.5% | **breached** |
| `social` | 1 | yes | B | 61 | `high` | 75.4% | **breached** |
| `social` | 42 | yes | A | 60 | `high` | 68.3% | **breached** |
| `social` | 42 | yes | B | 61 | `high` | 63.9% | **breached** |
| `social` | 123 | yes | A | 56 | `high` | 58.9% | **breached** |
| `social` | 123 | yes | B | 61 | `high` | 67.2% | **breached** |
| `social` | 777 | yes | A | 56 | `high` | 69.6% | **breached** |
| `social` | 777 | yes | B | 60 | `high` | 68.3% | **breached** |

**Twenty-seven of the thirty evaluations breach the ceiling, and the requirement is unmet under all
three sources.** `REQ-MOK-060` binds "on every declared verification seed", so a single breaching
seed leaves it unmet; here fourteen of the fifteen runs carry at least one breaching territory. In
every one of the twenty-seven breaches the class over the line is `high`. The widest class's share
runs from 37.3% to 81.6%.

The three that meet it are worth naming rather than rounding away, because they are what tells the
owner how far a correction has to reach.

- **`reference` at seed 777, in both territories, at 45.9% and 46.2%.** The one run of the fifteen
  that meets the ceiling throughout. Both figures sit within four points of one half, so this is not
  a different regime — it is the same drift caught a little earlier.
- **`individual` at seed 1 in territory B**, and this is the more interesting row of the three: the
  only evaluation in the whole default-density set where the widest class is not `high`. Its `low`
  class leads at 37.3%, `high` follows at 35.6% and `medium` is last at 27.1%. A territory can sit
  well under the ceiling and still be nowhere near the balanced third it started from, which is
  precisely the case manual assessment 5's second half asks about: whether a per-class floor is
  wanted beside the ceiling.

### What manual assessment 5 is ratified against

`VER-MOK-016` records why one half was chosen over 60% and 40%: for "the `17` points of headroom
above the balanced initial third that it leaves". Those points can now be measured rather than
reasoned about, over the fifteen bound runs at the default density:

| Figure | Value |
|---|---|
| The `high` class's share of these fifteen runs at tick 0 | 32.8% |
| The same share at tick 1,000 | 64.1% |
| The headroom a ceiling of one half leaves above tick 0 | **17.2 points** |
| The drift these runs actually carry | **31.3 points** |

The measured headroom is 17.2 points, which is the decision's own figure of `17` reproduced from the
bytes — so the reasoning the value was chosen on holds exactly. What the runs carry is 31.3 points,
about 1.8 times that. The consequence for `WO-MOK-017` is a scoping one: a correction has to remove
roughly half of the drift, not shave a few percent off the top of it.

### The pre-change contrast, and the half of it that is not available

`VER-MOK-016` asks for the pre-change state "as the contrast", expecting it to put "high class above
half in a territory, which is the recorded 45 of 61 this requirement ends". Read directly from
`baseline/summary.txt` rather than inferred from §4's identity, it does: **17 of the 20** pre-change
evaluations breach the ceiling, and §7 finds the recorded 45 of 61 itself.

But the coverage row anticipated a contrast between two states — a pre-change commit above the
ceiling against a candidate below it — and under the descope there is no such contrast. All 20
verdicts are identical at both commits, because the candidate *is* the pre-change state on this
measurement. The row is answered on its pre-change half; its candidate half belongs to
`WO-MOK-017`. That is recorded here rather than presented as a contrast that was not measured.

## 7. The mechanism, measured

`REQ-MOK-060`'s rationale states a mechanism: a high-class resource restores `50` satiety, so a
non-waste condition makes it eatable only at satiety of at most `50`; high class is therefore passed
over more often than low or medium; what is passed over stays standing while regeneration keeps
adding classes uniformly. Each step of that is measurable here, and none of it is taken on the
rationale's word.

**The initial composition is a balanced third, and the territory total is what the density resolves
to.** Checked at every density, with the per-territory count checked against
`hundredths × 128 × 64 / 10000` truncated:

| Density | Per territory | A low/med/high | B low/med/high | Balanced within one |
|---|---|---|---|---|
| 0.15 | 12 | 4 / 4 / 4 | 4 / 4 / 4 | yes |
| 0.75 | 61 | 21 / 20 / 20 | 21 / 20 / 20 | yes |
| 1.50 | 122 | 41 / 41 / 40 | 41 / 41 / 40 | yes |

Rule 14 places resources before any source has acted, so this is a property of the world rather than
of the run — which is what makes it a legitimate baseline for the drift.

**What the sources ate, and how far the share moved**, over the five seeds of each row:

| Source | Density | Ticks reached | Initial high | Final high | low+medium eaten | high eaten |
|---|---|---|---|---|---|---|
| `baseline` | 0.15 | 119–119 | 33.3% | 33.3% | 0 | 0 |
| `baseline` | 0.75 | 119–168 | 32.8% | 32.8% | 1 | 2 |
| `baseline` | 1.50 | 119–193 | 32.8% | 33.0% | 9 | 5 |
| `reference` | 0.15 | 409–927 | 33.3% | 39.2% | 184 | 106 |
| `reference` | 0.75 | 1000–1000 | 32.8% | 63.7% | 1,426 | 403 |
| `reference` | 1.50 | 1000–1000 | 32.8% | 74.6% | 1,879 | 330 |
| `individual` | 0.15 | 379–757 | 33.3% | 28.3% | 189 | 84 |
| `individual` | 0.75 | 1000–1000 | 32.8% | 60.3% | 1,436 | 506 |
| `individual` | 1.50 | 1000–1000 | 32.8% | 69.7% | 1,923 | 355 |
| `social` | 0.15 | 393–1000 | 33.3% | 35.8% | 210 | 87 |
| `social` | 0.75 | 1000–1000 | 32.8% | 68.1% | 1,400 | 391 |
| `social` | 1.50 | 1000–1000 | 32.8% | 75.7% | 1,884 | 258 |

The two right-hand columns are the mechanism itself. Every source eats far more low and medium than
high — which is what a waste condition makes it do — and the share beside it drifts upward as the
consequence.

**The drift is a function of how long the run lasts, and the ticks column is what shows it.** At
`0.15` the world cannot carry twelve agents: every non-`baseline` row ends in extinction well before
tick 1,000, with the single exception of `social` at seed 0, which reaches the limit with one agent
left. There the high share moves by at most 5.8 points, and under `individual` it moves *downward*.
At the two higher densities every run reaches the tick limit and the same share moves by up to 42.9
points.

That is the reason `REQ-MOK-060`'s trigger is a 1,000-tick run at the default density rather than any
run at all. The composition it constrains is a property the world only develops when it lasts long
enough to regenerate many times over what a starving population takes — and it is also why the
requirement cannot be evaluated on `baseline`, which never gets there. §8 is that argument in full.

**`SPEC-MOK-001` rule 5's recorded measurement is reproduced here.** Rule 5 records "high class at
45 of 61 resources in a territory by tick 1,000, against a balanced initial third", and that sentence
is where `REQ-MOK-060` comes from. Of the 120 territory evaluations in this capture, exactly one
produces those two numbers:

```
reference   d0.75 seed 0    territory A   low   7  medium   9  high  45  of 61
```

It was looked for rather than assumed, and two things make the search worth reporting. It is the
default source at the default density on the first declared seed — the run a spec author would have
measured. And the pre-change capture holds that same cell with a summary identical to the
candidate's, checked in §4, so the figure rule 5 records is one this packet still retains the bytes
for. The requirement's starting point is not a remembered number.

## 8. `baseline` is outside the obligation, and the measurement says why it has to be

`REQ-MOK-060` excludes `baseline` by name. The stated reason is that it is the source this initiative
holds byte-identical, so it cannot be given a new obligation about the world it produces. The
measurement adds a second, independent reason.

| Density | Seed | Reason | Ticks | Resources eaten | Final composition = initial |
|---|---|---|---|---|---|
| 0.15 | 0 | `extinction` | 119 | 0 | yes |
| 0.15 | 1 | `extinction` | 119 | 0 | yes |
| 0.15 | 42 | `extinction` | 119 | 0 | yes |
| 0.15 | 123 | `extinction` | 119 | 0 | yes |
| 0.15 | 777 | `extinction` | 119 | 0 | yes |
| 0.75 | 0 | `extinction` | 119 | 0 | yes |
| 0.75 | 1 | `extinction` | 119 | 0 | yes |
| 0.75 | 42 | `extinction` | 142 | 1 | yes |
| 0.75 | 123 | `extinction` | 168 | 1 | yes |
| 0.75 | 777 | `extinction` | 134 | 1 | no |
| 1.50 | 0 | `extinction` | 193 | 6 | no |
| 1.50 | 1 | `extinction` | 134 | 1 | yes |
| 1.50 | 42 | `extinction` | 169 | 5 | yes |
| 1.50 | 123 | `extinction` | 119 | 0 | yes |
| 1.50 | 777 | `extinction` | 142 | 2 | no |

**`baseline` has no waste condition to correct.** `Simulation::observation` (`:2203`, building the
list at `:2253`) puts one `Eat` in the valid-action list for each co-located resource, with no waste
condition attached, and `BaselineDecisionSource::decide` (`:904`) takes a uniform index into that
list. The correction
`REQ-MOK-060` permits is a change to a source's waste condition, and there is nothing here to change.

**And every one of its fifteen runs ends in extinction between ticks 119 and 193**, so none of them
reaches the 1,000-tick trigger the requirement is written against. An obligation about the
composition at tick 1,000 would be vacuous for this source even if it were given one. The exclusion
is therefore not a convenience; the requirement would be unfalsifiable on this source.

In 12 of the 15 cells the final standing composition is exactly the composition rule 14 placed, and
where it differs the widest difference in any class is 2. `baseline` eats almost nothing — 6
resources at most across a whole run with twelve agents in it, because `eat` only enters the list
when an agent is standing on a resource and then competes with `wait`, `sleep` and up to four moves
for a single uniform draw — and rule 15 regenerates what was eaten with a class drawn the same way
rule 14 draws one. So a class can shift by a resource or two with no drift at all, and that is what
the three `no` rows are: not a composition drifting, but a resource replaced by one of another class.

## 9. Why the ceiling is per territory, measured

`REQ-MOK-060` is stated per territory rather than over the world because, in its own words, "a world
average would hide a single territory's drift". Whether it hides one here is a measurement, not an
assumption.

| Source | Seed | World high share | Worst territory | The A/B gap |
|---|---|---|---|---|
| `reference` | 0 | 66.4% | 73.8% | 15.6% |
| `reference` | 1 | 70.8% | 75.4% | 11.3% |
| `reference` | 42 | 62.7% | 63.4% | 1.1% |
| `reference` | 123 | 71.1% | 73.3% | 4.5% |
| `reference` | 777 | 46.0% | 46.2% | 0.3% |
| `individual` | 0 | 60.4% | 61.7% | 3.1% |
| `individual` | 1 | 52.9% | 70.0% | 32.7% |
| `individual` | 42 | 55.4% | 60.0% | 9.2% |
| `individual` | 123 | 65.3% | 66.7% | 2.7% |
| `individual` | 777 | 69.1% | 77.4% | 18.3% |
| `social` | 0 | 69.1% | 81.6% | 22.6% |
| `social` | 1 | 73.0% | 75.4% | 4.9% |
| `social` | 42 | 66.1% | 68.3% | 4.4% |
| `social` | 123 | 63.2% | 67.2% | 8.3% |
| `social` | 777 | 69.0% | 69.6% | 1.3% |

**At this candidate it hides nothing.** Fourteen of the fifteen runs carry a territory over one
half, and in all fourteen the world average is over one half too, so a world-average form of the
requirement would have caught exactly the same runs. The per-territory form is not what makes the
breach visible here, and saying so is more useful than presenting the clause as vindicated.

What the columns do show is that the clause is not idle. The two territories are far from equal —
the gap between their worst shares reaches 32.7% — so a corrected world sitting just under one half
on average could still carry one territory well above it, which is the case the clause exists for.
Its value shows *after* the correction rather than before it. That is a measured reason to keep the
clause in `WO-MOK-017`, not evidence that it is doing work at this candidate.

## 10. The survivor figures the same line carries

Rule 18 prints the survivor counts on the same line as the composition, so they are reported from
here rather than reconstructed somewhere else. At the default density, against the floors already
recorded elsewhere:

| Source | Seed | Survivors | Deaths | A | B | Floor | Against the floor |
|---|---|---|---|---|---|---|---|
| `reference` | 0 | 8 | 4 | 3 | 5 | 8 | at or above |
| `reference` | 1 | 11 | 1 | 4 | 7 | 8 | at or above |
| `reference` | 42 | 8 | 4 | 3 | 5 | 8 | at or above |
| `reference` | 123 | 9 | 3 | 4 | 5 | 8 | at or above |
| `reference` | 777 | 11 | 1 | 6 | 5 | 8 | at or above |
| `individual` | 0 | 11 | 1 | 6 | 5 | 8 | at or above |
| `individual` | 1 | 9 | 3 | 5 | 4 | 8 | at or above |
| `individual` | 42 | 9 | 3 | 4 | 5 | 8 | at or above |
| `individual` | 123 | 10 | 2 | 7 | 3 | 8 | at or above |
| `individual` | 777 | 12 | 0 | 8 | 4 | 8 | at or above |
| `social` | 0 | 9 | 3 | 3 | 6 | 5 | at or above |
| `social` | 1 | 10 | 2 | 4 | 6 | 5 | at or above |
| `social` | 42 | 9 | 3 | 6 | 3 | 5 | at or above |
| `social` | 123 | 9 | 3 | 4 | 5 | 5 | at or above |
| `social` | 777 | 11 | 1 | 6 | 5 | 5 | at or above |

`REQ-MOK-014` binds `reference` at eight of twelve at the default density, `REQ-MOK-034` binds the
trait-aware source at eight, and `REQ-MOK-058`'s floor of five for `social` was ratified on the
measured curve of 2026-08-20. Two of the fifteen rows sit exactly on their floor, both under
`reference`. No floor is adjusted to fit a measurement here and none is ratified here: `post/runs.md`
carries the per-seed outcome tables and `escalation.md` §10 records the `social` floor's
ratification.

The reason these belong in this file at all is that the ceiling and the floor pull against each
other. A correction that suppresses high-class hoarding makes more resources edible, which moves
survivor counts — so whoever sets the ceiling in `WO-MOK-017` is also moving the numbers in this
table, and both are on one line of one record.

## 11. What this record does not establish

- **It is not a verification verdict.** `VER-MOK-016` is the contract and `VREC-MOK-016` the record.
- **It does not establish that `REQ-MOK-060` is met, and measures that it is not.** The ceiling's
  value, the permitted mechanisms and the per-class-floor question are `WO-MOK-017`'s, with manual
  assessment 5 following the requirement. §6 is the curve that decision is taken on, not the
  decision.
- **It does not measure a corrected composition anywhere**, because no candidate here carries a
  correction. `VER-MOK-016`'s assessment 5 asks for the ratification to be against "the measured
  corrected composition", and that phrase has no subject at this candidate. It is recorded as a
  defect of fit between the assessment's wording and the descope rather than worked around.
- **It does not carry a pre-change side for `social`**, for the reason §4 gives, and it does not
  compare the two commits at greater strength than one line per cell. `post/byte-identity.txt` is
  the whole-stream comparison and is the stronger instrument.
- **It does not attribute the drift to any particular decision inside a source.** It counts what was
  eaten, by class. `post/branches.md` is where proposals are classified branch by branch.
- **It does not predict what a correction would produce.** Every figure here is from an uncorrected
  candidate, and a waste condition that changes which resources are eaten changes the stream from
  the tick it first applies — so nothing in §7's drift columns can be extrapolated into a corrected
  world. `WO-MOK-017` re-measures.
