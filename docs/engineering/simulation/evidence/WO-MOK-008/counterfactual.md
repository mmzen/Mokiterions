# Each added case, run against the superseded renderer

**A case that passes on the implemented tree establishes nothing on its own.** The whole 302-case suite
passed against the renderer that presented a tick limit of `18446744073` where the run's was
`18446744073709551615`, and a retained count of `13` where the run had retained `136`. The count of
passing cases was never evidence about rule 8, and there is no reason to think ten more passing cases are,
unless they are shown to fail against the behaviour they were written to exclude.

So each added case was run against the superseded renderer, substituted behind the same private signature:
`footer_row` becomes the four-tier ladder and its unspecified fall-through as of `f7b1c45`, and `supplied`
becomes the identity. `render.rs` is restored from an in-memory copy afterwards and the revert verified by
SHA-256 in both directions.

## Result: 7 of the 10 added cases fail against the superseded renderer

| Case | Tier | Against the superseded renderer |
|---|---|---|
| `the_footer_sheds_no_more_fields_than_the_width_requires` | in-crate | **fails** — returns `s0 t100 @3 e136` where the width holds no declared form and clause 7 fixes `s0` |
| `no_footer_value_is_ever_cut_at_any_width` | in-crate | **fails** — `width 0: "s0 t100 @3 e136" overflows its pane` |
| `the_entropy_seed_survives_the_floor_across_the_whole_u64_range` | in-crate | **fails** — `seed 0: "s0 t18446744073709551615 @0 e100000" overflows the floor` |
| `the_candidate_commit_is_shed_before_every_field_rule_8_requires` | in-crate | **fails** — `a 1-character stamp shed 5 preamble field(s) where the same run unstamped shed 0` |
| `a_commit_variable_set_to_the_empty_string_is_not_a_supplied_commit` | in-crate | **fails** — `Some("")` where clause 2 as amended fixes `None` |
| `the_footer_never_clips_a_value_at_any_declared_viewport` | cross-crate | **fails** — `seed 9999, ticks 18446744073709551615 at 34x22: "s9999 t18446744073709551615 @0 e13" presents "13", which is no value of this run` |
| `the_footer_sheds_the_specified_fields_at_the_floor` | cross-crate | **fails** — `"s18446744073709551615 t100 @0 e136"` where clause 4 fixes `"s18446744073709551615 t100 d0.75%"` |
| `fields_leave_the_footer_in_the_order_clause_4_fixes` | in-crate | passes — asserts the amended rule's own machinery |
| `no_labelling_changes_a_value` | in-crate | passes — asserts the amended rule's own machinery |
| `the_retained_count_and_its_marker_are_never_separated` | in-crate | passes — clause 5's pairing held in the superseded row too |

**The three that pass are blind by construction and are named as such rather than counted as coverage.**
They assert `Provenance::row` directly, which is the machinery clause 4 introduces; against a *true*
revert they would not compile at all, because the type does not exist there. The substitution keeps the
machinery and replaces only the behaviour that composes the row, which is what makes the other seven
runnable against both trees.

## Two of the seven were blind when first written

This is the part worth recording, because it is the reason to run the check at all rather than assume it.

- `the_footer_never_clips_a_value_at_any_declared_viewport` originally swept every declared footer seed at
  the **default** tick limit. At the floor a 20-digit seed beside `t100` comes to 34 columns exactly under
  the superseded renderer, so nothing was cut and the case passed on both trees. It now crosses the
  declared seeds with both ends of the tick limit's range, and `VER-MOK-005` declares both sets for that
  reason.
- `no_footer_value_is_ever_cut_at_any_width` asserted only that no value in the row is cut, which the
  superseded renderer also satisfied — **it never cut a row itself.** It handed the pane a row wider than
  the pane and the pane kept a prefix. The case now asserts that the row fits, which is what makes the
  digit-run assertion a statement about what an operator reads.

Both were found by this check and by nothing else. `VER-MOK-005`'s *Residual uncertainty* discloses that
the check is not part of the gate: a case added there in future is not run against a counterfactual unless
its author chooses to.

## Full run

```
render.rs before : sha256 66c8a096f85293cb929c7b020874665a5d2c35e757437c73068e3d68a8ec2e56
render.rs patched: sha256 0a8a20da362b1c4654f6773995bcb83077e06db5ed815eeec7eb916e172afe3e

Each added case, run against the superseded renderer:

  the_footer_sheds_no_more_fields_than_the_width_requires            FAILS (detects the defect)
      mokiterions-tui\src\render.rs:2051:29:
      assertion `left == right` failed
      left: "s0 t100 @3 e136"
      right: "s0"
  fields_leave_the_footer_in_the_order_clause_4_fixes                passes (blind to the defect)
  no_labelling_changes_a_value                                       passes (blind to the defect)
  no_footer_value_is_ever_cut_at_any_width                           FAILS (detects the defect)
      mokiterions-tui\src\render.rs:2125:17:
      width 0: "s0 t100 @3 e136" overflows its pane
  the_entropy_seed_survives_the_floor_across_the_whole_u64_range     FAILS (detects the defect)
      mokiterions-tui\src\render.rs:2156:13:
      seed 0: "s0 t18446744073709551615 @0 e100000" overflows the floor
  the_retained_count_and_its_marker_are_never_separated              passes (blind to the defect)
  the_candidate_commit_is_shed_before_every_field_rule_8_requires    FAILS (detects the defect)
      mokiterions-tui\src\render.rs:2246:21:
      assertion `left == right` failed: seed 0, width 24: a 1-character stamp shed 5 preamble field(s) where the same run unstamped shed 0
      left: 5
      right: 0
  a_commit_variable_set_to_the_empty_string_is_not_a_supplied_commit FAILS (detects the defect)
      mokiterions-tui\src\render.rs:2276:9:
      assertion `left == right` failed
      left: Some("")
      right: None
  the_footer_never_clips_a_value_at_any_declared_viewport            FAILS (detects the defect)
      mokiterions-tui\tests\verification.rs:923:21:
      seed 9999, ticks 18446744073709551615 at 34x22: "s9999 t18446744073709551615 @0 e13" presents "13", which is no value of this run
  the_footer_sheds_the_specified_fields_at_the_floor                 FAILS (detects the defect)
      mokiterions-tui\tests\verification.rs:966:9:
      assertion `left == right` failed: ["--seed", "18446744073709551615"] at the floor
      left: "s18446744073709551615 t100 @0 e136"
      right: "s18446744073709551615 t100 d0.75%"

7 of 10 added cases fail against the superseded renderer.

render.rs after  : sha256 66c8a096f85293cb929c7b020874665a5d2c35e757437c73068e3d68a8ec2e56
revert verified  : yes
```
