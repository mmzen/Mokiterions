+++
id = "CAP-MOK-006"
type = "capability"
title = "Per-Mokiterion individuality: a derived trait, a fear attribute, and a trait-aware decision source"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
derives_from = ["INT-MOK-006"]
+++

# Capability: Per-Mokiterion individuality

## Actor and need

The **product owner** needs the population to stop behaving as one organism with twelve bodies, so that the
concept's claim of "individuality without randomness" becomes something the simulation demonstrates rather than
something the README promises.

The **operator** needs to select a decision source whose Mokiterions differ from one another, run it against the two
existing sources at a matched seed and density, and read the difference as a property of the source rather than of
the run — which requires that the existing sources produce exactly what they produced before.

The **technical owner** needs the attribute model closed at four attributes before conflict is designed, because a
Mokiterion cannot choose between fighting, retreating and surrendering without a disposition to be afraid, and the
observer's roster has held space reserved for that attribute by name since Phase 1.5.

## Capability statement

The engine derives one behavioral trait per Mokiterion from the run seed and the Mokiterion's own identifier,
maintains `fear` as a fourth bounded dynamic attribute alongside health, satiety and energy, and offers a third
selectable decision source that consumes the trait, so that two Mokiterions facing the same situation may propose
different actions for a stated and reproducible reason — while the two existing decision sources continue to produce
identical outcomes.

## Boundaries

**Included.**

- One trait per Mokiterion, fixed for the run, derived by a pure function of the seed and the identifier, drawing
  nothing from the shared entropy stream.
- `fear` as a fourth attribute in the same bounded range as the other three: engine-computed, saturating, driven by
  what the Mokiterion perceives, and reported wherever the other three attributes are reported.
- One additional decision source, selectable through the existing option, that reads the trait when it evaluates a
  resource.
- Reporting the trait once per Mokiterion at initialization in the text record, and `fear` in the per-tick survival
  and trace records.
- The observer presenting `fear` in the roster slot `SPEC-MOK-003` rule 4.5 reserved for it, and the bar arithmetic
  correction that makes the reserved slot non-empty at the reference roster.
- A measured habitability floor for the new source, and recorded evidence of what individuality does to high-class
  resource accumulation at tick 1,000 and tick 10,000.

**Excluded.**

- Any behavior that reads `fear`. The attribute is computed and reported; it decides nothing in this capability.
- Any second or third trait. `aggression` and `sociability` have no consumer until conflict exists.
- Per-agent entropy substreams. See `INT-MOK-006` for why they are declined rather than deferred by accident.
- Any change to the behavior of `--policy baseline` or `--policy reference`, to the survival values, the resource
  table, the density mapping, the perception radius, regeneration, or the finality of death.
- Any change to the default decision source, to the default density, or to the exit-code contract.
- Combat, threat response, retreat, surrender, social action, memory of encounters, model-backed decisions,
  structured output, persistence, and batch execution.
- Any new package, any new external dependency, and any growth of the engine's dependency table, which stays empty.
- Any growth of the engine's public interface beyond one snapshot field and one option value.

## Outcomes

- A run at a given seed produces the same twelve traits every time, and stating the seed states the population.
- A Mokiterion's willingness to spend a meal it cannot fully use is its own, and the text record says what it is.
- The roster's fourth bar carries a value the engine computes, closing a reservation that has been correct and empty
  for one phase.
- A comparison between the new source and the reference source is a comparison of decision policy, because
  everything else in the run — resource placement, regeneration, order of action, survival arithmetic — is
  bit-identical.
- The high-class accumulation effect disclosed in `VER-MOK-002` is measured under a population that does not decline
  the same food at the same moment, and the measurement is recorded whether or not it improves.
- Conflict can be specified against four attributes instead of three.

## Candidate requirements

- `REQ-MOK-031` — derive a per-Mokiterion behavioral trait from the seed and the identifier, without consuming
  shared entropy.
- `REQ-MOK-032` — maintain `fear` as a fourth bounded dynamic attribute driven by perceived company, and report it
  wherever the other attributes are reported.
- `REQ-MOK-033` — provide a third selectable decision source that consumes the trait, leaving the existing sources'
  outcomes unchanged.
- `REQ-MOK-034` — sustain a viable population under the new source at the default density over the same window and
  floor already required of the reference source.

Presentation needs no new requirement. `REQ-MOK-020` already obliges the observer to present survival state, and
`SPEC-MOK-003` rule 4.5 already fixed where the fourth bar goes; both need an amendment, not a successor.
