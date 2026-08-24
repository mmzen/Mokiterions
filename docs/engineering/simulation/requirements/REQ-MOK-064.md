+++
id = "REQ-MOK-064"
type = "requirement"
title = "Present the set rule 6 permits, targeted actions included"
status = "approved"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN the engine issues a decision request for one decision opportunity, THE SYSTEM SHALL enumerate in that request every action the specification permits that Mokiterion to propose at that opportunity, including every targeted action, and SHALL NOT present the baseline candidate list as that set."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Present the set rule 6 permits, targeted actions included

## Rationale

There are two different sets of actions in this system and they are easy to confuse, because one of them has a name
that sounds like it is the answer.

`Observation.valid_actions` is the candidate list rule 4 offers `baseline`. **It contains no targeted action, ever,**
and its own documentation states why: rule 4's baseline consumes one entropy selection over that list's length, so
adding a targeted action to it would move that selection and diverge every run ever recorded under `baseline`. The
field's documentation goes on to say, in terms, that the list is therefore no longer everything a source may
legitimately propose — rule 6 is — *"and a reader who takes it as the whole contract will be wrong about the social
source."*

A request built from that field would offer `wait`, `sleep`, `eat` and `move` and nothing else. The model would never
propose `attack`, `threaten`, `fight`, `retreat`, `surrender`, `approach` or `avoid`, and the run would produce a
verbose restatement of `reference` while being published as a model-backed result. This is the most likely way to
build the model-backed source and reach a confidently wrong conclusion, and it would not fail any check that does not
exist for it. That is why this obligation is a requirement of its own rather than a sentence inside `REQ-MOK-063`.

## Preconditions and trigger

- The engine is composing a decision request for one Mokiterion at one decision opportunity.
- Zero or more other Mokiterions are within that Mokiterion's perception radius, and zero or more are within contact
  radius.

## Required response

1. Determine the actions the specification permits that Mokiterion to propose at that opportunity — the rule 6 set,
   evaluated against the state standing at the start of the opportunity.
2. Enumerate that set in the request. Where a verb takes a target, the request states the verb and the admissible
   targets, and does so in a form that lets the response name exactly one of each.
3. Leave `baseline`'s candidate list out of the request entirely, so that no implementation can drift into using it as
   the menu.

## Failure and boundary behavior

- **No other Mokiterion is perceived.** The set contains no targeted verb, correctly, and the request says so. This is
  the common case and must not be confused with the trap: the absence is then a fact about the world rather than a
  fact about which field was read.
- **A perceived Mokiterion is outside contact radius.** Verbs the specification permits only at contact are absent for
  that target and present for none. The request reflects the opportunity, not the verb list.
- **The response names an action the request did not enumerate.** The engine validates and rejects it under
  `REQ-MOK-063`. This requirement obliges what the request offers, not what the response may contain; a model that
  proposes outside the offered set is a measurement of the model and is reported as a rejection.

## Constraints

- Enumerating the set must not perturb the entropy stream. Whatever the request computes, it computes without drawing
  from the stream that decides `baseline`'s selection, which is what `REQ-MOK-068` holds the system to.
- The enumeration is derived from the same state the observation carries. It introduces no read this system does not
  already make, and in particular no population-level aggregate, which `REQ-MOK-059` forbids.
- The specification may state the action grammar once in the request's cacheable prefix and name only the admissible
  targets per opportunity, provided every action the rule 6 set contains remains determinable by the reader of one
  request. `SPEC-MOK-007` fixes which form is used; `REQ-MOK-070`'s cost obligation and this requirement's
  completeness obligation both bind that choice.

## Acceptance examples

### Example: normal behavior

**Given** a Mokiterion at contact radius with one other Mokiterion, and two co-located resources

**When** the engine composes that opportunity's request

**Then** the request enumerates the core verbs, both `eat` options, the admissible `move` directions, and every
targeted verb rule 6 permits against that one neighbour — and a reader of the request can reconstruct the rule 6 set
from it exactly.

### Example: failure behavior

**Given** the same opportunity

**When** a candidate implementation composes the request from `Observation.valid_actions`

**Then** the check fails, because no targeted verb appears in a request whose opportunity admits seven of them.

## Open decisions

None at the product level. Whether the grammar is stated once in the cacheable prefix or enumerated per request is a
specification choice constrained above, measured under `WO-MOK-026`, and recorded in `SPEC-MOK-007`.
