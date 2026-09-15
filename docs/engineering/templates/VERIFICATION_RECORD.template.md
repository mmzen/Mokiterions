+++
id = "VREC-000"
type = "verification_record"
title = "Verification candidate for aggregate work"
status = "ready"
owners = ["quality-owner"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
commit = "0000000000000000000000000000000000000000"
git_object_format = "sha1"
worktree_state = "clean"
prepared_at = "YYYY-MM-DDTHH:MM:SSZ"
prepared_by = "quality-owner"
artifact_snapshot_sha256 = "0000000000000000000000000000000000000000000000000000000000000000"
evidence_paths = ["docs/engineering/DOMAIN/evidence/WO-001-verification.md", "docs/engineering/DOMAIN/evidence/WO-002-verification.md"]
evaluator_evidence_path = "docs/engineering/DOMAIN/evidence/VREC-000-evaluator.json"
evaluator_evidence_sha256 = "0000000000000000000000000000000000000000000000000000000000000000"
# Verification decision fields; omit while status is ready or a prepared record is superseded directly:
# verified_at = "YYYY-MM-DDTHH:MM:SSZ"
# verified_by = "quality-owner"
# Supersession-only fields; omit unless an accountable owner retires a ready record:
# superseded_at = "YYYY-MM-DDTHH:MM:SSZ"
# supersession_authorized_by = "quality-owner"

[relations]
verifies_work_order = ["WO-001", "WO-002"]
conforms_to = ["VER-001", "VER-002"]
# Supersession-only relation; exactly one verified or released coverage-preserving successor:
# superseded_by = ["VREC-001"]
+++

# Verification Record Candidate

Identify the final candidate, the work it covers, and evidence of testing the
combined result. Earlier verified records at different commits may support
this evidence; they remain unchanged. One shared integration report can cover
several work orders. `conforms_to` must contain all their verification contracts.
Keep the record `ready` until an assurance owner explicitly verifies it. Commit
the record and evaluator evidence after the candidate it names.

After a rebase that leaves relevant files unchanged, run
`harnessctl refresh-verification . --from VREC-OLD-001 --id VREC-NEW-001`.
The command compares relevant Git entries and governing inputs, preserves the
original record, and creates a ready successor with a reuse explanation. It
refuses changed relevant contents; test those with `capture-verification`.
Refresh never changes an assurance decision or rewrites verified history.

If a later verified or released VREC fully covers this record's work, a separate accountable governance decision may change only a `ready` record to `superseded`, add the structured fields and one typed successor relation shown above, and retain a decision note. Preserve the original commit, object format, worktree state, `prepared_at`, `prepared_by`, snapshot hash, evidence paths, work orders, and verification contracts. Do not add `verified_at` or `verified_by`: supersession retires the proposal but does not verify it. Superseded records remain historical and are not release-eligible. Historical records without preparation fields may retain their earlier `verified_at` capture timestamp without migration or a fabricated `verified_by`.
