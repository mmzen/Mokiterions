+++
id = "VREC-MOK-026"
type = "verification_record"
title = "Verification candidate for WO-MOK-032"
status = "verified"
owners = ["engineering owner"]
created = "2026-08-29"
updated = "2026-08-29"
commit = "3bb12105d82a9bbbf0cb63b472e8c30f36fed9ea"
git_object_format = "sha1"
worktree_state = "clean"
prepared_at = "2026-08-29T21:35:49Z"
prepared_by = "engineering owner"
artifact_snapshot_sha256 = "4d72afe1bdff11355d9a2f4db62073f50ee72ac967c1c4758aff1c74eda8f126"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-032/handoff-check.md", "docs/engineering/simulation/evidence/WO-MOK-032/mutation-check.md", "docs/engineering/simulation/evidence/WO-MOK-032/test-run.md"]
evaluator_evidence_path = "docs/engineering/simulation/evidence/VREC-MOK-026-evaluator.json"
evaluator_evidence_sha256 = "4f500366462d5da855322aa725d6a1d23250f1ef82b37e371b43317ef81945b6"

verified_at = "2026-08-29T21:38:23Z"
verified_by = "assurance-owner"
[relations]
verifies_work_order = ["WO-MOK-032"]
conforms_to = ["VER-MOK-018"]

[[lifecycle_events]]
from = "ready"
to = "verified"
decided_at = "2026-08-29T21:38:23Z"
decided_by = "assurance-owner"
reason = "DR-VREC-DECIDE decided verified by the repository owner acting as accountable assurance owner on 2026-08-29, by selecting the presented option to approve and execute WO-MOK-032 through the verification record. All three permitted outcomes were available and each one's cost was measured. VER-MOK-018 case L20 is recorded as satisfied IN PART: the empty-credential clause and the ceiling clause are newly asserted, and the malformed-credential clause is disclosed as open by the owner's earlier decision, taken with the cost of the alternative measured. Four tests bound, each shown to fail against the removal of the guard it asserts, every removal reverted, no src file changed. One finding is carried and not repaired: L20 requires its conditions in the engine's binary target and the refusals live in the library target."
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-032` to candidate commit `3bb12105d82a9bbbf0cb63b472e8c30f36fed9ea`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## The decision: transitioned to `verified` on 2026-08-29

The repository owner, acting as accountable **assurance owner**, took this transition on 2026-08-29
under `DR-VREC-DECIDE`. The decision was put with all three permitted outcomes and each one's cost
measured, and it was taken **by selecting the presented option**, whose label was *"Approve and
execute through the VREC"* and whose description was that this work order be carried through
preparation, capture and `verified` in one session, "carrying the unreadable-arm deviation as a
partial disposition". That is the channel and the wording; no utterance is attributed to the owner
beyond the option they selected.

**`status` moved from `ready` to `verified` and the evaluator wrote `verified_at`, `verified_by` and
one lifecycle event. Nothing else in the frontmatter moved.** `commit`, `worktree_state`,
`prepared_at`, `prepared_by`, `artifact_snapshot_sha256`, the three `evidence_paths`, both evaluator
evidence fields and both relations stand exactly as `capture-verification` wrote them. The
transition was applied through the 0.8.0 evaluator rather than by hand-editing the field, so the
provenance is the tool's and not this agent's. The heading still says *Candidate* for the same reason
`VREC-MOK-025`'s does: a transition that moves only `status` does not get to rewrite the prose to
read better.

### What this record verifies, stated before anything else

**`VER-MOK-018` case `L20` is satisfied IN PART.** The case enumerates four conditions. Their
disposition at this candidate, each stated separately because they are not in the same state:

| Clause of `L20` | Disposition at `3bb1210` |
|---|---|
| with a credential present and no live-mode selection, no provider call occurs and the run replays or refuses | covered, **not by this work order** |
| with a live-mode selection and no credential, no provider call occurs and the condition is named without printing any value | covered, **not by this work order** |
| an empty **or malformed** credential is treated as absent | **empty: newly asserted. Malformed: NOT asserted** |
| a live run with no declared ceiling is refused before the first exchange | **newly asserted** |

The first two clauses are asserted by `mokiterions-core/tests/connector.rs`' pre-existing gate-table
loop and `no_credential_refuses_on_the_first_exchange`, which this work order did not add and this
record does not re-verify: they were bound by the record covering the work order that wrote them.
They are named here so that a reader of `L20`'s disposition is not left to infer which half moved.

**The malformed-credential clause is open, by the owner's decision and not by oversight.** Rule 13.3
gives absent, empty and malformed one treatment and `mokiterions-core/tests/support/canned_connector.rs`
implements the three as one arm; rule 13.1 puts the read in the connector and rule 13.4 forbids
either host to perform it, so no engine code distinguishes the cases and there is nothing in the
product to assert about. Reaching the malformed value needs the first `cfg(windows)` or `cfg(unix)`
in either package plus an `OsStr`-taking helper signature, in order to enter the same arm the empty
case already enters. Both routes were measured and put to the owner, who chose four tests with the
arm recorded. **This record discloses the gap rather than claiming the case.**

### The four tests bound here, and what makes them evidence

| Test | Site it covers | `SPEC-MOK-007` rule | Removal it survives, or not |
|---|---|---|---|
| `a_live_run_with_no_connector_is_refused_before_any_tick` | `src/cli.rs:532` | 13.1's selection half | **failed** when the guard was removed |
| `a_live_run_with_no_transcript_output_is_refused_before_any_tick` | `src/cli.rs:541` | 19.6 | **failed** when the guard was removed |
| `a_live_run_with_no_ceiling_is_refused_before_any_tick` | `src/cli.rs:551` | 14.6, 19.2 | **failed** when the guard was removed |
| `an_empty_credential_is_treated_as_absent` | `tests/support/canned_connector.rs:212` | 13.3 | **failed** when the guard was removed |

**The mutation result is what this record rests on, not the green run.** Before this candidate all
four subjects could be deleted with all 481 tests still passing, which is precisely why `L20` had no
assertion under it: a suite that passes either way is evidence of nothing. So each test was run
against code with its own subject removed and each was required to fail. All four did, and every
removal was reverted; `mokiterions-core/src/cli.rs` and
`mokiterions-core/tests/support/canned_connector.rs` are unmodified at this commit and appear in no
change set. The evidence records what was observed rather than a diff, because no `src/` file is in
the work order's execution scope.

**One of the four removals established a fact that was not certain in advance.** Whether an empty
environment value reaches a child process on Windows *as empty* rather than as absent. It does. Had
it arrived as absent, `an_empty_credential_is_treated_as_absent` would have entered rule 13.3's
absent arm, passed, and asserted nothing new — and would also have passed with the guard removed. It
failed, so the value arrives empty and the guard is what turns it into a refusal. Without that
measurement the fourth test would have been indistinguishable from a duplicate of the third
connector test.

### The figures at this commit, measured and not carried from a commit message

| Figure | Value |
|---|---|
| `cargo test --locked --workspace` | **485 passed, 0 failed, 3 ignored** |
| the same, at the base `d7d438c` | 481 passed, 0 failed, 3 ignored |
| `Mokiterions` `tests/cli.rs` | 30 to **33** |
| `Mokiterions` `tests/connector.rs` | 13 to **14** |
| every other target in either package | unmoved |
| `cargo fmt --all -- --check` | exits 0 |
| `validate` | **198 artifacts, 0 errors, 142 warnings** |
| warning count against the base | unchanged |
| `src/` files changed in either package | **0** |

### The finding this record carries and does not repair

**`L20` requires its conditions "in the engine's binary target", and the four refusals are not in
it.** They are in `mokiterions-core/src/cli.rs`, which `src/lib.rs` declares as `pub mod cli;` and
which `SPEC-MOK-002` therefore places in the **library** target. Two readings are available and the
record does not choose between them: the phrase distinguishes the engine from the observer, which
the same sentence goes on to do explicitly by sending the observer's case to `L32`; or it names a
Cargo target, in which case it is inaccurate about where the code it governs lives.

The tests were written to the first reading, on an approved specification's authority rather than on
this agent's: `SPEC-MOK-002` rule 8 assigns `tests/cli.rs` "argument parsing: defaults, order
independence, policy selection, **duplicate and missing values**", and a missing required option is a
missing value in argument parsing, while `tests/process.rs` holds the process boundary "each with its
exit code". **The wording is reported and not amended.** `VER-MOK-018` is a verification contract and
moving its words is not something a work order that only adds tests may do.

A second, previously recorded finding stands unchanged: `L20` appears in this contract's enumerated
matrix and nowhere in its prose, so the matrix and the prose divide `L20` and `L32` differently.

### What this record does not do

- It does not close the malformed-credential clause of `L20`, and it does not claim it.
- It does not re-verify any case bound by an earlier record, and no earlier record is re-opened.
- It does not amend `VER-MOK-018`, `SPEC-MOK-002` or `SPEC-MOK-007`, and it moves no figure in any
  of them.
- It does not authorize `WO-MOK-027`'s live run, which needs its own written authorization naming a
  horizon, a seed set and a spend ceiling, and which the owner has decided lands after a further
  work order.
- It does not verify the rule 14 fifth price, which is not in this candidate and which will amend
  three of the four tests bound above.
- It does not push, tag, release or publish anything, and it does not open a pull request.
