# WO-MOK-003 evidence: completion summary

`VER-MOK-003` requires a completion summary naming the final target layout, the final public interface,
and the final tier populations. This is that summary, plus the adverse observations the work surfaced.

## Final target layout

One Cargo package, `Mokiterions`, edition 2024, with exactly two targets and no workspace:

```toml
[package]
name = "Mokiterions"
version = "0.1.0"
edition = "2024"

[lib]
name = "mokiterions"
path = "src/lib.rs"

[[bin]]
name = "Mokiterions"
path = "src/main.rs"

[dependencies]
```

| Target | Kind | Name | Path | Contents |
| --- | --- | --- | --- | --- |
| Library | `[lib]` | `mokiterions` | `src/lib.rs` | `pub mod cli;`, `pub mod simulation;`, `pub fn execute`, and the module doc stating rules 5 and 6. No simulation logic, no test. |
| Binary | `[[bin]]` | `Mokiterions` | `src/main.rs` | 19 lines: stream locking and buffering, one `execute` call, flush handling that maps a failed flush to exit `1`, and the conversion to `ExitCode`. No module declaration, no test. |

The two names differ deliberately. `cargo clippy --all-targets --all-features -- -D warnings` implies
`non_snake_case`, and a library crate named `Mokiterions` fails it outright; the gate was not relaxed and
no `allow` attribute was added. The binary keeps the operator-facing name because it is the first word of
`USAGE`, which `REQ-MOK-010`'s observable interface includes — and `baseline-comparison.md` confirms not
one byte of `USAGE` moved.

Not present, and each checked rather than assumed: no third target, no second package, no workspace, no
build script, no `[dev-dependencies]`, no `[features]`, no `pub(crate)`, and no `#[cfg]` attribute
anywhere in `src/` except the single `#[cfg(test)]` on the internal test module. `Cargo.lock` is
unchanged.

Source files: `src/lib.rs` (new, 60 lines), `src/main.rs` (reduced to 19 lines), `src/cli.rs` (test
module removed, parsing code untouched), `src/simulation.rs` (four visibility changes, one `impl
RunSummary` block added, six test functions and two now-unused constants removed). Five new files under
`tests/`.

## Final public interface

**13 items**, listed here with the classification `VER-MOK-003` asks for. The full inventory, the
method by which it was derived twice independently, and the trait impls reachable through it are in
`public-surface-inventory.md`.

| # | Item | Classification |
| ---: | --- | --- |
| 1 | `mokiterions::cli` | module (namespace) |
| 2 | `mokiterions::simulation` | module (namespace) |
| 3 | `mokiterions::execute` | pure function of its arguments; owns no state |
| 4 | `cli::USAGE` | value — `&'static str` |
| 5 | `cli::Command` | value type — `Help` \| `Run(Config)` |
| 6 | `cli::parse` | value constructor |
| 7 | `simulation::CELLS_PER_TERRITORY` | value — `usize` constant |
| 8 | `simulation::Config` | value type — five public fields, configuration inbound |
| 9 | `simulation::Density` | value type — `DEFAULT` (value), `parse` (value constructor), `resources_per_territory` and `Display` (pure functions of the value); field private |
| 10 | `simulation::Policy` | value type — two variants, `parse`, `Default` |
| 11 | `simulation::RunSummary` | opaque value type — four copying accessors; all eight fields private |
| 12 | `simulation::Simulation` | opaque handle — `new` (constructor) and `run`; no public field |
| 13 | `simulation::TerminationReason` | value type — two variants, `Display` |

Every item is inside `SPEC-MOK-002` rule 5's union. Nothing outside it is public: rustdoc generated no
page for any type, constant, or method on rule 6's prohibited list.

**Authorized but omitted.** Rule 5's accessor row authorizes six `RunSummary` values; four are exposed.
Population per territory and resource counts per territory by calorie class are **not** added, because
the additions list "is a ceiling, not a checklist" and no relocated test reads either. The reason is
recorded in the doc comment above `impl RunSummary` in `src/simulation.rs`, at the point of decision, as
well as in `public-surface-inventory.md`.

**No public item yields mutable or owned authoritative state.** No public signature returns `&T`,
`&mut T`, `impl Trait`, `Box<dyn Trait>`, an iterator, a slice, or a collection — checked mechanically,
not by reading. `Simulation::run` takes `&mut self` to advance the engine and returns an owned
`RunSummary`, with no lifetime tying the return to the borrow, so the compiler guarantees no reference
escapes. Full argument in `boundary-review.md`.

## Final tier populations

| Tier | Location | Tests |
| --- | --- | ---: |
| Public | `tests/cli.rs` | 5 |
| Public | `tests/process.rs` | 4 |
| Public | `tests/termination.rs` | 3 |
| Public | `tests/density.rs` | 2 |
| Public | `tests/viability.rs` | 1 |
| **Public subtotal** | five files, five integration-test targets | **15** |
| Internal | `src/simulation.rs`, one `#[cfg(test)] mod tests` | 37 |
| Internal | `src/cli.rs` | 0 |
| Internal | `src/lib.rs`, `src/main.rs` | 0 |
| **Internal subtotal** | | **37** |
| **Total** | | **52** |

52 before, 52 after, 0 lost, 0 gained, 15 relocated, 37 unchanged in place. Reconciled name by name in
`test-census.md`.

`src/cli.rs` ending with no test module is a result rather than an oversight: all five of its tests
needed only `cli::parse` and its error strings, so rule 7 moved all five and left nothing private in that
file to assert. Every one of the 37 internal tests has at least one named non-public dependency —
verified mechanically, with an empty-dependency list treated as a failure — so no test is inline merely
because it started there.

`cargo test` runs both tiers in one invocation across seven test executables plus a doc-test pass that
reports zero tests, with no feature, no environment variable, no `#[ignore]`, and no working-directory
requirement.

## Verification result

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | exit 0, no differences |
| `cargo clippy --all-targets --all-features -- -D warnings` | exit 0, no findings |
| `cargo build` | exit 0 |
| `cargo test` | **52 passed, 0 failed, 0 ignored** |
| Byte-for-byte comparison, 43 cells | **IDENTICAL** — digests, line counts, exit codes, all 40 summary lines, all 16 diagnostic cases |
| 10,000-tick resilience, four runs | no panic; survivors + deaths = 12 in all four; termination ticks unchanged |
| Per-seed survivors at the declared density | 8, 11, 8, 9, 11 — floor 8 met, identical to `VREC-MOK-002` |
| `python scripts/validate_engineering_artifacts.py` | PASS, 0 errors, 0 warnings |

Toolchain: rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable.

## Adverse observations

Two of `SPEC-MOK-002` rule 8's per-file subjects turned out to be only partly reachable through rule 5.
They are recorded here as adverse observations against the specification's subject wording, not against
the implementation.

1. **`tests/termination.rs` names termination by extinction as a subject.** The test that covers it forces
   extinction inside one tick by clearing the resource collection and writing agent health and satiety
   directly. Rule 6 prohibits exposing either. The test stayed internal with its assertions intact.
2. **`tests/density.rs` names the relationship between density, initial endowment, and capacity.** The
   test that covers it reads per-territory resource counts before any tick and drives regeneration across
   a replenishment interval. `RunSummary` reports counts only after a run ends, and no public item drives
   regeneration. The test stayed internal; the file covers the resolved-count half of its subject.

Neither triggered this work order's stop-and-escalate condition, which fires only when leaving a test
inline would leave a `VER-MOK-001` or `VER-MOK-002` case covered *only by a weaker assertion*. Both tests
keep their original assertions at their original strength. The correct remedy under rule 5's error
behavior is to record the insufficiency, not to grow the interface, and that is what was done. A future
amendment could reword rule 8's subject lists to match the tiers the placement rule actually produces.

No new test was written to close either gap. `VER-MOK-003` conserves the census at 52, so adding one
would itself have been a violation.

## Residual limitations

- **Equivalence is demonstrated on the declared matrix, not universally.** 5 seeds × 2 policies × 2
  densities × 2 trace settings at 1,000 ticks, plus two 20-tick traced runs, one default invocation, 16
  diagnostic cases, and four 10,000-tick runs. Densities outside `0.75%` and `1.50%`, seeds outside the
  declared five, and the release profile are not covered. Rule 11 scopes the obligation to the declared
  densities and seeds, so this is the scope of the claim rather than a shortfall in executing it.
- **The public interface is now a maintained contract.** It grows only when an approved requirement needs
  it to, with `SPEC-MOK-002` amended in the same act. That is a permanent obligation this change created.
- **Compile-time cost roughly doubled for the incremental case** — 554 ms to 1,006 ms for
  `cargo test --no-run` after touching `src/simulation.rs` — and `target/` grew from 26 MB to 88 MB. No
  obligation attaches; see `compile-time.md`.
- **The survivor floor still has no margin.** Two declared seeds sit exactly on eight. This work order
  preserved that exactly, but the underlying fragility that `VER-MOK-002` records is unchanged and
  belongs to the world model, not to this refactor.
- **Long-horizon extinction under the reference source is unchanged and still deferred.** Extinction at
  tick 9,154 at the default density is pre-existing accepted behavior, deferred to Phase 2 by the product
  owner on 2026-08-17. Reproducing it exactly is the correct outcome for an equivalence-preserving change.
