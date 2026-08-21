# The test census — `SPEC-MOK-004` rules 9, 10 and 11, corrected for two work orders

`test-census.txt` is the measurement. This file reconciles it against the three figures amendment 3
corrects, and it is the file that makes the joint correction auditable.

## Why three trees, and why one paragraph corrects two work orders

Rules 9, 10 and 11 last stated a tree correctly at **`f82cd3d`**, where the workspace ran **226** tests.
`WO-MOK-016` merged after that and did not correct them, so the figures in the rules describe a tree
that no longer exists — which is the condition amendment 3 exists to end. This work order's own three
arrivals cannot be stated without stating `WO-MOK-016`'s thirty-eight first: a paragraph reading "267,
of which this work order added 3" would leave 38 tests unaccounted for in a document whose rule 11 says
a work order that adds a test corrects these figures here.

The precedent is this rule's own: the 2026-08-19 amendment record row corrected `WO-MOK-010`'s and
`master`'s `WO-MOK-007` figures in one act, on the ground that neither was statable without the other.

| Tree | What it is | Engine | Observer | Workspace |
|---|---|---|---|---|
| `f82cd3d` | the tree rules 9 to 11 state | 85 | 141 | **226** |
| `f2a79e1` | this branch's HEAD: `WO-MOK-016` merged, this work order still a draft document | 122 | 142 | **264** |
| worktree | the implementing tree, both temporary oracles removed | 122 | 145 | **267** |

`f2a79e1`'s 264 is independently corroborated: `VREC-MOK-017` records the same figure at its own
candidate commit. The 38 attributed to `WO-MOK-016` therefore does not rest on this branch's tree alone,
which is what the work order's fourth stop-and-escalate condition asks for.

## The forty-one arrivals, attributed

| Target | `f82cd3d` | worktree | Arrivals | Whose |
|---|---|---|---|---|
| `mokiterions-core/src/simulation.rs` | 54 | 82 | +28 | `WO-MOK-016` |
| `mokiterions-core/tests/viability.rs` | 2 | 5 | +3 | `WO-MOK-016` |
| `mokiterions-core/tests/cli.rs` | 13 | 15 | +2 | `WO-MOK-016` |
| `mokiterions-core/tests/decisions.rs` | 1 | 3 | +2 | `WO-MOK-016` |
| `mokiterions-core/tests/process.rs` | 6 | 7 | +1 | `WO-MOK-016` |
| `mokiterions-core/tests/termination.rs` | 4 | 5 | +1 | `WO-MOK-016` |
| `mokiterions-tui/tests/verification.rs` | 20 | 22 | +2 | one each |
| `mokiterions-tui/tests/state.rs` | 21 | 22 | +1 | `WO-MOK-018` |
| `mokiterions-tui/src/state.rs` | 4 | 5 | +1 | `WO-MOK-018` |
| **total** | **226** | **267** | **+41** | 38 and 3 |

Every other target in both packages is unchanged in both count and names. `226 + 41 = 267`, so the
arrivals account for the whole movement and there is no residual to explain.

## The one rename, and why it is in neither figure

`WO-MOK-016` renamed `no_shipped_decision_source_has_a_proposal_rejected` to
`no_source_confined_to_the_valid_action_list_has_a_proposal_rejected` in
`mokiterions-tui/tests/verification.rs`. A rename is neither an arrival nor a departure, so it appears
in no column above; `SPEC-MOK-004` rule 12 is the authority.

**Attributing that file's two arrivals took a name-by-name diff and not a commit message.** The
`chore(WO-MOK-016)` commit `d72465c` adds no test to it: the twenty-first arrived in the merge commit
`259859d`. Pairing every `#[test]` with the `fn` that follows it, across `f82cd3d` and the worktree,
isolates the rename from the arrival, which reading either commit's diff does not. That is why
`test-census.txt` carries a name-level diff and not only counts.

## No departures, and no test loses its `#[test]`

Nothing departs on any of the three trees. `#[ignore]` appears **0** times anywhere in the workspace, so
no figure above counts a test that does not run — the executed totals in `gates.txt` are `267 passed, 0
failed, 0 ignored`, which is the same 267 counted statically here.

## The tier split, which is the cross-check

The observer's 145 must equal rule 10's internal total plus rule 9's public total, or one of the three
figures is wrong:

| Tier | Targets | Tests |
|---|---|---|
| internal (rule 10) | `src/render.rs` 20, `src/state.rs` 5, `src/verification.rs` 9, `src/main.rs` 8 | **42** |
| public (rule 9) | `tests/authority.rs` 4, `tests/export.rs` 7, `tests/layout.rs` 11, `tests/options.rs` 8, `tests/render.rs` 22, `tests/spatial.rs` 7, `tests/state.rs` 22, `tests/verification.rs` 22 | **103** |
| | | **145** |

The engine's 122 splits 82 internal and 40 public under `SPEC-MOK-002` rules 7 and 8, which this rule
does not govern and records only so that 267 is reproducible: `82 + 40 + 42 + 103 = 267`.

The observer's target count is unchanged at three internal-tier and eight public-tier targets. Every
arrival lands in a file rules 9 and 10 already name, so no row is added to either table — only three
cells move.

## Where this work order's three tests sit, and why each sits there

| Test | Tier | Why not the other tier |
|---|---|---|
| `a_death_carries_the_fear_the_engine_last_reported_for_its_subject` | public, `tests/state.rs` | it reaches `deaths()`, `events()` and `advance()`, all already public; nothing was widened for it |
| `the_inspector_presents_a_dead_subject_s_final_fear` | public, `tests/verification.rs` | it renders a frame and reads a pane, through `render::draw` and `layout::resolve`, both already public |
| `a_death_carries_no_attribute_the_engine_never_reported_for_its_subject` | internal, `src/state.rs` | it constructs a state no run reaches, which needs the private `ingest`; `ARCH-MOK-002` forbids widening it and forbids a fifth hook |

`SPEC-MOK-004` rule 8 places a test by the access it requires and not by the subject it covers, which is
why two tests about the same rule sit in two tiers. The third is not a convenience: the two ways to move
it to the public tier are both prohibited patterns of `ARCH-MOK-002`, named there in those words. It
renders a frame from where it sits, so the placement costs it no reach — `interface.md` and `inspector.md`
carry that.

## Two temporary oracles, and the tree these figures are measured on

Both were placed in the tree, run once, removed, and retained in this pack:

- `wo018-oracle.rs` — a child module of `state`, because it constructs the no-record state through the
  private `ingest`. Produced `inspector.txt`, `inspector-one-line.txt` and `filter-vocabulary.txt`.
- `wo018-non-perturbation-oracle.rs` — appended to `tests/verification.rs` so it could reuse that file's
  declared sets and helpers rather than reimplement the contract's comparison. Produced
  `non-perturbation.txt`.

Neither is in the tree the figures above are measured on, neither added a `pub` item, and neither is in
the tree the gates were run on. `cargo test --workspace` with the first present reports 269 and with the
second 268; both figures are artefacts of the capture and neither is a census figure.
