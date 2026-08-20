# `WO-MOK-012` evidence packet — index

| Field | Value |
|---|---|
| Work order | `WO-MOK-012` (Phase 3.1, encounters) |
| Verification contract | `VER-MOK-012` |
| Implements | `REQ-MOK-042` … `REQ-MOK-051` |
| Baseline (pre-change) commit | `39662d13abd08e3410648d1c59ad38384f8ad2d2` |
| Candidate (post-change) commit | `7c4aef3967406c05d80da963695898b77f5329e9` |
| Branch | `feature/phase-3-definition` |
| Date opened | 2026-08-20 |
| Packet size | 2.1 MB, 116 files |

**This packet is incomplete, and one requirement it measures is failing.** It holds the pre-change side
in full, the decision-independent part of the post-change side, and the escalation record. Both are
stated here so that neither an incomplete packet nor a failing measurement can be mistaken for
anything else:

- **`escalation.md` is the file to read first.** `REQ-MOK-049`'s lethality bound is missed on every
  declared seed, `surrender` never applies, and three tests fail at the candidate. Work is stopped on
  four owner decisions, each measured and put with its alternatives.
- **The curve-dependent evidence is deliberately not here.** The product owner directed on 2026-08-20
  that only the decision-independent items be produced now — the byte-identity manifests, the test
  census reconciled name by name, the `REQ-MOK-050` read enumeration and the escalation record — and
  that the survivor curve, verb counts, branch distribution and monotonicity band be held until the
  `REQ-MOK-048` amendment is measured, since the amendment moves all of them. The table at the end of
  this file marks each item **held**, **owed**, or held-and-failing.

The reason the pre-change side exists as its own commit is `VER-MOK-012` oracle 1: "The engine's
complete standard output and exit code are captured *before* any code change, at the commit the work
begins from… The baseline is captured once, from a tracked-clean worktree, with the commit recorded. A
discrepancy is never resolved by recapturing it." Capturing it after the fact, or recapturing it later,
would forfeit the oracle rather than satisfy it.

---

## Read in this order

| # | File | What it establishes |
|---|---|---|
| 1 | `escalation.md` | **the four failing obligations, their one cause, and the measurement of every option** — including the option the owner selected, which the measurement refutes |
| 2 | `post/byte-identity.txt` | the two clauses of oracle 1: `baseline` identical on 30 of 30 cells unprojected; the 60-cell divergence absent because `REQ-MOK-051` is unimplemented — `RESULT: MIXED` |
| 3 | `post/capture-state.txt` | the candidate capture's provenance, measured: 90 of 90 cells reproduced from a `git archive` of the candidate commit — `RESULT: PASS` |
| 4 | `post/test-census-reconciliation.md` | 212 names into 249, name by name: 211 retained, 1 rename proved verbatim by digest, 38 added, 0 removed, 0 ignored |
| 5 | `post/reads.md` | `REQ-MOK-050` discharged by enumeration: every rule, source and validation path, and the seven readers of a set with the reason each is outside the obligation |
| 6 | `baseline/capture-state.txt` | the baseline capture's provenance, on the same four columns — `RESULT: PASS` |
| 7 | `baseline/pre-manifest.txt` | the 90-cell matrix by per-cell SHA-256, bytes, lines and exit code |
| 8 | `baseline/cross-check.txt` | two free agreements with `WO-MOK-011`'s measurements — 90 of 90 cells, 212 of 212 test names — `RESULT: PASS` |
| 9 | `baseline/census.txt` | the seven targeted verbs and both new `action_trace` fields at **zero occurrences in 110 MB**, which is oracle 5's and oracle 6's absence claims measured on this side |
| 10 | `baseline/test-census.txt` | the pre-change workspace test census: 212 target-qualified names, all passing |

---

## Every retained file

### Packet root

| File | Contents |
|---|---|
| `README.md` | this index |
| `escalation.md` | the four failing obligations, the cause, the owner's selected option measured and refuted, and the one region of `REQ-MOK-048`'s own lever where both bounds hold |
| `capture.sh` | the harness that produces one 90-cell capture; `WO-MOK-011`'s script with adapted comments, so the two work orders' captures are comparable without a reader diffing the harness |
| `capture-social.sh` | the 30 `social` cells, in `capture.sh`'s shape unchanged — same loop order, same cell naming, same `.exit` file beside each stream — so the two captures are comparable by the same reader |

`capture.sh` deliberately does **not** take the 30 cells under `--policy social`. That source does not
exist at the baseline commit, and a fourth loop value would fail 30 runs with a configuration error.
The `social` cells are a separate post-change capture, listed separately in `VER-MOK-012`'s retention,
and `capture-social.sh` is that capture. Its shape is deliberately not improved on: the two scripts
differ in their policy list and in nothing else, so a discrepancy between the two manifests cannot be a
discrepancy between two harnesses.

`escalation.md` sits at the packet root rather than under `post/` because it is not a measurement of the
candidate: it is the record of what the whole work order cannot discharge, and it cites both sides.
Every path it names is relative to this directory, as `post/byte-identity.txt`'s are.

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

### `post/` — the candidate side, at `7c4aef3`

| File | Contents |
|---|---|
| `COMMIT.txt` | `7c4aef3967406c05d80da963695898b77f5329e9` — one line, so a script can read it |
| `capture-state.txt` | the provenance, on the baseline's terms: 90 of 90 cells reproduced from a `git archive` of the candidate commit and agreeing with the working-tree capture on all four columns — `RESULT: PASS` |
| `post-manifest.txt` | 90 rows, produced by `baseline/manifest.py` unchanged, from the archive reproduction |
| `social-manifest.txt` | the 30 `social` cells, labelled **provisional**: the `REQ-MOK-048` amendment moves every one of them |
| `byte-identity.txt` | oracle 1's two clauses compared and answered separately — `RESULT: MIXED` |
| `test-run.txt` | the full `cargo test --locked --workspace --no-fail-fast` log at the candidate |
| `test-census.txt` | that log as 249 target-qualified names with each outcome, sorted |
| `test-census-reconciliation.md` | row 248 discharged name by name, including the one rename proved verbatim by digest |
| `reads.md` | the `REQ-MOK-050` enumeration and the `fear`-writer enumeration; verdict **met** |

`--no-fail-fast` is on the candidate's invocation and was not on the baseline's, for a reason
`test-census-reconciliation.md` §4 states: without it cargo stops after the first failing target and
eight later targets never run, so a census taken from that log would be missing names and would read as
removals. The plain invocation was run too and exits `101` identically.

### `analysis/`

| File | Contents |
|---|---|
| `census.py` | `WO-MOK-011`'s census script, a byte-identical copy (`sha256 7d355454…`), so the two work orders' censuses are read by the same reader |
| `test-census.py` | turns one `cargo test` log into a target-qualified census. Validated against the side it did not write: run over `baseline/test-run.txt` it reproduces the hand-written `baseline/test-census.txt` line for line. **Records each outcome rather than filtering**, so a case that stopped passing cannot be lost by omission |

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
on. That is 120 cells across the two manifests, and the two are kept apart deliberately — oracle 1 asks
whether the first 90 moved, which is a question the thirty new ones cannot answer and must not dilute.

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
| the post-change 90-cell capture, with `baseline` compared byte for byte and the other 60 cells' divergence characterized | **held, and one clause fails** — `post/post-manifest.txt` and `post/byte-identity.txt`. `baseline` is identical on 30 of 30, unprojected. The 60-cell divergence **does not exist**, because `REQ-MOK-051` is unimplemented, so the characterization is unwritable and the requirement is recorded as failing in `escalation.md` |
| the new 30-cell capture under the `social` source, with exit codes | **held, provisionally** — `capture-social.sh` and `post/social-manifest.txt`, 30 cells, every cell exiting `0`. Provisional because the `REQ-MOK-048` amendment moves all thirty |
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
| the enumeration of reads per rule, source and validation path | **held** — `post/reads.md` §§2, 4 and 6, with §5's seven readers of a set and the reason each is outside the obligation. Verdict: `REQ-MOK-050` met |
| the enumeration of `fear`'s writers and of every path writing a second Mokiterion's state | **held** — `post/reads.md` §7: two `fear` writers, five paths writing a second Mokiterion, three functions |
| the engine public-interface enumeration at both commits, against the approved `SPEC-MOK-002` amendment | owed — and it is now checked against **four** growth rows, not three: that amendment gained a row on 2026-08-20 for the `suffered` field on the existing public `ActionTrace` payload, which its first three rows had omitted |
| the observer authority table's new rows and the `EventType::ALL` exhaustiveness check | owed |
| the post-change test census, reconciled name by name | **held** — `post/test-census.txt` and `post/test-census-reconciliation.md`: 249 names, 211 retained, 1 rename proved verbatim by digest, 38 added, 0 removed, 0 ignored. **246 pass, 3 fail** |
| `cargo fmt`, `cargo clippy`, `cargo test`, `cargo tree -p Mokiterions` | **half held** — `cargo test` at both commits, `post/test-run.txt` being the candidate's log. `fmt`, `clippy` and `tree` are owed as recorded output at the candidate |
| the 10,000-tick run's completion, composition and survivor figures | owed |
| the eleven manual assessments, each with its accountable role and date | owed |
| the amendment-approval check of oracle 7, with the recorded state of the `VREC-MOK-005` gate | owed |

**Eleven items are owed and ten of the eleven are held back on purpose**, on the product owner's
direction of 2026-08-20: every one of them is a measurement of the `social` source's behavior, and the
`REQ-MOK-048` amendment `escalation.md` puts to the owner moves all of them. Writing them now would
produce a packet describing a source that is about to change, and `VER-MOK-012` would need them written
twice. The eleventh, the public-interface enumeration, is owed for no such reason and is simply not yet
written.

`escalation.md` is not on this table because `VER-MOK-012`'s retention does not list it. It is required
by `WO-MOK-012`'s stop-and-escalate conditions, which oblige a recorded escalation rather than an
adjustment, and it is retained here because the same conditions forbid the alternative.

Nothing here is a verification verdict. `VER-MOK-012` is the contract, `VREC-MOK-012` will be the
record, and neither is written by the implementation.
