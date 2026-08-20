+++
id = "VREC-MOK-013"
type = "verification_record"
title = "Verification candidate for WO-MOK-013"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-20"
commit = "41c20cada40017b8b50ceb655f92b0ab48ed454e"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-20T15:38:30Z"
artifact_snapshot_sha256 = "043279964c2b406e86c269f9e853b3d91415aa8ea37def1920ca829067d2dfbb"
evidence_paths = [
  "docs/engineering/simulation/evidence/WO-MOK-013/README.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/analysis/interface.py",
  "docs/engineering/simulation/evidence/WO-MOK-013/analysis/test-census.py",
  "docs/engineering/simulation/evidence/WO-MOK-013/baseline-comparison.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/closing-review.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/completion-summary.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/discoverability-assessment.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/discoverability-frame-floor.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/discoverability-frame.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/engine-untouched.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/frames.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/gauge-resolution.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/gauge-resolution.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/interface.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/log-height.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/manual-assessment.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/non-perturbation-observed-export.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/non-perturbation-unobserved.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/non-perturbation.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/static-checks.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/test-census.md",
  "docs/engineering/simulation/evidence/WO-MOK-013/test-census.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/test-output.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/validation.txt",
  "docs/engineering/simulation/evidence/WO-MOK-013/wo013-oracle.rs",
]

[relations]
verifies_work_order = ["WO-MOK-013"]
conforms_to = ["VER-MOK-013"]
+++

# Verification Record Candidate

**This record is `ready`. It is not `verified`, and nothing below is a verification decision.** It was
prepared on 2026-08-20 by `harnessctl capture-verification` from the pinned 0.4.0 harness, on the repository
owner's instruction, verbatim: *"you can transition WO-MOK-013 as implemented, and create VREC-MOK-013"*.
That instruction moved the work order's status and created this file. It stated no judgement on the
evidence, and `DECISION_RIGHTS.md` reserves the transition it would take: *"Automation may prepare `ready`
verification and release records from bounded Git observations. Only accountable assurance and release
owners may transition those records to `verified` or `released`; record preparation never creates commits,
tags, or publications."* The `ready` → `verified` act is therefore **outstanding and named as such**, and the
harness raises the signal itself: `scripts/inspect_engineering_artifacts.py` reports `decision_required ->
review-assurance-decision (assurance-owner) [VREC-MOK-013]`, against a queue that was empty before this
file existed.

**Read *What this record does not claim* before relying on any figure here.** Two items are load-bearing.
One of the three requirements this work order implements — `REQ-MOK-048` — is **not verified**, and cannot
be until a manual assessment for which no admissible assessor is available is taken. And **four of this
chain's identifiers, this record's own among them, are also claimed by a branch that is already pushed.**

## The front matter is the capture's provenance, not a decision's

`commit`, `git_object_format`, `worktree_state`, `verified_at`, `artifact_snapshot_sha256`, both relations,
all 25 `evidence_paths` and the `title` — which reads *candidate*, because that is what this is — are
exactly what `capture-verification` produced. **`verified_at` is a capture timestamp and not a verification
decision**: the harness figures below were taken at `2026-08-20T15:38:30Z` from a worktree that `git status
--porcelain` reported as empty at `41c20ca`. Following `VREC-MOK-011`, they are left as captured; a decision
does not re-measure provenance, and when the transition is taken it will move `status` and `updated` and
nothing else.

**`artifact_snapshot_sha256` names the graph as it stood before this file existed, and that was measured
rather than assumed.** With this record moved out of the tree, `scripts/check_engineering_harness.sh`
reports **108 artifacts, 372 relations**, snapshot `043279964c2b406e86c269f9e853b3d91415aa8ea37def1920ca829067d2dfbb` —
the value recorded above. With it in place: **109 artifacts, 374 relations**, snapshot
`877862790cc463cf10a0232006eba377aba727f0eccf34a936db940feff80eb9`, still `PASS` with 0 errors and 0
warnings on all four validator planes. The recorded digest is the first of those two, exactly as
`VREC-MOK-011` states of its own.

`evidence_paths` was reflowed to one path per line from the single line the capture wrote, so that a later
diff of this file names the path that moved. **No path was added, removed or reordered.** All 25 are tracked
at `41c20ca`, and all 25 are the whole of `evidence/WO-MOK-013/` as it stood at that commit — the
directory's own `README.md` reconciled its file table against the directory row by row, 25 against 25.
**Nothing that postdates the bound commit is in the set**, and two files that now sit in the tree are
deliberately outside it: this record, and `evidence/WO-MOK-013/identifier-collision.md`, which is written in
the same commit as this record and is therefore not bound by it. That directory now holds 26 files and this
set names 25; the difference is that one, and it is the `VREC-MOK-011` precedent rather than an omission.

**Three of the 25 bound documents are edited in the same commit that introduces this record, and that is
disclosed rather than left to a diff.** `README.md`, `closing-review.md` and `completion-summary.md` each
said the work order was `approved` and that this record did not exist; all three now say what is true. What
changed in them is status and record statements, the twenty-second decision, and the `W-HEX-001` figure
moving from three to four — **no measurement, figure, capture or assessment outcome moved in any of them**,
and `manual-assessment.md` was not edited at all, on `VREC-MOK-011`'s precedent that a record's evidence is
not rewritten to agree with a later act. A reader re-deriving this record's claims should read those three at
`41c20ca`, which `git show 41c20ca:<path>` gives. **The other 22 bound paths are byte-identical to the bound
commit** — every capture, every analysis document, both frames, the oracle and both scripts.

The record is written after the candidate commit it names, so its own commit metadata is not
self-referential.

## What this record claims

`WO-MOK-013` is **`implemented`** and `VER-MOK-013` is **`approved`**. The work order's transition was taken
in the same instruction and the same commit as this record's creation, by the engineering owner, and
`WORKFLOW.md` is explicit that a status change "records authority; it is not a confidence estimate".
**`implemented` is not a verification and does not stand in for this record**, in either direction; the work
order's own *Transition to `implemented`* subsection says the same.

At candidate commit `41c20cada40017b8b50ceb655f92b0ab48ed454e`, **every automated case, property, static
check and comparison in `VER-MOK-013` was executed and passed. One of the contract's two manual assessments
has no author.** `VER-MOK-013` classes both manual assessments as "the primary evidence for their
requirements rather than a confirmation of the automated cases", so this record can carry two of the three
requirements and not the third.

| Gate | Result |
|---|---|
| `cargo test --workspace` | **exit 0. 21 runners, 226 passed, 0 failed, 0 ignored, 0 filtered out** |
| `cargo fmt --all -- --check` | exit 0, 0 diff lines |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, 0 warning or error lines. No `allow` added and no lint suppressed |
| `cargo tree -p Mokiterions` | **one line** — the package alone; dependency and dev-dependency tables empty |
| `cargo tree -p mokiterions-tui` | 111 lines, the path dependency plus `ratatui 0.30.2` and its tree, unchanged |
| `python scripts/validate_engineering_artifacts.py` | PASS — **109 artifacts, 0 errors, 0 warnings** across all four planes |
| `bash scripts/check_engineering_harness.sh` | PASS — 109 artifacts, **374 relations**, 0 errors |
| `python scripts/inspect_engineering_artifacts.py` | 0 errors, **11 warnings, 12 informational** — no warning is new; `validation.txt` attributes all eleven to three trees this implementation did not touch |

Zero ignored and zero filtered out is the part worth stating: a suite can be made to pass by not running,
and those two counts are what would show it.

**The two harness rows describe the bound commit's tree with this record added to it, which is not quite the
tree of the commit that introduces this record.** That commit also moves `WO-MOK-013` from `approved` to
`implemented`, and that transition is visible to the inspector: *Active work* goes from one item to none,
`W-HEX-001` goes from **3 observations to 4**, and the warning count from **11 to 12**. `W-HEX-001` warns
against every implemented work order that retains an evidence directory, because evidence discovery keys on
file names — it is a naming artefact of the harness and not a finding about this work, and
`evidence/WO-MOK-013/validation.txt` attributes all of them. The validator stays PASS at 109 artifacts with
0 errors and 0 warnings on all four planes either way, and no other figure in the table moves. The row is
left as measured at the bound commit, in the form `VREC-MOK-011` uses, and the difference is disclosed here
rather than absorbed into it. On the transitioned tree, `harnessctl preflight --phase review --work-order
WO-MOK-013` reports **PASS** and names the work order as `implemented`, `harnessctl validate` reports the
same 109 artifacts with 0 errors and 0 warnings, and `harnessctl doctor` reports PASS on every managed file —
no harness-managed file was edited by this work.

**The three requirements, one at a time.**

| Requirement | Status at this commit | What carries it |
|---|---|---|
| `REQ-MOK-047` — the gauge resolves the value it presents | **Verified** | Manual assessment 1, **SATISFIED** on 2026-08-20 by the repository owner as product owner at Windows Terminal on Windows 11 — *"the bar carried it"* — which is this requirement's primary evidence, plus the two internal-tier cases and the sweep in `gauge-resolution.txt` |
| `REQ-MOK-048` — the controls are findable | **Not verified** | Manual assessment 2 is **outstanding with no author**. The automated case is a necessary condition and `VER-MOK-013` says so in those words |
| `REQ-MOK-049` — the hidden-pane notice is actionable | **Verified** | The automated cases alone. `VER-MOK-013` never gated this requirement on a manual assessment and gives its reason |

**Every case in the contract's coverage matrix is discharged, and the mapping is not restated here.**
`evidence/WO-MOK-013/test-census.md` maps all fifteen automated cases, all five acceptance scenarios and all
four properties to the test that discharges each, names the one case discharged by a pre-existing test
rather than by an arrival, and names the three cases discharged by a single measurement with the reason.
It also reconciles the test census arrival by arrival — 212 at `ff3a155` to 226 here, with the one rename
recorded under both names — and validates the static census target by target against the executed run.

**The non-perturbation property is not an automated case, and the contract says so.** It is carried by three
retained captures rather than an assertion: `non-perturbation.txt` shows 7,534 records identical in order
between an observed and an unobserved run with no first difference; `baseline-comparison.txt` shows the
engine binary at `ff3a155`, built in a detached worktree of that commit, producing the same 7,535 lines as
the binary here; `engine-untouched.txt` shows `git diff --stat origin/master -- mokiterions-core/` empty and
establishes `origin/master` as an ancestor of this branch, so the two baselines are one commit. The two raw
streams are retained whole, so the comparison is reproducible with `diff`.

**The oracle asserts nothing.** `wo013-oracle.rs` was placed in the tree, run once, and removed; the source
is retained here and is not in the tree. Every figure it produced is read out of a rendered buffer, and
nothing in it knows `bar_width` or the layout's overhead constant — which is the independence
`VER-MOK-013` requires and which the `WO-MOK-012` oracle did not have.

**The figures in the packet were measured at `f82cd3d` and legitimately describe `41c20ca`.**
`git diff --stat f82cd3d 41c20ca -- . ':(exclude)docs'` is **empty**: the commit between them changes
documentation only, so no source, test, interface or rendered-buffer figure in `evidence/WO-MOK-013/` moves
between the two. The harness figures in the table above were re-measured at `41c20ca` rather than carried.

## What this record does not claim

1. **`REQ-MOK-048` is not verified, and no status on this record can make it so.** Manual assessment 2 of
   `VER-MOK-013` — show one frame to a person who has not read `SPEC-MOK-003` rule 7 and ask how they would
   find out what the keys do — **has no author**, and `VER-MOK-013`'s *Evidence retention* requires an
   author, a date, a role and a terminal for each. **There is no admissible assessor among the people this
   chain has available**: the contract's assessor must not have read rule 7, and the repository owner has
   read it and ratified six amendments to that document on 2026-08-20, two of which fix the hint's content
   and the abbreviation ladder's order of loss. On 2026-08-20 the assurance owner decided the **route** —
   find an admissible assessor — declining both the option of amending the contract and the option of
   closing the row by decision, on the ground that the chosen route is the only one that verifies the
   requirement. **A route is not an outcome.** The agent's half of that decision is delivered:
   `discoverability-frame.txt`, `discoverability-frame-floor.txt` and `discoverability-assessment.md` are
   the two frames and the administrator's packet. The person is not, and no agent can supply one.
   `manual-assessment.md` still reads **OUTSTANDING** and was not edited to agree with anything here.
2. **Four of this chain's identifiers are claimed by another pushed branch, including this record's own, and
   this record does not resolve that.** `origin/governance/adr-mok-006-third-party-crates`, tip `753a8b9`
   of 2026-08-20T17:43:09+02:00, based on the same `ff3a155` and not merged into `master`, holds a
   different `WO-MOK-013` (`implemented`, an engine dependency-set comparison), a different `VER-MOK-013`,
   a different `REQ-MOK-047` (`approved`, "Resolve each package's dependency set equal to its
   declaration…" against this chain's "Render a survival gauge at a width that resolves the value it
   presents"), and a `VREC-MOK-013` of its own — `ready`, bound to `65ac88b`, captured at
   `2026-08-20T15:29:34Z`, **nine minutes before this one**. `REQ-MOK-048` and `REQ-MOK-049` are free on all
   27 remote refs.

   > **Later fact.** That ref moved after this record was captured, twice: the tip was `f40b711` and their
   > record was bound to candidate `d33ecb8`, captured `2026-08-20T12:37:27Z`, when the figures above were
   > first taken. Re-measured at `753a8b9` before this branch was published, the collision is unchanged —
   > the same four identifiers, the same statuses, `REQ-MOK-048` and `REQ-MOK-049` still free, the same
   > eight merge conflicts and still none against `master` — and only their candidate, their capture
   > timestamp and their pack's file count moved. The gap between the two captures closed from three hours
   > to nine minutes, which sharpens the point rather than softening it. **This record's own claims and
   > figures do not move**, and `evidence/WO-MOK-013/identifier-collision.md` carries the re-derivation. **Both sides are owner-approved and both sides are pushed**, which is worse than the
   `WO-MOK-012` case where one side was still `draft`. Decision 3 of this chain leaves the conflict to
   whichever branch merges second and stop condition 8 does not fire during implementation, so this blocks
   no authorized act — but it means **the name `VREC-MOK-013` does not identify one record across this
   repository's refs**, and a reader resolving the name must resolve the branch too.
   `evidence/WO-MOK-013/identifier-collision.md` measures it and takes no decision.
3. **This record binds a branch commit and says nothing about `master`.** `41c20ca` is on
   `assessment/wo-mok-005-remediation`, nine commits ahead of `origin/master` at `ff3a155`, and two commits
   ahead of the branch's own pushed tip `a339902` — so at the moment of capture the bound commit is
   **local**. No merge has happened, and a record bound to a merge commit would be a **new record, not an
   edit of this one**. `WO-MOK-013`'s *Out of scope* puts a push, a pull request, a tag and a release outside
   the work order, and the pack's *What is not here* bullets are scoped to what the work order authorized.
   **The owner authorized publication of this branch separately, on 2026-08-20, and that act is theirs rather
   than part of this work order's discharge** — so the bound commit reaches `origin` while nothing here is
   tagged, released or deployed, and neither publication nor a pull request is a verification of it.
4. **Every automated result bound here is a claim about text or about an in-memory character buffer.** The
   fifteen cases and four properties read `TestBackend` buffers; no case was verified by looking at a
   terminal. Exactly one measurement in this packet was taken on a real terminal, and it is manual
   assessment 1.
5. **Manual assessment 1 was taken with an adverse fact disclosed first, and that disclosure stands.**
   `REQ-MOK-047` guarantees only that a **ten**-point change moves the fill. The subject the assessment was
   taken on, M01, falls in **five**-point steps through its health window, which at the reference
   viewport's 13 cells is 0.65 of a cell, so roughly every other step of that window draws the bar its
   predecessor drew. The satiety window — 61 to 1, one point per tick over ticks 200 to 260 — is the one
   that exercises the guarantee cleanly, and it was named as the harder and the easier case respectively
   before the owner judged. The requirement is verified as stated; a stronger property was never claimed.
6. **The gauge's resolution is finite and this record does not claim otherwise.** 13 cells give 14
   renderable states over 101 values, up from 2 cells and 3 states, and a ten-point step moves the fill at
   **91 of 91** values where it moved at 11. Two values seven apart can still draw the same bar.
   `VER-MOK-013`'s residual uncertainty records that a person's terminal is not the buffer.
7. **Decision 1's trade is recorded, not verified away.** Holding the log at six rows keeps
   `REQ-MOK-020`'s twelve roster entries and costs the log four recent events where it carried eight —
   `log-height.md` measures both forms out of rendered buffers. The product owner chose it, stood by it on
   the corrected figure, and stood by it a third time after the live pass. That it is the right trade is a
   product judgement recorded in `closing-review.md`, not a verified property.
8. **Nine findings are reported rather than fixed**, in item 8 of `completion-summary.md`, and none is
   remediated by this record. Among them the announcement ladder's implemented three rungs against the
   four the amended rule 5 admits, and the floor header at exactly 34 columns in a 34-column terminal with
   zero margin.
9. **Nothing outside this work order's scope is touched or claimed.** The eight `W-HEX-003` reassessments,
   the three `W-HEX-001` observations, `WO-MOK-008`'s draft disposition, `VREC-MOK-005`'s staleness, manual
   assessment 7 of `VER-MOK-005` and `ROADMAP.md`'s Phase 2 claim are all still open. `validation.txt`
   attributes the warnings; it resolves none.
10. **No release.** `VER-MOK-013` is a verification contract and not a release gate, and a verified record
    would be an input to a release decision rather than one. No release record binds this work.

## What must happen before this record can be verified

Each item names the role that owes it. Nothing in the list is waiting on code, and every automated case
passes at the bound commit.

1. **The assurance decision itself — assurance owner.** Reviewing the 25 retained paths and either
   transitioning this record to `verified` or withholding it. The harness's `decision_required` queue names
   exactly this and nothing else.
2. **Manual assessment 2 — product owner, administered by anyone admissible.** `VER-MOK-013` is **not
   satisfied** while it stands, on this one count. Transitioning this record to `verified` as it stands
   would record that the assurance owner accepts the evidence with `REQ-MOK-048` unverified and one of the
   contract's two manual assessments unperformed — which is what the contract itself calls being
   unsatisfied. That is the shape `VREC-MOK-011` was accepted in, and it is stated here so that an
   acceptance cannot be read as wider than what was put in front of the owner.
3. **The identifier collision — engineering owner, at the merge.** By decision 3 the second branch to reach
   `master` renumbers. Whichever side that is, the renumbering touches this record's `id`, its filename and
   its `verifies_work_order` relation.
4. **A record bound to the merge commit, if and when this chain merges — engineering owner then assurance
   owner.** This record's binding ends at `41c20ca`. Verifying it verifies that commit and nothing
   downstream of it.

## What preparing this record did not do

It approved nothing, verified nothing, merged nothing, tagged nothing, released nothing and published
nothing. It performed no manual assessment, resolved no identifier collision, remediated no reported
finding and re-measured no figure in `evidence/WO-MOK-013/`. It changed one thing in the repository beyond
its own existence: the harness now reports one item under *Decision required* where it reported none.
