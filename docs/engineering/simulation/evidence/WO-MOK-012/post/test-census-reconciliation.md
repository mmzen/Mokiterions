# `VER-MOK-012` row 248: the two test censuses reconciled name by name

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

The distribution is the one `VER-MOK-012` predicts: 28 of the 38 are internal-tier cases, because the
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
`REQ-MOK-043` and `REQ-MOK-049`, and all three fail from one cause, which `escalation.md` states and
measures. **No case that passed at the baseline fails at the candidate**: the 211 retained names all
report `ok`, and so does the renamed one.

`--no-fail-fast` is on the invocation and the baseline's was not, because without it cargo stops after
the first failing target and eight later targets never run — a census taken from that log would be
missing names and would read as removals. The plain invocation was run too and exits `101` identically;
the flag changes which targets execute, not any verdict.

## 5. What this file does not establish

It reconciles names and one body. It does not assert that the 38 new cases are the right cases, that
they cover what `VER-MOK-012` requires them to cover, or that any of them is well written — the
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

    $ python docs/engineering/simulation/evidence/WO-MOK-012/analysis/test-census.py <log> <out>
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

**The second rename, and why row 248 does not reach it.** `VER-MOK-012` oracle 5's outcome half was
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
amendment's, not the implementation's: it is approved in `VER-MOK-012`'s amendment record with the
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
line the engine compiles** — one is this evidence directory, the other is `VER-MOK-012` and
`WO-MOK-012` prose. So this log is `59d61b9`'s suite, on the same reasoning `post/capture-state.txt` §5
gives for naming a capture's commit.

**Both pairs are kept and neither replaces the other.** §§1 to 5 reconcile the 249-name census and name
its three failures, and `escalation.md` cites those three by name as the measured failure that stopped
the work; overwriting that file would leave five sections and an escalation record without their
artifact. The files are named for what they are, and this is which section reads which:

| File | Read by | State |
|---|---|---|
| `post/test-run.txt`, `post/test-census.txt` | §§1 to 5, `escalation.md` | `7c4aef3` — 249 names, 246 pass, 3 fail, exit `101` |
| `post/test-run-amended.txt`, `post/test-census-amended.txt` | §§6 to 8 | after the `REQ-MOK-048` amendment — 250 names, 250 pass, exit `0` |

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

The first three lines are `VER-MOK-012`'s third amendment row landing in the suite: oracle 5's outcome
check was one test asserting a monotonicity tripwire *and* a rank-correlation band, and the amendment
removed the band and put a turn-position survival bound in its place. One test became two because one
obligation became two, which is why this is `+2 −1` rather than a rename. The last two lines are §7's.

**The suite's cost moved with it**, and the figure belongs here rather than in a reader's assumption:
`tests/viability.rs` now runs 37.88 s of the whole invocation's wall time, against 2.08 s for the next
slowest target, because the turn-position bound is evaluated over the declared 200-seed diagnostic set at
1,000 ticks each. `VER-MOK-012` states no time bound on the suite and this section sets none; it records
what one invocation now costs, so that a later reader who finds `cargo test` slow knows which target to
look at and why it is expected.
