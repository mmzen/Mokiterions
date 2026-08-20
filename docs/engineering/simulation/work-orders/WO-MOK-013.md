+++
id = "WO-MOK-013"
type = "work_order"
title = "Make the observer's survival gauges resolve, its controls discoverable, and its hidden-pane notice actionable"
status = "draft"
owners = ["engineering owner"]
created = "2026-08-20"
updated = "2026-08-20"

[assurance]
commit_bound_verification = "required"
rationale = "This changes what the operator sees on the instrument every later phase is judged with, and the last presentation change argued rather than measured is the reason this work order exists: the fourth roster gauge was accepted on 2026-08-19 as a narrowing from six cells to two, and what shipped was a gauge with three renderable states for 101 values, which every automated case passed. The load-bearing claims here are claims about a rendered buffer at named and unnamed viewport sizes — that a ten-point value step moves a fill at every viewport presenting a bar row, that a character is present in the first frame with no input delivered, that an announcement states the threshold the layout itself decides presence from, and that no roster entry is lost without being counted as hidden. None can be asserted; each is a measurement against a commit. The change also moves the roster's row arithmetic, which SPEC-MOK-003 rule 4 fixes and three approved artifacts restate, and it touches SPEC-MOK-003 rule 5, whose gap in an earlier form let enlarging a terminal remove a pane. A regression here would misinform the judgement of every decision source Phase 2 and later add, and it would do so while the numbers beside the bars stayed correct."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-047", "REQ-MOK-048", "REQ-MOK-049"]
specifications = ["SPEC-MOK-003", "SPEC-MOK-004"]
verification = ["VER-MOK-013"]
+++

# Work Order: Make the observer's survival gauges resolve, its controls discoverable, and its hidden-pane notice actionable

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and the retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above. `VREC-MOK-013` is the record that binds it, and it does not
exist yet.

**The decision that blocked approval has been taken.** It was the choice between amending `REQ-MOK-020` and holding the
log at six rows; the product owner chose the latter on 2026-08-20, recorded as decision 1 in *Decision record* below.
Approval is no longer blocked by it. Nothing else about approval is settled: this work order is `draft`, no requirement
it implements is approved, and `VER-MOK-013` is `draft`.

**No amendment row in this chain is to be left `OUTSTANDING`.** Every specification amendment this work order needs is
stated below, in advance, in the terms the accountable owner must decide it in. The eleven provisions ratified on
2026-08-20 under `WO-MOK-012` were all drafted by implementation agents and left unratified for one or two days, and
`WO-MOK-005` records of five of them that it "is not verifiable until they are given or the scope is changed to avoid
needing them". That is the failure mode this section exists to prevent, and it is prevented by obtaining the
ratifications at approval rather than writing them afterwards.

## Objective

Three adverse observations were reported by the product owner from a live 200-tick pass on 2026-08-20, measured in
`evidence/WO-MOK-012/adverse-observations.md`, and directed to a separate chain by decision 12 of the closing review.
`WO-MOK-012` recorded them and completed. This is that chain.

At `ff3a155` the observer:

1. draws survival gauges two cells wide at the reference viewport, resolving 101 values into three states;
2. names the `?` key nowhere on screen, so the only documentation of its nineteen bindings is behind a key an operator
   cannot learn from it;
3. announces an excluded pane with the overlay key alone, unemphasised, naming neither the axis that excluded it nor the
   value that would restore it.

Make all three conform to `REQ-MOK-047`, `REQ-MOK-048` and `REQ-MOK-049`, and leave every other observable property of
the observer where it is.

## In scope

1. **The roster bar row's arithmetic**, so each gauge satisfies `REQ-MOK-047`'s ten-point property at the reference
   viewport. The settled design is two gauges per line, which raises the entry to three lines and the bar to **13**
   cells: `2 × (w + 6) + 5 + 2 ≤ 45` gives `w = 13`, and the row overhead becomes `5 + 2 × 6 + 1 × 2 = 19` with
   `bar_width(interior) = min(20, (interior - 19) / 2)`.
2. **A permanent header hint** naming the key rule 7 binds to the key-binding overlay, beside the run state, degrading
   to the key alone at the floor, satisfying `REQ-MOK-048`.
3. **The announcement's content and styling**, so it states the axis, the threshold value and the overlay key with
   emphasis, satisfying `REQ-MOK-049`. The threshold is read from the layout rather than restated.
4. **The `SPEC-MOK-003` amendments** the three require, enumerated below, each put to the technical owner at approval.
5. **The `SPEC-MOK-004` figure corrections** the added tests force — rule 11's test totals, arrival by arrival.
6. **The log's row count**, held at `6` wherever the log is present, per decision 1. This is what keeps `REQ-MOK-020`
   intact and it is the whole of what that decision changes.
7. **The `VER-MOK-005` matrix rows this chain makes false or stale** — three from decision 1 and one in its wording from
   `REQ-MOK-047`. An assurance-owner act on an approved contract, enumerated below. A fifth row is a pre-existing defect
   this chain surfaced; it is reported below and its disposition is not in this scope.
8. **The evidence `VER-MOK-013` contracts**, under `evidence/WO-MOK-013/`.
9. **A roadmap entry** for this chain.

## Out of scope

- **The engine.** `mokiterions-core` is not touched, in any file, for any reason. Its dependency table stays empty and
  its public interface stays item-for-item what `SPEC-MOK-002` rules 5 and 6 enumerate.
- **Any key binding.** Rule 7's table is read and not written. This work order advertises a binding and adds none.
- **Any pane threshold.** The roster's `W ≥ 100`, the inspector's `W ≥ 140`, the log's presence threshold `H ≥ 38` and
  the floor's `34 × 22` are untouched. Decision 1 changes how many rows the log occupies when present, not the viewport
  at which it appears, so no pane's presence changes at any viewport and monotonicity is not reached.
- **`REQ-MOK-020`.** Decision 1 was taken to leave it intact, and it is not amended, restated or reinterpreted here.
- **Widening the roster pane.** Four gauges of width 13 on one line need an 87-column interior and an 89-column pane.
  The technical owner declined the 61-column widening on 2026-08-19 because it takes fourteen columns from the map; 89
  takes forty-two. It is not reopened here.
- **The inspector, the export, the authority mapping, the provenance footer, the glyphs, the bands and the snapshot
  contract.** Rule 4 clause 7's three bands and clause 5's unbanded `fear` are carried onto the new row unchanged, and
  no boundary moves.
- **Manual assessment 7 of `VER-MOK-005`.** Outstanding by the assurance owner's decision of 2026-08-20, on the ground
  that the binary has no operator-reachable panic path. Nothing here creates one and nothing here closes it.
- **`VREC-MOK-005`'s staleness.** Whether the observer chain needs a re-captured verification record is undecided and is
  not decided by this work order. Note that this work order will make `VER-MOK-005`'s assessment 2 re-assessable, which
  is a fact about that record and not a change to it.
- **The two further observations** in `evidence/WO-MOK-012/adverse-observations.md` — log lines truncating mid-value
  without a marker, and `UNDERLINED` never occurring in the seed-42 run. They were recorded, not raised as findings, and
  carry no decision. They are not fixed here and not closed here.
- **Any push, pull request, tag or release.**

## Authorized decision envelope

The implementation may decide the exact wording of the hint and the announcement within what the amended rule 5 admits;
the abbreviation ladder between long and short forms; the concrete style used for emphasis, provided every distinction
survives the `(symbol, modifier)` projection under rule 2.5; the internal names of constants and helpers; and the
placement of new tests between the tiers `SPEC-MOK-004` rules 8 to 10 fix.

It may not decide any threshold, any figure rule 5 derives, any band boundary, any key binding, or the ten-point
granularity of `REQ-MOK-047`. `WO-MOK-005`'s envelope withholds rule 5's thresholds and rule 2's glyphs from the
implementation and that withholding carries here.

## Decision record

One decision has been taken on this work order. It is numbered in this work order's own sequence and does not continue
`WO-MOK-012`'s closing review, whose seventeen decisions are numbered there; where one of those bears on this work order
it is cited by its number and its document, as decision 13 is below.

Nothing else is decided. Approval, the six `SPEC-MOK-003` ratifications, the `VER-MOK-005` amendment, the
`commit_bound_verification` confirmation and the `WO-MOK-012` identifier collision are all open, and none of them is
settled by the decision recorded here.

### Decision 1 — the log is held at six rows — product owner, 2026-08-20

**Option B. The log is held at six rows, `REQ-MOK-020` is untouched, and all twelve entries stay visible at the
reference viewport.** The instruction, verbatim and complete:

> OK for option B

It named one act: the choice between the two options set out below. It approved no artifact of this chain, ratified no
amendment, and settled nothing about the `WO-MOK-012` identifier collision, which was put in the same turn and not
answered. The problem it resolved, and the two options as they were put, are retained below rather than replaced by
their outcome — the arithmetic is what makes the decision reviewable.

**At the reference viewport, `REQ-MOK-047` and `REQ-MOK-020` cannot both be satisfied at a 47-column roster and a
10-row log. The geometry, not a preference:**

| | |
|---|---|
| Reference viewport | `160 × 48`, fixed by rule 5 |
| Header, footer, log | `3 + 1 + 10 = 14` rows, so the body is `34` |
| Roster pane | `47` columns, body height; interior **`45 × 32`** |
| Two gauges per line | overhead `19`, bar `13` — satisfies `REQ-MOK-047` |
| Entry height | `1` identity line `+ 2` bar lines `= 3` rows |
| Twelve entries need | `36` rows in a `32`-row interior — **four short** |
| Four gauges of width 13 on one line need | an `87`-column interior, a `89`-column pane — the declined widening |

`REQ-MOK-020` obliges every living Mokiterion to be presented "without requiring the operator to scroll at the
reference viewport size", and `VER-MOK-005`'s matrix makes it an automated pass condition: "At `160 × 48` all twelve
two-line entries are present in the roster pane; none is hidden." `SPEC-MOK-003` rule 4 item 1 and its
*Examples and counterexamples* section both restate it.

**Decision 13 of 2026-08-20 chose ten of twelve visible.** The reasoning recorded in
`evidence/WO-MOK-012/closing-review.md` is that "the existing scrolling window already follows the selection, and the
roster title already states how many are hidden, so the two that fall off are announced by machinery that exists."
**That reasoning addresses announcement and does not address `REQ-MOK-020`, which was not put to the owner at the
time.** The agent did not measure the collision until authoring this pack. It is reported rather than resolved by
choosing quietly.

**Option A — take decision 13 as recorded.** Ten of twelve entries visible at the reference viewport.

- Amend **`REQ-MOK-020`** so the no-scroll obligation covers what the product owner actually requires — a product-owner
  act on an approved requirement, changing its `statement` field.
- Amend **`VER-MOK-005`**'s `REQ-MOK-020` matrix row, and rule 4 item 1, and the *Examples and counterexamples*
  sentence — an assurance-owner act on an approved contract, the second amendment to it in two days.
- Cost: two of twelve entries reachable only by scrolling, at the size the requirement was written to make sufficient.
- Touches three approved artifacts across three roles.

**Option B — hold the log at six rows at the reference height.** Twelve of twelve entries visible, `REQ-MOK-020`
untouched.

- Amend **`SPEC-MOK-003` rule 5** only: the log is `6` rows wherever it is present, dropping the growth to `10` at
  `W ≥ 140` and `H ≥ 48`. The body becomes `38`, the roster interior `36`, and `12 × 3 = 36` fits exactly.
- Consequential edits, all within rule 5: the derived table's `160 × 48` canvas becomes `67 × 36` from `67 × 32` and
  still presents the whole world, so its *Overview presents* column does not change; the non-monotone-area trade
  "Crossing `H = 48` at `W ≥ 140` grows the log from 6 rows to 10" is withdrawn; and the sentence "Where the log is 10
  rows it is `H ≥ 48`, which is the reference height" goes with it. No other derived row changes, because every other
  declared viewport already has a six-row log.
- Cost: four log rows at the reference viewport — six recent events visible instead of ten.
- **Monotonicity is unaffected**, since no pane's presence changes.

**This option was not among the three the owner was shown on 2026-08-20** and was reported because it is the only
arrangement satisfying `REQ-MOK-047` and `REQ-MOK-020` together.

**Correction to how option B was put.** It was put to the owner as touching "one approved artifact, in one role". That
was measured only against `SPEC-MOK-003` and it is wrong: **option B also amends `VER-MOK-005`**, in three matrix rows
enumerated under *Required `VER-MOK-005` amendment* below, because that contract fixes the reference canvas as `67 × 32`
and the log height as ten rows at `W ≥ 140` and `H ≥ 48`. So option B touches two approved artifacts in two roles, and
option A touched three in three. The comparison the owner drew still holds and the reason it holds is unchanged — option
B leaves an approved *requirement* intact where option A amends one — but the count put to them understated the work,
and the correction is recorded here rather than absorbed into the amendment list. The decision is not re-opened on it:
the difference is one approved artifact, in the role the option already required, and it does not bear on the trade the
owner was choosing between.

**Second correction, and this one is to the cost figure itself.** Option B's cost was put as "six recent events visible
instead of ten". Both numbers are pane rows, not events. The log pane's rows include its border in each axis, as rule 5
states for every canvas figure it derives, so a `10`-row log shows **8** records and a `6`-row log shows **4**. The
capture in `evidence/WO-MOK-005/frames.txt` measures it: the reference frame's log pane holds eight event lines between
its borders. **The real cost of option B is four visible events instead of eight** — half of them, not six-tenths. The
quantity lost is the same four either way, and the trade the owner weighed was four log rows against two roster
entries, which is unchanged. What is worse than described is the residual. **This correction is on the chosen side of
the trade and makes that side worse than it was put, so unlike the first it is a ground on which the product owner may
reasonably re-open decision 1.** It is reported for that purpose and no re-opening is assumed. Neither option-B
paragraph above is edited to the corrected figure: what was put to the owner is retained as put, and the correction
sits beside it.

**What the engineering owner recommended, with the decision remaining the product owner's.** Option B. The log is
scrollable by rule 9 item 1 — "Older retained events are reachable by scrolling within" — so its four rows cost reach,
not access, and an operator who wants an eleventh event can get it. The roster's no-scroll property is an approved
requirement precisely because paging a roster defeats it, which `REQ-MOK-020`'s rationale states in those terms.
Trading scrollable rows to preserve a no-scroll obligation is the cheaper trade, and it holds an approved requirement
rather than amending one. Against it: rule 5's own reasoning for admitting a six-row log at `H ≥ 38` is that "the log
carries the authoritative event stream", and option B makes the reference viewport show less of it than it does today.

**Option C, stated because it is the tempting one, and inadmissible.** Size the log from the living count, so the
roster gets its rows only while twelve are alive. This violates rule 5's opening obligation — "Layout is a pure function
of viewport width `W` and height `H`. It depends on nothing else — not tick, not run state, not entropy, not wall-clock
time" — and a layout depending on the population depends on the tick. It is not available at any price and is recorded
so it is not rediscovered as a compromise.

**Two consequences of the option taken, stated so neither is discovered later.**

**Entries still hide below the reference viewport, and that is not a regression.** At `160 × 44` the log is already six
rows, the body is 34, the roster interior 32, and three rows per entry shows 10 of 12. `REQ-MOK-020` binds at the
reference viewport size and rule 5 governs everything below it, so hiding there is the specified degradation and the
roster title states the count. Option B secures twelve entries at `160 × 48`; it does not secure them everywhere, and
nothing in this chain claims it does.

**The reference fit has no slack.** `12 × 3 = 36` fills a 36-row interior exactly.
`SPEC-MOK-001` bounds the population at twelve and calls a name table longer or shorter than the population
"a defect, not a spare", so the fit is safe today. A later phase raising the population above twelve breaks this
arrangement at the reference viewport, and it breaks it by hiding entries rather than by failing to build.

## Constraints

- **No engine change, and no perturbation.** The text stream, the event stream and the entropy state of a run must be
  byte-identical to `ff3a155`, observed or not. This is a presentation change and it is required to prove it.
- **`SPEC-MOK-003` rule 5's opening obligation holds absolutely.** Layout, the announcement and the hint are functions
  of `(W, H)` alone.
- **Monotonicity holds.** No pane present at a viewport may be absent at any larger one, checked over the plane.
- **The announcement stays an obligation ahead of optional detail.** `status_line` reserves its width before any
  optional segment, and the hint of `REQ-MOK-048` must not be satisfied by taking that reservation. Where a viewport
  cannot carry both in full, the amended rule 5 says which abbreviates — silently dropping either is a defect.
- **The floor does not move.** `34 × 22`, with the exit-`2` refusal and the current-and-required dimensions on standard
  error, is untouched.
- **Rule 2.5 holds.** Every distinction stays available without colour, including the new emphasis.
- **No band boundary, no glyph, no key, no export field and no snapshot field changes.**
- **The observer's public interface does not change.** `SPEC-MOK-004` rule 6's **94** items, **118** `pub` lines and
  **24** public fields are expected to be unmoved: `BAR_ROW_OVERHEAD` is a private constant, `bar_width` a private
  function, `rows_per_entry` a local. A movement in any of the three is a finding to report, not a figure to update
  quietly.
- **Every test that exists keeps its assertion.** `SPEC-MOK-004` rule 12's second paragraph is scoped to the
  `WO-MOK-006` restructuring as amended on 2026-08-19, so a test may be renamed here; an assertion may not be weakened.
  A test asserting two-cell bars at the reference roster is a test of the behavior being corrected and is updated with
  its reason recorded, not deleted.

## Expected change surface

Components rather than a promise about files. The observer package only.

| Component | Expected change |
|---|---|
| Observer rendering — header | A hint segment reserved rather than optional; the announcement given emphasis and the axis-and-value form |
| Observer rendering — roster | Row overhead `35 → 19`, `bar_width` divisor `4 → 2`, `rows_per_entry` `2 → 3` for the multi-gauge form |
| Observer layout | The log's row count, per decision 1: one function collapses to a constant and two of its three constants stop being read |
| Observer verification module | The declared-viewport table's reference row, whose canvas height changes with the log |
| Observer tests, both tiers | New cases per `VER-MOK-013`'s matrix; five existing cases carry the reference canvas or the ten-row log and are corrected; one loses its subject entirely |
| `SPEC-MOK-003` | The six amendments below — rule 4's bar row and its arithmetic, rule 5's admitted header content, its Announcement obligation, and its log row count |
| `SPEC-MOK-004` | Rule 11 figures, arrival by arrival |
| `VER-MOK-005` | Three matrix rows, enumerated below — an assurance-owner act |
| `docs/ROADMAP.md` | An entry for this chain |

`REQ-MOK-020` is absent from this table deliberately. Decision 1 exists to keep it out.

The named anchors were measured at `ff3a155` while authoring this work order and are stated as where to start, not as
the full extent:

| Anchor | Where | What it is |
|---|---|---|
| `BAR_ROW_OVERHEAD` | `render.rs:66` | The `35` that becomes `19` |
| `bar_width` | `render.rs:546` | The `/ 4` that becomes `/ 2` |
| `rows_per_entry` | `render.rs:429` | The local `2` that becomes `3` |
| `Span::raw` | `render.rs:178` | The announcement built without emphasis |
| `announcement_text` | `render.rs:187` | The text that gains the axis and the value |
| `ROSTER_WIDTH` | `layout.rs:23` | The `47` that does not change |
| `FULL_LOG_HEIGHT`, `COMPACT_LOG_HEIGHT` | `layout.rs:25`, `layout.rs:26` | `10` and `6`; only one survives decision 1 |
| `FULL_LOG_MIN_WIDTH`, `FULL_LOG_MIN_HEIGHT` | `layout.rs:34`, `layout.rs:35` | The `140` and `48` that stop being read for the log |
| `LOG_MIN_HEIGHT` | `layout.rs:33` | The `38` presence threshold, which does **not** change |
| `log_rows` | `layout.rs:96` | The three-branch function that becomes two |
| `RENDERABLE` | `verification.rs:59` | Row 1 `(160, 48, 67, 32)` becomes `(160, 48, 67, 36)` |

The five test cases carrying a figure decision 1 moves, each measured rather than expected:

| Test | Target | What moves |
|---|---|---|
| `the_log_is_ten_rows_only_where_both_thresholds_are_met` | `tests/layout.rs:57` | **Its subject ceases to exist.** There is no viewport at which the log is ten rows, so this is not a case to correct but a property that is withdrawn, and its removal is reported as such under rule 11 rather than counted as a lost test |
| `the_declared_viewports_yield_the_declared_canvases` | `tests/layout.rs:78` | The reference row's log rows and canvas height |
| `every_declared_viewport_has_its_derived_canvas_with_a_header_and_a_footer` | `tests/verification.rs:489` | The same figures, from the second tier |
| `every_declared_viewport_renders_and_annotates_what_it_presents` | `tests/render.rs:50` | The reference canvas height, and the whole-world annotation which must still hold at `67 × 36` |
| `the_roster_presents_four_gauges_at_every_declared_viewport_that_presents_it` | `tests/render.rs:653` | The entry height and the gauge width, from `REQ-MOK-047` rather than from decision 1 |

**No file under `evidence/` is edited, and neither is `VREC-MOK-005`.** `evidence/WO-MOK-005/frames.txt`,
`layout-and-viewports.txt`, `manual-assessment.md` and `requirement-to-test-mapping.md` all state the ten-row log or the
`67 × 32` reference canvas, and `VREC-MOK-005` lines 171 to 172 state all nine canvas interiors. Every one of them
records what was measured at a commit and remains true of that commit. The precedent is `evidence/WO-MOK-011/merge/`,
which re-measured superseded figures into a new directory rather than editing the originals, and `SPEC-MOK-003`'s own
rule that a capture is re-run rather than corrected. What decision 1 does to `VREC-MOK-005` is make its geometry figures
describe a superseded layout, which is a further instance of the staleness that record already discloses and is not a
change to it.

### Required `SPEC-MOK-003` amendments — technical owner, at approval

1. **Rule 4, the bar row.** The mockup, the prose and the arithmetic move from four gauges on one line to two gauges on
   each of two lines, and the entry from two lines to three. `bar_width(interior) = min(20, (interior - 35) / 4)` becomes
   `min(20, (interior - 19) / 2)`; the row overhead `5 + 4 × 6 + 3 × 2 = 35` becomes `5 + 2 × 6 + 1 × 2 = 19`; the
   reference roster's bars go from **2** cells to **13**. The 2026-08-19 reasoning that accepted the narrowing to two is
   retained in place rather than deleted, because it is what made two correct for a phase, and this amendment records
   that it was accepted as a narrowing and turned out to be a loss of the quantity.
2. **Rule 4, item 1 and the collapsed form.** Item 1's "Twelve living entries in the two-line form require 24 lines"
   becomes the three-line arithmetic. The collapsed one-line form below 47 columns is untouched — it has no bars, takes
   no band, and its four numeric values carry the level directly.
3. **Rule 4, clause 5 and clause 7.** Carried onto the new row unchanged. The three bands still apply to health,
   satiety and energy as whole gauges, `fear` still takes none, clause 4's zero rendering is unchanged, and clause 6's
   reversed-video selection still composes with the band across three lines instead of two.
4. **Rule 5, the header's admitted content.** The *Observability* section's closed list — draw failures, input failures,
   export outcomes, panes available only as overlays, hidden roster entries — gains the key-binding hint as a sixth
   admitted item, permanent rather than conditional, which is what makes it different in kind from the other five.
5. **Rule 5, the Announcement obligation.** The notice states the axis and the threshold value at which each excluded
   pane returns, in addition to the overlay key, and carries visual emphasis. The abbreviation ladder is fixed here, and
   so is which of the two remedies survives last at the narrowest viewports.
6. **Rule 5, the log's row count.** Per decision 1: the log is `6` rows wherever it is present, and the growth to `10`
   at `W ≥ 140` and `H ≥ 48` is withdrawn. The presence threshold `H ≥ 38` does not change. Six consequential edits,
   each located and each within rule 5 except the last:
   1. **The pane table**, whose log row reads "`10` rows when `W ≥ 140` and `H ≥ 48`, otherwise `6`; below the body" and
      becomes `6` rows unconditionally.
   2. **The derived table's `160 × 48` row**: panes "roster, inspector, log `10`" becomes log `6`, and the canvas `67 × 32`
      becomes `67 × 36`. Its *Overview presents* column does **not** change: rule 5 requires `Cw ≥ 64` and `Ch ≥ 32` for
      the whole world, `36 ≥ 32`, and 36 rows address 144 world rows of the 128 that exist. No other derived row moves,
      because every other declared viewport already has a six-row log — `160 × 44`, `160 × 40`, `140 × 44`, `140 × 43`
      and `120 × 48` each fail `W ≥ 140` or `H ≥ 48`, and `120 × 30`, `100 × 30` and `34 × 22` have no log at all.
   3. **The non-monotone canvas-area trade** "Crossing `H = 48` at `W ≥ 140` grows the log from 6 rows to 10" is
      withdrawn, leaving the two that remain. **This amendment removes a case in which enlarging the terminal made the
      canvas smaller**: `evidence/WO-MOK-005/layout-and-viewports.txt` line 106 measures `140 × 47 → 140 × 48` taking
      the canvas from `47 × 35` to `47 × 32`. Rule 5 declares that as a trade rather than a defect and it was a
      defensible one, but it is a real three-row loss under growth and decision 1 ends it. Canvas area becomes more
      monotone than the rule currently promises, not less, and pane-presence monotonicity is untouched either way.
   4. **The vertical fidelity sentence** "The vertical 1:1 threshold is `H ≥ 44`: `Ch ≥ 32` needs a body of 34 rows, and
      the header, footer and a 6-row log take 10 more. Where the log is 10 rows it is `H ≥ 48`, which is the reference
      height." The second sentence goes; the first becomes unconditional, since a six-row log is now the only log.
   5. **The `Examples and counterexamples` reference-viewport example**, which reads "the log has its full 10 rows. The
      canvas is 67 × 32 cells" and "The log shows 8 records". The log becomes 6 rows, the canvas `67 × 36`, and the
      records **4**: a log pane shows its rows less the two its border takes, which
      `evidence/WO-MOK-005/frames.txt` measures at eight lines in a ten-row pane. The same example's "Twelve roster
      entries are visible in the two-line form without scrolling" becomes the three-line form and keeps the word
      *twelve* — that half is amendment 2's, from `REQ-MOK-047`, and is named here so the sentence is not amended twice
      or once.
   6. **The amendment record row** states that the ten-row log was specified, approved and held for a phase, and was
      **traded rather than found wrong**. The reason rule 5 gives for admitting a log at all — that it "carries the
      authoritative event stream" — is retained in place, because it is what made ten rows right, and this amendment
      records that the reference viewport now shows less of that stream in exchange for an approved requirement it could
      not otherwise keep.

Each is put to the technical owner as its own question, enumerating its provisions, at approval. A single answer
covering all six would be an approval by implication of five.

### Required `VER-MOK-005` amendment — assurance owner, at approval

`VER-MOK-005` is an **approved** verification contract and this is the second amendment to it in two days, the first
being the eleven provisions ratified under `WO-MOK-012`. **Five of its matrix rows are affected, by three different
causes**, and the causes are separated because they call for different acts. Each row is quoted as it stands at
`ff3a155`.

**Three rows decision 1 makes false.**

| Row | Requirement | Pass condition as written | What decision 1 makes of it |
|---|---|---|---|
| Line 79 | `REQ-MOK-019` | "At `160 × 48` the canvas interior is 67 × 32 cells; every living Mokiterion and every standing resource is represented; no entity is omitted" | The figure becomes `67 × 36`. The rest of the row is unaffected: the whole-world claim holds a fortiori on a taller canvas |
| Line 130 | `REQ-MOK-024` | "The log occupies 10 rows when `W ≥ 140` and `H ≥ 48`, and 6 rows wherever else it is present" | The condition loses its ten-row clause and becomes 6 rows wherever the log is present. The case does not disappear — a constant is still a pass condition, and the presence threshold `H ≥ 38` it composes with does not move |
| Line 132 | `REQ-MOK-024` | "Interiors are 67 × 32, 67 × 32, 67 × 28, 47 × 32, 47 × 31, 71 × 36, 71 × 24, 51 × 24, and 32 × 16 respectively" | The **first** figure becomes `67 × 36`. The other eight are unchanged, and each was checked against the derived table rather than assumed |

**One row `REQ-MOK-047` makes stale in its wording only.**

Line 90, `REQ-MOK-020`: "At `160 × 48` all twelve two-line entries are present in the roster pane; none is hidden." **The
pass condition holds and is the one decision 1 was taken to preserve.** Only the words *two-line* are stale, since the
entry becomes three lines under amendment 2. The count, the viewport and the none-hidden obligation are untouched, and
`VER-MOK-013` adds a case asserting the same property so it is checked by both contracts.

**One row that is a pre-existing defect, surfaced by this chain and not caused by it.**

Line 92, `REQ-MOK-020`: "Each bar's filled cell count equals `round(value / 5)` of its twenty cells, and the numeric
value matches the snapshot." **This pass condition has never been satisfiable at any viewport, and it is not this work
order that broke it.** Measured at `ff3a155`:

- The roster pane is `47` columns at every viewport presenting it, so its interior is `45`. `bar_width(interior) =
  min(20, (interior - 35) / 4)` gives **2**. Twenty cells would need a `115`-column interior, which rule 5's fixed
  47-column pane cannot produce. Before `WO-MOK-010` the divisor was three over an overhead of 27, giving **6** at the
  same interior — `evidence/WO-MOK-005/frames.txt` shows those six-cell bars — and twenty would still have needed 87.
  **The row was unsatisfiable on the day it was approved.**
- The arithmetic differs as well as the width. `render.rs:572` computes `filled = (value × width / 100).min(width)`,
  which truncates; the row specifies `round`. At twenty cells and value 99 the two disagree, 19 against 20.

This bears directly on why this chain exists. **The one row of `VER-MOK-005` whose subject is the gauge's resolution was
written against a bar width the layout cannot produce**, so the contract's own check on fill granularity could not have
caught a two-cell gauge. `REQ-MOK-047` replaces the obligation with a property — a ten-point step moves the fill — that
holds at every width rather than at one that does not occur.

**Its disposition is the assurance owner's and is not decided here.** The options are to amend the row to the
implementation's arithmetic at the width the layout actually produces, or to withdraw it in favour of `VER-MOK-013`'s
ten-point cases, which cover the same ground without naming a width. `VREC-MOK-005` is `ready` rather than `verified`, so
no verified record asserts this row passed; that is a mitigation and not a reason to leave it. **This is a finding to
report, and it is reported rather than repaired quietly.**

**Two rows that look affected and are not**, stated so they are not amended without cause. The `REQ-MOK-024`
**monotonicity** row is untouched, because no pane's presence changes at any viewport. The `REQ-MOK-024` **whole-world
claim per viewport** row is untouched, because `160 × 48` is already on its whole-world side and stays there.

### Required `SPEC-MOK-004` amendment — technical owner, at completion

Rule 11's figures: the observer's **127**, the engine's **85** and the workspace's **212** are corrected for the tests
this work order adds, each arrival named from the target that runs it, in the form the `WO-MOK-011` row already uses.
The engine's 85 does not move. This one is at completion rather than approval because the figure is a measurement and
cannot be known before the work, which is the distinction rule 11 draws itself: "a work order that adds a test corrects
these figures here, and one that loses a test has a defect."

## Required verification

`VER-MOK-013` is the contract. In addition, and independent of it:

1. `cargo test --workspace` — every test passes, and the total is reconciled to rule 11 arrival by arrival.
2. `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` — clean.
3. `python scripts/validate_engineering_artifacts.py --root .` — PASS with zero errors and zero warnings on all four
   planes. **A new W-HEX-003 is expected** on every approved artifact this chain bumps `updated` on, and it must be
   reported with the count rather than absorbed: `WO-MOK-012` raised the inspector's warnings from 7 to 11 that way, and
   five of the eight W-HEX-003 observations at `ff3a155` are consequences of that bump rather than defects.
4. `git diff --stat origin/master -- mokiterions-core/` — empty.
5. The non-perturbation comparison at one declared seed, observed and unobserved, against `ff3a155`.

## Evidence to record

Under `evidence/WO-MOK-013/`, per `VER-MOK-013`'s *Evidence retention*, plus:

- **A `README.md` indexing every file**, on the `evidence/WO-MOK-012/README.md` pattern, stating what is not there.
- **The gauge-resolution table after the change**, read against `evidence/WO-MOK-012/assessment-material/bar-quantization.txt`,
  which is the same measurement before it.
- **Both manual assessments with an author, a date, a role and a terminal.** An assessment with no author is
  outstanding, and `VREC-MOK-005` disclosing seven such is what this chain descends from.
- **A closing review** recording every decision with the role that took it, on the `evidence/WO-MOK-012/closing-review.md`
  pattern, continuing this work order's own numbering from decision 1.
- **The log-height measurement after the change**: the reference frame's log pane with its event lines counted, read
  against `evidence/WO-MOK-005/frames.txt`, which is the eight-record before form. This is the figure both corrections
  to decision 1 turned on and it is measured rather than derived from the row count.
- **The frame captures**, by the oracle method: a program placed in the tree, run once, removed, its source retained as
  evidence. It asserts nothing. A capture is re-run rather than corrected.

## Stop and escalate conditions

Stop and report rather than deciding:

1. ~~**The open decision above, if it has not been taken.** Do not begin.~~ **Discharged 2026-08-20** by decision 1.
   This condition is struck rather than deleted, because it was the condition that held the work order shut and the
   record of what discharged it is part of the work order's history. It does not revive unless the product owner
   re-opens decision 1 on the corrected cost figure.
2. **Any further conflict between an approved artifact and one of the three requirements**, of the kind
   `REQ-MOK-020` turned out to be. The instruction is to measure the collision, state it with its arithmetic, and stop —
   not to pick the reading that lets the work proceed. **This condition has fired once already, before approval**: the
   `VER-MOK-005` line 92 defect above was measured, stated with its arithmetic and left to the assurance owner rather
   than corrected in passing. It is the pattern the condition asks for, and it is recorded here as one.
3. **Any figure in `SPEC-MOK-004` rule 6 moving.** The public interface is not expected to change. If it does, the
   change has left its scope.
4. **Any movement in the text stream, the event stream or the entropy state.** Non-perturbation is the property the
   whole observer rests on.
5. **A monotonicity violation at any viewport**, which is how the `160 × 40` missing roster reached the owner as a
   blocking defect on 2026-08-19.
6. **Any viewport above the floor that cannot carry both the announcement and the hint** in the form the amended rule 5
   fixes. This is a specification question and the amendment must answer it rather than the implementation choosing.
7. **A test whose assertion cannot survive**, as opposed to a test whose name or expected arithmetic changes.
8. **The `WO-MOK-012` identifier collision**, recorded in `evidence/WO-MOK-012/identifier-collision.md`, if it is
   resolved in a way that renumbers artifacts this work order cites.

## Completion report format

1. **Decision 1 as taken** — option B, product owner, 2026-08-20, the instruction verbatim — together with **both
   corrections to how the option was put**, since a completion report that repeats the figures the owner was shown
   without the figures that turned out to be true is not a record of what was built.
2. Each of the three requirements, with the measurement that shows it met and the before figure beside it.
3. Every `SPEC-MOK-003` and `SPEC-MOK-004` amendment made, each with the owner's act that ratified it and its date.
   **Any amendment row still reading `OUTSTANDING` is reported as an incomplete work order**, not as a footnote.
4. The complete change surface, `git diff --stat` against `origin/master`.
5. The test total, reconciled arrival by arrival, and the interface re-count.
6. The validator result and the inspector's warning count, with any new W-HEX-003 named and attributed.
7. Both manual assessments, with author, date, role and terminal.
8. What was left open, and what was decided not to do.
