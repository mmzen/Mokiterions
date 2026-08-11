# Workflow

1. Capture the problem and accountable actors in an intent.
2. Derive capabilities and observable normative requirements.
3. Specify exact behavior, architecture constraints, and independent verification.
4. Approve one bounded work order referencing the complete governing chain.
5. Run `harnessctl preflight . --work-order WO-...`, read its manifest, implement, validate, and run repository-specific quality checks.
6. Retain evidence keyed to every release-bearing work-order ID and run review preflight with `--phase review`.
7. Commit the clean final candidate source and evidence, then run `harnessctl capture-verification`; repeat `--work-order`, `--verification`, and `--evidence` for an aggregate candidate. Commit the resulting ready verification record in a later governance commit.
8. Have the assurance owner review and transition the verification record to `verified`. If a later verified or released record fully covers an older ready record, a separate governance decision may transition only that older record to `superseded`, name exactly one eligible successor through `superseded_by`, and preserve its captured provenance.
9. Run `harnessctl prepare-release`; repeat `--work-order` and `--verification-record` so released work exactly matches eligible coverage. Superseded records are historical and cannot qualify a release. Have the release owner review the prepared record, and separately create any authorized tag against the candidate commit.
10. Evaluate operating contracts through accountable humans.

Lifecycle values are `draft`, `ready`, `approved`, `in_progress`, `implemented`, `verified`, `released`, `superseded`, and `rejected`. A status change records authority; it is not a confidence estimate.

For work orders, the normal path is `draft -> approved -> in_progress -> implemented`: approval authorizes bounded execution, while implementation records completed work and retained evidence. Use work-order status `verified` or `released` only when an eligible commit-bound VREC explicitly covers that work and configured provenance requires it. Governance-only work that authorizes verification, release, tagging, review, or publication stops at `implemented` unless a distinct later VREC selects it; the status of the target VREC does not recursively verify the governance work order.

The VREC is the authoritative commit-bound assurance record: it moves separately from `ready` to `verified` through an accountable human decision. The RLS is the authoritative release record and moves separately from `ready` to `released`. Work-order status never substitutes for either record.

A record cannot contain the hash of its own commit. The verified or released candidate commit therefore precedes the later governance commit containing `VREC-*` or `RLS-*`.

Release payload is explicit. Include implementation work that the version intentionally delivers; do not automatically include publication, approval, verification-transition, or other governance-only work orders. One aggregate release remains bound to one exact final candidate commit; ancestor commits are history, not proof of final integration.

Verification supersession is explicit and human-authorized. Only a `ready` VREC may become `superseded` in the current model. It must record a UTC `superseded_at`, a non-empty `supersession_authorized_by`, and one `superseded_by` relation to a distinct `verified` or `released` VREC whose work-order set covers the old record. The old commit, object format, snapshot, evidence, work orders, and verification contracts remain unchanged. Automation may report possible stale-ready records but may not transition them.

Managed-file integrity uses schema-2 SHA-256 over the versioned `utf8-text-lf-v1` representation. LF, CRLF, and CR are equivalent line terminators; all other content distinctions remain significant. Schema-1 locks retain raw-byte semantics and migrate only when an exact legacy match or canonical equality to the rendered desired template proves the operation safe. `doctor` is read-only, and customized or ambiguous content is never overwritten.

Pull-request CI declares exactly one work order with a standalone `Harness-Work-Order: WO-...` field. CI never infers the ID from a branch, commit, or diff. Start preflight accepts `approved` or `in_progress`; review preflight additionally accepts `implemented`, `verified`, or `released` so completed work retains honest lifecycle status.
