+++
id = "VER-MOK-005"
type = "verification"
title = "Terminal observer and component separation verification"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-17"
updated = "2026-08-19"

[relations]
verifies = [
  "REQ-MOK-019",
  "REQ-MOK-020",
  "REQ-MOK-021",
  "REQ-MOK-022",
  "REQ-MOK-023",
  "REQ-MOK-024",
  "REQ-MOK-025",
  "REQ-MOK-026",
  "REQ-MOK-027",
]
+++

# Verification Contract: Terminal observer and component separation verification

## Amendment record

This section is added by the amendment below. No verification contract in this repository carried one before, because
none had been amended after approval. The format follows `SPEC-MOK-003`'s, so that a change to an approved assurance
artifact is recorded where a reader looks for it rather than inferred from a diff.

| Date | Change | Approval |
|---|---|---|
| 2026-08-19 | Amended to follow `SPEC-MOK-003` rule 5's replacement of the tier table by one pane threshold per axis. The declared viewport set gains `160 × 40`, `140 × 43` and `120 × 30` — the sizes at which the previous tier table dropped the roster, the inspector and the log at once, and which the old set did not reach. The tier-selection case becomes a pane-presence case; the canvas-interior and whole-world cases are restated for the nine viewports, one figure having changed (`100 × 30`, 98 × 24 → 51 × 24). A **layout monotonicity** case and property are added, checked by sweeping every dimension pair in `34 ≤ W ≤ 200` and `22 ≤ H ≤ 60` rather than at named sizes. The third residual-uncertainty bullet is rewritten: the risk it disclosed — a defect just inside a tier boundary, invisible to a set of named viewports — is the risk that materialized, and saying so is worth more than leaving the bullet as a general caution. Evidence retention gains the sweep result. No case about non-perturbation, entropy attribution, the engine boundary, the dependency set, export, authority mapping or terminal restoration changes, and no seed changes. | Approved 2026-08-19 by the repository owner as assurance owner, in the same act as the `SPEC-MOK-003` rule 5 amendment this contract depends on and as the direction to implement both. The implementation agent drafted this text and recorded this approval on the owner's explicit instruction; it holds no authority over either. |

## Independence

Verification checks observable behavior at three boundaries: the engine's public observation surface, the rendered
character buffer, and the process boundary. It does not name private Rust types, functions, modules, or files, and it
does not assert how layout is computed, how snapshots are built, or which widget draws a pane. Cases are stated so
that any implementation satisfying `SPEC-MOK-003` passes.

**Rendering is verified by asserting the character buffer, not by looking at a terminal.** The user-interface library
provides an in-memory backend whose cells are readable, and every presentation claim in this contract is a claim about
those cells: which glyph occupies which position, which cells are underlined, and which strings appear in which pane.
A screenshot, a recording, or a reviewer's recollection is not admissible for any obligation below. This is the whole
reason `ARCH-MOK-002` requires layout and coordinate mapping to be pure functions of viewport size and snapshot
content: a claim about a screen that only a human has seen is the weakest evidence this repository accepts, and the
observer's entire purpose is to be trusted about what it shows.

**Non-perturbation is verified by comparison, not by inspection.** That the observer cannot change a run is not
established by reading the code for mutating calls; it is established by running the same seed, configuration and
decision source twice — once through the engine binary and once through the observer under heavy operator
interaction — and comparing the authoritative event streams byte for byte. Reading the public surface is a
corroborating check, not the primary one.

The declared verification seed set is `0`, `1`, `42`, `123`, and `777`, the same set `VER-MOK-002` declared, so
observed and unobserved runs are compared on runs whose unobserved behavior is already recorded evidence.

The declared viewport set is `160 × 48`, `160 × 44`, `160 × 40`, `140 × 44`, `140 × 43`, `120 × 48`, `120 × 30`,
`100 × 30`, `34 × 22`, and `33 × 21`. It is fixed here so that layout cannot be demonstrated at a viewport chosen
after the fact. `33 × 21` is the one-below-floor case and is expected to be refused, not rendered.

`160 × 40`, `140 × 43` and `120 × 30` are in the set because the layout that preceded `SPEC-MOK-003` rule 5's
2026-08-19 amendment failed at exactly those shapes — wide enough for every pane, short enough that the superseded
tier table matched no row and excluded all three optional panes at once — and because the set that did not include
them reported PASS while the defect was present. A named set is coverage of the sizes named. That is why the
monotonicity case below sweeps the plane instead.

This contract verifies that the observer reports the simulation faithfully and changes nothing about it. It does not
verify any simulation rule. Every world rule remains verified by `VER-MOK-001` and `VER-MOK-002`, and a failure in
this contract is a defect in the observer or in the boundary, never evidence about the world.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-019` | automated-test | Whole world at the reference viewport | At `160 × 48` the canvas interior is 67 × 32 cells; every living Mokiterion and every standing resource is represented; no entity is omitted |
| `REQ-MOK-019` | automated-test | Coordinate mapping and orientation | A Mokiterion at world `y = 0` renders above one at world `y = 127`; the mapping is `canvas_y = 127 − world_y` in effect, asserted by cell position |
| `REQ-MOK-019` | automated-test | Territory boundary is drawn | A horizontal rule occupies the row between world rows 63 and 64 whenever that boundary is in the visible region |
| `REQ-MOK-019` | automated-test | Per-territory standing counts are presented | Both counts appear as text and equal the snapshot's `standing` values |
| `REQ-MOK-019` | automated-test | Dead Mokiterions are not rendered | After a death, no glyph for that identifier appears at any position |
| `REQ-MOK-019` | automated-test | Shared cell precedence and marking | Two Mokiterions in one rendered cell draw the lowest identifier and the cell is underlined |
| `REQ-MOK-019` | automated-test | Region annotation | At `34 × 22` the view title states the visible world range, so absence is not death |
| `REQ-MOK-019` | automated-test | Detail zoom is one cell per world cell | Resource class glyphs `○ ◎ ●` appear per class; a Mokiterion glyph takes precedence over a co-located resource glyph |
| `REQ-MOK-019` | automated-test | Overview encodes no per-resource class | In overview zoom no resource-class glyph appears in the canvas; class is available from the per-territory counts |
| `REQ-MOK-019` | automated-test | Degenerate worlds render | No living Mokiterions, no standing resources, and both at once each draw a frame without panic |
| `REQ-MOK-019` | automated-test | Legibility without colour | Every distinction is present in glyph, position or underline with all styling removed |
| `REQ-MOK-020` | automated-test | Twelve entries without scrolling at the reference viewport | At `160 × 48` all twelve two-line entries are present in the roster pane; none is hidden |
| `REQ-MOK-020` | automated-test | Roster order is acting order | Entries appear in ascending identifier order |
| `REQ-MOK-020` | automated-test | Bars and numerics agree | Each bar's filled cell count equals `round(value / 5)` of its twenty cells, and the numeric value matches the snapshot |
| `REQ-MOK-020` | automated-test | Zero renders as `0` with an empty bar | A zero attribute is distinguishable from an absent one, which renders `—` |
| `REQ-MOK-020` | automated-test | Reserved space carries no value | The reserved fourth bar position contains no label, no dash and no zero |
| `REQ-MOK-020` | automated-test | Collapse below 47 columns | Below 47 columns each entry is one line with the three numeric values and no bars |
| `REQ-MOK-020` | automated-test | Death is corroborated | On the tick a death is applied the entry disappears, the living count decreases, and the death total increases |
| `REQ-MOK-020` | automated-test | Applied action is the engine's | The action shown equals the snapshot's `applied_action` for the most recently completed tick |
| `REQ-MOK-021` | automated-test | Proposal, verdict and applied action are presented | For a selected Mokiterion the inspector shows the proposed action, the outcome word, and the applied action |
| `REQ-MOK-021` | automated-test | A rejection presents the engine's ground | A rejected proposal shows the engine's stated ground, taken from the snapshot and not re-derived |
| `REQ-MOK-021` | automated-test | Rejection is not a fault | No error or warning styling, wording or diagnostic accompanies a rejection |
| `REQ-MOK-021` | automated-test | Proposal and outcome share a tick | Advancing between reads never yields a proposal from one tick beside an outcome from another |
| `REQ-MOK-021` | automated-test | Verdict is never re-derived in the observer | Given a snapshot whose outcome contradicts what a validation rule would produce, the observer presents the snapshot's outcome |
| `REQ-MOK-021` | automated-test | Nothing selected does not default | With no selection the inspector says so and names no Mokiterion |
| `REQ-MOK-021` | automated-test | Before tick 1 | The inspector states that no proposal has been made |
| `REQ-MOK-021` | automated-test | Selected Mokiterion dies | Selection is retained; death, tick of death and final attributes are presented; the next selection control moves to the nearest living entry |
| `REQ-MOK-021` | automated-test | Absent attributes are absent | No field for fear, traits, name, age, kills, combats, remembered locations, latency or per-agent entropy appears |
| `REQ-MOK-022` | automated-test | Newest events visible without operator action | The most recent records occupy the log pane at every declared viewport where the log is present |
| `REQ-MOK-022` | automated-test | Line format is `SPEC-MOK-001`'s | Presented and exported records match `tick=<n> subject=<id> event=<type> result=<details>` |
| `REQ-MOK-022` | automated-test | Type and subject filters restrict presentation only | Filtering changes visible records; the retained buffer and its order are unchanged |
| `REQ-MOK-022` | automated-test | Empty filter result is stated | A filter matching nothing states that it matched no retained event |
| `REQ-MOK-022` | automated-test | Export ignores the active filter | With a filter active, the export contains every retained record |
| `REQ-MOK-022` | automated-test | Export path resolution | `--export` when supplied, otherwise `mokiterions-events-seed<seed>-ticks<tick>.log` in the working directory |
| `REQ-MOK-022` | automated-test | Export trailer | A final line states the retained count and whether truncation occurred |
| `REQ-MOK-022` | automated-test | Export reproducibility | Two exports from runs sharing seed, configuration, source and stopping tick are byte-identical |
| `REQ-MOK-022` | automated-test | Buffer bound and truncation marker | Exceeding `100_000` records drops oldest first, sets `truncated`, and displays and exports the marker |
| `REQ-MOK-022` | automated-test | Export failure handling | An unwritable path is reported in the header, the run continues, and no partial file is presented as complete |
| `REQ-MOK-022` | automated-test | Exports contain no environment values | No wall-clock timestamp, absolute path, environment variable or credential appears |
| `REQ-MOK-023` | automated-test | Every binding in rule 7 acts | Each key produces its specified state change from a defined starting state |
| `REQ-MOK-023` | automated-test | Single-step is accepted only while held | `.` while running changes nothing; `.` while held advances exactly one tick and remains held |
| `REQ-MOK-023` | automated-test | Speed steps are clamped | `+` at `64` and `-` at `1` leave speed unchanged; intermediate presses walk the declared set |
| `REQ-MOK-023` | automated-test | Selection cycles living Mokiterions only | `Tab` and `Shift-Tab` visit every living entry in roster order and no dead one |
| `REQ-MOK-023` | automated-test | Panning is clamped to the world | Panning at every edge never moves the visible region outside the world |
| `REQ-MOK-023` | automated-test | Follow requires a selection | `f` with nothing selected is ignored; with a selection the visible region centres on it, clamped |
| `REQ-MOK-023` | automated-test | `Esc` precedence | `Esc` closes an open overlay; with none open it clears the selection |
| `REQ-MOK-023` | automated-test | Unbound keys are ignored | An unbound key produces no state change, no diagnostic and no frame difference |
| `REQ-MOK-023` | automated-test | A key press is applied exactly once | A single press advances at most one step of its control |
| `REQ-MOK-023` | automated-test | Stepping is never invisible | A frame is drawn immediately after a single-step |
| `REQ-MOK-023` | automated-test | Finished run refuses to advance | After the run reports finished, progression and single-step change nothing, and the final state remains inspectable and exportable |
| `REQ-MOK-024` | automated-test | Pane presence at every declared viewport | Each pane is present exactly when rule 5's threshold for it is met — roster `W ≥ 100`, inspector `W ≥ 140`, log `H ≥ 38` — and its width, height and position match the table |
| `REQ-MOK-024` | automated-test | Log height at every declared viewport | The log occupies 10 rows when `W ≥ 140` and `H ≥ 48`, and 6 rows wherever else it is present |
| `REQ-MOK-024` | automated-test | Layout monotonicity over the plane | For every `34 ≤ W ≤ 200` and `22 ≤ H ≤ 60`, no pane present at `W × H` is absent at `W+1 × H` or at `W × H+1`; enlarging the viewport never removes a pane |
| `REQ-MOK-024` | automated-test | Canvas interior at every declared viewport | Interiors are 67 × 32, 67 × 32, 67 × 28, 47 × 32, 47 × 31, 71 × 36, 71 × 24, 51 × 24, and 32 × 16 respectively |
| `REQ-MOK-024` | automated-test | Whole-world claim per viewport | Whole world at `160 × 48`, `160 × 44` and `120 × 48`; a region at `160 × 40`, `140 × 44`, `140 × 43`, `120 × 30`, `100 × 30` and `34 × 22`, each annotated |
| `REQ-MOK-024` | automated-test | Header and footer are never excluded | Both are present at every viewport above the floor, including the floor itself |
| `REQ-MOK-024` | automated-test | Excluded panes are announced and reachable | The header lists panes available only as overlays, and each is opened by its bound key at that viewport |
| `REQ-MOK-024` | automated-test | Hidden roster entries are counted | When entries do not fit, the roster title states how many are hidden |
| `REQ-MOK-024` | automated-test | Floor is refused at start-up | `33 × 21` writes required and actual dimensions to standard error and exits `2`, drawing no frame |
| `REQ-MOK-024` | automated-test | Floor mid-run suspends drawing only | Resizing below the floor suspends drawing, does not terminate the run, and drawing resumes when large enough |
| `REQ-MOK-024` | automated-test | State survives resize | Selection, filter, zoom, camera, progression, speed and retained events are unchanged across every resize between declared viewports |
| `REQ-MOK-024` | automated-test | Layout is a pure function of dimensions | The same dimensions produce the same layout at different ticks, speeds, selections and run states |
| `REQ-MOK-025` | automated-test | Observed and unobserved streams are identical | On every declared seed, the observer's authoritative events and final state are byte-identical to the engine binary's, under interaction |
| `REQ-MOK-025` | automated-test | Interaction does not perturb | Holding, single-stepping, selecting, panning, zooming, filtering, exporting and resizing during the run change neither stream nor final state |
| `REQ-MOK-025` | automated-test | Entropy draw counts match per tick | Per-tick draw counts are identical observed and unobserved on every declared seed |
| `REQ-MOK-025` | automated-test | Held state consumes nothing | Held across many frames and many key presses, the tick, world state and entropy state are unchanged |
| `REQ-MOK-025` | automated-test | Completed-tick boundary only | No frame presents a partially applied tick; every held state has all twelve agent turns applied or none |
| `REQ-MOK-025` | automated-test | No catch-up | Delaying the loop past several intervals advances at most one tick per scheduling opportunity |
| `REQ-MOK-025` | automated-test | Early exit yields a prefix | A run ended by the operator yields a prefix identical to the unobserved run up to the stopping tick, reported as ended early |
| `REQ-MOK-025` | automated-test | Wall clock reaches no authoritative value | No engine input derives from time, frame cadence, input timing or terminal dimensions |
| `REQ-MOK-025` | automated-test | Observer failure leaves the tick intact | An injected draw, input or export failure leaves no tick partially applied |
| `REQ-MOK-026` | automated-test | Engine dependency set is empty | `cargo tree -p Mokiterions` resolves to the engine package alone |
| `REQ-MOK-026` | automated-test | No engine-to-observer edge | The observer package appears nowhere in the engine package's dependency resolution |
| `REQ-MOK-026` | automated-test | User-interface dependency is confined | `ratatui` and its transitive crates appear only in the observer package's resolution |
| `REQ-MOK-026` | automated-test | Engine builds and tests without a terminal | The engine package's build and tests pass with no terminal attached and the observer package excluded |
| `REQ-MOK-026` | automated-test | Surface exposes one mutating operation | The public surface has no `&mut self` operation other than the single-tick advance |
| `REQ-MOK-026` | automated-test | Snapshots are owned and inert | Snapshot types hold owned values, expose no mutating method, and outlive a subsequent advance unchanged |
| `REQ-MOK-026` | automated-test | Advance takes no operator data | The advance operation accepts no argument derived from operator input |
| `REQ-MOK-026` | evidence | Resolved dependency graph and feature set | `serde` absent; features exactly `crossterm`, `layout-cache`, `underline-color`; measured crate count recorded |
| `REQ-MOK-026` | automated-test | Two packages exactly | The workspace contains exactly `Mokiterions` and `mokiterions-tui` |
| `REQ-MOK-027` | automated-test | Footer provenance fields | Seed, tick limit, density as supplied, active source, current tick and retained count are present at every viewport |
| `REQ-MOK-027` | automated-test | Defaulted and explicit values present identically | A defaulted density and the same density supplied explicitly produce the same footer text |
| `REQ-MOK-027` | automated-test | Commit field is compile-time or absent | Present when supplied to the build, absent otherwise; no repository read and no version-control invocation occurs |
| `REQ-MOK-027` | automated-test | Footer carries no environment values | No wall-clock time, absolute path, environment variable or credential appears |
| `REQ-MOK-027` | automated-test | Authority mapping is exhaustive | Every event type the observer can present has an entry; `t` presents the mapped identifier |
| `REQ-MOK-027` | automated-test | Source-dependent mapping | `decision_source_selected` maps to `REQ-MOK-008` under `baseline` and `REQ-MOK-015` under `reference` |
| `REQ-MOK-027` | automated-test | A missing mapping is stated, not guessed | An event type without an entry causes the observer to state that the mapping is missing |
| `REQ-MOK-027` | automated-test | Mapping names identifiers only | No requirement text is reproduced in the observer |

## Acceptance scenarios

1. At `160 × 48` on seed `42`, a frame shows twelve roster entries, the whole 128 × 128 world with territory A above
   territory B, both territory counts, an inspector stating nothing is selected, and a footer naming seed `42`.
2. Held at a tick with a Mokiterion at the world's western edge selected, pressing `.` once completes exactly one
   tick; the inspector shows the proposed westward move, the outcome `rejected` with the engine's ground, and no
   applied movement; the log gains that tick's records; pressing `t` on the highlighted `action_trace` type presents
   `REQ-MOK-012`.
3. A run driven to completion through the observer on each declared seed, with the operator holding, stepping,
   selecting, panning, zooming, filtering, exporting and resizing throughout, produces an export byte-identical to
   the engine binary's text stream for the same seed and configuration, record for record.
4. The operator narrows the terminal from `160 × 48` to `120 × 48`; the inspector leaves the body and the header says
   it is available as an overlay; the log shrinks to six rows; the canvas becomes 71 × 36 and still presents the whole
   world; selection, filter and zoom are unchanged; the run does not pause.
5. The operator reduces the terminal from `160 × 48` to `160 × 40`, crossing no pane threshold. The roster, the
   inspector and the log are all still present; the log holds six rows rather than ten; the canvas becomes 67 × 28 and
   the view title states world rows 0–111 of 128; selection, filter and zoom are unchanged; the run does not pause.
   This is the shape the superseded tier table presented with no roster, no inspector and no log.
6. Starting at `33 × 21` prints the required and actual dimensions to standard error, exits `2`, and draws nothing.
7. A territory reaching one standing resource is presented as one from sterile, and on reaching zero as permanently
   depleted rather than as a count of zero, while the other territory continues to render normally.
8. Quitting normally, exiting on an injected terminal failure, and panicking each leave the terminal out of raw mode
   and off the alternate screen.

## Property and invariant tests

- **Non-perturbation.** For every declared seed, the multiset and order of authoritative events and the final state
  are identical between an observed and an unobserved run. This is the contract's primary property; every other
  property below is subordinate to it.
- **Entropy attribution.** Per-tick entropy draw counts are identical observed and unobserved, so the observer draws
  nothing.
- **Rendering purity.** Drawing a frame leaves world state, entropy state and the tick unchanged. Drawing the same
  snapshot at the same viewport twice produces identical buffers.
- **Layout purity.** Layout and the world-to-canvas mapping are functions of viewport size and snapshot content
  alone. Varying tick, speed, progression, selection, elapsed time and event count with dimensions fixed produces
  identical pane geometry.
- **Layout monotonicity.** For every viewport above the floor, every pane present at `W × H` is present at every
  `W' × H'` with `W' ≥ W` and `H' ≥ H`. Enlarging a terminal never removes a pane. This is asserted by sweeping the
  plane, not at the declared viewports, because the defect it exists to exclude was invisible to a set of named
  sizes. Canvas area is deliberately not monotone and is not asserted to be: rule 5 declares the inspector at
  `W = 140`, the log at `H = 38` and the taller log at `H = 48` as trades of area for a pane.
- **Mapping injectivity.** Wherever the view claims one dot per world cell, distinct world cells map to distinct
  dots. Where it does not, the view is annotated as a region.
- **Orientation.** For every pair of world rows `y1 < y2` in the visible region, `y1` renders on a screen row at or
  above `y2`. No viewport, zoom or camera position inverts this.
- **Presentation faithfulness.** Every numeric value on screen equals the corresponding snapshot field. No displayed
  quantity is computed by the observer from anything other than snapshot content.
- **No invented state.** No field, label, bar or value appears for an attribute absent from the snapshot contract.
- **Filter neutrality.** Applying and clearing any filter leaves the retained buffer, its order and its count
  unchanged, and leaves the export unchanged.
- **Retention bound.** The retained record count never exceeds `100_000`, and whenever a record has been dropped the
  truncation marker is set and remains set.
- **Idempotent input.** Applying the same key press twice performs its control twice, and applying an unbound key any
  number of times changes nothing.
- **Tick atomicity.** Every observable state has a whole number of applied ticks. No injected failure produces a
  partially applied tick.
- **Terminal restoration.** For every exit path, including panic, raw mode is off and the alternate screen is left.
- **Boundary immutability.** No public operation other than the single-tick advance changes engine state, and a
  snapshot taken before an advance is unchanged after it.

## Static and architecture checks

- `cargo fmt --all -- --check` reports no differences across the workspace.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` reports no findings.
- `cargo tree -p Mokiterions` resolves to the engine package alone, with no external crate.
- `cargo build -p Mokiterions` and `cargo test -p Mokiterions` succeed with the observer package's
  dependencies unavailable to them and with no terminal attached.
- The engine package's manifest declares no dependency, and no user-interface, networking, asynchronous-runtime,
  serialization or database crate appears in any engine-package resolution.
- The observer package's resolved graph matches the specified version and feature set exactly; `serde` is absent.
- The workspace contains exactly two packages.
- The engine's sources are not relocated, so the `REQ-MOK-010` text stream's implementation position is unchanged.
- Every existing `VER-MOK-001` and `VER-MOK-002` test still passes unmodified. A test changed to accommodate this
  work is an adverse finding requiring explanation, since this change is specified as additive.
- The observation-to-proposed-action boundary is unchanged: no decision source gains mutable state, and the observer
  is not a decision source.

## Security and privacy checks

- No network access, credential read, model provider, asynchronous runtime or database appears in either package.
- The filesystem is written only by an operator-requested export, and never read. No start-up path opens `--export`.
- An operator-supplied export path is treated as data: never executed, never used to read, never expanded.
- No credential, secret, environment variable, absolute path or wall-clock value appears in any frame or export.
- Standard-error diagnostics are never interleaved with frames, and observer diagnostics never enter an export.
- No repository file is read and no version-control command is invoked at run time.
- The operator has no control that mutates world state; the complete set of operator influence is when a tick is
  advanced.

## Performance and resilience checks

- A `10,000`-tick run through the observer at speed `64` completes without panic, and survivors plus deaths equal
  twelve.
- Frames are bounded to one per `33` ms and input polling to one per `16` ms, so observer work per wall-clock second
  is bounded independently of speed.
- Per-frame work is bounded by viewport area, population and standing resource count, not by tick count or by
  retained event count.
- Memory is bounded for an unbounded run, with the retained buffer at its declared capacity.
- A run deliberately delayed past several advance intervals finishes with the same events as an undelayed one, only
  later.
- Draw failure, input failure and export failure each leave the run running and are reported in the header.
- Rapid resizing across the declared viewport set, including below the floor and back, leaves the run correct and the
  terminal restorable.

## Manual assessments

- Run the observer for at least two hundred ticks on one declared seed and confirm the instrument answers the three
  questions `INT-MOK-004` names: where the population is, why a selected Mokiterion did what it did, and which
  requirement authorizes a highlighted event. A pass on every automated case above with a negative answer here is an
  adverse observation requiring product review, because the automated cases verify the parts and this assesses the
  instrument.
- Confirm on a real terminal, at the reference viewport, that the whole-world overview is legible: that resource dots
  and Mokiterion letters are distinguishable and that the territory boundary reads as a boundary. Buffer assertions
  prove the cells are correct; only a person can report that the result is readable.
- Confirm the same with colour disabled or on a monochrome terminal, and confirm no distinction is lost.
- Confirm that a rejection in the inspector reads as an ordinary authority outcome rather than as an error.
- Confirm the reserved fourth roster bar position reads as empty space rather than as a missing or broken value.
- Assess whether the overview's cell-granularity Mokiterion glyph is materially misleading about position. It locates
  a subject to within a 2 × 4 block of world cells by construction. If an operator misreads a position because of it,
  that is an adverse observation about rule 2 requiring a specification decision, not a defect to patch in the
  observer.
- Confirm the terminal is usable after a deliberate panic, by inspection of the live terminal rather than only by an
  automated assertion.

## Evidence retention

Retain under `docs/engineering/simulation/evidence/WO-MOK-005/`:

- formatter, linter, test and build output for the workspace and for the engine package alone;
- the requirement-to-test mapping;
- `cargo tree -p Mokiterions` output, as the empty-dependency-set proof for `REQ-MOK-026`;
- the observer package's resolved dependency graph, its measured crate count, and its enabled feature set;
- per-seed observed-versus-unobserved comparison results, including the comparison method and the interaction
  performed during each observed run;
- per-tick entropy draw-count comparisons;
- per-viewport layout and canvas-interior assertions for every declared viewport, and the refusal output for
  `33 × 21`;
- the layout monotonicity sweep result, stating the bounds swept, the number of dimension pairs checked, and the
  count of pairs at which a pane present at a smaller viewport was absent, which is required to be zero;
- one exported event file per declared seed, and the byte-comparison against the engine binary's stream;
- the `10,000`-tick resilience result and the terminal-restoration results for normal exit, error exit and panic;
- the manual assessment record, including the legibility and colour-independence assessments and their author;
- dependency, boundary and credential review;
- a completion summary naming the final affected components.

Rendered frames are retained only as buffer dumps accompanying an assertion. No screenshot or recording is admissible
evidence for any obligation in this contract.

## Residual uncertainty

- **Buffer assertions prove correctness, not legibility.** Every rendering obligation here is verified against an
  in-memory character buffer. That a buffer is correct does not establish that a person can read the result on a real
  terminal with a real font. The manual legibility assessments exist for exactly this gap and cannot be automated
  away.
- **Terminal behavior varies and is not exhaustively covered.** Emulators differ in braille coverage, underline
  support, colour depth and resize semantics. Verification covers the declared viewport set on the development
  environment's terminal. A defect appearing only in another emulator would not be caught here.
- **This contract's viewport coverage failed once, in exactly the way this bullet used to warn about.** Before the
  2026-08-19 amendment it read that layout was verified at seven viewports rather than over the plane, and that a
  defect "particularly just inside a tier boundary" would not be caught. That is what happened: `SPEC-MOK-003` rule
  5's tier table matched no row for `W ≥ 140` with `38 ≤ H < 44`, excluded the roster, the inspector and the log
  together at those sizes, and every declared viewport reported PASS. The disclosure was accurate and it was not
  sufficient, because a disclosed risk with no test behind it is still an untested risk. The monotonicity sweep now
  covers `34 ≤ W ≤ 200` and `22 ≤ H ≤ 60` for pane presence, which is what would have caught it. Two gaps remain and
  are narrower: canvas figures and pane geometry are still asserted only at the nine declared viewports, and the
  sweep bounds are finite, so a defect above 200 columns or 60 rows is outside it.
- **Non-perturbation is verified on five seeds under a scripted interaction sequence.** The property is structural —
  the observer's only mutating call is the advance, and it carries no data — but the evidence is five seeds and one
  interaction script. An operator sequence outside that script is covered by the structural argument and the public
  surface review, not by measurement.
- **The observer's correctness bounds the value of everything it shows.** A rendering defect presents a wrong world
  while the simulation is right, and an operator may then draw a wrong conclusion about behavior that is in fact
  correct. This is why faithfulness is asserted field by field against snapshot values rather than by sampling, and
  it remains the largest residual risk this contract carries.
- **The user-interface library is 57 crates of untrusted third-party code.** Verification confirms the version,
  feature set and containment; it does not audit the library. A defect inside it can corrupt what an operator sees.
  It cannot alter a simulation outcome, because the engine neither depends on it nor receives anything from the
  observer. Whether that surface is acceptable is `ADR-MOK-003`'s decision, not a finding of this contract.
- **The overview locates a Mokiterion to a 2 × 4 block, not a cell.** This is a consequence of a character cell being
  indivisible for text, and it is specified rather than a defect. Verification confirms exact coordinates are always
  available numerically. It cannot rule out an operator reading the glyph as exact.
- **This contract verifies observation, not what is observed.** Passing establishes that the observer reports the
  simulation faithfully and changes nothing about it. It establishes nothing about whether the behavior observed is
  interesting, plausible or correct, which remains the subject of the requirements `VER-MOK-001` and `VER-MOK-002`
  verify.
