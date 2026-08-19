+++
id = "WO-MOK-010"
type = "work_order"
title = "Give each Mokiterion a derived trait, a fear attribute, and a decision source that reads them"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-19"
updated = "2026-08-19"

[assurance]
commit_bound_verification = "required"
rationale = "The work changes executable behavior at the centre of the simulation: a fourth attribute on every Mokiterion, a derived per-Mokiterion trait, a third decision source, and a new survivor floor. Its load-bearing claim is a claim of absence — that the baseline and reference sources produce byte-identical projected output to the build before the change — and that claim is of the same standing as REQ-MOK-009's determinism claim: it cannot be asserted, and it can only be checked against a capture bound to the commit the work began from. A derivation that consumed one extra value from the shared entropy stream would pass every behavioral test and silently retire REQ-MOK-014's approved survivor floor, which later phases and every policy comparison rest on. The change also grows the engine's public interface, which SPEC-MOK-002 rule 5 maintains as a closed contract, and closes a finding recorded against an unverified record."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-031", "REQ-MOK-032", "REQ-MOK-033", "REQ-MOK-034"]
specifications = ["SPEC-MOK-001", "SPEC-MOK-002", "SPEC-MOK-003"]
verification = ["VER-MOK-010"]
+++

# Work Order: Give each Mokiterion a derived trait, a fear attribute, and a decision source that reads them

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and the retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

No `architecture` relation is declared. No active architecture carries an `addresses` edge to `REQ-MOK-031`,
`REQ-MOK-032`, `REQ-MOK-033` or `REQ-MOK-034`, so architecture is not applicable and fabricating coverage would be
worse than omitting it. That is a statement about the traceability rule, not a claim that the change is
architecturally trivial: it adds a field to a public type and a variant to a public enum, so `ARCH-MOK-001`'s
component boundaries, its prohibition on any public item exposing a mutable borrow into authoritative state, and its
prohibition on any engine dependency are all in force and are checked under `VER-MOK-010`. **The technical owner must
confirm at approval that `ARCH-MOK-001` requires no amendment.** If it does, this work order needs an `architecture`
relation and a deciding ADR before it can be approved.

**This work order cannot be approved before its governing artifacts are.** It depends on `INT-MOK-006`,
`CAP-MOK-006`, `REQ-MOK-031`, `REQ-MOK-032`, `REQ-MOK-033` and `REQ-MOK-034` being approved by the product owner, and
on `VER-MOK-010` being approved by the assurance owner. Every one of them was `draft` at the time this work order was
written. **All seven, and this work order, were approved by the repository owner on 2026-08-19**, acting as product
owner, technical owner and assurance owner; the same act confirmed that `ARCH-MOK-001` requires no amendment, on the
ground that the added snapshot field carries a value and grants no borrow into authoritative state, which is what its
prohibition addresses. Preflight must report this exact work order as eligible before implementation begins.

Status moved to `in_progress` on 2026-08-19, in the same commit as the first code change, on the owner's instruction
to implement. The pre-change capture named under *Evidence* was taken before that change, against the commit recorded
in `evidence/WO-MOK-010/baseline/COMMIT.txt`, because a capture taken afterwards could not establish the absence
claim this work order rests on.

### Transition to `implemented`

**Status moved from `in_progress` to `implemented` on 2026-08-19**, on the repository owner's instruction, given as
engineering owner: "you can push on existing branch, swicth VREC-MOK-010 to verified, commit and push again (implying
transitioning WO-MOK-10)". The implementation agent recorded the transition and did not decide it. The status had
stood at `in_progress` through implementation, through three verification captures and through the closing review,
because no earlier instruction covered a lifecycle transition and this work order's *Authorized decision envelope*
does not place one inside the agent's local decisions.

**What is implemented.** Everything under *In scope*: the derived `waste_tolerance` trait over `0..=40`, the `fear`
attribute with the approved `+10`/`-5` ratchet, `SPEC-MOK-001` rule 19's trait-aware decision source as the third
`--policy` value, the fourth roster gauge, and the twenty-one tests that carry them. The candidate commit is
`1a937a1a9a3ff24c23e45946ad023bde95f83d02`, whose gates are recorded in `VREC-MOK-010` and reproduced from
`evidence/WO-MOK-010/static-checks.txt`: 200 tests over 20 runners with 0 failed, 0 ignored and 0 filtered out,
`cargo fmt` and `cargo clippy -D warnings` clean with both crates re-linted from a cold target directory, the engine's
dependency table still empty at one `cargo tree` line, the public interface grown by exactly the one snapshot field
and the one enumeration variant, and validator PASS at 80 artifacts with 0 errors and 0 warnings across all four
planes. Nothing outside *In scope* changed in the implementation: `git diff --stat 035a001 1a937a1 -- ':!docs'` is
empty across the closing review, and the code lineage is `60fda9f` → implementation → merge at `7a2b502` →
`035a001` → `1a937a1`.

**What this status does not do.** It does not verify the work: `WORKFLOW.md` states that the verification record moves
separately through an accountable human decision and that work-order status never substitutes for it. It does not
release anything, and no release record exists. It does not discharge the one obligation this chain still carries —
the `VREC-MOK-005` layer, whose eleven provisions and seven manual assessments the owner's override of 2026-08-19 left
standing with a stated obligation: a work order of its own, completing before the next release record. That work order
does not exist yet, and *Out of scope* above puts resolving `VREC-MOK-005` outside this one.

**Two derived figures move with the status, and neither is a new defect.** The harness dashboard goes from 12 warnings
to 13; the thirteenth is the `W-HEX-001` that every implemented work order in this repository already carries, because
evidence discovery keys on file names beginning with the work-order identifier and this chain retains its evidence in
a directory named `WO-MOK-010/` instead. `inspect` drops *Active work* from 1 to 0, so its recommendation to continue
bounded work on a change that is complete is gone. Separately, `evidence/WO-MOK-010/analysis/amendments.py` expects
this file to read `in_progress`, which was true at the commit it was captured against; that retained capture describes
the candidate commit and is not edited to track a status that moved after it, so re-running it here reports that one
control failing for the stated reason. `evidence/WO-MOK-010/assurance-decision.md` records the measured before and
after.

### Decision record

On 2026-08-19 the repository owner, acting as product owner and technical owner, took the six open decisions this
packet was drafted around. **They are design decisions, not approvals.** Every artifact in this chain remains `draft`,
and none of the three specification amendments below has been approved. The decisions are recorded here so that the
substance a later approval acts on is written down rather than reconstructed.

| Decision | Outcome | Role |
|---|---|---|
| The two scope cuts `INT-MOK-006` declines — per-agent entropy substreams, and traits beyond the one with a consumer | Both accepted. One trait, no substreams, so the phase stays additive and `REQ-MOK-014`'s floor is untouched | product owner |
| `REQ-MOK-034`'s survivor floor | Eight of twelve, the number `REQ-MOK-014` already carries. Individuality must not cost habitability | product owner |
| Where the trait-aware source is specified | Appended as rule 19 rather than renumbering thirteen rules | technical owner |
| The roster's bar width once the fourth slot is filled | Narrowing accepted: two cells at the reference roster's 45-column interior | technical owner |
| `fear`'s driver and step sizes | Any perceived living Mokiterion, `+10` and `-5`. No distance constant is added | technical owner |
| The trait's range | `waste_tolerance` over `0..=100`, uniform. **Superseded by the row below on 2026-08-19; kept because a later reviewer should see what was decided before measurement, not only after** | technical owner |
| The trait's range, corrected under stop condition 6 | Narrowed to `0..=40`, uniform, chosen from a fifty-seed distribution rather than from the declared five. Narrowing the range was preferred to amending `REQ-MOK-034`'s floor, so individuality does not cost habitability. Evidence and the option comparison: `evidence/WO-MOK-010/escalation.md` | technical owner |
| The default decision source | Unchanged. `reference` stays the default; whether `individual` should become it is deferred until its floor is measured | product owner |
| Approval of this chain | All eight artifacts transitioned `draft` to `approved`, and `ARCH-MOK-001` confirmed to need no amendment | product owner, technical owner, assurance owner |
| The `VREC-MOK-005` gate | Overridden. Implementation proceeds with the gate unmet, under the separability mitigation recorded above | repository owner |

`REQ-MOK-032` records the arithmetic behind the `fear` driver, and provision 3 of the `SPEC-MOK-001` list records the
reasoning behind the trait range, in both its original and its amended form. Both are stated as reasons rather than as
preferences so that a later reviewer can disagree with the argument rather than only with the number — which is what
happened to the range: the argument for the full range was disagreed with by measurement, and provision 3 records the
correction beside the original rather than in place of it.

### The prior gate: this work cannot begin while the last change is unsettled

**This is the precondition that matters most, and it has nothing to do with this work order's own content.**

`WO-MOK-005` is `in_progress`. `VREC-MOK-005` is `ready`, not `verified`. That record discloses six amendments across
`SPEC-MOK-002`, `SPEC-MOK-003` and `ARCH-MOK-001` marked **OUTSTANDING** — written by an implementation agent,
approved by nobody — and seven manual assessments not yet made. Two of the three specifications this work order
amends are among those artifacts.

Amending a specification whose last amendment is unapproved builds this change on text that carries no authority, and
it makes the two changes impossible to disentangle afterwards: a later reviewer could not tell which amendment any
given clause belongs to, nor which owner approved it. Phase 0 of the functional roadmap exists to prevent exactly
this.

The gate is therefore:

1. `VREC-MOK-005` reaches `verified`, or its scope is formally changed, and
2. the six amendments it lists are approved by the technical owner, and
3. its seven manual assessments are recorded.

Drafting and approving this work order's governing chain may proceed in parallel; **implementation may not begin
until the gate is met**, and beginning anyway is a stop condition rather than a judgement call.

#### The gate was overridden, and this records it rather than obscures it

**On 2026-08-19 the repository owner, having been shown this gate and the three conditions above, directed that
implementation proceed with the gate unmet.** None of the three conditions is satisfied at the time the work begins:
`VREC-MOK-005` is `ready`, its six amendments remain **OUTSTANDING**, and its seven manual assessments remain
unrecorded. The decision is the owner's to take and it is taken; what follows is a statement of what it costs and of
the one mitigation available.

The cost is the disentanglement concern above, and it is real: `SPEC-MOK-002` and `SPEC-MOK-003` now carry two layers
of amendment, the earlier unapproved and the later approved. The mitigation is that the two layers are separable by
inspection. Every amendment-record row this work order adds is dated 2026-08-19, names this work order, and records
the owner's approval; every row left by `WO-MOK-005` keeps its 2026-08-18 date and its **OUTSTANDING** marking. **No
row from the earlier layer is edited, reordered or removed**, and no clause the earlier layer amended is amended again
here except where this work order's amendment list already names it. A reviewer settling `VREC-MOK-005` later can
therefore still see exactly which text that record's amendments introduced.

What the override does not do: it does not approve the six outstanding amendments, does not verify `VREC-MOK-005`,
does not perform its seven manual assessments, and does not transition `WO-MOK-005`. Those remain open and are
unaffected by anything in this work order. `VREC-MOK-005` binds commit `9d9641fe`, which is an ancestor of `master`,
so the evidence it holds describes a tree this work builds on top of rather than one it invalidates.

### Required amendments to approved specifications

This work order depends additionally on amendments to three specifications that are already approved. They are
stated here in full, and each requires the technical owner's separate act: approving this work order does not approve
them. The implementation agent writes the amended text and records in each amendment record that it did so; it does
not decide the substance.

**`SPEC-MOK-001`** — the behavior authority. Nine provisions:

1. *Scope*, the clause reading "It does not define OpenAI integration, fear, individual traits, combat, social
   behavior, persistence, structured output, or a user interface." `fear` and `individual traits` leave that list.
2. *State model → Mokiterion*. The contents list gains the trait, stated as fixed for the run, and gains `fear` among
   the attributes. The sentence "All three attributes start at `100`" is restated so that health, satiety and energy
   start at `100` and `fear` starts at `0`.
3. *State model*. A new subsection fixes the trait: its name `waste_tolerance`, its integer range, and the
   exact derivation from the seed and the identifier, with the explicit statement that the derivation uses a generator
   of its own and consumes nothing from the shared stream. Integer arithmetic only. At tolerance `T` a high-class
   resource restoring `50` satiety is eaten up to satiety `50 + T/2`, so `T = 0` reproduces rule 5 exactly.

   **The range was specified as `0..=100` and amended to `0..=40` during this work order.** The full range was chosen
   deliberately, on the argument that a wider range strengthens `REQ-MOK-033`'s divergence evidence, and the subsection
   recorded that narrowing it was its own amendment to make if measurement showed the range cost survivors. Measurement
   showed exactly that and stop condition 6 fired; the owner chose narrowing over amending the floor. The measured
   evidence and the option comparison are in `evidence/WO-MOK-010/escalation.md`, and the amendment is the second
   2026-08-19 row of `SPEC-MOK-001`'s amendment record. Both the original reasoning and its correction are left standing
   here, because the argument that a wider range gives better evidence was sound and was defeated by measurement rather
   than by being wrong in principle.
4. *Time and entropy*. The sentence "One explicit SplitMix64 pseudo-random stream, seeded from `--seed`, supplies all
   initialization, decision-source, and regeneration entropy" is qualified: trait derivation is outside it. And "The
   two decision sources consume from it at different rates, so a run under one source is comparable only with runs
   under the same source" becomes three sources.
5. *Inputs* and *Help output*. The third `--policy` value, its effect, and the unchanged default.
6. *Data and interface contracts*. The field lists of the `agent_initialized`, `survival_changed` and `action_trace`
   records gain `fear`, and `agent_initialized` gains the trait.
7. Rule 1, *Initialization order*. Where trait derivation happens, and that initialization events are still emitted
   only after the complete initial state is valid.
8. Rule 3, *Observation*. The observation carries the acting Mokiterion's trait, so a source can read it without
   reaching into state.
9. Rule 7, *Optional action trace*, and rule 12, *Survival decay*. Rule 7's enumerated fields gain `fear`, with the
   value stated as the pre-update one, since rule 7 already places the trace before decay. Rule 12 gains the `fear`
   update at its own position: `+10` when the rule 3 observation lists at least one other living Mokiterion and `-5`
   when it lists none, saturating at both bounds, reading that observation and nothing else, consuming no entropy and
   performing no new perception.

   **The driver adds no distance constant.** It is the perception radius rule's own list, non-empty or empty. A
   narrower threshold was considered and rejected on arithmetic: with twelve Mokiterions on a 128×128 grid the
   expected number of others inside a Chebyshev box of radius `r` is `11 * (2r+1)^2 / 16384`, which is about `0.73` at
   the perception radius of `16`, `0.19` at `8` and `0.05` at `4`. A four-cell threshold would hold `fear` at `0` on
   roughly nineteen agent-ticks in twenty, making it verifiable only through constructed states — the inert attribute
   `SPEC-MOK-003` rule 4.5 refused, reached by a different route. Reusing the radius removes a constant instead of
   adding one, and `REQ-MOK-032` records the arithmetic.

And one addition rather than an amendment: the trait-aware decision source is specified as **rule 19**, appended
after rule 18. Rules 1 through 18 are cited by identifier across the specifications, the verification contracts, the
verification records, the retained evidence and the source comments, so inserting a rule beside rule 5 would
renumber thirteen rules and invalidate every one of those citations. The cost of appending is that rule order stops
matching tick order for one rule, and the *Behavioral rules* preamble must say so. **The repository owner, acting as
technical owner, chose appending over renumbering on 2026-08-19**, on the ground that thirteen shifted rule numbers
would invalidate citations the corpus cannot cheaply re-verify. Rule 19 must fix the four cases, the tolerant test in
place of the non-waste test — `satiety + restored - 100 <= waste_tolerance * restored / 100`, integer arithmetic — that
the test governs eating *and* seeking, that only the search case consumes entropy, that the source never proposes
`wait`, that the source is named `individual` as the third `--policy` value, and that at `waste_tolerance = 0` the
source is proposal-identical to rule 5.

Rule 5 itself is not otherwise touched. Its paragraph recording high-class accumulation gains a cross-reference to
`REQ-MOK-034` and `VER-MOK-010`, which measure the effect under the new source; the reference source's own behavior
is unchanged, and changing it is out of scope under `INT-MOK-006`.

**`SPEC-MOK-002`** — the engine's public interface. Two provisions, both in rule 5's enumeration:

1. `simulation::Policy`, listed as an "enum with variants `Baseline` and `Reference`, with `parse` and `Default`",
   gains a third variant. `Default` is unchanged.
2. `simulation::AgentSnapshot`, listed as a "struct of `String`, `Coordinate`, `Territory`, three `u8` attributes and
   `Option<Action>`", carries four `u8` attributes. Its justification column — "Every field is already printed in the
   event stream" — continues to hold, because `REQ-MOK-032` requires `fear` in the event stream too.

Rule 6 is not amended and is re-checked instead: the added field carries a value, so no public item yields a mutable
borrow of or a reference into authoritative state, and the trait-aware source and the `Observation` it consumes stay
private, keeping the `ADR-MOK-001` trust boundary where it is. The trait deliberately does *not* join the snapshot,
because no approved requirement needs the observer to render it and rule 5 holds the interface to what approved
requirements need; it reaches the observer through the event log instead, which `REQ-MOK-022` already retains.

This specification is bound by a `verified` `VREC-MOK-003` and carries amendments outstanding under `VREC-MOK-005`.
The bound records are not edited; only the specification is amended, and its amendment record must show that.

**`SPEC-MOK-003`** — the observer. Three provisions, all in rule 4:

1. Rule 4's two-line mockup at lines 226–229 shows three gauges and must show four.
2. Rule 4's prose — "Line two carries health, satiety and energy, each as a twenty-cell proportional bar and a
   numeric value. Below 47 columns each entry collapses to one line carrying identifier, territory and the three
   numeric values without bars" — becomes four attributes and four numeric values.
3. Rule 4 item 5, the reservation. Its text — that the row "reserves trailing space for a fourth bar so that Phase
   2's `fear` occupies it", rendering "empty with no label, no dash and no zero", because "an inert `fear 0` would be
   a claim the engine cannot support" — is replaced by the presentation of a computed `fear`. Item 4 then governs the
   zero case: `fear 0` renders as `0` with an empty bar, and it is a computed zero rather than an inert one, which is
   precisely the condition item 5 was waiting for. **The reason for the reservation should be retained in the
   amendment record rather than deleted**, because it is the argument that made the empty slot correct for a phase.

   This provision carries a measured consequence, and the amendment must state it rather than leave it to be
   discovered. The width rule `WO-MOK-005` settled is `bar_width(interior) = min(20, (interior − 27) / 3)`, where 27 is
   five leading columns plus three groups of label, space, bar, space and a three-column value, separated by two
   columns. A fourth group makes that overhead 35 and the divisor 4. At the reference roster's 45-column interior the
   bars therefore narrow from `(45 − 27) / 3 = 6` cells to `(45 − 35) / 4 = 2`, while the three-column numeric values
   are unaffected. **The repository owner, acting as technical owner, accepted the narrowing on 2026-08-19** rather
   than widening the roster pane in rule 5, which would have taken fourteen columns from the map pane, or raising rule
   4's two-line threshold, which would have cost bars entirely to operators between 47 and 60 columns. The amendment
   records the accepted figure so that a later reader finds a stated consequence rather than an unexplained regression
   in bar length.

This specification also carries amendments outstanding under `VREC-MOK-005`, including the one this narrowing
supersedes.

## Objective

Make each Mokiterion an individual: derive one behavioral trait per Mokiterion from the seed and the identifier
without touching the shared entropy stream, maintain `fear` as a fourth bounded attribute driven by perceived
proximity, add a third decision source that reads the trait, present the fourth bar in the roster slot reserved for
it, and demonstrate that the two existing decision sources produce byte-identical projected output to the build this
work began from.

## In scope

- Trait derivation at initialization for all twelve Mokiterions, under every decision source and every density, using
  a generator seeded from `--seed` and the identifier and drawing nothing from the shared stream.
- The trait on the observation the engine hands a decision source.
- `fear` as a fourth attribute: initial value, the update at rule 12's position from the rule 3 observation,
  saturation at both bounds, and no consumer.
- `fear` in the `agent_initialized`, `survival_changed` and `action_trace` records, and the trait in
  `agent_initialized`.
- `fear` on the observation snapshot's Mokiterion entry.
- The third decision source: the four cases of rule 5 with the tolerant test in place of the non-waste test, applied
  to eating and to seeking alike.
- The third `--policy` value: parsing, display, the usage text, the selected-source record, and the third arm of every
  exhaustive match over the policy enumeration.
- The observer's fourth gauge, and the bar-width arithmetic that makes room for it, closing `VREC-MOK-005` finding 3.
- The nine `SPEC-MOK-001`, two `SPEC-MOK-002` and three `SPEC-MOK-003` provisions above, plus rule 19, written as
  amendments the technical owner has approved.
- Every check, measurement and manual assessment `VER-MOK-010` requires, and the evidence it retains.
- Updating `SIMULATION_RULES.md`, which describes the implemented behavior in plain language and states that
  `SPEC-MOK-001` governs where the two disagree.
- Updating the functional roadmap's Phase 2 entry, whose maintenance rule triggers when a phase is converted into an
  approved artifact chain, including recording the two scope items `INT-MOK-006` declines.

## Out of scope

- Any change to the behavior of `--policy baseline` or `--policy reference`. This is the constraint the whole change
  is measured against.
- Any consumer of `fear`, in any rule.
- Any second trait.
- Per-agent entropy substreams.
- Any change to the default decision source, the default density, the world constants, the survival values, the
  resource table, the density mapping, the perception radius, regeneration, the finality of death, the exit-code
  contract, or `REQ-MOK-014`'s floor.
- Any correction of high-class accumulation under the reference source.
- Combat, threat response, retreat, surrender, social behavior, encounter memory, model-backed decisions, structured
  output, persistence, and a batch runner.
- Any new package, target, build script or external dependency; any growth of the engine's dependency table, which
  stays empty.
- Any growth of the engine's public interface beyond the one snapshot field and the one enumeration variant above.
- Resolving `VREC-MOK-005`. That is the gate, and it belongs to `WO-MOK-005`.
- Any edit to a record bound by a `verified` verification record.

## Authorized decision envelope

The implementation agent may decide locally:

- Rust file organization, private type and function names, and where a private helper lives.
- Which test tier each new test belongs to, applying `SPEC-MOK-004`'s rule as written: the public tier only if the
  test is writable through the library target's public interface with its assertions unchanged and no item widened;
  otherwise inline beside the code. **Widening an item to `pub` in order to relocate a test is prohibited**, and a
  test that seems to need it belongs inline.
- The internal organization of a test module, and test helper functions.
- The exact wording of the amended text, once the technical owner has approved the substance, and the wording of the
  plain-language and roadmap updates.
- The form of the evidence files and the projection script, provided the projection is retained in full and reviewed.
- Column alignment and cosmetic whitespace in the amended usage text, within what the *Help output* section fixes.

The implementation agent may **not** decide:

- Any constant. The trait's name and range, the derivation, the `fear` driver, the increment, the decrement and the
  tolerance comparison are all `SPEC-MOK-001` matters, fixed in the amendment list above. A constant that does not
  produce the required outcome is an escalation, not an adjustment.
- The trait range, the tolerance test, the `fear` step sizes and its driver, the rule-19 placement, and the roster
  narrowing. All six were decided by the repository owner on 2026-08-19 and are recorded above; the agent writes them
  into the amended text and does not revisit them.
- The survivor floor. It is `REQ-MOK-034`'s and the product owner's.
- Whether the new source becomes the default. It does not, under this work order.
- Any relaxation of an assertion, any `allow` attribute, or any test marked ignored, to make a check pass.

## Constraints

- **The two existing decision sources produce byte-identical projected output to the pre-change build**, across the
  matrix `VER-MOK-010` declares. The projection removes only the fields this change adds. Any other difference is a
  defect in this change and is corrected rather than rebaselined.
- The pre-change baseline is captured **before the first line of code changes**, at the commit the work begins from,
  and is never recaptured to resolve a discrepancy.
- Trait derivation draws nothing from the shared entropy stream, and this is asserted directly against the stream's
  state, not only inferred from output.
- The `fear` update performs no new perception, examines nothing the rule 3 observation does not already carry, and
  consumes no entropy.
- Integer arithmetic throughout. No floating-point value in the derivation, the range mapping, the tolerance
  comparison or the `fear` update.
- The tolerant test governs both eating and seeking. Applying it to eating alone reintroduces the two-cell
  oscillation `SPEC-MOK-001` rule 5 records as its second defect, which was found by measurement after the
  specification had asserted the opposite.
- Nothing in the engine reads `fear`.
- The engine's dependency and dev-dependency tables stay empty, with no exception including a dependency shared with
  the observer.
- `SPEC-MOK-002` rule 6 holds: values only, no mutable borrow of and no reference into authoritative state, in any
  build configuration including test builds.
- Every observer rendering claim is asserted against an in-memory character buffer. No screenshot, no recording, no
  terminal, no pseudo-terminal.
- `cargo fmt --all --check` and `cargo clippy --all-targets --all-features -- -D warnings` clean, with no lint
  suppressed and no `allow` added.
- `cargo test` at the workspace root runs every tier of both packages in one invocation.
- Work proceeds on a `feature/` branch off `master`, lowercase and hyphenated, per `REPOSITORY_CONTEXT.md`.
- No secret enters the repository, in code, in a fixture, or in retained evidence.
- Where the specification and the implementation disagree, the specification is amended and re-approved. A quietly
  adjusted constraint or a relaxed assertion is prohibited, and `WO-MOK-005`'s completion record shows why: three of
  its findings were mismatches found only by measurement.

## Expected change surface

Stated as components. Paths are the current ones and are not a licence to change anything not named.

**Engine, `mokiterions-core/`.**

- The simulation module's Mokiterion state: two new fields, one derived at initialization and immutable, one updated
  per tick.
- Initialization: trait derivation for all twelve, positioned so the shared stream's draw sequence is untouched.
- The observation the engine builds per decision opportunity: the acting Mokiterion's trait.
- The per-agent tick path at rule 12's position: the `fear` update, before death evaluation.
- The event vocabulary and its rendering: three record kinds gain a field, one of them gains two.
- The decision-source layer: a third source. The existing two are not edited.
- The policy enumeration, its parser, its display, and every exhaustive match over it, including the one that names
  the source in the initialization events.
- The command-line layer's usage text.
- The observation snapshot's Mokiterion entry: one field.
- The module-level documentation comment, which states the public interface's shape and cites `SPEC-MOK-002` rules 5
  and 6.

**Observer, `mokiterions-tui/`.**

- The roster's bar row: a fourth gauge.
- The bar-width function and the bar-row overhead constant: `render.rs:495-498` and the constant at `render.rs:53`,
  whose doc comment states the three-group derivation and must state four.
- The comment in the roster entry builder that currently cites the inert-`fear` reasoning as the reason for the empty
  slot.
- The one-line collapsed form below 47 columns: a fourth numeric value.

**Governance and documentation.**

- `SPEC-MOK-001`, `SPEC-MOK-002`, `SPEC-MOK-003`: the amendments above, each with its amendment-record row.
- `SIMULATION_RULES.md` and `docs/mokiterions/ROADMAP.md`.
- `docs/engineering/simulation/evidence/WO-MOK-010/`.

**Tests.** New cases in both tiers of both packages, placed by `SPEC-MOK-004`'s rule. No existing test is deleted,
renamed without record, or made ignored; a test whose expected output changes because a record gained a field is
updated and the update is enumerated in the completion report.

## Required verification

`VER-MOK-010` in full. Nothing in it may be waived by this work order, and in particular:

- the projected byte comparison against the pre-change baseline, over the whole declared matrix, under both existing
  sources;
- the direct assertion that trait derivation leaves the shared stream's state unmoved;
- the enumerated lower-bound equivalence between the new source and the reference source;
- the `fear` driver at perception's own boundary, at Chebyshev distance `16` and at `17`, and its insensitivity to
  count, distance and direction within the radius;
- `fear` saturation at both bounds, and the range and step invariants over full runs;
- the survivor floor of eight under the new source at the default density on all five declared seeds;
- the cell-position assertions for four gauges at every declared viewport;
- the static gates, the empty engine dependency table, and `cargo tree -p Mokiterions` resolving to one crate;
- the seven manual assessments, each recorded by its accountable role;
- the check that every amendment this change requires is approved, and that none left outstanding by `VREC-MOK-005`
  remains outstanding.

`VER-MOK-001` through `VER-MOK-006` remain in force. Every case, invariant and check in them still passes.

## Evidence to record

Under `docs/engineering/simulation/evidence/WO-MOK-010/`, as `VER-MOK-010`'s *Evidence retention* section
enumerates: the pre-change baseline and the commit it was taken at; the post-change capture and every projected
comparison; the projection's full text and its no-op result on the pre-change stream; the twelve trait values per
declared seed; per-seed survivor counts and consumption totals under the new source; per-seed final resource counts
and class distributions; the tick-10,000 result; the oscillation rate per seed against 10.6% and 12.2%; the recorded
divergence instances with ticks, Mokiterions, traits and proposals; rendered roster buffers per declared viewport
with cell positions; the enumerated situation set and its size; the test census reconciled name by name; the
`fmt`, `clippy`, `test` and `tree` output; and the seven manual assessments with role and date.

Add a completion summary in the form `WO-MOK-005` and `WO-MOK-006` established, and record every mismatch between
specification and implementation found during the work, whether or not it was corrected.

## Stop and escalate conditions

Stop and escalate, rather than deciding locally, when:

1. **The prior gate is not met.** `VREC-MOK-005` is not `verified`, or any of its six amendments is still outstanding,
   or any of its seven assessments is unrecorded. Do not begin.
2. **Any governing artifact of this work order is not approved**, or preflight does not report this exact work order
   as eligible.
3. **A required amendment is not approved by the technical owner.** The substance of each is recorded above, including
   the six decisions of 2026-08-19, but recording a decision is not approving an amendment, and the amendments are what
   this work order builds on.
4. **The projected comparison shows any difference under `--policy baseline` or `--policy reference`.** This is the
   central failure mode. Do not adjust the baseline, do not widen the projection, and do not explain the difference as
   benign. Report it with the differing lines.
5. **The shared stream's state moves across trait derivation**, at any seed or density.
6. **The survivor floor of eight is missed on any declared seed** under the new source at the default density. The
   correction is either an amended `SPEC-MOK-001` constant approved by the technical owner or an amended floor
   approved by the product owner on the measured evidence. It is never an implementation adjustment, and it is never
   a change to the reference source.
7. **The lower-bound equivalence fails on any enumerated situation.** The design's whole claim to being additive
   rests on it.
8. **The tolerant test cannot be applied to seeking without reintroducing oscillation**, or the measured oscillation
   rate under the new source exceeds the 12.2% unbiased-walk rate.
9. **No declared viewport can render four gauges of at least one cell each**, or the narrowing turns out to be worse
   than the measured `(45 − 35) / 4 = 2`.
10. **A required constant produces an outcome the requirement forbids.** Constants are specified; escalate.
11. **A test appears to need an item widened to `pub`**, or a check appears to need an `allow`, a relaxed assertion or
    an ignore.
12. **`ARCH-MOK-001` turns out to require amendment**, which would mean this work order needs an `architecture`
    relation and a deciding ADR before approval.
13. **Any specification clause is found to contradict the implementation**, including a clause this work order does
    not name. Amend and re-approve; never adjust quietly.
14. **The scope would grow** — a second trait, a consumer for `fear`, a change to an existing source, a new
    dependency, or a public interface item beyond the two named.

## Completion report format

1. **Scope statement.** What was implemented, and confirmation that nothing outside *In scope* changed.
2. **The gate.** Evidence that `VREC-MOK-005` was `verified`, its six amendments approved and its seven assessments
   recorded, before implementation began, with dates.
3. **Approval record.** Each governing artifact, its approving role and date; each required amendment, its approving
   role and date, and whether the agent wrote the text or decided it; the technical owner's rule-19 and roster-width
   decisions; the technical owner's `ARCH-MOK-001` confirmation.
4. **The additivity result.** The pre-change baseline commit, the matrix covered, the projection, and the comparison
   outcome for every combination. Any difference, in full.
5. **Entropy neutrality.** The direct assertion's result, and confirmation the check was demonstrated to be capable
   of failing.
6. **Measured results.** Per-seed survivors under the new source; the trait values behind them; consumption totals;
   resource counts and class distributions at tick 1,000; the tick-10,000 result against the reference source's
   extinction at tick 9,154; the oscillation rate against 10.6% and 12.2%.
7. **Divergence.** The recorded instances, and the lower-bound equivalence result with the situation set's size.
8. **`fear`.** Band, increment and decrement as approved; the range and step invariant results; confirmation that the
   engine contains one writer and no reader.
9. **The observer.** The four gauges' cell positions per declared viewport, the measured bar width per viewport, and
   the statement that `VREC-MOK-005` finding 3 is closed by assertion.
10. **Public interface.** The two added items, and confirmation that nothing else was added and rule 6 still holds.
11. **Static gates.** `fmt`, `clippy`, `test` with the census reconciled name by name, `tree -p Mokiterions`, and the
    engine's empty dependency table.
12. **Test placement.** Every new test, its tier, and for each internal-tier test the private item or hook it
    requires. Every test updated because a record gained a field, enumerated.
13. **Specification mismatches found.** Each one, whether corrected, and how — in the form `WO-MOK-005`'s completion
    summary used, since that is the section a later reviewer reads first.
14. **Manual assessments.** All seven, each with its judgement, accountable role and date. An unrecorded assessment is
    reported as outstanding rather than omitted.
15. **Evidence index.** Every retained file and what it establishes.
16. **Residual disclosure.** Everything a verifier should weigh that the checks above do not settle, including
    `VER-MOK-010`'s stated residual uncertainty and anything found during the work.
