+++
id = "REQ-MOK-075"
type = "requirement"
title = "Publish the model-backed outcome beside the existing sources at the same seeds and horizon"
status = "approved"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN a model-backed measurement completes over a declared seed set at a declared horizon, THE SYSTEM SHALL report, for the model-backed source and for the reference and social sources at the same seeds and horizon, the survivors, the deaths and the deaths attributable to combat."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Publish the model-backed outcome beside the existing sources at the same seeds and horizon

## Rationale

`INT-MOK-011` sets no viability floor for this source. The repository owner decided on 2026-08-23 that *"defining a
floor is probably not possible as the whole point of this is to empirically see what is going to happen"*. That decision
removes an obligation on the outcome; it does not remove the obligation to **report** the outcome. Without a stated
reporting duty, a source held to no floor is a source that could produce a figure nobody ever writes down, and the
initiative would have measured nothing.

The comparison is the substance. A survivor count on its own is uninterpretable: 4 of 12 means nothing without knowing
that `reference` gives 7 and `social` gives 9 at the same seeds. Reporting all three side by side at the same seeds and
the same horizon is what turns a number into a result, and it is the form this repository already uses — `REQ-MOK-060`
reports composition figures across sources rather than for one.

Three quantities rather than one, because they distinguish outcomes that a survivor count conflates. Survivors, total
deaths and combat deaths together separate a population that starved from one that fought, and combat deaths are the
quantity most likely to differ under a source that can propose targeted actions freely.

This requirement deliberately states **no expected direction**. It does not say the model does better, worse or the
same. Any of the three is a valid result and none of them fails this requirement. What fails it is not measuring.

## Preconditions and trigger

- A model-backed measurement has completed over a declared seed set at a declared horizon, authorised under
  `REQ-MOK-076`, with a fallback count of zero under `REQ-MOK-074`.
- `reference` and `social` can be run at the same seeds and horizon, offline and free.

## Required response

For the declared seed set at the declared horizon, report:

1. For the model-backed source: survivors, deaths, and deaths attributable to combat, per seed and over the set.
2. For `reference`: the same three quantities at the same seeds and horizon.
3. For `social`: the same three quantities at the same seeds and horizon.
4. The seed set, the horizon and the density the figures were produced at, and the run records they came from.

## Failure and boundary behavior

- **The model-backed figures are worse than both existing sources.** Not a failure of this requirement, and not a
  failure of anything. It is a result, and `INT-MOK-011`'s non-goals record that no better-or-worse claim is being made.
- **The population goes extinct at every seed.** Reported as a result. `baseline` is the precedent: it goes extinct
  between ticks 119 and 193 on every declared seed and that is recorded as measurement, never as failure.
- **A run in the set has a non-zero fallback count.** Its figures are not published. `REQ-MOK-074` marks it unfit, and
  the seed is re-run or the set is reported as incomplete with the gap named.
- **A run stopped at its spend ceiling.** Its horizon is not the declared one. It is not published as a figure at the
  declared horizon; the truncation is reported under `REQ-MOK-071`.
- **The horizon is shorter than the existing sources' published horizons.** Permitted, and expected for the first
  measurement: cost and latency bound what can be run — an **estimated** 1.2 to 2.4 hours per 1,000-tick run. The
  comparison is valid because `reference` and `social` are re-run at the *same* horizon rather than quoted from their
  own longer runs.
- **`individual` and `baseline` are not included.** Permitted. They may be reported and this requirement does not
  require them. `reference` and `social` are named because they bracket the existing range.

## Constraints

- The comparison sources are re-run at the declared seeds and horizon rather than quoted from earlier evidence. A quoted
  figure from a different horizon is not a comparison, and re-running them costs nothing.
- The seed set and the horizon are declared before the measurement, not chosen after the figures are known.
- Every figure cites the run record and the transcript it came from, so a reader can replay it under `REQ-MOK-067`.
- This requirement sets no threshold on any of the nine numbers it obliges. It is a reporting obligation, and
  `VER-MOK-018` treats it as one.

## Acceptance examples

### Example: normal behavior

**Given** an authorised model-backed measurement over five declared seeds at a declared horizon, every run with a zero
fallback count

**When** the measurement is reported

**Then** survivors, deaths and combat deaths appear for the model-backed source and for `reference` and `social` at the
same five seeds and the same horizon, with the seed set, horizon, density and source run records named.

### Example: failure behavior

**Given** the same measurement, reported against `reference`'s and `social`'s previously published 1,000-tick figures
rather than fresh runs at the declared horizon

**When** the report is checked

**Then** it fails this requirement, because the three sources were not measured at the same horizon and the comparison
does not hold.

## Open decisions

None. The seed set and the horizon are fixed per measurement in the authorization `REQ-MOK-076` retains.
