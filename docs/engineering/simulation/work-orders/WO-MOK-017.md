+++
id = "WO-MOK-017"
type = "work_order"
title = "Correct the resource composition drift: the numeric waste condition, and both survivor floors re-measured against the corrected world"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-21"
updated = "2026-08-22"

[assurance]
commit_bound_verification = "required"
rationale = "This work changes the proposal logic of the two decision sources that every prior survivor measurement was taken under, so it moves `reference` and `individual` output at every seed and density and invalidates the 60 non-`baseline` cells of the matrix `VER-MOK-016` binds. Three claims it must carry cannot be asserted and can only be checked against captures bound to a commit: that `baseline` is still byte-identical, that every `reference` and `individual` divergence is attributable to this correction and to nothing else, and that no resolution moves the shared entropy stream. It also re-opens `REQ-MOK-014`'s and `REQ-MOK-034`'s measured survivor floors, which no self-assessment may retire, and it decides the fate of an identity two approved artifacts state — `SPEC-MOK-001` rule 19's proposal-identity with rule 5 at tolerance zero, and `REQ-MOK-033`'s reliance on it."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-060"]
specifications = ["SPEC-MOK-001"]
verification = ["VER-MOK-016"]
+++

# Work Order: Correct the resource composition drift

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope below.
Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the completed
change and the retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

### Why this work order exists

`REQ-MOK-060` was approved on 2026-08-20 as part of `WO-MOK-016`'s scope and deferred by the product owner on the same
day. On 2026-08-21 the repository owner, acting as engineering owner, descoped it from that work order and carried it
here, so that `WO-MOK-016` could reach `implemented` without asserting completion over a requirement it did not
implement. `WO-MOK-016`'s *Scope amendment of 2026-08-21* records that act, its two declined alternatives and its four
consequences; this section does not restate them.

**Nothing about the requirement changed in the transfer.** `REQ-MOK-060` keeps `approved` status, keeps
`derives_from = CAP-MOK-010`, and `SPEC-MOK-001` keeps it in `specifies`. Its ceiling of one half, ratified at that value
by the product owner on 2026-08-20 against `60%` and `40%`, stood at the transfer and **was amended to three fifths on
2026-08-21, on the measurement this work order took at approval and against the alternatives its own requirement named**;
*Approval decisions of 2026-08-21* records that act and `REQ-MOK-060`'s amendment record carries it. The surface the
correction may use is fixed by the requirement, restated in `SPEC-MOK-001` rule 5's accumulation paragraph, unchanged by
that amendment, and is not this work order's to widen.

**What this work order inherits is a measurement, not just an obligation.** `WO-MOK-016` took the composition measurement
at both of its commits and retained it, precisely so that this work order's open decisions are taken on a curve rather
than on an argument. `evidence/WO-MOK-016/post/byte-identity.txt` records that the ceiling fails in 14 of the 15 measured
cells at that candidate, and `evidence/WO-MOK-016/post/composition.md` carries rule 18's final summary per seed under all four
sources with the ratio computed per territory and per calorie class. The state the requirement ends — high class standing
at 45 of 61 in a territory by tick 1,000 against a balanced initial third — is measured there and is not re-derived here.

### Approval preconditions

Three things must be decided before this work order is approved, and none of them is an implementation choice. All three
are already reserved to their owners by artifacts in force; this section only names them and says what they are decided
on. **All three were decided on 2026-08-21 and the section that follows this one records the answers**, together with two
further decisions the first answer forced. This section is left as it was written, because what a precondition asked is
part of the record of how it was answered.

1. **Which permitted mechanism is used.** `REQ-MOK-060`'s *Open decisions* reserves this to the technical owner **on
   measurement**: relaxing `SPEC-MOK-001` rule 5's non-waste condition `satiety + restoration <= 100`, raising the
   tolerance floor rule 19's tolerant form `S + R - 100 <= T * R / 100` evaluates against, or both. `WO-MOK-016` declined
   to pre-empt this and recorded why; the measurement it retained is what the decision is taken on.
2. **What becomes of rule 19's `T = 0` identity.** `SPEC-MOK-001` rule 19 states that "this source is proposal-identical
   to rule 5 for every observation at tolerance zero", and the identity holds only because rule 19's first clause is the
   literal inequality `S + R <= 100` that rule 5 states today. The specification already fixes the obligation and it is
   quoted rather than paraphrased: "Relaxing rule 5's condition alone breaks the identity unless this clause is restated
   as rule 5's condition rather than as that literal inequality; introducing a floor breaks it by making tolerance zero
   unreachable in the test. Either outcome is permitted and neither may be silent: the amendment states whether the
   identity survives, and if it does not, it retracts the sentence above, the `waste_tolerance` `0` entry in *Acceptance
   examples*, and any test asserting them, naming each." **`REQ-MOK-033` depends on this**, since it requires a
   trait-aware source that does not change the existing sources and the `T = 0` identity is how that is stated at the
   range's lower bound. If the identity does not survive, `REQ-MOK-033` needs an amendment row of its own, approved by
   its owner, and that is a precondition of approval rather than a consequence of implementation.
3. **Whether a per-class composition floor accompanies the ceiling.** Deferred by `REQ-MOK-060`'s *Open decisions* until
   the corrected composition had been measured, on the ground that "a ceiling alone can be satisfied by a world that has
   drifted the other way". The pre-change curve is now retained; the post-change curve is produced by this work order, so
   the decision may be taken at approval on the first or reserved to the measurement of the second, and which of those it
   is should be settled when this work order is approved rather than assumed.

Two further deferrals travel with the requirement and are noted so they are not lost: whether branch 1 should be able to
decline to answer at all, and whether `REQ-MOK-058`'s lethality bound should become a rate rather than an existence
claim. Neither blocks this work order and neither is an implementation agent's.

### Approval decisions of 2026-08-21

**All three approval preconditions were decided by the repository owner on 2026-08-21, and two further decisions were
taken with them, because the first precondition's answer forced them.** The decisions are recorded here in the order they
were put; each was put with its measured alternatives rather than as a recommendation with reasons attached.

**Stop condition 4 fired before this work order was approved, and this section is the escalation being answered.** The
measurement the *Approval preconditions* reserve the first decision to did not produce a mechanism that meets the
ceiling. It produced a wall: **one half is not reachable inside `REQ-MOK-060`'s permitted surface while the three carried
survivor floors hold.** Stop conditions 4 and 5 are jointly unsatisfiable at the value the requirement stated, which is
precisely the case the two conditions were written to catch, and neither the mechanism nor the floors were adjusted to
escape it. The measurement is retained in `evidence/WO-MOK-017/approval/`: roughly 55 parameter points across three
mechanism shapes, each evaluated on all 15 obligated cells, plus a 150-cell survey over unbound seeds. Its load-bearing
figures are that the closest point meeting one half on every cell leaves 6 of 12 living under `reference` and
`individual`, and that no point holding all three floors meets one half at all — the best floor-respecting point measured is the condition ratified below, whose worst class share is `54.1%`, four points above one half.

1. **The ceiling is amended from one half to three fifths**, by the owner acting as product owner, on the measured curve.
   `REQ-MOK-060`'s amendment record of 2026-08-21 carries the decision, its figures and its three declined alternatives;
   this work order does not restate them. Two of those alternatives were live options for this work order rather than
   rhetorical, and both were declined by the owner and not by the agent: amending the three floors downward to 6, 6 and 3,
   and widening the correction surface to rule 15, rule 16, rule 9's eat effect or the food table. The second is the one
   stop condition 4 forbids the implementation from reaching for, and it stayed forbidden.
2. **Rule 19's `T = 0` proposal identity is preserved**, by the owner acting as technical owner, by restating rule 19's
   first clause as a reference to rule 5's condition rather than as the literal inequality `S + R <= 100`. This is the
   first of the two outcomes `SPEC-MOK-001` rule 19 permits, so **nothing is retracted**: the identity sentence stands,
   the `waste_tolerance` `0` entry in *Acceptance examples* stands, and every test asserting either stands. **`REQ-MOK-033`
   therefore needs no amendment row**, and the precondition attached to that consequence in *Approval preconditions* is
   discharged by the identity surviving rather than by an approval. The restatement is not cosmetic and not optional: the
   measurement shows `individual` and `social` both read rule 19's test and not rule 5's, so without it a rule 5
   correction reaches one of the three obligated sources and 14 of the 15 cells still breach.
3. **The per-class composition floor remains deferred**, by the owner acting as product owner, and is reserved again to
   the post-change composition this work order measures rather than taken on the approval measurement. `REQ-MOK-060`'s
   *Open decisions* carries the re-reservation. What this settles for the implementation is that a floor is not in scope
   here and that the post-change per-class figures are retained so the decision can be taken on them, which the *Evidence
   to record* list already requires.
4. **The numeric form `S + R - 100 <= R * R / 100` is ratified**, by the owner acting as technical owner. It is rule 19's
   own arithmetic evaluated at a tolerance equal to the restoration itself, which is why it needs no new constant and no
   new shape in the specification. Its measured consequences, all retained in `evidence/WO-MOK-017/approval/`: the
   allowance above `100` becomes `2` for low class, `9` for medium and `25` for high, so the satiety a Mokiterion will eat
   up to moves from today's `85`, `70` and `50` to `87`, `79` and `75`; the worst class share over all 15 obligated cells
   is `54.1%`; and minimum survivors are 8, 8 and 7 against the floors of 8, 8 and 5. Alternatives measured and declined:
   `R * R / 120`, which breaks the amended ceiling on the declared seeds, and the two-term forms `25R/100 + 10` and
   `20R/100 + 10` on high class alone, which hold the floors with more room but leave a worst share of `59.3%` and
   introduce a class-specific constant the specification does not have.
5. **This work order is approved and set to `in_progress` in the same act**, by the owner acting as engineering owner, and
   the four governance amendments the decisions above require are authorized to be written by the implementation agent
   under them. Those amendments are `REQ-MOK-060`'s ceiling amendment (product owner), `VER-MOK-016`'s realignment
   (assurance owner), `SPEC-MOK-001`'s second amendment to rules 5 and 19 (technical owner) and this section
   (engineering owner). **The agent wrote the text; it decided none of the substance**, and each amendment row names the
   owner and the role the decision was taken in. The alternative of taking the amendments in one round and the approval in
   a second was declined by the owner, so that the engine change, the 90-cell re-capture and the evidence packet follow
   without a further round trip.

**What is not decided by this section.** The three floors are unchanged at 8, 8 and 5, and stop condition 5 is in force
against them at the values they hold today — the approval measurement is not the re-measurement this work order owes, and
a floor missed on the shipped build is still a stop. The permitted surface is unchanged. `REQ-MOK-058`'s lethality bound
is unchanged. The declared seed set, the density and the 1,000-tick horizon are unchanged.

**One disclosure the approval measurement carries, which no obligation covers.** Over the 50 unbound seeds `0`–`49` the
corrected condition is a large improvement on the ceiling and a small regression on the floors: cells above `60%` fall
from 140 of 150 to 7 of 150 and the worst share from `88.5%` to `63.9%`, while cells below their source's floor rise from
6 of 150 to 17 of 150. No requirement binds those seeds and none is being read as if it did. It is recorded because the
ratified form was chosen against a shape that trades the other way, and the owner saw both before ratifying.

### Consequential citation corrections of 2026-08-21

Two stale references were found while the decisions above were being measured, and correcting them is authorized with
them. Neither changes an obligation.

- **This work order's own citation of the composition curve.** *Why this work order exists* cites
  `evidence/WO-MOK-016/composition.md`; the retained file is `evidence/WO-MOK-016/post/composition.md`. Corrected in place
  above, because a draft's body text is not an approved amendment row.
- **`SPEC-MOK-001` rule 5's accumulation paragraph and rule 19 both say the mechanism is "decided on measurement under
  `WO-MOK-016`".** `REQ-MOK-060` was descoped from that work order on 2026-08-21 and the measurement was taken here, so
  both are repointed to `WO-MOK-017` as part of the second amendment rather than as a separate edit. This is added to
  *In scope*.

### Transition to `approved` and `in_progress`, 2026-08-21

**Set to `approved` and then to `in_progress` on 2026-08-21 by the repository owner acting as engineering owner**, in the
act recorded as decision 5 above. The owner's instruction was to write the four amendment texts and authorize the
transitions together, so that implementation continues into the engine change, the re-capture and the evidence packet
without a second round trip.

`approved` authorizes the scope below and nothing wider. `in_progress` records that implementation has begun. It does not
authorize `implemented`, which requires the completed change and the retained evidence, nor verification or release,
which require separate commit-bound records, nor any push, pull request, tag or publication — those remain the owner's
acts and are not authorized by this transition. *Out of scope*'s bar on the agent transitioning artifact status is lifted
for these two transitions only, on the owner's explicit instruction, and remains in force for every other artifact.

### Transition to `implemented`, 2026-08-22

**Set to `implemented` on 2026-08-22 by the repository owner acting as engineering owner.** The instruction, verbatim and
complete:

> yes go for commits A and B + preparation of the verification record

It was answered to a framing that named the three commits it authorizes and named what it does not: commit A, the three
manual assessments and the findings' dispositions; commit B, this transition; and the composition of the verification
record as `ready`. The `ready` → `verified` transition was excluded from the framing and is excluded here.

**What the lifecycle clause requires of this status, and where each is met.** *Transition to `implemented` requires the
completed change and the retained evidence.*

- **The change is complete and merged.** `master` merged this work order's branch at
  `f7b1c452039dc2f03010ca8b8cc81e73c54727c0`, pull request #39, and that pull request carried five passing checks — the
  declared-dependency check, both candidate checks and both governor checks. The merge of `master` into the branch at
  `ae2e44f2382fe89f2daf78a8e8aca37febb8bd0f` is accounted for in `evidence/WO-MOK-017/merge/`, which states the
  byte-identity licence its carry-forward of source figures rests on rather than re-measuring 120 cells that did not
  move.
- **The evidence is retained**, under `evidence/WO-MOK-017/`, in the five sub-packets `pre/`, `post/`, `approval/`,
  `analysis/` and `merge/`, with `capture.sh` retained beside them. Every item of *Evidence to record* above has a
  record, including the one that could not be produced: `post/entropy.txt` §5 records why the per-run draw total is not
  derivable at this candidate and what stands in its place, and that gap is finding 1 of `manual-assessment.md`, now
  disposed.
- **The manual assessments this work order reaches are recorded**, which is the last item of *Evidence to record* and is
  the reason this transition follows commit A rather than preceding it. `VER-MOK-016`'s assessments 5a, 5b and 6's second
  half were recorded on 2026-08-22 by the repository owner as product owner, and eight of the nine findings raised beside
  them carry a disposition of the same date. **This ordering is deliberate and it is drawn from the precedent this chain
  set:** `VREC-MOK-016` was captured at `4539601` before `WO-MOK-016`'s eight outstanding assessments were signed at
  `ecba9fe`, and the record had to be superseded and rewritten at the later commit, because a verification record binds
  one commit and cannot be re-pointed at another. Signing first is what keeps this work order's record from repeating it.
- **The gates pass at the commit this transition is written on.** `cargo fmt --all -- --check` clean;
  `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` clean; `cargo test` **302 passed, 0
  failed** across both packages and both tiers; `scripts/check_declared_dependencies.py` reports every declared set
  matching its resolved graph; `python -m se_harness validate` PASS at 147 artifacts, 0 errors and 0 warnings on all four
  planes; and `python -m se_harness preflight --work-order WO-MOK-017 --phase review` PASS.

**What this transition does not authorize, and what it does not assert.** It authorizes no push, pull request, tag,
release or publication, and *Out of scope*'s bar on an implementation agent transitioning artifact status stays lifted for
this transition only. It asserts nothing about the two retained reports that read `RESULT: FAIL` —
`merge/world-rules-merged.txt` at 3 failed checks over 61 changed engine blocks, and `merge/reads-merged.md` at 1 failed
check for `write_metrics_territory` — which are the assurance owner's to read, and which no self-assessment retires. The
verification record composed against this commit is composed as `ready` for the same reason.

**Acts owed in other artifacts, none of them taken here.** `SPEC-MOK-004` rule 11 still reads `301` on `master` and its
amendment row of 2026-08-21 is still marked `**OUTSTANDING.**`; the correction to `302` is drafted and unapplied at
`merge/SPEC-MOK-004-rule-11.md`, and ratifying both rows is one act of the technical owner in that specification's own
record. `REQ-MOK-060`'s *Open decisions* third bullet still reads "remains deferred" where assessment 5b declined to bind
a per-class floor. `VER-MOK-013` and `VER-MOK-005` each owe an amendment row for findings 2 and 3, and `VER-MOK-016` owes
the further row finding 7 needs. None of the four is inside `evidence/WO-MOK-017/`, so none of them stales this packet or
the record that declares it.

**This is not the terminal status.** Commit-bound verification is classified `required` above, so a verification record
closes this work order, against a commit that includes this transition. **That record is `VREC-MOK-020`.**
`merge/README.md` nominated `VREC-MOK-017`, and that identifier is not free: it is `WO-MOK-016`'s record, already
`verified`. The identifier space was swept across every local and remote ref before this number was taken —
`VREC-MOK-001` through `VREC-MOK-019` exist and nothing at `VREC-MOK-020` or above appears anywhere in history — because
that space is shared across branches and sessions and the local maximum is not the free number.

## Objective

Correct the resource composition drift that `SPEC-MOK-001` rule 5 records and `REQ-MOK-060` obliges, by amending the
waste condition of the two decision sources that produce it and nothing else; re-measure `REQ-MOK-014`'s and
`REQ-MOK-034`'s survivor floors against the corrected world; and demonstrate that `baseline` did not move at all, that
every `reference` and `individual` divergence is attributable to this correction, and that the shared entropy stream's
consumption is unchanged in kind and order.

## In scope

- The second `SPEC-MOK-001` amendment, which rule 5's accumulation paragraph and rule 19's added paragraphs already name
  as the one to make: the numeric form of the corrected waste condition, written into rule 5, into rule 19's tolerant
  test, or into both, according to the decision taken at approval.
- The consequential statement about rule 19's `T = 0` identity, in the form the specification's own text requires —
  either the identity is preserved by restating rule 19's first clause as a reference to rule 5's condition, or it is
  retracted together with the `waste_tolerance` `0` acceptance entry and every test asserting it, each named.
- Any `REQ-MOK-033` amendment row that consequence requires, written as its owner has approved it. **The identity
  survives, so no such row is required**; see *Approval decisions of 2026-08-21*.
- Repointing rule 5's accumulation paragraph and rule 19 from "decided on measurement under `WO-MOK-016`" to this work
  order, in the same amendment, since `REQ-MOK-060` was descoped from that work order and measured here.
- The engine change implementing the corrected condition, in the sources' proposal logic only.
- Re-measuring `REQ-MOK-014`'s floor of eight of twelve under `reference` and `REQ-MOK-034`'s floor of eight of twelve
  under the trait-aware source, at the default density, on the declared verification seed set, against the corrected
  world.
- Re-capturing the 90-cell three-source matrix and characterizing each of the 60 `reference` and `individual`
  divergences by first diverging tick, first diverging record and the eat-or-seek decision that changed — the clause
  `WO-MOK-016` recorded as unwritable, which becomes writable for the first time here.
- Measuring the corrected composition ratio per territory per calorie class, per seed, under `reference`, `individual`
  and `social`, and comparing it against the curve `WO-MOK-016` retained.
- Re-measuring `REQ-MOK-058`'s survivor floor and lethality bound under `social`, because the corrected condition changes
  what a `social` Mokiterion eats and `REQ-MOK-058`'s floor of five was ratified on the uncorrected world.
- The `SIMULATION_RULES.md` and `docs/ROADMAP.md` updates this change requires.
- Every check, measurement and manual assessment `VER-MOK-016` requires of `REQ-MOK-060`, and the evidence it retains.

## Out of scope

- **Any change to `baseline`'s output, at all, at any seed, density or trace setting.** This is the constraint the whole
  permitted surface exists to protect, and `INT-MOK-010`'s byte-identity promise rests on it.
- **Rule 16's uniform class selection, rule 15's regeneration amount, rule 9's eat effect, and the food table's
  restoration values.** Each is world behavior under every policy, so each would move `baseline`; rule 16's would
  additionally risk moving the shared stream's consumption. `REQ-MOK-060` forbids all four and this work order does not
  reopen that.
- **The behavioral trait's range of `0..=40` and its derivation.** `SPEC-MOK-001` rule 19 already fixes that a tolerance
  floor belongs to the test and not to the range, so a Mokiterion's reported `waste_tolerance` continues to mean what it
  means today.
- **Any new attribute, event, interface item or instrumentation.** Rule 18's summary already reports what is measured.
- **The default density and the default policy.**
- Entropy substreams, still declined for Phase 2's reason.
- Interaction memory, perceived relative strength, cooperation and territorial defence — all still Phase 3.2, 3.3 or
  later, and untouched here.
- Anything `WO-MOK-016` implemented. Contact, the seven targeted actions, combat resolution, the suffered-attack window
  and the `social` source are in force and are not re-opened; this work order changes what a Mokiterion eats, not what
  it does to another Mokiterion.
- Committing, pushing, tagging, releasing or publishing anything, and transitioning any artifact's status. Those are the
  owner's acts.

## Authorized decision envelope

The implementation agent may decide locally:

- The internal structure of the corrected condition in code, provided it is the specification's arithmetic and is
  expressed once rather than duplicated across the two sources.
- Which test tier each new test belongs to, applying `SPEC-MOK-004`'s rule as written. Widening an item to `pub` to
  relocate a test is prohibited.
- The internal organization of test modules, constructed-state helpers and evidence scripts, provided every script is
  retained in full.
- The exact wording of amended text, once its owner has approved the substance, and of the plain-language and roadmap
  updates.

The implementation agent may **not** decide:

- Any of the three approval preconditions above.
- The ceiling's value, which is three fifths as amended on 2026-08-21 by the product owner. Neither the agent nor this
  work order may move it again, and a post-change curve that misses it is stop condition 4 rather than a value to adjust.
- Whether the `T = 0` identity survives, or the content of any amendment row.
- Any change to a floor, a threshold, a default or an exit code, including `REQ-MOK-014`'s and `REQ-MOK-034`'s floors of
  eight and `REQ-MOK-058`'s floor of five. A floor that the corrected world misses is a stop condition, not a value to
  adjust.
- Where the correction is placed, beyond the permitted surface `REQ-MOK-060` fixes.

## Constraints

- `baseline` is byte-identical on all 30 of its declared cells, compared byte for byte and not by projection.
- The shared entropy stream's consumption is unchanged in kind and order. The correction takes no new draw and moves no
  existing one. Where `reference` and `individual` runs consume a different total number of draws because they consume
  food at different times, that is a consequence of different world evolution and not of a changed draw discipline, and
  it is recorded as such.
- No population aggregate and no composition read. In particular no source may read the class composition of its
  territory in order to decide what to eat, which `REQ-MOK-060` forbids explicitly and `REQ-MOK-059` forbids in the
  general case.
- No approved amendment-record row is edited, and no record bound by a `verified` verification record is edited.
- Evidence is re-run rather than edited. Where a retained reader output belongs to `WO-MOK-016`'s commit it stays as it
  is, and a superseding measurement is a new record.

## Expected change surface

- The simulation engine's decision sources, in the two waste conditions only.
- `SPEC-MOK-001` rule 5's accumulation paragraph and rule 19's tolerant test, plus the *Acceptance examples* entry if the
  `T = 0` identity does not survive.
- `REQ-MOK-033`'s amendment record, if that consequence lands.
- The workspace test suite, in the tests asserting the waste condition, the `T = 0` identity, and both survivor floors.
- `SIMULATION_RULES.md` and `docs/ROADMAP.md`.
- A new evidence directory for this work order.

## Required verification

`VER-MOK-016` covers this work order and already carries `REQ-MOK-060` in its `verifies` relation together with the five
coverage rows for it: the ceiling itself, its measurement from rule 18's summary, the pre-change contrast, the
static-analysis check that the correction is where it is permitted to be, and the static-analysis check that no source
reads composition. Manual assessment 5 — the ceiling's ratification or amendment on the measured curve — travels with
them.

No new verification requirement is created for this work order, on `WO-MOK-008`'s precedent of reusing an existing one.
If the assurance owner judges that `REQ-MOK-060`'s coverage should move to a verification requirement of its own, that is
their act and a precondition of approval rather than something this work order assumes.

## Evidence to record

- The pre-change capture for this work order's own commit, and the post-change capture of the same 90 cells, retained by
  per-cell SHA-256 digest with a stated subset retained whole.
- The byte comparison of the 30 `baseline` cells.
- The characterization of each of the 60 `reference` and `individual` divergences, by first diverging tick, first
  diverging record and the decision that changed.
- The shared stream's recorded state either side of every resolution kind, and the total draw count per run per source at
  matched seeds.
- The composition ratio per territory per calorie class per seed under `reference`, `individual` and `social`, at both
  commits, set against `WO-MOK-016`'s retained curve.
- Per-seed survivor tables under `reference`, `individual` and `social`, against the floors of eight, eight and five.
- The line-for-line comparison showing rule 4, rule 9's eat effect, the food table and rules 14 to 16 unchanged.
- The enumeration showing no source reads composition.
- The pre-change and post-change workspace test census, reconciled name by name, with every retracted test named.
- `cargo fmt`, `cargo clippy` and `cargo test` output.
- The manual assessments this work order reaches, each with its accountable role and date.

## Stop and escalate conditions

Stop and escalate rather than proceeding, if:

1. **Any `baseline` cell differs**, in any byte or exit code, at any seed, density or trace setting.
2. **The shared stream's position moves** at any seed or density, or across any resolution.
3. **A `reference` or `individual` divergence cannot be attributed** to the corrected waste condition.
4. **The ceiling cannot be met within `REQ-MOK-060`'s permitted mechanisms.** Reaching for rule 16, rule 15, rule 9's eat
   effect or the food table is not a fallback available to the implementation; it changes what the project promises about
   `baseline` and it is the owner's decision. **This condition fired at one half before approval and was escalated rather
   than worked around**; the owner amended the ceiling to three fifths and left the surface closed. It stands unchanged
   against three fifths, and the widening it forbids was among the alternatives the owner declined.
5. **`REQ-MOK-014`'s or `REQ-MOK-034`'s re-measured floor of eight is missed**, or **`REQ-MOK-058`'s floor of five or its
   lethality bound is missed**. A correction that fixes composition by starving the population is not a correction, and
   `REQ-MOK-060` says so in its own *Required response*.
6. **The `T = 0` identity's fate turns out to be undecidable within the approved mechanism**, or a `REQ-MOK-033`
   amendment is needed that its owner has not approved.
7. **A degenerate composition appears** — a territory driven to a single class, or to zero of some class. This satisfies
   the ceiling as stated and `REQ-MOK-060` records it as something the owner is to see and decide a floor on, so it is
   reported rather than corrected.

## Completion report format

1. The mechanism chosen, and the decision that authorized it;
2. The amended specification text, with its amendment-record row;
3. What became of rule 19's `T = 0` identity, and every sentence, acceptance entry and test retracted by name;
4. The 30 `baseline` cells' byte comparison;
5. The 60 characterized divergences;
6. The shared stream's state either side of every resolution kind, and the draw counts;
7. The composition ratio per territory per class per seed, at both commits, against `WO-MOK-016`'s curve;
8. Survivor figures against the floors of eight, eight and five;
9. The test census, reconciled name by name;
10. The gate output;
11. Every manual assessment reached, and every one still outstanding.
