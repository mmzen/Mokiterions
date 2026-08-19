# Defect report: the provenance footer silently drops contract fields

Recorded 2026-08-19 against `master` at commit `54c21ab`, while designing release CI. Found
by measurement, not by review: nothing in the suite fails today.

This report states what was observed and what it contradicts. It decides nothing. Which
fields must survive at the narrowest viewport is a `SPEC-MOK-003` rule 8 question and
belongs to the technical owner.

## Summary

`draw_footer` selects the widest of four candidate renderings that fits the pane. When none
fits, an `unwrap_or_else` produces a fifth string that is not one of the four and is not
specified anywhere. That string omits the resource density and the active decision source.
`SPEC-MOK-003` rule 8 requires both.

The fall-through is reachable on `master` with no change to the repository and no unusual
build: an operator only has to choose a large seed.

## Location

`mokiterions-tui/src/render.rs`

| Line | What is there |
|---|---|
| 34 | `pub const COMMIT: Option<&str> = option_env!("MOKITERIONS_COMMIT");` |
| 683-684 | `commit` (`"  commit {commit}"`) and `short_commit` (`" #{commit}"`) |
| 690-724 | the four specified tiers: `long`, `medium`, `short`, `tiny` |
| 728-736 | `.find(|candidate| count(candidate) <= width)` then `.unwrap_or_else(...)` |

The fall-through string is:

```rust
format!("s{} t{} @{} e{}", config.seed, config.tick_limit, snapshot.tick, events.len())
```

Compared with `tiny`, it drops `d{density}%` and the decision-source letter.

## Observation 1 — a large seed is enough

A probe was inserted into `render::tests`, run, and reverted. It renders the footer row of a
34x22 frame, the narrowest declared viewport, with `MOKITERIONS_COMMIT` unset:

```
seed                   42 -> width 27 | s42 t100 d0.75% r @0 e136 #
seed 18446744073709551615 -> width 34 | s18446744073709551615 t100 @0 e136
```

The second row is the fall-through: no `d0.75%`, no decision source. It is 34 columns wide,
so it fits the pane and nothing reports a problem. `u64::MAX` is an accepted `--seed` value
by `SPEC-MOK-001`'s help contract, so this is ordinary operator input.

The first row also shows a second effect: the probe ran with `MOKITERIONS_COMMIT=` set to the
empty string, and `option_env!` returns `Some("")` for that, which appends a bare ` #` and
costs the `short` tier its place. An empty environment variable is not the same as an unset
one.

## Observation 2 — the commit stamp cannot be used at all

`COMMIT` exists so a build can carry its own provenance, and the footer has two forms ready
for it. It cannot be populated. At 34 columns:

- `short` renders `s42 t100 d0.75% reference @1 e160` — 33 columns, one column of slack.
- `short_commit` costs `" #" + commit`, a minimum of three columns.

So any non-empty stamp forces the drop to `tiny`, which abbreviates the decision source to
one letter. `mokiterions-tui/tests/verification.rs:486` requires the 34x22 footer to contain
`reference`. Measured by building and running `cargo test -p mokiterions-tui --locked` once
per value, forcing a recompile each time:

Two tests are involved, because they render at different seeds and assert different fields.
`every_declared_viewport_...` uses seed 42 and requires the word `reference`;
`the_footer_survives_the_narrowest_viewport` uses seed 0 and requires `d0.75%`.

| Stamp length | `every_declared_viewport_...` | `the_footer_survives_...` |
|---|---|---|
| 1 to 7 | fails | passes |
| 8 | fails | passes |
| 9, 10, 12, 16, 20, 40 | fails | fails |

The seed's digit count is what separates the two columns: seed 42 costs one more column than
seed 0, so the integration case loses its slack one character earlier.

There is no usable value. A release therefore cannot stamp its own commit into the binary
while `SPEC-MOK-003` rule 8 stands as written.

## Why the suite does not catch either one

- The declared seeds are small, so no case reaches the fall-through.
- Every test compiles with `MOKITERIONS_COMMIT` unset, so `COMMIT` is `None` and no test
  exercises either stamped form. The defect exists only in builds the suite never compiles.
- Neither package has a build script, so Cargo does not treat `MOKITERIONS_COMMIT` as a
  rebuild input. Setting it and re-running `cargo test` can reuse the unstamped artifacts and
  report a pass that means nothing. Each measurement above forced a recompile by touching
  `mokiterions-tui/src/*.rs` first.

## What this contradicts

- `SPEC-MOK-003` rule 8 (`### Rule 8 — Provenance footer`, line 372), which enumerates seed,
  configured tick limit, density as supplied, active decision source, current tick and
  retained-event count.
- `REQ-MOK-024`, "Degrade the layout across viewport sizes without losing authoritative
  information". The fall-through loses authoritative information rather than degrading.
- `REQ-MOK-027`, "Display run provenance and the authority for each event type".

`VER-MOK-005` verifies both requirements and did not catch this, so its case list is part of
the change surface, not only the renderer.

## Reproduction

```
git switch --detach 54c21ab
# Observation 1: insert a probe in render::tests that renders a 34x22 footer for
# --seed 18446744073709551615 and print it; the density and source fields are absent.
# Observation 2:
touch mokiterions-tui/src/render.rs
MOKITERIONS_COMMIT=0123456789ab cargo test -p mokiterions-tui --locked
```

## Not in this report

Whether the fix is a fifth specified tier, a field-priority order for shedding, a bounded
seed presentation, or a wider minimum viewport. Each changes `SPEC-MOK-003` rule 8, and
`SPEC-MOK-003` already carries an amendment row marked **OUTSTANDING** from 2026-08-18.
