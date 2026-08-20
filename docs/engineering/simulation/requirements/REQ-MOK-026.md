+++
id = "REQ-MOK-026"
type = "requirement"
title = "Keep the engine component independent of the observer component"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-20"
statement = "WHEN the repository is built or tested, THE SYSTEM SHALL build and test the simulation engine component with no dependency on the observer component, and SHALL confine every user-interface dependency to the observer component."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: Keep the engine component independent of the observer component

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-17 | Original approved content: the engine component builds and tests with **no external dependency** and with no dependency on the observer component, and every user-interface dependency is confined to the observer component. | Approved 2026-08-17 by the repository owner acting as accountable product owner. |
| 2026-08-20 | Struck *"with no external dependency and"* from `statement`, withdrawing the empty-dependency obligation and leaving the two obligations the title names: the engine does not depend on the observer, and user-interface dependencies stay in the observer. The three obligations the zero-dependency property carried — registry independence, auditability and determinism — are **not** withdrawn with it; the *Rationale* paragraph that names them now states that each rests on a check named in `SPEC-MOK-005` rule 8.4. Admission of a crate is governed by `ADR-MOK-006` and declared per package in `SPEC-MOK-002` and `SPEC-MOK-003`; this requirement no longer speaks to it. **This row reaches five clauses the deciding ADR did not enumerate** — one in *Required response*, two in *Failure and boundary behavior*, one in *Constraints* and the two *Acceptance examples* — because each restated the struck obligation, and leaving them would have made an approved requirement assert a rule the same approval withdrew. Nothing about dependency direction, the read-only interface, the one-way graph or the `REQ-MOK-010` relocation clause moves. | Approved 2026-08-20 by the repository owner acting as accountable product owner, by way of `ADR-MOK-006`, whose *Required amendments* section states this amendment. Written under `WO-MOK-013`; the implementation agent wrote the text and did not decide it. The five-clause reach is disclosed here and in `WO-MOK-013`'s completion report rather than assumed. |

## Rationale

`ARCH-MOK-001` states that the simulation engine must not depend on a terminal UI, and until now that has been
guaranteed by there being no terminal UI. Once one exists in the same repository, the guarantee needs an
enforcement mechanism, and prose is not one. Nothing in a single-crate layout stops an engine module from
importing a rendering type, and such an import would not fail any check the repository currently runs.

A component boundary makes the property structural instead of aspirational. If the engine is a component that does
not depend on the observer, the engine cannot import from it — not by oversight, not under time pressure, not in a
refactor. The build refuses.

The zero-dependency property carried three obligations beyond taste, and **all three survive the withdrawal of the
property itself**. It was what let the engine be built and tested in an environment with no package registry access;
what made the claim "no network, no credentials, no async runtime" checkable by inspection rather than by audit; and
the property `ADR-MOK-001` relied on when it asserts that the foundation requires no network and no credentials. Each
of the three is now an obligation with a named check rather than a side effect of an empty table: registry
independence is the offline resolution and test of the engine from the committed lockfile, auditability is the
declared-set comparison and the by-name scan, and `ADR-MOK-001`'s assertion rests on decision 4 of `ADR-MOK-006`,
which preserves every trust prohibition without relaxation. `SPEC-MOK-005` rule 8.4 names all of them, and
`REQ-MOK-047` is the obligation they now sit under. The observer's 57-crate dependency surface is acceptable for the
same reason as before — it can be shown not to reach the engine — and what changed is that the engine's own surface is
now shown to equal what was declared for it instead of shown to be nothing.

Stating the obligation as a requirement is also what makes the component split legitimate. `ARCH-MOK-001`
prohibits separate crates *without an approved requirement*; this is that requirement.

## Preconditions and trigger

The repository is built, tested, or has its dependency graph inspected.

## Required response

- The engine component builds and its tests run with the external dependency set declared for it in `SPEC-MOK-002`
  and no other, resolved from the committed lockfile without registry access.
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

- An added dependency that violates the direction, or an external dependency in either component that is not a
  declared entry of that component's set, is a build-level or check-level failure rather than a review observation.
- A shared dependency required by both components is permitted only as a declared entry of both sets, which is what
  makes it visible rather than duplicated; `SPEC-MOK-004` rule 1 governs where such a crate's version is keyed.
- The engine component remains independently testable with no terminal present, so its tests do not require a
  terminal, a display, or an interactive session.
- The observer component may fail to build or run without affecting whether the engine component builds, tests, or
  runs.

## Constraints

- The component names, their layout, the engine's read-only interface, and the mechanism used to demonstrate the
  dependency direction are fixed by `SPEC-MOK-003` and governed by `ARCH-MOK-002`.
- The engine's external dependency set is exactly the set `SPEC-MOK-002` declares for it. This requirement states no
  preference for few dependencies and sets no ceiling on their number; admission is `ADR-MOK-006`'s test, applied by
  the technical owner per crate.
- Both components share one repository, one version, and one candidate commit. This requirement creates a component
  boundary, not a service boundary, a release boundary, or a separately published artifact.
- No network call, credential, asynchronous runtime, or database is introduced into either component.

## Acceptance examples

### Example: normal behavior

**Given** the repository containing both components

**When** the engine component's dependency graph is resolved and its tests are run in isolation

**Then** the graph contains the engine component and the entries declared for it in `SPEC-MOK-002` and nothing else,
the tests pass with no terminal present, and the observer component does not appear in the graph.

### Example: failure behavior

**Given** a change that adds a rendering dependency to the engine component

**When** the repository is built and checked

**Then** the violation is reported as a failure rather than being accepted, and the obligation breached is identified:
a user-interface dependency outside the observer component, and an entry the engine's declared set does not contain.

## Open decisions

None. Component names and layout, the engine's read-only interface, and the demonstration mechanism are fixed by
`SPEC-MOK-003` and `ARCH-MOK-002`.
