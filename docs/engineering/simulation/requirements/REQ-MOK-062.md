+++
id = "REQ-MOK-062"
type = "requirement"
title = "Present the population's cumulative totals where no Mokiterion is selected"
status = "draft"
owners = ["product owner"]
created = "2026-08-22"
updated = "2026-08-22"
statement = "WHILE no Mokiterion is selected, THE SYSTEM SHALL continue to state that nothing is selected and SHALL present, in place of a single Mokiterion's record, the same cumulative totals summed over every Mokiterion the run initialized, living and dead alike, together with the tick and the living count the engine reports and the count of deaths the engine attributed to a strike, and SHALL make no such total readable by any engine rule or decision source."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: Present the population's cumulative totals where no Mokiterion is selected

## Rationale

`REQ-MOK-061` gives the inspector a profile of one Mokiterion. The same accumulation answers a question
`CAP-MOK-004` names among the three it was written for and which no pane answers today:

> The **product owner** watching whether the population behaves plausibly, who needs the whole world at once.

The spatial view shows where the population is and the roster shows what each member's attributes are. Neither states
what the population has *done*, and the run summary that would state it belongs to the engine binary's text stream,
arrives only when the run ends, and is not available to an operator who is watching. A population that has attacked
four times in three hundred ticks and one that has attacked nine hundred times look identical in every pane the
observer has.

**The pane this occupies is otherwise idle.** With nothing selected, the inspector presents two lines into a pane that
has roughly thirty-six interior rows at the reference viewport. The content is not competing with anything.

**Rule 10 clause 5 is the constraint this requirement satisfies rather than the one it breaks.** That clause reads:
"With nothing selected, the pane states that nothing is selected. It never defaults to an arbitrary Mokiterion." Its
second sentence forbids a specific substitution — presenting one member's figures as though a selection existed — and a
total over every member is the one presentation that cannot be mistaken for it. The first sentence is satisfied and not
displaced: the pane still states that nothing is selected, and states it above the totals, so no reader can take the
figures for one Mokiterion's. A total that could be read as an individual's record would defeat the clause whatever
this requirement said, which is why the statement carries that obligation rather than leaving it to the specification.

**The state is reachable and already governed.** `SPEC-MOK-003` rule 7's key table binds `Esc` to "close an overlay if
one is open, otherwise clear the selection", so the operator reaches this content by a control that already exists and
already means what it would need to mean. No key binding is added, no mode is added, and rule 7 is untouched.

**At extinction the observer arrives here by itself.** When no living Mokiterion remains, the observer clears the
selection rather than retaining a selection into a state where the next control has nowhere to move. So the last frame
of a run that ends in extinction presents the population's completed totals with no operator action at all. That
consequence is worth stating in the specification as intended, because a behaviour that is valuable and undeclared is
one a later change removes without noticing.

**This requirement creates a population-level aggregate, which one approved requirement forbids in a named place.**
`REQ-MOK-059` obliges every rule, every proposal validation and every decision source to read "only the acting
Mokiterion's own state, its own observation, and the state of any individual Mokiterion the rule names", and to read
"no aggregate over the population". Until now the observer computed no such aggregate and the obligation was met by
there being nothing to leak. It is now met by a boundary instead, and the boundary is real rather than nominal: the
totals live in the observer component, the engine has no knowledge of the observer under *Data and interface
contracts* clause 3, and a decision source receives an `Observation` and returns a `ProposedAction` and can reach
nothing else. The obligation is nonetheless carried in this requirement's own statement, because the compatibility of
these two requirements is the thing most worth a reviewer's attention and the thing a future refactor is likeliest to
break.

## Preconditions and trigger

- The observer is running and `selection` is none — at start-up, after `Esc` with no overlay open, or after the
  observer cleared it because no living Mokiterion remained.
- At least one tick has completed. Before that, the boundary behaviour below applies.
- The inspector pane is present at the current viewport or opened as an overlay under rule 5.

## Required response

The pane states that nothing is selected, states the control that makes a selection, and then presents:

1. **The same fourteen totals `REQ-MOK-061` defines**, each summed over every Mokiterion the run initialized, in the
   same order and under the same labels, so that the two states of this pane read as one instrument at two scales
   rather than as two reports.
2. **The count of decision opportunities**, summed the same way, against which the totals are readable.
3. **The tick and the living count exactly as the engine reports them** in its snapshot, not recomputed and not
   derived, on the ground `REQ-MOK-041` states for the name: where the engine states a value, the presented value is
   the engine's own.
4. **The count of Mokiterions the run initialized**, counted from the engine's own initialization records.
5. **The death count the engine reports, and the number of those deaths the engine attributed to a strike** — the
   latter being the population sum of `REQ-MOK-061`'s killed total. The remainder is presented as the deaths the
   engine did not attribute to a strike, and is **not labelled with a cause**: the engine adds no cause field to
   `agent_died`, which is the stated reason `attack_resolved` carries a died marker at all, and naming a cause the
   engine does not name would be the proxy `CAP-MOK-004` excludes.

Clauses on the sum itself:

6. **A dead Mokiterion's totals remain in the sum.** They are not removed when it leaves the roster and not removed
   when the run ends. A total that fell as the population died would not be a cumulative total, and the figure an
   operator needs from a collapsing run is what the population did, not what its survivors did.
7. **Every figure is an integer count**, on the same ground `REQ-MOK-061` clause 6 gives: no average, ratio,
   percentage or floating-point value, and no figure per Mokiterion or per tick.
8. **A zero total is presented as zero.** The population's records determine it exactly, so it is a measurement.

## Failure and boundary behavior

1. **Before tick 1 completes** the pane states that nothing is selected and that no tick has completed, and presents
   no total — symmetric with `REQ-MOK-061` and with rule 10 clause 4, and for the same reason.
2. **After the run has finished** the totals are presented unchanged and stop advancing, and remain inspectable for as
   long as the observer runs, consistent with the *Error and recovery behavior* row that keeps a finished run
   inspectable and exportable.
3. **At extinction** the totals are presented in this state without an operator control, because the observer clears
   the selection when no living Mokiterion remains. The living count presented is the engine's own zero.
4. **When the event buffer has truncated** no total is affected and no total is qualified, because no total is
   computed from the buffer.
5. **When the viewport excludes the inspector**, this content is reachable as the rule 5 overlay by its bound key and
   is not silently lost, exactly as the pane's other content is.

## Constraints

1. **No total, and no value derived from a total, is readable by any engine rule, any proposal validation or any
   decision source.** This is `REQ-MOK-059`'s obligation restated at the point where an aggregate now exists. It is
   discharged structurally — the aggregate is held in the observer, the engine does not depend on the observer, and
   no engine entry point accepts it — and it is verified by the run-identity property rather than asserted.
2. **`REQ-MOK-025` is preserved.** Summing consumes no entropy and changes no tick.
3. **No engine change and no new dependency**, as `REQ-MOK-061`.
4. **The export is unchanged.** The totals are presentation state and do not enter the rule 9.4 export, which stays
   the retained event stream in its text format. An operator needing the figures in a file has the engine's own
   structured record stream under `SPEC-MOK-006`, which `CAP-MOK-009` built for that purpose and which excludes
   observer output for that reason.
5. **The state is reached by the existing control only.** No key binding is added, and the selection is never cleared
   by this requirement — clearing remains rule 7's `Esc` and the observer's own extinction behaviour.
6. **The pane never presents the totals without the statement that nothing is selected.**

## Acceptance examples

### Example: normal behavior

**Given** an observed run at the reference viewport with twelve Mokiterions, advanced to tick `300`, with `M04`
selected

**When** the operator presses `Esc` with no overlay open

**Then** the pane states that nothing is selected, states the control that selects, and presents the fourteen totals
summed over all twelve Mokiterions, the summed decision-opportunity count, the tick `300` and the living count as the
engine reports them, the initialized count `12`, and the death count split into those the engine attributed to a
strike and those it did not — and each summed total equals the sum of the twelve per-Mokiterion totals
`REQ-MOK-061` presents for the same tick.

### Example: normal behavior at extinction

**Given** the same run with `M07` selected and one living Mokiterion remaining

**When** that Mokiterion dies

**Then** the observer clears the selection, and the pane presents the population's completed totals with a living
count of `0`, without the operator pressing a key.

### Example: failure behavior

**Given** an observed run started with `--start-paused` and nothing selected

**When** the operator reads the inspector before advancing a tick

**Then** the pane states that nothing is selected and that no tick has completed, and presents no total, no zero and
no blank-labelled field.

## Open decisions

None. One decision was taken in drafting and is recorded rather than left implicit:

- **The population totals replace the idle content of this pane; they are not shown beside a selected Mokiterion's.**
  Presenting both at once was considered and rejected on width: the inspector's interior is 42 columns at the
  reference viewport, the pairing rule 10 item 6 already uses exists because four word-labelled values exceed it, and
  a third column of figures would either clip or force the per-Mokiterion totals into a form no longer readable
  against the population's. The two states of the pane are the two scales, and `Esc` is the control between them.
