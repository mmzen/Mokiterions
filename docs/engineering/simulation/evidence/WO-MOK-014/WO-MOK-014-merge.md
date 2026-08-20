# `master` merged into this branch

This file records the merge of `master` into `governance/adr-mok-006-third-party-crates`, the resolution of the four
conflicts it raised, the four sentences it made false and the correction of each, and the whole gate set re-derived on
the merged tree. It is the second of the two acts the repository owner asked for on 2026-08-20:

> we need to resolve the merge conflict, renumber the collisioned artifacts / work orders

The first act, the renumbering of this chain from `013` to `014`, is recorded in `WO-MOK-014-renumbering.md` and is
assumed as read here. The owner also chose the integration method — **merge `master` in**, rather than rebase — and
placed both acts **inside this work order** rather than opening another. `WO-MOK-014` in-scope item 5 is the
`SPEC-MOK-004` figure corrections the change forces, and this file reports that no figure correction is owed.

**This file measures. It decides nothing.** No requirement, provision, oracle, decision or line of executable behaviour
is changed by the merge or by anything recorded here. The four corrections it records are corrections to statements of
fact about *other artifacts' status*, taken under a rule `master` itself wrote, and each is reproduced below in full.

## The three commits

| role | commit | what it is |
|---|---|---|
| merge base | `ff3a155` | the commit this branch started from, and the last common ancestor |
| ours, `HEAD` | `4a32a95` | this branch's tip after the renumbering: `ADR-MOK-006`, `WO-MOK-014`, `VER-MOK-014`, `REQ-MOK-050`, `VREC-MOK-014` and this evidence directory |
| theirs, `MERGE_HEAD` | `6b02573` | `origin/master`, carrying `WO-MOK-012` and `WO-MOK-013` and their chains |

Neither tip is an ancestor of the other. `git merge --no-commit --no-ff origin/master` was used so that the resolution
could be measured before it was recorded.

## What `master` brings

At the point of commit, `git status --porcelain` reports **56 `A`**, **ten files modified**, one of them `MM`, and
**four `UU`**. The additions are entirely `master`'s two chains — `WO-MOK-012` and `WO-MOK-013` with their verification,
requirements (`REQ-MOK-047`, `REQ-MOK-048`, `REQ-MOK-049`), records (`VREC-MOK-013`) and evidence directories. The
modifications divide into three groups:

| group | files | who changed them |
|---|---|---|
| auto-merged from `master` | `VER-MOK-005.md`, `mokiterions-tui/src/{layout,render,verification}.rs`, `mokiterions-tui/tests/{layout,render,verification}.rs` | `master` alone; no overlap with this branch |
| conflicted, resolved here | `ARCH-MOK-001.md`, `SPEC-MOK-002.md`, `SPEC-MOK-003.md`, `SPEC-MOK-004.md` | both sides, in the same amendment tables |
| written by this act | `docs/ROADMAP.md` (`MM`: `master`'s text plus a later-fact note), `ADR-MOK-006.md`, `WO-MOK-014.md` | this branch, in the merge commit |

This branch's own work touches no file `master` touched outside those four tables: the two manifests, the lockfile, the
workflow and everything under `scripts/` are `master`'s or untouched, and no `.rs` file is changed by this side at all.
That is why the engine replay hashes and the declared dependency sets are unaffected by the merge, and why the test
census is not.

## Why four conflicts and not nine

Before the renumbering there were **nine**, four of them `add/add` on `WO-MOK-013.md`, `VER-MOK-013.md`,
`VREC-MOK-013.md` and `REQ-MOK-047.md` — two unrelated artifacts arriving at the same path. Renaming this chain to
`014`/`050` removed all four outright, along with the `evidence/WO-MOK-013/` directory collision.
`WO-MOK-014-renumbering.md` records that measurement. What remains is four content conflicts, all of one kind.

## What the four conflicts actually were

**Not row additions.** Every one is in an amendment record table, and in each the two sides changed *the same rows*.
`master`'s `WO-MOK-012` was the owner's assessment review: it ratified the amendment rows that had been standing
`**OUTSTANDING.**` since 2026-08-18 and 2026-08-19, rewriting their disposition cells in place. This branch, working
from `ff3a155`, appended new 2026-08-20 rows below them and — in four of those rows — wrote a sentence saying the older
rows *stay* `OUTSTANDING`. Both sides therefore edited the same table, adjacently.

Measured across the four files, keyed on each row's description cell, which is unique within each table:

| artifact | rows at `4a32a95` | rows at `6b02573` | shared | added by this branch | shared rows whose disposition differed | rows merged |
|---|---|---|---|---|---|---|
| `ARCH-MOK-001` | 6 | 5 | 5 | 1 | 1 | **6** |
| `SPEC-MOK-002` | 5 | 4 | 4 | 1 | 3 | **5** |
| `SPEC-MOK-003` | 13 | 18 | 11 | 2 | 4 | **20** |
| `SPEC-MOK-004` | 6 | 6 | 5 | 1 | 1 | **7** |

The nine differing dispositions are two distinct edits by `master`, and both are the same act seen from two sides.

**Four rows moved from `**OUTSTANDING.**` to `**Ratified 2026-08-20 …**`.** At `ff3a155` exactly four rows in the whole
tree carried the bare `**OUTSTANDING.**` disposition, and they are these four:

| artifact and row | subject |
|---|---|
| `ARCH-MOK-001:47`, 2026-08-18 | the prohibition on public items narrowed from "mutable **or owned** authoritative state" |
| `SPEC-MOK-002:21`, 2026-08-18 | four provisions amended so the terminal observer can be conformed |
| `SPEC-MOK-003:53`, 2026-08-18 | *Data and interface contracts* corrected on a claim about the engine's surface |
| `SPEC-MOK-004:21`, 2026-08-19 | recorded figures and two subject lines corrected under `SPEC-MOK-003` rule 5 |

In the merged tree, **no row anywhere carries that disposition**: zero, against four at the base. Each now names the
ratifying act, the role, the date and the review that recorded it.

**Five later rows had their sentences about those four moved to the past tense.** `WO-MOK-012.md:213-223` states the
rule it applied:

> Five later rows asserted in the present tense that these provisions remain OUTSTANDING. Those sentences became false
> the moment the ratifications were recorded, so each is moved to the past tense and names the ratifying act… No
> approved row's substance is altered; only a statement about another row's status is brought up to date.

The five are `SPEC-MOK-002:22` and `:23`, and `SPEC-MOK-003:52`, `:54` and `:56` — measured, and exactly the five that
rule names. One of them, `SPEC-MOK-002:23`, does more than re-tense: it records that the sentence had **overstated the
count**, because only the first of the two 2026-08-18 rows ever was outstanding — the second, the path re-basing, was
approved the same day by way of `ADR-MOK-004`. `WO-MOK-012` kept the miscount rather than deleting it, on the ground
that it is "a defect in a governance record's cross-reference, not in any specification's substance".

## How each conflict was resolved

One rule, applied to all four files: **keep every one of `master`'s rows exactly as `master` wrote them, including all
nine disposition rewrites, and re-add only this branch's rows that `master` does not have.** Nothing is merged inside a
cell, nothing is reworded during resolution, and no row is invented.

Two mechanical checks were run on the resolved worktree rather than assumed.

**Row-set equality**, comparing the resolved table against both parents row by row:

| artifact | `master` rows missing | this branch's rows dropped | rows present in neither parent |
|---|---|---|---|
| `ARCH-MOK-001` | 0 | 0 | 0 |
| `SPEC-MOK-002` | 0 | 0 | 0 |
| `SPEC-MOK-003` | 0 | 0 | 0 |
| `SPEC-MOK-004` | 0 | 0 | 0 |

and for every one of the twenty-five shared rows the resolved disposition is byte-identical to `master`'s — the nine
that differ took `master`'s side, and the other sixteen were identical on both sides already.

**Length arithmetic.** For each conflicted file the resolved length equals this branch's length plus what `master` added
over the base. Newline counts, as `wc -l` reports them:

| artifact | base `ff3a155` | ours `4a32a95` | theirs `6b02573` | ours + (theirs − base) | resolved |
|---|---|---|---|---|---|
| `ARCH-MOK-001.md` | 140 | 148 | 140 | 148 | **148** |
| `SPEC-MOK-002.md` | 423 | 502 | 423 | 502 | **502** |
| `SPEC-MOK-003.md` | 825 | 994 | 971 | 1140 | **1140** |
| `SPEC-MOK-004.md` | 678 | 712 | 759 | 793 | **793** |

`ARCH-MOK-001` and `SPEC-MOK-002` are unchanged in length by `master` because its edits there are in-cell rewrites on
existing lines. No conflict marker remains: `^<<<<<<< `, `^>>>>>>> ` and `^||||||| ` all match nothing in the tree.

## The four sentences the merge made false, and their correction

Four rows this branch added carry a sentence asserting that an older row *stays* `OUTSTANDING`. Every one of those
sentences was true when written and false the moment `master` arrived. They are the same class of sentence
`WO-MOK-012` found five of and corrected; the only reason it did not correct these four is that they were on an
unmerged branch it could not see. They are corrected here under the same rule, in place, and each correction names the
ratifying act.

| site | what it said | what it says now |
|---|---|---|
| `ARCH-MOK-001:48` | "**The 2026-08-18 row above stays OUTSTANDING and was not touched**: it is `WO-MOK-005`'s precondition, and this approval neither clears nor inherits it." | that the row **was** outstanding when this row was written and was not touched; that it was `WO-MOK-005`'s precondition and this approval neither cleared nor inherited it; that the technical owner ratified it as written on 2026-08-20 under `WO-MOK-012`, which reached this branch by merge afterwards; and that the sentence is moved to the past tense under that work order's own rule, no obligation changing either way |
| `SPEC-MOK-002:24` | "**Both 2026-08-18 rows above stay OUTSTANDING and were not touched**: they are `WO-MOK-005`'s preconditions, and this approval neither clears nor inherits them." | opens "**Neither 2026-08-18 row above was touched.**", then records that the sentence had repeated the same miscount `WO-MOK-012` corrected at `SPEC-MOK-002:23` — only the first was ever outstanding, the second, the path re-basing, having been approved 2026-08-18 by way of `ADR-MOK-004` — and that the first was ratified as written on 2026-08-20 under `WO-MOK-012`. The miscount is recorded rather than deleted, following that work order's precedent |
| `SPEC-MOK-003:68` | "**The 2026-08-18 row above stays OUTSTANDING and was not touched.** `VREC-MOK-005`, which binds this specification, is not edited." | names the row — the 2026-08-18 *Data and interface contracts* row — records that it was outstanding when this row was written and was not touched, and that it was ratified as written on 2026-08-20 under `WO-MOK-012`. The `VREC-MOK-005` sentence is kept unchanged and remains true |
| `SPEC-MOK-003:69` | "**The 2026-08-18 row above stays OUTSTANDING and was not touched.**" | the same, in the shorter form that row carries |

**Two properties of the correction, asserted mechanically rather than assumed.** Each is a single-line byte
substitution: the replacement contains no newline, the line count is compared before and after and the edit would have
refused on any change, and each file's own ending style is preserved by reading and writing bytes. So **no line number
moves** — this packet cites `file:line` throughout, and a correction that shifted a cited coordinate would falsify the
citations wholesale. And each substitution asserts it matched **exactly once**: `SPEC-MOK-003:69`'s old text is a prefix
of `:68`'s, so `:68` was replaced first, and the counts prove neither hit the other's line.

**The authority.** `WO-MOK-012.md:213-223` is `master`'s own rule for exactly this sentence, quoted above, and it
arrives in this tree by this merge. `SPEC-MOK-003`'s standing provision that an implementation agent may correct a
statement of fact about another artifact — not a provision, obligation or decision — is what lets the agent apply it
without a new owner act. Both are narrow: **nothing is cleared and nothing is inherited here.** The ratification is
`WO-MOK-012`'s act; this work order neither performed it nor benefits from it, and the four amendments its own rows
record are approved on their own terms by `ADR-MOK-006`.

## What is added as a note rather than a change

Three places state something the merge overtakes but that must not be rewritten, because it is either an approved
instruction or another branch's record. Each gets a dated later-fact note instead.

| file | placement | what it records |
|---|---|---|
| `ADR-MOK-006.md` | three `**Later fact, 2026-08-20.**` notes under the *Required amendments* bullets for `ARCH-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-003` | those bullets prescribe the clause text that is now corrected. **The instruction is kept exactly as the owner approved it**; each note records the ratification under `WO-MOK-012`, that the correction is recorded here, and — for `SPEC-MOK-002` — that only the first of the two rows ever was outstanding. `ADR-MOK-006` grew 679 → 692 lines; the cited coordinates `:77` and `:287` are unmoved |
| `docs/ROADMAP.md` | a fifteen-line block quote after `master`'s paragraph ending "nothing was asked of either branch" | that this branch **renumbered**, so `master`'s two paragraphs naming `WO-MOK-013`, `VER-MOK-013`, `VREC-MOK-013` and `REQ-MOK-047` now resolve to `master`'s chain alone; why this side moved rather than `master`'s; that nothing is asked of `master`'s chain and `evidence/WO-MOK-013/identifier-collision.md` stays as written; that the conflict count measured **nine before and four after** in this clone while the **eight** recorded there was measured at an earlier tip; and that `master` has since been merged in |
| `WO-MOK-014.md` | an appended *Later facts, recorded when `master` was merged in on 2026-08-20* section | that every amendment row this work order lists as standing `OUTSTANDING` was ratified on 2026-08-20 under `WO-MOK-012`, that this work order still neither cleared nor inherited any of them, that four sentences in its own rows are corrected in place, that `VER-MOK-011`'s manual assessment 5 is untouched and still owed, and that required-report item 12's *standing OUTSTANDING rows* now names an empty set |

`docs/ROADMAP.md` records nothing else about this chain: **there is no `ADR-MOK-006`, `WO-MOK-014` or `REQ-MOK-050`
entry in it at all.** The note added here does not supply one — a roadmap entry for this work is owed, and it is not
written by an implementation agent inside a merge.

## What is deliberately left standing false

`evidence/WO-MOK-012/amendment-ratifications.md:180-184` states the counter-rule, and it governs everything in this
section:

> `VREC-MOK-005` … is not edited… it was verified with these eleven provisions disclosed as outstanding — its own words
> are that the eleven "stay OUTSTANDING"… That was correct at its commit.

A record bound to a commit is correct at that commit and is not brought up to date. Nine statements in this packet are
now false and are left exactly as written, named here so a reader finds them from the evidence rather than by surprise:

| site | the statement |
|---|---|
| `WO-MOK-014.md:274-276`, `:457-460` | the two the appended later-facts section answers instead of editing |
| `WO-MOK-014-amendments.md:194-197` | the rows stand `OUTSTANDING` |
| `assurance-decision.md:109-111`, item 6 | the same, as an input to the owner's assurance decision |
| `WO-MOK-014-completion-summary.md:365-368` | the same, **and** "Every pre-2026-08-20 amendment row in the six amended artifacts is byte-identical to `ff3a155`" — which no longer holds: `master` rewrote nine such rows' disposition cells, and that is what this merge accepts |
| `WO-MOK-014-transition.md:185-187` | the same |
| `VREC-MOK-014.md:313`, `:565-567` | the same, inside a `verified` record |
| `WO-MOK-014-capture.sh:263` | the same, in the capture script's own commentary |

`VREC-MOK-014` is the reason the line is drawn here rather than one file further on. It is `verified`, it binds
`65ac88b`, and its `artifact_snapshot_sha256 = a12ec1a3…` reproduces **only** at that commit — where, as
`WO-MOK-014-renumbering.md` records, the artifacts still carry their `013` names. Editing it would break the hash it
declares. A verification record bound to the merge commit is a different record, is owed, and is not created here.

For the same reason **this file is not added to `VREC-MOK-014`'s `evidence_paths`.** That list holds twenty-one paths;
the directory now holds twenty-three files, `assurance-decision.md` and `WO-MOK-014-renumbering.md` being already
unlisted on the same ground. The record is closed; the directory is the packet.

`master`'s own evidence is likewise untouched: `evidence/WO-MOK-013/identifier-collision.md` stays as written, and no
artifact of `master`'s chain is renamed, re-dated, demoted or re-opened by this merge.

## Every gate re-derived on the merged tree

Measured on the resolved worktree with `master` merged and all corrections applied, from the pinned `se-harness` 0.4.0
interpreter. Every command is offline.

| gate | this branch at `4a32a95` | merged | reading |
|---|---|---|---|
| `cargo fmt --all --check` | 0 | **0** | clean |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | 0 | **0** | clean |
| `cargo test --workspace --locked --offline` | 212 over 21 targets | **226 over 21 targets** | +14, all `master`'s; no target added or lost |
| Python test suite | 126 | **126** | unchanged; neither side's conflict touches a `scripts/` file |
| `check_declared_dependencies.py` | PASS | **PASS** | "Every declared set matches its resolved graph. 8.4a-8.4d pass" |
| `harnessctl validate` | PASS | **PASS**, 114 artifacts, E0, W0 | |
| `harnessctl dashboard` | 107 artifacts, 357 relations | **114 artifacts, 396 relations** | snapshot `da19c004092b4020…` without this file, `348251422870edae…` with it |
| `harnessctl inspect` | 20 findings, 7 warning, 13 info | **24 findings, 10 warning, 14 info** | `error` 0, `decision_required` 0 |
| `harnessctl doctor` | all PASS | **all PASS** | |
| `harnessctl preflight --work-order WO-MOK-014` | resolves | **resolves**, `REQ-MOK-050` | both `--phase start` and `--phase review` |

**The dashboard snapshot covers retained evidence**, so a hash recorded inside the packet it hashes cannot be stable:
writing the figure changes the figure. Both measurements are therefore given, taken by moving this file aside and back —
`da19c004092b4020264f583b4a992838d16d571e5214127b657e862d6aa2168f` with everything else in the merge commit present and
this file absent, and `348251422870edae1b5b178c756371067b9fcded72fc5021e0edd12536dffe19` with the file as it stood
before this paragraph was added. Neither is a declared value: no artifact declares a dashboard snapshot, and the
verification record that will bind the merge commit must take its own.

The test total is `cargo test`'s own per-target output, not a sum taken on trust:

| package | internal tier | public tier | total |
|---|---|---|---|
| `mokiterions` (engine) | `src/lib.rs` 54, `src/main.rs` 0 = **54** | `cli` 13, `decisions` 1, `density` 2, `naming` 3, `process` 6, `termination` 4, `viability` 2 = **31** | **85** |
| `mokiterions_tui` (observer) | `src/lib.rs` 33, `src/main.rs` 8 = **41** | `authority` 4, `export` 7, `layout` 11, `options` 8, `render` 22, `spatial` 7, `state` 21, `verification` 20 = **100** | **141** |
| workspace | | | **226** |

## `SPEC-MOK-004`'s recorded figures against the merged tree: nothing is owed

`WO-MOK-014` in-scope item 5 covers the figure corrections the change forces. **None is forced**, and that is a
measurement rather than an assumption:

- **Rule 9**, public tier **100**, reproduced file by file as 4 / 7 / 11 / 8 / 22 / 7 / 21 / 20 above.
- **Rule 10**, internal tier **41**, measured as the observer's lib target 33 plus its bin target 8.
- **Rule 11**, observer **141**, engine **85**, workspace **226** — exactly the figures `master`'s 2026-08-20 row
  records for its own `WO-MOK-013`, and that row's cross-check that 141 is rule 10's 41 plus rule 9's 100 holds.
- **Rule 6**, unchanged at **94 items, 24 public fields, 118 `pub` lines**. `master`'s own
  `evidence/WO-MOK-013/analysis/interface.py`, run on the merged tree, reports recorded equal to measured on all three,
  and the engine's 92 items are unchanged item for item against `ff3a155`.

This branch's own 2026-08-20 row at `SPEC-MOK-004:26` says "every test-count figure rules 9 to 11 record is untouched
by a prose-only change" and that no figure this specification counts moves. Both remain true after the merge: the
figures moved because `master` added fourteen tests, not because of this amendment, and `master`'s row records that
move. `cargo tree -p Mokiterions --locked --offline` still prints one crate and the observer still resolves the
fifty-seven-crate `ratatui` graph plus both packages, so neither declared set nor the empty-table conclusions move
either.

## The derived findings that moved

Twenty-four findings, `error` **0** and `decision_required` **0**, from twenty before the merge.

**`W-HEX-001`, 2 → 4** — implemented work orders with no evidence document keyed to their ID: `WO-MOK-010`,
`WO-MOK-011`, and now `master`'s `WO-MOK-012` and `WO-MOK-013`. **`WO-MOK-014` is not in the list.** That is the check
that the renumbering preserved evidence discovery: the inspector finds this work order's packet under its new name,
which it could not do if the rename had left the directory or the file names behind.

**`I-REV-001`, 13 → 14** — `VREC-MOK-013` arrives with `master`; `VREC-MOK-014` was already there. Every verification
record and `RLS-MOK-001` reports an observed checkout differing from its declared candidate commit, which is what a
later governance commit looks like.

**`W-HEX-003`, 5 → 6**, and this is the one that matters:

| observation | before | after |
|---|---|---|
| `ADR-MOK-002` predates `ARCH-MOK-001` | present | present |
| `ADR-MOK-004` predates `ARCH-MOK-002` | present | present |
| `VER-MOK-008` predates `REQ-MOK-036` | present | present |
| `WO-MOK-008` predates `SPEC-MOK-003` | present | present |
| `VER-MOK-005` predates `REQ-MOK-026` | **present** | **cleared** — `master` amended `VER-MOK-005` |
| `REL-MOK-001` predates `VER-MOK-005` | absent | **new** |
| `WO-MOK-008` predates `VER-MOK-005` | absent | **new** |

`VER-MOK-005` is the artifact the owner deferred: the four assertion sites `assurance-decision.md` item 1 names are to
be reworded in a separate work order after #33 merges. **The warning that prompted the deferral is gone and the
deferred substance is not.** `master`'s amendment moved `VER-MOK-005` forward of `REQ-MOK-026`, which clears that pair
and pushes the same staleness onto two older artifacts that point at it; the four assertion sites themselves are
byte-identical after the amendment. The deferral stands on its own terms, and its coordinates are re-derived below.

## Recorded coordinates the merge moved

The packet cites `file:line` throughout. `master` lengthened three cited files, so their coordinates are re-derived
here; the future `VER-MOK-005` and `VER-MOK-008` work order must use the right-hand column.

| citation | at `4a32a95` | merged |
|---|---|---|
| `VER-MOK-005` | `:149` | **`:152`** |
| `VER-MOK-005` | `:231` | **`:235`** |
| `VER-MOK-005` | `:234` | **`:238`** |
| `VER-MOK-005` | `:296` | **`:313`** |
| `VER-MOK-005` | `:318` | **`:335`** |
| `SPEC-MOK-003` | `:58`, `:59` | **`:68`, `:69`** |
| `SPEC-MOK-003` | `:794`, `:800` | **`:916`, `:922`** |
| `SPEC-MOK-004` | `:25` | **`:26`** |

Unmoved, and verified so rather than assumed: `VER-MOK-008:169`, `ARCH-MOK-001:48` and `:139`, `SPEC-MOK-002:24`,
`ADR-MOK-006:77` and `:287`, and `WO-MOK-014:87`, `:118`, `:303` and `:329`. The packet cites no `.rs` line coordinate,
so `master`'s six source and test files move nothing.

## Both chains after the merge

No identifier is duplicated and nothing is demoted. `WO-MOK-013` and `WO-MOK-014` are both `implemented`;
`VER-MOK-013` and `VER-MOK-014` both `approved`; `VREC-MOK-013` and `VREC-MOK-014` both `verified`; `REQ-MOK-047` and
`REQ-MOK-050` both `approved`. `harnessctl validate` reads 114 artifacts with no error and no warning, which is the
check that the two chains coexist rather than collide.

## What this act does not discharge

- **A verification record bound to the merge commit is owed.** `VREC-MOK-014` binds `65ac88b`, before the merge and
  before the rename landed, and its snapshot hash reproduces only there. Nothing in this file substitutes for that
  record, and no verification decision is taken here.
- **Nine statements in retained evidence and in `VREC-MOK-014` remain false**, listed above and left as written by
  design, including `WO-MOK-014-completion-summary.md:367-368`'s byte-identity claim against `ff3a155`.
- **`docs/ROADMAP.md` still records no entry for this chain.** The note added here corrects `master`'s two paragraphs
  about the collision; it does not create the missing `ADR-MOK-006`, `WO-MOK-014` and `REQ-MOK-050` entry.
- **`VER-MOK-005` and `VER-MOK-008` stay owed** at the four assertion sites, in a separate work order after #33 merges,
  at the re-derived coordinates above.
- **`VER-MOK-011`'s manual assessment 5 is untouched and still owed**, and `VER-MOK-014`'s assessment 7 is still only
  found not due — the first admission of a crate beyond `ratatui` owes it by name.
- **`REL-MOK-001` is not amended** and no release record is created. `RLS-MOK-001` released 0.1.0 from a commit that
  does not include this work.
- **Pull request #33 is not merged, its body is not edited, and nothing is pushed by this act.** Four statements in the
  body are overtaken by the renumbering and by this merge, and two commits are missing from its list; correcting it is
  the owner's to authorise.

Every command behind every figure in this file is offline, reads no credential, secret, token or environment value, and
none appears in this file or in any retained evidence.
