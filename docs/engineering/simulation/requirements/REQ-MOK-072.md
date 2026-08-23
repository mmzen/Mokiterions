+++
id = "REQ-MOK-072"
type = "requirement"
title = "Require both an explicit live selection and a credential before any provider call"
status = "draft"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN either an explicit live-mode selection or a provider credential in the process environment is absent, THE SYSTEM SHALL make no provider call."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Require both an explicit live selection and a credential before any provider call

## Rationale

The repository owner decided on 2026-08-23 that *"an explicit permission from the repository owner is needed to launch a
real run."* A permission recorded only in a document is a permission a script can bypass by accident. This requirement
is the mechanical half of that decision: two independent conditions must both hold, and neither is something a run
acquires by default.

The two conditions fail in different ways, which is why both are needed. A credential can be present on a developer's
machine for perfectly good reasons — a shell profile, an inherited environment, another project's tooling — and would
otherwise turn a routine `--policy` typo into a billed run. An explicit flag can be left in a saved command line or a
committed script, and would otherwise be enough on any machine that happens to hold a key. Requiring both means the
accident has to happen twice, in two different places, at once.

The default direction matters more than the mechanism. Absent an explicit live selection the model-backed source
**replays**, which is free, offline and the mode verification uses. `REQ-MOK-067` makes replay the equal of a live run
for every purpose except producing new decisions, so the safe default is also the useful one — there is no incentive to
route around it.

## Preconditions and trigger

- A run has selected the model-backed decision source.
- One or both of: no explicit live-mode selection was made; no provider credential is present in the process
  environment.

## Required response

1. Make no provider call: open no socket, spawn no provider process, read no credential.
2. If a transcript was supplied, replay from it under `REQ-MOK-067`.
3. If no transcript was supplied either, refuse the run with a message naming which condition was missing, and exit with
   the documented usage-error status rather than producing a partial run.

## Failure and boundary behavior

- **The flag is present and the credential is absent.** No provider call. The run says the credential is missing. It
  does not prompt for one, and it does not search a file, a keychain or a configuration directory for one — the
  environment is the only place looked.
- **The credential is present and the flag is absent.** No provider call. The run replays if a transcript was given and
  otherwise refuses. A present credential is never taken as consent.
- **Both are present.** The live path is available, and `REQ-MOK-076`'s authorization record and `REQ-MOK-071`'s ceiling
  still bind it. This requirement is a gate, not the whole authorization.
- **The credential is present but empty or malformed.** Treated as absent. An empty value is not a credential.
- **Another decision source is selected.** No provider call is possible at all, and the credential is not read even to
  check it.
- **Automated workflows.** Covered separately and more strongly by `REQ-MOK-073`, which keeps the credential out of
  continuous integration entirely rather than relying on this gate.

## Constraints

- The credential is read from the process environment and is never written to any file this repository tracks, never
  echoed to standard output or standard error, never included in a request record and never included in an error
  message. `REPOSITORY_CONTEXT.md` requires model-provider credentials to remain outside the repository, and a run's
  retained evidence is inside it.
- The engine does not read the environment. The host reads it, and hands the engine a connected port, on the precedent
  that the engine *"resolves no path, opens no file, creates no directory and removes none"* and receives an
  already-open sink from the caller that owns it.
- The check is verified without a credential ever existing in the verification environment. A test that needs a real key
  to prove that no call happens would be untestable in this repository by construction.

## Acceptance examples

### Example: normal behavior

**Given** the model-backed source selected, a retained transcript supplied, and no live-mode flag

**When** the run executes with a credential present in the environment

**Then** the run replays from the transcript, makes no provider call, and produces the byte-identical output
`REQ-MOK-067` requires.

### Example: failure behavior

**Given** the model-backed source selected with the live-mode flag and no credential in the environment

**When** the run executes

**Then** no provider call is made, the run reports that the credential is absent, the credential name is not printed
with any value, and the exit status is the documented usage error.

## Open decisions

None.
