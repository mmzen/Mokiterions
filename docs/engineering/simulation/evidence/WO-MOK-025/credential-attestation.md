# WO-MOK-025 credential attestation — recorded (`C6`)

`VER-MOK-018` check **C6** is the one item in that contract no check inside this repository can make, and it
is the fact the whole cost containment rests on. It is recorded here because the contract says where it
lives: "it is the repository owner's statement, **retained with the evidence**", and *Evidence retention*
lists "The credential attestation | Beside the measurement's evidence | one file". This is that file for
this stage.

**The attestation, verbatim, made 2026-08-24 by the repository owner over commit `4cfb297`:**

> *"Attest — none is configured"*

| Fact | Value |
|---|---|
| Check | `VER-MOK-018` **C6** |
| What is attested | no model-provider credential is configured in this repository's automation secrets |
| Accountable role | repository owner |
| Date | 2026-08-24 |
| Commit the attestation was made over | `4cfb297` |
| Corroborating measurement | below: **0** secrets, **0** variables at every scope a workflow can read one from |
| State | **RECORDED — attested** |

## The check, as amended the same day

`C6`'s wording changed on 2026-08-24, in the same pass that ruled on this attestation, and the amended text
is what this record answers:

> - **C6** The attestation that the credential is not configured in the repository's automation secrets. It
>   is the repository owner's statement, retained with the evidence, and it is the single fact the whole cost
>   containment rests on. **The clause "no check can see this" is withdrawn** *(amended 2026-08-24)*: secret
>   **names** are enumerable through the hosting platform's API at every scope a workflow can read a secret
>   from, and `WO-MOK-025`'s evidence retains that measurement […] What no check can see is a **value**, and
>   what no measurement establishes is the state of that surface at any moment other than the one it was
>   taken at. **So the measurement corroborates the attestation and does not make it** […]

That amendment was escalation **E20**, raised on taking the measurement below and finding the contract
claimed more ignorance than the repository is in. `VER-MOK-018`'s amendment record carries the ruling, and
`bf027c8` is the commit that wrote it. **The attestation itself is unchanged in force**: what moved is a
sentence about what a check can see, not the question the owner answered.

## The corroborating measurement

Taken 2026-08-24 against `github.com/mmzen/Mokiterions` with `gh` 2.97.0, over the four surfaces a workflow
in this repository can read a secret or a variable from, plus the ownership fact that closes the fifth.

| Surface | API path | Count | Names |
|---|---|---:|---|
| Repository Actions secrets | `repos/mmzen/Mokiterions/actions/secrets` | **0** | none |
| Dependabot secrets | `repos/mmzen/Mokiterions/dependabot/secrets` | **0** | none |
| Repository Actions variables | `repos/mmzen/Mokiterions/actions/variables` | **0** | none |
| Environment `github-pages` — secrets, variables | `…/environments/github-pages/{secrets,variables}` | **0**, **0** | none |
| Environment `release` — secrets, variables | `…/environments/release/{secrets,variables}` | **0**, **0** | none |
| Organization scope | — | **does not exist** | the owner `mmzen` is a **User** account, so there is no organization scope to inherit a secret from |

Two environments exist and both were enumerated by name rather than assumed: `github-pages` and `release`.
The organization row is not a zero but an absence — a personal account has no organization secrets, so there
is no fourth scope whose emptiness would have to be measured.

**Why the measurement is wider than the check.** `C6` names "the repository's automation secrets". Actions
secrets are what that plainly means; Dependabot secrets, Actions variables and per-environment secrets were
added by the implementation agent because each is a place a value can be put that a workflow can then read,
and a containment that held for one surface and not the others would be no containment. **The widening is
the agent's and the attestation is the owner's**: the owner's statement covers the surface `C6` names, and
the extra rows are corroboration of the same shape.

## Why this corroborates the attestation and does not make it

Three limits, stated so that a later reader does not mistake a retained zero for a check that passed:

- **Names are enumerable; values are not.** The API returns a secret's name, its creation date and its
  update date. It never returns a value. A zero count is therefore stronger than "no credential-looking name
  was found" — there is nothing at all — but it is a statement about the *presence of entries*, and the fact
  the containment rests on is about a credential, which is a value.
- **No measurement covers a moment other than its own.** This one was taken on 2026-08-24 over the surface
  as it then stood. A secret added an hour later would not falsify a single figure above. Only a person who
  administers the repository can speak to the state of that surface as a standing fact, and that is what an
  attestation is for.
- **It is not reproducible by a later reader as it was taken.** The commands can be re-run, and doing so is
  worth doing, but the result is a *new* measurement of a live remote surface rather than a re-derivation of
  this one. Every other figure in this packet re-derives from the repository; this one cannot, and that is
  the ordinary condition of any fact about a hosting platform.

## What no check inside this repository sees

`scripts/check_workflow_credentials.py` is the gate for `L21a` and `L21b`, and it says this in its own
output rather than leaving a reader to infer the boundary:

    NOT CHECKABLE HERE: whether a provider credential is present in the repository's Actions
      secrets. REQ-MOK-073's *Constraints* records that as the condition this containment rests on,
      and VER-MOK-018 carries it as an owner attestation.

That statement remains exactly right about what the *script* can see: it reads the repository's workflows and
nothing outside them. `candidate/gates.txt` retains its run at the candidate — `scripts/check_workflow_credentials.py --root .`
exiting `0`, with its own suite of 38 tests. What the amendment of 2026-08-24 corrected was a broader claim
in `VER-MOK-018` that nothing at all could see the surface — a claim about the world rather than about the
script.

## What this attestation is load-bearing for

- **`REQ-MOK-073`** — "WHEN the repository's automated workflows run, THE SYSTEM SHALL make no provider
  call, and no workflow in the repository SHALL reference a model-provider credential." The workflow half is
  checked; the *absence of a credential to reference* is this attestation.
- **`ADR-MOK-001`** — "Future provider credentials must remain outside the engine and repository."
- **`docs/engineering/REPOSITORY_CONTEXT.md`** — "Model-provider credentials and other secrets must remain
  outside the repository and must not be committed."
- **The cost containment.** No live run has been made or authorised at this stage; the owner's standing
  instruction is that "an explicit permission from the repository owner is needed to launch a real run". A
  credential absent from automation is what makes an unauthorised run impossible rather than merely
  forbidden.

## What this file does not establish

- **It does not verify.** `C6` is a check in a verification contract; the verification decision is the
  assurance owner's and lives in `VREC-MOK-024`.
- **It says nothing about a credential held anywhere else.** A credential on a person's own machine is
  outside this repository and outside `C6`, and it is where `ADR-MOK-001` requires one to be if it exists.
- **It is not a claim about any later state of the automation surface**, for the reason the second limit
  above gives.
- **It does not extend to `L28`.** Whether an authorisation for a live run is genuine is a separate owner
  attestation, there is no live run to authorise at this stage, and `L28` is listed among what was not
  verified.
