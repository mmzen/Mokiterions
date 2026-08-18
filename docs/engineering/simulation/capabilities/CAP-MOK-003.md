+++
id = "CAP-MOK-003"
type = "capability"
title = "Exercise the program's public contract from outside its implementation"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"

[relations]
derives_from = ["INT-MOK-003"]
+++

# Capability: Exercise the program's public contract from outside its implementation

## Actor and need

A developer or implementation agent needs to write a test that reaches the program the way anything outside it
would: through a stated interface, with no access to the engine's interior. Today that is impossible, because the
crate has no library target, so every test necessarily has private access and no test can distinguish a break in
the operator-visible contract from a break in an internal detail.

The same actor needs the opposite guarantee as well. A test that must reach authoritative state — to place an
agent, to build an observation, to apply an action, to force decay — must remain able to do so without the
interface being widened to accommodate it.

## Capability statement

`A developer can write and run a test that reaches the program only through an enumerated public interface which
exposes no mutable authoritative state, can keep a test that requires authoritative state beside the code it
covers, and can run both tiers with one cargo test invocation.`

## Boundaries

- The capability extends `CAP-MOK-001` and `CAP-MOK-002` without changing either. Every verified behavior — world
  rules, survival, perception, regeneration, both decision sources, density, termination, text output, exit codes
  — is preserved exactly and byte-for-byte.
- It adds a library target and a thin binary target in place of the single binary target, within the one existing
  Cargo package.
- It adds an enumerated public interface, comprising the items already written `pub` plus a short, closed list of
  read-only additions.
- It adds a second test tier located outside the implementation source files, and a rule that assigns every test
  to one tier by the access the test requires.
- It excludes any change to observable behavior, any test-support seam that exposes engine internals, any new
  package or dependency, and any public item that yields mutable or owned authoritative state.
- It excludes relocating tests that assert authoritative state. Those stay inline by rule, not by omission.

## Outcomes

- The program's promises to the outside are written down and are exercised, rather than inferred from which items
  happen to be spelled `pub`.
- A change that breaks argument handling, exit codes, event lines, the run summary, resolved density, termination,
  or the population floor fails a test that has no private access to repair itself with.
- World authority is unchanged, and demonstrably so: the enumerated interface returns copies of facts the program
  already emits and offers no path to mutation.
- Phase 4's additional output stream and Phase 5's provider adapter have a stated contract to be written against,
  settled once rather than negotiated per phase.
- The suite's structure states, for any future test, where it belongs and why.

## Candidate requirements

- `REQ-MOK-016` Expose a read-only public contract.
- `REQ-MOK-017` Place every test by the access it requires.
