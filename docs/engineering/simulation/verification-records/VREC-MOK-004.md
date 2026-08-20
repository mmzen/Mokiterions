+++
id = "VREC-MOK-004"
type = "verification_record"
title = "Verification candidate for WO-MOK-004"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-17"
updated = "2026-08-17"
commit = "95f0aa2079d4abced1c01f4c09b5c66dc5ab29fe"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-17T19:09:46Z"
artifact_snapshot_sha256 = "8d32b5c306464c3860dc05e5d31641a09e06361d3dff2392a900f54078ed2bd1"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-004/README.md", "docs/engineering/simulation/evidence/WO-MOK-004/after/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-004/after/full/short_seed42_baseline_trace_on.txt", "docs/engineering/simulation/evidence/WO-MOK-004/after/full/short_seed42_reference_trace_on.txt", "docs/engineering/simulation/evidence/WO-MOK-004/after/manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-004/after/summaries.txt", "docs/engineering/simulation/evidence/WO-MOK-004/after/test-census.txt", "docs/engineering/simulation/evidence/WO-MOK-004/after/test-run.txt", "docs/engineering/simulation/evidence/WO-MOK-004/after/usage.txt", "docs/engineering/simulation/evidence/WO-MOK-004/amendment-approval.md", "docs/engineering/simulation/evidence/WO-MOK-004/baseline-comparison.md", "docs/engineering/simulation/evidence/WO-MOK-004/baseline/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-004/baseline/full/short_seed42_baseline_trace_on.txt", "docs/engineering/simulation/evidence/WO-MOK-004/baseline/full/short_seed42_reference_trace_on.txt", "docs/engineering/simulation/evidence/WO-MOK-004/baseline/manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-004/baseline/summaries.txt", "docs/engineering/simulation/evidence/WO-MOK-004/baseline/test-census.txt", "docs/engineering/simulation/evidence/WO-MOK-004/baseline/usage.txt", "docs/engineering/simulation/evidence/WO-MOK-004/capture.sh", "docs/engineering/simulation/evidence/WO-MOK-004/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-004/drift-demonstrations.md", "docs/engineering/simulation/evidence/WO-MOK-004/drift-scratch-a.txt", "docs/engineering/simulation/evidence/WO-MOK-004/drift-scratch-b.txt", "docs/engineering/simulation/evidence/WO-MOK-004/drift-scratch-c.txt", "docs/engineering/simulation/evidence/WO-MOK-004/exit-codes.diff.txt", "docs/engineering/simulation/evidence/WO-MOK-004/new-tests.md", "docs/engineering/simulation/evidence/WO-MOK-004/public-surface-inventory.md", "docs/engineering/simulation/evidence/WO-MOK-004/resilience-10k.txt", "docs/engineering/simulation/evidence/WO-MOK-004/resilience-and-viability.md", "docs/engineering/simulation/evidence/WO-MOK-004/specification-to-help.md", "docs/engineering/simulation/evidence/WO-MOK-004/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-004/test-census.md", "docs/engineering/simulation/evidence/WO-MOK-004/usage-text.diff.txt", "docs/engineering/simulation/evidence/WO-MOK-004/usage-text.md"]

[relations]
verifies_work_order = ["WO-MOK-004"]
conforms_to = ["VER-MOK-004"]
+++

# Verified Verification Record

Following review of the retained evidence by the accountable assurance owner, this record was transitioned from `ready` to `verified` on 2026-08-17. It binds `WO-MOK-004` to candidate commit `95f0aa2079d4abced1c01f4c09b5c66dc5ab29fe` without changing its captured provenance.

The verification transition is a later governance decision. It does not alter the candidate commit, release, tag, publish, or deploy anything.

The record was prepared after the candidate commit it names, avoiding self-referential commit metadata.

## What this record claims

`WO-MOK-004` is `implemented` and `VER-MOK-004` is `approved`. At candidate commit
`95f0aa2079d4abced1c01f4c09b5c66dc5ab29fe` every row of `VER-MOK-004`'s twenty-row
requirement-to-evidence matrix, all seven acceptance scenarios, all nine properties and invariants, every
static and architecture check, every security and privacy check, every performance and resilience check,
and all five manual assessments were executed. None was waived, narrowed, or deferred.
`specification-to-help.md` and `new-tests.md` map each row to a named test or a retained file, and
`README.md` indexes the 34 files enumerated in `evidence_paths` above.

| Gate | Result |
|---|---|
| `cargo test` | 60 passed, 0 failed, 0 ignored, across seven test executables plus a doc-test pass reporting zero tests; one invocation, no extra flag or environment variable |
| `cargo fmt --all -- --check` | no differences |
| `cargo clippy --all-targets --all-features -- -D warnings` | 0 findings, re-run after `cargo clean -p Mokiterions`; invocation not narrowed and no `allow` attribute added |
| `cargo build` | clean |
| `cargo doc --no-deps --lib` | 13 public items, identical item for item to the inventory retained under `WO-MOK-003` |
| Dependencies | dependency and dev-dependency tables empty; no build script; `Cargo.toml` and `Cargo.lock` unchanged |
| Artifact validation | PASS — 0 errors, 0 warnings, structure, governance, policy, and maintenance all E0/W0, across the 40-artifact graph this record's snapshot binds; PASS at 41 artifacts once this record is present |
| `preflight --phase review` | PASS at `implemented`, no finding, commit-bound verification `required` |
| `doctor` | PASS — every managed file matches distribution; no managed file was edited |

Toolchain: rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable, edition 2024.

### The pre-change facts, reconciled against what was fixed in advance

`VER-MOK-004` recorded the pre-change figures in the contract itself "so that they cannot be adjusted
afterwards". Each was re-read for this record from the committed evidence blobs rather than from prose
about them, and each reconciles.

| Fixed in advance | Baseline | After | Result |
|---|---:|---:|---|
| Usage text lines / non-blank | 12 / 10 | **25 / 21** | as fixed |
| Declared defaults stated | 1 of 5 | **5 of 5** | as required |
| Capture matrix cells | 43 | 43 | as fixed |
| Named exit-code cases | 16 | 16 | as fixed |
| Tests / ignored | 52 / 0 | **60 / 0** | as fixed, plus the eight this work adds |

### The three independent oracles

`VER-MOK-004` requires that no check be a tautology over `cli::USAGE`, because
`tests/process.rs::help_exits_successfully` asserts the help output *equals* that constant and would
hold for every possible value of it including an empty string. All three oracles lie outside the
constant, and all three could have failed.

1. **The specification, transcribed without reading the code.** `SPEC-MOK-001`'s *Inputs* and *Help
   output* sections were transcribed into a table and diffed against the rendered help:
   **no shortfall, no surplus, no defect** across all six options, all five defaults, and all three value
   constraints. Because the table is prepared from the specification, it cannot be satisfied by making
   the text agree with the code.
2. **The program's applied configuration.** For each of the four value-taking options, the stated default
   is scraped out of the printed text, fed back through `cli::parse`, and required to equal the
   configuration an empty argument list produces. The test declares no expected value, so it fails if
   either side moves alone. This is the check that makes a hardcoded default safe, and it is executed by
   a test rather than established by review, because review cannot be re-run on a future change.
3. **The recorded pre-change baseline.** Captured at `08dc8d47aa569c77fb7c9b091e328eb8f40c5285` before
   the first edit, with `git status --short -- src tests Cargo.toml Cargo.lock` reporting zero modified
   files, and never regenerated. All **43 matrix cells are byte-identical** in digest, line count, exit
   code, and full summary text. All **14 invalid-configuration cases** keep exit `2`, empty standard
   output, and a byte-identical `configuration error:` line. The two retained traced runs are identical
   to their baseline counterparts as the same git blob object, `470bdd8e` and `1393ca97`. The only
   changed field in the entire capture is the `--help` case's standard output.

That last claim was established mechanically rather than by reading: removing the standard-error body
lines from the 342-line diff of the 16 named cases leaves exactly four lines, and all four are the help
case's standard-output digest and length.

### The demonstration that the equality check is load-bearing

Acceptance scenario 3 requires the check be shown failing in both directions. Three scratch divergences
were performed and none committed; `src/cli.rs` was verified back to digest `5498a62e…` after each.

| # | Divergence | Result |
|---:|---|---|
| A | applied `--ticks` default 100 → 1000, text untouched | `4 failed; 8 passed` — the new test names the disagreement, and three pre-existing tests also fail |
| B | printed `--seed` default 0 → 1, code untouched | **`1 failed; 11 passed` — only the new test.** All five pre-existing `tests/cli.rs` tests passed, including `defaults_are_stable` |
| C | `"--verbose"` arm added to `parse`, no help entry | `1 failed; 11 passed` — the coverage-totality test |

**B is the load-bearing result.** Before this work the program could have advertised a default it did not
apply with the whole suite green. C goes beyond the scenario and discharges `VER-MOK-004`'s recorded
residual uncertainty about the coverage property: that property is written over the parser's own `match`
arms, read out of `src/cli.rs` through `include_str!`, not over a hand-written list, which the contract
warns "is easy to write and looks identical when it passes".

### Performance and resilience

Survivor counts at the declared density of `0.75%` are **8, 11, 8, 9, 11** on seeds `0, 1, 42, 123, 777`
after 1,000 ticks — identical to the baseline and to what `VREC-MOK-002` and `VREC-MOK-003` recorded,
rather than merely above the floor of eight. The whole summary line matches in each case, including
per-territory population and per-class resource counts. Two seeds sit on the floor with zero margin and
both still do, at the same tick, with the same distribution. Four 10,000-tick runs at seed `123` are
byte-identical to `WO-MOK-003`'s record, with survivors plus deaths equal to twelve in every run.

## What the accountable assurance owner weighed before verifying

None of the eight matters below is a failure against this contract. All were disclosed before the
decision and all are accepted by the verification recorded above, which endorses the judgement that they
are acceptable at this candidate commit. They are retained in full rather than summarized away, because a
disclosure that survives only until it is accepted is not a disclosure.

1. **Two statements in the evidence were wrong about the repository, and the candidate commit is the
   correction.** `README.md`'s *What is not here* section claimed the two post-change traced runs had not
   been retained; `after/full/` holds both, and the Contents section listed them correctly, so the file
   contradicted itself, and the wrong half is the one a reviewer checking for gaps would read.
   Separately, `completion-summary.md`'s candidate-commit row said "one commit on that branch", a count
   that the next commit falsifies. The implementation commit `c565cd0` carries both. Both are corrected
   in `95f0aa20`, the commit this record binds, so that the commit named here carries an evidence
   directory that describes itself accurately; the correction is visible in the history rather than
   erased from it by amending a pushed commit under review. The substance the first bullet misreported is
   stronger than the claim it made: the paths resolve to the same git blob objects. What the owner is
   accepting is that both defects were found by checking the evidence against the repository rather than
   by reading it — which is the only way this class of error surfaces — and that neither touched a test,
   a source file, or a governance artifact.
2. **The recorded `stdout_sha` and `stdout_len` are one byte short of the emitted output, by
   construction.** `capture.sh` captures with `stdout=$(…)`, and command substitution strips trailing
   newlines, so both figures are computed over the output minus its single final newline. The emitted
   help text is **700 → 1,326 bytes**; the manifest records `699 → 1,325`. The same holds for the
   `lines=` counts, which are one less than the line count. This is systematic, was applied by the same
   unmodified script to both sides and to all 16 cases, and therefore does not weaken the equivalence
   comparison at all — but it means a reader who recomputes a digest from `usage.txt` will not reproduce
   `stdout_sha`, and that two separate offsets now stand between a retained file and a recorded digest:
   this one, and the line-ending conversion described in `README.md`. The delta is +626 bytes under
   either convention. `completion-summary.md`'s byte row states the emitted figures.
3. **"5 of 5 declared defaults" counts one default stated in prose rather than with the `Default:`
   marker.** Four options carry `Default: …`; `--trace-actions` states its behavior on omission as
   *"Off unless given."* The literal count of `Default: ` in the text is therefore **4**, and
   `each_declared_default_is_stated_once` asserts exactly that, with a companion test requiring
   `--trace-actions` to state no default *value*. This is the shape `SPEC-MOK-001` specifies for a flag,
   not a shortfall, but the phrase "5 of 5" invites the reading that five `Default:` strings are present.
   The baseline count under the same measure was **0**, its single documented default being the prose
   sentence *"It defaults to 0.75."* — so the before-and-after counts of 1 and 5 both include a
   prose-stated default and are measured consistently.
4. **The coverage-totality test is coupled to the layout of `src/cli.rs`.** It reads that file through
   `include_str!` and recognizes a match arm by its punctuation. Reformatting the arms breaks it loudly
   rather than silently, which is the right direction of failure, but it is a coupling between a test and
   the formatting of a source file, and it is the price of the strong form the contract asked for.
5. **One test name differs from the work order's authorized list.**
   `every_documented_option_is_accepted_by_the_parser` was written as
   `the_documented_options_are_exactly_the_options_the_parser_accepts`, because it asserts both
   directions and the authorized name understates the second. Test names are inside the work order's
   decision envelope; the deviation is recorded rather than treated as unremarkable.
6. **The tautological assertion remains in the suite.** `help_exits_successfully` was not changed or
   removed. It pins routing and the exit code, which is what it was written for, and `VER-MOK-004` treats
   it that way. The new `tests/process.rs` test supplies what it cannot: it pins the whole constant to
   the diagnostic path, which is the path 14 of the 16 recorded cases take, and which the two existing
   standard-error assertions did not cover because they check only the substring `Usage:` — satisfied by
   the synopsis alone.
7. **No architecture is engaged, deliberately.** `ARCH-MOK-001` does not address `REQ-MOK-018`.
   `WO-MOK-004` records the omission together with the counter-argument against it, since
   `SPEC-MOK-002` rule 2's remark that "`REQ-MOK-010`'s observable interface includes that text" can be
   read the other way. A reviewer who disagrees has the reasoning in front of them, and the remedy would
   be adding architecture coverage rather than changing the work.
8. **The printed `--ticks` default of `100` is now visible beside a viability floor stated at 1,000
   ticks.** `VER-MOK-004`'s fifth manual assessment requires a judgement on whether that reads as
   misleading. The assessment recorded here is that it does, to an operator who reads the help and then
   reads the requirements — and that making it visible is the intended effect of this work rather than a
   defect introduced by it. The value is deliberately unchanged; whether `100` is the right default is a
   product decision recorded in `REQ-MOK-018`'s *Open decisions* on 2026-08-17 and outside this contract.
   Nothing in this record should be read as accepting `100` as correct.

The two explanatory paragraphs below the options block were assessed against manual assessment 2 and
judged to earn their place: they say why the reference source exists and what `--density` binds together,
neither of which an options entry carries. The density paragraph was shortened by exactly the two facts
that moved into its options entry, so no paragraph now repeats an entry.

## Limits of coverage stated in this record

- **Content and the printed-to-applied equality are verified; alignment, column width, and wrapping are
  not.** `SPEC-MOK-001` explicitly leaves those unspecified, so no test freezes them. Widths are
  measured instead: the widest line the options block introduces is **80 columns**, and the widest line
  in the text is **81** — the first line of the synopsis, unchanged by this work.
- **Terminal rendering is not verified.** Behavior in a narrow window, under wrapping, or through a
  pager is untested. A later complaint about it is an amendment, not a defect in this verification.
- **Readability is assessed by review, not measured.** No mechanical check establishes that a description
  helps an operator. The standard applied is the one the contract names: an entry that merely restates
  its own option's name is an adverse observation, and none does.
- **Equivalence is demonstrated across the declared matrix, not the input space.** 5 seeds × 2 decision
  sources × 2 declared densities × 2 trace settings at 1,000 ticks, plus two 20-tick traced runs, the
  no-argument invocation, 16 named cases, and four 10,000-tick runs. Undeclared seeds, undeclared
  densities, and the release profile are not covered. A help-text change has no mechanism for
  seed-dependent divergence, so this is the shape of the claim rather than a shortfall in executing it.
- **The wording is not claimed to be the best possible**, only present, accurate against the
  specification, stated once, in agreement with the applied defaults, and readable. Improving it is an
  amendment to `SPEC-MOK-001`'s *Help output* section.
- **The equality is verified for the defaults that exist today.** An option added later with a default
  and no entry is caught by the coverage-totality property, which is why that property was written in
  the strong form and demonstrated failing; but the property protects the *existence* of an entry, and a
  future default stated wrongly is caught only because divergence B shows the equality test would name
  it.
- **The survivor floor still has no margin**, with two declared seeds sitting exactly on eight, and
  long-horizon extinction under the reference source is unchanged at tick 9,154 at the default density.
  Both are pre-existing properties of the world model that `VREC-MOK-002` records and the product owner
  accepted on 2026-08-17. Reproducing both exactly is the correct outcome for a change that must not
  perturb the simulation.

A first attempt at the 10,000-tick runs used seed `42` and produced four rows that did not match
`WO-MOK-003`'s record. `WO-MOK-003` fixes the seed at `123` in its prose but its raw transcript records
only policy and density. Re-run at `123`, all four rows are byte-identical. The cause was the invocation
and not the program. It is recorded in `resilience-and-viability.md` rather than discarded, because for
several minutes it looked like an unexplained divergence in a change required not to perturb the
simulation, which is a stop-and-escalate condition under `WO-MOK-004`.

The `SPEC-MOK-001` *Help output* amendment was approved by the repository owner on 2026-08-17 **before
the usage text changed**, which `VER-MOK-004` requires independently of code state. `amendment-approval.md`
records the ordering step by step, with a `preflight --phase start` run against the drafts reporting
`W016`, *specification coverage missing `REQ-MOK-018`*, as independent evidence that the specification
did not yet cover the requirement: the tool said so before the amendment existed and said nothing after
it did. `SPEC-MOK-002` was not amended; its rule 11 and `VER-MOK-003`'s *Behavior surface unchanged*
invariant, which together forbid any byte of `USAGE` changing, are scoped to `WO-MOK-003`'s
restructuring, a discharged contract that binds that work order and not this one. Rules 5 through 8,
which do bind here, hold unchanged.

## Authority

This record is `verified`. The verification decision was taken by the repository owner acting as
accountable assurance owner on 2026-08-17, after review of the evidence listed above and of the eight
disclosures; it was not taken by an implementation agent, and `DECISION_RIGHTS.md` places it outside what
such an agent may inherit. Recording the decision changed the record's status and this prose only.

Every captured provenance field is preserved exactly as the managed capture produced it: the candidate
commit, the git object format, the clean worktree state, the capture timestamp, the artifact snapshot
hash, the 34 evidence paths, the verified work order, and the verification contract. The snapshot was
taken before this record was written, so it records the 40-artifact graph the record binds rather than
the graph containing the record. No provenance field was recomputed for this transition, and the candidate
commit is unchanged.

`WO-MOK-004` remains `implemented`. Verification is carried by this record rather than by a change to
the work order, matching how `VREC-MOK-001`, `VREC-MOK-002`, and `VREC-MOK-003` were verified.

One derived observation survives into this record rather than being left to be rediscovered. The Harness
Explorer raises `W-HEX-001` against `WO-MOK-004`, reporting no evidence document keyed to its ID. That is
a detection mismatch and not a shortfall: `discover_evidence` in `scripts/generate_harness_dashboard.py`
keys evidence on the *file name*, while this work order's evidence is retained in a *directory* named for
the work order, so no file name matches the pattern. `../../../ROADMAP.md` already records the
mismatch and the need to align the convention in one direction. It applies identically to `WO-MOK-001`,
`WO-MOK-002`, and `WO-MOK-003`, and because the rule's status set includes `verified` and `released`, the
observation will not clear on verification and would not clear on release.

Verification is not release. Nothing here releases, tags, publishes, or deploys, and no release record
exists. Release remains a separate record and a separate accountable decision.
