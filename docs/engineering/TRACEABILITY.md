# Traceability

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**,
**SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and
**OPTIONAL** in this document are to be interpreted as described in BCP 14
(RFC 2119 and RFC 8174) when, and only when, they appear in all capitals.

This document defines the formal artifact chain, the permitted relation types,
and the applicability of each artifact type. It does not define lifecycle
transitions, decision ownership, or quality criteria.

## Normative chain

The formal chain is:

```text
Intent -> Capability -> Requirement <- specifies - Specification
                             ^                         ^
                             | addresses               | conforms_to
                         Architecture <----- decides - ADR
                             |
                         Work order(s) -> Verification
                             |
                  Verification record + commit
                             |
                    Release record + same commit
                             |
                     Operating contract
```

`TRC-001` - Only a declared relation in formal TOML metadata establishes a
formal trace. Filenames, directory paths, source comments, commits, tickets,
dashboards, and conversation text MUST NOT substitute for a declared relation.

`TRC-002` - Every relation MUST use a relation name and source/target type pair
listed below. A validator MUST reject an undeclared pair.

| Relation ID | Declared relation | Source -> target | Cardinality rule |
| --- | --- | --- | --- |
| `TRC-REL-001` | `derives_from` | `CAP -> INT` | Each active capability has one or more active intents. |
| `TRC-REL-002` | `derives_from` | `REQ -> CAP` | Each active requirement has one or more active capabilities. |
| `TRC-REL-003` | `specifies` | `SPEC -> REQ` | Each active requirement selected by a work order has one or more selected active specifications. |
| `TRC-REL-004` | `addresses` | `ARCH -> REQ` | Each architecture names every requirement that materially drives that architecture. |
| `TRC-REL-005` | `conforms_to` | `ARCH -> SPEC` | Each selected architecture names at least one selected specification covering each addressed requirement. |
| `TRC-REL-006` | `decides` | `ADR -> ARCH` | Each selected architecture marked `adr_required` has one or more selected active ADRs. |
| `TRC-REL-007` | `verifies` | `VER -> REQ` | Each active requirement selected by a work order has one or more selected active verification contracts. |
| `TRC-REL-008` | `implements` | `WO -> REQ` | A work order names every requirement in its authorized scope. |
| `TRC-REL-009` | `specifications` | `WO -> SPEC` | A work order names the specifications governing its requirements. |
| `TRC-REL-010` | `architecture` | `WO -> ARCH or ADR` | A work order names every applicable architecture and every required deciding ADR. |
| `TRC-REL-011` | `verification` | `WO -> VER` | A work order names the verification contracts for its requirements. |
| `TRC-REL-012` | `verifies_work_order` | `VREC -> WO` | A VREC names one or more work orders at one exact candidate commit. |
| `TRC-REL-013` | `conforms_to` | `VREC -> VER` | A VREC names every verification contract required by its work-order set. |
| `TRC-REL-014` | `superseded_by` | `VREC -> VREC` | A superseded VREC names exactly one distinct eligible successor. |
| `TRC-REL-015` | `gates` | `REL -> WO` | A release contract names every work order it permits a release to include. |
| `TRC-REL-016` | `satisfies` | `RLS -> REL` | A release record names exactly one applicable release contract. |
| `TRC-REL-017` | `includes_verification` | `RLS -> VREC` | A release record names one or more eligible VRECs at its candidate commit. |
| `TRC-REL-018` | `releases_work` | `RLS -> WO` | The released-work set equals the union of work covered by the included VRECs. |
| `TRC-REL-019` | `assures` | `OPS -> REQ` | An operating contract names every requirement for which it claims continuing assurance. |

`TRC-003` - A selected work order MUST have complete `INT -> CAP -> REQ`
upstream coverage and complete selected `SPEC` and `VER` coverage for every
implemented requirement.

`TRC-004` - Architecture is applicable only when an active architecture
directly addresses a requirement selected by the work order. Routine work MUST
NOT receive fabricated `ARCH` or `ADR` relations.

`TRC-005` - Catalogs in this document define artifact types and relations. They
MUST NOT contain a manually maintained list of artifact instances. The
validator and Explorer derive instances from repository metadata.

## Artifact applicability catalog

This is the authoritative catalog of formal artifact types. "Required" means
that the active graph or current lifecycle phase needs valid coverage; it does
not mean that each work order creates a new file of every type. An existing
active artifact MAY be reused only when its declared scope covers the new work.
A conditional artifact MUST be omitted when its applicability condition is
false.

<!-- artifact-catalog:begin -->
| Type | Prefix | Objective | Required or applicable when | Valid omission or reuse | Accountable owner | Primary relations |
| --- | --- | --- | --- | --- | --- | --- |
| `intent` | `INT-` | Records the approved problem, desired outcome, and accountability boundary. | Governed work needs an approved purpose at G0. | Reuse an active intent when its purpose and outcome still cover the work; do not create one per work order. | Product or domain owner. | `CAP.derives_from -> INT` |
| `capability` | `CAP-` | Describes an actor-visible ability that realizes intent. | Active requirements need an approved capability in their upstream chain. | Reuse when the same actor ability is extended; create a new capability when the observable ability is materially different. | Product or domain owner. | `CAP.derives_from -> INT`; `REQ.derives_from -> CAP` |
| `requirement` | `REQ-` | States one observable normative obligation containing `SHALL`. | Behavior, quality, governance, or operational obligations need explicit active requirements. | Reuse only when the obligation is unchanged; create or revise requirements when the normative obligation changes. | Requirements steward or product owner. | `REQ.derives_from -> CAP`; `SPEC.specifies -> REQ`; `VER.verifies -> REQ` |
| `specification` | `SPEC-` | Defines the exact behavior, interface, constraints, and rejection conditions that satisfy requirements. | Every active requirement selected for implementation needs selected active specification coverage at G1. | One specification may cover several requirements and may be reused while its detailed contract remains applicable. | Technical owner. | `SPEC.specifies -> REQ`; `ARCH.conforms_to -> SPEC`; `WO.specifications -> SPEC` |
| `architecture` | `ARCH-` | Defines structural boundaries and tactics for architecturally significant requirement drivers. | It applies when active architecture directly `addresses` a requirement implemented by the work order. | Omit from routine work when no active architecture addresses an implemented requirement; never fabricate nominal coverage. | Technical owner. | `ARCH.addresses -> REQ`; `ARCH.conforms_to -> SPEC`; `ADR.decides -> ARCH`; `WO.architecture -> ARCH` |
| `adr` | `ADR-` | Records one coherent significant architectural decision, alternatives, and consequences. | Selected architecture with `decision_assessment.outcome = "adr_required"` needs at least one active deciding ADR. | Omit when selected architecture has an accepted `no_significant_decision` assessment; one ADR may decide several related architectures. | Technical owner. | `ADR.decides -> ARCH`; `WO.architecture -> ADR` |
| `verification` | `VER-` | Defines independent methods and pass conditions for checking requirements. | Every active requirement selected for implementation needs selected active verification coverage at G1. | Reuse when methods and pass conditions remain suitable; do not treat test output as the contract itself. | Assurance or quality owner. | `VER.verifies -> REQ`; `WO.verification -> VER`; `VREC.conforms_to -> VER` |
| `work_order` | `WO-` | Grants bounded permission to execute selected implementation or governance work. | One phase-eligible work order is required before execution; its scope must select the complete applicable chain and explicitly classify commit-bound assurance. | Do not reuse completed scope for later unapproved work; one work order may select multiple coherent requirements. | Engineering owner. | `WO.implements -> REQ`; `WO.specifications -> SPEC`; conditional `WO.architecture -> ARCH or ADR`; `WO.verification -> VER` |
| `verification_record` | `VREC-` | Binds work, verification contracts, retained evidence, and one clean candidate commit for assurance review. | Create after candidate commit C for work explicitly classified `assurance.commit_bound_verification = "required"`; verified or released claims require an eligible VREC. | Omit while assurance is not required or not yet proposed; one aggregate record may cover several work orders at the same commit. | Assurance owner. | `VREC.verifies_work_order -> WO`; `VREC.conforms_to -> VER`; optional `VREC.superseded_by -> VREC` |
| `release_contract` | `REL-` | Defines the work scope, gates, rollback conditions, and authority boundary for a release. | Every release record needs an applicable active contract that gates its complete released-work set. | Omit while no release is proposed; a contract may gate several work orders when its policy genuinely covers them. | Release owner. | `REL.gates -> WO`; `RLS.satisfies -> REL` |
| `release_record` | `RLS-` | Records the accountable release decision for eligible verified work at one exact candidate commit. | Create only when a release is proposed; `released` requires eligible VRECs and matching commit identity. | Omit for unreleased continuous delivery; one aggregate record may release several work orders through included verification. | Release owner. | `RLS.satisfies -> REL`; `RLS.includes_verification -> VREC`; `RLS.releases_work -> WO` |
| `operating_contract` | `OPS-` | Defines continuing service, support, observability, or operational assurance obligations. | It applies when repository or service policy declares ongoing operational commitments at G5. | Omit when no operational assurance is claimed; absence never implies that an operational obligation is satisfied. | Service owner. | `OPS.assures -> REQ` |

Evidence, acceptance scenarios, source files, candidate commits, dashboards, tickets, and conversations are not formal artifact types. They may be retained or referenced as observations, but they do not establish product authority, work authorization, verification, or release by themselves.
<!-- artifact-catalog:end -->

## Coverage rules

`TRC-006` - An `ARCH.addresses -> REQ` relation declares a significant
architecture driver. An `ARCH.conforms_to -> SPEC` relation declares the exact
contract used by that architecture. For each addressed requirement, at least
one conforming specification MUST specify that requirement.

`TRC-007` - Each architecture MUST declare `decision_assessment.outcome` as
`adr_required` or `no_significant_decision`. An `adr_required` architecture
MUST be decided by a selected active ADR. `ADR.decides -> ARCH` establishes coverage.
`no_significant_decision` MUST include an accepted rationale and MUST have no
active decision trigger.

`TRC-008` - `ARCH.constrains` is compatibility-only. A validator MAY classify
an unambiguous completed historical relation and MUST report the migration. It
MUST reject a mixed or ambiguous target set. Installation and upgrade MUST NOT
rewrite repository-owned artifacts.

`TRC-009` - A VREC MUST bind one or more work orders, all their declared
verification contracts, retained evidence, and one exact clean candidate
commit. Preparing a VREC does not verify it.

`TRC-010` - An RLS MUST bind one release contract, one exact candidate commit,
one or more eligible VRECs at that commit, and a released-work set equal to the
union of work covered by those VRECs.

`TRC-011` - Governance work is release payload only when an RLS explicitly
includes eligible verification coverage for that work. An observed checkout,
dashboard, status label, or related record MUST NOT add work to a release.

`TRC-012` - Each approved or in-progress work order MUST declare
`assurance.commit_bound_verification` as `required` or `not_required`.
`required` applies when a later engineering, assurance, operating, or release
decision relies on changed trusted state. `not_required` applies only when the
work solely records or transports an already authorized governance decision.
Mixed scope MUST be split or classified `required`.

`TRC-013` - A superseded VREC MUST retain its candidate, evidence, work-order,
and verification-contract facts unchanged. Its `superseded_by` target MUST be a
distinct `verified` or `released` VREC covering every original work order. A
superseded VREC MUST NOT qualify a release.

`TRC-014` - An active `OPS.assures -> REQ` claim requires an active assured
requirement and at least one completed implementing work order. When verified
provenance is required, at least one such work order MUST be covered by a
`verified` or `released` VREC. This reachability does not approve the OPS or
prove continuing conformance.

Lifecycle transitions are defined only by [`WORKFLOW.md`](WORKFLOW.md) and
[`WORKFLOW.json`](WORKFLOW.json). Accountable roles are defined only by
[`DECISION_RIGHTS.md`](DECISION_RIGHTS.md), quality gates only by
[`QUALITY_GATES.md`](QUALITY_GATES.md), and canonical authoring locations only
by [`templates/README.md`](templates/README.md).
