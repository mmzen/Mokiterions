# The closing review of WO-MOK-013

This file records the decisions this work order rests on, with the role that took each, and the
choices the implementation agent made inside the envelope those decisions left it. It continues
**this work order's own numbering from decision 1** and does not continue `WO-MOK-012`'s closing
review, whose seventeen decisions are numbered there. **The two sequences collide in their numbers
and not in their subjects**, which `WO-MOK-013`'s *Decision record* states and this file repeats
because a reader arriving at "decision 13" needs to know which document it belongs to.

**Who decided what.** The repository owner holds all four accountable roles of
`ENGINEERING_HARNESS.md` — product owner, technical owner, assurance owner and release owner — and
acted in the named role for each decision below. The implementation agent put each question with the
measured facts already assembled, transcribed the answer, and decided none of the substance. Nothing
here was approved by implication of anything else: twenty-two acts, each put and answered on its own.

**What is unusual about this work order is when the decisions were taken.** The first eighteen were
taken **before** implementation began, and `WO-MOK-013` says why: "No amendment row in this chain is to
be left `OUTSTANDING`, and none is." The eleven provisions ratified under `WO-MOK-012` had been drafted
by implementation agents and left unratified for one or two days, and `WO-MOK-005` records of five of
them that it "is not verifiable until they are given". Obtaining the ratifications ahead of approval
is how that was prevented, and this review is therefore largely a record of decisions already documented
in the work order rather than a fresh round of questions.

**Decisions 19 to 21 were taken after implementation**, on the live pass of 2026-08-20 at the reference
viewport. They are three of the four this review records that the work order's *Decision record* does not,
because they could not be taken until there was something to look at. **Decision 22 is the lifecycle act
that follows from them**, and it is the only one of the twenty-two that changes an artifact's status.

## The twenty-two decisions, and the twenty-third

| # | Subject | Role | Outcome |
|---:|---|---|---|
| 1 | The log is held at six rows rather than amending `REQ-MOK-020` | product owner | **Option B** — six rows, twelve entries kept |
| 2 | Option B on the corrected cost figure | product owner | **Stands** |
| 3 | The `WO-MOK-012` identifier collision | engineering owner | **Left to the merge** |
| 4 | `VER-MOK-005`'s unsatisfiable fill row | assurance owner | **Withdrawn** |
| 5 | `REQ-MOK-047`, the gauge resolves | product owner | **Approved** |
| 6 | `REQ-MOK-048`, the controls are findable | product owner | **Approved** |
| 7 | `REQ-MOK-049`, the notice is actionable | product owner | **Approved** |
| 8 | `SPEC-MOK-003` declares coverage of the three | technical owner | **Declared** |
| 9 | `VER-MOK-013` | assurance owner | **Approved** |
| 10 | The two stale `VER-MOK-005` rows | assurance owner | **Amended to the true form** |
| 11 | How the six `SPEC-MOK-003` amendments are put | technical owner | **One at a time** |
| 12 | Amendment 1 — rule 4's bar row, two gauges per line, bar 2 → 13 | technical owner | **Ratified as enumerated** |
| 13 | Amendment 2 — rule 4 item 1's arithmetic and the 36-row provision | technical owner | **Ratified as enumerated** |
| 14 | Amendment 3 — rule 4 clauses 5 and 7 on the three-line entry | technical owner | **Ratified, gauge order fixed h/s then e/f** |
| 15 | Amendment 4 — the header's permanent affordance | technical owner | **Ratified as a distinct affordance** |
| 16 | Amendment 5 — rule 5's announcement obligation and the ladder | technical owner | **Ratified, threshold survives last** |
| 17 | Amendment 6 — rule 5's log row count, six unconditionally | technical owner | **Ratified as enumerated** |
| 18 | This work order as a bounded scope | engineering owner | **Approved** |
| 19 | Manual assessment 1 — the gauge is legible at `160 × 48` | product owner | **SATISFIED** — "the bar carried it" |
| 20 | The route for manual assessment 2, which has no admissible assessor | assurance owner | **Find an admissible assessor** — the other two options declined |
| 21 | Decision 1 re-opened on the corrected figure, after the live pass | product owner | **Stands** |
| 22 | `WO-MOK-013`'s status, and whether `VREC-MOK-013` exists | engineering owner | **`implemented`, and the record prepared as a `ready` candidate** |
| 23 | `VREC-MOK-013`'s `ready` → `verified` transition, and the basis for `REQ-MOK-048` | assurance owner | **`verified`, with `REQ-MOK-048` verified on its four automated cases accepted in place of the primary evidence** — the contract not amended, assessment 2 left outstanding |

Decisions 1 to 18 are each recorded in full in `WO-MOK-013`'s *Decision record*, with the instruction
verbatim where one was given verbatim, and nothing above restates a measurement that section already
carries. Decision 1 in particular retains the two options as they were put rather than only its outcome,
because the arithmetic is what makes it reviewable. Decisions 19 to 21 are recorded in
`manual-assessment.md` and in item 7 of `completion-summary.md`; the work order predates them. Decision 22
is recorded in `WO-MOK-013`'s own *Transition to `implemented`* subsection and in `VREC-MOK-013`.

**Decision 22 moved one status and created one record, and it is one act rather than two.** The
instruction, verbatim: *"you can transition WO-MOK-013 as implemented, and create VREC-MOK-013"*. The
status change is the accountable half — `implemented` asserts the completed change **and** the retained
evidence, which is why the implementation agent held the work order at `approved` through implementation
and through decisions 19 to 21 rather than moving it. The record's creation is **not** an accountable
decision: `DECISION_RIGHTS.md` classes preparing a `ready` verification record as automation's work and
reserves the transition to `verified` to the assurance owner. So `VREC-MOK-013` exists as a **`ready`
candidate**, the harness reports `decision_required -> review-assurance-decision (assurance-owner)` against
it, and **that decision is not among the twenty-two above because it has not been taken.**

> **Later fact, 2026-08-20 — it has now been taken, and it is decision 23.** The instruction, verbatim: *"i
> approve VREC-MOK-013, making REQ-MOK-048 verified."* Two acts in one sentence, and the second is not a
> consequence of the first: the record as captured said `REQ-MOK-048` was **not** verified and that *"no
> status on this record can make it so"*, so the basis was put back to the owner with three readings and
> their costs measured — accept the automated cases, amend `VER-MOK-013`, or transition the record only and
> leave the requirement disclosed as unverified. **The owner took the first.** `REQ-MOK-048` is verified on
> the four automated cases accepted in place of the primary evidence the contract designates; `VER-MOK-013`
> is **not amended** and still calls that case *"a necessary condition and not the property"*; manual
> assessment 2 is still **OUTSTANDING with no author**; and `VER-MOK-013` is therefore **not satisfied on
> that one count**. **`WO-MOK-013` did not move and stays `implemented`.** The paragraph above is left as
> written because it correctly describes the state decision 22 produced; `assurance-decision.md` records
> decision 23 in full, including the harness state either side of it — `decision_required` **1 → 0**,
> everything else unchanged. **Decision 23 is the second of the twenty-three to change an artifact's
> status**, and the sentence above about decision 22 being the only one is true of the twenty-two it was
> written about.

**Decision 2 exists because the figure decision 1 was taken on was wrong.** Option B was put as
showing six recent events rather than ten; both figures counted pane rows and not event lines, and the
true change is **eight to four**, which `log-height.md` measures. The owner was shown the correction
and stood by option B. `VER-MOK-013` records that the contract does not depend on the outcome and that
the owner may re-open decision 1 on the corrected figure.

**Decision 21 closes what decision 2 left open.** `WO-MOK-013` states that the product owner may
re-open decision 1 on the corrected eight-to-four figure; decision 2 stood by option B on the arithmetic
alone, before there was an implementation to look at. On the live pass the question was put again, this
time with the result on screen. Option B stands. The invitation in the work order is now spent rather
than still standing.

**Decision 20 is a route, not an outcome.** Assessment 2 needs a person who has not read
`SPEC-MOK-003` rule 7, and the owner has ratified six amendments to it. Three options were put: find
such a person, amend `VER-MOK-013` to an assessment the available assessors can take, or leave the row
outstanding by decision on the pattern of `evidence/WO-MOK-012/manual-assessment.md`'s assessment 7. The
third would have closed the row and required nothing further; the owner took the first, **the only one
that verifies `REQ-MOK-048`**, and the row therefore stays outstanding.

> **Later fact, 2026-08-20.** Decision 23 took a **fourth** course, which was not among the three put here:
> verify `REQ-MOK-048` on its automated cases and leave the assessment open. So "the only one that verifies
> `REQ-MOK-048`" was the reasoning on which decision 20 was taken, and decision 23 departs from it — the
> reasoning is left as recorded, because a decision's recorded ground is not rewritten when a later decision
> declines it. **Decision 20's route still stands**: the assessment is still worth taking, still unspent, and
> still needs a person who has not read rule 7. What it would now settle is whether the verified requirement
> holds in front of a reader, rather than whether it may be recorded as verified at all.

It is attributed to the
**assurance owner** because the two declined options are assurance acts on an approved contract — one
amends it, the other waives an assessment it requires — which is the role
`evidence/WO-MOK-012/manual-assessment.md` records for its own route decision. Decision 19, by contrast,
is a **product-owner** act, because `VER-MOK-013` makes an adverse manual outcome "an artifact decision
requiring product review".

## What the agent decided, inside the envelope

The *Authorized decision envelope* grants the implementation the hint's and the announcement's exact
wording within what the amended rule 5 admits, the short forms of each rung of the ladder, the style
used for emphasis, the internal names of constants and helpers, and the placement of tests between the
tiers. Every choice below is one of those, and each is recorded so that it is reviewable as a choice
rather than discovered as a fact.

| Choice | What was chosen | Where it is measured |
|---|---|---|
| The hint's long form | `? keys`, beside the run state | `frames.txt`, every viewport above the floor |
| The hint's short form | `?` alone, at the floor | `frames.txt` at `34 × 22` |
| The announcement's form | `overlays: <pane> <key> at <axis> <value>`, e.g. `inspector i at width 140` | `frames.txt` at `120 × 48` |
| The ladder's short forms | `r W100`, `L H38`, `i W140` — key, axis initial, threshold | `frames.txt` at `34 × 22` |
| Emphasis | a modifier on the announcement's cells, absent from the optional segments | `tests/render.rs::the_announcement_is_emphasised_and_the_optional_segments_are_not` |
| Test placement | 12 public tier, 2 internal tier | `test-census.md`, and `SPEC-MOK-004` rules 9 and 10 |

Two placements are worth naming because they are the agent's under a rule rather than free choices:

- **The two internal-tier tests.** `a_ten_point_step_moves_the_fill_at_the_reference_viewport` and
  `every_bar_width_the_roster_can_produce_resolves_a_ten_point_step` reach the `#[cfg(test)]` snapshot
  hook through the test module's own `hold_every_attribute_at` helper rather than naming it in the
  test body. `SPEC-MOK-004` rule 8 treats that as reaching it — the placement rule reads the access a
  test requires, not the line it is written on — so they are internal tier and the hook figure moves
  from 5 of 18 to 7 of 20.
- **The rename rather than the removal.** `the_log_is_ten_rows_only_where_both_thresholds_are_met`
  became `the_log_is_six_rows_wherever_it_is_present`. The work order expected the removal to be
  reported. Keeping it asserts the withdrawn growth absent instead of leaving it untested, which rule
  12 admits as a rename with its assertion strengthened. `test-census.md` records both names.

## Three things the enumeration missed, all corrected and all reported

`WO-MOK-013` enumerates the locations each amendment touches, and applying them found three more. All
three are corrected in the tree and named in the amendment records rather than presented as approved
text, on this repository's own precedent that an example or scenario illustrating a provision is swept
with it.

| Where | What was wrong | Status |
|---|---|---|
| `SPEC-MOK-003`, the *shrinking terminal* example | Said the log "shrinks to 6 rows, since the taller log needs both thresholds" and the canvas "becomes 71 × 36" — both false once `160 × 48` already has six rows | Corrected; named in that document's amendment row of 2026-08-20 |
| `VER-MOK-005`, acceptance scenario 4 | Said the log "shrinks to six rows" on narrowing from `160 × 48` to `120 × 48` | Corrected to "keeps its six rows"; named in that contract's amendment row |
| `VER-MOK-005`, acceptance scenario 5 | Said the log "holds six rows rather than ten" at `160 × 40` | Corrected to "holds its six rows at both heights"; named in the same row |

The scenarios' figures — `71 × 36` and `67 × 28` — are unchanged before and after; only the
comparison against a ten-row reference that no longer exists was corrected.

**A fourth item is a pre-existing defect rather than a miss.** `SPEC-MOK-004` rule 10 recorded
`src/render.rs` as declaring 47 private items; at `a339902`, the commit this work order starts from, it
declared 46 — 30 private functions and 16 private constants. The figure was one high before this work
order touched it. It is reported as a pre-existing defect in the amendment row that moves it to 48,
rather than absorbed silently into the new figure, and the `WO-MOK-011` paragraph's "48 total
declarations" is named as the count that was right.

## The four stop conditions that could have fired, and did not

| Condition | Measured |
|---|---|
| 3 — any figure in `SPEC-MOK-004` rule 6 moving | **Did not fire.** `interface.txt` re-counts 94 items, 118 `pub` lines, 24 public fields, module by module, unchanged |
| 4 — any movement in the text, event or entropy stream | **Did not fire.** `non-perturbation.txt` and `baseline-comparison.txt`, 7,534 records identical in order and the engine binary byte-identical at both commits |
| 5 — a monotonicity violation at any viewport | **Did not fire.** No pane's presence threshold was touched; the sweep over `34..=200 × 22..=60` holds |
| 6 — a viewport that cannot carry the announcement and the hint at the last rung | **Did not fire.** The floor `34 × 22` carries `HELD  ?  x8  r W100  L H38  i W140` with all three panes announced and rule 8's footer intact |

Condition 2 — a further conflict between an approved artifact and one of the three requirements — had
already fired once, before approval, and had already been answered by decisions 4 and 10. It did not
fire again. Condition 7 — a test whose assertion cannot survive — did not fire: the one test whose
subject ceased to exist was strengthened rather than weakened. Condition 8, the identifier collision,
does not fire during implementation by decision 3 and fires at the merge if this branch lands second.
**It now covers a second collision the condition was not written for**: `WO-MOK-013`, `VER-MOK-013`,
`VREC-MOK-013` and `REQ-MOK-047` are each claimed by
`origin/governance/adr-mok-006-third-party-crates` as well, which `identifier-collision.md` measures.
Decision 3's rule reaches it unchanged — the second branch to `master` renumbers — but the condition's
own words name the `WO-MOK-012` collision only, so the extension is stated here rather than assumed.

> **Later fact, 2026-08-20 — condition 8 is now spent, and it did not fire.** Its trigger is *"if this branch
> is the second of the two to reach `master`"*, and this branch was the **first**, merged at `798e5d5`.
> Neither colliding branch — `origin/feature/phase-4a-definition` for the first collision,
> `origin/governance/adr-mok-006-third-party-crates` for the second — is merged, so under decision 3's rule
> the renumbering falls to them in both cases and nothing is owed from this side. The condition was not
> waived and no renumbering was skipped that should have happened.
> `identifier-collision.md`'s *What the merge determined* measures the outcome, including the 9 and 8
> conflicts the two branches now report against `master`.

## What this review did not decide

- **Whether manual assessment 2 passes.** It is outstanding with no author, in `manual-assessment.md`.
  Decision 20 settles how it will be closed and not what it returns. The frame and the question are
  prepared in `discoverability-assessment.md`; the person is not, and no agent can supply one.
- **Whether `VREC-MOK-013` is verified.** The record exists as a **`ready` candidate** bound to
  `41c20ca`, prepared under decision 22. `DECISION_RIGHTS.md` reserves the `ready` → `verified`
  transition to the accountable assurance owner, the instruction of 2026-08-20 named the record's
  creation and stated no judgement on its evidence, and the harness reports the outstanding act as
  `decision_required -> review-assurance-decision`. Decision 19 is what made the record writable —
  `REQ-MOK-047` on the assessment, `REQ-MOK-049` on the automated cases — and the record discloses
  `REQ-MOK-048` as unverified.

  > **Later fact, 2026-08-20.** This review still did not decide it — **decision 23 did**, in a separate
  > instruction and a separate commit. `VREC-MOK-013` is `verified`, and `REQ-MOK-048` with it, on the four
  > automated cases accepted in place of the primary evidence. The bullet is kept because it is true of this
  > review; what it disclaims is now recorded in `assurance-decision.md`.
- **The four-identifier collision with `origin/governance/adr-mok-006-third-party-crates`.** That
  branch holds a different `WO-MOK-013`, `VER-MOK-013`, `VREC-MOK-013` and `REQ-MOK-047`, all pushed
  and all owner-approved on their side. `identifier-collision.md` measures it and takes no decision;
  decision 3's rule sends it to the merge and stop condition 8 does not fire during implementation.
  Neither side renumbered here.
- **`VREC-MOK-005`'s staleness**, `WO-MOK-008`'s draft disposition, the eight `W-HEX-003`
  reassessments, the three `W-HEX-001`s, manual assessment 7 of `VER-MOK-005`, and `ROADMAP.md`'s
  Phase 2 claim. All out of scope, all still open, and none touched.
- **Any push, pull request, tag or release.** `WO-MOK-013` puts all four out of scope and none was
  authorized in this work.

  > **Later fact, 2026-08-20.** The owner authorized publication separately, and the branch reached `master`
  > through pull request [#32](https://github.com/mmzen/Mokiterions/pull/32) at `798e5d5`. **Those are the
  > owner's acts, taken outside this work order's scope rather than as part of its discharge**, which is why
  > the bullet stands. Still no tag and no release. The merge is not a governed act of this chain and is not
  > verified by `VREC-MOK-013`, whose binding ends at `41c20ca`.
