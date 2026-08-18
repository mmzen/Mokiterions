+++
id = "REQ-MOK-016"
type = "requirement"
title = "Expose a read-only public contract"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN the repository is built, THE SYSTEM SHALL expose its argument handling, run execution, and run outcome through a library target's public interface, and that interface SHALL provide no means to obtain a mutable or owned reference to, or otherwise change, authoritative world, agent, resource, tick, entropy, or event-log state."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-003"]
+++

# Requirement: Expose a read-only public contract

## Rationale

The program currently promises nothing to anything outside itself, because nothing outside itself can reach it. The
items already written `pub` are reachable only by sibling modules, so the crate has no stated contract, and later
phases that must be written against one — an additional output stream, a model-provider adapter — have no fixed
surface to target.

Exposing a contract is only useful if it cannot become a hole in world authority. `REQ-MOK-004` obliges the engine
to prevent any decision source from directly changing world state, and `ADR-MOK-001` prohibits exposing a mutable
world, agent collection, resource collection, event log, or engine handle. A public interface that returned any of
those would satisfy this requirement's first clause while destroying the reason to want it. Both clauses are
therefore normative and neither is severable.

## Preconditions and trigger

The repository is built or its test suite is compiled. The obligation is a property of the compiled targets and
holds on every build, not at a point during a simulation run.

## Required response

The package provides a library target and a binary target. The library target owns the argument-handling module,
the simulation module, and the process-boundary function that maps arguments and output streams to an exit code.
The binary target contains only process startup, stream locking and buffering, the call to that function, flush
handling, and exit-code mapping.

The library target's public interface consists of exactly the items enumerated in `SPEC-MOK-002`, and no others.
Every item in it satisfies both of the following:

- it is a value, a value constructor, a pure function of a value, or an accessor returning a copy of a fact the
  program already emits;
- it offers no path — direct, by reference, by borrow, by trait, by callback, or by public field — to change world,
  agent, resource, tick, entropy, or event-log state, and no path to observe that state other than through the
  facts the program already emits.

Every remaining type, field, function, and constant in the crate stays private to the crate.

## Failure and boundary behavior

- An item required by a relocated test but absent from the enumerated interface is a placement decision to be
  resolved under `REQ-MOK-017`, not a licence to widen the interface.
- The binary target's observable behavior is unchanged by the move: identical arguments produce identical output,
  identical exit codes, and identical diagnostic text.
- A build that cannot satisfy the declared lint gate because of a target name is resolved by naming the target, not
  by relaxing the gate.
- The interface exposes no way to construct a simulation in an arbitrary state. Construction remains validated
  configuration in, engine out.
- Nothing in the interface acquires a network socket, a credential, a filesystem handle, or the wall clock.

## Constraints

- Preserve `REQ-MOK-004`. If a reviewer can name a public item through which a caller changes authoritative state,
  this requirement is unmet regardless of how the item is documented.
- Preserve `REQ-MOK-009`. The restructuring must not move, split, or re-seed the entropy stream.
- Preserve `REQ-MOK-010`, `REQ-MOK-011`, and `REQ-MOK-012`. Event lines, the summary, and action traces are
  byte-identical.
- Add no dependency, no second Cargo package, and no workspace. The dependency table stays empty.
- Introduce no feature flag, `cfg` attribute, or build script that changes which items are public.
- The operator-facing command name stays `Mokiterions`, because it appears in `USAGE` and in operator instructions.

## Acceptance examples

### Example: normal behavior

**Given** the repository built with a library target and a binary target

**When** a test file outside the implementation source files runs the program through the library's public
interface with `--ticks 1 --seed 42`

**Then** it compiles without reaching any private item, receives the exit code and the emitted output, and reads
the run outcome through accessors that return copied values.

### Example: failure behavior

**Given** a test that needs to place a Mokiterion at a chosen coordinate and then build its observation

**When** an author attempts to write that test outside the implementation source files

**Then** it does not compile, because no public item yields the agent collection or the observation constructor,
and the correct resolution is to leave the test beside the code rather than to make either item public.

## Open decisions

The public interface's exact membership is fixed by `SPEC-MOK-002` and is not open. One governance decision is
open and blocks approval: `ARCH-MOK-001` states one binary crate as a quality attribute and as a conformance
check, and prohibits separate crates without an approved requirement, so the technical owner must amend it and
must decide whether `ADR-MOK-001`'s corresponding bullet is amended in place or superseded. `ADR-MOK-002` states
the required amendments. No product decision is open.
