+++
id = "VREC-MOK-007"
type = "verification_record"
title = "Verification candidate for WO-MOK-007"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"
commit = "4f32a9f0accb141fb9d21795faf9554bdd3afbd6"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-19T11:25:08Z"
artifact_snapshot_sha256 = "545b9b9937ec7024ffff5ca2a139a1f9e965e44954f4a8d3037148c701421f28"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-007/README.md", "docs/engineering/simulation/evidence/WO-MOK-007/amendment-approvals.md", "docs/engineering/simulation/evidence/WO-MOK-007/analysis/amendments.py", "docs/engineering/simulation/evidence/WO-MOK-007/analysis/analyze.py", "docs/engineering/simulation/evidence/WO-MOK-007/analysis/capture-static.sh", "docs/engineering/simulation/evidence/WO-MOK-007/analysis/equivalence.py", "docs/engineering/simulation/evidence/WO-MOK-007/analysis/frames.py", "docs/engineering/simulation/evidence/WO-MOK-007/analysis/interface-census.py", "docs/engineering/simulation/evidence/WO-MOK-007/analysis/long-horizon.sh", "docs/engineering/simulation/evidence/WO-MOK-007/analysis/static-checks.py", "docs/engineering/simulation/evidence/WO-MOK-007/analysis/test-census.py", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/COMMIT.txt", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/capture.sh", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/compare.py", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-baseline-seed0-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-baseline-seed1-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-baseline-seed123-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-baseline-seed42-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-baseline-seed777-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-reference-seed0-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-reference-seed1-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-reference-seed123-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-reference-seed42-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-reference-seed42-ticks20-trace.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/full/pre-reference-seed777-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/pre-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/projection.py", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/rebuild-check.py", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/rebuild-check.txt", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/recapture-check.py", "docs/engineering/simulation/evidence/WO-MOK-007/baseline/recapture-check.txt", "docs/engineering/simulation/evidence/WO-MOK-007/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-007/escalation.md", "docs/engineering/simulation/evidence/WO-MOK-007/interface-and-purity.txt", "docs/engineering/simulation/evidence/WO-MOK-007/manual-assessment.md", "docs/engineering/simulation/evidence/WO-MOK-007/measurements/divergence.txt", "docs/engineering/simulation/evidence/WO-MOK-007/measurements/equivalence.txt", "docs/engineering/simulation/evidence/WO-MOK-007/measurements/fear.txt", "docs/engineering/simulation/evidence/WO-MOK-007/measurements/long-horizon.txt", "docs/engineering/simulation/evidence/WO-MOK-007/measurements/oscillation.txt", "docs/engineering/simulation/evidence/WO-MOK-007/measurements/proposals.txt", "docs/engineering/simulation/evidence/WO-MOK-007/measurements/traits.txt", "docs/engineering/simulation/evidence/WO-MOK-007/measurements/viability.txt", "docs/engineering/simulation/evidence/WO-MOK-007/negative-control/controls.py", "docs/engineering/simulation/evidence/WO-MOK-007/negative-control/oracle-2.txt", "docs/engineering/simulation/evidence/WO-MOK-007/negative-control/oracle-3.txt", "docs/engineering/simulation/evidence/WO-MOK-007/observer/frame-probe.rs", "docs/engineering/simulation/evidence/WO-MOK-007/observer/roster-frames.txt", "docs/engineering/simulation/evidence/WO-MOK-007/post/additivity.txt", "docs/engineering/simulation/evidence/WO-MOK-007/post/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-007/post/full/post-individual-seed42-ticks20-trace.log", "docs/engineering/simulation/evidence/WO-MOK-007/post/full/post-reference-seed42-ticks20-trace.log", "docs/engineering/simulation/evidence/WO-MOK-007/post/post-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-007/requirement-to-test-mapping.md", "docs/engineering/simulation/evidence/WO-MOK-007/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-007/test-census.txt"]

[relations]
verifies_work_order = ["WO-MOK-007"]
conforms_to = ["VER-MOK-007"]
+++

# Verification Record Candidate

This ready record binds the retained evidence for `WO-MOK-007` to candidate commit
`4f32a9f0accb141fb9d21795faf9554bdd3afbd6`. An accountable assurance owner must review the evidence
and decide whether to transition it to `verified`. Preparing it approved, verified, merged, tagged,
released and published nothing.

The record is intentionally created after the candidate commit it names, avoiding self-referential
commit metadata. **`verified_at` above is the capture timestamp, not a verification decision** — the
capture was taken at `2026-08-19T11:25:08Z` against a clean worktree at this commit, and the
`artifact_snapshot_sha256` is the harness snapshot of the **76**-artifact, 240-relation graph as it
stood before this file was written, so it records the graph this record binds rather than the graph
containing it. The status is `ready` and no verification decision has been taken by anyone.

## This record is written over its own evidence packet's objection, and that is stated first

`manual-assessment.md`, `requirement-to-test-mapping.md`, `README.md` and `completion-summary.md` §16
all say the same thing: **"no verification record can be written against this commit."** That
sentence is in the retained evidence, it was written before this file, and it is not being quietly
walked past.

Two readings of it exist and the difference is not academic.

- **The reading applied here.** What the packet means, and what `VER-MOK-007` actually forbids, is a
  *`verified`* record: the contract says it "is not satisfied while any [assessment] remains
  outstanding", which is a statement about satisfaction, and satisfaction is what a verification
  decision rests on. A `ready` candidate takes no decision. It binds the commit, fixes the evidence
  set, and puts the unmet obligations in front of the roles that own them in a form the harness can
  see — which is what `VREC-MOK-005` already is, and has been since 2026-08-18 with all seven of its
  own manual assessments outstanding. On this reading a `ready` record is the repository's existing
  form for precisely this state. That last claim is checkable rather than rhetorical: with this file in
  the tree, `scripts/inspect_engineering_artifacts.py` reports `decision_required ->
  review-assurance-decision (assurance-owner)` against `VREC-MOK-005` **and** `VREC-MOK-007`, where
  before it reported it against `VREC-MOK-005` alone. Adding the record raised the signal and changed
  no error, warning or informational count.
- **The narrower reading.** Nothing should be written until the assessments are recorded, in which
  case this file is premature. It binds no decision and confers nothing, so retracting it costs one
  commit and no recorded value; nothing in the evidence changes either way.

**The reading was chosen by the implementation agent and it should not have been.** It was prepared on
the repository owner's instruction to prepare the verification record, and the owner holds every
accountable role here, so the instruction is the strongest available signal that a candidate was
wanted. It is still an agent's construction of what the packet's own sentence permits, and it is named
at the top rather than left to be found.

## What this record claims

`WO-MOK-007` is `in_progress` and `VER-MOK-007` is `approved`. At candidate commit
`4f32a9f0accb141fb9d21795faf9554bdd3afbd6`, **every automated case, oracle, static check and
comparison in `VER-MOK-007` was executed and passed. Three of the contract's own obligations are not
satisfied — its manual-assessment clause and two rows of its requirement-to-evidence matrix — so this
record cannot claim that the contract is met.** That is the difference between this candidate and
`VREC-MOK-001` through `VREC-MOK-004` and `VREC-MOK-006`, and it is stated here because it changes
what an assurance owner is being asked to accept.

| Gate | Result |
|---|---|
| `cargo test --workspace` | **exit 0. 20 runners, 20 ok, 190 passed, 0 failed, 0 ignored, 0 filtered out** |
| `cargo fmt --all -- --check` | exit 0, 0 diff lines |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, 0 warning or error lines, **2 crates re-linted in the run** so the result is this tree's and not a cache's. No `allow` added, no lint suppressed |
| `cargo tree -p Mokiterions` | **one line** — the package alone; dependency and dev-dependency tables empty |
| `cargo tree -p mokiterions-tui` | 111 lines, identical to the pre-change commit's line for line with the checkout path normalised |
| `python scripts/validate_engineering_artifacts.py` | PASS — 76 artifacts, 0 errors, 0 warnings, across all four planes |
| `bash scripts/check_engineering_harness.sh` | PASS — 76 artifacts, 240 relations, 0 errors |
| `python scripts/inspect_engineering_artifacts.py` | 0 errors, **10 warnings**, 6 informational — **three warnings are new and caused by this change**, see the disclosures below |

Zero ignored and zero filtered out is the part worth stating: a suite can be made to pass by not
running, and those two counts are what would show it. `static-checks.txt` retains the transcripts and
`analysis/capture-static.sh` reproduces them.

The three harness rows are the state **at the candidate commit**, where this record does not yet exist.
With it in the tree the validator reports 77 artifacts and 242 relations, still 0 errors and 0 warnings
across all four planes, and the inspector's counts are unchanged at 10 warnings and 6 informational.
The three new warnings are `ARCH-MOK-001` and `ARCH-MOK-002` now predating specifications they declare
conformance to, and they belong to the change rather than to this record; the reason neither artifact
was edited to silence them is below.

`VER-MOK-007`'s central claim is that the two frozen decision sources are untouched and the new one
behaves as `SPEC-MOK-001` rule 19 specifies. That claim is not carried by one measurement. Five
oracles, each able to fail without the others failing:

- **Oracle 1 — a recorded pre-change baseline under a stated projection.** Captured at predecessor
  commit `60fda9faffbd452752a34efa356f16cc6ad1d3ff`, 42 frozen-source cells: **42 of 42 projected-equal
  with zero differing bytes**, exit codes and stderr byte counts identical, and the projection a
  **no-op on the pre-change stream, 42 of 42** — the one way this oracle could have been subverted. The
  20 new-source cells are byte-identical across two OS processes, 20 of 20, and differ from the
  reference source's, 20 of 20. `baseline/`, `post/additivity.txt`.
- **Oracle 2 — the shared entropy stream's position across trait derivation.** **72 draws** for twelve
  Mokiterions, asserted as a literal at every declared seed and density, because trait derivation
  builds `SplitMix64::new(seed ^ number.wrapping_mul(TRAIT_SALT))`, uses it and drops it. The check was
  shown capable of failing: perturbing the derivation to draw from the shared stream gives **84**, and
  three tests fail — the three the defect predicts. `negative-control/oracle-2.txt`.
- **Oracle 3 — arithmetic equivalence at the trait's lower bound.** All **2,808** situations
  enumerated rather than sampled; rule 19 at `T = 0` is proposal-identical to rule 5. Two negative
  controls, and control B fails at the very first case on the *stream position* rather than on the
  proposal, which is the failure mode a proposal-only comparison would have missed.
  `measurements/equivalence.txt`, `negative-control/oracle-3.txt`.
- **Oracle 4 — the in-memory character buffer, cell by cell.** **864 bar rows across 134 viewport
  renders**, rebuilt from `SPEC-MOK-003` rule 4's named parts rather than read back from the product,
  **0 discrepancies**, with the fourth gauge at its predicted absolute columns at every roster-drawing
  viewport — label 36, bar 38–39, value 41–43. `observer/roster-frames.txt`.
- **Oracle 5 — the governance state of the amended artifacts.** Its **first** condition holds: every
  provision the owner approved on 2026-08-19 is present in both the amendment record and the
  specification text, checked over disjoint text, with the two amend-by-deletion provisions shown to
  have deleted and 7 of 7 controls on the checks themselves holding. Its **second** condition does not.
  `amendment-approvals.md`.

Four structural obligations, each measured rather than argued:

- **The public interface grew by exactly two values and lost nothing.** Public items 49 → 49, public
  fields 42 → 43 (`AgentSnapshot.fear`), enum variants 47 → 48 (`Policy::Individual`); 2 additions, 0
  removals. `SPEC-MOK-002` rule 6's prohibition was re-checked rather than amended.
  `interface-and-purity.txt`.
- **The test census reconciles name by name in both directions.** 169 tests over 19 runners → **190
  over 20**, with **21 additions named in full and 0 removals**; on both sides the listed names equal
  the sum of the runners' own declared totals. One runner is new — `decisions (tests/decisions.rs)` —
  and the census names what moved into it and why, because a new public-tier runner is exactly how a
  relocated test would look. `test-census.txt`.
- **Tier placement is by required access, not by subject.** 13 of the 21 new tests are internal-tier
  and each is listed with the private item or `#[cfg(test)]` hook it requires; 8 are public-tier across
  5 files. One test changed tier, and its assertions are verbatim across the move as `SPEC-MOK-002`
  rule 12 requires. **No item was widened to `pub` to relocate it**, which is checked rather than
  asserted — the interface census finds the same two additions either way.
- **`fear` is bounded, stepped and perception-correspondent on every measured tick.** Over **111,604
  agent-ticks** across ten 1,000-tick runs: **zero** range, step or perception-correspondence
  violations, including at both saturating bounds and at the radius boundary, where 245–427 agent-ticks
  per run at Chebyshev exactly 16 all rose and 238–448 at exactly 17 all fell. `measurements/fear.txt`.

## What this record does not claim

**`VER-MOK-007` is not satisfied at this commit.** Three of its obligations are unmet, and each is
quoted rather than paraphrased.

1. **Five of the seven manual assessments are outstanding and a sixth is unsigned.** The contract:
   "An unrecorded assessment is an outstanding assessment, and this contract is not satisfied while
   any remains outstanding." Assessment 5, roster legibility at four bars, is **recorded 2026-08-19**
   by the technical owner. Assessment 7, the absence of a `fear` consumer, is recorded *in substance*
   in `SPEC-MOK-001`'s approved *Scope* sentence but not separately signed, and approving a
   specification sentence is not the same act as signing an assessment. Assessments 1, 2 and 3 belong
   to the product owner, 4 to the technical owner and 6 to the assurance owner, and **none has an
   author**. `manual-assessment.md` puts each in front of its role with the measured facts already
   assembled; it is not the record of the judgement, and it says so in its own second paragraph.
2. **Oracle 5's amendment row is not satisfied.** Two amendments written during implementation are
   OUTSTANDING, and the contract's own words are "an amendment nobody approved is not a
   specification". Detail in the next section.
3. **The `VREC-MOK-005` gate row is not satisfied.** `WO-MOK-005`'s six amendments and seven manual
   assessments were not resolved before this implementation began; the repository owner overrode the
   gate on 2026-08-19. `amendment-approvals.md` §4 checks the mitigation rather than asserting it —
   every amendment row dated before 2026-08-19 is byte-identical to `60fda9f`, and `VREC-MOK-005` and
   `ARCH-MOK-001` are unchanged — but the row is unmet, and **it is a cost carried forward, not a debt
   paid**.

Two further limits bound what the green gates mean:

- **Every automated result bound here is a claim about text or about an in-memory character buffer.**
  No case was verified by looking at a terminal. The judgements that need a person — whether the fourth
  gauge is legible beside the other three, whether `fear` at its ceiling on 39% of agent-ticks is
  intended, whether 3 to 10 divergences per run is individuality — are the outstanding assessments
  above.
- **`fear` has no consumer, so no outcome can falsify its constants.** One writer, no reader, by
  census. What is verified is that the attribute is maintained, bounded, perception-driven and
  reported; whether `+10`/`-5` is the right pair becomes answerable when something reads it.

## Amendments to approved artifacts that this record does not carry

Three amendments were written beyond the list `WO-MOK-007` states. **One is approved and two are
OUTSTANDING because ratification is the technical owner's act and the owner has not taken it.** This
record cannot supply it, and transitioning this record to `verified` would not supply it either — an
assurance owner's verification is a statement about evidence, not about another role's approvals.

- **Approved.** `SPEC-MOK-001`'s trait range narrowed from `0..=100` to `0..=40`, with rule 19's
  upper-bound note and the two acceptance examples that cited unreachable tolerances. The repository
  owner, acting as technical owner, chose narrowing over amending `REQ-MOK-034`'s survivor floor on
  2026-08-19 when `WO-MOK-007` stop condition 6 fired; `escalation.md` retains the measurement the
  decision was taken on. The first form of *Behavioral trait* named this amendment as the one to make
  on exactly this evidence, so it is a foreseen correction rather than an unplanned one. **Every
  survivor figure in this packet is downstream of it.**
- **OUTSTANDING.** `SPEC-MOK-001`'s *Help output* correction — a correction to this work order's *own*
  first amendment, which required the explanatory prose to name the default source and so contradicted
  the same section's approved *stated once* paragraph. The inherited test
  `cli::each_declared_default_is_stated_once`, bound by a `verified` `VREC-MOK-004`, asserts the side
  the implementation is on, so satisfying the withdrawn clause would have meant relaxing an assertion
  this work order forbids. **It corrects text the technical owner approved on 2026-08-19 and needs that
  owner's ratification.**
- **OUTSTANDING.** Three further `SPEC-MOK-003` provisions outside rule 4: the `AgentSnapshot` field
  list gains `fear`, rule 10 item 7 loses `fear` and traits from its list of values the engine does not
  compute, and rule 11's `decision_source_selected` row gains `REQ-MOK-033` for `individual`. Each is
  forced by the change rather than chosen with it, and each is written into the 2026-08-19 amendment
  row rather than made quietly. **They require the technical owner's ratification.**

The six `OUTSTANDING` amendment rows that `SPEC-MOK-002`, `SPEC-MOK-003` and `ARCH-MOK-001` carry from
`WO-MOK-005` are untouched and still say OUTSTANDING. No commit-bound record was edited.

**Two architecture artifacts were deliberately not edited**, and this is the reason the harness
inspection now reports three warnings it did not report before. `ARCH-MOK-001` (2026-08-18) now
predates `SPEC-MOK-001` and `SPEC-MOK-002` (2026-08-19), and `ARCH-MOK-002` predates `SPEC-MOK-003`,
in each case a specification they declare `conforms_to`, so the inspector asks for a reassessment. The
substantive reassessment was in fact made — the technical owner confirmed on 2026-08-19 that
`ARCH-MOK-001`'s boundaries, prohibited patterns and dependency prohibition hold unchanged, recorded
in `WO-MOK-007`'s *Decision record* — but it was recorded in the work order rather than in the
architecture artifact, so the date comparison still fires. Bumping an approved artifact's `updated`
field to silence a warning would have been a governance act on an artifact this work order was
authorized to *confirm* and not to amend, and would have created a third unratified amendment. **The
warning is left standing.** Closing it is the artifact owner's act, in `ARCH-MOK-001` and
`ARCH-MOK-002`.

## What the accountable assurance owner must weigh before verifying

The three unmet obligations and the two unratified amendments above are the first items.
`completion-summary.md` §16 discloses fifteen findings and its §4 two more; none is a failure against
an automated case, and each is stated so that verification, if given, is given knowingly. The nine that
bear most on how much the green gates mean:

1. **Individuality is demonstrated at the scale it was measured, and that scale is the weakest result
   in the packet.** `REQ-MOK-033`'s real-run divergence case yields **10, 3, 3, 5 and 3** divergent
   situations per thousand-tick run — within a factor of three of the figure `VER-MOK-007` itself names
   as a failure — and **zero** cases across all five seeds of two Mokiterions facing the same situation
   on the same tick, so no divergence is ever visible side by side in one frame. The 54 to 97
   waste-accepting eats per run, by 9 or 10 distinct Mokiterions of twelve, are the same behavior
   counted without requiring a coincidence, but substituting a more favourable measure for the one the
   approved contract names is the product owner's call and is exactly what assessment 2 exists to
   prevent.
2. **`fear` sits at its ceiling on 39% of agent-ticks**, and `0` is by a wide margin the most common
   transition — `-5` on 13,940, `0` on 90,201, `+5` on 219, `+10` on 7,244. This follows from the
   approved `+10`/`-5` ratchet against an expected 0.73 other Mokiterions inside the radius, and rule
   12 states that saturation is a normal outcome rather than an error. It is recorded because a future
   consumer would find the attribute pinned at its ceiling two agent-ticks in five and would be reading
   a near-constant rather than a signal. That belongs in front of whoever specifies that consumer.
3. **Oracle 1's pre-change baseline covers eleven cells with certainty and forty on an argument.**
   Eleven of the forty declared cells — the ten 1,000-tick default-density frozen-source runs and one
   20-tick traced run — were captured before the first line of code changed and are retained whole. The
   other 31 were captured afterwards from a clean git worktree at the same commit. That is a recapture;
   oracle 1 forbids recapturing the baseline *to resolve a discrepancy*, and this was not that, but the
   distinction rests on the recapture being from the same world, so `baseline/recapture-check.txt`
   compares the eleven shared cells byte for byte. All eleven match. **A reader who does not accept the
   argument should read oracle 1 as covering eleven cells, not forty.**
4. **The post-change capture came from a binary that then fell behind the source three times** —
   `cargo fmt` reformatted the engine source, one test moved from the crate's `#[cfg(test)]` block to
   the public tier, and one `debug_assert!` was tightened to the amended range's bound. Each has a
   reason it cannot matter and none of those reasons is relied on: the tree was rebuilt from the
   committed source and the whole matrix recaptured, **all 83 shared cells byte-identical**
   (`baseline/rebuild-check.txt`).
5. **Two seeds sit one death from `REQ-MOK-034`'s floor**, at 9 of 12, and the whole survivor table is
   downstream of a trait-range narrowing decided mid-implementation on a fifty-seed sweep whose
   `0..=40` miss rate is 4%. **A sixth declared seed could miss the floor.** The floor is met on the
   five seeds the contract declares and on nothing else: survivors under `individual` are 11, 9, 9, 10
   and **12**, against 8, 11, 8, 9 and 11 under `reference`, and neither territory reached zero
   standing resources on any seed under either source.
6. **The oscillation comparison reproduces `WO-MOK-002`'s counts but not its denominators.** The
   recorded 1,097/10,339 and 174/1,427 reproduce exactly, while this measurement's own pooled
   denominators are larger by **exactly 96** in both rows — one per Mokiterion-run of the eight runs
   involved, which is what an off-by-one tick-boundary convention looks like. The rates agree to 0.1
   percentage point either way: 10.8% `individual`, 10.8% `reference`, 11.7% `baseline`, against rule
   5's recorded 10.6% and the 12.2% unbiased-walk rate, so no seed exceeds the walk rate and
   `VER-MOK-007`'s finding condition is not triggered. **The seed-0 margin is 0.008 percentage
   points** — 11.815% against 11.823% — which is not a margin to treat as established.
7. **Only one roster bar width is reachable through `render::draw`.** The roster pane is 47 columns
   wide and rule 4's collapse threshold is also 47, so the drawn roster is always two-line at a
   45-column interior and `bar = 2` is the only width any frame can produce: a sweep of every width to
   160 observed bar widths `{2}`, two-line forms only, and 66 widths drawing no roster at all. Rule 4's
   `min(20, …)` cap and its collapsed one-line form are carried by three named internal render tests
   and by no frame an operator can produce. **This makes the technical owner's recorded decision on
   assessment 5 more load-bearing than it appeared when it was taken, not less**, and it arrived after
   that decision.
8. **A `.gitattributes` was added outside this work order's *In scope* list**, and without it no
   manifest digest in this packet or in the six before it reproduces from a Windows clone. With
   `core.autocrlf = true` and no attribute, a checkout rewrote every retained stream to CRLF, so every
   recorded digest failed against a file whose content was correct — not hypothetically:
   `WO-MOK-006`'s retained `baseline/engine/full/short_seed42_baseline_trace_on.txt` hashes to
   `08eadb21…` as checked out against the `3424133a…` its own manifest records. One line —
   `docs/engineering/simulation/evidence/** -text` — disables the conversion, and the fix was checked
   in a clean worktree at this commit rather than argued from the attribute: both that file and this
   packet's own streams hash to their recorded values there. It changes no code and no recorded value,
   but it is a repository-level file this work order did not set out to add.
9. **The trait derivation is not a security primitive.** SplitMix64 is not cryptographic and the seed
   is a public command-line argument, so trait values are trivially predictable by design. Nothing in
   this change relies on their unpredictability and no future requirement should. Separately, no
   model-provider credential or other secret enters the repository under this change, in code, in a
   fixture or in retained evidence, and the retained evidence contains simulation output only.

Four of the fifteen are stated in the sections above rather than repeated here: the overridden
`VREC-MOK-005` gate, the five outstanding assessments, the two unratified amendments and the three new
inspection warnings. **The last three need a read rather than a decision.** Equivalence is exhaustive
over a finite 2,808-situation set rather than over the input space, and oracle 1 covers 40 cells of a
much larger one. Seed 777 is both the only `individual` run to go extinct by tick 10,000, at 9,938, and
the only seed retaining all twelve at tick 1,000, and no explanation for the pairing is offered. And the
evidence packet is **6.9 MB against 888 KB** for the largest previous one, almost all of it the five
1,000-tick reference-source baseline streams retained whole so that oracle 1 can be re-derived by a
reviewer rather than taken from a digest — whether that trade is worth the repository weight is the
owner's call, and the alternative is retaining digests alone and asking a reviewer to trust
`compare.py`. Separately, `completion-summary.md` §13 reports four specification mismatches found during
the work, in the form `WO-MOK-005`'s summary used.

## Behavioural observations that are not failures

- **At the 10,000-tick horizon the two sources separate decisively.** `reference` reaches extinction on
  seeds 0, 1, 123 and 777 at ticks 5,423, 8,273, 9,154 and 9,598 and leaves one survivor at seed 42;
  `individual` reaches the tick limit on seeds 0, 1, 42 and 123 with 5, 4, 2 and 1 survivors and goes
  extinct at seed 777 on tick 9,938. `REQ-MOK-034` states no obligation on this, and assessment 3 asks
  the product owner to record improvement, regression or neither.
- **The recorded controls reproduce exactly.** The reference source reaches extinction at seed 123 on
  tick **9,154**, the figure `VER-MOK-002` records, to the tick — a frozen-source outcome reproduced
  nine times further out than the 1,000-tick oracle's window. Territory A under `reference` at seed 0
  holds 61 standing resources of which 45 are high class, the recorded 45 of 61, to the resource.
- **The high-class share at tick 1,000 is mixed.** `individual` is lower on three of five seeds and
  higher on two — 58/61, 70/35, 50/60, 66/63 and 77/59 per territory, against `reference`'s 73/58,
  75/64, 62/63, 68/73 and 45/46 — so the 1,000-tick picture does not carry the direction the
  10,000-tick one does. `SIMULATION_RULES.md` records the same fact under *what this did not fix*, and
  `REQ-MOK-034` states no obligation on it.
- **A re-derived expectation would have accepted the perturbed trait table.** Only the checked-in
  literal row of sixty trait values rejected it, so those sixty recorded values are load-bearing rather
  than decorative — and `simulation.rs` was verified byte-identical by SHA-256
  (`4850384d0fec95682dadda00d87a53fbeba026474a6916a773058a46927b3671`) before and after every negative
  control, so no control's perturbation survives in the candidate.
- **`WO-MOK-007` remains `in_progress`**, matching `WO-MOK-005` and `WO-MOK-006` and differing from
  `VREC-MOK-001` through `VREC-MOK-004`, whose work orders were transitioned to `implemented` first.
  Verification is carried by this record rather than by a change to the work order, and
  `scripts/validate_engineering_artifacts.py` reports no error or warning against that. Whether `WO-MOK-007` should also move
  is a separate lifecycle decision belonging to the owner; it was not instructed and was not taken.

## What must happen before this record can be verified

Stated as a list because "resolve the outstanding items" is not a specification of work. Each names the
role that owes it.

1. **Assessments 1, 2 and 3 — product owner.** Scarcity at the default density, whether the divergence
   figures answer `INT-MOK-006`, and whether the accumulation result is an improvement, a regression or
   neither. Assessment 2 is the one most likely to be answered adversely.
2. **Assessment 4 — technical owner.** Whether `+10`/`-5` is the pair to keep given 39% ceiling
   residency.
3. **Assessment 6 — assurance owner.** Whether the projection's three anchored patterns and their
   ordering delete the additions and nothing else. Reading three regular expressions is the whole of
   it.
4. **Assessment 7 — technical owner.** The signature. The substance is not in doubt; the approved
   `SPEC-MOK-001` *Scope* sentence carries it.
5. **Two amendment ratifications — technical owner.** The `SPEC-MOK-001` *Help output* correction and
   the three extra `SPEC-MOK-003` provisions.
6. **The `VREC-MOK-005` layer — repository owner.** Six amendments and seven manual assessments, which
   the override deferred rather than discharged. This is the only item that cannot be closed inside
   `WO-MOK-007`'s scope, and oracle 5's second condition stays unmet until it is.

Items 1 to 5 are all judgements or ratifications; **none is waiting on code, a measurement or a
re-run.** All five automated oracles pass at this commit.

## Scope of the transition being requested

**No transition is requested yet.** Transitioning this record to `verified` as it stands would record
that the assurance owner accepts the evidence **with five of `VER-MOK-007`'s seven manual assessments
unperformed, a sixth unsigned, two amendments to approved artifacts unratified, and two rows of the
contract's own requirement-to-evidence matrix unsatisfied** — which is not a judgement about
disclosures but the contract's own stated condition for being unsatisfied. This record should stay
`ready` until the six items above are closed, or until the owner decides on the record and states what
is being accepted in their place.

It would not perform those assessments, would not supply those ratifications, and would not merge,
release, tag, publish or deploy anything.

The candidate commit sits on `feature/phase-2-individuality` with no upstream: **nothing has been
pushed, and no pull request exists.** Verification is not merge and not release; release remains a
separate record and a separate accountable decision.
