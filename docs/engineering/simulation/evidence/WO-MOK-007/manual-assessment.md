# Manual assessment

`VER-MOK-007` names three manual assessments and requires each to state its environment, and requires that an
assessment which cannot be performed from the implementation environment says so and says why **rather than being
marked passed**.

**All three are OUTSTANDING. None has an author. None is claimed as passed.**

Environment of this implementation: a non-interactive shell on Windows 11, no terminal attached, no display. Every
automated case in this work order asserts against an in-memory character buffer and a style table through
`ratatui`'s `TestBackend`. Nothing here has put a pixel on a screen, and no automated case in either tier is offered
as evidence that a colour arrived at a terminal.

| # | Assessment | Status | Author | Why it is outstanding |
|---|---|---|---|---|
| 1 | The three bands are distinguishable on the owner's terminal. | **OUTSTANDING** | none | This is the one assessment that matters for the feature's purpose, and it requires a human eye on the owner's own terminal with the owner's own colour scheme. It cannot be reached from here at all. |
| 2 | A selected entry in the low band remains readable, where reversed video puts the band colour in the background. | **OUTSTANDING** | none | The composition is proved mechanically — `selected-entry-composes` shows `REVERSED` present and the band foreground unchanged, and `frames.txt` captures a red gauge under reversal — but whether red-on-inverted is *legible* is a judgement about a rendered screen. |
| 3 | The middle band reads as orange rather than as yellow or brown on the terminals the owner uses. | **OUTSTANDING** | none | `Color::Indexed(208)` is xterm palette entry 208. What a terminal actually paints for entry 208, and what it degrades to without 256-colour support, is a property of that terminal. If it reads wrong, the palette is the implementation's to change under `SPEC-MOK-003`'s grant and no amendment is needed. |

## What can be said from here, and is not the same thing

- `frames.txt` carries the reference-viewport roster with a band map under every bar row, unselected and with `M05`
  selected. It shows twelve entries, three bands present in one pane, three bands present in one entry, and
  `REVERSED` on the selected entry with its band foregrounds intact. That is a claim about a buffer.
- `VER-MOK-007`'s **Text identity** property, asserted over 2121 cases, means a monochrome terminal loses nothing
  the roster asserted before clause 7 existed. That is a claim about the encoding, not a claim that a monochrome
  terminal was tested.

## The standing backlog this adds to

`WO-MOK-005`'s `manual-assessment.md` records seven manual assessments as **OUTSTANDING** with no author. These
three are additional to those seven, not a replacement for them, and this work order does not discharge any of them.
