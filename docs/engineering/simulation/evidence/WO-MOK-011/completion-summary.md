# `WO-MOK-011` — completion summary

| Field | Value |
|---|---|
| Work order | `WO-MOK-011` (Phase 2.5, naming) |
| Implements | `REQ-MOK-040`, `REQ-MOK-041` |
| Specifications amended | `SPEC-MOK-001`, `SPEC-MOK-003`, and `SPEC-MOK-004` under its own rule 11 |
| Verification contract | `VER-MOK-011` |
| Baseline | `524a6758d74b5240079959e9827ea40a7af22a30` |
| Branch | `feature/phase-2-5-naming` |
| Date | 2026-08-19 |
| Toolchain | cargo 1.97.1 (c980f4866 2026-06-30); rustc 1.97.1 (8bab26f4f 2026-07-14) |
| Written by | the implementation agent |

The sections below follow `WO-MOK-011`'s *Completion report format*, in its order. Every figure is a
measurement retained in this packet and is cited to the file that holds it.

---

## 1. What changed, file by file — and what deliberately did not

### Changed

| File | + | − | What |
|---|---|---|---|
| `mokiterions-core/src/simulation.rs` | 212 | 1 | `NAMES`, `name_of`, the field, the record field and its rendering, the assignment, seven internal tests |
| `mokiterions-core/tests/naming.rs` | new | — | three public-tier tests |
| `mokiterions-tui/src/state.rs` | 28 | 0 | the name map, its single writer in the existing ingest path, the `pub(crate)` lookup |
| `mokiterions-tui/src/spatial.rs` | 16 | 19 | `agent_glyph` takes a name and returns its first character uppercased |
| `mokiterions-tui/src/render.rs` | 82 | 15 | the map draw, the roster entry, the inspector heading, one internal test |
| `mokiterions-tui/src/verification.rs` | 121 | 3 | two glyph call sites, one internal test |
| `mokiterions-tui/tests/verification.rs` | 270 | 2 | one glyph call site, three public-tier tests |
| `mokiterions-tui/tests/spatial.rs` | 22 | 11 | `glyphs_are_the_assigned_ones` asserts the new derivation |
| `SPEC-MOK-001.md` | 62 | 5 | five provisions, one amendment row |
| `SPEC-MOK-003.md` | 50 | 16 | four provisions, one amendment row |
| `SPEC-MOK-004.md` | 57 | 5 | the recorded figures, one amendment row |
| `SIMULATION_RULES.md` | 34 | 14 | the derived rules text |
| `docs/mokiterions/ROADMAP.md` | 61 | 0 | Phase 2.5 |

New governance artifacts, none of them modifications: `INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`,
`REQ-MOK-041`, `VER-MOK-011`, `WO-MOK-011`, and this evidence packet.

The engine's whole naming path, in four lines of production code:

    simulation.rs:396    name: &'static str,                  field of `Mokiterion`
    simulation.rs:418    const NAMES: [&str; 12] = [ … ]      the table SPEC-MOK-001 fixes
    simulation.rs:429    fn name_of(number: u8) -> &'static str    NAMES[usize::from(number) - 1]
    simulation.rs:1396   name: name_of(number),               the writer, at construction
    simulation.rs:1656   name: agent.name.to_string(),        the reader, at record emission

`static-checks.txt` item 3 is the search that finds no third site.

### Deliberately not changed

- **`AgentSnapshot`** carries no name. It is the observer's view of authoritative state, and a name
  is not state the engine computes per tick; the observer holds its own map instead, so a name is
  never presented as a snapshot value. This is why the engine's public interface is untouched
  (§6), and it is the same decision `AgentSnapshot` already embodies for `waste_tolerance`.
- **`Observation`** carries no name: `simulation.rs:500`–`515` is unchanged, and `SPEC-MOK-001`'s
  *Name* records the reason as identical to `fear`'s — a decision input that no rule reads is a
  claim the rules cannot support. Nothing in the engine reads a name, so nothing could read one
  from an observation.
- **`PerceivedMokiterion`** likewise untouched.
- **`bar_width` and `BAR_ROW_OVERHEAD`** are unchanged: `BAR_ROW_OVERHEAD` is still 35 and
  `bar_width(interior) = min(20, (interior − 35) / 4)` still gives 2 at the reference roster's
  45-column interior. The name occupies six columns of **line one only**. Measured, not argued: 96
  bar rows at nine viewports, `observer/bar-rows.diff` empty (§7).
- **`SPEC-MOK-002`** needs no amendment, which was the point of the design. Its rule 5 interface is
  identical at both revisions (§6), and its rule 8's closing sentence already admits
  `tests/naming.rs` as a further public-tier target without obliging the rule's arrangement table.
- **The identifier** is retained everywhere it appeared. The roster gains six columns at the head of
  line one and the identifier moves from `x=1` to `x=7`; it is not replaced. The inspector heading
  gains `Zug` and two spaces before `M01`. Every log line, every exported record, every retained
  stream and every citation still joins on `M01`–`M12`, and no record kind other than
  `agent_initialized` carries a name at all (§3, §5).
- **No rule, decision source, constant, floor, attribute, ordering or exit code.** `static-checks.txt`
  items 4–7: no added production line touches the stream, no ordering is keyed on a name, the acting
  order is still `for agent_index in 0..self.agents.len()`, no floating point entered the naming
  path, and both dependency tables and both resolutions are unchanged.

---

## 2. The twelve names, as the engine reported them

From `analysis/names-per-seed.txt`, read out of the engine's own tick-0 records at each declared
seed — not from a fixture:

| Identifier | Name | Initial |
|---|---|---|
| `M01` | `Zug` | `Z` |
| `M02` | `Krul` | `K` |
| `M03` | `Quib` | `Q` |
| `M04` | `Sput` | `S` |
| `M05` | `Trok` | `T` |
| `M06` | `Womp` | `W` |
| `M07` | `Hozz` | `H` |
| `M08` | `Nurb` | `N` |
| `M09` | `Vonk` | `V` |
| `M10` | `Gorm` | `G` |
| `M11` | `Xob` | `X` |
| `M12` | `Drix` | `D` |

- seeds `0`, `1`, `42`, `123` and `777` each report **the same twelve pairings in the same order**;
- seed 42 at three densities × three decision sources — nine combinations — reports the identical
  pairing;
- twelve names, twelve distinct initials, one to five ASCII letters each, and no name is also an
  identifier. `simulation.rs::the_names_are_the_specified_twelve` asserts all of those against a
  table written from the specification, plus that the table's length is the population a run
  actually builds.

`analysis/name-occurrences.txt`, on the binary's own output over 1,000 ticks with tracing on, under
each of the three sources: **twelve** occurrences of `name:`, all on tick 0, all on
`agent_initialized`, none on a survival, trace, decision, consumption, regeneration, death or summary
line, and none on the summary. Eleven event kinds appear in those runs and carry no name.

---

## 3. Oracle 1 — projected byte comparison, 90 cells

`additivity.txt`, **RESULT: PASS**. The matrix is `VER-MOK-002`'s five declared seeds × three
decision sources × three densities × tracing off and on, at 1,000 ticks: 90 cells, captured by
`capture.sh` before the first code change and again after it.

| Check | Result |
|---|---|
| cells compared | 90 of 90; 90 in the post-change capture |
| `sha256(project(post)) == sha256(pre)` | **True on all 90** |
| exit codes | equal on all 90 (`0 == 0`) |
| bytes removed per cell by the projection | exactly 118, on every cell |

118 is 46 characters of name plus twelve `name:` keys and their commas, which accounts for the entire
difference between the two captures and leaves nothing unexplained.

**Both captures' provenance is measured, not asserted** (`baseline/capture-state.txt`). The
pre-change capture was reproduced from a tree extracted with
`git archive 524a675 | tar -x`, which carries the commit's trees and no working-tree state at all:
**90 of 90 cells** match `baseline/pre-manifest.txt` on raw digest, byte count, line count and exit
code together. The post-change capture was reproduced from the restored tree after the mutation
control: **90 of 90 cells** match `post/post-manifest.txt` on those four columns and the projected
digest as well, which also measures that the revert was complete. That is `WO-MOK-011`'s
"worktree tracked-clean at capture" in a checkable form.

**The no-op check, stated separately.** `sha256(project(pre)) == sha256(pre)` is **True on all 90
cells**: applying the projection to the *pre-change* streams changes not one byte. A projection that
altered a pre-change byte would be hiding a difference rather than removing an addition, which is the
one way this oracle could be subverted. The projection is `baseline/projection.py`, whose docstring
is the reviewed artifact: a single anchored pattern, `result=name:[A-Za-z]{1,5},` → `result=`. It is
deliberately *not* anchored on the event kind, so a name leaking onto a second record kind would be
exposed by §2's occurrence count rather than silently deleted here.

Whether the projection could mask something else is manual assessment 5, which is **outstanding**
(§10). The independent evidence that the checks can fail at all is the mutation control:

**Acceptance scenario 2** (`analysis/mutation-control.txt`, **RESULT: PASS**). The naming step was
deliberately perturbed to draw one value from the shared stream, and reverted. Under the
perturbation, two of the ninety cells were recaptured and projected with the same unmodified
projection:

    seed0-baseline-d0.15-traceoff    projected  ef8004ba…  vs pre 562c210b…   DIFFERS
    seed42-individual-d0.75-traceon  projected  24304140…  vs pre 04008184…   DIFFERS

The first divergent line is `M02`'s placement, and 11 of the 38 initialization lines move. The source
was restored and re-measured to sha256
`cca972343a13d4b04cd643052baaee55a97127847bc67a719a1105f8b7f8ef30`; the restored build then
reproduced both cells' post-change digests exactly.

---

## 4. Oracle 2 — entropy neutrality, against the recorded stream positions

The recorded positions are `INITIALIZATION_DRAWS` at `simulation.rs:3285`, the expectations
`VER-MOK-007` oracle 2 pinned **before naming existed** and which this work order does not edit:

| Density | seed 0 | seed 1 | seed 42 | seed 123 | seed 777 |
|---|---|---|---|---|---|
| `0.15` | 72 | 72 | 72 | 72 | 72 |
| `0.75` | 270 | 268 | 268 | 268 | 268 |
| `1.50` | 516 | 516 | 516 | 514 | 516 |

Two instruments check them, and both pass on the delivered tree:

1. `simulation::tests::trait_derivation_leaves_the_shared_stream_where_it_found_it` — inherited,
   unmodified, 15 cells (3 densities × 5 seeds);
2. `simulation::tests::naming_draws_nothing_and_reads_neither_the_seed_nor_the_configuration` — new,
   45 cells (3 densities × 5 seeds × 3 policies). At every one: the shared stream is at the recorded
   position, the twelve names are the same twelve in the same order, and each is the one belonging to
   its own identifier.

Together they say the assignment reads neither the seed nor the configuration and moves nothing.
The static half is `static-checks.txt` item 4: no added or removed production line in
`simulation.rs` mentions `entropy`, `next_u64`, `splitmix`, a salt or the seed — every hit the filter
returns is a doc comment or a test line.

That these instruments can fail is measured, not assumed. Under the mutation control both failed,
with the arithmetic the perturbation predicts:

    naming_draws_nothing_and_reads_neither_the_seed_nor_the_configuration
      assertion `left == right` failed: naming moved the shared stream at seed 0, density 0.15%
        left: 84   right: 72

    trait_derivation_leaves_the_shared_stream_where_it_found_it
      assertion `left == right` failed: the shared stream is not where initialization at seed 0,
      density 0.15% left it before this change
        left: 84   right: 72

84 = 72 + 12, one extra draw per Mokiterion. Every name test passed under that perturbation: a suite
of name tests alone would have accepted the defect.

---

## 5. Oracle 3 — the census, reconciled name by name, and the positional parsers

`analysis/census-reconciliation.txt`. Both logs come from **one** `cargo test` invocation at the
workspace root, never assembled from per-package runs.

    193 before      205 after      12 additions      0 removals      0 renames
    205 passed, 0 failed, 0 ignored; no `#[ignore]` in either package

The reconciliation is by *qualified* name, so a case that moved between tiers or targets would show
as one addition and one removal even at an unchanged total. None does.

**The twelve additions, every one named:**

Engine, internal tier — `mokiterions-core/src/simulation.rs`, four:

1. `the_names_are_the_specified_twelve`
2. `naming_draws_nothing_and_reads_neither_the_seed_nor_the_configuration`
3. `the_reported_record_carries_the_name_the_agent_holds`
4. `a_name_is_the_same_value_at_both_ends_of_a_run`

Engine, public tier — `mokiterions-core/tests/naming.rs`, three:

5. `every_run_reports_the_specified_twelve_names_in_identifier_order`
6. `the_initialization_record_keeps_the_name_first_and_the_trait_last`
7. `a_name_is_reported_once_and_on_no_other_record`

Observer, internal tier, two:

8. `render::tests::an_entry_carries_the_name_before_the_identifier_and_takes_six_columns`
9. `verification::a_subject_whose_record_was_never_ingested_is_presented_without_a_name`

Observer, public tier — `mokiterions-tui/tests/verification.rs`, three:

10. `every_pane_identifying_a_mokiterion_presents_its_own_reported_name`
11. `every_glyph_drawn_is_its_own_subjects_initial_in_both_zooms`
12. `the_inspector_identifies_a_dead_subject_by_name_and_identifier`

Three of the twelve exist because `VER-MOK-011`'s coverage matrix was read back against the
implementation and three rows were found to have no case: additions 4, 9 and the population clause of
addition 1. The matrix found them, not the implementation.

Split by package: engine 85 (54 internal, 31 public), observer 120 (34 internal, 86 public),
workspace 205 — the figures `SPEC-MOK-004` rule 11 now records.

**The byte comparison of the two positional parsers** (`analysis/positional-parsers.txt`). The record
gained a field at its head, so the two engine suites that read the stream by rendered text rather
than by parsing keys into a map had to be shown *unedited*, not merely passing — passing after an
edit would prove nothing about a consumer this repository does not contain.

| File | working tree, LF form | baseline blob | verdict |
|---|---|---|---|
| `mokiterions-core/tests/process.rs` | `08d64608…b1c3555a` | `08d64608…b1c3555a` | IDENTICAL |
| `mokiterions-core/tests/viability.rs` | `04e3546e…f3e64f67` | `04e3546e…f3e64f67` | IDENTICAL |

(`core.autocrlf = true` in this clone, so the raw working-tree digest differs from the blob by line
endings alone; both digests are recorded in that file and the comparison is made on the LF form.)
`git diff --numstat <baseline> -- mokiterions-core/tests/` returns nothing, and
`git status --porcelain` on that directory lists only the untracked `naming.rs`. Neither parser
indexes by ordinal: `process.rs` counts `,fear:0,waste_tolerance:` and `viability.rs` reads the trait
with `rsplit_once(",waste_tolerance:")`, both anchored on the tail the new field cannot move. That is
why the name is *first* and `waste_tolerance` stays *last*, and
`tests/naming.rs::the_initialization_record_keeps_the_name_first_and_the_trait_last` asserts that
shape as a contract rather than leaving it an accident.

---

## 6. Oracle 5, first half — the public interface, item for item

`interface.txt`. One tool, `analysis/interface.py`, measured both revisions, the baseline sources
extracted with `git show` into a scratch directory.

| Package | Revision | items | public fields | `pub` lines |
|---|---|---|---|---|
| observer | pre | 101 | 24 | 125 |
| observer | post | 101 | 24 | 125 |
| engine | pre | 49 | 43 | 92 |
| engine | post | 49 | 43 | 92 |

**The engine:** `diff core-pre.txt core-post.txt` is empty. No item entered or left, and no item's
rendered declaration changed by one character. That is what `SPEC-MOK-002` needing no amendment
rests on.

**The observer:** the diff is one line, and it is a parameter's name.

    57c57
    < spatial.rs       item  pub fn agent_glyph(id: &str) -> char {
    ---
    > spatial.rs       item  pub fn agent_glyph(name: &str) -> char {

`&str` before and after, `char` before and after: the item's shape is unchanged and no caller outside
the crate is affected. One line of 125.

Reconciliation with `SPEC-MOK-004` rule 6's recorded 94 / 118 / 24 — the rule counts the seven
presentation modules and not `lib.rs`'s seven `pub mod` declarations:

    101 items    − 7 `pub mod` =  94   = rule 6's Total
    125 pub lines − 7          = 118   = the same interface counted the other way
    24 public fields                   = rule 6's field figure

and module by module, 5 + 3 + 10 + 8 + 2 + 19 + 47 = 94 with 0 + 0 + 7 + 4 + 0 + 6 + 7 = 24 fields,
reproducing the rule's table exactly. `Observer::name_of` is absent from both enumerations because it
is `pub(crate)`, which is the decision that keeps the figure at 94: the observer reads names through a
crate-internal accessor, and a test wanting it is an internal-tier test rather than a reason to widen
anything. Stop condition 5 therefore never triggered.

---

## 7. Oracle 4 — rendered buffers, cell by cell

`observer/frames.txt`, with the full captures in `frames-pre.txt` and `frames-post.txt` (916 lines
each) and the harness retained as `observer/frame-dump.rs`. Every claim is asserted against an
in-memory character buffer from ratatui's `TestBackend`. **No screenshot, recording, terminal or
pseudo-terminal is involved**, which is what `REQ-MOK-029` and `SPEC-MOK-004` require. The harness
compiles against both revisions and was run in this tree and in a scratch tree extracted with
`git archive 524a675`, so both sides come from one harness. Viewports are `VER-MOK-005`'s nine:
`160x48`, `160x44`, `160x40`, `140x44`, `140x43`, `120x48`, `120x30`, `100x30`, `34x22` (`33x21` is
below the floor and renders nothing). Configuration: seed 42, `--start-paused`, one advance,
reference policy, density 0.75.

**Roster cell positions at `160x48`**, pane `x=0 y=3 w=47 h=34`, a 45-column interior:

| Subject | identifier cell | the six columns before it | engine reported |
|---|---|---|---|
| `M01` | `x=7, y=4` | `\|Zug   \|` | `Zug` |
| `M02` | `x=7, y=6` | `\|Krul  \|` | `Krul` |
| `M03` | `x=7, y=8` | `\|Quib  \|` | `Quib` |
| `M04` | `x=7, y=10` | `\|Sput  \|` | `Sput` |
| `M05` | `x=7, y=12` | `\|Trok  \|` | `Trok` |
| `M06` | `x=7, y=14` | `\|Womp  \|` | `Womp` |
| `M07` | `x=7, y=16` | `\|Hozz  \|` | `Hozz` |
| `M08` | `x=7, y=18` | `\|Nurb  \|` | `Nurb` |
| `M09` | `x=7, y=20` | `\|Vonk  \|` | `Vonk` |
| `M10` | `x=7, y=22` | `\|Gorm  \|` | `Gorm` |
| `M11` | `x=7, y=24` | `\|Xob   \|` | `Xob` |
| `M12` | `x=7, y=26` | `\|Drix  \|` | `Drix` |

At the baseline the identifier is at `x=1` on every one of those rows and nothing precedes it. The
name is read from the records in the harness, never from a literal. `Xob` at three characters pads
with three spaces, `Krul` with two; the longest name is five, so nothing truncates in six columns.
Line one's fixed fields total 28 of 45, and the longest line one observed anywhere in the capture is
38 columns (`Xob   M11  B  123:115       move:north`).

**The bar-row comparison:**

    grep ' h .*  s .*  e .*  f ' frames-pre.txt  | sort > bars-pre
    grep ' h .*  s .*  e .*  f ' frames-post.txt | sort > bars-post
    diff bars-pre bars-post
    (no output)                     96 rows on each side

96 bar rows across nine viewports, identical, `observer/bar-rows.diff` empty. Stop condition 4
therefore never triggered: no bar width, pane width or layout threshold moved.

**The map**, at `160x48`, canvas `x=48 y=4 w=67 h=32`: the same twelve occupied cells before and
after, each drawing a different glyph — `(54,4) Z` for `M01` where the baseline drew `1`, through
`(108,34) G` for `M10` where it drew `A`. Twelve distinct glyphs before, twelve after; the baseline's
were base-13 digits `1`–`9`, `A`, `B`, `C`, the candidate's are the initials. Across all nine
viewports and both zooms the only letters or digits anywhere in the canvas are
`D G H K N Q S T V W X Z` — exactly the twelve initials and nothing else. The resource glyphs are
`○`, `◎`, `●` and the territory rule is `─`, none of them a letter, so stop condition 8 never
triggered.

**The inspector**, `x=116 y=3 w=44 h=34`: the heading reads `Zug  M01` where the baseline read `M01`,
and rows `y=3`, `y=5`–`y=13` and `y=36` are byte-identical to the baseline's. For a **dead** subject
(`M01`, died tick 119, baseline policy) the heading is `Zug  M01` and `died on tick 119` and
`final health 0  satiety 0  energy 81` are byte-identical to the baseline's.

The whole diff is 570 lines, and every one is a roster or inspector row whose head gained six columns
or a name, a printed cell position that moved from `x=1` to `x=7`, a canvas glyph that changed from a
digit to an initial, or the harness's own report of what the records said. No pane rectangle, no bar
row, and no pane's presence at any viewport changes.

---

## 8. The four gates

`gates.txt`, all four at the delivered tree, sources touched first so nothing is a cached result.

| Gate | Command | Result |
|---|---|---|
| format | `cargo fmt --all -- --check` | exit 0, no output |
| lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, no warning in either package |
| dependencies | `cargo tree -p Mokiterions` | `Mokiterions v0.1.0` alone — `SPEC-MOK-002` rule 1's empty table measured |
| | `cargo tree -p mokiterions-tui --depth 1` | the path dependency plus `ratatui v0.30.2` |
| tests | `cargo test` (one invocation, workspace root) | exit 0; **205 passed, 0 failed, 0 ignored** |

Per-target results are in `gates.txt`, the full log in `post/test-run.txt`, the census in
`post/test-census.txt`. Naming added no dependency to either package, which is what a hard-coded
table costs.

---

## 9. The amendment records, row by row

Three specifications carry a new row. Each row is quoted from the specification's own *Amendment
record*; the full text is in the artifacts and the material clauses are reproduced here with the
approval each carries. `amendment-approvals.md` is the measurement that each row is present in both
the record and the body, that no earlier row was edited or reordered, and that no commit-bound record
was re-opened — **RESULT: PASS**, 9 of 9 controls.

### `SPEC-MOK-001`, 2026-08-19 — five provisions

> Naming, under `CAP-MOK-008`. Five provisions. *Scope* names `CAP-MOK-008` and per-Mokiterion
> identity. *State model / Mokiterion* gains the name. A new *Name* subsection fixes the twelve names
> and their assignment to `M01` through `M12`, the character and length domain, the pairwise
> distinctness of the names and of their twelve first characters, and the three load-bearing
> properties: the assignment reads neither the seed nor the configuration, naming performs no draw
> against any generator, and nothing in the engine reads a name. It also records that a name is not
> carried on the rule 3 observation, for the reason `fear` is not. *Time and entropy* records that
> naming is not an exception to the single shared stream because it is not a draw at all. *Data and
> interface contracts* puts `name` first in the `agent_initialized` details, before `position`,
> reports it once and on no other record kind, and states that both the name's leading position and
> `waste_tolerance`'s trailing position are fixed because two test suites parse that record
> positionally. Rule 1 places the assignment at agent creation. **No other provision changed: no
> rule, no decision source, no constant, no floor, no attribute, no ordering and no exit code is
> touched, and every run's outcome and entropy sequence are unchanged.**

**Approval:** "Approved 2026-08-19 by the repository owner acting as technical owner, together with
`INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`, `REQ-MOK-041`, `VER-MOK-011` and `WO-MOK-011`. The
twelve names and their assignment are the product owner's decision of the same date, recorded in
`WO-MOK-011`; the name's position in the record and the decision that the observer sources it from
the retained event stream rather than from a new public interface item are the technical owner's
decisions of the same date, recorded there too. The implementation agent wrote the amended text under
`WO-MOK-011` and did not decide the substance."

### `SPEC-MOK-003`, 2026-08-19 — four provisions

> Four provisions amended under `REQ-MOK-041`, so the observer presents the name `REQ-MOK-040` makes
> the engine report. **Rule 2's glyph tables**: the detail table's `M01`–`M09 → 1`–`9` and
> `M10`–`M12 → A, B, C` rows are replaced by the name's first character uppercased, with `?` for a
> subject whose name was not received, and the overview layer table's "the identifier's last
> character" becomes "the name's first character" — the two zooms derive one glyph and had drifted,
> since the withdrawn overview rule gave `M10` a `0` where the detail table gave it an `A`. The twelve
> resulting glyphs `Z K Q S T W H N V G X D` are stated, resting on `SPEC-MOK-001`'s twelve
> pairwise-distinct first characters, which is what rule 2.5 needs […] **The anticipation is retained
> rather than deleted**: the withdrawn table and the sentence promising that "when agent naming is
> introduced by a later phase, the glyph becomes the name's first character and this table is
> amended" are quoted in place, because they are why the old assignment was correct while no name
> existed. **Rule 4**: the entry mockup and prose carry the name first, then the identifier, in the
> two-line form and in the collapsed one-line form, and the addition to the identifier rather than
> replacement of it is stated with its reason — the identifier is the join key into the log, the
> export and every retained stream. **Line two is measured to be untouched**: the name occupies six
> columns of line one only, so the bar row's five leading columns, its 35-column overhead and
> `bar_width(interior) = min(20, (interior - 35) / 4)` are unchanged and the reference roster's
> two-cell bars stand; line one's fixed fields total 28 columns of a 45-column interior, so nothing
> truncates. **Rule 10**: the presented-value list gains the name, before the identifier and for a
> dead subject as well as a living one, under rule 10.6's retained selection; item 7 loses `name`,
> because the engine now reports one […] **Nothing else changes**: no pane threshold, no floor, no
> layout figure, no key binding, no export form, no authority mapping row, no snapshot field, no
> interface item, and no obligation that layout be a pure function of viewport size.

**Approval:** "Approved 2026-08-19 by the repository owner acting as technical owner, together with
`INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`, `REQ-MOK-041`, `VER-MOK-011` and `WO-MOK-011`. The
twelve names are the product owner's decision of the same date; the decision that the name is
presented in addition to the identifier and precedes it, and that the observer sources it from the
retained event stream rather than from a new engine interface item, are the technical owner's,
recorded in `WO-MOK-011`. The implementation agent wrote the text and did not decide the substance;
rule 2's glyph assignments are withheld from it by `WO-MOK-005`'s decision envelope. **The 2026-08-18
row marked OUTSTANDING above is untouched**, as are the two rows recording the merge reconciliation
and `WO-MOK-007`'s outstanding capture re-derivation."

### `SPEC-MOK-004`, 2026-08-19 — the recorded figures

A third amendment, beyond the two the work order named. It is not a discretionary one: rule 11 states
the obligation itself — "a work order that adds a test corrects these figures here, and one that
loses a test has a defect" — so a work order adding twelve tests must write this row.

> **Recorded test-count figures corrected for `WO-MOK-011`, which adds twelve tests for `REQ-MOK-040`
> and `REQ-MOK-041`.** […] **Rule 9**: `tests/verification.rs` rises from 16 to **19**, so the public
> tier rises from 83 to **86** […] **Rule 10**: `src/render.rs` rises from 12 to **13**,
> `src/verification.rs` from 8 to **9**, and the internal tier from 32 to **34** […] **Rule 11**: the
> observer's executed total rises from 115 to **120**, the engine's from 78 to **85**, and the
> workspace's from 193 to **205** […] `mokiterions-core/tests/naming.rs` is recorded as a new engine
> public-tier target, admitted by `SPEC-MOK-002` rule 8's closing sentence […] **Rule 6 is unchanged
> at 94 items, 118 `pub` lines and 24 public fields**, measured rather than assumed: `Observer::name_of`
> is `pub(crate)` and `spatial::agent_glyph` keeps the `&str` parameter and `char` return it already
> had, so this work order adds no member of the interface and changes no member's shape. Nothing else
> changes — no target name, path, package name, tier boundary, hook or prohibition, and no item's
> visibility widens.

**Approval:** "Approved 2026-08-19 by the repository owner acting as technical owner, together with
`INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`, `REQ-MOK-041`, `VER-MOK-011`, `WO-MOK-011` and the
same-day amendments to `SPEC-MOK-001` and `SPEC-MOK-003`. Every figure in this row is a measured
outcome rather than a decision […] The implementation agent measured, wrote this text and decided
none of it. **The two rows above marked OUTSTANDING are untouched**, and no record bound to a commit
is re-opened."

Those two untouched `OUTSTANDING` rows belong to `WO-MOK-005`/`WO-MOK-006` and to `WO-MOK-007`; this
work order neither edits nor discharges them, and `amendment-approvals.md` verifies their status text
is unchanged and in place.

---

## 10. The seven manual assessments

`manual-assessment.md` records each one, with the role it belongs to and the measurement it was
decided against. `VER-MOK-011`: "An unrecorded assessment is an outstanding assessment, and this
contract is not satisfied while any remains outstanding."

| # | Assessment | Role | State |
|---|---|---|---|
| 1 | the names themselves | product owner | recorded 2026-08-19 |
| 2 | the assignment | product owner | recorded 2026-08-19 |
| 3 | legibility of the named entry | technical owner | recorded 2026-08-19 |
| 4 | retaining the identifier | product owner | recorded 2026-08-19 |
| 5 | **the projection** | **assurance owner** | **OUTSTANDING** |
| 6 | the glyph change | technical owner | recorded 2026-08-19 |
| 7 | the absence of a name reader | technical owner | recorded 2026-08-19 |

Six are discharged by the repository owner's approval of 2026-08-19, because the artifact each one
governs states the substance of the judgement and the owner approved that text in the same act as
`WO-MOK-011`.

**Assessment 5 is outstanding and is owed by the repository owner acting as assurance owner.** What
is to be assessed is `baseline/projection.py`: that it deletes only the field this change adds, and
that it could not mask a change to a position, an identifier, an event kind, an ordering, an
attribute value or a line's presence. It could not be discharged by the 2026-08-19 approval because
the projection is evidence written *after* that approval, described by no approved artifact, and the
implementation agent cannot assess its own instrument. What the owner would be deciding against: the
no-op result on all 90 pre-change streams, the 118-byte-per-cell accounting, the 90 byte-identical
projected comparisons with equal exit codes, and now the mutation control, which shows the oracle
rejecting a naming step that draws. `VER-MOK-011` itself records under *Residual uncertainty* that
neither mitigation is a proof, which is why the assessment exists.

`VER-MOK-011` is therefore **not yet satisfied**, on this one count and no other. Nothing in the code
or the evidence is blocked on it: the outstanding item is a judgement, not a measurement.

---

## 11. Residual uncertainty

**Stop and escalate conditions.** None of the eight triggered, and each is measured rather than
assumed: 1 — the census shows 12 additions, 0 removals, 0 renames and no assertion relaxed (see the
disclosure below); 2 — 90 of 90 cells byte-identical after projection with equal exit codes; 3 — the
shared stream is at its recorded position at every seed, density and policy; 4 — 96 bar rows
identical and no pane rectangle moved; 5 — the interface is 94 / 118 / 24, unchanged; 6 —
`SPEC-MOK-002`'s interface is identical at both revisions, so no amendment was needed; 7 — no name
was changed, added, removed or reassigned for any check; 8 — the only letters in the canvas are the
twelve initials, and no resource or territory glyph is a letter.

**Open, and disclosed rather than fixed:**

1. **Manual assessment 5 is outstanding** (§10). `VER-MOK-011` is unsatisfied until the assurance
   owner records it.
2. **Two existing inline tests were edited beyond the work order's "and only these" list.**
   `WO-MOK-011` names `tests/spatial.rs`'s glyph case and three glyph call sites as the only tests to
   be updated. In `mokiterions-tui/src/render.rs`, `the_bar_row_reproduces_the_specified_form` and
   `a_zero_value_is_a_zero_and_an_absent_value_is_a_dash` also changed, because `entry_lines` gained
   a `name` parameter — which the same work order instructs — and a call site must pass it. Each
   gained one argument, a fabricated stand-in (`"Blip"`, `"Ort"`); no assertion of either was
   weakened, removed or relaxed, and neither test's name changed, so neither appears in the census.
   `static-checks.txt` item 1 records both. Whether that is inside stop condition 1's "beyond the
   three glyph call sites named above" is the technical owner's call, not the implementation agent's;
   it is reported here rather than left to be found.
3. **Two test fixtures held engine names and were corrected in the sources.**
   `tests/spatial.rs::glyphs_are_the_assigned_ones` listed the twelve names against their glyphs —
   exactly the table `REQ-MOK-041` forbids the observer to hold — and the `render.rs` inline tests
   first passed real names. Both now assert more than before: a derivation over all 26 initials in
   either case and at three lengths, and a column arithmetic over every admissible name length.
   `static-checks.txt` item 1 is the search that now returns nothing.
4. **`VER-MOK-011`'s wording deviates in one place, and the deviation is in this packet's favour.**
   The contract asks that the name table's length be asserted "against the population constant". The
   engine has no population constant; the population is a function of the density input. The
   assertion in `the_names_are_the_specified_twelve` therefore compares the table's length against
   the population a run actually builds, which is a stronger check than a comparison against a
   literal would be. No approved text was amended for this.
5. **`mokiterions-tui/tests/export.rs::a_written_file_holds_exactly_the_rendered_records` is
   intermittently flaky on this platform**, failing at `fs::read_to_string` with `NotFound` after
   `write_file` returned `Ok`. It was observed failing at the baseline commit and passing in the
   recorded 193-test baseline census, so it is pre-existing. A test change is a stop condition here,
   so it is reported rather than adjusted. It passed in the recorded 205-test run.
6. **This tree does not carry `master`'s `WO-MOK-007`, `VER-MOK-007` and `VREC-MOK-007`.** Those three
   — the roster survival band chain, with a `verified` record — exist on `origin/master` and are
   absent here, where the same three identifiers hold the individuality chain instead, and
   `evidence/WO-MOK-007/` likewise holds the individuality packet. Restoring them is the owner's act
   on this branch; this work order neither touches nor cites them.
7. **Oracle 1's captures are reproducible rather than retained whole.** The 90 raw pre-change streams
   (110 MB) are not committed. Retained instead: `baseline/pre-manifest.txt` and
   `post/post-manifest.txt` with per-cell raw and projected SHA-256, byte and line counts and exit
   codes; the tick-0 initialization block of all 90 cells on both sides; and three whole 1,000-tick
   streams per side. `capture.sh` at the recorded commit regenerates the rest, and the digests detect
   a failed reproduction. This is the retention form `VER-MOK-011` and `baseline/compare.py` state,
   and a reviewer wanting a raw stream re-derives it — `baseline/capture-state.txt` records that both
   sides were re-derived in exactly that way, 90 of 90 cells each.
8. **The evidence packet was restructured to the work order's stated paths** after being first
   written under `analysis/` and `candidate/`: `candidate/` → `post/`, the manifests to
   `baseline/pre-manifest.txt` and `post/post-manifest.txt`, and `additivity.txt`, `interface.txt`
   and `static-checks.txt` to the packet root. All internal references were rewritten and every
   referenced path verified to exist. `README.md` indexes what is here.
9. **A concurrent session has run `git reset` in this clone twice** during this work. Nothing in this
   packet depends on the index, and every figure is measured against the baseline commit by digest
   rather than against working-tree state, but a reviewer should re-run the gates before trusting the
   tree they find.

**`VREC-MOK-011` is a separate commit-bound record that this work order does not write and cannot
self-approve.** `WO-MOK-011` classifies commit-bound verification as `required`. The verification
record must be captured against the commit that carries this implementation, by the assurance owner,
and it cannot be written as satisfied while manual assessment 5 remains outstanding. The
implementation agent has produced measurements and drafts; it holds no authority over intent,
architecture, verification or release, and this summary asserts no verification outcome.
