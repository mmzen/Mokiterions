+++
id = "VREC-MOK-007"
type = "verification_record"
title = "Verification candidate for WO-MOK-007"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"
commit = "dfab77b72d2d4db1700fc1ddb4ad7ab96be998e2"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-19T12:23:14Z"
artifact_snapshot_sha256 = "dd9a1e12cb65292b1b4c942c27adb5d5777ac57bf3d09f58fa95b9b427c008b5"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-007/README.md", "docs/engineering/simulation/evidence/WO-MOK-007/band-sweeps.txt", "docs/engineering/simulation/evidence/WO-MOK-007/changed-assertions.md", "docs/engineering/simulation/evidence/WO-MOK-007/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-007/core-and-manifests-untouched.txt", "docs/engineering/simulation/evidence/WO-MOK-007/frames.txt", "docs/engineering/simulation/evidence/WO-MOK-007/harness-gates.txt", "docs/engineering/simulation/evidence/WO-MOK-007/harness-inspect-and-dashboard.txt", "docs/engineering/simulation/evidence/WO-MOK-007/manual-assessment.md", "docs/engineering/simulation/evidence/WO-MOK-007/no-retained-tick.txt", "docs/engineering/simulation/evidence/WO-MOK-007/outstanding-amendment.md", "docs/engineering/simulation/evidence/WO-MOK-007/requirement-to-test-mapping.md", "docs/engineering/simulation/evidence/WO-MOK-007/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-007/test-run-before.txt", "docs/engineering/simulation/evidence/WO-MOK-007/test-run.txt"]

[relations]
verifies_work_order = ["WO-MOK-007"]
conforms_to = ["VER-MOK-007"]
+++

# Verified Verification Record

Following review of the retained evidence by the accountable assurance owner, this record was transitioned from
`ready` to `verified` on 2026-08-19. It binds `WO-MOK-007` to candidate commit
`dfab77b72d2d4db1700fc1ddb4ad7ab96be998e2` **without changing its captured provenance**. Every provenance field in the
frontmatter — `commit`, `git_object_format`, `worktree_state`, `verified_at`, `artifact_snapshot_sha256` and the
fifteen `evidence_paths` — is the value `harnessctl capture-verification` derived at capture time, and none was
recomputed, refreshed or edited for this transition. `status` is the only field that changed. In particular
`verified_at` names the moment of **capture**, `2026-08-19T12:23:14Z`, and not the moment of this decision, which is
what the field means as the command writes it; and `artifact_snapshot_sha256` still names the 70-artifact graph as
described below.

The verification transition is a later governance decision. It does not alter the candidate commit, merge, release,
tag, publish, or deploy anything.

The record was prepared after the candidate commit it names, avoiding self-referential commit metadata.

## How this record was produced

`harnessctl capture-verification` wrote the frontmatter and, above the heading it generated, two paragraphs. The
transition replaced that heading and those paragraphs with the verified form; they are preserved here verbatim so that
nothing the command asserted is lost by the edit:

> # Verification Record Candidate
>
> This ready record binds retained evidence for `WO-MOK-007` to candidate commit
> `dfab77b72d2d4db1700fc1ddb4ad7ab96be998e2`. An accountable assurance owner must review the evidence and transition
> the record to `verified`; this command did not approve, commit, tag, release, or publish anything.
>
> The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

Both sentences were accurate when written and remain accurate as statements about the capture: the command approved
nothing, and the owner has since done what the first paragraph says an accountable assurance owner must do. Unlike
`VREC-MOK-005`, which had to be hand-authored because the command refuses an ID that already exists in the catalog,
this is a first capture under a new ID and the command performed it:

```
harnessctl capture-verification --id VREC-MOK-007 --work-order WO-MOK-007 \
  --verification VER-MOK-007 --owner "assurance owner" --domain simulation \
  --evidence <each of the 15 files in docs/engineering/simulation/evidence/WO-MOK-007/> .
```

It ran on a clean worktree at the commit it names, which is the condition `require_clean_worktree` enforces. Every
provenance field above is the command's own derivation and none has been recomputed or edited. The command's closing
sentence is accurate as written: it approved nothing and committed nothing. The commit that carries this file into the
repository is a separate act, taken on the repository owner's explicit authorization, and it is not a verification
decision either.

**The `artifact_snapshot_sha256` names the graph before this record existed.** That is by construction rather than by
oversight — the command generates the dashboard and takes its digest before writing the record it is about — and it is
measurable: at 70 artifacts the digest is `dd9a1e12…`, the value in the frontmatter, and with this file present the
same generator reports 71 artifacts, 217 relations and `14f214e3…`. So the digest binds the graph this record makes a
statement about, not the graph containing the statement.

One further measurement, because it bounds what the digest proves: writing the prose of this record **did not move
it**. The generator reports `14f214e3…` both for the record as captured and for the record with every section below
written, so the snapshot is a function of the graph — the artifact set, their frontmatter and their relations — and
not of the prose inside a typed artifact. A reader should not take a matching digest as evidence that any sentence
here is unchanged; the commit is what binds the prose. `VREC-MOK-005` reported the same digest moving when its body
was written and attributed the move to the writing, which is worth reading as the record's *existence* changing the
artifact count rather than its prose changing the digest.

## Why this record binds the merge commit

`dfab77b` is the merge of pull request #18 into `master`. `75e3598` is the commit that carries the implementation.
This record names the merge, for three reasons that are checkable rather than stylistic:

- The trees are byte-identical. `git rev-parse 75e3598^{tree}` and `git rev-parse dfab77b^{tree}` both give
  `6310f611e345186c71fbd4f2646d90690e425473`, and `git diff --stat 75e3598 dfab77b` is empty. The record therefore
  binds the same bytes either way.
- `git merge-base --is-ancestor 75e3598 dfab77b` confirms the implementation commit is an ancestor of the merge, so
  this is one lineage and not a competing account.
- **Every gate below was executed at `dfab77b`, after the merge, not on the feature branch.** Binding the commit the
  measurements were actually taken at is what makes them provenance rather than recollection.

## What this record claims

**At candidate commit `dfab77b`, `WO-MOK-007` was `approved` and `VER-MOK-007` was `approved`.** The work order
moved to `implemented` after the capture and before this transition, on the same instruction that authorized it;
that is recorded in its own `## Lifecycle` section and in the section below, and it changes no measurement here. Every
figure in this record is a measurement at `dfab77b` and is left as measured.

At that commit, **every automated case, property, invariant, static check, security check and performance check in
`VER-MOK-007` was executed and passed.**
Each of the nine rows of its requirement-to-evidence matrix is mapped to a named test or a retained file in
`requirement-to-test-mapping.md`, which also states where coverage is partial rather than leaving it to be inferred.

| Gate | Result |
|---|---|
| `cargo test --workspace` | **179 passed, 0 failed, 0 ignored, 0 filtered** across 19 targets |
| the same on pre-merge `master` at `54c21ab` | 172 passed, 0 failed — so this work adds 7 and loses none |
| `cargo fmt --all -- --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, no finding |
| `git diff --stat 54c21ab dfab77b -- mokiterions-core/` | empty — the engine package is byte-identical |
| `git diff --stat 54c21ab dfab77b -- '*Cargo.toml' Cargo.lock` | empty — no new dependency, no version or feature change |
| `git diff --stat 54c21ab dfab77b` | 20 files: 2 source, 3 artifacts, 15 evidence |
| `harnessctl validate` | PASS — 0 errors, 0 warnings, all four planes E0/W0, at 70 artifacts and again at 71 with this record present |
| `harnessctl doctor` | PASS on every managed and seed file |
| `harnessctl dashboard` | PASS — 0 errors, 9 warnings, all pre-existing `W-HEX-001` and `W-HEX-003` observations |
| `harnessctl preflight --phase review --work-order WO-MOK-007` | PASS |
| CI on pull request #18 | `governor` and `candidate` both SUCCESS against `75e3598` |

`54c21ab` is an ancestor of `dfab77b`, so the additivity comparison is a before-and-after of one lineage.

One row of that table has since moved and is deliberately left as measured. The dashboard now reports **10** warnings
rather than 9, the tenth being the `W-HEX-001` that fires on every `implemented` work order with no evidence artifact
keyed to its ID, which `WO-MOK-007` acquired when its status moved. Nine of the ten remain the pre-existing
observations named in the row. `validate` stays at 0 errors and 0 warnings, `doctor` stays PASS and the review
preflight stays PASS on the tree carrying the transition.

Five obligations `VER-MOK-007` singles out, each measured rather than argued:

- **Text identity — the obligation the whole clause turns on.** `gauge` now returns a styled `Span` and `entry_lines`
  styled `Line`s, so "no rendered character moves" is a claim that needed proving. It is asserted over **2121 cases**
  — 21 bar widths, which `bar_width`'s own unmodified test establishes as the entire range the layout admits, by 101
  values — against the unbanded form **re-derived inside the test** rather than captured from the new implementation,
  so a regression in `gauge` cannot ratify itself. `SPEC-MOK-003` rule 4's mockup line is still asserted byte for
  byte.
- **The band table and its two boundaries.** All 101 values in `0..=100` are checked against a table stated in the
  test rather than imported from the implementation's constants, so the assertions do not agree with the code by
  construction. Both boundaries are asserted on both sides by literal value — `band(39)`, `band(40)`, `band(79)`,
  `band(80)` — plus `band(0)` and `band(100)`, six literal assertions, so an off-by-one cannot hide inside a range.
  Monotonicity holds across all 100 adjacent pairs, which is the property that would have caught the inverted trend
  encoding the owner rejected.
- **No retained tick.** `band` takes one `u8` and returns a `Color`, with **one call site** in shipped code, inside
  `gauge`. Asserted structurally by interleaved reads rather than by reading the diff, and corroborated by grep in
  `no-retained-tick.txt`, which also discloses the pre-existing `latest_survival` in `state.rs` and shows it
  unreachable from the roster path.
- **Selection composes with the band.** Measured in the terminal's own cells, not in the abstraction above them: the
  same entry before and after selection, `REVERSED` absent then present, gauge foregrounds identical, text identical.
  The mechanism is that `Cell::set_style` patches the fields a style sets rather than replacing them, so a line-level
  modifier and a span-level foreground both survive.
- **Style locality.** Within an entry, only a gauge's label, bar cells and value carry that gauge's band. The
  five-column indent, both separators and the reserved fourth-bar slot carry none, and in the frame buffer every cell
  outside the three gauges on all twelve bar rows reads `Color::Reset`.

The public-tier cases **name no band colour at all.** They assert that two gauges agree in colour exactly when they
agree in band, and the band table is restated locally. That is what `VER-MOK-007`'s independence section asks for, and
it means the palette can be corrected without touching them.

## What this record does not claim

**No manual assessment in `VER-MOK-007` was executed. All three are outstanding and none has an author.** This
includes the one the contract itself calls "the one assessment that matters for the feature's purpose": that the three
bands are distinguishable on the owner's terminal. The implementation environment has no terminal and no display, and
`manual-assessment.md` says so and says why rather than marking anything passed.

The consequence is precise, and it is the same boundary `VER-MOK-005` recorded. Every automated result bound here is
an assertion about an in-memory character buffer and a style table. **Nothing here establishes that a colour arrived
at a screen**, and the feature's entire purpose is that a colour arrives at a screen. What is verified is that the
right band is attached to the right value and that nothing else changed.

Two coverage limits narrower than that, both stated in `requirement-to-test-mapping.md` rather than left to be found:

- **Acceptance scenario 2's own numbers appear in no assertion.** The scenario names 44, 8 and 91; the obligation —
  three values in three bands, three differing styles, the numbers unchanged — is asserted at 12, 55 and 88 in the
  same shape, and the domain sweep separately asserts that 44, 8 and 91 take the middle, low and high bands. The two
  together carry the scenario. Its three literal numbers do not appear together in one case.
- **Acceptance scenario 3 is carried partly.** Twelve entries at the reference viewport are asserted present with
  colour agreeing with band throughout. The pane's text identity against a capture taken before clause 7 existed is
  **not** asserted as a captured pane; it is carried by the 2121-case property at the entry level and by nine
  unmodified viewport cases. A pre-change snapshot would have been the more direct evidence and was deliberately not
  used, because a snapshot is only as good as the moment it was taken and the property holds at every width.

**This record is testimony about the agent's own work.** The same implementation agent wrote the source, wrote both
tiers of tests, assembled the fifteen evidence files, and prepared this record. That is structural rather than a
defect in any one artifact, and it is the reason the `verified` transition is reserved: the accountable assurance
owner's review is the only independent step in the chain. Three specific mitigations are on the record above — the
band table is restated in the tests, text identity is checked against a re-derived form rather than a snapshot, and
the public tier names no colour — and they reduce the risk of a test agreeing with the code by construction. They do
not substitute for review.

## Amendments to approved artifacts that this record does not carry

**One provision is outstanding and this record does not supply it.** `SPEC-MOK-004` rules 9, 10 and 11 record
per-file test counts, an internal-tier private-item count for `mokiterions-tui/src/render.rs`, and executed totals.
Seven added tests and six added private items make eight of those figures wrong: `tests/render.rs` 8 → 10, the public
tier 80 → 82, `src/render.rs` 12 → 17, the internal tier 32 → 37, the module's private items 39 → 45, its functions
27 → 28, its constants 12 → 17, and the observer's and workspace's executed totals 112 → 119 and 172 → 179.

The full replacement text, the measurement table behind it and a ready amendment-record row are drafted in
`outstanding-amendment.md` and marked **OUTSTANDING**. They were not applied, because `WO-MOK-007`'s expected change
surface ends "no other file. In particular no file under `mokiterions-core/`, no manifest, and no other artifact".
The owner approved two amendment provisions on 2026-08-19, both to `SPEC-MOK-003`, and was not shown this one: it was
found afterwards by measuring both tiers, and it is reported rather than treated as covered by that approval.

Rules 9 and 11 carry their own correction clauses — rule 11 says verbatim that "a work order that adds a test corrects
these figures here" — so the arithmetic on the totals needs no owner act. Rule 10's private-item parenthetical is a
rationale figure with no correction clause, and that is the part requiring the technical owner.

**Rule 6 is unaffected.** Every added item is private: one function and five constants. The recorded public interface
extent does not move, nothing widens, and rule 7 holds by construction. That is the rule whose amendment would have
required the technical owner for a reason of substance rather than of arithmetic.

`harnessctl validate` reports 0 errors and 0 warnings with those figures stale, because they are prose rather than a
machine-checked constraint. A green validate is therefore not evidence that they are right, and this record does not
offer it as such.

## Lifecycle status at capture, and what moved before this transition

**At capture `WO-MOK-007` was `approved`**, which is to say the harness believed work the owner had already merged had
not started. That was the most unusual of the shapes in the repository: `VREC-MOK-001` through `VREC-MOK-004` each
verify a work order transitioned to `implemented` first, `VREC-MOK-005` and `VREC-MOK-006` verify work orders left
`in_progress`, and this one named a work order at `approved`. `harnessctl validate` reported no error or warning
against that, so the graph was coherent as it stood, but `harnessctl inspect` also listed `WO-MOK-007` as the single
active work and recommended *"Run start preflight and begin only the approved scope"* for work that had merged.

**`WO-MOK-007` is now `implemented`.** The owner's instruction of 2026-08-19 — *"you can set the work order as
implemented, and I validated the verification record, that can be transitioned"* — authorized that transition and this
one in one act. Both were recorded by the implementation agent and decided by neither it nor this record. The work
order's own `## Lifecycle` section carries the transition, the evidence recap behind it, the gates on the merged tree
and the two derived figures that moved.

**The order was deliberate.** The work order's transition was written first, so that this record describes an
`implemented` work order rather than going stale the moment it was verified. That was the correction available here and
not there: `VREC-MOK-005` opens "`WO-MOK-005` is `in_progress`" and `VREC-MOK-006` opens "`WO-MOK-006` is
`in_progress`", and both work orders are now `implemented`, so four sentences across those two records are stale. They
were correctly left alone, because a bound `verified` record is not re-edited to track state that moved after it was
bound. The window in which such a sentence can be fixed closes at the transition, and this record was fixed inside it.

Two figures moved with the status, both mechanical and neither a new defect. `harnessctl preflight --phase start
--work-order WO-MOK-007` now FAILs with `[W005] status 'implemented' is not eligible for start; expected one of
approved, in_progress`, which is preflight refusing to begin work that is done. And the dashboard's warning count went
from 9 to 10 as recorded above. In exchange, `harnessctl inspect` now reports **Active work (0)**, so the stale
recommendation quoted above is gone, and with this transition it reports **Decision required (0)** and **Assurance
pending (0)** as well: the entry it raised against this record — *review-assurance-decision (assurance-owner): Review
retained evidence and record or withhold the accountable verification decision* — is the one the owner has now taken,
and no other is open.

An empty queue is not a claim that nothing is outstanding. The three manual assessments and the `SPEC-MOK-004`
amendment are outstanding and the harness does not track either, which is why they are written down here.

One incidental measurement, because it sharpens the bound recorded above on what the snapshot digest proves. This
transition moved the dashboard digest — `99ce921d…` with the work order `implemented` and this record `ready`,
`3adb7852…` with this record `verified` — while writing every paragraph of this record moved it not at all. A `status`
field is frontmatter and therefore part of the graph the digest covers; a paragraph is not. The frontmatter value
`dd9a1e12…` is none of these three: it is the graph before this record existed, exactly as captured.

**No other lifecycle status moved and nothing else in this record changed.** `SPEC-MOK-003`, `SPEC-MOK-004`,
`REQ-MOK-020` and `VER-MOK-007` are untouched, the outstanding `SPEC-MOK-004` amendment is still outstanding, and the
three manual assessments are still outstanding and unauthored.

## Authority

**This record is `verified`. The decision was the repository owner's, taken as accountable assurance owner.**

The candidate was prepared by `harnessctl capture-verification` on the owner's explicit instruction, *"retry the
command, creating a verification record as ready is authorized"*, given after the agent stopped and disclosed that a
candidate record is the agent's own testimony about its own implementation. The owner authorized the candidate and
withheld the transition at that point. The transition was then taken on the instruction *"you can set the work order as
implemented, and I validated the verification record, that can be transitioned"*, given on 2026-08-19 after the
candidate had been pushed and read. Both instructions are quoted as given; the agent recorded this transition and did
not make it, and this paragraph is the whole of the authority behind the `verified` status.

`DECISION_RIGHTS.md` places the accountable verification decision outside what an AI agent inherits, and
`ENGINEERING_HARNESS.md` states that harness commands may prepare records but never exercise accountable decision
rights. Neither is waived here: the command prepared a `ready` record and a person moved it.

**What the decision was taken with, unchanged by the taking of it:** three manual assessments outstanding and
unauthored — including the one `VER-MOK-007` itself calls "the one assessment that matters for the feature's purpose",
that the three bands are distinguishable on the owner's terminal; one amendment provision to `SPEC-MOK-004`
outstanding, drafted and not applied; acceptance scenarios 2 and 3 carried indirectly in the ways stated above; and the
whole packet, including this record, written by the same agent that wrote the implementation. Every one of those is
disclosed in the sections above and was disclosed in the body of the pull request that carried the candidate. A
`verified` status does not retire any of them, and this record does not read the owner's instruction as an assessment
that any of them has been discharged.

Verification is not release and it is not merge. Nothing here releases, tags, publishes or deploys, and no release
record exists. Pull request #18, which carries the implementation, was merged by the repository owner on 2026-08-19
before this record was prepared; the pull request carrying this record and the work order's transition is left for the
owner to merge.
