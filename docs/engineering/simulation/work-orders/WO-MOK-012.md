+++
id = "WO-MOK-012"
type = "work_order"
title = "Record VER-MOK-005's seven manual assessments and ratify the eleven amended provisions they were blocked behind"
status = "draft"
owners = ["engineering owner"]
created = "2026-08-20"
updated = "2026-08-20"

[assurance]
commit_bound_verification = "not_required"
rationale = "This work order changes no executable behavior, no test, no managed policy and no CI definition. Its entire product is governance text: eleven approval cells that record a ratification the accountable owner performed on 2026-08-20, seven manual assessments the same owner authored on the same date, three amendments to VER-MOK-005 restating assessments whose subjects have moved, and the evidence retaining all of it. Nothing a later engineering, assurance, operational or release decision would rely on for correctness is altered, because the tree the assessments describe is byte-identical before and after: mokiterions-core/ and mokiterions-tui/ are untouched. Classifying this required would oblige a commit-bound record over a diff containing no trusted state, and would leave WO-MOK-005's assessment obligation open behind a second capture — the outcome this work order exists to end. The scope is not mixed: the one change that reaches an approved assurance artifact, the VER-MOK-005 amendment, is itself one of the recorded decisions of the 2026-08-20 review and was taken by the owner as assurance owner. THE ENGINEERING OWNER MUST CONFIRM THIS CLASSIFICATION AT APPROVAL; the implementation agent proposed it and holds no authority over it."
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
specifications = ["SPEC-MOK-003", "SPEC-MOK-002", "SPEC-MOK-004"]
architecture = [
  "ARCH-MOK-001",
  "ARCH-MOK-002",
  "ADR-MOK-001",
  "ADR-MOK-002",
  "ADR-MOK-003",
  "ADR-MOK-004",
]
verification = ["VER-MOK-005"]
+++

# Work Order: Record VER-MOK-005's seven manual assessments and ratify the eleven amended provisions

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and the retained evidence.

Commit-bound verification is classified `not_required` above, with the reasoning in the `rationale` field. This is
governance-only work and **stops at `implemented`**, as `WORKFLOW.md` requires: "Governance-only work that authorizes
verification, release, tagging, review, or publication stops at `implemented` unless a distinct later VREC selects it."
It does not become `verified` by recording assessments that `VER-MOK-005` requires, and it does not transition
`VREC-MOK-005`. **`VREC-MOK-005` is already `verified`**, bound to commit `f361370` — it was verified on 2026-08-19
*with* this debt disclosed, and it states so about itself: "at this candidate commit with all seven manual assessments
outstanding and unauthored, and eleven amendment provisions still OUTSTANDING". What this work order does is discharge
what that record disclosed. Whether the chain needs a re-captured record, and what a release record must inherit from
the disclosure that survives, are separate assurance decisions on separate acts; this work order neither takes them nor
presumes their outcome.

**No status transition in this file was performed by the implementation agent.** The agent authored the text; every
lifecycle act is the repository owner's.

### Why this work order is numbered 012

The identifier space is shared across branches and sessions, so the highest number in one worktree is not the free
number. `WO-MOK-012` was claimed only after enumerating all 24 remote heads of `origin` on 2026-08-20 — both the
filenames under `docs/engineering/simulation/` in each ref's tree and the file contents of every ref, searched for any
`WO-`, `VER-` or `VREC-MOK-01[2-9]` reference. `WO-MOK-011` is the highest that exists anywhere and no ref mentions
`012` in any file. The sweep and its output are retained in `evidence/WO-MOK-012/identifier-sweep.md`.

The same sweep found that `master` had advanced from `dec1b95` to `ff3a155` while this work was being prepared,
gaining `SPEC-MOK-005`, `REQ-MOK-031` through `REQ-MOK-041`, `INT-MOK-008` and `CAP-MOK-008`. Every figure and capture
this work order relies on was therefore re-derived against `ff3a155` before being used; see *Constraints*.

### Decision record

Fifteen decisions were taken by the repository owner on 2026-08-20, in one interactive review, and this work order is
the record of them. The division of authority is the same one `evidence/WO-MOK-010/closing-review.md` records for that
work order's closing review.

**Who decided what.** The repository owner holds all accountable roles of `ENGINEERING_HARNESS.md` — product,
technical, assurance and release owner — and acted in each of them here, with the role named for every decision. The
implementation agent put each question with the measured facts already assembled, and transcribed the answers. It
decided none of them, approved none of them, and transitioned no artifact.

**Each question was put and answered on its own.** No decision below was inferred from another and no answer was taken
as covering a second act. The eleven ratifications were put as four groups of separately stated provisions rather than
as one question, because a single answer covering eleven rows would be an approval by implication of ten of them. The
owner holds every role, so nothing here is approved by implication of anything else.

The decisions are recorded in full in `evidence/WO-MOK-012/closing-review.md`. They are, in summary:

| # | Decision | Role | Outcome, 2026-08-20 |
|---|---|---|---|
| 1–4 | The four `SPEC-MOK-002` provisions — rules 1, 3, 5, 6 | technical owner | **Ratified as written**, each on its own terms |
| 5 | `ARCH-MOK-001`'s narrowing of the public-item prohibition | technical owner | **Ratified as written** |
| 6 | `SPEC-MOK-003` *Data and interface contracts* clause 2 | technical owner | **Ratified as written** |
| 7–11 | The five `SPEC-MOK-004` provisions — rules 6, 9, 11, 12, 13 | technical owner | **Ratified as written** |
| 12 | Scope: does the observer fix belong in this work order? | product owner | **No — a separate chain.** This work order records and completes |
| 13 | The roster gauge layout | product owner | **Two gauges per line over three lines per entry**, bar width 13 |
| 14 | Discoverability of the `?` key | product owner | **A permanent header segment**, degrading to `?` at the floor |
| 15 | The hidden-pane notice | product owner | **State the enlargement remedy and require visual emphasis** |

Decisions 13 to 15 are recorded here but **are not implemented by this work order**. They are the substance of the
second chain that decision 12 directs, and they are written down now so that the chain that carries them inherits a
decided design rather than re-opening it.

### The seven manual assessments

`VER-MOK-005` names seven manual assessments and requires the evidence to carry them "including the legibility and
colour-independence assessments and their author". Before 2026-08-20 all seven were **OUTSTANDING** with no author, as
`evidence/WO-MOK-005/manual-assessment.md` states plainly of itself. Six are now performed and authored by the
repository owner; one remains outstanding by the owner's decision, with the reason recorded.

| # | Assessment | Role | Outcome, 2026-08-20 |
|---|---|---|---|
| 1 | The 200-tick instrument answers `INT-MOK-004`'s three questions | product owner | **Satisfied.** Position, causation and authority all answerable |
| 2 | Reference-viewport legibility | product owner | **ADVERSE OBSERVATION.** The survival bars are two columns wide |
| 3 | No distinction lost without colour, as restated | product owner | **Satisfied.** `UNDERLINED` and `REVERSED` are distinguishable |
| 4 | A rejection reads as an authority outcome, as restated | product owner | **Satisfied.** It reads as an outcome, not a fault |
| 5 | The fourth roster slot, as restated | product owner | **Satisfied.** `f ░░   0` reads as a computed zero |
| 6 | Whether overview cell granularity misleads | product owner | **Not materially misleading.** No rule 2 change required |
| 7 | The terminal is usable after a deliberate panic | assurance owner | **Remains OUTSTANDING** by decision, with a recorded reason |

Assessment 7's recorded reason: the shipped binary has no operator-reachable panic path, so the live-terminal
inspection `VER-MOK-005` asks for cannot be performed by running the observer. The automated result exists and is
retained in `WO-MOK-005`'s `terminal-restoration.txt`, and the contract's words "rather than only by an automated
assertion" make it insufficient by construction. The owner elected to leave the assessment outstanding and visible
rather than close it against a proxy, and **`VREC-MOK-005` must continue to disclose it**.

Assessment 2 produced an adverse observation, and two further adverse observations were reported in the same review
that no assessment asked for. All three are recorded in `evidence/WO-MOK-012/adverse-observations.md` and none is
fixed here. `VER-MOK-005` establishes the standing of such a report in advance — an adverse observation is "an
artifact decision, not a defect to patch" — and decision 12 disposes of all three to a separate chain.

### Required amendments to approved artifacts

**Eleven provisions across four approved artifacts are amended in the tree and every one was recorded as
OUTSTANDING.** `VREC-MOK-005` discloses this in its section "Amendments to approved artifacts that this record does
not carry", and `WO-MOK-005` states of the first five that it "is not verifiable until they are given or the scope is
changed to avoid needing them". This work order gives them.

| Artifact | Provisions | Row |
|---|---|---|
| `SPEC-MOK-002` | rules 1, 3, 5, 6 | 2026-08-18 |
| `ARCH-MOK-001` | the public-item prohibition narrowing | 2026-08-18 |
| `SPEC-MOK-003` | *Data and interface contracts* clause 2 | 2026-08-18 |
| `SPEC-MOK-004` | rules 6, 9, 11, 12, 13 | 2026-08-19 |

Each ratification is recorded by editing that row's Approval cell in place, stating the date, the role, that the
provisions were ratified as written, and that the row was **OUTSTANDING** until that act. This follows the precedent
of `SPEC-MOK-004`'s 2026-08-19 test-count row, which `WO-MOK-010`'s closing review ratified the same way rather than
by appending a row. The per-provision detail is in `evidence/WO-MOK-012/amendment-ratifications.md`.

**Five later rows asserted in the present tense that these provisions remain OUTSTANDING.** Those sentences became
false the moment the ratifications were recorded, so each is moved to the past tense and names the ratifying act,
following the precedent of `SPEC-MOK-004`'s row that already reads "which was **OUTSTANDING** when this row was
written and which the technical owner ratified on 2026-08-19". No approved row's substance is altered; only a
statement about another row's status is brought up to date.

One of those five was also **wrong when written**, independently of this work order: `SPEC-MOK-002`'s 2026-08-19 row
said "the two rows above this one, dated 2026-08-18, remain OUTSTANDING", but only the first of the two ever was — the
second, the path re-basing, was approved the same day by way of `ADR-MOK-004`. The corrected sentence records the
miscount rather than deleting it. This is a defect in a governance record's cross-reference, not in any specification's
substance, and no obligation changes either way.

**`VER-MOK-005` is amended in three provisions**, by the repository owner as assurance owner, because three of its
seven manual assessments name subjects that have moved since it was approved:

1. **Assessment 3** asked to confirm the frame "with colour disabled or on a monochrome terminal". There is no
   colour-disable mechanism: `mokiterions-tui` has no `--no-color` flag and honours no `NO_COLOR` environment
   variable, so the assessment as written cannot be performed on the shipped binary at all. It is restated as the
   check that survives — whether `UNDERLINED` and `REVERSED` render distinguishably — which is what
   `evidence/WO-MOK-005/manual-assessment.md` had already identified as the part a person must answer, and which the
   automated `every_distinction_survives_the_loss_of_colour` cannot reach because it reads a colourless projection.
2. **Assessment 4** asked to confirm a rejection reads as an authority outcome. `VREC-MOK-005` disclosure 5 records
   that no shipped decision source ever has a proposal rejected over 400 ticks of both policies, so the state is
   unreachable by running the observer. The assessment is restated to name the `#[cfg(test)]` hook route explicitly,
   so that a future assessor is not sent to look for a state that cannot occur.
3. **Assessment 5** asked whether "the reserved fourth roster bar position reads as empty space". The slot is no
   longer reserved: `SPEC-MOK-003` rule 4 as amended on 2026-08-19 presents a computed `fear` there. The assessment is
   restated against the slot as it now exists — whether a computed zero reads as computed rather than as missing or
   broken.

None of the three changes an obligation's strength; each points an existing obligation at a subject that exists. No
case about non-perturbation, entropy attribution, the engine boundary, the dependency set, export, authority mapping,
terminal restoration or any declared viewport changes, and no seed changes.

## Objective

Close the governance debt that `VREC-MOK-005` discloses and cannot itself carry: eleven unratified provisions of four
approved artifacts, and seven manual assessments that no person had performed. Leave `WO-MOK-005`'s chain in a state
where the only thing outstanding against it is outstanding by decision, with the decision and its reason on the
record.

## In scope

1. **The eleven ratifications**, recorded in the Approval cell of each amended row in `SPEC-MOK-002`,
   `ARCH-MOK-001`, `SPEC-MOK-003` and `SPEC-MOK-004`.
2. **The five stale cross-references** in later rows of `SPEC-MOK-002` and `SPEC-MOK-003`, moved to the past tense,
   including the recorded correction of the one that overcounted.
3. **The `VER-MOK-005` amendment** in the three provisions above, with an amendment-record row.
4. **The manual assessment record** at `evidence/WO-MOK-012/manual-assessment.md`, carrying all seven assessments
   with the owner as author for six and the recorded reason for the seventh.
5. **The three adverse observations**, recorded with the measurements that establish them and with the design the
   owner decided for each, at `evidence/WO-MOK-012/adverse-observations.md`.
6. **The three defects in `WO-MOK-005`'s recorded assessment procedure**, at
   `evidence/WO-MOK-012/procedure-defects.md`.
7. **The assessment material** — the ten capture files and the two oracle sources that produced them — retained
   under `evidence/WO-MOK-012/assessment-material/`.
8. **The closing review**, the identifier sweep, the amendment ratification detail, a README and a completion
   summary, under `evidence/WO-MOK-012/`.
9. **The roadmap entry** for this work order.

## Out of scope

- **Every fix for the three adverse observations.** Decision 12 places them in a separate chain. Nothing in
  `mokiterions-tui/src/` changes here.
- **`VREC-MOK-005`'s lifecycle.** It is already `verified`, and this work order does not revisit that act. Whether the
  chain needs a record re-captured now that six assessments are authored is a separate assurance decision.
- **`VREC-MOK-005`'s content.** It is bound to commit `f361370` and is not re-opened. Its disclosures were correct at
  that commit and its disclosure of assessment 7 is still correct today; this work order does not edit it, and a
  re-capture rather than an edit is the route if one is wanted.
- **`WO-MOK-005`'s own file and its retained evidence.** Its `manual-assessment.md` stated correctly, at the time,
  that all seven were outstanding and that its procedure was what an assessor should follow. The three defects in
  that procedure are recorded in this work order's evidence and the original is left as the record of what was
  believed then, following the precedent of `evidence/WO-MOK-011/merge/`, which re-measured into a new directory
  rather than editing the figures it superseded.
- **Assessment 7.** It stays outstanding by decision.
- **Any amendment to `SPEC-MOK-003` rules 4 or 5.** Decisions 13 to 15 require them and the second chain carries
  them.
- **Any push, pull request, tag or release.** None is authorized by this work order.

## Authorized decision envelope

The implementation agent may decide locally: the wording of every governance sentence it writes; the file layout under
`evidence/WO-MOK-012/`; which of the retained captures each evidence file cites; and the order of the commits.

The implementation agent may **not** decide: whether any provision is ratified; the outcome of any assessment; the
classification in `[assurance]`; the scope split of decision 12; the design of decisions 13 to 15; any lifecycle
transition of this or any other artifact; or whether `VREC-MOK-005` may be verified. Each of those is the repository
owner's and each is either recorded above or explicitly left open.

Where the agent found a fact the owner had not been shown — the `SPEC-MOK-002` cross-reference miscount, the three
procedure defects, and the absence of any colour-disable mechanism — it reported the fact and did not treat any
existing approval as covering it.

## Constraints

- **No file under `mokiterions-core/` or `mokiterions-tui/` changes.** The diff is documentation only. This is
  checkable and is the ground of the `not_required` classification.
- **Every figure and capture was re-derived against `ff3a155`.** `master` moved during preparation. The four amended
  artifacts and both packages were diffed `dec1b95..ff3a155` and are byte-identical across that range, so the
  captures taken before the move still describe the tree and the line numbers still resolve. The diffs are retained.
  Had either diff been non-empty the captures would have been retaken; a figure that describes a tree which no longer
  exists is not evidence.
- **The two oracle programs are retained as sources and are not in the tree.** Both were placed, run once and
  removed, following the precedent of `WO-MOK-010`'s `observer/frame-probe.rs` and `WO-MOK-006`'s
  `frame-and-export-oracle.rs`. An oracle asserts nothing; it writes down what the buffer holds so that the reading
  is a separate act from the capture. `mokiterions-tui/src/lib.rs` is byte-identical to `ff3a155`.
- **The workspace still passes.** 212 tests, `cargo fmt --all -- --check` clean, and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean, unchanged by this work order because
  it changes no code.
- **No artifact status is transitioned by the agent**, here or anywhere.

## Expected change surface

| Path | Change |
|---|---|
| `specifications/SPEC-MOK-002.md` | one Approval cell ratified; two cross-references corrected |
| `specifications/SPEC-MOK-003.md` | one Approval cell ratified; three cross-references corrected |
| `specifications/SPEC-MOK-004.md` | one Approval cell ratified |
| `architecture/ARCH-MOK-001.md` | one Approval cell ratified |
| `verification/VER-MOK-005.md` | three manual assessments restated; one amendment-record row |
| `work-orders/WO-MOK-012.md` | this file |
| `evidence/WO-MOK-012/` | README, closing review, manual assessment, amendment ratifications, adverse observations, procedure defects, identifier sweep, ten captures with two oracle sources, completion summary |
| `docs/ROADMAP.md` | a **Status** paragraph for Phase 1.5, which had none; plus three sentences elsewhere that this work order's own acts made false — Phase 1.5's "every artifact is `draft`", Phase 2's account of what the overridden gate still owes, and Phase 2.5's statement that the earlier `OUTSTANDING` rows are still outstanding |

No source file, test, manifest, lockfile, CI definition or managed harness file changes.

## Required verification

`VER-MOK-005` is the declared contract and this work order adds no case to it. What must hold at completion:

- the diff touches no file under `mokiterions-core/` or `mokiterions-tui/`;
- no `OUTSTANDING` claim about any of the eleven provisions survives in the present tense anywhere under `docs/`,
  other than the disclosure inside `VREC-MOK-005`, which is bound to its commit and correct there;
- all seven assessments carry a status and an author, or a recorded reason for having neither;
- `scripts/validate_engineering_artifacts.py` passes on the graph;
- the workspace's 212 tests, formatter and linter still pass.

## Evidence to record

Retain under `docs/engineering/simulation/evidence/WO-MOK-012/`:

- `README.md` — the index of this directory and what each file establishes;
- `closing-review.md` — the fifteen decisions, each with the role, the question put and the answer given;
- `manual-assessment.md` — the seven assessments, with author and outcome;
- `amendment-ratifications.md` — the eleven provisions, each with the text ratified and what it changes;
- `adverse-observations.md` — the three observations, their measurements, and the decided design for each;
- `procedure-defects.md` — the three defects in `WO-MOK-005`'s recorded procedure;
- `identifier-sweep.md` — the enumeration that established `012` was free, and the `master`-moved re-derivation;
- `assessment-material/` — the ten captures and the two oracle sources;
- `completion-summary.md` — what was done, what was decided, and what remains open.

## Stop and escalate conditions

Stop and escalate to the repository owner if:

- any of the eleven provisions turns out to require a substantive change rather than ratification as written;
- an assessment outcome recorded above cannot be reconciled with a capture, which would mean the owner assessed
  something other than what the tree does;
- `master` moves again such that a re-derived figure changes, in which case the figure is re-derived rather than
  carried;
- closing this work order would require transitioning `VREC-MOK-005`, editing a commit-bound record, or touching
  `mokiterions-tui/src/`;
- the `[assurance]` classification is not confirmed at approval.

## Completion report format

State: the eleven ratifications and where each is recorded; the six assessments authored and the one left
outstanding with its reason; the three adverse observations and the chain they are directed to; the three procedure
defects; that the diff contains no source change; and what remains open against `WO-MOK-005`'s chain after this work
order completes.
