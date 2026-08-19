+++
id = "VREC-MOK-010"
type = "verification_record"
title = "Verification candidate for WO-MOK-010"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"
commit = "035a001169757464c7f2eda2e2dfafc06b3f8910"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-19T16:50:02Z"
artifact_snapshot_sha256 = "71dc899e899465f9d851c5804f40d8ca799ccd613072d147e0036d504cfb8954"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-010/README.md", "docs/engineering/simulation/evidence/WO-MOK-010/amendment-approvals.md", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/amendments.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/analyze.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/capture-static.sh", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/equivalence.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/frames.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/interface-census.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/long-horizon.sh", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/static-checks.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/test-census.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/COMMIT.txt", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/capture.sh", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/compare.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed0-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed1-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed123-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed42-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed777-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed0-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed1-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed123-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed42-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed42-ticks20-trace.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed777-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/pre-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/projection.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/rebuild-check.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/rebuild-check.txt", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/recapture-check.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/recapture-check.txt", "docs/engineering/simulation/evidence/WO-MOK-010/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-010/escalation.md", "docs/engineering/simulation/evidence/WO-MOK-010/interface-and-purity.txt", "docs/engineering/simulation/evidence/WO-MOK-010/manual-assessment.md", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/divergence.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/equivalence.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/fear.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/long-horizon.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/oscillation.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/proposals.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/traits.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/viability.txt", "docs/engineering/simulation/evidence/WO-MOK-010/negative-control/controls.py", "docs/engineering/simulation/evidence/WO-MOK-010/negative-control/oracle-2.txt", "docs/engineering/simulation/evidence/WO-MOK-010/negative-control/oracle-3.txt", "docs/engineering/simulation/evidence/WO-MOK-010/observer/frame-probe.rs", "docs/engineering/simulation/evidence/WO-MOK-010/observer/roster-frames.txt", "docs/engineering/simulation/evidence/WO-MOK-010/post/additivity.txt", "docs/engineering/simulation/evidence/WO-MOK-010/post/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-010/post/full/post-individual-seed42-ticks20-trace.log", "docs/engineering/simulation/evidence/WO-MOK-010/post/full/post-reference-seed42-ticks20-trace.log", "docs/engineering/simulation/evidence/WO-MOK-010/post/post-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-010/renumbering.md", "docs/engineering/simulation/evidence/WO-MOK-010/requirement-to-test-mapping.md", "docs/engineering/simulation/evidence/WO-MOK-010/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-010/test-census.txt"]

[relations]
verifies_work_order = ["WO-MOK-010"]
conforms_to = ["VER-MOK-010"]
+++

# Verification Record Candidate

This ready record binds the retained evidence for `WO-MOK-010` to candidate commit
`035a001169757464c7f2eda2e2dfafc06b3f8910`. An accountable assurance owner must review the evidence
and decide whether to transition it to `verified`. Preparing it approved, verified, merged, tagged,
released and published nothing.

**It is a re-capture.** The candidate this replaces bound `4f32a9f0accb141fb9d21795faf9554bdd3afbd6`,
a commit that predates `master`'s merge into this branch; the next section says what moved and why the
figures below are not the figures it carried. A `ready` record may be re-captured; a `verified` one may
not, and none was touched.

The record is intentionally created after the candidate commit it names, avoiding self-referential
commit metadata. **`verified_at` above is the capture timestamp, not a verification decision** — the
capture was taken at `2026-08-19T16:50:02Z` against a clean worktree at this commit, and the
`artifact_snapshot_sha256` is the harness snapshot of the **80**-artifact, 248-relation graph as it
stood at that commit, so it records the graph this record binds rather than the graph containing this
text. On a re-capture that distinction is sharper than on a first capture and is worth stating exactly:
`dashboard-data.json` embeds this record's own `commit`, `verified_at` and `artifact_snapshot_sha256`
fields, so the graph it digests is the one holding the **retired** form of this file, and the digest
could not name the graph holding this form without being an input to itself. It also embeds
`repository.revision`, which is `035a001` — the digest names both the graph and the commit. The status
is `ready` and no verification decision has been taken by anyone.

## This record is written over its own evidence packet's objection, and that is stated first

`manual-assessment.md`, `requirement-to-test-mapping.md`, `README.md` and `completion-summary.md` §16
all say the same thing: **"no verification record can be written against this commit."** That
sentence is in the retained evidence, it was written before this file, and it is not being quietly
walked past.

Two readings of it exist and the difference is not academic.

- **The reading applied here.** What the packet means, and what `VER-MOK-010` actually forbids, is a
  *`verified`* record: the contract says it "is not satisfied while any [assessment] remains
  outstanding", which is a statement about satisfaction, and satisfaction is what a verification
  decision rests on. A `ready` candidate takes no decision. It binds the commit, fixes the evidence
  set, and puts the unmet obligations in front of the roles that own them in a form the harness can
  see — which is what `VREC-MOK-005` was for a day: `ready` from 2026-08-18 with all seven of its own
  manual assessments outstanding, until `master`'s assurance owner transitioned it to `verified` on
  2026-08-19 with those seven still outstanding. On this reading a `ready` record is the repository's
  existing form for precisely this state, and the precedent stands on what that record was when this
  reading was taken rather than on what it is now. That last claim is checkable rather than rhetorical:
  with this file in the tree, `scripts/inspect_engineering_artifacts.py` reports `decision_required ->
  review-assurance-decision (assurance-owner)` against `VREC-MOK-010`, and with the file removed from a
  worktree at this commit it reports that queue **empty** — 79 artifacts, 246 relations, 12 warnings and
  7 informational against this commit's 80, 248, 12 and 8. Adding the record raises the signal by
  exactly one informational observation, which is the decision it asks for, and changes no error and no
  warning. At the first capture the queue already named `VREC-MOK-005` and the informational count did
  not move; `master`'s transition is what makes this record the only entry in it now.
- **The narrower reading.** Nothing should be written until the assessments are recorded, in which
  case this file is premature. It binds no decision and confers nothing, so retracting it costs one
  commit and no recorded value; nothing in the evidence changes either way.

**The reading was chosen by the implementation agent and it should not have been.** It was prepared on
the repository owner's instruction to prepare the verification record, and the owner holds every
accountable role here, so the instruction is the strongest available signal that a candidate was
wanted. It is still an agent's construction of what the packet's own sentence permits, and it is named
at the top rather than left to be found.

## Why this record was re-captured, and what the retired candidate said

The retired candidate bound `4f32a9f0accb141fb9d21795faf9554bdd3afbd6`, captured at
`2026-08-19T11:25:08Z` over a 76-artifact, 240-relation graph digesting to `545b9b99…`. Everything it
said was true of a tree that no longer exists. `master` advanced by ten commits while this branch sat
unmerged and was merged in at `7a2b502b908be03ad8e2de7c23ee3eaaf4ece048`, which withdrew `SPEC-MOK-003`
rule 5's tier table in favour of one threshold per pane, added `master`'s own survival bands to rule 4,
and created a different `WO-MOK-007` — so this chain was renumbered to `010` on the owner's decision
(`renumbering.md`). A record binds a commit, and this one bound a commit from before all of that.

**It was re-captured rather than edited into agreement.** The distinction is the same one the evidence
packet applies to its captures: a candidate that has been corrected into looking current is not a
capture of anything. Three retained captures were re-run against the merged tree rather than adjusted —
`observer/roster-frames.txt`, `test-census.txt` and `static-checks.txt` — and the figures below are read
from those. Sixteen retained captures still name this work order `WO-MOK-007`, because that was its name
when they were taken, and `renumbering.md` says which and why.

What moved between the two candidates, so a reader of the retired one can see the difference rather than
diff it:

| Figure | Retired candidate at `4f32a9f` | This candidate at `035a001` |
|---|---|---|
| Workspace suite | 20 runners, 190 passed | 20 runners, **200 passed** — re-run at this commit |
| Test census | 169 over 19 runners → 190 over 20, before side `60fda9f` | **179 over 19 → 200 over 20**, before side `master`'s tip |
| Oracle 4 | 864 bar rows over 134 probed frames, 4 roster-drawing viewports | **996 bar rows over the 85 of 157 probed frames that draw a roster**, 8 viewports |
| Validator / harness | 76 artifacts, 240 relations | **80 artifacts, 248 relations** |
| Inspector | 10 warnings, 6 informational, 3 warnings new to this change | **12 warnings, 8 informational, 2 new to this change** |
| Amendments beyond the approved list | 3, of which 2 OUTSTANDING | **7, of which 4 OUTSTANDING** and 2 changing no provision |
| Oracle 5 controls | 7 | **12**, and a second base for the checks the merge made ambiguous |
| `VREC-MOK-005` | `ready`, gate overridden | **`verified` by `master`'s act**, gate still not met |

Five results did not move, and that is a measurement rather than an omission: `master` did not touch the
engine by one byte — `git diff --stat 60fda9f 7a2b502 -- mokiterions-core/` is empty — so oracle 1,
oracle 2, oracle 3, the `fear` measurements and the public-interface census stand at this commit with the
before side they were taken against. No figure in this record is carried over from the retired candidate
without either being re-run here or resting on that emptiness.

## What this record claims

`WO-MOK-010` is `in_progress` and `VER-MOK-010` is `approved`. At candidate commit
`035a001169757464c7f2eda2e2dfafc06b3f8910`, **every automated case, oracle, static check and
comparison in `VER-MOK-010` was executed and passed. Three of the contract's own obligations are not
satisfied — its manual-assessment clause and two rows of its requirement-to-evidence matrix — so this
record cannot claim that the contract is met.** That is the difference between this candidate and
`VREC-MOK-001` through `VREC-MOK-004` and `VREC-MOK-006`, and it is stated here because it changes
what an assurance owner is being asked to accept.

| Gate | Result |
|---|---|
| `cargo test --workspace` | **exit 0. 20 runners, 20 ok, 200 passed, 0 failed, 0 ignored, 0 filtered out** — re-run at this commit, not read off the retained capture |
| `cargo fmt --all -- --check` | exit 0, 0 diff lines |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, 0 warning or error lines, **2 crates re-linted in the run** so the result is this tree's and not a cache's. No `allow` added, no lint suppressed |
| `cargo tree -p Mokiterions` | **one line** — the package alone; dependency and dev-dependency tables empty |
| `cargo tree -p mokiterions-tui` | 111 lines, identical to the pre-change commit's line for line with the checkout path normalised |
| `python scripts/validate_engineering_artifacts.py` | PASS — 80 artifacts, 0 errors, 0 warnings, across all four planes |
| `bash scripts/check_engineering_harness.sh` | PASS — 80 artifacts, 248 relations, 0 errors |
| `python scripts/inspect_engineering_artifacts.py` | 0 errors, **12 warnings**, 8 informational — **two warnings are new and caused by this change**, see the disclosures below |

Zero ignored and zero filtered out is the part worth stating: a suite can be made to pass by not
running, and those two counts are what would show it. `static-checks.txt` retains the transcripts and
`analysis/capture-static.sh` reproduces them.

The three harness rows are the state **at the candidate commit**, where this record already exists in the
retired form this one replaces — that is what a re-capture means, and it is why the counts include it. A
worktree at this commit with the file removed reports 79 artifacts and 246 relations, 0 errors and 0
warnings across all four planes, and 12 warnings with 7 informational: this record's contribution to the
graph is itself, its two relations and the one informational observation asking for the decision it
exists to ask for. The two new warnings are `ARCH-MOK-001` now predating `SPEC-MOK-001` and
`SPEC-MOK-002`, which it declares conformance to; they belong to the change rather than to this record,
and the reason the artifact was not edited to silence them is below. The five `W-HEX-003` observations
the inspector reports are not all this change's, which is measured rather than assumed: three of them —
`ADR-MOK-001` → `ARCH-MOK-001`, `ARCH-MOK-002` → `SPEC-MOK-003` and `ARCH-MOK-002` → `SPEC-MOK-004` —
are present at `master`'s tip without this branch.

`VER-MOK-010`'s central claim is that the two frozen decision sources are untouched and the new one
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
- **Oracle 4 — the in-memory character buffer, cell by cell.** **996 bar rows across the 85 of 157
  probed frames that draw a roster**, rebuilt from `SPEC-MOK-003` rule 4's named parts rather than read
  back from the product, **0 discrepancies**, with the fourth gauge at its predicted absolute columns at
  **all eight** roster-drawing viewports — label 36, bar 38–39, value 41–43. Re-derived against the
  merged tree, where rule 5 as amended presents the roster at more viewports than the withdrawn tier
  table did; every figure moved upward and nothing true of the earlier capture became false.
  `observer/roster-frames.txt`.
- **Oracle 5 — the governance state of the amended artifacts.** Its **first** condition holds: every
  provision the owner approved on 2026-08-19 is present in both the amendment record and the
  specification text, checked over disjoint text, with the two amend-by-deletion provisions shown to
  have deleted and **12 of 12** controls on the checks themselves holding. The merge made two of its
  checks ambiguous and both were re-derived rather than left to read clean: row immobility stays on the
  branch point, because it establishes that *this branch* moved no earlier row, while the
  untouched-artifact checks move to `master`'s tip, because `VREC-MOK-005` is no longer the file this
  work started from; and every amendment row present at `master`'s tip is checked to be present here
  byte for byte, because a merge can lose a row with nothing on this branch having touched it. Its
  **second** condition does not hold. `amendment-approvals.md`.

Four structural obligations, each measured rather than argued:

- **The public interface grew by exactly two values and lost nothing.** Public items 49 → 49, public
  fields 42 → 43 (`AgentSnapshot.fear`), enum variants 47 → 48 (`Policy::Individual`); 2 additions, 0
  removals. `SPEC-MOK-002` rule 6's prohibition was re-checked rather than amended.
  `interface-and-purity.txt`.
- **The test census reconciles name by name in both directions.** 179 tests over 19 runners → **200
  over 20**, with **21 additions named in full and 0 removals**; on both sides the listed names equal
  the sum of the runners' own declared totals. One runner is new — `decisions (tests/decisions.rs)` —
  and the census names what moved into it and why, because a new public-tier runner is exactly how a
  relocated test would look. The before side is `master`'s tip rather than the branch point, so
  `master`'s own ten arrivals sit there rather than among this work order's additions, and
  `analysis/test-census.py` takes the commit as an argument so the recorded command says which
  comparison was made. `test-census.txt`.
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

**`VER-MOK-010` is not satisfied at this commit.** Three of its obligations are unmet, and each is
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
2. **Oracle 5's amendment row is not satisfied.** **Four** amendments written during implementation are
   OUTSTANDING, and the contract's own words are "an amendment nobody approved is not a
   specification". Two of the four were written after the merge and could not have been listed by a work
   order approved before it. Detail in the next section.
3. **The `VREC-MOK-005` gate row is not satisfied.** `WO-MOK-005`'s six amendment rows and seven manual
   assessments were not resolved before this implementation began; the repository owner overrode the
   gate on 2026-08-19. **`master` has since transitioned that record to `verified`, which does not close
   the gate**: the record's own text says the transition accepted the automated evidence with all seven
   manual assessments outstanding and eleven provisions across four approved artifacts awaiting the
   technical owner. The status moved and the substance did not. `amendment-approvals.md` §4 checks the
   mitigation rather than asserting it — every amendment row dated before 2026-08-19 is byte-identical to
   `60fda9f`, every row `master` carried at `7a2b502` survived the merge byte for byte, and neither
   `VREC-MOK-005` nor `ARCH-MOK-001` was changed by this branch — but the row is unmet, and **it is a
   cost carried forward, not a debt paid**.

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

Seven amendments were written beyond the list `WO-MOK-010` states. **One is approved, four are
OUTSTANDING because ratification is the technical owner's act and the owner has not taken it, and two
change no provision and so require none.** This record cannot supply a ratification, and transitioning it
to `verified` would not supply one either — an assurance owner's verification is a statement about
evidence, not about another role's approvals. Whether a row is outstanding is read off the specification's
own text by oracle 5 rather than asserted here; that check is what raised the count from the two the
retired candidate named.

Three were written before the merge and are the first three below. The last four exist because two
owners' approvals of the same date met in one tree, and an approved work order drafted before the merge
could not have listed them.

- **Approved.** `SPEC-MOK-001`'s trait range narrowed from `0..=100` to `0..=40`, with rule 19's
  upper-bound note and the two acceptance examples that cited unreachable tolerances. The repository
  owner, acting as technical owner, chose narrowing over amending `REQ-MOK-034`'s survivor floor on
  2026-08-19 when `WO-MOK-010` stop condition 6 fired; `escalation.md` retains the measurement the
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
- **No ratification required — it changes no provision.** `SPEC-MOK-003` gains a row recording the
  reconciliation of `WO-MOK-005`'s rule 5 amendment with this work order's rule 4 amendment, which were
  approved on the same date by the same owner against different trees and met in the merge. Both are
  retained verbatim, neither is edited or folded into the other, and the row adds, removes and rewords no
  provision: what the merge changes is a set rather than a rule, since rule 5 as amended presents the
  roster at eight declared viewports rather than four. That row names one thing as outstanding — the
  oracle 4 frame capture taken against the withdrawn tier table — and **that re-derivation is discharged
  at this commit**, at 996 bar rows over 85 roster-drawing frames. The row is not edited to say so,
  because a row is never edited once written.
- **OUTSTANDING.** `SPEC-MOK-003` rule 4 clause 7 amended in two provisions, so that clause 5's four
  gauges coexist with clause 7's bands. The two clauses meet at exactly one point — clause 5 makes the
  bar row four gauges, clause 7 bands "each of the three bars" — which left the fourth gauge unstated.
  The banded set is now named rather than counted: health, satiety and energy take the band, and `fear`
  renders as a bar and a numeric value with no colour, because the three bands are a survival scale on
  which a high value is a good one and `fear` inverts that. The collapsed one-line form's count is
  corrected from three numeric values to four. **The repository owner chose the banded set on 2026-08-19;
  the wording is the implementation agent's and needs the technical owner's ratification.**
- **OUTSTANDING.** `SPEC-MOK-004`'s recorded test-count figures corrected for this work order, under rule
  11 of that specification, which instructs a work order that adds a test to correct the counts there.
  This one adds twenty-one. `WO-MOK-010` states no provision of `SPEC-MOK-004`, so the correction is
  beyond its list although the instruction to make it comes from the amended artifact itself. **It needs
  the technical owner's ratification.**
- **No ratification required — it changes no provision.** A further `SPEC-MOK-004` row corrects rule 11's
  pointer, which named a census figure of 190 that the re-taken capture no longer carries. A capture is
  re-run rather than edited, and an amendment row is not edited once written, so the correction is a new
  row — the precedent is this work order's own *Help output* row above. It states a fact about retained
  evidence and ratifies nothing; what it points at is the row above it, which is OUTSTANDING.

The six `OUTSTANDING` amendment rows that `SPEC-MOK-002`, `SPEC-MOK-003` and `ARCH-MOK-001` carry from
`WO-MOK-005` are untouched and still say OUTSTANDING; by `VREC-MOK-005`'s own count that layer is now
eleven provisions across four approved artifacts. No commit-bound record was edited by this branch —
`master` re-captured and transitioned `VREC-MOK-005` itself, which is `master`'s act and is checked here
rather than inherited on trust.

**An architecture artifact was deliberately not edited**, and this is the reason the harness inspection
reports two warnings it did not report before. `ARCH-MOK-001` (2026-08-18) now predates `SPEC-MOK-001`
and `SPEC-MOK-002` (2026-08-19), in each case a specification it declares `conforms_to`, so the inspector
asks for a reassessment. `ARCH-MOK-002` predates `SPEC-MOK-003` and `SPEC-MOK-004` in the same way, but
those two observations are `master`'s and not this change's — they are present at `7a2b502` without this
branch, because `master` amended both specifications itself — which is measured rather than argued: the
inspector reports three `W-HEX-003` observations there and five here. The
substantive reassessment was in fact made — the technical owner confirmed on 2026-08-19 that
`ARCH-MOK-001`'s boundaries, prohibited patterns and dependency prohibition hold unchanged, recorded
in `WO-MOK-010`'s *Decision record* — but it was recorded in the work order rather than in the
architecture artifact, so the date comparison still fires. Bumping an approved artifact's `updated`
field to silence a warning would have been a governance act on an artifact this work order was
authorized to *confirm* and not to amend, and would have created a further unratified amendment. **The
warning is left standing.** Closing it is the artifact owner's act, in `ARCH-MOK-001`; the two
observations against `ARCH-MOK-002` are `master`'s to close.

## What the accountable assurance owner must weigh before verifying

The three unmet obligations and the four unratified amendments above are the first items.
`completion-summary.md` §16 discloses fifteen findings and its §4 two more; none is a failure against
an automated case, and each is stated so that verification, if given, is given knowingly. The nine that
bear most on how much the green gates mean:

1. **Individuality is demonstrated at the scale it was measured, and that scale is the weakest result
   in the packet.** `REQ-MOK-033`'s real-run divergence case yields **10, 3, 3, 5 and 3** divergent
   situations per thousand-tick run — within a factor of three of the figure `VER-MOK-010` itself names
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
   `VER-MOK-010`'s finding condition is not triggered. **The seed-0 margin is 0.008 percentage
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
`VREC-MOK-005` gate, the five outstanding assessments, the four unratified amendments and the two new
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
- **`WO-MOK-010` remains `in_progress`**, matching `WO-MOK-005` and `WO-MOK-006` and differing from
  `VREC-MOK-001` through `VREC-MOK-004`, whose work orders were transitioned to `implemented` first.
  Verification is carried by this record rather than by a change to the work order, and
  `scripts/validate_engineering_artifacts.py` reports no error or warning against that. Whether `WO-MOK-010` should also move
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
5. **Four amendment ratifications — technical owner.** The `SPEC-MOK-001` *Help output* correction, the
   three extra `SPEC-MOK-003` provisions, `SPEC-MOK-003` rule 4 clause 7's two reconciled provisions, and
   `SPEC-MOK-004`'s corrected test counts. The last two are consequences of the merge rather than of the
   implementation, and the owner's substantive decision on the banded set is already recorded — what is
   outstanding on that one is ratification of the agent's wording.
6. **The `VREC-MOK-005` layer — repository owner.** Six amendment rows and seven manual assessments,
   eleven provisions by that record's own count, which the override deferred rather than discharged.
   `master`'s transition of that record to `verified` did not discharge them either. This is the only item
   that cannot be closed inside `WO-MOK-010`'s scope, and oracle 5's second condition stays unmet until it
   is.

Items 1 to 5 are all judgements or ratifications; **none is waiting on code, a measurement or a
re-run.** All five automated oracles pass at this commit.

## Scope of the transition being requested

**No transition is requested yet.** Transitioning this record to `verified` as it stands would record
that the assurance owner accepts the evidence **with five of `VER-MOK-010`'s seven manual assessments
unperformed, a sixth unsigned, four amendments to approved artifacts unratified, and two rows of the
contract's own requirement-to-evidence matrix unsatisfied** — which is not a judgement about
disclosures but the contract's own stated condition for being unsatisfied. This record should stay
`ready` until the six items above are closed, or until the owner decides on the record and states what
is being accepted in their place.

It would not perform those assessments, would not supply those ratifications, and would not merge,
release, tag, publish or deploy anything.

The candidate commit sits on `feature/phase-2-individuality`, which now has an upstream: the branch is
pushed to `origin` and carries **draft pull request #17**, opened on 2026-08-19 on the repository owner's
instruction and pushed further on the same instruction. The retired candidate was written when nothing had
been pushed and said so, which is one of the facts this re-capture corrects. **A push and a draft pull
request are not verification, not merge and not release**: nothing is merged, no review is approved, the
pull request is a draft, and its body carries `Harness-Work-Order: WO-MOK-010`, which is the line the
harness `candidate` job selects its work order from and not an authorization of anything. Release remains
a separate record and a separate accountable decision.
