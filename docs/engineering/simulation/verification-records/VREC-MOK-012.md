+++
id = "VREC-MOK-012"
type = "verification_record"
title = "Verification candidate for WO-MOK-012"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-20"
commit = "50364a3719c68643f0b5354798b6d3084cff1c0e"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-20T12:08:52Z"
artifact_snapshot_sha256 = "16862ef3408e500d5e3488f9911eba100af3de3a4e584c4c55792131f837108d"
evidence_paths = [
  "docs/engineering/simulation/evidence/WO-MOK-012/README.md",
  "docs/engineering/simulation/evidence/WO-MOK-012/additivity.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/alphabet.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/amendment-approvals.md",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/amendments.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/capture-failures.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/census-by-target.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/census-by-target.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/census-reconciliation.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/compare.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/digest.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/entropy.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/prior-captures.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/reconstruct.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/replay.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/retain-sink.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/retain.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/static-checks.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/analysis/validate.py",
  "docs/engineering/simulation/evidence/WO-MOK-012/baseline/COMMIT.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/baseline/capture-state.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/baseline/full/seed42-baseline-d0.75-traceoff.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/baseline/full/seed42-individual-d0.75-traceoff.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/baseline/full/seed42-reference-d0.75-traceoff.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/baseline/pre-manifest.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/baseline/test-census.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/baseline/test-run.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/capture.sh",
  "docs/engineering/simulation/evidence/WO-MOK-012/completion-summary.md",
  "docs/engineering/simulation/evidence/WO-MOK-012/entropy-per-tick.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/entropy-states.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/entropy.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/failure-captures.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/gates.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/interface.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/json-validity.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/manual-assessment.md",
  "docs/engineering/simulation/evidence/WO-MOK-012/measure-sizes.sh",
  "docs/engineering/simulation/evidence/WO-MOK-012/negative-controls.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/oracle1/post-nosink-vs-post-sink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/oracle1/pre-vs-post-nosink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/oracle1/pre-vs-post-sink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/oracle2/reconstruction-result.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/oracle6/reconciliation.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/post-nosink-manifest.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/post-sink-manifest.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/post/full/seed42-baseline-d0.75-traceoff.jsonl",
  "docs/engineering/simulation/evidence/WO-MOK-012/post/full/seed42-baseline-d0.75-traceon.jsonl",
  "docs/engineering/simulation/evidence/WO-MOK-012/post/full/seed42-individual-d0.75-traceoff.jsonl",
  "docs/engineering/simulation/evidence/WO-MOK-012/post/full/seed42-reference-d0.75-traceoff.jsonl",
  "docs/engineering/simulation/evidence/WO-MOK-012/post/test-census.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/post/test-run.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/retained-sink-streams.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/sizes.txt",
  "docs/engineering/simulation/evidence/WO-MOK-012/static-checks.txt",
]

[relations]
verifies_work_order = ["WO-MOK-012"]
conforms_to = ["VER-MOK-012"]
+++

# Verification Record Candidate

This `ready` record binds the retained evidence for `WO-MOK-012` — the optional structured record
stream — to candidate commit `50364a3719c68643f0b5354798b6d3084cff1c0e` on
`feature/phase-4a-definition`. **An accountable assurance owner must review the evidence and decide
whether to transition it to `verified`.** `DECISION_RIGHTS.md` reserves that transition to that role
and states that record preparation never makes it: *"Automation may prepare `ready` verification and
release records from bounded Git observations. Only accountable assurance and release owners may
transition those records to `verified` or `released`."* **Preparing this record approved, verified,
merged, tagged, released and published nothing.** The instruction this file was prepared under — *"you
can prepapre the verification record, keep it ready, and commit and push"* — supplied the authority to
commit and push the file and stated no judgement on the evidence. **A later instruction, on the same
day, did state judgements**: the section immediately below records it, what it settled, and what it
left standing. It did not transition this record, and this record is still `ready`.

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
- **This record is still `ready`.** The owner's instruction named the contract and the assessments; it
  did not name this record, and `DECISION_RIGHTS.md` reserves the `ready` → `verified` transition to
  the accountable assurance owner as a separate act. Nothing here is approved by implication.
- **Nothing else moved.** The nine defects are uncorrected, the `SPEC-MOK-004` rule 11 amendment made
  beyond `ADR-MOK-005`'s approved list is still unapproved, the three carried-forward `OUTSTANDING`
  amendment rows still stand, `WO-MOK-012` is still `in_progress`, and pull request #31 is still a
  draft.

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

## Provenance, and what a `ready` record does

The record is written after the candidate commit it names, so its own commit metadata is not
self-referential. **`verified_at` is the capture timestamp, not a verification decision**: the gate
and harness figures below were taken at `2026-08-20T12:08:52Z` from a worktree
`git status --porcelain` reported as empty at this commit, and `artifact_snapshot_sha256` is the
digest `python scripts/generate_harness_dashboard.py --root .` printed there, over the **113**-artifact,
**371**-relation graph as it stood **before this file existed**. Both are left exactly as captured. A
`ready` record does not re-measure its own provenance, and the ordering matters: the capture cannot be
repeated once the file is in the tree, so it was taken first.

What a `ready` record *does* is checkable rather than rhetorical, and it was measured with this file
present. With it in the tree the validator reports **114 artifacts and 373 relations, still 0 errors
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
**The record raised the signal and changed no error or warning count.** Answering that signal is the
assurance owner's act, not this file's.

**`evidence_paths` names blobs at `50364a3`, not at the branch tip.** Three of the 55 have changed
since, all of them in the commit that recorded the owner's validation: `manual-assessment.md`, which
carried eight blank decision lines at this commit and now carries eight decisions; `completion-summary.md`
and the packet `README.md`, which each gained a marked later-fact note pointing at those decisions and,
in the summary's case, one corrected internal count. **No measured figure in any of the three was
edited**, which is the rule this repository states for evidence: measure again rather than edit into
agreement. Every other retained artifact, and every digest, byte count and oracle result in the packet,
is untouched. A reader checking this record against the tip should expect exactly those three
differences and no others.

**Read *What this record does not claim* before relying on this record.** That list was written against
the candidate commit and is left as written except where the validation reached it; its item 1 records
what the validation settled and what it did not.

## What this record claims

`WO-MOK-012` is `in_progress` and `VER-MOK-012` is `approved`. The work order was **not** transitioned
alongside this record; `WORKFLOW.md` is explicit that its status never substitutes for this record's,
and no lifecycle move was instructed.

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
| `python scripts/inspect_engineering_artifacts.py` | 0 errors; **`Decision required (0): none`**, `Assurance pending (0): none`, `WO-MOK-012 [in_progress]`; 19 findings — 2 `W-HEX-001`, 5 `W-HEX-003`, 12 `I-REV-001`, none of them new |

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
   owner accepts the retained evidence for `WO-MOK-012` at `50364a3` **with those two retention
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
   count is eight sites in two files and the observer never calls it; `WO-MOK-012` says "six
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
   names it with the rule that obliges it. **It is not approved**, and this record does not treat it as
   though it were. Three `OUTSTANDING` amendment rows carried forward from the earlier layer,
   including `ARCH-MOK-001`'s 2026-08-18 row, are named rather than counted in section 5 and are not
   resolved here.
5. **This record binds a branch commit, not `master`'s tree.** `50364a3` is the tip of
   `feature/phase-4a-definition` and is not an ancestor of `master`. Every figure above describes that
   tree. **On a merged tree the gates, the census, the interface enumeration and oracles 1 through 6
   need re-running rather than carrying over**, and a record bound to the merge commit is a new
   record, not an edit of this one. **Verification is not merge and not release.** Pull request #31 is
   open as a **draft** and was deliberately left that way. When it was prepared, the eight outstanding
   assessments were the reason a reviewer could not yet act; they are recorded now, so what remains
   before it is marked ready is this record's transition and the owner's decisions on the nine defects,
   the two retention deviations and the `SPEC-MOK-004` rule 11 amendment. **Marking it ready was not
   instructed and was not done.**
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

## What must happen before this record can be verified

Each item names the role that owes it. **Nothing on this list is waiting on code**, and all seven
oracles pass at the candidate commit.

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
2. **The nine defects in approved artifacts — the owner of each artifact.** Whether each is corrected,
   accepted as an imprecision, or recorded and left. The one that matters most to this contract is the
   first: until `SPEC-MOK-006` rule 3.2's direction domain is corrected, oracle 5's size assertion for
   that one domain has no independent witness.
3. **The amendment beyond `ADR-MOK-005`'s approved list — the owner, as architecture decision owner.**
   `SPEC-MOK-004` rule 11. Approving it or reverting it are both available; leaving it is what stands
   now, disclosed.
4. **The two retention deviations — the owner.** Whether digests plus four whole streams is the right
   trade for this repository, and whether bullet 3's post-change text stream should be retained whole
   after all.
5. **The merge, and a record bound to it — engineering owner then assurance owner.** Item 5 of *What
   this record does not claim*. This record does not create that record, stand in for it, or extend
   its own binding past `50364a3`.

**`status` stays at `ready`.** The validation of 2026-08-20 changed **no front-matter field of this
record** — not `status`, and not `updated`, which already read `2026-08-20`. It was an act on the
contract and on the eight assessments, and the body above records it; transitioning this record is a
different act, reserved by `DECISION_RIGHTS.md` to the accountable assurance owner, and it has not been
taken.

When it is taken, `status` and `updated` are the only front-matter fields that change: `commit`,
`git_object_format`, `worktree_state`, `verified_at`, `artifact_snapshot_sha256`, all 55
`evidence_paths`, both relations and the `title` — which reads *candidate*, because the record was
captured as one — stay exactly as the capture produced them. **The provenance is the capture's, not
the decision's**, and a decision does not re-measure it.
