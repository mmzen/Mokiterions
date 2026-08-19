+++
id = "VER-MOK-007"
type = "verification"
title = "Roster survival band verification"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
verifies = ["REQ-MOK-020"]
+++

# Verification Contract: Roster survival band verification

## Approval record

| Date | Change | Approval |
|---|---|---|
| 2026-08-19 | Original content, for `SPEC-MOK-003` rule 4 clause 7 under `WO-MOK-007`. | Approved 2026-08-19 by the repository owner as assurance owner, in one act with the `SPEC-MOK-003` rule 4 amendment this contract measures, with `WO-MOK-007`, and with the direction to implement. The implementation agent drafted this text and recorded this approval on the owner's explicit instruction; it holds authority over neither. |

## Independence

`REQ-MOK-020` is already verified by `VER-MOK-005`, which measured the roster's content, ordering, no-scroll
obligation and zero-versus-absent rendering. This contract does not re-measure those. It measures only what
`SPEC-MOK-003` rule 4 clause 7 adds, and it measures the one thing the addition puts at risk: that a colour was
added and nothing else changed.

Independence rests on three choices, each made so that a defect cannot pass by agreement between the test and the
implementation:

1. **The band table is stated here, not read from the source.** Every band assertion names its literal boundary
   value and its literal expected band. A test that imported the implementation's constants would pass whatever
   those constants said.
2. **Text identity is asserted against the specification's literal mockup**, not against the implementation's
   current output captured as a snapshot. A snapshot taken after the change would ratify a regression.
3. **The absence of a trend is asserted structurally**, over the observer's own state, rather than by reading the
   diff. A reviewer's eye is not evidence that no previous tick is retained.

The implementation may choose the concrete colour values. This contract therefore asserts *which band* a gauge is
in and *that the three bands differ from one another*, never a specific colour constant, except where a case must
confirm that two gauges in different bands do not share one style.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-020` — proportional indicator, unchanged form | automated test | `band-text-identity` | For every value in `0..=100` and every bar width the layout produces, a banded entry line's text equals the text the unbanded form produces; the twenty-cell case equals rule 4's mockup literally |
| `REQ-MOK-020` — indicator form per rule 4 clause 7 | automated test | `band-domain` | Every value in `0..=100` maps to exactly one band, with `80..=100` high, `40..=79` middle, `0..=39` low |
| `REQ-MOK-020` — indicator form per rule 4 clause 7 | automated test | `band-boundaries` | `39` and `40` are in different bands; `79` and `80` are in different bands; each of the four is asserted by its own literal value |
| `REQ-MOK-020` — a value of zero remains a zero | automated test | `zero-is-red-and-empty` | Zero is in the low band, renders as `0` with an empty bar, and remains distinct from the absent marker `—` |
| `REQ-MOK-020` — three attributes, independently presented | automated test | `three-bands-in-one-entry` | One entry whose health, satiety and energy fall in three different bands renders three gauges with three different styles |
| `REQ-MOK-020` — no derived survival quantity | automated test | `no-retained-tick` | The observer holds no previous-tick attribute value; advancing a tick and comparing state shows nothing retained beyond what exists before this change |
| `REQ-MOK-020` — selection distinguishable without colour | automated test | `selected-entry-composes` | A selected banded entry carries reversed video and its band; neither replaces the other |
| `REQ-MOK-020` — attributes the engine does not compute are absent | automated test | `reserved-slot-stays-empty` | The reserved fourth-bar slot carries no character and no style |
| `REQ-MOK-020` — collapsed form keeps the values | automated test | `collapsed-form-unstyled` | Below 47 columns the entry's text is unchanged and carries no band |

## Acceptance scenarios

### Scenario 1 — the specified entry, banded

**Given** the mockup's Mokiterion at health 100, satiety 81, energy 72 and a twenty-cell bar

**When** its entry is rendered

**Then** the second line's text is exactly rule 4's mockup line, and its three gauges are in the high, high and
middle bands respectively — 72 falling in the middle band is the case that distinguishes a band read from the value
from a band read from the row.

### Scenario 2 — a Mokiterion in trouble

**Given** a Mokiterion at health 44, satiety 8, energy 91

**When** its entry is rendered

**Then** health is in the middle band, satiety in the low band, energy in the high band, the three styles differ,
and the numeric values read `44`, `8` and `91` unchanged.

### Scenario 3 — the roster at the reference viewport

**Given** twelve living Mokiterions spanning all three bands

**When** a frame is drawn at the reference viewport size

**Then** twelve entries are present, the living count reads twelve, and the pane's text is identical to the text the
same state produced before clause 7 existed.

### Scenario 4 — starvation

**Given** a Mokiterion at satiety 0, which is the state in which `SPEC-MOK-001` applies the five-health penalty

**When** its entry is rendered

**Then** the satiety gauge is in the low band, its bar is empty, its value reads `0`, and it is distinguishable from
a gauge whose value is absent.

## Property and invariant tests

1. **Text identity.** For `value` in `0..=100` and every `bar` in the range `bar_width` produces across the widths
   the layout admits, the banded gauge's text equals the unbanded gauge's text. The case count is recorded. This is
   the property that makes every prior assertion about rule 4 still true.
2. **Total, disjoint banding.** Every value in `0..=100` is in exactly one band. No value is in none and no value
   is in two.
3. **Monotone banding.** The band never improves as the value falls: for every `v` in `1..=100`, the band of `v` is
   no worse than the band of `v - 1`. This is the property that would have caught the inverted trend encoding the
   owner rejected, and it holds for a level encoding by construction.
4. **Style locality.** Within one entry line, only the label character, the bar cells and the numeric value of a
   gauge carry that gauge's band. The five-column indent, the two-space separators and the reserved slot carry no
   band.
5. **Non-perturbation, unchanged.** Rendering a banded roster consumes no entropy and mutates no state, asserted by
   the same means `VER-MOK-005` used, so that this change cannot be the one that breaks the property.

## Static and architecture checks

1. `cargo fmt --check` clean.
2. `cargo clippy --workspace --all-targets` with no finding.
3. No file under `mokiterions-core/` differs from `master`, shown by diff and not by assertion.
4. `Cargo.toml` unchanged in both packages: no new dependency, no version or feature change.
5. `grep` shows no previous-tick field, no delta, and no comparison of two ticks anywhere in the observer's roster
   path.
6. The whole workspace test suite passes. Exactly two pre-existing unit tests change, and the change to each is the
   route by which it reaches the text, never the text it asserts. The before and after of each assertion is
   recorded verbatim.

## Security and privacy checks

The band is a function of one `u8` already present in a snapshot the observer already holds. No credential, secret,
environment variable, absolute path or wall-clock value can enter a band, and this change opens no input path, reads
no file and performs no network access. The existing checks under `VER-MOK-005` for the frame and the export remain
the authority for those surfaces and are unaffected: an export carries text, and no character of the text changes.

## Performance and resilience checks

1. A frame's cost is unchanged in order: banding adds a constant-time comparison per gauge and three styles per
   entry, so at most thirty-six comparisons per frame for twelve Mokiterions.
2. The roster renders at every viewport size from the floor upward without panic, including the widths at which
   `bar_width` returns zero and the entry collapses.

## Manual assessments

Recorded honestly as manual, with an author or as outstanding:

1. **The three bands are distinguishable on the owner's terminal.** Automated cases assert that styles differ; they
   cannot assert that a human sees the difference. This is the one assessment that matters for the feature's
   purpose and it requires the owner's eye on the owner's terminal.
2. **A selected entry in the low band remains readable**, where reversed video puts the band colour in the
   background.
3. **The middle band reads as orange rather than as yellow or brown** on the terminals the owner uses. If it does
   not, the palette is the implementation's to change under `SPEC-MOK-003`'s grant and no amendment is needed.

Each assessment states its environment. An assessment that cannot be performed from the implementation environment
says so and says why, rather than being marked passed.

## Evidence retention

Under `docs/engineering/simulation/evidence/WO-MOK-007/`, with a `README.md` indexing each file against the bullets
below:

1. workspace test output, before and after the change;
2. the band domain sweep and its case count;
3. the two boundary cases, each by literal value;
4. the text-identity sweep and its case count;
5. the reference-viewport roster captured as text, spanning three bands;
6. `cargo fmt --check` and `cargo clippy` output;
7. the empty diff of `mokiterions-core/` against `master`;
8. the `grep` results for retained-tick and delta patterns;
9. `harnessctl validate`, `preflight --phase review`, `inspect` and `dashboard` output;
10. the verbatim before and after of each changed test assertion;
11. `manual-assessment.md`, with the three assessments above and an author or an explicit outstanding marker for
    each;
12. `completion-summary.md`, numbering every disclosure.

## Residual uncertainty

1. **Every automated case is a claim about an in-memory character buffer and a style table, not about pixels.** No
   automated case can show that a colour arrived at a terminal, and none is offered as showing it. This is the same
   boundary `VER-MOK-005` recorded, and it is why manual assessment 1 exists.
2. **Whether the band boundaries are the right ones is not a verifiable question.** They are the technical owner's
   choice, recorded in `SPEC-MOK-003`'s amendment record. This contract measures that the implementation matches
   them, not that they serve the operator well.
3. **The bands say nothing about direction.** A Mokiterion at satiety 80 and falling and one at satiety 80 and
   rising are the same green. This is the deliberate consequence of the owner's decision to encode level rather
   than trend, taken after the measurement that a trend encoding inverts, and it is a known limit of the display
   rather than a defect in it.
4. **Colour is redundant here by construction**, so a monochrome terminal loses nothing this roster asserted before
   the change. That is a claim about the encoding, verified by the text-identity property, not a claim that a
   monochrome terminal was tested.
