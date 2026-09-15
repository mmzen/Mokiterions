# Operating card

Derived from `WORKFLOW.json` and `QUALITY_GATES.json`; `harnessctl` alone computes
legality and the next step.

## Stop when

- managed integrity fails;
- the selected graph is invalid or IDs collide;
- no phase-eligible selected work order exists;
- a required governing artifact or gate is missing;
- a required check fails;
- owner instructions conflict with the managed contract;
- remediation would exceed the selected work order;
- the action lacks its decision right or explicit authority;

Then report the failing rule, the unchanged state, and the corrective step.

## Traps

- PRs name one `Harness-Work-Order` or comma-separated `Harness-Work-Orders`; CI checks the combined scope.
- A VREC or RLS binds an earlier commit; it lives in a later governance commit and is never rewritten.
- Artifact IDs are shared across branches and sessions; check every ref before numbering.
- An unchanged rebase permits `refresh-verification --from OLD --id NEW`; changed inputs need fresh tests.
