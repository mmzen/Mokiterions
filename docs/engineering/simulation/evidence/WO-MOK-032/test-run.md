# Test run

`cargo test --locked --workspace`, on Windows, at the candidate worktree. Green, with the four tests
`WO-MOK-032` adds.

## The four tests

| Test | Target | Site covered | Rule |
|---|---|---|---|
| `a_live_run_with_no_connector_is_refused_before_any_tick` | `mokiterions-core/tests/cli.rs` | `mokiterions-core/src/cli.rs:532` | 13.1's selection half |
| `a_live_run_with_no_transcript_output_is_refused_before_any_tick` | `mokiterions-core/tests/cli.rs` | `mokiterions-core/src/cli.rs:541` | 19.6 |
| `a_live_run_with_no_ceiling_is_refused_before_any_tick` | `mokiterions-core/tests/cli.rs` | `mokiterions-core/src/cli.rs:551` | 14.6, 19.2 |
| `an_empty_credential_is_treated_as_absent` | `mokiterions-core/tests/connector.rs` | `mokiterions-core/tests/support/canned_connector.rs:212` | 13.3 |

## Counts

Only the two targets this work order touches move. Every other figure is quoted to show it did not.

| Target | Before | After |
|---|---|---|
| `Mokiterions` `tests/cli.rs` | 30 | **33** |
| `Mokiterions` `tests/connector.rs` | 13 | **14** |
| `Mokiterions` `src/lib.rs` (internal tier) | 173 | 173 |
| `Mokiterions`, all other targets | 67 | 67 |
| `mokiterions-tui`, all targets | 202 | 202 |
| **Total passed** | **481** | **485** |

Ignored: 3, unchanged — 2 in `Mokiterions`' internal tier and 1 in `tests/replay.rs`. Failed: 0. Both
doc-test targets run 0 tests, unchanged.

`Mokiterions`' 283 is 173 + 33 + 14 + 3 + 2 + 3 + 1 + 8 + 17 + 19 + 5 + 5 over
`src/lib.rs`, `cli`, `connector`, `decisions`, `density`, `naming`, `no_outcome_threshold`,
`process`, `records`, `replay`, `termination` and `viability`. `mokiterions-tui`'s 202 is
59 + 8 + 5 + 7 + 11 + 18 + 29 + 5 + 7 + 22 + 31.

## What was not run and why

No live run and no provider call. Every assertion here is either a parser refusal, which reaches no
process at all, or an exchange with `tests/support/canned_connector.rs`, which answers from a script.
No real credential was present at any point. The one credential-shaped value in the suite is the
existing placeholder, and `an_empty_credential_is_treated_as_absent` passes the empty string, which is
not a credential.

`cargo fmt --all -- --check` exits 0.
