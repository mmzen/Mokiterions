# `VER-MOK-016` row 248: the two test censuses reconciled name by name

| Field | Value |
|---|---|
| Baseline | `39662d13abd08e3410648d1c59ad38384f8ad2d2` — `baseline/test-census.txt`, 212 names |
| Candidate | `7c4aef3967406c05d80da963695898b77f5329e9` — `post/test-census.txt`, 249 names |
| Invocation | `cargo test --locked --workspace --no-fail-fast`, from the workspace root |
| Exit code | `101` — three cases fail; see §4 |
| Date | 2026-08-20 |
| Re-taken | **§6** — the candidate has moved three times since; 250 names, exit `0`, and a second rename |
| Re-taken again | **§7** — the branch-numbering correction; still 250 names and exit `0`, with a third rename |
| Retained | **§8** — §§6 and 7 measured from logs that were not kept. `post/test-run-amended.txt` and `post/test-census-amended.txt` are those two files, and §8 reconciles them against both retained censuses |

Row 248 states the obligation this file discharges: no case present before the change may be
**removed, renamed away or `#[ignore]`d**. It groups the three because they are the same loss from a
reader's side — a name that was checked and is now absent — and it means a rename cannot be waved
through as cosmetic. One rename happened. §3 reconciles it, and reconciles it by measurement rather
than by assertion.

---

## 1. The reconciliation

    $ names() { grep -v '^#' "$1" | awk -F' :: ' 'NF>2 {print $1" :: "$2}' | sort; }
    $ comm -23 <(names baseline/test-census.txt) <(names post/test-census.txt)   # lost
    $ comm -13 <(names baseline/test-census.txt) <(names post/test-census.txt)   # added
    $ comm -12 <(names baseline/test-census.txt) <(names post/test-census.txt)   # retained

| | Count |
|---|---:|
| names at the baseline | 212 |
| retained, target-qualified name unchanged | **211** |
| present at the baseline, absent at the candidate | **1** — the rename, §3 |
| added at the candidate | **38** |
| names at the candidate | **249** |
| `#[ignore]`d, either side | **0** |
| removed | **0** |

211 + 1 = 212 and 211 + 38 = 249, so every name on both sides is accounted for in exactly one row.

Both censuses are produced by one reader, `analysis/test-census.py`, and the reader is checked against
the side it did not write: run over `baseline/test-run.txt` it reproduces the hand-written
`baseline/test-census.txt` line for line, with no difference at all. That check matters because a
reconciliation between two lists written by two different readers measures the readers as much as the
suite.

## 2. Where the 38 new names sit

| Target | Baseline | Candidate | |
|---|---:|---:|---|
| `unittests` | 93 | 121 | +28 |
| `tests/cli.rs` | 13 | 15 | +2 |
| `tests/decisions.rs` | 1 | 3 | +2 |
| `tests/viability.rs` | 2 | 4 | +2 |
| `tests/process.rs` | 6 | 7 | +1 |
| `tests/termination.rs` | 4 | 5 | +1 |
| `tests/verification.rs` | 19 | 20 | +1 |
| `tests/authority.rs`, `density.rs`, `export.rs`, `layout.rs`, `naming.rs`, `options.rs`, `render.rs`, `spatial.rs`, `state.rs` | unchanged | unchanged | |

The distribution is the one `VER-MOK-016` predicts: 28 of the 38 are internal-tier cases, because the
resolution arithmetic, the entropy constraint, the branch order and the record's lifetime are all
oracles that need constructed state, and `SPEC-MOK-002` rule 7 fixes the tier by the access a test
requires. The ten public-tier additions are the ones that need only the public interface — argument
parsing for the new policy value, the process boundary, termination under it, the verb census, the
survivor curve, the identifier band, and the observer's presentation of a rejection.

Nine of the sixteen targets gained nothing, which is the same claim `post/byte-identity.txt` makes from
the other direction: this change reaches the decision layer and the resolution layer and nothing else.

## 3. The one rename, reconciled

| | |
|---|---|
| Absent at the candidate | `tests/verification.rs :: no_shipped_decision_source_has_a_proposal_rejected` |
| Present at the candidate | `tests/verification.rs :: no_source_confined_to_the_valid_action_list_has_a_proposal_rejected` |
| File | `mokiterions-tui/tests/verification.rs` |
| Body at the baseline | lines 671–684 |
| Body at the candidate | lines 683–696 |

**The body is byte-identical.** Measured, not asserted:

    $ was=$(git show 39662d1:mokiterions-tui/tests/verification.rs | sed -n '671,684p')
    $ now=$(sed -n '683,696p' mokiterions-tui/tests/verification.rs)
    $ diff <(printf '%s\n' "$was") <(printf '%s\n' "$now")
    (no output)
    $ printf '%s\n' "$was" | sha256sum
    7ef0a2580067665eab2a7d981d01abe3b50f4a5e3720a7e4e67b6b95c646e21b
    $ printf '%s\n' "$now" | sha256sum
    7ef0a2580067665eab2a7d981d01abe3b50f4a5e3720a7e4e67b6b95c646e21b

Fourteen lines, one digest. The swept policy list is the same three — `baseline`, `reference`,
`individual` — the loop is the same, the seed and tick count are the same, and the assertion is the
same `assert_eq!(decision.outcome, DecisionOutcome::Accepted, …)` with the same message. Nothing was
relaxed, widened, weakened or deleted; the identifier above the body changed and the doc comment above
that grew a paragraph.

**Why it moved.** The old name asserted a property of *what ships*: no shipped source is rejected. This
change ships a fourth source that **is** rejected, by design — `SocialDecisionSource` proposes targeted
actions, `SPEC-MOK-001` rule 3 keeps targeted actions off the observation's valid-action list on
purpose, so `allows` cannot screen them and rule 6 is the only gate. Rule 26's own text fixes that its
branch 1 proposes an answer "whether or not that answer can succeed". Leaving the old name in place
would have made it false the moment the fourth source shipped, and the honest repairs are only two:
narrow the name to the property the three sources actually share, or widen the sweep to include a
source that contradicts it. The second is not available, so the name moved.

**What replaces the coverage the old name implied.** A new case,
`tests/verification.rs :: the_social_source_is_rejected_only_as_the_specification_admits`, asserts
which grounds are reachable under `social` rather than that none is: the ground presented is one of
rule 6's nine and never a phrase of the observer's own, and never a fault or a warning. So the fourth
source is covered more strictly than the old name covered it — the old name would have been satisfied by
zero rejections, and the new pair asserts both that the three are never rejected and that the fourth's
rejections carry only specified grounds.

**Row 248 as applied here.** The prohibition is against a case being *renamed away* — lost behind a new
identifier. This case was not lost: it is in the census, in the same target, with the same body digest,
and the reconciliation above is what row 248 asks a reader to be given instead of a bare count. It is
recorded as a rename requiring the verifier's acknowledgement, not as an equivalence the
implementation is entitled to declare on its own.

## 4. The three failures

| Target | Case |
|---|---|
| `tests/decisions.rs` | `every_targeted_verb_applies_somewhere_in_the_declared_matrix` |
| `tests/viability.rs` | `the_social_source_keeps_the_world_habitable_and_combat_lethal` |
| `tests/viability.rs` | `no_identifier_series_is_monotone_in_identifier_or_correlated_beyond_the_band` |

246 pass, 3 fail, 0 ignored. All three are new cases, all three are the requirement-bearing oracles of
`REQ-MOK-052` and `REQ-MOK-058`, and all three fail from one cause, which `escalation.md` states and
measures. **No case that passed at the baseline fails at the candidate**: the 211 retained names all
report `ok`, and so does the renamed one.

`--no-fail-fast` is on the invocation and the baseline's was not, because without it cargo stops after
the first failing target and eight later targets never run — a census taken from that log would be
missing names and would read as removals. The plain invocation was run too and exits `101` identically;
the flag changes which targets execute, not any verdict.

## 5. What this file does not establish

It reconciles names and one body. It does not assert that the 38 new cases are the right cases, that
they cover what `VER-MOK-016` requires them to cover, or that any of them is well written — the
requirement-to-test mapping and the verifier's own reading are what settle that, and both are owed.
Nothing here is a verification verdict.

## 6. The candidate moved: the census re-taken, and a second rename

Everything above was measured at `7c4aef3`. The candidate has moved three times since — `77f3b25`,
`7d744bb` and `64d00e5` — and this section is re-taken at the commit that carries it, whose parent is
`64d00e5a423b1a4071f487260c629602fd2193de`. Row 248's obligation runs against the **baseline**, so it is
re-checked against the baseline and not only against the previous candidate.

| Field | Value |
|---|---|
| Invocation | `cargo test --locked --workspace --no-fail-fast`, from the workspace root |
| Exit code | **`0`** |
| Names | **250** — 250 passed, 0 failed, 0 ignored |
| Date | 2026-08-20 |

    $ python docs/engineering/simulation/evidence/WO-MOK-016/analysis/test-census.py <log> <out>
    250 names; 250 passed, 0 failed, 0 ignored
    $ comm -23 <(names baseline/test-census.txt) <(names <out>)          # baseline names absent
    tests/verification.rs :: no_shipped_decision_source_has_a_proposal_rejected
    $ comm -23 <(names post/test-census.txt) <(names <out>)              # absent since 7c4aef3
    tests/viability.rs :: no_identifier_series_is_monotone_in_identifier_or_correlated_beyond_the_band
    $ comm -13 <(names post/test-census.txt) <(names <out>)              # added since 7c4aef3
    tests/viability.rs :: no_identifier_series_is_monotone_in_identifier
    tests/viability.rs :: survival_by_turn_position_stays_inside_the_stated_bound

| | Count |
|---|---:|
| names at the baseline | 212 |
| retained, target-qualified name unchanged | **211** |
| present at the baseline, absent at the candidate | **1** — the same rename, §3, and no other |
| added at the candidate | **39** |
| names at the candidate | **250** |
| `#[ignore]`d, either side | **0** |
| removed | **0** |
| non-`ok` outcomes | **0** |

211 + 1 = 212 and 211 + 39 = 250. **The three failures of §4 are gone and no case that passed has
stopped passing**: the one baseline name absent is the §3 rename and the list is unchanged, so the
sweep above is the whole of row 248's obligation and it holds.

**The second rename, and why row 248 does not reach it.** `VER-MOK-016` oracle 5's outcome half was
restated by the amendment of 2026-08-20 — `escalation.md` §11 measures why, and the contract records the
decision — and the one case that carried it became two, because its two parts now need different seed
sets and cannot share one body:

| | |
|---|---|
| Absent since `7c4aef3` | `tests/viability.rs :: no_identifier_series_is_monotone_in_identifier_or_correlated_beyond_the_band` |
| Present at the candidate | `tests/viability.rs :: no_identifier_series_is_monotone_in_identifier` — the tripwire, on the five declared seeds |
| | `tests/viability.rs :: survival_by_turn_position_stays_inside_the_stated_bound` — the bound, on the declared 200 |

**This name was never at the baseline.** It was one of the 38 added by this work order, so it is not a
case that was checked before the change and is now absent, which is the loss row 248 exists to prevent.
The row is stated against the baseline census on purpose: a work order that could not restate an oracle
it wrote itself, in the same session that measured the oracle to be wrong, would be prevented from
correcting its own mistakes rather than prevented from hiding coverage. It is recorded here anyway,
because a reader diffing the two candidate censuses would otherwise find an unexplained absence.

What the split removed from the code is the `±0.5` rank-correlation assertion, and that removal is the
amendment's, not the implementation's: it is approved in `VER-MOK-016`'s amendment record with the
measurement it was approved against. Both correlations are still computed and still printed. Nothing
else in either body was weakened — the monotone check is transcribed unchanged, and the bound is an
assertion the suite did not have before.

**Two further bodies changed, and neither is a baseline name.** `escalation.md` §10 states both:

| Target | Case | What changed |
|---|---|---|
| `unittests` | `a_threat_composes_with_rule_12_in_turn_order_and_outlasts_its_tick` | Two assertions restated at the amended `ENGAGEMENT_FEAR_THRESHOLD` of `95`, and one added: the target now answers with a strike, which is asserted rather than left unmeasured |
| `tests/decisions.rs` | `the_acting_order_is_one_ascending_pass_per_tick_under_the_social_source` | Its model of which Mokiterions are entitled to an opportunity in a tick was contradicting `SPEC-MOK-001` rule 13, which combat exposed by killing anyone mid-tick for the first time |

Both were added by this work order, both still assert what they were written to assert, and the second
now asserts strictly more than it did — it narrows the expected width of a tick on a death before its
holder's turn, which the old form could not have detected.

## 7. The third rename: `branches_three_and_four` becomes `branches_four_and_five`

Re-taken again at the commit that carries this section, whose parent is `ef4f8aa`. The occasion is not
an oracle this time but the numbering: the amendment of 2026-08-20 hoisted rule 19's case 3 into rule 26
as a new branch 3, so the two branches that engage contact and close distance became 4 and 5, and four
doc comments and one **test name** still said 3 and 4.

| Field | Value |
|---|---|
| Invocation | `cargo test --locked --workspace --no-fail-fast`, from the workspace root |
| Exit code | **`0`** |
| Names | **250** — 250 passed, 0 failed, 0 ignored |
| Date | 2026-08-20 |

The census differs from §6's in exactly one line, and the diff is the whole of it:

    - unittests :: simulation::tests::branches_three_and_four_choose_by_distance_then_by_the_engagement_threshold :: ok
    + unittests :: simulation::tests::branches_four_and_five_choose_by_distance_then_by_the_engagement_threshold :: ok

| | Count |
|---|---:|
| names at the candidate | **250** — unchanged |
| names present in §6 and absent here | **1** — this rename |
| names added | **1** — this rename |
| present at the baseline, absent at the candidate | **1** — still the §3 rename, and no other |
| `#[ignore]`d | **0** |
| removed | **0** |
| non-`ok` outcomes | **0** |

**Row 248 does not reach this name either, and for the same reason as §6's**: rule 26 does not exist at
the baseline, so every test of its branches was added by this work order. What the row forbids is losing
a case that was checked before the change, and the count above shows none was.

**The body is untouched.** Not one assertion was added, removed, restated or reordered: the diff of that
test is its name, its doc comment, and two words in two inline comments. The doc comment now records that
the branches were numbered 3 and 4 when the test was written, so the history is on the case rather than
only in this file. The alternative — leaving a normatively numbered branch list disagreeing with the test
names that check it — would have cost every later reader the mapping, and `SPEC-MOK-001` rule 26 states
that the ordering is normative rather than illustrative.

## 8. The log §§6 and 7 measured from, retained

Sections 6 and 7 report a 250-name census at exit `0`, and until now **neither figure had a file behind
it.** The two retained files, `post/test-run.txt` and `post/test-census.txt`, are the `7c4aef3` pair:
249 names, three failures, exit `101`. A verifier reading §6's "250 passed, 0 failed" had nothing to
hash and nothing to re-census, which is the one thing this packet is not allowed to ask of them.

So the pair is retained, at the tree that carries this section:

| Field | Value |
|---|---|
| Commit | `139061530f1dba72c9a20427eeaac6ce69492fb2`, tracked-clean, nothing untracked |
| Invocation | `cargo test --locked --workspace --no-fail-fast`, from the workspace root |
| Exit code | **`0`** |
| Log | `post/test-run-amended.txt` — 377 lines, 22,721 bytes |
| Census | `post/test-census-amended.txt` — 250 names, 250 passed, 0 failed, 0 ignored |
| Reader | `analysis/test-census.py`, unchanged, the same reader that wrote every census here |
| Date | 2026-08-20 |

Two commits sit between this run and `59d61b9`, the commit §7 was taken at, and **neither contains a
line the engine compiles** — one is this evidence directory, the other is `VER-MOK-016` and
`WO-MOK-016` prose. So this log is `59d61b9`'s suite, on the same reasoning `post/capture-state.txt` §5
gives for naming a capture's commit.

**Both pairs are kept and neither replaces the other.** §§1 to 5 reconcile the 249-name census and name
its three failures, and `escalation.md` cites those three by name as the measured failure that stopped
the work; overwriting that file would leave five sections and an escalation record without their
artifact. The files are named for what they are, and this is which section reads which:

| File | Read by | State |
|---|---|---|
| `post/test-run.txt`, `post/test-census.txt` | §§1 to 5, `escalation.md` | `7c4aef3` — 249 names, 246 pass, 3 fail, exit `101` |
| `post/test-run-amended.txt`, `post/test-census-amended.txt` | §§6 to 8 | after the `REQ-MOK-057` amendment — 250 names, 250 pass, exit `0` |

### The reconciliation, recomputed from the retained files

Row 248's obligation runs against the **baseline**, so that is the comparison that discharges it, and it
is now computable by a verifier from two committed files:

    $ names() { grep -v '^#' "$1" | grep -v '^$' | sed 's/ :: [^:]*$//' | sort; }
    $ comm -23 <(names baseline/test-census.txt) <(names post/test-census-amended.txt)
    tests/verification.rs :: no_shipped_decision_source_has_a_proposal_rejected

| | Count |
|---|---:|
| names at the baseline | 212 |
| present at the baseline, absent here | **1** — still the §3 rename, and no other |
| retained, target-qualified name unchanged | **211** |
| added since the baseline | **39** |
| names here | **250** |
| `#[ignore]`d | **0** |
| removed | **0** |
| non-`ok` outcomes | **0** |

211 + 1 = 212 and 211 + 39 = 250, so every name on both sides sits in exactly one row.

The step from the `7c4aef3` census to this one is five lines, and it is exactly what §§6 and 7 describe
— two renames and one genuine addition, no removal:

    - tests/viability.rs :: no_identifier_series_is_monotone_in_identifier_or_correlated_beyond_the_band
    + tests/viability.rs :: no_identifier_series_is_monotone_in_identifier
    + tests/viability.rs :: survival_by_turn_position_stays_inside_the_stated_bound
    - unittests :: simulation::tests::branches_three_and_four_choose_by_distance_then_by_the_engagement_threshold
    + unittests :: simulation::tests::branches_four_and_five_choose_by_distance_then_by_the_engagement_threshold

The first three lines are `VER-MOK-016`'s third amendment row landing in the suite: oracle 5's outcome
check was one test asserting a monotonicity tripwire *and* a rank-correlation band, and the amendment
removed the band and put a turn-position survival bound in its place. One test became two because one
obligation became two, which is why this is `+2 −1` rather than a rename. The last two lines are §7's.

**The suite's cost moved with it**, and the figure belongs here rather than in a reader's assumption:
`tests/viability.rs` now runs 37.88 s of the whole invocation's wall time, against 2.08 s for the next
slowest target, because the turn-position bound is evaluated over the declared 200-seed diagnostic set at
1,000 ticks each. `VER-MOK-016` states no time bound on the suite and this section sets none; it records
what one invocation now costs, so that a later reader who finds `cargo test` slow knows which target to
look at and why it is expected.

## 9. The baseline moved: the census re-taken against `master`'s tip

Sections 1 to 8 all compare against `baseline/test-census.txt`, which holds **212** names. On 2026-08-21
`master` was merged into this branch at `259859d`, and that merge moved the before side of every
comparison above: `master`'s tip carries `WO-MOK-013`, which this branch had never seen, so the tree this
work order's additions sit on holds **226** names and not 212.

Nothing above is edited. `SPEC-MOK-004` rule 11 fixes the form this correction takes, because the
situation has arisen once before in this repository and the rule records how it was resolved:

> `WO-MOK-010`'s 21 additions and 0 removals are reconciled name by name in its `test-census.txt`, which
> was re-taken on 2026-08-19 against `master`'s tip and reads **179 before, 200 after**; `master`'s ten
> arrivals sit on its before side rather than among this work order's additions. It was not edited to
> reach that figure — the earlier capture, taken at `4f32a9f` against the branch point, reached 190, and
> a capture is re-run rather than corrected.

So `master`'s arrivals belong on the before side, the earlier captures stay as they are, and this section
is a fourth measurement rather than a revision of the first three.

**On the two commit names for one baseline.** The table at the head of this file names the baseline
`39662d13`, while the merge's own base against `master` is `dac9bac3`. They are different commits and
both hold 212 names, because `git diff 39662d1 dac9bac -- mokiterions-core mokiterions-tui` is empty:
no line either crate compiles differs between them. The baseline census is valid against either, and
both names are recorded here so that a reader who computes the merge base does not read the difference
as a discrepancy.

### 9.1 The two runs

| | Before | After |
|---|---|---|
| Commit | `d8e207941f99ee47ae6c7f3ffeb1769f560fd4dc` — `master`'s tip, the merge's second parent | `af78b9d`'s tree, tracked-clean, nothing untracked |
| Invocation | `cargo test --locked --workspace --no-fail-fast`, from the workspace root | the same |
| Exit code | **`0`** | **`0`** |
| Names | **226** — 226 passed, 0 failed, 0 ignored | **264** — 264 passed, 0 failed, 0 ignored |
| Log | `post/test-run-master.txt` — 412 lines, 22,428 bytes | `post/test-run-merged.txt` — 391 lines, 23,777 bytes |
| Census | `post/test-census-master.txt` — 229 lines, 19,000 bytes | `post/test-census-merged.txt` — 267 lines, 22,669 bytes |
| Reader | `analysis/test-census.py`, unchanged — the same reader that wrote every census in this file | the same |
| Date | 2026-08-21 | 2026-08-21 |

    176d2dc3d7a556799a43f462cca6f98e6a5abfa3fcf9ee25b04a2b8be2525f03  post/test-run-master.txt
    258179955a9b50a6356d2921b3e00663da4f3e4fe867b1feda79027c4719a6f8  post/test-census-master.txt
    9042aa847fa671730ab9c9d550e2019bb4b188ae0f4e905f8732f7a2b9cd4932  post/test-run-merged.txt
    0cacbbe3162555159c5bdb670a032137e976644c1c1b8486d29c29e86b6c4706  post/test-census-merged.txt

**The after run is the merge's suite, not this commit's.** Between `259859d` and the tree it was taken
at, twelve files changed and **every one of them is under `docs/`**: `git diff --name-only 259859d
af78b9d | grep -v '^docs/'` is empty. So no line the workspace compiles differs, and this log is the
merge commit's suite, on the same reasoning §8 gives for naming its own.

**The before run was built in a detached worktree, and its log says so.** `post/test-run-master.txt`
lines 4 and 59 read `Compiling Mokiterions v0.1.0 (C:\Users\mathi\mok-master-census\mokiterions-core)`
and the observer's equivalent, because `master`'s tip was checked out with `git worktree add --detach`
rather than by moving this branch's checkout. Those two lines are **kept**. A log rewritten to look as
though it came from somewhere else is not a log, and the path is the disclosure that the before side was
measured on a tree this branch never had checked out, which is the whole point of measuring it.

### 9.2 The reconciliation

    $ names() { grep -v '^#' "$1" | grep -v '^$' | sed 's/ :: [^:]*$//' | sort; }
    $ comm -23 <(names post/test-census-master.txt) <(names post/test-census-merged.txt)
    tests/verification.rs :: no_shipped_decision_source_has_a_proposal_rejected

| | Count |
|---|---:|
| names at `master`'s tip | 226 |
| retained, target-qualified name unchanged | **225** |
| present at `master`'s tip, absent at the merge | **1** — still the §3 rename, and no other |
| added at the merge | **39** |
| names at the merge | **264** |
| `#[ignore]`d, either side | **0** |
| removed | **0** |
| non-`ok` outcomes, either side | **0** |

225 + 1 = 226 and 225 + 39 = 264, so every name on both sides sits in exactly one row.

### 9.3 What re-basing the comparison did not change

The two reconciliations agree line for line on this work order's own side:

| | §8, against the branch point | §9, against `master`'s tip |
|---|---:|---:|
| names before | 212 | 226 |
| present before, absent after | **1** — `no_shipped_decision_source_has_a_proposal_rejected` | **1** — the same name |
| added | **39** | **39** — the same 39 names |
| names after | 250 | 264 |

That is the outcome rule 11's `WO-MOK-010` paragraph anticipates: `master`'s arrivals land wholly on the
before side and none of them is absorbed into this work order's additions. The 39 are the same names in
both comparisons, so §§1 to 8 stand unaltered as statements about what this work order added, and only
the totals either side of them move.

The complement is also computable, and it is the check that the merge introduced nothing of its own:

    $ comm -13 <(names post/test-census-amended.txt) <(names post/test-census-merged.txt)   # 15 names
    $ comm -23 <(names post/test-census-amended.txt) <(names post/test-census-merged.txt)   # 1 name

Those sixteen lines are exactly `master`'s own additions and its one rename, enumerated in §9.5. The
merge added no test of its own and lost none, which is consistent with the merge commit's finding that
`master` changed no line of `mokiterions-core/src/simulation.rs` in its thirty commits.

**The suite's cost moved again.** `tests/viability.rs` now runs **35.46 s** of the invocation's wall time
against **4.47 s** for the next slowest target, which is the engine's unit tier. §8 recorded 37.88 s
against 2.08 s; the ordering is unchanged and the second figure roughly doubled because the engine's
internal tier grew by 28 cases. No time bound is set here either.

### 9.4 The figures `SPEC-MOK-004` rule 11 states, measured at both commits

Rule 11 states one test-count figure per package and one for the workspace, and it obliges their
correction directly: *"a work order that adds a test corrects these figures here, and one that loses a
test has a defect."* `SPEC-MOK-002` states **no** test-count figure at all — its rules 7 to 9 fix the
placement rule and the target arrangement and no count — so rule 11 is the only place these figures live,
which is what its own text says when it records a split "because this paragraph is the only place the
workspace total is stated".

Both censuses are classified by package and tier, engine internal being the `unittests` names under
`simulation::`, engine public the seven `tests/` targets of `mokiterions-core`, and the observer's the
remainder:

| | `master`'s tip | rule 11 as recorded | The merge | Owed |
|---|---:|---:|---:|---:|
| engine, internal tier | 54 | 54 | **82** | +28 |
| engine, public tier | 31 | 31 | **40** | +9 |
| **engine total** | **85** | **85** | **122** | **+37** |
| observer, internal tier | 41 | 41 | **41** | 0 |
| observer, public tier | 100 | 100 | **101** | +1 |
| **observer total** | **141** | **141** | **142** | **+1** |
| **workspace total** | **226** | **226** | **264** | **+38** |

**The middle column is rule 11's own text and the left column is a measurement, and they agree in all
seven rows.** That agreement is why the right-hand column can be trusted: the classification used here
reproduces the specification's recorded figures exactly at the commit those figures were written for, so
it is the specification's own division of the corpus and not a second one invented to fit.

So the figures rule 11 owes are the observer's **142**, the engine's **122** and the workspace's **264**,
with the engine's split 82 internal and 40 public and the observer's 41 and 101. The 39 arrivals and one
rename that produce them are enumerated in §9.2's `comm` output and reconciled name by name in §§1 to 8.

**This section does not write that correction.** Amending `SPEC-MOK-004` is an act of the accountable
technical owner, not of an implementation agent, and rule 11's precedent is that the correction is
approved together with the work order rather than assumed by it — `SPEC-MOK-002`'s amendment record shows
the same shape, its 2026-08-19 row reading "Approved 2026-08-19 by the repository owner acting as
technical owner, together with `WO-MOK-010`. The implementation agent wrote the text and did not decide
the substance." The measurement above is what that decision is taken on, and the amendment is prepared
for the owner's signature in `amendment-approvals.md` rather than written into the specification here.

### 9.5 A defect in rule 11's `WO-MOK-013` row, reported and not absorbed

Measuring `master`'s tip against the branch point makes `WO-MOK-013`'s own arrivals visible, and they do
not match the row rule 11 records for them. The row reads *"The fourteen arrivals, each measured from the
target that runs it and none departing"*, over a table giving `tests/render.rs` 10, `src/render.rs` 2,
`tests/layout.rs` 1 and `tests/verification.rs` 1.

    $ comm -13 <(names baseline/test-census.txt) <(names post/test-census-master.txt)   # 15 names
    $ comm -23 <(names baseline/test-census.txt) <(names post/test-census-master.txt)   # 1 name
    tests/layout.rs :: the_log_is_ten_rows_only_where_both_thresholds_are_met

| Target | Recorded | Measured | |
|---|---:|---|---|
| `mokiterions-tui/tests/render.rs` | 10 | **+10** | agrees |
| `mokiterions-tui/src/render.rs` | 2 | **+2** | agrees |
| `mokiterions-tui/tests/verification.rs` | 1 | **+1** | agrees |
| `mokiterions-tui/tests/layout.rs` | 1 | **+2 −1** | `the_log_is_six_rows_wherever_it_is_present` and `the_reference_roster_interior_holds_the_whole_population` arrive; `the_log_is_ten_rows_only_where_both_thresholds_are_met` departs |
| **Total** | **14, none departing** | **15 added, 1 departed, net 14** | the net agrees; the composition does not |

**The net figure and the total of 226 are right, and the "none departing" clause is not.** A name that
was checked at the branch point is absent at `master`'s tip. It is a rename and not a loss of coverage,
and `WO-MOK-013` disclosed it in the place a reader is most likely to look: the test's own doc comment
reads *"This is `the_log_is_ten_rows_only_where_both_thresholds_are_met` renamed and corrected"*, and the
two viewports that asserted a ten-row log still assert a six-row one, so the withdrawn growth is asserted
absent rather than left untested. What is wrong is the row in the specification, which says no name
departed when one did.

This is reported rather than corrected here, for the reason rule 11 itself gives when it reports a
pre-existing figure defect instead of absorbing one: *"That is a pre-existing figure defect in this rule
and it is reported as one in `WO-MOK-013` rather than absorbed into the new figure."* It belongs to a
merged work order and its correction is the technical owner's, so it is carried into
`amendment-approvals.md` alongside §9.4's figures and is not folded silently into them.
