# `VER-MOK-005`'s extended cases, and the tests that discharge them

`WO-MOK-008`'s *Evidence to record* asks for this mapping, and the amendment that made it necessary is
recorded in `VER-MOK-005`'s row of 2026-08-22. That row withdrew an obligation this contract had reported
PASS against for as long as it stood: *Footer provenance fields* obliged six fields to be "present at every
viewport", which at `34 × 22` a twenty-digit seed beside a twenty-digit tick limit makes arithmetically
impossible, and **the row was mapped to a single case rendering seed 42 at `160 × 48`**. The obligation
quantified over viewports; the case named one. That is why this file states, for every row, *what the case
sweeps* rather than only which case it is.

Two tiers exist and `SPEC-MOK-004` rules 9 and 10 fix them. **In-crate** means `mokiterions-tui/src/render.rs`
under `#[cfg(test)]`, reached by `cargo test --lib`; **cross-crate** means `mokiterions-tui/tests/*.rs`,
which sees only the public interface. A case's tier is not a preference: `footer_row`, `Provenance`,
`SHED_ORDER` and `supplied` are private, and `SPEC-MOK-004` rule 7 prohibits widening the interface to
reach them, which `WO-MOK-008`'s *Out of scope* repeats.

## The eight added rows

| `VER-MOK-005` row | Req | Tier | Test | What it sweeps |
|---|---|---|---|---|
| No footer value is presented cut | `REQ-MOK-024` | cross-crate | `verification::the_footer_never_clips_a_value_at_any_declared_viewport` (`tests/verification.rs:874`) | 22 declared footer seeds × both members of the declared tick-limit set × 9 renderable viewports = **396 rendered frames**. Each asserts the row fits its pane, carries the seed whole, and that every maximal digit run in it equals a value of that run — the values read from `Observer::config` and `Observer::events`, not restated from the arguments. |
| The footer row fits its pane at every width | `REQ-MOK-024` | in-crate | `render::tests::no_footer_value_is_ever_cut_at_any_width` (`src/render.rs:2041`) | 7 provenance fixtures × widths `0..=200` = **1,407 rows**. Asserts the fit, the digit runs and the seed's presence. Clause 7's fall-back is the one admitted exception to the fit, and the assertion names it rather than excluding the width. |
| Fields leave the footer in the specified order | `REQ-MOK-027` | in-crate | `render::tests::fields_leave_the_footer_in_the_order_clause_4_fixes` (`src/render.rs:2007`) | 7 fixtures × 4 labellings × 7 shed levels = **196 rows**, each checked field by field against all six of `SHED_ORDER`: the first `shed` are absent, every later one present, and the seed leads the row. Matches on the separator-prefixed field so a digit run inside another field cannot satisfy it. |
| No field is shed while the width holds it | `REQ-MOK-027` | in-crate | `render::tests::the_footer_sheds_no_more_fields_than_the_width_requires` (`src/render.rs:1970`) | 7 fixtures × widths `0..=200`. For each width it computes the least shed level any labelling fits into and asserts the row is that level — so the case fails both on a field lost too early and on a row that overflows. |
| The entropy seed survives the floor | `REQ-MOK-027` | in-crate | `render::tests::the_entropy_seed_survives_the_floor_across_the_whole_u64_range` (`src/render.rs:2084`) | **44 seeds at `34 × 22`** — every decimal magnitude `10^0..10^19` and each one less, plus 0, 1, `u64::MAX` and `u64::MAX - 1` — against a tick limit of `u64::MAX`, the widest the start-up contract accepts, with the buffer truncated and a 40-character stamp supplied. This is `WO-MOK-008`'s in-scope item 3 in its own terms. |
| ″ | ″ | cross-crate | `verification::the_footer_sheds_the_specified_fields_at_the_floor` (`tests/verification.rs:941`) | **4 rendered rows at the floor, asserted character for character.** These are the rows the technical owner was shown when the order was fixed; the fourth is clause 6's case, `seed 18446744073709551615`. |
| The retained count and its truncation marker are one field | `REQ-MOK-027` | in-crate | `render::tests::the_retained_count_and_its_marker_are_never_separated` (`src/render.rs:2125`) | The 2 truncated fixtures × 4 labellings × 7 shed levels, asserting the count's presence and the marker's are the same boolean; and the 5 untruncated fixtures × widths `0..=200`, asserting no marker appears at any width. |
| A candidate commit costs no field rule 8 requires | `REQ-MOK-027` | in-crate | `render::tests::the_candidate_commit_is_shed_before_every_field_rule_8_requires` (`src/render.rs:2164`) | 4 seeds × 4 stamp lengths (1, 7, 12 and 40 characters) × widths `0..=200` = **3,216 comparisons** of a stamped run's shed count against the same run unstamped. Asserts `SHED_ORDER[0]` is the commit, and that at a width holding it the commit is present and labelled. |
| Labelling changes no value | `REQ-MOK-027` | in-crate | `render::tests::no_labelling_changes_a_value` (`src/render.rs:2252`) | 7 fixtures × 4 labellings: every labelling presents the seed, tick limit, current tick, retained count and density unchanged. This is the case that would fail a denser radix, which clause 8 now withholds in terms. |

## The two rows changed rather than added

| `VER-MOK-005` row | Tier | Test | How it is discharged now |
|---|---|---|---|
| **Footer provenance fields where the width holds them** — rewritten | cross-crate + in-crate | `render::the_footer_carries_the_provenance_and_nothing_environment_specific` (`tests/render.rs:203`), `render::tests::the_footer_survives_the_narrowest_viewport` (`src/render.rs:1832`), and `verification::the_footer_never_clips_a_value_at_any_declared_viewport` | Its second half — the entropy seed at every declared viewport for every declared footer seed and tick limit — is swept outright by the third case, 396 frames. Its first half is discharged at the two ends and **derived across the middle**; see *The one place this mapping argues* below. |
| **Commit field is compile-time or absent** — amended in place | in-crate | `render::tests::a_commit_variable_set_to_the_empty_string_is_not_a_supplied_commit` (`src/render.rs:2221`) | Asserts `supplied` maps `None` and `Some("")` alike to `None` and passes a non-empty value through, then sweeps widths `0..=200` for a run whose commit came from `supplied(Some(""))`, asserting no `#` and no `commit` label appears at any width. The prior mapping recorded this row as discharged "the absent case only"; the empty-string state is the one the superseded renderer drew as a bare `#`. |

## The one place this mapping argues rather than measures

The rewritten row's first half obliges all six fields at **every declared viewport** for the declared
verification seeds. Two cases assert it at the two ends of the declared range — seed 42 with all six fields
at `160 × 48`, and the default run at `34 × 22` — and no case sweeps the whole field set across the seven
viewports between them. The gap is closed by derivation, and the derivation's assumption is stated so it can
be checked:

- the footer is one row spanning the full width (rule 5), so the field set a row carries **is a function of
  the width alone** — no declared viewport's height reaches it;
- `the_footer_sheds_no_more_fields_than_the_width_requires` sweeps **every** width from 0 to 200, which
  strictly contains the five declared widths 34, 100, 120, 140 and 160;
- at the floor the default row is 32 columns of 34 with nothing shed, and the widest declared verification
  seed is three digits, two columns more.

**If the footer ever becomes two rows, or its content ever depends on the height, this derivation lapses and
the row needs a sweep.** `VER-MOK-005`'s *Provenance survival* property is the monotonicity half of the same
argument, and it is measured rather than derived.

## What this mapping does not claim

- **It does not claim a passing case is evidence.** Every one of these ten was run against the superseded
  renderer; `counterfactual.md` records that 7 of the 10 fail there, that the other 3 are blind by
  construction and are named as such, and that **two of the seven were themselves blind when first
  written** and were strengthened for that reason.
- **It does not claim the cases verify `REQ-MOK-027` as written.** They verify `SPEC-MOK-003` rule 8 as
  amended, which concedes the requirement's field set cannot hold at the floor. `VER-MOK-005` discloses
  that residual and it is the product owner's.
- **It does not claim clause 4's first row is approved text.** The case that measures it —
  `the_candidate_commit_is_shed_before_every_field_rule_8_requires` — measures conformance to a provision
  marked OUTSTANDING in `SPEC-MOK-003`. It asserts the position rather than assuming it, so a reversal
  changes one array element, one table row and this case.
- **It does not restate the 302 pre-existing cases.** They are unchanged and they measured nothing about
  rule 8's order of loss; `gates.md` records the full census.
