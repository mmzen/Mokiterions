# Item 16: the four existing sources re-compared against `WO-MOK-025`'s base-commit captures

Measured at candidate `6e9ca13` on 2026-08-29. This discharges `WO-MOK-026` *Evidence to record* item 16,
and it is the check `REQ-MOK-068` asks for: **the live path may be added, but the four deterministic
decision sources this repository already had must produce exactly what they produced before it existed.**

The comparison is against `WO-MOK-025`'s committed base capture at
`cc5418553cb433715b7d6b15dea3886bff30ffaa` — the commit before any live-path change — and against that work
order's own candidate capture at `8162b188e21c8b12a21b86a4ac85a2d0e3eea71a`, so the reader can see both
that nothing moved since the base and that nothing moved since the last time it was checked.

**Every figure below was produced by re-running the instruments at this candidate.** None of it is quoted
from `WO-MOK-025`'s run. That distinction is the whole point of the item: a citation would establish that
the sources were unmoved twelve commits ago, which is not what is being claimed.

## The grid

Forty cells per mode, unchanged from `WO-MOK-025`: five seeds — **0, 1, 42, 123, 777** — times the four
deterministic decision sources — **baseline, individual, reference, social** — times trace off and trace
on. Each cell records `sha256` of standard output, its byte and line counts, `sha256` of standard error,
the exit status, and in the sink mode the record stream's `sha256`, bytes and lines.

The instruments are `WO-MOK-025`'s own, unmodified and re-used rather than rewritten:
`docs/engineering/simulation/evidence/WO-MOK-025/capture.sh`, `manifest.sh`, and
`candidate/verify-schema-digit.sh`.

## The four readings

| Comparison | Result |
|---|---|
| no sink, this candidate vs `WO-MOK-025` **base** | **IDENTICAL** — all 40 cells, every column |
| no sink, this candidate vs `WO-MOK-025` **candidate** | **IDENTICAL** — all 40 cells, every column |
| record sink, this candidate vs `WO-MOK-025` **candidate** | **IDENTICAL** — all 40 cells, every column |
| record sink, this candidate vs `WO-MOK-025` **base** | differs in **40 of 40** cells, in **one column only** |

The first three are the claim. The fourth is the one difference in the whole grid, and it is the
record-stream digest — never standard output, never standard error, never an exit status, never a byte or
line count.

## The one difference, isolated

The record stream's header carries a schema digit. The base commit wrote `"schema":1`; `SPEC-MOK-006`'s
ratified increment made it `2`; this branch's tree writes `3`. The digit is one character wide at every
value, which is why **the record streams' byte and line counts are identical to the base's in all 40
cells** while their digests differ. First cell, the two lines side by side:

```
026  seed0-baseline-traceoff  8a9c8c1d…6486  167984  1604  e3b0c442…b855  0  5cf767a9…59ee  356148  1724
025  seed0-baseline-traceoff  8a9c8c1d…6486  167984  1604  e3b0c442…b855  0  41dc9327…e042  356148  1724
```

`verify-schema-digit.sh` re-writes each of this candidate's 40 record streams with the header digit set
back to `1` and hashes the result against the base manifest. Its output at this candidate:

```
40 cells compared against schema 1, 0 failures
```

Every cell reads `identical apart from the schema digit`. So the difference is not merely small: with the
one byte reverted, the streams are **byte-identical to the base commit's**, cell for cell.

## What this establishes and what it does not

**Establishes.** The live path added by this work order changes nothing an operator running any of the four
existing sources can observe — not the rendered output, not the diagnostics, not the exit status, and not
the record stream beyond a schema digit whose increment is a ratified specification act with its own record.
`REQ-MOK-068` holds at this candidate, measured.

**Does not establish.** Nothing here concerns the `llm` source, which is not in the grid: it cannot run
without a credential and an explicit live selection, and its measurements are in `live-run-measurements.md`
beside the authorization that paid for them. Nothing here concerns cross-platform reproducibility either —
every cell was captured on one Windows host, and the record stream's *not* being byte-reproducible across
platforms is recorded as an open finding in the completion report rather than resolved here.

## Files retained beside this one

| File | What it is |
|---|---|
| `nosink-manifest.txt` | this candidate's 40 no-sink cells, the reduced form the comparisons ran over |
| `sink-manifest.txt` | this candidate's 40 record-sink cells |
| `schema-digit-vs-base.txt` | the digit-reverted comparison, one line per cell, with its total |

The 80 raw capture directories are not retained. They are reproducible from the instruments named above at
this candidate, they run to hundreds of megabytes, and `WO-MOK-025` retains `base/full` for the side that
cannot be reproduced once the tree moves. This side can.
