# `WO-MOK-012` evidence packet — index

| Field | Value |
|---|---|
| Work order | `WO-MOK-012` (Phase 3.1, encounters) |
| Verification contract | `VER-MOK-012` |
| Implements | `REQ-MOK-042` … `REQ-MOK-051` |
| Baseline (pre-change) commit | `39662d13abd08e3410648d1c59ad38384f8ad2d2` |
| Candidate commits | **two, and the difference matters.** `7c4aef3967406c05d80da963695898b77f5329e9` — the 90-cell three-source matrix and the first test log. `59d61b915630fd55f04bcdbb346aa22cdbfdfff6` — the 30 `social` cells and the amended suite, after the `REQ-MOK-048` amendment. `post/COMMIT.txt` holds the first; `post/capture-state.txt` §5 relates the two and measures the ninety cells as **unchanged** at the second, which is why they were not retaken |
| Branch | `feature/phase-3-definition` |
| Date opened | 2026-08-20 |
| Packet size | 127 files, 2,245,184 bytes |

**This packet is incomplete, and one requirement it measures is deliberately unimplemented.** Both are
stated here so that neither can be mistaken for anything else:

- **`REQ-MOK-051` is not implemented**, under the product owner's approved deferral of 2026-08-20. It is
  what keeps `WO-MOK-012` out of `implemented`, it is why `post/byte-identity.txt` reads
  `RESULT: MIXED` — the 60-cell divergence that oracle 1 asks to be characterized does not exist to be
  characterized — and it is recorded rather than worked around.
- **Ten of `VER-MOK-012`'s retention items are not yet written**, and the table at the end of this
  file marks every one. Nothing is held back any more: the direction of 2026-08-20 that held the
  decision-dependent items until the `REQ-MOK-048` amendment was measured has been discharged — the
  amendment landed, and `post/runs.md`, `post/branches.md` and `identifier.md` are those items. What
  remains is simply unwritten, except the eleven manual assessments, which are the owner's acts and not
  the implementation's.

**The suite passes.** 250 names, 250 passing, 0 failing, 0 ignored, exit `0`, reconciled name by name
against the baseline's 212 in `post/test-census-reconciliation.md`. It did not always: three
requirement-bearing oracles failed at `7c4aef3`, and `escalation.md` is the record of that failure, of
the seventeen variants measured across three levers, of the four owner decisions taken on it, and of the
amendment that resolved it. **Read it first.** A packet whose suite passes today, on a source whose
ordering was measured to be unsatisfiable as first approved, is not self-explanatory without it.

The reason the pre-change side exists as its own commit is `VER-MOK-012` oracle 1: "The engine's
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
| 5 | `post/byte-identity.txt` | the two clauses of oracle 1: `baseline` identical on 30 of 30 cells unprojected; the 60-cell divergence absent because `REQ-MOK-051` is unimplemented — `RESULT: MIXED` |
| 6 | `post/capture-state.txt` | both captures' provenance, measured: 90 of 90 and 30 of 30 cells reproduced from a `git archive` of the commit each claims — `RESULT: PASS` |
| 7 | `post/test-census-reconciliation.md` | 212 names into 250, name by name, across three renames and two candidates: 211 retained, 39 added, 0 removed, 0 ignored, 0 non-`ok` |
| 8 | `post/reads.md` | `REQ-MOK-050` discharged by enumeration: every rule, source and validation path, and the seven readers of a set with the reason each is outside the obligation |
| 9 | `baseline/capture-state.txt` | the baseline capture's provenance, on the same four columns — `RESULT: PASS` |
| 10 | `baseline/pre-manifest.txt` | the 90-cell matrix by per-cell SHA-256, bytes, lines and exit code |
| 11 | `baseline/cross-check.txt` | two free agreements with `WO-MOK-011`'s measurements — 90 of 90 cells, 212 of 212 test names — `RESULT: PASS` |
| 12 | `baseline/census.txt` | the seven targeted verbs and both new `action_trace` fields at **zero occurrences in 110 MB**, which is this change's absence claim measured on the pre-change side |

---

## Every retained file

### Packet root

| File | Contents |
|---|---|
| `README.md` | this index |
| `escalation.md` | the four failing obligations, the cause, the owner's first selected option measured and refuted, the answers of 2026-08-20, the amendment that implemented Package A, and §11's identifier-band escalation with its four measured options and the owner's answer |
| `identifier.md` | oracle 5's deliverable: the three per-identifier series, both rank correlations recorded and bounding nothing, the turn-position bound evaluated at `1.082` against `1.25`, why 200 seeds and not 50, the identifier exchange, and the rule 25 ablation |
| `identifier-sweep.json` | the 1,000-seed sweep behind `identifier.md`, 116 KB, retained so every table in it is re-derivable without a two-minute re-run |
| `capture.sh` | the harness that produces one 90-cell capture; `WO-MOK-011`'s script with adapted comments, so the two work orders' captures are comparable without a reader diffing the harness |
| `capture-social.sh` | the 30 `social` cells, in `capture.sh`'s shape unchanged — same loop order, same cell naming, same `.exit` file beside each stream — so the two captures are comparable by the same reader |

`capture.sh` deliberately does **not** take the 30 cells under `--policy social`. That source does not
exist at the baseline commit, and a fourth loop value would fail 30 runs with a configuration error.
The `social` cells are a separate post-change capture, listed separately in `VER-MOK-012`'s retention,
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
holds it. Every path either names is relative to this directory, as `post/byte-identity.txt`'s are.

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
| `test-run.txt` | the `cargo test --locked --workspace --no-fail-fast` log at `7c4aef3`: 249 names, 246 passing, **3 failing**, exit `101` |
| `test-census.txt` | that log as 249 target-qualified names with each outcome, sorted |
| `test-run-amended.txt` | the same invocation after the amendment: 250 names, **250 passing**, exit `0` |
| `test-census-amended.txt` | that log as 250 names, produced by the same reader |
| `test-census-reconciliation.md` | row 248 discharged name by name across both candidates: three renames, one of them proved verbatim by digest, and §8 relating the two log pairs |
| `reads.md` | the `REQ-MOK-050` enumeration and the `fear`-writer enumeration; verdict **met** |
| `gates.txt` | the four gate commands recorded with their exit codes — `fmt`, `clippy` from a cleaned target directory, `tree`, and `cargo test`'s three logs by reference — with the `allow`-attribute, manifest, target and test-independence enumerations of the same contract section |

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
| `runs.py` | the reader behind `post/runs.md` and `post/branches.md`. Replays the released stream to classify all six branches of rule 26 — branch 3 and branch 6 both render as `move:<direction>`, so they cannot be counted off the trace lines — and **exits non-zero if any of its thirteen checks fires**. It is the one script here that can fail |

`runs.py` is a reader and not an instrument, and that is deliberate. The obvious way to count branches is
a temporary build that prints the branch it took, and a figure produced by a build that no longer exists
is not reproducible from retained bytes. So the branch is reconstructed from the stream every reviewer
has, and checked against the engine's own choice thirteen ways. `post/branches.md` §1 is the check table.

---

## The matrix

Five declared seeds of `VER-MOK-002` (`0`, `1`, `42`, `123`, `777`) × the three decision sources that
exist before this change (`baseline`, `reference`, `individual`) × the default density and the two swept
densities (`0.15`, `0.75`, `1.50`) × with and without `--trace-actions`, at 1,000 ticks. 90 cells, of
which 30 are `baseline`. 110 MB of stream, captured in 4.8 seconds.

The 110 MB is not committed. `VER-MOK-012` retains this capture "by per-cell SHA-256 digest, with a
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

---

## What is owed

| `VER-MOK-012` retains | State |
|---|---|
| the pre-change 90-cell capture, with the commit recorded | **held** — `baseline/` |
| the pre-change workspace test census | **held** — `baseline/test-census.txt` |
| the post-change 90-cell capture, with `baseline` compared byte for byte and the other 60 cells' divergence characterized | **held, and one clause is unwritable** — `post/post-manifest.txt` and `post/byte-identity.txt`. `baseline` is identical on 30 of 30, unprojected. The 60-cell divergence **does not exist**, because `REQ-MOK-051` is unimplemented under the approved deferral, so there is nothing to characterize and the requirement is recorded as outstanding in `escalation.md` |
| the new 30-cell capture under the `social` source, with exit codes | **held** — `capture-social.sh` and `post/social-manifest.txt`, 30 cells at `59d61b9`, every cell exiting `0`. The provisional label is withdrawn: the amendment that moved all thirty has landed |
| the captured exit code of every cell, both sides | **held** — a column of every manifest; every one of the 210 cells exits `0` |
| the constructed-state resolution tables for damage, energy cost, threat increase and forfeit, with every boundary case | **owed** — `resolution-tables.md` |
| the shared stream's recorded state either side of every resolution kind | **owed** — `entropy.txt` |
| per-seed tables of survivors, deaths by cause, encounters, each verb proposed and applied, and rejections by reason — on failing seeds too | **held** — `post/runs.md`, from `post/runs.txt`. The four extinction cells are recorded, not excluded |
| the per-observation comparison of `social` against `individual` where no living Mokiterion is perceived and no attack is unanswered | **owed** — `delegation.md` |
| the branch distribution under `social` per seed, including the answer branch's three choices | **held** — `post/branches.md` §§2 and 3: all six branches per cell, 118,201 decisions, 135 answers split 13 `fight` / 27 `retreat` / 95 `surrender`. This is the evidence manual assessment 8 is taken on |
| the measured strikes per encounter, forfeits discarded at a full recipient, and surrenders below `satiety` `2` | **held** — `post/branches.md` §§4 to 6, and two of the three are measured as **not reached in whole runs**: no recipient was at `satiety` `100` and no surrender was below `2`, while 90 of 95 surrenders discard part of the forfeit. This is what manual assessments 1 and 2 are taken on |
| the per-identifier series, rank correlations and band evaluation | **held** — `identifier.md` and `identifier-sweep.json`, as oracle 5 was amended: the tripwire on the declared five, the turn-position bound at `1.082` against `1.25` on the declared 200, and both correlations recorded and bounding nothing |
| the identifier-exchange comparison for the constructed encounter | **held** — `identifier.md` §6 |
| rule 18's final summary per seed under each of the four sources, at both commits | **half held** — three sources at the baseline commit, in `baseline/summary.txt`. The `social` side and the composition ratio are owed — `composition.md` |
| the line-for-line comparison showing rule 4, rule 9's eat effect, the food table and rules 14 to 16 unchanged | **owed** — `world-rules-unchanged.txt` |
| the enumeration of reads per rule, source and validation path | **held** — `post/reads.md` §§2, 4 and 6, with §5's seven readers of a set and the reason each is outside the obligation. Verdict: `REQ-MOK-050` met |
| the enumeration of `fear`'s writers and of every path writing a second Mokiterion's state | **held** — `post/reads.md` §7: two `fear` writers, five paths writing a second Mokiterion, three functions |
| the engine public-interface enumeration at both commits, against the approved `SPEC-MOK-002` amendment | **owed** — `interface.txt`, and it is checked against **four** growth rows, not three: that amendment gained a row on 2026-08-20 for the `suffered` field on the existing public `ActionTrace` payload, which its first three rows had omitted |
| the observer authority table's new rows and the `EventType::ALL` exhaustiveness check | **owed** — `observer.md` |
| the post-change test census, reconciled name by name | **held** — two log pairs and one reconciliation: 249 names with 3 failures at `7c4aef3`, 250 names all passing after the amendment, 211 retained from the baseline's 212, 39 added, 0 removed, 0 ignored, 0 non-`ok`, across three renames |
| `cargo fmt`, `cargo clippy`, `cargo test`, `cargo tree -p Mokiterions` | **held** — `post/gates.txt`, all four at exit `0`, clippy run from a cleaned target directory so that its exit code is a statement about the code and not about the cache. `cargo test` is retained whole at three commits and referenced rather than restated. The same file carries the section's adjacent enumerations: three `allow` attributes, all pre-existing and all in the observer; the four manifests byte-identical to the baseline's; fifteen test targets unchanged; and zero `#[ignore]`, feature gate, environment read, spawned process or path reference in any test |
| the 10,000-tick run's completion, composition and survivor figures | **owed** — `long-horizon.md` |
| the eleven manual assessments, each with its accountable role and date | **owed, and not the implementation's to write** — `manual-assessment.md` will hold the prepared records and the measured evidence each assessment is taken against; the assessments themselves are the owner's acts |
| the amendment-approval check of oracle 7, with the recorded state of the `VREC-MOK-005` gate | **owed** — `amendment-approvals.md` |

**Ten items remain: nine wholly unwritten and one held in part.** Nothing is held back any more.
The direction of 2026-08-20 — that the decision-dependent items wait until the `REQ-MOK-048` amendment
was measured, since the amendment moved all of them — has been discharged: the amendment landed, the
thirty `social` cells were retaken at it, and the four items that were waiting on it are `post/runs.md`,
`post/branches.md`, `identifier.md` and the retaken `post/social-manifest.txt`. What remains is unwritten
for no reason but order of work, with one exception recorded above: the eleven manual assessments are
the accountable owner's acts, and this packet can prepare their records and their evidence but cannot
take them.

`escalation.md` is not on this table because `VER-MOK-012`'s retention does not list it. It is required
by `WO-MOK-012`'s stop-and-escalate conditions, which oblige a recorded escalation rather than an
adjustment, and it is retained here because the same conditions forbid the alternative. `identifier.md`
**is** on the table, twice: `WO-MOK-012` names it as a deliverable and `VER-MOK-012` retains what it
holds.

Nothing here is a verification verdict. `VER-MOK-012` is the contract, `VREC-MOK-012` will be the
record, and neither is written by the implementation.
