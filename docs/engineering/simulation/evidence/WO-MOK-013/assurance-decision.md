# Assurance decision — `VREC-MOK-013` moved from `ready` to `verified`

**This file is not in `VREC-MOK-013`'s `evidence_paths`, and that is deliberate.** A record's evidence paths name what
the accountable owner accepted, and every one of the twenty-one it names existed at the candidate commit
`65ac88b0076dc1044adb4e6e984256b4428892b4`. This file postdates that commit and is the reasoning behind the status
rather than evidence the status accepts. Adding it would also move `artifact_snapshot_sha256`: the evidence path index
is inside the hashed document, so a decision that listed itself would change the digest of the graph it claims to have
reviewed. `evidence/WO-MOK-011/assurance-decision.md` set the same precedent on 2026-08-20 and is absent from that
record's 221 paths for the same reason, as the ten files under `evidence/WO-MOK-011/merge/` are.

**It takes no decision itself. It records one.**

Its name is also not prefixed `WO-MOK-013-` where the twenty-one evidence files in this directory are. That is
consistent with `evidence/WO-MOK-011/`, which uses unprefixed names throughout, and it is the reason
`docs/engineering/simulation/evidence/WO-MOK-013/` now holds two naming conventions — see *The identifier collision this
decision is taken inside*, because the directory name itself is contested.

## The instruction

> i validate VER-MOK-013, you can transition it, and commit + push

`DECISION_RIGHTS.md` reserves the `ready → verified` transition to the accountable assurance owner and states that
record preparation never makes it. The repository owner holds that role, as they hold the technical, product,
engineering and release roles, so **nothing here is approved by implication** — the instruction named one artifact and
three acts, and this file is the record of exactly those.

**The instruction names `VER-MOK-013`; the artifact transitioned is `VREC-MOK-013`.** That is a reading and it is
recorded rather than smoothed over, with the quotation left as it was given:

- `VER-MOK-013` is a verification contract at `status = "approved"`. All eleven verification contracts in this
  repository sit at `approved`, and none has ever been transitioned to anything else — there is no transition of a
  contract for this instruction to have meant.
- `VREC-MOK-013` was the only record in the repository at `ready`; the other eleven are already `verified`. It was also
  the sole entry in the inspector's `decision_required` queue, which is the queue an assurance validation answers.
- The same owner's instruction for the previous record, earlier the same day, read *"i validate the verification record
  VREC-MOK-011, you can transition it, commit and PR"* — the same three acts in the same order, against a record.
- The owner had already written *"i validate VREC-MOK-013"* in an earlier message the same day, naming the record
  directly. This instruction is the second, and it is the one that authorizes the transition, because it is the one that
  says *transition it*.

The same treatment is given elsewhere in this packet to `WO-POK-013`, which appears verbatim in two files because that
is what an instruction said. A quoted instruction is not corrected.

**`WO-MOK-013` was not moved.** No instruction named it, it stays at `implemented`, and `WORKFLOW.md` is explicit that a
work order's status never substitutes for a verification record's.

## What the decision accepted

In the `ready` record's own words, which are unchanged: *"It would accept the evidence enumerated above as satisfying
`VER-MOK-013` at commit `65ac88b0076dc1044adb4e6e984256b4428892b4`, and nothing beyond it"*. What that evidence is —
five oracles, twelve acceptance scenarios, the ten-way injection demonstration, the offline engine build and six manual
assessments — is enumerated in the record and is not restated here.

**One judgement was required and the owner gave it before this transition.** `VER-MOK-013:219` states that *"an
unrecorded assessment is an outstanding assessment, and this contract is not satisfied while any remains outstanding"*,
while its own amendment row at `:21` states that assessment 7 *"is not yet due"*. Asked which of the two readings the
contract's wording carries, the owner answered that **not yet due is not outstanding**. So:

- assessments 1, 2, 3, 4 and 6 are recorded, assessment 5 does not exist, and assessment 7 is not due;
- nothing is outstanding, and **`VER-MOK-013` is satisfied at `65ac88b`**;
- the alternative reading would have made the contract unsatisfiable until some later work order admits a crate, since
  only an admission can close assessment 7. That outcome is avoided by the reading and not by any evidence.

**This is the one difference from the `VREC-MOK-011` decision.** There the owner decided on the record with one of seven
manual assessments genuinely unperformed, so `VER-MOK-011` is **not satisfied** and that record says so throughout.
Here the contract is satisfied, and the load-bearing part is a reading of the contract's own wording rather than an
acceptance of a gap in the evidence. `VER-MOK-011`'s fifth assessment is a different question and is untouched by this
decision.

**Assessment 6's limit travels with the acceptance.** It accepted a compiled and uncalled transitive network capability
— `mio 1.2.2`'s `net` feature, reached through `ratatui` → `ratatui-crossterm` → `crossterm`, in two of three release
builds — and is **void the moment a behavior calls it**. Verifying this record does not widen that.

## What the decision does not retire

1. **`VER-MOK-005` and `VER-MOK-008` stay owed, and one of them is this change's own debt rather than inherited.**
   After the correction at `65ac88b`, `WO-MOK-013` declares `REQ-MOK-047` alone and this record conforms to
   `VER-MOK-013` alone, so no declared relation runs from `REQ-MOK-026` or `REQ-MOK-036` to the work order that rewrote
   their text. Both contracts still assert the withdrawn empty-dependency property at four sites —
   `VER-MOK-005:149`, `:231`, `:234`, `:296` and `VER-MOK-008:169`, the last of which also names a `release.yml` step
   this branch replaced with `scripts/check_declared_dependencies.py`. The owner scoped the repair on 2026-08-20 to
   those two contracts plus `REL-MOK-001`, at the depth of rewording those sites, in a separate work order taken up
   after pull request #33 merges. **The measured cost is 7 warnings to 8**, because staleness then propagates to
   `REL-MOK-001` twice and to `WO-MOK-008` once, where reassessing `REL-MOK-001` with them takes it to 6. Those
   coordinates are this branch's; `master` has since amended `VER-MOK-005`, so they must be re-derived against the
   merged tree before the repair is attempted.
2. **Assessment 7 is not discharged. It was found not to be due.** The first admission of a crate beyond `ratatui` owes
   it by name, together with assessments 1 and 2 for that crate and the seven-item evidence set `VER-MOK-013`'s *What
   an admission must retain* fixes.
3. **The identifier collision is untouched, and this decision raises the cost of resolving it.** See the next section.
4. **The merge, and every figure that will need re-deriving through it.** `65ac88b` is an ancestor of neither
   `origin/master` nor `755db72`. `origin/master` is at `798e5d5`, twelve commits past this branch's base `ff3a155`;
   pull request #33 is open, has not merged, and `git merge-tree --write-tree --messages HEAD origin/master` reports
   eight conflicts. Every figure in this packet stated relative to `ff3a155` — the amendment-row comparisons in
   `WO-MOK-013-amendments.md`, the `W-HEX-003` baseline — describes a base that has moved. **A record bound to whatever
   merge commit results is a new record, owed and not created here.** One such figure was re-checked and holds:
   `Cargo.lock` at this commit is byte-identical to `798e5d5`'s as well as to `ff3a155`'s.
5. **The release.** `REL-MOK-001` is `approved` and, by its own line at `:189`, *"closed by this approval"*; `RLS-MOK-001`
   released 0.1.0 from `755db72`, a commit that does not include this work. **No release record binds this work and none
   is created here.** `REL-MOK-001:94-96` still states *"`cargo tree -p Mokiterions --locked` resolves to one crate"* as
   a release-blocking requirement — the very property `ADR-MOK-006` withdrew — and it is **not amended**, because
   amending a closed contract whose release has been performed would rewrite a discharged decision. No workflow or
   script enforces a crate count any longer; `.github/workflows/release.yml:305-318` replaced that assertion with
   `check_declared_dependencies.py` and records the replacement in its own comment. A future release needs a new release
   contract and inherits nothing from that clause.
6. **What predates this branch stays as it was.** `VER-MOK-011`'s fifth manual assessment, the four `OUTSTANDING`
   amendment rows from 2026-08-18 and 2026-08-19, and the verification record owed against an earlier merge commit.
   None is cleared here and none is claimed as settled.

## The identifier collision this decision is taken inside

**`WO-MOK-013`, `VER-MOK-013`, `VREC-MOK-013` and `REQ-MOK-047` each name two different artifacts across two branches.**
Of this chain's five identifiers only `ADR-MOK-006` is free. `master` holds a `WO-MOK-013` titled *"Make the observer's
survival gauges resolve, its controls discoverable, and its hidden-pane notice actionable"* and a `VREC-MOK-013` binding
candidate `41c20cad`, captured at `2026-08-20T15:38:30Z` where this one was captured at `15:29:34Z` — **eight minutes
and fifty-six seconds apart, from two clones, against commits neither of which is an ancestor of the other.**

The disposition is the owner's and **it is on hold**: asked on 2026-08-20, they answered *hold — decide nothing yet*,
and separately that whatever is done belongs inside the existing work order and that integration is by merging `master`
in. The standing rule is decision 3 of the colliding chain's closing review, taken as engineering owner: *"Neither side
renumbers now. The conflict is resolved by whichever of the two branches merges to `master` second."* **Master merged
first, so that rule points at this branch renumbering.** `WO-MOK-014`, `VER-MOK-014`, `VREC-MOK-014` and `REQ-MOK-050`
are available for it — zero path hits across all thirty local and remote refs, re-swept at `798e5d5`.

**What this transition does to that, stated plainly rather than left to be discovered.** It makes this side's record
`verified`. `VERIFICATION_RECORD.template.md:30` permits a separate governance decision to move only a **`ready`** record
to `superseded`, and states no route for a `verified` one; the repository's only renumbering precedent —
`VREC-MOK-011`, moved from `010` to `011` — was taken while that record was still a candidate. `master`'s homonym is
still `ready` and still cheap to move; this one is not, and renumbering it would rewrite the subject of the decision
recorded in this file. **The collision was measured and put to the owner before this instruction was given; the owner
held its disposition and then instructed the transition.** That sequence is the reason this section exists rather than a
reason to have declined.

Two measurements of the exposure, for whoever takes the disposition:

- At `ee0c086`, the commit before this transition, the four names occur **553 times across 35 files** under `docs/` —
  `WO-MOK-013` 309, `VER-MOK-013` 132, `REQ-MOK-047` 92, `VREC-MOK-013` 20 — in **25 tracked paths** whose filenames
  carry one, plus `mokiterions-core/Cargo.toml:43`, `scripts/check_declared_dependencies.py` and its test module. This
  commit's text adds to that count. The other side priced its own renumbering at 430 occurrences across 31 files and 30
  paths, in `evidence/WO-MOK-013/identifier-collision.md` on `master`.
- **The evidence directory raises no conflict and merges silently.** Both chains write
  `docs/engineering/simulation/evidence/WO-MOK-013/`; `master`'s pack is 26 files under unprefixed names, this one is 21
  all named `WO-MOK-013-*`, and `comm` reports **zero** paths in common. A merge therefore yields one 47-file directory
  holding two unrelated work orders' evidence, with nothing reported by `git merge-tree` or by the validator, because
  every path in it is distinct. Of the eight conflicts that are reported, four are the add/add collisions —
  `REQ-MOK-047`, `VER-MOK-013`, `VREC-MOK-013`, `WO-MOK-013` — and four are ordinary content conflicts in
  `ARCH-MOK-001`, `SPEC-MOK-002`, `SPEC-MOK-003` and `SPEC-MOK-004`, where both chains added an amendment row.

**This file renumbers nothing and decides nothing about the collision.** It measures the exposure and names who owns the
disposition.

## No merge preceded this decision, and none follows from it

The `VREC-MOK-011` decision had to state that the merge came first and did not depend on it. Here the opposite needs
stating: **pull request #33 has not merged, and this verification does not authorize, enable or ready it.** It is `OPEN`
and not a draft — the owner took it out of draft on their own instruction — and `gh pr view 33` reports
`mergeable: CONFLICTING` with `mergeStateStatus: DIRTY`. Its five green checks were computed against `baseRefOid`
`ff3a155`, which is no longer `master`; **a green check row is not evidence of mergeability**, and the local
re-derivation above is what the eight-conflict figure comes from. Merging is the engineering owner's separate act, and
resolving the collision plausibly precedes it.

## Measured before and after

Both readings were taken in this clone with `HEAD` at `ee0c086` and the artifact content as the only moving input, which
is the only way a snapshot digest pair is comparable: `build_snapshot` puts `git rev-parse HEAD` into the hashed
document twenty-five times and the checkout directory's name once, so the same content measured after this commit exists
will hash to something else again. That is the mechanism `VREC-MOK-013`'s own snapshot section documents, and it is why
no digest is given here for this commit.

| Reading | Before | After |
|---|---|---|
| `validate_engineering_artifacts.py` | PASS — 107 artifacts, 0 errors, 0 warnings across four planes | **identical** |
| `generate_harness_dashboard.py` | PASS — 107 artifacts, 357 relations, 0 errors, 7 warnings | **identical but for the snapshot digest** |
| dashboard snapshot | `3e50f9a45be987cfbb88f17e5dcd311d11d87d13234481d5d43aa0bfde88cd9c` | `709b4359af54cbb4515779d1a32e4b66273c445bf6b744996d050848b44a754e` |
| `inspect_engineering_artifacts.py` findings | 20 — error 0, warning 7, info 13 | **identical** |
| `W-HEX-001` / `W-HEX-003` / `I-REV-001` observations | 2 / 5 / 13 | **2 / 5 / 13, unchanged** |
| Decision required | **1** — `VREC-MOK-013` `[ready]` assurance-review | **0 — none** |
| Definitions pending | 1 — `WO-MOK-008` `[draft]` | 1, unchanged |
| Active work | 0 — none | 0, unchanged |
| Assurance pending | 0 | 0 |
| Suggested next steps | 9 | **8** |
| `preflight --work-order WO-MOK-013 --phase review` | PASS, exit 0 | **PASS, exit 0** |

**The transition answers exactly the signal it was asked for and changes nothing else.** The `decision_required` queue
empties from its one entry to none, *Suggested next steps* drops with it from 9 to 8, and no error count, no warning
count and no finding total moves. The snapshot digest moves because the front matter moved and because this file adds a
path to the evidence index; it does **not** move for the prose rewritten in the record, since what is hashed is the
normalized front matter, the relations and the findings. **That was checked rather than assumed**: the `after` digest was
measured before this figure was written into this table, and re-measured with it written in. Both readings are
`709b4359…`, because this file's path entered the index the moment the file existed and its bytes never enter the digest.

**`assurance_pending` stays empty for a different reason than before, and the queue does not distinguish them.** It
holds an `implemented` work order that no verification record at `ready` or beyond covers. Before, `WO-MOK-013` was
covered by a `ready` record; now by a `verified` one. Neither is in the queue, and the transition is invisible there.

**`updated` did not move and introduced no `W-HEX-003` observation.** It already read `2026-08-20`, the day of both the
capture and the decision, so the five date observations are the same five against the same artifacts before and after.

## Four things reported rather than repaired

1. **A retained transcript is no longer reproducible at this tree, and it is not edited.**
   `WO-MOK-013-review-gate.md:189` records `Decision required (1): VREC-MOK-013 [ready] assurance-review` inside a gate
   transcript. That was a faithful capture of the run that produced it and it will now print `Decision required (0)`. A
   generated report that has been hand-corrected is no longer generated, which is the rule this packet already applies
   to its two corrected harness readings and its three snapshot figures.
2. **`W-HEX-001`'s two observations are other chains' and are untouched here.** They name `WO-MOK-010` and
   `WO-MOK-011`, whose evidence is retained in directories rather than under filenames beginning with the work-order
   identifier, which is what evidence discovery keys on. Retaining evidence keyed to those work orders is the
   engineering owner's act on them. This branch neither causes nor clears either.
3. **Two of `W-HEX-003`'s five observations are this change's own doing and the transition clears neither.** They name
   `VER-MOK-005` and `VER-MOK-008` through `REQ-MOK-026` and `REQ-MOK-036`, whose `updated` dates moved to 2026-08-20
   ahead of their unamended contracts. At `ff3a155` `W-HEX-003` reported five observations too, but an entirely disjoint
   five, tabulated in `WO-MOK-013-completion-summary.md:87-93`. The equal count was the trap, and `VREC-MOK-013`
   corrected a sentence of its own that fell into it. Reassessing those two contracts is item 1 of *What the decision
   does not retire*.
4. **Pull request #33's checks are green against a base that has moved.** Five successful check runs, all computed at
   `baseRefOid` `ff3a155`. Nothing is re-run here, and the local `git merge-tree` re-derivation is recorded above
   instead of trusting the cached verdict.

## What this commit does

Prose and one front-matter field. It moves `VREC-MOK-013`'s `status` from `ready` to `verified`, rewrites the parts of
that record which would otherwise assert a lifecycle state it no longer has — the heading, the candidate paragraph now
kept as a quotation, the two tail sections, and three later-fact notes where the pushed history overtook a statement —
and adds this file. **No measurement, digest, capture, oracle result or judgement in the retained evidence changed.** No
Rust source, manifest, lockfile, workflow or script is touched, so no code figure is this commit's to move. Nothing is
renumbered, nothing is merged, no work order moves, no tag and no release is taken.

Every command behind every figure in this file is offline, reads no credential, secret, token or environment value, and
none appears in this file or in any retained evidence.
