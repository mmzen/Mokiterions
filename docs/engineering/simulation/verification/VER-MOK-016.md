+++
id = "VER-MOK-016"
type = "verification"
title = "Encounter verification: exact resolution on constructed states, reachability in whole runs, an unmoved baseline, and two claims of absence"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-21"

[relations]
verifies = ["REQ-MOK-051", "REQ-MOK-052", "REQ-MOK-053", "REQ-MOK-054", "REQ-MOK-055", "REQ-MOK-056", "REQ-MOK-057", "REQ-MOK-058", "REQ-MOK-059", "REQ-MOK-060"]
+++

# Verification Contract: Encounter verification

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-20 | Original approved content. | Approved 2026-08-20 by the repository owner acting as assurance owner. |
| 2026-08-20 | **Realigned to `REQ-MOK-057`'s first amendment**, which added a branch and moved the engagement threshold from `30` to `95`. Four provisions: the engagement-threshold oracle now asserts `94` and `95`, and additionally asserts `30` and `60` so that the gate is proved *not* to be either answer threshold — the two were equal by coincidence before and a test that passed at `30` could not tell the constants apart; the draw-discipline oracle renumbers and adds the assertion that the new branch 3 draws nothing, which is what makes the amendment a reordering of decisions rather than of the shared stream; the differential oracle against `individual` is **widened** to hold wherever a tolerated resource is perceived, and both the narrow and the widened form are asserted; and manual assessment 8 records the six-branch ordering with the gate at `95`. No oracle is weakened or removed, and the retention list is unchanged. | Approved 2026-08-20 by the repository owner acting as assurance owner, on `REQ-MOK-057`'s amendment being approved the same date and on the measured evidence in `evidence/WO-MOK-016/escalation.md`. The realignment does not decide anything: it follows an approved requirement change, and the one oracle whose strength it alters it strengthens. |
| 2026-08-20 | **Oracle 5's outcome half is restated: the rank-correlation band is removed and a turn-position survival bound replaces it.** Four provisions. The monotone non-increasing check stays exactly as written and stays on the five declared seeds, because that is the part five seeds support. The `±0.5` rank-correlation band on identifier is **removed** and no rank correlation is bounded anywhere in this contract; both correlations are recorded as evidence with their event totals. A **declared diagnostic seed set** of the 200 seeds `0`–`199` is added, carrying no survivor floor, no lethality bound and no comparability obligation, and the obligations stay on the five declared seeds. Over that set, the ratio of the highest to the lowest survival rate across the six turn positions within a territory must be **below `1.25`**, a figure taken from the three-of-twelve survivor cost `REQ-MOK-058`'s floor concedes below `REQ-MOK-034`'s and not from the curve it bounds; the measured value on that set is `1.082`, and `1.063` over all 1,000 seeds measured. The bound and the set size are one provision: at 50 seeds a group would breach `1.25` on noise alone, which is measured in the section below. Manual assessment 11 is answered rather than deferred. **This weakens no oracle's ability to fail**: the removed bound could fail on noise and pass on a real advantage, and both replacements were measured against 9,194 resolved strikes and 12,000 identifier-runs over 1,000 seeds before being written. | Approved 2026-08-20 by the repository owner acting as product owner on accepting the measured turn-order advantage as a property of the world, and as assurance owner on what this row should bound, both against the four measured options in `evidence/WO-MOK-016/escalation.md` §11. The advantage is identifier-blind — the identifier-symmetry oracle still passes — and an ablation measured that amending `SPEC-MOK-001` rule 25 would shrink it by about two thirds without changing its ordering, so the alternative of changing the world was declined with its cost known. |
| 2026-08-21 | **Realigned to `REQ-MOK-060`'s amendment of the composition ceiling from one half to three fifths.** Five provisions, none of which changes what is measured or how. Oracle 4's coverage row asserts three fifths and states which side of the boundary passes — exactly three fifths passes, a share strictly above it is the breach — because the previous row said "more than half" without settling equality, and at a coarse standing total that is a reachable case rather than a pedantic one. The pre-change contrast row asserts the pre-change measurement exceeds three fifths, which is the stronger contrast the amended ceiling permits and which the retained `45` of `61` at `73.8%` satisfies. The composition-ratio property carries three fifths. Manual assessment 7 carries it on both of its halves. Manual assessment 5 is restated: the ceiling's value is no longer what the reviewer assesses, because `REQ-MOK-060`'s amendment record closes it and a verification contract does not reopen a requirement; what the reviewer assesses is that three fifths is met on the shipped build with the margin the amendment claimed, and whether the corrected composition warrants a per-class floor. The residual on pre-measurement numbers records that one of its seven has now moved. **No oracle is weakened in its ability to fail**: the ceiling oracle's threshold moves with its requirement, and the contrast oracle it is paired with is strengthened. Nothing about the other nine requirements, the retention list, the draw-discipline oracles or the `baseline` byte-identity obligation moves. | Approved 2026-08-21 by the repository owner acting as assurance owner, on `REQ-MOK-060`'s amendment being approved the same date by the same owner acting as product owner, and on the measurement retained in `evidence/WO-MOK-017/approval/`. This row realigns and decides nothing: it follows an approved requirement change, and where it alters an oracle's strength it raises it. The implementation agent wrote this text under `WO-MOK-017`; it did not decide the substance. |

## Independence

This is the largest behavioral change since `WO-MOK-001`, and the temptations it creates are specific enough to name.

Five claims decide whether it is sound, and only one of them is about whether combat works.

- **The resolution arithmetic is exactly what `SPEC-MOK-001` says**, not approximately, and not only at the states a run
  happens to reach.
- **It is reached.** A mechanism verified only on constructed states is a mechanism that might never fire in a real run,
  which is the failure `REQ-MOK-032` rejected a four-cell `fear` threshold for.
- **The `social` source's fallback is the trait-aware source verbatim.** `REQ-MOK-057`'s branches 2, 3 and 6 delegate to
  rule 19 rather than reimplementing it, which turns "unchanged" from a claim about how the code was written into an
  equality that can be checked on an observation.
- **The `baseline` source did not move**, at all, and the movement in `reference` and `individual` is attributable to
  `REQ-MOK-060` and to nothing else.
- **Two absences hold**: nothing reads a population aggregate, and the acting order does not hand a systematic advantage
  to low identifiers.

  **Measured 2026-08-20, and the second absence holds only as literally as it is written.** The acting order hands a
  systematic advantage to *later*-acting Mokiterions, which is the opposite direction, so no low identifier is favored
  and this claim is not falsified. It should not be read as "no advantage by identifier": there is one, it is ordered,
  and the turn-position bound below is what bounds it. Nothing reads an identifier — the symmetry oracle is what carries
  that — so the advantage is turn order's and not the identifier's.

The first two pull in opposite directions and the contract needs both, because contact is rare. `REQ-MOK-032`'s
arithmetic puts the expected number of other Mokiterions inside a Chebyshev box of radius `1` at about `0.0060` per
agent-tick — roughly 72 contact agent-ticks in a 1,000-tick run before any Mokiterion deliberately closes distance. A
suite that waits for encounters will exercise the interesting branches thinly and by accident. A suite that constructs
them will exercise everything and prove nothing about the world. So exact arithmetic is verified on constructed states
and reachability is verified on whole runs, and **neither oracle is permitted to stand for the other**.

The third claim is the additivity claim, and it is stronger here than in any prior contract: `baseline` is compared
**byte for byte with no projection at all**. No record kind that a `baseline` run emits gains a field, so any difference
anywhere in those streams is a defect. Where earlier contracts had to trust a projection, this one does not, and the
absence of a projection is itself a check: if a projection turns out to be needed for `baseline`, the change has touched
something `REQ-MOK-060` forbids it to touch.

The fourth pair is where a well-intentioned implementation does the most damage. `REQ-MOK-058` states a survivor floor,
and the cheapest way to meet a survivor floor is to have the engine suppress combat when the population runs low — which
is `README.md`'s prohibited shape with its sign reversed, and which would pass every habitability row in this matrix.
`REQ-MOK-059` is the row that forbids it, and it is verified by an enumeration of reads rather than by a search for a
string.

Eight independent oracles are used.

1. **A recorded pre-change capture of the existing three-source matrix, compared without projection.** The engine's
   complete standard output and exit code are captured *before* any code change, at the commit the work begins from,
   across the matrix `VER-MOK-011` declares and this contract reuses: the seeds `0`, `1`, `42`, `123` and `777`; the
   three existing decision sources; the default density `0.75%` and the two swept densities `VER-MOK-002` declares;
   `--ticks 1000`; with and without `--trace-actions`. That is 90 cells, of which 30 are `baseline`.

   Afterwards the same 90 cells are captured again. The 30 `baseline` cells must be **byte-identical, unprojected**, with
   identical exit codes. The 60 `reference` and `individual` cells are expected to differ, and each difference must be
   attributed to `REQ-MOK-060`'s corrected waste condition by a stated characterization — first diverging tick, first
   diverging record, and the eat-or-seek decision that changed — not merely noted.

   The baseline is captured once, from a tracked-clean worktree, with the commit recorded. A discrepancy is never
   resolved by recapturing it.

2. **The shared entropy stream's own position.** `VER-MOK-007` oracle 2 already pins the stream's state at the end of
   initialization for the declared seeds and densities as recorded expectations checked into the suite, and
   `VER-MOK-011` reused them unchanged. This contract reuses them unchanged again.

   Beyond initialization, an internal-tier test records the shared stream's state immediately before and immediately
   after each of the four resolutions this change adds — attack, fight, threaten, surrender — and immediately before and
   after each targeted move, and asserts equality. This is the direct form of `REQ-MOK-053`'s no-entropy constraint, and
   it catches the design oracle 1 cannot expose: a draw taken and afterwards restored would leave every stream
   byte-identical and still be wrong.

3. **Constructed adjacency, for exact arithmetic.** Internal-tier tests place two Mokiterions at stated positions with
   stated attributes and resolve one action, asserting the resulting attribute values against numbers computed from
   `SPEC-MOK-001`'s stated function **on paper**, not from the engine's own output. Every boundary each requirement
   names is constructed rather than waited for: `health` `1`, `energy` `0`, `satiety` `0`, `satiety` `100`, `fear` `100`,
   distance `0`, distance `1`, distance `2`, a target that died earlier in the same tick, an attacker that moved out of
   contact, and a suffered-attack record holding two entries.

   These tests are the only place the arithmetic is checked, and they are explicitly **not** evidence that anything
   happens in a run.

4. **Whole runs, for reachability.** Over the declared seed set at the default density under the `social` source, the
   event stream is counted: encounters, each of the seven targeted actions proposed and applied, rejections by reason,
   deaths by cause, and survivors at tick 1,000. Every one of the seven verbs must appear as an applied action somewhere
   in the matrix, and at least one death attributable to combat must appear on **every** seed. A verb that never applies
   in any run is an unreachable rule, and this oracle is what would say so.

5. **An identifier-symmetry pair and an identifier-monotonicity band.** The risk `INT-MOK-010` records — that
   deterministic resolution plus ascending-identifier acting order advantages `M01` — is bounded from two directions,
   because neither direction is sufficient alone.

   The mechanism check is exact: an internal-tier test resolves one constructed encounter, then resolves the identical
   encounter with the two Mokiterions' identifiers exchanged, and asserts the damage dealt and the resulting attributes
   are identical. Damage is a function of the striker's `energy` and `health` and of nothing else, and this is the direct
   form of that claim.

   The outcome check has two parts, and they are separated because they need different amounts of data. Over the
   declared seeds at the default density under the `social` source, per-identifier survival counts and per-identifier
   counts of attacks applied and attacks suffered must not form a monotone non-increasing sequence in identifier: that
   is the gross-advantage tripwire, five seeds carry it, and it is what catches `M01` surviving every seed while `M12`
   survives none. Separately, over a **declared diagnostic seed set** stated in this contract, the survival rate by
   turn position within a Mokiterion's own territory must lie inside a **magnitude bound** stated in this contract.

   **Amended 2026-08-20.** This check previously bounded the rank correlation between identifier and each series at
   `±0.5` over the five declared seeds, and that bound was measured to be unsupportable in either direction — the
   evidence is `evidence/WO-MOK-016/escalation.md` §11. It failed at the candidate on a real ordered advantage, and
   five seeds could neither establish nor refute one: the same statistic that reads `+0.586` there reads `+1.000` on the
   covariate that carries the effect, and a correlation over twelve points is underpowered at five seeds and saturated
   at a thousand. Rank correlation is therefore no longer bounded anywhere in this contract. It is **recorded** as
   evidence on both covariates with the event totals it was computed from, because a correlation is only interpretable
   beside them, and because the measurement is the deliverable `INT-MOK-010` asks for even where no threshold reads it.

6. **An enumeration of reads, for `REQ-MOK-059`.** Every rule of `SPEC-MOK-001` that governs a Mokiterion's behavior,
   every `DecisionSource` implementation, and every proposal-validation path is enumerated with what it reads, and each
   read is resolved to the acting Mokiterion's own state, its observation, or a named individual the rule acts on. Rules
   14 to 16 and 18 are enumerated too and recorded as outside the obligation with the reason, rather than omitted. The
   enumeration is retained as evidence and is the artifact the assurance owner signs, because a grep for a forbidden
   string cannot verify an absence that has no canonical name.

7. **The governance state of the artifacts this change amends.** Every amendment `WO-MOK-016` names must be present and
   approved before this contract can be satisfied — the `SPEC-MOK-001` amendments, the `SPEC-MOK-002` and `SPEC-MOK-003`
   amendments, and the amendments to `REQ-MOK-005`, `REQ-MOK-014` and `REQ-MOK-034`. Their absence fails this contract
   regardless of the state of the code, for the reason `VER-MOK-006` gives: an amendment nobody approved is not a
   specification.

   This oracle also carries the row `VREC-MOK-010` left standing. `VER-MOK-010` is recorded as satisfied but for its
   `VREC-MOK-005` gate row, and that gate is a precondition of this work rather than an output of it, exactly as
   `VER-MOK-011` states for itself.

8. **The delegation equality, observation by observation.** The technical owner decided on 2026-08-20 that the `social`
   source's fallback is `individual`'s and is reached by delegating to rule 19 rather than by reimplementing it. That
   decision buys this contract an oracle that costs nothing to run: on any observation carrying no perceived living
   Mokiterion and an empty suffered-attack record, `social` and `individual` must propose the **identical** action, and
   must leave the shared stream at the same position. **Widened by `REQ-MOK-057`'s amendment of 2026-08-20**: with rule 19's
   case 3 hoisted above the social branches, the equality now also holds on any observation where a tolerated resource is
   perceived and the record is empty, whether or not a Mokiterion is perceived. The oracle is asserted in both forms, and the
   widened form is the stronger of the two because it covers observations the narrow form excluded. An internal-tier test consults both sources on the same
   observation and asserts equality — over constructed observations covering each of rule 19's cases 1 to 4, and over
   observations captured from whole runs at every opportunity where both preconditions hold.

   The equality is stated **per observation and never per run**, and the reason belongs here rather than in a reviewer's
   head. Because `REQ-MOK-057`'s branches 4 and 5 pre-empt rule 19's case 4, a `social` run takes **fewer** draws
   than an `individual` run on the same seed and diverges from it at the first opportunity where one Mokiterion perceives
   another and no tolerated resource is perceived. A whole-run byte equality between the two sources would hold only in a
   world where no Mokiterion ever perceives another without also perceiving a meal, which at twelve Mokiterions and a
   perception radius of `16` is not a reachable run. A reviewer who
   reached for whole-run equality here would find it failing and would be looking for a defect that is not there.

   This is appended as an eighth oracle rather than folded into oracle 3 or oracle 4, for `WO-MOK-010`'s renumbering
   reason: the seven above are cited by number in every row of the matrix below.

The declared verification seed set is `0`, `1`, `42`, `123` and `777`, fixed by `VER-MOK-002` and reused unchanged, so
that this contract's survivor measurements are comparable at matched seeds with `REQ-MOK-014`'s and `REQ-MOK-034`'s.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-051` | automated-test | Contact at each distance (oracle 3) | Two living Mokiterions at Chebyshev distance `0` and `1` are in contact; at `2` and above they are not; asserted at every axis and diagonal, computed from positions on paper |
| `REQ-MOK-051` | automated-test | Contact across the territory boundary (oracle 3) | Mokiterions at `y=63` and `y=64` in one column are in contact, and no crossing event is emitted by the contact itself |
| `REQ-MOK-051` | automated-test | The dead are in contact with nobody (oracle 3) | A Mokiterion adjacent to one that died earlier in the same tick is not in contact with it, and a targeted proposal against it is rejected |
| `REQ-MOK-051` | automated-test | Contact is recomputed, never stored (oracle 3) | Moving either Mokiterion changes the relation with no state cleared or updated; no per-pair structure exists to hold it |
| `REQ-MOK-051` | static-analysis | One distance notion | Contact uses the same Chebyshev distance rule 3's perception uses; no second distance function, and the contact radius is a specification constant with no configuration input |
| `REQ-MOK-051` | static-analysis | No observation field added by contact | The observation gains no contact flag; the distance that decides it is the one rule 3 already carries |
| `REQ-MOK-052` | automated-test | Exactly once (oracle 3) | Each of the seven actions, applied, mutates the stated state once; no action applies twice, to a second target, or on a later tick |
| `REQ-MOK-052` | automated-test | Rejection mutates nothing, on both sides (oracle 3) | For every rejection case each requirement names, both Mokiterions' `health`, `satiety`, `energy`, `fear` and position are unchanged, and the opportunity is consumed |
| `REQ-MOK-052` | automated-test | Preconditions are checked against authoritative state (oracle 3) | A target in contact when the observation was built and dead or at distance `2` at validation time yields a rejection naming the unmet precondition |
| `REQ-MOK-052` | automated-test | `approach` and `avoid` are moves (oracle 3) | One cell, one cardinal axis, rule 8's contract, no additional energy cost, crossing event emitted as usual; the fallback axis is used where the preferred one is invalid |
| `REQ-MOK-052` | automated-test | `avoid` from co-location (oracle 3) | A co-located `avoid` moves one cell in the direction `SPEC-MOK-001` fixes and is not rejected |
| `REQ-MOK-052` | automated-test | Answers require an attack (oracle 3) | `fight`, `retreat` and `surrender` are rejected for a Mokiterion whose suffered-attack record is empty |
| `REQ-MOK-052` | automated-test | Every verb applies somewhere in the matrix (oracle 4) | Each of the seven appears as an applied action at least once across the declared seeds under the `social` source |
| `REQ-MOK-052` | automated-test | Acting order untouched (oracle 4) | Ascending identifier order, one opportunity per living Mokiterion per tick, over full runs; no Mokiterion acts twice and none acts out of order |
| `REQ-MOK-052` | static-analysis | The contract is closed | Eleven action kinds, no extension point, no generic interact verb, no data-driven action table; tie-breaks resolve by lowest identifier |
| `REQ-MOK-052` | automated-test | The trace reports targeted actions (oracle 4) | With `--trace-actions`, one line per decision opportunity, after mutation and before decay, for targeted actions as for core ones |
| `REQ-MOK-052` | automated-test | **The trace line's two conditional fields** (oracle 4) | `target` appears immediately after `proposal` on exactly the seven targeted verbs and on no other line, carrying the identifier the proposal named — on the technical owner's decision of 2026-08-20, against rendering it inside `proposal`. `suffered` appears after `fear` on exactly the lines whose record is non-empty. A `wait`, `sleep`, `eat` or `move` line under `social` with an empty record is byte-identical in shape to the same line under `reference`, and `proposal:move:south` keeps its inner colon |
| `REQ-MOK-052` | static-analysis | No parser reads this line positionally past `proposal` | Every suite and every retained script that parses `action_trace` addresses details by name, or is named in the evidence with its assertion count before and after. `target`'s insertion shifts `status` by one field on targeted lines, and this row is what makes that shift safe rather than assumed safe |
| `REQ-MOK-053` | automated-test | **Exact damage, from the specification on paper** (oracle 3) | For a stated grid of striker `energy` and `health`, the target's resulting `health` equals `10 + (energy + health) / 10` with the division truncating toward zero, computed by hand, and the striker's resulting `energy` is reduced by a flat `5` with saturation |
| `REQ-MOK-053` | automated-test | The damage range is `10..=30` (oracle 3) | The corners are asserted: a striker at `energy` `0` and `health` `1` deals `10`; one at `100` and `100` deals `30`; the four-to-ten-strike lethality this implies is asserted against a full-health target |
| `REQ-MOK-053` | automated-test | Damage floor (oracle 3) | No resolution deals `0`. At the decided function the floor is satisfied by construction — the constant term is `10` — so this row constrains a later amendment to the function rather than the current one |
| `REQ-MOK-053` | automated-test | Damage reads nothing else (oracle 3) | Varying the target's attributes, either identifier, the tick number and either trait leaves the damage unchanged |
| `REQ-MOK-053` | automated-test | Saturation (oracle 3) | `health` saturates at `0`, never negative; `energy` cost saturates at `0`; every attribute stays within `0..=100` over full runs |
| `REQ-MOK-053` | automated-test | Death by combat uses rule 13's path (oracle 3) | A target reduced to `0` health is marked dead with the existing death event, takes no later opportunity in that tick or after, and the resolution event records that it died |
| `REQ-MOK-053` | automated-test | Death by combat is distinguishable in the stream (oracle 4) | Deaths attributable to combat are countable from output alone, separately from starvation and exhaustion, on every declared seed |
| `REQ-MOK-053` | automated-test | Mutual destruction within one tick (oracle 3) | Both Mokiterions may die in one tick; the dead take no opportunity; no resolution is attributed to a dead striker |
| `REQ-MOK-053` | automated-test | Both parties' transitions on one event (oracle 4) | Every resolution event carries striker, target, damage, resulting `health`, resulting `energy` and whether the target died — for targets both above and below the striker's identifier |
| `REQ-MOK-053` | automated-test | **No entropy** (oracle 2) | The shared stream's state is equal either side of every resolution, and the post-initialization state equals `VER-MOK-007`'s recorded expectation, unchanged |
| `REQ-MOK-053` | automated-test | Determinism (oracle 4) | Byte-identical output on repeated runs at every declared seed under the `social` source |
| `REQ-MOK-053` | static-analysis | Integer arithmetic only | No floating point anywhere in resolution; no new attribute, no derived stat, no intermediate scale |
| `REQ-MOK-054` | automated-test | `fear` is carried (oracle 3) | The observation carries the observer's own `fear` in `0..=100`, and carries no other Mokiterion's attributes |
| `REQ-MOK-054` | automated-test | **`fear` is read** (oracle 3) | Two observations identical but for `fear` yield different proposals under the `social` source. This is the row that discharges rule 12's "no rule reads `fear`" |
| `REQ-MOK-054` | automated-test | Zero-tick and one-tick latency (oracle 3) | A defender with a higher identifier than its attacker answers within the same tick; one with a lower identifier answers on the next; both asserted on constructed states and observed in a run |
| `REQ-MOK-054` | automated-test | The window closes on the opportunity (oracle 3) | After a defender's source is consulted — answering, proposing something else, or having its proposal rejected — the record is empty at its next opportunity |
| `REQ-MOK-054` | automated-test | Multiple attacks in one window (oracle 3) | All are carried in resolution order; answering one closes the window on both |
| `REQ-MOK-054` | automated-test | The record dies with its Mokiterion (oracle 3) | A Mokiterion killed by the attack never reads the record and no state outlives it |
| `REQ-MOK-054` | static-analysis | Per-Mokiterion, not per-pair | No structure indexed by pairs of Mokiterions; no lifetime counter, no last-attacker field outliving the opportunity, no grudge |
| `REQ-MOK-054` | static-analysis | No name, no aggregate, no foreign attribute | The record names attackers by identifier only; `REQ-MOK-041` is unaffected; nothing on the observation summarizes a set |
| `REQ-MOK-054` | static-analysis | The observation stays read-only | No source mutates it and none holds a handle to authoritative state; rule 3's contract is unchanged |
| `REQ-MOK-055` | automated-test | Exact `fear` increase (oracle 3) | The target's `fear` rises by `30` — a constant of its own, not rule 12's `FEAR_INCREASE` of `10` — saturating at `100`, computed on paper |
| `REQ-MOK-055` | automated-test | A threat outlasts its tick (oracle 3) | `30` takes six quiet ticks to shed at rule 12's `FEAR_DECREASE` of `5`, asserted tick by tick on a Mokiterion that perceives nobody afterwards |
| `REQ-MOK-055` | automated-test | Nothing else changes (oracle 3) | No `health`, `satiety`, `energy` or position of either Mokiterion changes; the threatener pays nothing |
| `REQ-MOK-055` | automated-test | Saturation at the maximum succeeds (oracle 3) | A target at `fear` `100` is threatened validly, stays at `100`, and the event reports an effective increase of `0` — not a rejection |
| `REQ-MOK-055` | automated-test | Composition with rule 12 (oracle 3) | A Mokiterion threatened and then updated by rule 12 in the same tick ends with both writes composed in turn order, saturating; asserted for both the `+10` and the `-5` case |
| `REQ-MOK-055` | automated-test | A threat is not an attack (oracle 3) | A threatened Mokiterion's suffered-attack record stays empty and its `fight`, `retreat` and `surrender` are rejected |
| `REQ-MOK-055` | static-analysis | `fear`'s writers are enumerated | `fear` is written only by initialization, rule 12, and this rule; the enumeration is retained |
| `REQ-MOK-056` | automated-test | Exact transfer (oracle 3) | `satiety / 2`, the division truncating toward zero, leaves the surrendering Mokiterion and arrives at the recipient, both saturating, computed on paper — `40` from `80`, `10` from `20`, `0` from `1` |
| `REQ-MOK-056` | automated-test | Surrender is free below `satiety` `2` (oracle 3) | A Mokiterion at `satiety` `0` or `1` transfers `0` and the action succeeds; at `2` and `3` it transfers `1`. The cheapest answer is available to the Mokiterion least able to pay, which is the decided consequence of a proportion and not a rounding defect |
| `REQ-MOK-056` | automated-test | **The cap and the discard** (oracle 3) | A recipient at `satiety` `100` receives `0`, the surrendering Mokiterion still pays, and the event reports the discarded amount. Non-conservation is asserted, not tolerated silently |
| `REQ-MOK-056` | automated-test | Nothing left to give (oracle 3) | A surrendering Mokiterion at `satiety` `0` transfers `0` and the action succeeds |
| `REQ-MOK-056` | automated-test | No damage, no cost, no movement (oracle 3) | Neither party's `health`, `energy`, `fear` or position changes |
| `REQ-MOK-056` | automated-test | Contact is not required (oracle 3) | A surrender to an attacker that has moved out of contact is valid; one naming a Mokiterion absent from the record, or dead, is rejected |
| `REQ-MOK-056` | automated-test | No immunity (oracle 3) | A Mokiterion that surrendered may be attacked and may surrender again on the next tick, paying again |
| `REQ-MOK-056` | automated-test | Starvation stays rule 12's (oracle 3) | A surrender that empties `satiety` produces death, if at all, through the existing path with the existing cause; no event attributes death to the surrender |
| `REQ-MOK-056` | static-analysis | Transfer is unique to surrender | No other rule moves any attribute between Mokiterions, and rule 9's non-waste condition is unmodified |
| `REQ-MOK-057` | automated-test | `--policy social` (oracle 4) | The value `social` is accepted; usage text, long help and invalid-value diagnostic all name the four values; the diagnostic keeps the existing configuration-error exit code |
| `REQ-MOK-057` | automated-test | The source is reported (oracle 4) | `decision_source_selected` names the `social` source exactly once per run |
| `REQ-MOK-057` | automated-test | **The branch ordering is normative** (oracle 3) | Constructed observations that make two branches applicable at once resolve to the earlier branch: an unanswered attack beside a tolerated co-located resource yields the answer; a tolerated resource beside a Mokiterion in contact yields `eat`; `energy` below `REFERENCE_SLEEP_THRESHOLD` beside a Mokiterion in contact yields `sleep`; a Mokiterion in contact beside one perceived at distance yields the contact branch |
| `REQ-MOK-057` | automated-test | **The answer thresholds** (oracle 3) | A defender proposes `surrender` at `fear` `60` and above, `retreat` from `30` to `59`, `fight` below `30`, against the **first** attacker in the record; asserted at `29`, `30`, `59` and `60`, and with a record holding two attackers |
| `REQ-MOK-057` | automated-test | **The engagement threshold** (oracle 3) | A Mokiterion with a living Mokiterion at Chebyshev distance `1` or less proposes `attack` below `fear` `95` and `threaten` at `95` and above; one perceiving another at distance `2` or more proposes `approach` below `95` and `avoid` at `95` and above; asserted at `94` and `95`, and at `30` and `60` to prove the gate is **not** either answer threshold. The value was `30` as first approved and moved under `REQ-MOK-057`'s amendment of 2026-08-20 |
| `REQ-MOK-057` | automated-test | **One threat flips both branches** (oracle 3) | A Mokiterion at `fear` `10`, threatened once under `REQ-MOK-055` and so at `40`, proposes `threaten` where it would have proposed `attack` and `avoid` where it would have proposed `approach`. This is the row that makes `REQ-MOK-055`'s composition claim good |
| `REQ-MOK-057` | automated-test | Tie-breaks (oracle 3) | Nearest first, then lowest identifier, on both the contact branch and the perceived branch, asserted with two candidates equidistant and with two at different distances |
| `REQ-MOK-057` | automated-test | **An unanswerable attack is still answered** (oracle 3) | Branch 1 proposes against the first attacker in the record without re-checking rule 21's preconditions, on the technical owner's decision of 2026-08-20. Asserted in both cases the ordering admits: an attacker dead at the answering turn, and an attacker out of contact where `fear` below `30` selects `fight`. Each yields a rule 6 rejection naming the unmet precondition, mutates neither Mokiterion, spends the opportunity, and clears the record under rule 25 |
| `REQ-MOK-057` | automated-test | The spent turns are counted, not assumed rare | Over the declared seeds at the default density, the count of branch-1 proposals rejected for a dead or out-of-contact target is measured and retained. No obligation is stated on the figure; it is the evidence the decision above returns to if the population floor is missed |
| `REQ-MOK-057` | automated-test | **The `fear` the source reads** (oracle 3) | The value read is the one standing after the previous tick's rule 12 write, plus any threat applied by an earlier-acting Mokiterion in the current tick; asserted with the threatener's identifier below the reader's and above it |
| `REQ-MOK-057` | automated-test | **The delegation equality** (oracle 8) | On every observation carrying no perceived living Mokiterion and an empty suffered-attack record, `social` proposes what `individual` proposes, action for action, and both leave the shared stream at the same position |
| `REQ-MOK-057` | automated-test | **At most one draw, and never for a social decision** (oracle 2) | Branches 1, 4 and 5 leave the shared stream where they found it; branches 2 and 3 draw nothing; branch 6 draws exactly what rule 19's case 4 draws, from the same position. **Branch 3 drawing nothing is the load-bearing assertion of `REQ-MOK-057`'s amendment**: it is what makes hoisting rule 19's case 3 above the social branches a reordering of decisions rather than of the shared stream |
| `REQ-MOK-057` | automated-test | `social` consumes no more draws than `individual` (oracle 4) | Over the declared seeds, the total draw count under `social` is at most the count under `individual`, and the two runs' divergence begins at the first opportunity where a Mokiterion is perceived |
| `REQ-MOK-057` | static-analysis | Branch 2 does not delegate and inspect | Rule 19's cases 1 and 2 are tested directly. No path consults rule 19 and discards its result, which would take a draw for a decision never used and move the shared stream silently |
| `REQ-MOK-057` | static-analysis | Three new constants and no more | `60`, `30` and `30`. No new survival constant, no new hunger or fatigue threshold, and no duplicate of `REFERENCE_SLEEP_THRESHOLD` |
| `REQ-MOK-057` | static-analysis | The dependency runs one way | `social` calls rule 19's implementation; rule 19's implementation is unchanged and gains no parameter, no tolerance argument and no branch for `social`'s benefit, on the precedent `simulation.rs` records for why rule 5's selectors were not generalized |
| `REQ-MOK-057` | automated-test | The default is unchanged (oracle 1) | A run with no `--policy` is byte-identical to the pre-change `reference` capture, except as `REQ-MOK-060` accounts for |
| `REQ-MOK-057` | automated-test | Determinism (oracle 4) | Byte-identical output on repeated runs, at every declared seed, density and trace setting |
| `REQ-MOK-057` | automated-test | **`baseline` is byte-identical, unprojected** (oracle 1) | All 30 `baseline` cells identical byte for byte with identical exit codes, with no projection applied |
| `REQ-MOK-057` | automated-test | `reference` and `individual` diverge only as accounted (oracle 1) | Each of the 60 cells' divergence is characterized to `REQ-MOK-060`'s corrected condition by first diverging tick, first diverging record and the decision that changed |
| `REQ-MOK-057` | static-analysis | The source reads its observation and nothing else | No handle to authoritative state, no state between opportunities, no population aggregate; traits reach it as they reach `individual`, through the side generator |
| `REQ-MOK-057` | static-analysis | Interface growth is counted | One `Policy` variant, seven `Action` variants, and three `EventType` variants with their details, enumerated against `SPEC-MOK-002` rule 5 and approved in its amendment. Corrected 2026-08-20: the observation's two fields were listed here and are **not** interface growth, because `Observation` is private — see *Static and architecture checks*. The observation's valid-proposal list is checked to have **not** grown, since growing it would move `baseline`'s draw |
| `REQ-MOK-058` | automated-test | **Survivor floor** (oracle 4) | At least five of twelve living at tick 1,000, at the default density, under the `social` source, on every declared seed. Five, not six: the product owner lowered it on 2026-08-20 against the decided damage function, and `REQ-MOK-058` records what the move cost |
| `REQ-MOK-058` | automated-test | **Lethality** (oracle 4) | At least one death attributable to combat on every declared seed, counted from the stream |
| `REQ-MOK-058` | automated-test | Both bounds on each seed (oracle 4) | No seed satisfies one bound by failing the other; the per-seed table is retained |
| `REQ-MOK-058` | automated-test | The carried floors still hold (oracle 4) | `REQ-MOK-014`'s eight of twelve under `reference` and `REQ-MOK-034`'s eight of twelve under the trait-aware source, re-measured on the declared seeds against the corrected world |
| `REQ-MOK-058` | automated-test | Survivors are recorded whether or not the floor is met (oracle 4) | The per-seed count is retained even on a seed that fails, because the curve is what the floor is ratified on and a suite that stops at the first failure produces no curve |
| `REQ-MOK-058` | review | The floor was not bought | No damage, forfeit, threat, survival or resource constant was changed to reach the floor without a governed amendment; `baseline` still reproduces. The `social` source's own ordering and thresholds are the one legitimate lever, and a change to them is a governed change to `REQ-MOK-057` rather than a tuning commit |
| `REQ-MOK-059` | static-analysis | **The enumeration of reads** (oracle 6) | Every behavior-governing rule, every source and every validation path resolves each read to the acting Mokiterion's own state, its observation, or a named individual; rules 14 to 16 and 18 recorded as outside the obligation with the reason |
| `REQ-MOK-059` | static-analysis | No aggregate on the observation | No living count, population total, territory census, world food total or set summary is carried or reachable |
| `REQ-MOK-059` | static-analysis | No count-dependent branch | No rule's behavior depends on how many Mokiterions exist, are alive, are in contact or are in a territory; no threshold on a count, no scaling by population |
| `REQ-MOK-059` | automated-test | Perception is not a census | Rule 12's dependence on whether the observation lists a Mokiterion is unchanged and satisfies the bound; a run reproduces its `fear` trajectory exactly |
| `REQ-MOK-059` | review | The prohibited shape is absent | No path suppresses, weakens or gates combat on a population condition. This is the row that would catch a survivor floor met by making the engine stop the fighting |
| `REQ-MOK-060` | automated-test | **The ceiling** (oracle 4) | At tick 1,000, in each territory, no calorie class holds more than three fifths of that territory's standing resources, on every declared seed, under `reference`, `individual` and the `social` source. Exactly three fifths passes; the breach is a share strictly above it |
| `REQ-MOK-060` | automated-test | Measured from rule 18's summary (oracle 4) | The composition is read from the run's own final summary, with no new event and no new instrumentation |
| `REQ-MOK-060` | automated-test | The pre-change state is reproduced as the contrast (oracle 1) | The same measurement at the pre-change commit puts high class above three fifths in a territory, which is the recorded 45 of 61 — `73.8%` — this requirement ends |
| `REQ-MOK-060` | static-analysis | **The correction is where it is permitted to be** | The change touches rule 5's and rule 19's waste condition only; rule 4, rule 9's eat effect, the food table and rules 14 to 16 are unchanged, compared line for line against the pre-change commit |
| `REQ-MOK-060` | static-analysis | No composition read by any source | No decision source reads the class composition of a territory or of the world |
| both | automated-test | **Identifier symmetry** (oracle 5) | One constructed encounter resolved with the two identifiers exchanged yields identical damage and identical resulting attributes |
| both | automated-test | **Identifier monotonicity tripwire** (oracle 5) | Per-identifier survival, attacks applied and attacks suffered are not monotone non-increasing in identifier, over the five declared seeds |
| both | automated-test | **Turn-position survival bound** (oracle 5) | Over the declared diagnostic seed set, the survival rate by turn position within a Mokiterion's own territory varies by less than the magnitude bound stated below. The rank correlations are recorded with their event totals and bound nothing |
| both | automated-test | New event types are authorized | Each of the three added `EventType` variants maps to an authorizing requirement under `SPEC-MOK-003` rule 11 — `attack_resolved` to `REQ-MOK-053`, `threat_resolved` to `REQ-MOK-055`, `surrender_resolved` to `REQ-MOK-056` — and `mokiterions-tui`'s authority table gains those three rows with `EventType::ALL` covered exhaustively. `REQ-MOK-052` takes no row: `approach`, `avoid` and `retreat` emit no type of their own |
| both | automated-test | Prior coverage preserved, name by name | Every test name present at the pre-change commit is present and passing at the candidate; every difference is an addition; no case removed, renamed away or `#[ignore]`d |
| both | automated-test | Test-tier placement | Every added test is in the public tier only if writable through the library target's public interface with its assertions unchanged; no item widened to `pub` to relocate a test, per `SPEC-MOK-004` |
| both | static-analysis | **Required amendments present and approved** (oracle 7) | The `SPEC-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-003` amendments and the `REQ-MOK-005`, `REQ-MOK-014` and `REQ-MOK-034` amendments `WO-MOK-016` names are approved. Absence fails this contract regardless of code state |
| both | static-analysis | The `VREC-MOK-005` gate | The row `VREC-MOK-010` records as outstanding is a precondition of this work, not an output of it, and its state is recorded here |

## The stated monotonicity tripwire and turn-position bound

The outcome check of oracle 5 needs two numbers, and each needs its power stated with it.

**Amended 2026-08-20**, on the product owner's decision to accept the measured turn-order advantage as a property of the
world and the assurance owner's decision on what this row should bound. The section it replaces stated a single
rank-correlation band of `±0.5` on identifier over the five declared seeds. What that band was measured to do, and why
it is gone, is `evidence/WO-MOK-016/escalation.md` §11; the short form is that it was underpowered at five seeds and
pointed at the wrong covariate, so it could fail loudly on noise and pass quietly on a real advantage.

### Part one: the gross-advantage tripwire, unchanged

Over the five declared seeds at the default density under the `social` source, three per-identifier series are computed:
survivals to tick 1,000, attacks applied, and attacks suffered. **No series may be monotone non-increasing in
identifier.** A series in which every lower identifier does at least as well as every higher one is the failure this
oracle exists to catch, and it fails the row outright.

Five seeds and twelve identifiers support exactly this and no more. Its purpose is to fail loudly on a gross advantage —
`M01` surviving every seed while `M12` survives none — and it will not detect a small one. That limit is why part two
exists rather than why part one is weakened: the tripwire is kept at the declared seeds so that this row stays
comparable with `REQ-MOK-014`'s and `REQ-MOK-034`'s, and the measurement that needs more data is given its own set.

### Part two: the turn-position survival bound

**The declared diagnostic seed set is the 200 seeds `0` through `199` inclusive**, at the default density under the
`social` source, at 1,000 ticks. It is a diagnostic set and carries no survivor floor, no lethality bound and no
comparability obligation; `REQ-MOK-058`'s and `REQ-MOK-014`'s obligations stay on the five declared seeds and are not
evaluated here. Declaring a second set is what the replaced section forbade, and the reason it gave — that "a seed set
chosen to make this row pass would no longer be comparable" — is answered by separating the two roles rather than
overridden: the set that carries the obligations is untouched, and this set carries no obligation to be comparable to.

Two hundred is measured rather than chosen, and it is measured against two failures. The bound's own statistic is a
spread and so cannot change sign; the direction diagnostic beside it is the last-to-first ratio, and the 1,000 seeds
were partitioned into disjoint groups to see what each size supports:

| Group size | Groups | Bound's statistic, high to low | Groups at or above `1.25` | Last-to-first ratio | Groups putting the last actor ahead |
|---|---:|---|---:|---|---:|
| 50 | 20 | `1.068` – `1.284` | **2 of 20** | `0.934` – `1.221` | 16 of 20 |
| 100 | 10 | `1.058` – `1.201` | 0 of 10 | `0.981` – `1.201` | 8 of 10 |
| **200** | **5** | **`1.032` – `1.137`** | **0 of 5** | **`1.010` – `1.137`** | **5 of 5** |

**On the five declared seeds this bound reads `1.2857` and would fail**, at the same candidate whose advantage the
product owner accepted, on ten survival opportunities per position. That is the whole argument for a second set in one
figure: keeping the bound on five seeds would not have been strictness, it would have been a row that fails on a world
the owner has accepted and cannot be made to pass by any correct implementation.

At 50 the groups disagree about which way the advantage runs *and* two of twenty would breach the bound on noise alone.
At 100 they no longer breach it but still straddle `1.0` on direction. At 200 every group agrees on the direction and
the worst group clears `1.25` by `0.113`. **The bound and the set size are one provision and not two**: `1.25` is not a
statement about this world that holds at any sample size, and evaluating it on fewer seeds would make it fail falsely.

Mokiterions are placed six to a territory by identifier — `M01` to `M06` in A, `M07` to `M12` in B — and contact is
overwhelmingly within a territory, so **turn position within a Mokiterion's own territory** is the covariate that
carries turn order's effect. Positions `1` through `6` are `M01`…`M06` and `M07`…`M12` respectively, pooled.

- **The highest survival rate across the six turn positions, divided by the lowest, must be less than `1.25`.**

**Where `1.25` comes from, and what it is not.** It is not read off the curve it bounds. `REQ-MOK-034` binds the
trait-aware source at eight of twelve and `REQ-MOK-058` binds this source at five, so three of twelve — a quarter of the
population — is the survivor cost the product owner accepted combat may impose. A turn-order advantage worth more than
that whole quarter would be a structural asymmetry rather than a residue of the acting order, and that is the line. The
measured value on the declared set at the candidate is **`1.082`** — survival by position `0.7850  0.7325  0.7500
0.7350  0.7625  0.7925` — and `1.063` over all 1,000 seeds measured, so the bound holds with margin it did not choose.
Its sensitivity is stated rather than assumed: a world where the last actor survived at 90% and the first at 70% would
read `1.286` and fail, and the gross case part one catches reads `0` or unbounded.

**The ratio is of extremes across all six positions and not of the last to the first**, so it bounds any pair rather
than presuming which position is favored. That is stricter and it is deliberate: the measured direction is that later
actors are favored, and an oracle that encoded that direction would stop being able to catch its reversal.

### What is recorded and bounds nothing

Both rank correlations — against identifier `1`…`12` and against turn position in territory — are computed on all three
series and printed with the totals they were computed from, on both seed sets. They are evidence. `INT-MOK-010`'s
success measure *Win rate and survival by identifier* is met by part one, whose target is "not monotonic in identifier";
the correlations are what let a reader see how far from monotonic, and in which direction.

**The measured direction is the opposite of the one `INT-MOK-010` anticipated**, and that is recorded here rather than
corrected there. Its risk reads "deterministic resolution plus ascending-identifier acting order may hand `M01` a
systematic advantage"; the measurement is that `M01` is *disadvantaged* — first-acting positions strike less, are struck
more, and survive slightly less. That is exact over the 1,000-seed sweep, where position 1 holds the lowest survival
rate of the six; on the declared 200 position 1 is second-highest and the extremes are positions 6 and 2, so the effect
is directional in aggregate rather than exactly ordered at every position on every set. The bound is written for that:
it takes the extremes wherever they fall. The risk's remedy stands unchanged, because it says the advantage "is bounded by
measurement rather than assumed away", which is what this section does. Whether that artifact's stated direction should
be amended is the intent owner's and is **not** decided here.

## Acceptance scenarios

1. A reviewer captures the 90-cell three-source matrix at the commit the work begins from, captures it again at the
   candidate, and finds the 30 `baseline` cells byte-identical with identical exit codes and no projection applied.
2. A reviewer takes each of the 60 `reference` and `individual` cells, locates the first diverging record, and confirms
   it is an eat-or-seek decision the corrected waste condition explains — then confirms the same for a seed of their own
   choosing outside the declared set.
3. A reviewer computes the resolution of a stated encounter with pencil and paper from `SPEC-MOK-001`, runs the
   constructed-state test, and finds the same numbers.
4. A reviewer perturbs the resolution to draw one value from the shared stream, confirms oracle 2 fails and the 30
   `baseline` cells fail, and reverts. A check that cannot be made to fail has not been demonstrated to work.
5. A reviewer perturbs the resolution to read the striker's identifier, confirms the identifier-symmetry test fails, and
   reverts.
6. A reviewer runs the declared seed set under the `social` source and finds, on each seed, at least five survivors and
   at least one death attributable to combat, and finds all seven verbs applied somewhere across the set.
7. A reviewer reads rule 18's final summary on each seed and confirms no class holds more than three fifths of either
   territory's standing resources, then confirms the same measurement at the pre-change commit exceeds three fifths.
8. A reviewer adds a hypothetical rule that suppresses combat below seven living Mokiterions, confirms the enumeration
   of oracle 6 names it as a violating read, and reverts. This is the demonstration that `REQ-MOK-059` is checkable.
9. A reviewer constructs a target that dies during an earlier Mokiterion's turn, confirms the later attack against it is
   rejected with a reason naming the unmet precondition, and confirms neither Mokiterion's state changed.
10. A reviewer constructs a defender whose identifier is above its attacker's and one whose identifier is below, and
    finds the first answering in the same tick and the second on the next.
11. A reviewer constructs a recipient at `satiety` `100`, lets a surrender resolve, and finds the whole forfeit reported
    as discarded and the surrendering Mokiterion charged in full.
12. A reviewer runs the same seed twice under the `social` source and finds byte-identical output.
13. A reviewer confirms every named amendment is approved, and records the state of the `VREC-MOK-005` gate.
14. A reviewer builds an observation carrying no perceived Mokiterion and an empty suffered-attack record, consults
    `social` and `individual` on it, and finds the same proposed action and the same shared-stream position — then
    repeats the comparison at every opportunity in a whole run where both preconditions hold, and finds no exception.
15. A reviewer restructures branch 2 to delegate to rule 19 and inspect its result rather than testing rule 19's cases 1
    and 2 directly, confirms that oracle 2 fails because a draw was taken and then discarded, and reverts. This is the
    demonstration that the ordering's entropy discipline is checked and not merely asserted.
16. A reviewer takes a Mokiterion at `fear` `10` in contact with another, threatens it once, and finds its next proposal
    changed from `attack` to `threaten` — the whole of `REQ-MOK-055`'s claim that a threat changes what another
    Mokiterion decides.
17. A reviewer reads the branch distribution retained as evidence and records whether it is degenerate, which is manual
    assessment 8 whichever way it comes out.

## Property and invariant tests

- **Bounds.** Every attribute of every Mokiterion stays within `0..=100` at every tick of every declared run, including
  through combat, threats, forfeits and death.
- **Determinism of the whole engine.** `REQ-MOK-009`'s byte-identical reproducibility holds at every declared seed under
  every one of the four sources.
- **Entropy neutrality of the encounter path.** The shared stream's state is a function of seed, density and the world's
  own evolution alone; no resolution, validation, contact test, observation build or window clear moves it.
- **One action per opportunity.** Over full runs, the count of applied actions per living Mokiterion per tick is at most
  one, and the count of decision opportunities equals the count of living Mokiterions considered.
- **Death is final and singular.** Every death emits exactly one death event, and no dead Mokiterion appears in any
  later observation, decision, action, trace, survival update or resolution.
- **Contact symmetry.** For every pair and every tick, the contact relation computed from positions is symmetric, while
  its registration is ordered by identifier — the asymmetry rule 12 documents, asserted rather than assumed.
- **Window boundedness.** No suffered-attack record survives its Mokiterion's next decision opportunity, at any tick, on
  any seed.
- **`fear` has exactly three writers**, and the set is closed: initialization, rule 12, and the threat rule.
- **Fallback equality.** At every decision opportunity of every declared run, an observation carrying no perceived living
  Mokiterion and an empty suffered-attack record yields the same proposal under `social` as under `individual`. Asserted
  as a property over the captured observations rather than on a chosen few, and it is the whole of `REQ-MOK-057`'s
  delegation claim.
- **At most one draw per opportunity under `social`**, over full runs, and none at an opportunity where the source
  proposed one of the seven targeted actions.
- **Attribute transfer conservation, stated as its failure.** For every surrender, the amount leaving the surrendering
  Mokiterion equals the amount received plus the amount reported as discarded. Conservation of `satiety` across the
  population is deliberately **not** a property, and this identity is what replaces it.
- **Composition ratio.** At tick 1,000, per territory, per class, the ratio of standing resources of that class to the
  territory's standing total is at most three fifths — asserted as a property over the summary rather than sampled.
- **No aggregate reaches a decision.** Asserted structurally by oracle 6's enumeration, and behaviorally by the absence
  of any run in which two Mokiterions in identical local situations behave differently because the population differs.

## Static and architecture checks

- `cargo fmt --all -- --check` clean.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` clean, with no `allow` attribute added
  and no lint suppressed to accommodate this change.
- `cargo test` at the workspace root runs every tier of both packages in one invocation, with no feature, environment
  variable, ignore attribute, extra command, terminal or working-directory dependence.
- The engine's dependency and dev-dependency tables are empty, with no exception, and `cargo tree -p Mokiterions`
  resolves to the engine package alone.
- The observer's dependency set is one path dependency on `Mokiterions` and `ratatui` at the pinned version and feature
  set, with nothing added.
- No new package, no new target, no build script, and no change to any package, library or binary name.
- **The engine's public interface growth is enumerated and matches the approved `SPEC-MOK-002` amendment item for item**
  — one `Policy` variant, seven `Action` variants, and three `EventType` variants with their three details — and nothing
  else grows. `EventType::ALL` goes from twelve entries to fifteen, and the observation's valid-proposal list keeps its
  type, because it does not grow.

  **Re-checked 2026-08-20 and corrected: the observation's two new fields are not part of this count.** They were listed
  here, and listing them was wrong for the reason `WO-MOK-016` records against its own `SPEC-MOK-002` provision 1:
  `mokiterions-core/src/simulation.rs:500` declares `struct Observation {` with no `pub`, and `SPEC-MOK-002` rule 6 lists
  `Observation` among the ten names that stay private. A field added to a private type is not public-surface growth, and
  a check that counted it would be asserting against a specification amendment that does not claim it. The fields are
  not thereby unverified: `REQ-MOK-054`'s rows above check that each is carried, that `fear` is read, that no foreign
  attribute or aggregate rides along, and that the observation stays read-only. What this check loses is one item; what
  it gains is that the count it asserts and the amendment it asserts against are the same list.
- `EventType::ALL`'s length equals the number of variants, asserted against the variant set rather than against a
  literal, so that a variant added without a table row fails here.
- No public item yields a mutable borrow of, or a reference into, authoritative state, in any build configuration
  including test builds. `SPEC-MOK-002` rule 6 is re-checked because cross-agent mutation is introduced for the first
  time and because the observation gains fields.
- No floating-point type or operation appears anywhere in resolution, contact, transfer or the `social` source. The
  damage function's `/ 10` and the forfeit's `/ 2` are integer divisions truncating toward zero.
- Test placement follows `SPEC-MOK-004`. Every internal-tier test added here names the private item it requires, and no
  item is widened to `pub` to relocate a test.
- **Architecture review is required rather than waived, and its form is decided.** Cross-agent mutation is a new pattern:
  until this change, every rule wrote the acting Mokiterion's own state. The technical owner decided on 2026-08-20 that
  this is a **recorded confirmation that `ARCH-MOK-001` is satisfied unchanged**, rather than a new `ADR` — the pattern
  stays inside the engine component, crosses no boundary `ARCH-MOK-001` draws, and introduces no dependency, so there is
  no architectural decision left to record. Deciding the form does not discharge the confirmation: it is still made in
  review against the implemented code, and it remains manual assessment 9.

## Security and privacy checks

- No new input, no new file, no new network access, no new environment dependence and no new credential path. `social`,
  the fourth policy value, is the only new operator input and it is a fixed enumerated value.
- No model-provider credential or other secret enters the repository, in code, in a test fixture, or in retained
  evidence.
- Retained evidence contains simulation output only, about fictional entities.
- **Cross-agent mutation is confined to the engine and does not widen any trust boundary.** The observer gains new event
  types to present and no new authority; `ADR-MOK-001`'s boundary is unchanged, and the observer still decides nothing.
- **A rejected proposal cannot damage a Mokiterion.** Verified as a security-shaped property rather than only a
  correctness one: the only path that writes a second Mokiterion's state is a validated resolution, and the enumeration
  of writers is retained.
- No unbounded structure is introduced. The suffered-attack record is bounded by the number of Mokiterions that can
  strike one Mokiterion within one window, and that bound is asserted.

## Performance and resilience checks

- A 1,000-tick run at the default density completes within the bound `SPEC-MOK-001` states, on each declared seed, under
  each of the four sources.
- A 10,000-tick run under the `social` source completes without panic and without unbounded growth in retained state, and
  its composition and survivor figures are captured as evidence rather than bound as obligations.
- Per-tick work stays bounded by the population and the perception radius. Contact is computed from the perceived list
  the observation already builds, so it adds no scan of the world.
- No arithmetic in this change can panic in a debug build. Every subtraction and addition on an attribute is saturating,
  and the forfeit's cap is asserted rather than assumed.
- The observer's frame remains within the bound `SPEC-MOK-003` states with the new event types ingested.

## Manual assessments

Each of the following is an explicit judgement recorded by the accountable role. An unrecorded assessment is an
outstanding assessment, and this contract is not satisfied while any remains outstanding.

Eight of the eleven had their subject **decided in advance, on 2026-08-20**, before any measurement of this initiative
existed: the damage function and the strike cost, the forfeit, the threat's magnitude, the survivor floor, the
composition ceiling, the source's name, its decision rule, and the form the architecture review takes. Each decision is
stated in the requirement that owns it and is reproduced here so that a reviewer reads it rather than reconstructs it.

**Deciding a value in advance does not discharge the assessment.** What those eight now ask of their owner is a
ratification against the measured evidence: keeping the decided value or amending the requirement that states it are
both acceptable outcomes, and silence is not either of them. The numbering is preserved unchanged, because assessments
8, 9 and 10 are cited by number elsewhere in this contract.

1. **The damage function and the striker's energy cost, by the product owner.** Decided: damage is
   `10 + (striker.energy + striker.health) / 10`, the division truncating, giving the range `10..=30` and four to ten
   strikes to kill a full-health target; the strike costs a flat `5` `energy`. What remains is the ratification, and it
   has a specific question attached: `REQ-MOK-053` records that a flat `5` is deliberately a **weak** brake on repeated
   striking, so the owner ratifies having seen the measured strikes per encounter and the measured lethality, not only
   the survivor total.
2. **The forfeit's magnitude and its non-conservation, by the product owner.** Decided: `satiety / 2`, truncating, so
   `40` from a Mokiterion at `80` and `0` from one at `1`. The owner ratifies having seen the measured frequency of two
   cases the arithmetic creates — the full recipient, where the forfeit is paid and destroyed rather than returned, and
   the band below `satiety` `2`, where surrender is free — and records that the destruction is intended in both.
3. **The threat's magnitude, by the product owner.** Decided: a new constant of `30`, distinct from rule 12's
   `FEAR_INCREASE` of `10` and not pinned to `ATTRIBUTE_MAX`. The owner ratifies having seen how often a free action
   with an effect was proposed, which is the exposure `REQ-MOK-055` records about making a threat cost nothing.
4. **The survivor floor of five and the lethality bound, by the product owner**, on the first measured curve. Decided at
   five rather than six, pre-measurement, because of the damage decision taken in the same session. `REQ-MOK-058`
   records that five sits exactly on the line its own six-argument drew, so this ratification is doing real work rather
   than confirming a comfortable number: a curve that clusters at five or six is itself a finding about the damage
   function, and amending `REQ-MOK-058` upward is as available an outcome as ratifying it.
5. **The composition ceiling of three fifths, by the product owner**, on the measured corrected composition, together
   with the judgement on whether a per-class floor is also wanted. Ratified in advance at one half against `60%` and
   `40%`, for the `17` points of headroom above the balanced initial third that it leaves, and **amended to three fifths
   on 2026-08-21 on the first measured curve**, which showed one half unreachable inside `REQ-MOK-060`'s own permitted
   surface while the three carried survivor floors hold. What remains to be assessed here is therefore not the value —
   `REQ-MOK-060`'s amendment record closes it, and this contract does not reopen a requirement — but two things the
   approval measurement could not settle: that three fifths is met on the shipped build with the margin the amendment
   claimed, and whether the corrected composition warrants a per-class floor. A curve that meets three fifths only by
   sitting on it is a finding about the mechanism, and a further amendment is as available an outcome as ratifying the
   result.
6. **The additivity cost, by the product owner.** `reference` and `individual` outcomes move. The owner records that this
   is accepted, having seen the characterized divergence, and approves the narrowing of `REQ-MOK-034`'s frozen-outcomes
   clause to `baseline` alone.
7. **The fourth source's name, by the product owner.** Decided: `social`, from four candidates. Because it appears in
   `--help`, in the invalid-value diagnostic and in every captured stream, the assessment that remains is not the choice
   but the check that the decided name landed in all three places identically and that no captured evidence carries an
   earlier working name.
8. **The defender's decision rule, by the technical owner.** Decided: the six-branch ordering of `REQ-MOK-057`, survival
   first and then a perceived meal before society, with the defender answering `surrender` at `fear` `60`, `retreat` at `30`
   and `fight` below `30`, and the aggressor engaging below `95`. It was a five-branch ordering with the gate at `30` as
   first approved; the assessment now also records that the amendment of 2026-08-20 was taken on measurement rather than on
   preference, and that `SPEC-MOK-001` rule 12 was left as Phase 2 approved it. The owner records whether the measured branch distribution is degenerate — every
   defender surrendering, or every defender fighting, or a social branch that never fires — and if it is, that finding is
   the deliverable and not a failure, per `INT-MOK-010`. `REQ-MOK-057` leaves one thing genuinely open for this
   assessment to inform: whether branch 1 should be able to decline to answer at all.
9. **Cross-agent mutation, by the technical owner.** Decided: a recorded confirmation that `ARCH-MOK-001` is satisfied
   unchanged, rather than an `ADR` deciding the pattern — the mutation stays within the engine component, crosses no
   boundary `ARCH-MOK-001` draws, and adds no dependency. The confirmation itself is made in review against the
   implemented code and is outstanding until recorded; if the implementation turns out to need a boundary moved, the
   decision reverts to an `ADR` and this assessment says so.
10. **The read enumeration of oracle 6, by the assurance owner.** The owner confirms that the enumeration is complete —
    that no rule, source or validation path is missing from it — because an enumeration's value is entirely in its
    completeness, and no automated check can establish that.
11. **The monotonicity band's adequacy, by the assurance owner.** The owner records that the band is a coarse tripwire,
    what it would and would not catch, and that the identifier-symmetry test is what carries the mechanism claim.

    **Assessed 2026-08-20, and it was not adequate.** The band was measured against 9,194 resolved strikes and 12,000
    identifier-runs over 1,000 seeds — `evidence/WO-MOK-016/identifier.md` — and found to be
    both underpowered and aimed at the wrong covariate: it failed at the candidate on a real advantage it could not
    characterize, and correcting only its covariate would have made it pass by `0.007` while the same statistic reads
    `+1.000` on a thousand seeds. The assessment is therefore that a rank correlation over twelve points cannot carry
    this claim at any width, and it has been replaced — part one keeps the tripwire where five seeds support it, part two
    bounds the survival consequence on a set large enough to measure it, and the correlations are recorded without
    bounding anything. The identifier-symmetry test still carries the mechanism claim and still passes; the advantage
    that remains is turn order's, which is identifier-blind, and the product owner accepted it as a property of the world
    on the same date. `evidence/WO-MOK-016/escalation.md` §11 is the evidence this assessment was waiting for.

## Evidence retention

Retained under `docs/engineering/simulation/evidence/WO-MOK-016/`:

- the pre-change capture of the 90-cell three-source matrix, taken before any code change from a tracked-clean worktree,
  with the commit recorded; retained by per-cell SHA-256 digest, with a stated subset retained whole;
- the post-change capture of the same 90 cells, retained the same way, with the byte comparison of the 30 `baseline`
  cells and the characterized divergence of the 60 others;
- the new 30-cell capture under the `social` source, with exit codes;
- the captured exit code of every cell, both sides;
- the constructed-state resolution tables: inputs, the value computed from the specification on paper, and the value the
  engine produced, for damage, energy cost, threat increase and forfeit, including every boundary case;
- the shared stream's recorded state either side of every resolution kind, and the reused `VER-MOK-007` initialization
  expectations;
- per-seed tables of survivors, deaths by cause, encounters, each verb proposed and applied, and rejections by reason,
  retained on failing seeds as well as passing ones;
- the per-observation comparison of `social` against `individual` at every opportunity where no living Mokiterion is
  perceived and no attack is unanswered, carrying both proposals and the shared stream's position either side, together
  with the total draw count per run under each source at matched seeds;
- the branch distribution under `social` per seed — how often each of `REQ-MOK-057`'s six branches fired, and how often
  the answer branch chose `surrender`, `retreat` and `fight` — which is the evidence manual assessment 8 is taken on. The
  count reads six rather than the five this list carried as first approved, because the amendment of 2026-08-20 hoisted
  rule 19's case 3 into a new branch 3; the item itself is unchanged, as the second amendment row states;
- the measured strikes per encounter, the measured frequency of a forfeit discarded at a full recipient, and the measured
  frequency of a surrender in the free band below `satiety` `2`, which is what manual assessments 1 and 2 are taken on;
- the per-identifier series and the computed rank correlations, with the band evaluation;
- the identifier-exchange comparison for the constructed encounter;
- rule 18's final summary per seed under each of the four sources, and the composition ratio computed per territory per
  class, at both commits;
- the line-for-line comparison showing rule 4, rule 9's eat effect, the food table and rules 14 to 16 unchanged;
- the enumeration of reads for every behavior-governing rule, every source and every validation path, with rules 14 to
  16 and 18 recorded as outside the obligation;
- the enumeration of `fear`'s writers and of every path that writes a second Mokiterion's state;
- the engine public-interface enumeration at both commits, compared item for item against the approved `SPEC-MOK-002`
  amendment;
- the observer authority table's new rows and the `EventType::ALL` exhaustiveness check;
- the pre-change and post-change workspace test census, reconciled name by name;
- `cargo fmt`, `cargo clippy`, `cargo test` and `cargo tree -p Mokiterions` output;
- the 10,000-tick run's completion, composition and survivor figures, as evidence and not as an obligation;
- the eleven manual assessments above, each with its accountable role and date;
- the amendment-approval check of oracle 7, including the recorded state of the `VREC-MOK-005` gate.

Evidence is retained in the repository, is reproducible from the recorded commands and commit, and contains no secret.

## Residual uncertainty

- **The identifier-advantage bound is weak where it remains weak, and the study it deferred was commissioned.** Five
  seeds and twelve identifiers cannot support a strong claim about a distributional advantage, so part one of oracle 5's
  outcome check is a gross tripwire and nothing more. The identifier-symmetry test closes the mechanism — damage does not
  read an identifier.

  **Amended 2026-08-20, with this contract's third amendment row.** This bullet said that a modest systematic advantage
  arising from turn order rather than from the damage function "would be visible only in a larger study this contract
  does not commission". That study was commissioned and run, and it found the advantage: it is turn order's, it runs
  toward *later* actors rather than toward `M01`, and part two now bounds it at `1.25` over a declared 200-seed
  diagnostic set where it measures `1.082` — `1.063` over the 1,000 seeds measured in
  `evidence/WO-MOK-016/identifier.md`. What stays residual is narrower and is three things rather than one. The bound is
  a spread over six pooled turn positions, so an advantage distributed evenly across all six would not register in it.
  The diagnostic set carries no survivor floor and no lethality bound, so a regression that moved outcomes on that set
  without breaching `1.25` fails no row here. And the claim that the advantage is identifier-*blind* still rests on one
  constructed encounter with the two identifiers exchanged; the sweep bounds the consequence of turn order and does not
  independently establish that nothing reads an identifier.
- **Exact arithmetic is verified on constructed states, and constructed states are not the world.** Oracle 4 establishes
  that each verb applies somewhere and that combat kills on every seed; it does not establish that every arithmetic
  branch verified in oracle 3 is reached in a natural run. The count of each branch reached in whole runs is retained so
  that the gap is visible rather than assumed away.
- **Additivity is verified over the declared matrix, not over the input space.** A change that moved `baseline` only at a
  density outside the sweep, or only past tick 1,000, would pass oracle 1. Oracle 2 reduces this by asserting entropy
  neutrality directly, and the line-for-line comparison of the world rules reduces it further, but neither is a proof.
- **The divergence attribution for `reference` and `individual` is a characterization, not a derivation.** A reviewer
  confirms that the first diverging record is explained by the corrected waste condition. A second, independent cause
  that happened to first show itself at the same record would be absorbed into that explanation. The per-cell first
  divergence is retained so the claim can be re-examined.
- **`REQ-MOK-059` is verified by enumeration, and an enumeration can be incomplete.** Its completeness is manual
  assessment 10. Nothing in this contract can prove that no reader was omitted from a list written by hand.
- **Every number in this chain was decided before measurement existed.** Not only the survivor floor of five and the
  composition ceiling — one half as first stated — but the damage function, the strike cost of `5`, the threat of `30`,
  the forfeit of `satiety / 2` and the three thresholds `60`, `30` and `30` — all decided on 2026-08-20 so that the work
  order had obligations to verify against. `REQ-MOK-014`'s amendment record is the precedent for expecting such numbers
  to move. A first measurement that misses a bound is evidence for an amendment, and this contract's rows are written so
  that the amendment is visible as a governed decision rather than absorbed as a threshold adjustment. This is recorded
  as one residual rather than seven, and manual assessments 1 to 5 and 8 are where it is discharged. **One of the seven
  has now moved**: the composition ceiling went to three fifths on 2026-08-21, before any engine change, on a measurement
  that showed one half unreachable inside the surface its own requirement permits. That is this residual behaving as
  written rather than an exception to it, and it raises rather than lowers the standing of the six that have not moved —
  the mechanism by which a pre-measurement number becomes a governed amendment has now been exercised once and can be
  inspected.
- **The delegation equality checks the branches that are not new.** Oracle 8 establishes that `social` behaves as
  `individual` wherever nothing is perceived and nothing is unanswered. It says nothing at all about the four branches
  that are the point of this initiative: those have no prior source to be compared against and rest entirely on oracle
  3's constructed states and oracle 4's counts. The oracle is cheap and it is narrow, and it must not be read as
  coverage of the source.
- **Whether the defender's choice is meaningful is measured, not verified.** `CAP-MOK-010` withholds perceived relative
  strength. If the branch distribution is degenerate on every seed, every row here can pass while the most interesting
  decision in the world is a foregone conclusion. That is manual assessment 8 and it is a finding, not a defect.
- **This contract does not close `VREC-MOK-005`.** Its outstanding amendments and assessments, and the amendment rows
  left outstanding by earlier work, are a precondition of this work under oracle 7, not an output of it.
