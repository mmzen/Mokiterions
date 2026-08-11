# Engineering Artifacts for Mokiterions

> Repository-owned after installation. Maintain this file as the index of local artifact domains and supporting engineering documentation. Shared harness policy is routed from `ENGINEERING_HARNESS.md` and remains managed separately.

## Artifact domains

- [`simulation/`](simulation/): formal artifacts for the Mokiterions simulation core, beginning with the minimum runnable foundation.

## Repository-specific engineering documentation

- `REPOSITORY_CONTEXT.md`: owner-confirmed commands, entry points, and constraints.
- `../../README.md`: product concept, initial scope, non-goals, and success criteria.
- No repository-specific architecture overview or runbook exists yet.

## Initial engineering direction

- Implement the project in Rust.
- Use Claude Code, primarily with Opus, to implement the project.
- Use the owner-described `OpenAI GPT nano` model for runtime decisions made by simulated agents after the exact OpenAI API model identifier is confirmed.
- Keep the first stage text-only. It should produce simple output and should not include a graphical or web UI.
- Apply KISS: choose the smallest design that satisfies approved requirements, keep control flow easy to follow, and avoid abstractions created only for hypothetical future needs.
- Treat a richer future interface as later work rather than allowing it to complicate the first-stage design.

## Current maturity

The minimum simulation foundation has an approved intent-to-verification chain. `WO-MOK-001` is implemented with retained evidence and a passing review preflight. Candidate-commit provenance, commit-bound verification, and release authorization are recorded separately when available.

## Maintenance

Update this index when domains or local engineering documents are added, moved, or retired. Formal authority comes from typed artifact metadata and accountable lifecycle decisions, not from this index.
