# The renumbering of this chain from `013` to `014`

This file records a governance act taken on the packet itself rather than a measurement. It is retained for three
reasons: fifty occurrences in fourteen retained captures in this directory name this work order by an identifier that
now belongs to different work on `master`; the record this chain closes with was **already `verified`** when the rename
was applied, so the subject of a recorded assurance decision moved after the decision was taken; and a reader comparing
this packet against `master` will otherwise find two directories, two work orders and two verification records that look
like variants of each other and are not.

## What happened

This work order was drafted, approved, implemented and verified as `WO-MOK-013`, with `VER-MOK-013`, `REQ-MOK-047`,
`VREC-MOK-013` and an evidence directory at `docs/engineering/simulation/evidence/WO-MOK-013/`. `ADR-MOK-006`, the
fifth identifier in the chain, never collided and is unchanged.

While it sat unmerged on `governance/adr-mok-006-third-party-crates`, an unrelated chain took all four names and reached
`master` first, through pull request #32. `master`'s `WO-MOK-013` is *"Make the observer's survival gauges resolve, its
controls discoverable, and its hidden-pane notice actionable"* — a presentation work order with nothing to do with
dependency declarations — and it carries its own `VER-MOK-013`, `REQ-MOK-047`, `VREC-MOK-013` and
`evidence/WO-MOK-013/`. It also created `REQ-MOK-048` and `REQ-MOK-049`, which is why this chain's requirement moves to
`050` and not to `048`.

Both records were captured the same day, nine minutes apart, from two clones, against commits neither of which is an
ancestor of the other: this one at `2026-08-20T15:29:34Z` binding `65ac88b`, master's at `15:38:30Z` binding `41c20cad`.

**The renumbering, applied here:**

| was | is | length |
|---|---|---|
| `WO-MOK-013` | `WO-MOK-014` | 10 → 10 |
| `VER-MOK-013` | `VER-MOK-014` | 11 → 11 |
| `VREC-MOK-013` | `VREC-MOK-014` | 12 → 12 |
| `REQ-MOK-047` | `REQ-MOK-050` | 11 → 11 |
| `evidence/WO-MOK-013/` | `evidence/WO-MOK-014/` | with all twenty-one `WO-MOK-013-*` files inside it |

**Every substitution preserves byte length.** That is not decoration: this packet records line numbers, byte counts,
column positions and file lengths throughout, and a rename that changed any file's length would falsify them wholesale.
The property was asserted mechanically, file by file, rather than assumed — the rewriter compared lengths before and
after on all thirty-nine files it touched and would have refused on any mismatch.

**Nothing about the work changed.** No requirement, specification provision, oracle, measurement, judgement, decision or
line of executable behaviour differs because of the renumbering. Only the names do.

## Who decided it, and why this side moved

The instruction was the repository owner's, on 2026-08-20:

> we need to resolve the merge conflict, renumber the collisioned artifacts / work orders

Two answers of theirs from earlier the same day frame it and are not re-derived here: that whatever is done belongs
**inside the existing work order**, so this is not a new work order; and that integration is by **merging `master` in**,
which is a separate act following this one. They had also answered *hold — decide nothing yet* on the disposition before
`VREC-MOK-013` was verified, so the hold was lifted by this instruction and not by inference.

The standing rule is decision 3 of the colliding chain's closing review, taken by the same owner as engineering owner:
*"Neither side renumbers now. The conflict is resolved by whichever of the two branches merges to `master` second."*
This branch is the one that has not landed, so it moves.

**One asymmetry hardened between the measurement and the act, and it is the reason there was no cheaper option.** When
the collision was first priced, `master`'s `VREC-MOK-013` was at `ready` — a state
`VERIFICATION_RECORD.template.md:30` lets a governance decision move to `superseded`, and the state the repository's only
prior renumbering was taken in. By the time the rename was instructed, `origin/master` had advanced to `6b02573` and
**its record was `verified` too**, binding `41c20cad`. Two `verified` homonyms cannot both stand, and the one on `master`
is the one every other branch has already inherited. So the side that was still cheap to move stopped being cheap first,
and the more expensive move — this one — became the only one available.

**The cost that follows from that, stated plainly.** `VREC-MOK-014` was `verified` on the owner's instruction at commit
`3c3c2e4`, and `evidence/WO-MOK-014/assurance-decision.md` records that decision. This rename changes the name of the
artifact that decision was taken about, and the path of the file recording it. The exposure was measured and written into
that file *before* the instruction to rename was given, and the owner instructed the rename with it in front of them.
Nothing here re-opens the record's lifecycle: it is still `verified`, it still binds `65ac88b`, and no field of it was
re-measured.

**The numbers were re-swept before being taken**, across all thirty-two refs in `refs/heads` and `refs/remotes`, by
enumerating each ref's tree rather than by reading one working tree. `WO-MOK-014`, `VER-MOK-014` and `VREC-MOK-014` are
above the maximum on every ref, which is `013` for all three. `REQ-MOK-050` is both above the maximum — `049`, on
`master` — and the lowest free number, because `REQ-MOK-001` through `REQ-MOK-049` are all present somewhere. No ref
holds any of the four as a path, and no ref mentions any of the four in content except this branch's own text.

## What was rewritten, and what was not

The rename was applied by byte substitution across every tracked file, reading and writing bytes rather than lines. That
matters on this repository: the working tree is CRLF under `core.autocrlf`, and both `sed` and Python's `write_text`
silently normalize line endings, which would have rewritten every line of every file they opened and buried the four
identifier changes in thousands of ending changes.

**At `3c3c2e4`, the commit before this one**, the four names occurred **632 times across 36 files** under `docs/` —
`WO-MOK-013` 339, `VER-MOK-013` 154, `REQ-MOK-047` 99, `VREC-MOK-013` 40 — plus **12 occurrences in 3 files** outside
it: `mokiterions-core/Cargo.toml`, `scripts/check_declared_dependencies.py` and
`scripts/test_check_declared_dependencies.py`. **26 tracked paths** carried one of the names in the path itself.

**Rewritten — 25 files.** The four artifacts; `ADR-MOK-006`, `ARCH-MOK-001`, `ARCH-MOK-002`, `SPEC-MOK-002` through
`SPEC-MOK-005`, `REQ-MOK-026`, `REQ-MOK-036` and `docs/engineering/REPOSITORY_CONTEXT.md`; in this packet, the five
authored prose files — `WO-MOK-014-amendments.md`, `WO-MOK-014-completion-summary.md`,
`WO-MOK-014-manual-assessment.md`, `WO-MOK-014-review-gate.md`, `WO-MOK-014-transition.md` — and `assurance-decision.md`;
the two retained scripts `WO-MOK-014-capture.sh` and `WO-MOK-014-injection.sh`; and outside `docs/`,
`mokiterions-core/Cargo.toml` and the two `scripts/` modules.

The two retained scripts were rewritten deliberately, on the precedent of the `007` → `010` renumbering and for its
stated reason: each is retained so a figure can be reproduced from the recorded command rather than trusted, and a
script whose output paths point at a directory that no longer exists reproduces nothing. Their changed occurrences are
output paths and header strings.

`mokiterions-core/Cargo.toml` and the two `scripts/` modules were rewritten for the identifier alone. No behaviour, no
declared dependency, no test expectation and no exit code differs; the checking program's own test suite passes
unchanged.

**Not rewritten — the 14 retained `.txt` captures in this directory, which are byte-identical to their pre-rename
blobs.** Fifty occurrences across them still read an old name — `WO-MOK-013` 30, `VER-MOK-013` 18, `REQ-MOK-047` 2, and
`VREC-MOK-013` none, since no capture predating the record names it:

```
WO-MOK-014-build-scripts.txt        1    WO-MOK-014-graphs.txt                1
WO-MOK-014-check-run.txt            3    WO-MOK-014-harness.txt              12
WO-MOK-014-check-tests.txt          1    WO-MOK-014-injection.txt             2
WO-MOK-014-counts.txt               1    WO-MOK-014-offline-build.txt         5
WO-MOK-014-determinism-manifest.txt 1    WO-MOK-014-scan.txt                  3
WO-MOK-014-determinism.txt          7    WO-MOK-014-workflows.txt             5
WO-MOK-014-features.txt             1    WO-MOK-014-gates.txt                 7
```

These are program output and the capture script's own headers over it. **Editing them would make the packet assert that
a tool printed something it did not print, which is a worse defect than a stale name: a capture that has been improved
is no longer a capture.** That is the rule the `007` → `010` renumbering set for exactly this case, and the rule this
packet already applies to `WO-MOK-014-review-gate.md:189`, whose retained gate transcript reads
`Decision required (1): VREC-MOK-013 [ready]` and is not corrected either. The mechanical pass did rewrite all fourteen;
they were reverted afterwards and each was checked byte-for-byte against its blob at `3c3c2e4`.

**They name this work order as `WO-MOK-013` because that is what it was called when they were taken.** A reader who
meets that identifier in a `.txt` in this directory should read it as *this* work order under its former name, and should
**not** follow it to `master`'s `WO-MOK-013`, which is unrelated presentation work. The filenames around them carry
`WO-MOK-014`, so this packet is mixed on this point by design.

Two consequences of that are worth naming for whoever reproduces a figure. `WO-MOK-014-harness.txt:84` records that every
file in this packet is named `WO-MOK-013-*` and that `W-HEX-001` therefore does not fire on the work order — the
observation still holds, under the new prefix, and the capture states it under the old one. And re-running
`WO-MOK-014-capture.sh` writes to `evidence/WO-MOK-014/` while the captures beside it were written by the same script
pointing at `evidence/WO-MOK-013/`, so a byte comparison against a retained file must be made on content, not on the
header line, which will differ by one character.

**Restored by hand after the mechanical pass — 49 occurrences in 3 files.** A byte substitution cannot tell this chain's
identifiers from `master`'s, and it rewrote both. Restored:

- `assurance-decision.md`, 23 — every reference to `master`'s four artifacts, and every count of *pre-rename*
  occurrences of the old names. A collision measurement written in post-rename identifiers measures nothing. The
  section is kept as it was written, with a later-fact block quote at its head recording that the collision it describes
  has since been resolved this way.
- `VREC-MOK-014.md`, 23 — the same, in the record's *Read this first* section, which was rewritten for this act on the
  precedent of `VREC-MOK-011`'s. It now leads with the renumbering and keeps the collision measurement below it in the
  identifiers it was taken in.
- `ADR-MOK-006.md`, 3 — the ADR's *Why this is `006` and not `005`* passage names the identifiers that were checked free
  when it was written, which were the old ones. Rewriting it would have made it assert that `REQ-MOK-050`,
  `VER-MOK-014` and `WO-MOK-014` were swept in a sweep that never looked at them. The passage now records both sets, and
  its own closing sentence — *"if that branch renumbers into them, the collision is resolved by whichever chain has not
  yet landed"* — turned out to describe this act.

## The digests, and why the rename invalidates none of them

Checked rather than assumed, because the `007` → `010` renumbering did invalidate one and the check is cheap:

- **`Cargo.lock`** — `WO-MOK-014-offline-build.txt` records `sha256 4154a6f1…` before and after the offline build. The
  lockfile is untouched by the rename. Its blob is `824003e1` at this commit, at `ff3a155` and at `origin/master`
  `6b02573`, so all three trees hold byte-identical lockfiles and the recorded digest still reproduces.
- **The two workflow files** — `WO-MOK-014-workflows.txt` records `d97c267b…` for `.github/workflows/release.yml` at 625
  lines and `871d1fec…` for `.github/workflows/dependency-declarations.yml` at 108 lines. Neither file contains any of
  the four identifiers and neither was touched. Both digests and both line counts still reproduce.
- **The 90 determinism cells** in `WO-MOK-014-determinism-manifest.txt` are digests of raw replay stdout. No stream
  contains an artifact identifier. Untouched, and they still reproduce.
- **No capture in this packet records a digest of any file the rename rewrote.** `mokiterions-core/Cargo.toml`,
  `scripts/check_declared_dependencies.py` and `scripts/test_check_declared_dependencies.py` are the three rewritten
  files outside `docs/`, and no retained digest covers any of them. Searched for, not presumed.

**The two harness snapshot digests are a different matter and were already so.** `WO-MOK-014-harness.txt:51,53` record
`4de12c63…` for the baseline tree and `2d709365…` for the candidate. `build_snapshot` embeds `git rev-parse HEAD`
twenty-five times and the checkout's leaf directory name once, so neither figure was ever reproducible anywhere but in
the clone and at the commit that produced it. The rename adds a second reason — the hashed document contains artifact
identifiers and the evidence path index, both of which moved — but it takes away nothing that was there.

## What the rename does to `VREC-MOK-014`'s own provenance

`commit` still reads `65ac88b0076dc1044adb4e6e984256b4428892b4`; `verified_at` still reads `2026-08-20T15:29:34Z`;
`status` is still `verified`. **A rename is not a re-capture**, and the record still claims exactly the tree that commit
holds — a tree in which all four artifacts carry the **old** names.

- **`artifact_snapshot_sha256 = a12ec1a3…` is unchanged and is reproducible only at `65ac88b`.** It is deliberately not
  re-stated against the renamed tree. The hashed document contains the evidence path index, twenty-one of whose paths
  now have different names, so a re-taken value would be a different measurement wearing the same field name — and it
  would no longer be the digest the assurance decision was taken over. `VREC-MOK-011`'s renumbering reported one digest
  as unpreservable rather than re-stating it, and this is the same choice.
- **`evidence_paths` were rewritten**, to the twenty-one `WO-MOK-014-*` names the files now carry, and the two declared
  relations now read `WO-MOK-014` and `VER-MOK-014`. Those must resolve for the validator to read the record at all, and
  `VREC-MOK-011` was renumbered the same way, 222 paths at once. The bytes of every file they name are unchanged except
  for the identifier itself, and for fourteen of them not even that.
- **`title` still reads *"Verification candidate for WO-MOK-014"***, keeping the word the capture used.
- **This file is not in `evidence_paths`,** on the same precedent as `assurance-decision.md`: a record's evidence paths
  name what the accountable owner accepted, and both files postdate the candidate commit. Adding either would also move
  `artifact_snapshot_sha256`, since the path index is inside the hashed document.

## What the rename does to the merge, which is the act it was asked for

`git merge-tree --write-tree --messages HEAD origin/master`, against `6b02573`:

| | before the rename | after |
|---|---|---|
| add/add — `REQ-MOK-047`, `VER-MOK-013`, `VREC-MOK-013`, `WO-MOK-013`, `evidence/WO-MOK-013/assurance-decision.md` | **5** | **0** |
| content — `ARCH-MOK-001`, `SPEC-MOK-002`, `SPEC-MOK-003`, `SPEC-MOK-004` | 4 | 4 |
| **total** | **9** | **4** |

The rename removes every add/add conflict and leaves four ordinary ones, in each of which both chains appended an
amendment row to a table. Those are merge work and not collision work.

The fifth add/add is worth a sentence, because it appeared late. Both chains had by then written an
`assurance-decision.md` into their own `evidence/WO-MOK-013/`, so a directory overlap that had produced **no** reported
conflict at all became a real one the moment the second file was created.

**That overlap is the part of the collision no tool would have shown**, and renaming the directory is what removes it.
Before the rename, `master`'s pack held 27 files under unprefixed names and this one held 22 — twenty-one prefixed with
the work-order identifier and `assurance-decision.md`, which is the one path the two had in common and the whole of the
reported conflict. A merge would have produced a single directory holding two unrelated
work orders' evidence — nothing reported by `git merge-tree`, nothing reported by the validator, because every other
path in it is distinct. After the rename they are two directories and the question does not arise.

## Measured after the rename, in this clone at `HEAD 3c3c2e4`

A renumbering that moved any of these would not be a renumbering.

| Reading | Result |
|---|---|
| `validate_engineering_artifacts.py` | PASS — 107 artifacts, 0 errors, 0 warnings across all four planes, exit 0 |
| `generate_harness_dashboard.py` | PASS — 107 artifacts, 357 relations, 0 errors, 7 warnings, exit 0 |
| `inspect_engineering_artifacts.py` | 20 findings — error 0, warning 7, info 13 |
| `W-HEX-001` / `W-HEX-003` / `I-REV-001` | 2 / 5 / 13 observations |
| Decision required / Definitions pending / Active work / Assurance pending | 0 / 1 (`WO-MOK-008` `[draft]`) / 0 / 0 |
| Suggested next steps | 8 |
| `preflight --work-order WO-MOK-014 --phase review` | PASS, exit 0, work order read as `implemented` |
| `doctor .` | 81 PASS, 0 FAIL, 0 WARN, exit 0 |
| Python suite | 126 tests, OK |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0 |
| `cargo test` | 212 tests across 21 binaries, all passed |
| `check_declared_dependencies.py` | *Every declared set matches its resolved graph. 8.4a-8.4d pass.*, exit 0 |
| observer crate counts | 57 / 63 / 62 on the three release targets, union 66 — declared and resolved equal, unchanged |

**Every figure above is identical to the reading taken at `3c3c2e4` before the rename**, and identical is the point.
Two things are worth drawing out of the table rather than leaving in it:

- **`W-HEX-001` still reports exactly two observations, `WO-MOK-010` and `WO-MOK-011`, and does not report this work
  order.** That is the check that the *filenames* were renamed and not merely the directory. Evidence discovery keys on
  files whose names begin with the work-order identifier, so a rename that moved `evidence/WO-MOK-013/` to
  `evidence/WO-MOK-014/` and left twenty-one `WO-MOK-013-*` files inside it would have made this work order's evidence
  undiscoverable and added a third observation. It did not.
- **`check_declared_dependencies.py` now prints `VER-MOK-014` where the retained capture beside it prints
  `VER-MOK-013`.** That is the documented consequence of rewriting the script and not the capture, visible in one
  character, and it is what *Not rewritten* above predicts rather than a defect found afterwards.

**No snapshot digest pair is quoted, and that is deliberate.** The dashboard reports
`ab87146ab4a9df2082bb536588d8d53406885d74c658496ee9d07de7d9d9ee83` for the renamed tree. A before-and-after pair would
carry no information here: artifact identifiers and the evidence path index are both inside the hashed document, so this
digest moves *by construction* under any renumbering, and it would move again on commit because `build_snapshot` embeds
`git rev-parse HEAD` twenty-five times. What the pair cannot tell a reader is whether anything else moved, which is what
the fourteen rows above are for.

## What this act does not discharge

The renumbering resolves an identity collision. It does not make this packet current, and it is not the merge.
Independently of it, and recorded here so the two are not confused:

- **The merge has not happened.** Pull request #33 is open against a base `master` has moved fourteen commits past. Its
  five green checks were computed at `baseRefOid ff3a155` and are not evidence of anything about `6b02573`. Merging is
  the engineering owner's separate act.
- **A verification record bound to the merge commit is owed and is not created here.** `VREC-MOK-014` binds `65ac88b`,
  which is an ancestor of neither `origin/master` nor the released `755db72`. Renaming it does not extend its reach by
  one commit.
- **Every figure in this packet stated relative to `ff3a155` still needs re-deriving against the merged tree** — the
  amendment-row comparisons in `WO-MOK-014-amendments.md` and the `W-HEX-003` baseline in particular. `master`'s change
  surface is favourable and was checked: `mokiterions-tui/src/{layout,render,verification}.rs` and their three test
  files, with no engine source, no manifest, no lockfile, no workflow and nothing under `scripts/`. So the engine replay
  hashes, the declared dependency sets and `check_declared_dependencies.py`'s behaviour are unaffected; the test census
  is not, because `master` added TUI tests.
- **`VER-MOK-005` and `VER-MOK-008` stay owed**, at the four assertion sites `assurance-decision.md` item 1 names, in a
  separate work order after #33 merges. `master` has since amended `VER-MOK-005`, so those coordinates must be
  re-derived against the merged tree before that repair is attempted.
- **`VER-MOK-014`'s assessment 7 is not discharged**, only found not due. The first admission of a crate beyond
  `ratatui` owes it by name.
- **`REL-MOK-001` is closed by its own approval and is not amended**, including its withdrawn one-crate clause at
  `:94-96`. `RLS-MOK-001` released 0.1.0 from a commit that does not include this work. No release record binds it and
  none is created here.

Every command behind every figure in this file is offline, reads no credential, secret, token or environment value, and
none appears in this file or in any retained evidence.
