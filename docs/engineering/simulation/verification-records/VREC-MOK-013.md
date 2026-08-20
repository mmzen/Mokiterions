+++
id = "VREC-MOK-013"
type = "verification_record"
title = "Verification candidate for WO-MOK-013"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-20"
commit = "65ac88b0076dc1044adb4e6e984256b4428892b4"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-20T15:29:34Z"
artifact_snapshot_sha256 = "a12ec1a37930487c5ddec9ef6fd6f9802489a6a459ac78f5b0536b975ee7acf6"
evidence_paths = [
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-amendments.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-build-scripts.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-capture.sh",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-check-run.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-check-tests.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-completion-summary.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-counts.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-determinism-manifest.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-determinism.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-features.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-gates.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-graphs.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-harness.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-injection.sh",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-injection.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-manual-assessment.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-offline-build.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-review-gate.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-scan.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-transition.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/WO-MOK-013-workflows.txt",
]

[relations]
verifies_work_order = ["WO-MOK-013"]
conforms_to = ["VER-MOK-013"]
+++

# Verification Record Candidate

This `ready` record binds the retained evidence for `WO-MOK-013` — the declared dependency set that replaces the
engine's empty-dependency rule — to candidate commit `65ac88b0076dc1044adb4e6e984256b4428892b4` on
`governance/adr-mok-006-third-party-crates`. An accountable assurance owner must review the evidence and decide whether
to transition it to `verified`. **Preparing it approved, verified, merged, tagged, released and published nothing.**

**This record was first prepared against `d33ecb81062db005ee916fdb78be52bed7f142e4` and re-pointed on 2026-08-20**, on
the repository owner's instruction, when the review gate on pull request #33 forced a correction to `WO-MOK-013`'s
declared requirement set and moved the branch tip past that commit. *Why this record was re-pointed* below records the
act, what moved, and the figures that did not; the figures for `d33ecb8` are kept there as figures of that commit
rather than deleted.

Two sections are load-bearing for that decision and are not summary:

- *The snapshot figures, and what they are figures of* corrects the provenance of three digests already committed under
  this work order. None of the three is the digest of the commit it is presented against, this record's own
  `artifact_snapshot_sha256` is a fourth figure that none of them equals, and the reason is measured rather than
  argued. No earlier file is edited.
- *The one reading this record does not take* states the single place where `VER-MOK-013`'s own wording makes
  satisfaction a judgement instead of a count. `WO-MOK-013-manual-assessment.md:354` says a verification record against
  this contract is where that reading would be taken. **This candidate states it and does not take it**, because taking
  it is the transition and the transition is the assurance owner's.

The record is written after the candidate commit it names, so its own commit metadata is not self-referential and no
hash of its own commit appears in it, which is what `WORKFLOW.md` requires.

**`verified_at` is the capture timestamp, not a verification decision.** The figures below were taken from the
repository root at this commit with `git status --porcelain` empty, the harness figures at `2026-08-20T15:29:34Z` and
the code gates on the tree this commit records, byte-identical to it. What a status change would record, and when, is
not `verified_at`.

**`artifact_snapshot_sha256` is the digest of the graph this record reviews, which is the graph apart from this
record.** At the first candidate that needed no saying: the record did not yet exist in the tree it measured. At this
one it does, so the field is measured by the stated procedure — check out this commit, hold
`verification-records/VREC-MOK-013.md` out of the tree, run `python scripts/generate_harness_dashboard.py --root .` —
which yields `a12ec1a3…` over a **106**-artifact, **355**-relation graph. The digest of the commit as it actually
stands, with this record present at its previous figures, is `dcb28212…` over **107** artifacts and **357** relations;
it is recorded here so that a reader who runs the command without holding anything out can tell which figure they have.
Both are figures of a checkout whose leaf directory name is this repository's, for the reason *The snapshot figures*
gives below.

## What this record claims

### The candidate commit

| | |
|---|---|
| Commit | `65ac88b0076dc1044adb4e6e984256b4428892b4` |
| Branch | `governance/adr-mok-006-third-party-crates`, pushed to `origin` |
| Parent | `f40b711b0985316494c6d3ac24f9b0434b0f0ad7`, the attribution note on this record |
| Base | `ff3a155f3ce006fdc38abb62df3fca4a2c3c3aa3`, the tip of `master` this branch was cut from |
| Worktree | clean; `Cargo.lock` byte-identical to `origin/master` |
| Contents relative to its parent | two files — `WO-MOK-013.md`'s narrowed `implements`, its re-derived architecture paragraph and its new *The declared requirement set, corrected* subsection, and `WO-MOK-013-review-gate.md` |

The commits between the implementation and this candidate, each its own act:

| Commit | What it carries | What it leaves behind |
|---|---|---|
| `84b21b9cdc8566cf2bde45b47dae415944d19dca` | the whole change — four new artifacts, twelve amended, the checking program, two workflows, nineteen evidence files | `WO-MOK-013` at `in_progress` |
| `d33ecb81062db005ee916fdb78be52bed7f142e4` | the status change and `WO-MOK-013-transition.md` | `implemented`; **the first candidate this record bound** |
| `5e45d950676d038187a144f21a80b43b138a0722` | this record at `status = "ready"` | the candidate awaiting review |
| `f40b711b0985316494c6d3ac24f9b0434b0f0ad7` | one dated note attributing the snapshot correction to a fact the repository already held | unchanged |
| `65ac88b0076dc1044adb4e6e984256b4428892b4` | the declared requirement set corrected, and `WO-MOK-013-review-gate.md` | **the candidate this record now binds** |

**Why the candidate is not the implementation commit.** `84b21b9` carries the whole change and leaves `WO-MOK-013` at
`in_progress`; `assurance_pending` — the queue this record answers — holds a work order only at `implemented`. A record
bound to `84b21b9` would bind a tree in which the work order does not yet claim to be done.

**Why it is not `d33ecb8` either, any longer.** The correction at `65ac88b` edits a governed artifact, so a record left
bound to `d33ecb8` would verify a graph the branch no longer has. `git diff --name-only 84b21b9 65ac88b` is five
governance files and no Rust source, manifest, lockfile, workflow or script, so every code figure in this packet is a
figure of all three commits by construction, and the re-run below is the check on that rather than the assertion of it.

### The gates, measured at this commit

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, no warning line, no `allow` added |
| `cargo test --workspace --locked` | exit 0 — 21 `test result:` lines, **212 passed, 0 failed, 0 ignored, 0 filtered out** |
| `python scripts/check_declared_dependencies.py --root .` | exit 0 — *"Every declared set matches its resolved graph. 8.4a-8.4d pass."* with `disclosed and accepted` printed for `mio 1.2.2` and `signal-hook-mio 0.2.5` |
| `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 126 tests`, `OK`, exit 0; the new module alone is 56 of them |
| `python scripts/validate_engineering_artifacts.py --root .` | PASS — 107 artifacts, 0 errors, 0 warnings, all four planes E0/W0 |
| `python scripts/generate_harness_dashboard.py --root .` | PASS — 107 artifacts, 357 relations, 0 errors, 7 warnings, snapshot `dcb28212…`; `a12ec1a3…` over 106 artifacts and 355 relations with this record held out |
| `python scripts/inspect_engineering_artifacts.py --root .` | 107 artifacts, 357 relations, **19** findings — error 0, warning 7, **info 12**, exit 0, `decision_required` holding `VREC-MOK-013` and `assurance_pending` empty. **Read at any `HEAD` other than this commit the same tree gives 20 and info 13**, for the reason below the table |
| `python -m se_harness doctor .` under the pinned **0.4.0** | exit 0 — 81 PASS, 0 FAIL, 0 WARN |
| `python -m se_harness preflight . --work-order WO-MOK-013 --phase review` under the pinned **0.4.0** | **PASS**, exit 0 — the gate that failed on pull request #33 before the correction |
| `git status --porcelain` after every check | empty |

**The inspector's finding count depends on the observed `HEAD` and not only on the tree, and both readings are
measured.** `I-REV-001` fires per verification or release record whose declared `commit` differs from the observed
revision — `generate_harness_dashboard.py:687` tests `match_state == "different"`, where the observed revision is
`git rev-parse HEAD`. This record declares `65ac88b`, so at a checkout of `65ac88b` it is *not* observed as different and
`I-REV-001` holds **12** observations, for 19 findings in total. At `f40b711` with this commit's content in the tree the
same three artifacts give **13** and 20, measured in a detached worktree rather than predicted. A reviewer who checks out
the branch tip — which is a later commit, since this record cannot travel in the commit it names — will therefore read
20 findings and info 13, and that is the expected reading rather than a discrepancy. Neither figure moves an error or a
warning, `decision_required` holds `VREC-MOK-013` at both, and `assurance_pending` is empty at both.

**One figure inside the evidence this record binds is conditioned the same way, and it is disclosed rather than
rewritten.** `WO-MOK-013-review-gate.md`'s gate transcript records 20 findings and info 13, and its route table records
the inspector as identical across the three measured routes. Both were taken with `HEAD` still at `f40b711`, before the
candidate commit existed, so they are faithful transcripts of the run that produced them and they are the figures a
reader at the branch tip will reproduce. A reader who checks out `65ac88b` exactly will read 19 and info 12 instead. The
difference is `I-REV-001` alone and the cause is the rule above, not a change in the graph — the route table's claim that
the inspector output is identical across the three routes holds at either `HEAD`, because none of the routes touches a
record's declared commit.

`pytest` is not installed in this checkout, which is why the Python suite is run through `unittest discover`; the
figure is the whole `scripts/` suite and not a subset. The doctor figure is the pinned **0.4.0** one, which is the
version `.github/workflows/engineering-harness.yml` installs by exact pin. The machine-wide 0.4.1 install reports exit
1 and eight `distribution:` FAILs against a repository whose declared runtime is 0.4.0; `WO-MOK-013-harness.txt`
records that reading, records the baseline reporting the same eight, and records that an earlier draft of that file had
it wrong. **A reader who measures doctor with 0.4.1 will not reproduce this line and should not.**

### The five oracles

`VER-MOK-013`'s *Independence* section names five, deliberately anchored outside the effort that made the change.

| Oracle | Retained in | What it returned at this commit |
|---|---|---|
| 1 — the resolved graph read from Cargo, never from a manifest or lockfile | `WO-MOK-013-graphs.txt`, `-counts.txt`, `-features.txt`, `-build-scripts.txt` | engine: 1 node, 0 external crates, on all three release targets. Observer: **57** external on `x86_64-pc-windows-msvc`, **63** on `x86_64-unknown-linux-gnu`, **62** on `aarch64-apple-darwin`, **66** in union, **71** at `--target all`, against **182** packages in `cargo metadata --locked`. `syn 1.0.109` and `thiserror 1.0.69` are in the lockfile and reachable from neither package at any edge kind |
| 2 — the declaration read from the specification text, not restated in code | `WO-MOK-013-check-run.txt`, `-check-tests.txt`, `-workflows.txt` | both directions equal for both packages; one declared entry each side for the observer, `ratatui 0.30.2`; features `crossterm`, `layout-cache`, `underline-color` plus resolver-implied `std`, with `default` and `serde` asserted absent; 56 tests over the reader and the comparison; neither workflow contains a crate name, version or feature name |
| 3 — a retained pre-existing capture as the determinism baseline | `WO-MOK-013-determinism.txt`, `-determinism-manifest.txt`, `-capture.sh` | 90 cells captured twice — 5 seeds × 3 policies × 3 densities × 2 trace modes at 1,000 ticks. **run A == run B on all 90, raw hash and exit code**, and **run A == `WO-MOK-011`'s retained post-change capture on all 90**. The four contract-named seed-`123` cells: `baseline 0.75%` `fcd03d6f…`, `baseline 1.50%` `44a448a1…`, `reference 0.75%` `cebe44c4…`, `reference 1.50%` `9621f5f8…`, each exit 0 |
| 4 — an independently written prohibition list | `WO-MOK-013-scan.txt` | 126 terms — 104 across the six classes `ADR-MOK-006` decision 4 prohibits, plus 22 user-interface terms for the engine graph. **Raw hits before any disclosure: exactly `mio 1.2.2` and `signal-hook-mio 0.2.5`, on Linux and macOS.** Residue after disclosure: none. A positive control is retained, and matching is by token and not substring, so the `tui` in four `ratatui*` names and the `ws` in `windows-link` are near-misses a substring rule would have hit |
| 5 — the governance state of what this change amends | `WO-MOK-013-amendments.md` | all sixteen entries of `ADR-MOK-006`'s *Required amendments* present; the twelve landing in a governed artifact carry an approval naming the role the ADR assigned; the four carrying no authority present and consistent; `ARCH-MOK-001` carries both replacement conformance checks; every pre-2026-08-20 amendment row in the amended artifacts byte-identical to `ff3a155` |

Two further files carry the demonstrations the contract asks for by name rather than an oracle each:

- **`WO-MOK-013-injection.txt` — the check refusing, ten ways.** One declaration edited per case on a scratch copy,
  **23 refusal lines and exit 1** across the ten, covering a moved version, a removed declared entry, an undeclared
  manifest entry in each package, a feature arriving by unification, a build-script table that no longer matches, a
  disclosure row the graph does not reach, and a declared prohibited-class name that no disclosure can admit. The
  scratch root is removed and the checkout is not edited, which the program's own `git status` comparison asserts.
  This is what distinguishes a check that passes from a check that cannot fail.
- **`WO-MOK-013-offline-build.txt` — the engine offline.** `cargo build -p Mokiterions --locked --offline` and
  `cargo test -p Mokiterions --locked --offline` succeed with the registry unreachable, from the committed lockfile,
  with the working tree after identical to the working tree before. Before `ADR-MOK-006` this property was a side
  effect of an empty dependency table; it is now asserted instead of inferred, and the file states plainly that it
  establishes nothing about the observer.

### The manual assessments

`VER-MOK-013` declares six, numbered 1, 2, 3, 4, 6 and 7 with no fifth. The gap is preserved on the owner's decision
of 2026-08-20 rather than closed, because renumbering would break twenty citations by number across `SPEC-MOK-003`,
`SPEC-MOK-005`, `VER-MOK-013`, `WO-MOK-013` and the checking program. `WO-MOK-013.md` said *"seven"*; that was the
error and it is corrected in the work order with the correction disclosed.

| # | Assessment | Role | State at this commit |
|---|---|---|---|
| 1 | No declared entry implements simulation semantics | technical owner | **Recorded 2026-08-20**, with the line for future entries stated: a declared crate may compute the presentation of a value the engine produced and may not compute the value |
| 2 | Decision 1's criteria applied to each entry | technical owner | **Recorded 2026-08-20**, applied to `ratatui 0.30.2` retrospectively rather than grandfathered, with the measured debt — 57/63/62 crates, 7/9/9 build scripts, the disclosed `mio` capability — named as the yardstick for the next admission |
| 3 | The reach of the by-name scan | assurance owner | **Recorded 2026-08-20**: the 126 terms reach the six classes as far as a name can, the insufficiency is stated as a limit and not a caveat, and four compensating controls are named and locatable |
| 4 | The determinism baseline substitution | assurance owner | **Recorded** by the contract's own approval of 2026-08-20; the precision correction it needed — `VREC-MOK-002` holds no replay hash, the four are in `evidence/WO-MOK-002/determinism-and-resilience.md` — is applied to both artifacts that carried it |
| 5 | — | — | Does not exist |
| 6 | The disclosed transitive capabilities | technical owner | **Accepted 2026-08-20** on three grounds together, **limited to a compiled and uncalled capability and void the moment a behavior calls it**. It was `OUTSTANDING` when the contract was approved and was recorded later the same day |
| 7 | The strength this change gives up | technical owner | **Not yet due.** It triggers at the first admission of a crate beyond `ratatui`, and this change admits none |

Assessment 6 is the one the implementation produced rather than anticipated, and it is the finding that mattered most in
this work order. Rule 8.4d's by-name scan hits this repository's own graph: `mio 1.2.2` is reached through `ratatui` →
`ratatui-crossterm` → `crossterm` with its `net` feature on, compiling TCP and UDP socket types into two of three
release builds, and `signal-hook-mio 0.2.5` matches the same token. A term list omitting `mio` would have made the scan
pass by construction, so the rule gained a disclosure mechanism instead of a shorter list, and the program prints the
row whether or not the disclosure is accepted — because an acceptance that removed the line would be
indistinguishable from the disclosure never having existed. **An implementation agent may measure a transitive
capability and may not accept one.** The acceptance is the technical owner's, recorded in `SPEC-MOK-003`'s disclosure
table where a reader looking at the crate will land.

## The snapshot figures, and what they are figures of

Three snapshot digests are already committed under this work order. **None of them is the digest of the commit it is
presented against, and this record's `artifact_snapshot_sha256` is a further figure none of them equals.** The cause is
mechanical, it is not a defect in the change under verification, and it is measured here rather than argued.

`build_snapshot` puts the repository's **current** `HEAD` into the snapshot: `git_revision`
(`scripts/generate_harness_dashboard.py:143`) is read at `:1264` and lands in `repository.revision` at `:1295`, in each
of the twelve `revision_provenance` entries' `observed_revision`, and in twelve `findings[*].evidence` strings as
`observed=<sha>` — **25 occurrences of the same forty characters** in the serialized snapshot the digest is taken over
(`:1445`, `:1458`). It also puts the repository root's **directory name** into `repository.name` at `:1294`. So the
digest is a function of the governed graph, of the evidence path index, of `git rev-parse HEAD`, and of what the
checkout is called. A digest measured in a working tree before that tree is committed therefore cannot equal the digest
of the commit that tree becomes: committing changes `HEAD`, and `HEAD` is inside the figure.

| Digest | The graph it is over | `HEAD` when it was measured | Where it is recorded |
|---|---|---|---|
| `2d709365ad39ad1ff367a315265a26b4f557cdebbcc05a70520065cfca04c530` | commit `84b21b9`'s graph exactly — 19 evidence paths, `WO-MOK-013` at `in_progress` | `ff3a155`, before the commit existed | `WO-MOK-013-gates.txt:465`, `WO-MOK-013-harness.txt:53`, and `84b21b9`'s commit message |
| `3e5b93963cfc051d60f6239dc4a0755596f927543d9e4b414a807543d95ffbec` | the status change alone, still 19 evidence paths | `84b21b9`, before this commit existed | `WO-MOK-013-transition.md:80` |
| `7dacb4d581429010ad5bd3c8bcbd3a11aed32c9f4836ff2b173ec2dee569724e` | commit `d33ecb8`'s graph exactly — 20 evidence paths, `implemented` | `84b21b9`, before this commit existed | `WO-MOK-013-transition.md:66` and `:81`, and this commit's message |
| `df7b935514139d38ba368188e7d2611d7fff4a578f957f37d9c1a33949f75a75` | commit `84b21b9`'s graph | `84b21b9` | **this record — the digest that commit carries** |
| `ada8d829f6cdf4a1cfc7552acb9b0f244dd9c52522f34b9f71e672473344a597` | commit `d33ecb8`'s graph — 106 artifacts, 357 relations | `d33ecb8` | **this record's `artifact_snapshot_sha256` while it was bound to `d33ecb8`** — the digest that commit carries |
| `a12ec1a37930487c5ddec9ef6fd6f9802489a6a459ac78f5b0536b975ee7acf6` | commit `65ac88b`'s graph with this record held out of the tree — 106 artifacts, 355 relations | `65ac88b` | **this record's `artifact_snapshot_sha256`** |
| `dcb282120c06ae129fb75b14264e511eb6fa5743db7a97e402d99c01cf7796f3` | commit `65ac88b` exactly as it stands, this record present at its previous figures — 107 artifacts, 357 relations | `65ac88b` | this record, so a reader who holds nothing out can tell which figure they have |

Each row of the first three is a byte-level reconstruction and not an inference. Taking the JSON the dashboard wrote at
this commit and replacing the 25 occurrences of `d33ecb8…` with `84b21b9…` yields `7dacb4d…` exactly. Removing the
twentieth evidence path from the two lists that hold it — the `evidence` entry and the readiness gate condition that
repeats it — and applying the same substitution yields `3e5b939…` exactly. Measuring at `84b21b9` in a detached
worktree, correcting the single occurrence of that worktree's directory name to the repository's and rewinding the
revision to `ff3a155…`, yields `2d70936…` exactly. `df7b9355…` was predicted by that same substitution and then
**measured directly**, in a detached worktree at `84b21b9` whose leaf directory name equals this repository's.

What this corrects, and what it leaves standing:

- **`WO-MOK-013-transition.md:81` labels `7dacb4d…` *"the figure this commit carries"*. It is not.** It is the figure of
  this commit's content measured while `HEAD` was still the parent. The figure this commit carries is `ada8d829…`. The
  same reading applies to `2d70936…` in `WO-MOK-013-gates.txt:465` and `WO-MOK-013-harness.txt:53`, and to both commit
  messages, which cannot be edited on a pushed branch. **Nothing is rewritten**; both figures are given here so a
  reader at either commit can follow them, on the precedent this packet already uses for its two corrected harness
  readings.
- **The transition record's reasoning is unaffected.** It separates two causes of one moving number — the governed
  status change, then the new evidence *path* entering the index — and both of its figures were taken at the same
  `HEAD`, which the reconstruction above proves. Its statement that no evidence file's **content** reaches the snapshot
  is confirmed: `discover_evidence` (`:370`) returns paths and reads no bytes.
- **The graph itself moved by exactly four fields between the two commits.** A field-by-field comparison of the two
  snapshots shows `WO-MOK-013`'s `status`, the twentieth evidence path in the two lists that hold it, the readiness
  condition that reads `unsatisfied` at `in_progress` and `satisfied` at `implemented`, and the revision and name
  fields. Nothing else. Artifact, relation, finding and severity counts are identical across the two: 106, 357, 19, and
  0/7/12. Those are figures of `84b21b9` and `d33ecb8`, before this record existed and before the correction at
  `65ac88b`; that the relation count there also reads 357 is arithmetic coincidence and not the same 357 — it is
  `implements` holding three targets with this record's two relations absent, where `65ac88b` is `implements` holding one
  with those two present.
- **This is a provenance defect in three recorded figures, not a finding against the change.** Nothing in
  `WO-MOK-013`'s change surface touches the dashboard, its inputs or its serialization, and the counts at both commits
  are identical.

**No digest anywhere in this record is a figure of an uncommitted working tree**, which is the mistake this section
documents. The two figures for `65ac88b` are both figures of that commit, distinguished only by whether this file is
held out of the tree while the dashboard runs, and each is reproducible by the stated procedure in a checkout whose leaf
directory name equals this repository's. What the record's presence does to the graph is recorded as counts below.

**No digest is given for the commit that carries this re-point, and none can be.** The same mechanism that produced the
three corrected figures forbids it: `build_snapshot` writes `git rev-parse HEAD` into the hashed document, so the digest
of this file's own commit does not exist until that commit does, and a figure measured beforehand would be a working-tree
figure presented as a commit figure — the exact error above. It is the snapshot analogue of `WORKFLOW.md`'s rule that a
record cannot contain the hash of its own commit. The two `65ac88b` figures are given instead, because `65ac88b` is the
commit this record verifies and it existed before this text was written. A reader who wants the digest of this record's
own commit measures it at that commit; nothing here claims to have.

Both `65ac88b` figures were re-measured from a clean checkout at that commit while this re-point was being written, and
both reproduced exactly. **That check also demonstrates why the field is defined record-absent rather than as-committed:**
re-pointing this record edits its own front matter, and front matter is inside the hashed document, so an as-committed
digest would be a figure the act of recording it destroys. The record-absent digest is invariant under every edit to this
file, which is what makes it a figure of the graph under review rather than of the review.

**Note added 2026-08-20, after this record was first committed: this repository had already written the fact down, so
the three figures are a lapse and not a discovery.** `evidence/WO-MOK-011/assurance-decision.md:113-120` corrects an
earlier sentence of its own with exactly this: *"`build_snapshot` in `scripts/generate_harness_dashboard.py` puts
`repository.name` — the checkout directory's name — and `repository.revision`, which is `git rev-parse HEAD`, into the
hashed document"*, and concludes that *"a snapshot digest is comparable only against another taken in the same clone at
the same `HEAD`"*, citing `evidence/WO-MOK-011/merge/gates.txt` saying its own digest *"is not the digest of any commit
and no record binds it"*. That correction is dated 2026-08-20 and is in `master` at `ff3a155`, the commit this branch
was cut from — so it was available when every figure in this packet was captured, and the packet recorded three
pre-commit digests as commit figures anyway. Attributing that to the reader would be wrong; it was the implementation
agent's to read.

The same passage settles one further thing this record depends on: the digest hashes *"the normalized front matter, the
relations and the findings"* and not artifact prose. So this note cannot move `artifact_snapshot_sha256`, and the field
still binds the graph it was captured over. That was confirmed rather than taken on trust — with this note in the
working tree at the record's own commit, the dashboard prints the same digest it printed before the note was written.

## Why this record was re-pointed

This record was first committed at `5e45d95` bound to candidate `d33ecb8`. It now binds `65ac88b`, and the reason is a
finding neither this record nor the packet it binds could have raised: **the harness's `review` phase had never run
against this work order.** Opening pull request #33 ran it, and it failed with one diagnostic —

    [W016] docs/engineering/simulation/work-orders/WO-MOK-013.md: verification coverage is missing REQ-MOK-026,
    REQ-MOK-036

— because `WO-MOK-013` declared three requirements under `implements` while `VER-MOK-013`, its only declared
verification contract, verifies `REQ-MOK-047` alone. `REQ-MOK-026` is verified by `VER-MOK-005` and `REQ-MOK-036` by
`VER-MOK-008`, the two contracts the repository owner decided on 2026-08-20 to leave for a separate change. The rule is
this repository's own, in `TRACEABILITY.md`, and the check is `se_harness/preflight.py` in the pinned candidate runtime.
Both the selection step and the review preflight are guarded by `if: github.event_name == 'pull_request'`, so no push
run of this branch could have reached the phase, and the four green ones did not.

**The correction the owner chose narrows `implements` to `["REQ-MOK-047"]`**, on the reading that this change struck a
clause from each of the other two requirements under `ADR-MOK-006`'s product-owner authority rather than implementing
their obligations. `evidence/WO-MOK-013/WO-MOK-013-review-gate.md` — the twenty-first evidence file, and part of what
this record binds — carries the finding, the offline reproduction, the three measured routes, the route that was refused
and why, and the cost the narrowing incurs. That cost is not restated as satisfied here: after the correction no
declared relation runs from either amended requirement to the work order that rewrote its text, and preflight's reading
manifest for a reviewer shrinks from 23 artifacts to 19.

**Why the record moved with it rather than staying put.** The correction edits a governed artifact, so a record left
bound to `d33ecb8` would name a graph the branch no longer has, and `TRACEABILITY.md` requires a record to bind one
clean final candidate commit. Re-pointing was the repository owner's instruction, taken as accountable engineering
owner in the same answer that chose the narrowing.

**What moved in the record, and what did not.** `commit`, `verified_at`, `artifact_snapshot_sha256` and one added
`evidence_paths` entry moved; `status` is still `ready`, `worktree_state` still `clean`, `git_object_format` still
`sha1`, and both declared relations are unchanged. Every figure in *The gates* was re-measured at the new candidate: the
same 212 Rust tests, the same 126 Python tests, the same dependency-check verdict with both disclosed transitive matches
still printed, the same 81 doctor PASS, the same validator PASS at 107 artifacts and 0/0, and the same dashboard 107 and
7 warnings. **Two figures moved and neither was left to be inferred.** The dashboard's relation count is 357 where this record's
first version measured 359 in a tree holding both this record and the un-narrowed `implements`, and the difference is
`implements` losing two targets and nothing else. The inspector's finding count reads 19 with
info 12 at a checkout of `65ac88b` where it read 20 with info 13 before, which is `I-REV-001` comparing this record's
declared commit against the observed `HEAD` and no longer finding them different; the paragraph under *The gates* gives
both readings and the measurement behind each. The five oracles, the twelve acceptance scenarios and the six manual assessments are the same evidence
against the same contract; the correction touched no oracle, no scenario and no assessment. **The re-point is not a
re-verification and asserts no new satisfaction** — it moves the commit this evidence is bound to, and the assurance
owner's reading is as open at `65ac88b` as it was at `d33ecb8`.

**One earlier statement in this record is superseded rather than rewritten.** Its first version said no digest was
quoted for a tree holding this record. Two are now, both figures of `65ac88b` and neither of an uncommitted tree, for
the reason the section above gives. The figures for `d33ecb8` are kept in that section's table as figures of that
commit.

## What a `ready` record does to the graph

This record is already in the tree at the candidate commit, so its effect is measured as the difference the file makes
there rather than as a before-and-after across commits:

- the validator reports **107 artifacts, 0 errors, 0 warnings** across all four planes;
- the dashboard reports **107 artifacts and 357 relations**, and **106 artifacts and 355 relations** with this file held
  out — the difference is this record and its two declared relations, and nothing else — with the warning count
  unchanged at **7** either way;
- the inspector reports `decision_required` holding `VREC-MOK-013` and `assurance_pending` **empty**: `WO-MOK-013` is not
  in that queue because the queue holds an `implemented` work order that no verification record at `ready` or beyond
  covers through a declared `verifies_work_order` relation, and this record is that cover;
- and it reports **19 findings, error 0 / warning 7 / info 12** at a checkout of the candidate commit, **20 and info 13**
  at any other `HEAD`.

Those are the check on what a candidate is for: it raises the signal that an accountable review is owed and changes no
error or warning count at either reading. The one finding that moves with the re-point is `I-REV-001`, and it moves for a
reason worth stating plainly rather than as a count: **it is a comparison against the observed `HEAD`, so this record
contributes an observation exactly when it is read from a commit other than the one it names.** Bound to `d33ecb8` and
read at `5e45d95` or `f40b711` it contributed one. Bound to `65ac88b` and read at `65ac88b` it contributes none — 12
observations. Read from the commit that carries this text, or from any later one, it contributes one again — 13. **The
re-point does not resolve that finding and is not an attempt to**; it changes which commit the record is out of step
with, and a reader at the branch tip should expect to see it fire. It is the same informational finding every other
verification record in this repository carries, and the reason it exists.

## What this record does not claim

- **It does not claim any crate was admitted.** This change admits none. `Cargo.lock` is byte-identical to
  `origin/master`, no Rust behaviour changes, and the declared set for the observer is the one entry `ADR-MOK-003`
  already decided.
- **It does not claim the replaced property was preserved.** *The engine's dependency table is empty* could not be
  satisfied by a crate that lied about itself; *the resolved set equals the declared set* can. `ADR-MOK-006` states
  that as its first negative consequence and `VER-MOK-013` states that no arrangement of checks here restores it.
- **It does not claim set equality over the transitive graph.** Equality is checked over direct declared entries; the
  66 crates across three targets are held by `--locked`, by the build-script table and by the by-name scan. A crate
  that changed behaviour without changing its name, version, features or build script would pass every check in the
  contract. `VER-MOK-013` names this as its largest residual.
- **It does not claim absence of network capability.** `mio`'s `net` feature compiles TCP and UDP socket types into the
  Linux and macOS observer builds today. What assessment 6 accepted is a compiled and uncalled capability, and the
  acceptance is void the moment a behavior calls it. A reader who takes a passing 8.4d as *no socket code in the
  binary* has read it wrong.
- **It does not claim the scan is exhaustive.** It is by name. `mio` was legible as an I/O crate and nothing here would
  have surfaced the same capability inside a crate named after what it renders.
- **It does not claim determinism over the input space.** 90 cells at five declared seeds, three policies, three
  densities, two trace modes and 1,000 ticks, on one platform. A crate introducing nondeterminism only past tick 1,000,
  only at an undeclared density, or only on a fourth platform would pass.
- **It does not claim the two placements were differentially tested.** They read one declaration, so they cannot
  disagree about what is declared; nothing here proves they cannot disagree about what is resolved on a runner whose
  target differs from the one under test. `dependency-declarations.yml` is `pull_request`-triggered, and it **first ran
  when pull request #33 was opened** — run `32371118176`, job `Declared sets`, success in 22s, after four `push` runs on
  this branch had skipped every pull-request-only step. That is one runner on one target, which is exactly what this
  bullet says is not settled.
- **It does not claim the declaration is wise.** Whether a crate belongs in this repository at all is decisions 1, 10
  and 11 — three judgements by the technical owner — and no evidence substitutes for one.
- **It does not verify the merge commit, and it is not a merge, a tag or a release.** Pull request #33 exists and is a
  **draft**; taking it out of draft is the owner's act, merging is a further one, and neither is this. `RLS-MOK-001` is
  untouched.
- **It does not claim declared verification coverage for `REQ-MOK-026` or `REQ-MOK-036`.** After the correction at
  `65ac88b`, `WO-MOK-013` declares one requirement and this record conforms to the one contract that verifies it. The two
  amended requirements keep the text the product owner approved, and the coverage they are owed sits with `VER-MOK-005`
  and `VER-MOK-008`, which stay owed. `WO-MOK-013-review-gate.md` states the cost of that in full, and this record does
  not restate it as settled.
- **It closes nothing outstanding elsewhere, and one item on that list is created here rather than inherited.**
  `VER-MOK-005` and `VER-MOK-008` stay owed by the owner's decision of 2026-08-20 to leave both for a separate change,
  refined the same day into a scope, a depth and a timing: those two contracts and `REL-MOK-001`, which gates them;
  rewording the four sites that assert the withdrawn empty-dependency property — `VER-MOK-005:149`, `:231`, `:234`,
  `:296` and `VER-MOK-008:169`, the last of which also names a `release.yml` step that `WO-MOK-013` replaced; and a
  separate work order taken up after pull request #33 merges. The route is measured rather than assumed: reassessing the
  two contracts alone takes the dashboard from **7 warnings to 8**, because staleness then propagates to `REL-MOK-001`
  twice and to `WO-MOK-008` once, where reassessing `REL-MOK-001` with them takes it to **6**.
- **The `W-HEX-003` observations that name those two contracts are this change's own doing.** An earlier version of the
  bullet above said `W-HEX-003` "names them here as it did at `ff3a155`". That is false, and the evidence this record
  binds disproves it. `W-HEX-003` reports five observations at `ff3a155` and five here, but the two sets are
  **disjoint** — `WO-MOK-013-completion-summary.md:87-93` tabulates both, and at `ff3a155` they are
  `ADR-MOK-001`→`ARCH-MOK-001`, `ARCH-MOK-001`→`SPEC-MOK-001`/`SPEC-MOK-002` and
  `ARCH-MOK-002`→`SPEC-MOK-003`/`SPEC-MOK-004`, naming neither contract. Ten artifacts moving to 2026-08-20 cleared
  those five because their sources caught up, and created five because their targets moved ahead of unamended sources.
  The equal count was the whole trap. `WO-MOK-013-transition.md:184` compares against `84b21b9`, which is on this branch
  and after the dates moved, so it holds; `WO-MOK-013-review-gate.md:160` claims only that the five are unchanged across
  the three routes it measured, which is what was measured.
- **What is genuinely inherited is inherited.** `VER-MOK-011`'s fifth manual assessment, the four `OUTSTANDING`
  amendment rows from 2026-08-18 and 2026-08-19, and the verification record owed against an earlier merge commit
  predate this branch. None is cleared here and none is claimed as settled.

## The one reading this record does not take

`VER-MOK-013:219` states: *"An unrecorded assessment is an outstanding assessment, and this contract is not satisfied
while any remains outstanding — as `VER-MOK-011`'s fifth still is."* Its own amendment row at `:21` states:
*"**Assessments 4 and 7 are unchanged and 7 is not yet due**"*. Assessment 7 is neither recorded nor outstanding: its
trigger is the first admission of a crate beyond `ratatui`, and this change admits none, so there is nothing for the
technical owner to record and no crate the assessment could be about.

Read one way, five of six recorded and the sixth untriggered leaves nothing outstanding and the contract is satisfied.
Read the other, the contract's sentence admits only *recorded*, and *not yet due* is a third state its wording does not
provide for. Both readings are available in the approved text, and the approved text is the assurance owner's to read.

**This record does not choose.** `WO-MOK-013-manual-assessment.md:353-357` records the same tension, says the file does
not take the reading, and names a verification record against this contract as where it would be taken — which is this
file, at the moment its status changes. Preparing a candidate is not that moment. Whoever transitions this record to
`verified` takes that reading by doing so, and this section is here so that it is taken knowingly rather than by
arithmetic.

The narrower consequence, if the strict reading is preferred: assessment 7 can be closed only by an admission, so a
strict reading makes this contract unsatisfiable until some later work order admits a crate. That is the choice, stated
plainly. It is not the same question as `VER-MOK-011`'s fifth assessment, which is genuinely unperformed and
performable, and neither answer here touches it.

## What must happen before this record can move to `verified`

1. **An accountable assurance owner reviews the retained evidence and decides.** `DECISION_RIGHTS.md` reserves the
   `ready → verified` transition to that role, and record preparation never makes it. The implementation agent wrote
   every measurement in this packet and judged none.
2. **That decision takes the reading above**, explicitly, in one direction.
3. **The transition is its own act, recorded with its instruction quoted.** This packet's precedent is
   `WO-MOK-011`'s: the status change travels in its own commit so that no statement is made false in the commit that
   introduced it.

Not authorized and not performed at this commit: any tag, any release, any assurance decision, and any change to this
record's status. The pull request that carries this branch is a separate act from the transition, and creating one
verifies nothing.

## Scope a `verified` status would have

It would accept the evidence enumerated above as satisfying `VER-MOK-013` at commit
`65ac88b0076dc1044adb4e6e984256b4428892b4`, and nothing beyond it:

- **not the merge**, and not whatever tree results from merging this branch;
- **not a release**, which needs its own commit-bound record against `RLS-MOK-001`;
- **not the outstanding items elsewhere** listed under *What this record does not claim*;
- **not the three snapshot figures this record corrects**, which stay as they were recorded in the commits that
  recorded them, with the corrected figures here;
- **not any future admission of a crate**, each of which owes the seven-item evidence set `VER-MOK-013`'s *What an
  admission must retain* fixes, and owes manual assessments 1, 2 and 7 for that crate by name.

Every command behind every figure in this record is offline, reads no credential, secret, token or environment value,
and none appears in the retained evidence.
