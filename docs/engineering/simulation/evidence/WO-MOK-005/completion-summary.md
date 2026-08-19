# Completion summary — WO-MOK-005

Re-captured on 2026-08-19, after the repository owner reported that the observer's adaptive layout
does not present the left panel at some terminal sizes, approved the two amendments that correct it
and directed the implementation. What changed here is the component description, every figure in the
gate results, three of the fifteen disclosures and one of the two smaller notes, and two new
disclosures at the end. The disclosures that later work resolved are marked resolved and dated
rather than deleted, because the disclosure record is what the owner reads and a disappearing item
reads as one that was never raised.

## Final affected components

**`Mokiterions`** — the engine, in `mokiterions-core/` since `WO-MOK-006` moved it there, with its
package name and both target names unchanged. `src/simulation.rs` gained the observation surface and
the snapshot types, and its record-emitting code was refactored to build `Event` values that both
hosts consume. `Cargo.toml` at the repository root became the workspace root, and the package's own
`[dependencies]` table is still empty. **No file in the engine package differs from `origin/master`
at 05dc6ac** — not one byte, tracked or untracked — so no simulation rule and no engine test changed
under the 2026-08-19 work either. `additivity-proof.txt` measures it with one command.

The library target `mokiterions` and the `pub mod cli` / `pub mod simulation` declarations were not
created by this work. `WO-MOK-003` created them on `master`, and this work grew what they expose.

**`mokiterions-tui`** — new package, 5,920 lines. Ten modules under `src/`, 3,738 lines: `render.rs`
(every pane, 1,276), `state.rs` (observer state and key handling, 754), `verification.rs`
(`VER-MOK-005`'s cross-cutting internal cases, 624), `main.rs` (the loop and start-up, 425),
`layout.rs` (the pane thresholds and regions, 187), `spatial.rs` (the world-to-canvas mapping, 178),
`options.rs` (argument parsing, 146), `authority.rs` (the event-type-to-requirement mapping, 65),
`export.rs` (the event buffer and file write, 50), `lib.rs` (the module declarations, 33). Eight
files under `tests/`, 2,182 lines, which is the public tier `SPEC-MOK-004` rule 9 fixes.

The split into a library target and a binary target, and the eight-file public tier, are
`WO-MOK-006`'s work rather than this one's; disclosure 15 below is why that work order exists.

**`Cargo.lock`** — from one entry to 182.

**Gate results on the final tree:** `cargo fmt --all -- --check` exit 0; `cargo clippy --workspace
--all-targets --all-features -- -D warnings` exit 0, and again exit 0 for `-p Mokiterions` alone;
`cargo test --workspace` 172 passed, 0 failed (60 engine + 112 observer). `cargo test -p
Mokiterions` alone: 60 passed — 37 inline in `src/simulation.rs` and 23 across the five files in
`mokiterions-core/tests/`, the same 60 `WO-MOK-004` recorded. `cargo build --workspace` and
`cargo build -p Mokiterions` both exit 0. Retained in `static-checks.txt` and `test-run.txt`.

**Harness gates on the final tree:** `harnessctl validate` and `scripts/validate_engineering_artifacts.py`
both PASS, 68 artifacts, 0 errors, 0 warnings, all four planes clean. `harnessctl doctor` PASS, every
managed file unchanged. `harnessctl preflight --work-order WO-MOK-005 --phase review` PASS.
`scripts/generate_harness_dashboard.py --root .` PASS, 68 artifacts, 211 relations, 0 errors, 8
warnings. The same generator run against a clean extraction of `origin/master` at 05dc6ac reports 6
warnings, so this work adds exactly two, both of them the same finding twice: *"ARCH-MOK-002 predates
newer declared conforms_to target SPEC-MOK-003"* and the equivalent for `SPEC-MOK-004`. They exist
because the two amendments moved both specifications' `updated` date to 2026-08-19 while
`ARCH-MOK-002` stays at 2026-08-18. See the last paragraph of disclosure 16.

The observer's 112 is 109 plus the three `tests/layout.rs` gains the amended rule 5 requires. The
count is a net figure over three departures and six arrivals; `additivity-proof.txt` itemizes both
sides and states which approval covers each.

## Disclosures

`WO-MOK-005` requires the completion report to disclose anything the work found that the artifacts do
not say. Seventeen items, grouped by what a reader should do about them. Items 1 to 15 were found
on or before 2026-08-18; 16 and 17 come from the layout amendment. Line citations were re-resolved
on 2026-08-19, because amending `SPEC-MOK-003` shifted every line number below its rule 5.

### Artifacts that are wrong and should be corrected

1. **`ADR-MOK-003` line 205 is wrong.** It says "`Cargo.lock` acquires 57 entries". `Cargo.lock` has
   **182** `[[package]]` entries. 57 is the crate count from `cargo tree --edges normal`, which is a
   different measurement: the lock file records every platform's and every optional feature's
   packages, while `cargo tree` resolves for this host. The consequence the sentence draws — that the
   lock file must now be reviewed on change rather than confirmed empty — holds, and holds more
   strongly at 182. The number does not.

2. **The 57-crate figure is one of three defensible readings and is stated as if it were the only
   one.** It is stated twelve times: `SPEC-MOK-003` lines 59, 584, 645; `ARCH-MOK-002` lines 28, 159;
   `ADR-MOK-003` lines 80, 96, 144, 176, 177, 186, 202. (`ADR-MOK-003` line 205 also says 57, but of
   `Cargo.lock` entries, which is item 1 and a different error.) Measured: 57 with `--edges normal`,
   59 counting build dependencies, 37
   excluding proc-macro crates, 61 counting both workspace members. `dependency-review.txt` records
   all of them and names the two crates — `rustc_version` and `semver` — that the build-edge reading
   adds and neither binary links. The decision `ADR-MOK-003` takes is unaffected — no reading makes
   the surface small — but a reviewer comparing a future `cargo tree` against "57" needs to know
   which invocation produced it.

3. **`SPEC-MOK-003`'s rule 4 mockup does not fit the width rule 5 assigns.** The two-line roster
   entry at lines 226–231 shows three twenty-cell bars with labels and values, which needs about 87
   columns, plus rule 4.5's reserved fourth slot, which needs about 26 more. Rule 5 gives the roster
   47 columns, a 45-column interior. The two rules cannot both be satisfied literally. Resolved in
   the implementation by treating the mockup as the full form and scaling: `bar_width(interior) =
   min(20, (interior − 27) / 3)`, which is 6 at the reference roster and 20 where there is room, with
   `3 × bar_width + 27` never exceeding the interior. `render.rs:496-499`, with the constants at
   `render.rs:53,56`.

4. **Rule 2's layer table and its glyph table disagree about `M10`–`M12`.** The glyph table assigns
   `A`, `B`, `C`; the layer table's general rule is the identifier's last character uppercased, which
   would give `0`, `1`, `2`. Resolved by deriving the glyph from the numeric suffix as an uppercase
   base-13 digit, which reproduces the glyph table exactly for all twelve and falls back to the layer
   table's rule for an identifier with no digits. `spatial.rs:153-169`, `agent_glyph`.

### The specification's shape differed from the implementation's, and the specification was amended

All three items below were found by measurement, not by reading. `WO-MOK-005` states that such a
mismatch "requires an amended specification and re-approval, never a quietly adjusted constraint or
a relaxed assertion", so each is corrected in `SPEC-MOK-003`'s *Data and interface contracts* and
recorded in its amendment record marked **OUTSTANDING** — the technical owner has not approved them.

5. **`advance_tick` returns `Result<TickOutcome, String>`, not `TickOutcome`.** The listing showed
   the bare type. The engine's tick can fail — a refused advance on a finished run is the specified
   case, and rule 1.4 requires the refusal — so the error channel is what makes the refusal
   expressible. `new` likewise returns `Result<Self, String>` where the listing showed `Simulation`.
   Neither is a behavior difference; both are surface the listing did not show. Corrected to the
   real signatures at `SPEC-MOK-003` lines 483–489.

6. **The public surface exposes two `&mut self` methods that change simulation state, not one.**
   Rule 2 said `advance_tick` was the only one. `run`, the `REQ-MOK-010` whole-run entry point, is
   the second, and it was already publicly reachable at `origin/master`: `903c9943` has
   `pub mod simulation` in `src/lib.rs` and `pub fn run<W: Write>(&mut self, …)` at
   `src/simulation.rs:821`, both created by `WO-MOK-003` and bound by a `verified` `VREC-MOK-003`.
   This work did not expose it, and the rule was already inaccurate about the surface before this
   work began. Narrowing it would require relocating the engine's sources or duplicating the run
   loop, neither of which is in this work order's envelope. The observer never calls it. Rule 2 now
   states both methods, why `run` is there, and the checks that can be met. Analysed in full in
   `boundary-and-security-review.md`.

7. **`termination_reason` and `initialization_events` were on the surface and absent from the
   listing.** Both `&self`, so the mutation count is untouched. The observer needs the second for
   the pre-tick-1 records the event log presents. Both are now in the listing, with a note that the
   listing is what the observer calls and not the whole public interface.

### States the contract describes that a run cannot reach

8. **`VER-MOK-005` acceptance scenario 2 describes an unreachable state.** Neither shipped decision
   source can have a proposal rejected — `verification::no_shipped_decision_source_has_a_proposal_rejected`
   establishes it over 400 ticks of both policies — so no seed, speed or operator action will display
   a rejection. The rejection presentation is verified through the `#[cfg(test)]` hook
   `replace_decisions_for_test`, which injects a self-contradictory decision (a legal move carrying
   `rejected`) so that a test cannot pass by the observer re-deriving the verdict. The hook is
   compiled out of the shipped binary. Manual assessment 4 asks a person to confirm the rejection
   reads as authority rather than error, and cannot be performed by running the observer.

9. **A world with no standing resources is likewise unreachable.** `REQ-MOK-019`'s degenerate-world
   case names it. Rule 15 makes regeneration conditional on at least one standing resource, and
   neither decision source consumes fast enough to empty a territory before the population dies. Added
   `replace_snapshot_for_test` for this, also `#[cfg(test)]`. The no-living-Mokiterions half of the
   same case is reached through a real run.

10. **`REQ-MOK-024`'s hidden-roster-entry branch is unreachable at every declared viewport.** Twelve
    two-line entries need 24 rows; the roster interior is 32 rows at `160 × 48` and `160 × 44`, 36 at
    `120 × 48` and 24 at the tightest declared viewport that places it, and the overlay fits twelve
    one-line entries in 16. Nothing is ever hidden in the declared set, so the branch stating the
    hidden count is verified by reading the code and by nothing else. It exists for a population
    `SPEC-MOK-001` does not produce.

    **Changed 2026-08-19, and only partly.** The pane names in the original wording were tier names,
    which rule 5 no longer defines; the figures above are restated per viewport. The substance also
    changed: rule 5 as amended places the roster from 100 columns regardless of height, so the branch
    is now reachable at legal viewports outside the declared set — `100 × 22` hides 4 of 12,
    `100 × 26` hides 2. It is still unreachable at all nine declared viewports, so what item 17
    below adds is that the branch is now reachable and still unasserted, which is a narrower and more
    urgent statement than this one.

11. **Rule 9.2's "or one territory" is not expressible with the engine's records.** The rule says
    the subject filter restricts the log "to one Mokiterion or one territory". The filter matches on
    `event.subject`, and no engine event has a territory as its subject: the distinct subjects in a
    run are `M##`, `F####` and `world` — territory is a field inside `result`, not a subject. So the
    territory half of the rule cannot be satisfied by filtering on subject at all, whatever control
    were bound to it. It is implemented as far as the records allow: `Filter::Subject` compares the
    subject string, and the one control that sets it (`u`, `state.rs:479-484`) takes the current
    Mokiterion selection, so in practice it is always an `M##`. Closing this needs either a second
    filter dimension reading the `result` field or a change to the engine's subjects — both
    specification decisions, not observer fixes.

12. **Rule 9.1's log navigation needed a binding decision the spec does not make.** `j`, `k`, `Down`,
    `Up`, `PageUp` and `PageDown` are bound context-sensitively: they scroll the log while the log
    overlay has focus and pan the camera otherwise (`state.rs:450-477`). Rule 7 lists the keys and
    rule 9.1 requires older events to be reachable by scrolling, without saying which surface the
    keys act on when both the canvas and the log are present. Panning was chosen as the default
    because the canvas is on screen at every viewport above the floor and the log overlay is not.
    That reason held under the tier table and holds unchanged under rule 5 as amended, which places
    the view at every viewport and the log only from 38 rows.

### Measurements a reader should not misread

13. **Per-tick entropy draw counts are compared as record identity, not as a counter reading.**
    `VER-MOK-005`'s retention list asks for "per-tick entropy draw-count comparisons". The observation
    surface exposes no draw counter, and adding one would be public surface no artifact specifies. The
    comparison performed is per-tick identity of every entropy-bearing record across a whole run on
    each of the five declared seeds. Because the engine's entropy is one sequential stream, a single
    extra draw shifts every later value, so whole-run per-tick identity cannot hold if the counts
    differed. That is the observable form of the claim, not the claim. `non-perturbation.txt`.

14. **Four `VER-MOK-005` automated-test rows had no test behind them at the first evidence pass.**
    Found by auditing the matrix row by row against the actual test list: dead Mokiterions are not
    rendered, degenerate worlds render, the overview encodes no resource class, and legibility
    without colour. All four are now closed by four new tests in `verification.rs` plus the one new
    test hook in item 9. Disclosed because the first pass's evidence would have read as complete
    while four declared cases were unverified, and because an audit that finds four is not evidence
    that no fifth exists.

### An approved specification and the repository's own convention point opposite ways

15. **All 109 observer tests are in the internal tier, and the two governing documents disagree about
    whether that is right.** `SPEC-MOK-003`'s *Explicitly unspecified decisions* grants the
    implementation "test organization, fixtures and helpers", and the observer package builds one
    target — a binary — so `tests/` cannot reach it at all: a Rust integration test links a library
    target, and `mokiterions-tui` has none. Every test is therefore a `#[cfg(test)] mod tests` beside
    its subject, including `verification.rs`, whose 1,255 lines are the whole `VER-MOK-005`
    cross-cutting suite and which is `#[cfg(test)] mod verification` inside the binary
    (`main.rs:21-22`) for no reason other than that there is nowhere else to put it.

    `REPOSITORY_CONTEXT.md:29` states the opposite as a repository constraint: "A test that exercises
    the public contract … belongs in `tests/` and reaches the code through the library target's
    public API." Its cited authority is `SPEC-MOK-002` rules 7 to 10, and that specification's
    *Scope*, as amended 2026-08-18, says "Every rule below is a rule about the engine package". So the
    convention is stated repository-wide, its authority is engine-scoped, and the specification
    governing the observer explicitly hands the decision to the implementation. The implementation
    took the specification's grant. That is defensible and it is not what the context file describes,
    and a reader comparing the two packages will find one with two tiers and one with none.

    Closing it is not an evidence fix. Giving the observer a library target changes what
    `SPEC-MOK-003`'s component layout fixes, and its *Explicitly unspecified decisions* list says the
    implementation may **not** choose "the package layout". `INT-MOK-003`'s principle — "widening an
    item to `pub` in order to move a test is prohibited" — would be satisfied without any widening,
    because the observer's cross-module items are already `pub` and are merely unreachable outside the
    crate, which is the same condition `INT-MOK-003` found in the engine. But the target change itself
    needs an approved requirement and an amendment, so it belongs to a work order that has them, not
    to this one.

    **Resolved by `WO-MOK-006`, verified 2026-08-18.** That work order is the one with the requirement
    and the amendment this item said were needed: `REQ-MOK-028`, `CAP-MOK-005`, `INT-MOK-005`,
    `SPEC-MOK-004` and `VER-MOK-006`. The observer now builds a library target and a binary target,
    `src/verification.rs` is declared from `src/lib.rs` rather than from the binary, and 77 of the 109
    tests moved into eight files under `mokiterions-tui/tests/` with their assertions unchanged. The
    paragraphs above are the state at 2026-08-18 and the reasoning that led to that work order; the
    `main.rs:21-22` citation in the first one no longer resolves to what it described. Both packages
    now have two tiers, which is what the last sentence of the second paragraph asked for.

### Two smaller notes

- **Action tracing is always on in the observer.** The event log presents traced actions and the
  authority overlay maps them, so `trace_actions: true` is set unconditionally at `options.rs:108`
  and there is no flag to disable it. Tracing does not change a run — `additivity-proof.txt` and the
  engine's own unchanged tests cover that — but the observer's default differs from the engine
  binary's, where tracing is off unless `--trace-actions` is given.
- **The engine's three names differ in case, deliberately, and the observer's manifest keys on the
  package name.** The package and the binary target are `Mokiterions`; the library target is
  `mokiterions`, because the declared lint gate implies `non_snake_case` and `SPEC-MOK-002` rule 2
  fixes it in snake case. So `cli::USAGE` opens `Usage: Mokiterions`, matching the binary an operator
  invokes; the observer writes `use mokiterions::simulation::…`, matching the library; and
  `mokiterions-tui/Cargo.toml` writes `Mokiterions = { path = "../mokiterions-core" }`, because a path
  dependency is keyed by package name and not by directory. An earlier revision of this work renamed
  the engine package to `mokiterions-core` and its binary to `mokiterions`; that was reverted, and
  `WO-MOK-006` afterwards moved the package into a `mokiterions-core/` directory without renaming it.
  So the directory and the package name now differ deliberately: the directory is
  `mokiterions-core/`, the package and binary are still `Mokiterions`, and the library is still
  `mokiterions`. Renaming the binary target
  makes `USAGE`'s synopsis wrong until `src/cli.rs` is edited too, and `src/cli.rs` is one of the
  files this work keeps byte-identical to `origin/master` — it is the subject of `VER-MOK-001`'s and
  `VER-MOK-004`'s help-output cases, and `WO-MOK-005` scopes out changing an operator-facing string
  for a cosmetic reason. To be exact about the strength of that: no test asserts the synopsis line
  literally. `tests/process.rs:48,83` assert only `errors.contains("Usage:")`, and `tests/cli.rs:128`
  asserts the `Options:` block against the parser's real defaults. So the reason to keep the name is
  the scope rule and the unchanged-file property, not a literal assertion that would have failed.
  The observer has its own `options::USAGE` naming `mokiterions-tui`.

### A specified rule was wrong, the owner amended it, and one consequence is unapproved

16. **`SPEC-MOK-003` rule 5's tier table was not monotone, and a passing test pinned it.** The rule
    expressed a two-dimensional space as an ordered ladder of four rows, and the ladder had a hole:
    `W ≥ 140` with `38 ≤ H < 44` matched no row and fell to `otherwise`, which excluded the roster,
    the inspector and the log together. So a 160 × 40 terminal showed no left panel while 120 × 40 —
    same height, one third narrower — showed one, and making the window smaller added panes back. The
    owner reported it as a blocking defect on 2026-08-18. `mokiterions-tui/tests/layout.rs` asserted
    `tier_for(140, 43) == Tier::D` and passed, so the behavior was specified, implemented and pinned:
    a specification defect, not an implementation one.

    Measured rather than argued: over the 13,026 adjacent viewport pairs in `34 ≤ W ≤ 200` by
    `22 ≤ H ≤ 60`, the superseded table has **12** pairs where enlarging the terminal removes a pane
    and rule 5 as amended has **0**. Four of the nine declared viewports resolve differently under the
    two rules. `layout-and-viewports.txt` carries the sweep and the negative control, which is
    transcribed from the superseded code at 05dc6ac rather than paraphrased.

    On 2026-08-19 the owner chose one threshold per pane over the alternative of removing adaptive
    layout altogether, approved the `SPEC-MOK-003` rule 5 and `VER-MOK-005` amendments, and directed
    the implementation. Removal was not taken because `REQ-MOK-024` is approved and forbids clipping
    that could read as a different value, and truncating a fixed frame clips the footer, which is the
    provenance line a screen capture needs to be evidence. That would have needed a product-owner
    amendment to an approved requirement; the threshold change needs none, because `REQ-MOK-024` fixes
    no threshold and delegates every one to `SPEC-MOK-003`.

    **`SPEC-MOK-004`'s consequence is OUTSTANDING and requires the technical owner.** Removing `Tier`,
    its `label` method, `tier_for` and the `Panes::tier` field changes an interface that rule 6 closes
    by provenance and whose growth clause had no counterpart for removal: the recorded extent falls
    from 97 items to 94, and the rule gains a **Reduction** clause. Rules 9, 11 and 13 follow it for
    the test counts and one stale term, and rule 12's second paragraph is scoped to the `WO-MOK-006`
    restructuring so that renaming a test whose name cites a deleted rule is not itself a violation.
    The owner approved rule 5 without being shown any of this; the implementation agent found it
    afterwards by measuring the interface against the rule, and reports it rather than treating the
    rule 5 approval as covering it.

    **Terminology drift, reported and not amended.** `REQ-MOK-028`, `CAP-MOK-005`, `INT-MOK-005`,
    `WO-MOK-006` and `VER-MOK-006` each use "layout tier" for what a viewport yields, in clauses whose
    subject is that the restructuring changed nothing an operator can see. Those clauses were true at
    `VREC-MOK-006`'s commit and are not re-opened. What they now lack is a definition of the term,
    which rule 5 no longer supplies.

    **`ARCH-MOK-002` is now flagged stale by the dashboard, and reassessing it is the owner's call.**
    The generator's two new warnings say `ARCH-MOK-002` predates both specifications it declares
    `conforms_to`, because the amendments moved them to 2026-08-19 and it stays at 2026-08-18. On the
    substance the amendment does not touch what that artifact decides — the package split, the
    read-only engine surface and the dependency boundary are all untouched, and no rule 5 threshold
    appears in it — so the expected outcome is a re-affirmation rather than a change. It is left
    untouched all the same: it is `approved`, amending an approved architecture artifact is not in
    this work order's decision envelope, and a warning the owner can see is better than a date the
    implementation agent moved.

17. **The hidden-roster-entry branch became reachable, and nothing asserts it.** Rule 5 as amended
    places the roster from 100 columns whatever the height, so a terminal can now be wide enough for
    the roster and too short for twelve two-line entries. Measured: `100 × 22` presents 8 and reports
    `hidden 4`; `100 × 26` presents 10 and reports `hidden 2`. The branch behaves correctly —
    `render.rs:427` computes the count and `render.rs:449-455` puts it in the pane title in both the
    long and the short form — and no test reads either title, so it is unasserted rather than covered.

    This is a consequence of the change and is disclosed as one. It was not closed by adding an
    eleventh test to `tests/layout.rs`, because `SPEC-MOK-004` rule 9 fixes that file's count at 10 and
    changing it needs another amendment to a rule whose 2026-08-19 amendment is already outstanding.
    The two sizes above are legal and outside `VER-MOK-005`'s declared set, so no contract currently
    asks for them either. `requirement-to-test-mapping.md` caveat 1 and the roster-capacity section of
    `layout-and-viewports.txt` carry the measurement.

## Behavioural observations, not defects

- **Seed 42 under `--policy baseline` extinguishes all twelve on tick 142** (measured: `summary
  reason=extinction ticks=142 survivors=0 deaths=12`). This is why rule 10.6's long-run case is
  tested with `--policy reference`, and why manual assessment 1's 200-tick session must use the
  reference policy.
- **Under `--policy reference`, 11 of the 12 die by tick 10,000.** The living count and the death
  total sum to 12 throughout, so the corroboration contract `REQ-MOK-020` states holds; the
  observation is about world viability at the default density, which is `INT-MOK-002`'s subject and
  not this work's. `resilience.txt`.

## What is not complete

All seven manual assessments are outstanding, with no author. Three cannot be performed from this
environment at all, because `crossterm` on Windows reads the console input buffer and no keypress
can be delivered to the observer from this shell; the reasons and the measurements behind that
statement are in `manual-assessment.md`. The seventh has an automated result that `VER-MOK-005`
explicitly declines to accept alone ("rather than only by an automated assertion").

Every automated case in this packet is a claim about an in-memory character buffer. Nothing here
establishes that a person can read the result on a terminal.

**`VREC-MOK-005` in the tree is stale, and deliberately not edited.** It is `ready`, it binds
candidate commit `9d9641fe` — still an ancestor of this branch, so the commit exists, but its tree
predates both `WO-MOK-006` and this work order — and its figures are the superseded ones: 169 tests
where the workspace now runs 172, fifteen numbered disclosures where this file now carries
seventeen, 58 artifacts where validation now counts 68, six declared canvas interiors where
`VER-MOK-005` as amended declares nine renderable, `render.rs:495-498` where the citation is now
`496-499`, and its own item 11 — every observer test sitting in the internal tier — which
`WO-MOK-006` resolved. It is not
hand-corrected because the harness generates it: `harnessctl capture-verification` derives `commit`,
`worktree_state` and `artifact_snapshot_sha256` from the tree, and `require_clean_worktree` in
`se_harness/provenance.py` refuses with *"revision provenance requires a clean Git worktree"* while
anything is uncommitted. So the re-capture cannot precede a commit, and the commit is the owner's
act. The order is: owner authorizes the commit, then

    harnessctl capture-verification --id VREC-MOK-005 --work-order WO-MOK-005 \
      --verification VER-MOK-005 --evidence <each retained path> --owner "assurance owner"

against that commit, and only then does an accountable assurance owner move it to `verified`.
Nothing in this packet moves a lifecycle status.
