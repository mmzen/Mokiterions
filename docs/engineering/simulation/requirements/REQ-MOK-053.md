+++
id = "REQ-MOK-053"
type = "requirement"
title = "Resolve an attack deterministically and permit death by combat"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a valid attack or fight action resolves between two living Mokiterions, THE SYSTEM SHALL reduce the target's health by an integer amount determined only by the striker's energy and health, SHALL apply a stated energy cost to the striker, SHALL take no draw from the entropy stream, and SHALL mark a Mokiterion whose health reaches zero as dead through the existing death path."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-010"]
+++

# Requirement: Resolve an attack deterministically and permit death by combat

## Rationale

An attack that cannot kill is a gesture. `INT-MOK-010` states death by combat as a desired outcome and as something
reached at every declared seed, because a mechanism whose worst outcome is a smaller number is not conflict and would
make `fear` a reasonable thing to ignore.

Resolution is deterministic — an integer function of the striker's energy and health, with no entropy draw — for three
reasons, in descending order of weight. First, `REQ-MOK-009` and `INT-MOK-001`'s identical-results measure are
preserved without argument: a rule that draws nothing cannot move the shared stream, and the shared stream is what
`baseline` reproduction depends on. Second, every outcome is explainable from the two Mokiterions' own reported states,
which is what makes the event stream sufficient for an operator to reconstruct an encounter. Third, it keeps this
requirement's verification arithmetic exact rather than distributional: a test can assert the resulting health, not a
confidence interval.

The cost of determinism is stated plainly and is the reason `REQ-MOK-058` and the identifier-monotonicity bound exist.
With no draw to wash it out, the Mokiterion that strikes first strikes with full information and full health, and rule
2's ascending-identifier order decides who that is. Determinism does not create the advantage — the acting order does —
but it removes the noise that would have hidden it. The owner's decision is that **verification bounds it rather than the
rule compensating for it**: no reordering, no initiative roll, no first-strike penalty. If the measurement shows win
rate or survival monotonic in identifier, that is a finding to act on under its own change, not something this rule is
tuned to avoid.

Both `attack` and `fight` resolve through this rule. They differ in precondition — `attack` requires contact,
`fight` requires an unanswered attack — and not in effect. A defender that fights back strikes on the same terms it was
struck on, which is what makes the deferred response a real choice rather than a differently-named surrender.

**The values the product owner decided on 2026-08-20**, from three candidate shapes, are the steep additive form
`damage = 10 + (striker.energy + striker.health) / 10` with the division truncating toward zero, and a flat cost of `5`
`energy` per strike. Damage ranges from `10`, for a striker at `energy` zero and `health` `1`, to `30` for one at full
`energy` and full `health`, so a target at full `health` dies to between four and ten strikes and between `20` and `50`
`energy` buys a kill.

Three consequences of those values are recorded here rather than discovered during implementation. **The damage floor this
requirement states — at least `1` — is satisfied by construction**, so it constrains nothing about the arithmetic; the
binding minimum is `10`, and the floor's purpose is now to forbid a *later* amendment from making a resolution that
removes nothing. **A shallower alternative was declined** because at a range of `1` to `9` a kill needs twelve to a
hundred strikes, and at the contact rate `REQ-MOK-032`'s arithmetic gives, `REQ-MOK-058`'s one-death-per-seed bound would
likely have been unreachable. **And the `5` cost is deliberately a weak brake**: it does not stop a striker pressing an
attack through to a kill, which places `REQ-MOK-058`'s survivor floor on the `social` source's willingness to attack rather
than on the cost of attacking. That was the owner's choice with the trade named, and it is why `REQ-MOK-057`'s
survival-first ordering is load-bearing rather than a stylistic preference.

## Preconditions and trigger

The trigger is the engine applying a valid `attack` or `fight` under `REQ-MOK-052`, after every precondition of
`REQ-MOK-051` and `REQ-MOK-052` has been checked against authoritative state.

Both Mokiterions are living at the moment of resolution. A target that died earlier in the same tick yields a rejection
under `REQ-MOK-052` and never reaches this rule.

## Required response

- **Damage is an integer function of the striker's `energy` and `health` at the moment of resolution, and of nothing
  else.** Not of the target's attributes, not of the tick number, not of either Mokiterion's identifier, not of any trait
  and not of any population aggregate. Deriving damage from the striker's own condition is what makes a weakened
  Mokiterion a weaker attacker without any Mokiterion reading another's strength, which `CAP-MOK-010` excludes.
- **The function is `damage = 10 + (striker.energy + striker.health) / 10`**, the division truncating toward zero,
  evaluated at the moment of resolution in an integer width no intermediate overflows. Its range is `10..=30`.
- **Damage is at least `1` for a living striker.** A resolution that removes nothing is indistinguishable from a
  rejection and would make the event stream report an attack that did not happen. The decided function's minimum is `10`,
  so this bound is satisfied by construction and stands as a constraint on any later amendment rather than on this one.
- **The target's `health` is reduced by that amount, saturating at zero**, on the same saturating arithmetic every other
  survival write in `SPEC-MOK-001` rule 12 uses. Health does not go negative and is not clamped by any other attribute.
- **The striker pays a flat `5` `energy` per strike**, saturating at zero, applied whether or not the target dies. An
  attack that is free would make attacking dominate every alternative for every source that has it; `5` is a cost rather
  than a deterrent, and the deterrent is the source's own ordering.
- **No entropy draw is taken** by validation, damage computation, cost application or death handling.
- **A Mokiterion whose `health` reaches zero is dead**, through `SPEC-MOK-001` rule 13's existing path and its existing
  event: same finality, same immediacy, same exclusion from all later opportunities in the same tick and every tick after.
  Death by combat is not a second kind of death.
- **The resolution is reported as one event carrying both parties' transitions** — the striker's identifier, the target's
  identifier, the damage applied, the target's resulting `health`, the striker's resulting `energy`, and whether the
  target died. This is the reporting obligation `INT-MOK-010` records as a risk: rule 2 runs each Mokiterion's whole
  cycle in its own turn, so a target with a higher identifier than its striker has not yet emitted its
  `survival_changed` line and one with a lower identifier already has. Carrying the transitions on the event makes the
  record complete in both directions.
- **Death by combat is distinguishable in the stream from death by starvation or exhaustion**, so that the count
  `REQ-MOK-058` and `VER-MOK-016` measure can be taken from the run's own output rather than inferred.

## Failure and boundary behavior

- A striker at `energy` zero still deals `10` damage, the function's minimum, and pays a cost that saturates at zero. Being
  spent does not make a Mokiterion harmless: at these values it remains a third as dangerous as a fresh striker and can
  still kill a full-health target in ten strikes. The requirement does not introduce an energy precondition on attacking;
  if the owner later wants one, that is a change to `REQ-MOK-052`'s precondition list, not to this rule.
- A target at `health` `1` dies to any resolution. This is the intended reachable path, not an edge case to soften. At the
  decided function a target at `health` `30` or below dies to a single strike from a striker at full `energy` and `health`,
  which makes a one-blow kill an ordinary outcome rather than a corner of the range.
- A striker that kills its target still pays its own cost, and the tick continues. There is no follow-up action, no free
  move onto the target's cell, and no resource transfer — `REQ-MOK-056` is the only rule that moves satiety between
  Mokiterions, and it triggers on surrender rather than on death.
- A striker that dies of survival decay later in the same tick does not undo the damage it dealt. Resolution is not
  transactional across the tick.
- Two Mokiterions may each resolve an attack against the other within one tick, in either order, and both may die in it.
- No resolution produces a configuration error, a runtime error or a non-zero exit code. Combat is normal operation.

## Constraints

- **Integer arithmetic on existing attributes only.** No floating point, no new attribute, no derived stat, no
  intermediate scale. `SPEC-MOK-001`'s attributes are `0..=100` and stay so.
- **No entropy, at all.** This is the constraint the additivity property rests on for `baseline`, and it is not traded
  for variety in outcomes.
- **The function is stated in `SPEC-MOK-001`, not chosen by the implementation.** Its form and its constants are part of
  the specification, so that a change to how hard Mokiterions hit is a governed change and not a tuning commit.
- **No population aggregate is read**, including no count of living Mokiterions and no count of ongoing encounters.
  `REQ-MOK-059` states this generally.
- **No damage is dealt outside a resolved `attack` or `fight`.** `threaten` deals none (`REQ-MOK-055`), `surrender` deals
  none (`REQ-MOK-056`), `approach` and `avoid` deal none, and there is no collision damage, no passive attrition between
  Mokiterions in contact, and no area effect.
- **The new rule is appended after `SPEC-MOK-001` rule 19**, not inserted beside rule 12, on `WO-MOK-010`'s stated
  precedent that insertion renumbers rules cited across specifications, verification contracts, records, retained
  evidence and source comments.

## Acceptance examples

### Example: normal behavior

**Given** two living Mokiterions in contact, the striker at a known `energy` and `health`

**When** the striker's valid `attack` resolves

**Then** the target's `health` falls by the amount the specified function yields for those inputs, the striker's `energy`
falls by the stated cost, and one event reports both transitions and the damage.

### Example: determinism

**Given** the same seed, ticks, policy and density

**When** a run is repeated

**Then** every resolution deals the identical damage at the identical tick, and the run's whole output is byte-identical.

### Example: a fight answers on the same terms

**Given** a Mokiterion attacked in the previous tick that proposes `fight` at its next opportunity

**When** the action resolves

**Then** damage is computed from the *defender's* `energy` and `health` by the same function, and the original attacker's
`health` falls by it.

### Example: death by combat

**Given** a target whose `health` is at or below the damage the resolution yields

**When** the attack resolves

**Then** the target's `health` saturates at zero, it is marked dead through rule 13's existing path with its existing
event, it takes no further opportunity in that tick or any later one, and the resolution event records that it died.

### Example: mutual destruction within one tick

**Given** two living Mokiterions in contact, each low enough on `health` to die to the other

**When** the lower-identified one attacks and kills, and the higher-identified one is already dead

**Then** the survivor's damage stands, the dead Mokiterion takes no opportunity, and no resolution is attributed to it.

### Example: failure behavior

**Given** any run in which combat occurs

**When** the run completes

**Then** no attribute of any Mokiterion is outside `0..=100`, the shared entropy stream stands at the same position
immediately before and immediately after every resolution, and the exit code is `0`.

## Open decisions

- **The damage function's form and the striker's energy cost were decided by the product owner on 2026-08-20** and are
  stated above: `10 + (energy + health) / 10`, and a flat `5` `energy`. They are no longer open. They remain
  `SPEC-MOK-001`'s to state and the product owner's to approve **as specification text**, which is a separate act from
  this requirement's approval; recording the decision here does not perform it. Independently of the values, this
  requirement fixes their inputs (the striker's `energy` and `health`), their type (integer), their floor (at least `1`
  damage) and their arithmetic (saturating), so a later amendment to the constants does not reopen the shape.
- Whether resolution emits one event kind for both `attack` and `fight` or one each **was** the technical owner's,
  constrained by `SPEC-MOK-003` rule 11 and by `SPEC-MOK-002` rule 6's public-interface count. **It was decided on
  2026-08-20: one kind, `attack_resolved`, for both**, because they are one resolution invoked by either party — which is
  what this requirement already states as one function — and a reader distinguishes them by which Mokiterion the event's
  subject is. It is no longer open. The type is authorized by this requirement under `SPEC-MOK-003` rule 11, and it is
  the one place a target's `health` transition is reported, since rule 2 may have emitted the target's own
  `survival_changed` line for the tick before the strike landed.
- Whether an energy precondition should gate attacking at all is deferred. It is a product decision, it would go in
  `REQ-MOK-052`'s precondition list, and `VER-MOK-016`'s measurement of how often exhausted Mokiterions attack is the
  evidence it would be taken on.
