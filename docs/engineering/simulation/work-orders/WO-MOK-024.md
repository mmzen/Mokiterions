+++
id = "WO-MOK-024"
type = "work_order"
title = "Make both help texts understandable: one entry per option, every accepted value explained, nothing left implicit"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-22"
updated = "2026-08-22"

[assurance]
commit_bound_verification = "required"
rationale = "The subject is operator-visible program output on both of its emission paths, and `WO-MOK-004` set the precedent that help content is verified by a commit-bound record rather than by inspection — `VREC-MOK-004` binds the earlier form of this same text and its assertions are the ones a restructure is most likely to break. Three claims here cannot be asserted: that every printed default is still the value the parser applies, which is `REQ-MOK-018`'s own equality and the reason that requirement exists; that no simulation output, no entropy draw and no exit code moved, which is a claim about every run rather than about the diff; and that the four entries the observer shares with the engine are byte-identical across two packages, which is a claim about two constants that no single build compares unless a test does. The work also amends two approved specifications, one approved requirement and one approved verification contract, so the artifacts a later reader would cite as the oracle are themselves part of what changed."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-018"]
specifications = ["SPEC-MOK-001", "SPEC-MOK-003"]
verification = ["VER-MOK-004"]
+++

# Work Order: Make both help texts understandable

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below, and it is the same act as the approval of the five amendments in *Required amendments* — four of which are
prerequisites of the change rather than consequences of it. Transition to `in_progress` records that implementation
has begun. Transition to `implemented` requires the completed change and the retained evidence. Verification and
release require separate commit-bound records.

**The implementation was written before this work order was approved, at the owner's explicit direction, and this
paragraph is the record of that.** On 2026-08-22 the owner was shown both proposed help texts in full, pushed back on
one recommendation in them, directed that the observer's `--events-path` defect be opened as an issue and deferred,
and then directed: *"implement to proposed changed to the help output for both engine and tui"*. The change and its
evidence exist as of that direction. **What that act did not do is approve the five amendments below**, none of which
had been put to the owner when it was given, because none of them had been found yet: the restructure the owner
approved as text turns out to be forbidden by `REQ-MOK-018`'s *Verbosity stays bounded* clause and by
`SPEC-MOK-001`'s *Help output* ordering, and no owner can be taken to have amended an artifact they were not shown.
The five were therefore drafted **OUTSTANDING**, this status was held at `draft`, and each amendment was put to the
role it needed. The alternative — moving this work order to `implemented` on the strength of the direction and
letting the amendments follow — was rejected: it would make the governing artifacts trail the code they are supposed
to authorize, which is the arrangement this repository's whole chain exists to prevent.

### Transition to approved and to implemented, 2026-08-22

**All five amendments were ratified on 2026-08-22 by the repository owner**, each put as its own question with its
alternatives, and each recorded in its own artifact's amendment record naming the acting role. *Decision record*
below is the account of those five acts and of the eleven alternatives declined. With them taken, this work order
moved `draft` → `approved` → `implemented` in a single act of the engineering owner, on the completed change and the
evidence retained under `evidence/WO-MOK-024/`. The two transitions are recorded as one because nothing happened
between them: implementation was already complete and its evidence already captured when approval was given, which
is the sequence the paragraph above discloses rather than conceals.

`harnessctl preflight --work-order WO-MOK-024 --phase start` raised `W005` — *status 'draft' is not eligible for
start* — at every capture taken while this document was `draft`. That diagnostic was retained in
`evidence/WO-MOK-024/gates.md` rather than worked around, because it was correct: it is the harness saying a `draft`
work order authorizes no implementation, which was exactly the state.

**It does not clear on this transition, and saying so would be wrong.** The `start` phase admits `approved` and
`in_progress`, so at `implemented` it raises `W005` again for the opposite reason — the status is past start, not short
of it. **The applicable phase at `implemented` is `review`, and it PASSes with no diagnostic.** Both captures are
retained beside the original failing one, so a later reader sees the whole sequence rather than a single green
screenshot: `draft` fails `start` because implementation was unauthorized, `implemented` fails `start` because
implementation is finished, and `implemented` passes `review`, which is the gate this status is actually measured by.

Commit-bound verification is classified `required` above. `VREC-MOK-022` is prepared against the commit that carries
this transition and is `ready`, not `verified`: preparing a record is the implementation agent's act and accepting one
is the assurance owner's, and this transition did not include that acceptance. Release requires a separate record
again.

### Renumbered from `WO-MOK-020`, 2026-08-22

**This work order was drafted, approved, implemented and evidenced as `WO-MOK-020`, and renumbered to `WO-MOK-024`
before anything was pushed.** Its record was renumbered `VREC-MOK-020` → `VREC-MOK-022` in the same act. Both original
identifiers were already in use, which a fetch of the remote found and a local checkout cannot: `VREC-MOK-020` is a
`verified` record on `origin/master` binding `WO-MOK-017`, and `WO-MOK-020` is held by open draft PR #43 on
`origin/feature/observer-mokiterion-profile`. The renumber is act 11 of *Decision record*, taken by the engineering
owner, and the alternative that was recommended and declined — that PR #43 move instead — is recorded there.

`WO-MOK-020` survives in this work order's evidence in exactly five places, all of them inside fenced `harnessctl`
transcripts in `evidence/WO-MOK-024/gates.md`, plus the whole of the original `preflight-implemented.txt` capture
retained beneath its replacement. Those are verbatim records of commands run against a file that was named
`work-orders/WO-MOK-020.md` at the time, and rewriting them would make them fabrications. Every authored mention was
renumbered; no transcript was.

## Objective

Make `--help` answer, for both binaries, every question an operator can have about an option without sending them to
a specification, to the source, or to a paragraph somewhere below the block they are reading.

The owner's statement of the objective was: *"make the help menu understandable. Currently is it not clear. The point
is to have for each cli arguments: a simple explanation of it, if there are additional arguments (ex: policy ->
individual, social, ..) they must be explained in a simple and short way. There should be no implicit."*

Three things in the previous texts were implicit in exactly that sense.

1. **`--policy`'s four values were named and not explained.** The placeholder said `baseline|reference|individual|
   social`; what each one makes a Mokiterion do was four sentences of prose below the options block, written in the
   specification's vocabulary — "seeks and consumes perceived food so that world viability can be measured", "in
   proportion to its own waste tolerance, which is derived from the seed and its identifier". An operator choosing a
   value read four bare names.
2. **What `--density` is a percentage *of*, and what it does, was likewise below the block**, in a paragraph whose
   closing sentence — "Only the densities declared in the requirements carry a population viability floor" — cannot
   be read at all without the requirements it refers to.
3. **What the program writes, and what its exit codes mean, was stated nowhere.** Both are fixed by
   `SPEC-MOK-001`'s *Outputs* section, and neither reached the operator.

And two things in the observer's text were not merely implicit but silent: it accepts `--trace-actions`, which has
nothing to switch on, and it accepts `--events-path` and then writes nothing at all.

## In scope

### 1. The engine's usage text — `mokiterions-core/src/cli.rs`

`cli::USAGE` is rewritten to the structure `SPEC-MOK-001`'s amended *Help output* section fixes: the synopsis, then a
short orientation paragraph, then one blank-line-separated entry per option, then the order-and-repetition sentence,
then what the program writes, then the exit codes. The trailing explanatory prose is **deleted**, its content having
moved into the entries. The numeric placeholders read `<number>` rather than `<u64>`.

### 2. The observer's usage text — `mokiterions-tui/src/options.rs`

`options::USAGE` is rewritten to the same shape, and converted from one escaped single-line literal to one literal
per output line, for the reason the engine's constant already gives. The four entries for `--seed`, `--ticks`,
`--policy` and `--density` are the engine's own, **byte for byte**. Its own three inputs get entries of the same
form, including `--export`'s effective default path, which was not stated. It states the terminal floor, the exit
codes, and — pending its fix — that `--events-path` is accepted and not acted on.

### 3. The tests that hold the texts to their oracles

`mokiterions-core/tests/cli.rs`: `options_block()` now ends the block at the first line carrying text in column one
instead of at the first blank line, because entries are separated by blank lines. **Not one assertion in that file is
changed, relaxed or removed** — `VREC-MOK-004` binds them and `WO-MOK-010`'s precedent forbids it — and all
eighteen tests pass against the new text unmodified.

`mokiterions-tui/tests/options.rs`: one test is added, `the_shared_entries_are_the_engines_own_words`, which reads
each shared entry out of the observer's constant and requires the engine's constant to contain it byte for byte.

### 4. The six amendments in *Required amendments* below

The sixth was found after the first five were ratified, while executing `VER-MOK-004` to prepare the verification
record: three of that contract's checks were written for an engine-only change against a tree that master has since
moved past. It changes no code and no rendered text, it was ratified as three separate acts later the same day, and
`VER-MOK-004`'s *Second amendment of 2026-08-22* carries it.

## Out of scope

- **Fixing the observer's `--events-path` defect.** The owner deferred it on 2026-08-22 and it is tracked as GitHub
  issue 40. This work order discloses it and does not close it. What closing it needs is recorded in that issue: the
  engine's record writer is `pub(crate)`, and the observer drives ticks one at a time rather than through the
  whole-run path that writes records, so it is an interface change under `SPEC-MOK-002` rule 5 and not a wiring fix.
- **Changing any default, accepted value, constraint or exit code.** `--ticks`' default of `100` in particular stays
  where it is; `REQ-MOK-018`'s *Open decisions* item 1 reserves that to the product owner and this work order does
  not reopen it.
- **Changing any simulated behavior, any event line, any summary line, any action trace, or any record.**
- **Growing either public interface.** Both constants keep their name, type and visibility. Nothing is added.
- **Column alignment, widths and wrapping as governed matters.** `REQ-MOK-018`'s *Constraints* delegate them, and
  they stay delegated.
- **The two misplaced rows in `SPEC-MOK-001`'s amendment record.** The 2026-08-21 waste-condition row sits under the
  document's H1, above *Scope*, and the 2026-08-20 record-stream row sits below the table separated by a blank line,
  so neither renders as a table row. Both are recorded here as observations. Moving an approved row is the technical
  owner's act and is not bundled into an unrelated change.

## Required amendments

Four of the first five are prerequisites: the change as directed cannot be conformant without them, and this is stated
plainly rather than discovered at review. Each is written into its artifact.

**All six were ratified on 2026-08-22**, each put as its own question with its alternatives, and each recorded in its
artifact's own amendment record naming the acting role and the alternatives declined. The `OUTSTANDING` marks the five
carried while this work order was `draft` are gone from those records; *Decision record* below is the account. The
statements of substance in the six subsections that follow are unchanged from the drafted form the owner was shown —
only their ratification state is updated, so that what was approved is legible as what was put.

**The sixth was ratified in a second round**, after the first five and after the code, because executing the contract
is what found it. Its three parts became acts 7, 8 and 9, and the owner's direction to supersede and re-capture the
verification record became act 10. No `OUTSTANDING` mark remains anywhere in this work order or in the four artifacts
it amends.

### 1. `REQ-MOK-018` — the third clause, and the boundary that forbade it (product owner)

The statement gains "and for every option whose accepted values are a closed set, what each of those values means".
*Required response* gains the matching bullet and gains `--events-path`, absent from its enumeration since that
option was added on 2026-08-20. *Failure and boundary behavior*'s **"one entry per option, not a paragraph per
option"** is replaced by a bound on content rather than length, because four value descriptions cannot be one line
and the clause as written forbids the objective outright. The *Rationale*'s "five options" and the acceptance
example's "six options" and two-value `--policy` are corrected.

**This is the amendment the whole change rests on.** Without it the directed text violates an approved requirement.

**Ratified 2026-08-22 by the product owner, as written, together with amendment 3.** Three alternatives declined:
value descriptions beneath the block rather than in the entry; this requirement alone with amendment 3 deferred; and
declining outright, which would have reverted both texts.

### 2. `SPEC-MOK-001` *Inputs* — the numeric placeholder (technical owner)

The synopsis block reads `<number>` where it read `<u64>`, with a paragraph recording that both options still parse
into a `u64`, that `--ticks` still rejects zero, and that the placeholder now names what the operator supplies rather
than how it is stored. `REQ-MOK-018` names the operator at a terminal as the audience; `u64` addresses a different
one.

**Ratified 2026-08-22 by the technical owner, as written**, put as its own question because it is separable from
amendments 1 and 3. Two alternatives declined: restoring `<u64>`, and a third placeholder such as `<count>` or
`<whole number>`.

**One reason given for declining the third placeholder was false, and it is recorded rather than quietly dropped.**
The framing put to the owner said `--ticks <whole number>` "widens a synopsis line already at eighty columns". It does
not. The synopsis line carrying `--ticks` is 55 columns after the change and was 49 before; the widest synopsis line
is 68, and the only 80-column line in the whole text is prose. `<whole number>` would have made that line 62 columns
and cost nothing. The decision itself is unaffected — the owner ratified `<number>`, which is the shorter of the two
and reads no worse — but the width argument carried no weight and should not have been offered as though it did.

### 3. `SPEC-MOK-001` *Help output* — the structure (technical owner)

The four-item ordered content becomes six. The options block gains blank lines between entries and the rule that it
ends at the first line carrying text in column one. Its table gains a fourth column stating what else each entry
says, which is the retired trailing prose relocated. `--density`'s row states two constraints the help text did not
state before, both already in its *Inputs* bullet. The *stated once* paragraph is extended to say that the retired
prose is deleted rather than kept beside the entries. **The three obligations of the section are untouched**:
stated-equals-applied, each-fact-once, and states-no-behavior-stated-nowhere-else.

This amendment and amendment 1 are **one act**. The fourth column is unsatisfiable under the unamended verbosity
clause, and the amended clause is unverifiable without the fourth column.

**Ratified 2026-08-22 by the technical owner, as written, in that one act.** Amendments 2 and 3 share one row in
`SPEC-MOK-001`'s amendment record, which states all three of its provisions; they were put as two questions because
only amendment 3 is bound to amendment 1.

### 4. `SPEC-MOK-003` *Start-up inputs* — what the observer accepts and does not act on (technical owner)

A paragraph records the forwarding the section's identity claim already depends on, and its two unstated
consequences: `--trace-actions` is accepted and has nothing to switch on, and `--events-path` is accepted, validated
and **not acted on**. The second is named as a defect, tracked as issue 40, deferred on 2026-08-22, and disclosed in
the usage text until it is closed. A second paragraph records that the observer carries the four shared entries in
the engine's own words, byte for byte, held by a test.

**Separable from amendments 1 to 3**, and put with them only because one act produced both texts.

**Ratified 2026-08-22 by the technical owner, as written, including the defect characterization.** Two alternatives
declined: recording the behavior without calling `--events-path` a defect, and removing the disclosure from this
specification and from the observer's help text altogether. The second would have returned the observer to accepting
the option silently, which is the state that produced issue 40.

### 5. `VER-MOK-004` — the oracles (assurance owner)

Four matrix rows and two acceptance scenarios are added, for the third clause, the retirement of the prose, the two
added statements, and the cross-target byte-identity. Three stale enumerations are corrected — "six options" for five
and `--help`, `--policy`'s two values for four, and five declared defaults for six. **Nothing in the matrix is
withdrawn or weakened.**

**Cannot be taken before amendment 1**, since three of the four added rows verify clauses that amendment introduces.

**Ratified 2026-08-22 by the assurance owner, as written, after amendment 1 and before this work order's transition.**
Two alternatives declined: ratifying these rows while leaving this work order `draft`, and declining the rows
entirely — the second because the three stale enumerations are wrong today independently of this change, and leaving
them would leave the contract asserting that `--policy` accepts two values.

### 6. `VER-MOK-004` — three checks written for an engine-only change against a tree that has moved (assurance owner)

**Found on 2026-08-22 after the five above were ratified, while executing the contract to prepare `VREC-MOK-022`, and
ratified later the same day as three separate acts.** Stop-and-escalate condition 4 governed while it was outstanding:
it was added here and marked OUTSTANDING rather than presented as approved, and this work order did not claim the three
checks below passed as written until the owner acted. Nothing in the code changes for it; it realigns three checks to a
tree they predate. It is separable from amendments 1 to 5 and could not have been put with them, because it was not
visible until the matrix was run. The statements of substance below are unchanged from the form the owner was shown;
only their ratification state is updated, and each is now carried in `VER-MOK-004`'s *Second amendment of 2026-08-22*
with the measurement that replaces it.

**6a. The *Line width* row is wrong twice.** It reads: *"No line the options block introduces exceeds 80 columns; the
reproduced synopsis block, whose first line is 81 columns before and after, is unchanged."* The first clause holds —
zero lines over eighty on either text. The parenthetical is **false and always was for this text**: the synopsis's
first line is 49 columns before and 55 after, the widest synopsis line is 68, and the two 81-column lines in the
before text are prose lines 28 and 32. And the final clause is **contradicted by ratified amendment 2**, which changed
`<u64>` to `<number>` inside exactly that block. A row cannot both require the synopsis unchanged and be read beside an
approved amendment that changes it.

**6b. The *Test placement* row and the matching *Static and architecture checks* bullet name the wrong file.** Both say
the new tests sit in `tests/cli.rs`. This work order adds **no** test to `tests/cli.rs` — that file is 18 tests before
and after, with no assertion touched — and adds one to `mokiterions-tui/tests/options.rs`. The row's substance is
satisfied (public tier, existing subject, no new file, no test moved between tiers) and only its location clause is
stale, having been written when `REQ-MOK-018` could only be served from one package.

**6c. Two *Performance and resilience* figures describe a tree that no longer exists.** The bullets fix survivor counts
as *"8, 11, 8, 9, 11 on seeds 0, 1, 42, 123, 777"* and ask for the 10,000-tick runs to end *"at the same termination
ticks as recorded before"*. Measured at the base commit `f7b1c45` the counts are **8, 9, 10, 9, 9**, and
reference/0.75 at 10,000 ticks is `tick_limit` with 2 survivors where `WO-MOK-004` recorded extinction at tick 9154.
That is master moving between 2026-08-17 and 2026-08-22 — phase 3 and the resource-composition ceiling — not this
change moving it: base and candidate are byte-identical on all 43 cells and on both 10,000-tick captures. Read against
the fixed numbers, these bullets report a regression that did not happen. The per-seed floor of eight still holds on
every seed. `resilience-and-interface.txt` holds both measurements.

**What was proposed**: replace the *Line width* parenthetical and its synopsis-unchanged clause with a bound measured
rather than asserted; widen the *Test placement* location clause to the public tier of whichever package owns the
subject; and restate the two resilience checks as base-against-candidate identity plus the declared floor, rather than
as identity with figures fixed on 2026-08-17.

**What was decided.** All three were ratified as proposed on 2026-08-22 by the owner acting as assurance owner, each
put as its own question with the alternatives measured before they were put, and each recorded in `VER-MOK-004`'s
amendment record. Acts 7, 8 and 9 of *Decision record* below are the account, and act 10 is the consequence the owner
drew for the record. The declined alternatives were, for 6a, striking the row — which would leave the amended
80-column bound with no verification row in any contract — and leaving the false parenthetical standing with the row
recorded unmet; for 6b, leaving it unmet, and moving the test into the engine's `tests/cli.rs`, which measurement
showed needs either `mokiterions-tui` as a dev-dependency of the engine, inverting the dependency direction
`REQ-MOK-026` and `ADR-MOK-003` fix and breaking the contract's own empty-table row, or an `include_str!` across the
package boundary into another package's source; for 6c, weakening to the declared floor of eight, which would let a
later change shift survivor counts unnoticed, and leaving the fixed numbers, which would have every later reader
report a regression that did not happen.

**One correction the owner's act forced, recorded rather than left implicit.** `VER-MOK-004`'s *Independence* section
disclaimed column widths, and 6a puts a measured width bound in the matrix. That contradiction is older than this work
order — the *Line width* row and the disclaimer have coexisted since 2026-08-17 — but ratifying 6a made it a live
conflict rather than a latent one, so the amendment corrects the sentence to disclaim alignment, wrap-point choice and
sub-eighty-column rendering while owning the bound `REQ-MOK-018` now fixes. This was not separately put; it is the
stated consequence of the act taken, and it is disclosed here so that a reader can see the amendment reached one
sentence beyond the three rows.

## Decision record

Eleven acts of 2026-08-22, each put as its own question with its alternatives, and twenty alternatives declined. One
person holds all three accountable roles in this repository, which makes each approval cheap to obtain and
correspondingly easy to skip; the role each act was taken in is named because nothing here is approved by implication.

The acts fall in three rounds. Acts 1 to 6 were put together, before the code was written against the artifacts and
before the contract was executed. Acts 7 to 10 were put after — they exist because executing `VER-MOK-004` found three
of its own checks misaligned with the tree, which no reading of the artifacts would have surfaced. Act 11 was put last
of all, after the record was accepted, because it exists only because a fetch of the remote found the two identifiers
this work order and its record were already using taken by other work.

| # | Act | Role | Decision | Declined beside it |
|---:|---|---|---|---|
| 1 | `REQ-MOK-018`'s third clause and the replaced verbosity bound | product owner | as written | descriptions beneath the block; requirement alone with amendment 3 deferred; revert both texts |
| 2 | `SPEC-MOK-001` *Inputs* `<number>` | technical owner | as written | restore `<u64>`; a third placeholder |
| 3 | `SPEC-MOK-001` *Help output* structure | technical owner | as written, one act with 1 | — put with 1 |
| 4 | `SPEC-MOK-003` *Start-up inputs* disclosure | technical owner | as written, defect characterization kept | record without calling it a defect; drop the disclosure from spec and help text |
| 5 | `VER-MOK-004`'s four rows, two scenarios, three corrections | assurance owner | as written | ratify but hold this work order at `draft`; decline the rows |
| 6 | This work order `draft` → `approved` → `implemented` | engineering owner | taken with act 5 | hold at `draft` |
| 7 | Amendment 6a, the *Line width* row replaced by a bound measured over both texts | assurance owner | as written | strike the row, leaving the amended 80-column bound unverified anywhere; leave the false parenthetical and record the row unmet |
| 8 | Amendment 6b, *Test placement* widened to the public tier of the owning package | assurance owner | as written | leave it and record it unmet, requiring a check while forbidding its only possible location; move the test into the engine's `tests/cli.rs`, which needs an inverted dependency or a cross-package `include_str!` |
| 9 | Amendment 6c, the two resilience bullets re-anchored to this work order's base commit | assurance owner | as written | weaken to the declared floor of eight; keep the 2026-08-17 figures and have later readers report a regression that did not happen |
| 10 | `VREC-MOK-022` superseded and re-captured against the commit carrying amendment 6 | assurance owner | as written | leave the `ready` record describing the three rows as unmet; edit its prose in place without re-capturing, which would falsify its commit binding |
| 11 | This work order renumbered `WO-MOK-020` → `WO-MOK-024` and its record `VREC-MOK-020` → `VREC-MOK-022` | engineering owner | renumber this work order | have open draft PR #43 renumber instead, which was the recommendation put; `WO-MOK-021` and `WO-MOK-022`, adjacent to live work; keep `WO-MOK-020` and let two artifacts share one identifier |

**Acts 7 to 10 were not available when acts 1 to 6 were put.** Amendment 6 was found by executing the contract to
prepare the record, which happens after implementation by construction. The four are recorded as separate acts, in a
second round, rather than folded into act 5, so that a reader can see which approvals preceded the code and which
followed the measurement.

**Act 10 is why two earlier captures of `VREC-MOK-022` were discarded.** A verification record binds a commit and
cannot be re-pointed at a later one; a record whose prose says three contract rows are unmet cannot be edited to say
they are met without either falsifying that binding or being replaced. Both discarded captures stood at `ready` and
neither was ever accepted. `VREC-MOK-022` names them and says why.

**Act 11 was forced by the remote, not by anything in this branch.** `VREC-MOK-020` is a `verified` record on
`origin/master` that binds `WO-MOK-017`; `WO-MOK-020` is held by open draft PR #43 on
`origin/feature/observer-mokiterion-profile`. Neither was visible when this work began, because the identifier space is
shared across branches, sessions and agents and the highest identifier in a local checkout is not the next free one.
The recommendation put to the owner was that PR #43 renumber, since this side holds an implemented work order, a
twenty-six-file evidence directory and four ratified amendments against that side's three draft files; the owner
declined it and directed that this side move. That is recorded here because the declined alternative was the one
advised. The renumber is why the record was captured a fourth time: every one of its twenty-six `evidence_paths`
changed with the directory rename, and a record's paths cannot be edited after capture any more than its commit can.

**Act 1 is the one the change rests on**, and it is the one the owner's implementation instruction of the same date
did not cover. The instruction was given on the text; this act is on the requirement the text turns out to need. Had
it been declined, acts 2 to 6 would have had nothing to serve and both help texts would have reverted.

**No product decision was reopened.** `--ticks`' default of `100` stayed at `100`; `REQ-MOK-018`'s *Open decisions*
item 1 reserves it to the product owner and it was not put here. No default, accepted value, constraint, exit code,
simulated behavior or record changed in any of the eleven acts. Acts 7 to 9 change no code and no rendered text at
all; they change three sentences of a verification contract. Act 11 changes two identifiers and not one assertion.

**The implementation agent decided none of the eleven.** It found the four conflicts, wrote the amendments, measured the
gates, and put each act with its alternatives. It chose the wording of the descriptions and the layout of both texts,
which `REQ-MOK-018`'s *Constraints* delegate to it, and it wrote the issue-40 disclosure before that sentence had been
put to anyone — which act 4 has now made the owner's.

## Authorized decision envelope

The implementation agent decides, within the approved scope:

- the wording of every description, including the four `--policy` value descriptions, which `REQ-MOK-018`'s
  *Constraints* delegate along with alignment, widths and wrapping;
- the layout: indentation, the blank-line separation, the sub-list form for `--policy`'s values, and the order of
  sentences inside an entry;
- how the tests locate the block and the entries, provided no assertion moves;
- whether the observer's shared entries are held by a test or by a shared constant. **A test was chosen.** A shared
  literal would need a public macro in the engine, since `concat!` takes literals and not constants, and that is
  growth of the engine's public interface under `SPEC-MOK-002` rule 5, requiring an amendment there for a
  presentation concern. The test achieves byte-identity without touching either interface.

It decides none of the following, and none of them moved: any default, any accepted value, any constraint, any exit
code, any simulated behavior, any record, or the classification above.

## Constraints

- Preserve `REQ-MOK-009` through `REQ-MOK-012`. No byte of any event line, summary line or action trace changes, and
  determinism is untouched.
- Preserve `REQ-MOK-016` and `SPEC-MOK-002` rules 5 and 6. Neither public interface grows. Both constants stay
  `pub const USAGE: &str`.
- Add no dependency, and no build script. The declared dependency sets are unchanged.
- Every default the text states is the value the parser applies. This is `REQ-MOK-018`'s equality and it is held by
  a test, not by inspection.
- No fact is stated twice in one text. `SPEC-MOK-001`'s *stated once* paragraph, and `tests/cli.rs`'s assertion that
  no default and no value constraint appears after the block, are the checks.
- No line past column eighty, with no exception. **Corrected 2026-08-22**: this bullet exempted "the synopsis first
  line, which is 81 columns before and after", and measurement contradicts every part of that. The over-eighty lines
  in the engine's *before* text were two prose lines, 28 and 32, both at 81 columns; no synopsis line was over. The
  observer's *before* text had six, from 81 to 84 columns. After the change neither text has any: the engine's widest
  line is 80 columns and the observer's is 79. The bound `REQ-MOK-018`'s amendment states is therefore met outright
  rather than met-except-for-one-line, and the exemption was never needed.
- Test placement follows `SPEC-MOK-002` rule 8 and `SPEC-MOK-004` rule 9: the added observer test reaches only items
  that were already public, and it goes in the public tier beside the tests of the same subject.
- The operator-facing command names stay `Mokiterions` and `mokiterions-tui`.

## Expected change surface

| Path | Change |
|---|---|
| `mokiterions-core/src/cli.rs` | `USAGE` rewritten; doc comment records the amendment and the cross-target identity |
| `mokiterions-tui/src/options.rs` | `USAGE` rewritten and converted to one literal per line; doc comment likewise |
| `mokiterions-core/tests/cli.rs` | `options_block()`'s terminator only; no assertion touched |
| `mokiterions-tui/tests/options.rs` | one test and one helper added |
| `docs/engineering/simulation/requirements/REQ-MOK-018.md` | amendment 1 |
| `docs/engineering/simulation/specifications/SPEC-MOK-001.md` | amendments 2 and 3 |
| `docs/engineering/simulation/specifications/SPEC-MOK-003.md` | amendment 4 |
| `docs/engineering/simulation/verification/VER-MOK-004.md` | amendments 5 and 6, each in its own dated subsection and each with its own row in the amendment record. Amendment 6 also corrects one sentence of *Independence*, which is the consequence of putting a measured width bound in the matrix |
| `docs/engineering/simulation/evidence/WO-MOK-024/` | the evidence below: 26 files, 15 of them under `nonperturbation/` or added with items 7 to 9 |

Nothing under `mokiterions-core/src/simulation.rs` is touched: no engine code is in the diff at all.

**Corrected 2026-08-22.** That sentence previously read "which is the whole of the non-perturbation claim", and it
should not have. It is a claim about a diff; `VER-MOK-004`'s three non-perturbation rows are claims about every run,
and the contract's *Independence* section says review cannot be re-run on a future change. The rows were therefore
executed on both sides — *Required verification* item 7 — and the diff argument is now a corroboration of that
measurement rather than a substitute for it.

## Required verification

1. `cargo fmt --all -- --check` clean.
2. `cargo clippy --all-targets` with no warning.
3. The whole suite green, including all eighteen inherited `tests/cli.rs` tests **unmodified**.
4. The added observer test demonstrated load-bearing: one shared entry edited in one target only, in a scratch
   change, must fail it and name the option. The scratch change is reverted and not committed.
5. Both texts rendered and retained, with a column-width check.
6. `harnessctl doctor` and `validate_engineering_artifacts.py` at the pinned harness version.

**Items 7 to 9, added 2026-08-22** while executing `VER-MOK-004` to prepare `VREC-MOK-022`. The six above are the gates
this work order set itself; the three below are the contract's own rows, which no argument about the diff satisfies.

7. `VER-MOK-004`'s 43-cell matrix and its 16 named cases, executed with `WO-MOK-004`'s `capture.sh` **unmodified** on a
   worktree at the base commit and on the candidate. All 43 cells byte-identical; the fourteen invalid-configuration
   cases identical on exit code, empty standard output and the `configuration error:` line; and the two emission paths
   carrying the same bytes.
8. Acceptance scenario 3's three divergences re-run against the **rewritten** text rather than cited from
   `VREC-MOK-004`, which recorded them against the previous one.
9. The 10,000-tick resilience runs and the 1,000-tick survivor floor, base against candidate, and the public-surface
   check.

## Evidence to record

Under `docs/engineering/simulation/evidence/WO-MOK-024/`:

- `README.md` — what the packet is, the commit binding, and the line-ending note.
- `usage-text.md` — both texts as rendered, the width check, and the three implicit facts from *Objective* traced to
  where each is now stated.
- `gates.md` — the six results above, with the scratch demonstration for item 4.
- `before/` and `after/` — the four rendered texts. The before side is rendered from a `git worktree` at the base
  commit and built there, not reconstructed from a diff.
- `engine-usage.diff.txt`, `observer-usage.diff.txt` — the line-by-line diffs.
- `drift-demonstration.txt` — the cross-target identity test failing on a one-character divergence.
- `preflight-implemented.txt` — both post-ratification `preflight` runs verbatim, with the version banner and the exit
  codes: `--phase start` FAIL at exit 1, `--phase review` PASS at exit 0.
- `defaults-divergences.txt` — item 8, the three divergences against the rewritten text.
- `resilience-and-interface.txt` — item 9, and the two figures the contract fixes that master has moved past.
- `nonperturbation/` — item 7: both manifests, both summary files, both raw `exit-codes.txt` captures, the per-case
  comparison, the emission-path check, both 10,000-tick captures, and the two scripts. Its own `README.md` states what
  is retained and what is deliberately not: the four full 20-tick traces, identical by `diff -r` and by the digests in
  both manifests.

## Stop and escalate conditions

1. An inherited `tests/cli.rs` assertion cannot be satisfied by the new text. Stop. Do not relax it: `VREC-MOK-004`
   binds it. Escalate with the assertion and the text beside each other. *(Did not occur.)*
2. A description cannot be written without stating something no approved artifact states. Stop and escalate rather
   than inventing the fact. *(Did not occur; `--density`'s two further constraints and the four value descriptions
   were all already stated, and where they are stated is recorded in the evidence.)*
3. Any run's output, exit code or entropy sequence differs. Stop. *(Cannot occur: no engine code is in the diff.)*
4. An amendment is found to be needed that is not in *Required amendments*. Add it there, mark it **OUTSTANDING**,
   and name it in the completion report rather than presenting it as approved. *(All six were found this way.
   Amendments 1 to 5 came after the directed implementation and amendment 6 after executing the contract; each was
   drafted OUTSTANDING and carried that mark until the owner acted. See *Lifecycle* and *Decision record*.)*
5. The owner declines any of the six amendments. The text reverts to whatever the surviving set permits, which for
   amendment 1 declined is the previous text entirely; for amendment 6 declined nothing reverts, and `VREC-MOK-022`
   instead records the three contract rows as not satisfied as written. *(Did not occur; all six were ratified as
   written on 2026-08-22, in two rounds. See *Decision record*.)*

## Completion report format

1. What changed, by path.
2. The amendments, each with its acting role and its ratification state. **Widened 2026-08-22 from "the five
   amendments": a sixth was found while executing `VER-MOK-004`, and a format that named a count would have had no
   place to put it.**
3. The verification results, with the scratch demonstration. **Nine items, not the six this section first named**; see
   *Required verification*.
4. What was left undone and why: issue 40, the two misplaced amendment rows, and anything the evidence shows a
   required check does not cover.
5. What remains for the owner.

### Completion report, 2026-08-22

1. **What changed.** **Thirty-five paths** against `f7b1c45`, the shape of them in *Expected change surface* above. Two
   `USAGE` constants rewritten, one test helper's terminator, one test added, four governing artifacts amended, this
   work order, and **twenty-six** evidence files. The count is `git diff --name-only f7b1c45` at the commit
   `VREC-MOK-022` binds, not the table's row count, which groups the evidence directory into one line, and it excludes
   `VREC-MOK-022.md` itself, which is written against that commit and committed after it. No engine source is in the
   diff. **The evidence count was ten at the first capture and eleven after the ratification**; the other fifteen files
   are *Required verification* items 7 to 9, the contract's own rows, executed after the ratification. That growth is
   stated rather than smoothed over, because it is the reason this report was rewritten twice. **`VREC-MOK-022` was
   captured four times.** The first three bound commits that the work then moved past — an evidence file was corrected
   after the first, amendment 6 was ratified after the second, and act 11's renumber moved every one of the record's
   twenty-six `evidence_paths` after the third — and a record cannot be re-pointed at a later commit or re-pathed after
   capture. All three stood at `ready` and none was accepted. Act 10 of *Decision record* is the owner's direction for
   the third; act 11 is what forced the fourth.
2. **The amendments.** All six ratified as written on 2026-08-22, in two rounds. The first five are acts 1 to 5 of
   *Decision record*, in the product, technical, technical, technical and assurance owner roles respectively. **The
   sixth is acts 7, 8 and 9**, all in the assurance owner role, put after the first five and after the code because
   executing the contract is what found it: three of `VER-MOK-004`'s own checks were misaligned with the tree. It
   changes no code and no rendered text, and it carries one further correction — `VER-MOK-004`'s *Independence*
   sentence disclaiming column widths, which cannot stand beside a matrix row that now measures one. **No
   `OUTSTANDING` mark remains in any of the four artifacts or in this work order.**
3. **The six verification results.** `gates.md`. Formatting clean; clippy clean under
   `--workspace --all-targets --all-features --locked -- -D warnings`; 303 tests pass with 0 failed and 0 ignored;
   the cross-target identity test demonstrated failing on a one-character divergence and the scratch reverted, checked
   both by re-running the target and by diffing the program's live output against the retained text; both texts
   rendered on both sides and every line inside eighty columns, which neither text managed before; `doctor` 81/81
   PASS and `validate` 148 artifacts with 0 errors and 0 warnings. `preflight --phase review` **PASSes at
   `implemented` with no diagnostic**; `--phase start` FAILs with `W005` at `implemented` exactly as it did at `draft`,
   because that phase admits only `approved` and `in_progress`. All three readings are retained, the two
   post-ratification ones verbatim in `preflight-implemented.txt`. An earlier draft of this report said the diagnostic
   "clears on the transition"; it does not, and the correction is recorded in *Lifecycle* rather than overwritten.
   **`tests/cli.rs` is 18 tests before and 18 after, with no assertion touched** — the number measured in a worktree
   at the base commit rather than inferred from the diff.

   **Items 7 to 9.** `nonperturbation/`, `defaults-divergences.txt` and `resilience-and-interface.txt`. All 43 matrix
   cells byte-identical base against candidate, on digest, line count and exit code, with `WO-MOK-004`'s `capture.sh`
   copied byte for byte and `cmp`-clean; all 40 matrix summary lines identical; the two 20-tick traced runs identical
   by `diff -r`. **One of the sixteen named cases changed and it is `--help`**, 2195 → 3600 bytes of standard output;
   each of the fourteen invalid-configuration cases holds exit `2`, an empty standard output and a byte-identical
   `configuration error:` line, their standard error growing 39 → 72 lines because the appended usage block grew
   38 → 71. Both emission paths carry the same 3601 bytes and 71 lines, equal to the retained text, with zero carriage
   returns. Acceptance scenario 3's three divergences re-run against the **rewritten** text: the printed-equals-applied
   direction fails exactly one test of eighteen and names `--seed`, which is the load-bearing demonstration. The
   10,000-tick runs are identical across all four decision sources and the 1,000-tick survivor floor is `8 9 10 9 9` on
   both sides. One public-surface line pair changed, the observer's `USAGE` reformat, across 30 public items.
   **Two of the contract's figures and two of its checks were not satisfiable as written**, which is amendment 6; they
   are satisfied under it, and the measurements above are the ones its replacement rows call for.
4. **What was left undone.** GitHub issue 40, the observer's `--events-path` writing nothing, deferred by the owner
   and disclosed in the observer's text until it is closed; and the two misplaced amendment rows in `SPEC-MOK-001`,
   recorded in *Out of scope* as observations. `validate` does not read markdown table structure, so its PASS is not
   evidence that those two rows are fine.

   **Three of `VER-MOK-004`'s checks were not satisfiable as written, and are now amended rather than left unmet.**
   Its *Line width* row asserted a synopsis first line of 81 columns before and after and called the synopsis
   unchanged; both halves are false, and the second contradicted ratified amendment 2. Its *Test placement* row named
   `tests/cli.rs` for the added test; this work order adds none there and one to `mokiterions-tui/tests/options.rs`,
   and the cross-target row of amendment 5 cannot be satisfied from the engine's test target at all. Its *Performance
   and resilience* survivor counts and its "same termination ticks" clause describe a tree master has moved past.
   Amendment 6 is the fix; the owner ratified all three parts on 2026-08-22 as acts 7 to 9, and each replacement is a
   measurement over both texts or against this work order's base commit rather than a figure fixed for later readers
   to inherit. **What is genuinely left undone under this heading is nothing about these three rows** — the residue is
   issue 40 and the two misplaced `SPEC-MOK-001` rows above, and the limits of coverage `VREC-MOK-022` states in its
   own words.
5. **What remains for the owner.** **One act: release**, which requires a record again. **The record was accepted on
   2026-08-22**, in the owner's words *"i accept the record, you can transition it, commit, push and PR"*, which also
   authorized the push and the pull request; `VREC-MOK-022` moves `ready` → `verified` in the commit following this one,
   because the record this document was written beside is the pre-renumber capture and act 11 required a fourth. The
   acceptance was given against the third capture, whose prose, figures and claims the fourth carries unchanged; the
   only fields that moved are the bound commit, the snapshot digest and the twenty-six evidence paths. Amendment 6 is
   not among the remaining acts either: it was put and ratified as acts 7 to 9, and act 10 directed the record
   superseded and re-captured against the commit carrying it.
