+++
id = "WO-MOK-028"
type = "work_order"
title = "Complete SPEC-MOK-007's connector contract: the wire format's field names, the error vocabulary, and the option a live run writes its transcript to"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-28"
updated = "2026-08-28"

[assurance]
commit_bound_verification = "not_required"
rationale = "This work order changes no executable behavior. It amends one approved specification so that three things it leaves open are stated: the JSON field names of a wire format a third party implements against, the error vocabulary a connector may return, and the option through which a live run writes its transcript. Nothing here is measured, nothing is run, and no figure this repository publishes moves. The claim a later reader needs is that the amended text says what it says, which the diff establishes by inspection. `WO-MOK-026` is the work order that implements against this text and its assurance is `required`; a commit-bound record here would bind evidence about a document rather than about behaviour, and `SPEC-MOK-005`'s release gate reads the amended specification directly."
decided_by = "engineering owner"

[execution_scope]
paths = [
  "docs/engineering/simulation/evidence/WO-MOK-028/",
  "docs/engineering/simulation/specifications/SPEC-MOK-007.md",
  "docs/engineering/simulation/work-orders/WO-MOK-028.md",
]

[relations]
implements = ["REQ-MOK-069", "REQ-MOK-074", "REQ-MOK-077"]
specifications = ["SPEC-MOK-007"]
verification = ["VER-MOK-018"]
# Rule 18.4.2's amendment states which options each host acts on, and rule 10.4a's usage
# fields are what the engine's accounting reads, so both architectures are addressed and
# `ADR-MOK-007` is the active decision behind each. The same three `WO-MOK-026` selects,
# because this work order specifies exactly what that one implements.
architecture = ["ARCH-MOK-001", "ARCH-MOK-002", "ADR-MOK-007"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T21:35:00Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "approved"
to = "in_progress"
decided_at = "2026-08-28T21:38:38Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "in_progress"
to = "implemented"
decided_at = "2026-08-28T21:40:02Z"
decided_by = "engineering owner"
+++

# Work Order: complete SPEC-MOK-007's connector contract

## Lifecycle

Governance work, stopping at `implemented`. Commit-bound verification is `not_required` and the *Assurance*
rationale states why: this work order amends a document and runs nothing.

**On `implements`.** This work order does not implement `REQ-MOK-069`, `REQ-MOK-074` or `REQ-MOK-077` in code —
`WO-MOK-026` does. It implements them by **specifying** them: each of the three amendments below is a thing one
of those requirements needs stated before it can be built against, and stage 5b was stopped at exactly the point
where the specification ran out. The relation is stated this way rather than left empty because an empty
`implements` is refused, and stated with this paragraph beside it rather than silently, because a later reader
counting implementations of `REQ-MOK-069` would otherwise find two work orders and no way to tell what each did.

## Objective

State the three things `SPEC-MOK-007` leaves open that a connector author and a host implementer both need, so
that `WO-MOK-026` can be built against a specification rather than against a judgment.

## Why this exists as its own work order

`WO-MOK-026`'s *Expected change surface* names `REPOSITORY_CONTEXT.md`, `SPEC-MOK-003` and `SPEC-MOK-004`, and
**not** `SPEC-MOK-007`. It was written on the assumption that its governing specification already said everything
stage 5b needed. Building the connector protocol document and the canned connector showed three places where it
does not, and the execution scope derived from that surface text refused the file:

```text
QGP-G4I-PATHS: WEX201: changed path is outside execution scope:
  docs/engineering/simulation/specifications/SPEC-MOK-007.md
```

That refusal is the boundary working, not failing. The owner decided on 2026-08-28 that the amendments belong in
their own chain rather than by widening `WO-MOK-026` a second time, so that stage 5b's diff stays implementation
and this one stays specification repair.

## In scope

1. **Rule 10 gains the wire format's field names**, as normative. Rule 10.3 and 10.4 fix what a request and a
   response *carry* and never what their keys are called, so a connector author has no contract to write against
   and two independent implementations would not interoperate. The names are those `docs/CONNECTOR_PROTOCOL.md`
   already documents: `protocol`, `tick`, `actor`, `prompt`, `model`, `reasoning` and `schema` on a request;
   `action` with `verb` and optional `parameter`, `usage` with `prompt`, `cached_prompt`, `output` and
   `reasoning`, and `error` with `kind` and `message` on a response.
2. **Rule 19 gains the error vocabulary and what each kind does.** Rule 19.5 speaks only of "a transport failure"
   and its bounded retry. Four kinds are stated: `transport` and `provider` are retried under 19.5; `malformed`
   and `refused` are not retried and become an immediate counted fallback under `REQ-MOK-074`. Every attempt is
   a transcript record under rule 11.2 whichever kind it was.
3. **Rule 18.4.2 gains a fifth binary-target option, `--transcript-output`.** A live run has no transcript to
   read and must write one — rule 19.6 makes a transcript failure the one thing worth aborting a live run for —
   while `--transcript-path` was shipped by `WO-MOK-025` meaning "read this run's decisions from a transcript of
   an earlier run". The owner decided on 2026-08-28 that the read option keeps its meaning and a separate write
   option is added, rather than one option changing direction by mode. `WO-MOK-025`'s help text stays true
   unchanged, which is the decision's main merit.
4. **Nothing in `docs/CONNECTOR_PROTOCOL.md`.** That document records the same names and would be the natural place
   to note that they became normative, but it is `WO-MOK-026`'s file: it appears in that work order's execution scope
   and not in this one's, and it does not exist on this work order's branch at all. The note is `WO-MOK-026`'s to add.
   Rule 10.4b names the document from this side, which is the direction that costs nothing.

## Out of scope

- **Every implementation.** No Rust file is touched. Not the parser, not the spawn, not the observer's refusals.
  `WO-MOK-026` does all of it, and this work order exists so that it can.
- **`SPEC-MOK-004` rule 1's drift.** Its tree says the engine's `tests/` holds five files and the observer's
  eight where the tree holds ten and nine, and it does not show `tests/support/`. That is a real pre-existing
  finding, recorded in `WO-MOK-026`'s evidence, and the owner decided on 2026-08-28 not to correct it here. It
  belongs to whatever work order takes specification-to-tree drift as its subject.
- **Any other rule of `SPEC-MOK-007`.** Rules 1 to 9, 11 to 17 and 20 are untouched.
- **Any figure.** Nothing is measured and no published number moves.

## Authorized decision envelope

The engineering owner may word the amendment records.

The engineering owner may **not**, under this work order: change any rule of `SPEC-MOK-007` other than 10, 18.4.2
and 19; alter the field names from those `docs/CONNECTOR_PROTOCOL.md` already documents, since connector authors
may already have read them; add a sixth error kind; or write any implementation.

## Constraints

- Each amended rule keeps its number and every neighbouring rule keeps its text.
- The amendment record states what moved, by whose decision, on what date, and why the specification was silent.
- The field names are recorded as they already are, not improved. A name changed now breaks a contract this
  repository has already published under `WO-MOK-026`.

## Expected change surface

- `docs/engineering/simulation/specifications/SPEC-MOK-007.md` — rules 10, 18.4.2 and 19, and an amendment record.
- `docs/engineering/simulation/work-orders/WO-MOK-028.md` — this work order's own lifecycle events.
- `docs/engineering/simulation/evidence/WO-MOK-028/handoff-check.md` — the handoff checkpoint's snapshot binding.

## Required verification

None contracted beyond `SPEC-MOK-005`'s standing gates, which read the amended specification on every pull
request and at release. `VER-MOK-018` is the applicable contract and no case of it is discharged here: its cases
are about a running system, and this work order runs nothing. `WO-MOK-026` discharges them.

## Evidence to record

**One file, and the reason it exists is worth stating.** No *measurement* is retained: nothing here is run and
`commit_bound_verification` is `not_required`, so the diff is the evidence and the amendment record inside the
specification is where a later reader meets it.

But `not_required` assurance is not the same as no evidence. The handoff checkpoint requires a binding to the
formal snapshot of the graph it evaluated, for every work order, whatever its assurance classification:

```text
QGP-G4I-EVIDENCE: No readable evidence for WO-MOK-028, checkpoint handoff,
and formal snapshot 67ebd127... is available.
```

So `docs/engineering/simulation/evidence/WO-MOK-028/handoff-check.md` is retained, carrying that binding and the
declared change set, and the evidence directory is in this work order's execution scope for it. This paragraph
replaces an earlier draft of this section that read "None retained" — which was written before the gate was run
and was wrong about what the harness requires rather than about what this work order measures.

## Stop and escalate conditions

Stop and return to the engineering owner when:

- an amendment would change a rule other than 10, 18.4.2 or 19;
- stating a field name would contradict `docs/CONNECTOR_PROTOCOL.md` as already committed; or
- a fourth thing turns out to be unspecified, rather than being folded in silently.

## Completion report format

1. The three amended rules, each with what it said before and what it says now.
2. The three owner decisions of 2026-08-28 this work order carries, labelled as decisions, and the fourth that
   routed it here rather than into `WO-MOK-026`.
3. `validate` under the released evaluator.
4. Confirmation that no Rust file is in the change.
