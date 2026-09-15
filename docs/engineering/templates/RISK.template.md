+++
id = "RISK-xxx"
type = "risk"
title = "<What could go wrong>"
status = "identified"
owners = ["<risk owner>"]
created = "YYYY-MM-DD"
updated = "YYYY-MM-DD"
description = "<What could go wrong and why it matters.>"
action = "<The owner's next action.>"

[relations]
threatens = []
+++

# Risk: <title>

Prefer `harnessctl raise-risk` to record this note in `raised`.
An owner and next action are enough. Scoring and taxonomy are optional.
Use `--with-decision --threatens ARTIFACT-ID` only to request a blocking decision.
Earlier scored risks and their recorded dispositions remain readable unchanged.
