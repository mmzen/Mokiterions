+++
id = "REQ-MOK-065"
type = "requirement"
title = "Carry no other Mokiterion's state into a decision request"
status = "draft"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN the engine issues a decision request for one Mokiterion, THE SYSTEM SHALL include no attribute of any other Mokiterion, no population-level aggregate, and no value derived from another Mokiterion's request or response."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Carry no other Mokiterion's state into a decision request

## Rationale

`INT-MOK-011` requires each Mokiterion to be treated as an independent agent in an isolated context. Isolation has a
weak reading and a strong one, and only the strong one is worth having: it is not enough that the provider keeps
twelve separate conversations, because a single request that quotes a neighbour's `health` has already leaked whatever
the separation was protecting.

The engine is most of the way there already, and this requirement exists to keep it there rather than to build it.
`PerceivedMokiterion` carries an identifier, a direction and a distance, and its documentation states that it carries
*"no attribute of the perceived Mokiterion — not its `health`, its `energy` or its `fear`."* `REQ-MOK-059` already
forbids reading any population-level aggregate, and every read in every rule and every source was enumerated against
it. So the prompt contract is already governed by construction: what the observation carries may go into the request,
and adding a field to the observation is a specification amendment with an accountable owner.

What is new is the second clause. A request is composed by code that, unlike a rule-based source, sits next to eleven
other requests and their responses. Nothing in the existing artifacts forbids an implementation from summarising the
last tick's responses into this tick's request. That would be a cross-agent channel this system deliberately does not
have, and it would be invisible in the record stream.

## Preconditions and trigger

- The engine is composing a decision request for one Mokiterion at one decision opportunity.
- Other Mokiterions exist, alive or dead, and eleven other requests and responses exist for the current tick or
  earlier ticks.

## Required response

The request contains only:

1. Content that is byte-identical for every Mokiterion at every opportunity — the shared preamble, which by being
   identical carries no Mokiterion's information.
2. That Mokiterion's own identity and its own constants.
3. That Mokiterion's own observation, as the specification defines it, including the perception entries exactly as
   the observation carries them.
4. The enumeration `REQ-MOK-064` obliges.

Nothing else. In particular, no other Mokiterion's health, satiety, energy, fear, waste tolerance, territory or
proposal; no count, mean, maximum or ranking over the population; and no text, identifier or value taken from another
request or response.

## Failure and boundary behavior

- **A perceived neighbour's identifier appears in the request.** Permitted, and required: the identifier is what the
  observation carries and is what a targeted verb needs in order to name a target. The identifier is not an attribute.
- **An attack the Mokiterion suffered last tick names its attacker.** Permitted. The `suffered` record is this
  Mokiterion's own one tick of memory and the attacker's identifier is part of what happened to it. Nothing about the
  attacker's condition may accompany it.
- **A request quotes a neighbour's fear in order to explain a threat.** Forbidden, and a check failure.
- **A request summarises what other Mokiterions proposed this tick.** Forbidden, and a check failure, even when the
  summary names nobody.
- **The shared preamble is identical across all twelve.** Permitted, and it is what `REQ-MOK-070`'s cost obligation
  depends on. Any per-Mokiterion variation inside the prefix is a violation of that identity and is caught as a cache
  regression as well as an isolation one.

## Constraints

- The check is made over retained transcripts rather than over source code alone, because the property is about what
  was sent. A check that reads only the composing function cannot see a value that arrived through a shared buffer.
- Dead Mokiterions are covered by the same prohibition. A request may not mention that another Mokiterion died.
- This requirement does not weaken `REQ-MOK-059`, and `REQ-MOK-059` is not amended by it. The two hold different
  surfaces: `REQ-MOK-059` holds the engine's reads, this one holds the request's content.

## Acceptance examples

### Example: normal behavior

**Given** a retained transcript of a 20-tick run with twelve living Mokiterions

**When** every request in it is inspected

**Then** each request's variable content mentions exactly one Mokiterion's attributes — its own — and each perceived
entry carries only an identifier, a direction and a distance.

### Example: failure behavior

**Given** an implementation that appends the previous tick's twelve responses to each request as context

**When** the transcript is inspected

**Then** the check fails on the first request of tick 2, naming the derived content it found.

## Open decisions

None.
