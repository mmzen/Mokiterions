+++
id = "REQ-MOK-041"
type = "requirement"
title = "Present the name wherever the observer identifies a Mokiterion"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "WHERE the terminal observer identifies a Mokiterion, THE SYSTEM SHALL present the name the engine reported for it alongside its identifier, and SHALL derive the map glyph from that name's first character, without presenting any name the engine did not report."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-008"]
+++

# Requirement: Present the name wherever the observer identifies a Mokiterion

## Rationale

`REQ-MOK-040` makes the engine compute and report a name. That alone puts the name in the text stream and in the
observer's log pane, which is worth something but is not the outcome: an operator watching a run reads the roster, the
map and the inspector, and all three would still say `M07`. This requirement is what makes the name the thing the
operator actually reads.

The glyph clause is not an addition this requirement invents. `SPEC-MOK-003` rule 2's glyph table states that
identifiers "carry no names" and that "when agent naming is introduced by a later phase, the glyph becomes the name's
first character and this table is amended". The observer's half of this design was settled a phase ago, in the
specification, and this requirement is the approved need that lets the amendment be made.

The final clause is the one that constrains rather than enables. `WO-MOK-005` bound the observer to present no value the
engine does not compute, "including an inert placeholder", and a name is exactly the kind of value an observer could
plausibly synthesise from an identifier. It must not. Every name it presents is one the engine reported.

## Preconditions and trigger

The trigger is any frame in which the observer identifies a particular Mokiterion. That is the roster, the inspector, the
map, and no other pane: the log presents the engine's own record lines verbatim under `REQ-MOK-022` and needs no change,
because the name is already in the line the engine wrote.

The precondition is that the engine reported the name, which `REQ-MOK-040` guarantees before tick 1 for every Mokiterion
the run creates.

## Required response

- **Roster.** Each entry presents the name and the identifier, both, and the name first. The identifier is retained
  rather than replaced because it is the join key into the log pane, the export and every retained stream, and an
  operator cross-referencing a roster row against a record must not have to translate. The four attribute gauges and
  their arithmetic are untouched.
- **Inspector.** The selected Mokiterion is identified by name and identifier, for a living Mokiterion and for a dead one
  alike, since `SPEC-MOK-003` rule 10.6 retains the selection through death and presents it.
- **Map.** The Mokiterion glyph is the name's first character as an uppercase letter, in both zoom levels, replacing the
  identifier-derived `1`–`9`, `A`, `B`, `C` assignment. `REQ-MOK-040` guarantees the twelve first characters are
  distinct, so no two Mokiterions share a glyph and rule 2.5's colour-independent identity distinction holds as it did.
- **Provenance.** Every presented name is one the engine reported in its own record. The observer holds no name table, no
  fallback name, no derivation from an identifier and no placeholder.
- The presented name is the one belonging to the Mokiterion presented beside it. A frame that showed one Mokiterion's
  name against another's values would be a defect of the same kind as `SPEC-MOK-003` rule 10.3's proposal-outcome
  mismatch.

## Failure and boundary behavior

- **No name is ever absent in practice, and the observer must not be built as though one could be filled in.** The engine
  reports all twelve before tick 1 and the observer ingests those records at construction. If a name were nonetheless
  unavailable for an identifier, the observer presents no name for it — it does not substitute the identifier as a name,
  and it does not present a blank-labelled or placeholder name, because rule 10.7's principle is that an uncomputed value
  is absent. The glyph in that case falls back to a stated character rather than to a guess.
- Below the roster pane's width threshold the roster does not exist at all, under `SPEC-MOK-003` rule 5, so no degraded
  name form is reachable there. Where a narrower entry form is specified, the name is present in it.
- A name never truncates a value beside it. `REQ-MOK-040` bounds the name at five characters and `SPEC-MOK-003` rule 4
  fixes the column budget; the entry's other fields keep the widths they have.
- The map glyph is one character in one cell, as it was. Nothing about cell occupancy, the shared-cell underline, the
  drawing precedence or the overview block mapping changes.

## Constraints

- The observer presents; it never computes. This requirement adds no engine behavior and no engine authority.
- The name reaches the observer through the retained event stream, which `REQ-MOK-022` already obliges the observer to
  retain. No engine public interface item is added for it, and `SPEC-MOK-002` rule 5's enumeration does not grow.
- Every claim about what a frame presents is a claim about characters in an in-memory buffer at stated positions.
  `REQ-MOK-029` and `SPEC-MOK-004` admit no screenshot, recording, terminal or pseudo-terminal as evidence.
- The observer's dependency set is unchanged: one path dependency on the engine, and `ratatui` at its pinned version and
  feature set.
- Layout remains a pure function of viewport width and height under `SPEC-MOK-003` rule 5. A name changes what an entry
  says, never how wide a pane is or which panes exist.
- Non-perturbation holds: presenting a name reads retained records and mutates no simulation state, so the run is the
  same run whether it was watched or not.

## Acceptance examples

### Example: normal behavior

**Given** a run at any seed, at a viewport that presents the roster

**When** a frame is rendered

**Then** every roster entry carries the name the engine reported for that Mokiterion and its identifier, and the name
appears first.

### Example: the glyph is the name's initial

**Given** a run at any seed

**When** the map is rendered in either zoom level

**Then** each drawn Mokiterion's glyph is the uppercase first character of its own name, and no two Mokiterions in the
frame share a glyph.

### Example: the inspector through death

**Given** a selected Mokiterion that dies

**When** the inspector is rendered on the following tick

**Then** the pane still identifies it by name and identifier, and presents the death as `SPEC-MOK-003` rule 10.6 requires.

### Example: no name is invented

**Given** the observer's whole source

**When** it is searched for a name literal or an identifier-to-name derivation

**Then** none exists: every name presented came from an engine record.

### Example: failure behavior

**Given** a viewport below the roster's width threshold

**When** a frame is rendered

**Then** the roster is absent under rule 5 and no partially named entry is drawn, and the map's glyphs are unaffected.

## Open decisions

None. Which panes identify a Mokiterion is settled above; the exact entry layout, the glyph table's amended content and
the inspector's presented-value list are `SPEC-MOK-003`'s to fix.
