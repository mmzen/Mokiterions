# Owner authorization for the live run

`REQ-MOK-076` requires that a live run's evidence carry an authorization record naming the authorizing owner, the date,
the horizon, the seed set and the spend ceiling. This is that record for `WO-MOK-026` item 13's single instrument run.

It is written **before** the run rather than after it. `REQ-MOK-076`'s failure behaviour is explicit that a missing
authorization is named rather than "backfilled with a retrospective authorization, which would be a record of a decision
nobody made at the time", and the only way to be sure of that is to write the record first and let the run be measured
against it.

## The authorization

| | |
|---|---|
| **Authorizing owner** | The repository owner, who holds the product, technical and engineering owner roles. |
| **Date of authorization** | 2026-08-29. |
| **Horizon authorized** | **50 ticks.** |
| **Seed set authorized** | **{0}** — one seed, as item 13 requires ("at one seed"). |
| **Spend ceiling authorized** | **$2 (two United States dollars) = 200 cents**, as decided on 2026-08-23 and recorded in `ADR-MOK-007`'s *Decision record* act 7. |
| **Purpose** | An **instrument measurement**, not a published figure. |

**The ceiling is not a fresh decision.** Act 7 declared $2 once, for this run; this authorization supplies the horizon
and the seed that act 8 deferred, and adopts that ceiling rather than naming a new one. `ADR-MOK-007` act 12 states in
terms that approving the work orders "does not authorize any live run", so this is the separate act it names.

**This authorization does not extend to `WO-MOK-027`.** That stage's five-seed measurement is an **estimated** $5.20,
above this ceiling, and act 8 defers its horizon and seed set to its own authorization record. No run at any seed other
than 0, and no run past 50 ticks, is authorized by this document.

## How the four terms were settled

Recorded plainly, because the manner matters to the record's weight. The owner's instruction of this stage was **"ok go
WO-MOK-026"**. The four terms below were then settled by the owner choosing among options an implementation agent
measured and put to them; the selections are the owner's, the framing was not. Each option's cost was measured before it
was offered, on the precedent that a wrong cost figure in the framing makes a wrong decision record.

1. **Horizon — 50 ticks, 600 exchanges.** Chosen over 17, 20 and 100 ticks. Item 13 requires "at least 200 exchanges";
   this exceeds that floor by a factor of three, and the reason is stated under *What was disclosed before the run*
   below.
2. **Seed — 0.** The engine's default, and the seed of `WO-MOK-025`'s existing synthetic transcript, so the two are
   comparable.
3. **Retry disposition — re-run at the same seed and horizon if a retry occurs.** `VER-MOK-018` case `L25` requires a
   fallback count of `0` and no stop at the ceiling for a publishable transcript. Rather than accept a transcript
   containing a provider retry, the owner authorized repeated attempts at the same seed and the same horizon until a
   retry-free transcript is obtained. Each attempt costs the same as the first — see *Cost* below, where that figure
   turns out to be tariff-dependent — the ceiling is untouched, and no approved rule moves. The ceiling governs each run;
   this is not authorization to exceed it in aggregate.
4. **The connector — written by the implementation agent, run by the owner.** The agent wrote a connector for the
   declared binding outside this repository, and it is never committed to it. The owner supplies the credential and
   executes the run. **The credential does not reach the implementation agent and appears in no byte the agent
   produced.** `REQ-MOK-073`, `ADR-MOK-001` and `VER-MOK-018` case `C1` are the standing prohibitions; this arrangement
   is how they are kept for a run that necessarily involves a real credential.

## The declared binding this run exercises

Model **`gpt-5.6-luna`** at reasoning level **`none`**, which is `SPEC-MOK-007` rule 8.5 and `ADR-MOK-007` act 3. Both
are outside this work order's authorized decision envelope: the envelope reserves "the model identifier" and "the
reasoning level" to the owner, and `WO-MOK-026`'s *Out of scope* excludes "any model other than the declared one, any
reasoning level other than `none`, and any temperature or seed parameter". The connector sends neither a temperature nor
a seed parameter, and reports the identifier the provider names in its answer rather than the one it asked for.

## The evidence paths, named before the first capture

Named here, before any byte is produced, because a rename after capture means paying for a second run:

- `live-run-transcript.jsonl` — the complete transcript, every exchange's reported usage.
- `live-run-record-stream.txt` — the record stream and run record, which the owner's decision of 2026-08-29 puts on
  standard error.
- `live-run-measurements.md` — the cache-ratio computation re-derived from the transcript's own figures, the actual cost
  beside the estimate, and the measured token split.

All are under `docs/engineering/simulation/evidence/WO-MOK-026/`, which `.gitattributes` exempts from end-of-line
conversion, so their digests reproduce on any platform.

## What was disclosed before the run

Stated here rather than discovered in the results, so that a failure can be read against a prediction instead of
explained after it.

**`REQ-MOK-070`'s 0.85 cached share may fail for a reason no horizon and no connector can fix.** The engine's real
request bytes were measured at 2 ticks against a local stand-in for the provider, costing nothing: the mean prompt is
**6,183 characters** and the prefix common to all 24 prompts is **5,402 characters**, a shared share of **0.874**. That
is the ceiling on the cached share before any tokenizer granularity is lost, against an obligation of **0.85**. A
provider that caches in fixed-size blocks will round the cached prefix down, and a loss of about 3 % of the prompt is
enough to put the run under the threshold.

This is why the horizon is 600 exchanges rather than item 13's 200-exchange floor. The floor's arithmetic was estimated
at a cached share of about 0.83, below the obligation; a longer run amortizes the twelve unavoidable first-exchange
cache misses across more exchanges and is estimated at 0.866. The horizon was chosen to give the obligation its best
honest chance, not to make a marginal figure look better than it is.

**If the measured share falls below 0.85, that is a finding about the prompt layout, not about this run.** The layout is
`SPEC-MOK-007` rule 3's, and changing it is neither this work order's nor an implementation agent's. Item 14's
measurement against a real tokenizer is the diagnosis, and the disposition is the owner's.

**The run does not stop on a low cached share.** Measured in the same rehearsal: a run record reporting
`cache_ratio_basis_points` of 8327 still reported `unfit_to_publish` as `false`. The share is reported and is
owner-gated at `L15b`; it is not a condition the run enforces on itself.

## Cost

### The tariff, as published

**Retrieved from the provider's own pricing page on 2026-08-29**, for `gpt-5.6-luna` on the **Standard** tier at
**short context**, per million tokens: input **$0.20**, cached input **$0.02**, output **$1.20**. In the unit
`SPEC-MOK-007` rule 14.3a fixes — cents per million tokens, `prompt:cached:output:reasoning` — that is

```text
    --prices 20:2:120:120
```

**The fourth integer is the output price and not zero.** A reasoning token bills at the output rate, so that is its unit
price. The declared reasoning level is `none`, so the reasoning count should be `0` and the term should be multiplied
away; pricing it at `0` instead would agree with the example in rule 14.3a but would silently value a reasoning token at
nothing if one ever appeared, and under-reporting cost is the one direction that matters when a ceiling is the safeguard.

**Projected cost at 600 exchanges: 5 cents, against a ceiling of 200.** That is the engine's own arithmetic, not this
document's: a 50-tick rehearsal at these prices against a local stand-in produced a run record reading `cost_cents` **5**
and `ceiling_cents` **200**. A margin of about **forty times**.

**This settles the discrepancy recorded below in favour of `ADR-MOK-007`.** That ADR's **estimated** $1.04 per 1,000-tick
run prorates to $0.05 for 600 exchanges, which is what the published tariff gives. The four prices printed in rule
14.3a's example are about **6.7 times** the real tariff and are, as that rule presents them, an example rather than a
quotation.

Four qualifications on the figure, each of which would change it:

- **Long-context rates are double.** Not reached: the mean prompt is about 1,550 tokens.
- **Fast-mode rates are double** the Standard tier, and **Batch and Flex are half**. The projection assumes Standard.
- **Data-residency endpoints carry a 10 % uplift.**
- **There is a cache-write rate, and this engine has no input for it.** The published rate is **$0.25** per million
  tokens standard short — *higher* than the $0.20 input rate. Rule 14's cost model takes four prices and none of them is
  a cache-write price, which is `WO-MOK-026` item 9's open "cache-write multiplier" question with a number attached at
  last. The exposure is bounded and small: twelve cache writes, one per actor's first exchange, at about 1,350 tokens
  each and a $0.05 difference per million is about **$0.0008**, roughly **1 %** of a five-cent run. So the engine's
  reported cost will read very slightly low, by an amount that is immaterial against a $2 ceiling but is a real gap in
  the model rather than a rounding artefact.

### Why the estimate was worth checking at all

**The estimate depended entirely on a tariff no artifact in this repository stated, and the two figures available differed
by a factor of seven.** This is retained because a ceiling is only a safeguard if someone has checked that the run fits
under it, and because the check is what found the answer.

- **$0.05**, derived from `ADR-MOK-007`'s own **estimated** $1.04 for a 1,000-tick run at reasoning level `none`, which
  is 12,000 exchanges and so $0.0000867 an exchange. This is the figure consistent with `WO-MOK-026` item 13's
  **estimated** $0.02 to $0.03 at 200 exchanges.
- **$0.35**, computed at the four example prices `SPEC-MOK-007` rule 14.3a itself prints, `125:13:1000:0`. Measured, not
  estimated: a 50-tick rehearsal against a local stand-in, using the per-exchange token figures the measured character
  counts imply, produced a run record reading `cost_cents` **35**.

The first of the two proved right, as the published tariff above shows. Rule 14.3 forbids a compiled-in price precisely
because the price is an input, and the four integers stay the owner's to declare at the command line: this document
records what the provider publishes, and it does not turn that into a value the program carries.

**The check to make before running, kept because a published price is not a promise.** The run's token volume is known
well enough to test any tariff against the ceiling in one line. Projected from measured character counts at 600
exchanges: about **160,000 uncached prompt tokens**, **768,000 cached prompt tokens** and **7,000 output tokens**. With
the four prices in cents per million tokens as `p:c:o:r`, the run costs about

```text
    0.16 p  +  0.77 c  +  0.007 o    cents
```

and must come out below **200**. At the published tariff that is 3.2 + 1.5 + 0.8 = **5.5 cents**; at rule 14.3a's
example prices it is 20.0 + 10.0 + 6.6 = **36.6 cents**. The term that
dominates is the cached one, so **a provider that charges full price for cached tokens costs roughly three times as much
as one that discounts them by 90 %**. A tariff around ten times the example prices would exceed the ceiling, and the run
would stop at it — which `REQ-MOK-071` makes a correct outcome and `VER-MOK-018` case `L25` makes an unpublishable one,
so the spend would be real and the transcript useless. **If the arithmetic above comes out near or above 200, the run
should not be started under this authorization**; the horizon or the ceiling is then the owner's to revisit.

The actual cost, in the provider's units and in currency, is recorded in `live-run-measurements.md` beside both
estimates — `WO-MOK-026`'s *Evidence to record* item 5 calls that "the first point in this initiative where an estimate
meets a measurement", and it turns out to be the point where two of this initiative's own estimates meet each other as
well.

This record contains no credential and no account identifier, as `REQ-MOK-076`'s constraints require.
