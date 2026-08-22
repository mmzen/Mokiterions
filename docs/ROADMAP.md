# Mokiterions Functional Roadmap

> **Authority note.** This document is repository-owned functional planning. It is **not** approved product
> intent, a requirement, an architecture decision, work authorization, a verification contract, or release
> authority. Only formal artifacts under `engineering` carry that authority, per `../ENGINEERING_HARNESS.md`.
> Nothing here authorizes implementation. Each phase below must be converted into an approved artifact chain
> before any work begins.

- **Created:** 2026-08-17
- **Scope:** macro phases following the completed minimum simulation foundation (`WO-MOK-001`)
- **Audience:** product owner, engineering owner, assurance owner
- **Statuses re-measured against the artifact graph and against `master`:** 2026-08-22, at `5bdf607`. Every
  lifecycle word, commit and merge claim below was read from artifact metadata and from `git merge-base`
  rather than carried forward. Where a claim moved, the earlier form is retained in a blockquote beside it.
  Phase 3's `WO-MOK-017` account was re-derived a second time on the same day, because pull request #41 merged
  between the first measurement at `f7b1c45` and this one — which is the ordinary condition of this document, not
  an incident.

## Purpose

`WO-MOK-001` delivered a verified, deterministic, engine-authoritative foundation. This roadmap proposes the
macro evolutions from that foundation to the project's stated success criteria: twelve autonomous Mokiterions
inhabiting the world over an extended simulation and producing outcomes that are not predetermined by the
engine, with the whole lineage traceable through SE Harness.

## Current state

Implemented and verified under `VREC-MOK-001` (commit `ecd03a8`):

- 128 × 128 world, two territories, twelve Mokiterions, three food classes
- `health`, `satiety`, `energy`; bounded decay; death at zero health
- Actions `wait`, `sleep`, `eat`, `move`; engine-side validation with no partial mutation
- Conditional food regeneration, including permanent territory depletion
- Deterministic SplitMix64 entropy from an explicit seed
- Line-oriented text observation stream and optional `--trace-actions`
- A local seeded baseline decision source, explicitly a development placeholder

Added and verified under `VREC-MOK-002` (commit `68163ac`), completing Phase 1:

- Bounded perception: nearby resources and nearby living Mokiterions with direction and distance, radius 16
- Resource density as an explicit simulation input, binding initialization, capacity, and replenishment
- A deterministic reference decision source (eat → sustain → approach → search), selectable by `--policy`
- A measured population viability floor of eight of twelve survivors at 1,000 ticks on five declared seeds

Added under `WO-MOK-010` (implemented and verified under `VREC-MOK-010` at commit `1a937a1` — see Phase 2's
status below for the one matrix row that record leaves unsatisfied):

- One behavioral trait per Mokiterion, `waste_tolerance` in `0..=40`, derived from the seed and the identifier by a
  generator of its own and fixed for the run
- `fear` as the fourth dynamic attribute, `0..=100`, rising `+10` on any perceived living Mokiterion and falling `-5`
  otherwise — computed, bounded, reported, and read by nothing until `WO-MOK-016` below
- A third decision source, `--policy individual`, which is rule 5 with a trait-scaled waste allowance and is
  proposal-identical to rule 5 at tolerance zero
- A fourth roster gauge in the observer

Added under `WO-MOK-011` (implemented and verified under `VREC-MOK-011` at commit `9ddcf83` — see Phase 2.5's
status below for the manual assessment that record accepts unperformed):

- A name per Mokiterion, one of twelve the specification fixes, assigned by identifier number and reported once on
  `agent_initialized` ahead of every other field
- The observer presenting that name in the roster, the inspector and the map, sourced from the engine's own record and
  from nowhere else — no name table, no fallback, no derivation from an identifier
- The map glyph becoming the name's initial, replacing the digit-and-letter table, on twelve pairwise-distinct letters

Added under `WO-MOK-016` (implemented and verified at `VREC-MOK-017`; two approved obligations were measured as
failing and were resolved by a governed amendment inside the phase — see Phase 3's status below):

- Contact at Chebyshev radius 1, recomputed from position every tick and held in no stored relation
- The seven targeted actions `attack`, `fight`, `threaten`, `surrender`, `approach`, `avoid`, `retreat`, each with
  its own precondition and each applied at the acting Mokiterion's own opportunity
- Damage as a function of the striker's own condition, `10 + (energy + health) / 10` bounded to `10..=30`, a flat
  `5` energy cost per strike, and death through combat by the existing zero-health rule and no other path
- A one-tick record of attacks suffered, carried in the observation and cleared at the holder's next opportunity —
  the first and only memory anything in the system has
- Three event types, `attack_resolved`, `threat_resolved` and `surrender_resolved`, and a `suffered` field on
  `action_trace`
- A fourth decision source, `--policy social`, which reads `fear` at three points and is proposal-identical to
  `--policy individual` for a Mokiterion that perceives no company and holds an empty record
- No entropy drawn by any targeted resolution, so the shared stream is untouched by everything the phase adds

Added under `WO-MOK-017` (merged to `master` as `f7b1c45`, the work order now **`implemented`** and the chain
**verified** at `VREC-MOK-020` — see Phase 3's status below for what that record binds and what it leaves owed):

- The missing term in `SPEC-MOK-001` rule 5's non-waste condition: a resource whose satiety restoration would
  be clipped is eaten and approached anyway while the clipped amount is no more than the resource's own
  allowance, `R * R / 100` — `2` for low class, `9` for medium, `25` for high. The condition is now stated
  once and rule 19's tolerant test is that same expression at the acting Mokiterion's own tolerance, so no
  decision source can be stricter than `reference` and none can carry a copy that drifts
- `REQ-MOK-060`'s resource-composition ceiling, amended from one half to **three fifths** on the measured
  curve and met on every declared seed under all three bound sources — the high-class share of a territory's
  standing supply at tick 1,000 falls from 36%–82% to 33%–54%
- **The carried-in high-class accumulation of Phase 2 is closed by this**, and both survivor floors were
  re-measured rather than argued: `REQ-MOK-014`'s eight and `REQ-MOK-034`'s eight are met on every declared
  seed, with no margin on the worst of them, and `REQ-MOK-058`'s five is met by two

Added under `WO-MOK-019` (implemented, merged to `master`, and verified at **two** commits — `VREC-MOK-012` at the
branch candidate `50364a3` and `VREC-MOK-019` at the second merge commit `e96648a2` — see Phase 4a's status below
for what those records accept rather than satisfy, and for the merge neither of them binds):

- A structured record stream, `SPEC-MOK-006`, written to a file named by `--events-path` and to nothing by default:
  one record per line, four kinds — `header`, `event`, `metrics`, `run` — with every event record in one-to-one
  correspondence with a line of the text stream, in the same order and carrying the same values
- Per-tick world metrics the text stream never carried: living and dead counts, population per territory, the sums
  and extremes of the four attributes, and standing food per territory by class against capacity
- Seven cumulative counters and a per-Mokiterion death tick, retained by the engine, read only by the record
  producer, and reported in the `run` record together with the summary line's twelve figures
- The binary target owning every filesystem operation, the library owning none: it resolves the path, opens the
  destination, and removes only a file it created itself when a run fails


Not yet implemented: per-agent entropy substreams, any second trait, LLM-backed decisions, and anything that reads
the record stream — no batch runner, no distribution across seeds, no outcome classification. `fear`, social and
combat behavior and structured observability each appeared on this list until 2026-08-20 and are struck from it: the
first three arrived under `WO-MOK-016` and the fourth under `WO-MOK-019`.

Two limitations carried forward from Phase 1 are recorded in `VER-MOK-002`'s residual uncertainty and are not
restated as analysis here: high-class resources accumulate against capacity, and the viability floor is a claim
about tick 1,000 rather than a steady state.

**The two hard blockers of the next section are resolved.** Constraints 1 and 2 below are retained as the recorded
rationale for the phase ordering, not as a description of the current system. Their figures describe the
pre-Phase-1 constants and are superseded by the measurements in
`engineering/simulation/evidence/WO-MOK-002`.

## Three constraints that shape the ordering

Three properties of the current system, rather than preference, determine the phase sequence.

### 1. The world as specified cannot sustain twelve agents

Satiety is the only binding survival constraint, because sleep restores energy at no cost while satiety has no
non-food source. The population-level food economy is therefore decisive:

| Quantity | Value |
|---|---:|
| Satiety demand (2/tick × 12 agents) | 24 satiety/tick |
| Regeneration rate (1 item per territory per 10 ticks) | 0.2 items/tick |
| Mean food value across the three classes | 31.67 satiety |
| Regeneration supply | ≈6.3 satiety/tick |
| **Supply-to-demand deficit** | **≈3.8×** |
| **Implied steady-state carrying capacity** | **≈3 of 12 agents** |

Initial stock does not close the gap. Six items carry roughly 190 satiety, about eight ticks of population
demand, and climbing from six items to the 24-item cap would require about 90 ticks of zero consumption while
agents begin starving at tick 50.

The observed extinction at tick 69 on default settings is therefore **not** a deficiency of the baseline
policy. No policy of any sophistication survives this configuration; optimal play preserves roughly three
agents. Introducing an intelligent decision source before correcting the food economy would produce mass
death and yield no information about emergent behavior.

Because `SPEC-MOK-001`'s constants are approved, and `WO-MOK-001`'s decision envelope explicitly prohibited
changing specified constants, correcting this is a governed specification change supported by evidence, not a
tuning adjustment.

### 2. Agents cannot perceive anything beyond their own cell

`Observation` exposes only `co_located_food`. There is no nearby food, no nearby Mokiterions, no memory, and
no recent-event context. Any decision source, heuristic or model-backed, is effectively blind and cannot
navigate. Perception is a prerequisite for every behavioral phase.

### 3. A live model provider collides with the reproducibility requirement

`REQ-MOK-009` requires reproducible entropy, and `INT-MOK-001` sets a success measure of 100% identical
results for repeated runs at an identical seed. A live provider cannot satisfy this. The collision requires an
explicit governance decision, addressed in Phase 5.

## Dependency graph

```text
Phase 0  Harness upgrade landed under governance
   │
   ▼
Phase 1  Perceptible, survivable world  ── prerequisite for everything below
   │
   ▼
Phase 1.5  Terminal observer  ── the instrument every later phase is assessed with
   │
   ▼
Phase 2  Individuality: traits and fear
   │
   ├── Phase 2.5  Names  ── identity in the presentation; prerequisite of nothing
   │
   ├──────────────┬───────────────┐
   ▼              ▼               │
Phase 3        Phase 4a           │   (3 and 4 are independent
Conflict       Structured         │    and may run in parallel)
   │           measurement        │
   │              │               │
   │              ▼               │
   │           Phase 4b           │   (4b consumes 4a, and is
   │           Distribution and   │    deferred until it does)
   │           classification     │
   │              │               │
   └──────┬───────┘               │
          ▼                       │
    Phase 5  LLM decision source ─┘
          │
          ▼
    Phase 6  Emergence evaluation
```

Phase 1.5 is not a prerequisite of Phase 2 in a technical sense — Phase 2 could be built without it. It is placed
here because it is the instrument by which Phases 2 through 6 are *judged*, and building it after the behavior it
would have helped assess wastes its main benefit.

Phase 2.5 hangs off Phase 2 rather than following it because nothing depends on it. It needs the initialization
record Phase 1 established and the roster Phase 1.5 built, and no later phase needs it — a name changes what a run
is *called*, never what it does. It is placed here because that is when it was asked for.

Running alongside Phases 1–2: a bounded **LLM feasibility experiment** (see Cross-cutting work).

---

## Phase 0 — Land the in-flight harness upgrade

**Prerequisite gate, not a product phase.**

The working tree contains an SE Harness 0.2.1 → 0.4.0 migration, including a canonical artifact-layout move
and a schema migration of `ARCH-MOK-001`. That architecture artifact is `approved` yet modified, which is
itself a governance act. Landing this under one explicit governance work order prevents every later phase from
building on an unauthorized base.

**Also resolve:**

- The evidence-detection mismatch: `../scripts/generate_harness_dashboard.py` keys evidence on **file name**,
  while `WO-MOK-001` instructed evidence into a **directory** named for the work order. This produces a
  misleading `W-HEX-001` warning despite evidence existing. Align the convention in one direction.
- `W-HEX-003`: `ADR-MOK-001` now predates its newer declared target `ARCH-MOK-001` and needs reassessment.

**Verification:** `harnessctl doctor` clean, validator passing, `inspect` findings resolved or explicitly
accepted with rationale.

---

## Phase 1 — A perceptible, survivable world

**Goal.** Make survival possible, and make the world legible to a decision source.

**Why first.** Removes both hard blockers identified above. No downstream behavior is measurable until an
agent can locate food and the food economy can support a population.

**In scope**

- Perception radius: nearby food with direction and distance, nearby Mokiterions, territory boundaries
- Rebalanced food economy governed by an explicit **target carrying capacity**
- A deterministic heuristic reference policy (eat → sustain → seek nearest food → search) proving the world is
  survivable

No `explore` action. Directed navigation is movement plus perception, so a separate action would add a verified
behavior without adding capability. It was dropped when the packet was drafted.

**The carrying-capacity decision.** Target capacity should become an approved, verified requirement parameter
rather than an accidental consequence of constants. The productive regime is *narrow scarcity*: capacity
around 8–10 of 12, so competition genuinely constrains behavior while extinction is not preordained.
Abundance suppresses emergent behavior as effectively as famine does.

**The heuristic policy is a scientific control.** Without it, later phases cannot distinguish "the model
produced emergent cooperation" from "any competent policy survives this world." That distinction is the
difference between a defensible result and an unsupported claim.

**Out of scope.** Fear, combat, social behavior, model integration, structured output.

**Artifact chain.** New intent. `INT-MOK-001` is not widened, because its purpose and success measures were
achieved and verified under `VREC-MOK-001`; its non-goals do not exclude perception or food balance, but a
completed intent is not the place to record new outcomes.

The packet drafted on 2026-08-17 is: `INT-MOK-002`, `CAP-MOK-002`, `REQ-MOK-013` (perception), `REQ-MOK-014`
(viable population), `REQ-MOK-015` (reference decision source), `VER-MOK-002`, `WO-MOK-002`, and an in-place
amendment of `SPEC-MOK-001`. It is deliberately smaller than the 8–10 requirements estimated here: every
existing requirement is constant-free, so the food rebalance needs no requirement change, only the amended
specification. `SPEC-MOK-001` is amended rather than superseded so that no two active specifications state
conflicting survival values.

**Key verification.** Heuristic policy sustains at least the target capacity across ≥1,000 ticks on multiple
seeds; measured carrying capacity within an approved tolerance; byte-identical determinism preserved.

**Status.** Delivered. `WO-MOK-002` is `implemented` and verified under `VREC-MOK-002` at commit `68163ac`.

---

## Phase 1.5 — Terminal observer

**Goal.** Build the instrument for watching behavior, as a separate component that cannot affect what it watches.

**Why here, and why it is numbered 1.5.** It was not in the original sequence. It is inserted because Phase 1
produced the first result the text stream could not explain: extinction with 122 resources standing and both
territories at capacity. Position, not supply, decides outcomes, and position is exactly what a line-oriented
stream conveys worst. Every phase after this one produces behavior whose plausibility is a question about a
spatial situation at a specific tick — *why did that agent walk away from food*, *why did those two fight*, *is
this model output reasonable given what the agent could see*. Deferring the instrument means assessing those
phases without the means to assess them, and the cost of building it does not fall over time.

It is numbered 1.5 rather than made a new Phase 2 because it delivers no simulation behavior. Nothing about the
world changes.

**In scope**

- A terminal user interface as a **second package**, over a read-only observation surface on the engine package
- Whole-world spatial view, survival state for every living Mokiterion, an inspector exposing each proposal and
  the engine's verdict on it, a filterable and exportable event log, keyboard control of progression and focus,
  and run provenance with the authorizing requirement for each event type
- Layout that degrades across viewport sizes without losing authoritative information

**Out of scope.** Every simulation rule. This phase is additive to `SPEC-MOK-001` and changes nothing in it. Also
out of scope: every attribute the target design anticipates that the engine does not compute — fear, traits,
names, combat outcomes, remembered locations, per-agent entropy, model latency. The specification reserves space
for one of them and requires that space to render empty. An inert `fear 0` would be a claim the engine cannot
support, and it is prohibited rather than merely discouraged.

**This phase ends two properties the project has held since `WO-MOK-001`,** and both are the product owner's
decision rather than a technical detail:

1. **The empty dependency set.** `ratatui 0.30.2` resolves to a measured 57 crates. Trimming features saves six.
   This is the project's first external dependency, and the containment strategy is that it is confined to the
   observer package — which is *why* the two-package split is a requirement rather than a preference.
2. **The single crate.** `ARCH-MOK-001` prohibited separate crates *without an approved requirement*.
   `REQ-MOK-026` is that requirement, for exactly one further package.

**Distinct from Phase 4.** Phase 4 is a batch measuring instrument: structured output, multi-seed runs, outcome
distributions, statistics over many runs. Phase 1.5 is a live interactive instrument for one run. They answer
different questions — *what happens across many runs* against *what is happening in this one* — and neither
substitutes for the other. Phase 4 remains where it is.

**Artifact chain.** The packet drafted on 2026-08-17 is: `INT-MOK-004`, `CAP-MOK-004`, `REQ-MOK-019` through
`REQ-MOK-027`, `SPEC-MOK-003`, `ARCH-MOK-002`, `ADR-MOK-003`, an in-place amendment of `ARCH-MOK-001`,
`VER-MOK-005`, and `WO-MOK-005`. Every artifact was `draft` when that packet was drafted; the current lifecycle state
of each is in its own metadata, and the **Status** paragraph below states where the chain now stands.

Two of those artifacts are load-bearing rather than procedural. `REQ-MOK-026` is the approved requirement that
`ARCH-MOK-001`'s prohibition on separate crates has always named as its own unlock. The `ARCH-MOK-001` amendment
scopes its prohibition on user-interface frameworks to the engine package, where it becomes checkable per package
and therefore stronger than before. Without both, this phase is prohibited outright.

`ADR-MOK-001` is **not** superseded. Its migration clause requires a superseding ADR only for replacing engine
authority, and a read-only observer replaces none of it.

**Key verification.** An observed run and an unobserved run at the same seed are byte-identical in authoritative
events and final state, under heavy operator interaction, with identical per-tick entropy draw counts. The engine
package's dependency set is empty, proved per package. Rendering is asserted cell by cell from an in-memory
buffer at each declared viewport, because a claim about a screen only a human has seen is the weakest evidence
this project accepts — and the observer's whole value is being trusted about what it shows.

**Status.** Implemented under `WO-MOK-005` and **verified**: `VREC-MOK-005` is `verified`, bound to commit `f361370`.
But it was verified with its whole manual and amendment debt disclosed and outstanding, in its own words "with all
seven manual assessments outstanding and unauthored, and eleven amendment provisions still `OUTSTANDING`" — so the
record certified that the parts behave as specified and explicitly not that the instrument answers `INT-MOK-004`'s
questions or that anything is legible on a real terminal.

**That debt was discharged on 2026-08-20 under `WO-MOK-012`**, a governance-only work order that changes no executable
behavior — the diff touches nothing under `mokiterions-core/` or `mokiterions-tui/`, and the workspace's 212 tests,
formatter and linter are unmoved. In one interactive review the repository owner took fifteen decisions:

- **All eleven amendment provisions ratified as written** — four in `SPEC-MOK-002`, one in `ARCH-MOK-001`, one in
  `SPEC-MOK-003`, five in `SPEC-MOK-004`. The `ARCH-MOK-001` one is load-bearing: it is the provision that makes the
  shipped observer legal, and ratifying it authorized what was already in the tree rather than permitting anything new.
- **Six of the seven manual assessments performed and authored** by the owner, including the reference-viewport
  legibility and colour-independence assessments the contract names.
- **The seventh left outstanding by deliberate decision.** The shipped binary has no operator-reachable panic path, so
  the live-terminal inspection cannot be performed by running the observer, and the contract's "rather than only by an
  automated assertion" makes the retained automated result insufficient by construction. Closing it against a proxy
  would have the record assert that a person inspected something no person had seen. **`VREC-MOK-005` continues to
  disclose it, and a release record covering this chain inherits that disclosure.**
- **Three restatements of `VER-MOK-005`**, whose subjects had moved since approval. One is notable as a defect in an
  approved assurance artifact rather than in the implementation: assessment 3 asked for a run with colour disabled, and
  the observer has no such flag and honours no such variable, so the assessment was unperformable on the shipped binary
  from the day it was approved.

**Three adverse observations came out of the live pass, and none is fixed.** The `?` key that opens the control
documentation is advertised nowhere on screen; the roster's survival gauges are two columns wide, which draws three
distinct states for 101 attribute values; and the hidden-pane notice — which does exist, is permanent and does
conform — names the overlay key rather than saying the terminal is too small, and carries no visual emphasis. The
product owner placed all three in a **separate chain** with the design settled in advance, on the reasoning that
recording and fixing are different acts and folding a rule 4 redesign into this work order would keep the assessment
obligation open past the next release record. That chain re-sweeps its own identifiers rather than trusting a record of
what was free.

`WO-MOK-012` is **`implemented`** on the engineering owner's acts of 2026-08-20, which also confirmed its
`commit_bound_verification = "not_required"` classification as a separately stated act. Being governance-only that is
its terminal status: it takes no verification record and nothing further is owed on its own lifecycle. Two things are
disclosed rather than smoothed over — the governance text was committed before the work order was approved, which is not
the sequence `WORKFLOW.md` describes, and `in_progress` was therefore never recorded. Its *Lifecycle* section states
both. At `implemented` it emits one W-HEX-001 warning, accepted on the precedent `WO-MOK-010` and `WO-MOK-011` set.

Everything decided, and the measurement behind each decision, is in `engineering/simulation/evidence/WO-MOK-012` —
start with its `README.md`, then `closing-review.md` for the fifteen review decisions and `assurance-decision.md` for
the approval.

**All three adverse observations are answered under `WO-MOK-013`**, the separate chain the product owner directed them
to. `REQ-MOK-047`, `REQ-MOK-048` and `REQ-MOK-049` were approved on 2026-08-20 in three separate acts, `VER-MOK-013`
approved as their contract, six `SPEC-MOK-003` amendments ratified one question at a time, and the work order approved
as a bounded scope — eighteen decisions, **all taken before implementation began** so that no amendment row was left
`OUTSTANDING`, three more on the live pass that followed, a twenty-second that moved the status and a **twenty-third
that verified the record**. What changed:

- **The survival gauges resolve.** Two gauges per line instead of four raises the roster entry to three lines and the
  bar from **2 cells to 13**, which is 14 renderable states for 101 values where there were 3, and a ten-point change
  now moves the fill at every value rather than at eleven of ninety-one.
- **The controls are findable.** The header carries a permanent `? keys` hint beside the run state, reducing to `?` at
  the floor, present in the first frame with no input delivered.
- **The hidden-pane notice is actionable.** It states the axis and the threshold value at which each excluded pane
  returns — `inspector i at width 140` — read from the layout rather than written down, emphasised, and legible in the
  `(symbol, modifier)` projection. At the floor all three panes are announced as `r W100  L H38  i W140`.
- **The log is held at six rows wherever it is present**, which withdraws the ten-row growth at `160 × 48`. This is the
  product owner's decision 1 and it is a **trade, not a correction**: the reference viewport now shows four recent
  events where it showed eight. It is what keeps `REQ-MOK-020` intact, because twelve three-line entries need 36
  interior rows and a six-row log is what leaves exactly 36. The fit has no slack in either direction. The trade was
  put to the product owner a third time after the live pass, with the result on screen, and it **stands**.

The workspace runs **226** tests, up from 212 — fourteen arrivals, no losses, reconciled name by name in that pack's
`test-census.md`. The engine is untouched: `mokiterions-core/` is byte-identical to `master`, and non-perturbation is
closed as a measured chain against `ff3a155` rather than asserted.

**Status.** `WO-MOK-013` is **`implemented`** as of 2026-08-20, moved there by the repository owner as engineering owner
after the live pass. **`VREC-MOK-013` is `verified`** as of the same day, transitioned by the owner as accountable
assurance owner, and bound to `41c20ca`. `commit_bound_verification` was `required` and is discharged for that commit;
the harness's *Decision required* queue is empty. Two things that status does **not** say: `WO-MOK-013` did not move
with it and stays `implemented`, and the record binds a branch commit rather than `master`'s merge at `798e5d5` — the
code is identical between them, the harness graph is not, and **a record bound to the merge commit is owed**.

> **Earlier form, for a reader diffing this file.** It read *"It is **not verified**: `commit_bound_verification` is
> `required`, and `VREC-MOK-013` exists as a **`ready` candidate** bound to `41c20ca` rather than as a verified record
> — `DECISION_RIGHTS.md` reserves that transition to the accountable assurance owner, and the harness reports it as the
> one item under *Decision required*."* That was true until the decision was taken.

Of `VER-MOK-013`'s two manual assessments, **the gauge-legibility one was taken at `160 × 48` on 2026-08-20 and is
SATISFIED** — which is the adverse observation this chain exists to answer, now answered by a person rather than by
arithmetic. **The discoverability one is still outstanding with no author**: it needs a person who has not read
`SPEC-MOK-003` rule 7, which the available assessor has read and ratified six amendments to. The owner chose to find
such a person rather than amend the contract or close the row by decision — the only route that verifies
`REQ-MOK-048` — so the frame, the question and the administrator's rules are prepared in
`evidence/WO-MOK-013/discoverability-assessment.md` and the row waits on the person.

**`REQ-MOK-048` is verified, and not on that assessment.** On 2026-08-20 the assurance owner accepted the
requirement's **four automated cases** — the key hint on screen at frame 1, at every viewport of rule 5's table and at
the floor, present after 200 ticks in both run states, displacing neither the announcement nor the provenance footer —
**in place of the primary evidence `VER-MOK-013` designates**, and `VREC-MOK-013` records all three requirements as
verified. **The contract is not amended and is not satisfied**: it still calls that automated case *"a necessary
condition and not the property"*, the discoverability assessment is still **outstanding with no author**, and taking it
would now confirm or contradict a verified requirement rather than verify one. `evidence/WO-MOK-013/assurance-decision.md`
records the instruction verbatim, the three bases it was read against, what was accepted and the nine things the
acceptance does not retire. The earlier form of this paragraph ended *"`VREC-MOK-013` therefore records `REQ-MOK-047`
and `REQ-MOK-049` and **discloses `REQ-MOK-048` as unverified**"*, which is what the record said before the decision.

**One thing about this chain's names.** Four of its identifiers — `WO-MOK-013`, `VER-MOK-013`, `VREC-MOK-013` and
`REQ-MOK-047` — are also claimed by `governance/adr-mok-006-third-party-crates`, a branch off the same base that is
pushed and not merged, where they name a dependency-set work order and its contract. Both sides' artifacts are
owner-approved. Nothing was renumbered: decision 3's rule sends the conflict to whichever branch reaches `master` second.
`evidence/WO-MOK-013/identifier-collision.md` measures it, and **a citation of any of those four
names resolves only together with the branch it was written on** for as long as both branches exist.

**This chain merged first, in both of its identifier collisions**, at `798e5d5` on 2026-08-20 — so under decision 3's
rule the renumbering falls to the two other branches and none is owed here. Neither is merged;
`feature/phase-4a-definition` now reports **9** conflicts against `master` and
`governance/adr-mok-006-third-party-crates` **8**, four of the latter `add/add` on the colliding names. That is an
outcome of merge order rather than a decision anyone took, stop condition 8 did not fire because its trigger was this
branch merging *second*, and nothing was asked of either branch.

> **Later fact, recorded on 2026-08-20 on `governance/adr-mok-006-third-party-crates`, in the commit that merged
> `master` into it.** The two paragraphs above are overtaken on one point each. **That branch renumbered.** Its
> `WO-MOK-013`, `VER-MOK-013`, `VREC-MOK-013` and `REQ-MOK-047` are now `WO-MOK-014`, `VER-MOK-014`,
> `VREC-MOK-014` and `REQ-MOK-050`, so the four names above resolve to this chain alone and a citation of any of
> them no longer needs the branch it was written on. It moved because `VREC-MOK-013` here is `verified` against a
> commit while `VERIFICATION_RECORD.template.md` lets a governance decision move only a `ready` record to
> `superseded`: two `verified` homonyms cannot both stand, so the side that had not landed is the side that could
> move. The repository owner directed it inside that branch's own approved work order. Nothing is asked of this
> chain: no artifact here is renamed, re-dated or re-opened, and `evidence/WO-MOK-013/identifier-collision.md`
> stays as it was written. The conflict figure moved with the renumbering — measured in that clone, **9** before
> it and **4** after, with all four `add/add` gone; the **8** recorded above was measured against an earlier tip
> of that branch. It has since merged `master` into itself, resolving those four, and it is still not merged into
> `master`. Both acts are measured there in `evidence/WO-MOK-014/WO-MOK-014-renumbering.md` and
> `WO-MOK-014-merge.md`.

Everything measured is in `engineering/simulation/evidence/WO-MOK-013` — start with its `README.md`.

**That branch merged the same day, and its chain is a delivered one.** Pull request #33 merged
`governance/adr-mok-006-third-party-crates` into `master` as `d8e2079` on 2026-08-20, so the blockquote's closing
clause is overtaken: `WO-MOK-014` is `implemented`, and both `VREC-MOK-014` at the branch candidate `65ac88b0` and
`VREC-MOK-015` at the merge commit `9599c0a9` are `verified` against `VER-MOK-014`. The renumbering above is the
only trace this roadmap carried of that work until now, which understated it considerably — it is a governed chain
with its own ADR and two permanent CI gates. It is accounted for under *Cross-cutting work* below, in
*Governed chains delivered outside the phase sequence*.

**Two further observer defects were closed on 2026-08-21 under `WO-MOK-018`.** It is recorded here because its
verification record belongs to this phase's contract and would otherwise be unreachable from the roadmap. The work
order — *"Close the two observer defects Phase 3.1 left: a stale filterable-type count, and a fourth attribute that
becomes unreadable at death"* — is `implemented`, implements `REQ-MOK-021` and `REQ-MOK-022`, and therefore falls
inside this phase's `CAP-MOK-004` rather than opening an intent of its own. **`VREC-MOK-018` is `verified`**, bound
to `6051ef21`, against this phase's own `VER-MOK-005`. It reached `master` through pull request #37 as `7f4792a`.

> **This is a pointer and not the account.** What the two defects were, why Phase 3.1 produced defects in a Phase
> 1.5 component, what the fix traded, and how a contract approved for the terminal observer came to be the one a
> later correction conforms to — none of that is written in this roadmap, and all of it is owed. A reader auditing
> the verification records should find `VREC-MOK-018` referenced somewhere; that is all this paragraph achieves.

---

## Phase 2 — Individuality: traits and fear

**Goal.** Twelve agents that are genuinely not interchangeable.

**Why here.** All agents are currently identical and share a single entropy stream, so the concept's
"individuality without randomness" is unimplemented. `fear` is a hard prerequisite for Phase 3. Completing the
agent state model here means the observation contract stabilizes **once**, so Phase 5's provider adapter is
written against a final contract instead of being rewritten twice.

**In scope**

- Per-agent trait vector derived deterministically from seed and agent ID (for example caution, aggression,
  sociability)
- `fear` as the fourth dynamic attribute, with defined rise and decay dynamics
- Per-agent entropy substreams, so identical situations can diverge across individuals
- **Carried in from Phase 1: high-class resource accumulation.** `SPEC-MOK-001` rule 5 made a resource both
  eatable and approachable only while its satiety restoration would not be clipped **at all**, which for high
  class meant satiety of at most `50`. High-class resources were therefore sought least often and accumulated
  against the capacity that density fixes — roughly three quarters of standing supply by tick 1,000 — so
  territories stayed full while the food anyone would walk to grew scarce. The population declined against a
  full larder, and a 10,000-tick run at the default density reached extinction at tick 9,154. This was
  measured, disclosed, and accepted by the product owner on 2026-08-17 because no Phase 1 requirement speaks
  past tick 1,000, where the rule is decisively better than its predecessor. See
  `engineering/simulation/evidence/WO-MOK-002/density-curve.md`.

  **Closed on 2026-08-21 under `WO-MOK-017`, and the diagnosis above turned out to be incomplete.** The
  clipping bound was not a considered constant at all: rule 5's condition had simply lost the allowance term
  the rule was written to carry, so `50` stood where `75` belonged and the same omission had been copied into
  rule 19's tolerant test. What looked like a modelling consequence to be lived with was a defect to be
  corrected. `REQ-MOK-060` is the correction, and the high-class share of standing supply at tick 1,000 now
  runs 33%–54% against a starting third. The tick-9,154 figure above is **not** re-measured: it belongs to the
  uncorrected world, `REQ-MOK-060` binds tick 1,000 only, and re-taking the long horizon is outstanding work —
  see the note on the carried-in item below.

**Out of scope.** Combat resolution, model integration.

**Key verification.** Identical situations demonstrably produce divergent choices across agents; traits fully
reproducible from seed; fear dynamics bounded and observable.

**Note on the carried-in item.** Fixing accumulation means changing when a rich resource may be consumed, which
moves the density curve and invalidates `REQ-MOK-014`'s floor of eight. Expect that requirement to need
re-approval on a fresh measurement rather than an argument. A long-horizon stability requirement should be stated
at the same time, because the current floor is a claim about tick 1,000 and deliberately not about a steady state.

**What actually happened, on 2026-08-21.** The first sentence held and the second did not. The density curve
moved substantially — nine of the fifteen obligated runs at the default density leave *fewer* Mokiterions alive
at tick 1,000 than before the correction, and matrix consumption rises from 26,136 meals to 26,924 — but
`REQ-MOK-014`'s floor of eight and `REQ-MOK-034`'s floor of eight are both **met on every declared seed** and
needed no re-approval. Neither has any margin on its worst seed, so the prediction was right about the risk and
wrong about the outcome. The long-horizon requirement is **still not stated**, and `WO-MOK-017` did not re-run
the long horizon: it is the one part of this item that remains open, and it now has a second reason to be
stated, since the figures the earlier long-horizon claims rest on describe a world the engine no longer
implements.

### What was actually built, and what was cut

The packet approved on 2026-08-19 is `INT-MOK-006`, `CAP-MOK-006`, `REQ-MOK-031` through `REQ-MOK-034`,
`VER-MOK-010`, `WO-MOK-010`, and in-place amendments of `SPEC-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-003`. It is
deliberately narrower than the *In scope* list above, and the reductions are the product owner's decisions of
2026-08-19 rather than implementation shortfalls:

| *In scope* above | What was approved and built | Why |
|---|---|---|
| A trait **vector** — "for example caution, aggression, sociability" | **One** trait, `waste_tolerance` | A trait no rule reads is an inert field. Exactly one rule reads exactly one trait, so the individuality claim is measurable rather than asserted. A second trait is a later governed change and needs a rule to read it |
| **Per-agent entropy substreams** | **Not implemented.** The shared stream stays shared; the trait derivation uses a separate generator that neither reads nor advances it | Substreams would have moved every draw in every pre-existing run, so no run predating this phase would reproduce. Divergence is carried by the trait instead, which is what makes the whole change additive. Substreams remain available as a later change, and would cost the additivity property |
| `fear` with "defined rise and decay dynamics" | Delivered, and **inert at the time** — no rule and no decision source read it | `SPEC-MOK-003` rule 4.5 refused an attribute the engine cannot support; this is the opposite case, an attribute the engine computes and nothing consumes. Recorded as a residual rather than presented as complete. Phase 3.1's `REQ-MOK-057` is the first consumer, and the first measurement of it says the rise dynamics chosen here are consequentially wrong — see Phase 3's status |
| **Carried in: high-class resource accumulation** | **Not addressed under this work order.** Closed on 2026-08-21 under `WO-MOK-017` | Out of `WO-MOK-010`'s scope, and it stayed carried through Phase 2.5 and Phase 3.1. The measured share of standing high-class supply at tick 1,000 did not improve here. `REQ-MOK-060` closed it two phases later by correcting the omitted allowance in rule 5, and both floors survived the fresh measurement |

Two further cuts were accepted at the specification level, and both are the reason the observer changed as little as
it did: **no per-Mokiterion trait display** in the observer, and **no trait in the event vocabulary** beyond the one
`agent_initialized` record. Displaying a per-agent trait would have required a fifth roster field at a bar width
already narrowed to 2, and adding trait fields to further event types would have widened the record format for
values that never change.

**Status.** Implemented under `WO-MOK-010`, which is `implemented`, and **verified** under `VREC-MOK-010` at
commit `1a937a1`, on `master`. The record was verified with **one matrix row unsatisfied** — the
`VREC-MOK-005` gate, item 3 below — and says so about itself.

> **What this section said while the record was a candidate, and what has since moved.** It read: *"Implemented
> under `WO-MOK-010`, which is left at `in_progress`. **Not verified.** `VREC-MOK-010` is a `ready` candidate
> bound to commit `4f32a9f`; it takes no decision, states that `VER-MOK-010` is *not* satisfied at that commit,
> and should not be transitioned as it stands."* Two of the three obstacles below were then cleared and the
> third was not. **Item 1 is discharged**: the repository owner recorded all seven of `VER-MOK-010`'s manual
> assessments on 2026-08-19, and the record's own comparison table reads *7 of 7 recorded*. **Item 2 is
> discharged**: four amendments were ratified by the technical owner on the same day, one was approved under a
> stop condition and two needed none. **Item 3 stands**, which is why the record is verified with one
> unsatisfied row rather than none. The bound commit also moved — `4f32a9f` and then `035a001` were retired and
> the candidate was **re-captured** at `1a937a1` rather than corrected in place. The three items are left as
> written because the reasoning in them is what the closing review answered.

Three things stood in the way of a verified record:

1. Five of `VER-MOK-010`'s seven manual assessments are outstanding and a sixth is unsigned.
2. Two amendments written during implementation are unratified — a `SPEC-MOK-001` *Help output* correction and three
   `SPEC-MOK-003` provisions outside rule 4.
3. `WO-MOK-010`'s own gate required `VREC-MOK-005` to be `verified` with its six amendments approved and its seven
   assessments recorded. It was none of those when implementation began, and the product, technical and assurance
   owner overrode the gate on 2026-08-19 and authorized implementation over it, on the recorded mitigation that the
   two layers stay separable by inspection. **One half of that has since resolved and the other has not.**
   `VREC-MOK-005` is now `verified` on `master` and `WO-MOK-005` is `implemented`, so the gate's status condition is
   met. Its substance was not: the record was verified with all seven of its manual assessments outstanding and
   unauthored and eleven amendment provisions still `OUTSTANDING`, which the record states about itself. **This is now
   substantially resolved.** `WO-MOK-012` ratified all eleven provisions on 2026-08-20 and authored six of the seven
   assessments; the seventh is outstanding by the assurance owner's deliberate decision, for a reason recorded in
   Phase 1.5's status above, and it remains a disclosure any release record covering that chain inherits. What Phase 1.5
   still owes this gate is that one assessment, not the whole debt the gate was overridden against.

**The record was re-captured against the merged tree, and the re-derivation this section demanded is discharged.**
This paragraph previously read *"The record predates the merge with `master` and has not been re-captured against it"*,
and named two figures as measurements of code `master` had replaced: the oracle-4 frame capture enumerated its
viewports from the layout tier table that `WO-MOK-005`'s rule 5 amendment withdrew, and the test census reconciled
169 → 190 where the merged tree ran 193. **Both were answered by re-measurement rather than by adjustment.**
`VREC-MOK-010` was re-captured twice — `4f32a9f`, then `035a001`, then the verified candidate at `1a937a1`, each
retirement stated rather than corrected in place — and oracle 4 was re-derived against the merged tree at **996 bar
rows over the 85 of 157 probed frames that draw a roster, across all eight roster-drawing viewports**, where the
retired capture had 864 rows over 134 frames at four. The record calls that re-derivation *discharged at this commit*
and does not edit the amendment row that named it outstanding, because a row is never edited once written. The bar
arithmetic was unaffected throughout, the roster pane being 47 columns wide under both versions of rule 5. What
remains open under `WO-MOK-010` is item 3 above and nothing from this paragraph.

Everything measured, and everything it does not establish, is in
`engineering/simulation/evidence/WO-MOK-010` — start with its `../README.md`, then `completion-summary.md`.
The survivor figures there are downstream of one mid-implementation owner decision: `WO-MOK-010` stop condition 6
fired when `REQ-MOK-034`'s floor was missed on three of five declared seeds at the `0..=100` trait range first
specified, and the technical owner chose to narrow the range to `0..=40` rather than amend the floor
(`escalation.md`).

---

## Phase 2.5 — Names

**Goal.** Twelve Mokiterions who are someone rather than something: each has a name, and the presentation uses it.

**Why here, and why it is numbered 2.5.** It was not in the original sequence. It was asked for on 2026-08-19, in
those terms — you cannot be someone and not have a name — and it is numbered 2.5 for the same reason Phase 1.5 is
numbered 1.5: it delivers no simulation behavior. Nothing decides differently, nothing is perceived differently, no
rule reads a name, and no later phase needs one. It belongs after Phase 2 because Phase 2 is where a Mokiterion first
became distinguishable at all, and a name on interchangeable subjects would have been decoration.

**What was built**

- `SPEC-MOK-001`'s *Name*: twelve names — Zug, Krul, Quib, Sput, Trok, Womp, Hozz, Nurb, Vonk, Gorm, Xob, Drix —
  assigned to `M01` through `M12` by table lookup on the identifier's number, identical in every run at every seed
- The name reported once, first in the `agent_initialized` detail list, and on no other record
- The observer presenting it in the roster's first six columns, in the inspector heading beside the identifier, and as
  the map glyph — the name's initial uppercased, on twelve pairwise-distinct letters, replacing rule 2's `1`–`9` and
  `A`, `B`, `C` table
- The identifier retained everywhere it already was, because it is the join key between the panes and the stream

**What was deliberately not built**

| Considered | Decided | Why |
|---|---|---|
| Names generated from the seed | **Fixed list, assigned by number** | A generated name would make a run's cast depend on entropy, and the product owner accepted a hard-coded list explicitly. It also keeps the twelve initials distinct by construction rather than by luck |
| The name replacing the identifier in output | **Both, identifier unchanged** | Every pane and every test joins subject to record on `M01`–`M12`. Replacing it would have been a breaking change to the observation stream for a presentational gain |
| A name accessor in the observer's public interface | **`pub(crate)`** | `SPEC-MOK-004` rule 6's **Growth** clause: the interface grows only when an approved requirement needs it to, and a test is never that requirement. The public-tier tests read names from the engine's records through the already-public `Observer::events` |
| Renaming, nicknames, names for the dead differing from the living | **Out of scope** | Nothing asked for them, and a name that can change is a state field with rules attached rather than an identity |

**The additivity property Phase 2 established is preserved and was measured, not assumed.** A name costs no draw at
all — not even a side-generator draw, which is what the trait costs — so every pre-existing run makes the same
decisions in the same order and the only difference in its output is the name at the front of twelve lines. That is
checked across 90 recorded runs with the names projected back out, against the pre-change capture of the same 90.

**Status.** Implemented under `WO-MOK-011`, which is `implemented`, and **verified** under `VREC-MOK-011` at commit
`9ddcf83`, on `master`.

> **What this section said while the record was a candidate, and what the verification actually did with it.** It
> read: *"**Not verified.** `VREC-MOK-011` exists as a `ready` candidate bound to the commit that carries the
> implementation; it takes no decision, and the implementation agent can neither make nor approve one. One of
> `VER-MOK-011`'s seven manual assessments — the fifth, on the projection — has no author, and the contract is not
> satisfied while it is outstanding."* **The fifth assessment was never performed, and the record was verified
> anyway** — accepted unperformed by the assurance owner, which is the second of the two paths the candidate itself
> named. `manual-assessment.md` still reads `OUTSTANDING` and was not edited to suit the decision. So the sentence
> above is not stale in its substance: the contract is still not satisfied on that clause, and what changed is that
> an accountable owner decided on the record in that state and recorded what is accepted in its place. The
> re-derivation against the merged tree is **partly done and still standing** as that record's own closing review
> states.

**This chain was renumbered from `010` to `011`** on the owner's
decision, because `feature/phase-2-individuality` renumbered its own chain from 007 to 010 while this branch was
unpushed and the two claims met as eighteen merge conflicts. `011` is free on every remote ref, no measurement
changed, and `evidence/WO-MOK-011/renumbering.md` records what the rewrite touched and the one recorded digest it
could not preserve. **A second thing is still owed, and it is not about this work's substance either**: the packet
predates `master`'s release-ci work, so oracle 3's census, oracle 4's frames and `SPEC-MOK-004`'s counted rules are
re-derived when this branch merges `master`.

> **What the merge settled and what it left.** Re-measured on the merge rather than edited into agreement with it:
> **oracle 3's census, oracle 5's two halves, `SPEC-MOK-004`'s three rules, `render.rs`'s item counts, the declared
> gates and the harness state.** **Not re-derived: oracles 1 and 2, oracle 4's rendered buffers, and the mutation
> control** — `evidence/WO-MOK-011/merge/README.md` records those three as owed, and the sentence above expected
> oracle 4's frames among the settled ones, which they are not. A fourth item stands with them: one reading of stop
> condition 1 on the inline tests, the merge having widened it from two call sites to six plus one expected string,
> and the technical owner has not taken it. `VREC-MOK-011`'s `verified` status extends none of this — it binds
> `9ddcf83` and says so.

Three amendment rows written during this work — one each in `SPEC-MOK-001`, `SPEC-MOK-003` and
`SPEC-MOK-004` — are approved by the repository owner's act of 2026-08-19; the rows marked `OUTSTANDING` in those
specifications from earlier work were untouched by this work and outstanding when it completed, and were subsequently
ratified on 2026-08-20 under `WO-MOK-012`. Everything measured is in
`engineering/simulation/evidence/WO-MOK-011`.

**It does not close anything Phase 2 left open.** `fear` is still read by nothing, the trait vector is still one
trait, entropy substreams are still unimplemented, and high-class resource accumulation is still carried. Phase 2's
status section above stands unchanged. Two of those four have since moved, and neither moved here: `fear` acquired
its first consumer under `WO-MOK-016`, and the accumulation was closed under `WO-MOK-017`.

---

## Phase 3 — Social encounter and conflict

**Goal.** Complete the action contract: `approach`, `avoid`, `threaten`, `attack`, `fight`, `retreat`,
`surrender`.

**Why here.** Requires Phase 2's `fear` and Phase 1's perception of nearby agents. Completing the action set
before model integration means the decision contract is authored once.

**In scope**

- Encounter detection and resolution
- Combat energy and health costs; death through combat
- Response selection influenced by fear, health, energy, perceived relative strength, and prior interactions
- Interaction history feeding back into observation

**Critical constraint.** No population-level scripting. The concept explicitly forbids rules of the form
`if food < threshold: start_war()`. Conflict must be a consequence of individual decisions under world rules.
Verification should assert the **absence** of population-level triggers, not merely the presence of combat.

**Governance.** `REQ-MOK-005` ("Apply core actions") requires extension or supersession.

### What was built, and what is outstanding

The packet approved on 2026-08-20 is `INT-MOK-010`, `CAP-MOK-010`, `REQ-MOK-051` through `REQ-MOK-060`,
`VER-MOK-016`, `WO-MOK-016`, and in-place amendments of `SPEC-MOK-001`, `SPEC-MOK-002`, `SPEC-MOK-003`,
`REQ-MOK-005` and `REQ-MOK-034`. It is Phase 3.1: the whole action contract, one decision source that exercises
it, and no scripting of any kind.

The *In scope* list above is delivered with two reductions, both the product owner's decisions rather than
implementation shortfalls. "Perceived relative strength" is not read — damage depends on the striker's own
condition and on nothing about the target, so there is nothing to perceive and `REQ-MOK-059` forbids reading it.
"Interaction history feeding back into observation" is delivered as **one tick** of history, the attacks suffered
since the holder's last opportunity, because a longer window is a stored relation and the contact rule is
deliberately positional.

The critical constraint holds and is verified as an absence, as this section requires: `REQ-MOK-059` states that
no rule and no decision source reads any population-level aggregate, and every read in every rule, source and
validation path is enumerated against it.

**Two approved obligations were measured as failing inside Phase 3.1, and both were resolved before the phase
closed.** The sequence is recorded rather than the outcome alone, because what resolved them was a governed
amendment to a requirement rather than a correction to the code, and the coupling underneath them is still there.

What the first measurement of `social` reported:

- `REQ-MOK-058` requires at least five of twelve surviving at tick 1,000 under `social` at the default density
  **and** at least one death attributable to combat, on every declared seed. Measured: 6, 4, 8, 4 and 5
  survivors, and **zero** combat deaths on all five seeds.
- `VER-MOK-016` oracle 4 requires each of the seven targeted verbs to apply somewhere in the declared matrix.
  `surrender` was never proposed in any declared run.

Both followed from one coupling, and it was a design fault rather than a defect in the code. `fear` rises `+10`
for every tick company is *perceived*, at radius 16, while engagement required *contact*, at radius 1, below a
`fear` of 30. Fear therefore crossed the threshold on the third perceiving tick while closing the perception
radius takes fifteen, so a Mokiterion was nearly always past the threshold by the time it could act on it. Every
strike in the entire declared matrix landed at tick 1, 2 or 3, between Mokiterions initialized already in contact;
after tick 3 the source routed to `avoid` for the rest of the run, which also displaced the seek-move and dropped
meals eaten from 378–417 per run to 205–304. That, not the combat, was what lowered the survivor count. `avoid`
was proposed 6,329 times against `attack`'s 3.

**Resolved on 2026-08-20**, on the product owner's selection of package A of `WO-MOK-016`'s `escalation.md` §8:
rule 12 stays perception-driven exactly as Phase 2 approved it, `REQ-MOK-057`'s branch order is amended so that
rule 19's case 3 — walk toward food — precedes the social branches, `ENGAGEMENT_FEAR_THRESHOLD` becomes 95, and
`REQ-MOK-058`'s floor of five is ratified unchanged. Five artifacts were amended, each with its own
amendment-record row, before any code was written. Measured at the resulting candidate: 9, 10, 9, 9 and 11
survivors; combat deaths 1, 2, 2, 3 and 1; `surrender` proposed 5, 10, 8, 6 and 7 times; meals recovered to
326–389 against 205–304, so the starvation mechanism is gone. Both obligations hold, and `WO-MOK-016` is
`implemented` and verified at `VREC-MOK-017`, bound to `ecba9fee`.

**`VREC-MOK-017` is the second record written for this work order, and the first is `superseded`.**
`VREC-MOK-016`, bound to the governance transition commit `45396016`, never reached `verified`: it was captured
`ready` and moved straight to `superseded` on 2026-08-21 by the repository owner acting as accountable assurance
owner, with `superseded_by = ["VREC-MOK-017"]`. Its own reason is that its candidate holds
`manual-assessment.md` with **eight of eleven record blocks blank**, and `VER-MOK-016` counts an unrecorded
assessment as an outstanding one — so `verified` at that commit would have asserted satisfaction of a contract the
commit does not satisfy. The owner then took the eight assessments, and `ecba9fee` is the commit that holds them.
**One trap for anyone reading its metadata:** `VREC-MOK-016` does carry a `verified_at`, and that field is a
capture timestamp rather than an approval time — its own record says so — so the field is not evidence that the
record was ever `verified`. The status is. The retired record is worth reading rather than skipping: its
measurements are reproducible only at `45396016`, which is what a superseded record is for.

**The mitigation is not a repair, and the diagnosis above is why.** Raising the gate to 95 buys ten perceiving
ticks instead of three, which is enough to reach contact on the declared seeds; it does not change that `+10` up
against `−5` down saturates, so `fear` remains a stopwatch on company in sight rather than an appraisal of it.
Measured at `WO-MOK-017`'s candidate, at the default density across the five declared seeds: every one of the 63
strikes was proposed at a `fear` of 90 or below, thirteen of them at exactly 90; every one of the 1,724
approaches at 90 or below; all 114 threats at exactly 100; and all 4,268 avoidances at 95 or 100. Nothing
straddles the line. That is the gate working exactly as written, and it is also the shape of the fault: which
side of it a holder falls on is decided by how long it has had company, not by anything about that company.

`REQ-MOK-060`'s resource-composition ceiling was the third failing measurement, 14 of 15 cells, deferred to a
second amendment by the owner's decision of 2026-08-20 and descoped out of `WO-MOK-016` the following day. It is
`WO-MOK-017`, recorded under *Current state* above: the ceiling amended to three fifths, and the allowance term
rule 5 had lost restored.

**Both lifecycle acts have since been taken, and the chain is verified.** Pull request #39 merged
`feature/resource-composition-ceiling` into `master` as `f7b1c45` on 2026-08-21, carrying the candidate `26ae6ba`
and the merge `ae2e44f` above it, so the corrected world rule reached the integration branch a day before anything
accepted it. Pull request #41 closed the governance tail on 2026-08-22, merging as `5bdf607`: `WO-MOK-017.md` now
reads `status = "implemented"`, and **`VREC-MOK-020` is `verified`** against `VER-MOK-016`, bound to
`a30ae327a92f5acdfb294a3c3d98e32b806300d1` with a clean worktree, 319 declared evidence paths and
`artifact_snapshot_sha256 = 24a6fbd1…`. Between it and `VREC-MOK-017` all **93** of that contract's coverage rows
are claimed.

**What that record binds, and the exposure it carries.** Its candidate is a governance commit on the closure
branch, so it decided about a tree rather than about integrating one — it could not have gated #39's merge, which
had already happened, and the record says so in its own text rather than leaving a reader to notice. `a30ae32`
**is** now an ancestor of `master`, measured, because #41 merged; the binding therefore survives, and it survives
only while that history stands. The record is `verified`, and `VERIFICATION_RECORD.template.md` admits supersession
only from `ready` — so a rewrite of `a30ae32` would leave it binding a commit that does not exist, with no remedy
available at all. That is a permanent constraint on this history, not a note about one pull request.

**The record was written as `VREC-MOK-020`, and the collision in the evidence file still stands.** The work order's
own merge evidence (`evidence/WO-MOK-017/merge/README.md`) names `VREC-MOK-017` four times as the record for this
tree, and that identifier was already spent — `verified` against `WO-MOK-016` at `ecba9fee`, two sections up. The
free number was found by sweeping every ref rather than by taking the local maximum plus one, which is how `020` was
reached. Note what that number is: a **verification-record** number, not a work-order one. The two sequences
advance independently, so `VREC-MOK-020` sits over `WO-MOK-017` while `WO-MOK-019` carries `VREC-MOK-019` — a
reader who assumes the pairing will mis-attribute records here. The evidence file is deliberately **not** corrected:
a `verified` record's declared evidence is not rewritten after the decision, so the four wrong identifiers are named
in the record and left exactly as they printed.

**The packet's outstanding items were closed with the record, bar one.** The three manual assessments that were
prepared and unsigned — all three the product owner's — are now **recorded, none outstanding**, and of the nine
findings raised, **eight carry a disposition**. Finding 7 is the assurance owner's and was carried undecided
*through* the verification decision rather than resolved by it: `VER-MOK-016`'s realignment row of 2026-08-21 reads
*"Manual assessment 7"* where the item amended is *acceptance scenario 7*, and that row is **still owed**. Two of
the packet's thirteen readers still print `RESULT: FAIL` on the merged tree — both because `WO-MOK-019` rewrote
`simulation.rs` underneath them, neither a world-rule failure, and both retained as they printed rather than
repaired. Both failures were read and **accepted** by the owner, corroborated by a re-taken 120-cell capture
comparison from the merge commit's own archived tree: `RESULT: PASS — 120 of 120 cells identical on all four
columns, 0 differing`. Identical output streams cannot come from a changed world rule, which is why the acceptance
rests on that comparison rather than on the readers' own text.

> **Earlier forms, for a reader diffing this file.** Two, and the sequence is the point. The original read
> *"**Implemented and not verified** — the verification record is the assurance owner's act and has not been
> made."* At `f7b1c45` neither half was safe: the work order had not reached `implemented` and had not left
> `in_progress`, so the roadmap was claiming a transition the graph did not carry. The reconciliation of 2026-08-22
> replaced it with *"Its status is `in_progress`, and its chain is merged … Neither has been taken, so nothing
> accepts this tree"*, which was measured and true at that commit and was overtaken the same day by #41. So this
> paragraph has claimed a transition that had not happened, then correctly recorded its absence, and now records
> that it was made — which is what re-measuring against the graph rather than carrying a claim forward looks like
> when the graph is moving faster than the document.

`social` is therefore honestly described as **implemented, no longer starving, and still not the default**;
`--policy` unspecified selects `reference`. On the corrected world it leaves 9, 7, 9, 8 and 9 of twelve alive
against its floor of five, with 1, 2, 2, 3 and 1 of the deaths attributable to combat. No measurement was
weakened, skipped or removed to make any of this green: where a requirement moved, it moved in the open and
before the code did.

---

## Phase 4 — Analytical observability

**Goal.** Build the measuring instrument.

**Why here.** The claim that outcomes are not predetermined cannot be supported by a single-line text summary.
It requires outcome distributions across many seeds. **This phase is independent of Phase 3** — different
modules, no shared dependency — so the two may proceed in parallel.

**In scope, as originally written**

- Structured event stream (for example JSONL) alongside the existing text stream
- World-level metrics: surviving population, population per territory, available food, consumption and
  regeneration, average health, satiety, and energy, territory crossings, conflict frequency, deaths,
  survival time
- Multi-seed batch runner and run persistence
- Outcome classification (coexistence, famine, collapse, extinction, and similar)

**Constraint.** Preserve the existing text output so `REQ-MOK-010` remains satisfied. Add, do not replace.

**Split into 4a and 4b on 2026-08-20**, in `docs/PHASE_4_PROPOSAL.md` and authorized as drafted. The four items
above are not one deliverable: the first two are a change to the engine, the last two are a consumer of that
change, and the consumer's shape depends on a measurement the engine has to produce first. The split is between
producing measurable output and consuming it.

**Relationship to Phase 1.5.** Phase 1.5's terminal observer does not deliver any part of this phase, and this
phase does not make it redundant. Phase 1.5 answers *what is happening in this run*; this phase answers *what
happens across many runs*. Phase 1.5's event export is a per-run record in the existing text format, deliberately
not a structured stream, so the structured output this phase adds is still new work. If Phase 1.5's snapshot
contract turns out to be the natural source for structured output, that is a convenience to exploit rather than a
dependency to rely on.

### Phase 4a — Structured measurement in the engine

**Status: implemented under `WO-MOK-019`, verified at two commits, merged into `master`, and not released.** The
artifact packet — `INT-MOK-009`, `CAP-MOK-009`, `REQ-MOK-042` through `REQ-MOK-046`, `SPEC-MOK-006`,
`ADR-MOK-005`, `WO-MOK-019` and `VER-MOK-012` — was drafted, reviewed and validated by the owner on 2026-08-20, and
the whole chain was implemented, evidenced, verified and closed the same day in seven commits, one act each.

`VER-MOK-012`'s seven oracles are executed and passing, its eight manual assessments are recorded, and its evidence is
retained as a 56-file packet. **The three conditions this section set for completeness are met.** What the record
accepts rather than satisfies is stated in it and is not smoothed over here:

- **Two *Evidence retention* bullets are met by substitution rather than as written** — post-change standard output is
  held as digests, and four full sink streams are retained where the bullet asks for thirty. Nothing was re-captured
  and nothing deleted, so the contract is answered in substance and unsatisfied on those two bullets literally.
- **Nine defects measured in six approved artifacts are deferred, not corrected**, to a Phase 4b correction work order
  **that does not exist yet**. One of them — `SPEC-MOK-006` rule 3.2's direction domain — is why oracle 5's size
  assertion compares the engine against itself for one domain of thirteen; the measured residual is 0 diagonal
  direction words in any of the seven retained streams.
- **Three carried-forward `OUTSTANDING` amendment rows stand untouched**, none of them this chain's to pay.

**Merged, and verified twice — at neither of the merges into `master`.** `50364a3` is an ancestor of `master`: pull
request #31 merged `feature/phase-4a-definition` into `master` as `fac152f` on 2026-08-21, and pull request #38
followed it as `3f47743` the same day, carrying the two governance commits that prepared `VREC-MOK-019` and moved it
to `verified`. `master` moved under the branch twice while the packet was being built, so the branch merged forward
twice — `1e09f85` for `WO-MOK-018` and `e96648a` for this work order — and the **second** of those was itself
verified: `VREC-MOK-019` is `verified`, bound to `e96648a2`, against `VER-MOK-012`, and it records `00e58a9` as a
provenance it rejected rather than silently discarding it. `VREC-MOK-012` at `50364a3` stands unchanged beside it;
neither record is re-pointed, because a record binds the commit it was written against and there is no rebinding.
Both bound commits are branch commits, so **no record binds `fac152f` or `3f47743`** and the pattern here is the
one this repository has settled into: records bind branch commits, `master`'s merge carries none, and more than one
record per work order is normal rather than exceptional.

**Not released**: no release record binds this work. `RLS-MOK-001` is v0.1.0 at `755db72`, `REL-MOK-001` gates only
`WO-MOK-001` through `WO-MOK-007` and `WO-MOK-009`, so a v0.2.0 needs a new release contract; and `WO-MOK-008`, the
release-authorization chain, is unrelated and still `draft`.

> **Earlier form, for a reader diffing this file.** It read *"**Not merged.** `50364a3` is on
> `feature/phase-4a-definition` and is not yet an ancestor of `master`; pull request #31 is open, out of draft and
> green. By the precedent of `VREC-MOK-009` through `VREC-MOK-011` — one record per work order, each binding a
> branch commit that became an ancestor of `master` through its merge — the merge does not call for a second
> record."* The first two sentences were true when written and are now false. The third was an argument rather than
> a measurement, and it did not survive: a second record was made for this very work order, and the practice it
> appealed to had already been broken by `VREC-MOK-015` — `verified`, bound to `WO-MOK-014`'s merge commit
> `9599c0a9`, and on `master` since 2026-08-20, before this paragraph's own chain merged. `VREC-MOK-019` names it
> as the precedent it follows and reaches the same conclusion on the same fork, so the second record here is a
> settled pattern rather than an exception.

**What was built**

- One record per line, four kinds, each self-describing through a leading `record` field: `header` once at the top
  with the resolved configuration and a schema version, `event` for every text event line, `metrics` at the end of
  every completed tick, `run` once at the bottom
- The event projection placed at the single point every authoritative event already passes through, so the
  one-to-one correspondence with the text stream is structural rather than maintained by discipline
- Per-tick metrics that answer the roadmap's list without interpreting it, and the run-level totals that a reader
  would otherwise have had to derive by re-implementing the rules: crossings, consumption by class, regeneration,
  regenerations skipped and why, and each Mokiterion's death tick
- `--events-path`, and the destination's whole lifetime owned by the binary target: the library resolves no path,
  opens no file and removes none, which keeps the engine's prohibition on interpreting input as a path true of the
  library rather than merely observed by it

**What was deliberately not built**

| Considered | Decided | Why |
|---|---|---|
| A JSON library | **Hand-written writers, no dependency** | `ARCH-MOK-001` keeps the engine's dependency table empty and the owner declined to open it for this. The closed value alphabet is what makes that safe: no value can contain a quotation mark or a backslash, so there is nothing to escape |
| Any floating-point figure | **Integers only, sums rather than averages** | An average is a float, a float's text form is platform-sensitive, and the whole value of the stream is that two files can be compared byte for byte. A consumer that wants a mean has the sum and the count |
| An outcome label of any kind | **Prohibited, not deferred** | This was named in `WO-MOK-019` as the most tempting scope creep in the work. A file that labels its own numbers hands everyone downstream somebody else's opinion, and Phase 6's evaluation is where a classification is argued on evidence |
| A timestamp or a duration | **Neither, anywhere** | The same seed and build must produce an identical file on any machine on any day, so that a diff between two runs is a statement about the simulation |
| A reader, a schema file or an example analysis script | **None, in this phase** | 4a produces; 4b consumes. A consumer written here would be written before the measurement that decides 4b's shape |

### Phase 4b — Distribution and classification

**Deferred, and cheaper once 4a exists.** Multi-seed batch execution, run persistence across runs, distribution
across seeds, and outcome classification stated as a table over 4a's `run` records.

**What it needs before it can be specified honestly**, and what 4a produces toward it: a measured answer to whether
a batch loop needs to be a program at all. If a shell loop over the existing binary plus a script under `scripts/`
— where the Python instruments and their tests already live — produces the distribution evidence Phase 6 needs,
then 4b is a runbook and a verification contract, and `ARCH-MOK-001`'s third-package prohibition is never touched.
If it does not, the third package is argued on that finding. Deciding it now would be deciding it without the
measurement.

Phase 4a is independent of Phase 3 and does not block it. If Phase 3 lands first, its events join the projection
for free and its metrics arrive as `schema: 2`.

---

## Phase 5 — LLM decision source

**Goal.** Replace the baseline with a model-backed decision source at the **existing** trust boundary.

**Why last.** Every input the model consumes is produced by Phases 1–4. This ordering also matches
`INT-MOK-001`'s own recorded assumption that an engine-first foundation should be verified before adding an
external decision source, and it defers the only expensive, nondeterministic, credential-bearing work until
all cheap deterministic verification is complete.

**The central decision: determinism strategy.**

| Option | Assessment |
|---|---|
| **Record / replay** (recommended) | Live runs record every prompt→response pair to a transcript; replay mode reads the transcript and is fully deterministic. Preserves reproducible verification without constraining live behavior. |
| Response cache keyed on observation hash | Makes repeats deterministic but collapses exactly the individuality Phase 2 establishes. |
| Temperature 0 | Not a bitwise determinism guarantee from any provider. Should not be treated as one. |

Record/replay aligns unusually well with the project's engineering objective: **the transcript becomes
commit-bound verification evidence**, making it provable which model outputs produced a verified run.

**Governance.** `ADR-MOK-001` does **not** require superseding. `ARCH-MOK-001`'s own rationale already defers
"an external model provider to an adapter at the same boundary." What is needed is a **new** ADR covering the
provider adapter and the determinism strategy, plus an explicit decision on `REQ-MOK-009` and on
`INT-MOK-001`'s 100% reproducibility success measure.

**Constraints.** Confirm the exact OpenAI model identifier before integration. Credentials remain outside the
repository per `REPOSITORY_CONTEXT.md`. The decision source must remain unable to mutate authoritative state.

---

## Phase 6 — Emergence evaluation

**Goal.** Produce the actual proof-of-concept result.

**In scope.** Run random, heuristic, and model-backed policies across many seeds and report outcome
distributions: sustainable consumption, famine, migration, territorial behavior, raiding, aggression and
retaliation, surrender, war, asymmetric collapse, coexistence, extinction.

**Why this closes both objectives.** It answers whether emergence is real — model outcomes differing
*qualitatively* from the heuristic control, not merely scoring better — and simultaneously demonstrates
whether SE Harness traced the entire lineage from intent to exact commit.

**Success definition.** A defensible comparison. No particular behavior is required to occur; the requirement
is that the conditions permit emergence and that observed outcomes are not engine-determined.

---

## Cross-cutting work

### Governed chains delivered outside the phase sequence

**Five work orders were authorized, implemented and verified without a numbered phase.** Each states a property of
the repository or of the build rather than a behavior of the world, which is why none of them earned a phase — a
phase in this document is a step toward the success criteria, and none of these moves the simulation. They are
listed because the phase sequence is not a census of delivered work, and a reader who treats it as one will
conclude either that this work was never planned or that it was never governed. Both would be wrong.

| Work order | What it delivered | Where it hangs | Record | Released |
|---|---|---|---|---|
| `WO-MOK-003` | Library and binary targets split, tests placed by the access each needs | `REQ-MOK-016`, `REQ-MOK-017` → `CAP-MOK-003` | `VREC-MOK-003` at `a7f39f1` | v0.1.0 |
| `WO-MOK-004` | Every option's effect and default stated in the help output | `REQ-MOK-018` → `CAP-MOK-001` | `VREC-MOK-004` at `95f0aa2` | v0.1.0 |
| `WO-MOK-006` | Each package in its own directory, the observer given its own tested contract | `REQ-MOK-028` – `REQ-MOK-030` → `CAP-MOK-005` | `VREC-MOK-006` at `dd0c2c0` | v0.1.0 |
| `WO-MOK-009` | The release authorization gate, the release process, the compiler declaration | `REQ-MOK-035` – `REQ-MOK-039` → `CAP-MOK-007` | `VREC-MOK-008` at `d35f817` | v0.1.0 |
| `WO-MOK-014` | The engine's empty-dependency rule replaced by a declared-set comparison, checked on every pull request and at release | `REQ-MOK-050` → `CAP-MOK-004` | `VREC-MOK-014` at `65ac88b`, `VREC-MOK-015` at `9599c0a` | **no** |

The first four are covered a second time by `VREC-MOK-009`, the aggregate candidate at `755db72` that names eight
work orders and is the commit `RLS-MOK-001` released as v0.1.0.

**`WO-MOK-014` is the one that needs more than a table row**, and until now the roadmap mentioned it only inside a
footnote about identifier renumbering in Phase 1.5. It is the chain that ended `ARCH-MOK-001`'s empty-dependency
rule as an absolute and replaced it with something checkable: **`ADR-MOK-006`** — *"Criteria-based admission of
third-party crates in both packages, against a declared set, replacing the engine's empty-dependency rule"* —
decides both `ARCH-MOK-001` and `ARCH-MOK-002`, and **`REQ-MOK-050`** states the resulting obligation normatively:
each package's resolved dependency set must equal the set its specification declares, at the declared versions and
feature sets, with no crate providing network access, credential handling, an asynchronous runtime, a database, a
plugin system or dependency injection in either package, and no user-interface dependency outside the observer
package.

**Why it has no phase of its own is structural rather than an oversight.** `REQ-MOK-050` derives from
`CAP-MOK-004`, Phase 1.5's capability, so the chain reused the observer phase's capability instead of opening a new
intent. That is also the reason it is invisible: a reader walking intents down to phases never passes through it.

**What it leaves behind is a standing gate rather than a delivered feature.** `scripts/check_declared_dependencies.py`
is 1,225 lines and runs in two places — `.github/workflows/dependency-declarations.yml`, whose entire trigger list
is `pull_request`, and `.github/workflows/release.yml`. So every proposed change to this repository is now measured
against the declared sets, and **Phase 5 in particular cannot add a provider client without moving a declaration
through the technical owner first.** Phase 5's section does not yet say so; that gap is noted and not closed here.

Its governance reach was wide: **sixteen amendments, twelve of them in governed artifacts** — `ARCH-MOK-001`,
`ARCH-MOK-002`, `SPEC-MOK-002`, `SPEC-MOK-003`, `SPEC-MOK-004` and `SPEC-MOK-005`, each with its own approval row
by the technical owner on 2026-08-20 — and four in repository-owned files including
`engineering/REPOSITORY_CONTEXT.md`. Measured in `evidence/WO-MOK-014/WO-MOK-014-amendments.md`.

### LLM feasibility experiment (parallel to Phases 1–2)

A bounded, throwaway spike measuring cost per 1,000 ticks, latency per decision, and schema-valid response
rate. SE Harness 0.4.0 supports `experiments/` as a first-class artifact type, so this can be governed
evidence rather than an ad-hoc investigation. It retires integration risk early **without** committing the
architecture, which is what makes deferring Phase 5 to last acceptable.

### Ordering alternative

The alternative to the sequence above is front-loading the model integration to Phase 3, immediately after
perception and traits, to retire integration risk earlier — accepting that the decision contract is rewritten
when combat lands. The roadmap recommends against this, preferring the feasibility experiment as a cheaper
means to the same risk reduction.

## Governance deltas by phase

| Phase | Existing artifacts affected |
|---|---|
| 0 | `ARCH-MOK-001` schema migration; managed harness files; evidence-naming convention |
| 1 | `SPEC-MOK-001` amended in place; new intent and capability added; no existing requirement changed |
| 1.5 | `ARCH-MOK-001` amended in place to scope its one-crate, empty-dependency and UI-framework rules to the engine package; new intent, capability, nine requirements, specification, architecture and ADR added; `ADR-MOK-001` not superseded; `SPEC-MOK-001` unchanged |
| 2 | New intent, capability and four requirements added; `SPEC-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-003` all amended in place; `ARCH-MOK-001` confirmed unchanged; no existing requirement changed; observation contract extended. Broader than this row anticipated: the trait had to be stated in `SPEC-MOK-001`, the new tests' tiers in `SPEC-MOK-002`, and the fourth gauge in `SPEC-MOK-003` |
| 3 | `REQ-MOK-005` amended in place rather than superseded; new intent, capability and ten requirements added; `SPEC-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-003` all amended in place; `REQ-MOK-034` amended because a fourth source must not be read as extending the floor it freezes; `ARCH-MOK-001` confirmed unchanged. Broader than this row anticipated in the same way Phase 2 was: the carried floors had to be re-measured, `REQ-MOK-057`'s branch order and engagement threshold had to be amended after the first measurement, and `REQ-MOK-060`'s numeric ceiling took the second amendment it was deferred to — made under `WO-MOK-017` on 2026-08-21, after the requirement was descoped out of `WO-MOK-016` |
| 4a | New intent, capability and five requirements added; new specification `SPEC-MOK-006` and new `ADR-MOK-005`; `ARCH-MOK-001`, `SPEC-MOK-001` and `SPEC-MOK-002` amended in place; `REQ-MOK-010` preserved and extended additively — not one byte of the text stream changed; no existing requirement changed. Broader than this row anticipated by one artifact: **`SPEC-MOK-004` rule 11 was amended too**, beyond `ADR-MOK-005`'s approved list, because rule 11 obliges a work order that changes the test census to correct the recorded figures — 212 → 246 in the workspace, 85 → 119 in the engine. That row was approved separately by the technical owner on 2026-08-20 |
| 4b | Not yet argued. Depends on 4a's measurement of whether a batch loop needs to be a program: either a runbook and a verification contract with no architecture delta, or a third package argued against `ARCH-MOK-001` |
| 5 | `REQ-MOK-009` and `INT-MOK-001` reproducibility measure decided; new ADR for provider adapter |
| 6 | New verification contract; no existing artifact changed |
| *No phase* — repository contract (`WO-MOK-003`, `WO-MOK-004`, `WO-MOK-006`, `WO-MOK-009`) | Delivered inside the v0.1.0 window against `CAP-MOK-001`, `CAP-MOK-003`, `CAP-MOK-005` and `CAP-MOK-007`, none of which any phase row above claims. Target split, per-package directories and contracts, help-output completeness, and the release authorization gate. All four released under `RLS-MOK-001`. See *Governed chains delivered outside the phase sequence* |
| *No phase* — dependency declaration (`WO-MOK-014`) | New `ADR-MOK-006` **deciding both `ARCH-MOK-001` and `ARCH-MOK-002`**, which is the widest architecture delta any chain in this repository has taken: the engine's empty-dependency rule stops being absolute and becomes a declared-set comparison. New `REQ-MOK-050` under Phase 1.5's `CAP-MOK-004` rather than a new intent — which is why this chain has no phase and why it was invisible to this document until 2026-08-22. Sixteen amendments, twelve in governed artifacts: `ARCH-MOK-001`, `ARCH-MOK-002`, `SPEC-MOK-002`, `SPEC-MOK-003`, `SPEC-MOK-004`, `SPEC-MOK-005`. Two CI gates added, one on every pull request. **Constrains Phase 5**, whose row above does not yet say so |

## Open decisions requiring owner input

Two decisions are cheaper to settle now than at the phase in which they bind:

1. **Target carrying capacity for Phase 1.** *Decided 2026-08-17:* a floor of at least eight of twelve survivors
   at 1,000 ticks under the reference source, recorded as `REQ-MOK-014` and measured on the seed set declared in
   `VER-MOK-002`. All twelve surviving on every declared seed is an adverse observation, not a success.
2. **Determinism strategy for Phase 5.** Record/replay is recommended. This propagates into Phase 4's output
   format, so deciding it before Phase 4 avoids rework. *Still open after Phase 4a, and no longer urgent for this
   reason:* 4a's stream is a complete, ordered, byte-reproducible record of everything the engine did, so it can
   already be replayed against. Whether Phase 5 replays *this* stream or records provider exchanges separately is a
   Phase 5 decision that 4a does not foreclose either way.

A third was settled during Phase 2 rather than before it, and is recorded here because the phase order did not
anticipate it:

3. **Trait range for Phase 2.** *Decided 2026-08-19:* `0..=40` rather than the `0..=100` first specified, chosen on a
   fifty-seed distribution rather than on the five declared seeds, because the declared-five result is not monotonic
   in the bound. The upper half of the original range was measurably dominated, not a second strategy. Recorded in
   `SPEC-MOK-001`'s *Behavioral trait* subsection and in `WO-MOK-010`'s evidence as `escalation.md`.

A fourth was raised by Phase 3.1's first measurement and settled inside that phase, for the same reason:

4. **`REQ-MOK-057`'s thresholds and branch order.** *Decided 2026-08-20:* package A of `WO-MOK-016`'s
   `escalation.md` §8 — rule 12 stays perception-driven, rule 19's case 3 is hoisted ahead of the social branches,
   `ENGAGEMENT_FEAR_THRESHOLD` moves from 30 to 95, and `REQ-MOK-058`'s floor of five is ratified unchanged. The
   alternative of rescoping the fear rise was withdrawn. `REQ-MOK-058` and `VER-MOK-016` oracle 4 both hold on the
   measured curve afterwards. What the decision deliberately did not settle is the coupling that produced the
   failure: `fear` still rises `+10` per perceiving tick against a `−5` decay, so it saturates and the gate is a
   stopwatch on company in sight rather than an appraisal of it. Phase 3's status section measures where that
   leaves every social verb, and `WO-MOK-016`'s evidence carries the packages the owner chose between.

## Maintenance

Update this roadmap when a phase is converted into an approved artifact chain, when a phase is completed,
reordered, or abandoned, or when a constraint above is invalidated by measurement. Formal authority always
comes from typed artifact metadata and accountable lifecycle decisions under `engineering`, never from
this document.
