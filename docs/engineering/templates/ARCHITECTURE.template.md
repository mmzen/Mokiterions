+++
id = "ARCH-xxx"
type = "architecture"
title = "<Architecture boundary>"
status = "draft"
owners = ["<architect/technical owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"

[relations]
addresses = ["REQ-xxx"]
conforms_to = ["SPEC-xxx"]

[decision_assessment]
outcome = "<adr_required-or-no_significant_decision>"
triggers = []
rationale = "<why the selected outcome follows from the trigger assessment>"
assessed_by = "<accountable technical owner>"
+++

# Architecture: <title>

## Context and scope

Use `addresses` only for architecturally significant requirement drivers: requirements that materially shape boundaries, responsibilities, interfaces, data ownership, trust, deployment, technology, or quality-attribute tactics. Do not add every routine requirement merely to create nominal coverage. Use `conforms_to` for the detailed specifications whose behavior or interfaces this architecture must respect. Every addressed requirement must be specified by at least one conforming specification.

Before this artifact leaves `draft`, replace every decision-assessment placeholder. Omission is not a no-ADR decision. Select `adr_required` when this architecture introduces or materially changes any controlled trigger below. Select `no_significant_decision` only when every trigger is false, keep `triggers = []`, and record the technical owner's accepted rationale.

Significant-decision triggers:

- `system-boundary`
- `responsibility-or-dependency-direction`
- `public-interface-or-protocol`
- `data-ownership-or-persistence`
- `security-privacy-or-trust-boundary`
- `deployment-or-operating-model`
- `concurrency-consistency-reliability-or-failure-strategy`
- `technology-framework-vendor-or-external-service`
- `material-performance-scalability-or-cost-tradeoff`
- `cross-cutting-policy`
- `difficult-to-reverse`
- `material-alternatives`

Initial software design normally activates one or more triggers, but applicability is evidence-based rather than automatic.

## Components and responsibilities

## Dependency direction

## Data and control flow

## Trust boundaries

## Required patterns

## Prohibited patterns

## Quality attributes

## Conformance checks

## Related ADRs

When `outcome = "adr_required"`, identify the active ADR or ADRs whose `decides` relation targets this architecture. An ADR records a coherent significant decision; it is not a one-per-requirement quota.
