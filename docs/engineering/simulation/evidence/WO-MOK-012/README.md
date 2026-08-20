# Evidence — WO-MOK-012

`WO-MOK-012` records `VER-MOK-005`'s seven manual assessments and ratifies the eleven amended provisions they were
blocked behind. This directory is its retained evidence.

**What this work order is not.** It changes no executable behavior. Nothing under `mokiterions-core/` or
`mokiterions-tui/` is modified, and `completion-summary.md` records the diff check that establishes it. The three
adverse observations the assessment pass produced are recorded here and **fixed by a later chain**, by decision 12.

**That chain is now drafted.** `WO-MOK-013`, `VER-MOK-013` and `REQ-MOK-047` through `REQ-MOK-049` implement decisions 13
to 15, all `draft` and awaiting approval. Two facts found while authoring it belong to this directory rather than to it:
the identifier collision in `identifier-collision.md`, and that **decision 13 as recorded cannot be implemented without
amending approved `REQ-MOK-020`** — the geometry is in `WO-MOK-013`'s *Authorized decision envelope* and the collision
was not measured when the decision was taken.

## The files

| File | What it establishes |
|---|---|
| `closing-review.md` | The fifteen decisions the repository owner took on 2026-08-20, each with the role acted in and what the owner was shown before answering. The record of the acts. |
| `assurance-decision.md` | Decisions 16 and 17, taken in a later turn the same day: the approval of this work order and the confirmation of its `commit_bound_verification` classification. Records the instruction verbatim and what it did **not** authorize. |
| `amendment-ratifications.md` | The eleven ratified provisions, one by one: what each says, what it changes, and why ratification rather than revision was the right act. |
| `manual-assessment.md` | The seven assessments of `VER-MOK-005`, with status and author. Six authored by the owner; the seventh **outstanding by decision**, with its reason. |
| `adverse-observations.md` | The three findings from the live pass, each with the measurement that establishes it, the specification provision it bears on, and the remedy decided for the later chain. Plus two further observations recorded but not raised as findings. |
| `procedure-defects.md` | Three defects in `WO-MOK-005`'s recorded assessment procedure, each verified against `master` at `ff3a155`. |
| `identifier-sweep.md` | That `WO-MOK-012` was free across all 24 remote heads on 2026-08-20, and that `master` moving from `dec1b95` to `ff3a155` mid-task left the captures and line numbers in this directory valid. |
| `identifier-collision.md` | That `WO-MOK-012` **stopped being free later the same day**. A Phase 4a packet on `origin/feature/phase-4a-definition`, authored by another agent from the same base, claims the identifier for a different work order. Records both sides, and that the resolution is the owner's. |
| `assessment-material/` | Twelve files: ten terminal captures and two oracle sources. What the buffer held, so that analysis is a separate act from measurement. |

## `assessment-material/`

Ten `.txt` captures and two `.rs` oracle sources.

| File | Contents |
|---|---|
| `frame-160x48-tick200.txt` | The reference viewport at tick 200, whole frame |
| `panes-160x48-tick200.txt` | The same frame by pane — the source for assessments 1, 4 and 5 |
| `palette-160x48-tick200.txt` | Every colour in the frame, by cell |
| `modifiers-160x48-tick200.txt` | Every modifier in the frame, by cell |
| `reversed-and-underlined-160x48.txt` | `REVERSED` and `UNDERLINED` on one cell — seed 0, tick 18, `M09` selected |
| `underline-search.txt` | The sweep that found a co-occupancy cell, `UNDERLINED` being absent from the whole seed-42 run |
| `rejection-160x48.txt` | The inspector presenting a rejected proposal, reached through `replace_decisions_for_test` |
| `overlays-and-zoom-160x48.txt` | Each overlay and both zoom levels |
| `announcement-at-reduced-viewports.txt` | Nine viewports from 160 × 48 down past the 34 × 22 floor, with the header announcement drawn at each |
| `bar-quantization.txt` | `bar_width(45) = 2` and the three states a two-cell bar draws for 101 values |
| `assessment-oracle.rs` | The oracle that produced the frame, pane, palette, modifier, overlay and rejection captures |
| `announcement-oracle.rs` | The oracle that produced the reduced-viewport captures |

### On the two oracles

Each was placed in the tree as a `#[cfg(test)] mod` under `mokiterions-tui/src/`, run once, and removed. **The sources
are retained here; neither is in the tree.** `completion-summary.md` records the diff that establishes the tree is
byte-identical to `ff3a155` after their removal.

They are inside `src/` rather than a standalone binary because the captures need `#[cfg(test)]` hooks —
`replace_decisions_for_test` for the rejection capture — and a hook gated on `cfg(test)` cannot be linked from outside
the crate.

This follows the precedent of `WO-MOK-010`'s `observer/frame-probe.rs` and `WO-MOK-006`'s
`frame-and-export-oracle.rs`: **an oracle asserts nothing.** It writes down what the buffer holds, so that judging what
the buffer means is a separate act performed by a person who can be named.

## Reading order

`closing-review.md` first — it is the record of what was decided and by whom. Then whichever of
`amendment-ratifications.md`, `manual-assessment.md` or `adverse-observations.md` covers the decision at hand.
`completion-summary.md` states what remains open.

## What is not here

- **No commit hash of this work order's own commit.** A record cannot contain the hash of the commit that introduces it.
- **No verification record.** `WO-MOK-012` classifies `commit_bound_verification` as `not_required` and stops at
  `implemented`, being governance-only work. **The engineering owner confirmed that classification on 2026-08-20**, as
  a separately stated act; the implementation agent proposed it and held no authority over it. See
  `assurance-decision.md`.
- **No status transition, approval, assessment or release act taken by the agent.** Every decision recorded here was
  taken by the repository owner and is attributed.
