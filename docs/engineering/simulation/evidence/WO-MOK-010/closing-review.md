# The closing review of 2026-08-19

This file records twelve decisions the repository owner took on 2026-08-19, in one interactive review, to close
`WO-MOK-010`'s outstanding governance. It is the record of the acts. What each was decided *on* is in
`manual-assessment.md` for the assessments and in `amendment-approvals.md` §3 for the ratifications, and nothing
below restates a figure those files do not already carry.

**Who decided what.** The repository owner holds all three accountable roles of `ENGINEERING_HARNESS.md` and acted in
each of them here, naming the role for every decision. The implementation agent put each question, with the measured
facts already assembled, and transcribed the answers. It decided none of them and approved none of them, which is the
same division `escalation.md` records for the trait-range decision and `SPEC-MOK-003`'s amendment record for the
banding decision.

**Each question was put and answered on its own.** No decision below was inferred from another, and no approval was
taken as covering a second act: the four ratifications were put as four questions rather than one, because a single
answer covering four rows would be an approval by implication of three of them.

## The seven manual assessments of `VER-MOK-010`

`VER-MOK-010`: "Each of the following is an explicit judgement recorded by the accountable role. An unrecorded
assessment is an outstanding assessment, and this contract is not satisfied while any remains outstanding." All seven
are now recorded. Five were outstanding before this review, one was recorded on 2026-08-19 and is reaffirmed here
against a fact that arrived after it, and one was recorded in substance and is now signed.

| # | Judgement | Role | Decision, 2026-08-19 |
|---|---|---|---|
| 1 | Scarcity at the default density | product owner | **Satisfied.** The adverse condition the assessment guards against did not arise: twelve survive on one declared seed of five, not on all five. No reservation recorded. |
| 2 | Individuality is meaningful | product owner | **Satisfied on the measure `VER-MOK-010` names** — the 3-to-10 divergent situations, which clear the one-per-thousand-ticks figure the contract calls a failure. The 54-to-97 waste-accepting eats stand as corroboration and are **not** substituted for the named measure, so no provision of `VER-MOK-010` changes. |
| 3 | The accumulation result | product owner | **Neither an improvement nor a regression.** The high-class share moves both ways at 1,000 ticks, and the longevity gain at 10,000 is downstream of eating more rather than of accumulating less. `REQ-MOK-034` states no obligation on it. |
| 4 | `fear`'s constants | technical owner | **Keep `+10`/`-5` as approved.** The value moves on an interpretable timescale, the lower-bound failure mode is absent, and rule 12 states that saturation is a normal outcome rather than an error. The 39% ceiling residency is recorded as an observation for whoever specifies a consumer, not as a defect. |
| 5 | Roster legibility at four bars | technical owner | **The decision of 2026-08-19 stands, and the sweep is noted.** Oracle 4's later finding — that `bar = 2` is the only bar width reachable through `render::draw`, 61 widths drawing a 47-column pane and 66 drawing no roster — makes the accepted narrowing more load-bearing than it appeared when it was taken, and does not change the answer. |
| 6 | The projection | assurance owner | **Confirmed: the three patterns and their ordering delete only the additions.** Each is anchored on the leading comma and the field name, the transition form is deleted before the scalar form because the scalar pattern is a prefix of it, and the no-op on the pre-change stream closes the one route by which the projection could hide a difference. |
| 7 | The absence of a `fear` consumer | technical owner | **Signed as an assessment in its own right**, alongside the `SPEC-MOK-001` *Scope* sentence that carries the same content and the census that finds one writer and no reader. |

Assessment 2 was the closest of the seven to the threshold `VER-MOK-010` itself names as failure, and assessment 4
carried an adverse figure. Both were put in those terms, with the adverse reading stated first, and neither was
softened for the asking. The record of each, with the measurement it was taken on, is the corresponding section of
`manual-assessment.md`.

## The four ratifications

Each is a correction or an addition the implementation agent wrote into an approved specification and could not
approve. Each was **OUTSTANDING** in that specification's own amendment record, where a reader met it, until the act
below. The rows are named by the specification and the provision, and each specification's amendment record now
carries the ratification in the row's Approval column, pointing here.

| Artifact | What was ratified | Decision |
|---|---|---|
| `SPEC-MOK-001` | The *Help output* correction: the default clause this work order's own first amendment added is withdrawn, the three-source description stays. | **Ratified.** The contradiction is resolved in favour of the provision approved 2026-08-17 and the inherited test that already enforces it. |
| `SPEC-MOK-003` | Three provisions outside rule 4: `AgentSnapshot` gains `fear`; rule 10 item 7 loses `fear` and traits from what the engine does not compute; rule 11's `decision_source_selected` row gains `REQ-MOK-033`. | **Ratified, all three.** Each is a consequence of provisions already approved, and leaving any one out would make the specification contradict `SPEC-MOK-002` rule 5, state something false, or leave an exhaustive mapping incomplete. |
| `SPEC-MOK-003` | Rule 4 clause 7's wording in two provisions: the band applies to health, satiety and energy and not to `fear`, and `fear` renders with no colour at all. | **Ratified.** The substance was the owner's decision of 2026-08-19 already; what was outstanding was the agent's wording of it, which records that decision and nothing more. |
| `SPEC-MOK-004` | The recorded test-count figures of rules 9, 10 and 11, corrected against the merged tree for this work order and for `master`'s `WO-MOK-007`: public tier 85, `render.rs` 17 internal tests over 47 private items, workspace 200. | **Ratified in full**, on the reading that neither half of the correction is statable without the other, because only the merged tree runs both sets of tests. Rule 11's own instruction is what obliges the correction. |

**What the ratifications do not reach.** They cover these four rows and nothing else. `SPEC-MOK-004`'s row of the same
date for `WO-MOK-005`'s interface figures is not among them and remains **OUTSTANDING**, as do the five other rows of
the earlier layer; the decision below is what governs those. The two rows that change no provision — the rule 4/rule 5
reconciliation and rule 11's census pointer — needed no ratification and were given none.

## The `VREC-MOK-005` layer: the override stands, with an obligation

Oracle 5's second condition is that the amendments already outstanding under `VREC-MOK-005` be resolved before this
change is verified: six rows across `SPEC-MOK-002`, `SPEC-MOK-003` and `ARCH-MOK-001`, eleven provisions across four
artifacts, together with that record's seven unrecorded assessments. The repository owner overrode that gate on
2026-08-19, before this work began, and `master` has since transitioned `VREC-MOK-005` to `verified` — a transition its
own text says accepted the automated evidence with all seven assessments still outstanding, so the status moved and the
substance did not.

**The decision of this review: the override stands, and the debt is now named rather than carried silently.** The
eleven provisions and the seven assessments are to be resolved by a work order of their own, and that work order is to
complete before the next release record. `WO-MOK-010` does not resolve them, does not approve them, performs none of
those assessments and does not transition `WO-MOK-005`. The mitigation is unchanged and is checked rather than
asserted: every amendment row dated before 2026-08-19 is byte-identical to `60fda9f`, and every row `master` carried at
`7a2b502` survived the merge byte for byte (`amendment-approvals.md` §4).

## What these twelve decisions do not do

They complete the governance state of `WO-MOK-010`'s own artifacts. They are not a verification, and they transition
nothing on their own: `VREC-MOK-010` is re-captured against the commit that carries them and stays a `ready` candidate
until the accountable assurance owner reviews it and says so, `WO-MOK-010` stays `in_progress` until that owner moves
it, and nothing here merges, tags, releases or publishes anything. Release remains a separate record and a separate
decision.
