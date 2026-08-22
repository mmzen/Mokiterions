+++
id = "VREC-MOK-022"
type = "verification_record"
title = "Verification candidate for WO-MOK-024"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-22"
updated = "2026-08-22"
commit = "cdd88ddb1528681916dcb4523aab9ffbb67c4380"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-22T09:45:19Z"
artifact_snapshot_sha256 = "ad6256ba1ecce185aa168164ed96d3c39f5c479caa6b476f777da4279c7bbf84"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-024/README.md", "docs/engineering/simulation/evidence/WO-MOK-024/after/engine-usage.txt", "docs/engineering/simulation/evidence/WO-MOK-024/after/observer-usage.txt", "docs/engineering/simulation/evidence/WO-MOK-024/before/engine-usage.txt", "docs/engineering/simulation/evidence/WO-MOK-024/before/observer-usage.txt", "docs/engineering/simulation/evidence/WO-MOK-024/defaults-divergences.txt", "docs/engineering/simulation/evidence/WO-MOK-024/drift-demonstration.txt", "docs/engineering/simulation/evidence/WO-MOK-024/engine-usage.diff.txt", "docs/engineering/simulation/evidence/WO-MOK-024/gates.md", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/README.md", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/both-paths.txt", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/capture.sh", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/exit-codes-after.txt", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/exit-codes-base.txt", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/field-comparison.txt", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/manifest-after.txt", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/manifest-base.txt", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/resilience-10k-after.txt", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/resilience-10k-base.txt", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/resilience-10k.sh", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/summaries-after.txt", "docs/engineering/simulation/evidence/WO-MOK-024/nonperturbation/summaries-base.txt", "docs/engineering/simulation/evidence/WO-MOK-024/observer-usage.diff.txt", "docs/engineering/simulation/evidence/WO-MOK-024/preflight-implemented.txt", "docs/engineering/simulation/evidence/WO-MOK-024/resilience-and-interface.txt", "docs/engineering/simulation/evidence/WO-MOK-024/usage-text.md"]

[relations]
verifies_work_order = ["WO-MOK-024"]
conforms_to = ["VER-MOK-004"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-024` to candidate commit `cdd88ddb1528681916dcb4523aab9ffbb67c4380`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## The assurance decision, taken 2026-08-22

**The assurance owner accepted this record and it is `verified`.** The words were *"i accept the record, you can
transition it, commit, push and PR"*, given on 2026-08-22 after the record was presented complete, and they are
quoted rather than paraphrased because the acceptance is the act and the wording is all there is of it.

**This transition moved the `status` field and nothing else.** The title still says *candidate*, the two
paragraphs above are the ones `harnessctl capture-verification` wrote, and every claim, table, figure and limit
below is the candidate's own prose as the owner read it. Nothing was strengthened after acceptance and nothing
was softened; a record that is edited to look better than what was accepted is not the thing that was accepted.
`harnessctl` 0.4.0 has no `transition` subcommand, so the field was hand-edited, which is why this section
exists to say who decided and on what.

**The figures were re-measured in this commit, because a `verified` record can never be corrected.** All of
them held: 303 tests passed with 0 failed and 0 ignored, `fmt` and `clippy` clean, `validate` 149 artifacts
with 0 errors and 0 warnings with this file present and 148 without it, `doctor` 81 of 81 PASS,
`preflight --phase review` PASS and `--phase start` FAIL by design, both help texts re-rendered byte-identical
to `after/engine-usage.txt` and `after/observer-usage.txt`, the twenty-six `evidence_paths` equal to
`git ls-files` for the directory, exactly one `pub` pair in the diff against the base and that pair the same
item, and the one rustdoc warning still the inherited one. The identifiers were re-checked against every local
and remote ref after a fetch: `WO-MOK-024` and `VREC-MOK-022` are used by nothing but this branch.

**One statement in the retained evidence has aged and cannot be edited.** `evidence/WO-MOK-024/README.md`
calls the base commit `f7b1c452039dc2f03010ca8b8cc81e73c54727c0` "the branch point and the tip of `master`".
It is still the branch point and still an ancestor of `master`, but `origin/master` has since moved to
`d4a17c1280e15d4a62c99756023c5896ee31136f`, ten commits ahead. That file is bound by this record and editing it
would falsify the binding, so the correction is stated here instead: read "the tip of `master`" as of
2026-08-22 at capture. **This branch must not be rebased.** A rebase would rewrite
`cdd88ddb1528681916dcb4523aab9ffbb67c4380` and orphan the commit this verified record binds, and a verified
record can be neither re-pointed nor superseded to repair that. If integration needs `master`'s later work,
merge it.

**Release is a separate act and is not taken here.** `verified` says the evidence was reviewed and accepted
against `VER-MOK-004`; it does not authorize a release, a tag or a publish, and `REL-` records exist for that.

## What this record claims

`WO-MOK-024` is `implemented` and `VER-MOK-004` is `approved`, as amended twice on 2026-08-22. At candidate
commit `cdd88ddb1528681916dcb4523aab9ffbb67c4380` all nine items of the work order's *Required verification*
were executed, together with `VER-MOK-004`'s requirement-to-evidence matrix, its acceptance scenario 3, its
non-perturbation rows, its static and interface checks and its performance and resilience checks. **Every row
of the amended contract is claimed met**, and the amendments are named where they bear so that a reader can
see which text each row was measured against.

The evidence is the 26 files enumerated in `evidence_paths` above. `gates.md` maps each of the nine items to
its result. `usage-text.md` holds both texts as rendered, the width measurements, and each fact the new text
states traced to the approved artifact that already stated it. `nonperturbation/`,
`defaults-divergences.txt` and `resilience-and-interface.txt` are the contract's own rows executed on both
sides rather than argued from the diff.

**This record is the fourth capture of this work order's verification, and the three it replaces are named
rather than quietly dropped.** They bound commits `1b4ee1e`, `9153f35` and `a3a65ab`, and the first three were
captured under the identifier `VREC-MOK-020`. The first was overtaken by a correction to `usage-text.md`, which
would have falsified its binding had the file been edited under it; the second by amendment 6, ratified after
it was taken; the third by the renumber described below, which moved every one of its twenty-six
`evidence_paths`. A verification record cannot be re-pointed at a later commit, cannot be re-pathed after
capture, and the harness has no `superseded` status for one, so each was deleted and re-taken. All three stood
at `ready` and **none was ever accepted by the assurance owner**, so nothing accepted was withdrawn. Act 10 of
`WO-MOK-024`'s *Decision record* is the owner's direction for the third; act 11 is what forced this one.

**This work order was `WO-MOK-020` and this record was `VREC-MOK-020` until 2026-08-22.** Both identifiers were
already in use on the remote, which no local checkout reveals: `VREC-MOK-020` is a `verified` record on
`origin/master` that binds `WO-MOK-017`, and `WO-MOK-020` is held by an open draft pull request on
`origin/feature/observer-mokiterion-profile`. The renumber is act 11 of *Decision record*, taken by the
engineering owner, who declined the recommendation that the other pull request move instead. **The renumber
changed no measurement in this record and none was re-taken to suit it** — but the code gates were re-run
anyway, because four `.rs` doc comments name the work order, and both help texts were re-rendered and compared
byte for byte against the retained `after/` texts to show that no rendered output moved. Five fenced
`harnessctl` transcripts in `gates.md` still print `WO-MOK-020`, and the superseded `preflight` capture is
retained verbatim beneath its replacement, because rewriting the identifier inside a transcript would make it
a fabrication rather than a record.

## What this record does not claim, and what changed about that

**Four of `VER-MOK-004`'s checks could not be satisfied as written, and they are amended rather than
reinterpreted.** They are stated first, before the results, because a record that puts its exclusions last
invites a reader to stop before reaching them.

The four are fixed by **amendment 6** of `WO-MOK-024`, ratified on 2026-08-22 by the owner in the assurance
owner role as acts 7, 8 and 9 of that work order's *Decision record*, and carried in `VER-MOK-004`'s *Second
amendment of 2026-08-22*. It changes no code, no test and no rendered text. **An implementation agent cannot
amend the contract it is measured against**, which is why the two earlier captures of this record left these
rows unmet and why this one claims them only after the owner acted. What each row said, what was measured,
and what now stands in its place:

1. **The *Line width* row was false in both of its clauses.** It read "No line the options block introduces
   exceeds 80 columns; the reproduced synopsis block, whose first line is 81 columns before and after, is
   unchanged." The engine's before synopsis first line is **49** columns and the widest line in that block is
   68; the two lines that are 81 columns are prose lines 28 and 32, outside the synopsis. And the synopsis is
   **not** unchanged — ratified amendment 2 replaced `<u64>` with `<number>` in it, which is the one thing
   that clause asserts did not happen. The replacement row is a bound over the whole of both rendered texts:
   the engine's widest line is **80** columns after and 81 before, the observer's **79** after and 84 before,
   and the count of lines past column eighty falls from 2 to 0 and from 6 to 0. Those are the figures
   measured before the amendment was drafted, in `usage-text.md`; none was taken afterwards. The declined
   alternative was to strike the row, which would have left `REQ-MOK-018`'s amended 80-column bound with no
   verification row in any contract.
2. **The *Test placement* row named a file the oracle cannot live in.** It required the added test to live in
   `tests/cli.rs`. This work order adds no test there. It adds one to `mokiterions-tui/tests/options.rs`,
   because the oracle is an identity between two constants in two packages and the engine's test target
   cannot see the observer's; the same contract's amendment of 2026-08-22 added that identity as a required
   row, so as written it demanded a check and forbade its only possible location. It does change
   `mokiterions-core/tests/cli.rs` — **twelve lines added and three removed** — but all of it is one helper,
   `options_block()`: its terminating condition, because the new block separates entries with blank lines
   where a blank line used to close the block, and the doc comment above it. That file holds **eighteen tests
   before and eighteen after** and not one of its assertions is touched. **An earlier draft of this record
   said "exactly one line", which was the count of changed behavior and not the count of changed lines; the
   diff is the authority.** The replacement row asks for the public tier of the package that owns each
   oracle, and keeps every other clause: no new test file in either package, no test moved between tiers, and
   `SPEC-MOK-002` rule 8's subject discipline within each package. The declined alternative was to move the
   test into the engine, measured as needing either an inverted dependency — which breaks this contract's own
   empty-dependency-table row — or an `include_str!` across the package boundary.
3. **The *Performance and resilience* survivor counts described a tree that no longer exists.** The row fixed
   `8, 11, 8, 9, 11` on seeds `0, 1, 42, 123, 777` and asked for identity with those numbers rather than for
   clearing the floor. Measured at the base commit they are `8, 9, 10, 9, 9`.
4. **And so did its "same termination ticks" clause.** `reference` at density `0.75` now runs to the
   10,000-tick limit with 2 survivors; the contract's 2026-08-17 baseline recorded extinction at tick 9154.

For 3 and 4 the base and the candidate agree **exactly**, so what moved is `master` between 2026-08-17 and
2026-08-22 — phase 3 and the resource-composition ceiling both landed — and not this change. Read against the
fixed numbers, the clause would report a regression this work order did not cause. The replacement bullets
require identity with a baseline captured at the work order's own base commit, which is the stronger of the
two forms available: weakening to `REQ-MOK-014`'s floor of eight was put and declined, because it would let a
later change shift survivor counts unnoticed. That floor holds on every seed on both sides as well.

**One correction reached beyond the four rows.** `VER-MOK-004`'s *Independence* section disclaimed column
widths, and item 1 puts a measured width bound in the matrix; the two cannot both stand. The contradiction is
older than this work order — the *Line width* row and the disclaimer coexisted from 2026-08-17 — and the
amendment resolves it by owning the bound `REQ-MOK-018` now fixes while continuing to disclaim alignment,
wrap-point choice and rendering below eighty columns. That sentence was not separately put to the owner; it
is the stated consequence of act 7, and it is disclosed here and in `WO-MOK-024` rather than left for a
reader to find.

## Gates at the candidate commit

| Gate | Result |
|---|---|
| `cargo test --locked` | **303 passed, 0 failed, 0 ignored**; one invocation, no extra flag and no environment variable |
| `cargo fmt --all -- --check` | no differences; the `concat!` arms are one literal per line, which `rustfmt` leaves alone |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, no warning; the repository's own gate command, not narrowed, and no `allow` attribute added |
| `cargo doc --no-deps --lib -p Mokiterions` | **30 public items**, and one warning that is **inherited** rather than introduced |
| Dependencies | `Cargo.toml`, both member manifests and `Cargo.lock` are byte-identical to the base commit |
| Artifact validation | PASS — **148 artifacts, 0 errors, 0 warnings**, all four planes E0/W0, at the commit this record binds; PASS at **149** once this record's own file is present |
| `harnessctl preflight --phase review` | **PASS** at `implemented`, exit 0, and **no `Diagnostics:` block at all** — not a block with zero entries |
| `harnessctl preflight --phase start` | **FAIL by design**, exit 1, `W005`: that phase admits `approved` and `in_progress` only. Retained because the work order once claimed the diagnostic clears on the transition to `implemented`, and it does not |
| `harnessctl doctor` | PASS — **81 of 81 checks**, no WARN and no FAIL, 31 managed files `unchanged` |

Toolchain: rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable, Python 3.14.6, se-harness 0.4.0
from the pinned 0.4.0 environment rather than the machine-wide 0.4.1, which fails this repository on template
skew alone.

The three harness readings were taken a fourth time on the tree that carries amendment 6 and a **fifth** on
the renumbered tree this record binds, returning the same figures both times, which is what should happen:
neither act changed an artifact the harness reads for structure, and the renumber changed only which
identifier it prints. The `preflight` figures above are from the fifth reading, taken against `WO-MOK-024` and
retained verbatim at the head of `preflight-implemented.txt`; `gates.md`'s opening paragraphs record the
capture points, and its own identifier note explains why three fenced transcripts still name `WO-MOK-020`.
The code gates in this table are the re-run described in *Limits of coverage* item 2, on the renumbered tree,
not the original run at `6d6be40`.

### The before-and-after facts, measured rather than carried forward

`VER-MOK-004`'s figures were fixed on 2026-08-17 for `WO-MOK-004`, and both of its amendments of 2026-08-22
record that they are historical and not restated. Everything below was therefore re-derived against **this**
work order's base commit, `f7b1c452039dc2f03010ca8b8cc81e73c54727c0`, in a real `git worktree` created at
that commit and built there — not reconstructed from a diff and not reused from the earlier record.

| Measure | Engine before | Engine after | Observer before | Observer after |
|---|---:|---:|---:|---:|
| Lines | 38 | **71** | 22 | **84** |
| Non-blank lines | 34 | **59** | 17 | **69** |
| Bytes | 2196 | **3601** | 1114 | **4208** |
| Widest line | 81 | **80** | 84 | **79** |
| Lines past column 80 | 2 | **0** | 6 | **0** |
| Options-block entries | 7 | 7 | **0** | **8** |
| `Default: ` statements | 4 | 4 | 0 | **6** |
| Carriage-return bytes | 0 | 0 | 0 | 0 |

The observer had no options block at all before: its whole text was a synopsis and a paragraph, and its one
default was stated in prose as "defaults to". Both texts now state every default in the entry of the option it
belongs to. Rows four and five are the measurement the amended *Line width* row asks for.

**One byte figure appears twice with two values, and the difference is the measuring instrument.**
`nonperturbation/exit-codes-*.txt` records the help case's standard output as 2195 bytes before and 3600
after, one less than the table above on each side. `capture.sh` measures with
`printf '%s' "$stdout" | wc -c`, and `stdout=$($BIN ...)` strips the trailing newline before `wc` ever sees
it. The stream is 2196 and 3601 bytes; the capture is internally consistent and one byte short on both sides,
so the comparison it exists to make is unaffected. The retained texts in `before/` and `after/` are the
stream.

### The oracles, and why none of them is a tautology

`VER-MOK-004` requires that no check be a tautology over `cli::USAGE`, because the process test asserts the
program's output *equals* that constant and would hold for every possible value of it, including an empty
string. Four independent oracles lie outside the constant and each could have failed.

1. **The approved artifacts, read before the text was written.** `usage-text.md` traces every fact the new
   text states to the artifact that already stated it: the four `--policy` descriptions to `SPEC-MOK-001`'s
   *Decision sources* and its trait and fear rules; `--density`'s 8192 cells to the world geometry, and its
   two further constraints — at least one resource per territory, at most 100 — to the *Inputs* bullet, where
   they had been all along and where the operator could not see them; what the program prints and all three
   exit codes to *Outputs* and *Help output*. **Nothing in either text was invented by the implementation.**
   Because the trace is prepared from the specification, it cannot be satisfied by making the text agree with
   the code.
2. **The program's applied configuration.** For each value-taking option the stated default is scraped out of
   the printed text, fed back through `cli::parse`, and required to equal the configuration an empty argument
   list produces. The test declares no expected value, so it fails if either side moves alone.
3. **The recorded pre-change baseline, re-derived.** All 43 matrix cells byte-identical on digest, line count
   and exit code; all 40 matrix summary lines identical; the two 20-tick traced runs identical by `diff -r`.
4. **The cross-target identity.** `mokiterions-tui/tests/options.rs` holds each of the four entries the
   observer shares with the engine equal to `mokiterions::cli::USAGE`'s own text. `concat!` accepts only
   literals, so the shared text is a copy; this test is the only thing that keeps the two copies from
   drifting, and it is a test rather than a review note because review cannot be re-run on a future change.
   It is also the row that made the old *Test placement* clause unsatisfiable.

### The demonstrations that the oracles bite

Every scratch change below was reverted and none is committed. After each, the tree was checked back to its
committed state, and after the last the program's live `--help` was diffed against the retained
`after/engine-usage.txt` at exit 0 — the stronger check, because it confirms the reverted tree prints the text
this record binds.

| # | Divergence | Result |
|---:|---|---|
| — | one shared entry edited in the observer's copy only, one character | `mokiterions-tui --test options` **FAILED**, naming `--density`; 9 passed after the revert |
| A | applied `--ticks` default `100` to `1000`, text untouched | **FAILED**, 12 passed and 6 failed |
| B | printed `--seed` default `0` to `1`, parser untouched | **FAILED**, 17 passed and **exactly 1** failed |
| C | a `"--verbose"` arm added to `parse` with no help entry | **FAILED**, 17 passed and exactly 1 failed |

**B is the load-bearing result and the reason these were re-run rather than cited.** `VREC-MOK-004`
demonstrated the same three divergences against the text this work order replaced, so citing it would have
been citing a demonstration about text that no longer exists. On the rewritten text, a one-character edit to
prose inside a `concat!` arm fails exactly one test of eighteen,
`each_documented_default_parses_to_the_applied_default`, and its message names the option and both values.
Without that test the program could advertise a default it does not apply with the whole suite green. C fails
`the_documented_options_are_exactly_the_options_the_parser_accepts`, which prints both sorted lists.

### Non-perturbation, executed on both sides

`WO-MOK-024`'s original case for the contract's three non-perturbation rows was that no engine source is in
the diff. That is true — the diff against the base touches nothing under
`mokiterions-core/src/simulation.rs` — and it is a claim about a diff where the rows are claims about every
run. The rows were executed.

`nonperturbation/capture.sh` is `WO-MOK-004`'s script copied **byte for byte**, `cmp`-clean, so the cells are
the cells the contract fixed rather than a set chosen afterwards: five seeds, two decision sources, two
declared densities and two trace settings at 1,000 ticks, plus two 20-tick traced runs and the no-argument
invocation.

| Comparison | Result |
|---|---|
| all 43 matrix cells | **identical** |
| all 40 matrix summary lines | **identical** |
| the two 20-tick traced runs, `diff -r` | **identical** |
| the 16 named cases, on the four fields the contract fixes | **one changed of sixteen**, and it is `--help` |
| both emission paths | **byte-identical**, 3601 bytes and 71 lines each, equal to the retained text |

Each of the fourteen invalid-configuration cases holds exit `2`, an empty standard output and a byte-identical
`configuration error:` line; their standard error grew from 39 to 72 lines because the usage block appended
after that line grew from 38 to 71, which is the change this work order exists to make. The valid single-tick
case is byte-identical in every field, 14386 bytes at exit `0`. Neither emission path emits a carriage
return, because both constants are one literal per line with an explicit newline escape rather than a
multi-line literal that would inherit its endings from the checkout — a property the observer's constant
acquires here and the engine's already had.

### Resilience and the public surface

10,000 ticks at seed 123, at both declared densities, under **all four** decision sources — wider than the
row, which names only the two that existed when it was written — base against candidate: the two captures are
byte-identical, every row exits `0`, and survivors plus deaths equal twelve in every run. The 1,000-tick
survivor floor is `8 9 10 9 9` on both sides against a floor of eight. Under the amended bullets, base-against
-candidate identity is what the rows require and this is it; the two figures the earlier text of the contract
fixed, and which these measurements diverge from, are items 3 and 4 above.

The diff against the base over both `src` trees returns exactly **one pair of `pub` lines**, and they are the
same item: the observer's `USAGE` becoming a `concat!` of one literal per line. Name, visibility and type are
unchanged, both constants remain `pub const USAGE: &str`, and no public item was added, removed or changed in
signature, so `SPEC-MOK-002` rules 5 and 6 hold.

## Limits of coverage stated in this record

1. **The four rows above are met under an amendment ratified the same day the work was done, not under the
   contract as it stood on 2026-08-17.** That is the honest description and it is first because it is the one
   a reader most needs. Nothing was weakened to reach it: the width bound covers more text than the row it
   replaces, the placement rule keeps every clause but the filename, and the resilience checks keep identity
   with a measured baseline rather than dropping to a floor. What a reader must accept to accept these four
   is the owner's judgment that a contract row naming wrong numbers is better corrected than measured
   around — recorded as acts 7 to 9 with the declined alternatives beside them.
2. **The code gates were re-run for the renumber, and the tree they ran on is the bound commit's code.**
   They were first taken at commit `6d6be40`; eight commits separate that from the commit this record binds,
   and the diff across them is 38 paths, of which **four are `.rs` and none is a `.toml` or the lockfile**. The
   four are one line each — `cli.rs`, `tests/cli.rs`, `options.rs` and `tests/options.rs`, one doc-comment
   mention of the work order apiece, `+1/-1`. Because those four moved, `cargo fmt --all -- --check`,
   `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` and `cargo test --locked`
   were re-run on the renumbered tree: clean, clean, and **303 passed with 0 failed and 0 ignored**. The only
   files edited between that run and the bound commit are two markdown documents in the evidence directory, so
   no `.rs`, `.toml` or lockfile byte differs between the tree the gates measured and the tree this record
   binds. What this record still does **not** claim is a re-run of items 7 to 9 — the non-perturbation matrix,
   the divergence demonstrations and the resilience runs were not repeated for the renumber, because they
   compare program behavior and the programs' output is byte-identical to the retained texts.
3. **The four full 20-tick traces are deliberately not retained**, about 160 KB per side. They were compared
   with `diff -r` and were identical, their digests and line counts are in both manifests, and `WO-MOK-004`
   already holds the full text of the same two runs.
4. **The demonstrations rest on transcripts of reverted scratch changes.** Nothing in the committed tree
   proves that divergence B fails; what is committed is the transcript, the confirmation that the tree
   returned to its committed state, and the test that would fail again.
5. **GitHub issue 40 is not fixed here.** The observer accepts `--events-path`, validates it and writes
   nothing. The owner deferred the fix on 2026-08-22; the observer's text now discloses the behavior until it
   is closed. No gate in this record covers a fix that was not made.
6. **Two amendment rows in `SPEC-MOK-001` do not render as table rows** — one above *Scope*, one separated
   from the table by a blank line. `validate` does not read markdown table structure, so its PASS is not
   evidence they are fine. Moving an approved row is the technical owner's act; this is an observation.
7. **One rustdoc warning exists and is inherited**: `public documentation for 'suffered' links to private item
   'SufferedAttack'` at `mokiterions-core/src/simulation.rs:1542`. The diff of that file against the base is
   empty. `cargo doc` is not one of the repository's gate commands, so nothing else would mention it.
8. **The artifact count differs by one between this commit and the branch tip**, 148 against 149, because this
   record's own file cannot be inside the commit it binds.
9. **The retained evidence was edited twice after it was measured, and neither edit moved a measurement.**
   After amendment 6, three files gained dated ratification notes — `gates.md`, `README.md` and
   `usage-text.md` — saying that the rows are amended rather than outstanding; the figures had been captured
   before the amendment was drafted, which is the order that makes them evidence. After the renumber, two
   files gained dated identifier notes — `gates.md` and `README.md` — and `preflight-implemented.txt` gained
   the post-renumber capture of both `preflight` phases above the superseded one. **The only figures that
   moved in either edit are the line-endings table's own row counts**, which count the files those edits
   changed; they were re-measured to a fixed point and re-verified programmatically across all thirty-five
   rows, and `gates.md` states which seven moved and that the four `.rs` rows did not. The three discarded
   captures are the audit trail: their bindings are in the git history of this file and of
   `VREC-MOK-020.md`.

## Authority

This record is `ready`. **`ready` is not `verified`**: it means prepared and reviewable, and the transition is
the accountable assurance owner's separate act. Nothing in this file was decided by the agent that wrote it,
and no figure above was chosen after seeing whether it would pass.

Two acts remain the owner's and are not anticipated here: **accepting this record**, in the assurance owner
role, and **release**, which requires a record again. Amendment 6 is no longer among them — it was put and
ratified as acts 7 to 9, and act 10 directed this re-capture. Nothing has been pushed and no pull request has
been opened.
