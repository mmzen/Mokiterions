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

Identify the release contract, one explicitly verified final-candidate VREC,
and all released work. The final VREC must cover every work order and its
verification contracts at the RLS candidate commit. Evidence may refer to
earlier records; their commits need not match the final candidate. Keep this
record `ready` until the release owner explicitly authorizes release.