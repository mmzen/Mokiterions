# WO-MOK-004 evidence: the tests added

`VER-MOK-004` requires the output of the new tests and their names, and requires that no check be
written only against `cli::USAGE`, because such a check holds for every possible value of the constant
including an empty one.

Eight tests were added: seven in `tests/cli.rs` and one in `tests/process.rs`. Both files are public
tier under `SPEC-MOK-002` rule 8; every added test reaches the program through rule 5's interface, so no
test changed tier and no new test file was created. `tests/cli.rs`'s declared subject is argument
parsing, which is where the printed-to-applied equality belongs: it is a claim about `cli::parse`, not
about a string.

## Names, and the oracle each uses

| Test | File | Oracle outside `USAGE` |
| --- | --- | --- |
| `every_option_the_synopsis_names_has_an_options_entry` | `tests/cli.rs` | the synopsis block, compared against the options block; and a word count on each description |
| `the_documented_options_are_exactly_the_options_the_parser_accepts` | `tests/cli.rs` | the parser's own `match` arms, read out of `src/cli.rs`; then the running parser |
| `each_documented_default_parses_to_the_applied_default` | `tests/cli.rs` | `cli::parse` on an empty argument list |
| `the_entries_state_the_constraints_that_decide_validity` | `tests/cli.rs` | `cli::parse` on a value each stated constraint forbids |
| `the_flags_state_their_effect_and_no_default_value` | `tests/cli.rs` | `cli::parse` on an empty list and on `--trace-actions false` |
| `each_declared_default_is_stated_once` | `tests/cli.rs` | `SPEC-MOK-001`'s declared set, transcribed; occurrence counts within the text |
| `the_help_text_states_order_and_repetition` | `tests/cli.rs` | `cli::parse` on reordered and repeated arguments |
| `the_diagnostic_path_appends_the_whole_usage_text` | `tests/process.rs` | `execute`'s standard-error bytes |

## Why the added `tests/process.rs` test was needed

`tests/process.rs::help_exits_successfully` asserts that the help output **equals** `cli::USAGE`. That
assertion is true for every possible value of `USAGE`, so it constrains routing and the exit code and
says nothing about content. The other two standard-error assertions check the substring `Usage:`, which
the synopsis alone satisfies. Between them, the pre-existing suite would not have noticed an options
block that reached standard output and never reached standard error.

`REQ-MOK-018` is a property of the text, and the text is emitted on two paths, so the property cannot be
satisfied on one alone. `the_diagnostic_path_appends_the_whole_usage_text` asserts that the standard-error
text of an invalid configuration starts with `configuration error: ` and **ends with `cli::USAGE` in
full**. That pins the whole constant to the diagnostic path, which is the path fourteen of the sixteen
recorded cases take. No existing assertion was changed to achieve it.

## The central test, and why it carries no expected values

`each_documented_default_parses_to_the_applied_default` is the one that makes a hardcoded default safe.
`VER-MOK-004` names the failure mode explicitly: "A test that compares the text to a constant it declares
itself restates the drift problem one level up and is an adverse observation."

The test declares no value. For each of `--seed`, `--ticks`, `--policy`, and `--density` it scrapes the
default out of the printed text, feeds that token back through `cli::parse` as the option's value, and
requires the resulting `Config` to equal the `Config` from an empty argument list:

```rust
let applied = run_config(Vec::<String>::new());

for option in ["--seed", "--ticks", "--policy", "--density"] {
    let stated = documented_default(option)
        .unwrap_or_else(|| panic!("{option} declares a default and must state it"));

    assert_eq!(
        run_config([option, stated.as_str()]),
        applied,
        "the help states {stated} as the default for {option}, which is not the value \
         the program applies when {option} is omitted"
    );
}
```

This works for all four without a `Display` impl for `Policy` or `Density`, needs nothing that was not
already public, and fails if either side moves alone. `drift-demonstrations.md` records it failing in
both directions.

The four-option list is transcribed from `SPEC-MOK-001`'s *Help output* table rather than derived from
the text, deliberately: derived from the text, a default deleted from the help would be silently skipped
instead of failing. Written this way, it panics naming the option.

## Coverage totality in the strong form

`VER-MOK-004`'s residual uncertainty warns that the coverage property "would be caught … only if that
property is written over the parser's accepted options rather than over a hand-written list. The
distinction is recorded here because the weaker form is easy to write and looks identical when it
passes."

The strong form was written. `options_the_parser_accepts()` reads `src/cli.rs` through `include_str!`
and extracts the option string from each `match` arm, so the set is the parser's own, not a list in the
test:

```rust
include_str!("../src/cli.rs")
    .lines()
    .map(str::trim)
    .filter(|line| line.starts_with("\"--") && line.contains("\" => {"))
```

The extracted set is then confirmed against the running parser: each documented option is passed to
`parse` and must not be rejected as unknown, so a mis-extraction cannot silently produce a set that
happens to match the help. `drift-demonstrations.md` records the check failing when an option is added
to the parser and left out of the help.

The cost is a test that depends on the layout of a source file. If `src/cli.rs` is reformatted so the
arms no longer match, the test fails loudly rather than passing vacuously, which is the correct
direction for that failure mode to point. It is recorded here as a known coupling rather than left to be
discovered.

## Run output

From `after/test-run.txt`, the eight added tests:

```
test the_help_text_states_order_and_repetition ... ok
test the_entries_state_the_constraints_that_decide_validity ... ok
test each_documented_default_parses_to_the_applied_default ... ok
test every_option_the_synopsis_names_has_an_options_entry ... ok
test the_documented_options_are_exactly_the_options_the_parser_accepts ... ok
test each_declared_default_is_stated_once ... ok
test the_flags_state_their_effect_and_no_default_value ... ok
test the_diagnostic_path_appends_the_whole_usage_text ... ok
```

The whole suite, one `cargo test` with no extra flag and no environment variable:

```
running 37 tests   test result: ok. 37 passed; 0 failed; 0 ignored   (src/simulation.rs)
running  0 tests   test result: ok.  0 passed; 0 failed; 0 ignored   (doc-tests)
running 12 tests   test result: ok. 12 passed; 0 failed; 0 ignored   (tests/cli.rs)
running  2 tests   test result: ok.  2 passed; 0 failed; 0 ignored   (tests/density.rs)
running  5 tests   test result: ok.  5 passed; 0 failed; 0 ignored   (tests/process.rs)
running  3 tests   test result: ok.  3 passed; 0 failed; 0 ignored   (tests/termination.rs)
running  1 test    test result: ok.  1 passed; 0 failed; 0 ignored   (tests/viability.rs)
running  0 tests   test result: ok.  0 passed; 0 failed; 0 ignored
```

60 passed, 0 failed, 0 ignored.

## One name differs from the work order

`WO-MOK-004` listed the second test as `every_documented_option_is_accepted_by_the_parser`. It was
written as `the_documented_options_are_exactly_the_options_the_parser_accepts`, because it asserts both
directions — every documented option is accepted, and every accepted option is documented — and the
authorized name understates the second. Test names are inside the work order's authorized decision
envelope. Nothing else about the test differs from what was authorized.
