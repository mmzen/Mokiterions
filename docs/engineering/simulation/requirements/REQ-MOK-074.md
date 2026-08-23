+++
id = "REQ-MOK-074"
type = "requirement"
title = "Count every fallback and disqualify the run from sourcing a published figure"
status = "approved"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN a decision cannot be obtained from the port or cannot be parsed into an action, THE SYSTEM SHALL count the occurrence, report the count in the run record, and mark the run as unfit to source a published figure whenever that count exceeds zero."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Count every fallback and disqualify the run from sourcing a published figure

## Rationale

A model-backed run has a failure mode no rule-based source has: the source can decline to answer. A request can time
out, the provider can return an error, and a response can arrive that is not one of the actions the request enumerated.
Something must happen next, and every available option is a way of quietly corrupting the measurement.

Failing the whole run on the first non-answer would make a 10,954-exchange run hostage to one transport hiccup.
Substituting a rule-based proposal — `baseline`'s, say — is worse in a subtler way: the run then reports what a mixture
of two sources did while its label claims one, and a figure drawn from it is not a measurement of the model at all. The
resolution used here is the one this repository already applies to unmeasured things: **do something defined, count it,
and let the count decide whether the run may be quoted.**

The threshold is zero rather than a small tolerance, because the whole purpose of `INT-MOK-011` is to see what a model
does without deciding in advance what it should do. A run with three substituted decisions in ten thousand would
almost certainly give the same survivor count as a clean one — and there would be no way to know, because the property
being measured is exactly the one the substitution interfered with. A zero threshold costs a re-run, which is
**estimated** at about $1.04. Publishing a contaminated figure costs the credibility of every figure beside it.

## Preconditions and trigger

- A live run in which one decision request returned no usable action: a transport failure after the run's retries, a
  provider error, a response that is not one of the enumerated actions, or a response that names a target the
  observation did not contain.
- Or a replay in which the transcript could not supply the decision — which `REQ-MOK-067` makes a hard failure rather
  than a fallback.

## Required response

1. Substitute the defined fallback action, which is the least consequential action available at that opportunity, and
   never a proposal composed by another decision source.
2. Record the occurrence with its opportunity and its cause in the transcript, as `REQ-MOK-069` requires.
3. Increment a run-level fallback count.
4. Report that count in the run record, and report it as zero when it is zero, so that a clean run says so positively.
5. When the count exceeds zero, mark the run in its record as unfit to source a published figure.

## Failure and boundary behavior

- **The count is zero.** The run is fit to source a figure, subject to `REQ-MOK-071`'s ceiling not having truncated it.
  The zero is reported rather than omitted.
- **A retry succeeds.** No fallback occurred and the count does not move. The attempts are still recorded and still
  billed, under `REQ-MOK-069` and `REQ-MOK-071`.
- **The response is a well-formed action that the enumeration did not permit.** A fallback. The action is not attempted
  and is not passed to validation as a proposal, because `REQ-MOK-063` sends the port's proposal through the same
  validation as any other source and an unenumerated action would be rejected there in a way that hides its origin.
- **The response names a target the observation did not carry.** A fallback, for the same reason.
- **The provider returns a valid action the engine's rules then reject** — a move into an occupied cell, say. **Not** a
  fallback. That is an ordinary rejected proposal, resolved exactly as it is for the four existing sources, and it is
  part of what the run is measuring.
- **A marked run is used anyway.** Outside what a requirement can prevent. The mark exists so that using it is a visible
  act; `VER-MOK-018` states that a published figure cites the run record it came from.

## Constraints

- The fallback action is fixed by `SPEC-MOK-007` and is the same at every opportunity, so that a run's contamination is
  one identifiable thing rather than a second policy.
- The count is a single integer in the run record. It is not a rate, not a percentage and not bucketed by cause; causes
  live in the transcript where they can be read individually.
- The mark is a property of the run record, not of a summary somebody writes afterwards.
- This requirement does not set an outcome floor and does not judge the run's survivors. It judges only whether the run
  measured the source it claims to have measured. `INT-MOK-011` records that no viability floor binds this source, and
  nothing here reintroduces one.

## Acceptance examples

### Example: normal behavior

**Given** a live 20-tick run in which every request returned an enumerated action

**When** the run record is read

**Then** the fallback count is reported as zero and the run is not marked unfit.

### Example: failure behavior

**Given** a live run in which one response arrives naming an action the request did not enumerate

**When** the run completes

**Then** the fallback action was substituted at that one opportunity, the transcript records the response and the cause,
the run record reports a fallback count of one, and the run is marked as unfit to source a published figure.

## Open decisions

None. Which action is the fallback is a specification decision fixed in `SPEC-MOK-007`.
