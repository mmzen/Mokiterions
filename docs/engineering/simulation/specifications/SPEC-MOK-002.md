+++
id = "SPEC-MOK-002"
type = "specification"
title = "Crate targets, public interface, and test placement"
status = "approved"
owners = ["technical owner"]
created = "2026-08-17"
updated = "2026-08-18"

[relations]
specifies = ["REQ-MOK-016", "REQ-MOK-017"]
+++

# Specification: Crate targets, public interface, and test placement

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-17 | Original approved content for `REQ-MOK-016` and `REQ-MOK-017`. | Approved; implemented under `WO-MOK-003` and verified under `VREC-MOK-003`. |
| 2026-08-18 | Four provisions amended so that the terminal observer of `SPEC-MOK-003` can be conformed to. **Rule 1**: "no second package, no workspace" narrowed to a workspace of exactly two packages, on the approved requirement `REQ-MOK-026` that the clause reserved the exception for. **Rule 3**: the clause freezing `src/cli.rs` and `src/simulation.rs` scoped to the `WO-MOK-003` restructuring it was written for, so that an approved requirement may add code to them. **Rule 5**: the closed enumeration grown by the read-only observation surface, under rule 5's own growth clause. **Rule 6**: the prohibition narrowed from five named value types to the capability it was written to deny. Nothing about mutation, dependency direction, determinism or observable behavior is relaxed, and the engine package's dependency table stays empty. | **OUTSTANDING.** Requires the technical owner. All four are approval preconditions of `WO-MOK-005`, alongside the 2026-08-18 amendment to `ARCH-MOK-001`. None could have been part of the 2026-08-17 approval of the observer chain: this specification was not on that branch when the approval was given, and it reached `master` afterwards. |

## Scope

This is the structural contract for the **simulation engine package**: which targets it builds, exactly which items
its library target makes public, and where every automated test of that package lives.

**Boundary, as amended 2026-08-18.** Every rule below is a rule about the engine package. It was written when the
repository contained only that package, so the scoping narrows nothing: each rule previously bound a repository that
was exactly this package. What the amendment removes is the implication that no other package may exist, an
implication rule 1 already qualified by reserving an exception for an approved requirement. `SPEC-MOK-003` is the
structural and behavioral contract for the terminal observer package, and this specification states nothing about it.

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

Amended 2026-08-18: since rule 1 now admits a workspace, the engine-only form of each command is the one that answers
a question about this package — `cargo build -p Mokiterions`, `cargo test -p Mokiterions`, `cargo tree -p Mokiterions`.
The workspace-wide forms are also run, and the lint gate is not relaxed for either package.

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

No third target and no build script. The dependency and dev-dependency tables stay empty, with no exception,
including a dependency shared with another package in the same workspace.

**Amended 2026-08-18.** This rule read "No third target, no second package, no workspace, no build script." The
repository is a Cargo workspace of exactly two packages: this one at the root, unchanged in package name, in both
target names and in source location, and the terminal observer `mokiterions-tui` as its only other member.
`REQ-MOK-026` is the approved requirement that this rule and `ARCH-MOK-001`'s prohibited-pattern list both reserved
the exception for; `ADR-MOK-003` decides the split and `SPEC-MOK-003` governs the observer package. No third package,
no service, no network boundary and no separate release artifact is admitted. Every other clause of this rule is
unchanged, and the empty dependency table is the check that the split cost this package nothing: `cargo tree -p
Mokiterions` resolves to one crate.

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

`src/cli.rs` and `src/simulation.rs` keep their contents through the restructuring this specification governs, apart
from the visibility changes rule 5 authorizes and the test relocations rules 7 to 9 require.

**Amended 2026-08-18.** This paragraph read "keep their current contents apart from…", which read as a standing rule
forbids ever adding code to either file and so would freeze the engine. It is scoped to the restructuring that
`WO-MOK-003` performed, which is the change rule 11 requires to be equivalence-preserving. An approved requirement may
add code to `src/simulation.rs`; `REQ-MOK-019` through `REQ-MOK-027` are the first to do so, adding the observation
surface rule 5 admits. Rule 11 still binds: whatever is added, the program's observable output, final state and exit
codes for identical inputs do not change. Rules 5 and 6 still bind what any addition may make public.

### 4. Process-boundary function

`pub fn execute<I, S, W, E>(args: I, stdout: &mut W, stderr: &mut E) -> u8` moves from `src/main.rs` to
`src/lib.rs`. Its signature, its behavior, its diagnostic text, and its exit codes are unchanged: `0` on success or
help, `1` on output failure, `2` on invalid configuration, with the usage text written to standard error on
invalid configuration. It becomes public because rule 7 places the exit-code tests in the public tier.

### 5. Authorized public interface

The library target's public interface is exactly the union of the three lists below.

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
added. Nothing outside the three lists becomes public. `Simulation` itself is reachable, but it exposes no public
field and no method beyond `new`, `run` and the observation surface of the third list.

**Observation surface, added 2026-08-18 under `REQ-MOK-019` through `REQ-MOK-027`.**

`SPEC-MOK-003`'s *Data and interface contracts* section is the authority for the content, ordering and ownership of
every item below; this list is the enumeration that closes the interface, and the two must agree.

| Item | Form | Why it is admissible |
|---|---|---|
| `Simulation::snapshot` | `&self` in, `WorldSnapshot` out | Returns an owned tree of copies; borrows nothing and mutates nothing |
| `Simulation::advance_tick` | `&mut self` in, `Result<TickOutcome, String>` out | The single mutating operation; refuses a finished run and consumes entropy exactly as `run` does, because both route through one internal step |
| `Simulation::is_finished` | `&self` in, `bool` out | A copy of a termination fact the summary line already reports |
| `Simulation::termination_reason` | `&self` in, `Option<TerminationReason>` out | Same fact, already public as a `RunSummary` accessor |
| `Simulation::configuration` | `&self` in, `Config` out | A copy of the operator's own input; `Config` is already public |
| `Simulation::initialization_events` | `&self` in, `Vec<Event>` out | An owned copy of events the text stream already emits before tick 1 |
| `simulation::WorldSnapshot` | struct of owned fields, with `[TerritorySnapshot; 2]` and `Vec` of the three snapshot types | The observed state of one completed tick |
| `simulation::TerritorySnapshot` | struct of `Territory` and `usize`/`bool` counts | Every field is already printed in the summary line or derivable from it |
| `simulation::AgentSnapshot` | struct of `String`, `Coordinate`, `Territory`, three `u8` attributes and `Option<Action>` | Every field is already printed in the event stream |
| `simulation::ResourceSnapshot` | struct of `String`, `Coordinate`, `Territory`, `FoodClass` | Every field is already printed in the event stream |
| `simulation::DecisionSnapshot` | struct of `String`, `Action`, `DecisionOutcome`, `Option<Action>` | Exactly what a `--trace-actions` line already prints |
| `simulation::TickOutcome` | struct of `Vec<Event>`, `bool` and `Option<TerminationReason>` | What one tick emitted, owned |
| `simulation::Event` | struct with public `tick`, `subject`, `detail`, and `event_type` | One already-emitted line as a value; not the event log, and no path to it |
| `simulation::EventDetail` | enum with `event_type` | The per-event payload the text stream already formats |
| `simulation::EventType` | enum with `ALL: [Self; 12]` and `as_str` | The closed set of emitted kinds, so a host can filter by kind without parsing text |
| `simulation::DecisionOutcome` | enum carrying a rejection ground | Already printed on a trace line |
| `simulation::RegenerationSkipReason` | enum | Already printed in a regeneration event |
| `simulation::Coordinate` | struct with public `x` and `y`, both `u8` | A position already printed in the event stream; two bytes by value |
| `simulation::Territory` | enum | Already printed in the event stream and the summary line |
| `simulation::Direction` | enum | Reachable only inside `Action`, whose values a trace line already prints |
| `simulation::FoodClass` | enum | The calorie class already printed in the summary line |
| `simulation::Action` | enum | The proposed and applied action a trace line already prints |

Every item is a value, a pure function of a value, or an accessor returning a copy, exactly as `ADR-MOK-002` requires
of an admission. Five of the type names — `Coordinate`, `Territory`, `Direction`, `FoodClass` and `Action` — are named
by rule 6 as it was written; rule 6's 2026-08-18 amendment is what admits them, and it admits them as values only.
`advance_tick` is the only `&mut self` method in the whole interface, and that is checkable by inspection.

### 6. Prohibited public interface

None of the following may be public, and none may be reached from a public item by reference, borrow, public field,
trait method, callback, or closure argument:

- a mutable borrow of, or a reference into, the world grid, the agent collection, the resource collection, the tick
  counter, the entropy state or the event log, and any handle to the engine that permits mutation;
- `Mokiterion`, `Food`, `RelativeDirection`, `ActionResult`, `Observation`, `PerceivedFood`, `PerceivedMokiterion`,
  `SplitMix64`, `DecisionEntropy`, `DecisionSource` and its implementations;
- observation construction, action application, survival application, regeneration, food counting, decision
  dispatch, the summary constructor, and every simulation constant other than `CELLS_PER_TERRITORY`;
- any `pub(crate)` item widened to `pub` for a reason other than an approved requirement.

No feature flag, `cfg` attribute, self dev-dependency, or conditional-visibility mechanism may make any of the above
reachable from outside the crate, including in test builds. There is no test-support seam.

**Amended 2026-08-18, in two places.** The prohibition was written as a list of type names and as a ban on reaching
them "by … return value". Both are narrowed to the capability the rule exists to deny, and nothing else changes.

The first bullet read "the world grid, the agent collection, … the event log, or any handle to the engine that permits
mutation", and "return value" appeared in the list of paths above it. Together they forbade an owned, reference-free
copy of state the program already prints, which is what rule 5's observation surface returns and what
`REQ-MOK-019` through `REQ-MOK-027` require. What is denied is the capability: no public item hands out a borrow of
engine-owned state, a reference into it, an iterator over its collections, an interior-mutable value, or a handle
that permits mutation. A copy grants none of those, and `advance_tick` remains the only mutating operation.

The second bullet named fifteen types. `Coordinate`, `Direction`, `Territory`, `FoodClass` and `Action` are removed
from it, because rule 5's snapshots carry all five by value and cannot be expressed without them: a position, a
territory, a calorie class and an action are the facts being observed. Each is a small `Copy`-or-clone value with no
engine reference, and each is already printed in the event stream or the summary line, so publishing it grants no
capability that did not already exist. The other **ten** names stay in the bullet, stay prohibited and stay private:
`Mokiterion`, `Food`, `RelativeDirection`, `ActionResult`, `Observation`, `PerceivedFood`, `PerceivedMokiterion`,
`SplitMix64`, `DecisionEntropy` and `DecisionSource`. `Observation` and the `DecisionSource` trait are the two that
carry the `ADR-MOK-001` trust boundary, and they are deliberately among the ten: the observation surface is for a
host that watches, not for one that decides.

This rule remains the security-relevant rule of this specification, and `REQ-MOK-004` and `ADR-MOK-001` are preserved
exactly. Narrowing it makes it checkable by a property of the public surface rather than by a list that must be
maintained as types are renamed.

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

Amended 2026-08-18: `cargo test -p Mokiterions` runs both of this package's tiers and nothing else, with no terminal
present and with the observer package excluded from the build. That form is what demonstrates this rule. `cargo test`
at the workspace root additionally runs the observer package's tests, which `SPEC-MOK-003` governs.

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
- If rule 5's lists prove insufficient for a test that rule 7 assigns to the public tier, the correct outcome is that
  the test stays in the internal tier and the insufficiency is recorded, not that rule 5 grows. Rule 5 grows on an
  approved requirement and on nothing else; a test is never that requirement.

## Data and interface contracts

The public interface carries only values: configuration in, and copies of already-reported outcome facts out. No
public item returns a reference into engine-owned state, a mutable borrow, an iterator over engine-owned
collections, a trait object with mutating methods, or a closure holding engine state.

`RunSummary` stays opaque. Its accessors return owned copies, so the summary cannot be used as a window into live
state.

Amended 2026-08-18: the same holds of every snapshot type rule 5's third list admits. Each is a tree of owned values
with no reference into engine state, no shared handle and no interior mutability, so a snapshot is a photograph of one
completed tick and not a window onto the next. `SPEC-MOK-003` states the property and the ordering guarantee that
makes two frames of one tick identical.

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
- Added 2026-08-18: `SPEC-MOK-003` is the second specification the engine package's library target answers to. This
  one owns the target shape, the closed public enumeration and test placement; that one owns the observation surface's
  content, ordering and ownership, and the whole of the observer package. Rule 5's third list is the join between them,
  and the two must agree — if a future amendment to either changes the surface, the other is amended in the same act.
  `WO-MOK-005` implements the four amendments recorded above and `VER-MOK-005` covers them; `WO-MOK-003` and
  `VREC-MOK-003` remain the record of this specification as originally approved, and are not re-opened.

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
