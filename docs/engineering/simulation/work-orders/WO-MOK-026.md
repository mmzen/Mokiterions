+++
id = "WO-MOK-026"
type = "work_order"
title = "Stage 5b: the connector, the live path, the two gates, the usage accounting and the spend ceiling"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-23"
updated = "2026-08-29"

[assurance]
commit_bound_verification = "required"
rationale = "This is the first work in this repository that can spend money and the first that sends anything outside it, and both facts are verifiable only over evidence. That a credential reaches the connector and appears in no transcript, no record stream, no run record and no error message is a claim about produced bytes. That no provider call occurs unless both an explicit live selection and a credential are present is a claim about four combinations of two conditions, one of which is an environment variable, and a defect in it spends the owner's money without their instruction. That the ceiling is checked before an exchange rather than after it is a claim about ordering that only a run driven past the ceiling can show. The stage also produces the first real usage figures, and `REQ-MOK-070`'s eighty-five percent obligation is measured from them rather than from the estimate that motivated it — so the number this stage reports either confirms the cache-ordered layout or refutes it, and either way a later reader will cite it. Verification requires an owner-authorised live run, which means the record binds evidence that cannot be regenerated on demand."
decided_by = "engineering owner"

[execution_scope]
paths = [
  "docs/CONNECTOR_PROTOCOL.md",
  "docs/engineering/REPOSITORY_CONTEXT.md",
  "docs/engineering/simulation/evidence/WO-MOK-026/",
  "docs/engineering/simulation/specifications/SPEC-MOK-002.md",
  "docs/engineering/simulation/specifications/SPEC-MOK-003.md",
  "docs/engineering/simulation/specifications/SPEC-MOK-004.md",
  "docs/engineering/simulation/specifications/SPEC-MOK-007.md",
  "docs/engineering/simulation/work-orders/WO-MOK-026.md",
  "mokiterions-core/Cargo.toml",
  "mokiterions-core/src/cli.rs",
  "mokiterions-core/src/lib.rs",
  "mokiterions-core/src/main.rs",
  "mokiterions-core/src/simulation.rs",
  "mokiterions-core/tests/",
  "mokiterions-tui/src/main.rs",
  "mokiterions-tui/src/options.rs",
  "mokiterions-tui/src/state.rs",
  "mokiterions-tui/tests/",
]

[relations]
implements = ["REQ-MOK-069", "REQ-MOK-070", "REQ-MOK-071", "REQ-MOK-072", "REQ-MOK-077"]
specifications = ["SPEC-MOK-001", "SPEC-MOK-002", "SPEC-MOK-003", "SPEC-MOK-004", "SPEC-MOK-007"]
verification = ["VER-MOK-018"]
architecture = ["ARCH-MOK-001", "ARCH-MOK-002", "ADR-MOK-007"]

[[lifecycle_events]]
from = "approved"
to = "in_progress"
decided_at = "2026-08-28T21:15:16Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "in_progress"
to = "implemented"
decided_at = "2026-08-29T18:46:30Z"
decided_by = "engineering-owner"
+++

# Work Order: Stage 5b — the connector, the live path and the ceiling

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope below.
Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the completed
change and the retained evidence. Verification requires a separate commit-bound record.

**This work order was approved on 2026-08-23**, by the repository owner acting as accountable product, technical,
engineering and assurance owner, in the words *"i approve the 3 work orders"* — one act covering `WO-MOK-025`,
`WO-MOK-026` and `WO-MOK-027`. `ADR-MOK-007`'s *Decision record* holds it as act 12. **That approval does not make this
work order startable**, for the reason the next paragraph gives: its precondition is a verification record, not a status,
and `preflight` cannot see it. From this act onward the tooling reports this work order as start-eligible; the gate below
is the authority on whether it may actually begin.

**`WO-MOK-025` must be verified before this work order may begin.** Not merely implemented — verified, with a
commit-bound record. Every offline property this stage's live path depends on is established there: the port, the
cache-ordered layout, the transcript, the replay, the fallback accounting, the isolation checks. Building the live path
first would mean the first thing the owner pays for is the discovery of a defect the offline stage was designed to catch.

**Approval of this work order does not authorize any live run.** It authorizes building the path. Each live run needs its
own authorization under `REQ-MOK-076` — the owner's instruction naming the horizon, the seed set and the spend ceiling —
and this work order needs exactly one such run, described below. That is a second owner act, and in this repository one
person holds all three governance roles, so nothing here is approved by implication.

**The transport question is settled and this work order's scope no longer turns on it.** `ADR-MOK-007` decision 3 is
an **external connector the operator names by path**, spawned by the engine's binary target — the design the repository
owner proposed on 2026-08-23, which superseded the same day's earlier selection of option 3a. Its consequence for this
work order is that **no dependency artifact is amended**: not `REQ-MOK-050`, not `ADR-MOK-006`, not `SPEC-MOK-002`
rule 13, not `ARCH-MOK-001`'s by-name conformance check, and not `SPEC-MOK-004` rule 1. `ADR-MOK-007` records all six as
considered non-amendments rather than dropping them silently, and the reasoning matters here: Cargo declares dependencies
per **package**, not per target, so a crate admitted for this stage's host would have been reachable from the engine's
library target and transitively from the observer. A connector outside the workspace is the only shape that leaves both
packages' declared sets where they are.

## Objective

Build the live path: the line protocol a connector speaks, the spawn that starts one, the environment pass-through that
carries a credential to it without either Rust target reading it, the two conditions that gate a live run, the usage and
cost accounting, the spend ceiling that stops a run rather than exceeding it, and the retry behaviour a network needs.
Ship one canned connector as a test fixture so all of that is exercisable for free. Complete the observer's refusals for
the three options this stage adds. Then, under one owner authorization and a **$2** ceiling, execute the smallest live run
that can measure the cache ratio and produce a real canned transcript.

## In scope

1. **The connector**, per `SPEC-MOK-007` rule 10: a program **outside this repository**, named by the operator as a path,
   spawned by the engine's binary target, speaking the port's line protocol on one side and the provider's API on the
   other. This repository neither builds it, ships it, nor constrains its dependencies — rule 10.6 withdraws the
   standard-library constraint an earlier draft placed on it, because no check here can see a program the operator
   supplies. What this stage delivers is the **protocol**, the **spawn**, and the documentation an operator needs in order
   to write one.
2. **The canned connector**: the one connector this repository does own, as a test fixture inside an existing package's
   test tree — not a third workspace member. It speaks the protocol and answers from a script, so a real child process can
   be exercised with no provider, no credential and no network. It is the only connector `S2` can see, and `VER-MOK-018`
   states plainly that checking it establishes nothing about an operator's.
3. **The connector path option and the spawn**, per rule 18.4: the option recognised, validated and enforced at-most-once
   by the engine's shared parser, which **discards the value** on the `--events-path` precedent; the engine's **binary
   target** re-reads the raw argument, spawns the child, connects its streams and reaps it. The library target spawns
   nothing and resolves no path — `SPEC-MOK-006` rule 1.2, re-checked by `S3` and `S3a`.
4. **The environment pass-through**, per rules 10.5 and 13.4: the child inherits the environment the binary target
   inherited, and a credential in it passes through untouched. Neither Rust target reads it, parses it, logs it or places
   it in an argument. The connector is the one component that reads the value, which is what keeps `INT-MOK-011`
   principle 1's *neither target reads a credential* literally true rather than nearly true.
5. **The provider binding, declared in the connector rather than in the engine**: the model identifier, the reasoning
   level and the endpoint. `reasoning` is `none`, per the owner's decision of 2026-08-23, and the value is declared rather
   than defaulted so that changing it is visible. The **unit prices** are the exception: the engine needs them for its
   ceiling arithmetic, so they are declared to the engine per rule 14. The engine holds prices; it holds no endpoint.
6. **The observer's refusals for these options**, completing `REQ-MOK-077` and case `L32`: a connector path, a live-mode
   selection or a spend ceiling given to the observer produces a diagnostic on standard error and exit `2` before the
   terminal is entered, naming that this host replays only and that a live run is the engine binary's. This is the item
   most easily left undone, because the observer forwards unrecognised options to the engine's shared parser and this
   stage is what makes that parser accept them — so *doing nothing here* turns each new option into an option accepted
   and silently ignored, which is the `--events-path` defect GitHub issue 40 tracks, reproduced in the same file.
7. **The two gates**, per rule 13 and `REQ-MOK-072`: an explicit live-mode selection and a credential present. Absent
   either, no provider call. All four combinations behave as `L20` states, in the engine's binary target, which is the
   only host a live run is reachable from at all.
8. **The usage accounting**, per rule 14 and `REQ-MOK-069`: prompt, cached-prompt, output and reasoning token counts as
   the provider reports them, per exchange, in the transcript — the provider's numbers, not the engine's estimate.
9. **The cost arithmetic against real usage**, in integer units, with the cache-write multiplier — the arithmetic
   `WO-MOK-025` built against synthetic usage now driven by reported usage.
10. **The spend ceiling**, per `REQ-MOK-071`: the option that declares one, the check that runs **before** each exchange,
    the run's end when it is reached, and the ceiling and accumulated cost in the run record.
11. **The cache-ratio report**, per `REQ-MOK-070`: cached prompt tokens as a share of total prompt tokens over the run,
    computed from reported usage, reported in the run record, and compared against the eighty-five percent obligation.
12. **Retry**, per rule 19: bounded, each attempt its own transcript record, exhaustion becoming a counted fallback under
    `REQ-MOK-074` rather than ending the run. Exercised against the canned connector, which fails on command.
13. **One owner-authorised live run**, the smallest that can satisfy `L15b`: at least 200 exchanges, at one seed, with a
    declared ceiling. **The ceiling the owner declared for it is $2**, decided on 2026-08-23 and recorded in
    `ADR-MOK-007`'s *Decision record*. Two facts about that figure are stated here rather than discovered later: it is
    ample for this stage, whose run is an **estimated** $0.02 to $0.03 at 200 exchanges; and it is **below** the
    **estimated** $5.20 that `WO-MOK-027`'s five-seed measurement costs, so that stage needs its own ceiling and its own
    authorization and cannot proceed under this one. Its transcript becomes the real canned transcript that replaces or
    supplements `WO-MOK-025`'s synthetic one.
14. **The measurement of the enumeration rendering `WO-MOK-025` did not choose**, since a real tokenizer is now
    reachable, so that the layout's token split rests on measurement rather than on the estimate in `SPEC-MOK-007`.
15. **The owner attestation C6** and the credential attestation, retained with this stage's evidence.
16. The amendments `ADR-MOK-007` requires of `REPOSITORY_CONTEXT.md`, of `SPEC-MOK-003`'s *Start-up inputs* for the
    observer's disposition of these three options, and of `SPEC-MOK-004` **rule 11**'s test-count figures for the tests
    this stage adds — every figure measured against the tree at the candidate commit and never inferred from an unchanged
    total. **`SPEC-MOK-004` rule 1 does not move.** An earlier draft of this work order made its amendment a precondition,
    on the assumption that a provider package would need a layout entry; the connector lives outside the repository and
    the canned one lives in an existing test tree, so no directory is added and the rule stands unchanged.

## Out of scope

- **`REQ-MOK-075`'s comparison and `REQ-MOK-076`'s general authorization record.** `WO-MOK-027`'s. The one run here is
  authorised for an instrument measurement, not for a published outcome, and its figures are explicitly not the
  comparison.
- **Any run at the full 1,000-tick horizon or over the five-seed set.** That is `WO-MOK-027`'s and needs its own
  authorization *and its own ceiling*: an **estimated** $1.04 per run and $5.20 for five seeds both exceed the **$2** the
  owner declared here, and 1.2 to 2.4 hours per run are not this stage's to incur either.
- **Any change to the port's interface**, the request layout, the transcript format or the fallback rule. Those are
  `WO-MOK-025`'s decided surface; a change to any of them here means that stage was wrong and is an escalation.
- **Any crate added to either Rust package.** The connector's dependencies are the operator's business and are not
  constrained by this repository; the **canned** connector's are constrained, because it lives in a test tree here.
- **A third workspace member, a third package directory, or a connector path compiled into either package as a default.**
  A default path would make a live run reachable without the operator naming anything, which is `REQ-MOK-072`'s gate
  defeated by a constant.
- **Any live path in the observer.** `REQ-MOK-077` prohibits it. What this stage adds to the observer is three refusals.
- **Any second provider, any model other than the declared one, any reasoning level other than `none`**, and any
  temperature or seed parameter — neither is documented for `gpt-5.6-luna` and `SPEC-MOK-007` therefore relies on
  neither.
- **Any concurrency across Mokiterions.** Latency is the reason it is tempting and `SPEC-MOK-007` rule 16 is the reason
  it is not available: it would change the state each request describes.
- **Any credential in any workflow**, any live selection in any workflow, and any relaxation of `L21a`.

## Authorized decision envelope

The implementation agent may decide locally:

- The **canned** connector's language and structure, subject to being a test fixture in an existing package's test tree
  and to `S2`. A connector an operator writes is outside this decision and outside this repository.
- The line protocol's exact framing and error taxonomy, subject to `SPEC-MOK-007` rule 10's fields and to the protocol
  being documented well enough that a third party can implement a connector from the document alone. That documentation is
  the deliverable that replaces the provider program an earlier draft of this work order would have shipped.
- The spawn's details: how the child is started, how its streams are connected, how it is reaped, and what happens when it
  exits early — subject to `C1`, to `R1` and `R2`, and to no path or process reaching the library target.
- Its internal structure, its error taxonomy and its logging, subject to `C1`.
- The retry count, the backoff shape and which transport failures are retried, subject to `R1` and `R2` and to the
  bound being declared rather than implicit.
- The unit-price representation and the cost unit, subject to `P6`'s integer prohibition and to the run record stating
  the unit.
- How the ceiling's pre-exchange check estimates an exchange's cost, subject to `L19`: the estimate may be conservative
  but must never permit a crossing.
- Where the canned connector's fixture lives within an existing package's test tree, and what it is called. No layout
  amendment is needed for it, which is why this is a local decision rather than the technical owner's.

The agent may **not** decide: the model identifier; the reasoning level; whether both gates are required; whether the
ceiling check precedes the exchange; whether reported usage or an estimate is authoritative; whether the observer may
accept any of the three new options; whether a connector path may have a compiled-in default; the horizon, seed set or
ceiling of the live run; or whether the live run happens at all. The last three are the owner's, in the authorization
record, and the ceiling is already fixed at **$2**.

## Constraints

- **No live run occurs without a written owner authorization naming the horizon, the seed set and the spend ceiling.**
  This is the constraint the owner stated in their own words — *an explicit permission from the repository owner is
  needed to launch a real run* — and it is not satisfied by the approval of this work order. The ceiling for this stage's
  one run is **$2**, already decided; the horizon and seed still come from the authorization.
- **The credential never enters the repository, the library target, any workflow, or any produced byte.** Four
  prohibitions, from `REPOSITORY_CONTEXT.md`, `ADR-MOK-001`, `REQ-MOK-073` and `C1` respectively.
- **The ceiling is a stop, not a report.** A run that exceeds its ceiling and says so has failed `REQ-MOK-071`.
- **The engine's library target still opens nothing, spawns nothing and reads no environment variable.** The binary
  target connects the streams and starts the child; the prohibition from `WO-MOK-025` stands and `S3` and `S3a` are
  re-run. This is the constraint this stage is most able to break, because a spawn is the natural thing to put beside the
  code that needs its output.
- **No option this stage adds is accepted anywhere it cannot be honoured.** Three new options reach the engine's shared
  parser, and the observer forwards to that parser. Each must be diagnosed in the observer rather than accepted and
  ignored — `SPEC-MOK-007` rule 18.4.2, and the reason is `SPEC-MOK-003`'s own disclosure of GitHub issue 40.
- **The four existing sources stay byte-identical.** `L9` and `L10` are re-run at the candidate commit, against
  `WO-MOK-025`'s base-commit captures.
- **Replay stays the default.** After this stage a run with no live-mode selection still behaves exactly as it did
  before it.
- **The live run's transcript, record stream, run record and authorization are committed together.** A live run whose
  evidence is incomplete cost money and produced nothing citable.
- **The evidence path is named before the first capture.** It becomes provenance the moment a record binds it, and a
  rename forces a whole fresh capture — which here means paying for a second live run.
- **Governance artifacts are written CRLF and retained evidence LF**, as `WO-MOK-025` also requires. This stage's
  transcript, record stream and run record all land under `docs/engineering/simulation/evidence/**`, where
  `.gitattributes` disables end-of-line conversion, so their bytes are hashed exactly as written.

## Expected change surface

- **The connector protocol document**: the messages, their fields, their framing, the error cases, and a worked example
  — enough for a third party to write a connector without reading this repository's Rust.
- **The canned connector**: a test fixture in an existing package's test tree, speaking the protocol, answering from a
  script, and able to fail on command.
- **The engine's binary target**: the connector path re-read from the raw arguments, the spawn, the stream connection and
  the reap, the environment pass-through, the live-mode selection, the ceiling option, and the usage text for all three
  new options.
- **The engine's shared parser**: three options recognised, validated, enforced at-most-once, and — for the two that carry
  paths — their values discarded, on the `--events-path` precedent. The configuration value the library holds gains no
  field, which `S6a` checks.
- **The observer's option parsing**: three refusals, each diagnosed before the terminal is entered.
- **The engine's library target**: the usage figures on the transcript record, the accounting accumulator, the
  pre-exchange ceiling check, the ratio computation, and the run record's new fields. No new public item beyond what
  rule 15 needs.
- **The engine's test tiers**: the cases named below.
- **The evidence path**: the live run's four artifacts and the two attestations.
- **`REPOSITORY_CONTEXT.md`**, `SPEC-MOK-003`'s *Start-up inputs*, and `SPEC-MOK-004` rule 11, per the amendments.
  `SPEC-MOK-004` rule 1 is not among them.

## Required verification

`VER-MOK-018`, the cases `WO-MOK-025` could not reach plus the ones this stage's code creates:

**Owner-gated, and therefore dependent on the authorised live run**: **L15b** — over that run, cached prompt tokens are
at least eighty-five percent of total prompt tokens.

**Matrix cases**: **L16** and **L17** re-run against a live transcript rather than a stub's, **L18**, **L19**, **L20** in
full including the live half, **L21a** and **L21b** re-run, **L22** with a real transport failure, **L26** re-run,
**L29** re-run with the canned connector standing where `WO-MOK-025` used an in-process stub, and **L32** in full —
including the connector-path, live-mode and ceiling halves `WO-MOK-025` could not reach because the options did not
exist. **L30**, **L31** and **L33** are re-run unchanged, since this stage moves `execute`'s and `advance_tick`'s
signatures no further.

**Acceptance scenario A4**, **A7** in full including its refusal half, and **A1**, **A5** and **A6** re-run — **A1**
because "a run nobody paid for" is the property most at risk from this stage's existence.

**Properties P5** and **P6** against real usage figures, and **P1** and **P7** re-run against the live transcript.

**Static checks S1**, **S2**, **S2a**, **S3**, **S3a**, **S5** and **S5a**, with **S4a**, **S6a** and **S6b** re-run.
**S4a** is re-run rather than skipped because it costs nothing and because a stage that adds options to both hosts is
exactly where a fifth parameter could reappear on a signature this stage claims not to touch. **S2** now
applies, in exactly the scope `VER-MOK-018` gives it: the **canned** connector's dependency declaration and its reaching
no network. It says nothing about an operator's connector, and the report states that limit rather than implying
coverage. **S3a** applies in full for the first time — the spawn and the environment pass-through appear in the engine's
binary target and nowhere else in either package, and the observer's source contains no spawn at all.

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
   ceiling — **$2**, as decided on 2026-08-23 — and the purpose stated as an instrument measurement rather than a
   published figure. The record also states that this authorization does not extend to `WO-MOK-027`'s five-seed
   measurement, whose **estimated** $5.20 exceeds it.
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
11. **The canned connector's dependency declaration** and the `S2` output, with the statement that no check here sees an
    operator's connector.
12. **The connector protocol document** as shipped, and evidence that the canned connector was written against the
    document rather than against the engine's internals — the check being that the document alone is sufficient.
13. **The spawn and pass-through evidence** for `S3a`: that the spawn appears only in the engine's binary target, that the
    library target contains none, that the observer contains none, and that a synthetic credential placed in the parent's
    environment reached the child and appeared in no produced byte.
14. **The observer's three refusal outputs** for `L32`: connector path, live-mode selection and ceiling, each with its exit
    status and its standard-error bytes, and each shown to have been produced before the terminal was entered.
15. **The two attestations**, `C6` and the credential attestation, each naming the owner and the date.
16. **The four existing sources' re-comparison** against `WO-MOK-025`'s base-commit captures.

## Stop and escalate conditions

1. **The line protocol cannot be documented well enough for a third party to implement a connector from the document
   alone**, or the canned connector cannot be written against the document rather than against the engine's internals.
   Either means the connector is not really external, and `ADR-MOK-007` decision 3's whole value — that no dependency
   artifact moves — was bought with a boundary that does not exist. Do not resolve this by pulling the connector into the
   workspace; that is the option the owner superseded on 2026-08-23 and it is theirs to reconsider.
2. **The spawn, the stream connection or the reap cannot be built without the library target acquiring a path, a process
   or an environment read.** That is `SPEC-MOK-006` rule 1.2 and the shape of the port parameter is the owner's.
3. **The cache ratio comes in below eighty-five percent.** Escalate rather than adjusting the threshold, the layout or
   the measurement. `REQ-MOK-070` is an obligation on the design, so a miss means the design is wrong or the number was
   wrong — and which of those it is, is the owner's to decide. Report the measured value, the split that produced it,
   and whether the provider's cached-token reporting behaved as documented.
4. **The provider's reported usage does not distinguish cached prompt tokens.** `REQ-MOK-069` and `REQ-MOK-070` both
   rest on that field. Its absence makes the ratio unmeasurable and is a fact about the provider, not a defect to work
   around.
5. **The provider rejects, truncates or reorders the request**, or its caching does not key on the prefix as documented.
   Any of these invalidates the layout `SPEC-MOK-007` rule 3 fixes.
6. **A credential appears in any produced byte.** Stop, do not commit the evidence, and escalate. Committed evidence
   containing a credential cannot be corrected — the artifact is bound and the credential is disclosed.
7. **The live run would exceed its ceiling to complete.** It stops; that is the design. But if the ceiling proves too
   low to reach 200 exchanges, escalate for a new authorization rather than raising the ceiling locally.
8. **The run's actual cost exceeds the estimate materially** — take a factor of two as material. The estimate is what
   the owner's authorization was given against, so a large miss makes the next authorization's basis wrong.
9. **A retry policy cannot be bounded** without either dropping exchanges or extending a run indefinitely.
10. **Any pressure arises to make a workflow do a live run**, for any reason including convenience of measurement.
    `L21a` and `C6` are the whole cost containment.
11. **The synthetic transcript from `WO-MOK-025` and the live transcript disagree in form.** That means the stub was not
    faithful, and every offline case verified against it is weaker than its record claims.

## Completion report format

1. **What was built**, against the *In scope* list, each item done or escalated.
2. **The connector boundary**: the protocol document as shipped, the canned connector's language, dependency declaration,
   `S2` result and location, and a plain statement that nothing here establishes anything about an operator's connector.
3. **The spawn**: where it lives, what the child inherits, how it is reaped, and the `S3a` evidence that neither the
   library target nor the observer contains one.
4. **The observer's three refusals**, each with its exit status and its message, and the confirmation that none was
   accepted-and-ignored.
5. **The gate matrix**, all four combinations, with the evidence path for each.
6. **The live run**: its authorization, its seed, its horizon, its exchange count, its ceiling, its actual cost, its
   fallback count, and its cache ratio — with the ratio stated against the eighty-five percent obligation as pass or
   fail, plainly, before any explanation.
7. **The estimate against the measurement**: the per-run cost, the token split, the cached share and the latency, each
   as estimated in `SPEC-MOK-007` and `ADR-MOK-007` and as measured here, with the difference stated as a factor.
8. **Each verification case** in the required list, with its result and its evidence path.
9. **Every credential-handling decision**, so the owner can see the whole path a secret takes.
10. **The amendments made**, each with its artifact, provision and authorising act.
11. **What was not verified and why** — at least `L24`, `L25` and `M3`, which are `WO-MOK-027`'s, and the dependency
    surface of any connector this repository does not own, which `S2` cannot reach and `VER-MOK-018` records as a limit
    rather than a gap.
12. **Every local decision** taken under the envelope, and **every escalation** raised, with its resolution.

## Amendment record

**2026-08-28, `[execution_scope]` added, under `WO-HUP-002`, by the engineering owner.**

This work order was approved on 2026-08-23 under the `se_harness` 0.4.0 work-order template, which carried no
`[execution_scope]` table. The 0.8.0 root adopted under `WO-HUP-001` requires one to start work and enforces it
at the `start` checkpoint, so from that adoption until this amendment the work order was authorized and
unstartable, refused with `QGP-G3-SCOPE: WO-MOK-026 has no assessable execution scope`.

The table is derived item by item from this work order's own *Expected change surface*, which is unchanged. The
mapping from each surface item to the path admitting it is retained in
`../../harness/evidence/WO-HUP-002/`. Two paths are **owner decisions of 2026-08-28 rather than derivations**,
because the surface text does not settle them: `mokiterions-core/Cargo.toml`, because the canned connector must
be a real child process and therefore a declared target, and `docs/CONNECTOR_PROTOCOL.md`, because the surface
requires a connector protocol document without saying where it lives.

Nothing else in this artifact moves. Not `status`, not a relation, not an assurance field, and not one word of
the scope prose the table is derived from. This amendment changes what this work order *declares*, never what
it delivers.

**2026-08-29, `SPEC-MOK-002.md` admitted to `[execution_scope]` and to `[relations].specifications`, by the
repository owner acting as accountable engineering owner.**

`WO-MOK-030` added `SPEC-MOK-007` rule 14.3a on 2026-08-29 — the unit prices "arrive through `--prices`", and
"the shared parser validates it and **retains** the four values, like `--spend-ceiling` and unlike the paths,
because the run computes with them". The library is what computes with them: `SPEC-MOK-007` rule 14.6 stops the
run *before* an exchange and rule 15.2 puts the cost in the run record, which the library writes to a sink the
host lends it. So retaining the four values means a sixth public field on `simulation::Config`, and `SPEC-MOK-002`
rule 5's census enumerates that struct's public fields **exactly** and closes with "nothing outside the three
lists becomes public".

That file was outside this work order's scope, measured rather than assumed:

    QGP-G4I-PATHS: WEX201: changed path is outside execution scope:
    docs/engineering/simulation/specifications/SPEC-MOK-002.md

and the harness's own next step was to escalate under `DR-REMEDIATION-SCOPE`, which is what happened. So this is
a seventh gap of the same kind as the six `WO-MOK-030` closed, found one commit into the implementation rather
than by the conformance pass: that pass amended the census for `spend_ceiling`, which a commit had already added,
and did not amend it for the field the option it was creating in the same act would require.

Every alternative route was also an interface change, and `SPEC-MOK-002`'s own 2026-08-29 row had already worked
the identical question through for `spend_ceiling` one day earlier: "a sixth `execute` parameter moves rule 4,
and putting the ceiling on the port leaves the library unable to write the run record rule 15.2 requires". The
same holds of the prices, for the same two reasons, so nothing here re-derives it.

The alternative put to the owner was a fourth governance work order scoped to `SPEC-MOK-002.md` alone, following
`WO-MOK-028`, `WO-MOK-029` and `WO-MOK-030`. It was **declined** in favour of this amendment. Its cost was two
stacked pull requests; this amendment's cost is that this work order's own diff carries a specification amendment
and that the formal snapshot moves from `47aad296aa8686c64d37453fe230124226823260881163bd9da670714d7eac3e`.
The second cost is nil at this moment and only at this moment: the `handoff` check reports no evidence bound to
that snapshot, and no verification record is prepared, so nothing has to be re-captured. A later amendment would
have paid for a live run twice.

**The ordering is right this time and that is the point of the amendment.** The census is amended before the
field exists, not after — which is what `SPEC-MOK-002`'s 2026-08-29 row recorded as wrong about `spend_ceiling`
and "recorded rather than tidied". No gate would have caught the alternative: `validate` reads that census as
prose and cannot compare it to a struct.

Nothing else in this artifact moves. Not `status`, not an assurance field, not the scope prose, and not one item
of *In scope*, *Out of scope* or the *Expected change surface*. The prices are already **item 5**'s — "the engine
needs them for its ceiling arithmetic, so they are declared to the engine per rule 14. The engine holds prices;
it holds no endpoint" — and items 9, 10 and 11 are what compute with them. What this amendment admits is the
artifact that has to authorize the field, not any new work.

**2026-08-29, `SPEC-MOK-007.md` admitted to `[execution_scope]`, by the repository owner acting as accountable
engineering owner.**

Two provisions of that specification have to move before **item 8** can be written, and both were found by reading
the specification against the code rather than by a gate.

**The port's return type, rules 1.1 and 1.4.** Rule 11.3 obliges an exchange record to carry "the response as
received, in full, or the error", the provider's four reported counts, and "the action the response was parsed
into, or the fact that it was not parsed **and why**". Rule 11.1 puts the authoring of every record in the engine,
so the engine has to be told all three. It cannot be: rule 1.1's interface "returns either a proposal or the fact
that none was obtained" and rule 1.4 fixes that as "a value of the engine's existing action type, or as the absence
of one", which carries no response text, no count and no reason. `mokiterions-core/src/simulation.rs` has recorded
this since `WO-MOK-025` as "a pre-existing tension between rule 1.1's port shape and rule 11.3's field list", and
named this work order as where "the port's return type has to grow to carry them". Rule 11.3.1 says the same from
the other side: the two fields are "present and empty until a provider is called" and "`WO-MOK-026` is where either
first carries a value".

**Rule 19.2's credential condition cannot be reached.** It makes "a live-mode selection with no credential" a usage
error exiting "before any tick runs and before any provider call". Rule 13.1 puts that condition's check in the
connector — "the selection by the host, the credential by the connector — and neither component can satisfy the
other's condition" — rule 13.3 has the refusal "arrive after the connector was spawned", on the first exchange, and
rule 13.4 forbids either host to read the credential at all. So no host can detect the condition before a tick, and
rule 19.5a, amended one day later, makes a `refused` response an unconditional "immediate counted fallback" which a
host cannot except the credential case out of without interpreting a message rule 13.3 has it pass through in the
connector's own terms. The list item is a defect and is to be recorded as one at the rule.

That file was outside this work order's scope, measured rather than assumed:

    QGP-G4I-PATHS: WEX201: changed path is outside execution scope:
    docs/engineering/simulation/specifications/SPEC-MOK-007.md

and the harness's own next step was again to escalate under `DR-REMEDIATION-SCOPE`, which is what happened.

**Three routes were put to the owner with each one's cost measured, and the other two were declined.** The first
declined route was to leave rules 1.1 and 1.4 word for word and record in `SPEC-MOK-002` rule 5 that they already
admit evidence travelling beside the proposal, on the ground that rule 1.4's own stated purpose — "nothing arriving
through it can bypass rule 9's validation by being expressed in some other form" — holds either way. Its cost was
that a later reader meets a rule whose plain words and whose build disagree, reconciled in a different artifact. The
second declined route was a third method on the port, leaving both rules untouched and amending only the census: the
engine would ask the port for the exchange's evidence after each proposal. Its cost was a temporal contract between
two calls that no type enforces, so a port returning the previous exchange's evidence writes a wrong record in
silence — the one hazard the chosen route closes in the type. The chosen route's cost is that this work order's diff
carries a second specification amendment and that the formal snapshot moves again, from
`670b8733a05aa5af74157a2f2e78dfa8401fe14b1c502f4ae7f16d42eba39309`. That cost is still nil and still only at this
moment, for the reason the row above gives: the `handoff` check reports no evidence bound to that snapshot and no
verification record is prepared, so nothing has to be re-captured.

**Neither provision is a relaxation.** The proposal still crosses back as an action or its absence and the engine
still reads nothing else to decide, so rule 9's validation is untouched; what grows is what the *record* can say
about the exchange the proposal came from. And rule 19.2's defect is recorded rather than repaired by weakening a
gate: the run follows rule 19.5a, which is both the more specific rule and the later-dated one, and the connector's
message reaches the transcript's `response` field verbatim, which is what rule 13.3 and `VER-MOK-018` case `L20`
actually require — `L20` says "no provider call occurs and the run reports which condition was missing without
printing any value", and says nothing about an early exit.

Nothing else in this artifact moves. Not `status`, not a relation — `SPEC-MOK-007` has been in
`[relations].specifications` since approval — not an assurance field, not the scope prose, and not one item of *In
scope*, *Out of scope* or the *Expected change surface*. **Item 8** is already "the usage accounting: the provider's
reported counts into the transcript's reserved fields", and this amendment admits the artifact that has to authorize
the interface item 8 needs, not any new work.

**2026-08-29, `mokiterions-tui/src/state.rs` admitted to `[execution_scope]`, by the repository owner acting as
accountable engineering owner.**

`mokiterions-tui/src/state.rs:319` holds `LentPort`, the observer's implementation of `simulation::Proposer`: a
newtype over the port the observer owns, forwarding `propose` to it, which exists because `SPEC-MOK-007` rule 20.4
puts the port in the host and the observer's own state is what holds it for the whole run. A trait method's signature
and its implementations' cannot differ, so the amendment the row above authorizes forces this file. It was outside
this work order's scope, measured rather than assumed:

    QGP-G4I-PATHS: WEX201: changed path is outside execution scope:
    mokiterions-tui/src/state.rs

**This is a mechanical consequence of the port-shape decision and not new work, and the reason it is recorded rather
than waved through is that all three routes put to the owner forced it.** Growing the return type, re-reading rules
1.1 and 1.4 to admit the same growth, and adding a third method to the port each change the trait, so each obliges
`LentPort` to follow. The file's admission was therefore entailed by that decision rather than chosen after it — and
it is written down because nothing in this repository is approved by implication.

The alternative put to the owner was a fifth governance work order scoped to this file alone, on the `WO-MOK-028`,
`WO-MOK-029` and `WO-MOK-030` pattern. It was **declined**: its cost was a second stacked pull request for a
forwarding change of three lines, and it would leave the port half-grown in this work order's tree until that one
merged — a tree that does not compile, which is worse than the scope breadth it would buy back.

**What this admission does not license.** The observer remains the **replay** host of rule 20.1 and gains no
capability here. It spawns no connector, reads no credential and makes no provider call; `state.rs` may change only
so far as `Proposer`'s signature obliges, and rule 20.3's refusal and rule 20.2's measured reason for it are
untouched. The three new options are already **item 6**'s, which the observer refuses, and that item landed before
this amendment.

Nothing else in this artifact moves. Not `status`, not a relation, not an assurance field, not the scope prose, and
not one item of *In scope*, *Out of scope* or the *Expected change surface*.
