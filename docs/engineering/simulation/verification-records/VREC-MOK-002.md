+++
id = "VREC-MOK-002"
type = "verification_record"
title = "Verification candidate for WO-MOK-002"
status = "ready"
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

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-002` to candidate commit `68163ac452619e2f8d5a05ed3a73d42b920ba5f6`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

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

## What the accountable assurance owner should weigh before transitioning to `verified`

Neither item below is a failure against this contract. Both are disclosed here because verifying
this record endorses the judgement that they are acceptable.

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

This record is `ready`. Preparing it exercised no accountable decision: it did not verify, approve,
release, tag, publish, or deploy, and it did not alter the candidate commit. Transition to
`verified` is the accountable assurance owner's decision, and release remains a separate record and
a separate decision after that.
