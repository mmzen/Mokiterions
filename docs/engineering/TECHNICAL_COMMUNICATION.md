# Technical Communication

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**,
**SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and
**OPTIONAL** in this document are to be interpreted as described in BCP 14
(RFC 2119 and RFC 8174) when, and only when, they appear in all capitals.

## Purpose and claim

This policy controls eligible English prose written by agents for operators and
technical artifacts. It uses selected clarity principles based on ASD-STE100.
It is not ASD-STE100 compliance, certification, approval, or endorsement.

An agent MUST NOT download, search for, bundle, reproduce, parse, or attempt to
strictly implement ASD-STE100 or a controlled dictionary. The installed policy
is complete for its declared purpose.

## Authority and precedence

Apply these sources from highest to lowest:

1. machine-readable harness contracts and exact evaluator results;
2. approved formal artifact semantics and accountable decisions;
3. managed workflow, decision-right, quality-gate, traceability, and repository
   policy;
4. repository-approved technical terminology;
5. this policy; and
6. skills, adapters, prompts, and model prose.

A lower source MUST NOT rewrite, reinterpret, or weaken a higher source. Only
`harnessctl` computes lifecycle legality and the canonical next action. This
policy grants no product, technical, engineering, assurance, release, Git,
credential, network, or external-action authority.

## Eligible prose

Eligible prose is agent-authored English explanation that is not protected
content. For eligible prose, the agent SHOULD:

- use one stable term for one concept;
- define an uncommon project term before relying on it;
- identify the responsible actor when responsibility matters;
- state conditions, actions, and results directly;
- prefer active voice when it identifies responsibility;
- keep each sentence focused on one principal action;
- avoid ambiguous pronouns, decorative synonyms, hidden negation, vague
  references, and unnecessary introductions; and
- use a list or table when it clarifies parallel conditions, mappings, or
  ordered steps.

Sentence length is a review signal. It is not a conformance threshold and does
not justify removing necessary technical detail.

## Protected content

Exact protected content MUST remain byte-identical. It includes:

- code and inline code;
- commands, paths, identifiers, hashes, version strings, URLs, schemas, and
  field names;
- JSON, TOML, YAML, XML, and other machine-readable data;
- logs, diagnostics, evidence, evaluator output, and canonical restitution
  blocks;
- quotations; and
- operator-supplied text that is presented as supplied text.

Semantically protected content MUST NOT be automatically paraphrased. It
includes BCP 14 obligations, requirement statements, lifecycle and decision
meanings, safety or legal qualifications, acceptance thresholds, formulas, and
established terminology.

New normative prose MAY use the clarity principles during initial drafting only
when its actor, condition, force, scope, qualification, and result stay explicit.
An automatic rendering pass MUST NOT change those properties.

If a protected boundary, term, or comparison is ambiguous, the agent MUST stop
the rendering result and preserve the source. If a canonical procedure forbids
surrounding text, the canonical block MUST be returned alone and unchanged.

## Communication profiles

### `operator-communication`

Use this profile for explanations to an operator. Lead with the outcome or one
required action. Name the accountable actor. State limits and non-effects
directly. Present one principal action per sentence and do not invent a second
next step.

### `technical-artifact-writing`

Use this profile for narrative purpose, rationale, scope, descriptions,
procedures, assumptions, risks, and examples in a selected draft artifact.
Make actors, conditions, actions, results, and terminology explicit while
preserving necessary engineering detail.

Apply this profile minimally or not at all to metadata, normative statements,
semantic tables, code, exact results, and evidence.

Profiles apply during composition or deliberate revision of selected draft
prose. They are not automatic post-processors. They MUST NOT justify a
repository-wide rewrite or a style-only rewrite of approved or historical
artifacts.

Non-English output, exact-output requests, direct quotations, and unsupported
purposes MUST NOT claim a managed profile result.

## Deviations and human decision points

Routine eligible prose needs no new human review. Ask a human only to:

- decide or clarify meaning;
- approve a new project term;
- resolve an ambiguous protected boundary;
- exercise an existing accountable decision right; or
- approve a separately governed policy revision.

A material deviation is a conscious exception needed to preserve meaning. Its
record MUST name the policy rule, the reason, the meaning protected, and the
resulting limit. Routine protected spans are not deviations. A deviation MUST
NOT contain hidden reasoning.

## Examples

Clear operator prose: “WO-EX-001 is implemented. The assurance owner must decide
VREC-EX-001. This result does not merge or release the change.”

Invalid prose: “Everything is approved, so I released it.” This sentence hides
distinct decisions and claims an action that the communication policy cannot
authorize.

Valid artifact drafting may clarify a rationale while leaving `SHALL`,
`REQ-EX-001`, `0.6.0`, a JSON field, and an acceptance threshold unchanged.

Invalid rendering changes a hash, replaces a defined term with a synonym,
weakens `SHALL`, surrounds a verbatim restitution block with commentary, or
claims that the result is ASD-STE100 compliant.

## Failure and recovery

Missing policy integrity, wrong evaluator identity, malformed source, ambiguous
profile selection, an unsupported language, or failed preservation MUST stop a
completed profile claim. Recovery uses corrected local input or one accountable
decision. Recovery MUST NOT retrieve the external standard, broaden scope,
change protected content, or perform a lifecycle or external action.

Readability indicators and vocabulary suggestions are advisory evidence only.
They do not prove correctness, semantic equivalence, or compliance.
