+++
id = "VREC-MOK-010"
type = "verification_record"
title = "Verification candidate for WO-MOK-010"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"
commit = "1a937a1a9a3ff24c23e45946ad023bde95f83d02"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-19T17:53:16Z"
artifact_snapshot_sha256 = "f9bc57c5ab705ca4f73959b9787e3594009f0c55f51c8a3feab678f7f950627f"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-010/README.md", "docs/engineering/simulation/evidence/WO-MOK-010/amendment-approvals.md", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/amendments.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/analyze.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/capture-static.sh", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/equivalence.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/frames.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/interface-census.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/long-horizon.sh", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/static-checks.py", "docs/engineering/simulation/evidence/WO-MOK-010/analysis/test-census.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/COMMIT.txt", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/capture.sh", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/compare.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed0-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed1-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed123-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed42-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-baseline-seed777-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed0-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed1-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed123-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed42-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed42-ticks20-trace.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/full/pre-reference-seed777-ticks1000.log", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/pre-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/projection.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/rebuild-check.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/rebuild-check.txt", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/recapture-check.py", "docs/engineering/simulation/evidence/WO-MOK-010/baseline/recapture-check.txt", "docs/engineering/simulation/evidence/WO-MOK-010/closing-review.md", "docs/engineering/simulation/evidence/WO-MOK-010/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-010/escalation.md", "docs/engineering/simulation/evidence/WO-MOK-010/interface-and-purity.txt", "docs/engineering/simulation/evidence/WO-MOK-010/manual-assessment.md", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/divergence.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/equivalence.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/fear.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/long-horizon.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/oscillation.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/proposals.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/traits.txt", "docs/engineering/simulation/evidence/WO-MOK-010/measurements/viability.txt", "docs/engineering/simulation/evidence/WO-MOK-010/negative-control/controls.py", "docs/engineering/simulation/evidence/WO-MOK-010/negative-control/oracle-2.txt", "docs/engineering/simulation/evidence/WO-MOK-010/negative-control/oracle-3.txt", "docs/engineering/simulation/evidence/WO-MOK-010/observer/frame-probe.rs", "docs/engineering/simulation/evidence/WO-MOK-010/observer/roster-frames.txt", "docs/engineering/simulation/evidence/WO-MOK-010/post/additivity.txt", "docs/engineering/simulation/evidence/WO-MOK-010/post/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-010/post/full/post-individual-seed42-ticks20-trace.log", "docs/engineering/simulation/evidence/WO-MOK-010/post/full/post-reference-seed42-ticks20-trace.log", "docs/engineering/simulation/evidence/WO-MOK-010/post/post-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-010/renumbering.md", "docs/engineering/simulation/evidence/WO-MOK-010/requirement-to-test-mapping.md", "docs/engineering/simulation/evidence/WO-MOK-010/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-010/test-census.txt"]

[relations]
verifies_work_order = ["WO-MOK-010"]
conforms_to = ["VER-MOK-010"]
+++

# Verification Record Candidate

This ready record binds the retained evidence for `WO-MOK-010` to candidate commit
`1a937a1a9a3ff24c23e45946ad023bde95f83d02`. An accountable assurance owner must review the evidence
and decide whether to transition it to `verified`. Preparing it approved, verified, merged, tagged,
released and published nothing.

**It is the second re-capture.** The candidate this replaces bound
`035a001169757464c7f2eda2e2dfafc06b3f8910` and the one before that
`4f32a9f0accb141fb9d21795faf9554bdd3afbd6`; the section after next says what moved each time and why the
figures below are not the figures either carried. The commit this one names is the closing review of
2026-08-19, which recorded the seven manual assessments `VER-MOK-010` requires and the four amendment
ratifications the technical owner owed — so what moved is the governance state, and not one measurement.
A `ready` record may be re-captured; a `verified` one may not, and none was touched.

The record is intentionally created after the candidate commit it names, avoiding self-referential
commit metadata. **`verified_at` above is the capture timestamp, not a verification decision** — the
capture was taken at `2026-08-19T17:53:16Z` against a clean worktree at this commit, and the
`artifact_snapshot_sha256` is the harness snapshot of the **80**-artifact, 248-relation graph as it
stood at that commit, so it records the graph this record binds rather than the graph containing this
text. On a re-capture that distinction is sharper than on a first capture and is worth stating exactly:
`dashboard-data.json` embeds this record's own `commit`, `verified_at` and `artifact_snapshot_sha256`
fields, so the graph it digests is the one holding the **retired** form of this file, and the digest
could not name the graph holding this form without being an input to itself. It also embeds
`repository.revision`, which is `1a937a1` — the digest names both the graph and the commit, and that is
measurable rather than asserted: the same artifact content digested one commit earlier gives
`e2571bec…`, not the `f9bc57c5…` above, because the revision is one of the inputs. **The figure quoted
in the closing review's own commit message is that `e2571bec…`**, computed over this artifact content
before the commit existed and therefore naming its predecessor's revision; the value that belongs to the
candidate commit is the one in the front matter, and a reader re-running
`scripts/check_engineering_harness.sh` at `1a937a1` gets it. The status is `ready` and no verification
decision has been taken by anyone.

## The objection this record was written over, and how it lapsed

The two earlier candidates were written over their own evidence packet's stated objection.
`manual-assessment.md`, `requirement-to-test-mapping.md`, `README.md` and `completion-summary.md` §16 all
said the same thing: **"no verification record can be written against this commit."** That sentence was in
the retained evidence, it was written before either candidate, and it was named at the top of each rather
than walked past. Two readings of it were set out — that it forbids a *`verified`* record, which is the
reading those candidates applied, or that it forbids writing anything at all until the assessments are
recorded — and the record said plainly that **the choice between them was the implementation agent's and
should not have been.**

**The objection has now lapsed, and it lapsed by the owner recording the judgements rather than by the
sentence being edited away.** On 2026-08-19 the repository owner recorded all seven manual assessments and
ratified the four outstanding amendments, each as its own act, and the packet's sections were then written
to say what was decided. The clause the objection rested on — "this contract is not satisfied while any
[assessment] remains outstanding" — is satisfied at this commit. What survives is narrower and is stated in
full below: **one row of `VER-MOK-010`'s requirement-to-evidence matrix is still unsatisfied**, the
`VREC-MOK-005` gate, which the owner overrode and, in the same review, let stand with an obligation
attached rather than discharged.

Two things about that lapse are worth a reader's suspicion, so both are stated here.

- **The evidence packet was edited in the same commit this record binds.** A packet that has been
  corrected into agreeing with a record is not evidence of anything, which is the discipline the packet
  applies to its own captures. What was edited is the governance sections and only those: the decisions
  the owner took, where they are recorded, and the counts that follow from them. **No measurement, no
  digest, no capture, no test and no provision changed** — `git diff 035a001 1a937a1 -- ':!docs'` is
  empty, every `measurements/`, `baseline/`, `post/`, `observer/` and `negative-control/` file is
  byte-identical to `035a001`, and the retired candidates' text survives in git history and in the
  section below rather than being replaced by a tidier account.
- **The harness signal is unchanged by this record's presence except for the decision it asks for.** With
  this file in the tree, `scripts/inspect_engineering_artifacts.py` reports `decision_required ->
  review-assurance-decision (assurance-owner)` against `VREC-MOK-010`; with the file removed from a clean
  worktree at this commit it reports that queue **empty** — 79 artifacts, 246 relations, 0 errors and 0
  warnings across all four planes, 12 warnings and 7 informational, against this commit's 80, 248, 12 and
  8. Adding the record raises the signal by exactly one informational observation — `I-REV-001`, which
  compares a record's declared candidate commit against the observed checkout — and changes no error and no
  warning. The gate section below says why that observation is about this file's position in history and
  not about its content.

## Why this record was re-captured, and what the retired candidates said

**This re-capture.** The candidate retired here bound `035a001169757464c7f2eda2e2dfafc06b3f8910`, captured
at `2026-08-19T16:50:02Z` over an 80-artifact, 248-relation graph digesting to `71dc899e…`. It was accurate
when it was taken and is not accurate now, for one reason: the closing review of 2026-08-19 changed the
governance state it reported. A record binds a commit, so a governance state it no longer describes is
re-captured and not corrected in place.

| Figure | Retired candidate at `035a001` | This candidate at `1a937a1` |
|---|---|---|
| Manual assessments | 1 recorded, 1 recorded in substance, **5 outstanding** | **7 of 7 recorded**, `VER-MOK-010`'s manual-assessment clause satisfied |
| Amendments beyond the approved list | 7, of which **4 OUTSTANDING** | 7, of which **4 ratified by the technical owner on 2026-08-19**, 1 approved under a stop condition, 2 needing none |
| Unsatisfied matrix rows | **2** — the amendments row and the `VREC-MOK-005` gate | **1** — the `VREC-MOK-005` gate alone |
| Oracle 5 controls | 12 | **17**, five of them on the approval state a row carries |
| `VREC-MOK-005` layer | override recorded, debt unnamed | override **stands with a stated obligation**: resolved by a work order of its own, before the next release record |
| Evidence paths bound | 58 | **59** — `closing-review.md` added |
| Harness snapshot | `71dc899e…` | `f9bc57c5…` |

Every measured figure below is the figure that candidate carried, because nothing measurable moved:
`git diff --stat 035a001 1a937a1 -- ':!docs'` is empty, so the suite, the oracles, the interface census
and every retained capture stand exactly as they stood. `cargo test --workspace`, `cargo fmt`,
`cargo clippy` and both dependency trees were nevertheless re-run at this commit rather than carried on
that emptiness, and clippy was re-run into a fresh target directory so that both workspace crates were
actually re-linted rather than read from a warm cache.

**The first re-capture, for a reader coming to this file cold.** The candidate before that one bound
`4f32a9f0accb141fb9d21795faf9554bdd3afbd6`, captured at
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

What moved between those two candidates, so a reader of the first can see the difference rather than
diff it:

| Figure | First candidate at `4f32a9f` | Second candidate at `035a001` |
|---|---|---|
| Workspace suite | 20 runners, 190 passed | 20 runners, **200 passed** — re-run at this commit |
| Test census | 169 over 19 runners → 190 over 20, before side `60fda9f` | **179 over 19 → 200 over 20**, before side `master`'s tip |
| Oracle 4 | 864 bar rows over 134 probed frames, 4 roster-drawing viewports | **996 bar rows over the 85 of 157 probed frames that draw a roster**, 8 viewports |
| Validator / harness | 76 artifacts, 240 relations | **80 artifacts, 248 relations** |
| Inspector | 10 warnings, 6 informational, 3 warnings new to this change | **12 warnings, 8 informational, 2 new to this change** |
| Amendments beyond the approved list | 3, of which 2 OUTSTANDING | **7, of which 4 OUTSTANDING** and 2 changing no provision |
| Oracle 5 controls | 7 | **12**, and a second base for the checks the merge made ambiguous |
| `VREC-MOK-005` | `ready`, gate overridden | **`verified` by `master`'s act**, gate still not met |

Five results did not move across that re-capture, and that is a measurement rather than an omission:
`master` did not touch the engine by one byte — `git diff --stat 60fda9f 7a2b502 -- mokiterions-core/` is
empty — so oracle 1, oracle 2, oracle 3, the `fear` measurements and the public-interface census stand at
this commit with the before side they were taken against. **No figure in this record is carried over from
either retired candidate without being re-run here or resting on a measured emptiness**, and both
emptinesses are named: `60fda9f`…`7a2b502` for the engine across the merge, and `035a001`…`1a937a1` for
everything outside `docs/` across the closing review.

## What this record claims

`WO-MOK-010` is `in_progress` and `VER-MOK-010` is `approved`. At candidate commit
`1a937a1a9a3ff24c23e45946ad023bde95f83d02`, **every automated case, oracle, static check and
comparison in `VER-MOK-010` was executed and passed, and its manual-assessment clause is satisfied: all
seven judgements are recorded by the accountable role. One of the contract's obligations is still not
satisfied — one row of its requirement-to-evidence matrix, the `VREC-MOK-005` gate, unsatisfied by the
repository owner's recorded override — so this record still cannot claim that the contract is met in
full.** That is the difference between this candidate and `VREC-MOK-001` through `VREC-MOK-004` and
`VREC-MOK-006`, and it is stated here because it changes what an assurance owner is being asked to accept.
It is also narrower than what the two retired candidates asked: they carried five unperformed assessments,
a sixth unsigned and four unratified amendments besides.

| Gate | Result |
|---|---|
| `cargo test --workspace` | **exit 0. 20 runners, 20 ok, 200 passed, 0 failed, 0 ignored, 0 filtered out** — re-run at this commit, not read off the retained capture |
| `cargo fmt --all -- --check` | exit 0, 0 diff lines |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, 0 warning or error lines, **both workspace crates re-linted** — run into a fresh target directory, because at this commit the repository's own cache is warm and nothing outside `docs/` changed, so a cached exit 0 would have been the previous tree's result. No `allow` added, no lint suppressed |
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
warnings across all four planes, and 12 warnings with 7 informational, with the `decision_required` queue
empty: this record's contribution to the graph is itself, its two relations, the queue entry asking for the
decision it exists to ask for, and one informational observation. **That observation is `I-REV-001`, and
what it observes is where this file sits in history rather than anything about its content**: the rule
fires when the observed checkout differs from the candidate commit a record declares, which is true at the
candidate commit of the retired form declaring `035a001`, and true again from the commit that carries this
form, which is one commit later than the `1a937a1` it declares. Put this form's front matter into a
worktree at `1a937a1` and the count drops to 7, because declared and observed then agree — a configuration
no commit will ever hold, measured only to show what the observation is about. It is informational in the
harness's own classification and not a warning. The two new warnings are `ARCH-MOK-001` now predating `SPEC-MOK-001` and
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
  have deleted and **17 of 17** controls on the checks themselves holding. It now also checks the
  **approval state of each amendment written beyond the approved list**, read off the specification's own
  row rather than asserted here: a row that needed the technical owner's ratification fails unless its
  Approval column names the review that ratified it, the date, the ratifying role, **and** the
  `OUTSTANDING` state it was in until then, and a row that changes no provision is required to carry
  neither marking. All four ratified rows pass all four tests. Five of the seventeen controls are on that
  check, and they replace one that could pass for the wrong reason: the earlier check asked only whether
  `OUTSTANDING` appeared anywhere in a row, which the `SPEC-MOK-003` three-provisions row satisfied by way
  of an unrelated mention of another row's marking. The merge made two of its
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

**`VER-MOK-010` is not satisfied in full at this commit.** One of its obligations is unmet, and it is
quoted rather than paraphrased. The two that were unmet at the retired candidates are recorded here as
closed, with the act that closed each, because a reader of those candidates should be able to see what
changed rather than find the objection quietly absent.

1. **Closed — the manual-assessment clause is satisfied.** The contract: "An unrecorded assessment is an
   outstanding assessment, and this contract is not satisfied while any remains outstanding." All seven are
   recorded by the accountable role, on 2026-08-19: 1, 2 and 3 by the product owner, 4, 5 and 7 by the
   technical owner, 6 by the assurance owner. Each was put as its own question and answered on its own, so
   no judgement rests on an approval given for another; the implementation agent put the questions with the
   measured facts assembled and transcribed the answers, and decided none of them. `manual-assessment.md`
   holds each judgement with the measurement it was taken on, and `closing-review.md` holds the acts. Two
   carry adverse figures and were decided on them rather than around them: assessment 2 on the divergence
   count, **not** on the more favourable eats figure, and assessment 4 keeping `+10`/`-5` with the 39%
   ceiling residency recorded as an observation.
2. **Closed — oracle 5's amendment row is satisfied.** The four amendments written during implementation
   that were OUTSTANDING were ratified by the repository owner acting as technical owner on 2026-08-19,
   each in its own act, and each specification's amendment record carries the ratification where a reader
   of the specification meets it. The contract's own words were "an amendment nobody approved is not a
   specification"; they still apply to the earlier layer named in item 3. Detail in the next section.
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
  intended, whether 3 to 10 divergences per run is individuality — are the seven assessments, now
  recorded; recording them does not convert any of them into an automated result, and each remains a
  judgement that a later reader may weigh against the measurement it was taken on.
- **`fear` has no consumer, so no outcome can falsify its constants.** One writer, no reader, by
  census. What is verified is that the attribute is maintained, bounded, perception-driven and
  reported; whether `+10`/`-5` is the right pair becomes answerable when something reads it.

## Amendments to approved artifacts written beyond the work order's list

Seven amendments were written beyond the list `WO-MOK-010` states. **One was approved as a decision under
a stop condition, four were ratified by the repository owner acting as technical owner on 2026-08-19, and
two change no provision and so required none.** This record still supplies no ratification — an assurance
owner's verification is a statement about evidence, not about another role's approvals — and it did not
supply these: the owner took each of the four as its own act, and each specification's amendment record
carries it in the row's own **Approval** column, where a reader of the specification meets it rather than
here. Whether a row is ratified is read off the specification's own text by oracle 5 rather than asserted
here; that check is what raised the count from the two the first candidate named, and it is what now
confirms the four.

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
- **Ratified 2026-08-19.** `SPEC-MOK-001`'s *Help output* correction — a correction to this work order's
  *own* first amendment, which required the explanatory prose to name the default source and so
  contradicted the same section's approved *stated once* paragraph. The inherited test
  `cli::each_declared_default_is_stated_once`, bound by a `verified` `VREC-MOK-004`, asserts the side
  the implementation is on, so satisfying the withdrawn clause would have meant relaxing an assertion
  this work order forbids. It corrects text the technical owner approved earlier the same day, and **that
  owner ratified the withdrawal on 2026-08-19**, choosing the withdrawal over relaxing the inherited test.
  The row's Change column is untouched and still describes the withdrawal; the ratification is in its
  Approval column, which also retains that the row was OUTSTANDING until that act.
- **Ratified 2026-08-19.** Three further `SPEC-MOK-003` provisions outside rule 4: the `AgentSnapshot`
  field list gains `fear`, rule 10 item 7 loses `fear` and traits from its list of values the engine does
  not compute, and rule 11's `decision_source_selected` row gains `REQ-MOK-033` for `individual`. Each is
  forced by the change rather than chosen with it, and each was written into the 2026-08-19 amendment
  row rather than made quietly. **The technical owner ratified all three**, as one act on the three
  provisions together, having been shown each separately; the row records the ratification and that it
  was OUTSTANDING before it.
- **No ratification required — it changes no provision.** `SPEC-MOK-003` gains a row recording the
  reconciliation of `WO-MOK-005`'s rule 5 amendment with this work order's rule 4 amendment, which were
  approved on the same date by the same owner against different trees and met in the merge. Both are
  retained verbatim, neither is edited or folded into the other, and the row adds, removes and rewords no
  provision: what the merge changes is a set rather than a rule, since rule 5 as amended presents the
  roster at eight declared viewports rather than four. That row names one thing as outstanding — the
  oracle 4 frame capture taken against the withdrawn tier table — and **that re-derivation is discharged
  at this commit**, at 996 bar rows over 85 roster-drawing frames. The row is not edited to say so,
  because a row is never edited once written.
- **Ratified 2026-08-19.** `SPEC-MOK-003` rule 4 clause 7 amended in two provisions, so that clause 5's
  four gauges coexist with clause 7's bands. The two clauses meet at exactly one point — clause 5 makes the
  bar row four gauges, clause 7 bands "each of the three bars" — which left the fourth gauge unstated.
  The banded set is now named rather than counted: health, satiety and energy take the band, and `fear`
  renders as a bar and a numeric value with no colour, because the three bands are a survival scale on
  which a high value is a good one and `fear` inverts that. The collapsed one-line form's count is
  corrected from three numeric values to four. The repository owner chose the banded set earlier on
  2026-08-19; the wording was the implementation agent's, and **that owner ratified the wording as written
  on 2026-08-19** rather than restating it. The row keeps the distinction it drew — that choosing the set
  and wording the clause were separate acts — and the second act is now recorded beside the first.
- **Ratified 2026-08-19.** `SPEC-MOK-004`'s recorded test-count figures corrected for this work order,
  under rule 11 of that specification, which instructs a work order that adds a test to correct the counts
  there. This one adds twenty-one. `WO-MOK-010` states no provision of `SPEC-MOK-004`, so the correction
  is beyond its list although the instruction to make it comes from the amended artifact itself. **The
  technical owner ratified the corrected figures in full on 2026-08-19**, on the stated reading that a
  count and the census it is read from are not separately ratifiable — which is what the next bullet
  turns on.
- **No ratification required — it changes no provision.** A further `SPEC-MOK-004` row corrects rule 11's
  pointer, which named a census figure of 190 that the re-taken capture no longer carries. A capture is
  re-run rather than edited, and an amendment row is not edited once written, so the correction is a new
  row — the precedent is this work order's own *Help output* row above. It states a fact about retained
  evidence and ratifies nothing; what it points at is the row above it, which now carries the technical
  owner's ratification.

**What the four ratifications do not reach.** They make the amended text specification rather than draft;
they do not verify the implementation against it, they do not satisfy the one matrix row still unmet, and
they do not touch the earlier layer below. Nor do they retroactively approve the way the amendments were
written: each was written by the implementation agent before it was ratified, which is why the row retains
that it stood OUTSTANDING in the interval rather than reading as though it had been approved from the
start. Oracle 5 checks that retention as one of its controls, so a row cannot be made to look
approved-all-along by an edit.

The six `OUTSTANDING` amendment rows that `SPEC-MOK-002`, `SPEC-MOK-003` and `ARCH-MOK-001` carry from
`WO-MOK-005` are untouched and still say OUTSTANDING; by `VREC-MOK-005`'s own count that layer is now
eleven provisions across four approved artifacts. **The closing review of 2026-08-19 let that override
stand and attached an obligation to it — a work order of its own, resolving those eleven provisions and
seven assessments, completing before the next release record.** That obligation is recorded, not
discharged: nothing here approves, assesses or resolves any of them, and no such work order exists at this
commit. No commit-bound record was edited by this branch —
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

The one unmet obligation above is the first item, and the seven judgements the owner recorded on
2026-08-19 are the second: a verification is a statement about evidence, and part of this evidence is now
a set of recorded decisions whose *reasoning* the assurance owner may weigh even though the decisions
themselves are not theirs to retake. `completion-summary.md` §16 discloses fifteen findings and its §4 two
more; none is a failure against an automated case, and each is stated so that verification, if given, is
given knowingly. The nine that bear most on how much the green gates mean:

1. **Individuality is demonstrated at the scale it was measured, and that scale is the weakest result
   in the packet.** `REQ-MOK-033`'s real-run divergence case yields **10, 3, 3, 5 and 3** divergent
   situations per thousand-tick run — within a factor of three of the figure `VER-MOK-010` itself names
   as a failure — and **zero** cases across all five seeds of two Mokiterions facing the same situation
   on the same tick, so no divergence is ever visible side by side in one frame. The 54 to 97
   waste-accepting eats per run, by 9 or 10 distinct Mokiterions of twelve, are the same behavior
   counted without requiring a coincidence, but substituting a more favourable measure for the one the
   approved contract names was the product owner's call, and on 2026-08-19 **that owner declined to
   substitute it** — assessment 2 is recorded as satisfied on the divergence count itself, with the eats
   as corroboration rather than as the measure. So the figure the assurance owner weighs is 3 to 10, not
   54 to 97, and the recorded judgement does not soften it.
2. **`fear` sits at its ceiling on 39% of agent-ticks**, and `0` is by a wide margin the most common
   transition — `-5` on 13,940, `0` on 90,201, `+5` on 219, `+10` on 7,244. This follows from the
   approved `+10`/`-5` ratchet against an expected 0.73 other Mokiterions inside the radius, and rule
   12 states that saturation is a normal outcome rather than an error. The technical owner kept `+10`/`-5`
   on 2026-08-19 **with the 39% recorded as an observation rather than dismissed**, on the reasoning that
   no outcome can falsify the pair while nothing reads the attribute. It is retained here because a future
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
   that decision. The owner reaffirmed that decision on 2026-08-19 **with the sweep in front of them**,
   so the judgement now stands on the narrower fact rather than on the four-bar reading it was first taken
   under.
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
`VREC-MOK-005` gate and the obligation now attached to it, the seven assessments and what their recorded
form does and does not settle, the four ratifications and what they do not reach, and the two new
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
  extinct at seed 777 on tick 9,938. `REQ-MOK-034` states no obligation on this, and the product owner
  recorded assessment 3 on 2026-08-19 as **neither an improvement nor a regression** — so the separation is
  a recorded observation and not a claimed gain, and nothing downstream may cite it as one.
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

Six items stood here at the retired candidates. **Five are closed**, each by a recorded act of the
accountable role on 2026-08-19: assessments 1, 2 and 3 by the product owner, assessment 4 by the technical
owner, assessment 5 reaffirmed by the technical owner with the reachability sweep in front of them,
assessment 6 by the assurance owner, assessment 7 signed by the technical owner, and the four amendment
ratifications by the technical owner. `closing-review.md` records each act with the role it was taken in;
`manual-assessment.md` records what each was decided on. Two items remain, and neither is waiting on code,
a measurement or a re-run — all five automated oracles pass at this commit.

1. **The assurance owner's own review of this record — assurance owner.** Nothing above substitutes for it.
   The seven assessments include one this record's own accountable role took (assessment 6, the
   projection), and a role's judgement on one evidence item is not that role's verification of the record
   that binds all of them. What is being asked is a read of the fifteen disclosures, the one unmet matrix
   row, and the recorded judgements' reasoning — not a re-taking of judgements that belong to other roles.
2. **The `VREC-MOK-005` layer — repository owner.** Six amendment rows and seven manual assessments,
   eleven provisions by that record's own count, which the override deferred rather than discharged.
   `master`'s transition of that record to `verified` did not discharge them either. On 2026-08-19 the owner
   let the override stand and **attached an obligation: a work order of its own, resolving them, completing
   before the next release record.** That names the work and schedules it; it does not do it. This is still
   the only item that cannot be closed inside `WO-MOK-010`'s scope, the matrix row it fails is still
   unsatisfied, and no such work order exists at this commit.

## Scope of the transition being requested

**No transition is requested here.** What the closing review changed is how narrow the acceptance would be,
not that one is being asked for. Transitioning this record to `verified` as it stands would record that the
assurance owner accepts the evidence **with one row of the contract's own requirement-to-evidence matrix
unsatisfied — the `VREC-MOK-005` gate, overridden with an obligation recorded and not yet discharged — and
with the fifteen disclosures above read**. That is a narrower acceptance than either retired candidate
asked for: the seven assessments are recorded and the four ratifications are given, so the transition no
longer stands in for another role's missing act. It remains an acceptance of an unsatisfied row, which is
the contract's own stated condition for not being satisfied, and it is the assurance owner's to give or
withhold.

Three acts follow this record and none is taken here: moving it from `ready` to `verified` is the assurance
owner's, moving `WO-MOK-010` from `in_progress` to `implemented` is the engineering owner's, and taking
PR #17 out of draft and merging it is the repository owner's. **PR #17 remains a draft.**

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
