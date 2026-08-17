# Completion summary — WO-MOK-003

## Final affected components

**`mokiterions-core`** — the engine, at its original location. `src/lib.rs` is new (13 lines) and
declares the two public modules. `src/simulation.rs` gained the observation surface and the snapshot
types, and its record-emitting code was refactored to build `Event` values that both hosts consume;
`src/main.rs` changed 7 lines to route through the library. `Cargo.toml` became both the workspace
root and this package's manifest, with an empty `[dependencies]` table. No simulation rule and no
test changed: `additivity-proof.txt` shows all three test modules byte-identical to `48d16bd4`.

**`mokiterions-tui`** — new package, 5,500 lines across nine modules: `main.rs` (the loop and
start-up, 431), `state.rs` (observer state and key handling, 1,138), `render.rs` (every pane, 1,445),
`layout.rs` (tiers and regions, 348), `spatial.rs` (the world-to-canvas mapping, 307), `options.rs`
(argument parsing, 256), `export.rs` (the event buffer and file write, 188), `authority.rs` (the
event-type-to-requirement mapping, 132), `verification.rs` (`VER-MOK-003`'s cross-cutting cases,
1,255).

**`Cargo.lock`** — from one entry to 182.

**Gate results on the final tree:** `cargo fmt --all -- --check` exit 0; `cargo clippy --workspace
--all-targets --all-features -- -D warnings` exit 0; `cargo test --workspace` 161 passed, 0 failed
(48 engine library + 4 engine binary + 109 observer). `cargo test -p mokiterions-core` alone: 52
passed. Retained in `static-checks.txt` and `test-run.txt`.

## Disclosures

`WO-MOK-003` requires the completion report to disclose anything the work found that the artifacts do
not say. Fourteen items, grouped by what a reader should do about them.

### Artifacts that are wrong and should be corrected

1. **`ADR-MOK-002` line 198 is wrong.** It says "`Cargo.lock` acquires 57 entries". `Cargo.lock` has
   **182** `[[package]]` entries. 57 is the crate count from `cargo tree --edges normal`, which is a
   different measurement: the lock file records every platform's and every optional feature's
   packages, while `cargo tree` resolves for this host. The consequence the sentence draws — that the
   lock file must now be reviewed on change rather than confirmed empty — holds, and holds more
   strongly at 182. The number does not.

2. **The 57-crate figure is one of three defensible readings and is stated as if it were the only
   one.** It appears in `SPEC-MOK-002` (lines 55, 507, 539), `ARCH-MOK-002` (lines 28, 159) and
   `ADR-MOK-002` (six times). Measured: 57 with `--edges normal`, 59 counting build dependencies, 37
   excluding proc-macro crates. `dependency-review.txt` records all three. The decision
   `ADR-MOK-002` takes is unaffected — no reading makes the surface small — but a reviewer comparing
   a future `cargo tree` against "57" needs to know which invocation produced it.

3. **`SPEC-MOK-002`'s rule 4 mockup does not fit the width rule 5 assigns.** The two-line roster
   entry at line 226 shows three twenty-cell bars with labels and values, which needs about 87
   columns, plus rule 4.5's reserved fourth slot, which needs about 26 more. Rule 5 gives the roster
   47 columns, a 45-column interior. The two rules cannot both be satisfied literally. Resolved in
   the implementation by treating the mockup as the full form and scaling: `bar_width(interior) =
   min(20, (interior − 27) / 3)`, which is 6 at the reference roster and 20 where there is room, with
   `3 × bar_width + 27` never exceeding the interior. `render.rs:483-497`.

4. **Rule 2's layer table and its glyph table disagree about `M10`–`M12`.** The glyph table assigns
   `A`, `B`, `C`; the layer table's general rule is the identifier's last character uppercased, which
   would give `0`, `1`, `2`. Resolved by deriving the glyph from the numeric suffix as an uppercase
   base-13 digit, which reproduces the glyph table exactly for all twelve and falls back to the layer
   table's rule for an identifier with no digits. `spatial.rs:153-169`.

### The specification's shape differs from the implementation's

5. **`advance_tick` returns `Result<TickOutcome, String>`, not `TickOutcome`.** `SPEC-MOK-002`
   line 446 shows the bare type. The engine's tick can fail — a refused advance on a finished run is
   the specified case, and rule 1.4 requires the refusal — so the error channel is what makes the
   refusal expressible. `new` likewise returns `Result<Self, String>` where line 444 shows
   `Simulation`. Neither is a behavior difference; both are surface the listing does not show.

6. **The public surface exposes two `&mut self` methods that change simulation state, not one.**
   `SPEC-MOK-002` rule 2 says `advance_tick` is the only one. `run`, the `REQ-MOK-010` whole-run
   entry point, is the second. It predates this work; adding `pub mod simulation` to expose the
   observation surface made it publicly reachable for the first time. Narrowing it would require
   relocating the engine's sources, which this work order puts out of scope. The observer never calls
   it. Analysed in full in `boundary-and-security-review.md`.

7. **`termination_reason` and `initialization_events` are on the surface and absent from the
   listing.** Both `&self`. The observer needs the second for the pre-tick-1 records the event log
   presents.

### States the contract describes that a run cannot reach

8. **`VER-MOK-003` acceptance scenario 2 describes an unreachable state.** Neither shipped decision
   source can have a proposal rejected — `verification::no_shipped_decision_source_has_a_proposal_rejected`
   establishes it over 400 ticks of both policies — so no seed, speed or operator action will display
   a rejection. The rejection presentation is verified through the `#[cfg(test)]` hook
   `replace_decisions_for_test`, which injects a self-contradictory decision (a legal move carrying
   `rejected`) so that a test cannot pass by the observer re-deriving the verdict. The hook is
   compiled out of the shipped binary. Manual assessment 4 asks a person to confirm the rejection
   reads as authority rather than error, and cannot be performed by running the observer.

9. **A world with no standing resources is likewise unreachable.** `REQ-MOK-016`'s degenerate-world
   case names it. Rule 15 makes regeneration conditional on at least one standing resource, and
   neither decision source consumes fast enough to empty a territory before the population dies. Added
   `replace_snapshot_for_test` for this, also `#[cfg(test)]`. The no-living-Mokiterions half of the
   same case is reached through a real run.

10. **`REQ-MOK-021`'s hidden-roster-entry branch is unreachable at every declared viewport.** Twelve
    two-line entries need 24 rows; the roster interior is 32 rows at tiers A and B and 36 at tier C,
    and the tier-D overlay fits twelve one-line entries in 16. Nothing is ever hidden in the declared
    set, so the branch stating the hidden count is verified by reading the code and by nothing else.
    It exists for a population `SPEC-MOK-001` does not produce.

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
    because the canvas is on screen at every tier and the log overlay is not.

### Measurements a reader should not misread

13. **Per-tick entropy draw counts are compared as record identity, not as a counter reading.**
    `VER-MOK-003`'s retention list asks for "per-tick entropy draw-count comparisons". The observation
    surface exposes no draw counter, and adding one would be public surface no artifact specifies. The
    comparison performed is per-tick identity of every entropy-bearing record across a whole run on
    each of the five declared seeds. Because the engine's entropy is one sequential stream, a single
    extra draw shifts every later value, so whole-run per-tick identity cannot hold if the counts
    differed. That is the observable form of the claim, not the claim. `non-perturbation.txt`.

14. **Four `VER-MOK-003` automated-test rows had no test behind them at the first evidence pass.**
    Found by auditing the matrix row by row against the actual test list: dead Mokiterions are not
    rendered, degenerate worlds render, the overview encodes no resource class, and legibility
    without colour. All four are now closed by four new tests in `verification.rs` plus the one new
    test hook in item 9. Disclosed because the first pass's evidence would have read as complete
    while four declared cases were unverified, and because an audit that finds four is not evidence
    that no fifth exists.

### Two smaller notes

- **Action tracing is always on in the observer.** The event log presents traced actions and the
  authority overlay maps them, so `trace_actions: true` is set unconditionally at `options.rs:108`
  and there is no flag to disable it. Tracing does not change a run — `additivity-proof.txt` and the
  engine's own unchanged tests cover that — but the observer's default differs from the engine
  binary's, where tracing is off unless `--trace-actions` is given.
- **`cli::USAGE` still says `Usage: Mokiterions` with a capital M** while the binary is
  `mokiterions`. Pre-existing, untouched: `src/cli.rs` is under `VER-MOK-001`'s literal assertions
  and changing the string would change a verified test, which this work order makes a stop condition.
  The observer has its own `options::USAGE` with the correct lowercase name.

## Behavioural observations, not defects

- **Seed 42 under `--policy baseline` extinguishes all twelve on tick 142** (measured: `summary
  reason=extinction ticks=142 survivors=0 deaths=12`). This is why rule 10.6's long-run case is
  tested with `--policy reference`, and why manual assessment 1's 200-tick session must use the
  reference policy.
- **Under `--policy reference`, 11 of the 12 die by tick 10,000.** The living count and the death
  total sum to 12 throughout, so the corroboration contract `REQ-MOK-017` states holds; the
  observation is about world viability at the default density, which is `INT-MOK-002`'s subject and
  not this work's. `resilience.txt`.

## What is not complete

All seven manual assessments are outstanding, with no author. Three cannot be performed from this
environment at all, because `crossterm` on Windows reads the console input buffer and no keypress
can be delivered to the observer from this shell; the reasons and the measurements behind that
statement are in `manual-assessment.md`. The seventh has an automated result that `VER-MOK-003`
explicitly declines to accept alone ("rather than only by an automated assertion").

Every automated case in this packet is a claim about an in-memory character buffer. Nothing here
establishes that a person can read the result on a terminal.
