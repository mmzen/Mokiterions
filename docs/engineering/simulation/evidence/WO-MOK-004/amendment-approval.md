# WO-MOK-004 evidence: the amendment was approved before the text changed

`VER-MOK-004`'s static checks state: "`SPEC-MOK-001`'s *Help output* amendment is present and approved
**before** the usage text changes. Its absence fails this contract regardless of code state, because
without it the new content has no specified authority."

Ordering, not merely presence, is the claim. This is the record of it.

## The sequence

| Step | Act | Date |
| --- | --- | --- |
| 1 | `REQ-MOK-018`, `VER-MOK-004`, and `WO-MOK-004` created at `draft`. Validator PASS, 40 artifacts, 0 errors, 0 warnings; all three listed under *Definitions pending*. | 2026-08-17 |
| 2 | `harnessctl preflight --phase start --work-order WO-MOK-004` run against the drafts. **FAIL**, reporting `W005` (status not eligible), `W013` twice (governing artifacts not active), and `W016` (specification coverage missing `REQ-MOK-018`). | 2026-08-17 |
| 3 | `SPEC-MOK-001` amended: `specifies` gained `REQ-MOK-018`; a sixth amendment-record row added; the `### Help output` section inserted; the *Explicitly unspecified decisions* entry on help text narrowed to alignment, width, and wrapping. Approved by the repository owner acting as technical owner. | 2026-08-17 |
| 4 | `REQ-MOK-018` and `VER-MOK-004` approved; `WO-MOK-004` approved and moved to `in_progress`. Validator PASS, 40 / 0 / 0. `preflight --phase start` **PASS**. | 2026-08-17 |
| 5 | Pre-change baseline captured, with `git status --short -- src tests Cargo.toml Cargo.lock` confirming 0 modified source files. | 2026-08-17 |
| 6 | **First edit to `src/cli.rs`.** | 2026-08-17 |

Step 2 is the part worth keeping. The `W016` finding is independent evidence that the specification did
not yet cover the requirement — the tool said so before the amendment existed, and said nothing after it
did. The amendment is therefore demonstrably prior to the code change rather than asserted to be.

## The amendment, as approved

`SPEC-MOK-001` gained a `### Help output` section between the *Inputs* list and *Outputs*, stating the
ordered content of the usage text, the six-option ordering, and a three-column table of stated default and
stated constraint per option, followed by the equality clause — "Every default stated in the options block
is the value the program applies when the option is omitted … verified rather than maintained by
convention" — and the state-each-fact-once clause.

Its amendment-record row reads:

> Specified the content of the help text: an options block stating each accepted option's effect, its
> default where it has one, and its value constraint where it has one, with the stated defaults required
> to equal the applied defaults. Narrowed the *Explicitly unspecified decisions* entry on help text to
> alignment, width, and wrapping. No simulation behavior changed. | Approved 2026-08-17 by the repository
> owner acting as technical owner, together with `REQ-MOK-018`, `VER-MOK-004`, and `WO-MOK-004`, as an
> approval precondition of that work order.

`specification-to-help.md` diffs that section against the rendered help: no shortfall, no surplus, no
defect.

## No other artifact was amended

- **`SPEC-MOK-002` was not amended.** Its rule 11 — "The restructuring is equivalence-preserving … no
  byte of `USAGE` changes" — and `VER-MOK-003`'s *Behavior surface unchanged* invariant are scoped to
  `WO-MOK-003`'s restructuring, a discharged contract. They constrain that work order, not future ones.
  Rules 5 through 8, which do bind this work, are satisfied unchanged: the public interface is the same
  closed enumeration (`public-surface-inventory.md`), and the added tests sit in the public tier under an
  existing subject (`test-census.md`).
- **No architecture or ADR was engaged.** `ARCH-MOK-001` addresses `REQ-MOK-004`, `REQ-MOK-008`,
  `REQ-MOK-009`, `REQ-MOK-010`, and `REQ-MOK-016`, and does not address `REQ-MOK-018`. `WO-MOK-004`
  records the deliberate omission together with the counter-argument, so a reviewer who disagrees has the
  reasoning in front of them; the remedy would be adding coverage, not changing the work.
- **No prior evidence directory or verification record was opened for writing.**
  `WO-MOK-001`/`002`/`003` evidence and `VREC-MOK-001`/`002`/`003` are bound to their own commits and are
  untouched. `git status` confirms the only modified tracked file outside `src` and `tests` is
  `SPEC-MOK-001.md`.

## Governance state at completion

| Check | Result |
| --- | --- |
| `validate_engineering_artifacts.py` | **PASS** — 40 artifacts, 0 errors, 0 warnings; structure, governance, policy, maintenance all E0/W0 |
| `harnessctl preflight --phase review --work-order WO-MOK-004` | **PASS** — work order `in_progress`, commit-bound verification required |
| `harnessctl doctor` | **PASS** — every managed file matches distribution; no managed file was edited |
