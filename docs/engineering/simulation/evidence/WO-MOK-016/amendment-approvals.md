# Amendment approvals: `VER-MOK-016` oracle 7, and the `VREC-MOK-005` gate

| Field | Value |
|---|---|
| Contract | `VER-MOK-016`, oracle 7 — "the `SPEC-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-003` amendments and the `REQ-MOK-005`, `REQ-MOK-014` and `REQ-MOK-034` amendments `WO-MOK-016` names are approved. Absence fails this contract regardless of code state" |
| Second row | "The `VREC-MOK-005` gate — the row `VREC-MOK-010` records as outstanding is a precondition of this work, not an output of it, and its state is recorded here" |
| Reader | `analysis/amendments.py`, over the artifacts and the git history |
| Invocation | `python docs/engineering/simulation/evidence/WO-MOK-016/analysis/amendments.py > docs/engineering/simulation/evidence/WO-MOK-016/amendment-approvals.md` |
| Baseline | `39662d13` — this branch before the change, and before `master` moved |
| Master | `d8e20794` — `master`'s tip, merged into this branch at `259859df` |
| Date | 2026-08-21 |

This file is generated. What it checks, and why each check is not a formality, is in the script's header; the short version is that every provision is looked for **twice, in disjoint text** — in the amendment row that must state it, and in the artifact's body with every amendment row removed — because a record that claimed an amendment the text does not carry would otherwise satisfy both halves with one sentence.

It **signs nothing**. Where a decision is owed it is named as owed. Oracle 7 is a check on acts already taken, and this file's authority is that of a measurement.

---

## 1. The chain's governance state

| Artifact | status | expected | updated | ok | note |
|---|---|---|---|---|---|
| `INT-MOK-010` | `approved` | `approved` | 2026-08-20 | yes | this chain's intent |
| `CAP-MOK-010` | `approved` | `approved` | 2026-08-20 | yes | the capability every amendment below cites |
| `REQ-MOK-051` | `approved` | `approved` | 2026-08-20 | yes |  |
| `REQ-MOK-052` | `approved` | `approved` | 2026-08-20 | yes |  |
| `REQ-MOK-053` | `approved` | `approved` | 2026-08-20 | yes |  |
| `REQ-MOK-054` | `approved` | `approved` | 2026-08-20 | yes |  |
| `REQ-MOK-055` | `approved` | `approved` | 2026-08-20 | yes |  |
| `REQ-MOK-056` | `approved` | `approved` | 2026-08-20 | yes | amended in this chain, twice |
| `REQ-MOK-057` | `approved` | `approved` | 2026-08-20 | yes | amended in this chain |
| `REQ-MOK-058` | `approved` | `approved` | 2026-08-20 | yes | amended in this chain |
| `REQ-MOK-059` | `approved` | `approved` | 2026-08-20 | yes |  |
| `REQ-MOK-060` | `approved` | `approved` | 2026-08-20 | yes | descoped 2026-08-21 to `WO-MOK-017`, unamended |
| `VER-MOK-016` | `approved` | `approved` | 2026-08-20 | yes | amended in this chain, three rows |
| `WO-MOK-016` | `implemented` | `implemented` | 2026-08-21 | yes | this work order |
| `WO-MOK-017` | `draft` | `draft` | 2026-08-21 | yes | carries `REQ-MOK-060` |
| `SPEC-MOK-001` | `approved` | `approved` | 2026-08-20 | yes | amended, three rows |
| `SPEC-MOK-002` | `approved` | `approved` | 2026-08-20 | yes | amended, two rows |
| `SPEC-MOK-003` | `approved` | `approved` | 2026-08-20 | yes | amended, one row and one reconciliation |
| `SPEC-MOK-004` | `approved` | `approved` | 2026-08-20 | yes | not amended; rule 11 figures owed, section 5 |
| `SPEC-MOK-005` | `approved` | `approved` | 2026-08-20 | yes | not amended |
| `REQ-MOK-005` | `approved` | `approved` | 2026-08-20 | yes | amended, one row |
| `REQ-MOK-014` | `approved` | `approved` | 2026-08-17 | yes | not amended; the floor did not move, section 5 |
| `REQ-MOK-034` | `approved` | `approved` | 2026-08-20 | yes | amended, one row |
| `ARCH-MOK-001` | `approved` | `approved` | 2026-08-20 | yes | untouched |
| `ARCH-MOK-002` | `approved` | `approved` | 2026-08-20 | yes | untouched |
| `VREC-MOK-005` | `verified` | `verified` | 2026-08-19 | yes | the gate, section 8 |

`WO-MOK-016` is `implemented` and not `complete`: a work order is closed by a verification record that binds a commit, and that record is written after the commit it names. `WO-MOK-017` is `draft` because `REQ-MOK-060` was carried into it on 2026-08-21 and a work order leaves `draft` only on the owner act its own scope requires.

### The `specifies` relation, which is what made this chain approvable

| Specification | requirements it must specify | present | gained since `master` |
|---|---|---|---|
| `SPEC-MOK-001` | 10 | 10 of 10 | 10: `REQ-MOK-051`, `REQ-MOK-052`, `REQ-MOK-053`, `REQ-MOK-054`, `REQ-MOK-055`, `REQ-MOK-056`, `REQ-MOK-057`, `REQ-MOK-058`, `REQ-MOK-059`, `REQ-MOK-060` |
| `SPEC-MOK-002` | 6 | 6 of 6 | 6: `REQ-MOK-052`, `REQ-MOK-053`, `REQ-MOK-054`, `REQ-MOK-055`, `REQ-MOK-056`, `REQ-MOK-057` |
| `SPEC-MOK-003` | 5 | 5 of 5 | 5: `REQ-MOK-052`, `REQ-MOK-053`, `REQ-MOK-055`, `REQ-MOK-056`, `REQ-MOK-057` |

Without those entries `validate` raises `E007` on every requirement in this chain and `preflight --phase start` raises `W016`, which is why the amendment act had to be single: the relation is what made ten requirements approvable at all, so it could not follow their approval.

## 2. The amendment record against the approved list

The 21 subjects `WO-MOK-016`'s *Required amendments* section names across five artifacts, each checked in the row that must state it and in the body with every row removed. **The row is located by its own subject, not by its date**: eleven rows in `SPEC-MOK-003` share 2026-08-20, and a provision recorded in the wrong row is a defect this table reports rather than absorbs.

### `SPEC-MOK-001` — the `CAP-MOK-010` amendment: thirteen provisions, of which seven are appended rules

- rows in this artifact dated 2026-08-20: **3**, of which this one is located by its subject
- approval recorded in the row: yes — "Approved 2026-08-20 by the repository owner acting as technical owner"
- records that the implementation agent wrote the text and decided no substance: yes
- the row states its own count: "Thirteen provisions amended, of which seven are appended rules"
- subjects checked below: **14**, which cuts some of that count's sentences finer and its appended-rules clause coarser; it is a decomposition of the same thirteen provisions and not a different number of them

| Provision | in the row | in the text |
|---|---|---|
| 1. *Scope* names the capability and drops the interaction exclusion | yes | 2/2 phrases |
| 2. *Actors* names four decision sources | yes | 2/2 phrases |
| 3. *Inputs* and *Help output*: `--policy` takes a fourth value, the default stays | yes | 3/3 phrases |
| 4. *State model / Mokiterion*: no attribute, one item of transient state | yes | 3/3 phrases |
| 5. A *Contact* subsection fixes the relation and its radius | yes | 5/5 phrases |
| 6. *Data and interface contracts*: the eleven kinds, three event types, the trace shape | yes | 8/8 phrases |
| 7. Rule 3 carries `fear` and the record; its valid-proposal list is untouched | yes | 3/3 phrases |
| 8. *Name*'s justification is replaced, and the name stays off the observation | yes | 3/3 phrases |
| 9. Rule 5's accumulation and rule 19's tolerance state the ceiling as an obligation | yes | 3/3 phrases |
| 10. Rule 6 extends validation to targeted proposals | yes | 4/4 phrases |
| 11. Rule 7 fixes the trace-before-clearing order | yes | 3/3 phrases |
| 12. Rule 12's closing sentence is inverted and the composition stated | yes | 3/3 phrases |
| 13. Rule 13 states that combat death uses the existing path, event and finality | yes | 3/3 phrases |
| 14. Rules 20 to 26 appended after rule 19, each with its position in tick order | yes | 19/19 phrases |

### `SPEC-MOK-002` — the `CAP-MOK-010` amendment: rule 5's enumeration, rule 6 re-checked

- rows in this artifact dated 2026-08-20: **3**, of which this one is located by its subject
- approval recorded in the row: yes — "Approved 2026-08-20 by the repository owner acting as technical owner"
- records that the implementation agent wrote the text and decided no substance: yes

| Provision | in the row | in the text |
|---|---|---|
| 1. Rule 5's enumeration gains the variants, and `EventType::ALL` moves from 12 to 15 | yes | 5/5 phrases |
| 2. Rule 6 re-checked and recorded as not amended, cross-agent mutation being new | yes | 3/3 phrases |

### `SPEC-MOK-003` — the `CAP-MOK-010` amendment: three provisions

- rows in this artifact dated 2026-08-20: **11**, of which this one is located by its subject
- approval recorded in the row: yes — "Approved 2026-08-20 by the repository owner acting as technical owner"
- records that the implementation agent wrote the text and decided no substance: yes

| Provision | in the row | in the text |
|---|---|---|
| 1. Rule 11's authority table gains three rows, and `REQ-MOK-052` takes none | yes | 4/4 phrases |
| 2. Rule 4's roster and rule 10's inspector present a targeted action's subject | yes | 3/3 phrases |
| 3. Clause 5's refusal of inert values is satisfied a second way for `fear` | yes | 3/3 phrases |

### `REQ-MOK-005` — the core-set re-reading

- rows in this artifact dated 2026-08-20: **1**, of which this one is located by its subject
- approval recorded in the row: yes — "Approved 2026-08-20 by the repository owner acting as product owner"
- records that the implementation agent wrote the text and decided no substance: yes

| Provision | in the row | in the text |
|---|---|---|
| 1. The four verbs are the core set, beside which `REQ-MOK-052` places seven | yes | 3/3 phrases |

### `REQ-MOK-034` — the frozen-outcome narrowing

- rows in this artifact dated 2026-08-20: **1**, of which this one is located by its subject
- approval recorded in the row: yes — "Approved 2026-08-20 by the repository owner acting as product owner"
- records that the implementation agent wrote the text and decided no substance: yes

| Provision | in the row | in the text |
|---|---|---|
| 1. The frozen clause is narrowed to `baseline` alone | yes | 3/3 phrases |

## 3. The three rows taken after the single act

Two omissions were found **after** the single act above was taken, and one measurement refuted an ordering that act had approved. None could be folded into an act that had already happened, so each carries a row of its own, each stands on a ground its own approval cell names, and one says "in a **separate act**" in as many words. `WO-MOK-016` states all three in its *Required amendments* section, corrected in place rather than silently. The rule 26 row carries six provisions of that rule; the other two carry one each.

### `SPEC-MOK-001` — rule 21 and rule 6: the co-located fallback is `avoid`'s **and** `retreat`'s

- the approval cell carries: "Approved 2026-08-20 by the repository owner acting as technical owner"
- the approval cell carries: "on the discrepancy being put to them with the alternatives"
- the approval cell carries: "Both were declined"
- the approval cell carries: "The implementation agent found the discrepancy"
- the approval cell carries: "did not decide it"

| Provision | in the row | in the text |
|---|---|---|
| Both verbs are named where one was, and consequence 4 is corrected with them | yes | 3/3 phrases |

### `SPEC-MOK-001` — rule 26: a branch hoisted, the engagement gate at `95`

- the approval cell carries: "Approved 2026-08-20 by the repository owner acting as product owner and technical owner"
- the approval cell carries: "on the measured evidence in `evidence/WO-MOK-016/escalation.md`"
- the approval cell carries: "Seventeen variants were measured across three levers"
- the approval cell carries: "leaves rule 12 as Phase 2 approved it"

| Provision | in the row | in the text |
|---|---|---|
| Rule 19's case 3 becomes branch 3 and the gate moves from `30` to `95` | yes | 6/6 phrases |

### `SPEC-MOK-002` — rule 5: a field appended to a public variant that already existed

- the approval cell carries: "Approved 2026-08-20 by the repository owner acting as technical owner, in a **separate act** from the amendment above"
- the approval cell carries: "the omission having been found after that act was taken"
- the approval cell carries: "it did not decide the substance"
- the approval cell carries: "It is stated in full in `WO-MOK-016`'s *Required amendments* section as provision 3"

| Provision | in the row | in the text |
|---|---|---|
| `EventDetail::ActionTrace` gains `suffered`, and the growth becomes 1 + 7 + 3 + 3 + 1 | yes | 3/3 phrases |

Two of the three are **understatements of a closed enumeration** — a public variant that already existed gaining a field, and one verb named where two share a path — which is the failure mode an enumeration written from the change rather than from the surface produces. The third is an approved ordering refuted by measurement rather than an enumeration defect, and `escalation.md` is the record it was decided against.

## 4. What is retained rather than deleted

Four sentences were inverted or retracted by the amendments above, and `SPEC-MOK-004` rule 11 requires the record to keep what it corrects. So each must still be present — and must be present **as a quotation of a superseded position**, not as a position. No phrase search can show that a phrase is gone; here nothing may be gone, and what must be gone is its standing. So each paragraph holding the sentence is located and the introducer that makes it a quotation is asserted **in that same paragraph**. The count is not the check — a sentence may honestly be quoted twice, once by the amendment that retracted it and once by a later one naming the position it replaced — but an occurrence in a paragraph that introduces it as nothing is the retracted claim still asserted, and that is what the last column reports.

| Artifact | the superseded sentence | present | paragraphs holding it | holding it as a claim |
|---|---|---|---|---|
| `SPEC-MOK-001` | rule 3's refusal to carry `fear` | yes | 1, introducing it as superseded | none |
| `SPEC-MOK-001` | rule 12's closing sentence | yes | 1, introducing it as superseded | none |
| `SPEC-MOK-001` | rule 5's refusal to state an obligation | yes | 1, introducing it as superseded | none |
| `SPEC-MOK-003` | clause 5's reason for reserving the fourth slot | yes | 2, each introducing it as superseded | none |

`SPEC-MOK-003`'s is quoted twice and both are quotations: the 2026-08-19 amendment retracted it, and this chain's 2026-08-20 amendment names it again as "the earlier position" it is now satisfied a second way against. That is why this table counts paragraphs that hold the sentence as a claim rather than paragraphs that hold it at all.

## 5. What was amended beyond the list, and what the list asked for and did not get

### `REQ-MOK-057`, `REQ-MOK-058` and `VER-MOK-016`: amended inside their own chain

These three are not on the *Required amendments* list, because on 2026-08-20 they were being written rather than amended. The measurement refuted them after they were approved, and each carries a row of its own.

| Artifact | rows | the amendment | the act, as its own cell records it | ground named |
|---|---|---|---|---|
| `REQ-MOK-057` | 2 | The branch order and the engagement threshold | **approved** 2026-08-20, product owner and technical owner | `evidence/WO-MOK-016/escalation.md` |
| `REQ-MOK-058` | 2 | The floor of five is ratified unchanged | **ratified** 2026-08-20, product owner | `evidence/WO-MOK-016/escalation.md` |
| `VER-MOK-016` | 3 | Realigned to `REQ-MOK-057`'s first amendment | **approved** 2026-08-20, assurance owner | `evidence/WO-MOK-016/escalation.md` |

`REQ-MOK-058`'s cell says *ratified* rather than *approved* and the distinction is not cosmetic: its value did not move. The row records that the floor of five survived the first measured curve unchanged, against two lower alternatives measured and declined in the same act, which is a different act from amending a figure.

`REQ-MOK-057` and `REQ-MOK-058` each open with an **Original approved content** row, so the ordering that was refuted and the bound that was ratified are both readable at their approved form rather than only in their corrected one. `VER-MOK-016` carries three rows: its original content, its realignment to `REQ-MOK-057`'s amendment, and oracle 5's restatement.

### `SPEC-MOK-003`'s reconciliation row: recorded, and standing on no owner act

| | |
|---|---|
| located | yes |
| recorded by | the implementation agent, as a statement of fact about amendments it holds no authority over |
| ratifies anything | no — "Nothing is ratified here and no provision changes" |
| precedent cited | the two 2026-08-19 reconciliation rows above it |

It states that two owner acts written against different trees met in a merge and both hold, and it changes no provision. **It stands on a precedent rather than an approval**, which is named as a residual in `completion-summary.md` item 17 rather than settled here: whether a statement of fact about two amendments needs an act of its own is the owner's to decide.

### `REQ-MOK-014`: the amendment the list asked for and the measurement did not trigger

| | |
|---|---|
| amendment rows in total | 3, the latest dated 2026-08-17 |
| rows dated 2026-08-20 or 2026-08-21 | **0** |
| identical to `master` | yes |

`WO-MOK-016` made this amendment conditional in its own words: the floor "is expected to stand; whether it does is measured, and if it does not the amendment is the owner's decision". The measurement is in `post/byte-identity.txt` and `completion-summary.md` item 7: all sixty declared cells of `reference` and `individual` are byte-identical to the pre-change capture, and identical output is identical survivors. So the floor is **preserved rather than re-established**, the condition never fires, and the absence of a row is the correct state rather than a missing act. Oracle 7 names three requirement amendments; two exist and the third was contingent on a measurement that did not move.

### `SPEC-MOK-004`: an amendment this work order owes and has not made

| | |
|---|---|
| rows naming `WO-MOK-016` | **0** |
| identical to `master` | yes |

Rule 11 states the obligation in its own text: a work order that adds a test corrects the recorded figures there. This work order adds tests — `post/test-census-reconciliation.md` counts them — and **has not corrected them**. It is recorded as owed rather than done, for the reason `completion-summary.md` item 17 finding 1 gives: correcting the figures moves the tree that this packet's census, test-run and updated-test captures were taken against, and rule 11 requires figures to be re-derived rather than edited. Whether that re-derivation happens under this work order or the next is an owner's call, and it is the one thing on oracle 7's surface that is neither present nor contingent.

## 6. The earlier layer

Every amendment row that existed at the baseline, compared against the candidate in order. Some of them changed, and **none of them changed here**. A check that simply required byte equality would report a defect where the truth is another work order paying a debt through the merge, so the two cells of each row are classified separately: identifiers are masked to see through this chain's own renumber, and the operative cell is compared apart from the approval cell, because an edited provision and a moved approval state are different events and only one of them is legitimate after the fact.

| Artifact | rows at the baseline | rows now | identical | status moved | renumbered only | operative cell moved | absent |
|---|---:|---:|---:|---:|---:|---:|---:|
| `SPEC-MOK-001` | 11 | 13 | 10 | 0 | 1 | 0 | 0 |
| `SPEC-MOK-002` | 5 | 7 | 1 | 3 | 1 | 0 | 0 |
| `SPEC-MOK-003` | 12 | 22 | 7 | 4 | 1 | 0 | 0 |
| `SPEC-MOK-004` | 5 | 7 | 4 | 1 | 0 | 0 | 0 |
| **total** | **33** | | **22** | **8** | **3** | **0** | **0** |

So of the 33 rows the baseline held, 22 are byte-identical and 11 changed: 8 whose approval cell moved with the operative cell untouched to the byte, and 3 of this chain's own rows carrying nothing but the renumber recorded below. **No operative cell moved and no row is absent.**

| Artifact | row | subject | what changed | the act that changed it |
|---|---|---|---|---|
| `SPEC-MOK-001` | 2026-08-20 | Contact, conflict and society, under `CAP-MOK-009`. Thirteen provi | renumbered, and nothing else | this chain's own row, renumbered |
| `SPEC-MOK-002` | 2026-08-18 | Four provisions amended so that the terminal observer of `SPEC-MOK | status only | `WO-MOK-012`, on `master`, through the merge |
| `SPEC-MOK-002` | 2026-08-18 | Every root-relative path re-based on `mokiterions-core/`, the engi | status only | `WO-MOK-012`, on `master`, through the merge |
| `SPEC-MOK-002` | 2026-08-19 | Rule 5's enumeration amended in two entries, under `REQ-MOK-032` a | status only | `WO-MOK-012`, on `master`, through the merge |
| `SPEC-MOK-002` | 2026-08-20 | Rule 5's enumeration amended and rule 6 re-checked, under `CAP-MOK | renumbered, and nothing else | this chain's own row, renumbered |
| `SPEC-MOK-003` | 2026-08-18 | Corrected the statement of required `SPEC-MOK-002` amendments in * | status only | `WO-MOK-012`, on `master`, through the merge |
| `SPEC-MOK-003` | 2026-08-18 | *Data and interface contracts* corrected on a claim about the engi | status only | `WO-MOK-012`, on `master`, through the merge |
| `SPEC-MOK-003` | 2026-08-18 | Four provisions amended so that `REQ-MOK-028`, `REQ-MOK-029` and ` | status only | `WO-MOK-012`, on `master`, through the merge |
| `SPEC-MOK-003` | 2026-08-19 | Rule 4 amended in three provisions, under `REQ-MOK-032`, so the ro | status only | `WO-MOK-012`, on `master`, through the merge |
| `SPEC-MOK-003` | 2026-08-20 | Three provisions amended under `CAP-MOK-009`, and the frontmatter' | renumbered, and nothing else | this chain's own row, renumbered |
| `SPEC-MOK-004` | 2026-08-19 | Recorded figures and two subject lines corrected, because `SPEC-MO | status only | `WO-MOK-012`, on `master`, through the merge |

The 8 approval cells that moved all moved in the same direction and by the same act: `WO-MOK-012` ratified them on `master` on 2026-08-20, and each cell now names that act, the role it was taken in, and the interval it was outstanding for. 1 of them also corrects its own earlier wording, having said that both of two rows remained outstanding when only the first ever was — a miscount its new text reports rather than drops. The 3 renumbered rows are this chain's own amendment rows, and masking the identifiers shows both cells of each unchanged in every other character.

| | |
|---|---:|
| `SPEC-MOK-001`: `master`'s rows are a prefix of the candidate's, in order | yes |
| `SPEC-MOK-002`: `master`'s rows are a prefix of the candidate's, in order | yes |
| `SPEC-MOK-003`: `master`'s rows are a prefix of the candidate's, in order | yes |
| `SPEC-MOK-004`: `master`'s rows are a prefix of the candidate's, in order | yes |

So the rows this chain adds are **appended after everything `master` holds**. No row of `master`'s is edited here, reordered, summarised or folded into a later one; the 8 of them that differ from the baseline differ in their approval cells, in the direction of being ratified, by an act this branch did not take and does not claim.

| Rows carrying **OUTSTANDING** as their own state | count |
|---|---:|
| at the baseline `39662d13` | 4 |
| at `master`'s tip `d8e20794` | 0 |
| at this candidate | 0 |

The four at the baseline are the gate of section 8. **None is outstanding now**, and the act that closed each is `WO-MOK-012`'s, on `master`, recorded in the row it closed. The count covers every non-evidence artifact in `docs/engineering/simulation`, not only the four: a row left outstanding anywhere else would appear here.

## 7. What must not have changed at all

| Artifact | changed since `master` | line endings only | why it must not be |
|---|---|---|---|
| `SPEC-MOK-004` | no | yes | the test-census authority. Rule 11 obliges a figure correction this work order owes and has not made; section 5 measures it |
| `SPEC-MOK-005` | no | yes | the release specification; no amendment is required |
| `REQ-MOK-014` | no | yes | the default source's survivor floor. Its amendment was conditional on a measurement that did not move it; section 5 |
| `ARCH-MOK-001` | no | yes | no architecture amendment is required by this work order |
| `ARCH-MOK-002` | no | yes | the same |
| all 14 verification records | no | — | commit-bound records; none is re-opened, `VREC-MOK-005` and `VREC-MOK-010` included |

The middle column is why this table compares content rather than raw bytes: this clone has `core.autocrlf = true`, so a tracked governance file is CRLF in the working tree and LF in the blob. The check is run both ways — equal after folding CRLF to LF, and raw bytes equal once both sides are folded — so the normalization cannot absorb an edit.

`VREC-MOK-010` is in that list for a reason beyond commit-binding. It states that the `VREC-MOK-005` gate "is not satisfied" and that the transition of that record to `verified` "does not close the gate" — "a cost carried forward, not a debt paid". That was measured at its own commit and it is not edited here. What changed since is measured in the next section.

## 8. The `VREC-MOK-005` gate, recorded

| | |
|---|---|
| `VREC-MOK-005` status | `verified`, updated 2026-08-19, binding `f3613701` |
| its own statement of what it accepted | "with all seven manual assessments outstanding and unauthored, and eleven provisions across four approved artifacts awaiting the technical owner" |
| that sentence at this candidate | unchanged — the record is not edited, and it was correct at its commit |

### The eleven provisions

| Artifact | provisions | **OUTSTANDING** at the baseline | now | the act |
|---|---:|---|---|---|
| `SPEC-MOK-002` | 4 | 1 row | 0 | ratified 2026-08-20 by the technical owner under `WO-MOK-012` |
| `SPEC-MOK-003` | 1 | 1 row | 0 | ratified 2026-08-20 by the technical owner under `WO-MOK-012` |
| `SPEC-MOK-004` | 5 | 1 row | 0 | ratified 2026-08-20 by the technical owner under `WO-MOK-012` |
| `ARCH-MOK-001` | 1 | 1 row | 0 | ratified 2026-08-20 by the technical owner under `WO-MOK-012` |
| **total** | **11** | 4 rows | 0 | |

`WO-MOK-012`'s own `amendment-ratifications.md` opens on the same count — "Eleven provisions across four approved artifacts were amended in the tree with an Approval cell reading **OUTSTANDING**" — and records that all eleven were ratified **as written, without modification**, by the repository owner acting as technical owner, each by editing that row's Approval cell in place rather than appending a row. Section 6's classification is what checks that: the operative cell of every one of those rows is unchanged, so the act ratified text rather than replacing it.

### The seven manual assessments

| | |
|---|---|
| record | `evidence/WO-MOK-012/manual-assessment.md`, "closing `VER-MOK-005`'s seven" |
| assessments in its summary table | **7** |
| authored | **6**, every one by the repository owner in the role named |
| outstanding | **1** — assessment 7, "outstanding by decision" |

The seventh is the terminal-restoration inspection, and it is open **by the assurance owner's decision of 2026-08-20** rather than by omission: the contract asks for a live inspection "rather than only by an automated assertion", and the shipped binary has no operator-reachable panic path, so performing it would require inspecting a terminal restored by a program that is not the one released. The recorded decision is to leave it outstanding and visible, and that `VREC-MOK-005` must continue to disclose it.

### What that means for this contract, and what it does not

- **The amendment half of the gate is closed, and not by this work order.** Eleven of eleven provisions are ratified, each in the row it belongs to, by the technical owner on 2026-08-20 under `WO-MOK-012`. This branch inherited the closure through the merge at `259859df` and claims no part of it.
- **The assessment half is six of seven, with the seventh open by a recorded decision.** That is a different state from unauthored, and a different state from closed.
- **`VREC-MOK-010`'s reading is superseded rather than wrong.** It measured the gate open at its own commit and said the status had moved while the substance had not. The substance has since moved, on `master`, and this file is where that is recorded for this chain.
- **This contract still does not close `VREC-MOK-005`.** `VER-MOK-016` says so in its own residual-uncertainty section, and nothing here transitions that record, performs its seventh assessment or re-opens its commit.

## The controls on the checks themselves

All 19 held, so no line above is a check that looked for nothing:

- ok — a provision stated in its row and carried by the body reads clean
- ok — a provision its row does not state is reported
- ok — a provision absent from the body is reported
- ok — a row's own prose cannot satisfy a body check
- ok — a subject phrase matching two rows locates neither, rather than the first
- ok — a provision recorded in the wrong row is reported, not credited
- ok — a superseded sentence quoted by its introducer reads clean
- ok — a superseded sentence standing in a paragraph of its own is reported
- ok — a second copy in a paragraph that introduces it as nothing is reported
- ok — a sentence quoted twice, each time as superseded, is not reported
- ok — a retained sentence that was deleted instead is reported
- ok — the row reader finds rows in order and ignores prose
- ok — a row outstanding in its own status cell is told from one that mentions the word
- ok — a row ratified in place, its provision untouched, is classified as a status change and not as a provision change
- ok — a row carrying this chain's renumber and no other change is classified as that
- ok — a row both renumbered and ratified is reported as both, not as one
- ok — a row whose operative cell was rewritten is reported as absent, not absorbed
- ok — a row whose operative cell was reworded within the same status is reported
- ok — an unchanged row is reported as identical rather than as a change

## Result

| Check | Result |
|---|---|
| the chain carries the statuses it should | PASS |
| every amended specification specifies the requirements it must | PASS |
| every provision on the approved list is in its row and in the text | PASS |
| each row taken after the single act stands on its own recorded approval | PASS |
| every superseded sentence is retained as a quotation and nowhere as a claim | PASS |
| the three rows amended inside their own chain record their act and its ground | PASS |
| no earlier provision moved | PASS |
| this chain's rows are appended after `master`'s, none interleaved | PASS |
| nothing that must not have changed changed | PASS |
| the gate's assessments are measured, six authored and one open by decision | PASS |

**RESULT: PASS.**

What that means, stated as the claims it rests on. Every artifact in the chain carries the status it should, and the three amended specifications specify the requirements that made this chain approvable at all. All 21 subjects of the approved list are present in the row that must state them **and** in the artifact with every row removed, which are disjoint texts, and each row is found by its own subject rather than by a date eleven rows share. The three rows taken after the single act each stand on an approval their own cell records, with the role and the ground named. The four superseded sentences are retained and every paragraph holding one introduces it as superseded. No operative cell of any earlier row moved, this chain's rows are appended after everything `master` holds, and nothing that must not have changed changed.

What this file does **not** establish:

- **That oracle 7 is satisfied.** Two things on its surface are not acts this file can find. `SPEC-MOK-004` rule 11's figures are owed and unmade, by a decision recorded in `completion-summary.md` item 17 rather than taken; and `SPEC-MOK-003`'s reconciliation row stands on a precedent rather than an approval. Both are named for the owner, and neither is silently absorbed here.
- **That the `VREC-MOK-005` gate is closed.** Its amendment half is, its assessment half is six of seven, and whether that state satisfies this contract's static row is the assurance owner's reading. `manual-assessment.md` in this packet is where `VER-MOK-016`'s own eleven assessments wait, and this file is one of the things assessment 11 reads.
