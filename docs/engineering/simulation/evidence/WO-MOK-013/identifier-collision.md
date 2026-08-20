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
`f40b711b0985316494c6d3ac24f9b0434b0f0ad7`, committed `2026-08-20T14:51:06+02:00`, subject *"gov: attribute
VREC-MOK-013's snapshot correction to the fact already recorded"*. `git merge-base` with this branch is
**`ff3a155`** — the same commit this chain is based on — and `git branch -r --contains` reports it on that
branch alone, so **it is not merged into `master`**.

| Identifier | This chain | `governance/adr-mok-006-third-party-crates` |
|---|---|---|
| `WO-MOK-013` | "Make the observer's survival gauges resolve, its controls discoverable, and its hidden-pane notice actionable", `implemented` | "Replace the engine's empty-dependency rule with a declared-set comparison, and check it at pull-request time and at release", `implemented` |
| `VER-MOK-013` | "Legibility verification: a gauge that resolves, a control the operator can find, and a notice that names the remedy", `approved` | "Declared dependency set verification: what each package resolves, at which features, and what no graph read can answer", `approved` |
| `VREC-MOK-013` | `ready`, bound to `41c20ca`, captured `2026-08-20T15:38:30Z` | `ready`, bound to `d33ecb8`, captured `2026-08-20T12:37:27Z` |
| `REQ-MOK-047` | "Render a survival gauge at a width that resolves the value it presents", `approved`, `verification_method = "automated-test"` | "Resolve each package's dependency set equal to its declaration, within the preserved trust envelope", `approved`, `verification_method = "static-analysis"` |

The two verification records were captured **three hours and one minute apart** on the same day, from two
clones of the same repository, against two commits neither of which is an ancestor of the other.

**`REQ-MOK-048` and `REQ-MOK-049` are free.** Sweeping all 27 refs for content hits on either name returns
this branch and nothing else. The collision is four names, not six.

## Why this is worse than the first one

`evidence/WO-MOK-012/identifier-collision.md` measured a collision in which **one side's artifact was still
`draft`**, which left a cheap way out: the unapproved side could renumber without touching an accountable
act. Here **every one of the eight artifacts is owner-approved on its own side** — two `implemented` work
orders, two `approved` verification contracts, two `approved` requirements and two `ready` verification
records — and **both sides are pushed**:

- theirs entirely, at `f40b711`;
- ours at `a339902` on `origin/assessment/wo-mok-005-remediation`, which already carries `REQ-MOK-047`,
  `REQ-MOK-048`, `REQ-MOK-049`, `VER-MOK-013` and `WO-MOK-013`. Only `VREC-MOK-013` is local, and only
  because it was captured two commits after the pushed tip.

So renumbering is no longer an edit to an unpublished draft on either side. It rewrites artifacts whose
approvals are recorded, in a repository where another agent's clone has already read the names.

**The evidence directories collide in path and not in content, which is the part that will not announce
itself.** Both chains write `docs/engineering/simulation/evidence/WO-MOK-013/`. Theirs holds 20 files, every
one named `WO-MOK-013-*`; ours holds 26, none of them prefixed. `comm` over the two file lists is **empty**,
so a merge would combine the two packets into one 46-file directory **with no conflict raised** — two
unrelated bodies of evidence under one identifier, and nothing in the merge output to say so.

That naming difference has one visible consequence already. The harness discovers evidence by **file name**,
not by directory: `scripts/generate_harness_dashboard.py` matches `^(WO-[A-Z0-9-]*\d{3})(?:-|\.|$)` against
`path.name`. None of this pack's 26 file names match, which is why `W-HEX-001` fires against `WO-MOK-013`
here — the fourth observation, added by the transition to `implemented` — and why a prefixed pack would not
raise it. The warning is a naming artefact and not a finding about either body of work.

## What resolving it would cost from this side

Measured on the working tree at the commit this note lands in, so that the figure is the real one and not an
estimate:

| | |
|---|---|
| Occurrences of the four names across `docs/` | **424** — `WO-MOK-013` 179, `VER-MOK-013` 110, `REQ-MOK-047` 85, `VREC-MOK-013` 50 |
| Files containing at least one | **31** |
| Tracked paths whose own name carries one | **29**, including the 26-file evidence directory that would move wholesale |
| Occurrences inside verbatim owner instructions | **10**, in **5** quoted spans |

The last row is the same obstacle the first collision note measured on its side, and it is the one that
cannot be resolved by rewriting. All five spans are the same instruction — *"you can transition WO-MOK-013 as
implemented, and create VREC-MOK-013"* — quoted in `WO-MOK-013`, `VREC-MOK-013`, `closing-review.md`,
`completion-summary.md` and this note, and an instruction quoted verbatim cannot be renumbered without
ceasing to be what the owner said. `evidence/WO-MOK-012/identifier-collision.md` reached the same conclusion
on its own count: a renumbering has to add a note saying which name the instruction referred to, rather than
edit the quotation.

## What a merge reports today

`git merge-tree --write-tree --messages` at the prospective commit of this note:

| Against | Conflicts |
|---|---|
| `origin/master` (`ff3a155`) | **0** |
| `origin/governance/adr-mok-006-third-party-crates` (`f40b711`) | **8** |

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
