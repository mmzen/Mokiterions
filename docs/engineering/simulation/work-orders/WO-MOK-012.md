+++
id = "WO-MOK-012"
type = "work_order"
title = "Let Mokiterions act on each other: contact, seven targeted actions, a defender's own answer, a source that reads fear, and the resource composition corrected"
status = "draft"
owners = ["engineering owner"]
created = "2026-08-20"
updated = "2026-08-20"

[assurance]
commit_bound_verification = "required"
rationale = "This is the largest behavioral change since WO-MOK-001 and the first in which one Mokiterion's action writes another Mokiterion's state. Three of its load-bearing claims cannot be asserted, only checked against captures bound to the commit the work began from: that the baseline source's output is byte-identical, that the reference and individual sources' divergence is attributable to the corrected waste condition alone, and that no resolution moves the shared entropy stream. It also carries two claims of absence — that nothing reads a population aggregate, and that the acting order confers no systematic advantage on low identifiers — and an absence verified by the effort that created the capability is worth little without an independent record. Finally it re-opens REQ-MOK-014's and REQ-MOK-034's measured survivor floors, which no self-assessment may retire."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-042", "REQ-MOK-043", "REQ-MOK-044", "REQ-MOK-045", "REQ-MOK-046", "REQ-MOK-047", "REQ-MOK-048", "REQ-MOK-049", "REQ-MOK-050", "REQ-MOK-051"]
specifications = ["SPEC-MOK-001", "SPEC-MOK-002", "SPEC-MOK-003"]
verification = ["VER-MOK-012"]
+++

# Work Order: Let Mokiterions act on each other

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope below.
Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the completed
change and the retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

**This work order cannot be approved before its governing artifacts are.** It depends on `INT-MOK-009` and `CAP-MOK-009`
and on `REQ-MOK-042` through `REQ-MOK-051` being approved by their accountable owners — the product owner for
`REQ-MOK-042`, `043`, `044`, `046`, `047`, `049` and `051`, the technical owner for `REQ-MOK-045` and `REQ-MOK-048`, and
the assurance owner for `REQ-MOK-050` — and on `VER-MOK-012` being approved by the assurance owner. All of those roles are
held by the repository owner, which means every approval is a separate act by the same person and **none is implied by
any other**.

**The repository owner validated this chain and authorized its transition on 2026-08-20, and the transition could not be
completed.** `INT-MOK-009` and `CAP-MOK-009` moved to `approved` on that authorization, nothing being outstanding above
them. The ten requirements, `VER-MOK-012` and this work order did not, for a reason the harness states and this packet
had not accounted for: `validate` rejects an `approved` requirement that no active specification `specifies` — ten
`E007` errors, one per requirement — and `preflight --phase start` refuses the same chain under `W016`, "specification
coverage is missing". `docs/engineering/WORKFLOW.md` step 3 puts specification before the work-order approval of step 4,
and `SPEC-MOK-001`'s amendment record shows the practice: the individuality amendment was approved **together with**
`REQ-MOK-031` through `REQ-MOK-034`, `VER-MOK-010` and `WO-MOK-010`, in one act on 2026-08-19.

**The specification amendments below are therefore an approval precondition of this work order and not a task inside
it.** The ten requirements are approved with them or not at all, and the drafting order this packet used — requirements
and work order first, amendment text under the work order — was wrong in exactly that respect. What the owner's
validation covers is the text as it stood, and nothing else: the three decisions recorded below were **not** supplied
with it, and none of them was taken on its authority. They were put to the owner separately once the ordering was found,
and answered on the same date, before any amendment text existed. An approval is not a licence to choose them.

**An architecture review is required and is not waived; its form is decided.** No active architecture carries an
`addresses` edge to any requirement in this chain: `ARCH-MOK-001` addresses `REQ-MOK-004`, `008`, `009`, `010` and `016`,
and `ARCH-MOK-002` addresses `REQ-MOK-021`, `025`, `026` and `028`. No `architecture` relation is therefore declared, and
fabricating coverage would be worse than omitting it. But this change introduces a pattern the engine has never had — a
rule that writes a Mokiterion other than the acting one — so `ARCH-MOK-001`'s component boundaries, its prohibition on any
public item exposing a mutable borrow into authoritative state, and its prohibition on any engine dependency are all in
force.

**The technical owner decided on 2026-08-20 that this is a recorded confirmation that `ARCH-MOK-001` requires no
amendment, and not a new `ADR-MOK-005`.** The reasoning is that cross-agent mutation stays inside the engine component,
crosses no boundary `ARCH-MOK-001` draws, introduces no dependency, and reaches no public item as a borrow — so there is
no architectural decision left for an `ADR` to record. Deciding the form does not discharge the review: the confirmation
is made against the implemented code at approval and recorded as `VER-MOK-012` manual assessment 9, an unrecorded
confirmation is an outstanding assessment, and if the implementation turns out to need a boundary moved then the decision
reverts to an `ADR` and the assessment says so.

### Why this chain is numbered 009, 042 through 051, and 012

The identifier space is shared across branches and sessions in this repository, and `WO-MOK-011` records the owner
resolving a collision once already: `evidence/WO-MOK-007/renumbering.md` states that renaming approved artifacts is the
owner's act and not an implementation agent's.

The numbers here were therefore chosen by reading every ref, not the local maximum. Across all local and remote branches
the highest artifact of each family in use is `INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-041`, `SPEC-MOK-005`,
`ARCH-MOK-002`, `ADR-MOK-004`, `VER-MOK-011`, `WO-MOK-011`, `VREC-MOK-011`, `REL-MOK-001` and `RLS-MOK-001`. This chain
takes the next number above each: `INT-MOK-009`, `CAP-MOK-009`, `REQ-MOK-042` through `REQ-MOK-051`, `VER-MOK-012`,
`WO-MOK-012`, and `VREC-MOK-012` when the verification record is written. `ADR-MOK-005` was reserved for the architecture
decision and is **not** used, because that decision was taken as a confirmation instead; the number stays free for
whatever needs it next, and the survey above is what a later chain should repeat rather than trust.

Two notes on what that survey found, recorded so a later reader does not have to repeat it. `VER-MOK-009` does not exist
as an artifact on any branch, so the verification family has a gap at `009` that this chain does not fill and should not.
And a `git grep` for `VER-MOK-0NN` across refs reports a maximum of `034`, which is not an artifact: it is a typographic
citation of `REQ-MOK-034` inside `evidence/WO-MOK-010/completion-summary.md`, corrected on the
`bugfix/documentation-corrections` branch. A survey by grep alone would have numbered this contract `VER-MOK-035`.

### Decision record

On 2026-08-20 the repository owner, acting as product owner and technical owner, took the decisions this packet was
drafted around, in an interactive session. They are design decisions, not approvals.

| Decision | Outcome | Role |
|---|---|---|
| Scope of the first work order | Phase 3.1 only: contact, the seven verbs, the deferred answer, the `social` source, both carried floors re-measured, and the resource composition corrected | product owner |
| How a defender answers | Damage resolves immediately under engine authority; `fight`, `retreat` and `surrender` are the defender's own proposals at its own next opportunity, enabled by an attacks-suffered field on the rule 3 observation | technical owner |
| How resolution is computed | Deterministically, as an integer function of the striker's `energy` and `health`, with no entropy draw | technical owner |
| What counts as contact | Chebyshev adjacency, radius `1`, including co-location | product owner |
| The striker's first-move advantage | Bounded by verification, not compensated for by a rule. No reordering, no initiative roll, no first-strike penalty | product owner |
| What surrender costs | A forfeit of `satiety` to the Mokiterion surrendered to, capped at the recipient's maximum with the excess destroyed | product owner |
| What threatening does | Raises the target's `fear`, and nothing else. No damage, no cost | product owner |
| Where the high-class accumulation effect is addressed | Inside this work order's measurement, so both carried survivor floors are re-measured once against a world that has combat and corrected composition together | product owner |
| Which source proposes the new verbs | A fourth decision source, which is also the thing that reads `fear`. `baseline`, `reference` and `individual` gain no verb | product owner |

Three of these were reached against alternatives that were considered and declined, and the reasoning is recorded because
it constrains the implementation.

**The defender's answer is deferred rather than resolved inside the attack.** Resolving a defender's response inside the
attacker's turn would have made the most consequential moment in this world a computation the engine performs, and
Phase 5's model-backed source would arrive to find it already decided. The cost of deferring it is an asymmetric latency
— zero ticks when the defender's identifier is above its attacker's, one tick when below — which is the same within-tick
asymmetry rule 12 already documents for `fear`, and which is stated rather than corrected.

**The fourth source is a fourth source and not a change to `individual`.** Folding combat into the trait-aware source
would have made any movement in its survivor count a mixture of two causes with no way to attribute it, and would have
put social behavior into a source `REQ-MOK-034` freezes. A fourth source leaves `individual`'s floor as a control.

**The composition correction is placed in the sources' waste condition, not in the world.** Correcting rule 16's uniform
class selection, rule 15's amount, rule 9's eat effect or the food table would change what a `baseline` run does and
diverge every pre-existing baseline capture, and rule 16 would move the shared stream's draws as well. Rule 5's and rule
19's waste condition is `reference`'s and `individual`'s proposal logic; `baseline`'s rule 4 candidate list applies no
waste condition at all, so no change there can reach it. That is what makes `INT-MOK-009`'s promise of a byte-identical
`baseline` keepable, and `REQ-MOK-051` fixes the permitted set for that reason.

### The values decided after the packet was drafted

The nine decisions above fixed the shape. They left ten values unstated, and this work order's first draft listed them as
decisions outstanding before approval. The repository owner took all ten later on **2026-08-20**, in a second interactive
session, against the drafted packet. Each is now stated in the requirement that owns it, and each is reproduced here
because a work order that names a value in an amendment provision has to carry the value.

| Decision | Outcome | Role |
|---|---|---|
| The damage function | `10 + (striker.energy + striker.health) / 10`, the division truncating toward zero — range `10..=30`, four to ten strikes to kill a full-health target | product owner |
| The striker's energy cost | A flat `5` per strike, not scaled with the damage dealt | product owner |
| The forfeit's magnitude | `satiety / 2`, truncating — a proportion, not a flat amount | product owner |
| The threat's `fear` increase | A new constant of `30`, distinct from rule 12's `FEAR_INCREASE` of `10` and not a pin to `ATTRIBUTE_MAX` | product owner |
| The fourth source's name | `social`, from four candidates | product owner |
| The source's decision rule | Survival before society: five ordered branches, answering an unanswered attack first, then rule 19's cases 1 and 2, then contact, then perception, then rule 19's remainder | technical owner |
| Its thresholds | `surrender` at `fear` `60`, `retreat` at `30`, `fight` below `30`; engage at `attack` and `approach` below `30`, at `threaten` and `avoid` from `30` | technical owner |
| Its fallback where nothing is perceived | `individual`'s, verbatim, by delegating to rule 19 rather than reimplementing it | technical owner |
| `REQ-MOK-049`'s survivor floor | Lowered from six to five, before any measurement, because of the damage decision above | product owner |
| `REQ-MOK-051`'s composition ceiling | Ratified at one half, against `60%` and `40%` | product owner |

Four consequences of these values constrain the implementation and are recorded here rather than left to be discovered.

**The energy cost is deliberately a weak brake, which is what makes the ordering load-bearing.** A flat `5` does not stop
a striker pressing an attack through to a kill, and `REQ-MOK-044` records that the alternative — a cost scaling with the
damage dealt — was declined because it would likely have made `REQ-MOK-049`'s lethality bound unreachable. What holds the
survivor floor up instead is the `social` source's survival-first ordering: a hungry or tired Mokiterion forages rather
than fights. So the floor is squeezed from both ends, and the encounter rate decides it. Neither the cost nor the ordering
may be adjusted by the implementation to relieve the squeeze.

**Delegation is not generalization, and the direction matters.** Branches 2 and 5 call rule 19's existing implementation.
Rule 19's own code gains no parameter, no tolerance argument and no branch written for `social`'s benefit — the precedent
is `simulation.rs`'s own recorded reason for leaving rule 5's `fits` and its selectors alone rather than generalizing them
for rule 19: a shared implementation would make an older source's behavior depend on code written for a newer one. What
this buys is that "the fallback is `individual`'s, verbatim" is true by construction rather than by inspection, and it
yields `VER-MOK-012`'s oracle 8 for nothing.

**The ordering has an entropy trap in it, and the ordering is normative because of the trap.** Branch 2 tests rule 19's
cases 1 and 2 **directly**. It must not delegate to rule 19 and inspect the result: a full delegation reaches rule 19's
case 4, takes a draw from the shared stream, and then discards it when a social branch fires afterwards — silently moving
the stream for a decision that was never used. `REQ-MOK-048` states this and this work order forbids the restructuring
that would reintroduce it.

**Lowering the floor to five contradicts the argument that produced six, and `REQ-MOK-049` says so.** Six was half the
population and the draft argued from that; five sits exactly on the line that argument drew. The obligation is therefore
weaker than its own rationale, it stands as the owner's decision rather than as a figure derived from the world's
arithmetic, and the ratification at `VER-MOK-012` assessment 4 is where that is settled against a measured curve. Because
`REQ-MOK-049` is still `draft`, the move carries no amendment-record row: it was an edit to an unapproved obligation.
Every movement after the approval it is given is an amendment and belongs in its record.

### The three decisions the validation did not supply, taken on the amendment text

The ten values above were the bulk of what this packet left open. Three decisions remained after them, each a stop
condition on the amendment text rather than on the code, because the amendment is what this chain needed next. **The
repository owner took all three on 2026-08-20, acting as technical owner, after the ordering above was found and before
any amendment text was written.** Each is reproduced here for the same reason the ten above are.

| Decision | Outcome | Where it lands |
|---|---:|---|
| The event-type set | **Three new types, one wherever a second Mokiterion's state moves**: `attack_resolved`, shared by `attack` and `fight`; `threat_resolved`; `surrender_resolved` | `SPEC-MOK-001` provision 6, `SPEC-MOK-002` provision 1, `SPEC-MOK-003` provision 1 |
| The observation's valid-proposal list | **It does not grow**; the `social` source proposes targeted verbs the list never carries, and rule 6 validates them at application | `SPEC-MOK-001` provisions 6, 7 and 10, `SPEC-MOK-002` provision 1 |
| Rule 7's clearing order | **The trace line is written before the record is cleared** | `SPEC-MOK-001` provision 11 |

**The event-type decision was the technical owner's and this packet had drafted it as the agent's, which was wrong.**
`SPEC-MOK-002`'s rule 5 enumeration has to be approved before implementation begins and the enumeration names the event
types, so the choice belongs to the owner at the moment the amendment is written. The *Authorized decision envelope*
below is corrected accordingly. What the owner decided against was seven types, one per verb, and one shared
`encounter_resolved` carrying the verb in its detail. The reason given for three is `CAP-MOK-009`'s own driver for new
types — that they carry the state transitions they caused, *including transitions to a second Mokiterion* — which
`approach`, `avoid` and `retreat` do not have: each mutates only the actor and resolves as a rule 8 move, so rule 7's
`action_trace` already reports everything they change. **`REQ-MOK-043` therefore takes no row in `SPEC-MOK-003` rule 11's
authority table**, and provision 1 of that specification's amendment is corrected from the four requirements it named to
three: `REQ-MOK-044` for `attack_resolved`, `REQ-MOK-046` for `threat_resolved`, `REQ-MOK-047` for `surrender_resolved`.
`attack` and `fight` share `attack_resolved` because they are one resolution invoked by either party, which rule 22
states as one function; a reader distinguishes them by which Mokiterion is the subject.

**The valid-proposal decision was constrained to two live options rather than three, and the engine is why.**
`BaselineDecisionSource::decide` selects with `entropy.choose_index(observation.valid_actions.len())`, so growing that
list for every source moves `baseline`'s single draw and diverges every pre-existing `baseline` run — which
`CAP-MOK-009` excludes absolutely and `REQ-MOK-051` holds byte-identical. The live alternative was growing the list only
under `--policy social`, which `--policy` selecting one source for the whole run makes safe for `baseline`; the owner
declined it so that rule 3's observation stays a function of world state alone rather than of the selected policy. **The
cost is recorded rather than glossed**: `Observation::allows` and the `is_consistent` invariant stop describing the whole
proposal contract, since a source may now propose a verb the list does not carry, and rule 6 is the only place that
contract is complete.

**The clearing order follows rule 7's existing principle rather than establishing a second one.** Rule 7 already fixes
that the traced `fear` is the pre-update value, so the traced suffered-attack record is likewise what the source read.
The alternative — clearing first, for uniformity with the `position`, `health`, `satiety` and `energy` that
`emit_action_trace` reads after the action applies — was declined because it would empty the field on exactly the trace
lines it exists to explain, every line where a response was proposed.

Deferred to the first measured curve rather than outstanding at approval, and listed so that the two are not confused:
`REQ-MOK-049`'s floor and `REQ-MOK-051`'s ceiling are ratified or amended on measurement (`VER-MOK-012` assessments 4 and
5); whether a per-class composition floor should accompany the ceiling; whether branch 1 should be able to decline to
answer at all; and whether the lethality bound should become a rate rather than an existence claim. None of these blocks
approval, and all of them are the product owner's or the technical owner's rather than an implementation agent's.

### Required amendments to approved specifications

This work order depends on amendments to three approved specifications and three approved requirements. They are stated
here in full, and each requires its accountable owner's separate act: approving this work order does not approve them.
The implementation agent writes the amended text and records in each amendment record that it did so; it does not decide
the substance.

**The `SPEC-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-003` amendments come first, before this work order is approved.**
Each of them adds the requirements of this chain to its `specifies` relation, which is what makes those requirements
approvable at all — `validate`'s `E007` and `preflight`'s `W016`, recorded above. So the text below is written, then
approved together with `REQ-MOK-042` through `REQ-MOK-051`, `VER-MOK-012` and this work order in one act, and only then
does implementation begin.

The three requirement amendments in this section do not share that ordering, and they do not share each other's either.
`REQ-MOK-005`'s re-reading is text, is written with the specification amendments, and is approved in the same act.
`REQ-MOK-034`'s narrowing of its frozen clause is also text, and must be approved **before** the change that moves
`reference`, but need not precede this work order's approval. `REQ-MOK-014`'s and `REQ-MOK-034`'s survivor floors cannot
be amended in that act at all: re-measuring them requires the change to exist, so they are measured under this work order
and amended afterwards only if the measurement moves them, each on its owner's separate act.

#### The text was written on 2026-08-20, and what writing it exposed

The amended text of `SPEC-MOK-001`, `SPEC-MOK-002`, `SPEC-MOK-003`, `REQ-MOK-005` and `REQ-MOK-034` now exists in those
artifacts, each carrying an amendment-record row marked **OUTSTANDING** and pointing here. Every `status` field is
unchanged; nothing below has been approved by anyone.

Writing it found four defects in this section, and each is corrected in place above rather than silently: provision 1 of
the `SPEC-MOK-002` list stated that `Observation` is a public item, which the source contradicts; provision 6 of the
`SPEC-MOK-001` list omitted the `action_trace` line's shape, which provision 11's decision requires; provision 9 named a
numeric mechanism that `REQ-MOK-051` leaves open on measurement, so it is two amendments and only the first is written;
and the rule 21 bullet placed a tie-break in a rule that selects nothing.

**Eight consequences in the text were derived rather than decided, and they need the owner's ratification with it.**
None was put to an owner, because each follows from something already decided; but each is a statement a reader will
rely on, so each is named here rather than left inside prose:

1. **The `suffered` field is present only when the record is non-empty.** Forced: an unconditional field changes every
   `baseline --trace-actions` line and `CAP-MOK-009` excludes that.
2. **A targeted proposal renders as `proposal:<verb>:<target-id>`** inside the existing field, on the `position:<x:y>`
   precedent, so the trace line's field count does not change.
3. **The field lists of `attack_resolved`, `threat_resolved` and `surrender_resolved`**, including `target_died`,
   `discarded` and `increase`, each present because the transition it reports would otherwise appear in no record.
4. **`avoid` against a co-located target moves north**, then east, south, west, rather than being a rejection.
5. **The uniform field name `target`** across all seven targeted variants.
6. **`fight` carries two preconditions and not one** — contact as well as an unanswered attack in the record.
7. **`agent_died` gains no cause detail**; `attack_resolved`'s `target_died` makes a combat death recoverable without
   changing a released event's shape.
8. **`REQ-MOK-051`'s numeric form is deferred to a second amendment**, per the correction above.

**`SPEC-MOK-001`** — the behavior authority. Thirteen provisions, of which seven are appended rules.

1. *Scope*. No longer excludes social interaction between Mokiterions, and names `CAP-MOK-009`. The exclusion of
   cooperation, memory of encounters and perceived relative strength stays, and is restated as a boundary of this
   capability rather than of the specification.
2. *Actors*. Names four decision sources rather than three, the fourth being `social`.
3. *Inputs* and *Help output*. `--policy` takes `social` as its fourth value. The usage line, the long help and the
   invalid-value diagnostic all name four, and the default stays `reference`.
4. *State model → Mokiterion*. Gains no attribute. It gains one item of **transient per-Mokiterion state**: the record of
   attacks suffered since that Mokiterion's previous decision opportunity. The amendment must state that this is not a
   bounded `0..=100` attribute, that it is not reported as one, that it is cleared by the Mokiterion's own next
   opportunity, and that it is per-Mokiterion and never per-pair.
5. *State model → World*, or a new *Contact* subsection. Fixes the contact relation and its radius constant of `1`, as
   Chebyshev distance between two living Mokiterions, computed by the same distance rule 3's perception uses, recomputed
   from positions and never stored. The existing statements that Mokiterions do not block movement and may share a
   coordinate are unchanged.
6. *Data and interface contracts*. The observation gains the observer's own `fear` and the suffered-attack record, and
   **gains nothing else**: its valid-proposal list is unchanged, on the decision recorded above. The event vocabulary
   gains **three** types — `attack_resolved`, `threat_resolved`, `surrender_resolved` — each with its field list and
   order fixed, each carrying the state transitions it caused including transitions to a second Mokiterion. The
   amendment must state the reason for that last obligation: rule 2 runs each Mokiterion's whole cycle in its own turn,
   so a Mokiterion acted on by a higher-identified one has already emitted its `survival_changed` line for the tick, and
   a transition not carried on the event would appear in no record. It must also state why `approach`, `avoid` and
   `retreat` add no type: each mutates only the actor and resolves as a rule 8 move, so rule 7's trace already carries
   every transition they cause, and a type that reported nothing new would be interface growth for nothing.

   **Corrected 2026-08-20 while the text was written: this provision also fixes the `action_trace` line's shape, which
   the enumeration above omitted.** The omission was not cosmetic. Provision 11 decides that the trace line reports the
   suffered-attack record, and a line cannot report a field whose position and rendering are unfixed; this provision is
   where every other record kind's field order is fixed, so it is where this one belongs. Two shapes are fixed here.
   Targeted proposals render their target inside the existing `proposal` field as `<verb>:<target-id>`, on the
   inner-colon precedent `position:<x:y>` already sets, so the field count does not change. The suffered-attack record
   is appended after `fear` as a `suffered` field that is **present only when the record is non-empty** — which is
   derived, not chosen: an unconditional field would change every `baseline --trace-actions` line, and `CAP-MOK-009`
   excludes that outright. Provision 11 keeps the semantics, which is when the field is read relative to the clearing.
7. Rule 3, *Observation*. The sentence "`fear` is deliberately **not** carried: no rule and no decision source reads it"
   is **replaced**, not deleted: the observation carries `fear` and the suffered-attack record, and the amendment states
   which requirement obliges the reader that makes them non-inert — `REQ-MOK-045` for the fields and `REQ-MOK-048` for
   the reader. The amendment record retains the withheld form and the reason it was correct for two phases. The rule's
   valid-proposal list is **untouched**, and the amendment states the consequence: a source may propose a targeted verb
   the list does not carry, so this rule no longer enumerates everything a source may legitimately propose, and rule 6 is
   where that contract is complete.
8. *State model → Name* subsection. The sentence justifying a name's absence from the observation "for the same reason
   `fear` is not" **breaks the moment `fear` is carried** and is replaced by the reason that stands on its own: no
   decision source reads a name, and `REQ-MOK-041` obliges that nothing does. This is a correction of a justification,
   not of an obligation; the name stays off the observation.
9. Rule 5, *Reference decision*, and rule 19, *Trait-aware decision*. Rule 5's accumulation paragraph is amended: its
   "addressing it is out of scope for this revision" and its "**No obligation is stated on the result in either
   direction**" are both replaced by a reference to `REQ-MOK-051`'s stated ceiling, and both are retracted rather than
   dropped. The measured 45 of 61 figure is **retained** in the amendment record as the state this correction ends.

   **Corrected 2026-08-20 while the text was written: this provision is two amendments and not one, and only the first
   is written now.** As stated above it named the numeric form of the corrected condition — rule 5's
   `satiety + restoration <= 100` and a tolerant `S + R - 100 <= T * R / 100` in rule 19 — as though the mechanism were
   settled. It is not: `REQ-MOK-051`'s *Open decisions* leaves the mechanism to the technical owner **on measurement**,
   because which relaxation reaches the ceiling is not knowable before the ceiling is measured against. What this
   amendment therefore writes is the obligation and the surface: rule 5's paragraph states the ceiling, and rule 5's
   condition and rule 19's tolerance test are named as the only two places the correction may be made. The numeric form
   is a **second amendment to this specification, taken later on the measured evidence**, and rule 5's paragraph names
   it as the one to make. The precedent is this specification's own 2026-08-19 *Behavioral trait* amendment, where an
   approved rule named in advance the amendment measurement would require. Rule 19 additionally records a hazard the
   later amendment must answer: its first clause is literally `S + R <= 100`, which is rule 5's condition as it stands
   today, so relaxing rule 5 alone would silently break rule 19's stated `T = 0` identity, its `waste_tolerance` `0`
   acceptance example, and any test asserting them. Each is named there, to be retracted explicitly or preserved
   explicitly.
10. Rule 6, *Validation*. Extended to targeted proposals: the engine validates against the target's authoritative state
    as well as the actor's, a rejected targeted proposal consumes the opportunity and mutates **neither** Mokiterion, and
    validation never consults the observation the source read. **This rule becomes the complete statement of what may be
    proposed**, because rule 3's valid-proposal list is not growing: a targeted verb absent from that list is not thereby
    invalid, and rejection is this rule's judgement alone. The amendment states each targeted verb's rejection reason.
11. Rule 7, *Optional action trace*. Covers targeted actions on the same terms, and fixes the clearing order: **the trace
    line is written before the suffered-attack record is cleared**, so the trace reports what the Mokiterion was
    answering. The amendment states that this is rule 7's existing principle rather than a second one beside it — the
    same rule already fixes that the traced `fear` is the pre-update value — and records that the traced record is
    therefore the one field on the line read before the action applies rather than after, alongside `fear`.
12. Rule 12, *Survival decay and fear*. Its own computation is unchanged. Its closing sentence — "**No rule reads
    `fear`.** It is computed, bounded, reported and otherwise inert; a consumer is a later governed change" — is
    **inverted**: a consumer now exists, it is the source of `REQ-MOK-048`, and the appended threat rule writes another
    Mokiterion's `fear`. The amendment states the composition explicitly: within one tick a Mokiterion's `fear` may be
    written both by a threat at the threatener's turn and by rule 12 at its own, in rule 2's identifier order, both
    saturating, neither suppressed.
13. Rule 13, *Death*. Substantively unchanged, and amended only to state that `health` may reach zero through combat and
    that such a death uses this path, this event and this finality. Death stays one concept.

    **Rules 20 to 26 are appended after rule 19, not inserted.** This is `WO-MOK-010`'s precedent and its stated reason:
    inserting renumbers rules that the specifications, verification contracts, verification records, retained evidence
    and source comments all cite. The *Behavioral rules* preamble, which already records that rule order stopped matching
    tick order for rule 19, is amended to say the same for these seven and to state each one's position in tick order.

    - **Rule 20, Contact.** The relation, the radius, the living-only condition, and the symmetry-with-ordered-
      registration statement.
    - **Rule 21, Targeted actions.** The seven verbs, each with its precondition; exactly-once application; `approach`,
      `avoid` and `retreat` as rule 8 moves including the fallback axis and the co-located `avoid` direction; and the
      closed eleven-kind contract. **Corrected 2026-08-20: the tie-break is not this rule's.** This bullet listed "the
      lowest-identifier tie-break" here, and it cannot be here: this rule applies a proposal that already names its
      target, so it selects among no Mokiterions at all. The nearest-then-lowest-identifier tie-break belongs to rule
      26, which chooses; rule 21 states that it does not, so a reader looking for it is sent to the rule that has it.
    - **Rule 22, Combat resolution.** Damage is `10 + (striker.energy + striker.health) / 10`, the division truncating
      toward zero, giving the range `10..=30`; the striker pays a flat `5` `energy`; both applied with saturating
      arithmetic. The rule states that no resolution deals `0` — satisfied by the constant term rather than by a clamp —
      that damage reads the striker's `energy` and `health` and nothing else, that no entropy is drawn, and that death
      reaches rule 13's existing path.
    - **Rule 23, Threat.** The target's `fear` rises by `30`, a constant of its own beside `FEAR_INCREASE` and
      `FEAR_DECREASE` rather than a multiple of either, saturating at `ATTRIBUTE_MAX`, and nothing else changes for either
      Mokiterion.
    - **Rule 24, Surrender.** The forfeit is `satiety / 2` of the surrendering Mokiterion's own `satiety`, the division
      truncating toward zero; the recipient saturates at `ATTRIBUTE_MAX` and the excess is destroyed rather than returned;
      no damage and no cost is applied. The rule states that the forfeit cannot exceed what is held, so the saturation at
      zero constrains a later amendment rather than this one, and that surrender is free below `satiety` `2`.
    - **Rule 25, The suffered-attack window.** When it opens, what it carries, when it closes, and the asymmetric latency
      that follows from rule 2's order.
    - **Rule 26, The `social` decision source.** Selected by `--policy social`. Its five branches in order — answer the
      first unanswered attack, then rule 19's cases 1 and 2, then a living Mokiterion at Chebyshev distance `1` or less,
      then one perceived at distance `2` or more, then rule 19's remainder — with the thresholds `60`, `30` and `30`, the
      nearest-then-lowest-identifier tie-break, and the statement that branches 2 and 5 propose exactly what rule 19
      proposes. The rule states **where in the tick the `fear` it reads comes from**: the value standing after the previous
      tick's rule 12 write, plus any threat applied by an earlier-acting Mokiterion in the current tick. It states the
      entropy discipline as a consequence of the ordering — at most one draw per opportunity, never for a social decision,
      and only through rule 19's own search step — and that the ordering is normative rather than illustrative.

**`SPEC-MOK-002`** — the engine's interface authority. Two provisions, and **an amendment is required here**, unlike
under `WO-MOK-011`.

1. Rule 5's enumeration gains the fourth `Policy` variant, `Action`'s seven target-carrying variants, and the three
   added `EventType` variants with their `EventDetail` payloads. `EventType::ALL` is a `pub const` array whose length is
   part of the public surface and moves from `12` to `15`. The enumeration records what the interface does **not** gain,
   and why: no verb reaches the valid-proposal list, so the observation's existing fields keep their types and `Action`'s
   seven new variants are the whole of that growth.

   **Corrected 2026-08-20 against the source: this provision stated a fact about the engine that does not hold.** It
   read "`Observation` is a public item and two fields on it are interface growth", and both halves are false.
   `mokiterions-core/src/simulation.rs:500` declares `struct Observation {` with no `pub`, and rule 6 of this same
   specification lists `Observation` among the ten names that "stay prohibited and stay private". So the observation's
   two new fields are **not** interface growth, and the amended text says so. The distinction is load-bearing rather
   than pedantic: what makes a field addition here safe is precisely that no consumer outside the engine can name the
   type, which is why the same rule 6 can require the suffered-attack record to reach a source as owned values without
   that requirement touching the public surface at all. Stating it the other way would have obliged an interface-growth
   justification for a private struct and understated the one genuine growth, `EventType::ALL`'s length. `VER-MOK-012`'s
   interface-growth check is re-checked against this correction in the same act.
2. Rule 6 is re-checked and the re-check is recorded, because cross-agent mutation is introduced for the first time. No
   public item may yield a mutable borrow of, or a reference into, authoritative state, in any build configuration
   including test builds. The suffered-attack record must reach a source as owned values on the observation.

**`SPEC-MOK-003`** — the observer. Three provisions.

1. Rule 11's authority table gains **three** rows, one per new event type, each mapping to the requirement that
   authorizes it — `attack_resolved` to `REQ-MOK-044`, `threat_resolved` to `REQ-MOK-046`, `surrender_resolved` to
   `REQ-MOK-047` — because that rule requires every event type to map to an authorizing requirement, and
   `mokiterions-tui/src/authority.rs` covers `EventType::ALL` exhaustively. **`REQ-MOK-043` takes no row**, on the
   event-type decision recorded above: the three verbs it authorizes that emit no new type are reported by
   `action_trace`, whose row already maps to `REQ-MOK-012`.
2. Rule 4's roster and rule 10's inspector, wherever they present the applied action, must present a targeted action's
   subject as well as its verb; an event whose meaning is "attacked M07" cannot be presented as "attacked".
3. Rule 4.5's refusal of inert values is unchanged and is now satisfied differently for `fear`: it has a reader. The
   amendment records that, because rule 4.5 cited `fear` as its precedent.

**Three approved requirements are amended.** Each keeps its identifier and gains an amendment-record row.

- **`REQ-MOK-005`** — its four-verb enumeration is re-read as the *core* set, beside which `REQ-MOK-043` places the
  targeted set. Amended and not superseded: it is cited by `CAP-MOK-001`, by `SPEC-MOK-001`'s and `VER-MOK-001`'s
  covered-requirement lists, by `WO-MOK-001`, by `SPEC-MOK-003` rule 11's authority table as the authority for
  `territory_crossed`, and by two locations in `mokiterions-tui`, two of those artifacts being released under
  `RLS-MOK-001`.
- **`REQ-MOK-014`** — its survivor floor is re-measured against the corrected world. The floor of eight of twelve at the
  default density is expected to stand; whether it does is measured, and if it does not the amendment is the owner's
  decision recorded in that requirement's amendment record.
- **`REQ-MOK-034`** — its survivor floor is re-measured on the same terms, and its clause that the `reference` and
  `baseline` sources' outcomes "are frozen" is **narrowed to `baseline`**, because `REQ-MOK-051` moves `reference` by
  construction. This is the one amendment in this list that changes an obligation rather than re-measuring one, and
  `VER-MOK-012` manual assessment 6 is where the owner accepts it.

## Objective

Give a Mokiterion the ability to act on another — approach, avoid, threaten, attack, and as a defender fight, retreat or
surrender — resolve those actions deterministically under engine authority, let the defender choose its own answer at its
own next opportunity, introduce the `social` decision source that reads `fear` and proposes them, correct the resource
composition drift that invalidates the same two survivor floors combat does, and demonstrate four things: that the
arithmetic is exactly what the specification states, that it is reached in real runs, that `baseline` did not move at
all, and that nothing reads a population aggregate.

## In scope

- Contact as a relation at Chebyshev radius `1`, and its one constant.
- The seven targeted actions, their preconditions, their exactly-once application and their rejection behavior.
- Deterministic combat resolution, its energy cost, and death by combat through rule 13's existing path.
- The threat's `fear` write, and the surrender's `satiety` transfer with its cap and its reported discard.
- The suffered-attack window on the observation, the observation's `fear` field, and the defender's own answer.
- The `social` decision source — its five ordered branches, its three thresholds, its `--policy` value, its help text,
  its diagnostic and its reported name — and its delegation to rule 19 for every non-social decision.
- New event types, each carrying the transitions it caused, each mapped to an authorizing requirement, each presented by
  the observer with its subject.
- The corrected waste condition in rule 5 and rule 19, and nothing else touched to correct composition.
- Re-measuring `REQ-MOK-014`'s and `REQ-MOK-034`'s survivor floors against the corrected world, and measuring
  `REQ-MOK-049`'s floor and lethality bound and `REQ-MOK-051`'s ceiling.
- The read enumeration `REQ-MOK-050` requires, covering every behavior-governing rule, every source and every validation
  path, including the ones that already exist.
- The thirteen `SPEC-MOK-001`, two `SPEC-MOK-002` and three `SPEC-MOK-003` provisions above, and the three requirement
  amendments, each written as its owner has approved it, each with an amendment-record row.
- Every check, measurement and manual assessment `VER-MOK-012` requires, and the evidence it retains.
- Updating `SIMULATION_RULES.md`, including the four §16 entries this change retires and the §14 worked example.
- Updating `docs/ROADMAP.md` with this phase's entry and status.

## Out of scope

- **Interaction memory beyond the one-opportunity window.** No record of who met whom, no grudge, no reputation, no
  alliance. Phase 3.2.
- **Perceived relative strength.** No Mokiterion reads another's `health`, `energy` or trait. Phase 3.3, and only if this
  work order's measurement shows the defender's choice degenerate.
- Any change to what `baseline`, `reference` or `individual` **propose** as verbs. None of them gains one.
- Any change to `baseline`'s output, at all, at any seed, density or trace setting.
- Cooperation, sharing, grouping, mating, kinship, and any positive social act.
- Territorial defence. Crossing `y=63/64` continues to emit an event and mean nothing else.
- Any change to the perception radius, the food table, the density mapping, rules 14 to 16, rule 9's eat effect, the
  finality of death, the exit-code contract, the default policy, or the default density.
- Entropy substreams, still declined for Phase 2's reason.
- Model-backed decisions, persistence, structured output, batch execution, a new package, target or dependency.
- Any observer feature beyond presenting what the engine reports.
- Resolving `VREC-MOK-005`, or the amendment rows and manual assessments outstanding under it and under `VREC-MOK-010`.
- Any edit to a record bound by a `verified` verification record, and any edit to an existing amendment-record row.
- Committing, pushing, tagging, releasing or publishing anything, and transitioning any artifact's status. Those are the
  owner's acts.

## Authorized decision envelope

The implementation agent may decide locally:

- Rust file organization, private type and function names, and where a private helper lives.
- The internal representation of the suffered-attack record, provided it is per-Mokiterion, bounded, cleared at the
  opportunity, and reaches a source as owned values.
- The field order within a new event's detail, and the private representation behind it, provided the order is the one the
  approved `SPEC-MOK-002` amendment fixes and every emitted type maps to an authorizing requirement under `SPEC-MOK-003`
  rule 11. **Which event types exist is not in this envelope**: the owner decided three, and they are named above.
- Which of `REQ-MOK-051`'s **permitted** mechanisms is used — relaxing rule 5's waste condition, raising rule 19's
  tolerance floor, or both — on measurement, and within the permitted set only.
- Which test tier each new test belongs to, applying `SPEC-MOK-004`'s rule as written. **Widening an item to `pub` to
  relocate a test is prohibited**, and a test that seems to need it belongs inline.
- The internal organization of test modules, constructed-state helpers and evidence scripts, provided every script is
  retained in full.
- The exact wording of amended text, once its owner has approved the substance, and of the plain-language and roadmap
  updates.

The implementation agent may **not** decide:

- Any of the three decisions under *The three decisions the validation did not supply* above — the event-type set, the
  valid-proposal list, the clearing order. All three are now stated, and each is on the same footing as the ten below it:
  decided, and escalated rather than adjusted.
- Any of the ten values under *The values decided after the packet was drafted*. They are decided, and a measurement that
  makes one look wrong is escalated rather than adjusted.
- Whether the defender's answer is deferred, or resolved inside the attacker's turn.
- Whether resolution draws entropy. It does not.
- **The order of the `social` source's five branches, or the way branch 2 tests rule 19's cases 1 and 2.** The ordering is
  normative, and restructuring branch 2 to delegate to rule 19 and inspect the result would take a draw and discard it.
- **Whether rule 19's implementation is generalized to serve the `social` source.** It is not: the dependency runs one way,
  and no existing source's code path is altered to serve the new one.
- The contact radius, the set of eleven action kinds, or any precondition in `REQ-MOK-043`.
- Where the composition correction is placed. The permitted set is fixed by `REQ-MOK-051` and rule 4, rule 9's eat
  effect, the food table and rules 14 to 16 are outside it.
- Any relaxation, widening, removal, rename or `#[ignore]` of an existing test.
- Any `allow` attribute, any lint suppression, or any change to a gate.
- Any change to a floor, a threshold, a default or an exit code, including `REQ-MOK-049`'s floor of five, `REQ-MOK-051`'s
  ceiling of one half, and the `social` source's `60`, `30` and `30`. A measurement that misses a bound is escalated, not
  adjusted.
- The substance of any amendment, or the addition of an amendment row for an amendment its owner has not approved.

## Constraints

- **`baseline` does not move.** No draw against the shared entropy stream from any new path, and no change to anything a
  `baseline` run reads. Its 30 matrix cells are byte-identical, unprojected.
- **No entropy draw** in contact, validation, resolution, threat, forfeit, window handling or observation building.
- **No population aggregate** reaches any rule, source or validation, and the enumeration proving it covers the rules
  that already exist as well as the new ones.
- Integer, saturating arithmetic on existing attributes only. No floating point anywhere in this change.
- Every attribute stays within `0..=100` at every tick, and every transfer's identity — paid equals received plus
  discarded — holds.
- Owned, reference-free values on every public item; no mutable borrow of or reference into authoritative state in any
  build configuration including tests.
- The engine's dependency and dev-dependency tables stay empty; `cargo tree -p Mokiterions` resolves to one crate.
- The observer's dependency set stays at one path dependency and `ratatui` at its pinned version and feature set.
- Every rendering claim is asserted against an in-memory character buffer at stated cell positions. No screenshot, no
  recording, no terminal, no pseudo-terminal.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` and
  `cargo test` at the workspace root all clean, in one invocation each, with nothing suppressed.
- No existing test's assertions are relaxed. A test whose subject genuinely changed is updated without its assertion
  count falling, and every such test is named in the completion summary.
- No secret, credential or model-provider key enters the repository, in code, in a fixture or in retained evidence.

## Expected change surface

**`mokiterions-core/src/simulation.rs`**

- A contact-radius constant, and a contact predicate over two positions reusing the existing Chebyshev distance.
- `enum Action` gains seven variants, each carrying a target identifier. The contract stays closed.
- `enum Policy` gains a fourth variant for `social`; `cli.rs`'s parser, `USAGE` text and invalid-value diagnostic follow.
- A fourth `DecisionSource` implementation for `social`, reading `fear` and the suffered-attack record from the
  observation, with the five branches in the stated order and the three thresholds as named constants.
- **Branches 2 and 5 call the existing trait-aware implementation.** `IndividualDecisionSource`'s decide path, its
  tolerance handling and its selectors are **unchanged**: no parameter added, no tolerance argument threaded through, no
  branch inserted for `social`'s benefit. The three constants near `simulation.rs:404` and `:788` that state "only the
  trait-aware source reads it" and that rule 19 "reads the trait and nothing else new" are the comments this change makes
  stale, and updating them is part of the surface.
- Branch 2 tests rule 19's cases 1 and 2 directly rather than delegating and inspecting, so that no draw is taken for a
  decision that is then discarded.
- `struct Observation` gains the observer's own `fear` and the suffered-attack record, as owned values.
- `struct Mokiterion` gains the transient suffered-attack record, cleared at its own opportunity.
- Validation gains the targeted preconditions, checked against authoritative state, with rejection reasons naming the
  unmet precondition.
- Resolution functions for attack/fight, threaten and surrender, each pure integer arithmetic over the two Mokiterions'
  attributes, each with no access to any collection of Mokiterions.
- `enum EventType` and `enum EventDetail` gain `AttackResolved`, `ThreatResolved` and `SurrenderResolved`, with
  `EventType::ALL` extended from twelve entries to fifteen. Each detail carries both parties' transitions. `approach`,
  `avoid` and `retreat` add nothing here: they resolve as rule 8 moves and rule 7's trace already reports them.
- Rule 5's and rule 19's waste condition corrected, and **nothing else in the food, eat or regeneration paths touched**.
- Internal-tier tests: the constructed-state resolution tables, entropy neutrality either side of every resolution, the
  identifier-exchange comparison, window opening and closing, the transfer identity including the discard, and the
  boundary cases each requirement names.

**`mokiterions-core/src/cli.rs`**

- The `--policy` usage line, the long help block and the invalid-value diagnostic name four values. The `concat!` literal
  discipline — one line per literal — is preserved.

**`mokiterions-tui/src/authority.rs`**

- `for_type` gains one arm per new `EventType`, each naming the authorizing requirement. The existing test over
  `EventType::ALL` is what forces this, and the table's growth is the intended consequence rather than a surprise.

**`mokiterions-tui/src/state.rs`, `render.rs`**

- Ingest and presentation for the new event types, with the subject of a targeted action presented alongside its verb in
  the roster's applied-action position and in the inspector.
- No change to bar arithmetic, pane widths, layout thresholds or which panes exist.

**Documentation:** `SIMULATION_RULES.md` and `docs/ROADMAP.md`.

**Tests that must be updated because their subject changed** are named individually in the completion summary, with the
assertion count before and after. The census is what demonstrates that no other test moved.

## Required verification

`VER-MOK-012`, in full: its eight oracles, its requirement-to-evidence matrix, its stated monotonicity band, its
property and invariant tests, its static, security, privacy, performance and resilience checks, and its eleven manual
assessments. A manual assessment that is not recorded is outstanding, and the contract is not satisfied while any remains
outstanding. Eight of the eleven now have their subject decided; what each of those eight asks is a ratification against
measured evidence, and a decided value does not make an assessment recorded.

Oracle 1's pre-change capture of the 90-cell three-source matrix is taken **before the first code change**, from a
tracked-clean worktree, with the commit recorded. A discrepancy is never resolved by recapturing it.

Oracle 3's constructed states and oracle 4's whole runs are both required, and neither may stand for the other.

Oracle 8, the delegation equality, is checked **per observation and never per run**: `social` diverges from `individual` at
the first opportunity where a Mokiterion perceives another and takes fewer draws over a run, so a whole-run comparison
between the two sources is expected to differ and is not evidence of a defect.

## Evidence to record

Under `docs/engineering/simulation/evidence/WO-MOK-012/`, everything `VER-MOK-012`'s *Evidence retention* section lists.
In particular:

- `baseline/COMMIT.txt`, the commit the pre-change capture was taken at, with the worktree tracked-clean at capture;
- `capture.sh`, the comparison and analysis scripts, and the read-enumeration script if the enumeration is automated,
  each retained in full so that every figure is reproducible from a recorded command rather than trusted;
- `baseline/pre-manifest.txt` and `post/post-manifest.txt`, per-cell digests of the 90-cell matrix on both sides;
- `additivity.txt`, the 30 `baseline` cells' byte comparison with exit codes, and the 60 `reference` and `individual`
  cells' characterized divergence with first diverging tick and record;
- `social/`, the 30 new cells under the `social` source, with exit codes;
- `resolution-tables.md`, the constructed-state inputs, the value computed from the specification on paper, and the value
  the engine produced, for every case and boundary;
- `entropy.txt`, the shared stream's state either side of every resolution kind, and the reused `VER-MOK-007`
  initialization expectations;
- `runs.md`, per-seed survivors, deaths by cause, encounters, each verb proposed and applied, and rejections by reason,
  retained on failing seeds as well as passing ones because the curve is what the floor is ratified on;
- `delegation.md`, the per-observation comparison of `social` against `individual` wherever no living Mokiterion is
  perceived and no attack is unanswered, with both proposals, the shared stream's position either side, and each source's
  total draw count at matched seeds;
- `branches.md`, how often each of the `social` source's five branches fired per seed and how the answer branch split
  between `surrender`, `retreat` and `fight`, together with the measured strikes per encounter, the frequency of a forfeit
  discarded at a full recipient and the frequency of a surrender in the free band below `satiety` `2` — the evidence
  manual assessments 1, 2 and 8 are taken on;
- `identifier.md`, the per-identifier series, the rank correlations, the band evaluation, and the identifier-exchange
  comparison;
- `composition.md`, rule 18's summary per seed under each source at both commits, with the per-territory per-class ratio;
- `world-rules-unchanged.txt`, the line-for-line comparison showing rule 4, rule 9's eat effect, the food table and rules
  14 to 16 unchanged;
- `reads.md`, the enumeration `REQ-MOK-050` requires, with rules 14 to 16 and 18 recorded as outside the obligation and
  why;
- `writers.txt`, the enumeration of `fear`'s writers and of every path that writes a second Mokiterion's state;
- `interface.txt`, the engine's public interface at both commits, compared item for item against the approved
  `SPEC-MOK-002` amendment;
- `observer.md`, the authority table's new rows, the `EventType::ALL` exhaustiveness check, and rendered frames with cell
  positions;
- `test-census.txt`, reconciled name by name, with every updated test named and its assertion count before and after;
- `gates.txt`, the four gate commands and their output;
- `long-horizon.md`, the 10,000-tick run's completion, composition and survivor figures, as evidence and not as an
  obligation;
- `manual-assessment.md`, the eleven assessments with role and date;
- `amendment-approvals.md`, oracle 7's check, including the recorded state of the `VREC-MOK-005` gate;
- `completion-summary.md`.

## Stop and escalate conditions

Stop and escalate rather than proceeding, if:

1. **Any `baseline` cell differs**, in any byte or exit code, at any seed, density or trace setting.
2. **Oracle 2 shows any movement** in the shared stream's position, at any seed or density, or across any resolution.
3. **A `reference` or `individual` divergence cannot be attributed** to the corrected waste condition.
4. **The composition ceiling cannot be met within `REQ-MOK-051`'s permitted mechanisms.** Reaching for rule 16, rule 15,
   rule 9's eat effect or the food table is not a fallback available to the implementation; it is a decision that changes
   what this work order promises about `baseline`, and it is the owner's.
5. **`REQ-MOK-049`'s survivor floor or lethality bound is missed**, or **`REQ-MOK-014`'s or `REQ-MOK-034`'s re-measured
   floor fails.** The floors are not adjusted to fit the measurement; the measurement is reported and the owner decides.
6. **The identifier-monotonicity band fails, or the identifier-exchange test fails.** The second means resolution reads
   an identifier and is a defect; the first is a finding about turn order and is the owner's to weigh.
7. **A verb never applies** in any run across the declared matrix, or a defender's branch distribution is degenerate on
   every seed. The second is a finding `INT-MOK-009` anticipates and wants reported, not hidden by tuning the source.
8. **Any existing test's assertions would have to be relaxed**, widened, removed, renamed away or `#[ignore]`d.
9. **The public interface would grow beyond the approved `SPEC-MOK-002` amendment**, or `SPEC-MOK-002` rule 6's
   re-check finds a public item exposing a borrow into authoritative state.
10. **A read cannot be resolved** to the acting Mokiterion's own state, its observation or a named individual — that is,
    `REQ-MOK-050` cannot be satisfied without redesigning something.
11. **The work reaches a decision this work order does not carry.** The three that were outstanding are stated, so this
    condition no longer names them; it stands for the next one, and the test is whether the answer is written in an
    artifact rather than inferable from one. Choosing it locally is the failure this condition exists to prevent.
12. **Any required amendment is unapproved** when the evidence would otherwise be complete. An amendment nobody approved
    is not a specification.
13. **The delegation equality fails** on any observation where no living Mokiterion is perceived and no attack is
    unanswered. That is either a reimplementation of rule 19 where a delegation was required, or a change to rule 19
    itself, and neither is within this work order's authority.
14. **A draw is taken and then discarded** anywhere in the `social` source's path. The remedy is the stated branch
    ordering, never a compensating restore of the stream's position, which would leave every capture byte-identical and
    still be wrong.

**Disclosed rather than fixed, and outside this work order's scope:**

- **`VREC-MOK-010` records `VER-MOK-010` as satisfied but for its `VREC-MOK-005` gate row.** That gate — the outstanding
  amendments and manual assessments under `VREC-MOK-005` — is a precondition of this work under `VER-MOK-012` oracle 7,
  not an output of it. `VER-MOK-011` states the same for itself.
- **`python -m se_harness doctor .` exits `1` on this machine** with eight `FAIL distribution:` lines. This is version
  skew between the locally installed harness and the `0.4.0` the repository pins and CI installs, not a repository
  defect. `python -m se_harness validate .` passes.
- **`mokiterions-tui/tests/export.rs`'s `a_written_file_holds_exactly_the_rendered_records` is intermittently flaky on
  this platform**, as `WO-MOK-011` disclosed. A test change is a stop condition here, so it is reported as residual
  rather than adjusted.

## Completion report format

The completion summary records, in this order:

1. what changed, file by file, and what deliberately did not — rule 4, rule 9's eat effect, the food table, rules 14 to
   16, `baseline`'s output, the perception radius, the defaults, the exit codes;
2. the owner decisions as stated at approval — the nine of shape, the ten of value, and the three that were still
   outstanding when this work order was drafted, with what the owner supplied for each;
3. oracle 1's result: the 30 `baseline` cells byte for byte, and the 60 others' characterized divergence;
4. oracle 2's result, and the recorded stream positions it was checked against;
5. the constructed-state resolution tables, with the paper computation beside the engine's output;
6. the whole-run figures: survivors, deaths by cause, verbs proposed and applied, rejections by reason, per seed, the
   `social` source's branch distribution beside them, and oracle 8's delegation comparison with each source's draw count
   at matched seeds;
7. `REQ-MOK-049`'s two bounds per seed, and `REQ-MOK-014`'s and `REQ-MOK-034`'s re-measured floors;
8. `REQ-MOK-051`'s composition ratio per territory per class, at both commits;
9. the identifier-exchange result and the monotonicity band evaluation, with its weakness restated;
10. the read enumeration, with rules 14 to 16 and 18 recorded as outside the obligation;
11. the public-interface comparison, item for item, against the approved `SPEC-MOK-002` amendment;
12. the observer's authority rows and rendered-frame evidence;
13. the census reconciliation, with every updated test named and its assertion count before and after;
14. the four gates' output;
15. the amendment records — thirteen `SPEC-MOK-001` provisions, two `SPEC-MOK-002`, three `SPEC-MOK-003`, and the three
    requirement amendments — each row quoted, with the approval each carries;
16. the eleven manual assessments, or an explicit statement of which are outstanding and who owes them;
17. residual uncertainty, including everything under *Stop and escalate conditions* that remains open, and the statement
    that `VREC-MOK-012` is a separate commit-bound record that this work order does not write and cannot self-approve.
