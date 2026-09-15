+++
id = "REL-xxx"
type = "release_contract"
title = "<Promotion decision>"
status = "draft"
owners = ["<release owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
# Choose the final candidate and the work to release.
# `harnessctl release-unit . --from <tag> --to <commit>` gives advisory history.
candidate_commit = "<full commit id, 40 or 64 hex>"
previous_release_tag = "v<version>"

[relations]
gates = ["WO-xxx", "VER-xxx"]
+++

# Release Contract: <title>

## Release unit

Name the final candidate commit and the work the release owner approves in
`gates`. Use `harnessctl release-unit` to inspect history if useful; missing
trailers and differences from this scope do not require exemptions.

One explicitly verified final-candidate VREC must cover every released work
order and its required verification contracts. Its evidence may cite earlier
records at different commits, but must demonstrate testing of the final
integration. The RLS binds that final VREC at the exact candidate commit.

## Required evidence

## Compatibility and migration

## Security and provenance

## Promotion policy

## Human approval triggers

## Rollback criteria and procedure

Stop if the final candidate lacks complete verified coverage or release approval.

## Post-release observation window
