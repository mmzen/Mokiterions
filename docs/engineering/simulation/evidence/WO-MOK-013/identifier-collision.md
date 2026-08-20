# The second identifier collision — four names, both sides approved, both sides pushed

**This note measures a collision and takes no decision.** It records what was found, when, by what sweep,
and what it costs to resolve from this side. It does not renumber anything, does not propose which side
should, and does not treat the collision as settled. That is decision 3's rule and stop condition 8's
escalation, and both are named at the end.

It follows `evidence/WO-MOK-012/identifier-collision.md`, which measured the first collision in this chain
on the same pattern. **This is a different and a worse one.**

## What was found

On 2026-08-20, before `VREC-MOK-013` was captured, all **27** remote refs were swept for the six identifiers
this chain introduces. One branch holds four of them.

`origin/governance/adr-mok-006-third-party-crates`, tip
`753a8b9be0ae70cfd6041886b8bcb8a5ab582726`, committed `2026-08-20T17:43:09+02:00`, subject *"gov: re-point
VREC-MOK-013 to candidate 65ac88b"*. `git merge-base` with this branch is **`ff3a155`** — the same commit
this chain is based on — and `git branch -r --contains` reports it on that branch alone, so **it is not
merged into `master`**.

> **The colliding ref moves, and these figures are re-derived rather than carried.** The first sweep, taken
> before `VREC-MOK-013` was captured, found the tip at `f40b711b0985316494c6d3ac24f9b0434b0f0ad7` of
> `2026-08-20T14:51:06+02:00`. By the time this branch was published it had advanced twice, to `753a8b9`,
> and their `VREC-MOK-013` had been re-pointed from candidate `d33ecb8` to `65ac88b`. Every figure below is
> re-measured at `753a8b9`; the two that moved are named where they appear. **The collision itself did not
> move**: the same four identifiers, the same statuses, and `REQ-MOK-048` and `REQ-MOK-049` still free. A
> reader re-deriving any of this should expect the tip to have advanced again and should re-measure rather
> than quote.

| Identifier | This chain | `governance/adr-mok-006-third-party-crates` |
|---|---|---|
| `WO-MOK-013` | "Make the observer's survival gauges resolve, its controls discoverable, and its hidden-pane notice actionable", `implemented` | "Replace the engine's empty-dependency rule with a declared-set comparison, and check it at pull-request time and at release", `implemented` |
| `VER-MOK-013` | "Legibility verification: a gauge that resolves, a control the operator can find, and a notice that names the remedy", `approved` | "Declared dependency set verification: what each package resolves, at which features, and what no graph read can answer", `approved` |
| `VREC-MOK-013` | `ready`, bound to `41c20ca`, captured `2026-08-20T15:38:30Z` | `ready`, bound to `65ac88b`, captured `2026-08-20T15:29:34Z` — **re-pointed from `d33ecb8` after the first sweep** |
| `REQ-MOK-047` | "Render a survival gauge at a width that resolves the value it presents", `approved`, `verification_method = "automated-test"` | "Resolve each package's dependency set equal to its declaration, within the preserved trust envelope", `approved`, `verification_method = "static-analysis"` |

The two verification records were captured **eight minutes and fifty-six seconds apart** — theirs at
`15:29:34Z`, ours at `15:38:30Z` — from two clones of the same repository, against two commits neither of
which is an ancestor of the other. The first sweep put the gap at three hours and one minute, against their
earlier candidate; the re-pointing closed it to nine minutes. **Neither clone could have seen the other's
record when it captured its own**, and that is the point the figure carries: two agents named the same record
within the same ten minutes, and nothing in either clone's tooling said so.

**`REQ-MOK-048` and `REQ-MOK-049` are free.** Sweeping all 27 refs for content hits on either name returns
this branch and nothing else. The collision is four names, not six.

## Why this is worse than the first one

`evidence/WO-MOK-012/identifier-collision.md` measured a collision in which **one side's artifact was still
`draft`**, which left a cheap way out: the unapproved side could renumber without touching an accountable
act. Here **every one of the eight artifacts is owner-approved on its own side** — two `implemented` work
orders, two `approved` verification contracts, two `approved` requirements and two `ready` verification
records — and **both sides are pushed**:

- theirs entirely, at `753a8b9`;
- ours at `a339902` on `origin/assessment/wo-mok-005-remediation`, which already carries `REQ-MOK-047`,
  `REQ-MOK-048`, `REQ-MOK-049`, `VER-MOK-013` and `WO-MOK-013`. Only `VREC-MOK-013` was still local when
  this note was written, and only because it was captured two commits after the pushed tip. **The owner
  authorized publication of the whole branch on 2026-08-20**, and once it is published the sixth name is on
  `origin` too — so the collision is fully published on both sides rather than half of it.

So renumbering is no longer an edit to an unpublished draft on either side. It rewrites artifacts whose
approvals are recorded, in a repository where another agent's clone has already read the names.

**The evidence directories collide in path and not in content, which is the part that will not announce
itself.** Both chains write `docs/engineering/simulation/evidence/WO-MOK-013/`. Theirs holds **21** files at
`753a8b9`, every one named `WO-MOK-013-*` — 20 at the first sweep; ours holds 26, none of them prefixed.
`comm` over the two file lists is **empty**, so a merge would combine the two packets into one **47-file**
directory **with no conflict raised** — two unrelated bodies of evidence under one identifier, and nothing in
the merge output to say so. That count is the one figure here that grows on its own: their pack gained a file
between the two sweeps, and a merged directory is the sum of whatever both sides hold on the day.

That naming difference has one visible consequence already. The harness discovers evidence by **file name**,
not by directory: `scripts/generate_harness_dashboard.py` matches `^(WO-[A-Z0-9-]*\d{3})(?:-|\.|$)` against
`path.name`. None of this pack's 26 file names match, which is why `W-HEX-001` fires against `WO-MOK-013`
here — the fourth observation, added by the transition to `implemented` — and why a prefixed pack would not
raise it. The warning is a naming artefact and not a finding about either body of work.

## What resolving it would cost from this side

Measured on the working tree at the commit this note lands in, so that the figure is the real one and not an
estimate. It **rises as the chain is documented** — the first sweep counted 424 occurrences across 29 tracked
paths, taken while the verification record was still untracked — so it is a floor on the cost of renumbering
and not a fixed price:

| | |
|---|---|
| Occurrences of the four names across `docs/` | **430** — `WO-MOK-013` 182, `VER-MOK-013` 110, `REQ-MOK-047` 85, `VREC-MOK-013` 53 |
| Files containing at least one | **31**, of which this note is one: **32** of the 430 are its own citations |
| Tracked paths whose own name carries one | **30**, including the 26-file evidence directory that would move wholesale |
| Occurrences inside verbatim owner instructions | **10**, in **5** quoted spans |

The last row is the same obstacle the first collision note measured on its side, and it is the one that
cannot be resolved by rewriting. All five spans are the same instruction — *"you can transition WO-MOK-013 as
implemented, and create VREC-MOK-013"* — quoted in `WO-MOK-013`, `VREC-MOK-013`, `closing-review.md`,
`completion-summary.md` and this note, and an instruction quoted verbatim cannot be renumbered without
ceasing to be what the owner said. `evidence/WO-MOK-012/identifier-collision.md` reached the same conclusion
on its own count: a renumbering has to add a note saying which name the instruction referred to, rather than
edit the quotation.

## What a merge reports today

`git merge-tree --write-tree --messages`, re-measured at this branch's tip after the transition to
`implemented` and against their `753a8b9`. Both figures are unchanged from the first sweep:

| Against | Conflicts |
|---|---|
| `origin/master` (`ff3a155`) | **0** |
| `origin/governance/adr-mok-006-third-party-crates` (`753a8b9`) | **8** |

The eight divide cleanly, and only half of them are this collision:

- **Four add/add, one per colliding name** — `requirements/REQ-MOK-047.md`, `verification/VER-MOK-013.md`,
  `verification-records/VREC-MOK-013.md`, `work-orders/WO-MOK-013.md`. These are the collision.
- **Four content conflicts in artifacts both chains legitimately amended** — `ARCH-MOK-001`, `SPEC-MOK-002`,
  `SPEC-MOK-003` and `SPEC-MOK-004`, where both added amendment rows or moved declared counts. These would
  exist with no collision at all and are ordinary merge work.

Zero against `master` is worth stating: **neither branch blocks the other from merging first**, and
whichever does, the other's merge is where the collision becomes a conflict.

## What this note does not do, and what it leaves standing

- **It renumbers nothing.** Decision 3 of this chain's closing review, taken by the engineering owner on
  2026-08-20 for the first collision, reads: *"Neither side renumbers now. The conflict is resolved by
  whichever of the two branches merges to `master` second."* That rule reaches this collision unchanged, but
  it was decided about the other one, so applying it here is stated as an extension rather than assumed.
- **It does not decide which side renumbers.** By decision 3's rule that is settled by merge order and not
  by argument, and neither branch has merged.
- **It does not fire stop condition 8, and does not claim the condition covers it.** Condition 8's own words
  name "the `WO-MOK-012` identifier collision", and it "does not fire during implementation". It fires at
  the merge, and *reaching the merge without having renumbered is the escalation.* **This note is that
  escalation raised early**, while there is still a choice about which side pays.
- **It does not extend or reduce any claim `VREC-MOK-013` makes.** That record binds `41c20ca` and this note
  postdates it, so — following `VREC-MOK-011`'s treatment of `evidence/WO-MOK-011/merge/` — **it is
  deliberately not among that record's `evidence_paths`.** A record's evidence set is the capture's, not the
  discovery's.
- **It does not read, modify or fetch from the other agent's clone.** Every figure above comes from
  `origin/*` refs already in this clone's object store, read with `git ls-tree`, `git show`, `git grep`,
  `git merge-base`, `git branch -r --contains` and `git merge-tree`. Nothing was written outside this
  repository and no branch other than this one was checked out.
- **It states one thing plainly, because a reader will otherwise assume the opposite.** For as long as both
  branches exist, **the names `WO-MOK-013`, `VER-MOK-013`, `VREC-MOK-013` and `REQ-MOK-047` do not identify
  one artifact each in this repository.** Any citation of them — in this pack, in `docs/ROADMAP.md`, in a
  pull request or in a later record — resolves only together with the branch it was written on.
