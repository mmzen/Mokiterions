# Non-perturbation capture for WO-MOK-024

`VER-MOK-004`'s *Requirement-to-evidence matrix* fixes three rows that no argument satisfies:

| Row | Pass condition |
| --- | --- |
| Simulation output unchanged | All 43 matrix cells byte-identical to the baseline |
| Diagnostics unchanged apart from the usage block | For each of the fourteen invalid-configuration cases, exit code, empty standard output, and the `configuration error:` line are byte-identical |
| Both emission paths carry the same text | `--help` writes the new text to standard output; an invalid configuration writes the same text to standard error after the `configuration error:` line |

**These were captured rather than reasoned about, and that is the point of this directory.**
`WO-MOK-024`'s own case is that no engine source is in the diff at all, so no run can move — a
true statement and a weak one, because it is a claim about a diff and the rows are claims about
every run. The contract's *Independence* section says as much: review cannot be re-run on a
future change. So the matrix was executed on both sides.

## Method

`capture.sh` here is `WO-MOK-004`'s script, copied **byte for byte and unmodified**, so that the
43 cells and the 16 named cases are the same cells and the same cases the contract fixed, not a
set chosen after the fact. It defines 5 seeds × 2 decision sources × 2 declared densities × 2
trace settings at 1,000 ticks, plus two 20-tick traced runs and the no-argument invocation.

| | Value |
| --- | --- |
| Baseline side | a `git worktree` at `f7b1c452039dc2f03010ca8b8cc81e73c54727c0`, built there with `cargo build` |
| Candidate side | this branch at the commit `VREC-MOK-022` binds, built with `cargo build` |
| Binary | `./target/debug/Mokiterions` on both sides, the profile `capture.sh` names |
| Oracle | SHA-256 per cell, plus line count, exit code and the full final summary line |

The baseline was rendered from a real checkout of the base commit and built there. It is not
reconstructed, and it is not `WO-MOK-004`'s historical baseline reused: that one describes a
twelve-line usage text and a 52-test suite, and `VER-MOK-004`'s amendment of 2026-08-22 says its
figures are historical and not restated. Re-deriving against this work order's own base commit
removes the question of whether a stale baseline still describes the tree.

## Results

| Comparison | Result |
| --- | --- |
| `manifest-base.txt` vs `manifest-after.txt` | **identical**, all 43 cells: digest, line count and exit code |
| `summaries-base.txt` vs `summaries-after.txt` | **identical**, all 40 matrix summary lines |
| the two 20-tick traced runs, `diff -r` on the full text | **identical** |
| the 16 named cases, on the four fields the contract fixes | **one change of sixteen**, and it is `help` |
| both emission paths | **byte-identical**, 3601 bytes each, and equal to `../after/engine-usage.txt` |

`field-comparison.txt` is the per-case table. The single change is `--help`'s standard output,
2195 → 3600 bytes, which is the subject of this work order. Every one of the fourteen
invalid-configuration cases holds exit `2`, an empty standard output, and a byte-identical
`configuration error:` line; their standard error grew from 39 lines to 72 because the usage
block appended after that line grew from 38 lines to 71. The valid single-tick case is
byte-identical in every field, 14386 bytes of output at exit `0`.

`both-paths.txt` is the emission-path check. Both streams carry 3601 bytes and 71 lines, the two
blocks are byte-identical to each other and to the retained text, and **neither stream contains a
carriage return** on this Windows machine.

## What is retained here and what is not

Retained: both manifests, both summary files, both raw `exit-codes.txt` captures, the per-case
comparison, the emission-path check, and the script.

**Not retained: the four full 20-tick traces**, two per side, about 160 KB per side. They were
compared with `diff -r` and were identical, and their SHA-256 digests and line counts are in both
manifests, which is the oracle the contract names. `WO-MOK-004`'s evidence already holds the full
text of the same two runs. This is a deliberate omission of duplicated bytes, stated here rather
than left for a reader to notice.

Nothing here approves anything. These are observations of two trees.
