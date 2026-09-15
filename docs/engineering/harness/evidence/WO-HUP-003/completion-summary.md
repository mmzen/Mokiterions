# WO-HUP-003 completion summary

The standard root moved from `se_harness` 0.8.0, adopted 2026-08-28 under `WO-HUP-001`, to exact public
**0.18.0**, on 2026-09-15. Skill ownership is recorded as `plugin`. Every already-approved work order starts.
This summary follows the work order's *Completion report format*.

## 1. The adopted version, the wheel, and the evaluator it was run from

Exact public **0.18.0**. The evaluator was installed from the wheel `se_harness-0.18.0-py3-none-any.whl`,
SHA-256 `a683dbdf485d42aa20ea8502122c171a4c61c7d60f85db5b5f264bd336371c54`, which equals the digest of the
same wheel downloaded independently from the public index on 2026-09-15. It lives in the host's plugin data
directory at `verity-plane/evaluator`, **outside this checkout**, and was invoked as `python -I -m se_harness`
throughout. Its identity reading passes with `isolated_python` true and the archive digest above; the lock
now records that digest, which an index install would have left null. Retained as `identity.md`.

## 2. Both plans as measured

| | Transaction 1 (adopt) | Transaction 2 (replace) |
|---|---|---|
| Files in the plan | 64 | 41 |
| Unchanged | 3 | 22 |
| Adopt (managed to seed, bytes unchanged) | 28 | — |
| Remove | 23 | — |
| Update | 7 | 19 |
| Add | 3 | — |
| Customized or conflict | 0 | 0 |
| Postconditions | all true | all true |

Each was planned read-only and applied on the same tree, which is `SPEC-HUP-001` rule 3. Retained as
`a2-plan.md`, `a2-replace-plan.md`, `upgrade-transaction.json` and `replace-transaction.json`. The 19 replaced
seeds and why each was named are in `replace-list.md`: every one equalled its 0.8.0 template, so none carried a
repository edit.

**Two transactions rather than one is the evaluator's rule, not a choice.** `--replace-file` accepts only a
path the lock already calls a seed, and the first transaction is what makes the 28 adopted files seeds.

## 3. Skill ownership

`skill-ownership --provider plugin --plugin-root <installed verity-plane plugin> --apply`, run after both
transactions: `.agents/skills/harness-orient/`, `.agents/skills/harness-operator-brief/` and
`.claude/skills/harness-orient/` removed; the lock rewritten to **schema 4** with
`skill_ownership.provider = "plugin"`. The lock now holds 34 files: 3 managed, 4 fragment, 27 seed. Retained as
`skill-ownership.md`.

## 4. Validate and doctor, before and after

| Reading | Before (0.18.0 over the 0.8.0 root) | After all three steps |
|---|---|---|
| `validate` | 211 artifacts, 0 errors, 0 warnings, 0 advisories | 212 artifacts, 0 errors, 0 warnings, 0 advisories |
| `doctor` | 93 checks, **63 FAIL** | 64 checks, **0 FAIL** |

The 212th artifact is this work order. Under the 0.8.0 evaluator the same tree read 0 errors and 155
`W-AUT-*` warnings; 0.18.0 reports none. `REQ-HUP-001`'s measure is the error count, 0. Retained as
`pre-doctor.md`, `a1-validate.md` and `n2-doctor.md`.

## 5. The rule 11 enumeration

One work order is in an authority-granting state without having reached `implemented`: `WO-MOK-027`.

| Moment | Reading |
|---|---|
| Before the transaction | `QGP-G3-SCOPE: WEX-ECP-022: no recorded engineering-owner approval`; `QGP-G3-PREFLIGHT: mode 'managed'; expected 'seed'` |
| After the transactions and the skill switch | `QGP-G3-SCOPE: WEX-ECP-022: no recorded engineering-owner approval` |
| After the repair | **Completed**, all six gates pass |

The refusal that survives the adoption is the adoption's: 0.18.0 reads the execution grant from a scoped
approval event and `WO-MOK-027`, approved on 2026-08-23 under 0.4.0, had no event. The repair is the owner's
decision of 2026-09-15 — one new event dated at that decision, decided by `engineering-owner`, carrying the
five current scope paths, with an amendment record — and not a back-dated one: the adopted
`DECISION_RIGHTS.md` says historical records are not rewritten and an older approval needs owner approval of
remaining execution through amendment. Refusals after: **0**, which is `REQ-HUP-002`'s measure. Retained as
`b1-start-before.md`, `b2-start-after.md` and `b3-p1-p2-wo-mok-027.md`.

## 6. The change surface

| Protected path | Files changed |
|---|---|
| `mokiterions-core/` | 0 |
| `mokiterions-tui/` | 0 |
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | 0 |
| `docs/engineering/simulation/releases/RLS-MOK-001.md` | 0 (blob unchanged) |

80 files changed against `main` before the evidence packet: 20 added, 30 deleted, 30 modified. The three
hand edits are `.github/workflows/release.yml` (`SE_HARNESS_VERSION` to `0.18.0`; `validate .` and
`dashboard .` in place of two removed scripts), `docs/RELEASE_RUNBOOK.md` (the same replacement), and
`docs/engineering/simulation/work-orders/WO-MOK-027.md` (item 5). The scope checkpoint against `main` passes
with every changed path admitted. Retained as `a5-release-record-unmoved.md`, `a6-version-references.md` and
`a7-no-product-effect.md`.

The seven repository-owned gate suites under `scripts/test_*.py` — release authorization, release
reachability, workflow credentials, transcript reading, declared dependencies, the sweep driver and the
classifier — were run after the eight harness scripts were removed. All seven pass.

## 7. What A3, N1 and A4 could not observe

- **A3 and N1.** The 0.18.0 transaction reads no `[evaluator_upgrade]` declaration: its evidence carries no
  legacy-release field and refuses nothing on `RLS-MOK-001`. There was no refusal to observe before approval
  and no exemption set to read afterwards. `RLS-MOK-001` validates with 0 errors under A1 and is byte-identical
  under A5.
- **A4.** The adopted root contains no in-tree validator to agree with the external one;
  `scripts/validate_engineering_artifacts.py` is one of the eight files the transaction removes, and 0.18.0
  ships its validator only inside the evaluator. The assessment cannot be executed and `VER-HUP-001` is not
  amended here, because a verified terminal record attests it. The managed workflow's `qualify released-root`
  is the independent reading a later reader has instead.

## 8. Disclosures owed to the owner

- `SPEC-HUP-001`'s *Compatibility and migration* still describes a `master` default branch. The branch was
  renamed `main` on 2026-08-28 and the managed lane's push trigger now matches it; the paragraph is stale.
- `SPEC-HUP-001`'s *Actors* section says the public index supplies the evaluator. This adoption installed
  from the exact public wheel file, so that the lock and the runtime identity can bind the archive digest; an
  index install leaves both null. The wheel is the index's, byte for byte, but the sentence describes the
  older practice.
- `docs/engineering/README.md` lists the `simulation` domain only. The `harness` domain has existed since
  2026-08-28 and is not in the index. Pre-existing, and outside this work order.
- Under the 0.8.0 evaluator this tree reports 155 authoring advisories on requirements; under 0.18.0 it
  reports 0. The requirements did not change. Whoever next reads a warning count should say which evaluator
  produced it.

## 9. What is left owed

- The commit-bound verification record, prepared with `capture-verification` on the clean candidate and
  decided by the assurance owner.
- Native discovery of the plugin's `harness-orient` and `harness-operator-brief` skills, which only the host
  can show; `doctor` passing does not prove it.
