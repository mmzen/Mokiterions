# Live-run measurements

The third of the three evidence paths `live-run-authorization.md` named before the first byte was captured. It records
`WO-MOK-026` item 13's instrument measurement — the cache ratio re-derived from the transcript's own figures, the actual
cost beside both estimates, and the measured token split — and item 14's block D rendering measurement, which the same
transcript makes answerable exactly and for nothing.

Every figure below was recomputed from `live-run-transcript.jsonl` rather than read out of the run record, so the run
record is a thing this document checks and not a thing it repeats.

## Which run this is

**Two live runs happened. The second is the measurement; the first is a rejected attempt, and its captures are retained
under `attempt-1/` — a run that was paid for is part of what this work order cost, and this one turned out to carry a
measurement nothing else here can supply.**

| | Attempt 1 | Attempt 2 — **accepted** |
|---|---|---|
| Ticks reached | 50 of 50 | 50 of 50 |
| Exchanges | 567 | **503** |
| Fallbacks | 0 | **0** |
| Retries (`error` records) | 0 | **0** |
| Reasoning level actually requested | **none sent at all** | **`none`** |
| Reasoning tokens billed | 76,350 | **0** |
| Cost, as the engine reported it | 37 cents | **16 cents** |
| Cost, recomputed under rule 14.2a as amended here | 28.21 cents | **16.67 cents** |
| Cost, **billed**, with the cache-write charge the engine cannot see | 32.55 cents | **20.55 cents** |
| Captures retained | `attempt-1/`, three files | top level, three files |
| Verdict | **rejected** | **accepted** |

**The two runs cost 53.10 cents between them, and 61 % of that went to the rejected one.** The figure is the provider's,
not this document's: its billing record for 2026-08-29 reads `$0.53` for the day, and the two rows above are what that
total decomposes into. What the rejected attempt bought is not nothing — rule 14.2a was measured from it — but it is
recorded as the larger half of the bill rather than as a footnote.

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

## The owner's dispositions, taken 2026-08-29

**Seven** questions this stage's measurements raised were put to the repository owner, each with its options costed
before it was offered. The selections are recorded here because a measurement that ends in a decision should retain the
decision beside it. All seven are kept together here rather than filed beside their separate measurements, so that a
reader can see the whole set of choices this stage's figures forced; each names the measurement it turned on.

### `REQ-MOK-070` — recorded as outstanding, and nothing is amended

**The disposition: `VER-MOK-018` case `L15b` fails, the failure is recorded against it, and `REQ-MOK-070`'s text does not
move.** Four options were put — record it as outstanding and amend nothing; amend the requirement into a
provider-conditional obligation, thresholded only where a provider matches prefixes; retire it as resting on a premise
measurement falsifies; or try another model or endpoint, at the price of a further authorised run and a decision reserved
to the owner by rule 8.5. The owner chose the first.

**What that means, stated plainly rather than left to be inferred:**

- **`L15b` is a required case of this work order** — *Required verification* lists it first, as "owner-gated, and
  therefore dependent on the authorised live run" — and it now has a measured result of **0.000000** against a floor of
  **0.85**. It is reported as failing. It is not softened, not re-scoped, and not marked inapplicable.
- **`WO-MOK-026` stop-and-escalate condition 3 is the route this took**, and it anticipated this outcome exactly: "The
  cache ratio comes in below eighty-five percent. Escalate rather than adjusting the threshold, the layout or the
  measurement. `REQ-MOK-070` is an obligation on the design, so a miss means the design is wrong or the number was wrong
  — and which of those it is, is the owner's to decide." The condition asks for the measured value, the split that
  produced it, and whether the provider's cached-token reporting behaved as documented. All three are above.
- **The requirement stands unsatisfied rather than satisfied-as-amended.** That is the substantive content of the
  choice: an amendment would have made the contract passable, and recording it outstanding keeps the contract honest at
  the price of a permanent red case. Any later run against this binding fails `L15b` the same way, for the same reason.
- **`VER-MOK-018` needs no amendment for this**, because its own *Five cases cannot be satisfied by a build* section
  already treats `L15b` as a case that can fail against a layout that was correct when written, and says what that means:
  "it is a signal to re-measure and bring the layout or the floor back to the owner, not a reason to soften the number in
  place."

**One consequence is flagged rather than resolved**: this stage's verification record will bind a failing required case.
Whether it may, and how it must disclose it, is the assurance owner's and is not settled by this row.

### Block D's layout — the flat form stands

**The disposition: block D stays flat, and the measurement above is the record of why.** The two options were costed
before being offered: leave it flat, at no code change and no spec amendment, or nest it for 3,967 tokens over the run —
19.6 % of block D, 0.51 % of the prompt, **0.079 cents** of a 16.67-cent run and about 1.6 cents of a 1,000-tick run's
$3.33. The owner chose flat.

The measured case for it, beyond the size: the flat form produced a valid verb on all **503** exchanges with **0** parse
failures, so the "may be harder to answer well" half of the specification's trade-off has one clean reading in its favour
and the nested form has none.

**`SPEC-MOK-007`'s *Explicitly unspecified decisions* is deliberately left as it stands.** Its third bullet says the
flat-versus-nested trade-off "is unspecified, and the trade-off is left to measurement", and it is tempting to read a
completed measurement as closing it. It does not. That bullet states what the *specification* declines to fix, and an
implementation measuring and choosing does not make the specification fix anything — writing the flat form into the rule
would narrow it, turning a measured local choice into an obligation on every future implementation, which is a wider act
than the question asked for.

### Rule 11.7's singular phrasing — amended in this work order

**The disposition: amended here, not deferred and not recorded as a defect.** Rule 11.7 read "the transcript this
repository commits" and this branch commits two. Three options were put with their costs — amend it here, since
`SPEC-MOK-007.md` is already in this work order's execution scope; record it as a defect, as this work order did with the
third target and with `SPEC-MOK-006`'s duplicate rule 8.9; or defer it to a governance work order behind a stacked pull
request. The owner chose to amend it here.

`SPEC-MOK-007`'s amendment record carries the row, rule **11.7.2** carries the live transcript's measured figures, and
the confirmed 12.7 MB extrapolation is set out under *Item 13's canned transcript* above. **One thing that amendment
changed is worth naming here**: it moved the formal snapshot from `94609ecb…` to `57e4ebd1…`, so any handoff evidence
bound before it would now be stale.

### `L15b`'s failure and this stage's verification record

**The disposition: the verification record verifies this stage and records `L15b` as FAILED, with the measurement and its
provider-binding cause disclosed, plus a carried-forward item to revisit either the floor or the model binding.**

The question was how a verification record can bind a contract in which a *required* case fails. Three options were put.
Withholding verification was costed and declined: nothing in this repository can make `L15b` pass, because the cause is
the provider's exact-prompt caching rather than anything in the engine, the prompt or the connector — so the only two
routes are changing the model binding, which is reserved to the owner, or moving the floor, which the owner had already
declined. Withholding would therefore not have delayed verification, it would have suspended it indefinitely over a fact
no code change here can reach. Deferring `L15b` to `WO-MOK-027` was declined for a stronger reason: that stage needs a
live run anyway for `L24` and `L25`, and it would meet the identical `0.000000`, so deferral would schedule a known
failure into a later stage and spend another paid run to re-observe it.

**What makes the chosen route defensible is that `VER-MOK-018` anticipated it in writing.** Its *Residual uncertainty*
already says: "Case **L15b** fails against a layout that was correct when written, and that is the intended behaviour: it
is a signal to re-measure and bring the layout or the floor back to the owner, not a reason to soften the number in
place." That process has now run to completion — the measurement was taken, the cause was diagnosed, the layout and the
floor were both brought back to the owner, and the owner declined to move either. The case is recorded failing, and the
contract is not edited to make it pass. Every other required case of this stage passes.

### Rule 14's double-billing of reasoning tokens — amended in this work order

**The disposition: amended here rather than recorded as a defect for a later work order.** The fault, its measurement and
the full account of what changed are in *Attempt 1's separate arithmetic error* below. Two options were put. What decided
it was a measured fact stated in the framing: because this stage's accepted run reports **no** reasoning tokens, the
corrected arithmetic yields identical figures, so the amendment invalidates no retained cost, no run record and no
replay — the fix is far cheaper here than the words "amend an approved specification" suggest. Against that, recording it
as a defect would have left a known-wrong pricing rule in an approved specification, reachable only by spending real
money at a reasoning level above `none`, which is precisely the condition under which nobody would want to discover it.

### The cache-write charge — settled from the provider's billing record

**The disposition: the owner reads the provider's billing record for 2026-08-29 and reports the charged total. It was
read, and it settles the question.** The two readings it had to choose between are in *The cache-write rate* below: a
written token charged at $0.25 **instead of** the $0.20 input rate, or **in addition** to it, the second putting more
unbilled money on this run than the engine reported spending. The API says which nowhere. The alternative, leaving the
packet's cost as a disclosed lower bound, was declined because it leaves the true price of a live run unknown by a factor
that may exceed the reported cost, which weakens every projection built on it including `WO-MOK-027`'s.

**This was the one measurement left in this stage that costs nothing**, and it is an owner act rather than an agent one:
the billing record is outside this repository and behind the owner's provider account, which is exactly where
`ADR-MOK-001` requires that relationship to live.

**The reading: `$0.53` for every run of 2026-08-29.** That is the day's whole total, so it contains both live runs and the
three-request caching probe, and it is stated to two decimals — between 52.5 and 53.5 cents. The first reading predicts
**53.10 cents** for the two runs and the second **85.98**. The first lands inside the window with room for the probe; the
second is 33 cents outside it. **The first reading is confirmed**, and every cost figure in this packet is a figure rather
than a lower bound from here on.

**The bill corroborates rule 14.2a independently, which is not the question it was asked.** The 53.10 prediction depends
on attempt 1's **corrected** cost of 28.21 cents. Recomputed with the double-billing arithmetic this work order removed,
the same reading predicts **62.26 cents** — nine cents above a bill that the corrected figure matches to a tenth of a
cent, with the day's probe accounting for a further fraction of one. The provider's own accounting agrees with the
amendment, and it is the only check on that amendment that does not come from this repository.

### The two runs' evidence layout — the asymmetry stands and is disclosed

**The disposition: attempt 1's captures are retained under an `attempt-1/` subdirectory, attempt 2's stay at the top
level of this evidence directory, and the departure from `VER-MOK-018`'s "one directory per run" is disclosed rather than
hidden.**

The retention table says each authorised live run's transcript is committed "under that measurement's evidence path, one
directory per run". Attempt 2's captures sit at the top level, not in a per-run directory, and that is a real departure.
It arose for a reason worth stating plainly rather than dressing up: **when attempt 2's paths were fixed, a second
retained run was not contemplated** — attempt 1 had been rejected and was expected to be discarded, and the decision to
retain it came later, after its figures turned out to support a claim no reader could otherwise re-derive.

Moving attempt 2 into an `attempt-2/` directory was costed and declined. The same contract's retention principle 2 says
the evidence directory name is fixed before the first capture *because it is provenance*, and that getting the path wrong
costs the whole capture — which for a live transcript means paying for another run. `SPEC-MOK-007` rule 11.7.2 and the
committed leak-check guard both name attempt 2's current paths as well. Amending the retention wording to match what was
done was also declined: it would edit an approved verification contract to fit the evidence, on a clause written
specifically so that the layout is decidable before the first capture.

So the literal reading of "one directory per run" is **not** satisfied for attempt 2, and this packet says so rather than
claiming a compliance it does not have.

**What is retained, and one addition beyond the disposition.** Each run now keeps the same three files — its transcript,
its record stream and its own standard output:

```text
    live-run-transcript.jsonl              700,192          attempt-1/live-run-transcript.jsonl     759,168
    live-run-record-stream.txt                 443          attempt-1/live-run-record-stream.txt        448
    live-run-stdout.txt                     76,819          attempt-1/live-run-stdout.txt            81,169
```

`VER-MOK-018`'s retention table names the transcript, the record stream and the run record; **the standard output is an
addition to it, and it is retained because it turns a claim in this packet into a check**. `L30` requires the transcript
to replay, and this packet said the replay's rendered output was byte-for-byte identical to the live run's — a comparison
no reader could repeat, because the live run's output was not a committed file. It is one now, for both runs, and the
comparison is asserted in `mokiterions-core/tests/replay.rs` rather than reported here. Attempt 1's copy earns its place
the same way: it is what makes the *rejected* run's transcript checkable too, and a retained capture nothing checks is one
that can rot unnoticed. The engine writes LF and neither file holds a carriage return, so unlike the record stream beside
them both are byte-reproducible on any platform.

Attempt 1's three files were copied from outside the repository under their captured names, `stdout.txt` being renamed to
`live-run-stdout.txt` to match its siblings — done before anything bound them, which is the only time a path in this tree
can be chosen. **The owner's leak check over all three reported clean**, and the committed guard now scans all six.

## Cost: the actual figure beside both estimates

`WO-MOK-026`'s *Evidence to record* item 5 calls this "the first point in this initiative where an estimate meets a
measurement". Three figures meet here.

| | Cents |
|---|---|
| `ADR-MOK-007`'s **estimate**, prorated to this horizon | 5 |
| The authorization record's projection at the published tariff | 5 |
| Attempt 2, **measured**, recomputed from the transcript | **16.67** |
| Attempt 2, as the engine reported it | 16 |
| Attempt 2, **billed**, with the cache-write charge the engine cannot see | **20.55** |

**The projection was arithmetically right and factually wrong, and the difference is exactly the caching.** Recomputing
attempt 2's own token counts with 85 % of the prompt cached gives **4.78 cents** — which is the 5-cent projection, to the
cent. The projection did not miscalculate; it assumed a discount the provider did not give. Caching, had it engaged,
would have made this run **3.5 times cheaper**.

So `ADR-MOK-007`'s estimated $1.04 for a 1,000-tick run is not vindicated after all, and the earlier record in
`live-run-authorization.md` that it was settled "in favour of `ADR-MOK-007`" is superseded by this measurement. Scaling
attempt 2 linearly, from the **billed** figure rather than the reported one:

```text
    0.4110 cents per tick  ->  a 1,000-tick run costs about $4.11, against an estimated $1.04
```

**A factor of 4, and it propagates.** `WO-MOK-027`'s estimated $5.20 for five seeds becomes about **$20.55** on the same
basis — ten times this work order's $2 ceiling and above any ceiling `ADR-MOK-007` act 8 anticipated. That is stated here
because `WO-MOK-027`'s own authorization record will need a revised figure, and because the figure that record would
otherwise inherit is now known to be low by four times. **Scaling from the engine's own 16.67 would understate it again**,
at $3.33 per 1,000 ticks and $16.67 for five seeds; those are the figures a reader who trusts the run record arrives at,
and they are recorded here as wrong for the reason set out below.

**The engine truncates rather than rounds.** 16.67 cents is reported as 16. Rule 14.2 fixes the minor unit and does not
fix the direction, so this is an observation and not a defect; it is recorded because a truncating cost figure understates
spend against a ceiling, which is the one direction that matters.

### The cache-write rate: the four-price model has no slot for it, and the bill says what it costs

`WO-MOK-026` item 9's open "cache-write multiplier" question, with observed numbers rather than an estimate. The provider
returns **`cache_write_tokens`**, a fifth quantity rule 14's four prices cannot express, and the published rate is
**$0.25 per million — higher than the $0.20 input rate**.

**What was written, measured from each run's own prompt total.** The provider caches an exact prompt rather than a prefix,
and it writes all but the last three tokens of one, which the three-request probe established. Every exchange of both runs
was a miss, so every prompt was written and none was read, and a run's written total is its prompt total less three tokens
per exchange:

```text
    attempt 1:   870,319 - 3 x 567  =    868,618 tokens written
    attempt 2:   776,963 - 3 x 503  =    775,454 tokens written
                                       ----------
                                        1,644,072
```

**This corrects an earlier figure in this document.** It read "about 816,900 tokens" for attempt 2, taken from 503
exchanges at one probe's observed 1,624. The run's own prompt total is the better measurement and gives **775,454**, 5 %
lower, and the unbilled amount below moves with it.

**The two readings, priced.** Both are computed at the authorization record's declared prices — `--prices 20:2:120:120`,
cents per million in rule 14.3a's unit — over both runs' reported counts, with attempt 1 priced under rule 14.2a as
amended here:

| Reading | Unbilled on attempt 2 | Predicted total for 2026-08-29 |
|---|---|---|
| $0.25 **instead of** the $0.20 input rate — a 5-cent-per-million difference | **3.88 cents** | **53.10 cents** |
| $0.25 **in addition** to the input rate | **19.39 cents** | **85.98 cents** |

**The owner's billing record reads `$0.53` for the day, so the first reading is confirmed** and the second is 33 cents
outside a figure stated to the cent. Attempt 2 therefore cost **20.55 cents**, of which the engine saw **16.67**.

**The engine's cost figure is 81 % of the money spent, and that is a finding about the ceiling rather than about the
report.** `REQ-MOK-071`'s ceiling is a stop and it stops on the figure the engine computes; that figure omits the
cache-write charge entirely, so **the ceiling bounds the engine's belief about spend and not spend**. Nothing was at risk
here — 3.88 cents against a $2 ceiling — but the gap is 19 % of the true cost in the direction that lets a run continue
past its limit, and it compounds with rule 14.2's truncation, which errs the same way. **A fifth price would close it and
cannot be added without an owner decision**: `--prices` takes four values in a fixed order, so a fifth term moves that
option's arity and rule 14.3a's format. It is recorded here as an open finding rather than proposed as an amendment, and
it is now *decidable* where before the bill it was not — the shape of the charge is known.

### Attempt 1's separate arithmetic error — found here, and fixed in this work order

Attempt 1 exposed a second defect in rule 14's model. Attempt 2 makes it invisible; it does not make it harmless, and
the owner's disposition was to amend rather than to record.

**`completion_tokens` is inclusive of `reasoning_tokens`**, so pricing `output` and `reasoning` as two disjoint
quantities bills the reasoning twice. Measured, not inferred: across all 567 of attempt 1's exchanges, `output` minus
`reasoning` was between 18 and 26 with a mean of 24.1 — the size of the action object, on every single exchange. Attempt
1's true cost was **28.21 cents** against the 37.37 the engine computed.

**The measurement is re-derivable, because the file it was taken from is retained.** It is
`docs/engineering/simulation/evidence/WO-MOK-026/attempt-1/live-run-transcript.jsonl`, and it is the only transcript in
this repository whose exchanges carry a reasoning count at all — attempt 2 reports none, and the synthetic fixture reports
nothing. `SPEC-MOK-007` rule 11.7.3 records its figures and states that as the reason it is kept: a rule measured from a
discarded file is a rule no later reader can check. The provider's bill for the day is the second, independent check on
the same arithmetic, and it is in *The cache-write rate* above.

**Rule 14 already had this convention right for the other inclusive pair, and that asymmetry is the whole defect.** The
prompt count likewise contains the cached count, and cost has always been `prompt − cached` at the uncached price plus
`cached` at the cached price — rule 14.4's ratio is built on exactly that containment. The output and reasoning counts
have the same shape. One pair was handled and one was not.

**Why no check caught it, which is the part worth keeping.** Two independent gaps had to line up, and both did:

- **No cost assertion anywhere in the engine crate declared a non-zero reasoning count.** Every one passed
  `reasoning = 0`, and at zero the double bill adds nothing.
- **The internal fixture sets the reasoning price equal to the output price** — `1_000` and `1_000` — which is how a
  provider bills them, and is also what makes a misattribution between the two invisible to any assertion that happens
  to split them the wrong way.

The one place a non-zero reasoning count did appear was a framing test, `ExchangeUsage::reported(1, 0, 2, 3)`, which
asserts a response's round trip and no cost figure at all. It is worth noting for a different reason: its reasoning
count *exceeds* its output count, so it is the case that would panic on subtraction overflow if the corrected
arithmetic did not clamp. The clamp is therefore already exercised by a test that was not written for it.

**What was changed.** `SPEC-MOK-007` **rule 14.2a** was added, stating the containment, the subtraction and the clamp,
and stating that rule 14.1's four run totals stay exactly as the provider reported them so that a reader recomputing a
cost from those totals must subtract likewise. `RunAccount::cost_of` was corrected to mirror its own treatment of the
cached share. One internal-tier case was added,
`the_reasoning_share_of_the_output_count_is_billed_once`, which closes both gaps above: it prices the two quantities at
**different** rates so a wrong split cannot hide, and it also asserts the equal-rate case, because the double bill is
the reasoning count's entire cost rather than a difference between two rates.

**The guard was verified against the defect, not merely written.** With the previous arithmetic restored it fails,
reporting 501,400 microcents against the correct 301,400 — a difference of exactly the 200 reasoning tokens at the
output price. A guard test that passes against both the fault and the fix would evidence nothing, so this was measured
rather than assumed.

**No retained figure moves.** The fault is reachable only when a run reports reasoning tokens, and attempt 2 reports
none, so this stage's 16.67 cents, its run record, its transcript and its replay are all unchanged — confirmed by the
full suite passing both before and after the change, with the same figures. Had attempt 1 been the accepted run, its
cost figure would have been wrong by 9.16 cents in the direction that understates nothing and overstates spend.

**This does not address the cache-write quantity.** That is the separate finding above, and it stays open in a different
way now that the bill has been read: `cache_write_tokens` is a *fifth* quantity rule 14's four prices cannot express at
all, the charge is settled at the 5-cent-per-million difference, and what is undecided is whether a fifth price is added —
which moves `--prices`' arity and is an owner act. The double-billing fix is about the four quantities the rule already
has.

## Replay identity

`VER-MOK-018` case `L30` requires the transcript to replay. It does:

```text
    Mokiterions.exe --policy llm --seed 0 --ticks 50 --transcript-path live-run-transcript.jsonl
```

exits 0, writes nothing to standard error, and its rendered output is **byte-for-byte identical** to the live run's. This
holds because the run is retry-free; rule 1.1c records that a transcript containing a retried exchange is not a replay
input, so a run with a fallback or a retry could not have shown this.

**That identity is now a committed assertion and not a reported observation.** The live run's own output is retained beside
its transcript, so `the_live_transcript_replays_into_the_run_its_record_stream_describes` compares the two byte for byte
and names the first differing line when they diverge. Its earlier assertions — every tick present, and the whole summary
line including all six food figures — are kept rather than replaced: the byte comparison would fail for any reason they
would, but they are what say *what* went wrong, a missing tick being a replay that stopped and a moved census being a
replay that decided for itself. **Attempt 1's transcript replays into its own retained output on the same terms**, which
is asserted beside it; a rejected run whose transcript no longer replayed would leave rule 14.2a's measurement with
nothing behind it and nothing saying so.

## Disclosures

- **The record stream is not byte-reproducible across platforms, and the authorization record claims it is.** It contains
  **one CR in two lines**: the connector's diagnostic goes through Python's text-mode standard error, which is CRLF on
  Windows, while the engine's run-record line is LF. Under `.gitattributes`' `-text` the bytes are hashed as written, so
  the same run on Linux would produce a different digest for this file. **Both runs' record streams carry it**, one CR
  each, attempt 1's having been captured by the same connector. The transcripts and both standard-output captures are
  unaffected — 515 and 579 lines, 0 CRs, and the engine's own output is LF throughout — so the record stream is the only
  retained file whose bytes depend on the platform that produced it.
  **The file is retained exactly as captured rather than normalized**, because rewriting the bytes would make the
  evidence something edited rather than something produced. The connector's one-line fix is left unapplied so that the
  connector on disk remains exactly the one that produced this attempt.
- **The authorization record's cost section is superseded in part.** It records the tariff discrepancy as settled in
  favour of `ADR-MOK-007`; this measurement unsettles it. No figure in that record is edited, its own rule being that a
  record written before a run is what a run is measured against.
- **The connector is outside this repository and is not committed**, per the owner's decision of 2026-08-29. Its
  behaviour is therefore attested by this document and by `docs/CONNECTOR_PROTOCOL.md`, not by anything version-controlled
  here. Attempt 2 was produced by the connector as it stood after the `reasoning_effort` fix and before the CRLF fix.
- **The credential appears in no retained byte.** Each run's own check reported clean over every file it produced, the
  owner's check over attempt 1's three reported clean, and the committed guard scans all six retained files for ten
  credential-shaped strings on every `cargo test`. No `error` record exists in either transcript, which is the only path by
  which a provider message could have reached a retained file.
- **Two live runs were paid for**, 32.55 and 20.55 cents billed, **53.10 cents in total**. Each was individually under the
  200-cent ceiling, which is what `REQ-MOK-071` governs; the ceiling is per run and this is not a claim that it governs the
  total.
- **The engine's two reported figures also sum to 53 cents, and that agreement is a coincidence.** 37 plus 16 is 53, and
  the money spent was 53.10, so a reader comparing totals would conclude the pricing rule was sound. It is not: attempt 1
  was **over**-billed by 9.16 cents by the double-counted reasoning tokens, both runs were **under**-billed by 8.22 for
  cache writes, and truncation took a further 1.04 — three errors in two directions that cancel to within a tenth of a
  cent at this one pair of runs. This is recorded because it is the trap in the data: **the only reason the defect was
  found is that the per-run figures were recomputed, and the totals would have hidden it.**
- **No figure here is inferred from an unchanged total.** Each was recomputed from the transcript.
