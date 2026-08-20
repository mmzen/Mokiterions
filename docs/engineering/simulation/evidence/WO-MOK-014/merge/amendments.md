# `VER-MOK-014` oracle 5, re-derived at the merge commit `9599c0a`

Oracle 5 asks whether the sixteen entries of `ADR-MOK-006`'s *Required amendments* are present and, where they land in
a governed artifact, approved. The contract states that **absence fails this contract regardless of code state**, so it
has to be re-asked at any commit a record binds — and this merge is the one act on this branch that touched amendment
records, so re-asking it here is not a formality.

`WO-MOK-014-amendments.md` in the directory above is the packet's answer at candidate `65ac88b`. This file is not a
replacement for it: it answers the same question at the merge commit and it re-derives the coordinates, because two of
the ones that file cites no longer resolve and neither of those two shifts is the merge's fault.

## The twelve that land in a governed artifact

Every one is present and `status = "approved"`, `updated = "2026-08-20"`, read from the frontmatter at `9599c0a`:

| # | Artifact | status | Amendment row or note, at `9599c0a` |
|---|---|---|---|
| 1 | `REQ-MOK-026` | `approved` | `requirements/REQ-MOK-026.md:23` |
| 2 | `REQ-MOK-050` (new) | `approved` | the artifact itself |
| 3 | `REQ-MOK-036` | `approved` | `requirements/REQ-MOK-036.md:23` |
| 4 | `ARCH-MOK-001` | `approved` | `architecture/ARCH-MOK-001.md:48` — **text corrected by this merge**, same line |
| 5 | `ARCH-MOK-002` | `approved` | `architecture/ARCH-MOK-002.md:42` |
| 6 | `ADR-MOK-001` | `approved` | `architecture/adr/ADR-MOK-001.md:22`, a dated note in *Status* |
| 7 | `ADR-MOK-003` | `approved` | `architecture/adr/ADR-MOK-003.md:29`, a dated note in *Status* |
| 8 | `SPEC-MOK-002` | `approved` | `specifications/SPEC-MOK-002.md:24` — **text corrected by this merge**, same line |
| 9 | `SPEC-MOK-003` | `approved` | `specifications/SPEC-MOK-003.md:68` and `:69` — **moved by the merge from `:58` and `:59`**, and both texts corrected |
| 10 | `SPEC-MOK-004` | `approved` | `specifications/SPEC-MOK-004.md:26` — **moved by the merge from `:25`**, text unchanged |
| 11 | `SPEC-MOK-005` | `approved` | `specifications/SPEC-MOK-005.md:23` and `:24` |
| 12 | `VER-MOK-014` (new) | `approved` | `verification/VER-MOK-014.md:20`, with `:21` and `:22` |

`ARCH-MOK-001`'s two replacement conformance checks, which the contract names separately because their absence is the
one way the amendment could be present and hollow, are at `:132` and `:133`, with the amended one-library-one-binary
check at `:138` and the decision-11 review at `:139`. All four are byte-identical to the candidate and at the same
lines: `ARCH-MOK-001` neither gained nor lost a line in the merge.

## The four with no authority

| # | Where | State at `9599c0a` |
|---|---|---|
| 13 | `docs/engineering/REPOSITORY_CONTEXT.md` | All three statements the ADR named are present, at `:18`, `:25` and `:39`. 70 lines at the candidate and 70 here; the single differing line is `:18`'s `REQ-MOK-047` → `REQ-MOK-050`, which is `4a32a95`'s renumbering. **The merge changed nothing in this file.** |
| 14 | `.github/workflows/release.yml` | blob `1ea115fb`, byte-identical to the candidate and different from the base — this branch's release-gate placement, untouched by the merge |
| 15 | `.github/workflows/dependency-declarations.yml` | blob `ac6d34e0`, byte-identical to the candidate, absent at the base. The managed `.github/workflows/engineering-harness.yml` is blob `91262ffc` at the base, the candidate **and** the merge commit: untouched, as the contract requires |
| 16 | Source comments | `mokiterions-core/src/lib.rs` and `mokiterions-tui/Cargo.toml` are byte-identical to the candidate. `mokiterions-core/Cargo.toml` differs on one line, `:43`, `REQ-MOK-047` → `REQ-MOK-050`, again `4a32a95`'s |

## Twenty-one cited coordinates, re-derived

All twenty-one coordinates the packet's oracle-5 evidence cites were resolved by content at `9599c0a`, not assumed:

- **14 hold exactly**, same line and same bytes: `REQ-MOK-026:23`, `REQ-MOK-036:23`, `ARCH-MOK-001:132`, `:133`,
  `:138`, `:139`, `ARCH-MOK-002:42`, `ADR-MOK-001:22`, `ADR-MOK-003:29`, `SPEC-MOK-005:23`, `:24`, `VER-MOK-014:20`,
  `:21`, `:22`.
- **2 hold their line and had their text corrected by this merge**: `ARCH-MOK-001:48` and `SPEC-MOK-002:24`. Both are
  amendment rows whose closing sentence asserted that a 2026-08-18 row remained OUTSTANDING; `master` ratified those
  rows, so the sentence became false and each was corrected in place. The correction is a single-line substitution by
  construction, which is why the coordinate survives.
- **3 moved, and the merge moved them**: `SPEC-MOK-003:58` → `:68`, `:59` → `:69`, `SPEC-MOK-004:25` → `:26`.
  `master` added 7 amendment rows and 3 lines above them to `SPEC-MOK-003`, hence 10, and 1 row to `SPEC-MOK-004`.
  The other four amendment records — `ARCH-MOK-001`, `ARCH-MOK-002`, `SPEC-MOK-002`, `SPEC-MOK-005` — gained no row
  and shifted no line, because `master`'s edits to them were in-place changes to a disposition cell.
- **2 were already stale at the candidate or became stale before the merge, and are corrected here:**

  | citation in `WO-MOK-014-amendments.md` | true coordinate at `9599c0a` | what moved it |
  |---|---|---|
  | `architecture/adr/ADR-MOK-006.md:77` | **`:84`** | `4a32a95`, this branch's renumbering, which added 7 lines to the ADR's *Status* above the note |
  | `work-orders/WO-MOK-014.md:87` | **`:93`** | `65ac88b`, the narrowing of the declared requirement set, which added 51 lines above the note |

  Traced commit by commit: the ADR note sat at `:77` through `3c3c2e4` and moved at `4a32a95`; the work-order note sat
  at `:87` through `f40b711` and moved at `65ac88b`. So one of the two was **already wrong at the candidate the
  verified record binds** and the other went wrong one commit later. Neither is the merge's doing, and neither is
  corrected in the file that carries it: `WO-MOK-014-amendments.md` is in `VREC-MOK-014`'s `evidence_paths` and that
  record is `verified` against `65ac88b`. The precedent is the one `evidence/WO-MOK-012/amendment-ratifications.md`
  states for `VREC-MOK-005` — a commit-bound record is not edited, because what it says was true of the tree it was
  taken from. Here even that is generous about the second one: `WO-MOK-014-amendments.md:87` was stale at `65ac88b`
  itself. It is a defect in a cross-reference, recorded rather than deleted, on the rule `WO-MOK-012` set for a
  miscount in a governance record. The amendment it points at is present, approved and unchanged; only the number
  beside it is wrong.

  `WO-MOK-014-merge.md`'s *Recorded coordinates the merge moved* lists both of these as unmoved. That is true of the
  merge and is what the sentence says, and it is also the kind of true statement a reader can be misled by, so that
  file is corrected in the same commit as this one to say **unmoved by the merge** and to name the two real
  coordinates.

## What this does not say

It does not ratify anything. Four amendment rows this branch wrote asserted that a 2026-08-18 provision was still
outstanding; the repository owner acting as technical owner ratified those provisions on 2026-08-20 under
`WO-MOK-012`, which is that work order's act and not this one's, and the sentences here were corrected to say so.
`VER-MOK-011`'s manual assessment 5 is untouched and still owed. `VER-MOK-014`'s manual assessment 7 is still found
not due rather than satisfied. And oracle 5 is a presence-and-approval check: it does not re-read the amended text
against the ADR's prescription, which is manual assessments 1 and 2's work and was recorded at the candidate.
