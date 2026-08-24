+++
id = "VREC-MOK-024"
type = "verification_record"
title = "Verification candidate for WO-MOK-025"
status = "ready"
owners = ["assurance owner"]
created = "2026-08-24"
updated = "2026-08-24"
commit = "b0c18b8078aa7f26b645ce88140b6d3a152bbd65"
git_object_format = "sha1"
worktree_state = "clean"
verified_at = "2026-08-24T19:27:13Z"
artifact_snapshot_sha256 = "2263fd8b53642779d9083c4f180fd0eb6a6c4077e55764175cf8e222414b0829"
evidence_paths = ["docs/engineering/simulation/evidence/WO-MOK-025/README.md", "docs/engineering/simulation/evidence/WO-MOK-025/analysis/architecture-checks.py", "docs/engineering/simulation/evidence/WO-MOK-025/analysis/lending-cursor.py", "docs/engineering/simulation/evidence/WO-MOK-025/analysis/observer-screen.py", "docs/engineering/simulation/evidence/WO-MOK-025/analysis/request-layout.py", "docs/engineering/simulation/evidence/WO-MOK-025/analysis/static-checks.py", "docs/engineering/simulation/evidence/WO-MOK-025/base/entropy-instrument.patch", "docs/engineering/simulation/evidence/WO-MOK-025/base/entropy-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/full/seed42-baseline-traceoff.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/full/seed42-baseline-traceon.jsonl", "docs/engineering/simulation/evidence/WO-MOK-025/base/full/seed42-individual-traceoff.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/full/seed42-reference-traceoff.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/full/seed42-social-traceoff.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/gates.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/nosink-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/reproduction.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/schema-divergence.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/sink-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-025/base/wo-019-comparison.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/architecture-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/declared-dependencies.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/entropy-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/gates.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/nosink-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/observer-screen.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/per-tick-lending.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/public-surface.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/replay-identity.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/req-068-comparison.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/request-layout.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/schema-digit.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/sink-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/static-checks.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/transcript-reading.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/verification-cases.txt", "docs/engineering/simulation/evidence/WO-MOK-025/candidate/verify-schema-digit.sh", "docs/engineering/simulation/evidence/WO-MOK-025/capture.sh", "docs/engineering/simulation/evidence/WO-MOK-025/completion-report.md", "docs/engineering/simulation/evidence/WO-MOK-025/credential-attestation.md", "docs/engineering/simulation/evidence/WO-MOK-025/entropy-manifest.sh", "docs/engineering/simulation/evidence/WO-MOK-025/manifest.sh", "docs/engineering/simulation/evidence/WO-MOK-025/manual-assessment.md", "docs/engineering/simulation/evidence/WO-MOK-025/ratification/README.md", "docs/engineering/simulation/evidence/WO-MOK-025/ratification/increment-confinement.txt", "docs/engineering/simulation/evidence/WO-MOK-025/ratification/sink-manifest.txt", "docs/engineering/simulation/evidence/WO-MOK-025/ratification/verify-increment.sh"]

[relations]
verifies_work_order = ["WO-MOK-025"]
conforms_to = ["VER-MOK-018"]
+++

# Verification Record Candidate

This ready record binds retained evidence for `WO-MOK-025` to candidate commit `b0c18b8078aa7f26b645ce88140b6d3a152bbd65`. An accountable assurance owner must review the evidence and transition the record to `verified`; this command did not approve, commit, tag, release, or publish anything.

The record is intentionally created after the candidate commit it names, avoiding self-referential commit metadata.

## 1. What this record binds

Retained evidence for `WO-MOK-025` — the decision port, the request, the transcript and the replay, with no
provider, no credential read, no network access and no process spawn — bound to a clean candidate commit,
against `VER-MOK-018`. The packet is **46 tracked files, 5,251,984 bytes**, under
`docs/engineering/simulation/evidence/WO-MOK-025/`: a base-commit capture set, a candidate capture set, five
analysis programs, three capture scripts, a ratification directory, the two records of the owner acts of
2026-08-24, a `README.md` that binds each capture to the commit it was taken at, and a completion report.
Every file is `LF` and none contains a `CR`, which matters because `.gitattributes` marks this directory
`-text`: evidence bytes are hashed as written.

**The commit this record binds is not the commit the measurements were taken at, and the gap is measured
rather than asserted.** The captures were taken at `77f2974` and re-taken at `dbc9e6d`, which changed
behaviour rather than a comment; `4cfb297` re-established the packet at `dbc9e6d`. **Not one byte outside
`docs/` changed between `dbc9e6d` and the commit named above** — measured as an empty diff excluding that
directory — so every figure in the packet describes this commit's code and tests as well as the commit it was
measured at. Inside `docs/`, three commits followed `4cfb297`: `bf027c8` amended `VER-MOK-018`, `cef8842`
retained the three owner acts of 2026-08-24 in the packet, and the commit named above moved `WO-MOK-025` to
`implemented`. The last of those is deliberately alone, so that this record does not bind the commit
performing the transition its own capture depends on.

**Two things about the snapshot digest, so a later reader does not read a mismatch as tampering.** It covers
evidence **paths** rather than evidence bytes, so a declared or prefixed path moves it; and it also moves with
`HEAD`, with the checkout directory's basename and with clone depth. A digest recomputed in a different
checkout will differ, and that is not a defect. The bytes are covered by the packet's own manifests, named
below.

**There is no `MANIFEST.sha256` in this packet.** `WO-MOK-020`'s had one; neither `WO-MOK-025` nor
`VER-MOK-018` asks for one, and `capture-verification` does not need one. The packet's integrity claims live in
`candidate/nosink-manifest.txt`, `candidate/sink-manifest.txt`, `candidate/entropy-manifest.txt` and their
base-commit counterparts — digests of captured streams rather than of the packet, which is what the contract's
cases cite.

`conforms_to` holds exactly one contract because `WO-MOK-025` declares exactly one: *Required verification*
names `VER-MOK-018` and no other.

## 2. Two captures replaced rather than annotated

A reader of this record should know it before opening them. `candidate/static-checks.txt` and
`candidate/architecture-checks.txt` print source line numbers, and the engine binary grew from 211 lines to
226 at `dbc9e6d`. Each was re-run there and compared line for line against the file it replaced — **30
differing lines each, one being the commit named and 29 digit-only, with no finding, verdict or sentence
changed.** That is a disclosed departure from this packet's rule that a capture is not edited after the fact;
`README.md` and `candidate/gates.txt` both carry it.

**Every reading in the packet taken before `dbc9e6d` was taken on Windows and none of them said so, and two
were true of Windows only.** That is disclosed rather than corrected silently, and it is the reason the
`dbc9e6d` defect existed: `fs::File::open` on a directory succeeds on Linux and refuses only at the first
read, so a directory named as a transcript began a run there. Both hosts are now measured on both platforms.

## 3. The manual assessments — all three accounted for

`VER-MOK-018` requires three, and its own words are that "Three, and each is here because no check can make
it." **One person holds all three governance roles in this repository**, which is stated rather than left to
be inferred: no assessment is answered by implication from another.

| # | Role | Assessment | State |
|---|---|---|---|
| **M1** | assurance owner | the shared rules block carries no strategy, read against `SPEC-MOK-007` rule 4.4's prohibitions and against `SPEC-MOK-001` for accuracy | **RECORDED 2026-08-24**, over `4cfb297` — *"Met — it carries no strategy"*. `manual-assessment.md` section 1 |
| **M2** | assurance owner | the shared rules block agrees with `SPEC-MOK-001` | **RECORDED 2026-08-24** — *"Met — it agrees"*. `manual-assessment.md` section 2, with a claim-by-claim cross-check behind it |
| **M3** | assurance owner | the published comparison is honest | **NOT THIS RECORD'S.** It belongs to case `L24`, which needs authorised live runs. It is `WO-MOK-027`'s |

`manual-assessment.md` quotes the block in full — 5,381 characters, 5,385 bytes, 90 concatenated literals,
sha256 `39baca0f8665b17a519eb0b315443c667bdc1d6f5e9c90f343744303915a646a` — and the quotation is read back out
of its own fence and re-hashed to the same digest, so a later reader can settle its fidelity without trusting
the file's prose. **Neither section carries a recommendation, and that omission is deliberate:**
`DECISION_RIGHTS.md` bars an implementation agent from self-approving an assessment, and recommending an
outcome would be that by the back door. Each section states the case against as well as the case for.

**One finding in that record is worth a reader's attention before this decision is taken.** The one sentence in
the block that comes closest to a value statement, *"Nothing raises health."*, is also the one claim in M2's
cross-check whose agreement with `SPEC-MOK-001` rests on the **absence** of a provision rather than on one. It
is shown by an exhaustive reading of every provision that writes `health` — initialization to `100`, rule 12's
`-5` decay, rule 22's damage, with rule 13 marking death at zero — and by the food table having two columns
rather than three. An amendment that added a way to raise health would falsify that sentence without
contradicting any rule of `SPEC-MOK-001` the block quotes, so no mechanical check would catch it.

**C6's attestation is recorded too**, in `credential-attestation.md`: *"Attest — none is configured"*, made
2026-08-24 by the repository owner over `4cfb297`, with a corroborating measurement of **0** secrets and **0**
variables at repository Actions, at Dependabot, and at each of the two environments, and no organization scope
because the account is personal. The record states three limits — names are enumerable and values are not, no
measurement covers a moment other than its own, and a reading of a live remote surface is not re-derivable by
a later reader as it was taken — **so the measurement corroborates the attestation and does not make it.**

## 4. The case accounting — 68 rows, 60 pass

`candidate/verification-cases.txt` is the account, one row per case and per required half-case, under the
required list's own governing sentence: *"A case that cannot be run is escalated, not omitted."* Its own
`RESULT` block reads 67 rows and 59 pass at the reading it names, and is left as written; three amendment
blocks at its head carry what moved. **`L34` was added to `VER-MOK-018` on 2026-08-24 and passes on evidence
that already existed, so the account is 68 rows and 60 pass**; `M1` and `M2` are recorded, and with `M1` so is
`L27`; and `C6` is attested.

The eight rows that are not a plain green:

| Row | Case | State | Note |
|---|---|---|---|
| **L5** | the enumeration is not the core list | PASS, AND ESCALATED | **E8** ruled: the case is restricted to the requests that enumerate a targeted action, and the check prints both figures — 104 of 221 enumerating a targeted action, 117 of 221 with a set equal to the core list, the difference being exactly the requests with nothing to target |
| **L16** | every exchange retained | PASS IN PART | "A retried exchange appears as two records" is not exercised: a retry needs transport retry, which this stage does not build |
| **L17** | the transcript's constraints | PASS IN PART | **E9** ruled: the closed-alphabet clause is withdrawn and replaced by a round trip through the transcript's escaping function. The three surviving clauses — no floating-point value, no timestamp, no path — are unchanged and hold |
| **L30** | the port is lent, not rebuilt | PASS, figure disclosed | **E10** ruled: the illustrative "cost of two exchanges" is withdrawn. The ceiling must be reached in a later tick and not the first; at this configuration it is **eighteen**, one and a half ticks |
| **L27 / M1** | the prompt carries no strategy | **RECORDED 2026-08-24** | one assessment seen from two sections. Section 3 above; `manual-assessment.md` |
| **M2** | the block agrees with `SPEC-MOK-001` | **RECORDED 2026-08-24** | section 3 above; `manual-assessment.md` |
| **S2** | the connector's dependency surface | N/A | no connector exists, canned or otherwise, so there is no surface to check. The required list states this exclusion |
| **C1** | no credential in any produced byte | PASS IN PART | "A test that sets a synthetic credential value and asserts it appears in no produced byte" cannot be run, because no code path reads a credential. Reported as a FINDING by the instrument itself |

**`L34`, added the same day, is a plain pass** and is satisfied by evidence measured on Windows and on Linux at
`dbc9e6d`, none of it taken after the ruling: the directory case in `mokiterions-core/tests/replay.rs`, whose
assertion is now the missing file's case exactly, and `candidate/replay-identity.txt`'s case `R3c` and row
`O6`.

**Two reader traps the packet discloses rather than leaves to be walked into.** The three `#[ignore]`d tests
are named with their line numbers, because a case whose evidence is an ignored test is a false green that looks
like a real one; only one is cited under any row, for `L9`'s entropy half, and that row says it does not run in
the default suite. And `candidate/replay-identity.txt` numbers its own rows `R1` to `R7`, which are that file's
local labels for seven mismatched configurations and are **not** `VER-MOK-018`'s resilience checks `R1` to
`R5`; every row that cites the file says which it means.

## 5. What was not verified, and why — ten entries

Every one is a provider call, an option this stage does not add, or an owner attestation. **None is a check
that could have been run here and was not.**

| Case | Why not |
|---|---|
| **L15b** | the cached-prompt-token ratio over an authorised live run of at least 200 exchanges — needs a provider call and an authorisation |
| **L24** | the published comparison of survivors, deaths and combat deaths — needs live runs at the declared seeds and horizon. Its honesty assessment `M3` is `WO-MOK-027`'s |
| **L25** | that only fit runs are published — needs the runs `L24` needs |
| **L28** | the retained authorization record for a live run — there is no live run to authorise at this stage, and whether an authorisation is genuine is an attestation rather than a check |
| **C6** | the credential is not configured in the repository's automation secrets — **attested by the repository owner and retained**, not checked. `scripts/check_workflow_credentials.py` says so in its own output: "NOT CHECKABLE HERE" |
| **L20**, live half | a live-mode selection with no credential present — needs the live-mode flag |
| **L32**, three halves | the connector-path, live-mode and ceiling cases — need options this stage does not add |
| **R1 and R2** | transport retry, and with them `L16`'s retried-exchange clause — this stage builds no transport |
| **A4** | the money runs out — needs a declared ceiling, which needs the option that declares one |
| **A7**, refusal half | needs the connector-path option |

Three structural gaps about the required list and about coverage rather than about a case being owner-gated.
**Two are recorded and one is closed:**

- **The required list's enumerated matrix omitted `L20` and `L32`** while its prose brought each in by half.
  That was **E12**, settled 2026-08-24: the enumeration now names both *(in part)*, and *Required
  verification*'s opening sentence names the connector-path, live-mode **and** ceiling halves of `L32`.
  **Closed.**
- **The observer's path through the real `ReplayPort` over a real file has no automated test.** It is exercised
  by this packet's captures alone; `L31` uses a scripted stub for the reason its own header gives, and that
  reason is sound. The consequence is still a gap and is recorded as one — and the defect of `dbc9e6d` is what
  that gap costs, because the observer's half of it was the worse failure of the two and had no test of any
  kind. **Recorded, not closed.**
- **`VER-MOK-018` had no case for a transcript the platform refuses.** That was **E19**, and it is now
  settled: the owner ruled on 2026-08-24 that the required list gains the case, and the alternative —
  recording the gap and deferring it to `WO-MOK-026` — was **declined**. `L34` is that case. It is worth
  recording plainly that **running the required list in full before that ruling would not have found the
  defect `dbc9e6d` fixes.** **Closed.**

## 6. The twenty escalations

Seven were resolved as they arose. Eleven were put to the owner in one pass on 2026-08-24 and **all eleven were
ruled in the turn the question was asked.** **`E19` and `E20` were raised after those rulings and were ruled
later the same day, so all twenty are settled** and none is carried into this decision as an open question.

| # | What was raised | Resolution |
|---|---|---|
| **E1** | `SPEC-MOK-006`'s 2026-08-21 amendment row was still OUTSTANDING when this work needed it | Ratified in the same act that approved the 2026-08-23 row. This also closes `VER-MOK-018`'s residual condition that a live measurement might retain a `schema` value whose specification is unratified |
| **E2** | `INT-MOK-001`'s determinism success measure is the product owner's | *"Amend both now"* — the measure and the matching desired outcome |
| **E3** | `SPEC-MOK-003`, three provisions plus four locations | *"Amend all three now"* |
| **E4** | `ARCH-MOK-002`, five clauses | *"Amend all five now"* |
| **E5** | `SPEC-MOK-007` rule 11 in five places, every one found by building the transcript and measuring it | *"Write into `SPEC-MOK-007` now"* |
| **E6** | `SPEC-MOK-004` rule 11 listed `#[ignore]` among what no tier may use, which the instrument tests need | *"Amend rule 11 to admit instruments"* |
| **E7** | a defect of the implementation agent's own: `validate` reported FAIL on a rationale over the validator's 2,000-character limit | Re-expressed within the limit and **reported rather than presented as authorized** |
| **E8** | case `L5`'s literal wording is wider than the check the program enforces | Ruled: restrict the case to the requests that enumerate a targeted action; print both figures |
| **E9** | `VER-MOK-018` still stated the closed-alphabet clause that rule 11.4.1 withdrew the same day | Ruled: withdraw the clause, substitute the escaping round trip |
| **E10** | `L30`'s illustrative ceiling of "the cost of two exchanges" defeats the discriminator the case exists to be | Ruled: withdraw the figure, derive the ceiling from the tick's arity, state it with the run — **eighteen** here |
| **E11** | rule 11.4.1's character list is wrong in two of five entries | Ruled: correct it to the measured census. **The only ruling that reaches source** — both restatements in `mokiterions-core/src/simulation.rs` |
| **E12** | the required list's enumerated matrix omits `L20` and `L32` | Ruled: name both *(in part)* and correct the opening sentence. Written into `WO-MOK-025`'s *Lifecycle*, because a work order carries no amendment record |
| **E13** | four added public items are outside every list `SPEC-MOK-002` rule 5 carries | Ruled: rule 5's additions list gains them; the two-crate construction of `ReplayPort` is stated |
| **E14** | rule 5 property (b)'s `&'static str` carve-out is stale | Ruled — **and the recommendation was wrong in mechanism.** It named `EventType::as_str`, which the `'static` clause already carves out; the six references actually remaining are `DecisionRequest`'s accessors. The owner approved the correction the measurement supports. The conclusion did not move; the mechanism did, and `SPEC-MOK-002`'s amendment row says so |
| **E15** | `REQ-MOK-077` required response 3's refusal message | Ruled: **deferred to `WO-MOK-026` and recorded as untriggered, not met.** The three option names do not exist yet, so `cli.rs:272`'s generic `unknown option:` fallthrough is what the observer forwards to. The alternative — adding the message now — was **declined** |
| **E16** | two orphaned `SPEC-MOK-001` amendment rows outside their own table | Ruled: a separate `docs(spec)` commit **outside this work order**. Landed as `db8cf46`, carrying no work-order trailer and saying why |
| **E17** | `check_transcript_reading.py` was wired into nothing | Ruled: wire it into `provider-credentials.yml`. Recorded as an **addition** to in-scope item 11, not a correction, because it is one |
| **E18** | `REPOSITORY_CONTEXT.md`'s amendment, which `ADR-MOK-007` requires | Ruled: **draft it as a diff for the owner to approve or replace.** Drafted and held outside the repository. Drafting is not approving; it remains the repository owner's act, and it is the one act still outstanding |
| **E19** | `VER-MOK-018` had no case for a transcript the platform refuses — raised at `dbc9e6d`, out of the defect continuous integration found | **Ruled 2026-08-24 over `4cfb297`: add the case.** `L34`, under `REQ-MOK-077`, requiring both hosts to exit `1` with empty standard output and the host's own message prefix, exercised on more than one platform. Deferral to `WO-MOK-026` was put with it and **declined**. `bf027c8` wrote it |
| **E20** | `VER-MOK-018`'s `C6` claimed "no check can see this", which is stronger than the truth — secret **names** are enumerable at every scope a workflow can read one from | **Ruled 2026-08-24: amend the wording.** The clause is withdrawn; the bullet now says the measurement corroborates the attestation rather than replaces it, because a **value** is what no check can see and a moment other than the one measured is what no measurement covers. **The attestation itself is unchanged in force.** `bf027c8` wrote it |

## 7. What a green build does not establish here

Recorded because `VER-MOK-018` is explicit that this contract is in a weaker position than any previous one in
this repository, and that the weakness follows from the cost decision rather than from a shortcut.

- **Five cases cannot be satisfied by a build** — `L15b`, `L24`, `L25`, `L27` with `M2` and `M3` beside it, and
  `L28`'s genuineness half. `L21a` *is* fully checkable, but **C6**'s attestation behind it is not, and that
  attestation is the fact the whole cost containment rests on. It is outside the matrix because it is outside
  the repository.
- **`L4`, `L5`, `L12`, `L13` and `L14` are only as good as the transcript they read.** They check the requests
  one run produced. A code path composing a different request under a configuration no retained transcript
  covers is not reached. The committed transcript covers a run in which every Mokiterion acts, targeted
  actions are enumerated, food is and is not co-located, and at least one Mokiterion dies — a mitigation, not
  a proof.
- **No case establishes that the model understood anything.** A well-formed enumerated response is the whole of
  what is checkable. `VER-MOK-018` has no outcome oracle, deliberately.
- **The provider can change under us.** `L15b` failing against a layout that was correct when written is the
  intended behaviour — a signal to re-measure and bring the layout or the floor back to the owner, not a reason
  to soften the number in place.

## 8. What this candidate does not do

- **It does not verify.** `status` is `ready`. The verification decision and the transition to `verified` are
  the accountable assurance owner's act. That transition **moves only `status`**: the title, the provenance and
  this candidate's prose stay as `capture-verification` wrote and as this record states them, and the decision
  goes in a new opening section. It is also the last chance to correct any figure here, because a verified
  record can never be corrected and has no rebind.
- **It does not release, tag or merge.** Those are separate owner acts and none has been taken.
- **It does not close `E18`.** `REPOSITORY_CONTEXT.md`'s amendment, which `ADR-MOK-007` requires, is drafted
  outside the repository and not applied. It is the repository owner's act and the only one of the twenty
  escalations whose written outcome is still owed.
- **It does not assert `M3`, `L24`, `L25`, `L15b` or `L28`.** They need an authorised live run, which does not
  exist at this stage and which the standing instruction places behind the repository owner's explicit
  permission.
