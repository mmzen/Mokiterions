# A5 — the refusal ladder against the real repository

`VER-MOK-008` scenario **A5**, captured 2026-08-19. `VER-MOK-008`'s Evidence retention section names
this as the one measurement worth retaining in full, "because it is the one measurement that
demonstrates the gate discriminating between adjacent incomplete states of the real repository rather
than of a fixture."

## Why this is an independent oracle

The gate and its scenario suite share an author, which `VER-MOK-008` records as a residual
uncertainty. This measurement does not remove that, but it does not depend on it either: as long as
no `released` release record exists in this repository, the correct answer for *every* tag is a
refusal, and that answer was fixed by the state of the artifact graph long before the gate was
written. Nobody constructed this oracle for the gate to pass.

The ladder then adds the governance artifacts one at a time, in the order
`docs/RELEASE_RUNBOOK.md` phases D through F prescribe, and records which refusal remains at each
rung. The rung that refuses longest is the one only the release owner can climb.

## How it is run without exercising a reserved act

Creating a tag is an act `docs/engineering/DECISION_RIGHTS.md` reserves to the release owner. So the
ladder runs in a **throwaway clone**, deleted when the run ends. No tag was created in the
repository, no artifact was written under its artifact root, and no status was transitioned. The
clone carries only committed history, so the graph it holds is the real graph at
`54c21abcfb9caa4474c9ca5f194289e055c86a23` — none of `WO-MOK-009`'s own uncommitted artifacts reach
it.

## Transcript

```text
========================================================================================
A5 -- the refusal ladder against the real repository
========================================================================================

$ git clone --quiet --no-hardlinks --single-branch <repository> <clone>
  cloned commit 54c21abcfb9caa4474c9ca5f194289e055c86a23

--- rung 0: the repository as it stands, no tag ---
$ git -C <root> tag --list
  exit=0

$ python scripts/check_release_authorization.py --root <root> --tag v0.1.0
  REFUSED: v0.1.0 is not a tag in this repository. The release owner creates the tag; this workflow never does.
  
  This gate does not grant authority. It reports that the accountable release decision recorded under the artifact root does not authorize publishing this tag.
  exit=1

--- rung 1: an annotated tag, nothing else ---
$ git -C <root> cat-file -t refs/tags/v0.1.0
  tag
  exit=0

$ python scripts/check_release_authorization.py --root <root> --tag v0.1.0
  REFUSED: no release record has status 'released'. Only the release owner may make that transition; this workflow will not publish without it. No release record exists at all.
  
  This gate does not grant authority. It reports that the accountable release decision recorded under the artifact root does not authorize publishing this tag.
  exit=1

--- rung 2: + an approved release contract (runbook phase E) ---
$ python scripts/check_release_authorization.py --root <root> --tag v0.1.0
  REFUSED: no release record has status 'released'. Only the release owner may make that transition; this workflow will not publish without it. No release record exists at all.
  
  This gate does not grant authority. It reports that the accountable release decision recorded under the artifact root does not authorize publishing this tag.
  exit=1

--- rung 3: + a verified aggregate verification record (runbook phase D) ---
$ python scripts/check_release_authorization.py --root <root> --tag v0.1.0
  REFUSED: no release record has status 'released'. Only the release owner may make that transition; this workflow will not publish without it. No release record exists at all.
  
  This gate does not grant authority. It reports that the accountable release decision recorded under the artifact root does not authorize publishing this tag.
  exit=1

--- rung 4: + a release record, status = ready (runbook phase F) ---
$ python scripts/check_release_authorization.py --root <root> --tag v0.1.0
  REFUSED: no release record has status 'released'. Only the release owner may make that transition; this workflow will not publish without it. Found 1 release record(s) not yet transitioned to released: RLS-MOK-001.
  
  This gate does not grant authority. It reports that the accountable release decision recorded under the artifact root does not authorize publishing this tag.
  exit=1

--- rung 5: the release owner transitions ready -> released ---
$ python scripts/check_release_authorization.py --root <root> --tag v0.1.0
  AUTHORIZED release 0.1.0 of RLS-MOK-001
    candidate commit      54c21abcfb9caa4474c9ca5f194289e055c86a23
    release contract      REL-MOK-001
    released work         WO-MOK-001 WO-MOK-002 WO-MOK-003 WO-MOK-004 WO-MOK-005 WO-MOK-006
    included verification VREC-MOK-007
  exit=0

authorized: True

--- variant: the aggregate record left at `ready` ---
$ python scripts/check_release_authorization.py --root <root> --tag v0.1.0
  REFUSED: verification record VREC-MOK-007 is 'ready'. A release requires 'verified' or 'released'; a 'ready' record is a proposal the assurance owner has not accepted.
  
  This gate does not grant authority. It reports that the accountable release decision recorded under the artifact root does not authorize publishing this tag.
  exit=1

--- variant: the six existing records instead of one aggregate ---
$ python scripts/check_release_authorization.py --root <root> --tag v0.1.0
  REFUSED: included verification records bind ['68163ac452619e2f8d5a05ed3a73d42b920ba5f6', '95f0aa2079d4abced1c01f4c09b5c66dc5ab29fe', 'a7f39f18520433cddc72c044c6fca1b24104bb7d', 'dd0c2c0f7fc12c4fd11c457cfdc166d50b3d6ae4', 'ecd03a89c0c8680d8ce82d7767b787a49aa815fb', 'f3613701f3c55d3f3d849747c8cf0790a5729c14'], which is not exactly the released commit 54c21abcfb9caa4474c9ca5f194289e055c86a23. An aggregate release needs one verification record captured at the final candidate commit.
  
  This gate does not grant authority. It reports that the accountable release decision recorded under the artifact root does not authorize publishing this tag.
  exit=1

```

## The ladder, read as a table

| Rung | Graph state | Refusal that remains |
| --- | --- | --- |
| 0 | the repository as it stands | `v0.1.0 is not a tag in this repository` |
| 1 | + an annotated tag | `no release record has status 'released'. … No release record exists at all` |
| 2 | + an approved release contract (`REL-MOK-001`) | unchanged — the contract alone authorizes nothing |
| 3 | + a `verified` aggregate verification record (`VREC-MOK-007`) | unchanged — verification is not authorization |
| 4 | + a release record at `ready` | `Found 1 release record(s) not yet transitioned to released: RLS-MOK-001` |
| 5 | the release owner transitions `ready` → `released` | none — **AUTHORIZED** |

Rungs 2 and 3 are the informative ones. Adding an approved contract and a verified aggregate record —
everything an implementation agent or a verification run can produce — changes the refusal *not at
all*. The single fact that turns refusal into authorization is a human transition on a status field,
which is exactly the property `ENGINEERING_HARNESS.md` and `docs/engineering/DECISION_RIGHTS.md`
require and the only reason this workflow is safe to give write access to.

## Two variants worth keeping

**A `ready` aggregate record still refuses.** With the release record `released` but the verification
record left at `ready`, the gate refuses naming the record and its status: a `ready` record is a
proposal `capture-verification` prepared, not an assurance decision. This is the scenario a release
owner is most likely to hit in practice, because `capture-verification` emits `ready` and nothing
transitions it automatically.

**The six existing verification records cannot release an aggregate.** With the release record
including `VREC-MOK-001` through `VREC-MOK-006` instead of one aggregate, the gate refuses and lists
the six distinct commits they bind:

```text
68163ac452619e2f8d5a05ed3a73d42b920ba5f6
95f0aa2079d4abced1c01f4c09b5c66dc5ab29fe
a7f39f18520433cddc72c044c6fca1b24104bb7d
dd0c2c0f7fc12c4fd11c457cfdc166d50b3d6ae4
ecd03a89c0c8680d8ce82d7767b787a49aa815fb
f3613701f3c55d3f3d849747c8cf0790a5729c14
```

That is `docs/RELEASE_RUNBOOK.md` Phase D's reason for existing, measured on the real graph rather
than argued: each existing record binds the commit its own work order was verified at, and a release
needs the included records to identify exactly one candidate commit. `se_harness`'s own
`prepare_release` enforces the same constraint (`len(identities) == 1`), so this is not the gate
inventing a rule.

## Reproducing it

```bash
python scripts/test_check_release_authorization.py A5RealRepositoryTest
```

Six tests, all passing in 6.6 s as of this capture: the no-tag refusal, the no-record refusal, the
ladder, the `ready` variant, the six-record variant, and P4 against the real checkout.
