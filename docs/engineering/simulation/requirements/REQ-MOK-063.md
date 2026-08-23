+++
id = "REQ-MOK-063"
type = "requirement"
title = "Obtain each decision from the decision port and validate it like any other proposal"
status = "approved"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN a simulation runs under the model-backed decision source, THE SYSTEM SHALL obtain each living Mokiterion's proposed action for each decision opportunity from the decision port, and SHALL validate and resolve that proposal by the same rules it applies to every other decision source's proposal."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Obtain each decision from the decision port and validate it like any other proposal

## Rationale

A fifth decision source is what `INT-MOK-011` exists for, and the whole value of it depends on one property: the
engine must treat a model's proposal exactly as it treats a rule's. If the model's proposals were resolved by a
different code path, or trusted where the others are checked, then the run would measure the new path rather than the
model, and `ADR-MOK-001`'s statement that every decision source is untrusted would have acquired an exception nobody
decided.

This obligation is not new and is not this initiative's invention. `ADR-MOK-001` recorded it in 2026-08-11, before any
model existed to apply it to: *"Model output is untrusted input and must pass the same validation as the local
baseline."* This requirement is that sentence written as an obligation with a verification method, so that it binds a
build rather than describing an intention.

The second half of the obligation is the one that is easy to lose. `advance_tick` today selects a source by a
hardcoded match over four policies, and the source itself is a private trait whose `decide` returns an `Action` and
cannot fail. A network call can fail. Growing that boundary is the substance of this requirement, and the growth must
not become a second resolution path: what changes is where the proposal comes from, not what happens to it afterwards.

## Preconditions and trigger

- The run's configuration names the model-backed decision source.
- The engine has reached a decision opportunity: a living Mokiterion's turn within a tick, in the order the
  specification fixes.
- A decision port is available to the engine, supplied by the host as an open pair of streams. The engine resolves no
  path and opens no connection of its own, which `SPEC-MOK-006` rule 1.2 already requires of the library target.

## Required response

1. Issue one decision request describing that Mokiterion's observation, and read one response naming one action.
2. Treat the named action as a **proposal**, identical in standing to the proposal any other source returns.
3. Validate the proposal against the rules in force for that opportunity, and resolve it or reject it by those rules
   and no others.
4. Emit the same records for the outcome as any other source's proposal would produce, including the action trace
   when tracing is enabled and the rejection when the proposal is illegal.
5. Name the model-backed source in the record stream's `decision_source_selected` record, so a reader of a retained
   stream can tell which source produced the run.

## Failure and boundary behavior

- **The port yields no usable response.** That is a fallback, and `REQ-MOK-074` governs it: counted, reported, and
  disqualifying for a run that would source a published figure. It is never resolved silently by substituting a
  rule-based source's proposal without being counted.
- **The response names an action that is illegal at that opportunity.** This is ordinary rejection, not failure. The
  engine already rejects illegal proposals from `baseline`, and the rejection counter already exists. No new mechanism
  is created for it.
- **The response names an action outside the action vocabulary entirely.** It cannot be parsed, so it is a fallback
  under `REQ-MOK-074` rather than a rejection.
- **No living Mokiterion remains.** The run ends as it ends under any other source. No request is issued for a dead
  Mokiterion.

## Constraints

- The engine acquires no network capability, no credential handling, no asynchronous runtime and no model-provider
  crate. `ARCH-MOK-001`'s prohibited-pattern entry for the engine package and its by-name conformance scan both stand
  unamended, and the engine package's declared dependency set stays empty.
- `Observation` and the decision-source trait stay private types. What crosses is a serialized projection by value,
  on the precedent `SPEC-MOK-003` set when it narrowed `SPEC-MOK-002` rule 6 from a list of type names to the
  capability that rule denies.
- Decisions within one tick stay sequential. Each request describes the state standing at the start of that
  opportunity, after every earlier-acting Mokiterion in the same tick has acted.
- The four existing sources are untouched, which `REQ-MOK-068` states as its own obligation.

## Acceptance examples

### Example: normal behavior

**Given** a run configured for the model-backed source at seed 0 and the default density

**When** the engine reaches the third living Mokiterion's opportunity in tick 7

**Then** one request is issued for that Mokiterion, one response is read, the named action is validated against the
rules in force at that opportunity, and the resolution and its records are indistinguishable in form from those the
same action would have produced arriving from `social`.

### Example: failure behavior

**Given** the same run

**When** the response names `attack` against a Mokiterion outside contact radius

**Then** the proposal is **rejected** by the engine's existing validation, the rejection is recorded as any other
source's rejection would be, the fallback count does not increase, and the run continues.

## Open decisions

None at the product level. Where the provider that answers the port lives is a technical decision recorded in
`ADR-MOK-007`, and this requirement is satisfied identically under either option it considers.
