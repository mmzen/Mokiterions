+++
id = "INT-xxx"
type = "intent"
title = "<Outcome-oriented title>"
status = "draft"
owners = ["<accountable product/domain role>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
# The desired outcome and who benefits. The Explorer shows this summary under the title.
outcome = "<WHO> can <observable result after delivery>."

[relations]
+++

# Intent: <title>

Before approval, apply the shared design principle and `intent` checklist in
`docs/engineering/ARTIFACT_AUTHORING.md`.

## In plain words

<One or two sentences a newcomer understands. A project term used here is
defined in this repository's own glossary, `GLOSSARY.md` at the repository
root, which this repository writes.>

## Problem

<What happens today, to whom, and why it needs to change. Link useful evidence.>

## Success measures

<A success measure is observed in operation, after delivery, by someone who
has not read the code. "Observed" names a place and a cadence an operator
recognises. A row proved by a CI run, a test, a validator run, a
verification or an implementation review is an acceptance check and
belongs in the verification contract. "Today" may read "not measured".>

| Measure | Today | When reached | Observed |
| --- | --- | --- | --- |
| <what an operator can count or time after delivery> | <baseline, or "not measured"> | <target> | <where and how often, in operation> |

## Not this

- <what this initiative deliberately leaves alone; at most five bullets>
