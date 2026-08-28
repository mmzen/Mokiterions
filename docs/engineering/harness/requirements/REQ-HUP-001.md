+++
id = "REQ-HUP-001"
type = "requirement"
title = "An adopted evaluator validates the complete artifact graph with zero errors"
status = "approved"
owners = ["product owner"]
created = "2026-08-28"
updated = "2026-08-28"
statement = "WHEN the repository adopts an exact released evaluator version as its standard root, THE SYSTEM SHALL validate the complete artifact graph under that evaluator with zero errors."
verification_method = ["demonstration"]
priority = "must"
source = "INT-HUP-001"
measure = "0 errors reported by the adopted evaluator's validate operation over the complete artifact graph"

[relations]
derives_from = ["CAP-HUP-001"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T20:30:00Z"
decided_by = "product owner"
+++

# Requirement: an adopted evaluator validates the graph with zero errors

## Rationale

An evaluator that cannot validate the repository is worse than the one it replaced, because the harness
refuses to operate rather than operating with an older policy. `harnessctl transition` stops on
`WEX201: current artifact graph is invalid` before it plans anything, so an invalid graph withdraws every
lifecycle act at once — approval, implementation, verification and release alike.

That is not a hypothetical for this repository. Measured on 2026-08-28 at `0970363`, the `0.8.0` evaluator
reads one error against the tree, on `RLS-MOK-001`, and it is an error no artifact author introduced: the
record was cut correctly under `0.4.0`, before evaluator evidence existed. An adoption that landed and left
that error standing would leave this repository unable to approve its own repair.

So the obligation is stated at the level a reader can check in one command, and it is stated of the *complete*
graph rather than of the artifacts the adoption happens to touch. A graph is valid or it is not; there is no
useful partial state between them.

Warnings are deliberately not part of this obligation. The `0.8.0` evaluator reports 141 authoring advisories
against this tree, all of them `maintenance` severity and all of them about how existing requirements are
worded. They are real and they are someone's work, but they are not this adoption's work, and folding them in
would make a bounded transaction unbounded.

## Behavior

When an exact released evaluator version is adopted as the standard root:

- the complete artifact graph, as the adopted evaluator enumerates it, is validated by that evaluator;
- the validation reports zero entries of error severity;
- entries of warning or maintenance severity do not violate this requirement; and
- the validation is performed by an evaluator installed outside the checkout, so the tree does not grade
  itself.

## Assumptions and dependencies

- The adopted version is published and installable from the public index at an exact version.
- The repository's managed files are unmodified, so the transaction is not refused for customization.
- Releases that predate evaluator-evidence enforcement are resolvable by declaration under
  `SPEC-HUP-001` rules 4 to 6. Without that mechanism this requirement is unsatisfiable for any repository
  holding such a release, which is the condition `INT-HUP-001` records.

## Acceptance examples

### Example: normal behavior

The engineering owner adopts exact public `0.8.0`. `RLS-MOK-001` is declared under the authorizing work
order's `[evaluator_upgrade]` packet. The transaction applies 61 managed files. The adopted evaluator is then
run against the complete tree from outside the checkout and reports 0 errors and 141 maintenance warnings.
The requirement is satisfied: the count that matters is 0.

### Example: failure behavior

The engineering owner adopts exact public `0.8.0` **without** declaring `RLS-MOK-001`. The transaction is
refused before the first write, with
`released records predate evaluator-evidence enforcement and are not declared; no files were written`. Had it
instead succeeded, the adopted evaluator would report

```text
[E012] [governance] docs/engineering/simulation/releases/RLS-MOK-001.md:
       field 'evaluator_evidence_path' must be a non-empty string
```

and the requirement would be violated with exactly one error. Both outcomes are failures of the adoption; only
the second is a failure of this requirement, and the evaluator's refusal exists to keep the repository out of
it.

## Open decisions

None. The scope of the obligation — errors only, complete graph, external evaluator — is settled by this
requirement's own statement.
