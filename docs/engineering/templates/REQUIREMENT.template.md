+++
id = "REQ-xxx"
type = "requirement"
title = "<Observable obligation>"
status = "draft"
owners = ["<product/domain owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
# State an observable behavior. SHALL is optional.
statement = "<The observable behavior the system provides.>"
verification_method = ["test"]
priority = "must"
source = "<stakeholder, standard clause, incident, or artifact ID>"
measure = "<value and unit, for a quality requirement>"

[relations]
derives_from = ["CAP-xxx"]
+++

# Requirement: <title>

Before approval, apply the shared design principle and `requirement` checklist in
`docs/engineering/ARTIFACT_AUTHORING.md`.

## In plain words

<One or two sentences a newcomer understands. A project term used here is
defined in this repository's own glossary, `GLOSSARY.md` at the repository
root, which this repository writes.>

## Why

<Why the obligation exists. How
it is met belongs in the specification that specifies this requirement.>

## Behavior

| Trigger | Response | On failure |
| --- | --- | --- |
| <the observable condition or event; "always" for an invariant> | <what the reader can check> | <what happens when the response cannot be given> |

## Examples

### Normal

**Given** ...

**When** ...

**Then** ...

### Failure

**Given** ...

**When** ...

**Then** ...
