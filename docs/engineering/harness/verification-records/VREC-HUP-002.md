+++
id = "VREC-HUP-002"
type = "verification_record"
title = "Verification candidate for WO-HUP-002"
status = "verified"
owners = ["assurance owner"]
created = "2026-08-28"
updated = "2026-08-28"
commit = "c0f54a8015eff3467f9ccb0f7ca7c1a9d23a669f"
git_object_format = "sha1"
worktree_state = "clean"
prepared_at = "2026-08-28T21:12:41Z"
prepared_by = "assurance owner"
artifact_snapshot_sha256 = "e03e49cf3b16c2937c1ad6356bce0b948fe6d474916cf2c8809cd5d38f67bba5"
evidence_paths = ["docs/engineering/harness/evidence/WO-HUP-002/b1-refusals-before.md", "docs/engineering/harness/evidence/WO-HUP-002/b2-starts-after.md", "docs/engineering/harness/evidence/WO-HUP-002/b3-b4-change-surface.md", "docs/engineering/harness/evidence/WO-HUP-002/b5-validate.md", "docs/engineering/harness/evidence/WO-HUP-002/completion-summary.md", "docs/engineering/harness/evidence/WO-HUP-002/handoff-check.md", "docs/engineering/harness/evidence/WO-HUP-002/m1-cause-recorded.md", "docs/engineering/harness/evidence/WO-HUP-002/p1-p2-surface-to-scope.md"]
evaluator_evidence_path = "docs/engineering/harness/evidence/VREC-HUP-002-evaluator.json"
evaluator_evidence_sha256 = "4f500366462d5da855322aa725d6a1d23250f1ef82b37e371b43317ef81945b6"

verified_at = "2026-08-28T21:13:17Z"
verified_by = "assurance owner"
[relations]
verifies_work_order = ["WO-HUP-002"]
conforms_to = ["VER-HUP-002"]

[[lifecycle_events]]
from = "ready"
to = "verified"
decided_at = "2026-08-28T21:13:17Z"
decided_by = "assurance owner"
+++

# Verification Record Candidate

## Verification decision, 2026-08-28

The repository owner, acting as accountable assurance owner, verified this record on 2026-08-28 under
`DR-VREC-DECIDE`, having authorized the repair to run through to verification. The transition was taken with
the released 0.8.0 evaluator through `python -I -m se_harness` and moved `status`, `verified_at` and
`verified_by` and nothing else.

Every figure was re-measured immediately before the field moved, because a verified record can never be
corrected:

| Figure | Reading |
|---|---|
| Worktree | clean |
| Bound candidate `c0f54a8` | exists, 8 of 8 evidence files tracked at it |
| `validate` | exit 0, **0 errors**, **142 warnings** |
| `doctor` | **0 FAIL** |
| `WO-HUP-002` | `implemented` |
| `WO-MOK-026` / `WO-MOK-027` start checkpoints | **Completed** / **Completed** |
| `isolated_python` in the evaluator proof | `true` |

`B2` carries `REQ-HUP-002` and reads zero refusals over the enumerated set, against two before the repair.

Two things this record deliberately does **not** do. It does not correct `VREC-HUP-001`'s `a1-validate.md`,
which reports 141 warnings where the evaluator prints 142: that record is verified and terminal, and the
erratum needs an additional record rather than an edit. It does not re-open the 0.8.0 adoption, whose chain
stays verified — the finding against it is recorded in `REQ-HUP-002`'s rationale and in `SPEC-HUP-001` rule 11,
which is where the next adoption will meet it.

The warning figure above is stated as the evaluator prints it, by total rather than by code family, which is
the specific mistake `B5` exists to prevent from recurring.


This ready record binds retained evidence for `WO-HUP-002` to candidate commit `c0f54a8015eff3467f9ccb0f7ca7c1a9d23a669f`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.
