# `WO-MOK-020` completion report

In the six-point format the work order's *Completion report format* section fixes.

The work order is left at **`approved`**. Moving it to `implemented` asserts the completed change **and**
the retained evidence as an accountable judgement, and that is the engineering owner's act rather than the
implementation agent's; `WO-MOK-018`'s report and `WO-MOK-013` before it are the precedent. Nothing here
is a verification record, and this work order's *Lifecycle* states that neither the `verified` nor the
`released` transition is its to make.

## 1. What was implemented, in the two requirements' terms

**`REQ-MOK-061` — a profile per Mokiterion.** The observer retains one record per initialized Mokiterion
and accumulates it inside the single advance path, from the decision records and events of the tick being
ingested. The inspector's selected-subject pane presents, beneath the current-tick decision record
`REQ-MOK-021` already required, fifteen figures: one applied count for each of the eleven action kinds
`SPEC-MOK-001` rule 21 closes the contract at, then the rejected-proposal, territory-crossing and
fatal-strike totals, then the decision-opportunity count that is the denominator of the other fourteen.
A total is frozen at death and never removed, so a dead subject's pane is the profile it ended with.

**`REQ-MOK-062` — a population total where nothing is selected.** The no-selection state kept its two
lines — that nothing is selected, and that `Tab` selects a Mokiterion in roster order — above every
figure, and gained the population's sums of the same fifteen figures, then the engine's own tick, living,
initialized and death counts, with the deaths split into those attributed to a strike and the remainder,
which is given no cause. The sums include the dead, so no population total can fall.

### Decisions taken inside the envelope, named

The work order's decision envelope leaves the agent the shape of the retained state, the labels' text, the
layout, the test names and the seeds among the declared set. Each of these was the agent's:

1. **The counts per kind are an array indexed by the action kind's discriminant**, not eleven named
   fields. `ActionKind::ALL` is then the single list both the accumulator and the pane read, which is what
   makes P3's exhaustiveness check possible: a twelfth kind added to the contract fails a test rather than
   being silently unpresented.
2. **The applied action is counted, not the proposal.** Rule 10.3's proposal is what was asked for; the
   applied action is what the world did. The rejected total carries the difference, and the accounting
   identity — the eleven applied totals plus the rejections equal the opportunities — is asserted, so the
   choice is checkable rather than a matter of taste.
3. **The labels are the engine's own verb text**, and the four derived labels are `rejected`, `crossings`,
   `killed` and `decisions`. No label names a rate, a ratio or a per-tick figure, because none exists.
4. **The population total is summed on demand from the retained records**, not retained as a second
   aggregate. One source of truth, and P2's conservation property then holds by construction rather than by
   two accumulators agreeing.
5. **Nothing is recomputed from the retained event buffer.** That buffer drops its oldest record at
   `EVENT_CAPACITY`; a total read back from it would begin understating part-way through a long run while
   still presenting as a figure. This is the design decision the O8 capture exists to test.
6. **`u64` with `saturating_add`,** with the bound argued from `Config::tick_limit` — a Mokiterion
   contributes at most one to each figure per completed tick — rather than from saturation. Saturation is
   the discipline at a limit that is unreachable, not the mechanism the bound rests on.
7. **Two columns, filled column-major,** so the eleven verbs read down the first column and into the
   second in the contract's own order. The widest line this produces is 40 columns of an interior of 42;
   see point 3.
8. **A subject with no record presents no figure at all**, rather than fifteen zeros. Fifteen zeros for a
   subject that was never measured is the invented value rule 10.7 forbids and is indistinguishable on the
   pane from a measured zero. The state is unreachable through a run, since the engine names every
   Mokiterion before tick 1.
9. **Test placement follows `SPEC-MOK-004` rule 8's access rule**: an oracle that needs the retained
   record goes to the internal tier, one that can drive the observer through its public interface goes to
   the public tier. No item was widened to be reached by a test.

## 2. Every specification amendment, by rule and clause

### `SPEC-MOK-003` — seven additions

| # | Provision | What it now says |
|---|---|---|
| 1 | rule 10, presented-value list | the selected subject's fifteen cumulative figures beneath the decision record, every derived form prohibited, and the ground on which a presented zero is a measurement |
| 2 | rule 10 clause 5 | the no-selection state: the statement and the selecting control retained above every figure, then the population sums, then the engine's own counts with the death split |
| 3 | rule 10 item 7 | `kills` and `combats` leave the list of values the engine does not compute, by the procedure the item's own 2026-08-19 amendment established; age, remembered locations, model latency and per-agent entropy stay, and the item gains the record-set test that tells a measured zero from a zero-filled field |
| 4 | rule 10 clause 8 (new) | before tick 1 the pane states that no tick has completed and presents no figure, in both selection states |
| 5 | rule 10 clause 9 (new) | when no living Mokiterion remains the observer clears the selection, so a run ending in extinction presents the population's completed totals with no operator act |
| 6 | rule 11 | the new content mapped to `REQ-MOK-061` and `REQ-MOK-062`, identifiers only, no row added to the event-type table |
| 7 | *State model* and *Performance and capacity* | the `profiles` field with its domain and initial value; and the bound on it — one record per initialized Mokiterion, fixed by the population — with the prohibition on recomputing a total from the retained event buffer |

### `SPEC-MOK-003` — the two corrections, reported separately

These are not additions. Each fixes a statement that was untrue when it was approved.

1. **The *State model* table declared ten fields where the observer holds twenty-four.** It was incomplete
   rather than wrong, and the material omission was three fields of *derived retention* the observer
   already performed — `names`, `latest_survival` and `deaths`. A reader checking whether the observer
   retains anything derived would have concluded from that table that it does not, which is exactly the
   question `REQ-MOK-061` raises. The remaining bookkeeping fields are declared with them, because a table
   that omits a field cannot be read as closed. No obligation moves and no code changes.
2. **Rule 4 said "the observer holds no name table and no identifier-to-name derivation".** The second
   clause is true and load-bearing. The first was not true of the implementation on the day it was
   approved: the observer holds a map from identifier to the name the engine reported. The sentence's
   subject was derivation, and it is corrected in those terms — the observer derives no name and retains
   the engine's — rather than by changing an implementation that does the right thing. This is a strict
   narrowing of an overstated prohibition, not the relaxation of a met one; `REQ-MOK-041` is unaffected,
   because no presented name is one the engine did not report.

**What neither amendment touches.** Rule 10 item 7's 2026-08-20 ground for keeping the suffered-attack
record and the count of attacks suffered off this pane is neither read nor reopened. `ARCH-MOK-002` is
unamended, on the triggers its own amendment record declares. No engine rule, event, stream byte or public
item moves, and rule 12 is unaffected.

### `SPEC-MOK-004` — one amendment row, figures only

Rules 6, 9, 10 and 11 re-measured at the candidate. No rule's substance changes and no obligation on any
test or item changes. The figures are point 3.

**Both rows are written `OUTSTANDING`.** A work order's approval authorizes an amendment to be written; it
is not the approval of the amendment's text, and this work order's decision envelope withholds "the
substance of any specification amendment" by name. The precedent for writing the row outstanding rather
than as approved is `SPEC-MOK-003`'s two 2026-08-18 rows, which stood outstanding until the owner
ratified them.

## 3. The measured figures

### `SPEC-MOK-004`, base `ccb0584` against the candidate

| Rule | Figure | Base | Candidate |
|---|---|---|---|
| 6 | observer public items / `pub` lines / public fields | 94 / 119 / 25 | **unchanged**, re-measured |
| 9 | `tests/render.rs` | 22 | 29 |
| 9 | `tests/verification.rs` | 22 | 29 |
| 9 | public-tier total | 103 | 117 |
| 10 | `src/state.rs` internal tests | 5 | 21 |
| 10 | internal-tier total | 42 | 58 |
| 10 | `src/render.rs` private items | 49 | 55 (34 functions, 21 constants) |
| 11 | observer total | 145 | 175 (117 + 58) |
| 11 | engine total | 156 recorded | **157**, and the arrival is not this work order's |
| 11 | workspace total | 302 | 332 |

Rule 6 is unchanged because everything added is `pub(crate)` or private: the `Profile` record, the
`ActionKind` enumeration and its `label`, the three `Observer` accessors, `Profile`'s counters and
`src/render.rs`'s six private declarations. The **Growth** clause is therefore not invoked and no item's
visibility was widened.

Three cross-checks, all in `10-spec-mok-004-measured.txt`: 117 + 58 = 175; 175 + 157 = 332; and
332 − 302 = 30, which is the number of tests added — 16 internal-tier in `src/state.rs`, 7 in
`tests/render.rs`, 7 in `tests/verification.rs`. The measurement script is retained and the head comment
states each rule's counting definition, including why the public count is anchored at `^\s*pub` and the
private count at column zero: a function-local `const` is not a module item, and counting it as one is
what made a first measurement read 56 where the specification records 55.

### The widest line per pane state, against the interior width

Interior 42 columns at every viewport measured — the reference 160×48, the presence threshold 140×48, and
140×44. Identical at all three, which is what the two-column layout being width-independent means:

| Pane state | Widest line |
|---|---|
| nothing selected, before tick 1 | 40 of 42 |
| selected, before tick 1 | 35 of 42 |
| selected, mid-run | 33 of 42 |
| nothing selected, mid-run | 40 of 42 |

Thirteen captures are retained — twelve states plus the dead-selection pane — with each row bounded by a
bar so trailing blanks are unambiguous. The extinction pane is a fourteenth, at tick 119 with `living 0`,
`initialized 12  deaths 12` and `by a strike 0  unattributed 12`, reached with nothing pressed.

### The independent-count comparison, per seed

`11-independent-count.txt` carries, for each of the five declared seeds, the accumulated table, the
independently counted table and their signed difference: 12 subjects a seed, 60 subject rows, and **60
all-zero difference rows** across all fifteen columns. The comparison in the test is made after *every*
completed tick, which subsumes the matrix's "the final tick and three intermediate ticks" — 200
comparisons a seed rather than four.

Thirteen of the fifteen columns are non-zero. The two that are zero are the engine's own behaviour and are
stated as such in that file: no declared source ever waits, and a source confined to the valid action list
has no proposal rejected. The rejection counter is not left uncovered — the accounting identity holds it,
and it holds on every seed with rejections at zero.

The same run at 6,600 ticks, in `14-long-run-truncation.txt`: **100,000 retained events of 100,000
capacity, `truncated true`**, every total still equal to the independent count, and retained state still
12 records — 120 bytes a record plus 36 bytes of identifiers, at least 1,476 bytes. That is P4's bound
measured rather than argued.

## 4. Command results for every constraint

| Constraint | Command | Result |
|---|---|---|
| 1 | `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, from a cleaned tree | `exit=0`, no warning — `02-lint-candidate.txt` |
| 2 | `cargo test --workspace --locked` | **332 passed, 0 failed, 0 ignored** across 19 targets, `exit=0` — `03-test-candidate.txt` |
| 2 | the same at `ccb0584` | **302 passed, 0 failed** — `05-test-base.txt`. The platform's pre-existing failure profile for this repository is **empty**: nothing was tolerated, and 332 is measured against a green base |
| 3 | `cargo fmt --all -- --check` | `exit=0` — `01-format-candidate.txt`. The toolchain is the pinned one, `rust-toolchain.toml`, cargo and rustc 1.97.1 |
| 4 | `cargo tree -p Mokiterions -e normal --locked --offline` | one crate, the engine package itself |
| 4 | `cargo tree -p mokiterions-tui -e normal --locked --offline` | 59 unique crates: the observer, the engine and the 57-crate `ratatui` graph — rule 14's recorded outcome unmoved |
| 4 | `git status --short` on the two package manifests, the workspace manifest and `Cargo.lock` | no output: none is touched — `18-dependency-sets-unmoved.txt` |
| 5 | `grep -n 'pub fn .*&mut self' mokiterions-core/src/simulation.rs` | exactly two lines, `pub fn run` at 2004 and `pub fn advance_tick` at 2101 |
| 6 | O19.1–O19.4 and O20.1, five tests | all pass in `03-test-candidate.txt`: the engine declares no dependency on the observer, no profile identifier appears under `mokiterions-core/`, the engine's mutating surface is unchanged, no total reaches an `Observation` or a `DecisionSource`, and no total is a float |
| 7 | `sha256sum -c MANIFEST.sha256`, and `git ls-files --eol` on the packet | every file `OK`; the packet reads `i/lf w/lf attr/-text`, so the bytes the manifest hashes are the bytes committed |
| 8 | `git status --short` before the commit | four sources and exactly the two specifications this work order names; no other formal artifact edited |

Two further results, not constraints but obligations: `se_harness validate` is **PASS**, 152 artifacts,
0 errors, 0 warnings; and `se_harness preflight --work-order WO-MOK-020 --phase review` is **PASS**. Both
from the pinned 0.4.0 environment, in `19-harness-validate-and-preflight.txt`.

## 5. The evidence packet

`docs/engineering/simulation/evidence/WO-MOK-020/` — **23 files**, this report included.
`MANIFEST.sha256` covers the other 22 in `sha256sum` format and verifies with `sha256sum -c` from that
directory. Its own digest cannot be stated inside a file it hashes; it is quoted in the commit message of
the commit that adds the packet, which is the same commit as the code and the amendments. That is
deliberate: a verification record binds a commit, and a packet committed after the code is untracked at
the code's own commit.

`README.md` indexes every file with its recompute command, maps each of `VER-MOK-017`'s eight retention
bullets to the files that answer it, and states the four bullets met by substitution.

## 6. Not done, met by substitution, found and deferred

### Not done, and whose it is

| What | Whose act | Artifact that carries it |
|---|---|---|
| All **seven** manual assessments | product owner (1, 2), technical owner (3, 4, 7), assurance owner (5, 6) | `VER-MOK-017` *Manual assessments* |
| Ratifying the two amendment rows | technical owner | `SPEC-MOK-003`, `SPEC-MOK-004` |
| Moving the work order to `implemented` | engineering owner | `WO-MOK-020` |
| The commit-bound verification record | assurance owner | a new `VREC` against `VER-MOK-017` |

Assessment 6 — recomputing at least one amended `SPEC-MOK-004` figure independently of the retained
command output — is deliberately **not** answered by `10-spec-mok-004-measured.txt`, which is the retained
command output it must be independent of.

### Met by substitution, stated on the bullet

1. **The export captures are retained once, and by reconstruction rather than as a third copy.** Both
   trees produce the same bytes, and those bytes are `07-engine-stream-seed42-candidate.txt` with the
   retention footer in place of the engine's summary line. `17-export-unmoved.txt` carries both digests,
   the one-line diff and the two commands that rebuild the exact file, which were run.
2. **The engine's streams are retained in full for one seed of five**, with all ten digests and the
   recompute command for the other eight.
3. **No interactive terminal run is retained**, because none is available in this environment. The pane
   evidence is rendered-buffer captures. What that costs is that no human eye has seen the pane at a real
   terminal, which is what manual assessments 1, 2 and 7 are for.
4. **The per-tick cost is a series and a bound, not a pair.** See below.

### Found, and deferred with the artifact that would carry it

1. **The default release profile measures 266.3 µs/tick at the candidate against 153.0 at the base**, and
   that difference is real for anyone running that build. It is **not** the accumulation: disabling every
   accumulation path leaves it at 262.6, restoring only the base's `src/render.rs` — a file the measured
   path never calls, since the driver draws no frame — recovers it to 159.6, and at
   `codegen-units = 1` the candidate is 243.6 against the base's 250.1, the candidate faster, inside
   either configuration's spread. The cause is codegen-unit partitioning: about 130 added lines
   re-partition the crate and the hot path loses an inlining decision it happened to win. The
   accumulation's own cost is therefore reported as an **upper bound of about 25 µs/tick**, because the
   instrument cannot separate it from noise. Whoever owns the observer's frame budget has about 113 µs a
   tick to account for and the place to look is the partitioning; no artifact carries a per-tick budget
   today, so this is raised for the engineering owner rather than filed against one.
2. **The rule 10.6 dead-selection fixture was retargeted from `--policy baseline` to `--policy
   reference`, and it costs coverage.** `baseline` starves the whole population on one tick, so its first
   death *is* extinction; new clause 9 clears the selection at extinction, and the state that fixture
   exists for stopped existing under that policy when the amendment landed. Under `reference` the first
   death leaves eleven living and every assertion holds as written, but rule 10.6's retention is now
   exercised on a mid-run death and no longer on a terminal one. The reasoning is in the fixture's own doc
   comment. `VER-MOK-005` is the contract that carries rule 10.6, and it is not edited here.
3. **The engine's rule 11 figure is 157, not 156, and the arrival is `WO-MOK-017`'s** — commit `26ae6ba`,
   implementing `REQ-MOK-060`. It was owed from that work order's closure by `SPEC-MOK-004` rule 11's own
   closing sentence and was not made then, so the owner is shown it here for the first time. It is stated
   with its origin rather than absorbed into this work order's thirty. `SPEC-MOK-004` carries it.
4. **This work order's `SPEC-MOK-004` row will conflict with the one on the other branch.** The owner has
   already deferred PR #44's `SPEC-MOK-004` figure correction and its `SPEC-MOK-003` amendment-row
   conflict to that side. Both amendment tables now gain a row dated 2026-08-22 from two branches, and
   whichever merges second resolves by hand. Neither row is a figure this side may edit on the other's
   behalf.
5. **`docs/ROADMAP.md` is not updated.** Its entry records this chain as defined and unapproved, which was
   true when it was written. It is not corrected here because the work order is still `approved`:
   correcting the roadmap before the status transition would record a state the artifact graph does not
   yet hold. It becomes accurate to update it in the same act that moves the work order.
6. **Four items disclosed earlier in this chain remain owed and are not this work order's**: Phase 5 does
   not state `REQ-MOK-050`'s pull-request gate in its own section; `WO-MOK-018` has no phase account;
   finding 7's `VER-MOK-016` amendment row is the assurance owner's; and the non-artifact CI lane is
   missing from `.github/`.

### What this commit does not establish

That the change is verified. Every figure here is measured against `ccb0584`, on a branch commit that is
not an ancestor of `master`. On a merged tree the gates, the census, the counts, the frames and the
per-tick series need re-running rather than carrying over, and a record bound to the merge is a new record
rather than an edit of one bound to this commit.
