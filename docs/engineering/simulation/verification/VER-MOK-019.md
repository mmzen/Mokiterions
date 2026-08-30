+++
id = "VER-MOK-019"
type = "verification"
title = "A sweep's rows are bound to streams that no longer exist, its counts are independently reproduced, and its classification is revisable without touching a retained row"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-30"
updated = "2026-08-30"

[relations]
verifies = [
  "REQ-MOK-078",
  "REQ-MOK-079",
  "REQ-MOK-080",
  "REQ-MOK-081",
  "REQ-MOK-082",
  "REQ-MOK-083",
]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-30T16:11:24Z"
decided_by = "assurance owner"
reason = "Approved by the repository owner on 2026-08-30, by selecting the presented option, as part of the twelve-artifact Phase 4b chain. The chain converts docs/ROADMAP.md's Phase 4b open question into an approved shape on the strength of the measurement the roadmap reserved that decision to, taken at c90edc9 and recorded in ADR-MOK-008. It carries three disclosed and unrepaired findings: the threat mechanism is inert in 1447 of 1448 firings, the famine predicate is unreached in the swept space with food still standing at extinction, and no retreat event kind exists."
+++

# Verification Contract: Batch sweep, bound rows, and revisable classification

## Independence

Five things keep this contract independent of how the two instruments are written.

1. **Every derived count is checked against a count this contract's own tests compute**, by straight-line code over the
   stream, never by calling the instrument's counting path. The instrument confirming its own arithmetic verifies
   nothing. The oracle is an independently written count and the comparison is between two paths over the same bytes.
2. **The instrument's own cross-checks are never the evidence that the cross-checks work.** `SPEC-MOK-008` rule 10
   makes seven equalities a refusal condition, and all seven passed in all 35 streams measured on 2026-08-30 — which
   means a batch over real runs cannot distinguish a working check from a check that always returns true. Every rule 10
   equality is therefore also exercised against a **fixture stream carrying a deliberate disagreement**, where the
   required outcome is a refused cell. A check with no failing case is not a verified check.
3. **At least one case runs the real engine end to end.** `SPEC-MOK-008` rule 20.2 keeps the instruments' own tests on
   fixtures, which is correct for speed and wrong as the only evidence: a fixture corpus can encode a stream shape the
   engine does not emit, and a suite over it would pass while the instrument failed on every real run. B1 closes that,
   and it is the reason this contract is not satisfiable by the instruments' unit tests alone.
4. **Revisability is checked by digest, not by inspection.** `REQ-MOK-081`'s central claim is that editing a threshold
   changes no retained row. That is checked by hashing the retained output, editing a threshold, re-running the
   classifier, and re-hashing — Q6. Reading the code and observing that it does not write rows is not evidence.
5. **The retained rows are their own oracle for the discard.** `REQ-MOK-079` binds a row to a stream that is then
   destroyed, so the binding is checked by re-executing the cell and reproducing the digest — B7 — rather than by
   trusting that the digest was computed over what it claims.

This contract is authored before the instruments exist. It names their file paths, because `SPEC-MOK-008` rule 1 fixes
those as a decision rather than an implementation choice, and names no function, class or internal structure of either.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-078` | automated test | **B1** real-engine conformance | a batch over a real sweep of at least 8 cells, executing the built binary, produces a row for every cell; every field `SPEC-MOK-008` rule 8.1 names is present and of the stated type |
| `REQ-MOK-078` | automated test | **B2** transparency to the engine | a cell executed by the batch and the same cell executed by hand with rule 4.4's arguments produce byte-identical streams and byte-identical standard output |
| `REQ-MOK-078` | automated test | **B3** cell enumeration and order | the default sweep enumerates exactly 400 cells; the retained rows appear in rule 4.3's order — source as given, then density as given, then seed ascending — and do so identically under `--jobs 1` and `--jobs 4` |
| `REQ-MOK-078` | automated test | **B4** the declared default | with no axis option given, the `sweep` record states rule 3.1's four sources, five densities, twenty seeds and 1,000 ticks, and `defaulted_axes` names all four; with `--seeds 0-2` given, only `seeds` is absent from `defaulted_axes` |
| `REQ-MOK-078` | automated test | **B5** the `llm` refusal | a sweep naming `llm` exits `2`, executes no child process, writes no output file, and its message names `REQ-MOK-072` |
| `REQ-MOK-078` | automated test | **B6** usage errors | each of rule 15.1's nine conditions exits `2`, writes nothing, and spawns no child; a duplicate axis value is refused rather than collapsed |
| `REQ-MOK-079` | automated test | **B7** the digest reproduces | for every row of B1, re-executing the row's coordinates reproduces `stream_sha256` exactly; and does so from a debug build and a release build of the same commit |
| `REQ-MOK-079` | automated test | **B8** the stream is gone | after a batch, no stream file remains at the reusable path or anywhere the driver wrote one, except those `--keep-stream` named; peak stream bytes on disk during a 20-cell batch never exceed one stream |
| `REQ-MOK-079` | automated test | **B9** hashed as written | the digest equals a digest computed by this contract's own code over the file's raw bytes before any decode; a stream whose bytes are altered by one byte produces a different digest |
| `REQ-MOK-079` | automated test | **B10** `--keep-stream` changes no row | a batch with `--keep-stream` set for one cell produces a retained output byte-identical to the same batch without it |
| `REQ-MOK-079` | automated test | **B11** no ambient value | no `cell` record contains a filesystem path, hostname, process identifier, wall-clock time, duration or environment value, checked by pattern over every retained byte |
| `REQ-MOK-080` | automated test | **B12** independent counts | for every row of B1, `attempted` for each of the three kinds equals a count this contract computes independently over the retained stream of that cell |
| `REQ-MOK-080` | automated test | **B13** the three effectiveness tests | `effective` equals an independent count applying rule 9.2's three predicates; a fixture with `damage: 0`, one with `increase: 0` and one with `transferred: 0` each contribute to `attempted` and not to `effective` |
| `REQ-MOK-080` | automated test | **B14** lethality is not a narrowing | `lethal_attacks` equals an independent count of `target_died == "yes"`; a fixture whose only attack is lethal reports `attempted` 1, `effective` 1 and `lethal_attacks` 1 |
| `REQ-MOK-080` | automated test | **B15** zero is stated | a cell with no conflict of a given kind states `0` for that kind in both maps rather than omitting the key |
| `REQ-MOK-080` | automated test | **B16** the seven cross-checks pass | for every row of B1, `crosschecks_passed` is `7`, and each of rule 10.1's seven equalities is independently recomputed and holds |
| `REQ-MOK-080` | automated test | **B17** the seven cross-checks bite | seven fixture streams, each breaking exactly one of rule 10.1's equalities by one, each produce **no row**, a `missing` entry at stage `crosscheck`, and a reason naming both disagreeing figures |
| `REQ-MOK-080` | automated test | **B18** tracing independence | every count in every row is identical with `--trace-actions` present in the fixture stream and absent |
| `REQ-MOK-081` | automated test | **Q1** exactly one class | every row of B1 is assigned exactly one class from rule 16.5's five, and the assigning clause is stated |
| `REQ-MOK-081` | automated test | **Q2** each predicate fires | five fixtures, one per class, each assigned the intended class; and a fixture satisfying both clause 1 and clause 4 is assigned `extinction`, demonstrating the order carries meaning |
| `REQ-MOK-081` | automated test | **Q3** the residual | a fixture matching none of clauses 1 to 4 is `coexistence`; with clause 5 removed from the definitions it is reported as unclassified with its facts named, and is neither defaulted nor dropped |
| `REQ-MOK-081` | automated test | **Q4** a missing fact | a row lacking a field a predicate needs is reported unclassifiable with a stated reason, and is not classified on the fields present |
| `REQ-MOK-081` | automated test | **Q5** no judgement in a row | no retained byte of any `cell` record contains any class name, label, verdict, severity, threshold or predicate, checked by pattern over rule 16.5's whole vocabulary |
| `REQ-MOK-081` | automated test | **Q6** revisability, by digest | a digest over the retained output; a threshold in `scripts/classify_simulation_runs.py` edited; the classifier re-run; the classes change and the digest is **unchanged** |
| `REQ-MOK-082` | automated test | **Q7** rows alone | a distribution is produced in a checkout with no engine binary present and no stream on disk; the classifier spawns no process and opens no socket |
| `REQ-MOK-082` | automated test | **Q8** class counts sum | in every group, the five class counts plus unclassified equals the group's row count |
| `REQ-MOK-082` | automated test | **Q9** both axes | a distribution over `source` at fixed density, and over `density` at fixed source, are both produced from the same rows; each states rule 17.1's counts and order statistics |
| `REQ-MOK-082` | automated test | **Q10** attempted stays distinct | no group figure merges `attempted` and `effective`; both are reported for all three kinds |
| `REQ-MOK-082` | automated test | **Q11** an unobserved class | over rows in which no run meets the famine predicate, `famine` is reported with count `0` in every group and is not omitted |
| `REQ-MOK-082` | automated test | **Q12** the sweep travels | every distribution output restates the `sweep` record in full, including axis values that produced no rows, and states the engine version |
| `REQ-MOK-082` | automated test | **Q13** two engines are two experiments | rows carrying two distinct `engine` values are reported as separate groups and never merged into one |
| `REQ-MOK-082` | automated test | **Q14** small and empty groups | a one-row group is reported with equal order statistics; a zero-row aggregation is reported as zero rows and is not an error |
| `REQ-MOK-082` | automated test | **Q15** determinism | two classifier runs over the same rows with the same options are byte-identical |
| `REQ-MOK-083` | automated test | **B19** incompleteness is per-cell | a sweep of 60 cells in which 3 fail reports `requested` 60, `retained` 57, `complete` false, and names each of the 3 by full coordinates with a stage and a reason |
| `REQ-MOK-083` | automated test | **B20** the six stages | each of rule 12.3's six `stage` values is produced by a case that causes it, and each is distinguishable from the others |
| `REQ-MOK-083` | automated test | **B21** the engine's own words | a cell failing because the engine exited non-zero carries the engine's message verbatim in its reason, byte for byte |
| `REQ-MOK-083` | automated test | **B22** nothing is fabricated | for a missing cell, no row exists at those coordinates and no neighbouring seed's or density's row is duplicated; the retained row count equals the `cell` record count |
| `REQ-MOK-083` | automated test | **B23** exit statuses | rule 14.1's four statuses each arise from a case that causes them; `3` is returned for an incomplete sweep whose output was written, and `0` only when `complete` is true |
| `REQ-MOK-083` | automated test | **B24** total failure | a sweep in which every cell fails writes `sweep` and `batch`, no `cell` record, `complete` false, and the classifier states no distribution over it |
| `REQ-MOK-083` | automated test | **B25** unwritable output | where the retained output cannot be written, the batch exits `1` and retains nothing, rather than retaining rows whose completeness is unrecorded |
| `REQ-MOK-083` | automated test | **Q16** the distribution carries it | a distribution over an incomplete sweep's rows states the incompleteness and its missing-cell count; the figures cannot be read without it |
| `REQ-MOK-083` | automated test | **Q17** no sweep record | a distribution requested over rows with no `sweep` record is refused, and unknown completeness is treated as incomplete |
| `REQ-MOK-083` | automated test | **B26** partial rows survive | in B19's sweep, the 57 rows are complete, valid and classifiable, and none was discarded because of another cell's failure |
| all six | automated test | **B27** batch determinism | two batches over the same sweep at the same commit produce byte-identical retained output, under `--jobs 1` and `--jobs 4` |
| all six | static check | **T1–T8** structure and architecture | see *Static and architecture checks* |
| all six | static check | **T9** integers only | no floating-point value appears in any retained byte or any machine-readable classifier output |

## Acceptance scenarios

1. **A distribution that discriminates.** Over the default sweep, `baseline` and the three rule-following sources
   produce visibly different class distributions, and the direction matches the sources' stated behaviour. This is the
   capability's purpose and is checked as an outcome rather than inferred from the arithmetic. It is **not** a required
   distribution: the scenario passes on the distribution being stated and legible, not on any particular shape, because
   `SPEC-MOK-008` rule 17.7 forbids a passing shape.
2. **The density axis is the one that moves the outcome.** Over the default sweep, the class distribution differs
   across densities at fixed source, and this is stated as a measurement. The 2026-08-30 finding that seed varies the
   figures without changing the class is re-measured at twenty seeds rather than five, and whichever way it comes out is
   recorded — a reversal at twenty seeds is a finding, not a failure.
3. **A threshold is revised after publication.** A distribution is produced and its rows digested; a threshold is
   edited; the distribution is regenerated. The new distribution differs, the rows are byte-identical, and the old
   distribution is reproducible from the same rows by reverting the edit. This is the whole point of
   `SPEC-MOK-006` rule 8.7 and is checked as a workflow, not only as Q6's digest equality.
4. **A batch that loses cells at one end of an axis.** A sweep in which every cell at one density fails produces a
   distribution in which that density is present in the `sweep` record, absent from the rows, and named in the missing
   list. A reader can see that the axis is incomplete without being told.
5. **A stream that no longer exists is re-derived.** A row is chosen at random from a completed sweep, its cell
   re-executed from its coordinates alone, and its digest and every derived count reproduced.
6. **The instrument finds a defect.** The threat inertness of rule 9.6 is reproduced over the default sweep and
   reported as a figure. The scenario passes when the instrument surfaces it; it does not require the defect to be
   absent, and it does not require it to be repaired. `INT-MOK-012` principle 7 is the ground.

## Property and invariant tests

- **P1 — One row per completed cell.** The `cell` record count equals `retained`, and `retained` plus the length of
  `missing` equals `requested`, on every sweep including a totally failed one.
- **P2 — A row is a function of its cell.** The same coordinates at the same commit produce the same row, whatever
  order cells ran in, whatever `--jobs` was, and whichever build profile was used.
- **P3 — Classification is a function of the row and the definitions.** A row's class does not depend on how many rows
  are present, on the order they were read, or on any other row's contents. Checked by classifying a row alone and
  within a 400-row file and comparing.
- **P4 — Rows are immutable.** No execution of the classifier, at any option setting, modifies the retained output.
  Checked by digest before and after every classifier case in this contract.
- **P5 — Peak disk is one stream.** Retained stream bytes on disk never exceed one stream's size during a batch,
  measured rather than argued.
- **P6 — `attempted` bounds `effective`.** For each of the three kinds, `effective` is never greater than `attempted`,
  and `lethal_attacks` is never greater than `effective` for attacks. A violation is a defect whatever its value.
- **P7 — No total is a rate.** Every numeric value in retained output is an integer. Stated as an invariant because it
  is the property a later convenience field is likeliest to break.

## Static and architecture checks

These are `ADR-MOK-008`'s checkable claims. That record's substance is that **no amendment is required**, and a
verification contract is the only place such a claim becomes falsifiable.

- **T1 — No third package.** `Cargo.toml`'s workspace members are exactly `mokiterions-core` and `mokiterions-tui`,
  unchanged from the base commit. No directory holds the sources of a third package, and `SPEC-MOK-004` rule 1's
  structural census is re-measured and matches.
- **T2 — No product file is touched.** `git diff --name-only` against the base names no file under
  `mokiterions-core/src/`, `mokiterions-tui/src/`, either `Cargo.toml`, or `Cargo.lock`.
- **T3 — The schema is still 3.** A freshly produced stream's `header` record states schema `3`, and `SPEC-MOK-006` is
  unmodified. No retained record-stream digest elsewhere in the repository changes, checked by re-verifying at least one
  existing evidence manifest that binds one.
- **T4 — Declared dependency sets unchanged.** `scripts/check_declared_dependencies.py` passes, and both packages'
  declared sets are identical to the base commit's.
- **T5 — Standard library only.** Every `import` in both instruments and both test files resolves to a standard-library
  module, enumerated and checked against the running interpreter's standard module set rather than against a hand-kept
  list.
- **T6 — The instruments are not targets.** Neither script is referenced by any `Cargo.toml`, and the Cargo target
  census `SPEC-MOK-002` rule 5 records is unchanged.
- **T7 — Nothing writes into governance.** Neither instrument writes under `docs/engineering/`; a batch whose `--out`
  names such a path exits `2`. Checked both as behaviour and by pattern over the instruments for any such write path.
- **T8 — No retreat field is synthesised.** No field named for retreat appears in any retained record, and the absence
  of a `retreat_resolved` event kind in `SPEC-MOK-001`'s vocabulary is recorded as a measured gap against
  `docs/ROADMAP.md`'s Phase 6 wording.
- **T9 — Integers only.** No floating-point literal or formatted decimal appears in any retained byte or in the
  classifier's machine-readable output. `density` is carried as the engine's own string.
- **Lint and format.** The repository's existing Python checks pass on both new instruments and both test files, and
  `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` and `cargo fmt --check` remain clean —
  the latter two as evidence that the product is genuinely untouched.

## Security and privacy checks

- **No credential, and no path by which one could arrive.** Neither instrument reads any environment variable, and rule
  4.5 passes none to a child beyond the operator's own. Checked by pattern over both instruments for any environment
  read, and by executing a batch with a decoy variable set in the environment and confirming its value appears in no
  retained byte and no output.
- **No provider is reachable.** Neither instrument opens a socket or makes a network call, checked statically over the
  imports under T5 and behaviourally by running a batch with no network available. No connector option is passed under
  rule 4.4, and `llm` is refused under B5.
- **No live selection.** No case in this contract executes a live run, and none is permitted by it. `REQ-MOK-072`
  governs that and nothing here relaxes it.
- The evidence packet is inspected for incidental capture of any credential or token before it is committed, and a
  positive finding is a stop rather than a redaction.
- The engine's own message carried verbatim under B21 is the single channel by which text the instrument did not author
  reaches a retained byte. It is inspected in the evidence for anything beyond a configuration message.

## Performance and resilience checks

- **The default sweep's cost is measured and reported, not asserted.** Wall time for the batch, wall time for the
  classifier, retained bytes, and peak stream bytes, with the platform stated. `ADR-MOK-008` predicted about 19 s of
  execution, about 16 s of scanning and about 500 KB retained from a 20-run extrapolation; the measured figures are
  reported beside those predictions, and a material divergence is a finding about the extrapolation rather than a
  failure of the instrument.
- **Scanning cost is linear in stream bytes**, checked at two horizons rather than argued.
- **A long run.** At least one cell is executed at a horizon long enough to produce a stream materially larger than the
  measured 2,723,014 bytes, to establish that the driver streams rather than reading a whole file into memory. Memory
  high-water is reported.
- **Interruption.** A batch interrupted partway writes its `sweep` and `batch` records, names unreached cells as
  `not_attempted`, and leaves no stream behind. This is executed rather than reasoned about, because it is the state an
  operator will actually reach.
- **Concurrency.** `--jobs 4` is exercised on the default sweep, and B3, B27 and P2 all hold under it.

## Manual assessments

Each is recorded with the assessing role, the date, and the evidence read. An unsigned assessment is not a recorded one.

1. **Does the distribution answer Phase 6's question?** The assessor reads the default sweep's distribution and states
   whether *the conditions permit emergence* and *outcomes are not engine-determined* are answerable from it, and if not,
   what is missing. Assessed by the product owner.
2. **Is the class vocabulary the right one?** Whether rule 16.5's five classes name what actually happens in the swept
   space, and whether any observed outcome has no class that fits. Assessed by the product owner against rows, not
   against the specification text.
3. **Is the famine disclosure honest and sufficient?** The assessor states whether rule 16.8's record — the predicate
   unreached in 35 runs, and 8 units of food standing in each territory at extinction at density `0.10` — is stated
   plainly enough that a later reader cannot mistake the class for a measured one, and whether retaining the predicate
   at all is the right call. Assessed by the technical owner.
4. **Is the attempted-and-effective split justified by its measurement?** Whether rule 9.6's figure supports two
   counters rather than one, and whether the three effectiveness predicates read the right field. Assessed by the
   technical owner.
5. **Is `ADR-MOK-008`'s "no amendment required" claim sound?** The assessor independently reads `ARCH-MOK-001`'s
   prohibited-patterns list, `SPEC-MOK-004` rule 1 and `ADR-MOK-003` decision 1 against what was built, and states
   whether any of the three is in fact touched. T1 to T8 are mechanical; this is the judgement they cannot make.
   Assessed by the assurance owner.
6. **Is Python becoming load-bearing for published results an accepted consequence?** `ADR-MOK-008` records it as a
   real widening. The assessor states whether it is acceptable as built. Assessed by the technical owner.
7. **Are the reported measurements reproducible?** The assessor recomputes at least one retained digest and one derived
   count independently of the retained command output. Assessed by the assurance owner.
8. **Does the instrument's disclosure of the threat defect meet the owner's 2026-08-30 decision?** That decision was
   that 4b discloses the finding and a later chain repairs it. The assessor states whether the disclosure as built is
   sufficient to serve as the *before* figure for that later chain. Assessed by the assurance owner.

## Evidence retention

Retained under `docs/engineering/simulation/evidence/WO-MOK-033/` with a SHA-256 manifest covering every file, written
with the line endings the manifest hashes, on the `-text` attribute that tree carries.

- Full command transcripts: both instruments' test runs, clippy, fmt, the declared-dependency check, and the whole
  batch.
- The default sweep's retained output, with its digest. Rule 15.1 forbids a batch to write under `docs/engineering/`, so
  this is a deliberate copy and the copying command is retained with it.
- The distribution outputs for scenarios 1, 2 and 4, in both formats, with the commands that produced them.
- The independent-count comparison for B12, B13, B14 and B16, retained as the two counted tables and their diff, so a
  reviewer recomputes rather than reads a conclusion.
- The seven B17 fixture streams and the refusals they produced.
- The Q6 revisability record: the rows' digest before, the exact threshold edit as a diff, the two distributions, and
  the rows' digest after.
- The B7 digest reproduction, including the debug-build and release-build pair.
- The threat inertness figure over the default sweep — attempted and effective, per source and per density — retained
  as the *before* measurement for a later repair chain, with the 2026-08-30 figure of 1,447 of 1,448 beside it.
- The famine measurement: `regeneration_skipped.depleted` across the whole sweep, and the standing-food figures at
  density `0.10` at the final tick.
- The T1 to T9 static checks with their commands and outputs, including the `SPEC-MOK-004` rule 1 census measurement.
- The performance figures with the commands and the platform stated, beside `ADR-MOK-008`'s predictions.
- The interruption capture, including the `batch` record it wrote.

Where a bullet is met by substitution rather than as written, the record says so on that bullet rather than
reinterpreting the bullet.

## Residual uncertainty

1. **Both sides of every count read the same engine.** The oracles are independent of the *instrument*, not of the
   engine. A defect in the engine's own records would be reproduced identically by the instrument and by this
   contract's independent count, and this contract would pass. Rule 10's seven equalities narrow the gap, because a
   disagreement between the engine's two accounts of the same fact is detectable — but a fact the engine records wrongly
   in both places is not. That risk is carried by `SPEC-MOK-001`'s and `SPEC-MOK-006`'s own verification and is not
   reducible here.
2. **The cross-checks passed everywhere, so their sensitivity is only evidenced by fixtures.** B17 establishes that each
   check fires on a crafted disagreement. It does not establish that the checks would catch a real engine defect of a
   shape nobody anticipated.
3. **`REQ-MOK-079`'s binding rests on measured determinism, not on proof.** Determinism was measured in both
   directions, including across build profiles, on one platform, at one commit. A row bound to a destroyed stream is
   only as trustworthy as that property, and B7 samples it rather than proving it. `SPEC-MOK-008` rule 11.5 makes a
   non-reproducing digest a reported defect, which is the mitigation and not a closure.
4. **The famine class is verified as *stated and disclosed*, not as correct.** No case can establish that its predicate
   is the right one, because no measurement supports any predicate for it. Manual assessment 3 is the only judgement
   available and it is a judgement.
5. **The distribution has no pass condition, by design.** Scenario 1 checks that a distribution is produced and
   legible. Whether the distribution *supports* `INT-MOK-001`'s claim is a reading of evidence, not a test result, and
   `SPEC-MOK-008` rule 17.7 forbids this contract to assert a required shape. Phase 6 is where that reading happens.
6. **Twenty seeds is a sample.** Scenario 2's finding about the seed axis is a measurement over twenty seeds at five
   densities, not a property of the engine. A larger sweep could contradict it.
7. **The performance figures are indicative.** They are retained so a later regression has a baseline, not because a
   threshold is being asserted.
8. **`--jobs` concurrency is checked for row identity, not for freedom from every race.** B27 and P2 establish that the
   retained output is byte-identical at two job counts on the platforms measured. A race that manifests only under
   different scheduling remains possible, which is why rule 2.8's default is 1.
