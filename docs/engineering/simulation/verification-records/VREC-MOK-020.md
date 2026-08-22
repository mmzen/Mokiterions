+++
id = "VREC-MOK-020"
type = "verification_record"
title = "Verification candidate for WO-MOK-020"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-22"
updated = "2026-08-22"
commit = "9153f35c295083c17e08ae989784394e87d76cd8"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-22T09:01:44Z"
artifact_snapshot_sha256 = "577139885511086cd20358e37059e5571d3b5afceead0fe7cd7fb83762faa73d"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-020/README.md", "docs/engineering/simulation/evidence/WO-MOK-020/after/engine-usage.txt", "docs/engineering/simulation/evidence/WO-MOK-020/after/observer-usage.txt", "docs/engineering/simulation/evidence/WO-MOK-020/before/engine-usage.txt", "docs/engineering/simulation/evidence/WO-MOK-020/before/observer-usage.txt", "docs/engineering/simulation/evidence/WO-MOK-020/defaults-divergences.txt", "docs/engineering/simulation/evidence/WO-MOK-020/drift-demonstration.txt", "docs/engineering/simulation/evidence/WO-MOK-020/engine-usage.diff.txt", "docs/engineering/simulation/evidence/WO-MOK-020/gates.md", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/README.md", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/both-paths.txt", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/capture.sh", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/exit-codes-after.txt", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/exit-codes-base.txt", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/field-comparison.txt", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/manifest-after.txt", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/manifest-base.txt", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/resilience-10k-after.txt", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/resilience-10k-base.txt", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/resilience-10k.sh", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/summaries-after.txt", "docs/engineering/simulation/evidence/WO-MOK-020/nonperturbation/summaries-base.txt", "docs/engineering/simulation/evidence/WO-MOK-020/observer-usage.diff.txt", "docs/engineering/simulation/evidence/WO-MOK-020/preflight-implemented.txt", "docs/engineering/simulation/evidence/WO-MOK-020/resilience-and-interface.txt", "docs/engineering/simulation/evidence/WO-MOK-020/usage-text.md"]

[relations]
verifies_work_order = ["WO-MOK-020"]
conforms_to = ["VER-MOK-004"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-020` to candidate commit `9153f35c295083c17e08ae989784394e87d76cd8`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## What this record claims

`WO-MOK-020` is `implemented` and `VER-MOK-004` is `approved`, as amended on 2026-08-22. At candidate commit
`9153f35c295083c17e08ae989784394e87d76cd8` all nine items of the work order's *Required verification* were
executed, together with `VER-MOK-004`'s requirement-to-evidence matrix, its acceptance scenario 3, its
non-perturbation rows, its static and interface checks and its performance and resilience checks.

The evidence is the 26 files enumerated in `evidence_paths` above. `gates.md` maps each of the nine items to
its result and states which of the contract's rows the evidence does not reach. `usage-text.md` holds both
texts as rendered, the width measurements, and each fact the new text states traced to the approved artifact
that already stated it. `nonperturbation/`, `defaults-divergences.txt` and `resilience-and-interface.txt` are
the contract's own rows executed on both sides rather than argued from the diff.

**Four of the contract's checks are not claimed, and they come first below rather than last.** A record that
puts its exclusions after its results invites a reader to stop before reaching them.

## What this record does not claim

Four of `VER-MOK-004`'s checks are not satisfied as written. All four are fixed by **amendment 6** of
`WO-MOK-020`, which is **OUTSTANDING**: it was found while executing this contract to prepare this record,
after the six acts the owner took on 2026-08-22, and it was not put to them. It changes no code, no test and
no rendered text. **An implementation agent cannot amend the contract it is measured against**, so these rows
stand unmet here rather than reinterpreted into a pass.

1. **The *Line width* row is false in both of its clauses.** It reads "No line the options block introduces
   exceeds 80 columns; the reproduced synopsis block, whose first line is 81 columns before and after, is
   unchanged." The engine's before synopsis first line is **49** columns and the widest line in that block is
   68; the two lines that are 81 columns are prose lines 28 and 32, outside the synopsis. And the synopsis is
   **not** unchanged — ratified amendment 2 replaced `<u64>` with `<number>` in it, which is the one thing
   this clause asserts did not happen. What the row was reaching for does hold, measured separately: the
   engine's widest line is **80** columns and the observer's is **79**, against 81 and 84 before, and neither
   text has a line past column eighty where both did before.
2. **The *Test placement* row names the wrong file.** It requires the added test to live in `tests/cli.rs`.
   This work order adds no test there. It adds one to `mokiterions-tui/tests/options.rs`, because the oracle
   is an identity between two constants in two packages and the engine's test target cannot see the
   observer's. It changes exactly one line of `mokiterions-core/tests/cli.rs`, a helper's block terminator,
   and touches none of that file's eighteen assertions. The row's intent — that a new oracle live in a
   committed test rather than in review prose — is met in a file the row does not name, which is not the same
   as the row being met.
3. **The *Performance and resilience* survivor counts describe a tree that no longer exists.** The row fixes
   `8, 11, 8, 9, 11` on seeds `0, 1, 42, 123, 777` and asks for identity with those numbers rather than for
   clearing the floor. Measured at the base commit they are `8, 9, 10, 9, 9`.
4. **And so does its "same termination ticks" clause.** `reference` at density `0.75` now runs to the
   10,000-tick limit with 2 survivors; the contract's baseline recorded extinction at tick 9154.

For 3 and 4 the base and the candidate agree **exactly**, so what moved is `master` between 2026-08-17 and
2026-08-22 — phase 3 and the resource-composition ceiling both landed — and not this change. Reading the
clause against those numbers would report a regression this work order did not cause. The floor of eight
survivors per seed that `REQ-MOK-014` states holds on every seed on both sides.

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

### The before-and-after facts, measured rather than carried forward

`VER-MOK-004`'s figures were fixed on 2026-08-17 for `WO-MOK-004`, and its amendment of 2026-08-22 records
that they are historical and not restated. Everything below was therefore re-derived against **this** work
order's base commit, `f7b1c452039dc2f03010ca8b8cc81e73c54727c0`, in a real `git worktree` created at that
commit and built there — not reconstructed from a diff and not reused from the earlier record.

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
belongs to.

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

`WO-MOK-020`'s original case for the contract's three non-perturbation rows was that no engine source is in
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
survivor floor is `8 9 10 9 9` on both sides against a floor of eight. The stale figures these measurements
diverge from are items 3 and 4 of *What this record does not claim*.

The diff against the base over both `src` trees returns exactly **one pair of `pub` lines**, and they are the
same item: the observer's `USAGE` becoming a `concat!` of one literal per line. Name, visibility and type are
unchanged, both constants remain `pub const USAGE: &str`, and no public item was added, removed or changed in
signature, so `SPEC-MOK-002` rules 5 and 6 hold.

## Limits of coverage stated in this record

1. **Four of `VER-MOK-004`'s checks are unmet as written**, above, pending **OUTSTANDING** amendment 6. This
   is the limit that matters, and it is repeated here so that a reader who skips to this section still meets
   it.
2. **The code gates were taken at commit `6d6be40`, not at the commit this record binds.** The two commits
   since change markdown only: the diff between them is 24 paths and not one of them is a `.rs`, a `.toml` or
   the lockfile. They were not re-run, and this record says so rather than implying a fresh run.
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

## Authority

This record is `ready`. **`ready` is not `verified`**: it means prepared and reviewable, and the transition is
the accountable assurance owner's separate act. Nothing in this file was decided by the agent that wrote it,
and no figure above was chosen after seeing whether it would pass.

Two further acts remain the owner's and are not anticipated here: **ratifying or declining amendment 6**, in
the assurance owner role, and **release**, which requires a record again. Nothing has been pushed and no pull
request has been opened.
