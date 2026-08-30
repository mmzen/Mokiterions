# Evidence: `WO-MOK-033`, the sweep driver and the outcome classifier

## Summary

Two measuring instruments were built -- `scripts/run_simulation_batch.py`, which runs a sweep of
engine cells and retains one row per cell, and `scripts/classify_simulation_runs.py`, which reads
those rows and states an outcome distribution. This packet is what `VER-MOK-019` asked for: the
evidence that they measure what they claim to.

84 cases were executed across six phases, 0 failed. Verification found four
defects -- three in the driver and one in its own test suite -- and all four are repaired with tests
that fail on the pre-fix code; `harness/bite/bite-output.txt` is the record that they do. Nine gaps
between the approved specification and what was buildable are disclosed rather than amended, and four
findings about the swept space are recorded rather than acted on. The eight manual assessments
`VER-MOK-019` reserves to the product, technical and assurance owners are **not recorded**: they are
the owner's to make, and `findings/manual-assessments.md` states what each needs read.

## What is bound here

| Fact | Value |
|---|---|
| Work order | `WO-MOK-033` |
| Verification contract | `VER-MOK-019` |
| Specification | `SPEC-MOK-008` |
| Base commit | `c90edc9` |
| Default sweep | 400 cells: 4 sources x 5 densities x 20 seeds, 1,000 ticks |
| Retained sweep | 278,734 bytes, sha256 `a22e3e6b678e07b3...` |
| Cases | A 19 B 16 C 22 D 15 E 6 F 6 -- 84 total, 0 failed |
| Harness | 5,277 lines under `harness/`, standard library only |
| Withheld captures | 40 files, 141,111,666 bytes, stated in `not-committed.txt` |

The same 400-row output was produced four separate times during this work -- once before the phases
existed, twice in phase E at `--jobs 1` and `--jobs 4`, and once in phase F -- and all four are
byte-identical. That is `SPEC-MOK-008` rule 18.1's claim, and the `--jobs 4` production is what makes
it a measurement rather than a restatement of determinism.

## Where to start reading

1. `case-index.md` -- every case, by phase, in the order executed.
2. `findings/` -- the four defects, the nine disclosed gaps, the four sweep findings, and the eight
   assessments the owner has still to make.
3. `phase-f/scenario-1-distribution.txt` -- the answer to the question the sweep was built to ask.
4. `phase-<letter>/result.json` -- each phase's own machine-readable result, with the figure behind
   every case.
5. `assembly.txt` -- how this directory was produced, and what was left out of it.

## The distribution, in one place

The 400-cell default sweep, grouped by decision source:

| Source | extinction | asymmetric_collapse | collapse | coexistence | famine |
|---|---|---|---|---|---|
| `baseline` | 100 | 0 | 0 | 0 | 0 |
| `reference` | 29 | 7 | 14 | 50 | 0 |
| `individual` | 26 | 9 | 13 | 52 | 0 |
| `social` | 26 | 16 | 12 | 46 | 0 |

`baseline` takes no decision at all and every one of its 100 cells ends extinct. The three deciding
sources coexist in roughly half their cells. Whether that answers Phase 6's question -- *do the
conditions permit emergence*, and *are outcomes engine-determined* -- is manual assessment 1, and is
the product owner's to state, not this packet's.

## Reproducing a figure

The phase scripts are in `harness/` and take no arguments:

    python harness/phase_a_real_engine.py

Each wipes and rewrites its own output directory, so a re-run cannot leave a figure from an earlier
one standing. They expect a release build of the engine at `target/release/Mokiterions.exe` and the
repository at the path recorded in `assembly.txt`. Phase D runs the product's own cargo gates and
takes minutes; the others take seconds to a minute, except the sweep phases, which take about 30
seconds each.

Two things can be checked without running anything. Every committed file, against the manifest --
whose lines carry a byte count and a line count as well as the digest, so `sha256sum` needs the
columns it reads picked out:

    awk 'length($1)==64 {print $1 "  " $4}' manifest.sha256 | sha256sum -c

And the class of any retained row, against `phase-f/distribution-source-density.json`, whose per-row
entries name the ordered clause that assigned it.

## Line endings

Files here are stored as they were captured. Most are `\n`; a few are Windows standard-output
captures and carry `\r\n`. `.gitattributes` marks this tree `-text`, so git stores and checks out
exactly these bytes and the digests in `manifest.sha256` reproduce on any platform. The CRLF-carrying
files and their counts are listed in `assembly.txt`.

No total byte count for this directory appears in this file. `manifest.sha256` states one, over the
files it covers, which is every file except itself. A total stated inside a file that the total covers
cannot be written down and stay true.
