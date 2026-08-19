# The two unexercised artifact types

`WO-MOK-009`'s approval precondition 4 says: *"The release-record and release-contract types must be
exercisable. Neither has ever been used in this repository. If `prepare-release` or `create-artifact` cannot
produce a conforming artifact for either type, that is a harness question to settle before implementation
rather than during it."*

This settles the `create-artifact` half. Measured 2026-08-19 at commit
`17be4bad444a4199da53e72ae8be491ba5f46ee1`, from the pinned `se-harness==0.4.0` wheel, with `--dry-run` so
nothing was written; the worktree was empty under
`git status --porcelain --untracked-files=all` before and after.

```text
$ python -m se_harness create-artifact . --domain simulation --type release_record \
    --id TEST-MOK-999 --dry-run
harnessctl: artifact ID must use the RLS- prefix and a three-digit suffix

$ python -m se_harness create-artifact . --domain simulation --type release_contract \
    --id TEST-MOK-999 --dry-run
harnessctl: artifact ID must use the REL- prefix and a three-digit suffix

$ python -m se_harness create-artifact . --domain simulation --type release_record \
    --id RLS-MOK-001 --dry-run
create   docs/engineering/simulation/releases/RLS-MOK-001.md
dry run: no files were written

$ python -m se_harness create-artifact . --domain simulation --type release_contract \
    --id REL-MOK-001 --dry-run
create   docs/engineering/simulation/release/REL-MOK-001.md
dry run: no files were written
```

Both types are creatable. Three things a reader should take from it:

1. **The prefixes are enforced and are not the domain prefix.** `RLS-` for the record, `REL-` for the
   contract. Passing a plausible-looking ID is refused before anything is written.
2. **The two directories differ by one letter, and it is not a typo.** The record lands under
   `simulation/releases/`, plural; the contract under `simulation/release/`, singular. Anyone reading a path
   in a hurry will get this wrong.
3. **`docs/RELEASE_RUNBOOK.md` already names both correctly** — Phase E line 232 for
   `docs/engineering/simulation/release/REL-MOK-001.md`, Phase F line 272 for
   `docs/engineering/simulation/releases/RLS-MOK-001.md`, and both again in its closing file table. This
   measurement confirms the runbook rather than correcting it, which is the outcome worth recording: the
   procedure an operator will follow was written against paths that turn out to be right.

## What this does not settle

**The `prepare-release` half is not exercisable yet, and cannot be made so here.** It requires
`--verification-record`, and `WORKFLOW.md` step 9 requires released work to match *eligible* coverage, which
means a `verified` or `released` VREC. No verification record exists — `capture-verification` refuses while
`WO-MOK-009` is `draft`, per `commit-binding.md` — so there is nothing to pass. Its argument surface is
readable and consistent with what the runbook's Phase F invokes (`--id`, `--release-contract`,
`--verification-record`, `--work-order`, `--version`, `--authorized-by`, and an optional `--tag`), but that is
a reading of `--help`, not a run.

So precondition 4 is **half discharged**: the two types can be created, and whether `prepare-release`
produces a conforming record is answerable only after the assurance chain exists. That ordering is inherent
rather than an omission — the command's input is a verified record.

Nothing here creates an artifact, transitions a status, or authorizes a release.
