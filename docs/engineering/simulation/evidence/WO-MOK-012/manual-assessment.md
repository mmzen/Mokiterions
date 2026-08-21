# Manual assessment record — WO-MOK-012, closing VER-MOK-005's seven

`VER-MOK-005` names seven manual assessments and requires the retained evidence to carry them "including the legibility
and colour-independence assessments and their author".

**Six of the seven are performed and authored here by the repository owner. The seventh remains outstanding by the
owner's decision, with its reason recorded.** This supersedes nothing in `evidence/WO-MOK-005/manual-assessment.md`,
which stated correctly at the time that all seven were outstanding and none had an author; that file is left as the
record of what was true then, and the three defects since found in the procedure it recommends are in
`procedure-defects.md` rather than edited into it.

**Author for every assessment below: the repository owner**, acting in the role named. The implementation agent
assembled the material, put each question, and transcribed each answer. It authored no assessment and decided no
outcome. Where the agent's own reading of a capture differed from the owner's framing, it said so rather than
transcribing agreement — see assessment 2 and observation 3 in `adverse-observations.md`.

The live pass was performed by the owner on a real terminal on 2026-08-20. The captures cited below were taken from the
same tree — `master` at `ff3a155`, with `mokiterions-core/` and `mokiterions-tui/` byte-identical to the commit the
owner ran, which `identifier-sweep.md` establishes by diff. A capture is cited only where it quantifies something the
eye reported; no capture is offered as a substitute for the judgement.

## 1. Two-hundred-tick instrument assessment

**Status: SATISFIED. Author: repository owner, as product owner. Date: 2026-08-20.**

`VER-MOK-005` asks for at least 200 ticks on one declared seed, confirming the instrument answers the three questions
`INT-MOK-004` names. All three are answerable:

- **Where the population is** — the overview canvas presents the whole 128 × 128 world at the reference viewport, with
  territory A above territory B and the boundary reading as a boundary. Twelve letters were locatable against the
  roster's twelve entries.
- **Why a selected Mokiterion did what it did** — the inspector states the proposal, the outcome and the applied
  action for the selected subject at the current tick. Captured at `assessment-material/panes-160x48-tick200.txt`:
  `proposed move:north`, `outcome + accepted`, `applied move:north`.
- **Which requirement authorizes a highlighted event** — the authority overlay maps all thirteen event types to
  requirement identifiers, including `decision_source_selected` to `REQ-MOK-008` for baseline, `REQ-MOK-015` for
  reference and `REQ-MOK-033` for individual.

**A pass here is not a pass on discoverability.** The owner knew the key bindings before starting. An operator who does
not is the subject of observation 1 in `adverse-observations.md`, and this assessment does not clear it.

## 2. Reference-viewport legibility on a real terminal

**Status: SATISFIED as to the subject `VER-MOK-005` enumerates. ADVERSE OBSERVATION on the roster gauges at the same
viewport. Author: repository owner, as product owner. Date: 2026-08-20.**

The assessment as written asks whether "the whole-world overview is legible: that resource dots and Mokiterion letters
are distinguishable and that the territory boundary reads as a boundary". **On that subject it is satisfied.** Braille
resource dots and uppercase Mokiterion letters were distinguishable at the reference viewport, and the unbroken
territory rule read as a boundary rather than as a row of content.

**The owner reported a legibility defect at the same viewport that the assessment's particulars do not name**: the
roster's survival gauges are two columns wide. It is recorded against this assessment because this is the assessment
that asks about legibility at the reference viewport, and stating it anywhere else would leave the only legibility
assessment reading as an unqualified pass. The distinction is stated rather than blurred: the overview passed; the
roster did not.

Quantified in `assessment-material/bar-quantization.txt`. At the reference roster's 45-column interior,
`bar_width(45) = min(20, (45 − 35) / 4) = 2`, and `filled = value × 2 / 100`, so the bar has **three renderable states
for 101 values** — `░░` for 0–49, `█░` for 50–99, `██` for 100 alone. The three-column numeric value beside it is
unaffected and remains exact, so no information is lost from the pane; what is lost is the bar's function as a
glanceable indicator, which is the whole reason a bar sits beside the number.

This is observation 2 in `adverse-observations.md`, and decision 13 of `closing-review.md` settles the remedy.

## 3. Distinguishability without colour, as restated 2026-08-20

**Status: SATISFIED. Author: repository owner, as product owner. Date: 2026-08-20.**

The assessment as originally worded asked for a run "with colour disabled or on a monochrome terminal". **No such run
is possible**: `mokiterions-tui` has no `--no-color` flag and honours no `NO_COLOR` variable. `VER-MOK-005` was amended
the same day to state the check that survives — whether `UNDERLINED` and `REVERSED` render distinguishably from
unstyled cells and from each other, including where both fall on one cell. That is the part no test can reach, because
the automated `every_distinction_survives_the_loss_of_colour` reads a projection holding only `(symbol, modifier)` per
cell and cannot know whether a terminal draws underline at all.

Assessed and satisfied: both modifiers render distinguishably, and a cell carrying both is distinguishable from a cell
carrying either.

Reaching the co-occupancy case took a deliberate search, and this is worth recording for a future assessor.
`UNDERLINED` exists in the observer solely to mark a rendered cell holding more than one Mokiterion, and **it does not
occur anywhere in the seed-42 run** the rest of this assessment used. `assessment-material/underline-search.txt`
records the sweep that found it: **seed 0, tick 18**. `assessment-material/reversed-and-underlined-160x48.txt` captures
both modifiers on one cell, produced by selecting `M09` at that tick.

## 4. A rejection reads as an authority outcome, as restated 2026-08-20

**Status: SATISFIED. Author: repository owner, as product owner. Date: 2026-08-20.**

**This state cannot be reached by running the observer.** `verification::no_shipped_decision_source_has_a_proposal_rejected`
establishes over 400 ticks of both policies that no shipped decision source ever has a proposal rejected, which
`VREC-MOK-005` discloses as disclosure 5 and which also makes `VER-MOK-005` acceptance scenario 2 describe an
unreachable state. `VER-MOK-005` was amended the same day to name the `#[cfg(test)]` hook route explicitly so that a
future assessor is not sent after a state that cannot occur.

Assessed from `assessment-material/rejection-160x48.txt`, captured through `replace_decisions_for_test`. The inspector
presents the rejection as an outcome of the authority chain rather than as a fault: the proposed action, the outcome
line, the engine's stated ground for refusal, and the absence of an applied action. Nothing is styled as an error and
no wording suggests malfunction. Satisfied.

## 5. The fourth roster slot, as restated 2026-08-20

**Status: SATISFIED. Author: repository owner, as product owner. Date: 2026-08-20.**

The assessment as originally worded asked whether "the reserved fourth roster bar position reads as empty space".
**The slot is no longer reserved.** Since the `SPEC-MOK-003` rule 4 amendment of 2026-08-19 it presents a computed
`fear`, and rule 4 requires `fear 0` to render as `0` with an empty bar, distinguishable from an absent value.
`VER-MOK-005` was amended the same day to ask the question the slot now poses.

Assessed from `assessment-material/panes-160x48-tick200.txt`. The slot reads as a computed zero rather than as a
missing or broken value: `f ░░   0` carries the label, the empty bar and the explicit numeral, and it sits in a row
whose other three gauges are populated, so the zero reads as measured rather than as absent. Eight of the twelve
entries showed a non-zero `fear` at tick 200 — `f █░  95`, `f ██ 100` — which makes the zero legible as one value in a
range rather than as a slot that never fills.

`fear` is deliberately unbanded, unlike health, satiety and energy, because its direction inverts: a high `fear` is not
a healthy state. The owner confirmed that the absence of banding on this gauge alone does not read as a defect.

## 6. Whether the overview's cell granularity is materially misleading

**Status: NOT MATERIALLY MISLEADING. Author: repository owner, as product owner. Date: 2026-08-20.**

An overview Mokiterion glyph locates its subject to within a 2 × 4 block of world cells by construction.
`VER-MOK-005` states in advance that if an operator misreads a position because of this, it is "an adverse observation
about rule 2 requiring a specification decision, not a defect to patch", so the outcome either way is the owner's
artifact decision.

Assessed against `assessment-material/frame-160x48-tick200.txt` and the live pass, with the roster's exact coordinates
available for comparison. No position was misread. The two mitigations already specified carry it: `z` switches to one
character cell per world cell, and the view title states the world range presented — `x0-127 y0-127 whole world` at
the reference viewport, and an explicit `region` annotation with its range wherever the canvas presents less than the
whole world.

**No amendment to `SPEC-MOK-003` rule 2 is required.** This is recorded as a decision rather than as a non-finding,
because the contract asks for one either way.

## 7. The terminal is usable after a deliberate panic

**Status: OUTSTANDING BY DECISION. Author: none — deliberately. Decided by the repository owner as assurance owner,
2026-08-20.**

This assessment is **not** performed, and it is not closed. The reason is recorded rather than left to be inferred.

`VER-MOK-005` asks for the confirmation "by inspection of the live terminal rather than only by an automated
assertion". The words "rather than only" make the automated result insufficient by construction. The automated result
exists and is retained in `WO-MOK-005`'s `terminal-restoration.txt`: raw mode measured off before init, on after, and
off again after a caught panic, with alternate-screen enter and leave counts and the `ratatui` source citations for
`try_init`, `try_restore` and `set_panic_hook`. That is a measurement of the console's own state rather than of a
buffer, and it is real evidence — but it is the automated assertion the contract declines to accept alone.

**The live inspection cannot be performed by running the observer, because the shipped binary has no
operator-reachable panic path.** There is no key, flag or sequence that induces one. Performing it would require
building a binary that is not the shipped one, and an inspection of a terminal restored by a different program is not
the inspection this contract asks for.

The owner's decision: **leave it outstanding and visible.** An assessment closed against a weaker measurement than its
contract names is worse than one left open, because the record would then assert that a person inspected something no
person had seen. **`VREC-MOK-005` must continue to disclose this assessment as outstanding**, and a release record
covering this chain inherits the disclosure.

## Summary

| # | Assessment | Status | Author |
|---|---|---|---|
| 1 | 200-tick instrument | Satisfied | repository owner, product owner |
| 2 | Reference-viewport legibility | Satisfied on the overview; adverse on the roster gauges | repository owner, product owner |
| 3 | Distinguishability without colour, restated | Satisfied | repository owner, product owner |
| 4 | Rejection as authority outcome, restated | Satisfied | repository owner, product owner |
| 5 | The fourth roster slot, restated | Satisfied | repository owner, product owner |
| 6 | Overview cell granularity | Not materially misleading | repository owner, product owner |
| 7 | Terminal usable after panic | **Outstanding by decision** | none, deliberately |

Six authored. One outstanding with a recorded reason. Three adverse observations arising from the pass, in
`adverse-observations.md`, none of them fixed here and all three directed to a separate chain by decision 12.
