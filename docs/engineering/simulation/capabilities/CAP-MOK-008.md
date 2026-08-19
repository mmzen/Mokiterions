+++
id = "CAP-MOK-008"
type = "capability"
title = "Named Mokiterions: an engine-computed name, reported and presented"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
derives_from = ["INT-MOK-008"]
+++

# Capability: Named Mokiterions

## Actor and need

The **operator** needs to read a run and follow twelve individuals through it, which means the twelve subjects need
names rather than serial numbers. `INT-MOK-006` made them behave differently; this makes the difference readable.

The **product owner** needs the project's own plain-language material to be able to tell a story. The guide currently
explains its central rule by tracing `M05` for fifteen ticks, and that sentence is the measure of the problem.

The **technical owner** needs the change to be additive in the strict sense: no rule reads a name, no entropy is
consumed, no public interface item is added, and no existing assertion is disturbed. A legibility improvement that cost
a re-measured floor or a relaxed test would not be worth having.

## Capability statement

The engine gives each Mokiterion one name from a fixed table of twelve, reports it once per Mokiterion in the
initialization record, and changes nothing else about the run; the observer presents that name wherever it identifies a
Mokiterion, including as the map glyph's first character — so that an operator reads a population of named individuals
while every identifier, every outcome and every entropy draw stays exactly what it was.

## Boundaries

**Included.**

- One name per Mokiterion, from a hardcoded table of exactly twelve, assigned by identifier position, fixed for all
  runs and consuming no entropy.
- The name reported once per Mokiterion in the `agent_initialized` record, positioned so that no existing consumer of
  that record changes.
- The observer presenting the name in the roster and the inspector, for living and dead Mokiterions alike, sourced from
  the engine's own reported record.
- The map glyph becoming the name's first character, and `SPEC-MOK-003` rule 2's glyph table amended as that rule
  already said it would be.
- `SPEC-MOK-003` rule 10 item 7 losing `name` from its list of values the engine does not compute, and rule 10's
  presented-value list gaining it.
- Evidence that the run is unchanged: every stream of the declared matrix identical to the pre-change capture once the
  added field is removed, and identical exit codes.

**Excluded.**

- Any replacement of, or change to, the `M01`–`M12` identifier, anywhere.
- Any behavior that reads a name: no rule, no decision source, no validation, no ordering, no tie-break.
- Seed-varying names, operator-supplied names, renaming during a run, and any name in any input.
- Names for resources, territories or events.
- Any growth of the engine's public interface. The name reaches the observer through the retained event stream, as
  `SPEC-MOK-002`'s 2026-08-19 amendment established for `waste_tolerance`.
- Any second attribute of personhood: age, pronouns, species, backstory, kills, memory.
- Any change to a decision source, the survival values, the resource table, the density mapping, the perception radius,
  regeneration, the finality of death, the exit-code contract, the default policy, the default density, or any
  requirement's floor.
- Combat, social behavior, model-backed decisions, structured output, persistence, batch execution.
- Any new package, target, build script or external dependency.

## Outcomes

- An operator who runs the simulation learns twelve names from its own output and can then read the log, the roster and
  the inspector in those names.
- The map's twelve glyphs become twelve distinct initials, which is a strictly better encoding than `1`–`9`,`A`,`B`,`C`
  for the same number of cells, because each one points at a name a reader already holds.
- A reader of `SIMULATION_RULES.md` follows a named creature through a worked example rather than a serial number.
- Every retained stream, every evidence file, every specification citation and every test that keys on `M01`–`M12`
  continues to work unchanged, so the corpus costs nothing.
- A later phase that wants a name — a conflict record, a model prompt — finds one already computed and reported, and
  does not have to add a field to do it.

## Candidate requirements

- `REQ-MOK-040` — the engine gives each Mokiterion a stable name from a fixed table and reports it, consuming no
  entropy and changing no outcome.
- `REQ-MOK-041` — the observer presents the name wherever it identifies a Mokiterion, and derives the map glyph from
  it.

Two requirements rather than one, because the two halves have different owners, different verification methods and
different failure modes: the engine's claim is about a record and an absence of change, and the observer's is about
characters in a buffer.
