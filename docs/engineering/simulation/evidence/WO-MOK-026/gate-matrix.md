# Item 7: the gate matrix, all four combinations

Measured at candidate `6e9ca13` on 2026-08-29 by `cases.sh`, against the release binaries. Every figure
below is a byte this repository now retains under `cases/`, and `cases-manifest.txt` carries the `sha256`,
byte count and line count of each.

`SPEC-MOK-007` rule 13.1 puts the live selection in the host and the credential in the connector "so that
neither component can satisfy the other's condition". The matrix is what that separation looks like from
outside.

| # | live selection | credential | outcome | exit | standard error | case directory |
|---|---|---|---|---|---|---|
| 1 | absent | absent | **nothing is spawned** | `0` | **0 bytes** | `cases/row1-unselected-uncredentialled/` |
| 2 | absent | present | **nothing is spawned** | `0` | **0 bytes** | `cases/row2-unselected-credentialled/` |
| 3 | present | absent | spawned; **refuses on the first exchange** | `0` | 334 bytes, the run record | `cases/row3-selected-uncredentialled/` |
| 4 | present | present | spawned; **answers** | `0` | 359 bytes, the run record | `cases/row4-selected-credentialled/` |

## Rows 1 and 2: nothing is spawned

The mechanism is what makes this measurable rather than asserted. `--connector-path` was given
`cases/no-such-connector`, a well-formed path with no program at it: the parser accepts it, and **a host
that reached the platform with it would fail with exit `1` and say so**. Both runs exit `0` with an empty
standard error, which they can only do by never having attempted the spawn.

The decisions came from row 4's transcript, replayed at the same seed and horizon, so these are runs with
real decisions to reach — a refusal for some other reason would demonstrate nothing about the spawn.

Three measured identities:

```
row1-unselected-uncredentialled: standard output IDENTICAL to the recording's
row2-unselected-credentialled:   standard output IDENTICAL to the recording's
rows 1 and 2:                    indistinguishable
```

The last line is rule 13.1's separation, checked rather than assumed. The credential variable was **set**
for row 2 and **removed** for row 1, and the two runs produced the same 14,381 bytes and the same empty
standard error. A host that consulted a variable it claims never to read would differ here.

Standard error being empty in both is also `SPEC-MOK-007` rule 15.6, measured as a difference between two
runs rather than as the absence of a feature: the recording in row 4 wrote a run record to that same stream.

## Row 3: spawned, and refuses on the first exchange

The connector was started and spoke; it had no credential, so it answered rule 10.4's error form. The
**first** exchange record's response, verbatim:

```json
{"protocol":1,"error":{"kind":"refused","message":"no usable credential in the connector's environment: MOKITERIONS_TEST_CREDENTIAL is absent, empty or unreadable"}}
```

Counted over the transcript's 12 exchange records: **12** carry `"fallback":true`, **12** carry `refused`,
**12** name the variable `MOKITERIONS_TEST_CREDENTIAL`, and **none** carries its value — the value was
never set in this run, and rule 13.3 has a connector name the variable and never the value in any run. The
refusal is the first thing the connector says, not something it reaches after answering.

The run record on standard error:

```json
{"run_record":"llm","seed":42,"ticks":1,"density":"0.75","trace_actions":false,"model":null,"reasoning":null,"exchanges":12,"tokens":{"prompt":0,"cached_prompt":0,"output":0,"reasoning":0},"cache_ratio_basis_points":null,"cost_cents":0,"ceiling_cents":200,"fallbacks":12,"unfit_to_publish":true,"tick_reached":1,"ended":"tick_limit"}
```

`"fallbacks":12` and `"unfit_to_publish":true` are rules 15.4 and 15.3: every opportunity fell back, and the
run says so rather than presenting twelve `wait` decisions as a result. `"model":null`,
`"reasoning":null` and `"cache_ratio_basis_points":null` are the absences stated as absences — nothing
answered, so there is no identifier to quote and no ratio to compute. The run still exits `0`, because rule
9.5's fallback is a counted fallback and not rule 9.8's abort.

## Row 4: spawned, and answers

```json
{"run_record":"llm","seed":42,"ticks":1,"density":"0.75","trace_actions":false,"model":"canned-connector","reasoning":"none","exchanges":12,"tokens":{"prompt":12000,"cached_prompt":10800,"output":96,"reasoning":0},"cache_ratio_basis_points":9000,"cost_cents":0,"ceiling_cents":200,"fallbacks":0,"unfit_to_publish":false,"tick_reached":1,"ended":"tick_limit"}
```

Twelve prefix records and twelve exchange records, **all twelve `"fallback":false`**. The count of records
proves the spawn — nothing else could have answered — and only the flag proves the reading: a host that
spawned the connector and then failed to parse its answers would still exit `0` with a full transcript.
`"cost_cents":0` is truncation, not free money: twelve exchanges at the fixture's default usage cost **0.3864**
of a cent at the declared prices — 100 uncached prompt tokens at 125 microcents, 900 cached at 13, and 8
output at 1,000, which is 32,200 microcents an exchange — and rule 14.2's minor unit floors the total to
zero.

## No provider call, in all four rows

Item 7 asks for the confirmation in three rows. It holds in **four**, and the reason is worth stating
because it is the reason this matrix costs nothing to re-run:

- **Rows 1 and 2**: no process was started, so nothing existed that could call anything.
- **Row 3**: the connector was started and refused at its own gate, before any call. The refusal is its
  first line.
- **Row 4**: the connector answered from a script. It is `tests/support/canned_connector.rs`, whose entire
  import list is `std::env`, `std::io` and `std::process::ExitCode` — see
  `canned-connector-dependencies.md`. **No network type is reachable from it.**

The credential in rows 2, 3 and 4 is `sk-canned-0000-authenticates-nothing`, a value of the test suite's own
invention in a variable of its own invention. It authenticates nothing anywhere, and it is not the value
used in the live run — that one is in the operator's environment and appears in no file of this repository.
The live run's own gate behaviour is in `live-run-measurements.md`; nothing in this file was paid for.
