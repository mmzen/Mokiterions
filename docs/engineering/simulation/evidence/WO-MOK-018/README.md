# Evidence — WO-MOK-018

`WO-MOK-018` closes the two defects Phase 3.1 left in the observer: `SPEC-MOK-003` rule 9 item 2 stated a
count of core event types that no tree runs, and `fear` — the fourth attribute `CAP-MOK-010` and
`REQ-MOK-032` put on the roster's bars — became unreadable at the moment a Mokiterion dies, where rule 10
item 6 makes the inspector responsible for its final values. Both are defects against text already in
force. No requirement is created: `REQ-MOK-021` and `REQ-MOK-022` are `approved` and already carry both.
This directory is the work order's retained evidence, and its *Evidence to record* is the list satisfied
here.

**What this work order is.** It changes executable behaviour. `mokiterions-tui/` gains one public field,
one widened private map, two `ingest` arms and a two-line death presentation; **`mokiterions-core/` is not
touched by one byte**, which `git diff --stat` measures rather than states. Half the scope is specification
text no build reads, which is why the assurance classification is `required` rather than inferred.

**What its status is.** `WO-MOK-018` is **`in_progress`** and is left there. `implemented` asserts the
completed change **and** the retained evidence as an accountable judgement, which is the owner's act and
not the implementation agent's — the precedent is `WO-MOK-013`, where the agent held the work order at
`approved` through implementation for exactly this reason and the owner moved it. No `VREC` is written
here.

**Three provisions are OUTSTANDING and none of them is presented as approved text.** The owner approved
the work order, its two decisions and four amendments in one act on 2026-08-21 and directed implementation.
Implementation found a fifth amendment and two provisions inside amendments already approved:

| Outstanding | Where it is recorded | Why the approving act does not reach it |
|---|---|---|
| `SPEC-MOK-004` rule 6's figures — **amendment 5** | the rule, and `interface.md` | not among the four; the work order asserted the figures would be unmoved, and two of the three move |
| rule 10.6's **two-line pairing** | `SPEC-MOK-003` rule 10, and `inspector.md` | inside an approved amendment, but the owner was not shown it — the clipping it answers was found after approval |
| `VER-MOK-005`'s stale **`name`** | that contract's 2026-08-21 row | a correction of a false statement about another artifact, outside approved scope, and **not this work order's defect** |

**The one measurement that decided a design.** In-scope item 3 asked for `fear` appended to the existing
death line. That was implemented first and the added frame case failed: 45 columns against an interior of
42, and the paragraph carries `Wrap { trim: false }`, so it *wraps* rather than truncates — `fear` ends one
row as a label with no value and `90` begins the next as a value with no label, which is the appearance
rule 10.7 exists to prevent. The withdrawn form is captured through the same oracle into the same panes, so
the pairing rests on evidence rather than on argument.

## The files

| File | What it establishes |
|---|---|
| `README.md` | This index. Every file in the pack, what it establishes, and what is deliberately absent. |
| `completion-report.md` | The eight-point report `WO-MOK-018`'s *Completion report format* specifies: each in-scope item, each amendment with its acting owner and the three things the tree did not bear out, the census, rule 6's figures, the gates, every consequence derived rather than decided with its ratification state, six findings carried, and eight things this work order does not claim. |
| `test-census.md` | Rules 9, 10 and 11 reconciled across three trees: 226 → 267 as 41 arrivals, 0 departures and 1 rename in neither figure, each arrival attributed to `WO-MOK-016` or to this work order, the tier split cross-checked as 42 + 103 = 145, and each of this work order's three tests placed with its ground. |
| `interface.md` | Rule 6 re-counted and **grown**: 94 items unchanged, 24 → **25** public fields, 118 → **119** `pub` lines. States that the work order predicted the opposite, why the prediction was wrong, and the three alternatives measured and worse. |
| `inspector.md` | The death line at both viewports that matter, the withdrawn one-line form measured beside it, the width table at two and three digits, why the pairing rather than a rule 5 move, and the one clause no test reaches. |
| `non-perturbation.md` | `REQ-MOK-025` at five declared seeds and two depths: identical in every authoritative record and in final state, with seed 42 at 300 ticks agreeing with `WO-MOK-013`'s independently recorded 7,534. |
| `filter-vocabulary.md` | Amendment 1's figure by measurement: 14 stable core types plus `action_trace`, the three that made eleven into fourteen, and the standing risk that prose duplicating a derived value goes stale silently. |
| `gates.txt` | Every declared gate command in the form *Required verification* item 6 names, with its exit code and output tail. All eleven `exit=0`. Records the two departures from that item's wording. |
| `test-census.txt` | The census itself: `#[test]` counts **by name** per file on all three trees, with arrivals, departures, the tier split, the name-by-name diffs that isolate the rename from the arrival, and that no test carries `#[ignore]`. |
| `interface.txt` | The enumeration: `pub` lines, hooks, items and fields per module, on the implementing tree and at `f2a79e1`, with the sole arrival shown from `git diff`. |
| `inspector.txt` | The implemented pane, read cell by cell including its border, at `160 × 48` and `140 × 22`, in both the reported-`fear` case and the no-record case. |
| `inspector-one-line.txt` | The **withdrawn** one-line form, same oracle, same panes. The counterfactual behind the pairing, with the temporary `render.rs` edit and its SHA-256-verified revert recorded in its header. |
| `filter-vocabulary.txt` | `EventType::ALL` enumerated with each member's `event=` string, partitioned 14 core and 1 optional as `SPEC-MOK-001` partitions them. |
| `non-perturbation.txt` | The comparison itself: five seeds at 60 and at 300 ticks, record counts both ways, first difference, final state, and seed 42's summary line reproduced from both sides. |
| `wo018-oracle.rs` | The temporary oracle behind the three `inspector*` and `filter-vocabulary` captures. A **child module of `state`**, run once, removed; retained here and **not in the tree**. |
| `wo018-non-perturbation-oracle.rs` | The temporary oracle behind `non-perturbation.txt`, appended to `tests/verification.rs` so it reused that file's declared sets and helpers. Retained as a snippet: it **will not compile on its own**. |

Sixteen files. Every figure in the six `.md` documents traces to a line in one of the five `.txt` captures
or to a command in `gates.txt`.

## On the oracles

Both were placed in the tree, run once, and removed. **Neither is in the tree the census, the interface
figures or the gates were measured on** — `cargo test --workspace` reports 269 with the first present and
268 with the second, and neither figure is a census figure.

**No item was widened for either, and no fifth `#[cfg(test)]` hook was added.** `ARCH-MOK-002` names both
as prohibited patterns, and rule 6's interface is untouched by their presence: the hook count is 4 before
and 4 after.

`wo018-oracle.rs` is a child module of `state` rather than of the crate, and the reason is Rust module
privacy rather than convenience. It constructs a death for a subject the engine never reported survival
for — a state no run reaches, because the engine reports survival before it applies a death — and that
needs `Observer::ingest`, which has no visibility modifier and is therefore reachable from `state`'s
descendants only. A sibling module could not call it, and none of the four hooks injects a death.

**An oracle asserts nothing.** It writes down what the buffer holds, so that judging what the instrument
shows is a separate act. Every row of `inspector.txt` and `inspector-one-line.txt` is read out of a
rendered buffer cell by cell, border included, so a value the pane could not hold appears as a missing
character rather than as a shorter string — which is how the clipping was found at all.

`wo018-non-perturbation-oracle.rs` went into `tests/verification.rs` for the opposite reason: to reuse
that file's declared seed set, viewport set and helpers rather than reimplement the contract's own
comparison, which would have measured the reimplementation.

This follows `WO-MOK-013`'s `wo013-oracle.rs`, `WO-MOK-012`'s `assessment-oracle.rs`, `WO-MOK-010`'s
`observer/frame-probe.rs` and `WO-MOK-006`'s `frame-and-export-oracle.rs`.

## Reading order

`completion-report.md` first — it is the eight-point report the work order specifies and it states what
remains open. Then `interface.md` and `inspector.md`, which are the two documents that record something the
owner has not ratified. `test-census.md` next, because it is the one document that corrects another work
order's figures as well as this one's. `filter-vocabulary.md` and `non-perturbation.md` last: each closes
its half of the scope cleanly and neither carries an obligation.

## What is not here

- **No verification record.** `commit_bound_verification` is **`required`** and no `VREC` is written by
  this work order. A record binds a branch commit and never `master`'s merge, and writing one is a
  separate act.
- **No manual assessment.** `VER-MOK-005` contracts none for these two cases; the two added cases are
  automated, and the residual is disclosed rather than assessed.
- **No engine measurement beyond "untouched".** No file of `mokiterions-core` changes, so its recorded
  total of 85 at `f82cd3d` and 122 on the implementing tree are `WO-MOK-016`'s figures being carried
  through, not this work order's subject.
- **No measurement of rule 10.6's suppressed second line.** The death branch returns with that line last,
  so a line the code declined to emit and the pane's unwritten rows are the same cells; and counting the
  lines instead needs the private `inspector_lines` and the private `ingest`, which are **sibling**
  modules, so no test module is a descendant of both. `inspector.md` states it and `VER-MOK-005` discloses
  it as a residual. Closing it would cost a prohibited pattern.
- **No `fear` for a living selected subject**, no cause of death, no encounter tally, no direct filter
  jump to the three new types, and no canvas indication of an engagement. Every one is named in *Out of
  scope* and every one belongs to the next work order.
- **No figure inside any `VREC`, and no figure edited inside another work order's evidence.** Evidence is
  re-run and not corrected; a record bound to a commit remains true of that commit.
- **No amendment to the managed `engineering-harness.yml` workflow**, whose depth-1 checkout inflates the
  dashboard's warning count in CI. `VREC-MOK-017` records that, and *Out of scope* keeps it out.
- **No push and no pull request.** The change is committed on `feature/observer-fear-and-filter-count` and
  stops there.
