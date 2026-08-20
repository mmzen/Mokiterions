+++
id = "REQ-MOK-047"
type = "requirement"
title = "Resolve each package's dependency set equal to its declaration, within the preserved trust envelope"
status = "approved"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN the repository is built, tested, or has its dependency graph inspected, THE SYSTEM SHALL resolve each package's external dependency set equal to the set declared for that package, with each crate at its declared version and its declared feature set, and SHALL contain no crate providing network access, credential handling, an asynchronous runtime, a database, a plugin system or dependency injection in either package, and no user-interface dependency outside the observer package."
verification_method = "static-analysis"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: Resolve each package's dependency set equal to its declaration, within the preserved trust envelope

## Rationale

`ADR-MOK-006` withdrew the engine package's empty-dependency-table rule and admitted third-party crates in both
packages on the repository owner's criteria. The empty table was proving three things by construction — registry
independence, auditability of the trust envelope by reading one table, and determinism — and withdrawing it withdraws
all three proofs at once unless something replaces them. `TRACEABILITY.md` requires a new or revised requirement when
the normative obligation changes; this obligation is **new** rather than a revision of `REQ-MOK-026`'s, because
`REQ-MOK-026` governs dependency *direction* between the two components and this governs what either component is
permitted to contain. `REQ-MOK-026` keeps its own obligation and loses only the clause this one replaces.

**The criteria of `ADR-MOK-006` decision 1 are deliberately not in the `statement`.** The admission test the owner
stated — *"stable, well-maintained functionality that accelerates delivery without introducing excessive dependency
debt"*, and *"proven solutions for standard, non-core features"* — contains three judgements with no observable
threshold, and `ADR-MOK-006` decision 10 declines to invent one. A requirement whose satisfaction cannot be observed
would be a requirement this repository could not verify, and writing *stable* into a `SHALL` would either fabricate a
metric or produce a clause every check must skip.

So the obligation is split, and the split is the point:

- **The criteria govern the act of amending a declared set.** An accountable technical owner applies them to a
  candidate crate, and the amendment row that admits it records that they were applied. That is a decision by a named
  human, recorded where this repository records decisions.
- **This requirement governs what the tree then resolves to.** Every clause of the `statement` is observable from the
  resolved graph: set equality per package, version equality, feature equality, and the absence by name of the
  capability classes `ADR-MOK-001` and `ADR-MOK-006` decision 4 prohibit.

The principle is therefore not quietly weakened into a graph check. It is recorded as what it is — a decision rule for
an owner — with a mechanical consequence that is checkable, and the consequence is this requirement.

## Preconditions and trigger

The repository is built, tested, or has its dependency graph inspected. Inspection includes the pull-request check
`SPEC-MOK-005` requires and the release gate at rule 8.4, and it is not confined to either: the obligation holds at
every commit, not only at the ones a workflow happens to read.

## Required response

1. **Set equality, per package.** For each workspace package, the resolved external dependency set equals the set
   declared for that package — `SPEC-MOK-002` for the engine package, `SPEC-MOK-003` for the observer package — with
   no entry the declaration does not contain and no declared entry absent from the tree.
2. **The set is the resolved graph, not the lockfile.** It is what the package compiles at its declared features on
   the platform being checked. `Cargo.lock` holds crates that resolve into no build, so a check written against it
   would report crates the program never compiles and would miss a feature change that pulls a locked crate into the
   build without editing the lockfile at all.
3. **Version equality.** Each entry resolves to the version its declaration names.
4. **Feature equality.** Each entry resolves with exactly the feature set its declaration names, neither fewer nor
   more, so a feature enabled by unification is a mismatch rather than an unremarked change.
5. **The trust envelope holds by name.** No crate providing network access, credential handling, an asynchronous
   runtime, a database, a plugin system or dependency injection appears in either resolved graph.
6. **User-interface dependencies stay in the observer.** No user-interface crate appears outside the observer
   package's declared set, including one that both declarations would name.
7. **The engine resolves offline.** The engine package resolves, builds and tests from the committed lockfile with no
   registry access and with no terminal attached.
8. **Determinism binds admitted crates.** No entry in the engine package's declared set draws entropy, reads
   wall-clock time, reads the environment, or introduces iteration-order nondeterminism into any value the
   `REQ-MOK-010` stream, the authoritative event sequence or the final state observes. Where a crate offers such a
   capability behind a feature, that feature is off and its absence is part of the declared feature set.
9. **Build-script status matches.** Each declared entry's recorded build-script status matches the resolved graph, so
   a build script appearing in a crate that did not carry one is a mismatch and not an unremarked change.

Responses 1 to 7 and 9 are observable from the resolved graph and a build. Response 8 is observable only in part: a
feature that is off is a graph fact, and what a crate does with the capabilities its features leave on is a review
obligation. `VER-MOK-013` carries that part as a manual assessment rather than claiming a check for it.

## Failure and boundary behavior

- A mismatch in either direction is a check-level failure that names the entries either side of it, not a review
  observation and not a warning.
- A crate reaching the graph transitively is as much a mismatch as one added to a manifest. This is the case the
  obligation exists for: a lockfile update, a caret-range bump or feature unification can change the resolved set with
  no manifest line changing, so the failure must be detectable without a diff.
- A crate in a prohibited capability class is inadmissible however stable or well-maintained it is. The criteria of
  `ADR-MOK-006` decision 1 select *within* the envelope of response 5 and never widen it.
- The by-name scan of response 5 is a named-crate check and is **not** exhaustive over the capability classes it
  names. A crate providing a prohibited capability under a name the scan does not carry passes it, and is caught by
  the review obligation `VER-MOK-013` retains rather than by the scan. This is stated so a passing scan is not read as
  a proof it cannot give.
- Nothing here refuses a graph for its size. `ADR-MOK-006` decision 10 declares no crate-count ceiling, so a package
  whose declaration has grown large satisfies this requirement exactly as one with an empty declaration does.
- No advisory, yanked-version or licence condition is part of this obligation. `ADR-MOK-006` places advisory and
  licence policy expressly out of scope, so a declared crate with a published advisory satisfies this requirement.
  That consequence is recorded rather than hidden.

## Constraints

- Which crates are declared, at which versions, with which features, is fixed by `SPEC-MOK-002` and `SPEC-MOK-003`
  and governed by `ADR-MOK-006`. This requirement declares no crate and admits none.
- Resolution honours the committed lockfile and does not update it, so what is checked is what the repository
  recorded.
- Where the check runs is specified by `SPEC-MOK-005`: on every pull request in a repository-owned workflow, and again
  at the release gate. The managed `.github/workflows/engineering-harness.yml` is not the vehicle for it and is not
  modified.
- A crate shared by both packages is permitted only as a declared entry of both. `SPEC-MOK-004` rule 1 governs where
  its version may be keyed.

## Acceptance examples

### Example: normal behavior

**Given** the repository at a commit whose manifests and lockfile are unchanged from the declarations

**When** each package's dependency graph is resolved at its declared features and compared with its declared set

**Then** both comparisons report no difference in either direction, every entry matches its declared version, feature
set and build-script status, the by-name scan finds no prohibited crate, and the engine resolves, builds and tests
offline from the committed lockfile with no terminal present.

### Example: failure behavior

**Given** a commit that changes no manifest line, and whose lockfile update pulls a crate into the observer package's
resolved graph through a caret range

**When** the comparison runs on the pull request

**Then** the check fails, names the undeclared entry, and the change cannot merge until either the entry is removed or
`SPEC-MOK-003`'s declared set is amended by the technical owner with the criteria of `ADR-MOK-006` decision 1 applied
to that crate and recorded in the amendment row.

## Open decisions

None for this requirement. Two decisions are deliberately left with a named human rather than resolved here, and both
are `ADR-MOK-006`'s: whether a candidate crate meets the criteria of decision 1, which is the technical owner's per
admission, and whether a declared entry implements the proprietary core, which decision 11 prohibits and
`VER-MOK-013` retains as a manual assessment. Neither is an unresolved product decision; both are recurring decisions
this requirement is written not to pretend it can automate.
