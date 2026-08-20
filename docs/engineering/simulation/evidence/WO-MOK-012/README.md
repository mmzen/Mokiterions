# `WO-MOK-012` evidence packet — index

| Field | Value |
|---|---|
| Work order | `WO-MOK-012` (Phase 3.1, encounters) |
| Verification contract | `VER-MOK-012` |
| Implements | `REQ-MOK-042` … `REQ-MOK-051` |
| Baseline (pre-change) commit | `39662d13abd08e3410648d1c59ad38384f8ad2d2` |
| Branch | `feature/phase-3-definition` |
| Date opened | 2026-08-20 |
| Packet size | 1.9 MB, 103 files |

**This packet is incomplete by design.** It holds the pre-change side only, taken at the moment
implementation began and before any line of the change was written. `VER-MOK-012`'s *Evidence
retention* lists about twenty further items; every one of them is owed, and the table at the end of
this file names them so that an incomplete packet cannot be mistaken for a complete one.

The reason the pre-change side exists as its own commit is `VER-MOK-012` oracle 1: "The engine's
complete standard output and exit code are captured *before* any code change, at the commit the work
begins from… The baseline is captured once, from a tracked-clean worktree, with the commit recorded. A
discrepancy is never resolved by recapturing it." Capturing it after the fact, or recapturing it later,
would forfeit the oracle rather than satisfy it.

---

## Read in this order

| # | File | What it establishes |
|---|---|---|
| 1 | `baseline/capture-state.txt` | the capture's provenance, measured: 90 of 90 cells reproduced from a `git archive` of the baseline commit, on four columns — `RESULT: PASS` |
| 2 | `baseline/pre-manifest.txt` | the 90-cell matrix by per-cell SHA-256, bytes, lines and exit code |
| 3 | `baseline/cross-check.txt` | two free agreements with `WO-MOK-011`'s measurements — 90 of 90 cells, 212 of 212 test names — `RESULT: PASS` |
| 4 | `baseline/census.txt` | the seven targeted verbs and both new `action_trace` fields at **zero occurrences in 110 MB**, which is oracle 5's and oracle 6's absence claims measured on this side |
| 5 | `baseline/test-census.txt` | the pre-change workspace test census: 212 target-qualified names, all passing |

---

## Every retained file

### Packet root

| File | Contents |
|---|---|
| `README.md` | this index |
| `capture.sh` | the harness that produces one 90-cell capture; `WO-MOK-011`'s script with adapted comments, so the two work orders' captures are comparable without a reader diffing the harness |

`capture.sh` deliberately does **not** take the 30 cells under `--policy social`. That source does not
exist at the baseline commit, and a fourth loop value would fail 30 runs with a configuration error.
The `social` cells are a separate post-change capture, listed separately in `VER-MOK-012`'s retention.

### `baseline/` — the pre-change side, and the shared tooling

| File | Contents |
|---|---|
| `COMMIT.txt` | `39662d13abd08e3410648d1c59ad38384f8ad2d2` — one line, so a script can read it |
| `capture-state.txt` | the provenance, measured rather than asserted: the capture reproduced from a tree that cannot hold a working-tree modification |
| `cross-check.txt` | the 90-cell matrix against `WO-MOK-011/post/post-manifest.txt` and the test census against `WO-MOK-011/merge/test-census.txt`, with what each check does and does not establish |
| `manifest.py` | digests a capture directory into a manifest. Carries **no projection**, and says at length why its absence is the point |
| `pre-manifest.txt` | 90 rows: cell, raw SHA-256, bytes, lines, exit code. Every cell exits `0` |
| `retain.py` | writes the stated subset kept whole, and states the subset and the reason for each piece |
| `init/<cell>.txt` | the initialization block of **every** cell — every line up to and including the last `tick=0 ` line, 90 files, 1.4 MB |
| `summary.txt` | rule 18's final summary line of every cell, one line per cell |
| `census.txt` | per cell: every `event=` kind, every verb proposed, and the two fields this change adds — all zero for the seven targeted verbs and both fields |
| `full/seed42-baseline-d0.75-traceon.txt` | one cell whole, 414 KB: a *traced* `baseline` stream, where a leaked `target` or `suffered` field would appear first |
| `test-run.txt` | the full `cargo test --locked --workspace` log at the baseline commit: one invocation, workspace root |
| `test-census.txt` | that log as 212 target-qualified names, sorted |

### `analysis/`

| File | Contents |
|---|---|
| `census.py` | `WO-MOK-011`'s census script, a byte-identical copy (`sha256 7d355454…`), so the two work orders' censuses are read by the same reader |

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

---

## What is owed

| `VER-MOK-012` retains | State |
|---|---|
| the pre-change 90-cell capture, with the commit recorded | **held** — `baseline/` |
| the pre-change workspace test census | **held** — `baseline/test-census.txt` |
| the post-change 90-cell capture, with `baseline` compared byte for byte and the other 60 cells' divergence characterized | owed |
| the new 30-cell capture under the `social` source, with exit codes | owed |
| the constructed-state resolution tables for damage, energy cost, threat increase and forfeit, with every boundary case | owed |
| the shared stream's recorded state either side of every resolution kind | owed |
| per-seed tables of survivors, deaths by cause, encounters, each verb proposed and applied, and rejections by reason — on failing seeds too | owed |
| the per-observation comparison of `social` against `individual` where no living Mokiterion is perceived and no attack is unanswered | owed |
| the branch distribution under `social` per seed, including the answer branch's three choices | owed |
| the measured strikes per encounter, forfeits discarded at a full recipient, and surrenders below `satiety` `2` | owed |
| the per-identifier series, rank correlations and band evaluation | owed |
| the identifier-exchange comparison for the constructed encounter | owed |
| rule 18's final summary per seed under each of the four sources, at both commits | **half held** — three sources at the baseline commit, in `baseline/summary.txt` |
| the line-for-line comparison showing rule 4, rule 9's eat effect, the food table and rules 14 to 16 unchanged | owed |
| the enumeration of reads per rule, source and validation path | owed |
| the enumeration of `fear`'s writers and of every path writing a second Mokiterion's state | owed |
| the engine public-interface enumeration at both commits, against the approved `SPEC-MOK-002` amendment | owed |
| the observer authority table's new rows and the `EventType::ALL` exhaustiveness check | owed |
| the post-change test census, reconciled name by name | owed |
| `cargo fmt`, `cargo clippy`, `cargo test`, `cargo tree -p Mokiterions` | **quarter held** — `cargo test` at the baseline commit only |
| the 10,000-tick run's completion, composition and survivor figures | owed |
| the eleven manual assessments, each with its accountable role and date | owed |
| the amendment-approval check of oracle 7, with the recorded state of the `VREC-MOK-005` gate | owed |

Nothing here is a verification verdict. `VER-MOK-012` is the contract, `VREC-MOK-012` will be the
record, and neither is written by the implementation.
