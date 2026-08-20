+++
id = "WO-MOK-011"
type = "work_order"
title = "Give each Mokiterion a name, report it once, and present it wherever a Mokiterion is identified"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-19"
updated = "2026-08-20"

[assurance]
commit_bound_verification = "required"
rationale = "The work adds a field to a record two test suites parse positionally and changes the map's glyph alphabet, and its load-bearing claim is a claim of absence: that every reported value except the added one, every outcome and the shared entropy stream's position are exactly what they were. That claim is of the same standing as REQ-MOK-009's determinism claim — it cannot be asserted, only checked against a capture bound to the commit the work began from. A naming step that drew one value from the shared stream would pass every behavioral test and silently retire REQ-MOK-014's and REQ-MOK-034's measured survivor floors. The second claim, that no existing test's assertions changed, is likewise checkable only against a census taken before the change."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-040", "REQ-MOK-041"]
specifications = ["SPEC-MOK-001", "SPEC-MOK-003"]
verification = ["VER-MOK-011"]
+++

# Work Order: Give each Mokiterion a name

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and the retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

No `architecture` relation is declared. No active architecture carries an `addresses` edge to `REQ-MOK-040` or
`REQ-MOK-041`: `ARCH-MOK-001` addresses `REQ-MOK-004`, `REQ-MOK-008`, `REQ-MOK-009`, `REQ-MOK-010` and `REQ-MOK-016`,
and `ARCH-MOK-002` addresses `REQ-MOK-021`, `REQ-MOK-025`, `REQ-MOK-026` and `REQ-MOK-028`. Architecture is therefore
not applicable and fabricating coverage would be worse than omitting it. That is a statement about the traceability
rule, not a claim that the change is architecturally weightless: it adds a field to a public enum's payload, so
`ARCH-MOK-001`'s component boundaries, its prohibition on any public item exposing a mutable borrow into authoritative
state, and its prohibition on any engine dependency are all in force and are checked under `VER-MOK-011`. **The
technical owner must confirm at approval that `ARCH-MOK-001` requires no amendment.**

**This work order cannot be approved before its governing artifacts are.** It depends on `INT-MOK-008`, `CAP-MOK-008`,
`REQ-MOK-040` and `REQ-MOK-041` being approved by the product owner, and on `VER-MOK-011` being approved by the
assurance owner. **All five, and this work order, were approved by the repository owner on 2026-08-19**, acting as
product owner, technical owner and assurance owner; the same act confirmed that `ARCH-MOK-001` requires no amendment,
on the ground that the added field carries an owned value and grants no borrow into authoritative state, which is what
its prohibition addresses.

Status moved to `in_progress` on 2026-08-19, in the same commit as the first code change, on the owner's instruction to
implement. The pre-change capture named under *Evidence* was taken **before** that change, from a tracked-clean
worktree at the commit recorded in `evidence/WO-MOK-011/baseline/COMMIT.txt`, because a capture taken afterwards could
not establish the absence claim this work order rests on.

### Transition to `implemented`

**Status moved from `in_progress` to `implemented` on 2026-08-20**, on the repository owner's instruction, given as
engineering owner: "you can also transition WO-MOK-011 as implemented and push it to same branch". The implementation
agent recorded the transition and did not decide it. The status had stood at `in_progress` through implementation,
through the renumbering from `010`, through the verification capture, through the merge of `master`, and through this
chain's verification record moving to `verified` one commit earlier, because no earlier instruction covered a lifecycle
transition and this work order's *Authorized decision envelope* does not place one inside the agent's local decisions.

**The order is the reverse of the `WO-MOK-010` pair's, because the instructions arrived in that order.** There, the
work order was set to `implemented` first (`084b608`) so that the verification record could state the work order's
final status instead of going stale one commit later. Here the owner validated the record first, so `VREC-MOK-011`
moved to `verified` at `965fe67` while this file still read `in_progress` — and said so, in three places. Those three
statements are corrected in the same commit as this transition rather than left stale, each keeping what it said with
the later fact beside it.

**What is implemented.** Everything under *In scope*: the twelve-name table and its total, injective, fixed assignment;
the name stored at initialization, drawing nothing from any generator; the name reported as the first detail of every
`agent_initialized` record and on no other record; the observer's identifier-to-name map built from the initialization
records it already ingests and presented in the roster, the inspector and the map; the glyph derived from the name's
first character uppercased at both zoom levels; the inspector naming a dead selection; the five `SPEC-MOK-001` and
three `SPEC-MOK-003` provisions written as approved amendments, each with an amendment-record row; every check,
measurement and retained item `VER-MOK-011` requires; `SIMULATION_RULES.md` including its §14 worked example; and the
roadmap entry. Between `524a675` and the candidate commit that is nine non-documentation files, 974 insertions and 65
deletions.

The candidate commit is `9ddcf83fd460880ce25fc6548c768189bb3a5795`, whose gates are recorded in `VREC-MOK-011` and
reproduced from `evidence/WO-MOK-011/gates.txt`: **205 test names, 205 passed, 0 failed, 0 ignored, 0 filtered out** in
one workspace invocation; `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features --
-D warnings` clean in both packages; the engine's `cargo tree` still a single line; the public interface unchanged on
both sides at both packages — observer 101 items, 24 public fields, 125 `pub` lines and engine 49, 43 and 92, pre and
post identical (`interface.txt`); and validator PASS at 83 artifacts with 0 errors and 0 warnings across all four
planes.

**This status covers the merge resolution too, and no record binds that.** The lineage is `524a675` → `52b41c8`
(implementation and evidence) → `a44a388` (the first record) → `9ddcf83` (the renumbering) → `88ea456` and `1013470`
(two merges of `master`) → `master` at `dec1b95` through pull request #22. `evidence/WO-MOK-011/merge/` records the
thirteen conflict regions in four files and each resolution — including the six inherited `entry_lines` call sites
given a name argument — and re-derives oracle 3 (`master` 200 → **212** names, 0 removals, 0 renames) and both halves
of oracle 5 at the merged tree. **Oracles 1 and 2, oracle 4's rendered buffers and the mutation control were not
re-derived there and are still owed**, and a verification record bound to the merge commit is owed with them, as a new
record rather than an edit of `VREC-MOK-011`.

**What this status does not do.** It does not verify the work: `WORKFLOW.md` states that the verification record moves
separately through an accountable human decision and that work-order status never substitutes for it, and
`VREC-MOK-011` binds `9ddcf83` rather than the merged tree, so `master` carries this work unverified. It does not
perform manual assessment 5 of `VER-MOK-011` — the assurance owner's judgement on `baseline/projection.py` — which
`evidence/WO-MOK-011/manual-assessment.md` still records as **OUTSTANDING**, so `VER-MOK-011` is still not satisfied,
on one count and one only. It does not release anything: no release record binds this work, and `RLS-MOK-001` released
0.1.0 from `755db72`, which does not contain `9ddcf83`.

**Three derived figures move with the status, measured rather than predicted.** The harness dashboard goes from 6
warnings to 7; the seventh is a `W-HEX-001` against this work order, the same warning `WO-MOK-010` already carries.
`discover_evidence` in `scripts/generate_harness_dashboard.py` matches `^(WO-[A-Z0-9-]*\d{3})(?:-|\.|$)` against each
**file's own name**, and no file in the 8.2 MB `evidence/WO-MOK-011/` packet is named for the work order — the
identifier is the directory's — so the packet is invisible to discovery. `inspect` goes from 18 findings to 19 and
drops *Active work* from 1 to 0, so its recommendation to continue bounded work on a change that is complete is gone;
*Suggested next steps* stays at 8 source observations while its printed lines fall from 4 to 3, the `W-HEX-001` line
absorbing the second observation. The validator does not move: PASS at 102 artifacts, 0 errors and 0 warnings across
all four planes, before and after.

**Disclosed rather than repaired.** `evidence/WO-MOK-011/analysis/amendments.py` and
`evidence/WO-MOK-011/merge/amendments-vs-master.py` — oracle 5's governance half at the candidate tree and at the
merged tree — both expect this file to read `in_progress`, which was true at the commits they were captured against.
Re-run after this move, each reports that one control failing and **exactly one line of its report changes**: the
chain-status row, from `in_progress` / yes to `implemented` / **no**. The merged-tree run therefore goes `RESULT:
PASS` → `FAIL` on that single control. The candidate-tree run was already `FAIL` on the merged working tree for a
reason that predates this commit and is `master`'s, not this work order's — it measures against `524a675`, and
`SPEC-MOK-002` and `VREC-MOK-007`, both of which it requires to be unchanged, moved on `master` after that commit
while the candidate tree leaves both blobs identical to the base. Neither retained report is edited to track a status
that moved after it was taken. `evidence/WO-MOK-011/assurance-decision.md` records the measured before and after.

### Why this chain is numbered 008, 040, 041 and 010

The identifiers in this chain are not consecutive with `INT-MOK-006`'s, and the reason is recorded here rather than
left to be discovered.

This work was drafted as `INT-MOK-007`, `CAP-MOK-007`, `REQ-MOK-035`, `REQ-MOK-036`, `VER-MOK-008` and `WO-MOK-008`.
Every one of those identifiers is already taken by unrelated work in flight in this repository:

- `master` holds a `WO-MOK-007`, `VER-MOK-007` and a `verified` `VREC-MOK-007` for *Colour the roster survival bars by
  value band*.
- `feature/phase-2-individuality`, which this branch is built on, holds a `WO-MOK-007`, `VER-MOK-007` and
  `VREC-MOK-007` for the individuality work, and the owner has begun renumbering that chain to `008`.
- `feature/release-ci` holds an approved `INT-MOK-007`, `CAP-MOK-007`, `REQ-MOK-035`, `REQ-MOK-036`, `REQ-MOK-037`,
  `VER-MOK-008`, a `draft` `WO-MOK-008` and an `implemented` `WO-MOK-009` for the release-authorization work.

An identifier collision between branches is what `evidence/WO-MOK-007/renumbering.md` records the owner resolving once
already, and it records that renaming approved artifacts is the owner's act and not an implementation agent's. This
chain therefore takes the next identifier above every number used on every branch, in every family, so that it can be
merged in any order relative to the other two without renaming anything: `INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`,
`REQ-MOK-041`, `VER-MOK-011`, `WO-MOK-011`, and `VREC-MOK-011` when the verification record is written. The gap this
leaves in the sequence is deliberate and costs nothing; a second collision would cost a renaming of approved artifacts.

**This branch cites the individuality chain by the numbers it carries in this tree** — `WO-MOK-007`, `VER-MOK-007`,
`VREC-MOK-007`, `evidence/WO-MOK-007/`. If the owner completes the renumbering to `008` before this work is merged,
those citations here and in this chain's other artifacts must be updated with it. They are not written to the
renumbered names in advance, because a citation to an artifact that does not exist in the tree is worse than a citation
that a recorded act will move.

### Decision record

On 2026-08-19 the repository owner, acting as product owner and technical owner, took the decisions this packet was
drafted around. They are design decisions, not approvals.

| Decision | Outcome | Role |
|---|---|---|
| Do Mokiterions get names at all | Yes, one each, from a fixed table of twelve | product owner |
| Where the names come from | Hardcoded in the engine, invented, not seed-derived and not operator-supplied | product owner |
| The twelve names and their assignment | The table in `SPEC-MOK-001`'s *Name* subsection, fixed below | product owner |
| Whether the name replaces the identifier | No. `M01`–`M12` are unchanged everywhere, and the observer presents both | product owner |
| Where the name sits in the initialization record | First, before `position`, with `waste_tolerance` remaining last | technical owner |
| How the name reaches the observer | Through the retained event stream, not through a new public interface item | technical owner |
| The map glyph | The name's first character, uppercased, as `SPEC-MOK-003` rule 2 already anticipated | technical owner |

Three of these were reached against the implementation's first proposal and are recorded because the reasoning matters:

**The field goes first, not last.** The obvious placement — appending the name after `waste_tolerance` — breaks
`mokiterions-core/tests/viability.rs`, which reads the trait with `rsplit_once(",waste_tolerance:")` and requires the
trait to remain the record's last field. `mokiterions-core/tests/process.rs` separately asserts `,fear:0,waste_tolerance:`
as an adjacent pair. Placing the name first satisfies both without touching either, and puts identity at the head of the
record where a reader looks for it. `INT-MOK-008` records that a test which must change is a stop condition here, so
this is a constraint rather than a preference.

**The observer reads the event stream, not the snapshot.** Adding `name` to `AgentSnapshot` would have been the shorter
route and was declined for three reasons: it grows the engine's public interface, which `SPEC-MOK-002` rule 5 holds to
what approved requirements need; it would require a `SPEC-MOK-002` amendment, which this way needs none; and it cannot
name a **dead** Mokiterion, which `SPEC-MOK-003` rule 10.6 requires the inspector to keep presenting. The event-stream
route is the one `SPEC-MOK-002`'s 2026-08-19 amendment established for `waste_tolerance`, and `REQ-MOK-022` already
obliges the observer to retain the stream.

**The name does not join `Observation` or `PerceivedMokiterion`.** `SPEC-MOK-001` rule 3 refuses to carry `fear` into
the observation because no rule reads it. A name nothing reads would be the same inert value arriving by the same route,
so it stays out.

### Required amendments to approved specifications

This work order depends on amendments to two specifications that are already approved. They are stated here in full,
and each requires the technical owner's separate act: approving this work order does not approve them. The
implementation agent writes the amended text and records in each amendment record that it did so; it does not decide the
substance.

**`SPEC-MOK-001`** — the behavior authority. Five provisions:

1. *State model → Mokiterion*. The contents list gains the name, stated as fixed for the run and for every run, and
   stated as carrying no behavior.
2. *State model*. A new *Name* subsection fixes the twelve names and their assignment to the twelve identifiers, fixes
   the character and length domain, and states the three properties the requirement rests on: the assignment reads
   neither the seed nor the configuration, the naming step consumes nothing from the shared entropy stream, and no rule,
   decision source, ordering, tie-break or termination condition reads a name. The twelve, in identifier order, are:

   | Identifier | Name | Identifier | Name |
   |---|---|---|---|
   | `M01` | `Zug` | `M07` | `Hozz` |
   | `M02` | `Krul` | `M08` | `Nurb` |
   | `M03` | `Quib` | `M09` | `Vonk` |
   | `M04` | `Sput` | `M10` | `Gorm` |
   | `M05` | `Trok` | `M11` | `Xob` |
   | `M06` | `Womp` | `M12` | `Drix` |

   The twelve first characters — `Z K Q S T W H N V G X D` — are pairwise distinct, which is an obligation rather than
   an aesthetic: `REQ-MOK-041` derives a one-character glyph from them and `SPEC-MOK-003` rule 2.5 requires identity to
   survive the loss of colour. None of them is `A`, `B`, `F` or `M`, the letters the output already uses for the two
   territory labels, the resource identifier prefix and the Mokiterion identifier prefix.
3. *Time and entropy*. The sentence qualifying that trait derivation is outside the shared stream is extended: naming is
   outside it too, and for a stronger reason — it performs no draw at all, against any generator.
4. *Data and interface contracts*. The `agent_initialized` record's field list gains `name` as its **first** detail,
   before `position`, and the amendment states that `waste_tolerance` remains the last detail because two test suites
   parse the record positionally from that end. The section's existing sentence that `waste_tolerance` "is reported
   once, in `agent_initialized`" gains the same statement for the name.
5. Rule 1, *Initialization order*. Where naming happens — with the rest of each Mokiterion's construction, before any
   initialization event is emitted — and that it draws nothing.

**`SPEC-MOK-003`** — the observer. Three provisions:

1. Rule 2's glyph table. `Mokiterion, M01–M09 → 1–9` and `M10–M12 → A, B, C` become the twelve name initials, and the
   paragraph stating that identifiers "carry no names" and that "when agent naming is introduced by a later phase, the
   glyph becomes the name's first character and this table is amended" is replaced by the derivation it anticipated.
   **The anticipation is retained in the amendment record rather than deleted**, because it is the argument that made
   the digit table correct for two phases.
2. Rule 4, the roster. The two-line mockup, the prose describing line one, and the collapsed one-line form all gain the
   name ahead of the identifier. The amendment must state the column arithmetic explicitly: the bar row's overhead of
   `5 + 4×6 + 3×2 = 35` and therefore `bar_width` are **unchanged**, because the name lands on line one and line one had
   slack. A reader who met a widened roster or a narrowed bar in this diff would be right to treat it as a defect.
3. Rule 10, the inspector. The presented-value list gains the name, and item 7's list of values the engine does not
   compute loses `name` while keeping age, kills, combats, remembered locations, model latency and per-agent entropy.
   Item 7's principle — that an uncomputed value is absent rather than blank-labelled or zero-filled — is unchanged and
   still governs the six that remain.

**`SPEC-MOK-002` is deliberately not amended.** Its rule 5 enumerates `simulation::EventDetail` as an "enum with
`event_type`" without enumerating any variant's fields, so a new field inside `AgentInitialized` adds no item to the
enumeration. `simulation::AgentSnapshot` is unchanged. Rule 6 is re-checked rather than amended: the added field is an
owned `String` on a value type, so no public item yields a mutable borrow of or a reference into authoritative state.
**That this specification needs no amendment is a claim, and `VER-MOK-011` oracle 5 checks it by comparing the interface
enumeration item for item rather than accepting it.**

## Objective

Give each Mokiterion one name from a fixed table of twelve, report it exactly once per Mokiterion in the initialization
record, present it in every observer pane that identifies a Mokiterion and derive the map glyph from it — and
demonstrate that the run itself did not move: no entropy drawn, no other reported value changed, no exit code changed,
and no existing test's assertions changed.

## In scope

- The twelve-name table in the engine, and the total, injective, fixed assignment from identifier to name.
- The name stored on each Mokiterion at initialization, drawing nothing from any generator.
- The name reported as the first detail of every `agent_initialized` record, and on no other record.
- The observer holding the names it ingests from initialization records, and presenting them in the roster, the
  inspector and the map.
- The map glyph derived from the name's first character, uppercased, in both zoom levels.
- The inspector naming a dead selection, which only the retained record can supply.
- The five `SPEC-MOK-001` and three `SPEC-MOK-003` provisions above, written as amendments the technical owner has
  approved, each with an amendment-record row.
- Every check, measurement and manual assessment `VER-MOK-011` requires, and the evidence it retains.
- Updating `SIMULATION_RULES.md`, including its §14 worked example, which traces one Mokiterion for fifteen ticks and is
  the sentence `INT-MOK-008` cites as the measure of the problem.
- Updating the functional roadmap with this phase's entry.

## Out of scope

- Any change to the `M01`–`M12` identifier, anywhere: in `subject=`, in the summary, in any specification, in any
  retained evidence file, or as the observer's join key.
- Any behavior that reads a name: no rule, no decision source, no validation, no ordering, no tie-break, no termination
  condition.
- Seed-varying names, operator-supplied names, renaming during a run, a `--names` option, and any name in any input.
- Names for resources, territories, events or the world.
- Any second attribute of personhood: age, pronouns, species, backstory, kills, memory.
- Any growth of the engine's public interface, including any new field on `AgentSnapshot`, `Observation` or
  `PerceivedMokiterion`.
- Any change to a decision source, the survival values, the resource table, the density mapping, the perception radius,
  regeneration, the finality of death, the exit-code contract, the default policy, the default density, or any
  requirement's floor.
- Any change to the roster's bar arithmetic, pane widths, layout thresholds or which panes exist.
- Combat, social behavior, model-backed decisions, structured output, persistence, batch execution.
- Any new package, target, build script or external dependency.
- Resolving `VREC-MOK-005`, or the amendment rows and manual assessments outstanding under it and under `VREC-MOK-007`.
- Any edit to a record bound by a `verified` verification record, and any edit to an existing amendment-record row.
- Restoring the three `master` artifacts that commit `524a675` did not carry forward. That is recorded under *Stop and
  escalate conditions* as a disclosure, and it is the owner's act.

## Authorized decision envelope

The implementation agent may decide locally:

- Rust file organization, private type and function names, and where a private helper lives.
- Whether the stored name is an owned or a borrowed static value inside the engine, provided the reported field is an
  owned value and the struct holding it acquires no lifetime.
- Which test tier each new test belongs to, applying `SPEC-MOK-004`'s rule as written: the public tier only if the test
  is writable through the library target's public interface with its assertions unchanged and no item widened. **Widening
  an item to `pub` in order to relocate a test is prohibited**, and a test that seems to need it belongs inline.
- The internal organization of a test module, and test helper functions.
- The exact wording of the amended text, once the technical owner has approved the substance, and the wording of the
  plain-language and roadmap updates.
- The form of the evidence files, the projection script and the census script, provided the projection is retained in
  full and reviewed.
- The exact cell layout of the roster's line one and the inspector's title, within what the rule 4 and rule 10
  amendments fix, and provided the bar row is unchanged.

The implementation agent may **not** decide:

- Any of the twelve names, or which identifier a name belongs to. That is a product decision, recorded above and fixed
  in `SPEC-MOK-001`.
- The name's position in the record.
- Whether the observer sources the name from the event stream or from a public interface item.
- Whether a name is presented anywhere `REQ-MOK-041` does not require it, or omitted anywhere it does.
- Any relaxation, widening, removal, rename or `#[ignore]` of an existing test, or any edit to the assertions in
  `tests/process.rs` or `tests/viability.rs`.
- Any `allow` attribute, any lint suppression, or any change to a gate.
- Any change to a constant, a floor, a default or an exit code.
- The substance of any amendment, or the addition of an amendment row for an amendment the technical owner has not
  approved.

## Constraints

- **The run does not move.** No draw against the shared entropy stream, no change to any other reported value, no change
  to any exit code, no change to acting order.
- **No existing test's assertions change.** This is a target with a stop condition attached, not an aspiration.
- The engine's dependency and dev-dependency tables stay empty. `cargo tree -p Mokiterions` resolves to one crate.
- The observer's dependency set stays at one path dependency and `ratatui` at its pinned version and feature set.
- The engine's public interface does not grow.
- Owned, reference-free values on every public item; no mutable borrow of or reference into authoritative state in any
  build configuration including tests.
- Integer and text operations only; no floating point anywhere in this change.
- Every rendering claim is asserted against an in-memory character buffer at stated cell positions. No screenshot, no
  recording, no terminal, no pseudo-terminal.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings` and `cargo test`
  at the workspace root all clean, in one invocation each, with nothing suppressed.
- No secret, credential or model-provider key enters the repository, in code, in a fixture or in retained evidence.

## Expected change surface

**`mokiterions-core/src/simulation.rs`**

- A `NAMES` table of twelve `&'static str`, and a function mapping an identifier's number to its name.
- `struct Mokiterion` gains a name field. The struct must acquire no lifetime parameter.
- `EventDetail::AgentInitialized` gains `name` as its first field, as an owned `String` matching every other public
  string field on that enum.
- The `Display` arm for that variant emits `name:{name},` ahead of `position:`.
- The agent-construction loop assigns the name; the initialization-event emission reports it.
- `AgentSnapshot`, `Observation` and `PerceivedMokiterion` are **not** touched.
- Internal-tier tests: the table's well-formedness properties, the assignment, entropy neutrality either side of
  naming, the stream position after initialization against `VER-MOK-007`'s recorded expectation, immutability across a
  run, and the absence of a reader.

**`mokiterions-tui/src/state.rs`**

- The observer gains a name map, populated from `EventDetail::AgentInitialized` in the existing ingest path, which
  already runs over the initialization events at construction.
- A lookup that returns the name for an identifier and presents nothing when there is none.
- An internal-tier test that all twelve are present after construction.

**`mokiterions-tui/src/spatial.rs`**

- The glyph function takes the name and returns its first character uppercased, with a stated fallback for the empty
  case that the engine cannot produce and the type system does not forbid.

**`mokiterions-tui/src/render.rs`**

- The map's Mokiterion draw passes the looked-up name to the glyph function.
- The roster entry's two-line and one-line forms carry the name ahead of the identifier. **`bar_width` and
  `BAR_ROW_OVERHEAD` are unchanged.**
- The inspector's title carries the name and the identifier, on the living path and the dead path alike.
- Internal-tier and public-tier tests per `SPEC-MOK-004`'s rule, asserting cell positions.

**Tests that must be updated because their subject changed, and only these:**

- `mokiterions-tui/tests/spatial.rs`, the glyph assignment case: the glyph alphabet changed, so the case asserts the new
  derivation. This is not a relaxation and the count of assertions does not fall.
- `mokiterions-tui/src/verification.rs` and `mokiterions-tui/tests/verification.rs`, the three call sites that pass an
  identifier to the glyph function.

Every other existing test is expected to pass untouched, and the census is what demonstrates it.

**Documentation:** `SIMULATION_RULES.md` and `../../../ROADMAP.md`.

## Required verification

`VER-MOK-011`, in full: its five oracles, its requirement-to-evidence matrix, its property and invariant tests, its
static, security, privacy, performance and resilience checks, and its seven manual assessments. A manual assessment
that is not recorded is outstanding, and the contract is not satisfied while any remains outstanding.

Oracle 1's pre-change capture is taken before the first code change, and a discrepancy is never resolved by recapturing
it.

## Evidence to record

Under `docs/engineering/simulation/evidence/WO-MOK-011/`, everything `VER-MOK-011`'s *Evidence retention* section
lists. In particular:

- `baseline/COMMIT.txt`, the commit the pre-change capture was taken at, with the worktree tracked-clean at capture;
- `capture.sh`, `baseline/projection.py`, `baseline/compare.py`, `baseline/retain.py` and `analysis/census.py`,
  retained in full so that every figure is reproducible from a recorded command rather than trusted;
- `baseline/pre-manifest.txt` and `post/post-manifest.txt`, the per-cell digests of both captures;
- `baseline/init/` and `post/init/`, the initialization block of all 90 cells on both sides, where the added field lives;
- `baseline/full/` and `post/full/`, three cells whole per side;
- `additivity.txt`, the 90-cell projected comparison including exit codes;
- `baseline/test-census.txt` and `post/test-census.txt`, reconciled name by name, and the byte comparison of
  `tests/process.rs` and `tests/viability.rs`;
- `interface.txt`, the engine's public interface enumerated at both commits and compared item for item;
- `static-checks.txt`, the searches for a name reader in the engine, a name literal in the observer, and a name in any
  ordering;
- `observer/frames.txt`, the rendered roster, map and inspector buffers with cell positions stated, and the bar-row
  comparison against the pre-change frame;
- `gates.txt`, the four gate commands and their output;
- `manual-assessment.md`, the seven assessments with role and date;
- `amendment-approvals.md`, oracle 5's check;
- `completion-summary.md`.

## Stop and escalate conditions

Stop and escalate rather than proceeding, if:

1. **Any existing test's assertions would have to change** to accommodate the name, beyond the three glyph call sites
   named above. `INT-MOK-008` states this as a target and this work order states it as a stop condition, because the
   alternative — a relaxed assertion — is invisible in a passing suite.
2. **Oracle 1 shows any difference** in the projected streams, or any exit-code difference, in any of the 90 cells.
3. **Oracle 2 shows any movement** in the shared stream's position, at any seed or density.
4. The name cannot be presented in a pane `REQ-MOK-041` requires without changing a bar width, a pane width or a layout
   threshold.
5. The public interface would have to grow to reach the observer.
6. A `SPEC-MOK-002` amendment turns out to be required after all: the design was chosen to avoid one, so needing one
   means the design is wrong rather than the specification is.
7. Any name would have to be changed, added, removed or reassigned to make an automated check pass. The table is a
   product decision and a check that fights it is reporting something else.
8. A glyph initial collides with a character `SPEC-MOK-003` rule 2 already assigns meaning to.

**Disclosed rather than fixed, and outside this work order's scope:**

- **Commit `524a675` did not carry `master`'s `WO-MOK-007`, `VER-MOK-007` and `VREC-MOK-007` forward.** Those three
  artifacts — the roster survival band chain, with a `verified` record — exist on `origin/master` and are absent from
  this tree, where the same three identifiers hold the individuality chain instead. `evidence/WO-MOK-007/` likewise
  holds the individuality packet and not `master`'s. `renumbering.md` states that `master`'s artifacts "are untouched
  and keep the number they were approved under", which is true of `master` and not true of this tree. Restoring them is
  the owner's act on the individuality branch, not this work order's, and this work order neither touches nor cites
  them.
- **`mokiterions-tui/tests/export.rs`'s `a_written_file_holds_exactly_the_rendered_records` is intermittently flaky on
  this platform**, failing at its `fs::read_to_string` with `NotFound` after `write_file` returned `Ok`. It was observed
  failing at the baseline commit and passing in the recorded 193-test baseline census, so it is pre-existing and not
  caused by this work. A test change is a stop condition here, so it is reported as residual rather than adjusted.

## Completion report format

The completion summary records, in this order:

1. what changed, file by file, and what deliberately did not — `AgentSnapshot`, `Observation`, `bar_width`,
   `SPEC-MOK-002`, the identifier;
2. the twelve names as the engine reported them, at more than one seed;
3. oracle 1's result over all 90 cells, with the projection's no-op check stated separately;
4. oracle 2's result, and the recorded stream positions it was checked against;
5. the census reconciliation: the pre-change count, the post-change count, every addition named, and the byte
   comparison of the two positional parsers;
6. the public-interface comparison, item for item;
7. the rendered-frame evidence, with cell positions, and the bar-row comparison;
8. the four gates' output;
9. the two amendment records, each row quoted, with the approval each carries;
10. the seven manual assessments, or an explicit statement of which are outstanding and who owes them;
11. residual uncertainty, including everything under *Stop and escalate conditions* above that remains open, and the
    statement that `VREC-MOK-011` is a separate commit-bound record that this work order does not write and cannot
    self-approve.
