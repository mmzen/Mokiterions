# WO-MOK-004 evidence: resilience and viability after the help change

`VER-MOK-004` requires the per-seed survivor counts to be identical to the baseline "rather than merely
above the floor", and a 10,000-tick run under each decision source with survivors plus deaths equal to
twelve at the same termination ticks as recorded before.

A help-text change has no mechanism for perturbing a simulation. These checks are run anyway because
that reasoning is exactly what would be offered if the numbers had moved, and because the survivor floor
has no margin: two declared seeds sit on it with zero margin.

## Viability at the declared density

`VER-MOK-002` fixes the seed set at `0, 1, 42, 123, 777`, the density at `0.75%`, and the floor at 8
living Mokiterions after 1,000 ticks under the reference source. The figures come from the same
`capture.sh` invocation that produced the rest of `after/`, so they are the same bytes the equivalence
comparison used rather than a separate run that happens to agree.

| Seed | Reason | Ticks | Survivors | Floor | Margin | Baseline |
| ---: | --- | ---: | ---: | ---: | ---: | --- |
| 0 | tick limit | 1,000 | 8 | 8 | **0** | identical |
| 1 | tick limit | 1,000 | 11 | 8 | 3 | identical |
| 42 | tick limit | 1,000 | 8 | 8 | **0** | identical |
| 123 | tick limit | 1,000 | 9 | 8 | 1 | identical |
| 777 | tick limit | 1,000 | 11 | 8 | 3 | identical |

`8, 11, 8, 9, 11` — the counts `VER-MOK-004` fixed in advance and the counts `VREC-MOK-002` and
`VREC-MOK-003` recorded. The whole summary line matches in each case, including per-territory population
and per-class resource counts, not merely the survivor total.

## Resilience at 10,000 ticks

Seed `123`, ten times the verified horizon, all four combinations of source and swept density. Raw
transcript: `resilience-10k.txt`.

| Policy | Density | Reason | Ticks | Survivors | Deaths | Survivors + deaths | Exit |
| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: |
| reference | `0.75%` | extinction | 9,154 | 0 | 12 | **12** | 0 |
| reference | `1.50%` | tick limit | 10,000 | 7 | 5 | **12** | 0 |
| baseline | `0.75%` | extinction | 168 | 0 | 12 | **12** | 0 |
| baseline | `1.50%` | extinction | 119 | 0 | 12 | **12** | 0 |

The four summary lines are **byte-identical** to `WO-MOK-003/resilience-10k.txt`, verified with `diff`.
Conservation holds in every run, extinction still takes precedence over the tick limit and still
terminates cleanly, and no run panicked or grew unboundedly.

### A wrong first attempt, recorded

The first attempt at these runs used `--seed 42` and produced four rows that did not match
`WO-MOK-003`'s record: reference at `0.75%` reached the tick limit with one survivor instead of going
extinct at tick 9,154, and the two baseline extinctions landed at ticks 142 and 169 instead of 168 and
119.

The cause was the invocation, not the program. `WO-MOK-003`'s `resilience-and-viability.md` fixes the
seed at `123`; its `resilience-10k.txt` records only policy and density, so the seed had to be read from
the prose. Re-run at seed `123`, all four rows are byte-identical.

This is recorded rather than quietly discarded because for a few minutes it looked like an unexplained
divergence in a change that must not perturb the simulation, which is a stop-and-escalate condition
under `WO-MOK-004`. The resolution is that the comparison was invalid, not that the result was
acceptable. The lesson for the next raw transcript: record the full argument vector in the file, not only
in the prose beside it.

## Cost

| Measure | Effect |
| --- | --- |
| Runtime work per tick | unchanged; follows from byte-for-byte equivalence on the 43-cell matrix |
| Memory | unchanged |
| Simulation output volume | unchanged |
| Help output | 700 → 1,326 bytes, once per `--help` invocation |
| Diagnostic output | +626 bytes per invalid invocation, bounded and non-recurring |
| Binary size | one longer `&'static str`; `concat!` is evaluated at compile time, so there is no runtime cost |

Wall-clock figures for the four long runs are in `resilience-10k.txt`. They are not compared against the
baseline: they are single unrepeated measurements on a loaded desktop, and `VER-MOK-004` attaches no
performance obligation to them.
