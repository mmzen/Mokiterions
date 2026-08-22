+++
id = "VER-MOK-005"
type = "verification"
title = "Terminal observer and component separation verification"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-17"
updated = "2026-08-22"

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
| 2026-08-20 | **Three of the seven manual assessments restated, because each named a subject that has moved since approval.** *Assessment 3* asked for a confirmation "with colour disabled or on a monochrome terminal". The observer offers no way to disable colour — `mokiterions-tui` has no `--no-color` flag and honours no `NO_COLOR` variable — so the assessment as written could not be performed on the shipped binary at all, and its automated counterpart already reads a colourless projection. It is restated as the check that survives and that no test can reach: whether `UNDERLINED` and `REVERSED` render distinguishably on the assessor's terminal, including where both fall on one cell. That is the part `evidence/WO-MOK-005/manual-assessment.md` had already identified as a person's to answer. *Assessment 4* asked that a rejection read as an authority outcome; `VREC-MOK-005` disclosure 5 records that no shipped decision source ever has a proposal rejected over 400 ticks of both policies, so the state is unreachable in a live run. It is restated to name the `#[cfg(test)]` hook route explicitly, so a future assessor is not sent after a state that cannot occur. *Assessment 5* asked whether "the reserved fourth roster bar position reads as empty space"; the slot is no longer reserved, because rule 4 as amended on 2026-08-19 presents a computed `fear` there. It is restated against the slot as it now exists. **No obligation is weakened and none is added**: each restatement points an existing obligation at a subject that exists. No case, property, static check, security check, performance case, declared viewport, seed or retained-evidence item changes, and assessments 1, 2, 6 and 7 are untouched. | Approved 2026-08-20 by the repository owner acting as assurance owner, in the same review that recorded all seven assessments and ratified the eleven outstanding provisions, under `WO-MOK-012`. Each of the three was put as its own question, with the measurement that showed the subject had moved, and decided on its own. The implementation agent measured the three mismatches, wrote this text and decided none of the substance. `VREC-MOK-005` is not edited: it is `ready` and bound to its commit, and what it verified was correct there. |
| 2026-08-20 | **One `REQ-MOK-020` matrix row withdrawn, because its pass condition was never satisfiable.** The row read "Bars and numerics agree — Each bar's filled cell count equals `round(value / 5)` of its twenty cells, and the numeric value matches the snapshot." **Twenty cells is a width the layout cannot produce at any viewport.** `SPEC-MOK-003` rule 5 fixes the roster pane at `47` columns wherever it is present, so its interior is `45`, and rule 4's `bar_width(interior) = min(20, (interior - 35) / 4)` yields **2**; twenty cells would need a `115`-column interior. Before `WO-MOK-010` the divisor was three over an overhead of 27, yielding **6** at the same interior, and twenty would still have needed 87 — so the condition did not hold on the day this contract was approved either. The arithmetic differs as well as the width: the observer computes a truncating `value × width / 100`, where the row specifies `round`, and the two disagree at twenty cells and value 99. **The row is withdrawn rather than corrected**, in favour of `VER-MOK-013`'s obligation that a ten-point value step change the filled-cell count, which is a property holding at every width instead of a figure pinned to one that does not occur. **Nothing becomes unverified.** The row's second clause — that the numeric value matches the snapshot — is carried independently by this contract's own *Presentation faithfulness* invariant, which obliges every numeric value on screen to equal its snapshot field, and `VER-MOK-013` restates it for the gauges explicitly. No other case, property, static check, invariant, manual assessment, seed, declared viewport or retained-evidence item changes. | **Decided 2026-08-20 by the repository owner acting as assurance owner**, on the choice put to them once the defect was measured: amend the row to the implementation's arithmetic at the width the layout produces, withdraw it in favour of `VER-MOK-013`'s property, or leave it and record the defect. The owner chose withdrawal. The implementation agent measured the defect while authoring `WO-MOK-013`, put the choice with its arithmetic, wrote this text, and decided none of the substance. **How the row survived is recorded rather than left implicit**: `evidence/WO-MOK-005/requirement-to-test-mapping.md` maps it to `the_bar_row_reproduces_the_specified_form`, which asserts an exact rendered line for three named values and therefore never exercises the arithmetic the row states — a contract obligation and its test agreeing on an example while disagreeing on the rule. That is why `VER-MOK-013` asserts properties rather than renderings. `VREC-MOK-005` is not edited: it is `ready`, not `verified`, so no verified record asserts this row passed. |
| 2026-08-20 | **Two further `REQ-MOK-020` matrix rows amended, because both describe the three-gauge roster that `SPEC-MOK-003` rule 4 replaced on 2026-08-19.** *Reserved space carries no value* asserted that "the reserved fourth bar position contains no label, no dash and no zero". Rule 4 item 5 as amended replaces that reservation with the presentation of a computed `fear`, and item 4 governs its zero case, so **the row asserted the absence of exactly what the specification now requires be present**: an implementation conforming to rule 4 fails it, and an implementation passing it violates rule 4. It is rewritten to oblige the fourth position to present `fear` in the form of the other three, with a zero rendering as `0` and an empty bar rather than as an absence, and to carry no survival band — which is rule 4 clause 7 as reconciled with clause 5 on 2026-08-19, where `fear` was left unbanded because the three bands are a survival scale on which a high value is a good one and `fear` inverts it. *Collapse below 47 columns* said each collapsed entry carries "the three numeric values"; clause 5 made it four, and `SPEC-MOK-003`'s 2026-08-19 reconciliation row states that correction in terms — "**The collapsed one-line form's count is corrected from three numeric values to four**" — so the specification recorded the change and this contract was not swept with it. The count is corrected to four. **This contract now holds 86 automated cases against the 87 it held at `ff3a155`**, the one loss being the row withdrawn above; these two are amended in place and neither adds nor removes a case. Nothing else changes: no other case, property, static check, invariant, manual assessment, seed, declared viewport or retained-evidence item. **One provision of rule 4 clause 7 has no case here, before this amendment or after it** — that the collapsed one-line form takes no band. This amendment does not add one; the gap is named so that it is a recorded fact rather than coverage a reader infers from the row beside it. | **Decided 2026-08-20 by the repository owner acting as assurance owner**, on the choice put to them once both rows were measured: amend both to the form rule 4 now fixes, withdraw both as stale, or leave them and record the defect. The owner chose to amend both. The implementation agent found them while sweeping for the cause of the withdrawn fill row, measured each against rule 4 and against the rendered entry, wrote this text, and decided none of the substance. **The three stale rows have one cause**: the 2026-08-19 rule 4 amendment moved the roster from three gauges to four and this contract was not swept for the cases asserting the old form. The 2026-08-20 amendment above reached the *manual assessment* that referenced the reserved slot and not the *automated row* asserting the same withdrawn property, which is why the sweep was taken a second time and is the reason `VER-MOK-013`'s cases assert properties rather than renderings. `VREC-MOK-005` is not edited: it is `ready`, not `verified`, so no verified record asserts either row in its former wording. |
| 2026-08-20 | **Every figure and clause here that depends on the ten-row log corrected, because the growth it named is withdrawn.** `SPEC-MOK-003` rule 5 as amended the same day presents the log at **6 rows wherever it is present**, so the ten-row form at `W ≥ 140` and `H ≥ 48` no longer exists. Four matrix rows and one invariant clause change, each of them a figure or a clause about that form and none of them a new obligation. *Log height at every declared viewport* loses the ten-row clause and reads 6 rows wherever the log is present, which is wherever `H ≥ 38`. *Whole world at the reference viewport* and the first of the nine figures in *Canvas interior at every declared viewport* move from `67 × 32` to **`67 × 36`**: at `160 × 48` the body is `48 − 3 − 1 − 6 = 38` rows and the canvas interior is `38 − 2 = 36`. **The other eight interior figures are unmoved and were re-derived rather than assumed** — `160 × 44` and `140 × 44` already had a six-row log, `140 × 43` a body of 33, `120 × 48` and `120 × 30` are below the inspector's threshold, `120 × 30` and `100 × 30` below the log's, and `34 × 22` presents the canvas alone. *Twelve entries without scrolling at the reference viewport* keeps **twelve** and reads *three-line* rather than *two-line*, rule 4 as amended having made the entry three lines: `36 / 3 = 12` exactly, so `REQ-MOK-020` is preserved at the reference viewport and by no margin at all — a seventh log row would leave 35 interior rows and lose an entry. The **Layout monotonicity** invariant's closing parenthetical loses "and the taller log at `H = 48`" from its list of deliberate area trades, leaving the inspector at `W = 140` and the log at `H = 38`; the property itself, its bounds and its sweep are untouched, because they assert pane presence and never area. **Acceptance scenarios 4 and 5 are corrected as well, and they were not enumerated in `WO-MOK-013`**: scenario 4 said the log "shrinks to six rows" on narrowing from `160 × 48` to `120 × 48` and scenario 5 that it "holds six rows rather than ten" at `160 × 40`, and both compare against a ten-row reference that no longer exists. Their figures are unchanged — `71 × 36` and `67 × 28` are what those viewports produce before and after — and only the comparison is corrected. **This contract still holds 86 automated cases**, the same count as after the withdrawal row above: all five in-matrix changes are amendments in place. Nothing else changes — no seed, no declared viewport, no property other than that one parenthetical, no static, security or performance check, no manual assessment, and no retained-evidence item. | Covered by the approval of `WO-MOK-013` on 2026-08-20 by the repository owner acting as **assurance owner**, whose in-scope item 7 enumerates these locations and states that the act is taken once the implementation lands rather than at approval. **The substance was decided earlier and separately**: the withdrawal of the ten-row growth is the owner's decision 1 of 2026-08-20, taken in the `WO-MOK-012` review on the measurement that the ten-row log left the roster 32 interior rows for entries needing 36, and every change in this row is that decision's arithmetic rather than a fresh assurance judgement. **The two acceptance-scenario corrections are the exception and are reported as one**: they are consequences of the same decision at locations the work order's enumeration missed, corrected here on the precedent this repository already set — that an example or scenario illustrating a provision is swept with it — and named in `WO-MOK-013`'s completion report as a miss in the enumeration rather than presented as approved text. The implementation agent measured each figure at the layout, wrote this text and decided none of the substance. `VREC-MOK-005` is not edited: it is `ready` and bound to its commit, and the figures it recorded were correct there. |
| 2026-08-21 | **Two `REQ-MOK-021` matrix rows amended and two added, because `SPEC-MOK-003` rule 10 as amended presents a dead subject's fear and this contract asserted its absence.** *Selected Mokiterion dies* said "final attributes" without a count; it now names **all four** — health, satiety, energy and fear — and the two-line pairing rule 10.6 as amended fixes, with the obligation that no value be clipped off the pane at any viewport presenting the inspector, which is the defect the frame case found. *Absent attributes are absent* is the row this amendment exists for: it listed **fear** among the fields that must not appear, so an implementation conforming to rule 10 as amended fails it and an implementation passing it violates rule 10 — the same contradiction the 2026-08-20 rows above corrected for the roster's reserved slot. Fear is removed from the list and confined to the living case, where the original justification is undiminished. Two cases are added: *A dead subject's final fear is the engine's own*, which obliges the presented value to equal the engine's last reported `survival_changed` fear read from the retained stream, at the state and at the frame; and *An unreported final attribute is absent*, which carries rule 10.7's standing rule into the dead-subject case, including that a pair with neither of its values emits no line. **This contract now holds 88 automated cases against the 86 it held after the rows above.** A residual-uncertainty bullet is added disclosing what the absence case can and cannot reach: the state it needs is constructible only through a private call, so the case sits inside the crate rather than beside the other `REQ-MOK-021` cases, but the rendered pane is in reach from there and the absence is measured at the frame as well as at the derived value, with no widened visibility and no fifth hook. The one clause the bullet discloses as unmeasurable is the obligation that a pair carrying neither value emit no line, since the death branch returns with that line last and a suppressed line is indistinguishable from the pane's unwritten rows. **A second staleness in the same row is corrected and is not this work order's**: it also listed **name** among the absent fields, which `SPEC-MOK-003` rule 10 as amended on 2026-08-19 under `REQ-MOK-041` moved into the presented-value list, so the row has contradicted an approved specification since that date and `WO-MOK-011` did not sweep this contract for it. It is corrected here as a statement of fact about an approved specification rather than as a change of obligation, and it is reported as a found defect in `WO-MOK-018`'s completion report rather than absorbed. Nothing else changes: no seed, no declared viewport, no property, no static, security or performance check, no manual assessment, and no retained-evidence item. | **Approved 2026-08-21 by the repository owner acting as assurance owner**, as amendment 4 of `WO-MOK-018`, in the same act that approved the `SPEC-MOK-003` rule 10 amendment this contract depends on and directed the implementation. The two-line pairing obligation was not a provision the owner was shown at approval, because it was not yet known to be needed: the value was first implemented on one line and the added frame case failed on a clipped line, after which the pairing followed from rule 4's existing arrangement. **The `name` correction is the one part of this row the owner's act does not cover as approved scope**, and it is recorded as an implementation-agent correction of a false statement about another artifact, on this repository's own precedent for that form. The implementation agent found both stalenesses by reading the row against rule 10 as amended, measured the clipping at the layout, wrote this text and decided none of the substance. `VREC-MOK-005` is not edited: it is `ready`, not `verified`, so no verified record asserts either row in its former wording. |
| 2026-08-22 | **The `REQ-MOK-027` footer row rewritten, because its pass condition asserted the field set that `SPEC-MOK-003` rule 8 as amended the same day concedes cannot hold, and nine cases added for the order of loss the amendment fixes.** *Footer provenance fields* obliged seed, tick limit, density as supplied, active source, current tick and retained count to be “present at every viewport”. **That was never satisfiable and this contract reported PASS regardless.** At `34 × 22` a twenty-digit entropy seed beside a twenty-digit tick limit is 43 characters of 34 before a separator, so no implementation could satisfy the row and none was asked to: the row is mapped to a case that renders the default seed `0` at the declared viewports, where every field fits, and the declared verification seed set has at most three digits across its five members. The row is rewritten to oblige the whole field set at the declared viewports for the declared verification seeds, and the entropy seed at every declared viewport for every seed and tick limit the start-up contract accepts, which is rule 8 clause 6. Eight cases are added: two under `REQ-MOK-024` for fidelity — that no value is ever presented cut and that the row fits its pane — and six under `REQ-MOK-027` for clause 4's order of loss, clause 5's pairing of the retained count with its truncation marker, clause 6's guarantee at the floor, clause 8's separation of labelling from value, and the first row of clause 4, that a candidate commit is shed before every field the rule requires. *Commit field is compile-time or absent* is amended in place: `option_env!` reports a variable set to the empty string as a supplied value, and the superseded renderer drew it as a bare `#` carrying nothing, so the row now names all three states of the variable. **This contract now holds 96 automated cases against the 88 it held after the row above**, the rewritten row being one of the seven `REQ-MOK-027` rows now standing where it stood alone. A **Provenance survival** property is added, a declared footer seed set and a declared footer tick-limit set are fixed in *Independence*, three retained-evidence items are added under `WO-MOK-008`, and two residual-uncertainty bullets are added — one disclosing that these cases verify the amended rule and not `REQ-MOK-027` itself, the other that a passing case on the defective tree measured nothing and that most of this contract's did. Nothing else changes: no verification seed, no declared viewport, no other property, no static, security or performance check, no manual assessment, and no acceptance scenario. | **Approved 2026-08-22 by the repository owner acting as assurance owner**, in the same act that approved the `SPEC-MOK-003` rule 8 amendment this contract depends on, that fixed clause 4's order over the six preamble fields, and that directed the implementation and the evidence in one session. The owner was shown the measured floor cost of each rejected alternative before deciding. **Two parts of this row the owner's act does not cover as approved scope and both are named rather than absorbed.** The first is clause 4's first row, placing the candidate commit ahead of every other field: the owner decided the order over the six preamble fields, the commit's position is OUTSTANDING in `SPEC-MOK-003`'s row of the same date, and the case added here measures the implementation's conformance to a provision that is not yet ratified — if the position changes, this case changes with it. The second is the disclosed residual against `REQ-MOK-027`, which is the product owner's and is recorded below rather than resolved. **How the rewritten row survived is recorded rather than left implicit**: `evidence/WO-MOK-005/requirement-to-test-mapping.md` maps it to a case asserting a rendered footer for the default configuration, so the row's universal quantifier over viewports was never exercised against a seed wide enough to test it — the same shape as the withdrawn `REQ-MOK-020` fill row of 2026-08-20, a contract obligation and its test agreeing on an example while disagreeing on the rule. That is why the cases added here cross two declared sets and sweep every width to 200 rather than name viewports. The implementation agent measured the arithmetic, ran each added case against the superseded renderer, wrote this text and decided none of the substance. `VREC-MOK-005` is not edited: it is `ready`, not `verified`, so no verified record asserts the rewritten row in its former wording. |

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

The declared **footer seed set** is `0`, `1`, one seed of every decimal magnitude the accepted range admits from `9`
through `9_999_999_999_999_999_999`, and `u64::MAX` — twenty-two values. The declared **footer tick-limit set** is the
default `100` and `u64::MAX`. Both are fixed here for `SPEC-MOK-003` rule 8, whose order of loss is decided by the
digit counts of the entropy seed and the configured tick limit together and by nothing else about their values. They
are separate from the verification seed set above, which exists so that observed and unobserved runs are compared on
runs whose unobserved behavior is already recorded evidence, and whose five members have three digits at most.

**Crossing the two sets is the coverage; sweeping either alone is not.** The clipping defect this contract reported
PASS against was reachable only where both were wide. At the floor a twenty-digit seed beside the default limit of
`100` came to 34 characters exactly and nothing was cut, so a sweep of every seed in the range at the default limit
would have found nothing. Beside a twenty-digit limit the same row came to 51 characters and the pane kept 34.

This contract verifies that the observer reports the simulation faithfully and changes nothing about it. It does not
verify any simulation rule. Every world rule remains verified by `VER-MOK-001` and `VER-MOK-002`, and a failure in
this contract is a defect in the observer or in the boundary, never evidence about the world.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-019` | automated-test | Whole world at the reference viewport | At `160 × 48` the canvas interior is 67 × 36 cells; every living Mokiterion and every standing resource is represented; no entity is omitted |
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
| `REQ-MOK-020` | automated-test | Twelve entries without scrolling at the reference viewport | At `160 × 48` all twelve three-line entries are present in the roster pane; none is hidden |
| `REQ-MOK-020` | automated-test | Roster order is acting order | Entries appear in ascending identifier order |
| `REQ-MOK-020` | automated-test | Zero renders as `0` with an empty bar | A zero attribute is distinguishable from an absent one, which renders `—` |
| `REQ-MOK-020` | automated-test | The fourth gauge position carries `fear` | The fourth position of the bar row presents `fear` in the same form as the other three — a label, a proportional bar and a numeric value — and a zero value renders as `0` with an empty bar rather than as an absence. Unlike the other three it carries no survival band: its cells' colour does not vary with its value at any value |
| `REQ-MOK-020` | automated-test | Collapse below 47 columns | Below 47 columns each entry is one line with the four numeric values and no bars |
| `REQ-MOK-020` | automated-test | Death is corroborated | On the tick a death is applied the entry disappears, the living count decreases, and the death total increases |
| `REQ-MOK-020` | automated-test | Applied action is the engine's | The action shown equals the snapshot's `applied_action` for the most recently completed tick |
| `REQ-MOK-021` | automated-test | Proposal, verdict and applied action are presented | For a selected Mokiterion the inspector shows the proposed action, the outcome word, and the applied action |
| `REQ-MOK-021` | automated-test | A rejection presents the engine's ground | A rejected proposal shows the engine's stated ground, taken from the snapshot and not re-derived |
| `REQ-MOK-021` | automated-test | Rejection is not a fault | No error or warning styling, wording or diagnostic accompanies a rejection |
| `REQ-MOK-021` | automated-test | Proposal and outcome share a tick | Advancing between reads never yields a proposal from one tick beside an outcome from another |
| `REQ-MOK-021` | automated-test | Verdict is never re-derived in the observer | Given a snapshot whose outcome contradicts what a validation rule would produce, the observer presents the snapshot's outcome |
| `REQ-MOK-021` | automated-test | Nothing selected does not default | With no selection the inspector says so and names no Mokiterion |
| `REQ-MOK-021` | automated-test | Before tick 1 | The inspector states that no proposal has been made |
| `REQ-MOK-021` | automated-test | Selected Mokiterion dies | Selection is retained; death, tick of death and all four final attributes are presented — health, satiety, energy and fear, paired across two lines as `SPEC-MOK-003` rule 10.6 as amended fixes them, with no value clipped off the pane at any viewport presenting the inspector; the next selection control moves to the nearest living entry |
| `REQ-MOK-021` | automated-test | A dead subject's final fear is the engine's own | The presented value equals the engine's last reported `survival_changed` fear for that subject, read from the retained event stream and not re-derived, and it is present at the frame on a subject selected while living and held through its death |
| `REQ-MOK-021` | automated-test | An unreported final attribute is absent | Where the observer holds a death for a subject it received no survival record for, satiety, energy and fear are each absent rather than zero-filled, and the pair carrying neither of its values emits no line at all. Asserted at the derived state and at the rendered pane, from inside the crate because the state is reachable only through a private call; the emits-no-line clause is disclosed as unmeasurable at the frame in the residual-uncertainty bullet on this case |
| `REQ-MOK-021` | automated-test | Absent attributes are absent | No field for traits, age, kills, combats, remembered locations, latency or per-agent entropy appears. Fear is on this list for a living subject only, and the name is presented rather than absent |
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
| `REQ-MOK-024` | automated-test | Log height at every declared viewport | The log occupies 6 rows wherever it is present, which is wherever `H ≥ 38`, and no viewport presents it at any other height |
| `REQ-MOK-024` | automated-test | Layout monotonicity over the plane | For every `34 ≤ W ≤ 200` and `22 ≤ H ≤ 60`, no pane present at `W × H` is absent at `W+1 × H` or at `W × H+1`; enlarging the viewport never removes a pane |
| `REQ-MOK-024` | automated-test | Canvas interior at every declared viewport | Interiors are 67 × 36, 67 × 32, 67 × 28, 47 × 32, 47 × 31, 71 × 36, 71 × 24, 51 × 24, and 32 × 16 respectively |
| `REQ-MOK-024` | automated-test | Whole-world claim per viewport | Whole world at `160 × 48`, `160 × 44` and `120 × 48`; a region at `160 × 40`, `140 × 44`, `140 × 43`, `120 × 30`, `100 × 30` and `34 × 22`, each annotated |
| `REQ-MOK-024` | automated-test | Header and footer are never excluded | Both are present at every viewport above the floor, including the floor itself |
| `REQ-MOK-024` | automated-test | No footer value is presented cut | At every declared viewport, for every declared footer seed crossed with both members of the declared footer tick-limit set, the row fits its pane and every maximal digit run in it equals a value the run carries |
| `REQ-MOK-024` | automated-test | The footer row fits its pane at every width | For every width `0 ≤ w ≤ 200` the row is no wider than `w`, the one exception being the width at which not even the entropy seed alone fits, which no viewport above the floor reaches |
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
| `REQ-MOK-027` | automated-test | Footer provenance fields where the width holds them | Seed, tick limit, density as supplied, active source, current tick and retained count are all present at every declared viewport for the declared verification seeds; the entropy seed is present at every declared viewport for every declared footer seed and tick limit |
| `REQ-MOK-027` | automated-test | Fields leave the footer in the specified order | The fields absent from the row at any width are a prefix of `SPEC-MOK-003` rule 8 clause 4's order of loss, and every field after that prefix is present |
| `REQ-MOK-027` | automated-test | No field is shed while the width holds it | A field is lost only where no labelling of the fuller row fits, so the number of fields absent at a width is the least that width admits |
| `REQ-MOK-027` | automated-test | The entropy seed survives the floor | At `34 × 22`, against the widest tick limit the start-up contract accepts, the seed is present whole for every declared footer seed and at both ends of the `u64` range |
| `REQ-MOK-027` | automated-test | The retained count and its truncation marker are one field | No row states the count without its marker where the buffer has been truncated, and no row carries a marker where it has not |
| `REQ-MOK-027` | automated-test | A candidate commit costs no field rule 8 requires | At every width, a build stamped with a commit of any length loses exactly the fields the same run loses unstamped; where the width holds it, the commit is present and labelled |
| `REQ-MOK-027` | automated-test | Labelling changes no value | Every labelling presents the seed, tick limit, current tick, retained count and density unchanged; none abbreviates, rounds or re-bases a value |
| `REQ-MOK-027` | automated-test | Defaulted and explicit values present identically | A defaulted density and the same density supplied explicitly produce the same footer text |
| `REQ-MOK-027` | automated-test | Commit field is compile-time or absent | Present when supplied to the build as a non-empty value; absent when the variable is unset **and** absent when it is set to the empty string, which the compile-time read reports as a supplied value; no repository read and no version-control invocation occurs |
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
   it is available as an overlay; the log keeps its six rows; the canvas becomes 71 × 36 and still presents the whole
   world; selection, filter and zoom are unchanged; the run does not pause.
5. The operator reduces the terminal from `160 × 48` to `160 × 40`, crossing no pane threshold. The roster, the
   inspector and the log are all still present; the log holds its six rows at both heights; the canvas becomes
   67 × 28 and the view title states world rows 0–111 of 128; selection, filter and zoom are unchanged; the run does
   not pause.
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
  `W = 140` and the log at `H = 38` as trades of area for a pane.
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
- **Provenance survival.** Narrowing a viewport removes fields from the footer in `SPEC-MOK-003` rule 8 clause 4's
  order and never abbreviates, rounds, re-bases or cuts one. A field is present with its whole value or it is absent,
  and the entropy seed is present at every viewport above the floor for every seed and tick limit the start-up
  contract accepts. Every value is compared against the run's own configuration and retained buffer rather than
  against a transcription of them.
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
- Confirm that `UNDERLINED` and `REVERSED` render distinguishably from unstyled cells and from each other on the
  assessor's terminal, including where both fall on one cell. These two modifiers carry the shared-cell mark and the
  roster selection, and they are the whole of what a person must answer here: the automated counterpart,
  `verification::every_distinction_survives_the_loss_of_colour`, reads a projection holding only `(symbol, modifier)`
  per cell and so cannot pass by colour, but it also cannot know whether a terminal renders underline at all. **The
  observer offers no way to disable colour** — no flag and no honoured environment variable — so this is stated as a
  modifier-legibility check rather than as a monochrome run, which cannot be performed on the shipped binary.
- Confirm that a rejection in the inspector reads as an ordinary authority outcome rather than as an error. **This
  state cannot be reached by running the observer**: `verification::no_shipped_decision_source_has_a_proposal_rejected`
  establishes over 400 ticks of both policies that no shipped decision source ever has a proposal rejected. Assess it
  through the `#[cfg(test)]` hook `replace_decisions_for_test`, or from the rendering that
  `verification::the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault` produces. An assessor sent to
  find this state in a live run will not find it, and that is a property of the engine rather than a defect.
- Confirm the fourth roster bar position reads as a computed value rather than as a missing or broken one. Since the
  `SPEC-MOK-003` rule 4 amendment of 2026-08-19 the slot is no longer reserved: it presents a computed `fear`, and
  `fear 0` renders as `0` with an empty bar, which rule 4 requires be distinguishable from an absent value.
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


Retain in addition, under `docs/engineering/simulation/evidence/WO-MOK-008/`:

- the rendered footer row at every declared viewport for the declared footer seed and tick-limit sets, before and
  after the rule 8 amendment of 2026-08-22, each row's character count stated;
- the three states of the compile-time commit variable — unset, set to the empty string, and set to a forty-character
  value — with the recompilation between them shown, a stale build otherwise presenting the previous state;
- the result of running each added case against the superseded renderer, stating for each whether it fails there,
  because a case that passes against both trees measures nothing about this amendment.

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
- **A dead subject's absent final attributes are asserted from inside the crate, and one clause of the case is asserted
  by the code's shape rather than measured.** The case above obliges satiety, energy and fear to be absent rather than
  zero-filled where the observer holds a death for a subject it received no survival record for. No run reaches that
  state: the engine reports survival for a subject before it applies its death, so every death a run produces carries
  all three. The state is therefore constructible only by calling the observer's private `ingest` directly, which is
  what the case does, and `ingest` is private to its module, so the case cannot be a public-tier test. That placement
  is a constraint and not a limitation on what is asserted: from that same tier `render::draw`, `layout::resolve` and
  the existing `select_for_test` hook are all in reach, so **the absence is measured at the rendered pane and not only
  at the derived value** — with no widened visibility and no fifth hook, both of which `ARCH-MOK-002` prohibits by
  name. What remains disclosed is narrower than the whole case: the clause obliging a pair carrying neither of its
  values to **emit no line at all** cannot be measured at any frame, because the death branch returns with that line
  last, so a line the code declined to emit and the pane's own unwritten rows are the same cells. That clause is
  carried by the shape of the code — the second line is pushed only inside a guard that both values being absent
  cannot satisfy — and a regression that emitted it blank would be invisible to every case in this contract. Closing
  it would need a line-level assertion below the frame, which is a different kind of case than this contract declares.

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

- **`REQ-MOK-027` is not satisfiable at the floor, and these cases measure the concession rather than the
  requirement.** The requirement states that provenance is never the information sacrificed to degradation.
  `SPEC-MOK-003` rule 8 clause 6 as amended on 2026-08-22 concedes that at `34 × 22` a twenty-digit entropy seed
  beside a twenty-digit tick limit is 43 characters of 34, so the entropy seed alone is what the floor guarantees. The
  cases above verify conformance to the amended rule, which is the specification's resolution of that arithmetic. They
  do not establish that the requirement is met, and they cannot, because at that viewport it is not. The residual is
  disclosed in the same terms in `SPEC-MOK-003`'s amendment row of that date and is the **product owner's** to
  resolve — by raising the floor, by narrowing the accepted seed range, or by amending the requirement. It is
  recorded here so that no PASS from this contract can be read as `REQ-MOK-027` satisfied at every viewport.
- **A case that passes against the defective tree measures nothing, and almost every case in this contract did.** The
  whole suite — 302 cases at `f7b1c45` — passed against the renderer that presented a tick limit of `18446744073`
  where the run's was `18446744073709551615`, and against one that presented a retained count of `13` where the run
  had retained `136`. The count of passing cases was never evidence about rule 8. Each case added under `WO-MOK-008`
  was therefore also run against the superseded renderer, substituted behind the same private signature: **seven of
  the ten fail there** and are regression cases, and three pass because they assert the amended rule's own machinery
  and could not compile against a true revert at all. **Two of the seven were blind when first written and were
  strengthened only because they were measured** — one swept every declared footer seed at the default tick limit,
  where the defect is unreachable, and one asserted that no value in the row is cut, which the superseded renderer
  also satisfied by handing the pane a row wider than the pane and letting the pane keep a prefix. What is disclosed
  is that this check is not part of the gate: a case added to this contract in future is not run against a
  counterfactual unless its author chooses to, and nothing here fails if it is not.
