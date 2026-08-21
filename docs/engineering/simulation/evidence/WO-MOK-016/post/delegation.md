# Oracle 8's delegation equality: the precondition, the parting, and the draw count per run

| Field | Value |
|---|---|
| Retention item | "the per-observation comparison of `social` against `individual` at every opportunity where no living Mokiterion is perceived and no attack is unanswered, carrying both proposals and the shared stream's position either side, together with the total draw count per run under each source at matched seeds" |
| Oracle | `VER-MOK-016` oracle 8, in both the narrow and the widened form, and oracle 8's stated consequence that a `social` run takes fewer draws than an `individual` run at the same seed; `REQ-MOK-057` |
| Reader | `analysis/delegation.py`, over 120 cells — the 90-cell three-source matrix and the 30 `social` cells |
| Exit code | **`0`** — 30 checked properties, each holding at every cell it applies to |
| Raw output | `post/delegation.txt` |
| Capture | `git archive 7c4aef3967406c05d80da963695898b77f5329e9` (90 cells) and `git archive 59d61b915630fd55f04bcdbb346aa22cdbfdfff6` (30 `social` cells), every cell digest-matched against `post/post-manifest.txt` or `post/social-manifest.txt` |
| Date | 2026-08-21 |

---

## 1. Three clauses, three depths, and no temporary build

The retention item asks for three things at once, and the released streams answer them to three
different depths. This file says which is which rather than presenting one verdict.

- **The precondition is stream-visible, exactly.** Neither half looks visible at first: an
  observation's perceived-Mokiterion list is never emitted, and neither is the suffered-attack
  record on its own. Both come out anyway, and §2 is how.
- **Both proposals at one observation are visible only up to the parting.** The two sources part
  inside the first tick at every matched pair, and past it they are two worlds rather than two
  sources, so a per-opportunity comparison between them is not defined there. §5 carries the
  agreement before the parting, the parting itself with both proposals, and the stream's position
  either side of it.
- **The total draw count per run is nowhere reported, and is recovered here.** The engine prints no
  stream position and no test records a whole-run count: `shared_stream_draws` is a test helper, and
  at both of its call sites it reads a freshly initialized `Simulation` and compares it against
  `INITIALIZATION_DRAWS`. So §§6 and 7 reconstruct the total from the stream, and §8 checks the
  reconstruction against two counts arrived at without it.

No temporary build, for the reason `branches.md` §1 gives: a figure produced by a build that no
longer exists is not re-derivable by a verifier. No test is added either, because the workspace
census is fixed at `test-census-reconciliation.md`'s figures and an added test would move them.

The placement replay is copied from `analysis/entropy.py` and re-checked here against
`INITIALIZATION_DRAWS` rather than trusted, because everything below counts forward from where a
run starts. All fifteen declared pairs reproduce their recorded count — 72, 268, 270, 514 and 516
— and the state column of `entropy.txt` §3 is recomputed beside it.

## 2. The precondition decodes from the released bytes

**Rule 12's `fear` pair carries the perceived-Mokiterion half.** `run_tick` reads
`perceived_company` from the same observation the decision was taken on and carries it past the
decision; `apply_survival` then prints `fear` either side of the update. So the pair says what the
observation held:

| the pair | what the observation held |
|---|---|
| `fear` rises by `10`, or holds at `100` | a Mokiterion was perceived |
| `fear` falls by `5`, or holds at `0` | none was |

The equal case is not an ambiguity: `fear` is unchanged only where the update saturates, and the
two saturations are at opposite ends. Rule 23's threat increase of `30` cannot confuse it either,
because the pair is read inside one call and any threat this tick landed before it, in the value on
the left.

**The decode is exhaustive over opportunities**, which is what makes a census from it a census and
not a sample: `apply_survival` emits unconditionally and `run_tick` calls it for every agent it
consults, with no path between the two, so there is exactly one `survival_changed` record per
opportunity and, in a traced cell, exactly one `action_trace` beside it. §9 checks those two counts
against each other in all 120 cells and finds 60 pairs agreeing and 60 untraced cells at zero.

**The record's half needs no reconstruction.** Rule 17 renders the trace's `suffered` field only
when the record is non-empty, so its absence *is* the empty record.

## 3. The census, fifteen traced `social` cells, whole runs

| cell | opportunities | company | record | precondition holds | targeted there |
|---|---:|---:|---:|---:|---:|
| `seed0-d0.15` | 4,408 | 862 | 4 | 3,546 | 0 |
| `seed1-d0.15` | 2,507 | 525 | 4 | 1,982 | 0 |
| `seed42-d0.15` | 2,668 | 808 | 11 | 1,860 | 0 |
| `seed123-d0.15` | 2,881 | 1,076 | 3 | 1,805 | 0 |
| `seed777-d0.15` | 2,498 | 396 | 13 | 2,102 | 0 |
| `seed0-d0.75` | 10,847 | 3,649 | 7 | 7,198 | 0 |
| `seed1-d0.75` | 10,019 | 2,443 | 12 | 7,576 | 0 |
| `seed42-d0.75` | 9,279 | 2,281 | 15 | 6,998 | 0 |
| `seed123-d0.75` | 9,021 | 2,384 | 18 | 6,637 | 0 |
| `seed777-d0.75` | 11,009 | 4,114 | 7 | 6,895 | 0 |
| `seed0-d1.50` | 11,009 | 3,244 | 4 | 7,765 | 0 |
| `seed1-d1.50` | 10,018 | 2,738 | 9 | 7,280 | 0 |
| `seed42-d1.50` | 11,009 | 3,359 | 6 | 7,650 | 0 |
| `seed123-d1.50` | 11,008 | 3,681 | 12 | 7,327 | 0 |
| `seed777-d1.50` | 10,020 | 2,984 | 10 | 7,036 | 0 |
| **all fifteen** | **118,201** | **34,544** | **135** | **83,657** | **0** |

**The narrow precondition holds at 70.8% of all opportunities**, so the equality oracle 8 buys is
asserted over most of the matrix rather than a corner of it. Two readings follow from the same
table. The first is arithmetic: `holds` and `company` sum to `opportunities` in every row, and the
precondition is the conjunction of no company and no record, so **no opportunity in these fifteen
cells carried a record without carrying company too**. Rule 26 does not require that — rule 20's
contact distance of `1` inside a symmetric perception radius of `16` is what makes it come out that
way — and this reader observes it rather than checking it against the engine. The
second reading: the proposals at those 83,657 opportunities are rule 19's vocabulary and nothing
else —
**move 78,331, eat 2,927, sleep 2,399, and no targeted verb at any of them** — which is what
delegation to rule 19 predicts, checked rather than assumed.

**The widened form's population is larger and is not decoded here.** The amendment of 2026-08-20
widened the equality to any observation where a tolerated resource is perceived and the record is
empty, whether or not a Mokiterion is perceived. That precondition is a predicate over
reconstructed positions and the acting Mokiterion's own waste tolerance rather than a rendered
field, so this reader does not decode it — and it does not need to, because rule 26's branch 3 is
reached only where a tolerated resource is perceived and branch 1 did not pre-empt:

| | fifteen traced cells |
|---|---:|
| branch 3, from `branches.md` §2 | 58,441 of 118,201 decisions |
| the narrow precondition, from the table above | 83,657 of 118,201 |

## 4. The decode's three checks, each from a direction the decode does not use

| check | what it would catch | result |
|---|---|---:|
| targeted proposals at an opportunity with neither company nor a record | a `fear` pair read the wrong way round, or a driver taken from the wrong observation | **0** |
| resolutions at an opportunity the decode says had no company | the same error where it would change a resolution rather than a proposal | **0** |
| the `,suffered:` census against `branches.md`'s branch-1 count, cell for cell | either reader's reconstruction, from opposite directions | **0 disagreements** |

The third is the strongest, because the number it is checked against is another reader's.
`branches.md` reconstructed rule 26's branch for every decision from positions, traits and
geometry; this reader counts a rendered field. Two properties, two readers, one number:

| cell | `,suffered:` rendered | `branches.md` branch 1 |
|---|---:|---:|
| `seed0-d0.15` | 4 | 4 |
| `seed1-d0.15` | 4 | 4 |
| `seed42-d0.15` | 11 | 11 |
| `seed123-d0.15` | 3 | 3 |
| `seed777-d0.15` | 13 | 13 |
| `seed0-d0.75` | 7 | 7 |
| `seed1-d0.75` | 12 | 12 |
| `seed42-d0.75` | 15 | 15 |
| `seed123-d0.75` | 18 | 18 |
| `seed777-d0.75` | 7 | 7 |
| `seed0-d1.50` | 4 | 4 |
| `seed1-d1.50` | 9 | 9 |
| `seed42-d1.50` | 6 | 6 |
| `seed123-d1.50` | 12 | 12 |
| `seed777-d1.50` | 10 | 10 |
| **all fifteen** | **135** | **135** |

## 5. The parting, all fifteen matched pairs, with the stream's position either side

| cell | agreed | tick | subject | `social` proposal | `individual` proposal |
|---|---:|---:|---|---|---|
| `seed0-d0.15` | 0 | 1 | `M01` | `approach -> M06` | `move:north` |
| `seed1-d0.15` | 9 | 1 | `M10` | `approach -> M09` | `move:east` |
| `seed42-d0.15` | 1 | 1 | `M02` | `approach -> M04` | `move:north` |
| `seed123-d0.15` | 0 | 1 | `M01` | `approach -> M02` | `move:east` |
| `seed777-d0.15` | 0 | 1 | `M01` | `approach -> M03` | `move:north` |
| `seed0-d0.75` | 0 | 1 | `M01` | `approach -> M04` | `move:west` |
| `seed1-d0.75` | 1 | 1 | `M02` | `approach -> M01` | `move:east` |
| `seed42-d0.75` | 1 | 1 | `M02` | `approach -> M03` | `move:north` |
| `seed123-d0.75` | 0 | 1 | `M01` | `approach -> M03` | `move:south` |
| `seed777-d0.75` | 3 | 1 | `M04` | `approach -> M06` | `move:north` |
| `seed0-d1.50` | 2 | 1 | `M03` | `approach -> M06` | `move:east` |
| `seed1-d1.50` | 0 | 1 | `M01` | `approach -> M02` | `move:west` |
| `seed42-d1.50` | 2 | 1 | `M03` | `approach -> M04` | `move:south` |
| `seed123-d1.50` | 1 | 1 | `M02` | `approach -> M03` | `move:south` |
| `seed777-d1.50` | 1 | 1 | `M02` | `approach -> M06` | `move:north` |

**Every parting is inside tick 1, and every one is where oracle 8 says it will be.** Oracle 8 has
the two sources diverging "at the first opportunity where one Mokiterion perceives another and no
tolerated resource is perceived", and both halves are readable off the two proposals: `approach`
and `avoid` are branch 5's verbs and no rule-19 case proposes either, so `social`'s proposal says a
Mokiterion was perceived; and branch 5 is reached only past branch 3, so the same proposal says no
tolerated resource was. `individual`, on the identical observation, fell through rule 19's cases 1
to 3 to case 4's random step, which is the plain move it printed. Fifteen pairs, fifteen partings,
every one of that shape, and none of it asserted anywhere else in the packet. The 21 proposals in
the agreed prefixes are all plain moves, which is the equality holding for exactly as long as it is
defined to.

**The stream's position either side follows from the same two proposals.** Branch 5 draws nothing —
`entropy.txt` §7 shows each of branches 1 to 5 returning before the source's single `entropy` use —
and rule 19's case 4 draws exactly once. So at the parting the `social` stream stands still and the
`individual` stream advances by one value, from the position below:

| cell | agreed | plain moves | the stream's position before the parting | |
|---|---:|---:|---|---|
| `seed0-d0.15` | 0 | 0 | `0x7F9A3C2BCCF2E5E8` | exact |
| `seed1-d0.15` | 9 | 9 | `0x7F9A3C2BCCF2E5E9 .. 0x0F8D83B1469142A6` | 10 candidates |
| `seed42-d0.15` | 1 | 1 | `0x7F9A3C2BCCF2E612 .. 0x1DD1B5E54C3D6227` | 2 candidates |
| `seed123-d0.15` | 0 | 0 | `0x7F9A3C2BCCF2E663` | exact |
| `seed777-d0.15` | 0 | 0 | `0x7F9A3C2BCCF2E8F1` | exact |
| `seed0-d0.75` | 0 | 0 | `0xDE8261A4408EDE26` | exact |
| `seed1-d0.75` | 1 | 1 | `0xA2136E3141F9E5FD .. 0x404AE7EAC1446212` | 2 candidates |
| `seed42-d0.75` | 1 | 1 | `0xA2136E3141F9E626 .. 0x404AE7EAC144623B` | 2 candidates |
| `seed123-d0.75` | 0 | 0 | `0xA2136E3141F9E677` | exact |
| `seed777-d0.75` | 3 | 3 | `0xA2136E3141F9E905 .. 0x7CB9DB5DBFD95D44` | 4 candidates |
| `seed0-d1.50` | 2 | 2 | `0xE7D159E492221A54 .. 0x24404D5790B7127E` | 3 candidates |
| `seed1-d1.50` | 0 | 0 | `0xE7D159E492221A55` | exact |
| `seed42-d1.50` | 2 | 2 | `0xE7D159E492221A7E .. 0x24404D5790B712A8` | 3 candidates |
| `seed123-d1.50` | 1 | 1 | `0xAB626671938D22A5 .. 0x4999E02B12D79EBA` | 2 candidates |
| `seed777-d1.50` | 1 | 1 | `0xE7D159E492221D5D .. 0x8608D39E116C9972` | 2 candidates |

Where the agreed prefix is empty, the position is §1's post-initialization state exactly — 6 of the
15 pairs. Where it carries plain moves, each of them is either rule 19's case 3, a step toward a
tolerated resource that draws nothing, or its case 4, which draws; the released line does not
distinguish the two, so the position is given as the window they span rather than asserted.
`branches.md`'s reconstruction is what separates cases 3 and 4, and this file does not duplicate
it.

## 6. The draw accounting, and what pins it

Rule 16 regenerates on a fixed interval and **prints what it drew** — the class, and the coordinate
it settled on. `choose_free_coordinate` retries on an occupied cell, and the cells occupied at that
moment are reconstructible, because every food's position arrives in a `food_initialized` or
`food_regenerated` record and leaves in a `food_consumed` one. So a candidate stream position either
reproduces the printed class and coordinate or it does not, and the accounting is pinned at every
addition. Between two pins the only other consumer is the decision source, at no more than one
value per opportunity, and every opportunity emits one `survival_changed` record — so the number of
values in between is bounded above by a count the stream gives, and is solved for rather than
assumed. A skipped regeneration draws nothing at all, `regenerate_food` returning before its first
draw, so it pins nothing either.

**§7 reports the run's last addition, always, and never an earlier one.** That is what makes its
tail a bound rather than an estimate: the opportunities after the reported point are worth at most
one value each, and no addition follows to spend three or more without being counted. Reporting an
earlier addition with the same tail — the last one whose survivors all agreed on the total, say —
would understate the position instead of widening it.

Two of the bounds are exact rather than probabilistic. The coordinate bounds `128` and `64` divide
2^64, so `choose_index` never rejects for modulo bias there and a coordinate attempt costs exactly
two values; `entropy.txt` §1 derives that and gives the control. The class bound `3` does not
divide 2^64 — its rejection threshold is `1` — so a class draw could in principle cost more, and so
could a decision at a bound that does not divide either. **Neither is assumed away.** A rejection
anywhere would push the true consumption past the bound the solver searches, and the pin would then
find no consistent position at all. That is the `stream lost` count of §7 reading **0 of 60**: a
detection rather than a silent wrong figure.

## 7. The draw total per run, sixty cells, four sources at matched seeds

`initial` is initialization, replayed exactly. `total` is the stream's absolute position at the
run's last addition, `regen` the part of it regeneration spent and `decisions` the part the decision
source spent. `tail` is the opportunities after that addition, each worth at most one value and none
of them an addition, so the run's own total is `total` plus something in `0..=tail`. A figure shown
as a range is a set of whole hypotheses the evidence does not separate; where only `regen` and
`decisions` are ranged, the total beside them is still exact.

| source | density | seed | opps | initial | decisions | regen | total | tail | last addition |
|---|---|---:|---:|---:|---:|---:|---:|---:|---|
| `baseline` | `0.15` | 0 | 1,428 | 72 | 0 | 0 | 72 | 1,428 | initialization |
| `baseline` | `0.15` | 1 | 1,428 | 72 | 0 | 0 | 72 | 1,428 | initialization |
| `baseline` | `0.15` | 42 | 1,428 | 72 | 0 | 0 | 72 | 1,428 | initialization |
| `baseline` | `0.15` | 123 | 1,428 | 72 | 0 | 0 | 72 | 1,428 | initialization |
| `baseline` | `0.15` | 777 | 1,428 | 72 | 0 | 0 | 72 | 1,428 | initialization |
| `baseline` | `0.75` | 0 | 1,428 | 270 | 0 | 0 | 270 | 1,428 | initialization |
| `baseline` | `0.75` | 1 | 1,428 | 268 | 0 | 0 | 268 | 1,428 | initialization |
| `baseline` | `0.75` | 42 | 1,451 | 268 | 360 | 3 | 631 | 1,091 | tick 30 |
| `baseline` | `0.75` | 123 | 1,477 | 268 | 1,320 | 3 | 1,591 | 157 | tick 110 |
| `baseline` | `0.75` | 777 | 1,443 | 268 | 576-1,080 | 3 | 847-1,351 | 363 | tick 90 |
| `baseline` | `1.50` | 0 | 1,576 | 516 | 1,539 | 18 | 2,073 | 37 | tick 160 |
| `baseline` | `1.50` | 1 | 1,443 | 516 | 1,080 | 3 | 1,599 | 363 | tick 90 |
| `baseline` | `1.50` | 42 | 1,598 | 516 | 960 | 15 | 1,491 | 638 | tick 80 |
| `baseline` | `1.50` | 123 | 1,428 | 514 | 0 | 0 | 514 | 1,428 | initialization |
| `baseline` | `1.50` | 777 | 1,459 | 516 | 360 | 6 | 882 | 1,099 | tick 30 |
| `reference` | `0.15` | 0 | 2,342 | 72 | 1,637 | 96 | 1,805 | 109 | tick 300 |
| `reference` | `0.15` | 1 | 3,504 | 72 | 1,933 | 207 | 2,212 | 109 | tick 600 |
| `reference` | `0.15` | 42 | 2,992 | 72 | 1,861 | 153 | 2,086 | 99 | tick 470 |
| `reference` | `0.15` | 123 | 3,171 | 72 | 1,895 | 162 | 2,129 | 79 | tick 390 |
| `reference` | `0.15` | 777 | 4,438 | 72 | 2,606 | 252 | 2,930 | 87 | tick 840 |
| `reference` | `0.75` | 0 | 10,990 | 270 | 4,470 | 1,123 | 5,863 | 0 | tick 1000 |
| `reference` | `0.75` | 1 | 11,824 | 268 | 4,839 | 1,154 | 6,261 | 0 | tick 1000 |
| `reference` | `0.75` | 42 | 10,363 | 268 | 4,482-4,484 | 974-976 | 5,726 | 0 | tick 1000 |
| `reference` | `0.75` | 123 | 9,765 | 268 | 4,272 | 971 | 5,511 | 0 | tick 1000 |
| `reference` | `0.75` | 777 | 11,474 | 268 | 5,182-5,186 | 1,052-1,056 | 6,506 | 0 | tick 1000 |
| `reference` | `1.50` | 0 | 11,884 | 516 | 5,255 | 1,085 | 6,856 | 0 | tick 1000 |
| `reference` | `1.50` | 1 | 12,000 | 516 | 5,000-5,002 | 1,195-1,197 | 6,713 | 0 | tick 1000 |
| `reference` | `1.50` | 42 | 12,000 | 516 | 5,108-5,110 | 1,187-1,189 | 6,813 | 0 | tick 1000 |
| `reference` | `1.50` | 123 | 11,828 | 514 | 5,021-5,023 | 1,182-1,184 | 6,719 | 0 | tick 1000 |
| `reference` | `1.50` | 777 | 12,000 | 516 | 4,692-4,694 | 1,190-1,192 | 6,400 | 0 | tick 1000 |
| `individual` | `0.15` | 0 | 3,237 | 72 | 1,910 | 183 | 2,165 | 94 | tick 580 |
| `individual` | `0.15` | 1 | 3,830 | 72 | 1,893 | 246 | 2,211 | 87 | tick 670 |
| `individual` | `0.15` | 42 | 3,168 | 72 | 1,715 | 171 | 1,958 | 118 | tick 400 |
| `individual` | `0.15` | 123 | 2,755 | 72 | 1,706 | 132 | 1,910 | 113 | tick 350 |
| `individual` | `0.15` | 777 | 2,400 | 72 | 1,502 | 87 | 1,661 | 198 | tick 280 |
| `individual` | `0.75` | 0 | 11,924 | 270 | 5,418 | 1,139 | 6,827 | 0 | tick 1000 |
| `individual` | `0.75` | 1 | 11,018 | 268 | 4,660 | 1,097 | 6,025 | 0 | tick 1000 |
| `individual` | `0.75` | 42 | 11,067 | 268 | 4,768 | 1,139 | 6,175 | 0 | tick 1000 |
| `individual` | `0.75` | 123 | 11,179 | 268 | 4,705 | 1,136 | 6,109 | 0 | tick 1000 |
| `individual` | `0.75` | 777 | 12,000 | 268 | 4,749 | 1,184 | 6,201 | 0 | tick 1000 |
| `individual` | `1.50` | 0 | 12,000 | 516 | 5,033-5,035 | 1,185-1,187 | 6,736 | 0 | tick 1000 |
| `individual` | `1.50` | 1 | 12,000 | 516 | 4,940-4,942 | 1,196-1,198 | 6,654 | 0 | tick 1000 |
| `individual` | `1.50` | 42 | 12,000 | 516 | 5,050-5,052 | 1,192-1,194 | 6,760 | 0 | tick 1000 |
| `individual` | `1.50` | 123 | 11,940 | 514 | 4,978 | 1,186 | 6,678 | 0 | tick 1000 |
| `individual` | `1.50` | 777 | 11,445 | 516 | 4,949 | 1,189 | 6,654 | 0 | tick 1000 |
| `social` | `0.15` | 0 | 4,408 | 72 | 1,698 | 333 | 2,103 | 40 | tick 960 |
| `social` | `0.15` | 1 | 2,507 | 72 | 1,209 | 126 | 1,407 | 151 | tick 310 |
| `social` | `0.15` | 42 | 2,668 | 72 | 1,173 | 135 | 1,380 | 73 | tick 510 |
| `social` | `0.15` | 123 | 2,881 | 72 | 952 | 165 | 1,189 | 136 | tick 440 |
| `social` | `0.15` | 777 | 2,498 | 72 | 1,204 | 132 | 1,408 | 171 | tick 340 |
| `social` | `0.75` | 0 | 10,847 | 270 | 3,197 | 1,073 | 4,540 | 0 | tick 1000 |
| `social` | `0.75` | 1 | 10,019 | 268 | 3,074-3,076 | 1,113-1,115 | 4,457 | 0 | tick 1000 |
| `social` | `0.75` | 42 | 9,279 | 268 | 3,102-3,108 | 1,011-1,017 | 4,387 | 0 | tick 1000 |
| `social` | `0.75` | 123 | 9,021 | 268 | 3,055-3,057 | 965-967 | 4,290 | 0 | tick 1000 |
| `social` | `0.75` | 777 | 11,009 | 268 | 3,061-3,063 | 1,149-1,151 | 4,480 | 0 | tick 1000 |
| `social` | `1.50` | 0 | 11,009 | 516 | 3,468-3,472 | 1,193-1,197 | 5,181 | 0 | tick 1000 |
| `social` | `1.50` | 1 | 10,018 | 516 | 3,196-3,198 | 1,187-1,189 | 4,901 | 0 | tick 1000 |
| `social` | `1.50` | 42 | 11,009 | 516 | 3,219-3,223 | 1,201-1,205 | 4,940 | 0 | tick 1000 |
| `social` | `1.50` | 123 | 11,008 | 514 | 2,934-2,936 | 1,208-1,210 | 4,658 | 0 | tick 1000 |
| `social` | `1.50` | 777 | 10,020 | 516 | 2,933 | 1,171 | 4,620 | 0 | tick 1000 |

| | |
|---|---:|
| cells whose total is exact for the whole run | **30 of 60** |
| cells whose decision/regeneration split is exact | 43 of 60 |
| cells whose total is open at the last addition | 1 of 60 |
| cells reaching no addition at all | 8 of 60 |
| cells where the stream was lost | **0 of 60** |
| additions leaving two totals open | 20 |
| additions leaving only the split open | 4,139 |
| widest candidate set at any addition | 6 |

Four readings, and three of them are limitations rather than findings:

- **The 30 cells exact for the whole run are exactly the thirty at density `0.75` or `1.50` under
  `reference`, `individual` and `social`.** Their last regeneration is at tick 1,000 and
  regeneration is the last thing a tick does, so nothing follows it to be uncertain about, and for
  those cells the retention item's third clause is discharged with no residual at all. The other
  thirty are the twenty sparse cells and the ten remaining `baseline` cells, for the same reason in
  both: regeneration is what pins the stream, and it is scarce where resources are scarce and where
  nothing eats them. §8's second control closes the `social` cells among those thirty anyway.
- **8 cells reach no addition at all, and every one is `baseline`.** Regeneration is skipped while a
  territory is at capacity, and rule 4 selects uniformly rather than seeking food, so nothing is
  eaten, no cell is freed and nothing is added. Those runs end by **extinction at tick 119**, not at
  the tick limit — `step` stops at either and the event says which — so the whole regeneration
  record of each is 22 skips and no addition. For them this file reports initialization and a tail
  the width of the run and claims nothing further; `delegation.txt` §8 names all eight rather than
  dropping them.
- **An open split is not an open total.** Two positions that print the same class at the same
  coordinate agree on where the stream ends and disagree only on which consumer spent which part of
  it. Nothing printed later separates them, so the split stays open for the rest of the run while
  the total does not — 4,139 additions of that kind, against 43 of the 60 cells still ending with
  an exact split.
- **An open *total* closes at the next addition, where there is one.** A position that survives one
  addition by coincidence has to survive the next one too, so the totals converge and the cost is
  tail width rather than accuracy: 20 additions left two totals open and **one cell** carries one to
  the end, `baseline` at `0.75` seed 777, whose single addition is the ambiguous one. Its two
  hypotheses are 847 with 576 decisions and 1,351 with 1,080, and §8's first control selects
  between them.

## 8. Three controls, and one of them is the strongest thing in this file

**Rule 4's known count.** `baseline` draws exactly one value per opportunity, unconditionally, so
its decision count is its opportunity count — known without any solving, and the solver is not told
it. Where the two disagree the solver is wrong:

| density | seed | opportunities to the pin | recovered decisions | agree |
|---|---:|---:|---:|---|
| `0.75` | 42 | 360 | 360 | yes |
| `0.75` | 123 | 1,320 | 1,320 | yes |
| `0.75` | 777 | 1,080 | 576-1,080 | yes |
| `1.50` | 0 | 1,539 | 1,539 | yes |
| `1.50` | 1 | 1,080 | 1,080 | yes |
| `1.50` | 42 | 960 | 960 | yes |
| `1.50` | 777 | 360 | 360 | yes |

Seven of the fifteen `baseline` cells carry a pin at all; **7 of 7 agree and 6 of 7 recover the
known count with no slack whatever**, over hundreds of opportunities and dozens of additions, from
a stream that reports neither quantity. The seventh is §7's open-total cell, and there the control
does more than agree: the known count is one of the two hypotheses and not the other, so it selects
between them, and a solver wrong about the tail would have left it matching neither.

**Branch 6's independently measured count.** `entropy.txt` §7 enumerates every draw site in the
shipped engine and shows that the only one a `social` decision can reach is rule 19's case 4,
through branch 6 — each of branches 1 to 5 returns before the source's single `entropy` use. So
under `social` the decision source's whole-run draw count **is** rule 26's branch-6 count, and that
count already exists, measured by an instrument that never touched the stream:

| density | seed | recovered decision draws | `branches.md` branch 6 | agrees |
|---|---:|---:|---:|---|
| `0.15` | 0 | 1,698..1,738 | 1,726 | yes |
| `0.15` | 1 | 1,209..1,360 | 1,312 | yes |
| `0.15` | 42 | 1,173..1,246 | 1,243 | yes |
| `0.15` | 123 | 952..1,088 | 1,082 | yes |
| `0.15` | 777 | 1,204..1,375 | 1,365 | yes |
| `0.75` | 0 | 3,197 | 3,197 | yes |
| `0.75` | 1 | 3,074..3,076 | 3,076 | yes |
| `0.75` | 42 | 3,102..3,108 | 3,102 | yes |
| `0.75` | 123 | 3,055..3,057 | 3,057 | yes |
| `0.75` | 777 | 3,061..3,063 | 3,061 | yes |
| `1.50` | 0 | 3,468..3,472 | 3,468 | yes |
| `1.50` | 1 | 3,196..3,198 | 3,198 | yes |
| `1.50` | 42 | 3,219..3,223 | 3,221 | yes |
| `1.50` | 123 | 2,934..2,936 | 2,934 | yes |
| `1.50` | 777 | 2,933 | 2,933 | yes |

**15 of 15 inside the recovered interval, 2 of them one number against one number with no interval
at all.** This is the check that makes the accounting evidence rather than arithmetic: the solver
could have been wrong in a way no internal consistency would show — a miscounted opportunity, a
draw site missed, a rejection unaccounted for — and every one of those would move these figures
away from a number computed without the stream. None of them do.

It runs the other way too, and that is worth stating plainly: **the agreement is a check on
`branches.md`'s branch-3 and branch-6 split**, which is the one part of that reconstruction no
rendered field distinguishes and which its §1 rests on thirteen checks against the engine. It now
rests on the entropy stream as well. And it closes this file's intervals: where §7 reports a range,
branch 6's count is a single number inside it, so the run's decision count is that number and the
regeneration figure beside it is the total less that number less initialization.

**The trace setting.** `--trace-actions` adds a record and must move nothing else, so the two cells
of a `(seed, density, source)` triple are accounted independently and must reach the same position
from the same opportunity count: **60 pairs compared, 60 agreeing** on total, opportunities and
additions. This is also where §2's exhaustiveness claim is measured rather than argued — the traced
cell's `action_trace` count against its `survival_changed` count, and the untraced cell's against
zero, in all 120 cells.

## 9. Oracle 8's stated consequence: `social` takes fewer draws than `individual`

Oracle 8 states that because branches 4 and 5 pre-empt rule 19's case 4, "a `social` run takes
**fewer** draws than an `individual` run on the same seed". The two runs are different worlds past
the parting, so this is a comparison of totals and not of positions — which is how oracle 8 states
it, and why. Where a tail is open the comparison is made on the whole interval and reads as holding
only if the whole interval holds it:

| density | seed | `social` total | `individual` total | `social` fewer |
|---|---:|---:|---:|---|
| `0.15` | 0 | 2,103..2,143 | 2,165..2,259 | yes |
| `0.15` | 1 | 1,407..1,558 | 2,211..2,298 | yes |
| `0.15` | 42 | 1,380..1,453 | 1,958..2,076 | yes |
| `0.15` | 123 | 1,189..1,325 | 1,910..2,023 | yes |
| `0.15` | 777 | 1,408..1,579 | 1,661..1,859 | yes |
| `0.75` | 0 | 4,540 | 6,827 | yes |
| `0.75` | 1 | 4,457 | 6,025 | yes |
| `0.75` | 42 | 4,387 | 6,175 | yes |
| `0.75` | 123 | 4,290 | 6,109 | yes |
| `0.75` | 777 | 4,480 | 6,201 | yes |
| `1.50` | 0 | 5,181 | 6,736 | yes |
| `1.50` | 1 | 4,901 | 6,654 | yes |
| `1.50` | 42 | 4,940 | 6,760 | yes |
| `1.50` | 123 | 4,658 | 6,678 | yes |
| `1.50` | 777 | 4,620 | 6,654 | yes |

**15 of 15 strictly lower, 0 undecided, 0 contradicted** — including all five sparse cells, where
the two open intervals are still disjoint. The mechanism is §3's census read as arithmetic:
`social` spends a value only at branch 6, which `branches.md` §2 counts at 37,975 of 118,201
decisions, and every branch-4 and branch-5 decision is an opportunity where `individual` would have
reached rule 19's case 4 and drawn. The table above is that same difference measured on the stream
rather than counted on the branches, which is why it is worth having beside it and not instead of
it.

## 10. What this file does not establish

- Nothing here is a verification verdict. `VER-MOK-016` is the contract and `VREC-MOK-016` the
  record.
- **It does not consult the two sources on one observation.** That is the crate's own assertion, and
  past the parting it is the only instrument that can be: past the parting there is no shared
  observation to consult them on. §3 measures the population the assertion runs over and §5 measures
  why the released streams cannot reach further; neither restates the assertion as a measurement.
- **It does not decode the widened form's precondition.** §3 says why, and where the population it
  covers is already measured.
- **It does not close every tail by itself.** The thirty cells §7 names are exact for the whole run;
  the rest are exact to a stated pin, with the remaining opportunities carried as a range rather
  than folded into the figure. Closing them from the stream alone would need a position the engine
  does not print, which is `entropy.txt` §8's recorded residual and not a new one. §8's second
  control closes the `social` ranges from outside; the `reference` and `individual` ranges at
  density `0.15` have no such second instrument and stay open as printed.
- **It does not separate rule 19's case 3 from its case 4 on a released line.** §5's windows are
  that limitation stated where it bites, and `branches.md`'s reconstruction is where the separation
  is made.

Two observations for the record, neither of them this work order's to resolve. **No release-side
instrument reports a whole-run draw total under any source**: `shared_stream_draws` stops at
initialization and no command prints the stream, so every figure in §7 is a reconstruction from
printed regeneration and is retained as one. And the branch-3/branch-6 agreement of §8 is the first
time that split has been checked from the entropy side rather than from the classification's own
thirteen checks; it is offered as corroboration of `branches.md` and not as a change to it.
