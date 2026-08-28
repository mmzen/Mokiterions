# The repository can transition again

Not a `VER-HUP-001` assessment. Recorded because *frozen* is the state `INT-HUP-001` names as the
thing to avoid, and leaving it is what this adoption existed to do.

## Before

Every lifecycle transition, of every artifact, was refused before it planned anything:

```text
Blocked by
- WEX201: current artifact graph is invalid [E012]: field 'evaluator_evidence_path' must be a non-empty string
```

That is a repository in which nothing can be approved, implemented, verified or released — including the
repair of the record that caused it.

## After

`WEX201` is gone. What remains is an ordinary lifecycle rule, which is a different kind of answer:

    harnessctl transition --set WO-HUP-001=implemented --decision WO-HUP-001="engineering owner" .

```text
Blocked by
- QGS-EDGE: transition WO-HUP-001: approved -> implemented is not allowed
```

The graph is valid and the evaluator is now reasoning about *this* transition rather than refusing all of
them. Under `0.8.0`'s work-order lifecycle the path runs through `in_progress`, and that edge plans:

    harnessctl transition --set WO-HUP-001=in_progress --decision WO-HUP-001="engineering owner" .

```text
Outcome
Completed.
```

Both commands above were run **read-only**, without `--apply`. No lifecycle state was changed by either, and
`WO-HUP-001` is still `approved`.

## What is owed

Moving `WO-HUP-001` to `implemented` is the engineering owner's act, and under `0.8.0` it is two transitions
rather than one:

1. `approved` to `in_progress`
2. `in_progress` to `implemented`

Neither is performed here. The work order's *Lifecycle* section places `implemented` after the applied
transaction and its retained evidence, both of which are now present.
