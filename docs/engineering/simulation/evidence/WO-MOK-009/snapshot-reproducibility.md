# What `artifact_snapshot_sha256` is reproducible from

Measured 2026-08-19 while preparing `VREC-MOK-007`, from the pinned `se-harness==0.4.0` wheel. It is here
because a verification record's snapshot field is only worth having if a later reader can recompute it, and
the first attempt to recompute it produced a different value from the same commit.

## What the field is

`se_harness/provenance.py` computes it in one function:

```python
def _generate_snapshot(repository_root: Path) -> str:
    script = template_root() / "scripts" / "generate_harness_dashboard.py"
    ...
    snapshot = repository_root / "target" / "harness-dashboard" / "dashboard-data.json"
    ...
    return hashlib.sha256(snapshot.read_bytes()).hexdigest()
```

So it is the SHA-256 of a generated JSON file, not of the artifact files themselves. Whatever that generator
writes into `dashboard-data.json` is inside the hash.

## The measurement

The same commit, `9781d7a8615a028635674ea29e9662a4b70d4f9b`, captured from five clean checkouts. Each row is
one `capture-verification` run with identical arguments; only the checkout differs.

| Checkout | Directory basename | Line endings in the worktree | `artifact_snapshot_sha256` |
| --- | --- | --- | --- |
| the linked worktree the work was done in | `Mokiterions-release-ci` | LF | `579dfff…` |
| a fresh clone | `Mokiterions-release-ci` | **CRLF** | `579dfff…` |
| a fresh clone | `Mokiterions` | CRLF | `9d0ab9c…` |
| a fresh clone, `core.autocrlf=false` | `snapclone-lf` | LF | `d2b31cc…` |
| a fresh clone | `snapclone` | CRLF | `864091d…` |

Two readings follow, and the second is the one that matters.

**Line endings do not affect the field.** Rows 1 and 2 are the control: the same directory name, one worktree
LF and one CRLF — `core.autocrlf` is `true` on this machine and the repository has no `.gitattributes`, so a
fresh clone checks out CRLF while the files as authored are LF. The hash is identical. That disposes of the
first hypothesis, which was that the field silently binds a platform's line-ending convention.

**The checkout directory's basename is inside the hash.** Diffing the generated JSON between two checkouts of
the same commit gives exactly one differing line out of about 170 KB:

```diff
--- worktree/target/harness-dashboard/dashboard-data.json
+++ clone/target/harness-dashboard/dashboard-data.json
@@ -5319 +5319 @@
-    "name": "Mokiterions-release-ci",
+    "name": "snapclone-lf",
```

The dashboard records the repository's *directory name* as the project name, and the snapshot hashes the
dashboard. Nothing else in 170 KB differs. Within one checkout the value is deterministic: two consecutive
runs in the same clone agree, and it does not depend on the record's output path.

## Why this changed how `VREC-MOK-007` was prepared

`VREC-MOK-007` was first captured in the linked worktree this work was done in, whose directory is named
`Mokiterions-release-ci`. That yields `579dfff…`, which is correct, deterministic, and **not** what an
assurance owner checking the record would compute: a plain `git clone` of this repository produces a
directory named `Mokiterions`, and every one of `VREC-MOK-001` through `VREC-MOK-006` was captured in a
checkout named that. A record whose snapshot cannot be reproduced by the obvious check invites exactly the
wrong conclusion.

So the record in the repository was **re-prepared in a throwaway clone checked out into a directory named
`Mokiterions`**, and the file the harness produced there was moved into place unchanged. The two captures
differ in two fields and no others — `verified_at`, and the snapshot — which was confirmed by a byte-level
diff before the substitution. The commit, the object format, the clean worktree state, the evidence paths,
the work order and the verification contract are identical either way, because they are properties of the
commit rather than of the checkout.

**The order of the two commits is why this file's past tense is accurate.** `docs/engineering/WORKFLOW.md`
line 22 forbids a record from containing the hash of its own commit, so this file is committed first and the
re-preparation is performed against the commit that contains it; `VREC-MOK-007` then arrives in the next
commit, naming that hash. A reader at the earlier of the two commits will find this record and no
verification record, which is the same shape `approval-and-transition.md` describes for its own commit.

To recompute it: clone this repository so the working directory is named `Mokiterions`, check out the commit
the record names, and run `capture-verification` with the same arguments. The record's prose states this too,
so a reader who never opens this file is not misled.

## What this is not

**It is not a defect in the harness, and it is not a finding against `SPEC-MOK-005`.** No rule in
`SPEC-MOK-005` and no scenario in `VER-MOK-008` concerns the snapshot field; the release process reads
release records and verification-record *statuses*, and `check_release_authorization.py` never recomputes a
snapshot. Nothing in the scenario totals moves.

It is a property of a managed provenance field that is easy to trip over and expensive to diagnose after the
fact, and the useful thing to record is what the field is reproducible *from*: the commit, plus the name of
the directory it is checked out into.

The six existing records are unaffected in substance — they were captured in a checkout named `Mokiterions`
and are reproducible by the obvious check. This record makes `VREC-MOK-007` the same.

This record takes the directory to seventeen files — sixteen records and the index. `approval-and-transition.md`'s
closing count of sixteen was correct when it was written.
