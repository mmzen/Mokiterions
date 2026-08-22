# Evidence for WO-MOK-024

This directory retains implementation evidence for `WO-MOK-024`, *Make both help texts understandable*,
captured on 2026-08-22.

The records are observations of the working tree. They do not independently approve verification, create a
candidate commit, or authorize release. `VER-MOK-004` is the verification contract they serve; the
accountable assurance decision is the owner's act. `VREC-MOK-022` cites this directory and stands at status
`ready` at the commit it binds, which means prepared and not accepted — no file here anticipates the outcome
of that acceptance, and none was written or altered after it. The acceptance came afterwards, on 2026-08-22,
and it is recorded in the record itself rather than backfilled here.

**The five amendments were ratified on 2026-08-22 and `WO-MOK-024` is now `implemented`.** This directory
was written before that, while the work order was `draft` and all five amendments were **OUTSTANDING**, and
it is left readable in that order rather than rewritten to look as though authorization came first. What
actually happened is in `WO-MOK-024`'s *Lifecycle*: the owner directed the implementation on 2026-08-22
after being shown both texts in full; the amendments were found afterwards, in the course of writing the
change against the artifacts that govern it; and the owner then ratified all five in one act, as written,
and directed the transition. `gates.md` therefore records **three** `preflight` readings rather than one —
the `draft` FAIL, the `implemented` FAIL on the same `start` phase, and the `review` PASS that is the gate
this status is measured by. The middle one exists because the work order first claimed the FAIL "clears on
this transition", which measurement contradicted.

**A sixth amendment was found after all of this and ratified the same day.** Executing `VER-MOK-004` to
prepare the record — items 7 to 9 of *Required verification*, the three files added last — found three of
its checks misaligned with this change and with the tree: a false width assertion that also contradicts
ratified amendment 2, a test-placement row naming the wrong file, and survivor figures master has moved
past. That is amendment 6 of `WO-MOK-024`, and the owner ratified all three parts as acts 7, 8 and 9 of its
*Decision record*, in the assurance owner role, having declined for each the alternative of leaving the row
unmet. It changed no code and no rendered text, and **no measurement in this directory was taken or altered
to suit it**: the figures were captured first and the amendment states them, which is the order that makes
them evidence rather than decoration. `VER-MOK-004`'s *Second amendment of 2026-08-22* is the amendment
itself.

**The record was captured four times, and only the fourth survives.** The first binding was overtaken by a
correction to `usage-text.md`, the second by this amendment, and the third by the renumber below, which moved
every one of the record's twenty-six evidence paths. A verification record cannot be re-pointed at a later
commit or re-pathed after capture, so each was discarded and re-taken rather than edited. All four stood at
`ready` when taken. Act 10 of *Decision record* is the owner's direction for the third; act 11 is what forced
the fourth.

**This directory was `evidence/WO-MOK-020/` until 2026-08-22.** The work order was renumbered
`WO-MOK-020` → `WO-MOK-024` and its record `VREC-MOK-020` → `VREC-MOK-022`, as act 11 of *Decision record*,
because a fetch of the remote found both original identifiers already taken: `VREC-MOK-020` by a `verified`
record on `master` that binds `WO-MOK-017`, and `WO-MOK-020` by an open draft pull request on another branch.
Nothing was re-measured to suit the renumber and nothing needed to be — it changes two identifiers and one
directory name. The full gate set was re-run against it anyway, because four `.rs` doc comments name the work
order: formatting clean, clippy clean, **303 tests pass with 0 failed and 0 ignored**, `validate` 148
artifacts with 0 errors and 0 warnings, `doctor` 81/81 PASS, `preflight --phase review` PASS. Both help texts
were re-rendered from the renumbered tree and are **byte-identical** to `after/engine-usage.txt` and
`after/observer-usage.txt`, which is the check that matters: no rendered text contains the identifier, so
none of it moved. The three fenced `preflight` transcripts in `gates.md` still say `WO-MOK-020`, and that is
deliberate; `gates.md`'s own identifier note says why.

## Commit binding

| Fact | Value |
| --- | --- |
| Base commit | `f7b1c452039dc2f03010ca8b8cc81e73c54727c0`, the branch point and the tip of `master` |
| Implementation branch | `feature/help-output-clarity` |
| Candidate commit | the commit `VREC-MOK-022` binds. Its `commit` field is the authority and this table deliberately does not copy the hash: the commit that carries this file cannot state its own identity, and a second copy would be free to drift from the record's |
| Pre-change texts rendered from | a `git worktree` at the base commit, built and run there |
| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable, Python 3.14.6, se-harness 0.4.0 |

## Contents

- **`usage-text.md`** — both texts as rendered in full, the measurements, and the three implicit facts from
  `WO-MOK-024`'s *Objective* each traced to where it is now stated and to the approved artifact that
  governs it. Also records what the observer's text now discloses that its previous text did not, and why
  the four shared entries are a checked duplicate rather than a shared literal.
- **`gates.md`** — all nine items of *Required verification*, including the one that FAILs by design, and
  the three capture points: the code gates at `6d6be40`, the harness readings re-run after ratification, and
  items 7 to 9 run after `6d5b532`. It also states which of the contract's rows the evidence does **not**
  satisfy.
- **`preflight-implemented.txt`** — both post-ratification `preflight` runs verbatim, `--phase start`
  FAILing at exit 1 and `--phase review` PASSing at exit 0, with the version banner and the exit codes. Both
  were re-run after the renumber and lead the file; the superseded pre-renumber capture is retained verbatim
  beneath a divider rather than deleted.
- **`before/engine-usage.txt`**, **`before/observer-usage.txt`** — the texts at the base commit.
- **`after/engine-usage.txt`**, **`after/observer-usage.txt`** — the texts from the finished tree.
- **`engine-usage.diff.txt`**, **`observer-usage.diff.txt`** — the line-by-line diffs.
- **`drift-demonstration.txt`** — the cross-target identity test failing on a one-character divergence,
  captured rather than asserted. The scratch change that produced it was reverted and is not committed;
  `gates.md` item 4 records both checks that confirm it.
- **`defaults-divergences.txt`** — the three divergences of `VER-MOK-004` acceptance scenario 3, re-run
  against the **rewritten** text rather than cited from `VREC-MOK-004`: the applied `--ticks` default moved
  with the text left alone, the printed `--seed` default moved with the parser left alone, and a parser arm
  added with no help entry. The second is the load-bearing one — it fails exactly one test of eighteen.
- **`resilience-and-interface.txt`** — the 10,000-tick runs under all four decision sources, the 1,000-tick
  survivor floor, and the public-surface check. Also reports two figures the contract fixes that master has
  since moved past, and one inherited rustdoc warning.
- **`nonperturbation/`** — `VER-MOK-004`'s 43-cell matrix and its 16 named cases, executed on a worktree at
  the base commit and on the candidate. All 43 cells byte-identical; one of sixteen cases changed, and it is
  `--help`. See that directory's own `README.md`.

## A note on line endings

**Corrected 2026-08-22.** This section previously said that every text file here is stored in the object
database with `LF`, appears in a Windows working tree with `CRLF`, and therefore cannot be re-hashed to a
digest taken before a checkout. That is the behavior `.gitattributes` exists to prevent, and it is not what
this directory does.

`.gitattributes` carries `docs/engineering/simulation/evidence/** -text`, added while committing
`WO-MOK-010` after `WO-MOK-006`'s retained stream hashed differently from its own manifest. `-text`
disables end-of-line conversion in both directions, so **a retained file is stored exactly as it appears in
the working tree and comes back out of a checkout unchanged, on any platform**. Re-hashing a retained file
does reproduce a digest taken before a checkout, which is the whole point.

What follows from that is a rule for this directory rather than a caveat: the endings a file is written
with are the endings it keeps forever. The text files here are `CRLF`, matching the Windows working tree
they were captured in and matching `WO-MOK-004`'s retained evidence. The two shell scripts,
`nonperturbation/capture.sh` and `nonperturbation/resilience-10k.sh`, are `LF`; `capture.sh` is `LF`
because it is a byte-for-byte copy of `WO-MOK-004`'s, and `cmp` reports no difference.

The measurement that survives that conversion is in `usage-text.md`: the bytes both programs emit contain
**zero** carriage returns, on this machine and on any other, because both `USAGE` constants are built from
one string literal per line with an explicit `\n` rather than from a multi-line literal that would inherit
its line endings from the checkout. The observer's constant acquires that property under this work order;
the engine's already had it.
