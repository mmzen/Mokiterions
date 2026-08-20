# The closing review of 2026-08-20

This file records fifteen decisions the repository owner took on 2026-08-20, in one interactive review, to close the
governance debt that `VREC-MOK-005` discloses and cannot itself carry. It is the record of the acts. What each was
decided *on* is in `amendment-ratifications.md` for the eleven ratifications, in `manual-assessment.md` for the seven
assessments, and in `adverse-observations.md` for the three findings; nothing below restates a measurement those files
already carry.

**Who decided what.** The repository owner holds all accountable roles of `ENGINEERING_HARNESS.md` — product owner,
technical owner, assurance owner and release owner — and acted in each of them here, with the role named for every
decision. The implementation agent put each question, with the measured facts already assembled, and transcribed the
answers. It decided none of them, approved none of them, and transitioned no artifact. This is the same division
`evidence/WO-MOK-010/closing-review.md` records for that work order's closing review, and the same one
`SPEC-MOK-003`'s amendment record states for the banding decision.

**Each question was put and answered on its own.** No decision below was inferred from another and no answer was taken
as covering a second act. Because one person holds every role, nothing here is approved by implication of anything
else: an approval given as product owner is not an approval as technical owner, and a ratification of one provision is
not a ratification of the next. The eleven ratifications were put as four separate questions, each enumerating its
provisions individually, rather than as one question — a single answer covering eleven rows would be an approval by
implication of ten of them.

**What the owner was shown before each ratification.** For every provision: the exact text amended in the tree, the
clause it changes, the approved requirement or work order it rests on, and — where the provision is a figure — the
measurement that produces it. The agent stated in each case whether the provision authorizes something already in the
tree or something new, because the two are different acts and only the first was true here.

## Part 1 — The eleven ratifications

Eleven provisions across four approved artifacts were amended in the tree with an Approval cell reading
**OUTSTANDING**, meaning the amended text had been drafted by an implementation agent and never ratified by the
accountable owner. `WO-MOK-005` states of the first five that it "is not verifiable until they are given or the scope
is changed to avoid needing them". They are now given.

### Decisions 1 to 4 — `SPEC-MOK-002`, four provisions, technical owner

Put as one question enumerating four provisions, each answerable separately.

| # | Provision | Decision |
|---|---|---|
| 1 | **Rule 1** — "no second package, no workspace" narrowed to a workspace of exactly two packages | **Ratified as written** |
| 2 | **Rule 3** — the clause freezing `src/cli.rs` and `src/simulation.rs` scoped to the `WO-MOK-003` restructuring | **Ratified as written** |
| 3 | **Rule 5** — the closed enumeration grown by the read-only observation surface | **Ratified as written** |
| 4 | **Rule 6** — the prohibition narrowed from five named value types to the capability it was written to deny | **Ratified as written** |

The owner was shown that rule 1's narrowing rests on `REQ-MOK-026`, the approved requirement for which the clause
itself reserved an exception, so the narrowing takes an exception the rule already contemplated rather than inventing
one; that rule 3's freeze was written for a specific restructuring and, left unscoped, forbids any approved
requirement from ever adding code to those two files; that rule 5's growth happened under rule 5's own growth clause;
and that rule 6's prohibition was written to deny a capability — handing out mutable authoritative state — which the
narrowing preserves exactly, the five named types having been a listing of instances rather than the prohibition's
subject.

Nothing about mutation, dependency direction, determinism or observable behavior is relaxed by any of the four, and the
engine package's dependency table stays empty.

### Decision 5 — `ARCH-MOK-001`, the public-item prohibition, technical owner

**Ratified as written.** The prohibition on public items was narrowed from "mutable **or owned** authoritative state"
to a mutable borrow of, or a reference into, that state.

This is the provision that makes the shipped observer legal, and the owner was shown it in exactly those terms: the
prohibition as originally worded also forbade the owned, reference-free snapshots `SPEC-MOK-003` specifies, which are
the capability `REQ-MOK-019` through `REQ-MOK-025` and `REQ-MOK-027` require. **Ratifying it authorizes what is
already in the tree rather than permitting something new**, and declining it would have made the shipped observer
non-conforming rather than making it safer. The narrowing grants no mutation: no public item yields a mutable borrow
of, or a reference into, world grid, agents, resources, tick counter, entropy state or event log, in any build
configuration including tests.

### Decision 6 — `SPEC-MOK-003`, *Data and interface contracts* clause 2, technical owner

**Ratified as written.** Clause 2 said `advance_tick` was the only `&mut self` method on the surface that changes
state; `Simulation::run`, the `REQ-MOK-010` whole-run entry point, is a second.

The owner was shown that this row **corrects a false statement of fact rather than relaxing an obligation**.
`Simulation::run` was already public at `origin/master` before the observer existed, so the clause was wrong on the day
it was approved; the amendment makes the specification true rather than making the implementation permissible. The
non-perturbation property is unaffected, and the observer reaches neither `run` nor anything leading to it.

### Decisions 7 to 11 — `SPEC-MOK-004`, five provisions, technical owner

Put as one question enumerating five provisions. All five are consequences of deleting the `layout::Tier` enum, which
the `SPEC-MOK-003` rule 5 amendment of 2026-08-19 authorized.

| # | Provision | Decision |
|---|---|---|
| 7 | **Rule 6** — recorded extent 13 → 10 layout items and 97 → 94 items; 122 → 118 counted the other way; public fields 25 → 24; a new **Reduction** clause; the byte-identity check scoped to `WO-MOK-006` | **Ratified as written** |
| 8 | **Rule 9** — `tests/layout.rs` 7 → 10 tests, public tier 77 → 80 | **Ratified as written** |
| 9 | **Rule 11** — observer total 109 → 112, workspace 169 → 172 | **Ratified as written** |
| 10 | **Rule 12** — second paragraph scoped so renaming `excluded_panes_are_the_ones_the_tier_omits` is not a violation | **Ratified as written** |
| 11 | **Rule 13** — the term "layout tier" dropped | **Ratified as written** |

The owner was shown the distinction that matters here: the repository owner had approved the rule 5 amendment that
authorizes the removal and directed the implementation in the same act on 2026-08-19, **but was not shown this
consequence** — the implementation agent found it afterwards by measuring the interface against rule 6, and reported it
rather than treating the rule 5 approval as covering it. This ratification answers that report.

Of the five, three are measured figures and two are not. The owner was told which: the **Reduction** clause was
ratified on the reasoning that removal should be governed the way addition already was, and the byte-identity scoping
on the reasoning that an unscoped check forbids the observer's code from ever changing again, which was never its
subject.

**The two rows dated 2026-08-19 below this one in `SPEC-MOK-004` are not part of these eleven.** They were already
ratified in `WO-MOK-010`'s closing review, and the agent corrected its own earlier count before putting the question,
so the owner was shown eleven provisions rather than a larger number that would have included work already done.

## Part 2 — The seven manual assessments

`VER-MOK-005` names seven manual assessments. Before this review all seven were **OUTSTANDING** with no author, which
`evidence/WO-MOK-005/manual-assessment.md` states plainly of itself: "Every one of the seven is OUTSTANDING. None has
been performed."

The owner performed six and authored them. The full statements, with the material each was assessed against, are in
`manual-assessment.md`. In summary:

| # | Assessment | Role | Outcome |
|---|---|---|---|
| 1 | The 200-tick instrument answers `INT-MOK-004`'s three questions | product owner | **Satisfied** |
| 2 | Reference-viewport legibility | product owner | **Adverse observation** — the survival bars are two columns wide |
| 3 | No distinction lost without colour, as restated | product owner | **Satisfied** |
| 4 | A rejection reads as an authority outcome, as restated | product owner | **Satisfied** |
| 5 | The fourth roster slot, as restated | product owner | **Satisfied** |
| 6 | Whether overview cell granularity misleads | product owner | **Not materially misleading** |
| 7 | The terminal is usable after a deliberate panic | assurance owner | **Remains outstanding by decision** |

### Decision on assessment 7 — assurance owner

**Left outstanding, deliberately, with the reason recorded.** The shipped binary has no operator-reachable panic path,
so the live-terminal inspection the contract asks for cannot be performed by running the observer. The automated
result exists and is retained in `WO-MOK-005`'s `terminal-restoration.txt` — raw mode measured off before init, on
after, and off again after a caught panic — and the contract's words "rather than only by an automated assertion" make
it insufficient by construction.

The owner elected to leave the assessment visibly outstanding rather than close it against a proxy. **`VREC-MOK-005`
must continue to disclose it.** An assessment closed against a weaker measurement than the one its contract names is
worse than an assessment left open, because the record would then claim a person had inspected something no person had
seen.

### The three restatements — assurance owner

Three assessments named subjects that had moved since `VER-MOK-005` was approved. Each restatement was put as its own
question, with the measurement that showed the mismatch, and decided on its own. All three were approved and are
recorded as an amendment row in `VER-MOK-005`.

- **Assessment 3** asked for a confirmation "with colour disabled or on a monochrome terminal". The agent reported
  that **no colour-disable mechanism exists**: `mokiterions-tui` has no `--no-color` flag and honours no `NO_COLOR`
  variable, so the assessment could not be performed on the shipped binary at all. Restated as the check that
  survives and that no test can reach — whether `UNDERLINED` and `REVERSED` render distinguishably.
- **Assessment 4** asked that a rejection read as an authority outcome. The state is unreachable in a live run.
  Restated to name the `#[cfg(test)]` hook route explicitly.
- **Assessment 5** asked whether "the reserved fourth roster bar position reads as empty space". The slot is no longer
  reserved. Restated against the slot as it now exists.

The owner was told in each case that a restatement points an existing obligation at a subject that exists, and
weakens nothing; and that the alternative — assessing against the text as written — was impossible for 3 and 4 and
meaningless for 5.

## Part 3 — Disposition of the three adverse observations

The owner reported three findings from the live pass. Assessment 2 asked for one of them; the other two were volunteered
and no assessment covers them. All three are recorded with their measurements in `adverse-observations.md`.

1. **The `?` key is undiscoverable.** The key-binding overlay is the only route to the control documentation and
   nothing on screen mentions it.
2. **The survival bars are two columns wide.** At the reference roster's 45-column interior `bar_width` evaluates to 2,
   giving three renderable states for 101 values.
3. **Excluded panes are announced, but the notice names the wrong remedy and carries no emphasis.**

On the third the agent corrected the owner's framing rather than accepting it: the announcement obligation **does**
exist in `SPEC-MOK-003` rule 5, **is** permanent, and the implementation **does** conform to it. The real defects are
narrower — `render.rs:178` draws the announcement with `Span::raw`, so it carries no emphasis, and the text names the
overlay key rather than saying the terminal is too small and what size would restore the pane. The finding was
reframed to those two points and decided on that basis.

### Decision 12 — scope, product owner

**A separate chain.** This work order records the three observations, ratifies the eleven, amends `VER-MOK-005`,
leaves assessment 7 outstanding, and **completes**. A second chain then fixes the observer.

The reasoning put to the owner: `VER-MOK-005` establishes the standing of an adverse observation in advance — it is
"an artifact decision, not a defect to patch" — and folding a rule 4 redesign into this work order means the
assessment obligation never clears before the next release record, because the work order cannot reach `implemented`
until the redesign is specified, approved, built and verified. Recording and fixing are different acts with different
approval requirements, and separating them lets the first complete.

### Decisions 13 to 15 — the design of the second chain, product owner

Decided now so the chain that carries them inherits a settled design. **None is implemented by this work order.**

**Decision 13 — the roster gauge layout.** Two gauges per line over three lines per entry. From
`2 × (w + 6) + 5 + 2 ≤ 45` the bar width is **13**, more than double the original 6 and over six times the current 2.
The roster's 32-row interior at three rows per entry shows 10 of 12 entries; the existing scrolling window already
follows the selection, and the roster title already states how many are hidden, so the two that fall off are
announced by machinery that exists. The ten-row log is kept.

The owner was shown three alternatives and chose this one. One gauge per line over five lines gives the full 20-column
bar but shows only 6 of 12 entries. Widening the roster to 61 columns takes fourteen columns from the map, dropping
the view interior from 67 to 53 and the overview from the whole world to 106 of 128 columns — the owner had already
declined that trade on 2026-08-19. Keeping two columns was the fourth option and is what the finding rejects.

Smallest change that delivers it: `rows_per_entry` 2 → 3, `BAR_ROW_OVERHEAD` 35 → 19, and the `bar_width` divisor
4 → 2.

**Decision 14 — the `?` hint.** A permanent header segment, beside the run state, degrading to `?` alone at the floor.
This amends rule 5's closed header content list, which currently admits only draw failures, input failures, export
outcomes and overlay-only panes. The owner chose a permanent segment over a timed banner and over a footer entry, on
the ground that an operator who does not know the key is the one who needs it, and a hint that disappears is a hint
they will miss.

**Decision 15 — the hidden-pane notice.** Amend rule 5's Announcement obligation so the notice states the enlargement
remedy and the required dimension, not only the overlay key, and carries visual emphasis rather than `Span::raw` —
for example `inspector needs W>=140 — overlay: i`. The owner chose stating both remedies over replacing one with the
other, because the overlay is the remedy available now and the enlargement is the one that restores the layout.

## What this review did not decide

- **`VREC-MOK-005`'s lifecycle.** It is already `verified`, bound to commit `f361370`, and was verified on 2026-08-19
  with all seven assessments outstanding and all eleven provisions OUTSTANDING — which it discloses about itself. This
  review discharges what it disclosed; it does not revisit the verification act, and whether the chain needs a
  re-captured record is a separate assurance decision that was not taken here.
- **Whether `WO-MOK-005` changes status.** It is `implemented` and stays so.
- **The `[assurance]` classification of `WO-MOK-012`.** The agent proposed `not_required` with its reasoning; the
  engineering owner confirms or rejects it at approval. *Decided in a later turn the same day: the engineering owner
  approved this work order and confirmed the classification as a separately stated act — decisions 16 and 17, in
  `assurance-decision.md`. Neither was part of this review, and this list is left as the record of what the review
  itself left open.*
- **Any identifier for the second chain.** The sweep in `identifier-sweep.md` shows what was free on 2026-08-20, and
  the refs move; the chain re-sweeps rather than trusting that record.
- **Any push, pull request, tag or release.** None was authorized in this review.
