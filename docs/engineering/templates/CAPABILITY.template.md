+++
id = "CAP-xxx"
type = "capability"
title = "<Actor ability>"
status = "draft"
owners = ["<product owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
# What the actor can do and the conditions that matter. The Explorer shows this summary under the title.
ability = "<Actor> can <perform or achieve something> under <important conditions>."

[relations]
derives_from = ["INT-xxx"]
+++

# Capability: <title>

Before approval, apply the shared design principle and `capability` checklist in
`docs/engineering/ARTIFACT_AUTHORING.md`.

## In plain words

<One or two sentences a newcomer understands. A project term used here is
defined in this repository's own glossary, `GLOSSARY.md` at the repository
root, which this repository writes.>

## Actor and need

<: who the actor is, and what they need, in their
words. The outcome the need serves is the intent's and is not restated
here. The behaviors that meet the need are the requirements', and the way
they are met is the specification's.>

## Not decided here

- <what this capability leaves to a requirement, a specification or another
  capability; at most five bullets. The requirements that derive from this
  capability are read from the graph and shown by the Explorer; do not list
  them here.>
