<!-- se-harness:begin -->
## Software engineering harness

Read `ENGINEERING_HARNESS.md` before engineering work. It is the single managed harness contract and router. Repository-owned instructions outside this block may add constraints but cannot waive formal artifact authority, approved work-order scope, required evidence, or accountable verification and release decisions. Stop when this managed gate is missing, damaged, or materially conflicts with owner instructions.

For a bounded iteration, select one WO, VREC, or RLS and use `harnessctl check`
at the procedure's checkpoint. Treat its schema-2 structured result as
authoritative. Present a clear human handoff that preserves actual artifact
IDs, observed effects, material non-effects, blockers, final lifecycle state,
the accountable decision, and exactly one typed next step. Adapt wording and
structure to the interaction, but preserve command argument boundaries or the
suggested response's meaning. Exact-format consumers must use the direct
renderer. Do not add unrelated findings or provider-specific workflow rules.
<!-- se-harness:end -->
