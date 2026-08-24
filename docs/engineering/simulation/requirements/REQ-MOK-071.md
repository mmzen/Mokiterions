+++
id = "REQ-MOK-071"
type = "requirement"
title = "Stop rather than exceed the spend ceiling declared for the run"
status = "approved"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN the cost accumulated by a live model-backed run reaches the spend ceiling declared for that run, THE SYSTEM SHALL make no further provider call, SHALL end the run, and SHALL report the ceiling and the accumulated cost in the run record."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Stop rather than exceed the spend ceiling declared for the run

## Rationale

Every other decision source in this repository costs nothing to run, so nothing in the existing artifacts bounds how
much a run may consume. The model-backed source makes a run cost real money, and the ways a run can cost more than
intended are not exotic: a mistyped tick limit, a retry loop, a longer prompt than the layout intended, a provider
price change, or a horizon extended without re-estimating. An **estimated** $1.04 per 1,000-tick run is small; the same
mistake at a horizon typed with one extra zero is not.

A ceiling declared per run and enforced in the run is the containment that does not depend on anybody remembering. It
also makes the authorization `REQ-MOK-076` retains meaningful: the owner authorises a horizon, a seed set **and a
ceiling**, and the ceiling is a number the run itself honours rather than a note in a document.

Stopping is preferred over continuing-with-a-warning because the alternative is a run that keeps spending after it has
been told not to. A run that stops early is still evidence — it has a transcript, a tick count and a record stream, and
`REQ-MOK-074` and the run record make its truncation visible to anyone reading its figures.

## Preconditions and trigger

- A live run with a declared ceiling.
- The cost accumulated from the recorded usage has reached or would exceed that ceiling.

## Required response

1. Accumulate cost after each exchange from the provider's reported usage and the declared unit prices.
2. Before issuing an exchange, stop if the accumulated cost has reached the ceiling.
3. End the run in an orderly way: the record stream and the transcript are complete and readable to the point reached.
4. Report in the run record the declared ceiling, the accumulated cost, the tick reached, and that the run ended at the
   ceiling.
5. Exit with a distinct, documented status so a caller can tell a ceiling stop from a clean completion and from an
   error.

## Failure and boundary behavior

- **No ceiling was declared.** No live run happens. `REQ-MOK-076` requires an authorization naming a ceiling, and a
  live run without one is refused before the first exchange rather than run unbounded.
- **A single exchange would cross the ceiling.** The exchange is not issued. The check is made before spending, not
  after, so the ceiling is a bound and not a target the run overshoots by one call.
- **The run reaches its tick limit before the ceiling.** Normal completion. The accumulated cost and the ceiling are
  still reported, because a run that came close is worth seeing.
- **The provider's prices are not the ones the run was configured with.** The accumulated figure is an estimate from the
  configured prices, and the run record says so. This requirement bounds what the run believes it is spending; the
  provider's invoice is outside this system.
- **The run is a replay.** No cost accrues and no ceiling applies. This requirement binds live runs only.

## Constraints

- The accumulated figure is computed from the provider's reported token counts, the same ones `REQ-MOK-069` records, so
  that a reader can recompute it from the transcript.
- Cost accumulation is arithmetic over integer token counts and declared integer unit prices. `SPEC-MOK-006`'s
  prohibition on floating-point values in the record stream continues to hold, so the reported figure is an integer in
  a stated unit rather than a formatted decimal whose bytes vary by platform.
- The ceiling is declared per run by the caller and is not a compiled-in default. A default would be a value nobody
  authorised.
- A truncated run is never silently substituted for a completed one. Any figure published from a run that stopped at
  its ceiling carries that fact, and `VER-MOK-018` states how.

## Acceptance examples

### Example: normal behavior

**Given** a live run declared with a ceiling well above its expected cost

**When** it completes at its tick limit

**Then** the run record reports the ceiling, the accumulated cost below it, and a clean completion status.

### Example: failure behavior

**Given** a live run declared with a ceiling reached at roughly half its tick limit

**When** it is run

**Then** no exchange is issued after the ceiling is reached, the transcript and record stream are complete to that
tick, the run record names the ceiling, the accumulated cost and the tick reached, and the exit status distinguishes
the ceiling stop from a clean run.

## Open decisions

None.
