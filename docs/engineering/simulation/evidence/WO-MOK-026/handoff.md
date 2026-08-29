# `WO-MOK-026` handoff evidence

The binding this work order's `handoff` checkpoint requires, and the reading it was taken from.

artifact: WO-MOK-026
checkpoint: handoff
formal_snapshot_sha256: f5aba26a7f29ae1533dea39c4afbc20855bc1b8679160bea74134b51dfb18ef0

## How the digest was obtained

**Measured, not quoted.** The digest is read off the gate itself at the commit this file is
committed above, rather than copied from an earlier commit message:

    $ harnessctl check --artifact WO-MOK-026 --checkpoint handoff --changes-complete \
          --changed-path ... (the complete branch change set)

      Blocked by
      - QGP-G4I-EVIDENCE: No readable evidence for WO-MOK-026, checkpoint handoff, and formal
        snapshot f5aba26a7f29ae1533dea39c4afbc20855bc1b8679160bea74134b51dfb18ef0 is available.

    evaluator  ~/se-harness-eval-080/Scripts/harnessctl.exe, version 0.8.0
    root       .engineering-harness.toml, schema 2, tool_version 0.8.0
    read on    2026-08-29, natively on Windows 11

**Re-deriving it was not optional.** The snapshot moved four times during this work order --
`47aad296…` before `SPEC-MOK-002` was admitted to scope, `670b8733…` before `SPEC-MOK-007` was,
`d3400ab2…` before the last `SPEC-MOK-004` re-measurement, and `f5aba26a…` after it. Three of those
figures appear in this branch's own amendment rows and commit messages and each was true when
written. **A digest quoted from any of them would bind this evidence to a tree that no longer
exists**, and the gate would refuse it -- which is the safe direction, but only if the digest is
measured rather than carried forward hopefully.

## Why it did not move again

The commit this file follows retains twenty-two evidence entries and touches **no formal artifact**:
no specification, no requirement, no architecture record, no verification contract and not the work
order itself. The formal snapshot covers the artifact set, so evidence under
`docs/engineering/simulation/evidence/` is outside it and the digest is unchanged from the value
`6e9ca13`'s message recorded. That is stated here because it is the reason a packet of 22 files
could be added without re-binding anything, and it is measured above rather than assumed from the
rule.

## The change set this checkpoint was evaluated over

    $ git diff --name-only origin/main...HEAD

115 paths at the commit below this file; 116 with this file itself, which is why the check is run
twice -- **this evidence joins the change set it is evidence for**, so the first run's set is
necessarily one path short of the set the passing run declares. The declared set is the fixed point:
the second run's, with this file in it.

`QGP-G4I-PATHS` is silent over both sets. Every path falls inside this work order's
`[execution_scope]`, including the four admitted by owner amendment on 2026-08-28 and 2026-08-29 --
`mokiterions-core/Cargo.toml`, `docs/CONNECTOR_PROTOCOL.md`,
`docs/engineering/simulation/specifications/SPEC-MOK-002.md`,
`docs/engineering/simulation/specifications/SPEC-MOK-007.md` and
`mokiterions-tui/src/state.rs`. `QGP-G4I-EVIDENCE` was the only gate speaking, and this file is its
corrective input.

## What this file does not assert

It binds a snapshot to a checkpoint. It does not assert that any verification case passed:
`verification-cases.txt` is that record and it carries one **FAILED** row, `L15b`, along with two
escalations and a deviation. A green handoff checkpoint and a failing verification case are
consistent, and this packet keeps them apart deliberately -- the checkpoint is about whether the
work order's own governance is in order, and `L15b` is about whether the provider gives the discount
`REQ-MOK-070` requires. It does not.
