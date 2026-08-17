+++
id = "WO-MOK-002"
type = "work_order"
title = "Implement perception, resource rebalance, and the reference decision source"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-17"
updated = "2026-08-17"

[assurance]
commit_bound_verification = "required"
rationale = "This work changes authoritative executable behavior, including the observation contract, the survival and resource constants, and the set of available decision sources. Every later phase depends on the correctness of that behavior, and the population viability floor becomes a measured claim that assurance and product decisions will rely on."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-013", "REQ-MOK-014", "REQ-MOK-015"]
specifications = ["SPEC-MOK-001"]
verification = ["VER-MOK-002"]
+++

# Work Order: Implement perception, resource rebalance, and the reference decision source

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the
scope below. Transition to `in_progress` records that implementation has begun. Transition to `implemented`
requires the completed change and retained evidence. Verification and release require separate commit-bound
records.

Commit-bound verification is classified `required` above. Architecture is deliberately not selected: no active
architecture `addresses` any requirement implemented here. `ARCH-MOK-001` addresses `REQ-MOK-004`,
`REQ-MOK-008`, `REQ-MOK-009`, and `REQ-MOK-010`, none of which are in this scope. This work enriches the payload
of an interface that architecture already governs, without altering the system boundary, dependency direction,
or trust model, and it adds a decision source of exactly the kind `ADR-MOK-001` already anticipates. Nominal
architecture coverage is therefore omitted rather than fabricated. The technical owner confirms this omission at
approval.

## Objective

Make the Mokiterions world habitable and perceptible by implementing bounded perception, rebalancing the
specified resource economy to meet the population viability floor, and adding a selectable deterministic
reference decision source, exactly as amended in `SPEC-MOK-001` and covered by `VER-MOK-002`.

## In scope

- Extend the observation with perceived resources and perceived living Mokiterions within the specified radius,
  each with relative direction and distance, in the specified stable order.
- Apply the amended survival and regeneration constants from `SPEC-MOK-001`.
- Implement the reference decision source and its specified ranking and tie-breaking rules.
- Add the specified `--policy` input and make the reference source the default selection.
- Identify the active decision source in simulation output.
- Add the specified `--density` input, replacing the fixed initial endowment and fixed territory capacity, and
  bind it to initialization, capacity, and the replenishment target as `SPEC-MOK-001` requires.
- Correct reference-source case 1 from a fixed satiety threshold to the specified non-waste rule.
- Extend the same non-waste test to reference-source case 3, so a Mokiterion neither eats nor approaches a
  resource whose satiety restoration would be clipped, and remove the unreachable fallback clause from case 1, as
  amended in `SPEC-MOK-001`.
- Update existing automated tests that assert superseded constants or the previous observation contract.
- Add automated tests covering every requirement, case, and invariant in `VER-MOK-002`, including the case that a
  Mokiterion perceiving only clipping resources searches instead of approaching one.
- Record the per-seed calibration measurement at the declared density, sweep the density curve as evidence, and
  retain both under this work-order ID.

## Out of scope

- OpenAI or other model-provider integration, prompts, agent memory, or credentials.
- Fear, traits, per-agent individuality, combat, threats, retreat, surrender, or social behavior.
- Structured or machine-readable output, persistence, aggregate multi-run analysis, and outcome classification.
- New actions, including `explore`. Navigation is movement plus perception.
- Changes to health, satiety, and energy bounds, to decay-driven health loss, to the finality of death, or to the
  conditional-regeneration rule.
- Graphical or web interfaces, networking, async runtimes, databases, or services.
- Changes to approved behavior or artifact lifecycle outside this work order.

## Authorized decision envelope

The implementation agent may choose:

- private Rust type, function, module, and file names, including whether perception and the reference source are
  separate modules;
- standard-library collection types that preserve the specified observable ordering;
- how perceived entities are gathered, provided the specified order and metric are honoured;
- internal error types and exact diagnostic wording;
- test module organization, fixtures, and helper functions;
- comments and non-authoritative developer documentation.

This work order **explicitly authorizes updating existing tests** that assert constants or observation contents
superseded by the amended `SPEC-MOK-001`. `WO-MOK-001` prohibited changing specified constants; that prohibition
is lifted only for the constants this amendment changes, and only to match the amended specification.

The implementation agent may not change the amended constants themselves, the perception radius or metric, the
density-to-resource mapping or its rounding rule, the declared densities or their stated floors, event field
names or order, exit codes, trust boundaries, requirement scope, or lifecycle status. No external dependency may
be added without escalation and an updated architectural decision.

The default density and its declared floor are fixed by the amended `REQ-MOK-014`. Selecting a different density
to make a measurement pass is the unauthorized retune this work order forbids, and it is forbidden whether
achieved by changing the default or by changing which density is declared.

## Constraints

- Follow `INT-MOK-002`, `SPEC-MOK-001` as amended, and `VER-MOK-002`, and preserve everything verified under
  `VREC-MOK-001` that this amendment does not change.
- Keep one Rust binary crate and prefer the standard library. Dependencies remain empty.
- Use no network, model, credential, UI, persistence, async-runtime, or database dependency.
- Keep every decision source unable to mutate authoritative state and unable to act at a distance.
- Perception must consume no entropy, and every stochastic result must continue to derive from the specified
  stream.
- Keep the reference source small and readable. Do not add strategy such as cooperation, avoidance, territorial
  preference, or threat response.
- Preserve unrelated user changes and do not modify generated output as source.

## Expected change surface

- The simulation module's observation construction, survival and regeneration constants, and decision-source
  selection.
- A small reference decision source component alongside the existing baseline source.
- The command-line input component, for `--policy` and `--density`.
- Existing and new automated tests within the Rust crate.
- Work-order-keyed evidence under `docs/engineering/simulation/evidence/WO-MOK-002/`.
- `Cargo.toml` and `Cargo.lock` are not expected to change.
- No other product domain and no harness-managed policy file.

## Required verification

- Complete every case, invariant, check, and manual assessment in `VER-MOK-002`.
- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --all-targets --all-features -- -D warnings`.
- Run `cargo test`.
- Run `cargo build`.
- Run 1,000-tick default runs on seeds `0`, `1`, `42`, `123`, and `777` under the reference source, and record
  survivors and consumption for each.
- Sweep the density curve across the same seeds as evidence, including densities that carry no obligation, and
  measure the two-cell oscillation rate against a random-walk baseline.
- Demonstrate byte-identical replay for both decision sources.
- Run one 10,000-tick resilience run under each source.
- Capture one 20-tick traced excerpt for manual observation.

## Evidence to record

Retain, under `docs/engineering/simulation/evidence/WO-MOK-002/`, every item listed in `VER-MOK-002`'s evidence
retention section. The per-seed survivor and consumption table is the calibration record and must be retained
even when the floor is met comfortably, because later phases depend on knowing the measured carrying capacity
rather than the analytical estimate.

## Stop and escalate conditions

Stop before implementation if preflight does not report this exact work order as eligible or if any governing
artifact remains incomplete or unapproved. During implementation, stop and escalate if:

- measured survivors fall below eight on any declared seed. The constants are specified, so correction requires
  an amended specification and re-approval, never an unauthorized retune;
- all twelve Mokiterions survive on every declared seed, which meets the floor but contradicts
  `INT-MOK-002`'s scarcity principle and requires product review;
- perception cannot be added without exposing mutable state or consuming entropy;
- deterministic byte-identical output cannot be preserved for either source;
- the reference source cannot reach the floor without accumulating strategy beyond food seeking;
- an external dependency appears necessary;
- specified behavior is contradictory or needs a product decision;
- required verification fails and cannot be corrected within authorized scope;
- requested changes expand into an excluded feature or another artifact domain.

## Escalation record

On 2026-08-17 the first stop condition above fired. Measured survivors at 1,000 ticks under the reference source
were **zero on every declared seed**, against a required floor of eight. This work order therefore stays
`in_progress` and is not transitioned to `implemented`.

`REQ-MOK-013` and `REQ-MOK-015` are implemented and fully verified. `REQ-MOK-014` is implemented exactly as
specified and fails.

The cause is not the one the approved packet assumed. Raising regeneration yield from `2` to `8` saves nobody,
because `FOOD_CAPACITY_PER_TERRITORY` is the real ceiling on standing supply and territories already sit at it;
raising perception radius to `96` yields at most two survivors. The binding constraint is the ratio of travel
time to satiety drain during the regeneration ramp. Full measurements are in
`docs/engineering/simulation/evidence/WO-MOK-002/lever-sensitivity-study.md`, and the options requiring an
amended `SPEC-MOK-001` and re-approval are in the same directory's `escalation.md`.

No constant was retuned in the committed tree, every measured variant was reverted, and the failing viability
test was left failing rather than relaxed or ignored.

### Resolution, 2026-08-17

The repository owner resolved the escalation by restating the obligation instead of retuning a constant. Resource
density became an explicit simulation input, and `REQ-MOK-014` became a conditional obligation tabulated over
declared densities: a floor of five at the default `0.75%`, and a floor of eight at `1.50%`. Both floors were
chosen from the measured density curve rather than estimated.

This closes the escalation and re-authorizes this work order with the scope amended above. `SPEC-MOK-001`,
`REQ-MOK-014`, and `VER-MOK-002` were amended together and re-approved on the same date. Two defects found during
the failed attempt are corrected as part of the re-authorized scope: territory capacity rather than regeneration
yield governs standing supply, which is why density now binds capacity directly; and reference-source case 1 is
now the non-waste rule `REQ-MOK-015` always required rather than a fixed satiety threshold.

### Second escalation, 2026-08-17

The re-authorized scope is implemented in full and the first stop condition fired again. Measured worst case is
**three survivors at the default `0.75%` against a stated floor of five**, and **seven at `1.50%` against a
stated floor of eight**. This work order therefore stays `in_progress`.

The cause is not a code defect. The implementation matches amended `SPEC-MOK-001` rule 5 exactly. The floors were
selected from a density curve measured before the rule 5 correction that the same amendment mandates, so the
approved floors do not describe the specified system. This is an implementation-agent measurement error and is
recorded as such in `docs/engineering/simulation/evidence/WO-MOK-002/escalation.md`.

A specification defect was found alongside it. `SPEC-MOK-001` §5 claims the non-waste correction removes the
two-cell oscillation. For High-class resources the non-waste condition evaluates to `satiety <= 50`, numerically
identical to the threshold it replaced, so the claim is false for a third of the resource table and the
oscillation persists at 35.7% of agent-ticks. Its root cause is that case 3 re-targets a resource the Mokiterion
is standing on the instant it steps off, which correcting case 1 could never have addressed.

Both matters require owner decision. Options are in the same `escalation.md`. No constant, threshold, density, or
floor was changed in the committed tree, every measured variant was reverted, and the failing viability test was
left failing rather than relaxed.

### Resolution of the second escalation, 2026-08-17

The repository owner chose to fix the specification defect before setting floors, so that the floors would be
chosen once from a curve measured on the corrected system rather than twice.

`SPEC-MOK-001` was amended a third time: the rule 5 non-waste test now governs case 3 as well as case 1, so a
Mokiterion neither eats nor approaches a resource whose restoration would be clipped; the unreachable fallback
clause was removed from case 1; and the false claim about the oscillation was retracted in place. `REQ-MOK-014`
was amended to declare one density, the default `0.75%`, with a floor of **eight of twelve** — the original
product intent, now stated at the scarce default. The `1.50%` row was withdrawn because it retains all twelve on a
majority of declared seeds, which its own verification would report as an adverse observation; it remains a swept
comparison point with no obligation.

The owner also accepted a disclosed consequence of the corrected rule. High-class resources become consumable only
at satiety of at most `50`, so they accumulate against capacity, and a 10,000-tick run at the default density now
reaches extinction where the previous rule left one survivor. No requirement speaks past tick 1,000, and at that
horizon the correction raises the measured worst case from three survivors to eight. The effect is recorded as
residual uncertainty in `VER-MOK-002` and deferred to Phase 2 in the functional roadmap.

This closes the second escalation and re-authorizes this work order with the scope amended above. `SPEC-MOK-001`,
`REQ-MOK-014`, and `VER-MOK-002` were amended together and re-approved on the same date.

The re-authorized scope is implemented and every gate passes. Measured survivors at the declared density are 8,
11, 8, 9, and 11 on the five declared seeds, meeting the floor of eight on every seed. All 52 automated tests
pass, including the viability test that was deliberately left failing. `cargo fmt`, `cargo clippy -D warnings`,
and `cargo build` are clean, and dependencies remain empty. This work order is therefore transitioned to
`implemented`. Verification and release remain separate commit-bound records and separate owner decisions.

## Completion report format

Report:

1. implemented requirements and affected components;
2. any authorized local decisions;
3. verification commands and results;
4. the per-seed survivor and consumption table, and the measured carrying capacity it implies;
5. retained evidence paths;
6. residual limitations and explicitly deferred features;
7. final worktree and candidate-commit status.
