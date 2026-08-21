# The renumbering of this chain from `018` to `019`

This file records a governance act taken on the packet itself rather than a measurement. It is
retained for four reasons: it is the **second** time this chain's work-order identifier has moved,
and a reader who meets `WO-MOK-012` and `WO-MOK-018` in this directory needs to know which is which;
`master` now holds a *different* work order under `WO-MOK-018`, so following that identifier out of
this packet leads to unrelated work; one hundred and thirty-two occurrences of the old identifier
were inside retained tool captures, and this packet rewrote them, which is a departure from the
repository's most recent renumbering precedent and is put to the owner below rather than buried; and
the record this chain closes with, `VREC-MOK-012`, was already **`verified`** when the rename was
applied, so the subject of a recorded assurance decision moved after the decision was taken — for
the second time.

`merge/second/README.md` is the account of the merge this act made possible. This file is the account
of the act.

## What happened

This work order was drafted, approved and implemented as **`WO-MOK-012`**, verified by
**`VREC-MOK-012`** at commit `50364a3`, with `VER-MOK-012`, `SPEC-MOK-006`, `ADR-MOK-005`,
`CAP-MOK-009`, `INT-MOK-009` and `REQ-MOK-042` through `REQ-MOK-046`. `master` took `WO-MOK-012` for
unrelated work and reached the integration branch first, at `fa065cc`, so this chain renumbered to
**`WO-MOK-018`** at `fa0bfd9` on the owner's decision of 2026-08-21, *"018, nothing else moves"* —
`018` being the first number above `master`'s maximum of `WO-MOK-017`. That act moved one identifier
and two paths; the other ten this chain claims collided with nothing and did not move.

A few hours later `master` moved again. Pull request #37 — *"Close the two observer defects Phase 3.1
left"* — merged as `7f4792a` and created its own **`WO-MOK-018`**, together with `VREC-MOK-017` and
`VREC-MOK-018`. It is not a variant of this work order and shares nothing with it: *"Emit a
structured record stream to an operator-named sink"* against *"Close the two observer defects Phase
3.1 left"*, one an engine work order and one an observer work order.

`merge/README.md` finding 10 reported the collision as an open question. This act closes it by moving
this chain again, to **`WO-MOK-019`**, at commit `efe20e3`.

**The renumbering, applied here:**

| was | is | length |
|---|---|---|
| `WO-MOK-018` | `WO-MOK-019` | 10 → 10 |
| `work-orders/WO-MOK-018.md` | `work-orders/WO-MOK-019.md` | one path |
| `evidence/WO-MOK-018/` | `evidence/WO-MOK-019/` | one directory, with all 104 tracked files inside it |

**Nothing else in the chain moves.** `VER-MOK-012`, `VREC-MOK-012`, `SPEC-MOK-006`, `ADR-MOK-005`,
`CAP-MOK-009`, `INT-MOK-009` and `REQ-MOK-042` through `REQ-MOK-046` are unchanged, because none of
them collides with anything on any ref. `master` created no `VER-MOK-012` and no `VREC-MOK-012`, so
this chain fills those numbers rather than competing for them.

**Nothing about the work changed.** No requirement, specification provision, oracle, measurement,
judgement, decision or line of executable behaviour differs because of the renumbering. Only the
names do.

## Why this side moves, and on whose authority

The standing rule is decision 3 of the earlier collision's closing review, recorded in
`master`'s `evidence/WO-MOK-012/identifier-collision.md` and taken by the repository owner acting as
engineering owner:

> Neither side renumbers now. The conflict is resolved by whichever of the two branches merges to
> `master` second.

`master` reached the integration branch first both times. This branch is the second both times, so it
moves both times. The rule is mechanical and this act applies it; **no owner instruction was sought
for this second application and none is claimed.** The owner's *"018, nothing else moves"* answered
the first collision and chose the first-above-maximum rule; this act follows the same rule to the
next number, and it is disclosed rather than treated as authorized by that answer.

**The rule did not become cheaper by being applied twice.** `WO-MOK-013`'s stop-and-escalate
condition 8 states the same obligation from the other side — it does not fire during implementation
and fires at the merge, where *"reaching the merge without having renumbered is the escalation."*
This act is that escalation, discharged, for the second time in one day on one branch.

## The collision set was one identifier, and it was derived rather than assumed

A single-tree `validate` cannot see a cross-branch collision. At `f40fead` there is exactly one
`WO-MOK-018` and at `7f4792a` there is exactly one, and `validate` reports 0 errors and 0 warnings on
each. The clash exists only in the union, and whichever branch merges second inherits it.

The derivation: an identifier present on **both** tips but **absent at the merge base** `fa065cc` is
an independent creation on each side. Enumerated from each ref's tree rather than from a working
directory:

| | count |
|---|---|
| identifiers at the base `fa065cc` | 131 |
| identifiers on `master` `7f4792a` | 134 |
| identifiers on this branch `f40fead` | 143 |
| present on both tips | 132 |
| present on both tips and absent at the base | **1** |

    WO-MOK-018

`master`'s three arrivals since the base are `WO-MOK-018`, `VREC-MOK-017` and `VREC-MOK-018`. This
chain's twelve are `WO-MOK-018`, `ADR-MOK-005`, `CAP-MOK-009`, `INT-MOK-009`, `REQ-MOK-042` through
`REQ-MOK-046`, `SPEC-MOK-006`, `VER-MOK-012` and `VREC-MOK-012`. Exactly one name is in both lists.
**That is why this renumbering moves one identifier where the `013 → 014` precedent moved four**, and
it is a measurement rather than an optimistic reading of the diff.

The same figures state the cost in the graph: the pre-renumbering union is 143 + 134 − 132 = **145**
identifiers, and the merged tree measures **146** artifacts. The extra one is the collision unmade.
`merge/second/governance.txt` reports it from the other end.

**The identifier was re-swept across every ref before being taken**, by enumerating each ref's tree,
not by reading the local maximum:

| Reading | Result |
|---|---|
| refs enumerated | 9 — `refs/heads/master`, `refs/heads/wip/pr31-integration`, five `refs/remotes/origin/*`, `refs/remotes/origin/HEAD`, `refs/tags/v0.1.0` |
| `WO-MOK-*` present on some ref | `001`–`014`, `016`, `017`, `018` |
| maximum | `WO-MOK-018` |
| `WO-MOK-019` as a path on any ref | absent |
| `WO-MOK-019` in the content of any ref but this branch | absent — checked on all seven non-branch refs |
| `VREC-MOK-*` present on some ref | `001`–`018`, so `VREC-MOK-019` is free too, and is the number `merge/README.md` names for the record still owed |

**`WO-MOK-015` is unoccupied on every ref, and this act does not fill it.** `019` is the first number
above the maximum, not the lowest free number. That follows the rule the owner's *"018, nothing else
moves"* chose over the alternative, and whether the gap at `015` should ever be filled is not an
implementation agent's question. It is noted here so nobody later reads `019` as evidence that `015`
is taken.

## What was rewritten, and how

The rewrite was applied by **byte substitution**, reading and writing bytes and never lines. That is
not fastidiousness on this repository: the working tree is bimodally line-ended — documentation is
CRLF under `core.autocrlf`, and `.gitattributes` marks
`docs/engineering/simulation/evidence/** -text`, so every file in this directory is LF and verbatim
because the digests recorded against captured streams must reproduce. Both `sed` and Python's
`write_text` normalise line endings, and either would have rewritten every line of every file it
opened and buried 364 identifier changes in tens of thousands of ending changes.

**Every substitution preserves byte length**, by construction — both identifiers are ten bytes — and
the property was asserted mechanically per file rather than assumed. This packet records byte counts,
line numbers, column positions, file lengths and stream sizes throughout, and a rewrite that changed
any file's length would falsify them wholesale. The rewriter refused the entire run on any mismatch,
and additionally compared each file's `\r\n` count and its total `\r` count before and after.

**At `f40fead`, the commit before this one: 364 occurrences across 86 files.**

| kind | files | occurrences |
|---|---|---|
| `.md` — authored prose and artifacts | 19 | 172 |
| `.txt` — retained tool captures | 45 | 132 |
| `.py` — retained analysis scripts | 17 | 52 |
| `.sh` — retained capture scripts | 3 | 6 |
| `.rs` — engine source comments | 2 | 2 |
| **total** | **86** | **364** |

The commit touches **119 paths**: 105 renames — the work-order file and the 104 tracked files of the
evidence directory — of which **33 moved by path only**, their contents holding no occurrence; plus
**14 files modified in place**, which are `SIMULATION_RULES.md`, `docs/PHASE_4_PROPOSAL.md`,
`docs/ROADMAP.md`, `ARCH-MOK-001.md`, `ADR-MOK-005.md`, `INT-MOK-009.md`, `SPEC-MOK-001.md`,
`SPEC-MOK-002.md`, `SPEC-MOK-004.md`, `SPEC-MOK-006.md`, `VER-MOK-012.md`, `VREC-MOK-012.md`,
`mokiterions-core/src/simulation.rs` and `mokiterions-core/tests/cli.rs`.

The two paths were moved with `git mv` rather than by writing new files and deleting old ones, so
rename detection holds across all 105 and the diff reads as a rename rather than as 104 deletions and
104 creations. **The owner authorized that command specifically**, the permission having been refused
twice before it was asked for; nothing was modified in the interim.

**Line endings were verified in the index after the rename, not before it.** `git check-attr text eol`
on the moved files reports `text: unset`, so the `-text` attribute still applies at the new path, and
`git cat-file blob :<path>` on all 119 staged paths finds **0 carriage returns**. That check was run
because a first attempt at it was wrong — a shell quoting error made `grep -c` count every line
rather than every `\r`, and reported an alarm on some hundred and ten files. The staged diff was
352 insertions and 352 deletions, which is impossible if line endings had flipped, and that is what
exposed the false alarm rather than any second opinion about the pattern.

### The three non-documentation occurrences, disclosed individually

    SIMULATION_RULES.md:873          a table row listing evidence directories
    mokiterions-core/src/simulation.rs:4829   a /// doc comment naming the capture's densities
    mokiterions-core/tests/cli.rs:13          a //! module comment naming the option's origin

All three are comments or paths. **No executable behaviour, test expectation, exit code, declared
dependency, manifest, lockfile, workflow or public interface item differs.** The `SIMULATION_RULES.md`
row was rewritten because leaving it would point a reader at `evidence/WO-MOK-018/`, which after the
merge is `master`'s packet for different work — a stale path here does not merely dangle, it
misdirects.

### The retained captures were rewritten, and that departs from precedent

**132 of the 364 occurrences are inside 45 retained `.txt` captures, and this act rewrote all of
them.** The repository's most recent renumbering did the opposite. `evidence/WO-MOK-014/
WO-MOK-014-renumbering.md` reverted its fourteen retained captures to byte-identical blobs and gave
the reason:

> Editing them would make the packet assert that a tool printed something it did not print, which is
> a worse defect than a stale name: a capture that has been improved is no longer a capture.

That reasoning is sound and this packet is on the other side of it. Three things about how it got
there, none of them a defense:

1. **This packet's first renumbering already made the choice.** `fa0bfd9` swept 32 retained `.txt`
   captures from `WO-MOK-012` to `WO-MOK-018`. Its commit message records the occurrence count and
   the length-preservation argument and does not raise the captures question at all. No
   `renumbering.md` was written for that act; this file is the first record of it, written after the
   fact, which is itself late.
2. **Reverting now would restore a different falsehood, not the truth.** Of the 45 captures, **23
   predate `fa0bfd9`** and hold **60** occurrences: in those, the tool printed `WO-MOK-012`, and
   reverting them to `WO-MOK-018` would assert a string no tool ever emitted. The other **22
   postdate it** and hold **72** occurrences: in those the tool did print `WO-MOK-018`, and reverting
   is a coherent option. Doing it for 22 files and not for 23 would leave the packet reading three
   different identifiers for one work order across one directory.
3. **What is actually owed is the reading rule, stated once and plainly.** Every `.txt` in this
   directory now reads `WO-MOK-019`. Where the capture predates `fa0bfd9` the tool printed
   `WO-MOK-012`; where it postdates `fa0bfd9` and predates `efe20e3` the tool printed `WO-MOK-018`;
   nothing in this directory was captured by a tool that printed `WO-MOK-019` except the two files
   under `merge/second/`. **A reader who needs a capture's verbatim bytes must take them from
   `git show <commit>:<path>`, not from this working tree.** The commits are `fa0bfd9^` and
   `efe20e3^`.

**Put to the owner, as a question and not as a report:** whether the sweep-the-captures rule this
packet has now applied twice is the rule they want, or whether the `013 → 014` rule is, in which case
the 22 post-`fa0bfd9` captures should be reverted to `WO-MOK-018` and this file rewritten to say so.
It is one instruction either way and the implementation agent should not choose it.

**Three prose falsifications the blind sweep caused were repaired by hand rather than left**, because
in each the surrounding text claims verbatimness about a specific checkable string:
`merge/README.md`'s quotation of the first merge's commit message, `merge/README.md`'s narrative of
what the first renumbering renamed, and three lines of captured `preflight` output in
`merge/gates.txt`. Each repair is disclosed at the site.

### The retained scripts were rewritten deliberately

The 17 `.py` and 3 `.sh` files in `analysis/`, `merge/` and this directory hold 58 occurrences, all of
them output paths and header strings. They were rewritten on the `007 → 010` precedent and for its
stated reason: each script is retained so a figure can be reproduced from the recorded command rather
than trusted, and a script whose output paths point at a directory that no longer exists reproduces
nothing.

The consequence is visible and is predicted here rather than found later: re-running any of them
writes to `evidence/WO-MOK-019/` while the captures beside them were written by the same scripts
pointing at `evidence/WO-MOK-018/` or `evidence/WO-MOK-012/`, so a byte comparison against a retained
capture must be made on content and not on the header line, which will differ.

## The digests, and why the rename invalidates none of them

Checked rather than assumed, because an earlier renumbering in this repository did invalidate one:

- **No digest recorded anywhere in this packet covers a file under `docs/`.** Searched for across
  every `.txt` in the directory: every 64-hex digest in the packet is of a record stream, a text
  stream, a replay or a manifest of those, and none names a repository path. The 120 capture cells,
  the entropy series, the oracle manifests and `oracle2`'s reconstruction all measure streams written
  to sinks outside the checkout.
- **`sizes.txt`'s byte figures are stream sizes**, measured by `measure-sizes.sh` on files it creates
  and deletes per row. The rename touches none of them. The figures inside this packet that *are*
  measurements of repository files — byte counts in file tables, line numbers, column positions — are
  the ones length preservation protects, and it is why the rewriter asserts it per file.
- **No manifest, lockfile or workflow contains the identifier**, so none was touched and every digest
  recorded over them still reproduces.
- **The harness snapshot digests recorded in `gates.txt` and `merge/gates.txt` were never
  reproducible outside the clone and commit that produced them**, because `build_snapshot` embeds
  `git rev-parse HEAD` and the checkout's leaf directory name. The rename adds a second reason — the
  hashed document contains the evidence path index, 55 of whose paths moved — but takes nothing away
  that was there. `merge/second/gates.txt` states the same caveat for its own figure.

## What the rename does to `VREC-MOK-012`, which is `verified`

`commit` still reads `50364a3719c68643f0b5354798b6d3084cff1c0e`; `verified_at` still reads
`2026-08-20T12:08:52Z`; `status` is still `verified`. **A rename is not a re-capture**, and the record
still claims exactly the tree that commit holds — a tree in which this work order is called
`WO-MOK-012`.

- **`artifact_snapshot_sha256 = 16862ef3…` is unchanged and is deliberately not re-stated.** It is
  reproducible only at `50364a3`. The hashed document contains the evidence path index, so a value
  re-taken against the renamed tree would be a different measurement wearing the same field name, and
  it would no longer be the digest the assurance decision was taken over. `VREC-MOK-011`'s
  renumbering reported one digest as unpreservable rather than re-stating it, and this is the same
  choice.
- **`evidence_paths` were rewritten** — all 55 of them, to the `WO-MOK-019/` names the files now
  carry. They must resolve for the validator to read the record at all, and `VREC-MOK-011` was
  renumbered the same way, 222 paths at once. The bytes of every file they name are unchanged except
  for the identifier itself.
- **`title` now reads *"Verification candidate for WO-MOK-019"*.** This is a departure from the
  `013 → 014` precedent, which kept the title as the capture wrote it, and it was made by the blind
  sweep rather than by a decision. It is left as swept and flagged: the title has now moved twice,
  from `WO-MOK-012` to `WO-MOK-018` to `WO-MOK-019`, and the field's original wording is recoverable
  from `git show 50364a3:docs/engineering/simulation/verification-records/VREC-MOK-012.md`. Whether
  to restore it is the owner's to say, on the same instruction as the captures question above.
- **This file is not in `evidence_paths`**, on the precedent `WO-MOK-014-renumbering.md` set and on
  this packet's own: of the 110 tracked files this directory will hold, 55 are declared and 55 are
  not — the whole of `merge/` and `assurance-decision.md` among them — because they postdate the
  candidate commit `50364a3`. Adding this one would also
  move `artifact_snapshot_sha256`, since the path index is inside the hashed document.
- **It is named `renumbering.md` and not `WO-MOK-019-renumbering.md`**, matching the `007 → 010`
  precedent and this packet's own unprefixed naming, in which no file carries the identifier as a
  prefix. The consequence is that `W-HEX-001` continues to observe `WO-MOK-019` — an implemented work
  order with no evidence document keyed to its ID — and `merge/second/governance.txt` measures it at
  seven observations including this one. Naming this file with the prefix would have closed that
  observation and moved the dashboard snapshot, which is a change to a measurement and not a
  disclosure.

**The exposure, stated plainly.** `VREC-MOK-012` was verified on the owner's decision, and
`assurance-decision.md` records it. This rename changes the name of the artifact that decision was
taken about, and the path of every file recording it — for the second time, the first having happened
at `fa0bfd9` under the same record. Nothing here re-opens the record's lifecycle: it is still
`verified`, it still binds `50364a3`, and no field of it was re-measured. What it cannot do is reach
the merge commit, which is why a **new** record is owed there rather than a re-pointing of this one.

## Measured after the rename

A renumbering that moved any of these would not be a renumbering. Taken at `efe20e3` with
se-harness 0.4.0 from `C:\Users\mathi\harness-venv-040`:

| Reading | Result |
|---|---|
| `validate .` | PASS — 143 artifacts, 0 errors, 0 warnings across all four planes |
| `inspect .` | 32 findings — error 0, warning 15, info 17 |
| `preflight . --work-order WO-MOK-019 --phase review` | PASS — `Work order: WO-MOK-019 (implemented)` |
| `doctor .` | 81 lines, every one PASS |
| `cargo test --workspace` | 298 passed, 0 failed — the figure `merge/test-run.txt` retains at `e8114ad`, unchanged because the two `.rs` occurrences are comments |
| conflicted paths in `merge-tree HEAD origin/master` | 1 — `SPEC-MOK-004.md`, down from 5 |

**`preflight`'s `Work order:` line is the check that the sweep left no dangling reference.** The
command resolves the work order from the identifier it is given, and a rename that had missed one
occurrence in 143 artifacts would have failed to resolve it or resolved it to nothing.

`merge/second/governance.txt` carries the same readings on the merged tree, where the figures are
146 artifacts, 35 findings and 301 tests, and reconciles them across all four trees.

## What this act does not discharge

The renumbering resolves an identity collision. It is not the merge, it does not make this packet
current, and it decides nothing:

- **`VREC-MOK-019` at the merge commit is owed and is not created here.** `VREC-MOK-012` binds
  `50364a3`, which is an ancestor of neither `origin/master` nor the released `v0.1.0`. Renaming it
  does not extend its reach by one commit, and a record cannot be re-pointed at a later commit — a
  successor is written and the predecessor superseded.
- **Two ratifications are outstanding**, both drafted under the procedure the owner set on 2026-08-21:
  the 2026-08-21 `SPEC-MOK-006` row, and the new 2026-08-21 `SPEC-MOK-004` row for the second merge.
  `merge/second/README.md` is that row's account.
- **The captures question and the `title` question above are owed one owner instruction between
  them**, and this file does not act on either.
- **Everything `merge/README.md` §*What is still owed* lists remains owed**, less finding 10, which
  this file closes.
- **PR #31's body trailer must read `Harness-Work-Order: WO-MOK-019`.** CI reads the trailer from the
  stored event payload, so a body edit takes effect on the next push and not before; and a stale
  trailer does not fail the check, it passes it against the wrong work order.

Every command behind every figure in this file is offline, reads no credential, secret, token or
environment value, and none appears in this file or in any retained evidence.
