# Mutation check

Each of the four tests `WO-MOK-032` adds was run against code with the thing it claims to assert
removed, and each had to fail. A test that passes either way asserts nothing, which is the defect
this work order exists to repair — so demonstrating the assertion bites is part of the required
verification and not an extra.

## Result

| Removal | Test | Result |
|---|---|---|
| the `--connector-path` guard, `mokiterions-core/src/cli.rs:532` | `a_live_run_with_no_connector_is_refused_before_any_tick` | **failed** |
| the `--transcript-output` guard, `mokiterions-core/src/cli.rs:541` | `a_live_run_with_no_transcript_output_is_refused_before_any_tick` | **failed** |
| the `--spend-ceiling` guard, `mokiterions-core/src/cli.rs:551` | `a_live_run_with_no_ceiling_is_refused_before_any_tick` | **failed** |
| the `!value.is_empty()` guard, `mokiterions-core/tests/support/canned_connector.rs:212` | `an_empty_credential_is_treated_as_absent` | **failed** |

## How each failed

The three parser removals fail identically in shape: with the guard gone `cli::parse` returns
`Ok(Command::Run(Config { .. }))`, so `expect_err` panics carrying the configuration the parser would
have accepted. Each panic names its own file and line in `mokiterions-core/tests/cli.rs` — 1131, 1153
and 1180 — and the accepted `Config` is printed, which is the useful part: it shows the run that would
have proceeded. For the ceiling that configuration carries `spend_ceiling: None`, which is the run
`SPEC-MOK-007` rule 13.5 forbids a host to start.

The credential removal fails differently and more informatively. With the guard gone the connector
**answered** rather than refusing, so the assertion that fails is
`mokiterions-core/tests/connector.rs:464` — the per-record check for `refused` — and the record it
prints is a real exchange with a real observation in it. That is the failure mode worth having under
test: not an error, but a run that proceeded on an empty credential.

## One thing this measured that was not certain in advance

Whether an empty environment value reaches a child process on Windows *as empty* rather than as
absent. It does. Had it arrived as absent, the test would have entered rule 13.3's absent arm, passed,
and asserted nothing new — and it would have passed with the `!value.is_empty()` guard removed. It
failed, so the value arrives empty and the guard is what turns it into a refusal. The credential
travels through `Command::env` on the child at `mokiterions-core/tests/connector.rs:150`; neither
package calls `std::env::set_var` anywhere.

## Every removal was reverted

All four mutations were applied one at a time and reverted with `git checkout --` before the next.
`git status --porcelain` over `mokiterions-core/src/cli.rs` and
`mokiterions-core/tests/support/canned_connector.rs` is empty, and `git diff --stat` over `*/src/*` is
empty. Neither file is in this work order's execution scope, and neither appears in its change set.
No mutation was committed and none is recorded as a diff — only as what was observed.
