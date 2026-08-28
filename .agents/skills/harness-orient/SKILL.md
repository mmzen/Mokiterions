---
name: harness-orient
description: Orient an operator to an installed SE Harness repository or one selected formal artifact using its exact released evaluator. Use for read-only repository understanding and next-decision guidance; do not use for implementation, approval, release, Git, credential, or external-action requests.
---

# Harness Orient

Return one decision-ready, read-only orientation to the installed repository.
The harness remains the authority for integrity, lifecycle state, scope, gates,
and accountable roles.

## Required inputs

Obtain an unambiguous repository target, a structured launcher for the target's
exact external released evaluator, its expected version and installation root,
and an optional selected artifact. Do not discover an executable in the target
checkout or silently use one from `PATH`.

Read the applicable repository instructions and this file completely. Validate
the retained `skill-contract.json` before executing the procedure.

## Procedure

Before executing any bundled helper, use the supplied external launcher to run
`version`, released-evaluator `identity`, and `doctor` directly. This establishes
the managed skill bytes as trusted execution input. Stop without running the
helper if any check fails.

Then run `scripts/orient.py` with the same structured inputs. Supply the
evaluator launcher as a JSON array, never as a shell command string. The script
repeats the required `version`, `identity`, and `doctor` checks for its receipt,
then runs `validate --json` and `inspect --json`. When an artifact is selected,
it uses `focus --json` only if the verified evaluator advertises that public
command. It runs preflight only for an explicitly selected work order and
requested phase.

Return the script's canonical JSON result inline. Summarize its lifecycle
state, scoped blockers, repository blockers, separately counted background
observations, required accountable role, and exactly one recommended next
step. Keep candidate-source observations separately labeled and never present
them as the governing result.

## Boundaries

- Perform no repository, Git, lifecycle, environment, network, credential, or
  external mutation.
- Do not install or repair a missing evaluator or damaged managed content.
- Missing required evaluator behavior blocks orientation. Missing optional
  focus or requested preflight behavior degrades only the named output.
- Do not parse human prose to invent selected scope when focus JSON is absent.
- Do not start work, apply a transition, or claim an accountable decision.
- Stop on ambiguous selection, conflicting owner instructions, evaluator
  identity failure, managed-integrity failure, invalid formal state, malformed
  required JSON, or any observed state change during orientation.
- Use the complete single-agent procedure. Do not spawn or coordinate workers.

The receipt is evidence, not authority. Return it inline and write no receipt
or other evidence file into the target repository.
