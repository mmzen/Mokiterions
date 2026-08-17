+++
id = "REQ-MOK-023"
type = "requirement"
title = "Keep the engine component independent of the observer component"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN the repository is built or tested, THE SYSTEM SHALL build and test the simulation engine component with no external dependency and with no dependency on the observer component, and SHALL confine every user-interface dependency to the observer component."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-003"]
+++

# Requirement: Keep the engine component independent of the observer component

## Rationale

`ARCH-MOK-001` states that the simulation engine must not depend on a terminal UI, and until now that has been
guaranteed by there being no terminal UI. Once one exists in the same repository, the guarantee needs an
enforcement mechanism, and prose is not one. Nothing in a single-crate layout stops an engine module from
importing a rendering type, and such an import would not fail any check the repository currently runs.

A component boundary makes the property structural instead of aspirational. If the engine is a component that does
not depend on the observer, the engine cannot import from it — not by oversight, not under time pressure, not in a
refactor. The build refuses.

The zero-dependency property is worth preserving for reasons beyond taste. It is what lets the engine be built and
tested in an environment with no package registry access; it is what makes the claim "no network, no credentials,
no async runtime" checkable by inspection rather than by audit; and it is the property `ADR-MOK-001` relies on when
it asserts that the foundation requires no network and no credentials. The observer's 57-crate dependency surface
is acceptable precisely because it can be shown not to reach the engine.

Stating the obligation as a requirement is also what makes the component split legitimate. `ARCH-MOK-001`
prohibits separate crates *without an approved requirement*; this is that requirement.

## Preconditions and trigger

The repository is built, tested, or has its dependency graph inspected.

## Required response

- The engine component builds and its tests run with an empty external dependency set.
- The engine component's dependency graph contains no user-interface, terminal, network, asynchronous-runtime,
  database, or model-provider library.
- The engine component does not depend on the observer component, directly or transitively.
- The observer component depends on the engine component and obtains authoritative state only through the engine's
  declared read-only interface.
- Every user-interface dependency is a dependency of the observer component alone.
- The `REQ-MOK-010` text stream continues to be produced by the engine component, so the verified text behavior does
  not acquire an external dependency by relocation.

The direction of dependency is one-way and checkable per component rather than asserted for the repository as a
whole.

## Failure and boundary behavior

- An added dependency that violates the direction, or any external dependency added to the engine component, is a
  build-level or check-level failure rather than a review observation.
- A shared dependency required by both components is not permitted for the engine component, since the engine's
  external dependency set is empty and admits no exception.
- The engine component remains independently testable with no terminal present, so its tests do not require a
  terminal, a display, or an interactive session.
- The observer component may fail to build or run without affecting whether the engine component builds, tests, or
  runs.

## Constraints

- The component names, their layout, the engine's read-only interface, and the mechanism used to demonstrate the
  dependency direction are fixed by `SPEC-MOK-002` and governed by `ARCH-MOK-002`.
- The engine's external dependency set is empty. This is not a preference for few dependencies.
- Both components share one repository, one version, and one candidate commit. This requirement creates a component
  boundary, not a service boundary, a release boundary, or a separately published artifact.
- No network call, credential, asynchronous runtime, or database is introduced into either component.

## Acceptance examples

### Example: normal behavior

**Given** the repository containing both components

**When** the engine component's dependency graph is resolved and its tests are run in isolation

**Then** the graph contains the engine component alone with no external dependency, the tests pass with no terminal
present, and the observer component does not appear in the graph.

### Example: failure behavior

**Given** a change that adds a rendering dependency to the engine component

**When** the repository is built and checked

**Then** the violation is reported as a failure rather than being accepted, and the engine's empty dependency set is
identified as the obligation breached.

## Open decisions

None. Component names and layout, the engine's read-only interface, and the demonstration mechanism are fixed by
`SPEC-MOK-002` and `ARCH-MOK-002`.
