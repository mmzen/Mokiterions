+++
id = "SPEC-MOK-002"
type = "specification"
title = "Crate targets, public interface, and test placement"
status = "approved"
owners = ["technical owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
specifies = ["REQ-MOK-016", "REQ-MOK-017"]
+++

# Specification: Crate targets, public interface, and test placement

## Scope

This is the structural contract for the repository: which targets the package builds, exactly which items the
library target makes public, and where every automated test lives.

It states no simulation behavior. `SPEC-MOK-001` remains the single behavior contract, and rule 11 below binds this
specification to preserving it byte-for-byte. Where the two touch — `SPEC-MOK-001` currently delegates "test
organization and helper functions" to the implementation agent — that delegation is narrowed by rules 7 to 10 and
by nothing else; see *Compatibility and migration*.

## Actors and external systems

- The Rust toolchain and Cargo, which decide what a target is and what an integration test may link.
- Clippy under `-D warnings`, whose `non_snake_case` lint constrains the library target's name.
- Implementation agents and developers, who place tests and maintain the public interface.
- No external service, network endpoint, credential, or filesystem location participates.

## Inputs

`Cargo.toml` target declarations; the source files `src/lib.rs`, `src/main.rs`, `src/cli.rs`, and
`src/simulation.rs`; the test files under `tests/`; the commands `cargo build`, `cargo test`, `cargo fmt`, and
`cargo clippy --all-targets --all-features -- -D warnings`.

## Outputs

One library artifact and one executable artifact; one test binary per internal-tier target and one per public-tier
file; the observable program output, which this specification requires to be unchanged.

## State model

This specification governs no runtime state. Its subject is the compile-time arrangement of the crate, which has
two states: conformant, when every rule below holds, and non-conformant, when any does not. There is no partial or
transitional state — a build either satisfies the rules or fails a conformance check.

## Behavioral rules

### 1. Targets

The package name stays `Mokiterions`. It declares exactly two targets:

| Target | Kind | Name | Path |
|---|---|---|---|
| Library | `[lib]` | `mokiterions` | `src/lib.rs` |
| Binary | `[[bin]]` | `Mokiterions` | `src/main.rs` |

No third target, no second package, no workspace, no build script. The dependency and dev-dependency tables stay
empty.

### 2. Target names

The library target is named `mokiterions` rather than inheriting the package name. The declared lint gate is
`cargo clippy --all-targets --all-features -- -D warnings`, which implies `non_snake_case`; a library crate named
`Mokiterions` fails it with `crate 'Mokiterions' should have a snake case name`, and the gate is not to be relaxed.

The binary target keeps the name `Mokiterions`. It is the operator-facing command, it appears in the first line of
`USAGE`, and `REQ-MOK-010`'s observable interface includes that text. The two names differ deliberately, and this
rule is the reason.

### 3. Module ownership

`src/lib.rs` declares `pub mod cli;` and `pub mod simulation;` and defines the process-boundary function of rule 4.
It contains no simulation logic and no test.

`src/main.rs` contains only: the use of the library target, `fn main`, standard-output and standard-error locking
and buffering, one call to the process-boundary function, flush handling that maps a failed flush to exit code `1`,
and the conversion of the returned code to a process exit code. It declares no module and contains no test.

`src/cli.rs` and `src/simulation.rs` keep their current contents apart from the visibility changes rule 5 authorizes
and the test relocations rules 7 to 9 require.

### 4. Process-boundary function

`pub fn execute<I, S, W, E>(args: I, stdout: &mut W, stderr: &mut E) -> u8` moves from `src/main.rs` to
`src/lib.rs`. Its signature, its behavior, its diagnostic text, and its exit codes are unchanged: `0` on success or
help, `1` on output failure, `2` on invalid configuration, with the usage text written to standard error on
invalid configuration. It becomes public because rule 7 places the exit-code tests in the public tier.

### 5. Authorized public interface

The library target's public interface is exactly the union of the two lists below.

**Already public, and unchanged.**

| Item | Form |
|---|---|
| `cli::USAGE` | `&'static str` constant |
| `cli::Command` | enum with variants `Help` and `Run(Config)` |
| `cli::parse` | function returning `Result<Command, String>` |
| `simulation::Config` | struct with public fields `seed`, `tick_limit`, `policy`, `density`, `trace_actions` |
| `simulation::Density` | value type with associated constant `DEFAULT` and function `parse` |
| `simulation::Policy` | enum with variants `Baseline` and `Reference`, with `parse` and `Default` |
| `simulation::RunSummary` | opaque value type; its fields stay private |
| `simulation::Simulation::new` | `Config` in, `Result<Simulation, String>` out |
| `simulation::Simulation::run` | `&mut self` and a writer in, `io::Result<RunSummary>` out |

**Authorized additions.**

| Item | Form | Why it is admissible |
|---|---|---|
| `execute` | rule 4 | Maps arguments and two writers to an exit code; owns no state |
| `simulation::TerminationReason` | enum with variants `TickLimit` and `Extinction`, with `Display` | A reported outcome; required for a public `RunSummary` accessor |
| `RunSummary` accessors | each returns a copy: the termination reason, the tick count, survivors, deaths, population per territory, and resource counts per territory by calorie class | Every value is already printed in the summary line |
| `Density::resources_per_territory` | `self` in, `usize` out | A pure function of a value; the resolved count is already reported |
| `simulation::CELLS_PER_TERRITORY` | `usize` constant | A fixed world dimension, already implied by `SPEC-MOK-001` |

**The additions list is a ceiling, not a checklist.** An item on it that no relocated test requires must not be
added. Nothing outside the two lists becomes public. `Simulation` itself is reachable, but it exposes no public
field and no accessor beyond `new` and `run`.

### 6. Prohibited public interface

None of the following may be public, and none may be reached from a public item by reference, borrow, return value,
public field, trait method, callback, or closure argument:

- the world grid, the agent collection, the resource collection, the tick counter, the entropy state, the event log,
  or any handle to the engine that permits mutation;
- `Mokiterion`, `Food`, `Coordinate`, `Direction`, `RelativeDirection`, `Territory`, `FoodClass`, `Action`,
  `ActionResult`, `Observation`, `PerceivedFood`, `PerceivedMokiterion`, `SplitMix64`, `DecisionEntropy`,
  `DecisionSource` and its implementations;
- observation construction, action application, survival application, regeneration, food counting, decision
  dispatch, the summary constructor, and every simulation constant other than `CELLS_PER_TERRITORY`;
- any `pub(crate)` item widened to `pub` for a reason other than an approved requirement.

No feature flag, `cfg` attribute, self dev-dependency, or conditional-visibility mechanism may make any of the above
reachable from outside the crate, including in test builds. There is no test-support seam.

### 7. Tiers and the placement rule

Every test belongs to exactly one tier, and the tier is determined by the access the test requires:

- if the test can be written using only rule 5's interface, with its assertions unchanged, it belongs to the
  **public tier**;
- otherwise it belongs to the **internal tier**.

A test is not left inline for convenience when rule 5 suffices, and a test is not promoted to the public tier by
widening rule 5. Required access is a property of the test as written; the subject it covers does not decide the
tier.

### 8. Public tier

Located in `tests/`, one file per subject, each compiled as its own integration-test target and reaching the code
as `use mokiterions::…`. The initial arrangement is:

| File | Subject |
|---|---|
| `tests/cli.rs` | argument parsing: defaults, order independence, policy selection, duplicate and missing values, accepted and rejected density forms |
| `tests/process.rs` | the process boundary: help output, invalid configuration, a density resolving to no resources, and output failure, each with its exit code |
| `tests/density.rs` | resolved resources per territory, and the relationship between density, initial endowment, and capacity |
| `tests/termination.rs` | termination by tick limit and by extinction, and the emitted summary |
| `tests/viability.rs` | the population floor at the declared density on the declared seeds |

A further file may be added when a further public subject appears. One file per test is not the arrangement.

### 9. Internal tier

Located in a `#[cfg(test)] mod tests` inside the source file that owns the subject: `src/simulation.rs` for engine
state, observation construction, action validation and application, survival, regeneration, entropy, and both
decision sources; `src/cli.rs` for any parsing detail that rule 5 does not expose. `src/lib.rs` and `src/main.rs`
contain no tests, because rules 3 and 4 leave them with nothing private to assert.

### 10. One invocation

`cargo test` compiles and runs both tiers. Neither tier requires a feature, an environment variable, an
`#[ignore]` attribute, a separate command, or a particular working directory. The number of executed tests is the
same before and after relocation.

### 11. Behavior preservation

The restructuring is equivalence-preserving. For identical arguments and identical seed, the program emits
byte-identical output, reaches an identical final state, and returns an identical exit code, with and without
`--trace-actions`, under both decision sources, at every declared density.

No simulation constant, event field, event order, summary field, exit code, diagnostic message, or byte of `USAGE`
changes. Every case, invariant, and check in `VER-MOK-001` and `VER-MOK-002` remains covered.

### 12. Test content preservation

A relocated test keeps its assertions verbatim. Only the path by which it reaches the code changes — a `use` of the
library target in place of `use super::*`, and public accessors in place of private field reads. A relocated test
whose assertions cannot survive the move is a rule 7 misclassification and stays in the internal tier.

## Error and recovery behavior

- A public-tier file that fails to compile because it names a private item is a rule 7 signal: reclassify the test.
  It is never grounds for a rule 5 addition.
- A clippy failure caused by a target name is corrected under rule 2, never by narrowing the lint invocation or
  adding an `allow` attribute.
- Any observable difference from the pre-change baseline is a defect in the restructuring, not a new baseline.
- If the two lists in rule 5 prove insufficient for a test that rule 7 assigns to the public tier, the correct
  outcome is that the test stays in the internal tier and the insufficiency is recorded, not that rule 5 grows.

## Data and interface contracts

The public interface carries only values: configuration in, and copies of already-reported outcome facts out. No
public item returns a reference into engine-owned state, a mutable borrow, an iterator over engine-owned
collections, a trait object with mutating methods, or a closure holding engine state.

`RunSummary` stays opaque. Its accessors return owned copies, so the summary cannot be used as a window into live
state.

## Security and privacy properties

- No network access, credential read, filesystem access, environment read, or wall-clock read is introduced.
- Making items reachable from outside the crate grants no capability that did not already exist inside it: every
  authorized addition returns a copy of a value the program already prints.
- Rule 6 is the security-relevant rule of this specification. It preserves `REQ-MOK-004` and `ADR-MOK-001`'s
  prohibition on exposing mutable world state, and it holds in test builds as well as release builds.
- The trust boundary is unmoved. A decision source still receives immutable observations and returns typed
  proposals, and nothing in the public interface reaches that boundary.

## Performance and capacity

Runtime behavior is unchanged; rule 11 requires it. Compile-time cost grows: the binary target links the library
target, and each public-tier file becomes an additional test target, so `cargo test` builds more artifacts and
`target/` grows. Per-tick work, memory use, and output volume are unaffected.

## Observability

Unchanged. The event stream, the action-trace lines, the summary line, and the exit codes are exactly as verified.
This specification adds no log, metric, or diagnostic.

## Compatibility and migration

- The public interface becomes a maintained contract. It grows only when an approved requirement needs it to grow,
  and this specification is amended in the same act.
- `ARCH-MOK-001` states one binary crate as a quality attribute and as a conformance check, and prohibits separate
  crates without an approved requirement. This specification cannot be conformed to until the technical owner
  amends it. `ADR-MOK-002` states the required amendments; `WO-MOK-003` makes them an approval precondition.
- `SPEC-MOK-001`'s *Explicitly unspecified decisions* delegates "test organization and helper functions" to the
  implementation agent. Rules 7 to 10 narrow that delegation to file-internal organization and helper structure.
  `SPEC-MOK-001` needs a one-row in-place amendment pointing at this specification; that amendment is the technical
  owner's act and is likewise an approval precondition of `WO-MOK-003`.
- `REPOSITORY_CONTEXT.md` records the test-placement convention already. Its *Architecture* section still names
  `src/main.rs` as the entry point and is updated when this specification is conformed to.
- `WO-MOK-002`'s retained requirement-to-test mapping states that tests live in `src/simulation.rs`, `src/cli.rs`,
  and `src/main.rs`. It is bound to its commit by `VREC-MOK-002` and is not edited. A superseding mapping is
  produced as evidence under the implementing work order.

## Examples and counterexamples

**Example.** The exit-code test for `--density 0.01` needs argument handling, `execute`, and two byte buffers. Rule
7 places it in the public tier; rule 8 places it in `tests/process.rs`; rule 12 keeps all three of its assertions —
the code is `2`, standard output is empty, and standard error contains both `zero resources` and `Usage:`.

**Example.** The density-resolution test asserts that `0.15%`, `0.75%`, and `1.50%` resolve to `12`, `61`, and `122`
resources per territory. It needs `Density::parse` and `Density::resources_per_territory`; the second is an
authorized rule 5 addition, so the test moves to `tests/density.rs`.

**Counterexample.** The survival test that sets an agent's satiety to zero and calls the private survival routine
requires the agent collection. Making `Simulation`'s agent collection public so the test can move to `tests/`
violates rule 6, misapplies rule 7, and contradicts `REQ-MOK-004`. The test stays in `src/simulation.rs`.

**Counterexample.** Adding `#[cfg(feature = "test-support")] pub mod internals` so that all fifty-two tests can
live in `tests/` violates rule 6. Gating does not make exposure conditional in any sense that matters: the feature
is enabled during the build that exposes the state.

**Counterexample.** Replacing the survival test's direct assertion with "a death event appears somewhere in a
1,000-tick run" in order to place it in the public tier violates rule 12. The relocated test is weaker, so the
relocation is a defect rather than a move.

## Explicitly unspecified decisions

- Ordering of items within each source or test file, and whether public-tier helpers are duplicated per file or
  shared through a `tests/common/` module.
- The exact names and signatures of the `RunSummary` accessors, provided each returns an owned copy.
- Where the failing-writer helper used by the output-failure test lives.
- Doc comments, internal comments, and non-authoritative developer notes.
- Whether the internal tier's existing helper functions are kept, renamed, or consolidated, provided no assertion
  changes.
- Whether `src/simulation.rs` is later split into several private modules. Rules 5 and 6 constrain visibility, not
  file count, and no rule here requires or forbids such a split.
