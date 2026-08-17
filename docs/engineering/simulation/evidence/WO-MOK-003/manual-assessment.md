# Manual assessment record — WO-MOK-003

`VER-MOK-003` names seven manual assessments and requires this record to carry them "including the
legibility and colour-independence assessments and their author".

**Every one of the seven is OUTSTANDING. None has been performed. This record has no author for any
assessment because no person has made one.**

That is a completeness gap in the `WO-MOK-003` evidence, stated here rather than left to be
discovered. `VER-MOK-003` itself says of the first assessment that a pass on every automated case
with a negative answer here "is an adverse observation requiring product review", which only means
anything if the assessment is actually made. The verification decision on `WO-MOK-003` should not
be taken as covering these seven.

## Why they were not performed

The three that require a live terminal cannot be performed from this environment, and the reason is
specific rather than a matter of effort.

The observer reads keys through `crossterm`, which on Windows reads the console input buffer rather
than standard input. A keypress written to the process's stdin from this shell therefore never
reaches it: measured in `terminal-restoration.txt`, a piped `q` produced exit code 124 (the timeout's
own code), 7017 bytes of frame output and an empty stderr — the process drew frames and never saw the
key. `winpty` is installed but cannot supply a pseudo-terminal here either: `winpty -Xallow-non-tty`
aborts with `ASSERT_CONDITION("wp != nullptr && cols > 0 && rows > 0")` because it reads its size
from a tty it does not have, and `winpty --help` offers no size flag. There is no route from this
shell to an interactive observer session.

The four that do not name a live terminal — the rejection reading, the reserved-slot reading, the
overview-granularity judgment, and by extension any reading of a retained dump — were not performed
because each is a judgment about what a person perceives, and I am not the person `VER-MOK-003` is
asking. Reading a buffer dump and reporting that it "reads as" something would be the automated
assertion restated in prose, which is precisely the gap the contract's residual-uncertainty section
says cannot be automated away. What is below is the material an assessor needs and the procedure to
follow, not a substitute verdict.

## The seven assessments

### 1. Two-hundred-tick instrument assessment

**Status: OUTSTANDING. Author: —**

`VER-MOK-003` asks for at least 200 ticks on one declared seed, confirming the instrument answers
the three questions `INT-MOK-003` names: where the population is, why a selected Mokiterion did what
it did, and which requirement authorizes a highlighted event.

Procedure: `cargo run -p mokiterions-tui -- --seed 42 --policy reference --speed 8` in an interactive
terminal at least 160 × 48, and let it reach tick 200 (25 seconds at speed 8). Use `Tab` to select,
`z` to zoom, `f` to follow, `a` for the authority overlay.

One thing an assessor should know before starting, because it will look like a defect and is not:
under `--policy baseline` seed 42 reaches extinction on tick 142 (measured: `summary
reason=extinction ticks=142 survivors=0 deaths=12`), so the run ends before 200 ticks and the
assessment cannot be completed on that configuration. Use `--policy reference`. Under the reference
policy 11 of the 12 die by tick 10,000, so a long session will show a shrinking population; the
death and living counts stay consistent, which `resilience.txt` records.

### 2. Reference-viewport legibility on a real terminal

**Status: OUTSTANDING. Author: —**

Asks whether, at 160 × 48, resource dots and Mokiterion letters are distinguishable and the
territory boundary reads as a boundary.

Material: `frames.txt` section "160 x 48 — tier A full" carries the buffer dump. The dump is the
character content, which is what an automated assertion can check; the question is about the
rendered glyphs, so the dump cannot answer it. The specific risk worth attention is that the overview
plots Mokiterions and resources into the same braille canvas at 2 × 4 world cells per character
cell, so a Mokiterion letter and nearby resource dots can occupy adjacent cells at small visual
separation, and braille coverage differs between fonts.

### 3. The same with colour disabled

**Status: OUTSTANDING. Author: —**

Asks whether any distinction is lost on a monochrome terminal.

The automated counterpart exists and is stronger than usual for this kind of case:
`verification::every_distinction_survives_the_loss_of_colour` reads a projection of the frame that
holds only `(symbol, modifier)` per cell, with colour discarded, and requires the territory to be
readable by letter (`M01  A`, `M03  B`), depletion by the words `permanently depleted`, the shared
cell by exactly one underlined cell, the selection by reversed cells in the roster, the territory
rule by an unbroken leading run of at least 64 non-space cells on exactly one canvas row, and all
three resource classes by their distinct glyphs in detail zoom. It cannot pass by colour because
colour is not in the data it reads.

What remains for a person: whether `UNDERLINED` and `REVERSED` actually render distinguishably on
the assessor's terminal. Some emulators drop underline or render reverse video at low contrast, and
the shared-cell mark and the roster selection are the two distinctions that depend on them.

### 4. A rejection reads as an authority outcome, not an error

**Status: OUTSTANDING. Author: —**

Material: `frames.txt` section "The inspector pane, subject M01".

An assessor must know that this state cannot be reached by running the observer.
`verification::no_shipped_decision_source_has_a_proposal_rejected` establishes over 400 ticks of both
policies that neither shipped decision source ever has a proposal rejected, so there is no seed,
speed or interaction that will display a rejection. The presentation is exercised only through the
`#[cfg(test)]` hook `replace_decisions_for_test`. To assess it, read the rejection rendering that
`verification::the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault` produces, or
add a temporary decision source that proposes an illegal action. This is a finding about
`VER-MOK-003`'s acceptance scenario 2 rather than about the observer, and it is in the completion
summary.

### 5. The reserved fourth roster bar reads as empty space

**Status: OUTSTANDING. Author: —**

Material: `frames.txt` sections "The roster pane at the reference viewport" and "The roster overlay,
where the reserved slot has room".

The two sections are both needed, because the slot's width is `min(20, (interior_width − 27) / 3)`
and at the 47-column roster that evaluates to zero: the reserved slot is not empty space there, it
is absent. So the assessment has two parts — whether it reads as empty space where it has width, and
whether its absence where it has none reads as a truncated pane. `render::tests::the_bar_row_reproduces_the_specified_form`
asserts that nothing follows the third value: no label, no dash, no zero.

### 6. Whether the overview's cell granularity is materially misleading

**Status: OUTSTANDING. Author: —**

An overview Mokiterion glyph locates its subject to within a 2 × 4 block of world cells by
construction; `spatial::tests::a_character_cell_covers_two_by_four_world_cells_in_overview_and_one_in_detail`
pins the mapping. `VER-MOK-003` says in advance that if an operator misreads a position because of
this, it is an adverse observation about rule 2 requiring a specification decision, not a defect to
patch. So the outcome of this assessment is an artifact decision either way, and it is the owner's.

The mitigation already present is `z`, which switches to one character cell per world cell, and the
region annotation, which states the world range the canvas presents.

### 7. The terminal is usable after a deliberate panic

**Status: OUTSTANDING for the live-terminal part. Author: —**

The automated part was performed and is retained in `terminal-restoration.txt`: raw mode measured as
off before init, ON after, and off again after a panic was caught, with the alternate-screen
enter/leave counts and the `ratatui` source citations for `try_init`, `try_restore` and
`set_panic_hook`. That is a measurement of the console's own state, not of a buffer.

`VER-MOK-003` asks for this "by inspection of the live terminal rather than only by an automated
assertion", and the words "rather than only" make the automated result insufficient by construction.
Procedure: run the observer, trigger a panic, and confirm the shell still echoes typed characters
and that the prompt is on the normal screen rather than the alternate one.

## For the owner

Six of the seven are unperformed with no partial result. The seventh has an automated result the
contract explicitly declines to accept alone. If `WO-MOK-003` is to be verified with these
outstanding, the verification record should say so, because the residual-uncertainty section of
`VER-MOK-003` identifies exactly this gap — "a claim about a screen that only a human has seen is
the weakest evidence this repository accepts", and its inverse, that a buffer nobody has seen
rendered is not evidence of legibility at all.
