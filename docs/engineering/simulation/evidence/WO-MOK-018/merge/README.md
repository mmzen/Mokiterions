# `WO-MOK-018`: the merge of `master` into the Phase 4a chain, and what was re-derived on it

| Field | Value |
|---|---|
| Merge commit | `1e09f85664e69d092c0054098cae71f99b4e2694` — "Merge master into wip/pr31-integration for WO-MOK-018" |
| First parent | `fa0bfd9a4c15783628f521c6b2959e178ee7b8b3` — this branch, act 1's renumbering |
| Second parent | `fa065cc27aa250bd93c586b0c61da789dab49e33` — `master`'s tip, "Merge pull request #35 from mmzen/feature/phase-3-definition" |
| Merge base | `ff3a155f3ce006fdc38abb62df3fca4a2c3c3aa3` |
| Measured at | `e8114ad11884a56ccdd93f352644f804b4d010cc` — the merge plus act 3's `SPEC-MOK-004` correction; `COMMIT.txt` holds it |
| Branch | `wip/pr31-integration` |
| Date | 2026-08-21 |
| Toolchain | cargo 1.97.1 (c980f4866 2026-06-30); rustc 1.97.1 (8bab26f4f 2026-07-14) |
| Harness | se-harness 0.4.0, from `C:\Users\mathi\harness-venv-040` — the version this repository's workflows pin |

Every figure below is measured at `e8114ad` unless it names another commit. No source file differs
between `1e09f85` and `e8114ad`: `git diff 1e09f85 e8114ad --name-only` prints
`docs/engineering/simulation/specifications/SPEC-MOK-004.md` and nothing else, so every `cargo`
figure is equally a figure at the merge commit, and `gates.txt` says so where it matters.

## This is not a verification record and it takes no decision

Nothing here approves the merge, accepts a resolution, ratifies a figure or closes an obligation.
Every file in this directory is derived, read-only evidence: a command, its output, and the
reasoning needed to read the output. **`VREC-MOK-019`**, bound to the merge, is where a decision about
this tree is recorded, and it is not written yet — item 1 of *What is still owed* below.

**That identifier is provisional, and finding 10 is why.** This packet was drafted naming
`VREC-MOK-017`, which was free when act 1 renumbered the chain and was taken by `master` a few hours
later; `VREC-MOK-018` was then taken by a third branch while this packet was being corrected. `019`
is the lowest number free across all five remote heads as of the correcting commit, and it is not
reserved by anything. The number is to be re-derived across every ref at the moment the record is
written, never from this file and never from the local maximum.

The ten numbered findings below are worded as reports to the owner rather than as measurements, and
none of them is acted on here. Two of the ten are non-conformances of the merged engine against the
`SPEC-MOK-006` amendment this branch drafts — findings 8 and 9 — and the engine is deliberately left
as it is: the amendment reads OUTSTANDING, and changing the product to match an unratified
specification would be the wrong order. Finding 10 is not a property of this tree at all: it is a
collision between two branches, and it is recorded here because act 4 cannot be written without
resolving it.

## Why the merge is against `master`, and where it sits in the four acts

The repository owner authorized four acts on 2026-08-21, in one interactive session, and this
packet is the evidence of the third measured on the tree the second produced:

1. **Renumber** the chain from `WO-MOK-012` to `WO-MOK-018` — "018, nothing else moves" — done at
   `fa0bfd9`. `master` had already taken `013` through `017`, and `WO-MOK-016` is itself the same
   chain's renumbering out of four occupied ranges (`d72465c`), so the identifier space is shared
   and the local maximum was not the free number.
2. **Merge `master` and resolve** — done at `1e09f85`, and §*What the merge had to reconcile* below
   is its account.
3. **Re-derive the figures the merge made stale, by measurement** — the `SPEC-MOK-004` correction is
   `e8114ad`; this evidence sub-packet is the measurement behind it.
4. **Create a new verification record at the merge commit** — "New record at merge commit", not yet
   written.

The merge is against `master` rather than against PR #31's tip because `master` is what the branch
must merge into, and because three of this packet's figures are not statable on either parent alone.
`master` adds three event kinds and no record type; Phase 4a adds four record types and no event
kind; the record stream's `suffered` field exists on one side and the stream it is written to exists
on the other. `WO-MOK-011/merge/` set the precedent for measuring in the first tree that holds both
sides, and the 2026-08-19 `WO-MOK-010`/`WO-MOK-007` row of `SPEC-MOK-004`'s amendment record set it
for correcting two work orders' figures together when neither is statable without the other.

## What the merge had to reconcile: seven files, nineteen conflict regions

Recomputed rather than remembered. `git merge-tree --write-tree fa0bfd9 fa065cc` exits `1` and
writes tree `0c3973189a97249e2acd3a53d09dd37779820ebe`; a `git merge fa065cc --no-commit --no-ff`
in a detached worktree at `fa0bfd9` reproduces the same seven paths, and the region counts below are
`grep -c '^<<<<<<<'` over that result. Twelve files in the merge differ from both parents; seven of
them conflicted.

| Path | Regions | The largest region | What each side wanted |
|---|---:|---|---|
| `mokiterions-core/src/simulation.rs` | **5** | 1,795 lines — 527 ours, 1,265 theirs | Phase 4a's record-stream block and `master`'s combat block land in the same place. Regions 1 and 2 are 13 and 35 lines in the engine's action dispatch; regions 4 and 5 are 178 and 774 lines of the test module |
| `docs/engineering/simulation/specifications/SPEC-MOK-004.md` | 2 | 47 lines — 19 ours, 25 theirs | Both sides append to the same amendment record and to rule 11's paragraph sequence |
| `docs/engineering/simulation/architecture/ARCH-MOK-001.md` | 4 | 11 lines — 7 ours, 1 theirs | Both sides extend the same frontmatter relations and the same component list |
| `docs/ROADMAP.md` | 3 | 37 lines — 16 ours, 18 theirs | Both sides write the same phase's row and the same "what is next" section |
| `SIMULATION_RULES.md` | 2 | 31 lines — 14 ours, 14 theirs | Both sides add rules at the same anchor of the operator-facing summary |
| `docs/engineering/simulation/specifications/SPEC-MOK-002.md` | 2 | 31 lines — 7 ours, 21 theirs | Rule 5's enumeration on both sides, and `master`'s new rule 13 landing where Phase 4a appends |
| `docs/engineering/simulation/specifications/SPEC-MOK-001.md` | 1 | 9 lines — 3 ours, 3 theirs | Both sides declare a rule in the same numbered sequence |

**No conflicted file was resolved by taking a side.** Measured in both directions: for each of the
seven, `git diff --numstat` against `fa0bfd9` and against `fa065cc` is non-zero on both, so every
resolution keeps material from each parent rather than discarding one.

    path                  vs. ours (+/-)      vs. theirs (+/-)
    SIMULATION_RULES.md      205 / 39             62 / 2
    docs/ROADMAP.md          219 / 21            165 / 28
    ARCH-MOK-001.md           18 / 10             31 / 7
    SPEC-MOK-001.md          280 / 24             33 / 7
    SPEC-MOK-002.md          171 / 11             82 / 1
    SPEC-MOK-004.md          136 / 21             21 / 0
    simulation.rs          3,104 / 200        1,896 / 103

## The three files git merged and the two edited beyond it

The five source files git auto-merged without conflict were checked against what the merge actually
committed, which is the check a conflict list does not make: a clean auto-merge can still be wrong,
and an edit to a file that never conflicted is invisible in any conflict report.

| Path | Automatic result vs. committed merge |
|---|---|
| `mokiterions-core/src/cli.rs` | identical — taken exactly as git produced it |
| `mokiterions-core/src/lib.rs` | identical |
| `mokiterions-core/tests/cli.rs` | identical |
| `mokiterions-tui/src/render.rs` | identical |
| `mokiterions-core/tests/process.rs` | **+1 line** |
| `mokiterions-core/tests/records.rs` | **+138 / −34 lines, 7 hunks** |

**`tests/process.rs`, one line.** `master` added a call site of `execute` at
`the_social_source_runs_to_completion_and_exits_successfully`, and Phase 4a gave `execute` a fourth
parameter. The automatic merge produced a three-argument call in a four-parameter world, which does
not compile. The resolution adds `None,` — the sink absent — and nothing else.

**`tests/records.rs`, two tests strengthened and one helper added.** This is the file where Phase
4a asserts the record stream's shape, and `master`'s three combat resolutions put shapes into that
stream that the file's reference capture cannot reach. Git had nothing to merge — the file is new on
this branch only — so every line here is a hand edit and each is listed:

- a `social(label)` capture helper at seed 42, 300 ticks, `--trace-actions --policy social`, with its
  doc comment stating what that run resolves so the coverage claim is checkable rather than asserted;
- `every_key_in_the_stream_is_a_key_the_specification_names`: `ALLOWED` from `[&str; 61]` to
  `[&str; 62]` for `suffered`, a new `COMBAT: [&str; 13]` for the keys only a combat resolution
  reaches, and both lists asserted in both directions — every observed key on one of the two lists,
  every listed key observed, **and every `COMBAT` key asserted absent from the reference capture**,
  so a combat field leaking onto a non-combat line fails here rather than being absorbed by a union;
- `every_text_event_line_is_reconstructible_from_its_event_record`: the body loops over both
  captures, and `render_result` gains the two shapes only the social source produces — the
  `["action", "target"]` composite, which the record nests inside the proposal and the text line
  states beside it, and the `suffered` array, rendered as `attacker:damage` pairs joined by `;` and
  skipped where the array is empty.

**Neither edit adds a test or removes one.** `grep -c '^#\[test\]'` is **17** on both the automatic
result and the committed merge for `tests/records.rs`, and **7** on both for `tests/process.rs`. The
census reaches the same conclusion independently from the other end — `census-reconciliation.txt`
attributes all 34 of the engine's arrivals to Phase 4a names — so two methods agree that the merge
itself contributed no test.

## What was re-derived here, and the figures

| Oracle or gate | Re-derived | Result |
|---|---|---|
| `cargo fmt`, `cargo clippy -D warnings` | Both, at the merged tree, with the sources touched first so neither reports a cached clean | Clean — `gates.txt` |
| Test suite | One `cargo test --locked --workspace --no-fail-fast` invocation | **298 passed, 0 failed, 0 ignored**, 22 targets |
| Test census | Three censuses — `master` 264, branch 246, merge 298 — reconciled by target-qualified name | 0 removals in either direction; union 300, intersection 210 — `census-reconciliation.txt` |
| `SPEC-MOK-004` rule 6 | The observer's interface at eight revisions, six full enumerations retained | **94 items / 118 `pub` lines / 24 public fields**, unchanged — `interface.txt` |
| `SPEC-MOK-004` rule 10 | `src/render.rs`'s item counts, from the file truncated above `cfg(test)` | **49 private, 2 public**, 31 functions and 18 constants — `render-items.txt` |
| `SPEC-MOK-002` rules 4 and 5 | The engine's interface and rule 5's four greps, re-measured because `master` moved every line number | 49 items / 43 public fields; exactly two `pub fn … &mut self` — `interface.txt` part 2 |
| `VER-MOK-012` oracle 1, text streams | Comparisons A, B, D, E, F, G over 90 + 30 cells | PASS on every cell — `oracle1/comparison-*.txt` |
| `VER-MOK-012` oracle 1, record streams | **Comparison H**, the gap E and F disclosed: 90 record streams, candidate vs. merge | 45 byte-identical, 45 differing, every differing byte `= 14 × action_trace`, total **3,981,726 = 14 × 284,409**, line counts equal on all 90 — PASS |
| The record stream's event domain | Distinct event kinds in the emitted bytes, both directions | **12** across the 90 non-social cells with the three combat kinds in 0 of them; **15** across the 30 social cells with all three in 30 of 30 — `oracle1/record-kinds.txt` |
| `SPEC-MOK-006` rules 3.2 and 3.3 | Every string value in both captures, by field path: occurrences, distinct values and digit-generalized shapes, and the union of characters the streams use | 23 string-valued paths under the three older policies and 28 under the fourth; **no character outside rule 3.3's union in 1,365,884 records** — PASS, `oracle1/value-domains.txt` |
| Oracles 2, 3 and 6 | Re-taken against the drafted and extended matrices | `oracle2/`, `oracle3/`, `oracle6/` |
| Governance graph | `validate`, `inspect`, `doctor`, `dashboard` on both parents, the merge and the measured tree | Artifacts and relations reconcile exactly in both directions; validation PASS on all five trees; every managed file `unchanged` — `governance.txt` |
| `preflight` | `--work-order WO-MOK-018 --phase review`, from the pinned 0.4.0 venv | PASS |

## Ten things reported rather than fixed

### 1. `SPEC-MOK-004` rule 11's recorded workspace total, which `WO-MOK-016` measured and left owed

Rule 11's last correction on `master` is the 2026-08-20 `WO-MOK-013` row: the observer **141**, the
engine **85**, the workspace **226**. `master`'s own tree runs **264**. The identifier `WO-MOK-016`
appears **0** times in `master`'s `SPEC-MOK-004`.

**This is not a discovery of this packet, and it is important not to present it as one.**
`WO-MOK-016` measured it, in `post/test-census-reconciliation.md` §9.4, and this packet's census
reproduces its figures exactly and independently:

| | rule 11 as recorded (`WO-MOK-013`) | `master` at `fa065cc` | The merge |
|---|---:|---:|---:|
| engine, internal tier | 54 | 82 | 96 |
| engine, public tier | 31 | 40 | 60 |
| **engine total** | **85** | **122** | **156** |
| observer, internal tier | 41 | 41 | 41 |
| observer, public tier | 100 | 101 | 101 |
| **observer total** | **141** | **142** | **142** |
| **workspace total** | **226** | **264** | **298** |

The classification is rule 11's own and not a second one invented to fit, and that is established
rather than asserted: `WO-MOK-016` §9.4 applies it at `d8e2079` — `master` **before** its own
additions landed — and reproduces all seven of the left column's recorded figures there exactly, 54,
31, 85, 41, 100, 141 and 226 both as recorded and as measured. Its own right-hand column is then
82, 40, 122, 41, 101, 142 and 264, which is this table's middle column measured on a different
checkout by a different reader. The merge column is this packet's alone. Engine internal is the
`unittests` names under `simulation::`, engine public the `tests/` targets of `mokiterions-core`
— seven at `master` and eight at the merge, `tests/records.rs` being Phase 4a's — and the observer's
is the remainder; the sums are 96 + 60 = 156,
41 + 101 = 142 and 156 + 142 = 298.

`WO-MOK-016` then recorded the correction as **owed and deliberately not made**, in
`amendment-approvals.md` §5 ("*an amendment this work order owes and has not made*") and as finding
1 of `completion-summary.md` §17.4, on the reasoning that writing it moves the tree its own captures
were taken against, and it closed by referring the timing to the owner: "*Whether that
re-derivation happens under this work order or the next is an owner's call*."

**This branch is the next work order, and act 3 makes that re-derivation.** So ratifying the
2026-08-21 row of `SPEC-MOK-004`'s amendment record is the act that answers `WO-MOK-016`'s referral,
and the owner should be shown it as that rather than as a bare figure correction. Two consequences
follow and both are stated in the row:

- the two corrections cannot be stated separately. Rule 11 at the merge is 298, not 264: `master`'s
  +38 and Phase 4a's +34 are in one tree, and a row correcting only one would leave the rule stating
  a number no tree runs — which is verbatim the situation the 2026-08-19 `WO-MOK-010`/`WO-MOK-007`
  row faced and the precedent it set;
- no record bound to a commit is re-opened. `VREC-MOK-012` measured 246 at `50364a3` and
  `VREC-MOK-016` measured 264 at `4539601`; each was correct where it was taken, and the merge's
  figures are carried by a new record rather than by editing either.

**Two corrections to the second of those citations, both dated after this packet's capture.** They are
placed here rather than in a footnote because finding 1 rests on `WO-MOK-016`'s measurement and a
reader will go looking for the record that carries it.

`VREC-MOK-016` is **not** a verified record and never was. Its status in this tree is `ready`; its
populated `verified_at` of `2026-08-21T11:55:45Z` is the capture timestamp of a candidate, not a
verification. The queue table in `governance.txt` measured it correctly as `VREC-MOK-016 [ready]` at
all three trees, so this packet contradicted itself: three files and the drafted `SPEC-MOK-004` row
described it as "verified at `4539601`", and that phrase was wrong when it was written. All four are
corrected in the same commit as this paragraph. **The 264 figure is unaffected** — it is a
`cargo test` count that `WO-MOK-016`'s own census reproduces independently, and finding 1's argument
never needed the record's status.

`VREC-MOK-016` has since been **superseded**. `master` at `aeca808` (PR #36, merged after this
packet's capture) transitioned it from `ready` to `superseded` at `2026-08-21T12:57:39Z` on the
assurance owner's authority, with `superseded_by = ["VREC-MOK-017"]`, and wrote that successor
verifying `WO-MOK-016` at `ecba9fe` — the commit carrying the eight manual assessments whose absence
is the stated reason the candidate could not be verified where it stood. So the current record for
`WO-MOK-016` is `VREC-MOK-017` at `ecba9fe`, not `VREC-MOK-016` at `4539601`. This tree still holds
the pre-supersession copy, because the merge predates it; that is a fact about when the capture was
taken and not a disagreement with `master`.

### 2. `SPEC-MOK-004` rule 10's private-item count for `src/render.rs`, which no packet has reported

Rule 10 records `mokiterions-tui/src/render.rs` at **48** private items, 30 functions and 18
constants, as corrected in the `WO-MOK-013` row. The tree declares **49** — 31 functions and 18
constants. The one arrival is the private `fn action_text(action: &Action) -> String`, added by
`98a85ea` ("Implement contact, conflict and society under `WO-MOK-012`"), the implementation commit
of the chain later renumbered `WO-MOK-016`.

Unlike finding 1, **this one was not reported anywhere before this packet.** `WO-MOK-016`'s
`completion-summary.md` change-surface table records the function — "*one new function, `action_text`,
and three call sites*" — and nothing in that packet connects it to rule 10's figure; its own finding
1 names rule 11 and the `WO-MOK-013` row only. Act 3 corrects it to 49 in the same row as finding 1,
and it is disclosed here as a figure the owner is being shown for the first time rather than one
already on the record.

`render-items.txt` measures it three ways and reconciles the two counting conventions: 51 total
declarations above `cfg(test)` minus 2 public is 49, and 31 functions plus 18 constants is 49.

### 3. `WO-MOK-016` §9.5's reported defect in rule 11's `WO-MOK-013` paragraph, examined and carried forward

`WO-MOK-016` reports a second defect: rule 11's `WO-MOK-013` paragraph opens "*The fourteen
arrivals, each measured from the target that runs it and none departing*", while a name checked at
the branch point — `tests/layout.rs :: the_log_is_ten_rows_only_where_both_thresholds_are_met` — is
absent at `master`'s tip. Its §9.5 concludes "*the "none departing" clause is not [right]*".

This packet does not correct it, and does not simply repeat it either. Read at the paragraph rather
than at its opening clause, the disclosure is present three sentences later: "*`tests/layout.rs`
rises by one and not by two although two of its tests change:
`the_log_is_ten_rows_only_where_both_thresholds_are_met` is renamed rather than removed, as rule 9
records, and a rename is not an arrival. **No test is lost.***" §9.5 agrees on the substance — "*It
is a rename and not a loss of coverage*" — so what is in dispute is whether a renamed name counts as
a departure in the table's headline clause, and **rule 12's text does not settle it**: rule 12 is
scoped in bold "*by the `WO-MOK-006` restructuring*", by the 2026-08-19 amendment, so it supplies no
general convention for later renames.

That is the whole of what a measurement can say. Whether the clause is reworded is a correction to a
**ratified** row, which is the technical owner's act and not an implementation agent's, and
`WO-MOK-016` has already put it to them. It is carried here so that it is not lost between two work
orders, with the paragraph's own disclosure recorded beside the criticism so the owner is not shown
one without the other.

### 4. A product defect: a Mokiterion killed by a strike has no death tick

`resolve_strike` sets `target.alive = false` at `mokiterions-core/src/simulation.rs:2819` and emits
`AgentDied` at `:2839`–`:2846`, and it never writes `died_at`. The only assignment of that field in
the engine is `apply_survival`'s, at `:3039`. So a Mokiterion killed by an attack is dead in the
event stream, counted by the run record's `deaths`, and carries `"died_at":null` in the same run
record's roster, which `:3321`–`:3328` writes.

**The run record contradicts itself, and the contradiction is exactly attributable.** Over all 30
social record streams:

    streams whose roster is short of the deaths counter    28 of 30
    total deficit, deaths - non-null died_at                    42
    total strike deaths, target_died on attack_resolved         42
    streams where the deficit differs from the strike deaths     0

The per-stream deficit equals that stream's strike-death count in **all thirty** streams, and the
two streams with no deficit are the two in which no strike killed anyone. `seed0-social-d0.75` is
the smallest case a reader can check by hand: `"survivors":9, "deaths":3` over a twelve-Mokiterion
roster carrying two `died_at` values.

**The fix is one line**, `self.agents[target_index].died_at = Some(self.tick);` inside the `if died`
block at `:2839`, beside the event, mirroring `:3039`. The comment above that block explains how the
omission survived review — "*There is no second death and no combat-specific death event*" is true
of the event and was read as true of the field.

**The assertion that catches it already exists and is aimed at the wrong policy.**
`every_cumulative_counter_equals_its_event_count_in_the_text_stream`
(`simulation.rs:7434`) asserts precisely this invariant at `:7489`–`:7490` —
`assert_eq!(agent.died_at, died_at)` against the tick of the agent's `agent_died` line, and
`assert_eq!(agent.alive, died_at.is_none())` — and sweeps `reference_config` only, at `:7436`, under
which no attack occurs. Adding the social policy to that loop is what turns the assertion into
coverage, and it fails on the present code.

Neither the fix nor the test change is made here: both are engine changes outside the four acts the
owner authorized, and `VER-MOK-012` is a record-stream oracle, not a licence to edit the engine.

### 5. An instrument defect in this work order's own analysis directory

`WO-MOK-018/analysis/census-by-target.py` printed a sentence asserting that no test was removed
**unconditionally**, on every input, including one where a removal had occurred. The claim it makes
is true of these three censuses and the instrument was not entitled to make it. It is corrected to
state the measured condition, and `analysis/census-by-target.txt` is regenerated; both are in this
commit. The corrected instrument reproduces every figure the uncorrected one printed for these
inputs, which is why nothing that cites it moves.

It is reported and not buried because an instrument that prints a conclusion its input does not
support is the failure mode this whole packet exists to avoid.

### 6. `SPEC-MOK-006` specifies twelve event kinds and the merge emits fifteen

Phase 4a's `SPEC-MOK-006` was written against `EventType::ALL` at `[Self; 12]` and gives a record
shape for each kind. `master`'s Phase 3 takes it to `[Self; 15]`, and
`oracle1/record-kinds.txt` measures fifteen in the bytes: `attack_resolved`, `threat_resolved` and
`surrender_resolved` in 30 of 30 social cells and in 0 of the 90 others.

**The code covers all fifteen and the specification does not.** The engine's own
`every_event_kind_has_its_exact_record_shape` passes at the merge — it is one of the 298 — because it
reads `EventType::ALL`; the document is what has not moved. `oracle1/suffered-accounting.txt` and
`oracle1/value-domains.txt` enumerate the gap: three record shapes, fourteen field names, eleven
additions to `result.detail` — eight words master's targeted validation can produce and the three
patterns `damage:<u8>`, `increase:<u8>` and `transferred:<u8>` — seven added `proposal.action` verbs,
a closed two-value `target_died` domain, and `suffered` on `action_trace` as a key always present
with a possibly-empty list.

The owner's decision of 2026-08-21 is "Amend `SPEC-MOK-006`", and **the amendment is written in this
commit**, as a 2026-08-21 row of a new `## Amendment record` section whose approval cell reads
OUTSTANDING — `SPEC-MOK-006` was the one specification of the six carrying no such section, and it
now has one. Every field name in the row was measured rather than read off master's source:
`oracle3/drafted-social.txt` runs the drafted field set against both captures and reports PASS with
no field of either stream unnamed and no drafted name unexercised on the social capture, and
`oracle1/value-domains.txt` measures every string value's domain and confirms rule 3.3's character
union holds across all 1,365,884 records. What was **not** measured is disclosed in the same row:
neither capture rejects a targeted proposal, so the eight `detail` words come from the engine's
source and its unit tests and from no stream. Ratification is item 2 of *What is still owed*, and
findings 8 and 9 are the two non-conformances the drafting exposed.

One shape question in it is a decision and not a measurement, and it is put to the owner as one:
whether a targeted verb's `target` is a field **of** the proposal object (Option A, which is what the
merge emits and what `tests/records.rs`'s reconstruction now maps) or a sibling of `proposal` in the
record (Option B, which would match the text line's own layout).

### 7. The reassessment obligation act 3 opens against `ARCH-MOK-002`

`inspect` reports fifteen warnings at `e8114ad` where the merge commit has fourteen. The additional
one is

    ARCH-MOK-002 predates newer declared conforms_to target SPEC-MOK-004 and may require
    reassessment.

and its cause is act 3: `ARCH-MOK-002` is dated 2026-08-20, `SPEC-MOK-004` was dated 2026-08-20 at
the merge, and act 3 moves it to 2026-08-21, so same-day became earlier-than. **It is not reported
here as a date artefact.** `ARCH-MOK-002` declares `conforms_to SPEC-MOK-004`, and what act 3 changed
there is substantive: rule 9's per-target table, rule 10's `render.rs` item figures and rule 11's
three totals. An architecture declaring conformance to a specification whose measured figures just
moved is what the rule exists to catch, and the obligation is owed whether or not the warning fired.
`governance.txt` records it; this packet does not touch `ARCH-MOK-002`.

### 8. The engine writes `"schema":1` on a stream carrying fourteen fields version 1 does not have

`SPEC-MOK-006` rule 10.2 requires the `schema` integer to be incremented "when a record kind is added
or removed, a field is added, removed or renamed, a field's type changes, or a value's enumerated
domain in rule 3.2 gains a member". The 2026-08-21 amendment does the second fourteen times and the
last thirteen. The merged engine writes `1`: `RECORD_SCHEMA_VERSION` at
`mokiterions-core/src/simulation.rs` is unchanged from Phase 4a, so every header record in both
captures reads `"schema":1` on a stream that carries `suffered`, `target_died`, the three resolution
kinds and the rest. The constant is `const RECORD_SCHEMA_VERSION: u32 = 1;` at `:1797`, under the doc
comment at `:1790` that names rule 10 as its authority.

**This is the first test of rule 10.2, and `VER-MOK-012` predicted it in those words.** That
verification's own residual records that the schema version "is verified to be present, not to be
right" and that whether rule 10.2's triggers are complete "cannot be verified before a second version
exists — the first increment will be the first real test". This is that increment.

**The product change is two lines**, and it is not made here. `RECORD_SCHEMA_VERSION` becomes `2`, and
the one asserted literal that hard-codes the value — `simulation.rs:7382`, in the header unit test —
becomes `"schema":2` with it. Every other reference in the engine reads the constant. The reason for
not making it is the ordering: the specification row reads OUTSTANDING, and an engine that emits `2`
against a specification that has not been ratified as saying `2` is a worse state than the one this
finding reports. `SPEC-MOK-005`'s 2026-08-20 row is the precedent for making the obliged product
change **in the same commit as the ratification**, so that no commit in the history carries one
without the other, and that is what is proposed. *Owner: the technical owner, with the ratification.*

### 9. Rule 7.8's stated absence is still right and its stated reason is now false

`SPEC-MOK-006` rule 7.8 gave the metrics record no conflict, combat or social field, and gave as its
reason that the engine does not compute those phenomena. At the merge the engine does compute all
four of them — attacks, threats, retreats and surrenders resolve, and `oracle1/record-kinds.txt`
measures the three resolution kinds in 30 of 30 social cells — so the reason is false while the
absence it justifies is still correct.

The correct reason is rule 10.4's, which the specification already states elsewhere: no approved
requirement asks the metrics or run record to carry an aggregation of those events, and a field whose
arrival is merely expected is not reserved. The 2026-08-21 amendment corrects the reason, leaves the
absence, and corrects the *reserved field* counterexample and the *Conflict, combat and social
metrics* bullet of *Explicitly unspecified decisions* the same way. **No provision changes and no
field arrives**; what changes is a justification that a reader could have checked against the tree and
found untrue.

It is reported separately from finding 8 because the two are different kinds of defect. Finding 8 is
the product disagreeing with the specification; this is the specification's reasoning disagreeing with
the tree while its rule stays right. A packet that reported only the first would leave a reader
believing the absence itself was in doubt.

### 10. `WO-MOK-018` names two different work orders on two branches, and act 4's identifier moved twice

Act 1 renumbered this chain from `WO-MOK-012` to `WO-MOK-018` on the owner's decision of 2026-08-21,
"018, nothing else moves", because `master` held `013` through `017`. `018` was free when that decision
was taken. It is not free now.

    ref                                        WO-MOK-018 is                              VREC ids present
    feature/phase-4a-definition (this branch)  "Emit a structured record stream ..."       001-016
    feature/observer-fear-and-filter-count     "Close the two observer defects Phase 3.1   001-011, 013-018
                                                left ..." (PR #37, opened 2026-08-21)
    master (aeca808)                           absent                                      001-011, 013-017

Both are `status = "implemented"`, both target `master`, and **both declare evidence under the same
path**, `docs/engineering/simulation/evidence/WO-MOK-018/`, with entirely different contents — this
packet's merge measurements on one side, `filter-vocabulary`, `inspector` and `non-perturbation` files
on the other. That other branch also carries `VREC-MOK-018`, a `ready` candidate for *its* `WO-MOK-018`
bound to `6051ef21`.

**Nothing here is wrong about either branch in isolation; the collision exists only in the union.**
Whichever merges second inherits an identifier clash that `validate` on a single tree cannot see,
because on each tree alone there is exactly one `WO-MOK-018` and the graph is consistent — this
packet's own `governance.txt` reports PASS with 0 errors on all four trees it measures, and that
remains true.

The same shared space moved act 4's record identifier twice in one day: this packet was drafted naming
`VREC-MOK-017`, which `master` assigned to `WO-MOK-016`'s successor a few hours later; `VREC-MOK-018`
was taken by the third branch while these corrections were being written. The eight references are now
`VREC-MOK-019`, the lowest free across all five remote heads at this commit, and the identifier is
marked provisional at the top of this file for that reason.

**Two decisions follow and neither is an implementation agent's.** Which work order keeps `018` and
which renumbers — with the evidence directory, this packet's own path, moving with it. And whether
`VREC-MOK-019` is the number act 4 uses, re-derived across every ref at the moment it is written
rather than taken from this file. *Owner: the engineering owner, and the assurance owner for the
record.*

## What is still owed before anything binds this merge

1. **A new verification record at the merge commit, provisionally `VREC-MOK-019`** — the owner's
   decision of 2026-08-21 was "New record at merge commit", leaving `VREC-MOK-012` (`verified` at
   `50364a3`) and `VREC-MOK-016` (a `ready` candidate at `4539601`, never verified, and superseded on
   `master` after this capture) untouched, on `VREC-MOK-015`'s precedent. It declares the paths in this
   directory, and `governance.txt` records the consequence: the dashboard snapshot digest `52652ca4…`
   moves when it does, because that digest covers declared evidence paths. The identifier must be
   re-derived across every remote ref at the moment of writing and not read off this file — finding 10.
   *Owner: the verifier.*
2. **Ratification of the 2026-08-21 `SPEC-MOK-006` row**, which is written in this commit and reads
   OUTSTANDING. Three things in it are the owner's to decide and are put separately in the row itself
   rather than buried in its field list: the `schema` increment together with finding 8's two-line
   product change, both in the ratifying commit on `SPEC-MOK-005`'s precedent; the record's nesting of
   a targeted verb's `target` inside `proposal` where the text line states it beside (Option A, which
   the merge emits and `tests/records.rs` binds) against Option B's sibling key, which would match the
   text field for field and would be a product change; and the unconditional presence of `suffered`.
   *Owner: the technical owner, on the owner's "Amend `SPEC-MOK-006`" of 2026-08-21.*
3. **The extension of `VER-MOK-012`'s per-kind oracle from twelve event kinds to fifteen**, which is a
   change to a verification contract and is named in the amendment row as one this row does not make.
   The measurement exists — `oracle3/drafted-social.txt` runs the fifteen-kind field set against both
   captures — but the contract still specifies twelve. *Owner: the technical owner.*
4. **Ratification of the 2026-08-21 `SPEC-MOK-004` row**, which reads OUTSTANDING. It is where
   findings 1 and 2 are corrected, and where `WO-MOK-016`'s referral of the timing is answered.
   *Owner: the technical owner, under the "I draft, you ratify each" procedure of 2026-08-21.*
5. **The `died_at` fix and the test strengthening of finding 4.** *Owner: the technical owner, as a
   scope decision — it is an engine change and no act authorized one.*
6. **The `WO-MOK-013` clause of finding 3**, and the `ARCH-MOK-002` reassessment of finding 7.
   *Owners: the technical owner and `ARCH-MOK-002`'s owner respectively.*
7. **PR #31's body trailer**, which must read `Harness-Work-Order: WO-MOK-018` after act 1's
   renumbering, and its title, which still names `WO-MOK-012` and `VREC-MOK-012`. CI reads the trailer
   from the stored event payload, so a body edit takes effect only on the next push. **The stale trailer
   does not fail the check — it passes it against the wrong work order**, which is worse: the merge
   brought `master`'s own unrelated `WO-MOK-012` into the tree, so at the push of the `SPEC-MOK-006`
   amendment the `Review preflight` step reported "Work order: WO-MOK-012 (implemented)" and PASS, and
   all five checks were green without any of them having examined `WO-MOK-018`. A green rollup on this
   PR is therefore not evidence that this work order passed review preflight; the `Work order:` line in
   the job log is. Not done: an edit to the PR is an outward-facing act the owner has not authorized.

> Two facts a later reader will want, recorded here because they are easy to mis-derive.
> **The observer's whole source directory at the merge is byte-identical to `master`'s** —
> `git diff fa065cc 1e09f85 -- mokiterions-tui/src/` is empty. Resolving an entire source directory
> in one parent's favour is the shape in which the other parent's work is silently lost, and five
> observer files differ between the branch and the merge by 503 insertions and 100 deletions, which
> read alone looks like exactly that. It is not: `git diff ff3a155 fa0bfd9 -- mokiterions-tui/src/`
> is **also** empty, so the branch's observer *is* the base's observer, the 503 lines are `master`'s
> arrivals that the branch never had, and taking `master`'s copy wholesale loses nothing.
> `interface.txt` part 1 measures both directions, and `census-reconciliation.txt` reaches it from
> the test side with the observer's ten targets unchanged at 142.
> **`WO-MOK-011/merge/interface.txt`'s byte-identity claim no longer holds at the merge.** Its
> retained 125-line enumeration still equals this branch's exactly, and differs from `master`'s and
> from the merge's on one line — line 39, `options::USAGE` gaining `|social`. `interface.txt` part 3
> records the correction rather than repeating the claim.

## Files here

All files in this directory are **LF** and are committed verbatim: `.gitattributes` sets
`docs/engineering/simulation/evidence/** -text`, because recorded SHA-256 digests must reproduce.

| File | Bytes | What it is |
|---|---:|---|
| `README.md` | this file | The packet index, the resolution account and the nine findings |
| `COMMIT.txt` | 41 | `e8114ad…`, the commit every figure here is measured at |
| `gates.txt` | 24,412 | `fmt`, `clippy`, the 298-test run, `inspect`, `validate`, `doctor`, `preflight` |
| `governance.txt` | 14,787 | The governance graph on both parents, the merge and the measured tree |
| `interface.txt` | 56,440 | Rule 6 and `SPEC-MOK-002` rules 4–5, eight revisions, six full enumerations |
| `render-items.txt` | 6,078 | `src/render.rs`'s items on all three trees, and finding 2's measurement |
| `census-reconciliation.txt` | 11,849 | The three censuses reconciled by qualified name, 0 removals |
| `census-by-target-master-vs-merge.txt` | 13,024 | Per-target arrivals against `master` |
| `census-by-target-branch-vs-merge.txt` | 15,485 | Per-target arrivals against the branch |
| `test-run.txt`, `test-census.txt` | 26,719 / 25,610 | The merge's suite: 298 names, exit `0` |
| `test-run-master.txt`, `test-census-master.txt` | 23,854 / 22,665 | `master`'s: 264 names, exit `0` |
| `test-run-branch.txt`, `test-census-branch.txt` | 22,253 / 20,713 | The branch's: 246 names, exit `0` |
| `oracle1/manifest-*.txt` | four files | Per-cell digests for the merge's and the social captures |
| `oracle1/comparison-a…g` | six files, lettered a, b and d through g | The text-stream comparisons, PASS on every cell |
| `oracle1/comparison-h-candidate-vs-merge-records.txt` | 5,865 | The record-stream comparison E and F disclosed as not made |
| `oracle1/record-kinds.txt` | 1,871 | Twelve kinds, then fifteen, measured in the emitted bytes |
| `oracle1/suffered-field.txt` | 1,032 | The field on 402,610 records, with list lengths and entry key sets |
| `oracle1/suffered-accounting.txt` | 17,661 | Comparison H read in three parts, every string value by field path, and finding 6 stated |
| `oracle1/value-domains.txt` | 6,211 | Every string value's measured domain, and rule 3.3's character union holding across 1,365,884 records |
| `oracle2/`, `oracle3/`, `oracle6/` | four files each | The retained, drafted and extended matrices for each oracle |
| `record-field-accounting.py` | 14,579 | The retained instrument behind comparison H and the value domains: `kinds`, `field`, `compare`, `alphabet` |
| `capture-social.sh` | 2,179 | The 30-cell social capture; the 90-cell one is `WO-MOK-018/capture.sh` |
| `social-vs-master.py` | 4,324 | The social streams against `master`'s, for oracle 1's comparison G |
| `reconstruct-combat.py` | 11,697 | Every combat text line rebuilt from its record |
| `replay-combat.py` | 10,600 | The resolutions replayed against the rules independently of the engine |
| `validate-drafted.py` | 10,366 | The drafted `SPEC-MOK-006` shapes checked against the emitted stream |

The captures the manifests describe are not retained: 303 MB of record stream across the 120 cells,
213 MB and 90 MB. The manifests are what the comparisons read, and
`oracle1/suffered-accounting.txt` gives the two commands that re-take them.

## Authority

Every command in this directory is read-only and derived. Validation does not approve; inspection
does not authorize; a passing gate is not a verdict. `SPEC-MOK-004`'s figures are corrected by the
2026-08-21 amendment row and not by any file here, and that row reads OUTSTANDING until the technical
owner ratifies it; the same is true of `SPEC-MOK-006`'s 2026-08-21 row, which this commit writes and
does not approve. The ten findings are reported to the owner; none is acted on, and in particular the
engine is not changed to satisfy an unratified specification, and neither work order claiming
`WO-MOK-018` is renumbered by this file. The decision about this tree belongs to a new verification
record at the merge commit, provisionally `VREC-MOK-019`, and it is not written yet.

**Two dates apply to this directory and a reader needs both.** Every measurement is at `e8114ad`,
2026-08-21. The corrections in the commit that added finding 10 are later than that and are stated as
later: `master` moved to `aeca808`, `VREC-MOK-016` was superseded, and two identifiers this packet had
named were taken by other branches. No measured figure was re-taken for them, because none of them
touches a source file — `aeca808` is documentation only, and `git merge-tree HEAD aeca808` reports no
conflict. What changed is what this packet says about records and identifiers, not what it measured
about the tree.
