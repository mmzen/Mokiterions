# The `WO-MOK-012` identifier collision

Recorded 2026-08-20 on branch `assessment/wo-mok-005-remediation`. This note takes no decision. It records a measured
fact that the accountable owner has to resolve, and states what it does not resolve.

**Two different work orders now carry the identifier `WO-MOK-012`.** One is the governance work order this evidence
directory belongs to, approved and transitioned to `implemented` by the repository owner on 2026-08-20. The other is a
Phase 4a definition packet on a pushed remote branch, authored by another agent working in a separate clone of this
repository.

## The measurement

Taken after `git fetch --all --prune`, across all 26 remote heads, before any identifier was claimed for the chain
`WO-MOK-013` opens.

| | This clone | The other branch |
|---|---|---|
| Identifier | `WO-MOK-012` | `WO-MOK-012` |
| Title | Discharge the governance debt `VREC-MOK-005` discloses | Emit a structured record stream to an operator-named sink, leaving the observed run unchanged |
| Status | **`implemented`** | `draft` |
| `commit_bound_verification` | `not_required`, confirmed by the engineering owner | `required` |
| Implements | `REQ-MOK-019`–`REQ-MOK-027` | `REQ-MOK-042`–`REQ-MOK-046` |
| Verification | `VER-MOK-005` | `VER-MOK-012` |
| Branch | `assessment/wo-mok-005-remediation`, **not pushed** | `origin/feature/phase-4a-definition`, **pushed** |
| Commit | `dc6be47` | `de33d74`, committed 2026-08-20 09:17:11 +0200 |
| Base | `origin/master` at `ff3a155` | `origin/master` at `ff3a155` — the same commit |

Neither is merged into `master`. `git merge-base` puts both on `ff3a155`, so the two were authored in parallel from one
base by two agents that could not see each other's work.

The other branch claims eleven identifiers in total, all of them new, and it is the only remote ref claiming anything
above `master`'s maxima:

```
ADR-MOK-005  CAP-MOK-009  INT-MOK-009  REQ-MOK-042  REQ-MOK-043  REQ-MOK-044
REQ-MOK-045  REQ-MOK-046  SPEC-MOK-006  VER-MOK-012  WO-MOK-012
```

Ten of the eleven collide with nothing. `WO-MOK-012` is the one that does.

## Why the sweep of earlier that day did not catch it

`identifier-sweep.md` in this directory recorded what was free across the then-24 remote heads on 2026-08-20, and
`WO-MOK-012` was free when it was taken. `origin/feature/phase-4a-definition` did not exist yet. It was pushed later the
same day, from the same base, and the local clone did not learn of it until the `git fetch` that opened this chain.

**This is the hazard the sweep was written to guard against, and the guard worked in the direction it could work.** The
memory-recorded rule for this repository is that "the identifier space is shared across branches and sessions; the local
maximum is not the free number", and both `identifier-sweep.md` and `completion-summary.md` committed the next chain to
re-sweeping rather than trusting the earlier record — "the refs move". Re-sweeping found this. What no sweep can do is
reserve a number against a clone that has not pushed yet: two agents branching from one commit and choosing the next
free identifier will choose the same one, and neither is at fault for it.

## What is at stake in resolving it

The two artifacts are not equivalent, and the asymmetry runs in opposite directions.

**In favour of this clone renumbering.** The other branch is pushed and this one is not. A published identifier is one
other work can already cite; an unpublished one is cited only by files in this clone. Renumbering here costs nothing
outside this branch.

**In favour of the other branch renumbering.** This `WO-MOK-012` has been **approved and transitioned to
`implemented` by the accountable owner**, in two separate acts on 2026-08-20, both recorded verbatim in
`assurance-decision.md`. The other is `draft` and has been approved by nobody. Renumbering an artifact the owner
approved changes the identifier of the thing they approved: the approval instruction was *"i approve WO-MOK-012"*, and
after a renumber that sentence names a different work order. The record would have to say that the artifact approved as
`WO-MOK-012` is now `WO-MOK-0xx`, which is survivable but is a rewriting of the subject of a decision the owner took.
Renumbering a `draft` nobody has approved costs no decision anything.

**What the agent did not do.** It did not renumber either artifact. Renumbering this one alters the subject of two owner
decisions taken the same day; renumbering the other touches a branch belonging to a different agent, in a clone this one
was told to stay out of. Both are the owner's to direct. The three sibling clones in the parent directory were not
read, not modified, and not fetched into.

## The resolution — decided 2026-08-20 by the repository owner acting as engineering owner

**Neither side renumbers now. The conflict is resolved by whichever of the two branches merges to `master` second.**

The owner was shown three routes with this clone's renumbering cost measured: renumber here, direct the other agent to
renumber theirs, or defer to the merge. They chose deferral.

| The measured cost of renumbering this side | |
|---|---|
| Occurrences of `WO-MOK-012` | **98**, across **21** files |
| Directory to rename | `evidence/WO-MOK-012/`, **22** files |
| Occurrences inside verbatim owner instructions | **8** — 7 in `assurance-decision.md`, 1 in `closing-review.md` |

That last row is the one that made this more than a substitution. *"i approve WO-MOK-012"* is a quoted instruction; after
a renumber it names a different work order, so those eight would take an annotation rather than a rewrite.

**What deferral obliges, so that it is not mistaken for closure.** The conflict is not removed and its cost is not
reduced — it grows, because whichever branch merges second renumbers against a `master` that by then cites the identifier
too, in addition to its own tree. Three obligations follow:

1. **The second branch to reach `master` renumbers before it lands.** Not whichever is easier at the time; whichever is
   second.
2. **`evidence/WO-MOK-012/` is a directory rename**, not a text substitution, and the eight quoted instructions take
   annotations that preserve what the owner actually wrote.
3. **The eight quoted instructions are the reason this side is the more expensive one to move.** If this branch merges
   first, the cost falls on a `draft` nobody approved, which is the cheaper outcome — and that is a consequence of merge
   order, not a plan.

`WO-MOK-013` stop-and-escalate condition 8 carries this: the condition does not fire during implementation and does fire
at the merge.

## What this note does not resolve

- **Which artifact renumbers**, in the sense of naming one today. Decision 3 names a *rule* — the second to merge — and
  not a side. Which side that turns out to be is not yet determined, because neither branch has a merge date.
- **Whether `REQ-MOK-042`–`REQ-MOK-046`, `VER-MOK-012`, `SPEC-MOK-006`, `ADR-MOK-005`, `INT-MOK-009` and `CAP-MOK-009`
  stand.** Nothing here contests them. They collide with nothing and this chain avoided them.
- **The Phase 4a packet's substance.** Not reviewed. It was read only far enough to establish that its `WO-MOK-012` is a
  different work order from this one, which its title, its requirements and its `[assurance]` classification each
  establish on their own.
- **Whether the harness should reserve identifiers across clones.** A real question and not one this note answers. It
  would be an amendment to the harness contract, which is a managed file, and no such change is authorized here.

## What the chain that follows did instead

`WO-MOK-013` and its pack took identifiers verified free against **all 26 remote heads**, by filename and by content:

| Claimed | Content hits across all refs before claiming |
|---|---|
| `REQ-MOK-047`, `REQ-MOK-048`, `REQ-MOK-049` | 0 |
| `VER-MOK-013` | 0 |
| `WO-MOK-013` | 0 |

`VER-MOK-009` was deliberately not filled, though no file carries it. `RELEASE_RUNBOOK.md` on `master` records why the
gap exists — "`WO-MOK-009` pairs with `VER-MOK-008`, not `VER-MOK-009`; the numbering does not line up" — so it is a
documented hole rather than a free number, and taking it would have made a runbook sentence false.
