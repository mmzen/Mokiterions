# Assurance decision — `VREC-MOK-015` moved from `ready` to `verified`

**This file is not in `VREC-MOK-015`'s `evidence_paths`, and that is deliberate.** A record's evidence paths name what
the accountable owner accepted, and all twenty-two it names existed at or before the candidate commit
`9599c0a91bb2b6e183bce3a5e82b570d547594f8`. This file postdates that commit and is the reasoning behind the status
rather than evidence the status accepts. Adding it would also move `artifact_snapshot_sha256`: the evidence path index
is inside the hashed document, so a decision that listed itself would change the digest of the graph it claims to have
reviewed. `evidence/WO-MOK-011/assurance-decision.md` set that precedent and `evidence/WO-MOK-014/assurance-decision.md`
followed it for the record this one sits beside.

**It takes no decision itself. It records one.**

It is in `merge/` rather than in the directory above because `evidence/WO-MOK-014/assurance-decision.md` is already
taken by the decision on `VREC-MOK-014`, and because this record's evidence is this directory. One work order now holds
two decisions, one per record, each next to the evidence it was taken on.

## The instruction

> you can transition VREC-MOK-015 to verified, i validate, and commit / push

`DECISION_RIGHTS.md:14` states that *"only accountable assurance and release owners may transition those records to
`verified` or `released`; record preparation never creates commits, tags, or publications"*. The repository owner holds
that role, as they hold the technical, product, engineering and release roles, so **nothing here is approved by
implication** — the instruction named one artifact and three acts, and this file is the record of exactly those.

**This instruction names the record.** The previous one in this chain read *"i validate VER-MOK-014, you can transition
it, and commit + push"* and named the contract, which had to be read across to the record;
`evidence/WO-MOK-014/assurance-decision.md` records that reading at length. Nothing of the kind was needed here:
`VREC-MOK-015` is a verification record, it was the repository's only artifact at `ready`, and it was the sole entry in
the inspector's `decision_required` queue — the queue an assurance validation answers. *"i validate"* follows the
transition clause rather than preceding it, which changes the order of the words and not the acts.

**`WO-MOK-014` was not moved.** No instruction named it, it stays at `implemented`, and `WORKFLOW.md` is explicit that a
work order's status never substitutes for a verification record's. The work order is now covered by two `verified`
records bound to two commits.

## What the decision accepted

In the `ready` record's own words, which the transition took as they stood: the evidence enumerated in `evidence_paths`
*"as satisfying `VER-MOK-014` at commit `9599c0a91bb2b6e183bce3a5e82b570d547594f8`, and nothing beyond it"*. What that
evidence is — the five oracles re-derived at the merge commit rather than argued forward from a diff, the ten-way
refusal demonstration, the engine built and tested offline behind a closed port, the compiled gates at 226 tests, the
harness five in two columns, and six manual assessments whose subject was shown unchanged — is enumerated in the record
and in `README.md` beside this file, and is not restated here.

**No new judgement was required, and the one this decision rests on was given before the previous transition.**
`VER-MOK-014:219` states that an unrecorded assessment is an outstanding assessment and that the contract is not
satisfied while any remains outstanding, while its amendment row at `:21` states that assessment 7 *"is not yet due"*.
Asked on 2026-08-20 which reading the contract's wording carries, the owner answered that **not yet due is not
outstanding**. That is a reading of the contract, not of a tree, so it applies wherever the contract is applied: at
`9599c0a` assessments 1, 2, 3, 4 and 6 are recorded, 5 does not exist, 7 is not due, nothing is outstanding, and
**`VER-MOK-014` is satisfied at the merge commit** on the same reading that satisfied it at `65ac88b`.

**Assessment 6's limit travels with this acceptance too.** It accepted a compiled and uncalled transitive network
capability — `mio 1.2.2`'s `net` feature, reached through `ratatui` → `ratatui-crossterm` → `crossterm`, in two of three
release builds — and is void the moment a behavior calls it. `WO-MOK-014-scan.txt` in this directory shows the same two
disclosed hits, the same versions and the same two targets at this commit. Verifying this record does not widen that.

## `W-REV-004`: a finding this transition silences without answering

The `ready` record raised one warning that the previous record's transition did not have to face:

> `W-REV-004` (derived): `VREC-MOK-015` is ready but its work is fully covered by verified or released records; review
> possible supersession without inferring authority. `[VREC-MOK-014, VREC-MOK-015]`
>
> `W-REV-004` → `review-verification-supersession` (assurance-owner): Assess explicit supersession against one eligible
> verified or released successor; do not transition automatically.

**It is gone from the graph after this transition, and it was not answered.** Its predicate is `ready`; the record is
now `verified`, so the finding stops being raised. No supersession was assessed, none was taken, and neither the record
nor this file rules on the question the finding put.

Three things follow, and all three are disclosed rather than left to be discovered:

1. **The finding was reported to the owner before the instruction was given.** It was measured when the `ready` record
   was captured, written into the record, into `README.md` and into `harness.txt` in this directory, and reported in
   words to the owner as a second act owed to the same role. The instruction to transition came after that report.
2. **This transition forecloses the supersession route for `VREC-MOK-015` itself.**
   `VERIFICATION_RECORD.template.md:30` permits a separate governance decision to move only a **`ready`** record to
   `superseded` and states no route for a `verified` one. Both records are now `verified`, so neither can be superseded
   by the other under the template as written. That is the same exposure `evidence/WO-MOK-014/assurance-decision.md`
   priced for the renumbering, arising a second time from the same clause.
3. **What stands in its place is stated, not measured.** `VREC-MOK-014` binds `65ac88b` and `VREC-MOK-015` binds
   `9599c0a`; the second is not the first re-pointed, the two commits differ in content, and each record carries the
   evidence measured at its own. One work order with two `verified` records is the outcome, and the graph accepts it: no
   finding replaced `W-REV-004` and no error or queue entry appeared.

## What the decision does not retire

1. **`VER-MOK-011`'s fifth manual assessment stays owed** — the assurance owner's, outstanding since 2026-08-19. It
   belongs to another contract, no measurement in this packet reaches it, and `README.md` beside this file records it as
   an untouched residual rather than as inherited noise.
2. **`VER-MOK-014`'s assessment 7 is not discharged. It was found not to be due.** The first admission of a crate
   beyond `ratatui` owes it by name, together with assessments 1 and 2 for that crate and the seven-item evidence set
   the contract's *What an admission must retain* fixes. This merge admits none.
3. **`VER-MOK-005` and `VER-MOK-008` stay owed**, at the depth and in the order the owner set on 2026-08-20: those two
   contracts plus `REL-MOK-001`, rewording the four sites that assert the withdrawn empty-dependency property, in a
   separate work order taken up after pull request #33 merges. The `W-HEX-003` observations that name them are unmoved
   by this transition — six before and six after — and `master` has since amended `VER-MOK-005`, so the coordinates must
   be re-derived against the merged tree before the repair is attempted.
4. **A verification record for `WO-MOK-011`'s merge is still owed to that chain.** `evidence/WO-MOK-011/merge/README.md`
   recorded on 2026-08-19 that *"a record for the merge is a new record"* and none followed. `VREC-MOK-015` is that
   record for **this** merge only.
5. **The release.** `REL-MOK-001` is `approved` and closed by its own approval; `RLS-MOK-001` released 0.1.0 from
   `755db72`, which does not include this work. **No release record binds this work and none is created here.**
   `REL-MOK-001:94-96` still states the one-crate resolution `ADR-MOK-006` withdrew and is deliberately not amended,
   because amending a closed contract whose release has been performed would rewrite a discharged decision.
6. **Pull request #33.** It is `OPEN`, `MERGEABLE` and not a draft, and its five checks are green at head `13718cc` —
   but **verification is not merge**. Merging is the engineering owner's separate act. Four statements in the body are
   overtaken by the commits made after it was last edited, the body still carries the pre-renumbering identifiers
   throughout, and it does not mention `VREC-MOK-015` at all because it predates it. It is not edited here.

## Measured before and after

Both readings were taken in this clone with `HEAD` at `13718ccd3c6fd808c14d6f061f4fad823c8e4a1a` and the artifact
content as the only moving input, which is the only way a snapshot digest pair is comparable: `build_snapshot` puts
`git rev-parse HEAD` into the hashed document twenty-five times and the checkout directory's name once, so the same
content measured after this commit exists will hash to something else again. **No digest is given here for the commit
that carries this file, and none can be.** The digest `VREC-MOK-015` declares, `b3470146…`, is neither of the figures
below: it is the digest of the candidate `9599c0a`, measured in a clean detached worktree, and this transition does not
touch it.

| Reading | Before, `status = "ready"` | After, `status = "verified"` |
|---|---|---|
| `validate_engineering_artifacts.py` | PASS — 115 artifacts, 0 errors, 0 warnings across four planes | **identical** |
| `generate_harness_dashboard.py` | PASS — 115 artifacts, 398 relations, 0 errors, **11** warnings | PASS — 115, 398, 0 errors, **10** warnings |
| dashboard snapshot | `95d7b193d75ea2f9fa4e2a90afff6b45016c783da9bdb7c3412c27aa7a4dc840` | `3592f066fd19621b9bca4f6eeee1f72ee39654ff8c15f5c4bccf5b829c66b7da` |
| `inspect_engineering_artifacts.py` findings | 26 — error 0, warning 11, info 15 | **25 — error 0, warning 10, info 15** |
| `W-REV-004` | 1 observation, `[VREC-MOK-014, VREC-MOK-015]` | **absent** |
| `W-HEX-001` / `W-HEX-003` / `I-REV-001` observations | 4 / 6 / 15 | **4 / 6 / 15, unchanged** |
| Decision required | **1** — `VREC-MOK-015` `[ready]` assurance-review | **0 — none** |
| Definitions pending | 1 — `WO-MOK-008` `[draft]` | 1, unchanged |
| Active work | 0 — none | 0, unchanged |
| Assurance pending | 0 — none | 0, unchanged |
| Suggested next steps | 13 | **11** |
| `preflight --work-order WO-MOK-014 --phase review` | PASS, exit 0, `WO-MOK-014 (implemented)` | **PASS, exit 0, unchanged** |
| `doctor` under the pinned 0.4.0 venv | 81 PASS, 0 FAIL, exit 0 | **81 PASS, 0 FAIL, exit 0** |
| `unittest discover -s scripts` | `Ran 126 tests`, `OK` | **`Ran 126 tests`, `OK`** |

**Where this differs from the previous transition, and it matters.** `VREC-MOK-014`'s transition moved the
`decision_required` queue from 1 to 0 and *"no error count, no warning count and no finding total"* with it. This one
moves a warning count as well, 11 to 10, and the finding total 26 to 25, because `W-REV-004`'s predicate is the status
that changed. Two of the two next-step entries that disappear are that finding's and the emptied queue's. A reader who
expects a status change to be invisible in the warning count would be right about the previous record and wrong here.

**The snapshot digest moves for two reasons and not for a third.** `status` is inside the normalized front matter, and
the findings list is hashed too, so dropping `W-REV-004` moves the digest as well. It does **not** move for the prose
rewritten in the record — the heading, the kept quotation of what the candidate said, the later-fact note, the two tail
sections — because artifact prose is not hashed. That was checked rather than assumed: the `after` digest was measured
before this file existed, and re-measured with this file written and this figure inside it. Both readings are
`3592f066…`.

**There is no third figure, and that was measured rather than assumed.** With this file present and untracked the
dashboard reads `3592f066…` again at the same `HEAD`, over the same 115 artifacts and 398 relations. The hashed document
was then read directly to find out why, because a figure quoted inside the packet it covers is worth nothing on
assertion. Two mechanisms carry evidence into it and this file meets neither:

- **Declared paths.** A record's `evidence_paths` are hashed as the strings they are — `harness.sh` appears once, at
  `$.artifacts[100].evidence_paths[18]` — with no existence check and no content hash beside them. This file is not
  declared, by the choice the first paragraph records.
- **Discovered evidence, keyed on the filename.** The document's `evidence` node lists 34 paths for `WO-MOK-014`, and
  every one of them begins with `WO-MOK-014-`, at either depth: the 23 in the directory above and the 11
  `merge/WO-MOK-014-*` captures beside this file. The prefix is the key. This file, `README.md`, `harness.txt`,
  `harness.sh`, `contract-identity.txt`, `amendments.md` and `determinism.txt` are not in it, and neither is
  `evidence/WO-MOK-014/assurance-decision.md`.

So the digest is unmoved by writing this file, which is what lets its own figure be written inside it. **The reason is
the name, not the subject matter**: had this file been called `WO-MOK-014-assurance-decision.md` it would have entered
the discovered-evidence index and moved the digest without ever being declared anywhere. That is the same keying
`W-HEX-001` reports against four other work orders, seen from the inside.

It also fixes what the held-out pairs elsewhere in this packet actually measure. `WO-MOK-014-merge.md`'s `b3470146…`
against `5011aeb6…` is a **discovered path** appearing or not, since that file carries no front matter and is no
artifact; `VREC-MOK-014`'s own `a12ec1a3…` against `dcb28212…` is an **artifact** appearing or not, which moves the
count as well. Two different mechanisms behind two superficially similar pairs.

**`assurance_pending` stays empty for a different reason than before, and the queue does not distinguish them.** It
holds an `implemented` work order that no verification record at `ready` or beyond covers. Before, `WO-MOK-014` was
covered by one `verified` record and one `ready` one; now by two `verified` ones. The transition is invisible there.

**`updated` did not move and introduced no `W-HEX-003` observation.** It already read `2026-08-20` — the capture date
and the decision date both, the decision taken at `2026-08-20T20:12Z` against a capture timestamped `19:46:51Z` the same
evening — so the six date observations are the same six against the same artifacts before and after.

## Four things reported rather than repaired

1. **Two files this record binds now describe a lifecycle state the record no longer has, and neither is edited.**
   `README.md:123` in this directory lists *"`VREC-MOK-015` from `ready` to `verified`"* as owed, and `harness.txt:111`
   retains an inspector transcript reading `Decision required (1): VREC-MOK-015 [ready] assurance-review`. Both were
   faithful when taken; both are in `VREC-MOK-015`'s `evidence_paths`, and a `verified` record's evidence is not
   rewritten after the decision — the rule `evidence/WO-MOK-012/amendment-ratifications.md` states for `VREC-MOK-005`,
   and the rule that a generated report which has been hand-corrected is no longer generated. This file is where the
   later state is recorded, which is what a decision file is for.
2. **`W-HEX-001`'s four observations are other chains' and are untouched here.** They name `WO-MOK-010`, `WO-MOK-011`,
   `WO-MOK-012` and `WO-MOK-013` — the last being `master`'s homonym, which arrived with the merge — whose evidence is
   retained under names evidence discovery does not key on. Retaining it is the engineering owner's act on those work
   orders. This branch neither causes nor clears any of the four.
3. **Two of `W-HEX-003`'s six observations are this change's own doing and this transition clears neither.** They reach
   `VER-MOK-005` and `VER-MOK-008` through `REQ-MOK-026` and `REQ-MOK-036`, whose `updated` dates moved ahead of their
   unamended contracts. Item 3 of *What the decision does not retire* is where that repair sits.
4. **`W-REV-004` is silenced by the status and not answered**, as the section above records at length. It is listed
   again here because a reader comparing warning counts across this commit will see 11 become 10 and is owed the reason.

## What this commit does

Prose and one front-matter field. It moves `VREC-MOK-015`'s `status` from `ready` to `verified`, rewrites the parts of
that record which would otherwise assert a lifecycle state it no longer has — the heading, the candidate paragraph now
kept as a quotation, the `W-REV-004` paragraph now carrying a later-fact note, the tail list's first item and a new
*Scope of the transition taken* section — and adds this file. **No measurement, digest, capture, oracle result or
judgement in the retained evidence changed**, and no file in `evidence_paths` was touched. No Rust source, manifest,
lockfile, workflow or script is touched, so no code figure is this commit's to move. Nothing is renumbered, nothing is
merged, no work order moves, no tag and no release is taken.

Every command behind every figure in this file is offline, reads no credential, secret, token or environment value, and
none appears in this file or in any retained evidence.
