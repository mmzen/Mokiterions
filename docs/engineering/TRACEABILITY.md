# Traceability

The normative chain is:

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

Only declared relations in formal TOML metadata establish authority. Source comments, filenames, commits, tickets, and conversational references may aid discovery but do not satisfy formal coverage.

## Artifact applicability catalog

This is the authoritative catalog of the standard formal artifact types. “Required” means that the active graph or current lifecycle phase needs valid coverage; it does not mean that every work order creates a new file of every type. Reuse an existing active artifact only when it truthfully covers the new scope. Omit a conditional artifact rather than creating a ceremonial placeholder.

<!-- artifact-catalog:begin -->
| Type | Prefix | Objective | Required or applicable when | Valid omission or reuse | Accountable owner | Primary relations |
| --- | --- | --- | --- | --- | --- | --- |
| `intent` | `INT-` | Records the approved problem, desired outcome, and accountability boundary. | Governed work needs an approved purpose at G0. | Reuse an active intent when its purpose and outcome still cover the work; do not create one per work order. | Product or domain owner. | `CAP.derives_from → INT` |
| `capability` | `CAP-` | Describes an actor-visible ability that realizes intent. | Active requirements need an approved capability in their upstream chain. | Reuse when the same actor ability is extended; create a new capability when the observable ability is materially different. | Product or domain owner. | `CAP.derives_from → INT`; `REQ.derives_from → CAP` |
| `requirement` | `REQ-` | States one observable normative obligation containing `SHALL`. | Behavior, quality, governance, or operational obligations need explicit active requirements. | Reuse only when the obligation is unchanged; create or revise requirements when the normative obligation changes. | Requirements steward or product owner. | `REQ.derives_from → CAP`; `SPEC.specifies → REQ`; `VER.verifies → REQ` |
| `specification` | `SPEC-` | Defines the exact behavior, interface, constraints, and rejection conditions that satisfy requirements. | Every active requirement selected for implementation needs selected active specification coverage at G1. | One specification may cover several requirements and may be reused while its detailed contract remains applicable. | Technical owner. | `SPEC.specifies → REQ`; `ARCH.conforms_to → SPEC`; `WO.specifications → SPEC` |
| `architecture` | `ARCH-` | Defines structural boundaries and tactics for architecturally significant requirement drivers. | It applies when active architecture directly `addresses` a requirement implemented by the work order. | Omit from routine work when no active architecture addresses an implemented requirement; never fabricate nominal coverage. | Technical owner. | `ARCH.addresses → REQ`; `ARCH.conforms_to → SPEC`; `ADR.decides → ARCH`; `WO.architecture → ARCH` |
| `adr` | `ADR-` | Records one coherent significant architectural decision, alternatives, and consequences. | Selected architecture with `decision_assessment.outcome = "adr_required"` needs at least one active deciding ADR. | Omit when selected architecture has an accepted `no_significant_decision` assessment; one ADR may decide several related architectures. | Technical owner. | `ADR.decides → ARCH`; `WO.architecture → ADR` |
| `verification` | `VER-` | Defines independent methods and pass conditions for checking requirements. | Every active requirement selected for implementation needs selected active verification coverage at G1. | Reuse when methods and pass conditions remain suitable; do not treat test output as the contract itself. | Assurance or quality owner. | `VER.verifies → REQ`; `WO.verification → VER`; `VREC.conforms_to → VER` |
| `work_order` | `WO-` | Grants bounded permission to execute selected implementation or governance work. | One phase-eligible work order is required before execution; its scope must select the complete applicable chain and explicitly classify commit-bound assurance. | Do not reuse completed scope for later unapproved work; one work order may select multiple coherent requirements. | Engineering owner. | `WO.implements → REQ`; `WO.specifications → SPEC`; conditional `WO.architecture → ARCH or ADR`; `WO.verification → VER` |
| `verification_record` | `VREC-` | Binds work, verification contracts, retained evidence, and one clean candidate commit for assurance review. | Create after candidate commit C for work explicitly classified `assurance.commit_bound_verification = "required"`; verified or released claims require an eligible VREC. | Omit while assurance is not required or not yet proposed; one aggregate record may cover several work orders at the same commit. | Assurance owner. | `VREC.verifies_work_order → WO`; `VREC.conforms_to → VER`; optional `VREC.superseded_by → VREC` |
| `release_contract` | `REL-` | Defines the work scope, gates, rollback conditions, and authority boundary for a release. | Every release record needs an applicable active contract that gates its complete released-work set. | Omit while no release is proposed; a contract may gate several work orders when its policy genuinely covers them. | Release owner. | `REL.gates → WO`; `RLS.satisfies → REL` |
| `release_record` | `RLS-` | Records the accountable release decision for eligible verified work at one exact candidate commit. | Create only when a release is proposed; `released` requires eligible VRECs and matching commit identity. | Omit for unreleased continuous delivery; one aggregate record may release several work orders through included verification. | Release owner. | `RLS.satisfies → REL`; `RLS.includes_verification → VREC`; `RLS.releases_work → WO` |
| `operating_contract` | `OPS-` | Defines continuing service, support, observability, or operational assurance obligations. | It applies when repository or service policy declares ongoing operational commitments at G5. | Omit when no operational assurance is claimed; absence never implies that an operational obligation is satisfied. | Service owner. | `OPS.assures → REQ` |

Evidence, acceptance scenarios, source files, candidate commits, dashboards, tickets, and conversations are not formal artifact types. They may be retained or referenced as observations, but they do not establish product authority, work authorization, verification, or release by themselves.
<!-- artifact-catalog:end -->

Lifecycle transitions remain defined by [`WORKFLOW.md`](WORKFLOW.md), accountable roles by [`DECISION_RIGHTS.md`](DECISION_RIGHTS.md), gates by [`QUALITY_GATES.md`](QUALITY_GATES.md), and canonical authoring locations by [`templates/README.md`](templates/README.md).

An active `OPS.assures -> REQ` claim requires more than accepted operating prose. Each assured requirement must be active and have at least one completed implementing work order. When commit-bound verified-work provenance is enabled, at least one such work order must also be covered by a verified or released VREC. These reachability checks establish an evidence-backed implementation path; they do not approve the OPS or prove continuing operational conformance.

`ARCH.addresses -> REQ` declares only architecturally significant requirement drivers. `ARCH.conforms_to -> SPEC` declares the detailed behavioral or interface contracts relevant to that architecture. Every addressed requirement must be reachable through a conforming specification's `SPEC.specifies -> REQ` relation, but a specification may also cover routine requirements that do not drive architecture. Explorer may show that transitive path as derived context; it never replaces the direct declared relations or creates authority.

Work-order definition and verification coverage remain independent: every implemented requirement needs selected specification and verification coverage. Architecture is applicable when an active architecture directly addresses an implemented requirement, and every selected architecture must share a conforming specification with the work order. Do not fabricate architecture coverage for routine requirements.

The old polymorphic `ARCH.constrains` relation is compatibility-only. Completed unambiguous historical forms may be classified with a migration advisory; mixed target types fail closed. Installation and upgrade never rewrite repository-owned formal artifacts.

Architecture decision applicability is declared on each architecture. `ADR.decides -> ARCH` establishes coverage for an `adr_required` architecture; the mere presence of an ADR elsewhere in the work order does not. A work order selects every applicable architecture and each required deciding ADR. An accepted `no_significant_decision` assessment permits omission of an ADR for that architecture. ADR cardinality follows coherent significant decisions and is independent of requirement, specification, work-order, or architecture counts.

`verification_record` binds one or more release-bearing work orders, their declared verification contracts, and retained evidence to one clean final candidate commit. `release_record` binds a release contract to the same commit and an exact released-work set equal to included verification coverage. Single-work-order records remain aggregates of cardinality one.

Governance-only work may authorize review, verification transition, release transition, tagging, or publication in later commits, but it is not automatically release payload. The dashboard's observed checkout revision is derived context, not release authority.

Commit-bound verification applicability is explicit per work order. `required` applies when future engineering, assurance, operational, or release decisions rely on the correctness of changed executable behavior, managed policy, CI, definitions, traceability, or other trusted state. `not_required` applies only when the work solely records or transports an already authorized verification, release, supersession, publication, or deployment decision. Mixed scope is split or classified `required`; automation never infers either value. Every work order still requires ordinary validation and retained evidence.

A governance-only work order records completed execution as `implemented`. It does not become `verified` merely because it authorizes the transition of a different VREC; doing so would create recursive governance work. When configured provenance is required, a work order may claim `verified` or `released` only when a verified or released VREC explicitly includes it. VREC and RLS records, rather than work-order status alone, are the authoritative commit and release bindings.

A stale `ready` verification record may be retained as `superseded` only through a separate accountable governance decision. Its `superseded_by` relation names one distinct verified or released VREC that covers every original work order. Supersession preserves the old candidate and evidence facts, is terminal, and never contributes verification or release readiness. Dashboard overlap findings are non-authoritative prompts, not lifecycle decisions.
