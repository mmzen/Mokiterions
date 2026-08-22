+++
id = "WO-MOK-020"
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
evidence retained under `evidence/WO-MOK-020/`. The two transitions are recorded as one because nothing happened
between them: implementation was already complete and its evidence already captured when approval was given, which
is the sequence the paragraph above discloses rather than conceals.

`harnessctl preflight --work-order WO-MOK-020 --phase start` raised `W005` — *status 'draft' is not eligible for
start* — at every capture taken while this document was `draft`. That diagnostic was retained in
`evidence/WO-MOK-020/gates.md` rather than worked around, because it was correct: it is the harness saying a `draft`
work order authorizes no implementation, which was exactly the state.

**It does not clear on this transition, and saying so would be wrong.** The `start` phase admits `approved` and
`in_progress`, so at `implemented` it raises `W005` again for the opposite reason — the status is past start, not short
of it. **The applicable phase at `implemented` is `review`, and it PASSes with no diagnostic.** Both captures are
retained beside the original failing one, so a later reader sees the whole sequence rather than a single green
screenshot: `draft` fails `start` because implementation was unauthorized, `implemented` fails `start` because
implementation is finished, and `implemented` passes `review`, which is the gate this status is actually measured by.

Commit-bound verification is classified `required` above. `VREC-MOK-020` is prepared against the commit that carries
this transition and is `ready`, not `verified`: preparing a record is the implementation agent's act and accepting one
is the assurance owner's, and this transition did not include that acceptance. Release requires a separate record
again.

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

### 4. The five amendments in *Required amendments* below

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

Four of the five are prerequisites: the change as directed cannot be conformant without them, and this is stated
plainly rather than discovered at review. Each is written into its artifact.

**All five were ratified on 2026-08-22**, each put as its own question with its alternatives, and each recorded in its
artifact's own amendment record naming the acting role and the alternatives declined. The `OUTSTANDING` marks the five
carried while this work order was `draft` are gone from those records; *Decision record* below is the account. The
statements of substance in the five subsections that follow are unchanged from the drafted form the owner was shown —
only their ratification state is updated, so that what was approved is legible as what was put.

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

## Decision record

Five acts of 2026-08-22, each put as its own question with its alternatives, and eleven alternatives declined. One
person holds all three accountable roles in this repository, which makes each approval cheap to obtain and
correspondingly easy to skip; the role each act was taken in is named because nothing here is approved by implication.

| # | Act | Role | Decision | Declined beside it |
|---:|---|---|---|---|
| 1 | `REQ-MOK-018`'s third clause and the replaced verbosity bound | product owner | as written | descriptions beneath the block; requirement alone with amendment 3 deferred; revert both texts |
| 2 | `SPEC-MOK-001` *Inputs* `<number>` | technical owner | as written | restore `<u64>`; a third placeholder |
| 3 | `SPEC-MOK-001` *Help output* structure | technical owner | as written, one act with 1 | — put with 1 |
| 4 | `SPEC-MOK-003` *Start-up inputs* disclosure | technical owner | as written, defect characterization kept | record without calling it a defect; drop the disclosure from spec and help text |
| 5 | `VER-MOK-004`'s four rows, two scenarios, three corrections | assurance owner | as written | ratify but hold this work order at `draft`; decline the rows |
| 6 | This work order `draft` → `approved` → `implemented` | engineering owner | taken with act 5 | hold at `draft` |

**Act 1 is the one the change rests on**, and it is the one the owner's implementation instruction of the same date
did not cover. The instruction was given on the text; this act is on the requirement the text turns out to need. Had
it been declined, acts 2 to 6 would have had nothing to serve and both help texts would have reverted.

**No product decision was reopened.** `--ticks`' default of `100` stayed at `100`; `REQ-MOK-018`'s *Open decisions*
item 1 reserves it to the product owner and it was not put here. No default, accepted value, constraint, exit code,
simulated behavior or record changed in any of the six acts.

**The implementation agent decided none of the six.** It found the four conflicts, wrote the amendments, measured the
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
| `docs/engineering/simulation/verification/VER-MOK-004.md` | amendment 5 |
| `docs/engineering/simulation/evidence/WO-MOK-020/` | evidence below |

Nothing under `mokiterions-core/src/simulation.rs` is touched, which is the whole of the non-perturbation claim: no
engine code is in the diff at all.

## Required verification

1. `cargo fmt --all -- --check` clean.
2. `cargo clippy --all-targets` with no warning.
3. The whole suite green, including all eighteen inherited `tests/cli.rs` tests **unmodified**.
4. The added observer test demonstrated load-bearing: one shared entry edited in one target only, in a scratch
   change, must fail it and name the option. The scratch change is reverted and not committed.
5. Both texts rendered and retained, with a column-width check.
6. `harnessctl doctor` and `validate_engineering_artifacts.py` at the pinned harness version.

## Evidence to record

Under `docs/engineering/simulation/evidence/WO-MOK-020/`:

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

## Stop and escalate conditions

1. An inherited `tests/cli.rs` assertion cannot be satisfied by the new text. Stop. Do not relax it: `VREC-MOK-004`
   binds it. Escalate with the assertion and the text beside each other. *(Did not occur.)*
2. A description cannot be written without stating something no approved artifact states. Stop and escalate rather
   than inventing the fact. *(Did not occur; `--density`'s two further constraints and the four value descriptions
   were all already stated, and where they are stated is recorded in the evidence.)*
3. Any run's output, exit code or entropy sequence differs. Stop. *(Cannot occur: no engine code is in the diff.)*
4. An amendment is found to be needed that is not in *Required amendments*. Add it there, mark it **OUTSTANDING**,
   and name it in the completion report rather than presenting it as approved. *(Amendments 1 to 5 were all found
   this way, after the directed implementation. See *Lifecycle*.)*
5. The owner declines any of the five amendments. The text reverts to whatever the surviving set permits, which for
   amendment 1 declined is the previous text entirely. *(Did not occur; all five were ratified as written on
   2026-08-22. See *Decision record*.)*

## Completion report format

1. What changed, by path.
2. The five amendments, each with its acting role and its ratification state.
3. The six verification results, with the scratch demonstration.
4. What was left undone and why: issue 40, and the two misplaced amendment rows.
5. What remains for the owner.

### Completion report, 2026-08-22

1. **What changed.** Twenty paths against `f7b1c45`, the shape of them in *Expected change surface* above. Two `USAGE`
   constants rewritten, one test helper's terminator, one test added, four governing artifacts amended, this work
   order, and **eleven** evidence files — the eleventh is `preflight-implemented.txt`, added with the ratification.
   The count is `git diff --name-only f7b1c45`, not the table's row count, which groups the evidence directory into
   one line. No engine source is in the diff.
2. **The five amendments.** All five ratified as written on 2026-08-22 — acts 1 to 5 of *Decision record*, in the
   product, technical, technical, technical and assurance owner roles respectively. No `OUTSTANDING` mark remains in
   any of the four artifacts for this work order.
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
4. **What was left undone.** GitHub issue 40, the observer's `--events-path` writing nothing, deferred by the owner
   and disclosed in the observer's text until it is closed; and the two misplaced amendment rows in `SPEC-MOK-001`,
   recorded in *Out of scope* as observations. `validate` does not read markdown table structure, so its PASS is not
   evidence that those two rows are fine.
5. **What remains for the owner.** `VREC-MOK-020` is `ready` and not `verified`: accepting it is the assurance owner's
   separate act. Release requires a record again. Nothing has been pushed and no pull request has been opened.
