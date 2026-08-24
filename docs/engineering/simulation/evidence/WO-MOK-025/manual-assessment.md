# WO-MOK-025 manual assessments — recorded

`VER-MOK-018` states three manual assessments and states in one sentence why they exist at all:

> Three, and each is here because no check can make it.

**Two of the three are in scope at this stage, and both are recorded here.** **M1** and **M2** are the two
that read the shared rules block, which this work order writes. **M3 — the published comparison is honest**
depends on case **L24**, which needs an authorised live run at the declared seeds; no live run exists, none
is authorised, and M3 therefore belongs to the measurement rather than to this stage. It is listed among
what was not verified in `completion-report.md` item 12 and in `candidate/verification-cases.txt`, and it is
not recorded here.

`WO-MOK-025` *Evidence to record* item 7 fixes this file's form:

> **The assessment records** for **M1** and **M2**, naming the assurance owner and the date, with the shared
> rules block quoted in full in the **M1** record.

**Both assessments were made on 2026-08-24 by the repository owner acting as accountable assurance owner**,
over commit `4cfb297`, and each was put as its own question with its material displayed. The answers,
verbatim:

| # | Assessment | Accountable role | The owner's answer, verbatim | State | Date |
|---|---|---|---|---|---|
| **M1** | the shared rules block carries no strategy | assurance owner | *"Met — it carries no strategy"* | **RECORDED — met** | 2026-08-24 |
| **M2** | the shared rules block agrees with `SPEC-MOK-001` | assurance owner | *"Met — it agrees"* | **RECORDED — met** | 2026-08-24 |

**The commit the assessments were made over and the commit this file lands in hold the same block.**
`mokiterions-core/src/simulation.rs` is byte-identical between `4cfb297` and this commit — measured, not
assumed — so no reader has to decide whether the text assessed is the text retained. The commits between
them touch `docs/` only.

## Why nothing here carries a recommendation

The repository owner holds the product owner, technical owner and assurance owner roles at once. That is
what makes it possible for one turn to answer both of these, and it is also why **each was put separately
with its own material rather than as one act**: an assessment that is not answered is not answered by having
answered the other, and a single instruction covering both would have to name both to cover both. These two
were named.

**The implementation agent stated no recommendation on either, and the omission was deliberate.**
`DECISION_RIGHTS.md` is explicit that an implementation agent "may not self-approve that assessment unless
it is separately named as the accountable technical owner", and it is not so named here. Recommending an
outcome on an assessment reserved to the owner is that prohibition defeated by the back door: the owner
would then be ratifying the agent's judgement rather than making one. So what the agent supplied is
**material and measurement**, assembled before the decision and not written to lead it — which is why the
case against is stated in each section below, and why the one sentence in the block that comes closest to a
value statement is isolated in section 1 rather than dismissed there.

## 1. M1 — the shared rules block carries no strategy

> **M1 — the shared rules block carries no strategy.** Case **L27**. The assurance owner reads the block
> against `SPEC-MOK-007` rule 4.4's prohibitions — no goal, no preference, no objective, no advice — and
> against `SPEC-MOK-001` for accuracy. This is the assessment on which `ADR-MOK-007` decision 7 depends: a
> block that told the model to survive would make every figure a measurement of the instruction. An
> assessment that has not been made leaves this contract unsatisfied.

### The block, quoted in full

Item 7 requires the whole of it here rather than a citation, because an assessment of a text is not
reviewable against a pointer to that text.

**Provenance of this quotation.** `mokiterions-core/src/simulation.rs:1306`,
`const SHARED_RULES: &str = concat!(…)`, which is the **one** occurrence of the constant in the tree — check
**S7**, retained as check 9 of `candidate/architecture-checks.txt`, is what keeps the assessment to one
object so that it cannot be true of one copy and false of another. The quotation was produced mechanically,
by concatenating the 90 string literals in source order and un-escaping `\n` and nothing else; the extractor
asserts that no other backslash survives, which is what makes the un-escaping safe to do by hand. The
result is **5,381 characters, 5,385 bytes, 90 line breaks, 2 em dashes**, and its sha256 is
`39baca0f8665b17a519eb0b315443c667bdc1d6f5e9c90f343744303915a646a`. **The quotation was then read back out
of the fence below and re-hashed**: 5,381 characters, 5,385 bytes, the same digest. So a reader who doubts
the fidelity of the quotation can settle it without trusting this paragraph, and the block that was assessed
is the block that is retained.

The two figures a reader will see stated differently elsewhere are the same string in different units.
`simulation.rs:12731` asserts `SHARED_RULES.len() == 5_385`, and Rust's `len()` is **bytes**;
`candidate/request-layout.txt` records the block at "5385..5385 bytes" across the run and the completion
report calls it 5,381 characters. Both are right, and the difference is the two em dashes at three bytes
each.

```text
You are one Mokiterion. You will be given what you can see and the actions you may
take, and you answer with exactly one of them.

THE WORLD
The world is a grid 128 cells wide and 128 cells tall. A cell has an x coordinate from
0 to 127 and a y coordinate from 0 to 127. x rises toward the east and y rises toward
the south. The grid is split into two territories: territory A is every cell with y
from 0 to 63, territory B is every cell with y from 64 to 127. The boundary is a line
on the map and not a barrier, and crossing it is an ordinary move. Nothing exists
outside the grid, and a move that would leave it does not happen.

Time passes in turns called ticks. In one tick every living Mokiterion is asked for one
action, one after another, and each action is resolved before the next Mokiterion is
asked. Food resources occupy cells. Any number of Mokiterions and resources may share
one cell.

YOUR ATTRIBUTES
health, satiety, energy and fear are each an integer from 0 to 100.
  health   A Mokiterion whose health reaches 0 is dead and acts no further. Nothing
           raises health.
  satiety  Falls by 1 at the end of every tick. Eating raises it.
  energy   Falls by 1 at the end of every tick. Sleeping and eating raise it.
  fear     Rises by 10 at the end of a tick in which another Mokiterion was perceived,
           and falls by 5 at the end of a tick in which none was. Being threatened
           raises it by 30 at the moment of the threat.
At the end of a tick in which either satiety or energy is 0, health falls by 5.

waste_tolerance is an integer from 0 to 40. It never changes and it permits and forbids
nothing; it is stated because it is part of what a Mokiterion is.

PERCEPTION
A Mokiterion perceives every resource and every living Mokiterion at a distance of 16
or less, where the distance between two cells is the larger of the two coordinate
differences. Of each one it is given the identifier, the compass direction toward it
and that distance. A distance of 0 means the same cell. Two Mokiterions at a distance
of 1 or less — the same cell, or one of the eight cells around it — are in contact.
Nothing else about another Mokiterion is perceived: not its health, not its satiety,
not its energy, not its fear.

RESOURCES
A resource has a class, which is low, medium or high. Eating one raises satiety by 15,
30 or 50 and energy by 5, 10 or 20 for those three classes. Neither attribute passes
100, and whatever would have gone above 100 is lost. An eaten resource is removed from
the world.

THE FOUR CORE ACTIONS
  wait                Nothing happens.
  sleep               energy rises by 20, to no more than 100. Not permitted while
                      energy is already 100.
  eat <resource>      The resource must be in this Mokiterion's own cell. It is eaten
                      and removed from the world.
  move <direction>    One cell north, east, south or west. Not permitted where the
                      destination would be outside the grid.

THE SEVEN TARGETED ACTIONS
Each names exactly one other living Mokiterion by its identifier. A step toward or away
from a target moves on one axis: east or west while the target lies to the east or the
west, otherwise north or south, and on the other axis when the first would leave the
grid.
  attack <who>        The target must be in contact. Its health falls by 10 plus a
                      tenth of the sum of this Mokiterion's own energy and health, so
                      by 10 to 30. This Mokiterion's energy falls by 5. A target whose
                      health reaches 0 is dead.
  threaten <who>      The target must be in contact. Its fear rises by 30, to no more
                      than 100. Nothing else changes.
  fight <who>         An attack, resolved identically, permitted only against a
                      Mokiterion that struck this one since its previous action, and
                      only while that Mokiterion is in contact.
  retreat <who>       One cell away from the target. Permitted only against a
                      Mokiterion that struck this one since its previous action.
  surrender <who>     Permitted only against a Mokiterion that struck this one since
                      its previous action. Half of this Mokiterion's satiety, rounded
                      down, is taken from it and added to the target's, to no more than
                      100; whatever would have gone above 100 is lost.
  approach <who>      One cell toward the target. The target must be perceived and must
                      not be in the same cell.
  avoid <who>         One cell away from the target. The target must be perceived.

REJECTION
Every proposed action is checked against the world before it is applied, against the
conditions stated with it above. A rejected action changes nothing. The tick then moves
on, and the end-of-tick changes to satiety, energy, fear and health happen whether the
action was applied or rejected. The actions offered already exclude those whose
conditions what you are given shows to be unmet.

YOUR ANSWER
Answer with exactly one action from the set you are given. Name the verb, and where the
verb takes a resource, a direction or another Mokiterion, name exactly one and write it
exactly as that set writes it. Give no second action, no alternative, no explanation,
no reason and no confidence.
```

### Rule 4.4, and the block against it

> 4.4 It contains **no strategy, no goal, no preference and no advice.** It does not say that survival is
> desirable, that health should be kept high, that combat is risky, that cooperation pays, or that any
> action is better than any other in any circumstance. `INT-MOK-011` sets no viability floor for this
> source, and a block A that told the model to survive would be measuring the instruction rather than the
> model.

Rule 4.4 names five things the block must not say. None of the five appears in it: nothing in the text above
says that survival is desirable, that health should be kept high, that combat is risky, that cooperation
pays, or that any action is better than any other. The block's five sections state the world, the
attributes, perception, the resources and the eleven actions with their conditions and effects, and its last
section states the answer's grammar. **Every sentence is a statement about what is or what happens, except
the last section's, which is a statement about the form of the reply.**

### The mechanical half, and what it does not establish

`mokiterions-core/src/simulation.rs:11212 block_a_gives_no_strategy_and_names_nothing_that_varies` is the
automated part, and its own doc comment says what it is worth: it asserts rule 4.4 "by vocabulary, which is
a weaker instrument than the rule and is used deliberately". It asserts that the lower-cased block contains
none of `should`, `better`, `best`, `prefer`, `goal`, `strategy`, `survive`, `recommend`, `important`,
`aim to`, `in order to`, `so that you`, `risky`, `safe`; that it names no identifier of an actual run — the
identifiers are taken from a constructed simulation rather than guessed at — and neither the word `seed` nor
the seed's own digits; and, as the positive half without which every check above would pass on an empty
string, that it does state `0 to 100`, `0 to 40`, `distance of 16` and each of the eleven verbs in the form
the response grammar uses.

**What it cannot do is decide M1.** Advice can be written in words no list anticipates, and a list of
fourteen substrings is not a reading. The test's own comment says the review of the text is the rest of the
check, and that review is this assessment.

Rule 4.5's clause — no identity, no tick, no seed, no count of anything that varies — was also measured
directly over the extracted text: no `M01`…`M12` substring, no `tick` followed by a number, no occurrence of
`seed`, no occurrence of `survivor`. The **fifteen distinct integers** in the block are `0`, `1`, `5`, `10`,
`15`, `16`, `20`, `30`, `40`, `50`, `63`, `64`, `100`, `127`, `128`, and every one of them is a
specification constant that section 2's table locates in `SPEC-MOK-001`. None is a figure of a run.

### The case against, stated rather than left out

Four things in the block a reader could argue are directive, and what each actually is:

- **"You are one Mokiterion."** and the seven other occurrences of *you* or *your*. Second person addresses
  the reader; it does not tell the reader what to want. The block's alternative would be a third-person
  description of a Mokiterion, which rule 4.6's grammar would then have to bridge.
- **"Answer with exactly one action from the set you are given… Give no second action, no alternative, no
  explanation, no reason and no confidence."** This is the response grammar rule 4.6 requires the block to
  state, so that a response can be well-formed from block A alone. It constrains the *form* of the reply and
  names no action as better than another.
- **"The actions offered already exclude those whose conditions what you are given shows to be unmet."** A
  fact about how the offered set was built. It says nothing about which member of that set to pick.
- **"A Mokiterion whose health reaches 0 is dead and acts no further. Nothing raises health."** *This is the
  one sentence in the block that comes closest to a value statement, and it is isolated here rather than
  dismissed.* It states a mechanism and an absence; it names no preference, does not say that death is bad
  and does not say to avoid it. What a reader may nonetheless take from it is that one attribute is
  unrecoverable, which is information a strategy could be built on. Whether stating an irreversible
  mechanism is stating a preference is exactly the judgement rule 4.4 reserves to a person, and it is the
  judgement the decision below covers. The alternative — omitting it — would make the block inaccurate
  against `SPEC-MOK-001`, which M2 forbids, so the two assessments meet here.

### The decision

**Met.** Recorded 2026-08-24 by the repository owner as accountable assurance owner, over commit `4cfb297`.
The answer, verbatim: *"Met — it carries no strategy"*.

**What that settles.** `ADR-MOK-007` decision 7 rests on this assessment, and it now has it: a figure taken
from a live run of this source is a measurement of the model rather than of the instruction, as far as block
A is concerned. Case **L27** is satisfied for the block as it stands at this commit, and
`candidate/verification-cases.txt`'s row for it — which reads NOT MINE TO MAKE — is superseded by this
record and by the third amendment block at the head of that file, not by an edit to the row.

**What it does not settle.** It is an assessment of block A. It says nothing about block B, C or D, which
carry the run's own state and are checked mechanically; nothing about whether the model's answers are
reasonable, which no case in this contract asserts; and nothing about a later edit. A content change to the
block re-opens this assessment, because the object of it would then be a different text.

## 2. M2 — the shared rules block agrees with `SPEC-MOK-001`

> **M2 — the shared rules block agrees with `SPEC-MOK-001`.** The block is a restatement, and
> `SPEC-MOK-007` rule 4.2 fixes which governs, but nothing detects drift automatically. The assessment is
> made whenever the block or `SPEC-MOK-001`'s rules change, and check **S7** is what keeps it to one object.

Rule 4.2 fixes the direction of authority: the block's "content is derived from `SPEC-MOK-001` and is a
restatement for a reader, not a second authority. Where the two differ, `SPEC-MOK-001` governs." So the
question is not which text is right — that is settled — but whether the restatement is faithful.

### The cross-check, claim by claim

Every factual claim the block makes, against the provision of `SPEC-MOK-001` that fixes it. Line numbers
are that document's at this commit.

| The block's claim | `SPEC-MOK-001` | Agrees |
|---|---|---|
| grid 128 wide by 128 tall, `x` and `y` each 0 to 127 | line 271, coordinates in `0..=127`; line 846, "a 128 by 128 world" | yes |
| two territories: A is `y` 0 to 63, B is `y` 64 to 127 | lines 272–273 | yes |
| the boundary is a line on the map and not a barrier | line 274, "label state but do not block movement" | yes |
| nothing exists outside the grid; a move that would leave it does not happen | rule 8, line 569, a valid move changes one coordinate by one cell in a cardinal direction | yes |
| one action per living Mokiterion per tick, resolved one after another | rule 2, line 504, living agents in ascending identifier order; rule 6, line 543, one action opportunity per proposal | yes |
| any number of Mokiterions and resources may share one cell | line 290 | yes |
| `health`, `satiety`, `energy`, `fear` are integers 0 to 100 | line 284, values in `0..=100` | yes |
| nothing raises health | **an absence rather than a provision** — see below | yes |
| satiety and energy each fall by 1 at the end of every tick | rule 12, line 573 | yes |
| health falls by 5 at the end of a tick in which either is 0 | rule 12, line 573 | yes |
| fear rises by 10 when another was perceived, falls by 5 when none was | rule 12, line 573 | yes |
| being threatened raises fear by 30 at the moment of the threat | rule 23, line 666 | yes |
| `waste_tolerance` is an integer 0 to 40, never changes, permits and forbids nothing | line 361, "an integer in `0..=40`. It is fixed at initialization and never changes" | yes |
| perception reaches distance 16 or less, distance being the larger of the two coordinate differences | lines 453–454, radius `16`, Chebyshev distance | yes |
| of each perceived entity: identifier, compass direction, distance | line 455 and line 753 | yes |
| distance 1 or less is contact, including the same cell | lines 461–462, contact radius `1`, distance `0` included | yes |
| nothing else about another Mokiterion is perceived — not health, satiety, energy or fear | line 54, `CAP-MOK-010`'s exclusion, and line 753 | yes |
| eating raises satiety by 15, 30 or 50 and energy by 5, 10 or 20 by class | the food table, lines 413–417 | yes |
| neither attribute passes 100 and the excess is lost | rule 9, line 570, "Attribute values are capped at `100`" | yes |
| sleep raises energy by 20, to no more than 100, and is not permitted at 100 | rule 10, line 571; rule 4's `sleep` candidate at line 514 is gated on energy below `100` | yes |
| attack: the target must be in contact; health falls by 10 plus a tenth of the striker's energy plus health, so 10 to 30 | rule 22, line 651, damage `10 + (striker.energy + striker.health) / 10`, range `10..=30` | yes |
| the striker's energy falls by 5 | rule 22, line 655, "a flat `5` `energy`" | yes |
| threaten: fear rises by 30, to no more than 100, and nothing else changes | rule 23, lines 666 and 668 | yes |
| fight is an attack resolved identically, only against a Mokiterion that struck since the previous action, only while in contact | rule 22, line 649; the two preconditions at line 557 | yes |
| retreat and surrender require having been struck since the previous action | line 554, condition 4 for `fight`, `retreat` and `surrender` | yes |
| surrender transfers half of the surrendering Mokiterion's satiety, rounded down | rule 24, line 675, `satiety / 2` truncating toward zero | yes |
| approach and avoid: one cell toward or away; the target must be perceived | rule 21, line 640 | yes |
| a step toward or away moves on one axis, east/west while the target lies east or west, otherwise north/south, and the other axis when the first would leave the grid | line 640, rule 5 case 3's axis rule unchanged | yes |
| every proposal is checked before it is applied and a rejected action changes nothing | rule 6, line 543, "causes no action-specific mutation" | yes |
| end-of-tick changes happen whether the action was applied or rejected | rule 12, line 573 | yes |

**No claim in the block is unaccounted for in that table**, and no figure in the block is absent from
`SPEC-MOK-001`: the fifteen distinct integers section 1 enumerates are each located above.

### The one claim that is not a restatement of a provision

**"Nothing raises health."** No rule in `SPEC-MOK-001` says so, because there is nothing for such a rule to
prohibit. What the claim rests on is an exhaustive reading of every provision that writes `health`:
initialization sets it to `100` (line 288), rule 12's decay subtracts `5` (line 573), rule 22's damage
subtracts the computed amount (line 651), and rule 13 marks the Mokiterion dead at zero (line 584). Eating
raises satiety and energy and not health — the food table has two columns, not three. So the claim is true
of the specification as a whole while being derived from the **absence** of a provision rather than from
one, and it is the only line in the block whose agreement cannot be shown by pointing at a rule. That is
worth a reader's attention for one reason: a later amendment that adds a way to raise health would falsify
this line of the block without contradicting any sentence of `SPEC-MOK-001` that the block quotes, so no
drift check anchored on quoted rules would notice.

### What no check does, and what one check does

Nothing in this repository detects drift between the block and `SPEC-MOK-001`. `VER-MOK-018` says so, and it
is why M2 exists: "the assessment is made whenever the block or `SPEC-MOK-001`'s rules change". What check
**S7** does is narrower and still necessary — it holds the block to exactly one occurrence in the tree, so
that this assessment has one object and cannot be true of one copy and stale on another.
`candidate/architecture-checks.txt` check 9 is that measurement.

### The decision

**Met.** Recorded 2026-08-24 by the repository owner as accountable assurance owner, over commit `4cfb297`.
The answer, verbatim: *"Met — it agrees"*.

**What that settles.** The restatement is faithful as of this commit, for the block as quoted in section 1
and for `SPEC-MOK-001` as it stands here. **What it does not settle** is any later state of either: this
assessment is re-made whenever the block or the provisions above change, and the amendment that changes one
of them is where the re-making belongs.

## What these two records do not establish

- **They are not verification.** The verification decision for `WO-MOK-025` is the assurance owner's and
  lives in `VREC-MOK-024`. These are two of the inputs a record binds.
- **They do not cover M3.** M3 needs an authorised live run and belongs to the measurement stage.
- **They cover one commit.** The block and the cited provisions are as they stand here;
  `mokiterions-core/src/simulation.rs` is byte-identical to `4cfb297`, the commit the assessments were made
  over.
- **No check made either judgement, and none could.** The material above is re-derivable by a later reader;
  the judgement is not. That asymmetry is the reason `VER-MOK-018` states these as assessments rather than
  as cases, and the reason an unrecorded one leaves the contract unsatisfied rather than merely unmeasured.
