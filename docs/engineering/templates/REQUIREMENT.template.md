+++
id = "REQ-xxx"
type = "requirement"
title = "<Observable obligation>"
status = "draft"
owners = ["<product/domain owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
# One obligation per requirement; split on "and SHALL". Pick one shape:
#   THE SYSTEM SHALL <response>.                          (always)
#   WHEN <event>, THE SYSTEM SHALL <response>.            (event)
#   WHILE <state>, THE SYSTEM SHALL <response>.           (state)
#   IF <unwanted condition>, THEN THE SYSTEM SHALL <response>.   (unwanted)
#   WHERE <feature is present>, THE SYSTEM SHALL <response>.     (optional feature)
statement = "WHEN <event>, THE SYSTEM SHALL <observable response>."
verification_method = ["test"]
priority = "must"
source = "<stakeholder, standard clause, incident, or artifact ID>"
measure = "<value and unit, for a quality requirement>"

[relations]
derives_from = ["CAP-xxx"]
+++

# Requirement: <title>

## Rationale

Why this obligation exists, not what it does.

## Behavior

- Trigger: <the observable condition or event; "always" for an invariant>
- Response: <what the reader can check>
- On failure: <what happens when the response cannot be given>

## Assumptions and dependencies

<What this obligation relies on; not how it is built — that is a specification's job.>

## Acceptance examples

Executable scenarios live in `acceptance/<REQ-ID>.feature` and are named by
the verification contract that covers this requirement.

### Example: normal behavior

**Given** ...

**When** ...

**Then** ...

### Example: failure behavior

**Given** ...

**When** ...

**Then** ...

## Open decisions

None.
