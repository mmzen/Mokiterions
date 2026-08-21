# `WO-MOK-011` — the seven manual assessments of `VER-MOK-011`

| Field | Value |
|---|---|
| Work order | `WO-MOK-011` (Phase 2.5, naming) |
| Verification contract | `VER-MOK-011` |
| Baseline | `524a6758d74b5240079959e9827ea40a7af22a30` |
| Date of this record | 2026-08-19 |
| Written by | the implementation agent, which records measurements and states which role each judgement belongs to. **It makes none of them.** |

`VER-MOK-011` states: "Each of the following is an explicit judgement recorded by the accountable
role. An unrecorded assessment is an outstanding assessment, and this contract is not satisfied
while any remains outstanding."

Six of the seven are discharged by the repository owner's approval of 2026-08-19, because the
artifact each one governs states the substance of the judgement and the owner approved that text in
the same act as `WO-MOK-011`. **One is outstanding**: assessment 5 concerns the projection, which is
evidence produced after the approval and which no approved artifact describes.

Where a judgement has a measurable component, the measurement is given here and its source file is
named, so that the accountable role is deciding against numbers rather than against an assurance.

---

## 1. The names themselves — product owner

**Recorded 2026-08-19 by the repository owner acting as product owner**, by approving the
`SPEC-MOK-001` amendment that fixes the twelve names, in the act that also approved `INT-MOK-008`,
`CAP-MOK-008`, `REQ-MOK-040`, `REQ-MOK-041`, `VER-MOK-011` and `WO-MOK-011`.

The twelve are `Zug`, `Krul`, `Quib`, `Sput`, `Trok`, `Womp`, `Hozz`, `Nurb`, `Vonk`, `Gorm`, `Xob`,
`Drix`. The register was the owner's own instruction: names that are "short, sound original and
coming from another place, and maybe a little bit funny", and hard-coding a fixed list was stated as
acceptable in the same instruction.

What is **not** claimed here: no language-by-language screening was performed or recorded, and this
file does not assert one. The assessment is the owner's judgement that each name is inoffensive in
the languages the owner can assess and is not a real given name borrowed from a living person, and
it rests on the owner having chosen the register and approved the table. `VER-MOK-011` records under
*Residual uncertainty* that nothing can falsify a name, which is why this stays a judgement.

## 2. The assignment — product owner

**Recorded 2026-08-19 by the repository owner acting as product owner**, in the same act. The
approved `SPEC-MOK-001` *Name* subsection fixes the pairing `M01→Zug` through `M12→Drix` as a table,
and the amendment row states that the assignment is the product owner's decision.

The engine's behaviour matches that table on every declared seed, at every density and under every
source — `analysis/names-per-seed.txt` shows the five declared seeds reporting the identical twelve
pairings in the identical order, and the nine policy-by-density combinations reporting the same. An
implementation that reordered the table would fail
`mokiterions-core/src/simulation.rs::the_names_are_the_specified_twelve`, which compares against a
table written from the specification rather than read from the engine.

## 3. Legibility of the named roster entry — technical owner

**Recorded 2026-08-19 by the repository owner acting as technical owner**, by approving the
`SPEC-MOK-003` rule 4 amendment, which states the measurement it was approved against: "line one's
fixed fields total 28 columns of a 45-column interior, so nothing truncates."

The measured column usage at the reference viewport, from `observer/frames.txt`:

| Measurement | Value |
|---|---|
| Roster pane at `160x48` | `x=0 y=3 w=47 h=34`, so a 45-column interior |
| Fixed fields on line one | 28 columns: name 6, identifier 5, territory 3, position 14 |
| Longest line one observed in the capture | 38 columns of 45, e.g. `Xob   M11  B  123:115       move:north` |
| Widest applied action in the capture | `move:north` at 10 columns |
| Name field | 6 columns, left-aligned; the longest name is 5, so no name truncates |
| Bar row on line two | unchanged at every viewport — 96 rows compared, `observer/bar-rows.diff` is empty |

The owner's alternative under this assessment was to require the rule 4 layout to change instead.
The rule was amended and approved as it stands, which is the decision.

## 4. Retaining the identifier beside the name — product owner

**Recorded 2026-08-19 by the repository owner acting as product owner**, by approving
`INT-MOK-008`, `REQ-MOK-041` and the `SPEC-MOK-003` rule 4 amendment, each of which states the
ground: the identifier is the join key into the log, the export and every retained stream, and into
every citation in the corpus.

The consequence is measured rather than assumed: the identifier is still present on every roster
entry at every viewport that shows a roster, and in the inspector heading for a living and for a
dead subject — `observer/frames.txt`, where the identifier moves from `x=1` to `x=7` on all twelve
rows and no row loses it.

## 5. The projection — assurance owner — **OUTSTANDING**

**Not recorded. This assessment requires the repository owner acting as assurance owner, and this
contract is not satisfied until it is made.**

What is to be assessed: `baseline/projection.py`, the transformation oracle 1 applies to both
captures before comparing them. The owner is to confirm that it deletes only the field this change
adds, and that it could not mask a change to a position, an identifier, an event kind, an ordering,
an attribute value or a line's presence.

Why it could not be discharged by the 2026-08-19 approval: the projection is evidence written during
implementation, after the approval, and no approved artifact describes it. The implementation agent
cannot assess its own instrument.

What is already measured, and what the owner would be adding to:

- the projection is 
  a deletion of one leading `name:<value>,` from `agent_initialized` details and nothing else;
- applying it to the 90 **pre-change** streams changes not one byte — the no-op check, so the
  projection cannot be deleting anything that existed before this work order;
- with it applied to both sides, all 90 cells are byte-identical and all 90 exit codes are equal
  (`additivity.txt`, `RESULT: PASS`);
- the byte reduction per cell is exactly 118 — 46 characters of name plus twelve `name:` keys and
  their commas — which accounts for the whole difference and leaves nothing unexplained.

`VER-MOK-011` states under *Residual uncertainty* that "oracle 1 depends on its projection" and that
neither mitigation is a proof. That is why this assessment exists and why it cannot be closed here.

## 6. The glyph change — technical owner

**Recorded 2026-08-19 by the repository owner acting as technical owner**, by approving the
`SPEC-MOK-003` rule 2 amendment, which states the new alphabet, states the twelve resulting glyphs
`Z K Q S T W H N V G X D`, and states that they rest on `SPEC-MOK-001`'s twelve pairwise-distinct
first characters, which is what rule 2.5 needs.

The collision check the assessment asks for, measured on the buffers rather than reasoned about
(`observer/frames.txt` and `observer/frames-post.txt`):

- across all nine renderable viewports and both zooms, the only letters or digits drawn anywhere in
  the map canvas are `D G H K N Q S T V W X Z` — exactly the twelve initials, and nothing else;
- the resource glyphs are `○`, `◎`, `●` and the territory rule is `─`; none is a letter, so no glyph
  collides with one;
- the territory labels `A` and `B` are drawn in the roster and the inspector, never in the canvas,
  and neither is among the twelve initials in any case;
- the twelve are pairwise distinct at every viewport, so rule 2.5's colour-independent identity
  distinction holds;
- the baseline alphabet was `1`–`9`, `A`, `B`, `C`; the candidate's contains no digit, so a digit in
  the canvas would now be a defect and is asserted against in
  `mokiterions-tui/tests/spatial.rs`.

## 7. The absence of a name reader — technical owner

**Recorded 2026-08-19 by the repository owner acting as technical owner**, by approving
`REQ-MOK-040` and the `SPEC-MOK-001` amendment, which fixes as a load-bearing property that
"nothing in the engine reads a name" and records that a name is not carried on the rule 3
observation, for the reason `fear` is not.

Measured, in `static-checks.txt` item 3: the engine has exactly one name writer —
`simulation.rs:1396`, a table lookup at agent construction — and exactly one reader —
`simulation.rs:1656`, the tick-0 initialization record. No third site reads `agent.name`: it is not
in a decision input, not in a snapshot, not in any other record, and not in any termination or
census path. `DecisionSource::name` is pre-existing and names a policy, not a Mokiterion.

---

## Summary

| # | Assessment | Role | State |
|---|---|---|---|
| 1 | the names themselves | product owner | recorded 2026-08-19 |
| 2 | the assignment | product owner | recorded 2026-08-19 |
| 3 | legibility of the named entry | technical owner | recorded 2026-08-19 |
| 4 | retaining the identifier | product owner | recorded 2026-08-19 |
| 5 | the projection | assurance owner | **OUTSTANDING** |
| 6 | the glyph change | technical owner | recorded 2026-08-19 |
| 7 | the absence of a name reader | technical owner | recorded 2026-08-19 |

`VER-MOK-011` is therefore **not yet satisfied**, on one count and one only, and `VREC-MOK-011`
cannot be written as satisfied until the assurance owner records assessment 5. Nothing in the code
or the evidence is blocked on it; the outstanding item is a judgement, not a measurement.

> **Later fact, added 2026-08-20 without changing anything above.** `VREC-MOK-011` was transitioned to
> `verified` on that date, and **assessment 5 is still OUTSTANDING** — the owner decided on the record
> rather than performing the assessment, so what the `verified` status records is an acceptance of this
> evidence with this row unperformed. Both sentences above therefore still hold: `VER-MOK-011` is not
> satisfied, and the record does not claim it is. See `assurance-decision.md`.
