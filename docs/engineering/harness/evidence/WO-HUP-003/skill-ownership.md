# Skill ownership recorded as `plugin`

Evaluator: se-harness 0.18.0 from the exact public wheel, outside the checkout, `python -I -m se_harness`. Captured 2026-09-15 on the working tree of branch `harness/wo-hup-003-adopt-se-harness-0.18.0` after both transactions.

    harnessctl skill-ownership . --provider plugin --plugin-root <installed verity-plane plugin> --json
    harnessctl skill-ownership . --provider plugin --plugin-root <installed verity-plane plugin> --apply --json

Plan: outcome `planned`, passed `True`, written `False`. Apply: outcome `applied`, passed `True`, written `True`. Both report the same four changes:

| Action | Path |
|---|---|
| `remove` | `.agents/skills/harness-orient` |
| `remove` | `.agents/skills/harness-operator-brief` |
| `remove` | `.claude/skills/harness-orient` |
| `update` | `.engineering-harness.lock` |

The lock afterwards: schema `4`, `skill_ownership.provider = "plugin"`, evaluator `0.18.0` with `archive_sha256` `a683dbdf485d42aa20ea8502122c171a4c61c7d60f85db5b5f264bd336371c54`. The repository stores the provider and never the plugin's machine path; `.agents/skills/` and `.claude/skills/` are empty. An ordinary clone runs every evaluator command without a plugin installed. Whether this host actually discovers the plugin's `harness-orient` and `harness-operator-brief` skills is a fact only the host can show, and `doctor` does not prove it.

Order matters and was kept: the command was run after both transactions. Measured at `8ab8a30` in a disposable worktree, it also accepts a 0.8.0 root and strips its skills while `tool_version` still reads 0.8.0.
