+++
id = "ADR-MOK-006"
type = "adr"
title = "Criteria-based admission of third-party crates in both packages, against a declared set, replacing the engine's empty-dependency rule"
status = "approved"
owners = ["technical owner"]
created = "2026-08-20"
updated = "2026-08-20"

[relations]
decides = ["ARCH-MOK-001", "ARCH-MOK-002"]
+++

# ADR: Criteria-based admission of third-party crates in both packages, against a declared set, replacing the engine's empty-dependency rule

## Status

**Accepted 2026-08-20 by the repository owner acting as accountable technical owner.** The instruction, verbatim, in
the turn it was given: *"i approve the ADR, i approve the work order: go implement"*. It answered a stated requirement
that acceptance and a work order be given separately, and it gave both: this decision as technical owner, and
`WO-MOK-013` as engineering owner.

It was drafted by an implementation agent on the repository owner's instruction to relax the third-party-crate policy,
and the agent did not take this decision. `DECISION_RIGHTS.md` reserves acceptance to the technical owner and states
that an implementation agent may not self-approve an ADR it drafted unless separately named as the accountable
technical owner.

**What the acceptance covers is the *Required amendments* section, in full.** Each amendment there is stated
completely, so accepting this one file approves the whole change, following the `ADR-MOK-004` precedent that
`WO-MOK-006` implemented. **Four** of those amendments are not the technical owner's — `REQ-MOK-026`, `REQ-MOK-036` and
the new `REQ-MOK-047` are the product owner's, and `VER-MOK-013` is the assurance owner's. The owner holds all of
those roles and approved them here, by way of this section; each amended artifact records which role approved it and
that it was approved through this ADR rather than separately.

**The acceptance admits no crate.** Decision 8 is part of what was accepted: no crate is added to either manifest by
it, both declared sets land empty of anything not already pinned, and the first admission is a later act under the
amended specifications with the criteria of decision 1 applied to that crate.

`ADR-MOK-001` is **not** superseded and its trust decisions are preserved verbatim below. `ADR-MOK-003` is **not**
superseded either, but two of its statements are reversed rather than narrowed: decision 4 entirely, and the word
*"only"* in decision 5. That is a reversal and this ADR says so rather than calling it a refinement; a dated note in
`ADR-MOK-003`'s *Status* section records exactly which statements move, as `ADR-MOK-003` itself did for
`ADR-MOK-002`.

**Why this is `006` and not `005`.** `ADR-MOK-005` is taken by a draft on `origin/feature/phase-4a-definition`,
created the same day by another agent, which has also claimed `INT-MOK-009`, `CAP-MOK-005` to `CAP-MOK-009`,
`REQ-MOK-042` to `REQ-MOK-046`, `SPEC-MOK-006`, `VER-MOK-012` and `WO-MOK-012`. Every identifier this change uses —
`ADR-MOK-006`, `REQ-MOK-047`, `VER-MOK-013` and `WO-MOK-013` — was free across every ref in `refs/heads` and
`refs/remotes` when this file was written and again when the pack was implemented, checked by enumerating refs rather
than by reading one working tree. They are free-when-written, which is all any identifier can be in a shared space:
if that branch renumbers into them, the collision is resolved by whichever chain has not yet landed.

**Implementation note dated 2026-08-20, recorded by the implementation agent. One figure in this ADR is wrong, and no
decision changes.** Decision 13's supporting text, the `SPEC-MOK-003` amendment row and the *Positive* consequence about
the build-time surface each say **eight** crates carry a build script and name them, `syn` among them. The
re-measurement this ADR's own `SPEC-MOK-003` row requires — *"the figures are re-measured when the amendment is written
rather than copied from this ADR"* — finds **seven**, and `syn` is the one that is not there. `syn 1.0.109` does carry a
build script and is in `Cargo.lock`, but it is unreachable from either workspace package at any edge kind, including
`--target all`, and the two `syn` versions the observer resolves, `2.0.119` and `3.0.3`, carry none. `thiserror 1.0.69`
sits in the lockfile on the same footing, while the reachable `thiserror 2.0.20` does carry one and is in the table.
Where the eight came from cannot be reconstructed, and this note said "a lockfile-wide reading" before the counting was
finished: `Cargo.lock` holds **29** packages carrying a build script, the observer reaches **12** of them at
`--target all`, **10** across the three targets `SPEC-MOK-005` rule 10 builds, and **7**, **9** and **9** on those
targets taken one at a time. Eight is none of the four. The seven come from the resolved graph on one target, which is
what decision 7 and `REQ-MOK-047` require and what a build actually executes. It is the same distinction the amended
`SPEC-MOK-005` rule 8.4 now has to state in order to be checkable, and it is why this ADR asked for re-measurement
instead of copying. `evidence/WO-MOK-013/WO-MOK-013-build-scripts.txt` holds all four counts with the crates named
under each, including the 17 scripted lockfile packages no build of either package reaches.

The implemented figure is **seven**, in `SPEC-MOK-003`'s *Declared dependency set*, which records this deviation and its
cause. This note changes no decision, no option, no consequence and no driver, and it ratifies nothing: it is a
statement of fact about a measurement, of the kind `SPEC-MOK-003`'s 2026-08-19 reconciliation row is. Decision 13 stands
exactly as accepted — build scripts are admissible and each declared entry discloses whether it carries one — and the
argument the wrong figure supported is unaffected, since it turns on already-pinned crates carrying build scripts at
all, not on how many.

**Note dated 2026-08-20. One reference in this ADR is corrected, and no decision changes.** Approved by the repository
owner acting as accountable technical owner, on the same day and in the same session as the acceptance above. The first
*Negative* consequence spoke of *"`VREC-MOK-002`'s four replay hashes"*. That record holds **no replay hash** — its only
SHA-256 is its own `artifact_snapshot_sha256` — and the four hashes are in
`evidence/WO-MOK-002/determinism-and-resilience.md` at lines 12 to 15, which `VREC-MOK-002` binds through its
`evidence_paths`. The consequence now reads *"the four replay hashes retained under `VREC-MOK-002` in
`evidence/WO-MOK-002/determinism-and-resilience.md`"*. The three other places in this ADR that refer to those hashes —
*What the empty table is doing, beyond taste* item 3 at line 158, the `VER-MOK-013` amendment entry at line 514, and the
*Validation* list at line 660 — already said *"retained under"* and are unchanged. `VREC-MOK-002` itself is **not edited**, and neither is the evidence it binds: both were correct at the
commit that record names. **This changes no decision, no option, no driver and no consequence in substance**: the
consequence it corrects is that verified behavior is now downstream of a dependency choice, which is as true of hashes
held in bound evidence as of hashes held in a record. What is corrected is where a reader is sent to find them, which
matters because `VER-MOK-013` oracle 3 substitutes a newer baseline for these four and a reader checking that
substitution needs the file that holds them. `VER-MOK-013` carries the same correction in a dated amendment row, and
`WO-MOK-013` in a dated *Lifecycle* note. It was found while measuring, and it is recorded rather than fixed silently
because a reference an implementation quietly repointed is indistinguishable from one that was always right.

## Context

The instruction, verbatim, in the turn it was given:

> the current repository policy forbids additional third-part crates for the core crates (verify this statement),
> this needs to be relaxed: the new policy is that third-party crates are accepted as long as they comply with:
> "Choose third-party components when they provide stable, well-maintained functionality that accelerates delivery
> without introducing excessive dependency debt. Prioritize proven solutions for standard, non-core features so
> development teams can focus engineering effort entirely on building proprietary business value"

Six questions were put to the owner in the same session and answered. Each answer is the owner's and each is
load-bearing for a decision below; nothing else about scope was stated and nothing else is inferred.

| Question | Answer | Where it lands |
|---|---|---|
| Which packages does the policy govern? | **both workspace packages** | decision 2 |
| Do the separately-motivated prohibitions survive? | **all of them survive** — no network, no API credentials, no asynchronous runtime, no database, user-interface dependencies confined to the observer, determinism preserved | decision 4 |
| Does *excessive dependency debt* get a mechanical threshold? | **no numeric bar**; the owner applies the criteria per admission | decision 10 |
| Does *"standard, non-core features"* become an enforceable boundary? | **yes, a prohibition** | decision 11 |
| Does the check run at pull-request time? | **yes, in a repository-owned workflow** | decision 12 |
| How are build scripts in admitted crates treated? | **admissible, disclosed per crate** | decision 13 |

### The statement was verified before being relaxed

For the **engine package** (`mokiterions-core`, package `Mokiterions`) the statement holds and is stronger than
"forbids additional": the table must be *empty*, dev-dependencies included, with no exception. It is stated at six
levels of authority, which is why relaxing it is not an edit to one sentence:

| Authority | Where | What it says |
|---|---|---|
| Requirement, `approved` | `REQ-MOK-026` `statement` | build and test the engine component *"with no external dependency"* |
| ADR, `approved` | `ADR-MOK-003` decision 4 | *"empty, with no exception, including a dependency shared with the observer"* |
| Architecture | `ARCH-MOK-001` prohibited patterns; `ARCH-MOK-002` prohibited patterns | *"admits no exception"*; *"any external dependency in the engine package"* |
| Specification | `SPEC-MOK-002` rule 1 | *"The dependency and dev-dependency tables stay empty, with no exception"* |
| Release gate | `SPEC-MOK-005` rule 8.4 | the engine's tree *"resolves to exactly one crate"* |
| Requirement, release refusal | `REQ-MOK-036` | *"Given commit `C` in which the engine package's dependency table is no longer empty… the process refuses, naming the boundary"* |

`REPOSITORY_CONTEXT.md`, both manifests' comments and `mokiterions-core/src/lib.rs`'s module documentation restate it
without adding authority, and `.github/workflows/release.yml` enforces it by asserting that
`cargo tree -p Mokiterions --locked` prints exactly one line.

For the **observer package** the statement does not hold as written. There is no ban: `ratatui 0.30.2` with
`default-features = false` and features `crossterm`, `layout-cache`, `underline-color` is admitted and *fixed* by
`SPEC-MOK-003` and decided by `ADR-MOK-003` decision 5, and a further crate is gated rather than prohibited —
`REPOSITORY_CONTEXT.md` states that *"Adding any other dependency to either package requires escalation and an
updated architectural decision."* The asymmetry is exactly the word "only" in decision 5 against "no exception" in
decision 4: the observer has an escalation path and the engine does not.

**One measured detail changes where a relaxation bites.** The dependency check runs only in `release.yml`.
`engineering-harness.yml`, which provides the `candidate` and `governor` checks on every pull request, runs the
governor, preflight and the dashboard and invokes `cargo` nowhere. An engine dependency added today would pass pull-
request CI and be refused at release, by `REQ-MOK-036`'s scenario. Whatever replaces the rule inherits that
placement unless the amendment moves it.

### What the empty table is doing, beyond taste

`REQ-MOK-026`'s rationale and `ADR-MOK-003`'s context make three obligations rest on it, and the new policy speaks to
none of them. It addresses dependency debt and delivery speed; these are trust and reproducibility:

1. **Registry independence.** The engine builds and tests in an environment with no package registry access.
2. **Auditability by construction.** *"No network, no credentials, no async runtime"* is checkable by reading one
   table rather than by auditing a graph — the property `ADR-MOK-001` relies on when it asserts that the foundation
   requires no network and no credentials.
3. **Determinism.** `REQ-MOK-009` makes identical seed and configuration produce identical runs. Four replay hashes
   retained under `VREC-MOK-002` and the verified `REQ-MOK-010` text stream sit downstream of it, and
   `release.yml` compares two runs per policy byte for byte. Today no engine crate can draw entropy, read the clock
   or iterate a hash map in an unstable order, because there are no engine crates.

Withdrawing the rule withdraws all three proofs at once. The decision below keeps all three obligations and gives
each its own enforcement, because that is what the owner's second answer requires and what this repository's evidence
standard requires: a property that used to be a fact about a manifest becomes a check, and a check has to be named.

## Decision drivers

- Adopt the owner's stated principle as the admission test, in the owner's words, without paraphrase.
- Keep every property the empty table currently proves by construction, and replace each construction proof with a
  named check rather than with an assertion.
- Keep the engine independently buildable and testable without the observer and without registry access.
- Prefer mechanically checkable conditions; where a condition is a judgement, name the accountable human instead of
  pretending it is a gate.
- Leave `REQ-MOK-026`'s other half — the engine does not depend on the observer, and user-interface dependencies stay
  in the observer — untouched. Dependency *direction* is not in scope.
- Leave verified behavior untouched: the `REQ-MOK-010` stream, `REQ-MOK-009` determinism, and the records that bind
  them.
- Make the release gate say what the policy says. As written it refuses any engine dependency, so a relaxation that
  stops at the architecture leaves the repository unable to release the code it now permits.
- Catch the property where it can drift without a diff showing it, rather than only on release day.
- Reserve the simulation itself to this project. A policy that buys standard functionality should not be able to buy
  the thing the project exists to build.
- Accept the smallest surface that delivers what was asked, and keep the accepted surface enumerated.

## Considered options

### Option 1: Withdraw the rule and admit crates on the criteria alone

Delete the empty-table provisions, state the principle where they were, and let each admission be argued in the pull
request that makes it.

Cheapest to write and the closest literal reading of the instruction. It is rejected because it silently withdraws the
three obligations above with no successor: after it, *"the engine has no network and no credentials"* is an assertion
about a graph nobody enumerates, registry independence is untested, and determinism depends on a reviewer noticing
that a crate reads the clock. It also fails this repository's own standard, in which a claim that only a human has
checked is recorded as a human's decision or not recorded at all.

### Option 2: Criteria-based admission against a declared set, with a mechanical envelope

Admit third-party crates in both packages on the owner's criteria, and keep the shape the observer's one dependency
already has: every admitted crate named in a specification with its exact version and feature set, so that admission
is a specification amendment approved by the technical owner and never an implementation choice. The mechanical check
changes from *"the set is empty"* to *"the resolved set equals the declared set"*, which is strictly more
informative and no weaker. Around it sits an envelope of conditions that are checkable without judgement: the
preserved prohibitions, offline resolution from the committed lockfile, and — new, and the one genuinely new
obligation the relaxation creates — that no engine crate may draw entropy, read wall-clock time, read the
environment, or introduce iteration-order nondeterminism into anything the `REQ-MOK-010` stream observes.

It delivers the instruction, it keeps all three obligations, and it costs an amendment per admission. That cost is
real and is weighed in *Negative* below: the principle promises acceleration, and an amendment is friction.

### Option 3: Relax dev-dependencies only

Admit test-only crates in the engine and keep the runtime table empty. Narrow, cheap, and it would leave every
runtime property provable exactly as it is today. Rejected because it does not deliver what was asked, and because
the instruction is not about tests.

### Option 4: Keep the ban and record the principle as non-normative guidance

The honest do-nothing option, and a real one rather than a strawman: the principle would inform future decisions
without changing any provision, and the repository would keep its strongest structural property. It is rejected
because the owner instructed the change, and it is recorded because an option that was genuinely available should be
visible in the record.

### Option 5: Admit crates and vendor them into the repository

Preserves registry independence perfectly and makes the supply chain reviewable in-tree. Rejected as contrary to the
instruction it would serve: vendoring is the opposite of *"without introducing excessive dependency debt"* in review
terms, since every crate's source becomes this repository's to read, and it does not accelerate delivery.

## Decision

Adopt option 2.

1. **Withdraw the engine package's empty-dependency-table rule.** Third-party crates are admissible in both
   packages. The admission test is the owner's, quoted normatively and not paraphrased:

   > Choose third-party components when they provide stable, well-maintained functionality that accelerates delivery
   > without introducing excessive dependency debt. Prioritize proven solutions for standard, non-core features so
   > development teams can focus engineering effort entirely on building proprietary business value

2. **One policy for both packages.** The observer's *"escalation and an updated architectural decision"* gate and the
   engine's absolute prohibition are replaced by the same test. `ratatui`'s pin is unaffected: it is already a
   declared-set entry and stays one.
3. **Every admitted crate is declared.** A specification names each crate, its exact version and its exact feature
   set, per package. Admitting a crate, changing a version and changing a feature set are each a specification
   amendment approved by the technical owner, and none of them is an implementation choice. This generalizes the rule
   `ADR-MOK-003` already applied to `ratatui` rather than inventing a mechanism.
4. **These prohibitions are preserved without relaxation**, on `ADR-MOK-001`'s authority and the owner's second
   answer. No network calls. No API credentials, and no secret in the repository. No asynchronous runtime. No
   database. No plugin system and no dependency-injection container. Every user-interface dependency in the observer
   package and nowhere else, including one shared by both. No dependency edge from the engine package to the observer
   package. A crate that provides any of these is inadmissible under this policy regardless of how stable or
   well-maintained it is, so the criteria of item 1 select *within* this envelope and never widen it.
5. **The engine stays independently buildable and testable**, without the observer, without a terminal, and from the
   committed lockfile without registry access. This is a check after this decision rather than a fact, and item 7
   names it.
6. **Determinism binds every engine crate.** No crate admitted to the engine package may draw entropy, read
   wall-clock time, read the environment, or introduce iteration-order nondeterminism into any value the
   `REQ-MOK-010` stream, the authoritative event sequence or the final state observes. Where a crate offers such a
   capability behind a feature, the feature is off and its absence is part of the declared set.
7. **The mechanical check becomes an enumeration.** `cargo tree` per package no longer asserts a count; it is
   compared against the declared set, and a resolved set that differs from the declaration in either direction fails.
   Alongside it: an offline resolution and test of the engine package from the committed lockfile, a feature audit
   against the declaration, and a scan confirming no network, asynchronous-runtime, database or model-provider crate
   appears in either resolved graph.

   **The set is the resolved per-package graph at the declared features, not the lockfile.** Measured on 2026-08-20 in
   this checkout at toolchain `1.97.1`: `cargo metadata --locked` reports **182** packages, while
   `cargo tree -p mokiterions-tui -e normal --locked` resolves **59** — the 57 of `SPEC-MOK-003` plus both workspace
   packages. `serde` and `getrandom` are among the 123 that appear in `Cargo.lock` and resolve into no build, because
   the features that would pull them in are off. A check written against `Cargo.lock` would therefore fail on crates
   the program never compiles, and would miss a feature change that pulls a locked crate into the build without
   editing the lockfile at all. The declared set is compared against the resolved graph, per package, with the
   declared features, on each supported platform.
8. **Nothing is admitted by this decision.** No crate is added to either manifest by it, and no candidate crate is
   named in it. The first admission is a separate act under the amended specification, and the criteria of item 1 are
   applied then, by the technical owner, on that crate.
9. **The implementation may not choose** which crate, which version, which features, or whether a candidate meets the
   criteria.
10. **No numeric threshold is adopted for dependency debt.** *Excessive* is the owner's judgement per admission, and
    the amendment row that admits a crate records that the criteria of item 1 were applied to it. No per-package
    crate-count ceiling is declared, because any number this repository picked today would be reverse-engineered from
    the observer's 57 and would then refuse or admit the next candidate for a reason unconnected to it. The cost is
    stated in *Negative*: nothing refuses a bloated graph automatically, and every admission spends an accountable
    decision.
11. **Nothing admitted may implement the proprietary core.** No crate in either package's declared set may implement
    simulation semantics — the rules of `SPEC-MOK-001`, the world model, agent decision-making — own or advance
    entropy, or perform action validation. This is the second sentence of item 1's principle made enforceable rather
    than left inside the judgement: *standard, non-core* functionality is what a crate may supply, and the
    simulation is what this project builds. It binds the observer as well as the engine, where it strengthens
    `ARCH-MOK-002`'s existing prohibition on re-deriving the engine's validation verdict: an observer crate may not
    do it either. It is a review obligation, not a graph read, and item 7's checks cannot see it — `VER-MOK-013`
    carries it as a manual assessment, which is how this repository already handles a claim no script can settle.
12. **The check runs at pull-request time as well as at release**, in a **repository-owned** workflow, because the
    property this decision protects stops being visible in a diff: a resolved set can drift through a lockfile
    update, a caret-range bump or feature unification with no manifest line changing. The managed
    `.github/workflows/engineering-harness.yml` is **not** edited — it is SHA-256 locked in
    `.engineering-harness.lock` and integrity-checked by `python -m se_harness doctor`, and editing it would break
    managed integrity that `ENGINEERING_HARNESS.md` does not permit waiving. The new workflow carries item 7's checks
    and nothing else: it does not take on `cargo fmt`, `cargo clippy` or the test suite, which this repository
    deliberately captures as retained evidence at the candidate commit rather than in pull-request CI. Changing that
    is a different decision and this one does not make it.
13. **Build scripts are admissible, and disclosed per crate.** The declared set records, per entry, whether the crate
    carries a build script, so the build-time code-execution surface is enumerated rather than discovered. Prohibiting
    them is not available: measured on 2026-08-20, **8 of the observer's 59 resolved crates already ship one** —
    `instability`, `parking_lot_core`, `proc-macro2`, `quote`, `rustversion`, `syn`, `thiserror` and `winapi` — so the
    prohibition would retroactively eject a pin the technical owner has already approved. Those eight are recorded
    when the amendment is written, which is the first time this repository has written down that it executes
    third-party code at build time.

## Required amendments

Each is stated in full so that accepting this ADR accepts the change. **Four of them are not the technical owner's**:
`REQ-MOK-026`, `REQ-MOK-036` and the new `REQ-MOK-047` are the product owner's acts, and the verification contract is
the assurance owner's. `WO-MOK-013` makes every row below an approval precondition, as `WO-MOK-005` and `WO-MOK-006`
did for their ADRs.

### `REQ-MOK-026` — product owner

- `statement`: remove *"with no external dependency and"*, leaving the obligation to build and test the engine
  component with no dependency on the observer component and to confine every user-interface dependency to the
  observer component. The title, *"Keep the engine component independent of the observer component"*, stays exactly
  right afterwards; the clause being removed was never what the title named.
- *Rationale*: the paragraph beginning *"The zero-dependency property is worth preserving for reasons beyond taste"*
  names the three obligations. It is not deleted. It is amended to state that all three survive and that each now
  rests on a check named in `SPEC-MOK-005` rather than on an empty table, with the sentence about `ADR-MOK-001`
  redirected to decision 4 of this ADR.
- Amendment record row, in the `REQ-MOK-014` form, whose *Approval* cell names the product owner and this ADR.

### New requirement, `REQ-MOK-047` — product owner

The policy is an obligation over future builds, and requirements are the only artifacts that carry those.
`TRACEABILITY.md` requires a revised or new requirement when the normative obligation changes, and this obligation is
new rather than a revision of `REQ-MOK-026`'s.

- `statement`, in `SHALL` form: WHEN the repository is built, tested or has its dependency graph inspected, THE
  SYSTEM SHALL resolve each package's external dependency set equal to the set declared for that package, with each
  crate at its declared version and its declared feature set, and SHALL contain no crate providing network access,
  credential handling, an asynchronous runtime, a database, a plugin system or dependency injection in either
  package, and no user-interface dependency outside the observer package.
- `verification_method = "static-analysis"`, the value the repository already uses for graph and interface
  obligations.
- `derives_from`: the capability under which the workspace's structural obligations already sit, `CAP-MOK-004`, which
  is `REQ-MOK-026`'s.
- **The criteria of decision 1 are deliberately not in the `statement`.** *Stable*, *well-maintained* and *excessive
  dependency debt* are judgements, and a requirement whose satisfaction cannot be observed would be a requirement
  this repository could not verify. The criteria govern the act of amending the declared set, where an accountable
  human applies them and the amendment row records that they were applied. The requirement governs what the tree then
  resolves to. The *Rationale* section states this split in those terms, so the principle is not quietly weakened
  into a graph check: it is recorded as what it is, a decision rule for an owner, with a mechanical consequence that
  is checkable.

### `REQ-MOK-036` — product owner

- The acceptance scenario *"A dependency added to the engine"* currently refuses a release at any non-empty engine
  table. Replace the given clause with a resolved set that differs from the declared set — in either direction, since
  a declared crate that has vanished from the tree is as much a mismatch as an undeclared one — and keep the refusal,
  the naming of the boundary and the scenario's position in the list. The release still refuses; what it refuses
  changes from *any* engine dependency to an *undeclared* one.

### `ARCH-MOK-001` — technical owner

- *Prohibited patterns*, the entry beginning *"Network calls, API credentials, asynchronous runtimes, databases, UI
  frameworks, plugin systems, or dependency injection containers in the engine package."* The first sentence stays
  verbatim — decision 4 preserves every item in it. The second sentence, *"The engine package's external dependency
  set is empty and admits no exception, including a dependency shared with another package in the same workspace"*,
  is replaced by: the engine package's external dependency set is exactly the set declared for it, admission is
  governed by `ADR-MOK-006`, and a dependency shared with another package in the same workspace is admissible only
  as a declared entry of both.
- *Prohibited patterns*, third entry: *"Wall-clock time, operating-system randomness, unordered iteration, or thread
  scheduling affecting results"* stays as written and is extended to state that it binds admitted crates and not only
  first-party code, which is decision 6.
- *Prohibited patterns*, **a new entry** for decision 11: no crate in the engine package's declared set implements
  simulation semantics — the rules `SPEC-MOK-001` fixes, the world model, or agent decision-making — owns or advances
  entropy, or performs action validation. A crate supplies standard functionality the simulation uses; it does not
  supply the simulation. This entry sits beside the existing prohibition on separate packages and services, which
  reserves structure to approved requirements, because it reserves *substance* the same way.
- *Conformance checks*, the entry *"Confirm the engine package's dependency graph is empty, and therefore contains no
  network, async-runtime, database, or UI libraries."* The inference no longer holds — an empty graph implied the rest
  and a declared graph does not — so the check splits in two: confirm the resolved graph equals the declared set at
  the declared versions and features, and confirm by name that it contains no network, asynchronous-runtime,
  database, model-provider or user-interface crate. The per-package resolution sentence stays.
- *Conformance checks*, the entry beginning *"Confirm the engine builds as one Cargo package with exactly one library
  target and one binary target and an empty dependency table"*: strike *"and an empty dependency table"*, keep the
  target shape and the independence of the observer and of a terminal, and add resolution from the committed
  lockfile without registry access, which is decision 5.
- *Conformance checks*, **a new entry** for decision 11: review each admitted crate against what it supplies, and
  confirm that no declared entry implements simulation semantics, owns entropy, or validates an action. It is stated
  as a review because no graph read answers it, and `VER-MOK-013` is where the assessment is retained.
- *Quality attributes*: this list does not mention the dependency set at all today, because nothing threatened it.
  Add registry independence and the declared-set property, so that the properties decisions 5 and 7 protect are
  attributes of the architecture rather than only checks under it.
- `decision_assessment.rationale`: record that the repository's third-party-component policy is decided by this ADR,
  that it fires the already-declared technology-framework-vendor-or-external-service and material-alternatives
  triggers, that it leaves engine authority, the dependency direction, the trust boundary and determinism untouched,
  and that the assessment stays `adr_required` and is now covered by `ADR-MOK-001`, `ADR-MOK-002`, `ADR-MOK-003` and
  `ADR-MOK-006` together.
- Amendment record row. **The 2026-08-18 row above it stays OUTSTANDING and is not touched**; it is `WO-MOK-005`'s
  precondition and this change neither clears nor inherits it.

### `ARCH-MOK-002` — technical owner

- *Prohibited patterns*, first entry: *"Any dependency edge from the engine package to the observer package, or any
  external dependency in the engine package."* The first clause stays. The second is replaced by an undeclared
  external dependency in either package.
- *Prohibited patterns*, second entry, *"A user-interface dependency anywhere but the observer package, including a
  dependency shared by both"*: unchanged, and marked in the amendment row as deliberately unchanged, because it is
  the one dependency prohibition the relaxation does not touch.
- *Quality attributes*, **Containment**: *"the engine's empty dependency set survives the introduction of a 57-crate
  framework, provably and per package"* is no longer true as written. Replace with the property that survives — the
  engine's dependency set is exactly what is declared for the engine, the observer's framework is not in it, and both
  facts are provable per package rather than argued.
- *Prohibited patterns*, the entry *"Re-deriving the engine's validation verdict in the observer, which would let the
  display disagree with the engine about what was authorized"*: extended for decision 11 to bind admitted crates, so
  that no crate in the observer's declared set re-derives the verdict either. The prohibition gains reach rather than
  changing meaning.
- *Conformance checks*, first entry: *"Confirm `cargo tree` for the engine package resolves to the engine package
  alone"* becomes resolution against the declared set. The remaining checks are unchanged, including non-perturbation
  and the terminal-free test run.
- Amendment record row naming this ADR.

### `ADR-MOK-001` — technical owner

- *Status*: a dated note. Its assertion that the foundation requires no network and no credentials was supported by
  the engine having no dependencies at all. That support is withdrawn and the assertion is not: it now rests on
  decision 4 of this ADR and on the two conformance checks that replace the empty-graph inference. No decision, no
  option and no consequence of `ADR-MOK-001` changes, and it is not superseded.

### `ADR-MOK-003` — technical owner

- *Status*: a dated note recording that decision 4 is reversed in full and that decision 5's *"only"* is reversed,
  that `ratatui`'s version, feature set and 57-crate figure are untouched, and that every other decision — the
  two-package split, the read-only observation surface, the dependency direction, the one mutating operation,
  wall-clock confinement and the in-memory rendering assertions — stands. Not superseded, on the precedent of its own
  treatment of `ADR-MOK-002`.

### `SPEC-MOK-002` — technical owner

- Rule 1: *"The dependency and dev-dependency tables stay empty, with no exception, including a dependency shared
  with another package in the same workspace"* is replaced by the declared-set form for both tables. Dev-dependencies
  are declared like dependencies and are not exempt from decision 6, since a test-only crate that draws entropy can
  make a test flake in a repository whose figures are replay hashes.
- Rule 1's *"no third target and no build script"*: unchanged, and stated as unchanged. A build script is a code-
  execution surface that this decision does not open.
- The `cargo tree -p Mokiterions` check: becomes the declared-set comparison of decision 7, with the offline
  resolution beside it.
- **The declared set itself is specified here for the engine package** — one table, one row per crate, with version,
  features and, per decision 13, whether the crate carries a build script — and is empty on the day this amendment
  lands. An empty table is now a fact about the current declaration rather than a rule.
- Amendment record row. **Both 2026-08-18 rows stay OUTSTANDING and untouched.**

### `SPEC-MOK-003` — technical owner

- *Terminal user-interface library*: `ratatui`'s version, features, the 57-crate figure and the `serde`-off clause are
  unchanged. The clause making it *"a dependency of the observer component alone"* stays; the implication that it is
  the observer's *only* dependency is replaced by a pointer to the observer's declared set.
- **The observer's declared set is specified here**, in the same shape as the engine's: the `ratatui` entry with its
  version and its three features, and — per decision 13 — the eight resolved crates that carry a build script,
  recorded by name (`instability`, `parking_lot_core`, `proc-macro2`, `quote`, `rustversion`, `syn`, `thiserror`,
  `winapi`). The figures are re-measured when the amendment is written rather than copied from this ADR, since a
  toolchain or lockfile change between drafting and writing would move them.
- *Component layout* clause 1, *"The engine package's external dependency set is empty and admits no exception"*:
  the declared-set form.
- The two later restatements — the consequence sentence about a 57-crate surface confined to the observer, and the
  clause listing *"including the empty dependency table for the engine package"* among what the change verifies —
  amended to match, and the *Explicitly unspecified decisions* entry withholding the dependency, its version and its
  feature set from the implementation is **extended** to every declared crate rather than narrowed.
- Amendment record row; its own 2026-08-18 row stays OUTSTANDING and untouched.

### `SPEC-MOK-004` — technical owner

- Rule 1's clause that the root manifest declares no `[package]`, `[lib]`, `[[bin]]`, `[dependencies]` or
  `[workspace.dependencies]`: every entry but the last stays — a virtual manifest has no targets and no dependencies
  of its own. `[workspace.dependencies]` becomes the rule that governs a crate shared by both packages: it may be
  declared there only when the crate is a declared entry of both packages' sets, which is what makes a shared crate
  visible rather than duplicated.
- The *Counterexample* that rejects exactly that — *"Adding `[workspace.dependencies]` to the root manifest so both
  packages could share a version key violates rule 2 and rule 7, and would give the engine package a dependency table
  that is not empty in substance"* — is the provision this row reverses, and it is the reason this specification is in
  the list at all. It is replaced by a counterexample that still bites: a `[workspace.dependencies]` entry that is not
  in both packages' declared sets.
- Rule 1's sentence that the engine's `[dependencies]` table moves with the manifest *"unchanged, including the
  empty"* table, and any counted rule whose figures a manifest edit would move, are re-measured when the amendment is
  written, and the row records the re-measurement, as `SPEC-MOK-004`'s naming row did after the merge.

### `SPEC-MOK-005` — technical owner

- Rule 8.4, *"The engine package's dependency tree resolves to exactly one crate"*, is the release gate and is the
  provision this decision most directly reverses. It becomes the declared-set comparison for **both** packages, and
  gains the three companions of decision 7: the feature audit, the offline resolution and test of the engine package
  from the committed lockfile, and the by-name scan for network, asynchronous-runtime, database, model-provider and
  misplaced user-interface crates. The sentence naming which architectural claim it discharges is redirected to this
  ADR.
- Rule 8.4 also states what the set **is**, per decision 7: the resolved per-package graph at the declared features,
  on each supported platform, and expressly not `Cargo.lock`'s contents. The measured 182-against-59 gap and the two
  crates that sit in the lockfile and resolve into no build are recorded as the reason, so that a later reader does
  not simplify the check back into a lockfile read.
- **A new provision for decision 12**: the same comparison runs on every pull request, in a repository-owned
  workflow, and the release gate is not its only placement. The provision states that the managed
  `engineering-harness.yml` is not the vehicle and why — it is locked in `.engineering-harness.lock` — and that the
  pull-request workflow carries these checks alone, leaving `cargo fmt`, `cargo clippy` and the test suite where this
  repository already puts them, in retained evidence at the candidate commit.
- Rule 11.5, *"Producing the statement adds no dependency to either package and requires no build script"*:
  unchanged, and stated as unchanged. It constrains **this repository's** packages; decision 13 concerns a
  dependency's own build script, which rule 11.5 never spoke to.
- Amendment record row.

### New verification contract, `VER-MOK-013` — assurance owner

Independent methods and pass conditions for `REQ-MOK-047`, and the artifact that decides what evidence an admission
must retain. It must cover, at minimum: the declared-set comparison per package in both directions; the feature
audit; the offline engine build and test from the committed lockfile; the by-name prohibition scan; a determinism
re-derivation at the declared seeds, compared against the replay hashes retained under `VREC-MOK-002`, since decision
6 is the obligation a graph check cannot see; and **two manual assessments**, because two of this ADR's decisions are
judgements by construction — that no declared entry implements simulation semantics, owns entropy or validates an
action (decision 11), and that the criteria of decision 1 were applied to each entry by the technical owner
(decision 10). A manual assessment that has not been made is recorded **OUTSTANDING** and leaves the contract
unsatisfied, as `VER-MOK-011`'s fifth still does.

### `REPOSITORY_CONTEXT.md` — repository owner, no product or governance authority

- The three statements that restate the rule: the engine's tree resolving to one crate, the paragraph requiring the
  table to stay empty *"with no exception"* and stating that adding any other dependency to either package requires
  escalation and an updated architectural decision, and the sentence placing two named crates *"outside the engine's
  dependency set"*. All three are amended to the declared-set form and to point at this ADR for admission.

### `.github/workflows/release.yml` — no authority; changes or the release refuses

- The step *"The engine's dependency table must stay empty"* asserts one line of `cargo tree -p Mokiterions
  --locked`. It becomes the amended rule 8.4's check. **Until it changes, a release from a commit with any engine
  dependency refuses**, which is `REQ-MOK-036`'s scenario working as designed rather than a defect.
- The workflow's trigger is unchanged: `push: tags:`, a tag an accountable human created. Nothing about who releases
  moves.

### A new repository-owned pull-request workflow — no authority; implements decision 12

- A new file under `.github/workflows/`, repository-owned in the sense `release.yml` is: it implements a
  specification provision and it neither modifies nor invokes the managed workflow. Triggered on `pull_request`.
- It carries exactly the amended rule 8.4 checks — the per-package resolved-set comparison at the declared features,
  the feature audit, the by-name prohibition scan, and the engine's offline resolution and test from the committed
  lockfile — and nothing else. It does not add `cargo fmt`, `cargo clippy` or the test suite to pull-request CI.
- **`.github/workflows/engineering-harness.yml` is not touched.** It is listed in `.engineering-harness.lock` with a
  schema-2 SHA-256 lock, `python -m se_harness doctor` checks it, and the `governor` job would report the tamper.
  This ADR states that as a constraint on the implementation, not as a preference.
- The workflow is evidence, not authority: it can refuse a merge and it approves nothing.

### Source comments — no authority

- `mokiterions-core/Cargo.toml`'s comment above the empty `[dependencies]` table, `mokiterions-tui/Cargo.toml`'s
  comment fixing `ratatui`, and `mokiterions-core/src/lib.rs`'s module documentation asserting that the engine *"has
  no external dependencies"* and that `ARCH-MOK-001` *"admits no exception to that"*. Each states a rule it does not
  own; each is corrected to the declared-set form in the same commit, so no comment outlives the provision it cites.

## Consequences

### Positive

- The instruction is delivered: third-party crates are admissible in both packages, on the owner's criteria, in the
  owner's words.
- The mechanical check gets stronger where it changes. *The set is empty* answered one question; *the resolved set
  equals the declared set at the declared versions and features* answers that one, plus which crates are present,
  whether a transitive addition slipped in, and whether a feature was enabled without an amendment.
- A shared crate becomes visible instead of impossible. `[workspace.dependencies]` with a declared entry in both sets
  says so once; the old rule's *"including a dependency shared with another package"* clause could only forbid it.
- The observer's escalation gate and the engine's prohibition become one policy, so a contributor no longer has to
  know which package they are in to know which rule applies.
- Registry independence and determinism become *stated obligations with named checks* rather than side effects of an
  empty table. They were never written down as obligations before, because nothing threatened them.
- Dev-dependencies come into the policy. A test-only crate was previously banned outright and is now admissible under
  the same declaration and the same determinism obligation.
- The build-time code-execution surface gets written down for the first time. The repository has executed eight
  third-party build scripts since `ratatui` was admitted; after decision 13 it says which eight.
- Graph drift becomes a pull-request failure instead of a release-day surprise. A lockfile update or a feature
  unification that pulls in an undeclared crate is caught where it is cheap to fix, by the workflow decision 12 adds.

### Negative

- **The repository gives up its strongest structural property.** *The engine cannot contain a network client* was a
  fact about a manifest; it becomes a check that a reviewer, a script and a release gate have to keep running. The
  property is not equivalent afterwards, and this ADR does not claim it is.
- **It is difficult to reverse in practice.** Once a crate is in the engine and its API is in the engine's code,
  removing it is a rewrite, not a revert. The empty table can be legislated back and the code cannot.
- **The criteria are judgements in a repository whose standard is measurement.** *Stable*, *well-maintained* and
  *excessive dependency debt* have no threshold here, and this decision deliberately does not invent one — a
  fabricated numeric bar would be worse than a named owner. The cost is that every admission consumes an accountable
  decision.
- **The promised acceleration is partly spent on process.** Admission requires a specification amendment and an
  owner's approval. For a crate that saves a week, that is cheap; for one that saves an hour, the policy will not
  feel relaxed. This is the honest tension between the instruction's first sentence and this repository's amendment
  discipline, and it is not resolved here.
- **Determinism moves from structural to defended for the engine.** Decision 6 is a review obligation on every future
  crate, and the failure mode is quiet: a crate that iterates a hash map in a value the stream observes produces a
  run that differs only sometimes.
- **Verified behavior is now downstream of a dependency choice.** The four replay hashes retained under
  `VREC-MOK-002` in `evidence/WO-MOK-002/determinism-and-resilience.md` and `REQ-MOK-010`'s verified stream can be
  invalidated by an admitted crate, which is a heavier consequence than failing a gate: it retires evidence rather than
  blocking a commit.
- **`Cargo.lock` review grows** for both packages, and the engine's audit stops being *read one table*.
- **Decision 11 is a review obligation, and reviews drift.** *Does this crate implement simulation semantics* has a
  clear answer for a terminal backend and a debatable one for, say, a spatial-index or fixed-point-arithmetic crate.
  The prohibition will hold exactly as well as the assessment behind it, and `VER-MOK-013` records that assessment
  rather than automating it.
- **The pull-request workflow can refuse a merge for a reason no line of the diff shows.** That is its purpose, and it
  is still a new way for CI to block work: a contributor whose change is correct can be stopped by a transitive
  resolution they did not choose, and the fix is an amendment rather than a code edit.
- **CI surface grows by one repository-owned workflow.** It must be maintained, it duplicates checks the release
  workflow already runs, and the two can drift apart. `release.yml` already accepted that duplication against the
  managed workflow for the same reason — the managed one declares no `workflow_call` trigger — so this is the second
  instance of a pattern rather than a new one.
- **Two pre-existing OUTSTANDING amendment rows sit in the files this change amends** — on `ARCH-MOK-001`,
  `SPEC-MOK-002` and `SPEC-MOK-003`, all `WO-MOK-005`'s preconditions. Adding rows beside them clears nothing, and a
  reader of those tables will now see two unapproved amendments and one approved one interleaved.

### Operational and security

- The supply-chain surface of the engine package changes from zero to whatever is declared. That is the security
  consequence of this decision and the reason decision 4 preserves every trust prohibition verbatim: what is being
  relaxed is a *count*, not the *kinds* of capability admitted.
- No network access, no credential, no secret in the repository, no model provider, no asynchronous runtime and no
  database in either package. Unchanged, and now carried by a by-name check instead of by an empty graph.
- `--locked` resolution stays required, so a release is never built from versions the repository did not record.
- An admitted crate's build script executes code at build time, and decision 13 admits them and requires each to be
  disclosed in the declared set. `SPEC-MOK-002` rule 1's prohibition on a build script covers this repository's own
  packages and never covered a dependency's, so this is a surface the repository already had and had not written
  down: **8 of the observer's 59 resolved crates carry one today**. Recording them does not reduce the surface. It
  makes it reviewable, and it means a ninth appearing in a lockfile update is a declared-set mismatch rather than an
  unremarked change.
- **Yanked-version and advisory handling is the one thing this ADR leaves out of scope, and says so.** No
  `cargo audit`, no `cargo deny`, no licence policy is adopted here. If one is wanted it is a further decision;
  pretending this one covers it would be the kind of implied approval this repository does not accept. What follows
  from that honestly: a declared crate with a published advisory passes every check this decision adds.

### Migration

Nothing in the tree changes on the day this is accepted. Both manifests keep the tables they have, the engine's
declared set is written down empty, and the observer's is written down as the `ratatui` entry that
`SPEC-MOK-003` already fixes. The first commit under this policy therefore changes prose and CI only, and the gates
it must pass are the ones the repository already runs — which is what makes the change reviewable: the diff cannot
hide a dependency, because it does not add one.

The first *admission* is a separate act, under the amended `SPEC-MOK-002` or `SPEC-MOK-003`, with the criteria
applied to that crate by the technical owner and the amendment row recording that they were applied. Reversing this
policy later means reversing the code that uses what it admitted, so a decision to admit is materially harder to undo
than this decision is.

## Validation

- The resolved dependency set of each package, per package rather than for the workspace, equals that package's
  declared set at the declared versions and the declared feature sets, in both directions.
- No crate providing network access, credential handling, an asynchronous runtime, a database, a plugin system or
  dependency injection appears in either resolved graph, checked by name.
- No user-interface dependency appears outside the observer package's declared set, including one that both sets
  would declare.
- The engine package resolves, builds and tests from the committed lockfile with no registry access, and with no
  terminal attached.
- The engine package does not depend on the observer package, directly or transitively.
- An observed run and an unobserved run at the same seed, configuration and decision source remain byte-identical in
  authoritative events and final state, and the declared seeds reproduce the replay hashes retained under
  `VREC-MOK-002`.
- No admitted engine crate is reachable from a value the `REQ-MOK-010` stream, the authoritative event sequence or the
  final state observes while drawing entropy, reading wall-clock time, reading the environment, or iterating in an
  unspecified order — reviewed, since no graph read answers it.
- No crate in either declared set implements simulation semantics, owns or advances entropy, or performs action
  validation — reviewed, and the assessment retained under `VER-MOK-013` rather than asserted here.
- Each declared entry's build-script status matches the resolved graph, so a build script appearing in a crate that
  did not carry one is a declared-set mismatch and not an unremarked change.
- The comparison runs on every pull request and at release, and `python -m se_harness doctor` reports
  `.github/workflows/engineering-harness.yml` intact, which is what proves the pull-request check was added beside the
  managed workflow rather than inside it.
- Every crate in either declared set is traceable to an amendment row whose *Approval* cell names the technical owner
  and records that the criteria of decision 1 were applied to it.
