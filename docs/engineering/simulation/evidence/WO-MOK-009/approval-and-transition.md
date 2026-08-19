# Approval and transition

The repository owner approved this work order's governing chain on 2026-08-19 and authorized the work order's
transition. This record states what was transitioned, on whose word, what was deliberately left alone, and what the
gates report on the transitioned tree.

**The implementation agent recorded the transitions. It did not make the decisions.** `docs/engineering/DECISION_RIGHTS.md`
places them outside what such an agent may exercise, and `ENGINEERING_HARNESS.md:44` states that harness commands may
prepare records but never exercise accountable decision rights. The instruction was: *"I approve the artifact pack
related to WO-MOK-009, then WO-MOK-009 can be switch to implemented"*. The precedent for an agent recording an
owner's approval is `VREC-MOK-006`, which cites the owner's *"You approve, I record it"* for nine transitions in the
`INT-MOK-005` → `WO-MOK-006` chain.

## What was transitioned

Ten artifacts, in one edit and one commit.

| Artifact | From | To | Accountable role |
| --- | --- | --- | --- |
| `INT-MOK-007` | draft | approved | product owner |
| `CAP-MOK-007` | draft | approved | product owner |
| `REQ-MOK-035` … `REQ-MOK-039` | draft | approved | product owner |
| `SPEC-MOK-005` | draft | approved | technical owner |
| `VER-MOK-008` | draft | approved | assurance owner |
| `WO-MOK-009` | draft | **implemented** | engineering owner |

`WO-MOK-009` went to `implemented` rather than stopping at `approved`, which is what the owner instructed and what
every completed work order in this repository carries — `WO-MOK-001` through `WO-MOK-006` are all `implemented`.
`docs/engineering/WORKFLOW.md` line 16 gives the normal path as `draft -> approved -> in_progress -> implemented`;
that path is recorded in the work order's *Approval record* rather than walked commit by commit, because the candidate
and its evidence were already complete when approval was given, so no revision of this repository would have honestly
described the intermediate states. Line 16 also settles the endpoint: *"Governance-only work that authorizes
verification, release, tagging, review, or publication stops at `implemented`"*, and this work order authorizes
release.

`SPEC-MOK-005`'s amendment record now names the technical owner against its original row, which previously read
*"Draft. Requires the technical owner."* The rule 12.5 row already named them.

## What was not transitioned, and why

- **`WO-MOK-008` remains `draft`.** It is the defect report against `SPEC-MOK-003`'s provenance footer, a separate
  work order with no implementation, and the owner approved the pack related to `WO-MOK-009`. Nothing here approves
  it. Its review preflight consequently still fails, which is correct: `REQ-MOK-036` requires review preflight to
  pass for every **released** work order, and a draft defect report is not in any release payload.
- **No verification record was transitioned to `verified`.** `capture-verification` emits `ready`, and
  `DECISION_RIGHTS.md:14` reserves `verified` and `released` to accountable owners. `VREC-MOK-007` is prepared from
  the revision this record is committed in and carries `status = "ready"`.
- **No tag, no release record, no release contract, no merge, no publication.** The four remain what
  `a5-refusal-ladder.md` shows them to be: reserved acts that no amount of approved governance substitutes for.

## The one item to check rather than accept

`WO-MOK-009`'s approval precondition 2 asks the technical owner to decide whether the release process needs an
architecture artifact, and says that if one is required, *"an architecture and a deciding record must exist and this
work order must gain the relation before approval"*. The owner approved the pack as written — no `architecture`
relation, no `ARCH`, no `ADR` — so the decision recorded is that none is required and `SPEC-MOK-005` governs the
release process alone.

**That is a reading of an approval, not a separately worded decision, and it is the weakest link in this record.**
It is stated here and in the work order's *How each precondition was discharged* section so that a reader meets it
rather than discovers it. The precedent cuts both ways: `WO-MOK-002` and `WO-MOK-004` carry no architecture relation,
while `WO-MOK-001`, `WO-MOK-003`, `WO-MOK-005` and `WO-MOK-006` do — and the trigger the precondition names,
deployment-and-operating-model, is one of the twelve the validator's own
`DECISION_TRIGGERS` vocabulary lists. If the technical owner intends an architecture artifact, the correction is an
`ARCH`, an `ADR`, the relation, and an amendment to both notes; the approval would then have been premature rather
than merely under-documented. No measurement in this directory changes under either reading.

## Gates on the transitioned tree

Re-run from the pinned `se-harness==0.4.0` wheel after the ten edits and before the commit.

```text
$ python scripts/validate_engineering_artifacts.py --root .
Engineering artifact validation: PASS
Artifacts: 79 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
exit=0

$ python -m se_harness doctor .
81 verdict lines: PASS 81, WARN 0, FAIL 0
exit=0

$ python -m se_harness preflight . --work-order WO-MOK-009 --phase review
Harness preflight: PASS
Phase: review
Work order: WO-MOK-009 (implemented)
exit=0
```

**The validator's silence is worth more here than before the transition.** `E007` and `E008` fire for a requirement
in an active status with no active specification or no active verification coverage, and draft artifacts are exempt.
Before the transition the five requirements were exempt; now they are checked, and `SPEC-MOK-005`'s `specifies` and
`VER-MOK-008`'s `verifies` are what answer for them. This is why approval precondition 1 required the chain to move
as a whole: the five requirements alone would have tripped both codes.

The artifact count is unchanged at 79 because a transition edits a field rather than adding a file.

The full rule 7.4 loop, which is what the release process runs:

| Work order | Review preflight |
| --- | --- |
| `WO-MOK-001` … `WO-MOK-006` | PASS, each `implemented` |
| `WO-MOK-008` | **FAIL**, `draft` — see above |
| `WO-MOK-009` | PASS, `implemented` |

The scenario suite was re-run rather than assumed, because six of its cases read the real repository rather than a
fixture: `python -m unittest discover -s scripts -p 'test_check_release_*.py'` → **Ran 70 tests, OK**, 44.480s.

## What this changes about the managed CI, and what it does not

Pull request #20's *Review preflight* step failed with ten findings — one `[W005]` naming `WO-MOK-009`'s `draft`
status as ineligible for review, and nine `[W013]` naming each draft governing artifact as not active. All ten
address exactly the statuses this record transitions, and the local review preflight above is the same check the step
runs. Nothing in the diff was ever among the findings, and nothing in the diff changed here.

The `governor` job and every other `candidate` step already passed, on all three commits, in both the push-event and
the pull-request runs.

## Two things that go stale, named so they are not read as current

1. **`a5-refusal-ladder.md`'s clone is at `54c21abc…`,** which holds none of this chain. Its transcript states that
   commit, and its rungs 0 and 1 hold at any revision of this repository for a reason independent of these statuses:
   no release record exists in any revision, so every tag is correctly refused. Rungs 2 through 5 are fixtures
   constructed inside the clone. A re-run today would clone a graph containing the approved chain and reach the same
   verdicts, because rule 4 reads release records and the work orders a release record names, and there is still no
   release record.
2. **`VREC-MOK-007` is now two different things in this directory.** The ladder's rung-3 fixture invents a
   *hypothetical aggregate* record under that ID, covering `WO-MOK-001` through `WO-MOK-006` at one candidate commit,
   to show that a verified aggregate still does not authorize a release. The real `VREC-MOK-007` prepared from this
   revision verifies `WO-MOK-009` alone. The ID is the next free one in the sequence and skipping it to avoid the
   collision would leave a gap that is harder to explain than the collision itself. When a release is eventually
   prepared, the aggregate record it needs will carry a later ID.

## What this record is not

It is not the verification record, and it does not stand in for one. `docs/engineering/WORKFLOW.md` line 22 states
that *"A record cannot contain the hash of its own commit"*, so `VREC-MOK-007` is captured from the commit that
contains this file and committed separately afterwards. It will be `ready`: prepared provenance awaiting the
accountable assurance owner, who is the only party that may make it `verified`.

This record takes the directory to sixteen files — fifteen records and the index. `commit-binding.md`'s closing count
of fifteen was correct when it was written.
