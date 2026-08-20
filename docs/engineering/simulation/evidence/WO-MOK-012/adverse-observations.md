# Three adverse observations from the assessment pass of 2026-08-20

The repository owner reported three findings from the live pass. One arose under manual assessment 2; the other two
were volunteered and **no assessment in `VER-MOK-005` asks for either**, which is worth stating because it means the
contract's seven assessments would have passed without surfacing them.

**None is fixed by `WO-MOK-012`.** `VER-MOK-005` establishes the standing of an adverse observation in advance — it is
"an artifact decision, not a defect to patch" — and decision 12 of `closing-review.md` directs all three to a separate
chain. Each is recorded here with the measurement that establishes it, the specification provision it bears on, and the
design the owner decided.

Nothing under `mokiterions-tui/src/` changes in this work order. The three remedies below are decided, not built.

---

## Observation 1 — the `?` key is undiscoverable

**Reported by the repository owner, 2026-08-20.** In the owner's words: "the control keys are not easily accessible, the
user needs to press `?` to get access to the control documentation, but `?` is indicated nowhere. That's an important
point."

### What was measured

Confirmed against the tree at `ff3a155`:

- `mokiterions-tui/src/state.rs` binds `Char('?')` to the key-binding overlay. The overlay is the only route to the
  control documentation from inside the observer.
- `mokiterions-tui/src/render.rs` draws no key hint. The header's content is the run state, the speed, the view mode,
  the selection, the filter, and — when panes are excluded — the overlay announcement. There is no hint segment, and
  the footer carries provenance only.
- `SPEC-MOK-003` rule 5 states the header's content as a closed list: "The header reports observer conditions: draw
  failures, input failures, export outcomes, panes available only as overlays". **A key hint is not in it**, so adding
  one requires amending that list rather than only changing the renderer.
- `SPEC-MOK-003` rule 7's key table specifies `?` and twelve other bindings. **It states no discoverability
  obligation.** The gap is in the specification, not only in the implementation.
- The single place `?` is documented is `mokiterions-tui/src/options.rs`, in the `--help` text: "Press ? inside the
  observer for the key bindings." An operator who launches the observer without reading `--help` has no route to it.

### Why the seven assessments did not catch it

Manual assessment 1 asks whether the instrument answers `INT-MOK-004`'s three questions, and it was satisfied — but the
owner knew the bindings before starting. No assessment asks whether a first-time operator can find them, and the
automated suite cannot: every key test presses the key directly.

### Decided remedy — decision 14

**A permanent header segment**, beside the run state, degrading to `?` alone at the 34 × 22 floor where the header has
roughly nine columns to spare. Amends rule 5's closed header content list.

Chosen over a timed banner and over a footer entry: an operator who does not know the key is precisely the one who needs
it, so a hint that disappears is a hint they will miss, and the footer is provenance rather than control.

---

## Observation 2 — the survival gauges are two columns wide

**Reported by the repository owner, 2026-08-20.** In the owner's words: "The status bar are now too narrow (2 columns),
I suggest restore the initial width of the status gauge, and having them over multiple lines for a given Mokiterion."

### What was measured

`assessment-material/bar-quantization.txt`, reproduced:

```
render.rs:572  filled = value * width / 100, integer division.
layout ROSTER_WIDTH is 47, so the pane interior is 45 and
render.rs:546  bar_width(45) = min(20, (45 - 35) / 4) = 2.

bar    values          span
░░     0..=49       50 values
█░    50..=99       50 values
██   100..=100      1 values
```

**A two-cell bar draws three distinct states for 101 attribute values.** Two attributes 49 apart draw the same bar. The
exact level is carried by the three-column numeric value beside it, which is why no information is lost from the pane —
but the fill is not doing the work a bar exists to do, which is to be readable at a glance without reading the number.

### How it got here, and why it is not a regression against an approved figure

This is a **consequence the owner approved without being shown this figure**, and it is already on the record as such.
`SPEC-MOK-003` rule 4 as amended on 2026-08-19 states it explicitly: adding the fourth gauge for computed `fear` raised
the row overhead from 27 to 35 and the divisor from 3 to 4, "so at the reference roster's 45-column interior the bars
narrow from 6 cells to 2". The amendment was approved with that sentence in it.

So the specification is conformed to and the implementation is correct. What the assessment establishes is that the
approved figure is not good enough in use — which is exactly what a manual legibility assessment is for, and why
`VER-MOK-005`'s residual-uncertainty section says buffer assertions "prove correctness, not legibility".

The original width was 6, at three gauges and overhead 27. Two is not a regression against a specified minimum, because
rule 4 specifies no minimum bar width.

### Decided remedy — decision 13

**Two gauges per line over three lines per entry**, giving a bar width of **13**.

From `2 × (w + 6) + 5 + 2 ≤ 45`, `w = 13` — more than double the original 6 and over six times the current 2. Entry
capacity at the roster's 32-row interior becomes `32 / 3 = 10` of 12 entries; the scrolling window in
`render.rs` already follows the selection and the roster title already states how many entries are hidden, so the two
that fall off are announced by machinery that exists rather than silently dropped. The ten-row log is kept.

Smallest change that delivers it: `rows_per_entry` 2 → 3, `BAR_ROW_OVERHEAD` 35 → 19, `bar_width` divisor 4 → 2.

**Alternatives put to the owner and declined:**

| Option | Bar width | Entries visible | Why declined |
|---|---|---|---|
| One gauge per line, five lines per entry | 20 (the `FULL_BAR` cap) | 6 of 12 | Halves the roster for four extra columns of bar |
| Widen the roster to 61 columns | 13 at two per line | 12 of 12 | Takes 14 columns from the map: view interior 67 → 53, overview drops from the whole world to 106 of 128 columns. The owner declined this trade on 2026-08-19 |
| Also shrink the log 10 → 6 rows | 13 | 12 of 12 exactly | Available as a follow-on; costs log depth to gain the two hidden entries. Not chosen now |
| Leave it | 2 | 12 of 12 | Is the finding |

---

## Observation 3 — the hidden-pane notice names the wrong remedy and carries no emphasis

**Reported by the repository owner, 2026-08-20.** In the owner's words: "When the terminal is not wide or tall enough,
some panels are hidden, but that is not indicated. A notive should be permanently written when the terminal should be
enlarged."

### The finding as reported is not what the tree does, and the agent said so

**The premise that exclusion is unindicated does not hold**, and this was reported back to the owner rather than
transcribed as agreement. `SPEC-MOK-003` rule 5 carries an **Announcement** obligation: "Whenever any pane is excluded,
any roster entry is not visible, or the view presents a region, the observer states it: the header lists the panes
currently available only as overlays, the roster title states how many entries are hidden, and the view title states
the visible world range."

`render.rs` implements it and evaluates it on every frame, so the statement is **already permanent** wherever it appears
at all. `assessment-material/announcement-at-reduced-viewports.txt` captures nine viewports and confirms conformance
at each. Extracts:

| Viewport | Panes excluded | Header announcement as drawn |
|---|---|---|
| 160 × 48 | none | *(none — correctly absent)* |
| 139 × 48 | inspector | `overlays: inspector i` |
| 120 × 37 | inspector, log | `overlays: log L  inspector i` |
| 99 × 30 | roster, inspector, log | `overlays: roster r  log L  inspector i` |
| 34 × 22 (floor) | all three | `ovl r L i` |
| 33 × 21 | — | below the floor: nothing drawn, exit 2 |

So a permanent notice already exists and the obligation is already permanent. **Two narrower defects are real**, and
they are what the finding reduces to:

1. **The notice carries no emphasis.** `render.rs:178` builds it as
   `announcement.map(Span::raw).into_iter().collect()` — `Span::raw` applies no style, so the announcement is drawn in
   the header's default styling and right-aligned, where an operator scanning the left-anchored run state can miss it.
2. **It names the overlay remedy and not the enlargement remedy.** `overlays: inspector i` tells an operator how to see
   the pane as an overlay. It does not say the terminal is too small, and it does not say what size would restore the
   pane to the body. The owner's request — a notice saying the terminal should be enlarged — is not satisfied by a
   notice that names a key.

### Decided remedy — decision 15

**Amend rule 5's Announcement obligation** so the notice states the enlargement remedy and the required dimension
alongside the overlay key, and **requires visual emphasis** rather than leaving styling unconstrained — for example
`inspector needs W>=140 — overlay: i`, drawn with a modifier rather than `Span::raw`.

Both remedies are stated rather than one replacing the other: the overlay is the remedy available at the current size,
and the enlargement is the one that restores the layout. The floor form must still degrade to something that fits
roughly nine columns.

The thresholds the notice would quote are already specified in rule 5: roster `W ≥ 100`, inspector `W ≥ 140`, log
`H ≥ 38` at six rows and ten when `W ≥ 140` and `H ≥ 48`.

---

## Two further observations, recorded but not raised as findings

Neither was reported by the owner; both were measured while assembling the material and are recorded so they are not
lost.

- **Log lines truncate mid-value with no marker.** At the reference viewport, log entries are cut at the pane edge in
  the middle of a field: `...satiety:5` and `...en` appear in
  `assessment-material/frame-160x48-tick200.txt`. Nothing indicates truncation, so a reader cannot tell a cut value
  from a complete one. `SPEC-MOK-003` states no obligation about log truncation, so this is a specification gap rather
  than a defect against a rule.
- **`UNDERLINED` never occurs in the seed-42 run.** The modifier marks a rendered cell holding more than one
  Mokiterion and is absent throughout the seed the rest of the assessment used. It is reachable at seed 0, tick 18
  (`assessment-material/underline-search.txt`). A future assessor asked to check it on seed 42 will not find it.
