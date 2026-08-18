+++
id = "INT-MOK-003"
type = "intent"
title = "Give the program a stated public contract that its tests exercise"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
+++

# Intent: Give the program a stated public contract that its tests exercise

## Problem

Every one of the fifty-two automated tests in this repository runs inside the implementation source files, because
the crate has no library target and Rust offers no other place to put a test. Two consequences follow, and neither
is a matter of taste.

First, the program has no stated public contract. `Config`, `Density`, `Policy`, `RunSummary`, `Simulation::new`,
`Simulation::run`, `cli::parse`, `cli::USAGE`, and `cli::Command` are already written `pub`, but nothing outside
the crate can reach them, so no reader can tell which of them is a maintained contract and which is an
implementation detail that happened to be spelled `pub` for a sibling module. The functional roadmap places a
model-provider adapter at the decision boundary in Phase 5 and an additive output stream in Phase 4. Both are
written against a contract this repository has never stated.

Second, no test can fail because the operator-visible contract changed. A test with private access can be repaired
by adjusting the internals it reaches into, so the suite reports on the engine's interior and is silent about the
surface an operator actually touches: argument handling, exit codes, emitted event lines, the run summary,
resolved resource density, termination, and the population floor. The four exit-code tests in `src/main.rs` are
the clearest case — they verify a process contract from inside the process, calling a private function.

The concentration is real as well as structural: forty-three tests share one 2,517-line source file with the
engine they test.

`INT-MOK-001` and `INT-MOK-002` are achieved and verified. Neither covers this outcome. Establishing an
executable foundation and making a foundation habitable are product outcomes; stating what the program promises to
anything outside itself is a distinct engineering outcome with distinct measures, and a completed intent is not
the place to record it.

## Desired outcomes

- The program has an enumerated public interface, and a reader can tell from that enumeration what is promised.
- A tier of tests reaches the program only through that interface, so a change to the operator-visible contract
  breaks a test rather than passing quietly.
- Tests that assert engine internals stay next to the code they cover, because the alternative is publishing
  authoritative state to reach them.
- The public interface exposes no way to read a mutable handle to, or to change, authoritative world state, so
  world authority is exactly as strong after the change as before it.
- A later phase writing a provider adapter or an additional output stream has a contract to write against.
- Approved behavior is untouched. Identical inputs produce byte-identical output, an identical final state, and an
  identical exit code.

## Actors and stakeholders

- The product owner accepts that this initiative changes no product behavior and consumes engineering capacity for
  an engineering outcome.
- The technical owner owns the architecture and specification consequences, including the amendment that permits a
  library target at all.
- Developers and implementation agents place and relocate tests and maintain the public interface.
- Assurance reviewers confirm equivalence by comparison against verified output rather than by re-reading the code.
- Operators are unaffected and must observe no difference of any kind.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Tests reaching the program only through a stated public interface | 0 | ≥ 14 | Automated verification |
| Public interface items yielding mutable or owned authoritative state | 0 | 0 | Every build |
| Automated tests present before and after the change | 52 | 52 | Automated verification |
| Tests requiring a feature flag, environment variable, or ignore attribute to run | 0 | 0 | Every `cargo test` |
| Output bytes differing from the verified baseline at an identical seed and density | 0 | 0 | Automated verification |
| Cases in `VER-MOK-001` and `VER-MOK-002` still covered after relocation | 100% | 100% | Automated verification |
| New runtime dependencies | 0 | 0 | Every build |

## Non-goals

- Any change to simulation behavior, constants, event fields, event order, exit codes, diagnostic wording, or
  `USAGE` text. This initiative is an equivalence-preserving restructuring and nothing else.
- Relocating every test. Tests that assert authoritative state stay where they are; see the second principle below.
- A test-support seam, feature-gated or otherwise, that exposes engine internals outside the crate.
- A second Cargo package, a workspace, a test framework, an assertion library, a snapshot tool, or any other
  dependency.
- A published API-documentation programme, doc-comment coverage target, or semantic-versioning commitment.
- Perception, fear, traits, combat, social behavior, model-provider integration, structured output, persistence,
  or aggregate analysis. This initiative adds no capability to the simulation.

## Principles and immutable constraints

- The simulation engine remains the only authority over world state. A public interface that weakened this would
  defeat the purpose of stating one.
- A test that needs authoritative state is evidence that the test belongs beside the code, never a reason to open
  the boundary. Widening an item to `pub` in order to move a test is prohibited.
- The public interface is a ceiling, not a wish list. It grows only when an approved requirement needs it to grow.
- One command runs everything. `cargo test` covers both tiers, and neither tier is optional or conditional.
- Output bytes are frozen. Equivalence is demonstrated by comparison against verified output, not asserted.
- Keep it small. The change is a target split, a move of one function, a short list of read-only accessors, and a
  relocation of tests that already pass.

## Risks and assumptions

- Fact: the declared lint gate is `cargo clippy --all-targets --all-features -- -D warnings`, which implies
  `non_snake_case`. A library target inheriting the package name `Mokiterions` fails that gate with
  `crate 'Mokiterions' should have a snake case name`. This was reproduced before this intent was written. The
  mitigation is a library target named `mokiterions` alongside a binary target that keeps the name `Mokiterions`,
  which preserves the operator-facing command and the `USAGE` text; that configuration was also reproduced and
  passes the gate clean.
- Fact: thirty-four of the fifty-two tests assign agent state directly or call private engine methods —
  observation construction, action application, survival decay, regeneration. They cannot be relocated without
  either publishing authoritative state or building a test-support seam, and both are excluded above.
- Fact: `ARCH-MOK-001` names one binary crate as a quality attribute and as a conformance check, and prohibits
  separate crates without an approved requirement. A library target is a second crate in the compilation sense, so
  this initiative cannot proceed on the architecture as written.
- Open decision: the technical owner must amend `ARCH-MOK-001`, and decide whether `ADR-MOK-001`'s one-binary-crate
  bullet is amended in place or superseded. `ADR-MOK-002` states the exact amendments required and is the
  instrument for that decision. No implementation may begin before it is settled.
- Risk: relocating a test is an opportunity to weaken it. The mitigation is that a relocated test keeps its
  assertions verbatim; a test that cannot is misclassified and stays inline.
- Risk: the four exit-code tests currently reach a private `execute`. Making it public fixes its shape — an
  argument iterator, two writers, a `u8` return — as a maintained contract, and later phases inherit that shape.
- Risk: an accessor added for a test becomes public surface forever. The mitigation is enumeration in a
  specification and a reviewed inventory rather than incremental additions.
- Assumption: each integration-test file links the library target, so `cargo test` compile time and `target/`
  size grow modestly. Runtime behavior is unaffected.
- Assumption: `WO-MOK-002`'s retained requirement-to-test mapping is bound to its commit and must not be edited.
  A superseding mapping is therefore part of this initiative's evidence rather than an edit to the old record.
