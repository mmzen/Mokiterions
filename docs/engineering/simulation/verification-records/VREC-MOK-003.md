+++
id = "VREC-MOK-003"
type = "verification_record"
title = "Verification candidate for WO-MOK-003"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-17"
updated = "2026-08-17"
commit = "a7f39f18520433cddc72c044c6fca1b24104bb7d"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-17T17:46:18Z"
artifact_snapshot_sha256 = "c0f473fbd1ce22e7081b7f738b8898f050b13dd60bbb79aa15645b57f443b90e"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-003/README.md", "docs/engineering/simulation/evidence/WO-MOK-003/after/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-003/after/manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-003/after/summaries.txt", "docs/engineering/simulation/evidence/WO-MOK-003/after/test-census.txt", "docs/engineering/simulation/evidence/WO-MOK-003/after/viability.txt", "docs/engineering/simulation/evidence/WO-MOK-003/amendment-approvals.md", "docs/engineering/simulation/evidence/WO-MOK-003/baseline-comparison.md", "docs/engineering/simulation/evidence/WO-MOK-003/baseline/capture.sh", "docs/engineering/simulation/evidence/WO-MOK-003/baseline/exit-codes.txt", "docs/engineering/simulation/evidence/WO-MOK-003/baseline/full/short_seed42_baseline_trace_on.txt", "docs/engineering/simulation/evidence/WO-MOK-003/baseline/full/short_seed42_reference_trace_on.txt", "docs/engineering/simulation/evidence/WO-MOK-003/baseline/manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-003/baseline/summaries.txt", "docs/engineering/simulation/evidence/WO-MOK-003/baseline/test-census.txt", "docs/engineering/simulation/evidence/WO-MOK-003/baseline/viability.txt", "docs/engineering/simulation/evidence/WO-MOK-003/boundary-review.md", "docs/engineering/simulation/evidence/WO-MOK-003/compile-time.md", "docs/engineering/simulation/evidence/WO-MOK-003/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-003/public-surface-inventory.md", "docs/engineering/simulation/evidence/WO-MOK-003/relocated-test-diff-body.md", "docs/engineering/simulation/evidence/WO-MOK-003/relocated-test-diff.md", "docs/engineering/simulation/evidence/WO-MOK-003/requirement-to-test-mapping.md", "docs/engineering/simulation/evidence/WO-MOK-003/resilience-10k.txt", "docs/engineering/simulation/evidence/WO-MOK-003/resilience-and-viability.md", "docs/engineering/simulation/evidence/WO-MOK-003/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-003/test-census.md", "docs/engineering/simulation/evidence/WO-MOK-003/test-placement.md", "docs/engineering/simulation/evidence/WO-MOK-003/test-run.txt"]

[relations]
verifies_work_order = ["WO-MOK-003"]
conforms_to = ["VER-MOK-003"]
+++

# Verified Verification Record

Following review of the retained evidence by the accountable assurance owner, this record was transitioned from `ready` to `verified` on 2026-08-17. It binds `WO-MOK-003` to candidate commit `a7f39f18520433cddc72c044c6fca1b24104bb7d` without changing its captured provenance.

The verification transition is a later governance decision. It does not alter the candidate commit, release, tag, publish, or deploy anything.

The record was prepared after the candidate commit it names, avoiding self-referential commit metadata.

## What this record claims

`WO-MOK-003` is `implemented` and `VER-MOK-003` is `approved`. At candidate commit
`a7f39f18520433cddc72c044c6fca1b24104bb7d` every case in the requirement-to-evidence matrix, every
property and invariant, every static and architecture check, every security and privacy check, every
performance and resilience check, and every manual assessment in `VER-MOK-003` was executed. Each one
is mapped to a named test or a retained evidence file in `requirement-to-test-mapping.md`.

| Gate | Result |
|---|---|
| `cargo test` | 52 passed, 0 failed, 0 ignored, across seven test executables plus a doc-test pass reporting zero tests |
| `cargo fmt --all -- --check` | no differences |
| `cargo clippy --all-targets --all-features -- -D warnings` | 0 findings, and no `non_snake_case` finding against either target name |
| `cargo build` | clean |
| Dependencies | dependency and dev-dependency tables empty; no build script; `Cargo.lock` unchanged |
| Artifact validation | PASS — 0 errors, 0 warnings, across the 36-artifact graph this record's snapshot binds |

Toolchain: rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable, edition 2024.

### The three independent oracles

`VER-MOK-003` requires that none of the oracles be the changed code's own suite, because a suite that
passes because nothing changed would also pass if the restructuring had never happened. All three were
used and all three could have failed.

1. **The recorded pre-change baseline.** Captured at `77010d02319051a20f8e45282f9c813ce4199956`
   before the first edit and never regenerated. Post-change output is **byte-identical** across all 43
   matrix cells in digest, line count, and exit code; all 40 summary lines match in full text, including
   the per-territory and per-class counts that no accessor exposes; all 16 diagnostic cases match in
   exit code, stdout digest and length, and complete standard-error text; and the two retained traced
   runs match file to file. Not one differing byte.
2. **The public-surface inventory.** **13 public items**, each classified as a value, a value
   constructor, a pure function of a value, or a copying accessor, derived twice independently — from
   rustdoc and from the sources — and diffed against `SPEC-MOK-002` rule 5. No surplus item, and no
   item on rule 6's prohibited list is public or reachable from a public item. The inventory does not
   read the tests, so it could not be satisfied by a test that avoids a leaked item.
3. **The test census.** `cargo test -- --list` before and after, reconciled name by name: **52 before,
   52 after, 0 lost, 0 gained, 0 ignored**, 15 relocated and 37 in place. The pre-change distribution
   `VER-MOK-003` fixed in advance — 5 in `src/cli.rs`, 43 in `src/simulation.rs`, 4 in `src/main.rs` —
   reconciles exactly: all 5 and all 4 relocated, plus 6 of the 43, giving the 15 in `tests/`.

`REQ-MOK-016`'s floor conditions hold: exactly one `[lib]` named `mokiterions` at `src/lib.rs` and one
`[[bin]]` named `Mokiterions` at `src/main.rs`, no third target, no workspace, no conditional
visibility, no `pub(crate)`, and no `#[cfg]` in `src/` except the single `#[cfg(test)]` on the internal
test module. `REQ-MOK-017`'s tier disjointness and totality hold: every one of the 52 tests is in
exactly one tier, every one of the 37 internal tests has at least one named non-public dependency
derived mechanically with an empty list treated as a failure, and each relocated test's assertions are
verbatim — 11 of 15 bodies character-identical, the other 4 changed only by a field read becoming an
accessor call returning the same value.

`VER-MOK-002`'s population floor is met on every declared seed at the declared density of `0.75%`:
**8, 11, 8, 9, and 11** survivors on seeds `0`, `1`, `42`, `123`, and `777` — identical to the counts
`VREC-MOK-002` recorded, not merely above the floor. Four 10,000-tick runs complete without panic with
survivors plus deaths equal to twelve in each, at the same termination ticks as `WO-MOK-002` recorded.

## What the accountable assurance owner weighed before verifying

Neither matter below is a failure against this contract. Both were disclosed before the decision and both
are accepted by the verification recorded above, which endorses the judgement that they are acceptable at
this candidate commit.

1. **Two of `SPEC-MOK-002` rule 8's per-file subjects remain covered from the internal tier.**
   Termination by extinction requires clearing the resource collection and writing agent health and
   satiety directly; the density-to-capacity binding requires per-territory resource counts before any
   tick and direct regeneration. Rule 6 prohibits exposing either, so both tests stayed inline with
   their assertions intact, and the shortfall is recorded rather than resolved by widening the
   interface. This did not trigger the work order's stop-and-escalate condition, which fires only when
   leaving a test inline would leave a `VER-MOK-001` or `VER-MOK-002` case covered *only by a weaker
   assertion*; both tests keep their original assertions at their original strength. The verification
   recorded above endorses recording the insufficiency against rule 8's subject wording instead of
   growing the public interface, and accepts that a future amendment may reword those subject lists.
2. **`RunSummary` carries a pre-existing `Debug` derive that renders the two withheld counts as text.**
   Rule 5 authorizes six accessors and four are exposed; population per territory and resource counts
   per class were deliberately omitted because no relocated test reads either, the enumeration being a
   ceiling rather than a checklist. The derive predates this work, the values it renders are already
   printed on every summary line, and it grants no borrow and no mutation path — so it is recorded as an
   observation rather than waived. The verification recorded above accepts that judgement.

No new test was written to close either gap, because `VER-MOK-003` conserves the census at 52 and
adding one would itself have been a violation.

## Limits of coverage stated in this record

- **Equivalence is demonstrated across the declared matrix, not the input space.** 5 seeds × 2 decision
  sources × 2 declared densities × 2 trace settings at 1,000 ticks, plus two 20-tick traced runs, the
  no-argument invocation, 16 diagnostic cases, and four 10,000-tick runs. Undeclared seeds, undeclared
  densities, and the release profile are not covered. `SPEC-MOK-002` rule 11 scopes the obligation to
  the declared seeds and densities, so this is the shape of the claim rather than a shortfall in
  executing it.
- **No compile-fail harness exists.** That a public-tier test cannot reach authoritative state is
  established by inventory and review, not by a mechanical proof that a violating test fails to
  compile, which would need an excluded test dependency. Review is load-bearing here. This is why the
  inventory was derived independently of the suite.
- **The interface is verified read-only as written today, and is not verified to be well designed.**
  Whether it is the right contract for Phase 4 and Phase 5 is established when those phases are written
  against it; a shortfall found then is an amendment to `SPEC-MOK-002`, not a defect in this
  verification.
- **The initiative's benefit is not directly measured.** Demonstrating that a break in the
  operator-visible contract now fails a test would require deliberately breaking it, which is out of
  scope. What is verified is the precondition: a populated tier reaches the program only through that
  contract.
- **The survivor floor still has no margin**, with two declared seeds sitting exactly on eight, and
  **long-horizon extinction under the reference source is unchanged** at tick 9,154 at the default
  density. Both are pre-existing properties of the world model that `VREC-MOK-002` already records; the
  product owner accepted the latter on 2026-08-17 and deferred it to Phase 2. Reproducing both exactly
  is the correct outcome for an equivalence-preserving change.

The `ARCH-MOK-001`, `ADR-MOK-001`, and `SPEC-MOK-001` amendments named in `ADR-MOK-002` were approved
by the repository owner on 2026-08-17 **before any code changed**, which `VER-MOK-003` requires
independently of code state; `amendment-approvals.md` records the dates, the authority, and the ordering
of the steps. Compile-time growth is recorded with no pass condition attached: incremental
`cargo test --no-run` after touching `src/simulation.rs` goes from 554 ms to 1,006 ms and `target/`
from 26 MB to 88 MB.

## Authority

This record is `verified`. The verification decision was taken by the repository owner acting as
accountable assurance owner on 2026-08-17, after review of the evidence listed above; it was not taken
by an implementation agent, and `DECISION_RIGHTS.md` places it outside what such an agent may inherit.
Recording the decision changed the record's status and this prose only.

Every captured provenance field is preserved exactly as the managed capture produced it: the candidate
commit, the git object format, the clean worktree state, the capture timestamp, the artifact snapshot
hash, the evidence paths, the verified work order, and the verification contract. The snapshot was taken
before this record was written, so it records the graph the record binds rather than the graph containing
the record. No provenance field was recomputed for this transition, and the candidate commit is
unchanged.

`WO-MOK-003` remains `implemented`; verification is carried by this record, not by a change to the work
order, matching how `VREC-MOK-001` and `VREC-MOK-002` were verified.

One derived observation survives this transition and is recorded here rather than left to be
rediscovered. The Harness Explorer raises `W-HEX-001` against `WO-MOK-003`, reporting no evidence
document keyed to its ID. That is a detection mismatch, not a shortfall: `discover_evidence` in
`scripts/generate_harness_dashboard.py` keys evidence on the *file name*, while this work order's
evidence is retained in a *directory* named for the work order, so no file name matches the pattern.
`docs/mokiterions/ROADMAP.md` already records the mismatch and the need to align the convention in one
direction. It applies identically to `WO-MOK-001` and `WO-MOK-002`, whose records are also `verified`,
and because the rule's status set includes `verified` and `released`, the observation does not clear on
verification and would not clear on release. The 29 retained evidence files are enumerated in
`evidence_paths` above and were reviewed for this decision.

Verification is not release. Nothing here releases, tags, publishes, or deploys, and no release record
exists. Release remains a separate record and a separate accountable decision.
