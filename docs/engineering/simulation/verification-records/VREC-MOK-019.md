+++
id = "VREC-MOK-019"
type = "verification_record"
title = "Verification candidate for WO-MOK-019 at the second merge commit"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-21"
updated = "2026-08-21"
commit = "e96648a2a6524a80c761b791378ca91289f02ba2"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-21T17:04:11Z"
artifact_snapshot_sha256 = "c4da6a5e5bd727bc087d117d0101446a526553cbe6c09f2199fe8c131101c4e9"
evidence_paths = [
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/COMMIT.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/README.md",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/capture-social.sh",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/census-by-target-branch-vs-merge.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/census-by-target-master-vs-merge.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/census-reconciliation.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/gates.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/governance.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/interface.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/comparison-a-pre-vs-merge-nosink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/comparison-b-merge-nosink-vs-sink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/comparison-d-candidate-vs-merge-nosink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/comparison-e-candidate-vs-merge-sink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/comparison-f-social-nosink-vs-sink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/comparison-g-master-social-vs-merge-social.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/comparison-h-candidate-vs-merge-records.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/manifest-merge-nosink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/manifest-merge-sink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/manifest-social-nosink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/manifest-social-sink.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/record-kinds.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/suffered-accounting.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/suffered-field.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle1/value-domains.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle2/extended-reference-matrix.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle2/extended-social.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle2/retained-reference-matrix.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle2/retained-social.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle3/drafted-reference-matrix.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle3/drafted-social.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle3/retained-reference-matrix.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle3/retained-social.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle6/extended-reference-matrix.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle6/extended-social.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle6/retained-reference-matrix.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/oracle6/retained-social.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/reconstruct-combat.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/record-field-accounting.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/render-items.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/replay-combat.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/second/README.md",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/second/gates.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/second/governance.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/second/test-census.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/second/test-run.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/social-vs-master.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/test-census-branch.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/test-census-master.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/test-census.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/test-run-branch.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/test-run-master.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/test-run.txt",
  "docs/engineering/simulation/evidence/WO-MOK-019/merge/validate-drafted.py",
  "docs/engineering/simulation/evidence/WO-MOK-019/renumbering.md",
]

[relations]
verifies_work_order = ["WO-MOK-019"]
conforms_to = ["VER-MOK-012"]
+++

# Verification Record Candidate

This record is `ready`. It names commit `e96648a2a6524a80c761b791378ca91289f02ba2` — the second merge of
`origin/master` into this chain — as the candidate satisfying `VER-MOK-012`, and the fifty-four retained files
listed above as the evidence for it. **It decides nothing.** `DECISION_RIGHTS.md` reserves the transition to
`verified` to the accountable assurance owner, and no part of this record's preparation is an acceptance. What the
transition would accept and what it would leave standing is in *What this record does not discharge*; **read that
section before relying on the status.**

**This record is written after `master` merged the work, and that ordering has to be stated rather than hidden.**
Pull request #31 merged at 2026-08-21T16:52:03Z as `fac152ffde369db7f76deaeceb127dc04fb414f7`, whose parents are
`7f4792a` and `00e58a9`. So the merge did not wait on this record and this record cannot have gated it.
`WORKFLOW.md` never makes merging a governed artifact act, and `VREC-MOK-015` — the precedent this record follows
in every other respect — was prepared while its pull request was still open. Ours was not. The consequence is
narrow and it is real: this candidate is offered for a decision about a tree, not for a decision about whether to
integrate it, because that decision has already been taken by the owner on other grounds. Nothing below is
weakened by the ordering, and nothing below should be read as having authorized the integration.

`e96648a` is an ancestor of `origin/master` — `git merge-base --is-ancestor e96648a origin/master` succeeds —
which is what makes it still bindable. Verification is not merge and not release: no release record binds this
work and this status makes no commit release-eligible.

## The front matter is the capture's provenance, not a decision's

Every field above is a measurement, and the record was composed by hand because the harness tool will not produce
the one that is wanted. **`harnessctl capture-verification` was run from both positions and neither yields this
record.** Both outcomes are correct behaviour and both are quoted rather than described.

**From a clean detached worktree at the candidate**, `e96648a`:

```
$ harnessctl capture-verification . --id VREC-MOK-019 --work-order WO-MOK-019 \
      --verification VER-MOK-012 --evidence .../merge/second/gates.txt
harnessctl: evidence file does not exist: docs/engineering/simulation/evidence/WO-MOK-019/merge/second/gates.txt
exit=2
```

It refuses because at `e96648a` six of the fifty-four declared files are not committed yet. They are the record
of that very merge, and a merge cannot carry its own account.

**From the main checkout at `00e58a9`**, clean, with all fifty-four present, it succeeds — and binds the wrong
commit:

```
prepared ready verification record: .../verification-records/VREC-MOK-019.md
commit = "00e58a95f947845bd4e1b31cb38702b09f839c27"
verified_at = "2026-08-21T17:01:27Z"
artifact_snapshot_sha256 = "7baaace60d87c44a56468bb55e87f1d6063028de003369c712d772d01516d569"
owners = ["quality-owner"]
```

That output was taken, read and then replaced by this file. It is not kept as evidence, because a rejected
provenance is not evidence of anything; it is quoted here so the rejection is checkable. `00e58a9` is the commit
whose only content beyond `e96648a` is this record's own evidence — ten files, six added and four modified, every
one of them under `evidence/WO-MOK-019/` and none of them source, artifact or configuration. Binding to it would
name the paperwork rather than the merge. `VREC-MOK-015` faced the identical fork, in the identical order, and
settled it the same way: *"following it here would bind the record to a commit whose only content beyond this
merge is this record's own evidence, rather than to the merge."* The precedent for hand-composed front matter
re-measured at the commit it names is `VREC-MOK-014`'s re-point.

So each field was measured directly, in one detached worktree held at `e96648a` throughout:

| Field | How it was measured |
|---|---|
| `commit` | `git rev-parse HEAD` in that worktree: `e96648a2a6524a80c761b791378ca91289f02ba2`. The commit was pushed before any figure below was taken and is now an ancestor of `origin/master` |
| `worktree_state` | `git status --porcelain` in that worktree: **0 entries**, before and after every measurement. In the main checkout the same command returns this file, which belongs to the commit that will carry it and is not a tracked difference from `e96648a` in anything the contract reads |
| `verified_at` | the **capture** timestamp, as the template names it, and not an approval time. The provenance was read at 2026-08-21T17:04:11Z; every gate below was measured in the same worktree between 17:00Z and 17:06Z. A later transition to `verified` must not move this field, because a decision does not re-measure provenance |
| `artifact_snapshot_sha256` | `python scripts/generate_harness_dashboard.py --root . --output <outside the checkout>` in that worktree, whose **leaf directory name is `Mokiterions-20260821-102633-559`, equal to this repository's checkout**, because `build_snapshot` puts `repository_root.name` into the hashed document at line 1294. PASS, 146 artifacts, 523 relations, 0 errors, 17 warnings, snapshot `c4da6a5e…`, and `sha256sum` of `dashboard-data.json` equal to it |
| `owners` | `["assurance owner"]`, the role `DECISION_RIGHTS.md` names and the string `VREC-MOK-012` and `VREC-MOK-015` carry. The tool emitted `["quality-owner"]`, which is the distribution's default and not this repository's role name |
| `status` | `ready`. The tool cannot emit anything else and neither can record preparation |

**The snapshot digest is reproducible only under three conditions, and they are stated because two of them are not
about the tree.** It covers `repository.name` (the checkout's directory basename), `repository.revision` (`HEAD`),
and the evidence *paths* `discover_evidence` walks — not evidence bytes. A reader who clones into a differently
named directory, or checks out a different commit, or clones at depth 1, will compute a different digest and
nothing will be wrong. It is written down so a later run **at `e96648a`, in a directory of that name, at full
depth** has something to compare against, which is the only claim `merge/gates.txt` and
`WO-MOK-011/merge/gates.txt` make for theirs.

It reproduced exactly. `merge/second/gates.txt` recorded `c4da6a5e…` when the second merge was measured, and the
clean detached worktree recomputed the same sixty-four characters — which also establishes that the earlier figure
was not polluted by the untracked evidence present in the checkout at the time.

## What this record claims

### The candidate commit

| | |
|---|---|
| Commit | `e96648a2a6524a80c761b791378ca91289f02ba2` |
| Subject | `Merge master into wip/pr31-integration for WO-MOK-019` |
| First parent | `efe20e3e9e680eeb94b2defb9d5dcc1556f4c316`, this chain's renumbering of the `018` identifier to `019` |
| Second parent | `7f4792a8e4de9bb0e2ce1a53a35bb9f5c5fb10a3`, `origin/master` |
| Merge base | `fa065cc27aa250bd93c586b0c61da789dab49e33`, the first merge's second parent |
| Worktree | clean at the commit, 0 porcelain entries |
| Contents relative to the first parent | 29 files, +3,885 / −153. Four are source, all the observer's — `mokiterions-tui/src/{render,state}.rs` and `mokiterions-tui/tests/{state,verification}.rs` — and they are `master`'s. 25 are governance documents. **Zero** files under `mokiterions-core/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `.github/` or `scripts/` |
| Ancestry | an ancestor of `origin/master` through `fac152f`, pull request #31 |

**Two empty diffs fix the boundary of what this merge did, and they are the licence for every stream figure the
declared evidence carries forward.** `git diff efe20e3 e96648a -- mokiterions-core` is empty: the engine that
wrote the 303 MB of record stream measured in `merge/oracle1/` through `merge/oracle6/` is byte-identical to the
engine at `efe20e3`. `git diff 7f4792a e96648a -- mokiterions-tui` is empty: the observer is byte-identical to
`master`'s. The claim is not that the engine probably did not change; it is that no byte of it did, on a
comparison that names the trees. `merge/second/gates.txt` states the same boundary and adds that had either diff
been non-empty the 120 capture cells would have had to be re-taken.

### Why `VREC-MOK-012` is not re-pointed to this commit instead

`VREC-MOK-012` is `verified`, at `50364a3`, on the owner's decision of 2026-08-20 recorded in
`evidence/WO-MOK-019/assurance-decision.md`. The template permits supersession of a `ready` record only, and
re-pointing a verified record would move a decision the owner took about one tree onto a different tree. A record
for the merge is a new record — `evidence/WO-MOK-011/merge/README.md` settled that in those words and
`VREC-MOK-015` adopted them. `VREC-MOK-012` and `VREC-MOK-016` are untouched by this record and stay bound to
`50364a3` and `4539601`.

Two `verified` records over one work order is not an anomaly. `VREC-MOK-014` and `VREC-MOK-015` are both verified
over `WO-MOK-014`, the second bound to a merge of `master` into the branch, which is this record's exact shape.
Records are counted against commits that satisfy the contract, not against work orders.

### The gates, measured at this commit

All ten in the one detached worktree, `git status --porcelain` empty throughout. The first four are the static
checks `VER-MOK-012` names in *Static and architecture checks*; the rest are the harness's.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, clean |
| `cargo test --workspace` | exit 0 — **22 suites, 301 passed, 0 failed, 0 ignored**, one invocation at the workspace root with no feature or environment selection |
| `cargo tree -p Mokiterions` | exit 0, and the output is **one line** — `Mokiterions v0.1.0 (…/mokiterions-core)`. The engine's dependency and dev-dependency tables are empty, with no exception |
| `python -m se_harness validate .` | PASS — 146 artifacts, 0 errors, 0 warnings, all four planes E0/W0 |
| `python -m se_harness inspect .` | 146 artifacts, 523 relations, 35 findings — **0 error**, 17 warning, 18 info |
| `python -m se_harness doctor .` | 81 lines, **every one PASS**, every managed and distribution file unchanged |
| `python -m se_harness preflight . --work-order WO-MOK-019 --phase review` | PASS — `Work order: WO-MOK-019 (implemented)`, commit-bound verification **required**, decided by engineering owner |
| `python scripts/generate_harness_dashboard.py` | PASS — 146 / 523 / 0 errors / 17 warnings, snapshot `c4da6a5e…` |
| `git status --porcelain` | 0 entries |

The harness was se-harness **0.4.0**, from `C:\Users\mathi\harness-venv-040`, which is the version this
repository's workflows pin. The machine-wide 0.4.1 FAILs on template skew alone and is not a governing verdict
here. Toolchain: cargo 1.97.1 (c980f4866 2026-06-30), rustc 1.97.1 (8bab26f4f 2026-07-14).

### The governance graph at this commit

    tree                        artifacts   relations
    merge base   fa065cc              131         473
    master       7f4792a              134         485
    branch       efe20e3              143         511
    candidate    e96648a              146         523

    134 + 143 - 131 = 146          485 + 511 - 473 = 523

Both sums are exact, and that is the strongest single statement available about this merge as a governance
operation. Nothing was lost — one dropped artifact would read 145. Nothing was duplicated. Nothing arrived from
outside either parent. And the relation count reconciling too means no declared edge was silently repointed,
which the artifact count alone would not catch.

**145 is also what the sum would read had the renumbering not been applied**, because `WO-MOK-018` would have
been one identifier claimed by two work orders and the union would hold one fewer. So the arithmetic is not only
a loss check; it is the renumbering of `efe20e3` measured. The same result appears in the findings: `W-HEX-001`
reports seven observations at this commit and both `WO-MOK-018` and `WO-MOK-019` are among them —

    WO-MOK-010, WO-MOK-011, WO-MOK-012, WO-MOK-013, WO-MOK-016, WO-MOK-018, WO-MOK-019

— which is master's six and this chain's one, coexisting. `merge/second/governance.txt` is the fuller account.
`W-HEX-003` reports ten and `I-REV-001` eighteen; all seventeen warnings are pre-existing packet-shape or
staleness signals, and the zero in the error column is the figure that matters for a candidate.

### One finding this record's own existence opens, and it asks whether the record is wanted

The figures above are the candidate's. On the tree that will *carry* this file the graph reads **147 artifacts,
525 relations, 37 findings — 0 error, 18 warning, 19 info**, and `validate` is still PASS with 0 errors and 0
warnings. The arrivals are one artifact, its two declared relations, one more `I-REV-001` observation, and one
new warning that is about this record rather than about the work:

    W-REV-004  VREC-MOK-019 is ready but its work is fully covered by verified or released
               records; review possible supersession without inferring authority.
               artifacts: VREC-MOK-012, VREC-MOK-019    evidence: possible_successors=VREC-MOK-012

It is quoted rather than dismissed, because it is not noise and it does not say what a reader might expect. It
does not ask whether `VREC-MOK-012` should be superseded by this record. **It asks whether this record is
redundant** — whether `WO-MOK-019` is already fully covered by a `verified` record, and therefore whether this
candidate should be superseded by `VREC-MOK-012` rather than transitioned. That is a fair question and it is the
assurance owner's, so the case each way is stated and neither is inferred.

**Why a second record is nonetheless the right shape.** `VREC-MOK-012` binds `50364a3`, a commit that predates
both merges. Every figure in it was measured on a tree with no `attack_resolved`, no `threat_resolved`, no
`surrender_resolved`, no `suffered` field and none of master's governance. It cannot speak to the merged tree,
and a record's provenance cannot be re-pointed to make it do so. The precedent is exact rather than analogous:
`VREC-MOK-014` and `VREC-MOK-015` are both `verified` over `WO-MOK-014`, the second bound to a merge of `master`
into the branch, which is this record's shape to the letter. The rule fired on neither of them only because both
were already `verified` — `W-REV-004` tests a `ready` record — so this warning is a property of this record's
status and not of its content, and it goes quiet whichever way the owner resolves it.

**Why the other reading is available.** If the owner judges that `VREC-MOK-012`'s decision of 2026-08-20 was
about the work order rather than about a tree, then the merges changed no product behaviour of the engine — the
two empty diffs above say so — and a second decision buys nothing. Superseding this candidate with
`VREC-MOK-012` would then be the correct disposal, and the evidence declared here would stay retained and
undeclared, as `merge/README.md` already records it being. This record does not take that decision and does not
lean on the reader to take the other one.

## The declared evidence, and what is deliberately not declared

The packet holds **110** tracked files. `VREC-MOK-012` declares **55** of them — the pre-merge capture, taken at
`50364a3`. This record declares **54** more. One file is left undeclared on purpose, so the arithmetic closes at
`55 + 54 + 1 = 110` rather than appearing to leave a remainder.

The undeclared file is `evidence/WO-MOK-019/assurance-decision.md`. It records the owner's decision of
2026-08-20 on `VREC-MOK-012`; it postdates the commit that record binds, and a record's evidence set is the
capture's rather than the decision's. Its own text states that exclusion and cites the `VREC-MOK-007`,
`VREC-MOK-010` and `VREC-MOK-011` precedents. `VREC-MOK-015` kept its own `assurance-decision.md` out of
`evidence_paths` for the same reason, and if this record is transitioned to `verified` the note recording that
decision must likewise stay out.

**Six of the fifty-four do not exist at `e96648a`, and four more differ from their bytes there.** This is the
same fact as the tool's first refusal and it is spelled out rather than left in an error message:

| | paths | why |
|---|---|---|
| absent at `e96648a` | `merge/second/{README.md, gates.txt, governance.txt, test-census.txt, test-run.txt}`, `renumbering.md` | they are the account of this merge and of the renumbering that preceded it, so they were written after it |
| modified after `e96648a` | `merge/{README.md, gates.txt, governance.txt}`, `merge/oracle1/suffered-accounting.txt` | the first merge's packet, corrected for the renumbering and repointed at this commit |
| byte-identical at `e96648a` and `00e58a9` | the remaining 44 | |

All fifty-four exist and are readable at `00e58a9` and therefore on `master` at `fac152f`, whose tree is
byte-identical to `00e58a9` — `git diff 00e58a9 fac152f` is empty. A reader resolving these paths should resolve
them there. The commit this record *binds* is the one whose properties it asserts; it is not a claim that the
account of those properties was written before them.

## What this record does not discharge

Nine items. The first five are owed regardless of the status; the last four are limits on what the status could
mean.

1. **`VER-MOK-012`'s per-kind oracle covers twelve event kinds and the candidate emits fifteen.**
   `merge/oracle1/record-kinds.txt` measures exactly twelve under `baseline`, `reference` and `individual` and
   exactly fifteen under `social`, with `attack_resolved`, `threat_resolved` and `surrender_resolved` in all
   thirty social cells. `SPEC-MOK-006` was written against twelve and specifies a record shape for each; three
   record shapes, five `result.detail` values, a closed `target_died` domain and fourteen field rows are missing
   from it. The engine's own `every_event_kind_has_its_exact_record_shape` passes because it reads
   `EventType::ALL`; the specification is what does not. **This is the largest thing this record leaves open.**

2. **The `SPEC-MOK-006` amendment is drafted and its approval cell reads `OUTSTANDING`.** It is derived from the
   streams measured in `merge/oracle1/` rather than from reading master's source, which is why it is offered as
   evidence-backed rather than as a reading. Ratifying it is the artifact owner's act.

3. **The `SPEC-MOK-004` amendment row for this merge also reads `OUTSTANDING`.** `SPEC-MOK-004` is the one file
   that conflicted in this merge — 144 insertions, 75 deletions — and its five conflict regions were resolved on
   the principle that where approved text and an unratified draft answer the same question the approved text
   stands and the draft is withdrawn. `merge/second/README.md` states that principle so it can be disagreed with,
   and discloses a defect in rule 10's figures found while resolving it.

4. **`ARCH-MOK-002` must be reassessed against `SPEC-MOK-004` as amended.** `W-HEX-003` names it at this commit.
   The obligation is owed to that artifact's owner whether or not the warning had appeared, because what changed
   in `SPEC-MOK-004` is substantive rather than a date.

5. **The method question on the retained captures is unanswered.** The renumbering swept `WO-MOK-018` into
   `WO-MOK-019` across 132 occurrences in 45 retained tool captures. The `013 → 014` precedent refused the
   equivalent sweep, reasoning that *"a capture that has been improved is no longer a capture"*. This packet did
   the opposite, twice, and `renumbering.md` discloses it, states the reading rule for the verbatim bytes
   (`git show fa0bfd9^:<path>` and `git show efe20e3^:<path>`) and puts the question. `VREC-MOK-012`'s `title`
   field, which now reads *"Verification candidate for WO-MOK-019"*, rides on the same answer. Until it is
   answered, 45 declared evidence files carry an identifier the tools that produced them never printed.

6. **`VER-MOK-012`'s eight manual assessments are recorded, and assessment 1's answer predates three event
   kinds.** All eight were recorded on 2026-08-20 by the repository owner and are in
   `evidence/WO-MOK-019/manual-assessment.md`, which `VREC-MOK-012` declares. The contract states that an
   unrecorded assessment is an outstanding assessment; none is unrecorded. But assessment 1 asks the product
   owner whether the record schema is sufficient for Phase 4b and to *"name any fact found missing"*, and the
   recorded answer — **"Sufficient. No fact is named missing"** — was given against the twelve-kind stream at
   `50364a3`. The candidate emits fifteen kinds and a `suffered` field on all 284,409 `action_trace` records.
   `merge/oracle1/suffered-accounting.txt` names precisely what is missing from the specification. So the
   assessment clause is satisfied as written and assessment 1 is stale in substance. This record does not
   re-record it and cannot: it is a judgement held by the product owner.

7. **Nothing here verifies that the records are useful**, and the limits `VER-MOK-012` records as residual
   uncertainty are inherited unchanged — the alphabet argument being only as complete as the enumeration,
   non-perturbation being verified over the declared matrix, the replay consumer being a second implementation of
   part of the engine, the metrics record's capacity and depletion fields having the weakest independent witness,
   and `fear`'s maximum being a well-formed figure about an attribute with no consumer.

8. **The eight `detail` words master's targeted validation can produce are exercised by neither capture**, and
   the drafted amendment enumerates them from the engine's source and unit tests rather than from a stream. One
   of them, `target_missing`, sits on arms guarded by `debug_assert!` and is unreachable while the invariant
   holds. `merge/oracle1/suffered-accounting.txt` states this as a limit rather than a result, and this record
   repeats it rather than letting the amendment's completeness stand on the stream.

9. **The green checks on pull request #31 did not examine this work order.** The `candidate` job's `Review
   preflight` step took its work order from the pull-request body's `Harness-Work-Order` trailer, which still
   read `WO-MOK-012`, and reported `Work order: WO-MOK-012 (implemented)` with commit-bound verification
   **`not_required`** — master's unrelated work order and the inverse of this one's classification, which is
   `required`. It is the only step in either workflow run that names a work order: the `push` run has no
   preflight step and the `governor` job proves bootstrap identity only. The trailer is read from the stored
   event payload, so it could not have been corrected without a further push. The consequence is durable in two
   places — the merge commit `fac152f` carries the pull request's title, *"(WO-MOK-012 implemented, VREC-MOK-012
   verified)"*, permanently, and no CI run has ever evaluated review preflight against `WO-MOK-019`. **The
   evidence that it passes is in this record's gates table, measured locally, and nowhere else.**

## Authority

Every command quoted here is derived, read-only evidence and each prints its own boundary. Validation does not
approve. Inspection does not validate by exit status, approve, authorize, verify, release or remediate. Preflight
does not approve artifacts, authorize a diff, verify work, release software, commit, push, tag, publish or deploy.
Two empty diffs and two exact sums are measurements, not judgements.

This record's preparation took no decision and created no commit, tag or publication. Transitioning it from
`ready` to `verified` is the accountable assurance owner's act on the evidence declared above, and if it is taken,
`status` is the only front-matter field that may move: `commit`, `worktree_state`, `verified_at`,
`artifact_snapshot_sha256`, all fifty-four `evidence_paths`, both declared relations and the `title` — which will
still read *candidate*, because the record was captured as one — are the capture's and not the decision's.
