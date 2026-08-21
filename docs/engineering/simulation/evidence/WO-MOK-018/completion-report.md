# `WO-MOK-018` completion report

In the eight-point format the work order's *Completion report format* section fixes.

The work order is left at **`in_progress`**. The transition to `implemented` asserts the completed change
**and** the retained evidence as an accountable judgement, and that is the owner's act, not the
implementation agent's — the precedent is `WO-MOK-013`, where the agent held the work order at `approved`
through implementation for this reason and the owner moved it.

## 1. What was implemented, against each in-scope item

| In-scope item | State | What was done |
|---|---|---|
| 1. the rule 9 item 2 figure | done | `eleven` → `fourteen` in `SPEC-MOK-003` rule 9 item 2, with the amendment-record row. No code. |
| 2. `fear` on the death record | done | `Death` gains `pub fear: Option<u8>`; `latest_survival` widens from `BTreeMap<String, (u8, u8)>` to `BTreeMap<String, (u8, u8, u8)>`; `ingest`'s `SurvivalChanged` arm stops discarding the reported `fear` and its `AgentDied` arm reads it from that map exactly as it reads the other two. |
| 3. the rendering | done, **not in the form the work order foretold** | `inspector_lines`' death branch presents the four values **paired across two lines** rather than appended to one. The one-line form the item describes clips at every viewport presenting the inspector; see point 2. |
| 4. tests | done | three arrive — two public-tier, one internal-tier, each placed by the access it requires under `SPEC-MOK-004` rule 8. Point 3 and `test-census.md` carry the placements and their grounds. |
| 5. the `SPEC-MOK-004` figure corrections | done | rules 9, 10 and 11 corrected jointly for `WO-MOK-016` and this work order, in amendment 3's form, measured on the implementing tree. |
| 6. evidence | done | eleven files under `evidence/WO-MOK-018/`, indexed in that directory's `README.md`. |

No in-scope item was left unreached.

**The tuple was chosen over a named struct**, which the decision envelope leaves to the agent: it stays
private, its three members are the three the pane reads, and a named struct would add a type declaration
for no reader outside the module.

## 2. Each amendment as applied, and what the tree did not bear out

| # | Artifact | Acting owner | Date | State |
|---|---|---|---|---|
| 1 | `SPEC-MOK-003` rule 9 item 2 | repository owner as technical owner | 2026-08-21 | approved as written |
| 2 | `SPEC-MOK-003` rule 10, preamble and items 6 and 7 | repository owner as technical owner | 2026-08-21 | approved, **with one provision the owner was not shown** |
| 3 | `SPEC-MOK-004` rules 9, 10 and 11 | repository owner as technical owner | 2026-08-21 | approved; every figure in it measured |
| 4 | `VER-MOK-005`, two rows amended and two added | repository owner as assurance owner | 2026-08-21 | approved, **with one correction the act does not cover** |
| 5 | `SPEC-MOK-004` rule 6 | — | 2026-08-21 | **OUTSTANDING.** Not in the four the owner approved. |

Three things the amendment texts or the work order foretold that the tree did not bear out:

1. **The one-line death line.** In-scope item 3 says the death branch "appends `fear` to the `final
   values` line". It was implemented that way first and the added frame case failed. The line is 45
   columns against an interior of 42, and the paragraph carries `Wrap { trim: false }`, so it *wraps*
   rather than truncates: `fear` ends one row as a label with no value and `90` begins the next as a
   value with no label — the exact appearance rule 10.7 exists to prevent. The two-line pairing follows
   rule 4 clause 5's existing `REQ-MOK-047` arrangement and moves no figure in rule 5. **The owner was
   not shown it**, because it was not yet known to be needed. `inspector.md` and `inspector-one-line.txt`
   are the measurement, the withdrawn form captured through the same oracle into the same panes so the
   choice rests on evidence rather than on argument.
2. **Rule 6's figures.** The *Constraints* section said the three figures stand and are "to be measured
   and not assumed". They were measured, and two of the three move. This is amendment 5, and
   `interface.md` records it. The constraint is left as written rather than edited to match the outcome.
3. **`VER-MOK-005`'s stale `name`.** The *Absent attributes are absent* row also listed `name` among the
   fields that must not appear, which `SPEC-MOK-003` rule 10 as amended on 2026-08-19 under `REQ-MOK-041`
   moved into the presented-value list. The row has contradicted an approved specification since that
   date. It is corrected as a statement of fact about another artifact rather than as a change of
   obligation, and it is reported here rather than absorbed. **It is not this work order's defect**, and
   the owner's act does not cover it as approved scope.

## 3. The test census

| Tree | Engine | Observer | Workspace |
|---|---|---|---|
| `f82cd3d` — the tree rules 9 to 11 stated | 85 | 141 | **226** |
| `f2a79e1` — this branch's HEAD before the change | 122 | 142 | **264** |
| the implementing tree | 122 | 145 | **267** |

**41 arrivals, 0 departures, 1 rename.** 38 of the arrivals are `WO-MOK-016`'s and 3 are this work
order's; `226 + 41 = 267`. **None departed.** The rename —
`no_shipped_decision_source_has_a_proposal_rejected` to
`no_source_confined_to_the_valid_action_list_has_a_proposal_rejected`, `WO-MOK-016`'s — is in neither
figure, under `SPEC-MOK-004` rule 12.

The observer's 145 cross-checks as rule 10's internal 42 plus rule 9's public 103, and the workspace's
267 as 82 + 40 + 42 + 103. `#[ignore]` appears 0 times, so the static count and the executed count are
the same 267. `f2a79e1`'s 264 agrees with `VREC-MOK-017`'s figure at its own commit, so the fourth
stop-and-escalate condition is not tripped.

This work order's three tests and their tiers:

| Test | Target | Tier |
|---|---|---|
| `a_death_carries_the_fear_the_engine_last_reported_for_its_subject` | `tests/state.rs` | public |
| `the_inspector_presents_a_dead_subject_s_final_fear` | `tests/verification.rs` | public |
| `a_death_carries_no_attribute_the_engine_never_reported_for_its_subject` | `src/state.rs` | internal |

The third is internal because the state it needs — a death for a subject no `survival_changed` record was
ever seen for — exists in no run and is constructible only through the private `ingest`. The two ways to
move it to the public tier are both `ARCH-MOK-002` prohibited patterns, named there in those words:
widening an item to reach it from a test, and adding a fifth `#[cfg(test)]` hook. The placement costs the
case no reach: `render::draw` and `layout::resolve` are already `pub` and `select_for_test` is one of the
four existing hooks, so the absence is asserted at the **rendered pane** from inside the crate and not
only at the derived value.

## 4. Rule 6's three interface figures, measured

| Figure | Before, at `f2a79e1` | After |
|---|---|---|
| public items | 94 | **94** |
| public fields | 24 | **25** |
| `pub` lines | 118 | **119** |

The before column is read off the tree at `f2a79e1` and reproduces rule 6's stated figures and its
per-module table row by row, so the after figure is a correction of one cell. The whole of the movement is
one line, `+    pub fear: Option<u8>,`. `94 + 25 = 119` by construction. The four `#[cfg(test)]` hooks are
4 before and 4 after. `interface.md` is the enumeration.

## 5. The gate commands and their exit codes

Every command in the form *Required verification* item 6 names, run on the implementing tree with both
temporary oracles removed. `se_harness` from the pinned `0.4.0` venv.

| Command | Exit |
|---|---|
| `cargo test --workspace` | 0 — 267 passed, 0 failed, 0 ignored |
| `cargo fmt --all -- --check` | 0 |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | 0 |
| `cargo tree -p Mokiterions -e normal --locked --offline` | 0 |
| `python scripts/validate_engineering_artifacts.py` | 0 — 133 artifacts, 0 errors, 0 warnings |
| `bash scripts/check_engineering_harness.sh` | 0 |
| `python scripts/check_declared_dependencies.py` | 0 |
| `python scripts/generate_harness_dashboard.py --root .` | 0 — 133 artifacts, 483 relations, 0 errors, 15 warnings |
| `python scripts/inspect_engineering_artifacts.py` | 0 |
| `se_harness validate` | 0 |
| `se_harness preflight --work-order WO-MOK-018 --phase review` | 0 |

Two notes, both in `gates.txt`:

- **The preflight form differs from the work order's.** Item 6 names `preflight --phase review`; the
  command requires `--work-order`, and without it exits 2 on an argument error rather than on a finding.
  The form actually run is recorded.
- **The dashboard's 15 warnings are the branch's state and not a regression.** They are `W-HEX-001` and
  `W-HEX-003` observations on earlier artifacts plus the two `draft` definitions `WO-MOK-008` and
  `WO-MOK-017`, none of which this work order selects. The managed workflow's depth-1 checkout inflates
  the figure in CI, which `VREC-MOK-017` records and *Out of scope* keeps out of this work order.

## 6. Every consequence derived rather than decided

Named individually, with whether an act already taken covers it:

| Consequence | Covered? |
|---|---|
| the corrected figure `fourteen` | covered — it is `SPEC-MOK-001`'s measurement, and amendment 1 is approved |
| every figure in the `SPEC-MOK-004` rules 9, 10 and 11 row | covered — amendment 3 is approved and in-scope item 5 names the corrections; each figure is `cargo test`'s output |
| `WO-MOK-016`'s owed rule 11 correction, discharged here | covered — amendment 3 states it as forced by rule 11's own text, on the 2026-08-19 row's precedent |
| the internal-tier placement of the absence case | covered — the envelope grants the tier under rule 8, "reporting the placement and its ground" |
| the tuple rather than a named struct for `latest_survival` | covered — the envelope grants it |
| the wording and spacing of the `fear` segment | covered — the envelope grants it within amendment 2's order and item 7's rule |
| **rule 10.6's two-line pairing** | **awaiting ratification.** A provision inside an approved amendment that the owner was not shown; the wrapping defect it answers was found after approval |
| **`SPEC-MOK-004` rule 6's figures — amendment 5** | **awaiting ratification.** Not among the four approved; the work order asserted the opposite |
| **`VER-MOK-005`'s `name` correction** | **awaiting ratification.** A correction of a false statement about another artifact, outside the approved scope, and not this work order's defect |

One further correction was made to text this agent had itself written earlier in the same
implementation, and is recorded so it is not read as approved arithmetic: **rule 11's arrival prose said
"the forty arrivals" and "39 arrivals against 1 departure", while its own table sums to 41.** The measured
truth is 41 arrivals, 0 departures and 1 rename in neither figure. Both numbers are corrected in the rule.
The error was subtracting the rename as though it were a departure.

## 7. Findings carried rather than closed

| Finding | Who inherits it |
|---|---|
| Rule 10.6's "a pair carrying neither value emits no line at all" is not measured by any case. The death branch returns with that line last, so a suppressed line and the pane's unwritten rows are the same cells; and counting the lines instead needs the private `inspector_lines` and the private `ingest`, which are in **sibling** modules, so no test module is a descendant of both. Closing it costs a prohibited pattern. | disclosed as a residual in `VER-MOK-005`; carried, not owed to a work order |
| A figure restated in prose from a derived value goes stale silently — this defect's own shape. Rule 9 item 2 held a count of the engine's vocabulary and `CAP-MOK-010` moved the vocabulary. | carried in `filter-vocabulary.md`; no artifact currently obliges a sweep for restated figures |
| `fear` is still not presented for a **living** selected subject. Item 7's reachability reasoning holds while the subject lives, so this is a decision and not a defect. | *Out of scope*; a later technical-owner decision |
| The cause of death, the encounter tallies, the direct filter jump to the three new types, and any canvas indication of an engagement. | **Tier 3**, the next work order, every item named in this work order's *Out of scope* |
| `cycle_type_filter` now needs up to fifteen presses of `e`. A rule 7 key-binding question that correcting rule 9 item 2 neither creates nor answers. | Tier 3 |
| `WO-MOK-008` and `WO-MOK-017` remain `draft`, and the dashboard counts both. | not this work order's; `WO-MOK-017` carries `REQ-MOK-060`'s composition drift |

## 8. What this work order does not claim

- **It does not claim verification.** Commit-bound verification is classified `required`, and no `VREC`
  is written here. Verification and release require separate commit-bound records, and a record binds a
  branch commit and never `master`'s merge.
- **It does not claim the three outstanding provisions are approved.** Each is marked in the artifact
  that carries it and named in point 6 above.
- **It does not claim `implemented`.** The status stays `in_progress`; the transition is the owner's.
- **It does not claim the pane presents everything a reader might expect at a death.** No cause of death,
  no engagement tally, no suffered-attack record — rule 10.7's re-check places those elsewhere and *Out
  of scope* keeps them there.
- **It does not claim non-perturbation beyond the declared seeds.** Five seeds at two depths, identical
  in every authoritative record and in final state; a perturbation reachable only at an undeclared seed
  is outside what was measured.
- **It does not claim rule 6's figures are correct beyond the observer package.** `SPEC-MOK-002` rule 5
  closes the engine's interface and is untouched here, as is every file of `mokiterions-core`.
- **It does not claim the dashboard's warning count is clean.** Fifteen warnings stand, each attributed.
- **It has not been pushed, and no pull request is opened.** The change is committed on
  `feature/observer-fear-and-filter-count` and stops there.
