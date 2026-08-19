# WO-MOK-010 evidence packet

`VER-MOK-010`'s evidence-retention list has 15 bullets. Each is below with the file that discharges it, so retention
completeness is checkable without reading the packet.

| Retention bullet | File |
|---|---|
| The pre-change baseline capture of the declared matrix, captured before any code change, with the commit it was taken at recorded | `baseline/` — `capture.sh`, `pre-manifest.txt` (all 42 cells by digest), `full/` (the 11 streams retained whole), `exit-codes.txt`; the commit is in `baseline/COMMIT.txt` and is **60fda9f**. **11 of the declared 40 cells were captured before the first line of code changed; the other 31 were captured afterwards from a clean worktree at the same commit — see the disclosure below and `baseline/recapture-check.txt`** |
| The post-change capture of the same matrix, and the projected comparison result for every combination | `post/additivity.txt` — 42 frozen-source cells all projected-equal to the pre-change stream with the projection a no-op on it, 20 new-source cells byte-identical across two processes, and all 20 differing from the reference source; `post/post-manifest.txt`, `post/full/`, `post/exit-codes.txt` |
| The full text of the projection, and the result of applying it to the pre-change stream alone | `baseline/projection.py` (three anchored patterns, quoted in full in `manual-assessment.md` §6); the no-op result on the pre-change stream is in `baseline/recapture-check.txt` |
| The twelve derived trait values per declared seed, as the record of which populations the floor was measured on | `measurements/traits.txt` |
| Per-seed 1,000-tick survivor counts and consumption totals under the new source | `measurements/viability.txt` |
| Per-seed final resource count and class distribution per territory under the new source at tick 1,000, and whether either territory reached zero | `measurements/viability.txt` — the two territory columns and their high-class shares; neither territory reached zero on any seed under either source |
| The tick-10,000 result under the new source at the default density, alongside the reference source's recorded extinction at tick 9,154 | `measurements/long-horizon.txt` — the recorded control reproduces to the tick at seed 123 |
| The measured oscillation rate under the new source per declared seed, against rule 5's 10.6% and the 12.2% unbiased-walk rate | `measurements/oscillation.txt` |
| The recorded divergence instances behind the real-run divergence check, each naming the tick, the two Mokiterions, their traits and their differing proposals | `measurements/divergence.txt` |
| Rendered roster buffers at each declared viewport, as text, with the four gauges' cell positions stated | `observer/roster-frames.txt` (the analysis and the gauge columns), `observer/frame-probe.rs` (the probe that produced the captures) |
| The enumerated situation set used by oracle 3, and its size | `measurements/equivalence.txt` — 2,808 situations, enumerated rather than sampled |
| The workspace test census before and after, reconciled name by name | `test-census.txt` |
| `cargo fmt`, `cargo clippy`, `cargo test` and `cargo tree -p Mokiterions` output | `static-checks.txt`; the capture is reproducible with `analysis/capture-static.sh` |
| The seven manual assessments above, each with its accountable role and date | `manual-assessment.md` — **all seven recorded**, five of them in the closing review of 2026-08-19 that `closing-review.md` records; see below |
| The amendment-approval check of oracle 5 | `amendment-approvals.md` |

**Nine files are not on the retention list.** `requirement-to-test-mapping.md` maps every one of `VER-MOK-010`'s 45
matrix rows to the test or file that discharges it, in the contract's own order, and names the one row that is *not*
satisfied in place rather than in a footnote; both prior work orders retained the same mapping and
`measurements/traits.txt` cites it. `completion-summary.md` is the work order's own required closing report.
`escalation.md` records `WO-MOK-010` stop condition 6 firing and the owner's decision on it, because a stop condition
that fired and was resolved is not something a reader should have to reconstruct from an amendment row.
`closing-review.md` records the twelve decisions the repository owner took on 2026-08-19 to close this work order's
outstanding governance — the seven manual assessments, the four ratifications, and the decision on the `VREC-MOK-005`
layer — with the role each was taken in and what the ratifications do not reach; it is the record of the acts, and what
each was decided on stays in `manual-assessment.md` and `amendment-approvals.md` §3.
`negative-control/` holds the controls on oracles 2 and 3 — each designed failure injected and confirmed to fail.
`interface-and-purity.txt` is the public-interface census and the `fear` writer/reader count.
`measurements/proposals.txt` counts what each source actually proposed, and is the check that the trait-aware source
never proposes `wait` and that the engine's own validation rejected none of its proposals — zero `wait` and zero
rejections on all ten runs. `renumbering.md` records a governance act rather than a measurement: this chain was
approved and implemented as `WO-MOK-007`, `master` then created a different `WO-MOK-007` and verified it,
`feature/release-ci` had taken `008` and `009`, and the owner renumbered this one to `010`. **Read it before
hashing anything, and before reading any `.txt` here that says `WO-MOK-007`** — the retained captures were not
edited, so sixteen of them still carry the former name, and one recorded digest no longer reproduces for that
reason. It also records the three captures re-taken against the merged tree, which name it `WO-MOK-010`. This
`README.md` is the map itself.

**Before hashing anything here, note that a `.gitattributes` was added for this directory.** Every manifest below
records the SHA-256 of a captured file so that a reviewer can hash the retained file and get the recorded number, and
with `core.autocrlf = true` and no `.gitattributes` a Windows checkout rewrote every retained stream to CRLF, so every
recorded digest failed against a file whose content was correct. That is not hypothetical: `WO-MOK-006`'s retained
`baseline/engine/full/short_seed42_baseline_trace_on.txt` hashes to `08eadb21…` as checked out, against the
`3424133a…` its own manifest records. One line — `docs/engineering/simulation/evidence/** -text` — disables the
conversion. Checked in a clean worktree at this commit rather than argued from the attribute: both that file and this
packet's own streams hash to their recorded values there. `completion-summary.md` §15 has it in full.

Every `.py`, `.sh` and `.rs` file in the packet is the tooling that produced the `.txt` beside it, retained so that
each figure is reproducible from the recorded command rather than trusted. The one exception is
`observer/frame-probe.rs`, which was written into `mokiterions-tui/tests/`, run once, retained here and then deleted
from the crate — the `WO-MOK-006` precedent for a probe that must not become a permanent test.

## Read these five first

- **`manual-assessment.md`** — the seven judgements no script can make, **all seven recorded**, each with the
  measurement it was decided on, so `VER-MOK-010`'s manual-assessment contract is *satisfied*. Two carry facts that
  point adversely and are named at the top of that file rather than the bottom: assessment 2, where the divergence
  count is within a factor of three of the figure the contract itself names as failure, and assessment 4, where `fear`
  sits at its ceiling on 39% of agent-ticks. Both were decided on those facts rather than around them, and neither was
  resolved by substituting a more favourable measure.
- **`closing-review.md`** — the twelve acts of 2026-08-19, in one place: which role took each decision, that each
  question was put and answered on its own so that no approval covers a second act, what the four ratifications do
  **not** reach, and the obligation attached to the `VREC-MOK-005` override. It transitions nothing.
- **`escalation.md`** — stop condition 6 fired: `REQ-MOK-034`'s survivor floor was missed on three of five declared
  seeds at the `0..=100` trait range first specified. The owner, as technical owner, chose to narrow the range to
  `0..=40` rather than amend the floor. Every survivor figure in this packet is downstream of that decision, which is
  why it is read early rather than found late.
- **`amendment-approvals.md`** — oracle 5. Every provision the owner approved is in both the amendment record and the
  specification text, checked over disjoint text so a record cannot vouch for itself. It also names **seven amendments
  written during implementation beyond the owner's stated list** — one approved as a decision under a stop condition,
  **four ratified by the technical owner on 2026-08-19**, and two that change no provision and needed no ratification —
  checks that each of the four carries that ratification, its date and its role in the specification's own Approval
  column, and checks that every amendment row `master` carried at **7a2b502** survived the merge byte for byte.
- **`observer/roster-frames.txt`** — oracle 4, and the packet's one reachability finding: because the roster pane's
  width and rule 4's collapse threshold are both 47, the drawn roster is *always* two-line at a 45-column interior,
  so `bar = 2` is the only bar width reachable through `render::draw`. The `min(20, …)` cap and the collapsed
  one-line form are carried by three named internal render tests instead.

## The five independent oracles

`VER-MOK-010`'s central claim is that the two frozen decision sources are untouched and the new one behaves as rule 19
specifies. That claim is not carried by one measurement:

1. **The recorded pre-change baseline under a stated projection** — 42 frozen-source cells at **60fda9f**, compared
   against the post-change capture with only the three added fields deleted, and every one byte-identical. "The
   baseline is captured once. A discrepancy is never resolved by recapturing it." The projection is also applied to
   the pre-change stream alone, where it must be a no-op, which is the one way this oracle could be subverted.
2. **The shared entropy stream's position across trait derivation** — trait derivation uses a generator of its own, so
   the shared stream's draw sequence is unmoved and every pre-existing run is bit-identical.
3. **Arithmetic equivalence at the trait's lower bound** — all 2,808 situations enumerated, not sampled: rule 19 at
   `T = 0` is proposal-identical to rule 5.
4. **The in-memory character buffer, cell by cell** — 996 bar rows across the 85 of 157 probed frames that draw a
   roster, rebuilt from rule 4's named parts rather than from the product, matching character for character with every
   gauge at its predicted absolute column. Re-derived on 2026-08-19 against the merged tree; see the note at the end of
   `renumbering.md`.
5. **The governance state of the amended artifacts** — an amendment nobody approved is not a specification.

Each can fail without the others failing, which is why all five are here. Oracles 2 and 3 additionally have negative
controls in `negative-control/`: each was made to fail on purpose before being trusted to pass.

## What none of this establishes

**`fear` has no consumer.** Nothing reads it — no rule, no decision source, one writer and no reader by census. So no
outcome can falsify its constants, and the 39% ceiling residency is an observation rather than a defect. What is
verified is that the attribute is maintained, bounded, perception-driven and reported; whether `+10`/`-5` is the right
pair will only be answerable once something consumes it.

**Individuality is demonstrated at the scale it was measured, not at the scale a reader might assume.** There are 3 to
10 divergent situations per thousand-tick run and **zero** cases of two Mokiterions facing the same situation on the
same tick, so no divergence is ever visible side by side in a single frame. The 54 to 97 waste-accepting eats per run
are the same behavior counted without requiring a coincidence, but substituting that measure for the one
`VER-MOK-010` names was the product owner's call, not the implementation agent's — and on 2026-08-19 that owner declined
to substitute it, recording the assessment on the divergence count with the eats as corroboration. So this paragraph
stands unchanged by the decision, which is why it is still here.

**Equivalence is demonstrated across the declared matrix and the enumerated situation set, not across the input
space.** Oracle 3 is exhaustive over its 2,808 situations because that set is finite; oracle 1 covers 40 cells of a
much larger space.

**The `VREC-MOK-005` gate was overridden, not met.** The amendments from the previous work order remain
**OUTSTANDING** and its seven manual assessments remain unrecorded, by the repository owner's explicit decision of
2026-08-19. `master` has since transitioned that record to `verified`, which does not close the gate: the record's own
text says the transition accepted the automated evidence with all seven assessments outstanding and eleven provisions
across four artifacts awaiting the technical owner. The status moved and the substance did not. The mitigation is that
the two layers stay separable by inspection, and `amendment-approvals.md` checks that claim rather than asserting it:
every amendment row dated before 2026-08-19 is byte-identical to **60fda9f**, and every row `master` carried at
**7a2b502** survived the merge byte for byte. It is a cost carried forward, not a debt paid. **The closing review of
2026-08-19 let the override stand and named the debt: those eleven provisions and seven assessments are to be resolved
by a work order of their own, completing before the next release record.** Naming it is not paying it — nothing in this
packet resolves, approves or assesses any of them, and `WO-MOK-005` is not transitioned here.

**The pre-change baseline is incomplete relative to the declared 40 cells, and the shortfall is disclosed rather than
repaired.** Eleven cells — the ten 1,000-tick default-density frozen-source runs and one 20-tick traced run — were
captured before the first line of code changed and are retained in full. The other 31, the 1.50%-density and
`--trace-actions` variants, were not: they were captured afterwards from a clean git worktree at the same commit. That
is a recapture, and oracle 1 forbids recapturing the baseline *to resolve a discrepancy*. It was not used to resolve
one — no discrepancy existed, and those 31 cells are ones the original capture never covered — but the distinction
rests on the recapture being from the same world, so `baseline/recapture-check.txt` compares the eleven cells the two
captures have in common byte for byte rather than asserting it. All eleven match. **A reader who does not accept that
argument should treat oracle 1 as covering eleven cells, not forty.**

Separately, `baseline/rebuild-check.txt` records that the post-change capture every figure here derives from came from
a binary that then fell behind the engine source three times: `cargo fmt` reformatted it, one test was moved out of the
crate's `#[cfg(test)]` block into the public tier, and one `debug_assert!` invariant was tightened to the bound the
amended trait range states. Each has a reason it cannot matter and none of those reasons is relied on: the tree was
rebuilt from the committed source and the whole matrix captured again, and all 83 shared cells came out byte-identical.

This packet was written before the implementation was committed, and its governance sections were completed after the
closing review of 2026-08-19. `WO-MOK-010` is left at `status = "in_progress"`, and `VREC-MOK-010` is a **`ready`
candidate** bound to the commit that carries this packet — a verification record binds a commit and is created after the
one it names, which is why it is re-captured whenever that commit moves rather than rewritten. Moving it from `ready` to
`verified` is the accountable assurance owner's act and moving this work order to `implemented` is the engineering
owner's; neither has been taken, and nothing here merges, tags or releases anything.
