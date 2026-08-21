# The merge of `master` into the `WO-MOK-017` chain

| Field | Value |
|---|---|
| Merge commit | `ae2e44f2382fe89f2daf78a8e8aca37febb8bd0f` — "Merge master into feature/resource-composition-ceiling for WO-MOK-017" |
| First parent | `51f44cf5def92d36903da43386b80d5caddf9f26` — this branch, the 289-file evidence packet |
| Second parent | `3f47743bf1db9577af39c72be9742381ae77b296` — `master`'s tip, "Merge pull request #38 from mmzen/feature/phase-4a-definition" |
| Merge base | `7f4792a8e4de9bb0e2ce1a53a35bb9f5c5fb10a3` — `master`'s tip when this branch was cut |
| Candidate commit | `26ae6ba648be4eecf6234da15c0beb763b403a0a` — the commit every figure in `../post/` is measured at, and it is **not** rewritten by this merge |
| Branch | `feature/resource-composition-ceiling`, pull request **#39** |
| Date | 2026-08-22 |
| Toolchain | cargo 1.97.1 (c980f4866 2026-06-30); rustc 1.97.1 (8bab26f4f 2026-07-14); clippy 0.1.97 (8bab26f4f6 2026-07-14) |
| Harness | se-harness 0.4.0, from `C:\Users\mathi\harness-venv-040` — the version this repository's workflows pin |

`../README.md` is the account of the packet, measured at the candidate `26ae6ba`, and this directory is
the account of the merge. It is short for a reason it states and bounds in `gates.txt` §1.

**No measurement in `../README.md` is falsified and none is re-written.** Three things were added to it,
all of them pointers to here, because an index that does not index this directory is wrong about its own
packet: a `Merge commit` row in its header table, a `### merge/` section in its index, and a scope on the
*Conventions* bullet that claimed no reader prints `RESULT: FAIL` — true at the candidate, and two do
here. Every figure, verdict and file description above the candidate stands as it was committed at
`51f44cf`.

## This is not a verification record and it takes no decision

Every file here is derived, read-only evidence: a command, its output, and the reasoning needed to read
the output. Two of the outputs are `RESULT: FAIL` and they are retained as they printed. **`VREC-MOK-017`
is where a decision about this tree would be recorded, it binds a commit, it is the assurance owner's act,
and it is not written.** Nothing here approves, verifies, ratifies a figure, closes an obligation or
transitions a status.

## Why there is a merge

`master` moved while this packet was being built. Pull request #38, from
`feature/phase-4a-definition`, merged as `3f47743`, and it carried the whole `WO-MOK-019` chain —
*"Emit a structured record stream to an operator-named sink"*, itself merged into that branch by pull
request #31 as `fac152f`, plus the two governance commits that prepared `VREC-MOK-019` and transitioned
it to `verified`. Its implementation rewrote about two thousand lines of
`mokiterions-core/src/simulation.rs`, the same file in which this work order corrects the non-waste
condition of `SPEC-MOK-001` rule 5.

Pull request #39 therefore stopped being mergeable, and this merge is the resolution. It is a merge and
**not a rebase**, deliberately and irreversibly: rewriting `26ae6ba` would orphan every digest, every
manifest line and every capture-state claim in the 289-file packet, and a `verified` record cannot be
re-pointed at a later commit. The candidate keeps its identity and the merge sits on top of it.

## The two conflicts, and how they were resolved

`git merge` reported two conflicted paths. Both are content conflicts inside a file each side edited, and
neither is a governance document with an approved amendment record on both sides.

| Path | What collided | Resolution |
|---|---|---|
| `mokiterions-core/src/simulation.rs` | `master`'s `Sinks<'_, '_, W>` threading and its per-class counters against this branch's corrected non-waste condition, in the `eat` arm of `apply_action` and in `regenerate_food` | **Both taken.** `master`'s sink parameter, its `emit(sinks, …)` call sites and its three `saturating_add` counters, with this branch's condition, allowances and boundary satieties unchanged inside them. |
| `docs/ROADMAP.md` | Two adjacent Phase 3.1 entries, one per chain | **Both taken**, in date order. No figure, status or owner line is edited. |

**The resolution principle, stated so it can be disagreed with:** where the two sides change *different*
things in the same region, both survive; nothing is dropped to make a region compile. Neither side's text
is preferred over the other's here, because neither answers a question the other answers — `master`
instruments the engine and this branch corrects a world rule.

**The resolution is verified from the other end, not asserted.** `gates.txt` §5 diffs both conflicted
regions block by block, with this packet's own extractors, and the whole of each difference is
`master`'s instrumentation: one parameter and three counters. `gates.txt` §2 then measures the
consequence directly, which is the stronger statement — the merged tree's 120-cell capture is
byte-identical to the candidate's on all 120 cells and all four columns. Identical streams cannot come
from a changed world rule.

## What this merge did not have to re-derive, and what it therefore does not license

**The licence is an output equality and not a structural argument.** `WO-MOK-019`'s
`merge/second/gates.txt` established its licence with two empty diffs — one half of that merge was
byte-identical to `master` and the other to its own chain. **Neither empty diff is available here.** Both
halves of this merge are genuinely merged text, so the licence had to be measured on the streams
themselves:

    git archive ae2e44f | tar -x           the merge commit's own tree, not a working tree
    capture.sh                             120 cells, 5.604s
    compare-to-candidate.py                RESULT: PASS -- 120 of 120 cells identical
                                           on all four columns, 0 differing

That carries every capture-derived figure in `../post/` forward to the merge commit: the **54.1%** worst
composition against `REQ-MOK-060`'s three fifths, the three survivor-floor margins **0**, **0** and **2**,
the 30 byte-identical `baseline` cells, the 90 attributed divergences, and the entropy, health-fall and
dead-neighbour readings. Eleven of the packet's thirteen readers hold at this tree because their inputs
are byte-identical at this tree.

**What it does not license** is in `capture-comparison.txt` §4, and one bullet of it needs stating here:
`capture.sh` never passes `--events-path`, so these 120 cells exercise the default path and say nothing
about `master`'s record sink. `VREC-MOK-019` is where that stream is verified, and this merge does not
re-verify it or claim to.

## The two readers that do not carry forward, and both print `FAIL`

The other two of the thirteen read `mokiterions-core/src/simulation.rs` rather than a capture. `master`
changed that file, so both were re-run and both failed. **The failures are `master`'s arrival, not a
regression in this work order, and they are retained rather than repaired.**

    world-rules-merged.txt    RESULT: FAIL -- 3 check(s) failed
                              61 changed engine blocks outside the condition family;
                              the eat arm and regenerate_food differ
    reads-merged.md           RESULT: FAIL -- 1 check(s) failed
                              write_metrics_territory calls food_counts and is not classified

The 61 blocks are the record stream. The two that bear on a world rule are the two conflicted regions,
diffed in full in `gates.txt` §5. `reads.py`'s failure is its completeness check asking for a
classification a human writes: `write_metrics_territory` takes `&self`, has one caller — `write_metrics`,
the metrics record of `SPEC-MOK-006` rule 7.6 — and writes JSON, so it is on the observation side of
`INT-MOK-010`, the side allowed to read composition. **The oracle itself holds**: each of the four
`DecisionSource::decide` functions still takes only an `Observation`, and `Observation` still carries no
composition field.

Neither reader is edited. Adding a classification to `analysis/reads.py`, or widening
`analysis/world-rules.py`'s block set, would edit a program whose output is retained evidence at
`26ae6ba` — which is what "evidence is re-run rather than edited" forbids. So both are left failing and
disclosed, here and in `gates.txt`.

## Test conservation

    cargo test --locked --workspace       ok. 302 passed; 0 failed; 0 ignored
    census-conservation.py                RESULT: PASS -- 302 names on the merged tree,
                                          0 lost, 1 added and accounted for

The reconciliation runs in both directions, against committed artifacts rather than against figures
quoted from a specification. **268 + 34**: this branch's candidate plus the thirty-four engine tests that
arrive with `master`. **301 + 1**: `master`'s own census, taken by the other chain at `e96648a`, plus this
work order's one test. The first direction would fail if the resolution had lost an observer test, the
second if it had lost an engine test. Neither does.

## The gates

    cargo fmt --all -- --check                   exit 0, no output
    cargo clippy --workspace --all-targets
      --all-features --locked -- -D warnings     exit 0, 60 lines, 0 diagnostics,
                                                 from a cleaned target directory
    cargo test --locked --workspace              ok. 302 passed; 0 failed; 0 ignored
    validate .                                   PASS   147 artifacts | Errors 0 | Warnings 0
    doctor .                                     PASS   81 lines, every one PASS
    preflight . --work-order WO-MOK-017
              --phase review                     PASS   Work order: WO-MOK-017 (in_progress)
    dashboard . --output <outside the checkout>  PASS   147 artifacts | 525 relations
                                                        Errors 0 | Warnings 18
    scripts/check_declared_dependencies.py       exit 0, 8.4a-8.4d pass

**This branch creates no artifact**, so 134 + 13 = 147 is the whole of the arithmetic and there was no
identifier to collide: `WO-MOK-017.md` already existed at the merge base as a draft, and this branch
approved it in place. `gates.txt` §6 measures the count on all four trees.

`preflight` reports `in_progress`, which is what the work order is. Transitioning it is the engineering
owner's act.

**One check on pull request #39 has never run, and this is the disclosure.**
`dependency-declarations.yml` is `pull_request`-only, and no `pull_request` run exists on #39 — the branch
was pushed before the PR was opened, so only the push run exists, and `engineering-harness.yml`'s own
work-order-trailer selection and review preflight are gated `if: github.event_name == 'pull_request'` and
were skipped with it. Two green checks, none of which read the trailer, the preflight or the dependency
graph. The repository's own dependency program was run locally to close the gap by measurement, which is
not a substitute for the run: the push that carries this merge is what fires it.

## Files in this directory

| File | Bytes | What it is |
|---|---|---|
| `README.md` | this file | The account of the merge |
| `COMMIT.txt` | 41 | The merge commit's full hash, alone on its line |
| `gates.txt` | 14,293 | Every command run on the merged tree, its output and its exit code |
| `merge-manifest.txt` | 14,673 | `analysis/manifest.py` over the capture from `git archive ae2e44f` |
| `compare-to-candidate.py` | 9,214 | The reader that compares that manifest to `../post/post-manifest.txt` |
| `capture-comparison.txt` | 4,398 | Its report — the carry-forward licence, and what it does not cover |
| `census-conservation.py` | 7,784 | The reader that reconciles three censuses in both directions |
| `census-conservation.txt` | 2,797 | Its report |
| `test-run.txt` | 26,892 | `cargo test --locked --workspace` in full on the merged tree |
| `test-census.txt` | 26,027 | `analysis/test-census.py` over that run — 302 names |
| `world-rules-merged.txt` | 15,451 | `analysis/world-rules.py` re-run — `RESULT: FAIL`, retained |
| `reads-merged.md` | 9,180 | `analysis/reads.py` re-run — `RESULT: FAIL`, retained |
| `declared-dependencies.txt` | 2,987 | `scripts/check_declared_dependencies.py` in full |
| `SPEC-MOK-004-rule-11.md` | 10,614 | The correction rule 11 obliges, measured and drafted in full, **not applied** |

Byte counts are of the files as committed, this file excepted, since a file cannot state its own length.
Thirteen files, plus this one.

Every text file here is LF and UTF-8, matching the packet's convention and `.gitattributes`' `-text` pin
on this tree. Two of them were captured twice for that reason: a Windows shell redirect of a Python
program's output produces CRLF and cp1252, so `declared-dependencies.txt` was re-taken by running the
repository's program unmodified under `runpy` with `sys.stdout.reconfigure(encoding='utf-8',
newline='\n')` — the same reconfiguration every reader in this packet performs on itself.

## What this merge leaves owed

Everything `../README.md` and `../manual-assessment.md` list remains owed — **3 manual assessments**
prepared and unsigned, and **9 findings**, none of which this merge closes or reopens. Added by this
merge, in the order they should be read:

1. **`SPEC-MOK-004` rule 11's figures, drafted and unapplied.** The rule says a work order that adds a
   test corrects its figures there; this one adds a test and never touched the file — at the candidate as
   well as at the merge, so **the omission is this work order's own and is older than the merge**.
   `SPEC-MOK-004-rule-11.md` measures the correction, reproduces 302 from four tiers and from both
   predecessors, and drafts the paragraph and the amendment-record row verbatim. It is not applied
   because amending an approved specification is the technical owner's act and this work order's approved
   amendment set is the three in `SPEC-MOK-001`. **One ratifying act, or a reasoned decline** — declining
   is coherent and leaves the rule reading 301 against a tree that runs 302, which the record can hold as
   long as it says so. Note that the 301 it moves from is itself an `OUTSTANDING` draft; this draft does
   not restate or reopen it.
2. **The two `RESULT: FAIL` reports need a reading, not a fix.** Both are disclosed above and in
   `gates.txt` §5, and both are `master`'s arrival. Whoever prepares `VREC-MOK-017` decides whether the
   reasoning offered for each is sufficient, or whether `analysis/reads.py` owes a classification for
   `write_metrics_territory` under a work order that is allowed to edit it.
3. **`VREC-MOK-017` at `ae2e44f`**, declaring `evidence/WO-MOK-017/`. `VREC-MOK-019` at its own commit is
   not re-pointed and not reopened — a record binds the commit it was written against, and there is no
   rebinding. If rule 11's correction is ratified, it lands on a commit that is not this tip, so the
   applying commit changes the tree a record would bind; this file takes no view on the order.
4. **The `pull_request` CI run**, which the push of this merge fires and which no run on #39 has yet
   performed.
5. **`WO-MOK-017`'s transition out of `in_progress`**, and the merge of #39, both the owner's acts.

## Authority

Every command in this directory is derived, read-only evidence, and each prints its own boundary.
Validation does not approve. Inspection does not validate by exit status, approve, authorize, verify,
release or remediate. Preflight does not approve artifacts, authorize a diff, verify work, release
software, commit, push, tag, publish or deploy. Nothing here records a decision, ratifies a figure,
closes an obligation or transitions a status. `VREC-MOK-017`, at the merge commit, is where a decision
about this tree is recorded, and it binds a commit.
