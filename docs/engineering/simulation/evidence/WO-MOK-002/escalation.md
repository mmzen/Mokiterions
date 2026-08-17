# Escalation — WO-MOK-002

Two escalations are recorded here. **Both are closed.** No escalation is open, and no owner
decision is outstanding.

---

# Escalation 2 (CLOSED) — the amended floors did not describe the specified system

Closed 2026-08-17 by the owner decision recorded at the end of this section. Retained in full,
including the options that were not chosen, because it is the record of a measurement error of
mine that reached an approved artifact.

## Status as escalated

Declared stop condition: measured survivors fall below the floor stated for a declared density.

| Density | Stated floor | Measured worst case | Failing seed |
|---:|---:|---:|---:|
| `0.75%`, the default | 5 | **3** | 0 |
| `1.50%` | 8 | **7** | 777 |

Both floors fail. Full data in `calibration-record.md` and `density-curve.md`.

`REQ-MOK-013` and `REQ-MOK-015` are implemented and fully verified. Every non-floor row of
`REQ-MOK-014` passes: the density mapping resolves to the specified counts, density binds
initialization, capacity, and the replenishment target, a zero-resource density is rejected
with exit code 2, conditional regeneration is preserved, and consumption is non-zero on every
seed at both densities.

## Cause: my measurement error, not a code defect

The implementation matches amended `SPEC-MOK-001` rule 5 exactly, including the non-waste
correction to case 1. The defect is in how the floors were chosen.

**The curve I gave you was measured on a tree that did not contain the rule 5 correction.** The
2026-08-17 amendment mandates two changes together: density as an input, and the correction of
case 1 from a fixed satiety threshold to the non-waste rule. I measured the density sweep with
only the first applied, then presented that curve as the basis for the floors. The correction
changes which resource is eaten when, so it changes trajectories wholesale, and the curve moved
under the floors as soon as it was applied.

This was confirmed rather than inferred. Reverting case 1 to the threshold form in a scratch
build reproduced the superseded curve exactly, 6/7/7/5/5 at `0.75%` and 9/12/12/10/12 at
`1.50%`. The scratch build was reverted and `src/simulation.rs` verified byte-identical
afterwards. The correction is the sole cause.

I own this. You approved floors from evidence I produced, and the evidence was not of the system
the same decision specified.

## A specification defect found while verifying the correction

The correction does not do what `SPEC-MOK-001` §5 says it does, and the sentence that says so is
mine.

Case 1 eats a co-located resource when its satiety restoration is not clipped at `100`:

| Class | Satiety restored | Eatable at satiety of at most |
|---|---:|---:|
| Low | 15 | 85 |
| Medium | 30 | 70 |
| High | 50 | **50** |

For High class the non-waste condition is **numerically identical to the fixed threshold of
`50` it replaced.** The spec claims the correction "removes both effects", meaning the dead
satiety buffer and the two-cell oscillation. For the richest third of the resource table it
removes neither.

The oscillation is therefore still present, measured at **35.7% of all agent-ticks** on seed
`42` at the default density. The mechanism, from `manual-observation.md`:

| Standing at | Nearest resource at distance > 0 | Proposed |
|---|---|---|
| `6:3` | F0015, High, at `7:3`, distance 1 | `move:east` |
| `7:3` | F0042, High, at `4:3`, distance 3 | `move:west` |

At `7:3` the Mokiterion is on F0015. Case 1 declines it because High would clip. Case 3 excludes
it because case 3 only targets resources at distance greater than zero, and picks the next
nearest, which is back west. Stepping west makes F0015 nearest again. Stable cycle.

The root cause is not the eat condition at all. It is that case 3 re-targets a resource the
Mokiterion is standing on the instant it steps off. Correcting case 1 could never have fixed
that.

Assessed honestly, the behavior is not purely harmful: an oscillating Mokiterion is camped one
step from a rich resource and will eat it when satiety reaches 50, and movement costs no energy.
What it loses is every other option, including reaching a resource that fits now, and it means
High-class resources are still consumed on the schedule the amendment intended to replace.

No current requirement prohibits it. It is recorded as an adverse observation, not a
requirement failure. Fixing it requires changing specified ranking behavior, which
`WO-MOK-002` does not authorize.

## Options

Each needs an amended `REQ-MOK-014`, which is a product-owner artifact, and options C and D also
need an amended `SPEC-MOK-001`. None is adopted. No constant was retuned and the failing test
was left failing.

**Option A — restate both floors from the corrected curve.** Default `0.75%` → floor 3;
`1.50%` → floor 7. Smallest change, preserves the structure you chose, and closes this work
order today. Cost: both floors sit exactly on the measured worst case, so there is zero margin,
and a floor of 3 of 12 is a thin claim to make about habitability.

**Option B — keep the floor of eight, at the density that delivers it.** Default `0.75%` →
floor 3; replace the `1.50%` row with `1.00%`, which measures a worst case of exactly 8. Keeps
the number you originally wanted as the headroom claim. Cost: also zero margin, and `1.00%` was
measured at a worst case of 2 on the superseded curve, which is a sharp reminder of how little
the row means away from the measured point.

**Option C — fix the specification defect first, then measure once, then set floors.** Amend
rule 5 so case 3 cannot re-target the cell just left, or so case 1 may consume a clipping
resource when nothing better is reachable. Re-measure the curve and choose floors from it. Cost:
one more amendment cycle before this work order can close. Benefit: any floor set under A or B
will be invalidated by this fix, because it changes 36% of agent-ticks. C sets floors once
instead of twice, and it removes a false claim currently standing in an approved specification.

**Option D — split the work order.** Transition perception and the reference source to
`implemented` under a narrowed `WO-MOK-002`, and move `REQ-MOK-014` to a successor work order
carrying the density calibration and the rule 5 defect together. Cost: the same objection as
last time, that a survivor count is meaningless without a stated policy, so splitting defers
the only requirement that tests whether the world works.

## Recommendation

**Option C.** Two reasons, neither of which is that C is tidier.

First, `SPEC-MOK-001` currently contains a sentence that is false, and I wrote it. That should
not stay in an approved artifact regardless of what happens to the floors.

Second, the oscillation governs 36% of agent-ticks. Any floor chosen from today's curve is a
floor for a system with that inefficiency baked in, and fixing it later will move the curve
again and force `REQ-MOK-014` back through product approval a third time. Setting floors after
the fix costs one cycle; setting them before costs two.

If you would rather close this work order now and treat the oscillation as Phase 2 scope, Option
A is the honest version of that: state the floors that were actually measured, with the zero
margin recorded explicitly, and accept that the Phase 2 fix will require re-approval.

I recommend against Option B. Keeping the number 8 because it was the original intent, on a row
chosen because it happens to produce 8, is fitting the requirement to the measurement while
appearing not to.

## What I did not do while this was open

- No constant, threshold, density, or floor was changed in the committed tree to make a
  measurement pass.
- Every scratch variant was reverted and the source verified byte-identical afterwards.
- The failing viability test was left failing rather than relaxed, ignored, or deleted.
- `WO-MOK-002` was not transitioned to `implemented`.
- Nothing was committed, pushed, tagged, or released.

## Resolution, 2026-08-17

**The owner chose Option C, and additionally reduced the declared density set to one row.**

The specification was fixed first, so the floors were chosen once from a curve measured on the
corrected system. `SPEC-MOK-001` rule 5 now applies the non-waste test to case 3 as well as case
1, so a Mokiterion neither eats nor approaches a resource whose restoration would be clipped, and
falls through to a search step when nothing perceived fits. The unreachable fallback clause was
deleted from case 1. The false sentence was retracted in place rather than quietly edited.

`REQ-MOK-014` now declares one density, the default `0.75%`, with a floor of **eight of twelve**.
The floor went up, not down: it is the number the product owner wanted from the outset, and the
corrected rule delivers it at the scarce default. The `1.50%` headroom row was withdrawn because
it retains all twelve survivors on three of five seeds, which its own verification would report as
an adverse observation; it remains a swept comparison point carrying no obligation.

### What the fix measured

| Density | Worst case before | Worst case after |
|---:|---:|---:|
| `0.50%` | 1 | 6 |
| `0.75%`, the default | **3** | **8** |
| `1.00%` | 8 | 10 |
| `1.50%` | 7 | 11 |

Per-seed at the declared density: 8, 11, 8, 9, 11. The floor is met on every declared seed with
zero margin, which `VER-MOK-002` records as deliberate. The oscillation fell from 35.7% of
agent-ticks to 10.6%, below the 12.2% rate an unbiased cardinal walk produces on the same world,
so the defect is removed rather than reduced. Earliest death moved from tick 212 to tick 244, and
on one seed the first death is at tick 604.

### The cost, disclosed before the decision and accepted

Because a high-class resource restores `50` satiety, the corrected rule makes it approachable only
at satiety of at most `50`. High-class resources are therefore sought less often and accumulate
against the capacity density fixes: roughly three quarters of standing supply by tick 1,000, where
the previous rule left the mix near balanced. Territories stay full while the food anyone will
walk to becomes scarce, and the 10,000-tick run at the default density now reaches **extinction at
tick 9,154** where the previous rule left one survivor.

This was measured and presented before the decision, not discovered afterwards. The owner accepted
it on the grounds that no requirement in scope speaks past tick 1,000 and the correction is
decisively better at the horizon that is verified. It is recorded as residual uncertainty in
`VER-MOK-002`, as a deferred matter in `REQ-MOK-014`, as a specified consequence in
`SPEC-MOK-001` rule 5, and as Phase 2 scope in the functional roadmap. Any future long-horizon
stability requirement will need a rule that lets high-class resources be consumed, and that will
move the density curve and require `REQ-MOK-014` to be re-approved.

### Status

Both escalations are closed. `WO-MOK-002` moves to `implemented`. All 52 tests pass; the viability
test that was deliberately left failing now passes against a higher floor. Nothing has been
committed, pushed, tagged, or released, and no verification record has been captured — those
remain owner decisions.

---

# Escalation 1 (CLOSED) — the original absolute floor was unsatisfiable

## Status

Closed 2026-08-17 by the owner decision that made density an input and restated `REQ-MOK-014`
as a conditional obligation. Retained as the record of how the density design was reached.

Original stop condition: escalate if measured survivors fall below eight on any declared seed.
Measured survivors were **zero on every declared seed** at the density the original
`SPEC-MOK-001` fixed by constant, `0.15%`.

## What was decided in the packet, and why it was wrong

The 2026-08-17 amendment to `SPEC-MOK-001` was calibrated on a supply-versus-demand argument: at
satiety decay `1`, twelve Mokiterions demand 12 satiety per tick, while regeneration of `2`
resources per territory per ten ticks at a mean of 31.67 satiety supplies about 12.67 per tick, a
gross margin of about 1.05x. That argument is arithmetically correct and operationally
irrelevant. It assumes every regenerated resource is found and eaten.

Two measurements falsified the premise:

1. **Raising regeneration yield from `2` to `8` saved nobody on any seed.** Supply was not the
   binding constraint. The territory capacity constant was the true ceiling on standing supply,
   and territories sat at that ceiling for a third to two thirds of all regeneration
   opportunities, discarding the surplus. The amendment raised the lever that does nothing and
   left the lever that governs the outcome untouched.
2. **Raising perception radius from `16` to `96`, effectively global vision, yielded at most two
   survivors.** Neither seeing food nor searching for it faster was the constraint.

The binding constraint was the ratio of travel time to satiety drain, evaluated during the
regeneration ramp from an endowment of three toward capacity. Full measurements are in
`lever-sensitivity-study.md`.

## How it was closed

Density became an explicit input binding initialization, capacity, and the replenishment target
together, which removed the ramp. `REQ-MOK-014` became a conditional obligation tabulated over
declared densities. Deaths moved from ticks 119 to 229 out to tick 212 at the earliest, which
confirms the ramp was the mechanism.

The floors chosen at that closure are the subject of Escalation 2 above.
