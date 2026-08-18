+++
id = "REQ-MOK-030"
type = "requirement"
title = "Locate each package's manifest, sources and tests under its own directory"
status = "approved"
owners = ["product owner"]
created = "2026-08-18"
updated = "2026-08-18"
statement = "WHEN the repository is inspected or built, THE SYSTEM SHALL place each package's manifest, sources and tests under a single directory named for that package, SHALL keep the repository root's manifest a workspace manifest and nothing else, and SHALL leave every package name, target name, target kind, operator command and output byte unchanged."
verification_method = "static-analysis"

[relations]
derives_from = ["CAP-MOK-005"]
+++

# Requirement: Locate each package's manifest, sources and tests under its own directory

## Rationale

The repository root's `Cargo.toml` is two manifests in one file: the workspace manifest and the engine package's
manifest. The engine's sources are at `src/` and its tests at `tests/`, both at the root, beside `mokiterions-tui/`.
Nothing is broken by this, and it is not an accident: the root was the whole repository when those paths were
written, and `SPEC-MOK-003` deliberately left the engine's sources in place so that the `REQ-MOK-010` text stream
would not move while the observer was being built.

What it costs is legibility, and the cost grows. A reader looking for the engine finds it by knowing that the
repository used to be one crate. A reader asking which package owns `tests/` cannot answer from the path. A
reader comparing the two packages sees one with a directory and one without, and the workspace manifest is not
separable from a package manifest, so a change to workspace-wide settings and a change to the engine's package
settings are edits to the same file.

The asymmetry also blocks the rest of this initiative from being stated cleanly. `REQ-MOK-028` gives the observer a
`src/lib.rs` and `REQ-MOK-029` gives it a `tests/` directory; with the engine still at the root, the repository
would then contain `src/`, `tests/`, `mokiterions-tui/src/` and `mokiterions-tui/tests/`, and the two packages
would be described by two different path conventions in the same specification set.

This requirement is a relocation and nothing else. It is stated separately from the two above because it is
verified differently — by comparing relocated content byte for byte against its source and by re-running the
operator's own commands — and because its risk is different: a move produces a diff in which every line is new, so
the usual review of a diff proves nothing.

## Preconditions and trigger

The repository is inspected, built, tested, or has a package's files located by path.

## Required response

- Each package's manifest, sources and tests are under one directory named for that package. The engine package's
  directory is `mokiterions-core/`; the observer package's remains `mokiterions-tui/`.
- The repository root's `Cargo.toml` is a workspace manifest and declares no package.
- The workspace declares exactly the two members and no third.
- Every package name, target name and target kind is unchanged. The engine package is `Mokiterions`, its library
  target is `mokiterions`, its binary target is `Mokiterions`; the observer package and its binary target are
  `mokiterions-tui`.
- The observer package's dependency on the engine package is by path and continues to be keyed by the engine's
  package name.
- The engine package's dependency and dev-dependency tables stay empty.
- The operator's commands keep working and keep meaning what they meant, including running either binary by name,
  building or testing either package alone, and resolving either package's dependency graph alone.
- Relocated content is unchanged. Each moved source file, each moved test file and each moved test assertion is
  byte-identical to its previous location's content, apart from path references that the move itself requires.
- Both packages' observable behavior is unchanged: identical inputs produce byte-identical output, an identical
  final state and an identical exit code, and the observer presents identical frames and writes identical exports.
- The number of executed tests is unchanged, in each tier of each package.

## Failure and boundary behavior

- A package name, target name, or target kind that changes is a failure of this requirement, not a cosmetic
  consequence of the move.
- An operator command that stops working, or that resolves to a different target, is a failure. A bare build or
  test invocation at a virtual workspace root does not resolve the way it does at a package root, and this is the
  specific risk the requirement exists to catch.
- A moved file whose content differs from its source by anything other than a required path reference is a defect,
  even if every test still passes.
- A dependency that appears in the engine package's manifest as a consequence of the move is a failure against
  `REQ-MOK-026` as well as this one.
- A verified record or a retained evidence file that names an old path is not edited. It is bound to its commit; a
  superseding mapping is produced instead.

## Constraints

- The directory names, the workspace manifest's contents, the path dependency's form, and the set of files that
  move are fixed by `SPEC-MOK-004`.
- The engine's own test tiers keep their placement. This requirement moves the directory that contains them and
  changes nothing about which tier a test is in.
- No file is renamed while it is moved, and no module is split, merged or reordered as part of the move.
- `Cargo.lock` changes only where a path changes. No dependency version resolves differently.
- No new dependency, feature, build script, or workspace-level dependency table is introduced.

## Acceptance examples

### Example: normal behavior

**Given** the restructured repository

**When** the engine's binary is run by name from the repository root, the engine package is built and tested alone,
and its dependency graph is resolved alone

**Then** the binary is the same binary with the same name and the same first line of `USAGE`, the package's tests
pass with no terminal present and in the same number as before, the graph resolves to the engine package alone, and
every one of the engine's source and test files under its new directory is byte-identical to its content at the
previous location.

### Example: failure behavior

**Given** the engine relocated under its own directory

**When** the root manifest retains a `[package]` section, or the observer's path dependency still points at the
repository root

**Then** the violation is reported as a failure of this requirement rather than accepted as a working
configuration, because the root would still be a package directory and the layout would still be asymmetric.

## Open decisions

None. `SPEC-MOK-004` fixes the directory names, the workspace manifest's contents and the relocation's exact
surface.
