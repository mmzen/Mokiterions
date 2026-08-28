---
name: harness-orient
description: Discover the canonical read-only SE Harness orientation skill for understanding repository state without changing it.
metadata:
  adapter-schema: se-harness-host-adapter-v1
  canonical-name: harness-orient
  canonical-path: .agents/skills/harness-orient
---

# SE Harness discovery adapter

This is a non-authoritative Claude Code discovery adapter. It defines no
workflow, engineering authority, or repository effect.

1. Resolve the fixed directory
   `${CLAUDE_PROJECT_DIR}/.agents/skills/harness-orient`. Do not search
   anywhere else.
2. Require the resolved directory to remain inside the project root and to
   match `canonical-name` exactly.
3. Read the complete canonical `SKILL.md` and `skill-contract.json`. Require
   the contract name and managed integrity to match this adapter.
4. Load every resource referenced by the canonical procedure only from that
   canonical directory.
5. Stop before a helper or repository effect if binding, identity, integrity,
   contract validation, or resource loading fails.

After these checks, yield entirely to the canonical procedure.
