# The assurance decision on `WO-MOK-017`, taken 2026-08-22

| | |
|---|---|
| What this file is | the record of one accountable act: the transition of `VREC-MOK-020` from `ready` to `verified`. It is **not** the verification record and it claims no coverage |
| Decided by | the repository owner acting as **assurance owner**. `DECISION_RIGHTS.md:14` reserves this transition to that role |
| Date | 2026-08-22 |
| Candidate commit | `a30ae327a92f5acdfb294a3c3d98e32b806300d1` |
| Record | `VREC-MOK-020`, now `verified`. Its own *The assurance decision, taken 2026-08-22* section is the authoritative statement; this file is the reading behind it |
| Declared in `evidence_paths` | **no, deliberately.** See *Why this file is not declared* below |

`merge/README.md`'s closing list, item 2, reads *"**The two `RESULT: FAIL` reports need a reading, not a fix.** Both are
disclosed above and in `gates.txt` §5, and both are `master`'s arrival. Whoever prepares `VREC-MOK-017` decides whether
the reasoning offered for each is sufficient"*. **This file is that reading**, taken by the owner rather than by the
preparer, and recorded under `VREC-MOK-020` because `VREC-MOK-017` was already taken by `WO-MOK-016`'s record. It was
written after the decision, on the material the packet retained, and it invents no figure: every number below is quoted
from a file in this packet or measured by a command reproduced here.

## The instruction

Verbatim and complete:

> i validate the verification record, you can transition it to verified and commit. Then you can push the branch +
> create the PR

One validation and four acts. **Two decisions were put to the owner before the transition, because the record could not
be moved without them and neither was an implementation agent's to take:** what the verified record says about the two
`RESULT: FAIL` readings, and whether finding 7's `VER-MOK-016` amendment row is taken in this branch. Both were answered
with the costs and the alternatives stated. Nothing else was inferred from the instruction.

## Reading 1 — `merge/world-rules-merged.txt`: no world rule moved

The report reads, at exit 1:

```
RESULT: FAIL -- 3 check(s) failed:
  61 changed engine block(s) outside the condition family
  Simulation::apply_action, the eat arm differs between the two commits
  Simulation::regenerate_food differs between the two commits
```

**Decision: accepted. `REQ-MOK-060`'s `static-analysis` row *the correction is where it is permitted to be* is satisfied
on the merged source as well as at `26ae6ba`.**

**Why the report fails at all.** `analysis/world-rules.py` compares the pre-change engine — `pre/COMMIT.txt`, which reads
`1ba5a3aabe775c9cdee29a04b399af3bb82dde90` — against whatever source it is handed. Handed the merged tree instead of the
candidate, it necessarily reports `master`'s arrival as well as this work order's own change: the non-test engine extent
goes from **3012** lines pre-change to **3772** merged, and **68** blocks differ, of which 7 sit inside the non-waste
condition family and **61** outside it. **The reader is doing its job; the question is whether any of what it found is a
world rule.**

**The 61 blocks are the record stream** — `gates.txt` §5 says so in those words — outside the condition family by the
reader's own classification, and outside the row's subject.

**The two that bear on a world rule were diffed region by region, with the reader's own extractors, and `merge/gates.txt`
§5 retains the whole of each difference:**

| Function | Extent | The whole of the difference |
|---|---|---|
| `apply_action`, the eat arm | candidate 44 lines, merged 46 | `+ let class_index = food.class.index();` · `+ self.consumed[class_index] = self.consumed[class_index].saturating_add(1);` · `self.emit(output, event)` → `self.emit(sinks, event)` |
| `regenerate_food` | candidate 61 lines, merged 65 | `output: &mut W` → `sinks: &mut Sinks<'_, '_, W>` · `+ let reason_index = reason.index();` · `+ self.regeneration_skipped[reason_index] = …saturating_add(1);` · `+ self.regenerated = self.regenerated.saturating_add(1);` · two more `output` → `sinks` at the `emit` calls |

**A sink parameter and three saturating counters**, in `gates.txt` §5's own summary. The restoration, the clip, the food
table and the regeneration decision are untouched. The four things the retained artefact undertakes to show unchanged —
rule 4, rule 9's eat effect, the food table, and rules 14 to 16 — are none of them among the differences.

**The corroboration that makes this a measurement rather than an inspection.** `merge/capture-comparison.txt` re-took the
whole 120-cell capture at the merge commit `ae2e44f2382fe89f2daf78a8e8aca37febb8bd0f`, from that commit's own archived
tree, and compared it to `post/post-manifest.txt` cell for cell on four columns — raw SHA-256, byte count, line count,
exit code — across all four sources at 30 cells each, reading
`RESULT: PASS -- 120 of 120 cells identical on all four columns, 0 differing`. **Identical output streams cannot come from
a changed world rule.** The region-by-region diff says what changed; the 120 cells say that nothing observable changed.
The two arguments are independent and they agree, and `gates.txt` §5 makes the same appeal — *"identical streams cannot
come from a changed rule"*.

**What that table explicitly does not cover, and so neither does this reading:** the record stream itself. `capture.sh`
never passes `--events-path`, so the 120 cells exercise the default path and say nothing about the sink. `VREC-MOK-019` is
where the stream is verified.

**What is accepted and what is not.** Accepted: that no world rule moved between `26ae6ba` and the merged source, so the
row holds on both trees. **Not accepted, because it was not asked:** anything about whether `master`'s instrumentation is
correct, well-placed or well-tested. `VREC-MOK-019` covers that work at its own commit and this decision does not reach
it.

## Reading 2 — `merge/reads-merged.md`: the oracle holds, the failure is a completeness check

The report reads, at exit 1:

```
RESULT: FAIL -- 1 check(s) failed:
  Simulation::write_metrics_territory calls food_counts and is not classified
```

**Decision: accepted. `REQ-MOK-060`'s `static-analysis` row *No composition read by any source* is satisfied on the
merged source as well as at `26ae6ba`.**

**What the requirement forbids is a decision source reading composition.** `write_metrics_territory` is not a decision
source. It arrived with `master`, takes `&self` and a `&mut dyn Write`, has exactly one caller — `write_metrics`,
`SPEC-MOK-006` rule 7.6's metrics record — and writes JSON. **It is on the observation side of `INT-MOK-010`, which is
the side permitted to read composition.**

**The substantive checks in the same report pass**, and they are the ones the row turns on: each of the four
`DecisionSource::decide` functions takes only an `Observation`, and `Observation` carries no composition field. A new
source inherits the same inability, and granting one composition would mean amending `SPEC-MOK-001` rule 3 in the open.
What fails is the reader's completeness check — it holds a hand-written classification for every caller of `food_counts`
it knows, and a caller arrived that it does not know.

**Why the fix was refused and the failure kept.** Writing the classification into `analysis/reads.py` would edit a reader
whose output is retained evidence at the candidate commit. **The packet's rule is that evidence is re-run, never edited.**
So the report is left failing, at exit 1, and the classification is recorded here instead.

## What both acceptances do to the coverage matrix

`VER-MOK-016` declares **93** requirement-to-evidence rows. `VREC-MOK-017` claimed **88** and carried `REQ-MOK-060`'s
**5** to this work order.

| `REQ-MOK-060` row | Method | Before this decision | After |
|---|---|---|---|
| The ceiling (oracle 4) | automated-test | satisfied | satisfied |
| Measured from rule 18's summary (oracle 4) | automated-test | satisfied | satisfied |
| The pre-change state is reproduced as the contrast (oracle 1) | automated-test | satisfied | satisfied |
| The correction is where it is permitted to be | static-analysis | claimed at `26ae6ba` only | **satisfied on both trees** |
| No composition read by any source | static-analysis | claimed at `26ae6ba` only | **satisfied on both trees** |

**93 of 93 between the two records, and `VER-MOK-016`'s matrix is satisfied for `REQ-MOK-060`.**

## What this decision does not do

- **It edits no evidence.** Both reports keep their `RESULT: FAIL` and their non-zero exit codes. Nothing in `post/`,
  `pre/`, `approval/`, `analysis/` or `merge/` is touched by the commit that carries this file.
- **It takes no product-owner or technical-owner act.** The three manual assessments and eight of the nine findings were
  recorded on 2026-08-22 at `9105ae3`; this decision re-records none of them.
- **It does not take finding 7's amendment row.** `VER-MOK-016`'s realignment row of 2026-08-21 still reads *"Manual
  assessment 7"* where the item amended on both halves is **acceptance scenario 7**. The owner verified the record
  without it, on the finding's own ground that the substance landed correctly and nothing depends on the label. **The row
  is still owed by the assurance owner.**
- **It ratifies no amendment.** `SPEC-MOK-004`'s 2026-08-21 row for rules 9, 10 and 11 still carries `**OUTSTANDING.**` in
  its authority cell, so rule 11 still states the workspace's **301** against a tree that runs **302** — the exact reading
  `merge/README.md` item 1 says a record "can hold as long as it says so", and this decision holds it and says so.
  `REQ-MOK-060`'s *Open decisions* third bullet still reads "remains deferred"; `VER-MOK-013` and `VER-MOK-005` each still
  owe the row findings 2 and 3 name.
- **It moves no work order.** `WO-MOK-017` stays `implemented`, on the `WO-MOK-016` precedent under the `verified`
  `VREC-MOK-017`.
- **It authorizes no merge, tag or release**, and makes no commit release-eligible. `origin/master` already carries this
  work order's implementation through `f7b1c45`, taken before this record existed and on other grounds.
- **It closes no residual.** The per-run draw total stays undecidable at this candidate; `REQ-MOK-014` and `REQ-MOK-034`
  stay met at margin 0; nothing is measured past tick 1,000; the one density-`1.50` evaluation above three fifths stays
  disclosed and unbound.

## Why this file is not declared in `evidence_paths`

`VREC-MOK-020` declares **319** files and this is not one of them. **A record's declared evidence is what the decision was
taken on, not the account of the decision itself** — the rule `VREC-MOK-019` stated in advance, on the `WO-MOK-011`,
`WO-MOK-014` and `WO-MOK-019` precedents.

Two consequences, both measured rather than argued:

1. **The candidate's tree is unchanged.** This file lands in a commit after `a30ae32`, so the record's claim that *"319
   declared against 319 in the candidate's tree, with nothing left over on either side"* stays true of the commit it is a
   statement about.
2. **It cannot move `artifact_snapshot_sha256`.** The hashed document's evidence node keys on a `WO-MOK-NNN-` **filename**
   prefix, and this chain retains a **directory**, so no file inside `evidence/WO-MOK-017/` is discovered by it at all.
   That is `W-HEX-001`'s standing observation against `WO-MOK-017`, seen from the inside, and it is why the digest
   `24a6fbd1…` is a figure of the graph and not of this packet's contents.

## Figures, before and after

Measured at `HEAD = b017e31`, the commit that carries the `ready` record, with artifact content as the only moving input.
All commands from the pinned `0.4.0` venv at `C:\Users\mathi\harness-venv-040`.

| | Before the transition | After |
|---|---|---|
| `validate` | PASS — 148 artifacts, 0 errors, 0 warnings, all four planes `E0/W0` | **identical** |
| `inspect` findings | 39 — 0 error / 19 warning / 20 info | **39 — 0 / 19 / 20**, unmoved |
| `decision_required` | **1** — `VREC-MOK-020 [ready] assurance-review` | **0 — none** |
| `assurance pending` | 0 | **0** |
| `definitions pending` | 1 — `WO-MOK-008 [draft]` | **1**, untouched by this act |
| Warning codes | `W-HEX-001` 8, `W-HEX-003` 11 | **8 and 11**; neither names this record in its observations |
| `I-REV-001` | 20 observations | **20**, this record among them either way |
| `dashboard` | PASS — 148 artifacts, 527 relations, 0 errors, 19 warnings | **identical** |
| `dashboard` snapshot | `4c2d2679891a254152bb61ca414a9ff3a2550a3c07eb1f4866239e18eea8dffb` | **`62b586a33dcc102a99fd2c77982b3e7fc0cf934b12c25e34eb83bddfa21a354c`** |
| `doctor` | 81 checks, 81 PASS, 0 FAIL | **81 / 81 / 0** |
| `preflight --work-order WO-MOK-017 --phase review` | PASS, `WO-MOK-017 (implemented)` | **identical**, and still `Commit-bound verification: required` |

**The one figure that moves is the snapshot digest, and it moves for exactly the reason the record states.** `HEAD` is
`b017e31` for both readings, the graph is 148 artifacts and 527 relations for both, and the only changed input is
`status = "ready"` → `status = "verified"` in normalized front matter. That is the transition's whole measurable footprint
on the harness: `4c2d2679…` → `62b586a3…`, no error, no warning, one fewer entry in the decision queue.

**Neither digest is `24a6fbd1…`, and neither is meant to be.** The declared value is a figure of commit `a30ae32`, taken
in a detached worktree whose leaf directory name equals the repository's, and it is unreachable from any working-tree run
at a later `HEAD`. Both readings above are working-tree figures at `b017e31` and are published as such.

**`W-REV-004` never fired here and is not silenced by this transition.** Its predicate is a `ready` record over a work
order already fully covered by a `verified` or `released` one, and `WO-MOK-017` had no prior record —
`VREC-MOK-017` is `WO-MOK-016`'s. That is the one difference from `VREC-MOK-019`'s transition, where the rule fired and
had to be argued both ways.

**The snapshot digest the record declares does not move**, and cannot: `24a6fbd1…` is a figure of commit `a30ae32`, whose
tree holds no copy of this record to change. `build_snapshot` hashes normalized front matter, so even the status field
that moved here is invisible to it at that commit.

## What this decision costs, disclosed

**The supersession route is now foreclosed permanently.** `VERIFICATION_RECORD.template.md` permits a governance decision
to change only a **`ready`** record to `superseded`. `VREC-MOK-020` is no longer `ready`, so if its bound commit
`a30ae327a92f5acdfb294a3c3d98e32b806300d1` is ever rewritten — by a rebase of `governance/wo-mok-017-closure`, or by
merging anything beneath it in a stack first — the record will bind a commit that does not exist and **there is no
remedy**: it can be neither re-pointed nor superseded. This is the exposure
`evidence/WO-MOK-014/merge/assurance-decision.md` priced when `VREC-MOK-015` was transitioned, and `VREC-MOK-016` is this
chain's own instance of paying it.

**The consequence for the pull request the same instruction authorized: it must be merged without rewriting the branch.**

Every command behind every figure in this file is offline, reads no credential, secret, token or environment value, and
none appears in the retained evidence.
