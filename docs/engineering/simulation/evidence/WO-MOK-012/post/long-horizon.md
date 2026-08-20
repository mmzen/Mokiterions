# Ten thousand ticks: what the run completes with, and what it retains

| Field | Value |
|---|---|
| Retention item | "the 10,000-tick run's completion, composition and survivor figures, as evidence and not as an obligation" |
| Oracle | `VER-MOK-012` *Performance and resilience checks*: "A 10,000-tick run under the `social` source completes without panic and without unbounded growth in retained state, and its composition and survivor figures are captured as evidence rather than bound as obligations." |
| Capture | `capture-horizon.sh`, ten cells, 64,273,153 bytes of stream, **not retained** — the digests and the readers' output are |
| Readers | `analysis/horizon.py` (the stream) and `analysis/peak.py` (the process), neither of which imports, parses or opens `simulation.rs` |
| Exit codes | **`0`** and **`0`** — 561,958 lines checked with no disagreement; the peak flat at a factor of at most `1.0046` across three retained ladders, against a tolerance of `1.05` |
| Raw output | `post/horizon.txt`, 112 lines, and `post/horizon-memory.txt`, 132 lines, both retained whole; every figure below is transcribed from them |
| Commit | `1ae4267e729efd5a200f98498f8c7652df391dac` on `feature/phase-3-definition` |
| Binary | `target/release/Mokiterions.exe`, `cargo build --release --locked`, sha256 `fa7df55709f486b960dae9dd2f056de9dc5667f0b88426fd677b44339faacf81` |
| Provenance | the ladder's 1,000-tick rung reproduces the released `social` cell at digest `aa139c35f57d010fb4b8839144494f01e32af8e3dedd093b104c49a031c3184d`, machine-checked |
| Controls | nine in `horizon.py`, one in `peak.py`; one term each, applied alone, every one reverted |
| Date | 2026-08-20 |

---

## 1. The clause asks for three things, and they do not share an instrument

**Completion** is the cheap one: an exit code, a `simulation_ended` line, a summary line, and nothing
after them.

**Composition and survivor figures** are the summary line's own fields, but taking them from there and
stopping would be a transcript. They are read here from the stream's events as well, and the two are
reconciled field by field.

**"Without unbounded growth in retained state"** is the one that needed building. It is not a claim any
single line supports, and it cannot be answered by looking at the end of a run — a collection that grew
and was then dropped at shutdown would look identical to one that never grew. It needs a comparison in
which the *only* difference is how long the world ran, and it needs to reach state no event reports.

So there are two instruments, and they are reported apart because they measure different things.

## 2. What is retained, and which half of it a stream can show

Five things persist across a tick boundary. Three are visible in the stream and are reconstructed from
it, tick by tick:

| Retained | Read from | Bound it must respect |
|---|---|---|
| the Mokiterions | `agent_initialized`, `agent_died` | twelve, only falling, no resurrection, nothing naming a dead one afterwards |
| the food collection | `food_initialized`, `food_consumed`, `food_regenerated`, `food_regeneration_skipped` | 61 per territory, and the engine's own at-capacity count must equal the reader's |
| each Mokiterion's suffered-attack record | the `suffered` field of `action_trace` | eleven, one per other Mokiterion, since rule 25 closes the window at the sufferer's own opportunity |

Two are not visible in any stream: the previous tick's decision snapshots, which no event reports, and
the collected-event vector the text host leaves absent. **They are reachable only through the process's
own memory**, which is what `analysis/peak.py` exists for and why §12 is a separate measurement rather
than another column in §8.

## 3. Completion: ten cells, and two reasons a run stops

All ten cells exit `0`. None panics. Each ends with one `simulation_ended` line and one summary line,
with nothing after them — the reader treats any line following `simulation_ended` as a failure, and
finds none.

The termination reason is not the same on every seed:

| Cell | Reason | Ticks | Lines |
|---|---|---|---|
| `seed0-…-t10000` | **`extinction`** | **7,890** | 41,152 |
| `seed1-…-t10000` | `tick_limit` | 10,000 | 80,513 |
| `seed42-…-t10000` | `tick_limit` | 10,000 | 63,096 |
| `seed123-…-t10000` | `tick_limit` | 10,000 | 52,700 |
| `seed777-…-t10000` | **`extinction`** | **9,511** | 60,192 |

**Two of the five declared seeds never reach tick 10,000.** This is why the capture takes the clause's
run on all five seeds rather than on the one the clause's singular phrasing would allow: a single-seed
capture would have shown one of the two termination reasons and would have read as the whole picture.

An extinct run still completes, and completing is what the clause asks for. `extinction` is a stated
termination reason, the exit code is `0`, and the summary line is present and consistent. Nothing here
is a breach. It is, however, a fact about the world at this horizon that the product owner has not
seen, and §4 is where it becomes one.

## 4. Survivors and composition at ten thousand ticks

Transcribed from `post/horizon.txt` §3, which reads the summary line and reconciles every field of it
against the reader's own count:

| Cell | Survivors | Deaths | Territory A / B | Food A (l/m/h) | Food B (l/m/h) |
|---|---|---|---|---|---|
| `seed0-…-t10000` | **0** | 12 | 0 / 0 | 12/10/39 | 11/19/31 |
| `seed1-…-t10000` | **5** | 7 | 5 / 0 | 7/10/44 | 9/9/43 |
| `seed42-…-t10000` | **2** | 10 | 1 / 1 | 4/16/41 | 8/7/46 |
| `seed123-…-t10000` | **2** | 10 | 0 / 2 | 6/9/46 | 14/10/37 |
| `seed777-…-t10000` | **0** | 12 | 0 / 0 | 12/9/40 | 11/12/38 |

Survivors plus deaths is twelve on every cell, and `territory_a + territory_b` equals the survivor
count on every cell. Both are checked rather than read.

**The figure to put in front of the product owner.** `REQ-MOK-049`'s bound is *at least five of twelve
living at tick 1,000*, ratified unchanged on 2026-08-20 on the measured curve, where the same five
seeds leave **9, 10, 9, 9 and 11**. At tick 10,000 the same five seeds leave **0, 5, 2, 2 and 0**.

The contract is explicit that this is not a breach: these figures are "captured as evidence rather
than bound as obligations", and `REQ-MOK-049`'s obligation is stated at tick 1,000 and holds there. The
1,000-tick rung of this capture is byte-identical to the released cell that was ratified, so the two
figures describe the same world at two horizons rather than two different worlds.

**And the two agree where they overlap.** This reader derives 9 survivors and 1 combat death for seed 0
at 1,000 ticks, from the stream and without reading the implementation. `REQ-MOK-049`'s ratification
records 9 survivors and 1 combat death for the first of the declared seeds. A reader written for a
different purpose reproducing the ratified figure is the cheapest confirmation available that the two
are talking about the same measurement.

But the ratification's reasoning was "four survivors of margin at the worst seed", and that margin is
a property of the 1,000-tick curve alone. Whether the world *should* still be populated at 10,000
ticks is a product question, not an assurance one, and it has not been asked. It is raised for the
owner rather than answered here.

## 5. The same seed at five horizons: the decay curve

Seed 0's ladder is the population question asked within one world, with the tick limit as the only
variable:

| Limit | Reason | Survivors | Deaths | Opportunities taken |
|---|---|---|---|---|
| 1,000 | `tick_limit` | 9 | 3 | 10,847 |
| 2,000 | `tick_limit` | 8 | 4 | 19,261 |
| 5,000 | `tick_limit` | 2 | 10 | 32,837 |
| 10,000 | `extinction` at 7,890 | 0 | 12 | 37,603 |
| 20,000 | `extinction` at 7,890 | 0 | 12 | 37,603 |

The curve is not linear in either direction: seed 0 loses one Mokiterion between tick 1,000 and 2,000
and six between 2,000 and 5,000. The opportunity column is the integral of the population curve, and
it flattens for the same reason.

## 6. Deaths: what lands the blow, and what sets it up

`agent_died` carries one field, `health:0`, and no cause — rule 13's deliberate choice, since "death
stays one concept: there is no second death, no combat-specific event". The rule states where the cause
is instead recoverable: "That a given death was combat's is recoverable from the stream through
`attack_resolved`'s `target_died` field". The reader takes it from exactly there, and the remainder is
rule 12's decay.

| Cell | Deaths | Combat | Starved | First | Last |
|---|---|---|---|---|---|
| `seed0-…-t10000` | 12 | 2 | 10 | 9 | 7,890 |
| `seed1-…-t10000` | 7 | 4 | 3 | 9 | 9,107 |
| `seed42-…-t10000` | 10 | 3 | 7 | 8 | 9,833 |
| `seed123-…-t10000` | 10 | 3 | 7 | 6 | 8,427 |
| `seed777-…-t10000` | 12 | 1 | 11 | 10 | 9,511 |

**Combat is the minority cause and the majority of the early deaths.** It accounts for 13 of 51 deaths
across the five cells, yet **the first death in all five is a combat death** — at ticks 9, 9, 8, 6 and
10, each on the same tick as an `attack_resolved` line naming that Mokiterion with `target_died:yes`.

The last blow is combat's; the setup is starvation's. In every one of those five opening deaths the
victim's health *before* the lethal strike was already low — 20, 20, 10, 21 and 15 of 100 — so rule
12's decay had brought it within one strike's reach and rule 22 finished it. A reading of the combat
column alone as "how lethal is combat" would be wrong in both directions.

**The volume is small against the opportunity count.** Over seed 1's 10,000 ticks and 74,370 decision
opportunities there are 31 `attack_resolved`, 25 `surrender_resolved` and 319 `threat_resolved` lines —
375 encounter resolutions, or one per 198 opportunities. The other seeds are the same order: 10/6/83,
27/17/300, 23/8/50 and 17/16/112. The encounter machinery is rare rather than dominant at this horizon.

## 7. The forfeited turn: rule 13's specified case, measured

Rule 13 states a consequence of combat resolving inside another Mokiterion's turn:

> rule 22 resolves inside another Mokiterion's turn, so a Mokiterion may die at a point in the tick
> where it has not yet acted, and it then receives no opportunity that tick or ever. Its
> suffered-attack record dies with it, unread.

That sentence is what makes the population check in §11 non-trivial, and it is measurable. Rule 2 fixes
the turn order as ascending identifier order, so "has not yet acted" is exactly "was killed by a
*lower* identifier in that tick". The reader derives the exception in that form and finds **nine
instances across the capture**, every one in the specified direction:

| Cell | Victim, tick | Striker |
|---|---|---|
| `seed0-…-t5000`, `-t10000`, `-t20000` | `M10` in tick 4,378 | `M07` |
| `seed1-…-t10000` (both trace settings) | `M11` in tick 9,107 | `M07` |
| `seed42-…-t10000` | `M11` in tick 9 | `M10` |
| `seed42-…-t10000` | `M10` in tick 6,953 | `M08` |
| `seed123-…-t10000` | `M06` in tick 9 | `M04` |
| `seed777-…-t10000` | `M12` in tick 10 | `M07` |

The converse holds too, and it is the sharper half: the five opening deaths of §6 were killed by
`M12`, `M12`, `M12`, `M11` and `M07` against victims `M09`, `M08`, `M07`, `M10` and `M12` — a *higher*
identifier in four of the five, and in each of those four the victim is found taking its opportunity in
that same tick before dying in it. The `strike-order` control is that comparison reversed, and it fails
on all 23 lethal strikes in the capture.

This is the first evidence in the packet that the case rule 13 specifies actually occurs in a released
run. It is nine ticks out of 73,291, so no test constructed to be representative would have found it.

## 8. The three collections a stream shows

Transcribed from `post/horizon.txt` §5:

| Cell | Survivors | A high | B high | A low | B low | at capacity | food ids | widest record |
|---|---|---|---|---|---|---|---|---|
| `seed0-…-t10000` | 0 | 61 | 61 | 47 | 43 | 797 | 1,325 | — |
| `seed1-…-t10000` | 5 | 61 | 61 | 39 | 54 | 485 | 2,595 | 1 |
| `seed42-…-t10000` | 2 | 61 | 61 | 47 | 55 | 705 | 2,082 | — |
| `seed123-…-t10000` | 2 | 61 | 61 | 36 | 57 | 900 | 1,757 | — |
| `seed777-…-t10000` | 0 | 61 | 61 | 43 | 51 | 752 | 1,946 | — |

**No collection grows.** The population only falls, on every cell, checked at every death rather than
at the end. Both territories reach the capacity of 61 in every run and **never once exceed it** over
the capture's 73,291 ticks. The suffered-attack record's widest observed value is 1 against a bound of 11.

**The at-capacity column is the engine's own count, not the reader's.** Each of those 485 to 900
`food_regeneration_skipped` lines carries `count:61`, and the reader compares it against the count it
has been maintaining independently from the consume and regenerate lines. Agreement on all 5,333 of
them across the capture is what makes the food reconstruction a check rather than a paraphrase — a
reader that had lost track would be caught by the engine's own figure.

**The low-water marks are tracked from tick 1 rather than tick 0**, and the reason is recorded in the
reader: during tick 0 territory B is empty while A is being filled, so a minimum of zero there would be
an artifact of the fill order and not a figure about the world. From tick 1 the collections never fall
below 36 of 61 in either territory.

## 9. Food's accounting closes

The collection's size at the end of a run must be its initial 122 less what was consumed plus what was
regenerated, and it is, on every cell:

| Cell | 122 − consumed + regenerated | Final collection |
|---|---|---|
| `seed0-…-t1000` | 122 − 369 + 357 = 110 | 61 + 49 = 110 |
| `seed0-…-t2000` | 122 − 644 + 642 = 120 | 61 + 59 = 120 |
| `seed0-…-t5000` | 122 − 1,062 + 1,062 = 122 | 61 + 61 = 122 |
| `seed0-…-t10000` | 122 − 1,203 + 1,203 = 122 | 61 + 61 = 122 |
| `seed1-…-t10000` | 122 − 2,473 + 2,473 = 122 | 61 + 61 = 122 |
| `seed42-…-t10000` | 122 − 1,960 + 1,960 = 122 | 61 + 61 = 122 |
| `seed123-…-t10000` | 122 − 1,635 + 1,635 = 122 | 61 + 61 = 122 |
| `seed777-…-t10000` | 122 − 1,824 + 1,824 = 122 | 61 + 61 = 122 |

Eight of the ten cells end at the full 122, both territories at capacity. The two that do not are the
two shortest runs, which stop while enough Mokiterions are still alive to hold the collection below it.
The final six food fields of the summary line are checked against the reader's own per-class counts, so
this table is derived twice and agrees.

A consumed identifier never returns and a regenerated one is never already held: both are failures in
the reader, and neither occurs in 29,678 consume and regenerate lines.

## 10. The one retained figure that grows, and why it is not a structure

The food identifier counter rises with the run: high-water 479 at 1,000 ticks and 1,325 to 2,595 at
10,000. It is the only retained figure in the capture that grows without bound.

It is a counter and not a collection, and the reader establishes that rather than asserting it: **the
highest identifier issued equals the number issued**, on every cell — 2,595 issued and `F2595` the
highest, 1,325 and `F1325`, and so on. A contiguous, never-reused sequence means nothing holds an entry
per identifier; if anything did, the collection size and the counter would diverge and §8's per-territory
count would exceed 61. Reuse or a regression is a failure in the reader, and there are none.

## 11. The population read a second way, and a third

§4's survivor count is one number at the end of a run. The same population is available at every tick
of it, because every living Mokiterion emits one `survival_changed` line per tick. So the reader checks,
**for each of the capture's 73,291 ticks**, that the subjects of that tick's lines are exactly the
Mokiterions alive at its start — less §7's forfeited turns — and that they appear in ascending
identifier order, which is rule 2's own ordering.

That check is what a summary field cannot do. It fails if a tick is skipped, if a dead Mokiterion
decides, if one decides twice, if the turn order moves, or if the reconstructed curve and the stream's
own per-opportunity lines disagree anywhere. It passes on all 73,291.

The third reading is the trace-on cell: it emits one `action_trace` per opportunity, and its
**74,370 traces against 74,370 opportunities** is the same figure reached by a third route.

## 12. Peak working set: the half no stream reaches

The decision snapshots and the absent event vector are retained state with no event of their own, so
`analysis/peak.py` measures the process instead. It reads `PeakWorkingSetSize` from
`GetProcessMemoryInfo` on the child's handle after it exits — a ceiling over the whole run rather than a
sample, which is what a growth claim needs: a sampler can miss a peak between samples, and a
final-state reading misses anything freed before exit.

**The criterion is a ratio, not a limit**, because the absolute figure is this machine's and the ratio
is the world's. The ladder was taken three times and all three are retained whole:

| Ticks | First | Second | Third |
|---|---|---|---|
| 1,000 | 4,328 KiB | 4,324 KiB | 4,324 KiB |
| 2,000 | 4,332 KiB | 4,332 KiB | 4,332 KiB |
| 5,000 | 4,332 KiB | 4,332 KiB | 4,332 KiB |
| 10,000 | 4,344 KiB | 4,332 KiB | 4,344 KiB |
| 20,000 | 4,336 KiB | 4,332 KiB | 4,336 KiB |
| **spread** | 16 KiB | 8 KiB | 20 KiB |
| **factor** | **1.0037** | **1.0019** | **1.0046** |
| same cell ×3 | 8 KiB | 12 KiB | 8 KiB |

Twenty times the ticks moves the peak by 8 to 20 KiB, a factor of at most 1.0046 against a tolerance of
1.05. Three things make that a reading rather than a rounding:

- **The 20,000-tick run never peaks above the 10,000-tick one** — 4,336 against 4,344, 4,332 against
  4,332, 4,336 against 4,344. A monotone column at this spread would be suggestive; a column where
  twice the ticks never costs more says the spread is the measurement's.
- **Re-running the same cell three times moves 8, 12 and 8 KiB**, and in the second ladder that band is
  *larger* than the whole spread the tick count produces. The band is reported beside the figure rather
  than subtracted from it.
- **The spread is not reproducible.** 16, then 8, then 20 KiB from the identical command. A figure that
  moves when nothing moved is the instrument's.

Two further ladder invocations were taken at this commit and are **not** retained — they were run to
confirm the script still worked before the record was written, and the widest spread across all five is
24 KiB at a factor of 1.0056, still inside the tolerance. That is the largest figure this measurement
produced, and it is stated here rather than left to the three retained columns to imply.

**The instrument is controlled**, because a flat column is worthless if the reading is broken. A child
that holds 16 MiB reads 29,648 KiB and one that holds 64 MiB reads 78,836 KiB: 48 MiB more held moves
the reading by 49,188 KiB. The engine, at 4,336 KiB in the same invocation, is flat because nothing
accumulates and not because nothing is being read.

## 13. The tick ladder: the limit decides only where a run stops

The same seed at five limits, with the tick count as the only difference. Each shorter run's body — the
stream less its `simulation_ended` and `summary` lines — must be a byte-exact prefix of the longer one's:

| Comparison | Lines | Prefix |
|---|---|---|
| t1000 in t2000 | 11,778 | yes |
| t2000 in t5000 | 20,845 | yes |
| t5000 in t10000 | 35,641 | yes |
| t10000 in t20000 | 41,150 | yes |

And because seed 0 goes extinct at 7,890, **t10000 and t20000 are byte-identical, not merely
prefixed** — the reader checks that rather than assuming it. A world with nobody in it produces the same
stream however long the limit is.

This is the growth measurement's other half. If a retained structure grew with ticks and leaked into
behaviour, the longer run would diverge from the shorter one somewhere; it does not, at any of the four
rungs. The `ladder` control reverses the direction of the prefix comparison and fails on three of the
four rungs — the fourth being the identical pair, which is a prefix of itself in either direction, and
that is worth saying plainly rather than counting as a pass.

## 14. Tracing changes nothing, in the strong form

`SPEC-MOK-001` rule 7 fixes that trace configuration "never changes entropy consumption or simulation
state". The strong form of that claim is available here at no cost: the trace-on stream of seed 1, with
its **74,370** `action_trace` lines removed, is the trace-off stream of the same world **byte for byte
over all 10,000 ticks** — not over a constructed encounter. Every other figure in this file agrees
across the pair, including the survivor and composition figures and the food accounting. The
`projection` control compares the stream unprojected and fails.

## 15. Provenance: the shortest rung is the released cell

The released `social` capture is 1,000 ticks at these seeds and densities, so this capture's 1,000-tick
rung is the same cell under a longer name and must reproduce it. It does, at digest
`aa139c35f57d010fb4b8839144494f01e32af8e3dedd093b104c49a031c3184d`, 1,244,721 bytes and 11,780 lines,
checked against `post/social-manifest.txt` rather than by eye.

This is what ties every figure above to the released evidence and to `REQ-MOK-049`'s ratified numbers.
Without it the horizon figures would be provenance-less rather than merely different — they would
describe *a* binary. The `provenance` control compares the 2,000-tick rung instead and fails, so the
check is a comparison and not a coincidence.

## 16. The controls, and the one that had to be replaced

Nine in the reader and one in the memory harness. Each names one global and the single value it is
given; nothing else moves, and every one is reverted:

| Control | Perturbation | Disagreements |
|---|---|---|
| `population` | `AGENTS` 12 → 13 | 10 |
| `capacity` | `PER_TERRITORY` 61 → 60 | 11,158 |
| `window` | `WINDOW_BOUND` 11 → 0 | 27 |
| `split` | `SPLIT` 63 → 62 | 6,955 |
| `order` | `DECISION_ORDER` ascending → descending | 420 |
| `strike-order` | `STRIKE_ORDER` reversed | 23 |
| `ladder` | prefix comparison reversed | 3 |
| `projection` | trace lines left in | 1 |
| `provenance` | the 2,000-tick rung compared instead | 1 |
| `peak.py --control` | a child holding 16 then 64 MiB | reads 49,188 KiB of growth |

The `population` control moves `WINDOW_BOUND` with `AGENTS`, because at module load the bound *is*
`AGENTS - 1`; keeping that relation is what makes it one term rather than two.

**One control was dead and was replaced.** The first version of the food control reordered the `CLASSES`
tuple, and the reader exited `0` under it: the class counts are compared as a dictionary, so permuting
the names the reader iterates changes nothing it computes. A control that passes measures nothing, so it
was replaced with `split`, which perturbs the reader's own derivation of a regenerated food's territory
— the derivation the engine's at-capacity count is checked against — and fails 6,955 times. The dead
control is recorded here rather than deleted, because "nine controls all failed" is only informative if
the ones that could not fail are named.

## 17. What this does not establish

- **20,000 ticks is not unbounded.** The claim measured is that nothing grows across a twenty-fold tick
  range. A structure growing slowly enough to stay inside 20 KiB over 20,000 ticks — or one that grows
  only under conditions no declared seed reaches — is not excluded.
- **The peak is a process ceiling, not a per-collection figure.** For the decision snapshots and the
  absent event vector there is no finer instrument here. A leak in one of them smaller than the 8 to
  12 KiB the same cell moves by on a re-run would not be visible.
- **`starved` is a remainder, not a reported cause.** The reader separates combat deaths using
  `attack_resolved`'s own `target_died` field and attributes the rest to rule 12. If a third death path
  existed it would be counted as starvation. What is checked is that combat deaths never exceed
  `agent_died` lines; the split itself rests on rule 13's statement that there are two paths and one
  event.
- **The suffered-attack record's bound is checked and never approached.** 27 of the 74,370 traced lines
  carry the field and none is wider than one entry, against a bound of eleven. `VER-MOK-012` says that
  bound "is asserted"; what the engine asserts is that rule 25 clears the record at the opportunity
  (`mokiterions-core/src/simulation.rs:6855`), and the multi-entry case exists only in constructed
  state at `:6828`. The gap is raised in the escalation record, not closed here.
- **One source, one density, five seeds.** The clause names the `social` source and this is it, at the
  default density on all five declared seeds. No other source was run at this horizon, and no other
  density.
- **The peak figure is Windows-specific and machine-specific.** `PeakWorkingSetSize` has no portable
  equivalent; the host is `x86_64-pc-windows-msvc`. The ratio is what is claimed, not the 4.3 MB.
- **The survivor figures are evidence and carry no obligation** — the contract says so in as many
  words. §4's comparison against `REQ-MOK-049`'s ratified floor is put to the product owner as a
  finding, and this file does not decide whether a horizon obligation should exist.
