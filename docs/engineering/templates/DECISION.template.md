+++
id = "DEC-xxx"
type = "decision"
title = "<The question, as a noun phrase>"
status = "open"
owners = ["<role that holds the decision right for the blocked artifact>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
# One of: question (an ambiguity met while authoring or planning) or
# deviation (an implementation cannot meet one rule of one specification).
kind = "question"
question = "<One sentence ending with a question mark?>"
raised_by = "<actor or role that raised it>"
recommendation = "<id of the recommended option>"
# A deviation also needs the departed rule and the observed fact:
# against = "SPEC-xxx#PREFIX-AREA-NNN"   (a rule identifier the specification defines)
# observed = "<what cannot be met, in one or two sentences>"

# At least two options. A deviation's options are drawn from
# amend, supersede, accept, stop, and include stop.
[[options]]
id = "<option-id>"
label = "<what choosing it means, in one sentence>"

[[options]]
id = "<option-id>"
label = "<what choosing it means, in one sentence>"

[relations]
concerns = ["<every artifact the question is about>"]
blocks = ["<the artifacts that cannot change state until this is disposed>"]
+++

# Decision: <title>

## Question

State the question once, then the facts a decider needs and nothing else.

## Options

One paragraph per option: what it means, what it costs, what it forecloses.

## Recommendation

Which option, and why, in two or three sentences.

## Disposition

Written by `harnessctl decide`; do not edit by hand. The `[disposition]`
table records the option, its label, the role, the time and the verbatim
reason. A `deferred` decision records its scope and revisit trigger. An
accepted deviation records its revisit trigger and stays visible on the
departed specification, the work order and the records until the rule
changes.
