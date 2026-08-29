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
   retry-free transcript is obtained. Each attempt costs the same **estimated** $0.05, the ceiling is untouched, and no
   approved rule moves. The ceiling governs each run; this is not authorization to exceed it in aggregate.
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

**Estimated $0.05** for 600 exchanges, against the ceiling of **$2**. `WO-MOK-026` item 13 states the run as an
**estimated** $0.02 to $0.03 at 200 exchanges; 600 exchanges at the same per-exchange figure is consistent with that.
The actual cost, in the provider's units and in currency, is recorded in `live-run-measurements.md` beside the estimate —
`WO-MOK-026`'s *Evidence to record* item 5 calls that "the first point in this initiative where an estimate meets a
measurement".

This record contains no credential and no account identifier, as `REQ-MOK-076`'s constraints require.
