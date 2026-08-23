+++
id = "REQ-MOK-066"
type = "requirement"
title = "Answer each decision from a self-contained request that carries no earlier exchange"
status = "draft"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN a decision request is issued, THE SYSTEM SHALL make that request complete on its own and SHALL include in it no request and no response from any earlier decision opportunity."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Answer each decision from a self-contained request that carries no earlier exchange

## Rationale

The repository owner decided on 2026-08-23 that isolation means **one fresh context per decision**, as a first step.
`REQ-MOK-065` keeps other Mokiterions out of a request; this requirement keeps *earlier ticks of the same Mokiterion*
out of it, which is a separate property with separate reasons.

The weaker reason is cost. A conversation that accumulates for a thousand ticks reaches roughly $36 per run with
caching and an **estimated** $352 without it, against $1.04 for fresh requests — a factor of thirty-odd for memory
nobody specified.

The stronger reason is that accumulated context is **state that decides behaviour and that nothing retains**. It would
live in a vendor's context window, appear in no event, no metric and no record-stream line, and it would make a later
reader of a retained transcript unable to reconstruct why a Mokiterion acted. That contradicts this project's own
observability goal, and it would quietly reverse a decision already taken: this simulation has one tick of memory, the
`suffered` record, and a longer window was refused on the ground that a longer window is a stored relation while the
contact rule is deliberately positional.

There is a third reason that matters for evidence. A self-contained request is **independently replayable**: any single
exchange in a transcript can be examined, re-sent or reasoned about without reconstructing the thousand exchanges
before it. A conversational design makes the transcript a single indivisible object.

## Preconditions and trigger

- The engine is composing a decision request at any decision opportunity after the first for that Mokiterion.

## Required response

1. The request states everything needed to choose an action: the shared rules, the Mokiterion's own constants, its
   current observation, and the enumeration `REQ-MOK-064` obliges.
2. The request carries no prior request, no prior response, no running summary, no turn counter used as memory and no
   identifier of a provider-side conversation.
3. The provider is asked for one action from that request alone.

## Failure and boundary behavior

- **The shared preamble is byte-identical across requests.** Permitted and intended. Identical content carries no
  history: it is the same text at tick 1 and at tick 1,000, so it cannot encode what happened in between.
- **The Mokiterion's own constants repeat every request.** Permitted. They are constants of the run, not memory of it.
- **The observation carries `suffered` from the previous tick.** Permitted. That is the engine's own bounded one-tick
  memory, specified independently of this initiative, and it arrives as observation rather than as retained context.
- **The tick number appears in the request.** Permitted. It is part of the observation.
- **A provider-side conversation or session identifier appears.** Forbidden. It is a handle to accumulated state even
  when the request text looks self-contained.
- **A retry after a transport failure re-sends the same request.** Permitted, and it is the same exchange rather than a
  second one; `REQ-MOK-069` records what was sent and `REQ-MOK-071` counts what it cost.

## Constraints

- This requirement fixes the first step and not the final answer. If bounded per-Mokiterion memory is later wanted, it
  is engine-owned, specified, bounded and emitted in the record stream, under its own intent — not acquired by
  loosening this requirement.
- Prompt caching is not memory and this requirement does not restrict it. A cache hit returns no content to the
  provider that the request did not contain; it only avoids re-charging for tokens the request sent.

## Acceptance examples

### Example: normal behavior

**Given** a retained transcript of a 20-tick run

**When** the request at tick 20 for Mokiterion 3 is compared with the request at tick 1 for the same Mokiterion

**Then** the two differ only in the observation block and the enumerated action set, and neither contains any part of
the other.

### Example: failure behavior

**Given** an implementation that maintains one provider conversation per Mokiterion and sends only the new observation

**When** the transcript is inspected

**Then** the check fails, because the request is not complete on its own and a conversation identifier appears in it.

## Open decisions

None. Whether to revisit fresh-per-decision isolation after the first measurement is a product decision reserved to
`INT-MOK-011`'s successor rather than to this requirement.
