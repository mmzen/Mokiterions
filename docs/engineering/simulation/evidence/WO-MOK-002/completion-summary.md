# Completion summary — WO-MOK-002

Reported 2026-08-17 in the format `WO-MOK-002` requires. The work order is `implemented`.

## 1. Implemented requirements and affected components

| Requirement | State |
|---|---|
| `REQ-MOK-013` — bounded perception | implemented, fully verified |
| `REQ-MOK-014` — population viability at a stated density | implemented, floor of 8 met on every declared seed |
| `REQ-MOK-015` — reference decision source | implemented, fully verified |

Governed by `SPEC-MOK-001` as amended three times on 2026-08-17, and covered by `VER-MOK-002`.

Components:

- `src/simulation.rs` — perception in the observation; the `Density` type and its exact integer
  mapping; density bound to initialization, capacity, and the replenishment target; the reference
  decision source and its ranking rules including the non-waste test on cases 1 and 3;
  automated tests.
- `src/cli.rs` — `--policy` and `--density` inputs, usage text, argument tests.
- `src/main.rs` — process-boundary test for a zero-resource density.
- `Cargo.toml` and `Cargo.lock` — unchanged, as the work order expected. Dependencies remain empty.

Artifacts amended: `SPEC-MOK-001` (rule 5, third amendment), `REQ-MOK-014` (single declared
density, floor raised to 8), `VER-MOK-002` (matrix, residual uncertainty, manual assessments),
`WO-MOK-002` (scope, escalation resolutions, status). `../../../../ROADMAP.md` records the
one deferred item in Phase 2.

## 2. Authorized local decisions

Within the work order's decision envelope:

- Private naming: `Density`, `best_fitting_co_located_food`, `best_fitting_distant_food`, and a
  shared `fits` predicate. The non-waste test lives in one place because rule 5 applies one test
  to two cases; duplicating it would let the two drift, which is exactly the defect the third
  amendment fixed.
- Removed the reference hunger threshold constant. The corrected rule states no threshold, and
  the equivalent bound now derives from the food table. Retaining a constant no longer used in
  non-test code would have been dead code.
- Test organization: a `config_at(seed, ticks, density)` helper, and `DECLARED_SEEDS` and
  `DECLARED_FLOORS` constants so the seed set and floor are stated once and cannot drift from
  `VER-MOK-002`.
- The new rule-5 test asserts entropy draw counts to distinguish a search step from an approach.
  This tests observable behavior at the boundary without naming how ranking is implemented, which
  `VER-MOK-002`'s independence section requires.

Nothing outside the envelope was changed. No specified constant, mapping, rounding rule, declared
density, floor, event field, exit code, trust boundary, or lifecycle status was altered except as
an approved amendment directed.

## 3. Verification commands and results

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | no differences |
| `cargo clippy --all-targets --all-features -- -D warnings` | 0 findings |
| `cargo build` | clean |
| `cargo test` | **52 passed, 0 failed, 0 ignored** |
| `bash scripts/check_engineering_harness.sh .` | PASS — 27 artifacts, 0 errors, 0 warnings |
| `python scripts/inspect_engineering_artifacts.py --root .` | PASS — 0 decisions required, 0 active work |

Toolchain: cargo 1.97.1, rustc 1.97.1. Full output in `static-checks.txt` and `test-run.txt`.

Beyond the gates: 1,000-tick runs on all five declared seeds at the declared density; a nine-point
density sweep on all five seeds; byte-identical replay for both sources at two densities;
10,000-tick runs for both sources at two densities; a 20-tick traced excerpt; an oscillation
measurement against a random-walk baseline; and a resource-mix measurement at seven horizons.

## 4. Per-seed survivors and consumption, and the carrying capacity implied

1,000 ticks, reference source, declared density `0.75%` (61 resources per territory):

| Seed | Survivors | Floor of 8 met | Consumed | Regenerated |
|---:|---:|:---:|---:|---:|
| `0` | 8 | yes | 379 | 373 |
| `1` | 11 | yes | 410 | 384 |
| `42` | 8 | yes | 344 | 324 |
| `123` | 9 | yes | 324 | 323 |
| `777` | 11 | yes | 372 | 350 |

**Measured carrying capacity: 8 to 11 of 12 at tick 1,000, worst case 8.** The floor sits exactly
on the worst case, so there is no margin; `VER-MOK-002` records that as deliberate. No seed retains
all twelve, so scarcity holds. No territory reached zero resources on any seed.

This is a measurement at tick 1,000 and not a steady state. The same configuration declines under
sustained running and reaches extinction at tick 9,154 — see item 6.

## 5. Retained evidence

All under `docs/engineering/simulation/evidence/WO-MOK-002/`:

| File | Contents |
|---|---|
| `static-checks.txt` | toolchain, dependency state, formatter, linter, build output |
| `test-run.txt` | full `cargo test` output |
| `calibration-record.md` | per-seed survivors, consumption, depletion, death timing, cause of health loss, resource mix, carrying capacity |
| `density-curve.md` | nine-density per-seed sweep, before-and-after comparison, non-monotonicity examples, confirmation experiment, oscillation and accumulation measurements |
| `manual-observation.md` | 20-tick traced excerpt with a rule-by-rule reading, and assessments of search-while-full, oscillation, scarcity, and resource mix |
| `determinism-and-resilience.md` | four replay hashes, 10,000-tick results, the long-horizon decline table, bounded-work analysis |
| `requirement-to-test-mapping.md` | every `VER-MOK-002` row mapped to a named test or evidence file |
| `boundary-and-security-review.md` | dependency, trust-boundary, input-surface, network, credential, and output-hygiene review |
| `escalation.md` | both escalations in full, including options not chosen, with resolutions |
| `lever-sensitivity-study.md` | the pre-density diagnosis that identified capacity as the governing lever, retained as history |
| `completion-summary.md` | this file |

## 6. Residual limitations and deferred features

- **High-class resource accumulation, deferred to Phase 2.** The corrected rule 5 makes a
  high-class resource approachable only at satiety of at most `50`, so it accumulates against
  capacity — roughly three quarters of standing supply by tick 1,000. Territories stay full while
  the food anyone will walk to grows scarce, and a 10,000-tick run at the declared density reaches
  **extinction at tick 9,154** where the previous rule left one survivor. Measured, disclosed
  before the decision, and accepted by the product owner because no requirement in scope speaks
  past tick 1,000, where the rule raises the worst case from three survivors to eight.
- **No margin on the floor.** Eight is the measured worst case, on two of five seeds. Any change
  to world size, movement cost, satiety decay, the calorie table, or rule 5 will likely break it.
- **One declared density.** The floor is an obligation at a point. Interpolation and extrapolation
  are unsound, and per-seed counts are demonstrably non-monotonic in density.
- **Five seeds, not the seed space.** Passing does not prove viability for every seed.
- **A full Mokiterion cannot approach anything**, since even low class clips above satiety 85, so
  the reference source is indistinguishable from a random walk for part of every life. Assessed as
  acceptable and arguably preferable to the previous camping behavior.
- Out of scope and not implemented, per the work order: model-provider integration, fear, traits,
  combat, social behavior, structured output, persistence, aggregate analysis, new actions,
  graphical or web interfaces, networking, async runtimes, databases.

## 7. Worktree and candidate-commit status

- Branch `feature/phase-1-definition`, which branched from `master` at `09c4e1a` with no divergence
  before this work. Nothing has been pushed, tagged, or released.
- The whole of this work is captured as **one candidate commit** on that branch, authorized by the
  repository owner on 2026-08-17. It contains:
  - modified `src/simulation.rs`, `src/cli.rs`, `src/main.rs`,
    `docs/engineering/simulation/specifications/SPEC-MOK-001.md`, `../../../../ROADMAP.md`;
  - new `INT-MOK-002`, `CAP-MOK-002`, `REQ-MOK-013`, `REQ-MOK-014`, `REQ-MOK-015`, `VER-MOK-002`,
    `WO-MOK-002`, and this evidence directory.
- `Cargo.toml` and `Cargo.lock` are unmodified, and `master` is untouched.
- `WO-MOK-002` classifies commit-bound verification as `required`. The candidate commit exists to be
  bound; its hash is recorded by the verification record rather than here, because a file cannot
  state the hash of the commit that contains it. Pushing, opening a pull request, capturing
  `VREC-MOK-002`, and releasing are accountable owner decisions and none has been taken.
- No secret, credential, or model-provider key is present in the source or in any evidence file.
