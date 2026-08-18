+++
id = "REQ-MOK-029"
type = "requirement"
title = "Place every observer test by the access it requires"
status = "approved"
owners = ["product owner"]
created = "2026-08-18"
updated = "2026-08-18"
statement = "WHEN an observer test is written or relocated, THE SYSTEM SHALL place it in the public tier when it can be written using only the library target's public interface with its assertions unchanged and with no item widened, SHALL place every other observer test in the internal tier beside the code it covers, and SHALL run both tiers under one cargo test invocation."
verification_method = "static-analysis"

[relations]
derives_from = ["CAP-MOK-005"]
+++

# Requirement: Place every observer test by the access it requires

## Rationale

`REQ-MOK-017` establishes this rule for the engine and `SPEC-MOK-002` rules 7 to 10 state it. The observer has no
such rule, and `SPEC-MOK-003` explicitly hands "test organization, fixtures and helpers" to the implementation. The
result is that all 109 observer tests are in one tier while all 60 engine tests are in two, and the repository's
own `REPOSITORY_CONTEXT.md` describes the two-tier convention as though it already applied everywhere.

A library target alone does not fix this. `REQ-MOK-028` makes a public tier possible; without a placement rule,
which tests go there would be decided case by case, and the two failure modes both follow from that. A test left
inline when the interface suffices keeps its private access and so keeps its ability to be repaired against a
changed contract — the exact silence `INT-MOK-003` set out to remove. A test promoted by widening an item buys tier
membership with public surface, which is prohibited.

Stating the rule as required access rather than as a target count is what makes it decidable. "Move the
cross-cutting suite" is a judgement about a file; "can this test be written through the interface with its
assertions unchanged" is a property of the test as written, and a compiler answers it.

The rule must also protect the tests that cannot move. 32 of the observer's tests name a private item of their own
module or one of the four `#[cfg(test)]` hooks on `Observer`. Twelve of those are in the rendering module, which
declares 39 private items and 2 public ones; they assert drawing internals and belong beside the drawing code.
Sixteen use a hook to reach a state a run cannot produce — a rejected proposal, a world with no standing
resources. Under this requirement those stay inline because the rule says so, not because nobody got to them.

## Preconditions and trigger

An observer test is written, relocated, or reviewed for placement; or the observer package's tests are run.

## Required response

- Every observer test belongs to exactly one tier, and the tier is determined by the access the test requires.
- A test that can be written using only the library target's public interface, with its assertions unchanged and
  with no item widened, belongs to the public tier and is located under the observer package's `tests/` directory,
  reaching the code through the library target.
- Every other observer test belongs to the internal tier and is located in a `#[cfg(test)]` module inside the
  crate, beside the code it covers.
- A relocated test keeps its assertions verbatim. Only the path by which it reaches the code changes.
- A test that cannot keep its assertions through the move is a misclassification and stays in the internal tier.
- A test is not left inline for convenience when the public interface suffices, and a test is not promoted to the
  public tier by widening an item.
- `cargo test` compiles and runs both tiers of both packages. Neither tier requires a feature, an environment
  variable, an `#[ignore]` attribute, a separate command, a terminal, or a particular working directory.
- The number of executed observer tests is the same before and after relocation, and so is the number of executed
  engine tests.
- The engine package's existing tier placement is unchanged: its internal tier stays inside its source files and
  its public tier stays in its own `tests/` directory.

## Failure and boundary behavior

- A public-tier file that fails to compile because it names an item the interface does not expose is a
  classification signal: the test belongs to the internal tier. It is never grounds for a `REQ-MOK-028` addition.
- A relocated test whose assertion is weakened, generalized, or replaced by a looser observation is a defect in
  the relocation, even when it passes.
- A test that reaches a `#[cfg(test)]` hook is in the internal tier by definition, because the hook does not exist
  in the build a public-tier test links.
- A test count that differs before and after relocation is a failure regardless of the direction, since a test was
  either lost or silently added.
- An observer test that requires a terminal, a pseudo-terminal, a screenshot, or a recording is not admissible in
  either tier. A rendering claim is asserted against an in-memory character buffer.

## Constraints

- The tier definitions, the location of each tier, the file arrangement of the public tier, and where the
  cross-cutting internal tests live are fixed by `SPEC-MOK-004`.
- No item is widened to `pub` and no `#[cfg(test)]` attribute is removed in order to place a test. This is
  `INT-MOK-005`'s prohibition and it is not subject to a trade-off against tier membership.
- The engine's two-tier placement, verified under `VREC-MOK-003`, is not re-litigated. This requirement governs
  the observer package and leaves the engine's tests where they are.
- Test content is frozen. No assertion changes, in either tier, in either package.
- No test framework, assertion library, snapshot tool, or other dependency is introduced.

## Acceptance examples

### Example: normal behavior

**Given** the observer's cross-cutting verification suite, whose tests reach other modules only through items that
are already public

**When** each of its tests is classified by required access

**Then** those that name no private item and no `#[cfg(test)]` hook are located under the observer package's
`tests/` directory with their assertions verbatim, those that use a hook remain in the internal tier, the two
groups together account for every test in the suite, and the total number of executed observer tests is unchanged.

### Example: failure behavior

**Given** a rendering test that asserts the exact form of a bar row by calling a private helper and reading two
private constants

**When** the implementation relocates it to the public tier by replacing that assertion with a check that the
rendered pane contains a digit

**Then** the relocation is reported as a failure of this requirement, because the assertion was weakened, and the
required correction is that the test returns to the internal tier in its original form.

## Open decisions

None. `SPEC-MOK-004` fixes the tiers, their locations, the public tier's file arrangement, and the treatment of the
four `#[cfg(test)]` hooks.
