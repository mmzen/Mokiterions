# Artifact Authoring

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**,
**SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and
**OPTIONAL** in this document are to be interpreted as described in BCP 14
(RFC 2119 and RFC 8174) when, and only when, they appear in all capitals.

## Purpose and precedence

This policy states how a good formal artifact is written, per type. It is
consumed by templates, by `harnessctl create-artifact`, which prints the
type's checklist, and by the drafting skill. No consumer restates it.

It ranks below the machine contracts, approved formal semantics, and the
workflow, decision-right, quality-gate, and traceability policies, and above
templates, skills, prompts, and model prose. It grants no authority: a
checklist item is a review standard, not an approval. Rules marked
*mechanical* are enforced by the validator or a gate; the rest are judgement
rules for the accountable reviewer.

## requirement

### Checklist

- One obligation: the statement contains exactly one `SHALL`; split on "and SHALL". *(mechanical: W-AUT-002)*
- One of the five shapes opens the statement: `THE SYSTEM SHALL …` (always), `WHEN <event>, …` (event), `WHILE <state>, …` (state), `IF <unwanted condition>, THEN …` (unwanted), `WHERE <feature>, …` (optional feature). *(mechanical: W-AUT-001)*
- The statement is one sentence; at most 300 characters. *(mechanical: W-AUT-003)*
- `verification_method` lists the methods that will verify it: `test`, `analysis`, `inspection`, `demonstration`. *(mechanical)*
- `priority` says whether it is a `must`, a `should`, or a `could`.
- `source` names where the obligation came from: a stakeholder, a standard clause, an incident, or an artifact.
- A quality requirement carries a `measure`: a value and a unit, not an adjective.
- Rationale says why the obligation exists, not what it does.
- Two acceptance examples, one normal and one failure, each Given/When/Then; executable scenarios live in `acceptance/<REQ-ID>.feature`.
- `Open decisions` reads `None` before approval is requested. *(mechanical at approval, once QG-G1 carries `QGP-G1-AUTHORING`)*
- No template placeholder (`<…>`) survives. *(mechanical at approval, likewise)*

### Guidance

Write the trigger the reader can observe, the response the reader can check,
and nothing else. Avoid escape clauses ("where appropriate", "as needed"),
vague quantities ("fast", "adequate"), and "and/or". If a requirement needs a
diagram or a table to be understood, the detail belongs in a specification
that `specifies` it. A requirement that reads like a plan of work is a work
order in disguise.

## intent

### Checklist

- Problem, outcome, scope boundary, and accountable product owner are each one paragraph.
- The success measure is observable after delivery.
- No solution language.

### Guidance

An intent survives many requirements. Write it so that a reader can tell,
years later, whether the outcome was reached.

## capability

### Checklist

- Names what an actor can do, not how the system does it.
- Derives from at least one active intent and lists its derived requirements.
- Boundaries state what the capability does not decide.

## specification

### Checklist

- Every rule carries a stable identifier (`<PREFIX>-<AREA>-NNN`) and one testable sentence.
- Every `specifies` target requirement is covered by at least one rule.
- Inputs, outputs, and failure behaviour are explicit; no rule depends on prose elsewhere.

### Guidance

A specification is where detail lives. Rules are numbered so that verification
contracts and evidence can cite them exactly.

## architecture

### Checklist

- `addresses` names only architecturally significant requirements; `conforms_to` names the specifications it must respect.
- Components, responsibilities, dependency direction, and trust boundaries are each stated.
- The decision assessment is complete: `adr_required` with triggers, or `no_significant_decision` with a rationale.

## adr

### Checklist

- Context, drivers, at least two considered options with consequences, the decision, and its consequences.
- The chosen option is stated as a decision, not a preference.
- Every `decides` target architecture is an active artifact.

## verification

### Checklist

- Independence: expected values derive from the requirements and specifications, never from candidate output.
- A requirement-to-evidence matrix with a pass condition per requirement.
- Acceptance scenarios cover the failure path, not only the normal path.
- Pass criteria name the platforms and the evaluator.

## work_order

### Checklist

- `[execution_scope].paths` is exact: files or component-prefix directories, nothing wider.
- `[assurance]` is classified and its rationale names what later decisions depend on.
- In scope and out of scope are both stated; the decision envelope says what the implementer may and may not decide.
- Stop conditions and the completion report format are stated.

## release_contract

### Checklist

- `gates` names every work order the release may include.
- Version, rollback conditions, and evidence expectations are explicit.

## operating_contract

### Checklist

- `assures` names every requirement it claims continuing assurance for.
- Observability, support, and operating obligations are measurable.

## risk

### Checklist

- One cause, one effect, one threatened stage; likelihood and impact on the 5x5 scale.
- Prefer `harnessctl raise-risk`, which computes the score and the raise.
- Disposition rationale and residual are written by the disposing role, not the raiser.

## verification_record and release_record

Prepared by `capture-verification` and `prepare-release`; they are not
authored by hand. This policy does not apply to them.
