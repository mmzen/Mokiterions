+++
id = "SPEC-MOK-003"
type = "specification"
title = "Terminal observer presentation and read-only observation contract"
status = "approved"
owners = ["technical owner"]
created = "2026-08-17"
updated = "2026-08-20"

[relations]
specifies = [
  "REQ-MOK-019",
  "REQ-MOK-020",
  "REQ-MOK-021",
  "REQ-MOK-022",
  "REQ-MOK-023",
  "REQ-MOK-024",
  "REQ-MOK-025",
  "REQ-MOK-026",
  "REQ-MOK-027",
  "REQ-MOK-032",
  "REQ-MOK-041",
]
+++

# Specification: Terminal observer presentation and read-only observation contract

## Scope

This specification fixes the exact behavior of a terminal observer over the existing simulation engine, and the
read-only interface through which the observer obtains authoritative state.

It governs `REQ-MOK-019` through `REQ-MOK-027`. It does not govern any simulation rule. Every world rule —
dimensions, territories, attributes, food classes, tick order, actions, validation, perception, decay, death,
regeneration, density, entropy, the event vocabulary, and the text stream — remains fixed by `SPEC-MOK-001`, which
this specification never restates as authority and never contradicts. Where a figure from `SPEC-MOK-001` appears
here, it appears as a consumed input; if the two ever disagree, `SPEC-MOK-001` governs the world and this document
has a defect.

This specification adds no simulation behavior and no simulation state.

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-17 | Original content for `CAP-MOK-004`, covering `REQ-MOK-019` through `REQ-MOK-027`. | Approved by the technical owner. |
| 2026-08-17 | Corrected two derived figures in rule 5 before approval. The `100 × 30` row omitted the pane border and claimed a whole-world presentation its 98 × 24 interior cannot deliver, since 24 cells address 96 of 128 world rows; the tier C example stated a 71 × 42 canvas where the arithmetic yields 71 × 36. A `120 × 48` row was added so tier C carries a checkable obligation. No rule changed. | Approved by the technical owner. |
| 2026-08-18 | Corrected the statement of required `SPEC-MOK-002` amendments in *Compatibility and migration*, which the merge with `master` exposed as understated. Rule 3 is added as a fourth required amendment: it freezes `src/simulation.rs`'s contents against anything but a visibility change, and the observation surface is new code. Rule 6's required narrowing is extended to the "by … return value" path, since every accessor returns an owned copy. The list of names that stay private is corrected from nine to **ten** — `DecisionEntropy` was omitted. No rule of this specification changed, and no obligation on the observer changed. | Corrected by the implementation agent as a statement of fact about another artifact; the four amendments it states were the technical owner's act and were outstanding when this row was written. The owner ratified all four in `SPEC-MOK-002` on 2026-08-20 under `WO-MOK-012`. |
| 2026-08-18 | *Data and interface contracts* corrected on a claim about the engine's surface that does not hold as written, and which `WO-MOK-005` requires be fixed by amending the specification rather than by relaxing the assertion. Clause 2 said `advance_tick` was the only `&mut self` method on the surface that changes state; `Simulation::run`, the pre-existing `REQ-MOK-010` whole-run entry point, is a second. The clause now states the two, why `run` is there, why it cannot be narrowed away without relocating the engine's sources or duplicating the run loop, and the checks that can actually be met — including that the observer reaches neither `run` nor anything leading to it. The method listing is corrected to the real signatures and completed with `termination_reason` and `initialization_events`, with a note that it is what the observer calls and not the whole public interface. No obligation on the observer changed, and the non-perturbation property is unaffected. | **Ratified 2026-08-20 by the repository owner acting as technical owner**, as written and without modification, in the assessment review recorded under `WO-MOK-012`. It was **OUTSTANDING** from 2026-08-18 until that act, as a correction to an approved specification; `boundary-and-security-review.md` under `WO-MOK-005` is the measurement that found it. The owner was shown that this row corrects a false statement of fact rather than relaxing an obligation: `Simulation::run` was already public at `origin/master` before the observer existed, so the clause was wrong on the day it was approved and the amendment makes the specification true rather than making the implementation permissible. The non-perturbation property is unaffected and the observer reaches neither `run` nor anything leading to it. The implementation agent wrote this text and decided none of the substance. `VREC-MOK-005` is not edited here: it is `ready`, not `verified`, and `WO-MOK-012` records what it does not carry. |
| 2026-08-18 | Four provisions amended so that `REQ-MOK-028`, `REQ-MOK-029` and `REQ-MOK-030` can be conformed to. **Component layout**: the tree restated to one directory per package, matching `SPEC-MOK-004` rule 1, and clause 2 given the concrete path-dependency form. **Clause 3**: "The engine's sources are not relocated" replaced by the reason it existed for — the `REQ-MOK-010` text stream does not move — which a directory move preserves and `VER-MOK-006` measures. **Data and interface contracts clause 2**: its reasoning no longer appeals to the component layout forbidding relocation, since it no longer does; what would narrow `run` away is a target split, not a directory move, and the `grep` check is re-based on `mokiterions-core/src/simulation.rs`. **Explicitly unspecified decisions**: the grant of "test organization" withdrawn to `REQ-MOK-029` and `SPEC-MOK-004` rules 8 to 10, leaving fixtures and helpers with the implementation; and "the package layout", withheld but previously fixed nowhere, now pointed at `SPEC-MOK-004` rules 1 to 4, together with the observer's target shape and test-tier placement. No rule about the observer's behavior, presentation, key bindings, export, snapshot contract or non-perturbation changes, and no figure changes. | Approved 2026-08-18 by the repository owner as technical owner, by way of `ADR-MOK-004`, whose *Required amendments* section states this amendment in full. The implementation agent wrote the text under `WO-MOK-006`; it did not decide it. `VREC-MOK-005`, which binds this specification to `WO-MOK-005`'s commit, is not edited: what it verified was correct at its commit, and the two rows above it are untouched. Those two were **OUTSTANDING** when this row was written — the first because the four `SPEC-MOK-002` amendments it states were unratified, the second in its own right — and both were closed on 2026-08-20 under `WO-MOK-012`. |
| 2026-08-19 | **Rule 5's four-row tier table replaced by one threshold per pane on the axis that constrains it**: roster `W ≥ 100`, inspector `W ≥ 140`, log `H ≥ 38` at 6 rows and 10 when `W ≥ 140` and `H ≥ 48`. The tier table was an ordered ladder over a two-dimensional space and left a gap: `W ≥ 140` with `38 ≤ H < 44` matched no row and fell to `otherwise`, which excludes the roster, the inspector and the log at once. A 160 × 40 terminal therefore presented no roster while 120 × 40 — the same height, narrower — presented one, so enlarging a terminal could remove panes. The repository owner reported the missing roster as a blocking defect. A **monotonicity** obligation is added — no pane present at a viewport is absent at any larger one — which holds by construction and is checkable over the whole plane rather than at named sizes, and which is the obligation whose absence let the gap through. The derived table gains `160 × 40`, `140 × 43` and `120 × 30`, the three sizes the previous table handled worst; one existing figure changes, `100 × 30` from 98 × 24 to 51 × 24, because the roster is now present at that width — it remains a region and remains annotated, and the row carrying the width-versus-height asymmetry becomes `120 × 30`. Two trades are stated where canvas area is not monotone: the inspector at `W = 140`, already declared, and the log at `H = 38`, which costs whole-world rows between 38 and 43. The alternative of admitting the log only at `H ≥ 44` is stated and rejected, and is reversible by changing one threshold. Consequentially: rule 8's "present in every tier" becomes "at every viewport above the floor", two examples no longer name a tier, and the withheld-decisions sentence points at rule 5's pane thresholds instead of its tier table. The floor and its exit `2` refusal, the announcement obligation, the resize behavior, rule 2's fidelity minimum and mapping, the key bindings, the glyphs, the export, the authority mapping and the snapshot contract are untouched, and no requirement changes — `REQ-MOK-024` fixes no threshold itself and delegates every one of them to this specification. | Approved 2026-08-19 by the repository owner as technical owner, who directed the implementation in the same act. The owner had chosen this direction over removing adaptive layout altogether on 2026-08-18, after being shown both, and reviewed this text before approving it. The implementation agent measured the defect, drafted this text and recorded this approval on the owner's explicit instruction; it holds no authority over either, since `WO-MOK-005`'s decision envelope withholds rule 5's thresholds from the implementation. `VREC-MOK-005` is not edited here: it is `ready`, not `verified`, and it is re-captured against the commit that carries the implementation. |
| 2026-08-19 | Rule 4 amended in three provisions, under `REQ-MOK-032`, so the roster presents a computed `fear`. The two-line mockup shows four gauges. Rule 4's prose reads four attributes and four numeric values, in the two-line form and in the collapsed one-line form below 47 columns. **Item 5, the reservation, is replaced by the presentation of a computed value, and item 4 now governs the zero case**: `fear 0` renders as `0` with an empty bar, distinguishable from an absent value, and it is a computed zero rather than an inert one — which is the condition item 5 was waiting for. The reservation's own reasoning is retained here rather than deleted, because it is what made the empty slot correct for a phase: the row reserved trailing space for a fourth bar so that Phase 2's `fear` could occupy it, rendering empty with no label, no dash and no zero, because "an inert `fear 0` would be a claim the engine cannot support". The measured consequence is stated in the amended item: the width rule becomes `bar_width(interior) = min(20, (interior - 35) / 4)`, since a fourth group of label, space, bar, space and a three-column value raises the row overhead from 27 to 35 and the divisor from 3 to 4, so at the reference roster's 45-column interior the bars narrow from 6 cells to 2 while the three-column numeric values are unaffected. This closes `VREC-MOK-005` finding 3, which recorded that the reserved slot was zero-wide and therefore absent rather than empty. | Approved 2026-08-19 by the repository owner acting as technical owner, who accepted the narrowing on that date rather than widening the roster pane in rule 5, which would have taken fourteen columns from the map pane, or raising rule 4's two-line threshold, which would have cost bars entirely to operators between 47 and 60 columns. The implementation agent wrote the text and did not decide the substance. **The 2026-08-18 row marked OUTSTANDING above is untouched** and belonged to `WO-MOK-005`; the finding-3 resolution it recorded is superseded by this row, but its own amendment is not, and it was awaiting the owner's separate act when this row was written. That act came on 2026-08-20 under `WO-MOK-012`. **Three further provisions were found during implementation and are amended here, beyond the three `WO-MOK-010` named.** Each is forced by the change rather than chosen with it. *Data and interface contracts*: the `AgentSnapshot` field list gains `fear`, which `SPEC-MOK-002` rule 5 admits to the interface, because a field list that omitted it would contradict that rule. Rule 10 item 7 loses `fear` and traits from its list of values the engine does not compute, because the engine now computes both; the item states why each is still absent from the inspector, and rule 10's presented-value list is **not** amended, so the inspector is unchanged. Rule 11's `decision_source_selected` row gains `REQ-MOK-033` for `individual`, because that mapping is exhaustive by construction — the observer resolves it in a `match` over the policy — so a third source without a row is a gap the compiler reaches before an operator does. **Those three provisions were unratified until 2026-08-19, when the repository owner, acting as technical owner, ratified all three in the closing review of `WO-MOK-010` recorded in `evidence/WO-MOK-010/closing-review.md`.** The ratification covers the three provisions of this paragraph and nothing else in this row: the amendment at its head was approved with the work order, and the 2026-08-18 row above was **OUTSTANDING** when this row was written, until the technical owner ratified it on 2026-08-20 under `WO-MOK-012`. |
| 2026-08-19 | **Rule 4 gains clause 7, survival bands.** The roster's three bars take a colour read from the value they present — green `80..=100`, orange `40..=79`, red `0..=39` — applied to each gauge's label, bar cells and numeric value together. The bands are a presentation of a number the bar already carries, so no quantity the engine does not compute enters the roster and `REQ-MOK-020`'s constraint against derived survival estimates is untouched; nor is any threshold borrowed from `SPEC-MOK-001` rule 5's reference-source sleep threshold of `20`, which is a decision source's policy rather than a survival state. Zero is red and clause 4's rendering of `0` as `0` with an empty bar is unchanged, as is what distinguishes zero from an absent value. Rule 2.5 needs no amendment, because the numeric value and the proportional fill already carry level without colour, which is what makes a band redundant reinforcement rather than the sole carrier of a distinction. No character of the entry moves and clause 4's mockup is unchanged. Clause 6's reversed-video selection composes with the band. The collapsed one-line form takes no band, stated in the clause and reversible by one sentence. No other rule, figure, glyph, key binding, export, authority mapping or snapshot contract changes. | Approved 2026-08-19 by the repository owner as technical owner, who fixed the three bands in the same act and reviewed this text before approving it. The owner's request had been green when the attribute is fine, orange when it is decreasing, red when it is low; the trend half was withdrawn after being shown that it inverts. `SPEC-MOK-001` decays satiety and energy by one each tick for every living Mokiterion, so "decreasing" is true of nearly every bar on nearly every tick and would become false only at zero, and `EventDetail::SurvivalChanged` carries the decay leg alone so reading the engine's own trend reproduces that inversion. Zero was unstated in the owner's bands and was confirmed as red on the same occasion. The implementation agent measured the inversion, drafted this text and recorded this approval on the owner's explicit instruction; it holds authority over neither, since this specification withholds thresholds of this kind from the implementation. The concrete colour values are the implementation's under the grant of "the exact palette, provided every distinction remains available without colour", and are recorded in `WO-MOK-007`. |
| 2026-08-19 | **No rule changed. This row records the reconciliation of the rule 5 and rule 4 rows above, which were written independently and met in a merge.** `WO-MOK-005`'s rule 5 amendment and `WO-MOK-010`'s rule 4 amendment were approved on the same date by the same owner against different trees, and both are retained above verbatim: neither owner act is edited, summarised, or folded into the other. They occupy disjoint text and do not contradict each other in substance — rule 4 fixes the form of a roster entry, rule 5 fixes which viewports present the roster pane and how wide it is. **The one figure spanning them holds unchanged**: rule 5 gives the roster `47` columns wherever it is present, before and after its amendment, so rule 4's 45-column interior, its `bar_width(interior) = min(20, (interior - 35) / 4)` and the two-cell bars that follow are untouched, and rule 4's 47-column two-line threshold is met at every viewport presenting the pane at all. **What the merge changes is not a rule but a set**: rule 5's derived table now presents the roster at eight of its nine declared viewports rather than four, `100 × 30` among them, so any measurement of *which viewports present the roster* taken against the withdrawn tier table describes a table this document no longer contains. `WO-MOK-010`'s oracle 4 frame capture and its `the_roster_presents_four_gauges_at_every_declared_viewport_that_presents_it` test were both taken that way. The test is corrected in the merge commit against rule 5 as amended; the capture is **OUTSTANDING** re-derivation under `WO-MOK-010`. | Recorded by the implementation agent as a statement of fact about two approved amendments it holds no authority over. No provision of this specification is added, removed, or reworded by this row. The re-derivation it names belongs to `WO-MOK-010` and is the assurance owner's to accept once taken. **It was taken on 2026-08-19**: the frame capture was re-derived against rule 5 as amended and reads 996 bar rows over the 85 of 157 probed frames that draw a roster, with zero discrepancies, in `evidence/WO-MOK-010/observer/roster-frames.txt` with the method recorded in `evidence/WO-MOK-010/renumbering.md`. The **OUTSTANDING** re-derivation this row names is therefore discharged, and `VREC-MOK-010` binds it. This row still changes no provision and still ratifies nothing. |
| 2026-08-19 | **Rule 4 clause 7 amended in two provisions, so that the four gauges of clause 5 coexist with the bands of clause 7.** The two clauses were approved on the same date by the same owner against different trees, and each is retained above verbatim. They meet at exactly one point: clause 5 makes the bar row four gauges and clause 7 bands "each of the three bars", which leaves the fourth gauge unstated. **The banded set is now named rather than counted**: the band applies to health, satiety and energy, and `fear` renders as a bar and a numeric value with no colour at all. The reason is stated in the clause rather than left to be inferred — the three bands are a survival scale on which a high value is a good one, and `fear` inverts that, so a banded `fear 100` would read green while naming the worst state that attribute has. **The collapsed one-line form's count is corrected from three numeric values to four**, which clause 5 had already changed; it remains unstyled and takes no band, so that provision's substance is untouched. Nothing else in either clause changes: no boundary, no colour, no glyph, no character of the entry, no bar width, and no band for health, satiety or energy either gains or loses. Rule 2.5 still holds for the same reason it held before, and holds a fortiori for `fear`, which now carries no colour to be the sole carrier of anything. | **Decided 2026-08-19 by the repository owner acting as technical owner**, on the choice put to them once the collision was found: band `fear` on the same three-band scale, give it a second and opposite scale of its own, or leave it unbanded. The owner chose unbanded, on the reasoning that the scale is a survival scale and `fear`, whose direction inverts, does not borrow it; a second opposite scale was declined because it would put two contradictory colour meanings on one row. The implementation agent found the collision, put the choice, wrote this text and the amended clause, and decided none of the substance. **The wording is the agent's and was OUTSTANDING for the owner's ratification until 2026-08-19, when the repository owner, acting as technical owner, ratified it in the closing review of `WO-MOK-010` recorded in `evidence/WO-MOK-010/closing-review.md`; the decision it records never needed one.** `VREC-MOK-010` is a `ready` candidate bound to a commit that predates this row and is re-captured against the merge, not edited. |
| 2026-08-19 | Four provisions amended under `REQ-MOK-041`, so the observer presents the name `REQ-MOK-040` makes the engine report. **Rule 2's glyph tables**: the detail table's `M01`–`M09 → 1`–`9` and `M10`–`M12 → A, B, C` rows are replaced by the name's first character uppercased, with `?` for a subject whose name was not received, and the overview layer table's "the identifier's last character" becomes "the name's first character" — the two zooms derive one glyph and had drifted, since the withdrawn overview rule gave `M10` a `0` where the detail table gave it an `A`. The twelve resulting glyphs `Z K Q S T W H N V G X D` are stated, resting on `SPEC-MOK-001`'s twelve pairwise-distinct first characters, which is what rule 2.5 needs and what the identifier-derived assignment had by construction. **The anticipation is retained rather than deleted**: the withdrawn table and the sentence promising that "when agent naming is introduced by a later phase, the glyph becomes the name's first character and this table is amended" are quoted in place, because they are why the old assignment was correct while no name existed. **Rule 4**: the entry mockup and prose carry the name first, then the identifier, in the two-line form and in the collapsed one-line form, and the addition to the identifier rather than replacement of it is stated with its reason — the identifier is the join key into the log, the export and every retained stream. **Line two is measured to be untouched**: the name occupies six columns of line one only, so the bar row's five leading columns, its 35-column overhead and `bar_width(interior) = min(20, (interior - 35) / 4)` are unchanged and the reference roster's two-cell bars stand; line one's fixed fields total 28 columns of a 45-column interior, so nothing truncates. **Rule 10**: the presented-value list gains the name, before the identifier and for a dead subject as well as a living one, under rule 10.6's retained selection; item 7 loses `name`, because the engine now reports one, and the item's principle is restated as the reason the presented name must be the engine's own — a name derived from an identifier or filled in as a placeholder would be a value the engine did not compute. **Nothing else changes**: no pane threshold, no floor, no layout figure, no key binding, no export form, no authority mapping row, no snapshot field, no interface item, and no obligation that layout be a pure function of viewport size. | Approved 2026-08-19 by the repository owner acting as technical owner, together with `INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`, `REQ-MOK-041`, `VER-MOK-011` and `WO-MOK-011`. The twelve names are the product owner's decision of the same date; the decision that the name is presented in addition to the identifier and precedes it, and that the observer sources it from the retained event stream rather than from a new engine interface item, are the technical owner's, recorded in `WO-MOK-011`. The implementation agent wrote the text and did not decide the substance; rule 2's glyph assignments are withheld from it by `WO-MOK-005`'s decision envelope. **The 2026-08-18 row marked OUTSTANDING above is untouched**, as are the four rows above it that `WO-MOK-010` and its merges left: rule 4's clause 5, its clause 7, the row reconciling clause 5 with `WO-MOK-005`'s rule 5, and the row reconciling clause 7 with clause 5. **Clause 7 and this row do not meet**: the bands colour cells on line two and this row adds the name to line one, so the banded set, its three boundaries, the unbanded `fear`, the 35-column bar overhead and `bar_width` are neither read nor written here, and the frame re-derivation the reconciliation row records as discharged is not reopened by a name. This sentence is the one part of this row the merge rewrote, and it changes no provision. |

## Actors and external systems

- **Operator.** A person at an interactive terminal. The only actor. Observes and navigates; never mutates world
  state.
- **Terminal emulator.** An external system whose dimensions, colour support and Unicode support the observer does
  not control and must not assume. It is entered in an alternate screen with raw input and must be restored on every
  exit path.
- **Terminal user-interface library.** `ratatui` version `0.30.2` with `default-features = false` and features
  `crossterm`, `layout-cache`, `underline-color`. This resolves to a measured surface of **57 crates** including
  itself, and it is a dependency of the observer component alone. The `serde` feature is off, and no feature enabling
  networking, an asynchronous runtime, or serialization is enabled.
- **Filesystem.** Written to exactly once per operator-requested export, at an operator-supplied or default path.
  Never read from.

No network, credential, model provider, database, or asynchronous runtime is involved in either component.

## Inputs

### Start-up inputs

The observer accepts the same simulation inputs as the engine binary, with identical names, identical parsing,
identical validation, identical defaults and identical rejection behavior: `--seed`, `--ticks`, `--density`,
`--policy`, and `--help`. Their semantics are fixed by `SPEC-MOK-001` and are not restated here.

It additionally accepts:

| Input | Values | Default | Meaning |
|---|---|---|---|
| `--speed` | integer in `{1, 2, 4, 8, 16, 32, 64}` | `8` | ticks advanced per second while progression is running |
| `--start-paused` | flag | absent | begin held before tick 1 rather than running |
| `--export` | path | absent | default path used by the export control; the export still requires the operator's key press |

An invalid value for any input is rejected before the terminal is entered, with a diagnostic on standard error and
exit code `2`. `--export` is validated as a string only; it is never opened at start-up, never interpreted as code,
and never used to read.

### Runtime inputs

Key presses, and terminal resize notifications. Nothing else. There is no configuration file, no environment
variable, and no standard-input protocol.

## Outputs

- Rendered frames to the terminal's alternate screen. Frames are presentation, not a record, and no claim rests on
  them alone.
- One export file per operator request, written to the resolved path.
- Diagnostics to standard error, before the terminal is entered or after it is restored, never interleaved with
  frames.
- Exit codes: `0` on normal exit; `2` for invalid configuration or a viewport below the floor at start-up; `1` for an
  unrecoverable runtime or terminal failure. This matches `SPEC-MOK-001` so the two binaries cannot be confused by
  their status.

The observer does not write the `REQ-MOK-010` text stream to standard output. That stream remains the engine
binary's output and is unchanged.

## State model

The observer holds one `Simulation` value and its own presentation state. Presentation state is not simulation
state, is never persisted, and never influences any engine computation.

| Field | Domain | Initial |
|---|---|---|
| `progression` | `Running` \| `Held` | `Running`, or `Held` when `--start-paused` |
| `speed` | `{1,2,4,8,16,32,64}` ticks per second | `--speed`, default `8` |
| `selection` | Mokiterion identifier or none | none |
| `follow` | on \| off | off |
| `zoom` | `Overview` \| `Detail` | `Overview` |
| `camera` | top-left world cell of the visible region | `0:0` |
| `filter` | none \| event type \| subject | none |
| `overlay` | none \| roster \| log \| inspector \| help \| authority | none |
| `events` | ring buffer, capacity `100_000` records | empty |
| `truncated` | bool — capacity was reached and records were dropped | false |

`events` retains the authoritative events the observer has seen this run. It is a presentation buffer: dropping the
oldest record when full loses presentability, never authority, because the engine binary's text stream remains the
unbounded record. `truncated` is displayed and exported when true.

## Behavioral rules

### Rule 1 — Progression

The observer advances the simulation by calling the engine's single-tick advance and by no other means.

1. While `Held`, the engine is not advanced. The observer holds the simulation at a **completed-tick boundary**: a
   tick is either fully applied, including all twelve agent turns in `SPEC-MOK-001` order and any regeneration, or
   not started. There is no state in which the observer presents a partially applied tick.
2. While `Running`, the observer advances one tick every `1000 / speed` milliseconds, measured from the previous
   advance. If the observer falls behind, it advances at most one tick per scheduling opportunity and never advances
   two ticks to catch up in zero elapsed time; falling behind slows the run and never changes it.
3. The single-step control is accepted only while `Held`. It advances exactly one tick and remains `Held`.
4. Advancing is refused, with no state change, once the engine reports the run finished. The final state remains
   fully inspectable, selectable and exportable.
5. Wall-clock time is read only to decide *when* rule 1.2 advances and when rule 6 draws. It is never passed to the
   engine and never enters any authoritative value.

### Rule 2 — The spatial view

The spatial view is a bordered pane containing a canvas of `Cw × Ch` character cells, determined by rule 5.

**Overview zoom.** The canvas uses braille marker cells: each character cell carries a 2-wide by 4-tall grid of
independently addressable dots, so a canvas of `Cw × Ch` cells addresses `2·Cw × 4·Ch` world cells at one dot per
world cell. The 128 × 128 world is therefore presented in full, at one dot per world cell, exactly when
`Cw ≥ 64` and `Ch ≥ 32`.

**Detail zoom.** One character cell is one world cell. The visible region is `Cw × Ch` world cells.

In both zooms:

1. **Orientation.** World `y` increases downward on screen, so territory A (`y` 0–63) is presented above territory B
   (`y` 64–127), matching `SIMULATION_RULES.md`. The canvas coordinate system is bottom-up, so the observer maps
   world `y` to canvas `127 − y`. A view that presents territory A below territory B is a defect.
2. **Territory boundary.** A horizontal rule is drawn between world rows 63 and 64 whenever that boundary lies in
   the visible region.
3. **Region indication.** When the visible region is smaller than the world in either axis, the pane title states the
   visible world range, so absence from the view is never read as death.

**Overview rendering layers**, drawn in this order:

| Layer | Encoding |
|---|---|
| Resources | one braille dot per standing resource |
| Territory boundary | a rule as in 2.2 |
| Mokiterions | the name's first character as an uppercase glyph filling the whole character cell containing it |

An overview Mokiterion glyph therefore locates its subject to within the 2 × 4 block of world cells that character
cell covers, and it replaces the braille content of that cell. This is a consequence of a character cell being
indivisible for text, not a choice: a cell can carry braille dots or a letter, never both. Positional exactness is
obtained by switching to detail zoom, and the selected Mokiterion's exact coordinates are always shown numerically
in the inspector and the roster, so no exact value is ever only available graphically.

**Detail rendering** places, per world cell, at most one glyph, with this precedence: a Mokiterion glyph over a
resource glyph.

| Entity | Glyph | Colour |
|---|---|---|
| Mokiterion, named | the name's first character, uppercased | by current territory |
| Mokiterion, name not received | `?` | by current territory |
| Resource, low | `○` | class colour |
| Resource, medium | `◎` | class colour |
| Resource, high | `●` | class colour |

Mokiterion glyphs are derived mechanically from the name the engine reported for the subject, which `SPEC-MOK-001`
fixes for `M01` through `M12` as `Zug`, `Krul`, `Quib`, `Sput`, `Trok`, `Womp`, `Hozz`, `Nurb`, `Vonk`, `Gorm`, `Xob`
and `Drix`, so the twelve glyphs are `Z`, `K`, `Q`, `S`, `T`, `W`, `H`, `N`, `V`, `G`, `X` and `D`. That
specification fixes the twelve first characters as pairwise distinct, which is what rule 2.5 relies on and what the
previous assignment obtained by construction instead.

Before this amendment the table read `Mokiterion, M01–M09 → 1–9` and `Mokiterion, M10–M12 → A, B, C`, and the
paragraph here stated that glyphs "are derived mechanically from the engine's identifiers, which are `M01`–`M12` and
carry no names", and that "when agent naming is introduced by a later phase, the glyph becomes the name's first
character and this table is amended; nothing here anticipates that value". That is the phase, and this is that
amendment. The anticipation is retained here rather than deleted: it is why the identifier-derived assignment was
correct while no name existed, and it is the condition this amendment satisfies rather than waives.

The glyph is derived from the name and from nothing else. The observer holds no name table and no identifier-to-name
derivation; the name reaches it in the engine's own `agent_initialized` record. A Mokiterion for which no name was
received is drawn as `?` — a stated character, not the identifier, not a digit and not a guess — which is
unreachable in a run the engine initialized, because `SPEC-MOK-001` rule 1 names every Mokiterion before tick 1.

4. **Shared cells.** When two or more Mokiterions fall in the same rendered cell, the one with the lowest identifier
   is drawn and the cell is underlined to mark the cell as shared. The count of Mokiterions in the selected cell is
   shown in the inspector. Two resources sharing an overview dot are indistinguishable by construction; the
   authoritative per-territory, per-class counts of rule 3 are the exact figures.
5. **Colour independence.** Every distinction that carries identity is available without colour: Mokiterions by
   glyph, resource class by glyph in detail zoom, sharing by underline, territory by position relative to the
   boundary. Colour is redundant reinforcement in every case. In overview zoom, per-resource class is not encoded
   at all — a single dot cannot carry three states and a character cell has one foreground colour for eight world
   cells — and class is obtained from rule 3's counts or from detail zoom.
6. **Panning** moves `camera` by one world cell per press, or by one visible region per paged press, clamped so the
   visible region never leaves the world. **Following** sets `camera` each frame so the selected Mokiterion is
   centred, clamped identically; following is ignored while nothing is selected.

### Rule 3 — Territory resource headline

For each territory the observer presents the standing resource count, its breakdown into low, medium and high, and
the capacity implied by the run's density.

1. A territory whose standing count is `0` is presented as **permanently depleted**, not as a count of zero, because
   `SPEC-MOK-001` makes regeneration conditional on at least one remaining resource and the state is therefore
   irreversible.
2. A territory whose standing count is `1` is presented as **one from sterile**, since consuming that resource
   destroys the territory for the remainder of the run.
3. Both indications are textual as well as coloured.

### Rule 4 — Roster

The roster lists every living Mokiterion in ascending identifier order, which is the order in which
`SPEC-MOK-001` processes them, so reading position in the roster corresponds to acting order.

Each entry occupies two lines at widths of 47 columns or more:

```text
Trok  M05  A  81:14         eat F0058
     h ████████████████████ 100  s ████████████████░░░░  81  e ██████████████░░░░░░  72  f ████░░░░░░░░░░░░░░░░  20
```

Line one carries the name, the identifier, current territory, position, and the action the engine applied on the most
recently completed tick, in that order. Line two carries health, satiety, energy and fear, each as a proportional bar
of at most twenty cells and a numeric value. Below 47 columns each entry collapses to one line carrying the name, the
identifier, territory and the four numeric values without bars.

**The name is presented in addition to the identifier, not instead of it, and it precedes it.** The identifier is the
join key into the log pane, the export and every retained stream, so an operator cross-referencing a roster row
against an engine record must not have to translate. The name is first because it is what the operator reads to tell
one Mokiterion from another; the identifier follows as the reference.

**Line two and its arithmetic are untouched by the name.** The name occupies six columns of line one, which carries
name, identifier, territory and position in fixed fields and the applied action last, and it is the only line the
name appears on. So the bar row's five leading columns, its `5 + 4 * 6 + 3 * 2 = 35` columns of overhead and
`bar_width(interior) = min(20, (interior - 35) / 4)` are all unchanged, and the reference roster's 45-column interior
still yields two-cell bars. Line one's fixed fields total `6 + 5 + 3 + 14 = 28` columns before the applied action,
which leaves 17 at that interior, so the name costs no other field a column and truncates nothing. `SPEC-MOK-001`
bounds a name at five characters, which the six-column field holds with its separating space.

1. Twelve living entries in the two-line form require 24 lines plus the pane border, which the reference viewport
   provides; the no-scroll obligation of `REQ-MOK-020` is an obligation at the reference size and rule 5 states what
   happens below it.
2. The living count is presented in the pane title.
3. A Mokiterion is removed from the roster on the tick its death is applied. The pane states the number of deaths so
   far, so a disappearance is corroborated by a total.
4. A value of `0` renders as `0` with an empty bar, which is distinguishable from an absent value because absent
   values render as `—`.
5. Attributes the engine does not compute are absent. The line-two bar row carries four gauges, the fourth being
   `fear`, which `SPEC-MOK-001` rule 12 computes and reports. Item 4 governs its zero case like any other: `fear 0`
   renders as `0` with an empty bar, and it is a computed zero rather than an inert one.

   This item previously reserved the fourth slot instead of filling it, requiring it to render "empty with no label,
   no dash and no zero" because "an inert `fear 0` would be a claim the engine cannot support". That reasoning is
   retained here rather than deleted: it is what made an empty slot correct while the engine computed three
   attributes, and it is the condition this amendment satisfies rather than waives.

   **The bar width follows from the fourth gauge, and the consequence is stated rather than left to be discovered.**
   The row is five leading columns, then four groups of label, space, bar, space and a three-column value, separated
   by two columns: `5 + 4 * 6 + 3 * 2 = 35` columns of overhead and four bars. So
   `bar_width(interior) = min(20, (interior - 35) / 4)`, replacing the three-gauge rule
   `min(20, (interior - 27) / 3)`. At the reference roster's 45-column interior the bars therefore narrow from
   `(45 - 27) / 3 = 6` cells to `(45 - 35) / 4 = 2`, while the three-column numeric values are unaffected at every
   width. The narrowing was accepted rather than avoided: widening the roster pane in rule 5 would have taken
   fourteen columns from the map pane, and raising this rule's 47-column two-line threshold would have cost bars
   entirely to operators between 47 and 60 columns. This also closes `VREC-MOK-005` finding 3, which recorded that
   the reserved slot was zero-wide at the reference roster and therefore absent there rather than empty.
6. Selecting a roster entry and selecting a Mokiterion are the same operation; the selected entry is highlighted by
   reversed video, not by colour alone.
7. **Survival bands.** Each of the three survival bars — health, satiety and energy — carries a colour band read from
   the value it presents: green at `80..=100`, orange at `40..=79`, red at `0..=39`. The band applies to the gauge as
   a whole — its label character, its bar cells and its numeric value — so one gauge reads as one state; the two
   spaces separating gauges and the five-column indent are unstyled. A band is a second presentation of the number the
   bar already shows. It introduces no quantity the engine does not compute, no trend, and no threshold borrowed from
   anything else: in particular it is not `SPEC-MOK-001` rule 5's reference-source sleep threshold of `20`, which is
   one decision source's policy rather than a survival state, and which a Phase 2 decision source need not share.
   `REQ-MOK-020`'s constraint against derived survival estimates is therefore unaffected. Level stays available
   without colour through the numeric value and the proportional fill, so rule 2.5 holds and colour is redundant
   reinforcement here as everywhere else. Zero takes the red band and still renders as `0` with an empty bar under
   clause 4, which remains what distinguishes it from an absent value. Banding changes no character of the entry: the
   rendered text is identical with and without it, and clause 4's mockup stands unchanged. A selected entry's reversed
   video composes with the band rather than replacing it, so clause 6 is unaffected; the band colour becomes the
   reversed cell's background there, and selection remains marked by reversal rather than by colour. The collapsed
   one-line form below 47 columns has no bars and takes no band: its four numeric values are unstyled, because that
   form exists to keep the numbers legible where the bar cells will not fit, and the numbers carry the level directly.

   **The fourth gauge takes no band.** `fear`, which clause 5 fills the reserved slot with, renders as a bar and a
   numeric value with no colour at all, in the two-line form as in the collapsed one. The three bands are a
   survival scale, and on that scale a high value is a good one; `fear` inverts it, so a banded `fear 100` would
   read green while naming the worst state that attribute has. Giving `fear` a second scale of its own, running the
   other way, was declined: it would put two contradictory colour meanings on one row, and a reader would have to
   know which gauge a colour belongs to before knowing what the colour says. Leaving it unstyled costs nothing that
   rule 2.5 protects, because `fear`'s level is carried by its numeric value and its proportional fill exactly as
   the other three are. This is the single point at which clause 5 and this clause meet, and it is decided rather
   than derived: neither provision forces it.

### Rule 5 — Layout and degradation

Layout is a pure function of viewport width `W` and height `H`. It depends on nothing else — not tick, not run
state, not entropy, not wall-clock time — so the same dimensions always produce the same layout.

**Floor.** When `W < 34` or `H < 22`, the observer does not enter the terminal. It writes the current and required
dimensions to standard error and exits `2`. The floor is derived: a canvas of 32 × 16 cells is the minimum fidelity
of rule 2, its pane border adds two cells in each axis, and the header and footer occupy four rows.

**Panes.** Each pane is present or absent on a threshold in the one axis that constrains it. The combination that
applies at a viewport is whatever those thresholds independently decide; there is no ordered table of named
configurations and no viewport that matches none of them.

| Pane | Present when | Size and position |
|---|---|---|
| header | always | `3` rows, topmost |
| footer | always | `1` row, bottommost |
| roster | `W ≥ 100` | `47` columns, leftmost in the body |
| inspector | `W ≥ 140` | `44` columns, rightmost in the body |
| log | `H ≥ 38` | `10` rows when `W ≥ 140` and `H ≥ 48`, otherwise `6`; below the body |
| view | always | every column and every row the body has left |

The axis each threshold reads is the axis that constrains the pane. The roster is a vertical list in a fixed-width
column, so it is decided by width; a short roster shows fewer entries and says how many are hidden, which rule 4 and
the announcement below already provide for. The log is a fixed-width band of rows, so it is decided by height. The
inspector needs width for the roster and a usable view beside it, so it is decided by width.

The view absorbs the remainder in both axes, so the body is always covered exactly and no pane is drawn outside the
viewport. A pane the current size excludes is reachable as a full-body overlay by its bound key. The header and the
footer are never excluded, because the footer carries the provenance of rule 8 and a frame without provenance cannot
serve as evidence.

**Monotonicity.** For any two viewports above the floor with `W' ≥ W` and `H' ≥ H`, every pane present at `W × H` is
also present at `W' × H'`. Enlarging a terminal never removes a pane, and shrinking one never adds a pane. This holds
by construction, because each pane's presence is one threshold on one axis, and unlike the derived figures below it is
checkable over the whole plane rather than at named sizes.

Canvas *area* is deliberately not monotone, and each place it is not is a declared trade rather than a defect.
Crossing `W = 140` introduces the inspector, which takes 44 columns from the view. Crossing `H = 48` at `W ≥ 140`
grows the log from 6 rows to 10. Crossing `H = 38` introduces the log, which takes 6 rows from the view. In each, a
pane the operator would otherwise have to open as an overlay is worth more than the columns or rows it costs, and the
view states the region it can then present.

**Derived consequences**, which are obligations because they are checkable at named sizes:

| Viewport | Panes besides header, view and footer | Canvas cells | Overview presents |
|---|---|---|---|
| 160 × 48 | roster, inspector, log `10` | 67 × 32 | the whole world at one dot per world cell |
| 160 × 44 | roster, inspector, log `6` | 67 × 32 | the whole world at one dot per world cell |
| 160 × 40 | roster, inspector, log `6` | 67 × 28 | all 128 columns, world rows 0–111 of 128; a region, so annotated |
| 140 × 44 | roster, inspector, log `6` | 47 × 32 | world columns 0–93 of 128; a region, so annotated |
| 140 × 43 | roster, inspector, log `6` | 47 × 31 | world columns 0–93 and rows 0–123 of 128; a region, so annotated |
| 120 × 48 | roster, log `6` | 71 × 36 | the whole world at one dot per world cell |
| 120 × 30 | roster | 71 × 24 | all 128 columns, world rows 0–95 of 128; a region, so annotated |
| 100 × 30 | roster | 51 × 24 | world columns 0–101 and rows 0–95 of 128; a region, so annotated |
| 34 × 22 | none | 32 × 16 | world 64 × 64 of 128 × 128; a region, so annotated |

Each canvas figure is the view pane's interior: the columns and rows the pane occupies less the two cells its border
occupies in each axis. Width alone never suffices. A viewport can be wide enough to address every world column and
still be too short to address every world row, which is what the `120 × 30` row shows: 71 cells address 142 world
columns, more than the 128 that exist, while 24 cells address 96 world rows of 128. Presenting the whole world
requires `Cw ≥ 64` **and** `Ch ≥ 32`, and a canvas that satisfies one and not the other presents a region and is
annotated as one.

The horizontal 1:1 threshold is `W ≥ 157` with the inspector shown, since `47 + 44 + 66 = 157`, and `W ≥ 113` with the
roster but not the inspector, since `47 + 66 = 113`. Between 140 and 156 columns the inspector is retained and the
overview presents a region, which is the declared trade at widths already below the reference size. The vertical 1:1
threshold is `H ≥ 44`: `Ch ≥ 32` needs a body of 34 rows, and the header, footer and a 6-row log take 10 more. Where
the log is 10 rows it is `H ≥ 48`, which is the reference height.

Between 38 and 43 rows the log is present and the overview therefore presents a region in rows, where the same
heights without a log would have addressed every world row. Admitting the log only at `H ≥ 44`, where it costs no
vertical fidelity, was considered and rejected: at those heights the inspector is often already absent, the log
carries the authoritative event stream, and a whole-world view whose events are only reachable as an overlay serves
an operator worse than an annotated region beside a visible log. The technical owner may reverse this trade by
changing one threshold, and nothing else in this rule depends on it.

**Announcement.** Whenever any pane is excluded, any roster entry is not visible, or the view presents a region, the
observer states it: the header lists the panes currently available only as overlays, the roster title states how
many entries are hidden, and the view title states the visible world range.

**Resize.** The layout is recomputed for the new dimensions on the next frame. Selection, filter, zoom, camera,
progression, speed and retained events all survive a resize. A resize below the floor mid-run suspends drawing,
presents nothing, does not terminate the run, and resumes drawing when the viewport is large enough; the simulation
is unaffected either way.

### Rule 6 — Frames and input

1. The observer draws at most one frame every `33` milliseconds, and always draws immediately after a single-step so
   that stepping is never invisible.
2. Input is polled at most every `16` milliseconds. A key press is applied exactly once. Polling never blocks the
   engine and never causes a tick to be skipped or repeated.
3. Draw cadence, input timing and resize events do not reach the engine.
4. An unbound key is ignored: no action, no state change, no diagnostic.

### Rule 7 — Key bindings

| Key | Control |
|---|---|
| `Space` | hold or release progression |
| `.` | advance exactly one tick; accepted only while held |
| `+` / `-` | next faster / next slower speed step, clamped to `1` and `64` |
| `Tab` / `Shift-Tab` | select next / previous living Mokiterion in roster order |
| `Esc` | close an overlay if one is open, otherwise clear the selection |
| `f` | toggle follow |
| `z` | toggle overview and detail zoom |
| `←` `↓` `↑` `→` or `h` `j` `k` `l` | pan one world cell |
| `PageUp` / `PageDown` | pan one visible region vertically |
| `e` | cycle the event-type filter through the vocabulary of rule 9 and none |
| `u` | filter the log to the selected Mokiterion; ignored while nothing is selected |
| `c` | clear the filter |
| `x` | export |
| `t` | open the authority overlay for the highlighted event type |
| `r` / `L` / `i` | open the roster / log / inspector overlay |
| `?` | open the key-binding overlay |
| `q` | quit |

No binding mutates world state. The complete set of operator influence over the simulation is when rule 1 advances
it.

### Rule 8 — Provenance footer

One row, present at every viewport above the floor, containing the entropy seed, the configured tick limit, the
resource density as supplied, the active decision source, the current tick, and the retained-event count with a
truncation marker when `truncated` is set.

1. Values are read from the engine's configuration, so a defaulted value and an explicitly supplied value present
   identically.
2. The candidate commit is displayed when it was supplied to the build as a compile-time value, and the field is
   absent otherwise. The observer does not read repository files, invoke git, or guess.
3. The footer contains no wall-clock time, no absolute path, no environment variable and no credential.

### Rule 9 — Event log, filtering and export

The observer presents the events the engine emits, in authoritative order, with the fields and vocabulary
`SPEC-MOK-001` fixes: `tick`, `subject`, `event`, `result`. It defines no event type and renames no field.

1. The newest events are visible without operator action. Older retained events are reachable by scrolling within
   the log overlay.
2. Filtering by event type restricts the presentation to one of the eleven core types or to `action_trace`.
   Filtering by subject restricts it to one Mokiterion or one territory. Filtering changes presentation only.
3. When a filter matches nothing, the pane states that the filter matched no retained event.
4. **Export** writes every retained event, ignoring any active filter, to the resolved path: `--export` when
   supplied, otherwise `mokiterions-events-seed<seed>-ticks<tick>.log` in the working directory. Records use exactly
   the `SPEC-MOK-001` line format, in authoritative order. A final line states the retained-event count and whether
   truncation occurred.
5. An export contains no wall-clock timestamp, no absolute path, no environment-specific value and no credential.
   Two exports from runs sharing seed, configuration, decision source and stopping tick are byte-identical.
6. A failed export is reported in the header, leaves the run running, and does not present a partial file as
   complete. A partially written file is removed if it can be.

### Rule 10 — Inspector

For the selected Mokiterion the inspector presents its name, its identifier, exact position, territory, health,
satiety, energy, the count of Mokiterions sharing its rendered cell, and the decision record of the most recently
completed tick: the proposed action with its target where it has one, the engine outcome as accepted or rejected, the
engine's stated ground on rejection, and the action applied. The name is presented with the identifier, before it, and
for a dead subject as well as a living one, so rule 10.6's retained selection is identified the same way throughout.

1. Accepted and rejected are distinguished by an explicit word and by symbol, not by colour alone.
2. A rejection is presented as an expected outcome of the authority boundary, never as a program fault or warning.
3. The proposal and the outcome presented are always from the same tick. Presenting a proposal from one tick beside
   an outcome from another is a defect.
4. Before tick 1 completes, the pane states that no proposal has yet been made.
5. With nothing selected, the pane states that nothing is selected. It never defaults to an arbitrary Mokiterion.
6. When the selected Mokiterion dies, the selection is retained and the pane presents the death, the tick of death,
   and the final attribute values. The next selection control moves to the nearest living Mokiterion in roster
   order.
7. Fields for values the engine does not compute — age, kills, combats, remembered locations, model latency
   and per-agent entropy — are absent, not blank-labelled and not zero-filled.

   Amended 2026-08-19 under `REQ-MOK-041`: this list named `name`, and the engine now reports one, so this rule's
   presented-value list above carries it and this list must not. The principle is unchanged and is the reason the
   presented name is the engine's own: an observer that derived a name from an identifier, or filled a placeholder in,
   would be presenting a value the engine did not compute, which this item forbids for every value it still names.

   Amended 2026-08-19: this list named `fear` and traits, and the engine now computes both, so naming them here
   asserted something untrue of the engine. Each is nonetheless still absent from this pane, for a different reason
   in each case, and the reasons are stated rather than left as an apparent oversight. `fear` is absent because this
   rule's presented-value list above is not amended by `WO-MOK-010`, whose observer change surface is the roster's
   bar row alone, and rule 4 presents `fear` there for every living Mokiterion including the selected one, so no
   value is unreachable. The trait is absent because `SPEC-MOK-002` rule 5 deliberately keeps `waste_tolerance` off
   `AgentSnapshot`, so the read-only interface this specification consumes does not carry it; it reaches a host only
   through the retained event log. Presenting either in this pane is a later decision, and either would need this
   rule's presented-value list amended first.

### Rule 11 — Authority mapping

The observer carries a static, exhaustive mapping from event type to the identifier of the requirement that
authorizes the behavior the event reports. The `t` control presents it for the highlighted event type.

| Event type | Authorizing requirement |
|---|---|
| `world_initialized` | `REQ-MOK-001` |
| `food_initialized` | `REQ-MOK-001` |
| `agent_initialized` | `REQ-MOK-002` |
| `decision_source_selected` | `REQ-MOK-008` when the source is `baseline`, `REQ-MOK-015` when `reference`, `REQ-MOK-033` when `individual` |
| `survival_changed` | `REQ-MOK-003` |
| `agent_died` | `REQ-MOK-003` |
| `food_consumed` | `REQ-MOK-006` |
| `food_regenerated` | `REQ-MOK-007` |
| `food_regeneration_skipped` | `REQ-MOK-007` |
| `territory_crossed` | `REQ-MOK-005` |
| `simulation_ended` | `REQ-MOK-011` |
| `action_trace` | `REQ-MOK-012` |

The inspector's proposal-and-outcome presentation maps to `REQ-MOK-004`, and perceived-entity information maps to
`REQ-MOK-013`.

1. The mapping names identifiers only. It never restates requirement text, which could drift from the artifact that
   holds it.
2. Every event type the observer can present has an entry. A type without one is a defect in this table, and the
   observer states that the mapping is missing rather than presenting a plausible identifier.

### Rule 12 — Outcome preservation

1. The observer's only call that changes simulation state is the single-tick advance of rule 1.
2. The observer draws no value from the simulation entropy source. Entropy draw counts per tick are identical
   observed and unobserved.
3. The observer does not reorder ticks, reorder agent turns within a tick, skip a tick, or apply a tick twice.
4. A failure inside the observer never leaves a tick partially applied.
5. A run ended early by the operator yields a prefix of the unobserved run's events, identical up to the stopping
   tick, and reports itself as ended early rather than completed.

## Error and recovery behavior

| Condition | Behavior |
|---|---|
| Invalid start-up input | diagnostic on standard error before entering the terminal; exit `2` |
| Viewport below the floor at start-up | required and actual dimensions on standard error; exit `2` |
| Viewport below the floor mid-run | drawing suspended, run continues, resumes when large enough |
| Draw failure | reported in the header; the run continues; not a simulation result |
| Input read failure | reported in the header; progression continues under rule 1 |
| Export failure | reported in the header; run continues; no partial file presented as complete |
| Terminal cannot be entered | diagnostic on standard error; exit `1`; no frame drawn |
| Panic on any path | terminal restored before the process exits |
| Run finished | advance refused; final state remains inspectable and exportable |

Terminal restoration is unconditional. Leaving an operator's terminal in raw mode on an alternate screen is treated
as a defect of the same severity as a wrong displayed value.

## Data and interface contracts

The engine component exposes a read-only observation surface, and the observer drives it through exactly one mutating
operation.

```text
Simulation::new(Config)                       -> Result<Simulation, String>
Simulation::snapshot(&self)                   -> WorldSnapshot
Simulation::advance_tick(&mut self)           -> Result<TickOutcome, String>
Simulation::is_finished(&self)                -> bool
Simulation::termination_reason(&self)         -> Option<TerminationReason>
Simulation::configuration(&self)              -> Config
Simulation::initialization_events(&self)      -> Vec<Event>
```

This is the whole of what the observer calls. It is not the whole of the library target's public interface, which
`SPEC-MOK-002` rule 5 enumerates and which additionally carries the command-line entry point, the argument parser and
the whole-run method `Simulation::run`.

```text
WorldSnapshot {
  tick, living_count, deaths,
  territories: [TerritorySnapshot; 2],
  agents:      [AgentSnapshot],        // living only, ascending identifier
  resources:   [ResourceSnapshot],     // standing only, stable order
  decisions:   [DecisionSnapshot],     // most recent completed tick, ascending identifier
}

TerritorySnapshot { id, standing, low, medium, high, capacity, permanently_depleted }
AgentSnapshot     { id, position, territory, health, satiety, energy, fear, applied_action }
ResourceSnapshot  { id, position, territory, class }
DecisionSnapshot  { agent_id, proposed, outcome: Accepted | Rejected(ground), applied }
TickOutcome       { events: [Event], finished, reason }
```

1. Every snapshot type contains owned values only: no reference into engine state, no shared handle, no interior
   mutability, and no method that mutates.
2. `advance_tick` is the only operation the observer uses to change simulation state, and the only mutating operation
   this specification adds. Amended 2026-08-18: the library target carries a second `&mut self` method that changes
   state, `Simulation::run`, and this clause previously said there was no such method. `run` is the `REQ-MOK-010`
   whole-run entry point; it predates this specification, `SPEC-MOK-002` rule 5's first list already carries it, and
   it is reachable because rule 3 of that specification places the `simulation` module in the library target — not
   because anything here admits it. Narrowing it away would mean splitting the engine's sources so the whole-run entry
   point leaves the library target, or writing a second run loop, which would give the rules two implementations.
   Amended 2026-08-18: this clause said "relocating the engine's sources, which the component layout below forbids".
   The component layout no longer forbids relocation — the engine's sources are now under `mokiterions-core/` — and the
   move does not narrow `run` away, because moving a directory changes no module's target membership. What would be
   needed is a split, which is what the clause is now stated in terms of and which remains out of scope. So the check
   is stated as it can be met: `grep -n 'pub fn .*&mut self' mokiterions-core/src/simulation.rs` returns exactly `run`
   and `advance_tick`;
   the observer imports neither the whole-run method nor anything that reaches it; and no `&self` method mutates
   through interior mutability, because no engine type contains a `Cell`, a `RefCell`, an `Rc`, an `Arc`, a lock or an
   atomic. Both methods route through one internal step, which is what makes the two hosts execute the identical tick
   sequence.
3. Dependency direction is one-way: the observer depends on the engine; the engine has no knowledge of the observer.
4. Snapshot ordering is stable and specified, so two frames of the same tick present identically.
5. The engine's own `SPEC-MOK-001` behavior — including the text stream, the trace lines and the summary — is
   unchanged by the existence of this surface.

The host process owning the `Simulation` and calling `advance_tick` is the role the existing engine binary already
performs. The invariant `ADR-MOK-001` protects concerns decision sources, which receive an `Observation` and return a
`ProposedAction` and are unchanged by this specification.

### Component layout

**Amended 2026-08-18 for `REQ-MOK-030`.** The tree below and clause 3 stated the layout as it stood when this
specification was conformed to: the engine package at the repository root, sharing one `Cargo.toml` with the
workspace. Each package now has its own directory. `SPEC-MOK-004` rule 1 is the authoritative tree; the one below is
restated to match it so that this specification does not contradict it. Nothing else in this section changes, and
nothing about the observer changes.

```text
Cargo.toml                 # workspace manifest only; declares no package
mokiterions-core/
  Cargo.toml               # package Mokiterions; the engine, its CLI and its binary
  src/
  tests/
mokiterions-tui/
  Cargo.toml               # package mokiterions-tui; the only ratatui dependency
  src/
  tests/
```

1. The engine package's external dependency set is empty and admits no exception, including a dependency shared with
   the observer.
2. The observer package depends on the engine package by path, as
   `Mokiterions = { path = "../mokiterions-core" }`.
3. The engine's sources move as a directory and are not otherwise touched, so the `REQ-MOK-010` text stream does not
   move and its verified behavior is not disturbed by this change. **Amended 2026-08-18.** This clause read "The
   engine's sources are not relocated". What it was protecting is the text stream, not the directory: the reason it
   gave is the whole of its force, and that reason is preserved by moving the files unchanged. `VER-MOK-006` makes
   the engine's byte-identical output across the declared matrix the evidence, rather than the file location that
   used to stand in for it.
4. The engine package, its library target and its binary target keep the names `SPEC-MOK-002` rules 1 and 2 fix:
   `Mokiterions`, `mokiterions` and `Mokiterions`. The observer package and its binary are both named
   `mokiterions-tui`. The observer reaches the engine as `use mokiterions::…`.
5. `cargo test` at the workspace root runs both packages' tests. `cargo tree -p Mokiterions` demonstrates the
   empty set required by `REQ-MOK-026`.

## Security and privacy properties

- No network access, no credential, no model provider, no asynchronous runtime, no database in either component.
- The filesystem is written once per requested export and never read.
- An operator-supplied export path is data. It is never interpreted as code and never used to read.
- No credential, secret, environment variable, absolute path or wall-clock value appears in a frame or an export.
- The observer receives no mutable handle to world, agent, resource or event-log state, and offers the operator no
  control that mutates the world.
- The observer's dependency surface is 57 crates and is confined to the observer package, so it cannot reach the
  engine. Whether that surface is acceptable is decided by `ADR-MOK-003`, not here.

## Performance and capacity

- A frame at the reference viewport renders at most 12 Mokiterion glyphs and the standing resources of the run; at
  the default density that is 122 dots. Work per frame is bounded by viewport area and by population, not by tick
  count.
- Frames are bounded to one per 33 ms and input polling to one per 16 ms, so observer work per wall-clock second is
  bounded independently of speed.
- At speed `64` the observer advances at most 64 ticks per second; the engine's own per-tick work is bounded by
  `SPEC-MOK-001` and unchanged.
- The event buffer is bounded at `100_000` records. Memory is therefore bounded for an unbounded run, and the bound
  is declared rather than silent.
- Falling behind the requested speed slows the run and never alters it.

## Observability

- The header reports observer conditions: draw failures, input failures, export outcomes, panes available only as
  overlays, and hidden roster entries.
- The footer reports run provenance per rule 8.
- The export is the observer's retainable artifact and the only observer output admissible as evidence.
- Observer diagnostics are never written into an export, so an export contains authoritative events only.

## Compatibility and migration

- Additive. No `SPEC-MOK-001` behavior changes: the engine binary, its inputs, its text stream, its trace lines, its
  summary and its exit codes are untouched.
- The engine's library target already exists: `SPEC-MOK-002` rule 3 declares it and rule 5 enumerates its public
  interface. This specification adds the read-only observation surface to that enumeration; it does not create the
  target. Rule 5 provides for exactly this — the interface "grows only when an approved requirement needs it to grow,
  and this specification is amended in the same act" — so the growth is anticipated rather than a departure. Adding
  it changes no existing output.
- The engine package name and both engine target names are unchanged, so the produced binary's filename is unchanged
  and the first line of `USAGE` is untouched. `SPEC-MOK-002` rule 2 ties that line to the binary's name, and
  `SPEC-MOK-001`'s *Help output* section fixes its content while `VER-MOK-004` verifies it.
- Four provisions of `SPEC-MOK-002` cannot be satisfied as written and must be amended before this specification can
  be conformed to:
  - **Rule 1** admits "no third target, no second package, no workspace". `REQ-MOK-026` requires a second package,
    and it is the approved requirement that rule 1 and `ARCH-MOK-001`'s prohibited-pattern list both reserve the
    exception for. The amendment permits a workspace of exactly two packages and keeps every other clause of rule 1,
    including the empty dependency table for the engine package.
  - **Rule 3** states that `src/cli.rs` and `src/simulation.rs` "keep their current contents apart from the visibility
    changes rule 5 authorizes and the test relocations rules 7 to 9 require". The observation surface is new code in
    `src/simulation.rs`, not a visibility change, so as written the clause forbids it — and read as a standing rule it
    would freeze the engine against every later phase. The amendment scopes it to the `WO-MOK-003` restructuring it
    was written for. Rule 11's equivalence obligation is untouched and still binds this change.
  - **Rule 5** closes the library target's public interface to an enumeration of fourteen items. The observation
    surface this specification's *Data and interface contracts* section defines adds to it, and rule 5's own growth
    clause is the authority for doing so.
  - **Rule 6** forbids `Coordinate`, `Direction`, `Territory`, `FoodClass` and `Action` from being public, and forbids
    reaching any prohibited item "by … return value". The snapshots carry all five by value, and every accessor
    returns an owned copy, so the prohibition must be narrowed from the named types and from the return-value path to
    the capability it was written to deny: a mutable borrow of, or a reference into, authoritative state. The other
    **ten** names in that clause — `Mokiterion`, `Food`, `RelativeDirection`, `ActionResult`, `Observation`,
    `PerceivedFood`, `PerceivedMokiterion`, `SplitMix64`, `DecisionEntropy` and `DecisionSource` — stay prohibited and
    stay private. `Observation` and `DecisionSource` are the two that carry the `ADR-MOK-001` trust boundary, and the
    observer never reaches them: it watches, it does not decide.

  Each amendment is the technical owner's act, and `WO-MOK-005` makes all four an approval precondition, exactly as
  `SPEC-MOK-002` itself did for the `ARCH-MOK-001` and `SPEC-MOK-001` amendments that it required.
- `Cargo.toml` and `Cargo.lock` change from an empty dependency set to a workspace with a 57-crate observer surface.
  This is the change `ADR-MOK-003` decides and `ARCH-MOK-001` must be amended to permit.
- When a later phase adds an attribute the observer reserves space for, this specification is amended to define its
  presentation. Nothing here presents such a value before then.

## Examples and counterexamples

### Example: the reference viewport

At 160 × 48, every pane is present and the log has its full 10 rows. The canvas is 67 × 32 cells, so the whole
128 × 128 world appears at one dot per world cell with territory A above territory B. Twelve roster entries are
visible in the two-line form without scrolling.
The inspector occupies 44 columns. The log shows 8 records. The footer reads the seed, tick limit, density, source,
tick and retained count.

### Example: single-stepping to a rejection

Held at tick 40 with `M03` selected at the world's western edge. Pressing `.` completes tick 41 exactly once. The
inspector shows the proposed westward move, the outcome `rejected` with the engine's ground, and no applied
movement. The log gains tick 41's records. Pressing `t` on the highlighted `action_trace` type presents
`REQ-MOK-012`.

### Example: a shrinking terminal

At 160 × 48 the operator narrows the terminal to 120 columns, crossing the inspector's `W ≥ 140` threshold: the
inspector leaves the body and the header states that it is available as an overlay; the log shrinks to 6 rows, since
the taller log needs both thresholds; the roster keeps 47 columns, since 120 is above its own threshold; the canvas
becomes 71 × 36 and still presents the whole world, since 71 ≥ 64 and 36 ≥ 32. Selection, filter, zoom and retained
events are unchanged, and the run does not pause.

### Counterexample: territory A drawn below territory B

The canvas coordinate system increases `y` upward. Plotting world `y` directly places territory A at the bottom,
contradicting rule 2.1 and every diagram in the project. The mapping `canvas_y = 127 − world_y` is required.

### Counterexample: letters in the 1:1 overview

Presenting a Mokiterion as a braille dot in overview zoom to preserve exact position defeats `REQ-MOK-019`'s
requirement that Mokiterions be distinguishable from resources, since both would be dots. Presenting resource class
by glyph in overview zoom is impossible, since a glyph consumes the whole character cell that eight world cells share.
Rule 2 resolves both: letters at cell granularity over dots, and class in detail zoom or from rule 3's counts.

### Counterexample: `fear 0`

Rendering the reserved fourth bar as an empty gauge labelled `fear` with value `0` would present a computed value
the engine does not produce. Rule 4.5 requires the reserved space to render with no label and no value.

### Counterexample: a filtered export

Exporting only the records matching the active filter would produce an evidence file whose completeness depended on
an interface setting at the moment of the key press, and no reviewer could distinguish it from a complete export.
Rule 9.4 requires export to ignore the filter.

### Counterexample: catching up

Advancing several ticks in one scheduling opportunity after the observer falls behind would make the number of ticks
applied per unit of wall-clock time depend on host load. It cannot change the run's outcome, because the tick
sequence is the same, but it does make single-stepping and speed meaningless and it hides overload. Rule 1.2 forbids
it.

## Explicitly unspecified decisions

The implementation may choose:

- private Rust type, function, module and file names in both packages, and how rendering is decomposed;
- how snapshots are built internally, provided the specified content, ordering and ownership hold;
- the concrete widget used for each pane, provided the specified content, constraints and announcements hold;
- exact diagnostic and title wording, and the exact palette, provided every distinction remains available without
  colour;
- fixtures and helpers within a tier;
- whether the reserved fourth bar is reserved by layout arithmetic or by a placeholder that renders nothing.

**Amended 2026-08-18 for `REQ-MOK-029`.** The first of those bullets read "test organization, fixtures and helpers".
That grant is withdrawn as to organization: `REQ-MOK-029` and `SPEC-MOK-004` rules 8 to 10 now fix the observer's two
tiers, where each lives, and how a test is assigned to one. The implementation still chooses fixtures and helpers
inside a tier. The grant was taken in good faith and is why all 109 observer tests were in one tier; withdrawing it
is what makes the placement rule binding rather than advisory.

The implementation may not choose: the dependency, its version or its feature set; the package layout or dependency
direction; the observer's target shape or its test-tier placement; the coordinate mapping or orientation; the fidelity
thresholds, rule 5's pane thresholds or the floor; the glyph assignments; the key bindings; the buffer capacity; the
export format or filter semantics; the authority mapping; the snapshot contract; any figure fixed by `SPEC-MOK-001`;
or any lifecycle status.

**Amended 2026-08-18 for `REQ-MOK-028` and `REQ-MOK-030`.** "The package layout" in the sentence above was withheld
from the implementation without being fixed anywhere, because at the time there was one layout and no reason to state
it. `SPEC-MOK-004` rules 1 to 4 now fix it — one directory per package, and the observer's library and binary targets
— so the withholding points at an authority instead of at nothing. The observer's target shape and test-tier
placement are added to the same sentence for the same reason.
