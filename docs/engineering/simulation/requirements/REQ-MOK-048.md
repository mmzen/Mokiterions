+++
id = "REQ-MOK-048"
type = "requirement"
title = "Advertise the route to the control documentation on screen"
status = "draft"
owners = ["product owner"]
created = "2026-08-20"
updated = "2026-08-20"
statement = "WHEN the terminal observer is drawing above the floor, THE SYSTEM SHALL present on screen, without any operator action, the key that opens the key-binding overlay."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-004"]
+++

# Requirement: Advertise the route to the control documentation on screen

## Rationale

`SPEC-MOK-003` rule 7 binds nineteen keys. `?` opens the overlay that documents them, and that overlay is the only
place they are documented to an operator at all. Nothing on screen names `?`.

So the complete account of the observer's controls sits behind a key an operator has no way to learn from the
observer. They can learn it from `SPEC-MOK-003`, from the README, or by trying punctuation until something happens.
The first two are not on screen and the third is not a design.

The product owner reported this from a live pass on 2026-08-20 and called it "an important point"; it is recorded with
the other two findings in `evidence/WO-MOK-012/adverse-observations.md`. **It is the one finding of the three that no
verification case could ever have caught**, because every automated case in `VER-MOK-005` asserts against a buffer
whose contents it already knows how to look for, and an assertion that a key is discoverable requires knowing that
someone did not know it.

The observer exists to be the instrument later phases are judged with — `INT-MOK-004` names three questions it must
answer. An instrument whose controls are undiscoverable answers them only for the person who wrote it.

## Preconditions and trigger

The observer is drawing. This includes every viewport above the floor of `SPEC-MOK-003` rule 5, in every run state,
with any pane set, with or without a selection, and while an overlay is open.

Below the floor the observer does not enter the terminal and there is nothing to present. While drawing is suspended by
a resize below the floor the obligation is likewise vacant, because rule 5 presents nothing at all in that state.

## Required response

The key `?` is named on screen, together with enough words to say what it opens, in a form the operator did not have to
act to reveal.

The obligation is on presence and permanence, not on wording or position. Two properties are required of the form:

1. **It is not timed.** A hint that appears and disappears is one an operator can miss, and the operator who needs it
   is the one who was not watching when it showed.
2. **It survives narrowing.** Where the viewport is too narrow for the words, the key itself is still presented. The
   obligation degrades to `?` rather than vanishing, because the key is the part that is load-bearing.

## Failure and boundary behavior

- **At the floor** (`W = 34`, `H = 22`) the hint reduces to its shortest conforming form and is still present. A
  34-column header is the tightest case and the requirement holds there.
- **A hint that displaces an obligation is a defect, not a satisfaction.** `SPEC-MOK-003` rule 5's Announcement is an
  obligation and so is rule 8's provenance footer. This requirement must not be met by dropping either. Where a
  viewport cannot carry both this hint and the Announcement in full, the resolution belongs in `SPEC-MOK-003` rule 5
  and `WO-MOK-013` states it; what is forbidden is silently winning the contest.
- **While the key-binding overlay is open** the hint may remain or may be replaced by a close hint. Either conforms;
  the operator has already found the overlay.
- **The hint names the binding rule 7 fixes.** If rule 7 ever rebinds the overlay, the hint names the new key. It is
  not a literal `?` independent of the binding.

## Constraints

- **No new key binding.** `SPEC-MOK-003` rule 7's table is unchanged. This requirement advertises a binding that
  exists; it adds none, removes none and rebinds none.
- **No mutation.** Presenting a hint reads nothing from the engine and writes nothing anywhere. `SPEC-MOK-003` rule 7's
  closing property — that no binding mutates world state — is untouched because no binding changes.
- **No colour-only distinction.** `SPEC-MOK-003` rule 2.5 applies: the hint is words and a key character, so it is
  legible without colour, and any emphasis is reinforcement.
- **Layout stays a pure function of viewport dimensions.** The hint's presence does not depend on tick, run state,
  entropy or wall-clock time, which is what makes it presentable in a header whose content rule 5 constrains.

## Acceptance examples

### Example: normal behavior

**Given** the observer freshly started at the reference viewport size

**When** the first frame is drawn and the operator has pressed nothing

**Then** the screen names `?` and what it opens

### Example: failure behavior

**Given** the observer at `ff3a155`

**When** the first frame is drawn and the operator has pressed nothing

**Then** no rendered cell anywhere on screen contains the character `?` — which is the state measured on 2026-08-20 and
the state this requirement rejects

## Open decisions

None. The product owner settled the form on 2026-08-20, recorded as decision 14 in
`evidence/WO-MOK-012/closing-review.md`: a permanent header segment beside the run state, degrading to `?` alone at the
floor, chosen over a timed banner and over a footer entry.

That choice constrains `SPEC-MOK-003` rule 5, whose header content is a closed list — the *Observability* section
admits draw failures, input failures, export outcomes, panes available only as overlays, and hidden roster entries, and
nothing else. The amendment adding a sixth admitted item is a technical-owner act and is stated in `WO-MOK-013`; it is
not a product decision and does not belong to this requirement.
