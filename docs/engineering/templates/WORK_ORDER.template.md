+++
id = "WO-xxx"
type = "work_order"
title = "<Bounded implementation objective>"
status = "draft"
owners = ["<engineering owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"

[relations]
implements = ["REQ-xxx"]
specifications = ["SPEC-xxx"]
architecture = ["ARCH-xxx", "ADR-xxx"]
verification = ["VER-xxx"]
+++

# Work Order: <title>

## Lifecycle

Use `approved` to authorize bounded execution and `implemented` after the work and retained evidence are complete. Governance-only work normally stops at `implemented`. Use `verified` or `released` only when an eligible commit-bound VREC explicitly covers this work order under the repository's configured provenance policy.

## Objective

## In scope

## Out of scope

## Authorized decision envelope

What may the implementation agent decide locally?

## Constraints

## Expected change surface

Use components rather than guessed files when the code has not yet been inspected.

## Required verification

## Evidence to record

## Stop and escalate conditions

## Completion report format
