# A2 — the record lives in a later commit than the one it names

`VER-MOK-008` scenario **A2**, captured 2026-08-19. Discharges `SPEC-MOK-005` rules 1.2 and 4, and
the defect recorded in `WO-MOK-009`'s Lifecycle section: an earlier draft of
`.github/workflows/release.yml` checked out the *tag* in its authorization job, which would have
refused every genuine release.

This is the ordering fact the whole workflow is shaped around. A release record states the commit it
authorizes, so it quotes that commit's hash — and can therefore only be written in a *later* commit.
The tagged tree never contains its own release record. `se_harness/provenance.py` says so in its own
words ("The release candidate commit may precede the governance commit retaining this record"), and
`VREC-MOK-001` in this repository already works this way.

The absence is **asserted**, not assumed: `git cat-file -e v1.0.0:…/RLS-001.md` must fail. If a
future change made the tagged tree contain the record, this transcript would stop reproducing.

The construction below is the committed suite's own fixture, driven outside `unittest` so the git
facts are visible. The corresponding assertion lives at
`scripts/test_check_release_authorization.py:230`
(`test_a2_the_record_lives_in_a_later_commit_than_the_one_it_names`).

## Transcript

```text
========================================================================================
A2 -- the record lives in a later commit than the one it names
========================================================================================

candidate commit (the tag's target)   bdfff661d0a0667714fcc027fc717db98cb6921a
$ git -C <root> rev-parse v1.0.0^{commit}
  bdfff661d0a0667714fcc027fc717db98cb6921a
  exit=0

$ git -C <root> cat-file -t refs/tags/v1.0.0
  tag
  exit=0

governance commit (holds the record)  10d53ad46487c0b549d5e96cb1f1494808a2cef0
they differ: True

The tagged tree does not contain the release record that names it:
$ git -C <root> cat-file -e v1.0.0:docs/engineering/domain/release/RLS-001.md
  fatal: path 'docs/engineering/domain/release/RLS-001.md' exists on disk, but not in 'v1.0.0'
  exit=128  (a failure is the point)

The governance commit does:
$ git -C <root> cat-file -e HEAD:docs/engineering/domain/release/RLS-001.md
  exit=0

$ git -C <root> show --stat --oneline HEAD
  10d53ad record the release
   docs/engineering/domain/release/REL-001.md       | 10 ++++++++++
   docs/engineering/domain/release/RLS-001.md       | 16 ++++++++++++++++
   docs/engineering/domain/verification/VREC-001.md | 12 ++++++++++++
   docs/engineering/domain/work-orders/WO-001.md    |  7 +++++++
   4 files changed, 45 insertions(+)
  exit=0

What the record says about the commit it authorizes:
$ git -C <root> show HEAD:docs/engineering/domain/release/RLS-001.md
  +++
  id = "RLS-001"
  type = "release_record"
  status = "released"
  version = "1.0.0"
  commit = "bdfff661d0a0667714fcc027fc717db98cb6921a"
  git_object_format = "sha1"
  tag = "v1.0.0"
  
  [relations]
  satisfies = ["REL-001"]
  includes_verification = ["VREC-001"]
  releases_work = ["WO-001"]
  +++
  
  body
  exit=0

$ python scripts/check_release_authorization.py --root <root> --tag v1.0.0
  AUTHORIZED release 1.0.0 of RLS-001
    candidate commit      bdfff661d0a0667714fcc027fc717db98cb6921a
    release contract      REL-001
    released work         WO-001
    included verification VREC-001
  exit=0

authorized: True
repository unchanged by the gate: True

Reading the graph from the tag instead would refuse every genuine release: the
record is not in the tagged tree, and cannot be, because it quotes that tree's
own commit hash. The workflow's `authorize` job therefore checks out the default
branch and resolves the tag separately.

```

## What it establishes

| Fact | Value |
| --- | --- |
| Candidate commit | `f1a1a32f6721209b01d104848d2b39b26d872ff5` (the tag's target) |
| Governance commit | `1df8ea7f75c54764588d4921470cc7e650e550fc` (holds `RLS-001.md`) |
| The two differ | yes |
| `v1.0.0:…/RLS-001.md` | absent — `git` exits 128 |
| `HEAD:…/RLS-001.md` | present, and states `commit = "f1a1a32f…"` |
| Gate verdict | `AUTHORIZED`, naming the candidate commit, not the governance commit |
| Repository after the run | unchanged |

The hashes are fixture hashes and differ on every run; the *relation* between them does not.

## Why the workflow is shaped this way

`authorize` checks out `${{ github.event.repository.default_branch }}` — never the tag — and
resolves the tag separately through the git tag namespace. It then pins the commit it read as
`governance_commit`, and every later job that reads the graph checks out that pin, so a push landing
mid-run cannot change the answer underneath the run. `harness` asserts it is standing on that exact
commit before it runs `doctor`, `validate` or `preflight`.
