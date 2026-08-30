+++
id = "WO-MOK-032"
type = "work_order"
title = "Cover VER-MOK-018 case L20's untested refusals"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-29"
updated = "2026-08-29"

[assurance]
commit_bound_verification = "required"
rationale = "The change is executable: four test functions are added to the engine's public and connector test targets, and a test is trusted engineering state. `VER-MOK-018` case `L20` is an automated-test case whose satisfaction is asserted by exactly these tests, and `WO-MOK-027`'s authorization decision will rest on the two-gate behaviour they cover being under test rather than merely written. A future reader who finds `L20` marked satisfied needs a commit-bound record naming the commit at which the assertions existed. The one deviation this work order takes -- rule 13.3's unreadable arm left untested -- is a disclosure a verification record must carry, not one a work order can close by itself."
decided_by = "engineering owner"

[execution_scope]
paths = [
  "docs/engineering/simulation/evidence/WO-MOK-032/",
  "docs/engineering/simulation/work-orders/WO-MOK-032.md",
  "mokiterions-core/tests/cli.rs",
  "mokiterions-core/tests/connector.rs",
]

[relations]
implements = ["REQ-MOK-072"]
specifications = ["SPEC-MOK-002", "SPEC-MOK-007"]
verification = ["VER-MOK-018"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-29T21:22:38Z"
decided_by = "engineering owner"
reason = "Approved by the repository owner acting as accountable engineering owner on 2026-08-29, by selecting the presented option. Two decisions were taken in the turns their questions were asked, each with the alternatives' costs measured: that this coverage work order lands BEFORE the rule 14 fifth-price work order, accepting that the fifth price will later amend the three new refusal tests along with a_live_run, cli.rs:566's message and connector.rs's PRICES constant, in exchange for the --spend-ceiling refusal being under test before the parser holding it is edited; and that rule 13.3's unreadable arm is left untested and recorded as a stated deviation, over introducing the first cfg(windows)/cfg(unix) in either package to exercise a catch-all in test-support code that the empty case already enters. Assurance is required and not discretionary: the change is executable."

[[lifecycle_events]]
from = "approved"
to = "in_progress"
decided_at = "2026-08-29T21:22:53Z"
decided_by = "engineering owner"
reason = "Execution started under the owner's decision of 2026-08-29 to approve and execute through the verification record. Preflight PASS at approved; the reading manifest was read."

[[lifecycle_events]]
from = "in_progress"
to = "implemented"
decided_at = "2026-08-29T21:33:39Z"
decided_by = "engineering owner"
reason = "DR-WO-COMPLETE decided implemented by the repository owner acting as accountable engineering owner on 2026-08-29, by selecting the presented option to approve and execute through the verification record. Four tests added, no src file in either package touched, cargo test --locked --workspace green at 485 passed and 0 failed against a baseline of 481, cargo fmt clean. Each of the four was shown to fail against the removal of the guard it asserts, and every removal was reverted; the mutation evidence records what was observed rather than a diff. Rule 13.3's unreadable arm is left untested as a stated deviation with its cost measured, and the handoff evidence directs a covering verification record to record VER-MOK-018 case L20 as satisfied in part."
+++

# Work Order: Cover VER-MOK-018 case L20's untested refusals

## Lifecycle

`draft` to `approved` authorizes execution. `approved` to `in_progress` starts it. `in_progress` to
`implemented` requires the four tests, `cargo test` green on both packages, and the retained evidence.
Assurance is `required`, so this work order does not stop at `implemented`: a verification record covers it
and carries the one disclosed deviation.

## Objective

`VER-MOK-018` case `L20` is an automated-test case over `REQ-MOK-072`'s two gates. Four of the conditions it
enumerates have no assertion anywhere in either package. Add the four tests that assert them, change no
behaviour, and disclose the one condition deliberately left untested together with what leaving it costs.

## The gap, as measured

`SPEC-MOK-007` rule 19.2 and rule 14.3 make `cli::parse` refuse a live-mode selection that omits any of the
four options a live run cannot be conducted without. The refusals are at:

| Site | Option | Rule | Asserted today |
|---|---|---|---|
| `mokiterions-core/src/cli.rs:535` | `--connector-path` | 13.1's selection half | no |
| `mokiterions-core/src/cli.rs:544` | `--transcript-output` | 19.6 | no |
| `mokiterions-core/src/cli.rs:554` | `--spend-ceiling` | 14.6, 19.2 | no |
| `mokiterions-core/src/cli.rs:566` | `--prices` | 14.3 | yes |

Only the fourth is covered, by `a_live_run_with_no_prices_is_refused_before_any_tick` at
`mokiterions-core/tests/cli.rs:1066`, built on the `a_live_run(prices: &str)` helper at `:899`. That helper
supplies all four options, so every existing live-run test passes whether or not the other three refusals
exist. **The `--spend-ceiling` refusal is the consequential one**: it is the host-side half of the protection
`REQ-MOK-071` asks for, and deleting the check leaves the suite green.

`SPEC-MOK-007` rule 13.3 gives an absent, empty or malformed credential one treatment -- the connector makes
no provider call and returns an error on the first exchange. Rule 13.1 puts that read in the connector and
rule 13.4 forbids either host to perform it, so the only implementation of the arm in this repository is
`mokiterions-core/tests/support/canned_connector.rs:212`, whose guard is `Ok(value) if !value.is_empty()`
with a single catch-all beside it. `tests/connector.rs` reaches it only through `Option<&str>`, and only
`None` and `Some(CREDENTIAL)` are ever passed: `no_credential_refuses_on_the_first_exchange` at `:406` takes
the absent arm and the matrix loop at `:364` takes both rows of rule 13.1's table. The empty value is never
passed, so the `is_empty` guard is unexercised.

## In scope

1. **Three refusal tests** in `mokiterions-core/tests/cli.rs`, one per uncovered site, each asserting that a
   live invocation missing exactly one required option is refused, that the message names the missing option
   and `--live`, and that no tick is taken. They follow
   `a_live_run_with_no_prices_is_refused_before_any_tick`'s existing shape, including its restraint: it
   asserts on the option names and not on the whole message, and it asserts no numeral appears.
2. **One credential test** in `mokiterions-core/tests/connector.rs` that passes `Some("")` and asserts the
   same outcome the absent arm produces -- no provider call, an error on the first exchange, and rule 19.5a's
   counted fallback -- so that rule 13.3's "one treatment" is asserted rather than assumed. The existing
   helper already takes `Option<&str>` and injects through `Command::env` on the child at `:150`, so no
   helper signature moves and no process-global environment mutation is introduced.
3. **The disclosure** of rule 13.3's unreadable arm, in this work order's *Deviations* section and in the
   retained evidence, in the terms the owner was shown.

## Out of scope

- **Any behaviour change.** No file under `src/` is touched in either package. If a new test fails, the
  finding is reported and this work order stops; it does not repair the engine.
- **Rule 13.3's unreadable arm.** Decided by the engineering owner on 2026-08-29, with the cost measured. See
  *Deviations*.
- **The fifth price.** `cache_write_tokens` and every consequence of it, including `--prices`' arity,
  rule 14.3a's format, `cli.rs:566`'s message, the `a_live_run` helper and `tests/connector.rs`'s `PRICES`
  constant. A separate work order, which the owner decided on 2026-08-29 lands **after** this one.
- **`VER-MOK-018` itself.** Case `L20`'s wording does not move. The tests are written to the case as
  approved.
- **Splitting `mokiterions-core/tests/cli.rs`** or adding a test file. `SPEC-MOK-002` rule 8 puts argument
  parsing in `tests/cli.rs`, which is where three of these belong.

## Deviations

**Rule 13.3's unreadable arm is left untested, and this is a stated deviation from `VER-MOK-018` case `L20`,
not an omission.** The case asks that "an empty or malformed credential is treated as absent". The empty half
is covered by item 2. The malformed half means a value that is not Unicode, which is what `env::var` fails on
beside an absent variable.

Reaching it was measured. It needs no `unsafe`: the credential travels to the connector as a child-process
environment entry and `Command::env` accepts any `AsRef<OsStr>`, and neither package calls `std::env::set_var`
anywhere. What it needs is a non-Unicode `OsString`, which has no portable constructor --
`OsStringExt::from_wide` on Windows and `OsStringExt::from_vec` elsewhere -- and therefore **the first
`cfg(windows)` or `cfg(unix)` in either package**; there is currently no platform-conditional code in any Rust
file in this repository. It also needs an `OsStr`-taking variant of the connector test helper's credential
parameter.

What that would buy is bounded. The arm is a single catch-all in
`mokiterions-core/tests/support/canned_connector.rs`, a test-support file; the empty case in item 2 already
enters it; and no engine code is exercised by the distinction, because rule 13.4 forbids either host to read
the credential at all. The owner was shown both routes with these costs and chose four tests with the arm
recorded, over five tests with platform-conditional code.

The consequence is disclosed rather than left to be found: after this work order, `L20` is satisfied except
for its malformed-credential clause, whose only implementation is test-support code reached by an adjacent
assertion. A verification record covering this work order must carry that as a partial disposition.

## Authorized decision envelope

The implementation agent may decide: each test function's name, following the suite's existing convention of
a sentence naming the behaviour; each test's doc comment and the rules it cites; whether a refusal test
truncates `a_live_run`'s argument list or builds its own, provided it does not change `a_live_run`; whether
the three refusal tests share a helper; and the exact assertion form, provided each asserts the missing
option's name, `--live`, and that no tick occurred.

The implementation agent may **not** decide: whether rule 13.3's unreadable arm is tested; whether any
`src/` file is touched; whether a refusal message's wording changes; whether `VER-MOK-018` is amended;
whether `--prices`' arity moves; whether a crate is added; or whether a test that fails is repaired rather
than reported.

## Constraints

- **No crate is added to either package**, and no dev-dependency either.
- **No `src/` file in either package is modified.** The execution scope admits none.
- **The credential never enters the repository, the library target, any workflow, or any produced byte.** The
  empty-credential test passes `""`, which is not a credential; the suite's existing placeholder
  `sk-not-a-real-key` is not a credential either, and no real one is present at any point.
- **No live run occurs and no provider is called.** Every test here is a refusal or a canned-connector
  exchange. Nothing in this work order spends money and nothing in it needs an action-time authorization.
- **No platform-conditional code is introduced**, per *Deviations*.
- **`a_live_run` is not modified.** The fifth-price work order moves it, and moving it here would put this
  work order's diff inside that one's subject.
- The existing test count is 481. Four tests are added, so the expected count is 485; any other total is a
  finding.

## Expected change surface

| Path | Change |
|---|---|
| `mokiterions-core/tests/cli.rs` | three test functions added |
| `mokiterions-core/tests/connector.rs` | one test function added |
| `docs/engineering/simulation/work-orders/WO-MOK-032.md` | this file, lifecycle only |
| `docs/engineering/simulation/evidence/WO-MOK-032/` | retained evidence |

No `src/` file, no manifest, no workflow, no other governance artifact.

## Required verification

`cargo test --locked` over both packages, green, with a test count of 485. Each of the four new tests must be
shown to fail against the code with its assertion's subject removed, or the test asserts nothing: for the
three refusal tests this means the corresponding `cli.rs` guard, and for the credential test the
`!value.is_empty()` condition. That demonstration is performed in the worktree and **not committed** -- no
`src/` file is in the execution scope, so the removal is reverted before the change set is assembled, and the
evidence records what was observed rather than a diff.

## Evidence to record

Under `docs/engineering/simulation/evidence/WO-MOK-032/`:

- `handoff-check.md` -- the checkpoint binding, with `artifact`, `checkpoint` and `formal_snapshot_sha256`.
- `test-run.md` -- the `cargo test` invocation and its per-target counts, before and after, and the four new
  test names.
- `mutation-check.md` -- for each of the four, what was removed, which test failed, and the assertion message,
  together with the statement that every removal was reverted.

## Stop and escalate conditions

1. **A new test fails against unmodified code.** The engine does not do what `L20` says it does. Report,
   stop, and do not repair.
2. **A test cannot be written without touching `src/`.** The scope is wrong, not the code.
3. **Any of the four survives the removal of its subject.** The test asserts something other than what it
   claims; report the measurement.
4. **The test count is not 485.** Something else moved.
5. **A credential, or anything resembling one, appears in any produced byte.** Stop, do not commit, escalate.
6. **`cargo fmt` or `cargo clippy` demands a change outside the execution scope.** Report it; do not widen
   the scope to satisfy a lint.
7. **Rule 13.3's unreadable arm turns out to be reachable without platform-conditional code.** The
   deviation's cost was measured and the owner decided on it; a cheaper route makes that decision stale and
   is the owner's to re-take.

## Completion report format

1. Each of the four tests: its name, its file, the site it covers, and the rule it cites.
2. The test counts before and after, per target.
3. The mutation check: four rows of removal, failing test, and message.
4. Rule 13.3's unreadable arm, restated as a deviation with its disposition.
5. Any finding, reported and not repaired.
6. The lifecycle facts, the formal snapshot digest, and the one typed next step.
