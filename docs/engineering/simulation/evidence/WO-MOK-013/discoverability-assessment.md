# The discoverability assessment packet — WO-MOK-013

This file is what the repository owner's decision of 2026-08-20 asked for. Faced with an assessment whose
contract excludes every assessor available, they chose **find an admissible assessor** over amending
`VER-MOK-013` or closing the row by decision, on the ground that it is the only route that actually verifies
`REQ-MOK-048`. That decision is recorded in `manual-assessment.md` as decision 20 of `closing-review.md`, and
this packet is the half of it that falls to the implementation agent: the frame, the question, and the rules
for administering it.

**This file is not the assessment.** `manual-assessment.md` assessment 2 stays **OUTSTANDING** until a person
takes it and its outcome is recorded there with an author, a date, a role and a terminal. Nothing here may be
read as a result.

**It is written to be handed to whoever administers the assessment**, and to be usable without reading
`SPEC-MOK-003`, `VER-MOK-013` or `WO-MOK-013`. That is deliberate: the administrator needs the procedure, and
the assessor must not be shown the specification the assessment tests them against.

## Who may assess

| | |
|---|---|
| **Admissible** | Any person who has not read `SPEC-MOK-003` rule 7. That is the whole requirement. No technical background, no familiarity with this repository, and no terminal of their own is needed. |
| **Not admissible — the repository owner** | They have read rule 7 and ratified six amendments to `SPEC-MOK-003` on 2026-08-20, two of which fix the hint's content and the abbreviation ladder's order of loss. They know the key from the specification, so a pass from them would confirm nothing about the screen. |
| **Not admissible — any agent in this chain** | The implementation agent wrote the hint. The `WO-MOK-012` agent read rule 7 to raise the observation this work order answers. |
| **Not admissible — anyone who has already been shown a frame from this pack** | `VER-MOK-013`'s residual uncertainty: "the assessment is available once per assessor", and "a second run of it on the same person confirms nothing." Once a person has seen the hint they cannot un-know the key. |

**The administrator is not the assessor and need not be admissible.** The owner may administer it. What
disqualifies them is answering the question, not asking it.

## What to show

Show the **contents** of one of the two frames below, on a screen or on paper. Do not name the file, the
directory, or this repository; the path itself tells the assessor what the artifact is for.

| Frame | Size | Header line as drawn | Use |
|---|---|---|---|
| `discoverability-frame.txt` | 48 rows × 160 columns | `HELD  ? keys  x8  overview  sel M01  filter none` | **The contracted case.** The reference viewport, at tick 200 of seed 42 under the reference policy, with all twelve roster entries drawn and every pane present. |
| `discoverability-frame-floor.txt` | 22 rows × 34 columns | `HELD  ?  x8  r W100  L H38  i W140` | **The hard case, for a second assessor only.** The smallest viewport the observer renders. The hint has shrunk to `?` alone and three hidden panes are announced at the last rung of the abbreviation ladder. |

Both are extracts of `frames.txt` in this directory, unmodified — the reference frame is its lines 14–61 and
the floor is its lines 389–410. They were carved out so that a frame can be shown without the surrounding
capture, whose `===` banners name the viewport and would prime the assessor. In each file the character `?`
appears on the header row and **nowhere else**, so the frame does not answer its own question.

**A text capture satisfies this assessment where it would not satisfy assessment 1.** `VER-MOK-013` asks for
"one frame" here and for "a real terminal" there. The property under test is what a person can find in a
drawn frame, which a faithful capture carries; legibility of a proportional gauge is not, which is why
assessment 1 names the terminal and this one does not. Recording the terminal is still contracted for both,
and for a capture the honest entry is the medium it was shown on.

If the frames are shown as monospaced text, keep them monospaced and unwrapped. A 160-column frame reflowed to
80 columns is a different frame, and the header line is the first thing that breaks.

## The question

Ask it as written, and nothing else:

> How would you find out what the keys do?

That is `VER-MOK-013`'s wording. Do not paraphrase it, do not narrow it to the header, and do not ask a
follow-up before they have answered.

## What the outcome is

| The assessor | Outcome | Belongs to |
|---|---|---|
| Names the key from the screen — points at `?` and says they would press it | **SATISFIED** | `REQ-MOK-048` verified as to this assessment |
| Finds `?` but cannot say what it opens | **ADVERSE OBSERVATION on the wording** | `SPEC-MOK-003` rule 5, not the implementation. `VER-MOK-013` states this split. |
| Does not find anything on the screen and would look elsewhere — documentation, a README, trial and error | **ADVERSE** | `REQ-MOK-048`. An artifact decision requiring product review. |

`VER-MOK-013` sets the pass at "they name the key from the screen". Naming it is the whole bar: they need not
know what the overlay contains, what the other keys are, or that `?` is conventional.

## What the administrator must not do

- **Do not say the word "key", "hint", "help", "overlay" or "press" before the question.** The question
  contains "keys" and that is the only mention the assessment admits.
- **Do not point anywhere on the frame**, including at the header row.
- **Do not confirm or correct** while they are looking. Wait for an answer, then stop.
- **Do not explain what the screen is.** If asked, "it is a terminal program showing a simulation" is enough.
  What the program does is not what is being assessed.
- **Do not re-ask, re-show, or try a second frame on the same person.** If the first answer is adverse, that
  is the outcome; a second frame to the same assessor confirms nothing and spends an assessor.
- **Do not take the assessment yourself** if you have read `SPEC-MOK-003` rule 7.

## How to record it

In `manual-assessment.md`, assessment 2, replace the status line with the four fields `VER-MOK-013`'s
*Evidence retention* requires — author, date, role, terminal — and the outcome. The role is **product owner**:
an adverse outcome here is an artifact decision requiring product review, which is a product-owner judgment,
and the accountable role is the owner's even where the observation is another person's. Name the assessor as
the observer and the owner as the role that accepts the outcome, on the pattern
`evidence/WO-MOK-012/manual-assessment.md` uses for its six taken assessments.

Then update, in this order: the summary table and the *Consequence for the chain* paragraph in
`manual-assessment.md`; item 7 of `completion-summary.md`; the *What this review did not decide* section of
`closing-review.md`; the *What it does not close* paragraph of `README.md`; and `docs/ROADMAP.md`'s
`WO-MOK-013` status paragraph. `VREC-MOK-013` can record `REQ-MOK-048` as verified only once this row carries
an author.

**If the outcome is adverse, none of that changes.** An adverse assessment closes the row and does not satisfy
the requirement, and the disclosure it creates is the same shape as the one this whole chain descends from.
