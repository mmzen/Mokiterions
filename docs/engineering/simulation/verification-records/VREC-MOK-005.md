+++
id = "VREC-MOK-005"
type = "verification_record"
title = "Verification candidate for WO-MOK-005"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"
commit = "f3613701f3c55d3f3d849747c8cf0790a5729c14"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-19T07:50:45Z"
artifact_snapshot_sha256 = "88813d934ba6bd47b7a4e35593c8101951d04ede4215ee0fef5baf7e280431ea"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-005/README.md", "docs/engineering/simulation/evidence/WO-MOK-005/additivity-proof.txt", "docs/engineering/simulation/evidence/WO-MOK-005/boundary-and-security-review.md", "docs/engineering/simulation/evidence/WO-MOK-005/completion-summary.md", "docs/engineering/simulation/evidence/WO-MOK-005/dependency-review.txt", "docs/engineering/simulation/evidence/WO-MOK-005/export-fidelity.txt", "docs/engineering/simulation/evidence/WO-MOK-005/exports.txt", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed0-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed1-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed123-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed42-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/exports/events-seed777-ticks20.log", "docs/engineering/simulation/evidence/WO-MOK-005/frames.txt", "docs/engineering/simulation/evidence/WO-MOK-005/layout-and-viewports.txt", "docs/engineering/simulation/evidence/WO-MOK-005/manual-assessment.md", "docs/engineering/simulation/evidence/WO-MOK-005/non-perturbation.txt", "docs/engineering/simulation/evidence/WO-MOK-005/requirement-to-test-mapping.md", "docs/engineering/simulation/evidence/WO-MOK-005/resilience.txt", "docs/engineering/simulation/evidence/WO-MOK-005/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-005/terminal-restoration.txt", "docs/engineering/simulation/evidence/WO-MOK-005/test-run.txt"]

[relations]
verifies_work_order = ["WO-MOK-005"]
conforms_to = ["VER-MOK-005"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-005` to candidate commit
`f3613701f3c55d3f3d849747c8cf0790a5729c14`. An accountable assurance owner must review the evidence
and transition the record to `verified`; producing this record approved nothing, and nothing on this
branch is tagged, released, published or deployed.

The record is intentionally created after the candidate commit it names, avoiding self-referential
commit metadata.

## How this record was produced, and why not by `harnessctl capture-verification`

**This file was hand-authored. The harness command that normally writes it refused, and the reason is
structural rather than incidental, so it is stated here rather than left for a reader to discover.**

`capture_verification` in the installed harness package's `se_harness/provenance.py` refuses when the
record ID already exists in the validated catalog — `harnessctl: artifact ID already exists:
VREC-MOK-005` — and refuses again if the output path exists. A re-capture under the same ID is
therefore not something the command supports: it can create a record, not replace one. Removing the
existing file first was declined as an irreversible local deletion of a governed artifact, and I did
not work around that.

What the command derives, I derived with the same operations, on a clean worktree at the commit named
above:

| Field | How it was derived |
|---|---|
| `commit` | `git rev-parse HEAD` |
| `git_object_format` | `git rev-parse --show-object-format` |
| `worktree_state` | `git status --porcelain` empty, which is the condition `require_clean_worktree` enforces with *"revision provenance requires a clean Git worktree"* |
| `artifact_snapshot_sha256` | `sha256` of `target/harness-dashboard/dashboard-data.json` after a PASS generation, which is the file and the order the command uses |
| `verified_at` | UTC at the moment of that derivation |

The digest's dependence on `HEAD` was measured rather than assumed, and `static-checks.txt` carries
the three measurements: `dashboard-data.json` embeds `repository.revision` and an `observed_revision`
per record, both of which are `HEAD`. Editing evidence prose does not move the digest, because an
evidence file carries no frontmatter and is not a typed artifact; committing does. Editing a typed
artifact moves it without any commit at all, which is measurable here — writing the body of this record
changed the digest from the value in the frontmatter above to `69c4d706…` on an otherwise untouched
tree. The frontmatter therefore names the digest at `f361370`, the last clean commit before this file
was written, which is the same value `capture-verification` would have derived: it reads `HEAD` and
generates the dashboard *before* the record it is about exists. The value was cross-checked two ways —
the generator prints its own `Snapshot:` digest, and an independent `sha256` of the file it wrote
agrees.

So the digest is commit-bound by construction, which is what makes it provenance, and it is why this
record names the commit it does rather than the commit that carries the source change. `360a5e4` is the
layout fix; `f361370` is the head of the branch at the moment the evidence packet was complete and
correct, after three further commits that corrected the packet's own prose. Both are on this branch,
and `git merge-base --is-ancestor` confirms `360a5e4` is an ancestor of the commit named above, so the
record binds a tree that contains the change it is about.

The frontmatter's shape, field order and the two paragraphs under the heading follow the generator's
output, so this file can be diffed against `VREC-MOK-001` through `VREC-MOK-004` without noise. One
sentence deviates from the template and is not verbatim: where the generator writes "this command did
not approve, commit, tag, release, or publish anything", no command ran here, and commits *were* made
on this branch under the repository owner's explicit authorization. The paragraph above says what is
true instead.

## Why this record was recaptured, for the third time

This is the fourth candidate under this ID. Its three predecessors bound `b3fed6a7`, `6601d29b` and
`9d9641fe`, in that order. All three described trees that no longer exist, and all three are ancestors
of the commit named above, so the sequence is one lineage rather than competing accounts. The first
two were retired over the packet's own defects — an additivity baseline that credited this work order
with `master`'s changes, and a disclosure about the observer's test tiers. The third, `9d9641fe`, is
retired for two reasons that are independent of each other.

**A defect the repository owner reported on 2026-08-18.** The observer sometimes drew no left panel,
and making the window smaller could bring it back: a 160 × 40 terminal showed no roster while
120 × 40 — same height, one third narrower — showed one. The cause was in the specification, not the
implementation. `SPEC-MOK-003` rule 5 expressed a two-dimensional space as an ordered ladder of four
tiers, and the ladder had a hole: `W ≥ 140` with `38 ≤ H < 44` matched no row and fell through to
`otherwise`, which excluded the roster, the inspector and the log together. A test asserted
`tier_for(140, 43) == Tier::D` and passed, so the behavior was specified, implemented and pinned, and
the retired candidate offered that state for verification. Rule 5 is amended, the implementation
follows it, and `Tier`, `Tier::label`, `tier_for` and the `Panes::tier` field are deleted.

**A restructuring by another work order.** `WO-MOK-006` moved the engine package into
`mokiterions-core/` and moved 77 of the observer's tests into eight files under
`mokiterions-tui/tests/`. Every path and every per-binary figure in the retained packet was therefore
wrong even where the totals were still right. All twenty-one evidence files are re-derived against
the merged tree, and each one separates that work order's contribution from this one's, so the test
count change is attributable to this change alone.

Both are recorded because a reader comparing this record against the retired one will find figures
that moved for two unrelated reasons, and attributing all of them to the layout fix would overstate
its blast radius.

A related question a reader will reasonably ask: `VREC-MOK-006` is `verified` and records 97 public
items and 169 workspace tests, both of which this work changes to 94 and 172, and it is **not**
re-captured. That is the rule rather than an inconsistency. A `verified` record is a statement an
accountable owner made about a named tree, and it was correct at that tree; re-writing it would rewrite
the owner's judgement. This record is `ready` — a candidate no owner has accepted — so replacing it
retires nothing but my own draft. `VREC-MOK-001` through `VREC-MOK-004` are likewise left untouched.
`SPEC-MOK-004`'s amendment record states the same principle in its own words.

## What this record claims

`WO-MOK-005` is `in_progress` and `VER-MOK-005` is `approved`. At candidate commit
`f3613701f3c55d3f3d849747c8cf0790a5729c14`, **every automated case, invariant, static check, security
check and resilience check in `VER-MOK-005` as amended was executed**, and each row of its
requirement-to-evidence matrix is mapped to a named test or a retained file in
`requirement-to-test-mapping.md`.

| Gate | Result |
|---|---|
| `cargo test --workspace` | 172 passed, 0 failed, 0 ignored, 0 filtered (60 engine, 112 observer) |
| `cargo test -p Mokiterions` | 60 passed, 0 failed — 37 inline in `mokiterions-core/src/simulation.rs`, 23 across the five files in `mokiterions-core/tests/` |
| `cargo fmt --all -- --check` | no differences |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | 0 findings, and 0 again for `-p Mokiterions` alone |
| `cargo build --workspace` and `-p Mokiterions` | clean |
| `cargo tree -p Mokiterions` | resolves to the package alone on every edge kind |
| `harnessctl validate` and `scripts/validate_engineering_artifacts.py --root .` | PASS — 68 artifacts, 0 errors, 0 warnings, all four planes clean |
| `harnessctl doctor` | PASS — 81 lines, every managed file `unchanged` |
| `preflight --work-order WO-MOK-005 --phase review` | PASS — `in_progress`, the form CI runs |
| `scripts/generate_harness_dashboard.py --root .` | PASS — 68 artifacts, 211 relations, 0 errors, 8 warnings against 6 for a clean extraction of `origin/master` at 05dc6ac |

Every cargo gate was run from clean: both packages are `cargo clean`ed before each invocation, and
`clippy -p Mokiterions` is run before the workspace pass, so no recorded line is a cache hit standing
in for a check that did not run. `static-checks.txt` and `test-run.txt` retain the transcripts.

Five obligations `VER-MOK-005` singles out, each measured rather than argued:

- **Additivity.** Measured against `origin/master` at 05dc6ac, which `git merge-base --is-ancestor`
  confirms is an ancestor of this branch's head, so the comparison is a before-and-after of one
  lineage. The result is stronger than any retired candidate's and needs fewer checks to state: the
  **whole engine package** is byte-identical, not merely its test corpus.
  `git diff --stat origin/master -- mokiterions-core` and
  `git status --porcelain -- mokiterions-core` are both empty, so no source, no manifest, no test file
  and no inline test module differs, and the engine's 60 tests pass on those bytes — the figure
  `WO-MOK-004` recorded, in the placement `SPEC-MOK-002` rules 7 to 10 require.
- **Layout monotonicity**, which the amended contract adds. Over the 13,026 adjacent viewport pairs
  in `34 ≤ W ≤ 200` by `22 ≤ H ≤ 60`, rule 5 as amended has **0** pairs where enlarging the terminal
  removes a pane. The superseded tier table has **12**, measured by transcribing it from `layout.rs`
  at 05dc6ac rather than paraphrasing it, so the sweep is a test that fails against the rule this work
  replaces — a negative control, without which a clean sweep would prove nothing. Four of the nine
  declared viewports resolve differently under the two rules, and two of them, `160 × 40` and
  `140 × 43`, are the reported defect itself. Pane *presence* is monotone; canvas *area* deliberately
  is not, and rule 5 declares each of the three sizes where a new pane costs the view rows or columns.
- **Non-perturbation.** Observed and unobserved runs are identical on seeds `0`, `1`, `42`, `123` and
  `777`, with a frame drawn every tick and an interaction applied every tick.
- **Export fidelity.** The observer's exports are byte-identical to the engine binary process's own
  stdout across ten comparisons — five seeds at 20 ticks against the retained files, five at the
  default 100 ticks against `sha256` digests.
- **Layout geometry.** All nine renderable canvas interiors measured exactly as derived — 67 × 32,
  67 × 32, 67 × 28, 47 × 32, 47 × 31, 71 × 36, 71 × 24, 51 × 24 and 32 × 16 — with `33 × 21` refused
  before the terminal is entered, its dimensions written to stderr and exit code 2. Six viewports were
  declared before the amendment and five of their figures are unchanged. The sixth, `100 × 30`, moves
  from a 98 × 24 interior to 51 × 24, because the roster is now placed there; `160 × 40`, `140 × 43`
  and `120 × 30` are newly declared.

## What this record does not claim

**No manual assessment in `VER-MOK-005` was executed. All seven are outstanding and none has an
author.** `VREC-MOK-002` was able to claim that every case *and manual assessment* had been executed;
this record cannot, and the difference is deliberate rather than an omission in drafting.

`manual-assessment.md` records the reason. Three of the seven require an interactive terminal, and the
observer cannot be driven from this environment: `crossterm` on Windows reads the console input
buffer, so a keypress piped to the process never reaches it — measured as exit code 124, 7,017 bytes
of frame output and empty stderr — and `winpty` cannot supply a pseudo-terminal without a tty to read
its size from. The remaining four are judgments about what a person perceives, and recording my
reading of a buffer dump as a pass would restate an automated assertion in prose, which is the exact
gap `VER-MOK-005`'s residual-uncertainty section says cannot be automated away.

The consequence for this record is specific. `VER-MOK-005` states that a pass on every automated case
combined with a negative answer to the two-hundred-tick instrument assessment "is an adverse
observation requiring product review, because the automated cases verify the parts and this assesses
the instrument". The automated half of that condition is satisfied. The other half is unknown. So this
record establishes that the parts behave as specified; it does not establish that the instrument
answers the questions `INT-MOK-004` names, and it does not establish that anything is legible on a
real terminal.

**It also does not establish that the reported defect is gone from the operator's screen.** What is
measured is that no viewport in a 167 × 39 grid loses a pane when it grows, in an in-memory buffer.
The owner reported a symptom on a real terminal, and the assessment that closes the loop between
those two is manual assessment 1, which is outstanding.

Every automated result bound here is an assertion about an in-memory character buffer.

## Amendments to approved artifacts that this record does not carry

**Eleven provisions across four approved artifacts are amended in the tree and every one is recorded
as OUTSTANDING, because each is the technical owner's act and the owner has not taken it.** The
amended text is written; the approval is not given. This record cannot supply it, and transitioning
this record to `verified` would not supply it either — an assurance owner's verification is a
statement about evidence, not about another role's approvals.

- **`SPEC-MOK-002`, four provisions** (amendment record, 2026-08-18): rule 1's "no second package, no
  workspace" narrowed to a workspace of exactly two packages, on the approved `REQ-MOK-026` the
  clause reserved the exception for; rule 3's freeze on the engine's `simulation.rs` contents scoped
  to the `WO-MOK-003` restructuring it was written for; rule 5's closed public enumeration grown by
  the observation surface, under rule 5's own growth clause; rule 6's prohibition narrowed from five
  named value types, and from the return-value path, to the capability it was written to deny.
- **`ARCH-MOK-001`, one provision** (2026-08-18): the prohibition on public items narrowed from
  "mutable **or owned** authoritative state" to a mutable borrow of, or a reference into, that state.
  As written it forbade the owned, reference-free snapshots `SPEC-MOK-003` specifies — the capability
  `REQ-MOK-019` through `REQ-MOK-025` and `REQ-MOK-027` require, and one that grants no mutation.
- **`SPEC-MOK-003`, one provision** (2026-08-18): rule 2 of *Data and interface contracts*, which said
  `advance_tick` was the only `&mut self` method on the surface that changes state. It is not; `run`
  is a second. Found by measurement during implementation rather than before it, so it is not among
  the preconditions `WO-MOK-005` lists.
- **`SPEC-MOK-004`, five provisions** (2026-08-19): rules 6, 9, 11, 12 and 13, following the deletion
  of `Tier` that rule 5 as amended authorizes. Rule 6 closes a public interface by provenance and its
  recorded figure falls from **97 items to 94**; the rule had a growth clause and no counterpart for
  removal, so it gains a **Reduction** clause. Rules 9 and 11 carry the counts, rule 13 a stale term,
  and rule 12's second paragraph is scoped to the `WO-MOK-006` restructuring so that renaming a test
  whose name cites a deleted rule is not itself a violation. **The owner approved rule 5 without being
  shown this consequence.** It was found afterwards, by measuring the interface against the rule, and
  it is reported rather than treated as covered by that approval.

`WO-MOK-005` names the first five provisions — `SPEC-MOK-002`'s four and `ARCH-MOK-001`'s one — as
approval preconditions and states plainly that it "is not verifiable until they are given or the scope
is changed to avoid needing them". None of the five could have been part of the 2026-08-17 approval:
the wording each narrows reached `master` afterwards, under `WO-MOK-003`. The sixth arose from this
work's own measurement, and the last five from this work's own change. Nothing about mutation,
dependency direction, determinism or observable behavior is relaxed by any of them, and the engine
package's dependency table stays empty.

Two amendments **are** approved, both on 2026-08-19 and both by the repository owner: `SPEC-MOK-003`
rule 5, and `VER-MOK-005` itself. The owner chose that direction over removing adaptive layout
altogether after being shown both options, reviewed the text, and directed the implementation in the
same act. The implementation agent drafted both texts and **recorded both approvals on the owner's
explicit instruction**; each artifact says so, and says that the agent holds no authority over either.
Three further amendments — `SPEC-MOK-002`'s and `SPEC-MOK-003`'s path re-basings and `ARCH-MOK-002`'s
target-shape change — were approved on 2026-08-18 by way of `ADR-MOK-004` and belong to `WO-MOK-006`,
not to this work order; they matter here only because they moved the artifacts this evidence cites.

**One consequence of this re-capture reaches two approved amendment rows, and correcting them is not
this work's act.** `SPEC-MOK-003`'s 2026-08-18 row states that "`VREC-MOK-005`, which binds this
specification to `WO-MOK-005`'s commit, is not edited", and `ARCH-MOK-002`'s states that
"`VREC-MOK-005` binds this architecture's 2026-08-17 content to `WO-MOK-005`'s commit and is not
edited". Both were accurate as statements about what `WO-MOK-006` did. Neither is accurate as a
standing claim: this record is edited, and at the commit it now names, `SPEC-MOK-003` and
`ARCH-MOK-002` both carry their 2026-08-18 amended content. So this record binds the amended
architecture and the amended path layout, not the 2026-08-17 content those rows describe. Amending an
approved row in an approved artifact is the technical owner's act, so this is disclosed and not
touched. The substance is unaffected: `ARCH-MOK-002`'s package split, read-only observation surface
and dependency boundary are what this evidence measures, and none of them changed.

## What the accountable assurance owner must weigh before verifying

The outstanding manual assessments and the eleven outstanding amendment provisions above are the first
two items. `completion-summary.md` discloses seventeen numbered findings and two further notes. None
is a failure against `VER-MOK-005`; each is stated so that verification, if given, is given knowingly.

Twelve of them bear on how much the green gates mean, and they are listed below in this record's own
numbering, which is not the packet's. Ten map to a numbered disclosure — this record's 1 to 8 are the
packet's 1, 2, 3, 6, 8, 9, 11 and 14, and this record's 10 and 11 are the packet's 16 and 17. Two are
not in that list: item 9 is about these records themselves, and item 12 is argued in
`additivity-proof.txt:173-186`. Seven numbered disclosures are not restated here — the packet's 4, 5,
7, 10, 12, 13 and 15 — because each is either resolved in the implementation with the resolution
recorded, or superseded by a later item; they are not thereby less worth reading.

1. **`ADR-MOK-003` line 205 is factually wrong.** It says `Cargo.lock` acquires 57 entries; the file
   has 182. 57 is the `cargo tree --edges normal` crate count, a different measurement — the lock file
   records every platform's and every optional feature's packages, while `cargo tree` resolves for
   this host. The conclusion the sentence draws from the number holds, and holds more strongly at 182.
   The number does not. Correcting it amends an approved artifact.
2. **The 57-crate figure is one of four readings** and is stated twelve times as though it were the
   only one: `SPEC-MOK-003` lines 59, 584, 645; `ARCH-MOK-002` lines 28, 159; `ADR-MOK-003` lines 80,
   96, 144, 176, 177, 186, 202. Measured on this tree: 57 with `--edges normal`, 59 counting build
   dependencies, 37 excluding proc-macro crates, 61 counting both workspace members.
   `dependency-review.txt` records all four and names the two crates the build-edge reading adds and
   neither binary links — `rustc_version` and `semver`.
3. **`SPEC-MOK-003` rules 4 and 5 cannot both be satisfied literally.** Rule 4's roster mockup needs
   about 87 columns of bars plus rule 4.5's reserved fourth slot, about 26 more; rule 5 gives the pane
   a 45-column interior. Resolved as `min(20, (interior − 27) / 3)` at `render.rs:496-499`, which
   makes the reserved slot zero-wide at the reference roster — absent there rather than empty, which
   is not what rule 4.5 describes.
4. **`SPEC-MOK-003` rule 2 does not hold as written, and the deviation predates this branch.** The
   public surface exposes two `&mut self` operations that change simulation state, not one:
   `advance_tick` and `run`. `run` is the `REQ-MOK-010` whole-run entry point and was **already
   publicly reachable at `origin/master`** — 05dc6ac has `pub mod simulation` at
   `mokiterions-core/src/lib.rs:27` and `pub fn run<W: Write>(&mut self, …)` at
   `mokiterions-core/src/simulation.rs:1205`, both created by `WO-MOK-003` and bound by a `verified`
   `VREC-MOK-003`. This work did not expose it. The observer never calls it, and
   `grep -rn 'pub fn .*&mut self' mokiterions-core/src/` returns exactly those two.
5. **`VER-MOK-005` acceptance scenario 2 describes an unreachable state.** No shipped decision source
   can have a proposal rejected, asserted over 400 ticks of both policies, so the rejection
   presentation is verified only through a `#[cfg(test)]` hook that injects a self-contradictory
   decision. Manual assessment 4 therefore cannot be performed by running the observer at all.
6. **A world with no standing resources is likewise unreachable** and is verified through a second
   `#[cfg(test)]` hook. Both hooks are compiled out of the shipped binary.
7. **`SPEC-MOK-003` rule 9.2's territory subject filter is not expressible** with the engine's
   records, whose distinct subjects in a run are `M##`, `F####` and `world` — territory is a field
   inside `result`, not a subject. Closing it needs a filter dimension that reads `result`, or a change
   to the engine's subjects. Both are specification decisions, not observer fixes.
8. **Four matrix rows had no test behind them at the first evidence pass** and were closed by four new
   tests before the first record was prepared. An audit that found four is not evidence that there is
   no fifth.
9. **Claims in all three earlier candidate records were wrong, and I wrote them.** The first
   candidate's additivity baseline attributed `master`'s own `WO-MOK-003` and `WO-MOK-004` changes to
   this work order, and its fourth disclosure said `Simulation::run` became publicly reachable through
   this work, which `origin/master` disproves; both were corrected in the second. The second disclosed
   nothing about the observer's test placement, and retiring it was what added that disclosure. The
   third then stated as its item 11 that all 109 observer tests were internal — true when written, and
   obsolete within a day, because `WO-MOK-006` moved 77 of them into the public tier. The third also
   offered for verification a layout rule that a passing test had pinned and that the owner then
   reported as a defect. In every case the correction came from
   re-measuring on a changed tree, or from the owner using the software — not from review of the prose.
   That is a fact about how far unreviewed prose in a packet can be trusted, and it is the reason this
   record states its own derivation in full.
10. **`SPEC-MOK-003` rule 5's tier table was not monotone, and a passing test pinned it.** This is the
    defect the owner reported, and the finding is that specification, implementation and test agreed
    with each other and all three were wrong. A test can only be as good as the rule it asserts, and
    the third candidate offered this one for verification. The amended rule is measured over the
    whole plane rather than at named sizes, which is what makes the new assertion stronger than the
    one it replaces rather than merely different.
11. **A newly reachable branch, disclosed rather than closed.** The roster now appears from 100
    columns at any height, so a terminal can be wide enough for it and too short for twelve two-line
    entries: `100 × 22` presents 8 and reports `hidden 4`; `100 × 26` presents 10 and reports
    `hidden 2`. The branch behaves correctly and **no test asserts it**, because no test reads the pane
    title. Under the superseded rule it was unreachable at every declared viewport, which is why the
    retired candidates had no occasion to record it at all. Closing it now would need an eleventh
    test in a file whose count `SPEC-MOK-004` rule 9 fixes at 10 — an amendment to the rule whose
    amendment is already outstanding.
12. **The stop condition in `WO-MOK-005` was read narrowly, and the reading is disclosed rather than
    resolved.** The work order says "Every existing test must pass unmodified. A test that must change
    to accommodate this work is a stop condition, not a task, because this change is specified as
    additive." Three observer tests changed: two asserted the deleted tier table, and one was renamed
    off the word "tier" with its assertion unchanged. The narrow reading is applied — the constraint
    sits next to "Preserve everything verified under `VREC-MOK-001` and `VREC-MOK-002`", its stated
    reason is additivity with respect to the engine, and the engine's test code is byte-identical. On
    the broad reading, any test in the tree, the stop condition fired on 2026-08-19 and the owner's
    approval of the rule 5 amendment together with the direction to implement is what answers it,
    because a specification amendment that deletes a rule cannot leave the test asserting that rule
    standing. **Reading a constraint down is not the implementation agent's act**, and the distinction
    only became live when `WO-MOK-006` put the observer's tests on `master`, after the constraint was
    written. `additivity-proof.txt:173-186`, under *What is left for the owner to decide*.

Three coverage limits, narrower than the above. Non-perturbation is measured under a script driving 13
of the 25 controls `state.rs` binds. The arrow-key aliases `Left`, `Right`, `Up` and `Down` are bound
at `state.rs:448-457` and exercised by no test in either tier. And the dashboard reports two warnings
this work adds, both the same finding stated twice — `ARCH-MOK-002` predates `SPEC-MOK-003` and
`SPEC-MOK-004`, which the amendments moved to 2026-08-19 while it stays at 2026-08-18. Reassessing it
is the technical owner's call; the expected outcome is a re-affirmation, because no threshold appears
in it and the package split, the read-only surface and the dependency boundary are untouched.

## Behavioural observations that are not failures

- Seed `42` under `--policy baseline` reaches extinction on tick 142, which is why the long-run case
  and manual assessment 1 use `--policy reference`.
- Under `--policy reference`, 11 of 12 Mokiterions die by tick 10,000. Living count plus deaths sums to
  12 throughout, so `REQ-MOK-020`'s corroboration contract holds. World viability at the default
  density is `INT-MOK-002`'s subject, not this work order's.
- "Layout tier" survives as a term in `REQ-MOK-028`, `CAP-MOK-005`, `INT-MOK-005`, `WO-MOK-006` and
  `VER-MOK-006`, in clauses whose subject is that the `WO-MOK-006` restructuring changed nothing an
  operator can see. Those clauses were true at the commits they bind and are not re-opened here. What
  they now lack is a definition of the term, since the artifact that defined it no longer does.

## Scope of the transition being requested

Transitioning this record to `verified` would record that the assurance owner accepts the automated
evidence at this candidate commit **with all seven manual assessments outstanding and eleven
provisions across four approved artifacts awaiting the technical owner**. It would not perform those
assessments, would not supply those approvals, and would not release, tag, publish or deploy anything.
If either set is to gate verification instead, this record should stay `ready` until the assessments
are performed and their author recorded in `manual-assessment.md`, and until the amendments are
approved by the role that owns them.

The record is `ready`. Transitioning it is the accountable assurance owner's act, and no agent may
take it.
