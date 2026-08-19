# `WO-MOK-011` evidence packet — index

| Field | Value |
|---|---|
| Work order | `WO-MOK-011` (Phase 2.5, naming) |
| Verification contract | `VER-MOK-011` |
| Implements | `REQ-MOK-040`, `REQ-MOK-041` |
| Baseline (pre-change) commit | `524a6758d74b5240079959e9827ea40a7af22a30` |
| Branch | `feature/phase-2-5-naming` |
| Date | 2026-08-19 |
| Packet size | 8.2 MB |

Start with `completion-summary.md`. It reports every figure in `WO-MOK-011`'s stated order and cites
the file each one comes from. `manual-assessment.md` records which judgements are made and which is
outstanding.

This chain was renumbered from `010` to `011` after the packet was captured; `renumbering.md`
records what that rewrite touched, what it could not preserve, and why no measurement here moves.

---

## Read in this order

| # | File | What it establishes |
|---|---|---|
| 1 | `completion-summary.md` | the whole change and every measurement, in the work order's eleven-section order |
| 2 | `manual-assessment.md` | `VER-MOK-011`'s seven manual assessments; **assessment 5 is OUTSTANDING** |
| 3 | `additivity.txt` | oracle 1: 90 cells byte-identical after projection, equal exit codes — `RESULT: PASS` |
| 4 | `analysis/mutation-control.txt` | acceptance scenario 2: oracles 1 and 2 shown to *fail* under a deliberate perturbation, then reverted — `RESULT: PASS` |
| 5 | `analysis/census-reconciliation.txt` | oracle 3: 193 → 205, twelve additions named, no removal, no rename |
| 6 | `analysis/positional-parsers.txt` | oracle 3, second half: both positional parsers byte-identical to the baseline |
| 7 | `interface.txt` | oracle 5, first half: both packages' public interfaces, item for item, at both revisions |
| 8 | `amendment-approvals.md` | oracle 5, second half: the amended artifacts' governance state — `RESULT: PASS`, 9/9 controls |
| 9 | `observer/frames.txt` | oracle 4: rendered character buffers, cell positions, the bar-row comparison |
| 10 | `static-checks.txt` | the seven static searches `VER-MOK-011` retains |
| 11 | `gates.txt` | `cargo fmt`, `cargo clippy -D warnings`, `cargo tree`, `cargo test` |

---

## Every retained file

### Packet root

| File | Contents |
|---|---|
| `README.md` | this index |
| `completion-summary.md` | the completion report |
| `manual-assessment.md` | the seven manual assessments, with the measurement each was decided against |
| `additivity.txt` | oracle 1's per-cell table: 90 rows, `proj(post)==pre`, `proj(pre)==pre`, exit codes, bytes removed |
| `interface.txt` | oracle 5's interface enumeration, both packages, both revisions, plus the full post-change listings (125 and 92 lines) |
| `amendment-approvals.md` | the generated governance report over the amended and the frozen artifacts |
| `static-checks.txt` | the seven static searches, each with its command and its finding |
| `gates.txt` | the four gates' commands and output, with per-target test results |
| `capture.sh` | the harness that produces one 90-cell capture; run at the baseline commit and at the candidate |
| `renumbering.md` | why this chain is numbered `011` and not `010`, what the rename touched, and the one recorded digest it moves |

### `baseline/` — the pre-change side, and the shared tooling

| File | Contents |
|---|---|
| `COMMIT.txt` | `524a6758d74b5240079959e9827ea40a7af22a30` — the commit the pre-change capture was taken at, one line because `compare.py` reads it |
| `capture-state.txt` | the provenance of both captures, measured: all 90 pre-change cells reproduced from a `git archive` of the baseline commit, all 90 post-change cells from the restored tree |
| `pre-manifest.txt` | per-cell `sha256(raw)`, `sha256(projected)`, byte count, line count and exit code, 90 cells |
| `projection.py` | oracle 1's projection, with its reasoning in its docstring — **the artifact manual assessment 5 is about** |
| `compare.py` | writes both manifests and `additivity.txt`; comparison is SHA-256 over whole streams |
| `retain.py` | selects what is kept from a 110 MB capture, and states why each piece |
| `init/<cell>.txt` | **90 files** — every line up to and including the last `tick=0` line, for every cell |
| `full/<cell>.txt` | **3 files** — three complete 1,000-tick streams, seed 42, default density, one per decision source, untraced |

### `post/` — the post-change side, same shape

| File | Contents |
|---|---|
| `post-manifest.txt` | the same six columns per cell, 90 cells, at the candidate tree |
| `init/<cell>.txt` | **90 files** — the initialization block of every cell, where the added field appears |
| `full/<cell>.txt` | **3 files** — the same three cells whole |
| `test-run.txt` | the full `cargo test` log at the candidate: one invocation, workspace root |
| `test-census.txt` | 205 test names, each qualified by the target that ran it |

`baseline/test-run.txt` and `baseline/test-census.txt` are the same two files at the baseline: 193
names. The pair is what oracle 3 reconciles.

### `analysis/` — the derived measurements and their tools

| File | Contents |
|---|---|
| `mutation-control.txt` | acceptance scenario 2, in full: the perturbation, both oracles' failures, the revert and its digest |
| `census-reconciliation.txt` | 193 → 205 by qualified name, the twelve additions with the obligation each carries, per-target counts |
| `positional-parsers.txt` | the byte comparison of `tests/process.rs` and `tests/viability.rs` against the baseline blobs |
| `names-per-seed.txt` | the twelve pairings the engine reported at each of the five declared seeds, and at nine policy × density combinations |
| `name-occurrences.txt` | every occurrence of `name:` in a 1,000-tick traced run, under each of the three sources |
| `census.py` | qualifies every test name by its target, so the reconciliation is by name and not by count |
| `interface.py` | counts what `SPEC-MOK-004` rule 6 counts, and documents that rule in its own docstring |
| `amendments.py` | generates `amendment-approvals.md`; nine self-tests must pass before it prints anything |

### `observer/` — oracle 4

| File | Contents |
|---|---|
| `frames.txt` | the read-out: pane rectangles, roster cell positions, the glyph scan, the inspector, and what the whole diff contains |
| `frames-pre.txt` | the baseline capture, 916 lines, nine viewports |
| `frames-post.txt` | the candidate capture, 916 lines, nine viewports |
| `bar-rows.txt` | the 96 bar rows the harness printed |
| `bar-rows.diff` | **empty** — the 96 rows are identical on both sides |
| `frame-dump.rs` | the throwaway harness's source; it is deleted from the tree, and it compiles against both revisions |

---

## Two things about this packet that a reviewer should know

**The 90 raw streams are not committed.** The declared matrix is 90 cells and about 110 MB per side,
and committing 220 MB of text to reproduce a comparison that SHA-256 already settles is not a
reasonable thing to do to this repository. What is retained is the form `VER-MOK-011` and
`baseline/compare.py` both state: every cell by digest, with byte and line counts and exit codes; the
initialization block of **every** cell on both sides, which is where the added field appears; and
three whole 1,000-tick streams per side for a reviewer who wants to see that the rest of a stream is
untouched. `capture.sh`, run at the commit in `baseline/COMMIT.txt`, regenerates a capture, and the
manifests detect a failed reproduction cell by cell. An earlier draft of this packet did retain the
90 pre-change streams and came to 118 MB; that was pruned, not the evidence's substance.

**`compare.py` writes all three of its outputs into one directory; this packet files them where the
work order says they belong.** Running

    python baseline/compare.py <pre-capture-dir> <post-capture-dir> <out-dir>

leaves `pre-manifest.txt`, `post-manifest.txt` and `additivity.txt` side by side in `<out-dir>`. In
the packet, `pre-manifest.txt` sits under `baseline/`, `post-manifest.txt` under `post/`, and
`additivity.txt` at the root, because those are the paths `WO-MOK-011`'s evidence list names. The
files are the tool's own output, moved and not edited. `retain.py` behaves the same way and its
docstring says `<out-dir>/manifest` where the packet has the manifest one level up. Nothing reads
these paths programmatically — both scripts take their directories as arguments — so a reviewer
re-running them chooses the layout and should not expect the tools to reproduce this one.

---

`VREC-MOK-011` is not in this packet. It is a separate commit-bound verification record, written by
the assurance owner against the commit that carries this implementation, and the implementation agent
neither writes nor approves it. `VER-MOK-011` is not satisfied while manual assessment 5 is
outstanding — see `manual-assessment.md`.
