# Repository Context for Mokiterions

> Repository-owned after installation. This context helps engineering agents operate the repository, but it is not approved product intent, a requirement, architecture approval, work authorization, verification, or release authority. Replace every named placeholder with owner-confirmed facts before implementation preflight.

## Purpose

- Repository purpose: Build Mokiterions, a Rust proof-of-concept for an autonomous artificial-life simulation, while using SE Harness to keep engineering work explicit and traceable.
- Primary users or operators: Developers building the simulation and people running or observing simulation experiments.
- Accountable repository owners: ```product owner```

## Commands

- Setup: Install the stable Rust toolchain. No additional project setup is currently required.
- Build: `cargo build`
- Test: `cargo test`
- Lint or format: `cargo fmt --all -- --check` and `cargo clippy --all-targets --all-features -- -D warnings`
- Additional required verification: Run the program with `cargo run` and inspect its text output. Behavior-specific verification must be added as simulation requirements are approved.

## Architecture

- Entry points: `src/main.rs` is the application entry point.
- Major components and responsibilities: The first stage is a single Rust application containing the simulation rules and state, a bounded model-decision boundary, and simple text output. Introduce additional modules only when concrete requirements make them useful.
- External services or dependencies: The intended runtime decision model for simulated agents is owner-described as `OpenAI GPT nano`. Confirm the exact OpenAI API model identifier before integration. Project implementation is expected to be performed with Claude Code, primarily using Opus. No other external service is currently required.

## Repository constraints

- Generated paths: `target/` and `target/harness-dashboard/`.
- Restricted or sensitive paths: Model-provider credentials and other secrets must remain outside the repository and must not be committed.
- Files requiring specialized review: None identified at this stage.
- Local conventions not captured elsewhere: Keep it simple. Prefer direct Rust code and the smallest useful design over speculative abstractions. Use Claude Code, primarily with Opus, as the implementation agent. The first stage has no graphical or web UI and reports through simple text output. A richer interface may be introduced later, but it is outside the current stage.

## Maintenance

Review this file when commands, boundaries, ownership, or repository structure change. Put product decisions and approvals in the formal artifact chain under `docs/engineering/`, not in this context file.
