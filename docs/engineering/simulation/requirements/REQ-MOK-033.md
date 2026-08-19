+++
id = "REQ-MOK-033"
type = "requirement"
title = "Provide a trait-aware decision source without changing the existing sources"
status = "draft"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "WHEN the operator selects the trait-aware decision source, THE SYSTEM SHALL propose actions that depend on the acting Mokiterion's behavioral trait, so that two Mokiterions holding different trait values may propose different actions from otherwise identical observations."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-006"]
+++

# Requirement: Provide a trait-aware decision source without changing the existing sources

## Rationale

A trait that no source reads is a number in a record. `REQ-MOK-031` derives individuality; this requirement is what
makes it observable as behavior, and without it the trait is exactly the inert value `SPEC-MOK-003` rule 4.5
prohibits.

It must be a third source rather than a change to the reference source, for two reasons. The reference source is the
control: `REQ-MOK-014`'s survivor floor, `VER-MOK-002`'s measurements, the oscillation rate of 10.6% against a
random-walk 12.2%, and the accumulation figure of 45 of 61 are all measured on it, and every later claim that a
smarter source is smarter is a comparison against it. Changing it retires that baseline. And `SPEC-MOK-001` rule 5
records two defective earlier forms of its own rules, one of them found only by measurement after the specification
had asserted the opposite; a source with that history is not one to edit in place while adding a feature.

The trait's chosen consumer is the non-waste test, because that test is where uniformity does measurable damage.
Rule 5 applies it to eating and to seeking, so a high-class resource is both eatable and approachable only at satiety
of at most `50`, and every Mokiterion crosses that line at the same moment. The result is recorded in the
specification itself: high class reaches 45 of 61 standing resources by tick 1,000. A per-Mokiterion tolerance for
clipped restoration turns one shared threshold into twelve, so the population stops declining the same food
simultaneously. The trait therefore acts on the one rule where individuality has a mechanism, not merely a label.

The design has a property worth stating as a reason rather than a detail: at a tolerance of zero the tolerant test is
arithmetically the reference test, so the new source is action-for-action identical to the reference source for a
Mokiterion at the range's lower bound. That makes the new source pinnable against the control by construction, and it
means a divergence anywhere else is attributable to the trait and to nothing else.

## Preconditions and trigger

The trigger is a living Mokiterion's decision opportunity in a tick while the run's selected decision source is the
trait-aware one. The source is selected through the existing decision-source option, as a third accepted value
alongside the two that exist. It is not the default.

The source receives the same read-only observation as the existing sources, plus the acting Mokiterion's trait value.
It gains no other input, and in particular no ability to read or mutate authoritative state.

## Required response

- The decision-source option accepts a third value naming the trait-aware source, and the usage text states its
  effect and the unchanged default, as `REQ-MOK-018` requires of every option.
- The selected source is reported once before agent processing begins, as `SPEC-MOK-001` rule 1 already requires, so
  a run under the new source is no more ambiguous than a run under either existing one.
- The source proposes actions using the acting Mokiterion's trait, such that two Mokiterions with different trait
  values, presented with otherwise identical observations, may propose different actions. The dependence is on the
  trait value, not on the identifier: two Mokiterions holding equal traits in identical situations propose identical
  actions.
- A Mokiterion whose trait is at the range's lower bound proposes, on every decision opportunity and for every
  observation, exactly the action the reference source proposes for that observation.
- The source returns exactly one proposal per decision opportunity, drawn from the actions the observation lists as
  currently valid, and the engine validates it under `SPEC-MOK-001` rule 6 with no relaxation. The source is advisory
  in exactly the sense `ADR-MOK-001` fixes; being trait-aware grants it no authority.
- The source is deterministic. At one seed, one density and one tick limit it produces the same run, byte for byte,
  on every execution, as `REQ-MOK-009` requires of the engine as a whole.

## Failure and boundary behavior

- An unrecognized decision-source value remains invalid configuration and exits with code `2`, writing the usage text
  to standard error. Adding a third accepted value changes which values are accepted and nothing about how a
  rejection behaves.
- The new source proposes an action for every decision opportunity and has no failure path of its own. It cannot
  return nothing, so it cannot forfeit an opportunity; a proposal the engine rejects consumes the opportunity under
  rule 6 exactly as it does for the existing sources.
- Where no perceived resource satisfies the acting Mokiterion's tolerance, the source falls through to the same
  search step the reference source uses, consuming one entropy selection. It never proposes `wait`, for the reason
  rule 5 gives: a blind Mokiterion searches rather than stands still.
- Because tolerance decides how often the search fallback is reached, the new source consumes the shared entropy
  stream at a rate of its own. Runs under it are comparable only with runs under it, at the same seed and density —
  the restriction `SPEC-MOK-001` already states for the two existing sources, now stated for three.
- At the trait range's upper bound the source is permitted to accept any restoration the resource offers, including
  restoration entirely clipped by the attribute maximum. That is a legitimate individual, not a defect: `REQ-MOK-015`
  requires consuming "when consuming it is not wasteful", and this requirement re-parameterizes what counts as
  wasteful per Mokiterion rather than abolishing the judgement.

## Constraints

- **The existing decision sources do not change.** Under `--policy baseline` and under `--policy reference`, at the
  same seed, density and tick limit, the survivor count, the death count, every surviving Mokiterion's final
  coordinate, and the standing resource count and class distribution of each territory are identical to those the
  build before this change produces. This is the load-bearing constraint of the whole initiative and it is
  demonstrated by comparison against recorded output, not asserted.
- That equivalence is about outcomes, not bytes. `REQ-MOK-032` adds `fear` to two existing event lines and
  `REQ-MOK-031` adds the trait to a third, so the output stream is not byte-identical to the previous build. No claim
  in this initiative depends on byte equality across builds; `REQ-MOK-009`'s byte equality is between runs of one
  build, and it is untouched.
- The trait-aware test is applied to *both* eating and seeking, as rule 5 applies its non-waste test to both.
  Applying it to eating alone would reintroduce the two-cell oscillation rule 5 records as its second defect: a
  Mokiterion standing on a resource its tolerance declines would step off, immediately perceive that resource as the
  nearest one at a distance greater than zero, and step back. Keeping one test in both positions is what keeps the
  resource just left excluded from targeting for exactly as long as it would be declined.
- Integer arithmetic only, with no floating-point value anywhere in the tolerance comparison, so the result is
  identical on every target.
- No new package, no new external dependency, and no growth of the engine's empty dependency table.
- The public interface grows by exactly one option value. The trait-aware source is not a public type, and neither is
  the observation it consumes; `SPEC-MOK-002` rule 6 keeps both private, because they carry the `ADR-MOK-001` trust
  boundary.
- The tolerance comparison, the trait's range and the source's name in the option and in the record are specification
  matters. This requirement fixes only that the source exists, is selectable, is not the default, reads the trait,
  collapses to the reference source at the trait's lower bound, and leaves the existing sources' outcomes untouched.

## Acceptance examples

### Example: normal behavior

**Given** two living Mokiterions with different trait values, each standing on a high-class resource, each at a
satiety at which the reference source would decline it

**When** each is given a decision opportunity under the trait-aware source

**Then** the Mokiterion whose tolerance admits the clipped restoration proposes `eat`, the Mokiterion whose tolerance
does not proposes something else, and the two proposals differ solely because the trait values differ.

### Example: equivalence to the control at the lower bound

**Given** a Mokiterion whose trait is at the range's lower bound

**When** it is given a decision opportunity under the trait-aware source for an observation

**Then** the proposal is identical to the one the reference source returns for that same observation.

### Example: the control is unchanged

**Given** the seeds `0`, `1`, `42`, `123` and `777` at the default density and one thousand ticks

**When** runs complete under `--policy reference` and under `--policy baseline`

**Then** the survivor count, the death count, every surviving Mokiterion's final coordinate, and each territory's
standing resource count and class distribution match the recorded output of the build before this change, on every
seed and under both sources.

### Example: failure behavior

**Given** the argument `--policy individualist`

**When** the command is executed

**Then** the process writes a configuration error and the usage text to standard error, exits with code `2`, and runs
no ticks.

## Open decisions

None required for approval. Two matters are deliberately left to later governed changes rather than left open here:
whether the trait-aware source should become the default, which the product owner deferred on 2026-08-19 as a
decision to take after its floor under `REQ-MOK-034` has been measured, leaving `reference` the default in the
meantime; and whether the reference source's own non-waste test should be revised to address
accumulation, which would move the curve `REQ-MOK-014` is measured on and is out of scope under `INT-MOK-006`.
