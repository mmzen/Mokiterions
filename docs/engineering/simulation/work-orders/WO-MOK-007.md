+++
id = "WO-MOK-007"
type = "work_order"
title = "Colour the roster survival bars by value band"
status = "approved"
owners = ["engineering owner"]
created = "2026-08-19"
updated = "2026-08-19"

[assurance]
commit_bound_verification = "required"
rationale = "This changes what the observer asserts about how close a Mokiterion is to dying, on the instrument every later phase uses to judge whether a decision source behaves plausibly. A band applied to the wrong attribute, or an off-by-one at a boundary, would misinform that judgement in the direction of false reassurance, and it would do so silently because the number beside the bar would still be right. The change also converts the roster entry from plain text to styled spans, so the claim that no rendered character moves is a claim about the presentation contract `REQ-MOK-020` and `SPEC-MOK-003` rule 4 fix, and it must be bound to a commit rather than asserted."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-020"]
specifications = ["SPEC-MOK-003"]
verification = ["VER-MOK-007"]
+++

# Work Order: Colour the roster survival bars by value band

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

No `architecture` relation is declared. `ARCH-MOK-001` addresses `REQ-MOK-004`, `REQ-MOK-008`, `REQ-MOK-009`,
`REQ-MOK-010` and `REQ-MOK-016`; `ARCH-MOK-002` addresses `REQ-MOK-021`, `REQ-MOK-025`, `REQ-MOK-026` and
`REQ-MOK-028`. No active architecture addresses `REQ-MOK-020`, the only requirement this work order implements, so
the relation is omitted rather than fabricated.

## Approval preconditions

**Both provisions below were approved on 2026-08-19 by the repository owner as technical owner and are applied to
`SPEC-MOK-003`.** The owner reviewed this text and approved it together with this work order and `VER-MOK-007`, in
one act, having first been shown that the trend encoding their request asked for inverts. The implementation agent
drafted the provisions and recorded the approval; it decided neither. The provisions are retained here verbatim
because this work order's scope is defined by them, and because the amendment record in `SPEC-MOK-003` cites this
document for the palette.

**One amendment to an approved specification had to be applied first.** `REQ-MOK-020` states that "indicator form"
is fixed by `SPEC-MOK-003`, and rule 4 currently specifies each bar as "a twenty-cell proportional bar and a
numeric value" with no colour. Colouring it changes indicator form, and the band boundaries are exactly the kind of
threshold rule 4's *Explicitly unspecified decisions* withholds from the implementation. The amendment is the
technical owner's act. Its text is stated in full below so that approving it requires no drafting.

### Provision 1 — `SPEC-MOK-003` rule 4 gains clause 7

Insert after clause 6:

> 7. **Survival bands.** Each of the three bars carries a colour band read from the value it presents: green at
>    `80..=100`, orange at `40..=79`, red at `0..=39`. The band applies to the gauge as a whole — its label
>    character, its bar cells and its numeric value — so one gauge reads as one state; the two spaces separating
>    gauges and the five-column indent are unstyled. A band is a second presentation of the number the bar already
>    shows. It introduces no quantity the engine does not compute, no trend, and no threshold borrowed from
>    anything else: in particular it is not `SPEC-MOK-001`'s reference sleep threshold, which is one decision
>    source's policy and not a survival state, and which a Phase 2 decision source may not share.
>    `REQ-MOK-020`'s constraint against derived survival estimates is therefore unaffected. Level stays available
>    without colour through the numeric value and the proportional fill, so rule 2.5 holds and colour is redundant
>    reinforcement here as everywhere else. Zero takes the red band and still renders as `0` with an empty bar
>    under clause 4, which remains what distinguishes it from an absent value. Banding changes no character of the
>    entry: the rendered text of both forms is identical with and without it, and clause 4's mockup stands
>    unchanged. A selected entry's reversed video composes with the band rather than replacing it, so clause 6 is
>    unaffected; the band colour becomes the reversed cell's background there, and selection remains marked by
>    reversal rather than by colour. The collapsed one-line form below 47 columns has no bars and takes no band:
>    its three numeric values are unstyled, because that form exists to keep the numbers legible where the bar
>    cells will not fit, and the numbers carry the level directly.

### Provision 2 — amendment record row

Append to the *Amendment record* table:

> | 2026-08-19 | **Rule 4 gains clause 7, survival bands.** The roster's three bars take a colour read from the value they present — green `80..=100`, orange `40..=79`, red `0..=39` — applied to each gauge's label, bar cells and numeric value together. The bands are a presentation of a number the bar already carries, so no quantity the engine does not compute enters the roster and `REQ-MOK-020`'s constraint against derived survival estimates is untouched; nor is any threshold borrowed from `SPEC-MOK-001`'s reference sleep threshold, which is a decision source's policy rather than a survival state. Zero is red and clause 4's rendering of `0` as `0` with an empty bar is unchanged, as is what distinguishes zero from an absent value. Rule 2.5 holds without amendment because the numeric value and the proportional fill already carry level without colour, which is what makes a band redundant reinforcement rather than the sole carrier of a distinction. No character of the entry moves and clause 4's mockup is unchanged. Clause 6's reversed-video selection composes with the band. The collapsed one-line form takes no band, stated in the clause and reversible by one sentence. No other rule, figure, glyph, key binding, export, authority mapping or snapshot contract changes. | Approved 2026-08-19 by the repository owner as technical owner, who fixed the three bands in the same act after being shown that a trend-based encoding inverts: `SPEC-MOK-001` decays satiety and energy by one each tick for every living Mokiterion, so "decreasing" is true of nearly every bar on nearly every tick and would become false only at zero. The implementation agent measured that, drafted this text and recorded this approval on the owner's explicit instruction; it holds authority over neither. The concrete colour values are the implementation's under this specification's grant of "the exact palette, provided every distinction remains available without colour", and are recorded in `WO-MOK-007`. |

## Objective

Give each of the roster's three per-Mokiterion bars a colour that states how close that attribute is to spent, so
that an operator scanning twelve entries sees which Mokiterions are in trouble without reading twenty-four
numbers. Add nothing else, and move no rendered character.

## In scope

1. `SPEC-MOK-003` rule 4 clause 7 as approved above: the three bars of the roster's two-line entry take the green,
   orange and red bands at `80..=100`, `40..=79` and `0..=39`.
2. The band applies to the whole gauge — label character, bar cells, numeric value — and not to the indent or the
   separators.
3. The roster entry becomes styled spans rather than plain strings, which is what allows three independently
   coloured gauges on one line. The selected entry's reversed video is preserved and composes with the band.
4. Adapting the two existing unit tests that assert `entry_lines`'s strings, so that they assert the same strings
   through the styled line's own text. The assertions keep their current text exactly; only how they reach it
   changes.
5. New tests for the bands, the boundaries, and the byte-identity of the rendered text before and after.

## Out of scope

- **Any trend.** Nothing retains a previous tick, compares two ticks, or reads a direction of change. The observer
  keeps one snapshot, replaced wholesale, exactly as it does now.
- **The collapsed one-line form** below 47 columns, whose three numeric values stay unstyled. Clause 7 states this,
  and extending the band to it is one sentence of amendment and about three lines of code if the owner wants it.
- **The inspector's** health, satiety and energy values. Same three attributes, but rule 6 fixes that pane's
  content and this work order amends only rule 4.
- **The spatial view's palette**, including `LOW_COLOUR`, `MEDIUM_COLOUR` and `HIGH_COLOUR` for resource class.
- Any change to the engine package, to any snapshot type, to `AgentSnapshot`, or to the event log.
- Any lifecycle status of any artifact, including this one.
- Any threshold other than the two the amendment fixes.

## Authorized decision envelope

The implementation agent may decide:

- the concrete colour values, under `SPEC-MOK-003`'s grant of "the exact palette, provided every distinction
  remains available without colour". The intended choice, to be recorded in the completion report and correctable
  by the owner in one word, is `Color::Green` for the high band, `Color::Indexed(208)` for the middle band, and
  `Color::Red` for the low band. `Indexed(208)` is xterm's dark orange, chosen over `Color::Yellow` because
  `MEDIUM_COLOUR` is already `Color::Yellow` for a medium-class resource in the spatial view, and two unrelated
  meanings sharing one colour on one screen is avoidable here. On a terminal without 256-colour support the
  middle band degrades to that terminal's nearest colour, which costs nothing that rule 2.5 relies on, because
  the numeric value and the fill still carry the level;
- how the band function and the styled entry are decomposed into private functions, types and signatures;
- whether the selected entry's reversed video is applied to the line or to each span, provided the result is
  reversed video over a banded gauge;
- test names, fixtures and helpers within their tier.

The implementation agent may **not** decide: the band boundaries or the number of bands; whether zero is banded;
whether any trend is computed; whether the collapsed form or the inspector is banded; the rendered text of any
entry; or any lifecycle status.

## Constraints

1. **No rendered character changes.** For every value in `0..=100` and every bar width the layout can produce, the
   text of a banded entry line equals the text the current implementation produces. This is the constraint that
   keeps rule 4's mockup true and keeps the existing assertions meaningful.
2. **Present no value the engine does not compute.** The band is a function of one `u8` the engine produced. No
   previous value is retained, no delta is formed, and no time-to-death is estimated. `REQ-MOK-020`'s constraint
   is not amended and must still hold literally after this change.
3. **Every distinction stays available without colour.** The numeric value and the proportional fill both carry
   level. A reader on a monochrome terminal loses nothing the roster asserted before this change.
4. **No new dependency**, no change to `ratatui`'s version or feature set.
5. **The engine package is not touched.** No file under `mokiterions-core/` differs from `master`.
6. **Every existing test passes**, with the sole exception of the two named in scope item 4, whose asserted text is
   unchanged and whose adaptation is mechanical. Any other test that needs changing is a stop condition.
7. **Rule 4.5's reserved fourth-bar slot stays empty** — no label, no dash, no zero, and now also no colour.
8. Displaying a band consumes no simulation entropy and mutates nothing.

## Expected change surface

- `mokiterions-tui/src/render.rs`, roster section: `entry_lines` changes its return type from `Vec<String>` to a
  styled line sequence; `gauge` returns a styled gauge rather than a `String`; a band function is added; the caller
  at the roster loop composes the selection style with the band instead of wrapping one span. `ratatui`'s `Line`
  implements `Display` by concatenating its spans' content, which is what makes constraint 1 checkable directly and
  what lets the two existing assertions keep their text.
- `mokiterions-tui/src/render.rs`, unit tests: `the_bar_row_reproduces_the_specified_form` and
  `a_zero_value_is_a_zero_and_an_absent_value_is_a_dash` reach the same text through the line rather than through a
  `String`. `a_bar_row_shrinks_to_its_pane_and_never_overflows_it` asserts only on `bar_width` and is untouched.
- New tests for band assignment, both boundaries, the reserved slot, and the text-identity sweep.
- `docs/engineering/simulation/specifications/SPEC-MOK-003.md`, if the owner directs the agent to apply the two
  approved provisions rather than applying them personally.
- No other file. In particular no file under `mokiterions-core/`, no manifest, and no other artifact.

## Required verification

`VER-MOK-007` is the contract. It must at minimum measure:

1. Band assignment over the whole domain `0..=100`, with the boundaries at 39/40 and 79/80 asserted individually
   rather than inferred from a range.
2. Text identity: over every value in `0..=100` and every bar width the layout produces, the banded line's text
   equals the unbanded text, asserted against the literal form rule 4's mockup fixes.
3. The mockup line itself, unchanged, at the full twenty-cell width.
4. Zero: red band, `0` with an empty bar, still distinct from `—`.
5. The three gauges of one entry carrying three different bands simultaneously, since one shared style would be
   the obvious defect and would pass a single-gauge test.
6. A selected banded entry: reversed video present, band present, neither replacing the other.
7. The collapsed one-line form: unstyled, and its text unchanged.
8. That no previous-tick state was introduced — asserted structurally, not by inspection.

## Evidence to record

Under `docs/engineering/simulation/evidence/WO-MOK-007/`, keyed to this work-order ID, with a `README.md` indexing
each file against the retention bullets of `VER-MOK-007`:

- the test output for the whole workspace, before and after;
- the band domain sweep and the two boundary assertions;
- the text-identity sweep, with its case count stated;
- the rendered roster at the reference viewport, captured as text, showing three bands in one pane;
- `cargo fmt --check` and `cargo clippy` output;
- a diff of `mokiterions-core/` against `master`, expected empty;
- `harnessctl validate`, `preflight --phase review`, `inspect` and `dashboard` output;
- a completion summary numbering every disclosure, including the palette choice and anything the agent decided.

## Stop and escalate conditions

Stop and escalate before continuing when:

- either provision above is unapproved when implementation would begin, since the band boundaries are the
  technical owner's and this work order cannot supply them;
- constraint 1 cannot be met — that is, colouring cannot be added without moving a character — because the
  specification's mockup would then have to change and that is a different amendment;
- any test outside the two named in scope item 4 requires modification;
- meeting the request appears to need a previous tick, a delta, or any value the engine does not compute;
- the reserved fourth-bar slot would acquire a colour, a label, a dash or a zero;
- a band boundary appears to need to differ per attribute, since the amendment fixes one table for all three;
- `harnessctl validate` or `preflight` reports anything the change cannot account for.

## Completion report format

A `completion-summary.md` under this work order's evidence directory, numbering each disclosure, stating:

1. what was implemented, with rule 4 clause 7 read back out of the source rather than restated from the
   specification;
2. the concrete palette chosen and why;
3. the text-identity result, with the number of cases swept;
4. the two boundary results;
5. every existing test that changed, with what its assertion asserted before and after;
6. the gate results, each as the command and its output;
7. everything the agent decided under the envelope above;
8. everything it did not do, and every stop condition it came close to.
