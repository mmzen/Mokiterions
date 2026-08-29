+++
id = "SPEC-MOK-004"
type = "specification"
title = "Package directories, observer targets, and observer test placement"
status = "approved"
owners = ["technical owner"]
created = "2026-08-18"
updated = "2026-08-24"

[relations]
specifies = ["REQ-MOK-028", "REQ-MOK-029", "REQ-MOK-030"]
+++

# Specification: Package directories, observer targets, and observer test placement

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-18 | Original content for `REQ-MOK-028`, `REQ-MOK-029` and `REQ-MOK-030`. | Approved by the technical owner; implemented under `WO-MOK-006` and verified under `VREC-MOK-006`. |
| 2026-08-19 | Recorded figures and two subject lines corrected, because `SPEC-MOK-003` rule 5 as amended the same day replaced the observer's four-row layout-tier table with one threshold per pane, and the implementation that conforms to it removes the `layout` module's `Tier` enum, that enum's `label` method, the `tier_for` function and the `Panes::tier` field. **Rule 6**: the recorded extent falls from 13 `layout` items to **10** and from **97** items to **94**; the same interface counted the other way falls from 122 to **118**, and its public fields from 25 to **24**. What the field figure counts is now stated, because 25 is reproducible only as 122 − 97 — it counts public fields, and the variants of a public enum are not written `pub` and were never in it. The `layout` row's subject, "viewport tiers and the pane geometry of each", is corrected: there are no tiers to name. A **Reduction** clause is added, symmetric with the existing **Growth** clause, so that removal is governed the way addition already was. The byte-identity check is scoped to the `WO-MOK-006` restructuring it was written for, exactly as `SPEC-MOK-002` rule 3's freeze was scoped to `WO-MOK-003`; unscoped it forbids the observer's code from ever changing again, which was never its subject and is not what rule 6 is for. **Rule 9**: `tests/layout.rs` rises from 7 tests to **10** and the public tier from 77 to **80**. The +3 is a net figure and the rule now carries the measured composition: three tests go, because they assert the tier table and the per-tier minimums that rule 5 no longer defines, and six arrive — the rename of the third departure, the per-pane threshold case, the ten-row log case and the monotonicity sweep that `VER-MOK-005` as amended requires, and two one-to-one threshold cases that its **Mapping injectivity** property needs once rule 5 creates a second placement regime. **Rule 11**: the observer's executed total rises from 109 to **112** and the workspace's from 169 to **172**, with the "same before and after" clause scoped to the restructuring. **Rule 12**: its second paragraph is scoped to the `WO-MOK-006` restructuring, exactly as rule 13's clause is and for the same reason — unscoped it forbids any later work order from renaming a test in either package, which is a freeze on test maintenance the rule was not written to impose, and it would make `excluded_panes_are_the_ones_the_tier_omits` unrenamable after the term "tier" was deleted from the specification it cites. The assertion itself is unchanged; only the name and the rule it names change. **Rule 13** and the second worked example no longer say "layout tier". No rule changes what it requires. No target, target name, path, package name, tier boundary, hook or prohibition changes, and no item's visibility widens: the interface only loses items, so rule 7 is satisfied by construction. | **Ratified 2026-08-20 by the repository owner acting as technical owner**, all five provisions as written and without modification, in the assessment review recorded under `WO-MOK-012`. It was **OUTSTANDING** from 2026-08-19 until that act. The repository owner approved the `SPEC-MOK-003` rule 5 amendment that authorizes the removal and directed the implementation in the same act on 2026-08-19, but was not shown this consequence: the implementation agent found it after the fact, by measuring the interface against this rule, and reported it rather than treating the rule 5 approval as covering it. That report is what this ratification answers, and the owner was shown the five provisions as consequences forced by a removal already approved rather than as fresh decisions — every figure in the row is a measured outcome, and the two provisions that are not figures, the **Reduction** clause and the scoping of the byte-identity check, were each ratified on the reasoning stated in the row: that removal should be governed the way addition already was, and that an unscoped byte-identity check forbids the observer's code from ever changing again, which was never its subject. Records bound to commits are not re-opened — `VREC-MOK-006` measured 97 items and 169 tests and both were correct at its commit. The implementation agent wrote this text and decided none of the substance. |
| 2026-08-19 | **Recorded test-count figures corrected for `WO-MOK-010` and for `master`'s `WO-MOK-007`, both of which added tests without correcting them.** Rule 11 states the obligation this row discharges: "a work order that adds a test corrects these figures here, and one that loses a test has a defect". The figures are measured on the merge of `master` at `7a2b502` into this branch, which is the first tree in which both sets of additions exist; neither work order's figures are statable without the other's, and correcting only one would leave the rule stating a number no tree runs. **Rule 9**: `tests/options.rs` rises from 7 to **8** and `tests/render.rs` from 8 to **12**, so the public tier rises from 80 to **85**. Five tests arrive, three from this work order and two from `WO-MOK-007`, each named below with the obligation it carries, and none departs. **Rule 10**: `mokiterions-tui/src/render.rs` rises from 12 tests to **17** and the internal tier from 32 to **37**, all five arrivals `WO-MOK-007`'s bands. The same row's item count rises from 39 private items to **47** — 30 functions and 17 constants — because `WO-MOK-007` adds one function and five constants and this work order adds two functions, none of them public. The five internal tests that use a hook are still the same five: neither work order adds one that does. **Rule 11**: the observer's executed total rises from 112 to **122**, the engine's from 60 to **78**, and the workspace's from 172 to **200**. Of the 28 the workspace gains, 21 are `WO-MOK-010`'s — 18 engine and 3 observer — and 7 are `WO-MOK-007`'s, 5 internal and 2 public. The engine's 18 are 13 internal and 5 public under `SPEC-MOK-002` rules 5 and 7, which state no figure of their own, and the split is recorded here only because the workspace total is stated here and is otherwise not reproducible. Nothing else changes — no target, target name, path, package name, tier boundary, hook or prohibition, and no item's visibility. The interface of rule 6 is untouched at 94, measured rather than assumed: neither work order adds a public item to the observer, and `WO-MOK-007`'s band constants and its `band` function are private to `render.rs`. | **Ratified 2026-08-19 by the repository owner acting as technical owner**, in the closing review of `WO-MOK-010` recorded in `evidence/WO-MOK-010/closing-review.md`, on the reading that neither half of the correction is statable without the other. It was **OUTSTANDING** until that act. This is a defect in two work orders' conformance rather than in this specification, and the second of them is not this branch's to answer for: `WO-MOK-007` reached `master` with seven tests added and rules 9, 10 and 11 left as they were. The implementation agent found both by measuring the merged tree against this rule, wrote this text, and decides none of it. Records bound to commits are not re-opened: `VREC-MOK-006` measured 97 items and 169 tests and both were correct at its commit, and `VREC-MOK-007` is verified at `dfab77b` against the tree it was taken on. `VREC-MOK-010` measured 190 and is bound to a commit that predates this correction and the merge; it is re-captured, not edited. |
| 2026-08-19 | **Rule 11's pointer to `WO-MOK-010`'s census corrected to the recapture the row above foretold.** That row closes "`VREC-MOK-010` measured 190 and is bound to a commit that predates this correction and the merge; it is re-captured, not edited", and the same was true of the census itself: rule 11's last paragraph said it "was captured at `4f32a9f` and reaches 190 rather than 200; that census is a capture and is re-taken against the merge rather than edited". It has since been re-taken, from a clean worktree at `master`'s tip `7a2b502`, and reads **179 before, 200 after** with `master`'s ten arrivals on its before side. The paragraph now says so, and keeps the superseded 190 on the record rather than deleting it. **No provision of this specification is added, removed or reworded, and no figure changes**: the observer's 122, the engine's 78 and the workspace's 200 are the row above's and are unmoved, as are the 21 additions and 0 removals; only the sentence saying where and against which tree they are reconciled is brought up to date. | Recorded by the implementation agent as a statement of fact about retained evidence, under rule 11's own instruction that a work order which adds a test corrects these figures here. It carries no ratification of its own because it changes nothing that requires one; the correction it points at is the row above, which was **OUTSTANDING** when this row was written and which the technical owner ratified on 2026-08-19 in the closing review of `WO-MOK-010`. |
| 2026-08-19 | **Recorded test-count figures corrected for `WO-MOK-011`, which adds twelve tests for `REQ-MOK-040` and `REQ-MOK-041`.** Rule 11 states the obligation this row discharges: "a work order that adds a test corrects these figures here, and one that loses a test has a defect". **Rule 9**: `tests/verification.rs` rises from 16 to **19**, so the public tier rises from 85 to **88**; the three arrivals are named with the obligation each carries, none departs, and the file's subject line is deliberately not extended, because the name is a presented value from the moment `SPEC-MOK-003` rule 10 as amended lists it as one. It is also recorded there that all three read the expected name from the engine's own records through the already-public `Observer::events`, which is what keeps them in this tier. **Rule 10**: `src/render.rs` rises from 17 to **18**, `src/verification.rs` from 8 to **9**, and the internal tier from 37 to **39**; the two arrivals are the entry's column arithmetic asserted through the private `entry_lines`, and `REQ-MOK-041`'s provenance claim in its negative form, which needs a subject the observer holds and was never told the name of and therefore needs the state hook. `src/render.rs`'s private and public item counts are unchanged, because `entry_lines` gains a parameter and no item is added or removed, and `src/verification.rs` still declares no item at all. **Rule 11**: the observer's executed total rises from 122 to **127**, the engine's from 78 to **85**, and the workspace's from 200 to **212**, with the twelve arrivals attributed to the five targets that run them and the engine's 54-internal, 31-public split recorded because 212 is otherwise not reproducible. `mokiterions-core/tests/naming.rs` is recorded as a new engine public-tier target, admitted by `SPEC-MOK-002` rule 8's closing sentence without obliging that rule's initial-arrangement table, which `tests/decisions.rs` already joined the same way. **Rule 6 is unchanged at 94 items, 118 `pub` lines and 24 public fields**, measured rather than assumed: `Observer::name_of` is `pub(crate)` and `spatial::agent_glyph` keeps the `&str` parameter and `char` return it already had, so this work order adds no member of the interface and changes no member's shape. Nothing else changes — no target name, path, package name, tier boundary, hook or prohibition, and no item's visibility widens. **Every figure in this row is measured on the merge of `master` at `2157f77`, which is the first tree in which both this work order's tests and `master`'s survival bands exist**; the row as approved carried the figures of the unmerged branch — 86 in rule 9, 34 in rule 10 and 205 in rule 11 — and those describe a tree that no longer exists. The merge adds no test and removes none: it gives six inherited internal-tier call sites in `src/render.rs` and one public-tier row locator in `tests/render.rs` the name argument, corrects one expected string for the six-column field, and adds `NAME_COLUMNS` to `tests/render.rs`, so `tests/render.rs` stays at 12 and `src/render.rs`'s five band tests stay in rule 10's table. | Approved 2026-08-19 by the repository owner acting as technical owner, together with `INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`, `REQ-MOK-041`, `VER-MOK-011`, `WO-MOK-011` and the same-day amendments to `SPEC-MOK-001` and `SPEC-MOK-003`. Every figure in this row is a measured outcome rather than a decision: the counts are `cargo test`'s per-target output and the interface figures are the enumeration retained as `WO-MOK-011` evidence. The two decisions the figures reflect — that the observer's name accessor stays `pub(crate)`, and that the entry test is placed in the internal tier rather than reached by widening a public item — are the technical owner's of the same date and are recorded in `WO-MOK-011`. The implementation agent measured, wrote this text and decided none of it. **No row above is touched by this one**, and no record bound to a commit is re-opened. **The figures were re-measured when this branch merged `master` at `2157f77`, after that approval.** The approval covers the amendment's substance, and every figure in it is a measured outcome rather than a decision — the counts are `cargo test`'s per-target output on the merged tree and the interface figures are `analysis/interface.py`'s enumeration of it — but the numbers are this merge's measurement and not the ones the owner read. The re-measurement is recorded in `evidence/WO-MOK-011/merge/`. |
| 2026-08-20 | **Recorded test-count figures corrected for `WO-MOK-013`, which adds fourteen tests for `REQ-MOK-047`, `REQ-MOK-048` and `REQ-MOK-049`.** Rule 11 states the obligation this row discharges: "a work order that adds a test corrects these figures here, and one that loses a test has a defect". **Rule 9**: `tests/render.rs` rises from 12 to **22**, `tests/layout.rs` from 10 to **11** and `tests/verification.rs` from 19 to **20**, so the public tier rises from 88 to **100**. The twelve arrivals are named with the obligation each carries, and none departs. It is also recorded there that `the_log_is_ten_rows_only_where_both_thresholds_are_met` in `tests/layout.rs` was **renamed and not removed**, to `the_log_is_six_rows_wherever_it_is_present`: the work order expected its subject to cease to exist and its removal to be reported here, and it was kept instead so that the withdrawn ten-row growth is asserted absent rather than left untested, which under rule 12 is a rename with its assertion strengthened and not a loss. That is why the file reads 11 and not 12. **Rule 10**: `src/render.rs` rises from 18 tests to **20** and the internal tier from 39 to **41**; the two arrivals are `REQ-MOK-047` over all 101 attribute values and the same property at every distinct bar width the roster's geometry can produce, both of which need the `#[cfg(test)]` snapshot hook to hold a value the run does not reach and cell-level reading no public entry point yields. **The same row's hook figure moves from 5 of 18 to 7 of 20**, because both arrivals reach the hook through the test module's own helper, which rule 8 treats as reaching it: the placement rule reads the access a test requires, not the line it is written on. The same row's private-item count rises from 47 to **48** — 30 functions and **18** constants — because two constants arrive for the permanent affordance's long and short forms, `BAR_ROW_OVERHEAD` changes value and not kind, and `announcement_text` gains a renamed parameter, so no function is added or removed and no item's visibility widens. **The 47 was itself one high**: at `a339902`, the commit this work order starts from, the module declares 30 private functions and 16 private constants, so 46. That is a pre-existing defect in this rule's figures, introduced by the 2026-08-19 `WO-MOK-010`/`WO-MOK-007` row and inconsistent with the `WO-MOK-011` row's own count of the same module as 48 total declarations, which is 46 private and 2 public and is the figure that was right. It is corrected to the measurement rather than absorbed into the new one, and it is reported as a finding of `WO-MOK-013` rather than left to be found again. **Rule 11**: the observer's executed total rises from 127 to **141**, the engine's 85 is unchanged, and the workspace's rises from 212 to **226**, with the fourteen arrivals attributed to the four targets that run them and the observer's total cross-checked as rule 10's 41 plus rule 9's 100. All fourteen are the observer's: this work order changes no file of `mokiterions-core`, so the engine's 54-internal, 31-public split is unmoved and the figures are measured on this branch's implementing tree rather than on a merge, with the two rows above governing should `master` add a test before the merge. **Rule 6 is unchanged at 94 items, 118 `pub` lines and 24 public fields**, re-measured rather than assumed and reproducing that rule's per-module table row by row: `BAR_ROW_OVERHEAD` is a private constant, the two hint constants are private, `bar_width` is a private function and the entry's row count is a local, so this work order adds no member of the interface and changes no member's shape. Nothing else changes — no target, target name, path, package name, tier boundary, hook or prohibition, and no item's visibility widens. | Covered by the approval of `WO-MOK-013` on 2026-08-20 by the repository owner acting as technical owner, which the work order records together with the requirement approvals and the `SPEC-MOK-003` amendments of the same date, and whose in-scope item 5 is "the `SPEC-MOK-004` figure corrections the added tests force". **Every figure in this row is a measured outcome rather than a decision**: the test counts are `cargo test`'s per-target output on the implementing tree, the item counts are an enumeration of `src/render.rs` retained as `WO-MOK-013` evidence, and rule 6's three figures are that rule's own counting rule applied to the seven `pub mod` files. The two placements the figures reflect — that both value-range cases stay in the internal tier rather than being reached by widening a public item, and that the log-height case is renamed rather than removed — are the implementation agent's under rule 8 and rule 12 respectively; the first is the placement rule applied, and the second is reported to the owner in this work order's completion report as a departure from what the work order foretold. The implementation agent measured, wrote this text and decided none of the substance. **No row above is touched by this one**, and no record bound to a commit is re-opened: `VREC-MOK-006` measured 97 items and 169 tests, `VREC-MOK-007`, `VREC-MOK-010` and `VREC-MOK-011` measured their own trees, and each was correct where it was taken. `VREC-MOK-013` is not written by this row and re-measures at its own commit. |
| 2026-08-20 | **Rule 2's `[workspace.dependencies]` prohibition reversed into the rule that governs a shared crate**, decided by `ADR-MOK-006`. An entry is admissible there when, and only when, the crate is a declared entry of **both** packages' sets, each member inheriting it with `workspace = true`; the table holds no entry today because neither declared set does. Every other table rule 2 declares none of stays — a virtual manifest has no targets and no dependencies of its own. **The *Counterexample* that rejected exactly this is replaced** by one that still bites: a `[workspace.dependencies]` entry for a crate that is not in both declared sets, which is how an undeclared dependency enters a workspace without either package's manifest naming it. Rule 3's two empty-table clauses — *"including the empty dependency table"* and *"the dependency and dev-dependency tables stay empty with no exception"* — take the declared-set form; **the second of those is a reach beyond the amendments `ADR-MOK-006` enumerated** and is disclosed here for that reason, as is rule 7's last bullet, rule 14's `cargo tree -p Mokiterions` row, and the *Compatibility and migration* bullet asserting `ARCH-MOK-001` needs no amendment. **Rule 7's bullet** is kept as the completed fact it was — the relocation introduced no table — and, as a standing rule, now admits an entry only as a declared entry under `ADR-MOK-006`, while **a build script in either of this repository's own packages stays prohibited outright** by `SPEC-MOK-002` rule 1. The two historical bullets are left standing as records of what the relocation did, with a dated note that a later ADR moved what they cite. **Re-measured at writing time, as `ADR-MOK-006` requires: no figure this specification counts moves.** Neither manifest is edited and no declared set gains an entry, so `cargo tree -p Mokiterions --locked --offline` still prints one crate, the observer still resolves the 57-crate `ratatui` graph plus both packages, the observer's `lib.rs` still declares seven `pub mod` items with `verification` still `#[cfg(test)]`, and every test-count figure rules 9 to 11 record is untouched by a prose-only change. | Approved 2026-08-20 by the repository owner acting as accountable technical owner, by way of `ADR-MOK-006`, whose *Required amendments* section states this amendment in full. Written under `WO-MOK-014`; the implementation agent wrote the text and re-took the measurements, and decided neither. It added no entry to any of the three manifests. The 2026-08-19 row that records a `SPEC-MOK-004` re-measurement after a merge is the precedent for the re-measurement clause above. |
| 2026-08-20 | **Recorded test-count figures corrected for `WO-MOK-019`, which adds thirty-four tests for `REQ-MOK-042` through `REQ-MOK-046`.** Rule 11 states the obligation this row discharges: "a work order that adds a test corrects these figures here, and one that loses a test has a defect". **Rule 11**: the workspace's executed total rises from 212 to **246**; the engine's from 85 to **119**, split 68 internal and 51 public because 246 is otherwise not reproducible; the observer's stays at **127**, because this work order adds a stream the observer does not read and no observer test is added, removed or moved. The thirty-four arrivals are attributed to the three targets that run them — fourteen in `mokiterions-core/src/simulation.rs`, three in `mokiterions-core/tests/cli.rs`, and seventeen in `mokiterions-core/tests/records.rs` — and none departs, which is measured by qualified name rather than by count: 34 additions and **0 removals** against the baseline log, so no rename can hide in the diff. `mokiterions-core/tests/records.rs` is recorded as a new engine public-tier target, admitted by `SPEC-MOK-002` rule 8's closing sentence in the same way `tests/naming.rs` and `tests/decisions.rs` were, without obliging that rule's initial-arrangement table. **Rules 9 and 10 are unchanged**, both being observer rules. **Rule 6 is unchanged at 94 items, 118 `pub` lines and 24 public fields**, measured at five revisions rather than assumed: the mechanical enumeration of `mokiterions-tui/src` is byte-identical to the one `WO-MOK-011` retained, and the mechanical figure reconciles with the recorded one by the same documented subtraction of `lib.rs`'s seven `pub mod` declarations. The engine's own interface grows by one parameter on `execute` and by no item; `SPEC-MOK-002` rules 4 and 5 as amended the same day are the authority for that, not this rule. Nothing else changes — no target name, path, package name, tier boundary, hook or prohibition, and no item's visibility widens. Every figure is measured on one `cargo test` invocation at the workspace root against the candidate tree, with every source touched first so that nothing reported a cached result. | **Approved 2026-08-20 by the repository owner acting as technical owner**, in a separate act after the candidate commit `50364a3` and after this cell was written. The confirmation the cell below asks for is the confirmation given: that `SPEC-MOK-004` belongs in this chain, and that no figure in this row is wrong — the workspace's executed total at **246**, the engine's at **119** split 68 internal and 51 public, the observer's unchanged at **127**, rules 9 and 10 unchanged as observer rules, and rule 6 unchanged at 94 items, 118 `pub` lines and 24 public fields. **The text below is what this cell said at the candidate commit and is kept unedited**, not because a superseded statement is worth preserving for its own sake, but because `evidence/WO-MOK-019/amendment-approvals.md` measures this cell's wording and that measurement was taken before this approval existed; editing the cell into agreement with a later decision is the thing this repository's evidence rule forbids. Read the two together: the row was unapproved when the packet measured it, and it is approved now. **The substance requires no owner decision and is not claimed as approved. Recorded rather than claimed: `SPEC-MOK-004` was not among the three amendments — `ARCH-MOK-001`, `SPEC-MOK-001` and `SPEC-MOK-002` — that the repository owner approved on 2026-08-20 with this chain, and `ADR-MOK-005`'s *Required amendments* section does not name it.** This row is written as the discharge of rule 11's own delegating clause, which obliges the correction of these figures by the work order that causes them and makes the figures measurements rather than decisions. What remains for the repository owner acting as technical owner is the confirmation that `SPEC-MOK-004` belongs in this chain at all and that no figure in this row is wrong; that confirmation is **OUTSTANDING** and is recorded as such in `evidence/WO-MOK-019/amendment-approvals.md`, alongside the eight manual assessments `VER-MOK-012` requires. The implementation agent measured, wrote this text and decided none of it. **No row above is touched by this one**, and no record bound to a commit is re-opened. The measurements are retained in `evidence/WO-MOK-019/gates.txt`, `evidence/WO-MOK-019/analysis/census-reconciliation.txt` and `evidence/WO-MOK-019/interface.txt`. |
| 2026-08-21 | **Rules 9, 10 and 11's test figures corrected for `WO-MOK-016` and `WO-MOK-018` in one act.** Rule 9's `tests/verification.rs` reads 22 rather than 20, `tests/state.rs` 22 rather than 21 and the total 103 rather than 100; rule 10's `src/state.rs` reads 5 rather than 4 and the total 42 rather than 41; rule 11 gains a paragraph reading the observer **145**, the engine **122** and the workspace **267**, against the 141, 85 and 226 it recorded for `WO-MOK-013`. Forty tests arrive across the two work orders and one name changes: `no_shipped_decision_source_has_a_proposal_rejected` becomes `no_source_confined_to_the_valid_action_list_has_a_proposal_rejected`, which rule 12 governs as a rename with its sweep unchanged, so the reconciliation is 39 arrivals against 1 departure and **no test is lost**. **The two work orders are corrected together because neither's figures are statable without the other's** — `WO-MOK-016` left all three rules uncorrected, so a correction naming only `WO-MOK-018` would state 267 while accounting for 3 of the 41 tests between them, and a reader could not tell which work order the rest belonged to. Rule 11's clause is what obliges this: *"a work order that adds a test corrects these figures here, and one that loses a test has a defect."* The engine's 37 are under `SPEC-MOK-002` rules 7 and 8 and are counted here only because 267 is otherwise not reproducible. No placement rule, tier boundary, target, invocation, or preservation obligation changes. | **Approved 2026-08-21 by the repository owner acting as technical owner**, who directed the implementation in the same act, as amendment 3 of `WO-MOK-018`. The joint form follows the 2026-08-19 row above, which corrected two work orders' figures together on the same ground. The implementation agent measured every figure by `cargo test --workspace` and by a name-by-name diff of both trees, wrote this text and decided none of the substance. `WO-MOK-016`'s 264 is independently corroborated by `VREC-MOK-017` at its own candidate commit, so the 38 attributed to it is not derived from this branch's tree alone. `VREC-MOK-017` is not edited: what it recorded was true of the tree it was bound to, and the uncorrected figures were already stale at that commit. |
| 2026-08-21 | **Rule 6's interface grows by one public field, the first growth since that rule was written.** `state::Death` gains `pub fear: Option<u8>` beside the `satiety` and `energy` it already declares, so the pair reads **25** public fields and **119** `pub` lines rather than 24 and 118. **The item count of 94 is unchanged and the table's every row is untouched**, including `state` at 47, because a field is part of the item that declares it under this rule's own counting. The authorizing requirement is `REQ-MOK-021` by way of `SPEC-MOK-003` rule 10.6 as amended on this date, which makes the fourth attribute one of the final attribute values the inspector presents for a dead subject; the **Growth** clause's "a test is never that requirement" is satisfied, since the tests follow the presentation rather than justify the field. Three alternatives are recorded as measured and worse: an accessor on `Observer` would add a public item and move 94; a `pub(crate)` field beside two `pub` siblings would make one struct partly opaque and relocate its public-tier case for no reason but visibility; and deriving the value in `render` is impossible, since the state is private and reaching it is what rule 7 forbids. Rule 7 is untouched — a field added to a struct whose fields are already public widens no existing item — and the byte-identity check is scoped to the `WO-MOK-006` restructuring and does not reach this commit. The coincidence with the superseded pair 25 and 122 is stated as a coincidence in the rule: 122 − 97 is still not how 119 is reached. | **OUTSTANDING for the technical owner's ratification as of 2026-08-21.** This is the implementation agent's text and the owner has **not** been shown it: it is recorded as amendment 5 of `WO-MOK-018`, which had four when the owner approved it. **The four the owner approved did not include this one, and the work order said in terms that rule 6's figures would be confirmed unmoved.** That prediction was wrong, and it was wrong because rule 6 counts public fields separately from items while the change was scoped by its effect on items. The error was found by measuring the figures rather than by asserting them, the baseline reproduced at exactly 118 before the change and 119 after, and `WO-MOK-018` records the correction against its own text rather than silently widening its amendment set. That prediction rested on a second error the work order states in its decision envelope, that a field on `Death` is "private to `mokiterions-tui::state`": `Death` is `pub` in a `pub mod`, and its three sibling fields are `pub`, so the envelope's grant of "the field name and type" rested on a false premise about what the field is. **The growth is nonetheless forced by the presentation the owner did approve**, and the alternative that avoids it adds a public *item* and moves 94, which `WO-MOK-018`'s first stop-and-escalate condition forbids outright — so the narrowest available form was implemented and is reported rather than the work being left undone. On the precedent of the 2026-08-19 row above, which was **OUTSTANDING** for the same reason until the owner ratified it under `WO-MOK-012`: the agent found the consequence after the fact by measuring the interface against this rule, and reports it rather than treating the work order's approval as covering it. The implementation agent measured the figures, wrote this text and decided none of the substance. |
| 2026-08-21 | **Rules 9, 10 and 11 re-measured on the second merge of `master`, at `7f4792a`, into this chain, and this chain renumbered from `WO-MOK-018` to `WO-MOK-019`.** Rule 11 reads the workspace **301**, the engine **156** and the observer **145**, against the 267 the row above records and the 298 this chain's own superseded draft row recorded for the first merge. **The figure is established by structure rather than by a census diff, which is what makes it checkable in one command each.** The merged tree's `mokiterions-tui/src` and `mokiterions-tui/tests` are **byte-identical to `master`'s** at `7f4792a`, and its `mokiterions-core/src` and `mokiterions-core/tests` are **byte-identical to this chain's** at `efe20e3`: `git diff origin/master -- mokiterions-tui` and `git diff efe20e3 -- mokiterions-core` are both empty. So the observer half's figures are `master`'s own and the engine half's are this chain's own, neither half is a merge of anything, and the workspace total is their sum — 145 + 156 = 301. It reconciles from both predecessors as well: 298 + 3, `master`'s three observer arrivals under its `WO-MOK-018`, and 267 + 34, this chain's engine additions. **Rules 9 and 10's tables therefore take the row above's approved figures unchanged** — the public tier at **103** with `tests/state.rs` and `tests/verification.rs` at 22 each, the internal tier at **42** with `src/state.rs` at 5 — not because this row re-decides them but because the half they count is `master`'s byte for byte, and they were re-measured at 22, 22, 103 and 42 rather than carried over. **Rule 10's private-item count of 49 is retained and its ground strengthened**: `src/render.rs` declares **31** private functions and **18** private constants, measured on the merged tree, on `master` at `7f4792a` and on this chain at `efe20e3` — the same figure on all three, so the 48 this rule recorded is one low on every tree that exists and not an artefact of either merge. **Rule 6 takes the row above's 94 items, 119 `pub` lines and 25 public fields**, which follows from the same byte-identity and is not independently re-derived here. The engine's **156** is 96 internal and 60 public under `SPEC-MOK-002` rules 7 and 8 rather than this rule, recorded because 301 is otherwise not reproducible. Nothing else changes — no target, target name, path, package name, tier boundary, hook or prohibition, and no item's visibility widens. Every figure is one `cargo test --workspace` invocation's per-target output on the merged tree, retained with the qualified-name census in `evidence/WO-MOK-019/merge/second/`. | **OUTSTANDING.** Drafted by the implementation agent for ratification by the repository owner acting as technical owner, under the correction procedure the owner set on 2026-08-21: the agent drafts each correction and the owner ratifies each. **Every figure in this row is a measured outcome rather than a decision**, and **no provision of this specification is added, removed or reworded.** Two things are put for ratification rather than the numbers. First, that this chain's own 2026-08-21 draft row is **replaced rather than kept**: it was OUTSTANDING, and it was drafted to answer the referral `WO-MOK-016` made about which work order re-derives rule 11's figures — a referral the row above has since answered on the owner's authority, under `master`'s `WO-MOK-018`. Keeping both would leave two unratified answers to a question already decided, so the draft is withdrawn and what survives of it is the one figure the approved row does not state, rule 10's 49. Second, that rule 10's 48 is a defect no record has reported: it is not on `VREC-MOK-016`, whose packet names the private `action_text` in a change-surface table and connects it to no item count, and it is not in the approved row above. **No row above is touched by this one**, and no record bound to a commit is re-opened. The implementation agent measured, wrote this text and decided none of it. |
| 2026-08-22 | **Rules 6, 9, 10 and 11's recorded figures re-measured at `WO-MOK-020`'s candidate commit, and one figure of another work order's corrected rather than absorbed.** **Rule 6 is unchanged and is recorded as re-measured**, at 94 items, 119 `pub` lines and 25 public fields with every module row unmoved: everything that work order adds to `src/state.rs` and `src/render.rs` is `pub(crate)` or private — the `Profile` record, the `ActionKind` enumeration, its `label`, the three `Observer` accessors, `Profile`'s five private counters and `src/render.rs`'s six private declarations — so the **Growth** clause is not invoked and no item's visibility is widened. **Rule 9** reads `tests/render.rs` 29 rather than 22, `tests/verification.rs` 29 rather than 22 and the total 117 rather than 103, with the fourteen arrivals named and their obligations stated, and with the scope of the two static checks that are scoped rather than global measured rather than assumed. **Rule 10** reads `src/state.rs` 21 rather than 5 and the total 58 rather than 42, with the sixteen arrivals named; its `src/render.rs` row's private-item count reads 55 rather than 49, being 34 functions and 21 constants, and the previous 49 is **confirmed** at the base commit under the same counting method rather than corrected; that row's test count stays 20 and its hook figure stays 7 of 20. **Rule 11** reads the observer 175, which is 117 public and 58 internal, and the workspace 332. **The engine reads 157 rather than 156, and the one test is not `WO-MOK-020`'s**: `mokiterions-core/src/simulation.rs` carries 97 internal-tier tests at this candidate against 96 on the tree the 2026-08-21 row measured, and the arrival is `WO-MOK-017`'s, added by commit `26ae6ba` implementing `REQ-MOK-060`. It is stated with its origin rather than absorbed into this work order's thirty, on the precedent of the row above that reported a figure as one high. That correction was owed from `WO-MOK-017`'s closure by this rule's own closing sentence and was not made then, so the owner is shown the engine's 157 here for the first time — a weaker position than a correction already reported, and stated as such. **No approved figure is contradicted**: each superseded figure is true of the tree it names, and none is edited. No rule's substance changes, no obligation on any test or item changes, and no row above is touched. | **Approved 2026-08-22 by the repository owner acting as accountable technical owner**, ratifying these figures as this rule's record and, separately, accepting the engine's 157 as `WO-MOK-017`'s uncorrected arrival rather than as a defect of this work order. The text was written on 2026-08-22 by the implementation agent under `WO-MOK-020` §4, which requires rule 6 and rule 9 re-measured **at the candidate commit and not projected** and one amendment row. Every figure above was measured on the candidate tree by `cargo test --workspace --locked` and by counting declarations per file, and the agent decided none of the substance: what a figure is, is measured rather than chosen. **These figures are true of `WO-MOK-020`'s candidate and are already known to move at the next merge, which is why they are ratified with a reconciliation owed rather than as this rule's settled state.** The owner was shown that measurement before ratifying. On 2026-08-22 the implementation agent trial-merged the `WO-MOK-008` branch, whose pull request changes `src/render.rs` and `tests/verification.rs` and amends this specification nowhere, and measured on the merged tree — green, with no test failing — `tests/verification.rs` at 31, rule 9's total at 119, rule 10's total at 66 with `src/render.rs`'s private items at 60, the observer at 185 and the workspace at 342. Ratifying at that merged tree instead was declined, because it would have closed this work order with its own specification figures unratified and would have ratified figures no retained evidence covers; leaving the merged figures unrecorded was declined too, since that branch amends this document nowhere and the staleness would be silent. **A reconciliation row re-measuring rules 9, 10 and 11 is therefore owed at that merge**, on the precedent of the two 2026-08-20 reconciliation rows of `SPEC-MOK-003` and of this table's own 2026-08-21 re-measurement row. The trial figures in this cell are a measurement of a worktree that was discarded and are not this rule's record: whoever merges second re-measures rather than copies them. No record bound to a commit is re-opened. |
| 2026-08-22 | **Rules 9 and 11's figures reconciled at the merge of `master` at `3ca2028`, which the row above records as owed. No rule's substance changes, no obligation on any test or item changes, and no row above is touched.** **Rule 9** reads `tests/options.rs` 9 rather than 8 and the total 118 rather than 117. **Rule 11** reads the observer 176, which is 118 public and 58 internal, and the workspace 333. The single arrival is `the_shared_entries_are_the_engines_own_words` in `tests/options.rs`, and **it is not `WO-MOK-020`'s**: it comes from `master`'s usage-text work, which amends this specification nowhere, and it is stated with its origin rather than absorbed, on the precedent of the engine's 157 in the row above. **Rules 6 and 10 are re-measured unmoved** — 94 items, 119 `pub` lines and 25 public fields with every module row unchanged, and an internal total of 58 with every row unchanged — and **the engine is unmoved at 157**, 97 internal and 60 public, measured rather than inferred: no `#[test]` attribute and no `fn` declaration is added or removed in either `mokiterions-core` file the merge touches. **No approved figure is contradicted**: the 117, the 175 and the 332 are true of `WO-MOK-020`'s candidate commit `f633eda`, the paragraphs recording them are kept as figures of that tree, and `VREC-MOK-023`, which binds that commit and is `verified`, is not re-opened. | **Approved 2026-08-22 by the repository owner acting as accountable technical owner**, in the same instruction that directed the merge. The owner was asked who should resolve this branch's one conflict with `master` and chose that the merge be taken and this row written; the question stated beforehand that the suite was expected at 333 and rule 9 at 118, from a trial merge of 2026-08-22 in a worktree since discarded. Both were then measured on the real merge rather than assumed, and both are what the question predicted. The instrument is `cargo test --workspace --locked` with `docs/engineering/simulation/evidence/WO-MOK-020/09-measure-spec-mok-004.py` run over its `--list` output, which is the same instrument the row above used, and the four tier figures are independently reproduced by counting `#[test]` per file. That script is a file of a packet a `verified` record binds; it was executed and not edited. The implementation agent wrote this text and decided none of the substance: what a figure is, is measured rather than chosen. **This row discharges the reconciliation the row above leaves owed and no other obligation**: the trial figures that cell records against the `wo-mok-008-footer-shedding` branch — rule 9 at 119, rule 10 at 66 with `src/render.rs`'s private items at 60, the observer at 185 and the workspace at 342 — remain a measurement of a discarded worktree, and whoever merges that branch re-measures rather than copies them. |
| 2026-08-24 | **Rule 11 admits `#[ignore]` for instruments, and rules 9, 10 and 11's figures are corrected for `WO-MOK-025`.** **Rule 11's opening** loses `#[ignore]` from the list of things no tier requires and gains the class that replaces it: an **instrument** is a `#[test]` function that asserts nothing and exists to produce something a human reads or commits, three exist and all three are named in the rule by file, and no test that asserts anything carries the attribute. The obligation the opening exists for is therefore undisturbed — every tier's assertions still run on a bare `cargo test` — and the attribute is what keeps that true rather than what weakens it: without it the suite prints roughly 20,000 lines of capture output and one member rewrites a committed file as a side effect, which would make the suite's verdict depend on the order its tests ran in. This rule's figures consequently carry **two** totals, the tests that exist and the tests that execute. **Rule 9** reads `tests/authority.rs` **5** rather than 4, `tests/options.rs` **14** rather than 9, `tests/verification.rs` **31** rather than 29, `tests/replay.rs` arriving with **4**, and the total **130** rather than 118; ten of the twelve are this work order's and two are `master`'s, so the baseline at the merge base `2a93914` is 120 and not the recorded 118, and each of the ten is named with the obligation it carries. `mokiterions-tui/tests/replay.rs` is a **new** public-tier target, admitted by that rule's own "A further file may be added when a further public subject appears", so **the observer's public-tier target count moves from eight to nine for the first time since this specification was written**, and every sentence stating it as eight is superseded and kept. **Rule 6's interface grows by one item and one public field, and neither is any test's**: `state::Observer::with_port` is a public item, so the `state` row reads **48** rather than 47 and the total **95** rather than 94; `options::Options::transcript_path` is a public field, so the pair reads **26** and **121** rather than 25 and 119 while the `options` row stays at 8, a field being part of the item that declares it. `SPEC-MOK-007` rule 20.4.1 is the requirement behind the first and rule 18.4 behind the second, and both are forced by this specification's own rule 4: `mokiterions-tui/src/main.rs` is a separate crate from the library, it is the target that opens the transcript because rule 12.1.1 puts the opening in the host and rule 19.7 forbids an engine message from naming a path the engine resolved, and `src/main.rs:134` is the caller that makes the constructor `pub`. Three alternatives are recorded in rule 6 as measured and worse. Rule 7 is untouched, since both arrivals are new declarations and no existing item's visibility widens. The other five module rows and `authority`'s 5 items are re-measured unchanged. **Rule 10** reads `src/render.rs` **29** rather than 20 and the total **67** rather than 58, with the hook figure moving from 7 of 20 to **8 of 29** and the private-item count from 55 to **60**, being 36 functions and 24 constants. **Only one of the nine tests and none of the five declarations is this work order's**: measured at three revisions, `master` at `3ca2028` carries 20 tests with 34 and 21 declarations, the merge base `2a93914` carries 28 with 36 and 24, and the candidate carries 29 with 36 and 24 — so the eight tests and all five declarations are `master`'s gauge and footer work, arriving through a merge this work order did not make, and the correction was owed from that merge. The one arrival is named and is a hook user. The other three rows are re-measured unmoved at 9, 21 and 8. **Rule 11** reads the observer **197**, the engine **221** and the workspace **418**, of which **415** execute; the observer's before-figure is **186** and not the recorded 176, the ten between them being `master`'s two in `tests/verification.rs` and eight in `src/render.rs`, and the engine is unmoved at 157 across that same span, measured rather than inferred. Seventy-five tests arrive, **none departs and none is renamed**: 64 engine and 11 observer, attributed to the eight targets that run them, so 343 + 75 = 418 from the merge base and 333 + 10 + 75 = 418 from the approved figure. The engine's 221 is 143 internal and 78 public under `SPEC-MOK-002` rules 7 and 8, recorded because 418 is otherwise not reproducible, and `mokiterions-core/tests/replay.rs` is a new engine public-tier target under that rule's closing sentence, taking its public-tier target count from eight to nine. All three instruments are the engine's, so it executes 218 of 221 and the observer executes all 197. **No approved figure is contradicted**: each superseded figure is true of the tree it names and none is edited. **What this row does not do.** It does not move **rule 1's layout or rule 2's manifest**: `ADR-MOK-007` states positively that decision 3 leaves both alone, this stage adds no directory and no workspace member, and the one connector this repository owns is a fixture inside an existing package. It does not add a fifth member to any census of decision sources, **because this specification carries none that is live** — measured rather than assumed: the four enumerations of sources in it are `WO-MOK-011`'s and `WO-MOK-016`'s historical arrival tables, `WO-MOK-020`'s row describing what `the_verb_totals_and_the_rejections_account_for_every_opportunity` asserts, which genuinely loops four sources in the code at this candidate, and rule 13's byte-identity clause, which the 2026-08-19 amendment scoped to the `WO-MOK-006` restructuring and which stood unamended through the third and fourth sources' arrival. Writing "five" into any of them would put a false statement in an approved specification. It amends no placement rule, tier boundary, target name, path, package name, hook or prohibition, and widens no item's visibility. | **Three approvals, and they cover different things; the third is OUTSTANDING.** The figure corrections are covered by `ADR-MOK-007`, approved 2026-08-23 by the repository owner acting as accountable technical owner with the words "i approve the artifact pack", whose *Required amendments* section names this specification and states its part as the census, rule 1's layout not moving, "**Rule 11's test-count figures** move for every stage that adds a test, by that rule's own delegating clause", and an amendment record row; that section's *Status* clause distributes its provisions across `WO-MOK-025`, `WO-MOK-026` and `WO-MOK-027`, and this row writes this stage's share. **The `#[ignore]` amendment is not in that ADR and was not approved by it.** It was escalated in this session under `WO-MOK-025`'s stop-and-escalate condition 6 — "An amendment turns out to be needed that `ADR-MOK-007` does not name. No approved artifact is amended on an implementation agent's judgement" — and the repository owner acting as accountable technical owner decided on **2026-08-24** to amend rule 11's opening to admit instruments, in preference to the two alternatives measured and put with it: dropping the attribute from all three, which makes the suite's verdict order-dependent and adds roughly 20,000 lines to its output, and reclassifying the transcript regenerator as a script outside the test corpus, which removes it from `cargo test`'s reach and so from the one command this rule exists to make sufficient. **Rule 6's growth is neither approval's and is OUTSTANDING for the technical owner's ratification as of 2026-08-24.** `ADR-MOK-007`'s *Required amendments* section names this specification's census, rule 1's layout and rule 11's figures, and it names **no** rule 6 amendment; its `mokiterions-tui/src/options.rs` and `state.rs` section describes both changes as source with no authority and does not state that either becomes public. The owner has not been shown this consequence. It is reported here rather than treated as covered, on the exact precedent of the 2026-08-21 row above, which was OUTSTANDING for the same reason and for the same cause: rule 6 counts public fields separately from items, and a change scoped by its effect on behaviour can move a figure nobody predicted. **The growth is nonetheless forced by work the owner did approve**, and it is the narrowest available form — rule 6 records the three alternatives, of which the only one that avoids a public item puts a library-resolved path inside the library and so breaks rule 19.7. What is put for ratification is the growth and the two figures, not the implementation: the item and the field are what decision 3 needs, and the agent chose neither. **Every other figure in this row is a measured outcome rather than a decision**: the test counts are `cargo test --workspace --locked`'s per-target output at the candidate, reproduced independently by counting `#[test]` per file, and the declaration, hook and interface counts are enumerations of the named modules at the three revisions the row states. The implementation agent measured, wrote this text and decided none of the substance. **No row above is touched by this one**, and no record bound to a commit is re-opened: the 118, the 58, the 176 and the 333 are true of the tree that merged `master` at `3ca2028`, and `VREC-MOK-023` binds `f633eda` and is not re-opened. **Rule 6's module table is edited rather than corrected beneath**, on the 2026-08-19 precedent that edited its `layout` row from 13 to 10 and kept the superseded figure in the prose above it; the 47 and the 94 are recorded in rule 6 as what the table read until this amendment. **Rules 9 and 10's tables are edited to the candidate's figures in the same way**, which is how both have been maintained since `WO-MOK-006` — rule 9's `authority.rs` to 5, `options.rs` to 14, `verification.rs` to 31, a `tests/replay.rs` row at 4 and the total to 130, and rule 10's `src/render.rs` row to 29 tests, 60 private items and 8 hook users with the total at 67 — and every superseded figure is kept in the correcting paragraph beneath each table, as every row above has done. Rule 11 states its figures only in paragraphs and has no table to edit. |
| 2026-08-24 | **Rules 9 and 11's figures re-measured at `bce4229`, `WO-MOK-025`'s candidate after four commits that close verification cases the row above did not cover.** No rule's substance changes, no obligation on any test, target or item changes, and no rule 6 figure moves: the four commits change no file under either package's `src/`, which is measured from the diff rather than inferred from an unchanged total, so there is no line on which a `pub` item could have arrived. **Rule 9's table is edited to the candidate's figures** — `tests/replay.rs` to **5** and the total to **131** — and its arrival table gains a twelfth row, `the_observer_replays_this_source_to_the_horizon_with_every_pane`, carrying `VER-MOK-018` case **L31**; the superseded 130 and four are kept in the correcting paragraph beneath, as every row above has done. **Rule 11 gains a paragraph and an arrivals table** rather than an edit, which is the only form it has ever been maintained in: the observer's total reads **198** and the engine's **227**, so the workspace's is **425**, of which **422** execute and the same three are instruments; the observer's 198 is rule 10's internal **67**, unmoved, and rule 9's public **131**, and the engine's 227 is `SPEC-MOK-002` rule 7's internal **143**, unmoved, and rule 8's public **84**. Seven tests arrive, none departs and none is renamed: five in `mokiterions-core/tests/replay.rs`, which drive the five cases `VER-MOK-018` states over the declared seed set or over both of rule 20.5's hosts at every seed and through both hosts rather than at one of each; one in `mokiterions-core/tests/no_outcome_threshold.rs`, a **new** public-tier target of the engine carrying case **L26**; and one in `mokiterions-tui/tests/replay.rs`. **The engine's public-tier target count moves from nine to ten**, under `SPEC-MOK-002` rule 8's closing sentence and on the same ground `tests/replay.rs` was admitted by the row above: rule 7 places a test by the access it requires, and this one requires none of the engine at all, since case L26 is an absence in the suite and is checked by reading the suite's own source as text. The observer's target counts are unmoved. Measured by `cargo test --workspace --locked`'s per-target output at `bce4229` and reproduced independently by counting `#[test]` per file, which agrees target by target. **No row above is touched by this one** and no record bound to a commit is re-opened: rule 9's **130**, rule 11's **197**, **221**, **418** and **415**, and the engine's public **78** are true of `d8ff20e`, the candidate the row above was written at, and this work order's candidate moved after them. Rule 10's **67** and rule 6's figures are not superseded, because nothing moved them. | Covered by `ADR-MOK-007`, approved 2026-08-23 by the repository owner acting as accountable technical owner with the words "i approve the artifact pack", whose *Required amendments* section names this specification and states that "**Rule 11's test-count figures** move for every stage that adds a test, by that rule's own delegating clause"; rule 11's own closing sentence is the delegation, and rule 9's table has been maintained the same way since `WO-MOK-006`. **Nothing in this row needs an approval the row above did not already have**, and nothing in it is OUTSTANDING: the `#[ignore]` amendment that row escalated is not touched, and rule 6's growth that row reports as OUTSTANDING is neither moved nor discharged here — it remains outstanding exactly as that row states it. The implementation agent measured, wrote this text and decided none of the substance. |
| 2026-08-29 | **Rules 9 and 11's figures re-measured at `WO-MOK-026`'s candidate `7f9e20a`, after twelve commits that add fifty-three tests and lose none; rule 11's test-binary count corrected for two arrivals, only one of which is this chain's; and rule 6 re-measured and found unmoved. Rule 1 does not move.** Rule 9's table is edited — `tests/options.rs` **14 to 18** and the total **131 to 135** — and the superseded figures are kept in a correcting paragraph beneath it, which is how this table has been maintained since `WO-MOK-006`; the four arrivals are enumerated there with the obligation each carries, all four being the observer's refusal of the live-run options. Rule 11 gains a paragraph and an arrivals table rather than an edit, having no table of its own. **Its figures at this candidate**: the observer's total **202** and the engine's **276**, so the workspace's **478**, of which **475** execute and **3** are `#[ignore]`d instruments — against the **198**, **227** and **425** the row above records. The observer's 202 decomposes as rule 10's internal **67**, unmoved, and rule 9's public **135**; the engine's 276 as `SPEC-MOK-002` rule 7's internal **174** and rule 8's public **102**, from **143** and **84**. **Neither unmoved figure is inferred from an unchanged total**: rule 10's 67 is re-measured file by file as 8, 29, 21 and 9, and every per-file `#[test]` count at merge base `d96cced` is identical to `bce4229`'s, which is what establishes that the whole fifty-three-test arrival is this chain's and that `main` moved after `bce4229` without adding a test. **Fifty-three arrive, none departs and none is renamed** — 49 the engine's and 4 the observer's — in `mokiterions-core/src/simulation.rs` **31**, `mokiterions-core/tests/connector.rs` **13**, `mokiterions-core/tests/cli.rs` **5** and `mokiterions-tui/tests/options.rs` **4**. **The engine's public-tier target count moves from ten to eleven** for `tests/connector.rs`, and the observer's is unmoved at **nine** public files and **three** internal ones. **The test-binary count reads twenty-five and only one of its two arrivals is this chain's**: fourteen the engine's and eleven the observer's, against the twenty-three the row above records as twelve and eleven. One is `tests/connector.rs`. The other is `canned-connector`, a second `[[bin]]` target of the engine package at `tests/support/canned_connector.rs`, which declares no test and so moves no test figure; it reached `main` at `1046f23` under this same work order's item 2, and **the correction was owed from that commit and was not made there** — its message reasons that "a `[[bin]]` adds no test, so `SPEC-MOK-004` rule 11's counts are conserved and need no correction here", which is true of every test figure and not of the binary count this rule states beside them. **`SPEC-MOK-002` rule 1 has not been amended for that target and still reads "exactly two targets" and "No third target"**; that conflict is recorded as a defect with this stage's evidence, at `evidence/WO-MOK-026/defect-third-target-unamended.md`, and is **not repaired here**. **Rule 6 is re-measured and not amended**: the interface holds at **95** items, **26** public fields and **121** `pub` lines, module row by module row — `authority` 5, `export` 3, `layout` 10 with 7 fields, `options` 8 with 5, `render` 2, `spatial` 19 with 6 and `state` 48 with 8 — measured at this candidate by the same enumeration that reproduces `bce4229`'s figures exactly. Three observer source files changed under this work order and none of them widens it: `options.rs` gains a **private** `const LIVE_RUN_OPTIONS`, `state.rs` gains a `Proposer` implementation for the **private** `state::LentPort` whose two methods are not `pub` declarations, and the refused list is read into the usage text through `include_str!` as text and not as an item. **Rule 1 does not move**, this work order saying so in terms; its stated ground — that "no directory is added" — is false at this candidate, `mokiterions-core/tests/support/` having arrived with the target above, and that is recorded in the same defect note rather than repeated here. **No figure above supersedes a figure the row above states**: its 425, 422, 198, 227, 131 and 84 are true of `bce4229`, and its 95, 26 and 121 are re-measured here as still true. | **Covered by `ADR-MOK-007`, approved 2026-08-23 by the repository owner acting as accountable technical owner with the words "i approve the artifact pack"**, whose *Required amendments* section names this specification and states that "**Rule 11's test-count figures** move for every stage that adds a test, by that rule's own delegating clause", and whose *Status* clause distributes its provisions across `WO-MOK-025`, `WO-MOK-026` and `WO-MOK-027`. This row writes this stage's share, discharging `WO-MOK-026` item 16's third clause — "of `SPEC-MOK-004` **rule 11**'s test-count figures for the tests this stage adds — every figure measured against the tree at the candidate commit and never inferred from an unchanged total" — and that item's closing instruction, "**`SPEC-MOK-004` rule 1 does not move**", is obeyed. Rule 9's table and rule 11's paragraphs have been maintained this way since `WO-MOK-006`. **Nothing in this row needs an approval an earlier row did not already have, and nothing in it is a decision**: every figure is `cargo test --workspace --locked`'s per-target output at the candidate, reproduced independently by counting `#[test]` per file, and rule 6's three figures are an enumeration of the named modules at three revisions whose method was validated first by reproducing `bce4229`'s recorded row-by-row figures exactly. **Two things are reported rather than treated as covered.** The third `[[bin]]` target contradicts `SPEC-MOK-002` rule 1, which is a change of substance in an approved rule that this work order's stop-and-escalate condition 6 forbids an implementation agent making on its own judgement, so it is recorded as a defect for the owner with three measured repair routes and none chosen; and rule 1's false ground is recorded in the same note, because the instruction not to move rule 1 is followed and the premise it rests on is not repeated as if true. **Rule 6's growth stays OUTSTANDING.** The 2026-08-24 row above reports that growth as outstanding for the technical owner's ratification, on the ground that `ADR-MOK-007` names no rule 6 amendment; this row neither moves nor discharges it — the interface is unmoved at that row's own three figures — and it remains outstanding exactly as that row states it. The `#[ignore]` amendment that row records is likewise untouched. **The frontmatter's `updated` field reads `2026-08-24` and this row is later**, and it is left as it stands: whether correcting it is a statement of fact an implementation agent may make or bookkeeping on an approved artifact it may not is a question already put to the repository owner in `SPEC-MOK-007`'s 2026-08-29 row, where the same staleness arises and where this repository's own rows read it two different ways, and it is not re-decided here. The implementation agent measured every figure, wrote this text and decided none of the substance. **No row above is touched by this one** and no record bound to a commit is re-opened: `VREC-MOK-023` and `VREC-MOK-024` bind earlier content of this specification, at `f633eda` and `b0c18b8`, and neither is edited. |

## Scope

This is the structural contract for the **repository's package layout** and for the **terminal observer package**:
where each package's manifest, sources and tests live; which targets the observer package builds; exactly what its
library target makes public; and where every automated test of that package lives.

It stands to the observer package as `SPEC-MOK-002` stands to the engine package, and it takes over the two subjects
`SPEC-MOK-003` withheld or delegated:

- `SPEC-MOK-003`'s *Explicitly unspecified decisions* withholds "the package layout", so no approved artifact fixes
  it. This specification fixes it, for both packages.
- The same section grants "test organization, fixtures and helpers" to the implementation agent. Rules 8 to 11 below
  narrow that grant for the observer package to file-internal organization and helper structure, exactly as
  `SPEC-MOK-002` rules 7 to 10 narrowed `SPEC-MOK-001`'s equivalent grant for the engine.

It states no simulation behavior and no presentation behavior. `SPEC-MOK-001` remains the single behavior contract
for the engine and `SPEC-MOK-003` for the observer, and rule 13 below binds this specification to preserving both
byte for byte.

It changes nothing about the engine package other than which directory holds it. The engine's targets, its target
names, its closed public interface and its two test tiers stay exactly as `SPEC-MOK-002` states them; every clause
of that specification that names a root-relative path is amended to the new directory and nothing else. Rule 3
below is the only rule here that touches the engine, and it moves files.

## Actors and external systems

- Cargo, which decides what a workspace member is, what a target is, what an integration test may link, and how a
  bare command resolves at a virtual workspace root.
- Clippy under `-D warnings`, whose `non_snake_case` lint constrains the observer's library target name exactly as
  it constrains the engine's.
- Implementation agents and developers, who locate a package's files, place tests, and maintain the observer's
  public interface.
- The operator, whose commands and whose output must not change.
- No external service, network endpoint, credential, or filesystem location outside the repository participates.

## Inputs

The repository root's `Cargo.toml`; each package's `Cargo.toml`; the engine's source files and the test files under
its `tests/` directory; the observer's ten source files and the test files under its `tests/` directory;
`Cargo.lock`; and the commands `cargo build`, `cargo test`, `cargo run`, `cargo tree`, `cargo fmt` and
`cargo clippy --all-targets --all-features -- -D warnings`, in both their workspace-wide and their `-p` forms.

## Outputs

Two library artifacts and two executable artifacts. Test binaries: one per package per internal-tier target, and one
per public-tier file. The observable output of both packages, which this specification requires to be unchanged.

## State model

This specification governs no runtime state. Its subject is the compile-time arrangement of the repository, which
has two states: conformant, when every rule below holds, and non-conformant, when any does not. There is no partial
or transitional state — a build either satisfies the rules or fails a conformance check.

## Behavioral rules

### 1. Repository layout

Each package's manifest, sources and tests are under one directory named for that package. The repository root
holds the workspace manifest, the lock file, the repository-level configuration and documentation, and no package.

```text
Cargo.toml                     # workspace manifest only; declares no package
Cargo.lock
mokiterions-core/              # the engine package, whose name stays `Mokiterions`
  Cargo.toml
  src/                         # lib.rs, main.rs, cli.rs, simulation.rs
  tests/                       # the engine's public tier, five files
mokiterions-tui/               # the observer package, whose name stays `mokiterions-tui`
  Cargo.toml
  src/                         # lib.rs, main.rs, and the eight module files
  tests/                       # the observer's public tier, eight files
```

`mokiterions-core` is a directory name and is not a package name. It is chosen so that the directory says which
package it holds; the package inside it is `Mokiterions`, unchanged. The directory is not named `Mokiterions`
because a directory named for the binary invites the belief that renaming one renames the other, and because an
earlier revision of `WO-MOK-005` that renamed the package itself was reverted precisely to keep the operator-facing
names still.

No third package directory, no nested workspace, and no directory holding the sources of more than one package.

### 2. Workspace manifest

The root `Cargo.toml` declares `[workspace]` with `members = ["mokiterions-core", "mokiterions-tui"]` and
`resolver = "3"`, and declares no `[package]`, `[lib]`, `[[bin]]` or `[dependencies]` table. The resolver is stated
explicitly because a virtual manifest does not inherit a member's edition default, and an unstated resolver is both a
behavioral difference and a build-time warning.

A `[workspace.dependencies]` table is the form a crate shared by both packages takes, and it is admissible only for
such a crate: an entry may appear there when, and only when, that crate is a declared entry of **both** packages' sets
— `SPEC-MOK-002` rule 13 for the engine and `SPEC-MOK-003`'s *Declared dependency set* for the observer — and each
member then inherits it with `workspace = true`. An entry that is not in both declared sets is a violation, whether it
is unused by one package or undeclared by it. The table holds no entry today, because neither declared set does.

**Amended 2026-08-20.** `[workspace.dependencies]` was in the list of tables this manifest declares none of.
`ADR-MOK-006` reverses that: a shared crate is now visible in one place instead of impossible, and a version keyed once
for both packages is the reason the table exists. Every other entry in the list stays — a virtual manifest has no
targets and no dependencies of its own, which is what makes it virtual, and that is unchanged.

A `default-members` or a member-level `default-run` key is permitted if, and only if, rule 14's command check shows
that a form the operator uses no longer resolves without it. It is then the only addition, and the resolution it
restores is stated in the manifest as a comment. A rename is never the correction.

`Cargo.lock` stays at the repository root. No dependency resolves to a different version as a result of this
layout; the lock file changes only where a recorded path changes.

### 3. Engine package relocation

The engine package's manifest, its `src/` directory and its `tests/` directory move under `mokiterions-core/`. The
`[package]`, `[lib]`, `[[bin]]` and `[dependencies]` tables move with the manifest unchanged, including the dependency
table — empty then and empty now — and the target paths `src/lib.rs` and `src/main.rs`, which are package-relative and
therefore already correct.

Every `SPEC-MOK-002` rule continues to bind the engine package with its paths read under `mokiterions-core/`. The
package name stays `Mokiterions`, the library target stays `mokiterions`, the binary target stays `Mokiterions`, the
dependency and dev-dependency tables hold exactly what `SPEC-MOK-002` rule 13 declares for this package and nothing
else, and rules 7 to 10 of that specification remain the engine's test-placement contract.

**Amended 2026-08-20.** Two clauses of this rule asserted the engine's tables were empty as a rule: *"including the
empty dependency table"* and *"the dependency and dev-dependency tables stay empty with no exception"*. `ADR-MOK-006`
withdrew that rule, so both are restated against `SPEC-MOK-002` rule 13, whose table is empty as this amendment lands.
Nothing about the relocation this rule governs changes, and no table in either manifest is edited: what moved
unchanged still moved unchanged, and the emptiness is now a measured fact rather than a requirement this rule restates. This specification states nothing about which tier an engine test is in.

The observer package's dependency on the engine becomes `Mokiterions = { path = "../mokiterions-core" }`. It stays a
path dependency, stays keyed by the engine's package name, and gains no feature and no version requirement. The
observer's `ratatui` dependency, its version and its feature set are unchanged; `SPEC-MOK-003` remains their
authority.

No file is renamed while it is moved, and no module is split, merged or reordered as part of the move.

### 4. Observer targets

The observer package name stays `mokiterions-tui`. It declares exactly two targets:

| Target | Kind | Name | Path |
|---|---|---|---|
| Library | `[lib]` | `mokiterions_tui` | `mokiterions-tui/src/lib.rs` |
| Binary | `[[bin]]` | `mokiterions-tui` | `mokiterions-tui/src/main.rs` |

No third target and no build script. The library target is named in snake case for the reason `SPEC-MOK-002` rule 2
gives for the engine's: the declared lint gate implies `non_snake_case`, and a library crate named
`mokiterions-tui` is not a legal crate name at all. `mokiterions_tui` is also the name Cargo would derive, so the
declaration is explicit rather than novel.

The binary target keeps the name `mokiterions-tui`. It is the operator-facing command, it appears in the observer's
own usage text, and `SPEC-MOK-003` fixes it.

### 5. Observer module ownership

`mokiterions-tui/src/lib.rs` declares, and contains nothing else:

```rust
pub mod authority;
pub mod export;
pub mod layout;
pub mod options;
pub mod render;
pub mod spatial;
pub mod state;

#[cfg(test)]
mod verification;
```

It defines no item, holds no state and declares no test of its own. The `verification` module is declared here and
not from the binary target because rule 10 places the cross-cutting internal tests inside the library crate.

`mokiterions-tui/src/main.rs` keeps its contents: the start-up path, the `Launch` decision, `fn main`, the event
loop, the frame and input scheduling, the idle calculation, the diagnostic report, and its own `#[cfg(test)] mod
tests`. It declares no module. It reaches the presentation layer through the library target — `use mokiterions_tui::…`
in place of `crate::…` — and it compiles no module a second time.

The **seven** `pub mod` files keep their contents. Outside their `#[cfg(test)]` blocks they are byte-identical to
their content at the predecessor commit; inside them, tests leave under rule 9 and no test's assertions change under
rule 12. A module's internal cross-references stay written as `crate::…`, which remains correct inside a library
crate.

`mokiterions-tui/src/verification.rs` is the exception, and it is an exception only because the whole file is test
code: it is declared `#[cfg(test)]`, so it has no non-test content for the byte-identity clause to bind. Its module
documentation and its import list change as the 16 tests of rule 9 leave, and an import left unused by that departure
is removed rather than allowed. The eight tests it keeps are unchanged under rule 12.

The binary target is not required to become thin, and rule 5 of `SPEC-MOK-002` is not applied to it. The engine's
binary is a shim because `execute` is public for a reason the binary itself requires and because that made four
exit-code tests public-tier. The observer's equivalent function returns a private type, so a shim would add a
public item and relocate no test; see `ADR-MOK-004`'s Option 4.

### 6. The observer's public interface, closed by provenance

**The library target's public interface is exactly the set of items that were public in the observer's non-test code
at the predecessor commit.** No item is added to it, no item is removed from it, and no item's visibility changes.

**Scope of the byte-identity check, as amended 2026-08-19.** The clause above and the diff check below are the
`WO-MOK-006` restructuring's obligation: they say that turning already-public items into a maintained contract added
nothing, removed nothing and widened nothing. They are not a freeze on the observer's code for all later work, in the
same way that `SPEC-MOK-002` rule 3's freeze on `src/simulation.rs` is scoped to the `WO-MOK-003` restructuring it
was written for. Afterwards the interface is governed by the **Growth** and **Reduction** clauses below and by rule 7,
whose no-widening prohibition stands unscoped and at every commit.

The set is closed by provenance rather than by enumeration, and the check is a property of the diff: for each of the
seven `pub mod` files, the content outside every `#[cfg(test)]` block is byte-identical to the predecessor commit's.
Byte-identity implies that no `pub`, `pub(crate)`, `pub(super)` or private item changed, so the no-widening
prohibition of rule 7 is enforced directly rather than as a consequence of a list someone maintains. `verification.rs`
is not part of this check and cannot be: it is `#[cfg(test)]` in its entirety and declares no item at all, so it can
neither hold nor widen a member of the interface.

Its measured extent, recorded so that a later reader can tell whether the interface has grown and so that
verification has a figure to compare against. The figures are the `WO-MOK-006` predecessor commit's, amended
2026-08-19 for the one reduction there has been since.

**What is counted.** One **item** is one declaration written `pub` outside every `#[cfg(test)]` block — a `pub fn`,
`pub struct`, `pub enum`, `pub const`, `pub static`, `pub type`, `pub trait` or `pub use`. A public field of a public
struct and a variant of a public enum are parts of the item that declares them and are not counted again; there are
**25** public fields, and a count of **119** is the same interface counted the other way — every line that writes
`pub`, which is one per item plus one per public field. A variant of a public enum is written without `pub` and is in
neither figure, so a count that includes variants is larger than 119 and is not this figure; that is what the
2026-08-19 amendment settled, since the superseded pair 25 and 122 was reproducible only as 122 − 97. The four
`#[cfg(test)]` hooks are excluded, because rule 7 keeps them out of the library target. Verification states which
rule it counted by, and a figure derived under the other rule is not a shortfall.

**Amended 2026-08-21 under `WO-MOK-018`: the pair reads 25 and 119, not 24 and 118, and the item count of 94 below is
unchanged.** The interface grows by exactly one public field and by no item: `state::Death` gains `pub fear: Option<u8>`
beside the `satiety` and `energy` it already declares, which the **Growth** clause below admits because `REQ-MOK-021`
needs it, by way of `SPEC-MOK-003` rule 10.6 as amended on the same date, which makes the fourth attribute one of the
final attribute values the inspector presents for a dead subject. The `state` row of the table below stays at **47**
items for that reason — a field is part of the item that declares it, which is this paragraph's own rule — and every
other row is untouched. The three alternatives were each measured and each is worse: an accessor on `Observer` would add
a public **item** and move 94; leaving the field `pub(crate)` while its two siblings are `pub` would make one struct
partly opaque and push its public-tier case into rule 10 for no reason but visibility; and deriving the value in
`render` is impossible, because the state it comes from is private and reaching it is what rule 7 forbids. **The
coincidence with the superseded figure above is a coincidence and not a reversion**: the 2026-08-19 amendment retired
the pair 25 and 122, and 122 − 97 is still not how 119 is reached. Rule 7 is untouched, since a field added to a struct
whose fields are already public widens no existing item's visibility, and the byte-identity check of the paragraphs
above is scoped to the `WO-MOK-006` restructuring and does not reach this commit.

| Module | Public items | Subject |
|---|---|---|
| `authority` | 5 | the engine's verdict and its presentation |
| `export` | 3 | the export writer and its rendered form |
| `layout` | 10 | the viewport floor, the pane thresholds and the geometry each one yields |
| `options` | 8 | the observer's own argument handling |
| `render` | 2 | the frame entry points |
| `spatial` | 19 | world-to-canvas mapping |
| `state` | 48 | the observer's state, its accessors, its filters and its event buffer |
| **Total** | **95** | |

**Re-measured 2026-08-22 under `WO-MOK-020` and unchanged: 94 items, 119 `pub` lines, 25 public fields, and every
module row above at the figure it already carries.** The re-measurement is recorded rather than the figures being
assumed to hold, because that work order adds executable behaviour to two of the seven `pub mod` files and this rule
is the only place a reader can tell a growth from an unmoved figure. Nothing it adds is a member of the interface:
`Profile`, `ActionKind`, `ActionKind::label` and the three accessors `Observer::profile_of`,
`Observer::population_profile` and `Observer::initialized_count` are all `pub(crate)`, and a `pub(crate)` item is
outside this rule's count as `Observer::name_of` has been since `WO-MOK-011` measured it that way; `Profile`'s five
counters are private fields of a `pub(crate)` type and are in neither figure; and `src/render.rs`'s six arrivals are
private to that module. So the **Growth** clause is not invoked, no item's visibility is widened, and the pair 25 and
119 is reproducible by the counting rule stated above at the candidate commit. The interface is deliberately not
widened to let a test reach these items: the sixteen cases that need them are in rule 10's tier for that reason, and
rule 6's **Growth** clause is why — a test is never the requirement that grows the interface.

**Amended 2026-08-24 under `WO-MOK-025`: the interface grows by one item and one public field, so the figures read 95
items, 26 public fields and 121 `pub` lines**, and the `state` row of the table above reads **48** rather than 47. It read
47 and the total 94 until this amendment, and the pair read 25 and 119. The two arrivals, with the requirement that
authorizes each:

- `state::Observer::with_port(options: Options, port: Option<Box<dyn Proposer>>) -> Result<Self, String>`, a public
  **item**, so it moves the item count and the `state` row. `SPEC-MOK-007` rule 20.4.1 needs it: the replay port is built
  once and held for the whole run, so the transcript cursor, the accumulated cost and the fallback count are the run's and
  not a tick's. `Observer::new` remains and delegates to it with no port, so no existing signature changes.
- `options::Options::transcript_path: Option<String>`, a public **field** of a public struct, so it moves the field count
  and the pair while leaving the `options` row at 8 items — a field is part of the item that declares it, which is this
  rule's own counting. `SPEC-MOK-007` rule 18.4 needs it: this host re-reads the raw `--transcript-path` argument and the
  value has to cross from the parser to the start-up path.

**Both are forced by rule 4's own target structure and not by a test.** `mokiterions-tui/src/main.rs` is a separate crate
from the library and can reach only `pub` items; it is the target that opens the transcript, because `SPEC-MOK-007` rule
12.1.1 puts the opening in the host and rule 19.7 requires that no engine message name a path the engine resolved, so what
crosses into the library is an already-open reader and never a path. So the binary reads `options.transcript_path`, builds
the port, and hands it to `Observer::with_port`. The **Growth** clause's "A test is never that requirement" is satisfied:
the four public-tier cases in `tests/replay.rs` call `with_port` because the binary already does, not the other way round —
`src/main.rs:134` is the caller that makes it `pub`.

Three alternatives were each measured and each is worse. **Opening the transcript inside `Observer::new`** adds no public
item, and it puts a path the library resolved inside the library, which is what rule 19.7 forbids a diagnostic to name and
what rule 12.1.1 divides the other way; it would also make the observer's start-up failure the library's message rather
than the host's. **A `pub(crate)` field beside four `pub` siblings** on `Options` would make one struct partly opaque for
no reason but visibility, which the 2026-08-21 amendment above already rejected for `Death`, and it would not work at all:
`src/main.rs` is outside the crate. **Rebuilding the port per tick** inside `advance` would need no constructor and is
prohibited outright by rule 20.4.1. Rule 7 is untouched, since no existing item's visibility widens — both arrivals are new
declarations — and the byte-identity check above is scoped to the `WO-MOK-006` restructuring and does not reach this
commit.

The `layout` row read 13 items and "viewport tiers and the pane geometry of each" until the 2026-08-19 amendment. The
three items it lost are the `Tier` enum, that enum's `label` method and the `tier_for` function, together with the
`Panes::tier` field, which is the one of the 25 recorded public fields that goes. `SPEC-MOK-003` rule 5 as amended is
the authority: it decides each pane on one threshold in one axis, so there is no configuration left to name, to label,
or to return.

`SPEC-MOK-002` rule 5 closes the engine's interface as a list of items and justifies each one, because every engine
item is a potential path to authoritative state. That is not the situation here, and rule 6 of *this* specification
is deliberately weaker in form and stronger in effect: weaker, because it names no item; stronger, because it admits
nothing at all. The observer holds no authority over world state, so there is no admission to justify — and a
94-row table would have to be re-derived on every refactor that renames a private helper, which is a maintenance
burden that buys nothing.

**Growth.** The interface grows only when an approved requirement needs it to grow, and this rule is amended in the
same act, recording the added items and the requirement that authorizes them. A test is never that requirement.

**Reduction, added 2026-08-19.** The interface shrinks only when an approved requirement, or an approved amendment to
the specification that states it, removes the need for an item, and this rule is amended in the same act, recording
the removed items, the figures they change and the authority that removes them. Neither convenience nor a refactor is
that authority: an item that no longer has a caller is still part of the contract until the amendment says otherwise,
and the module's own tests are never the reason. A reduction cannot widen anything, so it is never in tension with
rule 7; what it can do is silently break a reader who relies on this rule's figures, which is why the figures are
amended and not merely re-measured.

### 7. Prohibited

- No item's visibility is widened. In particular no private, `pub(crate)` or `pub(super)` item becomes `pub`, and
  no item becomes `pub(crate)` from private, in order to relocate a test.
- No `#[cfg(test)]` attribute is removed, and no `#[cfg(test)]` item becomes unconditional. The four test-only hooks
  on the observer's state type — the ones that select a subject, set an overlay, replace the decision list and
  replace the snapshot — stay `#[cfg(test)]`, so they are absent from the library target and unreachable from any
  test outside the crate.
- No feature flag, `cfg` attribute, self dev-dependency, or conditional-visibility mechanism makes a private item
  or a hook reachable from outside the crate, including in test builds. There is no test-support seam, and the
  prohibition is `SPEC-MOK-002` rule 6's, restated for this package.
- The binary target does not declare the presentation modules a second time.
- No public item of the engine package is added. `SPEC-MOK-002` rule 5 stays closed; this specification adds nothing
  to it and needs nothing from it.
- No dependency, dev-dependency, feature, build script or workspace dependency table is introduced in either
  package. **Amended 2026-08-20.** This bullet was written as a prohibition on the relocation `REQ-MOK-030` requires,
  and the relocation introduced none of them: that half is a completed fact and is unchanged. As a standing rule it is
  now narrower than `ADR-MOK-006` allows, so it reads: a dependency, dev-dependency or workspace dependency entry is
  introduced only as a declared entry admitted under `ADR-MOK-006` and recorded in the declaring specification's
  declared set, and a feature is enabled only as part of such an entry. **A build script in either of this
  repository's own packages stays prohibited outright**, by `SPEC-MOK-002` rule 1, which `ADR-MOK-006` leaves
  untouched; decision 13 concerns a *dependency's* build script and admits none here.

### 8. Tiers and the placement rule

Every observer test belongs to exactly one tier, and the tier is determined by the access the test requires:

- if the test can be written using only rule 6's interface, with its assertions unchanged and with no item widened,
  it belongs to the **public tier**;
- otherwise it belongs to the **internal tier**.

This is `SPEC-MOK-002` rule 7 verbatim, with rule 6 of this specification in place of rule 5 of that one. A test is
not left inline for convenience when the interface suffices, and a test is not promoted to the public tier by
widening the interface. Required access is a property of the test as written; the subject it covers does not decide
the tier, and neither does the file it currently sits in.

A test that reaches one of the four hooks is in the internal tier by definition, because the hook does not exist in
the build a public-tier test links.

### 9. Public tier

Located in `mokiterions-tui/tests/`, one file per subject, each compiled as its own integration-test target and
reaching the code as `use mokiterions_tui::…`. The arrangement, with the count each file receives:

| File | Subject | Tests |
|---|---|---|
| `tests/authority.rs` | the engine's verdict and its presentation | 5 |
| `tests/export.rs` | export content and its rendered form | 7 |
| `tests/layout.rs` | the pane thresholds, the floor and pane geometry | 11 |
| `tests/options.rs` | the observer's argument handling | 18 |
| `tests/render.rs` | frame content asserted through the frame entry points | 29 |
| `tests/replay.rs` | the observer as a host of the decision port | 5 |
| `tests/spatial.rs` | world-to-canvas mapping | 7 |
| `tests/state.rs` | observer state, accessors, filters and the event buffer | 22 |
| `tests/verification.rs` | the cross-cutting properties: non-perturbation, export fidelity, presented-value fidelity, the authority verdict, colour independence | 31 |
| **Total** | | **135** |

The counts began as the measured outcome of applying rule 8 to the 109 tests at the `WO-MOK-006` predecessor commit —
77 in this table — and they are stated so that a relocation that loses or invents a test is detectable. They are not a
quota: if applying rule 8 during implementation assigns a test differently than measured, rule 8 governs, the count is
corrected here, and the discrepancy is recorded as work-order evidence. A further file may be added when a further
public subject appears. One file per test is not the arrangement.

`tests/layout.rs` reads 10 rather than 7 as amended 2026-08-19, and the 10 is a net figure rather than three
additions to an unchanged seven. `SPEC-MOK-003` rule 5 as amended adds a monotonicity obligation over the whole plane
and a threshold per pane, and `VER-MOK-005` as amended requires a case for each; the same amendment deletes the tier
table those thresholds replace. The measured composition is three tests gone and six added:

| Gone | Why |
|---|---|
| `tiers_match_the_specified_table_including_its_boundaries` | asserts `tier_for` and the four-row table, neither of which rule 5 still defines |
| `tier_minimums_hold_wherever_the_tier_declares_one` | asserts the per-tier minimum columns and rows the table carried |
| `excluded_panes_are_the_ones_the_tier_omits` | renamed rather than deleted; see the first addition below |

| Added | Obligation it carries |
|---|---|
| `excluded_panes_are_the_ones_the_viewport_omits` | the rename of the third test above, same assertion against the amended rule |
| `each_pane_appears_at_its_threshold_on_the_axis_that_constrains_it` | `VER-MOK-005`'s pane-presence case, checked on both sides of each of the three thresholds |
| `the_log_is_ten_rows_only_where_both_thresholds_are_met` | `VER-MOK-005`'s log-height case |
| `enlarging_the_viewport_never_removes_a_pane` | `VER-MOK-005`'s layout-monotonicity case and property |
| `the_one_to_one_threshold_with_the_roster_alone_is_113_columns` | `VER-MOK-005`'s mapping-injectivity property, in the roster-only band the amendment creates |
| `the_vertical_one_to_one_threshold_is_44_rows` | the same property on the other axis, which the amendment moves |

The last two are not named by `VER-MOK-005`'s case table. They belong to its **Mapping injectivity** property, which
the pre-existing `the_one_to_one_threshold_with_the_inspector_shown_is_157_columns` covered when the inspector was the
only regime; rule 5 as amended creates a second regime, so the property needs a threshold case in it and one on the
row axis. Every item all nine name is already in rule 6's interface, so rule 8 places them in the public tier and
nothing widens.

`tests/options.rs` reads 8 rather than 7 and `tests/render.rs` reads 12 rather than 8 as corrected for
`WO-MOK-010`, which `VER-MOK-010` obliges to assert `REQ-MOK-032` at the frame, and for `master`'s `WO-MOK-007`,
which `VER-MOK-007` obliges to assert rule 4 clause 7 at the frame. Five tests arrive and none departs:

| Added | Obligation it carries |
|---|---|
| `the_usage_text_advertises_every_policy_the_engine_accepts` | `SPEC-MOK-001`'s *Help output* against the third decision source `REQ-MOK-033` adds |
| `the_roster_presents_four_gauges_at_every_declared_viewport_that_presents_it` | `VER-MOK-010` oracle 4's cell-position case, at every viewport rule 5 presents the roster at |
| `the_fourth_gauge_is_a_proportional_bar_at_zero_and_away_from_it` | `VREC-MOK-005` finding 3, read against `SPEC-MOK-003` rule 4.4 at zero and away from it |
| `the_survival_bands_reach_the_frame_and_three_differ_in_one_entry` | `WO-MOK-007`: rule 4 clause 7's bands in drawn cells, on an entry whose three attributes fall in three bands |
| `a_selected_entry_keeps_its_bands_under_reversed_video` | `WO-MOK-007`: clause 6's reversal and clause 7's band on one entry, neither replacing the other |

All five reach the code as `use mokiterions_tui::…` and name only items already in rule 6's interface, so rule 8
places them in the public tier and nothing widens. The internal tier of rule 10 rises to 37, all five of its
arrivals `WO-MOK-007`'s: `WO-MOK-010` adds no observer test that requires a private item or a hook.

`tests/verification.rs` reads 19 rather than 16 as corrected for `WO-MOK-011`, which `VER-MOK-011` obliges to assert
`REQ-MOK-041` at the frame. Three tests arrive and none departs:

| Added | Obligation it carries |
|---|---|
| `every_pane_identifying_a_mokiterion_presents_its_own_reported_name` | `SPEC-MOK-003` rule 10 as amended, which adds the name to the presented values, against the roster and the inspector in both entry forms |
| `every_glyph_drawn_is_its_own_subjects_initial_in_both_zooms` | `SPEC-MOK-003` rule 2 as amended, checked cell by cell in overview and in detail |
| `the_inspector_identifies_a_dead_subject_by_name_and_identifier` | the same rule 10 clause on a subject whose record is the only place its name survives |

The file's subject line is unchanged and is not extended for them: presented-value fidelity is what all three assert,
and the name is a presented value from the moment `SPEC-MOK-003` rule 10 as amended lists it as one. All three read
the expected name from the engine's own `agent_initialized` records through the already-public `Observer::events`,
which is what keeps them in this tier. `Observer::name_of` is `pub(crate)`, so a test that wanted it would belong to
the internal tier under rule 8 rather than justify widening it — rule 6's **Growth** clause says a test is never the
requirement that grows the interface, and none of the three needs it.

`tests/render.rs` reads **22** rather than 12, `tests/layout.rs` **11** rather than 10 and `tests/verification.rs`
**20** rather than 19 as corrected for `WO-MOK-013`, whose `VER-MOK-013` obliges the observer to assert `REQ-MOK-047`,
`REQ-MOK-048` and `REQ-MOK-049` at the frame. Twelve tests arrive in this tier and none departs:

| Added | Where | Obligation it carries |
|---|---|---|
| `every_living_mokiterion_has_an_entry_at_the_reference_viewport` | `tests/render.rs` | `REQ-MOK-020` at the three-line entry: twelve entries drawn, none hidden, at two ticks of one run |
| `a_declining_mokiterion_shows_a_declining_bar` | `tests/render.rs` | `REQ-MOK-047`'s acceptance scenario 1, over a 200-tick run's own trajectory rather than a held value |
| `the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport` | `tests/render.rs` | `REQ-MOK-048` at every declared viewport and the floor, in the first frame, with the overlay reachable from it |
| `the_hint_is_present_after_two_hundred_ticks_in_both_run_states` | `tests/render.rs` | the same affordance's permanence, running and held, over 200 frames each |
| `the_hint_displaces_neither_the_announcement_nor_the_footer` | `tests/render.rs` | rule 5's reservation: the hint, every announced pane and rule 8's footer on one frame |
| `the_announcement_states_the_axis_and_the_value_the_layout_decides_presence_from` | `tests/render.rs` | `REQ-MOK-049`'s content, per excluded pane, at every declared viewport |
| `the_announcement_is_emphasised_and_the_optional_segments_are_not` | `tests/render.rs` | rule 5's emphasis clause against an optional segment on the same row |
| `the_announcement_and_the_hint_read_nothing_but_the_viewport` | `tests/render.rs` | rule 5's opening obligation, for the two elements this work order adds |
| `the_announcement_appears_and_disappears_with_the_pane_it_names` | `tests/render.rs` | `VER-MOK-013`'s scenario 5, across the inspector's threshold and back on one observer |
| `no_entry_is_lost_silently_at_any_viewport_presenting_the_roster` | `tests/render.rs` | `VER-MOK-013`'s invariant: drawn plus hidden equals living, over the whole plane |
| `the_reference_roster_interior_holds_the_whole_population` | `tests/layout.rs` | `SPEC-MOK-003` rule 4 item 1 as amended: 36 interior rows, twelve entries of three, and no row to spare |
| `the_announcement_and_the_hint_survive_the_loss_of_colour` | `tests/verification.rs` | `SPEC-MOK-003` rule 2.5 for both, through the monochrome projection that file already uses |

Every one is writable through rule 6's interface without widening it, so rule 8 places all twelve here; the two cases
that need the range of a value rather than a run's trajectory reach a hook and are in rule 10 instead. **One test in
`tests/layout.rs` was renamed rather than removed**, which is why that file reads 11 and not 12:
`the_log_is_ten_rows_only_where_both_thresholds_are_met` becomes `the_log_is_six_rows_wherever_it_is_present`.
`WO-MOK-013` expected its subject to cease to exist and its removal to be reported here. It was kept instead, because a
constant is a pass condition and the two viewports that carried ten rows are the ones that now assert six, so the
withdrawn growth is asserted absent rather than left untested. Under rule 12 that is a rename with its assertion
strengthened, not a loss, and the departure the work order anticipated does not appear in the figures above.

`tests/verification.rs` reads **22** rather than 20 and `tests/state.rs` **22** rather than 21, so the total reads
**103** rather than 100, as corrected for `WO-MOK-016` and `WO-MOK-018` together. **The two are corrected in one act
because neither is statable without the other**: `WO-MOK-016` left this table uncorrected, so a correction naming only
`WO-MOK-018` would have to state 22 for `tests/verification.rs` while accounting for one of its two arrivals, and a
reader could not tell which work order the unexplained test belonged to. Three tests arrive and none departs:

| Added | Where | Obligation it carries |
|---|---|---|
| `the_social_source_is_rejected_only_as_the_specification_admits` | `tests/verification.rs`, under `WO-MOK-016` | which grounds of `SPEC-MOK-001` rule 6 the fourth shipped source reaches, asserted as the specification admits them rather than as none |
| `a_death_carries_the_fear_the_engine_last_reported_for_its_subject` | `tests/state.rs`, under `WO-MOK-018` | `SPEC-MOK-003` rule 10.6 as amended at the derived state: the fourth final attribute is the engine's own last reported value, compared against the `survival_changed` payload in the event buffer rather than against a constant |
| `the_inspector_presents_a_dead_subject_s_final_fear` | `tests/verification.rs`, under `WO-MOK-018` | the same clause at the frame, on a subject selected while living and held through its death |

All three reach the code as `use mokiterions_tui::…` and name only items already in rule 6's interface, so rule 8
places them here. **One further name changed under `WO-MOK-016` and is a rename rather than an arrival**, which is why
`tests/verification.rs` rises by two and not by three across the two work orders:
`no_shipped_decision_source_has_a_proposal_rejected` becomes
`no_source_confined_to_the_valid_action_list_has_a_proposal_rejected`, because a fourth shipped source is rejected and
the case's name had been a count of what ships rather than the property the others share. Rule 12 governs it as a
rename with its sweep unchanged, on the same footing as `tests/layout.rs`'s rename in the paragraph above. **No test is
lost.** `WO-MOK-018`'s third arrival is in rule 10, for the access it requires; the engine's 37 arrivals under
`WO-MOK-016` are governed by `SPEC-MOK-002` rather than by this rule and are counted in rule 11.

`tests/render.rs` reads **29** rather than 22, `tests/verification.rs` **29** rather than 22, and the total **117**
rather than 103, as corrected for `WO-MOK-020`. Fourteen tests arrive, none departs and none is renamed:

| Added | Where | Obligation it carries |
|---|---|---|
| `the_population_pane_states_what_it_is_not_above_every_total` | `tests/render.rs` | `VER-MOK-017` O13: `SPEC-MOK-003` rule 10 clause 5 as amended, that the retained statement and the selecting control are above every figure of the population block |
| `no_line_the_inspector_presents_is_clipped_or_wrapped_at_either_width` | `tests/render.rs` | O16: every label and its figure share one rendered row, in four pane states, at the reference viewport and at both shapes of the presence threshold, with the widest row measured against the interior |
| `traversing_the_selection_states_resets_no_total` | `tests/render.rs` | `VER-MOK-017` scenario 4: the totals are the run's history and not the selection's, walked through the selection states and back |
| `two_frames_of_the_same_tick_present_identical_figures` | `tests/render.rs` | P5: drawing is a function of state, so a second frame of one tick presents the same figures |
| `both_pane_states_present_the_same_figures_as_the_overlay_below_the_threshold` | `tests/render.rs` | scenario 6: the same content through the overlay at a width rule 5 gives no inspector pane |
| `the_totals_are_beneath_the_decision_record_and_displace_none_of_its_lines` | `tests/render.rs` | scenario 2: rule 10's existing content keeps its position and every line it had |
| `a_selected_mokiterion_that_dies_keeps_its_death_lines_and_its_frozen_totals` | `tests/render.rs` | scenario 3: rule 10.6's death lines and `REQ-MOK-061` clause 2's frozen record on one pane |
| `presenting_the_totals_perturbs_no_run_on_any_declared_seed` | `tests/verification.rs` | O17: `REQ-MOK-025` with the inspector drawing both blocks on every tick of every declared seed, on records and on per-tick entropy-bearing counts |
| `no_total_reaches_the_export_and_it_stays_the_engines_records` | `tests/verification.rs` | O18: rule 9.4's export is the engine's own bytes and carries none of the pane's vocabulary |
| `the_engine_declares_no_dependency_on_the_observer` | `tests/verification.rs` | O19.1: the engine's declared dependency set, over the manifest itself |
| `no_profile_type_or_accessor_appears_in_the_engine` | `tests/verification.rs` | O19.2: the retained profile's type, kind enumeration and accessors are absent from the engine's tree |
| `the_engine_still_exposes_exactly_two_mutating_entry_points` | `tests/verification.rs` | O19.3: `SPEC-MOK-003`'s *Data and interface contracts* clause 2 and `WO-MOK-020` constraint 5, parsed as the command's meaning rather than its text |
| `no_total_can_reach_an_observation_or_a_decision_source` | `tests/verification.rs` | O19.4: `REQ-MOK-062`'s boundary claim, over the observer's own tree |
| `no_total_is_a_float_and_no_presented_figure_is_a_ratio` | `tests/verification.rs` | O20.1: no `f32` or `f64` in the two modules the change surface names, and no percentage, decimal separator or ratio between digits in the rendered pane |

Every one of the fourteen reaches the code as `use mokiterions_tui::…` and names only items already in rule 6's
interface, so rule 8 places them here; `Profile`, `ActionKind` and the three profile accessors are `pub(crate)`, so
the cases that need them are in rule 10 instead and rule 6's **Growth** clause is not invoked for any of them — a
test is never the requirement that grows the interface. **The five static checks are in this tier rather than the
internal one on rule 8's own test**, which reads the access a case requires: each reads a file of the repository
through `env!("CARGO_MANIFEST_DIR")` and names no item of either package at all, so the public tier suffices and
nothing widens. Two of them are scoped rather than global, and each scope is measured rather than assumed:
`no_profile_type_or_accessor_appears_in_the_engine` asserts the profile's own names and not the counter words it
shares with the engine, because the engine already keeps one private world-wide `crossings` total for its structured
run record that predates this work order; and `no_total_is_a_float_and_no_presented_figure_is_a_ratio` asserts
`src/state.rs` and `src/render.rs` rather than the package, because `src/spatial.rs` already computes canvas
coordinates in `f64` and is untouched here. A package-wide form of either would fail on state this work order does
not touch and would say nothing about what it adds. Neither `tests/render.rs`'s nor `tests/verification.rs`'s subject
line is extended: frame content and the cross-cutting properties are what all fourteen assert.

`tests/options.rs` reads **9** rather than 8 and the total **118** rather than 117, re-measured on the tree that
merges `master` at `3ca2028` into `WO-MOK-020`'s candidate. One test arrives, none departs and none is renamed:
`the_shared_entries_are_the_engines_own_words`, in `tests/options.rs`. **It is not this work order's**, and it is
stated with its origin rather than absorbed into this work order's fourteen, on the precedent of the engine's 157 in
rule 11 below: it arrives from `master`'s usage-text work, which changes `mokiterions-core/src/cli.rs`,
`mokiterions-core/tests/cli.rs`, `mokiterions-tui/src/options.rs` and `mokiterions-tui/tests/options.rs` and amends
this specification nowhere. It reaches the code as `use mokiterions_tui::…` and names only items already in rule 6's
interface, so rule 8 places it here and rule 6's **Growth** clause is not invoked: **rule 6 is re-measured unmoved**
at **94** items, **119** `pub` lines and **25** public fields, with every module row of its table unchanged and
`options.rs` among them at 8 items, because the 97 lines that work adds to `src/options.rs` add one private function
and no member of the interface. That file's subject line is not extended. **No approved figure is contradicted**: the
117 stands in the paragraph above as the figure of `WO-MOK-020`'s candidate commit `f633eda`, which `VREC-MOK-023`
binds, and is not edited; the table carries this tree's figure.

`tests/authority.rs` reads **5** rather than 4, `tests/options.rs` **14** rather than 9, **`tests/replay.rs` arrives with
4**, `tests/verification.rs` reads **31** rather than 29, and the total **130** rather than 118, as corrected for
`WO-MOK-025` and measured at its candidate commit. **Two of the twelve tests behind those movements are not this work
order's**, and they are separated before this work order's ten are stated, because a total that mixed them would
attribute `master`'s work here. The baseline is this branch's merge base `2a93914`, at which this tier measures **120**: `tests/verification.rs`
had already risen from 29 to **31** on `master` after the tree the paragraph above measured, and that is the whole of the
difference between 118 and 120. **This work order adds ten and loses none:**

| Added | Where | Obligation it carries |
|---|---|---|
| `the_source_row_names_every_source_and_agrees_with_the_mapping` | `tests/authority.rs` | `SPEC-MOK-007` rule 18.5's second obligation: the overlay's source row names all five sources and names each with the identifier `for_type` gives, compared against that function rather than against a written-out expectation, because a written-out expectation is the hard-coded description that rule corrects |
| `the_replay_source_with_no_transcript_is_refused_at_start_up` | `tests/options.rs` | `SPEC-MOK-007` rules 18.5 and 20.3: this host accepts the fifth source only with a transcript and refuses it without one, and the refusal names the option that would satisfy it |
| `the_transcript_option_is_never_accepted_and_ignored` | `tests/options.rs` | rule 18.4.2, which is the defect GitHub issue 40 tracks and which this option was forbidden to repeat: an option the shared parser accepts reaches a diagnosis rather than silence |
| `the_transcript_path_is_re_read_verbatim_and_reaches_no_configuration` | `tests/options.rs` | rule 12.1.1's division of labour: the observer re-reads the argument and holds no path the library could reach |
| `the_transcript_option_states_when_it_is_required_and_when_it_is_refused` | `tests/options.rs` | `SPEC-MOK-003`'s usage-text obligation for the one new option this host acts on, in both directions |
| `the_usage_text_states_that_this_host_only_replays` | `tests/options.rs` | `REQ-MOK-077` in the observer's own words, which is the one thing this host may say that is not the engine's text byte for byte |
| `every_target_spells_the_transcript_option_the_same_way` | `tests/options.rs` | `SPEC-MOK-003`'s byte-identity obligation extended to the new shared option, asserted across both targets rather than within one |
| `the_replay_source_with_no_port_is_refused_on_the_first_tick` | `tests/replay.rs` | rule 20.8 through this host: a selected source with no port refuses rather than borrowing another source's decision, and rule 9.7 is why there is no run left to fall back to |
| `a_source_that_decides_for_itself_needs_no_port` | `tests/replay.rs` | rule 20.9's other side: a port is ignored under the four sources that decide for themselves, exactly as an absent record sink is, and is not an error |
| `one_port_serves_the_whole_run_and_is_never_rebuilt` | `tests/replay.rs` | rule 20.4.1: one port is lent for the run, so the transcript cursor, the accumulated cost and the fallback count are the run's and not a tick's |
| `a_port_failure_ends_the_run_in_the_ports_own_words` | `tests/replay.rs` | rules 12.4 and 19.7: a replay that cannot satisfy an opportunity fails in the port's own diagnosis, and the engine adds no words of its own to a failure it cannot diagnose |
| `the_observer_replays_this_source_to_the_horizon_with_every_pane` | `tests/replay.rs` | `VER-MOK-018` case **L31**: this host advances to the transcript's horizon under operator control and its six panes behave as they do under `social`, each read in its own region and compared against that source at the same seed |

The table above lists **eleven** rows against ten arrivals, because `the_usage_text_states_that_this_host_only_replays`
and `every_target_spells_the_transcript_option_the_same_way` are both `tests/options.rs`'s and both counted once:
`tests/options.rs` moves by **five** and `tests/replay.rs` arrives with **four**, which with `tests/authority.rs`'s one
is the ten. The count is stated this way rather than left to a reader's addition because five plus four plus one is the
only arithmetic that reaches 120 + 10 = **130**.

**`tests/replay.rs` is a new file of this tier and rule 9's own text admits it**: "A further file may be added when a
further public subject appears." The subject is **the observer as a host of the decision port** — `REQ-MOK-077`'s
division, which belongs to no file above: its cases are neither argument handling nor frame content, and placing them in
`tests/options.rs` would have made that file's subject line false while hiding the one target whose existence is the
requirement. The observer's public-tier target count therefore moves from **eight to nine**, for
the first time since this rule was written, and rule 11's sentence that names it moves with it. All ten reach the code as
`use mokiterions_tui::…`, so rule 8 places them here. **Rule 6's interface does move at this candidate — by one item and
one public field, to 95 items, 26 public fields and 121 `pub` lines — and none of the ten is the reason.** Rule 6 records
the growth, the requirement behind each arrival and the three measured alternatives; what belongs here is only that the
**Growth** clause's "A test is never that requirement" holds of these ten. `Observer::with_port` is `pub` because
`mokiterions-tui/src/main.rs` is a separate crate and is the target that opens the transcript, and
`Options::transcript_path` is `pub` because that target reads it; the four cases in `tests/replay.rs` call the constructor
the binary already calls, which is the opposite of an item widened to be reached. `authority.rs` stays at 5 items, measured
rather than assumed: the fifth source's identifier is a new arm of an existing `match`, the row that presents it is derived
from that function, and the list of sources the overlay walks is a function-local `const`. **No approved figure is
contradicted**: the 118 stands in the paragraph above as the figure of the tree it names and is not edited.

**A twelfth row and an eleventh arrival, re-measured at `bce4229` within the same work order**, so the table reads
**twelve** rows against **eleven** arrivals and `tests/replay.rs` **five** rather than four: the paragraph above's five
plus four plus one is now five plus **five** plus one, and 120 + 11 = **131**. `tests/options.rs`'s five and
`tests/authority.rs`'s one are unmoved; the whole of the difference is `tests/replay.rs`. The arrival is `VER-MOK-018` case **L31**, which nothing in this tier
carried — `llm` appeared in three of the observer's nine public-tier files and in none of the six that own the panes — and
it belongs to `tests/replay.rs` on the subject line that file was given, the observer as a host of the decision port, since
what it asks is whether this host replays to a transcript's horizon rather than anything about a frame in isolation. **The
paragraph above is not contradicted**: its 130 and its four are true of `d8ff20e`, and the correction is owed to the tree
finally offered, exactly as the 118 is kept above. Nothing widens: the case calls `Observer::with_port` and `options::parse`
as the four beside it do, and reads the panes through `layout::resolve` and `render::draw`, all of which rule 6's interface
already carried. The observer's public-tier target count is unmoved at **nine**.

**Four arrivals under `WO-MOK-026`, measured at its candidate `7f9e20a`**, so the table reads `tests/options.rs` **18**
rather than 14 and the total **135** rather than 131. The four are counted separately from the eleven above rather than
added to them: those eleven are `WO-MOK-025`'s and the table listing them is that work order's reconciliation, and a
total that mixed the two would attribute this stage's refusals there. The baseline is this branch's merge base
`d96cced`, at which this tier measures **131** — the figure the paragraph above records, re-measured file by file rather
than inferred from an unchanged total, because `main` moved after `bce4229` and a figure that had drifted there would
otherwise be absorbed silently. **All four are `tests/options.rs`'s, none departs and none is renamed**, which is
measured by qualified name against that baseline across both packages: 53 additions and **0 removals**, of which 4 are
this tier's, so no rename can hide in the diff.

| Added | Obligation it carries |
|---|---|
| `the_live_run_options_are_refused_and_name_the_host` | `SPEC-MOK-007` rule 18.4.2 for the five live-run options the shared parser now accepts: each is refused **by this program**, naming the option the operator typed and this host's own limit as the reason, never calling it unknown and offering no substitute source under rule 20.3. Asserted with `llm` selected, with another source selected and with none, because rule 18.4.3's "only used by `--policy llm`" already refuses the last two and this host must not be resting on that — under `llm` it does not fire at all, and an operator would otherwise reach a live run this host cannot perform |
| `the_refused_list_and_the_engines_own_options_agree` | the same rule's other direction, that the refusal covers every live-run option the shared parser accepts and no other. Neither list is derivable from the other, so an option added to the engine's parser and not to this program's list is accepted and ignored again — the shape GitHub issue 40 tracks — and an option in the list the parser does not accept makes the refusal claim the other binary accepts something it does not |
| `no_invocation_this_program_accepts_carries_a_spend_ceiling` | rule 18.4.2 read as unreachability rather than erasure. `Options::spend_ceiling` was overridden to `None` after the shared parser had validated and retained the operator's amount, which is an accepted-and-ignored option with an override standing in for a refusal; an unreachable field needs no override and an override needs a reader to keep it in step with every future option that could set one |
| `a_refused_live_run_enters_no_terminal_and_spawns_nothing` | `VER-MOK-018` case **L32**'s two structural halves, both asserted as absences: the one call that enters the terminal is reached after start-up has had its chance to refuse, and no file under this package's `src/` names a process-spawning primitive in any target, which is stronger than "this path spawns nothing" |

**Nothing widens, and rule 6's interface is unmoved** at the **95** items, **26** public fields and **121** `pub` lines
the paragraphs above record — measured at this candidate module row by module row rather than inferred from an unchanged
total, which matters here because this work order does change three files under `mokiterions-tui/src/`. What it adds
there is a private `const LIVE_RUN_OPTIONS` in `options.rs` and a `Proposer` implementation for the private
`state::LentPort`, whose two methods are not `pub` declarations; the `options` row stays at 8 items and 5 public fields
and the `state` row at 48 and 8. The four cases reach the code through `options::parse`, through
`mokiterions::cli::parse` and `mokiterions::cli::USAGE`, and by reading this package's own sources as text, all of which
this tier already had: the refused list is read through `include_str!` **as text and not as an item**, which is why a
private `const` can be held equal to the engine's own accepted set without becoming a member of the interface. **The
paragraph above is not contradicted**: its 131 and its five are true of `bce4229`.

### 10. Internal tier

Located in a `#[cfg(test)]` module inside the crate, beside the code it covers:

| Location | Tests | Why they cannot move |
|---|---|---|
| `mokiterions-tui/src/render.rs` | 29 | assert drawing internals; the module declares 60 private items — 36 functions and 24 constants — against 2 public ones, and 8 of the 29 additionally reach a hook |
| `mokiterions-tui/src/verification.rs` | 9 | reach several modules **and** a hook, so they belong to no single module's tier and cannot leave the crate |
| `mokiterions-tui/src/state.rs` | 21 | 3 use a hook; 1 asserts a private detail of the state type; 1 calls the private `ingest` directly and then renders through a hook; 16 name the `pub(crate)` retained profile, its kind enumeration or its three accessors, none of which exists in the build a public-tier test links |
| `mokiterions-tui/src/main.rs` | 8 | every one requires a private item of the binary: 4 name `tick_interval`, `due`, `idle_for` or `report` directly, and 4 reach the start-up function and the private `Launch` type through two helpers in the test module |
| **Total** | **67** | |

`mokiterions-tui/src/render.rs` reads 17 rather than 12 as corrected for `master`'s `WO-MOK-007`. Its five arrivals
are `the_survival_bands_are_the_three_the_rule_fixes`, `banding_changes_no_character_of_an_entry`,
`each_gauge_carries_its_own_band_and_nothing_else_carries_one`, `a_band_reads_only_the_value_it_is_given` and
`the_collapsed_form_takes_no_band`. Each names `band`, `gauge` or `entry_lines`, every one of them private to the
module, so rule 8 places them here and none of them is writable in the public tier. None uses a hook, which is why the
hook figure in the table above is unchanged at five.

It reads **18** rather than 17, `src/verification.rs` **9** rather than 8, and the total **39** rather than 37, as
corrected for `WO-MOK-011`. Two tests arrive:

| Added | Where, and why it cannot move |
|---|---|
| `an_entry_carries_the_name_before_the_identifier_and_takes_six_columns` | `src/render.rs`: it asserts the column arithmetic of `SPEC-MOK-003` rule 4 as amended — the name occupies the entry's first six columns and the identifier follows it — by calling the private `entry_lines`. No public entry point yields one roster entry |
| `a_subject_whose_record_was_never_ingested_is_presented_without_a_name` | `src/verification.rs`: it reaches the state hook **and** three modules, and `REQ-MOK-041`'s provenance claim in its negative form needs a subject the observer holds and was never told the name of, which no run produces because `Observer::new` ingests the initialization records before the first frame |

Rule 8 places both here because of the access they require and not because of the subject they cover, and widening
an item to relocate either is what rule 8's second paragraph and rule 6's **Growth** clause forbid. `src/render.rs`'s
item counts in the row above are unchanged, measured rather than assumed: the merged module declares the same 48
items `master` declares, the only difference between them being that `entry_lines` gains a parameter and with it a
wrapped signature, so no private function, constant or public item is added or removed. `src/verification.rs` still
declares no item at all, so the sentence below about it holding "only the eight tests above" reads nine as of this
correction.

`src/render.rs` reads **20** rather than 18 and the total **41** rather than 39, as corrected for `WO-MOK-013`, whose
`VER-MOK-013` obliges a case for `REQ-MOK-047` over the whole value range and over every bar width the roster can
produce. Two tests arrive and none departs:

| Added | Where, and why it cannot move |
|---|---|
| `a_ten_point_step_moves_the_fill_at_the_reference_viewport` | `src/render.rs`: `REQ-MOK-047` is a property of all 101 values and a run reaches only the values it reaches, so the case holds every attribute at each value in turn through the `#[cfg(test)]` snapshot hook. It also reads the drawn gauges cell by cell, which no public entry point yields |
| `every_bar_width_the_roster_can_produce_resolves_a_ten_point_step` | `src/render.rs`: the same property at every distinct roster geometry over the plane `MIN_WIDTH..=200 × MIN_HEIGHT..=60`, which needs the same hook and the same cell reading. Sweeping the plane at the layout and drawing once per distinct roster rectangle is what makes it affordable, and the reduction is recorded in the test rather than left as a silent sample |

Both reach the hook through the test module's own `hold_every_attribute_at` helper rather than naming it in the test
body, which rule 8 treats as reaching it: the placement rule reads the access a test requires, not the line it is
written on. **So the hook figure in the table above moves from 5 of 18 to 7 of 20**, and the two arrivals are the only
tests of this work order that could not be written in the public tier — its other twelve are in rule 9.

`src/render.rs`'s private-item count in the table above moves with them, from 47 to **48**: `BAR_ROW_OVERHEAD` changes
value and not kind, and two constants arrive for the permanent affordance's long and short forms. Its 30 private
functions are unchanged, since `announcement_text` gains a renamed parameter and no function is added or removed.
**The 47 was itself one high**, measured rather than assumed: at `a339902`, the commit this work order starts from, the
module declared 30 private functions and 16 private constants, so 46 rather than 47. That is a pre-existing figure
defect in this rule and it is reported as one in `WO-MOK-013` rather than absorbed into the new figure; the
`WO-MOK-011` paragraph above states the same module as 48 total declarations, which is 46 private and 2 public and is
the count that was right. Rule 6's three figures are unmoved and re-measured, at **94** items, **118** `pub` lines and
**24** public fields, with every module's row of that rule's table unchanged: `BAR_ROW_OVERHEAD` is a private constant,
`bar_width` a private function and the entry's row count a local, so nothing this work order adds is a member of the
interface.

`mokiterions-tui/src/state.rs` reads **5** rather than 4 and the total **42** rather than 41, as corrected for
`WO-MOK-018`. One test arrives and none departs:

| Added | Where, and why it cannot move |
|---|---|
| `a_death_carries_no_attribute_the_engine_never_reported_for_its_subject` | `src/state.rs`: `SPEC-MOK-003` rule 10.7's standing rule on the one attribute for which no run produces the negative case — every death a run reaches is preceded by a `survival_changed` record for the same subject — so the absent branch is reachable only by calling the private `ingest` with a death for a subject the observer was never told the survival of. It ingests both subjects in one call, so the case discriminates between the two branches instead of observing a uniform absence, and then selects the unreported subject through `select_for_test` and reads the drawn inspector pane, so the absence is asserted where the rule binds and not only on the derived value |

**Its placement is forced by `ARCH-MOK-002` and not chosen for convenience.** The two ways to assert the same claim
from the public tier are both prohibited patterns of that architecture, by name: making `ingest` `pub(crate)` is
"widening any item's visibility, in either package, in order to reach it from a test", and a fifth `#[cfg(test)]` hook
is foreclosed by "the four hooks on the observer's state type stay as they are". Rule 6's **Growth** clause reaches the
same result from this document's side, since a test is never the requirement that grows the interface. **The placement
costs the case nothing in reach.** From inside the crate `render::draw`, `layout::resolve` and the existing
`select_for_test` hook are all callable, so the case constructs the state through the private `ingest` and then asserts
the absence on the rendered inspector pane as well as on the derived value — which is why this rule's cell for
`src/state.rs` records it as both a hook user and a private-call site. What that leaves outstanding is one clause and
not the case: rule 10.6's obligation that a pair carrying neither value emit no line is unmeasurable at any frame,
because the death branch returns with that line last and a suppressed line occupies the same cells as the pane's
unwritten rows. `VER-MOK-005` discloses that clause as a residual and `WO-MOK-018`'s completion report reports it
rather than leaving it implied.

The same row's private-item count reads **49** rather than 48 — **31** functions and 18 constants — and the
arrival is `action_text`, added by `master`'s `WO-MOK-016` and private to the module. **The figure is the same on
every tree that exists**, measured rather than inferred from a merge: 31 private functions and 18 private constants
on the merged tree, on `master` at `7f4792a`, and on this chain at `efe20e3`. So the 48 this rule recorded is one
low everywhere and not an artefact of either merge, and it is corrected here rather than absorbed into any work
order's arrivals, on the `WO-MOK-013` row's precedent for a figure that "was itself one high". **It is on no record
before the 2026-08-21 rows of the amendment record**: `WO-MOK-016`'s packet names the function in a change-surface
table and nothing in it reaches this count, and the approved row does not state it — so the owner is shown this
figure here for the first time, which is a weaker position than a correction already reported and is stated as such.
**No test of this rule arrives or departs with it**: the four rows read 20, 9, 5 and 8 and the total **42**, as the
paragraph above corrects them, and the hook figure stays at 7 of 20. Rule 6's three figures are untouched by it:
`action_text` is private, and the observer's whole source directory on the merged tree is byte-identical to
`master`'s, which is where that rule's 94 items, 119 `pub` lines and 25 public fields are measured.

`mokiterions-tui/src/state.rs` reads **21** rather than 5 and the total **58** rather than 42, as corrected for
`WO-MOK-020`. Sixteen tests arrive, none departs and none is renamed:

| Added | Obligation it carries |
|---|---|
| `every_total_equals_an_independent_count_on_every_declared_seed` | `VER-MOK-017` O1, the contract's primary oracle: every per-Mokiterion total against a second, independently configured `Simulation`'s own records, at every tick of every declared seed |
| `each_total_the_engine_states_twice_agrees_with_its_second_record` | O2: the three totals the engine states in two places agree with the second one — applied verbs against `action_trace`, eats against `food_consumed`, strikes against `attack_resolved` |
| `the_verb_totals_and_the_rejections_account_for_every_opportunity` | O3: `REQ-MOK-061` clause 2's identity, per Mokiterion and per tick, on all four decision sources |
| `the_killed_total_is_the_engine_s_fatal_strikes_where_both_verbs_occur` | O4: the kill count is the engine's own fatal strikes, on a run in which `attack`, `fight` and a fatal strike all occur |
| `a_dead_mokiterion_s_totals_stop_moving_and_are_never_removed` | O5: `REQ-MOK-061` clause 2, that a dead subject's record is frozen and retained rather than dropped |
| `a_kind_that_never_happened_presents_a_measured_zero_while_an_uncomputed_value_stays_absent` | O6: `SPEC-MOK-003` rule 10 item 7's record-set test, both sides of it on one pane |
| `no_total_moves_with_the_action_trace_flag` | O7: the accumulation reads the decision records and the tick's events, not the optional trace |
| `totals_survive_the_event_buffer_dropping_its_oldest_record` | O8 and P4: the totals are unaffected by truncation, measured on a run that truncates |
| `every_population_total_is_the_independent_sum_at_every_tick` | O9 and P2: `REQ-MOK-062`'s sum against the independent count's own sum, not against the code under test |
| `no_total_ever_decreases_on_any_declared_seed` | O10 and P1: monotonicity, per Mokiterion, at every tick of every declared seed |
| `the_population_pane_states_the_engine_s_own_tick_living_and_death_counts` | O11: the engine's four figures are read from the snapshot and not re-derived |
| `the_death_split_accounts_for_every_death_and_names_no_cause_it_cannot` | O12: the split sums to the engine's death count and attributes no cause the engine did not state |
| `extinction_clears_the_selection_and_presents_the_completed_totals_unprompted` | O14 and scenario 5: `SPEC-MOK-003` rule 10 clause 9 as amended |
| `before_the_first_completed_tick_the_pane_states_so_and_presents_no_figure` | O15: rule 10 clause 8 as amended, in both selection states, with every withheld label asserted absent |
| `every_kind_of_the_action_contract_reaches_the_pane_under_its_own_label` | P3: all eleven kinds reach the pane under their own labels, in the contract's own discriminant order |
| `two_decision_sources_produce_profiles_that_differ_in_the_stated_direction` | acceptance scenario 1: the profile distinguishes two decision sources on the verbs that separate them |

**Rule 8 places all sixteen here for the access they require and not for the subject they cover.** Each names
`Profile`, `ActionKind` or one of `Observer::profile_of`, `Observer::population_profile` and
`Observer::initialized_count`, every one of them `pub(crate)`, so none exists in the build a public-tier target links.
The two ways to move them are both prohibited: widening any of the five is what rule 7's first bullet forbids and what
`ARCH-MOK-002` names as a prohibited pattern, and rule 6's **Growth** clause says a test is never the requirement that
grows the interface. **No fifth `#[cfg(test)]` hook is added**, which rule 7's second bullet and that architecture both
foreclose, and the hook figure of the `src/render.rs` row above is unchanged at 7 of 20. The placement costs these
cases nothing in reach: from inside the crate `render::inspector_text` and `render::draw` are both callable, so the
cases that assert a presented figure assert it on the rendered pane and not only on the derived value — which is why
this rule's cell for `src/state.rs` records the sixteen as reaching a `pub(crate)` item rather than as pane-blind.

The same row's `src/render.rs` private-item count reads **55** rather than 49 — **34** functions and **21**
constants — and the six arrivals are all `WO-MOK-020`'s and all private to the module: the functions `totals`,
`totals_lines` and `selected_totals`, and the constants `TOTAL_LABEL_WIDTH`, `TOTAL_FIGURE_WIDTH` and
`NO_TICK_COMPLETED`. **The 49 is confirmed rather than corrected**, measured at this work order's base commit under
the same method that yields 55 at the candidate: 31 module-level private functions and 18 module-level private
constants there, which is this rule's own figure and not one high or one low. No test of that row arrives or departs
with the six, so it still reads 20. Rule 6's three figures are unmoved and re-measured, at **94** items, **119** `pub`
lines and **25** public fields, with every module row of that rule's table unchanged: everything this work order adds
to either module is `pub(crate)` or private, so no member of the interface is added, removed or reshaped.

`mokiterions-tui/src/verification.rs` is declared from `lib.rs` under rule 5 and contains only the eight tests above.
This is the one place the observer's structure differs from the engine's, where `SPEC-MOK-002` rule 3 leaves
`src/lib.rs` with no test: the engine has no cross-module internal test and the observer has eight. `lib.rs` itself
still declares no test.

`mokiterions-tui/src/authority.rs`, `export.rs`, `layout.rs`, `options.rs` and `spatial.rs` are left with no
`#[cfg(test)]` module, because rule 8 assigns every one of their tests to the public tier.

**Rule 10's figures are re-measured unmoved on the tree that merges `master` at `3ca2028`**, at a total of **58**
with every row of the table above unchanged, `src/render.rs`'s 20 tests and 55 private items included. `master`'s one
arrival is a public-tier test, and its change to `src/options.rs` adds a single private function and no
`#[cfg(test)]` module, so the paragraph above still holds as written: that module declares no internal-tier test.

`mokiterions-tui/src/render.rs` reads **29** rather than 20 and the total **67** rather than 58, as corrected for
`WO-MOK-025`. **Only one of the nine is this work order's**, and the eight are separated from it rather than absorbed,
on the precedent of the rule 11 paragraph that reported an engine figure "one higher … and the one is not this work
order's". Measured at three revisions rather than inferred: `master` at `3ca2028` carries 20, the merge base `2a93914`
carries **28**, and the candidate carries **29**. The eight between the first two are `master`'s own gauge and footer
work, arriving on this chain through a merge this work order did not make, and the correction was owed from that merge
rather than from here. This work order's one arrival:

| Added | Where, and why it cannot move |
|---|---|
| `the_authority_overlay_shows_every_source_at_a_width_that_would_have_cut_one` | `src/render.rs`: `SPEC-MOK-007` rule 18.5's row is now one line per source, and the claim is that a 132-column terminal shows all five rather than four and a truncation. The overlay renders a paragraph that truncates rather than wraps, so the case reaches the drawn cells through the `#[cfg(test)]` snapshot hook and reads them at a width that would have cut the single-line form. No public entry point yields the overlay's cells at a chosen width |

**So the hook figure of that row moves from 7 of 20 to 8 of 29**, measured at both revisions: seven of the twenty-eight
at `2a93914` reach one of the four hooks and eight of the twenty-nine do at the candidate, so the arrival is a hook user
and none of `master`'s eight is. The row's private-item count moves from 55 to **60** — **36** functions and **24**
constants — and **all five of those are `master`'s too**, measured the same way: 34 and 21 on the tree that merged
`3ca2028`, 36 and 24 at `2a93914`, and 36 and 24 unchanged at the candidate. **This work order adds no declaration to
this module at all.** The five `Policy` values the overlay's row iterates are a `const` local to `authority::table`
rather than a module-level item — deliberately, as that function records — so they are in none of these counts and in
rule 6's none either.

**The other three rows are re-measured unmoved**: `src/verification.rs` **9**, `src/state.rs` **21** and `src/main.rs`
**8**, so the total is 29 + 9 + 21 + 8 = **67**. `src/main.rs`'s eight are unmoved although this host gains an option:
`--transcript-path` is validated in `src/options.rs`, which rule 9 sends to the public tier and which still declares no
`#[cfg(test)]` module, and the four cases that reach the private start-up function assert the launch it produces rather
than enumerating the options that reach it. `src/authority.rs` also still declares no `#[cfg(test)]` module, although
this work order changes it: rule 8 sends both of its arrivals to the public tier, one to `tests/authority.rs` and one to
this rule's `src/render.rs` row, and the second is here for the drawn cells rather than for the mapping.

**One coincidence in these figures is stated so that it cannot be mistaken for a check.** `cargo test --lib -p
mokiterions-tui` reports 58 at the merge base, which is this rule's own standing total, and the two 58s are unrelated:
the rule's folds in `src/main.rs`'s eight, which is a separate cargo target that `--lib` never builds, while the
harness's is 28 + 9 + 21 with `src/render.rs` already eight higher than the rule records. At the candidate the two
diverge — 59 from `--lib` and 67 here — and the divergence is the correction rather than a defect. Rule 6's interface
**does move**, to **95** items, **26** public fields and **121** `pub` lines, and rule 6 above records both arrivals with
the requirement behind each; neither is in this tier and neither is reached by any test of it. The `state` row of that
rule's table moves from 47 to 48 with `Observer::with_port`, and this rule's `src/state.rs` row is unaffected, because a
public constructor is not an internal-tier test.

### 11. One invocation

`cargo test` compiles and runs every tier of both packages. No tier requires a feature, an environment variable, a
separate command, a terminal, a pseudo-terminal, or a particular working directory.

**`#[ignore]` is admitted for instruments and for nothing else, as amended 2026-08-24.** An **instrument** is a `#[test]`
function that asserts nothing and exists to produce something a human reads or commits: a capture, a generated constant,
or a regenerated fixture. Three exist, all in the engine, and they are named here so that the class stays closed rather
than becoming a habit — `print_base_commit_entropy_literals` and
`the_four_existing_sources_entropy_state_at_every_tick_boundary` in `mokiterions-core/src/simulation.rs`, which print a
generated constant and a base-commit capture, and `regenerate_the_committed_transcript` in
`mokiterions-core/tests/replay.rs`, which rewrites a committed fixture. **No test that asserts anything carries the
attribute**, so the obligation the paragraph above exists for is undisturbed: every tier's assertions still run on a bare
`cargo test`, because an instrument has none to run. **The attribute is what keeps that true rather than what weakens
it.** Without it `cargo test` would print roughly 20,000 lines of capture output, and one member of the suite would
rewrite a committed file as a side effect — which would make the suite's verdict depend on the order its tests ran in and
would quietly repair a divergence a reader needs to see. Each of the three states its own reason at its own attribute. An
instrument is run by name, and this rule's figures therefore state **two** totals: the tests that exist and the tests
that execute.

`cargo test -p mokiterions-tui` runs the observer's three internal-tier targets and its eight public-tier targets and
nothing else. `cargo test -p Mokiterions` runs the engine's two tiers and nothing else, with the observer excluded
from the build — `SPEC-MOK-002` rule 10 as amended is the authority for that form and it is unaffected here.

The number of executed tests is the same before and after the restructuring: 109 for the observer, 60 for the engine,
169 for the workspace, with the per-tier split of rules 9 and 10. As amended 2026-08-19 the observer's total is **112**
and the workspace's **172**, the engine's 60 unchanged, the three added by `WO-MOK-005`'s conformance to
`SPEC-MOK-003` rule 5 as amended. The clause this paragraph exists for is conservation across a move, not a ceiling on
the corpus: a work order that adds a test corrects these figures here, and one that loses a test has a defect.

As corrected for `WO-MOK-010` and for `master`'s `WO-MOK-007`, the observer's total is **122**, the engine's is
**78**, and the workspace's is **200**. The observer's 10 are the five of rule 9 above and the five of rule 10; the
engine's 18 are 13 in its internal tier and 5 in its public tier, under `SPEC-MOK-002` rules 5 and 7 rather than this
rule, which states no figure of its own for them. The split is recorded here because this paragraph is the only place
the workspace total is stated and 200 is otherwise not reproducible.

No test was lost, and the two work orders' additions are reconciled separately because their evidence was taken on two
different trees. `WO-MOK-010`'s 21 additions and 0 removals are reconciled name by name in its `test-census.txt`, which
was re-taken on 2026-08-19 against `master`'s tip and reads **179 before, 200 after**; `master`'s ten arrivals sit on
its before side rather than among this work order's additions. It was not edited to reach that figure — the earlier
capture, taken at `4f32a9f` against the branch point, reached 190, and a capture is re-run rather than corrected.
`WO-MOK-007`'s 7 additions and 0 removals are the seven named in rules 9 and 10 above, which is where its own evidence
left them uncounted.

As corrected for `WO-MOK-011` and measured on the merge of `master` at `2157f77`, the observer's total is **127**
and the engine's is **85**, so the workspace's is **212**. The twelve arrivals, each measured from the target that
runs it and none departing:

| Where | Tests | What arrives |
|---|---|---|
| `mokiterions-core/src/simulation.rs` | 4 | the name table is the specified twelve, well-formed and as long as the population a run creates; naming draws nothing and reads neither the seed nor the configuration; the reported record carries the name the agent holds; a name is the same value at both ends of a run, including for a Mokiterion that died in it |
| `mokiterions-core/tests/naming.rs` | 3 | the twelve names in identifier order at every declared seed, density, decision source and trace setting; a name reported once and on no other record; the record's field order, with the name first and the trait last |
| `mokiterions-tui/src/render.rs` | 1 | rule 10 above |
| `mokiterions-tui/src/verification.rs` | 1 | rule 10 above |
| `mokiterions-tui/tests/verification.rs` | 3 | rule 9 above |

The engine's split is 54 internal and 31 public, under `SPEC-MOK-002` rules 7 and 8 rather than this rule; it is
recorded here for the same reason the previous paragraph records one, which is that 212 is otherwise not reproducible.
`mokiterions-core/tests/naming.rs` is a new public-tier target of the engine, admitted by `SPEC-MOK-002` rule 8's
closing sentence — "A further file may be added when a further public subject appears" — which grants the addition
without obliging that rule's table, whose own heading calls it the initial arrangement and which `tests/decisions.rs`
already joined the same way. The observer's target count is unchanged, at three internal-tier and eight public-tier
targets. Rule 6's interface is unchanged at **94** items, **118** `pub` lines and **24** public fields, measured
rather than assumed: `Observer::name_of` is `pub(crate)`, and `spatial::agent_glyph` keeps the `&str` parameter and
the `char` return it already had, so nothing in this work order is a member of the interface or changes one's shape.

As corrected for `WO-MOK-013`, the observer's total is **141** and the engine's is **85** unchanged, so the workspace's
is **226**. The fourteen arrivals, each measured from the target that runs it and none departing:

| Where | Tests | What arrives |
|---|---|---|
| `mokiterions-tui/tests/render.rs` | 10 | rule 9 above |
| `mokiterions-tui/src/render.rs` | 2 | rule 10 above |
| `mokiterions-tui/tests/layout.rs` | 1 | rule 9 above |
| `mokiterions-tui/tests/verification.rs` | 1 | rule 9 above |

Every one of the fourteen is the observer's: the engine's 85 is unmoved because this work order changes no file of
`mokiterions-core`, and its 54-internal, 31-public split is unchanged for the same reason. The observer's 141 is rule
10's internal **41** and rule 9's public **100**, which is the cross-check that makes 226 reproducible from the two
tables above rather than only from this paragraph. `tests/layout.rs` rises by one and not by two although two of its
tests change: `the_log_is_ten_rows_only_where_both_thresholds_are_met` is renamed rather than removed, as rule 9
records, and a rename is not an arrival. **No test is lost.** The observer's target count is unchanged, at three
internal-tier and eight public-tier targets, and no target is added: every arrival lands in a file rules 9 and 10
already name.

The figures are measured by `cargo test --workspace` on this branch's implementing tree rather than on a merge, because
this work order touches no file `master` is changing and the engine's total is the same on both sides. Should `master`
add a test before this branch merges, the two rows of the amendment record that faced that case govern: the figures are
re-measured on the merged tree and the superseded ones are kept on the record. Rule 6's interface is unchanged at **94**
items, **118** `pub` lines and **24** public fields, re-measured rather than assumed, as rule 10 above records item by
item.

As corrected for `WO-MOK-019` and measured on the record-stream candidate tree, the observer's total is **127**
unchanged and the engine's is **119**, so the workspace's is **246**. The thirty-four arrivals, each measured from the
target that runs it and none departing:

| Where | Tests | What arrives |
|---|---|---|
| `mokiterions-core/src/simulation.rs` | 14 | the header record's nine fields and the absence of a destination among them; the record shape of every one of the twelve event kinds and of every proposal; null extrema over an empty living population; each cumulative counter against its event count in the text stream; a saturated counter; that no rule reads a counter; every closed domain's members and its size against the alphabet; identical text bytes with a sink; no entropy movement at any tick boundary; the entropy state after initialization and at the thousandth tick; and the two sink-failure paths |
| `mokiterions-core/tests/cli.rs` | 3 | the sink option validated and its value not retained; the help text naming what it writes and replaces; the binary and the parser spelling it the same way |
| `mokiterions-core/tests/records.rs` | 17 | the stream's framing and order; a closed key set; null only where a fact does not exist; no path in any record; every text line reconstructed from its record and the summary line from the run record; every cumulative figure against its record count; each metrics record's internal consistency; byte-identical text with and without a sink and byte-identical records across runs; and the six failure, overwrite and cleanup paths of rule 13 |

The engine's split is 68 internal and 51 public, under `SPEC-MOK-002` rules 7 and 8 rather than this rule; it is
recorded here for the same reason the previous paragraphs record one, which is that 246 is otherwise not reproducible.
`mokiterions-core/tests/records.rs` is a new public-tier target of the engine, admitted by `SPEC-MOK-002` rule 8's
closing sentence in the same way `tests/naming.rs` and `tests/decisions.rs` were. The observer's target count is
unchanged, at three internal-tier and eight public-tier targets, and no observer test is added, removed or moved: this
work order adds a stream the observer does not read. Rule 6's interface is unchanged at **94** items, **118** `pub`
lines and **24** public fields, measured at five revisions rather than assumed — the enumeration is byte-identical to
the one `WO-MOK-011` retained, which is recorded in `WO-MOK-019`'s `interface.txt`. The engine's own interface grows by
one parameter on `execute` and by no item, which `SPEC-MOK-002` rules 4 and 5 as amended are the authority for.

As corrected for `WO-MOK-016` and `WO-MOK-018` together, the observer's total is **145**, the engine's is **122**, and
the workspace's is **267**. **The two work orders are reconciled in one paragraph because neither's figures are statable
without the other's**, which is the same ground on which the 2026-08-19 row of the amendment record corrected two work
orders' figures at once: `WO-MOK-016` left this rule uncorrected at 226, and a paragraph correcting only `WO-MOK-018`
would have to state 267 while accounting for 3 of the 41 tests between them. The forty-one arrivals and the one rename,
each measured from the target that runs it:

| Where | Tests | What arrives |
|---|---|---|
| `mokiterions-core/src/simulation.rs` | 28 | `WO-MOK-016`'s engine additions, under `SPEC-MOK-002` rules 7 and 8 rather than this rule |
| `mokiterions-core/tests/viability.rs` | 3 | `WO-MOK-016`, the same |
| `mokiterions-core/tests/cli.rs` | 2 | `WO-MOK-016`, the same |
| `mokiterions-core/tests/decisions.rs` | 2 | `WO-MOK-016`, the same |
| `mokiterions-core/tests/process.rs` | 1 | `WO-MOK-016`, the same |
| `mokiterions-core/tests/termination.rs` | 1 | `WO-MOK-016`, the same |
| `mokiterions-tui/tests/verification.rs` | 2 | one under `WO-MOK-016` and one under `WO-MOK-018`, both rule 9 above |
| `mokiterions-tui/tests/state.rs` | 1 | `WO-MOK-018`, rule 9 above |
| `mokiterions-tui/src/state.rs` | 1 | `WO-MOK-018`, rule 10 above |

The engine's **122** is 82 internal and 40 public, so `WO-MOK-016` moved it by 37 — 28 internal and 9 public — from the
54 and 31 the `WO-MOK-011` paragraph records. That split is under `SPEC-MOK-002` rules 7 and 8 and is stated here for
the reason the earlier paragraphs state theirs, which is that 267 is otherwise not reproducible. The observer's **145**
is rule 10's internal **42** and rule 9's public **103**, the cross-check that makes 267 reproducible from the two
tables above rather than only from this paragraph. `WO-MOK-018` contributes 3 of the 41 and `WO-MOK-016` the other 38,
which is 41 arrivals against **0** departures. The rename is in neither figure:
`no_shipped_decision_source_has_a_proposal_rejected` becomes
`no_source_confined_to_the_valid_action_list_has_a_proposal_rejected`, as rule 9 records, and a rename is neither an
arrival nor a departure — which is why 226 + 41 = 267 with the rename accounted for and not subtracted. **No test is
lost.** The observer's target count is unchanged, at three internal-tier and eight public-tier targets, and no target
is added.

The figures are measured by `cargo test --workspace` on this branch's implementing tree. `WO-MOK-016`'s 264 is
independently corroborated by `VREC-MOK-017`, which records the same figure at its own candidate commit, so the 38
attributed to it here is not derived from this work order's tree alone. **Rule 6's interface is not unchanged, for the
first time since that rule was written**: it grows by one public field, recorded in rule 6 itself and in the amendment
record rather than only here, so the figures become **94** items unchanged, **119** `pub` lines and **25** public
fields.

As re-measured on the second merge of `master`, at `7f4792a`, into this chain — the first tree in which this work
order's record stream and `master`'s `WO-MOK-018` both exist — the observer's total is **145** and the engine's is
**156**, so the workspace's is **301**. The merge itself adds no test and loses none.

**The figure is established by structure rather than by a census diff.** The merged tree's `mokiterions-tui/src` and
`mokiterions-tui/tests` are byte-identical to `master`'s, and its `mokiterions-core/src` and `mokiterions-core/tests`
are byte-identical to this chain's at `efe20e3`; `git diff origin/master -- mokiterions-tui` and `git diff efe20e3 --
mokiterions-core` are both empty. Neither half is a merge of anything. The observer half's figures are therefore
`master`'s own — rule 10's internal **42** and rule 9's public **103**, which is the **145** the paragraph above
records — and the engine half's are this chain's own, at 96 internal and 60 public, which is the **156** the
paragraph before it records. The workspace total is their sum, 145 + 156 = 301, and 42 + 103 + 96 + 60 = 301 reaches
it from the four tiers instead. **This is a stronger statement than a name-by-name reconciliation and a cheaper one**:
two empty diffs establish that no test on either side was dropped, renamed or moved by the resolution, which is the
failure a census diff is run to exclude, and they establish it for every file at once rather than for the tests alone.

It reconciles from both predecessors, and both directions are stated because each catches a different mistake.
**298 + 3**: the three arrivals `master`'s `WO-MOK-018` brings are `tests/verification.rs`'s
`the_inspector_presents_a_dead_subject_s_final_fear`, `tests/state.rs`'s
`a_death_carries_the_fear_the_engine_last_reported_for_its_subject` and `src/state.rs`'s
`a_death_carries_no_attribute_the_engine_never_reported_for_its_subject`, one in each of the three targets rules 9 and
10 name, and no engine test arrives with them. **267 + 34**: this chain's thirty-four engine additions, unchanged in
number and in name from the 2026-08-20 paragraph above, because nothing in either merge touched them. The first
direction would fail if the resolution had lost an engine test; the second if it had lost an observer test. Neither
does. **No test is lost**, no target is added or removed, and the observer's target count is unchanged at three
internal-tier and eight public-tier targets.

Rule 6's interface is **94** items, **119** `pub` lines and **25** public fields on this tree, which is the paragraph
above's approved figure and follows from the observer half's byte-identity with `master` rather than from a fresh
enumeration here. The engine's own interface reads 49 items and 43 public fields, unmoved by this merge, under
`SPEC-MOK-002` rules 4 and 5 rather than this rule.

As corrected for `WO-MOK-020` and measured at its candidate commit, the observer's total is **175** and the engine's
is **157**, so the workspace's is **332**. The thirty arrivals, each measured from the target that runs it and none
departing:

| Where | Tests | What arrives |
|---|---|---|
| `mokiterions-tui/src/state.rs` | 16 | rule 10 above |
| `mokiterions-tui/tests/render.rs` | 7 | rule 9 above |
| `mokiterions-tui/tests/verification.rs` | 7 | rule 9 above |

The observer's **175** is rule 10's internal **58** and rule 9's public **117**, which is the cross-check that makes
332 reproducible from the two tables above rather than only from this paragraph. **No test is lost**, no target is
added or removed, and the observer's target count is unchanged at three internal-tier and eight public-tier targets:
every arrival lands in a file rules 9 and 10 already name. The figures are measured by `cargo test --workspace
--locked` on this work order's implementing tree, and the four tier figures are independently reproduced by counting
`#[test]` per file, which agrees with the harness output target by target.

**The engine's 157 is one higher than the 156 the paragraph above records, and the one is not this work order's.**
It is stated here rather than absorbed, on the precedent of the rule 10 row that reported a figure "itself one high".
`mokiterions-core/src/simulation.rs` carries **97** internal-tier tests at this candidate and 96 on the tree the
2026-08-21 paragraph measured; the arrival is `WO-MOK-017`'s, added by commit `26ae6ba` implementing `REQ-MOK-060`,
and its public tier is unmoved at 60. `WO-MOK-020` changes no file of `mokiterions-core`, so none of the thirty
arrivals above is an engine test and the engine's figure moves here only because this paragraph re-measures it.
**This is not a contradiction of an approved figure**: the 156 is true of the tree it names, at the second merge of
`master` at `7f4792a`, and this rule's own closing sentence — that a work order which adds a test corrects these
figures here — is what leaves the correction owed. What it does mean is that the correction was owed from
`WO-MOK-017`'s closure and was not made then, so the owner is shown the engine's 157 here for the first time. Rule 6's
interface is **94** items, **119** `pub` lines and **25** public fields, re-measured at this candidate rather than
assumed, as rule 6 and rule 10 above record item by item.

**Re-measured on the tree that merges `master` at `3ca2028`**, the observer's total is **176** and the engine's is
**157**, so the workspace's is **333**. The observer's 176 is rule 10's internal **58**, unmoved, and rule 9's public
**118**, which is the same cross-check as above and makes 333 reproducible from the two tables rather than only from
this paragraph. **One test arrives and it is not this work order's**:
`the_shared_entries_are_the_engines_own_words` in `tests/options.rs`, from `master`'s usage-text work. **The engine is
unmoved at 157** — 97 internal and 60 public — and that is measured rather than inferred from the total: no `#[test]`
attribute and no `fn` declaration is added or removed in either `mokiterions-core` file the merge touches, which
changes what those tests assert and not whether they exist. **No approved figure is contradicted**: the 175 and the
332 are true of `WO-MOK-020`'s candidate `f633eda`, which `VREC-MOK-023` binds, and the paragraph above keeps them.
Measured by `cargo test --workspace --locked`, which reports **333 passed, 0 failed, 0 ignored** on this tree, and
independently by counting `#[test]` per file, which agrees target by target.

As corrected for `WO-MOK-025` and measured at its candidate commit, the observer's total is **197** and the engine's is
**221**, so the workspace's is **418** — of which **415** execute, the other three being the instruments the opening of
this rule admits. The observer's 197 is rule 10's internal **67** and rule 9's public **130**, which is the cross-check
that makes 418 reproducible from the two tables above rather than only from this paragraph. The seventy-five arrivals,
each measured from the target that runs it and none departing:

| Where | Tests | What arrives |
|---|---|---|
| `mokiterions-core/src/simulation.rs` | 46 | the request's four blocks in the cacheable order and what each block may and may not say; the transcript's two record kinds, its head and its escaping round trip; one port lent for the run, its accumulated cost and its integer ceiling arithmetic; the fallback, its count and its distinction from a proposed `wait`; a replay reproducing the recorded run's bytes and the refusals a wrong prefix, a wrong opportunity, an early end or a surplus tail reach; that the four existing sources draw what the base commit drew; and the two entropy instruments — all under `SPEC-MOK-002` rules 7 and 8 rather than this rule |
| `mokiterions-core/tests/replay.rs` | 10 | the committed transcript replayed through the binary into the recorded run's bytes, its own coverage of what the reading cases rest on, that it carries no credential and no conversion, the two refusals and their exit statuses, and the instrument that regenerates it — the same, and a **new** public-tier target of the engine |
| `mokiterions-core/tests/cli.rs` | 7 | the transcript option validated and its value not retained, the binary and the parser spelling it the same way, the usage text stating when it is required and when it is refused, both refused combinations with their exit statuses, and help surviving both — the same |
| `mokiterions-core/tests/process.rs` | 1 | that the library never opens the transcript it was told about, observed from outside the process — the same |
| `mokiterions-tui/tests/options.rs` | 5 | rule 9 above |
| `mokiterions-tui/tests/replay.rs` | 4 | rule 9 above, and a **new** public-tier target of the observer |
| `mokiterions-tui/tests/authority.rs` | 1 | rule 9 above |
| `mokiterions-tui/src/render.rs` | 1 | rule 10 above |

**Sixty-four of the seventy-five are the engine's and eleven the observer's**, so the engine moves 157 → 221 and the
observer 186 → 197. **The observer's before-figure is 186 and not the 176 the paragraph above records**, and the ten
between them are `master`'s rather than this work order's: `tests/verification.rs` reads 29 at the tree that merged
`3ca2028` and **31** at the merge base `2a93914`, and `src/render.rs` reads 20 and **28**, which is the +2 and +8 rule 10
above separates arrival by arrival. Both are measured at both revisions rather than inferred. **This is not a
contradiction of an approved figure**: the 176 and the 333 are true of the tree they name, and this rule's own closing
sentence — that a work order which adds a test corrects these figures here — is what leaves the correction owed. The
engine's 157 is unmoved across that same span and that too is measured rather than inferred from an unchanged total: no
`#[test]` attribute is added or removed in any `mokiterions-core` file between `3ca2028` and `2a93914`. So 343 + 75 = 418
from the merge base, and 333 + 10 + 75 = 418 from the approved figure, and both directions are stated because the first
would fail if an arrival had been double-counted and the second if `master`'s ten had been.

The engine's **221** is 143 internal and 78 public, under `SPEC-MOK-002` rules 7 and 8 rather than this rule; it is
recorded here for the reason the earlier paragraphs record a split, which is that 418 is otherwise not reproducible. **The
three instruments are all the engine's** — two in its internal tier and one in `tests/replay.rs` — so the engine executes
218 of its 221 and the observer executes all 197. `mokiterions-core/tests/replay.rs` is admitted by `SPEC-MOK-002` rule
8's closing sentence, in the same way `tests/naming.rs`, `tests/decisions.rs` and `tests/records.rs` were, and it takes
the engine's public-tier target count from eight to nine under that rule rather than this one.

**The observer's public-tier target count moves from eight to nine, for the first time since this rule was written**, and
rule 9 above is the authority: `mokiterions-tui/tests/replay.rs` arrives under that rule's own "A further file may be
added when a further public subject appears". Every sentence of this rule that states the count as eight is superseded by
this paragraph and kept on the record; the sentence above naming "its eight public-tier targets" for
`cargo test -p mokiterions-tui` reads **nine** as of this correction, and the observer's internal-tier count is unchanged
at three. **No test is lost**: seventy-five arrive, none departs and none is renamed. The figures are measured by
`cargo test --workspace --locked`, which reports **415 passed, 0 failed, 3 ignored** across the twenty-two test binaries
it builds — eleven per package, of which the engine's `src/main.rs` declares no test and both packages' doc-test targets
declare none — and independently by counting `#[test]` per file, which agrees binary by binary and yields 418 because the
count reads the attribute and not the run. **Rule 6's interface is not unmoved**: it reads **95** items, **26** public
fields and **121** `pub` lines at this candidate, against the 94, 25 and 119 the paragraphs above record, and rule 6 itself
carries the two arrivals, the requirement behind each and the three measured alternatives. No test of either tier is the
reason for either, which is that rule's **Growth** clause satisfied rather than waived.

**Re-measured at `bce4229` within the same work order**, after four further commits closing verification cases the
paragraph above did not cover, the observer's total is **198** and the engine's is **227**, so the workspace's is **425**
— of which **422** execute, the three instruments being the same three. The observer's 198 is rule 10's internal **67**,
unmoved, and rule 9's public **131**; the engine's 227 is `SPEC-MOK-002` rule 7's internal **143**, unmoved, and rule 8's
public **84**. **The paragraph above is not contradicted**: its 418, 415, 197, 221, 130 and 78 are true of `d8ff20e`, the
candidate it was written at, and they are kept on the record. This paragraph exists because this work order's candidate
moved after them, and this rule's closing sentence — that a work order which adds a test corrects these figures here —
is owed to the tree that is finally offered rather than to the first one measured. Seven arrive, none departs and none
is renamed:

| Where | Tests | What arrives |
|---|---|---|
| `mokiterions-core/tests/replay.rs` | 5 | `VER-MOK-018`'s five cases that are stated over the declared seed set or over both of rule 20.5's hosts, each now driven where it is stated: replay identity at all five seeds crossed with tracing on and off, the replay refused at another density, the source named once before any tick at every seed, no record form unique to the port aggregated over ten cells, and both of rule 20.5's doors carrying the port |
| `mokiterions-core/tests/no_outcome_threshold.rs` | 1 | case **L26**, that no outcome threshold exists for the model-backed source anywhere in the verification suite — a **new** public-tier target of the engine, reading the suite's own source as text because an absence produces no behaviour to observe |
| `mokiterions-tui/tests/replay.rs` | 1 | case **L31**, the observer replaying this source to the transcript's horizon under operator control with the roster, map, event log, inspector, filter and export read in their own regions and compared against `social` |

**The engine's public-tier target count moves from nine to ten** and `SPEC-MOK-002` rule 8's closing sentence is again
the authority, on the same ground `tests/replay.rs` was admitted by the paragraph above: `tests/no_outcome_threshold.rs`
requires no access to the engine at all and its subject is the suite rather than the engine, which is why it is a target
of its own rather than a case inside one. The observer's public-tier target count is unmoved at nine.
The figures are measured by `cargo test --workspace --locked`, which reports **422 passed, 0 failed, 3 ignored** across
the **twenty-three** test binaries it builds — twelve the engine's and eleven the observer's, of which the engine's
`src/main.rs` declares no test and both packages' doc-test targets declare none — and independently by counting
`#[test]` per file, which agrees target by target and yields 425 for the same reason as above, that the count reads the
attribute and not the run. **Rule 6's interface is unmoved** at the **95** items, **26** public fields and **121** `pub`
lines the paragraph above records, and that is measured rather than inferred from an unchanged total: the four commits
change no file under either package's `src/`, so there is no line on which a `pub` item could have arrived.

**Corrected for `WO-MOK-026` and measured at its candidate `7f9e20a`**, the observer's total is **202** and the engine's
is **276**, so the workspace's is **478** — of which **475** execute, the three instruments being the same three. The
observer's 202 is rule 10's internal **67**, unmoved, and rule 9's public **135**; the engine's 276 is `SPEC-MOK-002`
rule 7's internal **174** and rule 8's public **102**. **Neither unmoved figure is inferred from an unchanged total.**
Rule 10's 67 is re-measured file by file — `src/main.rs` 8, `src/render.rs` 29, `src/state.rs` 21, `src/verification.rs`
9 — which matters because this work order changes three files under `mokiterions-tui/src/`, and the baseline it is
unmoved against is this branch's merge base `d96cced`, at which every one of those four files carries the same figure it
carries at `bce4229`. **The paragraph above is not contradicted**: its 425, 422, 198, 227, 131 and 84 are true of
`bce4229`, and they are kept on the record.

**`main` moved after `bce4229` and it added no test**, which is measured rather than assumed: every `mokiterions-core`
and `mokiterions-tui` source file's `#[test]` count is identical at `bce4229` and at `d96cced`, so this branch's baseline
is that paragraph's 425 and the whole of the arrival is this chain's. Fifty-three arrive, none departs and none is
renamed — 4 additions and 0 removals for the observer and 49 and 0 for the engine, reconciled by qualified name rather
than by count:

| Where | Tests | What arrives |
|---|---|---|
| `mokiterions-core/src/simulation.rs` | 31 | the request as one line of the five documented fields and the schema offering exactly the verbs the engine admits; the response read, framed and written into a record, including one terminated with a carriage return and one whose closure is an error rather than an empty answer; every response the grammar refuses counted as a fallback and billed as well as counted; a reported count that is unreported or unusable held absent rather than zero, and an exchange that reported nothing still an exchange; the declared prices crossing into the arithmetic in the stated order; the ceiling stopping the run inside the first tick, stopping the retrying before its bound, leaving both streams complete to the tick reached, and never halting a replay whatever the recorded run spent; a transient failure attempted four times for one counted fallback, one that then succeeds for none, only the two transient kinds attempted more than once, a failed pipe not retried, and a transcript holding a retried exchange unreplayable; and rule 15's run record — absent for a deterministic source, bound to the first reported model and level and never replaced, surviving a connector that stops answering, needing both fields and a non-empty model, and reporting what answered even for a response refused on its verb — all under `SPEC-MOK-002` rules 7 and 8 rather than this rule |
| `mokiterions-core/tests/connector.rs` | 13 | the live path observed from outside the process: the spawn behind both of rule 13's gates, with no connector started without the live selection and no credential refusing on the first exchange; the credential value reaching no produced byte; a connector that cannot be started leaving no transcript, one that exits badly reported without changing the exit code, and one that dies mid-run leaving a run of fallbacks; an existing transcript destination refused and left untouched; a transport failure retried with every attempt recorded, exhausted retries a counted fallback with the run continuing, and the resulting transcript refused by its own replay; a response naming neither model nor level a fallback; and the ceiling stop leaving the record stream behind — a **new** public-tier target of the engine |
| `mokiterions-core/tests/cli.rs` | 5 | `--prices` validated with its four values retained, read in the stated order so that a transposition is a different run, refused under every source that decides for itself, its usage entry stating its unit and its order, and a live run with no prices refused before any tick |
| `mokiterions-tui/tests/options.rs` | 4 | rule 9 above |

**The engine's public-tier target count moves from ten to eleven** and `SPEC-MOK-002` rule 8's closing sentence is again
the authority, on the same ground `tests/replay.rs` and `tests/no_outcome_threshold.rs` were admitted by the two
paragraphs above; that rule already records `tests/connector.rs`'s arrival in its own amendment record. The observer's
public-tier target count is unmoved at **nine** and its internal-tier count at **three**. The figures are measured by
`cargo test --workspace --locked`, which reports **475 passed, 0 failed, 3 ignored**, and independently by counting
`#[test]` per file, which agrees target by target and yields 478 for the reason the paragraphs above give, that the count
reads the attribute and not the run. **Rule 6's interface is unmoved** at the **95** items, **26** public fields and
**121** `pub` lines, measured at this candidate module row by module row; rule 9 above records what the three changed
observer source files add and why none of it is a member.

**The test-binary count reads twenty-five, and only one of the two arrivals is this chain's.** It is **fourteen** the
engine's and **eleven** the observer's, against the twenty-three the paragraph above records as twelve and eleven. One
arrival is `tests/connector.rs`, this work order's and counted among the fifty-three above. The other is
`canned-connector`, a second `[[bin]]` target of the engine package at `tests/support/canned_connector.rs`, which
**declares no test** and so adds nothing to any figure this rule states while still being a binary `cargo test` builds
and runs. It reached `main` before this branch, at `1046f23`, under this same work order's item 2 and by way of the owner
decision `WO-HUP-002` records, and the path is `tests/support/` rather than `tests/` precisely so that Cargo does not
also discover it as an integration-test target. **That correction was owed from `1046f23` and was not made there**: its
message reasons that "a `[[bin]]` adds no test, so `SPEC-MOK-004` rule 11's counts are conserved and need no correction
here", which is true of every test figure and not of the binary count this rule states beside them. It is stated with its
origin rather than absorbed into this chain's arrival, on the precedent of the engine's 157 in the 2026-08-22 row above.
**`SPEC-MOK-002` rule 1 has not been amended for it and still reads
"exactly two targets" and "No third target"**; that conflict is recorded as a defect with this stage's evidence, at
`evidence/WO-MOK-026/defect-third-target-unamended.md`, and is not repaired here — it is a change of substance in
another rule of another specification, which this work order's stop-and-escalate condition 6 reserves to the owner. **It
is named here rather than left implicit** because this rule's own figure is what makes it visible: a reader who
reconciles twenty-three to twenty-five finds a target no approved rule admits.

An observer test asserts a rendering claim against an in-memory character buffer. A test requiring a terminal, a
pseudo-terminal, a screenshot, or a recording is not admissible in either tier; `SPEC-MOK-003` is the authority and
this rule restates it because the public tier is a new place such a test could be written.

### 12. Test content preservation

A relocated test keeps its assertions verbatim. Only the path by which it reaches the code changes — a `use` of the
library target in place of `use super::*` or `use crate::…`. A relocated test whose assertions cannot survive the
move is a rule 8 misclassification and stays in the internal tier.

No assertion is weakened, generalized, replaced by a looser observation, split, merged or renamed, in either tier,
in either package, **by the `WO-MOK-006` restructuring**. A helper used only by relocated tests moves with them; a
helper used by tests in both tiers is duplicated or shared under the delegation in *Explicitly unspecified
decisions*, and its behavior does not change.

The scoping in bold is the 2026-08-19 amendment, and it is the same scoping rule 13 receives for the same reason.
This rule is about a move: its first paragraph says so in every sentence. Read without the scope, its second
paragraph forbids any later work order from ever renaming a test in either package, which is a freeze on test
maintenance that no requirement asks for and that this rule was not written to impose. A later work order that
changes an assertion is governed by the specification and contract it works under; if it weakens one, that is a
defect there, and this rule is not what detects it.

### 13. Behavior preservation

The restructuring is equivalence-preserving for both packages.

For identical arguments and identical seed, the engine emits byte-identical output, reaches an identical final state,
and returns an identical exit code, with and without `--trace-actions`, under both decision sources, at every
declared density. No simulation constant, event field, event order, summary field, exit code, diagnostic message, or
byte of `USAGE` changes. Every case, invariant and check in `VER-MOK-001`, `VER-MOK-002` and `VER-MOK-004` remains
covered.

For identical arguments, identical seed and identical viewport, the observer presents identical frames, writes
byte-identical exports, responds to identical key bindings, resolves identical panes, draws identical glyphs and
returns identical exit codes. Every case, invariant and check in `VER-MOK-005` remains covered. The clause read
"selects identical layout tiers" until the 2026-08-19 amendment; it is the same obligation, named the way
`SPEC-MOK-003` rule 5 as amended names it, and it binds this restructuring rather than later work that amends what a
viewport yields.

Relocated content is compared, not reviewed. Each moved file's content is byte-identical to its content at the
predecessor commit apart from path references the move itself requires, and the comparison is performed against the
predecessor commit rather than asserted, because a move produces a diff in which every line is new.

### 14. Operator commands

Each form below resolves to the same target and produces the same output as at the predecessor commit, and each is
executed rather than assumed. A virtual workspace root does not resolve a bare command the way a package root does,
and this rule exists because that is the change's one real operator-facing risk.

| Command | Required outcome |
|---|---|
| `cargo run --bin Mokiterions` | runs the engine binary; the first line of `USAGE` is byte-identical to the verified text |
| `cargo run -p mokiterions-tui` | runs the observer binary |
| `cargo build -p Mokiterions` / `cargo build -p mokiterions-tui` | builds that package alone |
| `cargo test` / `cargo test -p <package>` | rule 11 |
| `cargo tree -p Mokiterions` | resolves to exactly the set `SPEC-MOK-002` rule 13 declares for the engine package plus the package itself, which is the engine package alone while that set is empty; this is the check that the layout cost the engine's dependency surface nothing *(amended 2026-08-20 from "the engine's empty dependency table"; the required outcome is the same today)* |
| `cargo tree -p mokiterions-tui` | resolves to the observer, the engine, and the `ratatui` graph fixed by `SPEC-MOK-003` |
| `cargo fmt` / `cargo clippy --all-targets --all-features -- -D warnings` | pass workspace-wide with no `allow` attribute added and no narrowing of the invocation |

A form that no longer resolves is corrected under rule 2 and never by renaming a package or a target.

## Error and recovery behavior

- A public-tier file that fails to compile because it names an item rule 6 does not expose is a rule 8 signal:
  reclassify the test to the internal tier. It is never grounds for widening an item, and rule 6 does not grow to
  admit it.
- A test that compiles in the public tier only after its assertion is loosened is a rule 12 defect. The correction
  is that the test returns to the internal tier in its original form.
- A private item left with no user by the relocation, surfacing as a dead-code warning under the lint gate, is a
  rule 8 signal that the test which used it was misclassified. The correction is to return that test to the internal
  tier, never to delete the item and never to add an `allow` attribute.
- A clippy failure caused by a target name is corrected under rule 4, never by narrowing the lint invocation.
- A test-count difference before and after, in either direction, is a failure: a test was lost or silently added.
- Any observable difference from either package's verified baseline is a defect in the restructuring, not a new
  baseline.
- A verified record or a retained evidence file that names an old path is not edited. It is bound to its commit by
  its verification record; a superseding mapping is produced as work-order evidence instead.

## Data and interface contracts

The observer's public interface carries presentation values, pure functions of values, read-only accessors and the
export writer. It reaches no engine mutation path: it cannot construct an `Observation`, cannot implement or invoke
a `DecisionSource`, and holds no mutable borrow of engine-owned state. The engine's interface is unchanged, so
`SPEC-MOK-002`'s *Data and interface contracts* section continues to hold verbatim with its paths read under
`mokiterions-core/`.

The observer's own state type keeps sole ownership of its snapshot, its event buffer and its selection. Making its
already-public accessors reachable from outside the crate hands out the copies they already returned inside it, and
grants no capability that did not exist at the predecessor commit — which is the whole of why rule 6 can admit
nothing and still be sufficient.

The dependency direction is unchanged and unchangeable: the observer depends on the engine by path, the engine
depends on nothing, and rule 3 preserves both. Rule 1's directory layout carries no dependency meaning; a package's
directory does not make it a dependency of anything.

## Security and privacy properties

- No network access, credential read, environment read, wall-clock read, or filesystem read is introduced. Neither
  package gains a dependency.
- Rule 7 is the security-relevant rule of this specification. It preserves `REQ-MOK-004` and `ADR-MOK-001` by
  keeping every prohibited engine item private and by keeping the observer's four hooks compiled out of every
  shipped artifact, in test builds as well as release builds.
- The trust boundary is unmoved. A decision source still receives immutable observations and returns typed
  proposals. `Observation` and `DecisionSource` stay among the ten names `SPEC-MOK-002` rule 6 keeps private, and
  the observer's public interface reaches neither.
- World authority is unchanged. Nothing on the observer's interface can mutate a world, and the observer still
  calls exactly one mutating engine method.
- Export behavior is unchanged. An operator-supplied export path is data, is validated as a string only, is never
  interpreted as code, is never used to read, and is not opened at start-up. No credential, secret, environment
  variable, absolute path or wall-clock value appears in a frame or an export.
- A directory move creates no new trust boundary and no new artifact. Two packages remain, one repository, one
  version, one candidate commit.

## Performance and capacity

Runtime behavior is unchanged; rule 13 requires it. Compile-time cost grows: the observer's binary target links its
library target, and each public-tier file becomes an additional test target, so the observer contributes eleven test
binaries where it contributed one, and `target/` grows. Per-tick work, per-frame work, memory use, frame budget,
input latency and output volume are unaffected. A first build after the move is a full rebuild, because every
package path changes.

## Observability

Unchanged. The engine's event stream, action-trace lines, summary line and exit codes, and the observer's frames,
panes, glyphs, diagnostics, exports and exit codes are exactly as verified. This specification adds no log, metric
or diagnostic, and the observer's diagnostic report is untouched.

## Compatibility and migration

- `SPEC-MOK-002` requires amendment wherever it names a root-relative path — its *Inputs*, rule 1's target table and
  its 2026-08-18 note, rule 3, rule 4, rule 5's `grep` check, rule 8's file table and rule 9's locations — and
  wherever it implies the engine package is at the root. Its rules bind unchanged in substance; only the paths move.
  Its rules 7 to 10 remain the engine's test-placement contract, and this specification is the observer's.
- `SPEC-MOK-003` requires amendment in its *Component layout*, whose tree and clause 3 fix the engine's sources at
  the root; in *Data and interface contracts* rule 2, whose parenthetical justifies keeping `Simulation::run` public
  by appealing to a component layout that "forbids" relocating the engine's sources — the conclusion holds, because
  `run` is the `REQ-MOK-010` whole-run entry point that the engine's own binary calls, but the reasoning and the
  path reference are stale; and in *Explicitly unspecified decisions*, both in the entry that withholds the package
  layout and in the entry that delegates test organization.
- `ARCH-MOK-002` requires amendment in the component that calls the observer host "the new binary", in the
  *Testability without a terminal* quality attribute, in its required and prohibited patterns, and in its
  conformance checks, and it gains `REQ-MOK-028` among the requirements it addresses and this specification among
  those it conforms to. `ADR-MOK-004` states the amendments; `WO-MOK-006` makes them an approval precondition.
- `ARCH-MOK-001` needs no amendment. It names no source path, and every quality attribute and conformance check it
  states about the engine package — one library target, one thin binary target, an empty dependency table — holds
  unchanged after the move. *(Amended 2026-08-20. That was true of this specification's change and stays true of it:
  the move required nothing of `ARCH-MOK-001`. `ADR-MOK-006` amended that architecture afterwards for an unrelated
  reason, replacing the empty-dependency-table check with a comparison against the declared set. This bullet is a
  statement about the relocation, not a claim that the check is still worded that way.)*
- `REPOSITORY_CONTEXT.md` requires updating in its *Commands* and *Architecture* sections, and in the sentence that
  states the two-tier test convention repository-wide while citing engine-scoped authority for it. That sentence's
  disagreement with the code is what `WO-MOK-005` disclosed; after this specification it is true of both packages,
  citing `SPEC-MOK-002` rules 7 to 10 for the engine and this specification for the observer.
- Records bound to commits are not re-opened. `VREC-MOK-001` through `VREC-MOK-005` and the retained evidence under
  `WO-MOK-001` through `WO-MOK-005` name paths that this specification moves, and they stay as written.
- Reversal is a directory move and the deletion of one file. No behavior in either package depends on the outcome,
  which is what makes this specification safe to conform to in one work order.

## Examples and counterexamples

**Example.** The cross-cutting non-perturbation test drives the observer over a fixed number of ticks and asserts
that a second engine run from the same seed produces a byte-identical text stream. It reaches the engine through the
engine's public interface and the observer through already-public items, and names no hook. Rule 8 places it in the
public tier, rule 9 places it in `tests/verification.rs`, and rule 12 keeps its assertion verbatim.

**Example.** The layout tests assert which panes a given viewport yields. Every item they named was among the
`layout` module's 13 public items at the predecessor commit, so all seven moved to `tests/layout.rs` and `layout.rs`
is left with no `#[cfg(test)]` module at all. The same holds of the three tests added there since, against the 10
items rule 6 records now.

**Counterexample.** A rendering test asserts the exact form of a bar row by calling a private drawing helper and
reading two private constants. Making the helper and the constants `pub` so the test can move to `tests/render.rs`
violates rule 7, misapplies rule 8, and is the widening `INT-MOK-005` prohibits. The test stays in
`src/render.rs`.

**Counterexample.** A test builds a world with no standing resources by calling the hook that replaces the
snapshot, then asserts the empty-resource pane. Removing the hook's `#[cfg(test)]` attribute so the test can move
violates rule 7 and installs the test-support seam `SPEC-MOK-002` rule 6 and `ARCH-MOK-001` both deny. The test
stays in the internal tier.

**Counterexample.** Replacing that rendering test's assertion with "the rendered pane contains a digit" in order to
place it in the public tier violates rule 12. The relocated test is weaker, so the relocation is a defect rather
than a move.

**Counterexample.** Moving the engine to `mokiterions-core/` and renaming the package to `mokiterions-core` at the
same time violates rule 3 and rule 14: `cargo tree -p Mokiterions` would fail, `cargo run --bin Mokiterions` would
resolve to nothing, and the first line of `USAGE` would change. The directory name and the package name are
independent, and only the directory moves.

**Counterexample.** Adding a `[workspace.dependencies]` entry to the root manifest for a crate that is not a declared
entry of **both** packages' sets violates rule 2, and the member that inherits it with `workspace = true` violates
`SPEC-MOK-002` rule 1 or `SPEC-MOK-003`'s *Declared dependency set* as well, because its resolved graph then holds a
crate its declaration does not. Keying a version once is the table's purpose; keying a version for a crate only one
package declares, or for a crate neither declares, is how an undeclared dependency enters a workspace without either
package's manifest naming it. *(Amended 2026-08-20. This counterexample read: adding `[workspace.dependencies]` "so both
packages could share a version key violates rule 2 and rule 7, and would give the engine package a dependency table
that is not empty in substance, against `SPEC-MOK-002` rule 1 and `ARCH-MOK-001`." `ADR-MOK-006` reverses exactly that:
sharing a version key is now the admitted form. The replacement keeps the shape of the old one — a manifest edit that
looks convenient and defeats a per-package declaration — and still bites.)*

## Explicitly unspecified decisions

- Ordering of items within each source or test file, and whether public-tier helpers are duplicated per file or
  shared through a `tests/common/` module in either package.
- The order in which the seven `pub mod` declarations appear in the observer's `lib.rs`, provided all seven are
  present and the `verification` declaration stays `#[cfg(test)]`.
- Whether the observer's binary target imports the library's modules individually or as a group, provided it
  declares no module of its own.
- Whether the internal tier's existing helper functions are kept, renamed or consolidated, provided no assertion
  changes.
- Doc comments, internal comments, and non-authoritative developer notes, including whether each manifest keeps the
  specification-citing comments it carries today.
- Whether the observer's module files are later split into further private modules. Rules 6 and 7 constrain
  visibility, not file count.
- The order in which the relocation, the library target and the test moves are performed within the implementing
  work order, provided every rule holds at the candidate commit.
