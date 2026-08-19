+++
id = "VER-MOK-010"
type = "verification"
title = "Naming verification: a computed name, reported once, presented everywhere, and a run that did not move"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
verifies = ["REQ-MOK-040", "REQ-MOK-041"]
+++

# Verification Contract: Naming verification

## Independence

This change is small, and that is the trap. Twelve string literals threaded to two presenters is the kind of work that
invites a suite asserting that `M01` is called `Zug` — twelve assertions that restate the table and would follow it
wherever it went. Those assertions are worth having, but they are not what decides whether this change is sound.

Two claims decide it, and both are claims of *absence*:

- the run did not move — no entropy was drawn, no outcome changed, no other reported value changed;
- no existing assertion had to be relaxed to accommodate a new field in a record two tests parse positionally.

A suite written by the effort that made the change is a poor witness to either. So the first is verified against a
recorded artifact of the world before the change, and the second against a census taken before the change and
reconciled name by name, neither of which the changed code can influence.

Five independent oracles are used.

1. **A recorded pre-change baseline, compared under a stated projection.** The engine's complete standard output and
   exit code are captured *before* any code change, at the commit the work begins from, across the declared matrix: the
   seeds `0`, `1`, `42`, `123` and `777`; all three decision sources; the default density `0.75%` and the swept
   densities `VER-MOK-002` declares; `--ticks 1000`; with and without `--trace-actions`. That is 90 cells. Afterwards
   the same matrix is captured again and the two are compared **byte for byte after removing the one field this change
   adds** — and nothing else. The projection deletes `name:<letters>,` from `agent_initialized` lines. Any other
   difference, anywhere in the 90 streams, is a defect in this change.

   Comparing projected streams rather than survivor counts is deliberate, for the reason `VER-MOK-007` gives in the same
   position: a count matching by coincidence is possible, tens of thousands of event lines matching line for line while
   the entropy sequence moved is not.

   The projection is the one place this oracle could be subverted, so its full text is retained, it is reviewed as
   evidence in its own right, and it is verified to be a **no-op on the pre-change stream**. A projection that also
   deleted a position, an identifier, an event kind or an attribute value would fail that check.

   The baseline is captured once, from a tracked-clean worktree, with the commit recorded. A discrepancy is never
   resolved by recapturing it.

2. **The shared entropy stream's own position.** An internal-tier test records the shared stream's state immediately
   before and immediately after the whole naming step for every Mokiterion and asserts they are equal, and asserts the
   post-initialization stream state equals a value recorded before this change. This is the direct form of
   `REQ-MOK-040`'s load-bearing constraint. It catches a design oracle 1 cannot always expose — a name chosen by a draw
   whose position was afterwards restored would pass oracle 1 on every seed and still be wrong.

   `VER-MOK-007` oracle 2 already pins the stream's position at the end of initialization for the declared seeds and
   densities, as a recorded expectation checked into the suite. This contract reuses those recorded numbers unchanged.
   A name that consumed entropy would move them, and the failure would name itself.

3. **A test census taken before the change and reconciled name by name.** `REQ-MOK-040` and `INT-MOK-008` both state a
   target of zero existing tests requiring an assertion change. That target cannot be verified by looking at the diff,
   because a relaxed assertion looks like a changed line and a deleted case looks like nothing at all. The census of
   every test name in the workspace is taken at the pre-change commit, taken again at the candidate, and reconciled:
   every pre-change name must still be present and passing, every difference must be an addition, and the two tests
   named below must pass **with their assertions byte-identical**.

   Two tests parse the `agent_initialized` record positionally and are therefore the specific tests this change could
   break: `mokiterions-core/tests/process.rs` asserts `,fear:0,waste_tolerance:` as an adjacent pair, and
   `mokiterions-core/tests/viability.rs` reads the trait by splitting on the record's last field. Their source is
   compared byte for byte against the pre-change commit, and the comparison is retained.

4. **An in-memory character buffer, cell by cell.** Every claim about what a frame presents is asserted against the
   rendered buffer at stated cell positions, never against a screenshot, a recording, a terminal or a pseudo-terminal,
   as `REQ-MOK-029` and `SPEC-MOK-004` require of every observer rendering claim. This includes the roster's entry
   lines, the map's glyphs and the inspector's title, for a living Mokiterion and for a dead one.

5. **The governance state of the artifacts this change amends.** The `SPEC-MOK-001` and `SPEC-MOK-003` amendments
   `WO-MOK-010` names must be present and approved before this contract can be satisfied. Their absence fails this
   contract regardless of the state of the code, for the reason `VER-MOK-006` gives: an amendment nobody approved is not
   a specification. This oracle also checks the narrower claim this change makes about `SPEC-MOK-002` — that **no**
   amendment to it is required, because the engine's public interface does not grow — by comparing the interface
   enumeration item for item rather than by asserting it.

The declared verification seed set is `0`, `1`, `42`, `123` and `777`, fixed by `VER-MOK-002` and reused unchanged.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-040` | automated-test | The twelve names at initialization, on each declared seed | Twelve `agent_initialized` records, each reporting exactly one name, and the twelve names are the ones `SPEC-MOK-001` fixes in the assignment it fixes, compared against a table written from the specification rather than read from the engine |
| `REQ-MOK-040` | automated-test | Stability across runs | `M01` reports the same name at all five declared seeds, at all three densities, under all three sources, at two tick limits, and in two processes; and so does every other identifier |
| `REQ-MOK-040` | automated-test | Distinctness | The twelve names are pairwise distinct, and their twelve first characters are pairwise distinct — asserted as a property of the table, so a later edit that collided two initials fails here rather than in the observer |
| `REQ-MOK-040` | automated-test | Character and length domain | Every character is in `A`–`Z` or `a`–`z`; every name is 1 to 5 characters long. Asserted because the name is written unescaped into a comma-and-colon delimited record and into a fixed-width column |
| `REQ-MOK-040` | automated-test | No name is readable as an identifier | No name equals any `M01`–`M12` identifier or any resource identifier form the output emits |
| `REQ-MOK-040` | automated-test | Table length equals population | The name table holds exactly as many entries as the run creates Mokiterions, asserted against the population constant rather than against the literal `12` |
| `REQ-MOK-040` | automated-test | Reported exactly once, and nowhere else | Across a full 1,000-tick run under every source, `name:` occurs exactly twelve times, all of them on `agent_initialized` lines, and on no `survival_changed`, `action_trace`, decision, consumption, regeneration, death or summary line |
| `REQ-MOK-040` | automated-test | Field position | The name is the first detail on the `agent_initialized` line and `waste_tolerance` remains the last, which is the property the two positional parsers of oracle 3 depend on |
| `REQ-MOK-040` | automated-test | Immutability across a run | Each Mokiterion's name is unchanged at tick 1,000 — asserted by the absence of any second report and by an internal-tier read of the stored value at both ends of a run |
| `REQ-MOK-040` | automated-test | Entropy neutrality (oracle 2) | The shared stream's state either side of naming is equal, for every Mokiterion, at every declared seed and density; and the post-initialization state equals `VER-MOK-007`'s recorded expectation, unchanged |
| `REQ-MOK-040` | automated-test | **The run is unchanged**, projected byte comparison against the recorded baseline (oracle 1) | All 90 cells byte-identical under the projection, exit codes identical, and the projection a no-op on all 90 pre-change streams |
| `REQ-MOK-040` | review | The projection used by oracle 1 | Deletes only the added field; anchored on the field name and its delimiters; verified to be a no-op on the pre-change stream; full text retained |
| `REQ-MOK-040` | static-analysis | No reader inside the engine | No rule, decision source, validation path, ordering, tie-break, termination condition or regeneration rule reads a name. The stored field has exactly one writer, at initialization, and exactly one reader, the record emission |
| `REQ-MOK-040` | static-analysis | Acting order untouched | Acting order remains ascending identifier order under `SPEC-MOK-001` rule 2; no comparison, sort or iteration anywhere in the engine keys on a name |
| `REQ-MOK-040` | static-analysis | Public interface (oracle 5) | The engine's public interface gains **no** item. `SPEC-MOK-002` rule 5's enumeration is compared item for item against the pre-change commit and is unchanged, and the observation snapshot gains no field |
| `REQ-MOK-040` | static-analysis | Purity of the derivation | No floating-point type or operation; no time, environment, address, hash-order or unordered-iteration input; no allocation whose result is observable in an ordering |
| `REQ-MOK-040` | static-analysis | Dependency table | The engine's dependency and dev-dependency tables remain empty, and `cargo tree -p Mokiterions` resolves to the engine package alone |
| `REQ-MOK-041` | automated-test | Roster entry, at every declared viewport (oracle 4) | Each entry's line one carries that Mokiterion's name and its identifier, at stated cell positions, with the name first; the identifier is present and unchanged in form |
| `REQ-MOK-041` | automated-test | The narrower entry form (oracle 4) | Where `SPEC-MOK-003` rule 4 specifies a one-line entry, the name is present in it and no field is truncated |
| `REQ-MOK-041` | automated-test | Bar arithmetic is untouched (oracle 4) | Bar width at every declared viewport equals the pre-change value, cell for cell; the four gauges keep their widths, labels and positions. A name that narrowed a bar is a defect against this row |
| `REQ-MOK-041` | automated-test | Map glyph, both zoom levels (oracle 4) | Each drawn Mokiterion's glyph is the uppercase first character of its own name, asserted at the cell the Mokiterion occupies; the twelve glyphs are pairwise distinct in a frame that draws all twelve |
| `REQ-MOK-041` | automated-test | Glyph derivation reads the reported name | The glyph function's input is the name the engine reported, not the identifier. Asserted by giving it a name whose initial differs from anything derivable from the identifier |
| `REQ-MOK-041` | automated-test | Inspector, living (oracle 4) | The pane identifies the selection by name and identifier at stated cell positions, and every value `SPEC-MOK-003` rule 10 lists is still present |
| `REQ-MOK-041` | automated-test | Inspector, through death (oracle 4) | After the selected Mokiterion dies, the pane still carries its name and identifier, and the death presentation of rule 10.6 is unchanged. This is the case only the retained event stream can satisfy |
| `REQ-MOK-041` | automated-test | Name-to-subject correspondence | In a frame drawing all twelve, each presented name sits beside the values of the Mokiterion that name belongs to. Asserted by constructing a population whose attribute values are distinguishable per subject |
| `REQ-MOK-041` | automated-test | Provenance: the observer holds no name | The observer presents only names it ingested from engine records. With no `agent_initialized` record ingested, no name is presented and no placeholder appears |
| `REQ-MOK-041` | static-analysis | No name literal in the observer | No name string literal, no identifier-to-name table and no derivation of a name from an identifier appears anywhere in `mokiterions-tui`, including its tests' fixtures where they stand in for engine records |
| `REQ-MOK-041` | static-analysis | Layout purity | Layout remains a pure function of viewport width and height; no pane threshold, pane width or pane presence changes |
| `REQ-MOK-041` | static-analysis | Observer dependency set | One path dependency on `Mokiterions` and `ratatui` at the pinned version and feature set, with nothing added |
| both | automated-test | **Prior coverage preserved, name by name** (oracle 3) | Every test name present at the pre-change commit is present and passing at the candidate; every difference is an addition; no case removed, renamed away or `#[ignore]`d |
| both | automated-test | **The two positional parsers are byte-identical** (oracle 3) | `mokiterions-core/tests/process.rs` and `mokiterions-core/tests/viability.rs` are unchanged from the pre-change commit, or every changed byte is an addition of a new test that leaves the existing assertions intact |
| both | automated-test | Test-tier placement | Every added test is in the public tier only if writable through the library target's public interface with its assertions unchanged; no item widened to `pub` to relocate a test, per `SPEC-MOK-004` |
| both | static-analysis | **Required amendments present and approved** (oracle 5) | The `SPEC-MOK-001` and `SPEC-MOK-003` amendments `WO-MOK-010` names are approved; and the claim that `SPEC-MOK-002` needs no amendment is checked by interface comparison, not asserted. Absence fails this contract regardless of code state |

## Acceptance scenarios

1. A reviewer captures the 90-cell matrix at the commit the work begins from, captures it again at the candidate,
   applies the stated projection to both, and finds all 90 pairs byte-identical with identical exit codes. The reviewer
   then applies the projection to the pre-change streams alone and confirms it changes nothing in any of them.
2. A reviewer deliberately perturbs the naming step to draw one value from the shared stream, confirms oracle 2 fails
   and oracle 1 fails, and reverts. A check that cannot be made to fail has not been demonstrated to work.
3. A reviewer greps a full 1,000-tick run for `name:` and finds exactly twelve occurrences, all on
   `agent_initialized` lines.
4. A reviewer reads the twelve names out of `SPEC-MOK-001` on paper, runs the engine at a seed of their choosing, and
   finds the same twelve in the same assignment — then runs it at four other seeds and finds them again.
5. A reviewer renders the map into an in-memory buffer with all twelve Mokiterions alive and reads twelve distinct
   uppercase initials at their twelve cells.
6. A reviewer renders the roster at each declared viewport, names the cell positions of the name, the identifier, the
   territory, the position and the applied action on line one, and confirms the bar row is unchanged cell for cell
   against the pre-change frame.
7. A reviewer selects a Mokiterion, lets it die, and finds the inspector still naming it on the following tick.
8. A reviewer constructs an observer that has ingested no initialization record, renders a frame, and finds no name and
   no placeholder where a name would be — not a dash, not a blank label, not the identifier standing in for one.
9. A reviewer searches the observer for a name literal and finds none, then searches the engine for a read of a name
   and finds only the record emission.
10. A reviewer diffs `tests/process.rs` and `tests/viability.rs` against the pre-change commit and finds no changed
    assertion.
11. A reviewer confirms both required amendments are approved, and confirms by item-for-item comparison that the
    engine's public interface did not grow.

## Property and invariant tests

- **Name determinism.** For every identifier, the name is a function of the identifier alone: stable across processes,
  seeds, densities, sources and tick limits. Asserted as a property, not sampled.
- **Name immutability.** No tick, action, rejection, death or regeneration changes a name.
- **Table well-formedness.** Pairwise-distinct names, pairwise-distinct first characters, ASCII-letter characters only,
  length within bounds, and table length equal to the population — asserted over the table itself so that any future
  edit to it is checked by this suite.
- **Entropy neutrality.** The shared stream's state after initialization is a function of seed and density alone, equal
  to the pre-change build's recorded value. Asserted as a property rather than inferred from output.
- **Report-once.** Over full runs, the count of name reports equals the population, and every occurrence is on an
  initialization line.
- **Record shape stability.** On every `agent_initialized` line, the name is the first detail and `waste_tolerance` is
  the last, and the details between them are in the order and form `SPEC-MOK-001` fixes.
- **Presentation correspondence.** For every rendered frame and every presented Mokiterion, the name shown is the name
  the engine reported for that subject, and the glyph is that name's first character uppercased.
- **Layout purity.** Bar width and pane geometry remain pure functions of viewport dimensions, numerically unchanged by
  naming.
- **Determinism of the whole engine.** `REQ-MOK-009`'s byte-identical reproducibility holds at every declared seed
  under every source, exactly as before.

## Static and architecture checks

- `cargo fmt --all -- --check` clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean, with no `allow` attribute added and no
  lint suppressed to accommodate this change.
- `cargo test` at the workspace root runs every tier of both packages in one invocation, with no feature, environment
  variable, ignore attribute, extra command, terminal or working-directory dependence.
- The engine's dependency and dev-dependency tables are empty, with no exception, and `cargo tree -p Mokiterions`
  resolves to the engine package alone.
- The observer's dependency set is one path dependency on `Mokiterions` and `ratatui` at the pinned version and feature
  set, with nothing added.
- No new package, no new target, no build script, and no change to any package, library or binary name.
- **The engine's public interface grows by exactly nothing.** `SPEC-MOK-002` rule 5's enumeration is compared item for
  item against the pre-change commit. The name travels in the event stream the observer already retains.
- No public item yields a mutable borrow of, or a reference into, authoritative state, in any build configuration
  including test builds. `SPEC-MOK-002` rule 6 is re-checked because a stored field was added to an internal type and
  because the reported field is a value on a public enum.
- No floating-point type or operation appears in the naming path.
- Test placement follows `SPEC-MOK-004`. Every internal-tier test added here names the private item it requires, and no
  item is widened to `pub` to relocate a test.
- No architecture amendment is required. No active architecture `addresses` `REQ-MOK-040` or `REQ-MOK-041` —
  `ARCH-MOK-001` addresses `REQ-MOK-004`, `008`, `009`, `010` and `016`, and `ARCH-MOK-002` addresses `REQ-MOK-021`,
  `025`, `026` and `028` — and the technical owner confirms in review that `ARCH-MOK-001`'s component boundaries,
  prohibited patterns and dependency prohibition are satisfied unchanged.

## Security and privacy checks

- No new input, no new file, no new network access, no new environment dependence and no new credential path. The name
  is not accepted from anywhere: it is not an option, not an environment variable and not a file.
- No model-provider credential or other secret enters the repository, in code, in a test fixture, or in retained
  evidence.
- Retained evidence contains simulation output only, and the names are properties of fictional entities.
- **A name is not a security primitive and carries no authority.** It is not an identifier, nothing keys on it, and no
  decision reads it. The observer gains no authority from receiving one: `ADR-MOK-001`'s trust boundary is unchanged.
- The names are invented rather than drawn from any living language's given-name stock, and no name is derived from any
  person, place or trademark. The judgement that each is inoffensive is manual assessment 1 below, because no automated
  check can make it.

## Performance and resilience checks

- Per-tick work is unchanged. Naming happens once, at initialization, in a fixed twelve steps, and nothing on the tick
  path reads a name.
- A 1,000-tick run at the default density completes within the bound `SPEC-MOK-001` states, on each declared seed.
- A 10,000-tick run completes without panic and without unbounded growth in retained state.
- The observer's frame remains within the bound `SPEC-MOK-003` states. The roster is at most twelve entries and a name
  adds bounded work per entry; the observer's name lookup is over at most twelve records.
- No arithmetic or slicing in this change can panic in a debug build. In particular the glyph derivation takes the first
  character of a name that the table guarantees is non-empty, and that guarantee is asserted rather than assumed.

## Manual assessments

Each of the following is an explicit judgement recorded by the accountable role. An unrecorded assessment is an
outstanding assessment, and this contract is not satisfied while any remains outstanding.

1. **The names themselves, by the product owner.** Each of the twelve is inoffensive in the languages the owner can
   assess, is not a real given name borrowed from a living person, and reads as the short, odd, faintly comic word the
   initiative asked for. No automated check can make this judgement and no implementation may make it.
2. **The assignment, by the product owner.** Which name belongs to which identifier is a product decision. The owner
   records that the twelve-to-twelve assignment in `SPEC-MOK-001` is the intended one, and that it is not to be
   reordered by an implementation for any reason.
3. **Legibility of the named roster entry, by the technical owner.** Line one now carries a name, an identifier, a
   territory, a position and an applied action within the roster's 45-column interior. The owner records the measured
   column usage at the reference viewport and confirms it is acceptable, or requires the rule 4 layout to change
   instead. Either way the decision is stated in the amendment, not settled in the implementation.
4. **Retaining the identifier beside the name, by the product owner.** Presenting both costs columns and presents the
   operator with two handles for one creature. The owner records that this is intended, on the ground `INT-MOK-008`
   states: the identifier is the join key into every retained stream and every citation in the corpus.
5. **The projection, by the assurance owner.** The transformation oracle 1 applies deletes only the field this change
   adds, and the owner confirms it could not mask a change to a position, an identifier, an event kind, an ordering, an
   attribute value or a line's presence.
6. **The glyph change, by the technical owner.** The map's glyph alphabet changes from `1`–`9`,`A`,`B`,`C` to twelve
   letters. The owner confirms that no glyph now collides with a resource glyph, a terrain glyph, a territory label or
   any other character `SPEC-MOK-003` rule 2 assigns meaning to, and that rule 2.5's colour-independent identity
   distinction is preserved.
7. **The absence of a name reader, by the technical owner.** Nothing in the engine reads a name, and the owner confirms
   that the value's only purpose in this scope is to be reported. A name that a rule read would be a different change
   with a different verification.

## Evidence retention

Retained under `docs/engineering/simulation/evidence/WO-MOK-010/`:

- the pre-change baseline capture of the 90-cell matrix, taken before any code change from a tracked-clean worktree,
  with the commit recorded; retained by per-cell SHA-256 digest, with a stated subset retained whole because it cannot
  be reproduced from the candidate commit;
- the post-change capture of the same matrix, retained the same way, and the projected comparison for all 90 cells;
- the full text of the projection, and the result of applying it to the pre-change streams alone;
- the captured exit code of every cell, both sides;
- the twelve names as reported by the engine, per declared seed, as the record that the assignment does not read the
  seed;
- the count and location of every `name:` occurrence in a full run under each source;
- rendered roster, map and inspector buffers as text, with the cell positions of the name, the identifier and the glyph
  stated, at each declared viewport, and the bar-row comparison against the pre-change frame;
- the rendered inspector buffer for a dead selection;
- the pre-change and post-change workspace test census, reconciled name by name, and the byte comparison of
  `tests/process.rs` and `tests/viability.rs`;
- the engine public-interface enumeration at both commits, compared item for item;
- the static searches: no name literal in the observer, no name reader in the engine, no name in any ordering;
- `cargo fmt`, `cargo clippy`, `cargo test` and `cargo tree -p Mokiterions` output;
- the seven manual assessments above, each with its accountable role and date;
- the amendment-approval check of oracle 5.

Evidence is retained in the repository, is reproducible from the recorded commands and commit, and contains no secret.

## Residual uncertainty

- **The stream is not byte-identical to the previous build, and cannot be.** One record kind gained a field. Every
  equivalence claim in this contract is a claim about the projected stream, and the distinction is stated wherever the
  claim is made.
- **Additivity is verified over the declared matrix, not over the input space.** A change that consumed entropy only at
  a density outside the sweep, or only past tick 1,000, would pass oracle 1. Oracle 2 is what reduces this risk, by
  asserting neutrality directly rather than inferring it from output. For this change the residual is smaller than it
  was under `VER-MOK-007`, because there is no derivation to consume anything.
- **Oracle 1 depends on its projection.** Two mitigations are stated — the no-op check on the pre-change streams and
  the assurance owner's review — and neither is a proof.
- **The zero-assertion-change target is verified by census and by byte comparison, not proved.** A test that still
  passes but now tests less — one whose assertion was widened rather than removed — would pass oracle 3. The byte
  comparison of the two positional parsers closes the specific case this change threatens; it does not close the
  general one.
- **Nothing can falsify the names.** No run can be wrong because a name is bad, so this contract verifies that names are
  well-formed, stable, reported once and presented faithfully; it cannot verify that they are good names. That is
  manual assessment 1 and it stays a judgement.
- **The glyph alphabet's distinctness is verified against the twelve names in the table**, not against every name a
  future edit might introduce. The table-level property test is what makes a future collision fail loudly, and it is the
  only guard.
- No claim is made about the observer's appearance to a human eye. Every rendering claim here is a claim about
  characters in a buffer at stated positions, and legibility is manual assessment 3.
- **This contract does not close `VREC-MOK-005`.** Its six outstanding amendments and seven outstanding assessments,
  and the amendment rows left outstanding on `SPEC-MOK-001` and `SPEC-MOK-003` by earlier work, are a precondition of
  this work under oracle 5, not an output of it.
