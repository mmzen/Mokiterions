+++
id = "INT-HUP-001"
type = "intent"
title = "The repository is governed by a current, supported evaluator, and moving to one is a bounded transaction rather than a rediscovery"
status = "approved"
owners = ["product owner"]
created = "2026-08-28"
updated = "2026-08-28"

[relations]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T20:00:00Z"
decided_by = "product owner"
+++

# Intent: a current evaluator, adopted by a bounded transaction

## Problem

This repository installed its standard root on 2026-08-11 at `se_harness` **0.4.0** and has not moved it
since. Five releases have shipped in the interval — `0.5.0`, `0.6.0`, `0.7.0`, `0.7.1` and `0.8.0` — and the
distance is no longer only a question of missing features.

From `0.6.0` onward the evaluator binds every **released** record to evaluator evidence. `RLS-MOK-001`, this
repository's `0.1.0` release, was cut on 2026-08-19 under `0.4.0`, before that binding existed, and therefore
carries no `evaluator_evidence_path`. Measured on 2026-08-28 at `0970363`, the complete artifact graph read by
the `0.8.0` evaluator reports exactly **one error**, and it is that one:

```text
[E012] [governance] docs/engineering/simulation/releases/RLS-MOK-001.md:
       field 'evaluator_evidence_path' must be a non-empty string
```

The consequence is not cosmetic. `harnessctl transition` refuses to plan or apply any lifecycle transition
while the graph is invalid — it stops at `WEX201: current artifact graph is invalid [E012]` — so under a
current evaluator this repository can approve nothing, implement nothing and verify nothing. A repository
that stays where it is keeps a governance surface that works; a repository that moves without settling that
record arrives in a state where the harness refuses to operate at all. Neither is a position the owner chose,
and the second is reached by an ordinary upgrade command.

The second problem is that the first one was discovered rather than known. Nothing in this repository records
what adopting a new evaluator involves, what it is permitted to change, what it must leave alone, or what has
to be true afterwards. The next upgrade would begin with the same investigation.

## Desired outcomes

- The repository's standard root is an exact published `se_harness` release, and the complete artifact graph
  validates under that release's evaluator with **zero errors**.
- Releases cut before evaluator-evidence enforcement existed are resolved by an explicit, accountable
  declaration rather than by fabricating evidence for a commit that was never evaluated that way.
- The adoption is one bounded transaction with retained evidence, so a later reader can see which version was
  adopted, what it moved, and what it deliberately did not.
- A subsequent upgrade is a repeat of a stated procedure, not a fresh investigation.

## Actors and stakeholders

The **engineering owner** authorizes and performs the adoption. The **assurance owner** decides what evidence
the transaction must retain and whether it satisfies its verification contract. The **release owner** is
affected but does not act here: the standing release `RLS-MOK-001` is the subject of the declaration, and
nothing about it is re-decided, re-cut or re-pointed.

Every future reader of the artifact graph is a stakeholder, because the evaluator is what makes the graph
mean anything.

## Success measures

Observable after delivery, and each of them by running a command rather than by reading prose:

1. `harnessctl validate` under the adopted evaluator reports 0 errors.
2. `harnessctl doctor` under the adopted evaluator reports no FAIL.
3. The repository's own `scripts/validate_engineering_artifacts.py`, which the adoption replaces, agrees with
   the evaluator on the same tree.
4. The retained transaction evidence names the adopted version and lists what it declared.

## Non-goals

- **No simulation behavior changes.** No rule, event, stream byte, entropy draw, public item or dependency of
  `mokiterions-core` or `mokiterions-tui` moves. This intent is about the harness that governs the code, not
  the code.
- **No release.** Adopting an evaluator is not a product release and does not authorize one. `RLS-MOK-001`
  stays `released` at `0.1.0`, at the commit it already binds.
- **No re-evaluation of past work.** No existing verification record is superseded, re-pointed or re-measured
  because the evaluator moved.
- **No branch rename.** A post-adoption effect on the managed continuous-integration trigger is recorded
  rather than repaired here; see `SPEC-HUP-001`'s *Compatibility and migration*.

## Principles and immutable constraints

- **The evaluator is never the tree under test.** It is installed outside the checkout, from the public index,
  at an exact version. A checkout cannot grade itself.
- **A declaration exempts; it never writes evidence.** Resolving `RLS-MOK-001` must not add, recompute or
  repoint any field on that record. What predates enforcement is stated as predating it.
- **Managed files are the evaluator's, not this repository's.** The adoption replaces them wholesale. A
  managed file this repository has edited is a conflict to be surfaced, never a merge to be attempted.
- **Nothing is approved by implication.** The declaration only has force from a work order the engineering
  owner explicitly approved, and the approval instant is part of the record.

## Risks and assumptions

- **Assumption:** the adopted version is published on the public index and installable without a credential.
  Measured true for `0.8.0` on 2026-08-28.
- **Risk:** a managed file has been locally edited, which blocks the transaction with
  `installation has conflicts or customizations; no files were written`. Measured absent on 2026-08-28 at
  `0970363`: the plan reports 61 files, 13 unchanged, and no customization.
- **Risk:** the adoption changes managed continuous-integration policy, and a defect there is not visible
  until a later pull request runs. This is why `WO-HUP-001` classifies commit-bound verification as
  `required`.
- **Risk:** repository-owned files that name the evaluator version drift out of step with the root, which
  would leave a workflow installing one version to grade a root installed by another. `SPEC-HUP-001` rule 7
  makes moving them part of the transaction rather than a follow-up.
