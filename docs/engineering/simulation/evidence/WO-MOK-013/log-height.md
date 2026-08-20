# Log height — WO-MOK-013

Measured 2026-08-20 on this branch, by the oracle retained beside this file as `wo013-oracle.rs`.
Every figure below is counted out of a rendered character buffer: a pane's height is the number of
terminal rows between its top border and its bottom border inclusive, located by those borders and
not by the layout's constants. The capture the figures are read out of is retained as `frames.txt`.

The before form is `evidence/WO-MOK-005/frames.txt`, captured against the commit `VREC-MOK-005`
records. Its heading for the reference viewport reads

```
## 160 x 48 — roster, inspector, log 10 rows      (line 16)
```

while the other five headings that name a log pane already read `log 6 rows`. That one heading is
the whole of what decision 1 of 2026-08-20 withdrew, and `SPEC-MOK-003` rule 5 as amended the same
day now presents the log at six rows wherever it is present.

## The log at every declared viewport, before and after

| Viewport | Log before | Log after | Records drawn after | `H ≥ 38` |
|---|---:|---:|---:|:--:|
| **`160 × 48`, the reference** | **10 rows** | **6 rows** | **4** | yes |
| `160 × 44` | 6 rows | 6 rows | 4 | yes |
| `160 × 40` | 6 rows | 6 rows | 4 | yes |
| `140 × 44` | 6 rows | 6 rows | 4 | yes |
| `140 × 43` | 6 rows | 6 rows | 4 | yes |
| `120 × 48` | 6 rows | 6 rows | 4 | yes |
| `120 × 30` | absent | absent | — | no |
| `100 × 30` | absent | absent | — | no |
| `34 × 22` | absent | absent | — | no |

One row moved. Six of the nine renderable declared viewports present a log pane and all six present
it at six rows; the three that do not present it are the three below `H = 38`. Presence agrees with
the threshold in both directions, which is the second half of what `REQ-MOK-024` obliges — *no
viewport presents it at any other height* — and it is measured over the nine declared sizes here
rather than over the plane, which `verification.rs`'s sweep does.

The log pane's own capacity is its height less the two rows its border takes, so six rows carry
four records. The before capture measures eight records in the ten-row pane at the reference
viewport, and `SPEC-MOK-003`'s amendment row of 2026-08-20 states the 4 against that 8. **This is
the cost of the trade and it is not a defect**: the reference viewport now shows half as much of the
event stream, and the stream itself is unaffected — the export retains 7,534 records at 300 ticks,
which `non-perturbation.txt` measures.

## What the four rows bought

`REQ-MOK-020` obliges twelve entries without scrolling at the reference viewport. `SPEC-MOK-003`
rule 4 as amended makes a roster entry three lines, so twelve entries need 36 interior rows. The
roster's interior is the body height less its two border rows, and the body is the viewport height
less 3 header rows, 1 footer row and the log's rows.

| Viewport | Log | Roster interior | Entries drawn | Pane title |
|---|---:|---:|---:|---|
| **`160 × 48`, the reference** | **6** | **36** | **12** | `living 12  deaths 0` |
| `160 × 44` | 6 | 32 | 10 | `living 12  deaths 0  hidden 2` |
| `160 × 40` | 6 | 28 | 9 | `living 12  deaths 0  hidden 3` |
| `140 × 44` | 6 | 32 | 10 | `living 12  deaths 0  hidden 2` |
| `140 × 43` | 6 | 31 | 10 | `living 12  deaths 0  hidden 2` |
| `120 × 48` | 6 | 36 | 12 | `living 12  deaths 0` |
| `120 × 30` | absent | 24 | 8 | `living 12  deaths 0  hidden 4` |
| `100 × 30` | absent | 24 | 8 | `living 12  deaths 0  hidden 4` |

`36 / 3 = 12` exactly. Two of the eight roster-bearing viewports hide nothing, and the reference
viewport is one of them — the requirement is stated at that size and is kept there.

**The counterfactual is measured rather than computed.** A ten-row log at `160 × 48` leaves a body
of `48 − 3 − 1 − 10 = 34` rows and a roster interior of 32, and a roster interior of 32 rows with a
three-line entry is a geometry the tree still produces today, at `160 × 44` and at `140 × 44`. Both
draw **10** entries and both title the pane `hidden 2`. So the ten-row log would not have cost the
reference viewport an approximation or a margin: it would have drawn `hidden 2` in the roster title
at the size where `REQ-MOK-020` forbids anything to be hidden, and the pane would have said so.

The before capture's own reference frame does not show this, because there the entry was two lines:
32 interior rows held twelve two-line entries in 24 rows with 8 rows of slack, and the roster title
read `living 12  deaths 0` with nothing hidden. The slack is what the three-line entry consumed.

## There is no slack in either direction

A seventh log row leaves 35 interior rows, which holds eleven whole entries, and on the titling
behaviour measured above the pane would read `hidden 1`. A fifth would leave 37, which fits twelve
with one row unused and no thirteenth subject to put in it — `SPEC-MOK-001` fixes the population at
twelve and calls a table longer or shorter than it a defect. So six is the only
height at which the log is as tall as it can be while `REQ-MOK-020` holds at the reference
viewport, and `SPEC-MOK-003` rule 4 item 1 as amended states the 36 interior rows as a provision so
that a later change to either height fails written text rather than costing an entry quietly.

This is also why `verification.rs`'s `RENDERABLE` reference row moved from `(160, 48, 67, 32)` to
`(160, 48, 67, 36)` in this work order: the canvas takes the four rows the log gave up, and the
interior at the reference viewport is `67 × 36`. The other eight interior figures are unmoved, and
`frames.txt` re-derives all nine rather than assuming the eight.

## What was not done

`evidence/WO-MOK-005/frames.txt` is **not corrected**. Its `log 10 rows` heading was true of the
commit it was captured against, and this repository's rule is that a capture is re-run rather than
edited — the precedent is `evidence/WO-MOK-011/merge/` and `SPEC-MOK-003`'s own amendment row of
2026-08-20, which declines to touch any file under `evidence/` for the same reason. `VREC-MOK-005`
is likewise not edited: it is `ready` and bound to its commit. The re-run is `frames.txt` in this
pack, and the record that supersedes `VREC-MOK-005`'s figures is `VREC-MOK-013`, which is captured
against the commit that carries this implementation and is not written here.

## How it was captured

The oracle placed a `#[cfg(test)]` module in `mokiterions-tui/src/`, drew each declared viewport
into an in-memory buffer at tick 200 of the reference run, wrote the frames out, and was removed;
`mokiterions-tui/src/lib.rs` carries no trace of it and `git diff origin/master` on that file is
empty. The oracle asserts nothing. It reports what it drew, and the readings above are made against
its output and against the before capture, both of which are retained in full.
