+++
id = "REQ-MOK-070"
type = "requirement"
title = "Hold the cached share of prompt tokens at or above 85 percent"
status = "draft"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN a live model-backed run completes, THE SYSTEM SHALL report cached prompt tokens as at least 85 percent of total prompt tokens over the run, computed from the provider's reported usage."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Hold the cached share of prompt tokens at or above 85 percent

## Rationale

The repository owner decided on 2026-08-23 that cache efficiency is *"an important requirement"* rather than an
intention. This requirement is what that decision looks like when it is written so that it can fail.

`gpt-5.6-luna` charges cached prompt tokens at $0.02/MTok against $0.20/MTok uncached — a tenth. Cache hits are
**prefix-based**: the provider matches the longest identical leading span of a request against a recent one. Two designs
that send exactly the same information therefore differ tenfold in price depending only on the order in which they
place it. That is not an implementation detail an author can be trusted to remember, and it is not visible in any test
that only checks the decision made. A stated numeric floor, computed from the provider's own figures, is.

The floor is 85 percent because it follows from the layout `SPEC-MOK-007` fixes. **Estimated** token counts per
request: about 1,200 for the shared rules, about 30 for the Mokiterion's own constants, about 200 variable. Cacheable
prefix 1,230 of 1,430 total, which is 86 percent. 85 percent leaves room for the first request of a run, whose prefix
is a cache write rather than a hit, and for a provider that trims a cached span at a boundary of its own choosing. It
leaves no room for a layout that puts the observation before the rules, which would report near zero.

The corresponding cost figures are **estimates** at the time of writing: about $1.04 for a 1,000-tick run under this
layout, about $1.36 with caching but no layout discipline, about $3.72 with no caching. The requirement is stated
against the ratio and not the dollar figure, because the ratio is a property of this repository's design while the
price is the provider's to change.

## Preconditions and trigger

- A live run has completed, whether it ended at its tick limit, through extinction, or at the ceiling `REQ-MOK-071`
  enforces.
- The provider reported usage for the exchanges, as `REQ-MOK-069` requires them to be recorded.

## Required response

1. Sum prompt tokens and cached prompt tokens over every exchange of the run, from the recorded usage.
2. Report both sums and the ratio in the run record.
3. Hold the ratio at 0.85 or above.

## Failure and boundary behavior

- **The ratio is below 0.85.** The check fails and reports the two sums. The run itself is not invalidated — it produced
  real decisions and its transcript replays — but it stands as a cost regression against this requirement, and the
  figures it carries are quoted with the ratio beside them.
- **A run is short enough that the first request's cache write dominates.** The floor is claimed over runs of at least
  200 exchanges. Below that the ratio is reported and not held, because a single uncached prefix is a large share of a
  small denominator. The bound is `VER-MOK-018`'s to fix in its case set.
- **The provider reports no cached-token figure at all.** The ratio cannot be computed, and that is a failure of this
  check rather than a pass by default. `REQ-MOK-069` records the absence, and the check reports that it could not be
  evaluated.
- **The provider changes its caching behaviour or its minimum cacheable prefix length.** The check fails against a
  layout that was correct when written. That is the intended behaviour: it is a signal to re-measure and to bring the
  layout or the floor back to the owner, not a reason to soften the number in place.
- **The run is a replay.** No prompt tokens are spent and no ratio exists. This requirement binds live runs only.

## Constraints

- The ratio is computed from the provider's reported usage and never from a local token estimate. A local estimate
  would let an implementation pass this check while paying full price.
- The layout that makes the ratio achievable is specified rather than left to the composing code: shared rules first,
  then the Mokiterion's own constants, then the variable observation and the enumerated action set last. `SPEC-MOK-007`
  fixes the order; this requirement fixes the outcome.
- The shared prefix must be byte-identical across Mokiterions and across ticks. `REQ-MOK-065` requires that already for
  isolation reasons, and the two requirements protect the same bytes for different reasons.
- Cache writes cost 1.25× uncached input. At one write per run they are negligible — an **estimated** $0.004 per run —
  and the requirement does not account for them separately.

## Acceptance examples

### Example: normal behavior

**Given** a live run of 1,200 exchanges under the layout `SPEC-MOK-007` fixes

**When** the recorded usage is summed

**Then** cached prompt tokens are at least 85 percent of total prompt tokens, and both sums and the ratio appear in the
run record.

### Example: failure behavior

**Given** an implementation that places the current observation ahead of the shared rules

**When** the same run is measured

**Then** the ratio is near zero because no two requests share a leading span, the check fails, and the run record
carries the two sums that show why.

## Open decisions

None. The exchange count below which the ratio is reported rather than held is fixed by `VER-MOK-018`.
