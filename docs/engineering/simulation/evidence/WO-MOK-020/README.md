# Evidence: `WO-MOK-020`, the cumulative activity profile and the population total

This packet is the evidence `VER-MOK-017` requires for `WO-MOK-020`. Twenty-three files, including this
one, the completion report and `MANIFEST.sha256`.

The **candidate** is the working tree at `ccb0584` plus this work order's change surface — four source
files and two specifications — committed as the commit that adds this directory. The **base** is `ccb0584`
itself, checked out in a separate worktree at `/c/Users/mathi/mok-base-ccb0584` so that every
before-and-after pair is two trees rather than one tree twice. That worktree is removed after the capture;
`git worktree add /c/Users/mathi/mok-base-ccb0584 ccb0584 --detach` recreates it.

Platform for every capture: Windows 11 Home 10.0.26200, `cargo` and `rustc` 1.97.1 as
`rust-toolchain.toml` pins them, `--locked` on every cargo command. The harness commands run from the
pinned 0.4.0 environment, `C:/Users/mathi/harness-venv-040/Scripts/python.exe -m se_harness`.

Every file carries its own header with its own command line, its provenance and its own result, which is
the form the earlier packets in this repository established. The table below is the index, not the record.

## The files

| File | What it is | Recompute |
|---|---|---|
| `01-format-candidate.txt` | Format gate at the candidate, exit 0. | `cargo fmt --all -- --check` |
| `02-lint-candidate.txt` | Lint gate at the candidate from a cleaned tree, so every unit is checked rather than replayed from cache. Exit 0, no warning. | `cargo clean -p mokiterions-tui -p Mokiterions` then `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` |
| `03-test-candidate.txt` | The whole suite at the candidate: **332 passed, 0 failed** across 19 targets. Also the transcript for O19.1–O19.4 and O20.1, which are tests. | `cargo test --workspace --locked` |
| `04-format-and-lint-base.txt` | The same two gates at the base, exit 0 each. | as 01 and 02, in the base worktree |
| `05-test-base.txt` | The whole suite at the base: **302 passed, 0 failed**. The platform's pre-existing failure profile is clean, so the candidate's 332 is measured against a green base rather than a tolerated one. | `cargo test --workspace --locked` in the base worktree |
| `06-engine-records-seed42-candidate.jsonl` | The engine's structured record stream, seed 42, in full. | see `08` |
| `07-engine-stream-seed42-candidate.txt` | The engine's operator-facing stream, seed 42, in full. | see `08` |
| `08-engine-output-unmoved.txt` | O17's engine half: ten digest pairs, records and stream for each of the five declared seeds, at both trees. **All ten identical.** | its own header, per seed |
| `09-measure-spec-mok-004.py` | The retained measurement script for `SPEC-MOK-004`'s rule 6, 9, 10 and 11 figures. Its head comment states each rule's counting definition and why the public and private counts are anchored differently. | — |
| `10-spec-mok-004-measured.txt` | The script's output at both trees, with three cross-checks and the engine's 157 attributed to `WO-MOK-017`/`26ae6ba`. Rule 9 103→117, rule 10 42→58, observer 145→175, workspace 302→332, and 30 arrivals matching the passing-test delta exactly. | `cargo test --workspace --locked -- --list > list.txt` then `python .../09-measure-spec-mok-004.py list.txt`, in each tree |
| `11-independent-count.txt` | O1: the accumulated table and the independently counted table for all five declared seeds — 60 subject rows and 60 all-zero difference rows — with the retained-state figure beside the tick it was reached at. | its own header |
| `12-frames-and-columns.txt` | Frame captures for four pane states at three viewports, plus the dead-selection pane, each row bar-bounded so trailing blanks are unambiguous; and O16's widest-line measurement, 40/35/33/40 columns of an interior of 42. | its own header |
| `13-extinction-frame.txt` | O14 and acceptance scenario 5: the extinction pane, `tick 119 living 0`, `initialized 12 deaths 12`, `by a strike 0 unattributed 12`, presented with nothing pressed. | its own header |
| `14-long-run-truncation.txt` | O8 and P4: the truncating run executed rather than reasoned about. `retained events 100000 of capacity 100000, truncated true, tick 6600`, and retained state 12 records / 1476 bytes. | its own header |
| `15-per-tick-cost-driver.rs` | The timing instrument. **Not part of the commit**: copied to `mokiterions-tui/tests/timing.rs` for the measurement and removed. | — |
| `15-per-tick-cost.txt` | The per-tick cost series, seven configurations A–G, with each repetition, the spread, the engine-alone scale figure and the conclusion. | its own header |
| `16-export-capture-driver.rs` | The export instrument. **Not part of the commit**: copied to `mokiterions-tui/tests/capture.rs` for the capture and removed. | — |
| `17-export-unmoved.txt` | O18's file half: the export written at both trees, byte-identical, with digests, the pane-vocabulary counts and the security counts over the same bytes. | its own header |
| `18-dependency-sets-unmoved.txt` | Constraint 4 and the preflight's dependency-set command: the engine resolves to one crate, the observer to 59, and no manifest or lock file is touched. | `cargo tree -p Mokiterions -e normal --locked --offline` and the same for `-p mokiterions-tui` |
| `19-harness-validate-and-preflight.txt` | The harness's own verdict at the candidate: `validate` PASS, 152 artifacts, 0 errors, 0 warnings; `preflight --phase review` PASS. | `python -m se_harness validate` and `... preflight --work-order WO-MOK-020 --phase review` |
| `completion-report.md` | `WO-MOK-020`'s completion report, in the six-point format that work order fixes. It is the packet's only file the manifest hashes but which cannot state the manifest digest, so that digest is in the commit message instead. | — |
| `README.md` | This file. |  — |
| `MANIFEST.sha256` | SHA-256 of every other file in the packet, in `sha256sum` format. | `sha256sum -c MANIFEST.sha256` in this directory |

## Where each retention bullet is answered

| `VER-MOK-017` *Evidence retention* bullet | Files |
|---|---|
| Full command transcripts: clippy, fmt, the whole test run, before and after | `01`–`05` |
| The independent-count comparison per declared seed, as the two counted tables and their diff | `11`, and `14` for the truncating run |
| Frame captures: selected living, selected dead, no-selection population, pre-tick-1 in both states, extinction, one at the presence threshold | `12`, `13` |
| The widest-line column measurement per pane state with the interior width beside it | `12` |
| The `SPEC-MOK-004` measurements with their commands | `09`, `10` |
| The pre-change and post-change export captures for O18, with digests | `17`, with `07` (see the deviation below) |
| The long-run capture for O8, including the `truncated` indicator's state | `14` |
| Performance and memory figures with the commands and the platform stated | `15-per-tick-cost.txt`, and `11` and `14` for memory |

The other checks: O2–O7, O9–O13, O15, P1–P6, O19 and O20 are tests, and their transcript is `03`;
`SPEC-MOK-004`'s interface census is `09` and `10`; the lint and format gate is `01`, `02` and `04`; the
overflow check is met as `VER-MOK-017` admits it — "the chosen width makes saturation unreachable for any
admissible run, which is stated with the arithmetic" — and the arithmetic is stated at
`mokiterions-tui/src/state.rs` on the `Profile` type, over `u64` counters bounded by `Config::tick_limit`.

## Where a bullet is met by substitution

Stated on the bullet, in the terms `VREC-MOK-019` used, rather than by reinterpreting the bullet.

1. **The export captures are retained once rather than twice, and by reconstruction rather than as a third
   copy.** The export is byte-identical at both trees, and it is byte-identical to `07` except for its last
   line: the engine's summary line against the rule 9.4 retention footer. `17` carries both digests, the
   one-line diff and the two commands that reconstruct the exact bytes, which were run and produce the
   stated digest. Retaining the file twice more would add 1,183,214 bytes of near-duplicate to the packet.
2. **The engine's own streams are retained in full for one seed of five.** `08` carries all ten digests and
   states which eight captures are deliberately not retained and how to recompute them. This is the
   deviation the earlier packets in this repository already established for multi-seed stream captures.
3. **No interactive terminal run is retained.** The review preflight's repository commands include running
   the observer with `cargo run -p mokiterions-tui` in an interactive terminal. No interactive terminal is
   available in this environment, so the pane evidence is the rendered-buffer captures in `12` and `13`
   instead. `VER-MOK-017`'s *Residual uncertainty* 2 already states what a frame capture evidences and what
   it does not, and this substitution does not narrow that further: it means no human eye has seen the pane
   at a real terminal, which is exactly what manual assessments 1, 2 and 7 are for and which is why they
   are outstanding rather than answered here.
4. **The per-tick cost is reported as a series and a bound, not as a before-and-after pair.** The bullet
   asks for wall-clock before and after with the variance stated. The pair alone reads as a 74% regression
   and is not one; `15-per-tick-cost.txt` reports all seven configurations, states that the difference is
   codegen-unit partitioning rather than accumulation, and reports the accumulation's own cost as an upper
   bound of about 25 µs/tick because the instrument cannot separate it from noise. The default-profile
   figure — candidate 266.3 µs against base 153.0 µs — is reported rather than suppressed, and it is real
   for anyone running that build.

## Security and privacy

`VER-MOK-017`'s three security items and what was done about each.

- **No total and no frame carries an environment value, a path outside the resolved export path, or an
  operator identity.** The frame half is a pre-existing test, `no_frame_carries_an_environment_value`, in
  `03`. The export half is measured on the exported bytes in `17`: zero occurrences of `C:`, `Users`, the
  capture host's user name, `AppData`, and of `TOKEN`, `KEY`, `SECRET` and `PASSWORD`. The only path in
  the run is the one `MOK_EXPORT_CAPTURE` named, and it appears in the driver's stdout, not in the file.
- **The retained state is in-process only and the export is unchanged.** The export capture run wrote
  exactly one file, the export, and its digest is unmoved. The profile records live in a
  `BTreeMap<String, Profile>` with no serialization path; `14` reports their size from that map.
- **No credential, token or provider secret is introduced, read or retained, and the packet is inspected
  for incidental capture before it is committed.** The inspection was run over the whole directory,
  case-insensitively, for `ANTHROPIC`, `api_key`, `apikey`, `sk-`, `ghp_`, `github_pat`, `token`, `secret`,
  `password`, `passwd`, `Bearer`, `AWS` and `PRIVATE KEY`. Every hit is accounted for: `token`, `secret`
  and `password` in `17`, as entries in that file's own vocabulary table; `AWS` in `03`, `05` and
  `15-per-tick-cost.txt`, matching only inside the word `draws` in test names; and every one of the
  thirteen in this file, because this paragraph names them. Nothing else matched, and no credential,
  token or provider secret is present. `MANIFEST.sha256` was written after this inspection, so the bytes
  it hashes are the bytes that were inspected.

The packet does carry absolute paths of this checkout and of the base worktree, and those contain the
operator's user name. That is provenance rather than a secret — it is what makes a transcript's tree
identifiable — and it is what the earlier packets in this repository retain for the same reason. The
security item above is about totals and frames, and no total and no frame carries either.

## What this packet does not establish

**That the change is verified.** Nothing here is a verification record. `WO-MOK-020`'s *Lifecycle* stops at
`implemented` once the code, the two specification amendments and this packet are complete; `verified`
needs a commit-bound record against `VER-MOK-017`, decided by the assurance owner, and `released` needs a
release record. The preflight in `19` says the same thing in its own last line: it "does not approve
artifacts, authorize a diff, verify work, release software, commit, push, tag, publish, or deploy."

**None of the seven manual assessments.** All seven are OUTSTANDING, and the ratification above does not
discharge any of them: it decided the substance assessments 3 and 4 turn on — item 7's amendment and the two
corrections — but an assessment is a statement recorded in a verification record, which does not exist yet. Three belong to the product owner
(1 and 2) and the technical owner (3, 4 and 7), one to the assurance owner (5), and one — assessment 6,
recomputing at least one amended `SPEC-MOK-004` figure independently of the retained command output — is
the assurance owner's and is deliberately **not** answered by `10`, which is the retained command output
it must be independent of.

**Both specification amendment rows were OUTSTANDING when this packet was captured and were ratified
afterwards.** The captures in `01`–`19` were taken against a tree in which both rows read OUTSTANDING, which is
what an amendment row the implementation agent wrote is until the accountable role signs it. The repository owner,
acting as technical owner, ratified both on 2026-08-22 in a separate governance act after this packet was complete:
`SPEC-MOK-003`'s nine provisions in one act, correction 2's fork decided explicitly in favour of narrowing rule 4's
prohibition to derivation; and `SPEC-MOK-004`'s figures with **a reconciliation row owed at the next merge**, because
the `WO-MOK-008` branch changes two of the same source files and a trial merge measures rules 9, 10 and 11 at 119,
66 and 342 rather than the 117, 58 and 332 this packet retains evidence for. Both rows carry that act's own record.
This paragraph is the only statement in the packet that the ratification changes, and no captured transcript was
re-taken for it.

**That the figures survive a merge.** Every figure here is measured against `ccb0584`. `master` has moved
before in this repository and will again; on a merged tree the gates, the census, the counts and the
frames need re-running rather than carrying over, and a record bound to a merge commit is a new record
rather than an edit of one bound to a branch commit.
