+++
id = "WO-MOK-030"
type = "work_order"
title = "Close the five specification gaps and one contradiction the conformance pass found, so stage 5b can be built without stopping"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-29"
updated = "2026-08-28"

[assurance]
commit_bound_verification = "not_required"
rationale = "This work order changes no executable behavior. It amends three approved specifications so that what stage 5b must build is written down: a minor unit that rule 14.2 calls stated and does not state, an input for prices that rule 14.3 calls an input of the run and provides no way to supply, a retry bound rule 19.5 leaves as the word bounded, a run record shape that carries none of the figures rule 15.2 requires of it, an interface census that a committed field has already falsified, and a contradiction about where the provider binding lives. Nothing is measured and nothing is run. The claim a later reader needs is that the amended text says what it says, which the diff establishes by inspection. `WO-MOK-026` implements against all of it and its assurance is `required`."
decided_by = "engineering owner"

[execution_scope]
paths = [
  "docs/engineering/simulation/evidence/WO-MOK-030/",
  "docs/engineering/simulation/specifications/SPEC-MOK-002.md",
  "docs/engineering/simulation/specifications/SPEC-MOK-006.md",
  "docs/engineering/simulation/specifications/SPEC-MOK-007.md",
  "docs/engineering/simulation/work-orders/WO-MOK-030.md",
]

[relations]
implements = ["REQ-MOK-069", "REQ-MOK-070", "REQ-MOK-071", "REQ-MOK-074"]
specifications = ["SPEC-MOK-002", "SPEC-MOK-006", "SPEC-MOK-007"]
verification = ["VER-MOK-018"]
architecture = ["ARCH-MOK-001", "ARCH-MOK-002", "ADR-MOK-007"]

[[lifecycle_events]]
from = "draft"
to = "approved"
decided_at = "2026-08-28T22:19:36Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "approved"
to = "in_progress"
decided_at = "2026-08-28T22:23:25Z"
decided_by = "engineering owner"

[[lifecycle_events]]
from = "in_progress"
to = "implemented"
decided_at = "2026-08-28T22:23:34Z"
decided_by = "engineering owner"
+++

# Work Order: close what the conformance pass found

## Lifecycle

Governance work, stopping at `implemented`. Assurance is `not_required`: this work order amends documents and
runs nothing. Two transitions under 0.8.0, and the handoff checkpoint takes a snapshot binding whatever the
assurance classification, which this scope admits an evidence directory for.

**On `implements`.** As with `WO-MOK-028` and `WO-MOK-029`, this work order implements the named requirements by
**specifying** them rather than in code. `WO-MOK-026` implements them. The relation is stated this way because an
empty `implements` is refused, and stated with this paragraph beside it so a later reader counting
implementations of `REQ-MOK-069` can tell what each work order did.

## Why this is one work order and not four

`WO-MOK-028` and `WO-MOK-029` each closed one gap, and each was authored, approved, evidenced, transitioned and
pushed to do it. A third and a fourth appeared immediately after the second. On 2026-08-29 the owner directed a
conformance pass instead of a fifth patch: read the specifications against what stage 5b's remaining items
actually need, list every gap at once, and close them in one chain.

The pass is retained at `../WO-MOK-026/conformance-pass.md`. This work order is its consequence.

## In scope

**Six amendments, across three specifications.**

1. **`SPEC-MOK-007` rule 14.2 states the minor unit: the US cent.** The rule requires cost to be "an integer in a
   **stated** minor unit" and states none. Every cost figure in the specification is already written in dollars,
   so the currency was implied everywhere and normative nowhere, and `--spend-ceiling 2` meant two of something
   unnamed.

2. **`SPEC-MOK-007` rule 14.3 gains the input the prices arrive through: `--prices`.** The rule calls the unit
   prices "inputs of the run, not compiled-in constants" and provides no way to supply them, which leaves the one
   thing it forbids as the only thing an implementation could do. The form is four integers in cents per million
   tokens, colon-separated — prompt, cached, output, reasoning — parsed by the shared parser and retained,
   like the ceiling and for the same reason: the run computes with them. A compact option rather than a file,
   because a file needs a format, a grammar, error cases and a test tier that no rule has, and because the whole
   price list is then visible in the command that produced a run, which is what a later reader recomputing a cost
   figure needs.

3. **`SPEC-MOK-007` rule 18.4.2 lists six binary-target options**, `--prices` being the sixth. It said four until
   `WO-MOK-028` and five until `WO-MOK-029`.

4. **`SPEC-MOK-007` rule 19.5 fixes the retry bound at three**, so an exchange is attempted at most four times.
   The rule said "a bounded number of times" and gave no number, while rule 11.2 makes each retry "its own billed
   exchange" — so the missing number was a missing spend. At the specification's own figures an exchange costs
   about $0.0001, so a worst case of every exchange retrying three times is around four cents on a
   two-hundred-exchange run, and rule 14.6's check runs before each attempt, so the bound cannot breach a ceiling
   whatever it is.

5. **`SPEC-MOK-007` rules 10.3a and 10.4a move the provider binding to the connector, where `WO-MOK-026` item 5
   already put it.** The request drops the model identifier and the reasoning level; the response gains them. This
   resolves a contradiction rather than a silence: rule 10.3 had the engine's request carry them, rule 15.2 has the
   engine's run record carry them, and item 5 declares them "in the connector rather than in the engine", which
   cannot all be true. The owner chose on 2026-08-29 to keep item 5 true, and the consequence is that **rule 15.2's
   report becomes a record of the model that answered rather than of the model that was asked for** — which is the
   reading that makes a published figure mean what it says.

6. **`SPEC-MOK-006` rule 8 states the live run's figures**, and **`SPEC-MOK-002` rule 5's interface census gains
   `Config`'s `spend_ceiling` field.** Rule 8.2 fixes the run record's shape and carried none of rule 15.2's seven
   figures, so items 10 and 11 could not be written at all. The census names that struct's public fields exactly
   and was **already falsified** by commit `c13c327`, which added the field under `WO-MOK-026` before this
   contradiction was found; that disclosure is item 8 below.

## Out of scope

- **Every implementation.** No Rust file is touched. `WO-MOK-026` implements all six.
- **`docs/CONNECTOR_PROTOCOL.md`.** Amendment 5 changes what that document must say about the request and the
  response, but it is `WO-MOK-026`'s file, in that work order's execution scope and not this one's. Updating it is
  that work order's act, and until it happens the document and rule 10.4a disagree — which is stated here rather
  than left for a reader to find.
- **`SPEC-MOK-004` rule 1's drift**, recorded under `WO-MOK-026` and still not corrected.
- **Any figure.** No published number moves, and every estimate in `SPEC-MOK-007` stays an estimate.

## Authorized decision envelope

The engineering owner may word the amendment records.

The engineering owner may **not**, under this work order: choose a different unit, price form, retry bound or
binding location from those the owner decided on 2026-08-29; amend a rule not named above; or write any
implementation.

## Constraints

- Every neighbouring rule keeps its text and no existing sub-rule number moves.
- Each of the three specifications takes its own amendment record, in the form that specification already uses.
- `--prices` is named consistently with the surface that exists, and its value is integers so that rule 14.2's
  arithmetic stays integer arithmetic from the input onward.

## Expected change surface

- `docs/engineering/simulation/specifications/SPEC-MOK-007.md` — rules 10.3a, 10.4a, 14.2, 14.3, 18.4.2, 19.5,
  and an amendment record.
- `docs/engineering/simulation/specifications/SPEC-MOK-006.md` — rule 8, and an amendment record.
- `docs/engineering/simulation/specifications/SPEC-MOK-002.md` — rule 5's census, and an amendment record.
- `docs/engineering/simulation/work-orders/WO-MOK-030.md` — this work order's lifecycle events.
- `docs/engineering/simulation/evidence/WO-MOK-030/handoff-check.md` — the handoff snapshot binding.

## Required verification

None contracted beyond `SPEC-MOK-005`'s standing gates. `VER-MOK-018` is the applicable contract and no case of it
is discharged here: its cases are about a running system and this work order runs nothing.

## Evidence to record

The handoff checkpoint's snapshot binding, and nothing else. No measurement is taken.

## Stop and escalate conditions

Stop and return to the engineering owner when:

- a **seventh** gap appears in these specifications. The conformance pass was taken so that the count would stop
  rising by discovery; one more would mean the pass itself was incomplete, which is a fact worth knowing rather
  than absorbing.
- an amendment would change a rule not named in *In scope*; or
- amendment 6 cannot be written without also moving `SPEC-MOK-002` rule 4 or rule 6, which this work order does
  not authorize.

## Completion report format

1. The six amendments, each with what its rule said before and says now.
2. The four owner decisions of 2026-08-29, labelled as decisions.
3. The disclosure of item 8 below.
4. `validate`, and confirmation that no Rust file is in the change.

## 8. Disclosure: an amendment made after the fact

Commit `c13c327`, under `WO-MOK-026`, added `spend_ceiling` to `simulation::Config` and so falsified
`SPEC-MOK-002` rule 5's census **before** this work order existed to authorize it. That ordering is wrong and is
recorded rather than tidied.

It was found by the conformance pass, not by a gate: `validate` reads the census as prose and cannot compare it to
a struct. The field is not removable without stalling `WO-MOK-026` items 9 to 11, because rule 14.6 stops the run
before an exchange and rule 15.2 puts the ceiling in the run record, neither of which a host can do on the
library's behalf without the library knowing the number — and every alternative route is also an interface
change. Amendment 6 is therefore the census catching up with a change already made, which is the same shape as
`SPEC-MOK-002`'s own 2026-08-20 row for `EventDetail::ActionTrace`, and it is disclosed here for the same reason
that row disclosed itself.
