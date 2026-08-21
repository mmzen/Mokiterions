# `WO-MOK-017` evidence packet — index and reproduction

| Field | Value |
|---|---|
| Work order | `WO-MOK-017` — "Correct the resource composition drift: the numeric waste condition, and both survivor floors re-measured against the corrected world" |
| Requirement | `REQ-MOK-060`, whose ceiling is **three fifths** as amended 2026-08-21; `SPEC-MOK-001` rules 5 and 19 as amended the same date |
| Verification contract | `VER-MOK-016`, which already carries `REQ-MOK-060`. No new verification requirement was created |
| Pre-change commit | `pre/COMMIT.txt` — `1ba5a3aabe775c9cdee29a04b399af3bb82dde90`, this work order's own governance-only approval commit |
| Candidate commit | `post/COMMIT.txt` — `26ae6ba648be4eecf6234da15c0beb763b403a0a`, on `feature/resource-composition-ceiling` off `origin/master` `7f4792a` |
| Merge commit | `merge/COMMIT.txt` — `ae2e44f2382fe89f2daf78a8e8aca37febb8bd0f`, the merge of `master` at `3f47743`. Added after the candidate; see `merge/README.md` |
| Toolchain | cargo 1.97.1, rustc 1.97.1, clippy 0.1.97, host `x86_64-pc-windows-msvc` |
| Verdicts | **13 reader reports at the candidate, 13 `RESULT: PASS`, 0 `FAIL`.** On the merged tree, 11 hold by byte-identical inputs and the 2 source-side readers print `FAIL`, retained and read in `merge/` |
| Outstanding | **3 manual assessments**, prepared and unsigned in `manual-assessment.md`, plus **9 findings** raised there |
| Date | 2026-08-21 |

**The headline result.** `REQ-MOK-060` is met: no calorie class holds more than three fifths of any
territory's standing resources on any of the 30 territory evaluations the requirement binds, worst
**54.1%** against 60.0%. All three carried survivor floors hold, at margins **0**, **0** and **2**. The
30 `baseline` cells are byte-identical across the change and the 90 obligated cells all diverge, each
divergence attributed to a route with none unattributed. 268 tests pass, 267 before.

**What this packet does not contain, and cannot.** No verification record and no verdict: `VER-MOK-016`
is the contract and its record is a separate, commit-bound act reserved to the assurance owner. No
signed manual assessment: `manual-assessment.md` is prepared material and every record block in it is
empty, which is the state it is meant to be in when an implementation agent hands it over.

---

## Reproducing the whole packet

Everything below is derived from two captures of the same 120-cell matrix, one per commit. The captures
are about 156 MB each, are written outside the repository and are **not** retained — `analysis/retain.py`
states what is kept instead, cell by cell, and why each piece is worth its bytes. These are the commands
that rebuild them.

```sh
# 1. Both captures, each from its own commit's archived tree rather than from a working tree.
for side in pre post; do
  rm -rf /scratch/$side-tree /scratch/$side-cells
  mkdir -p /scratch/$side-tree /scratch/$side-cells
  git archive $(cat docs/engineering/simulation/evidence/WO-MOK-017/$side/COMMIT.txt) \
    | tar -x -C /scratch/$side-tree
  ( cd /scratch/$side-tree && cargo build --release --offline -p Mokiterions )
  ( cd /scratch/$side-tree \
    && bash <repo>/docs/engineering/simulation/evidence/WO-MOK-017/capture.sh /scratch/$side-cells )
done
```

Each capture takes about six seconds and yields 120 `<cell>.txt` streams with 120 `<cell>.exit` codes.
`capture.sh` is used on both sides deliberately, so that the comparison is a statement about the two
trees and not about two harnesses. `pre/capture-state.txt` and `post/capture-state.txt` are where that
provenance is measured rather than asserted.

```sh
# 2. The digests and the retained subset, per side.
python .../analysis/manifest.py /scratch/pre-cells  .../pre/pre-manifest.txt \
    "pre-change capture at 1ba5a3a..."
python .../analysis/manifest.py /scratch/post-cells .../post/post-manifest.txt \
    "post-change capture at 26ae6ba..., 120 cells, all four sources"
python .../analysis/retain.py   /scratch/pre-cells  .../pre/  "pre-change capture at 1ba5a3a..."
python .../analysis/retain.py   /scratch/post-cells .../post/ "post-change capture at 26ae6ba..."

# 3. The seven readers that take both captures. Each writes to stdout, exits 0 on every check passing,
#    and ends its report with RESULT: PASS or RESULT: FAIL.
python .../analysis/composition.py     /scratch/pre-cells /scratch/post-cells > .../post/composition.txt
python .../analysis/survivors.py       /scratch/pre-cells /scratch/post-cells > .../post/survivors.txt
python .../analysis/entropy.py         /scratch/pre-cells /scratch/post-cells > .../post/entropy.txt
python .../analysis/health-falls.py    /scratch/pre-cells /scratch/post-cells > .../post/health-falls.txt
python .../analysis/dead-neighbours.py /scratch/pre-cells /scratch/post-cells > .../post/dead-neighbours.txt
python .../analysis/divergence.py      /scratch/pre-cells /scratch/post-cells .../post/
python .../analysis/docs-figures.py    /scratch/pre-cells /scratch/post-cells /scratch/out-of-matrix \
    > .../post/docs-figures.txt

# 4. The readers that take source or a test log instead of a capture.
git show $(cat .../pre/COMMIT.txt):mokiterions-core/src/simulation.rs > /scratch/pre-simulation.rs
python .../analysis/world-rules.py /scratch/pre-simulation.rs mokiterions-core/src/simulation.rs \
    > .../post/world-rules-unchanged.txt
python .../analysis/reads.py mokiterions-core/src/simulation.rs > .../post/reads.md
python .../analysis/test-census.py .../post/test-run.txt .../post/test-census.txt
cd docs/engineering/simulation/evidence/WO-MOK-017 \
  && python analysis/census-reconcile.py pre/test-census.txt post/test-census.txt \
       > post/test-census-reconciliation.md
```

Three practical notes, each of which cost time to find:

* **`divergence.py` takes an out-directory, not stdout.** It writes two reports — `divergence.txt` and
  `byte-identity.txt` — because the byte comparison and the characterization are read from the same
  parse of the same 240 streams and splitting them into two invocations would double the work.
* **`census-reconcile.py` must be run from this packet's directory.** It prints the two paths it was
  given in its own header, and the committed report carries the two short relative paths. Every other
  reader in `analysis/` is invoked from the repository root and is indifferent to the working directory.
* **`docs-figures.py` takes a third directory** holding the seven runs that are *not* cells of the
  matrix. Those seven are one command each, printed with their output in `post/docs-figures.txt` §10,
  and one of them is deliberately a matrix cell's parameters so the out-of-matrix binary is shown to be
  the matrix binary.

Every one of the 120 candidate cells, the retained subset and every capture-derived file under `post/`
have been reproduced byte for byte from `git archive 26ae6ba` on this toolchain, the deliberately
changed label lines aside. `post/capture-state.txt` is that measurement.

---

## The index

### Packet root

| Path | What it is | Produced by |
|---|---|---|
| `README.md` | this file | hand-composed |
| `capture.sh` | the 120-cell capture: 5 seeds × 4 sources × 3 densities × trace on/off, at 1,000 ticks, with each cell's exit code beside it. `WO-MOK-016`'s script with `social` as a fourth loop value rather than a second script | hand-composed |
| `manual-assessment.md` | the manual assessments this work order reaches, prepared with their evidence and **unsigned**, plus nine findings raised for their owners. **3 outstanding, 0 recorded** | hand-composed; the record blocks are the owner's |

### `pre/` — the pre-change side, 8 files and 2 directories

| Path | What it is | Produced by |
|---|---|---|
| `COMMIT.txt` | the bare pre-change hash, one line, so a script can read it | — |
| `capture-state.txt` | the provenance of the pre-change capture: tracked-clean at `1ba5a3a`, reproduced 120 of 120 cells on four columns from the commit's own archived tree, and cross-checked against `WO-MOK-016`'s two post-change manifests on all 120. **`RESULT: PASS`** | hand-composed from measured output |
| `pre-manifest.txt` | per cell: raw SHA-256, bytes, lines, exit code. 120 cells, no projection | `analysis/manifest.py` |
| `init/` | the initialization block of every cell, 120 files. Initialization is untouched by this change, which is what makes the block worth keeping: it is the cheapest per-cell demonstration that both captures are of the same 120 worlds | `analysis/retain.py` |
| `full/` | 2 cells retained whole: `seed42-baseline-d0.75-traceon` and `seed42-reference-d0.15-traceon` | `analysis/retain.py` |
| `summary.txt` | rule 18's final summary line of every cell, 120 lines. Carries the six food counts the ceiling is measured from and the survivor count the floors are measured from | `analysis/retain.py` |
| `census.txt` | per cell, the count of every `event=` kind and every verb proposed | `analysis/retain.py` |
| `eaten.txt` | per cell and class: resources consumed, highest pre-eat satiety, satiety wasted. This is the census that makes the corrected condition visible **in the stream** rather than inferred from the composition | `analysis/retain.py` |
| `test-run.txt` | the pre-change `cargo test` log, whole. 267 names, 267 passed, exit 0 | `cargo test` |
| `test-census.txt` | one line per test name, from that log | `analysis/test-census.py` |

### `post/` — the candidate side, 21 files and 2 directories

Eight entries are the same artifact as their `pre/` counterpart, taken the same way by the same reader
on the other commit, and are not restated here: `COMMIT.txt`, `post-manifest.txt`, `init/`, `full/`,
`summary.txt`, `census.txt`, `eaten.txt` and `test-census.txt`. The remaining fifteen are this side's
own, and they are the analysis.

| Path | What it establishes | Produced by | Verdict |
|---|---|---|---|
| `capture-state.txt` | every retained byte under `post/` is the output of `26ae6ba`, produced by building and running that commit's archived tree; the two manifests partition exactly by source, 30 `baseline` rows identical and 90 others moved; and the three-column prediction `pre/capture-state.txt` §4 made before the change was written is met on all four sources | hand-composed from measured output | **PASS** |
| `composition.txt` | `REQ-MOK-060` evaluated exactly as amended. 30 of 30 territory evaluations meet three fifths, worst 54.1% of 61; the value the ceiling was amended from is evaluated beside it, 5 of 30 still above one half, each named; §9 is the narrowest-class measurement the deferred per-class floor is reserved to | `analysis/composition.py` | **PASS** |
| `survivors.txt` | all three floors at both commits — `REQ-MOK-014` 8, `REQ-MOK-034` 8, `REQ-MOK-058` 5 — with their margins, the lethality bound, and §4's direction of travel: 9 of 15 runs leave fewer living | `analysis/survivors.py` | **PASS** |
| `byte-identity.txt` | the 30 `baseline` cells compared byte for byte with their exit codes. 30 of 30 identical. `INT-MOK-010`'s promise and `REQ-MOK-060`'s load-bearing constraint | `analysis/divergence.py` | **PASS** |
| `divergence.txt` | 90 of 90 obligated cells diverge, each first divergence attributed — 57 direct, 9 through the shared stream, 24 through the traced sibling, **0 unattributed** — with the refutation check that no cell diverges before the tick its band is first entered | `analysis/divergence.py` | **PASS** |
| `entropy.txt` | the correction consumes no entropy; all 15 initialization draw totals and all 120 initialization prefixes identical at both commits; every resolution kind counted either side. §5 records the one residual gap, the per-run draw total, and why it is not derivable here | `analysis/entropy.py` | **PASS** |
| `world-rules-unchanged.txt` | rule 4, rule 9's eat effect, the food table and rules 14 to 16 compared line for line against the pre-change source. Each is a way the correction could have escaped its own scope | `analysis/world-rules.py` | **PASS** |
| `reads.md` | the static check that no decision source reads composition | `analysis/reads.py` | **PASS** |
| `gates.txt` | `cargo fmt`, `cargo clippy` and `cargo test` with their exit codes, plus `cargo tree` and the manifest, package, target and `#[ignore]` checks. Records that clippy was run from a cleaned target directory, and that the first `fmt` invocation failed on three pieces of this work order's own test code | hand-composed from command output | — |
| `test-run.txt` | the candidate `cargo test` log, whole. 268 names, 268 passed, exit 0 | `cargo test` | — |
| `test-census-reconciliation.md` | the two censuses reconciled name by name: 267 → 268, one added, none removed, none renamed, none ignored | `analysis/census-reconcile.py` | **PASS** |
| `updated-tests.md` | the six amended test bodies and the one added, each with what forced it, what it asserts now, and why that is at least as strong. Names both retracted assertions and the one constant that moved. `SPEC-MOK-002` rule 12 is the standard it is written against | hand-composed; the two observer-side arguments rest on the two readers below | — |
| `health-falls.txt` | the health decline available to `VER-MOK-013`'s gauge scenario, per source per seed, at both commits. The guard genuinely fired: 80 → 0 under `reference`, 45 → 0 under `individual` | `analysis/health-falls.py` | **PASS** |
| `dead-neighbours.txt` | which identifier dies first and whether it is interior. `M05` at tick 604 → `M01` at tick 314, the lowest identifier, from which BackTab wraps. Also why `baseline` cannot serve as the scenario's source | `analysis/dead-neighbours.py` | **PASS** |
| `docs-figures.txt` | every figure in `SIMULATION_RULES.md` and `docs/ROADMAP.md` recomputed against the corrected world: 120 claims, 103 recomputed, 8 cited to a sibling file, 9 flagged as history, **0 disagreements** | `analysis/docs-figures.py` | **PASS** |

### `analysis/` — 13 readers

Every reader writes its report in full, prose included, and ends with `RESULT: PASS` or `RESULT: FAIL`;
none writes a report it has not itself checked. All thirteen call
`sys.stdout.reconfigure(encoding='utf-8', newline='\n')`, so their output is LF on Windows as well as
elsewhere — see *Conventions* below for why that matters.

| Reader | Writes |
|---|---|
| `manifest.py` | `pre/pre-manifest.txt`, `post/post-manifest.txt` |
| `retain.py` | `{pre,post}/{init/,full/,summary.txt,census.txt,eaten.txt}` |
| `composition.py` | `post/composition.txt` |
| `survivors.py` | `post/survivors.txt` |
| `divergence.py` | `post/divergence.txt` **and** `post/byte-identity.txt` |
| `entropy.py` | `post/entropy.txt` |
| `world-rules.py` | `post/world-rules-unchanged.txt` |
| `reads.py` | `post/reads.md` |
| `health-falls.py` | `post/health-falls.txt` |
| `dead-neighbours.py` | `post/dead-neighbours.txt` |
| `docs-figures.py` | `post/docs-figures.txt` |
| `test-census.py` | `pre/test-census.txt`, `post/test-census.txt` |
| `census-reconcile.py` | `post/test-census-reconciliation.md` |

### `approval/` — 16 files, committed with the governance amendments in `1ba5a3a`

The measurement the owner's decisions of 2026-08-21 were taken on, retained in full because a figure
from a build that no longer exists is not evidence. `measurement.md` is the report; `probe-017.patch` is
the patch the figures were taken through, applied and then reverted, so `mokiterions-core` is unchanged
at `1ba5a3a`; the seven `probe-*.py` scripts and their seven `raw-*.txt` outputs are the measurement
itself. `raw-ratified.txt` is the per-cell table the ratified condition was chosen on, `raw-shape.txt`
is the comparison of mechanism shapes, and `raw-unbound-seeds.txt` is the 50-seed disclosure of what the
correction costs outside the declared set.

**This directory is not re-derived by anything above.** Its figures were taken on a patched build at
`1ba5a3a` and the packet's own measurements were taken on the shipped build at `26ae6ba`; where the two
speak to the same quantity, `post/composition.txt` §6 and `post/survivors.txt` §3 record the agreement
and are the ones that bind.

### `merge/` — 14 files, the merge of `master` at `3f47743`

Added after the candidate, when `master` moved and pull request #39 stopped being mergeable. **Nothing
above this line is re-measured or re-written by it**, because the merged tree's own 120-cell capture is
byte-identical to `post/post-manifest.txt` on all 120 cells and all four columns — which is the licence,
measured rather than argued, and it is what carries this packet's figures forward to the merge commit
`ae2e44f`. `merge/README.md` is the account and `merge/gates.txt` is every command; between them they
record the two conflicts and their resolution, test conservation at 302 in both directions, the two
source-side readers that **do** print `RESULT: FAIL` on the merged tree and why neither is a world rule
moving, and the `SPEC-MOK-004` rule 11 correction this work order owes, measured and drafted in full and
deliberately not applied.

---

## Conventions

* **Every reader emits its whole report, prose included.** No file here is a table a human then wrote
  around; the argument and the numbers come out of the same invocation, so a reviewer who re-runs a
  reader gets the sentences as well as the figures and can diff the whole thing.
* **`RESULT:` is the last line of every report**, and it is the reader's own verdict on its own checks.
  A reader that finds a problem prints `RESULT: FAIL`, lists the failed checks and exits non-zero. None
  of the thirteen does at the candidate commit. Two of them do on the merged tree, and `merge/` retains
  both reports as they printed rather than repairing the reader — see `merge/README.md`.
* **LF and UTF-8 throughout, except `approval/raw-*.txt`.** `.gitattributes` pins
  `docs/engineering/simulation/evidence/** -text`, so the committed bytes are the bytes, and a Windows
  clone does not rewrite a retained stream to CRLF and invalidate every recorded digest. The seven
  `approval/raw-*.txt` files were captured through a shell redirect on Windows before that discipline
  was applied to this packet and are CRLF; they are committed and are not edited, because editing
  retained evidence is worse than an inconsistent line ending in it.
* **Figures live in one place.** Where two files would state the same number, the later one cites the
  earlier rather than recomputing it — `post/docs-figures.txt` §11 is an explicit list of the eight
  figures it takes from a sibling. Where any file in this packet and a reader's own report disagree, the
  report is right.
* **Absolute paths are shortened** to the workspace member's or the packet's own directory name wherever
  a tool prints them. Nothing else in any captured output is edited, and every exit code is its own.
