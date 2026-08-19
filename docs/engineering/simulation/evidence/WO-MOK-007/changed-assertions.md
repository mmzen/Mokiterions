# Changed assertions

`WO-MOK-007` in-scope item 4 permits exactly two pre-existing tests to change, and constraint 6 makes any third a
stop condition. `VER-MOK-007`'s static check 6 requires the before and after of each to be recorded verbatim. Both
are below, from `mokiterions-tui/src/render.rs`, against `master` at `54c21ab`.

Two tests changed. No third. In both, **every asserted string is byte-identical before and after**; what changed is
that `entry_lines` now returns styled lines rather than `String`s, so the test reaches the same text through
`Line`'s `Display`, which writes span content and nothing else. One assertion was **added** to each test, and in both
cases it asserts a band rather than replacing anything: `VER-MOK-007`'s acceptance scenario 1 in the first and its
`zero-is-red-and-empty` case in the second.

## 1. `the_bar_row_reproduces_the_specified_form`

Before:

```rust
        let lines = entry_lines(&agent, FULL_BAR, true);

        assert_eq!(
            lines[1],
            "     h ████████████████████ 100  s ████████████████░░░░  81  e ██████████████░░░░░░  72"
        );
```

After:

```rust
        // Rule 4.7 made an entry line a sequence of styled spans. `Line`'s `Display` writes span
        // content and nothing else, so the text asserted here is the text that reaches a cell.
        let lines: Vec<String> = entry_lines(&agent, FULL_BAR, true)
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_eq!(
            lines[1],
            "     h ████████████████████ 100  s ████████████████░░░░  81  e ██████████████░░░░░░  72"
        );
```

The remaining four assertions of this test — `lines[0].starts_with("M05  A  81:14         ")`,
`lines[0].ends_with("eat:F0058")`, `lines[1].trim_end() == lines[1]` and
`count(&lines[1]) == 3 * FULL_BAR + BAR_ROW_OVERHEAD` — are unchanged in text and in form, because `lines` is a
`Vec<String>` in both versions. This test is rule 4's mockup, so it is the one that would have caught a moved
character; it did not have to move.

One assertion was added at the end, which is `VER-MOK-007`'s acceptance scenario 1 stated literally:

```rust
        // Rule 4.7 on the mockup's own values: 100 and 81 are high, 72 is middle. Two gauges
        // sharing a band and one differing is the shape a band read from the row rather than from
        // the value would get wrong, and it would get it wrong while still reading all-green.
        let bands: Vec<Option<Color>> = entry_lines(&agent, FULL_BAR, true)
            .remove(1)
            .spans
            .iter()
            .map(|span| span.style.fg)
            .collect();
        assert_eq!(
            bands,
            vec![
                None,
                Some(Color::Green),
                None,
                Some(Color::Green),
                None,
                Some(Color::Indexed(208))
            ]
        );
```

It is placed here rather than in a sixth new test because this is where the mockup's fixture already lives, and
because its value is the *shape* of the case: two gauges in one band and a third in another, which
`each_gauge_carries_its_own_band_and_nothing_else_carries_one` does not cover with its three all-distinct values.

## 2. `a_zero_value_is_a_zero_and_an_absent_value_is_a_dash`

Before:

```rust
        let lines = entry_lines(&agent, 4, true);
        assert!(lines[0].ends_with(ABSENT), "{}", lines[0]);
        assert!(lines[1].contains("h ░░░░   0"), "{}", lines[1]);

        agent.applied_action = Some(Action::Wait);
        assert!(entry_lines(&agent, 4, true)[0].ends_with("wait"));

        // The one-line form keeps the numbers and drops the bars.
        let compact = entry_lines(&agent, 0, false);
        assert_eq!(compact.len(), 1);
        assert_eq!(compact[0], "M01  A  h  0 s  0 e  0");
```

After:

```rust
        let lines: Vec<String> = entry_lines(&agent, 4, true)
            .iter()
            .map(ToString::to_string)
            .collect();
        assert!(lines[0].ends_with(ABSENT), "{}", lines[0]);
        assert!(lines[1].contains("h ░░░░   0"), "{}", lines[1]);
        // Rule 4.7 puts zero in the low band. It stays a `0` with an empty bar, so what
        // distinguishes it from an absent value is still the character and not the colour.
        assert_eq!(
            entry_lines(&agent, 4, true).remove(1).spans[1].style.fg,
            Some(Color::Red)
        );

        agent.applied_action = Some(Action::Wait);
        assert!(
            entry_lines(&agent, 4, true)[0]
                .to_string()
                .ends_with("wait")
        );

        // The one-line form keeps the numbers and drops the bars.
        let compact = entry_lines(&agent, 0, false);
        assert_eq!(compact.len(), 1);
        assert_eq!(compact[0].to_string(), "M01  A  h  0 s  0 e  0");
```

Four assertions, four identical strings: `ABSENT`, `"h ░░░░   0"`, `"wait"` and `"M01  A  h  0 s  0 e  0"`. Three
of them gained a `to_string()` on the value under test and nothing else. The fifth assertion is new and is the
`zero-is-red-and-empty` case of `VER-MOK-007`'s matrix: it asserts that zero takes the low band while the two
assertions above it continue to assert that zero renders as `0` with an empty bar and stays distinct from `—`.

## Tests that did not change

`a_bar_row_shrinks_to_its_pane_and_never_overflows_it` asserts only on `bar_width` and `BAR_ROW_OVERHEAD`, so it
was untouched, as `WO-MOK-007`'s expected change surface predicted. Every other test in both packages is unchanged:
`git diff master -- mokiterions-tui/src mokiterions-tui/tests` touches two files, and the two above are the only
pre-existing tests inside them that differ.

## Tests added

Five in the internal tier (`mokiterions-tui/src/render.rs`) and two in the public tier
(`mokiterions-tui/tests/render.rs`); see `completion-summary.md` item 5 and `outstanding-amendment.md` for the
`SPEC-MOK-004` count correction they cause.
