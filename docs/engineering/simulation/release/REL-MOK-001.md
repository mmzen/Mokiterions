+++
id = "REL-MOK-001"
type = "release_contract"
title = "Promote Mokiterions 0.1.0"
status = "draft"
owners = ["release owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
gates = ["WO-MOK-001", "WO-MOK-002", "WO-MOK-003",
         "WO-MOK-004", "WO-MOK-005", "WO-MOK-006",
         "WO-MOK-007", "WO-MOK-009",
         "VER-MOK-001", "VER-MOK-002", "VER-MOK-003",
         "VER-MOK-004", "VER-MOK-005", "VER-MOK-006",
         "VER-MOK-007", "VER-MOK-008"]
+++

# Release Contract: Promote Mokiterions 0.1.0

This is a `draft`. Every judgement below is **proposed** for the release owner's decision, not
decided. The transition to `approved` is that decision, and nothing here anticipates it.

The version is `0.1.0`, which is what `mokiterions-core`'s manifest already declares — this release
publishes the version the repository holds rather than bumping to a new one, so no manifest change is
part of it.

## Release unit

Three archives built from one authorized commit, one per target, plus a checksum for each:

| Target | Runner | Assets |
| --- | --- | --- |
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` | `Mokiterions-0.1.0-x86_64-unknown-linux-gnu.tar.gz` + `.sha256` |
| `aarch64-apple-darwin` | `macos-latest` | `Mokiterions-0.1.0-aarch64-apple-darwin.tar.gz` + `.sha256` |
| `x86_64-pc-windows-msvc` | `windows-latest` | `Mokiterions-0.1.0-x86_64-pc-windows-msvc.tar.gz` + `.sha256` |

Each archive contains both binaries — `Mokiterions`, the engine, and `mokiterions-tui`, the terminal
observer — together with `LICENSE`, `README.md`, `SIMULATION_RULES.md` and a generated
`PROVENANCE.txt`. All four inputs are present at the candidate commit; the packaging step copies them
unconditionally and would fail if any were missing.

What is **not** in the release unit: no crates.io publication, no container image, no installer, no
package-manager manifest, and no source archive beyond the one GitHub attaches to a tag by itself.
The `aarch64-apple-darwin` target is cross-built on an Apple runner and is not exercised by any test
on that architecture — the workspace test run happens on the authorized commit, not per target.

## Required evidence

The release may proceed only on this evidence, all of which exists at the point of approval:

- `VREC-MOK-009`, `verified`, binding the candidate commit. It is the aggregate record: the eight
  work orders' own records each bind a different commit, and this one binds the single commit being
  released. It must be `verified` by an accountable assurance owner before `prepare-release` runs.
- The eight per-work-order records it stands on — `VREC-MOK-001` through `VREC-MOK-008`, all
  `verified` — each covering the work order it names.
- The nine evidence files `VREC-MOK-009` binds, under
  `docs/engineering/simulation/evidence/release-0.1.0/`: one per work order stating that its verified
  commit is an ancestor of the candidate, and `candidate-checks.md` recording every gate run against
  the candidate tree.
- The candidate's own results: `cargo fmt`, `clippy` and `cargo tree` clean, 179 tests passed and 0
  failed, `validate` PASS with 0 errors and 0 warnings across all four planes, `doctor` 81 PASS with
  0 WARN and 0 FAIL, review preflight passing for all eight released work orders, and the
  authorization gate's two suites at 48 and 22 cases.

**One limitation is required reading rather than a footnote.** `VREC-MOK-008` recorded that the
release machinery was verified statically and against constructed fixtures and had never been run;
`VREC-MOK-009` restates it. This release is the first execution of
`.github/workflows/release.yml`. Approving this contract accepts that the first real run of the
release process happens on a real release.

## Compatibility and migration

Nothing to migrate, and the reason is a property of the software rather than a promise about it: the
engine and the observer hold no persistent state, open no network connection, read no configuration
file and write no data outside their own standard output. A run is determined entirely by its
command-line options and its seed. There is no on-disk format, no protocol and no stored artifact
that a later version could be incompatible with.

`0.1.0` is the first release, so there is no prior version to be compatible with. Compatibility
obligations begin with the next one.

The public Rust API is not part of this release unit. The archives ship binaries; the crates are not
published, so no downstream code can depend on the library surface and no semantic-versioning promise
about it is made here.

## Security and provenance

- **No secrets, and none needed.** The workflow uses the automatic `GITHUB_TOKEN` and no other
  credential. Nothing in the release unit embeds a key, and
  `docs/engineering/REPOSITORY_CONTEXT.md` requires credentials to stay outside the repository.
- **No third-party dependencies in the engine.** `cargo tree -p Mokiterions --locked` resolves to one
  crate, and that is a release-blocking requirement rather than an observation: a second line in that
  output stops the release. The observer's dependencies are locked, and every build step passes
  `--locked` so the lockfile cannot be silently resolved around.
- **Every archive states its own provenance.** `PROVENANCE.txt` records the version, the release
  record, the release contract, the released work orders, the verification records, the tag, the full
  authorized commit hash and the target, so an asset separated from this run can still be traced to
  the decision that authorized it.
- **A checksum accompanies each archive**, generated on the runner that produced it.
- **The gate refuses before anything is built.** The authorization job establishes the tag, a
  `released` release record for that tag, that the record's commit is the tag's target, and that the
  commit is reachable from a release-bearing branch. Build and publish are downstream of that
  refusal, so an unauthorized tag produces no assets.
- **Not claimed:** the archives are not signed, there is no attestation or SBOM, and the binaries are
  not reproducible across runners. Provenance here is a stated chain, not a cryptographic one.

## Promotion policy

One authorized commit, one tag, one draft release. Promotion is strictly ordered and each step
refuses if the one before it did not happen:

1. `VREC-MOK-009` is `verified` and this contract is `approved`.
2. `RLS-MOK-001` is prepared by `prepare-release`, which takes its commit from `VREC-MOK-009`, and
   the release owner transitions it to `released`.
3. The governance revision — the records above — is merged to `master`.
4. The release owner pushes the maintenance branch `release/0.1`, then the annotated tag `v0.1.0` at
   the authorized commit.
5. The tag push starts the workflow, which authorizes, checks compliance, checks the repository,
   builds three targets and uploads a **draft**.
6. A person publishes the draft.

No step may be reordered to save time. Pushing the tag before `RLS-MOK-001` is `released` produces a
refusal, which is the designed outcome and not a failure to work around.

## Human approval triggers

Automation prepares; people decide. Four decisions in this release belong to a named role and to no
command:

| Decision | Role | Act |
| --- | --- | --- |
| `VREC-MOK-009` `ready` → `verified` | assurance owner | Judges whether the retained evidence supports the record's two claims |
| `REL-MOK-001` `draft` → `approved` | release owner | Accepts this contract, including the first-run limitation above |
| `RLS-MOK-001` `ready` → `released` | release owner | The single act the entire workflow is downstream of |
| Publishing the draft release | release owner | The workflow never publishes |

Additionally, the release owner alone creates and pushes `v0.1.0` and pushes `release/0.1`. The
workflow creates no tag, moves no tag, transitions no artifact, bumps no version, creates no branch
and publishes nothing.

Re-running the workflow by hand via `workflow_dispatch` requires an existing remote tag and re-runs
the real process against it. It is not a preview, and it is not a way to rehearse.

## Rollback criteria and procedure

**Roll back if**, after publication, any of the following is observed: an archive does not extract;
either binary does not start on its advertised target; the engine's exit codes do not match
`SPEC-MOK-001`; a run is not reproducible from its seed; `PROVENANCE.txt` names a commit, record or
tag that does not match the release record; or a checksum does not match its archive.

**The procedure is to install the previous archive**, and for `0.1.0` that has a specific
consequence worth stating rather than glossing: there is no previous archive. Rollback for this
release means un-publishing — return the GitHub release to draft or delete it, so the assets stop
being reachable — and it does not mean reverting the repository. Users who already downloaded an
archive have a self-contained directory of binaries with no installer, no registry entry and no
state, so removing that directory is a complete uninstall on every target.

Because the software persists nothing and connects to nothing, a rollback cannot leave data behind,
cannot corrupt a store and cannot strand a partially-migrated user. That is the whole of the risk
argument, and it is a property of the release unit rather than a promise about response time.

The tag and the `released` record are **not** rolled back. They are the record that this release was
authorized and happened; a superseding release, or an explicit governed correction, is how a mistake
is recorded. Deleting them would falsify history rather than repair it.

## Post-release observation window

**Seven days from publication.** What is actually observed, in order of what would matter most:

1. **The workflow's own first real run.** Every job's result, and specifically whether any step that
   has only ever run against fixtures behaves differently against a real tag. This is the first
   execution of the release process, so the run itself is an observation and its logs are evidence to
   retain.
2. **Asset integrity.** Each of the three archives downloaded, checksum verified, extracted, and both
   binaries started on the matching target. The `aarch64-apple-darwin` archive deserves particular
   attention because it is cross-built and no test ran on that architecture.
3. **`PROVENANCE.txt` against `RLS-MOK-001`.** The commit, tag, record, contract, work orders and
   verification records in the archive must match the release record exactly.
4. **The gate, re-run after the fact**, to confirm it now authorizes the tag it previously refused.
5. **Issues reported against the release**, which for a simulation with no network and no persistence
   are expected to be about behaviour or documentation rather than security.

The window closing is not itself a decision and grants nothing. If it passes with nothing observed,
the release simply stands. If something is observed, the rollback criteria above apply, and anything
learned about the release process belongs in `docs/RELEASE_RUNBOOK.md` and in a work order rather
than in this contract, which is closed once approved.
