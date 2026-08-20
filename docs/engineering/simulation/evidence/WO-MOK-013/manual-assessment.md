# Manual assessment record — WO-MOK-013

`VER-MOK-013` contracts **two** manual assessments and states that they "are the primary evidence for
their requirements rather than a confirmation of the automated cases". Both are recorded here.

**Both are OUTSTANDING and neither has an author.** An assessment with no author is outstanding —
`VER-MOK-013`'s *Evidence retention* says so in the same sentence that requires an author, a date, a
role and a terminal, and `VREC-MOK-005` disclosing seven such is what this chain descends from. The
implementation agent cannot take either: one requires a person at a real terminal judging legibility,
and the other requires a person who has not read `SPEC-MOK-003` rule 7. Recording a status against
either would put a signature where no person looked, which is the failure mode
`evidence/WO-MOK-012/manual-assessment.md` assessment 7 refused and stated its reason for.

The material and the procedure are prepared below so that each can be taken in one sitting. The
procedure is the one `evidence/WO-MOK-012/procedure-defects.md` corrected — its three defects are not
reintroduced here.

## 1. Reference-viewport gauge legibility

**Status: OUTSTANDING. Author: none. Date: —. Role: product owner. Terminal: —.**

`VER-MOK-013`: at `160 × 48` on a real terminal, over a run of at least 200 ticks on a declared seed,
confirm that a Mokiterion's decline is visible as a change in the bar and not only in the number.

**This is assessment 2 of `VER-MOK-005` re-taken, and its earlier outcome is inherited in neither
direction.** On 2026-08-20 it returned "SATISFIED as to the subject `VER-MOK-005` enumerates. ADVERSE
OBSERVATION on the roster gauges", which is the observation this chain exists to answer. A pass here
does not follow from the arithmetic below and a fail here is not contradicted by it.

**Procedure.** In an interactive terminal at least 160 × 48:

```
cargo run -p mokiterions-tui -- --seed 42 --ticks 300 --policy reference --speed 8
```

`--ticks` is required: the default is 100 and the assessment needs 200. `--policy reference` is
required: seed 42 under baseline reaches extinction at tick 142. Watch one Mokiterion's health or
satiety gauge fall and judge whether the bar moved, without reading the number beside it.

**Material.** `frames.txt` in this directory carries the reference viewport at tick 200 with all
twelve entries; `gauge-resolution.md` and `gauge-resolution.txt` carry what the gauge can and cannot
resolve. The before form is `evidence/WO-MOK-012/assessment-material/bar-quantization.txt`.

**What the material does not settle.** The gauge now has 14 renderable states where it had 3, and a
ten-point step moves the fill at every value. Neither figure is this assessment: 14 states over 101
values still means two values seven apart can draw the same bar, and whether that reads as movement
to a person watching is what the assessment asks. `VER-MOK-013`'s residual uncertainty records that a
person's terminal is not the buffer.

## 2. Overlay discoverability

**Status: OUTSTANDING. Author: none. Date: —. Role: product owner. Terminal: —.**

`VER-MOK-013`: show one frame to a person who has not read `SPEC-MOK-003` rule 7 and ask how they
would find out what the keys do. A pass is that they name the key from the screen.

**Procedure.** Draw one frame — the run may be held — and show it without saying anything about keys.
Ask the question as written. If the assessor finds the hint but cannot tell what it opens, that is an
adverse observation on the wording and belongs to `SPEC-MOK-003` rule 5, not to the implementation.

**Material.** `frames.txt` carries the header line at all nine renderable viewports. At `120 × 48`
the header reads `overlays: inspector i at width 140` beside the hint; at the floor `34 × 22` the
whole notice reduces to `HELD  ?  x8  r W100  L H38  i W140`, which is the last rung of the ladder
and the hardest case for this assessment.

**The assessor cannot be the repository owner, and this is not the agent's to resolve.** The contract
asks for a person who has not read rule 7. The owner has read it and ratified six amendments to
`SPEC-MOK-003` on 2026-08-20, two of which — decisions 15 and 16 — fix the hint's content and the
abbreviation ladder's order of loss, so the owner knows the key from the specification rather than
from the screen. `VER-MOK-013`'s own residual uncertainty says the assessment "is available once per
assessor" and that a second run on the same person confirms nothing; here the first run is already
spent. **Finding an admissible assessor is outside what an implementation agent can arrange**, so the
options are the owner's:

- find a person who has not read rule 7 and take the assessment as contracted;
- amend `VER-MOK-013` to state an assessment the available assessors can take, which is an
  assurance-owner act on an approved contract;
- leave it outstanding by decision with the reason recorded, on the pattern of assessment 7 of
  `evidence/WO-MOK-012/manual-assessment.md`.

The third is the only one that requires nothing further, and none of the three is taken here.

**The automated case is not this assessment.** `VER-MOK-013` says so: that the character is in the
buffer of the first frame is "a necessary condition and not the property". That condition is measured
and passing —
`tests/render.rs::the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport` asserts it
at every viewport with no input delivered and at tick zero — and it does not close this row.

## Summary

| # | Assessment | Status | Author | Blocking |
|---|---|---|---|---|
| 1 | Reference-viewport gauge legibility | **Outstanding** | none | a person at a terminal at `160 × 48` |
| 2 | Overlay discoverability | **Outstanding** | none | a person who has not read `SPEC-MOK-003` rule 7 |

**Consequence for the chain.** `VER-MOK-013` classes both as primary evidence for their requirements,
so `VREC-MOK-013` cannot record either requirement as verified while they stand outstanding, and any
release record covering this chain inherits the disclosure. This does not block the work order's
implementation, which is what this pack evidences; it blocks verification, which is a separate
commit-bound record and a separate accountable act.
