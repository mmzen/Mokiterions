# WO-MOK-007 completion summary

The sixteen sections `WO-MOK-007`'s *Completion report format* asks for, in its order and under its headings.

Two of them do not report success. **Section 2** reports that the gate the work order set was overridden rather than
met, and **section 14** reports that five of seven manual assessments are outstanding and a sixth is unsigned. Both are
stated where the format puts them rather than deferred to section 16, because a reader who stops after section 6 should
already know that no verification record can be written against this commit.

---

## 1. Scope statement

Implemented, under `INT-MOK-006` → `CAP-MOK-006` → `REQ-MOK-031`…`REQ-MOK-034` → `SPEC-MOK-001`/`002`/`003` as amended:

- **A fixed behavioral trait per Mokiterion.** `waste_tolerance`, in `0..=40`, derived once at initialization from the
  seed and the identifier by a generator of the derivation's own, reported once in `agent_initialized`, and never
  written again.
- **A `fear` attribute**, `0..=100`, starting at `0`, rising `+10` on any tick the Mokiterion's own observation lists at
  least one other living Mokiterion and falling `-5` otherwise, saturating at both bounds, reported in
  `survival_changed` and on the observation snapshot, and read by nothing.
- **A third decision source**, `--policy individual`, specified as `SPEC-MOK-001` rule 19 and appended rather than
  renumbered. It is rule 5 with one difference: a resource whose restoration would overshoot satiety is accepted when
  `S + R - 100 <= T * R / 100` in truncating integer arithmetic, and the same test governs seeking as governs eating.
- **A fourth roster gauge** in the observer, `f`, on the same footing as `h`, `s` and `e`.

**Nothing outside *In scope* changed.** No second trait, no consumer for `fear`, no change to `BaselineDecisionSource`
or `ReferenceDecisionSource`, no dependency, no new package, target or build script, and no public interface item beyond
the two the work order names. The two scope cuts the owner accepted — no per-Mokiterion trait display in the observer
and no trait in the event vocabulary beyond the initialization record — were not implemented and are recorded as
declined in `docs/mokiterions/ROADMAP.md`.

Four changes were made that the *In scope* list does not name and that are reported here rather than left to a diff:

1. `mokiterions-tui/src/options.rs`'s usage text advertised `--policy <baseline|reference>` while forwarding whatever
   the engine accepts. Adding a third value would have left the observer documenting two of three. Corrected, with a
   test that asserts the general property.
2. `mokiterions-tui/src/authority.rs`'s decision-source mapping is exhaustive over `Policy`, so a third variant is a
   compile error there, not a choice. `REQ-MOK-033` was added as the third arm, and two inherited public-tier tests in
   `mokiterions-tui/tests/authority.rs` were extended to cover it — see section 12.
3. `mokiterions-tui/tests/verification.rs`'s `no_shipped_decision_source_has_a_proposal_rejected` swept two policies.
   A third source shipped, so the sweep was extended to it rather than leaving the case's name broader than what it
   checked.
4. A `.gitattributes` was added, with one line disabling end-of-line conversion for
   `docs/engineering/simulation/evidence/`. Without it no digest in this packet reproduces from a Windows clone.
   Sections 15 and 16 carry the check and the disclosure.

## 2. The gate

**The gate was not met. It was overridden by the repository owner on 2026-08-19, and this section reports that rather
than a satisfied condition.**

`WO-MOK-007` requires "Evidence that `VREC-MOK-005` was `verified`, its six amendments approved and its seven
assessments recorded, before implementation began, with dates". The true state, at the commit this work started from
(**60fda9f**) and at the commit it ends at:

| What the gate required | State before implementation began | State now |
|---|---|---|
| `VREC-MOK-005` at `verified` | `ready` | `ready`, unchanged |
| Its six amendments approved | six **OUTSTANDING** across `SPEC-MOK-002`, `SPEC-MOK-003` and `ARCH-MOK-001` | six **OUTSTANDING**, unchanged |
| Its seven manual assessments recorded | none recorded | none recorded, unchanged |

The override is recorded as the tenth row of `WO-MOK-007`'s *Decision record*, dated 2026-08-19, taken by the
repository owner acting as product, technical and assurance owner in answer to a blocking question from the
implementation agent. The agent did not proceed past the gate on its own reading.

The mitigation the owner accepted was that the two layers stay separable by inspection. That claim is checked rather
than asserted: `amendment-approvals.md` §4 compares every amendment row dated before 2026-08-19 against **60fda9f**
and finds each byte-identical, and records that `VREC-MOK-005` itself is untouched. **It is a cost carried forward, not
a debt paid**, and `VER-MOK-007`'s own matrix row for it — "the amendments left outstanding by `VREC-MOK-005` are
resolved, before this change is verified" — is unsatisfied. That row is reproduced in
`requirement-to-test-mapping.md` in place.

## 3. Approval record

**The governing chain.** All eight artifacts of the pack, approved by the repository owner on **2026-08-19**:

| Artifact | Role that approved | Date |
|---|---|---|
| `INT-MOK-006` | product owner | 2026-08-19 |
| `CAP-MOK-006` | product owner | 2026-08-19 |
| `REQ-MOK-031` … `REQ-MOK-034` | product owner | 2026-08-19 |
| `SPEC-MOK-001`, `SPEC-MOK-002`, `SPEC-MOK-003` (amendments) | technical owner | 2026-08-19 |
| `VER-MOK-007` | assurance owner | 2026-08-19 |
| `WO-MOK-007` | engineering owner | 2026-08-19, left at `in_progress` |
| `ARCH-MOK-001` confirmation — boundaries, prohibited patterns and the dependency prohibition satisfied unchanged, no amendment required | technical owner | 2026-08-19 |

`amendment-approvals.md` §1 reads each artifact's front matter and confirms `status` and `approved` agree.

**The amendments the owner approved as a list.** Nine provisions plus one appended rule in `SPEC-MOK-001`, two rule-5
entries in `SPEC-MOK-002`, three rule-4 provisions in `SPEC-MOK-003`. Every one is present in both the amendment record
and the specification body, checked over disjoint text; two amend by deletion and are checked by locating the sentence
and asserting its contents against an anchor at the far end of the list. All 7 self-tests on those checks held.
**In every case the implementation agent wrote the text and did not decide the substance**, and each amendment row says
so.

**The technical owner's two design decisions**, both taken 2026-08-19 before implementation:

- **Rule ordering** — rule 19 appended rather than inserted next to rule 5, so no existing rule is renumbered and no
  inherited citation moves.
- **Roster bar width** — the narrowing from three gauges to four accepted, `BAR_ROW_OVERHEAD` rising from 27 to 35.

**The three amendments written during implementation, beyond that list.** Named in full in `amendment-approvals.md` §3
and in each specification's own amendment record:

| Amendment | State |
|---|---|
| `SPEC-MOK-001`: the trait range narrowed from `0..=100` to `0..=40`, with rule 19's upper-bound note and two acceptance examples | **Approved.** The owner, as technical owner, decided it on 2026-08-19 when stop condition 6 fired; `escalation.md` holds the measurement it was decided on |
| `SPEC-MOK-001`: the *Help output* sentence this work order's own first amendment added, corrected — the clause requiring the prose to state the default is withdrawn | **OUTSTANDING.** A correction to text the technical owner approved on 2026-08-19; needs that owner's ratification. See section 13 |
| `SPEC-MOK-003`: three provisions outside rule 4 — `AgentSnapshot` gains `fear`, rule 10 item 7 loses `fear` and traits, rule 11 gains `REQ-MOK-033` | **OUTSTANDING.** Forced by the change rather than chosen with it, written into the 2026-08-19 row rather than made quietly; needs the technical owner's ratification |

Oracle 5's first condition holds and its second does not. `amendment-approvals.md` returns **PASS** on the first and
states the second unmet rather than counting it.

## 4. The additivity result

**Baseline commit: 60fda9faffbd452752a34efa356f16cc6ad1d3ff.**

**The matrix.** Seeds 0, 1, 42, 123, 777 × the frozen sources `baseline` and `reference` × densities 0.75% and 1.50% ×
with and without `--trace-actions` = **40 declared cells**, plus a no-argument run and a 20-tick traced run, captured as
42 frozen-source cells. Under the new source the same grid was captured **twice per cell** — 20 cells, 40 runs — as
`REQ-MOK-009`'s determinism evidence.

**The projection**, in `baseline/projection.py`, is three anchored patterns that delete only the three added fields:
`,fear:<n>` and `,waste_tolerance:<n>` from `agent_initialized`, `,fear:<a>-><b>` from `survival_changed`, and
`,fear:<n>` from the `action_trace` tail. Its full text is quoted in `manual-assessment.md` §6.

**The comparison outcome, per combination:**

| Check | Cells | Result |
|---|---|---|
| `proj(post) == pre` under `--policy baseline` and `--policy reference` | 42 | 42 of 42 equal, **zero differing bytes** |
| `proj(pre) == pre` — the projection is a no-op on the pre-change stream | 42 | 42 of 42 True |
| Exit codes and stderr byte counts, pre against post | 42 | identical |
| New-source cells byte-identical across two OS processes | 20 | 20 of 20 |
| New-source cells differing from the reference source's | 20 | "differing cells: 20 of 20" |

**No difference was found, so there is none to report in full.** `post/additivity.txt` carries the per-cell digests.

Two disclosures belong to this section rather than to section 16:

- **Eleven of the forty declared cells were captured before the first line of code changed**; the other 31 were
  captured afterwards from a clean git worktree at the same commit. That is a recapture, and oracle 1 forbids
  recapturing the baseline *to resolve a discrepancy*. It was not used to resolve one — no discrepancy existed and
  those 31 cells were never covered by the original capture — but the distinction rests on the recapture being from the
  same world, so `baseline/recapture-check.txt` compares the eleven shared cells byte for byte. All eleven match. **A
  reader who does not accept that argument should read oracle 1 as covering eleven cells, not forty.**
- **The post-change capture came from a binary that then fell behind the source three times**: `cargo fmt` reformatted
  the engine source, one test was later moved out of the crate's `#[cfg(test)]` block into the public tier, and one
  `debug_assert!` invariant was tightened. Each has a reason it cannot matter; none is relied on. The tree was rebuilt
  from the committed source and the whole matrix captured again — **all 83 shared cells byte-identical**
  (`baseline/rebuild-check.txt`).

## 5. Entropy neutrality

**The direct assertion.** `simulation::tests::trait_derivation_leaves_the_shared_stream_where_it_found_it` records the
shared stream's position after initialization as a draw count measured against a fresh generator seeded identically:
**72 draws** for twelve Mokiterions at the declared density, asserted as a literal, at every declared seed and density.
Trait derivation constructs `SplitMix64::new(seed ^ number.wrapping_mul(TRAIT_SALT))`, uses it and drops it, so the
shared stream is never passed to it.

**The check was demonstrated capable of failing.** `negative-control/oracle-2.txt` perturbs the derivation to
`derive_waste_tolerance(entropy.next_u64(), number)` — one draw from the shared stream per agent — and records the
result:

- three tests failed, and the named ones are the three the defect predicts;
- the draw count became **84 against the recorded 72**: twelve extra, one per agent, which is the magnitude the defect
  predicts and not merely a failure;
- the perturbation was reverted and `simulation.rs` verified byte-identical by SHA-256 before and after
  (`4850384d0fec95682dadda00d87a53fbeba026474a6916a773058a46927b3671`), and the unperturbed suite passes.

That control also records something adverse about the design of the check: **a re-derived expectation would have
accepted the perturbed trait table.** Only the checked-in literal row rejected it. The sixty recorded trait values are
therefore load-bearing, not decorative.

Independently, oracle 1 corroborates the same claim from the other direction: the `fear` update and the trait
derivation both run under all three sources, so a single extra draw would shift every later value, and 42 frozen-source
cells are byte-identical to the pre-change baseline.

## 6. Measured results

**Survivors, consumption, and standing resources at tick 1,000, density 0.75%** — from `measurements/viability.txt`,
which reads the engine's own `summary` and `food_consumed` records:

| source | seed | survivors | deaths | consumed | territory A | territory B | A high | B high |
|---|---|---|---|---|---|---|---|---|
| individual | 0 | **11** | 1 | 400 | 41 (7/10/24) | 60 (10/13/37) | 58% | 61% |
| individual | 1 | **9** | 3 | 368 | 60 (7/11/42) | 59 (22/16/21) | 70% | 35% |
| individual | 42 | **9** | 3 | 378 | 61 (13/17/31) | 60 (13/11/36) | 50% | 60% |
| individual | 123 | **10** | 2 | 379 | 60 (9/11/40) | 61 (8/14/39) | 66% | 63% |
| individual | 777 | **12** | 0 | 417 | 53 (6/6/41) | 44 (10/8/26) | 77% | 59% |
| reference | 0 | 8 | 4 | 379 | 61 (7/9/45) | 55 (7/16/32) | 73% | 58% |
| reference | 1 | 11 | 1 | 410 | 57 (4/10/43) | 39 (7/7/25) | 75% | 64% |
| reference | 42 | 8 | 4 | 344 | 61 (6/17/38) | 41 (8/7/26) | 62% | 63% |
| reference | 123 | 9 | 3 | 324 | 61 (8/11/42) | 60 (4/12/44) | 68% | 73% |
| reference | 777 | 11 | 1 | 372 | 61 (13/20/28) | 39 (8/13/18) | 45% | 46% |

`REQ-MOK-034`'s floor is **8 of 12 on every declared seed**: met on all five, with two seeds at the margin of one.
Consumption is greater than zero on every seed. **Neither territory reached zero standing resources on any seed under
either source.**

**The trait values behind those results** — `measurements/traits.txt`, read out of each run's own `agent_initialized`
records and independently recomputed from the amended specification:

| seed | M01…M12 | min | max | distinct | expectation agrees |
|---|---|---|---|---|---|
| 0 | 6 8 8 5 4 32 15 10 39 18 20 37 | 4 | 39 | 11 | yes |
| 1 | 26 3 22 39 39 37 2 17 15 16 28 0 | 0 | 39 | 11 | yes |
| 42 | 11 40 4 24 21 13 7 40 24 15 10 23 | 4 | 40 | 10 | yes |
| 123 | 20 33 40 13 35 19 40 35 24 0 19 4 | 0 | 40 | 9 | yes |
| 777 | 36 3 7 10 30 18 36 24 0 22 8 38 | 0 | 38 | 11 | yes |

Nine to eleven distinct values per seed, `0` and `40` both reached across the set. **The floor was therefore not met by
a population that reproduces the reference source**, which is the condition `VER-MOK-034`'s third row exists to check.

**Every survivor figure above is downstream of an owner decision taken mid-implementation.** At the specified
`0..=100` range the floor was missed on three of five declared seeds. `WO-MOK-007` stop condition 6 fired, the agent
stopped and escalated, and the owner — as technical owner — chose to narrow the range to `0..=40` rather than amend the
floor. A fifty-seed sweep put the `0..=100` mean at 7.40 survivors against a floor of 8, and `0..=40` at 9.94 with a 4%
miss rate against the reference source's own 6%. `escalation.md` holds the sweep and the decision.

**Tick 10,000 at the default density** — `measurements/long-horizon.txt`. This horizon carries no obligation in either
direction and none is claimed:

| source | seed | reason | ticks | survivors | consumed |
|---|---|---|---|---|---|
| reference | 0 | extinction | 5,423 | 0 | 1,014 |
| reference | 1 | extinction | 8,273 | 0 | 1,379 |
| reference | 42 | tick_limit | 10,000 | 1 | 1,038 |
| reference | 123 | **extinction at 9,154** | 9,154 | 0 | 1,352 |
| reference | 777 | extinction | 9,598 | 0 | 1,448 |
| individual | 0 | tick_limit | 10,000 | 5 | 2,370 |
| individual | 1 | tick_limit | 10,000 | 4 | 2,035 |
| individual | 42 | tick_limit | 10,000 | 2 | 1,857 |
| individual | 123 | tick_limit | 10,000 | 1 | 1,633 |
| individual | 777 | **extinction at 9,938** | 9,938 | 0 | 1,669 |

**The recorded control reproduces to the tick.** `VER-MOK-002` and `WO-MOK-007` record the reference source reaching
extinction at tick 9,154; that is this table's reference row at seed 123 exactly. It is a frozen-source outcome
reproduced nine times further out than the 1,000-tick oracle's window, on a run that had 9,154 ticks in which to
diverge. Adversely: **seed 777 is the one individual run that goes extinct at this horizon**, at tick 9,938, and it is
also the only seed that retains all twelve at tick 1,000.

**Oscillation** — `measurements/oscillation.txt`, over 1,000 ticks per declared seed:

| | seed 0 | seed 1 | seed 42 | seed 123 | seed 777 | pooled |
|---|---|---|---|---|---|---|
| individual | 11.8% | 10.6% | 11.3% | 10.4% | 10.0% | **10.8%** (6,175/57,068) |
| reference | | | | | | **10.8%** (5,888/54,296) |
| baseline | | | | | | 11.7% (835/7,107) |

Against rule 5's recorded **10.6%** residual and the **12.2%** unbiased-walk rate: no seed exceeds the walk rate, so
`VER-MOK-007`'s finding condition is not triggered. Two things temper that. **The seed-0 margin is 0.008 percentage
points** — 11.815% against 11.823% — which is not a margin to treat as established. And while this measurement
reproduces `WO-MOK-002`'s recorded counts 1,097/10,339 and 174/1,427 exactly, **its own pooled denominators are larger
by exactly 96 in both rows**, one per Mokiterion-run of the eight runs involved, which is what an off-by-one tick
boundary convention looks like. The rates agree to 0.1 percentage point either way.

## 7. Divergence

**The lower-bound equivalence result (oracle 3).** At `T = 0` the trait-aware source is proposal-identical to the
reference source over **2,808 enumerated situations** — enumerated, not sampled. The set is
13 satiety values × 3 resource classes × 18 placements × 2 energy states × 2 companion configurations, and its size is
counted three independent ways that agree (`measurements/equivalence.txt`). Both the **proposal** and the **stream
position** are compared in every situation. Result: **PASS**, every pair equal.

`negative-control/oracle-3.txt` shows that comparison able to fail, twice:

- **Control A** adds `+ 1` to the tolerant bound. It fails on the proposal, at a case where the two sources part.
- **Control B** adds one extra `choose_index` draw inside `IndividualDecisionSource::decide`. It fails **at the very
  first case of the enumeration, on the stream position and not on the proposal** — a resource underfoot at satiety
  `0`, where both sources propose to eat. That is precisely the defect a proposal comparison cannot see, and it is why
  the oracle asserts on both.

**Divergence from a trait difference alone.** `simulation::tests::a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten`
places two Mokiterions at one coordinate, at one satiety, on one resource, differing only in trait: one proposes `eat`
and the other does not, and swapping the two trait values swaps the proposals. Two with equal traits propose
identically.

**Divergence in a real run** — `measurements/divergence.txt`, and **the weakest result in this packet**:

| seed | 0 | 1 | 42 | 123 | 777 |
|---|---|---|---|---|---|
| situations presented / distinct | 449/136 | 408/137 | 401/124 | 410/131 | 454/134 |
| **divergent situations** | **10** | **3** | **3** | **5** | **3** |
| same-tick coincidences | 0 | 0 | 0 | 0 | 0 |
| disagreements / contradictions / unseparated | 0/0/0 | 0/0/0 | 0/0/0 | 0/0/0 | 0/0/0 |
| waste-accepting eats | 73 | 86 | 54 | 97 | 75 |
| distinct eaters | 9 | 9 | 9 | 10 | 9 |

Each divergent instance is recorded with its tick, the two Mokiterions, their traits and their differing proposals, as
`VER-MOK-007` requires. But three to ten per thousand-tick run is **within a factor of three of the figure the contract
itself names as failure**, and **zero same-tick coincidences** means no divergence is ever visible side by side in one
frame. The file also records a counterexample to the naive reading of the trait ordering: at seed 0 a Mokiterion at
tolerance `10` never accepted a resource that one at tolerance `6` did, because the situations they met were not the
same. The 54 to 97 waste-accepting eats by 9 to 10 distinct eaters are the same behavior counted without requiring a
coincidence — but substituting that measure for the one the contract names is the product owner's call, and it is
outstanding as `manual-assessment.md` assessment 2.

## 8. `fear`

**Band, increment and decrement as approved.** Range `0..=100`, saturating at both bounds, initial value `0`.
Increment `+10` on any tick the Mokiterion's own observation lists at least one other living Mokiterion; decrement
`-5` otherwise. The driver is **any perceived living Mokiterion**, with no sensitivity to count, distance or direction,
and **no distance constant of its own** — the owner's decision of 2026-08-19, which is why the perception radius is the
only boundary involved.

**The invariant results**, over `measurements/fear.txt`'s **111,604 agent-ticks** across all declared seeds and all
three sources:

| Invariant | Result |
|---|---|
| Range — every value in `0..=100` on every tick | **0 violations** |
| Step — every transition is exactly `+10`, exactly `-5`, or a saturating truncation of one at a bound | **0 violations**. Observed steps: `-5` = 13,940, `0` = 90,201, `+5` = 219, `+10` = 7,244 |
| Correspondence — the direction of every transition matches whether that Mokiterion's own observation listed another living Mokiterion | **0 violations** |
| The perception boundary | at exactly Chebyshev `16`: 245–427 agent-ticks per seed, **all rose**; at exactly `17`: 238–448 per seed, **all fell** |

Two figures in that table point adversely and are named here rather than in section 16. **The `+5` step occurs 219
times**: it is the saturating truncation of `+10` at `95`, so it is specified behavior, but it is the one magnitude a
reader would not predict from the constants. And **`fear` sits at exactly `100` on 39% of agent-ticks**, with 48% to
62% non-zero — the `+10`/`-5` pair saturates quickly in a world where perception is usually populated.

**One writer and no reader.** `interface-and-purity.txt` counts 24 occurrences of the identifier in the engine's
non-test source: one declaration, one initialization at `0`, **one mutation at `simulation.rs:1962`** (rule 12's), the
snapshot field the observer reads, three event records that report it, and comments. Zero code occurrences inside
`trait DecisionSource`, inside all three `fn decide`, and inside `struct Observation` — the last being where rule 3
states the absence, in the place the next person to add a field will read it.

**This is the section a verifier should discount most heavily, and the reason is structural: nothing reads `fear`.** No
outcome can falsify `+10`/`-5`, so the 39% ceiling residency is an observation and not a defect. What is verified is
that the attribute is maintained, bounded, perception-driven and reported. Whether the constants are right becomes
answerable only when something consumes it. `manual-assessment.md` assessment 4 is where that judgement sits, and it is
outstanding.

## 9. The observer

**Oracle 4** rebuilt **864 bar rows** across **134 viewport renders** from rule 4's named parts — label, space, bar,
space, three value columns, two columns between gauges — rather than from the product, and compared them character for
character. **0 discrepancies** (`observer/roster-frames.txt`).

**The four gauges' cell positions.** Identical at all four roster-drawing viewports, because the interior is the same
45 columns at each:

| gauge | label column | bar columns | value columns |
|---|---|---|---|
| `h` | 6 | 8–9 | 11–13 |
| `s` | 16 | 18–19 | 21–23 |
| `e` | 26 | 28–29 | 31–33 |
| `f` | **36** | **38–39** | **41–43** |

**The measured bar width per viewport.** `bar_width(interior) = min(20, (interior − 35) / 4)`, with
`BAR_ROW_OVERHEAD = 35 = 5 + 4×6 + 3×2`:

| viewport | roster drawn | interior | bar width | form |
|---|---|---|---|---|
| 160 × 48 | yes | 45 | **2** | two-line |
| 160 × 44 | yes | 45 | **2** | two-line |
| 140 × 44 | yes | 45 | **2** | two-line |
| 120 × 48 | yes | 45 | **2** | two-line |
| 100 × 30 | no — tier D on height | — | — | — |
| 34 × 22 | no | — | — | — |
| 33 × 21 | no | — | — | — |

**`VREC-MOK-005` finding 3 is closed by assertion, not by inspection.** The `f` gauge is found at its predicted
absolute columns at every viewport that draws the roster; it is never blank, never a dash, never zero-width, and it
carries the same bar width as the other three. `render::the_fourth_gauge_is_a_proportional_bar_at_zero_and_away_from_it`
asserts the exact rendered line at a computed `0` and away from it, and the values presented are compared against the
snapshot's own `health`, `satiety`, `energy` and `fear` as multisets after thirty ticks, so they are values the engine
had to compute.

**The packet's one reachability finding.** The roster pane is 47 columns wide and rule 4's collapse threshold is also
47, so the drawn roster is *always* two-line at a 45-column interior and **`bar = 2` is the only width reachable
through `render::draw`**. A sweep of 200 widths found bar widths observed = `{2}`, forms = two-line only, and 66 widths
that draw no roster at all. The `min(20, …)` cap and the collapsed one-line form are therefore carried by three named
internal render tests that call the layout arithmetic directly, not by any frame a user can produce. This is a finding
about `SPEC-MOK-003` rule 4, not a defect in the fourth gauge, and it is not amended — the provisions are correct, they
are simply unreachable at the declared viewports.

## 10. Public interface

**The two added items**, from `interface-and-purity.txt`, which enumerates the engine's public surface at **60fda9f**
and at the candidate tree and diffs them:

| | before | after | change |
|---|---|---|---|
| public items | 49 | 49 | +0 |
| public fields | 42 | 43 | **+1** — `AgentSnapshot.fear` |
| enum variants | 47 | 48 | **+1** — `Policy::Individual` |
| removals | | | **0** |

**Nothing else was added.** The trait itself adds nothing: `waste_tolerance` lives on the private `Mokiterion` and on
the private `Observation`, and is reported through the existing event stream. `IndividualDecisionSource`,
`DecisionEntropy` and `Observation` stay private, so `ADR-MOK-001`'s boundary is unchanged: the trait-aware source
proposes and the engine decides.

**Rule 6 still holds.** `SPEC-MOK-002` rule 6 forbids any public item yielding a mutable borrow of, or a reference
into, authoritative state, in any build configuration including test builds. It was re-checked because a new field was
added to a public type: `AgentSnapshot.fear` is a `u8` by value on an owned snapshot, so the snapshot still carries
values only and the new field grants the observer no new authority. The engine's one mutating public operation set is
unchanged.

One honesty note on the contract's own wording: `VER-MOK-007`'s `REQ-MOK-031` row says rule 5's enumeration grows by
the `fear` field alone and its `REQ-MOK-033` row says exactly one option value is added. **Read singly, neither row
describes the census; read together they describe it exactly.** Both hold jointly.

## 11. Static gates

From `static-checks.txt`, regenerated from a fresh capture at the committed source
(`analysis/capture-static.sh`, `analysis/static-checks.py`):

| Gate | Command | Result |
|---|---|---|
| Formatting | `cargo fmt --all -- --check` | **exit 0, 0 diff lines** |
| Lints | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | **exit 0, 0 warning or error lines**, 2 crates re-linted in this run so the result is this tree's and not a cache's. **No `allow` attribute was added and no lint suppressed** |
| Tests | `cargo test --workspace` | **exit 0. 20 runners, 20 ok, 190 passed, 0 failed, 0 ignored, 0 filtered out** |
| Engine dependencies | `cargo tree -p Mokiterions` | **one line** — the package alone. Dependency and dev-dependency tables empty, no exception |
| Observer dependencies | `cargo tree -p mokiterions-tui` | 111 lines, **identical to the pre-change commit's line for line** with the checkout path normalised |

Zero ignored and zero filtered out is the part worth stating: a suite can be made to pass by not running, and those two
counts are what would show it.

**The harness preflight, which is not on `VER-MOK-007`'s retention list and is reported here anyway:**

| Gate | Result |
|---|---|
| `python scripts/validate_engineering_artifacts.py` | **PASS. 76 artifacts, 0 errors, 0 warnings**, across all four planes |
| `bash scripts/check_engineering_harness.sh` | **PASS.** 76 artifacts, 240 relations, 0 errors |
| `python scripts/inspect_engineering_artifacts.py` | 0 errors, **10 warnings, 6 informational** — and **three of those warnings are new, caused by this change** |

The three new ones are all the same `W-HEX-003` shape, and they are honest signals rather than noise:

- `ARCH-MOK-001` (2026-08-18) now predates `SPEC-MOK-001` and `SPEC-MOK-002` (2026-08-19), which it declares
  `conforms_to`;
- `ARCH-MOK-002` now predates `SPEC-MOK-003` for the same reason.

Amending three specifications made all three newer than the architecture artifacts that declare conformance to them,
so the inspector asks for a reassessment of each. **The reassessment was in fact made** — the technical owner
confirmed on 2026-08-19 that `ARCH-MOK-001`'s boundaries, prohibited patterns and dependency prohibition are
satisfied unchanged and no amendment is required, recorded in `WO-MOK-007`'s *Decision record* — but it was recorded
in the work order rather than in `ARCH-MOK-001` itself, so the date comparison still fires. **Neither architecture
artifact was edited to silence it.** Bumping an approved artifact's `updated` field would be a governance act on an
artifact this work order was authorized to confirm and not to amend, and it would have added a third unratified
amendment to fix a warning. The warning is left standing and disclosed instead; closing it is the artifact owner's
act, in `ARCH-MOK-001` and `ARCH-MOK-002`, not this one's.

**The census, reconciled name by name** — `test-census.txt`, against `cargo test --workspace -- --list` captured from a
clean worktree at **60fda9f**:

- 169 tests over 19 runners → **190 tests over 20 runners**; on both sides the number of names listed equals the sum of
  the runners' own declared totals, so nothing is missing from the reconciliation;
- **21 additions, named in full with the runner each runs in; 0 removals**; no runner lost a test and none was emptied;
- **one runner is new** — `decisions (tests/decisions.rs)` — and the census names what moved into it and why, because a
  new public-tier runner is exactly how a relocated test would look.

No new package, no new target, no build script, and no change to any package, library or binary name.

## 12. Test placement

**21 new tests: 13 in the internal tier, 8 in the public tier.** Every internal-tier test is there because it requires
a private item or hook, named below; `SPEC-MOK-002` rule 7 fixes the tier by the access a test requires and states that
"the subject it covers does not decide the tier".

**Internal tier — `mokiterions-core/src/simulation.rs`, `#[cfg(test)] mod tests`:**

| Test | Private item or hook it requires |
|---|---|
| `the_twelve_traits_are_the_recorded_ones_and_are_neither_uniform_nor_out_of_range` | `traits_of` → `Simulation.agents[].waste_tolerance`; `derive_waste_tolerance` |
| `the_trait_reads_the_seed_and_not_only_the_identifier` | `derive_waste_tolerance`, a private free function |
| `trait_derivation_leaves_the_shared_stream_where_it_found_it` | `shared_stream_draws` → `Simulation.entropy`, `Simulation.config`, `SplitMix64.state` |
| `the_trait_is_fixed_for_the_run_and_independent_of_every_configuration` | `traits_of` → `Simulation.agents` |
| `at_tolerance_zero_the_trait_aware_source_proposes_what_the_reference_source_proposes` | `decide_individual_once` → `Simulation::observation`, `IndividualDecisionSource`, `ReferenceDecisionSource`, `DecisionEntropy`; `enumerated_placements` |
| `a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten` | writes `Simulation.agents[].waste_tolerance`, `.position`, `.satiety`, `Simulation.foods`; `decide_individual_once`, `with_companions` |
| `the_tolerant_test_governs_seeking_as_well_as_eating` | the same private writes, plus `WASTE_TOLERANCE_MAX` |
| `fear_rises_at_the_perception_boundary_and_decays_one_cell_beyond_it` | `Simulation::apply_survival`, `with_companions`, `Simulation.agents[].fear` |
| `fear_ignores_how_many_are_perceived_how_far_and_in_which_direction` | the same |
| `fear_saturates_at_both_bounds_and_is_reported_every_tick` | `Simulation::apply_survival`, `Simulation.agents[].fear`, `.alive` |
| `the_trace_reports_the_fear_the_survival_record_then_changes` | `Simulation::apply_survival` and the trace it emits |
| `a_dead_mokiterion_reports_no_fear_and_no_decision` | `Simulation::apply_survival`, `Simulation::run_tick`, `Simulation.agents[].health`, `.satiety`, `.alive`, `IndividualDecisionSource` |
| `individual_runs_are_reported_and_byte_identically_reproducible` | `state_snapshot` → `Simulation.tick`, `.agents`, `.foods`, `.entropy`, `.next_food_id` |

**Public tier — 8 tests, in 5 files:**

| Test | File |
|---|---|
| `the_trait_aware_policy_is_selectable_and_does_not_become_the_default` | `mokiterions-core/tests/cli.rs` |
| `the_trait_aware_source_never_waits_and_proposes_only_valid_actions` | `mokiterions-core/tests/decisions.rs` — **new file** |
| `the_trait_aware_source_runs_to_completion_and_reports_each_trait_once` | `mokiterions-core/tests/process.rs` |
| `a_long_run_is_bounded_under_the_trait_aware_source` | `mokiterions-core/tests/termination.rs` |
| `the_trait_aware_source_sustains_the_population_at_every_declared_density` | `mokiterions-core/tests/viability.rs` |
| `the_usage_text_advertises_every_policy_the_engine_accepts` | `mokiterions-tui/tests/options.rs` |
| `the_roster_presents_four_gauges_at_every_declared_viewport_that_presents_it` | `mokiterions-tui/tests/render.rs` |
| `the_fourth_gauge_is_a_proportional_bar_at_zero_and_away_from_it` | `mokiterions-tui/tests/render.rs` |

**One new file, added on rule 8's own terms** — "a further file may be added when a further public subject appears".
The subject of `mokiterions-core/tests/decisions.rs` is what a decision source proposes over a whole run, which none of
the five files rule 8 lists covers: they cover argument parsing, the process boundary, resolved density, termination and
the population floor.

**One test changed tier during this work order, and it is the second entry in section 13.**
`the_trait_aware_source_never_waits_and_proposes_only_valid_actions` was written inline beside the source it covers and
moved out; its assertions are verbatim across the move, as `SPEC-MOK-002` rule 12 requires, and only the construction
of `Config` changed, from a private test helper to the public struct literal. **No item was widened to `pub` to relocate
it**, which is checkable rather than asserted: `interface-and-purity.txt` finds the same two public additions either
way.

**Tests updated because a record or a type gained a field, enumerated in full:**

| Test | Why | What changed |
|---|---|---|
| `simulation::tests::survival_decay_saturates_and_death_is_final` | `survival_changed` gained `fear` | the expected record text |
| `render::tests::the_bar_row_reproduces_the_specified_form` | the bar row gained a fourth gauge | the exact expected line, and `BAR_ROW_OVERHEAD` 27 → 35 |
| `render::tests::a_zero_value_is_a_zero_and_an_absent_value_is_a_dash` | the same | the exact expected lines, both forms |
| `render::tests::a_bar_row_shrinks_to_its_pane_and_never_overflows_it` | the same | the widths swept and the overhead |
| `verification::the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault` | `AgentSnapshot` gained `fear` | its `contrived` fixture gained `fear: 30` |
| `export::records_use_the_engines_own_line_format_in_authoritative_order` | `survival_changed` gained `fear` | its fixture gained `fear: (12, 22)` and the asserted line gained `,fear:12->22` |
| `cli::the_entries_state_the_constraints_that_decide_validity` | `--policy` gained a value | asserts `individual` is named *and* accepted, both directions |

**Tests extended because a third policy shipped**, which is not a field gaining but is the same obligation:

| Test | What changed |
|---|---|
| `authority::every_event_type_the_observer_can_present_has_an_entry` | the exhaustiveness sweep now runs under all three policies rather than `Reference` alone |
| `authority::the_decision_source_maps_by_the_source_the_record_names` | one added assertion: `individual` → `REQ-MOK-033` |
| `verification::no_shipped_decision_source_has_a_proposal_rejected` | `"individual"` added to the swept policies |
| `process::the_trait_aware_source_runs_to_completion_and_reports_each_trait_once` | one added assertion: twelve `,fear:0,waste_tolerance:` occurrences, discharging `REQ-MOK-032`'s initial-value row |

**No assertion was weakened, no test ignored, no item widened and no `allow` added anywhere in this change.** One
inherited test's name was left alone deliberately: `cli::both_policies_are_selectable_and_reference_is_the_default`
still says "both", because a rename cannot be distinguished from a removal in the census `VER-MOK-007` requires, and
what it asserts is still true. The third value got a named sibling instead.

## 13. Specification mismatches found

Four, in the form `WO-MOK-005`'s completion summary used: what was found, whether it was corrected, and how.

**1. `SPEC-MOK-001`'s *Help output* section contradicted itself, and the contradiction was introduced by this work
order's own first amendment. Corrected in the specification.**

The amendment added a sentence requiring the explanatory prose on the decision sources to "state which one is the
default". The same section's *stated once* paragraph, approved 2026-08-17, requires each declared default to be stated
exactly once — and the inherited public-tier test `cli::each_declared_default_is_stated_once`, bound by a `verified`
`VREC-MOK-004`, asserts that the prose contains no occurrence of "default" at all. The implementation is on the side of
the older paragraph and the test.

Satisfying the new clause would have required relaxing an assertion a verified record binds, which `WO-MOK-007`
explicitly forbids. So **the specification was corrected, not the code and not the assertion**: the default clause is
withdrawn and the sentence now reads that the prose "states no default and no value constraint", with the options block
carrying both. A third amendment row was appended to `SPEC-MOK-001`, marked **OUTSTANDING** and attributed to the
implementation agent, naming the contradiction and both artifacts it sits between. It is a correction to text the
technical owner approved on 2026-08-19 and **needs that owner's ratification**; `amendment-approvals.md` §3 is where
that obligation is recorded. Found by reading the amended specification against the inherited tests while assembling
this section — not by a failing test.

**2. A new test was in the wrong tier. Corrected by relocating it.**

`the_trait_aware_source_never_waits_and_proposes_only_valid_actions` was written inline in
`mokiterions-core/src/simulation.rs`, beside the source it covers. It requires only `Config`, `Policy`, `Density`,
`Simulation::new` and `Simulation::run` — all public — so `SPEC-MOK-002` rule 7 forbids leaving it there: "A test is not
left inline for convenience when rule 5 suffices… the subject it covers does not decide the tier." Relocated verbatim
into the new public-tier file `mokiterions-core/tests/decisions.rs`, with only the `Config` construction changed from a
private helper to the public struct literal. This is a mismatch between the implementation and `SPEC-MOK-002`/
`SPEC-MOK-004`, not a specification defect, and needs no amendment.

**3. A debug invariant admitted values the amended range forbids. Corrected in the code.**

`Observation::is_consistent` gained `self.waste_tolerance <= ATTRIBUTE_MAX` — a bound of 100 — when the trait range was
still `0..=100`. After the owner narrowed the range to `0..=40` the invariant was left at the wider bound, so a
derivation producing `41..=100` would have passed a check whose purpose is to catch exactly that. Tightened to
`WASTE_TOLERANCE_MAX`. Every constructed tolerance in the suite is at or below `40`, so no assertion changed meaning,
and the release binary is unaffected because `debug_assert!` expands to nothing there. **This is the code being weaker
than the specification, not the specification being wrong**, so no amendment follows — but it is reported here because
a reader auditing the narrowing would otherwise find the old bound in the tree.

**4. The observer's usage text documented two of three decision sources. Corrected in the code.**

`mokiterions-tui/src/options.rs`'s `USAGE` advertised `--policy <baseline|reference>` while forwarding the value to the
engine's parser, which now accepts three. `REQ-MOK-018`'s obligation is on the engine's help text, so nothing was
formally violated, but the observer would have accepted an undocumented value. Corrected, and the new test
`options::the_usage_text_advertises_every_policy_the_engine_accepts` asserts the general property — every value the
engine accepts appears in the observer's usage text — so the gap cannot reopen when a fourth source is added.

**Nothing was resolved by a quietly adjusted constraint or a relaxed assertion**, which `WO-MOK-007` forbids. Two of
the four were corrected in code, one in the specification with an outstanding amendment row, and one by moving a file.

## 14. Manual assessments

**Five of seven are outstanding and a sixth is unsigned, so `VER-MOK-007`'s manual-assessment contract is not
satisfied.** The contract is explicit: "An unrecorded assessment is an outstanding assessment, and this contract is not
satisfied while any remains outstanding." Full text in `manual-assessment.md`.

| # | Judgement | Accountable role | Status |
|---|---|---|---|
| 1 | Scarcity at the default density | product owner | **outstanding** |
| 2 | Individuality is meaningful | product owner | **outstanding** — the closest of the seven to the threshold the contract names as failure |
| 3 | The accumulation result | product owner | **outstanding** |
| 4 | `fear`'s constants | technical owner | **outstanding** — carries an adverse figure |
| 5 | Roster legibility at four bars | technical owner | **recorded 2026-08-19** |
| 6 | The projection | assurance owner | **outstanding** |
| 7 | The absence of a `fear` consumer | technical owner | recorded in substance 2026-08-19; **not separately signed** |

Assessment 5's judgement, as recorded: at a bar width of 2 the four gauges are legible because each is labelled and
each carries its numeric value at a fixed column, and the bar is corroboration rather than the reading — the same basis
on which three gauges were accepted under `WO-MOK-006`. Assessment 7's substance is recorded as the census result — one
writer, no reader — but the technical owner has not signed it as an assessment.

**Two of the outstanding judgements carry facts that point adversely**, and `manual-assessment.md` names them at the
top of the file rather than the bottom: assessment 2, where the divergence count is within a factor of three of the
figure the contract names as failure, and assessment 4, where `fear` sits at its ceiling on 39% of agent-ticks.
Nothing in this packet asks the owner to decide either quickly, and the agent has recorded no judgement it is not
accountable for.

## 15. Evidence index

`README.md` maps `VER-MOK-007`'s 15 retention bullets to files. What each file establishes:

| File | What it establishes |
|---|---|
| `README.md` | The retention map, the five oracles, and what none of the evidence establishes |
| `requirement-to-test-mapping.md` | Every one of the contract's 45 matrix rows against the test or file that discharges it, with twelve caveats and the two rows that are not satisfied |
| `completion-summary.md` | This report |
| `escalation.md` | Stop condition 6 firing, the fifty-seed sweep, and the owner's decision to narrow the trait range |
| `manual-assessment.md` | The seven judgements no script can make, five outstanding and one unsigned |
| `amendment-approvals.md` | Oracle 5: every approved provision present in both the record and the text over disjoint text; the three amendments beyond the list; the earlier layer byte-identical to **60fda9f**; 7 of 7 self-tests held |
| `baseline/COMMIT.txt`, `capture.sh`, `pre-manifest.txt`, `full/`, `exit-codes.txt` | The pre-change baseline at **60fda9f**: 42 cells by digest, 11 streams retained whole |
| `baseline/projection.py` | The projection's full text — three anchored patterns |
| `baseline/compare.py`, `post/additivity.txt`, `post/post-manifest.txt`, `post/full/`, `post/exit-codes.txt` | Oracle 1: 42 frozen cells byte-identical projected, the projection a no-op on the pre-change stream, 20 new-source cells reproducible across processes and all differing from the reference |
| `baseline/recapture-check.py`, `recapture-check.txt` | The eleven cells the two baseline captures share, byte for byte — the disclosure behind oracle 1's coverage |
| `baseline/rebuild-check.py`, `rebuild-check.txt` | The committed source rebuilt and all 83 shared cells reproduced, closing three source-versus-binary gaps |
| `measurements/traits.txt` | The twelve derived traits per declared seed, independently recomputed |
| `measurements/viability.txt` | `REQ-MOK-034`: survivors, deaths, consumption, per-territory resources and class shares at tick 1,000 |
| `measurements/long-horizon.txt` | Tick 10,000 under both sources, and the reference source's extinction at 9,154 reproduced to the tick |
| `measurements/oscillation.txt` | The oscillation rate per seed against 10.6% and 12.2%, and the pooled-denominator discrepancy |
| `measurements/divergence.txt` | Every recorded divergence instance with tick, pair, traits and proposals; the same-tick coincidence count; the waste-accepting eats |
| `measurements/equivalence.txt` | Oracle 3's enumerated situation set, its size counted three ways, and the result |
| `measurements/fear.txt` | The range, step and correspondence invariants over 111,604 agent-ticks, and the perception boundary at 16 and 17 |
| `measurements/proposals.txt` | What each source proposed: zero `wait` and zero rejections under the new source on all ten runs |
| `negative-control/controls.py`, `oracle-2.txt`, `oracle-3.txt` | Oracles 2 and 3 each made to fail on purpose, with the failure's magnitude checked against the defect's prediction, and the source verified byte-identical after revert |
| `observer/frame-probe.rs`, `roster-frames.txt` | Oracle 4: 864 bar rows rebuilt from rule 4's parts, the four gauges' cell positions, and the bar-width reachability finding |
| `interface-and-purity.txt` | The public-interface census both sides, zero floats in code, and `fear`'s one writer and no reader |
| `test-census.txt` | The suite reconciled name by name and tier by tier against **60fda9f** |
| `static-checks.txt` | `fmt`, `clippy`, `test` and both dependency trees, captured rather than transcribed |
| `analysis/*.py`, `*.sh` | The tooling that produced each `.txt`, retained so every figure is reproducible from the recorded command |

`observer/frame-probe.rs` was written into `mokiterions-tui/tests/`, run once, retained here and deleted from the
crate — the `WO-MOK-006` precedent for a probe that must not become a permanent test.

**One repository-level change was needed to make any of this reproducible from a clone, and it was found while
committing.** Every manifest here records the SHA-256 of a captured file so a reviewer can hash the retained file and
get the recorded number. With `core.autocrlf = true` and no `.gitattributes`, a Windows checkout rewrites every
retained stream to CRLF, and the recorded digest then fails against a file whose content is correct. **The defect is
not hypothetical and not new:** `WO-MOK-006`'s retained `baseline/engine/full/short_seed42_baseline_trace_on.txt`
hashes to `08eadb21…` as checked out here, while its own manifest records `3424133a…` — which is the digest of the
same bytes with LF endings, and of the blob git actually stores. So a `.gitattributes` was added with one line,
`docs/engineering/simulation/evidence/** -text`, which disables conversion for the retained evidence in both
directions. **Checked in a fresh checkout rather than argued from the attribute:** the commit was checked out into a
clean git worktree, and there `WO-MOK-006`'s file hashes to `3424133a…` — its own recorded value, the one it failed
before — while this packet's `pre-baseline-seed0-ticks1000.log` hashes to `801b67cd…`, its row in
`baseline/pre-manifest.txt`. `mokiterions-core/src/simulation.rs` still checks out CRLF in the same worktree, so the
attribute is scoped to the evidence and changed nothing about how source is written. No committed blob and no recorded
number changed; the fix is to how the retained files are written out, and it repairs the same defect for `WO-MOK-001`
through `WO-MOK-006` as a side effect.

## 16. Residual disclosure

Everything a verifier should weigh that the checks above do not settle. `VER-MOK-007`'s own stated residual
uncertainty is items 1 to 4; items 5 to 11 were found during the work.

1. **`fear` has no consumer, so its constants are unfalsifiable by any outcome.** One writer, no reader, by census.
   The 39% ceiling residency and the 219 `+5` truncations are observations, not defects. Nothing here establishes that
   `+10`/`-5` is the right pair; that becomes answerable when something reads the attribute.
2. **Individuality is demonstrated at the scale it was measured, not at the scale a reader might assume.** Three to ten
   divergent situations per thousand-tick run, and **zero** same-tick coincidences, so no divergence is ever visible
   side by side in one frame. The 54–97 waste-accepting eats per run are the same behavior counted without requiring a
   coincidence, but that substitution is the product owner's call.
3. **Equivalence is exhaustive over a finite set, not over the input space.** Oracle 3 covers all 2,808 enumerated
   situations because that set is finite. Oracle 1 covers 40 cells of a much larger space.
4. **The trait derivation is not a security primitive.** SplitMix64 is not cryptographic and the seed is a public
   command-line argument, so trait values are trivially predictable by design. Nothing in this change relies on their
   unpredictability and no future requirement should.
5. **The `VREC-MOK-005` gate was overridden, not met** — section 2. Six amendments and seven assessments from the
   previous work order remain unresolved. The mitigation is checked, not asserted, but it is a cost carried forward.
6. **Five manual assessments are outstanding and a sixth is unsigned** — section 14. No verification record can be
   written against this commit.
7. **Two amendments written during implementation are OUTSTANDING** — the `SPEC-MOK-001` *Help output* correction and
   the three extra `SPEC-MOK-003` provisions. Oracle 5's own words: "an amendment nobody approved is not a
   specification."
8. **Oracle 1's pre-change baseline covers eleven cells with certainty and forty on an argument** — section 4.
9. **The oscillation comparison reproduces `WO-MOK-002`'s counts but not its denominators**, differing by exactly 96 in
   both rows, and the seed-0 margin against the unbiased-walk rate is 0.008 percentage points.
10. **Only one roster bar width is reachable through `render::draw`.** `bar = 2` at every declared viewport. Rule 4's
    `min(20, …)` cap and its collapsed one-line form are carried by three internal render tests and by no frame a user
    can produce.
11. **Two seeds sit one death from `REQ-MOK-034`'s floor** at 9 of 12, and the whole survivor table is downstream of a
    trait-range narrowing decided mid-implementation on a fifty-seed sweep whose `0..=40` miss rate is 4%. A sixth
    declared seed could miss the floor. The floor is met on the five seeds the contract declares and on nothing else.
12. **Seed 777 is the only individual run that goes extinct by tick 10,000**, at 9,938, and the only seed that retains
    all twelve at tick 1,000. Nothing in the contract turns on either fact, and no explanation for the pairing is
    offered here.
13. **This change added three `W-HEX-003` warnings to the harness inspection** — section 11. `ARCH-MOK-001` and
    `ARCH-MOK-002` now predate specifications they declare conformance to. The substantive reassessment exists in
    `WO-MOK-007`'s decision record; the artifacts themselves were deliberately not edited.
14. **A `.gitattributes` was added outside this work order's *In scope* list** — one line, disabling end-of-line
    conversion for the retained evidence, without which no manifest digest in this packet or in the six before it
    reproduces from a Windows clone. Section 15 has the check. It changes no code and no recorded value, but it is a
    repository-level file this work order did not set out to add.
15. **The evidence packet is 6.9 MB, against 888 KB for the largest previous one.** Almost all of it is the five
    1,000-tick reference-source baseline streams, retained whole so that oracle 1 can be re-derived by a reviewer
    instead of taken from a digest. Whether that trade is worth the repository weight is the owner's call; the
    alternative is retaining digests alone and asking a reviewer to trust `compare.py`.

**Status of this work order.** `WO-MOK-007` is left at `status = "in_progress"` and no verification record exists. A
verification record binds a commit and is created after the one it names; five manual assessments are outstanding, two
amendments are unratified, and oracle 5's second condition is unmet by the owner's recorded override. This packet was
written before the implementation was committed, and it is the input to a verification decision, not that decision.
