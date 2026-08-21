# `WO-MOK-016` manual assessments — prepared records

| Field | Value |
|---|---|
| Contract | `VER-MOK-016`, *Manual assessments* — eleven, each with its accountable role and date |
| Retention item | "the eleven manual assessments above, each with its accountable role and date" |
| What this file is | **a prepared record, not a decision.** Each section states the assessment's own words, the value decided in advance where there is one, the measured evidence it is taken against with a citation to the record that holds each figure, the outcomes available, and an **empty record block for the accountable owner to complete**. An implementation agent may prepare this and may not sign it |
| State | **2 recorded, 1 half-recorded, 1 carried to `WO-MOK-017`, 7 outstanding** |
| Baseline | `39662d13abd08e3410648d1c59ad38384f8ad2d2` |
| Candidate | `139061530f1dba72c9a20427eeaac6ce69492fb2` |
| Merge | `259859dffe1f5f856e154263c48d8d1e04808903` — where source figures are re-taken |
| Figures | **no measurement is taken here.** Every figure is cited to the packet record that holds it; where this file and that record would differ, the record is right |
| Date prepared | 2026-08-21 |

`VER-MOK-016` states the standard this file is written against: "Each of the following is an explicit
judgement recorded by the accountable role. **An unrecorded assessment is an outstanding assessment, and
this contract is not satisfied while any remains outstanding.**"

**Eight of the eleven had their subject decided in advance, on 2026-08-20, before any measurement of this
initiative existed.** The contract is equally explicit about what that does and does not settle:
"**Deciding a value in advance does not discharge the assessment.** What those eight now ask of their
owner is a ratification against the measured evidence: keeping the decided value or amending the
requirement that states it are both acceptable outcomes, and silence is not either of them." Each of the
eight therefore carries its decided value below, and the measurement beside it.

**How to complete one.** Fill the four lines of that assessment's **Record** block and nothing else. The
evidence and outcome sections are the prepared material and are not the owner's to sign around; leaving
them as they stand is what makes the completed record legible as a judgement taken against stated
evidence. A record here **amends nothing by itself**: where the outcome is an amendment, the amendment is
an act in the amended artifact's own record, and the block cites it rather than substituting for it.

The numbering is `VER-MOK-016`'s, unchanged, because assessments 8, 9 and 10 are cited by number
elsewhere in that contract.

---

## The eleven, and where each stands

| # | Subject | Owed by | State |
|---|---|---|---|
| 1 | the damage function and the striker's energy cost | product owner | **outstanding** |
| 2 | the forfeit's magnitude and its non-conservation | product owner | **outstanding** |
| 3 | the threat's magnitude | product owner | **outstanding** |
| 4 | the survivor floor of five and the lethality bound | product owner | **recorded** — `REQ-MOK-058`, 2026-08-20 |
| 5 | the composition ceiling of one half, and the per-class floor question | product owner | **carried to `WO-MOK-017`** with `REQ-MOK-060`, 2026-08-21 |
| 6 | the additivity cost | product owner | **half-recorded** — the `REQ-MOK-034` narrowing is approved; the other half's premise is false at this candidate |
| 7 | the fourth source's name | product owner | **outstanding** |
| 8 | the defender's decision rule | technical owner | **outstanding** |
| 9 | cross-agent mutation against `ARCH-MOK-001` | technical owner | **outstanding** |
| 10 | the read enumeration of oracle 6 | assurance owner | **outstanding** |
| 11 | the monotonicity band's adequacy | assurance owner | **recorded** — 2026-08-20, and it was not adequate |

The repository owner holds all three roles. That does not merge them: an assessment owed by the product
owner is taken as product owner, and the role line of each block records which one acted.

---

## 1. The damage function and the striker's energy cost

**The contract's words.** "Decided: damage is `10 + (striker.energy + striker.health) / 10`, the division
truncating, giving the range `10..=30` and four to ten strikes to kill a full-health target; the strike
costs a flat `5` `energy`. What remains is the ratification, and it has a specific question attached:
`REQ-MOK-053` records that a flat `5` is deliberately a **weak** brake on repeated striking, so the owner
ratifies having seen the measured strikes per encounter and the measured lethality, not only the survivor
total."

**The evidence.**

| Figure | Value | Record |
|---|---|---|
| rule 22 over its whole domain | 10,201 `(energy, health)` pairs collapsed onto **21** distinct damage outputs, boundaries as interval ends rather than as chosen cases | `post/resolution-tables.md` |
| the table against the engine | **24 of 24** retained rows and **1,742 of 1,742** released resolutions agree | `post/resolution-tables.md` |
| the flat `5`, separately | contradicted by **no** retained row and by **all 312** attack lines, so the two halves of the engine column cover disjoint parts of one rule | `post/resolution-tables.md` |
| strikes per encounter | **1.36** over 115 encounters; **58.3% of contacts are bloodless**; of the 48 that strike, 22 produce one or two and 5 produce six or more; longest exchange **8** | `post/branches.md` §4 |
| lethality | **21 of 156** strikes were fatal | `post/branches.md` §4 |
| survivors and combat deaths at the default density | 9, 10, 9, 9, 11 survivors and 1, 2, 2, 3, 1 combat deaths, by seed | `post/runs.md` §2 |
| combat deaths by density | 5 at `0.15`, 9 at `0.75`, 7 at `1.50`; the 54 attrition deaths at `0.15` are starvation and not the damage function | `post/runs.md` §2 |
| at 10,000 ticks | the same five seeds leave 0, 5, 2, 2 and 0, two of them by extinction — outside `REQ-MOK-058`'s 1,000-tick trigger and retained "as evidence and not as an obligation" | `post/long-horizon.md` |

**Two things the packet asks the owner to see beside those figures.** `STRIKE_BASE_DAMAGE` is guarded
asymmetrically by a `debug_assert!` at `simulation.rs:2624` that moves with it, so the suite's true oracle
coverage of the constant is **7 tests and not the 23** an upward mutation appears to show
(`post/resolution-tables.md`'s engine-side controls). And rule 24's forfeit divisor is an unnamed literal
`2` at `simulation.rs:2741` where the other three magnitudes are named constants — assessment 2's
subject, noted here because the four magnitudes were decided in one session.

**The outcomes available.** Ratify both values as decided; or amend `REQ-MOK-053`'s damage function; or
amend the strike cost to a stronger brake, which is the question the clause attaches. A finding that the
lethality is lower than intended is an amendment to `REQ-MOK-053` and not a defect in this measurement.

    Decision:
    Role:      product owner
    Date:
    Basis:

---

## 2. The forfeit's magnitude and its non-conservation

**The contract's words.** "Decided: `satiety / 2`, truncating, so `40` from a Mokiterion at `80` and `0`
from one at `1`. The owner ratifies having seen the measured frequency of two cases the arithmetic creates
— the full recipient, where the forfeit is paid and destroyed rather than returned, and the band below
`satiety` `2`, where surrender is free — and records that the destruction is intended in both."

**The evidence.**

| Figure | Value | Record |
|---|---|---|
| rule 24 over its whole domain | the domain collapsed onto **51** distinct forfeits, every boundary an interval end | `post/resolution-tables.md` |
| surrenders in the matrix | **95** | `post/branches.md` §§5–6 |
| surrenders discarding part of the forfeit | **90 of 95** | `post/branches.md` §5 |
| surrenders whose recipient was already at `satiety` `100` | **0** | `post/branches.md` §5 |
| surrenders below `satiety` `2`, where the half is nothing | **0** — the smallest forfeit measured is 11 | `post/branches.md` §§5–6 |

**Both cases the clause asks for a frequency of were not reached in whole runs.** That is the measurement
and not a gap: the ordinary discard — a recipient who cannot hold the whole forfeit — happens in 90 of 95
surrenders, while the two extremes the arithmetic admits occur zero times over 15,000 ticks of the
declared matrix. The clause asks the owner to record that the destruction is intended in both, which is a
judgement about a case the world can produce rather than about one it did.

**The outcomes available.** Record the destruction as intended in both cases; or amend `REQ-MOK-054` so
that a forfeit at a full recipient is returned rather than destroyed, or that the free band below
`satiety` `2` is closed. The unnamed literal `2` at `simulation.rs:2741` is a separate technical-owner
matter carried in the completion summary's findings, not part of this ratification.

    Decision:
    Role:      product owner
    Date:
    Basis:

---

## 3. The threat's magnitude

**The contract's words.** "Decided: a new constant of `30`, distinct from rule 12's `FEAR_INCREASE` of
`10` and not pinned to `ATTRIBUTE_MAX`. The owner ratifies having seen how often a free action with an
effect was proposed, which is the exposure `REQ-MOK-055` records about making a threat cost nothing."

**The evidence.**

| Figure | Value | Record |
|---|---|---|
| rule 23 over its whole domain | the domain collapsed onto **6** distinct outputs | `post/resolution-tables.md` |
| threats proposed, and applied, across the fifteen traced cells | **620 / 620** — a free action with an effect, proposed 620 times in 118,201 decisions | `post/runs.md` §3 |
| threats by density | 379 at `0.15`, 128 at `0.75`, 113 at `1.50`; the extinction cells are where threatening concentrates | `post/runs.md` §2 |
| `threat_resolved` events across **all thirty** cells | **1,240**, and **every one reports `increase:0`** | `post/branches.md` |
| the shape of every one of them | `result=target:M04,increase:0,target_fear:100->100` — the target's `fear` is already saturated when the threat lands | `post/branches.md` |

**So the constant `30` did not move a value anywhere in the declared matrix.** `THREAT_FEAR_INCREASE` is
behaviourally inert at the measured `fear` levels, not because the increase is small but because the
targets are already at `100` by the time a threat resolves. The exposure `REQ-MOK-055` records — that a
threat costs nothing — is measured at 620 proposals; the effect the constant was chosen for is measured at
zero. Both belong in the same ratification, which is why this file states them together.

**The outcomes available.** Ratify `30` as decided; or amend `REQ-MOK-055`'s constant; or amend the rule
so that a threat's effect is reachable, which is the finding rather than the constant. Recording the
inertness as accepted is itself a decision the clause admits.

    Decision:
    Role:      product owner
    Date:
    Basis:

---

## 4. The survivor floor of five and the lethality bound — **recorded**

**The contract's words.** "Decided at five rather than six, pre-measurement, because of the damage
decision taken in the same session. `REQ-MOK-058` records that five sits exactly on the line its own
six-argument drew, so this ratification is doing real work rather than confirming a comfortable number: a
curve that clusters at five or six is itself a finding about the damage function, and amending
`REQ-MOK-058` upward is as available an outcome as ratifying it."

**The evidence it was taken on.** `post/runs.md` §2's default-density rows: survivors **9, 10, 9, 9 and
11** and combat deaths **1, 2, 2, 3 and 1**, so no seed is below the floor and no seed is bloodless. The
same five pairs arrive by a second path — `mokiterions-core/tests/viability.rs`'s in-process oracle counts
`target_died:yes` while `analysis/runs.py` reads the released binary's stdout — and the two agree.

**What arrived afterwards, and does not reopen it.** `post/long-horizon.md` measures the same five seeds
at 10,000 ticks leaving **0, 5, 2, 2 and 0**, two by extinction before the horizon. `REQ-MOK-058` binds at
1,000 ticks and is met there, so this is outside its trigger and is retained as evidence. What it bears on
is the ratification's stated *reasoning* — "four survivors of margin at the worst seed" is a property of
the 1,000-tick curve alone — and it is carried as a residual for the product owner rather than as a
breach.

    Decision:  ratified unchanged at five, with the survival-first clause corrected
    Role:      product owner
    Date:      2026-08-20
    Basis:     `REQ-MOK-058`'s second amendment row, on the first measured curve —
               `post/runs.md` §2 and `mokiterions-core/tests/viability.rs`'s `DECLARED_FLOORS`

---

## 5. The composition ceiling of one half — **carried to `WO-MOK-017`**

**The contract's words.** "The composition ceiling of one half, by the product owner, on the measured
corrected composition, together with the judgement on whether a per-class floor is also wanted. Ratified
in advance at one half against `60%` and `40%`, for the `17` points of headroom above the balanced initial
third that it leaves; the ratification here is against the measured curve rather than against the
reasoning."

**Why there is no record block.** This assessment is `REQ-MOK-060`'s, and the *Scope amendment of
2026-08-21* descoped `REQ-MOK-060` from this work order to `WO-MOK-017`. The assessment travels with the
requirement. It is named here because `VER-MOK-016` numbers it 5 and the numbering is preserved.

**The measurement is taken anyway, so the follow-on inherits the curve and not just the question.**

| Figure | Value | Record |
|---|---|---|
| default-density territory evaluations breaching the ceiling | **27 of 30**, `high` over the line in every one of the 27 | `post/composition.md` §6 |
| runs carrying a breaching territory | **14 of 15** | `post/composition.md` §6 |
| the widest class's range | **37.3% to 81.6%** | `post/composition.md` §6 |
| the ceiling's stated headroom, measured | **17.2** points, against a drift of **31.3** — about 1.8× | `post/composition.md` |
| the pre-change side | **17 of the 20** pre-change evaluations breach it too, and all 20 verdicts are identical at both commits | `post/composition.md`, completion summary item 8 |

So a correction has to remove half the drift rather than shave it, and the breach is not this change's
doing.

**A defect in the clause itself, for `WO-MOK-017` to inherit with it.** "On the measured corrected
composition" has **no subject at this candidate**: there is no corrected composition, `REQ-MOK-060` being
unimplemented. The wording needs amending wherever the assessment is re-declared.

---

## 6. The additivity cost — **half-recorded**

**The contract's words.** "`reference` and `individual` outcomes move. The owner records that this is
accepted, having seen the characterized divergence, and approves the narrowing of `REQ-MOK-034`'s
frozen-outcomes clause to `baseline` alone."

**The half that is taken.** `REQ-MOK-034`'s amendment row of 2026-08-20 narrows the frozen-outcome
constraint "from 'the reference or baseline source' to `baseline` alone", by the product owner, in the
same act as `WO-MOK-016` — earlier than its ordering required, since it had only to precede the change
that moves `reference`. It is **taken and unused**: a permission granted ahead of its use.

**The half whose premise is false at this candidate.** "Having seen the characterized divergence" presumes
a divergence. `post/byte-identity.txt` measures **all 60** `reference` and `individual` cells
byte-identical between the two commits — the same finding `post/composition.md` reaches from the other
direction, with all 45 shared rule 18 summaries identical — because the change that would have moved them
is `REQ-MOK-060`, which is unimplemented and now descoped. **There is no divergence to characterize**, and
the oracle that asks for one reads `RESULT: MIXED` for exactly this reason. The assessment becomes
assessable when `WO-MOK-017` moves those outcomes.

    Decision:  the narrowing of `REQ-MOK-034` to `baseline` alone is approved
    Role:      product owner
    Date:      2026-08-20
    Basis:     `REQ-MOK-034`'s amendment row, in the single act that also approved `WO-MOK-016`

    Decision:
    Role:      product owner
    Date:
    Basis:     the additivity cost itself — outstanding, and its premise is false at this
               candidate: `post/byte-identity.txt` measures 60 of 60 cells identical

---

## 7. The fourth source's name

**The contract's words.** "Decided: `social`, from four candidates. Because it appears in `--help`, in the
invalid-value diagnostic and in every captured stream, the assessment that remains is not the choice but
the check that the decided name landed in all three places identically and that no captured evidence
carries an earlier working name."

**The three places, read at the merge tip.**

| Surface | Where | Asserted by |
|---|---|---|
| `--help`, twice | `mokiterions-core/src/cli.rs:12` (usage line) and `:20` (option entry), both `<baseline\|reference\|individual\|social>` | `mokiterions-tui/tests/options.rs :: the_usage_text_advertises_every_policy_the_engine_accepts`, now over four policies, and `mokiterions-core/tests/cli.rs :: the_entries_state_the_constraints_that_decide_validity` |
| the invalid-value diagnostic | `mokiterions-core/src/cli.rs:102` — "invalid --policy value: {value}; expected baseline, reference, individual, or social" | **no test asserts this string.** `parse(["--policy", "random"]).is_err()` asserts the rejection and not the message, so this surface is read from source here rather than pinned by an oracle |
| every captured stream | 30 cell names under `social` in `post/social-manifest.txt`, every cell exiting `0`, and the `--policy social` invocation in `capture-social.sh` | the manifest's digests |

**The closed-set form of the absence clause.** `Policy` has exactly four variants at the merge tip —
`Baseline`, `Reference`, `Individual`, `Social` — and the policy token of every one of the **210** cell
names across the three manifests is one of the four: 30 `baseline`, 30 `reference` and 30 `individual` in
each 90-cell manifest, and 30 `social`. No artifact in the repository enumerates the four candidate names
the decision chose between, so "no captured evidence carries an earlier working name" cannot be searched
for by name; it is discharged in the only form available, which is that the set of names that do appear is
closed and is the shipped four.

**The outcomes available.** Confirm that the decided name landed identically in all three places; or name
the surface that is wrong. The untested diagnostic string is a technical-owner matter the confirmation may
note in passing — one assertion would pin it — and is not itself a product-owner decision.

    Decision:
    Role:      product owner
    Date:
    Basis:

---

## 8. The defender's decision rule

**The contract's words.** "Decided: the six-branch ordering of `REQ-MOK-057`, survival first and then a
perceived meal before society, with the defender answering `surrender` at `fear` `60`, `retreat` at `30`
and `fight` below `30`, and the aggressor engaging below `95`. It was a five-branch ordering with the gate
at `30` as first approved; the assessment now also records that the amendment of 2026-08-20 was taken on
measurement rather than on preference, and that `SPEC-MOK-001` rule 12 was left as Phase 2 approved it.
The owner records whether the measured branch distribution is degenerate — every defender surrendering, or
every defender fighting, or a social branch that never fires — and if it is, that finding is the
deliverable and not a failure, per `INT-MOK-010`. `REQ-MOK-057` leaves one thing genuinely open for this
assessment to inform: whether branch 1 should be able to decline to answer at all."

**The measured distribution.** `post/branches.md` §§2–3, over 118,201 decisions in the fifteen traced
cells:

| Branch | Fired | What it is |
|---|---:|---|
| 1 | **135** | answer the first unanswered attack |
| 2 | **7,579** | survival — rule 19's case 1 |
| 3 | **58,441** | a perceived meal, rule 19's case 3 hoisted by the amendment |
| 4 | **763** | engage a perceived Mokiterion below the threshold |
| 5 | **13,308** | approach company |
| 6 | **37,975** | delegate to rule 19, which is also this source's entire entropy consumption |

The answer branch's three choices are **13 `fight` / 27 `retreat` / 95 `surrender`** of the 135.

**Not degenerate, on all three of the clause's shapes.** Not every defender surrenders — 40 of the 135
answers are not a surrender. Not every defender fights — 13 are. And no branch never fires: the scarcest
is branch 4 at 763, and branch 1, the rarest by construction, fires 135 times. So the `INT-MOK-010`
finding-instead-of-failure path is available and, on this measurement, not needed.

**The amendment was taken on measurement.** `escalation.md` records seventeen variants measured across
three levers and three packages put to the owner, with the chosen one leaving `SPEC-MOK-001` rule 12 as
Phase 2 approved it; `VER-MOK-016`'s and `SPEC-MOK-001`'s amendment rows of 2026-08-20 are the acts.

**The open question, with the evidence it asked for.** Whether branch 1 should be able to decline to
answer turns on the cost rule 26 accepts — an answer whose target died at a third Mokiterion's hands, or a
`fight` target that moved out of contact. `post/runs.md` §5 measures it: of the 135 answers, **0** named a
Mokiterion that was not living, **0** were rejected on any ground, and there were **0** rejections on
`target_dead`, `target_unknown` or `out_of_contact` from any branch. The accepted cost did not materialize
anywhere in the declared matrix — recorded as **not observed**, which is a different claim from **cannot
happen**, since `validate_targeted` returns `target_dead` and constructed cases exercise it.

**The outcomes available.** Ratify the six-branch ordering and the four thresholds; or amend
`REQ-MOK-057`; or record a degeneracy finding under `INT-MOK-010`; and, separately, decide whether branch
1 may decline to answer.

    Decision:
    Role:      technical owner
    Date:
    Basis:

---

## 9. Cross-agent mutation

**The contract's words.** "Decided: a recorded confirmation that `ARCH-MOK-001` is satisfied unchanged,
rather than an `ADR` deciding the pattern — the mutation stays within the engine component, crosses no
boundary `ARCH-MOK-001` draws, and adds no dependency. The confirmation itself is made in review against
the implemented code and is outstanding until recorded; if the implementation turns out to need a boundary
moved, the decision reverts to an `ADR` and this assessment says so."

**The evidence.**

| Claim | Measurement | Record |
|---|---|---|
| every path that writes a second Mokiterion's state is named | **two** `fear` writers, **five** paths writing a second Mokiterion, **three** functions — enumerated, not sampled | `post/reads.md` §7 |
| the mutation stays inside the engine component | the engine's public surface is enumerated at both commits — 172 entries to **186** — and rule 6's ten prohibited names are checked absent; the 16 surface differences are the approved `SPEC-MOK-002` growth, 14 added and 2 changed in place | `post/interface.txt` §§4, 6 |
| no dependency added | the declared dependency set compared against the resolved graph in both directions at exit `0`, with the engine at **0 external crates** on all three declared targets; `Cargo.lock` byte-unchanged across the merge | `post/merge-recheck.txt` §6, `post/gates.txt` |
| the observer side is unchanged in kind | the authority mapping gains rows and no wildcard; `mokiterions-tui/src/state.rs` has **zero changed lines** | `post/observer.md`, completion summary item 1.1 |

**What this file cannot do.** The confirmation is "made in review against the implemented code". This
record prepares the material the review reads; the review and its recording are the technical owner's.

**The outcomes available.** Confirm `ARCH-MOK-001` satisfied unchanged; or find that a boundary must move,
in which case the decision reverts to an `ADR` and this block says so.

    Decision:
    Role:      technical owner
    Date:
    Basis:

---

## 10. The read enumeration of oracle 6

**The contract's words.** "The owner confirms that the enumeration is complete — that no rule, source or
validation path is missing from it — because an enumeration's value is entirely in its completeness, and
no automated check can establish that."

**The evidence.** `post/reads.md` §§2, 4 and 6 classify **48** reads, every one of them, with none
omitted: each is either a read of a named individual — the acting agent, or an individual the rule names —
or one of the **seven** aggregate reads of a set, and §5 gives the reason each of the seven is outside the
obligation, the extinction test among them. The verdict recorded there is that `REQ-MOK-059` is met.

**Why the assessment exists.** The reader can show that every read it found is classified; it cannot show
that it found every read. That is the completeness claim, and it is the one thing here that a run cannot
establish.

**The outcomes available.** Confirm the enumeration complete; or name the rule, source or validation path
that is missing from it, which is a finding against `post/reads.md` and not against `REQ-MOK-059`.

    Decision:
    Role:      assurance owner
    Date:
    Basis:

---

## 11. The monotonicity band's adequacy — **recorded**

**The contract's words.** "The owner records that the band is a coarse tripwire, what it would and would
not catch, and that the identifier-symmetry test is what carries the mechanism claim." Then, in the
contract itself: "**Assessed 2026-08-20, and it was not adequate.**"

**The evidence it was taken on.** `identifier.md`: 9,194 resolved strikes, 45,791 threats, 6,033
surrenders and 2,712 deaths of which 1,274 were struck to death, over **12,000 identifier-runs** on 1,000
seeds. The band failed at the candidate on a real advantage it could not characterize; correcting only its
covariate would have made it pass by `0.007` while the same statistic reads `+1.000` on a thousand seeds.
The replacement's part two evaluates the turn-position bound at **`1.0819`** against `< 1.25`, passing
with **`0.168`** of margin, and both rank correlations are recorded while bounding nothing.
`escalation.md` §11 is the evidence this assessment was waiting for, and the identifier-symmetry test
still carries the mechanism claim.

**Its own residuals, which the record does not close.** The rule 25 ablation is not reproducible from a
retained script, and `1.25` is not established as the right bound — only where it came from and that it
holds at `1.082` (`identifier.md` §§7–8). `identifier.md` §8 also states `INT-MOK-010`'s risk in the
opposite direction from the interface record, which is a technical-owner correction carried in the
completion summary's findings.

    Decision:  not adequate. The band is replaced by the two-part oracle 5 — a tripwire where five
               seeds support it, and a survival-consequence bound on a set large enough to measure
               it — with the correlations recorded and bounding nothing
    Role:      assurance owner
    Date:      2026-08-20
    Basis:     `VER-MOK-016` assessment 11's own recorded text, on `identifier.md` and
               `escalation.md` §11

---

## What this file does not do

- **It signs nothing.** Seven assessments are outstanding and one is half-outstanding; those eight blocks
  are empty because an implementation agent may prepare a record and may not complete it.
- **It takes no measurement.** Every figure is cited to the packet record that holds it, so this file
  cannot disagree with the packet; where it would, the record is right.
- **It amends nothing.** Where an outcome above is an amendment, the amendment is an act in the amended
  artifact's own record; a completed block here cites that act rather than being it.
- **It is not a verification verdict.** `VER-MOK-016` is the contract and `VREC-MOK-016` is the record.
  `VREC-MOK-016` **cannot reach `verified`** while any of these assessments is unrecorded, and it names
  the outstanding ones in its residual section for that reason.
