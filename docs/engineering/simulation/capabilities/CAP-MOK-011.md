+++
id = "CAP-MOK-011"
type = "capability"
title = "Model-backed decisions: a decision port outside the engine, an exactly replayable transcript, and a source held to no outcome"
status = "draft"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"

[relations]
derives_from = ["INT-MOK-011"]
+++

# Capability: Model-backed decisions

## Actor and need

The **operator** — in this repository the owner, who also holds every governance role — needs to run the simulation
with each Mokiterion's action chosen by a language model rather than by a rule, and needs the result to be examinable
afterwards by someone with no credential, no network and no budget.

Three needs sit behind that, and they pull in different directions.

The operator needs the run to be **cheap enough to repeat and bounded enough to be safe**. Repeating is how a single
run stops being an anecdote; bounded is how an unattended loop stops being a bill. The **assurance owner** needs
something to verify, and for the first time in this repository it cannot be an outcome: nobody can say in advance what
the population should do, so what is verified is the instrument that produced the observation. The **reader of a
published figure** needs to be able to tell an observation from an obligation, because this repository has trained its
readers that a number in a requirement is a promise, and the numbers here are not promises.

## Capability statement

`An operator can run the simulation with every Mokiterion's action supplied by a language model, one isolated decision
at a time, under an authorization they gave for that run and a spend ceiling they declared for it, and can afterwards
replay the run exactly, offline and free, from the transcript the run retained.`

## Boundaries

**What the model decides.** One action for one Mokiterion at one decision opportunity, chosen from the set rule 6
permits it to propose. Nothing else. The model does not resolve the action, does not learn whether it succeeded except
by observing the next tick's state, and does not see the world outside its own perception radius.

**What the engine keeps.** Everything else. The engine owns all mutable state, advances the tick, validates every
proposal and rejects an illegal one exactly as it does for the other four sources. `ADR-MOK-001`'s trust boundary is
not moved by this capability: a decision source was always untrusted, and this one is untrusted in the same way and
by the same code.

**What crosses the boundary.** A serialized projection of one `Observation`, by value, and one action by value. No
mutable borrow, no reference into engine state, no handle that permits mutation. `Observation` and `DecisionSource`
stay private types; what becomes public is a stream-carried protocol, on the precedent `SPEC-MOK-006` set for the
record stream and `WO-MOK-019` set for a caller-owned sink.

**What does not cross it.** No socket, no credential, no asynchronous runtime, no model-provider crate. The engine
package's declared dependency set stays empty and `ARCH-MOK-001`'s by-name conformance scan keeps passing without an
amendment. Where the provider lives is `ADR-MOK-007`'s decision; that it does not live in `mokiterions-core` is not
open.

**What no request contains.** Any attribute of any other Mokiterion, any population aggregate, and anything derived
from another request or another response. `PerceivedMokiterion` already carries only an identifier, a direction and a
distance, and `REQ-MOK-059` already forbids the aggregate; this capability adds that a request may not reintroduce
either by another route.

**What no decision remembers.** Nothing. Each decision is answered from its own request. A shared preamble is
permitted and carries no Mokiterion's information, because it is byte-identical for every Mokiterion and every tick.
Per-Mokiterion memory is out of scope and would need its own intent.

**What this capability is not held to.** Any survivor count, death count, combat-death count or resource-composition
share. `REQ-MOK-014`, `REQ-MOK-034`, `REQ-MOK-058` and `REQ-MOK-060` each name their decision sources inside their own
statements and therefore do not reach this one. **None of them is amended by this capability and none is weakened**;
they are not extended, which is a different act. `baseline` is the precedent: it carries no floor, goes extinct on
every declared seed, and that extinction is recorded as a measurement and never as a failure.

**Where the run may not happen.** In continuous integration, in a unit test, in an integration test, on push, on
merge, on tag, or on release. A real run needs the repository owner's authorization for that run, retained with the
run's evidence.

## Outcomes

| Outcome | How it is observed |
|---|---|
| A model-backed source exists and is selectable beside the other four | the source's own name in the record stream's `decision_source_selected` record |
| A run's every decision is retained with its request, its response and its token usage | the transcript, retained as work-order evidence |
| Any retained run replays byte-identically, offline, at no cost | standard output, record stream and exit code compared with `cmp` |
| The four existing sources are unchanged | byte comparison of output and record streams at matched seeds, and per-tick entropy draw counts |
| No request carries another Mokiterion's state | a check over every request in a retained transcript |
| A live run's cached prompt share is at or above the declared threshold | the provider's own `usage` figures, recorded per call |
| A run stops rather than exceeding its declared ceiling | the refusal, reported in the run record |
| No provider call is reachable from continuous integration | no workflow references a provider credential, and none is held in the repository's CI secrets |
| Every live run has a retained authorization naming its owner, horizon, seeds and ceiling | the authorization file in the run's evidence |
| The comparison against `reference` and `social` is published whatever it shows | the measurement work order's retained report |

## Candidate requirements

Fifteen, `REQ-MOK-063` through `REQ-MOK-077`. Grouped by what they are for rather than by number.

**The source and the boundary.**

- `REQ-MOK-063` — obtain the action from the decision port and validate it like any other proposal.
- `REQ-MOK-064` — present the set rule 6 permits, targeted actions included, not `Observation.valid_actions`.
- `REQ-MOK-065` — carry no other Mokiterion's state, and nothing derived from another exchange, into a request.
- `REQ-MOK-066` — answer each decision from a self-contained request that carries no earlier exchange.

**The hosts.** One requirement, added after the other fourteen because the design that needed it came later: the
repository owner's decision of 2026-08-23 that the terminal observer hosts this source too, for replay only.

- `REQ-MOK-077` — replay a model-backed run in either host, and run live only from the engine's binary. The prohibition
  is the substance: the observer owes a frame every 33 milliseconds and one live tick costs an **estimated** 4 to 9
  seconds, so a live run there is not slow but impossible, and no other requirement here says so.

**Evidence and determinism.**

- `REQ-MOK-067` — replay a recorded run byte-identically from seed, configuration and transcript.
- `REQ-MOK-068` — leave the four existing sources byte-identical, entropy draws included.
- `REQ-MOK-069` — record every exchange, with the provider's reported token counts.
- `REQ-MOK-074` — count every fallback, name it in the run record, and disqualify a run that has any from sourcing a
  published figure.

**Cost, which is an obligation here rather than an optimisation.**

- `REQ-MOK-070` — hold the cached share of prompt tokens at or above the declared threshold.
- `REQ-MOK-071` — stop rather than exceed the run's declared spend ceiling.

**Authorization.**

- `REQ-MOK-072` — make no provider call without both an explicit live selection and a credential from the environment.
- `REQ-MOK-073` — keep every provider credential out of continuous integration.
- `REQ-MOK-076` — retain an owner authorization for every live run.

**Publication.**

- `REQ-MOK-075` — report the comparison against the reference and social sources at the same seeds and horizon, with
  no floor attached to any figure it reports.
