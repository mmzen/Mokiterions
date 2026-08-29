# Item 10: the ceiling stop — `VER-MOK-018` cases `A4`, `L18` and `L19`

Measured at candidate `6e9ca13` on 2026-08-29 by `cases.sh`. The capture is `cases/ceiling/`, with digests
in `cases-manifest.txt`.

`REQ-MOK-071` and `SPEC-MOK-007` rule 14 make the ceiling **a stop, not a report**: a run that exceeds its
ceiling and says so has failed the requirement. This case is the one place that distinction is measurable
for free, because the canned connector can be told to report a usage that costs exactly one cent.

## The declared ceiling and the arithmetic that makes it reachable

```
--spend-ceiling 0.02 --prices 125:13:1000:0
cases/script-whole-cent   ok wait prompt=1000 cached=1000 output=987 reasoning=0
```

The prices are microcents per token: uncached prompt 125, cached prompt 13, output 1,000, reasoning 0. The
directive's usage therefore costs `1000 × 13 + 987 × 1000 = 1,000,000` microcents — **exactly one cent** —
and the ceiling is two of them. So the second exchange reaches the ceiling and the third is the one that
must never be issued.

A cent exactly is the point. The fixture's own default usage costs 0.0322 of a cent, which would need
thirty-two exchanges and three ticks to reach the smallest declarable ceiling, and the count this case names
would then depend on how many Mokiterions were still alive in the third tick.

## What was measured

**Exit status `3`** — rule 19.3's fourth status, distinct from `0`, from configuration error `2` and from
runtime error `1`.

Standard error, **both lines and only those two** (428 bytes):

```
spend ceiling reached at tick 1: the run stopped before the next exchange
{"run_record":"llm","seed":42,"ticks":1,"density":"0.75","trace_actions":false,"model":"canned-connector","reasoning":"none","exchanges":2,"tokens":{"prompt":2000,"cached_prompt":2000,"output":1974,"reasoning":0},"cache_ratio_basis_points":10000,"cost_cents":2,"ceiling_cents":2,"fallbacks":0,"unfit_to_publish":false,"tick_reached":1,"ended":"ceiling"}
```

Neither severity keyword appears: standard error contains no `runtime error:` and no `configuration error:`.
**A ceiling stop is the run doing what it was asked**, and the stream says so by not dressing it as a fault.

## `A4`: the check precedes the exchange

Two figures, together:

- `"cost_cents":2` against `"ceiling_cents":2` — **at the declared ceiling and not above it.**
- **two exchange records** in the transcript, for a tick that offers twelve decision opportunities.

That is the stop rather than the report. Had the check followed the exchange, the third request would have
been issued, answered and paid for, and this record would state `"cost_cents":3` while satisfying every
other assertion in this file. The measurement that discriminates is the *equality*, and it is not an
inequality that happens to hold: one more exchange at these prices costs exactly one more cent, so
`cost_cents ≤ ceiling_cents` and `cost_cents = ceiling_cents` are the same claim here only because nothing
beyond the ceiling was spent.

`"tick_reached":1` and `"ended":"ceiling"` are rule 15.5's two halves — the record names which of the two
endings this was, and at which tick, rather than quoting a horizon the run did not reach.

## `L18` and `L19`: what survives, and what must not appear

The transcript, `cases/ceiling/transcript.jsonl`:

- **12 prefix records** — the prefix head is complete, because it is written before the first exchange.
- **2 exchange records**, both `"fallback":false`. The two exchanges that happened were real decisions.

The record stream, `cases/ceiling/records.jsonl`, **139 lines, and it survived**. Rule 13.4 has the host
remove a record sink it created when the run *failed*, and rule 14.7 requires a ceiling-stopped stream to
survive complete and readable to the tick reached. The two disagree only in appearance: a ceiling stop is not
a failure, and the host tells them apart by the library's exit code. **A host that compared against `0`
alone would delete the evidence of every ceiling-stopped run.**

Its first line, and the absences:

```json
{"record":"header","schema":3,"engine":"0.1.0","config":{"seed":42,"ticks":1,"policy":"llm","density":"0.75","trace_actions":false}}
```

| checked for | occurrences |
|---|---|
| `"record":"run"` — `SPEC-MOK-006` rule 8's run record | **0** |
| `simulation_ended` | **0** |
| `"run_record":` — rule 15's record leaking into this stream | **0** |
| `summary reason=` on standard output | **0** |

The first two absences are the run not having ended: rule 8.9's `reason` domain has no member for a stop,
and rule 15.5 forbids quoting a figure at a horizon the run did not reach. The third is the destination
decision asserted where it would fail — rule 12.6 requires a replay to reproduce these bytes and a replay
reports no run record, so a rule 15 record here would make the two rules contradict.

**And the operator is nevertheless told the whole account**, on standard error. That is what makes the
absences legible rather than lossy: the stream is short of a rule 8 run record, and nothing here is a stream
a reader mistakes for a complete run.

Every line of the stream begins with `{` and ends with `}`, so what survived is readable and not a truncated
final write.
