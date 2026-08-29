# WO-MOK-026 authorization genuineness attestation — `L28`'s second half

`VER-MOK-018` case **L28** has two halves and only one of them is a check. The static half is checkable and is
checked below. The other half the contract states in bold: "**Whether the authorization is genuine is an owner
attestation**, not a check". This file is that attestation.

**State: PROPOSED — awaiting the repository owner's statement.** Everything below the attestation table is the
implementation agent's, and every static figure in it was measured at the candidate. The attestation is the
owner's and is not made here.

| Fact | Value |
|---|---|
| Case | `VER-MOK-018` **L28**, genuineness half, under `REQ-MOK-076` |
| What is to be attested | that `live-run-authorization.md` records an authorization the owner actually gave, on the date it names, with the horizon, seed set and ceiling it names |
| Accountable role | repository owner |
| Date | 2026-08-29 |
| The record attested to | `docs/engineering/simulation/evidence/WO-MOK-026/live-run-authorization.md` |
| Static half | **PASSES** in full — measured below |
| State | **PROPOSED — awaiting the owner's statement** |

## Why this attestation exists at all, and why here

`WO-MOK-025`'s credential attestation closes with the reason: "**It does not extend to `L28`.** Whether an
authorisation for a live run is genuine is a separate owner attestation, there is no live run to authorise at
this stage, and `L28` is listed among what was not verified."

That condition has now changed. There is a live run, it was authorised, and the authorisation is retained. So
the attestation `WO-MOK-025` deferred is due here, and this is the file it was deferred to.

**It is not the same statement as `C6`.** `C6` is about a surface — whether a credential sits in this
repository's automation. This is about a document — whether a record of a decision is a record of a decision
that was made. The two are independent: a genuine authorization would still be a genuine authorization if a
credential were misconfigured, and an ungenuine one would still be ungenuine if the automation were clean.
`WO-MOK-026` item 15 asks for "the two attestations, `C6` and the credential attestation", which names `C6`
twice — `VER-MOK-018`'s *Evidence retention* table calls `C6`'s file "The credential attestation". The item's
count of **two** is right and one of its two names is wrong; the second attestation the contract actually
requires is this one, and the work order's own *Required verification* asks for it four sections earlier:
"**L28** is verified for this stage's one run — its authorization is retained with its evidence". This
discrepancy is recorded rather than resolved silently, and it amends nothing.

## The static half, measured

`L28`'s checkable half has three obligations. All three pass.

**1. The record names the authorizing owner, the date, the horizon, the seed set and the ceiling.** All five
are present as labelled rows:

| Term `L28` requires | What the record says |
|---|---|
| Authorizing owner | the repository owner, holding the product, technical and engineering owner roles |
| Date | 2026-08-29 |
| Horizon | **50 ticks** |
| Seed set | **{0}** — one seed |
| Ceiling | **$2 = 200 cents** |

**2. The run's actual seed, horizon and ceiling fall within it.** Read from the run record in
`live-run-record-stream.txt`, not from prose:

| | Authorized | Run record | Within |
|---|---|---|:-:|
| Seed | {0} | `"seed":0` | yes |
| Horizon | 50 ticks | `"ticks":50`, `"tick_reached":50` | yes |
| Ceiling | 200 cents | `"ceiling_cents":200` | yes |
| Spend | at most 200 cents | `"cost_cents":16` | yes |

The run ended `"ended":"tick_limit"` and not at the ceiling, so `REQ-MOK-071`'s stop was never reached — which
is the outcome the authorization anticipated and not a silent margin.

**3. The record contains no credential and no account identifier.** Measured over the file rather than
asserted. The words "authorization" and "credential" occur 11 and 4 times, which is what a document *about* an
authorization and a credential looks like; what matters is whether a **value** is present, and none is:

| Probe | Result |
|---|---|
| Tokens of 20 or more characters from `[A-Za-z0-9_-]` | **4**, all repository identifiers: `cache_ratio_basis_points`, `live-run-measurements`, `live-run-record-stream` |
| `sk-` followed by an alphanumeric | **0** |
| `org-` followed by an alphanumeric (provider organization identifier) | **0** |
| `user-` followed by an alphanumeric | **0** |
| `bearer`, `api_key`, `apikey`, `api-key`, `password` | **0** each |

The long-token probe is the one that does the work: a credential is an opaque high-entropy string, and looking
for one by that shape catches a value whose *name* nobody thought to search for. It found only three distinct
identifiers, every one of which is a path or a field in this repository.

## What the attestation adds that the measurement cannot

Everything above can be re-derived by a reader from files in this repository. None of it touches the question
`L28` reserves to the owner, and the gap is worth stating precisely, because it is easy to look at a passing
static half and think the case is closed.

**A record can satisfy every static obligation and still be false.** The three checks establish that the
document is well-formed, internally consistent, and consistent with the run. They cannot establish that the
owner said what it reports them as saying. A record written by an implementation agent, naming the owner,
naming a plausible date, and naming terms the run then respected, would pass all three — which is precisely
the failure `REQ-MOK-076` is written against. That requirement's failure behaviour names it: a missing
authorization is to be named rather than "backfilled with a retrospective authorization, which would be a
record of a decision nobody made at the time".

Two facts about how this record was produced bear on the attestation, and are stated as the agent's account
rather than as proof:

- **It was written before the run, not after it.** The record says so in its own second paragraph and gives
  that reason. A reader can corroborate the ordering from this branch's commit history: the authorization
  record was created at `3f838d1` and amended twice at `9bc68d5` and `be7b1b8`, and the run's captures arrived
  three commits after its creation at `a0916b0`. **Both amendments also precede the run**, and both concern the
  same figure: `9bc68d5` replaced a bare **estimated** $0.05 with a disclosure that no artifact here states a
  tariff and that the two available figures disagree, and `be7b1b8` settled it at a projected **5 cents**
  against the 200-cent ceiling. **The projection was too low** — the run cost **16.67 cents**, more than three
  times it. That is recorded here rather than passed over, because it is the ordering that makes it harmless:
  an estimate revised before the bill arrives is a forecast, one revised afterwards is a rationalisation, and
  the ceiling rather than the forecast is what bounded the spend.
- **The four terms were settled by the owner choosing among measured options.** The record's *How the four
  terms were settled* section names the owner's instruction, "ok go WO-MOK-026", and states that the
  selections are the owner's while the framing was the agent's.

**Neither of those is the attestation.** Commit ordering shows when a file was written, not who decided its
contents; and an agent's account of a conversation is an agent's account. Only the owner can say that the
decision the record describes is the decision they took.

## What this file does not establish

- **It does not verify.** `L28` is a case in a verification contract; the verification decision is the
  assurance owner's and belongs in this stage's verification record.
- **It does not extend to `WO-MOK-027`.** The authorization it attests to states in terms that it does not,
  and that stage's five-seed measurement — an **estimated** $5.20 when written, and about **$16.67** on this
  stage's measured tariff — needs its own authorization record and its own attestation.
- **It says nothing about `C6`.** That is the separate attestation in `credential-attestation.md`.
- **It covers one run.** The general obligation that every live run retains an authorization is
  `WO-MOK-027`'s, as this work order's *Required verification* states.
