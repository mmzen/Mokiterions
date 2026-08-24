+++
id = "REQ-MOK-069"
type = "requirement"
title = "Record every exchange with the provider's reported token usage"
status = "approved"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN a decision is obtained from the model provider, THE SYSTEM SHALL record in the run's transcript the request sent, the response received, and the provider's reported prompt, cached-prompt, output and reasoning token counts for that exchange."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Record every exchange with the provider's reported token usage

## Rationale

The transcript is the load-bearing artifact of this initiative. Three other requirements are satisfiable only because
it exists: `REQ-MOK-067` replays from it, `REQ-MOK-065` is checked over it, and `REQ-MOK-070` computes a ratio from it.
If it recorded only the chosen action it would support none of them.

The token counts are the part most likely to be treated as optional, and they are the part that makes a cost claim
auditable. The repository owner made cache efficiency an obligation rather than an optimisation on 2026-08-23. An
obligation whose satisfaction rests on a figure somebody reported once, from a run nobody kept, is not an obligation
this repository can hold anyone to. Recording the provider's own `usage` values per exchange turns the cache ratio and
the spend figure into measurements a later reader recomputes, in the same way the retained record streams let a reader
recompute a survivor count rather than trusting a printed summary.

Reasoning tokens are recorded separately from output tokens because they are billed as output and are not visible in
the response text. A run at reasoning `none` should show zero of them, and recording the field is how that is
confirmed rather than assumed.

## Preconditions and trigger

- A live run: the model-backed source is selected, live mode was chosen explicitly and a credential was present, as
  `REQ-MOK-072` requires.
- One exchange has completed, successfully or not.

## Required response

For each exchange, in order, record:

1. The decision opportunity it belongs to — enough to bind the exchange to the tick and the Mokiterion, so that
   `REQ-MOK-067`'s replay can detect a mismatch.
2. The request as sent.
3. The response as received.
4. The provider's reported counts: prompt tokens, cached prompt tokens, output tokens, reasoning tokens.
5. The action the response was parsed into, or the fact that it could not be parsed.

## Failure and boundary behavior

- **The exchange failed.** It is recorded, with what was sent and what came back or what error occurred, and it counts
  under `REQ-MOK-074`. A failed exchange that leaves no trace would make the fallback count unauditable.
- **The provider reports no usage figures.** The absence is recorded as absence rather than as zero. A missing count and
  a count of zero mean different things, and `SPEC-MOK-006` rule 4 already fixes how this repository distinguishes
  them in a record stream.
- **A retry re-sends the same request.** Each attempt is recorded, because each attempt was billed.
- **The run is a replay.** No exchange occurs, so nothing is recorded. This requirement binds live runs.

## Constraints

- The transcript's format follows `SPEC-MOK-007`, which adopts `SPEC-MOK-006`'s existing constraints: one record per
  line, no floating-point values, no timestamps, a closed value alphabet, and bytes comparable between runs. A
  transcript is then diffable evidence rather than a log, and two runs can be compared with `cmp`.
- The transcript contains no credential, no authorization header and no provider account identifier. `REPOSITORY_CONTEXT.md`
  requires model-provider credentials to remain outside the repository, and a transcript is retained inside it.
- Size is a known consequence and is bounded by choice of horizon rather than by truncation: an **estimated** 4.7 MB
  for a 1,000-tick run and roughly 23 MB for five seeds. A transcript is never abbreviated to fit; what is retained
  where is `VER-MOK-018`'s to state.

## Acceptance examples

### Example: normal behavior

**Given** a live 20-tick run at reasoning `none`

**When** the transcript is read

**Then** it holds one record per decision opportunity, each binding tick and Mokiterion, each carrying the request, the
response, four token counts and the parsed action, and every reasoning-token count is zero.

### Example: failure behavior

**Given** a live run in which one exchange times out and is retried once, successfully

**When** the transcript is read

**Then** both attempts appear, the first marked as failed with what was sent, the second with its own usage figures,
and the fallback accounting under `REQ-MOK-074` can be reconstructed from the transcript alone.

## Open decisions

None.
