# Artifact Authoring

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**,
**SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and
**OPTIONAL** in this document are to be interpreted as described in BCP 14
(RFC 2119 and RFC 8174) when, and only when, they appear in all capitals.

## Purpose and precedence

This policy guides authoring, design and review. Templates and skills point
here; `harnessctl create-artifact` prints the selected type's checklist.
The shared principles and questions are maintained here.

It ranks below the machine contracts, approved formal semantics, and the
workflow, decision-right, quality-gate, and traceability policies, and above
templates, skills, prompts, and model prose. It grants no authority: a
checklist item is a review standard, not an approval. Rules marked
*mechanical* are enforced by the validator or a gate; the rest are judgement
rules for the accountable reviewer.

## Design simplicity

Choose the least complex design that satisfies the agreed requirements and
constraints. Justify material added complexity by the need it serves and why
a simpler approach is insufficient. Consider the effort to understand, build,
test, operate and change the whole solution. Defer speculative needs. Review
both the necessity of the specified behavior and the simplicity of its
implementation. Preserve correctness and the project's required quality and
safety properties.

Start from the smallest complete solution. Each project supplies its agreed
users, scale, operating conditions and quality constraints. Agreed future needs
can matter; imagined flexibility alone is insufficient. Credible risk analysis
can justify prevention before an incident. Fewer lines, shorter specifications
or fewer tests do not establish simplicity; moving work to users, operations
or dependencies also has a cost.

Apply the relevant questions below before definition approval and during design
and implementation review. Return unjustified obligations or design complexity
for revision through the existing decision process. Explain material choices
in the existing specification, design or review: the need, why a simpler option
is insufficient and the added cost. Scale the explanation to the decision; a
small choice may need one sentence, a significant design may use its existing
architecture decision. Trivial changes need no separate comparison document.

These are human review standards. They introduce no separate KISS artifact,
mandatory score, receipt, automated simplicity gate or additional approval.

## requirement

### Checklist

- State the observable behavior and why it is needed. Ordinary language is accepted; `SHALL` is optional.
- What agreed outcome does this obligation serve, for whom and under which conditions? Separate requirements from assumptions and speculative needs.
- Include a concrete acceptance condition in `Examples`, `Acceptance`, `measure`, `verification_notes`, or a linked verification contract.
- Name the source capability and a verification method. Supported method arrays contain `test`, `analysis`, `inspection`, or `demonstration`; retained text methods remain readable.
- Replace unfinished placeholders and resolve blocking decisions before approval.

### Guidance

Write enough for someone to implement and check the behavior. Use the template's
sections when helpful. Word counts, sentence shapes and numbers of code references
are not checked. Writing hints never block approval. Optional metadata must still
have a valid value when supplied.

## intent

### Checklist

- Explain the present problem, who it affects, and the outcome wanted.
- What is the smallest useful outcome, and which needs and constraints are actually agreed?
- Say how the owner will know the change helped; an honest baseline may be `not measured`.
- State the important scope limits. Create a new intent when the desired outcome changes.

## capability

### Checklist

- Say what an actor should be able to do and any relevant conditions.
- Does each supported case serve an agreed need under those conditions?
- Link the intent it serves. The graph lists the derived requirements.
- Put implementation details where they help the implementer; no required `can ... under` wording.

## specification

### Checklist

- Explain what an implementation must do, including meaningful failure behavior.
- Is each promised behavior necessary? Could a simpler contract meet the agreed need? Prescribe a mechanism only when it is a necessary constraint.
- Link the requirements and show an example someone can check.
- Give a rule a stable identifier when evidence or a decision needs to refer to it.
- Check that supplied rule references exist and identifiers are unambiguous.

Missing summaries and ambiguous rule references may produce draft hints. Hints
help a reviewer; they do not withhold approval. Existing approved text need not
be rewritten to adopt these checklists.

## architecture

### Checklist

- `addresses` names only architecturally significant requirements; `conforms_to` names the specifications it must respect.
- Components, responsibilities, dependency direction, and trust boundaries are each stated.
- The decision assessment is complete: `adr_required` with triggers, or `no_significant_decision` with a rationale.
- For material added complexity, what simpler approach was considered, including existing capabilities? Explain why it is insufficient and the added cost across development, testing, operation and change.

## adr

### Checklist

- Context, drivers, at least two considered options with consequences, the decision, and its consequences.
- The chosen option is stated as a decision, not a preference.
- If the chosen option adds material complexity, why is the simpler option insufficient for the agreed constraints, and is the added cost justified?
- Every `decides` target architecture is an active artifact.

## verification

### Checklist

- Independence: expected values derive from the requirements and specifications, never from candidate output.
- A requirement-to-evidence matrix with a pass condition per requirement.
- Cover normal behavior and meaningful failure outcomes or risks; do not invent failure scenarios to fill a template.
- What accepted behavior or quality does each test or CI check establish? Cover distinct relevant outcomes, reuse applicable evidence, and avoid mirroring every implementation branch or combination.
- Pass criteria name the platforms and the evaluator.

## work_order

### Checklist

- `[execution_scope].paths` is exact: files or component-prefix directories, nothing wider.
- `[assurance]` is classified and its rationale names what later decisions depend on.
- In scope and out of scope are both stated; the decision envelope says what the implementer may and may not decide.
- Stop conditions and the completion report format are stated.
- Is every planned change needed for the agreed outcome? Include removal or adjustment of obsolete behavior, tests and instructions within the approved scope.

## Review of implemented changes

- Does the result satisfy the necessary contract, and did implementation reveal unnecessary specified behavior? Resolve an excessive approved obligation through the existing amendment process before changing its behavior.
- Is each material addition needed? Could a direct solution or existing capability meet the same constraints with less total work to understand, test, operate and change?
- Does the normal design or review evidence explain significant complexity and its cost? Remove unused generality or indirection that has no agreed purpose.
- Do tests and CI establish the accepted behavior and relevant quality properties? Remove obsolete checks together with the obligations they no longer serve.

Retain material findings and their resolution with the ordinary review evidence.
Applying these questions is part of that review, not a second review or receipt.

## release_contract

### Checklist

- `gates` names every work order the release may include.
- Version, rollback conditions, and evidence expectations are explicit.

## operating_contract

### Checklist

- `assures` names every requirement it claims continuing assurance for.
- Observability, support, and operating obligations are measurable.

## decision

A pending question becomes a `decision` (`DEC-`) when it blocks a transition
of another artifact, concerns more than one artifact, or must survive the
approval of the artifact that raised it. Below that threshold the actor asks
and the answer stays in the transition's `reason`.

### Checklist

- `kind` is `question` (an ambiguity met while authoring or planning) or `deviation` (an implementation cannot meet one rule of one specification). *(mechanical)*
- One `question`, `raised_by`, at least two `[[options]]` with `id` and `label`, and a `recommendation` naming one option. *(mechanical)*
- A deviation also names `against = "SPEC-xxx#PREFIX-AREA-NNN"`, a rule identifier the specification defines *(mechanical: E-DCM-005)*, and the `observed` fact; its options are drawn from `amend`, `supersede`, `accept`, `stop` and include `stop`. *(mechanical)*
- `concerns` names every artifact the question is about; `blocks` names the artifacts that cannot change state while it is `open`, each also in `concerns`. *(mechanical)*
- The `[disposition]` table is written by `harnessctl decide`; a hand-written one is `E-DCM-003`. A deferral needs a scope and a revisit trigger; accepting a deviation needs a revisit trigger. *(mechanical)*
- `decided` and `withdrawn` decisions are never deleted or rewritten.
- The definition templates carry no `Open decisions` section: a definition's pending decisions are the `DEC-` artifacts that name it in `blocks`, and the approval gate reads them from the graph. A legacy section, where one remains, reads `None` or lists `DEC-` identifiers; prose there is `E-DCM-004`.

## risk

### Checklist

- Record a description, an owner and a next action.
- Use `harnessctl raise-risk --description TEXT --action TEXT --owner ROLE` with a domain and title.
- Stage, category, threatened artifacts and scoring are optional. If scoring is useful, supply likelihood and impact together (1–5); the command computes their product.
- Request `--with-decision` only when the owner needs a decision that blocks named artifacts. A risk by itself does not block work.
- Keep earlier scores, decisions and dispositions as recorded; no migration is required.


## verification_record and release_record

Prepared by `capture-verification` and `prepare-release`; they are not
authored by hand. This policy does not apply to them.
