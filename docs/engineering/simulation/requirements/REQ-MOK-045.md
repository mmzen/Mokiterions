+++
id = "REQ-MOK-045"
type = "requirement"
title = "Carry fear and suffered attacks on the observation so a defender answers for itself"
status = "approved"
owners = ["technical owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN the observation of a living Mokiterion's decision opportunity is built, THE SYSTEM SHALL carry that Mokiterion's own fear and the attacks it has suffered since its previous decision opportunity, and SHALL make fight, retreat and surrender available only as proposals its own decision source takes at that opportunity."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Carry fear and suffered attacks on the observation so a defender answers for itself

## Rationale

What a Mokiterion does when it is attacked is the most interesting decision in this world, and `INT-MOK-009` states that
it is not the engine's to make. That principle has one concrete consequence, and this requirement is it: damage resolves
immediately under engine authority, and the answer — `fight`, `retreat` or `surrender` — is a proposal the *defender's own
source* makes at the defender's own next decision opportunity. The engine does not choose it, does not compute a
retaliation, and does not grant the defender an out-of-turn opportunity.

That requires two fields on rule 3's observation, and both are additions to an approved specification that currently
refuses one of them by name.

**`fear`.** Rule 3 withholds it and says why: "`fear` is deliberately **not** carried: no rule and no decision source
reads it." Rule 12 closes with the same admission from the other side and names its own discharge: "**No rule reads
`fear`.** It is computed, bounded, reported and otherwise inert; a consumer is a later governed change." This is that
change. Both sentences are amended — rule 3's withholding is reversed, rule 12's closing is inverted — and neither is
quietly edited: the amendment states that a reader now exists and which requirement obliges it.

One sentence elsewhere breaks as a side effect, and it is fixed rather than left standing. The *Name* subsection
justifies a name's absence from the observation "for the same reason `fear` is not". Once `fear` is carried, that
borrowed reason is gone. A name's absence stands on its own ground — no decision source reads it, `REQ-MOK-041` obliges
that nothing does — and the subsection must state that ground itself.

**Attacks suffered since the previous opportunity.** A defender cannot answer an attack it cannot see, and it must see it
without the engine remembering anything durable. The window is exactly one decision opportunity wide: it opens when an
attack resolves against a Mokiterion and closes when that Mokiterion next takes an opportunity, whether or not it
answers. This is the narrowest field that makes the response a decision, and it is deliberately narrower than
interaction memory, which `CAP-MOK-009` excludes: nothing records *who has ever* attacked whom, only what has landed
since the defender last had a say.

**The latency this produces is asymmetric and is stated rather than corrected.** Rule 2 gives each Mokiterion its whole
cycle in its own turn, in ascending identifier order. A defender with a *higher* identifier than its attacker reaches its
opportunity later in the same tick and answers with zero ticks of latency; a defender with a *lower* identifier has
already acted and answers on the next tick. This is the same within-tick asymmetry rule 12 already documents for `fear`
— "two mutually adjacent Mokiterions may not both register the encounter on the same tick" — and it is one of the two
inputs to the identifier-advantage risk `VER-MOK-012` bounds.

**No inert value.** `SPEC-MOK-003` rule 4.5 refused a field nothing reads, and `fear` was the precedent for that refusal.
Both fields added here are read by the source `REQ-MOK-048` introduces, at the same opportunity they are carried on. If
that source did not exist, neither field would be added.

## Preconditions and trigger

The trigger is the construction of a living Mokiterion's observation at its decision opportunity under `SPEC-MOK-001`
rule 2, before its source is consulted.

A dead Mokiterion receives no observation, per rule 13, so the fields never exist for one.

## Required response

- **The observation carries the observing Mokiterion's own `fear`**, as its current bounded value in `0..=100`, alongside
  the `health`, `satiety` and `energy` it already carries. It is the observer's own `fear` and never another
  Mokiterion's; nothing on the observation exposes another Mokiterion's attributes, which is `CAP-MOK-009`'s exclusion of
  perceived relative strength.
- **The observation carries the attacks the observing Mokiterion has suffered since its previous decision opportunity**,
  sufficient for its source to propose an answer: for each, the striking Mokiterion's identifier and the damage it dealt.
  Where a Mokiterion has been struck more than once in the window, all are carried, in the order they resolved.
- **The window closes when the opportunity is taken.** After a Mokiterion's source has been consulted, the suffered-attack
  record for that Mokiterion is cleared, whether it answered, proposed something else, or had its proposal rejected. A
  Mokiterion gets exactly one opportunity to answer any given attack.
- **`fight`, `retreat` and `surrender` are valid only when that record is non-empty**, and only against a Mokiterion named
  in it. This is `REQ-MOK-043`'s precondition and this requirement supplies the state it is checked against.
- **`retreat` moves the defender one cell away from the named attacker**, on the `avoid` contract of `REQ-MOK-043`: one
  cell, one axis, the cardinal-move rules of `SPEC-MOK-001` rule 8, no additional energy cost beyond a move's, and a
  territory crossing emitted as usual. Retreating is not escape — the attacker may pursue with `approach` — and it ends
  no encounter, because `REQ-MOK-042` defines contact as a relation over positions and not as a state to be exited.
- **The record is per-Mokiterion state, not per-pair state.** It hangs off the defender, it is cleared by the defender's
  own opportunity, and no structure indexed by pairs of Mokiterions is introduced.
- **Nothing else reads the record.** It is not an input to damage, not an input to contact, not an input to survival
  decay, and not reported as an attribute. The resolution events of `REQ-MOK-044` already report what struck whom; this
  field exists so the defender's source can read it.

## Failure and boundary behavior

- A Mokiterion attacked twice before its next opportunity may answer only one attacker — the one it names — and the
  window closes on both. Answering is one action, and the closed contract of `REQ-MOK-043` applies exactly one action per
  opportunity.
- A Mokiterion attacked by a Mokiterion that has since died may still propose `fight` naming it; the proposal is rejected
  under `REQ-MOK-043`'s living-target precondition, the opportunity is consumed, and the window closes.
- A Mokiterion attacked by a Mokiterion that has since moved out of contact may propose `fight`; it is rejected on
  contact. `retreat` in the same situation is valid, because retreating from a non-adjacent attacker is a move away from a
  known position and needs no contact.
- A Mokiterion that dies of the attack takes no opportunity and never reads the record. The record dies with it.
- A Mokiterion whose `fear` is `0` or `100` carries that value; the bounds are rule 12's and are unchanged.
- No source is obliged to read either field. `baseline`, `reference` and `individual` continue to ignore what they ignore
  today, and carrying a field they do not read is not an inert value while `REQ-MOK-048`'s source reads it.

## Constraints

- **The observation stays a value the engine builds and the source only reads.** No source mutates it, and it holds no
  handle to authoritative state. This is `SPEC-MOK-001` rule 3's existing contract and it is not widened.
- **No name is added.** `REQ-MOK-041` obliges that nothing reads one, and the suffered-attack record names its attackers
  by identifier.
- **No other Mokiterion's attributes are carried.** The record carries damage dealt, which is a fact about what happened
  to the observer, not a reading of the attacker's condition.
- **Two fields, and the interface growth is counted** under `SPEC-MOK-002` rule 6.
- **No entropy draw** is taken to build an observation, clear a window, or resolve a `retreat` beyond what a move already
  takes — which is none.
- **No population aggregate** is carried or read. `REQ-MOK-050`.
- **No memory beyond the window.** No count of lifetime attacks, no last-attacker field that outlives the opportunity, no
  grudge. `CAP-MOK-009` excludes it and this field is bounded so that it cannot become it.

## Acceptance examples

### Example: normal behavior

**Given** a living Mokiterion at its decision opportunity

**When** its observation is built

**Then** the observation carries its own `fear` as a value in `0..=100`, and a suffered-attack record that is empty if
nothing has struck it since its previous opportunity.

### Example: zero-tick latency

**Given** `M03` attacking `M07` during tick `t`

**When** `M07` reaches its own opportunity later in tick `t`

**Then** `M07`'s observation carries that attack, and `M07` may answer within the same tick.

### Example: one-tick latency

**Given** `M07` attacking `M03` during tick `t`, after `M03` has already acted

**When** `M03` reaches its opportunity in tick `t+1`

**Then** `M03`'s observation carries that attack, and `M03` answers one tick after it was struck.

### Example: the window closes

**Given** a Mokiterion carrying one suffered attack

**When** it takes its opportunity and proposes `eat` instead of answering

**Then** the action is applied or rejected on its own terms, and at its next opportunity the suffered-attack record is
empty.

### Example: fear is read

**Given** a run under the source of `REQ-MOK-048`

**When** two Mokiterions differ only in `fear`

**Then** the proposals differ, demonstrating that the carried value reaches a reader and is not inert.

### Example: failure behavior

**Given** a Mokiterion that has suffered no attack since its previous opportunity

**When** its source proposes `fight`, `retreat` or `surrender`

**Then** each is rejected naming the unmet precondition, the opportunity is consumed, and neither Mokiterion's state
changes.

## Open decisions

- The field names and the shape of the suffered-attack record are the technical owner's, constrained by rule 3's
  contract, by `REQ-MOK-041`, and by `SPEC-MOK-002` rule 6's interface count.
- Whether the record is cleared before or after the action trace line of `SPEC-MOK-001` rule 7 is written **was** the
  technical owner's, and the trace must be able to report what the Mokiterion was answering. **It was decided on
  2026-08-20: the trace line is written first, and the record is cleared after it.** The reason recorded is that rule 7
  already fixes the traced `fear` as the pre-update value, so this extends that rule's existing principle rather than
  adding a second convention beside it; clearing first would empty the field on exactly the lines it exists to explain.
  It is no longer open, and it remains rule 7's to state as text.
- Whether a defender may answer a *second* attacker in a later opportunity when it never had the chance to answer the
  first is closed by this requirement: it may not. If measurement shows that discards a meaningful number of encounters,
  that is evidence for a later change.
