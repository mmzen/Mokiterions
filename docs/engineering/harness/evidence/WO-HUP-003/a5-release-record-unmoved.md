# A5 - the declared record did not move

`git diff main -- docs/engineering/simulation/releases/RLS-MOK-001.md` on 2026-09-15:

```text
(empty: no difference of any kind)
```

Blob at `main`: `e0301c38eea1e0682ad4cc8572d4fb3efb08a4b5`. Blob in the working tree: `e0301c38eea1e0682ad4cc8572d4fb3efb08a4b5`. Equal.

The 0.18.0 transaction read no `[evaluator_upgrade]` declaration and refused nothing on this record: `upgrade-transaction.json` carries no `legacy_releases_without_evaluator_evidence` field and its `work_order` and `authorization_path` are null. `WO-HUP-001`'s declaration stands where it was written. `RLS-MOK-001` validates under 0.18.0 with 0 errors as part of A1. So `SPEC-HUP-001` rules 4 to 6 had nothing to act on, `VER-HUP-001` A3 has no refusal to observe and N1 no resolution to read, and this file records that rather than reconstructing either.
