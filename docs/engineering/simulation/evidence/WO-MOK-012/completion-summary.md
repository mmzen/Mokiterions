# Completion summary — WO-MOK-012

Recorded 2026-08-20 on branch `assessment/wo-mok-005-remediation`, based on `origin/master` at `ff3a155`.

**Status of this record.** It states what was done, what was decided, and what remains open. It does not transition any
artifact. `WO-MOK-012` is `draft` and stays `draft` until the engineering owner approves it; being governance-only work
it then stops at `implemented` and takes no verification record.

---

## 1. What was done

### The eleven ratifications

Four approved artifacts carried amendment rows whose Approval cell read **OUTSTANDING**. All eleven provisions were
ratified as written by the repository owner acting as technical owner, and each Approval cell was edited in place to
record the act, the date, the role, and that the row was outstanding until then.

| Artifact | Row | Provisions |
|---|---|---|
| `SPEC-MOK-002` | 2026-08-18 | 4 — rules 1, 3, 5, 6 |
| `ARCH-MOK-001` | 2026-08-18 | 1 — the public-item prohibition |
| `SPEC-MOK-003` | 2026-08-18 | 1 — *Data and interface contracts* clause 2 |
| `SPEC-MOK-004` | 2026-08-19 | 5 — rules 6, 9, 11, 12, 13 |

`amendment-ratifications.md` states each provision, what it changes, and why ratification rather than revision was the
right act.

### Five stale cross-references corrected

Ratifying made five later sentences false — each said one or more of these provisions "remain OUTSTANDING". All five
were moved to the past tense, naming the 2026-08-20 act.

**One of the five was also wrong when written**, and the correction is recorded rather than made silently:
`SPEC-MOK-002`'s 2026-08-19 row said both of its two 2026-08-18 predecessors remained outstanding, when only the first
ever was — the second had been approved the same day by way of `ADR-MOK-004`. The corrected sentence states the
miscount. This is a defect found in an approved record, not introduced by this work order.

### Six assessments authored, one left open

`VER-MOK-005`'s seven manual assessments were all outstanding with no author. Six are now performed and authored by the
repository owner. The seventh is **outstanding by decision**, because the shipped binary has no operator-reachable
panic path and the contract's words "rather than only by an automated assertion" make the retained automated result
insufficient by construction. Full statements in `manual-assessment.md`.

### `VER-MOK-005` amended

Three of the seven named subjects that had moved since approval. Each was restated, each restatement was put and decided
on its own, and a 2026-08-20 amendment row records them. Assessment 3 in particular asked for a colour-disabled run the
binary cannot perform.

### Three adverse observations recorded

From the live pass: the `?` key is undiscoverable; the roster survival gauges are two columns wide; the hidden-pane
notice names the overlay remedy rather than the enlargement remedy and carries no emphasis. Each is recorded in
`adverse-observations.md` with its measurement, and **none is fixed here** — decision 12 directs all three to a
separate chain, with the design settled by decisions 13 to 15.

On the third the agent corrected the owner's framing rather than transcribing it: the announcement obligation does
exist in `SPEC-MOK-003` rule 5, is permanent, and the implementation conforms. Two narrower defects are real and are
what the finding reduces to.

### Three procedure defects recorded

`WO-MOK-005`'s recorded procedure names `a` for the authority overlay when `a` is unbound and the key is `t`, and omits
`--ticks` so its default of 100 cannot reach the required 200. The third is in `VER-MOK-005` itself. All three verified
against `ff3a155`; see `procedure-defects.md`. The original evidence file is not edited, following the
`evidence/WO-MOK-011/merge/` precedent of re-measuring into a new directory.

---

## 2. Required verification

`WO-MOK-012`'s *Required verification* section names four checks. All four were performed on 2026-08-20 at `ff3a155`.

### The diff touches no product source

```
$ git diff --stat origin/master -- mokiterions-core/ mokiterions-tui/ \
      Cargo.toml Cargo.lock rust-toolchain.toml
(empty)
```

Empty. Both packages, both manifests, the lockfile and the toolchain pin are byte-identical to `origin/master`. This is
the check that establishes the two oracles were removed completely: they lived as `#[cfg(test)] mod` declarations under
`mokiterions-tui/src/`, so any residue would appear here.

The complete change surface is seven paths, all under `docs/`:

```
 M docs/engineering/simulation/architecture/ARCH-MOK-001.md
 M docs/engineering/simulation/specifications/SPEC-MOK-002.md
 M docs/engineering/simulation/specifications/SPEC-MOK-003.md
 M docs/engineering/simulation/specifications/SPEC-MOK-004.md
 M docs/engineering/simulation/verification/VER-MOK-005.md
 M docs/ROADMAP.md
 ?? docs/engineering/simulation/evidence/WO-MOK-012/
 ?? docs/engineering/simulation/work-orders/WO-MOK-012.md
```

### Four roadmap sentences corrected

`docs/ROADMAP.md` gains a **Status** paragraph for Phase 1.5, which had none, and three sentences were corrected
because **this work order's own acts made them false**: Phase 1.5 said "every artifact is `draft`"; Phase 2 said the
overridden `WO-MOK-010` gate still owed the whole eleven-provision and seven-assessment debt; Phase 2.5 said the earlier
`OUTSTANDING` rows "are untouched and still outstanding". Each was left stating what was true of the work it describes
and then corrected forward, rather than rewritten as though it had always said this.

### The graph validates

```
$ python scripts/validate_engineering_artifacts.py
Engineering artifact validation: PASS
Artifacts: 103 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
```

Zero errors and zero warnings on all four planes with `WO-MOK-012` in the graph. In particular no **W-HEX-001** (an
implemented work order with no evidence keyed to its identifier), no **W-HEX-003** (an artifact predating a newer
declared target) and no **W-HEX-004** (a dependency cycle).

`harnessctl` is not installed in this environment; `scripts/validate_engineering_artifacts.py` is the graph validator
the repository provides and is what was run.

### Tests, formatter and linter

```
$ cargo test --workspace          # 212 passed, 0 failed, across 21 test binaries
$ cargo fmt --all -- --check      # clean
$ cargo clippy --workspace --all-targets --all-features -- -D warnings   # clean
```

**212** is the figure `SPEC-MOK-004` rule 11 records for the workspace at `ff3a155`, unchanged — as it must be, since no
source or test file is touched.

### One stray directory removed

The oracles wrote their captures to `assessment-material/` at the repository root, and that directory was copied into
`evidence/WO-MOK-012/assessment-material/`. `diff -rq` confirmed the two byte-identical before the root copy was
removed. All twelve files are retained under `evidence/`; nothing was lost.

---

## 3. What remains open

### Left open deliberately

- **Manual assessment 7.** Outstanding by the assurance owner's decision. **`VREC-MOK-005` must continue to disclose
  it**, and any release record covering this chain inherits the disclosure. It is not a gap to be closed quietly later;
  closing it requires either an operator-reachable panic path or a decision to accept the automated result, and neither
  was taken.

### Awaiting the engineering owner

- **Approval of `WO-MOK-012`.** It is `draft`.
- **Confirmation of the `[assurance]` classification.** The agent proposed `commit_bound_verification = "not_required"`
  on the ground that the diff contains no executable behavior, no test, no managed policy and no CI definition, and
  that the work's sole purpose is recording already-authorized governance decisions — which is the condition
  `WORKFLOW.md` states for `not_required`. **The engineering owner confirms or rejects this at approval.** The agent
  holds no authority over it and did not decide it.

### Not decided by the closing review

- **`VREC-MOK-005`'s verification act.** It is already `verified`, bound to commit `f361370`. It was verified on
  2026-08-19 *with* this whole debt disclosed — "with all seven manual assessments outstanding and unauthored, and
  eleven amendment provisions still OUTSTANDING" — and it adds that "verification does not advance the amendments".
  **This work order is the act that record said had not been taken.** It does not revisit the verification, and it does
  not edit the record: what the record certified was correct at its commit, and its disclosure of assessment 7 is still
  correct today.
- **Whether the chain needs a re-captured record.** Not decided. Ten of the eleven provisions and six of the seven
  assessments are no longer in the state the record describes, so a reader who takes `VREC-MOK-005` as current will be
  wrong about them. A re-capture rather than an edit is the route if one is wanted; the tree's own precedent for this is
  `VREC-MOK-006`, which says "`VREC-MOK-005` is still `ready`" and was left standing when that ceased to be true, with
  `VREC-MOK-005` disclosing the staleness rather than `VREC-MOK-006` being edited.
- **`WO-MOK-005`'s status.** `implemented`, and it stays so.

### The second chain

Decisions 13 to 15 settle the design; nothing is built. Three observer changes — the roster gauge relayout to width 13
over three lines per entry, the permanent `?` header hint, and the hidden-pane notice stating the enlargement remedy
with emphasis — together with the `SPEC-MOK-003` rule 4 and rule 5 amendments they require.

**Identifiers for that chain are not claimed here.** `identifier-sweep.md` records what was free across all 24 remote
heads on 2026-08-20, and the refs move — `master` moved from `dec1b95` to `ff3a155` during this work order alone. The
chain re-sweeps rather than trusting that record.

Two further observations in `adverse-observations.md` — log lines truncating mid-value with no marker, and `UNDERLINED`
never occurring in the seed-42 run — are recorded but were not raised as findings and carry no decision.

---

## 4. What the agent did not do

- **Transitioned no artifact status.** Every artifact is in the lifecycle state it was in before, `WO-MOK-012` included.
- **Approved, ratified, assessed and released nothing.** Every decision recorded in this directory is attributed to the
  repository owner in a named role. The agent put the questions with the measured facts assembled, and transcribed the
  answers.
- **Did not push, open a pull request, tag or release.** The work is committed on `assessment/wo-mok-005-remediation`
  and stops there. No such act was authorized in this review.
- **Did not treat one answer as covering another.** Because one person holds every accountable role, nothing here is
  approved by implication of anything else. The eleven ratifications were put as four questions each enumerating its
  provisions individually.
- **Did not silently accept a finding it believed wrong.** Adverse observation 3 was reframed and the reasoning stated;
  assessment 2's scope was split between the overview that passed and the roster that did not, rather than blurred.
