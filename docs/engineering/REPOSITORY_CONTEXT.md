# Repository Context for Mokiterions

> Repository-owned after installation. This context helps engineering agents operate the repository, but it is not approved product intent, a requirement, architecture approval, work authorization, verification, or release authority. Replace every named placeholder with owner-confirmed facts before implementation preflight.

## Purpose

- Repository purpose: Build Mokiterions, a Rust proof-of-concept for an autonomous artificial-life simulation, while using SE Harness to keep engineering work explicit and traceable.
- Primary users or operators: Developers building the simulation and people running or observing simulation experiments.
- Accountable repository owners: ```product owner```

## Commands

- Setup: Install `rustup`. `rust-toolchain.toml` pins the toolchain, so the first `cargo` command run in this repository installs and selects that exact version together with `rustfmt` and `clippy`; do not install a toolchain by hand and do not override the pin with `+stable`. Harness commands run as `python -m se_harness` — `harnessctl` is not on `PATH`. No additional project setup is currently required.
- Build: `cargo build`. `cargo build -p Mokiterions` builds the engine package alone, which is the form that answers a question about the engine. The root is a virtual workspace manifest, so a bare `cargo build` builds both members; the `-p` form is the one to reach for when the answer must be about one package.
- Test: `cargo test` runs both packages and both tiers of each. `cargo test -p Mokiterions` runs the engine package alone, with no terminal present; `cargo test -p mokiterions-tui` runs the observer package alone, also with no terminal present.
- Lint or format: `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`. `--locked` is not optional: `.github/workflows/release.yml` passes it, so a lint run without it can resolve around `Cargo.lock` and pass locally against dependencies the release would refuse.
- Release: `docs/RELEASE_RUNBOOK.md` is the ordered procedure. It is operator documentation, not authority: the accountable decisions it names are the release owner's, and `.github/workflows/release.yml` publishes nothing that is not already recorded under `docs/engineering/`.
- Additional required verification: Run the engine with `cargo run --bin Mokiterions -- <options>` and inspect its text output; run the observer with `cargo run -p mokiterions-tui -- <options>` in an interactive terminal. Each package's resolved dependency set must equal the set declared for it, at the declared versions and feature sets: `SPEC-MOK-002` rule 13 declares the engine's and `SPEC-MOK-003`'s *Declared dependency set* declares the observer's, `SPEC-MOK-005` rule 8.4 is the check, and `REQ-MOK-047` is the obligation. `cargo tree -p Mokiterions -e normal --locked --offline` resolving to one crate is what that comparison predicts today, because the engine's declared set is empty; it is a measurement of the declaration and no longer a rule of its own. Behavior-specific verification must be added as simulation requirements are approved.

## Architecture

- Packages: The repository is a Cargo workspace of exactly two packages, each under its own directory, with the root holding a workspace manifest and no package. `mokiterions-core/` holds the package `Mokiterions`, the simulation engine, governed by `ARCH-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-004`; `mokiterions-tui/` holds the package `mokiterions-tui`, the terminal observer, governed by `ARCH-MOK-002`, `SPEC-MOK-003` and `SPEC-MOK-004`. The directory name `mokiterions-core` is a directory name only: the package, its library target and its binary are still `Mokiterions`, `mokiterions` and `Mokiterions`. The observer depends on the engine by path; nothing depends on the observer. There is no third package. `SPEC-MOK-004` rules 1 to 3 are the authority for the layout.
- Entry points: The engine package builds two targets. `mokiterions-core/src/lib.rs` is the library target `mokiterions`, which owns the modules, the process-boundary function `execute`, and the read-only observation surface; `mokiterions-core/src/main.rs` is the binary target `Mokiterions`, a thin shim that locks and buffers the standard streams, calls `execute`, and converts its return value to a process exit code. The library target is named in snake case because the declared lint gate implies `non_snake_case`; the binary keeps the operator-facing name, which appears in the first line of `USAGE`. `SPEC-MOK-002` is the contract for both. The observer package also builds two targets: `mokiterions-tui/src/lib.rs` is the library target `mokiterions_tui`, which carries the presentation layer — layout, mapping, rendering, key dispatch, retention and export — and `mokiterions-tui/src/main.rs` is the binary target `mokiterions-tui`, which acquires the terminal, decides whether to launch, schedules and loops. The observer's binary is deliberately not thin: its start-up and launch decision are covered by tests that reach private items. `SPEC-MOK-004` rules 4 to 6 are the contract for both.
- Major components and responsibilities: A Rust engine containing the simulation rules and state, a bounded model-decision boundary, and simple text output; and a terminal observer that watches a run without altering it. Introduce additional modules only when concrete requirements make them useful. The library target's public interface is a closed read-only enumeration owned by `SPEC-MOK-002` rule 5; it carries values only, it hands out no borrow of and no reference into engine state, and it grows only when an approved requirement needs it to. `advance_tick` is its only mutating operation. The observer's library target has a public interface too, owned by `SPEC-MOK-004` rule 6, but it is closed differently: it is exactly the items that were already public before the target existed, and the check is that no item's visibility changed. It is not a trust boundary — the engine's is — and it holds no authority over world state.
- External services or dependencies: Each package declares its own dependency set, and its resolved graph must equal that declaration in both directions — `SPEC-MOK-002` rule 13 for the engine, `SPEC-MOK-003`'s *Declared dependency set* for the observer, with `SPEC-MOK-005` rule 8.4 as the check on release and on every pull request. **The engine's declared set is empty today, and that is a fact about the declaration rather than a rule**: it is empty because nothing has been admitted to it, not because it may not be. The observer's declared set holds one entry, `ratatui` `0.30.2` with `default-features = false` and features `crossterm`, `layout-cache`, `underline-color`, which resolves to 57 external crates on `x86_64-pc-windows-msvc`, 63 on `x86_64-unknown-linux-gnu` and 62 on `aarch64-apple-darwin`. Adding a dependency to either package means amending that package's declared set, which is the technical owner's act under `ADR-MOK-006`: decision 1's criteria are applied and recorded, decision 4's trust envelope is checked first and relaxes nothing — no network access, no credential handling, no asynchronous runtime, no database, no plugin system, no dependency injection, and no user-interface crate outside the observer package — and decision 11 reserves the simulation semantics, which no third-party crate may implement. A crate shared by both packages is admissible only as a declared entry of both. Escalation is still the route for anything the declared-set amendment cannot carry, and adding a crate is never an implementation agent's decision. The intended runtime decision model for simulated agents is owner-described as `OpenAI GPT nano`. Confirm the exact OpenAI API model identifier before integration. Project implementation is expected to be performed with Claude Code, primarily using Opus. No other external service is currently required.

## Repository constraints

- Generated paths: `target/` and `target/harness-dashboard/`.
- Test placement: Put a test where its subject is. This convention holds in **both** packages, and each package's tier placement has its own authority: `SPEC-MOK-002` rules 7 to 10 for the engine, and `SPEC-MOK-004` rules 8 to 10 for the observer. Both state the same rule — a test belongs in the public tier when it can be written using only the library target's public interface, with its assertions unchanged and with no item widened, and in the internal tier otherwise.
  - Engine: a test that exercises the public contract — argument parsing, exit codes, emitted event lines, the run summary, density resolution, termination, and the population floor — belongs in `mokiterions-core/tests/` and reaches the code through the library target's public API. A test that asserts engine internals — direct state mutation, observation construction, action application, survival decay, or regeneration — stays in a `#[cfg(test)] mod tests` beside the code it covers.
  - Observer: a test that exercises what the observer presents — layout selection, world-to-canvas mapping, the pane contents a viewport yields, key dispatch, event retention, export, the authority verdict, non-perturbation — belongs in `mokiterions-tui/tests/` and reaches the code through the library target `mokiterions_tui`. A test that asserts the presentation layer's internals — a private drawing helper, a private constant, a private scroll or pan calculation, the binary's start-up or its launch decision — stays in a `#[cfg(test)] mod tests` beside the code it covers. A test that uses one of the four `#[cfg(test)]` hooks on `Observer` is in the internal tier by definition, because those hooks do not exist in the build a test outside the crate links.
  - `cargo test` runs both tiers of both packages and no tier is optional, feature-gated, `#[ignore]`d, or dependent on a terminal.
  - Do not widen a type, field, or method to `pub`, and do not remove or gate a `#[cfg(test)]` attribute, in order to relocate a test. For the engine, `ARCH-MOK-001` prohibits exposing mutable world state, so a test that needs to reach authoritative state is evidence that it belongs inline, not that the boundary should be opened. For the observer the prohibition is `INT-MOK-005`'s and it is absolute: the interface is exactly what was already public, so a test that will not compile outside the crate is telling you its tier, not asking for an addition.
  - A relocated test keeps its assertions verbatim. A test that cannot keep them through the move is misclassified and stays inline.
- Observer test evidence: A claim about what the observer renders is asserted against an in-memory character buffer, never against a screenshot or a recording. `SPEC-MOK-003` and `VER-MOK-005` require it; a rendering claim with no buffer assertion behind it is not evidence in this repository.
- Restricted or sensitive paths: Model-provider credentials and other secrets must remain outside the repository and must not be committed.
- Files requiring specialized review: None identified at this stage.
- Local conventions not captured elsewhere: Keep it simple. Prefer direct Rust code and the smallest useful design over speculative abstractions. Use Claude Code, primarily with Opus, as the implementation agent. There is no graphical or web interface: the engine reports through simple text output and the observer is a terminal interface. Neither is a declared entry of the engine's dependency set, and no user-interface crate may become one: `ADR-MOK-006` decision 4 keeps that prohibition intact while relaxing the rest, so the observer's `ratatui` graph stays confined to the observer package.

## Git branching model

Follow this Git branching model and naming convention:

* **`master`** is the integration branch and contains all accepted engineering work.
* Create normal development branches from `master`:

    * **`feature/<short-description>`** for new functionality.
    * **`bugfix/<short-description>`** for normal defect fixes.
* Merge `feature/*` and `bugfix/*` back into `master` through pull requests.
* Create **`release/<major>.<minor>`** from `master` when a release enters stabilization, e.g. `release/1.4`.
* Only release-specific fixes may target a release branch. Use **`release-fix/<short-description>`**, created from the corresponding `release/*` branch.
* If a release fix also applies to the current product, propagate it to `master`, preferably by forward-porting/cherry-picking the relevant fix.
* Identify exact released versions using tags such as **`v1.4.0`**, `v1.4.1`, etc.

Branch names must use lowercase words separated by hyphens, for example:

`feature/resource-respawn`
`bugfix/invalid-energy-calculation`
`release/1.4`
`release-fix/startup-crash`

Do not develop normal features directly against a `release/*` branch.

> **Note:** This branching model is repository guidance, not an authoritative governance rule. It does not supersede SE Harness requirements, policies, validations, or authorized-work constraints. If a conflict exists, the applicable harness-governed rules take precedence.


## Maintenance

Review this file when commands, boundaries, ownership, or repository structure change. Put product decisions and approvals in the formal artifact chain under `docs/engineering/`, not in this context file.
