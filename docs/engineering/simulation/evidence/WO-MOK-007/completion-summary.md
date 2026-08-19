# Completion summary — WO-MOK-007

Numbered per `WO-MOK-007`'s *Completion report format*. Every disclosure is numbered, including the ones that are
unwelcome.

Implementation branch `feature/wo-mok-007-roster-bands`. Two commits precede the implementation: `93330f9` (the draft
artifact pack) and `0b0fe5d` (the owner's approval recorded, and the two approved provisions applied to
`SPEC-MOK-003`).

---

## 1. What was implemented, read back out of the source

Six items added to `mokiterions-tui/src/render.rs`, all private. Quoted from the source rather than from the
specification:

```rust
/// Rule 4.7's three survival bands. `Indexed(208)` is xterm's dark orange rather than
/// `Color::Yellow`, which `MEDIUM_COLOUR` already spends on a medium-class resource: two unrelated
/// meanings sharing one colour on one screen is avoidable here.
const BAND_HIGH_COLOUR: Color = Color::Green;
const BAND_MIDDLE_COLOUR: Color = Color::Indexed(208);
const BAND_LOW_COLOUR: Color = Color::Red;

/// The two boundaries rule 4.7 fixes: green at `80..=100`, orange at `40..=79`, red at `0..=39`.
const BAND_HIGH_FLOOR: u8 = 80;
const BAND_MIDDLE_FLOOR: u8 = 40;
```

```rust
/// Rule 4.7's band for one survival value.
///
/// A band is a second presentation of the number the bar already shows. It reads one `u8` the
/// engine computed and retains nothing, so `REQ-MOK-020`'s constraint against any quantity the
/// engine does not produce holds literally. The boundaries are the specification's, not this
/// implementation's.
fn band(value: u8) -> Color {
    if value >= BAND_HIGH_FLOOR {
        BAND_HIGH_COLOUR
    } else if value >= BAND_MIDDLE_FLOOR {
        BAND_MIDDLE_COLOUR
    } else {
        BAND_LOW_COLOUR
    }
}
```

`gauge` returns a styled span rather than a `String`, with the band on the whole gauge:

```rust
fn gauge(label: char, value: u8, width: usize) -> Span<'static> {
    let filled = (usize::from(value) * width / 100).min(width);
    Span::styled(
        format!(
            "{label} {}{} {value:>3}",
            "\u{2588}".repeat(filled),
            "\u{2591}".repeat(width - filled)
        ),
        Style::new().fg(band(value)),
    )
}
```

`entry_lines` returns `Vec<Line<'static>>` rather than `Vec<String>`. The bar row is six spans, which is what lets
three values hold three bands:

```rust
        Line::from(vec![
            Span::raw("     "),
            gauge('h', agent.health, bar),
            Span::raw("  "),
            gauge('s', agent.satiety, bar),
            Span::raw("  "),
            gauge('e', agent.energy, bar),
        ]),
```

The collapsed form returns early, before any gauge is built, and so takes no band:

```rust
    if !two_line {
        // Rule 4.7: the collapsed form has no bars and takes no band. It exists to keep the
        // numbers legible where the bar cells will not fit, and the numbers carry the level.
        return vec![Line::from(format!(
            "{:<5}{territory:<3}h{:>3} s{:>3} e{:>3}",
            agent.id, agent.health, agent.satiety, agent.energy
        ))];
    }
```

The roster's caller applies rule 4.6's reversed video to the line, not to each span:

```rust
        // Rule 4.7: the entry's style is the line's, which every span patches rather than
        // replaces, so reversed video covers the whole entry and each gauge keeps its band
        // inside it. Selection stays marked by reversal and never by colour.
        for line in entry_lines(agent, bar, two_line) {
            lines.push(line.style(style));
        }
```

Why that composes rather than collides, which is the claim `WO-MOK-007`'s stop conditions turned on: `ratatui`'s
`Line::render_with_alignment` calls `buf.set_style(area, self.style)` and then renders the spans, and `Cell::set_style`
patches only the fields the incoming style sets — `if let Some(c) = style.fg` — while inserting modifiers
(`self.modifier.insert(style.add_modifier)`). A line-level `REVERSED` and a span-level foreground therefore both
survive. `frames.txt` shows the result: `fg=Red bg=Reset mod=REVERSED` on the selected low-band gauge.

Diff against `master`: `mokiterions-tui/src/render.rs` +292/−21, `mokiterions-tui/tests/render.rs` +277/−1. No other
source file. No manifest. No file under `mokiterions-core/`.

## 2. The palette chosen, and why

| Band | Value range | Colour |
|---|---|---|
| high | `80..=100` | `Color::Green` |
| middle | `40..=79` | `Color::Indexed(208)` |
| low | `0..=39` | `Color::Red` |

This is the palette `WO-MOK-007`'s decision envelope named as intended, chosen under `SPEC-MOK-003`'s grant of "the
exact palette, provided every distinction remains available without colour". `Indexed(208)` is xterm's dark orange,
taken over `Color::Yellow` because `MEDIUM_COLOUR` already spends `Color::Yellow` on a medium-class resource in the
spatial view, and two unrelated meanings sharing one colour on one screen is avoidable here. On a terminal without
256-colour support the middle band degrades to that terminal's nearest colour, which costs nothing rule 2.5 relies
on, because the numeric value and the proportional fill still carry the level. Correctable by the owner in one word;
`manual-assessment.md` item 3 is the assessment that would prompt it.

## 3. Text identity

**2121 cases, all passing.** `banding_changes_no_character_of_an_entry` sweeps every bar width the layout can ask for
(`0..=FULL_BAR`, 21 widths) against every value in `0..=100` (101 values) and asserts the case count as `21 * 101`.
The comparison is against the unbanded form **re-derived inside the test**, not captured from the current
implementation, so a regression in `gauge` cannot ratify itself. A whole-entry case at twenty cells additionally
covers the five-column indent and the two separators, and `the_bar_row_reproduces_the_specified_form` still asserts
rule 4's mockup line byte for byte.

The bar-width range is not a guess: `a_bar_row_shrinks_to_its_pane_and_never_overflows_it` (unmodified) asserts
`bar_width(20) == 0`, `bar_width(45) == 6` and `bar_width(158) == FULL_BAR`, and sweeps `28..200` for the invariant,
so `0..=20` is the whole range.

## 4. The two boundary results

Each by its own literal value, on both sides, so an off-by-one cannot hide inside a range:

| Assertion | Result |
|---|---|
| `band(39) == Color::Red` | pass |
| `band(40) == Color::Indexed(208)` | pass |
| `band(79) == Color::Indexed(208)` | pass |
| `band(80) == Color::Green` | pass |
| `band(0) == Color::Red` | pass |
| `band(100) == Color::Green` | pass |

Plus the whole domain: 101 values compared against a band table stated in the test rather than read from the
implementation's constants; and monotonicity across 100 adjacent pairs, which is the property a trend encoding would
have failed.

## 5. Every existing test that changed

**Two**, both named in `WO-MOK-007` in-scope item 4, exactly as predicted. The verbatim before and after of each is in
`changed-assertions.md`. In summary:

| Test | What changed | What did not |
|---|---|---|
| `the_bar_row_reproduces_the_specified_form` | reaches the text through `Line`'s `Display` rather than a `String`; **one assertion added** for rule 4.7 on the mockup's own 100/81/72 | all five existing assertions, including the mockup line byte for byte |
| `a_zero_value_is_a_zero_and_an_absent_value_is_a_dash` | same route change on four values; **one assertion added** for zero taking the low band | all four existing assertions: `ABSENT`, `"h ░░░░   0"`, `"wait"`, `"M01  A  h  0 s  0 e  0"` |

No third pre-existing test changed, in either package or either tier. `a_bar_row_shrinks_to_its_pane_and_never_overflows_it`
was untouched, as the expected change surface predicted.

Seven tests were added: five internal-tier and two public-tier. See item 8, disclosure 8.1.

## 6. Gate results

| Command | Result | Evidence |
|---|---|---|
| `cargo fmt --all -- --check` | exit 0, no output | `static-checks.txt` |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, no finding | `static-checks.txt` |
| `cargo test --workspace` (after) | **179 passed, 0 failed** across 19 targets | `test-run.txt` |
| `cargo test --workspace` (before, `master` @ `54c21ab`) | 172 passed, 0 failed | `test-run-before.txt` |
| `git diff master -- mokiterions-core/` | empty | `core-and-manifests-untouched.txt` |
| `git diff master -- '*Cargo.toml' Cargo.lock` | empty | `core-and-manifests-untouched.txt` |
| `harnessctl validate` | **PASS**, 70 artifacts, 0 errors, 0 warnings | `harness-gates.txt` |
| `harnessctl preflight --phase review --work-order WO-MOK-007` | **PASS** | `harness-gates.txt` |
| `harnessctl inspect` | PASS, 70 artifacts, 215 relations, 0 errors | `harness-inspect-and-dashboard.txt` |
| `harnessctl dashboard` | PASS, 0 errors, 9 warnings (all pre-existing `W-HEX-001`/`W-HEX-003`) | `harness-inspect-and-dashboard.txt` |
| `harnessctl doctor` | PASS on every managed and distribution file | `harness-inspect-and-dashboard.txt` |

The harness captures were taken with `HEAD` at `0b0fe5d` and the implementation in the working tree, so they cover the
artifacts rather than the commit. Commit-bound verification is `required` for this work order and is `VREC-MOK-007`'s
job, not this document's; no verification record is authored here.

## 7. Everything decided under the envelope

| Decision | What was chosen | Where the envelope grants it |
|---|---|---|
| 7.1 Concrete colours | `Green`, `Indexed(208)`, `Red` | "the concrete colour values" |
| 7.2 Decomposition | one private `fn band(u8) -> Color`, two floor constants, three colour constants; `gauge` returns `Span<'static>`; `entry_lines` returns `Vec<Line<'static>>` | "how the band function and the styled entry are decomposed into private functions, types and signatures" |
| 7.3 Where reversal is applied | to the `Line`, via `line.style(style)`, not to each span | "whether the selected entry's reversed video is applied to the line or to each span" |
| 7.4 Test names and fixtures | the seven new names, the 12/55/88 fixture, the `specified_band` helper in each tier, the search for a three-band entry in the public tier | "test names, fixtures and helpers within their tier" |
| 7.5 Tier placement of the seven | five internal, two public | `SPEC-MOK-004` rule 8's placement rule: the five name private items, the two reach only rule 6's interface |
| 7.6 How the public tier locates a gauge | the pieces are derived from rule 4's form and the bar width is counted off the row, rather than hard-coded coordinates | not named in the envelope; it is test-internal structure under the same grant |
| 7.7 The three-band fixture in the public tier | found by advancing a seeded run until one Mokiterion spans three bands, rather than by writing down a tick | same grant. It lands at tick 71 on `M05` at 100/59/39; searched rather than fixed so the case survives a change in how fast the world moves |

Nothing on the envelope's **may not** list was decided: the band boundaries and the number of bands are the
specification's, zero's band is the owner's, no trend is computed, neither the collapsed form nor the inspector is
banded, no entry's rendered text changed, and **no lifecycle status was moved** — see disclosure 8.2.

## 8. What was not done, and the stop conditions that came close

**8.1 A third amendment provision exists and is not applied.** `SPEC-MOK-004` rule 9 records a per-file public-tier
test count, rule 10 a per-location internal-tier count and a private-item count for `src/render.rs`, and rule 11 the
executed totals. Adding seven tests and six private items makes eight of those figures wrong. The owner approved two
provisions on 2026-08-19, both to `SPEC-MOK-003`, and was not shown this one. `WO-MOK-007`'s expected change surface
ends "no other artifact", so `SPEC-MOK-004` was not touched. The full text, drafted so that applying it requires no
drafting, plus the measurement table and the amendment-record row, is in `outstanding-amendment.md`. Rules 9 and 11
carry their own correction clauses; rule 10's private-item figures do not, and that is the part genuinely requiring
the technical owner. **Rule 6 is unaffected** — every added item is private, so the recorded interface extent does not
move and nothing widens.

**8.2 `WO-MOK-007` is still `approved`, not `in_progress` and not `implemented`.** Its own *Out of scope* list says
"Any lifecycle status of any artifact, including this one", and its envelope forbids the agent deciding any lifecycle
status. The owner's instruction on 2026-08-19 covered setting `WO-MOK-007` and `VER-MOK-007` to `approved` and
directed the implementation; it did not cover the later transitions. Both remaining transitions are the owner's act.
Review preflight passes at `approved`, so nothing is blocked by this.

**8.3 No verification record.** `VREC-MOK-007` is not authored. Commit-bound verification is `required`, so it must be
captured against the commit that carries this implementation rather than against a working tree, and verification is
an accountable decision this agent does not hold.

**8.4 Three manual assessments are OUTSTANDING with no author**, including the one that matters most for the
feature's purpose: whether the three bands are distinguishable on the owner's terminal. No automated case in either
tier is offered as evidence that a colour reached a screen. See `manual-assessment.md`. These are additional to the
seven already outstanding under `WO-MOK-005`.

**8.5 Two of the palette constants are named as literals inside the internal-tier tests.** `specified_band` in
`src/render.rs` states `Color::Green`, `Color::Indexed(208)` and `Color::Red` as literals rather than importing
`BAND_HIGH_COLOUR` and its siblings, so a palette change fails those tests and must be made deliberately in both
places. The public tier names no colour at all: it asserts that two gauges agree in colour exactly when they agree in
band, and that unstyled cells read `Color::Reset`. This is a deliberate asymmetry, not an oversight, and it is the
reason a palette correction is "one word" in the source plus one line in a test rather than one word alone.

**8.6 Acceptance scenario 2's own numbers appear in no assertion.** `VER-MOK-007` scenario 2 names 44/8/91; the
obligation is asserted at 12/55/88 in the same shape, and `band-domain` separately asserts the bands of 44, 8 and 91.
Scenario 3's "the pane's text is identical to the text the same state produced before clause 7 existed" is carried by
the 2121-case text-identity property at the entry level and by the nine unmodified viewport cases, not by a captured
pre-change pane. Both are stated in `requirement-to-test-mapping.md` rather than implied.

**8.7 Stop conditions approached and cleared.**

- *"constraint 1 cannot be met — colouring cannot be added without moving a character"* — the nearest one. It was
  settled before implementing by reading `ratatui-core`'s `impl fmt::Display for Line`, which concatenates span
  content only, and it is now asserted over 2121 cases. Had it failed, rule 4's mockup would have had to change and
  that is a different amendment.
- *"any test outside the two named in scope item 4 requires modification"* — none did. Both named ones changed and
  nothing else.
- *"meeting the request appears to need a previous tick, a delta, or any value the engine does not compute"* — this
  was the original request's shape and it was reported to the owner before any code was written: satiety and energy
  decay by one every tick for every living Mokiterion, so "decreasing" is true of nearly every bar on nearly every
  tick. The owner replaced trend with level bands. `no-retained-tick.txt` shows the implemented form reads one `u8`
  through one call site and retains nothing.
- *"the reserved fourth-bar slot would acquire a colour, a label, a dash or a zero"* — it acquired none;
  `each_gauge_carries_its_own_band_and_nothing_else_carries_one` asserts exactly six spans and a row ending at the
  third value.
- *"a band boundary appears to need to differ per attribute"* — it did not; one table serves all three gauges and
  `band` takes no attribute argument, so a per-attribute boundary is not expressible without changing the signature.
- *"`harnessctl validate` or `preflight` reports anything the change cannot account for"* — both PASS. The nine
  dashboard warnings are the pre-existing `W-HEX-001` and `W-HEX-003` observations on `WO-MOK-001`…`WO-MOK-006` and
  the older artifacts, unrelated to this change.

**8.8 Also left alone.** No trend, no inspector band, no collapsed-form band, no spatial-palette change, no engine
change, no snapshot-type change, no new dependency, no manifest change. `feature/wo-mok-005-layout-axes` remains a
stale remote branch and is not this work order's to remove.
