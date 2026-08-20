# Whole runs under `social`: outcomes, deaths by cause, verbs, rejections and encounters

| Field | Value |
|---|---|
| Retention item | "per-seed tables of survivors, deaths by cause, encounters, each verb proposed and applied, and rejections by reason — on failing seeds too" |
| Oracle | `VER-MOK-012` oracle 4 |
| Reader | `analysis/runs.py`, over the thirty streams `social-manifest.txt` digests |
| Invocation | `python docs/engineering/simulation/evidence/WO-MOK-012/analysis/runs.py <capture-dir>` |
| Exit code | **`0`** — the reader exits non-zero if any of its thirteen checks fires, and none did |
| Raw output | `post/runs.txt`, 151 lines, retained whole. Every table below is transcribed from it |
| Capture | `git archive 59d61b915630fd55f04bcdbb346aa22cdbfdfff6`, 30 cells, every cell exiting `0` — `post/capture-state.txt` §5 |
| Date | 2026-08-20 |

Everything here is derived from the released event stream and from nothing else: the same 45.6 MB of
bytes `social-manifest.txt` digests, read by a script this packet retains. No instrumented build, no
private access, no figure typed in by hand.

`branches.md` is the other half of this reading — the six-branch distribution, the answer branch's three
choices, strikes per encounter and rule 23's and rule 24's boundaries. The two files share one reader
and one capture, so no figure in either can disagree with the other.

---

## 1. The thirty cells, and what `--trace-actions` does not change

Five declared seeds of `VER-MOK-002` × `--policy social` × densities `0.15`, `0.75` and `1.50` × trace
off and on, at 1,000 ticks. Fifteen pairs, and the two cells of a pair differ in one command-line flag.

`SPEC-MOK-001` rule 7 states the property: "Trace configuration never changes entropy consumption or
simulation state." The two cells of a pair cannot be compared by digest — one carries 96,416 more lines
than the other — so the comparison is made on every outcome the stream reports:

| | |
|---|---:|
| pairs compared | 15 |
| pairs agreeing on **all eleven** outcome columns — exit code, termination reason, survivors, deaths, deaths by combat, deaths by attrition, territory A and B occupancy, strikes, threats, surrenders | **15** |
| pairs disagreeing on any column | **0** |

So every figure below can be read off the traced cell of a pair and is the untraced cell's figure too.
That is what makes the branch reconstruction in `branches.md` a statement about the source rather than
about the source under observation.

## 2. Per-cell outcomes and deaths by cause

The fifteen traced cells. `runs.txt` §1 carries all thirty rows, untraced ones included, and the
untraced fifteen are identical column for column by §1 above.

| cell | exit | end | survivors | deaths | combat | attrition | terr A | terr B | strikes | threats | surrenders |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `seed0-social-d0.15` | 0 | tick_limit | 1 | 11 | 0 | 11 | 1 | 0 | 4 | 56 | 4 |
| `seed0-social-d0.75` | 0 | tick_limit | 9 | 3 | 1 | 2 | 3 | 6 | 8 | 11 | 5 |
| `seed0-social-d1.50` | 0 | tick_limit | 11 | 1 | 1 | 0 | 5 | 6 | 5 | 5 | 2 |
| `seed1-social-d0.15` | 0 | **extinction** | 0 | 12 | 1 | 11 | 0 | 0 | 5 | 92 | 4 |
| `seed1-social-d0.75` | 0 | tick_limit | 10 | 2 | 2 | 0 | 4 | 6 | 14 | 7 | 10 |
| `seed1-social-d1.50` | 0 | tick_limit | 10 | 2 | 2 | 0 | 6 | 4 | 11 | 5 | 7 |
| `seed42-social-d0.15` | 0 | **extinction** | 0 | 12 | 1 | 11 | 0 | 0 | 12 | 32 | 11 |
| `seed42-social-d0.75` | 0 | tick_limit | 9 | 3 | 2 | 1 | 6 | 3 | 17 | 18 | 8 |
| `seed42-social-d1.50` | 0 | tick_limit | 11 | 1 | 1 | 0 | 5 | 6 | 7 | 20 | 5 |
| `seed123-social-d0.15` | 0 | **extinction** | 0 | 12 | 1 | 11 | 0 | 0 | 4 | 148 | 3 |
| `seed123-social-d0.75` | 0 | tick_limit | 9 | 3 | 3 | 0 | 4 | 5 | 21 | 43 | 6 |
| `seed123-social-d1.50` | 0 | tick_limit | 11 | 1 | 1 | 0 | 7 | 4 | 13 | 56 | 7 |
| `seed777-social-d0.15` | 0 | **extinction** | 0 | 12 | 2 | 10 | 0 | 0 | 15 | 51 | 8 |
| `seed777-social-d0.75` | 0 | tick_limit | 11 | 1 | 1 | 0 | 6 | 5 | 8 | 49 | 7 |
| `seed777-social-d1.50` | 0 | tick_limit | 10 | 2 | 2 | 0 | 7 | 3 | 12 | 27 | 8 |

102 survivors and 78 deaths across the fifteen cells, which is 180 — twelve Mokiterions in each,
accounted for on one side or the other with none left over. A death is attributed to **combat** where a
strike resolved against that Mokiterion in the tick it died and reported `target_died:yes`, and to
**attrition** otherwise; the two are read from the stream rather than from the death event, because
`SPEC-MOK-001` rule 13 gives combat death the existing path, event and finality on purpose, so
`agent_died` cannot and should not tell them apart.

| density | survivors of 60 | combat deaths | attrition deaths | strikes | threats | surrenders |
|---|---:|---:|---:|---:|---:|---:|
| `0.15` | 1 | 5 | 54 | 40 | 379 | 30 |
| `0.75` | 48 | 9 | 3 | 68 | 128 | 36 |
| `1.50` | 53 | 7 | 0 | 48 | 113 | 29 |

**The four extinction cells are all at density `0.15`.** Across all five cells at that density, 59 of
60 Mokiterions die and 54 of the 59 die of attrition, so extinction there is starvation and not combat. `REQ-MOK-049`'s floor of
five survivors is asserted at the default density — `mokiterions-core/tests/viability.rs`'s
`DECLARED_FLOORS` fixes that density and the `social` oracle's own `FLOOR` is `5` — and this
capture's `0.75` rows reproduce that oracle's curve by a wholly separate path: survivors 9, 10, 9, 9
and 11, and combat deaths 1, 2, 3, 2 and 1, so no seed is below the floor and no seed is bloodless. The
oracle runs the engine in-process and counts `target_died:yes`; this reads the released binary's stdout
with a different reader. Two paths, the same five pairs of numbers.

The low-density cells are recorded and not excluded, which is what "on failing seeds too" asks for. They
are not a failure of this change: density `0.15` is the sweep's low end, where `WO-MOK-011` already
recorded the world as barely habitable under `individual`, and the deaths there are starvation.

## 3. Every verb proposed, and every verb applied

The fifteen traced cells, `proposed/applied` per verb. `runs.txt` §2 is this table.

| cell | wait | sleep | eat | move | attack | threaten | fight | retreat | surrender | approach | avoid |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `seed0-d0.15` | 0/0 | 118/118 | 111/111 | 3772/3772 | 4/4 | 56/56 | 0/0 | 0/0 | 4/4 | 162/162 | 181/181 |
| `seed0-d0.75` | 0/0 | 315/315 | 369/369 | 8656/8656 | 8/8 | 11/11 | 0/0 | 2/2 | 5/5 | 412/412 | 1069/**1045** |
| `seed0-d1.50` | 0/0 | 316/316 | 439/439 | 8845/8845 | 5/5 | 5/5 | 0/0 | 2/2 | 2/2 | 404/404 | 991/**978** |
| `seed1-d0.15` | 0/0 | 64/64 | 42/42 | 2099/2099 | 5/5 | 92/92 | 0/0 | 0/0 | 4/4 | 74/74 | 127/127 |
| `seed1-d0.75` | 0/0 | 283/283 | 371/371 | 8409/8409 | 14/14 | 7/7 | 0/0 | 2/2 | 10/10 | 305/305 | 618/618 |
| `seed1-d1.50` | 0/0 | 288/288 | 405/405 | 8264/8264 | 11/11 | 5/5 | 0/0 | 2/2 | 7/7 | 307/307 | 729/729 |
| `seed42-d0.15` | 0/0 | 70/70 | 45/45 | 2123/2123 | 12/12 | 32/32 | 0/0 | 0/0 | 11/11 | 165/165 | 210/210 |
| `seed42-d0.75` | 0/0 | 266/266 | 336/336 | 7707/7707 | 14/14 | 18/18 | 3/3 | 4/4 | 8/8 | 328/328 | 595/595 |
| `seed42-d1.50` | 0/0 | 319/319 | 441/441 | 9016/9016 | 7/7 | 20/20 | 0/0 | 1/1 | 5/5 | 388/388 | 812/812 |
| `seed123-d0.15` | 0/0 | 75/75 | 55/55 | 2182/2182 | 4/4 | 148/148 | 0/0 | 0/0 | 3/3 | 158/158 | 256/256 |
| `seed123-d0.75` | 0/0 | 253/253 | 326/326 | 7463/7463 | 15/15 | 43/43 | 6/6 | 6/6 | 6/6 | 329/329 | 574/574 |
| `seed123-d1.50` | 0/0 | 309/309 | 443/443 | 8778/8778 | 10/10 | 56/56 | 3/3 | 2/2 | 7/7 | 408/408 | 992/**988** |
| `seed777-d0.15` | 0/0 | 68/68 | 44/44 | 2139/2139 | 14/14 | 51/51 | 1/1 | 4/4 | 8/8 | 81/81 | 88/88 |
| `seed777-d0.75` | 0/0 | 315/315 | 389/389 | 8841/8841 | 8/8 | 49/49 | 0/0 | 0/0 | 7/7 | 414/414 | 986/**983** |
| `seed777-d1.50` | 0/0 | 290/290 | 414/414 | 8122/8122 | 12/12 | 27/27 | 0/0 | 2/2 | 8/8 | 357/357 | 788/788 |
| **all fifteen** | **0/0** | 3349/3349 | 4230/4230 | 96416/96416 | 143/143 | 620/620 | 13/13 | 27/27 | 95/95 | 4292/4292 | 9016/**8972** |

118,201 proposals and 118,157 applications, so **44 proposals in 118,201 were rejected**, all of them
`avoid` and all in four cells; §4 is the whole of them. The eleven columns sum to 118,201 on the
proposal side, which is one decision per living Mokiterion per opportunity with nothing unclassified.

Two rows of this table are oracles rather than figures:

- **`wait` is `0/0` in every cell.** `REQ-MOK-048`'s branches 2, 3 and 6 delegate to rule 19, which
  never waits, and branches 1, 4 and 5 derive a verb. `mokiterions-core/tests/decisions.rs` asserts the
  same zero over the same five seeds at the default density; this widens it to all three densities.
- **Each of the seven targeted verbs applies somewhere.** `fight` applies 13 times and `retreat` 27,
  the two scarcest, and both are concentrated in the cells where a strike met a Mokiterion whose `fear`
  was still low — `seed123-d0.75` alone carries 6 of the 13 fights. This is oracle 4's reachability
  claim measured over the whole matrix rather than at the default density: no rule in `SPEC-MOK-001`
  rule 21 is unreachable as rule 26 orders its branches.

## 4. Rejections by reason

| cell | verb | ground | count |
|---|---|---|---:|
| `seed0-social-d0.75-traceon` | `avoid` | `out_of_bounds` | 24 |
| `seed0-social-d1.50-traceon` | `avoid` | `out_of_bounds` | 13 |
| `seed123-social-d1.50-traceon` | `avoid` | `out_of_bounds` | 4 |
| `seed777-social-d0.75-traceon` | `avoid` | `out_of_bounds` | 3 |
| | | **total** | **44** |

**One verb and one ground, in 118,201 decisions.** Ten of the eleven verbs are never rejected, and the
eight grounds of rule 6 other than `out_of_bounds` never occur.

The mechanism is a wall. `SPEC-MOK-001` rule 21 makes `avoid` "rule 5 case 3's axis rule, unchanged, in
the opposite direction", and that rule chooses one axis and falls back to the other **when the preferred
one is invalid** — but "invalid" there is the axis having no component, not the step leaving the world.
A Mokiterion against an edge, fleeing a target that is inland of it, has an away-step that leaves the
world on the preferred axis and no away-component on the other, so the proposal is made and rule 6
rejects it. All 44 occur at nine distinct cells and **every one of the nine is on a world boundary** —
`x=0`, `x=127`, `y=0` or `y=127` — and eight are pinned against a single edge while one,
`(127, 127)`, is the corner itself:

    13 at (90,127)   8 at (127,127)   6 at (127,57)   5 at (5,127)   4 at (127,72)
     3 at (77,0)     2 at (4,127)     2 at (127,105)  1 at (0,72)

The run of consecutive ticks is the same Mokiterion pinned in place: at `seed0-d0.75`, `M12` sits at
`(127, 127)` and proposes a rejected `avoid` of `M11` on ticks 495 to 498 and beyond, spending each
opportunity, its energy falling by one a tick. The proposal is rejected, no state changes, and the
trace line carries both the verb and the ground, which is `SPEC-MOK-001` rule 7 doing what
`post/reads.md` and `mokiterions-tui/tests/verification.rs`'s
`the_social_source_is_rejected_only_as_the_specification_admits` require of it: the rejection is
visible, and the ground is one of rule 6's nine rather than a phrase of the observer's own.

**This is the run-level counterpart of the rename `test-census-reconciliation.md` §3 records.** The old
test name asserted that no shipped decision source has a proposal rejected. Here is the fourth source's
rejection count and its single ground, measured: 44, `avoid`, `out_of_bounds`. The property the three
older sources have is not a property of this one, by design, and this table is what replaces the
assertion the name used to carry.

## 5. Branch 1's accepted cost, measured

`SPEC-MOK-001` rule 26 states that branch 1 "answers the first attack in the record whether or not that
answer can succeed", names the two ways an answer can fail — the attacker died at a third Mokiterion's
hands, or a `fight` target moved out of contact — accepts the cost of a spent turn, and says
"`VER-MOK-012` measures it rather than assuming it stays rare". This is that measurement.

| | |
|---|---:|
| answers proposed by branch 1 across the fifteen traced cells | **135** |
| of which `fight` / `retreat` / `surrender` | 13 / 27 / 95 |
| answers naming a Mokiterion that was **not living** when the answer was proposed | **0** |
| answers rejected on any ground | **0** |
| rejections on `target_dead`, `target_unknown` or `out_of_contact`, from any branch | **0** |

So the accepted cost did not materialize anywhere in the declared matrix: every one of the 135 answers
named a living target, and the 13 `fight`s all found their attacker still in contact. The failure paths
are not unreachable — `validate_targeted` returns `target_dead` and `mokiterions-core`'s constructed
cases exercise it, including the one that kills a striker and then refuses every targeted proposal
naming it — they simply do not occur over 15,000 ticks of the declared matrix. Recorded as **not
observed**, which is what the rule asked for and is a different claim from **cannot happen**.

The reader is careful about one thing here, because getting it wrong would have inverted this table: a
target killed by the very action being traced is already out of the living set by the time its killer's
trace line is emitted, since rule 22 resolves damage inside the striker's turn and rule 7 puts the trace
after the action applies. Twenty-one strikes in this matrix are fatal, and treating their targets as
already dead at the moment of the decision would have reported 21 phantom decisions against dead
Mokiterions. `analysis/runs.py` states the case where it handles it and tells it apart from a death by
rule 13's decay, which genuinely precedes the next decision.

## 6. Encounters per seed

An **encounter** is defined by the reader, because the specification does not define one: a maximal run
of consecutive ticks in which one unordered pair of living Mokiterions is in contact — rule 20's
Chebyshev distance of `1` — with a pair counted as in contact in a tick if it was in contact at any
moment within that tick. `analysis/runs.py`'s docstring states the definition and why the "any moment"
is deliberate.

| cell | encounters | contact ticks | mean length | longest | strikes | threats | surrenders | deaths | silent | orphans |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `seed0-d0.15` | 3 | 38 | 12.67 | 29 | 4 | 56 | 4 | 0 | 0 | 0 |
| `seed0-d0.75` | 10 | 96 | 9.60 | 45 | 8 | 11 | 5 | 1 | 3 | 0 |
| `seed0-d1.50` | 6 | 56 | 9.33 | 24 | 5 | 5 | 2 | 1 | 1 | 0 |
| `seed1-d0.15` | 5 | 75 | 15.00 | 55 | 5 | 92 | 4 | 1 | 0 | 0 |
| `seed1-d0.75` | 6 | 28 | 4.67 | 6 | 14 | 7 | 10 | 2 | 0 | 0 |
| `seed1-d1.50` | 5 | 29 | 5.80 | 8 | 11 | 5 | 7 | 2 | 0 | 0 |
| `seed42-d0.15` | 5 | 53 | 10.60 | 18 | 12 | 32 | 11 | 1 | 0 | 0 |
| `seed42-d0.75` | 10 | 64 | 6.40 | 12 | 17 | 18 | 8 | 2 | 1 | 0 |
| `seed42-d1.50` | 10 | 60 | 6.00 | 13 | 7 | 20 | 5 | 1 | 5 | 0 |
| `seed123-d0.15` | 3 | 120 | 40.00 | 90 | 4 | 148 | 3 | 1 | 0 | 0 |
| `seed123-d0.75` | 8 | 79 | 9.88 | 18 | 21 | 43 | 6 | 3 | 1 | 0 |
| `seed123-d1.50` | 13 | 91 | 7.00 | 27 | 13 | 56 | 7 | 1 | 4 | 0 |
| `seed777-d0.15` | 4 | 47 | 11.75 | 27 | 15 | 51 | 8 | 2 | 0 | 0 |
| `seed777-d0.75` | 16 | 117 | 7.31 | 19 | 8 | 49 | 7 | 1 | 4 | 0 |
| `seed777-d1.50` | 11 | 75 | 6.82 | 20 | 12 | 27 | 8 | 2 | 2 | 0 |
| **all fifteen** | **115** | **1028** | **8.94** | **90** | **156** | **620** | **95** | **21** | **21** | **0** |

**115 encounters over 15,000 ticks**, holding every one of the 156 strikes, 620 threats, 95 surrenders
and 21 combat deaths the matrix produced. The orphan column is the check on the definition rather than a
finding: a resolution is attributed to the episode holding both its tick and its pair, and **not one of
the 871 resolutions falls outside an episode**. Twenty-one encounters resolve nothing at all — two
Mokiterions adjacent for a few ticks, both below the thresholds that make a verb — and those are the
reason the mean is worth stating beside the count. `branches.md` reads the strikes-per-encounter
distribution and rules 23 and 24 at their boundaries.

## 7. What this file does not establish

- It is not a verification verdict. `VER-MOK-012` is the contract, `VREC-MOK-012` will be the record,
  and neither is written by the implementation.
- The encounter definition is this reader's, stated because the specification has none. A different
  maximal-run rule — pairs by tick rather than by moment, say — would move the 115 and the 8.94 without
  moving any resolution count, all of which are read from events directly.
- Nothing here measures `REQ-MOK-051`, which is unimplemented under the owner's approved deferral. The
  waste ceiling is unchanged in these runs and `escalation.md` records why.
- The `0.15` cells are recorded, not explained. Whether a world that starves eleven of twelve
  Mokiterions at that density is the intended low end is a product question `WO-MOK-011` already put and
  this work order does not reopen.
