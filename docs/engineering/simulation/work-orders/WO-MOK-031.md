+++
id = "WO-MOK-031"
type = "work_order"
title = "Reconcile the two approved artifacts stage 5b's measurements falsified: case L17's floating-point clause and WO-MOK-027's transcript-size figure"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-29"
updated = "2026-08-29"

[assurance]
commit_bound_verification = "not_required"
rationale = "This work order changes no executable behavior, runs nothing and measures nothing. It states which reading one verification case means, and it replaces one estimate in an approved work order with a figure already measured and already recorded in an approved specification. The claim a later reader needs is that the amended text says what it says, which the diff establishes by inspection, and every figure it introduces is quoted from `SPEC-MOK-007` rules 11.7.2 and 11.7.3 rather than derived here. The case that is clarified was already discharged against a candidate under `VREC-MOK-025`, and that record is not re-opened: its `L17` disposition of PASS IN PART, ESCALATED stands as written, because this work order states what the contract now requires and not that a past result changed. `WO-MOK-027` implements against the corrected figure and its assurance is `required`."
decided_by = "engineering owner"

[execution_scope]
paths = [
  "docs/engineering/simulation/evidence/WO-MOK-031/",
  "docs/engineering/simulation/verification/VER-MOK-018.md",
  "docs/engineering/simulation/work-orders/WO-MOK-027.md",
  "docs/engineering/simulation/work-orders/WO-MOK-031.md",
]

[relations]
implements = ["REQ-MOK-069"]
specifications = ["SPEC-MOK-007"]
verification = ["VER-MOK-018"]
architecture = ["ARCH-MOK-001", "ARCH-MOK-002", "ADR-MOK-007"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-29T20:41:25Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "approved"
to = "in_progress"
decided_at = "2026-08-29T20:41:41Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "in_progress"
to = "implemented"
decided_at = "2026-08-29T20:50:35Z"
decided_by = "engineering owner"
+++

# Work Order: reconcile what stage 5b measured

## Lifecycle

Governance work, stopping at `implemented`. Assurance is `not_required`: this work order amends two documents
and runs nothing. Two transitions under 0.8.0, and the handoff checkpoint takes a snapshot binding whatever the
assurance classification, which this scope admits an evidence directory for.

**On `implements`.** As with `WO-MOK-028`, `WO-MOK-029` and `WO-MOK-030`, this work order implements the named
requirement by **specifying** rather than in code — here by stating which reading its verification case means.
`WO-MOK-026` implemented `REQ-MOK-069` and `VREC-MOK-025` verified it. The relation is stated this way because an
empty `implements` is refused, and stated with this paragraph beside it so a later reader counting implementations
of `REQ-MOK-069` can tell what each work order did.

**Amendment 2 implements nothing and the relation deliberately does not claim otherwise.** It corrects a figure in
another work order's own *Constraints*, which is not an implementation of any requirement. Naming
`REQ-MOK-075` or `REQ-MOK-076` here to cover it would be fabricated traceability, so neither is named.

## Why this is one work order and not two

Both amendments have the same cause, the same authorising act and the same shape: an approved artifact whose text
was correct when written and which stage 5b's live run measured to be false. The owner decided on 2026-08-29, with
each option's cost measured, that they be reconciled in one work order rather than in two — the alternative being
two branches, two pull requests and two verification-free chains to correct four sentences between them.

Neither could be repaired where it was found. `WO-MOK-026` had reached `implemented` and `VREC-MOK-025` `verified`
before both were escalated, and `WO-MOK-026`'s stop-and-escalate condition 6 forbids amending an approved artifact
on an implementation agent's judgement in any case. They are recorded in that work order's completion report as
escalations and in `VREC-MOK-025`'s decision section as two of the four findings put to the owner.

**The third escalation of that set is not here.** `REQ-MOK-071`'s ceiling bounding the engine's belief about spend
rather than spend needs a fifth price on `--prices`, which moves a parser, a specification's census and both hosts'
usage text. That is executable behavior and a separate work order.

## In scope

**Two amendments, across two artifacts.**

1. **`VER-MOK-018` case `L17` states which reading its floating-point clause means.** The case requires that "no
   floating-point value ... appears in a transcript". `WO-MOK-025`'s retained instrument reads that as any
   fraction-shaped substring anywhere in the file, and on that reading it returns FAIL on **both** live
   transcripts — **503** hits on the accepted run and **567** on attempt 1 — while the number of **genuine JSON
   floats in either file is 0**. Every hit is the same single distinct string, `5.6`, in the same single field, in
   the same context: the model identifier `gpt-5.6-luna`, once per exchange. The same instrument returns **0 hits
   over the 233 records** of the synthetic fixture, which is why the wording survived until a provider was
   actually called.

   **The unqualified reading is not merely wide, it is unsatisfiable jointly with two approved rules.**
   `SPEC-MOK-007` rule 10.4a, amended 2026-08-29 under `WO-MOK-030`, makes the connector's **response** carry the
   model identifier, and rule 15.2 obliges the run record to state it — a decision taken so that a published
   figure names the model that **answered** rather than the model that was asked for. The provider's identifier
   contains a dot-decimal version. So a conforming live transcript **must** carry a fraction-shaped substring, and
   a transcript that satisfied the unqualified clause could not name the model that produced it. This is the same
   shape as the closed-alphabet clause this case lost on 2026-08-24, where a transcript satisfying the withdrawn
   clause could not have carried a request.

   **The clause's substance is untouched and the amendment is a statement of reading rather than a relaxation.**
   What the clause exists for is the rule that no number crosses a boundary as a float — case `P6`, and the reason
   rule 14.4's cache ratio crosses as basis points. That property holds in both transcripts exactly: 0 genuine
   floats. The precedent for stating a reading rather than moving a bound is case `L5` in this contract's
   2026-08-24 row, which "now says which reading it means" while its substance did not change.

2. **`WO-MOK-027`'s transcript-size figure becomes the measured one.** Its *Constraints* state "an **estimated**
   4.7 MB per transcript and 23 MB for five seeds is what the repository takes on, and that figure is confirmed
   against the tree rather than assumed". Measured, the accepted 50-tick transcript is **700,192 bytes**;
   `SPEC-MOK-007` rule 11.7.2 extrapolates that to **12,722,347 bytes** at 1,000 ticks and rule 11.7.3 confirms it
   a second time at **13,901,867 bytes** at a different exchange rate. Five 1,000-tick seeds are therefore roughly
   **64 to 70 MB**, not 23 — a factor of about **2.8**.

   **The sentence's second half is what makes this more than a stale number.** It claims the figure is "confirmed
   against the tree", and the tree records the opposite: `SPEC-MOK-007` rule 11.7.1 **withdrew** the 4.7 MB
   estimate on 2026-08-24, naming it as superseded, five days before this claim was read. The amendment replaces
   both halves — the figure, and the claim of confirmation — and cites the two sub-rules the new figure is quoted
   from.

## Out of scope

- **Every implementation.** No Rust file is touched, and no file under either package's `src/` or `tests/` is in
  the execution scope.
- **`WO-MOK-025`'s `L17` instrument.** Narrowing it to genuine JSON floats was put to the owner on 2026-08-29 with
  its cost and **declined**: it is a file inside `WO-MOK-025`'s evidence packet, which `VREC-MOK-024` binds and
  `SPEC-MOK-004` rule 12 makes a strictness change. Editing an instrument's retained output or an instrument a
  verified record binds is the one thing that would make it worthless. The consequence is stated rather than left
  to be found: **the instrument will keep returning FAIL on every live transcript**, and after this amendment that
  FAIL is a known disagreement between a check and the case it implements rather than an open question.
- **`WO-MOK-027`'s cost figures.** Its `$1.04` per seed and `$5.20` for five seeds are **not** amended, although
  stage 5b measured the per-run cost at about four times the estimate. That work order already requires the
  estimate to be "re-derived from `WO-MOK-026`'s **measured** per-exchange cost before the authorization is
  sought", so the figure is bound to be replaced by the act that spends the money, and amending it here would put
  a second number in front of that step without removing the obligation to derive it. Only the size claim moves,
  because only the size claim asserts a confirmation it does not have.
- **`REQ-MOK-070`.** It stays **outstanding** and its text does not move. Case `L15b` stays FAILED. The owner
  declined on 2026-08-29 to amend either the requirement or the layout, and nothing here revisits that.
- **The fifth price**, `cache_write_tokens`, and every consequence of it.
- **Every other open finding carried out of `WO-MOK-026`**, including `ARCH-MOK-002` component 3's description of
  the command-line host, `SPEC-MOK-002` rule 5's census omissions, `SPEC-MOK-006`'s two rules numbered 8.9,
  `SPEC-MOK-004` rule 1's drift, and `L20`'s coverage gaps. Each is recorded where it was found.

## Authorized decision envelope

The engineering owner may word both amendment records, and may choose whether amendment 1 is written as a
qualification of the existing clause or as a sub-clause beneath it, provided the enumerated matrix row and the
prose agree afterwards.

The engineering owner may **not**, under this work order: change what either amendment decides, as the owner
settled it on 2026-08-29; amend any case other than `L17`; move any pass condition of `L17`'s three surviving
clauses; amend any rule of any specification; narrow the `L17` instrument; touch `WO-MOK-027`'s cost figures, its
horizon, its seed set or its `[execution_scope]`; or re-open any verification record.

## Constraints

- **Every neighbouring case and clause keeps its text**, and no existing case identifier moves.
- **`VER-MOK-018`'s enumerated matrix row for `L17` and its prose must agree after the amendment.** They already
  divide `L20` and `L32` differently, which is a finding recorded under `WO-MOK-026`; this work order must not add
  a third such disagreement.
- **Each artifact takes its own amendment record**, in the form that artifact already uses. `WO-MOK-027`'s
  existing record has one row, "2026-08-28, `[execution_scope]` added, under `WO-HUP-002`, by the engineering
  owner", and that row is the form to follow — a later work order amending an approved work order in place is
  established practice here rather than a novelty this work order introduces.
- **Every figure is quoted, not re-derived.** The byte counts come from `SPEC-MOK-007` rules 11.7.2 and 11.7.3 and
  the hit counts from `WO-MOK-026`'s `live-transcript-reading.md`. Where a figure is restated it is cited to the
  rule or the evidence file it comes from.
- **No estimate becomes a measurement by wording.** The amended sentence in `WO-MOK-027` states a measured figure
  for a 1,000-tick transcript and an **extrapolated** total for five seeds, and says which is which.
- **Neither artifact's frontmatter `status` moves**, and `VREC-MOK-025` and `VREC-MOK-024` are not edited.

## Expected change surface

- `docs/engineering/simulation/verification/VER-MOK-018.md` — case `L17`'s clause, its enumerated matrix row, and
  an amendment record.
- `docs/engineering/simulation/work-orders/WO-MOK-027.md` — the transcript-size constraint, and an amendment
  record row.
- `docs/engineering/simulation/work-orders/WO-MOK-031.md` — this work order's lifecycle events.
- `docs/engineering/simulation/evidence/WO-MOK-031/handoff-check.md` — the handoff snapshot binding.

## Required verification

None contracted beyond `SPEC-MOK-005`'s standing gates. `VER-MOK-018` is the applicable contract and **no case of
it is discharged here**: its cases are about a running system and this work order runs nothing. `L17` is the case
this work order amends, which is not the same as discharging it — the next candidate to discharge `L17` is
`WO-MOK-027`'s.

`cargo` is not re-run and the completion report says so, no file under either package being in scope.

## Evidence to record

The handoff checkpoint's snapshot binding, and nothing else. No measurement is taken and no figure originates
here.

## Stop and escalate conditions

Stop and return to the engineering owner when:

- amendment 1 cannot be written without also amending `SPEC-MOK-007` rule 11.4, which adopts the same
  "no floating-point value" constraint for the transcript and is **not** in this work order's scope. Rule 11.4 is
  believed correct as written, the word *value* already excluding a substring inside an opaque string; if writing
  the case shows that belief to be wrong, the specification and the contract disagree and that is the owner's to
  resolve;
- amendment 1 would change the disposition of any case already recorded against a candidate, or would require
  `VREC-MOK-025` or `VREC-MOK-024` to be re-opened;
- amendment 2 cannot be written without moving `WO-MOK-027`'s cost figures, its horizon or its seed set;
- a **third** artifact turns out to carry the same falsified figure, in which case the count is rising by
  discovery and the reconciliation is incomplete rather than the third one being absorbed; or
- either amendment would leave `VER-MOK-018`'s matrix and prose disagreeing about `L17`.

## Completion report format

1. **The two amendments**, each with what its text said before and says now, quoted.
2. **The owner's decisions of 2026-08-29**, labelled as decisions: to amend case `L17` rather than the instrument,
   to correct `WO-MOK-027`'s size figure, and to reconcile both in one work order.
3. **The disclosure that `WO-MOK-025`'s `L17` instrument still returns FAIL** on every live transcript after this
   amendment, and that this is now a recorded disagreement between a check and its case rather than an open
   finding.
4. **The statement that no figure originates in this work order**, with each restated figure's source rule or
   evidence file.
5. **Confirmation that `VREC-MOK-025`'s `L17` disposition stands as written** and that no verification record is
   re-opened.
6. `validate`, and confirmation that no Rust file is in the change and that `cargo` was not re-run.
