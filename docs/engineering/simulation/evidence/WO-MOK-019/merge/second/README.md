# The second merge of `master`, and the renumbering that had to precede it

| Field | Value |
|---|---|
| Merge commit | `e96648a` — "Merge master into wip/pr31-integration for WO-MOK-019" |
| First parent | `efe20e3` — this branch, renumbered from `WO-MOK-018` to `WO-MOK-019` |
| Second parent | `7f4792a` — `master`'s tip, "Merge pull request #37 from mmzen/feature/observer-fear-and-filter-count" |
| Merge base | `fa065cc27aa250bd93c586b0c61da789dab49e33` — `master`'s tip at the first merge |
| Branch | `wip/pr31-integration` |
| Date | 2026-08-21 |
| Toolchain | cargo 1.97.1 (c980f4866 2026-06-30); rustc 1.97.1 (8bab26f4f 2026-07-14) |
| Harness | se-harness 0.4.0, from `C:\Users\mathi\harness-venv-040` — the version this repository's workflows pin |

`../README.md` is the account of the **first** merge, `1e09f85`, measured at `e8114ad`. It stays as
written except where the renumbering falsified a quotation, and it says where. This directory is
the account of the second, and it is short for a reason it states and bounds: `gates.txt` §*Why this
is a short file and not a second full packet*.

## This is not a verification record and it takes no decision

Every file here is derived, read-only evidence: a command, its output, and the reasoning needed to
read the output. **`VREC-MOK-019`**, bound to the merge commit, is where a decision about this tree
is recorded, and it is not written yet.

## Why there is a second merge at all

`master` moved again while the first merge's evidence was being corrected. Pull request #37 —
*"Close the two observer defects Phase 3.1 left"* — merged as `7f4792a`, and it brought a work order
numbered **`WO-MOK-018`**, which is the identifier this chain had renumbered *itself* into a few
hours earlier on the owner's decision "018, nothing else moves".

`../README.md` finding 10 is where that collision was reported. It is now closed, by moving this
chain again, to **`WO-MOK-019`**, at `efe20e3`. `../../renumbering.md` is the record of that act:
the derivation of the collision set, the sweep, and what it disclosed. Two things about it belong
here rather than there, because they are properties of *this merge*.

**The collision set was one identifier, and it was derived rather than assumed.** An identifier
present on both `efe20e3` and `7f4792a` but absent at the merge base `fa065cc` is an independent
creation on each side. Exactly one qualified: `WO-MOK-018`. `VREC-MOK-017` and `VREC-MOK-018` are
`master`'s alone; this chain declares neither. That is why this renumbering moved one identifier
where the `013 → 014` precedent moved four.

**What the collision would have cost is measurable, and it is worse than a conflict.** Merging the
pre-renumbering tip `f40fead` against `7f4792a` — `git merge-tree --write-tree --name-only`, which
computes a merge without touching a working tree — reports five conflicted paths:

    CONFLICT (add/add)   work-orders/WO-MOK-018.md
    CONFLICT (add/add)   evidence/WO-MOK-018/README.md
    CONFLICT (add/add)   evidence/WO-MOK-018/gates.txt
    CONFLICT (add/add)   evidence/WO-MOK-018/interface.txt
    CONFLICT (content)   specifications/SPEC-MOK-004.md

Three of the four add/add conflicts are evidence files, and **they are three of one hundred and
twenty**. `master`'s packet holds 16 files, this chain's 104, and only those three basenames
collide. The other thirteen of `master`'s and the other hundred and one of this chain's do not
conflict, because at each of those paths only one side has a file — so git merges them, correctly
and silently, into **one directory of 117 files presenting itself as one work order's evidence**.
The conflict markers would have named 3 files and the damage would have covered 117. The same
reading applies to `work-orders/WO-MOK-018.md`: one file, resolved either way, and *"Emit a
structured record stream to an operator-named sink"* and *"Close the two observer defects Phase 3.1
left"* would have had one requirement set, one verification plan and one status between them.

After the renumbering the same command reports **one** conflicted path, `SPEC-MOK-004.md`, and both
packets coexist: `evidence/WO-MOK-018/` is `master`'s sixteen files and `evidence/WO-MOK-019/` is
this chain's hundred and four. `governance.txt` measures the same result from the other end: 146
artifacts, where a resolved collision would have counted 145.

## The one conflict, and how it was resolved

`SPEC-MOK-004.md` conflicted in five regions, all of them figures and none of them provisions. No
target, target name, path, package name, tier boundary, hook or prohibition is touched by the
resolution, and no item's visibility widens.

Both sides had amended the same document on 2026-08-21, and the two amendments are not of equal
standing. `master`'s two rows are **approved** — ratified by the repository owner as technical
owner. This chain's row read **OUTSTANDING**. The resolution follows from that asymmetry rather
than from a preference:

| Region | What collided | Resolution |
|---|---|---|
| 1 | The amendment record's rows | This chain's earlier row is kept, `master`'s approved row is taken, and a new 2026-08-21 row for this merge is appended. This chain's own 2026-08-21 draft row is **replaced**. |
| 2 | Rule 9's public-tier table | `master`'s approved figures, unchanged. |
| 3 | Rule 9's paragraph | `master`'s approved text, unchanged. |
| 4 | Rule 10's private-item paragraph | `master`'s approved text, plus a new paragraph for the figure the approved text does not state. |
| 5 | Rule 11's totals | This chain's candidate-tree paragraph and table are kept; the first-merge paragraph is superseded by a second-merge paragraph; `master`'s approved paragraph stands between them. |

**The principle, stated so it can be disagreed with:** where approved text and an unratified draft
answer the *same* question, the approved text stands and the draft is withdrawn rather than merged
into it; where the two sides are about *different* figures, both survive. Region 1 is the case that
makes this matter. This chain's draft row was written to answer the referral `WO-MOK-016` made about
which work order re-derives rule 11's figures — and the approved row above it has since answered
that referral, on the owner's authority, under `master`'s `WO-MOK-018`. Keeping both would leave two
unratified answers to a question already decided. So the draft is withdrawn, and what survives of it
is the one figure the approved row does not state: rule 10's **49**.

**One figure in the resolution is a defect no record has reported, and it is flagged as such.**
Rule 10's private-item count reads 48 and the measured figure is **49** — 31 private functions and
18 private constants in `mokiterions-tui/src/render.rs`. It is 49 on the merged tree, on `master`
at `7f4792a` and on this chain at `efe20e3`: the same on every tree that exists, so the 48 is one
low everywhere and not an artefact of either merge. It is not on `VREC-MOK-016`, whose packet names
the private `action_text` in a change-surface table and connects it to no item count, and it is not
in `master`'s approved row. The owner is being shown it here for the first time, which is a weaker
position than a correction already reported, and the new row says so.

The new row and both new paragraphs read **OUTSTANDING**. They are drafted under the procedure the
owner set on 2026-08-21 — the agent drafts each correction, the owner ratifies each — and they are
owed a ratification that has not been given.

## What this merge did not have to re-derive, and why

`gates.txt` records the licence in full; the measurement is two commands that print nothing:

    git diff efe20e3 -- mokiterions-core          empty
    git diff origin/master -- mokiterions-tui     empty

Pull request #37 changed four files, all of them the observer's, and no file of the engine. The
record stream is the engine's output, so the 120 capture cells, six oracles and 303 MB of stream in
`../oracle1/` through `../oracle6/` are figures at this tree as well. In the other direction the
observer half is `master`'s byte for byte, which is why rules 9, 10 and 6 take `master`'s approved
figures — re-measured rather than carried over, and they agree.

**The two empty diffs are the whole of the licence.** Had either been non-empty this directory would
not exist and the captures would have had to be re-taken.

## The gates

    cargo test --workspace                       ok. 301 passed; 0 failed; 0 ignored
    validate .                                   PASS   146 artifacts | Errors 0 | Warnings 0
    doctor .                                     PASS   81 lines, every one PASS
    preflight . --work-order WO-MOK-019
              --phase review                     PASS   Work order: WO-MOK-019 (implemented)
    dashboard . --output <outside the checkout>  PASS   146 artifacts | 523 relations
                                                        Errors 0 | Warnings 17

`preflight`'s `Work order: WO-MOK-019 (implemented)` is the line that closes the sweep: the command
resolves the work order from the identifier it is given, and a renumbering that had left a dangling
reference anywhere in 146 artifacts would have failed to resolve it.

## Files in this directory

| File | Bytes | What it is |
|---|---|---|
| `README.md` | this file | The account of the second merge |
| `gates.txt` | 6,528 | The gates, their outputs, and the carry-forward licence |
| `governance.txt` | 11,336 | The artifact graph across four trees, the queues, and the findings rule by rule |
| `test-run.txt` | 26,795 | `cargo test --workspace` in full on the merged tree |
| `test-census.txt` | 22,997 | `cargo test --workspace -- --list` in full on the merged tree |

Byte counts are of the files as written, before this table was added to `README.md`; the two `.txt`
capture files are unaffected by it.

## What the second merge leaves owed

Everything `../README.md` §*What is still owed* lists remains owed, less finding 10, which
`../../renumbering.md` closes. Added by this merge:

1. **Ratification of the new 2026-08-21 `SPEC-MOK-004` row** and the two paragraphs it covers,
   including the two things the row puts explicitly: that this chain's own draft row is replaced
   rather than kept, and that rule 10's 48 is an unreported defect.
2. **`VREC-MOK-019` at `e96648a`**, declaring `evidence/WO-MOK-019/merge/`. `VREC-MOK-012` at
   `50364a3` and `VREC-MOK-016` at `4539601` are not re-pointed and not re-opened — a record binds
   the commit it was written against, and there is no rebinding.
3. **`ARCH-MOK-002` reassessed against `SPEC-MOK-004` as amended** — `governance.txt` records the
   `W-HEX-003` observation and why it is not merely a date artefact. Owed to that document's owner.

## Authority

Every command in this directory is derived, read-only evidence, and each prints its own boundary.
Validation does not approve. Inspection does not validate by exit status, approve, authorize,
verify, release or remediate. Preflight does not approve artifacts, authorize a diff, verify work,
release software, commit, push, tag, publish or deploy. Nothing here records a decision, ratifies a
figure or closes an obligation. `VREC-MOK-019`, at the merge commit, is where a decision about this
tree is recorded, and it binds a commit.
