+++
id = "REQ-MOK-017"
type = "requirement"
title = "Place every test by the access it requires"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN the automated test suite is compiled and executed by one cargo test invocation, THE SYSTEM SHALL run every test that requires only the public interface from a test target outside the implementation source files, and every test that requires deliberately non-public state from within the implementation source file that owns that state, with no test excluded, ignored, or gated behind a feature or environment condition."
verification_method = "static-analysis"

[relations]
derives_from = ["CAP-MOK-003"]
+++

# Requirement: Place every test by the access it requires

## Rationale

A public interface that no test uses states nothing that can be relied on, and a rule that says "move tests out of
`src/`" without saying which ones invites the opposite failure: widening the interface until every test fits
through it. Thirty-four of the fifty-two current tests assign agent state directly or call private engine methods,
so a literal relocation of all tests would require publishing authoritative state and would contradict
`REQ-MOK-016` and `REQ-MOK-004`.

The obligation is therefore a placement rule with a stated criterion, and the criterion is the access a test
requires rather than the subject it covers. Access is a property of the test as written, so the rule decides every
case without argument, and it cannot be satisfied by weakening a test until it fits the other tier.

The rule is also what makes `REQ-MOK-016` non-vacuous. Without a populated public tier, an interface can be
declared and then quietly bypassed.

## Preconditions and trigger

The test suite is compiled and executed. The obligation binds the arrangement of the suite, so it holds on every
run and applies to every test that exists, whether relocated by this change or added later.

## Required response

Every test belongs to exactly one of two tiers.

- **Public tier.** Located in test files outside the implementation source files, each compiled as its own test
  target, reaching the code only through the library target's public interface. A test belongs here when it can be
  written using only that interface, with its assertions unchanged.
- **Internal tier.** Located inside the implementation source file that owns its subject, compiled only for test
  builds. A test belongs here when it requires state or behavior the public interface deliberately withholds.

Assignment is determined by required access. A test that can be written against the public interface is not left
inline for convenience, and a test that cannot is not moved by widening the interface.

One `cargo test` invocation compiles and runs both tiers. The total number of executed tests is unchanged by
relocation.

## Failure and boundary behavior

- A test that would pass in the public tier only after an assertion is weakened, removed, or replaced by a coarser
  one belongs to the internal tier. The weakened variant is a defect, not a relocation.
- A test whose subject is public but which additionally reaches one private detail belongs to the internal tier
  until that detail is no longer needed. Splitting it into a public part and an internal part is permitted; dropping
  the internal part is not.
- A test marked `#[ignore]`, gated behind a feature, or dependent on an environment variable satisfies neither
  tier and is a failure of this requirement.
- A relocation that reduces the executed test count is a failure, whether the loss is a deleted test, a test file
  that is never compiled, or a test module that is no longer reachable.
- Helper functions, fixtures, and fake writers follow the tests they serve. A helper is not a reason to place a
  test in the wrong tier.

## Constraints

- The public tier reaches the code only through the interface enumerated in `SPEC-MOK-002`. It names no private
  type, function, module, or field.
- No test-support seam, feature flag, `cfg` attribute, self dev-dependency, or conditional visibility may be
  introduced to make a test relocatable.
- A relocated test keeps its assertions verbatim. Only the path by which it reaches the code changes.
- Every case, invariant, and check in `VER-MOK-001` and `VER-MOK-002` remains covered after relocation, with the
  covering test identified.
- Test placement is repository-visible, so the convention is also recorded in developer guidance; the requirement
  is met by the arrangement of the suite, not by the guidance.

## Acceptance examples

### Example: normal behavior

**Given** the test asserting that an invalid `--density` value is rejected before initialization with exit code `2`

**When** the placement rule is applied

**Then** the test is in the public tier, because it needs only argument handling, the process-boundary function,
and the two output streams, and it is written there with its three assertions unchanged.

### Example: failure behavior

**Given** the test asserting that a Mokiterion's health decreases when its satiety reaches zero, which sets the
agent's attributes directly and then calls the private survival routine

**When** an author moves it to the public tier and, finding it will not compile, replaces its assertions with a
check that some death event eventually appears in a long run's output

**Then** the placement is wrong twice over: the test required non-public state and belonged to the internal tier,
and the substituted assertion no longer verifies the obligation the original covered.

## Open decisions

None. The placement criterion, the two tiers, and the single-invocation obligation were decided by the repository
owner on 2026-08-17. The file-level arrangement within each tier is delegated to `SPEC-MOK-002`, and file-internal
ordering and helper structure are delegated to the implementation agent.
