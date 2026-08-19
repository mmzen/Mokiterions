# WO-MOK-005 evidence packet

`VER-MOK-005`'s evidence-retention list has thirteen bullets. Each is below with the file that
discharges it, so retention completeness is checkable without reading the packet. The count was
twelve when this file was written and the 2026-08-19 amendment added the monotonicity-sweep bullet;
this table names thirteen because the list does.

| Retention bullet | File |
|---|---|
| Formatter, linter, test and build output for the workspace and for the engine package alone | `static-checks.txt`, `test-run.txt` |
| The requirement-to-test mapping | `requirement-to-test-mapping.md` |
| `cargo tree -p Mokiterions` output, as the empty-dependency-set proof | `dependency-review.txt` |
| The observer's resolved dependency graph, measured crate count and enabled feature set | `dependency-review.txt` |
| Per-seed observed-versus-unobserved comparison, with method and interaction performed | `non-perturbation.txt` |
| Per-tick entropy draw-count comparisons | `non-perturbation.txt` — compared as record identity, not a counter; see disclosure 13 |
| Per-viewport layout and canvas-interior assertions, and the `33 × 21` refusal output | `layout-and-viewports.txt`, `frames.txt` |
| The layout monotonicity sweep: bounds swept, pairs checked, and the count of pairs where enlarging removes a pane | `layout-and-viewports.txt` — `34 ≤ W ≤ 200` by `22 ≤ H ≤ 60`, **13,026** adjacent pairs, **0** violations, against **12** for the superseded table |
| One exported event file per declared seed, and the byte-comparison against the engine binary's stream | `exports/` (5 files), `export-fidelity.txt`, `exports.txt` |
| The `10,000`-tick resilience result and terminal restoration for normal, error and panic exit | `resilience.txt`, `terminal-restoration.txt` |
| The manual assessment record, including legibility and colour-independence, and their author | `manual-assessment.md` — **all seven outstanding, no author** |
| Dependency, boundary and credential review | `dependency-review.txt`, `boundary-and-security-review.md` |
| A completion summary naming the final affected components | `completion-summary.md` |

`additivity-proof.txt` is not on the retention list. It is here because `WO-MOK-005` makes a changed
existing test a stop condition, and a passing test count does not establish that nothing changed. It
compares the whole engine package byte for byte against `origin/master` at 05dc6ac, which is an
ancestor of this branch's head, and then compares the observer's test-name set file by file, because
three observer tests did change and each one's authority has to be stated rather than assumed. Its
subject and its baseline are both wider than when this line was written: it named the engine's test
files against 903c9943, at a time when the observer package was not on `master` at all.

## Read these three first

- `completion-summary.md` — the final components, the gate results, and seventeen disclosures. Four
  are artifact errors that should be corrected: `ADR-MOK-003`'s `Cargo.lock` figure, the ambiguous
  57-crate measurement, the rule 4 / rule 5 roster-width contradiction, and rule 2's two glyph
  tables disagreeing about `M10`–`M12`. Items 16 and 17 are this revision's: the defect the owner
  reported, and the one consequence of fixing it that no test asserts.
- `manual-assessment.md` — the packet's completeness gap. Every manual assessment `VER-MOK-005`
  requires is unperformed, three of them because this environment cannot deliver a keypress to the
  observer at all.
- `boundary-and-security-review.md` — the surface claims `SPEC-MOK-003` makes, and the one that does
  not hold as written: the public surface has two mutating operations, not one.

## What none of this establishes

Every automated result in this packet is an assertion about an in-memory character buffer.
`VER-MOK-005` forbids screenshots and recordings as evidence, so buffer dumps are what `frames.txt`
carries — and a correct buffer is not evidence that a person can read the rendered result. That gap
is what the outstanding manual assessments exist to close.
