# Release evidence for Mokiterions 0.1.0

This directory holds the retained evidence for `VREC-MOK-009`, the aggregate verification record for
the 0.1.0 release candidate, commit `79512b4aba0a444c23605157bc4da4f2f7eb435b`.

It is deliberately **not** filed under any work order. The eight work orders in this release were
each implemented, verified and recorded separately, and their evidence stays where it was retained,
under `docs/engineering/simulation/evidence/WO-MOK-<id>/`. Nothing here supersedes any of it.

What an aggregate record adds is the one reconciliation the per-work-order records cannot state
about themselves: **each of them binds a different commit, and none of them binds the commit being
released.** `docs/RELEASE_RUNBOOK.md` Phase D is where that is explained. This directory states, per
work order, that the commit its own verified record binds is contained in the candidate, what
changed afterwards, and whether anything in that change is uncovered.

## Why these file names carry work-order IDs

Because the harness requires it, and because the requirement is easy to misread — this directory's
naming is the correction of a real refusal, not a style choice.

`capture-verification` refuses an aggregate record unless every selected work order has at least one
evidence path keyed to its own ID, and it tests the **file name**, not the directory:

```python
def _evidence_is_keyed_to(evidence_path: str, work_order_id: str) -> bool:
    return re.match(rf"^{re.escape(work_order_id)}(?:-|\.|$)", Path(evidence_path).name) is not None
```

— `se_harness/provenance.py`, guarded by `if len(selected_work) > 1`, so it fires only for
aggregates.

Every verification record in this repository before this one covers exactly one work order, so the
rule had never been exercised here, and the entire evidence tree keys by **directory**:
`evidence/WO-MOK-001/completion-report.md` is keyed to `WO-MOK-001` by any reading a person would
give it, and by none that the regex gives it. The first genuine aggregate capture therefore refused
all eight work orders at once:

```console
$ python -m se_harness capture-verification . --id VREC-MOK-009 ... \
    --evidence docs/engineering/simulation/evidence/WO-MOK-001/completion-report.md ...
harnessctl: aggregate evidence is not keyed to work orders: WO-MOK-001, WO-MOK-002, WO-MOK-003,
WO-MOK-004, WO-MOK-005, WO-MOK-006, WO-MOK-007, WO-MOK-009
$ echo $?
2
```

Those were the runbook's own Phase D arguments, unmodified. The runbook stated the rule — *"each
work order needs at least one evidence path keyed to its own ID"* — directly above a worked example
that violated it, so the example was not executable and the sentence was ambiguous about what
"keyed" tests. Both are corrected in the same commit that adds this directory.

There was a shortcut available and it was not taken. Passing each work order's own artifact,
`docs/engineering/simulation/work-orders/WO-MOK-001.md`, satisfies the regex and the
file-existence check with no new files at all. A work order is a definition of work, not evidence
that the work is contained in a release candidate, so that would have satisfied the check without
satisfying the rule. These files state something instead.

## Files

| File | What it records |
| --- | --- |
| `candidate-checks.md` | Every gate and check run against the candidate commit, with results |
| `WO-MOK-001-containment.md` | Containment of `VREC-MOK-001`'s commit; the widest gap, and the late assurance classification |
| `WO-MOK-002-containment.md` | Containment of `VREC-MOK-002`'s commit |
| `WO-MOK-003-containment.md` | Containment of `VREC-MOK-003`'s commit; the crate split that moved the earlier work orders' paths |
| `WO-MOK-004-containment.md` | Containment of `VREC-MOK-004`'s commit; the smallest surface |
| `WO-MOK-005-containment.md` | Containment of `VREC-MOK-005`'s commit |
| `WO-MOK-006-containment.md` | Containment of `VREC-MOK-006`'s commit; verified before `VREC-MOK-005` but binding an older commit |
| `WO-MOK-007-containment.md` | Containment of `VREC-MOK-007`'s commit; the only work order whose code is byte-identical to what was verified |
| `WO-MOK-009-containment.md` | Containment of `VREC-MOK-008`'s commit, **and the three post-verification edits no other record covers** |

Read `WO-MOK-009-containment.md` first if you are reading only one. It is the only file here that
records a gap rather than a containment.

## What this evidence is not

- It is not a re-verification. No work order's scenarios were re-run for this release; each was
  performed once, against its own commit, and its own record holds the result.
- It is not an approval. `VREC-MOK-009` is captured as `ready`. Only an accountable assurance owner
  may transition it to `verified`, and no command does that.
- It does not claim the release process has been run. `VREC-MOK-008` stated that the release
  machinery had never been exercised end to end, and that is still true when this evidence is
  written. Running it is what this release does.
