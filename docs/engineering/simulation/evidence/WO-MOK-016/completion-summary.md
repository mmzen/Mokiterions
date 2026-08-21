# `WO-MOK-016` completion summary

| Field | Value |
|---|---|
| Work order | `WO-MOK-016`, Phase 3.1 — contact, conflict and society, under `CAP-MOK-010` |
| Format | the work order's *Completion report format*, seventeen items, in its order and with its wording as each section's heading |
| Baseline | `39662d13abd08e3410648d1c59ad38384f8ad2d2` — the commit the work began from, 212 test names at exit `0` |
| Candidate | `139061530f1dba72c9a20427eeaac6ce69492fb2` — the amended candidate, 250 names at exit `0`; the tree every packet figure that names "the candidate" was taken against |
| Merge | `259859dffe1f5f856e154263c48d8d1e04808903` — the merge of `origin/master`, and the last commit that moved a line of non-documentation source, 264 names at exit `0` |
| Branch tip | `75ea9b548a38ba72e934805cdc1153da3ab2d210`, documentation-only since the merge |
| Packet | `docs/engineering/simulation/evidence/WO-MOK-016/`, indexed by its own `README.md`, whose *Packet size* row is the authority for the file count and byte total |
| Figures | **every figure below is cited to the record that holds it and is not re-derived here.** No measurement is taken by this file, so it cannot disagree with the packet; where a record and this summary would differ, the record is right |
| Date | 2026-08-21 |

This is a report, not a decision. It states what changed, what was measured, what was approved and by
whom, and what remains open. It sets no status, ratifies no figure, signs no assessment and takes no
verification verdict: `WO-MOK-016`'s status is the engineering owner's act, `VER-MOK-016` is the
verification contract, and `VREC-MOK-016` is the record that binds a verdict to a commit. Item 17 states
that boundary in the terms the format requires.

Two things about scope, before any figure. `REQ-MOK-060` was **descoped on 2026-08-21** and carried to
`WO-MOK-017` by the repository owner acting as engineering owner; the work order's *Scope amendment of
2026-08-21* is the record of that act and its four consequences. And the chain was renumbered from
`WO-MOK-012`/`REQ-MOK-042`-and-up to `WO-MOK-016`/`REQ-MOK-051`-through-`060` at `d72465c`; figures taken
before that commit name the pre-renumber identifiers in their own retained text, and this file uses the
current ones throughout, naming the substitution where it matters.

### The commits this file names

| Commit | What it is | Where it appears |
|---|---|---|
| `39662d13` | baseline | everywhere; `baseline/COMMIT.txt` |
| `7c4aef39` | first candidate, three failing tests | `post/test-census-reconciliation.md` §§1–5, `post/reads.md`, the 90-cell matrix |
| `a5ded20e` | branch HEAD on 2026-08-20 | `post/interface.txt` §§2–3 |
| `59d61b91` | the `social` capture's commit | `post/social-manifest.txt`, `post/resolution-tables.md`, `post/composition.md`, `post/delegation.md` |
| `6c1496be` | branch HEAD when oracle 2 was computed | `entropy.txt` |
| `7fda4402` | branch HEAD when the observer was read | `post/observer.md` |
| `1b06d851` | branch HEAD when the four gates were run | `post/gates.txt` |
| `13906153` | the amended candidate | the census, `post/updated-tests.txt` run 1 |
| `d72465c4` | the renumber | `post/updated-tests.txt` run 3 |
| `d8e20794` | `master`'s tip, the merge's second parent | `post/test-census-master.txt`, run 2 |
| `259859df` | the merge | `post/merge-recheck.txt`, `post/test-run-merged.txt` |

`post/interface.txt` §8 measures that its two candidate commits carry the same public surface, and
`a5ded20e`'s non-documentation tree is identical to the candidate's — `git diff --name-only a5ded20
13906153` names no file outside `docs/`. The same holds for `59d61b91`: `post/updated-tests.txt`'s tree
table records it, so the capture commit and the census commit extract the same suite.

---

## 1. What changed, file by file, and what deliberately did not

### 1.1 The change surface, measured

`git diff --numstat 39662d13 13906153`, everything outside `docs/`. **Fifteen files, 3,835 insertions and
213 deletions.**

| File | + | − | What changed |
|---|---:|---:|---|
| `mokiterions-core/src/simulation.rs` | 2,842 | 157 | the whole of the behavior change and its internal-tier tests: the contact constant and predicate, seven `Action` variants, `Policy::Social`, the fourth `DecisionSource` with six branches and three named thresholds, `Observation`'s two added fields, `Mokiterion`'s transient suffered-attack record, rule 6's targeted preconditions, the three resolution functions, three `EventType`/`EventDetail` pairs and `ALL` from twelve to fifteen |
| `mokiterions-core/tests/viability.rs` | 298 | 0 | the survivor curve and the identifier band, public tier |
| `mokiterions-core/tests/decisions.rs` | 218 | 0 | the verb census over the declared matrix |
| `SIMULATION_RULES.md` | 205 | 39 | the reader-facing mirror of `SPEC-MOK-001`'s amended rules, rules 20 to 26 among them |
| `mokiterions-tui/tests/verification.rs` | 63 | 2 | the observer's presentation of a rejection, and the nine renderable viewports |
| `mokiterions-core/tests/cli.rs` | 51 | 0 | the fourth `--policy` value at the argument boundary |
| `mokiterions-core/tests/process.rs` | 38 | 0 | the process boundary under `social` |
| `mokiterions-tui/src/render.rs` | 35 | 4 | one new function, `action_text`, and three call sites |
| `mokiterions-core/tests/termination.rs` | 28 | 0 | termination under the fourth source |
| `mokiterions-tui/tests/authority.rs` | 22 | 1 | three authority rows and the exhaustiveness tripwire |
| `mokiterions-tui/src/authority.rs` | 10 | 1 | one `DecisionSourceSelected` arm, three `EventType` arms, and `table`'s hand-written source row extended |
| `mokiterions-tui/tests/options.rs` | 10 | 3 | the advertised policy set |
| `mokiterions-core/src/cli.rs` | 8 | 4 | two `--policy` lines gain `\|social`, five help lines describe the source, and the diagnostic becomes "expected baseline, reference, individual, or social" |
| `mokiterions-core/tests/naming.rs` | 6 | 1 | the swept policy list |
| `mokiterions-tui/src/options.rs` | 1 | 1 | the observer's own `USAGE` gains `\|social` — §1.2 |

`SIMULATION_RULES.md` is at the repository root and is one of the fifteen. Under `docs/` the same
interval moves `docs/ROADMAP.md` at 73/5, six governance artifacts at their pre-renumber paths —
`VER-MOK-016` 161/35, `WO-MOK-016` 129/16, `REQ-MOK-057` 113/55, `REQ-MOK-058` 49/17, `SPEC-MOK-002`
22/1, `SPEC-MOK-001` 19/10 — and the evidence packet itself, added file by file. All told the interval is
146 files at 26,046 insertions and 352 deletions, of which the packet is the bulk.

The six governance figures are the **escalation** package and the realignments it forced, not the
original amendment: `SPEC-MOK-001` already carried its `CAP-MOK-010` row at the baseline, because the
work order's own ordering puts the amendment act before implementation begins. Item 15 names all of them.

### 1.2 Two deviations from the declared surface, both measured

The work order's *Expected change surface* is right about thirteen of the fifteen files and wrong about
two, in opposite directions. Both are measured rather than argued.

- **`mokiterions-tui/src/state.rs` is named and carries zero changed lines.** The surface asks for
  "ingest and presentation for the new event types". Presentation moved; ingest did not have to.
  `state.rs:224`'s `ingest` matches on `&event.detail`, keeps only the five details the observer's own
  state needs, and ends `_ => {}` — so three added detail variants need no arm and the file compiles
  unchanged. The observer's state is not a mirror of the vocabulary, which is why the wildcard is
  admissible here and refused in `authority.rs`, where the mapping *is* the vocabulary and a wildcard
  would let a type pass untested. `post/observer.md` §3 is where that asymmetry is read.
- **`mokiterions-tui/src/options.rs` changed and is not named**, at 1 insertion and 1 deletion: the
  observer's own `USAGE` gains `|social`. The observer parses its own arguments, so the engine's help
  text is not the only place the value set appears. It is one line, it is asserted by
  `tui/tests/options.rs :: the_usage_text_advertises_every_policy_the_engine_accepts`, and it is
  recorded here because a declared surface that omits a changed file is the kind of omission a
  completion report exists to close.

### 1.3 What deliberately did not change, each backed by a record

The format names eight subjects. Each is a promise this work order made and kept, and each has a
measurement behind it rather than an assertion.

| Subject | Kept how | Record |
|---|---|---|
| Rule 4, `baseline`'s candidate list | untouched. `baseline` applies no waste condition at all, so no correction could reach it | `post/byte-identity.txt` — 30 of 30 `baseline` cells byte-identical |
| Rule 9's eat effect | untouched | `post/world-rules-unchanged.txt` — 21 regions byte-identical |
| The food table | untouched | same |
| Rules 14 to 16 | untouched — placement, regeneration amount and class selection all as they were | same; and `post/reads.md` §5 rows 1, 2 record them as outside `REQ-MOK-059`'s obligation |
| `baseline`'s output | byte-identical at every declared cell, which is `INT-MOK-010`'s promise | `post/byte-identity.txt`, 30 of 30 |
| The perception radius | unchanged at `16`; `fear` is still driven by perception at that radius, which is what made rule 26's first-approved gate unsatisfiable | `escalation.md` |
| The defaults | unchanged; `reference` is still the default source | `post/interface.txt` §2, the `#[default]` marker on `Policy` present at both commits |
| The exit codes | unchanged | `post/runs.md` §2 — exit `0` at all fifteen cells, extinction included; `post/gates.txt` §4 |

The `reference` and `individual` sources gain no verb, and that is the strongest of these: all **60** of
their declared cells are byte-identical to the pre-change capture, so nothing about their behavior can
have moved. Identical output is identical survivors, which is why item 7's carried floors need no
re-measurement.

---

## 2. The owner decisions as stated at approval

The work order's *Decision record*, *The values decided after the packet was drafted* and *The three
decisions the validation did not supply* are the authority for this section; the outcomes are reproduced,
the reasoning is not.

### 2.1 The nine of shape — 2026-08-20, interactive session

| Decision | Outcome | Role |
|---|---|---|
| Scope of the first work order | Phase 3.1 only | product owner |
| How a defender answers | damage resolves immediately under engine authority; `fight`, `retreat` and `surrender` are the defender's own proposals at its own next opportunity, enabled by an attacks-suffered field on the rule 3 observation | technical owner |
| How resolution is computed | deterministically, an integer function of the striker's `energy` and `health`, no entropy draw | technical owner |
| What counts as contact | Chebyshev adjacency, radius `1`, co-location included | product owner |
| The striker's first-move advantage | bounded by verification, not compensated for by a rule | product owner |
| What surrender costs | a forfeit of `satiety` to the Mokiterion surrendered to, capped at the recipient's maximum, excess destroyed | product owner |
| What threatening does | raises the target's `fear` and nothing else | product owner |
| Where the high-class accumulation effect is addressed | inside this work order's measurement | product owner |
| Which source proposes the new verbs | a fourth decision source, which is also the thing that reads `fear` | product owner |

Three were reached against declined alternatives, and the reasoning constrains the implementation: the
defender's answer is **deferred** rather than resolved inside the attack, at the cost of an asymmetric
latency of zero or one tick by identifier order; the fourth source is a **fourth source** and not a
change to `individual`, which leaves `individual`'s floor as a control; and the composition correction is
placed in the **sources' waste condition** and not in the world, which is what keeps `INT-MOK-010`'s
byte-identical `baseline` promise keepable.

### 2.2 The ten of value — 2026-08-20, second interactive session

| Decision | Outcome | Role |
|---|---|---|
| The damage function | `10 + (striker.energy + striker.health) / 10`, truncating — range `10..=30` | product owner |
| The striker's energy cost | a flat `5` per strike, not scaled with damage | product owner |
| The forfeit's magnitude | `satiety / 2`, truncating | product owner |
| The threat's `fear` increase | a new constant `30`, distinct from rule 12's `FEAR_INCREASE` of `10` | product owner |
| The fourth source's name | `social`, from four candidates | product owner |
| The source's decision rule | survival before society, five ordered branches — **amended the same day to six**, rule 19's case 3 hoisted to branch 3 | product owner |
| Its thresholds | `surrender` at `fear` `60`, `retreat` at `30`, `fight` below `30`; engagement **`30` as first approved, moved to `95`** | product owner |
| Its fallback where nothing is perceived | `individual`'s, verbatim, by delegating to rule 19 rather than reimplementing it | technical owner |
| `REQ-MOK-058`'s survivor floor | lowered from six to five before any measurement; **ratified at five on 2026-08-20** on the first measured curve, against measured alternatives of three and two | product owner |
| `REQ-MOK-060`'s composition ceiling | ratified at one half, against `60%` and `40%` | product owner |

Two of the ten moved after they were first taken, and both movements are `escalation.md`'s: the branch
count and the engagement threshold. Item 15 carries the amendment rows that record them.

### 2.3 The three the validation did not supply — 2026-08-20, technical owner, on the amendment text

| Decision | What the owner supplied | Declined |
|---|---|---|
| The event-type set | **three new types, one wherever a second Mokiterion's state moves** — `attack_resolved` shared by `attack` and `fight`, `threat_resolved`, `surrender_resolved`. `REQ-MOK-052` therefore takes **no** row in `SPEC-MOK-003` rule 11's table | seven types, one per verb; and one shared `encounter_resolved` carrying the verb in its detail |
| The observation's valid-proposal list | **it does not grow.** The `social` source proposes targeted verbs the list never carries, and rule 6 validates them at application | growing the list only under `--policy social`, declined so that rule 3's observation stays a function of world state alone rather than of the selected policy |
| Rule 7's clearing order | **the trace line is written before the record is cleared** | clearing first, for uniformity with the attributes `emit_action_trace` reads after the action applies — declined because it would empty the field on exactly the lines it exists to explain |

The first of these was drafted as the implementation agent's and was **not**: `SPEC-MOK-002` rule 5's
enumeration names the event types and has to be approved before implementation begins, so the choice is
the owner's at the moment the amendment is written. The work order's *Authorized decision envelope* was
corrected accordingly, and the correction is recorded rather than made silently.

The declared cost of the valid-proposal decision is also recorded rather than glossed:
`Observation::allows` and the `is_consistent` invariant stop describing the whole proposal contract, and
rule 6 becomes the only place that contract is complete.

---

## 3. Oracle 1's result: the 30 `baseline` cells byte for byte, and the 60 others' characterized divergence

`post/byte-identity.txt`, `RESULT: MIXED`.

- **The `baseline` clause passes on 30 of 30 cells**, byte for byte against the pre-change capture. That
  is `INT-MOK-010`'s promise and the reason the composition correction was placed in the sources' waste
  condition rather than in the world.
- **The 60 `reference` and `individual` cells are also byte-identical**, and that is not the result
  oracle 1 asks for. The oracle asks for a *characterized divergence*, and there is none to
  characterize, "which is `REQ-MOK-060` measured as unimplemented". `post/world-rules-unchanged.txt`
  says the same thing from the other direction: 21 regions of the food, eat and regeneration paths
  byte-identical.

The **descope of 2026-08-21 re-classifies this result and does not rewrite it** — consequence 3 of the
work order's scope amendment: the measurement was correct when taken and is correct now; what changed is
that the missing divergence is no longer an unmet obligation of this work order but the expected result
of a world this work order leaves alone. The file is retained verbatim.

**Two sentences inside that retained file are stale, and are recorded here rather than edited into it.**
The work order's consequence 3 is explicit that the re-classification belongs in the work order and in
`VREC-MOK-016` rather than in the evidence, so this is where they are stated:

1. §3 closes "So this section is a recorded failure of an approved requirement … and `WO-MOK-016` cannot
   reach `implemented` while it stands." **Falsified by the scope amendment of 2026-08-21**, which is
   the act that removed the requirement from this work order's scope for exactly that reason.
2. §4 says `post/social-manifest.txt` records the thirty `social` cells "separately **and
   provisionally**". The provisional label was withdrawn when those thirty were retaken at `59d61b91`
   and digest-matched.

Neither sentence carries a figure, and no figure in the file moves. Both are flagged to the owner in
item 17.

---

## 4. Oracle 2's result, and the recorded stream positions it was checked against

`entropy.txt`, computed at `6c1496be` against the baseline. The oracle asks for a *state*, and the engine
has no command that prints one, so the file computes the state and then checks the computation against
things the engine did print.

| Clause | Result |
|---|---|
| The fifteen pairs — the shared stream's state either side of every resolution kind, at the five declared seeds and three densities | §3. Seed 0 at density `0.75` stands at **270 draws** and state `0xDE8261A4408EDE26`; the suite's fifteen recorded counts are reproduced |
| The reconstruction checked against retained output | §§4–5, against **90 retained initialization excerpts** |
| Every resolution leaves the stream unmoved | §6. **33 call sites over 15 seeds**; at seed 9, 274 draws, and the state before equals the state after equals `0x5760488A3DB8CE83` |
| Every draw site the shipped engine has | §7, enumerated; every `return` in the `social` source's body precedes its one `entropy` use |

That last enumeration is what makes branch 1 to 5's no-draw constraint structural rather than
observational, and it pairs with `post/branches.md` §2's count: the `social` source spends a value only at
branch 6, at 37,975 of 118,201 decisions.

`entropy.txt` §8 states one residual, carried to item 17.

---

## 5. The constructed-state resolution tables, with the paper computation beside the engine's output

`post/resolution-tables.md`, reader `analysis/resolutions.py`, which does not import, parse or read
`simulation.rs`. **Exit `0`: 24 of 24 retained rows and 1,742 of 1,742 released resolutions agree with
the tables.**

| Magnitude | Rule | Where the engine keeps it | Domain read |
|---|---|---|---|
| Damage | 22 | `STRIKE_BASE_DAMAGE`, `simulation.rs:23` | §3, whole domain |
| Energy cost | 22 | `STRIKE_ENERGY_COST`, `:30` | §4, whole domain |
| Threat increase | 23 | `THREAT_FEAR_INCREASE`, `:35` | §5, whole domain |
| Forfeit | 24 | `STRIKE_CONDITION_DIVISOR`, `:27`, and rule 24's **unnamed `2`** at `:2741` | §6, including where a forfeit starts being destroyed |

Three columns from three sources, which is what the clause asks for: the paper value computed from the
specification's own sentences, the engine's constructed-state column from three retained unit tests at
`simulation.rs:5411`, `:5570` and `:5639`, and the released capture's 1,742 resolutions from
`post/social-manifest.txt`'s 30 cells at 30 of 30 digests. §10 records what the runs actually reach: 312
attack lines spanning damage 17–30, so `12`–`16` is paper-only; cost `5` on all 312; all 1,240 threat
lines at `increase:0`, the saturation being reached; and 180 of 190 surrenders destroying part of the
forfeit. Eleven controls — six in the reader, five in the engine, one term each, every one reverted.

Two things this section states plainly. Rule 24's divisor is an **unnamed literal `2`** where the other
three magnitudes are named constants, and rule 23's constant is behaviourally inert at the measured
`fear` levels because every threat line saturates: both are in item 17.

---

## 6. The whole-run figures

`post/runs.md` and `post/runs.txt` for the runs, `post/branches.md` for the source's own distribution,
`post/delegation.md` for oracle 8.

**Survivors and deaths by cause, per seed**, at the fifteen traced cells (`post/runs.md` §2): 102
survivors and 78 deaths across the fifteen, which is 180 — twelve in each cell, accounted for with none
left over. At the default density the five seeds leave **9, 10, 9, 9 and 11** survivors with **1, 2, 2, 3
and 1** combat deaths. All four extinction cells are at density `0.15`, where 59 of 60 die and 54 of the
59 die of attrition: extinction there is starvation, not combat.

**Verbs proposed and applied** (§3): **118,201 proposals, 118,157 applications**. `wait` is proposed 0
times and applied 0 times everywhere; `fight` 13, `retreat` 27.

**Rejections by reason** (§4): **44, all of them `avoid` rejected as `out_of_bounds`**, in 118,201
decisions. Ten of the eleven verbs are never rejected and the eight other grounds of rule 6 never occur.
All 44 occur at nine distinct cells and every one of the nine is on a world boundary — 13 at `(90,127)`,
8 at `(127,127)`, 6 at `(127,57)`, 5 at `(5,127)`, 4 at `(127,72)`, 3 at `(77,0)`, 2 at `(4,127)`, 2 at
`(127,105)`, 1 at `(0,72)`. The mechanism is a wall: rule 21's axis fallback fires when the preferred
axis has no component, not when the step leaves the world.

**The `social` source's branch distribution** (`post/branches.md` §2), over 118,201 decisions:

| Branch | Decisions | Share |
|---|---:|---:|
| 1, answer an unanswered attack | 135 | 0.11% |
| 2, rule 19's cases 1 and 2 | 7,579 | 6.41% |
| 3, food perceived | 58,441 | 49.44% |
| 4, contact | 763 | 0.65% |
| 5, perception | 13,308 | 11.26% |
| 6, rule 19's case 4 | 37,975 | 32.13% |

Branch 1's 135 answers are `13` `fight`, `27` `retreat`, `95` `surrender`, and **0** naming a
non-living target (§3). §4 reads the encounters: 115 of them, 1.36 strikes each, **58.3% bloodless**,
longest 8, and 21 of 156 strikes fatal.

**Oracle 8's delegation comparison, with each source's draw count at matched seeds**
(`post/delegation.md`, exit `0`, 30 checked properties over 120 cells): the precondition decodes from the
released bytes exactly (§2); both proposals are comparable only up to the parting, which happens inside
the first tick at every matched pair, and §5 carries all fifteen partings with the stream's position
either side. §9 evaluates oracle 8's stated consequence — a `social` run takes fewer draws than an
`individual` run at the same seed — at **15 of 15 strictly lower, 0 undecided, 0 contradicted**,
including all five sparse cells where both totals are open intervals and the intervals are still
disjoint. At the default density: `social` 4,540 / 4,457 / 4,387 / 4,290 / 4,480 against `individual`
6,827 / 6,025 / 6,175 / 6,109 / 6,201.

**One transcription in `post/runs.md` §2 was corrected on 2026-08-21** and the correction is stated in
that file: the prose listed the per-seed combat deaths in `runs.txt`'s lexical cell order (`seed123`
before `seed42`) while presenting it as seed order, transposing two adjacent values. The seed-ordered
list is `1, 2, 2, 3 and 1`, which is what §2's own table, `runs.txt` §1's rows and `REQ-MOK-058`'s
approved ratification row all carry. No figure moved and nothing was re-run.

---

## 7. `REQ-MOK-058`'s two bounds per seed, and `REQ-MOK-014`'s and `REQ-MOK-034`'s re-measured floors

`REQ-MOK-058` binds two things at the default density on each declared seed: **at least five of the
twelve living at tick 1,000**, and **at least one death attributable to combat**. Both must hold
simultaneously on each seed.

| Seed | Living at 1,000 | Floor | Combat deaths | Both bounds |
|---|---:|---:|---:|---|
| 0 | 9 | 5 | 1 | hold |
| 1 | 10 | 5 | 2 | hold |
| 42 | 9 | 5 | 2 | hold |
| 123 | 9 | 5 | 3 | hold |
| 777 | 11 | 5 | 1 | hold |

**Five of five, on both bounds, with four survivors of margin at the worst seed.** Two independent paths
produce these ten numbers: `mokiterions-core/tests/viability.rs`'s `the_social_source_keeps_the_world_
habitable_and_combat_lethal`, which runs the engine in-process and counts `target_died:yes`, and
`post/runs.md` §2, which reads the released binary's stdout with a different reader. The requirement's
own ratification row of 2026-08-20 records the same curve, and it is the row that made the floor of five
final rather than provisional.

**`REQ-MOK-014`'s and `REQ-MOK-034`'s floors are not re-measured, and the reason is stronger than a
re-measurement.** The work order's ordering paragraph put the measurement here and the amendment
afterwards, "only if the measurement moves them". Nothing can have moved them: `reference` and
`individual` gain no verb, no Mokiterion under either source ever proposes a targeted action, and
`post/byte-identity.txt` measures all **60** of their declared cells byte-identical to the pre-change
capture. Identical output is identical survivors. So the floors are **preserved rather than
re-established**, the conditional amendment never triggers, and `REQ-MOK-014` correctly carries no
2026-08-20 or 2026-08-21 amendment row — its three rows are all 2026-08-17.

That is the finding, not a gap. `REQ-MOK-034`'s own amendment row anticipated it when it made the
re-measurement "a separate later act … because re-measuring it requires the change to exist".

---

## 8. `REQ-MOK-060`'s composition ratio per territory per class, at both commits

`post/composition.md`, reader `analysis/composition.py`, exit `0`, 35 checked properties over 120 cells,
read from rule 18's own final summary line with no new event and no new instrumentation.

**`REQ-MOK-060` is unimplemented at this candidate** — deferred by the product owner on 2026-08-20 and
descoped to `WO-MOK-017` on 2026-08-21 — so this section measures the curve the deferred decision will be
taken on, which the work order requires to be taken here regardless of which work order owes the fix.

§5 carries the full 60-row per-territory per-class table; §6 is the requirement evaluated exactly as
stated, over three sources × five seeds × two territories, with the 1,000-tick trigger checked rather
than assumed at all fifteen runs:

**Twenty-seven of the thirty evaluations breach the ceiling of one half.** Fourteen of the fifteen runs
carry at least one breaching territory, and the requirement binds on every declared seed, so it is unmet
under all three sources. In every one of the twenty-seven breaches the class over the line is `high`. The
widest class's share runs from **37.3% to 81.6%**. The three that meet it: `reference` at seed 777 in
both territories, at 45.9% and 46.2% — within four points of the line, the same drift caught earlier —
and `individual` at seed 1 in territory B, the only evaluation in the default-density set where the
widest class is not `high` (`low` 37.3%, `high` 35.6%, `medium` 27.1%).

**At both commits.** For `reference` and `individual` the ratio at the candidate *is* the ratio at the
baseline, because all 60 of their cells are byte-identical (item 3); `baseline` is outside the obligation
and §8 measures why it has to be; and `social` has no baseline side, the source not existing there. The
pre-change side is read directly from `baseline/summary.txt` and `baseline/init/` at `39662d13` rather
than inferred from that identity, and **17 of the 20 pre-change evaluations breach the ceiling**, with §7
finding the recorded 45 of 61 itself.

So the coverage row that asked for a contrast between a pre-change state above the ceiling and a
candidate below it is answered on its pre-change half only: **all 20 verdicts are identical at both
commits, because on this measurement the candidate is the pre-change state.** That is recorded rather
than presented as a contrast that was not measured, and the candidate half belongs to `WO-MOK-017`.

One figure belongs to the ratification and is carried to item 17: `VER-MOK-016` records that one half was
chosen for "the `17` points of headroom above the balanced initial third that it leaves", and the
measured drift over these fifteen runs is **32.8% at tick 0 to 64.1% at tick 1,000 — 31.3 points against
17.2 of headroom**. The ceiling is breached at 27 of 30 because the drift is roughly twice the headroom
the choice was argued from.

---

## 9. The identifier-exchange result and the monotonicity band evaluation, with its weakness restated

`identifier.md`, over a retained 1,000-seed sweep — 1,000 ticks, `--policy social`, default density,
`identifier-sweep.json` at SHA-256 `7d7033c4…`, produced twice by two independently written readers that
agree on all 1,000 seeds' three series.

- **The band passes.** §3, on the declared 200: highest ÷ lowest is **`1.0819`** — position 6 over
  position 2 — against a bound of **`< 1.25`**, with **`0.168` of margin**.
- **The identifier exchange passes.** §6: one constructed encounter resolved with the two roles
  exchanged — the same striker attributes, the same target — and the outcome depends on the roles and
  not on the identifiers. That is the whole of stop condition 6.
- **The weakness, restated as the file states it.** `1.25` is not read off the `1.082` it bounds, but
  neither is it derived: §5 measures that a bound of `1.25` on five seeds would fail, which is why 200
  were used, and §8 is explicit that nothing here establishes `1.25` is the right bound — only where it
  came from and that it holds at `1.082`. Two further weaknesses are carried to item 17: the rule 25
  ablation of §7 is **not reproducible from a retained script**, and §8 states `INT-MOK-010`'s risk in
  the opposite direction from the interface's own wording.

---

## 10. The read enumeration, with rules 14 to 16 and 18 recorded as outside the obligation

`post/reads.md`, discharging `REQ-MOK-059` by the documented static examination its *Open decisions*
admit — "**not by an assertion that nobody wrote the forbidden line**".

The candidate set is found mechanically first: every `.count()`, `.sum()`, `.len()`, `.filter(`, `.any(`,
`.all(`, `.position(`, `.max()`, `.min_by`, `.max_by` and `.fold(` in the production half of
`simulation.rs` — lines 1 to 2,972, above `#[cfg(test)]` — is **46 expressions in 17 functions**, plus
**two** added by hand because they search with `.find()`: `Observation::nearest_in_contact` and
`Observation::nearest_beyond_contact`. All **48** are classified; none is omitted.

Seven readers of a set survive classification, and each is outside `REQ-MOK-059`'s trigger — which is
rule evaluation, source consultation and proposal validation — with the reason enumerated as the
requirement's *Constraints* require rather than permit:

| # | Reader | Why outside |
|---|---|---|
| 1 | `Simulation::new`, 1753 | **rule 14** places resources, decides nothing about any Mokiterion, and runs before any acts |
| 2 | `regenerate_food`, 2842–2869 | **rules 15 and 16**; the requirement states these satisfy it as they stand |
| 3 | `summary`, 2896–2907 | **rule 18**'s report — called once at the end of `run`, its `RunSummary` returned to the caller and never to a rule |
| 4 | `food_counts`, 2920 | rule 18's report, same reason |
| 5 | `entity_initialization_events`, 2027 | a `with_capacity` hint and a field of the initialization record |
| 6 | `Simulation::step`, 2080 | **the extinction test** — a genuine population aggregate, read every tick, deciding whether the *run* ends. Recorded explicitly rather than folded into "reporting" |
| 7 | `run_tick`, 2113 | the bound of the acting pass: rule 5's turn order, not what anyone proposes |

`Simulation::snapshot` also reads a living count and a death count; it is the observer's input and the
requirement is explicit that the observer displaying a population count does not violate it, so it is
listed for completeness and not counted among the seven. §6 enumerates every validation path; each reads
the acting agent's own state, or for a targeted verb the one named target.

---

## 11. The public-interface comparison, item for item, against the approved `SPEC-MOK-002` amendment

`post/interface.txt`, reader `analysis/interface.py`, exit `0`. `SPEC-MOK-002` rule 5 does not describe
the interface, it **closes** it, so the comparison needs the surface as a list — and `grep pub` cannot
produce one, because fourteen of the fifteen units of growth are enum variants and a variant carries no
visibility keyword.

|  | Baseline | Candidate |
|---|---:|---:|
| `pub` declarations | 49 | 49 |
| public fields | 43 | 43 |
| trait implementations on public types | 13 | 13 |
| enum variants | **48** | **62** |
| private fields | 18 | 18 |
| the `#[default]` marker on `Policy` | 1 | 1 |
| **entries** | **172** | **186** |

The first two rows are exactly the 92 lines a `grep -cE "^[[:space:]]*pub([[:space:]]|\()"` sees, at both
commits, with nothing left over on either side — which is how the listing's claim to have missed nothing
is checked against a cruder reader that cannot miss anything.

Item for item against the amendment's four growth rows:

| Row | Declared | Measured |
|---|---:|---:|
| `simulation::Policy` | 1 | 1 — `Social,` |
| `simulation::Action` | 7 | 7 — the seven target-carrying variants |
| `simulation::EventType` | 3 + 3 | 3 + 3 — three variants, three payloads |
| `EventDetail::ActionTrace`, existing | 1 | 1 — `suffered: Vec<(String, u8)>` |
| | **15** | **15** |

14 added entries, 2 changed (`EventType::ALL`'s length; `ActionTrace`'s field list), **0 removed**,
everything else identical entry for entry. **The 14-and-1 split is the whole argument for the fourth
row**, and it is the row `SPEC-MOK-002`'s own third amendment added after the second omitted it: a field
appended to a public variant that already existed is the one form of growth an enumeration of added
variants cannot catch. Item 15 carries that row.

`ALL`'s three entries are **inserted before `SimulationEnded`**, not appended, in the enum's own variant
order, with the twelve existing entries keeping their order and relative positions; `suffered` is appended
after `fear`, so a host matching `ActionTrace` with a `..` rest pattern is unaffected; and the field is
`Vec<(String, u8)>` rather than the engine's `SufferedAttack`, so **no type is added** and rule 6's ten
private names stay private, which §6 measures. §7 records two changes the surface diff cannot show, one
of which is a wording defect carried to item 17.

---

## 12. The observer's authority rows and rendered-frame evidence

`post/observer.md`, read at `7fda4402`; the four files it reads are byte-identical at `7c4aef39` and
there, so the reading holds at both candidates. Five one-line mutations are used as controls, each
stated exactly enough to re-apply, each reverted, worktree clean afterwards.

- **The authority rows are settled by comparison**, §2: `SPEC-MOK-003` rule 11 names fifteen event types
  and fifteen identifiers, and the code agrees row for row. The three added rows are `attack_resolved` →
  `REQ-MOK-053`, `threat_resolved` → `REQ-MOK-055`, `surrender_resolved` → `REQ-MOK-056`, and
  `decision_source_selected` gains `REQ-MOK-057` for its fourth value. `REQ-MOK-052` takes no row.
  `REQ-MOK-004` and `REQ-MOK-013`, the two identifiers rule 11 names outside its table, are unchanged.
- **The mapping cannot omit a type, and the compiler is what makes that true**, §3: `for_type`'s match is
  wildcard-free, so a variant added without an arm is `error[E0004]`.
- **The exhaustiveness check has had its failure path read twice**, §5 and `post/merge-recheck.txt` §7:
  deleting the `ThreatResolved` row reports `left: 14, right: 15` at `tests/authority.rs:70`, at two
  different commits.
- **Rendered layout**, §9: the authority overlay grows from 20 lines to **23** with an event
  highlighted, against a longest-other-overlay of 20. At eight of the nine renderable viewports 23
  lines fit. At the floor viewport `34×22` the interior is sixteen lines, which now holds the
  highlighted line, its identifier, a blank and **thirteen of the fifteen rows**, where the same sixteen
  lines held all twelve and a blank at the baseline. What rule 11 obliges — the `t` control presenting
  the highlighted type — is the first two lines and is never clipped; the whole-table listing is more
  than the rule asks for and is what loses two rows.

**The packet holds no rendered frames with cell positions, and this item asks for rendered-frame
evidence.** §9's figures are arithmetic on the retained layout constants — pane heights, `HEADER_HEIGHT`
`3`, `FOOTER_HEIGHT` `1`, a `Paragraph` carrying no `.scroll()` — and the file says so in its own words:
"These figures are arithmetic on the retained layout constants, not measurements of a rendered frame."
Whether a given terminal clips a given row is a question this packet does not answer, and the frame bound
itself is a different `VER-MOK-016` clause — "the observer's frame remains within the bound
`SPEC-MOK-003` states with the new event types ingested" — and a different retention item. This is stated
plainly rather than left to be inferred from the absence of a file, and it is item 17's residual.

---

## 13. The census reconciliation, with every updated test named and its assertion count before and after

Two records answer this item's two clauses, and they answer them from two different kinds of data: the
census reconciles **name lists** from `cargo test` logs, and `post/updated-tests.md` reads **source
bodies**, because a name list cannot show what a test that kept its name now asserts.

### 13.1 The census

`post/test-census-reconciliation.md`, nine sections, reconciled four times as the tree moved.

| | Baseline `39662d13` | Candidate `13906153` | `master` `d8e20794` | Merge `259859df` |
|---|---:|---:|---:|---:|
| Names | 212 | **250** | 226 | **264** |
| Passed | 212 | 250 | 226 | 264 |
| Failed / ignored | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |
| Exit | `0` | `0` | `0` | `0` |

211 + 1 + 38 = 250 against the baseline and 225 + 1 + 39 = 264 against `master`'s tip, so every name on
both sides of both comparisons sits in exactly one row: **retained, renamed once, or added**. The two
reconciliations agree line for line on this work order's own side — the same single rename and the same
39 additions — which is what re-basing the comparison did not change. The first candidate `7c4aef39` is
retained as measured at 249 names with three failures at exit `101`, and §8 reconciles the two 250-name
figures of §§6–7 against the logs that were later retained for them rather than leaving them
unsupported.

The 38 additions at the first candidate sit as `VER-MOK-016` predicts: 28 internal-tier (`unittests`),
and 10 public-tier across `tests/cli.rs` +2, `decisions.rs` +2, `viability.rs` +2, `process.rs` +1,
`termination.rs` +1, `tui/tests/verification.rs` +1. **Nine of the sixteen targets gained nothing**,
which is `post/byte-identity.txt`'s claim from the other direction: this change reaches the decision
layer and the resolution layer and nothing else.

### 13.2 Every updated test, with its assertion count

`post/updated-tests.md`, from `analysis/updated-tests.py` over five extracted trees, eight runs of which
five are controls and **three must fail**. Nine retained bodies changed: **34 assertions before, 38
after**.

| Test | Assertions | The change |
|---|---:|---|
| `src/simulation.rs :: a_name_is_the_same_value_at_both_ends_of_a_run` | 2 → 2 | `Policy::Social` appended to the swept list |
| `src/simulation.rs :: naming_draws_nothing_and_reads_neither_the_seed_nor_the_configuration` | 4 → 4 | the same |
| `src/simulation.rs :: the_trait_is_fixed_for_the_run_and_independent_of_every_configuration` | 2 → 2 | the same |
| `tests/naming.rs :: every_run_reports_the_specified_twelve_names_in_identifier_order` | 2 → 2 | the same |
| `tui/tests/authority.rs :: every_event_type_the_observer_can_present_has_an_entry` | 3 → 3 | the same, over a vocabulary that also grew |
| `tui/tests/options.rs :: the_usage_text_advertises_every_policy_the_engine_accepts` | 3 → 3 | the same, plus a compiler-required match arm |
| `tests/cli.rs :: the_entries_state_the_constraints_that_decide_validity` | 12 → **14** | two assertions appended |
| `tui/tests/authority.rs :: the_decision_source_maps_by_the_source_the_record_names` | 5 → **6** | one assertion appended |
| `tui/tests/authority.rs :: the_mapping_is_the_specified_one` | 1 → **2** | three rows added to the swept table, and a new exhaustiveness tripwire |

**Six of the nine hold the same count, and a count cannot tell a widened sweep from a weakened check**,
so all nine were read by hand. All six are one edit — a swept list of three policies becoming four — so
every assertion the baseline made is still made over strictly more sources; the sweeps go 3→4, 45→60,
18→24, 90→120 and 36→60 cases. `EventType::ALL` went from 12 entries to 15 with all 12 retained in
their original relative order, so the 36 pairs one of those tests checked at the baseline are a subset of
the 60 it checks now.

The renumber `d72465c` moved seven further bodies at **250 names and 25 assertions on both sides and not
one changed non-comment body line**; three of the seven move an identifier a test asserts, and a
one-sided renumber would fail at the merge tip, where all 264 cases pass.

Stop condition 8 is settled clause by clause: *removed* measured at 0 with 202 retained bodies
byte-identical; *renamed away* measured at one absent name, paired by body, with control C refusing that
rename when the body moves and control D accepting it when it does not; *`#[ignore]`d* at 0 before and 0
after and re-grepped at 0 at the merge tip; *relaxed* and *widened* read rather than measured, in the
nine sections above.

**One defect is recorded rather than fixed**: the comment above the two appended assertions in
`mokiterions-core/tests/cli.rs` still says the help "can neither hide the third source nor advertise a
fourth", and both counts are now wrong. Nothing the test asserts is affected. It is not fixed because
correcting three words moves the tree that `post/test-run-merged.txt`,
`post/test-census-merged.txt`, `post/test-census-reconciliation.md` §9 and all eight runs of
`post/updated-tests.txt` were taken against, and `SPEC-MOK-004` rule 11 requires figures to be
re-derived rather than edited. Whether that re-derivation is worth three words is the owner's call and is
in item 17.

---

## 14. The four gates' output

`post/gates.txt` at `1b06d851`, toolchain cargo 1.97.1 / rustc 1.97.1 / clippy 0.1.97, host
`x86_64-pc-windows-msvc`. Each of `VER-MOK-016`'s four declared invocations run once, character for
character as the contract states them, each with its own exit code.

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | no output, **exit `0`** |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | **exit `0`**, cold — from `cargo clean`, which removed 3,699 files and 814.7 MiB, so the exit code is a statement about the lints and not about a cache |
| `cargo tree -p Mokiterions` | **exit `0`** |
| `cargo test` at the workspace root | retained whole rather than restated: 212 at `39662d13`, 249 at `7c4aef39` with three failures at exit `101`, **250 at `13906153` at exit `0`** |

**All four were re-run after the merge** — `post/merge-recheck.txt` at `259859df`, on a toolchain
identical character for character, with `git status --short` empty before the first command and after the
last. All four reach exit `0` again, `cargo test` at **264 names, 264 passed, exit `0`**. The merge also
**added a fifth gate** that did not exist when `post/gates.txt` was written,
`scripts/check_declared_dependencies.py`, and §6 records it at exit `0`; this packet neither takes nor
re-opens the judgement behind it.

The merge falsifies exactly one claim in the packet, and it is `post/gates.txt` §5's four-manifest claim,
re-measured in `post/merge-recheck.txt` §5. Nothing was corrected by edit: "a figure produced by a
command is a statement about a tree, and the merge changed the tree", so the commands were re-run and
both files stand as measured at the commits they name. `post/gates.txt` §3 also rests on a prohibition
`ADR-MOK-006` withdrew, which is in item 17.

---

## 15. The amendment records

The format asks for "each row quoted, with the approval each carries". **The rows run from 200 to over
900 words each**, and reproducing them here would put a second copy of approved text in an ungoverned
record where it can drift from the authority it copies. So each row below is identified by its authority,
its date and its **operative clause quoted verbatim**, with the accountable role and the act structure of
its approval, and cited to the file and line that holds it whole. That is a narrower reading of the
clause than its wording, it is deliberate, and it is flagged to the owner in item 17 as the one place
this summary departs from its declared format.

**The check that each of those rows is recorded where it was approved is not in this file either.**
`amendment-approvals.md` is oracle 7's record and holds it: all **21** provisions of the approved list
found twice in disjoint text — in the amendment row, and in the artifact with every amendment row deleted
— every row located by its own subject rather than by a date eleven rows share, the four superseded
sentences measured as retained and retained only as quotations, and all **33** rows the baseline held
classified with the operative cell compared apart from the approval cell. It is generated by
`analysis/amendments.py` at exit `0`, `RESULT: PASS` on ten verdicts, and it names the two provisions this
packet owes and does not make — one of them the row of item 15.3 below. Where this section and that record
would differ, that record is right.

### 15.1 `SPEC-MOK-001` — three rows

| Row | Operative clause | Approval |
|---|---|---|
| 2026-08-20, line 72 | "Contact, conflict and society, under `CAP-MOK-010`. **Thirteen provisions amended**, of which seven are appended rules, and the frontmatter's `specifies` gains `REQ-MOK-051` through `REQ-MOK-060`." Rules **20 to 26 are appended after rule 19 and not inserted** | repository owner as **technical owner**, in the *single act this amendment's own ordering requires* — together with `REQ-MOK-051`–`060`, `VER-MOK-016` and `WO-MOK-016`, because this `specifies` relation is what makes those ten requirements approvable at all (`validate` `E007`, `preflight` `W016`, both measured). Implementation begins after this act and not before |
| 2026-08-20, line 73 | "Rule 21's co-location fallback and rule 6's paragraph on it are corrected to name **both `avoid` and `retreat`**, under `REQ-MOK-052`." "**Nothing about the fallback's behavior changes**" | repository owner as **technical owner**, on the discrepancy being put with two declined alternatives — recording it as residual uncertainty, or restricting `retreat` in the code to match the text literally. **The implementation is unchanged**; it was already what the corrected text states |
| 2026-08-20, line 74 | "Rule 26's branch order and engagement threshold, under `REQ-MOK-057`'s first amendment. **Six provisions of that rule alone; no other rule is touched, and rule 12 in particular is not.**" The five-branch list becomes six; the engagement threshold moves from `30` to `95` | repository owner as **product owner and technical owner**, on the measured evidence in `evidence/WO-MOK-016/escalation.md`: seventeen variants across three levers, three packages put, and the one chosen **leaves rule 12 as Phase 2 approved it**. `REQ-MOK-058`'s floor of five is ratified unchanged in the same act |

The first row was already in the tree at the baseline, which is why the interval `39662d13..13906153`
shows `SPEC-MOK-001` at only 19 insertions and 10 deletions: the ordering puts the amendment before
implementation, so what moved inside the interval is rows two and three.

The second row is the second instance of one failure mode this chain met three times — **an enumeration
written from the change rather than from the surface understates a closed enumeration**. Here rule 21's
fallback named `avoid` alone where `avoid` and `retreat` share the path.

### 15.2 `SPEC-MOK-002` — three rows

| Row | Operative clause | Approval |
|---|---|---|
| 2026-08-20, line 36 | "**Rule 1's empty-table rule withdrawn** and replaced by the declared-set form, decided by `ADR-MOK-006`." New **rule 13, *Declared dependency set*** | repository owner as **accountable technical owner**, by way of `ADR-MOK-006`; written under `WO-MOK-014`. Records and does not delete a miscount it first made about the two 2026-08-18 rows |
| 2026-08-20, line 37 | "Rule 5's enumeration amended and rule 6 re-checked, under `CAP-MOK-010`", `specifies` gaining `REQ-MOK-052`–`057`. `Policy` +1, `Action` +7, `EventType` +3 with three payloads, `EventType::ALL` from `12` to `15`. "**The observation's two new fields are not interface growth**", `Observation` being declared without `pub`. Rule 6 recorded as **not amended** | repository owner as **technical owner**, in the same **single act** as `SPEC-MOK-001`'s first row |
| 2026-08-20, line 38 | "**Rule 5's growth table gains a fourth row, which the amendment above omitted**": `EventDetail::ActionTrace` gains `suffered: Vec<(String, u8)>`. Growth is therefore `1 + 7 + 3 + 3 + 1` and the table says four items change shape where it said three | repository owner as **technical owner**, in a **separate act**, the omission having been found after the first. Two alternatives declined: bundling the row with `REQ-MOK-060`'s deferred amendment, and treating the field as covered by `SPEC-MOK-001`'s trace provision. **The implementation is unchanged** — the field was already present and already in pair form at `simulation.rs:1425` |

The third row is the third instance of the same failure mode, and the clearest: "the row above enumerates
added *variants*, and a field appended to a public variant that already existed is the one form of growth
such an enumeration does not catch". Item 11's 14-and-1 split is that row measured.

### 15.3 `SPEC-MOK-003` — the two this chain owns, of eleven 2026-08-20 rows

| Row | Operative clause | Approval |
|---|---|---|
| 2026-08-20, line 75 | "Three provisions amended under `CAP-MOK-010`", `specifies` gaining `REQ-MOK-052`, `053`, `055`, `056`, `057`. **Rule 11's** authority table gains three rows — `attack_resolved` → `REQ-MOK-053`, `threat_resolved` → `REQ-MOK-055`, `surrender_resolved` → `REQ-MOK-056` — and `decision_source_selected` gains `REQ-MOK-057`; "**`REQ-MOK-052` takes no row**" | in the same **single act**; item 12 is the code measured against this table row for row |
| 2026-08-20, line 76 | "**No rule changed.** This row records the reconciliation of the `CAP-MOK-010` rule 4 amendment above with the `WO-MOK-013` amendments above it, which were written against different trees and met in a merge." Both retained verbatim, neither owner act edited, summarised or folded into the other | **no owner act.** Its approval cell reads "Recorded by the implementation agent as a statement of fact about amendments it holds no authority over, on the precedent of the two 2026-08-19 reconciliation rows above. Nothing is ratified here and no provision changes." Whether a row in an approved specification's amendment record may stand on that basis is the owner's, and it is flagged in item 17 |

The other nine 2026-08-20 rows on this specification are `WO-MOK-013`'s and `ADR-MOK-006`'s and are not
this work order's; they are named here only so their presence in the same record is not read as this
chain's.

### 15.4 The three requirement amendments

The work order's *Required amendments* section is explicit that these three do not share the
specifications' ordering and do not share each other's.

| Amendment | Row | Approval | State |
|---|---|---|---|
| `REQ-MOK-005` | 2026-08-20: "The four-verb enumeration is re-read as the **core** set rather than as the whole action contract"; "**Nothing this requirement obliges changes**" | repository owner as **product owner**, in the single act. **Amended and not superseded**, because it is cited by `CAP-MOK-001`, `SPEC-MOK-001`, `VER-MOK-001`, `WO-MOK-001`, `SPEC-MOK-003` rule 11 and two locations in `mokiterions-tui`, two of those released under `RLS-MOK-001` | **taken** |
| `REQ-MOK-034` | 2026-08-20: narrowed the frozen-outcome constraint "from 'the reference or baseline source' to `baseline` alone"; "**The floor of eight of twelve is not touched by this row**" | repository owner as **product owner**, in the same act as `WO-MOK-016` — earlier than its ordering required, since it had only to precede the change that moves `reference` | **taken, and unused.** `reference` has not moved, so it is a permission granted ahead of its use; `WO-MOK-017` is where it is used |
| `REQ-MOK-014`'s and `REQ-MOK-034`'s survivor floors | none | — | **untaken, and correctly so.** The amendment was conditional on the measurement moving the floors; item 7 measures them unmoved, `REQ-MOK-014` carries no row later than 2026-08-17, and `REQ-MOK-034`'s row anticipated this as "a separate later act … taken only if the measurement moves them" |

**Four further rows were taken by the escalation and are not among the three the work order foresaw**, and
they are named here because a completion report that quoted only the foreseen ones would understate the
record:

| Requirement | Rows |
|---|---|
| `REQ-MOK-057` | 2026-08-20 original content — five branches, constants `60`, `30`, `30`, technical owner. Then 2026-08-20: "**The branch order and the engagement threshold**" — six branches, threshold `30` → `95` |
| `REQ-MOK-058` | 2026-08-20 original content — the two-sided bound, product owner. Then 2026-08-20: "**The floor of five is ratified unchanged on the first measured curve, and the survival-first clause is corrected**" — the row that made item 7's floor final rather than provisional |

`VER-MOK-016` itself carries three 2026-08-20 rows realigning the contract to all of the above. Its third
row names four provisions, and **it does not name the residual-uncertainty rewrite that its own first
residual bullet attributes to it** — the fourth instance of the same understated-enumeration failure
mode, and item 17's flag.

---

## 16. The eleven manual assessments

`VER-MOK-016`'s *Manual assessments* section. **Two are recorded, one is half-recorded, one is carried
out of scope, and seven are outstanding.** Their subjects are not restated here; the classification is
what this item asks for, with the role that owes each.

| # | State | Owed by | Note |
|---|---|---|---|
| 1 | **outstanding** | product owner | |
| 2 | **outstanding** | product owner | |
| 3 | **outstanding** | product owner | |
| 4 | **recorded** | product owner | ratified by `REQ-MOK-058`'s 2026-08-20 amendment row on the measured curve — survivors 9, 10, 9, 9, 11 and combat deaths 1, 2, 2, 3, 1, against measured alternatives of three and two. Item 7 is that curve |
| 5 | **carried to `WO-MOK-017`** | product owner | it is the ceiling's ratification and the per-class-floor question, and both follow `REQ-MOK-060` under the scope amendment of 2026-08-21. The measurement it will be decided on is nonetheless taken and retained — item 8 — so `WO-MOK-017` inherits the curve and not just the question. Its wording also has a defect: "measured corrected composition" has no subject at this candidate, there being no corrected composition |
| 6 | **half-recorded** | product owner | the `REQ-MOK-034` narrowing half is approved. The other half asks for a **characterized divergence** and its premise is false at this candidate: item 3 measures 60 of 60 cells identical, so there is no divergence to characterize |
| 7 | **outstanding** | product owner | |
| 8 | **outstanding** | technical owner | |
| 9 | **outstanding** | technical owner | |
| 10 | **outstanding** | assurance owner | |
| 11 | **recorded** | assurance owner | assessed 2026-08-20, "and it was not adequate" — replaced by the two-part oracle 5, which item 9 reports |

**This summary signs none of them.** A manual assessment is an accountable owner's judgement and an
implementation agent may prepare the record and may not complete it. `manual-assessment.md` in this packet
is that preparation, and it is prepared rather than empty: each of the eleven carries the contract's own
words, the value decided in advance where there is one, the measured evidence with a citation to the
record here that holds each figure, the outcomes available, and a **Record block left blank for the
accountable owner**. Eight of those blocks carry no decision — assessments 1, 2, 3 and 7 and the second
half of 6 for the product owner, 8 and 9 for the technical owner, 10 for the assurance owner — and
`VREC-MOK-016` cannot reach `verified` while any of them does not. Item 17 states that as the residual it is.

---

## 17. Residual uncertainty

### 17.1 What remains open under *Stop and escalate conditions*

Fourteen conditions are declared. **None of the fourteen fired**, and the three items the section
discloses in advance all stand:

- **`VREC-MOK-005`'s gate** — disclosed, unchanged by this work order.
- **`doctor` exits `1` on version skew** between the locally installed harness and the `0.4.0` the
  repository pins and CI installs. A local skew, not a repository defect; `python -m se_harness validate
  .` passes.
- **`mokiterions-tui/tests/export.rs`'s `a_written_file_holds_exactly_the_rendered_records` is
  intermittently flaky on this platform**, as `WO-MOK-011` disclosed. A test change is a stop condition
  here, so it is reported as residual rather than adjusted.

Condition 8 — no existing test's assertions relaxed, widened, removed, renamed away or `#[ignore]`d — is
the one that needed measuring rather than asserting, and item 13.2 settles it clause by clause with three
of its five clauses measured and two read. Condition 6, the identifier exchange, is item 9.

One condition's disclosure was **carried on 2026-08-21 to `WO-MOK-017` with `REQ-MOK-060`**: the one
about what this work order promises regarding `baseline`. The promise itself is kept and measured at 30
of 30 (item 3).

### 17.2 `VER-MOK-016`'s nine residual bullets

All nine stand as written; the first carries "Amended 2026-08-20, with this contract's third amendment
row". The five coverage rows belonging to `REQ-MOK-060` are **carried to `WO-MOK-017`**, named there, and
this packet claims no coverage it does not have — consequence 4 of the scope amendment.

### 17.3 What this packet does not hold, or holds and should not be read as more than it is

- **No rendered frames with cell positions** (item 12). §9's overlay figures are arithmetic on retained
  layout constants; whether a terminal clips a given row is unmeasured, and the frame bound is a
  different clause and a different retention item.
- **The rule 25 ablation is not reproducible from a retained script** (`identifier.md` §7).
- **`1.25` is not established as the right bound**, only where it came from and that it holds at `1.082`
  (`identifier.md` §8).
- **No release-side instrument reports whole-run draw totals per source.** `shared_stream_draws` is a
  test helper; `post/delegation.md` §6 recovers the totals rather than reading them off an instrument.
- **`entropy.txt` §8's residual** stands as written.
- **The observer's two residuals in `post/observer.md` §6 did not narrow, and the merge measured that
  they did not.** The exhaustiveness assertion catches a variant added to `ALL` without a table row and
  not a variant added to `EventType` and omitted from `ALL`; and one identifier in the presented table is
  produced by no call to `for_type` and stays unasserted. `post/merge-recheck.txt` §7 records that
  `origin/master`'s `WO-MOK-013` added **682 non-comment lines to `mokiterions-tui/tests/render.rs`** and
  grew the observer package from 128 cases to 142 without one of the 14 asserting that row's content.
  Both residuals are one assertion away from closed, and closing either moves the reconciled census.
- **At 10,000 ticks the five seeds leave 0, 5, 2, 2 and 0 survivors** against a floor of five, two of
  them by extinction before the horizon (`post/long-horizon.md`). `REQ-MOK-058` binds at 1,000 ticks and
  is met there — item 7 — so this is outside its trigger and is retained "as evidence and not as an
  obligation". What it bears on is the ratification's *reasoning*: "four survivors of margin at the worst
  seed" is a property of the 1,000-tick curve alone, and whether the world should still be populated at
  10,000 is a question this packet raises and does not answer.
- **The composition ceiling is breached at 27 of 30, at a drift of 31.3 points against 17.2 of
  headroom** (item 8). Outside this work order's scope since 2026-08-21, and the curve `WO-MOK-017`
  inherits.
- **`post/gates.txt` §3 rests on a prohibition `ADR-MOK-006` withdrew**, and **§5's four-manifest claim
  is falsified at the merge tip** — re-measured, not edited, in `post/merge-recheck.txt` §5.
- **`post/byte-identity.txt` carries two stale sentences**, item 3, recorded here rather than edited into
  the file because the scope amendment's consequence 3 puts the re-classification in the work order and
  in `VREC-MOK-016`.
- **This section 15 quotes each amendment row's operative clause rather than the row whole**, which is a
  narrower reading of the format than its wording; the reason is stated there.
- **`VER-MOK-016`'s retention table lists no completion report**, so this file sits in the packet on
  `WO-MOK-016`'s authority — its *Completion report format* — and not on the verification contract's.
  The packet's `README.md` therefore carries no "What is owed" row for it and says so in the same
  paragraph that says it of `escalation.md`. Whether the contract should retain the report the work order
  demands is an owner's edit to `VER-MOK-016`, not this file's to make.

### 17.4 Eleven findings for the owner that no record in this packet can close

Each is measured or read, none is a failure of a clause as written, and every one of them is either a
judgement an owner owes or an edit that would move a commit the packet's figures are bound to.

| # | Finding |
|---|---|
| 1 | `SPEC-MOK-004` rule 11's own figure needs correcting, and rule 11's `WO-MOK-013` row carries a defect |
| 2 | `SPEC-MOK-002` rule 5's line-250 wording defect |
| 3 | `VER-MOK-016`'s third amendment row does not name the residual-uncertainty rewrite among its four provisions |
| 4 | `SPEC-MOK-003`'s reconciliation row of 2026-08-20 stands on an implementation agent's statement of fact with no owner act, on a precedent rather than an approval (item 15.3) |
| 5 | Rule 23's `THREAT_FEAR_INCREASE` is behaviourally inert at the measured `fear` levels: all 1,240 threat lines report `increase:0` |
| 6 | Rule 24's forfeit divisor is an unnamed literal `2` at `simulation.rs:2741` where the other three magnitudes are named constants |
| 7 | One assertion would close the strike-cost gap; one would close each observer residual |
| 8 | The asymmetric `debug_assert!` at `simulation.rs:2624` |
| 9 | The suffered-record bound is unasserted |
| 10 | `INT-MOK-010`'s risk is stated in the opposite direction in `identifier.md` §8 |
| 11 | The stale comment in `mokiterions-core/tests/cli.rs`, and whether three words are worth re-deriving every figure bound to that tree (item 13.2) |

### 17.5 `VREC-MOK-016`

**`VREC-MOK-016` is a separate, commit-bound verification record that this work order does not write and
cannot self-approve.** It binds a verdict to a commit; it is composed against `VER-MOK-016`'s coverage
matrix, not against this summary; it reports `REQ-MOK-060`'s five coverage rows as carried to
`WO-MOK-017` and claims no coverage it does not have; and it cannot reach `verified` while any of item
16's outstanding manual assessments is unsigned, because each of those is an accountable owner's
judgement and not an implementation agent's. Nothing in this file is a verification verdict, and nothing
in this file sets a status.

---

## What this summary does not do

- It takes no measurement. Every figure is cited to the record that holds it, so this file cannot
  disagree with the packet; where it would, the record is right.
- It sets no status, signs no assessment, ratifies no figure and approves no amendment.
- It is not the packet index. `README.md` is, and it carries each record's figures with citations and the
  "What is owed" table.
- It does not restate the evidence. Where a claim needed a run, the run is in the record named beside the
  claim, and the record was not re-taken for this file.
