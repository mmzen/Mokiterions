+++
id = "REQ-MOK-061"
type = "requirement"
title = "Present a selected Mokiterion's cumulative activity as totals over records the engine stated"
status = "draft"
owners = ["product owner"]
created = "2026-08-22"
updated = "2026-08-22"
statement = "WHEN an operator selects a Mokiterion, THE SYSTEM SHALL present, beside the decision record of the most recently completed tick, one cumulative total for each action kind the engine applied for that Mokiterion since the run began, one for the proposals the engine rejected, one for the territory crossings it made, and one for the Mokiterions that died of a strike it resolved — every total being a sum over records the engine itself stated, and no total being presented for a quantity no engine record states."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: Present a selected Mokiterion's cumulative activity as totals over records the engine stated

## Rationale

The inspector answers *what is this Mokiterion doing now*. `REQ-MOK-021` fixes that scope exactly — the proposal of
the most recently completed tick and the engine's decision on it — and it is the whole of what the pane says about
behaviour. Nothing in the observer answers *what has this Mokiterion been doing*, and after `CAP-MOK-010` that is the
question the instrument is most often put to: a Mokiterion that attacks at every opportunity and one that has attacked
once are indistinguishable in this pane, and they are the two profiles the phase exists to produce.

The operator can in principle recover the answer today by filtering the log to the subject and counting lines by eye.
That is not an answer for three reasons, and the third is disqualifying. It is manual, so it is not repeatable; it is
slow enough that the run has moved on before it finishes; and the log is a ring buffer of `100_000` records that drops
the oldest when full and marks itself `truncated`, so beyond that point counting the retained records returns a number
that is confidently wrong. `SPEC-MOK-003`'s *State model* is explicit that dropping a record "loses presentability,
never authority" — which is true of presentation and false of arithmetic. A total has to be accumulated as the records
arrive or it cannot be stated at all.

**This requirement is the fill `CAP-MOK-004` promised rather than a widening of it.** That capability's boundaries
already name this content:

> **Only values the engine computes.** Where the eventual design shows a quantity the engine does not yet produce —
> fear, kills, combats, remembered places, model latency, per-agent names — the capability shows nothing there. It does
> not substitute a placeholder that reads as a real zero, and it does not derive a proxy. Those panes fill when the
> phases that create the data land.

Three of the six named quantities have since been produced and presented — `fear` under `CAP-MOK-006` and `WO-MOK-018`,
per-agent names under `CAP-MOK-008` — and `CAP-MOK-010` landed the conflict facts that make kills and combats
statements the engine now makes. The condition the boundary set is met for those two, and unmet for age, remembered
places, model latency and per-agent entropy, which stay absent.

**The load-bearing distinction is between a total and a proxy**, and the artifact graph has already drawn it. `SPEC-MOK-003`
rule 10 item 7's 2026-08-20 re-check holds that the count of attacks a Mokiterion has suffered is a quantity *the engine
computes*, on the stated ground that it "[is] carried by `SPEC-MOK-001`'s `attack_resolved` events", even though no
engine field holds it and no engine method returns it. A count over engine records is therefore already governed as an
engine-computed value in this specification, and this requirement claims nothing broader. It claims the converse too, in
the same words the capability uses: a figure the observer would have to *estimate*, *interpolate* or *infer from a
correlate* is a proxy and stays out. Addition reports nothing the engine did not already state; it only states it once
instead of a thousand times.

The alternative — per-Mokiterion counters kept by the engine and published on `AgentSnapshot` — is more authoritative
and is deliberately not chosen here. The engine already keeps run-level cumulative counters under `SPEC-MOK-006`'s
*State model*, and their own governing comment records why they stay unreachable: they are private, the type exposes no
accessor for any of them, and it is that privacy which means "`SPEC-MOK-002` rule 6 needs no relaxation to admit them".
Publishing per-Mokiterion equivalents would widen the engine's public interface, which rule 6 admits only for an
approved requirement, and would move a presentation concern into the component this repository treats as the authority.
The presentation need does not justify that trade. It is recorded here as the option rejected, not as an option
overlooked, and it remains open to a later initiative that needs these figures in an export or a release gate.

## Preconditions and trigger

- The observer is running under `REQ-MOK-019` and a Mokiterion is selected, living or dead.
- At least one tick has completed. Before that, the boundary behaviour below applies rather than this response.
- The inspector pane is present at the current viewport or is opened as the overlay `SPEC-MOK-003` rule 5 provides.

The trigger is the selection, exactly as in `REQ-MOK-021`, and this requirement adds no control, no key binding and no
mode. It is additional content in a pane an operator already reaches the same way.

## Required response

For the selected Mokiterion the observer presents, as integers, each total below, accumulated from the first completed
tick of this run to the most recently completed one:

| Total | Accumulated from | Counted when |
|---|---|---|
| one per action kind, for each of the eleven kinds | the decision record of each completed tick | the engine applied that kind for this Mokiterion |
| rejected proposals | the same record | the engine stated a rejection ground instead of applying an action |
| territory crossings | the authoritative event stream | a `territory_crossed` event names this Mokiterion as its subject |
| Mokiterions killed | the authoritative event stream | an `attack_resolved` event names this Mokiterion as its subject and reports that the target died |

1. **The eleven action kinds are the closed contract's own**, and all eleven are presented rather than a chosen subset:
   `wait`, `sleep`, `eat`, `move`, `attack`, `threaten`, `fight`, `retreat`, `surrender`, `approach` and `avoid`. A
   subset would be an editorial judgement about which behaviours matter, which is exactly the judgement a profile is
   meant to let the operator make.
2. **An action kind is counted when it was applied, not when it was proposed.** A rejected proposal applies nothing,
   so it is counted once, in the rejected total, and in no verb total. The eleven verb totals and the rejected total
   therefore sum to the number of decision opportunities the observer has seen for this Mokiterion, and that identity
   is the requirement's own arithmetic rather than a coincidence to be checked for.
3. **Killed counts deaths, not strikes.** Both `attack` and `fight` resolve through one engine resolution and one
   event type, and that event does not distinguish the two verbs; the two verbs are distinguished by their own totals
   above. The killed total is the subset of those resolutions in which the target died, which the engine marks in the
   event rather than leaving to be inferred from a following `agent_died`.
4. **Crossings are counted from the event and not from the `move` total**, because a crossing is a consequence of a
   move and not a verb: most moves cross nothing, and one move may cross.
5. **The count of decision opportunities is presented with the totals**, so every total is readable against a
   denominator without the observer computing a rate.
6. **Every figure is an integer count.** No average, mean, ratio, percentage or floating-point value is presented,
   on `CAP-MOK-009`'s own exclusion and for its reason: a ratio is a claim about the population's behaviour rather
   than a record of it, and its threshold is a decision no pane should make silently.
7. **A total is presented as zero where the engine stated the records and none of them matched.** This is a
   measurement and not a placeholder, and it is the case `SPEC-MOK-003` rule 10 item 7 does not govern: that item
   forbids zero-filling a field for a value the engine does not compute, and a Mokiterion that has never attacked
   has an attack total the engine's records determine exactly.

## Failure and boundary behavior

1. **Before tick 1 completes** the pane states that no tick has completed, in place of the totals. It does not
   present fourteen zeros, which would be indistinguishable from a Mokiterion that had acted a thousand times and
   done nothing — the same reason `REQ-MOK-021` is given clause 4 of rule 10 rather than an empty decision record.
2. **When the selected Mokiterion is dead** the totals are presented unchanged and stop advancing. They are the
   profile of a completed life and are the one part of this pane that is more informative after death than before it,
   so they are retained under rule 10 item 6's retained selection rather than cleared with the roster entry.
3. **When the event buffer has truncated**, every total is unaffected and no total is qualified, because no total is
   computed from the buffer. The observer's `truncated` indicator continues to describe the log's presentability only.
4. **A quantity no engine record states is absent**, not zero and not blank-labelled. Age, remembered locations,
   model latency and per-agent entropy stay out of this pane on `SPEC-MOK-003` rule 10 item 7 unchanged, and the
   record of attacks *suffered* stays out of it on the separate ground the 2026-08-20 re-check gives it, which this
   requirement neither reads nor disturbs.
5. **A pane too narrow to present a total does not present it clipped.** `SPEC-MOK-003` rule 10 item 6 settled the
   principle for this pane in the terms this requirement inherits — a value clipped off the pane is not a value
   presented — and the specification fixes how the totals are arranged so that it holds at the reference viewport.

## Constraints

1. **No engine change.** The engine's public interface, its rules, its event stream, its text stream, its entropy
   draws and its configuration are untouched. Every figure is obtained from the read-only observation surface
   `SPEC-MOK-003`'s *Data and interface contracts* already fixes.
2. **No new dependency.** The observer's declared dependency set is unchanged.
3. **No total reaches a decision.** No total, and no value derived from one, is placed in an `Observation`, passed to
   a `DecisionSource`, or read by any engine rule. `REQ-MOK-059` forbids a rule or a source reading any aggregate over
   the population and `REQ-MOK-062` introduces exactly such an aggregate on the observation side; the two are
   compatible only because the dependency direction is one-way, and this clause is what makes that a stated obligation
   rather than a property of the current file layout.
4. **`REQ-MOK-025` is preserved.** The accumulation happens on the observer's side of the boundary, consumes no
   entropy, and changes no tick. An observed run's event sequence and final state remain identical to the unobserved
   run's on every declared seed.
5. **Nothing is persisted.** The totals are presentation state under `SPEC-MOK-003`'s *State model*: they are not
   written to the export, which stays the rule 9.4 text format of retained events, and they do not survive the
   process.
6. **No distinction carried by colour alone**, per rule 2 clause 5, which every pane in this specification obeys.

## Acceptance examples

### Example: normal behavior

**Given** an observed run at the reference viewport, seed `42`, with a decision source that proposes `attack` whenever
it has a target and `move` otherwise

**When** the operator selects `M03` on tick `214`, having let the run advance from tick `1` without pausing

**Then** the inspector presents `M03`'s current-tick decision record exactly as `REQ-MOK-021` requires it, and beneath
it fourteen integer totals and the count of decision opportunities; the eleven verb totals and the rejected total sum
to that count; the `attack` total equals the number of `attack_resolved` events in the retained log whose subject is
`M03`, minus those attributable to its `fight` total; and the killed total equals the number of those events that
report the target as having died.

### Example: normal behavior at a boundary the phase created

**Given** the same run

**When** `M03` dies of starvation on tick `260` and the operator does not change the selection

**Then** the pane presents the death, the tick of death and the four final attributes under rule 10 item 6, and
presents `M03`'s fourteen totals unchanged from tick `259`, with no total advancing on any later tick and no total
removed.

### Example: failure behavior

**Given** an observed run started with `--start-paused`

**When** the operator selects `M01` before advancing a single tick

**Then** the pane states that no proposal has yet been made, as rule 10 clause 4 already requires, and states that no
tick has completed in place of the totals — presenting no zero, no blank label and no field for any of the fourteen
figures.

## Open decisions

None. Two decisions were taken in drafting and are recorded here so that a reviewer weighs them rather than
rediscovering them:

- **All eleven verbs are presented, not a subset.** The alternative was the seven the operator is likeliest to ask
  about. It was rejected because the omitted four would have been an unstated editorial claim, and because eleven
  totals fit the pane at the reference viewport, which the specification demonstrates.
- **Attacks suffered are not presented here.** They were considered, because the log pane presents the events and the
  arithmetic is identical. They are excluded because `SPEC-MOK-003` rule 10 item 7's 2026-08-20 re-check placed them
  outside this pane on a ground of its own, and overturning that reasoning is a separate decision from filling the
  two fields the same item's list anticipated. A later initiative may take it; this one does not need it.
