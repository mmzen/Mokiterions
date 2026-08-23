+++
id = "INT-MOK-011"
type = "intent"
title = "Let a language model decide, and record what happens without deciding in advance what should"
status = "approved"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"

[relations]
+++

# Intent: Let a language model decide, and record what happens without deciding in advance what should

## Problem

Four decision sources exist and every one of them is a rule this repository wrote. `baseline` chooses uniformly at
random among the actions rule 4 offers it. `reference` walks to the nearest resource and refuses waste. `individual`
does the same with a per-Mokiterion waste tolerance the seed fixes. `social` adds seven targeted verbs governed by
`fear`. Each was specified before it was built, and each is held to an outcome its specification names.

That is the right way to build a simulation and it is the wrong way to find out anything. The behaviour of all four is
a restatement of their own rules, and `README.md`'s second objective — whether anything worth calling emergence
appears — cannot be answered by a source whose every move was written down first.

A model-backed source is different in kind. Nobody writes what it will propose. It is handed one Mokiterion's
observation and it answers with one action, and what the population then does is a finding rather than a
reimplementation. `ROADMAP.md` has carried this as Phase 5 since the roadmap was written, and has deferred it as *"the
only expensive, nondeterministic, credential-bearing work."*

Two of those three words are now measurable rather than feared. **Measured** at `2a93914`, a 1,000-tick run under
`social` at the default density takes 10,954 decision opportunities, and one opportunity is one model call;
**published** prices for `gpt-5.6-luna` on 2026-08-23 put a five-seed measurement at an **estimated** $5.18 with the
prompt laid out for the provider's cache. Expensive is not the obstacle. Nondeterministic is real and is solved by
recording, not by asking a vendor for a guarantee no vendor gives. Credential-bearing is real and is solved by
keeping the credential outside the engine, which `ARCH-MOK-001` already decided it would be.

The obstacle that remains is the one this intent exists to remove: **there is no honest way to hold a model-backed
source to an outcome, and every requirement this repository has written so far holds its source to one.** Four
sources in a row received a survivor floor. A fifth that received one would be built until it met the floor, and the
figure would then describe the tuning rather than the model.

## Desired outcomes

- A Mokiterion's next action can come from a language model, one decision at a time, through the same validation
  every other source's proposal passes. The engine remains the only authority over what actually happens.
- A run driven by the model can be **replayed exactly**, offline, at no cost, by anyone holding the retained
  transcript. What the model said is evidence, not an event that happened once and cannot be examined again.
- What the population does under the model is **recorded and published without a pass or fail attached**. Survivors,
  deaths, combat deaths and resource composition are observations. No number in this initiative is a target.
- The comparison against `reference` and `social` on the same seeds and horizon is made and published **whatever it
  shows**, including the finding that the model is indistinguishable from a source that costs nothing.
- The credential and the socket stay outside `mokiterions-core`, and the engine's declared dependency set stays
  empty. The property *"the engine cannot make a network call"* survives this initiative intact.
- The cost of a run is bounded before the run starts, and a run cannot begin without the repository owner having
  authorized it.

## Actors and stakeholders

| Actor | Interest |
|---|---|
| Repository owner | Holds every governance role. Authorizes each live run individually, and bears the provider bill |
| Product owner | Decides that this source carries no outcome floor, and that the comparison is published either way |
| Technical owner | Decides where the provider lives and what may enter each package's dependency set |
| Assurance owner | Verifies an instrument rather than a result, which is a different contract from the four before it |
| Reader of a published figure | Needs to know which figures are obligations and which are observations. This initiative produces only the second kind, and says so |
| A later implementer | Needs the retained transcript to reproduce a run without a credential, a network or a budget |

## Success measures

Every row is a property of the instrument. **No row names a survivor count, a death count or a composition share**,
and *Non-goals* explains why.

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Runs replayable byte-identically from a retained transcript, offline | n/a | 100% | every recorded run |
| Provider calls made by continuous integration | 0 | **0** | every CI run, permanently |
| Existing four sources' output bytes changed by this work | n/a | **0** | every declared seed, every policy |
| Decision requests containing another Mokiterion's attribute | n/a | **0** | every request in every retained transcript |
| Silent fallbacks to a rule-based source in a run that sources a published figure | n/a | **0** | every measurement run |
| Cached share of prompt tokens on a live run | n/a | **≥ 0.85** | every live run |
| Live runs executed without a retained owner authorization | n/a | **0** | permanently |
| Actual spend against the ceiling declared for the run | n/a | **within** | every live run |

## Non-goals

- **No survivor floor, no death floor, no composition ceiling for this source.** This is the substantive decision of
  the initiative rather than an omission from it. `REQ-MOK-014` binds `reference`, `REQ-MOK-034` binds the trait-aware
  source, `REQ-MOK-058` binds `social`, and `REQ-MOK-060` binds those three by name; each names its source inside its
  own statement, so none of them reaches a fifth. **None is amended, relaxed or weakened by this initiative** — they
  are simply not extended, and `baseline` is the standing precedent for a source that carries no floor and goes
  extinct on every declared seed as a recorded measurement.
- **No claim that the model behaves better.** Or worse. The comparison is published; the adjective is not.
- **No memory across decisions.** Each decision is answered from its own request alone. This system has one tick of
  memory, the `suffered` record, and a longer window was refused deliberately. If per-Mokiterion memory is ever
  wanted it is engine-owned, bounded and emitted in the record stream, under its own intent.
- **No agent-to-agent communication.** Mokiterions do not negotiate, signal or cooperate. What one perceives of
  another is what `PerceivedMokiterion` carries and nothing more.
- **No provider determinism.** Temperature and seed support are not documented for this model and the design does not
  ask for them. Reproducibility comes from the transcript.
- **No emergence verdict.** Deciding what would count as emergence, and against what, is `ROADMAP.md` Phase 6's
  problem. This initiative produces the material that question needs and does not answer it.
- **No live run outside the engine's own binary.** The observer becomes a host of this source, but a **replay-only**
  one: it may replay a committed transcript and it may not run live. The reason is measured rather than preferred — the
  observer owes a frame every 33 milliseconds and one live tick costs an **estimated** 4 to 9 seconds — and
  `ADR-MOK-007` decision 8 records it. An earlier draft of this intent said "No second host", which was wrong in both
  directions: there are already two hosts of the engine library, and this initiative gives the second one real work.

## Principles and immutable constraints

1. **The engine decides nothing about the network.** No socket, no asynchronous runtime and no model-provider crate
   reaches `mokiterions-core`, in either of its targets. Its declared dependency set stays empty, and `ARCH-MOK-001`'s
   by-name conformance check keeps passing unchanged — which matters because that check scans the engine **package**, and
   the package's own binary target is the host of a live run.
   *Stated precisely, because one word of it is easy to overstate:* **neither target reads a credential.** The engine's
   binary target starts the connector and may hand it the environment it inherited, and a credential in that environment
   passes through untouched — inherited, not read, not parsed, not logged and never in an argument. `SPEC-MOK-007` rules
   10.5 and 13.4 fix this, and the connector is the one component that reads the value.
   The library target keeps a stronger property than the package does: it resolves no path, opens no file, spawns no
   process and reads no environment variable. Both of this initiative's streams reach it as already-open handles from the
   host that owns them, which is `SPEC-MOK-006` rule 1.2 extended to a second stream rather than excepted for one.
2. **The engine remains the sole authority.** A proposal from a model is a proposal. It is validated, and rejected
   when illegal, by the same code path that validates the other four sources.
3. **A decision request carries one Mokiterion's observation and nothing else.** No attribute of another Mokiterion,
   no population aggregate, nothing derived from another request or another response. `REQ-MOK-059` already forbids
   the aggregate; this initiative extends the same discipline to the request.
4. **A run that cannot be replayed is not evidence.** The transcript is retained with the run, and a verified run is
   a replay run.
5. **The four existing sources do not move.** Same bytes, same records, same entropy draw counts, same exit codes.
6. **Nothing bills without the owner's word for that run.** Not a test, not a workflow, not a default, not a retry.
7. **An observation is never reported as a check.** Where this initiative measures the world rather than the
   instrument, the figure is published with no pass condition attached to it.

## Risks and assumptions

**Facts, measured at `2a93914` or published by the provider on 2026-08-23.**

- 10,954 decision opportunities in a 1,000-tick `social` run at seed 0 and the default density; 1,200 in a 100-tick
  `reference` run.
- `gpt-5.6-luna`: $0.20 per MTok input, $0.02 per MTok cached input, $1.20 per MTok output, cache writes at 1.25×
  uncached input, 1.05M context, 128K maximum output, reasoning levels none through max, structured outputs and
  function calling both supported.
- Decisions inside one tick are sequential by specification. An agent observes what the previous one left, so twelve
  concurrent calls from the tick-start state would change the simulation rather than accelerate it.
- `Observation.valid_actions` excludes every targeted action by design. Rule 6, not that field, is the set a source
  may legitimately propose from.
- All four existing outcome obligations name their decision source inside their own statement.

**Assumptions, each of which a later measurement may overturn.**

- Round-trip latency at reasoning `none` is 0.4–0.8 s, making a 1,000-tick run 1.2–2.4 hours. *Estimate.* It cannot
  be measured without a provider call and is measured before any horizon is fixed.
- A prompt of roughly 1,230 cacheable and 200 variable tokens produces a cached share at or above 0.85. *Estimate.*
- Cache entries stay warm across a run because each Mokiterion's prefix is re-touched roughly every six seconds under
  round-robin ordering. *Estimate.*
- A model handed the rule 6 set will mostly propose from it. The rejection rate is unknown and is measured.

**Decisions this intent does not take. All four were taken by the repository owner on 2026-08-23 and are recorded in
`ADR-MOK-007`'s *Decision record*; they are listed here with their answers so that this intent is not read as still
open.**

- **Where the provider lives**: settled as an **external connector the operator names by path**, spawned by the host, not
  a package in this workspace. `REQ-MOK-050` does not move, and neither does any other dependency artifact.
- **Whether a short committed transcript may be replayed in continuous integration**: settled as **yes**, and it is the
  whole of verification tier 1. Replay costs nothing, makes no call and needs no credential.
- **The spend ceiling to declare**: settled as **$2**, which governs the single instrument run of `WO-MOK-026`. It is
  below the **estimated** $5.20 that five 1,000-tick seeds cost, so the measurement stage needs its own ceiling.
- **The horizon and seed set for the measurement stage**: **deferred**, deliberately, until `WO-MOK-026` has measured the
  real per-exchange cost and latency. The deferral is a decision and `WO-MOK-027` records it as one.
