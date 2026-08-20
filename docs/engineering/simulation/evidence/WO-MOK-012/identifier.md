# `identifier.md`: the per-identifier series, the correlations, the band evaluation, and the identifier exchange

| Field | Value |
|---|---|
| Deliverable | `WO-MOK-012` *Evidence to record* — "`identifier.md`, the per-identifier series, the rank correlations, the band evaluation, and the identifier-exchange comparison" |
| Oracle | `VER-MOK-012` oracle 5, both halves, as amended on 2026-08-20 |
| Binary | `target/release/Mokiterions.exe`, built `--locked` at the candidate |
| Sweep | `python …/analysis/identifier.py sweep <binary> 1000 identifier-sweep.json` — 1,000 seeds, 1,000 ticks, `--policy social`, default density |
| Tables | `python …/analysis/identifier.py tables identifier-sweep.json` — every figure below, in a second |
| Retained data | `identifier-sweep.json`, 116 KB, SHA-256 `7d7033c48da4de206dac19a7202cc3a28e2b416b32f996bf43a27792a62e0d76` |
| Date | 2026-08-20 |

The sweep is retained rather than only its figures, because it is 116 KB and it makes every table below re-derivable
without a two-minute re-run — including tables nobody has asked for yet. It was produced twice by two independently
written readers, the second being the script retained here, and the two agree on all 1,000 seeds' three series
exactly. Its per-seed series also agree, at the five declared seeds, with what the in-process test prints.

The decision that shaped this row is `evidence/WO-MOK-012/escalation.md` §11, and the amendment it produced is in
`VER-MOK-012`'s amendment record. **This file is the measurement, not the decision.**

---

## 1. What each seed set carries

| Set | Seeds | Obligation | Reason it exists |
|---|---|---|---|
| Declared | `0, 1, 42, 123, 777` | Part one, the gross-advantage tripwire. Also `REQ-MOK-049`'s floor and lethality bound, `REQ-MOK-014`'s and `REQ-MOK-034`'s floors | Comparability. Every survivor figure in this chain is quoted at these five seeds |
| Declared diagnostic | `0`–`199` | Part two, the turn-position survival bound, and **nothing else** — no floor, no lethality bound, no comparability obligation | The bound cannot be measured on five; §5 measures why |
| Sweep | `0`–`999` | None. Diagnostic | It is what the escalation was decided against, and what fixes the direction and shape of the effect |

The three readers are one set of patterns, shared by this script and `tests/viability.rs` on purpose:

    survival   the absence of `subject=<id> event=agent_died` — which Mokiterion lived, not how many
    applied    `subject=<id> event=attack_resolved`
    suffered   `event=attack_resolved result=target:<id>,`

The `event=` prefix on the third is load-bearing: without it the pattern matches every other verb's
`result=target:` field and over-counts by about threefold. The check that caught that is arithmetic and is asserted in
the script — every strike has exactly one target, so applied and suffered must total the same, and they do at every
seed count below.

## 2. The declared five: part one, the tripwire

Over 1,000 ticks at the default density under `social`:

| Series | `M01` … `M12` | Total | vs identifier | vs turn position |
|---|---|---:|---:|---:|
| survivals | 5, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4 | 48 of 60 | −0.414 | −0.169 |
| attacks applied | 3, 3, 6, 8, 0, 4, 7, 3, 5, 9, 7, 13 | 68 | +0.586 | +0.493 |
| attacks suffered | 3, 0, 7, 5, 0, 8, 5, 8, 7, 9, 7, 9 | 68 | +0.731 | +0.383 |

**No series is monotone non-increasing in identifier, so part one passes.** The gross case it exists to catch — `M01`
surviving every seed while `M12` survives none — is nowhere near: `M01` survives five of five and `M12` four of five,
and `M01` applies three strikes against `M12`'s thirteen.

The two correlations outside `±0.5` are what the removed band failed on. They are recorded and bound nothing. The
in-process test prints exactly these three series and these totals, and its figures and this script's agree to the
unit, which is a check on both: one runs the engine in-process and reads a buffer, the other runs the shipped binary
and reads its stdout.

## 3. The declared 200: part two, the bound

| Series | `M01` … `M12` | Total | vs identifier | vs turn position |
|---|---|---:|---:|---:|
| survivals | 150, 156, 150, 142, 151, 158, 164, 137, 150, 152, 154, 159 | 1,823 of 2,400 | +0.303 | +0.371 |
| attacks applied | 129, 149, 184, 166, 165, 222, 128, 134, 131, 150, 161, 181 | 1,900 | +0.063 | **+1.000** |
| attacks suffered | 178, 146, 179, 213, 175, 153, 116, 174, 145, 149, 157, 115 | 1,900 | −0.580 | +0.086 |

Pooled by turn position within a Mokiterion's own territory — position 1 is `M01` and `M07`, position 6 is `M06` and
`M12` — over 400 survival opportunities per position:

| Turn position | 1st | 2nd | 3rd | 4th | 5th | 6th |
|---|---:|---:|---:|---:|---:|---:|
| attacks applied | 257 | 283 | 315 | 316 | 326 | 403 |
| attacks suffered | 294 | 320 | 324 | 362 | 332 | 268 |
| survivals, of 400 | 314 | 293 | 300 | 294 | 305 | 317 |
| **survival rate** | **0.7850** | 0.7325 | 0.7500 | 0.7350 | 0.7625 | **0.7925** |

| | |
|---|---|
| Highest ÷ lowest | **`1.0819`** — position 6 over position 2 |
| Bound | **`< 1.25`** |
| Verdict | **passes**, with `0.168` of margin |
| Last ÷ first | `1.0096` |

The bound is on the extremes of all six positions and not on last-over-first, so it bounds any pair and keeps working
if the direction reverses. On this set the extremes are not the ends, which is what that provision is for.

## 4. The sweep: the direction and shape of the effect

Diagnostic. 9,194 resolved strikes, 45,791 threats, 6,033 surrenders, 2,712 deaths of which 1,274 were struck to
death, over 12,000 identifier-runs.

| Series | vs identifier `M01`…`M12` | within A, `M01`…`M06` | within B, `M07`…`M12` | pooled by turn position |
|---|---:|---:|---:|---:|
| survivals | +0.084 | +0.486 | +0.829 | **+0.986** |
| attacks applied | +0.601 | +0.943 | **+1.000** | **+1.000** |
| attacks suffered | −0.357 | −0.657 | −0.943 | **−0.943** |
| net strikes | +0.566 | **+1.000** | **+1.000** | **+1.000** |

| Turn position | 1st | 2nd | 3rd | 4th | 5th | 6th |
|---|---:|---:|---:|---:|---:|---:|
| attacks applied | 1,255 | 1,384 | 1,411 | 1,578 | 1,706 | 1,860 |
| attacks suffered | 1,653 | 1,651 | 1,574 | 1,633 | 1,413 | 1,270 |
| survivals, of 12,000 | 1,518 | 1,521 | 1,544 | 1,544 | 1,548 | 1,613 |
| survival rate | 0.7590 | 0.7605 | 0.7720 | 0.7720 | 0.7740 | 0.8065 |

The last actor in a territory strikes **48% more** and is struck **23% less** than the first. Net strikes rank
`+1.000` within each territory independently — twelve identifiers, two territories, no inversion. Survival follows
in the same direction and is **small**: 4.75 percentage points, `1.0626` as a ratio.

**Why the identifier column is the weak one.** Placement is six to a territory by identifier and contact is
overwhelmingly within a territory, so against identifier `1`…`12` the effect is a sawtooth that resets at `M07`. The
reset is what makes `+1.000` on turn position read as `+0.601` on identifier, and `−0.943` read as `−0.357`.
`escalation.md` §11 states the mechanism, from `SPEC-MOK-001`'s own text: a Mokiterion acting later in the pass
observes a world earlier actors have already moved through, so it finds company already in contact at its own turn.

## 5. Why 200 seeds, measured

The 1,000 seeds partitioned into disjoint groups. The left column is the bound's own statistic; the right is the
direction diagnostic, which the bound cannot be, being a spread:

| Group size | Groups | Bound's statistic | Groups that would breach `1.25` | Groups putting the last actor ahead |
|---|---:|---|---:|---:|
| 50 | 20 | `1.068` – `1.284` | **2 of 20** | 16 of 20 |
| 100 | 10 | `1.058` – `1.201` | 0 of 10 | 8 of 10 |
| **200** | **5** | **`1.032` – `1.137`** | **0 of 5** | **5 of 5** |
| 250 | 4 | `1.073` – `1.115` | 0 of 4 | 4 of 4 |
| 500 | 2 | `1.064` – `1.084` | 0 of 2 | 2 of 2 |

**And on the five declared seeds the bound reads `1.2857` and would fail** — ten survival opportunities per position,
one of which differs. So the choice of set is not a convenience: a bound of `1.25` on five seeds would fail on the
world the product owner accepted, at a candidate no correct implementation could make pass. Two hundred is where the
groups stop disagreeing about the direction and stop breaching the bound on noise.

`1.25` itself is not read off the `1.082` it bounds. `REQ-MOK-034` binds the trait-aware source at eight survivors of
twelve and `REQ-MOK-049` binds this source at five, so three of twelve is the survivor cost combat was accepted to
impose; an advantage worth more than that whole quarter is structural rather than residual.

## 6. The identifier exchange: the mechanism half

`mokiterions-core/src/simulation.rs :: exchanging_the_two_identifiers_changes_no_outcome`, internal tier, **passes**.

One constructed encounter is resolved with the two roles exchanged — the same striker attributes, the same target
attributes, the identifiers swapped — and the damage, the striker's energy cost and the resulting attributes are
identical. This is the half that would be a **defect** rather than a finding: damage is a function of the striker's
`energy` and `health` and of nothing else, so any difference would mean an identifier had reached the arithmetic.

The distinction is the whole of `WO-MOK-012` stop condition 6. The exchange test passing and the outcome series
showing an ordered effect are consistent, and together they say what the effect is: **turn order, not identifier**.
Every Mokiterion in position 6 of its territory enjoys it, and `M01` sits in position 1 of A: the **worst** of the six
by survival rate over the sweep at `0.7590`, and the second-best over the declared 200 at `0.7850`. On neither set is
it the best.

## 7. The rule 25 ablation, and what it is not

`escalation.md` §11 records an ablation over 400 seeds in which branch 1 of rule 26 never fires, so no suffered
attack is answered: the ordering by turn position is **unchanged** at `+0.943` and the magnitude spread narrows from
379 net strikes to 139. It bounds what amending `SPEC-MOK-001` rule 25 could buy — about two thirds of the
magnitude, none of the ordering — and that is why option D was declined with its cost known rather than on taste.

**It is not reproducible from a retained script.** The ablation was a throwaway scaffold in
`SocialDecisionSource::decide`, gated on an environment variable, and it was fully reverted; `git status` was clean
before the candidate was built. Rebuilding it is a one-line change and §11 states the shape. It is recorded here as
what it is — a measurement taken on a tree that no longer exists, quoted to bound an option rather than to establish
a property of the shipped engine.

## 8. What this file does not establish

- It does not establish that the advantage is acceptable. That was the product owner's decision of 2026-08-20, taken
  against `escalation.md` §11's four measured options.
- It does not establish that `1.25` is the right bound, only where it came from and that it holds at `1.082` with the
  set it is declared on. Its adequacy is `VER-MOK-012` manual assessment 11, and the assurance owner recorded it.
- It measures survival, strikes applied and strikes suffered. It does not measure resource acquisition, threats
  received or turns spent on unanswerable answers by identifier — those sit in `runs.md` and `branches.md`.
- **`INT-MOK-009`'s risk is stated in the opposite direction to the measurement.** Its text anticipates that the
  acting order "may hand `M01` a systematic advantage"; the advantage runs to the last actor in a territory. Its
  success measure — "not monotonic in identifier" — is met, and amending the risk's direction is the intent owner's
  decision, which `WO-MOK-012` stop condition 11 reserves. It is recorded and unmade.
