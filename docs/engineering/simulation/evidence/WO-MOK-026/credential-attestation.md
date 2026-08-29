# WO-MOK-026 credential attestation — `C6`, for the stage that spent money

`VER-MOK-018` check **C6** is the one item in that contract no check inside this repository can make, and it
is the fact the whole cost containment rests on. It is recorded here because the contract says where it
lives: "it is the repository owner's statement, **retained with the evidence**", and *Evidence retention*
lists "The credential attestation | Beside the measurement's evidence | one file". This is that file for this
stage.

**State: PROPOSED — awaiting the repository owner's statement.** The body below is the implementation
agent's, and is complete and measured. The attestation itself is not, and cannot be: it is the owner's
sentence about a surface outside this repository. Nothing in this file asserts that the attestation has been
made.

| Fact | Value |
|---|---|
| Check | `VER-MOK-018` **C6** |
| What is to be attested | no model-provider credential is configured in this repository's automation secrets |
| Accountable role | repository owner |
| Date | 2026-08-29 |
| Commit the attestation is made over | the candidate this packet is bound to |
| Corroborating measurement | **not re-taken at this candidate** — see *The corroborating measurement* below, which states why and what stands in its place |
| State | **PROPOSED — awaiting the owner's statement** |

## Why this attestation is re-made rather than inherited

`WO-MOK-025` carries an attestation of the same check, made 2026-08-24 over commit `4cfb297`, and it would be
easy to treat this stage as covered by it. It is not, and the reason is the whole point of this stage.

**When that attestation was made, no credential existed anywhere in this initiative.** No live run had been
made or authorised, and the file says so in terms: "No live run has been made or authorised at this stage."
The containment it attested was protecting against a hypothetical.

**It is no longer hypothetical.** Under this stage a real provider credential was held by the owner, placed
in a process environment, and spent — 16 cents against a 200-cent ceiling, over 503 exchanges. The
proposition "no credential is configured in this repository's automation secrets" therefore changed in
character without changing in words: on 2026-08-24 it excluded a risk nobody could yet run, and on 2026-08-29
it excludes one that a person demonstrably can. An attestation is a statement about a moment, and this is a
different and materially more load-bearing moment.

That is also why the date and the commit above are this stage's and not `WO-MOK-025`'s. An attestation
inherited across the event it was protecting against would be the weakest kind of record: correct in wording
and empty in force.

## The check, as it stands

> - **C6** The attestation that the credential is not configured in the repository's automation secrets. It
>   is the repository owner's statement, retained with the evidence, and it is the single fact the whole cost
>   containment rests on. **The clause "no check can see this" is withdrawn** *(amended 2026-08-24)*: secret
>   **names** are enumerable through the hosting platform's API at every scope a workflow can read a secret
>   from, and `WO-MOK-025`'s evidence retains that measurement […] What no check can see is a **value**, and
>   what no measurement establishes is the state of that surface at any moment other than the one it was
>   taken at. **So the measurement corroborates the attestation and does not make it** […]

The wording is unchanged since that amendment. This stage does not amend `C6`.

## The corroborating measurement

**It was not re-taken at this candidate, and this file does not pretend otherwise.** The implementation agent
attempted the enumeration and the attempt was refused by the agent's own permission boundary before any
request was made. The refusal is recorded rather than worked around, because a corroborating figure obtained
by circumventing a control would corroborate nothing worth having.

What stands in its place, and its exact weight:

| | |
|---|---|
| Prior reading | 2026-08-24, retained in `WO-MOK-025`'s credential attestation: **0** secrets and **0** variables at every scope a workflow can read one from, across repository Actions secrets, Dependabot secrets, Actions variables, and both environments `github-pages` and `release`, with no organization scope because the account is a **User** account |
| What it covers | the surface as it stood on 2026-08-24 |
| What it does not cover | the surface as it stands on 2026-08-29, which is the date of the run this packet measures and the date of the attestation this file holds |

**The gap is five days and one live run**, and `C6`'s own text is what makes it a real gap rather than a
formality: "what no measurement establishes is the state of that surface at any moment other than the one it
was taken at."

A later reader, or the owner, can take a fresh reading with these five requests. They read secret and
variable **names**, never values, and the second environment list should be re-derived rather than assumed
because a new environment would add a scope:

    gh api repos/mmzen/Mokiterions/actions/secrets     --jq '{count:.total_count, names:[.secrets[].name]}'
    gh api repos/mmzen/Mokiterions/dependabot/secrets  --jq '{count:.total_count, names:[.secrets[].name]}'
    gh api repos/mmzen/Mokiterions/actions/variables   --jq '{count:.total_count, names:[.variables[].name]}'
    gh api repos/mmzen/Mokiterions/environments        --jq '{count:.total_count, names:[.environments[].name]}'
    # then, per environment name returned above:
    gh api repos/mmzen/Mokiterions/environments/NAME/secrets --jq '{count:.total_count, names:[.secrets[].name]}'

**The attestation does not depend on that reading.** `C6` puts the fact in the owner's statement and the
measurement in a supporting role, in that order and deliberately. A fresh zero would strengthen this record;
its absence does not weaken the attestation, only the corroboration.

## What no check inside this repository sees

`scripts/check_workflow_credentials.py` is the gate for `L21a` and `L21b`, and it states its own boundary in
its output rather than leaving a reader to infer it:

    NOT CHECKABLE HERE: whether a provider credential is present in the repository's Actions
      secrets. REQ-MOK-073's *Constraints* records that as the condition this containment rests on,
      and VER-MOK-018 carries it as an owner attestation.

That remains exactly right about the *script*: it reads the repository's workflows and nothing outside them.
This stage adds nothing to what it can see, and adds nothing to any workflow — `WO-MOK-026`'s *Out of scope*
excludes "any credential in any workflow", "any live selection in any workflow" and "any relaxation of
`L21a`", and no workflow file is in this work order's execution scope at all.

## What this attestation is load-bearing for

- **`REQ-MOK-073`** — "WHEN the repository's automated workflows run, THE SYSTEM SHALL make no provider call,
  and no workflow in the repository SHALL reference a model-provider credential." The workflow half is
  checked; the *absence of a credential to reference* is this attestation.
- **`ADR-MOK-001`** — "Future provider credentials must remain outside the engine and repository."
- **`docs/engineering/REPOSITORY_CONTEXT.md`** — "Model-provider credentials and other secrets must remain
  outside the repository and must not be committed."
- **The cost containment, now with a demonstrated price attached.** This stage establishes that a 50-tick run
  at seed 0 costs 16.67 cents and that a 1,000-tick run would cost about **$3.33**. A credential absent from
  automation is what keeps an unauthorised run impossible rather than merely forbidden, and this stage is the
  first at which "impossible" and "forbidden" could have come apart.

## What this file does not establish

- **It does not verify.** `C6` is a check in a verification contract; the verification decision is the
  assurance owner's and belongs in this stage's verification record.
- **It says nothing about a credential held anywhere else.** The credential this stage's run used was held by
  the owner, outside this repository, which is where `ADR-MOK-001` requires one to be. That it appears in no
  produced byte is `C1`'s business and is evidenced separately.
- **It is not a claim about any later state of the automation surface.**
- **It does not extend to `L28`.** Whether this stage's authorisation for its live run is genuine is a
  separate owner attestation, and unlike at `WO-MOK-025` there now *is* a live run to authorise. It is held
  in `authorization-genuineness-attestation.md` beside this file.
