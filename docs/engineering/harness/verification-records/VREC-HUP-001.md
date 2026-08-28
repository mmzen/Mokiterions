+++
id = "VREC-HUP-001"
type = "verification_record"
title = "Verification candidate for WO-HUP-001"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-28"
updated = "2026-08-28"
commit = "3b826f562420c6f9fc6502a92b56f0bada1ae32c"
git_object_format = "sha1"
worktree_state = "clean"
prepared_at = "2026-08-28T20:24:28Z"
prepared_by = "assurance owner"
artifact_snapshot_sha256 = "22fe3cd472460802a6c7cdb063c176a09f788c634721c90a92a2c0786e136e04"
evidence_paths = ["docs/engineering/harness/evidence/WO-HUP-001/a1-validate.md", "docs/engineering/harness/evidence/WO-HUP-001/a2-plan-declared.md", "docs/engineering/harness/evidence/WO-HUP-001/a3-undeclared-refusal.md", "docs/engineering/harness/evidence/WO-HUP-001/a4-in-tree-validator.md", "docs/engineering/harness/evidence/WO-HUP-001/a5-release-record-unmoved.md", "docs/engineering/harness/evidence/WO-HUP-001/a6-version-references.md", "docs/engineering/harness/evidence/WO-HUP-001/a7-no-product-effect.md", "docs/engineering/harness/evidence/WO-HUP-001/completion-summary.md", "docs/engineering/harness/evidence/WO-HUP-001/handoff-check.md", "docs/engineering/harness/evidence/WO-HUP-001/n1-declaration-resolution.md", "docs/engineering/harness/evidence/WO-HUP-001/n2-doctor.md", "docs/engineering/harness/evidence/WO-HUP-001/s1-no-secrets.md", "docs/engineering/harness/evidence/WO-HUP-001/transition-unblocked.md", "docs/engineering/harness/evidence/WO-HUP-001/upgrade-transaction.json"]
evaluator_evidence_path = "docs/engineering/harness/evidence/VREC-HUP-001-evaluator.json"
evaluator_evidence_sha256 = "4f500366462d5da855322aa725d6a1d23250f1ef82b37e371b43317ef81945b6"

[relations]
verifies_work_order = ["WO-HUP-001"]
conforms_to = ["VER-HUP-001"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-HUP-001` to candidate commit `3b826f562420c6f9fc6502a92b56f0bada1ae32c`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## Residual uncertainty

`VER-HUP-001`'s own residual uncertainty stands. Four further gaps were found while preparing this
record. The repository owner, as accountable assurance owner, was shown all four on 2026-08-28 and
directed that the packet be bound as it stands, with the gaps recorded here rather than repaired.

1. **A6's retained sweep is narrower than A6's wording.** The assessment says "search the repository
   for the superseded version string". The command retained in `a6-version-references.md` filtered to
   six extensions and excluded Markdown, JSON, HTML and Rust. Markdown was searched separately before
   this record was prepared and the conclusion was unchanged — every hit is either a HUP artifact
   stating history or an immutable verification record recording what an earlier root was — but that
   second search is not in the packet.

2. **Three captures were edited after capture.** `a2-plan-declared.md`, `n2-doctor.md` and
   `s1-no-secrets.md` came off a Windows console with CRLF, and the carriage returns were stripped so
   the worktree matched the blob Git would store. No digest binds those files and no `-text` rule
   covers this evidence tree, so no recorded value moves; the content is otherwise exactly as
   captured. The files do not say this happened.

3. **The two-edge lifecycle is recorded only in evidence.** `WO-HUP-001`'s *Lifecycle* section reads
   as though `implemented` is one move from `approved`. Under 0.8.0 it is two, through `in_progress`,
   which `transition-unblocked.md` records and the work order does not.

4. **`transition-unblocked.md` is claimed by no assessment.** It is the one file in the packet that
   `VER-HUP-001`'s *Evidence retention* does not list. It is supplementary rather than contracted.

None of the four falsifies a figure in this packet. A1's zero, A5's byte identity and N1's exemption
are each independent of all of them.

## What this record does not cover

The managed lane has not run against this branch. `VER-HUP-001` anticipated that its first reading
would come from a pull request rather than a push event on the default branch, and at this candidate
neither has occurred: nothing is pushed. Whether 0.8.0's continuous integration accepts this work is
therefore outside what any evidence here can show.
