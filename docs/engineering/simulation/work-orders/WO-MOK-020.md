+++
id = "WO-MOK-020"
type = "work_order"
title = "Give the inspector a cumulative activity profile per Mokiterion and a population total where nothing is selected"
status = "approved"
owners = ["engineering owner"]
created = "2026-08-22"
updated = "2026-08-22"

[assurance]
commit_bound_verification = "required"
rationale = "The scope is mixed, and `WORKFLOW.md` forbids inferring a classification for mixed scope, so the higher classification governs. Two parts are executable behaviour on the instrument every later phase is assessed with: the observer begins retaining derived per-Mokiterion state across ticks, and the inspector presents fourteen figures no pane has presented before. Neither load-bearing claim can be asserted from a diff. That each total equals an independent count over the engine's own records is a claim about arithmetic accumulated inside the single advance path, not about a value read from the read-only surface, and an off-by-one at a tick boundary or a double count on the final tick is invisible to review and indistinguishable from a correct figure on screen. That a population-level aggregate now computed in this process is unreachable from any rule or decision source is `REQ-MOK-059`'s obligation met by a boundary rather than by absence, and a boundary claim needs the run-identity property measured rather than argued. The work also amends `SPEC-MOK-004`'s recorded observer interface extent and test-tier figures, which are trusted engineering state that later censuses read, and it corrects two recorded statements in `SPEC-MOK-003` that the implementation has already outgrown, so a figure wrong here is wrong for every artifact that reads it. `REQ-MOK-025`'s non-perturbation obligation is in scope by construction, because the change widens what the observer accumulates from the event stream."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-061", "REQ-MOK-062"]
specifications = ["SPEC-MOK-003", "SPEC-MOK-004"]
verification = ["VER-MOK-017"]
+++

# Work Order: A cumulative activity profile in the inspector

## Lifecycle

This work order carries executable behaviour and is classified `commit_bound_verification = "required"` above. It
stops at `implemented` when the code, the specification amendments and the retained evidence are complete; it reaches
`verified` only through an eligible commit-bound verification record against `VER-MOK-017`, and `released` only through
a release record that covers it. Neither transition is this work order's to make.

**The `architecture` relation is deliberately absent, and the omission is reasoned rather than inherited.** No active
architecture carries an `addresses` edge to `REQ-MOK-061` or `REQ-MOK-062`, and the work order template admits omission
in exactly that case while forbidding fabricated coverage. `ARCH-MOK-002` is the observer's architecture and is relied
upon here — `REQ-MOK-062`'s safety argument rests on its one-way dependency direction — but nothing in this work order
moves what that architecture decides. Its own amendment record states the triggers: a dependency edge, a trust
boundary, a non-perturbation property, a framework selection, a target shape. This change moves none of them. It adds
no component, no boundary, no interface, no dependency and no engine public item, and it is confined to the
presentation layer of a package whose shape is already decided. Growing `ARCH-MOK-002`'s `addresses` list is itself a
governed architectural amendment requiring an ADR, as its 2026-08-18 row records, and incurring one to describe a
presentation change would be the fabrication the template names.

## Objective

Make a Mokiterion's history legible in the pane that already shows its present, and make the population's history
legible in the same pane when no Mokiterion is selected — using only totals over records the engine has already
stated, without touching the engine, and without the aggregate becoming readable by anything that decides.

Implement `REQ-MOK-061` and `REQ-MOK-062`, and amend `SPEC-MOK-003` and `SPEC-MOK-004` so that both are governed
before they are conformed to.

## In scope

### 1. Accumulation in the observer

- Retained presentation state, per Mokiterion identifier, holding one count per action kind for the eleven kinds of
  the closed contract, a rejected-proposal count, a territory-crossing count, a killed count, and a
  decision-opportunity count. A Mokiterion's record is created when the engine's initialization records name it and is
  never removed, so a dead Mokiterion's totals stay available to `REQ-MOK-061` clause 2 of *Failure and boundary
  behavior* and stay in `REQ-MOK-062`'s sum.
- Accumulation performed **inside the observer's single advance path**, once per completed tick, alongside the
  accumulation that path already performs for names, last reported survival values and retained deaths. Two sources
  are read there and each for what only it carries: the refreshed snapshot's decision records supply the verb and the
  rejection, and the tick's authoritative events supply crossings and kills.
- **No design that recomputes a total by scanning the retained event buffer.** That buffer has capacity `100_000`,
  drops its oldest record when full and marks itself `truncated`; a total computed from it would begin returning a
  confidently wrong figure part-way through a long run, which is the failure mode a profile pane can least afford.
- Population totals obtained by summing the per-Mokiterion records, not accumulated a second time. One accumulation
  with one summation cannot disagree with itself; two accumulations can.

### 2. Presentation in the inspector

- With a Mokiterion selected: the fourteen totals and the decision-opportunity count, beneath the existing
  decision record, presented as integers under short labels arranged so that every figure is inside the pane's
  interior at the reference viewport.
- With nothing selected: the statement that nothing is selected and the control that selects it, both retained and
  both above the totals, then the population sums, the engine's own tick and living count, the initialized count, and
  the death count split into deaths the engine attributed to a strike and deaths it did not.
- Before the first completed tick, in both states: a statement that no tick has completed, and no figure.

### 3. `SPEC-MOK-003` amendments

- **Rule 10's presented-value list** gains the totals for the selected-subject case.
- **Rule 10 clause 5** amended for the no-selection state: the statement that nothing is selected is retained and the
  prohibition on defaulting to an arbitrary Mokiterion is restated as satisfied rather than waived.
- **Rule 10 item 7** amended by the procedure its own 2026-08-19 amendment established: `kills` and `combats` leave the
  list of values the engine does not compute, because `CAP-MOK-010` landed the records that state them and because
  this item's 2026-08-20 re-check already holds that a count carried by `attack_resolved` is computed by the engine.
  Age, remembered locations, model latency and per-agent entropy stay named. **The suffered-attack record and the
  count of attacks suffered keep their own ground untouched and unread**, as `REQ-MOK-061`'s *Open decisions* records.
  The amendment must state, not leave to be inferred, why a zero total is a measurement here while a zero for a value
  the engine does not compute remains forbidden.
- **Rule 10** gains the extinction consequence as declared behaviour: the observer clears the selection when no living
  Mokiterion remains, so a run ending in extinction presents the population's completed totals with no operator act.
- **Rule 11** gains the authority mapping for the new content, beside the existing sentence that maps the
  proposal-and-outcome presentation to `REQ-MOK-004`. The mapping names identifiers only, per clause 1.
- **The *State model* table** gains the retained per-Mokiterion record with its domain and initial value.
- **Two recorded statements the implementation has already outgrown are corrected in the same pass**, because this
  work order amends both places and a wrong statement left standing beside a new one reads as ratified:
  1. The *State model* table declares ten fields and declares none of the derived retention the observer already
     performs — the name map filled from `agent_initialized`, the last reported survival values, and the retained
     deaths that rule 10 item 6 presents. The table is the specification's statement of what the observer holds, and
     it is incomplete rather than wrong. The correction declares what is held today as well as what this work order
     adds, so that the table can be read as exhaustive.
  2. Rule 4's naming paragraph states that "the observer holds no name table and no identifier-to-name derivation".
     The second clause is true and is the load-bearing one; the first is not true of the implementation, which holds a
     map filled from the engine's own records. The sentence's subject was derivation, and it is corrected in those
     terms — the observer derives no name and retains the engine's — rather than by changing an implementation that
     does the right thing. `REQ-MOK-041` is not affected: no presented name is one the engine did not report.
- ***Performance and capacity*** gains the bound on the retained state, which is one record per initialized
  Mokiterion and is therefore fixed by the population rather than growing with ticks.
- One row in the **Amendment record**, in the existing three-column form, stating every change above and its approval.

### 4. `SPEC-MOK-004` amendments

- Re-measure and amend the recorded extent of the observer's public interface under rule 6 and the recorded test
  counts per tier under rule 9. Both are recorded as exact figures that later censuses read, and both move if this
  work order adds a public item or a test. Every figure is to be **measured against the implementation at the
  candidate commit and not projected**, and the amendment states what it counts where the arithmetic is not obvious,
  as the 2026-08-19 row did.
- One row in that specification's amendment record.

### 5. Tests

New tests in the observer's existing tiers, discovering their placement from `SPEC-MOK-004` rule 9 rather than
choosing it, covering at minimum every case `VER-MOK-017`'s matrix names.

## Out of scope

- **Any engine change.** No rule, no event, no text-stream byte, no entropy draw, no configuration default, no public
  item and no dependency of `mokiterions-core` moves. Per-Mokiterion counters kept by the engine were considered and
  rejected in `REQ-MOK-061`'s rationale; adopting them here would relax `SPEC-MOK-002` rule 6 without an approved
  requirement.
- **Attacks suffered**, and any reopening of the ground `SPEC-MOK-003` rule 10 item 7's 2026-08-20 re-check gives for
  keeping them out of this pane.
- **The export.** Rule 9.4's export stays the retained event stream in its text format, and no total enters it.
- **The structured record stream.** `SPEC-MOK-006` is untouched; `CAP-MOK-009` excludes observer output by design.
- **Any new key binding, control, mode or pane.** `Esc` already clears the selection under rule 7.
- **Any average, ratio, percentage or floating-point figure**, and any per-tick or per-Mokiterion normalisation.
- **`ARCH-MOK-002`**, for the reasons under *Lifecycle*.
- **Any status transition on any artifact**, including the two requirements this work order implements. Approval is
  the accountable owner's act and is not performed by an implementation.
- **`docs/ROADMAP.md`.** The roadmap is repository-owned functional planning and is reconciled separately; this
  initiative sits outside the phase sequence and its roadmap entry is owed under its own change.

## Authorized decision envelope

The implementation may decide locally, and must record what it decided in its completion report:

- The exact label text, the ordering of the fourteen figures, and their arrangement into columns, within the
  obligations the amended rule 10 states and subject to every figure being inside the interior at the reference
  viewport.
- The integer width of each counter and its overflow discipline, provided a total cannot wrap. Saturating arithmetic
  is acceptable and matches the engine's own cumulative counters; a width at which saturation is unreachable for any
  admissible run is preferable to relying on it.
- Module placement, type and field names, and whether the record is one struct with named fields or an indexed array
  over the action kinds, provided no action kind can be silently omitted when the contract grows.
- Test names, and the choice of seeds among those the declared set already carries.
- The wording of the "no tick has completed" statement and of the retained "nothing is selected" statement.

The implementation may **not** decide: which figures are presented, whether a zero is presented, whether a dead
Mokiterion stays in the sum, where the accumulation happens, whether a total reaches the export, or the substance of
any specification amendment listed above.

## Constraints

1. `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` is clean. `--locked` is not
   optional: the lock file is part of the declared dependency set under `ADR-MOK-006`.
2. `cargo test --workspace --locked` passes, and the pre-existing failure profile of this platform is stated rather
   than silently absorbed.
3. `cargo fmt --check` is clean, and the toolchain is the pinned one.
4. The observer's declared dependency set is unchanged, and no `Cargo.toml` gains an entry.
5. `grep -n 'pub fn .*&mut self' mokiterions-core/src/simulation.rs` returns exactly `run` and `advance_tick`, as
   *Data and interface contracts* clause 2 requires it to.
6. No total, and nothing derived from one, is passed to an engine entry point, placed in an `Observation`, or reached
   by a `DecisionSource`. The dependency direction stays one-way.
7. Retained evidence is written under `docs/engineering/simulation/evidence/WO-MOK-020/` with a digest manifest, and
   the `-text` attribute on that tree means the retained bytes must be produced with the line endings the manifest
   hashes.
8. No formal artifact outside the four this work order names is edited.

## Expected change surface

| Component | Expected change |
|---|---|
| observer presentation state | the retained per-Mokiterion record and its accumulation in the single advance path |
| observer inspector rendering | the selected-subject block and the no-selection block |
| observer test tiers | new cases in the tiers `SPEC-MOK-004` rule 9 assigns them to |
| `SPEC-MOK-003` | rules 4, 10 and 11, the *State model* table, *Performance and capacity*, one amendment row |
| `SPEC-MOK-004` | rule 6 and rule 9 recorded figures, one amendment row |
| engine | none |

Files are not named because the implementation is expected to locate them from `SPEC-MOK-004` rule 1's authoritative
tree rather than from this work order's guess.

## Required verification

`VER-MOK-017`. Every oracle in its matrix executes and passes, and every manual assessment it names is recorded and
signed by the accountable role before the chain can be presented as verified. The two properties this work order
cannot be accepted without are:

- **Totals equal an independent count.** For a declared seed and tick count, each per-Mokiterion total equals a count
  computed independently from the engine's own records, and each population total equals the sum of the
  per-Mokiterion totals.
- **The run is unperturbed.** The observed run's event sequence and final state are identical to the unobserved run's
  on every declared seed, and the per-tick entropy draw counts are identical.

## Evidence to record

- Command transcripts for clippy, fmt and the full test run, before and after.
- The independent-count comparison for at least the declared seeds, as retained files with a digest manifest, in a
  form a reviewer can recompute rather than a summary of a comparison already made.
- Frame captures of both pane states at the reference viewport: a selected living Mokiterion, a selected dead one, the
  no-selection population state, the pre-tick-1 state in both selection states, and the extinction frame.
- A column measurement for the widest presented line in each state, against the pane's interior width, so the
  no-clipping obligation is a measured figure and not a claim.
- The measured `SPEC-MOK-004` figures with the commands that produced them.
- A statement of anything the contract asks for that was met by substitution, in the terms `WO-MOK-019`'s record used,
  rather than a bullet quietly reinterpreted.

## Stop and escalate conditions

Stop, record what was measured, and escalate rather than deciding, when any of these hold:

1. A total cannot be made to equal an independent count for any reason other than a defect in the implementation —
   for example if a decision record proves not to be present for every Mokiterion on every completed tick, which
   would make `REQ-MOK-061` clause 2's identity unattainable as stated and is a requirement question, not a coding one.
2. The figures cannot be arranged inside the interior at the reference viewport without dropping one. Dropping a
   figure is outside the decision envelope.
3. Any specification amendment above cannot be written without also amending an artifact this work order does not
   name.
4. `REQ-MOK-025` cannot be demonstrated, or any measured difference appears between an observed and an unobserved run.
5. The `SPEC-MOK-004` re-measurement contradicts a figure another approved artifact records, which would mean a defect
   older than this work order.
6. Any part of the scope turns out to require an engine change.

## Completion report format

1. What was implemented, in the terms of the two requirements, with the decisions taken inside the envelope named.
2. Every specification amendment made, by rule and clause, with the two corrections of §3 reported separately from the
   additions so a reviewer can see what was fixed as distinct from what was added.
3. The measured figures: the `SPEC-MOK-004` counts, the widest line per pane state against the interior width, and the
   independent-count comparison result per seed.
4. Command results for every constraint above, quoted.
5. The evidence packet's path, file count and manifest digest.
6. Anything not done, anything met by substitution, and anything found and deferred — each stated as such, with the
   artifact that would carry it.
