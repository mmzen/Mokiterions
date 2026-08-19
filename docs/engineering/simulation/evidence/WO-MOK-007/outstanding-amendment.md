# Outstanding amendment: `SPEC-MOK-004` rules 9, 10 and 11

**This is a third amendment provision. The owner approved two on 2026-08-19, both to `SPEC-MOK-003`, and was not
shown this one. It is reported rather than applied.** The implementation agent found it while measuring the two test
tiers after implementing, and it decided nothing here.

`WO-MOK-007`'s *Expected change surface* ends "No other file. In particular no file under `mokiterions-core/`, no
manifest, and **no other artifact**." `SPEC-MOK-004` is another artifact, so this work order cannot touch it. The
text below is drafted in full so that applying it requires no drafting, exactly as this work order's own approval
preconditions drafted the two `SPEC-MOK-003` provisions.

## Why an amendment is needed at all

`SPEC-MOK-004` rule 9 records a test count per public-tier file and a public-tier total; rule 10 records a test count
per internal-tier location, an internal-tier total, and — as the stated reason those tests cannot move — a count of
`src/render.rs`'s private items; rule 11 records the executed totals for each package and the workspace. This work
order adds seven tests: five in `mokiterions-tui/src/render.rs` and two in `mokiterions-tui/tests/render.rs`. Every
figure above that names `render.rs` or a total is now wrong.

Two of the three corrections are authorized by the rules themselves and are named here for completeness rather than
for decision:

- **Rule 11** says verbatim: "The clause this paragraph exists for is conservation across a move, not a ceiling on
  the corpus: **a work order that adds a test corrects these figures here**, and one that loses a test has a defect."
- **Rule 9** says its counts "are not a quota" and that where measurement disagrees "the count is corrected here, and
  the discrepancy is recorded as work-order evidence." This document is that record.

The part that is genuinely outstanding is rule 10's private-item parenthetical. It is a rationale figure with no
correction clause, and leaving it stale would make the rule's stated reason false.

**Rule 6 is unaffected.** Every item this work order adds is private: one function (`band`) and five constants
(`BAND_HIGH_COLOUR`, `BAND_MIDDLE_COLOUR`, `BAND_LOW_COLOUR`, `BAND_HIGH_FLOOR`, `BAND_MIDDLE_FLOOR`). The recorded
interface extent does not move, nothing widens, and rule 7 is satisfied by construction. That is the rule whose
amendment would have required the technical owner for a reason of substance rather than of arithmetic.

## The measurement

Taken at the implementation, against `master` at `54c21ab`.

| Figure | Rule records | Measured on master | Measured now | Δ |
|---|---|---|---|---|
| `tests/render.rs` tests | 8 | 8 | **10** | +2 |
| Public tier total (observer) | 80 | 80 | **82** | +2 |
| `src/render.rs` tests | 12 | 12 | **17** | +5 |
| Internal tier total (observer) | 32 | 32 | **37** | +5 |
| `src/render.rs` private items | 39 | 39 | **45** | +6 |
| — of which functions | 27 | 27 | **28** | +1 |
| — of which constants | 12 | 12 | **17** | +5 |
| `src/render.rs` public items | 2 | 2 | 2 | — |
| Observer executed tests | 112 | 112 | **119** | +7 |
| Engine executed tests | 60 | 60 | 60 | — |
| Workspace executed tests | 172 | 172 | **179** | +7 |

Every other row of rules 9 and 10 is unchanged and was re-measured to confirm it: `authority.rs` 4, `export.rs` 7,
`layout.rs` 10, `options.rs` 7, `spatial.rs` 7, `state.rs` 21, `verification.rs` 16; internal `verification.rs` 8,
`state.rs` 4, `main.rs` 8. The private-item counts count `src/render.rs` above its `#[cfg(test)]` module, on the same
basis rule 10 used: sixteen module-level constants plus the one function-local `BINDINGS`, which is how the recorded
12 was reproducible on master and would otherwise read 11.

The before and after totals are in `test-run-before.txt` (172) and `test-run.txt` (179), both all-passing.

## Provision A — rule 9, the public-tier table

`tests/render.rs` reads **10** rather than 8, and **Total** reads **82** rather than 80. Append after the paragraph
that begins "`tests/layout.rs` reads 10 rather than 7":

> `tests/render.rs` reads 10 rather than 8 as amended 2026-08-19 under `WO-MOK-007`. `SPEC-MOK-003` rule 4 as
> amended the same day gives the roster's three bars a colour band read from the value each presents, and
> `VER-MOK-007` requires a case for the two obligations that only a drawn frame can carry: that three different
> bands appear in one entry, where a single style shared by all three gauges is the obvious defect and would pass
> any single-gauge case; and that a selected entry's reversed video composes with its band rather than replacing
> it. Both cases reach the code through rule 6's interface only — `draw`, `Observer` and the option parser — so
> rule 8 places them in the public tier and nothing widens.
>
> | Added | Obligation it carries |
> |---|---|
> | `the_survival_bands_reach_the_frame_and_three_differ_in_one_entry` | `VER-MOK-007`'s `three-bands-in-one-entry` case, plus agreement between colour and band over every gauge in the pane |
> | `a_selected_entry_keeps_its_bands_under_reversed_video` | `VER-MOK-007`'s `selected-entry-composes` case, asserted as the band being unchanged by selection rather than merely present |

## Provision B — rule 10, the internal-tier table

The `mokiterions-tui/src/render.rs` row reads **17** rather than 12, and its *Why they cannot move* cell reads:

> assert drawing internals; the module declares 45 private items — 28 functions and 17 constants — against 2 public
> ones, and 5 of the 17 additionally use a hook

**Total** reads **37** rather than 32. Append after the paragraph about `verification.rs`:

> `mokiterions-tui/src/render.rs` reads 17 rather than 12 as amended 2026-08-19 under `WO-MOK-007`, and the module's
> private items 45 rather than 39. `SPEC-MOK-003` rule 4 clause 7 adds one private function and five private
> constants — the band function, the three band colours and the two band floors — and `VER-MOK-007` requires five
> cases that name them or name the private `gauge` and `entry_lines`: the band table over its whole domain with both
> boundaries by literal value, the text-identity sweep over 2121 cases, the per-gauge locality of a band, the
> absence of retained state, and the collapsed form taking no band. Rule 8 keeps all five inside the crate, and rule
> 7 forbids widening any of the six items to move them out. The public interface is unchanged, so rule 6's recorded
> extent does not move.
>
> | Added | Obligation it carries |
> |---|---|
> | `the_survival_bands_are_the_three_the_rule_fixes` | the band table over `0..=100`, both boundaries by literal value, and the monotonicity property |
> | `banding_changes_no_character_of_an_entry` | text identity over 21 widths × 101 values, against the unbanded form re-derived in the test |
> | `each_gauge_carries_its_own_band_and_nothing_else_carries_one` | style locality: three bands on one row, and no band on the indent, the separators or the reserved slot |
> | `a_band_reads_only_the_value_it_is_given` | that no previous value is retained, asserted structurally by interleaved reads |
> | `the_collapsed_form_takes_no_band` | clause 7's exclusion of the one-line form below 47 columns |

## Provision C — rule 11, the executed totals

The sentence "As amended 2026-08-19 the observer's total is **112** and the workspace's **172**, the engine's 60
unchanged, the three added by `WO-MOK-005`'s conformance to `SPEC-MOK-003` rule 5 as amended." gains:

> As amended again 2026-08-19 the observer's total is **119** and the workspace's **179**, the engine's 60 still
> unchanged, the seven added by `WO-MOK-007`'s conformance to `SPEC-MOK-003` rule 4 clause 7 — five in the internal
> tier and two in the public tier, under the corrected tables of rules 9 and 10.

The clause "`cargo test -p mokiterions-tui` runs the observer's three internal-tier targets and its eight public-tier
targets" is unchanged: this work order adds no file to either tier.

## Amendment record row

> | 2026-08-19 | **Recorded test and item counts corrected for `WO-MOK-007`.** `SPEC-MOK-003` rule 4 clause 7 as amended the same day gives the roster's three survival bars a colour band, and conforming to it adds seven tests and six private items to `mokiterions-tui/src/render.rs` and `mokiterions-tui/tests/render.rs`. **Rule 9**: `tests/render.rs` rises from 8 to **10** and the public tier from 80 to **82**, the two additions being the obligations only a drawn frame can carry — three bands in one entry, and a selected entry's reversed video composing with its band. **Rule 10**: `src/render.rs` rises from 12 to **17** and the internal tier from 32 to **37**; the module's private items rise from 39 to **45**, its functions from 27 to **28** and its constants from 12 to **17**, the six additions being the band function, the three band colours and the two band floors. **Rule 11**: the observer's executed total rises from 112 to **119** and the workspace's from 172 to **179**, the engine's 60 unchanged. No rule changes what it requires. **Rule 6 is untouched**: every added item is private, the recorded interface extent does not move, and nothing widens, so rule 7 holds by construction. No target, target name, path, package name, tier boundary, hook or prohibition changes. | **OUTSTANDING.** Requires the technical owner for rule 10's private-item figures, which are a rationale count with no correction clause. Rule 11's own text authorizes the total corrections — "a work order that adds a test corrects these figures here" — and rule 9's non-quota clause authorizes its table, so those two need no owner act and none is claimed. The repository owner approved two provisions to `SPEC-MOK-003` on 2026-08-19 and directed the implementation in the same act, but was not shown this consequence: the implementation agent found it afterwards by measuring both tiers, and has reported it rather than treating that approval as covering it. Records bound to commits are not re-opened — `VREC-MOK-006` measured 169 tests and `VREC-MOK-005` 172, and both were correct at their commits. |

## What applying this does not require

No source file changes. No test changes. No other artifact changes. The corrections are arithmetic on figures that
`test-run.txt` and the measurement table above already establish, and they are reversible by restoring the previous
numbers.
