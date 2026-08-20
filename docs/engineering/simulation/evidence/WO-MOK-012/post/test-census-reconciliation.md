# `VER-MOK-012` row 248: the two test censuses reconciled name by name

| Field | Value |
|---|---|
| Baseline | `39662d13abd08e3410648d1c59ad38384f8ad2d2` — `baseline/test-census.txt`, 212 names |
| Candidate | `7c4aef3967406c05d80da963695898b77f5329e9` — `post/test-census.txt`, 249 names |
| Invocation | `cargo test --locked --workspace --no-fail-fast`, from the workspace root |
| Exit code | `101` — three cases fail; see §4 |
| Date | 2026-08-20 |

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
