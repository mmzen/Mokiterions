# Declared gates

Run at the tip of `wo-mok-008-footer-shedding`, in the forms `harnessctl preflight --work-order
WO-MOK-008` reports as this repository's commands. `--locked` is not optional on the lint gate:
`.github/workflows/release.yml` passes it, so a run without it can resolve around `Cargo.lock`.

## Formatter

```
$ cargo fmt --all -- --check
(no output)
exit 0
```

**A finding, recorded rather than absorbed.** `rustfmt` rewrote both touched files entirely to LF, in a
worktree where `core.autocrlf` is true and every other Rust source file is CRLF. Git normalises on commit,
so the committed content is unaffected either way, but the worktree was left inconsistent with the rest of
the repository. Both files were converted back to CRLF and `cargo fmt --all -- --check` re-run clean
afterwards, so the formatting is rustfmt's and the line endings are the worktree's.

## Linter

```
$ cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
    Checking mokiterions-tui v0.1.0 (C:\Users\mathi\Mokiterions-understand-20260822-085811-11\mokiterions-tui)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.84s
clippy exit 0
```

## Tests

```
$ cargo test --workspace
```

| Target | Cases | Failed |
|---|---|---|
| `Mokiterions` lib (in-crate) | 97 | 0 |
| `Mokiterions` `tests/cli.rs` | 18 | 0 |
| `Mokiterions` `tests/decisions.rs` | 3 | 0 |
| `Mokiterions` `tests/density.rs` | 2 | 0 |
| `Mokiterions` `tests/naming.rs` | 3 | 0 |
| `Mokiterions` `tests/process.rs` | 7 | 0 |
| `Mokiterions` `tests/records.rs` | 17 | 0 |
| `Mokiterions` `tests/termination.rs` | 5 | 0 |
| `Mokiterions` `tests/viability.rs` | 5 | 0 |
| `mokiterions-tui` lib (in-crate) | **42** | 0 |
| `mokiterions-tui` bin (in-crate) | 8 | 0 |
| `mokiterions-tui` `tests/authority.rs` | 4 | 0 |
| `mokiterions-tui` `tests/export.rs` | 7 | 0 |
| `mokiterions-tui` `tests/layout.rs` | 11 | 0 |
| `mokiterions-tui` `tests/options.rs` | 8 | 0 |
| `mokiterions-tui` `tests/render.rs` | 22 | 0 |
| `mokiterions-tui` `tests/spatial.rs` | 7 | 0 |
| `mokiterions-tui` `tests/state.rs` | 22 | 0 |
| `mokiterions-tui` `tests/verification.rs` | **24** | 0 |
| **Total** | **312** | **0** |

**302 of these 312 cases passed before this work order and they measured nothing about it.** The count is
recorded in `counterfactual.md` for that reason: the whole suite passed against a renderer that presented a
tick limit of `18446744073` where the run's was `18446744073709551615`.

The ten added cases are 8 in `mokiterions-tui`'s in-crate tier (42, from 34) and 2 in its cross-crate
`tests/verification.rs` (24, from 22). `SPEC-MOK-004` rules 9 and 10 fix that split; `verification-mapping.md`
states which case discharges which `VER-MOK-005` obligation and why each sits where it does.

## Dependency tree

```
$ cargo tree -p Mokiterions -e normal --locked --offline
Mokiterions v0.1.0 (C:\Users\mathi\Mokiterions-understand-20260822-085811-11\mokiterions-core)
tree exit 0
```

One crate. The engine's declared dependency set is empty and this work order did not touch
`mokiterions-core/` by one byte — `git diff --stat -- mokiterions-core` is empty, which is the measurement
rather than the claim. `REQ-MOK-026` and `REQ-MOK-050` are undisturbed: no dependency was added to either
package, and no build script was introduced, both of which `WO-MOK-008` puts out of scope.
