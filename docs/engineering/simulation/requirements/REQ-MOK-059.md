+++
id = "REQ-MOK-059"
type = "requirement"
title = "Read no population aggregate anywhere a decision or a rule is made"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN the engine evaluates any rule governing a Mokiterion's behavior, consults any decision source, or validates any proposed action, THE SYSTEM SHALL read only the acting Mokiterion's own state, its own observation, and the state of any individual Mokiterion the rule names, and SHALL read no aggregate over the population."
verification_method = "static-analysis"

[relations]
derives_from = ["CAP-MOK-010"]
+++

# Requirement: Read no population aggregate anywhere a decision or a rule is made

## Rationale

`README.md` states this project's central prohibition as a line of code that must not exist:
`if food < threshold: start_war()`. Conflict must arise from what individual Mokiterions decide, not from a rule that
fires on a population-level condition. Every phase so far has satisfied that prohibition by not having conflict at all.
This one cannot, and that changes the nature of the obligation: the mechanism by which conflict *could* be scripted is
being built, so the absence of scripting stops being a consequence of the design and becomes a claim that has to be
checked.

**The claim has to be a property, not a search.** "No population-level trigger" cannot be verified by grepping a diff for
`start_war`, because the thing prohibited has no canonical name. It is verifiable as a restriction on **what is readable
where**: a rule, a source or a validation may read the acting Mokiterion's own state and the observation the engine built
for it, and nothing else. Anything that would need a census — how many are alive, how many are in this territory, how much
food remains in the world, how many encounters are in progress, how the population's average `fear` is trending — is
outside what those readers may see. This is the shape `REQ-MOK-041` used for names and `REQ-MOK-032` used for `fear`:
state the absence as a bound on reads, then check the bound.

**It has to be stated separately from the requirements it constrains** because it constrains all of them at once and
because it is the assurance owner's rather than the product owner's. `REQ-MOK-052` through `REQ-MOK-057` each repeat a
narrow form of it where the temptation is local — contact would like a count, target selection would like a ranking, the
`social` source would like to know if it is winning. Repetition in those requirements is not redundancy; it is where the
obligation will actually be tested. This requirement is what those repetitions derive from, and it extends to the rules
that already exist as well as the ones this initiative adds.

**And the temptation is now concrete rather than hypothetical.** `REQ-MOK-058` states a survivor floor. The cheapest way
to meet a survivor floor is to have the engine suppress combat when the population falls low — which is
`if food < threshold: start_war()` with the sign reversed, and would pass every test `REQ-MOK-058` states while
destroying the property that makes those tests worth passing. This requirement is what forbids it.

The separation it draws is between the simulation and its measurement. Aggregates are legitimate and necessary
*outside* the simulation: `REQ-MOK-014`, `REQ-MOK-034` and `REQ-MOK-058` are all survivor counts, `VER-MOK-016` counts
deaths by cause, and the observer displays a population. None of that is prohibited. What is prohibited is an aggregate
crossing back into anything that decides or resolves.

**One boundary has to be drawn before the obligation can be stated, because the system already crosses it legitimately.**
`SPEC-MOK-001` rule 15 reads how many resources a territory holds and compares it to that territory's capacity; rule 18's
final summary reports remaining food by territory and class. Those are aggregates over the *world*, read by rules that
place resources and by a report — not aggregates over the *population*, and not inputs to any Mokiterion's decision. This
requirement bounds what may inform a Mokiterion's behavior. It does not bound what the world's own bookkeeping may count
about itself, and it is written so that rules 15 and 16 satisfy it as they stand.

## Preconditions and trigger

The trigger is the evaluation of any `SPEC-MOK-001` rule that governs a Mokiterion's behavior, any consultation of a
decision source, and any validation of a proposed action — for every such rule and every source, not only those this
initiative adds.

Rules that place or replenish resources — rules 14, 15 and 16 — and rules that report — rule 18 — are outside this
requirement's obligation, on the boundary stated above. They decide nothing about any Mokiterion.

## Required response

- **A decision source reads the observation it was given, and the observation only.** It holds no reference to
  authoritative world state, no collection of Mokiterions, and no state carried between opportunities.
- **The observation carries no aggregate.** No living count, no population total, no territory census, no world food
  total, no tick-scoped statistic, and no summary of any set the observing Mokiterion is a member of. It carries the
  observer's own attributes and the food and Mokiterions the observer perceives, each as an individual entry.
- **A behavioral rule reads the acting Mokiterion's own state and, where the rule acts on a second Mokiterion, that
  Mokiterion's own state as the rule's stated inputs require.** `REQ-MOK-053`'s damage reads the striker's `energy` and
  `health`; `REQ-MOK-055` writes the target's `fear`; `REQ-MOK-056` moves `satiety` between two named Mokiterions. Those
  are reads and writes of named individuals, which this requirement permits and bounds — and none of them is a read of a
  set.
- **No rule's behavior depends on how many Mokiterions exist, are alive, are in contact, are in a territory, or are in any
  other condition.** No threshold on a count, no branch on a proportion, no scaling by a population size.
- **No proposal validation reads an aggregate.** A rejection reason names the unmet precondition of that proposal, derived
  from the two Mokiterions and the world cell in question.
- **No target selection or tie-break reads an aggregate.** Ties are broken by identifier order, which is a total order over
  individuals and not a statistic about them.
- **The verification and observation paths may read aggregates freely**, and this requirement does not restrict
  `REQ-MOK-014`, `REQ-MOK-034`, `REQ-MOK-058`, `VER-MOK-016`, the event stream's terminal summary, or the observer. It
  restricts only the direction of flow: nothing computed over the population may become an input to a rule, a source or a
  validation.
- **The check is stated and repeatable.** The obligation is discharged by a documented static examination of every rule
  evaluation, every `DecisionSource` implementation and every validation path, enumerating what each reads, recorded as
  evidence under the work order — not by an assertion that nobody wrote the forbidden line.

## Failure and boundary behavior

- A rule that reads a count of living Mokiterions violates this requirement even if the count never changes its outcome on
  any declared seed. The bound is on the read, not on the effect, because an unexercised dependence is a latent one.
- A decision source that reads a population aggregate violates this requirement even if it is a source no operator selects
  by default.
- A source reading the number of entries in *its own observation* — how many Mokiterions it perceives, how many foods it
  perceives — does **not** violate this requirement. That is a fact about one Mokiterion's perception, bounded by the
  perception radius, and rule 12 already depends on exactly it: `fear` rises when the observation lists another living
  Mokiterion. The distinction is between counting what one individual can see and counting what exists.
- Rule 15 reading a territory's resource count against its capacity does **not** violate this requirement, and neither does
  rule 16 selecting a coordinate from the free coordinates of a territory. Both are the world placing resources, and
  neither informs any Mokiterion's decision. A change to either that made a *Mokiterion's* behavior depend on that count
  would violate it.
- A run's terminal summary reporting survivors does not violate this requirement, because nothing reads it back.
- The observer displaying a population count does not violate this requirement. `mokiterions-tui` decides nothing.
- No violation of this requirement produces a runtime error. It is a structural property and its failure mode is a failed
  verification row, not a failed run.

## Constraints

- **The bound covers existing rules, not only new ones.** Every rule among 1 to 19 of `SPEC-MOK-001` that governs a
  Mokiterion's behavior is examined on the same terms as the rules this initiative appends, so that the claim is about the
  system rather than about the change. Rules 14 to 16 and 18 are enumerated too, and recorded as outside the obligation
  with the reason, rather than omitted from the examination.
- **The bound covers all four decision sources**, including `baseline`, whose uniform selection reads its candidate list
  and nothing else.
- **No new interface is introduced to carry an aggregate**, and no existing interface is widened to make one reachable from
  a source.
- The enumeration is recorded as evidence and is expected to be re-derived when rules or sources change, on the ground that
  a static claim about reads goes stale exactly when the code moves.

## Acceptance examples

### Example: normal behavior

**Given** the engine's rule evaluations, its four decision sources and its proposal validation

**When** each is examined against this requirement

**Then** the recorded enumeration shows every read resolving to the acting Mokiterion's own state, its observation, or a
named individual the rule acts on.

### Example: perception is not a census

**Given** rule 12's `fear` write, which depends on whether the observation lists another living Mokiterion

**When** it is examined

**Then** it satisfies this requirement, because it reads the observer's own perception and not the population.

### Example: the prohibited shape

**Given** a hypothetical rule that suppresses combat while fewer than seven Mokiterions are alive

**When** it is examined

**Then** it violates this requirement, and it does so even though it would raise the survivor count `REQ-MOK-058`
measures.

### Example: measurement is unaffected

**Given** `REQ-MOK-058`'s survivor floor and `VER-MOK-016`'s deaths-by-cause count

**When** both are evaluated

**Then** both read aggregates, neither violates this requirement, and neither value is reachable from any rule, source or
validation.

### Example: failure behavior

**Given** any rule, source or validation path that reads a count or summary over the population

**When** the examination is performed

**Then** this requirement is not met, and the finding names the reader and what it read.

## Open decisions

- Whether the enumeration is produced by an automated check over the source or by a documented manual examination is the
  assurance owner's, constrained by its needing to be repeatable and recorded. A manual examination is acceptable and is
  what `verification_method = "static-analysis"` means elsewhere in this repository; it is not acceptable for it to be
  unrecorded.
