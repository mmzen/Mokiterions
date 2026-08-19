# Release Runbook

> Repository-owned operator documentation. **Not authority.** Nothing here approves anything,
> and nothing here overrides `ENGINEERING_HARNESS.md`, `docs/engineering/DECISION_RIGHTS.md`,
> or an approved artifact. Where this file and a governed artifact disagree, the artifact wins
> and this file is wrong.

This is the ordered procedure for cutting a release. It exists because the acts that make a
release are deliberately **not** automated — `ENGINEERING_HARNESS.md` reserves committing,
tagging, releasing and publishing to accountable humans — so the sequence has to be written
down somewhere or it lives only in someone's head.

`.github/workflows/release.yml` does the mechanical part. It builds, re-checks, packages, and
attaches a **draft**. It creates no tag, transitions no artifact, bumps no version, creates no
branch, and publishes nothing. Every check in it fails closed.

The tooling this file operates is governed: `INT-MOK-007` → `CAP-MOK-007` → `REQ-MOK-035`..`039`
→ `SPEC-MOK-005` → `VER-MOK-008` → `WO-MOK-009`. All of those were `draft` when this file was
written; the owners approved the chain on 2026-08-19, `WO-MOK-009` is `implemented`, and
`VREC-MOK-008` is `verified` against candidate commit `d35f817`. `SPEC-MOK-005` is the contract
this procedure's automation is written against; where the two disagree, the specification is
right and this file is wrong. `WO-MOK-009`'s Lifecycle section records that the automation was
written before the chain that describes it.

**Approved governance is not a release.** `check_release_authorization.py` still refuses every
tag, because no tag, no release contract and no release record exists — a verified verification
record is an input to a release decision, not a release decision. Nothing below has been run.

---

## The two revisions

Almost every mistake in this procedure comes from conflating these.

| | What it is | Where it lives |
|---|---|---|
| **candidate commit** | the code being released | reachable from `master` or a `release/*` branch |
| **governance revision** | the tip of `master` holding the release record | strictly later than the candidate |

A release record names a commit, so it quotes that commit's hash and can therefore only be
written **afterwards**. The harness says so itself: *"The release candidate commit may precede
the governance commit retaining this record"* (`se_harness/provenance.py`), and
`VREC-MOK-001` already works this way — it binds `ecd03a89c0c8680d8ce82d7767b787a49aa815fb`
from a later commit.

Consequences you cannot design around:

- **The tagged tree does not contain its own release record.** That is normal. Don't try to
  fix it.
- The workflow therefore reads the formal graph from `master` and resolves the tag separately.
  It pins the `master` commit it read and hands it to the compliance job, so a push landing
  mid-run cannot change the answer underneath it.
- The compliance job runs `doctor`, `validate` and `preflight` at the **governance** revision,
  not the candidate. The candidate's own graph was already validated by the managed
  `engineering-harness.yml` workflow when it merged; what is unproven until release time is
  that the records added *since* are valid.

---

## Who decides what

| Act | Who | Never |
|---|---|---|
| which work orders the release contains | release owner | CI |
| `ready` → `verified` on a verification record | assurance owner | CI, the implementation agent |
| `draft` → `approved` on the release contract | release owner | CI |
| `ready` → `released` on the release record | release owner | CI, anyone else |
| creating and pushing the tag | release owner | CI |
| creating `release/<major>.<minor>` | release owner | CI |
| clicking Publish on the draft | release owner | CI |

---

## Phase A — decide what the release is

Nothing in this phase is mechanical. Do not start Phase B until each item has an answer.

1. **Which work orders?** Only `implemented`, `verified` or `released` ones are eligible. **Derive
   the set from the command below rather than from this paragraph** — the list here is a dated
   reading, and it has already gone stale once: it said `WO-MOK-001` through `WO-MOK-006` and
   named `WO-MOK-007` as in flight on an unmerged branch. As of `a8fa962` on 2026-08-19 the
   eligible set is **`WO-MOK-001` through `WO-MOK-007` and `WO-MOK-009`**, all `implemented`;
   `WO-MOK-008` is a `draft` proposal and is not eligible. Eligible is not the same as chosen —
   which of the eight the release covers is yours to decide here, and every command below has to
   match that decision.

   How `WO-MOK-007` stopped being ineligible is the part worth keeping. Pull request #18 merged
   its branch, and that changed nothing here: it was still `approved`, and `approved` is not in
   `RELEASABLE_WORK_STATUSES`. Pull request #19 set it to `implemented`, and *that* made it
   eligible. **Eligibility follows the status field, not the merge**, so a paragraph like this one
   goes stale on a governance commit that may touch no code at all.

   ```bash
   python -m se_harness inspect .
   python -m se_harness preflight . --work-order WO-MOK-0NN --phase review   # per candidate
   ```

   The second command is rule 7.4's own check, and it is worth running per work order before you
   commit to a set: a work order that fails review preflight cannot be released, and at
   `a8fa962` `WO-MOK-008` is the only one that fails.

2. **Which version?** Both package manifests declare `0.1.0`, so the first release is
   `v0.1.0` on branch `release/0.1`. Changing a version in a manifest is *authorized work* —
   it needs a work order — not a step in this runbook. Decide the version before tagging, not
   after.

3. **The four outstanding amendment rows.** `SPEC-MOK-002:21-22`, `SPEC-MOK-003:47-48`,
   `SPEC-MOK-004:21` and `ARCH-MOK-001:46` each carry a row marked **OUTSTANDING**. Decide,
   per row, whether it blocks this release. The harness will not decide it for you: these are
   amendment records, not validation errors, and `validate` passes with all four open.

4. **`WO-MOK-008`.** `docs/engineering/simulation/evidence/WO-MOK-008/footer-tier-fallthrough.md`
   records a measured defect: at the narrowest declared viewport the provenance footer drops
   the resource density and the decision source for a large seed, and no commit stamp can be
   compiled in at all. Either approve and complete `WO-MOK-008` first, or release knowingly
   with the defect present. If you release knowingly, say so in the release contract's
   `## Security and provenance` section — that is what makes it a decision rather than an
   oversight.

   Because of it, the build deliberately leaves `MOKITERIONS_COMMIT` unset. The full 40-character
   commit still travels in four places: `PROVENANCE.txt` inside each archive, the release
   notes, the annotated tag, and the release record.

---

## Phase B — establish the candidate commit

```bash
git switch master
git pull --ff-only
git status --porcelain --untracked-files=all   # must print nothing
CANDIDATE="$(git rev-parse HEAD)"; echo "$CANDIDATE"
```

A clean worktree is not politeness: `capture-verification` and `prepare-release` both refuse
to run without one.

Run the repository's own declared checks, which are the ones the `verify` job re-runs. They
come from `docs/engineering/REPOSITORY_CONTEXT.md`, not from this file:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo tree -p Mokiterions --locked          # must resolve to exactly 1 crate
```

The last one discharges a named architectural claim rather than general hygiene:
`ARCH-MOK-001` as amended admits no engine dependency, with no exception, including one shared
with `mokiterions-tui`.

Then the harness:

```bash
python -m se_harness doctor .
python scripts/validate_engineering_artifacts.py --root .
for wo in 001 002 003 004 005 006; do
  python -m se_harness preflight . --work-order "WO-MOK-$wo" --phase review
done
```

All six must pass. `harnessctl` is not on `PATH` in this repository; `python -m se_harness` is
the invocation.

---

## Phase C — cut the maintenance branch

`docs/engineering/REPOSITORY_CONTEXT.md` cuts `release/<major>.<minor>` from the integration
branch **when a release enters stabilization** — before publication, not after. That is why the
workflow does not create it: by the time CI runs, the moment has passed.

```bash
git switch --create release/0.1 "$CANDIDATE"
git push --set-upstream origin release/0.1
git switch master
```

From here on:

- Only release-specific fixes may target `release/0.1`, on a `release-fix/<short-description>`
  branch cut from it.
- A fix that also applies to the current product must be forward-ported to `master`.
- If stabilization adds commits, **the candidate commit changes** — it becomes the tip of
  `release/0.1`. Re-run Phase B against the new tip and use that value for `CANDIDATE`
  everywhere below. The workflow accepts a candidate reachable from `master` or from any
  `origin/release/*` branch, and refuses anything else.

---

## Phase D — the aggregate verification record

The existing verification records each bind a **different** commit, one per work order. A
release needs the included records to identify exactly **one** candidate commit — the harness
enforces it (`prepare_release`) and so does the gate. So an aggregate release needs one new
verification record captured at the final candidate commit. The existing records are not
edited and not replaced; they remain the record of their own work.

Pick the next free record ID, and pick it from `origin/master` rather than from the branch you are on.
`VREC-MOK-001` through `008` are taken as of `a8fa962`: `007` was claimed by `WO-MOK-007` while `WO-MOK-009`
was in flight, which is the case this step exists for, and skipping the check once already produced an add/add
merge conflict — `docs/engineering/simulation/evidence/WO-MOK-009/id-collision.md` records it. Do not read
that count as current either; run the query, then set the variable from what it returns:

```bash
git ls-tree -r --name-only origin/master \
  -- docs/engineering/simulation/verification-records/ | sort
VREC=VREC-MOK-0NN      # the next ID the command above does not list
```

The three sets in the command below are **the eight eligible work orders as of `a8fa962`, not the
release's scope**. Cut them down to the work orders Phase A actually chose, and cut all three sets
the same way — the harness refuses a `--verification` set that is not exactly the union of the
chosen work orders' `verification` relations, so an edit to one list that misses another is a
refusal, not a silent mistake.

The evidence paths have to exist as committed files **before** you capture, because the command
requires a clean worktree and checks every path. So the keyed evidence lands in its own commit
first, and that commit becomes the candidate — the one the record binds, and the one Phase H
eventually tags. Choosing a candidate in Phase A and then adding the aggregate evidence on top of
it is normal and is not a change of candidate in any way that matters, provided the evidence commit
touches no Rust source, no manifest and no lockfile. Check that rather than assume it:

```bash
git diff --name-only <phase-A candidate> HEAD -- '*.rs' Cargo.toml Cargo.lock '*/Cargo.toml'
```

Empty output means the repository checks from Phase B still describe the tree you are about to
bind. Non-empty means you have a new candidate and Phase B has to run again.

With `HEAD` at that commit and the worktree clean:

```bash
python -m se_harness capture-verification . \
  --id "$VREC" \
  --work-order WO-MOK-001 --work-order WO-MOK-002 --work-order WO-MOK-003 \
  --work-order WO-MOK-004 --work-order WO-MOK-005 --work-order WO-MOK-006 \
  --work-order WO-MOK-007 --work-order WO-MOK-009 \
  --verification VER-MOK-001 --verification VER-MOK-002 --verification VER-MOK-003 \
  --verification VER-MOK-004 --verification VER-MOK-005 --verification VER-MOK-006 \
  --verification VER-MOK-007 --verification VER-MOK-008 \
  --evidence docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-001-containment.md \
  --evidence docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-002-containment.md \
  --evidence docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-003-containment.md \
  --evidence docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-004-containment.md \
  --evidence docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-005-containment.md \
  --evidence docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-006-containment.md \
  --evidence docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-007-containment.md \
  --evidence docs/engineering/simulation/evidence/release-0.1.0/WO-MOK-009-containment.md \
  --evidence docs/engineering/simulation/evidence/release-0.1.0/candidate-checks.md \
  --owner "assurance owner"
```

`WO-MOK-009` pairs with `VER-MOK-008`, not `VER-MOK-009`; the numbering does not line up, and
reading it as if it did is the mistake this line exists to prevent.

Three things about that command are not arbitrary:

- The `--verification` set must be **exactly** the union of the `verification` relations the
  selected work orders declare. For an aggregate, a missing one and an extra one are both
  refused.
- Every selected work order needs at least one evidence path whose **file name begins with that
  work order's ID**, followed by `-`, `.`, or nothing. The harness matches
  `^WO-MOK-0NN(?:-|\.|$)` against the file's *base name* and ignores the directories above it, so
  `evidence/WO-MOK-001/completion-report.md` is **not** keyed to `WO-MOK-001` — the directory
  carries the ID and the file name does not. Every path is also checked for existence and must name
  a file rather than a directory, which is a separate requirement from the keying.

  This is worth spelling out because this file got it wrong. The example above used to pass the
  eight per-work-order `completion-summary.md` paths, and every one of them was rejected:

  ```console
  harnessctl: aggregate evidence is not keyed to work orders: WO-MOK-001, WO-MOK-002, WO-MOK-003,
  WO-MOK-004, WO-MOK-005, WO-MOK-006, WO-MOK-007, WO-MOK-009
  ```

  The check runs only when more than one work order is selected, and every record in this
  repository before `VREC-MOK-009` covered exactly one, so nothing had ever exercised it while the
  whole evidence tree keys by directory. If you are capturing an aggregate for a different release,
  the paths above will not exist and you need keyed files of your own; do not reach for
  `work-orders/WO-MOK-0NN.md`, which satisfies the regex without being evidence of anything.
- The record binds `commit` to `HEAD`, so being at the candidate commit is the whole point.

The result is `docs/engineering/simulation/verification-records/<VREC>.md` with
`status = "ready"`. `ready` is a proposal. The **assurance owner** reads the evidence and
edits `status` to `verified`; nobody else may, and no command does it.

Then commit it on `governance/vrec-mok-0NN-verified` and merge through a pull request so the
managed `engineering-harness.yml` workflow checks it.

---

## Phase E — the release contract

No release contract exists yet; `REL-MOK-001` is free.

```bash
python -m se_harness create-artifact . --domain simulation \
  --type release_contract --id REL-MOK-001
```

That writes an incomplete `draft` at `docs/engineering/simulation/release/REL-MOK-001.md`.
Fill in every section — release unit, required evidence, compatibility and migration, security
and provenance, promotion policy, human approval triggers, rollback criteria, post-release
observation window — and set:

```toml
[relations]
gates = ["WO-MOK-001", "WO-MOK-002", "WO-MOK-003",
         "WO-MOK-004", "WO-MOK-005", "WO-MOK-006",
         "WO-MOK-007", "WO-MOK-009",
         "VER-MOK-001", "VER-MOK-002", "VER-MOK-003",
         "VER-MOK-004", "VER-MOK-005", "VER-MOK-006",
         "VER-MOK-007", "VER-MOK-008"]
```

Every released work order must appear in `gates`, or both the harness and the gate refuse. This
list is the same dated eight as Phase D's and carries the same instruction: match it to Phase A's
decision, not to this file.

Rollback deserves a real answer rather than a placeholder: this repository ships two binaries
with no state, no network and no persistence, so rollback is "install the previous archive" —
but say that explicitly, and say what the post-release observation window actually observes.

The **release owner** transitions `draft` → `approved`. Commit and merge through a pull
request.

---

## Phase F — the release record

Clean worktree, valid graph, verification record `verified`, contract `approved`:

```bash
python -m se_harness prepare-release . \
  --id RLS-MOK-001 \
  --release-contract REL-MOK-001 \
  --verification-record "$VREC" \
  --work-order WO-MOK-001 --work-order WO-MOK-002 --work-order WO-MOK-003 \
  --work-order WO-MOK-004 --work-order WO-MOK-005 --work-order WO-MOK-006 \
  --work-order WO-MOK-007 --work-order WO-MOK-009 \
  --version 0.1.0 \
  --authorized-by "release owner" \
  --tag v0.1.0
```

This writes `docs/engineering/simulation/releases/RLS-MOK-001.md` with `status = "ready"` and
`commit` taken **from the verification record**, not from `HEAD` — which is exactly why the
record can name the candidate commit while living in a later one.

`--tag` is optional. Supply it. Without it the gate falls back to the conventional `v<version>`
tag, which happens to be the same string here; with it, the record states its own intent and a
mismatched tag is refused rather than inferred.

The **release owner** reviews and transitions `ready` → `released`. `docs/engineering/DECISION_RIGHTS.md`
reserves this transition, and it is the single act the whole workflow is downstream of: nothing
is built or published without it.

Commit and merge to `master` through a pull request. **That merge is the governance revision.**

---

## Phase G — rehearse the gate before pushing anything

**Run the scenario suites first, while no tag exists.** They are named after the scenarios
`docs/engineering/simulation/verification/VER-MOK-008.md` enumerates and they cover both
refusal sets:

```bash
python scripts/test_check_release_authorization.py    # A1-A5, R1-R22, P4, properties
python scripts/test_check_release_reachability.py     # R23-R24, properties
```

That order is a defect rather than a preference. A5's real-repository test guards on
`git tag --list` being empty in a throwaway clone of this repository, so it fails as soon as
**any** tag exists locally — including the `v0.1.0` you are about to create, and including an
unrelated tag someone else left in the object store.
`docs/engineering/simulation/evidence/WO-MOK-009/suite-output.md` records an occurrence, the
measurement separating it from a real refusal, and the one-line narrowing that would fix it. If
it fails on a tag you did not expect, read `git tag --list` before reading it as a release
blocker.

Then the gate itself, which also runs locally. Create the tag but do not push it:

```bash
git switch master && git pull --ff-only
git tag -a v0.1.0 "$CANDIDATE" -m "Mokiterions 0.1.0"
python scripts/check_release_authorization.py --root . --tag v0.1.0
```

It must print `AUTHORIZED`, the candidate commit, the contract, the released work and the
included verification record. If it refuses, it says exactly which fact is missing. Fix the
graph, `git tag -d v0.1.0`, and repeat.

The gate refuses a lightweight tag, so create it with `-a`. It also refuses if
`.engineering-harness.toml` ever stops requiring full commits, because its central check is
equality of complete hashes.

Then rehearse the second check, which the gate deliberately does not perform — reachability is
a property of the history rather than of the artifact graph, and keeping it separate leaves the
gate runnable in a clone with no remote:

```bash
python scripts/check_release_reachability.py --root . --commit "$CANDIDATE" \
  --default-branch master --remote origin
```

It prints the release-bearing branch that contains the candidate. It reads
`refs/remotes/origin/*`, so a `release/0.1` you created but have not pushed does not count —
which is the point.

Rehearse both before pushing, because a pushed tag is awkward to withdraw and must never be
moved (see below). `workflow_dispatch` also exists, but it needs a tag that is already on the
remote, so it re-runs a release rather than previewing one.

The scenario suites at the top of this phase are what cover both refusal sets. Run them there,
before the tag exists, for the reason stated there.

---

## Phase H — push the tag

```bash
git push origin v0.1.0
```

The tag must be **annotated** — the gate checks the git object type and refuses a lightweight
tag, because an annotated tag records who tagged and when.

The push triggers `release.yml`:

1. **Authorization** — pins the governance revision, runs the gate (which resolves the tag,
   checks it is annotated, and reads the graph), records who tagged, then runs the reachability
   check against `origin/master` and every `origin/release/*` branch.
2. **Harness compliance** — asserts the installed `se-harness` equals the version
   `.engineering-harness.toml` declares, then `doctor`, `validate`, and a review `preflight`
   per released work order, at the governance revision.
3. **Repository checks** — the Phase B commands again, at the candidate commit, plus two
   byte-identical seeded runs per decision policy.
4. **Build** — Linux x86-64, macOS arm64, Windows x86-64, cold with no cache, each archive
   carrying `PROVENANCE.txt` and a `.sha256`.
5. **Publish draft** — the only job with write access.

If any job refuses, nothing reaches the outside world. Read the `REFUSED:` line; each one names
the fact it could not establish.

---

## Phase I — publish

The workflow attaches a **draft**. Publishing is the release owner's act.

1. Check the draft's notes state the record, contract, commit, released work and verification
   record. They are built from the authorized facts, not from `git log`.
2. Check three archives and three `.sha256` files are attached.
3. Verify one checksum yourself, and read the `PROVENANCE.txt` inside that archive:

   ```bash
   gh release download v0.1.0 --repo <owner>/Mokiterions --dir /tmp/rel
   ( cd /tmp/rel && sha256sum -c ./*.sha256 )
   tar -xzOf /tmp/rel/Mokiterions-0.1.0-x86_64-unknown-linux-gnu.tar.gz \
     Mokiterions-0.1.0-x86_64-unknown-linux-gnu/PROVENANCE.txt
   ```

   The `commit` line must equal `RLS-MOK-001`'s `commit`, all 40 characters.
4. Publish.

If the release owner wants a second human gate *before* any asset is uploaded, protect the
`release` environment in repository settings with a required reviewer. The workflow already
references that environment, so no edit is needed to turn it on.

---

## If something is wrong after the tag is pushed

**Never move or delete a published tag.** `v0.1.0` identifies one commit permanently; a moved
tag makes every prior statement about it false, and the gate deliberately refuses a tag whose
commit no longer matches its record.

Instead:

- **A refused job.** Nothing was published. Fix the fact it named, merge that fix, and re-run
  the workflow. If the candidate commit itself must change, the version must change too.
- **A bad candidate, caught before publishing.** Delete the *draft* release, leave the tag,
  and cut `v0.1.1` from a corrected candidate with its own verification record and its own
  release record. `RLS-MOK-001` stays as the record of what was decided; supersede it rather
  than editing it.
- **A bad candidate, caught after publishing.** Same, plus mark the published release as
  superseded in its notes. The rollback procedure is the one written in `REL-MOK-001`, not one
  invented here.

---

## Files this procedure touches

| Path | Mode | Note |
|---|---|---|
| `.github/workflows/release.yml` | repository-owned | this procedure's automation |
| `scripts/check_release_authorization.py` | repository-owned | the gate; absent from `.engineering-harness.lock` |
| `scripts/check_release_reachability.py` | repository-owned | the reachability check, `SPEC-MOK-005` rule 5 |
| `scripts/test_check_release_authorization.py` | repository-owned | scenarios A1–A5, R1–R22, P4 |
| `scripts/test_check_release_reachability.py` | repository-owned | scenarios R23–R24 |
| `rust-toolchain.toml` | repository-owned | pins the compiler the evidence was produced with |
| `.github/workflows/engineering-harness.yml` | **managed** | do not edit; `doctor` integrity-checks it |
| `docs/engineering/simulation/specifications/SPEC-MOK-005.md` | artifact | the contract the automation is written against; `approved` |
| `docs/engineering/simulation/verification/VER-MOK-008.md` | artifact | the verification contract for it; `approved` |
| `docs/engineering/simulation/work-orders/WO-MOK-009.md` | artifact | authorizes implementing it; `implemented` |
| `docs/engineering/simulation/verification-records/VREC-MOK-008.md` | artifact | binds `WO-MOK-009`'s evidence to `d35f817`; `verified` |
| `docs/engineering/simulation/release/REL-MOK-001.md` | artifact | Phase E |
| `docs/engineering/simulation/releases/RLS-MOK-001.md` | artifact | Phase F |
| `docs/engineering/simulation/verification-records/VREC-MOK-0NN.md` | artifact | Phase D |
