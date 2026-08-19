+++
id = "REQ-MOK-040"
type = "requirement"
title = "Give each Mokiterion a stable name and report it"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "WHEN a simulation is initialized, THE SYSTEM SHALL give each Mokiterion one name from a fixed set of twelve, distinct from every other Mokiterion's name and stable across every run, and SHALL report it exactly once for that Mokiterion in the text observation record, without consuming any value from the shared entropy stream and without changing any other reported value."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-008"]
+++

# Requirement: Give each Mokiterion a stable name and report it

## Rationale

A name has to come from the engine. `SPEC-MOK-003` rule 10 item 7 lists `name` among the values the engine does not
compute and therefore forbids the observer from presenting one, and `WO-MOK-005` bound the observer to present no value
the engine does not compute, "including an inert placeholder". So the observer cannot be given a name table; the engine
has to hold one and report it. That is not an implementation preference reached by elimination, it is what two approved
artifacts already require.

The name is stable across runs rather than derived from the seed. `SPEC-MOK-003` rule 2.5 requires every distinction
carrying identity to be available without colour and names Mokiterions by glyph, and rule 2 commits the glyph to
becoming the name's first character; so twelve distinct first characters is an obligation. A fixed assignment satisfies
it by construction. `INT-MOK-008` records why a seed-derived permutation was declined and what it would have to solve
first.

The last two clauses of the statement carry this requirement's weight, and they are the reason it is worth verifying at
all rather than just implementing. Consuming nothing from the shared stream keeps every measured floor — `REQ-MOK-014`'s
and `REQ-MOK-034`'s — exactly where it was measured. Changing no other reported value is what makes this a legibility
change rather than a behavioral one, and it is checkable against a recorded capture instead of argued.

## Preconditions and trigger

The trigger is initialization of a simulation, before the first tick, under any decision source, any density and any
tick limit.

Every Mokiterion the run creates is named, regardless of configuration. The name is a property of the Mokiterion and
not of the run: two runs at different seeds hold the same twelve names in the same assignment, which is what makes a
name usable as a label across runs and across evidence files.

## Required response

The system assigns each Mokiterion exactly one name, and:

- The twelve names are pairwise distinct, and their first characters are pairwise distinct. The first-character property
  is a requirement rather than a consequence, because `REQ-MOK-041` derives a single-character glyph from it and two
  Mokiterions sharing a glyph would lose an identity distinction the observer's colour-independence rule obliges.
- Each name consists only of characters in `A`–`Z` and `a`–`z`, and is between one and five characters long. The
  character restriction is a requirement because the text record's details are comma-and-colon delimited and a name is
  written into them unescaped; the length restriction is a requirement because the observer renders the name in a
  fixed-width column whose budget `SPEC-MOK-003` rule 4 fixes.
- The assignment from identifier to name is total, injective and fixed: `M01` receives one name in every run of every
  build, `M02` another, and so on through `M12`. The name never changes after initialization, and nothing in any tick
  alters it.
- The set of names holds exactly as many entries as the run creates Mokiterions. A name table longer or shorter than the
  population is a defect, not a spare.
- The name is reported exactly once for each Mokiterion, in the `agent_initialized` record `REQ-MOK-010` requires, so an
  operator reading a run's output learns the names from the run. It is reported nowhere else, because it cannot change.
- Naming performs no draw against the shared entropy stream. After initialization the shared stream stands at exactly
  the position it stood at in a build without names, having produced exactly the same values in the same order.
- No other reported value changes. For any seed, density, decision source, tick limit and trace setting, the run's
  standard output is identical to the pre-change build's once the added field is removed from the `agent_initialized`
  records, and the exit code is identical.

## Failure and boundary behavior

- There is no failure path. Every Mokiterion the run creates has a name, so a name cannot be absent, cannot be empty and
  cannot be out of range. There is no configuration error, no runtime error and no exit-code consequence.
- No name is a reserved or ambiguous value. In particular no name equals an identifier of any entity the output names,
  so a name can never be read as an identifier.
- Nothing reads a name. No rule, no decision source, no validation path, no ordering, no tie-break and no termination
  condition consults it. Acting order remains ascending identifier order under `SPEC-MOK-001` rule 2; ordering by name
  would reorder acting order and change every outcome, which this requirement forbids by forbidding any reader.
- A name is not an attribute. It is absent from the roster's attribute bars and from the per-tick survival record,
  because it does not vary and reporting it per tick would imply that it can.

## Constraints

- The name adds no item to the engine's public interface. `SPEC-MOK-002` rule 5 holds the interface to what approved
  requirements need, and `REQ-MOK-041` needs the name in the observer, not on a new public item: the observer already
  retains the event stream the name is reported in, which is the route `SPEC-MOK-002`'s 2026-08-19 amendment established
  for `waste_tolerance`.
- The engine's dependency table stays empty. Twelve string literals need nothing.
- The name's position within the `agent_initialized` record's details is fixed by `SPEC-MOK-001` and is chosen so that
  no existing consumer of that record changes. A change to an existing test's assertions is not an acceptable cost of
  this requirement.
- No allocation, formatting or comparison involving a name may affect entropy consumption, iteration order of anything
  observably ordered, or the content of any other record.
- The names themselves are fixed by `SPEC-MOK-001` and are the product owner's decision. Their spelling, their
  assignment to identifiers, and the judgement that each is inoffensive are not the implementation's to take.

## Acceptance examples

### Example: normal behavior

**Given** any accepted seed

**When** the simulation is initialized

**Then** the output holds twelve `agent_initialized` records, each reporting one name, and the twelve names are the
twelve names `SPEC-MOK-001` fixes, in the assignment it fixes.

### Example: names are stable across runs

**Given** the seeds `0`, `1`, `42`, `123` and `777`

**When** each is initialized

**Then** `M01` reports the same name in all five, and so does every other identifier, because the assignment does not
read the seed.

### Example: the names are distinguishable

**Given** the twelve names

**When** they are compared

**Then** all twelve are distinct, all twelve first characters are distinct, every character is an ASCII letter, and no
name exceeds five characters.

### Example: the run is unchanged

**Given** the seeds `0`, `1`, `42`, `123` and `777`, the densities `0.15`, `0.75` and `1.50`, all three decision
sources, with and without `--trace-actions`

**When** a run of one thousand ticks completes

**Then** the standard output is byte-identical to the pre-change build's after the name field is removed from the
`agent_initialized` records, and the exit code is identical.

### Example: failure behavior

**Given** any accepted seed

**When** the simulation is initialized and run

**Then** no code path reports a name as absent, unset, empty or unavailable, because there is no such state; and no
Mokiterion's name changes at any tick.

## Open decisions

None. The twelve names, their assignment and the field's position in the record are `SPEC-MOK-001`'s to fix; the product
decision — that Mokiterions are named at all, from a fixed table, without displacing the identifier — is settled by this
requirement and by `INT-MOK-008`.
