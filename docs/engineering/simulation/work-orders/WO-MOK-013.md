+++
id = "WO-MOK-013"
type = "work_order"
title = "Replace the engine's empty-dependency rule with a declared-set comparison, and check it at pull-request time and at release"
status = "in_progress"
owners = ["engineering owner"]
created = "2026-08-20"
updated = "2026-08-20"

[assurance]
commit_bound_verification = "required"
rationale = "The load-bearing claim of this change is that relaxing a prohibition moved nothing: the engine's resolved graph is still the engine alone, the observer's is the same 57, 63 and 62 crates on the three release targets, and the four replay hashes REQ-MOK-009 rests on are byte-identical to the ones retained before it. Each of those is a claim about a tree at a commit and none can be asserted. A declared set is a measurement, so a figure written into a specification is true only of the commit it was measured at, and the program that compares the two is new code which can pass by being wrong. The by-name prohibition scan has the same standing: it refused nothing on the day it was written, and a check designed to pass behaves identically, so the evidence has to include a demonstration that it refuses when it should. Commit-bound verification is also what makes the two disclosed transitive capabilities checkable later: mio resolves with its net feature on two of the three targets, and a record that does not name the commit whose graph that is cannot be re-derived by anyone."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-026", "REQ-MOK-036", "REQ-MOK-047"]
specifications = ["SPEC-MOK-002", "SPEC-MOK-003", "SPEC-MOK-004", "SPEC-MOK-005"]
architecture = ["ARCH-MOK-001", "ARCH-MOK-002", "ADR-MOK-006"]
verification = ["VER-MOK-013"]
+++

# Work Order: A declared dependency set, in place of an empty one

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and the retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

The `architecture` relation is declared and carries three identifiers, because unlike `WO-MOK-011` this change is
architectural in the traceability rule's own terms: `ARCH-MOK-001` addresses `REQ-MOK-016` and `ARCH-MOK-002` addresses
`REQ-MOK-026`, both of which this work order implements, and both architectures carry prohibitions and conformance
checks whose text this change amends. `ADR-MOK-006` sits beside them as the deciding ADR, on the precedent of
`WO-MOK-001`, `WO-MOK-003`, `WO-MOK-005` and `WO-MOK-006`, each of which names its ADR in the same relation.

**This work order cannot be approved before the artifacts it depends on are.** `REQ-MOK-047` is the product owner's
act, `VER-MOK-013` is the assurance owner's, `ADR-MOK-006` is the technical owner's, and every amendment below is one
of those three. **`ADR-MOK-006` and this work order were both approved by the repository owner on 2026-08-20**, in one
instruction — *"i approve the ADR, i approve the work order: go implement"* — acting as product owner, technical owner,
assurance owner and engineering owner. The owner holds every accountable role in this repository, so that instruction
is four acts and not one, and nothing is approved by implication: the acts it does **not** contain are named under
*Out of scope*.

### Approval preconditions

`ADR-MOK-006`'s *Required amendments* section holds **sixteen** entries, and every one of them is a precondition of
this work order, exactly as `WO-MOK-005` and `WO-MOK-006` made their ADRs' amendments preconditions. Each is stated in
full in the ADR and is not restated here; what this section does is bind them to the approval and say who owes each.

**Twelve land in a governed artifact and carry an approval:**

| Artifact | What the amendment does | Accountable role |
|---|---|---|
| `REQ-MOK-026` | Removes *"with no external dependency and"* from the statement; keeps all three obligations in the rationale, each now resting on a named check | product owner |
| `REQ-MOK-047` | **New.** The declared-set obligation in `SHALL` form, `verification_method = "static-analysis"`, deriving from `CAP-MOK-004` | product owner |
| `REQ-MOK-036` | The release-refusal scenario refuses a resolved set that differs from the declared one, in either direction, instead of any engine dependency | product owner |
| `ARCH-MOK-001` | Three *Prohibited patterns* entries, three *Conformance checks* entries including the split of the empty-graph inference into two, two *Quality attributes*, and the decision assessment's rationale | technical owner |
| `ARCH-MOK-002` | Four *Prohibited patterns* entries, the **Containment** quality attribute, and the first conformance check | technical owner |
| `ADR-MOK-001` | A dated *Status* note: the no-network, no-credentials assertion stands and now rests on decision 4 rather than on an empty graph. Not superseded | technical owner |
| `ADR-MOK-003` | A dated *Status* note: decision 4 reversed in full, decision 5's *"only"* reversed, every other decision standing. Not superseded | technical owner |
| `SPEC-MOK-002` | Rule 1's two tables in declared-set form, the `cargo tree` check replaced by the comparison, and **rule 13**, the engine's declared set, empty on the day it lands | technical owner |
| `SPEC-MOK-003` | The `ratatui` clause repointed, the observer's declared set added with its build-script table, *Component layout* clause 1, and two later restatements | technical owner |
| `SPEC-MOK-004` | Rule 1's `[workspace.dependencies]` clause becomes a rule about a crate declared in both sets; the counterexample that forbade it is replaced by one that still bites | technical owner |
| `SPEC-MOK-005` | Rule 8.4 becomes the four-part comparison, states what the set is and is not, and **rule 15** places it at pull-request time | technical owner |
| `VER-MOK-013` | **New.** Independent methods and pass conditions for `REQ-MOK-047`, and the decision about what an admission must retain | assurance owner |

**Four carry no authority, and are checked for presence and consistency rather than approval:**
`REPOSITORY_CONTEXT.md`'s three restatements; `.github/workflows/release.yml`'s dependency step; the new
repository-owned pull-request workflow; and the source comments. The ADR says of each that it states a rule it does not
own. They are still preconditions of completion, because a comment or a CI step that outlives the provision it cites is
how a withdrawn rule keeps being enforced.

**Each amendment record row records that the implementation agent wrote the text and did not decide the substance.**
Every row cites `ADR-MOK-006` as its approval. Where the written text reaches beyond what the ADR enumerated, the row
says so in the row itself rather than in a commit message; those reaches are listed under *Completion report format*
and each is a disclosure the technical owner can reverse.

### Transition to `in_progress`

Status moved from `approved` to `in_progress` on 2026-08-20, in the same commit as the change, on the owner's
instruction to implement. It is **not** moved to `implemented` in that commit: no instruction covers that transition,
`WORKFLOW.md` reserves it, and `WO-MOK-011`'s record of the same situation is the precedent — the status stood at
`in_progress` through implementation and moved only when the owner said so.

### Note dated 2026-08-20 — three corrections to this work order, no scope change

This work order carries no *Amendment record* table, because `WORK_ORDER.template.md` defines none; a work order records
a later change in *Lifecycle*, as an ADR records one in *Status*. This note is that record. **It changes no scope, adds
no in-scope item, removes no out-of-scope item and authorizes no act.** All three corrections were decided by the
repository owner on 2026-08-20 acting as accountable **engineering owner**, in answer to questions this work order's
implementation put to the owner; the implementation agent wrote the text and decided none of it.

1. **Seven manual assessments becomes six.** `VER-MOK-013`'s *Manual assessments* list is numbered 1, 2, 3, 4, 6, 7 and
   contains no fifth. This work order said *"seven"* under *Required verification* and in its evidence list and its
   *Completion report format*, and said *"Assessments 1 to 5 and 7"* under *Out of scope*, naming an assessment the
   contract does not state. The owner resolved the discrepancy in the direction that **six is the intended set** and the
   *"seven"* was the error. `VER-MOK-013` is **not** renumbered and the gap in its numbering stays: assessment numbers
   are cited by number in `SPEC-MOK-003`, `SPEC-MOK-005`, `VER-MOK-013` itself and
   `scripts/check_declared_dependencies.py`, and `evidence/WO-MOK-013/WO-MOK-013-manual-assessment.md` enumerates the
   twenty citations renumbering would break. No obligation is added or removed by this: the same six judgements were
   owed before this note and are owed after it.
2. **The exclusions bullet now speaks of every assessment the contract states**, rather than of *"the two manual
   assessments this change itself creates"* — a phrase that named only assessment 6 while assessments 3 and 4 are also
   created by this change. It also records assessment 6's disposition: the owner recorded it later the same day as
   **accepted**, limited to a compiled and uncalled capability. The bullet's substance is unchanged — every one of these
   judgements belongs to an accountable role and none to the implementation agent.
3. **One reference corrected.** *Required verification* and *Completion report format* said `VREC-MOK-002`'s *"four
   hashes"*. That record holds no replay hash — it holds its own `artifact_snapshot_sha256` — and the four are in
   `evidence/WO-MOK-002/determinism-and-resilience.md` at lines 12 to 15, which its `evidence_paths` binds. Both places
   now read *"retained under `VREC-MOK-002` in `evidence/WO-MOK-002/determinism-and-resilience.md`"*. The baseline
   substitution of oracle 3 is untouched; what is corrected is where a reader is sent. `VER-MOK-013` and `ADR-MOK-006`
   carry the same correction, each under its own dated row or note, and this third correction is a **reach beyond what
   the owner's answer enumerated** — the answer named those two artifacts — disclosed here for that reason and
   reversible by striking this paragraph and the two phrases it changed.

### Why this chain is numbered 006, 047 and 013

The identifiers are not consecutive with what this tree holds, and the reason is recorded here rather than left to be
discovered. `origin/feature/phase-4a-definition` holds a `WO-MOK-012` and a `VER-MOK-012` for unrelated work in
flight. The local maximum in this tree is `WO-MOK-011` and `VER-MOK-011`, and taking `012` would collide on merge,
which `evidence/WO-MOK-007/renumbering.md` records the owner resolving once already and records as the owner's act
rather than an implementation agent's. This chain therefore takes the next identifier above every number used on every
branch in each family it touches: `ADR-MOK-006`, `REQ-MOK-047`, `VER-MOK-013`, `WO-MOK-013`, and `VREC-MOK-013` when
the verification record is written. Every ref was checked, not only the local maximum.

### Decision record

On 2026-08-20 the repository owner took the scope decisions this packet is drafted around, before the ADR was written.
They are design decisions, not approvals; `ADR-MOK-006`'s numbered decisions are where they became normative.

| Decision | Outcome | Role |
|---|---|---|
| Which packages the relaxation covers | Both. One policy, not an engine-only exception | technical owner |
| The separately-motivated prohibitions | Keep them all — network, credentials, async runtime, database, plugin system, dependency injection, and the user-interface confinement | technical owner |
| Whether an ADR precedes the pack | Yes, draft the deciding ADR first | technical owner |
| A numeric threshold for dependency debt | None. *Excessive* is the owner's judgement per admission | technical owner |
| *Standard, non-core features* | Make it an enforceable prohibition: nothing admitted may implement simulation semantics | technical owner |
| Where the declared-set check runs | At pull-request time as well as at release, in a repository-owned workflow | technical owner |
| Build scripts in an admitted crate | Admissible, and disclosed per crate in the declared set | technical owner |

**The originating instruction is quoted in the ADR's *Context* and was verified before being relaxed.** The instruction
asserted that repository policy forbids additional third-party crates for the core crates and asked for that to be
verified; it was, and the ADR records where the prohibition was written — nine places across five artifact families,
plus three source comments and a CI step.

## Objective

Replace one prohibition with one comparison, and prove that nothing else moved.

The prohibition is the engine package's empty dependency table, *"with no exception"*. The comparison is
`REQ-MOK-047`: each package's resolved external dependency set equals the set its specification declares, crate by
crate, version by version, feature by feature, on each of the three targets the release builds — with the capability
classes `ADR-MOK-006` decision 4 preserves checked by name, and the engine still building and testing offline from the
committed lockfile. The comparison runs on every pull request and again at the release gate, from one implementation,
so the two placements cannot disagree about what is declared.

Nothing is admitted by this work order. No crate is added to any manifest, no version moves, no feature is switched
on, and the engine's declared set is empty when the change lands — which is now a fact about the declaration rather
than a rule.

## In scope

- The twelve governed amendments and the four unauthoritative corrections listed under *Approval preconditions*, each
  with its amendment record row where the artifact carries one, and each dated 2026-08-20.
- `scripts/check_declared_dependencies.py`: the shared implementation of `SPEC-MOK-005` rule 8.4a to 8.4d, which reads
  the declared sets out of `SPEC-MOK-002` rule 13 and `SPEC-MOK-003` and restates neither, since rule 15.5 forbids a
  third source of truth.
- `scripts/test_check_declared_dependencies.py`: its `unittest` suite, which is also the injection-based demonstration
  that each check refuses — `VER-MOK-013` acceptance scenarios 2, 3, 4 and 10.
- `.github/workflows/release.yml`: the *"The engine's dependency table must stay empty"* step replaced by
  `cargo fetch --locked` followed by the program. The trigger is unchanged.
- `.github/workflows/dependency-declarations.yml`: the new repository-owned `pull_request` workflow, carrying rule
  8.4's checks and nothing else.
- The source comments that state the withdrawn rule: `mokiterions-core/Cargo.toml`, `mokiterions-tui/Cargo.toml`,
  `mokiterions-core/src/lib.rs`, and — beyond the ADR's enumeration, disclosed — the root `Cargo.toml` and
  `rust-toolchain.toml`.
- Every measurement `VER-MOK-013` requires, and the evidence it retains under
  `docs/engineering/simulation/evidence/WO-MOK-013/`, in files named for the work order.
- The determinism re-derivation at the five declared seeds, and its comparison against a retained capture.

## Out of scope

- **Admitting any crate.** No entry is added to either declared set, either manifest, or `[workspace.dependencies]`.
  Decision 8 of the ADR is explicit that it admits nothing, and decision 9 is explicit that the implementation may not
  choose a crate, a version, a feature set, or whether a candidate meets the criteria.
- Any change to `Cargo.lock`, to a resolved version, or to a feature. The graph after this change is byte-identical to
  the graph before it.
- Any change to simulation behaviour, to the public interface of either package, to a rule, a default, a floor or an
  exit code. This change compiles no differently.
- **`.github/workflows/engineering-harness.yml`**, which is managed, SHA-256 locked in `.engineering-harness.lock` and
  checked by `python -m se_harness doctor`. Rule 15.3 states this as a constraint, not a preference.
- Adding `cargo fmt`, `cargo clippy` or the test suite to pull-request CI. Rule 15.2 keeps the new workflow narrow.
- Advisory scanning, licence policy, `cargo deny`, `cargo audit`, vendoring, and a crate-count ceiling. The ADR
  considered and declined each; a check this work order did not build is not a check this repository has.
- Resolving any amendment row that stands **OUTSTANDING** from earlier work: `ARCH-MOK-001` and `SPEC-MOK-002` (two)
  and `SPEC-MOK-003` (one) at 2026-08-18, `SPEC-MOK-004` at 2026-08-19, and `VER-MOK-011` manual assessment 5. None is
  touched, cleared or inherited.
- **Every manual assessment `VER-MOK-013` states.** `VER-MOK-013` manual assessment 6 — whether the disclosed
  transitive capability of `mio` with its `net` feature is accepted — is the technical owner's, and was **OUTSTANDING**
  as this work order was approved; the owner recorded it on 2026-08-20 as **accepted**, limited to a compiled and
  uncalled capability. Assessments 1, 2, 3, 4 and 7 are likewise the accountable roles', not the agent's; 1, 2 and 3
  were recorded the same day and 4 and 7 stand as they did. The agent may draft the text an owner then adopts, and may
  make none of these judgements itself.
- Committing, pushing, tagging, opening a pull request, or transitioning this work order to `implemented`. The
  approving instruction covered implementation; each of those is a separate act and none was named.

## Authorized decision envelope

The implementation agent may decide locally:

- The internal organization of `check_declared_dependencies.py`: its functions, its dataclasses, its exception type and
  its output format, following the conventions `check_release_reachability.py` already sets — Python 3.11 standard
  library only, a `Refusal` exception, `REFUSED:` on standard error and exit 1, `--root`, and a docstring naming the
  rule it implements and stating that it is repository-owned.
- Which facts the program reads from where, provided no declared set is restated in it, and provided the only
  repository knowledge written into it is the pointer from each package to the specification that declares its set.
- The test suite's organization, and the shape of each injected mismatch.
- The wording of every amendment's text, once its substance is approved, and the wording of the corrected comments.
- The evidence packet's file names and layout, subject to each file being named for the work order.
- The workflow files' step names, comments and step ordering, provided `cargo fetch --locked` precedes any offline
  check and the managed workflow is untouched.

The implementation agent may **not** decide:

- Whether any crate is admissible, or the substance of any amendment.
- Which crates the by-name scan looks for, in the sense of trimming the list so that the graph passes. The terms are
  written from decision 4's capability classes; a term removed because it matched is a check designed to pass.
- Whether a transitive capability the scan finds is acceptable. It may measure one and record it as measured; recording
  it as accepted is the technical owner's act, which `SPEC-MOK-005` rule 8.4d states in those terms.
- Any figure in a specification. Every count, version and feature set written into an amendment is measured in this
  checkout and recorded with the command that produced it.
- Any lint suppression, any gate change, any `#[ignore]`, or any edit to an existing amendment record row.
- Any edit to an artifact bound by a `verified` verification record. `VREC-MOK-002` in particular is **not** edited,
  and the reason its hashes cannot serve as this change's determinism baseline is recorded in `VER-MOK-013` instead.

## Constraints

- **The graph does not move.** No lockfile change, no version change, no feature change, no new crate on any target.
- **The run does not move.** The four replay hashes at the declared seeds and densities are byte-identical to the
  retained capture, and `REQ-MOK-009` is re-derived rather than assumed.
- **Every prohibition of decision 4 survives**, and is checked by name rather than inferred from an empty graph. The
  user-interface confinement to the observer package is the one dependency prohibition the relaxation does not touch
  at all.
- No declared set appears in a script, a workflow, a fixture or a test as data. One declaration, two readers.
- Every cargo invocation in a check passes `--locked`, and every one after the fetch step passes `--offline`. No check
  writes a tracked file; the program compares `git status` across its own run and refuses if the tree moved.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` and
  `cargo test --workspace --locked` clean, in one invocation each, with nothing suppressed.
- The artifact validator passes with 0 errors and 0 warnings across all four planes, and `python -m se_harness doctor`
  reports no tamper.
- No secret, credential or model-provider key enters the repository, in code, in a fixture or in retained evidence.

## Expected change surface

**Artifacts** — the twelve amendments above, in `docs/engineering/simulation/`: `requirements/REQ-MOK-026.md`,
`requirements/REQ-MOK-047.md` (new), `requirements/REQ-MOK-036.md`, `architecture/ARCH-MOK-001.md`,
`architecture/ARCH-MOK-002.md`, `architecture/adr/ADR-MOK-001.md`, `architecture/adr/ADR-MOK-003.md`,
`architecture/adr/ADR-MOK-006.md` (new), `specifications/SPEC-MOK-002.md`, `specifications/SPEC-MOK-003.md`,
`specifications/SPEC-MOK-004.md`, `specifications/SPEC-MOK-005.md`, `verification/VER-MOK-013.md` (new), and this file.

**`scripts/check_declared_dependencies.py`** (new)

- Reads the declared sets from the two specifications' Markdown tables: the entry table, the per-target figures, the
  build-script table and the disclosed-capability table.
- Reads each manifest with `tomllib` over `dependencies`, `dev-dependencies`, `build-dependencies` and every
  `target.<cfg>` variant; a path dependency on a workspace member is not an entry and a path dependency outside the
  workspace refuses.
- Reads the resolved graph from `cargo tree -p <package> -e <edges> --locked --offline --target <triple>
  --prefix none --no-dedupe -f "{p}|{f}"`, and build-script status from one `cargo metadata --locked --offline`.
- 8.4a in both directions per package and over `[workspace.dependencies]`, which is admissible only for a crate
  declared in both sets. 8.4b against declared-plus-implied features with the negatives asserted. 8.4c the engine's
  offline build and test. 8.4d the by-name scan, where a declared entry refuses outright and a transitive hit refuses
  unless disclosed for that target at that version. Plus rule 13's strict direction for the engine, the per-target
  counts and their union, and the build-script table per target.
- Each check reports every mismatch it finds rather than the first, and the program exits 1 if any refusal was
  recorded.

**`scripts/test_check_declared_dependencies.py`** (new) — reading tests over the real artifacts, so an amendment that
renames a heading or writes an unreadable *Features* cell fails here; and injection tests that construct each mismatch
and assert the refusal.

**`.github/workflows/release.yml`** — one step replaced by two in the `verify` job. **`.github/workflows/dependency-declarations.yml`** (new).

**Source comments** — `Cargo.toml`, `mokiterions-core/Cargo.toml`, `mokiterions-tui/Cargo.toml`,
`mokiterions-core/src/lib.rs`, `rust-toolchain.toml`, and `docs/engineering/REPOSITORY_CONTEXT.md`.

**No Rust source file's behaviour changes.** `mokiterions-core/src/lib.rs` is touched in its module documentation only.

## Required verification

`VER-MOK-013`, in full: its five oracles, its requirement-to-evidence matrix, its twelve acceptance scenarios, its
property and invariant tests, its static, architecture, security, privacy, performance and resilience checks, and its
six manual assessments — the contract's list is numbered 1, 2, 3, 4, 6, 7 and contains no fifth. A manual assessment
that is not recorded is outstanding, and the contract is not satisfied while any remains outstanding.

Oracle 3's determinism baseline is a **retained pre-existing capture**, and which capture it is matters: the four
hashes retained under `VREC-MOK-002` in `evidence/WO-MOK-002/determinism-and-resilience.md` cannot serve, because they
were taken on 2026-08-17 at commit `68163ac452619e2f8d5a05ed3a73d42b920ba5f6`, before `WO-MOK-007` added `fear` and
`WO-MOK-011` added `name:` to the record they hash. `ADR-MOK-006` names them, and it names a tree that no longer exists.
`evidence/WO-MOK-011/post/post-manifest.txt`, bound by `VREC-MOK-011`, is substituted. Neither `VREC-MOK-002` nor the
evidence it binds is edited, and the substitution is `VER-MOK-013` manual assessment 4, the assurance owner's.

## Evidence to record

Under `docs/engineering/simulation/evidence/WO-MOK-013/`, everything `VER-MOK-013`'s *Evidence retention* section
lists. **Every file is named `WO-MOK-013-*`**, because `discover_evidence` in `scripts/generate_harness_dashboard.py`
matches the work-order identifier against each *file's own name* and not its directory's, so a packet named only by its
directory is invisible to discovery and raises `W-HEX-001` — the warning `WO-MOK-010` and `WO-MOK-011` both carry. In
particular:

- the per-package, per-target resolved graphs, with the exact commands and the toolchain version;
- the per-target crate counts and their union, and the `--target all` and `cargo metadata` figures beside them, so the
  four numbers a reader might confuse are all present with their questions attached;
- the build-script enumeration per target, and the two lockfile crates that carry a build script and resolve into no
  build;
- the resolved feature sets, including the implied `std` and the asserted absence of `default` and `serde`;
- **the by-name scan's raw hits before any disclosure is applied**, the search terms and their provenance in decision
  4, the disclosed set they are matched against, and the residue;
- the four replay hashes at the declared seeds and densities, and their comparison against
  `evidence/WO-MOK-011/post/post-manifest.txt`;
- the program's own test suite output, and the injected-mismatch demonstrations of scenarios 2, 3, 4 and 10;
- the gate output: `cargo fmt`, `cargo clippy`, `cargo test`, the validator, the dashboard, the inspector and
  `python -m se_harness doctor`;
- both workflow files as they stand, and the search showing that neither restates a declared set;
- oracle 5's sixteen entries, each located in the artifact that carries it;
- `WO-MOK-013-manual-assessment.md`, the six assessments with role and date, each outstanding one recorded as
  **OUTSTANDING** rather than omitted, and each recorded one carrying the judgement in the accountable role's own words;
- `WO-MOK-013-completion-summary.md`.

## Stop and escalate conditions

Stop and escalate rather than proceeding, if:

1. **A check cannot be made to refuse.** A check that passes on today's tree and cannot be shown failing on an injected
   mismatch is not evidence of anything, and adjusting the mismatch until it passes is the failure this condition
   exists to prevent.
2. **The by-name scan hits the observer's existing graph** — which it does, and this condition is what it triggered.
   The two options that do not require the owner are both wrong: removing the term makes the check pass by
   construction, and refusing every release rejects a graph `ADR-MOK-003` already accepted. The finding is recorded in
   the artifact that declares the set, with the judgement marked **OUTSTANDING**. Escalated, not resolved.
3. **Any replay hash differs** from the retained capture, at any seed or density.
4. **Any measured figure contradicts `ADR-MOK-006`** — which it does, in three places, all disclosed rather than
   silently corrected: the build-script crates are ten and not eight, `syn` is not among them, and the count is 7, 9, 9
   and 10 by target rather than a single number; and the resolved feature set carries `std`, which no earlier statement
   in this repository mentioned.
5. An amendment would require editing an artifact bound by a `verified` verification record, or an existing amendment
   record row.
6. The declared-set comparison cannot be implemented without restating a declared set in code. That would be rule
   15.5's third source of truth, and a passing check built that way is worse than no check.
7. The managed `engineering-harness.yml` would have to change for the pull-request placement to work.
8. `Cargo.lock`, a version or a feature would have to move to make any check pass.

## Completion report format

The completion summary records, in this order:

1. what changed, artifact by artifact and file by file, and what deliberately did not — no crate admitted, no lockfile
   change, no behaviour change, no managed file touched;
2. the sixteen amendment entries, each with its approval or its explicit lack of authority, and **every reach beyond
   what `ADR-MOK-006` enumerated**, named as a reach the technical owner can reverse: `SPEC-MOK-003`'s security bullet
   and its disclosed-capability section, `SPEC-MOK-005` rule 8.4d's disclosure mechanism, `SPEC-MOK-002` rule 13's
   mechanical reading convention, the three `specifies` relations the ADR did not enumerate, `SPEC-MOK-004`'s and
   `ARCH-MOK-001`'s further clauses, and the root manifest and toolchain comments;
3. the measured declared sets: both packages, all three targets, with the commands and the toolchain version;
4. the per-target dependence of the graph, stated as a finding rather than a footnote, and the four different figures
   (57/63/62, union 66, `--target all` 71, `cargo metadata` 182) with the question each answers;
5. the build-script enumeration, and the deviation from the ADR's eight, including `syn`'s absence and the seven-nine-
   nine-ten shape;
6. the feature audit, including the implied `std` finding and the asserted absence of `default` and `serde`;
7. **the by-name scan: its terms, its raw hits, the two disclosed crates `mio 1.2.2` and `signal-hook-mio 0.2.5` with
   `net` enabled on two of three targets, and the disposition of the assessment they owe** — recorded as OUTSTANDING if
   the technical owner has not judged them, and if judged then with the grounds and the limit of the judgement, so that a
   reader can tell an acceptance from a silence;
8. the determinism re-derivation, the four hashes, and the baseline substitution with its reason, naming the four
   superseded hashes as retained under `VREC-MOK-002` in `evidence/WO-MOK-002/determinism-and-resilience.md`;
9. the injected-mismatch demonstrations, one per check;
10. the gate output, before and after, including the dashboard's and inspector's derived figures;
11. the six manual assessments, or an explicit statement of which are outstanding and who owes them;
12. residual uncertainty, including everything under *Stop and escalate conditions* that remains open, the standing
    OUTSTANDING rows this change does not touch, and the statement that `VREC-MOK-013` is a separate commit-bound
    record that this work order does not write and cannot self-approve.
