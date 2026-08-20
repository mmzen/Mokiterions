+++
id = "VER-MOK-013"
type = "verification"
title = "Legibility verification: a gauge that resolves, a control the operator can find, and a notice that names the remedy"
status = "draft"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-20"

[relations]
verifies = ["REQ-MOK-047", "REQ-MOK-048", "REQ-MOK-049"]
+++

# Verification Contract: a gauge that resolves, a control the operator can find, and a notice that names the remedy

## Independence

**This contract has to answer for its own predecessor.** All three requirements it verifies exist because a person ran
the observer and reported what an automated suite had passed. `VER-MOK-005` holds 40 automated cases over the roster,
the layout and the header; every one passed at `ff3a155`; and at that commit the survival gauges were two cells wide,
the `?` key appeared nowhere on screen, and the hidden-pane notice named the wrong remedy in unstyled text. The suite
was not wrong. It asserted what it had been told to assert.

Two consequences are built into this contract rather than left as good intentions.

**Every automated case here asserts a property, not a rendering.** A case that asserts a gauge is thirteen cells wide
passes on the day it is written and says nothing about whether a gauge resolves a value. The cases below assert that a
ten-point change moves the fill, that the character `?` is present in the buffer without any input having been
delivered, and that the announcement contains the axis and value the layout itself computes. Each would fail if the
implementation met the letter of its requirement and lost its purpose, which is what the two-cell gauge did.

**The manual assessments are the primary evidence for two of the three requirements, and they are named as such.**
Legibility and discoverability are properties of a person reading a screen. `REQ-MOK-047` and `REQ-MOK-048` are
verifiable automatically only in their negative direction — a suite can prove a gauge is too coarse to carry a value and
can prove `?` is absent, and it cannot prove the result is readable or that someone who did not know the key would find
it. An adverse manual assessment against a passing automated case is what produced this chain, and under this contract
it retains the standing `VER-MOK-005` gives it: an artifact decision, not a defect to patch.

Independence from implementation choice is preserved as `VER-MOK-005` preserves it. No case names a constant, a
function, a module path or a colour. `bar_width`, `BAR_ROW_OVERHEAD` and `rows_per_entry` appear nowhere below; they are
`WO-MOK-013`'s to change and this contract does not know them.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-047` | automated-test | A ten-point step moves the fill, over the whole value range | For every gauge the reference viewport presents and every `v` in `0..=90`, the filled-cell count at `v + 10` exceeds the count at `v` |
| `REQ-MOK-047` | automated-test | The gauge resolves at every viewport that presents a bar row | At every viewport of rule 5's derived table presenting the roster in its multi-line form, the obligation above holds; where rule 4 collapses the entry to numeric values, no gauge is drawn and the case is vacant rather than passing by absence |
| `REQ-MOK-047` | automated-test | The values are the engine's | Each gauge's numeric value equals the corresponding field of the `AgentSnapshot` for that identifier, unchanged from `VER-MOK-005`'s obligation |
| `REQ-MOK-047` | manual | Reference-viewport legibility, re-assessed | A person at `160 × 48` reports that a declining attribute is visible as a declining bar. This is the assessment that failed on 2026-08-20 and it is re-taken, not inherited |
| `REQ-MOK-048` | automated-test | The overlay key is on screen from the first frame | With no input delivered, the rendered buffer of the first frame contains the character bound to the key-binding overlay by rule 7 |
| `REQ-MOK-048` | automated-test | It survives every viewport above the floor | The same holds at every viewport of rule 5's derived table and at the floor `34 × 22` |
| `REQ-MOK-048` | automated-test | It is not timed | The character is present in the buffer at frame 1 and after 200 ticks of drawing, in both run states |
| `REQ-MOK-048` | automated-test | It displaces no obligation | At every viewport above the floor the announcement of rule 5, where one is due, and the provenance footer of rule 8 are both present in full alongside the hint |
| `REQ-MOK-048` | manual | Discoverability | A person who has not read `SPEC-MOK-003` rule 7 is shown one frame and asked how to see the controls |
| `REQ-MOK-049` | automated-test | The announcement names the axis and the value | At every viewport of rule 5's derived table excluding at least one pane, the announcement contains, for each excluded pane, the axis that excludes it and the threshold value at which it returns |
| `REQ-MOK-049` | automated-test | The stated value is the layout's own | The value in the announcement is obtained from the same source the layout decides presence from; a test that fixes a literal `140` in its expectation is not admissible for this row |
| `REQ-MOK-049` | automated-test | The overlay key is retained | The announcement still names the key that opens each excluded pane as an overlay |
| `REQ-MOK-049` | automated-test | The announcement is emphasised | The announcement's cells carry a style differing from the cells of the optional header segments in the same line |
| `REQ-MOK-049` | automated-test | Legible without colour | Over a projection holding only `(symbol, modifier)` per cell, the axis, the value and the key are all still readable — the projection `VER-MOK-005` established for rule 2.5 |
| `REQ-MOK-049` | automated-test | Every excluded pane is announced | At `34 × 22`, where all three panes are excluded, all three are named with their axis and value |

## Acceptance scenarios

1. **The declining Mokiterion.** Run 200 ticks at a declared seed at the reference viewport. For one Mokiterion whose
   health falls by at least thirty over the run, the filled-cell count of its health gauge is non-increasing over that
   fall and strictly smaller at the end than at the start. This is the scenario the two-cell gauge fails: at
   `ff3a155` a fall from 99 to 50 leaves the count at 1.
2. **The cold start.** Draw exactly one frame with no input delivered, at the reference viewport and at the floor. The
   overlay key is on screen in both.
3. **The narrowed terminal.** At `120 × 48` the inspector is excluded. The announcement states the axis, the value and
   the key, is emphasised, and remains legible in the `(symbol, modifier)` projection.
4. **The floor.** At `34 × 22` all three panes are excluded and the hint reduces to its shortest form. All three
   announcements and the hint are present, and rule 8's footer is intact.
5. **Resize across a threshold.** Cross `W = 140` in both directions. The announcement appears and disappears with the
   pane, states the same threshold value each time, and rule 5's monotonicity obligation is not violated in either
   direction.

## Property and invariant tests

- **Gauge resolution over the whole plane.** For every viewport in `34..=200 × 22..=60` above the floor, and every
  gauge the roster draws there, either no bar is drawn or a ten-point value step changes the fill. Checked over the
  plane rather than at named sizes, on the model of rule 5's monotonicity obligation, because a named-size check is what
  let the two-cell gauge pass.
- **No entry is lost silently.** For every viewport presenting the roster, the number of entries drawn plus the number
  the roster title reports hidden equals the living count. This is the invariant that makes an entry falling off the
  pane an announced fact rather than a disappearance, and it is load-bearing for whichever resolution `WO-MOK-013`'s
  open decision takes.
- **Layout purity.** Layout, the announcement and the hint are functions of `(W, H)` and the excluded-pane set alone.
  Same dimensions, same result, at any tick, in any run state, at any entropy state.
- **Non-perturbation.** The text stream, the event stream and the entropy state of a run observed with these changes
  are byte-identical to the same run observed at `ff3a155` and to the same run unobserved. This restates
  `VER-MOK-005`'s obligation because a presentation change must not be trusted to have kept it.

## Static and architecture checks

- `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean.
- `SPEC-MOK-002` rules 5 and 6: the engine's public interface is unchanged, item for item. Nothing in this chain adds an
  interface item, and the enumeration is re-counted rather than assumed.
- `SPEC-MOK-004` rule 6: the observer's public interface is re-counted against its recorded **94** items, **118** `pub`
  lines and **24** public fields. The changes are expected to touch private constants and private functions only, so a
  movement in any of the three figures is a finding rather than a figure to update.
- `SPEC-MOK-004` rule 11: the workspace test total is re-counted from its recorded **212** and the arrivals are named
  one by one, from the target that runs each. A total that moves without a named arrival is a lost test.
- The engine package's dependency table stays empty.

## Security and privacy checks

The announcement states a viewport dimension and a key. The hint states a key. Neither carries a path, an environment
variable, a credential or a wall-clock time, and `SPEC-MOK-003` rule 8's prohibition on all four in the footer is
unaffected. No observer diagnostic reaches an export.

## Performance and resilience checks

- Rule 6's frame budget of at most one frame per `33` milliseconds is met, and the added work is per-frame string
  formatting bounded by the viewport width.
- A resize below the floor mid-run still suspends drawing, presents nothing, does not terminate the run, and resumes.
- A draw failure is still reported in the header, and the run still continues.

## Manual assessments

Two, and they are the primary evidence for their requirements rather than a confirmation of the automated cases. The
form is `VER-MOK-005`'s: the assessment names its subject, and an adverse outcome is an artifact decision requiring
product review.

- **Reference-viewport gauge legibility.** At `160 × 48` on a real terminal, over a run of at least 200 ticks on a
  declared seed, confirm that a Mokiterion's decline is visible as a change in the bar and not only in the number.
  **This is assessment 2 of `VER-MOK-005` re-taken.** It returned an adverse observation on 2026-08-20 — recorded in
  `evidence/WO-MOK-012/manual-assessment.md` as "SATISFIED as to the subject `VER-MOK-005` enumerates. ADVERSE
  OBSERVATION on the roster gauges" — and it is the assessment this chain exists to let pass. It is re-taken against
  the new implementation and its earlier outcome is not inherited in either direction.
- **Overlay discoverability.** Show one frame to a person who has not read `SPEC-MOK-003` rule 7 and ask how they would
  find out what the keys do. A pass is that they name the key from the screen. **An automated case cannot perform this
  assessment**, and the automated case beside it — that the character is in the buffer — is a necessary condition and
  not the property. If the assessor finds the hint but cannot tell what it opens, that is an adverse observation on the
  wording and belongs to `SPEC-MOK-003` rule 5.

**No third manual assessment is contracted for `REQ-MOK-049`.** The announcement's content is fully checkable — the
axis, the value, the key and the emphasis are all assertable against a buffer — and contracting a manual assessment for
a property a machine settles would put a person's signature where it adds nothing. The wording's usefulness is assessed
under the discoverability assessment above, which sees the same header line.

## Evidence retention

Retained under `evidence/WO-MOK-013/`:

- The automated run output, with the per-target test counts and the workspace total reconciled to `SPEC-MOK-004`
  rule 11's recorded figures, arrival by arrival.
- A frame capture at every viewport of rule 5's derived table plus the floor, showing the header line in full and one
  complete roster entry. Captured by the oracle method `WO-MOK-010` and `WO-MOK-012` used — a program placed in the
  tree, run once, removed, its source retained. **A capture is re-run rather than corrected**, which is the precedent
  `SPEC-MOK-003`'s amendment record states for `WO-MOK-010`'s roster frames and `evidence/WO-MOK-011/merge/` sets for
  re-measuring rather than editing.
- The gauge-resolution table: for each gauge width the implementation can produce, the filled-cell count at every value
  in `0..=100`, so the ten-point property is inspectable and not only asserted. `bar-quantization.txt` under
  `evidence/WO-MOK-012/assessment-material/` is the before form of exactly this table and is what the after form is read
  against.
- The interface re-count for `SPEC-MOK-004` rule 6 and the engine surface re-count for `SPEC-MOK-002` rules 5 and 6.
- The non-perturbation comparison: text stream, event stream and entropy state, observed and unobserved, at the same
  seed.
- Both manual assessments, each with its author, its date, its role, and the terminal it was performed on. An
  assessment with no author is outstanding, which is the condition `VREC-MOK-005` disclosed for all seven of
  `VER-MOK-005`'s and which this contract is written to avoid repeating.

## Residual uncertainty

- **The discoverability assessment is not repeatable.** Once a person has seen the hint they cannot un-know the key, so
  the assessment is available once per assessor. A second run of it on the same person confirms nothing. This is stated
  rather than engineered around, and it is the reason the automated case beside it is contracted as a necessary
  condition.
- **A person's terminal is not the buffer.** Emphasis is asserted as a style on a cell; whether a given terminal
  renders that style distinguishably is outside the workspace, which is the same limit `VER-MOK-005`'s modifier
  assessment records. Rule 2.5's redundancy is what keeps this from being load-bearing: the words carry the notice
  without the style.
- **The ten-point step is a chosen granularity.** `REQ-MOK-047` fixes ten because a ten-point move in a `0..=100`
  survival attribute is the coarsest change an operator should never miss. A finer obligation would demand a wider
  gauge and more of the roster; the figure is the product owner's and this contract verifies it rather than defending
  it.
- **What this contract cannot verify is whether the trade `WO-MOK-013` resolves was the right one.** If the open
  decision is taken toward showing ten of twelve entries, every case here can pass while an operator is worse off for
  the two entries they must scroll to; if it is taken toward a six-row log, every case here can pass while an operator
  is worse off for the four log rows. **That is a product judgement and no verification case reaches it.** The
  reference-viewport manual assessment is the closest this contract comes, and it is deliberately worded to let an
  assessor report it.
