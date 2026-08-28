# WO-HUP-001 completion summary

The standard root moved from `se_harness` 0.4.0, installed 2026-08-11, to exact public **0.8.0**, on
2026-08-28. This summary follows the work order's *Completion report format*.

## 1. The adopted version and the evaluator it was run from

Exact public **0.8.0**, published on the public package index. The evaluator was installed by an
unauthenticated index install into `C:/Users/mathi/se-harness-eval-080`, **outside this checkout**, and every
figure below was produced by invoking that installation with this repository as its target — `SPEC-HUP-001`
rule 2, and `VER-HUP-001`'s *Independence*.

## 2. The plan as measured

| | |
|---|---|
| Files in the plan | 61 |
| Unchanged | 13 |
| Add or update | 48 |
| Customized | 0 |
| Conflict | 0 |

Planned at `0970363` and re-derived at apply, which is `SPEC-HUP-001` rule 3. Retained as
`a2-plan-declared.md`, and as the `plan` array of `upgrade-transaction.json`.

## 3. The declaration, and the two instants

`RLS-MOK-001` was declared under `WO-HUP-001`'s `[evaluator_upgrade]` packet. The adopted evaluator resolves

```text
exemptions: {'RLS-MOK-001': 'WO-HUP-001'}
defects:    ()
undeclared: ()
```

| Fact | Value |
|---|---|
| `RLS-MOK-001` `released_at` | `2026-08-19T17:53:05Z` |
| `WO-HUP-001` draft-to-approved `decided_at` | `2026-08-28T20:30:00Z` |

The approval is later than the release, which is `SPEC-HUP-001` rule 6. Retained as
`n1-declaration-resolution.md`.

**The refusal was observed before the declaration had force**, not reconstructed afterwards: with the chain
present but unapproved, `upgrade --apply` exited `2` with `no files were written` and the working tree
unchanged. Retained as `a3-undeclared-refusal.md`.

## 4. Validate and doctor

| Assessment | Result |
|---|---|
| A1 — `harnessctl validate` | **0 errors**, 141 warnings, exit 0 |
| N2 — `harnessctl doctor` | **0 FAIL**, 143 PASS |

The 141 warnings are all `W-AUT-*` authoring advisories at `maintenance` severity, all of them pre-existing
observations about how existing requirements are worded. They are carried forward unaddressed and are outside
this transaction, which `WO-HUP-001`'s *Out of scope* states. Retained as `a1-validate.md` and `n2-doctor.md`.

## 5. The in-tree validator's agreement

`scripts/validate_engineering_artifacts.py` — the copy this transaction replaced — reports **0 errors and 141
warnings** against the same tree, agreeing with A1. It is a cross-check, never the source of the pass.
Retained as `a4-in-tree-validator.md`.

The repository's five own gate suites under `scripts/test_*.py` all pass under the adopted root.

## 6. The change surface, against out-of-scope directories

| Protected path | Files changed |
|---|---|
| `mokiterions-core/` | 0 |
| `mokiterions-tui/` | 0 |
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | 0 |
| Tags created, moved or deleted | 0 |

`docs/engineering/simulation/releases/RLS-MOK-001.md` is **byte-identical** across the whole change, blob
`e0301c38eea1e0682ad4cc8572d4fb3efb08a4b5` before and after. The declaration exempted it; nothing was written
into it. That is `SPEC-HUP-001` rule 5, and it is retained as bytes rather than asserted as prose.

`.github/workflows/release.yml`'s `SE_HARNESS_VERSION` moved from `0.4.0` to `0.8.0` under rule 7. It is
repository-owned, so the transaction did not move it and this work order did. The only remaining occurrences
of the superseded version are retained historical evidence under `WO-MOK-014`, which records what an earlier
root *was*, and a synthetic `project_name = "Fixture"` config inside
`scripts/test_check_release_authorization.py`, which constructs a temporary repository and does not read this
one. Neither is a live reference. Retained as `a5-`, `a6-` and `a7-`.

The lock moved to schema 3 recording evaluator `0.8.0`, payload `ea75cc53…`, with `archive_name` and
`archive_sha256` both null — the shape an index install produces. `.engineering-harness.toml` keeps
`schema_version = 2`, which is the configuration's schema and not the lock's, and its `tool_version` is now
`0.8.0`.

## 7. Post-adoption effect, stated as deferred

> The managed lane's push trigger fires on `main`, `release/**` and `candidate/**`. This repository's default
> branch is `master`, so after this adoption the managed lane runs on pull requests and no longer on pushes to
> the default branch. Pull-request runs are unaffected. The branch name is not settled by this work order.

The workflow is a **managed** file. Editing it to say `master` would report it customized and refuse every
subsequent adoption under `SPEC-HUP-001` rule 9, which is why the mismatch is recorded rather than repaired.
The engineering owner deferred the branch question on 2026-08-28. It is also stated in `SPEC-HUP-001`'s
*Compatibility and migration* and in that specification's *Explicitly unspecified decisions*, which is what
manual assessment M1 confirms.

## 8. What is left owed

- **The commit-bound verification record.** `WO-HUP-001` classifies commit-bound verification as `required`.
  A `VREC` binding an exact candidate commit, covering A1 through A7, N1, N2, S1 and M1 against this retained
  evidence, is a separate and later act that this work order does not authorize.
- **The first post-merge reading.** `VER-HUP-001`'s residual uncertainty records that the managed lane's
  behavior over a full merge cycle is observable only after the merge, and that the first such reading will
  come from a pull request rather than from a push event on the default branch.
- **The 141 authoring advisories**, recorded and not resolved.
- **The branch-name question**, deferred.
