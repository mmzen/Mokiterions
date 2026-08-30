+++
id = "VREC-MOK-027"
type = "verification_record"
title = "Verification candidate for WO-MOK-033"
status = "verified"
owners = ["engineering owner"]
created = "2026-08-30"
updated = "2026-08-30"
commit = "f79dcf225fea7eb7bce1323141385dc1839a756c"
git_object_format = "sha1"
worktree_state = "clean"
prepared_at = "2026-08-30T18:19:53Z"
prepared_by = "engineering owner"
artifact_snapshot_sha256 = "223600098ed6bb9c6f19e23c40e0862b37560bbed9f208f0c6bd1c461e98c53d"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-033/README.md", "docs/engineering/simulation/evidence/WO-MOK-033/assembly.txt", "docs/engineering/simulation/evidence/WO-MOK-033/case-index.md", "docs/engineering/simulation/evidence/WO-MOK-033/findings/disclosed-gaps.md", "docs/engineering/simulation/evidence/WO-MOK-033/findings/instrument-defects.md", "docs/engineering/simulation/evidence/WO-MOK-033/findings/manual-assessments.md", "docs/engineering/simulation/evidence/WO-MOK-033/findings/sweep-findings.md", "docs/engineering/simulation/evidence/WO-MOK-033/handoff-check.md", "docs/engineering/simulation/evidence/WO-MOK-033/manifest.sha256", "docs/engineering/simulation/evidence/WO-MOK-033/not-committed.txt", "docs/engineering/simulation/evidence/WO-MOK-033/phase-a/result.json", "docs/engineering/simulation/evidence/WO-MOK-033/phase-b/result.json", "docs/engineering/simulation/evidence/WO-MOK-033/phase-c/result.json", "docs/engineering/simulation/evidence/WO-MOK-033/phase-d/result.json", "docs/engineering/simulation/evidence/WO-MOK-033/phase-e/result.json", "docs/engineering/simulation/evidence/WO-MOK-033/phase-f/result.json"]
evaluator_evidence_path = "docs/engineering/simulation/evidence/VREC-MOK-027-evaluator.json"
evaluator_evidence_sha256 = "4f500366462d5da855322aa725d6a1d23250f1ef82b37e371b43317ef81945b6"

verified_at = "2026-08-30T18:23:17Z"
verified_by = "assurance-owner"
[relations]
verifies_work_order = ["WO-MOK-033"]
conforms_to = ["VER-MOK-019"]

[[lifecycle_events]]
from = "ready"
to = "verified"
decided_at = "2026-08-30T18:23:17Z"
decided_by = "assurance-owner"
reason = "DR-VREC-DECIDE decided verified by the repository owner acting as accountable assurance owner on 2026-08-30, by selecting the presented option to approve and execute WO-MOK-033 through preparing and verifying this record. All three permitted outcomes were available. VER-MOK-019's mechanical half is satisfied: 84 cases across six phases, 0 failed, at candidate f79dcf2 with a clean worktree. Its judgement half is NOT: the eight manual assessments are reserved to the product, technical and assurance owners, an unsigned assessment is not a recorded one, and none is recorded -- the record discloses that gap in its first section rather than closing it. Four defects were found and repaired, three in the driver and one in its own suite, each with a test measured to fail on the pre-fix code (3 of 3). Nine gaps in SPEC-MOK-008 and one wrong citation in VER-MOK-019 case T6 are disclosed and not amended, per condition 6. Four findings are carried and not repaired, among them that the seed changes the outcome class in 10 of 20 groups, reversing a five-seed reading, and that the threat mechanism is effective 8 times in 7,701 resolutions. Residual uncertainties 6 and 8 are recorded as moved by this sweep: the --jobs race was real."
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-033` to candidate commit `f79dcf225fea7eb7bce1323141385dc1839a756c`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## What this record verifies, stated before anything else

`VER-MOK-019` has two halves. The mechanical half — 84 cases across six phases — is **executed and
passing at this candidate**. The judgement half — eight manual assessments reserved to the product,
technical and assurance owners — is **not recorded**, and this record discloses that rather than
closing it.

So: this record binds the evidence that the two instruments measure what they claim to. It does not
record the owner's reading of what they measured.

| Phase | What it covers | Cases | Failed |
|---|---|---|---|
| A | the real engine end to end | 19 | 0 |
| B | failure, refusal and disagreement | 16 | 0 |
| C | options, classes and revisability | 22 | 0 |
| D | static, architecture, security and privacy | 15 | 0 |
| E | performance and interruption | 6 | 0 |
| F | acceptance scenarios and the famine measurement | 6 | 0 |

Each case is one assertion over measured figures. The figures are in each phase's `result.json`, all
six of which are bound below, and `case-index.md` lists every case in the order executed.

### The eight manual assessments are not recorded

`VER-MOK-019` states it plainly: *an unsigned assessment is not a recorded one*. Assessments 1 and 2
are the product owner's, 3, 4 and 6 the technical owner's, 5, 7 and 8 the assurance owner's. None is
recorded here, and an implementation agent cannot sign any of them. The three roles are all held by one
person, so no role can be substituted by another either.

`findings/manual-assessments.md` states, for each one, what it needs read. Assessment 5 is the one to
note: `ADR-MOK-008` claims no amendment is required to `ARCH-MOK-001`'s prohibited-patterns list,
`SPEC-MOK-004` rule 1 or `ADR-MOK-003` decision 1. Cases T1, T2, T4, T5 and T6 measure everything
measurable about that claim — the workspace has two members and no third package, the structural
census is 40 paths blob-for-blob identical to `c90edc9` with a clean worktree over all of them, no
product file is touched, both packages' dependency tables are byte-identical and their declared sets
unchanged, every import of all four Python files resolves to the standard library, and neither
instrument is a build target of any of the three `Cargo.toml` files. Whether that amounts to *none of
the three is touched* is the judgement the cases cannot make.

## The five Independence points, and how each was met

1. **Every derived count is checked against this contract's own straight-line count.** The oracle in
   `harness/oracle.py` recomputes every behavioural counter directly over the stream bytes and never
   calls the instrument's counting path. Cases B12, B13 and B14 are the comparison.
2. **The instrument's own cross-checks are not their own evidence.** All seven rule 10 equalities
   passed in every real stream measured, which is exactly why B17 exists: seven fixtures, each
   breaking one equality by one, produce **0 rows and 7 refusals at stage `crosscheck`**. The same
   principle was applied a second time, to the regression tests themselves — see the four defects
   below — and it is what caught defect 2.
3. **At least one case runs the real engine end to end.** Phase A is that case set, and it is where
   three of the four defects were found. Rule 20.2 keeps the instruments' own suites on fixtures, and
   the suites' 78 cases were all passing while three defects stood.
4. **Revisability is checked by digest.** Q6 hashes the retained output, edits a threshold, re-runs the
   classifier and re-hashes: the report moves and no retained row does.
5. **The retained rows are their own oracle for the discard.** B7 re-executes a cell whose stream was
   destroyed and reproduces the bound digest; scenario 5 does it again on a randomly chosen row —
   number 133, `reference:0.25:13`, digest `edae218a6c75f78b…` — with all seven rule 10 equalities
   re-derived over the fresh stream.

## What was found: four defects, all repaired

Three in the driver, one in its own test suite. Each has a test that fails on the pre-fix code, and
`harness/bite/check.py` is the instrument that measures that: it patches one defect back into a fresh
copy, runs the one test that names it, and reports `3 of 3 tests fail on their own pre-fix copy`.

1. **An explicit `--binary` path was not normalised to the platform's separator.** A forward-slash
   relative path passes `os.path.isfile` and then fails to launch, so a 400-cell sweep reported 400
   identical failures and no cells. Found by case B1 against the real engine; the `runner` seam cannot
   find it, because the seam never reaches the process boundary.
2. **The test written for defect 1 passed against defect 1.** It took its relative path from
   `os.path.relpath(__file__)`, which carries no separator when the suite is started from `scripts/`.
   Found by `harness/bite/check.py`, and it is the reason that instrument exists. Repaired by building
   the path inside a temporary tree, with `os.chdir` rather than `contextlib.chdir` so no Python 3.11
   floor is introduced into a suite whose specification states no minimum version.
3. **One reusable stream path could be held by two concurrent cells.** The slot was `index % jobs`;
   with eight cells over four workers, position 4 starts when *any* of 0 to 3 finishes. Measured
   against the real engine: **two of eight cells lost**, one reading a half-written line and one
   finding the file already removed. Repaired with a `queue.SimpleQueue` slot pool, which makes
   exclusion a property of the pool's size rather than of the scheduler's order.
4. **A cell interrupted mid-run left its stream on disk while the record said none was left.** Rule
   7.3's removal was on the normal path only. Both halves are wrong and the second is worse: a
   `leftover_streams` field that disagrees with the directory is worse than no field. Repaired with a
   `finally:`.

Defects 1 and 3 both produce a sweep an operator would have read as a measurement. That is the
argument for Independence point 3, and it is worth carrying into the next contract of this shape.

## What is disclosed and not repaired

`WO-MOK-033` condition 6 forbids amending an approved artifact on an implementation agent's judgement.
Nine gaps in `SPEC-MOK-008` and one wrong citation in `VER-MOK-019` are therefore built one way and
disclosed. `findings/disclosed-gaps.md` states each in full; in brief:

1. Rule 12.7 sends an unwritable output to rule 15's status `2`; rule 14.1's table gives it `1`. Built
   as `1`.
2. Rule 7.2 constrains where the stream goes and rule 2 has no option to say where. Built with a
   `--stream-dir` option the specification does not list.
3. Rule 7.1's *one reusable path* and rule 2.8's `--jobs` cannot both be literal. Built as one path per
   worker — and reading 7.1 literally is what defect 3 was.
4. Rule 12.3's `refused` stage is unreachable per cell; every refusal the specification defines is a
   whole-sweep refusal before any cell runs. Five of the six stages appear in this packet and `refused`
   is the one that cannot.
5. `crosschecks_passed` is `7` on every retained row that can ever exist, because a row exists only if
   all seven passed. Built as specified; a reader treating it as a measurement is reading a constant.
6. Rule 14 states exit statuses for the driver only. The classifier is built with `0` and `2` and no
   `1` or `3`, on rule 14.3's one-convention reasoning.
7. Case Q4's missing-fact rule has no home in rule 16 or 17.1. Built to satisfy Q4.
8. Rule 17.1 omits `attempted`, which rule 9.6 requires exist. Built with both, which is a superset of
   17.1 rather than a conflict with it.
9. Rule 16.8's `famine` predicate is still unreached at 400 runs: `depleted` is 0 against 12,827
   capacity skips, and at density `0.10` all 80 cells end extinct with 16 units of food standing across
   both territories. Built as specified and reported as zero per rule 16.9.
10. `VER-MOK-019` case T6 cites `SPEC-MOK-002` rule 5 for a target census that rule does not record.
    The case was executed against the three `Cargo.toml` files directly — 25 targets, none of them a
    Python file — so the fact T6 wants is measured and the citation is what is wrong.

## What is carried and not repaired

Four findings about the swept space, in `findings/sweep-findings.md`:

1. **`asymmetric_collapse` accounts for 32 of 400 cells** — 7 `reference`, 9 `individual`, 16 `social`,
   0 `baseline` — where the earlier 35-run reading saw none. The class is real.
2. **The seed changes the outcome class in 10 of the 20 source-and-density groups**, and moves the run
   figures in all 20. This reverses the five-seed finding of 2026-08-30, which was that the seed moved
   figures but not classes. Scenario 2 requires the result be recorded either way. The consequence:
   a single-seed run states nothing about its cell.
3. **The threat mechanism is effective 8 times in 7,701 resolutions** (0.1039%), all 8 under `social`.
   This is the *before* figure for the repair chain the owner deferred on 2026-08-30, and whether it
   suffices as that figure is manual assessment 8.
4. **`retreat` is a targeted action with no resolved-event kind.** `SPEC-MOK-001` declares no
   `retreat_resolved`; the engine emitted 14 kinds over eight real streams and none names retreat. It
   is available as an intention and unobservable as an outcome, so no sweep can report on it. This
   belongs to the roadmap reconciliation, which is outside every work order.

Carried from earlier work and not touched here: `VER-MOK-018` case `L20`'s "binary target" wording;
`SPEC-MOK-006`'s two rules both numbered 8.9; issue #40; and the nine defects in six approved artifacts
deferred by Phase 4a.

## The residual uncertainties, at this candidate

`VER-MOK-019` names eight. Six stand unchanged. Two are moved by this sweep and the movement is
recorded here rather than left to be rediscovered:

- **Residual 6, twenty seeds is a sample.** It now cuts harder than when it was written: the seed axis
  turned out to move the class in half the groups, which the five-seed reading had missed. A larger
  sweep could move the figure again in either direction.
- **Residual 8, `--jobs` is checked for row identity and not for freedom from every race.** A race was
  in fact found — defect 3 — and it was found by a case built as an exclusion property over observed
  time spans, not by the byte-identity comparison, which cannot see it. The residual is therefore not
  hypothetical, and rule 2.8's default of 1 is doing real work.

Residuals 1 to 5 and 7 stand as written. Residual 4 in particular: the famine class is verified as
*stated and disclosed*, not as correct, and 400 runs did not change that.

## The evidence binding

Sixteen files are bound in `evidence_paths`. The packet holds 148, and binding all of them would be
a list no reader reads. The sixteen are the packet's own index and its judgements: `README.md`,
`manifest.sha256`, `handoff-check.md`, `case-index.md`, `not-committed.txt`, `assembly.txt`, the four
`findings/` files, and all six phases' `result.json`.

`manifest.sha256` is the transitive binding: it carries the sha256, byte count and line count of the
other 147 files, and it is itself bound here, so any change to any retained file breaks a digest a
reader can check. It does not cover itself and says so — it is the one file it cannot cover.

Forty captures over 400,000 bytes, 141,111,666 bytes in total, are **not committed**. They are engine
record streams and scan inputs, stated in `not-committed.txt` with a digest, a byte count and a line
count each, on `WO-MOK-025`'s precedent. The phase scripts that produced them are retained under
`harness/` and the streams are deterministic in the engine's seed, which is what B7 measures.

Bytes are retained as captured. The evidence tree carries `-text`, so the 29 files that came off a
Windows standard-output pipe keep their CRLF and every digest in the manifest reproduces on any
platform. `assembly.txt` lists those files and their CRLF counts.

## What this record does not cover

The `docs/ROADMAP.md` reconciliation, which is owed under its own trailer-less `docs(roadmap):` change
and is outside every work order. The rule 14 fifth-price work order, undrafted, which becomes
`WO-MOK-034`. `WO-MOK-027`, stage 5c, which the owner set aside. The fear-and-threat repair chain,
deferred by owner decision on 2026-08-30, whose *before* figure this packet retains.

## The decision: transitioned to `verified` on 2026-08-30

The repository owner, acting as accountable **assurance owner**, took this transition on 2026-08-30
under `DR-VREC-DECIDE`, with all three permitted outcomes available. It was taken **by selecting the
presented option**, whose label was *"Approve and execute"* and whose description was to approve all
twelve artifacts and then execute `WO-MOK-033` — build both instruments and their tests, run the
400-cell sweep, retain the evidence packet, and prepare and verify `VREC-MOK-027`. That is the channel
and the wording; no utterance is attributed to the owner beyond the option they selected.

**`status` moved from `ready` to `verified` and the evaluator wrote `verified_at`, `verified_by` and
one lifecycle event. Nothing else in the frontmatter moved.** `commit`, `worktree_state`,
`prepared_at`, `prepared_by`, `artifact_snapshot_sha256`, the sixteen `evidence_paths`, both evaluator
evidence fields and both relations stand exactly as `capture-verification` wrote them. The transition
was applied through the 0.8.0 evaluator rather than by hand-editing the field, so the provenance is the
tool's and not this agent's. The heading still says *Candidate*, for the same reason `VREC-MOK-025`'s
and `VREC-MOK-026`'s do: a transition that moves only `status` does not get to rewrite the prose to
read better.

### What the decision was taken over

The selection was made before the eight manual assessments were known to be unrecordable by anyone but
the owner, and before the four defects and ten disclosures existed to be read. Everything above this
section was therefore written while the record was still `ready` and correctable, and the decision is
recorded as taken over that text: 84 mechanical cases passing, eight owner judgements outstanding, four
defects repaired, ten disclosures, four findings carried, two residual uncertainties moved.

A verified record cannot be corrected. If the owner's reading of any figure above differs from what is
written, the route is a successor record and not an edit to this one.
