# The eleven ratifications of 2026-08-20

Eleven provisions across four approved artifacts were amended in the tree with an Approval cell reading
**OUTSTANDING** — meaning the amended text had been drafted by an implementation agent and never ratified by the
accountable owner. `VREC-MOK-005` discloses this in its section "Amendments to approved artifacts that this record does
not carry", and `WO-MOK-005` states of the first five that it "is not verifiable until they are given or the scope is
changed to avoid needing them".

**All eleven were ratified as written, without modification, by the repository owner acting as technical owner on
2026-08-20.** The act is recorded in `closing-review.md`; this file states what each provision is, what it changes, and
why ratification rather than revision was the right act.

## How the ratifications are recorded

Each is recorded by **editing that row's Approval cell in place**, stating the date, the role, that the provisions were
ratified as written, and that the row was **OUTSTANDING** until that act. No new row is appended.

That follows the precedent already in the tree: `SPEC-MOK-004`'s 2026-08-19 test-count row was ratified the same way in
`WO-MOK-010`'s closing review, and its Approval cell now opens "**Ratified 2026-08-19 by the repository owner acting as
technical owner** … It was **OUTSTANDING** until that act." Appending a row instead would leave the amendment record
with two rows for one amendment and an Approval cell that still reads OUTSTANDING, which is the state a reader would
have to reconcile by diff.

**No amended text changed.** A ratification records authority over text that already exists; had any provision needed
revision it would have been a new amendment requiring its own row, and this work order would have stopped and escalated.

## Where the counting was corrected before the question was put

The agent's first reading of `SPEC-MOK-004` treated it as carrying five separate OUTSTANDING rows. Reading the table
showed **one row carrying five provisions**, with two later rows already ratified in `WO-MOK-010`'s closing review. The
count was corrected before the question reached the owner, so the owner was shown eleven provisions rather than a larger
number that would have re-opened work already done.

---

## `SPEC-MOK-002` — four provisions, row dated 2026-08-18

All four were amended so the terminal observer of `SPEC-MOK-003` could be conformed to. None could have been part of
the 2026-08-17 approval of the observer chain: this specification was not on that branch when the approval was given
and it reached `master` afterwards.

### 1. Rule 1 — the workspace

"No second package, no workspace" narrowed to a workspace of exactly two packages.

**Why ratification.** The clause itself reserved an exception for an approved requirement, and `REQ-MOK-026` is that
requirement, approved 2026-08-17. The narrowing therefore takes an exception the rule already contemplated rather than
inventing one. The engine package's dependency table stays empty, which is the property the clause protected.

### 2. Rule 3 — the file freeze

The clause freezing `src/cli.rs` and `src/simulation.rs` against anything but a visibility change, scoped to the
`WO-MOK-003` restructuring it was written for.

**Why ratification.** Left unscoped the clause forbids any approved requirement from ever adding code to those two
files — a permanent freeze on two engine sources, which was not its subject. `SPEC-MOK-004` rule 6's byte-identity check
was later scoped the same way and for the same reason, and that scoping is provision 7 below, so the two are consistent.

### 3. Rule 5 — the interface enumeration

The closed enumeration grown by the read-only observation surface.

**Why ratification.** Rule 5 carries its own growth clause and the growth happened under it. The surface added is
read-only and value-only, so the enumeration grew without the interface gaining a mutation path.

### 4. Rule 6 — the prohibition

Narrowed from five named value types to the capability it was written to deny.

**Why ratification.** The five named types were a listing of instances, not the prohibition's subject. What rule 6
exists to deny is handing out mutable authoritative state, and the narrowing preserves that denial exactly. Nothing
about mutation, dependency direction, determinism or observable behavior is relaxed.

---

## `ARCH-MOK-001` — one provision, row dated 2026-08-18

### 5. The public-item prohibition

Narrowed from "mutable **or owned** authoritative state" to a mutable borrow of, or a reference into, that state, with
the matching conformance check narrowed the same way.

**Why ratification, and why this one matters most.** **This is the provision that makes the shipped observer legal.**
The prohibition as originally worded also forbade the owned, reference-free snapshots `SPEC-MOK-003` specifies, which
are the capability `REQ-MOK-019` through `REQ-MOK-025` and `REQ-MOK-027` require. Declining it would have made the
shipped observer non-conforming rather than making it safer, and the observer has been the instrument used to assess
Phase 2.

The owner was shown it in exactly those terms: **ratifying it authorizes what is already in the tree rather than
permitting something new.** The narrowing grants no mutation — no public item yields a mutable borrow of, or a
reference into, world grid, agents, resources, tick counter, entropy state or event log, in any build configuration
including tests — and the `ADR-MOK-001` trust boundary is where it was.

The wording it narrows reached `master` after the 2026-08-17 approval, under `WO-MOK-003`, which is why the approval
could not have covered it.

---

## `SPEC-MOK-003` — one provision, row dated 2026-08-18

### 6. *Data and interface contracts*, clause 2

Clause 2 said `advance_tick` was the only `&mut self` method on the surface that changes state. `Simulation::run`, the
`REQ-MOK-010` whole-run entry point, is a second. The clause now states both, why `run` is there, why it cannot be
narrowed away without relocating the engine's sources or duplicating the run loop, and the checks that can actually be
met — including that the observer reaches neither `run` nor anything leading to it. The method listing is corrected to
the real signatures and completed with `termination_reason` and `initialization_events`.

**Why ratification.** This row **corrects a false statement of fact rather than relaxing an obligation**.
`Simulation::run` was already public at `origin/master` before the observer existed, so the clause was wrong on the day
it was approved. The amendment makes the specification true rather than making the implementation permissible — the
distinction the owner was shown, and the reason ratification is the right act rather than a new decision.

`boundary-and-security-review.md` under `WO-MOK-005` is the measurement that found it. The non-perturbation property is
unaffected.

---

## `SPEC-MOK-004` — five provisions, row dated 2026-08-19

All five are consequences of deleting the `layout::Tier` enum, its `label` method, the `tier_for` function and the
`Panes::tier` field — a removal the `SPEC-MOK-003` rule 5 amendment of 2026-08-19 authorized when it replaced the
four-row tier table with one threshold per pane.

**The situation the owner was shown.** The owner had approved that rule 5 amendment and directed the implementation in
the same act, **but was not shown this consequence**: the implementation agent found it afterwards by measuring the
interface against rule 6, and reported it rather than treating the rule 5 approval as covering it. This ratification
answers that report. Three of the five are measured figures; two are not, and the owner was told which.

### 7. Rule 6 — the recorded interface extent

Recorded extent falls from 13 `layout` items to **10** and from **97** items to **94**; the same interface counted the
other way from 122 to **118**; public fields from 25 to **24**. The `layout` row's subject, "viewport tiers and the pane
geometry of each", is corrected because there are no tiers to name. A **Reduction** clause is added, symmetric with the
existing **Growth** clause. The byte-identity check is scoped to the `WO-MOK-006` restructuring.

**Why ratification.** The figures are measured outcomes of an approved removal, not decisions. The two provisions that
are not figures were each ratified on the reasoning stated in the row: the **Reduction** clause because removal should
be governed the way addition already was, and the byte-identity scoping because an unscoped check forbids the observer's
code from ever changing again, which was never its subject. Rule 7 is satisfied by construction here — the interface
only loses items, so no visibility widens.

### 8. Rule 9 — public-tier test counts

`tests/layout.rs` rises from 7 tests to **10** and the public tier from 77 to **80**. The +3 is net: three tests go,
because they assert the tier table and per-tier minimums rule 5 no longer defines, and six arrive.

**Why ratification.** A measured outcome. Rule 11 obliges a work order that adds a test to correct these figures, so
recording them discharges an existing obligation rather than creating one.

### 9. Rule 11 — executed totals

The observer's executed total rises from 109 to **112** and the workspace's from 169 to **172**, with the
"same before and after" clause scoped to the restructuring.

**Why ratification.** A measured outcome, under the same rule 11 obligation.

### 10. Rule 12 — the test-renaming scope

The second paragraph scoped to the `WO-MOK-006` restructuring, exactly as rule 13's clause already is.

**Why ratification.** Unscoped, the paragraph forbids any later work order from renaming a test in either package — a
freeze on test maintenance the rule was not written to impose. Concretely, it would make
`excluded_panes_are_the_ones_the_tier_omits` unrenamable after the term "tier" was deleted from the specification it
cites, so the rule would forbid the tree from being made consistent with itself. The assertion is unchanged; only the
name and the rule it names change.

### 11. Rule 13 — terminology

Rule 13 and the second worked example no longer say "layout tier".

**Why ratification.** A term the specification no longer defines. No rule changes what it requires.

---

## What the ratifications do not do

- **No commit-bound record is re-opened.** `VREC-MOK-003` measured this specification's 2026-08-17 content,
  `VREC-MOK-006` measured 97 items and 169 tests, and each was correct at its commit. None is edited.
- **`VREC-MOK-005` is not edited.** It is `verified`, bound to commit `f361370`, and it was verified *with* these
  eleven provisions disclosed as outstanding — its own words are that the eleven "stay OUTSTANDING" and that
  "verification does not advance the amendments". That was correct at its commit. These ratifications are the act the
  record said had not been taken; they do not retroactively change what it certified. A re-capture rather than an edit
  is the route if one is wanted.
- **No requirement, no obligation and no figure is changed by the act of ratifying.** Every figure in these rows was
  already in the tree; what was missing was authority over it.
- **Five later rows that asserted these provisions "remain OUTSTANDING" were moved to the past tense**, because the
  ratification made those sentences false. One of the five was also wrong when written — `SPEC-MOK-002`'s 2026-08-19
  row said both of its two 2026-08-18 predecessors remained outstanding, when only the first ever was. The corrected
  sentence records the miscount rather than deleting it.
