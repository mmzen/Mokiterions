+++
id = "REQ-MOK-073"
type = "requirement"
title = "Keep the provider credential and every provider call out of continuous integration"
status = "draft"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN the repository's automated workflows run, THE SYSTEM SHALL make no provider call, and no workflow in the repository SHALL reference a model-provider credential."
verification_method = "static-analysis"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Keep the provider credential and every provider call out of continuous integration

## Rationale

The repository owner decided on 2026-08-23 that continuous integration *"can not replay the LLM policy after push,
release etc"*, that the same holds for unit and automated tests, and that a real run needs explicit permission. The word
*replay* in that instruction is worth reading precisely: what the owner is refusing is **spending**, and
`REQ-MOK-067`'s replay spends nothing. This requirement therefore forbids provider **calls** in automation, and leaves
transcript replay available to it — that is what makes the model-backed source testable in CI at all.

The stronger half is the credential. `REQ-MOK-072` gates a call behind a flag and a key, but a key placed in the
repository's Actions secrets would put a spendable credential one workflow edit, one pull request from a fork, or one
compromised action away from being used. The containment that actually holds is **capability-based**: the credential is
never available to automation, so no workflow change and no injected step can spend money. That is a property of the
repository's configuration rather than of its code, which is why this requirement is verified by static analysis over
the workflow files rather than by a test.

This also settles what a green CI run means for this initiative. It means the port, the replay, the isolation checks and
the byte-identity of the four existing sources all hold. It does not mean a model was consulted. `VER-MOK-018` states
which checks are consequently manual and owner-gated, so the gap is written down rather than discovered later.

## Preconditions and trigger

- Any automated workflow in the repository runs: on push, on pull request, on tag, on release, or on a schedule.

## Required response

1. No provider call occurs in any workflow job.
2. No workflow file references a model-provider credential — not as a secret, not as an environment variable, not as an
   input, and not through a step that fetches one from elsewhere.
3. Any workflow step that exercises the model-backed source does so in replay mode against a transcript committed to the
   repository.
4. The prohibition is checked by a repository check that fails the build when a workflow acquires such a reference.

## Failure and boundary behavior

- **A workflow adds the credential as a secret reference.** The static check fails and names the file and the line. This
  is the case the check exists for, and it fails on the pull request that introduces it rather than after it merges.
- **A workflow runs the model-backed source in live mode without a credential.** `REQ-MOK-072` makes it call nothing, so
  no money is spent, but the static check still fails on the live-mode selection appearing in a workflow, because a
  workflow that asks to spend is one secret away from spending.
- **A canned transcript is committed for CI replay.** Permitted and intended. It carries no credential, is small — an
  **estimated** 100 to 260 KB for 20 to 50 ticks — and is provenance once a verification record binds it.
- **A developer runs a live run on their own machine.** Out of this requirement's scope and governed by `REQ-MOK-072`
  and `REQ-MOK-076`. This requirement binds the repository's automation.
- **A future workflow genuinely needs a live run.** It does not get one by adding a secret. It requires the owner to
  decide, in an amendment to this requirement, that CI may spend money — an accountable act with a record, which is the
  point.

## Constraints

- The check reads the workflow definitions in the repository, so it detects an intent to use a credential even on a
  machine where none exists. A runtime check could only observe the runs that happened.
- The credential is not placed in the repository's Actions secrets. That is a configuration act outside the working
  tree, and it is stated here because it is the condition the whole containment rests on; `VER-MOK-018` records it as an
  owner attestation rather than as something a check can see.
- This requirement does not exempt the model-backed source from automated testing. It fixes what those tests may do:
  everything except spend.

## Acceptance examples

### Example: normal behavior

**Given** the repository's workflows at a candidate commit

**When** the static check scans them

**Then** it finds no model-provider credential reference and no live-mode selection, it finds the replay step running
against the committed transcript, and the check passes.

### Example: failure behavior

**Given** a pull request that adds a provider key to a workflow's environment in order to measure a run in CI

**When** the static check runs

**Then** it fails, names the workflow file and line, and the build is red until the reference is removed or the owner
amends this requirement.

## Open decisions

None.
