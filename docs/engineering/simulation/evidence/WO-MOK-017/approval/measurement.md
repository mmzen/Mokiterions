# WO-MOK-017 approval measurement: the correction surface swept against the ceiling and the floors

Taken 2026-08-21 on branch `feature/resource-composition-ceiling`, at `master`'s commit `7f4792a`, before any governance
amendment and before any engine change. This is the measurement `WO-MOK-017`'s *Approval preconditions* reserve the
mechanism decision to, and the measurement `REQ-MOK-060`'s *Open decisions* reserved the ceiling's amendment to.

**It answers one question: is there a point inside `REQ-MOK-060`'s permitted surface that meets the ceiling while
`REQ-MOK-014`'s and `REQ-MOK-034`'s survivor floors of eight and `REQ-MOK-058`'s floor of five hold?** At one half there
is not. That is `WO-MOK-017` stop conditions 4 and 5 firing together, and it is why the owner amended the ceiling rather
than the implementation choosing a mechanism.

## What was measured on, and why the build is temporary

The permitted surface is `SPEC-MOK-001` rule 5's non-waste condition and rule 19's tolerant form of it. A sweep needs to
evaluate many candidate conditions, so `probe-017.patch` parameterizes both from the environment. **It is a probe and not
a candidate implementation.** It is retained here in full, with every script that drove it, because `WO-MOK-016`
established that a figure from a build that no longer exists is not re-derivable by anyone and therefore is not evidence.
The patch was applied, the release binary built, every figure below re-derived from that binary into the `raw-*.txt` files
beside this record, and the patch then reverted — `git diff` on `mokiterions-core/` reports no change, and the control run
reproduces `master`'s behavior through the parameterized path, which is what makes the probe a measurement of the world
rather than of itself.

**Reproducing it:** apply `probe-017.patch`, `cargo build --release`, then run each script from the repository root with
any Python 3.11 or later. Every script is deterministic; no script writes to the repository.

### The five parameters

| Environment variable | Meaning |
|---|---|
| `MOK_PROBE_K` | proportional allowance `K * R / 100` above the attribute maximum, for every class |
| `MOK_PROBE_W` | flat allowance `W`, for every class |
| `MOK_PROBE_HIGH` | additional allowance on high class alone |
| `MOK_PROBE_SQ` | single-term allowance `R * R / denominator` |
| `MOK_PROBE_SHARED` | when `1`, rule 19's first clause is rule 5's corrected condition; when `0`, the literal `S + R <= 100` |

`MOK_PROBE_FLOOR` raises the tolerance rule 19's test evaluates against, which is the second permitted mechanism.
`MOK_PROBE_SHARED` is the parameter that decides whether rule 19's `T = 0` identity with rule 5 survives, so the sweep
measured both settings rather than assuming the answer.

All runs are 1,000 ticks at the default density `0.75`, on the declared verification seeds `0`, `1`, `42`, `777` and
`123`, under `reference`, `individual` and `social` — the 15 cells `REQ-MOK-060` obliges. Composition is read from rule
18's final summary, which is where `REQ-MOK-060` requires it read from.

## The control: the world as it stands

`raw-control.txt`, from `probe-sweep.py control`.

**14 of the 15 obligated cells hold a class above one half. The worst is `81.6%`.** All three floors hold. The one cell
that passes is `reference` on seed `777` at `45.9%` and `46.2%`, which is the same single passing cell `WO-MOK-016`
retained in `evidence/WO-MOK-016/post/composition.md`. The drift is measured, uncorrected, and not seed-specific.

## The wall

`raw-sweep.txt` sweeps the two permitted mechanisms and three allowance shapes; `raw-frontier.txt` scans the two-term
family for points that hold every floor. Roughly 55 parameter points, each on all 15 cells. Breach counts in
`raw-sweep.txt` and `raw-frontier.txt` are against **one half**, the ceiling as it then stood.

- **The tolerance floor alone cannot work, at any value.** At floors `10`, `20`, `30` and `40` — the last being the top of
  the trait's whole range — 14 of 15 cells still breach. The reason is structural rather than numeric: `reference` does
  not read rule 19's test at all, so no floor reaches a third of the obligated cells. This is what made rule 5 the
  mechanism rather than a preference between two options.
- **Relaxing rule 5 while leaving rule 19's first clause literal is worse than relaxing both.** At `K = 40` it breaches
  15 of 15 — more than the control — because `individual` and `social` keep the unrelaxed test while the world around them
  changes. Rule 19's first clause has to follow rule 5's condition for the correction to reach all three sources, which is
  the same fact that preserves the `T = 0` identity.
- **Pushing the shared proportional allowance meets one half only after the floors have collapsed.** `K = 40` holds every
  floor and breaches 15 of 15 at `79.7%`. The first points with no breach are `K = 85` and `K = 90`, where `reference` and
  `individual` fall to 4 and 2 survivors against a floor of 8, and `social` to 3 against 5. Every intermediate point is on
  one side of the wall or the other.
- **The flat and high-class-only shapes hit the same wall differently.** Flat `W <= 20` holds the floors and breaches 15
  of 15; `W >= 30` breaks eight or more floors. A high-class-only allowance moves the drift into medium class instead of
  removing it — at `H = 30` and above the dominant class is medium and the worst share climbs back to `73.1%`.
- **The best two-term point that holds all three floors is `25R/100 + 10` on high**, at a worst share of `59.3%` with
  survivors 8, 8 and 6.
- **The closest point that meets one half on all 15 cells is `45R/100 + 10` on high**, at a worst share of `48.8%` — and
  it leaves 6, 6 and 3 survivors, so `REQ-MOK-014`, `REQ-MOK-034` and `REQ-MOK-058` all fail. `raw-detail-two-points.txt`
  carries both points cell by cell, and it shows on its face what killed the floor-amendment alternative: at that point
  the `reference` and `individual` blocks are **identical row for row on all five declared seeds** — same survivors, same
  per-class counts in both territories — so `waste_tolerance` no longer changes any outcome and `CAP-MOK-006`'s
  individuality is void. A relaxation that large masks the trait entirely. The same two blocks differ on every seed at the
  best floor-respecting point above it and at the ratified condition.

**No measured point meets one half while the floors hold.** The best floor-respecting point found anywhere in the sweep is
the condition ratified below, at `54.1%` — four points above one half.

## The ratified condition

`raw-shape.txt` compares candidate arithmetic against the amended ceiling of `60%`; `raw-ratified.txt` carries the chosen
form cell by cell.

**`S + R <= 100`, or else `S + R - 100 <= R * R / 100`.** This is rule 19's own arithmetic at a tolerance equal to the
restoration itself, so it adds no constant the specification did not have. It grants `2` satiety of allowance for low
class, `9` for medium and `25` for high, moving the satiety at which a resource is eaten and approached from `85`, `70`
and `50` to `87`, `79` and `75`.

| Measured | Result |
|---|---|
| Cells above three fifths | **0 of 15** |
| Worst class share | **`54.1%`** — `social` seed `1`, territory B, high class at 33 of 61 |
| Smallest class share | `14.3%` — `individual` seed `42`, territory B, low class at 8 of 56 |
| Minimum survivors | **8 `reference`, 8 `individual`, 7 `social`** against floors 8, 8 and 5 |
| Degenerate territories | none: no class reaches zero on any obligated cell |
| Trait still live | `reference` and `individual` differ on **5 of 5** declared seeds |
| `baseline` moved | **0 of 20** summaries compared across probe settings |

Against the amended ceiling, `R * R / 80` and `R * R / 60` also clear it but break 4 and 9 floors. `R * R / 120` breaks
the ceiling on the declared seeds, 4 of 15 cells over `60%` at `67.2%`. The two-term forms that clear `60%` do so with a
worse worst share and a class-specific constant. So the ratified form is not the only one that satisfies both
obligations, and it is the best of those that do.

**There is no margin on two of the three floors.** `reference` and `individual` land exactly on 8. That is stated here
because it is the figure `WO-MOK-017` stop condition 5 will be evaluated against on the shipped build, and a form that
sits on a floor in a probe can miss it in an implementation that differs in any respect.

## What this measurement does not settle

- **It is not the re-measurement `WO-MOK-017` owes.** These figures come from a probe build that has been reverted. The
  floors and the ceiling are re-measured on the implementation, at that commit, and stop condition 5 is in force against
  8, 8 and 5 as they stand.
- **It says nothing about `baseline` beyond 20 summary comparisons.** The byte-identity obligation is 30 declared cells
  compared byte for byte, at the implementation's commit.
- **It does not decide the per-class floor.** The product owner reserved that again to the post-change composition. What
  it shows is that the composition has not inverted: the dominant class is still high or medium on every obligated cell.

## Disclosure: the 50 unbound seeds

`raw-unbound-seeds.txt`. **No obligation binds the seeds `0`–`49`**, and none is read here as if one did. The comparison
is recorded because the ratified form was chosen against shapes that trade the other way, and the owner saw this before
ratifying.

| Over 150 cells, seeds `0`–`49` | Today | Corrected |
|---|---|---|
| Cells above `60%` | 140 | **7** |
| Cells above `50%` | 149 | 61 |
| Worst class share | `88.5%` | **`63.9%`** |
| Cells below their source's floor | 6 | **17** |
| Lowest survivors seen | 5 / 7 / 3 | 5 / 5 / 4 |

The correction is a large improvement in composition and a small regression in the survivor tail off the declared seeds.
`R * R / 120` trades the other way and was measured to break the ceiling on the declared seeds, so the trade is not
avoidable within this shape. Recorded as a disclosure and not as a finding against an obligation.

## Files

| File | What it is |
|---|---|
| `probe-017.patch` | the temporary parameterization of both permitted mechanisms; applied and reverted, retained verbatim |
| `probe-sweep.py` | the harness: one cell is one run, read from rule 18's summary; `control`, `flooronly`, `rule5only`, `sweep`, `fine`, `flat`, `high`, `detail` |
| `probe-scan.py` | Pareto scan over the two-term family against breach count, worst share and floor deficit |
| `probe-frontier.py` | the best two-term points that miss no floor |
| `probe-detail.py` | cell-by-cell detail for the two decision-relevant points, and the `baseline` no-movement check |
| `probe-shape.py` | candidate arithmetic shapes against the amended ceiling of `60%` |
| `probe-verify.py` | the ratified condition: per-cell detail, the trait-still-live check, the unbound-seed survey |
| `probe-control-robust.py` | today's world against the corrected one over the 50 unbound seeds |
| `raw-control.txt` | the control: `master`'s behavior through the parameterized build |
| `raw-sweep.txt` | the six sweeps, against one half |
| `raw-frontier.txt` | the floor-respecting frontier |
| `raw-detail-two-points.txt` | the best floor-respecting two-term point and the closest ceiling-meeting point |
| `raw-shape.txt` | the arithmetic shapes against `60%` |
| `raw-ratified.txt` | the ratified condition, all 15 cells |
| `raw-unbound-seeds.txt` | the unbound-seed disclosure |

## Authority

**This measurement approves nothing.** It is the evidence four decisions of 2026-08-21 were taken on, each by the
repository owner in a named role and each recorded in the artifact it belongs to: `REQ-MOK-060`'s amendment record for the
ceiling, `SPEC-MOK-001`'s for the numeric form and the preserved `T = 0` identity, `VER-MOK-016`'s for the realigned
oracle, and `WO-MOK-017`'s *Approval decisions of 2026-08-21* for all five together with the approval itself. The
implementation agent measured the options and wrote the records; it decided none of the substance.
