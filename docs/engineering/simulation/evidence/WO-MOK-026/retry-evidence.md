# Item 9: the retry path — `VER-MOK-018` cases `R1` and `R2`

Measured at candidate `6e9ca13` on 2026-08-29 by `cases.sh`. The captures are under `cases/retry-bounded/`,
`cases/retry-exhausted/` and `cases/retried-replay/`, with digests in `cases-manifest.txt`.

`SPEC-MOK-007` rule 19.5 bounds the retrying at **three retries — four attempts** for a transport failure,
and rule 11.2 gives every attempt its own transcript record. The retry count, the backoff shape and which
transport failures are retried are the three things `WO-MOK-026`'s *Authorized decision envelope* delegates
to the implementation, subject to these two cases; everything below is what was chosen, measured.

## The stub

The canned connector, with a script. No provider, no network, no credential beyond the suite's own synthetic
value. `error transport <message>` is rule 10.4's error form with the kind that rule 19.5a makes retryable:

```
cases/script-retry-two      error transport the socket closed
                            error transport the socket closed
                            ok wait
cases/script-retry-always   error transport the socket closed
```

The fixture repeats its last directive when the script is exhausted, so the first script fails twice and
then answers for the rest of the run, and the second fails at every attempt forever.

## `R1`: a transport failure is retried, and every attempt is a record

`cases/retry-bounded/`, exit **`0`**.

**Fourteen exchange records for twelve opportunities.** That figure is the whole claim: a host that retried
nothing would write twelve, and a host that retried without recording the attempts would also write twelve.

The first four records, as measured:

| # | tick | actor | `fallback` | `usage` | response |
|---|---|---|---|---|---|
| 0 | 1 | `M01` | `false` | `null` | `{"protocol":1,"error":{"kind":"transport","message":"the socket closed"}}` |
| 1 | 1 | `M01` | `false` | `null` | `{"protocol":1,"error":{"kind":"transport","message":"the socket closed"}}` |
| 2 | 1 | `M01` | `false` | present | `{"protocol":1,"action":{"verb":"wait"},"model":"canned-connector",…}` |
| 3 | 1 | `M02` | `false` | present | an answer |

Three records for the first opportunity, same tick and same actor, the two failures ahead of the answer; and
the fourth record has **moved on to another actor**, so the retrying was bounded by the answer rather than
running to the bound. `the socket closed` appears in exactly **2** of the 14 records.

**No record is marked as a fallback — 0 of 14 carry `"fallback":true`.** That is the half rule 15.4 rests
on: the opportunity reached a decision, so nothing about it was rule 9.5's fallback. The two abandoned
attempts are `false` because they were not the decision; the third is `false` because it carried an action.
A run that retried and succeeded is a clean run, and the run record says so:

```json
{"run_record":"llm",…,"exchanges":14,"tokens":{"prompt":12000,"cached_prompt":10800,"output":96,"reasoning":0},"cache_ratio_basis_points":9000,"cost_cents":0,"ceiling_cents":200,"fallbacks":0,"unfit_to_publish":false,"tick_reached":1,"ended":"tick_limit"}
```

`"exchanges":14` counts attempts and `"fallbacks":0` counts opportunities that fell back. The two figures
are different quantities in the same record and both are right.

## `R2`: exhausted retries are a counted fallback, and the run continues

`cases/retry-exhausted/`, exit **`0`**, run with `--trace-actions` because "the run continues" is a claim
about the *decisions* and not only about the exit code.

Every attempt fails, so every opportunity spends the bound:

```
exchange records: 48
marked as fallbacks: 12
indices marked:  [3, 7, 11, 15, 19, 23, 27, 31, 35, 39, 43, 47]
expected group*4+3: [3, 7, 11, 15, 19, 23, 27, 31, 35, 39, 43, 47]
```

**Forty-eight records, four attempts an opportunity, and exactly one mark in every group of four — the
last.** The position is the load-bearing part: the mark is on the attempt that exhausted the bound, not on
the first failure, because an attempt that was retried is not a decision. The first eight records name
`M01` four times and then `M02` four times, so the grouping is per opportunity and not interleaved.

Every one of the 48 carries `the socket closed`, and every one carries `"action":{"verb":"wait"}` — rule
9.5's fallback applied and rule 9.7's prohibition holding, nothing supplying a substitute action from
another source for an attempt that obtained none.

**Twelve marks for twelve opportunities is case `P5`'s reconciliation** in the only form available from
outside: rule 15.4's figure is a count per opportunity, and a port counting per attempt would report 48
against these same 12 marks. The run record reports `"fallbacks":12`, `"exchanges":48`:

```json
{"run_record":"llm",…,"trace_actions":true,"model":null,"reasoning":null,"exchanges":48,"tokens":{"prompt":0,"cached_prompt":0,"output":0,"reasoning":0},"cache_ratio_basis_points":null,"cost_cents":0,"ceiling_cents":200,"fallbacks":12,"unfit_to_publish":true,"tick_reached":1,"ended":"tick_limit"}
```

And the run continued rather than ending at the first exhausted opportunity: standard output holds
`proposal:wait` **12** times, `status:rejected` **0** times, and `summary reason=tick_limit`.

## The disclosure: a retried transcript is not a replay input

`cases/retried-replay/`, exit **`1`**, standard error:

```
runtime error: transcript: tick 1 actor M02: record is for actor M01
```

Rule 11.2 gives every attempt its own record; rule 12.3 has a replay consume **one record per decision
opportunity**, checking the tick and the actor. The two do not reconcile. The first opportunity consumes the
first *attempt*'s record — marked `"fallback":false` and carrying rule 9.5's `wait`, so nothing distinguishes
it from a legitimately recorded decision — and the second opportunity then meets a record naming `M01`.

**This is disclosed rather than stopped on because the failure is loud and specific.** No replay silently
invents a run: the message names the opportunity and the actor the record was for, and the exit code is rule
4's `1`. What it cannot say is that the record was an attempt, because no field rule 11.3 fixes distinguishes
one.

The consequence for this work order, stated so it is not discovered later: **a live run that retried cannot
supply case `L30`'s replay identity.** Repairing it means either a new field on the exchange record or rule
12.3 reading a group per opportunity — both changes of substance in an approved artifact, which
`WO-MOK-026`'s stop-and-escalate condition 6 reserves to the owner. The live run this work order paid for
retried nothing, so `L30` is unaffected in fact; the defect is in the shape, not in this run.
