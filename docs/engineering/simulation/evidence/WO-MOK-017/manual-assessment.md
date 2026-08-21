# `WO-MOK-017` manual assessments — prepared records

| Field | Value |
|---|---|
| Contract | `VER-MOK-016`, *Manual assessments* — assessment 5 as realigned on 2026-08-21, and the second half of assessment 6, carried here by `WO-MOK-016`'s own record |
| Retention item | "The manual assessments this work order reaches, each with its accountable role and date" (`WO-MOK-017` *Evidence to record*); and *Required verification*: "Manual assessment 5 — the ceiling's ratification or amendment on the measured curve — travels with them" |
| What this file is | **prepared material, and nothing more.** Each section states the assessment's own words, the measured evidence it is to be taken against with a citation to the record that holds each figure, the outcomes available, and an **empty** record block. Everything outside the blocks was written by an implementation agent, which may prepare a record and may not sign one. **No block below is completed.** |
| State | **0 recorded, 3 outstanding.** Assessment 5 in both of its realigned halves, and assessment 6's second half. `VER-MOK-016`'s assessment 4 is re-measured here and is **not** reopened; nine findings are raised for their owners |
| Baseline | `pre/COMMIT.txt` — `1ba5a3aabe775c9cdee29a04b399af3bb82dde90`, this work order's own approval commit, no source file changed |
| Candidate | `post/COMMIT.txt` — the commit the packet's post-change capture was taken from |
| Merge | **none.** This work order is unmerged; a record here binds a branch commit and not `master`'s merge of it |
| Figures | **no measurement is taken here.** Every figure is cited to the packet record that holds it; where this file and that record would differ, the record is right |
| Date prepared | 2026-08-21 |

`VER-MOK-016` states the standard this file is written against: "Each of the following is an explicit
judgement recorded by the accountable role. **An unrecorded assessment is an outstanding assessment, and
this contract is not satisfied while any remains outstanding.**"

**What is different about assessment 5 here.** `REQ-MOK-060`'s ceiling was ratified in advance at one half
and **amended to three fifths on 2026-08-21**, on the approval measurement retained in `approval/`. That
amendment is an act in `REQ-MOK-060`'s own record and it closes the *value*. `VER-MOK-016`'s realignment row
of the same date says so in terms: "the ceiling's value is no longer what the reviewer assesses, because
`REQ-MOK-060`'s amendment record closes it and a verification contract does not reopen a requirement". What
the assessment now asks for is the two things the approval measurement could not settle — that three fifths
is met **on the shipped build** with the margin the amendment claimed, and whether the corrected composition
warrants a per-class floor. Both are measured for the first time in this packet, and neither has been put to
the owner. They are therefore outstanding, not discharged by the amendment that preceded them.

**How to complete one.** Fill the four lines of that assessment's **Record** block and nothing else. The
evidence and outcome sections are the prepared material and are not the owner's to sign around; leaving them
as they stand is what makes a completed record legible as a judgement taken against stated evidence. A record
here **amends nothing by itself**: where the outcome is an amendment, the amendment is an act in the amended
artifact's own record, and the block cites it rather than substituting for it.

The numbering is `VER-MOK-016`'s, unchanged.

---

## Where each stands

| # | Subject | Owed by | State |
|---|---|---|---|
| 5a | three fifths met on the shipped build, with the margin the amendment claimed | product owner | **outstanding** — measured here for the first time |
| 5b | whether the corrected composition warrants a per-class floor | product owner | **outstanding** — re-reserved to this measurement on 2026-08-21, and the measurement now exists |
| 6, second half | the additivity cost, "having seen the characterized divergence" | product owner | **outstanding** — carried here by `WO-MOK-016`'s record of 2026-08-21, and assessable for the first time |
| 4 | the survivor floor of five and the lethality bound | product owner | **recorded 2026-08-20, and not reopened here** — re-measured as an oracle; its stated reasoning moved, which is finding 8 |
| 1, 2, 3, 7, 8, 9, 10, 11 | `WO-MOK-016`'s combat assessments | product, technical and assurance owners | **recorded 2026-08-20 or 2026-08-21**, outside this work order's scope, and untouched by it |

The repository owner holds all three roles. That does not merge them: an assessment owed by the product owner
is taken as product owner, and the role line of each block records which one acted.

---

## 5a. Three fifths on the shipped build — **outstanding**

**The contract's words.** "What remains to be assessed here is therefore not the value — `REQ-MOK-060`'s
amendment record closes it, and this contract does not reopen a requirement — but two things the approval
measurement could not settle: that three fifths is met on the shipped build with the margin the amendment
claimed, and whether the corrected composition warrants a per-class floor. A curve that meets three fifths
only by sitting on it is a finding about the mechanism, and a further amendment is as available an outcome as
ratifying the result."

**What the amendment claimed.** `REQ-MOK-060`'s row of 2026-08-21: "The best floor-respecting point measured
is the condition ratified with this row, and its worst class share is `54.1%` — four points above one half,
and six below three fifths." That figure came from the approval probe against a patched build, on the 15
obligated cells. The claim under assessment is that the shipped engine reproduces it.

**The evidence.**

| Figure | Value | Record |
|---|---|---|
| territory evaluations the requirement binds | **30** — 3 sources × 5 seeds × 2 territories, every one reaching 1,000 ticks | `post/composition.txt` §6 |
| evaluations breaching three fifths | **0 of 30**; runs carrying a breaching territory **0 of 15** | `post/composition.txt` §6 |
| the widest class's share, lowest and highest | **33.3%** and **54.1%** | `post/composition.txt` §6 |
| the worst evaluation, named | social seed 1 territory B, `high` at **54.1% of 61** — **5.9 points** below the ceiling, and **4.1 points above** the value it was amended from | `post/composition.txt` §6 |
| the amendment's claim, on the shipped build | the approval measurement predicted a worst share of **54.1%**, "reproduced here on the shipped build to the tenth of a point" | `post/composition.txt` §6 |
| what the correction moved | pre-change **27 of 30** over one half, **20 of 30** over three fifths, worst **81.6%**; candidate **5 of 30** over one half, **0 of 30** over three fifths, worst **54.1%** | `post/composition.txt` §7 |
| evaluations still over one half at the candidate | **5**, each named — social 1 B at 54.1%, individual 42 B at 53.6%, individual 777 A at 52.2%, individual 42 A at 51.1%, individual 0 A at 50.8% | `post/composition.txt` §7 |
| the retained pre-change curve, reproduced | `WO-MOK-016`'s four headline composition figures reproduce, including its worst of **81.6%** | `post/composition.txt` §8 |
| outside the bound, disclosed | at density **1.50**, which the requirement does not bind: **30** evaluations above three fifths pre-change and **1** after — individual seed 1 territory A, `high` at 63.9% of 61 | `post/composition.txt` §10 |
| the price of meeting it | **9 of 15** obligated runs leave fewer Mokiterions living, 2 more, 4 unmoved; **788** more resources consumed across the matrix and **85** fewer standing at the horizon | `post/survivors.txt` §4 |
| the floors, which the ceiling was traded against | all three met; margins **0**, **0** and **2** | `post/survivors.txt` §3 |

**The outcomes available.** Ratifying the result is one. So is a further amendment of `REQ-MOK-060` — the
contract says a curve that meets three fifths "only by sitting on it" is a finding about the mechanism, and
the margin is 5.9 points at the worst row against a distribution whose top five rows all sit above one half.
So is deciding that the trade the ceiling was met by — a thinner larder, 9 of 15 runs leaving fewer living,
and two floors at zero margin — is not the trade the owner intended, in which case the amendment to revisit
is not `REQ-MOK-060`'s ceiling but one of the floors, or the correction's permitted surface. Silence is not
an outcome.

    Record — OUTSTANDING
    Decision:  --
    Role:      product owner
    Date:      --
    Basis:     --

---

## 5b. The per-class floor — **outstanding**

**The contract's words.** The same clause, second half: "whether the corrected composition warrants a
per-class floor." `REQ-MOK-060`'s *Open decisions* defers it beside the ceiling on the ground that "a ceiling
alone can be satisfied by a world that has drifted the other way", and the owner **re-reserved the decision
on 2026-08-21** to the corrected composition this work order measures. That measurement now exists, so the
ground the deferral stood on is gone.

**The evidence.** `post/composition.txt` §9 exists for this decision and for nothing else. No floor is in
force and none is proposed there or here.

| Figure | Value | Record |
|---|---|---|
| the narrowest class's share, lowest at the candidate | **14.3%** — individual seed 42 territory B, `low` of 56 | `post/composition.txt` §9 |
| the same figure at the pre-change commit | **4.1%** | `post/composition.txt` §9 |
| evaluations where the narrowest class is `low` | **27 of 30** | `post/composition.txt` §9 |
| evaluations where the narrowest class holds nothing at all | **0 of 30** | `post/composition.txt` §9 |
| the whole per-territory per-class table, both commits | 30 evaluations either side | `post/composition.txt` §5 |

The correction raises the narrowest class as well as lowering the widest — 4.1% to 14.3% at the worst — which
is the direction a floor would want and is why the deferral was reasonable to carry this far. It is also why
the decision is now takeable rather than merely deferrable again: the figure a floor would be set against is
measured, on the shipped build, on every cell the requirement binds.

**The outcomes available.** Three, and a fourth that is not one. Binding a floor is available, and it would
be an amendment to `REQ-MOK-060` and a new coverage row in `VER-MOK-016`, which are acts in those artifacts.
Declining to bind one is available, and the ground for it is on the table: the drift the requirement was
written about is one-directional, the narrowest class rose rather than fell, and nothing was measured
approaching zero. Deferring it a third time is available, but the ground the previous two deferrals rested on
— that there was no corrected composition to measure it against — has been discharged, so a third deferral
needs a new reason stated. Leaving it unrecorded is not available: `VER-MOK-016` names it inside assessment 5,
and an unrecorded assessment is an outstanding assessment.

    Record — OUTSTANDING
    Decision:  --
    Role:      product owner
    Date:      --
    Basis:     --

---

## 6, second half. The additivity cost — **outstanding**

**The contract's words.** "The additivity cost, by the product owner. `reference` and `individual` outcomes
move. The owner records that this is accepted, having seen the characterized divergence, and approves the
narrowing of `REQ-MOK-034`'s frozen-outcomes clause to `baseline` alone."

**Why it is here.** The first half — the narrowing of `REQ-MOK-034` to `baseline` alone — was approved on
2026-08-20 and `WO-MOK-016`'s record calls it "taken and unused: a permission granted ahead of its use". The
second half was recorded on 2026-08-21 as **not assessable at that candidate**, because the premise "having
seen the characterized divergence" was false there: nothing had diverged, the change that would move those
outcomes being `REQ-MOK-060`, descoped to this work order. That record carried it here in terms — "it is taken
where the outcomes actually move". The outcomes have now moved, and the permission granted ahead of its use is
now used.

**The evidence.**

| Figure | Value | Record |
|---|---|---|
| obligated cells that diverge | **90 of 90** — every `reference`, `individual` and `social` cell at every density | `post/divergence.txt` §3 |
| `baseline` cells, which the narrowed clause still freezes | **30 of 30** byte-identical, and 30 of 30 exit codes equal | `post/byte-identity.txt` |
| attribution of each cell's first divergence | **57** direct (R1), **9** through the shared stream (R2), **24** through the traced sibling (R3), **0** unattributed | `post/divergence.txt` §3 |
| the refutation check | earliest first divergence **tick 14**, earliest measured band entry **tick 14**, cells diverging before their own band entry **0** | `post/divergence.txt` §4 |
| the trait itself | cells whose `waste_tolerance` assignment moved **0** — the same trait read differently, not a different trait drawn | `post/divergence.txt` §4 |
| every first diverging record, in full | one per cell, 90 of them | `post/divergence.txt` §6 |
| the correction's entropy cost | the corrected condition consumes **no** entropy; all 15 initialization draw totals and all 120 initialization prefixes identical at both commits | `post/entropy.txt` §§1–3 |
| what the movement cost | 9 of 15 runs fewer living, 2 more, 4 unmoved; consumption 26,136 → 26,924; standing at the horizon 1,680 → 1,595 | `post/survivors.txt` §4 |

**What the characterization concedes, in its own words.** `post/divergence.txt` §9: route R2 means "a
divergence can reach a Mokiterion the condition never applied to, through the draw count of one it did apply
to... it does mean the *set* of Mokiterions whose behavior moved is larger than the set whose answer the
condition changed." And it does not claim every *subsequent* difference in a diverged stream is individually
attributable — once two runs part they are different worlds, and stop condition 3 is written about the first
diverging record, which is where a divergence is attributable at all. The owner is asked to accept the cost
having seen that, which is the form the contract asks for.

**The outcomes available.** Accepting the cost is one, and it is the outcome the first half's permission
anticipated. Not accepting it is available and has a consequence worth naming: `REQ-MOK-034`'s narrowing is
already approved, so declining here does not restore the frozen outcomes — it puts the correction itself in
question, which is `REQ-MOK-060`, not this clause.

    Record — OUTSTANDING
    Decision:  --
    Role:      product owner
    Date:      --
    Basis:     --

---

## 4. The survivor floor of five — **recorded 2026-08-20, and not reopened here**

`VER-MOK-016` assessment 4 was recorded on 2026-08-20, "on the first measured curve", and `WO-MOK-016`'s
packet holds the block. This work order re-measures the floor because `REQ-MOK-058`'s ratification was taken
on the uncorrected world, but it re-measures it **as an oracle**, not as a new assessment: the record binds the
curve it names, and an implementation agent does not reopen a closed record.

| Figure | Value | Record |
|---|---|---|
| `REQ-MOK-058`, floor 5 | met at both commits; worst seed 1 leaves **7** living with 2 combat deaths | `post/survivors.txt` §3 |
| `REQ-MOK-014` and `REQ-MOK-034`, floor 8 | met at both commits; worst seeds leave **8** and **8** | `post/survivors.txt` §3 |
| the `social` default-density survivors | **9, 10, 9, 9, 11** pre-change → **9, 7, 9, 8, 9** at the candidate | `post/survivors.txt` §4 |
| combat deaths on the same five seeds | **1, 2, 2, 3, 1** at **both** commits — unmoved | `post/survivors.txt` §4, `post/docs-figures.txt` §2 |
| the lethality bound | met on every declared seed; traced and untraced agree 30 of 30 | `post/survivors.txt` §§5–6 |

The verdict does not move: the floor of five is met, and no seed is bloodless. What moved is the
ratification's stated **reasoning**, and that is finding 8 below rather than anything here.

---

## Findings raised, each for its owner

None of these is an assessment `VER-MOK-016` names. Each is something this work order measured or met that
needs a decision it has no authority to take, and each is raised here so that accepting it is an act rather
than an omission. Three of them are raised by an evidence reader in its own closing text and point at this
file by name.

**1. The per-run draw total is not derivable at this candidate — technical owner.** `VER-MOK-016`'s retention
item asks for "the total draw count per run per source at matched seeds". `post/entropy.txt` §5 records that
it cannot be produced and why, rather than substituting a proxy: a counter on the released surface is barred by
this work order's *Out of scope* ("any new attribute, event, interface item or instrumentation"); recovering
the count by replay means reimplementing every draw-consuming path, because occupancy rejections are invisible
in the stream; and a golden-value test would pin an arbitrary number no requirement states. What stands in its
place is measured and is listed there. **The decision owed:** whether a draw counter belongs on the released
surface, or whether a work order may carry a replay of the whole engine as evidence. `WO-MOK-016` left a
comparable clause unwritable and recorded it; this is the same discipline on a narrower gap.

**2. `VER-MOK-013`'s gauge scenario is coupled to a model rather than constructed — technical and assurance
owners.** `a_declining_mokiterion_shows_a_declining_bar` searches a live 200-tick run for a Mokiterion whose
health falls thirty points. Its own guard fired at this candidate: `post/health-falls.txt` measures the deepest
such fall falling from **80 → 0** under `reference` and **45 → 0** under `individual`, because the correction
feeds Mokiterions sooner and health falls only once satiety reaches zero. The amendment selects `--policy
social`, where seed 42 falls **85 → 85** and the decline is combat damage. That is a scenario-parameter change
and not a retreat from an assertion — both assertions and the threshold of thirty are untouched — but it
substitutes one model dependency for another. **The decision owed:** whether the scenario should construct a
declining subject directly, which is a `VER-MOK-013` amendment. The nutrition model has now broken this
scenario twice.

**3. `VER-MOK-005`'s dead-selection scenario, the same question — technical and assurance owners.**
`a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour` searches a 700-tick run for a
dead identifier with living identifiers on both sides. `post/dead-neighbours.txt` measures why the pinned
version broke: the first death moved from `M05` at tick 604, interior, to **`M01` at tick 314**, the lowest
identifier, from which BackTab wraps and `assert!(backward < dead)` cannot hold. The amended search settles at
tick 399 at the candidate, inside the horizon at both commits. The state is constructible in one line through
the existing test hooks. **The decision owed:** whether it should be, which is a `VER-MOK-005` amendment — the
test discharges `REQ-MOK-021`'s "Selected Mokiterion dies" and `REQ-MOK-023`'s "Selection cycles living
Mokiterions only", and neither states how the state is reached.

**4. Nine figures in the two reader-facing documents describe a world the engine no longer implements —
product owner.** `post/docs-figures.txt` §12 enumerates them and confirms each one carries a flag in its own
document saying it was not re-measured; a figure of that kind stated flatly fails that reader. Six are in
`SIMULATION_RULES.md` and three in `docs/ROADMAP.md`. **The substantive one is the long horizon**, in that
reader's own words: "`REQ-MOK-060` binds tick 1,000 and nothing past it, deliberately, so `WO-MOK-017`
measured nothing past it either — and the figures that made high-class accumulation look fatal over 10,000
turns are exactly the figures the correction is most likely to have moved." Those are the extinction tick of
**9,154**, the 10,000-turn comparison leaving 1 to 5 survivors on four of five seeds, and the tolerance-range
measurement that cut the range at 40 on three of five seeds dropping below eight survivors. **The decision
owed:** whether a long-horizon requirement is stated, and whether the correction's effect past tick 1,000 is
measured before or after it. `docs/ROADMAP.md` carries the re-measurement as outstanding work against a
requirement that is still unstated.

**5. The `--help` prose no longer distinguishes `individual` from `reference` — technical owner.**
`mokiterions-core/src/cli.rs:33` says the individual policy "seeks and consumes as the reference policy does,
except that each Mokiterion also accepts food it would partly waste, in proportion to its own waste
tolerance". After the correction the reference rule accepts partly-wasted food too, by rule 5's own allowance,
so the distinguishing clause is now the *amount* of waste admitted and not the fact of it. **It was
deliberately not edited.** `SPEC-MOK-001` *Help output* requires the prose to describe all four sources and to
state no default or constraint, and fixes no wording, so the text is the technical owner's and amending it is
outside this work order's expected change surface. It is not wrong about behavior — `individual` does admit
more waste — but a reader takes it as the boundary between the two sources, and that boundary moved.

**6. Fear saturation leaves rule 23's threat inert — product owner, carried from `WO-MOK-016` assessment
3.** `post/docs-figures.txt` §4 measures **2,850** `threat_resolved` records across 60 social cells at both
commits, of which **one** moved the target's fear at all, and by 5 through the clamp:
`tick=337 subject=M02 event=threat_resolved result=target:M04,increase:5,target_fear:95->100`. Every other
threat lands on a target already at 100. §5 measures **34%** of 160,921 creature-turns sitting at fear 100 and
52% above 0. The gate is working exactly as written — nothing straddles 95, every engagement is proposed at 90
or below and every backing-off at 95 or above — but the consequence is that "which side of the line a
Mokiterion falls on is decided by how many turns it has had company, not by anything about that company".
`REQ-MOK-055`'s constant of `30` is inert against rule 12's `+10 / −5` driver. This work order changed nothing
here and could not: a new attribute or a changed fear driver is out of scope. **The decision owed:** whether
the driver is what wants amending, which assessment 3 ratified `30` in the knowledge of and which the
correction has not moved.

**7. An approved amendment row in `VER-MOK-016` names the wrong item — assurance owner.** The realignment row
of 2026-08-21 reads "**Manual assessment 7** carries it on both of its halves." Manual assessment 7 is the
fourth source's name, it has no halves, and it is untouched. The item that was amended on both of its halves
is **acceptance scenario 7** (`VER-MOK-016` lines 385–386: three fifths on each seed, then the pre-change
measurement exceeding three fifths). The substance landed correctly in the artifact and nothing depends on the
label. **Why it is a finding and not a fix:** `WO-MOK-017`'s *Out of scope* forbids editing an approved
amendment-record row, so correcting it needs a further row by the assurance owner rather than an edit by an
implementation agent.

**8. `REQ-MOK-058`'s ratification reasoning has moved a second time — product owner.** `WO-MOK-016`'s
assessment 4 was ratified with "four survivors of margin at the worst seed" as its stated reasoning, and that
packet already recorded the 10,000-tick curve as a residual against it. On the corrected world the margin at
the worst seed is **2**, not 4: `post/survivors.txt` §3 measures seed 1 leaving 7 living against the floor of
5. The floor is met, so this is not a breach and assessment 4 is not reopened by it. **The decision owed:**
whether a ratification whose stated reasoning has now moved twice — once at the long horizon, once on the
corrected curve — is still the ratification the owner intends. Amending `REQ-MOK-058` upward is as available an
outcome as leaving it, and `post/survivors.txt` §4 records the wider disclosure the owner ratified against:
over the 50 unbound seeds, cells below their source's floor rose from **6 of 150 to 17 of 150** while cells
above the ceiling fell from 140 of 150 to 7 of 150.

**9. Two survivor floors are met at zero margin — product owner.** `REQ-MOK-014` and `REQ-MOK-034` both bind
eight, and both worst seeds leave exactly **eight**: `post/survivors.txt` §3, margins **0** and **0**. The
approval measurement named that absence of margin as the reason it declined the alternatives with more room,
and `REQ-MOK-060`'s amendment record carries the trade — "the surface can be pushed until the ceiling is met
or until the floors hold, and not both". **What this raises:** a requirement met at zero margin is met, and it
is also one Mokiterion away from not being. This is stated as a property of the world the ceiling was traded
for, for the owner to see beside 5a rather than after it.

---

## What this file does not do

- **It signs nothing.** Every word here was written by an implementation agent, which may prepare a record
  and may not complete one. All three record blocks are empty, and they are the owner's.
- **It takes no measurement.** Every figure is cited to the packet record that holds it, so this file cannot
  disagree with the packet; where it would, the record is right.
- **It amends nothing, and it defers nothing.** Where an outcome above is an amendment, the amendment is an
  act in the amended artifact's own record. Where a finding names a decision, naming it is not taking it.
- **It does not reopen a closed record.** `VER-MOK-016`'s assessments 1 to 4 and 7 to 11 are recorded in
  `evidence/WO-MOK-016/manual-assessment.md`. Assessment 4's figures are re-measured here because this work
  order re-measures the floor, and the two findings that follow from that are raised as residuals rather than
  as a re-assessment.
- **It is not a verification verdict, and it does not establish the ceiling or the floors.** The ceiling is
  `post/composition.txt`, the floors are `post/survivors.txt`, the divergence is `post/divergence.txt`, and
  all of them are the assurance owner's to accept. `VER-MOK-016` is the contract; the verification record is
  a further act, and this file being complete would clear one obstacle to it rather than being it.
- **It does not make the contract satisfied.** Three assessments are outstanding, and `VER-MOK-016`'s own
  bar is that "this contract is not satisfied while any remains outstanding". That is the state this file
  reports, not a state it can change.
