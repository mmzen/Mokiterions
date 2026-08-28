+++
id = "RLS-000"
type = "release_record"
title = "Release candidate VERSION"
status = "ready"
owners = ["release-owner"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
version = "VERSION"
commit = "0000000000000000000000000000000000000000"
git_object_format = "sha1"
prepared_at = "YYYY-MM-DDTHH:MM:SSZ"
prepared_by = "release-owner"
# Release decision fields; omit while status is ready:
# released_at = "YYYY-MM-DDTHH:MM:SSZ"
# authorized_by = "release-owner"
tag = "vVERSION"
evaluator_evidence_path = "docs/engineering/DOMAIN/evidence/RLS-000-evaluator.json"
evaluator_evidence_sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[relations]
satisfies = ["REL-000"]
includes_verification = ["VREC-001"]
releases_work = ["WO-001", "WO-002"]
+++

# Release Record Candidate

Copy the one shared commit from the included verified verification records and bind the canonical lock-matching released-evaluator evidence produced during preparation. The released-work set must equal their verification-coverage union, and every work order must be gated by the release contract. A single work order remains valid. Keep status `ready` until the accountable release owner authorizes the instance. Preparation metadata is not release authorization. Repository-specific payload or publication policy remains outside this portable template. This document does not create a tag or publish a release.
