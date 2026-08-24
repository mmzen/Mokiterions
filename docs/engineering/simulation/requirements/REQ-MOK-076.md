+++
id = "REQ-MOK-076"
type = "requirement"
title = "Retain an owner authorization record with every live run's evidence"
status = "approved"
owners = ["product owner"]
created = "2026-08-23"
updated = "2026-08-23"
statement = "WHEN a live model-backed run is executed, THE SYSTEM SHALL retain with that run's evidence an authorization record naming the authorizing owner, the date, the horizon, the seed set and the spend ceiling authorized."
verification_method = "static-analysis"

[relations]
derives_from = ["CAP-MOK-011"]
+++

# Requirement: Retain an owner authorization record with every live run's evidence

## Rationale

The repository owner decided on 2026-08-23 that *"an explicit permission from the repository owner is needed to launch a
real run."* `REQ-MOK-072` and `REQ-MOK-073` are the mechanical side of that decision — a flag, a credential, and no
credential in automation. Those keep an accident from spending money. They cannot tell whether the person who typed the
command had permission, and they leave no trace of the permission afterwards.

This requirement supplies the accountable side. An authorization is a **record retained with the evidence**, in the same
way this repository already binds a verification record to a commit rather than to a claim that verification happened.
Its practical effect is that a live run's evidence is incomplete without it: a reader reviewing a published figure sees
who authorised the spend, for what horizon, over which seeds and up to what amount, and can check that the run actually
matched.

Naming a spend ceiling in the authorization is what connects this to `REQ-MOK-071`. The ceiling the run enforces is the
ceiling the owner named — not a number the operator chose, and not a default. In this repository the owner holds every
governance role, so an authorization is never implied by an earlier approval of anything else: it is a separate act for
a separate run.

## Preconditions and trigger

- A live model-backed run has been executed, whether it completed, ended at its ceiling, or ended in error after at
  least one exchange.

## Required response

The run's retained evidence includes an authorization record naming:

1. The authorizing owner.
2. The date of the authorization.
3. The horizon authorised — the tick limit.
4. The seed set authorised.
5. The spend ceiling authorised, in a stated currency and unit.

The record is retained alongside the run's transcript, record stream and run record, under the evidence path the work
order names.

## Failure and boundary behavior

- **A live run's evidence has no authorization record.** The static check fails. The run's figures are not published and
  the gap is named rather than backfilled with a retrospective authorization, which would be a record of a decision
  nobody made at the time.
- **The run's actual seed, horizon or ceiling differs from the authorised one.** The check fails and reports both. A run
  at seed 7 under an authorization for seeds 0 to 4 is unauthorised, even though each individual run was cheap.
- **One authorization covers several runs.** Permitted, when it names the seed set and the horizon they all fall within —
  a measurement over five seeds is one authorised act, not five. Each run's evidence references the same record.
- **The run ended at its ceiling or in error.** The authorization is still retained. It authorised the spend, and the
  spend happened.
- **A replay.** No authorization is needed, because nothing is spent. `REQ-MOK-072` makes replay the default and
  `REQ-MOK-073` puts it in automation, so the overwhelming majority of runs of this source need no authorization at all.
- **A developer's own exploratory live run on their own credential.** Still a live run of this repository's code
  producing evidence in this repository, so still covered. If its evidence is not retained, it produced no figure and
  cited nothing — and any figure it did produce is unpublishable under this requirement.

## Constraints

- The authorization record contains no credential and no account identifier. It names a person's role and an amount.
- It is a repository artifact under the run's evidence path, not an external message, so that it travels with the
  evidence it authorises.
- The check reads the retained evidence, which is why the method is static analysis: nothing observable at run time can
  establish that permission was given.
- `docs/engineering/simulation/evidence/**` is exempt from end-of-line conversion, so a retained authorization record's
  digest reproduces on any platform.
- This requirement does not define the wording of an authorization, and it does not make the record a substitute for the
  gate. Both hold: `REQ-MOK-072` decides whether a call is possible, this decides whether it was permitted.

## Acceptance examples

### Example: normal behavior

**Given** a measurement of five seeds at a declared horizon, authorised by the repository owner on a stated date with a
stated ceiling

**When** the retained evidence is checked

**Then** an authorization record naming the owner, the date, the horizon, the five seeds and the ceiling is present, and
every run's actual seed, horizon and ceiling falls within it.

### Example: failure behavior

**Given** a live run at a horizon twice the authorised one, with the transcript, record stream and run record all
retained

**When** the retained evidence is checked

**Then** the check fails naming the authorised horizon and the run's horizon, and the run's figures are not published.

## Open decisions

None.
