# WO-MOK-003 evidence: resilience and viability after relocation

`VER-MOK-001` requires at least one 10,000-tick seeded run to expose overflow, unbounded state growth,
or lifecycle failure. `VER-MOK-002` requires a 10,000-tick run **under each source** with survivors
plus deaths always equal to twelve, and requires the survivor floor at the declared density on the
five declared seeds.

Both were verified before this work order. They are re-run here because a refactor that moved code
between compilation units is exactly the kind of change that could perturb a long run without
perturbing a short one, and because the survivor floor has no margin: `VER-MOK-002` records that the
floor of eight sits exactly on the measured worst case of eight, reached on more than one declared
seed. A one-survivor drift would be a requirement failure, not a rounding difference.

## Resilience at 10,000 ticks

Seed `123`, ten times the verified horizon, all four combinations of source and swept density. Raw
transcript: `resilience-10k.txt`.

| Policy | Density | Reason | Ticks | Survivors | Deaths | Survivors + deaths | Exit |
| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: |
| reference | `0.75%` | extinction | 9,154 | 0 | 12 | **12** | 0 |
| reference | `1.50%` | tick limit | 10,000 | 7 | 5 | **12** | 0 |
| baseline | `0.75%` | extinction | 168 | 0 | 12 | **12** | 0 |
| baseline | `1.50%` | extinction | 119 | 0 | 12 | **12** | 0 |

Every row is identical to the corresponding row of `WO-MOK-002/determinism-and-resilience.md`,
including the tick at which each run ends. So is the end-of-run resource mix: the reference run at the
default density ends with `5 / 11 / 45` in territory A and `12 / 4 / 45` in territory B by calorie
class, matching that record's figures.

What this establishes:

- **No panic, no overflow, no unbounded growth.** Four runs, one of them 10,000 full ticks, all exit
  `0` with exactly one summary line.
- **Conservation holds.** Survivors plus deaths equal twelve in every run, which is the invariant
  `VER-MOK-002` states for this check.
- **Extinction still takes precedence over the tick limit** and still terminates cleanly, in the three
  runs that reach it.
- **Termination ticks are unchanged**, which is the stronger claim. Two independent runs agreeing on
  the exact tick of extinction after nine thousand ticks of divergent opportunity means the entropy
  stream, the decision order, the survival arithmetic, and the regeneration schedule are all
  bit-identical. That is far more sensitive than "did not panic".

The reference source starving at the default density by tick 9,154 is pre-existing, accepted behavior,
not a finding of this work order: `VER-MOK-002`'s residual uncertainty records it, the product owner
accepted it on 2026-08-17, and it was deferred to Phase 2. No requirement in scope speaks past tick
1,000. It is reproduced here unchanged, which is the correct outcome for an equivalence-preserving
refactor — a refactor that "fixed" it would have broken rule 11.

Wall-clock times were `467 ms`, `1,092 ms`, `71 ms`, and `76 ms`. No wall-clock target is imposed by
either contract and none is claimed; the figures are recorded only to show the runs are the cheap ones
they were before.

## Viability at the declared density

`VER-MOK-002` fixes the seed set at `0, 1, 42, 123, 777` and the density carrying a floor at `0.75%`,
the default, with a floor of **8 living Mokiterions** after 1,000 ticks under the reference source.

The verifying test, `the_reference_source_sustains_the_population_at_every_declared_density`, is one of
the fifteen that moved; it now lives in `tests/viability.rs` and links the library target. It passes.

Per-seed measured survivors, from `after/viability.txt` — byte-identical to `baseline/viability.txt`:

| Seed | Reason | Ticks | Survivors | Floor | Margin |
| ---: | --- | ---: | ---: | ---: | ---: |
| 0 | tick limit | 1,000 | 8 | 8 | **0** |
| 1 | tick limit | 1,000 | 11 | 8 | 3 |
| 42 | tick limit | 1,000 | 8 | 8 | **0** |
| 123 | tick limit | 1,000 | 9 | 8 | 1 |
| 777 | tick limit | 1,000 | 11 | 8 | 3 |

These are the counts `VREC-MOK-002` recorded: `8, 11, 8, 9, 11`. Every seed terminates by tick limit
rather than extinction, every seed meets the floor, and every run reports non-zero food consumption,
which the test also asserts.

`viability.txt` is **byte-identical** between the pre-change and post-change captures, so this is not
merely five runs that still pass — it is five runs whose complete summary lines, including
per-territory population and per-class resource counts, are unchanged. Two seeds sit exactly on the
floor with zero margin, and both still do, at the same tick, with the same distribution. Given that
`VER-MOK-002` warns the floor "is likely to break immediately" under any change to the world model, an
exact match is the only result that would have been acceptable.

## Method note

Both measurements were produced from the finished tree on branch
`feature/library-target-and-test-placement`, using the binary target built by `cargo build`. The
viability figures come from the same `capture.sh` invocation that produced the rest of the `after`
capture, so they are the same bytes the equivalence comparison used rather than a separate run that
happens to agree. The 10,000-tick runs are outside `capture.sh` because they are not part of the
equivalence matrix; they are recorded here in full with the exact policy, density, seed, and tick
limit that produced each line.
