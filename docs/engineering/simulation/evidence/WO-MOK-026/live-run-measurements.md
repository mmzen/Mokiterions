# Live-run measurements

The third of the three evidence paths `live-run-authorization.md` named before the first byte was captured. It records
`WO-MOK-026` item 13's instrument measurement — the cache ratio re-derived from the transcript's own figures, the actual
cost beside both estimates, and the measured token split — and item 14's block D rendering measurement, which the same
transcript makes answerable exactly and for nothing.

Every figure below was recomputed from `live-run-transcript.jsonl` rather than read out of the run record, so the run
record is a thing this document checks and not a thing it repeats.

## Which run this is

**Two live runs happened. The second is the measurement; the first is a rejected attempt and is recorded here because a
run that was paid for and discarded is part of what this work order cost.**

| | Attempt 1 | Attempt 2 — **accepted** |
|---|---|---|
| Ticks reached | 50 of 50 | 50 of 50 |
| Exchanges | 567 | **503** |
| Fallbacks | 0 | **0** |
| Retries (`error` records) | 0 | **0** |
| Reasoning level actually requested | **none sent at all** | **`none`** |
| Reasoning tokens billed | 76,350 | **0** |
| Cost, as the engine reported it | 37 cents | **16 cents** |
| Verdict | **rejected** | **accepted** |

**Attempt 1 was rejected because it did not exercise the declared binding.** The connector held the declared reasoning
level as a constant, reported it to the engine in every response, and never put it in the request. So the run used the
model's default effort — 76,350 reasoning tokens prove it did — while its run record asserted `"reasoning":"none"`. Rule
15.2 makes that record a record of the model and level that answered; asserting a level the request never carried makes
it false. This was a defect in the connector, which is outside this repository, found by running it and not by reading
it. The fix sends `reasoning_effort` and is covered by a check that reads the **request** rather than the response,
because reporting a level and sending one are what this defect proved to be different things.

The two runs' exchange counts differ because turning reasoning off changed the model's choices, which changed the
trajectory, which changed how many decision opportunities 50 ticks contained. Both exceed item 13's floor of 200.

## The measured token split

Attempt 2, all 503 exchanges, recomputed from the transcript:

| | Total | Mean per exchange | Min | Max |
|---|---|---|---|---|
| `prompt` | 776,963 | 1,544.7 | 1,472 | 1,664 |
| `cached_prompt` | **0** | 0.0 | 0 | 0 |
| `output` | 9,429 | 18.7 | 18 | 20 |
| `reasoning` | **0** | 0.0 | 0 | 0 |

12 prefix records, one per actor. Ticks 1 to 50 all present. 503 exchange records, every one carrying an action; none
carrying an `error`; none flagged `fallback`.

The verbs chosen, which say something about whether the model was answering the question at all: `move` 288, `approach`
137, `eat` 32, `fight` 30, `sleep` 7, `attack` 5, `wait` 3, `surrender` 1. Eight of the eleven. **The three `wait`s are
the model's own choices and not rule 9.5's fallback** — the `fallback` flag is `false` on all three, which is the
distinction rule 9.5 and rule 11.3 exist to keep visible, and here it is doing that work.

Output of 18 to 20 tokens is the action object alone. At reasoning level `none` the model emits the answer and nothing
else.

## Item 13's canned transcript: it supplements, and here is why it cannot replace

Item 13 says the transcript "becomes the real canned transcript that **replaces or supplements**
`WO-MOK-025`'s synthetic one". It supplements. That is a measurement, not a preference.

**The two fixtures are rule 11.3.1's two cases, and neither can be the other.** The synthetic transcript's
`HuntingPort` answers out of block D, so its 221 exchange records carry `"response":null` and `"usage":null` —
the absent case, and all any free fixture can ever reach. The live transcript's 503 exchange records carry a
populated response and four populated counts, which rule 11.3.1 says `WO-MOK-026` is "where either first carries
a value". **This commit is that.**

**Replacing would have cost four things, each measured rather than argued:**

- `SPEC-MOK-007` rule 11.7's figures describe the synthetic file by name and would become false of the tree.
- `.github/workflows/provider-credentials.yml` names `--seed 0 --ticks 20` on one line, and `replay.rs`'s
  `RECORDED_RUN` asserts against that line deliberately so the two cannot drift. A 50-tick file breaks the pair.
- `WO-MOK-025`'s completion report is bound evidence stating the synthetic file's 233 records and 305,568 bytes.
- CI's offline replay would come to depend on a file that **cost money and cannot be regenerated**. The synthetic
  one is rewritten by an `--ignored` test whenever the recorded run changes; nothing can rewrite this one.

**What the supplement is.** Four tests in `mokiterions-core/tests/replay.rs`, all offline and free:

| Test | What only this file can establish |
|---|---|
| `the_live_transcript_replays_into_the_run_its_record_stream_describes` | a transcript with populated response and usage fields replays through the binary to its horizon |
| `the_live_transcript_is_populated_where_the_recorded_one_is_absent` | rule 11.3.1's populated case exists, **and the synthetic file is still the absent one** |
| `the_live_transcript_records_no_fallback_and_no_retried_attempt` | rule 1.1c's precondition, read from the transcript rather than from the run record |
| `the_live_evidence_carries_no_credential_and_names_the_provider_once` | rule 11.6 over the first bytes this repository ever committed that met a provider |

The file is read from its evidence path rather than copied into `tests/`. The path is provenance and cannot move;
a second copy would be 700 KB required to stay byte-identical to the first with nothing checking that it had.

**One assertion in this group was wrong when first written, and the correction is the useful part.** The replay
test originally asserted `survivors=8 deaths=4`, on the reasoning that who dies is downstream of the recorded
decisions. Mutating 32 of the transcript's `eat` decisions to `wait` in a scratch copy **left both figures
untouched** and moved all six food figures. So the survival figures are not sensitive to the decisions and the
assertion could not have failed. It now pins the whole summary line, and the mutation no longer matches it. The
measurement is recorded in the test's own comment so the assertion is not weakened back.

### Rule 11.7's figures, and the first estimate in this initiative a measurement confirms

Rule 11.7's own terms applied to the live transcript, beside the synthetic figures that rule states:

| | Synthetic, 20 ticks | Live, 50 ticks |
|---|---|---|
| Total bytes | 305,568 | **700,192** |
| Prefix records | 12, totalling 67,447 | **12, totalling 67,447** |
| Block A | 5,385 bytes each time | **5,385 bytes each time** |
| Exchange records | 221, totalling 238,121 | **503, totalling 632,745** |
| Mean exchange record | 1,078 bytes | **1,258 bytes** |
| Bytes per tick | 15,278 | **14,004** |

**The prefix bytes are identical, to the byte**, because both runs are seed 0 and the prompt for a given actor is
the same text — so the two files cross-check each other's block A rather than merely resembling it. The live
exchange record is 180 bytes larger on average, which is the response and the four counts and nothing else.

**Rule 11.7 extrapolates "a 1,000-tick run is an estimated 12 MB". Measured here: 12.7 MB.** After a cache ratio
that missed by its whole value and a cost that missed by 3.2×, this is the one estimate in this initiative that a
measurement confirms, and it is recorded as such because the others are recorded as failures.

**A drift is recorded and not corrected.** Rule 11.7 reads "the transcript this repository commits" and names one
file. The repository now commits two, and the rule's figures remain true of the one it names. Amending the
phrasing is a change of substance in an approved rule, which this work order's stop-and-escalate condition 6
forbids an implementation agent making on its own judgement, so it is **reported for the owner** rather than
written. Rule 11.7's figures are not wrong; its definite article is.

## Item 14: the block D rendering, measured rather than estimated

`WO-MOK-026` item 14 asks for "the measurement of the enumeration rendering `WO-MOK-025` did not choose, since a real
tokenizer is now reachable". `SPEC-MOK-007` leaves the question open in these terms:

> Whether block D enumerates verb-target pairs as one flat list or as a verb list with per-verb target lists is
> unspecified, and the trade-off is left to measurement: a flatter list is longer and costs more variable tokens, a
> nested one is shorter and may be harder to answer well.

### The tokenizer is calibrated, not assumed

`tiktoken`'s `o200k_base` was checked against the provider's own `prompt_tokens` on **every one of the 503 exchanges**:

```text
    reported - tiktoken   min +68   max +68   mean +68.00
```

**A delta that is exactly constant across 503 prompts of varying length settles two things at once**: `o200k_base` is
this model's encoding of the message text, and 68 tokens is fixed per-request framing the message does not contain. A
tokenizer that were merely close would have drifted with length. So every count below is exact, cost nothing, and needed
no further provider call. `tiktoken` was installed outside the repository; no crate was added to either package.

### The measured split of the prompt

Mean over 503 exchanges, message text only:

| Block | Mean tokens | Min | Max | Share of the message |
|---|---|---|---|---|
| Prefix, block 1 (the standing instructions) | 1,249.0 | 1,249 | 1,249 | **84.6 %** |
| Prefix, block 2 | 15.0 | 15 | 15 | 1.0 % |
| Observation, block C | 172.5 | 104 | 259 | 11.7 % |
| **Permitted, block D** | **40.2** | **25** | **93** | **2.7 %** |
| Message total | 1,476.7 | | | |
| Request framing | +68.0 | | | |
| **Reported `prompt_tokens`** | **1,544.7** | 1,472 | 1,664 | |

### Flat against nested

All 503 permitted blocks were re-rendered from the transcript and counted both ways. Three nested separators are shown
because the spec fixes no nested form, so a single choice of punctuation would have made the answer partly mine:

| Rendering | Mean | Total over the run | Against flat |
|---|---|---|---|
| Flat, one verb-target pair per line — **as sent** | 40.2 | 20,205 | — |
| Nested, `verb: a, b, c` | 35.9 | 18,039 | −10.7 % |
| Nested, `verb a\|b\|c` | 33.0 | 16,603 | −17.8 % |
| Nested, `verb: a b c` | **32.3** | **16,238** | **−19.6 %** |

The widest block in the run, 19 permitted actions, is the case that most favours nesting: 93 tokens flat against 74
nested.

### The answer, and it is not the one the trade-off anticipated

**Nesting is real but negligible. The spec's framing is sound and its premise is not.**

```text
    saved over the whole run : 3,967 tokens = 19.6% of block D
                            but 0.51% of the run's 776,963 prompt tokens
                            = 0.079 cents of a 16.67-cent run
```

Extrapolated on the same basis, a 1,000-tick run saves about **1.6 cents of roughly 333**, and `WO-MOK-027`'s five seeds
save about **8 cents of roughly 1,667**. **Block D is 2.7 % of the prompt and the standing instruction block is 84.6 %**,
so a fifth of block D is a rounding error and the cost half of the trade-off does not discriminate between the layouts.

**This holds whether or not caching ever engages, and that is worth stating precisely.** Block D varies every exchange,
so it sits in the part of the prompt no caching scheme would cover; the 0.079-cent saving is identical in the measured
world and in the world `REQ-MOK-070` assumes. What changes is only the share: 0.48 % of the measured bill, about 1.7 % of
the 4.78-cent bill that 0.85 caching would have produced. **Still not a figure a layout decision should rest on.**

**So the decision should rest on the half of the trade-off this measurement cannot reach.** The spec's own words for it
are "may be harder to answer well", and answerability is not measurable from a transcript: it needs the same exchanges
sent both ways to a live provider, which is spend, which needs an owner authorization naming a horizon, a seed set and a
ceiling. That authorization does not exist and this work order's ceiling is committed. **Recorded as the measurement's
boundary rather than treated as a residual**: item 14 asked for the token side and the token side is now measured
exactly, which is what makes the remaining question a clean one.

One observation bearing on it, offered as an observation: the flat form as sent was answered with a valid verb on all
503 exchanges, 0 parse failures and 0 fallbacks, and the model used 8 of the 11 verbs. **The layout in use has no
measured answerability problem**, so there is no defect pressing a change — which, given the cost figures above, is an
argument for leaving block D alone that the measurement supports without recommending.

**Disclosure.** The three nested renderings are this document's constructions, since `SPEC-MOK-007` names the nested
option without fixing its punctuation. The 19.6 % figure is therefore the best of three plausible forms and not the
nested form's canonical cost; a different separator moves it between 10.7 % and 19.6 %, and none of that range changes
the conclusion.

## The cache ratio, and why it is zero

```text
    cached_prompt / prompt  =  0 / 776,963  =  0.000000       (0 basis points)
```

**`REQ-MOK-070`'s obligation is 0.85. The measurement is 0.00, on every one of 503 exchanges.**

This is not the marginal failure the authorization record predicted before the run. That disclosure projected about
0.866 falling below 0.85 through tokenizer granularity, and reasoned from a shared character prefix of 0.874. **The
prediction was wrong in kind, not in degree**, and the reason is a provider behaviour no character measurement could
have found.

### What was ruled out

Each of these was measured, not assumed, and each was a candidate cause:

- **Not the missing reasoning parameter.** Attempt 1 sent none and attempt 2 sent `none`; both measured 0.
- **Not the schema.** The engine's `response_format` was captured across 24 real requests: **one distinct value**. It
  does not vary per exchange.
- **Not prompt layout.** The static prefix is 5,402 characters common to all 24 measured prompts, **about 1,316 tokens,
  above the provider's 1,024-token minimum**, and it is genuinely at the *start* of the prompt.
- **Not the prefix being sent too rarely.** 503 exchanges across 12 actors is about 42 sends of each actor's prefix
  inside seven minutes.
- **Not caching being unavailable.** It demonstrably works — see below.

### What it is

Three requests against the live provider, built to mirror the connector exactly — same `reasoning_effort`, same
`response_format`, same schema, same single user message — so that nothing but the prompt differed:

| Request | `prompt_tokens` | `cached_tokens` | `cache_write_tokens` |
|---|---|---|---|
| 1. prime the prefix | 1,627 | 0 | 1,624 |
| 2. **identical** prompt again | 1,627 | **1,624** | 0 |
| 3. **same 1,316-token prefix, different suffix** | 1,637 | **0** | 1,634 |

Request 3 is the run's case, and it cached nothing. Request 2 is the control, and it cached almost everything.

**This model caches an exact prompt, not a prefix.** The hit in request 2 was 1,624 of 1,627 tokens — the whole prompt
less a three-token tail, and not a multiple of the 128-token block a longest-common-prefix scheme would report. A second
observation agrees: 1,560 of 1,563 on a different prompt. The provider is matching the prompt, not walking it.

**The consequence for `REQ-MOK-070` is that no prompt layout can satisfy it under this binding.** The obligation rests on
a long shared prefix earning a discount; the run never repeats a prompt exactly, because each observation reports a
different world. Every exchange is therefore a full-price miss by construction. This is a finding about the requirement
meeting the provider, not a defect in the engine, the prompt or the connector, and **its disposition is the owner's** —
`VER-MOK-018` case `L15b` is where the share is gated, and rule 8.5's model identifier is reserved to the owner in this
work order's decision envelope. Item 14's tokenizer measurement was scoped as the diagnosis for a *marginal* miss; it
would not have found this, and it does not diagnose it now.

One lever is named without being recommended, because naming it is not deciding it: the measurement holds for this model
at this endpoint with this request shape. Whether another model, another endpoint or an explicit cache-control parameter
behaves differently is untested here and is not an implementation agent's to choose.

## Cost: the actual figure beside both estimates

`WO-MOK-026`'s *Evidence to record* item 5 calls this "the first point in this initiative where an estimate meets a
measurement". Three figures meet here.

| | Cents |
|---|---|
| `ADR-MOK-007`'s **estimate**, prorated to this horizon | 5 |
| The authorization record's projection at the published tariff | 5 |
| Attempt 2, **measured**, recomputed from the transcript | **16.67** |
| Attempt 2, as the engine reported it | 16 |

**The projection was arithmetically right and factually wrong, and the difference is exactly the caching.** Recomputing
attempt 2's own token counts with 85 % of the prompt cached gives **4.78 cents** — which is the 5-cent projection, to the
cent. The projection did not miscalculate; it assumed a discount the provider did not give. Caching, had it engaged,
would have made this run **3.5 times cheaper**.

So `ADR-MOK-007`'s estimated $1.04 for a 1,000-tick run is not vindicated after all, and the earlier record in
`live-run-authorization.md` that it was settled "in favour of `ADR-MOK-007`" is superseded by this measurement. Scaling
attempt 2 linearly:

```text
    0.3334 cents per tick  ->  a 1,000-tick run costs about $3.33, against an estimated $1.04
```

**A factor of 3.2, and it propagates.** `WO-MOK-027`'s estimated $5.20 for five seeds becomes about **$16.67** on the
same basis — above this work order's $2 ceiling and above any ceiling `ADR-MOK-007` act 8 anticipated. That is stated
here because `WO-MOK-027`'s own authorization record will need a revised figure, and because the figure that record would
otherwise inherit is now known to be low by more than three times.

**The engine truncates rather than rounds.** 16.67 cents is reported as 16. Rule 14.2 fixes the minor unit and does not
fix the direction, so this is an observation and not a defect; it is recorded because a truncating cost figure understates
spend against a ceiling, which is the one direction that matters.

### The cache-write rate: the four-price model has no slot for it

`WO-MOK-026` item 9's open "cache-write multiplier" question, with observed numbers rather than an estimate. The provider
returns **`cache_write_tokens`**, a fifth quantity rule 14's four prices cannot express, and the published rate is
**$0.25 per million — higher than the $0.20 input rate**.

Every one of attempt 2's 503 exchanges was a miss, and each miss wrote about 1,624 tokens: **about 816,900 tokens
written**, which the engine did not see and did not bill.

**Two readings, and this document cannot choose between them:**

- If a written token is charged at $0.25 **instead of** the $0.20 input rate, the unbilled amount is the 5-cent-per-million
  difference: about **4.1 cents**, a quarter of the reported cost.
- If it is charged **in addition** to the input rate, the unbilled amount is about **20.4 cents** — more than the entire
  16-cent figure the engine reported.

The API does not say which, and the difference is the whole question. **The provider's billing record for 2026-08-29 is
what settles it**, and reading it is the one remaining measurement that costs nothing. Until it is read, the reported cost
of a live run should be treated as a **lower bound** rather than a figure, and that is true of every run this engine has
priced, not only this one.

### Attempt 1's separate arithmetic error, kept because it is a real gap

Attempt 1 exposed a second defect in rule 14's model, which attempt 2 makes moot without fixing.

**`completion_tokens` is inclusive of `reasoning_tokens`**, so pricing `output` and `reasoning` as two disjoint
quantities bills the reasoning twice. Measured, not inferred: across all 567 of attempt 1's exchanges, `output` minus
`reasoning` was between 18 and 26 with a mean of 24.1 — the size of the action object, on every single exchange. Attempt
1's true cost was **28.21 cents** against the 37.37 the engine computed.

It does not affect attempt 2, where the reasoning count is zero and there is nothing to double-count. It is recorded
because it is a live fault in the cost model that only a reasoning run reveals, and a later work order raising the
reasoning level above `none` would meet it immediately.

## Replay identity

`VER-MOK-018` case `L30` requires the transcript to replay. It does:

```text
    Mokiterions.exe --policy llm --seed 0 --ticks 50 --transcript-path live-run-transcript.jsonl
```

exits 0 and its rendered output is **byte-for-byte identical** to the live run's. This holds because the run is
retry-free; rule 1.1c records that a transcript containing a retried exchange is not a replay input, so a run with a
fallback or a retry could not have shown this.

## Disclosures

- **The record stream is not byte-reproducible across platforms, and the authorization record claims it is.** It contains
  **one CR in two lines**: the connector's diagnostic goes through Python's text-mode standard error, which is CRLF on
  Windows, while the engine's run-record line is LF. Under `.gitattributes`' `-text` the bytes are hashed as written, so
  the same run on Linux would produce a different digest for this file. The transcript is unaffected: 515 lines, 0 CRs.
  **The file is retained exactly as captured rather than normalized**, because rewriting the bytes would make the
  evidence something edited rather than something produced. The connector's one-line fix is left unapplied so that the
  connector on disk remains exactly the one that produced this attempt.
- **The authorization record's cost section is superseded in part.** It records the tariff discrepancy as settled in
  favour of `ADR-MOK-007`; this measurement unsettles it. No figure in that record is edited, its own rule being that a
  record written before a run is what a run is measured against.
- **The connector is outside this repository and is not committed**, per the owner's decision of 2026-08-29. Its
  behaviour is therefore attested by this document and by `docs/CONNECTOR_PROTOCOL.md`, not by anything version-controlled
  here. Attempt 2 was produced by the connector as it stood after the `reasoning_effort` fix and before the CRLF fix.
- **The credential appears in no retained byte.** The run's own check reported clean over every produced file, and an
  independent scan of all three files for credential-shaped strings found nothing. No `error` record exists in either
  transcript, which is the only path by which a provider message could have reached a retained file.
- **Two live runs were paid for**, 37 and 16 cents as the engine reported them, 53 cents in total. Each was individually
  under the 200-cent ceiling, which is what `REQ-MOK-071` governs; the ceiling is per run and this is not a claim that it
  governs the total.
- **No figure here is inferred from an unchanged total.** Each was recomputed from the transcript.
