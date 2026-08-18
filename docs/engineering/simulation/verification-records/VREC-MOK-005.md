+++
id = "VREC-MOK-005"
type = "verification_record"
title = "Verification candidate for WO-MOK-005"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-18"
updated = "2026-08-18"
commit = "6601d29b09a6acf18491f91011d7ce311da7d40c"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-18T06:30:10Z"
artifact_snapshot_sha256 = "42013c623d2e70887225a356004c07f7048fae2e8d09e4c2f3f59f8d9f590217"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-005/README.md", "docs/engineering/simulation/evidence/WO-MOK-005/additivity-proof.txt", "docs/engineering/simulation/evidence/WO-MOK-005/boundary-and-security-review.md", "docs/engineering/simulation/evidence/WO-MOK-005/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-005/dependency-review.txt", "docs/engineering/simulation/evidence/WO-MOK-005/export-fidelity.txt", "docs/engineering/simulation/evidence/WO-MOK-005/exports.txt", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed0-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed1-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed123-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed42-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed777-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/frames.txt", "docs/engineering/simulation/evidence/WO-MOK-005/layout-and-viewports.txt", "docs/engineering/simulation/evidence/WO-MOK-005/manual-assessment.md", "docs/engineering/simulation/evidence/WO-MOK-005/non-perturbation.txt", "docs/engineering/simulation/evidence/WO-MOK-005/requirement-to-test-mapping.md", "docs/engineering/simulation/evidence/WO-MOK-005/resilience.txt", "docs/engineering/simulation/evidence/WO-MOK-005/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-005/terminal-restoration.txt", "docs/engineering/simulation/evidence/WO-MOK-005/test-run.txt"]

[relations]
verifies_work_order = ["WO-MOK-005"]
conforms_to = ["VER-MOK-005"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-005` to candidate commit `6601d29b09a6acf18491f91011d7ce311da7d40c`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## Why this record was recaptured

An earlier candidate bound commit `b3fed6a7`, which is no longer this branch's head. `master` then
merged its own `WO-MOK-003` and `WO-MOK-004`, and this branch merged `master`, so every figure in the
first candidate described a tree that no longer exists. The whole evidence packet was re-derived
against the merged tree in `ede00bf`, and the earlier record was retired in the candidate commit this
one names.

Two claims in the first record were not merely stale but wrong, and both are corrected below: its
additivity claim used `48d16bd4` as the comparison baseline, which attributed `master`'s own
`WO-MOK-003` and `WO-MOK-004` changes to this work order; and its fourth disclosure said
`Simulation::run` "became publicly reachable when the library target was added" by this work, which
`origin/master` disproves.

## What this record claims

`WO-MOK-005` is `in_progress` and `VER-MOK-005` is `approved`. At candidate commit
`6601d29b09a6acf18491f91011d7ce311da7d40c`, **every automated case, invariant, static check,
security check and resilience check in `VER-MOK-005` was executed**, and each row of its
requirement-to-evidence matrix is mapped to a named test or a retained file in
`requirement-to-test-mapping.md`.

| Gate | Result |
|---|---|
| `cargo test --workspace` | 169 passed, 0 failed, 0 ignored (60 engine, 109 observer) |
| `cargo test -p Mokiterions` | 60 passed, 0 failed — 37 inline in `src/simulation.rs`, 23 across the five files in `tests/` |
| `cargo fmt --all -- --check` | no differences |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | 0 findings, and 0 again for `-p Mokiterions` alone |
| `cargo build --workspace` and `-p Mokiterions` | clean |
| `cargo tree -p Mokiterions` | resolves to the package alone on every edge kind |
| Artifact validation | PASS — 58 artifacts, 0 errors, 0 warnings |

Every gate above was run from clean: both packages are `cargo clean`ed before each invocation, and
`clippy -p Mokiterions` is run before the workspace pass, so no recorded line is a cache hit standing
in for a check that did not run. `static-checks.txt` and `test-run.txt` retain the transcripts.

Four obligations `VER-MOK-005` singles out, each measured rather than argued:

- **Additivity.** Measured against `origin/master` at `903c9943`, which `git merge-base
  --is-ancestor` confirms is an ancestor of this branch's head, so the comparison is a
  before-and-after of one lineage. The five files under `tests/`, `src/main.rs` and `src/cli.rs` are
  byte-identical; the inline `#[cfg(test)] mod tests` in `src/simulation.rs` diffs empty at 1,072
  lines and 37 cases; the set of 45 test-function names is identical; the engine total is 60, the
  figure `WO-MOK-004` recorded. The stop condition — a test that must change — did not occur.
- **Non-perturbation.** Observed and unobserved runs are identical on seeds `0`, `1`, `42`, `123` and
  `777`, with a frame drawn every tick and an interaction applied every tick.
- **Export fidelity.** The observer's exports are byte-identical to the engine binary process's own
  stdout across ten comparisons — five seeds at 20 ticks against the retained files, five at the
  default 100 ticks against `sha256` digests. Re-verified against the rebuilt `Mokiterions.exe` on
  the merged tree, with every figure unchanged.
- **Layout.** All six declared canvas interiors measured exactly as derived — 67 × 32, 67 × 32,
  47 × 32, 71 × 36, 98 × 24, 32 × 16 — with `33 × 21` refused before the terminal is entered.

## What this record does not claim

**No manual assessment in `VER-MOK-005` was executed. All seven are outstanding and none has an
author.** `VREC-MOK-002` was able to claim that every case *and manual assessment* had been
executed; this record cannot, and the difference is deliberate rather than an omission in drafting.

`manual-assessment.md` records the reason. Three of the seven require an interactive terminal, and
the observer cannot be driven from this environment: `crossterm` on Windows reads the console input
buffer, so a keypress piped to the process never reaches it — measured as exit code 124, 7,017 bytes
of frame output and empty stderr — and `winpty` cannot supply a pseudo-terminal without a tty to
read its size from. The remaining four are judgments about what a person perceives, and recording my
reading of a buffer dump as a pass would restate an automated assertion in prose, which is the exact
gap `VER-MOK-005`'s residual-uncertainty section says cannot be automated away.

The consequence for this record is specific. `VER-MOK-005` states that a pass on every automated case
combined with a negative answer to the two-hundred-tick instrument assessment "is an adverse
observation requiring product review, because the automated cases verify the parts and this assesses
the instrument". The automated half of that condition is satisfied. The other half is unknown. So
this record establishes that the parts behave as specified; it does not establish that the instrument
answers the questions `INT-MOK-004` names, and it does not establish that anything is legible on a
real terminal.

Every automated result bound here is an assertion about an in-memory character buffer.

## Amendments to approved artifacts that this record does not carry

Six provisions across three approved artifacts have been amended in the tree, and **every one is
recorded as OUTSTANDING because it is the technical owner's act and the owner has not taken it.** The
amended text is written; the approval is not given. This record cannot supply it, and transitioning
this record to `verified` would not supply it either — an assurance owner's verification is a
statement about evidence, not about another role's approvals.

- **`SPEC-MOK-002`, four provisions** (amendment record, 2026-08-18): rule 1's "no second package, no
  workspace" narrowed to a workspace of exactly two packages, on the approved `REQ-MOK-026` the
  clause reserved the exception for; rule 3's freeze on `src/simulation.rs`'s contents scoped to the
  `WO-MOK-003` restructuring it was written for; rule 5's closed public enumeration grown by the
  observation surface, under rule 5's own growth clause; rule 6's prohibition narrowed from five
  named value types, and from the return-value path, to the capability it was written to deny.
- **`ARCH-MOK-001`, one provision** (amendment record, 2026-08-18): the prohibition on public items
  narrowed from "mutable **or owned** authoritative state" to a mutable borrow of, or a reference
  into, that state. As written it forbade the owned, reference-free snapshots `SPEC-MOK-003`
  specifies — the capability `REQ-MOK-019` through `REQ-MOK-025` and `REQ-MOK-027` require, and one
  that grants no mutation.
- **`SPEC-MOK-003`, one provision** (amendment record, 2026-08-18): rule 2 of *Data and interface
  contracts*, which said `advance_tick` was the only `&mut self` method on the surface that changes
  state. It is not; `run` is a second. This one was found by measurement during implementation, not
  before it, so it is not among the preconditions `WO-MOK-005` lists.

`WO-MOK-005` names the first five as approval preconditions and states plainly that it "is not
verifiable until they are given or the scope is changed to avoid needing them". None of the five
could have been part of the 2026-08-17 approval: the wording each narrows reached `master`
afterwards, under `WO-MOK-003`. The sixth arose from this work's own measurement. Nothing about
mutation, dependency direction, determinism or observable behavior is relaxed by any of them, and the
engine package's dependency table stays empty.

## What the accountable assurance owner must weigh before verifying

The outstanding manual assessments and the six outstanding amendments above are the first two items.
`completion-summary.md` discloses fourteen findings and two further notes. None is a failure against
`VER-MOK-005`; each is stated so that verification, if given, is given knowingly. The ten that bear
on how much the green gates mean:

1. **`ADR-MOK-003` line 205 is factually wrong.** It says `Cargo.lock` acquires 57 entries; the file
   has 182. 57 is the `cargo tree --edges normal` crate count, a different measurement — the lock
   file records every platform's and every optional feature's packages, while `cargo tree` resolves
   for this host. The conclusion the sentence draws from the number holds, and holds more strongly at
   182. The number does not. Correcting it amends an approved artifact.
2. **The 57-crate figure is one of four readings** and is stated twelve times as though it were the
   only one: `SPEC-MOK-003` lines 57, 528, 589; `ARCH-MOK-002` lines 28, 159; `ADR-MOK-003` lines 80,
   96, 144, 176, 177, 186, 202. Measured on this tree: 57 with `--edges normal`, 59 counting build
   dependencies, 37 excluding proc-macro crates, 61 counting both workspace members.
   `dependency-review.txt` records all four and names the two crates the build-edge reading adds and
   neither binary links — `rustc_version` and `semver`. The decision `ADR-MOK-003` takes is
   unaffected; a reviewer comparing a future `cargo tree` against "57" needs to know which invocation
   produced it.
3. **`SPEC-MOK-003` rules 4 and 5 cannot both be satisfied literally.** Rule 4's roster mockup at
   lines 226–229 needs about 87 columns of bars plus rule 4.5's reserved fourth slot, about 26 more;
   rule 5 gives the pane a 45-column interior. Resolved as `min(20, (interior − 27) / 3)` at
   `render.rs:495-498`, which makes the reserved slot zero-wide at the reference roster — absent
   there rather than empty, which is not what rule 4.5 describes.
4. **`SPEC-MOK-003` rule 2 does not hold as written, and the deviation predates this branch.** The
   public surface exposes two `&mut self` operations that change simulation state, not one:
   `advance_tick` and `run`. `run` is the `REQ-MOK-010` whole-run entry point and was **already
   publicly reachable at `origin/master`** — `903c9943` has `pub mod simulation` in `src/lib.rs` and
   `pub fn run<W: Write>(&mut self, …)` at `src/simulation.rs:821`, both created by `WO-MOK-003` and
   bound by a `verified` `VREC-MOK-003`. This work did not expose it; the rule was already inaccurate
   about the surface before this work began. The observer never calls it, and
   `grep -rn 'pub fn .*&mut self' src/` returns exactly those two. Narrowing `run` would require
   relocating the engine's sources, which `WO-MOK-005` excludes from scope, or duplicating the run
   loop, which would give two implementations of the rules.
5. **`VER-MOK-005` acceptance scenario 2 describes an unreachable state.** No shipped decision source
   can have a proposal rejected, asserted over 400 ticks of both policies, so the rejection
   presentation is verified only through a `#[cfg(test)]` hook that injects a self-contradictory
   decision. Manual assessment 4 therefore cannot be performed by running the observer at all.
6. **A world with no standing resources is likewise unreachable** and is verified through a second
   `#[cfg(test)]` hook. Both hooks are compiled out of the shipped binary.
7. **`REQ-MOK-024`'s hidden-roster-entry case is unreachable at every declared viewport** and is
   verified by code inspection only. Twelve two-line entries need 24 rows and never overflow a 32- or
   36-row interior.
8. **`SPEC-MOK-003` rule 9.2's territory subject filter is not expressible** with the engine's
   records, whose distinct subjects in a run are `M##`, `F####` and `world` — territory is a field
   inside `result`, not a subject. Closing it needs a filter dimension that reads `result`, or a
   change to the engine's subjects. Both are specification decisions, not observer fixes.
9. **Four matrix rows had no test behind them at the first evidence pass** and were closed by four
   new tests before the first record was prepared. An audit that found four is not evidence that
   there is no fifth.
10. **Two claims in the first candidate record were wrong, and I wrote them** — the additivity
    baseline and the `run`-reachability sentence named above. Both are corrected here and in the
    evidence. The correction came from re-measuring on a changed tree rather than from review, which
    is itself a fact about how far unreviewed prose in a packet can be trusted.

Two further coverage limits, narrower than the above: non-perturbation is measured under a script
driving 13 of the 25 controls `state.rs` binds, and the arrow-key aliases `Left`, `Right`, `Up` and
`Down` are bound in `state.rs` and exercised by no test in the package.

## Behavioural observations that are not failures

- Seed `42` under `--policy baseline` reaches extinction on tick 142, which is why the long-run case
  and manual assessment 1 use `--policy reference`.
- Under `--policy reference`, 11 of 12 Mokiterions die by tick 10,000. Living count plus deaths sums
  to 12 throughout, so `REQ-MOK-020`'s corroboration contract holds. World viability at the default
  density is `INT-MOK-002`'s subject, not this work order's.

## Scope of the transition being requested

Transitioning this record to `verified` would record that the assurance owner accepts the automated
evidence at this candidate commit **with all seven manual assessments outstanding and six amendments
to approved artifacts awaiting the technical owner**. It would not perform those assessments, would
not supply those approvals, and would not release, tag, publish or deploy anything. If either set is
to gate verification instead, this record should stay `ready` until the assessments are performed and
their author recorded in `manual-assessment.md`, and until the amendments are approved by the role
that owns them.
