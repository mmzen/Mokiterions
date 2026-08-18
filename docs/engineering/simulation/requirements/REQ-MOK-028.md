+++
id = "REQ-MOK-028"
type = "requirement"
title = "Expose the observer's contract through a library target"
status = "draft"
owners = ["product owner"]
created = "2026-08-18"
updated = "2026-08-18"
statement = "WHEN the observer package is built, THE SYSTEM SHALL build it as a library target and a binary target, SHALL make the library target's public interface exactly the items that were already public for a cross-module reason, and SHALL widen no item's visibility."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-005"]
+++

# Requirement: Expose the observer's contract through a library target

## Rationale

`mokiterions-tui` builds one target, a binary. Nothing outside it can reach its code, so the package has no stated
contract: its 97 public items are public only so that its own modules can call each other, and a reader cannot
tell which of them the package promises to maintain. This is the condition `REQ-MOK-016` corrected for the engine,
in a package that did not exist then.

The concrete cost is that no observer test can be written from outside. `VER-MOK-005`'s cross-cutting suite — the
one that asserts non-perturbation, export fidelity, presented-value fidelity, the authority verdict and colour
independence — is 1,255 lines inside the binary, and a change to the observer's own surface cannot break it. A
test with private access can be repaired by adjusting whatever it reaches into, so the suite reports on the
presentation layer's interior and is silent about what the package promises.

The library target is what makes a public tier possible at all. A Rust integration test links a library target;
with no library target, `mokiterions-tui/tests/` compiles against nothing, and the placement rule `REQ-MOK-029`
states would have one tier to place tests in.

This requirement is deliberately narrower than `REQ-MOK-016`. That requirement had to authorize a short list of
new public items, because the engine's interface is a trust boundary and every addition to it had to be justified.
The observer holds no authority over anything: it cannot change world state, it cannot decide, and it never
reaches the `Observation` and `DecisionSource` types `ADR-MOK-001` protects. So this requirement authorizes no
addition at all. It makes already-public items reachable and it stops there, which is what keeps it consistent
with the prohibition `INT-MOK-003` states and `INT-MOK-005` restates: an item is never widened in order to move a
test.

## Preconditions and trigger

The observer package is built, tested, or has its public interface inspected.

## Required response

- The observer package declares two targets: a library target and a binary target. No third target and no build
  script.
- The library target declares the package's presentation modules and makes them reachable from outside the crate.
- The library target's public interface is exactly the set of items that were public in the observer's non-test
  code before this change. No item is added to it, and no item is removed from it.
- No item's visibility changes. In particular, no private item becomes `pub`, and no `#[cfg(test)]` item becomes
  unconditional.
- The binary target reaches the library target's items through the library, rather than by declaring the modules a
  second time.
- The four `#[cfg(test)]` hooks on `Observer` remain `#[cfg(test)]`, so they are absent from the library target's
  public interface and unavailable to any test outside the crate.
- The observer package's dependency set is unchanged: one path dependency on the engine package and one external
  dependency, with the same version and the same feature set.
- Every frame, export, key binding, layout tier, glyph, diagnostic and exit code the observer produces is
  unchanged.

## Failure and boundary behavior

- An item widened to `pub` so that a test can be relocated is a failure of this requirement, not an
  implementation detail. The correct outcome is that the test stays in the internal tier.
- A `#[cfg(test)]` attribute removed so that a test outside the crate can reach a hook is the test-support seam
  `INT-MOK-005` excludes, and is a failure of this requirement.
- A public-tier test that fails to compile because it names an item the library target does not expose is a
  placement signal under `REQ-MOK-029`. It is never grounds for widening an item.
- The binary target continuing to declare the presentation modules itself, so that the package compiles the same
  code twice, is a failure: the library target would then not be the package's interface but a copy of it.
- A change in the observer's rendered output, its export bytes, or its exit codes is a defect in the
  restructuring rather than a new baseline.

## Constraints

- The observer package and its binary target keep the name `mokiterions-tui`. The library target's name is fixed
  by `SPEC-MOK-004`, in snake case, because the declared lint gate implies `non_snake_case`.
- The observer's binary target is not required to become thin. Its start-up, its loop and its scheduling stay
  where they are, because the tests that cover them reach private items and stay in the internal tier either way.
- The observer's public interface is not a trust boundary and grants no capability over world state. It carries
  presentation types, layout and mapping functions, the event buffer, the observer's own read-only accessors, and
  the export writer.
- The engine package's public interface is unchanged. `SPEC-MOK-002` rule 5 remains closed and this requirement
  adds nothing to it.
- No network access, credential read, asynchronous runtime, database, or model-provider dependency is introduced.

## Acceptance examples

### Example: normal behavior

**Given** the observer package with a library target

**When** a test file under the observer package's `tests/` directory names the package's layout, mapping,
rendering, state, options, authority and export modules and asserts against an in-memory buffer

**Then** it compiles and passes, reaching every item through the library target's public interface, and no item's
visibility differs from what it was before the library target existed.

### Example: failure behavior

**Given** a public-tier test that needs a private drawing helper in the observer's rendering module

**When** the implementation makes that helper `pub` so the test compiles

**Then** the violation is reported as a failure of this requirement, and the required correction is that the test
returns to the internal tier with its assertions unchanged.

## Open decisions

None. The library target's name, the modules it declares, where the cross-cutting internal tests live, and the
inventory that closes the public interface are fixed by `SPEC-MOK-004` and governed by `ARCH-MOK-002` as amended.
