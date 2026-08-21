# `WO-MOK-016` evidence packet — index

| Field | Value |
|---|---|
| Work order | `WO-MOK-016` (Phase 3.1, encounters) |
| Verification contract | `VER-MOK-016` |
| Implements | `REQ-MOK-051` … `REQ-MOK-060` |
| Baseline (pre-change) commit | `39662d13abd08e3410648d1c59ad38384f8ad2d2` |
| Candidate commits | **two, and the difference matters.** `7c4aef3967406c05d80da963695898b77f5329e9` — the 90-cell three-source matrix and the first test log. `59d61b915630fd55f04bcdbb346aa22cdbfdfff6` — the 30 `social` cells and the amended suite, after the `REQ-MOK-057` amendment. `post/COMMIT.txt` holds the first; `post/capture-state.txt` §5 relates the two and measures the ninety cells as **unchanged** at the second, which is why they were not retaken |
| Branch | `feature/phase-3-definition` |
| Date opened | 2026-08-20 |
| Packet size | 147 files, 2,675,176 bytes |
| Renumbered | on 2026-08-21, from `WO-MOK-012` / `VER-MOK-012` / `VREC-MOK-012` / `INT-MOK-009` / `CAP-MOK-009` / `REQ-MOK-042`–`051`. See the note immediately below |

**This chain was renumbered after the evidence was captured, and the packet says so wherever it shows.**
The chain was numbered on 2026-08-19 from the highest identifier then visible on `master`. By 2026-08-21
every number in it except one had been taken by work published elsewhere: `WO-MOK-012` by a different
work order since merged to `master`, `REQ-MOK-047`–`049` by requirements merged with it, `INT-MOK-009`,
`CAP-MOK-009` and `REQ-MOK-042`–`046` by `feature/phase-4a-definition`, and `REQ-MOK-050` by
`governance/adr-mok-006-third-party-crates`. The identifier space is shared across branches, so the
local maximum was never the free number. This chain is the unpushed one, so this chain moved:

| Was | Is | | Was | Is |
|---|---|---|---|---|
| `WO-MOK-012` | `WO-MOK-016` | | `REQ-MOK-046` | `REQ-MOK-055` |
| `VER-MOK-012` | `VER-MOK-016` | | `REQ-MOK-047` | `REQ-MOK-056` |
| `INT-MOK-009` | `INT-MOK-010` | | `REQ-MOK-048` | `REQ-MOK-057` |
| `CAP-MOK-009` | `CAP-MOK-010` | | `REQ-MOK-049` | `REQ-MOK-058` |
| `REQ-MOK-042` | `REQ-MOK-051` | | `REQ-MOK-050` | `REQ-MOK-059` |
| `REQ-MOK-043` | `REQ-MOK-052` | | `REQ-MOK-051` | `REQ-MOK-060` |
| `REQ-MOK-044` | `REQ-MOK-053` | | *evidence directory* | `evidence/WO-MOK-016/` |
| `REQ-MOK-045` | `REQ-MOK-054` | | `VREC-MOK-012` | `VREC-MOK-016` |

**981 identifier references were rewritten, and the rewrite was verified rather than trusted.** Every
reader in this packet whose output names the evidence path was **re-run at the new path**, and each
reproduced its retained output byte for byte: `analysis/test-census.py` over all three test logs, and
`analysis/horizon.py` over the ten long-horizon cells at exit `0`. A substitution that changed a figure
would have shown up as a difference there and did not.

**Eighteen further references were found after that commit and corrected in the merge that follows it, and
both misses have the same shape: a reference that names an identifier without spelling it.** Twelve are
forward references to `VREC-MOK-012`, the record this packet is written for, which no substitution of
`VER-MOK-012` could reach because `VREC` is not `VER` — nine of them in this packet, one in
`SPEC-MOK-001`'s amendment record and two in `WO-MOK-016`. Six are bare-number back-references in
`WO-MOK-016`'s lifecycle sentence — `043`, `044`, `046`, `047`, `049` and `051`, naming requirements whose
prefix the sentence had already given once — which no substitution of a full identifier could reach either.
A substitution keyed on whole identifiers is blind to both forms, so the sweep that found them was keyed on
the shape of the reference instead: every `VREC-MOK-0NN` in the tree, and every backtick-quoted bare
three-digit number in every artifact this branch touched. The running total is **999**.

One reader was not re-run, for the reason it exists to record. `analysis/peak.py` does not reproduce its
own figures from one invocation to the next — that is `post/long-horizon.md` §12's finding, not a defect
— so re-running it would have replaced four honest measurements with four different ones. Its four
invocations are retained exactly as taken, and in the four files whose records name the pre-rename path
(`post/horizon-memory.txt`, `post/capture-state.txt`, `baseline/capture-state.txt`,
`baseline/cross-check.txt`) **captured command lines and captured command output keep the path as it
stood when the command was run, while bare identifiers are current.** A record of a command that was run
must not be edited to claim a different command was run. Each of those four files carries the note.

**This packet is incomplete, and one requirement it measures is deliberately unimplemented.** Both are
stated here so that neither can be mistaken for anything else:

- **`REQ-MOK-060` is not implemented**, under the product owner's approved deferral of 2026-08-20. It is
  what keeps `WO-MOK-016` out of `implemented`, it is why `post/byte-identity.txt` reads
  `RESULT: MIXED` — the 60-cell divergence that oracle 1 asks to be characterized does not exist to be
  characterized — and it is recorded rather than worked around.
- **Three of `VER-MOK-016`'s retention items are not yet written**, and the table at the end of this
  file marks every one. Nothing is held back any more: the direction of 2026-08-20 that held the
  decision-dependent items until the `REQ-MOK-057` amendment was measured has been discharged — the
  amendment landed, and `post/runs.md`, `post/branches.md` and `identifier.md` are those items. What
  remains is simply unwritten, except the eleven manual assessments, which are the owner's acts and not
  the implementation's.

**The suite passes.** 264 names, 264 passing, 0 failing, 0 ignored, exit `0`, reconciled name by name
against `master`'s tip's 226 in `post/test-census-reconciliation.md` §9. Before the merge of 2026-08-21
it read 250 against the branch point's 212, in §§1 to 8, and both comparisons are kept: the 39 names this
work order adds and the one it renames away are **the same names in both**, so re-basing the comparison
onto `master`'s tip moved the totals either side of them and nothing between them. It did not always
pass, either: three
requirement-bearing oracles failed at `7c4aef3`, and `escalation.md` is the record of that failure, of
the seventeen variants measured across three levers, of the four owner decisions taken on it, and of the
amendment that resolved it. **Read it first.** A packet whose suite passes today, on a source whose
ordering was measured to be unsatisfiable as first approved, is not self-explanatory without it.

The reason the pre-change side exists as its own commit is `VER-MOK-016` oracle 1: "The engine's
complete standard output and exit code are captured *before* any code change, at the commit the work
begins from… The baseline is captured once, from a tracked-clean worktree, with the commit recorded. A
discrepancy is never resolved by recapturing it." Capturing it after the fact, or recapturing it later,
would forfeit the oracle rather than satisfy it.

---

## Read in this order

| # | File | What it establishes |
|---|---|---|
| 1 | `escalation.md` | **the four failing obligations, their one cause, and the measurement of every option** — including the option the owner first selected, which the measurement refutes, the amendment that resolved it, and §11's separate escalation of the identifier band on data that did not exist when §1 was written |
| 2 | `post/runs.md` | whole runs under `social`: 15 trace pairs agreeing on all eleven outcome columns, survivors and deaths by cause per cell, every verb proposed and applied, all 44 rejections, and encounters per seed |
| 3 | `post/branches.md` | rule 26's six branches reconstructed from the released stream with **thirteen checks against the engine, all reading zero** over 118,201 decisions; the answer branch's three choices; strikes per encounter; rules 23 and 24 at their boundaries |
| 4 | `identifier.md` | oracle 5, both halves: the tripwire on the declared five, the turn-position bound on the declared 200, the 1,000-seed sweep behind them, and the identifier exchange |
| 5 | `post/byte-identity.txt` | the two clauses of oracle 1: `baseline` identical on 30 of 30 cells unprojected; the 60-cell divergence absent because `REQ-MOK-060` is unimplemented — `RESULT: MIXED` |
| 6 | `post/capture-state.txt` | both captures' provenance, measured: 90 of 90 and 30 of 30 cells reproduced from a `git archive` of the commit each claims — `RESULT: PASS` |
| 7 | `post/test-census-reconciliation.md` | 226 names into 264, name by name against `master`'s tip after the merge: 225 retained, 39 added, 0 removed, 0 ignored, 0 non-`ok`. §§1 to 8 are the same reconciliation against the branch point's 212 into 250, across three renames and two candidates; §9.4 measures the `SPEC-MOK-004` rule 11 figures this work order owes, and §9.5 reports a defect it found in that rule's `WO-MOK-013` row |
| 8 | `post/reads.md` | `REQ-MOK-059` discharged by enumeration: every rule, source and validation path, and the seven readers of a set with the reason each is outside the obligation |
| 9 | `post/world-rules-unchanged.txt` | `REQ-MOK-060`'s permitted-and-forbidden constraint measured both ways: 21 named regions byte-identical across the two commits, 3 controls measured as changed, and §4 on why the permitted regions are unchanged too |
| 10 | `post/gates.txt` | the four gate commands with their exit codes, and the `allow`-attribute, manifest, target and test-independence enumerations of the same contract section |
| 11 | `post/interface.txt` | the public surface enumerated at both commits, 186 entries, diffed against the amended rule 5's four growth rows — 15 declared, 15 measured — with rule 6's ten prohibited names checked and one wording defect in the spec recorded |
| 12 | `post/observer.md` | rule 11's fifteen rows against the code item for item, the compiler as what makes the mapping total, the five tests that run, and **five reverted one-line mutations** — three that fail, one of them at compile time, and two that pass and so bound what nothing checks |
| 13 | `post/resolution-tables.md` | rules 22, 23 and 24 tabulated over their **whole domains** — 10,201 pairs collapsed onto 21 damage values, 101 fear values, 101 satiety values — with the paper column transcribed from the rules by a reader that never opens `simulation.rs`, the 24 retained test rows and 1,742 released resolutions as the engine's two columns, and **eleven controls**: six in the reader and five in the engine |
| 14 | `entropy.txt` | oracle 2, and the `no entropy draw` constraints of `REQ-MOK-052` to `REQ-MOK-056`, on a quantity no released stream reports: the shared stream's **absolute state**, computed from the placement arithmetic because the suite records only a count, then checked against the engine's own output over **12,780 placed cells** in 90 excerpts. All fifteen declared draw counts reproduced without being given them; the state at all **15** encounter seeds behind the suite's 33 call sites, with the 28 no-draw cases as 4 geometries × 7 verbs; and every draw site in the shipped half enumerated — **9** of them, of which none is reachable by any of the nine resolution, validation and application functions, so the constraint is discharged structurally rather than sampled |
| 15 | `post/delegation.md` | oracle 8's three clauses, held to three different depths, and a quantity the release reports nowhere: the shared stream's **whole-run draw total** under all four sources at matched seeds. The precondition is decoded from rule 12's `fear` pair and censused at **83,657 of 118,201 opportunities** with **0** targeted proposals among them; the two sources part **inside tick 1 at all fifteen matched pairs**, exactly where oracle 8 says they will, so the per-observation comparison is carried to the parting with both proposals and no further. The draw total is recovered from printed regeneration and reported at each run's **last** addition, exact for the whole run in **30 of 60** cells — and checked against two counts arrived at without the stream: `baseline`'s known one-draw-per-opportunity count, recovered with no slack in **6 of the 7** cells that carry a pin, and `post/branches.md`'s branch-6 count, inside the recovered interval **15 of 15**. Read it after item 14, whose §7 is what makes the second control valid |
| 16 | `post/long-horizon.md` | the 10,000-tick clause on all five declared seeds, not one: **two of them go extinct before the limit**, at ticks 7,890 and 9,511, and the five leave 0, 5, 2, 2 and 0 survivors against `REQ-MOK-058`'s ratified floor of five at tick 1,000 — evidence, as the clause says, and put to the product owner. Retained state measured in both halves: three collections reconstructed over the capture's 73,291 ticks, and peak working set for the two no stream shows, flat at a factor of at most `1.0046` across a twenty-fold tick range on three ladders retained whole |
| 17 | `baseline/capture-state.txt` | the baseline capture's provenance, on the same four columns — `RESULT: PASS` |
| 18 | `baseline/pre-manifest.txt` | the 90-cell matrix by per-cell SHA-256, bytes, lines and exit code |
| 19 | `baseline/cross-check.txt` | two free agreements with `WO-MOK-011`'s measurements — 90 of 90 cells, 212 of 212 test names — `RESULT: PASS` |
| 20 | `baseline/census.txt` | the seven targeted verbs and both new `action_trace` fields at **zero occurrences in 110 MB**, which is this change's absence claim measured on the pre-change side |

---

## Every retained file

### Packet root

| File | Contents |
|---|---|
| `README.md` | this index |
| `escalation.md` | the four failing obligations, the cause, the owner's first selected option measured and refuted, the answers of 2026-08-20, the amendment that implemented Package A, and §11's identifier-band escalation with its four measured options and the owner's answer |
| `identifier.md` | oracle 5's deliverable: the three per-identifier series, both rank correlations recorded and bounding nothing, the turn-position bound evaluated at `1.082` against `1.25`, why 200 seeds and not 50, the identifier exchange, and the rule 25 ablation |
| `identifier-sweep.json` | the 1,000-seed sweep behind `identifier.md`, 116 KB, retained so every table in it is re-derivable without a two-minute re-run |
| `entropy.txt` | oracle 2's deliverable: the shared stream's absolute state, derived from the placement arithmetic and the measured rejection threshold of `0`, at all fifteen declared `(seed, density)` pairs and at all fifteen encounter seeds; the replay checked against 12,780 engine-printed cells; the six excerpts of each pair proved identical, which is where the source and the trace setting are shown to move no placement draw; every draw site in the shipped half enumerated with the nine resolution-side functions measured unable to reach the stream; and the residual §8 declines to cover |
| `capture.sh` | the harness that produces one 90-cell capture; `WO-MOK-011`'s script with adapted comments, so the two work orders' captures are comparable without a reader diffing the harness |
| `capture-social.sh` | the 30 `social` cells, in `capture.sh`'s shape unchanged — same loop order, same cell naming, same `.exit` file beside each stream — so the two captures are comparable by the same reader |
| `capture-horizon.sh` | the ten long-horizon cells: the 10,000-tick run on **all five** declared seeds rather than the one the clause's singular phrasing allows, one trace-on cell at the seed that survives longest, and seed 0's tick ladder at 1,000, 2,000, 5,000 and 20,000. Same cell naming with the tick count added and the same `.exit` beside each stream, so no new reader is needed to digest it |

`capture.sh` deliberately does **not** take the 30 cells under `--policy social`. That source does not
exist at the baseline commit, and a fourth loop value would fail 30 runs with a configuration error.
The `social` cells are a separate post-change capture, listed separately in `VER-MOK-016`'s retention,
and `capture-social.sh` is that capture. Its shape is deliberately not improved on: the two scripts
differ in their policy list and in nothing else, so a discrepancy between the two manifests cannot be a
discrepancy between two harnesses.

**`capture-social.sh` invokes `./target/release/Mokiterions`, relative to the working directory**, so a
reproduction must be run with the working directory inside the archived tree. Run from the repository it
silently measures the repository's own binary, which is a working-tree capture and not an archive
reproduction. `post/capture-state.txt` §5 records both, compared, 30 of 30 identical.

`escalation.md` sits at the packet root rather than under `post/` because it is not a measurement of the
candidate: it is the record of what the work order could not discharge as approved, and it cites both
sides. `identifier.md` sits there for the same reason — its sweep is 1,000 seeds and neither capture
holds it. `entropy.txt` sits there because what it measures is the shipped source and the test suite:
its subject is a quantity the streams do not contain, and it reads the captures only to check a
reconstruction, so filing it under `post/` would claim it as a reading of the candidate capture when it
is not one. Every path any of the three names is relative to this directory, as
`post/byte-identity.txt`'s are.

### `baseline/` — the pre-change side, and the shared tooling

| File | Contents |
|---|---|
| `COMMIT.txt` | `39662d13abd08e3410648d1c59ad38384f8ad2d2` — one line, so a script can read it |
| `capture-state.txt` | the provenance, measured rather than asserted: the capture reproduced from a tree that cannot hold a working-tree modification |
| `cross-check.txt` | the 90-cell matrix against `WO-MOK-011/post/post-manifest.txt` and the test census against `WO-MOK-011/merge/test-census.txt`, with what each check does and does not establish |
| `manifest.py` | digests a capture directory into a manifest. Carries **no projection**, and says at length why its absence is the point |
| `pre-manifest.txt` | 90 rows: cell, raw SHA-256, bytes, lines, exit code. Every cell exits `0` |
| `retain.py` | writes the stated subset kept whole, and states the subset and the reason for each piece |
| `init/<cell>.txt` | the initialization block of **every** cell — every line up to and including the last `tick=0 ` line, 90 files, 1.2 MB |
| `summary.txt` | rule 18's final summary line of every cell, one line per cell |
| `census.txt` | per cell: every `event=` kind, every verb proposed, and the two fields this change adds — all zero for the seven targeted verbs and both fields |
| `correction-fight-verb.md` | a defect in this side's own tooling, found and corrected: `retain.py` counted a `verb:observe` that rule 21 does not define and omitted `fight`, which it does. `census.txt` regenerated; the absence claim now covers all seven verbs |
| `full/seed42-baseline-d0.75-traceon.txt` | one cell whole, 414 KB: a *traced* `baseline` stream, where a leaked `target` or `suffered` field would appear first |
| `test-run.txt` | the full `cargo test --locked --workspace` log at the baseline commit: one invocation, workspace root |
| `test-census.txt` | that log as 212 target-qualified names, sorted |

### `post/` — the candidate side

| File | Contents |
|---|---|
| `COMMIT.txt` | `7c4aef3967406c05d80da963695898b77f5329e9` — one line, so a script can read it. It is the 90-cell capture's commit; `capture-state.txt` §5 is where the second candidate is related to it |
| `capture-state.txt` | the provenance of both captures: 90 of 90 cells at `7c4aef3` and 30 of 30 at `59d61b9`, each reproduced from a `git archive` and agreeing with the working-tree capture on all four columns, plus the measurement that the ninety did not move between the two — `RESULT: PASS` |
| `post-manifest.txt` | 90 rows, produced by `baseline/manifest.py` unchanged, from the archive reproduction. Valid at both candidates, measured |
| `social-manifest.txt` | the 30 `social` cells at `59d61b9`, every cell exiting `0`, 45.6 MB of stream. **No longer provisional**: the amendment that moved all thirty has landed and this is the retaken manifest |
| `byte-identity.txt` | oracle 1's two clauses compared and answered separately — `RESULT: MIXED` |
| `runs.md` | whole runs under `social`: trace parity, outcomes and deaths by cause, every verb proposed and applied, rejections by reason, encounters per seed |
| `branches.md` | rule 26's six branches, the answer branch's three choices, strikes per encounter, and rules 23 and 24 at their boundaries |
| `runs.txt` | the reader's own output over the thirty streams, 151 lines, retained whole. Every table in `runs.md` and `branches.md` is transcribed from it |
| `delegation.md` | oracle 8: the precondition decoded from the released bytes and censused over fifteen `social` cells, the parting with both proposals and the shared stream's position either side, the whole-run draw total for all sixty matrix cells, and three controls — `baseline`'s known draw count, `post/branches.md`'s branch-6 count, and the trace setting |
| `delegation.txt` | `analysis/delegation.py`'s own output over 120 cells, 558 lines, retained whole: thirteen sections and `RESULT: PASS`. Every table in `delegation.md` is transcribed from it |
| `resolution-tables.md` | rules 22, 23 and 24 over their whole domains, the retention clause's three columns kept apart, and eleven controls. Carries the two readings the passing run does not show: rule 22's magnitudes are contradicted **only** by the retained rows and its flat cost **only** by the released stream, and the damage values `12` through `16` have a paper column and no engine column at all |
| `resolutions.txt` | the reader's own output, 262 lines, retained whole: the five exhaustive tables, the 24 retained rows with the controls that contradict each, the 1,742-line check and the coverage counts. Every table in `resolution-tables.md` is transcribed from it |
| `long-horizon.md` | the 10,000-tick clause's three demands, each with its own instrument: completion on all five declared seeds with **two termination reasons and not one**, composition and survivor figures with the extinction finding put to the product owner, and retained state measured in two disjoint halves. Also carries rule 13's forfeited turn — the case where a Mokiterion killed by a lower identifier never reaches its own opportunity — measured at nine occurrences, the first evidence in this packet that the specified case occurs at all |
| `horizon.txt` | `analysis/horizon.py`'s own output, 112 lines, retained whole: the ten cells by digest, completion, survivors, deaths by cause and by tick, the three reconstructed collections, the food accounting, the tick ladder, the suffered-attack record and the provenance tie. Every table in `long-horizon.md` §§3 to 11 and §§13 to 15 is transcribed from it |
| `horizon-memory.txt` | `analysis/peak.py`'s four invocations with their exit codes, in `gates.txt`'s command-record shape: the tick ladder's peak working set taken three separate times, the same cell three times within each for the measurement's own noise band, and the instrument's control. The half of the retention clause no stream can answer. **The ladder was retained three times rather than once because the spread it reads is not reproducible** — 16, 8 and 20 KiB from the identical command — and in one of the three, re-running a single cell moves the reading further than twenty-fold-ing the tick count does |
| `test-run.txt` | the `cargo test --locked --workspace --no-fail-fast` log at `7c4aef3`: 249 names, 246 passing, **3 failing**, exit `101` |
| `test-census.txt` | that log as 249 target-qualified names with each outcome, sorted |
| `test-run-amended.txt` | the same invocation after the amendment: 250 names, **250 passing**, exit `0` |
| `test-census-amended.txt` | that log as 250 names, produced by the same reader |
| `test-run-master.txt` | the same invocation at **`master`'s tip** `d8e2079`, the merge's second parent: 226 names, 226 passing, exit `0`. The before side of the reconciliation after the merge of 2026-08-21. Its two `Compiling` lines name a detached worktree, kept rather than scrubbed, because that is the disclosure that this side was measured on a tree this branch never had checked out |
| `test-census-master.txt` | that log as 226 names, produced by the same reader |
| `test-run-merged.txt` | the same invocation on the merge: 264 names, **264 passing**, exit `0` |
| `test-census-merged.txt` | that log as 264 names, produced by the same reader |
| `test-census-reconciliation.md` | row 248 discharged name by name across both candidates and then again across the merge: three renames, one of them proved verbatim by digest, §8 relating the two pre-merge log pairs, and §9 re-taking the whole reconciliation against `master`'s tip at 226 into 264 without editing a figure above it |
| `reads.md` | the `REQ-MOK-059` enumeration and the `fear`-writer enumeration; verdict **met** |
| `world-rules-unchanged.txt` | `REQ-MOK-060`'s permitted-and-forbidden constraint measured both ways: 21 named regions of `simulation.rs` compared line for line against the baseline commit, 3 controls, and §4 on why the two permitted regions are unchanged too |
| `gates.txt` | the four gate commands recorded with their exit codes — `fmt`, `clippy` from a cleaned target directory, `tree`, and `cargo test`'s three logs by reference — with the `allow`-attribute, manifest, target and test-independence enumerations of the same contract section |
| `interface.txt` | the library target's public surface as a list, 186 entries at the candidate, the baseline as the diff from it, and every differing line matched to a row of rule 5's amended growth table: 14 entries added, 2 changed, 0 removed. Carries rule 6's ten prohibited names, the `cli::USAGE` value change the surface diff cannot show, and the measurement that both candidates' surfaces are byte-identical |
| `observer.md` | rule 11's fifteen rows against `for_type`, `EventType::ALL` and the presented table, arm for arm, with the baseline's 15-rows-against-12-types explained as the artifacts-first ordering rather than a defect. **The mapping cannot omit a type because the match has no wildcard**, and control C is the `error[E0004]` that says so. Five one-line mutations, each stated exactly and each reverted: two that fail the named-pair assertion, and two that pass — a variant absent from `ALL`, and `social` dropped from the overlay's hand-written source row — recorded as bounded residuals of clauses this work order satisfies as written. Also carries `ALL`'s order against rule 11's, the three types' emission counts, and the overlay's 20 → 23 lines against the nine renderable viewports |

`--no-fail-fast` is on both candidate invocations and was not on the baseline's, for a reason
`test-census-reconciliation.md` §4 states: without it cargo stops after the first failing target and
eight later targets never run, so a census taken from that log would be missing names and would read as
removals. The plain invocation was run too at `7c4aef3` and exits `101` identically.

**Both test log pairs are kept and neither replaces the other.** `test-census-reconciliation.md` §§1 to 5
reconcile the 249-name census and name its three failures, and `escalation.md` cites those three by name;
overwriting them would leave five sections and an escalation record without their artifact. §8 is the
table of which section reads which.

### `analysis/`

| File | Contents |
|---|---|
| `census.py` | `WO-MOK-011`'s census script, a byte-identical copy (`sha256 7d355454…`), so the two work orders' censuses are read by the same reader |
| `test-census.py` | turns one `cargo test` log into a target-qualified census. Validated against the side it did not write: run over `baseline/test-run.txt` it reproduces the hand-written `baseline/test-census.txt` line for line. **Records each outcome rather than filtering**, so a case that stopped passing cannot be lost by omission |
| `identifier.py` | the 1,000-seed sweep and every table in `identifier.md`. `sweep` writes `identifier-sweep.json`; `tables` reads it back, so the tables are re-derivable in a second and the sweep is run once |
| `regions.py` | the reader behind `post/world-rules-unchanged.txt`. Lifts named regions of `simulation.rs` out by anchor line rather than by line number, so one region table serves both commits, and compares the raw extracted bytes with no normalization. Carries a per-region expectation and **exits non-zero if any region fails it**, controls included |
| `interface.py` | the reader behind `post/interface.txt`. **Discovers the public items rather than listing them**, so the enumeration cannot omit one, and drops bodies, match arms, comments and constant values so that two commits' surfaces can be diffed. Imports its brace counter from `regions.py` rather than copying it, so the two readers cannot disagree about where an item ends, and it stops rather than guessing on a shape it does not handle |
| `resolutions.py` | the reader behind `post/resolution-tables.md`. Builds rules 22, 23 and 24's tables by **enumerating each function's whole domain**, so a boundary case is the first or last row of an interval rather than something to remember; transcribes the four functions from the rules and **never imports, parses or opens `simulation.rs`**, because an oracle that read the implementation would agree with it by construction. Checks every released resolution against the tables, exits non-zero on any disagreement, on a line shape it does not recognize, or on a capture holding no resolution at all, and carries `--control` for each of the six terms |
| `horizon.py` | the reader behind `post/horizon.txt` and `post/long-horizon.md`. Reconstructs the three stream-visible retained collections **tick by tick** rather than at the end, because a collection that grew and was dropped at shutdown looks identical at the end to one that never grew; checks the engine's own at-capacity count against its independently maintained one 5,333 times; reads the population a second way from every living Mokiterion's per-tick line and a third from the trace-on cell's `action_trace` count; and **never imports, parses or opens `simulation.rs`**. Exits non-zero on any disagreement or on a line shape it does not recognize, and carries `--control` for each of nine terms |
| `peak.py` | the reader behind `post/horizon-memory.txt`, and the only instrument here that reaches the two retained collections no event reports. Reads `PeakWorkingSetSize` from `GetProcessMemoryInfo` on the child's handle after it exits — a ceiling over the whole run rather than a sample. **The criterion is a ratio and not a limit**, because the absolute figure is one machine's; `--control` runs a child holding a known 16 then 64 MiB, so a flat column is shown to be a reading rather than a broken instrument. Windows only |
| `entropy.py` | the reader behind `entropy.txt`. **Reimplements `SplitMix64` rather than calling it**, so the state it reports is an independent computation and not the engine's own arithmetic echoed back; derives the rejection threshold rather than assuming one value per index, and derives all fifteen recorded draw counts rather than reading them, comparing against the transcribed figures afterwards. Enumerates every draw site and classifies every function in the shipped half by extent, scanning bodies with comments excluded — `resolve_strike`'s doc comment claims the very thing under test. Every claim in the output is a check, and the script **exits non-zero if any of them fires**. Writes LF regardless of platform, so a plain redirect reproduces the retained file byte for byte |
| `runs.py` | the reader behind `post/runs.md` and `post/branches.md`. Replays the released stream to classify all six branches of rule 26 — branch 3 and branch 6 both render as `move:<direction>`, so they cannot be counted off the trace lines — and **exits non-zero if any of its thirteen checks fires**. It is the one script here that can fail |
| `delegation.py` | the reader behind `post/delegation.md`. Decodes oracle 8's precondition from rule 12's `fear` pair rather than from a list no stream prints, and **recovers the shared stream's whole-run position from printed regeneration**: every `food_regenerated` line names the class and the coordinate it settled on, which pins the accounting, and the opportunities between two pins bound what the decision source can have spent in between. Reports each run's **last** addition and never an earlier one, so the remaining opportunities are a bound rather than an estimate, and carries every position a coincidence leaves standing rather than picking one. Reimplements `SplitMix64` and opens no engine source, and **exits non-zero if any of its 30 checks fires** — including the one that fires when no candidate position survives an addition, which is how a lost stream is detected instead of quietly mis-counted |

`runs.py` is a reader and not an instrument, and that is deliberate. The obvious way to count branches is
a temporary build that prints the branch it took, and a figure produced by a build that no longer exists
is not reproducible from retained bytes. So the branch is reconstructed from the stream every reviewer
has, and checked against the engine's own choice thirteen ways. `post/branches.md` §1 is the check table.

`delegation.py` is the second reader of that kind, and its subject is worse served by a temporary build
still: a stream position printed by a build nobody has is not evidence of anything. What makes its
recovery evidence instead of arithmetic is that two quantities computed **without** the stream — rule 4's
one draw per opportunity, and `post/branches.md`'s branch-6 count — land on the numbers it recovers. The
agreement runs both ways, and `post/delegation.md` §8 records it in both directions.

---

## The matrix

Five declared seeds of `VER-MOK-002` (`0`, `1`, `42`, `123`, `777`) × the three decision sources that
exist before this change (`baseline`, `reference`, `individual`) × the default density and the two swept
densities (`0.15`, `0.75`, `1.50`) × with and without `--trace-actions`, at 1,000 ticks. 90 cells, of
which 30 are `baseline`. 110 MB of stream, captured in 4.8 seconds.

The 110 MB is not committed. `VER-MOK-016` retains this capture "by per-cell SHA-256 digest, with a
stated subset retained whole"; `pre-manifest.txt` is the digests and `retain.py`'s docstring is the
stated subset. Both scratch captures — the original and the `git archive` reproduction — were written
outside the repository and deleted after measurement.

The same matrix is taken again at the candidate commit, giving `post/post-manifest.txt`, and the fourth
source adds **30 cells more**: the same five seeds × `social` × the same three densities × trace off and
on, 45.6 MB. That is 120 cells across the two manifests, and the two are kept apart deliberately —
oracle 1 asks whether the first 90 moved, which is a question the thirty new ones cannot answer and must
not dilute. The separation earned itself: the amendment moved every one of the thirty and none of the
ninety, so `social-manifest.txt` was retaken and `post-manifest.txt` was not.

One note for a re-derivation, because it costs a run to rediscover: `baseline/manifest.py` asserts its
own matrix size, `return 0 if len(cells) == 90 and exits == {'0'} else 1`, so it exits `1` on the 30-cell
`social` capture. That is the reader checking that it read a whole 90-cell matrix, not a finding about
the capture, and the 30 rows it writes are correct. The script is reused unchanged rather than
generalized, so that both manifests are produced by one reader.

**A third capture sits outside the matrix**: `capture-horizon.sh`'s ten long-horizon cells, 64,273,153
bytes, taken at `1ae4267` and not committed on the same terms as the other two. It is not part of the
matrix because it varies the one axis the matrix holds fixed — the tick count — on one source and one
density, and folding it in would make 120 cells read as 130 of something else. It has **no manifest file
of its own**: the same assertion above would have recorded a spurious failure on ten cells, and rather
than generalize the reader or add a second one, the digest table is `analysis/horizon.py`'s own §1, which
is retained whole in `post/horizon.txt`. One retained output rather than two, and the digests are still
the first thing a re-derivation compares.

---

## What is owed

| `VER-MOK-016` retains | State |
|---|---|
| the pre-change 90-cell capture, with the commit recorded | **held** — `baseline/` |
| the pre-change workspace test census | **held** — `baseline/test-census.txt` |
| the post-change 90-cell capture, with `baseline` compared byte for byte and the other 60 cells' divergence characterized | **held, and one clause is unwritable** — `post/post-manifest.txt` and `post/byte-identity.txt`. `baseline` is identical on 30 of 30, unprojected. The 60-cell divergence **does not exist**, because `REQ-MOK-060` is unimplemented under the approved deferral, so there is nothing to characterize and the requirement is recorded as outstanding in `escalation.md` |
| the new 30-cell capture under the `social` source, with exit codes | **held** — `capture-social.sh` and `post/social-manifest.txt`, 30 cells at `59d61b9`, every cell exiting `0`. The provisional label is withdrawn: the amendment that moved all thirty has landed |
| the captured exit code of every cell, both sides | **held** — a column of every manifest; every one of the 210 cells exits `0` |
| the constructed-state resolution tables for damage, energy cost, threat increase and forfeit, with every boundary case | **held** — `post/resolution-tables.md`, from `analysis/resolutions.py` at exit `0`: 24 of 24 retained rows and 1,742 of 1,742 released resolutions agree with the tables. The clause's three columns have three sources and are kept apart: the paper column is transcribed from rules 22, 23 and 24 by a reader that never opens `simulation.rs`, the constructed-state column is the three retained tests' own literals, and the released column is every resolution line of the thirty `social` cells. "Every boundary case" is discharged by **enumeration rather than selection** — each function's domain is bounded in `0..=100`, so the tables are the whole domain collapsed onto its 21, 6, 32 and 51 distinct outputs, and a boundary is the first or last row of an interval. Eleven controls: the reader's six perturb one term each and their **split is the finding** — rule 22's base and divisor are contradicted by 9 and 6 retained rows and by **none** of the 1,742 released lines, while its flat `5` is contradicted by **no** retained row and by all 312 attack lines, so the two halves of the engine column cover disjoint parts of one rule. The engine's five each mutate one line and were reverted; two of them measure that `STRIKE_BASE_DAMAGE` is guarded asymmetrically by a `debug_assert!` that moves with it, and that the suite's true oracle coverage of it is 7 tests and not the 23 the upward mutation appears to show |
| the shared stream's recorded state either side of every resolution kind, and the reused `VER-MOK-007` initialization expectations | **held** — `entropy.txt`, from `analysis/entropy.py` at exit `0`, eight sections and every check reading clean. The oracle asks for a **state** and `INITIALIZATION_DRAWS` holds a **count**, and no released stream reports either, so the state is computed rather than transcribed: `SplitMix64` advances by one fixed increment per value, so the absolute position is `seed + draws × γ`, and `choose_index`'s rejection threshold is measured at **`0`** at both bounds in play — `128` and `64` both divide 2^64 — which is what makes "two values per coordinate attempt" exact rather than typical. On that footing the reader replays `Simulation::new` and reproduces **all fifteen** recorded draw counts without being given them, adding two figures the suite does not record: the coordinate attempts and the **occupancy rejections** behind each count, which is why 270 and 514 sit off the no-rejection floor. What keeps this from measuring only itself is §4: every reconstructed cell is compared against the engine's own event stream over all **90** retained excerpts, **12,780** cells, 0 differing. §5 then uses those excerpts for a claim asserted nowhere else in the packet — that neither the decision source nor the trace setting moves a placement draw — by requiring the six files of each `(seed, density)` pair to agree cell for cell, 15 of 15. §6 gives the state at which every resolution in the suite is asserted to leave the stream unmoved, at **all 15** encounter seeds behind its 33 call sites rather than only the one the test under oracle 2 uses, with the 28 cases named as 4 geometries × 7 verbs; the table is checked against itself, since seeds sharing a draw count must differ in state by exactly their seed difference and 63 cross pairs must not. Two controls: a bound that *would* reject, and the single extra draw that moves the state by γ — which is what makes the state the quantity that catches a draw taken and restored, and the count the one that does not. §7 discharges the `no entropy draw` constraints of `REQ-MOK-052` to `REQ-MOK-056` **structurally rather than by sampling**: all **9** draw sites in the shipped half are enumerated against an expected set, and each of the nine resolution, validation and application functions is measured to mention the stream nowhere outside a comment — including `resolve_strike`, whose own doc comment claims it, which is the thing being checked and not the check. The same enumeration makes `post/branches.md`'s branch-6 count of **37,975** the `social` source's entire draw consumption, because every one of branches 1 to 5 returns before `entropy` is reached |
| per-seed tables of survivors, deaths by cause, encounters, each verb proposed and applied, and rejections by reason — on failing seeds too | **held** — `post/runs.md`, from `post/runs.txt`. The four extinction cells are recorded, not excluded |
| the per-observation comparison of `social` against `individual` where no living Mokiterion is perceived and no attack is unanswered | **held** — `post/delegation.md`, from `analysis/delegation.py` at exit `0` over **120 cells**, every one digest-matched to its manifest row. The clause has three parts and they are held to three different depths, which the record states rather than averages. The **precondition** turns out to be stream-visible exactly, though neither half looks it: rule 12's `fear` pair decodes `perceived_company` from the same observation the decision was taken on, because `run_tick` carries it past the decision and the two saturations of the update are at opposite ends, and rule 17 renders `suffered` only when the record is non-empty, so its absence *is* the empty record. It holds at **83,657 of 118,201 opportunities** — 70.8%, so the equality is asserted over most of the matrix — with **0** targeted proposals among them and the proposals there rule 19's vocabulary and nothing else. The decode is checked against another reader's number rather than only against itself: `post/branches.md`'s branch-1 count, reconstructed from positions, traits and geometry, agrees cell for cell, **15 of 15 and 135 of 135**. The **per-observation comparison** reaches the parting and no further, because the two sources part **inside tick 1 at all fifteen matched pairs** and past it there is no shared observation left to consult them on. Both proposals are carried at every parting — `approach` 15 of 15 against a plain move 15 of 15, which is exactly the shape oracle 8 predicts and is asserted nowhere else in the packet — and the stream's position either side is exact in 6 of 15 and a 2-to-10-candidate window in the rest, with the reason it cannot be narrower stated. The **draw total per run** is reported by nothing in the release, so it is recovered from printed regeneration and reported at each run's **last** addition, which is what makes the remaining opportunities a bound: **30 of 60 cells exact for the whole run**, 43 of 60 with an exact decision/regeneration split, **1** with an open total, **0** where the stream was lost, and the 8 cells that reach no addition at all named with the extinction tick that explains them. Two controls from outside the accounting: rule 4 makes `baseline`'s decision count its opportunity count, and the solver — not told it — recovers it with no slack in **6 of the 7** cells that carry a pin and *selects* the true hypothesis in the seventh; and `post/branches.md`'s branch-6 count, which `entropy.txt` §7 makes the `social` source's entire consumption, falls inside the recovered interval **15 of 15**, twice as one number against one number with no interval at all. That closes every `social` range this reader leaves open, and corroborates `post/branches.md`'s branch-3/branch-6 split from the entropy side, which is the one part of that classification no rendered field distinguishes. Oracle 8's stated consequence — that a `social` run takes fewer draws than an `individual` run at the same seed — holds **15 of 15 strictly, 0 undecided, 0 contradicted**, the five sparse pairs on disjoint intervals |
| the branch distribution under `social` per seed, including the answer branch's three choices | **held** — `post/branches.md` §§2 and 3: all six branches per cell, 118,201 decisions, 135 answers split 13 `fight` / 27 `retreat` / 95 `surrender`. This is the evidence manual assessment 8 is taken on |
| the measured strikes per encounter, forfeits discarded at a full recipient, and surrenders below `satiety` `2` | **held** — `post/branches.md` §§4 to 6, and two of the three are measured as **not reached in whole runs**: no recipient was at `satiety` `100` and no surrender was below `2`, while 90 of 95 surrenders discard part of the forfeit. This is what manual assessments 1 and 2 are taken on |
| the per-identifier series, rank correlations and band evaluation | **held** — `identifier.md` and `identifier-sweep.json`, as oracle 5 was amended: the tripwire on the declared five, the turn-position bound at `1.082` against `1.25` on the declared 200, and both correlations recorded and bounding nothing |
| the identifier-exchange comparison for the constructed encounter | **held** — `identifier.md` §6 |
| rule 18's final summary per seed under each of the four sources, at both commits | **half held** — three sources at the baseline commit, in `baseline/summary.txt`. The `social` side and the composition ratio are owed — `composition.md` |
| the line-for-line comparison showing rule 4, rule 9's eat effect, the food table and rules 14 to 16 unchanged | **held** — `post/world-rules-unchanged.txt`, from `analysis/regions.py` at exit `0`: 21 named regions byte-identical across the two commits and 3 controls measured as changed, so the run distinguishes a region that moved intact from one the extractor failed to track. The thirteen forbidden regions are unchanged because the constraint holds; the eight permitted ones are unchanged because `REQ-MOK-060` is unimplemented, and §4 says which is which rather than reporting one clean result |
| the enumeration of reads per rule, source and validation path | **held** — `post/reads.md` §§2, 4 and 6, with §5's seven readers of a set and the reason each is outside the obligation. Verdict: `REQ-MOK-059` met |
| the enumeration of `fear`'s writers and of every path writing a second Mokiterion's state | **held** — `post/reads.md` §7: two `fear` writers, five paths writing a second Mokiterion, three functions |
| the engine public-interface enumeration at both commits, against the approved `SPEC-MOK-002` amendment | **held** — `post/interface.txt`, from `analysis/interface.py` at exit `0`: 186 surface entries at the candidate, 172 at the baseline, 18 differing lines of which 2 are the reader's own headers. The 16 surface differences are 14 added and 2 changed in place, and they match the amendment's **four** growth rows at 15 declared units against 15 measured. Four rows and not three: the amendment gained one on 2026-08-20 for the `suffered` field on the already-public `ActionTrace` payload, and §4 measures why no reader that counts added items could have found it — 14 of the 15 units are new entries, the fifteenth is a field inside an entry that already existed. Rule 6's ten prohibited names are checked in §6, and §7 records a wording defect in rule 5 that this measurement found |
| the observer authority table's new rows and the `EventType::ALL` exhaustiveness check | **held** — `post/observer.md`: rule 11's fifteen rows against `for_type`, `EventType::ALL` and the presented table, and the five tests that cover them. The exhaustiveness half is measured in three directions rather than asserted in one. Every variant maps to an identifier because `for_type`'s match has **no wildcard** — control C is the `error[E0004]` a sixteenth variant produces, so the clause is compiler-enforced. A variant in `ALL` with no row in the named-pair table fails the new `expected.len() + 1 == ALL.len()` assertion, which is the contract clause's own sentence: control A measures it at `left: 14 right: 15`, and control B measures that a well-formed *wrong* identifier fails too, at `Some("REQ-MOK-056")` against `Some("REQ-MOK-055")`. Both controls also record which test did **not** fail. Two directions pass and are recorded as bounded residuals, not as contract failures: a variant absent from `ALL` (250 tests pass, clippy clean), and `social` dropped from the overlay's hand-written source row (128 pass, clippy clean). §8 measures that the three types are emitted unconditionally — 156, 620 and 95 over the traced cells — so "every type the observer can present" is not vacuous, and §9 records the overlay at 23 lines against a 16-line interior at the floor viewport |
| the post-change test census, reconciled name by name | **held** — three log pairs and one reconciliation in nine sections: 249 names with 3 failures at `7c4aef3`, 250 names all passing after the amendment, and 264 on the merge of 2026-08-21, reconciled both against the branch point's 212 (211 retained, 39 added) and against `master`'s tip's 226 (225 retained, 39 added), with 0 removed, 0 ignored and 0 non-`ok` in every comparison and three renames accounted for. §9.4 measures the `SPEC-MOK-004` rule 11 figures the additions oblige — observer **142**, engine **122**, workspace **264** — and states plainly that writing them is the technical owner's act and not this packet's |
| `cargo fmt`, `cargo clippy`, `cargo test`, `cargo tree -p Mokiterions` | **held** — `post/gates.txt`, all four at exit `0`, clippy run from a cleaned target directory so that its exit code is a statement about the code and not about the cache. `cargo test` is retained whole at three commits and referenced rather than restated. The same file carries the section's adjacent enumerations: three `allow` attributes, all pre-existing and all in the observer; the four manifests byte-identical to the baseline's; fifteen test targets unchanged; and zero `#[ignore]`, feature gate, environment read, spawned process or path reference in any test |
| the 10,000-tick run's completion, composition and survivor figures, as evidence and not as an obligation | **held** — `post/long-horizon.md`, from `analysis/horizon.py` at exit `0` over 561,958 lines and `analysis/peak.py` at exit `0`. The clause's singular "a 10,000-tick run" is taken on **all five declared seeds**, and that is what makes it a measurement: **two of the five go extinct before the limit**, at ticks 7,890 and 9,511, so a single-seed capture would have shown one of the two termination reasons and read as the whole picture. All ten cells exit `0` and none panics. The survivor figures are 0, 5, 2, 2 and 0 against `REQ-MOK-058`'s floor of five ratified on the 1,000-tick curve, where the same seeds leave 9, 10, 9, 9 and 11 — **evidence and not a breach**, in the clause's own words, and raised for the product owner rather than resolved here. "Without unbounded growth in retained state" is measured in two disjoint halves, because five things are retained and only three appear in a stream: the population, the food collection and the suffered-attack record are reconstructed tick by tick over 73,291 ticks, with the engine's own at-capacity count checked against the reader's 5,333 times and neither territory ever exceeding 61; the decision snapshots and the absent event vector are reachable only through process memory, where peak working set is flat over a **twenty-fold** tick range against a 5% tolerance. That ladder is retained three times rather than once, at factors of `1.0037`, `1.0019` and `1.0046`, because the spread it reads is not reproducible from one invocation to the next — 16, 8 and 20 KiB from the identical command — and a figure that moves when nothing moved belongs to the instrument. Across all three the 20,000-tick run never peaks above the 10,000-tick one, and in one of them merely re-running a single cell moves the reading further than twenty-fold-ing the tick count does. Ten controls: nine in `horizon.py` and the memory instrument's own, which is shown to move 49,188 KiB for 48 MiB held so that a flat column is a finding. The 1,000-tick rung reproduces the released `social` cell by digest, which is what ties every figure here to the ratified ones |
| the eleven manual assessments, each with its accountable role and date | **owed, and not the implementation's to write** — `manual-assessment.md` will hold the prepared records and the measured evidence each assessment is taken against; the assessments themselves are the owner's acts |
| the amendment-approval check of oracle 7, with the recorded state of the `VREC-MOK-005` gate | **owed** — `amendment-approvals.md` |

**Three items remain: two wholly unwritten and one held in part.** Nothing is held back any more.
The direction of 2026-08-20 — that the decision-dependent items wait until the `REQ-MOK-057` amendment
was measured, since the amendment moved all of them — has been discharged: the amendment landed, the
thirty `social` cells were retaken at it, and the four items that were waiting on it are `post/runs.md`,
`post/branches.md`, `identifier.md` and the retaken `post/social-manifest.txt`. What remains is unwritten
for no reason but order of work, with one exception recorded above: the eleven manual assessments are
the accountable owner's acts, and this packet can prepare their records and their evidence but cannot
take them.

`escalation.md` is not on this table because `VER-MOK-016`'s retention does not list it. It is required
by `WO-MOK-016`'s stop-and-escalate conditions, which oblige a recorded escalation rather than an
adjustment, and it is retained here because the same conditions forbid the alternative. `identifier.md`
**is** on the table, twice: `WO-MOK-016` names it as a deliverable and `VER-MOK-016` retains what it
holds.

Nothing here is a verification verdict. `VER-MOK-016` is the contract, `VREC-MOK-016` will be the
record, and neither is written by the implementation.
