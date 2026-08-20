+++
id = "REQ-MOK-049"
type = "requirement"
title = "Keep the world habitable and combat lethal under the social decision source"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN a simulation runs to 1,000 ticks using the social decision source at the default resource density, THE SYSTEM SHALL leave at least five of the twelve Mokiterions living and SHALL record at least one death attributable to combat, on every declared verification seed."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-009"]
+++

# Requirement: Keep the world habitable and combat lethal under the social decision source

## Rationale

Two failures are possible once Mokiterions can kill each other, and they point in opposite directions. The world can
depopulate, in which case combat has replaced the simulation rather than joined it. Or combat can be inconsequential, in
which case the seven verbs are decoration and `fear` has a reader that never matters. A floor alone catches the first and
would be trivially satisfied by the second — a source that never attacks passes any survivor floor. So this requirement
is deliberately two-sided: a lower bound on survivors and a lower bound on lethality, both on every declared seed.

**Five of twelve.** `REQ-MOK-014` binds `reference` at eight of twelve at the default density, and `REQ-MOK-034` binds the
trait-aware source at the same eight. Five says that combat may cost this world three more Mokiterions than trait
individuality does, and no more.

**The floor was set at five rather than six, in advance of any measurement, by the product owner on 2026-08-20.** What
moved it is `REQ-MOK-044`'s damage decision taken in the same session: a range of `10..=30` kills a full-health target in
four to ten strikes, and a flat `5` `energy` cost does not stop a striker pressing an attack through to a kill. Six was
drafted before those values existed, and the owner's judgment is that a floor drafted against an unknown damage function
should not become the thing that forces the damage function to be re-decided.

**What moving it costs is stated rather than glossed.** Six was half the population, and the draft argued from that: below
half, the run stops being a society under pressure and becomes a cull, because with five or fewer alive at tick 1,000
contact at the rate `REQ-MOK-032`'s arithmetic gives is rare enough that the survivors are back to being solitary
foragers — the state this whole initiative exists to leave. **Five sits exactly on that line.** This obligation is
therefore weaker than the argument that produced six, and it stands as the owner's decision rather than as a figure
derived from the world's own arithmetic. Whether five was the right place is settled by the measured curve, not by this
text: `VER-MOK-012` records survivors per seed whether or not the floor is met, so the evidence will show how much
headroom five actually bought, and a curve that clusters at five or six is itself a finding about the damage decision.

**At least one death by combat.** `INT-MOK-009` states that an attack which cannot kill is a gesture, and
`REQ-MOK-044` makes death by combat reachable. Reachable is not reached. This obligation is what distinguishes a
mechanism that exists from a mechanism that happens, and it is stated per seed rather than in aggregate so that it cannot
be satisfied by one violent seed among many. `REQ-MOK-044` obliges the stream to distinguish death by combat from death
by starvation or exhaustion, which is what makes this countable from a run's own output.

**The floor stated here is provisional in a specific and precedented sense.** `REQ-MOK-014`'s amendment record shows what
happens to an unmeasured floor: its original absolute form measured zero survivors on every declared seed and had to be
restated, and its later floors were selected from a curve measured before a correction that invalidated them. This floor
is stated before any measurement exists, because a work order cannot be verified against an obligation that does not yet
exist — and it is expected to be ratified or amended by the product owner on the first measured curve, with the amendment
recorded in this requirement's own amendment record. What is **not** provisional is the shape: a two-sided bound, per
seed, at the default density.

It has already moved once, from six to five, before any measurement. That move carries **no amendment-record row**, and
deliberately: this requirement is still `draft`, so the change was an edit to an unapproved obligation rather than an
amendment to an approved one. The amendment record begins at the approval this requirement is given, and every movement
of this floor after that belongs in that record.

**One thing this requirement must not become.** A survivor floor is a population aggregate. It is read by the
*verification* path and by nothing inside the simulation — no rule, no source and no validation of a proposal may consult
it or anything like it. `REQ-MOK-050` states that separation, and it exists partly because this requirement makes the
temptation concrete: the easy way to meet a survivor floor is to have the engine stop combat when the population gets
low, and that is exactly `README.md`'s `if food < threshold: start_war()` with the sign reversed.

## Preconditions and trigger

The trigger is a completed 1,000-tick run at the default resource density under the `social` decision source of
`REQ-MOK-048`, on a seed in the declared verification set.

The obligation is stated at the default density only, on `REQ-MOK-014`'s reasoning: a higher density would document
headroom, and headroom is worth measuring as evidence rather than binding as an obligation.

## Required response

- **At least five of the twelve Mokiterions are living at tick 1,000**, on every declared verification seed, at the default
  density, under this source.
- **At least one death in the run is attributable to combat**, on every declared verification seed, distinguishable in the
  event stream from death by starvation or exhaustion under `REQ-MOK-044`.
- **Both bounds hold simultaneously on each seed.** A seed that depopulates and a seed that is peaceful are both failures,
  and a matrix that trades one against the other does not satisfy this requirement.
- **The floor is met by the behavior of the `social` source, not by weakening any rule.** No damage constant, no forfeit
  amount, no survival constant and no resource constant is tuned to reach this floor without the change being a governed
  amendment with its own approval. In particular, `REQ-MOK-014`'s and `REQ-MOK-034`'s floors must still hold after any
  such change, and `baseline` must still reproduce byte-identically.
- **The `social` source's own ordering and thresholds are the one legitimate lever**, because they are that source's
  behavior rather than a rule's constant. `REQ-MOK-048`'s survival-first ordering — a Mokiterion acts socially only when it
  is neither hungry nor tired — is what this floor rests on, given that `REQ-MOK-044`'s `5` `energy` cost brakes combat
  weakly. Changing those thresholds is a change to `REQ-MOK-048` and is governed there; it is not a tuning commit, and it
  is not forbidden by the bullet above.
- **The measured curve is recorded as evidence**, on the pattern of `WO-MOK-002`'s `density-curve.md`, including survivors
  per seed, deaths by cause per seed, and the count of each targeted action proposed and applied. The evidence is what the
  product owner ratifies the floor on.

## Failure and boundary behavior

- A run with zero contacts on some seed fails the lethality bound. The remedy is a matrix that reaches contact or a source
  that seeks it, not an exemption.
- A run in which every Mokiterion dies fails the survivor bound whatever the cause of death, including starvation. This
  requirement does not distinguish causes on the survivor side; it binds habitability, and a world made uninhabitable by a
  combat-driven collapse of foraging behavior fails it as squarely as one where they killed each other.
- A run at a non-default density is outside this requirement's obligation. It may be captured as evidence.
- A run under `baseline`, `reference` or `individual` is outside this requirement. `REQ-MOK-014` and `REQ-MOK-034` bind
  those, and both are re-measured under this initiative because a change to the resource composition of the world moves
  them.
- Fewer than twelve living Mokiterions at initialization is not a reachable state; the population is fixed by
  `SPEC-MOK-001`.

## Constraints

- **No population aggregate is read inside the simulation** to satisfy this requirement, and nothing about a survivor count
  is available to any rule, source or proposal validation. `REQ-MOK-050`.
- **No entropy change.** Meeting this floor must not be achieved by moving the shared stream's consumption, which would
  diverge every pre-existing run.
- **Meeting this floor must not be achieved by any change to the `baseline` source.** Its outcomes are held
  byte-identical by this initiative. This is deliberately narrower than `REQ-MOK-034`'s existing clause, which names
  `reference` too; that clause is amended by `REQ-MOK-051`, which moves `reference` by construction.
- **The default density and the default policy are unchanged.** The floor is measured at the default density under an
  explicitly selected policy.
- The declared verification seed set is the existing one. This requirement does not introduce a seed set of its own, so
  that its results are comparable with `REQ-MOK-014`'s and `REQ-MOK-034`'s at matched seeds.

## Acceptance examples

### Example: normal behavior

**Given** the default density and the `social` decision source

**When** a 1,000-tick run completes on each declared verification seed

**Then** each run leaves at least five Mokiterions living and records at least one death attributable to combat.

### Example: the lethality bound bites

**Given** a source that reads `fear` but never proposes `attack` or `fight`

**When** the matrix runs

**Then** every seed leaves twelve survivors and every seed fails this requirement.

### Example: the habitability bound bites

**Given** a damage function large enough that encounters are usually fatal

**When** the matrix runs

**Then** seeds fall below five survivors and fail, and the remedy is a governed amendment to the specification's damage
constants rather than a change to the verification threshold.

### Example: comparability

**Given** the same declared seeds under `reference`, `individual` and the `social` source at the default density

**When** all three matrices run

**Then** the survivor counts are directly comparable, and the difference between the `social` source and `individual` is the
measured cost of combat.

### Example: failure behavior

**Given** any declared seed that leaves fewer than five living, or that records no death attributable to combat

**When** the verification matrix is evaluated

**Then** the requirement is not met, and the run's exit code is `0` regardless — habitability is a measured property, not a
runtime error.

## Open decisions

- **The floor's value is the product owner's to ratify or amend on the first measured curve.** Five is stated so that the
  work order has an obligation to verify against; `REQ-MOK-014`'s amendment record is the precedent for expecting the
  number to move once evidence exists, and for recording the move here rather than silently in a verification contract.
  It has already moved from six to five once, before measurement and while `draft`, for the reason recorded in the
  rationale — which means the ratification on the first curve is now a check on a number the owner chose against a known
  damage function rather than against no information at all.
- Whether the lethality bound should be stronger than "at least one" — a rate rather than an existence claim — is deferred
  until the first curve exists. An existence claim per seed is the weakest form that cannot be satisfied by an inert
  mechanism.
