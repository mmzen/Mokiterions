# Handoff checkpoint evidence

Retained for the `in_progress` to `implemented` transition of `WO-MOK-033`.

artifact: WO-MOK-033
checkpoint: handoff
formal_snapshot_sha256: 526baa6e54be0273f86adf52721373024461807b94bda1ca4be7554ac6636309

Assurance is `required`, decided by the engineering owner on 2026-08-30, so this work order does not
stop at `implemented`: `VREC-MOK-027` covers it and carries the disclosures below.

## The declared change set

```text
  docs/engineering/simulation/evidence/WO-MOK-033/          148 files, 147 of them in manifest.sha256
  docs/engineering/simulation/work-orders/WO-MOK-033.md
  scripts/classify_simulation_runs.py
  scripts/run_simulation_batch.py
  scripts/test_classify_simulation_runs.py
  scripts/test_run_simulation_batch.py
```

Every path was declared to the check individually; the directory line above stands for 148 files, of
which `manifest.sha256` lists 147 and is the one it cannot list. Four new Python files under `scripts/`,
this work order's own governance, and its evidence. **No file under either package's `src/`, neither
package manifest, neither `Cargo.toml`, not `Cargo.lock`, no workflow, and no other governance
artifact** -- which is `ADR-MOK-008`'s claim that no amendment is required, and phase D cases T1, T2
and T4 are the measurement of it.

The formal snapshot is taken over the repository's formal artifacts and not over retained evidence, so
naming this file in its own change set does not move the digest above. It was read from the transition
check before this file existed and reproduces after.

## What this hands off

**Four defects found and repaired**, three in the driver and one in its own suite: an explicit binary
path that was not normalised to the platform's separator, a test for that defect that passed against
the defect, one reusable stream path handed to two concurrent cells under `--jobs`, and a stream left
on disk when a cell was interrupted while the record said none was left. Each has a test that fails on
the pre-fix code, and `harness/bite/bite-output.txt` is the record that all three surviving cases do:
`3 of 3 tests fail on their own pre-fix copy`. Details in `findings/instrument-defects.md`.

**Nine gaps between `SPEC-MOK-008` and what was buildable, plus one wrong citation in
`VER-MOK-019`**, each built one way and disclosed rather than amended, because condition 6 of this
work order forbids amending an approved artifact on an implementation agent's judgement. Details in
`findings/disclosed-gaps.md`.

**Four findings about the swept space**, recorded and not acted on: `asymmetric_collapse` accounts for
32 of 400 cells where a 35-run reading saw none; the seed changes the outcome *class* in 10 of the 20
source-and-density groups, reversing the five-seed finding of 2026-08-30; the threat mechanism is
effective 8 times in 7,701 resolutions, which is the *before* figure for the repair chain the owner
deferred; and `retreat` remains an action with no resolved-event kind, so no sweep can report on it.
Details in `findings/sweep-findings.md`.

**Eight manual assessments are not recorded.** `VER-MOK-019` reserves them to the product, technical
and assurance owners and states that an unsigned assessment is not a recorded one. All 84 mechanical
cases pass; these eight are the judgements those cases cannot make, and an implementation agent cannot
sign any of them. `VREC-MOK-027` discloses the gap rather than closing it, and
`findings/manual-assessments.md` states what each assessment needs read.
