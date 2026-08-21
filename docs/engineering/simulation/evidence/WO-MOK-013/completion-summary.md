# Completion summary — WO-MOK-013

`WO-MOK-013`'s *Completion report format* specifies eight items. This document is those eight, in that order.

**All nine in-scope items are implemented, and the status is now `implemented`.** The transition was taken on
2026-08-20 by the repository owner as engineering owner, after this report was written and after the three
post-implementation decisions of item 7 — the implementation agent held the work order at `approved` throughout, because
`implemented` asserts the completed change **and** the retained evidence as an accountable judgement. `WORKFLOW.md`
makes a status change a record of authority rather than a confidence estimate, so `implemented` does not assert that
`VER-MOK-013` is satisfied; it is not, on one count. `VREC-MOK-013` was captured in the same act and is a **`ready`
candidate, not verified**. The work order's *Transition to `implemented`* subsection carries both statements.

> **Later fact, 2026-08-20.** `VREC-MOK-013` is now **`verified`**, on the assurance owner's decision 23, which also
> verified `REQ-MOK-048` on its automated cases in place of the assessment `VER-MOK-013` contracts for. **`WO-MOK-013`
> stays `implemented`** and **`VER-MOK-013` is still not satisfied, on that same one count** — the sentence above about
> the contract holds unchanged, and it is the sentence that matters most in this paragraph. `assurance-decision.md`.

## 1. Decision 1 as taken, with both corrections to how the option was put

**Option B — the log is held at six rows, `REQ-MOK-020` is untouched, and all twelve entries stay visible at the
reference viewport. Product owner, 2026-08-20.** The instruction, verbatim and complete:

> OK for option B

It named one act: the choice between the two options the work order sets out. It approved no artifact, ratified no
amendment, and did not answer the `WO-MOK-012` identifier collision, which was put in the same turn.

**Correction 1 — the cost figure the option was put on was wrong.** Option B was framed as showing "six recent events
rather than ten". Both figures counted **pane rows and not event lines**. A log pane carries its height less the two
rows its border takes, so the true change is **eight visible records to four**. `log-height.md` measures both forms out
of rendered buffers: eight lines in the ten-row pane of `evidence/WO-MOK-005/frames.txt`, four in the six-row pane of
`frames.txt` here. The error was the implementation agent's, in the framing and not in the owner's answer.

**Correction 2 — a fact that was not established when decision 1 was taken.** The framing did not state that six is
the *maximum* log height at which twelve entries fit. The roster interior is `42 − log` rows and twelve three-line
entries need 36, so:

| Log rows at `160 × 48` | Roster interior | Entries of 3 lines |
|---:|---:|---:|
| 4 | 38 | 12 |
| 5 | 37 | 12 |
| **6** | **36** | **12** |
| 7 | 35 | 11 |
| 10 | 32 | 10 |

There is **no intermediate compromise** between option A and option B. That is what makes option B the only
arrangement satisfying `REQ-MOK-047` and `REQ-MOK-020` together at a 47-column roster.

**Decision 2 — the product owner stood by option B on the corrected figures, 2026-08-20.** Both corrections were put
explicitly as grounds for re-opening, together with a third route — re-opening the 89-column roster widening declined
on 2026-08-19, which satisfies both requirements *and* keeps the ten-row log — whose measured price is a 25-cell view
interior addressing 50 of 128 world columns, so the reference viewport would stop presenting the whole world, which
`REQ-MOK-019` obliges. The owner declined it.

**What the trade costs, stated as a trade.** The reference viewport now shows **four** of the authoritative event
stream's records where it showed eight. The stream itself is unaffected: `non-perturbation.txt` measures 7,534 records
retained and exported, identical in order to the unobserved run. `SPEC-MOK-003` rule 5's own reason for admitting a log
— that it "carries the authoritative event stream" — is retained in place rather than deleted, because it is what made
ten rows right for a phase.

**Two consequences, neither discovered later.** Entries still hide below the reference viewport — at `160 × 44` the
interior is 32 and the roster draws 10 of 12 and titles itself `hidden 2` — which is rule 5's specified degradation and
not a regression; and the reference fit has **no slack in either direction**, since `12 × 3 = 36` fills a 36-row
interior exactly. `SPEC-MOK-001` bounds the population at twelve and calls a table longer or shorter than the
population a defect, so the fit is safe today and a later phase raising the population breaks it by hiding entries.

## 2. The three requirements, each with its measurement and the before figure

### `REQ-MOK-047` — the gauge resolves

> WHERE the terminal observer presents a survival attribute as a proportional gauge at the reference viewport size,
> THE SYSTEM SHALL render that gauge at a width at which any change of ten in the value it presents changes the number
> of filled cells.

| | Before, at `ff3a155` | Now | Measured in |
|---|---:|---:|---|
| Gauges per line / entry height | 4 on 1 line / 2 rows | **2 on each of 2 lines / 3 rows** | `frames.txt`, `gauge-resolution.txt` |
| Row overhead | `5 + 4×6 + 3×2 = 35` | **`5 + 2×6 + 1×2 = 19`** | `SPEC-MOK-003` rule 4, re-derived |
| Bar width at the 45-column interior | 2 cells | **13 cells** | `gauge-resolution.txt`, counted from cells |
| Renderable states over 101 values | 3 | **14** | `gauge-resolution.md` |
| Widest span drawing one state | 50 values | **8 values** | `gauge-resolution.md` |
| Ten-point steps that move the fill | **11 of 91** | **91 of 91** | `gauge-resolution.txt`, every value in `0..=100` |

The requirement is met at every gauge width the implementation can produce, not only at the reference one: the sweep
over `34..=200 × 22..=60` finds **one** distinct roster pane width, 47 columns, which is what establishes rule 5's fixed
width rather than taking it on trust. Ten is the minimum width satisfying the property; 13 is what fits. The before
figure of 11 is arithmetic on the measured three-state table in
`evidence/WO-MOK-012/assessment-material/bar-quantization.txt`: only `v ∈ 40..=49` and `v = 90` crossed a boundary.

Automated: `src/render.rs::a_ten_point_step_moves_the_fill_at_the_reference_viewport` and
`::every_bar_width_the_roster_can_produce_resolves_a_ten_point_step`. **Manual assessment 1 is outstanding** — see
item 7.

### `REQ-MOK-048` — the controls are findable

| | Before, at `ff3a155` | Now | Measured in |
|---|---|---|---|
| On screen at the reference viewport | **nothing** | `? keys`, beside the run state | `frames.txt` line 14 |
| At the floor `34 × 22` | **nothing** | `?` alone | `frames.txt` line 389 |
| Reachable without operator action | no — rule 7's table opened only by pressing `?` | **yes, from the first frame** | `tests/render.rs` |

Before, the observer's only documentation of its controls was reachable only through one of them, so an operator who
did not already know that key had no way on screen to learn any of the rest. That is the adverse observation this
answers.

Automated: `the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport` (every viewport, no input
delivered, tick zero), `the_hint_is_present_after_two_hundred_ticks_in_both_run_states`,
`the_hint_displaces_neither_the_announcement_nor_the_footer`. **Manual assessment 2 is outstanding, and no admissible
assessor is available** — see item 7.

### `REQ-MOK-049` — the notice is actionable

| Viewport | Before, at `ff3a155` | Now |
|---|---|---|
| `120 × 48` | `overlays: inspector i` | `overlays: inspector i at width 140` |
| `120 × 37` | `overlays: log L  inspector i` | `overlays: log L at height 38  inspector i at width 140` |
| `34 × 22`, the floor | `ovl r L i` | `HELD  ?  x8  r W100  L H38  i W140` |

The before forms are `evidence/WO-MOK-012/assessment-material/announcement-at-reduced-viewports.txt`; the after forms
are `frames.txt`, and the floor line was verified byte-exactly. **The notice named the pane and the key; it did not
name the axis or the value.** An operator was told a pane was missing and given no way to learn what would bring it
back except by resizing until it did. The threshold value is read from `SPEC-MOK-003` rule 5's own thresholds and is
not restated in the presentation layer, so a threshold changed there cannot leave a notice quoting the old one.

The axis and the threshold survive last in the abbreviation ladder, so the remedy that persists at the narrowest
viewports is enlarging the terminal — the one remedy that needs no key press.

Automated: `the_announcement_states_the_axis_and_the_value_the_layout_decides_presence_from`,
`the_announcement_is_emphasised_and_the_optional_segments_are_not`,
`the_announcement_appears_and_disappears_with_the_pane_it_names`,
`the_announcement_and_the_hint_read_nothing_but_the_viewport`, and
`tests/verification.rs::the_announcement_and_the_hint_survive_the_loss_of_colour`.

## 3. Every amendment, with the ratifying act and its date

**No amendment row in this chain reads `OUTSTANDING`.** That was swept rather than assumed: every approval cell in
`SPEC-MOK-001` through `SPEC-MOK-004`, `ARCH-MOK-001`, `ARCH-MOK-002`, `VER-MOK-005` and `VER-MOK-013` was read, and
each mention of the word is a historical note on a row since ratified — `"It was OUTSTANDING from … until that act"` —
not a live state. This work order was authored so that this would be true: it says so itself, and `WO-MOK-005` records
of five earlier provisions that it "is not verifiable until they are given".

| Artifact | Amendment | Ratifying act | Date |
|---|---|---|---|
| `SPEC-MOK-003` | Coverage of `REQ-MOK-047`–`REQ-MOK-049` declared; **no rule changed** | Repository owner as **technical owner**, decision 8, in the same turn as the requirement approvals and in response to the validator's `E007` | 2026-08-20 |
| `SPEC-MOK-003` | **Amendment 1** — rule 4's bar row from four gauges on one line to two on each of two; overhead `35 → 19`; bar `2 → 13` | Repository owner as **technical owner**, **decision 12**, put as its own question and ahead of the work order's approval | 2026-08-20 |
| `SPEC-MOK-003` | **Amendment 2** — rule 4 item 1's arithmetic on the three-line entry, and the reference viewport's 36 interior rows stated as a provision | Repository owner as **technical owner**, **decision 13**, put as its own question | 2026-08-20 |
| `SPEC-MOK-003` | **Amendment 3** — clauses 5 and 7 carried onto the two bar lines unchanged; the gauge order fixed | Repository owner as **technical owner**, **decision 14**; the order `health`/`satiety` then `energy`/`fear` is the owner's, not the agent's | 2026-08-20 |
| `SPEC-MOK-003` | **Amendment 4** — the header admits exactly one permanent affordance, the overlay key | Repository owner as **technical owner**, **decision 15**, answered as a **distinct affordance** rather than a sixth item of the closed list | 2026-08-20 |
| `SPEC-MOK-003` | **Amendment 5** — rule 5's Announcement gains the axis, the threshold, emphasis and a fixed order of loss | Repository owner as **technical owner**, **decision 16**; that the threshold survives last is the owner's choice over keeping the key last | 2026-08-20 |
| `SPEC-MOK-003` | **Amendment 6** — the log is `6` rows wherever present; the growth to `10` withdrawn, in six enumerated locations and a seventh found while applying it | Repository owner as **technical owner**, **decision 17**, on the substance of the product owner's decisions 1 and 2 | 2026-08-20 |
| `SPEC-MOK-004` | Rules 9, 10 and 11's recorded figures corrected; rule 6 re-measured and unchanged | Covered by the **approval of `WO-MOK-013`** by the repository owner as technical owner, whose in-scope item 5 is these corrections. **Every figure is a measured outcome, not a decision.** | 2026-08-20 |
| `VER-MOK-005` | The unsatisfiable `REQ-MOK-020` fill row **withdrawn** | Repository owner as **assurance owner**, **decision 4**, choosing withdrawal over amending the row or recording the defect | 2026-08-20 |
| `VER-MOK-005` | Two further stale `REQ-MOK-020` rows amended to the true form | Repository owner as **assurance owner**, **decision 10**, choosing to amend both over withdrawing them | 2026-08-20 |
| `VER-MOK-005` | Every figure and clause depending on the ten-row log corrected — four matrix rows, one invariant clause, plus acceptance scenarios 4 and 5 | Covered by the **approval of `WO-MOK-013`** as **assurance owner**, whose in-scope item 7 enumerates these locations and states the act is taken once the implementation lands | 2026-08-20 |
| `VER-MOK-013` | One figure in *Independence* corrected — `VER-MOK-005`'s case count | **Corrected by the implementation agent** as an arithmetic error in its own text. No obligation created, removed or reworded; no owner decision restated. | 2026-08-20 |

Three of the twelve are the implementation agent's own writing of a measured fact rather than an owner decision, and
each says so in its own approval cell: `SPEC-MOK-004`'s figure corrections, `VER-MOK-005`'s ten-row-log sweep, and
`VER-MOK-013`'s arithmetic correction. The other nine are owner acts and are attributed to the role acted in.

## 4. The complete change surface

Against `origin/master` at **`ff3a155`**, which `engine-untouched.txt` establishes is an ancestor of this branch's
HEAD, so the two baselines are one commit and not two:

```
 docs/ROADMAP.md                                    |  91 +-
 .../simulation/architecture/ARCH-MOK-001.md        |   4 +-
 .../simulation/evidence/WO-MOK-012/  (22 files)    | 3031 +
 .../simulation/requirements/REQ-MOK-047.md         | 129 +
 .../simulation/requirements/REQ-MOK-048.md         | 108 +
 .../simulation/requirements/REQ-MOK-049.md         | 116 +
 .../simulation/specifications/SPEC-MOK-002.md      |   8 +-
 .../simulation/specifications/SPEC-MOK-003.md      | 232 +-
 .../simulation/specifications/SPEC-MOK-004.md      |  97 +-
 .../simulation/verification/VER-MOK-005.md         |  47 +-
 .../simulation/verification/VER-MOK-013.md         | 224 +
 .../simulation/work-orders/WO-MOK-012.md           | 382 +
 .../simulation/work-orders/WO-MOK-013.md           | 881 +
 mokiterions-tui/src/layout.rs                      |  50 +-
 mokiterions-tui/src/render.rs                      | 487 ++-
 mokiterions-tui/src/verification.rs                |  14 +-
 mokiterions-tui/tests/layout.rs                    |  79 +-
 mokiterions-tui/tests/render.rs                    | 930 ++-
 mokiterions-tui/tests/verification.rs              | 130 +-
 40 files changed, 6759 insertions(+), 281 deletions(-)
```

That is the **whole branch**, which is what the format asks for, and it includes `WO-MOK-012`'s governance work. **The
implementation's own surface** is the diff against `a339902`, the commit this work order started from:

```
 docs/ROADMAP.md                                    |  30 +
 .../simulation/specifications/SPEC-MOK-003.md      | 218 ++-
 .../simulation/specifications/SPEC-MOK-004.md      |  93 ++-
 .../simulation/verification/VER-MOK-005.md         |  18 +-
 .../simulation/work-orders/WO-MOK-013.md           |  32 +-
 mokiterions-tui/src/layout.rs                      |  50 +-
 mokiterions-tui/src/render.rs                      | 487 ++-
 mokiterions-tui/src/verification.rs                |  14 +-
 mokiterions-tui/tests/layout.rs                    |  79 +-
 mokiterions-tui/tests/render.rs                    | 930 ++-
 mokiterions-tui/tests/verification.rs              | 130 +-
 11 files changed, 1821 insertions(+), 260 deletions(-)
```

Plus this directory, `evidence/WO-MOK-013/`, twenty files untracked at the time of writing and therefore absent from
both figures. **Nothing under `mokiterions-core/` appears in either**: `git diff --stat origin/master --
mokiterions-core/` is empty and `git diff --numstat` over it yields zero lines, which is what the work order's *Out of
scope* requires and what `engine-untouched.txt` records.

The five source files are the whole executable surface: `layout.rs` and `render.rs` carry the change, `verification.rs`
carries the colourless projection's new case, and the three test files carry the fourteen arrivals.

## 5. The test total, reconciled arrival by arrival, and the interface re-count

`cargo test --workspace`: **226 passing, 0 failing, 0 ignored**, retained whole as `test-output.txt`.

| | At `ff3a155` | Now | `SPEC-MOK-004` rule 11 records |
|---|---:|---:|---|
| Engine | 85 | **85** | 85, "unchanged" |
| Observer | 127 | **141** | 141 |
| Workspace | 212 | **226** | 226 |
| Observer internal tier, rule 10 | 39 | **41** | 41 |
| Observer public tier, rule 9 | 88 | **100** | 100 |

`41 + 100 = 141` and `141 + 85 = 226`, which is rule 11's own cross-check. The census is taken **by test name at both
commits**, because a count cannot distinguish a rename from a removal plus an addition and this work order contains
exactly one rename: fifteen names arrive, one departs, and the net is rule 11's fourteen. The departed name is the
rename's old name. The method is validated rather than trusted — `test-census.md` shows the static per-file counts
equal `test-output.txt`'s executed counts for **every one of twenty targets**, workspace 226/226.

The fourteen, with what each serves:

| File | Test | Serves |
|---|---|---|
| `src/render.rs` | `a_ten_point_step_moves_the_fill_at_the_reference_viewport` | `REQ-MOK-047`, the whole value range |
| `src/render.rs` | `every_bar_width_the_roster_can_produce_resolves_a_ten_point_step` | `REQ-MOK-047`, as a plane property |
| `tests/layout.rs` | `the_reference_roster_interior_holds_the_whole_population` | the interior the fit assumes |
| `tests/render.rs` | `every_living_mokiterion_has_an_entry_at_the_reference_viewport` | `REQ-MOK-020` preserved, not traded |
| `tests/render.rs` | `a_declining_mokiterion_shows_a_declining_bar` | acceptance scenario 1 |
| `tests/render.rs` | `the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport` | `REQ-MOK-048`, cold start, every viewport |
| `tests/render.rs` | `the_hint_is_present_after_two_hundred_ticks_in_both_run_states` | `REQ-MOK-048`, not timed |
| `tests/render.rs` | `the_hint_displaces_neither_the_announcement_nor_the_footer` | `REQ-MOK-048`, displaces no obligation |
| `tests/render.rs` | `the_announcement_states_the_axis_and_the_value_the_layout_decides_presence_from` | `REQ-MOK-049`, axis, value, key, floor |
| `tests/render.rs` | `the_announcement_is_emphasised_and_the_optional_segments_are_not` | `REQ-MOK-049`, emphasis |
| `tests/render.rs` | `the_announcement_and_the_hint_read_nothing_but_the_viewport` | the layout-purity property |
| `tests/render.rs` | `the_announcement_appears_and_disappears_with_the_pane_it_names` | acceptance scenario 5 |
| `tests/render.rs` | `no_entry_is_lost_silently_at_any_viewport_presenting_the_roster` | the no-entry-lost-silently invariant |
| `tests/verification.rs` | `the_announcement_and_the_hint_survive_the_loss_of_colour` | `REQ-MOK-049`, legible without colour |

**The one rename**, reported as a departure from what the work order foretold:
`tests/layout.rs::the_log_is_ten_rows_only_where_both_thresholds_are_met` became
`::the_log_is_six_rows_wherever_it_is_present`. Decision 1 removed its subject and the work order expected its removal
to be reported here. It was **kept and strengthened** instead: the two viewports that carried ten rows are the two that
now assert six, so the withdrawn growth is asserted absent rather than left untested, which `SPEC-MOK-004` rule 12
admits as a rename with its assertion strengthened. That is why `tests/layout.rs` reads 11 and not 12. **No test was
lost.**

**Interface re-count, `SPEC-MOK-004` rule 6.** Stop condition 3 fires on any figure in it moving. It did not fire:

| | Recorded | Measured |
|---|---:|---:|
| Public items | 94 | **94** |
| Public fields | 24 | **24** |
| `pub` lines | 118 | **118** |

Re-counted module by module and reproducing rule 6's per-module table row by row — `authority` 5, `export` 3, `layout`
10, `options` 8, `render` 2, `spatial` 19, `state` 47. `BAR_ROW_OVERHEAD` is a private constant, the two hint constants
are private, `bar_width` is a private function, and the entry's row count is a local, so this work order adds no
member of the interface and changes no member's shape. Retained as `interface.txt`.

**Static checks**, retained as `static-checks.txt`: `cargo fmt --all -- --check` `exit=0`, and
`cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` `exit=0`.

## 6. The validator, the inspector's warning count, and the W-HEX-003 attribution

Run from the **pinned 0.4.0 harness venv**, retained whole as `validation.txt`.

```
Engineering artifact validation: PASS
Artifacts: 108 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
```

Inspector: `108 artifacts | 372 relations | 23 findings`, severity `error 0 | warning 11 | info 12`. The eleven are the
inspector's **derived** findings, which the validator does not raise: `W-HEX-001 × 3` on `WO-MOK-010`, `WO-MOK-011` and
`WO-MOK-012`, and `W-HEX-003 × 8`. The twelve informational are `I-REV-001`, the verification and release records' own
review cadence, unmoved.

**No new W-HEX-003, and none new of any kind.** This was measured across three trees rather than asserted of one:

| Tree | Warnings | W-HEX-001 | W-HEX-003 |
|---|---:|---:|---:|
| `ff3a155`, the baseline | 7 | 2 | 5 |
| `a339902`, before implementing | 11 | 3 | 8 |
| Working tree, after implementing | **11** | **3** | **8** |

The set is identical at `a339902` and now, observation for observation. **The implementation bumps no artifact's
`updated` field**: `SPEC-MOK-003`, `SPEC-MOK-004`, `VER-MOK-005` and `WO-MOK-013` all already read `2026-08-20` at
`a339902`, so no new date skew can arise from editing them.

The three-warning rise from `ff3a155` to `a339902` belongs to this chain's **governance** work and to `WO-MOK-012`,
both of which landed before implementation began. Five observations arrive and two depart:

| | Relation |
|---|---|
| arrives | `ADR-MOK-002 → ARCH-MOK-001` (`decides`) |
| arrives | `ADR-MOK-003 → ARCH-MOK-001` (`decides`) |
| arrives | `REL-MOK-001 → VER-MOK-005` (`gates`) |
| arrives | `WO-MOK-008 → SPEC-MOK-003` (`specifications`) |
| arrives | `WO-MOK-008 → VER-MOK-005` (`verification`) |
| departs | `ARCH-MOK-001 → SPEC-MOK-001` (`conforms_to`) |
| departs | `ARCH-MOK-001 → SPEC-MOK-002` (`conforms_to`) |

The two departures are not a remediation: `ARCH-MOK-001`'s own `updated` moved to `2026-08-20` when `WO-MOK-012`
ratified its public-item prohibition, so it no longer predates either specification. **Every one of the eight is a
reassessment owed by an artifact owner and none is authorized here.**

`W-HEX-001` stood at three when this item was measured, accepted on the precedent the first two set, and `WO-MOK-013`
did not appear in it because the work order was still `approved`. **The transition to `implemented` added the fourth, as
predicted here**: at that status the inspector reports `W-HEX-001` against `WO-MOK-010`, `WO-MOK-011`, `WO-MOK-012` and
`WO-MOK-013`, its warning count moves from 11 to 12, and *Active work* is empty. The warning fires despite this pack
existing, because evidence discovery keys on file names rather than on the directory; it is the same naming artefact the
first three are, `validation.txt` attributes them, and the validator stays PASS with 0 warnings on all four planes.

## 7. Both manual assessments, with author, date, role and terminal

| # | Assessment | Requirement | Status | Author | Date | Role | Terminal |
|---:|---|---|---|---|---|---|---|
| 1 | Reference-viewport gauge legibility | `REQ-MOK-047` | **SATISFIED** | the repository owner | 2026-08-20 | product owner | Windows Terminal on Windows 11 |
| 2 | Overlay discoverability | `REQ-MOK-048` | **OUTSTANDING** | **none** | — | product owner | — |

**Assessment 1 was taken and passed on 2026-08-20**, in the words the owner gave: *"SATISFIED — the bar carried it."*
This is `VER-MOK-005`'s assessment 2 re-taken, and it is the assessment this whole chain exists to let pass: the
adverse observation of 2026-08-20 on the roster gauges is answered by this row and by nothing else in this pack.
`VER-MOK-013` puts the manual assessment **above** the automated cases for `REQ-MOK-047` rather than beside them, so
the arithmetic in item 2 is a necessary condition this outcome does not rest on.

**A fact adverse to a clean pass was disclosed before the owner judged.** M01 under the contracted invocation loses
health in **five**-point steps — eighteen of them, ticks 261 to 278, then to zero at 279 — and `REQ-MOK-047`
guarantees only that a change of **ten** moves the fill. At 13 cells a five-point step is 0.65 of a cell, so roughly
every other one of those steps draws the same bar as the step before. The satiety window, ticks 200 to 260, falls
exactly one point per tick from 61 to 1 with no refill and is the window that exercises the guarantee cleanly. Both
were traced out of `non-perturbation-unobserved.txt` and named before the assessment was put, so the judgment was
taken against the property the requirement states rather than against a guarantee never made.

**Assessment 2 remains outstanding, and its route is now decided.** The contract asks for a person who has **not read
`SPEC-MOK-003` rule 7**. The available assessor has read it and ratified six amendments to that document on
2026-08-20, two of which — decisions 15 and 16 — fix the hint's content and the ladder's order of loss, so they know
the key from the specification rather than from the screen; `VER-MOK-013`'s residual uncertainty records that the
assessment "is available once per assessor" and that a second run on the same person confirms nothing. Of the three
options `manual-assessment.md` set out, the owner took **find an admissible assessor** as an **assurance-owner** act
on 2026-08-20, on the ground that it is the only one that verifies the requirement — amending `VER-MOK-013` and
closing the row by decision are both recorded as declined. **The option that verifies the requirement was preferred
over the option that closes the row**, which is why this row is still outstanding rather than closed as
`VER-MOK-005`'s assessment 7 is.

That decision divides the work: the agent prepares the frame and the question, the owner finds the person. The agent's
half is delivered — `discoverability-frame.txt` (48 × 160, the reference viewport) and
`discoverability-frame-floor.txt` (22 × 34, the floor and the harder case) carve two frames out of `frames.txt`
without its viewport-naming banners, and `discoverability-assessment.md` is the packet an administrator can work from
without reading rule 7 to the assessor. In each frame the character `?` appears on the header row and nowhere else.

**Consequence, and it has since been drawn.** `VREC-MOK-013` was captured on 2026-08-20 against `41c20ca` and does
exactly this: it records `REQ-MOK-047` as verified on assessment 1, `REQ-MOK-049` on the automated cases — that
requirement was never gated on a manual assessment, and `VER-MOK-013` states that and gives its reason — and it
**discloses `REQ-MOK-048` as unverified** while assessment 2 stands. The record is a **`ready` candidate**: the
verification decision on it is the assurance owner's and is still outstanding. None of this blocked the implementation
this pack evidences, and none of it is retired by the work order's move to `implemented`.

> **Later fact, 2026-08-20 — the decision was taken, and it did not go as this paragraph anticipated.**
> `VREC-MOK-013` is **`verified`**, and `REQ-MOK-048` is verified with it — **on the requirement's four
> automated cases, accepted by the assurance owner in place of the primary evidence** rather than on
> assessment 2, which is still **OUTSTANDING with no author**. The record therefore does **not** disclose the
> requirement as unverified. **`VER-MOK-013` is not amended** and still says of that case that it is *"a
> necessary condition and not the property"*, so the contract is **not satisfied** on this one count, and
> `WO-MOK-013` did not move and stays `implemented`. `assurance-decision.md` records the instruction verbatim,
> the three bases it was read against, what was accepted, and the nine things the acceptance does not retire.

**A third act was taken on the same live pass.** Having watched the implementation run at `160 × 48`, the owner was
asked whether decision 1 is re-opened on the corrected eight-to-four figure — the log now carries four recent events
where it carried eight. It is not: **decision 1 stands**, as a product-owner act of 2026-08-20, this time on the
corrected figure and after seeing the result rather than only the arithmetic.

## 8. What was left open, and what was decided not to do

### Found while implementing, reported rather than fixed

| # | Finding | Why it is reported |
|---:|---|---|
| 1 | **`SPEC-MOK-003`'s *shrinking terminal* example** said the log "shrinks to 6 rows, since the taller log needs both thresholds" and the canvas "becomes 71 × 36" — both false once `160 × 48` already has six rows. **Corrected**; a seventh location the work order's enumeration missed. | Corrected as a consequence, not as an amendment of its own: it illustrates a provision rather than stating one. The miss is reported, which is what amendment 6's row says it would be. |
| 2 | **`VER-MOK-005` acceptance scenarios 4 and 5** said the log "shrinks to six rows" on narrowing and "holds six rows rather than ten" at `160 × 40`. **Corrected** to "keeps its six rows" and "holds its six rows at both heights". | Same reason. The scenarios' figures `71 × 36` and `67 × 28` are unchanged; only the comparison against a reference that no longer exists moved. Two more enumeration misses. |
| 3 | **`SPEC-MOK-004` rule 10's private-item count was one high before this work order.** It recorded `src/render.rs` as declaring 47 private items; at `a339902` it declared **46** — 30 functions and 16 constants. | A **pre-existing defect**, from the 2026-08-19 `WO-MOK-010`/`WO-MOK-007` row, inconsistent with the `WO-MOK-011` row's own count of 48 total declarations, which is the figure that was right. Corrected to the measurement (48 now) rather than absorbed silently into the new figure. |
| 4 | **`SPEC-MOK-003`'s `fear 0` counterexample** (line 925) and the **"whether the reserved fourth bar is reserved by layout arithmetic or by a placeholder" bullet** in *Explicitly unspecified decisions* (line 953) still describe the reserved fourth slot that rule 4 item 5 replaced with a computed `fear` on **2026-08-19**. | **Stale before this work order and not in its change surface.** The counterexample now contradicts the rule it illustrates. Correcting it is a technical-owner act on approved text under a different provision, and this work order's *Out of scope* does not reach it. |
| 5 | **`pub const ROSTER_TWO_LINE_WIDTH`** (`layout.rs:45`) is misnamed now the entry is three lines. | Renaming it changes a member of rule 6's recorded interface and would make three retained evidence files — `WO-MOK-006`'s census and two `WO-MOK-011` interface captures — read against a name that no longer exists. Stop condition 3 covers the count, not the name; the rename is a decision, not a measurement. |
| 6 | **The ladder has three rungs, and the rule's order of loss admits a fourth that is not implemented.** Amendment 5 fixes the order as joining words, then the pane's full name in favour of its initial, then **the overlay key**, with the axis and threshold never dropped. `announcement_text` builds three rungs — spelled, named, initials — and never drops the key. At the floor the third rung draws `HELD  ?  x8  r W100  L H38  i W140`, which is **exactly 34 columns** in a 34-column terminal: the header is full, with nothing to spare. | The rule fixes an *order* of loss, not that every loss must be realized, and the shortest implemented rung fits everywhere the observer draws — so no viewport needs the fourth. But the margin is zero, not comfortable: a threshold gaining a digit, or a fourth overlay-only pane, makes the third rung overflow, and the implementation then draws the shortest rung rather than dropping the key, since "a rung that fits nowhere is not silently truncated". Reported so the missing rung is a known gap rather than a surprise at the next threshold change. |
| 7 | **The two plane sweeps draw at a deduplicated subset of the plane, not at every point.** `every_bar_width_the_roster_can_produce_resolves_a_ten_point_step` and `no_entry_is_lost_silently_at_any_viewport_presenting_the_roster` both walk `34..=200 × 22..=60`, but draw one full frame per **distinct roster rectangle**: **33 of them**, of which 20 hide an entry. | Drawing 6,513 frames in a unit test is not affordable, and the roster rectangle is what both properties depend on. But two viewports sharing a roster rectangle and differing in the panes around it are not both drawn, so neither sweep is a per-viewport guarantee. Stated so it is not read as one. The no-roster half of the plane **is** visited at every point, asserting no gauge is drawn there. |
| 8 | **`a_subject_whose_record_was_never_ingested_is_presented_without_a_name`** (`src/verification.rs:484`) needed its setup modified. Its thirteenth subject used to be drawn below the twelfth; `12 × 3 = 36` now fills the interior exactly, so the test adds `observer.select_for_test(stranger)` to bring it into rule 4's window. | The assertion — `name_of(stranger) == None`, and the drawn entry carrying no name — is unchanged in what it requires, and the field it reads is the same field in either position. Reported because a modified setup on a pre-existing test is the shape a weakened assertion also takes, and a reader should not have to diff to tell them apart. |
| 9 | **Assessment 2 has no admissible assessor among the people this chain has available.** | Item 7. Not the agent's to resolve, and it was not resolved by amending the contract: the owner's route of 2026-08-20 is to find one. The frame and the question are prepared; the row stays outstanding until a person takes it. |

Findings 1 to 3 are all one kind: an amendment moved a rendering and the documents illustrating the old one were not
swept with it. That is the same cause `VER-MOK-005`'s three stale rows had, which `WO-MOK-012` recorded, and it
recurred inside a work order written to enumerate its own locations.

### Decided not to do

- **`evidence/WO-MOK-005/frames.txt` is not corrected and `VREC-MOK-005` is not edited.** Each records what was
  measured at a commit and remains true of that commit. `SPEC-MOK-003`'s own rule is that a capture is **re-run rather
  than corrected**, on the precedent of `evidence/WO-MOK-011/merge/`. What decision 1 does to `VREC-MOK-005` is make
  its geometry figures describe a superseded layout — a further instance of the staleness that record already
  discloses, and out of scope here.
- **`evidence/WO-MOK-012/closing-review.md` is not edited**, though its decision 13 has been superseded in its route.
  It records what the owner decided on the material they were shown, and it remains a true record of that act.
- **No status transition taken by the agent.** `WO-MOK-013` stayed `approved` through implementation and through the
  three decisions of item 7. `in_progress` would have recorded an authority given as an instruction rather than as a
  status act, and `implemented` asserts an accountable judgement. **The owner took the second on 2026-08-20**, as
  engineering owner, in the instruction *"you can transition WO-MOK-013 as implemented, and create VREC-MOK-013"* —
  recorded as decision 22 in `closing-review.md` and in the work order's *Transition to `implemented`* subsection.
- **No verification decision.** `commit_bound_verification` is `required`, and the same instruction created
  `VREC-MOK-013` — bound to `41c20ca`, recording `REQ-MOK-047` on assessment 1 and `REQ-MOK-049` on the automated cases,
  and **disclosing `REQ-MOK-048` as unverified**. It is a **`ready` candidate and not `verified`**:
  `DECISION_RIGHTS.md` reserves that transition to the accountable assurance owner, the instruction stated no judgement
  on the evidence, and the harness reports the outstanding act as `decision_required -> review-assurance-decision`.

  > **Later fact, 2026-08-20.** Taken since, by the owner as assurance owner, in a separate instruction and a
  > separate commit: `VREC-MOK-013` is `verified` and `REQ-MOK-048` with it, on the automated cases accepted in
  > place of the primary evidence. The queue is empty — `decision_required` **1 → 0**. Recorded as decision 23
  > in `closing-review.md` and in full in `assurance-decision.md`. Nothing else in this list moves.
- **Nothing is re-opened that a record binds.** `VREC-MOK-006` measured 97 items and 169 tests, and `VREC-MOK-007`,
  `VREC-MOK-010` and `VREC-MOK-011` measured their own trees. Each was correct where it was taken.

### Still open, and none of it authorized here

- The **eight `W-HEX-003`** reassessments and the **`W-HEX-001`s, now four** with this work order's own — item 6
  attributes them and resolves none.
- ~~**The verification decision on `VREC-MOK-013`**, which the harness names as the one item under *Decision
  required*.~~ **Closed 2026-08-20**: taken by the assurance owner, `decision_required` 1 → 0, decision 23. What
  is **not** closed by it is assessment 2, below, and the record bound to the merge commit, which is owed and not
  created. `assurance-decision.md`.
- **`WO-MOK-008`**, still `draft`, and the inspector's "Definitions pending (1)".
- **`VREC-MOK-005`'s staleness** and **manual assessment 7 of `VER-MOK-005`**.
- **`ROADMAP.md`'s Phase 2 claim**, which this work order's ROADMAP entry does not touch. Phases 2 and 2.5 are
  unreleased.
- **Assessment 2 of `VER-MOK-013`**, outstanding with no author. Its route is decided and its material is prepared;
  what remains is a person who has not read `SPEC-MOK-003` rule 7, which no agent can arrange. **Still open after
  decision 23**, which verified the requirement on the automated cases instead of on this assessment: what taking it
  would now do is confirm or contradict that, and `VER-MOK-013` stays unsatisfied on this count until it is taken.
- **A record bound to the merge commit.** `VREC-MOK-013` binds `41c20ca`; `master` carries this chain at `798e5d5`.
  The code is identical between them — `git diff --stat 41c20ca 798e5d5 -- . ':(exclude)docs'` is empty — but the
  harness graph is not, and **decision 23 neither created that record nor extended this one's binding**.
- The **`WO-MOK-012` identifier collision**, deferred to the merge by decision 3. Whichever branch reaches `master`
  second renumbers before it lands. **Determined 2026-08-20 and not in this side's hands any more**: this chain
  merged first, `origin/feature/phase-4a-definition` has not, so the renumbering is that branch's; it now reports
  **9** conflicts against `master`.
- **A second collision, found on 2026-08-20 and measured in `identifier-collision.md`.**
  `origin/governance/adr-mok-006-third-party-crates` holds a different `WO-MOK-013`, `VER-MOK-013`, `VREC-MOK-013` and
  `REQ-MOK-047`, all pushed and all owner-approved on that side. Decision 3's rule reaches it unchanged; nothing was
  renumbered here. **Determined the same way**: that branch is unmerged and now reports **8** conflicts against
  `master`, four of them `add/add` on the colliding names. Stop condition 8 did not fire — this branch was first.
- **No push, pull request, tag or release.** `WO-MOK-013`'s *Out of scope* puts all four outside it, and none was
  authorized in this work. **The owner authorized publication separately** and the branch merged through pull request
  [#32](https://github.com/mmzen/Mokiterions/pull/32) at `798e5d5`; those are the owner's acts rather than this work
  order's discharge, and there is still no tag and no release.
