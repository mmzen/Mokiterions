+++
id = "WO-MOK-005"
type = "work_order"
title = "Split the workspace and implement the terminal observer"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-17"
updated = "2026-08-19"

[assurance]
commit_bound_verification = "required"
rationale = "This work restructures the repository into two packages, takes the project's first external dependency at a measured surface of 57 crates, and promotes a read-only observation surface to a maintained public interface. It also introduces the instrument that later phases will use to assess behavior, so a defect in it would misinform every subsequent product judgement. The claim that observation cannot change a simulation outcome is a determinism claim of the same standing as REQ-MOK-009 and must be bound to a commit rather than asserted."
decided_by = "engineering owner"

[relations]
implements = [
  "REQ-MOK-019",
  "REQ-MOK-020",
  "REQ-MOK-021",
  "REQ-MOK-022",
  "REQ-MOK-023",
  "REQ-MOK-024",
  "REQ-MOK-025",
  "REQ-MOK-026",
  "REQ-MOK-027",
]
specifications = ["SPEC-MOK-003"]
architecture = ["ARCH-MOK-002", "ADR-MOK-003"]
verification = ["VER-MOK-005"]
+++

# Work Order: Split the workspace and implement the terminal observer

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

**This work order cannot be approved before its governing artifacts are.** It depends on `INT-MOK-004`, `CAP-MOK-004`,
and `REQ-MOK-019` through `REQ-MOK-027` being approved by the product owner; on `SPEC-MOK-003`, `ARCH-MOK-002`,
`ADR-MOK-003`, and the 2026-08-17 amendment to `ARCH-MOK-001` being approved by the technical owner; and on
`VER-MOK-005` being approved by the assurance owner. Every one of them was `draft` at the time this work order was
written. Preflight must report this exact work order as eligible before implementation begins.

It depends additionally on four amendments to `SPEC-MOK-002`, which is approved, implemented under `WO-MOK-003` and
bound by a `verified` `VREC-MOK-003`. `SPEC-MOK-003`'s *Compatibility and migration* section states them: rule 1's
"no second package, no workspace" narrowed to permit a workspace of exactly two packages; rule 3's clause freezing
`src/simulation.rs`'s contents scoped to the restructuring it was written for, so that an approved requirement may add
code to that file; rule 5's closed public enumeration grown by the observation surface; and rule 6's prohibition
narrowed from five named value types, and from the return-value path, to the capability it was written to deny.
**These four were not part of the 2026-08-17 approval recorded below and are outstanding.** They could not have been:
`SPEC-MOK-002` did not exist on this branch when that approval was given, and it reached `master` afterwards. They are
the technical owner's act, and this work order is not verifiable until they are given or the scope is changed to avoid
needing them. A fifth amendment, to `ARCH-MOK-001`, is outstanding for the same reason and is recorded in that
artifact's amendment record: the wording it narrows — "mutable **or owned** authoritative state" — also reached
`master` under `WO-MOK-003`, after the approval.

**Approval record.** On 2026-08-17 the repository owner, acting in all four accountable roles, approved the complete
governing chain — `INT-MOK-004`, `CAP-MOK-004`, `REQ-MOK-019` through `REQ-MOK-027`, `SPEC-MOK-003`, `ARCH-MOK-002`,
`ADR-MOK-003`, the `ARCH-MOK-001` amendment, and `VER-MOK-005` — and authorized this work order, which was
transitioned `draft` → `approved` → `in_progress` on the same date. The implementation agent recorded the
transitions; it did not make the decision.

Two of those approvals are load-bearing rather than procedural. `REQ-MOK-026` is the approved requirement that
`ARCH-MOK-001`'s prohibition on separate crates has always required as its unlock, and without it a second package is
prohibited outright. The `ARCH-MOK-001` amendment is what scopes its prohibition on user-interface frameworks to the
engine package, and without it the dependency this work order adds is prohibited outright. Neither can be assumed and
neither may be self-approved by the implementation agent. The same is true of the four `SPEC-MOK-002` amendments
above and of the `ARCH-MOK-001` amendment dated 2026-08-18, with one difference that matters: those two approvals were
given, and these five have not been.

`ARCH-MOK-001` is deliberately **not** selected in `architecture` above, and its exclusion is not an oversight. The
applicability rule is that architecture is selected when active architecture directly `addresses` a requirement the
work order implements. `ARCH-MOK-001` addresses `REQ-MOK-004`, `REQ-MOK-008`, `REQ-MOK-009`, and `REQ-MOK-010`, none
of which are in this scope. Its amendment is a prerequisite of this work order's approval rather than work performed
under it, and its amended conformance checks appear below as constraints on this work. Nominal coverage is therefore
omitted rather than fabricated. The technical owner confirms this omission at approval.

**Layout defect reported 2026-08-18; amendments approved and implementation directed 2026-08-19.** The repository owner reported that the observer's adaptive layout
does not present the left panel at some terminal sizes, and classified it as blocking for this work order while
accepting the rest of the observer as a first version. The owner's initial proposal was to remove adaptive layout
altogether and truncate the view when the terminal is too small. After being shown the measurement below, the owner
chose instead to replace `SPEC-MOK-003` rule 5's tier table with one pane threshold per axis. The implementation agent
measured the defect, drafted the amendments and recorded this paragraph; it decided nothing, and rule 5's thresholds
remain withheld from it by the *Authorized decision envelope* below, which is neither amended nor read down.

The measurement. Rule 5's tier table is an ordered ladder of four rows over a two-dimensional space, and it leaves a
gap: `W ≥ 140` with `38 ≤ H < 44` satisfies no row and falls to `otherwise`, which excludes the roster, the inspector
and the log together. A 160 × 40 terminal therefore presents no left panel, while 120 × 40 — the same height, one
third narrower — presents one, so making the window smaller adds panes back. `mokiterions-tui/src/layout.rs` lines 102
to 112 implement the table faithfully, and `mokiterions-tui/tests/layout.rs` line 37 asserts
`tier_for(140, 43) == Tier::D`. The behavior was specified, implemented and pinned by a passing test, so this is a
specification defect and not an implementation defect: correcting it was outside this work order's envelope until the
owner amended the specification, and the amendment is what admits the code change recorded below.

Why removal was not taken. `REQ-MOK-024` is approved and requires that "no content is clipped in a way that could be
read as a different value", that "a number that does not fit is not shown truncated", and that "degradation never
hides the run's provenance, since a screen capture that does not identify its own run cannot serve as evidence".
Truncating a fixed frame clips the footer, which rule 5 retains at every size for that exact reason and which
`VER-MOK-005` requires at every viewport. Removal would therefore have needed a product-owner amendment to an approved
requirement on top of the specification and contract changes. The threshold change needs no requirement amendment,
because `REQ-MOK-024` fixes no threshold itself and delegates every one of them to `SPEC-MOK-003`.

What this adds to the scope above. `mokiterions-tui/src/layout.rs` and its tests; the header's `tier` label in
`mokiterions-tui/src/render.rs` line 158, which loses its referent and is removed — that string is the
implementation's to choose under *exact diagnostic and pane-title wording*, no artifact requires it, and the overlay
announcement already names every absent pane with the key that opens it; and a monotonicity sweep in the observer's
test tier. Nothing else in the observer changes, and no engine file changes.

What landed. On 2026-08-19 the owner reviewed both drafted amendments, approved them and directed the implementation in
the same act: `SPEC-MOK-003` rule 5 as technical owner, `VER-MOK-005` as assurance owner, each recorded in its own
amendment record with the note that the implementation agent drafted the text and recorded the approval on the owner's
explicit instruction. The implementation is the per-pane thresholds in `mokiterions-tui/src/layout.rs`, the removal of
the header's `tier` label, and three added tests in `mokiterions-tui/tests/layout.rs` — the per-pane threshold case,
the ten-row log case and the monotonicity sweep, which compares 13,026 adjacent viewport pairs over `34..=200` by
`22..=60` and finds no pane that a larger viewport removes. The observer's test total rises from 109 to 112 and the
workspace's from 169 to 172. `layout-and-viewports.txt` under this work order's evidence is re-captured for the nine
declared viewports and carries the sweep and a negative control that shows the superseded table failing the same
cases.

What is outstanding. **`SPEC-MOK-004` rule 6 requires the technical owner.** Removing `Tier`, its `label`, `tier_for`
and the `Panes::tier` field changes an interface that rule 6 closes by provenance and whose growth clause had no
counterpart for removal, so the recorded extent falls from 97 items to 94 and the rule gains a **Reduction** clause;
rules 9, 11 and 13 follow it for the test counts and one stale term. The owner was not shown this consequence when
approving rule 5 — the implementation agent found it afterwards by measuring the interface — so it is recorded
**OUTSTANDING** in that specification's amendment record and reported rather than treated as covered. The three
2026-08-18 rows that were already **OUTSTANDING** in `SPEC-MOK-002` and `SPEC-MOK-003` are untouched and remain so.
`VREC-MOK-005` is `ready` rather than `verified`; it is not edited in place, and it is re-captured against the commit
that carries this implementation rather than carried forward. No lifecycle status is moved by the implementation
agent, and nothing here is committed, pushed or merged without the owner's instruction.

Terminology left alone. `REQ-MOK-028`, `CAP-MOK-005`, `INT-MOK-005`, `WO-MOK-006` and `VER-MOK-006` each use "layout
tier" as a name for what a viewport yields, in clauses whose subject is that the restructuring changed nothing an
operator can see. Those clauses were true at `VREC-MOK-006`'s commit and are not re-opened here; what they now lack is
a definition of the term, which `SPEC-MOK-003` rule 5 no longer supplies. This is terminology drift, not a defect, and
it is reported to the owner rather than amended under a work order that has no requirement to touch them.

**Moved to `implemented` on 2026-08-19**, on the repository owner's explicit instruction after pull request **#15** was
merged. The implementation agent recorded the transition; it did not make the decision.

The change is complete and the evidence `VER-MOK-005` requires is retained under
`docs/engineering/simulation/evidence/WO-MOK-005/` — 21 files, indexed by its `README.md` against each of
`VER-MOK-005`'s thirteen retention bullets, and reported in `completion-summary.md` with seventeen numbered
disclosures. **One of those thirteen bullets is discharged by a file that records nothing performed.**
`manual-assessment.md` states that all seven manual assessments are **OUTSTANDING** and that it has no author for any
of them; three are unperformable from the implementation environment because `crossterm` on Windows reads the console
input buffer rather than standard input, which that file measures rather than asserts. Every automated case in the
packet is a claim about an in-memory character buffer, so nothing in it establishes that a person can read the result
on a terminal. This transition records that the change is complete, not that the instrument was assessed by eye.

What the 2026-08-19 work implemented is rule 5 as amended: one pane threshold per axis, measured in
`mokiterions-tui/src/layout.rs` as the roster at `W ≥ 100`, the inspector at `W ≥ 140`, the log at `H ≥ 38`, ten log
rows at `W ≥ 140` and `H ≥ 48` against six otherwise, and the floor refusing below `34 × 22`. It replaces the
four-row tier ladder whose gap left a 160 × 40 terminal with no left panel while 120 × 40 had one. `Tier`,
`Tier::label`, `tier_for` and the `Panes::tier` field are deleted, the header's `tier` label with them, and
`mokiterions-tui/tests/layout.rs` gains three cases including the monotonicity sweep: **13,026 adjacent viewport pairs
over `34..=200` by `22..=60`, 0 of them removing a pane as the viewport grows**, against **12** for the superseded
table transcribed from `master` as a negative control.

Gates on the merged tree at `0ffe79c`, whose tree is byte-identical to the final candidate `a53712c`: `cargo fmt --all
-- --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit `0`, and clippy again for
`-p Mokiterions` alone; `cargo test --workspace` reports **172 passed, 0 failed, 0 ignored** across 19 suites, 60 in
the engine and 112 in the observer; `harnessctl validate` PASS at 68 / 0 / 0 with all four planes clean; `doctor` PASS
at 81 lines with every managed file unchanged; `preflight --work-order WO-MOK-005 --phase review` PASS; the dashboard
PASS at 68 artifacts, 211 relations, 0 errors and 8 warnings, two more than a clean `origin/master` extraction at
05dc6ac reports and both of them one finding stated twice. The additivity claim holds at package level rather than at
test level: **no file in the engine package differs from `origin/master` at 05dc6ac**, so no simulation rule and no
engine test changed under this work.

Committed on 2026-08-19 to `feature/wo-mok-005-layout-axes`, branched from `master` at 05dc6ac. `360a5e4` carries the
implementation; `7fa037b` re-bases the evidence citations onto the engine `WO-MOK-006` moved; `6ef5d95` corrects a
dashboard-digest claim; `f361370` corrects three evidence figures; `3696fae` re-captures `VREC-MOK-005`; `a53712c`
transitions that record to `verified`. Merged to `master` as `0ffe79c`, a merge commit rather than a squash, so
`f361370` — the commit the record binds — is an ancestor of `master` and the record's provenance still resolves. The
pull-request-event check on `a53712c` failed, and its cause was outside the candidate: the managed selector
`scripts/select_harness_work_order.py` matches `Harness-Work-Order:` with a trailing `[ \t]*$`, and the pull-request
body had been uploaded from a Windows file with CRLF endings, so the `\r` defeated the match and no work order was
selected. The body was normalized to LF, the push-event run on the same commit passed every candidate check, and the
run on the merge commit is green.

`VREC-MOK-005` binds `f361370` and was transitioned `ready` → `verified` by the repository owner as accountable
assurance owner on 2026-08-19, again recorded by the agent rather than decided by it. **That transition preceded this
one**, as `WO-MOK-006`'s did, and for the same reason it changes nothing: `harnessctl validate` reports no error and no
warning against a `verified` record on an `in_progress` work order, and verification is carried by the record rather
than by this status. Two of the six chains now verify before they implement.

Four sentences in two commit-bound `verified` records go stale on this transition and none is edited. `VREC-MOK-005`
opens *What this record claims* with "`WO-MOK-005` is `in_progress`", records `in_progress` in its preflight row, and
states that "`WO-MOK-005` remains `in_progress`, as `WO-MOK-006` does"; `VREC-MOK-006` states that "`WO-MOK-005` and
`WO-MOK-006` are both still `in_progress`". Each was true at the commit its record binds and at its verification, and
the convention established at `WO-MOK-006`'s transition is that bound records stay as they are rather than track later
state. The *What is outstanding* paragraph above is left standing for a weaker version of the same reason: it says
`VREC-MOK-005` is `ready`, which was true when it was written and until the owner verified the record on 2026-08-19,
and it is a record of what was outstanding at implementation rather than a present-tense claim. Correcting it in place
is available on the owner's word; it is not taken here, because this transition was authorized and a rewrite of the
implementation narrative was not.

After the transition, on the tree that carries it: `harnessctl validate` and
`scripts/validate_engineering_artifacts.py` both PASS at 68 / 0 / 0 with all four planes clean, `doctor` PASS at 81
lines with no line of any other severity, and `preflight --work-order WO-MOK-005 --phase review` PASS, now reporting
`implemented`. Two derived figures move, and both are mechanical consequences of the status rather than new defects.
The default `start` phase turns `FAIL` with `[W005] status 'implemented' is not eligible for start; expected one of
approved, in_progress`, which is preflight refusing to begin work that is already done and is the correct answer. And
the dashboard goes from 8 warnings to **9**: the ninth is `W-HEX-001`, *"WO-MOK-005 is implemented but has no evidence
document keyed to its ID"*, which fires only on an `implemented` work order and which each of the other five already
carries — `WO-MOK-006` included, with its 67 retained files. The catalog holds nine artifact types and `evidence` is
not among them, so that finding is about a typed document this repository's convention never creates rather than about
the 21 files retained under this ID. `harnessctl inspect` now reports `Active work (0): none` beside
`Assurance pending (0): none`.

What is outstanding is unchanged by this transition and is not made smaller by it. **Eleven amendment provisions across
four approved artifacts await the technical owner** — `SPEC-MOK-002` rules 1, 3, 5 and 6; `ARCH-MOK-001`'s 2026-08-18
row; `SPEC-MOK-003`'s *Data and interface contracts* rule 2; and `SPEC-MOK-004` rules 6, 9, 11, 12 and 13, where rule
6's recorded extent falls from 97 items to 94 and the rule needs a **Reduction** clause it does not have. The seven
manual assessments remain unauthored. The terminology drift recorded above is unamended. This work order stays
`implemented`. Nothing here tags, releases, publishes or deploys anything, and no release record exists.

## Objective

Restructure the repository into two packages and implement a terminal observer over a read-only engine surface,
exactly as specified by `SPEC-MOK-003`, governed by `ARCH-MOK-002` and `ADR-MOK-003`, and covered by `VER-MOK-005`,
without changing any simulation rule or any verified engine behavior.

## In scope

- Convert the repository to a Cargo workspace with exactly two packages: the engine package `Mokiterions` at the
  root with its sources in their existing location, and the observer package `mokiterions-tui` as a member.
- Keep the engine package's name and both of its target names exactly as `SPEC-MOK-002` rules 1 and 2 fix them, so
  that no operator-facing string and no verified test changes for a cosmetic reason.
- Extend the engine's existing library target with the read-only observation surface of `SPEC-MOK-003`: `snapshot`,
  `advance_tick`, `is_finished`, `termination_reason`, `configuration`, `initialization_events`, and the snapshot
  types, all owning their data. The target itself is already there under `ADR-MOK-002`; what this work order does is
  grow its enumerated public interface, which `SPEC-MOK-002` rule 5 provides for on an approved requirement, and
  rule 5's third list is where the grown enumeration is written.
- Route the engine's whole-run entry point and the observer's single-tick advance through one internal step, so the
  command-line host and the observer are peer hosts of one interface rather than two paths through the engine.
- Add `ratatui` version `0.30.2` with `default-features = false` and features `crossterm`, `layout-cache`,
  `underline-color` as the observer package's only external dependency.
- Implement the observer: start-up input handling including `--speed`, `--start-paused` and `--export`; the spatial
  view in both zooms with the specified mapping, orientation, glyphs and shared-cell handling; the territory resource
  headline; the roster; the inspector; the event log with filtering and export; the provenance footer; the authority
  mapping; the tier-based layout with its floor, announcements and resize behavior; the key bindings; and
  unconditional terminal restoration on every exit path.
- Implement progression under rule 1: single-tick advance only, held at completed-tick boundaries, at most one tick
  per scheduling opportunity, and refusal to advance a finished run.
- Add automated tests covering every case, invariant and check in `VER-MOK-005`, including buffer assertions at every
  declared viewport and observed-versus-unobserved comparison on every declared seed.
- Record and retain the evidence `VER-MOK-005` lists under this work-order ID.

## Out of scope

- Any change to a simulation rule, constant, event type, event field, field order, exit code, text-stream line format,
  trace line, or summary. This work is additive to `SPEC-MOK-001` and changes nothing it fixes.
- Relocating the engine's sources. Their position is deliberately preserved so the `REQ-MOK-010` text stream is not
  disturbed.
- Any third package, service, network boundary, or separate release artifact.
- Fear, traits, names, combat, memory, model-provider integration, credentials, prompts, or per-agent entropy — every
  attribute the target design anticipates and the engine does not compute. `SPEC-MOK-003` reserves space for one of
  them and requires that space to render empty.
- Serialization, persistence, asynchronous runtimes, threads sharing simulation state, databases, and networking.
- Mouse input, configuration files, environment-variable configuration, and a standard-input protocol.
- Graphical or web interfaces.
- Reading the engine's text output back in. The observer obtains state from the surface, never by parsing.
- Structured or machine-readable simulation output, aggregate multi-run analysis, and outcome classification.
- Changes to approved behavior or artifact lifecycle outside this work order.

## Authorized decision envelope

The implementation agent may choose:

- private Rust type, function, module and file names in both packages, and how rendering is decomposed;
- how snapshots are built internally, provided the specified content, ordering and ownership hold;
- the concrete widget used for each pane, provided the specified content, constraints and announcements hold;
- exact diagnostic and pane-title wording, and the exact palette, provided every distinction remains available
  without colour;
- internal error types, and how terminal restoration is guaranteed on the panic path;
- test organization, fixtures and helpers, and whether layout tests construct a terminal or call layout directly;
- whether the reserved fourth roster bar is reserved by layout arithmetic or by a placeholder that renders nothing;
- comments and non-authoritative developer documentation.

The implementation agent may **not** choose: the dependency, its version, or its feature set; the package layout,
package names, binary names, or dependency direction; the coordinate mapping or orientation; the fidelity thresholds,
the tier table, or the floor; the glyph assignments; the key bindings; the event buffer capacity; the export format,
export path resolution, or filter semantics; the authority mapping; the snapshot contract or the number of mutating
operations on it; any figure fixed by `SPEC-MOK-001`; or any artifact lifecycle status.

Adding any external dependency beyond the one named above requires escalation and an updated architectural decision.
This includes a dependency that would appear only in the observer package, and it includes any dependency added to
make a test easier to write.

## Constraints

- Follow `INT-MOK-004`, `CAP-MOK-004`, `SPEC-MOK-003`, `ARCH-MOK-002`, `ADR-MOK-003`, and `ARCH-MOK-001` as amended.
- Preserve everything verified under `VREC-MOK-001` and `VREC-MOK-002`. Every existing test must pass unmodified. A
  test that must change to accommodate this work is a stop condition, not a task, because this change is specified as
  additive.
- Keep the engine package's external dependency set **empty**, with no exception, including a dependency shared with
  the observer package.
- Keep every user-interface dependency in the observer package's manifest and in no other.
- Add no dependency edge from the engine package to the observer package.
- Keep the engine package buildable and testable with no terminal present and with the observer package excluded.
- Use no network, model, credential, persistence, serialization, async-runtime, or database dependency in either
  package.
- Let the observer hold no mutable handle to world, agent, resource, or event-log state, and expose no operator
  control that mutates the world.
- Keep wall-clock time confined to deciding when to draw and when to advance. It must reach no engine input and no
  displayed authoritative value.
- Let the observer consume no simulation entropy.
- Re-derive no engine verdict in the observer. The displayed outcome is the engine's outcome or it is a defect.
- Present no value the engine does not compute, including an inert placeholder that reads as a computed zero.
- Read no repository file and invoke no version-control command at run time.
- Never open `--export` at start-up; validate it as a string only.
- Preserve unrelated user changes and do not modify generated output as source.

## Expected change surface

- `Cargo.toml` at the root: a workspace table only. The package name, the library target and the binary target are
  unchanged, and the dependency table stays empty.
- `Cargo.lock`: from an empty dependency set to a workspace including the observer's 57-crate surface.
- The engine's existing sources: the public observation surface added to the library target `WO-MOK-003` established,
  and the routing of the whole-run entry point and the single-tick advance through one internal step. No simulation
  rule changes, and no change to `src/main.rs` or to the tests `WO-MOK-003` placed in `tests/`.
- A new `mokiterions-tui/` directory with its manifest, sources and tests.
- Automated tests in both packages.
- Work-order-keyed evidence under `docs/engineering/simulation/evidence/WO-MOK-005/`.
- `docs/mokiterions/ROADMAP.md`, to record the observer phase.
- The amendments this work order's approval is conditioned on, applied in the artifacts they belong to and recorded in
  their amendment records with their approval status stated: `SPEC-MOK-002` rules 1, 3, 5 and 6; `ARCH-MOK-001`'s
  2026-08-18 row; and a dated note in `ADR-MOK-002`'s *Status* section, which refined-not-superseded is what
  `ADR-MOK-003` does to it. Writing the amended text is implementation; approving it is not, and each record says
  which of the two happened.
- `docs/engineering/REPOSITORY_CONTEXT.md`, whose *Architecture* and *Repository constraints* sections describe a
  single-package repository with no user interface. It is repository-owned guidance, not a governed artifact, and it is
  brought into line rather than amended.
- No other product domain and no harness-managed policy file.

## Required verification

- Complete every case, invariant, check and manual assessment in `VER-MOK-005`.
- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test` at the workspace root, and `cargo test -p Mokiterions` alone.
- Run `cargo build` at the workspace root, and `cargo build -p Mokiterions` alone.
- Run `cargo tree -p Mokiterions` and retain the output as the empty-dependency-set proof.
- Retain the observer package's resolved dependency graph, its crate count, and its enabled feature set.
- For each declared seed `0`, `1`, `42`, `123`, `777`: run the engine binary to completion, run the same
  configuration through the observer under the scripted interaction sequence, export, and compare byte for byte.
- Compare per-tick entropy draw counts observed and unobserved for each declared seed.
- Assert the character buffer at each declared viewport `160 × 48`, `160 × 44`, `140 × 44`, `120 × 48`, `100 × 30`,
  `34 × 22`, and capture the refusal output at `33 × 21`.
- Run one `10,000`-tick observed run at speed `64`.
- Demonstrate terminal restoration on normal exit, on error exit, and on panic.
- Perform and record the manual assessments, including the legibility and colour-independence assessments.

## Evidence to record

Retain, under `docs/engineering/simulation/evidence/WO-MOK-005/`, every item listed in `VER-MOK-005`'s evidence
retention section. The `cargo tree -p Mokiterions` output and the per-seed observed-versus-unobserved comparison
are the two records that carry the load: the first is the only proof that the engine's dependency set survived this
change, and the second is the only proof that observation did not perturb a run. No screenshot or recording is
admissible for any rendering obligation; retain buffer dumps accompanying their assertions instead.

## Stop and escalate conditions

Stop before implementation if preflight does not report this exact work order as eligible, or if any governing
artifact remains `draft` or unapproved — in particular `REQ-MOK-026` and the 2026-08-17 `ARCH-MOK-001` amendment,
without which a second package and a user-interface dependency are both prohibited; the four `SPEC-MOK-002`
amendments, without which a workspace, new code in `src/simulation.rs`, the grown public enumeration and the five
public value types are each prohibited by a specification whose verification record is already `verified`; and the
2026-08-18 `ARCH-MOK-001` amendment, without which returning an owned snapshot is prohibited by an approved
architecture.

During implementation, stop and escalate if:

- an observed run and an unobserved run differ in any authoritative event or in final state on any declared seed. The
  non-perturbation property is the point of the design, so a difference means the design or the implementation is
  wrong and must be corrected rather than tolerated or documented;
- per-tick entropy draw counts differ observed and unobserved;
- the specified layout arithmetic does not close at any declared viewport, or a canvas interior does not match the
  figure `SPEC-MOK-003` derives. The specification is the authority, so a mismatch requires an amended specification
  and re-approval, never a quietly adjusted constraint or a relaxed assertion;
- the whole world cannot be presented at one dot per world cell at a viewport where `SPEC-MOK-003` says it can;
- the observation surface cannot be exposed without leaking a reference into engine state, an interior-mutable value,
  or a second mutating operation;
- the engine package cannot be kept free of external dependencies, or the observer's dependency cannot be confined to
  its own package;
- the resolved observer dependency graph does not match the specified version and feature set, or exceeds the
  measured 57 crates;
- any dependency beyond the one specified appears necessary, for any reason including test ergonomics;
- rendering cannot be asserted from an in-memory buffer, since verification then has no admissible evidence and the
  whole approach to covering presentation fails;
- an existing `VER-MOK-001` or `VER-MOK-002` test fails, or would need to be modified;
- terminal restoration cannot be guaranteed on the panic path;
- a value the target design shows cannot be rendered because the engine does not compute it. Such a value is out of
  scope above and must be left absent; inventing a placeholder for it is forbidden, and if it is judged necessary
  that is a product decision;
- specified behavior is contradictory or needs a product or technical decision;
- required verification fails and cannot be corrected within authorized scope;
- requested changes expand into an excluded feature or another artifact domain.

## Completion report format

Report:

1. implemented requirements and affected components;
2. any authorized local decisions, including how rendering was decomposed and how panic-path restoration is
   guaranteed;
3. verification commands and results;
4. the `cargo tree -p Mokiterions` output, and the observer's crate count and enabled feature set as resolved;
5. the per-seed observed-versus-unobserved comparison result and the interaction sequence performed;
6. the per-viewport canvas interiors as measured, against the figures `SPEC-MOK-003` derives;
7. retained evidence paths;
8. residual limitations and explicitly deferred features, including every attribute of the target design left absent;
9. final worktree and candidate-commit status.
