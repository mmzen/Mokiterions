# Verification results for WO-MOK-001

- Work order: `WO-MOK-001`
- Verification contract: `VER-MOK-001`
- Captured: 2026-08-11
- Environment: Windows, stable Rust, debug profile

## Repository gates

### Formatting

```text
cargo fmt --all -- --check
exit: 0
```

### Lint

```text
cargo clippy --all-targets --all-features -- -D warnings
exit: 0
Finished `dev` profile [unoptimized + debuginfo]
```

### Automated tests

```text
cargo test
exit: 0
running 22 tests
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The initial compile identified one missing standard `Hash` derive used by a test helper. It was corrected without changing simulation behavior; the final formatter, test, lint, and build runs all passed.

### Build

```text
cargo build
exit: 0
Finished `dev` profile [unoptimized + debuginfo]
```

### Harness validation

```text
.venv\Scripts\harnessctl.exe validate .
Engineering artifact validation: PASS
Artifacts: 19 | Errors: 0 | Warnings: 0
```

Start preflight passed for `WO-MOK-001` in both `approved` and `in_progress` states. Review preflight also passed with the completed implementation and retained evidence while the work order remained `in_progress` pending the accountable lifecycle decision.

## Deterministic replay

Two executions of the built binary used `--seed 42 --ticks 100`.

```text
Replay equal: True
Replay hash 1: 6ed92d9eb25bfbcfd4f1778494580789b00e3bfb52659fffa4b783789dba16a6
Replay hash 2: 6ed92d9eb25bfbcfd4f1778494580789b00e3bfb52659fffa4b783789dba16a6
Replay summary: summary reason=extinction ticks=84 survivors=0 deaths=12 territory_a=0 territory_b=0 food_a_low=5 food_a_medium=3 food_a_high=2 food_b_low=3 food_b_medium=4 food_b_high=4
```

The hashes cover normalized UTF-8 standard output with a final newline. They demonstrate identical observed output for this working tree, not commit-bound provenance.

## Optional action-trace comparison

The same simulation was run with seed `17` for five ticks, once with tracing disabled and once with `--trace-actions`.

```text
Trace count: 60
Trace preserves core output: True
Trace summary: summary reason=tick_limit ticks=5 survivors=12 deaths=0 territory_a=6 territory_b=6 food_a_low=1 food_a_medium=1 food_a_high=1 food_b_low=1 food_b_medium=1 food_b_high=1
```

The automated test additionally compared complete authoritative in-memory state, including tick, agents, food, entropy state, and next food identifier. Traced and untraced states were equal.

## Long configured run

The binary was run with `--seed 123 --ticks 10000`. Extinction correctly terminated it before the configured limit.

```text
summary reason=extinction ticks=69 survivors=0 deaths=12 territory_a=0 territory_b=0 food_a_low=1 food_a_medium=3 food_a_high=5 food_b_low=2 food_b_medium=3 food_b_high=4
```

No panic, overflow, or unbounded state growth was observed.

## Manual text-output assessment

Command:

```text
cargo run --quiet -- --seed 42 --ticks 20 --trace-actions
```

The first tick produced twelve ordered action traces, one for each living Mokiterion. Representative lines:

```text
tick=1 subject=M01 event=action_trace result=proposal:move:south,status:accepted,detail:position:102:56,position:102:56,territory:A,health:100,satiety:100,energy:100
tick=1 subject=M02 event=action_trace result=proposal:move:south,status:accepted,detail:position:92:51,position:92:51,territory:A,health:100,satiety:100,energy:100
tick=1 subject=M03 event=action_trace result=proposal:move:west,status:accepted,detail:position:116:29,position:116:29,territory:A,health:100,satiety:100,energy:100
tick=1 subject=M10 event=action_trace result=proposal:wait,status:accepted,detail:waited,position:117:94,territory:B,health:100,satiety:100,energy:100
```

Final summary:

```text
summary reason=tick_limit ticks=20 survivors=12 deaths=0 territory_a=6 territory_b=6 food_a_low=3 food_a_medium=1 food_a_high=1 food_b_low=2 food_b_medium=1 food_b_high=2
```

Assessment: ordering is readable, proposals and outcomes are explicit, state is visible, and the summary is comprehensible. The duplicated position inside the move detail and state fields is intentional: one identifies the action result and the other is the uniform post-action state field.

## CLI process checks

```text
Mokiterions.exe --help
exit: 0
first line: Usage: Mokiterions [--seed <u64>] [--ticks <u64>] [--trace-actions]

Mokiterions.exe --ticks 0
exit: 2
first line: configuration error: --ticks must be greater than zero
```

An injected output writer failure is covered by an automated process test and returns runtime exit code `1`.

## Architecture and dependency review

```text
cargo tree --edges normal
Mokiterions v0.1.0
```

- No external dependency is declared.
- No networking, async runtime, database, UI framework, model client, credential, or provider endpoint exists in `src/` or `Cargo.toml`.
- `DecisionSource::decide` receives only an immutable `Observation` and a seeded candidate index.
- Only `Simulation::apply_action` owns validation and action-specific mutation.
- The optional trace path receives immutable post-action state and does not consume entropy.

## Security and privacy review

- No credentials, tokens, secrets, provider URLs, filesystem inputs, or dynamic code execution were introduced.
- CLI input is parsed only as fixed flags and unsigned integers.
- The foundation performs no network access and runs without model-provider environment variables.
