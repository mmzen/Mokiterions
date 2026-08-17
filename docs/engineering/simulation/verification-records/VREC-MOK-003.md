+++
id = "VREC-MOK-003"
type = "verification_record"
title = "Verification candidate for WO-MOK-003"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-17"
updated = "2026-08-17"
commit = "b3fed6a7bf09e534a719de136cd45d8a199c230a"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-17T19:51:00Z"
artifact_snapshot_sha256 = "036cb68d08c03e5b3bbdea66b7458bcf3a11b8e253a7d5a907a1d08f647106ee"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-003/README.md", "docs/engineering/simulation/evidence/WO-MOK-003/additivity-proof.txt", "docs/engineering/simulation/evidence/WO-MOK-003/boundary-and-security-review.md", "docs/engineering/simulation/evidence/WO-MOK-003/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-003/dependency-review.txt", "docs/engineering/simulation/evidence/WO-MOK-003/export-fidelity.txt", "docs/engineering/simulation/evidence/WO-MOK-003/exports.txt", "docs/engineering/simulation/evidence/WO-MOK-003/exports/events-seed0-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-003/exports/events-seed1-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-003/exports/events-seed123-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-003/exports/events-seed42-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-003/exports/events-seed777-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-003/frames.txt", "docs/engineering/simulation/evidence/WO-MOK-003/layout-and-viewports.txt", "docs/engineering/simulation/evidence/WO-MOK-003/manual-assessment.md", "docs/engineering/simulation/evidence/WO-MOK-003/non-perturbation.txt", "docs/engineering/simulation/evidence/WO-MOK-003/requirement-to-test-mapping.md", "docs/engineering/simulation/evidence/WO-MOK-003/resilience.txt", "docs/engineering/simulation/evidence/WO-MOK-003/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-003/terminal-restoration.txt", "docs/engineering/simulation/evidence/WO-MOK-003/test-run.txt"]

[relations]
verifies_work_order = ["WO-MOK-003"]
conforms_to = ["VER-MOK-003"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-003` to candidate commit `b3fed6a7bf09e534a719de136cd45d8a199c230a`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## What this record claims

`WO-MOK-003` is `in_progress` and `VER-MOK-003` is `approved`. At candidate commit
`b3fed6a7bf09e534a719de136cd45d8a199c230a`, **every automated case, invariant, static check,
security check and resilience check in `VER-MOK-003` was executed**, and each row of its
requirement-to-evidence matrix is mapped to a named test or a retained file in
`requirement-to-test-mapping.md`.

| Gate | Result |
|---|---|
| `cargo test --workspace` | 161 passed, 0 failed, 0 ignored (48 + 4 engine, 109 observer) |
| `cargo test -p mokiterions-core` | 52 passed, 0 failed |
| `cargo fmt --all -- --check` | no differences |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | 0 findings |
| `cargo build --workspace` and `-p mokiterions-core` | clean |
| `cargo tree -p mokiterions-core` | empty on every edge kind |
| Artifact validation | PASS — 44 artifacts, 0 errors, 0 warnings |

Four obligations `VER-MOK-003` singles out, each measured rather than argued:

- **Additivity.** All three engine test modules are byte-identical to `48d16bd4` and their test-name
  sets are unchanged, so the stop condition — a test that must change — did not occur.
- **Non-perturbation.** Observed and unobserved runs are identical on seeds `0`, `1`, `42`, `123` and
  `777`, with a frame drawn every tick and an interaction applied every tick.
- **Export fidelity.** The observer's exports are byte-identical to the engine binary process's own
  stdout across ten comparisons.
- **Layout.** All six declared canvas interiors measured exactly as derived — 67 × 32, 67 × 32,
  47 × 32, 71 × 36, 98 × 24, 32 × 16 — with `33 × 21` refused before the terminal is entered.

## What this record does not claim

**No manual assessment in `VER-MOK-003` was executed. All seven are outstanding and none has an
author.** `VREC-MOK-002` was able to claim that every case *and manual assessment* had been
executed; this record cannot, and the difference is deliberate rather than an omission in drafting.

`manual-assessment.md` records the reason. Three of the seven require an interactive terminal, and
the observer cannot be driven from this environment: `crossterm` on Windows reads the console input
buffer, so a keypress piped to the process never reaches it — measured as exit code 124, 7,017 bytes
of frame output and empty stderr — and `winpty` cannot supply a pseudo-terminal without a tty to
read its size from. The remaining four are judgments about what a person perceives, and recording my
reading of a buffer dump as a pass would restate an automated assertion in prose, which is the exact
gap `VER-MOK-003`'s residual-uncertainty section says cannot be automated away.

The consequence for this record is specific. `VER-MOK-003` states that a pass on every automated case
combined with a negative answer to the two-hundred-tick instrument assessment "is an adverse
observation requiring product review, because the automated cases verify the parts and this assesses
the instrument". The automated half of that condition is satisfied. The other half is unknown. So
this record establishes that the parts behave as specified; it does not establish that the instrument
answers the questions `INT-MOK-003` names, and it does not establish that anything is legible on a
real terminal.

Every automated result bound here is an assertion about an in-memory character buffer.

## What the accountable assurance owner must weigh before verifying

The outstanding manual assessments above are the first item. Nine further findings were disclosed
before this record was prepared. None is a failure against `VER-MOK-003`; each is stated so that
verification, if given, is given knowingly.

1. **`ADR-MOK-002` line 198 is factually wrong.** It says `Cargo.lock` acquires 57 entries; the file
   has 182. The conclusion it draws from the number holds, and holds more strongly. The number does
   not. Correcting it amends an approved artifact.
2. **The 57-crate figure is one of three readings** and appears eleven times across `SPEC-MOK-002`,
   `ARCH-MOK-002` and `ADR-MOK-002` as though it were the only one: 57 on normal edges, 59 with build
   edges, 37 excluding proc-macros.
3. **`SPEC-MOK-002` rules 4 and 5 cannot both be satisfied literally.** Rule 4's roster mockup needs
   about 87 columns of bars plus a reserved fourth slot; rule 5 gives the pane a 45-column interior.
   Resolved as `min(20, (interior − 27) / 3)`, which makes the reserved slot zero-wide at the
   reference roster — absent there rather than empty, which is not what rule 4.5 describes.
4. **`SPEC-MOK-002` rule 2 does not hold as written.** The public surface exposes two `&mut self`
   operations that change simulation state, not one: `advance_tick` and `run`. `run` is the
   `REQ-MOK-010` whole-run entry point and became publicly reachable when the library target was
   added. The observer never calls it. Narrowing it would require relocating the engine's sources,
   which `WO-MOK-003` excludes from scope.
5. **`VER-MOK-003` acceptance scenario 2 describes an unreachable state.** No shipped decision source
   can have a proposal rejected, asserted over 400 ticks of both policies, so the rejection
   presentation is verified only through a `#[cfg(test)]` hook. Manual assessment 4 therefore cannot
   be performed by running the observer at all.
6. **A world with no standing resources is likewise unreachable** and is verified through a second
   `#[cfg(test)]` hook. Both hooks are compiled out of the shipped binary.
7. **`REQ-MOK-021`'s hidden-roster-entry case is unreachable at every declared viewport** and is
   verified by code inspection only. Twelve entries never overflow a 32- or 36-row interior.
8. **`SPEC-MOK-002` rule 9.2's territory subject filter is not expressible** with the engine's
   records, whose subjects are `M##`, `F####` and `world`. Closing it needs a filter dimension that
   reads the `result` field, or a change to the engine's subjects.
9. **Four matrix rows had no test behind them at the first evidence pass** and were closed by four
   new tests before this record was prepared. An audit that found four is not evidence that there is
   no fifth.

Two further coverage limits, narrower than the above but relevant to how much the green gates mean:
non-perturbation is measured under a script driving 13 of the observer's 25 controls, and the
arrow-key aliases `Left`, `Right`, `Up` and `Down` are bound in `state.rs` and exercised by no test
in the package.

## Behavioural observations that are not failures

- Seed `42` under `--policy baseline` reaches extinction on tick 142, which is why the long-run case
  and manual assessment 1 use `--policy reference`.
- Under `--policy reference`, 11 of 12 Mokiterions die by tick 10,000. Living count plus deaths sums
  to 12 throughout, so `REQ-MOK-017`'s corroboration contract holds. World viability at the default
  density is `INT-MOK-002`'s subject, not this work order's.

## Scope of the transition being requested

Transitioning this record to `verified` would record that the assurance owner accepts the automated
evidence at this candidate commit **with all seven manual assessments outstanding**. It would not
constitute performance of those assessments, and it would not release, tag, publish or deploy
anything. If the assessments are to gate verification instead, this record should stay `ready` until
they are performed and their author recorded in `manual-assessment.md`.
