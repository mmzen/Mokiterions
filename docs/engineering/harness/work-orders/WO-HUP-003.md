+++
id = "WO-HUP-003"
type = "work_order"
title = "Adopt exact public se_harness 0.18.0 as the standard root, replace the inherited guides and templates, move skill ownership to the verity-plane plugin, and re-check every approved work order"
status = "in_progress"
owners = ["engineering owner"]
created = "2026-09-15"
updated = "2026-09-15"

[assurance]
commit_bound_verification = "required"
rationale = "This work replaces the managed policy, the machine workflow and gate contracts, the artifact templates, the managed continuous-integration workflow and the agent instruction fragments that every later engineering, assurance and release decision in this repository is read through, and it removes the in-tree harness scripts that the release workflow still calls. Four claims cannot be settled by reading the diff. That the complete artifact graph validates with zero errors under the adopted evaluator is a claim about 211 artifacts read by an evaluator outside the tree. That every already-approved work order is startable afterwards is a claim about an enumerated set evaluated at a checkpoint, and the one member of that set is refused today for a reason only the adopted evaluator can express. That no product behavior moved is a claim about the whole package tree. That the release workflow still runs after the scripts it called are gone is a claim about a file this work order edits by hand beside a transaction it does not control. Only an external evaluator bound to an exact commit can settle whether the result is sound, and the transaction replaces the very instrument a later reader would reach for."
decided_by = "engineering owner"

[execution_scope]
paths = [
  ".agents/",
  ".claude/",
  ".engineering-harness.lock",
  ".engineering-harness.toml",
  ".github/workflows/engineering-harness.yml",
  ".github/workflows/release.yml",
  ".gitignore",
  "AGENTS.md",
  "CLAUDE.md",
  "ENGINEERING_HARNESS.md",
  "GLOSSARY.md",
  "docs/RELEASE_RUNBOOK.md",
  "docs/engineering/ARTIFACT_AUTHORING.md",
  "docs/engineering/DECISION_RIGHTS.md",
  "docs/engineering/OPERATING_CARD.md",
  "docs/engineering/QUALITY_GATES.json",
  "docs/engineering/QUALITY_GATES.md",
  "docs/engineering/TECHNICAL_COMMUNICATION.md",
  "docs/engineering/TRACEABILITY.md",
  "docs/engineering/WORKFLOW.json",
  "docs/engineering/WORKFLOW.md",
  "docs/engineering/harness/",
  "docs/engineering/simulation/work-orders/WO-MOK-027.md",
  "docs/engineering/templates/",
  "scripts/artifact_layout_registry.py",
  "scripts/check_engineering_harness.ps1",
  "scripts/check_engineering_harness.sh",
  "scripts/generate_harness_dashboard.py",
  "scripts/harness_explorer/",
  "scripts/inspect_engineering_artifacts.py",
  "scripts/select_harness_work_order.py",
  "scripts/validate_engineering_artifacts.py",
]

[relations]
implements = ["REQ-HUP-001", "REQ-HUP-002"]
specifications = ["SPEC-HUP-001"]
verification = ["VER-HUP-001", "VER-HUP-002"]
[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-09-15T18:42:41Z"
decided_by = "engineering-owner"
reason = "Approved by the repository owner acting as engineering owner on 2026-09-15, by selecting the presented option 'Approve as drafted'. In the same act the owner selected the replacement of the 19 inherited seeds, the repair of WO-MOK-027 inside this scope, and push plus one ready pull request as the authorized delivery action. Recorded by hand in the adopted evaluator's shape: the 0.18.0 evaluator refuses to mutate a 0.8.0 root (MG005 RID002) and the 0.8.0 evaluator writes no scope_paths."
scope_paths = [".agents/", ".claude/", ".engineering-harness.lock", ".engineering-harness.toml", ".github/workflows/engineering-harness.yml", ".github/workflows/release.yml", ".gitignore", "AGENTS.md", "CLAUDE.md", "ENGINEERING_HARNESS.md", "GLOSSARY.md", "docs/RELEASE_RUNBOOK.md", "docs/engineering/ARTIFACT_AUTHORING.md", "docs/engineering/DECISION_RIGHTS.md", "docs/engineering/OPERATING_CARD.md", "docs/engineering/QUALITY_GATES.json", "docs/engineering/QUALITY_GATES.md", "docs/engineering/TECHNICAL_COMMUNICATION.md", "docs/engineering/TRACEABILITY.md", "docs/engineering/WORKFLOW.json", "docs/engineering/WORKFLOW.md", "docs/engineering/harness/", "docs/engineering/simulation/work-orders/WO-MOK-027.md", "docs/engineering/templates/", "scripts/artifact_layout_registry.py", "scripts/check_engineering_harness.ps1", "scripts/check_engineering_harness.sh", "scripts/generate_harness_dashboard.py", "scripts/harness_explorer/", "scripts/inspect_engineering_artifacts.py", "scripts/select_harness_work_order.py", "scripts/validate_engineering_artifacts.py"]

[[lifecycle_events]]
from = "approved"
to = "in_progress"
decided_at = "2026-09-15T18:45:24Z"
decided_by = "engineering-owner"
reason = "Execution of DR-WO-START under recorded engineering-owner approval; relevant local gates passed."
+++

# Work Order: adopt exact public se_harness 0.18.0

## Lifecycle

Governance work. `approved` authorizes the transactions below and the hand edits beside them;
`implemented` follows the applied transactions and the retained evidence. Under the adopted root the
move from `approved` to `implemented` runs through `in_progress`. Commit-bound verification is
`required`, so a verification record binding an exact candidate commit is a separate, later act and is
not authorized by this work order.

The approval is recorded in the shape the adopted evaluator reads: a `draft` to `approved` event
decided by `engineering-owner` that carries `scope_paths` equal to `[execution_scope].paths`. The 0.8.0
evaluator cannot write that shape and the 0.18.0 evaluator refuses to mutate a 0.8.0 root — measured on
2026-09-15 as `MG005 (delegated-work-order-start): RID002 harness_version: resolved '0.18.0'; expected
'0.8.0'` — so the approval event is written by hand, as `WO-HUP-001`'s was, quoting the owner's
selection. Every lifecycle transition after the transaction is applied by the adopted evaluator.

## Objective

Move this repository's standard root from `se_harness` **0.8.0**, adopted 2026-08-28 under `WO-HUP-001`,
to exact public **0.18.0**; give the owner-editable guides and templates the 0.18.0 text they would have
had under a fresh installation, since none of them carries a repository edit; record that the
`harness-orient` and `harness-operator-brief` skills are provided by the `verity-plane` plugin rather than
by generated copies in the tree; keep the repository-owned release workflow and runbook working after the
in-tree harness scripts they call are removed; and leave the complete artifact graph validating with zero
errors and every already-approved work order startable under the adopted evaluator.

## Why this is more than `WO-HUP-001` at a different version

`SPEC-HUP-001` says the next adoption is a repeat of the same transaction, and the transaction is. Three
things around it are not, and each was found by rehearsing the adoption in a disposable worktree at
`8ab8a30` on 2026-09-15 rather than by reading release notes.

1. **Most managed files become owner-editable seeds, and the transaction keeps their old text.** The
   0.18.0 lock has three modes: `managed` for the router and the two machine contracts, `fragment` for the
   four instruction and ignore fragments, and `seed` for everything else — the human guides, the fourteen
   templates, the configuration, the continuous-integration workflow. The plan reports 28 files as `adopt`:
   their mode moves from `managed` to `seed` and their bytes do not change. So after the transaction alone
   `docs/engineering/DECISION_RIGHTS.md` lacks the `approved-execution` section the new work-order template
   points at, `WORKFLOW.md` explains a `WORKFLOW.json` it no longer matches, `create-artifact` produces a
   draft from the 0.8.0 template, and the managed workflow still installs `se-harness==0.8.0`. The evaluator's
   own remedy is `upgrade --replace-file PATH` per seeded file, which it accepts only once the file *is* a
   seed — measured: naming an adopted path in the first run refuses with `--replace-file must name a seeded
   file`. The adoption is therefore two transactions, and the second is planned before it is applied like the
   first.

   Every one of the 28 adopted files is byte-identical to its 0.8.0 distribution template, so none carries a
   repository edit and replacing it loses nothing. 21 differ from their 0.18.0 template; 2 of those are the
   `harness-orient` skill files item 3 removes, which leaves **19** to replace.

2. **Skill ownership is a recorded choice, and the plugin route removes generated copies.** 0.18.0
   introduces `skill-ownership`, which records in the lock whether `harness-orient` and
   `harness-operator-brief` live in the tree or in an installed plugin. Selecting the plugin removes
   `.agents/skills/harness-orient/`, `.agents/skills/harness-operator-brief/` and
   `.claude/skills/harness-orient/` and rewrites the lock to schema 4 with `skill_ownership.provider =
   "plugin"`. The three other skill trees of 0.8.0 — `harness-draft-change`, `harness-execute-work-order`,
   `harness-prepare-assurance` — are not in the 0.18.0 standard template at all and are removed by the first
   transaction whatever the ownership choice. The repository stores the provider, never the plugin's machine
   path; an ordinary clone runs every evaluator command without a plugin installed.

3. **The transaction removes eight in-tree scripts, and two repository-owned files call two of them.**
   0.18.0's standard template has no `scripts/` entry: `validate_engineering_artifacts.py`,
   `generate_harness_dashboard.py`, `inspect_engineering_artifacts.py`, `select_harness_work_order.py`,
   `artifact_layout_registry.py`, `check_engineering_harness.sh`, `check_engineering_harness.ps1` and
   `harness_explorer/index.template.html` are all `remove`. `.github/workflows/release.yml` runs the first
   two at lines 207 and 231, and `docs/RELEASE_RUNBOOK.md` names the first at line 165. Both are
   repository-owned, so the transaction does not touch them and this work order does, under `SPEC-HUP-001`
   rule 7's reasoning: a repository-owned file that names the evaluator's surface moves with the root. The
   evaluator's own `validate` and `dashboard` commands replace the two calls; `dashboard` writes to
   `target/harness-dashboard`, which is the path the workflow already uploads. The seven repository-owned
   scripts under `scripts/` that carry this project's release, dependency, credential, transcript and sweep
   checks import none of the removed files and are untouched.

## In scope

1. **The adoption transaction**, planned and then applied by the 0.18.0 evaluator installed from the exact
   public wheel into an environment outside this checkout, with its evidence written to
   `docs/engineering/harness/evidence/WO-HUP-003/upgrade-transaction.json`. Measured plan at `8ab8a30`:
   **64 files — 28 adopt, 23 remove, 7 update, 3 add, 3 unchanged; no customization, no conflict.**
2. **The replacement transaction**, `upgrade --replace-file` for each of the 19 adopted seeds that differ
   from their 0.18.0 template, planned and then applied, with its evidence written to
   `.../WO-HUP-003/replace-transaction.json`. Measured plan: **19 update, 22 unchanged.** The list is
   retained as evidence and is exactly: the managed workflow, the seven guides under `docs/engineering/`
   that differ, and eleven templates. `TECHNICAL_COMMUNICATION.md`, `OPERATING_CONTRACT.template.md` and
   the two `harness-operator-brief` files already equal their 0.18.0 text and are not named.
3. **Skill ownership recorded as `plugin`**, through `skill-ownership --provider plugin --plugin-root
   <installed verity-plane plugin> --apply`, after the transactions and never before them — measured: the
   command accepts a 0.8.0 root and would strip its skills while the version still reads 0.8.0.
4. **The repository-owned references**, edited by hand in the same change: `.github/workflows/release.yml`
   moves `SE_HARNESS_VERSION` from `0.8.0` to `0.18.0` and replaces the two script invocations with the
   evaluator's `validate .` and `dashboard .`; `docs/RELEASE_RUNBOOK.md` replaces its one script invocation
   the same way. Nothing else in either file moves.
5. **`SPEC-HUP-001` rule 11**, performed rather than assumed: the `start` checkpoint of every work order in
   an authority-granting state that has not reached `implemented` is evaluated under the adopted evaluator
   before and after the transaction. The set has **one** member, `WO-MOK-027`, and it is refused before
   *and* after: `QGP-G3-SCOPE: WEX-ECP-022: WO-MOK-027.md has no recorded engineering-owner approval`. The
   cause is the adoption's: 0.18.0's execution grant is the approval event itself, read as `to =
   "approved"`, `decided_by = "engineering-owner"` and `scope_paths`, and `WO-MOK-027` — approved on
   2026-08-23 under 0.4.0, which wrote no lifecycle events — has none. `DECISION_RIGHTS.md#approved-execution`
   as adopted says what repairs it: *an older approval without that grant requires owner approval of
   remaining execution through the existing amendment process*, and *neither current metadata nor an actor
   label can manufacture a missing historical grant*.

   **The repair is the owner's act and is in this work order's scope on the owner's decision of
   2026-09-15**: one new `[[lifecycle_events]]` table on `WO-MOK-027`, from `draft` to `approved` because
   that is the only edge into `approved` the adopted evaluator admits — measured: an `approved` to
   `approved` event is `E014: unsupported transition` — dated at the owner's decision and not at 2026-08-23,
   decided by `engineering-owner`, carrying the work order's five current `scope_paths`, with a `reason`
   stating that it approves remaining execution of a scope the 2026-08-23 approval predates; plus an
   `## Amendment record` entry and the `updated` field. No other field of `WO-MOK-027` moves. Measured with
   the correct paths in the rehearsal: `validate` 0 errors and the start checkpoint passes.
6. **The evidence `VER-HUP-001` and `VER-HUP-002` contract**, as *Evidence to record* enumerates, plus the
   evaluator's identity reading for the environment that produced every figure.

## Out of scope

- **Every product change.** No file under `mokiterions-core/` or `mokiterions-tui/`, and no `Cargo.toml`,
  `Cargo.lock` or `rust-toolchain.toml`, is touched. `SPEC-HUP-001` rule 10; `VER-HUP-001` A7 checks it.
- **Any release act.** `RLS-MOK-001` stays `released` at `0.1.0`, byte-identical. The 0.18.0 transaction
  reads no `[evaluator_upgrade]` declaration and refuses nothing on that record; `WO-HUP-001`'s declaration
  stands where it was written and is neither repeated nor removed here.
- **Editing any locked file by hand.** `ENGINEERING_HARNESS.md`, `QUALITY_GATES.json`, `WORKFLOW.json`
  and the four fragments are written by the transaction only.
- **Replacing an owner-edited seed.** `docs/engineering/README.md`, `.github/PULL_REQUEST_TEMPLATE.md`,
  `.engineering-harness.toml` and `GLOSSARY.md` carry or will carry repository content and are not named
  to `--replace-file`. The configuration keeps its 0.8.0 keys; the transaction moves `tool_version` alone.
- **`WO-MOK-027`'s work, and every other field of it.** Item 5 adds one event and one amendment row. It
  starts nothing and lifts none of that work order's own gates.
- **Amending `VER-HUP-001`.** Its assessment A4 asks the in-tree validator to agree with the external one,
  and the adopted root has no in-tree validator. A4 is recorded as not executable at 0.18.0, with the reason,
  and the contract is not edited: it is attested by a verified, terminal record.
- **Amending `SPEC-HUP-001`.** Its *Compatibility and migration* section still describes a `master` default
  branch that was renamed `main` on 2026-08-28, and its *Actors* section describes an index install where
  this adoption installs from the exact wheel so that the lock can bind the archive digest. Both are
  disclosed in the completion summary and owed to the owner, not settled here.
- **The `docs/engineering/README.md` index**, which does not list the `harness` domain. Pre-existing, and
  not this transaction's.
- **Any verification record.** `required` assurance is discharged separately.
- **Pushing or opening a pull request**, unless the decision envelope below names it.

## Authorized decision envelope

The engineering owner may, without further authorization: choose the exact evidence paths below
`docs/engineering/harness/evidence/WO-HUP-003/`; re-run either plan and apply on a settled tree after a
refusal; word the retained summaries and the two amendment records; commit on the branch
`harness/wo-hup-003-adopt-se-harness-0.18.0`; and, once the work order is `implemented` and its verification
record is prepared, push that branch to `origin` and open **one** ready pull request against `main` carrying
the `Harness-Work-Order: WO-HUP-003` trailer — the delivery action the owner selected on 2026-09-15.

The engineering owner may **not**, under this work order: adopt a version other than exact public
`0.18.0`; install the evaluator from anywhere but the wheel whose SHA-256 is
`a683dbdf485d42aa20ea8502122c171a4c61c7d60f85db5b5f264bd336371c54`, which is the public index's; name a
seed to `--replace-file` that is not byte-identical to its 0.8.0 template; edit a locked file by hand;
select `repository` as the skill provider; change any field of `WO-MOK-027` beyond item 5; amend any other
artifact; open a second pull request; or merge or tag. Each of those is a fresh decision.

## Constraints

- `SPEC-HUP-001` rules 1 to 11 govern the transactions in full. Rules 4 to 6 have nothing to act on at
  0.18.0 and that fact is recorded rather than assumed.
- The evaluator is 0.18.0 installed from the exact public wheel into
  `<plugin data root>/verity-plane/evaluator`, outside the checkout, and is invoked as `python -I -m
  se_harness`. Its identity reading — role `released-evaluator`, `isolated_python` true, archive digest
  equal to the wheel's — is retained. No in-tree copy of the evaluator exists after the transaction, and
  none is used before it.
- Order is fixed: adoption transaction, replacement transaction, skill ownership, hand edits, readings.
  The two transactions are each planned before they are applied.
- Every figure quoted here was measured at `8ab8a30`. The pre-transaction readings are re-taken at the
  commit the transaction actually runs on, and the retained evidence carries those.

## Expected change surface

Measured on 2026-09-15 at `8ab8a30` by rehearsing all three steps in a disposable worktree.

- **Transaction 1 (64 files):** `adopt` 28 — the two remaining skill trees, the managed workflow, seven
  guides under `docs/engineering/`, fourteen templates and `templates/README.md`; `remove` 23 — the three
  retired skill trees (15 files) and the eight scripts; `update` 7 — `.engineering-harness.toml`
  (`tool_version` only), `.gitignore` (fragment markers), `AGENTS.md`, `CLAUDE.md`, `ENGINEERING_HARNESS.md`,
  `QUALITY_GATES.json`, `WORKFLOW.json`; `add` 3 — `GLOSSARY.md`, `templates/DECISION.template.md`,
  `templates/RISK.template.md`; `unchanged` 3. The lock moves to schema 3 at 0.18.0 with the wheel's
  `archive_name` and `archive_sha256` recorded, which an index install would leave null.
- **Transaction 2 (19 files):** `update` for each named seed; the managed workflow's
  `SE_HARNESS_VERSION` becomes `0.18.0` by this step and no other.
- **Skill ownership:** three directories removed, the lock rewritten to schema 4 with
  `skill_ownership.provider = "plugin"`. Afterwards `.agents/skills/` and `.claude/skills/` are empty.
- **Hand edits (3 files):** `.github/workflows/release.yml`, `docs/RELEASE_RUNBOOK.md`,
  `docs/engineering/simulation/work-orders/WO-MOK-027.md`.
- **New artifacts:** this work order and its evidence directory.
- **Readings:** `doctor` moves from 63 FAIL of 93 checks before the transaction to 0 FAIL of 64 after the
  three steps; `validate` reads 211 artifacts, 0 errors, 0 warnings, 0 advisories before and after.

## Required verification

`VER-HUP-001`: A1, A2, A5, A6, A7, N2, S1 and M1 in full. A3 and N1 concern a declaration the 0.18.0
transaction does not read; each is recorded as having no refusal and no resolution to observe, with the
transaction evidence as the proof. A4 is recorded as not executable, because the adopted root contains no
in-tree validator. A1 carries `REQ-HUP-001`.

`VER-HUP-002`: B1, B2, B4 and B5 in full, over the enumerated set of item 5. B1 requires the refusal to be
captured **before** the transaction and again immediately after it, so that the reading which shows the
refusal surviving the adoption exists before the repair removes it. B3, P1 and P2 are read against item 5's
edit: the only governed artifact touched outside the `harness` domain is `WO-MOK-027`, it gains one event
and one amendment row, and its `scope_paths` equal its unchanged `[execution_scope]`. B2 carries
`REQ-HUP-002`.

## Evidence to record

Under `docs/engineering/harness/evidence/WO-HUP-003/`:

- `pre-doctor.md` — the 0.18.0 evaluator's `doctor` over the 0.8.0 root, 63 FAIL, so the distance is
  measured before it is closed;
- `a2-plan.md` and `upgrade-transaction.json` — transaction 1's plan and applied evidence (A2, A3, N1, S1);
- `a2-replace-plan.md`, `replace-list.md` and `replace-transaction.json` — transaction 2's plan, the
  named list with each file's 0.8.0 and 0.18.0 template equality, and its applied evidence (A2);
- `skill-ownership.md` — the plan and the applied result;
- `a1-validate.md` — `validate` after all three steps, with the four counts (A1, B5);
- `n2-doctor.md` — `doctor` after all three steps (N2);
- `a5-release-record-unmoved.md`, `a6-version-references.md`, `a7-no-product-effect.md` — as named;
- `b1-start-before.md` and `b2-start-after.md` — the enumerated set and each member's start-checkpoint
  reading before the transaction, after it, and after the repair (B1, B2);
- `b3-p1-p2-wo-mok-027.md` — the diff of `WO-MOK-027` and the scope equality (B3, P1, P2);
- `identity.md` — the evaluator's runtime identity reading;
- `handoff-check.md` — the handoff checkpoint at the digest fixed point;
- `completion-summary.md` — following the format below.

## Stop and escalate conditions

Stop and return to the engineering owner when:

- either plan reports any path as `customized` or `conflict`, or a `--replace-file` target is not
  byte-identical to its 0.8.0 template;
- either transaction refuses for any reason, or its evidence reports a postcondition false;
- `validate` reports any error after any step, or `doctor` any FAIL after the third;
- the rule 11 enumeration finds a member other than `WO-MOK-027`, or a refusal survives item 5's repair, or
  a new refusal appears at a later checkpoint;
- `RLS-MOK-001` shows any difference at all;
- the identity reading of the evaluator does not pass, or its archive digest is not the wheel's;
- satisfying a refusal would require editing a locked file, changing a field this work order does not
  authorize, or adopting a different version.

## Completion report format

1. The adopted version, the wheel digest, and the evaluator installation and identity it was run from.
2. Both plans as measured: files, unchanged, add or update or remove, customization; the replacement list.
3. The skill-ownership result and the lock's schema and provider.
4. `validate` and `doctor` before and after, with counts.
5. The rule 11 enumeration: each member's reading before the transaction, after it, and after the repair.
6. The change surface, against the out-of-scope directories, and the three hand edits.
7. What A3, N1 and A4 could not observe, and why.
8. The two disclosures owed to the owner on `SPEC-HUP-001`, and the `README.md` index gap.
9. What is left owed: the commit-bound verification record, and the plugin's native discovery, which only
   the host can show.
