# Rule 26's six branches, and rules 22 to 24 at their boundaries

| Field | Value |
|---|---|
| Retention items | "the branch distribution under `social` per seed, including the answer branch's three choices"; "the measured strikes per encounter, forfeits discarded at a full recipient, and surrenders below `satiety` `2`" |
| Oracle | `VER-MOK-016` oracle 4 |
| Reader | `analysis/runs.py`, the same reader and the same thirty streams as `post/runs.md` |
| Exit code | **`0`** — thirteen checks, every one reading zero |
| Raw output | `post/runs.txt` §§4 to 7 |
| Capture | `git archive 59d61b915630fd55f04bcdbb346aa22cdbfdfff6`, 30 cells, every cell exiting `0` |
| Date | 2026-08-20 |

`VER-MOK-016` was written against `REQ-MOK-057`'s **five** branches. The amendment of 2026-08-20 hoisted
rule 19's case 3 into rule 26 as a new branch 3, so the distribution below has six, and the branch
numbers here are the amended ones throughout — as are the numbers in the code and in the test names, per
`test-census-reconciliation.md` §7.

---

## 1. Why this is a replay and not a count, and why its numbers are evidence

Four of the six branches are legible on a trace line: branch 1 by a non-empty `suffered` field, branch 2
by `eat` or `sleep`, branch 4 by `attack` or `threaten`, branch 5 by `approach` or `avoid`. **Branch 3
and branch 6 both render as `move:<direction>`** — one is rule 19 case 3's step toward a tolerated
resource, the other rule 19 case 4's random step — and no field on the line distinguishes them. They are
also the pair that matters most, because branch 6 is the only branch that draws from the shared entropy
stream.

The obvious instrument is a temporary build that prints the branch. This packet does not use one, for the
reason `escalation.md` gives about the ablation measurement: a figure produced by a build that no longer
exists is not reproducible from retained bytes, and a verifier cannot re-derive it. The released stream
already carries every input rule 26 reads — `analysis/runs.py`'s docstring enumerates the six and where
each comes from — so the branch each decision took is **reconstructed** from the stream and then
**checked against the engine's own choice**. The checks are what make this evidence. Nine of the
reader's thirteen are checks on the classification and are the table below; the other four re-derive
rules 22, 23 and 24 from the events, and §§5 and 6 report those where their figures are:

| Check | What it would catch | Result over 118,201 decisions |
|---|---|---:|
| branch 3's predicted direction against the engine's | a food predicate that finds the wrong resource, or an axis rule transcribed wrongly | **0 disagreements** |
| branch 6 classified with a living Mokiterion perceived | a branch-3 decision hiding in the branch-6 count, which is what a too-strict predicate produces | **0** |
| branch 4 or 5 with a branch-3 candidate available | the same error in the other direction: a too-lax predicate | **0** |
| branch 4 without contact; branch 5 in contact | rule 20's relation misread from reconstructed positions | **0** and **0** |
| branch 1's verb against `60` and `30`; branch 4's and 5's against `95` | the three constants of rule 26 | **0** and **0** |
| a `wait` proposed; a `move` with no direction | a decision the classification could not place | **0** and **0** |

Checks 1 and 3 bound the food predicate from both sides and check 2 bounds the branch-3/branch-6 split
where it is load-bearing, so a reconstruction that passed all of them while misclassifying would have to
be wrong in a way invisible to the rule it reconstructs.

There is a second, free check, and it is worth stating because it is independent of all of the above: the
branch counts must partition the verb counts `runs.md` §3 reads straight off the lines. They do, exactly,
over all fifteen traced cells:

| | | |
|---|---|---:|
| branch 1 | `fight` + `retreat` + `surrender` | 13 + 27 + 95 = **135** |
| branch 2 | `eat` + `sleep` | 4,230 + 3,349 = **7,579** |
| branches 3 + 6 | `move` | 58,441 + 37,975 = **96,416** |
| branch 4 | `attack` + `threaten` | 143 + 620 = **763** |
| branch 5 | `approach` + `avoid` | 4,292 + 9,016 = **13,308** |

## 2. The six-branch distribution, per cell

| cell | b1 answer | b2 eat/sleep | b3 seek | b4 contact | b5 distance | b6 search | total |
|---|---:|---:|---:|---:|---:|---:|---:|
| `seed0-d0.15` | 4 | 229 | 2,046 | 60 | 343 | 1,726 | 4,408 |
| `seed0-d0.75` | 7 | 684 | 5,459 | 19 | 1,481 | 3,197 | 10,847 |
| `seed0-d1.50` | 4 | 755 | 5,377 | 10 | 1,395 | 3,468 | 11,009 |
| `seed1-d0.15` | 4 | 106 | 787 | 97 | 201 | 1,312 | 2,507 |
| `seed1-d0.75` | 12 | 654 | 5,333 | 21 | 923 | 3,076 | 10,019 |
| `seed1-d1.50` | 9 | 693 | 5,066 | 16 | 1,036 | 3,198 | 10,018 |
| `seed42-d0.15` | 11 | 115 | 880 | 44 | 375 | 1,243 | 2,668 |
| `seed42-d0.75` | 15 | 602 | 4,605 | 32 | 923 | 3,102 | 9,279 |
| `seed42-d1.50` | 6 | 760 | 5,795 | 27 | 1,200 | 3,221 | 11,009 |
| `seed123-d0.15` | 3 | 130 | 1,100 | 152 | 414 | 1,082 | 2,881 |
| `seed123-d0.75` | 18 | 579 | 4,406 | 58 | 903 | 3,057 | 9,021 |
| `seed123-d1.50` | 12 | 752 | 5,844 | 66 | 1,400 | 2,934 | 11,008 |
| `seed777-d0.15` | 13 | 112 | 774 | 65 | 169 | 1,365 | 2,498 |
| `seed777-d0.75` | 7 | 704 | 5,780 | 57 | 1,400 | 3,061 | 11,009 |
| `seed777-d1.50` | 10 | 704 | 5,189 | 39 | 1,145 | 2,933 | 10,020 |
| **all fifteen** | **135** | **7,579** | **58,441** | **763** | **13,308** | **37,975** | **118,201** |
| share | 0.11% | 6.41% | 49.44% | 0.65% | 11.26% | 32.13% | |

| density | b1 | b2 | b3 | b4 | b5 | b6 | decisions |
|---|---:|---:|---:|---:|---:|---:|---:|
| `0.15` | 0.2% | 4.6% | 37.3% | 2.8% | 10.0% | 45.0% | 14,962 |
| `0.75` | 0.1% | 6.4% | 51.0% | 0.4% | 11.2% | 30.9% | 50,175 |
| `1.50` | 0.1% | 6.9% | 51.4% | 0.3% | 11.6% | 29.7% | 53,064 |

Four readings, each of which the numbers support directly:

- **The hoisted branch is the busiest branch in the source.** Rule 19 case 3's seek step takes 49.4% of
  all decisions, more than the other five together. That is the amendment's own consequence: before it,
  any of these 58,441 decisions taken where a Mokiterion was *also* perceived would have gone to branch 4
  or branch 5 instead, and the source would have chased company over food. `escalation.md` measures what
  that cost in survivors; this is the same change counted as decisions.
- **Density moves the seek/search split and nothing much else.** From `0.15` to `1.50` branch 3 rises
  37.3% → 51.4% and branch 6 falls 45.0% → 29.7%, while branches 1, 2 and 5 move by less than three
  points. Fewer resources in perception means more decisions with no tolerated resource to walk toward,
  which is exactly the case branch 6 exists for.
- **Contact is rare and it is rarest where there is most food.** Branch 4 takes 2.8% of decisions at
  density `0.15` and 0.3% at `1.50` — seven times as often in the sparse world, where Mokiterions
  converge on the few resources that exist. This is the mechanism behind `runs.md` §2's threat counts:
  379 of the matrix's 620 threats are in the five `0.15` cells.
- **Branch 1 is 0.11% of decisions and 100% of the combat the source starts.** 135 answers across
  15,000 ticks. It is the branch with the shortest reach and the highest precedence, which is what
  `SPEC-MOK-001` rule 26 designed it to be.

**Entropy, from the same table.** Only branch 6 draws, and it draws exactly once, so the source consumed
**37,975 draws** across the fifteen traced cells for 118,201 decisions — 0.32 draws per decision. The
other 80,226 decisions are derived. `entropy.txt` is where that claim is made against
`REQ-MOK-057`'s constraint; here it is only the count.

**A corrected citation.** Until 2026-08-21 the sentence above cited `REQ-MOK-055`, which is the
threaten resolution's *No entropy draw* clause and not this claim's requirement. The constraint that
branches 1 to 5 draw nothing and branch 6 draws exactly what rule 19's case 4 draws is
`REQ-MOK-057`'s, as line 13 of this file already had it. No figure changes: the count above and every
table it is read from are untouched, and only the requirement the count answers to is renamed.
`entropy.txt` §7 is where the structural side is measured — every draw site in the shipped engine
enumerated, and every `return` in this source's body shown to precede its one `entropy` use.

## 3. The answer branch's three choices

| cell | answers | `fight` | `retreat` | `surrender` | naming a target not living |
|---|---:|---:|---:|---:|---:|
| `seed0-d0.15` | 4 | 0 | 0 | 4 | 0 |
| `seed0-d0.75` | 7 | 0 | 2 | 5 | 0 |
| `seed0-d1.50` | 4 | 0 | 2 | 2 | 0 |
| `seed1-d0.15` | 4 | 0 | 0 | 4 | 0 |
| `seed1-d0.75` | 12 | 0 | 2 | 10 | 0 |
| `seed1-d1.50` | 9 | 0 | 2 | 7 | 0 |
| `seed42-d0.15` | 11 | 0 | 0 | 11 | 0 |
| `seed42-d0.75` | 15 | 3 | 4 | 8 | 0 |
| `seed42-d1.50` | 6 | 0 | 1 | 5 | 0 |
| `seed123-d0.15` | 3 | 0 | 0 | 3 | 0 |
| `seed123-d0.75` | 18 | 6 | 6 | 6 | 0 |
| `seed123-d1.50` | 12 | 3 | 2 | 7 | 0 |
| `seed777-d0.15` | 13 | 1 | 4 | 8 | 0 |
| `seed777-d0.75` | 7 | 0 | 0 | 7 | 0 |
| `seed777-d1.50` | 10 | 0 | 2 | 8 | 0 |
| **all fifteen** | **135** | **13** | **27** | **95** | **0** |

Every one of the 135 answers matched the verb the two thresholds require of the `fear` its own trace line
reports — `surrender` at `60` and above, `retreat` from `30`, `fight` below — with zero disagreements.
The distribution is heavily toward `surrender`, and the mechanism is rule 12 rather than rule 26: an
attack arrives inside an encounter, an encounter means mutual perception, and mutual perception adds
`10` to `fear` every tick and never subtracts while it lasts. A Mokiterion that has been in perception
of another for six ticks is already at `60`. `fight` is what happens when the first strike of an
encounter lands early, and 6 of its 13 occurrences are in one cell.

**No answer in the matrix named a dead or fled target**, so the cost `SPEC-MOK-001` rule 26 accepts and
asks `VER-MOK-016` to measure did not materialize here. `runs.md` §5 records that with the reason it
would have been easy to report wrongly.

## 4. Strikes per encounter

Over the 115 encounters `runs.md` §6 defines and counts:

| strikes in the encounter | encounters | share |
|---:|---:|---:|
| 0 | 67 | 58.3% |
| 1 | 12 | 10.4% |
| 2 | 10 | 8.7% |
| 3 | 6 | 5.2% |
| 4 | 5 | 4.3% |
| 5 | 10 | 8.7% |
| 6 | 1 | 0.9% |
| 7 | 2 | 1.7% |
| 8 | 2 | 1.7% |
| | **115** | **1.36 strikes per encounter** |

**A majority of encounters are bloodless, and the mean is not the shape.** 58.3% of contacts produce no
strike at all — rule 26 branch 4 proposes `threaten` rather than `attack` at `fear` `95` and above, and
`runs.md` §2's 620 threats against 156 strikes is the same fact counted by verb. Of the 48 encounters
that do produce a strike, 22 produce one or two and 5 produce six or more; the longest exchange is 8
strikes. `SPEC-MOK-001` rule 22's damage range is `10..=30`, so 8 strikes is enough to kill twice over,
and 21 of the 156 strikes were fatal.

## 5. Rule 23 at its boundary: every threat in the matrix is saturated

| | |
|---|---:|
| `threat_resolved` events, traced cells | 620 |
| of which reported `increase:0` | **620** |
| of which found the target at `fear` `100` | **620** |
| threats where `THREAT_FEAR_INCREASE` had any numeric effect | **0** |

    tick=11 subject=M01 event=threat_resolved result=target:M04,increase:0,target_fear:100->100

Every one of the 1,240 `threat_resolved` events across all thirty cells is of that form. The reader
re-derives rule 23's arithmetic — `increase == min(30, 100 - before)` and `after == before + increase` —
on every one of them and finds no disagreement, so the rule is implemented correctly and its constant is
**inert in whole runs**.

The mechanism is the composition of two owner decisions rather than a defect in either. `threaten` is
proposed only where the threatener's own `fear` is `95` or above, and `fear` moves in steps of `+10` per
tick of perception and `-5` per tick without it, so a threatener has necessarily been perceiving someone
for at least ten ticks. Rule 20's contact is a Chebyshev distance of `1` inside a perception radius of
`16`, and perception is symmetric, so by the time a pair is in contact the *target* has been accumulating
the same `+10` for at least as long. Both parties are at `100` before contact is reached, and `100` is
where rule 23 has nothing left to add.

**This is recorded as an observation for the product owner, not as a defect.** `THREAT_FEAR_INCREASE` of
`30` is exercised by `mokiterions-core`'s constructed-state cases, which is where the boundary belongs;
what these runs show is that no reachable whole-run state exercises it. Two things follow, and both are
the owner's to weigh rather than this work order's: `threaten` is presently a verb whose only effect on
the world is the record of it, and any future change to rule 12's step sizes or to the `95` gate would
give rule 23 an effect it does not have today.

## 6. Rule 24 at its boundaries: the forfeit, the discard, and the halving of nothing

| | |
|---|---:|
| `surrender_resolved` events, traced cells | 95 |
| satiety forfeited in total, at half each and truncating | 2,813 |
| satiety **transferred** to the attacker | **540** |
| satiety **discarded** because the attacker had no room | **2,273** — 80.8% of everything forfeited |
| surrenders with a non-zero discard | **90 of 95** |
| surrenders whose recipient was already at `satiety` `100`, forfeiting everything | **0** |
| surrenders below `satiety` `2`, where the half is nothing | **0** |
| smallest and largest forfeit | 11 and 47 |
| recipient's `satiety` at the surrender, least and greatest | 26 and 99 |

The reader closes rule 24's arithmetic on every one of the 190 surrender events across all thirty cells
— forfeit is `before // 2`, the transfer is `min(forfeit, 100 - recipient)`, the discard is the
remainder, and both parties' reported transitions must follow — with no disagreement.

Two boundaries `VER-MOK-016` names explicitly are **not reached** in whole runs, and both are covered by
constructed-state cases instead:

- **A forfeit discarded at a *full* recipient.** No surrender in the matrix had a recipient at exactly
  `100`, so `transferred:0` never occurs. What does happen, in 90 of the 95, is a *partial* discard: the
  recipient had less room than the forfeit and the remainder was destroyed. The other five had room to
  take the whole of it. Recipients' `satiety` at the moment of surrender spans 26 to 99, so the discard
  path is exercised heavily and only its endpoint is missing.
- **A surrender below `satiety` `2`.** The smallest forfeit is 11, so the smallest surrendering
  `satiety` in the matrix is 22. Nothing in these runs halves a `1` to a `0`.

**The 80.8% discard is the substantive reading here**, and it is a direct consequence of the owner's
decision of 2026-08-20 to forfeit "half its satiety" with the excess destroyed rather than capped at what
the attacker can hold. Attackers are well-fed — a Mokiterion strikes at low `fear` or when it is
cornered, not when it is starving — so most of what a surrender gives up leaves the world. Whether that
is the intended economy is a product question; the measurement is here because `VER-MOK-016` asks for it
by name, and this file does not answer it.

## 7. What this file does not establish

- Nothing here is a verification verdict. `VER-MOK-016` is the contract, `VREC-MOK-016` the record.
- The branch classification is a reconstruction with thirteen checks against the engine, not a reading of
  the engine's own internal state. Its correctness rests on those checks and on the reader this packet
  retains, both of which a verifier can re-run over the same digested bytes.
- The encounter definition is the reader's, as `runs.md` §6 states, so the 115 and the histogram in §4
  move with it. Every resolution count in §§4 to 6 is read from events and does not.
- §§5 and 6 report two constants as unexercised **in whole runs at the declared matrix**. That is not a
  claim that either is unreachable, and the constructed-state tables in `resolution-tables.md` are where
  their boundaries are covered.
