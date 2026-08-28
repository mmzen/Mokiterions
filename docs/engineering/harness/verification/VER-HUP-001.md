+++
id = "VER-HUP-001"
type = "verification"
title = "Evidence that an adopted evaluator validates the complete graph, that the declaration exempted rather than wrote, and that no product behavior moved"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-28"
updated = "2026-08-28"

[relations]
verifies = ["REQ-HUP-001"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T20:30:00Z"
decided_by = "assurance owner"
+++

# Verification Contract: the standard-root adoption

## Independence

The evaluator that grades the adoption is **not** the tree under test. It is installed from the public
package index at an exact version, into an environment outside this checkout, and every figure this contract
requires is produced by invoking that installation with the repository as its target.

This matters more here than in an ordinary chain. The adoption's own subject includes
`scripts/validate_engineering_artifacts.py` — the in-tree copy of the validator. A verification that ran the
in-tree script would be asking the changed artifact whether the change was correct. The in-tree script is
therefore used exactly once, as assessment A4 below, as a **cross-check** against the external evaluator, and
never as the source of a pass.

The engineering owner performs the transaction and the assurance owner accepts the evidence. This
contract does not require a second human reviewer, and no assessment below is written as one.

## Requirement-to-evidence matrix

| Requirement | Obligation | Evidence |
|---|---|---|
| `REQ-HUP-001` | The adopted evaluator validates the complete artifact graph with zero errors | A1 |

`SPEC-HUP-001`'s rules are covered as follows. Rules 1 to 3 by A2; rule 4 by A3 and N1; rules 5 and 6 by A5;
rule 7 by A6; rule 8 by A2; rule 9 by A2's plan reading no customization; rule 10 by A7.

## Acceptance scenarios

**A1 — the graph validates.** With the adoption applied, run the adopted evaluator's `validate` against the
repository from outside the checkout. **Pass:** zero entries of error severity. Warning and maintenance
entries are counted and retained but do not fail. This is `REQ-HUP-001`'s measure and the only assessment
that carries it.

**A2 — the transaction is planned, bounded and evidenced.** Retain the plan produced before the apply and the
evidence file the apply wrote. **Pass:** the plan names every path the apply reported, reports no path as
`customized` or `conflict`, and the retained evidence names the adopted version and the declared identifiers.

**A3 — the refusal is real.** Before declaring, run the transaction's plan against the undeclared tree.
**Pass:** the notice names `RLS-MOK-001` and the packet field that would declare it. A contract that only
observed the success path would not show that the guard exists.

**A4 — the in-tree validator agrees.** Run the repository's own `scripts/validate_engineering_artifacts.py`
against the same tree. **Pass:** it reports the same error count as A1. A disagreement is a finding about the
adoption, not about the artifacts.

**A5 — the declared record did not move.** Diff `docs/engineering/simulation/releases/RLS-MOK-001.md` across
the whole change. **Pass:** no difference of any kind. This is `SPEC-HUP-001` rule 5 stated as bytes.

**A6 — no version reference is left behind.** Search the repository for the superseded version string.
**Pass:** no repository-owned file names it as the version to install. Historical prose that records what an
earlier root *was* is expected and is not a failure.

**A7 — no product effect.** Diff the change surface against the package directories. **Pass:** no file under
`mokiterions-core/` or `mokiterions-tui/`, and neither `Cargo.toml`, `Cargo.lock` nor `rust-toolchain.toml`,
appears in the change.

## Property and invariant tests

**N1 — the declaration is closed.** The declared identifier resolves to an exemption only while the declaring
work order grants authority and carries a `draft` to `approved` event dated after the record's `released_at`.
Evidence: the resolved exemption set read from the adopted evaluator, showing `RLS-MOK-001` mapped to
`WO-HUP-001`, together with the two instants.

**N2 — the root is internally consistent.** The adopted evaluator's `doctor` reports no FAIL over managed
files, required scripts, seeds and hash-bound classes. Evidence: its full output.

## Static and architecture checks

No new executable code is introduced by this work, so no static analysis is contracted. The managed scripts
the transaction replaces are the adopted evaluator's own and were graded in its release, not here.

The repository gains a second artifact domain, `harness`. Its canonical layout is checked by A1: an artifact
in a non-canonical location is an error the evaluator reports.

## Security and privacy checks

**S1 — nothing secret is read or written.** The transaction reads no credential and contacts no service other
than the public package index. Evidence: the retained transaction evidence and the plan, neither of which
carries a credential field, together with the statement that the evaluator was installed by an unauthenticated
index install.

No further security assessment is contracted. `SPEC-HUP-001` introduces no network path, no input parsing and
no privilege.

## Performance and resilience checks

None contracted. `SPEC-HUP-001`'s *Performance and capacity* fixes no figure, so there is nothing to measure
and a fabricated measurement would be worse than an absence.

## Manual assessments

**M1 — the post-adoption effect is stated where a reader will meet it.** The assurance owner confirms that the
managed continuous-integration trigger's mismatch with the default branch `master` is recorded in
`SPEC-HUP-001` and in the work order's evidence, and that it is recorded as deferred rather than as resolved.

This is the only manual assessment, and it is a reading of committed text that one accountable person
performs. No assessment in this contract requires a judgment that cannot be made from the retained evidence.

## Evidence retention

Retained under `docs/engineering/harness/evidence/WO-HUP-001/`:

- the transaction plan, before and after declaration (A2, A3);
- the evaluator's applied-transaction evidence file (A2);
- `validate` output from the adopted evaluator, with its error and warning counts (A1);
- `doctor` output (N2);
- the in-tree validator's output (A4);
- the resolved exemption set and the two instants (N1);
- the change-surface listing used by A5, A6 and A7;
- a completion summary recording the adopted version, the declared identifiers, and the deferred branch
  question (M1).

## Residual uncertainty

- **The managed policy adopted here is graded by its own author.** This contract confirms that the adopted
  evaluator accepts this repository; it does not independently assess whether `0.8.0`'s policy is correct.
  That is the adopted release's own assurance, not this repository's.
- **The pull-request lane is observed once.** A1 through A7 are measured on the branch. Whether the managed
  workflow behaves as intended over a full merge cycle is observable only after the merge, and the loss of the
  default-branch push run means the first post-merge reading will come from a pull request rather than from a
  push event.
- **The 141 authoring advisories are carried forward unaddressed.** They are recorded, not resolved, and a
  later reader will meet them.
