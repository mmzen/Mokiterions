+++
id = "SPEC-xxx"
type = "specification"
title = "<What this specification binds, as a noun phrase>"
status = "draft"
owners = ["<technical/domain owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
# What an implementation must do to conform. The Explorer shows this summary under the title.
contract = "<One sentence: what an implementation must do to conform.>"

[relations]
specifies = ["REQ-xxx"]
+++

# Specification: <title>

Before approval, apply the shared design principle and `specification` checklist in
`docs/engineering/ARTIFACT_AUTHORING.md`.

## In plain words

<One or two sentences a newcomer understands. A project term used here is
defined in this repository's own glossary, `GLOSSARY.md` at the repository
root, which this repository writes.>

## Scope

<What this specification governs and what it leaves to another artifact.>

## Terms

- **<term>.** <One sentence. Only terms this specification introduces; the
  glossary has the rest.>

## Rules

<Describe testable behavior. Give a rule a stable identifier when another artifact
needs to refer to it. Include failure behavior and examples below.>

**<PREFIX>-<AREA>-001.** <The implementation MUST ...>

**<PREFIX>-<AREA>-002.** <The implementation MUST NOT ...>

## Failure behaviour

| Trigger | Response | Diagnostic |
| --- | --- | --- |
| <the condition> | <what the implementation does, in one sentence> | `<code or none>` |

## Examples

**Given** <state>, **when** <event>, **then** <what holds, naming the rule
by identifier>.

## Coverage

<Every requirement in `specifies` has a row; every rule named exists above.
The validator reads this table and the Explorer shows it on both sides.>

| Requirement | Rules |
| --- | --- |
| `REQ-xxx` | <PREFIX>-<AREA>-001, <PREFIX>-<AREA>-002 |

## Not decided here

- <A choice this specification leaves to the implementation, at most five
  bullets. Actors, inputs and outputs, a state model, data contracts,
  security, performance, observability and compatibility are optional
  sections; the authoring guide says when each earns its place.>
