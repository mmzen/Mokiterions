+++
id = "INT-MOK-008"
type = "intent"
title = "Give each Mokiterion a name"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
+++

# Intent: Give each Mokiterion a name

## Problem

**A Mokiterion is a someone, and the engine addresses it as a serial number.** `SPEC-MOK-001`'s state model gives each
one "a stable identifier from `M01` through `M12`", and that identifier is the only handle anything has on it: the
event stream's `subject=`, the roster's first column, the inspector's title, the map glyph, and every sentence anyone
writes about a run. The concept the project is built on asks for twelve individuals whose divergence a reader can
follow. `INT-MOK-006` delivered the divergence. Nobody can follow it, because the twelve subjects of the story are
called `M01` through `M12`.

This is not a cosmetic complaint, and the plain-language guide is where it shows. `SIMULATION_RULES.md` §14 traces one
Mokiterion through fifteen ticks to explain the non-waste rule, and every sentence of that trace has to say `M05`.
`INT-MOK-006` was approved on the argument that individuality must be "attributable to a stated value rather than to a
coin toss" — and it is, but the attribution lands on a row of a table rather than on anyone. Reading a divergence
record and reading a story about two creatures are different experiences, and only the second one makes a reader ask
why.

**The observer feels it hardest, and it cannot fix it alone.** `SPEC-MOK-003` rule 2 already commits to the change in
writing: its glyph table records that identifiers "carry no names" and that "when agent naming is introduced by a later
phase, the glyph becomes the name's first character and this table is amended". Rule 10 item 7 lists `name` first among
the values the engine does not compute, and therefore forbids the inspector from showing one. `WO-MOK-005` bound the
observer to "present no value the engine does not compute, including an inert placeholder". The observer has been
waiting on the engine for two phases, and the waiting is recorded in the specification rather than implied.

**Nothing else in the project is blocked behind this, and that is the honest framing.** Conflict does not need names;
a model-backed source does not need them either, though a prompt that says "You are Zug" reads better than one that
says "You are M07". This initiative buys legibility, and legibility is the whole of what it buys. It is proposed on the
ground that a simulation whose stated purpose is emergent behavior an observer can interpret should not make the
observer do the interpreting through a serial number.

`INT-MOK-006` is achieved and verified in substance under `VREC-MOK-007`. Its outcomes are behavioural — trait, fear,
source, habitability — and naming is none of them. A completed intent is not the place to record a new outcome, which
is the reason this one exists rather than an amendment to that one.

## Desired outcomes

- Every Mokiterion has a name that the engine computes and reports, so an operator reading a run's own output learns
  the population's names from the run rather than from a document.
- The name is short, pronounceable, and distinct from the other eleven at a glance and at a single character.
- `M01` through `M12` continue to mean exactly what they mean today, in `subject=`, in every retained stream, and in
  every citation across the corpus. **Nothing that already identifies a Mokiterion stops working.**
- The observer presents the name wherever it identifies a Mokiterion, and the map glyph becomes the name's first
  character, which is the amendment `SPEC-MOK-003` rule 2 already anticipated.
- A run's outcome is unchanged in every respect. Survivors, deaths, positions, standing resources and the entropy
  sequence are what they were, at every seed, under every source.
- The plain-language guide can tell the story of a tick in names.

## Actors and stakeholders

- The operator gains a name in the output and in every pane that identifies a Mokiterion, and loses nothing: the
  identifier is still there and still means what it meant.
- The product owner owns the names themselves, because a name is a product decision and not a technical one, and owns
  the judgement that twelve hardcoded names are enough for a world of exactly twelve agents.
- The technical owner owns the two specification consequences: the engine's record format gains a field, and the
  observer's glyph derivation changes its input.
- The assurance owner owns the claim that carries this initiative's only real risk, which is again a claim of
  *absence* — that a run's behavior did not change — and it is a much narrower claim than `INT-MOK-006`'s, because no
  rule reads a name.
- Developers and implementation agents write a table of twelve strings and thread it to two presenters.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Mokiterions with a name the engine computes | 0 of 12 | 12 of 12 | Automated verification |
| Distinct first characters across the twelve names | not applicable | 12 | Automated verification |
| Values in `SPEC-MOK-003` rule 10 item 7's not-computed list that the engine now computes | 1 | 0 | Static check |
| Survivors, deaths, final positions and standing food differing from the baseline capture, under every source | 0 | 0 | Automated verification, declared matrix |
| Entropy draws taken by naming | not applicable | 0 | Automated verification |
| Engine package external dependencies | 0 | 0 | Every build |
| Public interface items added | not applicable | 0 | Static check |
| Existing tests requiring an assertion change | not applicable | 0 | Test census |

The last two are stated as targets rather than as hopes, and both are achievable by design. The public interface does
not need to grow because the observer already retains the event stream the name is reported in. No existing assertion
needs changing because the name can be placed where no current parser looks. A change that missed either target would
still deliver the outcome, and would cost more than the outcome is worth.

## Non-goals

- **Renaming the identifier.** `M01` through `M12` stay, in `subject=`, in the summary, in every specification, every
  verification record and every retained evidence file. A name is an addition to identity, not a replacement for it.
  This is the constraint that keeps the initiative cheap and the corpus intact.
- **Names that vary with the seed.** A seed-derived permutation is considered and declined in *Risks and assumptions*
  below, on a technical ground rather than a preference.
- **Names for anything but Mokiterions.** Resources keep `F0058`. They are not someones.
- **Any behavior that reads a name.** No rule, no decision source, no validation, no tie-break. Ordering stays on the
  identifier, because ordering by name would silently reorder acting order and move every outcome.
- **Any change to a decision source, a survival value, the resource table, the density mapping, the perception radius,
  regeneration, the finality of death, the exit-code contract, or any floor.**
- Renaming, nicknaming or user-supplied names, a `--names` option, localization, or a name in any input.
- Pronouns, gender, age, species, backstory, or any other attribute of personhood. One field.
- Combat, social behavior, model-backed decisions, persistence, and a batch runner.
- Any new package, target, or external dependency.

## Principles and immutable constraints

- The engine is the only authority over world state, and a name is state. The observer presents a name the engine
  computed and reported; it never invents, derives or completes one. This is `WO-MOK-005`'s constraint and it is not
  relaxed.
- **The identifier is the join key and stays the join key.** Every mechanical consumer of the output — every test,
  every parser, every evidence file, every future analysis — keys on `M01`–`M12`. A name is for people.
- Determinism is absolute, and here it is free: a name drawn from a fixed table consumes no entropy, so the shared
  stream cannot move.
- No inert value. A name that the engine held and never reported would be exactly the placeholder `SPEC-MOK-003` rule
  4.5 refused for `fear`.
- The public interface is a ceiling and does not need to move for this. If it turns out to need to move, that is a
  signal the design is wrong rather than a licence to grow it.
- Keep it small. Twelve strings, one record field, two presenters.

## Risks and assumptions

- **Fact: the observer cannot do this alone, and the specification says so.** `SPEC-MOK-003` rule 10 item 7 lists
  `name` among values the engine does not compute, and `WO-MOK-005` forbids the observer from presenting one. A
  TUI-side name table is not a cheaper version of this initiative; it is prohibited.
- Fact: `SPEC-MOK-003` rule 2's glyph table already states the amendment this initiative makes to it, including that
  "nothing here anticipates that value". The observer's half of this work was designed a phase ago.
- **Assumption, and the property everything rests on: a name consumes no entropy and no rule reads it, so the run is
  unchanged.** This is stronger than `INT-MOK-006`'s equivalent assumption, which had to survive a derivation running
  beside the shared stream. Here there is no derivation. It is nonetheless verified rather than asserted, because it is
  the one thing that could be wrong.
- **A seed-derived permutation is declined, and the reason is the glyph.** `SPEC-MOK-003` rule 2.5 requires every
  distinction carrying identity to be available without colour, and names Mokiterions by glyph. If the glyph is the
  name's first character, then twelve distinct first characters is an obligation, not a preference. A permutation drawn
  from a pool cannot guarantee it without partitioning the pool by first letter and drawing one from each partition —
  machinery whose only product is a different arbitrary assignment each run. A fixed assignment guarantees the property
  by construction and consumes provably nothing. The side-generator pattern `SPEC-MOK-001`'s *Behavioral trait*
  established remains available if seed-varying names are ever wanted, and would then have to solve the partition
  problem.
- Assumption: twelve hardcoded names are sufficient because the world holds exactly twelve agents, which
  `SPEC-MOK-001`'s *Performance and capacity* section fixes. A thirteenth Mokiterion would need a thirteenth name, and
  it would need a governed change to the population size first. The table's length being tied to the population is a
  property to assert, not a coincidence to rely on.
- Risk, and the one this initiative can actually get wrong: **a name placed where an existing parser looks would break
  a test, and a test that must change is a stop condition in this repository.** Two consumers parse the initialization
  record positionally — `mokiterions-core/tests/process.rs` asserts `,fear:0,waste_tolerance:` as an adjacent pair, and
  `mokiterions-core/tests/viability.rs` reads the trait by splitting on the record's last field. The mitigation is
  placement rather than negotiation: the field goes where neither looks.
- Risk: the output stream is not byte-identical to the previous build, because one record kind gains a field. Every
  equivalence claim in this initiative is about outcomes and about all other lines, never about the whole stream's
  bytes, and the distinction must be stated wherever the claim is made — as it was under `INT-MOK-006`.
- Risk: a name is a word, and words carry meaning nobody intended. The names are invented rather than borrowed from
  any living language's given-name stock, and the product owner owns the judgement that each is inoffensive. This is a
  manual assessment because no automated check can make it.
- Assumption: the engine's dependency table stays empty and the observer's stays at one path dependency and `ratatui`.
  Twelve string literals need nothing.
