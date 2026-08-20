+++
id = "REQ-MOK-048"
type = "requirement"
title = "A fourth decision source that reads fear and proposes the targeted actions"
status = "approved"
owners = ["technical owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHERE the operator selects the social policy value, THE SYSTEM SHALL use a deterministic decision source that reads the observing Mokiterion's fear and suffered attacks, may propose any of the eleven action kinds, and delegates every non-social decision to the trait-aware source unchanged, and THE SYSTEM SHALL leave the baseline, reference and individual sources proposing exactly what they propose today."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: A fourth decision source that reads fear and proposes the targeted actions

## Rationale

The seven verbs of `REQ-MOK-043` are unreachable until some source proposes them, and `fear` stays inert until some
source reads it. This requirement is the reader, and it is a **fourth** source rather than a change to any existing one
because each of the three existing sources fails a different test.

**`baseline` cannot gain a verb.** It selects uniformly among valid actions with a single entropy draw, so widening its
candidate list changes the size of the set that draw indexes into. Every subsequent selection in every pre-existing
baseline run would move. That is the additivity property `WO-MOK-010` protected when it declined entropy substreams, and
it is the one source in this initiative that can be held byte-identical. It is held.

**`reference` must not gain one.** It is documented in `--help` as "a deterministic development instrument, not
autonomous behavior" and `REQ-MOK-034` names it and `baseline` as the sources whose outcomes are frozen. Giving the
reference instrument social behavior would make the instrument the thing it exists to measure against.

**`individual` could gain one, and gaining one would hide the result.** It seeks and consumes as `reference` does except
for trait-derived bias, and `REQ-MOK-034` binds its survivor floor. Folding combat into it would mean any change in its
survivor count is a mixture of two causes — the verbs and the traits — with no way to attribute it. A fourth source
leaves `individual`'s floor a control against which the `social` source's floor under `REQ-MOK-049` can be read.

There is also a claim that only a fourth source can make honestly. `CAP-MOK-009` promises that `fear` stops being inert.
A source that reads `fear` is the whole of that promise: `baseline` reads nothing, so combat placed behind `baseline`
would leave `fear` exactly as inert as it is today while appearing to discharge rule 12's closing sentence. The reader
has to be a source that reads.

## The decided source

**Its name is `social`**, decided by the product owner on 2026-08-20 from four candidates. It sits beside `baseline`,
`reference` and `individual`, which name a source's character rather than its behavior, and it leaves room for the
cooperative verbs `CAP-MOK-009` defers to Phase 3.2 without a later rename of a value that appears in every captured
stream.

**Its ordering, decided by the technical owner on the same date, is survival first and then `fear`.** It returns the first
applicable branch:

1. **An unanswered attack is answered.** Where the suffered-attack record of `REQ-MOK-045` is non-empty, the source answers
   the **first** attack in it — the record is in resolution order, so this is deterministic — and chooses by its own `fear`:
   `surrender` at `fear` `60` or above, `retreat` at `30` or above, and `fight` below `30`.
2. **Survival comes before society.** Where rule 19 case 1 or case 2 applies — a tolerated co-located resource, or `energy`
   below `REFERENCE_SLEEP_THRESHOLD` — the source proposes exactly what rule 19 proposes, which is `eat` or `sleep`.
3. **A Mokiterion in contact is engaged.** Where a living Mokiterion is at Chebyshev distance `1` or less, the source
   proposes `attack` while its own `fear` is below `30` and `threaten` otherwise, against the nearest such Mokiterion and
   the lowest identifier among ties.
4. **A Mokiterion merely perceived is closed on or avoided.** Where a living Mokiterion is perceived at distance `2` or
   more, the source proposes `approach` while its own `fear` is below `30` and `avoid` otherwise, on the same tie-break.
5. **Otherwise the trait-aware source decides.** The source proposes exactly what rule 19 proposes, which at this point is
   its case 3 seek-move or its case 4 search step.

**It introduces no survival constant of its own.** Branch 2's condition is rule 19's own cases 1 and 2, and branches 2 and
5 delegate to rule 19 rather than reimplementing it, so a Mokiterion under `social` that never meets another behaves
exactly as it would under `individual`. That is the fallback the technical owner chose over `reference`'s: it means the
twelve differ from one another when alone as well as in conflict, and the trait variation of `CAP-MOK-006` compounds with
the `fear` variation this capability adds. It also yields a differential oracle that costs nothing to check — on any
observation where no Mokiterion is perceived and no attack is unanswered, `social` and `individual` must propose the
identical action.

Delegation rather than a shared generalized implementation is deliberate, on the precedent `simulation.rs` already records
for rule 19: `fits` and its selectors were left as they were rather than generalized to take a tolerance, so that the
reference source's behavior could not come to depend on code written for the trait-aware one. The same reasoning applies
one step further out, and it runs the same way: `social` depends on rule 19, and rule 19 depends on nothing new.

**The three new thresholds are `60`, `30` and `30`, and they compose with `REQ-MOK-046` by design.** A threat adds `30`
`fear`, so a single threat is exactly enough to move a calm Mokiterion from branch 3's `attack` to its `threaten`, and from
branch 4's `approach` to its `avoid`. Two threats, or a threat and a perception, reach `60` and make a defender surrender
rather than fight. This is the mechanism by which `threaten` changes what another Mokiterion decides, which is the claim
`REQ-MOK-046` exists to make good.

**Where in the tick the `fear` it reads comes from.** Rule 3 builds the observation at the acting Mokiterion's own turn, and
rule 12's `fear` write happens at that Mokiterion's own turn as well. The value this source reads is therefore the one
standing after the previous tick's rule 12 write, plus any threat applied to it by an earlier-acting Mokiterion in the
current tick. The `SPEC-MOK-001` amendment must state that position explicitly, because every threshold above is sensitive
to it and a reader should not have to derive it from rule ordering.

**The entropy discipline follows from the ordering and is stated as a consequence, not a rule.** Branches 1, 3 and 4 return
without any draw. Branch 2 returns `eat` or `sleep`, which rule 19 reaches without a draw. Only branch 5 can draw, and it
draws exactly what rule 19 would: at most one selection, from the single shared stream, in rule 19's own order. So
`social` takes **at most one draw per opportunity, and never for a social decision** — which is what keeps `REQ-MOK-044`'s
no-entropy obligation clean and makes the stream-position oracle of `VER-MOK-012` checkable either side of every
resolution.

This is also why branch 2 tests rule 19's cases 1 and 2 directly instead of delegating and inspecting the result. A full
delegation would reach rule 19's case 4 and **consume a draw the source then discarded** when a social branch fired
afterwards, silently moving the shared stream for a decision that was never used. The ordering above cannot do that, and
the implementation may not restructure it in a way that can.

A consequence worth recording: because branches 3 and 4 pre-empt rule 19's cases 3 and 4, a `social` run consumes **fewer**
draws than an `individual` run on the same seed, and diverges from it at the first opportunity where a Mokiterion is
perceived. That is different world evolution rather than a changed draw discipline, and it is the reason the differential
oracle above is stated per observation rather than as a whole-run comparison: a whole-run equality holds only in a
constructed world where no Mokiterion ever perceives another, which at twelve Mokiterions and a perception radius of `16`
is not a reachable run.

## Preconditions and trigger

The trigger is `--policy` naming `social` on a run, and thereafter each living Mokiterion's decision opportunity
under `SPEC-MOK-001` rule 2.

The default policy is unchanged. A run with no `--policy` uses `reference`, exactly as today.

## Required response

- **`--policy` accepts `social`**, and its usage text, its long help and its invalid-value diagnostic all name the four
  values. The diagnostic remains a configuration error with the exit code `SPEC-MOK-002` already fixes for one; no new exit
  code is introduced.
- **The run reports which source was selected**, through the existing `decision_source_selected` event, carrying the new
  source's name.
- **The source reads, at each opportunity, the observing Mokiterion's own `fear` and its suffered-attack record**, both
  carried by `REQ-MOK-045`, alongside the `health`, `satiety`, `energy`, position, perceived food and perceived
  Mokiterions the observation already carries.
- **`fear` changes at least one proposal.** Two Mokiterions identical but for `fear` must be able to propose differently
  under this source. This is the obligation that makes `fear` non-inert and it is stated as an observable difference
  rather than as a promise about the code.
- **The source may propose any of the eleven action kinds**, and over the declared verification matrix it proposes each of
  the seven new ones at least once. A verb no source ever proposes is a rule with no reachable path, which is the inert
  value problem of `SPEC-MOK-003` rule 4.5 reached through the action contract instead of the state model.
- **A Mokiterion carrying an unanswered attack always answers it**, by branch 1, choosing `surrender` at `fear` `60` or
  above, `retreat` at `30` or above and `fight` below `30`, against the first attacker in the record. The draft of this
  requirement reserved the freedom not to answer and stated that no rule forces a branch at a given `fear`; the technical
  owner exercised that freedom in the opposite direction on 2026-08-20, and this bullet records the decision rather than the
  latitude. What remains true is that answering is the *source's* choice and not the engine's: nothing in `SPEC-MOK-001`
  compels an answer, a different source may ignore an attack entirely, and `REQ-MOK-049`'s measurement is what this one is
  held to.
- **The ordering of `REQ-MOK-048`'s five branches is normative**, not illustrative. An implementation may not reorder them,
  and in particular may not test branch 2 by delegating to rule 19 and inspecting the result, because that consumes a draw
  it may then discard.
- **Branches 2 and 5 delegate to the trait-aware source unchanged.** On any observation carrying no perceived Mokiterion
  and no unanswered attack, `social` proposes what `individual` proposes, action for action. This is checkable per
  observation and is `VER-MOK-012`'s cheapest oracle on this requirement.
- **The source takes at most one entropy draw per opportunity, and never for a social decision.** Branches 1, 3 and 4 draw
  nothing; branch 2 draws nothing; branch 5 draws exactly what rule 19 draws.
- **The source is deterministic.** Given the same observation and the same entropy state it proposes the same action. Any
  entropy it draws is drawn from the single shared stream, in a stated order, and only at opportunities where the source
  genuinely selects among candidates.
- **`baseline`, `reference` and `individual` propose exactly what they propose today.** Not "similar" and not "equivalent":
  byte-identical output on the declared matrix for `baseline`, and for `reference` and `individual` identical but for the
  divergence `REQ-MOK-051` causes and attributes.
- **Traits still apply, and now more strongly than the draft intended.** Branches 2 and 5 *are* rule 19, so a Mokiterion's
  derived trait under `REQ-MOK-033` governs every non-social decision `social` makes. Trait derivation continues to run at
  agent creation for every policy, from the side generator, so it cannot move the shared stream — `simulation.rs` already
  carries a test asserting exactly that, and no new derivation is introduced.

## Failure and boundary behavior

- An invalid `--policy` value is a configuration error naming the four accepted values, with the existing exit code, and
  no simulation runs.
- `social` never proposes an action whose precondition it can see is unmet — but where it does, the engine rejects
  it under `REQ-MOK-043` and the run continues. The source has no authority to validate and is not trusted to.
- A Mokiterion with an empty suffered-attack record never has `fight`, `retreat` or `surrender` accepted, whatever the
  source proposes.
- A world in which no contact ever occurs under this source is a valid run producing valid output. The verification matrix
  is responsible for containing seeds where contact occurs; the source is not responsible for forcing it.
- A dead Mokiterion is not consulted, per rule 13.

## Constraints

- **One new `Policy` variant, one new source, and the interface growth is counted** under `SPEC-MOK-002` rule 6.
- **Three new constants and no more**: the answer thresholds `60` and `30`, and the engagement threshold `30`. No new
  survival constant, no new hunger or fatigue threshold, no duplicate of `REFERENCE_SLEEP_THRESHOLD`. Branch 2 reuses rule
  19's own conditions.
- **Contact is derived from the observation's existing `distance` field** on each perceived Mokiterion, which rule 3
  already carries. This source needs no new observation field for contact, and the two fields `REQ-MOK-045` adds are the
  only growth of the observation in this chain.
- **`social` depends on rule 19; rule 19 depends on nothing new.** The dependency runs one way, on the precedent
  `simulation.rs` records for why rule 5's helpers were not generalized for rule 19's benefit. No existing source's code
  path is altered to serve this one.
- **No population aggregate is read.** Not a survivor count, not a census by territory, not the number of ongoing
  encounters, not another Mokiterion's attributes beyond what the observation carries. This is `README.md`'s central
  prohibition and `REQ-MOK-050` states it as a checkable obligation across every source, not only this one.
- **No new dependency.** The engine's dependency table stays empty.
- **The source reads only its own observation.** It holds no state between opportunities, no memory of prior ticks, and no
  handle to authoritative state.
- **No entropy substream.** Any draw comes from the single shared stream, for Phase 2's reason.
- **The default policy and the default density are unchanged.**
- **The source's decision rule is documented in `SPEC-MOK-001`** to the same standard as `reference`'s and `individual`'s,
  in a rule appended after rule 19.

## Acceptance examples

### Example: normal behavior

**Given** a run with `--policy social`

**When** the run starts

**Then** the `decision_source_selected` event names `social`, and over the run the event stream contains at least
one instance of each of the seven targeted actions across the declared matrix.

### Example: the fallback is the trait-aware source

**Given** an observation carrying no perceived Mokiterion and an empty suffered-attack record

**When** `social` and `individual` are each consulted on it

**Then** both propose the identical action, and if that action is rule 19's search step both consume one draw from the same
position.

### Example: a threat changes what the target decides

**Given** a Mokiterion at `fear` `10` in contact with another, which would propose `attack` at its next opportunity

**When** it is threatened once, raising its `fear` to `40` under `REQ-MOK-046`

**Then** at that opportunity it proposes `threaten` instead of `attack`.

### Example: fear is read

**Given** two runs whose worlds differ only in one Mokiterion's `fear` at one opportunity

**When** that Mokiterion is consulted

**Then** the proposed actions differ.

### Example: the other sources are untouched

**Given** the declared verification matrix of seeds, ticks and densities

**When** each run is repeated under `baseline` before and after this change

**Then** every output is byte-identical, and under `reference` and `individual` every difference is attributable to
`REQ-MOK-051` and to nothing else.

### Example: determinism

**Given** the same seed, ticks, policy and density under `social`

**When** the run is repeated

**Then** the output is byte-identical.

### Example: failure behavior

**Given** `--policy` with an unrecognized value

**When** the command runs

**Then** it reports a configuration error naming the four accepted values, runs no simulation, and exits with the code
`SPEC-MOK-002` fixes for a configuration error.

## Open decisions

All three decisions this requirement left open have been taken, on 2026-08-20. They are recorded here and stated in full
under *The decided source*; each remains `SPEC-MOK-001`'s to state as specification text, which is a separate act.

- **The name is `social`**, by the product owner, from four candidates. It appears in `--help`, in the invalid-value
  diagnostic and in every captured event stream, so it was not a detail.
- **The decision rule is the five-branch ordering above**, by the technical owner, with the defender answering at `fear`
  `60` and `30` and the aggressor engaging below `30`. It is bounded by `REQ-MOK-049`'s floor and by `VER-MOK-012`'s
  degeneracy measurement.
- **The source draws entropy only through rule 19's own search step**, by the technical owner: at most one draw per
  opportunity, from the shared stream, never for a social decision.

What remains open:

- **Whether the observation's list of currently valid proposals grows to include the targeted actions** **was** the
  technical owner's, under `SPEC-MOK-002` rule 6's interface count. This source does not need it: branch 3 and branch 4
  read `distance` from the perceived list, and the engine validates under `REQ-MOK-043` regardless. **It was decided on
  2026-08-20: the list does not grow**, so this source proposes targeted verbs the observation never lists and
  `SPEC-MOK-001` rule 6 judges them at application.

  The decision had only two live options rather than three, and the engine is why. `baseline` selects with
  `choose_index` over the length of that list, so growing it for every source moves `baseline`'s single draw and diverges
  every pre-existing `baseline` run, which `CAP-MOK-009` excludes outright. The live alternative was growing it only
  under `--policy social` — safe for `baseline`, since one source is selected for a whole run — and the owner declined it
  so that the observation stays a function of world state alone rather than of the selected policy. **The cost is
  recorded rather than glossed**: the observation's list stops enumerating everything a source may legitimately propose,
  so a reader who takes it as the complete contract will be wrong about this source, and rule 6 is the only complete
  statement.
- **Whether branch 1 should be able to decline to answer** is deferred to the first measured curve. If `VER-MOK-012` finds
  the branch distribution degenerate — every defender surrendering, or every defender fighting — that is the evidence a
  change would be taken on, and `INT-MOK-009` records the degenerate case as a finding to report rather than a failure to
  hide.
