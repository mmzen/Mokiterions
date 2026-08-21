# The observer's interface — `SPEC-MOK-004` rule 6, re-counted

`interface.txt` is the enumeration. This file states what it shows, which is **not** what this work
order's *Evidence to record* said it would.

## The work order predicted the wrong result

The brief for this file reads: "the enumeration establishing that rule 6's 94 items, 118 `pub` lines and
24 public fields are **unmoved**." Two of those three figures moved.

| Figure | Predicted | Measured on the implementing tree |
|---|---|---|
| public items | 94 | **94** — unmoved |
| public fields | 24 | **25** |
| `pub` lines | 118 | **119** |

The prediction was wrong for a reason worth recording rather than absorbing: the change was scoped by its
effect on **items**, and on items the prediction holds — no `pub fn`, `struct`, `enum`, `const`, `static`,
`type`, `trait` or `use` arrives. But rule 6 counts public **fields** separately, in a second figure that
is one line per item plus one per public field, and `state::Death` gains one.

It rested on a second, more concrete error, which the work order's decision envelope states in its own
words: that a field on `Death` is "private to `mokiterions-tui::state`". It is not. `Death` is `pub` in a
`pub mod`, and its three sibling fields — `id`, `tick`, `health` — and the two optional attributes beside
them are all `pub`. So the envelope's grant of "the field name and type" was a grant over a member of the
public interface, framed as though it were an internal detail.

**This is why the figures were measured and not asserted.** Every earlier row of rule 6's amendment table
that reads "unchanged at 94, 118 and 24" says "measured rather than assumed" in the same sentence; had
this file quoted the rule it is checking, the defect would have shipped as a conformance claim.

## The one arrival

From `git diff` against `f2a79e1`, restricted to `pub` lines in the observer package, the whole of the
interface's movement is:

```
+    pub fear: Option<u8>,
```

`mokiterions-tui/src/state.rs`, in `pub struct Death`, beside the `satiety` and `energy` it already
declares. Nothing else in the diff writes `pub`.

## The enumeration, before and after

Both figures are read off a tree, not quoted. The before column is the tree at `f2a79e1` — this branch's
only commit, which carries the draft work order and no code — so the baseline is reproduced rather than
inherited from the rule under test.

| Module | Items | Fields | `pub` lines before | `pub` lines after |
|---|---|---|---|---|
| `authority` | 5 | 0 | 5 | 5 |
| `export` | 3 | 0 | 3 | 3 |
| `layout` | 10 | 7 | 17 | 17 |
| `options` | 8 | 4 | 12 | 12 |
| `render` | 2 | 0 | 2 | 2 |
| `spatial` | 19 | 6 | 25 | 25 |
| `state` | 47 | 7 → **8** | 54 | **55** |
| **total** | **94** | 24 → **25** | **118** | **119** |

The baseline reproduces rule 6's stated figures exactly — 94 items, 118 lines, 24 fields — and every row
of the rule's own per-module table row by row. That is what makes the after figure a correction of one
cell rather than a re-derivation of the whole rule.

The two figures self-check by construction and not by coincidence: every `pub` line the rule counts is
either an item or a public field, so `94 + 25 = 119`.

## What is excluded, and why the table sums to the rule's total

- **The engine's interface.** `SPEC-MOK-002` rule 5 closes that one; rule 6 counts the observer package
  only, and no figure here is a workspace figure.
- **`lib.rs`'s seven `pub mod` lines.** They are the module declarations themselves and are in neither
  figure — which is exactly what lets the seven rows above sum to the rule's own 94 and 119.
- **The four `#[cfg(test)]` hooks on the observer's state type.** Rule 6 excludes them because rule 7
  keeps them out of the library target. They are counted in a separate column of `interface.txt` and are
  **4 before and 4 after**: `ARCH-MOK-002` names adding a fifth as a prohibited pattern, and none was
  added.
- **Variants of public enums.** Written without `pub`, in neither figure. This is the distinction the
  2026-08-19 amendment settled, and it is why 119 is not reached as 122 − 97.

## The three alternatives, each measured and each worse

The narrowest available form was implemented. The others were not left as assertions:

| Alternative | What it costs |
|---|---|
| an accessor on `Observer` | adds a public **item** and moves 94 — which this work order's first stop-and-escalate condition forbids outright |
| `pub(crate)` beside two `pub` siblings | makes one struct partly opaque and pushes its public-tier case into rule 10 for no reason but visibility |
| derive it in `render` | impossible: the state it comes from is private, and reaching it is what rule 7 forbids |

The field is the only form that leaves the item count still. Rule 7 is untouched — a field added to a
struct whose fields are already `pub` widens no existing item's visibility — and rule 6's **Growth**
clause is satisfied by `REQ-MOK-021`, by way of `SPEC-MOK-003` rule 10.6 as amended on this date; the
clause's "a test is never that requirement" holds because the three tests follow the presentation rather
than justify the field.

## Status

The correction is recorded as **amendment 5** of `WO-MOK-018`, which had four when the owner approved it,
and it is **OUTSTANDING for the technical owner's ratification**. The owner has not been shown it. The
precedent is rule 6's own 2026-08-19 row, which was OUTSTANDING for the same reason — a consequence the
agent found after the fact by measuring the interface against the rule — until the owner ratified it
under `WO-MOK-012`.

The growth is nonetheless forced by the presentation the owner *did* approve. The alternative that avoids
it is prohibited by the work order itself, so the narrowest form was implemented and reported rather than
the work being left undone.
