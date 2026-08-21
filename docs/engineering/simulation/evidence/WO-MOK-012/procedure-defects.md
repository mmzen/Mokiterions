# Three defects in WO-MOK-005's recorded assessment procedure

`evidence/WO-MOK-005/manual-assessment.md` records, for each of the seven outstanding assessments, "the material an
assessor needs and the procedure to follow". Three defects in that procedure were found on 2026-08-20 while performing
the assessments. Each would have obstructed or misdirected an assessor following it literally.

**The original file is not edited.** It is retained evidence of a work order that reached `implemented`, and it stated
correctly at the time that all seven assessments were outstanding. The precedent followed is `evidence/WO-MOK-011/merge/`,
which re-measured superseded figures into a new directory rather than editing the ones it superseded. These defects are
therefore recorded here, keyed to `WO-MOK-012`, and `manual-assessment.md` in this directory carries the corrected
procedure.

All three were verified against `master` at `ff3a155`.

---

## Defect 1 — the authority overlay key is wrong

`evidence/WO-MOK-005/manual-assessment.md`, assessment 1, states:

> Use `Tab` to select, `z` to zoom, `f` to follow, **`a` for the authority overlay**.

**`a` is not bound to anything.** The authority overlay is `t`.

Verified in `mokiterions-tui/src/state.rs`, where the key dispatch binds:

```
518:            KeyCode::Char('t') => self.overlay = Overlay::Authority,
519:            KeyCode::Char('r') => self.overlay = Overlay::Roster,
521:            KeyCode::Char('i') => self.overlay = Overlay::Inspector,
522:            KeyCode::Char('?') => self.overlay = Overlay::Help,
```

No arm matches `Char('a')`. The specification agrees with the code and not with the procedure: `SPEC-MOK-003` rule 7's
key table gives `t` as "open the authority overlay for the highlighted event type".

**Consequence.** An assessor following the procedure presses a dead key and cannot answer the third of the three
`INT-MOK-004` questions — which requirement authorizes a highlighted event — and would reasonably report the authority
overlay as missing. The defect is in the recorded procedure alone; the implementation and the specification are correct
and agree.

This defect was in the agent's own earlier framing of the assessment checklist, taken from the recorded procedure. It
was corrected by reading `state.rs` and rule 7 before the question was put to the owner, so the owner was shown `t`.

---

## Defect 2 — the command cannot reach tick 200

`evidence/WO-MOK-005/manual-assessment.md`, assessment 1, states:

> Procedure: `cargo run -p mokiterions-tui -- --seed 42 --policy reference --speed 8` in an interactive terminal at
> least 160 × 48, and let it reach tick 200 (25 seconds at speed 8).

**The command omits `--ticks`, and the default is 100.** The run terminates at tick 100 and the assessment, which
`VER-MOK-005` requires be performed over "at least two hundred ticks", cannot be completed.

Verified in `mokiterions-core/src/cli.rs`:

```
137:            tick_limit: ticks.unwrap_or(100),
```

and `mokiterions-tui/src/options.rs`, whose `USAGE` states that "`--seed`, `--ticks`, `--policy` and `--density` carry
exactly the meaning, the defaults and the validation the Mokiterions binary gives them" — so the observer inherits the
same default of 100.

**Consequence.** The assessor watches the run stop at tick 100 with `--speed 8` after about 12 seconds rather than the
25 the procedure predicts, and either reports a premature termination or records the assessment as performed over half
the required horizon.

**Corrected procedure:** add `--ticks 300`. That is what the captures in `assessment-material/` were taken with, so the
retained material and the corrected procedure agree.

The procedure's own warning about `--policy baseline` is correct and worth keeping: seed 42 under baseline reaches
extinction on tick 142, so that configuration cannot reach 200 ticks whatever `--ticks` says. Use `--policy reference`.

---

## Defect 3 — the assessment asks for something the binary cannot do

`evidence/WO-MOK-005/manual-assessment.md`, assessment 3, is headed "The same with colour disabled" and asks whether any
distinction is lost on a monochrome terminal. `VER-MOK-005` as approved asked the same: "Confirm the same with colour
disabled or on a monochrome terminal".

**There is no way to disable colour.** The observer has no such flag and honours no such environment variable.

Verified across `mokiterions-tui/src/` and `mokiterions-core/src/`: no occurrence of `NO_COLOR`, `--no-color` or any
`env::var` read. The observer's complete flag set, from `options.rs`'s `USAGE`, is `--seed`, `--ticks`, `--policy`,
`--density`, `--speed`, `--start-paused`, `--export` and `--help`. The only matches for "monochrome" in the workspace are
in `mokiterions-tui/src/verification.rs`, where `monochrome()` is a **test helper** that projects a rendered buffer to
`(symbol, modifier)` pairs — a colourless view of a frame for assertion purposes, not a runtime mode.

**Consequence.** An assessor can satisfy this only by using a terminal emulator that itself suppresses colour, which is
a property of their environment rather than of the observer, and which `VER-MOK-005` does not name. Strictly, the
assessment as written was unperformable on the shipped binary from the day it was approved.

Unlike defects 1 and 2, **this is not a defect in the procedure**: the procedure faithfully restates the contract, and it
had already identified the part a person must actually answer — "whether `UNDERLINED` and `REVERSED` actually render
distinguishably on the assessor's terminal". The defect is in `VER-MOK-005`. It was reported to the owner as such and
`VER-MOK-005` was amended on 2026-08-20 to state the modifier-legibility check instead, which is what assessment 3 in
`manual-assessment.md` records as performed.

**Whether the observer should gain a colour-disable flag is not decided here.** No approved requirement asks for one.
It is noted as a candidate for the second chain and nothing in this work order presumes the answer.

---

## Summary

| # | Where | Defect | Corrected how |
|---|---|---|---|
| 1 | `WO-MOK-005` procedure, assessment 1 | `a` named as the authority overlay key; `a` is unbound and the key is `t` | Corrected procedure in this directory's `manual-assessment.md` |
| 2 | `WO-MOK-005` procedure, assessment 1 | Command omits `--ticks`; the default of 100 cannot reach the required 200 | Corrected procedure adds `--ticks 300` |
| 3 | `VER-MOK-005` assessment 3 | Asks for a colour-disabled run the binary cannot perform | `VER-MOK-005` amended 2026-08-20 to the modifier-legibility check |

Defects 1 and 2 are in retained evidence and are corrected by this directory superseding that procedure. Defect 3 was in
an approved assurance artifact and is corrected by amendment.
