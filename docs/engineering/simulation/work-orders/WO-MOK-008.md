+++
id = "WO-MOK-008"
type = "work_order"
title = "Make the provenance footer shed fields without losing authoritative information"
status = "in_progress"
owners = ["engineering owner"]
created = "2026-08-19"
updated = "2026-08-22"

[assurance]
commit_bound_verification = "required"
rationale = "This work changes what the observer displays at the narrowest declared viewport, which is authoritative information an operator reads to know which run they are looking at. Assurance decisions rely on it: VER-MOK-005 measures REQ-MOK-024 and REQ-MOK-027 through the footer, and a release cannot stamp its own commit into the binary until this is settled, so release decisions depend on the outcome too. The change also reaches an approved specification's rule text rather than only the renderer."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-024", "REQ-MOK-027"]
specifications = ["SPEC-MOK-003"]
verification = ["VER-MOK-005"]
+++

# Work Order: Make the provenance footer shed fields without losing authoritative information

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the completed change and retained evidence. Verification and release require separate commit-bound records.

### Approval preconditions

Approval requires the technical owner to decide the `SPEC-MOK-003` rule 8 question first, because the correct behavior is not currently specified. Two things follow from that and neither is an implementation choice:

1. **`SPEC-MOK-003` rule 8 must be amended** to say what the footer does when no specified tier fits. Today the code answers that with an unspecified fifth string; rule 8 enumerates six fields and does not rank them. That amendment requires the technical owner, and `SPEC-MOK-003` already carries an **OUTSTANDING** amendment row from 2026-08-18.
2. **`VER-MOK-005`'s case list must be extended** by the assurance owner. It verifies `REQ-MOK-024` and `REQ-MOK-027` and did not catch this, so its cases are part of what changes.

The relation to architecture is deliberately absent. `ARCH-MOK-002` addresses `REQ-MOK-021`, `REQ-MOK-025`, `REQ-MOK-026` and `REQ-MOK-028`, none of which this work order implements, and no other active architecture addresses `REQ-MOK-024` or `REQ-MOK-027`.

## Objective

Make the provenance footer either state every field `SPEC-MOK-003` rule 8 requires or degrade in a specified order, at every viewport the observer admits and for every input the engine accepts — and make the observer's own tests capable of detecting a failure to do so.

The reported symptoms are recorded in `docs/engineering/simulation/evidence/WO-MOK-008/footer-tier-fallthrough.md`: an operator-chosen large seed reaches an unspecified rendering that omits resource density and the active decision source, and no value of `MOKITERIONS_COMMIT` can be compiled in without the same class of loss.

## In scope

- Implement the rule 8 behavior the technical owner decides, in `mokiterions-tui/src/render.rs`.
- Remove the unspecified fall-through at `render.rs:728-736`, or make it a specified tier.
- Cover the narrowest declared viewport for seeds across the accepted `u64` range, not only small ones.
- Cover the stamped footer forms, so `COMMIT` being `Some(..)` is exercised rather than only `None`.
- Distinguish an unset `MOKITERIONS_COMMIT` from one set to the empty string, which `option_env!` reports as `Some("")`.
- Extend `VER-MOK-005`'s cases to match, once the assurance owner has decided them.
- Retain evidence under this work-order ID.

## Out of scope

- Any change to the engine package. The defect is entirely in the observer's presentation layer, and `ARCH-MOK-001` as amended keeps the engine's dependency table empty.
- Any change to what the footer means: the six rule 8 fields, their values, or where they come from.
- The prohibitions in rule 8.3 — no wall-clock time, no absolute path, no environment value, no credential — which stay exactly as they are.
- Widening the observer's public interface. `SPEC-MOK-004` rule 7 prohibits it and rule 6 counts it.
- Adding a build script to either package, or any new dependency.
- Deciding whether release CI stamps a commit at all. That is a separate question and this work order only removes the obstacle.

## Authorized decision envelope

The implementation agent may choose:

- private function and helper names, and how the tier ladder is expressed in code;
- test names, fixtures, and how seeds and widths are enumerated in tests;
- comments and non-authoritative developer documentation.

The implementation agent may not decide which fields survive a narrowing, invent a tier the specification does not state, change any field's value or source, relax rule 8.3, widen the public interface, or transition any artifact's status.

## Constraints

- Follow `SPEC-MOK-003` rule 8 as amended, `REQ-MOK-024`, `REQ-MOK-027` and `VER-MOK-005`.
- Change no observable simulation behavior. `REQ-MOK-025` requires the observed run to equal the unobserved run, and the cross-cutting suite measures it on every declared seed.
- Keep the engine package's dependency table empty and add no build script.
- Preserve unrelated changes in the worktree.

## Expected change surface

- The footer rendering and its tier selection in the observer's presentation component.
- The observer's own tests, in both tiers: the in-crate module and the cross-crate suite.
- `SPEC-MOK-003` rule 8 and its amendment record, written by the implementation agent only after the technical owner has decided the content.
- `VER-MOK-005`'s case list, written only after the assurance owner has decided it.
- Work-order-keyed evidence under `docs/engineering/simulation/evidence/WO-MOK-008/`.
- No engine source, no manifest, no harness-managed file.

## Required verification

- Complete every case and review in `VER-MOK-005` as extended.
- Render the footer at every declared viewport for seeds at both ends of the accepted range, including `u64::MAX`, and show that every field rule 8 requires is present or that the shedding order is the specified one.
- Run the observer's tests with `MOKITERIONS_COMMIT` unset, set to the empty string, and set to a full 40-character SHA-1, and show the result in each case. Force a recompile between values: neither package has a build script, so Cargo does not treat the variable as a rebuild input and a stale artifact will report a pass that means nothing.
- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace`.
- Run `cargo tree -p Mokiterions` and show it resolves to one crate.
- Show two identical seeded engine runs, byte for byte, to evidence that nothing in the engine moved.

## Evidence to record

Retain, under `docs/engineering/simulation/evidence/WO-MOK-008/`:

- the defect report already present in that directory, unchanged;
- formatter, linter, test and dependency-tree output;
- the rendered footer at each declared viewport for the seeds exercised, before and after;
- the three `MOKITERIONS_COMMIT` states and their results, with the recompile shown;
- the mapping from `VER-MOK-005`'s extended cases to the tests that discharge them;
- the deterministic replay comparison;
- a completion summary identifying the final affected components.

## Stop and escalate conditions

Stop before implementation if preflight does not report this exact work order as eligible, if the `SPEC-MOK-003` rule 8 amendment is not yet approved by the technical owner, or if `VER-MOK-005`'s extended cases are not yet decided by the assurance owner. During implementation, stop and escalate if:

- the specified shedding order cannot be satisfied at some declared viewport for some accepted seed, which would make it a specification question again rather than an implementation one;
- satisfying rule 8 appears to require a wider minimum viewport, a change to a field's value or source, or a new dependency;
- the observed run stops matching the unobserved run on any declared seed;
- the change would need the public interface widened to be tested;
- required verification fails and cannot be corrected within the scope above.

## Completion report format

Report:

1. the rule 8 behavior as amended, and the tests that discharge each of its cases;
2. any authorized local decisions;
3. verification commands and results, including the three `MOKITERIONS_COMMIT` states;
4. retained evidence paths;
5. whether a release may now stamp a commit, and at what length, stated as a measurement rather than a recommendation;
6. residual limitations and final worktree and candidate-commit status.
