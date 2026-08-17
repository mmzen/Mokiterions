# Determinism and resilience — WO-MOK-002

Measured 2026-08-17 against the committed tree, after the third `SPEC-MOK-001` amendment.

## Byte-identical replay

`REQ-MOK-009` requires byte-identical output for identical seed, configuration, and policy.
Each configuration below was run twice and the complete standard output hashed.

| Policy | Density | SHA-256 of full output | Replay |
|---|---:|---|---|
| reference | `0.75%` | `97e0581c7aa1725490238842dfb63fa330d544f20307f34594362ab15bd95d5d` | identical |
| reference | `1.50%` | `58b7edc12f4ea1eae91a03cfa37a48cadfece3f7c071f01b41435a349473a9a8` | identical |
| baseline | `0.75%` | `82aa98b34fdbd9b6b7e271c2d250146c322d2f79954c9dccdd1464c7e6b63609` | identical |
| baseline | `1.50%` | `85f052bbcccf68213f211d6ea01617b223ba86cf1fbb8b463f6b3264f46f076e` | identical |

All at seed `123`, 1,000 ticks. Every hash reproduces exactly on replay, and all four differ from
one another, which is the point: policy and density each select a different consumption pattern
from the shared entropy stream, so neither a source nor a density is a cosmetic setting. Density
is parsed as an integer count of hundredths of a percent, so no floating-point value participates
in the resolved resource count and no platform-dependent rounding can enter.

Output carries no timestamp, path, pointer, or hostname, which is what makes hashing whole runs a
meaningful comparison rather than an accident of the machine.

## Resilience at 10,000 ticks

Seed `123`, ten times the verified horizon. No run panicked, and every run emitted exactly one
summary.

| Policy | Density | Reason | Ticks | Survivors | Survivors + deaths |
|---|---:|---|---:|---:|---:|
| reference | `0.75%` | extinction | 9,154 | 0 | 12 |
| reference | `1.50%` | tick limit | 10,000 | 7 | 12 |
| baseline | `0.75%` | extinction | 168 | 0 | 12 |
| baseline | `1.50%` | extinction | 119 | 0 | 12 |

Conservation holds in every case: survivors plus deaths equal twelve. Extinction terminates
cleanly and takes precedence over the tick limit, as rule 17 requires. The baseline source
starving is expected and carries no obligation; it starves faster at the higher density because a
denser world lets it eat itself into a worse position early rather than because density hurts.

A 10,000-tick reference run at `1.50%` completes in **0.28 s**, so raising the resource count from
12 per territory under the superseded constant to 122 is measurable but immaterial at these
sizes. Per-tick work stays bounded by twelve agents, the resource collection, perception within a
radius of 16, and emitted events.

## The long-horizon result, stated plainly

**The reference source at the declared density reaches extinction at tick 9,154.** Under the
previous rule the same configuration left one survivor at tick 10,000. The corrected rule is
better at the horizon `REQ-MOK-014` verifies and worse in the tail, and both halves of that
sentence are load-bearing.

Survivors and resource mix for seed `123` at the default density, sampled by running to each
horizon:

| Horizon | Survivors | A low / medium / high | B low / medium / high |
|---:|---:|---|---|
| 1,000 | 9 | 8 / 11 / 42 | 4 / 12 / 44 |
| 2,000 | 8 | 11 / 9 / 41 | 6 / 10 / 45 |
| 3,000 | 5 | 9 / 7 / 45 | 9 / 9 / 43 |
| 5,000 | 4 | 3 / 9 / 49 | 6 / 8 / 47 |
| 7,000 | 3 | 8 / 14 / 39 | 14 / 5 / 42 |
| 9,000 | 1 | 5 / 10 / 46 | 12 / 4 / 45 |
| 10,000 | 0 at tick 9,154 | 5 / 11 / 45 | 12 / 4 / 45 |

The mechanism is visible in the table and is not a mystery. Both territories stay at or near
capacity — 61 resources — for the whole run, so the world never runs out of food. What it runs
out of is food anyone will approach. High class settles at roughly three quarters of standing
supply and is worth approaching only at satiety of at most `50`, so the effective supply of
low- and medium-class resources is a fraction of the nominal density. The population declines
against a full larder.

The same effect at `1.50%` leaves 103 and 104 high-class resources of 122 per territory, yet
seven Mokiterions survive to 10,000 ticks: the absolute quantity of low and medium class still
regenerating is large enough to sustain a reduced population. Density buys throughput, not mix.

This is disclosed, accepted, and deferred. `REQ-MOK-014` states a floor at tick 1,000 and makes
no claim about a steady state. `VER-MOK-002` records it as residual uncertainty and notes that a
long-horizon stability requirement will need a rule change that lets high-class resources be
consumed, which will move the density curve and require re-approval.

## Bounded work

Perception is evaluated per agent against the resource collection, so per-tick cost is
O(agents × resources). At twelve agents and 122 resources that is 1,464 distance evaluations per
tick, and at `1.50%` it is 2,928. There is no unbounded allocation, no recursion, and no `unsafe`
block in the crate. The measured timings above are the evidence that this is acceptable at the
scale `SPEC-MOK-001` fixes; a substantially larger world or population would need this revisited
rather than assumed.
