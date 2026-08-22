# `WO-MOK-017` manual assessments — prepared records

| Field | Value |
|---|---|
| Contract | `VER-MOK-016`, *Manual assessments* — assessment 5 as realigned on 2026-08-21, and the second half of assessment 6, carried here by `WO-MOK-016`'s own record |
| Retention item | "The manual assessments this work order reaches, each with its accountable role and date" (`WO-MOK-017` *Evidence to record*); and *Required verification*: "Manual assessment 5 — the ceiling's ratification or amendment on the measured curve — travels with them" |
| What this file is | **prepared material, and now the record taken on it.** Each section states the assessment's own words, the measured evidence it is to be taken against with a citation to the record that holds each figure, the outcomes available, and a record block. Everything outside the blocks was written by an implementation agent, which may prepare a record and may not sign one; **every block was completed by the accountable owner**, on 2026-08-22 |
| State | **3 recorded, none outstanding.** Assessment 5 in both of its realigned halves, and assessment 6's second half, all three recorded 2026-08-22. `VER-MOK-016`'s assessment 4 is re-measured here and is **not** reopened; nine findings were raised for their owners, and **eight of the nine now carry a disposition** taken the same day. Finding 7 is the assurance owner's and is carried, undecided, to the verification decision |
| Baseline | `pre/COMMIT.txt` — `1ba5a3aabe775c9cdee29a04b399af3bb82dde90`, this work order's own approval commit, no source file changed |
| Candidate | `post/COMMIT.txt` — the commit the packet's post-change capture was taken from |
| Merge | `ae2e44f2382fe89f2daf78a8e8aca37febb8bd0f` — `master` merged into the branch, accounted for in `merge/`, where the source figures are re-taken. `master` has since merged the branch, at `f7b1c452039dc2f03010ca8b8cc81e73c54727c0` (PR #39). **No figure in this file was re-measured for either merge**; `merge/README.md` holds the byte-identity licence the carry-forward rests on |
| Figures | **no measurement is taken here.** Every figure is cited to the packet record that holds it; where this file and that record would differ, the record is right |
| Date prepared | 2026-08-21 |
| Date recorded | 2026-08-22 — all three blocks, and all nine finding dispositions |

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
the owner. They were therefore outstanding when this file was prepared, and not discharged by the amendment
that preceded them; they are recorded below, on 2026-08-22.

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
| 5a | three fifths met on the shipped build, with the margin the amendment claimed | product owner | **recorded** — ratified on the shipped build, 2026-08-22 |
| 5b | whether the corrected composition warrants a per-class floor | product owner | **recorded** — declined to bind a floor, 2026-08-22 |
| 6, second half | the additivity cost, "having seen the characterized divergence" | product owner | **recorded** — the cost accepted, 2026-08-22 |
| 4 | the survivor floor of five and the lethality bound | product owner | **recorded 2026-08-20, and not reopened here** — re-measured as an oracle; its stated reasoning moved, which is finding 8 |
| 1, 2, 3, 7, 8, 9, 10, 11 | `WO-MOK-016`'s combat assessments | product, technical and assurance owners | **recorded 2026-08-20 or 2026-08-21**, outside this work order's scope, and untouched by it |

The repository owner holds all three roles. That does not merge them: an assessment owed by the product owner
is taken as product owner, and the role line of each block records which one acted.

---

## 5a. Three fifths on the shipped build — **recorded**

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

    Record — RECORDED
    Decision:  the result is ratified. Three fifths is met on the shipped build and
               `REQ-MOK-060` stands unamended at three fifths. The margin is accepted as a
               margin and not read as sitting on the ceiling, and the trade the ceiling was
               met by — the thinner larder, 9 of 15 obligated runs leaving fewer living, and
               the two floors at zero margin — is ratified as the trade intended
    Role:      product owner
    Date:      2026-08-22
    Basis:     the two figures the clause makes the ratification conditional on, both from
               `post/composition.txt` §6 — 0 of 30 obligated evaluations breach three fifths,
               and the worst is social seed 1 territory B at 54.1% of 61, which is 5.9 points
               of margin and reproduces the amendment's predicted 54.1% to the tenth of a
               point on the shipped build. The distribution's top five rows sitting above one
               half (§7) is the shape of a corrected world and not of one at its limit: the
               same section measures 20 of 30 over three fifths and a worst of 81.6% before
               the correction. The price is read as stated rather than discounted —
               `post/survivors.txt` §4's 788 more consumed and 85 fewer standing, and §3's
               margins 0, 0 and 2. The two zero-margin floors are accepted here, which is
               finding 9's disposition and is recorded beside this rather than after it.
               `REQ-MOK-058`'s floor of five is left at five, which is finding 8's.

---

## 5b. The per-class floor — **recorded**

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

    Record — RECORDED
    Decision:  no per-class floor is bound. The decision is declined, not deferred a third
               time: the question is answered on the measurement it was reserved for, and it
               is not open again on this evidence
    Role:      product owner
    Date:      2026-08-22
    Basis:     the drift `REQ-MOK-060` was written about is measured one-directional and the
               correction moved it the way a floor would want — `post/composition.txt` §9's
               narrowest class rose from 4.1% to 14.3% at the worst evaluation, 0 of 30
               evaluations leave a class holding nothing, and in 27 of 30 the narrowest class
               is `low`, which is where the ceiling's own mechanism puts it. A floor bound
               now would be bound against a figure no measurement is approaching, and the
               ceiling is what the obligation is stated on. **What this record does not do:**
               `REQ-MOK-060`'s *Open decisions* third bullet still reads "remains deferred",
               and closing it is an act in that requirement's own record. This block cites
               that act as owed rather than substituting for it.

---

## 6, second half. The additivity cost — **recorded**

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

    Record — RECORDED
    Decision:  the additivity cost is accepted, having seen the characterized divergence, and
               the permission the first half granted on 2026-08-20 is now used. `REQ-MOK-034`'s
               frozen-outcomes clause is narrowed to `baseline` alone as already approved, and
               nothing further is amended by this record
    Role:      product owner
    Date:      2026-08-22
    Basis:     the divergence is characterized rather than merely observed, which is the form
               the clause asks acceptance in — `post/divergence.txt` §3 attributes every one of
               the 90 diverging cells, 57 direct, 9 through the shared stream, 24 through the
               traced sibling and **0 unattributed**, and §4's refutation check finds 0 cells
               diverging before their own band entry and 0 cells whose `waste_tolerance`
               assignment moved. What the clause still freezes is intact: `post/byte-identity.txt`
               measures 30 of 30 `baseline` cells byte-identical with equal exit codes, and
               `post/entropy.txt` §§1–3 measure the corrected condition consuming no entropy at
               all. Route R2 is accepted on the concession §9 states in its own words — the set
               of Mokiterions whose behavior moved is larger than the set whose answer the
               condition changed — because that is a property of a shared stream and not of this
               correction. The cost is read at `post/survivors.txt` §4: 9 of 15 runs fewer
               living, 2 more, 4 unmoved, consumption 26,136 → 26,924, standing 1,680 → 1,595.
               Declining would put `REQ-MOK-060` in question rather than this clause, and
               `REQ-MOK-060` is ratified above.

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

**The dispositions of 2026-08-22.** Eight of the nine were taken on the same day as the three assessment
blocks above, in the same session, each in the role the finding names. A **Disposition** line closes each
finding below. The dispositions follow the same rule the blocks do: **none of them amends anything by
itself.** Where a disposition defers an amendment to a later work order, that work order is the act, and
naming it here is not writing it. Where a disposition leaves a value where it stands, that is a decision
and not a silence, which is the reason this section exists. Finding 7 is the assurance owner's, it is
**not taken here**, and the reason is stated under it.

**1. The per-run draw total is not derivable at this candidate — technical owner.** `VER-MOK-016`'s retention
item asks for "the total draw count per run per source at matched seeds". `post/entropy.txt` §5 records that
it cannot be produced and why, rather than substituting a proxy: a counter on the released surface is barred by
this work order's *Out of scope* ("any new attribute, event, interface item or instrumentation"); recovering
the count by replay means reimplementing every draw-consuming path, because occupancy rejections are invisible
in the stream; and a golden-value test would pin an arbitrary number no requirement states. What stands in its
place is measured and is listed there. **The decision owed:** whether a draw counter belongs on the released
surface, or whether a work order may carry a replay of the whole engine as evidence. `WO-MOK-016` left a
comparable clause unwritable and recorded it; this is the same discipline on a narrower gap.

**Disposition, 2026-08-22 — technical owner: what stands in its place is accepted, and neither alternative
is authorized.** No draw counter goes on the released surface, and no work order may carry a replay of the
engine as evidence. Both were declined on the same ground: the released surface is `SPEC-MOK-002` rules 5
and 6's closed interface and a counter on it would be instrumentation the specifications do not state, while
a replay is a second implementation of every draw-consuming path, which is evidence that can disagree with
the engine rather than measure it. `post/entropy.txt` §5's substitutes are what the retention item is
satisfied by here — the 15 identical initialization draw totals, the 120 identical prefixes, and the
per-cell first-divergence attribution — and the gap is left recorded rather than closed. `VER-MOK-016`'s
retention item is not amended: the item asks for a figure that is not derivable at this candidate, and
recording why is the discipline `WO-MOK-016` set.

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

**Disposition, 2026-08-22 — technical and assurance owners: it should, and the scenario is to be constructed
directly.** The decision is taken; the amendment is not written here. `VER-MOK-013`'s scenario is to be
amended so that a declining subject is constructed through the existing test hooks rather than searched for
in a live run, and the assertions and the threshold of thirty are to survive the amendment unchanged — the
finding is about how the subject is reached, not about what is asserted of it. **The act owed:** an
amendment row on `VER-MOK-013` by the assurance owner and the scenario change under a later work order. The
`--policy social` selection this work order applied stands in the meantime, and it is not the answer; it is
the second model dependency the finding names, carried until the construction replaces it.

**3. `VER-MOK-005`'s dead-selection scenario, the same question — technical and assurance owners.**
`a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour` searches a 700-tick run for a
dead identifier with living identifiers on both sides. `post/dead-neighbours.txt` measures why the pinned
version broke: the first death moved from `M05` at tick 604, interior, to **`M01` at tick 314**, the lowest
identifier, from which BackTab wraps and `assert!(backward < dead)` cannot hold. The amended search settles at
tick 399 at the candidate, inside the horizon at both commits. The state is constructible in one line through
the existing test hooks. **The decision owed:** whether it should be, which is a `VER-MOK-005` amendment — the
test discharges `REQ-MOK-021`'s "Selected Mokiterion dies" and `REQ-MOK-023`'s "Selection cycles living
Mokiterions only", and neither states how the state is reached.

**Disposition, 2026-08-22 — technical and assurance owners: it should, on the same ground as finding 2, and
the two travel together.** `VER-MOK-005`'s scenario is to be amended so the dead-with-living-neighbours state
is constructed through the existing test hooks, which the finding measures as one line. Neither requirement
states how the state is reached, so constructing it asserts exactly what is required and nothing about the
nutrition model. **The act owed:** an amendment row on `VER-MOK-005` by the assurance owner and the scenario
change under the same later work order as finding 2's. The amended search at tick 399 stands in the meantime
and is likewise not the answer.

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

**Disposition, 2026-08-22 — product owner: measure first, and state no requirement until the measurement
exists.** The ordering question is answered and the substantive one is not: the correction's effect past tick
1,000 is to be measured under a later work order, and whether a long-horizon requirement is stated is decided
**on that measurement** and not before it. The ground is the one the reader states — the figures that made
high-class accumulation look fatal over 10,000 turns are the figures the correction is most likely to have
moved, so a requirement written now would be written against a curve nobody has seen. The three figures the
measurement is owed against are named: the extinction tick of **9,154**, the 10,000-turn comparison leaving 1
to 5 survivors on four of five seeds, and the tolerance-range measurement that cut the range at 40. **What
this leaves standing, deliberately:** all nine figures keep the not-re-measured flag their own documents
carry. Neither `SIMULATION_RULES.md` nor `docs/ROADMAP.md` is corrected here: a flag saying a figure was not
re-measured is honest, and a replacement figure taken from no measurement would not be. The three
long-horizon figures wait on the measurement above; the rest wait with them so that the two documents are
corrected once, against one curve, rather than in two passes against two.

**5. The `--help` prose no longer distinguishes `individual` from `reference` — technical owner.**
`mokiterions-core/src/cli.rs:33` says the individual policy "seeks and consumes as the reference policy does,
except that each Mokiterion also accepts food it would partly waste, in proportion to its own waste
tolerance". After the correction the reference rule accepts partly-wasted food too, by rule 5's own allowance,
so the distinguishing clause is now the *amount* of waste admitted and not the fact of it. **It was
deliberately not edited.** `SPEC-MOK-001` *Help output* requires the prose to describe all four sources and to
state no default or constraint, and fixes no wording, so the text is the technical owner's and amending it is
outside this work order's expected change surface. It is not wrong about behavior — `individual` does admit
more waste — but a reader takes it as the boundary between the two sources, and that boundary moved.

**Disposition, 2026-08-22 — technical owner: the prose is to be amended, in a later work order.** The finding
is accepted as stated: the text is not wrong about behavior, and it is misleading about the boundary, which is
enough to amend it. It is not amended here, and the reason is the one the finding gives — `SPEC-MOK-001`'s
*Help output* fixes no wording, so the wording is the technical owner's, and `WO-MOK-017`'s expected change
surface does not include it. **The act owed:** a later work order amending the `individual` sentence in
`mokiterions-core/src/cli.rs`'s `USAGE`, so that the distinguishing clause is the *amount* of waste admitted
rather than the fact of it. No specification amendment is needed for it, and `tests/cli.rs`'s help oracles are
written against the text's structure rather than its wording — all four sources named, no default or density
constraint restated in the prose — so an amendment that keeps those properties needs no test to move with it.

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

**Disposition, 2026-08-22 — product owner: the driver is what wants amending, and it is amended in a later
work order.** `REQ-MOK-055`'s constant of `30` stays where assessment 3 ratified it on 2026-08-21, and that
ratification is not reopened: it was taken in the knowledge of the inertness, on the ground that the cause is
saturation and not magnitude, and this work order's measurement confirms rather than disturbs it — 2,850
`threat_resolved` records at **both** commits, one of which moved fear at all, and by 5 through the clamp.
What is accepted is that a threat whose effect is unreachable in 2,849 of 2,850 cases is not the behavior
`REQ-MOK-055` was written to obtain, and that rule 12's `+10 / −5` driver is where that is fixed. **The act
owed:** a later work order amending the fear driver, taken against the exposure `post/docs-figures.txt` §5
measures — 34% of 160,921 creature-turns at fear 100 and 52% above 0 — with `REQ-MOK-055`'s constant left to
be re-read once the driver no longer saturates. Nothing is amended by this disposition, and rule 23's gate is
confirmed to be working as written.

**7. An approved amendment row in `VER-MOK-016` names the wrong item — assurance owner.** The realignment row
of 2026-08-21 reads "**Manual assessment 7** carries it on both of its halves." Manual assessment 7 is the
fourth source's name, it has no halves, and it is untouched. The item that was amended on both of its halves
is **acceptance scenario 7** (`VER-MOK-016` lines 385–386: three fifths on each seed, then the pre-change
measurement exceeding three fifths). The substance landed correctly in the artifact and nothing depends on the
label. **Why it is a finding and not a fix:** `WO-MOK-017`'s *Out of scope* forbids editing an approved
amendment-record row, so correcting it needs a further row by the assurance owner rather than an edit by an
implementation agent.

**Not disposed, 2026-08-22 — carried to the verification decision.** This is the one finding of the nine with
no disposition here, and its absence is deliberate rather than an omission. It is owed by the **assurance
owner**, and the assurance owner's acts on this work order are the reading of the two retained `RESULT: FAIL`
reports and the `ready` → `verified` transition of the verification record; a mislabelled row in `VER-MOK-016`
belongs with those and not with the product and technical owners' session of 2026-08-22. Nothing waits on it:
`VER-MOK-016` is outside `evidence/WO-MOK-017/`, so the row can be added without staling this packet or the
record that declares it, and the finding's own text establishes that the substance landed correctly and that
nothing depends on the label. **The act owed:** a further amendment row on `VER-MOK-016` by the assurance
owner, naming acceptance scenario 7 as the item the realignment of 2026-08-21 amended on both of its halves,
and leaving that row's words as approved.

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

**Disposition, 2026-08-22 — product owner: the floor stays at five, and the ratification is still the one
intended.** `REQ-MOK-058` is not amended. The margin at the worst seed is now 2 and not 4, and that is
recorded as the correct figure — the reasoning cited in 2026-08-20's ratification described the uncorrected
curve, and it described it accurately at the time. What the amendment question turns on is what the floor is
*for*, which is a bound on how lethal the world may be, and a bound of five met with two survivors of margin
is met. Raising the floor was declined on the trade `REQ-MOK-060`'s amendment record already states in terms —
"the surface can be pushed until the ceiling is met or until the floors hold, and not both" — so a floor
raised now would be a floor raised against the ceiling ratified in 5a above, on the same evidence, in the same
session. The wider disclosure is read and accepted with it: `post/survivors.txt` §4's rise from 6 of 150 to 17
of 150 cells below their source's floor over the 50 unbound seeds is disclosed about seeds no requirement
binds, beside a fall from 140 of 150 to 7 of 150 above the ceiling on the same cells. **What this does not
settle:** whether the floor holds past tick 1,000, which is finding 4's measurement and not this one's. The
ratification's reasoning has now moved twice, and the record of it is here rather than in `REQ-MOK-058`,
because a decision to leave a requirement unamended is not a row in it.

**9. Two survivor floors are met at zero margin — product owner.** `REQ-MOK-014` and `REQ-MOK-034` both bind
eight, and both worst seeds leave exactly **eight**: `post/survivors.txt` §3, margins **0** and **0**. The
approval measurement named that absence of margin as the reason it declined the alternatives with more room,
and `REQ-MOK-060`'s amendment record carries the trade — "the surface can be pushed until the ceiling is met
or until the floors hold, and not both". **What this raises:** a requirement met at zero margin is met, and it
is also one Mokiterion away from not being. This is stated as a property of the world the ceiling was traded
for, for the owner to see beside 5a rather than after it.

**Disposition, 2026-08-22 — product owner: accepted, and accepted beside 5a as this finding asked.** Neither
`REQ-MOK-014` nor `REQ-MOK-034` is amended, and the absence of margin is ratified as part of the trade rather
than noticed after it: 5a's record above names the two floors at zero margin among the things it ratifies, and
this disposition is that same act read from the floors' side. A requirement met at zero margin is met, and the
alternatives were measured rather than argued — `REQ-MOK-060`'s amendment record declined relaxing these two
floors to six on the ground that a measured floor is what a correction is held against, and declined the
points with more room because no point that holds all three floors meets one half at all. **What is accepted
with it:** that these two floors are now the binding constraint on any further movement of the correction
surface, so a later work order that touches that surface re-measures them first. That is a consequence of this
disposition and not a further decision.

---

## What this file does not do

- **An implementation agent signed nothing.** Everything outside the record blocks and the disposition
  paragraphs was written by an implementation agent, which may prepare a record and may not complete one. The
  three record blocks and the eight dispositions were completed by the accountable owner on 2026-08-22, in the
  role each names.
- **It takes no measurement.** Every figure is cited to the packet record that holds it, so this file cannot
  disagree with the packet; where it would, the record is right.
- **It amends nothing.** Every one of the eleven decisions above leaves its artifact unamended or defers the
  amendment to that artifact's own record: `REQ-MOK-060` is ratified unamended, `REQ-MOK-058`, `REQ-MOK-014`
  and `REQ-MOK-034` are left where they stand, and `REQ-MOK-034`'s narrowing was already approved on
  2026-08-20. **Three acts are owed elsewhere and are named where they arise, not taken here:** closing
  `REQ-MOK-060`'s *Open decisions* third bullet, which 5b's record leaves reading "remains deferred"; the
  amendment rows on `VER-MOK-013` and `VER-MOK-005` that findings 2 and 3 decide; and the further row on
  `VER-MOK-016` that finding 7 needs and that no one has taken. Five pieces of work are deferred to later work
  orders, by findings 2, 3, 4, 5 and 6, and naming a later work order is not writing one.
- **It does not reopen a closed record.** `VER-MOK-016`'s assessments 1 to 4 and 7 to 11 are recorded in
  `evidence/WO-MOK-016/manual-assessment.md`. Assessment 4's figures are re-measured here because this work
  order re-measures the floor, and the two findings that follow from that are raised as residuals rather than
  as a re-assessment.
- **It is not a verification verdict, and it does not establish the ceiling or the floors.** The ceiling is
  `post/composition.txt`, the floors are `post/survivors.txt`, the divergence is `post/divergence.txt`, and
  all of them are the assurance owner's to accept. `VER-MOK-016` is the contract; the verification record is
  a further act, and this file being complete would clear one obstacle to it rather than being it.
- **It clears one bar and it is not the verdict.** `VER-MOK-016`'s bar is that "this contract is not
  satisfied while any remains outstanding", and with these three records no assessment of that contract
  remains outstanding — the other eight are recorded in `evidence/WO-MOK-016/manual-assessment.md`. That
  clears the outstanding-assessment obstacle and nothing else: the contract's oracles, acceptance scenarios
  and retained evidence are separate items, two of the packet's reports read `RESULT: FAIL` and have not been
  read by the assurance owner, and the verification record is a further act by that owner. **This file being
  complete is a precondition of that act and is not it.**
