+++
id = "WO-MOK-026"
type = "work_order"
title = "Stage 5b: the provider program, the live path, the two gates, the usage accounting and the spend ceiling"
status = "draft"
owners = ["engineering owner"]
created = "2026-08-23"
updated = "2026-08-23"

[assurance]
commit_bound_verification = "required"
rationale = "This is the first work in this repository that can spend money and the first that sends anything outside it, and both facts are verifiable only over evidence. That a credential reaches the provider program and appears in no transcript, no record stream, no run record and no error message is a claim about produced bytes. That no provider call occurs unless both an explicit live selection and a credential are present is a claim about four combinations of two conditions, one of which is an environment variable, and a defect in it spends the owner's money without their instruction. That the ceiling is checked before an exchange rather than after it is a claim about ordering that only a run driven past the ceiling can show. The stage also produces the first real usage figures, and `REQ-MOK-070`'s eighty-five percent obligation is measured from them rather than from the estimate that motivated it — so the number this stage reports either confirms the cache-ordered layout or refutes it, and either way a later reader will cite it. Verification requires an owner-authorised live run, which means the record binds evidence that cannot be regenerated on demand."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-069", "REQ-MOK-070", "REQ-MOK-071", "REQ-MOK-072"]
specifications = ["SPEC-MOK-001", "SPEC-MOK-007"]
verification = ["VER-MOK-018"]
architecture = ["ARCH-MOK-001", "ADR-MOK-007"]
+++

# Work Order: Stage 5b — the provider program, the live path and the ceiling

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope below.
Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the completed
change and the retained evidence. Verification requires a separate commit-bound record.

**`WO-MOK-025` must be verified before this work order may begin.** Not merely implemented — verified, with a
commit-bound record. Every offline property this stage's live path depends on is established there: the port, the
cache-ordered layout, the transcript, the replay, the fallback accounting, the isolation checks. Building the live path
first would mean the first thing the owner pays for is the discovery of a defect the offline stage was designed to catch.

**Approval of this work order does not authorize any live run.** It authorizes building the path. Each live run needs its
own authorization under `REQ-MOK-076` — the owner's instruction naming the horizon, the seed set and the spend ceiling —
and this work order needs exactly one such run, described below. That is a second owner act, and in this repository one
person holds all three governance roles, so nothing here is approved by implication.

If `ADR-MOK-007`'s transport decision is settled as option **3a** rather than the recommended **3b**, this work order's
scope changes materially and its *Required amendments* precondition grows by `REQ-MOK-050`, `ADR-MOK-006` and
`SPEC-MOK-002` rule 13. See *Stop and escalate conditions* item 1.

## Objective

Build the live path: a provider program outside the workspace that speaks the `gpt-5.6-luna` API on one side and the
port's line protocol on the other; the two conditions that gate it; the usage and cost accounting; the spend ceiling that
stops a run rather than exceeding it; and the retry behaviour a network needs. Then, under one owner authorization,
execute the smallest live run that can measure the cache ratio and produce a real canned transcript.

## In scope

1. **The provider program**, per `SPEC-MOK-007` rule 10: a separate program outside both Rust packages, reading requests
   and writing responses on the port's streams, calling the provider's API, declaring the model, and using its own
   language's standard library only.
2. **The provider binding as configuration**: the model identifier, the reasoning level, the endpoint and the unit prices,
   each declared where a change of provider or price does not touch the engine. `reasoning` is `none`, per the owner's
   decision, and the value is declared rather than defaulted so that changing it is visible.
3. **The credential path**: read from the process environment by the provider program, from nowhere else, and never by
   the library target.
4. **The two gates**, per rule 13 and `REQ-MOK-072`: an explicit live-mode selection and a credential present. Absent
   either, no provider call. All four combinations behave as `L20` states.
5. **The usage accounting**, per rule 14 and `REQ-MOK-069`: prompt, cached-prompt, output and reasoning token counts as
   the provider reports them, per exchange, in the transcript — the provider's numbers, not the engine's estimate.
6. **The cost arithmetic against real usage**, in integer units, with the cache-write multiplier — the arithmetic
   `WO-MOK-025` built against synthetic usage now driven by reported usage.
7. **The spend ceiling**, per `REQ-MOK-071`: the option that declares one, the check that runs **before** each exchange,
   the run's end when it is reached, and the ceiling and accumulated cost in the run record.
8. **The cache-ratio report**, per `REQ-MOK-070`: cached prompt tokens as a share of total prompt tokens over the run,
   computed from reported usage, reported in the run record, and compared against the eighty-five percent obligation.
9. **Retry**, per rule 19: bounded, each attempt its own transcript record, exhaustion becoming a counted fallback under
   `REQ-MOK-074` rather than ending the run.
10. **One owner-authorised live run**, the smallest that can satisfy `L15b`: at least 200 exchanges, at one seed, with a
    declared ceiling. Its transcript becomes the real canned transcript that replaces or supplements `WO-MOK-025`'s
    synthetic one.
11. **The measurement of the enumeration rendering `WO-MOK-025` did not choose**, since a real tokenizer is now
    reachable, so that the layout's token split rests on measurement rather than on the estimate in `SPEC-MOK-007`.
12. **The owner attestation C6** and the credential attestation, retained with this stage's evidence.
13. The amendments `ADR-MOK-007` requires of `REPOSITORY_CONTEXT.md` and of `SPEC-MOK-004` **rule 1** — the layout entry
    admitting the provider program's directory as a non-package entry, which is a precondition of the change rather than
    a consequence of it — together with `SPEC-MOK-004` **rule 11**'s test-count figures for the tests this stage adds,
    measured against the tree at the candidate commit and never inferred from an unchanged total.

## Out of scope

- **`REQ-MOK-075`'s comparison and `REQ-MOK-076`'s general authorization record.** `WO-MOK-027`'s. The one run here is
  authorised for an instrument measurement, not for a published outcome, and its figures are explicitly not the
  comparison.
- **Any run at the full 1,000-tick horizon or over the five-seed set.** That is `WO-MOK-027`'s and needs its own
  authorization. An **estimated** $1.04 per run and 1.2 to 2.4 hours are not this stage's to incur.
- **Any change to the port's interface**, the request layout, the transcript format or the fallback rule. Those are
  `WO-MOK-025`'s decided surface; a change to any of them here means that stage was wrong and is an escalation.
- **Any crate added to either Rust package**, and any dependency in the provider program beyond its standard library.
- **Any second provider, any model other than the declared one, any reasoning level other than `none`**, and any
  temperature or seed parameter — neither is documented for `gpt-5.6-luna` and `SPEC-MOK-007` therefore relies on
  neither.
- **Any concurrency across Mokiterions.** Latency is the reason it is tempting and `SPEC-MOK-007` rule 16 is the reason
  it is not available: it would change the state each request describes.
- **Any credential in any workflow**, any live selection in any workflow, and any relaxation of `L21a`.

## Authorized decision envelope

The implementation agent may decide locally:

- The provider program's language, subject to a standard library sufficient for HTTPS and JSON, and to `S2`.
- Its internal structure, its error taxonomy and its logging, subject to `C1`.
- The retry count, the backoff shape and which transport failures are retried, subject to `R1` and `R2` and to the
  bound being declared rather than implicit.
- The unit-price representation and the cost unit, subject to `P6`'s integer prohibition and to the run record stating
  the unit.
- How the ceiling's pre-exchange check estimates an exchange's cost, subject to `L19`: the estimate may be conservative
  but must never permit a crossing.
- The provider program's directory name, subject to it being outside both Rust packages and to `SPEC-MOK-004` rule 1's
  layout being amended to admit it as a non-package entry. The agent chooses the name; that the rule admits it at all is
  the technical owner's, in the amendment.

The agent may **not** decide: the model identifier; the reasoning level; whether both gates are required; whether the
ceiling check precedes the exchange; whether reported usage or an estimate is authoritative; the horizon, seed set or
ceiling of the live run; or whether the live run happens at all. The last three are the owner's, in the authorization
record.

## Constraints

- **No live run occurs without a written owner authorization naming the horizon, the seed set and the spend ceiling.**
  This is the constraint the owner stated in their own words — *an explicit permission from the repository owner is
  needed to launch a real run* — and it is not satisfied by the approval of this work order.
- **The credential never enters the repository, the library target, any workflow, or any produced byte.** Four
  prohibitions, from `REPOSITORY_CONTEXT.md`, `ADR-MOK-001`, `REQ-MOK-073` and `C1` respectively.
- **The ceiling is a stop, not a report.** A run that exceeds its ceiling and says so has failed `REQ-MOK-071`.
- **The engine still opens nothing.** The host connects the streams; the library target's prohibition from
  `WO-MOK-025` stands and `S3` is re-run.
- **The four existing sources stay byte-identical.** `L9` and `L10` are re-run at the candidate commit, against
  `WO-MOK-025`'s base-commit captures.
- **Replay stays the default.** After this stage a run with no live-mode selection still behaves exactly as it did
  before it.
- **The live run's transcript, record stream, run record and authorization are committed together.** A live run whose
  evidence is incomplete cost money and produced nothing citable.
- **The evidence path is named before the first capture.** It becomes provenance the moment a record binds it, and a
  rename forces a whole fresh capture — which here means paying for a second live run.
- **Newly written files match the repository's stored line endings**, CRLF, as `WO-MOK-025` also requires.

## Expected change surface

- **A new program outside both Rust packages**: the provider adapter, its configuration, its declared model and prices,
  and its own tests.
- **The engine's binary target**: the live-mode selection, the ceiling option, the process or stream wiring that
  connects the provider program, and the usage text for both new options.
- **The engine's library target**: the usage figures on the transcript record, the accounting accumulator, the
  pre-exchange ceiling check, the ratio computation, and the run record's new fields. No new public item beyond what
  rule 15 needs.
- **The engine's test tiers**: the cases named below.
- **The evidence path**: the live run's four artifacts and the two attestations.
- **`REPOSITORY_CONTEXT.md`**, and `SPEC-MOK-004` rules 1 and 11, per the amendments.

## Required verification

`VER-MOK-018`, the cases `WO-MOK-025` could not reach plus the ones this stage's code creates:

**Owner-gated, and therefore dependent on the authorised live run**: **L15b** — over that run, cached prompt tokens are
at least eighty-five percent of total prompt tokens.

**Matrix cases**: **L16** and **L17** re-run against a live transcript rather than a stub's, **L18**, **L19**, **L20** in
full including the live half, **L21a** and **L21b** re-run, **L22** with a real transport failure, **L26** re-run.

**Acceptance scenario A4**, and **A1**, **A5** and **A6** re-run — **A1** because "a run nobody paid for" is the property
most at risk from this stage's existence.

**Properties P5** and **P6** against real usage figures, and **P1** and **P7** re-run against the live transcript.

**Static checks S1**, **S2**, **S3** and **S5**. **S2** now applies and is the check that makes `ADR-MOK-007` decision 3
true: the provider program's dependency declaration against its standard library.

**Security checks C1**, **C3** and **C5** in full, **C2** re-run, and **C6** — the owner's attestation that the
credential is not configured in the repository's automation secrets. `VER-MOK-018` calls **C6** the single fact the whole
cost containment rests on, and no check can make it.

**Resilience checks R1**, **R2**, and **R3** and **R5** re-run.

**Manual assessment M2** re-made if the shared rules block moved, which it should not have.

**L28** is verified for this stage's one run — its authorization is retained with its evidence — but the general
obligation is `WO-MOK-027`'s.

## Evidence to record

Under the evidence path this work order names, fixed before the first capture:

1. **The owner authorization record** for the live run: the authorizing owner, the date, the horizon, the seed, the
   ceiling, and the purpose stated as an instrument measurement rather than a published figure.
2. **The live run's transcript**, complete, with every exchange's reported usage.
3. **The live run's record stream and run record**, including the accumulated cost, the ceiling, the fallback count and
   the cache ratio.
4. **The cache-ratio computation** shown from the transcript's own figures, so that `L15b` can be re-derived by a reader
   rather than taken from the run record.
5. **The actual cost of the run**, in the provider's units and in currency, beside the estimate `SPEC-MOK-007` and
   `ADR-MOK-007` carry — the first point in this initiative where an estimate meets a measurement.
6. **The token split as measured**: the shared block, the actor block, a representative observation block and enumerated
   set, both enumeration renderings, and the resulting cached share.
7. **The gate matrix**: all four combinations of live-mode selection and credential presence, each with its outcome and
   the confirmation that no provider call occurred in three of them.
8. **The credential-leak check output** for `C1`, including the synthetic-credential test's assertion over every
   produced byte.
9. **The retry evidence** for `R1` and `R2`: the stubbed failures, the attempt records, and the counted fallback.
10. **The ceiling evidence** for `A4`, `L18` and `L19`: the declared ceiling, the tick reached, the exit status, and the
    demonstration that the check preceded the exchange.
11. **The provider program's dependency declaration** and the `S2` output.
12. **The two attestations**, `C6` and the credential attestation, each naming the owner and the date.
13. **The four existing sources' re-comparison** against `WO-MOK-025`'s base-commit captures.

## Stop and escalate conditions

1. **`ADR-MOK-007`'s transport decision is unsettled, or is settled as option 3a.** Under 3a the provider is inside the
   workspace and this work order's scope, dependency prohibitions and amendment set all change. Do not choose the
   binding; it is recorded in the ADR and is the technical owner's.
2. **The cache ratio comes in below eighty-five percent.** Escalate rather than adjusting the threshold, the layout or
   the measurement. `REQ-MOK-070` is an obligation on the design, so a miss means the design is wrong or the number was
   wrong — and which of those it is, is the owner's to decide. Report the measured value, the split that produced it,
   and whether the provider's cached-token reporting behaved as documented.
3. **The provider's reported usage does not distinguish cached prompt tokens.** `REQ-MOK-069` and `REQ-MOK-070` both
   rest on that field. Its absence makes the ratio unmeasurable and is a fact about the provider, not a defect to work
   around.
4. **The provider rejects, truncates or reorders the request**, or its caching does not key on the prefix as documented.
   Any of these invalidates the layout `SPEC-MOK-007` rule 3 fixes.
5. **A credential appears in any produced byte.** Stop, do not commit the evidence, and escalate. Committed evidence
   containing a credential cannot be corrected — the artifact is bound and the credential is disclosed.
6. **The live run would exceed its ceiling to complete.** It stops; that is the design. But if the ceiling proves too
   low to reach 200 exchanges, escalate for a new authorization rather than raising the ceiling locally.
7. **The run's actual cost exceeds the estimate materially** — take a factor of two as material. The estimate is what
   the owner's authorization was given against, so a large miss makes the next authorization's basis wrong.
8. **A retry policy cannot be bounded** without either dropping exchanges or extending a run indefinitely.
9. **Any pressure arises to make a workflow do a live run**, for any reason including convenience of measurement. `L21a`
   and `C6` are the whole cost containment.
10. **The synthetic transcript from `WO-MOK-025` and the live transcript disagree in form.** That means the stub was not
    faithful, and every offline case verified against it is weaker than its record claims.

## Completion report format

1. **What was built**, against the *In scope* list, each item done or escalated.
2. **The provider program**: its language, its dependency declaration, its `S2` result, and its location.
3. **The gate matrix**, all four combinations, with the evidence path for each.
4. **The live run**: its authorization, its seed, its horizon, its exchange count, its ceiling, its actual cost, its
   fallback count, and its cache ratio — with the ratio stated against the eighty-five percent obligation as pass or
   fail, plainly, before any explanation.
5. **The estimate against the measurement**: the per-run cost, the token split, the cached share and the latency, each
   as estimated in `SPEC-MOK-007` and `ADR-MOK-007` and as measured here, with the difference stated as a factor.
6. **Each verification case** in the required list, with its result and its evidence path.
7. **Every credential-handling decision**, so the owner can see the whole path a secret takes.
8. **The amendments made**, each with its artifact, provision and authorising act.
9. **What was not verified and why** — at least `L24`, `L25` and `M3`, which are `WO-MOK-027`'s.
10. **Every local decision** taken under the envelope, and **every escalation** raised, with its resolution.
