# The inspector's death line — `SPEC-MOK-003` rule 10 items 6 and 7

Two captures back this file. `inspector.txt` is the implemented presentation; `inspector-one-line.txt`
is the withdrawn one-line form, rendered by the same oracle into the same panes so that the two-line
pairing is justified by measurement. Neither capture asserts anything: both read cells out of a
rendered buffer, border included, so a value the pane could not hold is visible as a missing character
rather than as a shorter string.

## The two viewports, and why they are the two

| Viewport | Inspector pane | Interior | Why this one |
|---|---|---|---|
| `160 × 48` | `44 × 38` | 42 × 36 | `SPEC-MOK-003` rule 5's reference viewport |
| `140 × 22` | `44 × 18` | 42 × 16 | the smallest viewport presenting the inspector |

The second row is the smallest in both axes: the pane arrives at `INSPECTOR_MIN_WIDTH = 140` columns
and the viewport floor is `MIN_HEIGHT = 22` rows, so no smaller viewport in either axis carries the
pane at all. **Its interior width is the same 42.** `INSPECTOR_WIDTH` is a fixed 44 wherever the pane
is present, so the width available to the death line does not vary with the viewport and the two rows
above differ only in height. That is what makes one width measurement cover every viewport that
presents the inspector, and it is why the work order's second stop-and-escalate condition — the death
line failing "at any viewport that presents the inspector" — is decided by one figure rather than by a
sweep.

## What the implemented form presents

Case A is an ordinary run: seed default, `--policy baseline`, advanced until the first death. `M01`,
reported by the engine as `Zug`, died on tick 119 with health 0, satiety 0, energy 81 and fear 90 —
every one of those values read from the engine's own last `survival_changed` record for that subject.

```
┌ inspector ───────────────────────────────┐
│Zug  M01                                  │
│died on tick 119                          │
│final health 0  satiety 0                 │
│energy 81  fear 90                        │
```

Identical at `140 × 22`. Four values, no wrap, `fear` presented unbanded exactly as rule 4 presents it
for the living.

Case B is the no-record case: a death ingested for `M99`, a subject no `survival_changed` record was
ever seen for. No run reaches this state — the engine reports survival before it applies a death — so
it is constructed through the observer's private `ingest`.

```
┌ inspector ───────────────────────────────┐
│M99                                       │
│died on tick 7                            │
│final health 0                            │
```

Three absences and no second line. The heading is the identifier alone, because `M99` has no reported
name and rule 10 as amended 2026-08-19 forbids presenting an identifier as one.

## Why one line does not work, measured

The withdrawn form put all four values on one line. At the reference viewport it renders:

```
┌ inspector ───────────────────────────────┐
│Zug  M01                                  │
│died on tick 119                          │
│final health 0  satiety 0  energy 81  fear│
│90                                        │
```

The line is **45 columns against an interior of 42**. The inspector's paragraph carries
`Wrap { trim: false }`, so the overflow is not truncated — it wraps at the last space that fits. The
result is worse than truncation and is the reason this is a defect rather than a cosmetic complaint:
`fear` ends one row as a label with no value, and `90` begins the next as a value with no label. Rule
10 item 7 forbids presenting a value the engine did not compute; this form presents a computed value
in the shape of the two things that rule exists to prevent.

It is not a two-digit accident. Every attribute is bounded `0..=100`, so three digits is the maximum,
and the widths are:

| Form | Two digits, as captured | Three digits, the maximum | Interior |
|---|---|---|---|
| one line, four values | 45 | 51 | 42 |
| paired, first line | 25 | 29 | 42 |
| paired, second line | 18 | 20 | 42 |

The one-line form overflows at every width the values can take. The paired form has 13 columns of slack
on its worst line. No viewport presenting the inspector is narrower, so the pairing holds everywhere the
pane appears.

## Why the pairing, rather than a wider pane

Rule 4 already pairs these same attributes across two lines for a living Mokiterion, under
`REQ-MOK-047`: health with satiety on the first bar line, energy with fear on the second. The death
line adopts that arrangement rather than inventing one, so a reader who has learned the roster's
grouping reads the death line the same way.

The alternative was to move `INSPECTOR_WIDTH` or `INSPECTOR_MIN_WIDTH`. That is a rule 5 question — pane
thresholds, floors and geometry — and the work order's stop-and-escalate conditions reserve it for the
technical owner, who was not shown it. **The pairing moves no figure in rule 5**: no threshold, no floor,
no pane width, no canvas dimension. It changes the number of lines the inspector's death branch pushes,
and rule 10 imposes no line-count constraint on that branch; the pane has 36 interior rows at the
reference viewport and 16 at the smallest, against four lines used. That is why this was resolved inside
rule 10 rather than escalated.

## The one clause not measured here

Rule 10.6 as amended requires that a pair carrying neither of its values emit **no line at all**, so
that a bare line does not read as a field whose value was withheld. Case B satisfies it — but no frame
can show that it does. The death branch returns with that line last, so a line the code declined to emit
occupies the same cells as the pane's own unwritten rows. The clause is carried by the shape of the code
and is disclosed as a residual in `VER-MOK-005`. A regression that emitted the line blank would be
invisible to every case in that contract.

**Counting the lines instead of reading the cells does not reach it either, and the reason is module
privacy rather than effort.** The assertion would need `render::inspector_lines`, which is private to
`render`, and the no-record state, which is constructible only through `Observer::ingest`, private to
`state`. Those are sibling modules, so no test module is a descendant of both: a test in `src/state.rs`
cannot name `inspector_lines`, and a test in `src/render.rs` cannot build the state — none of the four
`#[cfg(test)]` hooks injects a death, and `ARCH-MOK-002` forbids adding a fifth. Closing this residual
therefore costs a prohibited pattern, which is why it is disclosed rather than closed.
