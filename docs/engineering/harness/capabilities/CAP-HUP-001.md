+++
id = "CAP-HUP-001"
type = "capability"
title = "The engineering owner can move the standard root to an exact released evaluator in one bounded transaction that leaves the graph valid"
status = "approved"
owners = ["product owner"]
created = "2026-08-28"
updated = "2026-08-28"

[relations]
derives_from = ["INT-HUP-001"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T20:00:00Z"
decided_by = "product owner"
+++

# Capability: adopt an exact released evaluator

## Actor and need

The **engineering owner** needs to move this repository's standard root from the evaluator version it
currently carries to a named, exact, published `se_harness` release, and to know before starting whether the
move will succeed, what it will change, and what will be true when it finishes.

Today that need is met by investigation. The owner cannot tell from the repository whether an upgrade will
apply cleanly, whether the resulting tree validates, or what the new evaluator will refuse that the old one
allowed. The `E012` condition `INT-HUP-001` records is exactly that gap: it is discoverable only by running a
newer evaluator against the tree and reading what comes back.

## Capability statement

The engineering owner can adopt an exact released evaluator version as the repository's standard root through
a single planned, applied and evidenced transaction, which:

- is planned before it is applied, and reports its whole file surface in the plan;
- refuses rather than proceeds when the repository is not in a state the adoption can leave valid;
- resolves releases that predate evaluator-evidence enforcement by an accountable declaration attached to the
  authorizing work order, without altering the release records themselves;
- replaces managed files, integrates fragments, and leaves repository-owned files alone; and
- retains evidence naming the adopted version and what the transaction declared.

## Boundaries

**Inside.** The standard root: the managed configuration, the lock, the managed contract documents, the
managed templates, the managed scripts, the managed continuous-integration workflow, the managed agent
instruction fragments, and the repository-owned files that name the evaluator version.

**Outside.** Product code, simulation behavior, tests, product releases and tags, existing verification
records, and the content of any engineering artifact in the `simulation` domain. The adoption reads those and
changes none of them.

**Also outside.** Deciding the *policy* the new evaluator brings. The adoption takes the managed policy of the
version it adopts as given. Where that policy does not fit this repository, the mismatch is recorded as a
post-adoption effect and settled by its own change.

## Outcomes

- A named evaluator version governs the repository, recorded in `.engineering-harness.toml` and in the lock.
- The complete artifact graph validates under that evaluator with zero errors.
- Standing releases that predate evaluator-evidence enforcement are declared, and the repository is not frozen
  by them.
- The transaction's surface and declarations are retained as evidence.
- The next adoption is a repeat of `SPEC-HUP-001`, at a different version.

## Candidate requirements

- `REQ-HUP-001` — the adopted root validates the complete artifact graph with zero errors. *(Approved for this
  capability.)*
- A release that predates evaluator-evidence enforcement is resolved by declaration and not by written
  evidence. *(Stated as `SPEC-HUP-001` rules 4 to 6 under `REQ-HUP-001`; a separate requirement is not raised
  because the obligation is observable only through the same measure.)*
- The adoption retains transaction evidence naming the adopted version. *(Stated as `SPEC-HUP-001` rule 8.)*
