+++
id = "REQ-MOK-047"
type = "requirement"
title = "Surrender forfeits satiety to the attacker and settles the encounter"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a valid surrender action resolves, THE SYSTEM SHALL transfer a stated amount of the surrendering Mokiterion's satiety to the Mokiterion it surrendered to, SHALL saturate the recipient's satiety at ATTRIBUTE_MAX and discard any excess, and SHALL apply no damage and no further effect."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Surrender forfeits satiety to the attacker and settles the encounter

## Rationale

Surrender has to cost something the surrendering Mokiterion cares about, or it is a free way out and no source will ever
choose `fight`. It cannot cost `health`, because then it is a weaker attack and not a different answer. Forfeiting
`satiety` to the attacker makes it the one action in this world that transfers a resource between Mokiterions: the loser
buys its life with food it has already eaten, and the winner gets fed for winning. That is the shape of the decision the
product owner wants a defender to face — take damage and maybe die, run and stay hungry, or live and starve sooner.

It also gives combat a reason beyond attrition. An attacker with a food motive has something to gain from an encounter
that is not merely another Mokiterion's absence, which is what makes `attack` a plausible proposal for a source reading
its own `satiety` rather than only its own `fear`.

**The magnitude the product owner decided on 2026-08-20** is **half the surrendering Mokiterion's `satiety`**, the division
truncating toward zero — a proportion rather than the flat constant that was the alternative. Surrender therefore costs
what the surrendering Mokiterion can bear: `40` from a Mokiterion at `80`, `10` from one at `20`, nothing from one at `1`.

Three consequences are recorded here rather than left to be found. **The forfeit can never exceed what the payer holds**, so
this requirement's "saturating at zero" clause is satisfied by construction and constrains a later amendment rather than
this one. **Surrender is free below `satiety` `2`**, by truncation, which means the answer a starving Mokiterion can always
afford is the one that costs it nothing — deliberate under a proportional forfeit, and the opposite of what a flat
constant would have produced, where a poor Mokiterion pays everything it has. And **the amount the attacker gains is no
longer predictable from the specification alone**: it depends on the victim's `satiety`, so a well-fed victim is worth
attacking and a starving one is not, without either Mokiterion reading the other's state — the attacker learns the value
only after the fact, from what it received.

**The forfeit needs an explicit cap rule, and this is why.** `SPEC-MOK-001` rule 9's non-waste condition —
`satiety + restoration <= ATTRIBUTE_MAX` — is a precondition on *choosing to eat*: a Mokiterion may not select a food
whose restoration it cannot fully absorb. It is not a cap on *receipt*, because until now nothing could give a
Mokiterion satiety it had not chosen. A forfeit is received, not chosen, so the non-waste condition does not apply and
cannot be stretched to apply. This requirement therefore states the cap directly: the recipient saturates at
`ATTRIBUTE_MAX` and **the excess is destroyed rather than left with the loser**. Conservation is deliberately not
preserved. Returning the remainder would mean a well-fed attacker takes less, which reads as mercy from arithmetic, and
would make the forfeit's size depend on the recipient's condition — a reading of the other Mokiterion's state that
`CAP-MOK-009` excludes on both sides.

**"Settles" is the honest verb, and "ends the encounter" needs to be read narrowly.** There is no encounter object to
end. `REQ-MOK-042` defines contact as a relation recomputed from positions, and `CAP-MOK-009` excludes interaction
memory, so there is nothing to hold a truce in. What a surrender ends is the answer: the transfer is the whole
settlement, the suffered-attack window closes under `REQ-MOK-045`, and neither Mokiterion carries any further obligation
or protection. A Mokiterion that surrendered may be attacked again on the next tick, by the same attacker, and must
surrender again or choose otherwise. A bounded immunity was considered and declined, because the only way to hold one is
per-pair state.

## Preconditions and trigger

The trigger is the engine applying a valid `surrender` under `REQ-MOK-043`: the acting Mokiterion is living, has an
attack in its suffered-attack record under `REQ-MOK-045`, and names as its target a living Mokiterion in that record.

Contact is **not** required. A Mokiterion may surrender to an attacker that has since moved away, on the same ground
`retreat` needs no contact: the answer is to the attack, and the attack has already happened.

## Required response

- **Half the surrendering Mokiterion's `satiety`, truncating toward zero, is transferred to the named attacker.** The
  amount is fixed by `SPEC-MOK-001` as `satiety / 2` and derived only from the surrendering Mokiterion's own `satiety` at
  the moment of resolution — not from the attacker's attributes, not from the damage suffered, not from either identifier,
  not from any trait.
- **The surrendering Mokiterion's `satiety` is reduced by the transferred amount, saturating at zero.** A Mokiterion with
  little to give gives little; a Mokiterion at `satiety` `0` gives nothing and the action still succeeds, because it has
  still declined to fight. At the decided proportion the reduction can never exceed what is held, so the saturation is a
  guarantee about later amendments rather than a live case.
- **The recipient's `satiety` is increased by the same amount, saturating at `ATTRIBUTE_MAX`, and any excess is
  discarded.** The excess is not returned to the surrendering Mokiterion, not banked, and not converted into another
  attribute.
- **No damage is dealt, to either party.** No `health` changes, no `energy` cost is paid by either Mokiterion, and neither
  moves.
- **No `fear` is written by a surrender**, in either direction. Rule 12's own write still applies to each Mokiterion at
  its own opportunity, and `REQ-MOK-046` is the only other rule that writes `fear`.
- **The suffered-attack window closes**, as it does for any answer, under `REQ-MOK-045`. The surrendering Mokiterion may
  not also `fight` or `retreat` that attack.
- **The surrender is reported as one event carrying both transitions** — the surrendering Mokiterion's identifier, the
  recipient's identifier, the amount transferred, the amount discarded, and both resulting `satiety` values. The amount
  discarded is reported rather than inferred, so that a run's own output shows where non-conservation occurred.
- **Death by starvation remains rule 12's.** A surrender may reduce a Mokiterion's `satiety` to zero, and if that
  Mokiterion then dies, it dies through the existing survival path with the existing event and the existing cause. A
  surrender is not itself a cause of death.

## Failure and boundary behavior

- A surrendering Mokiterion at `satiety` `0` transfers `0`. The action succeeds; it is not a rejection.
- A surrendering Mokiterion at `satiety` `1` also transfers `0`, by truncation. Surrender is free in the band `0..=1` and
  costs `1` at `satiety` `2` or `3`. The cheapest answer is therefore always available to the Mokiterion least able to pay,
  which follows from choosing a proportion and is not a defect to round away.
- A recipient at `satiety` `100` receives `0` and the whole transferred amount is discarded; the surrendering Mokiterion
  still pays it. This is the non-conservation case and it is reported.
- A surrender naming a Mokiterion that has since died is rejected under `REQ-MOK-043`'s living-target precondition, the
  opportunity is consumed, and no `satiety` moves.
- A surrender naming a Mokiterion not in the suffered-attack record is rejected.
- A Mokiterion that surrenders on one tick and is attacked again on the next may surrender again, and pays again. There is
  no immunity and no diminishing cost.
- No surrender produces a configuration error, a runtime error or a non-zero exit code.

## Constraints

- **The forfeit's magnitude is the stated integer function `satiety / 2` of the surrendering Mokiterion's own `satiety`,
  fixed in `SPEC-MOK-001`.** Integer arithmetic with the division truncating toward zero, saturating on both sides, no
  floating point and no rounding rule other than truncation.
- **No entropy draw.**
- **No population aggregate read.** `REQ-MOK-050`.
- **This is the only rule that moves any attribute from one Mokiterion to another.** Damage under `REQ-MOK-044` reduces
  without crediting anyone; a threat under `REQ-MOK-046` writes without spending. Transfer is unique to surrender and the
  set is closed.
- **No per-pair state.** No truce, no cooldown, no record that this pair has settled. `CAP-MOK-009`'s exclusion of
  interaction memory is not breached by this requirement.
- **The non-waste condition of `SPEC-MOK-001` rule 9 is not modified.** It continues to govern the choice to eat, and this
  requirement states its own cap rather than extending rule 9's.

## Acceptance examples

### Example: normal behavior

**Given** a living Mokiterion with an unanswered attack, holding `satiety` well above the forfeit, and an attacker with
room to receive it

**When** its valid `surrender` resolves

**Then** the stated amount moves from the surrendering Mokiterion to the attacker, no `health` or `energy` changes for
either, and one event reports both resulting `satiety` values with a discarded amount of `0`.

### Example: the recipient is full

**Given** an attacker at `satiety` `100`

**When** a valid `surrender` resolves against it

**Then** the surrendering Mokiterion pays the full amount, the attacker's `satiety` stays `100`, and the event reports the
whole amount as discarded.

### Example: nothing left to give

**Given** a surrendering Mokiterion at `satiety` `0`

**When** its valid `surrender` resolves

**Then** the action succeeds, `0` is transferred, no attribute of either Mokiterion changes, and the event reports it.

### Example: no immunity

**Given** a Mokiterion that surrendered to `M02` on tick `t`

**When** `M02` attacks it again on tick `t+1`

**Then** the attack is valid and resolves under `REQ-MOK-044`.

### Example: starvation after a forfeit

**Given** a surrender that reduces a Mokiterion's `satiety` to `0`

**When** survival decay runs at its next opportunity

**Then** it dies, if at all, through rule 12's existing path with the existing cause, and no event attributes its death to
the surrender.

### Example: failure behavior

**Given** a `surrender` naming a Mokiterion absent from the suffered-attack record, or one that has died

**When** it is validated

**Then** it is rejected naming the unmet precondition, the opportunity is consumed, and no `satiety` moves in either
direction.

## Open decisions

- **Decided by the product owner on 2026-08-20: a proportion, `satiety / 2`.** A proportion makes surrender affordable when
  poor and expensive when rich; the flat constant that was declined would have been a fixed price a poor Mokiterion cannot
  fully pay. It remains `SPEC-MOK-001`'s to state and the product owner's to approve as specification text, which is a
  separate act from approving this requirement.
- Whether a bounded truce should exist at all is deferred, not left open in the implementation. It is declined here for its
  cost in per-pair state, and `VER-MOK-012`'s measurement of repeat surrenders between the same pair is the evidence a
  later change would be taken on.
