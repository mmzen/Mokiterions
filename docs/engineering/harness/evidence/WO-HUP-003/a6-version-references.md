# A6 - no version reference is left behind

Search for the superseded version over tracked files, excluding retained evidence, on 2026-09-15 after every edit:

    git grep -n -E '\b0\.8\.0\b' -- . ':!docs/engineering/*/evidence/**'

```text
docs/ROADMAP.md:34:  **The standard root moved from `se_harness` 0.4.0 to exact public 0.8.0**, in the same pull request under
docs/ROADMAP.md:36:  by a 0.8.0 one. The artifact graph validates with **0 errors** under it, and with **142 warnings**, all at
docs/ROADMAP.md:1282:| `WO-HUP-001` | The standard root moved from `se_harness` 0.4.0 to exact public 0.8.0, declaring `RLS-MOK-001` as a release that predates evaluator evidence | `REQ-HUP-001` → `CAP-HUP-001` → `INT-HUP-001`, in the `harness` domain | `VREC-HUP-001` at `3b826f5` | **no** |
docs/engineering/harness/capabilities/CAP-HUP-001.md:74:  work could still begin, and the 0.8.0 adoption satisfied the first while breaking the second.)*
docs/engineering/harness/intent/INT-HUP-001.md:24:since. Five releases have shipped in the interval — `0.5.0`, `0.6.0`, `0.7.0`, `0.7.1` and `0.8.0` — and the
docs/engineering/harness/intent/INT-HUP-001.md:30:the `0.8.0` evaluator reports exactly **one error**, and it is that one:
docs/engineering/harness/intent/INT-HUP-001.md:104:  Measured true for `0.8.0` on 2026-08-28.
docs/engineering/harness/requirements/REQ-HUP-001.md:34:That is not a hypothetical for this repository. Measured on 2026-08-28 at `0970363`, the `0.8.0` evaluator
docs/engineering/harness/requirements/REQ-HUP-001.md:43:Warnings are deliberately not part of this obligation. The `0.8.0` evaluator reports 141 authoring advisories
docs/engineering/harness/requirements/REQ-HUP-001.md:70:The engineering owner adopts exact public `0.8.0`. `RLS-MOK-001` is declared under the authorizing work
docs/engineering/harness/requirements/REQ-HUP-001.md:77:The engineering owner adopts exact public `0.8.0` **without** declaring `RLS-MOK-001`. The transaction is
docs/engineering/harness/requirements/REQ-HUP-002.md:37:Measured on 2026-08-28 at `330c086`, immediately after the 0.8.0 adoption merged, the repository
docs/engineering/harness/requirements/REQ-HUP-002.md:47:`[execution_scope]` table. 0.8.0 requires one to start work and enforces it at the `start` checkpoint.
docs/engineering/harness/requirements/REQ-HUP-002.md:74:  0.8.0 does: `check --artifact <id> --checkpoint start` is read-only.
docs/engineering/harness/requirements/REQ-HUP-002.md:90:The 0.4.0 to 0.8.0 adoption of `WO-HUP-001`. `validate` reported 0 errors, `doctor` 0 FAIL, and every
docs/engineering/harness/specifications/SPEC-HUP-001.md:137:The lock is the adopted evaluator's, in its own schema. At `0.8.0` it is schema 3 and records the evaluator's
docs/engineering/harness/specifications/SPEC-HUP-001.md:164:**The managed continuous-integration trigger does not match this repository's default branch.** The `0.8.0`
docs/engineering/harness/specifications/SPEC-HUP-001.md:181:**Example.** Exact public `0.8.0` is named in `WO-HUP-001`; `RLS-MOK-001` is declared; the plan reads 61
docs/engineering/harness/specifications/SPEC-HUP-001.md:182:files, 13 unchanged, no customization; the transaction applies; the lock becomes schema 3 at `0.8.0`;
docs/engineering/harness/specifications/SPEC-HUP-001.md:207:The 0.4.0-to-0.8.0 adoption governed by this specification reported complete success: `validate` 0 errors,
docs/engineering/harness/verification-records/VREC-HUP-001.md:39:stands. The transition was taken with the released 0.8.0 evaluator through `python -I -m se_harness`,
docs/engineering/harness/verification-records/VREC-HUP-001.md:51:| `validate`, released 0.8.0, isolated | exit 0, **0 errors** |
docs/engineering/harness/verification-records/VREC-HUP-001.md:91:   as though `implemented` is one move from `approved`. Under 0.8.0 it is two, through `in_progress`,
docs/engineering/harness/verification-records/VREC-HUP-001.md:104:neither has occurred: nothing is pushed. Whether 0.8.0's continuous integration accepts this work is
docs/engineering/harness/verification-records/VREC-HUP-002.md:38:the released 0.8.0 evaluator through `python -I -m se_harness` and moved `status`, `verified_at` and
docs/engineering/harness/verification-records/VREC-HUP-002.md:58:erratum needs an additional record rather than an edit. It does not re-open the 0.8.0 adoption, whose chain
docs/engineering/harness/verification/VER-HUP-001.md:135:  evaluator accepts this repository; it does not independently assess whether `0.8.0`'s policy is correct.
docs/engineering/harness/verification/VER-HUP-002.md:110:  the 0.4.0-to-0.8.0 adoption introduced no *other* condition of the same shape — one invisible to validation
docs/engineering/harness/work-orders/WO-HUP-001.md:4:title = "Adopt exact public se_harness 0.8.0 as the standard root, declaring RLS-MOK-001 as a release that predates evaluator evidence"
docs/engineering/harness/work-orders/WO-HUP-001.md:74:# Work Order: adopt exact public se_harness 0.8.0
docs/engineering/harness/work-orders/WO-HUP-001.md:89:**0.8.0**, and leave the complete artifact graph validating with zero errors under that evaluator.
docs/engineering/harness/work-orders/WO-HUP-001.md:95:2. Applying the transaction with an evaluator installed from the public index at exact version `0.8.0`, into
docs/engineering/harness/work-orders/WO-HUP-001.md:97:3. Moving `.github/workflows/release.yml`'s `SE_HARNESS_VERSION` from `0.4.0` to `0.8.0`, under
docs/engineering/harness/work-orders/WO-HUP-001.md:110:- **The 141 authoring advisories** the `0.8.0` evaluator reports against existing requirements. They are
docs/engineering/harness/work-orders/WO-HUP-001.md:121:The engineering owner may **not**, under this work order: adopt a version other than exact public `0.8.0`;
docs/engineering/harness/work-orders/WO-HUP-001.md:150:The lock moves to schema 3 and records evaluator `0.8.0` with a null `archive_name`/`archive_sha256` pair,
docs/engineering/harness/work-orders/WO-HUP-001.md:152:field is the configuration's schema, not the lock's — and its `tool_version` becomes `0.8.0`.
docs/engineering/harness/work-orders/WO-HUP-002.md:4:title = "Give WO-MOK-026 and WO-MOK-027 the execution scope 0.8.0 requires, and oblige the next adoption to check for what froze them"
docs/engineering/harness/work-orders/WO-HUP-002.md:46:# Work Order: repair the execution scope the 0.8.0 adoption required
docs/engineering/harness/work-orders/WO-HUP-002.md:52:later act. Under 0.8.0 the move to `implemented` is two transitions, through `in_progress`.
docs/engineering/harness/work-orders/WO-HUP-002.md:56:Make every already-approved work order startable under the adopted 0.8.0 root, and state the obligation that
docs/engineering/harness/work-orders/WO-HUP-002.md:91:- **Re-opening the 0.8.0 adoption.** `WO-HUP-001` is `implemented` and its chain is verified. This is a
docs/engineering/harness/work-orders/WO-HUP-002.md:110:- The evaluator is the released 0.8.0 one, outside the checkout, invoked as `python -I -m se_harness`.
docs/engineering/harness/work-orders/WO-HUP-002.md:134:> The 0.4.0-to-0.8.0 adoption reported complete success — `validate` 0 errors, `doctor` 0 FAIL, and all eleven
docs/engineering/harness/work-orders/WO-HUP-003.md:60:reason = "Approved by the repository owner acting as engineering owner on 2026-09-15, by selecting the presented option 'Approve as drafted'. In the same act the owner selected the replacement of the 19 inherited seeds, the repair of WO-MOK-027 inside this scope, and push plus one ready pull request as the authorized delivery action. Recorded by hand in the adopted evaluator's shape: the 0.18.0 evaluator refuses to mutate a 0.8.0 root (MG005 RID002) and the 0.8.0 evaluator writes no scope_paths."
docs/engineering/harness/work-orders/WO-HUP-003.md:82:decided by `engineering-owner` that carries `scope_paths` equal to `[execution_scope].paths`. The 0.8.0
docs/engineering/harness/work-orders/WO-HUP-003.md:83:evaluator cannot write that shape and the 0.18.0 evaluator refuses to mutate a 0.8.0 root — measured on
docs/engineering/harness/work-orders/WO-HUP-003.md:85:'0.8.0'` — so the approval event is written by hand, as `WO-HUP-001`'s was, quoting the owner's
docs/engineering/harness/work-orders/WO-HUP-003.md:90:Move this repository's standard root from `se_harness` **0.8.0**, adopted 2026-08-28 under `WO-HUP-001`,
docs/engineering/harness/work-orders/WO-HUP-003.md:111:   draft from the 0.8.0 template, and the managed workflow still installs `se-harness==0.8.0`. The evaluator's
docs/engineering/harness/work-orders/WO-HUP-003.md:117:   Every one of the 28 adopted files is byte-identical to its 0.8.0 distribution template, so none carries a
docs/engineering/harness/work-orders/WO-HUP-003.md:126:   "plugin"`. The three other skill trees of 0.8.0 — `harness-draft-change`, `harness-execute-work-order`,
docs/engineering/harness/work-orders/WO-HUP-003.md:158:   command accepts a 0.8.0 root and would strip its skills while the version still reads 0.8.0.
docs/engineering/harness/work-orders/WO-HUP-003.md:160:   moves `SE_HARNESS_VERSION` from `0.8.0` to `0.18.0` and replaces the two script invocations with the
docs/engineering/harness/work-orders/WO-HUP-003.md:196:  to `--replace-file`. The configuration keeps its 0.8.0 keys; the transaction moves `tool_version` alone.
docs/engineering/harness/work-orders/WO-HUP-003.md:223:seed to `--replace-file` that is not byte-identical to its 0.8.0 template; edit a locked file by hand;
docs/engineering/harness/work-orders/WO-HUP-003.md:280:- `pre-doctor.md` — the 0.18.0 evaluator's `doctor` over the 0.8.0 root, 63 FAIL, so the distance is
docs/engineering/harness/work-orders/WO-HUP-003.md:284:  named list with each file's 0.8.0 and 0.18.0 template equality, and its applied evidence (A2);
docs/engineering/harness/work-orders/WO-HUP-003.md:301:  byte-identical to its 0.8.0 template;
docs/engineering/simulation/verification-records/VREC-MOK-025.md:49:transition was applied through the 0.8.0 evaluator rather than by hand-editing the field, so the
docs/engineering/simulation/verification-records/VREC-MOK-026.md:53:transition was applied through the 0.8.0 evaluator rather than by hand-editing the field, so the
docs/engineering/simulation/verification-records/VREC-MOK-027.md:233:was applied through the 0.8.0 evaluator rather than by hand-editing the field, so the provenance is the
docs/engineering/simulation/work-orders/WO-MOK-026.md:391:`[execution_scope]` table. The 0.8.0 root adopted under `WO-HUP-001` requires one to start work and enforces it
docs/engineering/simulation/work-orders/WO-MOK-027.md:334:table, and therefore unstartable under the 0.8.0 root with `QGP-G3-SCOPE: WO-MOK-027 has no assessable
docs/engineering/simulation/work-orders/WO-MOK-029.md:52:work order amends a document and runs nothing. Under 0.8.0 the move to `implemented` is two transitions, through
docs/engineering/simulation/work-orders/WO-MOK-030.md:54:runs nothing. Two transitions under 0.8.0, and the handoff checkpoint takes a snapshot binding whatever the
docs/engineering/simulation/work-orders/WO-MOK-031.md:53:and runs nothing. Two transitions under 0.8.0, and the handoff checkpoint takes a snapshot binding whatever the
```

Every hit is historical prose recording what an earlier root *was* or which evaluator applied an earlier transition, which `VER-HUP-001` A6 expects. None names 0.8.0 as the version to install. The four files that name a version to install now read:

```text
.github/workflows/release.yml:64:  SE_HARNESS_VERSION: "0.18.0"
.github/workflows/engineering-harness.yml:25:  SE_HARNESS_VERSION: "0.18.0"
.engineering-harness.toml:3:tool_version = "0.18.0"
ENGINEERING_HARNESS.md:3:This repository uses SE Harness 0.18.0.
```

`.github/workflows/release.yml` is repository-owned and was moved by hand under `SPEC-HUP-001` rule 7, together with its two calls into scripts the transaction removed; `docs/RELEASE_RUNBOOK.md` likewise. The other three are written by the transactions.
