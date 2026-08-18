+++
id = "CAP-MOK-005"
type = "capability"
title = "Find and exercise each package's contract in the package's own directory"
status = "draft"
owners = ["product owner"]
created = "2026-08-18"
updated = "2026-08-18"

[relations]
derives_from = ["INT-MOK-005"]
+++

# Capability: Find and exercise each package's contract in the package's own directory

## Actor and need

A developer or implementation agent needs two things the repository gives for one package and not the other.

**They need to test the observer the way anything outside it would.** Today that is impossible:
`mokiterions-tui` builds one target, a binary, so there is no library target for an integration test to link and
`mokiterions-tui/tests/` would compile against nothing. Every one of the observer's 109 tests therefore has
private access, and no observer test can distinguish a break in what the observer presents from a break in how a
private drawing helper happens to be written. This is the identical need `CAP-MOK-003` served for the engine, in
the package that did not exist when `CAP-MOK-003` was approved.

**They need to find a package's files by knowing which package they want.** The engine's manifest is the
repository root's `Cargo.toml`, which is also the workspace manifest; its sources are `src/` and its tests are
`tests/`, both at the root, beside the observer's own directory. So the actor answering "where does the engine
live" answers it from history, and the actor answering "which package owns `tests/`" cannot answer it from the
path at all.

The same actor needs the opposite guarantee in both packages. A test that must reach a private drawing helper, a
private layout constant, or one of the observer's four test-only hooks must remain able to do so, beside the code
it covers, without the interface being widened to accommodate it.

## Capability statement

`A developer can write and run a test that reaches the observer only through an enumerated public interface, can
keep an observer test that requires internal access beside the code it covers, can find either package's
manifest, sources and tests under a single directory named for that package, and can run every tier of both
packages with one cargo test invocation — while every operator-facing name, command and output byte stays exactly
as verified.`

## Actor-visible behavior

**The observer has a library target.** It declares the package's seven presentation modules, each already
carrying only items that were public before this capability existed. A test in `mokiterions-tui/tests/` reaches
them as `use mokiterions_tui::…`, exactly as an engine test reaches the engine as `use mokiterions::…`.

**Every observer test is placed by the access it requires.** A test that can be written using only the public
interface, with its assertions unchanged and with no item widened, is in the public tier. Every other test is in
the internal tier, beside its subject. The rule decides; the subject does not.

**Each package occupies one directory.** `mokiterions-core/` holds the engine's manifest, its `src/` and its
`tests/`. `mokiterions-tui/` holds the observer's manifest, its `src/` and its `tests/`. The repository root holds
the workspace manifest, which is a workspace manifest and nothing else.

**Nothing an operator touches changes.** The engine package is still `Mokiterions`, its library target still
`mokiterions`, its binary still `Mokiterions`; the observer package and its binary are still `mokiterions-tui`.
`cargo run --bin Mokiterions`, `cargo run -p mokiterions-tui`, `cargo test -p Mokiterions` and
`cargo tree -p Mokiterions` all keep working and keep meaning what they meant. The first line of `USAGE` is
untouched.

**One convention, stated once, true of both packages.** The repository's test-placement rule covers the engine and
the observer, on authority that covers both, so a reader comparing the two packages finds the same structure
rather than one package with two tiers and one with none.

## Boundaries

- The capability extends `CAP-MOK-003` and `CAP-MOK-004` without changing either. Every verified behavior — world
  rules, text output, exit codes, and every frame, key binding, layout tier, glyph and export the observer
  produces — is preserved exactly.
- It adds a library target to the observer package alongside its existing binary target. The binary keeps its
  start-up and its loop; it is not required to become thin.
- It adds no public item to either package. The observer's public interface is exactly the items already written
  `pub` for a cross-module reason, made reachable by the library target; the engine's is unchanged.
- It adds a public test tier to the observer package, and a rule that assigns every observer test to one tier by
  the access the test requires.
- It moves the engine package into its own directory. It renames nothing and it changes no target's kind.
- It excludes any change to observable behavior in either package, any test-support seam, any ungating of the
  observer's four `#[cfg(test)]` hooks, any new package, and any new dependency.
- It excludes relocating observer tests that reach private items or those hooks. Those stay inline by rule, not by
  omission.
- It excludes restructuring the engine's tests. Their tier placement is `WO-MOK-003`'s verified outcome; only the
  directory containing them moves.

## Outcomes

- The observer's promises to the outside are written down and exercised, rather than inferred from which items
  happen to be spelled `pub` for a sibling module.
- A change that breaks what the observer presents, what it exports, how it lays out a viewport, how it maps the
  world to a canvas, or what it reports as the engine's verdict fails a test with no private access to repair
  itself with. `VER-MOK-005`'s cross-cutting suite becomes such a test rather than a block inside the binary.
- Either package's files are found from its name, and the workspace root stops doubling as one package's
  implementation directory.
- The repository's stated test convention and its actual structure agree, closing the disagreement `WO-MOK-005`
  disclosed as its fifteenth finding and `VREC-MOK-005` records as the eleventh item an assurance owner must
  weigh.
- A later phase adding a pane, a decision source, or a second output stream inherits a structure that states
  where its tests go in both packages.

## Dependencies and assumptions

- The observer's cross-module items must already be public, or the library target would require widening and the
  capability would contradict `INT-MOK-005`'s own principle. Measured: 97 public items across the seven modules,
  and every item the cross-cutting suite reaches is among them.
- The four `#[cfg(test)]` hooks must stay `#[cfg(test)]`, so the 16 tests that use them stay in the internal tier.
- Cargo must resolve a uniquely named binary across workspace members for `cargo run --bin Mokiterions` to keep
  working from a virtual workspace root. This is verified rather than assumed.
- The engine package's dependency table must stay empty after the move, and the observer's must stay at one
  direct dependency.
- Equivalence must be demonstrable by comparison against verified output, since a file move produces a diff in
  which every line is new.

## Rationale

`INT-MOK-005` argues that the repository applies one structural standard to the package that was restructured and
no standard to the package that arrived afterwards. A capability that gives the observer a stated interface and
puts each package in its own directory is the smallest thing that makes the two packages comparable. Anything
smaller — documenting the asymmetry, or relaxing the convention to match the code — leaves the observer's whole
verification suite unable to fail for the reason it exists.

The capability is deliberately stated as adding no public item. The engine's interface is a trust boundary, and
`CAP-MOK-003` had to buy its test tier with a short, closed list of read-only additions that `ADR-MOK-002`
justified one at a time. The observer holds no authority, so there is nothing to buy: its items are already
public and merely unreachable, which is the same condition `INT-MOK-003` found in the engine and a weaker one,
because reaching them grants no capability over anything. That is what allows this capability to be added without
touching either the trust boundary `ADR-MOK-001` protects or the determinism `REQ-MOK-025` establishes.

## Candidate requirements

- `REQ-MOK-028` Expose the observer's contract through a library target.
- `REQ-MOK-029` Place every observer test by the access it requires.
- `REQ-MOK-030` Locate each package's manifest, sources and tests under its own directory.
