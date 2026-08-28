+++
id = "REL-xxx"
type = "release_contract"
title = "<Promotion decision>"
status = "draft"
owners = ["<release owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
# The release unit is one candidate commit. Its work-order census is measured,
# not declared: `harnessctl release-unit . --from <previous_release_tag> --to <candidate_commit> --toml`
# writes the gates array below, and `--contract <this id>` re-measures it.
candidate_commit = "<full commit id, 40 or 64 hex>"
previous_release_tag = "v<version>"

[relations]
gates = ["WO-xxx", "VER-xxx"]
+++

# Release Contract: <title>

## Release unit

The unit is candidate commit `<candidate_commit>`, cut from `main`. The
`gates` array is the census `harnessctl release-unit` derives from the
`Harness-Work-Order` trailers on the first-parent history from
`<previous_release_tag>` to that commit; every listed work order is
`implemented`. A commit on that path with no trailer is listed here under an
explicit exemption, with the reason, or the derivation fails.

A merge to `main` after the cut changes nothing about this unit. A fix to the
release itself is a new candidate commit on `candidate/<version>` and a new
contract that names it.

## Required evidence

## Compatibility and migration

## Security and provenance

## Promotion policy

## Human approval triggers

## Rollback criteria and procedure

Stop condition: the candidate commit is not an ancestor of the ref being
released, or `harnessctl release-unit --contract <this id>` reports
`E-CIP-001` (the derived census differs from `gates`). The remedy is a new
contract naming a new candidate commit, never an in-place edit of `gates`.

## Post-release observation window
