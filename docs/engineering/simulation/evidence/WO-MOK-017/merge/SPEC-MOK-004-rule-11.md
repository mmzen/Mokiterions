# `SPEC-MOK-004` rule 11's test totals, measured for `WO-MOK-017` and **not applied**

| Field | Value |
|---|---|
| What this is | The correction rule 11 obliges this work order to make, drafted in full and left unapplied |
| Status | **NOT APPLIED.** `SPEC-MOK-004` is byte-unchanged by this branch, at the candidate and at the merge alike |
| Rule | `SPEC-MOK-004` rule 11, *One invocation*: "a work order that adds a test corrects these figures here, and one that loses a test has a defect" |
| Whose act | The repository owner acting as **technical owner**. The 2026-08-21 rows of that document's amendment record set the procedure: the agent drafts each correction and the owner ratifies each |
| Measured at | `ae2e44f2382fe89f2daf78a8e8aca37febb8bd0f`, the merge commit, and at `26ae6ba648be4eecf6234da15c0beb763b403a0a`, the candidate |
| Evidence | `census-conservation.txt` and `test-run.txt` in this directory; `post/test-run.txt` and `post/test-census-reconciliation.md` for the candidate |
| Date | 2026-08-22 |

## The obligation, and that this work order is inside it

Rule 11 exists for conservation across a move, and its own text says what it is not: "The clause this
paragraph exists for is conservation across a move, not a ceiling on the corpus: a work order that adds
a test corrects these figures here, and one that loses a test has a defect." Eight paragraphs of that
rule are corrections made under that sentence, by `WO-MOK-005`, `WO-MOK-007`, `WO-MOK-010`,
`WO-MOK-011`, `WO-MOK-013`, `WO-MOK-016`, `WO-MOK-018` and `WO-MOK-019`.

**`WO-MOK-017` adds a test and did not correct the figures.** It adds exactly one,
`simulation::tests::the_corrected_non_waste_condition_admits_the_specified_boundaries`, in
`mokiterions-core`'s internal tier, and `SPEC-MOK-004.md` is byte-unchanged on this branch:
`git diff 7f4792a HEAD -- docs/engineering/simulation/specifications/SPEC-MOK-004.md` is empty. The
omission is this work order's own and it is older than the merge — the figure was already one low at
the candidate commit, before `master` was merged at all.

**It is disclosed here rather than repaired here** because rule 11's figures sit in an approved
specification, and this work order's approved amendment set is three amendments in `SPEC-MOK-001`,
enumerated in that document's 2026-08-21 row. A fourth amendment, in a second specification, is not
inside what the owner approved on 2026-08-21, and an implementation agent adding one on its own reading
of an obligation is the failure mode the amendment record exists to prevent. So the correction is
measured, drafted to the letter, and left for one ratifying act.

## What is stale, and on which tree

| Tree | Rule 11's stated workspace total | Measured by `cargo test --locked --workspace` | Short by |
|---|---|---|---|
| `7f4792a`, the branch point | **267** | 267 | — |
| `26ae6ba`, the candidate | **267** | **268** | 1 |
| `3f47743`, `master` now | **301** | 301, per `WO-MOK-019`'s `merge/second/` | — |
| `ae2e44f`, the merge | **301** | **302** | 1 |

The two rows that are short are short by the same one test. Neither figure is wrong about a tree it was
measured on; both are stale about the tree this branch offers.

**The 301 that stands on `master` is itself an unratified draft**, in the amendment-record row marked
`OUTSTANDING` and dated 2026-08-21 for the second merge of `master` into the `WO-MOK-019` chain. This
correction is drafted after it and against it, and it does not restate, reword or reopen it: if the
owner declines that row, this one still reads correctly, because it states its own tree's figure and
cites the earlier one as the predecessor it moves from.

## The figures, reproducible from the four tiers

Measured on the merged tree at `ae2e44f`, from `test-run.txt` in this directory, attributed to a tier by
the target that runs it:

| Tier | Rule | `master` at 301 | This merge | Moved by |
|---|---|---|---|---|
| Observer, internal | rule 10 | 42 | **42** | — |
| Observer, public | rule 9 | 103 | **103** | — |
| Engine, internal | `SPEC-MOK-002` rule 7 | 96 | **97** | +1 |
| Engine, public | `SPEC-MOK-002` rule 8 | 60 | **60** | — |
| Observer total | rule 11 | 145 | **145** | — |
| Engine total | rule 11 | 156 | **157** | +1 |
| **Workspace total** | rule 11 | **301** | **302** | **+1** |

42 + 103 + 97 + 60 = 302 reaches the total from the four tiers, and 145 + 157 = 302 reaches it from the
two package totals. `census-conservation.txt` reaches it from both predecessors: **268 + 34**, this
branch's candidate plus the thirty-four engine tests that arrive with `master` — the same thirty-four
that rule 11's 2026-08-20 paragraph names, unchanged in number and in name — and **301 + 1**,
`master`'s tree plus this work order's own test. No test is lost in either direction, no target is
added or removed, and every one of the 302 reports `ok`.

The observer's figures do not move, so **rules 9 and 10 need no correction and neither table is
touched**. The engine's split is stated under `SPEC-MOK-002` rules 7 and 8 rather than under this rule,
which is the convention every earlier paragraph of rule 11 follows, and it is recorded here because 302
is otherwise not reproducible.

## The drafted paragraph

To be appended to rule 11 after the paragraph beginning "Rule 6's interface is **94** items", which is
the last of the second-merge paragraphs, so that the rule's paragraphs stay in the order the trees
happened:

> As corrected for `WO-MOK-017` and measured on the merge of `master` at `3f47743` into that work
> order's chain, at `ae2e44f`, the observer's total is **145** unchanged and the engine's is **157**, so
> the workspace's is **302**. The one arrival is `mokiterions-core`'s
> `simulation::tests::the_corrected_non_waste_condition_admits_the_specified_boundaries`, an
> internal-tier engine test asserting the three boundary satieties the corrected non-waste condition of
> `SPEC-MOK-001` rule 5 admits — `87`, `79` and `75` — and the decline one point above each, in both of
> that rule's screened cases. Nothing departs and nothing is renamed. The engine's split is 97 internal and 60 public, under
> `SPEC-MOK-002` rules 7 and 8 rather than this rule; it is recorded here for the reason the earlier
> paragraphs record theirs, which is that 302 is otherwise not reproducible. The observer's **145** is
> rule 10's internal **42** and rule 9's public **103**, both unmoved: `WO-MOK-017` amends two observer
> tests and adds none, and `post/test-census-reconciliation.md` reconciles them under their own names.
> **This figure is established by a name-by-name census in both directions and not by a structural
> argument**, because neither half of this merge is byte-identical to either parent — `master` rewrote
> `mokiterions-core/src/simulation.rs` for the record stream while this chain corrected the condition
> inside it, which is the case the two empty diffs of the paragraph above were available for and this
> one is not. `docs/engineering/simulation/evidence/WO-MOK-017/merge/census-conservation.txt` is the
> reconciliation, and it reaches 302 from 268 + 34 and from 301 + 1 alike. **`WO-MOK-017` left this
> rule uncorrected at 267 for the duration of its own candidate**, which its merge packet discloses as
> a finding against itself rather than as a consequence of the merge.

And the amendment-record row, to be appended after the 2026-08-21 row for the second merge:

> | 2026-08-22 | **Rule 11's test totals corrected for `WO-MOK-017`, measured on the merge of `master` at `3f47743` into that chain.** The observer's total is unchanged at **145**, the engine's rises by one to **157**, and the workspace's is **302**. One internal-tier engine test arrives, `simulation::tests::the_corrected_non_waste_condition_admits_the_specified_boundaries`, and none departs; the engine's split becomes 97 internal and 60 public under `SPEC-MOK-002` rules 7 and 8. **No provision, table, target, tier boundary, prohibition or figure of rules 9 and 10 is touched**, because the observer's figures do not move. The row also records what the paragraph records: that `WO-MOK-017` left this rule uncorrected at 267 through its own candidate commit `26ae6ba`, so the correction is late as well as owed, and that the 301 it moves from is itself an `OUTSTANDING` draft of 2026-08-21 which this row does not restate or reopen. | **OUTSTANDING.** Drafted by the implementation agent for ratification by the repository owner acting as technical owner, under the correction procedure the owner set on 2026-08-21: the agent drafts each correction and the owner ratifies each. **Every figure in this row is a measured outcome rather than a decision** — `docs/engineering/simulation/evidence/WO-MOK-017/merge/census-conservation.txt` and `.../merge/test-run.txt` are the measurements, and `.../merge/SPEC-MOK-004-rule-11.md` is the draft in full with the four-tier reproduction. The owner has not been shown this text at the time it was written, and the work order it corrects is `in_progress`. |

## What the correction must not touch

* **No provision.** Rule 11's obligations — one invocation, both tiers, no terminal, no network — are
  not restated, reworded or qualified by a figure correction.
* **No table of rules 9 or 10.** The observer's tiers are unmoved and their tables are `master`'s
  approved figures.
* **No earlier paragraph.** Rule 11's corrections accumulate; a superseded figure stays on the record
  with the tree it was measured on, which is why the draft appends rather than edits.
* **No approved row of the amendment record**, and no row bound by a `verified` verification record.
* **Rule 6's interface figures** — 94 items, 119 `pub` lines, 25 public fields — are not restated here.
  This work order adds no public item and no public field, and `post/gates.txt` records the four
  manifests and `Cargo.lock` byte-unchanged; the figures are `master`'s approved ones and stand.

## What is owed, in one line each

1. **Ratify or decline the paragraph and the row above**, as technical owner. Declining is a coherent
   answer: it leaves rule 11 reading 301 against a tree that runs 302, and it leaves this work order in
   breach of the sentence quoted at the top of this file, which is a state the record can hold as long
   as it says so.
2. **If ratified, the figures land on a commit that is not yet this branch's tip**, so the applying
   commit is itself a change to the tree a verification record would bind. `VREC` preparation is the
   assurance owner's act and this file takes no view on the order.
