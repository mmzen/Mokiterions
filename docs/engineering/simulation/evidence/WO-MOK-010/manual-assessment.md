# WO-MOK-010 manual assessments

`VER-MOK-010` names seven judgements that no measurement can make: "Each of the following is an explicit judgement
recorded by the accountable role. An unrecorded assessment is an outstanding assessment, and this contract is not
satisfied while any remains outstanding."

**The judgements are the owner's; the writing is the agent's.** This file was written by the implementation agent,
which under `ENGINEERING_HARNESS.md` may draft and implement but may never self-approve. What it does is put each
judgement in front of the role that owns it with the measured facts already assembled, and then record the answer that
role gave, in the role's own terms. Every decision below was taken by the repository owner, in the role named against
it; five of the seven were taken in the closing review of 2026-08-19, which `closing-review.md` records as the act and
which put each question separately so that no answer covered a second judgement.

**Status: seven of seven recorded.** `VER-MOK-010`'s manual-assessment contract is **satisfied**. Five judgements were
outstanding until the closing review of 2026-08-19; one was recorded earlier that day and is reaffirmed there against a
measurement that arrived after it; one was recorded in substance in an approved specification and is now signed as an
assessment in its own right. Two of the seven carried facts that point adversely, and both are recorded as decided on
those facts rather than around them: the divergence count of assessment 2 and the 39% ceiling residency of
assessment 4.

Every figure below is quoted from a file in this evidence pack and is reproducible from the commands those files
record. Nothing here is a new measurement.

| # | Judgement | Accountable role | Status | Decision |
|---|---|---|---|---|
| 1 | Scarcity at the default density | product owner | **recorded 2026-08-19** | satisfied, no reservation |
| 2 | Individuality is meaningful | product owner | **recorded 2026-08-19** | satisfied on the measure the contract names; the eats corroborate and are not substituted for it |
| 3 | The accumulation result | product owner | **recorded 2026-08-19** | neither an improvement nor a regression |
| 4 | `fear`'s constants | technical owner | **recorded 2026-08-19** | keep `+10`/`-5` as approved; the adverse figure recorded as an observation |
| 5 | Roster legibility at four bars | technical owner | **recorded 2026-08-19**, reaffirmed the same day | the narrowing stands, and oracle 4's sweep is noted |
| 6 | The projection | assurance owner | **recorded 2026-08-19** | confirmed: the three patterns and their ordering delete only the additions |
| 7 | The absence of a `fear` consumer | technical owner | **recorded 2026-08-19**, having been recorded in substance earlier that day | signed as an assessment in its own right |

---

## 1. Scarcity, by the product owner — **recorded 2026-08-19**

**What was to be judged.** `VER-MOK-010`: at the default density under the new source, not all declared seeds retain
twelve survivors; twelve everywhere would satisfy `REQ-MOK-034`'s literal floor and contradict `INT-MOK-002`'s
scarcity principle, and would have to be reported as adverse rather than passed over.

**The measurement** (`measurements/viability.txt`, 1,000 ticks, density 0.75%, the only density carrying a floor):

| seed | 0 | 1 | 42 | 123 | 777 |
|---|---|---|---|---|---|
| survivors, `individual` | 11 | 9 | 9 | 10 | **12** |
| survivors, `reference` | 8 | 11 | 8 | 9 | 11 |

**What the figures say.** The adverse condition this assessment guards against did not arise: twelve survive on one
declared seed of five, not on all five, and deaths occur on four. `REQ-MOK-034`'s floor of eight of twelve holds on
every seed under both sources. Neither territory reached zero standing resources on any seed under either source, so
scarcity is a pressure on the population rather than a collapse of the world.

Two facts belong in the same view rather than in a footnote. Seed 777 is the one seed with no deaths at 1,000 ticks —
and it is also the trait-aware source's only extinction at 10,000 ticks, at tick 9,938 (`measurements/long-horizon.txt`).
And the survivor distribution is a consequence of the owner's own decision of 2026-08-19: at the `0..=100` range first
specified, the floor was missed on three of five seeds, and narrowing to `0..=40` is what brings it back. The
distribution above is the one that bound was chosen on, so the choice is upstream of these numbers and not independent
of them.

**The decision, 2026-08-19, by the repository owner acting as product owner: satisfied, with no reservation
recorded.** Put the question of whether 11, 9, 9, 10 and 12 survivors reads as the scarcity `INT-MOK-002` intends, given
that the trait-aware source keeps more Mokiterions alive at this horizon than the reference source does on three of the
five seeds, the owner recorded that the adverse condition this assessment guards against did not arise: twelve survive
on one declared seed of five and not on all five, so scarcity is a pressure on the population rather than a collapse of
the world. The two facts above were put in the same view and neither was recorded as a reservation — the option of
recording one was offered and declined. `closing-review.md` records the act.

## 2. Individuality is meaningful, by the product owner — **recorded 2026-08-19**

**What was to be judged.** `VER-MOK-010`: reading the recorded divergences, the trait produces behavior a reader would
call different individuals rather than arithmetic noise — and, in its own words, "a change that satisfies every
automated check while producing one visible divergence per thousand ticks has not delivered `INT-MOK-006`".

**The measurement** (`measurements/divergence.txt`, the 1,000-tick traced run at the default density):

| seed | 0 | 1 | 42 | 123 | 777 |
|---|---|---|---|---|---|
| divergent situations | 10 | 3 | 3 | 5 | 3 |
| waste-accepting eats | 73 | 86 | 54 | 97 | 75 |
| distinct eaters | 9 | 9 | 9 | 10 | 9 |
| situations presented / distinct | 449/136 | 408/137 | 401/124 | 410/131 | 454/134 |
| same-tick coincidences | 0 | 0 | 0 | 0 | 0 |
| disagreements, contradictions, unseparated | 0 | 0 | 0 | 0 | 0 |

**The adverse reading, stated first.** A *divergent situation* is one where two different traits, presented with the
same resource class at the same satiety, decided differently. On three of five seeds there are **three** of them in a
thousand ticks. That is within a factor of three of the figure `VER-MOK-010` itself names as a failure. The count is
low for a structural reason and not by accident: a divergence requires two Mokiterions to meet the *same* class at the
*same* satiety at some point in the run, and 401–454 presented situations spread over 124–137 distinct ones leaves few
situations visited by traits on both sides of their threshold. There were also **zero** cases across all five seeds of
two Mokiterions facing the same situation on the same tick, so no divergence is ever visible side by side in one frame.

**The favourable reading.** Waste-accepting eats are the trait acting: 54–97 per run, by 9 or 10 distinct
Mokiterions of twelve, each one a tick at which the reference source would have declined the same resource for the
same Mokiterion. Traits are well spread — 9 to 11 distinct values of twelve agents per seed, spanning 0 to 40
(`measurements/traits.txt`) — and monotonic separation held on every divergent situation with no contradictions, so
where the trait acts it acts as rule 19 says it should. At the 10,000-tick horizon the two sources separate
decisively: the reference source goes extinct on four of five seeds, the trait-aware source reaches the tick limit on
four of five (`measurements/long-horizon.txt`).

**The decision, 2026-08-19, by the repository owner acting as product owner: satisfied on the measure `VER-MOK-010`
names.** The question put was which measure answers `INT-MOK-006` — the 3-to-10 divergent situations, which is what the
contract names, or the 54-to-97 waste-accepting eats, which is the same behavior counted without requiring a second
Mokiterion to have met the same situation. The adverse reading above was stated first and the option of recording the
assessment adversely was on the table. The owner recorded that the divergence count clears the one-per-thousand-ticks
figure the contract itself calls a failure, and that monotonic separation held on every divergent situation with no
contradictions; the eats stand as corroboration and are **not** substituted for the named measure, so no provision of
`VER-MOK-010` is amended and no more favourable measure is read into it. The zero same-tick coincidences remain on the
record as the limit of what was demonstrated, in `README.md` under *What none of this establishes*. `closing-review.md`
records the act.

## 3. The accumulation result, by the product owner — **recorded 2026-08-19**

**What was to be judged.** `VER-MOK-010`: the measured high-class share at tick 1,000 and the tick-10,000 outcome under
the new source are read against the reference source's recorded 45 of 61 and its extinction at tick 9,154, and the
owner records whether the result is an improvement, a regression or neither. `REQ-MOK-034` states no obligation on it.

**The measurement.** The recorded control reproduces exactly: territory A under `reference` at seed 0 holds 61
standing resources of which 45 are high class — the recorded 45 of 61, to the resource — and the reference source
reaches extinction at seed 123 on tick 9,154, to the tick (`measurements/viability.txt`, `measurements/long-horizon.txt`).

High-class share at tick 1,000, territory A / territory B:

| seed | 0 | 1 | 42 | 123 | 777 |
|---|---|---|---|---|---|
| `reference` | 73% / 58% | 75% / 64% | 62% / 63% | 68% / 73% | 45% / 46% |
| `individual` | 58% / 61% | 70% / 35% | 50% / 60% | 66% / 63% | 77% / 59% |

At 10,000 ticks: `reference` reaches extinction on seeds 0, 1, 123 and 777 (ticks 5,423, 8,273, 9,154 and 9,598) and
leaves one survivor at seed 42; `individual` reaches the tick limit on seeds 0, 1, 42 and 123 with 5, 4, 2 and 1
survivors, and goes extinct at seed 777 on tick 9,938.

**What the figures say.** Accumulation of unwanted high-class resources is the mechanism rule 5's own accumulation
paragraph names as what eventually starves the reference source. The trait-aware source's high-class share is lower on
three of five seeds and higher on two, so the 1,000-tick picture is mixed. The 10,000-tick picture is not mixed: the
new source outlives the old one on four of five seeds. The direction is what rule 19's arithmetic predicts — a
Mokiterion that accepts the resource underfoot does not spend ticks walking past resources — and the consumption
column moves the same way, but the two are not independent, since a longer run has more ticks in which to eat, so
consumption is corroboration and not a second finding.

**The decision, 2026-08-19, by the repository owner acting as product owner: neither an improvement nor a regression.**
The owner recorded that the high-class share moves both ways at 1,000 ticks and that the longevity gain at 10,000 ticks
is downstream of eating more rather than of accumulating less, so the result is recorded as neither. `REQ-MOK-034`
states no obligation on it and nothing in the approved chain turns on the answer; `VER-MOK-010` required it recorded
rather than inferred, and this is that record. `closing-review.md` records the act.

## 4. `fear`'s constants, by the technical owner — **recorded 2026-08-19**, carrying an adverse figure

**What was to be judged.** `VER-MOK-010`: the `+10` increment and the `-5` decrement produce a value that moves on a
timescale a reader can interpret, neither saturating in the first few ticks nor effectively constant across a run.
"This is a judgement because nothing consumes `fear`, so no outcome can falsify the constants." The owner also records
the measured share of agent-ticks on which `fear` was non-zero, because a value pinned at the lower bound would be the
inert attribute `SPEC-MOK-003` rule 4.5 refused, arriving by another route.

**The measurement** (`measurements/fear.txt`, every `survival_changed` record of ten 1,000-tick runs — 111,604
agent-ticks):

- `fear > 0` on **48%–62%** of agent-ticks, by source and seed. The lower-bound failure mode does not occur.
- `fear` is at **100** on **39%** of agent-ticks measured. This is the adverse figure.
- transition magnitudes pooled over all ten runs: `-5` on 13,940; `0` on 90,201; `+5` on 219; `+10` on 7,244.
- range, step and perception-correspondence violations: **zero** on every run, including at both saturating bounds and
  at the radius boundary — 245–427 agent-ticks per run with the nearest other Mokiterion at exactly 16 all rose, and
  238–448 at exactly 17 all fell.

**The adverse fact, stated as such.** The attribute spends two agent-ticks in five at its ceiling, and `0` is by a
wide margin the most common transition — because a `+10` rise against a `-5` decay is a two-to-one ratchet, and rule
12's own arithmetic puts the expected number of other Mokiterions inside the radius at about 0.73, making a solitary
tick the minority case. This follows from the approved constants rather than from any departure from them, and rule 12
states in the same breath that saturation is a normal outcome rather than an error. It is recorded because it is new
information: **a future consumer of `fear` would find it pinned at the ceiling for two agent-ticks in five and would
be reading a near-constant rather than a signal.** That belongs in front of whoever specifies that consumer.

The `+5` step is worth naming separately. It occurs 219 times and exists only because a decay of 5 from the ceiling
leaves 95, so the ceiling is approached from 95 as well as from 90. It is a consequence of the two constants not being
multiples of one another, and it is the one transition magnitude a reader of rule 12 would not predict from its text.

**The decision, 2026-08-19, by the repository owner acting as technical owner: keep `+10`/`-5` as approved.** The
question put was whether that pair is the one to keep given a 39% ceiling residency, or whether the constants should be
revised before a consumer is specified; the adverse figure was stated first and revision was on the table. The owner
recorded that the value moves on a timescale a reader can interpret, that the lower-bound failure mode this assessment
guards against is absent (`fear > 0` on 48%–62% of agent-ticks), and that rule 12 states saturation to be a normal
outcome rather than an error. **The 39% ceiling residency is recorded as an observation for whoever specifies a
consumer of `fear`, not as a defect**, and it is not softened: a consumer added later would be reading a near-constant
for two agent-ticks in five, and it is that specification's problem to solve rather than this one's. No constant
changes and `SPEC-MOK-003` rule 12 is not amended. `closing-review.md` records the act.

## 5. Roster legibility at four bars, by the technical owner — **recorded 2026-08-19**

**What was to be judged.** `VER-MOK-010`: filling the reserved slot narrows the bars; at the reference roster's
45-column interior, four gauges leave `(45 − 35) / 4 = 2` cells per bar where three left 6. The owner records that a
2-cell bar beside its 3-column numeric value is acceptable at that viewport, or requires the roster pane to be widened
in `SPEC-MOK-003` rule 5 instead — and "either way the decision is stated in the rule 4.5 amendment, not settled in
the implementation".

**The decision, and where it is recorded.** The repository owner, acting as technical owner, accepted the narrowing on
2026-08-19 rather than widening the roster pane, which would have taken fourteen columns from the map pane, or raising
rule 4's 47-column two-line threshold, which would have cost bars entirely to operators between 47 and 60 columns. It
is recorded in `SPEC-MOK-003`'s amendment record for that date and in the amended text of rule 4 item 5 itself, in the
terms the assessment requires. `amendment-approvals.md` confirms both.

**One measured fact that arrived after the decision and sharpens it.** Oracle 4 swept every width from
`layout::MIN_WIDTH` to 160 and found that because the roster pane's fixed width and rule 4's collapse threshold are
both 47, the drawn roster is *always* two-line, always at a 45-column interior, and therefore always at
`bar = 2`: 61 widths draw a 47-column pane, 66 draw no roster at all, and no other bar width is reachable through
`render::draw` (`observer/roster-frames.txt`). The 2-cell bar is not the narrow end of a range the operator can
escape — it is the only bar the observer draws. That makes the recorded decision more load-bearing than it appeared
when it was taken, not less. The 20-cell cap and the collapsed one-line form are unreachable through `draw` and are
carried by three named internal render tests instead, which oracle 4 runs.

**Reaffirmed on 2026-08-19 by the repository owner acting as technical owner, with the sweep noted.** The sweep was put
to the owner as a fact that arrived after the decision, with widening the roster pane still on the table; the owner
recorded that the decision of 2026-08-19 stands and that oracle 4's `bar = 2`-only finding — 61 widths drawing a
47-column pane, 66 drawing no roster, no other bar width reachable through `render::draw` — is noted on the record as
making the accepted narrowing more load-bearing rather than as changing the answer. `SPEC-MOK-003` rule 5 is not
widened. `closing-review.md` records the act.

## 6. The projection, by the assurance owner — **recorded 2026-08-19**

**What was to be judged.** `VER-MOK-010`: the transformation oracle 1 applies deletes only the fields this change adds,
and the owner confirms it could not mask a change to a position, an identifier, an event kind, an ordering or an
attribute value.

**The projection, in full** (`baseline/projection.py`, three anchored patterns applied in this order):

```python
TRANSITION = re.compile(r',fear:\d+->\d+')      # survival_changed
TRAIT      = re.compile(r',waste_tolerance:\d+')  # agent_initialized
SCALAR     = re.compile(r',fear:\d+')             # agent_initialized, action_trace
```

Each pattern is anchored on the leading comma and on the field name, so it cannot match a position, an identifier, an
event kind, an ordering or another attribute's value. The transition form is deleted before the scalar form because
the scalar pattern is a prefix of it — an ordering dependency worth the owner's eye, since reversing it would leave
`->\d+` behind rather than silently deleting more.

**What has been established about it mechanically.** The projection is applied to *both* captures, not only the
post-change one: applied to the pre-change stream it must be a no-op, and it is
(`baseline/recapture-check.txt`), which is the one way this oracle could be subverted — a projection that changed a
pre-change byte would be hiding a difference rather than removing an addition. `negative-control/oracle-2.txt` records
that each designed perturbation failed in the way it was designed to fail.

**The decision, 2026-08-19, by the repository owner acting as assurance owner: confirmed — the three patterns and their
ordering delete only the additions.** The judgement was put as reading the three regular expressions above, with the
ordering dependency named: each is anchored on the leading comma and the field name, so none can reach a position, an
identifier, an event kind, an ordering or another attribute's value; the transition form is deleted before the scalar
form because the scalar pattern is a prefix of it; and the no-op on the pre-change stream closes the one route by which
a projection could hide a difference rather than remove an addition. The owner recorded that it could not mask a change
to any of the five. Reading three regular expressions was the whole of it, which is why the projection was kept to
three regular expressions. `closing-review.md` records the act.

## 7. The absence of a `fear` consumer, by the technical owner — **signed 2026-08-19**

**What was to be judged.** `VER-MOK-010`: no engine rule reads the attribute, and the owner confirms that the value's
only purpose in this scope is to be reported.

**Where it is already recorded.** `SPEC-MOK-001`'s *Scope*, as amended and approved on 2026-08-19, states it as an
approved provision: "It defines one behavioral trait and the `fear` attribute, but no rule reads `fear`: threat
response, retreat, surrender and encounter memory remain undefined." Rule 3 states that `fear` is deliberately absent
from the observation a decision source reads, and rule 12 is its only writer. So the owner has approved the sentence
that carries this assessment's content, on this date, in the specification.

**And what confirms it mechanically.** The interface census finds exactly one writer of `fear` and no reader
(`interface-and-purity.txt`), and `SPEC-MOK-003` rule 10 item 7 as amended states why the attribute is still absent
from the inspector — that rule's presented-value list was not amended by this work order — rather than leaving the
absence to read as an oversight.

**The decision, 2026-08-19, by the repository owner acting as technical owner: signed as an assessment in its own
right.** `VER-MOK-010` asks for this as one of seven explicit assessments, and approving a specification sentence is not
the same act as signing an assessment — so the implementation agent recorded the distinction rather than deciding that
the approval already covered it, and put the signature as its own question. The owner signed it: no engine rule reads
`fear`, and the value's only purpose in this scope is to be reported. The *Scope* sentence and the census stand
alongside the signature rather than in place of it. `closing-review.md` records the act.

---

## What this means for the verification record

`VER-MOK-010` is explicit: "An unrecorded assessment is an outstanding assessment, and this contract is not satisfied
while any remains outstanding." **Seven of seven are now recorded, so that clause is satisfied** — five were taken in
the closing review of 2026-08-19, one was reaffirmed there against a fact that arrived after it, and one was signed
there. Each was put as its own question and answered on its own, so no judgement here rests on an approval given for
another; `closing-review.md` records the twelve acts of that review, of which these seven are the first.

Two of the seven carry facts that point adversely, and they are named in the table at the top of this file rather than
at the bottom of it: the divergence count of assessment 2 and the 39% ceiling residency of assessment 4. Neither was
softened for the asking and neither was resolved by substituting a more favourable measure. What each decision left
standing is recorded with it — the zero same-tick coincidences of assessment 2 and the ceiling residency of assessment
4 remain on the record as observations, the first in `README.md` under *What none of this establishes*.

**What these seven do not settle.** They satisfy `VER-MOK-010`'s manual-assessment contract and nothing more. Nothing
in the code or the automated oracles was ever waiting on them — all five automated oracles pass — and recording them
verifies nothing on its own: `VREC-MOK-010` is re-captured against the commit that carries them and remains a `ready`
candidate until the accountable assurance owner reviews it and says so.
