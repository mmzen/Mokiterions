# WO-MOK-004 evidence: test census, reconciled

`VER-MOK-004`'s *Census growth is accounted* property requires the total to rise from 52 by exactly the
number of tests this work adds, every new test named, no pre-existing test removed, renamed, or ignored,
and the ignored count to stay 0.

Raw listings: `baseline/test-census.txt` and `after/test-census.txt`, both from `cargo test -- --list`.

| Measure | Before | After | Change |
| --- | ---: | ---: | ---: |
| Tests | 52 | 60 | +8 |
| Ignored | 0 | 0 | 0 |
| Removed | — | — | 0 |
| Renamed | — | — | 0 |

## Name-by-name reconciliation

`diff baseline/test-census.txt after/test-census.txt`, both sorted, in full:

```diff
9a10,12
> each_declared_default_is_stated_once: test
> each_documented_default_parses_to_the_applied_default: test
> every_option_the_synopsis_names_has_an_options_entry: test
50a54,58
> the_diagnostic_path_appends_the_whole_usage_text: test
> the_documented_options_are_exactly_the_options_the_parser_accepts: test
> the_entries_state_the_constraints_that_decide_validity: test
> the_flags_state_their_effect_and_no_default_value: test
> the_help_text_states_order_and_repetition: test
```

Eight lines, all additions. No `<` line and no changed line, so the diff is itself the proof that all 52
pre-existing names survive unaltered — a rename would appear as a paired removal and addition.

## Distribution by executable

| Target | Before | After | Change |
| --- | ---: | ---: | ---: |
| `src/simulation.rs` (internal tier) | 37 | 37 | 0 |
| `src/lib.rs`, `src/main.rs` | 0 | 0 | 0 |
| `tests/cli.rs` | 5 | 12 | +7 |
| `tests/process.rs` | 4 | 5 | +1 |
| `tests/density.rs` | 2 | 2 | 0 |
| `tests/termination.rs` | 3 | 3 | 0 |
| `tests/viability.rs` | 1 | 1 | 0 |
| doc-tests | 0 | 0 | 0 |
| **Total** | **52** | **60** | **+8** |

No test changed tier: the internal tier is unchanged at 37, and both files that grew are public tier
under `SPEC-MOK-002` rule 8. No test file was created and none was removed, so rule 8's declared
subjects are the same set as before.

`tests/cli.rs`'s subject is argument parsing. Seven of the eight additions belong there because their
claim is about `cli::parse` — what the program applies when an option is omitted — and not about a
string. The eighth is a claim about `execute`'s standard-error bytes, so it belongs in
`tests/process.rs`, whose subject is the process boundary.
