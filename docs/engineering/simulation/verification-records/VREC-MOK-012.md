+++
id = "VREC-MOK-012"
type = "verification_record"
title = "Verification candidate for WO-MOK-019"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-20"
commit = "50364a3719c68643f0b5354798b6d3084cff1c0e"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-20T12:08:52Z"
artifact_snapshot_sha256 = "16862ef3408e500d5e3488f9911eba100af3de3a4e584c4c55792131f837108d"
evidence_paths = [
  "docs/engineering/simulation/evidence/WO-MOK-019/README.md",
  "docs/engineering/simulation/evidence/WO-MOK-019/additivity.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/alphabet.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/amendment-approvals.md",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/amendments.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/capture-failures.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/census-by-target.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/census-by-target.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/census-reconciliation.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/compare.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/digest.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/entropy.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/prior-captures.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/reconstruct.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/replay.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/retain-sink.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/retain.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/static-checks.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/analysis/validate.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/baseline/COMMIT.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/baseline/capture-state.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/baseline/full/seed42-baseline-d0.75-traceoff.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/baseline/full/seed42-individual-d0.75-traceoff.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/baseline/full/seed42-reference-d0.75-traceoff.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/baseline/pre-manifest.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/baseline/test-census.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/baseline/test-run.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/capture.sh",
  "docs/engineering/simulation/evidence/WO-MOK-019/completion-summary.md",
  "docs/engineering/simulation/evidence/WO-MOK-019/entropy-per-tick.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/entropy-states.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/entropy.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/failure-captures.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/gates.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/interface.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/json-validity.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/manual-assessment.md",
  "docs/engineering/simulation/evidence/WO-MOK-019/measure-sizes.sh",
  "docs/engineering/simulation/evidence/WO-MOK-019/negative-controls.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/oracle1/post-nosink-vs-post-sink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/oracle1/pre-vs-post-nosink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/oracle1/pre-vs-post-sink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/oracle2/reconstruction-result.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/oracle6/reconciliation.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/post-nosink-manifest.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/post-sink-manifest.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/post/full/seed42-baseline-d0.75-traceoff.jsonl",
  "docs/engineering/simulation/evidence/WO-MOK-019/post/full/seed42-baseline-d0.75-traceon.jsonl",
  "docs/engineering/simulation/evidence/WO-MOK-019/post/full/seed42-individual-d0.75-traceoff.jsonl",
  "docs/engineering/simulation/evidence/WO-MOK-019/post/full/seed42-reference-d0.75-traceoff.jsonl",
  "docs/engineering/simulation/evidence/WO-MOK-019/post/test-census.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/post/test-run.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/retained-sink-streams.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/sizes.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/static-checks.txt",
]

[relations]
verifies_work_order = ["WO-MOK-019"]
conforms_to = ["VER-MOK-012"]
+++

# Verification Record Candidate

This record binds the retained evidence for `WO-MOK-019` — the optional structured record stream — to
commit `50364a3719c68643f0b5354798b6d3084cff1c0e` on `feature/phase-4a-definition`. **It is
`verified`.** The repository owner, acting as accountable assurance owner, transitioned it from `ready`
on 2026-08-20; `evidence/WO-MOK-019/assurance-decision.md` records that act and what it accepted.

**The title still reads *candidate*, and that is deliberate.** The record was captured as one, and a
verification decision does not rewrite the capture — `VREC-MOK-010`'s and `VREC-MOK-011`'s titles read
the same way after their own transitions, for the same reason.

`DECISION_RIGHTS.md` reserves the transition to that role and states that record preparation never
makes it: *"Automation may prepare `ready` verification and release records from bounded Git
observations. Only accountable assurance and release owners may transition those records to `verified`
or `released`."* **Preparing this record approved, verified, merged, tagged, released and published
nothing**, and the instruction it was prepared under — *"you can prepapre the verification record, keep
it ready, and commit and push"* — supplied the authority to commit and push the file and stated no
judgement on the evidence. The judgements came in two later instructions of the same day, both recorded
below: the first validated the contract and the eight manual assessments and did **not** reach this
record, the second took the transition.

## The owner's validation of 2026-08-20, and what it did and did not do

**On 2026-08-20 the owner validated `VER-MOK-012` and all eight of its manual assessments**, in one
act, verbatim: *"i validate VER-MOK-012 and and 8 manual assessments"*. This happened **after** the
candidate commit this record binds. What it changes and what it leaves standing:

- **`VER-MOK-012`'s own stated condition for being unsatisfied is discharged.** The contract says "an
  unrecorded assessment is an outstanding assessment, and this contract is not satisfied while any
  remains outstanding". All eight are now recorded, in `manual-assessment.md`, each with its
  accountable role and the date. Every automated oracle, scenario and static check already passed at
  this commit. **That is the substance of the contract answered.**
- **`VER-MOK-012`'s status did not change and did not need to.** It was already `approved`, which is
  the terminal status for a verification contract; a contract is *satisfied* by evidence and
  judgements, not by a further status. Its text was not edited, so defect 5 — assessment 4's miscited
  rule and miscounted facts — is still in it.
- **Two bullets of the contract's *Evidence retention* list are still not satisfied as written**, and
  no validation reached them: bullet 3 asks for the post-change sink capture's standard output, held
  here as digests only, and bullet 4 asks for thirty full sink streams, of which four are retained.
  Both are disclosed with their reason and cost in the packet's `README.md`. **So `VER-MOK-012` is
  answered in substance and not satisfied in every literal respect**, and this record says which is
  which rather than reporting the validation as a clean sweep.
- **That validation did not transition this record.** The owner's instruction named the contract and
  the assessments; it did not name this record, and `DECISION_RIGHTS.md` reserves the `ready` →
  `verified` transition to the accountable assurance owner as a separate act. Nothing was approved
  here by implication. **The transition came later the same day, as its own act** — the section below
  records it — and this record stayed `ready` across the whole interval between the two, including for
  one commit of its own.
- **Nothing else moved with it.** At the moment that validation was recorded the nine defects were
  uncorrected, the `SPEC-MOK-004` rule 11 amendment made beyond `ADR-MOK-005`'s approved list was
  still unapproved, the three carried-forward `OUTSTANDING` amendment rows stood, `WO-MOK-019` was
  `in_progress`, and pull request #31 was a draft. **Each of those then moved by its own named act, bar
  one.** Approved and taken by the commit before this one: the `SPEC-MOK-004` rule 11 row. Taken by
  this commit: this record's transition. Authorized in the same turn and taken in the commits after
  this one: `WO-MOK-019` to `implemented`, and the pull request made ready for review. **Not moved: the
  three carried-forward `OUTSTANDING` amendment rows, which still stand untouched.** The nine defects
  were dispositioned rather than corrected — deferred to a correction work order — which is a decision
  about them and not a repair of them.

**What a one-act validation of eight questions does and does not supply.** The instruction names all
eight explicitly and the owner holds all three accountable roles, so each of the eight is covered. But
it is one act rather than eight separately worded judgements, so each decision recorded in
`manual-assessment.md` is **the affirmative side of the question as that file prepared it** — and three
of the eight invited the owner to name something in the alternative, where **nothing was named**:
assessment 1 invited any missing fact, assessment 5 an amendment to `ARCH-MOK-001`'s wording, and
assessment 7 any misplaced figure. Each is therefore accepted rather than resolved, and the two worth
carrying forward are that **build identity beyond the package version is accepted as not a gap in this
schema** (so if Phase 4b needs to attribute a distribution to a build rather than a version, that is a
new requirement, not a defect), and that **assessment 7's three named exceptions stand** — the
thirty-combination sweep parses the text summary line, the entropy figures come from a `#[cfg(test)]`
accessor, and the test census comes from `cargo test`. Assessment 4 was recorded against the measured
substance rather than its own prompt: **two** facts at rule **7.6**, not three at 7.8.

**Assessment 8 is a judgement and is recorded as one.** Its mechanical half — that the reconstructor
carries no event-type-specific branch — is checked at `oracle2/reconstruction-result.txt` line 103. Its
other half, that the replay consumer is derived from `SPEC-MOK-001` rather than from the engine's code,
admits no mechanical check: a line-for-line transcription of the engine would pass every test in the
packet. The owner's confirmation is the discharge the contract asked for, and **this record does not
upgrade it to a proof.**

## The assurance decision of 2026-08-20, and what it accepted

**`status` moved from `ready` to `verified` on 2026-08-20, by the repository owner acting as accountable
assurance owner.** It is a second act, distinct from the validation above, taken in answer to a
question that put this record's three available answers side by side with their measured costs. The full
note is `evidence/WO-MOK-019/assurance-decision.md`; what the decision accepted, stated in the terms
this record used as a candidate to describe what accepting it would mean:

- **The seven oracles as measured at `50364a3`**, all passing, with **oracle 5's size assertion for one
  domain of thirteen having no independent witness** — defect 1, `SPEC-MOK-006` rule 3.2's direction
  domain, was not corrected first. What bounds that acceptance is measured rather than argued: 0
  diagonal direction words appear in any of the seven retained streams, so the domain the specification
  misdescribes is one no retained record uses.
- **The two *Evidence retention* deviations**, unchanged: bullet 3's post-change standard output held as
  digests only, and four of thirty full sink streams retained. Accepting them accepts that oracle 1's
  byte-identity result is what stands in for the unretained text streams. **`VER-MOK-012` is therefore
  answered in substance and still not satisfied in every literal respect**, and this decision is
  recorded against that description of it rather than against a claim of full satisfaction.
- **The nine defects uncorrected, and deferred rather than dismissed.** The owner's disposition is a
  correction work order in Phase 4b, carrying all nine with its own verification contract and evidence.
  This record does not create that work order and does not assume it: the defects stay as
  `completion-summary.md` item 16 measured them, and the deferral is what makes them a scheduled debt
  rather than an open question.
- **The eight manual assessments as recorded**, including assessment 8, whose second half is a reading
  and not a proof, and assessment 4, recorded against measured substance rather than its own defective
  prompt.

**What the decision did not accept, because the act before it removed it from the list.** The
`SPEC-MOK-004` rule 11 amendment was still unapproved when the question was put; the owner approved it
as technical owner in the commit immediately preceding this one. So this decision does not accept an
unapproved amendment beyond `ADR-MOK-005`'s list — there is none left to accept.

**Only `status` changed.** `updated` already read `2026-08-20`, and `commit`, `git_object_format`,
`worktree_state`, `verified_at`, `artifact_snapshot_sha256`, all 55 `evidence_paths`, both relations and
the `title` are exactly as the capture produced them. **The provenance is the capture's, not the
decision's**, and a decision does not re-measure it. `assurance-decision.md` is deliberately **not**
added to `evidence_paths`: it postdates the commit this record binds, and a record's evidence set is the
capture's rather than the decision's — the same reason it was kept out of `VREC-MOK-007`'s,
`VREC-MOK-010`'s and `VREC-MOK-011`'s.

## Provenance, and what this record's figures are

The record was written after the candidate commit it names, so its own commit metadata is not
self-referential. **`verified_at` is the capture timestamp, not a verification decision**: the gate
and harness figures below were taken at `2026-08-20T12:08:52Z` from a worktree
`git status --porcelain` reported as empty at this commit, and `artifact_snapshot_sha256` is the
digest `python scripts/generate_harness_dashboard.py --root .` printed there, over the **113**-artifact,
**371**-relation graph as it stood **before this file existed**. Both are left exactly as captured.
Neither a record nor a decision on one re-measures the record's provenance, and the ordering matters:
the capture cannot be repeated once the file is in the tree, so it was taken first. **Being `verified`
does not change one figure in this section**, which is the point of separating the capture from the
decision.

What this file *did* while it was `ready` is checkable rather than rhetorical, and it was measured with
the file present. With it in the tree the validator reports **114 artifacts and 373 relations, still 0 errors
and 0 warnings** across all four planes, and `scripts/inspect_engineering_artifacts.py` moves from
`Decision required (0): none` to reporting `decision_required -> review-assurance-decision
(assurance-owner)` against `VREC-MOK-012`. **The finding count is unchanged at 19** — the same 2
`W-HEX-001`, 5 `W-HEX-003` and 12 `I-REV-001` observations, with none added against this record,
because at the worktree this was measured in the observed checkout *is* the commit this record
declares. **That did not stay true, and the follow-on was observed rather than predicted:** committing
this file moved `HEAD` past `50364a3`, and at `e840ba7` the inspector reports **20 findings — 13
`I-REV-001` rather than 12** — the addition being this record, whose declared commit is no longer the
observed checkout. That is what a governance record written after the commit it binds looks like from
then on. It is an observation about ordering, not a defect, and the error and warning counts are
unchanged at 0 and 7.
**The record raised the signal and changed no error or warning count.** Answering that signal was the
assurance owner's act, not this file's, and **it has now been answered** — the transition recorded above
is the answer, and with `status` at `verified` the inspector no longer lists this record under
`Decision required`.

**`evidence_paths` names blobs at `50364a3`, not at the branch tip.** Three of the 55 have changed
since, and only three: `manual-assessment.md`, which carried eight blank decision lines at this commit
and now carries eight decisions; `completion-summary.md`, which gained marked later-fact notes pointing
at those decisions and at the `SPEC-MOK-004` rule 11 approval, plus one corrected internal count; and
the packet `README.md`, likewise a later-fact note. **No measured figure in any of the three was
edited**, which is the rule this repository states for evidence: measure again rather than edit into
agreement. `amendment-approvals.md` is one of the 55 and is **byte-identical**, which is a measured
claim and not an assumption: `analysis/amendments.py` was re-run after the `SPEC-MOK-004` approval and
reproduces the retained file exactly, exit 0. `assurance-decision.md` is a new file in the packet
directory and is not among the 55, deliberately. Every other retained artifact, and every digest, byte
count and oracle result in the packet, is untouched. A reader checking this record against the tip
should expect exactly those three differences and no others.

**Read *What this record does not claim* before relying on this record.** That list was written against
the candidate commit and is left as written except where the validation reached it; its item 1 records
what the validation settled and what it did not.

## What this record claims

`WO-MOK-019` is `in_progress` as this record is transitioned, and `VER-MOK-012` is `approved`. The work
order is **not** moved in the same commit, and not because it must not be: the owner authorized
`implemented` in the same turn. `WORKFLOW.md` is explicit that a work order's status never substitutes
for this record's, so the two are separate commits by separate accountable roles — assurance here,
engineering next — in the order `VREC-MOK-011`'s transition and `WO-MOK-011`'s establish. The validator
permits it either way: `in_progress` and `implemented` are both in the set a `verified` record's work
order may carry.

> **Later fact, 2026-08-20, in the commit after this record's transition: `WO-MOK-019` is now
> `implemented`.** The paragraph above is what stood when this record moved, and it stands unedited
> because the sentence it makes — that the two are separate commits by separate accountable roles — is
> only demonstrated by having been written while the first had happened and the second had not. The
> engineering-owner act is recorded in that work order's *Transition to `implemented`* section with the
> derived figures measured either side of it. **Nothing in this record's substance depends on which of
> the two statuses the work order carries**, which is the reason the paragraph names the validator's set
> rather than a single permitted value.

At candidate commit `50364a3719c68643f0b5354798b6d3084cff1c0e`, **every automated case, oracle,
scenario and static check in `VER-MOK-012` was executed and passed.** The contract's eight manual
assessments had no author at that commit and were **recorded by the owner on 2026-08-20**, afterwards;
the two retention obligations named in item 3 below are still met by substitution rather than as
written, which is why this record claims the contract answered in substance rather than satisfied
outright.

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0, no output — every file formatted |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, **no warning in either package at any target**. No `allow` added, no lint suppressed. `--locked` held: `Cargo.lock` was not rewritten to make the command pass |
| `cargo test` — one invocation at the workspace root | exit 0. **246 passed, 0 failed, 0 ignored, 0 filtered out, over 22 targets** |
| `cargo tree -p Mokiterions` | **one line** — the package alone, no dependency line. `SPEC-MOK-002` rule 1's empty dependency table measured rather than assumed |
| `cargo tree -p mokiterions-tui` | 111 lines: the path dependency plus `ratatui 0.30.2` and its tree, unchanged |
| `python scripts/validate_engineering_artifacts.py` | PASS — **113 artifacts, 0 errors, 0 warnings** across structure, governance, policy and maintenance |
| `bash scripts/check_engineering_harness.sh` | PASS — 113 artifacts, **371 relations**, 0 errors |
| `python scripts/inspect_engineering_artifacts.py` | 0 errors; **`Decision required (0): none`**, `Assurance pending (0): none`, `WO-MOK-019 [in_progress]`; 19 findings — 2 `W-HEX-001`, 5 `W-HEX-003`, 12 `I-REV-001`, none of them new |

Zero ignored and zero filtered out is the part worth stating: a suite can be made to pass by not
running, and those two counts are what would show it. Every source was touched before the clippy and
test runs, so neither reports a cached result.

`VER-MOK-012`'s central claim is that a record stream was added and the observed run did not move.
That claim is not carried by one measurement. Seven oracles, each able to fail without the others
failing:

- **Oracle 1 — the text stream is unmoved, with a sink and without.** The 90-cell matrix — five seeds
  × three policies × three densities × tracing off and on, 1,000 ticks — captured pre-change at
  `de33d744` with the working tree's state recorded, then twice at the candidate tree. **Three
  comparisons, 90 cells each, 0 differing**, on standard output, standard error and exit code, over
  **114,723,785 bytes and 905,247 lines** summed from the manifests. `VER-MOK-012` declares sixty
  cells; this is a superset, adding density `0.15` so the re-run against the prior packets compares
  cell for cell. `oracle1/*.txt`, `baseline/capture-state.txt`.
- **Oracle 2 — the text stream is reconstructible from the records alone.** **90 cells, 905,247 text
  lines reconstructed, 0 cells differing**, byte for byte against the standard output the same process
  wrote. The reconstructor implements `SPEC-MOK-006` rule 6.6's generic walk and **none of the twelve
  event-type spellings appears anywhere in it** — a mechanical check, not a reading, and the property
  that makes the oracle independent of the emitter. `oracle2/reconstruction-result.txt`,
  `analysis/reconstruct.py`.
- **Oracle 3 — every record is JSON to a parser outside this repository.** **90 streams, 961,105
  records, 0 findings**, under Python 3.14.6's standard-library `json` module: every line an object
  with `record` first, every number an integer and not a float, every key a field the specification
  names, every `null` one of the three permitted absences, no duplicate key. `json-validity.txt`.
- **Oracle 4 — a sink moves no entropy draw.** **30 series, 4,388 tick-boundary rows, 4,388 equal, 0
  differing**, sink against no sink, at every boundary of every series rather than inferred from
  output; and the state after initialization and at tick 1,000 against the pre-change build, **90
  cells compared, 0 differing**, three times. The draw count never goes backwards in 30 of 30 series,
  and `k(0) == k(1)` in 30 of 30. `entropy.txt`, `entropy-states.txt`, `entropy-per-tick.txt`.
- **Oracle 5 — the value alphabet is closed, so no escaping function is needed.** **13 domains, 306
  members, 0 off the alphabet** — exhaustive over `SPEC-MOK-006` rule 3.2 rather than
  representative, with each domain's size asserted against the specification's so that a domain
  gaining a member silently fails here. `0x22`, `0x5c` and every byte below `0x20` are absent from the
  union of emitted bytes, which is the whole of rule 3.3's totality argument. The union is **53 of
  rule 3.3's 69 characters**: `+`, `:`, `;` and `>` are on the alphabet because the *text* stream's
  separators use them, not because any value carries them. A strict subset makes the argument
  stronger. `alphabet.txt`.
- **Oracle 6 — the metrics and run records reconcile against a replay of the events.** **90 streams,
  55,768 tick boundaries reconciled, 0 findings.** The replay consumer implements `SPEC-MOK-001`'s
  resource rules and was written independently of the engine, so the two disagree when either is
  wrong. `oracle6/reconciliation.txt`, `analysis/replay.py`.
- **Oracle 7 — the amendments this change depends on are approved.** **RESULT: PASS.** `ADR-MOK-005`
  is `approved`; **all 28 required provisions are found twice each**, once in the amendment record and
  once in the document text, in disjoint text; the provision that amends by deletion is shown to have
  deleted; **13 controls on the checks themselves held**, so no line of the report is a check that
  looked for nothing; and the 18 documents that must not have moved are identical in content.
  `amendment-approvals.md`.

**The oracles were shown capable of failing.** `VER-MOK-012` acceptance scenarios 4, 5 and 6 were
performed as written: the record path was perturbed to draw one value and oracle 4 failed; a counter's
increment was moved into the wrong branch and oracle 6 failed with the two predicted findings —
`regeneration_skipped` reading `{"depleted":17,"capacity":0}` against the replay's inverse; and a
thirteenth event type was added to the engine without adding it to the specification and oracle 5's
size assertion failed. Scenario 6's perturbation additionally stopped `cargo build --workspace` at
`mokiterions-tui/src/authority.rs:20`, because the observer matches exhaustively over `EventType` with
no wildcard arm — a second, independent check that nobody wrote for this purpose. **All three
perturbations were reverted**, and the gates above were run against the reverted tree.
`negative-controls.txt`.

What else is measured, beyond the seven oracles:

- **Twelve static and architecture checks, all PASS**, of which six are the six `VER-MOK-012`
  declares. The library target reaches three standard-library modules and no filesystem, path,
  environment or process module; `write_event_record` has exactly one call site outside tests; there
  is no `f32`, `f64`, float cast or decimal literal in 6,037 lines of library source; **61 field names
  over 4 record kinds carry no outcome, label, category, verdict, severity or interpretation field**;
  all seven hash- and tree-ordered collections are inside `#[cfg(test)]` regions; the four private
  counter fields have exactly one write each and every read in `write_run_record` alone; and the
  sink's path is parsed in `cli.rs`, opened in `main.rs`, and never reaches the library. Every scan
  runs over source with comments and literals blanked, so a doc comment naming a prohibited call is
  not mistaken for the call. `static-checks.txt`.
- **The public interface grew by one parameter and by no item.** 49 items, 43 public fields, 92 `pub`
  lines at the candidate; the pre-change enumeration is identical but for `execute`'s signature line.
  `interface.txt`, `RESULT: PASS`.
- **The test census reconciles name by name.** 212 → 246: **34 additions, 0 removals, 0 renames**, by
  qualified name, cross-checked by a second census that splits by the binary that ran each test and
  reports the same 34 and the same 0. The observer's ten targets are untouched at 127 tests.
  `analysis/census-reconciliation.txt`, `analysis/census-by-target.txt`.
- **Every prior retained capture reproduces.** **199 captures, 92 distinct configurations re-run, 0
  failing**, byte-compared against what `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011` retained, each
  classified by its own bytes into one of the three shapes the text stream has had. `additivity.txt`.
- **Six process-boundary captures.** Sink not creatable, write failure mid-run, flush failure,
  run-record write failure, reserved-spelling rejection, and the overwrite — each with its standard
  error, exit code and the destination's state afterwards. Where a write has to fail the fault is a
  byte-range lock held externally, so no engine code is compiled differently and no test double is
  substituted. `failure-captures.txt`.
- **Stream sizes over thirty combinations.** The declared 1,000-tick row: 1,270,326 text bytes,
  2,730,025 record bytes, 214%, 12,915 records. `sizes.txt`, `measure-sizes.sh`.
- **No retained record stream carries the path it was written to**, which `VER-MOK-012` names as the
  property making this evidence class safe to retain at all. Not taken on trust: each retained cell
  was run twice to two deliberately different destinations and the record bytes required identical,
  the digests then compared against a manifest taken at a **third** destination, and the complete set
  of distinct characters in all **6,444,508** retained bytes enumerated — **64 characters, and neither
  `/` nor `\` among them**. A path cannot be spelled without one of those two, so the enumeration is a
  statement about every destination rather than about the ones used here.
  `retained-sink-streams.txt`.

## What this record does not claim

1. **`VER-MOK-012` is answered in substance, and it is not satisfied in every literal respect.** What
   this record said as a candidate, before the owner's validation: *"All eight manual assessments are
   OUTSTANDING — assessments 1 and 2 the product owner's, 3 through 6 the technical owner's, 7 and 8
   the assurance owner's ... the decision line of all eight is blank, so no assessment carries a
   decision date, because none has been decided."* **All eight are now recorded**, so that clause is
   discharged and the paragraph is kept rather than overwritten because it is what the owner was
   deciding against. **At the commit this record binds, `manual-assessment.md` still reads
   `OUTSTANDING` for all eight**: the decisions are in a later commit, because a decision cannot be
   inside the commit it decides about, and `evidence_paths` names the blob at `50364a3` rather than the
   blob at the branch tip.

   **What is not discharged.** Two bullets of the contract's *Evidence retention* list remain
   unsatisfied as written — item 3 below measures both — and the validation did not reach them. So this
   record does **not** claim `VER-MOK-012` is satisfied without qualification; it claims the eight
   judgements are recorded, every automated case passes, and two retention obligations are met by
   substitution rather than as written, with the substitution's dependence on oracle 1 named.

   **What transitioning this record to `verified` would now record.** That the accountable assurance
   owner accepts the retained evidence for `WO-MOK-019` at `50364a3` **with those two retention
   deviations, the nine uncorrected defects and the unapproved `SPEC-MOK-004` rule 11 amendment
   standing** — and with assessment 8 discharged as a reading rather than as a measurement, which is
   the strongest form that assessment admits. It is a materially narrower acceptance than the candidate
   form of this record described, because the eight assessments are no longer part of what would be
   waived. It is stated here so it cannot be read as either wider or narrower than it is.
2. **Nine defects stand measured in approved artifacts, and none is corrected here.** Amending an
   approved artifact is the owner's act, so each is reported in `completion-summary.md` item 16 rather
   than fixed. **One of them weakens an oracle:** `SPEC-MOK-006` rule 3.2 says the direction domain
   carries "the eight fixed direction words" and it carries **four** cardinal ones — the eight belong
   to the private `RelativeDirection` perception enum, which reaches no record field — so the size
   assertion for that domain was transcribed from the engine rather than from the specification, and
   **for one domain of thirteen oracle 5 compares the engine against itself.** Measured mitigation: 0
   occurrences of any diagonal word in all four retained record streams and all three retained text
   streams, and rule 3.3 is unaffected because both vocabularies use only lowercase and underscore.
   The other eight are: rule 3.2's `"direction":"north_east"` example is unreachable; `SPEC-MOK-002`'s
   amendment row names four `execute` call sites including two in `mokiterions-tui`, where the real
   count is eight sites in two files and the observer never calls it; `WO-MOK-019` says "six
   cumulative counters" in three places, including the assurance rationale in its own front matter,
   where there are **seven** `u64` counters in four struct fields; `VER-MOK-012` assessment 4 miscites
   rule 7.8 and says "three facts" where its own *Residual uncertainty* names two; `ADR-MOK-005`
   counts `SPEC-MOK-001`'s list as nine where it carries eleven; `ADR-MOK-005` attributes `execute`'s
   signature to `SPEC-MOK-002` rule 5 where it is rule 4's, which is why 28 provisions were verified
   against 27 ADR bullets; `REQ-MOK-045`'s matrix row is unsatisfiable as written, because two prior
   work orders retained captures of older text-stream shapes; and `VER-MOK-012`'s retention bullet 4
   asks for roughly 120 MB. Each is an imprecision in text, none changes a behaviour, and **the
   corrections are the owner's to approve.**
3. **Two retention deviations stand, disclosed rather than left to be inferred from what is absent.**
   The captures are not retained whole: 110 MB of standard output per capture, three captures, plus
   thirty full sink streams, would put roughly a third of a gigabyte of generated text into the
   repository, most of it byte-identical copies of the same thing. Retained instead: a digest manifest
   of every cell of every capture, three whole pre-change text streams, and **four of the thirty
   declared record streams**. **No post-change *text* stream is retained whole** — bullet 3 asks for
   the sink capture's standard output and this packet holds its digests only. The substitution rests
   on oracle 1's result, which is named here rather than relied on silently: a reader who does not
   accept oracle 1 should not accept the substitution either. Whether that trade is right for this
   repository is the owner's call. `README.md`, `retained-sink-streams.txt`.
4. **One amendment was made beyond `ADR-MOK-005`'s approved list, and is recorded as unapproved rather
   than claimed.** `SPEC-MOK-004` rule 11's conservation clause was amended because the change adds
   test targets and the rule obliges the counts to move with them. `amendment-approvals.md` section 4
   names it with the rule that obliges it. It was **not** approved when this record was captured, and
   the capture does not treat it as though it were. **It is approved now** — the owner, as technical
   owner, approved that row in the commit immediately before this record's transition, so this is the
   one item on this list that the transition found already settled rather than accepted open. The
   approval was recorded by prepending to the row's status cell and keeping the cell's original text
   unedited, precisely so that `amendment-approvals.md`'s measurement of that cell still holds; re-running
   `analysis/amendments.py` with the approval present reproduces the retained file byte for byte. Three
   `OUTSTANDING` amendment rows carried forward from the earlier layer, including `ARCH-MOK-001`'s
   2026-08-18 row, are named rather than counted in section 5 and are **still not resolved** — none of
   them is this chain's to pay.
5. **This record binds a branch commit, not `master`'s tree.** `50364a3` is the tip of
   `feature/phase-4a-definition` and is not an ancestor of `master`. Every figure above describes that
   tree. **On a merged tree the gates, the census, the interface enumeration and oracles 1 through 6
   need re-running rather than carrying over**, and a record bound to the merge commit is a new
   record, not an edit of this one. **Verification is not merge and not release**, and being `verified`
   does not extend this record's binding one commit past `50364a3`. Pull request #31 was open as a
   **draft** while the eight assessments were outstanding, because that was the reason a reviewer could
   not yet act. Every condition this item named as remaining is now met — the assessments recorded, the
   `SPEC-MOK-004` row approved, the retention deviations accepted, the nine defects deferred to a
   correction work order, and this record transitioned — and **the owner authorized marking the pull
   request ready for review in the same turn as the transition**, so it is marked ready in an act after
   this commit. That is a request for review of the branch. It is not a merge, it is not a release, and
   it creates no record bound to `master`.
6. **No claim is made about a consumer.** The reconstructor and the replay consumer exist for
   verification, are retained as evidence, and are not maintained artifacts. No reader, parser or
   schema file is product, and nothing in the workspace reads the record stream — the observer's ten
   targets are untouched at 127 tests precisely because this change adds a stream it does not read.
7. **Every automated result bound here is a claim about bytes.** Whether the schema holds the facts
   Phase 4b will need, whether refusing to classify is right, and whether the closed alphabet is a
   constraint worth keeping are judgements, not measurements. They are assessments 1, 2 and 3, and they
   are now **decided** — but decided is not measured, and nothing in this packet turns them into
   evidence. Assessment 1 in particular was answered affirmatively **without naming the one candidate
   gap its own material raised**, build identity beyond the package version, so that gap is accepted on
   a judgement and not closed by a measurement.
8. **`VER-MOK-012`'s own *Residual uncertainty* is inherited unchanged**, seven items of it,
   including that oracle 3 checks the captures rather than the format, that non-perturbation is
   verified over the declared matrix and at every tick within it rather than everywhere, that the
   replay consumer is a second implementation of part of the engine, that `capacity` and `depleted`
   have the weakest independent witness in the contract, and that `fear`'s maximum is a well-formed
   figure about an attribute with no consumer. None of those is discharged by this record.

## The five conditions this record set for its own verification, and how each was met

This list was written while the record was `ready`, as the conditions it set for being verified. It is
kept in its own order and its own words, with the outcome of each stated rather than the item deleted:
a record that erased its own preconditions once they were met would leave a reader unable to see what
the decision turned on. **Nothing on it was ever waiting on code**, and all seven oracles pass at the
candidate commit.

1. **Discharged — the eight manual assessments, product owner, technical owner, assurance owner.** Two,
   four and two respectively. What this record said as a candidate: *"`manual-assessment.md` has the
   material each needs assembled, so each is a reading and a decision rather than a measurement.
   Assessments 7 and 8 are the assurance owner's own and are about the instruments oracles 2 and 6
   depend on; deciding on this record without performing them accepts those two oracles on the strength
   of their construction rather than on a judgement of it."* **All eight were recorded on 2026-08-20**,
   assessments 7 and 8 among them, so oracles 2 and 6 now rest on a judgement of their instruments and
   not only on their construction. The decisions, and what each accepts by being an affirmative answer
   to a prepared question, are in `manual-assessment.md`; the section at the top of this record states
   the three places where nothing was named in the alternative.
2. **Deferred, not corrected — the nine defects in approved artifacts, the owner of each artifact.**
   What this record asked: *"whether each is corrected, accepted as an imprecision, or recorded and
   left. The one that matters most to this contract is the first: until `SPEC-MOK-006` rule 3.2's
   direction domain is corrected, oracle 5's size assertion for that one domain has no independent
   witness."* **The owner's disposition on 2026-08-20 is a fourth answer the item did not offer: a
   correction work order in Phase 4b**, carrying all nine with its own verification contract and
   evidence. So none of the nine is corrected here, none is dismissed, and the one that matters most to
   this contract is accepted **with its consequence unresolved and its blast radius measured** — 0
   diagonal direction words in any of the seven retained streams. That work order does not exist yet.
   Commissioning it is a definition act, and this record neither performs it nor assumes it.
3. **Discharged — the amendment beyond `ADR-MOK-005`'s approved list, the owner as technical owner.**
   `SPEC-MOK-004` rule 11. What this record said: *"Approving it or reverting it are both available;
   leaving it is what stands now, disclosed."* **Approved on 2026-08-20**, in the commit immediately
   before this record's transition, with the reverting option declined on the ground that it would trade
   an unapproved row for a stale figure — rule 11's own text calls a work order that loses a test count
   a defect. Item 4 of *What this record does not claim* records how the approval was written so that
   oracle 7's measurement of the row still holds.
4. **Accepted as they stand — the two retention deviations, the owner.** What this record asked:
   *"whether digests plus four whole streams is the right trade for this repository, and whether bullet
   3's post-change text stream should be retained whole after all."* The answer is that the trade stands
   and bullet 3's text stream is **not** retained whole. Nothing was re-captured and nothing was
   deleted. `VER-MOK-012`'s *Evidence retention* list is therefore **still not satisfied as written on
   those two bullets**, and this record's acceptance of them is not a claim that it is.
5. **Outstanding — the merge, and a record bound to it, engineering owner then assurance owner.** Item 5
   of *What this record does not claim*. **This is the one item of the five that verification did not
   discharge and could not.** Pull request #31 is made ready for review after this commit, which asks
   for that review; it does not perform the merge, and a record bound to the merge commit is a new
   record rather than an edit of this one. This record does not create it, stand in for it, or extend
   its own binding past `50364a3`.

**`status` moved from `ready` to `verified`, and it is the only front-matter field that moved.**
`updated` already read `2026-08-20`, so even it did not change. `commit`, `git_object_format`,
`worktree_state`, `verified_at`, `artifact_snapshot_sha256`, all 55 `evidence_paths`, both relations and
the `title` — which reads *candidate*, because the record was captured as one — stay exactly as the
capture produced them. **The provenance is the capture's, not the decision's**, and a decision does not
re-measure it.

**What being `verified` does not make true.** It does not make `VER-MOK-012` satisfied in every literal
respect; two retention bullets stand unmet by their own terms. It does not correct a defect, resolve a
carried-forward amendment row, merge a branch or release anything. It records that the accountable
assurance owner read what is retained here, including everything this record declines to claim, and
accepted it for `WO-MOK-019` at `50364a3`. **That is the whole of it, and the whole of it is on this
page.**
