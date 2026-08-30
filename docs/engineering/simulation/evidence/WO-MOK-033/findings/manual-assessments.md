# The eight manual assessments: not recorded

`VER-MOK-019` reserves eight assessments to the product owner, the technical owner and the assurance
owner, and states plainly: *an unsigned assessment is not a recorded one*. None is recorded here. An
implementation agent cannot sign any of them, and the three owner roles are all held by one person, so
none can be substituted by another role either.

The mechanical cases -- all 84 of them -- are executed and passing. These eight are the judgements
those cases cannot make. `VREC-MOK-027` discloses this gap rather than closing it.

| # | Assessment | Role | Read |
|---|---|---|---|
| 1 | Does the distribution answer Phase 6's question? | product owner | `phase-f/scenario-1-distribution.txt`, `phase-f/distribution-source.md`, README's distribution table |
| 2 | Is the class vocabulary the right one? | product owner | `phase-f/distribution-source-density.json` rows, not `SPEC-MOK-008` rule 16.5's text |
| 3 | Is the famine disclosure honest and sufficient? | technical owner | `phase-f/famine.txt`, `findings/disclosed-gaps.md` item 9 |
| 4 | Is the attempted-and-effective split justified by its measurement? | technical owner | `phase-f/threat-inertness.txt`, `findings/sweep-findings.md` item 3 |
| 5 | Is `ADR-MOK-008`'s "no amendment required" claim sound? | assurance owner | `phase-d/t1-census.txt`, `t2-touched.txt`, `t6-target-census.txt`, against `ARCH-MOK-001`, `SPEC-MOK-004` rule 1 and `ADR-MOK-003` decision 1 |
| 6 | Is Python becoming load-bearing for published results accepted? | technical owner | `harness/` as built, `phase-d/t4-declared-dependencies.txt`, `t5-imports.txt` |
| 7 | Are the reported measurements reproducible? | assurance owner | recompute one digest from `manifest.sha256` and one derived count from `phase-f/default-sweep.jsonl`, independently of any retained command output |
| 8 | Does the threat disclosure meet the 2026-08-30 decision? | assurance owner | `findings/sweep-findings.md` item 3, `phase-f/threat-inertness.txt` |

Assessment 5 is worth a note. `ADR-MOK-008` claims no amendment is required to `ARCH-MOK-001`'s
prohibited-patterns list, `SPEC-MOK-004` rule 1 or `ADR-MOK-003` decision 1. Cases T1 to T6 measure
what can be measured: the workspace has two members and no third package, the structural census is 40
paths blob-for-blob identical to `c90edc9` with a clean worktree over all of them, no product file is
touched, both packages' dependency tables are byte-identical and their declared sets unchanged, every
import of all four Python files resolves to the standard library, and neither instrument is a build
target. Whether that is the same thing as *none of the three is touched* is the judgement, and it is
the one the mechanical cases cannot make.
