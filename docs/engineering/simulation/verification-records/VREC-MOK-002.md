+++
id = "VREC-MOK-002"
type = "verification_record"
title = "Verification candidate for WO-MOK-002"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-17"
updated = "2026-08-17"
commit = "68163ac452619e2f8d5a05ed3a73d42b920ba5f6"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-17T15:49:52Z"
artifact_snapshot_sha256 = "35a66d905d5e090783d94a2b936be3c855605ec1fd0f80735078aee0363ce1ae"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-002/boundary-and-security-review.md", "docs/engineering/simulation/evidence/WO-MOK-002/calibration-record.md", "docs/engineering/simulation/evidence/WO-MOK-002/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-002/density-curve.md", "docs/engineering/simulation/evidence/WO-MOK-002/determinism-and-resilience.md", "docs/engineering/simulation/evidence/WO-MOK-002/escalation.md", "docs/engineering/simulation/evidence/WO-MOK-002/lever-sensitivity-study.md", "docs/engineering/simulation/evidence/WO-MOK-002/manual-observation.md", "docs/engineering/simulation/evidence/WO-MOK-002/requirement-to-test-mapping.md", "docs/engineering/simulation/evidence/WO-MOK-002/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-002/test-run.txt"]

[relations]
verifies_work_order = ["WO-MOK-002"]
conforms_to = ["VER-MOK-002"]
+++

# Verified Verification Record

Following review of the retained evidence by the accountable assurance owner, this record was transitioned from `ready` to `verified` on 2026-08-17. It binds `WO-MOK-002` to candidate commit `68163ac452619e2f8d5a05ed3a73d42b920ba5f6` without changing its captured provenance.

The verification transition is a later governance decision. It does not alter the candidate commit, release, tag, publish, or deploy anything.

The record was prepared after the candidate commit it names, avoiding self-referential commit metadata.

## What this record claims

`WO-MOK-002` is `implemented` and `VER-MOK-002` is `approved`. At candidate commit
`68163ac452619e2f8d5a05ed3a73d42b920ba5f6` every case, invariant, static check, security check,
resilience check, and manual assessment in `VER-MOK-002` was executed, and each one is mapped to a
named test or a retained evidence file in `requirement-to-test-mapping.md`.

| Gate | Result |
|---|---|
| `cargo test` | 52 passed, 0 failed, 0 ignored |
| `cargo fmt --all -- --check` | no differences |
| `cargo clippy --all-targets --all-features -- -D warnings` | 0 findings |
| `cargo build` | clean |
| Dependencies | empty; `Cargo.toml` and `Cargo.lock` unchanged |
| Artifact validation | PASS — 0 errors, 0 warnings |

`REQ-MOK-014`'s floor is met on every declared seed at the declared density of `0.75%`: **8, 11, 8,
9, and 11** survivors on seeds `0`, `1`, `42`, `123`, and `777`. Not one seed retains all twelve, so
`INT-MOK-002`'s scarcity principle holds and the adverse-observation condition in `VER-MOK-002` is
not triggered.

## What the accountable assurance owner weighed before verifying

Neither item below is a failure against this contract. Both were disclosed before the decision and
both are accepted by the verification recorded above, which endorses the judgement that they are
acceptable at this candidate commit.

1. **The floor has no margin.** Eight is the measured worst case and it is reached on two of five
   declared seeds. `VER-MOK-002` records this as a deliberate trade rather than an oversight, since
   stating the floor at a comfortable density would have contradicted the scarcity principle.
2. **High-class resources accumulate, and the long horizon is worse than the short one.** A
   10,000-tick run at the declared density reaches extinction at tick 9,154. No requirement in this
   contract's scope speaks past tick 1,000, where the corrected rule 5 raises the worst case from
   three survivors to eight. The product owner accepted this knowingly on 2026-08-17 and deferred it
   to Phase 2.

Two stop conditions fired during `WO-MOK-002` and both were resolved by amended artifacts and
re-approval rather than by retuning a constant. `escalation.md` records both in full, including the
options not chosen. During the second escalation an earlier claim in `SPEC-MOK-001` about the
two-cell oscillation was found to be false and is retracted in place rather than silently corrected.

Two limits of coverage are stated plainly: the floor is verified on five declared seeds rather than
the seed space, and at one declared density with no claim in either direction at any other, because
per-seed survivor counts are demonstrably non-monotonic in density.

## Authority

This record is `verified`. The verification decision was taken by the repository owner acting as
accountable assurance owner on 2026-08-17, after review of the evidence listed above; it was not
taken by an implementation agent, and `DECISION_RIGHTS` places it outside what such an agent may
inherit. Recording the decision changed the record's status and this prose only.

Every captured provenance field is preserved exactly as the managed capture produced it: the
candidate commit, the git object format, the clean worktree state, the capture timestamp, the
artifact snapshot hash, the evidence paths, the verified work order, and the verification contract.
`WO-MOK-002` remains `implemented`; verification is carried by this record, not by a change to the
work order.

Verification is not release. Nothing here releases, tags, publishes, or deploys, and no release
record exists. Release remains a separate record and a separate accountable decision.
