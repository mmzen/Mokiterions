# Evidence for WO-MOK-020

This directory retains implementation evidence for `WO-MOK-020`, *Make both help texts understandable*,
captured on 2026-08-22.

The records are observations of the working tree. They do not independently approve verification, create a
candidate commit, or authorize release. `VER-MOK-004` is the verification contract they serve; the
accountable assurance decision is the owner's act.

**`WO-MOK-020` is `draft` and five amendments are OUTSTANDING.** This evidence therefore documents a change
that is written and measured but not yet authorized, which is unusual for this directory and is the reason
`gates.md` records a `preflight` FAIL as a correct result rather than a problem. `WO-MOK-020`'s *Lifecycle*
section states how that came about: the owner directed the implementation on 2026-08-22 after being shown
both texts in full, and the amendments were found afterwards, in the course of writing the change against
the artifacts that govern it. No verification record exists, because a record binds a candidate commit and
there is nothing to bind while its oracle is unapproved.

## Commit binding

| Fact | Value |
| --- | --- |
| Base commit | `f7b1c452039dc2f03010ca8b8cc81e73c54727c0`, the branch point and the tip of `master` |
| Implementation branch | `feature/help-output-clarity` |
| Candidate commit | none; the work order is `draft` |
| Pre-change texts rendered from | a `git worktree` at the base commit, built and run there |
| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable, Python 3.14.6, se-harness 0.4.0 |

## Contents

- **`usage-text.md`** — both texts as rendered in full, the measurements, and the three implicit facts from
  `WO-MOK-020`'s *Objective* each traced to where it is now stated and to the approved artifact that
  governs it. Also records what the observer's text now discloses that its previous text did not, and why
  the four shared entries are a checked duplicate rather than a shared literal.
- **`gates.md`** — all six items of *Required verification*, including the one that FAILs by design.
- **`before/engine-usage.txt`**, **`before/observer-usage.txt`** — the texts at the base commit.
- **`after/engine-usage.txt`**, **`after/observer-usage.txt`** — the texts from the finished tree.
- **`engine-usage.diff.txt`**, **`observer-usage.diff.txt`** — the line-by-line diffs.
- **`drift-demonstration.txt`** — the cross-target identity test failing on a one-character divergence,
  captured rather than asserted. The scratch change that produced it was reverted and is not committed;
  `gates.md` item 4 records both checks that confirm it.

## A note on line endings

The repository is worked on under Windows with `core.autocrlf = true`, so every text file here is stored in
the object database with `LF` and appears in a Windows working tree with `CRLF`. Re-hashing a retained file
after a checkout will therefore not reproduce a digest taken before it, which is the checkout's conversion
and not drift. `WO-MOK-003`'s and `WO-MOK-004`'s evidence is stored the same way.

The measurement that survives that conversion is in `usage-text.md`: the bytes both programs emit contain
**zero** carriage returns, on this machine and on any other, because both `USAGE` constants are built from
one string literal per line with an explicit `\n` rather than from a multi-line literal that would inherit
its line endings from the checkout. The observer's constant acquires that property under this work order;
the engine's already had it.
