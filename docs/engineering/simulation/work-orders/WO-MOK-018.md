+++
id = "WO-MOK-018"
type = "work_order"
title = "Close the two observer defects Phase 3.1 left: a stale filterable-type count, and a fourth attribute that becomes unreadable at death"
status = "in_progress"
owners = ["engineering owner"]
created = "2026-08-21"
updated = "2026-08-21"

[assurance]
commit_bound_verification = "required"
rationale = "The scope is mixed and `WORKFLOW.md` forbids inferring a classification for mixed scope: one half is specification text that no build reads, and the other changes what the inspector presents at the moment a Mokiterion dies, which is executable behaviour on the instrument every later phase is judged with. The load-bearing claim cannot be asserted — that a dead subject's final `fear` is the engine's own last reported value and is absent rather than zero-filled where the engine reported none — because it is a claim about a rendered buffer for a subject that has left `WorldSnapshot::agents`, and the value reaches the pane through a derived map the observer builds rather than through the read-only interface. The work also corrects `SPEC-MOK-004`'s recorded test figures, which are trusted engineering state that three approved artifacts and every later census read, and it corrects them across two work orders' additions at once, so a figure that is wrong here is wrong for `WO-MOK-016` as well as for this work order. `REQ-MOK-025`'s non-perturbation obligation is also in scope by construction, because the change widens a map the observer fills from the event stream."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-021", "REQ-MOK-022"]
specifications = ["SPEC-MOK-003", "SPEC-MOK-004"]
architecture = ["ARCH-MOK-002", "ADR-MOK-003"]
verification = ["VER-MOK-005"]
+++

# Work Order: Close the two observer defects Phase 3.1 left

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and the retained evidence. Verification and release require separate commit-bound records.

**Approved and directed on 2026-08-21.** The repository owner, who holds every accountable role in this repository, was
shown this work order with its two decisions and its four required amendments in full, approved the two-tier scope, and
then directed implementation in one act. That act is read as covering the work order's own approval and the four
amendments its *Required amendments* section tabulates, each of which names its acting owner — amendments 1, 2 and 3 the
technical owner's and amendment 4 the assurance owner's — on the precedent of `WO-MOK-003`, where the same owner cleared
a gate explicitly and directed the tabulated amendments to be applied. Nothing outside that tabulation is approved by
it, and this section is the record of what it reaches.

**A fifth amendment was found during implementation and is not covered by that act.** `SPEC-MOK-004` rule 6's figures
move, which this work order predicted they would not; amendment 5 below states it, and it is **OUTSTANDING** for the
technical owner's ratification. The same is true of two provisions inside amendments already approved: rule 10.6's
two-line pairing, and `VER-MOK-005`'s correction of a stale `name`. Each is named in the completion report and marked
where it is recorded, rather than presented as approved text.

Commit-bound verification is classified `required` above.

`ARCH-MOK-002` is selected because it `addresses` `REQ-MOK-021`, and `ADR-MOK-003` is selected as the active ADR that
decides it, on the precedent of `WO-MOK-005`. The architecture is selected for coverage and **not because any boundary
moves**: no dependency edge, no trust boundary, no target shape and no member of the observer's public interface
changes here, and the change surface is one private struct, one private map and one private rendering function. If the
implementation finds that it cannot hold that line, the *Stop and escalate* conditions below apply.

### Why this work order exists

`WO-MOK-016` delivered Phase 3.1's encounter model and is closed: it is `implemented`, and `VREC-MOK-017` is `verified`
against `ecba9fe`. On 2026-08-21 the repository owner reported that the observer did not reflect the new changes or the
new attribute. The observer was measured against `SPEC-MOK-003` rather than against expectation, and most of what
looked absent is **specified as absent with a stated reason**: rule 10 item 7's re-check of 2026-08-20 puts the
suffered-attack record and the count of attacks suffered in the set this pane does not present, and names the log pane
and the export as where they are carried. Presenting them is a later decision that the item itself says needs rule
10's presented-value list amended first, and that decision is not taken here.

**Two things were found that no rule authorizes and no oracle caught.** Both are defects against text already in
force, and this work order exists for those two and for the consequences they force. Neither is a new capability, and
no requirement is created: `REQ-MOK-021` and `REQ-MOK-022` are `approved` and already carry both.

### Defect 1 — `SPEC-MOK-003` rule 9 item 2 states a count that no tree runs

Rule 9 item 2 reads *"Filtering by event type restricts the presentation to one of the eleven core types or to
`action_trace`."* That figure was correct before `CAP-MOK-010` and is now wrong in the vocabulary's own terms:

- `SPEC-MOK-001`'s *Output* section names the stable core event types and, as amended under `CAP-MOK-010`, names
  **fourteen** of them — `world_initialized`, `food_initialized`, `agent_initialized`, `decision_source_selected`,
  `food_consumed`, `food_regenerated`, `food_regeneration_skipped`, `territory_crossed`, `attack_resolved`,
  `threat_resolved`, `surrender_resolved`, `survival_changed`, `agent_died` and `simulation_ended` — and adds that
  *"Optional per-action lines use `action_trace`."*
- `EventType::ALL` in `mokiterions-core/src/simulation.rs` is `[Self; 15]`, which is those fourteen plus
  `action_trace`, in the order `SPEC-MOK-001` lists them.
- `Observer::cycle_type_filter` walks `ALL`, so the observer already offers all fifteen filter positions.

**The vocabulary owner was amended and the observer specification's derived count was missed.** The `CAP-MOK-010`
amendment row in `SPEC-MOK-003`'s amendment record names rules 4, 10 and 11 and closes *"no pane geometry, key
binding, export format, snapshot contract or figure changes"*; this is a figure that needed changing, and the sentence
is true of what that amendment did and false of what it should have done. The code is right and the specification is
wrong, which is the direction that matters: an operator reading rule 9 item 2 would conclude that three of the types
the observer offers are outside the contract.

**It is the only occurrence.** `eleven core` appears once in `docs/engineering/` outside `evidence/`, and
`REQ-MOK-022`'s own statement carries no count at all — it says *"restricted to a single event type or a single
subject"* — so the requirement is unaffected and nothing above the specification needs to move.

### Defect 2 — `fear` becomes unreadable at death, and rule 10 contradicts itself there

Rule 10 item 6 requires that when the selected Mokiterion dies *"the selection is retained and the pane presents the
death, the tick of death, and the final attribute values."* Rule 4 clause 5, as amended under `REQ-MOK-032` and
re-grounded under `CAP-MOK-010`, makes `fear` the fourth of four attributes on the roster's bar lines, and
`SPEC-MOK-002` rule 5 carries it on `AgentSnapshot` beside health, satiety and energy. So `fear` is an attribute, and
rule 10 item 6 asks for the final value of every attribute.

Rule 10's presented-value list does not carry it, and item 7 states why:

> `fear` is absent because this rule's presented-value list above is not amended by `WO-MOK-010`, whose observer
> change surface is the roster's bar row alone, and rule 4 presents `fear` there **for every living Mokiterion**
> including the selected one, **so no value is unreachable**.

**The justifying premise lapses at exactly the case item 6 governs.** `draw_roster` iterates `WorldSnapshot::agents`,
which is the living set, so a dead Mokiterion has no roster entry and its `fear` is presented nowhere. Item 7's
reasoning is that the value is reachable elsewhere; after death it is not. Item 7's 2026-08-20 re-check re-asserted the
premise in the same words — *"rule 4 still presents `fear` for every living Mokiterion including the selected one"* —
so the re-check carried the gap forward rather than finding it.

The code follows item 7 rather than item 6: `Death` in `mokiterions-tui/src/state.rs` carries `id`, `tick`, `health`,
`satiety: Option<u8>` and `energy: Option<u8>`, and `inspector_lines`' death branch renders `final health` with
`satiety` and `energy` appended only where each was recorded. **There is no defect in the code against the
specification as written.** The defect is that the specification cannot be satisfied in both items at once, and the
technical owner has resolved it in item 6's direction.

**The value is already in the observer's hands and is discarded.** `EventDetail::SurvivalChanged` carries
`health: (u8, u8)`, `satiety: (u8, u8)`, `energy: (u8, u8)` and `fear: (u8, u8)`. `Observer::ingest` matches that
variant and destructures it as `{ satiety, energy, .. }`, storing the pair into
`latest_survival: BTreeMap<String, (u8, u8)>` and dropping the reported `fear` on the floor. The death branch reads
that same map. So closing the defect adds no new source of truth, takes no new interface item, and reads nothing the
observer was not already receiving from the engine's own records.

## Decision record

Both decisions were taken by the repository owner on 2026-08-21, each put as its own question with the alternative and
its measured cost stated. The implementation agent found the defects, measured the options, wrote this text and decided
neither.

| # | Decision | Taken as | Alternative put and declined |
|---|---|---|---|
| 1 | Rule 10's contradiction closes in **item 6's** direction: `fear` is named among the final attribute values a dead subject's pane presents, and item 7's justification is restated so that it no longer rests on a premise that lapses at death. | technical owner | Amend item 7 alone, so that `fear` is stated to be deliberately unpresented after death as well as during life. Text only, no code, no test. Declined: it leaves the operator with a fourth attribute that becomes unreadable at the tick it mattered most, and rule 10 item 6 already asks for every attribute's final value. |
| 2 | `commit_bound_verification` is **`required`**. | engineering owner | `not_required`, on the reading that the work order transports authorized governance text. Declined: it is only defensible under decision 1's declined alternative, and under decision 1 as taken it would classify executable behaviour away, which `WORKFLOW.md` forbids for mixed scope. |

## Required amendments

Four amendments are required and **none of them is this work order's to make**. Each is stated here so that the act
approving this work order can approve the amendment it authorizes, on the precedent of `WO-MOK-016`'s own *Required
amendments* section. Implementation begins after those acts and not before.

**A fifth was found during implementation, is stated as amendment 5 below, and was not part of the approving act.** It
is recorded here rather than in a later work order because rule 6's **Growth** clause requires the rule to be amended
*in the same act* as the growth, so deferring it would leave the specification stating a figure the tree contradicts.

### 1. `SPEC-MOK-003` rule 9 item 2 — the figure (technical owner)

`eleven` becomes `fourteen`. Nothing else in the item changes: `action_trace`'s separate mention, the subject filter,
the presentation-only clause and items 1 and 3 through 6 are untouched. The amendment record carries a row stating that
the figure was correct until `CAP-MOK-010` added three core types, that the `CAP-MOK-010` row's *"no figure changes"*
sentence is true of what that amendment did rather than of what this rule required, and that the corrected figure is
`SPEC-MOK-001`'s measurement and not a new decision.

### 2. `SPEC-MOK-003` rule 10 — the death case (technical owner)

Item 6's *"the final attribute values"* is made explicit as the four attributes rule 4 clause 5 names, so that a reader
cannot satisfy the item while omitting one. Item 7's `fear` paragraph is restated: the value is presented in this pane
for a dead subject under item 6, and item 7's reachability reasoning is recorded as having held only while the subject
was living — retained rather than deleted, on this document's own practice of keeping the reasoning that made an
earlier position correct. **Rule 10's presented-value list gains `fear` for the dead-subject case only**, which is the
narrowest form that satisfies item 6; whether a living subject's `fear` is added to the pane is a separate decision
that item 7's existing reasoning still answers in the negative, and it is out of scope below. The rendering follows
item 7's standing rule that a value the engine did not compute is absent and not zero-filled, so a Mokiterion that
died before any `survival_changed` record reported its `fear` presents no `fear` at all — the same treatment `satiety`
and `energy` already receive in that branch.

### 3. `SPEC-MOK-004` rules 9, 10 and 11 — the recorded test figures (technical owner)

Rule 11 states the obligation in its own text: *"a work order that adds a test corrects these figures here, and one
that loses a test has a defect."* This work order adds tests, so the correction is forced rather than chosen.

**The correction necessarily covers `WO-MOK-016`'s additions as well as this work order's, and that is this
specification's own precedent rather than a widening of scope.** The last figures rule 11 records are
`WO-MOK-013`'s — observer **141**, engine **85**, workspace **226** — and `WO-MOK-016` reached `master` having added
tests without correcting them; `evidence/WO-MOK-016/amendment-approvals.md` §5 records that omission as owed and calls
it *"the one thing on oracle 7's surface that is neither present nor contingent"*. `VREC-MOK-017` measures the
workspace at **264** at `ecba9fe`. Any figure measured on this work order's implementing tree therefore already
contains `WO-MOK-016`'s arrivals, and stating only this work order's would leave the rule at a number no tree runs —
which is exactly the reasoning of the amendment record's 2026-08-19 row, where `WO-MOK-010`'s and `master`'s
`WO-MOK-007`'s additions were corrected together because *"neither work order's figures are statable without the
other's"*.

So this work order **discharges `WO-MOK-016`'s owed rule 11 correction** as a consequence of obeying rule 11 for its
own tests. Every figure in the row is a measured outcome and not a decision: the totals are `cargo test --workspace`'s
per-target output on the implementing tree, and rule 6's interface figures are that rule's own counting rule applied to
the observer's `pub mod` files. The arrivals are attributed to `WO-MOK-016` or to this work order, target by target, so
that a reader can tell which work order each belongs to.

### 4. `VER-MOK-005` — a case for the death-case value (assurance owner)

`VER-MOK-005` is `approved`, `verifies` `REQ-MOK-021` and `REQ-MOK-022`, and already carries an amendment record from
2026-08-20, so amending an approved verification contract has precedent here. It gains the coverage decision 1 creates:
that a dead subject's pane presents the engine's own last reported `fear`, and that it presents no `fear` where the
engine reported none. Rule 9 item 2's corrected figure is a specification-text correction with no runtime subject and
needs no case of its own; whether the contract nonetheless wants the filter's reachable set asserted at fifteen is the
assurance owner's call at approval, and this work order does not presume it.

### 5. `SPEC-MOK-004` rule 6 — one public field (technical owner) — OUTSTANDING, found during implementation

**This amendment was not in the four the owner approved, and this work order asserted the opposite.** Its *Constraints*
section said rule 6's "**94** items, **118** `pub` lines and **24** public fields stand. This is to be measured and not
assumed." They were measured, at 118 before the change and 119 after, and they do not stand: `state::Death` gains
`pub fear: Option<u8>`, so the pair reads **25** public fields and **119** `pub` lines. **The item count of 94 is
unchanged**, which is why the prediction looked safe and why it was wrong — rule 6 counts a public field separately from
the item that declares it, and the change was scoped by its effect on items.

Two errors in this work order produced the wrong prediction, and both are recorded rather than corrected away:

1. The *Authorized decision envelope* grants the agent "the field name and type on `Death`" on the stated ground that
   it and `latest_survival` are "both private to `mokiterions-tui::state` and neither is a member of the interface
   `SPEC-MOK-004` rule 6 counts". **`latest_survival` is private; `Death` is not.** `Death` is `pub` in a `pub mod` and
   its `id`, `tick`, `health`, `satiety` and `energy` fields are all `pub`, so the grant rested on a false premise about
   what the field is.
2. The *Lifecycle* section states that "no member of the observer's public interface changes here". Under rule 6's own
   counting that sentence is true — a member is an item, and no item is added — which is exactly why it did not catch
   the field. **The first stop-and-escalate condition is not tripped**: it forbids closing defect 2 by way of "a member
   of the observer's public interface", and the growth is a field rather than a member.

Implementation continued rather than stopping, because the alternative that avoids the field is worse on this work
order's own terms: an accessor on `Observer` would add a public **item** and move 94, which the first stop-and-escalate
condition forbids outright. A `pub(crate)` field beside three `pub` siblings would make one struct partly opaque and
relocate its public-tier case for no reason but visibility, and deriving the value in `render` is impossible because
the state it comes from is private. So the narrowest available form was implemented and is reported here.

The amendment as applied records the figures, names `REQ-MOK-021` by way of `SPEC-MOK-003` rule 10.6 as the requirement
the **Growth** clause needs, states the three alternatives as measured and worse, and states that rule 7 is untouched
because a field added to a struct whose fields are already public widens no existing item. It is **OUTSTANDING** for the
technical owner's ratification, on the precedent of `SPEC-MOK-004`'s own 2026-08-19 row, which was outstanding for the
same reason — an interface consequence found after the fact by measuring the rule — until the owner ratified it under
`WO-MOK-012`.

## Objective

Leave `SPEC-MOK-003` internally consistent with `SPEC-MOK-001`'s vocabulary and with itself, and leave a dead
Mokiterion's fourth attribute readable in the pane that rule 10 item 6 makes responsible for its final values —
without adding a value to the living-subject pane, without touching the observer's public interface, and without
moving any figure that a record bound to a commit already measured.

## In scope

1. **The rule 9 item 2 figure.** Apply amendment 1 once approved. No code.
2. **`fear` on the death record.** `Death` gains a `fear: Option<u8>` field. `latest_survival` widens from
   `(u8, u8)` to carry the `fear` that `SurvivalChanged` already reports, and `Observer::ingest`'s
   `SurvivalChanged` arm stops discarding it. The `AgentDied` arm reads it from that map exactly as it reads `satiety`
   and `energy` today.
3. **The rendering.** `inspector_lines`' death branch appends `fear` to the `final values` line under the same
   `Option` guard the other two use, in the attribute order rule 4 clause 5 fixes — health, satiety, energy, fear —
   so that the death line reads in the same order as the roster's gauges.
4. **Tests.** At least one public-tier case asserting the presented death line for a subject whose `fear` was
   reported, and at least one asserting that no `fear` is presented where none was reported. Placement follows
   `SPEC-MOK-004` rule 8: the internal tier only where the assertion requires access the public interface does not
   yield, and the public tier otherwise.
5. **The `SPEC-MOK-004` figure corrections the added tests force**, in the joint form amendment 3 states, measured on
   the implementing tree.
6. **Evidence** keyed to `WO-MOK-018`, as listed below.

## Out of scope

- **`fear` in the living-subject inspector.** Rule 10 item 7's reachability reasoning still holds while the subject is
  living, because rule 4 presents the gauge. Adding it there is a separate decision and is not taken here.
- **The suffered-attack record, the count of attacks suffered, and any derived encounter tally.** Rule 10 item 7's
  2026-08-20 re-check places all three outside this pane and names the log pane and the export as where they are
  carried. `Observer::ingest` continues to drop `AttackResolved`, `ThreatResolved` and `SurrenderResolved` at its
  catch-all arm, and no derived social state is built.
- **A cause of death on the death line.** `AttackResolved { target_died }` would distinguish a combat death from a
  starvation death, and no rule requires it. It needs rule 10 item 6 amended in a way decision 1 did not reach, and it
  belongs to the next work order.
- **A direct filter selection of the three new event types.** `cycle_type_filter` now needs up to fifteen presses of
  `e` to reach `surrender_resolved`. That is a rule 7 key-binding question and a rule 9 question, and correcting rule
  9 item 2's figure neither creates it nor answers it.
- **Any canvas indication of an engagement**, which is rule 2's subject and would meet rule 2.5's colour-alone
  constraint.
- **`REQ-MOK-060`'s composition drift**, which is `WO-MOK-017`'s and is untouched here.
- **Any figure inside `evidence/` or inside any `VREC-*`.** Evidence is re-run and not edited, and a record bound to a
  commit remains true of that commit.
- **The managed `engineering-harness.yml` workflow**, whose depth-1 checkout inflates the dashboard warning count.
  `VREC-MOK-017` records that and it is not this work order's to amend.

## Authorized decision envelope

The implementation agent may decide locally:

- the field name and type on `Death`, and whether `latest_survival` becomes a wider tuple or a small named struct —
  both are private to `mokiterions-tui::state` and neither is a member of the interface `SPEC-MOK-004` rule 6 counts;

  **The stated ground is wrong for `Death` and right for `latest_survival`.** `Death` is `pub` in a `pub mod` and its
  existing fields are `pub`, so a field added to it is part of rule 6's interface and moves that rule's public-field
  and `pub`-line figures, though not its item count. The grant itself is not withdrawn — the field's name and type are
  still the agent's, and `fear: Option<u8>` mirrors the two siblings it joins — but the consequence the ground denied is
  real and is amendment 5.
- the exact wording and spacing of the death line's `fear` segment, within amendment 2's attribute order and item 7's
  absent-not-zero-filled rule;
- test names, and the tier each case lands in under `SPEC-MOK-004` rule 8, reporting the placement and its ground;
- the wording of the `SPEC-MOK-004` figure row, every figure in which is measured rather than chosen.

It may not decide:

- whether `fear` is presented for a living subject, or whether any further value joins rule 10's list;
- any figure in amendment 1, which is `SPEC-MOK-001`'s measurement;
- the substance or wording of rule 10 item 6 or item 7 beyond amendment 2 as approved;
- whether `VER-MOK-005` gains a case, which is the assurance owner's;
- the work order's assurance classification.

## Constraints

- The observer reads the value from the engine's own records and derives nothing. `REQ-MOK-021`'s constraint that the
  observer *"does not re-derive, re-evaluate, or predict"* the values it presents governs `fear` as it governs the
  other three, and a `fear` inferred from a `ThreatResolved` increase, from a co-location count or from any rule of
  `SPEC-MOK-001` rule 12 would violate it.
- No mutable handle to world, agent, resource, event-log or engine state, per `ADR-MOK-001` and `ARCH-MOK-002`.
- `REQ-MOK-025`: an observed run and an unobserved run remain identical in every authoritative event and in final
  state at every declared seed. Widening a derived map consumes no entropy and must not.
- No member of the observer's public interface is added, removed or reshaped, so `SPEC-MOK-004` rule 6's **94** items,
  **118** `pub` lines and **24** public fields stand. This is to be measured and not assumed.

  **Measured, and the second half does not hold.** The item count of **94** stands and no member is added, removed or
  reshaped, so the constraint's first clause is satisfied. The `pub` line and public field figures move to **119** and
  **25**, because `Death` gains one public field and rule 6 counts a field separately from the item declaring it. This
  constraint is left as written rather than edited to match the outcome: it is what the owner approved, and amendment 5
  is where the consequence is recorded.
- `SPEC-MOK-003` rule 10 item 7's rule holds: a value the engine did not compute is absent, never blank-labelled and
  never zero-filled.
- No existing test is renamed or removed. `SPEC-MOK-004` rule 12 governs a rename and rule 11 makes a loss a defect.
- The death line must not truncate a value at any viewport that presents the inspector, and rule 5's pane thresholds
  are not moved to make room.

## Expected change surface

| Component | Change |
|---|---|
| `SPEC-MOK-003` | rule 9 item 2's figure; rule 10 items 6 and 7; two amendment-record rows |
| `SPEC-MOK-004` | rules 9, 10 and 11 recorded figures; one amendment-record row covering both work orders |
| `VER-MOK-005` | one case, by the assurance owner's amendment |
| observer state | `Death`'s fields, the `latest_survival` map and two arms of `Observer::ingest` |
| observer presentation | `inspector_lines`' death branch |
| observer tests | the public tier's inspector cases, and the internal tier only if a case requires it |
| engine | **none.** No file of `mokiterions-core` changes, so its recorded total of 85 is unmoved |

## Required verification

`VER-MOK-005` as amended, and specifically:

1. A dead subject's pane presents the engine's last reported `fear` for that subject, at the value the engine
   reported, in rule 4 clause 5's attribute order.
2. A dead subject for whom the engine reported no `survival_changed` record presents no `fear` field at all —
   asserted as an absence and not as a zero.
3. The selection behaviour of rule 10 item 6 is otherwise unchanged: the death, the tick of death, the retained
   selection and the identity heading are as they were.
4. Rule 9 item 2's corrected figure agrees with `EventType::ALL`, whose length the observer's existing sweeps already
   assert; no new assertion is required for the figure itself, and if one is added it belongs to the public tier.
5. `REQ-MOK-025`'s non-perturbation property at every declared seed, which the existing contract already requires and
   which this change must not move.
6. The full declared gate set at the candidate: `cargo test --workspace`, `cargo fmt --all -- --check`,
   `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`,
   `cargo tree -p Mokiterions -e normal --locked --offline`, `validate_engineering_artifacts.py`,
   `check_engineering_harness.sh`, `check_declared_dependencies.py`, the dashboard, the inspector, and `se_harness`
   `validate` and `preflight --phase review` under the pinned `0.4.0` venv. `--locked` is not optional.

## Evidence to record

Under `evidence/WO-MOK-018/`:

- `test-census.md` — the per-target census before and after, on the implementing tree, reconciled name by name, with
  each arrival attributed to `WO-MOK-016` or to this work order. This is what amendment 3's figures are measured from
  and it is the file that makes the joint correction auditable.
- `interface.md` — the enumeration establishing that rule 6's 94 items, 118 `pub` lines and 24 public fields are
  unmoved.

  **The enumeration was made and it establishes the opposite of what this brief predicts.** 94 items stands; the `pub`
  line and public field figures read 119 and 25. The file records the growth, the two errors in this work order that
  produced the wrong prediction, and the three alternatives measured and worse. The brief is left as written because it
  is what the owner approved, and amendment 5 is where the consequence is recorded.
- `inspector.md` — the rendered death line at the reference viewport and at the smallest viewport presenting the
  inspector, in both the reported-`fear` and the no-record cases.
- `non-perturbation.md` — the observed-versus-unobserved comparison at every declared seed.
- `gates.txt` — the declared gate commands and their exit codes, in the forms named above.
- `filter-vocabulary.md` — the fourteen core types plus `action_trace` as `SPEC-MOK-001` names them, against
  `EventType::ALL`, establishing amendment 1's figure by measurement rather than by count-in-prose.
- `completion-report.md` — in the format below.

## Stop and escalate conditions

Stop and escalate if:

- closing defect 2 requires a member of the observer's public interface, a new engine interface item, or any change to
  `AgentSnapshot` — none is authorized, and each would move `ARCH-MOK-002`'s boundary rather than merely be covered
  by it;
- the death line cannot present four values at any viewport that presents the inspector without truncation, which
  would make this a rule 5 layout question the owner has not been shown;
- amendment 2's narrowest form turns out not to satisfy rule 10 item 6 — for instance if the item cannot be read as
  admitting a value for a dead subject that it does not admit for a living one — in which case the choice deferred by
  *Out of scope* is no longer deferrable and returns to the technical owner;
- the `SPEC-MOK-004` figures cannot be stated jointly for both work orders on one tree, or the measured total differs
  from `VREC-MOK-017`'s 264 before this work order's arrivals, either of which means the census rests on a tree that
  is not the one assumed here;
- any existing test must be renamed or removed to accommodate the change;
- `REQ-MOK-025`'s non-perturbation property moves at any declared seed;
- a figure this work order corrects is found also to be stated in an artifact this work order does not select.

## Completion report format

1. What was implemented, against each in-scope item, with the item that was not reached named if any.
2. Each amendment as applied, with the acting owner and the date, and any provision the amendment text foretold that
   the tree did not bear out.
3. The test census: totals before and after, arrivals attributed per work order, and the statement that none departed.
4. Rule 6's three interface figures, measured.
5. The gate commands and their exit codes in the declared forms.
6. Every consequence the implementation derived rather than decided, named individually and marked as awaiting
   ratification where it is not covered by an act already taken.
7. Findings carried rather than closed, each with the work order or requirement that inherits it.
8. What this work order does not claim.
